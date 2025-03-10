import { describe, it, beforeEach, vi, expect } from 'vitest'
import { processFerriedDepositEvents, processFerriedDeposit } from '../src/scraper/DepositScraper'
import {depositRepository, withdrawalRepository} from '../src/repository/TransactionRepository'
import { ApiPromise } from '@polkadot/api'
import logger from '../src/util/Logger'

vi.mock('../src/repository/TransactionRepository')
vi.mock('@polkadot/api')
vi.mock('../src/util/Logger')

describe('DepositScraper', () => {
  let mockApi: ApiPromise

  beforeEach(() => {
    mockApi = {} as ApiPromise
    vi.clearAllMocks()
  })

  describe('processFerriedDepositEvents', () => {
    it('should process ferried deposit events', async () => {
      const mockBlock = {
        events: [
          [0, { section: 'rolldown', method: 'DepositFerried', data: { deposit: { requestId: { id: '1' } }, chain: 'testchain' } }],
          [1, { section: 'other', method: 'OtherEvent', data: {} }],
        ],
      }

      depositRepository.search.mockReturnValue({
        where: vi.fn().mockReturnThis(),
        equals: vi.fn().mockReturnThis(),
        and: vi.fn().mockReturnThis(),
        returnFirst: vi.fn().mockResolvedValue({
          requestId: 1,
          chain: 'testchain',
          type: 'deposit',
          status: 'SubmittedToL2',
        }),
      })

      await processFerriedDepositEvents(mockApi, mockBlock as any)

      expect(depositRepository.search).toHaveBeenCalled()
      expect(depositRepository.save).toHaveBeenCalledWith(
        expect.objectContaining({
          status: 'Processed',
          closedBy: 'ferry',
        })
      )
    })
  })

  describe('processFerriedDeposit', () => {
    it('should process a single ferried deposit', async () => {
      const mockEventData = {
        deposit: { requestId: { id: '1' } },
        chain: 'testchain',
      }

      depositRepository.search.mockReturnValue({
        where: vi.fn().mockReturnThis(),
        equals: vi.fn().mockReturnThis(),
        and: vi.fn().mockReturnThis(),
        returnFirst: vi.fn().mockResolvedValue({
          requestId: 1,
          chain: 'testchain',
          type: 'deposit',
          status: 'SubmittedToL2',
        }),
      })
      vi.spyOn(depositRepository, 'save').mockResolvedValue({
        requestId: 1,
        chain: 'testchain',
        type: 'deposit',
        status: 'Processed',
        closedBy: 'ferry',
      })

      const result = await processFerriedDeposit(mockApi, mockEventData)

      expect(depositRepository.search).toHaveBeenCalled()
      expect(depositRepository.save).toHaveBeenCalledWith(
        expect.objectContaining({
          status: 'Processed',
          closedBy: 'ferry',
        })
      )
      expect(result).toEqual(
        expect.objectContaining({
          status: 'Processed',
          closedBy: 'ferry',
        })
      )
    })

    it('should handle case when no matching transactions are found', async () => {
      const mockEventData = {
        deposit: { requestId: { id: '1' } },
        chain: 'testchain',
      }

      depositRepository.search.mockReturnValue({
        where: vi.fn().mockReturnThis(),
        equals: vi.fn().mockReturnThis(),
        and: vi.fn().mockReturnThis(),
        returnFirst: vi.fn().mockResolvedValue(null),
      })

      const result = await processFerriedDeposit(mockApi, mockEventData)

      expect(depositRepository.search).toHaveBeenCalled()
      expect(depositRepository.save).not.toHaveBeenCalled()
      expect(result).toBeUndefined()
    })
  })
})