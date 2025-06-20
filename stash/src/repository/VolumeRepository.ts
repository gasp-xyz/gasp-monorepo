import { Decimal } from 'decimal.js'

import { redis } from '../connector/RedisConnector.js'
import { TimestampedAmount } from '../schema/Models.js'
import { API_LIMIT } from '../util/Misc.js'
import * as redisUtil from '../util/Redis.js'

const PREFIX_ASSET = 'volumes:asset:'
const PREFIX_POOL = 'volumes:pool:'

const keyAsset = (id: number | string) => PREFIX_ASSET + id
const keyPool = (id: number | string) => PREFIX_POOL + id

export const get = async (
  id: number | string,
  isPool: boolean,
  from: number,
  to: number,
  interval: number,
): Promise<TimestampedAmount[]> => {
  const key = isPool ? keyPool(id) : keyAsset(id)
  if (!(await redisUtil.hasKey(redis, key))) {
    return []
  }
  const start = from === 0 ? '-' : from
  const call =
    interval > 0
      ? redis.client.call(
          'TS.RANGE',
          key,
          start,
          to,
          'AGGREGATION',
          'avg',
          interval,
          'LIMIT',
          0,
          API_LIMIT,
        )
      : redis.client.call('TS.RANGE', key, start, to, 'LIMIT', 0, API_LIMIT)
  const stored = (await call) as [number, string][]
  return stored.map(([tsp, price]) => [tsp, new Decimal(price)])
}
