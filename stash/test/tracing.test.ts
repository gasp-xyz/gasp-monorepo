import { describe, expect, vi, beforeEach, it } from 'vitest'
import {
  startTracingTransaction,
  getTransactionsByAddress,
  getStatusByTxHashOrEntityId,
  getTransactionByEntityId,
  getTransactionByTxHash,
  findTransactionsByAddressAndStatus,
} from '../src/service/TracingService'
import { depositRepository } from '../src/repository/TransactionRepository'

vi.mock('../src/repository/TransactionRepository')
vi.mock('../src/util/Logger')

describe('TracingService', () => {
  const mockTransaction = {
    requestId: null,
    txHash: '0x102',
    address: '0x102',
    created: 1725613967329,
    updated: 1725613967329,
    status: 'PendingOnL1',
    type: 'deposit',
    chain: 'Ethereum',
    amount: '400000000000000000',
    asset_chainId: '0x106',
    asset_address: '0x107',
  }

  beforeEach(() => {
    vi.clearAllMocks()
  })

  it('startTracingTransaction should start tracing a new transaction', async () => {
    depositRepository.search.mockReturnValue({
      where: vi.fn().mockReturnThis(),
      equals: vi.fn().mockReturnThis(),
      and: vi.fn().mockReturnThis(),
      return: { all: vi.fn().mockResolvedValue([]) },
    })
    depositRepository.save.mockResolvedValue(mockTransaction)

    const result = await startTracingTransaction(mockTransaction)

    expect(depositRepository.search).toHaveBeenCalled()
    expect(depositRepository.save).toHaveBeenCalledWith(
      expect.objectContaining({
        txHash: '0x102',
        address: '0x102',
        status: 'PendingOnL1',
      })
    )
    expect(result).toHaveProperty('txHash', '0x102')
    expect(result).toHaveProperty('status', 'PendingOnL1')
  })

  it('getTransactionsByAddress should return transactions for a given address', async () => {
    depositRepository.search.mockReturnValue({
      where: vi.fn().mockReturnThis(),
      equals: vi.fn().mockReturnThis(),
      return: { all: vi.fn().mockResolvedValue([mockTransaction]) },
    })

    const result = await getTransactionsByAddress('0x102')

    expect(depositRepository.search).toHaveBeenCalled()
    expect(result).toEqual([mockTransaction])
  })

  it('getStatusByTxHashOrEntityId should return status for a given txHash', async () => {
    depositRepository.search.mockReturnValue({
      where: vi.fn().mockReturnThis(),
      equals: vi.fn().mockReturnThis(),
      return: { all: vi.fn().mockResolvedValue([mockTransaction]) },
    })

    const result = await getStatusByTxHashOrEntityId('0x102')

    expect(depositRepository.search).toHaveBeenCalled()
    expect(result).toBe('PendingOnL1')
  })

  it('getTransactionByEntityId should return transaction for a given entityId', async () => {
    depositRepository.fetch.mockResolvedValue(mockTransaction)

    const result = await getTransactionByEntityId('entityId')

    expect(depositRepository.fetch).toHaveBeenCalledWith('entityId')
    expect(result).toEqual(mockTransaction)
  })

  it('getTransactionByTxHash should return transaction for a given txHash', async () => {
    depositRepository.search.mockReturnValue({
      where: vi.fn().mockReturnThis(),
      equals: vi.fn().mockReturnThis(),
      return: { all: vi.fn().mockResolvedValue([mockTransaction]) },
    })

    const result = await getTransactionByTxHash('0x102')

    expect(depositRepository.search).toHaveBeenCalled()
    expect(result).toEqual(mockTransaction)
  })

  it('findTransactionsByAddressAndStatus should return transactions for a given address and status', async () => {
    depositRepository.search.mockReturnValue({
      where: vi.fn().mockReturnThis(),
      equals: vi.fn().mockReturnThis(),
      and: vi.fn().mockReturnThis(),
      return: { all: vi.fn().mockResolvedValue([mockTransaction]) },
    })

    const result = await findTransactionsByAddressAndStatus(
      '0x102',
      'PendingOnL1'
    )

    expect(depositRepository.search).toHaveBeenCalled()
    expect(result).toEqual([mockTransaction])
  })
})
