import { swapRepository } from '../repository/TransactionRepository.js'

export const getDataByWallet = async (wallet: string): Promise<object[]> => {
  return await swapRepository
    .search()
    .where('account')
    .equals(wallet)
    .return.all()
}
