import { Chain } from 'viem'
import {
  anvil,
  arbitrumSepolia, arbitrum,
  baseSepolia, base,
  holesky, mainnet,
  localhost,
    sonic
} from 'viem/chains'

export const CONFIG_TO_CHAIN = new Map<string, Chain>([
  ['localhost-arbitrum', anvil],
  ['localhost-ethereum', anvil],
  ['localhost-base', anvil],
  ['testnet-ethereum', holesky],
  ['testnet-arbitrum', arbitrumSepolia],
  ['testnet-base', baseSepolia],
  ['prod-ethereum', mainnet],
  ['prod-arbitrum', arbitrum],
  ['prod-base', base],
  ['prod-sonic', sonic],
  ['frontend-arbitrum', localhost],
  ['frontend-ethereum', localhost],
  ['frontend-base', localhost],
])
