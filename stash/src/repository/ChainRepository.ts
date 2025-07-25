import { Decimal } from 'decimal.js'
import _ from 'lodash'

import { redis } from '../connector/RedisConnector.js'
import { Event } from '../scraper/BlockScraper.js'
import logger from '../util/Logger.js'
import * as ratesStore from './PoolRatesRepository.js'

const PREFIX = 'chain:'
const PREFIX_POOL = PREFIX + 'pool:'
const KEY_ASSETS = PREFIX + 'assets'
const KEY_EVENTS = PREFIX + 'events'
const KEY_LATEST = PREFIX + 'latest'

const keyPool = (id: number) => PREFIX_POOL + id

const ROLLUP_STARTER_BLOCK = 0 //8 July 2024
const BATCH = 300 * 6 * 1000 // ~= 1 hour of blocks

export const LIMIT = 10_000

const setStarterBlock = () => {
  return ROLLUP_STARTER_BLOCK
}

export interface Latest {
  timestamp: number
  block: number
}

let batchElapsed = BATCH
export const setBatchMode = (current: number, latest: number) => {
  batchElapsed = latest - current < 5 ? 0 : BATCH
}

export const getLatest = async (): Promise<Latest> => {
  const latest = await redis.client.get(KEY_LATEST)
  const json = _.isNull(latest)
    ? {
        timestamp: 0,
        block: setStarterBlock(),
      }
    : JSON.parse(latest)
  return {
    timestamp: Number.parseInt(json.timestamp),
    block: Number.parseInt(json.block),
  }
}

export const saveLatest = async (latest: Latest) => {
  await redis.client.set(KEY_LATEST, JSON.stringify(latest))
}

export const saveAssets = async (assets: Asset[]) => {
  await redis.client.set(KEY_ASSETS, JSON.stringify(assets))
}

let batchEvents: EventEntry[] = []
export const saveEvents = async (event: EventEntry) => {
  batchEvents.push(event)
  const first = batchEvents[0].timestamp
  if (event.timestamp - first >= batchElapsed) {
    logger.debug(`saving batch of ${batchEvents.length} events`)
    const toStore = batchEvents
      .map((e) => [e.timestamp, JSON.stringify(e)])
      .flat()
    await redis.client.zadd(KEY_EVENTS, ...toStore)
    batchEvents = []
  }
}

export const getEvents = async (
  latest: number,
  to: number,
): Promise<EventEntry[]> => {
  const stored = await redis.client.zrangebyscore(
    KEY_EVENTS,
    `(${latest}`,
    to,
    'LIMIT',
    0,
    LIMIT,
  )
  return stored
    .map((s) => JSON.parse(s))
    .map((json) => {
      return {
        block: Number.parseInt(json.block),
        timestamp: Number.parseInt(json.timestamp),
        events: json.events,
      }
    })
}

let batchPools: PoolEntry[][] = []
export const savePools = async (pools: PoolEntry[]) => {
  batchPools.push(pools)
  const first = batchPools[0][0].timestamp
  if (pools[0].timestamp - first >= batchElapsed) {
    logger.debug(`saving batch of ${batchPools.flat().length} pools`)
    const trx = redis.client.multi()
    batchPools
      .flat()
      .forEach((p) => trx.zadd(keyPool(p.id), p.timestamp, JSON.stringify(p)))
    await trx.exec()
    batchPools = []
  }
}

export const getPools = async (
  poolId: number,
  from: number,
  to: number = -1,
): Promise<PoolEntry[]> => {
  const max = to === -1 ? '+inf' : to
  const stored = await redis.client.zrangebyscore(
    keyPool(poolId),
    `(${from}`,
    max,
    'LIMIT',
    0,
    LIMIT,
  )
  return stored
    .map((s) => JSON.parse(s))
    .map((p) => {
      return {
        id: Number.parseInt(p.id),
        assets: [Number.parseInt(p.assets[0]), Number.parseInt(p.assets[1])],
        block: Number.parseInt(p.block),
        timestamp: Number.parseInt(p.timestamp),
        amounts: [new Decimal(p.amounts[0]), new Decimal(p.amounts[1])],
      }
    })
}

export const getAssets = async (): Promise<Asset[]> => {
  const assets = await redis.client.get(KEY_ASSETS)
  return _.isNull(assets)
    ? []
    : JSON.parse(assets).map((a) => {
        const asset: Asset = {
          id: Number.parseInt(a.id),
          idx: a.idx,
          pool: [Number.parseInt(a.pool[0]), Number.parseInt(a.pool[1])],
          poolx: a.poolx,
          createdAt: Number.parseInt(a.createdAt),
        }
        asset.toString = () => `${asset.id}->[${asset.pool}]`
        return asset
      })
}

export const removeProcessedPoolRates = async (): Promise<void> => {
  const assets = await getAssets()
  
  logger.info(`Starting pool rates cleanup for ${assets.length} assets`)
  
  const trx = redis.client.multi()
  for (const asset of assets) {
    const latestTimestamp = await ratesStore.getLatest(asset.id)
    const cutoffTimestamp = latestTimestamp - (24 * 60 * 60 * 1000) // 24 hours ago in ms
    const poolKey = keyPool(asset.id)
    trx.zremrangebyscore(poolKey, '-inf', cutoffTimestamp)
  }
  
  const results = await trx.exec()
  const removedCount = results?.reduce((sum, result) => sum + (result?.[1] as number || 0), 0) || 0
  
  logger.info(`Pool rates cleanup completed. Removed ${removedCount} entries from ${assets.length} pools`)
}

export interface Asset {
  id: number
  idx: string
  pool: [number, number]
  poolx: string
  createdAt: number
  toString(): string
}

export interface PoolEntry {
  id: number
  assets: [number, number]
  block: number
  timestamp: number
  amounts: [Decimal, Decimal]
}

export interface EventEntry {
  block: number
  timestamp: number
  events: Event[][]
}
