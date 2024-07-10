import { pairs, tickers } from '../src/service/CoingeckoListingService'
import { describe, expect, it, vi, beforeAll, afterAll } from 'vitest'
import { Decimal } from 'decimal.js'

vi.mock('@mangata-finance/sdk')

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
              '8': [[1691280000000, new Decimal('6.896015943828684')]],
              '9': [[1690761600000, new Decimal('190.68743584230737')]],
            }
            resolve(volumePoolResponse[poolId] || [])
          })
        }),
        getTokenPrices: vi.fn().mockImplementation((tokenId: string) => {
          return new Promise((resolve) => {
            const tokenResponses = {
              '4': [[1691280000000, new Decimal('21.54441188086054')]],
              '0': [[1691366400000, new Decimal('0.0005226090144381267')]],
              '7': [[1691366400000, new Decimal('0.003762788700625028')]],
            }

            resolve(tokenResponses[tokenId] || [])
          })
        }),
        getLiquidityPoolInUsd: vi.fn().mockImplementation((poolId: string) => {
          return new Promise((resolve) => {
            const volumePoolResponse = {
              '5': [[1691366400000, new Decimal('508786.95209646935')]],
              '8': [[1691366400000, new Decimal('57038.15488204146')]],
              '9': [[1691366400000, new Decimal('126.74932683307718')]],
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
      { ticker_id: 'KSM_MGX', base: 'KSM', target: 'MGX', pool_id: '5' },
      { ticker_id: 'MGX_TUR', base: 'MGX', target: 'TUR', pool_id: '8' },
      { ticker_id: 'KSM_TUR', base: 'KSM', target: 'TUR', pool_id: '9' },
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
        ticker_id: 'KSM_MGX',
        base_currency: 'KSM',
        target_currency: 'MGX',
        last_price: '41375.099191776760995251',
        base_volume: '40.263506880313510874',
        target_volume: '1659851.9199481527807',
        pool_id: '5',
        liquidity_in_usd: '508786.95209646935',
      },
      {
        ticker_id: 'MGX_TUR',
        base_currency: 'MGX',
        target_currency: 'TUR',
        last_price: '0.1395751849',
        base_volume: '13195.363557291116508',
        target_volume: '1832.6875337653700768',
        pool_id: '8',
        liquidity_in_usd: '57038.15488204146',
      },
      {
        ticker_id: 'KSM_TUR',
        base_currency: 'KSM',
        target_currency: 'TUR',
        last_price: '8700.7130012718',
        base_volume: '8.8509000336977783626',
        target_volume: '50677.157558869231101',
        pool_id: '9',
        liquidity_in_usd: '126.74932683307718',
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
