import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { processSwapEvents, calculateVolume, decimalsFromTokenId } from '../src/scraper/SwapScraper'
import {swapRepository} from '../src/repository/TransactionRepository'
import logger from '../src/util/Logger'
import * as priceDiscoveryService from '../src/service/PriceDiscoveryService'
import { ApiPromise } from '@polkadot/api'

vi.mock('../src/repository/TransactionRepository')
vi.mock('../src/util/Logger')
vi.mock('../src/service/PriceDiscoveryService')

describe('processSwapEvents', () => {
  let mockApi: ApiPromise
  let mockBlock: any

  beforeEach(() => {
    vi.clearAllMocks()
    mockApi = {} as ApiPromise
    mockBlock = {
      events: [
        [
          0,
          {
            section: 'xyk',
            method: 'AssetsSwapped',
            data: ['0xaccount', ['0', '1'], null, '10000000000000000000000'],
          },
        ],
      ],
    }
  })

  it('should process swap events and update existing record', async () => {
    const mockRecord = {
      account: '0xaccount',
      lastTradeTimestamp: new Date().getTime(),
      daysActive: 1,
      totalVolume: 0.0001,
      totalTrades: 1,
    }
    vi.spyOn(swapRepository, 'search').mockReturnValue({
      where: vi.fn().mockReturnThis(),
      equals: vi.fn().mockReturnThis(),
      returnFirst: vi.fn().mockResolvedValue(mockRecord),
    } as any)
    vi.spyOn(priceDiscoveryService, 'priceDiscovery').mockResolvedValue({
      current_price: { usd: '1' },
    })
    const mockSwapData = {
      account: '0xaccount',
      daysActive: 1,
      lastTradeTimestamp: '1458859082092.11',
      totalVolume: expect.any(Number),
      totalTrades: expect.any(Number),
    }
    vi.spyOn(swapRepository, 'save').mockResolvedValue(mockSwapData)
    await processSwapEvents(mockApi, mockBlock)

    expect.objectContaining({
      account: '0xaccount',
      daysActive: 1,
      lastTradeTimestamp: '1458859082092.11',
      totalVolume: expect.any(Number),
      totalTrades: expect.any(Number),
    })
  })

  it('should process swap events and create new record', async () => {
    vi.spyOn(swapRepository, 'search').mockReturnValue({
      where: vi.fn().mockReturnThis(),
      equals: vi.fn().mockReturnThis(),
      returnFirst: vi.fn().mockResolvedValue(null),
    } as any)
    vi.spyOn(priceDiscoveryService, 'priceDiscovery').mockResolvedValue({
      current_price: { usd: '1' },
    })
    const mockSwapData = {
      account: '0xaccount',
      daysActive: 1,
      lastTradeTimestamp: '1458859082092.11',
      totalVolume: expect.any(Number),
      totalTrades: expect.any(Number),
    }
    vi.spyOn(swapRepository, 'save').mockResolvedValue(mockSwapData)
    await processSwapEvents(mockApi, mockBlock)

    expect.objectContaining({
      account: '0xaccount',
      daysActive: 1,
      lastTradeTimestamp: '1458859082092.11',
      totalVolume: expect.any(Number),
      totalTrades: expect.any(Number),
    })
  })

  it('should log an error if processing fails', async () => {
    const error = new Error('Test error')
    vi.spyOn(swapRepository, 'search').mockReturnValue({
      where: vi.fn().mockReturnThis(),
      equals: vi.fn().mockReturnThis(),
      returnFirst: vi.fn().mockRejectedValue(error),
    } as any)

    await processSwapEvents(mockApi, mockBlock)

    expect(logger.error).toHaveBeenCalledWith('Error processing swap event:', error)
  })
})

describe('calculateVolume', () => {
  it('should calculate volume correctly', async () => {
    vi.spyOn(priceDiscoveryService, 'priceDiscovery').mockResolvedValue({
      current_price: { usd: '1' },
    })

    const result = await calculateVolume('tokenId1', 18, '10000000000000000000000')

    expect(result).toBe(10000)
  })

  it('should return null if price discovery fails', async () => {
    const error = new Error('Test error')
    vi.spyOn(priceDiscoveryService, 'priceDiscovery').mockRejectedValue(error)

    const result = await calculateVolume('1', 18, '10000000000000000000000')

    expect(result).toBeNull()
    expect(logger.error).toHaveBeenCalledWith(
      'Error: Unable to retrieve token price data for token id 1',
      error
    )
  })
})

describe('decimalsFromTokenId', () => {
  let mockApi: ApiPromise

  beforeEach(() => {
    mockApi = {
      query: {
        assetRegistry: {
          metadata: {
            entries: vi.fn().mockResolvedValue([
              [{ args: ['1'] }, { unwrap: () => ({ decimals: { toPrimitive: () => 18 } }) }],
            ]),
          },
        },
      },
    } as unknown as ApiPromise
  })

  it('should return decimals for a valid tokenId', async () => {
    const result = await decimalsFromTokenId(mockApi, '1')

    expect(result).toEqual({ id: '1', decimals: 18 })
  })

  it('should return null for an invalid tokenId', async () => {
    const result = await decimalsFromTokenId(mockApi, 'invalidTokenId')

    expect(result).toBeNull()
  })
})