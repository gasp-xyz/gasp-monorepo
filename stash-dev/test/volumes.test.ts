/* eslint-disable @typescript-eslint/no-non-null-assertion */
import { afterAll, beforeAll, chai, describe, expect, it, vi } from 'vitest'
import { TimestampedAmount } from '../src/schema/Models'
import * as fixtures from './fixtures'
chai.should()

describe('test volumes processor', () => {
  const store: Map<number, TimestampedAmount[]> = new Map()
  const latests: Map<number, number> = new Map()

  beforeAll(async () => {
    vi.doMock('../src/repository/VolumeRepository', () => ({
      getLatest: vi.fn().mockImplementation((id) => (latests.has(id) ? latests.get(id) : 0)),
      save: vi.fn().mockImplementation(async (asset, volumes, latest) => {
        const all = (store.has(asset.id) ? store.get(asset.id) : [])!.concat(volumes)
        store.set(asset.id, all)
        latests.set(asset.id, latest)
      }),
      saveLatest: vi.fn().mockImplementation((id, latest) => latests.set(id, latest)),
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
      getPools: vi.fn().mockImplementation((id, from, to) =>
        fixtures.pools
          .get(id)!
          .filter((p) => p.timestamp > from && p.timestamp <= to)
          .slice(0, fixtures.LIMIT)
      ),
      LIMIT: fixtures.LIMIT,
    }))
  })

  afterAll(async () => {
    vi.unmock('../src/repository/VolumeRepository')
    vi.unmock('../src/repository/PriceRepository')
    vi.unmock('../src/repository/ChainRepository')
    vi.unmock('../src/connector/CoinGecko')
  })

  it('check volumes pricing', async () => {
    const volumes = await import('../src/processing/TVLProcessorService')
    const volumeStore = await import('../src/repository/VolumeRepository')
    await volumes.initService()

    // check that we processed all of the pool entries that had procecessed prices
    fixtures.assets.map((asset) => volumeStore.getLatest(asset.id)).should.be.eql([...fixtures.latest.values()])

    // check len of the processed entries
    store.get(fixtures.asset_10.id)!.length.should.be.equal(fixtures.LEN)
    store.get(fixtures.asset_11.id)!.length.should.be.equal(fixtures.LEN / 2)
    store.get(fixtures.asset_12.id)!.length.should.be.equal(fixtures.LEN / 2)
    store.get(fixtures.asset_13.id)!.length.should.be.equal(0)
    store.get(fixtures.asset_14.id)!.length.should.be.equal(fixtures.LEN / 4)
    store.get(fixtures.asset_15.id)!.length.should.be.equal(0)
    store.get(fixtures.asset_16.id)!.length.should.be.equal(fixtures.LEN / 4)
    store.get(fixtures.asset_17.id)!.length.should.be.equal(0)
    store.get(fixtures.asset_18.id)!.length.should.be.equal(fixtures.LEN / 2)

    expect(store).toMatchSnapshot()
  })
})
