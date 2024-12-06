import { Chain } from 'viem'
import { anvil, arbitrumSepolia, base, holesky, localhost } from 'viem/chains'

export const CONFIG_TO_CHAIN = new Map<string, Chain>([
  ['localhost-arbitrum', anvil],
  ['localhost-ethereum', anvil],
  ['localhost-base', anvil],
  ['holeskky-ethereum', holesky],
  ['holesky-arbitrum', arbitrumSepolia],
  ['holesky-base', base],
  ['frontend-arbitrum', localhost],
  ['frontend-ethereum', localhost],
  ['frontend-base', localhost],
])
