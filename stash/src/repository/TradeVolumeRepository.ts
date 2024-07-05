import { Decimal } from 'decimal.js'
import { ChainableCommander } from 'ioredis'
import { timeseries } from '../connector/RedisConnector.js'
import {
  ASSET_ALL,
  TimestampedAmount,
  TimestampedAmountPool,
} from '../schema/Models.js'
import logger from '../util/Logger.js'
import { API_LIMIT } from '../util/Misc.js'
import * as redis from '../util/Redis.js'
import { Asset } from './ChainRepository.js'

const PREFIX_ASSET = 'trades:asset:'
const PREFIX_POOL = 'trades:pool:'
const PREFIX_LATEST = PREFIX_POOL + 'latest:'

const keyAsset = (id: number | string) => PREFIX_ASSET + id
const keyPool = (id: number | string) => PREFIX_POOL + id
const keyLatest = (id: number | string) => PREFIX_LATEST + id

export const getLatest = async (asset: Asset) => {
  const latest = await timeseries.client.get(keyLatest(asset.id))
  return latest ? Number.parseInt(latest) : asset.createdAt
}

export const save = async (
  asset: Asset,
  trades: TimestampedAmountPool[],
  latest: number
) => {
  let timeseriesData = []
  const trx = timeseries.client.multi()

  if (trades.length > 0) {
    await createTable(asset, trx)
    timeseriesData = trades
      .map(([t, [a0, a1]]) => [keyAsset(asset.pool[0]), t, a0.toJSON()])
      .flat()
    trx.call('TS.MADD', timeseriesData)
    timeseriesData = trades
      .map(([t, [a0, a1]]) => [keyAsset(asset.pool[1]), t, a1.toJSON()])
      .flat()
    trx.call('TS.MADD', timeseriesData)

    timeseriesData = trades
      .map(([t, [a0, a1]]) => [
        keyPool(ASSET_ALL.id),
        t,
        a0.add(a1).div(new Decimal(2)).toJSON(),
      ])
      .flat()
    trx.call('TS.MADD', timeseriesData)
    timeseriesData = trades
      .map(([t, [a0, a1]]) => [
        keyPool(asset.id),
        t,
        a0.add(a1).div(new Decimal(2)).toJSON(),
      ])
      .flat()
    trx.call('TS.MADD', timeseriesData)
  }

  trx.set(keyLatest(asset.id), latest)

  const result = await redis.execute(trx)
  logger.debug(`TradeVolumeRepository: saved ${result} entries for ${asset}`)
}

const createTable = async (asset: Asset, trx: ChainableCommander) => {
  const args: redis.CreateTableArgs = [
    [keyPool(ASSET_ALL.id), ['pool', ASSET_ALL.id]],
    [
      keyPool(asset.id),
      ['pool', asset.id, 'asset', asset.pool[0], 'asset', asset.pool[1]],
    ],
    [keyAsset(asset.pool[0]), ['asset', asset.pool[0]]],
    [keyAsset(asset.pool[1]), ['asset', asset.pool[1]]],
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
        'SUM',
        'LABELS',
        ...label
      )
    }
  }
}

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
  return stored.map(([tsp, price]) => [tsp, new Decimal(price)])
}
