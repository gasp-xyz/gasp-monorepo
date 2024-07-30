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
              '1': [[1691280000000, 0]],
            }
            resolve(volumeTokenResponse[tokenId] || [])
          })
        }),
        getTokenPrices: vi.fn().mockImplementation((tokenId: string) => {
          return new Promise((resolve) => {
            const tokenResponses = {
              '1': [[1691280000000, 0]],
              '0': [[1691366400000, new Decimal('0.0005226090144381267')]],
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
                '1': [[1691366400000, 0]],
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

  it('should mock the token details endpoint method for GETH token', async () => {
    const expectedResponse: TokenInfoStats = {
      tokenId: '0',
      tokenName: 'Gasp Ethereum',
      symbol: 'GETH',
      priceInUSD: '0.0005226090144381267',
      volume24hInUSD: '867.4535759973365',
      liquidity24hInUSD: '508786.95209646935',
      priceChange24hInPerc: '2.33',
      volumeChange24hInPerc: '4.21',
    }
    const result = await tokenDetails('GETH')
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

  it('should mock the token details endpoint method for GASP token', async () => {
    const expectedResponse: TokenInfoStats = {
      tokenId: '1',
      tokenName: 'Gasp V2',
      symbol: 'GASPV2',
      priceInUSD: '0',
      volume24hInUSD: '0',
      liquidity24hInUSD: '0',
      priceChange24hInPerc: '2.33',
      volumeChange24hInPerc: '4.21',
    }
    const result = await tokenDetails('GASPV2')
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
        tokenName: 'Gasp Ethereum',
        symbol: 'GETH',
        priceInUSD: '0.0005226090144381267',
        volume24hInUSD: '867.4535759973365',
        liquidity24hInUSD: '508786.95209646935',
        priceChange24hInPerc: '2.33',
        volumeChange24hInPerc: '4.21',
      },
      {
        tokenId: '1',
        tokenName: 'Gasp V2',
        symbol: 'GASPV2',
        priceInUSD: '0',
        volume24hInUSD: '0',
        liquidity24hInUSD: '0',
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
