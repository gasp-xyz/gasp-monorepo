import { beforeEach, describe, expect, it, vi } from 'vitest'

import { swapRepository } from '../src/repository/TransactionRepository'
import { getDataByWallet } from '../src/service/TradingService'

vi.mock('../src/repository/TransactionRepository')

describe('TradingService', () => {
  describe('getDataByWallet', () => {
    beforeEach(() => {
      vi.clearAllMocks()
    })

    it('should return data for a valid wallet', async () => {
      const mockData = { id: '1', account: '0xaccount' }
      ;(swapRepository.search as vi.Mock).mockReturnValue({
        where: vi.fn().mockReturnThis(),
        equals: vi.fn().mockReturnThis(),
        returnFirst: vi.fn().mockResolvedValue(mockData),
      })

      const result = await getDataByWallet('0xaccount')

      expect(result).toEqual(mockData)
      expect(swapRepository.search().where).toHaveBeenCalledWith('account')
      expect(
        swapRepository.search().where('account').equals
      ).toHaveBeenCalledWith('0xaccount')
      expect(
        swapRepository.search().where('account').equals('0xaccount').returnFirst
      ).toHaveBeenCalled()
    })

    it('should return null for a wallet with no data', async () => {
      ;(swapRepository.search as vi.Mock).mockReturnValue({
        where: vi.fn().mockReturnThis(),
        equals: vi.fn().mockReturnThis(),
        returnFirst: vi.fn().mockResolvedValue(null),
      })

      const result = await getDataByWallet('0xaccount2')

      expect(result).toBeNull()
      expect(swapRepository.search().where).toHaveBeenCalledWith('account')
      expect(
        swapRepository.search().where('account').equals
      ).toHaveBeenCalledWith('0xaccount2')
      expect(
        swapRepository.search().where('account').equals('0xaccount2')
          .returnFirst
      ).toHaveBeenCalled()
    })
  })
})
