import { Decimal } from 'decimal.js'
import _ from 'lodash'
import { redis } from '../connector/RedisConnector.js'

const PREFIX = 'chain:'
const KEY_ASSETS = PREFIX + 'assets'

export const LIMIT = 10_000

export interface Latest {
  timestamp: number
  block: number
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

export interface Event {
  method: string
  section: string
  data: object
  index: string
}
