import { Decimal } from 'decimal.js'
import { ChainableCommander } from 'ioredis'
import { timeseries } from '../connector/RedisConnector.js'
import {
  TimestampedAmount,
  TimestampedBaseTargetAmount,
} from '../schema/Models.js'
import logger from '../util/Logger.js'
import { API_LIMIT } from '../util/Misc.js'
import * as redis from '../util/Redis.js'
import { Asset } from './ChainRepository.js'

const PREFIX = 'rates:'
const PREFIX_LATEST = PREFIX + 'latest:'

const keyPair = (base: number | string, target: number | string) =>
  `${PREFIX}:${base}:${target}`
const keyLatest = (id: number | string) => PREFIX_LATEST + id

export const getLatest = async (id: number) => {
  const latest = await timeseries.client.get(keyLatest(id))
  return latest ? Number.parseInt(latest) : 0
}

export const save = async (
  asset: Asset,
  rates: [TimestampedBaseTargetAmount[], TimestampedBaseTargetAmount[]],
  latest: number
) => {
  let timeseriesData = []
  const trx = timeseries.client.multi()

  if (rates.length > 0) {
    await createTable(asset, trx)
    timeseriesData = rates[0]
      .map(([tsp, b, t, v]) => [keyPair(b, t), tsp, v.toJSON()])
      .flat()
    trx.call('TS.MADD', timeseriesData)
    timeseriesData = rates[1]
      .map(([tsp, b, t, v]) => [keyPair(b, t), tsp, v.toJSON()])
      .flat()
    trx.call('TS.MADD', timeseriesData)
  }

  trx.set(keyLatest(asset.id), latest)

  const result = await redis.execute(trx)
  logger.debug(`PoolRatesRepository: saved ${result} entries for ${asset}`)
}

const createTable = async (asset: Asset, trx: ChainableCommander) => {
  const args: redis.CreateTableArgs = [
    [
      keyPair(asset.pool[0], asset.pool[1]),
      ['asset', asset.pool[0], 'asset', asset.pool[1]],
    ],
    [
      keyPair(asset.pool[1], asset.pool[0]),
      ['asset', asset.pool[1], 'asset', asset.pool[0]],
    ],
  ]
  for (const [key, label] of args) {
    const exist = await redis.hasKey(timeseries, key)
    if (!exist) {
      trx.call(
        'TS.CREATE',
        key,
        'RETENTION',
        '0',
        'DUPLICATE_POLICY',
        'FIRST',
        'LABELS',
        ...label
      )
    }
  }
}

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
