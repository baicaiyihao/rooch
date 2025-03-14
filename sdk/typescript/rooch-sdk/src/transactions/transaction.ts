// Copyright (c) RoochNetwork
// SPDX-License-Identifier: Apache-2.0

import { bcs } from '../bcs/index.js'
import { Authenticator } from '../crypto/index.js'
import { address, Bytes, u64 } from '../types/index.js'

import { MoveAction, TransactionData } from './transactionData.js'
import { CallFunctionArgs } from './types.js'

export class Transaction {
  private data: TransactionData | undefined
  private auth: Authenticator | undefined
  private info: string | undefined

  callFunction(
    input: {
      info?: string
    } & CallFunctionArgs,
  ) {
    this.info = input.info
    this.data = new TransactionData(
      MoveAction.newCallFunction(input),
      input.maxGas ? BigInt(input.maxGas) : undefined,
    )
  }

  getInfo() {
    return this.info
  }

  getMaxGas() {
    return this.getData().maxGas
  }

  setMaxGas(input: number) {
    this.getData().maxGas = BigInt(input)
  }

  setSender(input: address) {
    this.getData().sender = input
  }

  setAuth(input: Authenticator) {
    this.auth = input
  }

  setChainId(input: u64) {
    this.getData().chainId = input
  }

  setSeqNumber(input: u64) {
    this.getData().sequenceNumber = input
  }

  hashData(): Bytes {
    return this.getData().hash()
  }

  encodeData() {
    return this.data!.encode()
  }

  encode() {
    return bcs.RoochTransaction.serialize({
      data: this.data!.encode().toBytes(),
      auth: this.auth!.encode(),
    })
  }

  private getData() {
    this.isValid()
    return this.data!
  }

  private isValid() {
    if (!this.data) {
      throw new Error('Transaction data is not initialized. Call action first.')
    }
  }
}
