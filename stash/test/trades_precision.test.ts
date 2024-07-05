/* eslint-disable @typescript-eslint/no-non-null-assertion */
import { Decimal } from 'decimal.js'
import _ from 'lodash'
import { afterAll, beforeAll, chai, describe, it, vi } from 'vitest'
import { TimestampedAmountPool } from '../src/schema/Models'
import { KSM, parseNumber } from '../src/util/Chain'
import { ZERO } from '../src/util/Misc'
import * as fixtures from './fixtures'
chai.should()

type SwapData = [Decimal, Decimal]

describe.skip('test trades amounts', () => {
  const KSM_PRICE = 57.67
  const MGX_PRICE = 0.003910197566
  const asset = {
    pool: [4, 0],
    id: 5,
  }

  const store: Map<number, TimestampedAmountPool[]> = new Map()
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
        return id === KSM
          ? [[from, KSM_PRICE]]
          : fixtures.tradesFirstDay
              .filter((ev) => ev.timestamp >= from && ev.timestamp <= to)
              .map((e) => [e.timestamp, MGX_PRICE])
      }),
      getLatest: vi
        .fn()
        .mockImplementation((id) => _.last(fixtures.tradesFirstDay)?.timestamp),
    }))
    vi.doMock('../src/repository/ChainRepository', () => ({
      getAssets: vi.fn().mockReturnValue([asset]),
      getEvents: vi
        .fn()
        .mockImplementation((from, to) =>
          fixtures.tradesFirstDay
            .filter((ev) => ev.timestamp > from && ev.timestamp <= to)
            .slice(0, fixtures.LIMIT)
        ),
      LIMIT: fixtures.LIMIT,
    }))
  })

  afterAll(async () => {
    vi.unmock('../src/repository/TradeVolumeRepository')
    vi.unmock('../src/repository/PriceRepository')
    vi.unmock('../src/repository/ChainRepository')
  })

  it('test amounts for first day', () => {
    const amounts = fixtures.tradesFirstDay
      .map((e) =>
        e.events
          .flat()
          .filter((e) => e.section === 'xyk' && e.method === 'AssetsSwapped')
          .map((e) => {
            const ids = _.isArray(e.data[1])
              ? [
                  Number.parseInt(_.first(e.data[1])),
                  Number.parseInt(_.last(e.data[1])),
                ]
              : [Number.parseInt(e.data[1]), Number.parseInt(e.data[3])]
            const amounts = _.isArray(e.data[1])
              ? [
                  new Decimal(parseNumber(e.data[2])),
                  new Decimal(parseNumber(e.data[3])),
                ]
              : [
                  new Decimal(parseNumber(e.data[2])),
                  new Decimal(parseNumber(e.data[4])),
                ]
            return {
              pool: ids,
              amount: amounts,
            }
          })
          .map((e) => {
            if (_.isEqual(e.pool, asset.pool)) {
              return e.amount as SwapData
            } else if (_.isEqual(e.pool, [asset.pool[1], asset.pool[0]])) {
              return [e.amount[1], e.amount[0]] as SwapData
            } else {
              return [ZERO, ZERO] as SwapData
            }
          })
          .reduce(
            (acc, ev) => [acc[0].add(ev[0]), acc[1].add(ev[1])],
            [ZERO, ZERO]
          )
      )
      .reduce((acc, ev) => [acc[0].add(ev[0]), acc[1].add(ev[1])], [ZERO, ZERO])

    amounts[0]
      .div(new Decimal('1e12'))
      .mul(KSM_PRICE)
      .should.be.eql(new Decimal('28690.59604163819624'))
    amounts[1]
      .div(new Decimal('1e18'))
      .mul(MGX_PRICE)
      .should.be.eql(new Decimal('29439.4205813848103'))
  })

  it('test trade processor amounts', async () => {
    const trades = await import('../src/processing/TradeVolumeProcessorService')
    await trades.initService()

    const amounts = store
      .get(5)!
      .reduce(
        (acc, trade) => [acc[0].add(trade[1][0]), acc[1].add(trade[1][1])],
        [ZERO, ZERO]
      )

    amounts[0]
      .div(new Decimal('1e12'))
      .should.be.eql(new Decimal('28690.59604163819624'))
    amounts[1]
      .div(new Decimal('1e18'))
      .should.be.eql(new Decimal('29439.420581384810303'))
  })
})
