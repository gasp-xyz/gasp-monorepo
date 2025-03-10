import { Decimal } from 'decimal.js'
import { timeseries } from '../connector/RedisConnector.js'
import { TimestampedAmount } from '../schema/Models.js'
import { API_LIMIT } from '../util/Misc.js'
import * as redis from '../util/Redis.js'

const PREFIX = 'rates:'

const keyPair = (base: number | string, target: number | string) =>
  `${PREFIX}:${base}:${target}`

export const get = async (
  base: number | string,
  target: number | string,
  from: number,
  to: number,
  interval: number
): Promise<TimestampedAmount[]> => {
  const key = keyPair(base, target)
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
          'avg',
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
  return stored.map(([tsp, price]) => [tsp, new Decimal(price)])
}
