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

const PREFIX_ASSET = 'volumes:asset:'
const PREFIX_POOL = 'volumes:pool:'
const PREFIX_LATEST = PREFIX_POOL + 'latest:'

const keyAsset = (id: number | string) => PREFIX_ASSET + id
const keyPool = (id: number | string) => PREFIX_POOL + id
const keyLatest = (id: number | string) => PREFIX_LATEST + id

export const getLatest = async (id: number) => {
  const latest = await timeseries.client.get(keyLatest(id))
  return latest ? Number.parseInt(latest) : 0
}

export const save = async (
  asset: Asset,
  volumes: TimestampedAmountPool[],
  latest: number
) => {
  let timeseriesData = []
  const trx = timeseries.client.multi()

  if (volumes.length > 0) {
    await createTable(asset, trx)
    timeseriesData = volumes
      .map(([t, v]) => [keyPool(ASSET_ALL.id), t, v[0].add(v[1]).toJSON()])
      .flat()
    trx.call('TS.MADD', timeseriesData)
    timeseriesData = volumes
      .map(([t, v]) => [keyPool(asset.id), t, v[0].add(v[1]).toJSON()])
      .flat()
    trx.call('TS.MADD', timeseriesData)
    timeseriesData = volumes
      .map(([t, [v0, v1]]) => [keyAsset(asset.pool[0]), t, v0.toJSON()])
      .flat()
    trx.call('TS.MADD', timeseriesData)
    timeseriesData = volumes
      .map(([t, [v0, v1]]) => [keyAsset(asset.pool[1]), t, v1.toJSON()])
      .flat()
    trx.call('TS.MADD', timeseriesData)
  }

  trx.set(keyLatest(asset.id), latest)

  const result = await redis.execute(trx)
  logger.debug(`VolumeRepository: saved ${result} entries for ${asset}`)
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
