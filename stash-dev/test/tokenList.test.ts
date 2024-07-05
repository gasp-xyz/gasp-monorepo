import { describe, expect, it, vi, beforeAll, afterAll } from 'vitest'
import { Decimal } from 'decimal.js'
import { TimestampedAmount } from '../src/schema/Models'
import { tokenDetails, tokenList } from '../src/service/TokenListService'
import { TokenInfoStats } from '../src/util/Listing'

vi.mock('gasp-sdk')

describe('[Token list]', () => {
  beforeAll(() => {
    vi.mock('../src/util/Listing', async () => {
      const actual = await vi.importActual('../src/util/Listing')
      return {
        ...actual,
        getTokenVolumeInUsd: vi.fn().mockImplementation((tokenId: string) => {
          return new Promise((resolve) => {
            const volumeTokenResponse = {
              '0': [[1691280000000, new Decimal('867.4535759973365')]],
              '4': [[1691280000000, new Decimal('6.896015943828684')]],
              '7': [[1690761600000, new Decimal('190.68743584230737')]],
            }
            resolve(volumeTokenResponse[tokenId] || [])
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
        getLiquidityTokenInUsd: vi
          .fn()
          .mockImplementation((tokenId: string) => {
            return new Promise((resolve) => {
              const volumeTokenResponse = {
                '0': [[1691366400000, new Decimal('508786.95209646935')]],
                '4': [[1691366400000, new Decimal('57038.15488204146')]],
                '7': [[1691366400000, new Decimal('126.74932683307718')]],
              }
              resolve(volumeTokenResponse[tokenId] || [])
            })
          }),
        calculate24PriceChange: vi
          .fn()
          .mockImplementation((tokenPrices: TimestampedAmount[]) => {
            return new Decimal(2.33).toFixed(2)
          }),
        calculate24VolumeChange: vi
          .fn()
          .mockImplementation((tokenVolumes: TimestampedAmount[]) => {
            return new Decimal(4.21).toFixed(2)
          }),
      }
    })
  })

  afterAll(async () => {
    vi.unmock('../src/util/Listing')
  })

  it('should mock the token details endpoint method for MGX token', async () => {
    const expectedResponse: TokenInfoStats = {
      tokenId: '0',
      tokenName: 'Mangata',
      symbol: 'MGX',
      priceInUSD: '0.0005226090144381267',
      volume24hInUSD: '867.4535759973365',
      liquidity24hInUSD: '508786.95209646935',
      priceChange24hInPerc: '2.33',
      volumeChange24hInPerc: '4.21',
    }
    const result = await tokenDetails('MGX')
    expect(result).deep.equal(expectedResponse)
    expect(result).toHaveProperty('tokenId')
    expect(result).toHaveProperty('tokenName')
    expect(result).toHaveProperty('symbol')
    expect(result).toHaveProperty('priceInUSD')
    expect(result).toHaveProperty('volume24hInUSD')
    expect(result).toHaveProperty('liquidity24hInUSD')
    expect(result).toHaveProperty('priceChange24hInPerc')
    expect(result).toHaveProperty('volumeChange24hInPerc')
  })

  it('should mock the token details endpoint method for KSM token', async () => {
    const expectedResponse: TokenInfoStats = {
      tokenId: '4',
      tokenName: 'Kusama Native',
      symbol: 'KSM',
      priceInUSD: '21.54441188086054',
      volume24hInUSD: '6.896015943828684',
      liquidity24hInUSD: '57038.15488204146',
      priceChange24hInPerc: '2.33',
      volumeChange24hInPerc: '4.21',
    }
    const result = await tokenDetails('KSM')
    expect(result).deep.equal(expectedResponse)
    expect(result).toHaveProperty('tokenId')
    expect(result).toHaveProperty('tokenName')
    expect(result).toHaveProperty('symbol')
    expect(result).toHaveProperty('priceInUSD')
    expect(result).toHaveProperty('volume24hInUSD')
    expect(result).toHaveProperty('liquidity24hInUSD')
    expect(result).toHaveProperty('priceChange24hInPerc')
    expect(result).toHaveProperty('volumeChange24hInPerc')
  })

  it('should mock the token list endpoint method', async () => {
    const expectedResponse: TokenInfoStats[] = [
      {
        tokenId: '0',
        tokenName: 'Mangata',
        symbol: 'MGX',
        priceInUSD: '0.0005226090144381267',
        volume24hInUSD: '867.4535759973365',
        liquidity24hInUSD: '508786.95209646935',
        priceChange24hInPerc: '2.33',
        volumeChange24hInPerc: '4.21',
      },
      {
        tokenId: '4',
        tokenName: 'Kusama Native',
        symbol: 'KSM',
        priceInUSD: '21.54441188086054',
        volume24hInUSD: '6.896015943828684',
        liquidity24hInUSD: '57038.15488204146',
        priceChange24hInPerc: '2.33',
        volumeChange24hInPerc: '4.21',
      },
      {
        tokenId: '7',
        tokenName: 'Turing native token',
        symbol: 'TUR',
        priceInUSD: '0.003762788700625028',
        volume24hInUSD: '190.68743584230737',
        liquidity24hInUSD: '126.74932683307718',
        priceChange24hInPerc: '2.33',
        volumeChange24hInPerc: '4.21',
      },
    ]
    const results = await tokenList()
    expect(results).deep.equal(expectedResponse)
    results.forEach((element) => {
      expect(element).toHaveProperty('tokenId')
      expect(element).toHaveProperty('tokenName')
      expect(element).toHaveProperty('symbol')
      expect(element).toHaveProperty('priceInUSD')
      expect(element).toHaveProperty('volume24hInUSD')
      expect(element).toHaveProperty('liquidity24hInUSD')
      expect(element).toHaveProperty('priceChange24hInPerc')
      expect(element).toHaveProperty('volumeChange24hInPerc')
    })
  })
})
