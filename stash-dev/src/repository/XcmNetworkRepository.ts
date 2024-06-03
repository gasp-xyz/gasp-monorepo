import { redis } from '../connector/RedisConnector.js'
import 'core-js'

const DEFAULT_STRING = 'default'
const CHANNELS_V2_KEY = 'xcm_channels_v2'
const TOKENS_V2_KEY = 'xcm_tokens_v2'
const defaultChannel = {
  channelId: DEFAULT_STRING,
  name: DEFAULT_STRING,
  status: DEFAULT_STRING,
  unitWeightCost: DEFAULT_STRING,
  xcmTransferWeight: DEFAULT_STRING,
  url: DEFAULT_STRING,
  xcmVersion: DEFAULT_STRING,
  chainType: DEFAULT_STRING,
} as XcmChannel

const defaultToken = {
  tokenId: DEFAULT_STRING,
  name: DEFAULT_STRING,
  symbol: DEFAULT_STRING,
  decimals: DEFAULT_STRING,
  location: '{}',
  feePerSec: DEFAULT_STRING,
  channelId: DEFAULT_STRING,
  permissions: [DEFAULT_STRING],
  extrinsicPath: DEFAULT_STRING,
  balancePath: DEFAULT_STRING,
  destinationTokenId: '{}',
  existentialDeposit: DEFAULT_STRING,
} as XcmChannelToken

export const initNetwork = async (): Promise<void> => {
  const allChannels = await listAllChannels()
  const allTokens = await listAllTokens()

  if (allChannels.size == 0) {
    await redis.client.hset(CHANNELS_V2_KEY, {
      [DEFAULT_STRING]: JSON.stringify(defaultChannel),
    })
  }
  if (allTokens.size == 0) {
    await redis.client.hset(TOKENS_V2_KEY, {
      [DEFAULT_STRING]: JSON.stringify(defaultToken),
    })
  }
}

export const listAllChannels = async (): Promise<Map<string, XcmChannel>> => {
  const allChannels = await redis.client.hgetall(CHANNELS_V2_KEY)

  return new Map(
    Object.values(allChannels)
      .map((it) => JSON.parse(it) as XcmChannel)
      .map((it) => [it.channelId, it])
  )
}

export const listAllTokens = async (): Promise<
  Map<string, XcmChannelTokenDto>
> => {
  const allTokens = await redis.client.hgetall(TOKENS_V2_KEY)

  return new Map(
    Object.values(allTokens)
      .map((it) => JSON.parse(it) as XcmChannelToken)
      .map((it) => {
        return {
          tokenId: it.tokenId,
          name: it.name,
          symbol: it.symbol,
          decimals: it.decimals,
          location: Function('return ' + it.location)(),
          feePerSec: it.feePerSec,
          channelId: it.channelId,
          permissions: it.permissions,
          extrinsicPath: it.extrinsicPath,
          balancePath: it.balancePath,
          destinationTokenId: Function('return ' + it.destinationTokenId)(),
          existentialDeposit: it.existentialDeposit,
        } as XcmChannelTokenDto
      })
      .map((it) => [tokenIdFromToken(it), it])
  )
}

export interface XcmChannel {
  channelId: string
  name: string
  status: string
  unitWeightCost: string
  xcmTransferWeight: string
  url: string
  xcmVersion: string
  chainType: string
}

export interface XcmChannelTokenDto {
  tokenId: string
  name: string
  symbol: string
  decimals: string
  location: any
  feePerSec: string
  channelId: string
  permissions: string[]
  extrinsicPath: string
  balancePath: string
  destinationTokenId: any
  existentialDeposit: string
}

export interface XcmChannelToken {
  tokenId: string
  name: string
  symbol: string
  decimals: string
  location: string
  feePerSec: string
  channelId: string
  permissions: string[]
  extrinsicPath: string
  balancePath: string
  destinationTokenId: string
  existentialDeposit: string
}

export const tokenIdFromToken = (token: XcmChannelToken): string => {
  return tokenIdFromIds(token.channelId, token.tokenId)
}

export const tokenIdFromIds = (channelId: string, tokenId: string): string => {
  return channelId + ':' + tokenId
}
