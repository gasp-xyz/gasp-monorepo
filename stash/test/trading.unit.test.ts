import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { getDataByWallet } from '../src/service/TradingService'
import { swapRepository } from '../src/repository/TransactionRepository'

vi.mock('../src/repository/TransactionRepository')

describe.skip('TradingService', () => {
  describe('getDataByWallet', () => {
    beforeEach(() => {
      vi.clearAllMocks()
    })

    it('should return data for a valid wallet', async () => {
      const mockData = [{ id: '1', account: '0xaccount' }]
      ;(swapRepository.search as vi.Mock).mockReturnValue({
        where: vi.fn().mockReturnThis(),
        equals: vi.fn().mockReturnThis(),
        return: { all: vi.fn().mockResolvedValue(mockData) },
      })

      const result = await getDataByWallet('0xaccount')

      expect(result).toEqual(mockData)
      expect(swapRepository.search().where).toHaveBeenCalledWith('account')
      expect(swapRepository.search().where('account').equals).toHaveBeenCalledWith('0xaccount')
      expect(swapRepository.search().where('account').equals('0xaccount').return.all).toHaveBeenCalled()
    })

    it('should return an empty array for a wallet with no data', async () => {
      ;(swapRepository.search as vi.Mock).mockReturnValue({
        where: vi.fn().mockReturnThis(),
        equals: vi.fn().mockReturnThis(),
        return: { all: vi.fn().mockResolvedValue([]) },
      })

      const result = await getDataByWallet('0xaccount2')

      expect(result).toEqual([])
      expect(swapRepository.search().where).toHaveBeenCalledWith('account')
      expect(swapRepository.search().where('account').equals).toHaveBeenCalledWith('0xaccount2')
      expect(swapRepository.search().where('account').equals('0xaccount2').return.all).toHaveBeenCalled()
    })
  })
})