import { describe, it, expect, vi, beforeEach, beforeAll } from 'vitest'
import {
  calculateVolume,
  decimalsFromTokenId,
  processDataForDashboard,
  processDataForVolumeHistory,
  processDataForTVLHistory,
} from '../src/scraper/SwapScraper'
import {swapRepository} from '../src/repository/TransactionRepository'
import logger from '../src/util/Logger'
import { ApiPromise } from '@polkadot/api'
import * as tokenPriceService from '../src/service/TokenPriceService'
import { timeseries } from '../src/connector/RedisConnector'
import { GenericContainer, Wait } from 'testcontainers'


vi.mock('../src/repository/TransactionRepository')
vi.mock('../src/util/Logger')
vi.mock('../src/service/PriceDiscoveryService')

describe('processSwapEvents', () => {
  let mockApi: ApiPromise
  let mockBlock: any
  let mockEvent: any
  let mockSwapData: any
  // let timeseriesContainer
  //
  // beforeAll(() => {
  //   vi.clearAllMocks()
  //   timeseriesContainer = await new GenericContainer('redis/redis-stack:latest')
  //       .withExposedPorts({ container: 6379, host: 6381 })
  //       .withWaitStrategy(Wait.forLogMessage('Ready to accept connections'))
  //       .start()
  //
  //   process.env.REDIS_HOST = 'localhost'
  //   process.env.REDIS_PORT = '6380'
  //   process.env.REDIS_PASS = ''
  //
  //   process.env.TIMESERIES_HOST = timeseriesContainer.getHost()
  //   process.env.TIMESERIES_PORT = '6381'
  //   process.env.TIMESERIES_PASS = ''
  // })

  beforeEach(() => {
    vi.clearAllMocks()
    mockApi = {
      query: {
        assetRegistry: {
          metadata: {
            entries: vi.fn().mockResolvedValue([
              [{ args: ['0'] }, { unwrap: () => ({ decimals: { toPrimitive: () => 18 } }) }],
              [{ args: ['1'] }, { unwrap: () => ({ decimals: { toPrimitive: () => 18 } }) }],
            ]),
          },
        },
      },
    } as unknown as ApiPromise
    mockBlock = {
      events: [
        [
          0,
          {
            section: 'market',
            method: 'AssetsSwapped',
            data: {
              who: '0x0404040404040404040404040404040404040404',
              swaps: [
                {
                  pool_id: 6,
                  kind: 'Xyk',
                  assetIn: 0,
                  assetOut: 1,
                  amountIn: '40648650414565365',
                  amountOut: '20181563007698743',
                },
              ],
            },
          },
        ],
      ],
    }
    mockEvent = {
      data: {
        who: '0x0404040404040404040404040404040404040404',
        swaps: [
          {
            poolId: 6,
            assetIn: 0,
            assetOut: 1,
            amountIn: '40648650414565365',
            amountOut: '20181563007698743',
          },
        ],
      },
    }
    mockSwapData = {
      account: '0xaccount',
      daysActive: 1,
      lastTradeTimestamp: '1458859082092.11',
      totalVolume: expect.any(Number),
      totalTrades: expect.any(Number),
    }
  })

  it('should process swap events for dashboard and update existing record', async () => {
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
    vi.spyOn(tokenPriceService, 'getTokenPrice').mockResolvedValue(1)
    vi.spyOn(swapRepository, 'save').mockResolvedValue(mockSwapData)
    await processDataForDashboard(mockApi, mockBlock.events[0][1])

    expect.objectContaining({
      account: '0xaccount',
      daysActive: 1,
      lastTradeTimestamp: '1458859082092.11',
      totalVolume: expect.any(Number),
      totalTrades: expect.any(Number),
    })
  })

  it('should process swap events for dashboard and create new record', async () => {
    vi.spyOn(swapRepository, 'search').mockReturnValue({
      where: vi.fn().mockReturnThis(),
      equals: vi.fn().mockReturnThis(),
      returnFirst: vi.fn().mockResolvedValue(null),
    } as any)
    vi.spyOn(tokenPriceService, 'getTokenPrice').mockResolvedValue(1)
    vi.spyOn(swapRepository, 'save').mockResolvedValue(mockSwapData)
    await processDataForDashboard(mockApi, mockBlock.events[0][1])

    expect.objectContaining({
      account: '0xaccount',
      daysActive: 1,
      lastTradeTimestamp: '1458859082092.11',
      totalVolume: expect.any(Number),
      totalTrades: expect.any(Number),
    })
  })

  it('should log an error if processing  of dashboard event fails', async () => {
    const error = new Error('Test error')
    vi.spyOn(swapRepository, 'search').mockReturnValue({
      where: vi.fn().mockReturnThis(),
      equals: vi.fn().mockReturnThis(),
      returnFirst: vi.fn().mockRejectedValue(error),
    } as any)
    await expect(processDataForDashboard(mockApi, mockBlock.events[0][1])).rejects.toThrow('Test error')  })

  it('should process volume history when timeseries.client.call returns 0', async () => {
    vi.spyOn(timeseries.client, 'call').mockResolvedValue(['', '0'])
    await processDataForVolumeHistory(mockApi, mockEvent)

    expect(timeseries.client.call).toHaveBeenCalledWith('TS.GET', 'trades:pool:6')
    expect(timeseries.client.call).toHaveBeenCalledWith('TS.ADD', 'trades:pool:6', '*', expect.any(Number))
    expect(timeseries.client.call).toHaveBeenCalledWith('TS.GET', 'trades:pool:ALL')
    expect(timeseries.client.call).toHaveBeenCalledWith('TS.ADD', 'trades:pool:ALL', '*', expect.any(Number))
    expect(timeseries.client.call).toHaveBeenCalledWith('TS.GET', 'trades:asset:0')
    expect(timeseries.client.call).toHaveBeenCalledWith('TS.ADD', 'trades:asset:0', '*', expect.any(Number))
    expect(timeseries.client.call).toHaveBeenCalledWith('TS.GET', 'trades:asset:1')
    expect(timeseries.client.call).toHaveBeenCalledWith('TS.ADD', 'trades:asset:1', '*', expect.any(Number))
    console.log(logger.info.mock.calls)
  })

  it('should process volume history when there is already an existing key with value', async () => {
    // fill pools, assets and pool:ALL
  })

  it('should handle errors during volume history processing', async () => {
    // Add test implementation here
  })
  it('should process TVL history when there is no existing key', async () => {
    // fill pools, assets and pool:ALL
  })

  it('should process TVL history when there is already an existing key with value', async () => {
    // fill pools, assets and pool:ALL
  })

  it('should handle errors during TVL history processing', async () => {
    // Add test implementation here
  })
})

describe('calculateVolume', () => {
  it('should calculate volume correctly', async () => {
    vi.spyOn(tokenPriceService, 'getTokenPrice').mockResolvedValue(1)

    const result = await calculateVolume('1', 18, '10000000000000000000000')

    expect(result).toBe(10000)
  })

  it('should return null if getting token price fails', async () => {
    const error = new Error('Test error')
    vi.spyOn(tokenPriceService, 'getTokenPrice').mockRejectedValue(error)

    const result = await calculateVolume('1', 18, '10000000000000000000000')

    expect(result).toBe(0)
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