import { pairs, tickers } from '../src/service/CoingeckoListingService'
import { describe, expect, it, vi, beforeAll, afterAll } from 'vitest'
import { Decimal } from 'decimal.js'
import app from '../src/app'

vi.mock('gasp-sdk')

describe('[CoinGecko listing]', () => {
  beforeAll(() => {
    vi.mock('../src/util/Listing', async () => {
      const actual = await vi.importActual('../src/util/Listing')
      return {
        ...actual,
        getPoolVolumeInUsd: vi.fn().mockImplementation((poolId: string) => {
          return new Promise((resolve) => {
            const volumePoolResponse = {
              '5': [[1691280000000, new Decimal('867.4535759973365')]],
              '6': [[1691280000000, new Decimal('6.896015943828684')]],
              '7': [[1690761600000, new Decimal('190.68743584230737')]],
              '8': [[1690761600000, new Decimal('190.68743584230737')]],
            }
            resolve(volumePoolResponse[poolId] || [])
          })
        }),
        getTokenPrices: vi.fn().mockImplementation((tokenId: string) => {
          return new Promise((resolve) => {
            const tokenResponses = {
              '1': [[1691280000000, new Decimal('21.54441188086054')]],
              '0': [[1691366400000, new Decimal('0.0005226090144381267')]],
            }

            resolve(tokenResponses[tokenId] || [])
          })
        }),
        getLiquidityPoolInUsd: vi.fn().mockImplementation((poolId: string) => {
          return new Promise((resolve) => {
            const volumePoolResponse = {
              '5': [[1691366400000, new Decimal('508786.95209646935')]],
              '6': [[1691366400000, new Decimal('57038.15488204146')]],
              '7': [[1691366400000, new Decimal('126.74932683307718')]],
              '8': [[1691366400000, new Decimal('127.74932683307718')]],
            }
            resolve(volumePoolResponse[poolId] || [])
          })
        }),
      }
    })
  })

  afterAll(async () => {
    vi.unmock('../src/util/Listing')
  })

  it('should mock the pairs endpoint method', async () => {
    const expectedResponse = [
      {
        ticker_id: 'GASPV2_GETH',
        base: 'GASPV2',
        target: 'GETH',
        pool_id: '5',
      },
      {
        ticker_id: 'L1Asset_GETH',
        base: 'L1Asset',
        target: 'GETH',
        pool_id: '6',
      },
      {
        ticker_id: 'L1Asset_GASPV2',
        base: 'L1Asset',
        target: 'GASPV2',
        pool_id: '7',
      },
      {
        ticker_id: 'GASPV2_L1Asset',
        base: 'GASPV2',
        target: 'L1Asset',
        pool_id: '8',
      },
    ]
    const results = await pairs()
    expect(results).deep.equal(expectedResponse)
    results.forEach((element) => {
      expect(element).toHaveProperty('ticker_id')
      expect(element).toHaveProperty('base')
      expect(element).toHaveProperty('target')
      expect(element).toHaveProperty('pool_id')
    })
  })

  it('should mock the tickers endpoint method', async () => {
    const expectedResponse = [
      {
        base_currency: 'GASPV2',
        base_volume: '40.263506880313510874',
        last_price: '41375.099191776760995251',
        liquidity_in_usd: '508786.95209646935',
        pool_id: '5',
        target_currency: 'GETH',
        target_volume: '1659851.9199481527807',
        ticker_id: 'GASPV2_GETH',
      },
      {
        base_currency: 'L1Asset',
        base_volume: '0',
        last_price: '0',
        liquidity_in_usd: '57038.15488204146',
        pool_id: '6',
        target_currency: 'GETH',
        target_volume: '13195.363557291116508',
        ticker_id: 'L1Asset_GETH',
      },
      {
        base_currency: 'L1Asset',
        base_volume: '0',
        last_price: '0',
        liquidity_in_usd: '126.74932683307718',
        pool_id: '7',
        target_currency: 'GASPV2',
        target_volume: '8.8509000336977783626',
        ticker_id: 'L1Asset_GASPV2',
      },
      {
        base_currency: 'GASPV2',
        base_volume: '8.8509000336977783626',
        last_price: '0',
        liquidity_in_usd: '127.74932683307718',
        pool_id: '8',
        target_currency: 'L1Asset',
        target_volume: '0',
        ticker_id: 'GASPV2_L1Asset',
      },
    ]
    const results = await tickers()
    expect(results).deep.equal(expectedResponse)
    results.forEach((element) => {
      expect(element).toHaveProperty('ticker_id')
      expect(element).toHaveProperty('base_currency')
      expect(element).toHaveProperty('target_currency')
      expect(element).toHaveProperty('last_price')
      expect(element).toHaveProperty('base_volume')
      expect(element).toHaveProperty('target_volume')
      expect(element).toHaveProperty('pool_id')
      expect(element).toHaveProperty('liquidity_in_usd')
    })
  })
})
