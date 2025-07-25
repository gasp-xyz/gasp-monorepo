import { beforeEach, describe, expect, it, vi } from 'vitest'

import { depositRepository } from '../src/repository/TransactionRepository'
import {
  findTransactionsByAddressAndStatus,
  getByTxHashOrEntityId,
  getTransactionsByAddress,
  startTracingTransaction,
} from '../src/service/TracingService'

vi.mock('../src/repository/TransactionRepository')
vi.mock('../src/util/Logger')

describe.skip('TracingService', () => {
  const mockTransaction = {
    requestId: null,
    txHash: '0x102',
    address: '0x102',
    recipient: '0x102',
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
        recipient: '0x102',
        status: 'PendingOnL1',
      }),
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

    const result = await getTransactionsByAddress('0x102', 'deposit')

    expect(depositRepository.search).toHaveBeenCalled()
    expect(result).toEqual([mockTransaction])
  })

  it('getTransactionByTxHashOrEntityId should return a transaction for a given txHash', async () => {
    depositRepository.search.mockReturnValue({
      where: vi.fn().mockReturnThis(),
      equals: vi.fn().mockReturnThis(),
      return: { all: vi.fn().mockResolvedValue([mockTransaction]) },
    })

    const result = await getByTxHashOrEntityId('0x102', 'deposit')

    expect(depositRepository.search).toHaveBeenCalled()
    expect(result).toEqual({
      txHash: '0x102',
      address: '0x102',
      recipient: '0x102',
      created: 1725613967329,
      updated: 1725613967329,
      status: 'PendingOnL1',
      type: 'deposit',
      chain: 'Ethereum',
      amount: '400000000000000000',
      asset_chainId: '0x106',
      asset_address: '0x107',
      requestId: null,
    })
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
      'PendingOnL1',
      'deposit',
    )

    expect(depositRepository.search).toHaveBeenCalled()
    expect(result).toEqual([mockTransaction])
  })
})
