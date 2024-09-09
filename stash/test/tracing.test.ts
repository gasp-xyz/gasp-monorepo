import { describe, expect, vi, beforeEach, it } from "vitest";
import {
  startTracingTransaction,
  getTransactionsByAddress,
  getStatusByTxHashOrEntityId,
  getTransactionByEntityId,
  getTransactionByTxHash,
  findTransactionsByAddressAndStatus,
} from '../src/service/TracingService'
import { transactionRepository } from '../src/repository/TransactionRepository'

vi.mock('../src/repository/TransactionRepository')
vi.mock('../src/util/Logger')

describe('TracingService', () => {
  const mockTransaction = {
    requestId: null,
    txHash: '0x102',
    address: '0x102',
    created: 1725613967329,
    updated: 1725613967329,
    status: 'L1_INITIATED',
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
    transactionRepository.search.mockReturnValue({
      where: vi.fn().mockReturnThis(),
      equals: vi.fn().mockReturnThis(),
      and: vi.fn().mockReturnThis(),
      return: { all: vi.fn().mockResolvedValue([]) },
    })
    transactionRepository.save.mockResolvedValue(mockTransaction)

    const result = await startTracingTransaction(mockTransaction)

    expect(transactionRepository.search).toHaveBeenCalled()
    expect(transactionRepository.save).toHaveBeenCalledWith(
      expect.objectContaining({
        txHash: '0x102',
        address: '0x102',
        status: 'L1_INITIATED',
      })
    )
    expect(result).toHaveProperty('txHash', '0x102')
    expect(result).toHaveProperty('status', 'L1_INITIATED')
  })

  it('getTransactionsByAddress should return transactions for a given address', async () => {
    transactionRepository.search.mockReturnValue({
      where: vi.fn().mockReturnThis(),
      equals: vi.fn().mockReturnThis(),
      return: { all: vi.fn().mockResolvedValue([mockTransaction]) },
    })

    const result = await getTransactionsByAddress('0x102')

    expect(transactionRepository.search).toHaveBeenCalled()
    expect(result).toEqual([mockTransaction])
  })

  it('getStatusByTxHashOrEntityId should return status for a given txHash', async () => {
    transactionRepository.search.mockReturnValue({
      where: vi.fn().mockReturnThis(),
      equals: vi.fn().mockReturnThis(),
      return: { all: vi.fn().mockResolvedValue([mockTransaction]) },
    })

    const result = await getStatusByTxHashOrEntityId('0x102')

    expect(transactionRepository.search).toHaveBeenCalled()
    expect(result).toBe('L1_INITIATED')
  })

  it('getTransactionByEntityId should return transaction for a given entityId', async () => {
    transactionRepository.fetch.mockResolvedValue(mockTransaction)

    const result = await getTransactionByEntityId('entityId')

    expect(transactionRepository.fetch).toHaveBeenCalledWith('entityId')
    expect(result).toEqual(mockTransaction)
  })

  it('getTransactionByTxHash should return transaction for a given txHash', async () => {
    transactionRepository.search.mockReturnValue({
      where: vi.fn().mockReturnThis(),
      equals: vi.fn().mockReturnThis(),
      return: { all: vi.fn().mockResolvedValue([mockTransaction]) },
    })

    const result = await getTransactionByTxHash('0x102')

    expect(transactionRepository.search).toHaveBeenCalled()
    expect(result).toEqual(mockTransaction)
  })

  it('findTransactionsByAddressAndStatus should return transactions for a given address and status', async () => {
    transactionRepository.search.mockReturnValue({
      where: vi.fn().mockReturnThis(),
      equals: vi.fn().mockReturnThis(),
      and: vi.fn().mockReturnThis(),
      return: { all: vi.fn().mockResolvedValue([mockTransaction]) },
    })

    const result = await findTransactionsByAddressAndStatus(
      '0x102',
      'L1_INITIATED'
    )

    expect(transactionRepository.search).toHaveBeenCalled()
    expect(result).toEqual([mockTransaction])
  })
})
