import { Decimal } from 'decimal.js'
import { ChainableCommander } from 'ioredis'
import { timeseries } from '../connector/RedisConnector.js'
import { TimestampedAmount } from '../schema/Models.js'
import { API_LIMIT } from '../util/Misc.js'
import * as redis from '../util/Redis.js'

const PREFIX_ASSET = 'trades:asset:'
const PREFIX_POOL = 'trades:pool:'

const keyAsset = (id: number | string) => PREFIX_ASSET + id
const keyPool = (id: number | string) => PREFIX_POOL + id

export const get = async (
  id: number | string,
  isPool: boolean,
  from: number,
  to: number,
  interval: number
): Promise<TimestampedAmount[]> => {
  const key = isPool ? keyPool(id) : keyAsset(id)
  if (!(await redis.hasKey(timeseries, key))) {
    return []
  }
  const start = from === 0 ? '-' : from
  const call =
    interval > 0
      ? timeseries.client.call(
          'TS.RANGE',
          key,
          start,
          to,
          'AGGREGATION',
          'sum',
          interval,
          'LIMIT',
          0,
          API_LIMIT
        )
      : timeseries.client.call(
          'TS.RANGE',
          key,
          start,
          to,
          'LIMIT',
          0,
          API_LIMIT
        )
  const stored = (await call) as [number, string][]
    if (isPool) {
        return stored.map(([tsp, price]) => [tsp, new Decimal(price).dividedBy(2)])
    } else {
        return stored.map(([tsp, price]) => [tsp, new Decimal(price)])
    }}
