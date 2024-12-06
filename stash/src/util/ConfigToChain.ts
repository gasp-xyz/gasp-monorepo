import { Chain } from 'viem'
import { anvil, arbitrumSepolia, base, holesky, localhost } from 'viem/chains'

export const CONFIG_TO_CHAIN = new Map<string, Chain>([
  ['localhost-arbitrum', anvil], //for localhost
  ['localhost-ethereum', anvil],
  ['localhost-base', anvil],
  ['holeskky-ethereum', holesky], //for testnet holesky
  ['holesky-arbitrum', arbitrumSepolia], //for testnet arbitrumSepolia
  ['holesky-base', base], //for testnet base
  ['frontend-arbitrum', localhost], //for frontend (fungible) use this
  ['frontend-ethereum', localhost],
  ['frontend-base', localhost],
])
