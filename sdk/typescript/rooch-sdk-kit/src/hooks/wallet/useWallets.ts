// Copyright (c) RoochNetwork
// SPDX-License-Identifier: Apache-2.0

import { useWalletStore } from './index'

/**
 * Retrieves all wallets
 */
export function useWallets() {
  return useWalletStore((state) => state.wallets)
}
