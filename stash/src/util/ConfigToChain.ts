import { Chain } from 'viem'
import {
  anvil,
  arbitrum,
  arbitrumSepolia,
  base,
  baseSepolia,
  holesky,
  localhost,
  mainnet,
  sonic,
} from 'viem/chains'

export const CONFIG_TO_CHAIN = new Map<string, Chain>([
  ['localhost-arbitrum', anvil],
  ['localhost-ethereum', anvil],
  ['localhost-base', anvil],
  ['localhost-sonic', localhost],
  ['testnet-ethereum', holesky],
  ['testnet-arbitrum', arbitrumSepolia],
  ['testnet-base', baseSepolia],
  ['testnet-sonic', localhost],
  ['prod-ethereum', mainnet],
  ['prod-arbitrum', arbitrum],
  ['prod-base', base],
  ['prod-sonic', sonic],
  ['frontend-arbitrum', localhost],
  ['frontend-ethereum', localhost],
  ['frontend-base', localhost],
  ['frontend-sonic', localhost],
])
