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
  it('check pools pricing', async () => {
    const prices = await import('../src/processing/PriceProcessorService')
    const pricesStore = await import('../src/repository/PriceRepository')
    await prices.initService(fixtures.BASE)

    // check that we processed all of the pool entries
    fixtures.assets
      .map((asset) => pricesStore.getLatest(asset.id))
      .should.be.eql([...fixtures.latest.values()])

    // check len of the processed entries
    //asset_10 (1000 pools)
    store.get(1)!.length.should.be.equal(fixtures.LEN)
    // asset_11(500 pools) + asset_18 dependent of 7 (750 prices)
    store.get(2)!.length.should.be.equal((fixtures.LEN / 4)) //todo: gonzalo to check this
    // dependent on asset 2
    store.get(3)!.length.should.be.equal(0)
    // zeros
    store.get(5)!.length.should.be.equal(0)
    // half of range, asset_14(250 pools) + asset_15(250 pools) should add up
    store.get(6)!.length.should.be.equal(fixtures.LEN / 4)
    // asset_18 dependent of asset 2(500 prices) + asset_16 dependent of 6 (500 prices)
    store.get(7)!.length.should.be.equal(fixtures.LEN / 4)
    // no prices for asset_17 -> 9,8
    expect(store.get(8)).to.be.undefined
    expect(store.get(9)).to.be.undefined

    expect(store).toMatchSnapshot()
  })
})
