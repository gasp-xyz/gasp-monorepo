import { Decimal } from 'decimal.js'
import { timeseries } from '../connector/RedisConnector.js'
import { TimestampedAmount } from '../schema/Models.js'
import logger from '../util/Logger.js'
import { API_LIMIT } from '../util/Misc.js'
import * as redis from '../util/Redis.js'

const PREFIX = 'price:asset:'
const PREFIX_LATEST = 'price:pool:latest:'

const keyPrice = (id: number | string) => PREFIX + id
const keyLatest = (id: number | string) => PREFIX_LATEST + id

export const getLatest = async (id: number) => {
  const latest = await timeseries.client.get(keyLatest(id))
  return latest ? Number.parseInt(latest) : 0
}

export const saveLatest = async (id: number, latest: number) => {
  await timeseries.client.set(keyLatest(id), latest)
}

export const save = async (
  poolId: number | string,
  id: number,
  prices: TimestampedAmount[],
  latest: number
) => {
  const trx = timeseries.client.multi()
  const key = keyPrice(id)

  if (prices.length > 0) {
    const exist = (await timeseries.client.exists(key)) > 0
    if (!exist) {
      trx.call(
        'TS.CREATE',
        key,
        'RETENTION',
        '0',
        'DUPLICATE_POLICY',
        'FIRST',
        'LABELS',
        'asset',
        id
      )
    }

    const timeseriesData = prices.map(([t, p]) => [key, t, p.toJSON()]).flat()
    trx.call('TS.MADD', timeseriesData)
  }

  trx.set(keyLatest(poolId), latest)

  await trx.exec()
  logger.debug(`PricesRepository: saved prices for ${id}`)
}

export const get = async (
  id: number | string,
  from: number,
  to: number,
  interval: number = 0
): Promise<TimestampedAmount[]> => {
  const key = keyPrice(id)
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
