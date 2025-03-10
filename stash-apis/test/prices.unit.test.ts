/* eslint-disable @typescript-eslint/no-non-null-assertion */
import { afterAll, beforeAll, chai, describe, expect, it, vi } from 'vitest'
import { TimestampedAmount } from '../src/schema/Models'
import * as fixtures from './fixtures'
chai.should()

describe('test pricing processor', () => {
  const store: Map<number, TimestampedAmount[]> = new Map()
  const latests: Map<number, number> = new Map()

  beforeAll(async () => {
    vi.doMock('../src/repository/PriceRepository', () => ({
      getLatest: vi
        .fn()
        .mockImplementation((id) =>
          id === fixtures.BASE
            ? fixtures.LEN
            : latests.has(id)
            ? latests.get(id)
            : 0
        ),
      save: vi.fn().mockImplementation(async (poolId, id, prices, latest) => {
        const all = (store.has(id) ? store.get(id) : [])!.concat(prices)
        store.set(id, all)
        latests.set(poolId, latest)
      }),
      get: vi.fn().mockImplementation(async (id, from, to) => {
        const arr =
          id === fixtures.BASE
            ? fixtures.basePrices
            : !store.has(id)
            ? []
            : store.get(id)
        return arr!.filter(([t]) => t >= from && t <= to)
      }),
      saveLatest: vi
        .fn()
        .mockImplementation((id, latest) => latests.set(id, latest)),
    }))
    vi.doMock('../src/repository/ChainRepository', () => ({
      getAssets: vi.fn().mockReturnValue(fixtures.assets),
      getPools: vi.fn().mockImplementation((id, from, to) =>
        fixtures.pools
          .get(id)!
          .filter((p) => p.timestamp > from && p.timestamp <= to)
          .slice(0, fixtures.LIMIT)
      ),
      getLatest: vi.fn().mockReturnValue({ timestamp: fixtures.LEN }),
      LIMIT: fixtures.LIMIT,
    }))
    vi.doMock('../src/connector/CoinGecko', () => ({
      getCoinHistory: vi.fn().mockResolvedValue([]),
    }))
  })

  afterAll(async () => {
    vi.unmock('../src/repository/PriceRepository')
    vi.unmock('../src/repository/ChainRepository')
    vi.unmock('../src/connector/CoinGecko')
  })
})
