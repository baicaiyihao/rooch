// Copyright (c) RoochNetwork
// SPDX-License-Identifier: Apache-2.0

use crate::service::rpc_service::RpcService;
use anyhow::Result;
use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::StructTag;
use moveos_types::access_path::AccessPath;
use moveos_types::h256::H256;
use moveos_types::moveos_std::object::ObjectID;
use moveos_types::state::PlaceholderStruct;
use rooch_rpc_api::jsonrpc_types::account_view::BalanceInfoView;
use rooch_rpc_api::jsonrpc_types::CoinInfoView;
use rooch_types::address::RoochAddress;
use rooch_types::framework::account_coin_store::AccountCoinStoreModule;
use rooch_types::framework::coin::{CoinInfo, CoinModule};
use rooch_types::framework::coin_store::{CoinStore, CoinStoreInfo};
use rooch_types::indexer::state::{IndexerStateID, ObjectStateFilter, ObjectStateType};
use rooch_types::indexer::transaction::IndexerTransaction;
use rooch_types::transaction::TransactionWithInfo;
use std::collections::HashMap;

/// AggregateService is aggregate RPC service and MoveFunctionCaller.
#[derive(Clone)]
pub struct AggregateService {
    rpc_service: RpcService,
}

impl AggregateService {
    pub fn new(rpc_service: RpcService) -> Self {
        Self { rpc_service }
    }
}

impl AggregateService {
    pub async fn get_coin_infos(
        &self,
        coin_types: Vec<StructTag>,
    ) -> Result<HashMap<StructTag, Option<CoinInfoView>>> {
        let access_path = AccessPath::objects(
            coin_types
                .iter()
                .cloned()
                .map(CoinModule::coin_info_id)
                .collect(),
        );
        self.rpc_service
            .get_states(access_path, None)
            .await?
            .into_iter()
            .zip(coin_types)
            .map(|(state_opt, coin_type)| {
                Ok((
                    coin_type,
                    state_opt
                        .map(|state| {
                            Ok::<CoinInfoView, anyhow::Error>(CoinInfoView::from(
                                state
                                    .into_object_uncheck::<CoinInfo<PlaceholderStruct>>()?
                                    .value,
                            ))
                        })
                        .transpose()?,
                ))
            })
            .collect::<Result<HashMap<_, _>>>()
    }

    pub async fn get_coin_stores(
        &self,
        coin_store_ids: Vec<ObjectID>,
    ) -> Result<Vec<Option<CoinStoreInfo>>> {
        let access_path = AccessPath::objects(coin_store_ids);
        self.rpc_service
            .get_states(access_path, None)
            .await?
            .into_iter()
            .map(|state_opt| state_opt.map(CoinStoreInfo::try_from).transpose())
            .collect::<Result<Vec<_>>>()
    }

    pub async fn get_balance(
        &self,
        account_addr: RoochAddress,
        coin_type: StructTag,
    ) -> Result<BalanceInfoView> {
        let coin_info = self
            .get_coin_infos(vec![coin_type.clone()])
            .await?
            .into_values()
            .flatten()
            .next()
            .ok_or_else(|| {
                anyhow::anyhow!("Can not find CoinInfo with coin_type: {}", coin_type)
            })?;

        let coin_store_id =
            AccountCoinStoreModule::account_coin_store_id(account_addr.into(), coin_type);
        let balance = self
            .get_coin_stores(vec![coin_store_id])
            .await?
            .pop()
            .flatten()
            .map(|coin_store| coin_store.balance())
            .unwrap_or_default();

        Ok(BalanceInfoView::new(coin_info, balance))
    }

    pub async fn query_account_coin_stores(
        &self,
        owner: AccountAddress,
        cursor: Option<IndexerStateID>,
        limit: usize,
    ) -> Result<Vec<(ObjectID, IndexerStateID)>> {
        self.rpc_service
            .indexer
            .query_object_ids(
                ObjectStateFilter::ObjectTypeWithOwner {
                    object_type: CoinStore::struct_tag_without_coin_type(),
                    filter_out: false,
                    owner,
                },
                cursor,
                limit,
                false,
                ObjectStateType::ObjectState,
            )
            .await
    }

    pub async fn get_balances(
        &self,
        owner: AccountAddress,
        cursor: Option<IndexerStateID>,
        limit: usize,
    ) -> Result<Vec<(Option<IndexerStateID>, BalanceInfoView)>> {
        let indexer_coin_stores = self.query_account_coin_stores(owner, cursor, limit).await?;
        let coin_store_ids = indexer_coin_stores
            .iter()
            .map(|(id, _state_id)| id.clone())
            .collect::<Vec<_>>();

        let coin_stores = self.get_coin_stores(coin_store_ids.clone()).await?;

        let coin_types = coin_stores
            .iter()
            .flatten()
            .map(|coin_store| coin_store.coin_type())
            .collect::<Vec<_>>();

        let coin_info_map = self.get_coin_infos(coin_types).await?;

        let mut result = vec![];
        for ((id, state_id), coin_store) in indexer_coin_stores.into_iter().zip(coin_stores) {
            let coin_store = coin_store
                .ok_or_else(|| anyhow::anyhow!("Can not find CoinStore with id: {}", id))?;
            let coin_info = coin_info_map
                .get(&coin_store.coin_type())
                .cloned()
                .flatten()
                .ok_or_else(|| {
                    anyhow::anyhow!("Can not find CoinInfo for {}", coin_store.coin_type_str())
                })?;
            let balance_info = BalanceInfoView::new(coin_info.clone(), coin_store.balance());
            result.push((Some(state_id), balance_info))
        }

        Ok(result)
    }

    pub async fn get_transaction_with_info(
        &self,
        tx_hashes: Vec<H256>,
    ) -> Result<Vec<Option<TransactionWithInfo>>> {
        let transactions = self
            .rpc_service
            .get_transactions_by_hash(tx_hashes.clone())
            .await?;

        let execution_infos = self
            .rpc_service
            .get_transaction_execution_infos_by_hash(tx_hashes.clone())
            .await?;

        debug_assert!(transactions.len() == execution_infos.len());

        transactions
            .into_iter()
            .zip(execution_infos)
            .map(|(tx_opt, exec_info_opt)| match (tx_opt, exec_info_opt) {
                (Some(tx), Some(exec_info)) => Ok(Some(TransactionWithInfo {
                    transaction: tx,
                    execution_info: Some(exec_info),
                })),
                (Some(tx), None) => Ok(Some(TransactionWithInfo {
                    transaction: tx,
                    execution_info: None,
                })),
                _ => Ok(None),
            })
            .collect::<Result<Vec<_>>>()
    }

    pub async fn build_transaction_with_infos(
        &self,
        indexer_txs: Vec<IndexerTransaction>,
    ) -> Result<Vec<TransactionWithInfo>> {
        let tx_hashes = indexer_txs.iter().map(|m| m.tx_hash).collect::<Vec<_>>();
        let ledger_txs = self
            .rpc_service
            .get_transactions_by_hash(tx_hashes.clone())
            .await?;
        let execution_infos = self
            .rpc_service
            .get_transaction_execution_infos_by_hash(tx_hashes)
            .await?;

        let data = indexer_txs
            .into_iter()
            .zip(ledger_txs)
            .zip(execution_infos)
            .map(|((_indexer_tx, ledger_tx_opt), execution_info_opt)| {
                let ledger_tx =
                    ledger_tx_opt.ok_or(anyhow::anyhow!("LedgerTransaction should have value"))?;
                let execution_info = execution_info_opt.ok_or(anyhow::anyhow!(
                    "TransactionExecutionInfo should have value"
                ))?;
                Ok(TransactionWithInfo::new(ledger_tx, execution_info))
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(data)
    }
}
