import { redis } from '../connector/RedisConnector.js'
import 'core-js'

const DEFAULT_STRING = 'default'
const TOKEN_ORDER_BUCKETS_KEY = 'token_order_buckets'

const defaultTokenOrderBucket = {
  bucket: DEFAULT_STRING,
  tokens: [DEFAULT_STRING],
} as TokenOrderBucket

export const initData = async (): Promise<void> => {
  const allChannels = await listTokenOrderBuckets()

  if (allChannels.size == 0) {
    await redis.client.hset(TOKEN_ORDER_BUCKETS_KEY, {
      [DEFAULT_STRING]: JSON.stringify(defaultTokenOrderBucket),
    })
  }
}

export const listTokenOrderBuckets = async (): Promise<
  Map<string, TokenOrderBucket>
> => {
  const allChannels = await redis.client.hgetall(TOKEN_ORDER_BUCKETS_KEY)

  return new Map(
    Object.values(allChannels)
      .map((it) => JSON.parse(it) as TokenOrderBucket)
      .map((it) => [it.bucket, it])
  )
}

export interface TokenOrderBucket {
  bucket: string
  tokens: string[]
  rank: number
}
