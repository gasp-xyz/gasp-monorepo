import MangataClient from '../connector/MangataNode.js'
import * as poolRatesService from '../processing/PoolRatesProcessorService.js'
import * as priceService from '../processing/PriceProcessorService.js'
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

  const latestBlock = process.env.START_BLOCK ? parseInt(process.env.START_BLOCK, 10) : (await store.getLatest()).block
  if (process.env.START_BLOCK) {
    logger.warn(`Starting from block ${latestBlock} (override via START_BLOCK environment variable)`)
  }
  await blocks.withBlocks(api, latestBlock, async (block) => {
    try {

      let blockStart = Date.now()
      await Promise.all([
        withdrawals.processWithdrawalEvents(api, block),
        deposits.processFerriedDepositEvents(api, block),
        pools.fetchPools(block),
        staking.processStaking(api, block),
        staking.processLiquidStaking(api, block),
        swaps.processSwapEvents(api, block)
      ]);

      await store.saveLatest({
        timestamp: block.timestamp,
        block: block.number,
      })

      const totalTime = Date.now() - blockStart
      logger.info(`Block ${block.number} processed in ${totalTime}ms`)
    } catch (e) {
      logger.error('Error in processing block: ', e)
    }
  })
}
