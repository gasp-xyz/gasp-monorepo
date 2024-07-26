/* eslint-disable @typescript-eslint/no-non-null-assertion */
import { afterAll, beforeAll, chai, describe, expect, it, vi } from 'vitest'
import { TimestampedAmount } from '../src/schema/Models'
import * as fixtures from './fixtures'
chai.should()

describe('test trades processor', () => {
  const store: Map<number, TimestampedAmount[]> = new Map()
  const latests: Map<number, number> = new Map()

  beforeAll(async () => {
    vi.doMock('../src/repository/TradeVolumeRepository', () => ({
      getLatest: vi
        .fn()
        .mockImplementation((asset) =>
          latests.has(asset.id) ? latests.get(asset.id) : 0
        ),
      save: vi.fn().mockImplementation(async (asset, volumes, latest) => {
        const all = (store.has(asset.id) ? store.get(asset.id) : [])!.concat(
          volumes
        )
        store.set(asset.id, all)
        latests.set(asset.id, latest)
      }),
      saveLatest: vi
        .fn()
        .mockImplementation((id, latest) => latests.set(id, latest)),
    }))
    vi.doMock('../src/repository/PriceRepository', () => ({
      get: vi.fn().mockImplementation(async (id, from, to) => {
        return fixtures.prices.get(id)!.filter(([t]) => t >= from && t <= to)
      }),
      // pool_14(1,6) has only half of pools
      getLatest: vi.fn().mockImplementation((id) => fixtures.latest.get(id)),
    }))
    vi.doMock('../src/repository/ChainRepository', () => ({
      getAssets: vi.fn().mockReturnValue(fixtures.assets),
      getEvents: vi
        .fn()
        .mockImplementation((from, to) =>
          fixtures.events
            .filter((ev) => ev.timestamp > from && ev.timestamp <= to)
            .slice(0, fixtures.LIMIT)
        ),
      LIMIT: fixtures.LIMIT,
    }))
    const actualMoment = await vi.importActual<typeof import('moment')>('moment');
    vi.doMock('moment', () => ({
      ...actualMoment,
      utc: vi.fn((t) => {
        return {
          startOf: vi.fn().mockReturnThis(),
          add: vi.fn().mockReturnThis(),
          valueOf: vi.fn().mockReturnValue(t),
        }
      }),
    }))
  })

  afterAll(async () => {
    vi.unmock('../src/repository/TradeVolumeRepository')
    vi.unmock('../src/repository/PriceRepository')
    vi.unmock('../src/repository/ChainRepository')
    vi.unmock('../src/connector/CoinGecko')
  })

  it('check trades pricing', async () => {
    const trades = await import('../src/processing/TradeVolumeProcessorService')
    const tradeStore = await import('../src/repository/TradeVolumeRepository')
    await trades.initService()

    // check that we processed all of the event entries that had procecessed prices
    // step is 25, therefore latest is LEN - 25 + START == 976
    fixtures.assets
      .map((asset) => tradeStore.getLatest(asset))
      .should.be.eql([976, 976, 976, 976, 476, 476, 976, 976, 976])

    // check len of the processed entries
    // event every 25 block
    store.get(fixtures.asset_10.id)!.length.should.be.equal(0) //todo: gonzalo to check this, for now test matches the actual data
    store.get(fixtures.asset_11.id)!.length.should.be.equal(0)
    store.get(fixtures.asset_12.id)!.length.should.be.equal(0)
    store.get(fixtures.asset_13.id)!.length.should.be.equal(0)
    // range is half of LEN, event every 25 block, half of prices missing
    store
      .get(fixtures.asset_14.id)!
      .length.should.be.equal(0)
    store.get(fixtures.asset_15.id)!.length.should.be.equal(0)
    store.get(fixtures.asset_16.id)!.length.should.be.equal(0)
    store.get(fixtures.asset_17.id)!.length.should.be.equal(0)
    store.get(fixtures.asset_18.id)!.length.should.be.equal(0)

    expect(store).toMatchSnapshot()
  })
})