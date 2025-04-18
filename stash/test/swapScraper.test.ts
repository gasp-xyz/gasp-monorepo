import { ApiPromise } from '@polkadot/api'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import { timeseries } from '../src/connector/RedisConnector'
import { swapRepository } from '../src/repository/TransactionRepository'
import {
  calculateVolume,
  checkKey,
  decimalsFromTokenId,
  processDataForDashboard,
  processDataForTVLHistory,
  processDataForVolumeHistory,
} from '../src/scraper/SwapScraper'
import * as tokenPriceService from '../src/service/TokenPriceService'
import logger from '../src/util/Logger'

vi.mock('../src/repository/TransactionRepository')
vi.mock('../src/util/Logger')
vi.mock('../src/service/PriceDiscoveryService')

describe('process dashboard events', () => {
  let mockApi: ApiPromise
  let mockBlock: any
  let mockSwapData: any

  beforeEach(() => {
    vi.clearAllMocks()
    mockApi = {
      query: {
        assetRegistry: {
          metadata: {
            entries: vi.fn().mockResolvedValue([
              [
                { args: ['0'] },
                { unwrap: () => ({ decimals: { toPrimitive: () => 18 } }) },
              ],
              [
                { args: ['1'] },
                { unwrap: () => ({ decimals: { toPrimitive: () => 18 } }) },
              ],
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

  it('should log an error if processing  of specific swap dashboard event fails', async () => {
    const error = new Error('Test error')
    vi.spyOn(swapRepository, 'search').mockReturnValue({
      where: vi.fn().mockReturnThis(),
      equals: vi.fn().mockReturnThis(),
      returnFirst: vi.fn().mockRejectedValue(error),
    } as any)
    await processDataForDashboard(mockApi, mockBlock.events[0][1])
    expect(logger.error).toHaveBeenNthCalledWith(
      1,
      'Error processing data for a specific swap for dashboard:',
      error
    )
  })
})

describe('volume history events', () => {
  let mockApi: ApiPromise
  let mockEvent: any

  beforeEach(() => {
    vi.clearAllMocks()
    mockApi = {
      query: {
        assetRegistry: {
          metadata: {
            entries: vi.fn().mockResolvedValue([
              [
                { args: ['0'] },
                { unwrap: () => ({ decimals: { toPrimitive: () => 18 } }) },
              ],
              [
                { args: ['1'] },
                { unwrap: () => ({ decimals: { toPrimitive: () => 18 } }) },
              ],
            ]),
          },
        },
      },
    } as unknown as ApiPromise
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
  })
  it('should process volume history when price of the token is 0', async () => {
    vi.spyOn(tokenPriceService, 'getTokenPrice').mockResolvedValue(0)
    await processDataForVolumeHistory(mockApi, mockEvent)
    expect(logger.info).toHaveBeenNthCalledWith(
      1,
      'Entered processDataForVolumeHistory'
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      2,
      'Swap data:',
      expect.any(Object)
    )
    const poolId = 6
    const key = `trades:pool:${poolId}`
    const [timestamp, value] = (await timeseries.client.call(
      'TS.GET',
      key
    )) as [string, string]
    console.log(`Timestamp: ${timestamp}, Value: ${value}`)
    expect(logger.info).toHaveBeenNthCalledWith(
      3,
      `Fetched pool volume for 6, latest value from the database is : ${value}`
    ) //todo: replace everywhere with value from db
    expect(logger.info).toHaveBeenNthCalledWith(
      4,
      'Formula for poolId 6 new volume is =  133.70512668894136 + 0 + 0 but if the price of one token is 0 pool volume stays unchanged'
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      5,
      'Updated pool volume for 6, new value in the database is : 133.70512668894136'
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      6,
      'Formula for ALL pools new volume is =  3.0801617002600388 + 0 + 0 but if the price of one token is 0 ALL pool volume stays unchanged'
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      7,
      'Updated pool volume for ALL: 3.0801617002600388'
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      8,
      'Fetched volume for asset with id 0, value from the database is: 3.0847888679184536'
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      9,
      'Formula for assetId 0 new volume is =  3.0847888679184536 + 0'
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      10,
      'Updated volume for asset with id 0, new value in the database is: 3.0847888679184536'
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      11,
      'Fetched volume for asset with id 1, value from the database is: 3.075534532601624'
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      12,
      'Formula for assetId 1 new volume is =  3.075534532601624 + 0'
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      13,
      'Updated volume for asset with id 1, new value in the database is: 3.075534532601624'
    )
  })

  it('should process volume history when there are prices for tokens', async () => {
    vi.spyOn(tokenPriceService, 'getTokenPrice').mockResolvedValue(1.22)
    await processDataForVolumeHistory(mockApi, mockEvent)
    expect(logger.info).toHaveBeenNthCalledWith(
      1,
      'Entered processDataForVolumeHistory'
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      2,
      'Swap data:',
      expect.any(Object)
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      3,
      'Fetched pool volume for 6, latest value from the database is : 133.70512668894136'
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      4,
      'Formula for poolId 6 new volume is =  133.70512668894136 + 0.04959135350576975 + 0.024621506869392466 but if the price of one token is 0 pool volume stays unchanged'
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      5,
      'Updated pool volume for 6, new value in the database is : 133.77933954931652'
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      6,
      'Formula for ALL pools new volume is =  3.0801617002600388 + 0.04959135350576975 + 0.024621506869392466 but if the price of one token is 0 ALL pool volume stays unchanged'
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      7,
      'Updated pool volume for ALL: 3.154374560635201'
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      8,
      'Fetched volume for asset with id 0, value from the database is: 3.0847888679184536'
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      9,
      'Formula for assetId 0 new volume is =  3.0847888679184536 + 0.04959135350576975'
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      10,
      'Updated volume for asset with id 0, new value in the database is: 3.1343802214242236'
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      11,
      'Fetched volume for asset with id 1, value from the database is: 3.075534532601624'
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      12,
      'Formula for assetId 1 new volume is =  3.075534532601624 + 0.024621506869392466'
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      13,
      'Updated volume for asset with id 1, new value in the database is: 3.1001560394710164'
    )
  })

  it('should handle errors if a specific swap in  volume history processing fails', async () => {
    const error = new Error('Timeseries error')
    vi.spyOn(timeseries.client, 'call').mockRejectedValue(
      new Error('Timeseries error')
    )
    await processDataForVolumeHistory(mockApi, mockEvent)
    expect(logger.error).toHaveBeenNthCalledWith(
      1,
      'Error processing data for a specific swap for volume history:',
      error
    )
  })
})

describe('TVL history events', () => {
  let mockApi: ApiPromise
  let mockEvent: any

  beforeEach(() => {
    vi.clearAllMocks()
    vi.resetAllMocks()
    vi.restoreAllMocks()
    mockApi = {
      query: {
        assetRegistry: {
          metadata: {
            entries: vi.fn().mockResolvedValue([
              [
                { args: ['0'] },
                { unwrap: () => ({ decimals: { toPrimitive: () => 18 } }) },
              ],
              [
                { args: ['1'] },
                { unwrap: () => ({ decimals: { toPrimitive: () => 18 } }) },
              ],
            ]),
          },
        },
      },
    } as unknown as ApiPromise
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
  })
  it('should process TVL history when price of the token is 0', async () => {
    vi.spyOn(tokenPriceService, 'getTokenPrice').mockResolvedValueOnce(0)
    vi.spyOn(logger, 'info').mockImplementation(() => {})
    vi.spyOn(logger, 'error').mockImplementation(() => {})
    await processDataForTVLHistory(mockApi, mockEvent)
    expect(logger.info).toHaveBeenNthCalledWith(
      1,
      'Entered processDataForTVLHistory'
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      2,
      'Swap data:',
      expect.any(Object)
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      3,
      'Fetched pool TVL for 6, value in the database is: 11018.218716490845'
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      4,
      'Formula for poolId 6 new TVL is =  11018.218716490845 + 0 - 0 but if the price of one token is 0 pool TVL stays unchanged'
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      5,
      'Updated pool TVL for 6, new value in the database is: 11018.218716490845'
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      6,
      'Fetched pool TVL ALL pools, value in the database is: 632439770.9843899'
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      7,
      'Formula for ALL pools new TVL is =  632439770.9843899 + 0 - 0 but if the price of one token is 0 ALL pool TVL stays unchanged'
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      8,
      'Updated pool TVL for ALL, new value in the database is: 632439770.9843899'
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      9,
      'Fetched TVL for asset with id 0, value in the database is: 316215799.41475797'
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      10,
      'Formula for assetId 0 new TVL is =  316215799.41475797 + 0'
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      11,
      'Updated TVL for asset with id 0, new value in the database is: 316215799.41475797'
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      12,
      'Fetched TVL for asset with id 1, value in the database is : 316131011.7208412'
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      13,
      'Formula for assetId 1 new TVL is =  316131011.7208412 - 0 but if the formula value is negative, new TVL value will be 0'
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      14,
      'Updated TVL for asset with id 1, new value in the database is: 316131011.7208412'
    )
  })

  it('should process TVL history when there are prices for tokens', async () => {
    vi.spyOn(tokenPriceService, 'getTokenPrice').mockResolvedValue(1.22)
    vi.spyOn(logger, 'info').mockImplementation(() => {})
    vi.spyOn(logger, 'error').mockImplementation(() => {})
    await processDataForTVLHistory(mockApi, mockEvent)
    expect(logger.info).toHaveBeenNthCalledWith(
      1,
      'Entered processDataForTVLHistory'
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      2,
      'Swap data:',
      expect.any(Object)
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      3,
      'Fetched pool TVL for 6, value in the database is: 11018.218716490845'
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      4,
      'Formula for poolId 6 new TVL is =  11018.218716490845 + 0.04959135350576975 - 0.024621506869392466 but if the price of one token is 0 pool TVL stays unchanged'
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      5,
      'Updated pool TVL for 6, new value in the database is: 11018.24368633748'
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      6,
      'Fetched pool TVL ALL pools, value in the database is: 632439770.9843899'
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      7,
      'Formula for ALL pools new TVL is =  632439770.9843899 + 0.04959135350576975 - 0.024621506869392466 but if the price of one token is 0 ALL pool TVL stays unchanged'
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      8,
      'Updated pool TVL for ALL, new value in the database is: 632439771.0093597'
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      9,
      'Fetched TVL for asset with id 0, value in the database is: 316215799.41475797'
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      10,
      'Formula for assetId 0 new TVL is =  316215799.41475797 + 0.04959135350576975'
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      11,
      'Updated TVL for asset with id 0, new value in the database is: 316215799.4643493'
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      12,
      'Fetched TVL for asset with id 1, value in the database is : 316131011.7208412'
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      13,
      'Formula for assetId 1 new TVL is =  316131011.7208412 - 0.024621506869392466 but if the formula value is negative, new TVL value will be 0'
    )
    expect(logger.info).toHaveBeenNthCalledWith(
      14,
      'Updated TVL for asset with id 1, new value in the database is: 316131011.69621974'
    )
  })

  it('should handle errors if a specific swap in  TVL history processing fails', async () => {
    vi.spyOn(logger, 'error').mockImplementation(() => {})
    vi.spyOn(logger, 'info').mockImplementation(() => {})
    const error = new Error('Timeseries error')
    vi.spyOn(timeseries.client, 'call').mockRejectedValue(
      new Error('Timeseries error')
    )
    await processDataForTVLHistory(mockApi, mockEvent)
    expect(logger.error).toHaveBeenNthCalledWith(
      1,
      'Error processing data for a specific swap for TVL history:',
      error
    )
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
            entries: vi
              .fn()
              .mockResolvedValue([
                [
                  { args: ['1'] },
                  { unwrap: () => ({ decimals: { toPrimitive: () => 18 } }) },
                ],
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

describe('checkKey', () => {
  it('should create a new key if it does not exist', async () => {
    const callSpy = vi.spyOn(timeseries.client, 'call').mockResolvedValue('OK')

    await checkKey('testKey', ['label1', 'label2'])

    expect(callSpy).toHaveBeenCalledWith(
      'TS.CREATE',
      'testKey',
      'RETENTION',
      '0',
      'DUPLICATE_POLICY',
      'SUM',
      'LABELS',
      'label1',
      'label2'
    )
  })

  it('should not create a new key if it already exists', async () => {
    const callSpy = vi.spyOn(timeseries.client, 'call')

    await checkKey('testKey', ['label1', 'label2'])

    expect(callSpy).not.toHaveBeenCalledWith('TS.CREATE')
  })
})
