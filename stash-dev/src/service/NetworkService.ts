import { redis } from '../connector/RedisConnector.js'

const NETWORK_LIST_KEY = 'affirmed_networks_list'
const TOKEN_LIST_KEY = 'affirmed_token_list'

type NativeToken = {
  name: string
  decimals: string
  symbol: string
}

enum Status {
  PENDING,
  APPROVED,
  INACTIVE,
}

type Network = {
  key: string
  name: string
  chainId: string
  layer: number
  parentChainId: number
  explorerUrl: string
  rpcUrl: string
  contractAddress: string
  nativeToken: NativeToken
  status: Status
}

type AffirmedToken = {
  name: string
  chainId: string
  decimals: string
  symbol: string
  address: string
  status: Status
}

const defaultNetworks: Array<Network> = [
  {
    key: 'default',
    name: 'default',
    chainId: 'default',
    layer: 1,
    parentChainId: 1,
    explorerUrl: 'default',
    rpcUrl: 'default',
    contractAddress: 'default',
    nativeToken: {
      name: 'default',
      decimals: 'default',
      symbol: 'default',
    },
    status: Status.APPROVED,
  },
]

const defaultTokens: Array<AffirmedToken> = [
  {
    name: 'default',
    chainId: 'default',
    decimals: 'default',
    symbol: 'default',
    address: 'default',
    status: Status.APPROVED,
  },
]

export const initService = async () => await initData()

export const initData = async () => {
  const networks = await listAffirmedNetworks()
  const tokens = await listAffirmedTokens()

  if (networks.length === 0) {
    await redis.client.set(NETWORK_LIST_KEY, JSON.stringify(defaultNetworks))
  }

  if (tokens.length === 0) {
    await redis.client.set(TOKEN_LIST_KEY, JSON.stringify(defaultTokens))
  }
}

export const listAffirmedNetworks = async (): Promise<Array<Network>> => {
  const redisNetworksResponse = await redis.client.get(NETWORK_LIST_KEY)
  return redisNetworksResponse === null
    ? []
    : (JSON.parse(redisNetworksResponse) as Array<Network>)
}

export const listAffirmedTokens = async (): Promise<Array<AffirmedToken>> => {
  const redisTokensListResponse = await redis.client.get(TOKEN_LIST_KEY)
  return redisTokensListResponse === null
    ? []
    : (JSON.parse(redisTokensListResponse) as Array<AffirmedToken>)
}
