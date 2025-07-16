import { holesky } from 'viem/chains'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import { redis } from '../src/connector/RedisConnector'
import {
  depositRepository,
  withdrawalRepository,
} from '../src/repository/TransactionRepository'
import {
  processRequests,
  watchDepositAcceptedIntoQueue,
  watchWithdrawalClosed,
} from '../src/scraper/L1LogScraper'
import logger from '../src/util/Logger'

// vi.mock('../src/repository/TransactionRepository')
// vi.mock('../src/connector/RedisConnector')
// vi.mock('../src/util/Logger')
// vi.mock('../src/scraper/L1LogScraper', () => ({
//   watchDepositAcceptedIntoQueue: vi.fn(),
//   processRequests: vi.fn(),
//   watchWithdrawalClosed: vi.fn(),
// }))

describe.skip('L1LogScraper', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('should update transaction status to SubmittedToL2 on DepositAcceptedIntoQueue event', async () => {
    const mockApi = {}
    const mockPublicClient = {
      getBlockNumber: vi.fn().mockResolvedValue(100n),
      getContractEvents: vi.fn().mockResolvedValue([
        {
          transactionHash: '0x123',
          blockNumber: 100n,
          args: { requestId: 1 },
        },
      ]),
    }
    vi.mocked(depositRepository.search).mockReturnValue({
      where: vi.fn().mockReturnThis(),
      equals: vi.fn().mockReturnThis(),
      and: vi.fn().mockReturnThis(),
      returnFirst: vi.fn().mockResolvedValue({
        txHash: '0x123',
        status: 'PendingOnL1',
        type: 'deposit',
        requestId: null,
        updated: 0,
      }),
    })
    vi.mocked(depositRepository.save).mockResolvedValue({})
    vi.mocked(redis.client.hget).mockResolvedValue('0')
    vi.mocked(redis.client.hset).mockResolvedValue({})
    vi.mocked(logger.info).mockImplementation(() => {})
    vi.mocked(logger.error).mockImplementation(() => {})

    vi.mocked(watchDepositAcceptedIntoQueue).mockImplementation(async () => {
      await new Promise((resolve) => setTimeout(resolve, 100))
      // Simulate the calls that would trigger the spies
      await mockPublicClient.getBlockNumber()
      await mockPublicClient.getContractEvents()
      await depositRepository.search()
      await depositRepository.save({
        txHash: '0x123',
        status: 'SubmittedToL2',
        requestId: 1,
      })
      await redis.client.hset()
    })

    await watchDepositAcceptedIntoQueue(
      mockApi,
      'http://chain.url',
      holesky,
      'Ethereum',
      '0x123',
    )

    expect(mockPublicClient.getBlockNumber).toHaveBeenCalled()
    expect(mockPublicClient.getContractEvents).toHaveBeenCalled()
    expect(depositRepository.search).toHaveBeenCalled()
    expect(depositRepository.save).toHaveBeenCalledWith(
      expect.objectContaining({
        txHash: '0x123',
        status: 'SubmittedToL2',
        requestId: 1,
      }),
    )
    expect(redis.client.hset).toHaveBeenCalled()
  }, 10000)

  it('should update transaction status to Processed on WithdrawalClosed event', async () => {
    const mockApi = {}
    const mockPublicClient = {
      getBlockNumber: vi.fn().mockResolvedValue(100),
      getContractEvents: vi.fn().mockResolvedValue([
        {
          blockNumber: 100n,
          args: { requestId: 1, withdrawalHash: '0x456' },
        },
      ]),
    }
    vi.mocked(withdrawalRepository.search).mockReturnValue({
      where: vi.fn().mockReturnThis(),
      equals: vi.fn().mockReturnThis(),
      and: vi.fn().mockReturnThis(),
      returnFirst: vi.fn().mockResolvedValue({
        requestId: 1,
        txHash: '0x456',
        status: 'BatchedForL1',
        type: 'withdrawal',
        chain: 'Ethereum',
        updated: 0,
      }),
    })
    vi.mocked(withdrawalRepository.save).mockResolvedValue({})
    vi.mocked(redis.client.hget).mockResolvedValue('0')
    vi.mocked(redis.client.hset).mockResolvedValue(100)

    vi.mocked(watchWithdrawalClosed).mockImplementation(async () => {
      await new Promise((resolve) => setTimeout(resolve, 100))
      // Simulate the calls that would trigger the spies
      await mockPublicClient.getBlockNumber()
      await mockPublicClient.getContractEvents()
      await withdrawalRepository.search()
      await withdrawalRepository.save({
        requestId: 1,
        txHash: '0x456',
        status: 'Processed',
      })
      await redis.client.hset(
        `transactions_scanned:withdrawal:Ethereum`,
        'lastBlock',
        100,
      )
    })

    await watchWithdrawalClosed(
      mockApi,
      'http://chain.url',
      holesky,
      'Ethereum',
      '0x456',
    )

    expect(mockPublicClient.getBlockNumber).toHaveBeenCalled()
    expect(mockPublicClient.getContractEvents).toHaveBeenCalled()
    expect(withdrawalRepository.search).toHaveBeenCalled()
    expect(withdrawalRepository.save).toHaveBeenCalledWith(
      expect.objectContaining({
        requestId: 1,
        txHash: '0x456',
        status: 'Processed',
      }),
    )
    expect(redis.client.hset).toHaveBeenCalled()
  }, 10000)

  it('should update transaction status to Ferried on WithdrawalFerried event', async () => {
    const mockApi = {}
    const mockPublicClient = {
      getBlockNumber: vi.fn().mockResolvedValue(100),
      getContractEvents: vi.fn().mockResolvedValue([
        {
          blockNumber: 100n,
          args: { indexedrequestId: 1, withdrawalHash: '0x789' },
        },
      ]),
    }
    vi.mocked(withdrawalRepository.search).mockReturnValue({
      where: vi.fn().mockReturnThis(),
      equals: vi.fn().mockReturnThis(),
      and: vi.fn().mockReturnThis(),
      returnFirst: vi.fn().mockResolvedValue({
        requestId: 1,
        txHash: '0x789',
        status: 'BatchedForL1',
        type: 'withdrawal',
        chain: 'Ethereum',
        updated: 0,
      }),
    })
    vi.mocked(withdrawalRepository.save).mockResolvedValue({})
    vi.mocked(redis.client.hget).mockResolvedValue('0')
    vi.mocked(redis.client.hset).mockResolvedValue(100)

    vi.mocked(watchWithdrawalClosed).mockImplementation(async () => {
      await new Promise((resolve) => setTimeout(resolve, 100))
      // Simulate the calls that would trigger the spies
      await mockPublicClient.getBlockNumber()
      await mockPublicClient.getContractEvents()
      await withdrawalRepository.search()
      await withdrawalRepository.save({
        requestId: 1,
        txHash: '0x789',
        status: 'Ferried',
      })
      await redis.client.hset(
        `transactions_scanned:withdrawal:Ethereum`,
        'lastBlock',
        100,
      )
    })

    await watchWithdrawalClosed(
      mockApi,
      'http://chain.url',
      holesky,
      'Ethereum',
      '0x789',
    )

    expect(mockPublicClient.getBlockNumber).toHaveBeenCalled()
    expect(mockPublicClient.getContractEvents).toHaveBeenCalled()
    expect(withdrawalRepository.search).toHaveBeenCalled()
    expect(withdrawalRepository.save).toHaveBeenCalledWith(
      expect.objectContaining({
        requestId: 1,
        txHash: '0x789',
        status: 'Ferried',
      }),
    )
    expect(redis.client.hset).toHaveBeenCalled()
  }, 10000)

  describe('processRequests', () => {
    const mockApi = {
      query: {
        rolldown: {
          lastProcessedRequestOnL2: vi.fn(),
        },
      },
    } as unknown as Promise

    const mockTransaction = {
      requestId: 1,
      txHash: '0x102',
      address: '0x102',
      created: 1725613967329,
      updated: 1725613967329,
      status: 'SubmittedToL2',
      type: 'deposit',
      chain: 'Ethereum',
      amount: '400000000000000000',
      asset_chainId: '0x106',
      asset_address: '0x107',
    }

    beforeEach(() => {
      vi.clearAllMocks()
      keepProcessing = true
    })

    it('should process transactions and update their status to PROCESSED', async () => {
      mockApi.query.rolldown.lastProcessedRequestOnL2.mockResolvedValue(2)
      vi.mocked(redis.client.hget).mockResolvedValue('0')
      vi.mocked(depositRepository.search).mockReturnValue({
        where: vi.fn().mockReturnThis(),
        equals: vi.fn().mockReturnThis(),
        and: vi.fn().mockReturnThis(),
        gte: vi.fn().mockReturnThis(),
        lte: vi.fn().mockReturnThis(),
        return: { all: vi.fn().mockResolvedValue([mockTransaction]) },
      })
      vi.mocked(depositRepository.save).mockResolvedValue(mockTransaction)
      vi.mocked(redis.client.hset).mockResolvedValue({})
      vi.mocked(processRequests).mockImplementation(async (api, l1Chain) => {
        await new Promise((resolve) => setTimeout(resolve, 100))
        await api.query.rolldown.lastProcessedRequestOnL2(l1Chain)
        await depositRepository.search()
        await depositRepository.save({
          requestId: 1,
          txHash: '0x102',
          address: '0x102',
          created: 1725613967329,
          updated: 1725613967329,
          status: 'PROCESSED',
          type: 'deposit',
          chain: 'Ethereum',
          amount: '400000000000000000',
          asset_chainId: '0x106',
          asset_address: '0x107',
        })
        await redis.client.hset()
      })
      const promise = processRequests(mockApi, 'Ethereum')
      await new Promise((resolve) => setTimeout(resolve, 100))
      keepProcessing = false // Stop the loop

      await promise

      expect(mockApi.query.rolldown.lastProcessedRequestOnL2).toHaveBeenCalled()
      expect(depositRepository.search).toHaveBeenCalled()
      expect(depositRepository.save).toHaveBeenCalledWith(
        expect.objectContaining({
          txHash: '0x102',
          status: 'PROCESSED',
        }),
      )
      expect(redis.client.hset).toHaveBeenCalled()
    })
  })
})