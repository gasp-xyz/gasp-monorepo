import { Chain } from 'viem'
import {
  anvil,
  arbitrumSepolia, arbitrum,
  baseSepolia, base,
  holesky, mainnet,
  localhost,
} from 'viem/chains'

export const CONFIG_TO_CHAIN = new Map<string, Chain>([
  ['localhost-arbitrum', anvil],
  ['localhost-ethereum', anvil],
  ['localhost-base', anvil],
  ['testnet-ethereum', holesky],
  ['testnet-arbitrum', arbitrumSepolia],
  ['testnet-base', baseSepolia],
  ['mainnet-ethereum', mainnet],
  ['mainnet-arbitrum', arbitrum],
  ['mainnet-base', base],
  ['frontend-arbitrum', localhost],
  ['frontend-ethereum', localhost],
  ['frontend-base', localhost],
])
