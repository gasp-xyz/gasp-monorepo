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

  const latestBlock = (await store.getLatest()).block
  // const latestBlock = 2500000
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
      // timings.saveLatest = Date.now() - stepStart

      // // let processedByPoolRates = await poolRatesService.processRates();
      // timings.poolRates = Date.now() - stepStart

      // stepStart = Date.now()
      // // let processedByPriceService = await priceService.processPrices(1);
      // timings.priceService = Date.now() - stepStart

      // stepStart = Date.now()
      // // const mergedProcessed = new Map([...processedByPoolRates])
      // // for (const [key, value] of processedByPriceService) {
      // //   if (!mergedProcessed.has(key) || value < mergedProcessed.get(key)) {
      // //     mergedProcessed.set(key, value)
      // //   }
      // // }
      // // await store.removeUnusedKeys(mergedProcessed)
      // timings.removeUnusedKeys = Date.now() - stepStart

      const totalTime = Date.now() - blockStart
      logger.info(`Block ${block.number} processed in ${totalTime}ms`)
    } catch (e) {
      logger.error('Error in processing block: ', e)
    }
  })
}
