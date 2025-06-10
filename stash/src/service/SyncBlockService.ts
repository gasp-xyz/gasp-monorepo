import MangataClient from '../connector/MangataNode.js'
import * as store from '../repository/ChainRepository.js'
import * as blocks from '../scraper/BlockScraper.js'
import * as deposits from '../scraper/DepositScraper.js'
import * as pools from '../scraper/PoolsScraper.js'
import * as staking from '../scraper/StakingScraper.js'
import * as swaps from '../scraper/SwapScraper.js'
import * as withdrawals from '../scraper/WithdrawalScraper.js'
import logger from '../util/Logger.js'

export const initService = async () => {
  const api = await MangataClient.api()

  const latestBlock = (await store.getLatest()).block
  // const latestBlock = 3719278
  await blocks.withBlocks(api, latestBlock, async (block) => {
    try {
      if (process.env.SAVE_EVENTS === 'true') {
        await blocks.processEvents(block)
      }
      await withdrawals.processWithdrawalEvents(api, block)
      await deposits.processFerriedDepositEvents(api, block)
      await pools.fetchPools(block)
      await staking.processStaking(api, block)
      await staking.processLiquidStaking(api, block)
      await swaps.processSwapEvents(api, block)
      await store.saveLatest({
        timestamp: block.timestamp,
        block: block.number,
      })
    } catch (e) {
      logger.error('Error in processing block: ', e)
    }
  })
}
