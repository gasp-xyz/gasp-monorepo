import MangataClient from '../connector/MangataNode.js'
import * as store from '../repository/ChainRepository.js'
import * as blocks from '../scraper/BlockScraper.js'
import * as deposits from '../scraper/DepositScraper.js'
import * as pools from '../scraper/PoolsScraper.js'
import * as staking from '../scraper/StakingScraper.js'
import * as swaps from '../scraper/SwapScraper.js'
import * as withdrawals from '../scraper/WithdrawalScraper.js'
import * as poolRatesService from '../processing/PoolRatesProcessorService.js'
import * as priceService from '../processing/PriceProcessorService.js'

import logger from '../util/Logger.js'

export const initService = async () => {
  const api = await MangataClient.api()

  // const latestBlock = (await store.getLatest()).block
  const latestBlock = 2500000
  await blocks.withBlocks(api, latestBlock, async (block) => {
    try {
      const blockStart = Date.now()
      const timings: { [key: string]: number } = {}

      let stepStart = Date.now()
      await withdrawals.processWithdrawalEvents(api, block)
      timings.withdrawals = Date.now() - stepStart

      stepStart = Date.now()
      await deposits.processFerriedDepositEvents(api, block)
      timings.deposits = Date.now() - stepStart

      stepStart = Date.now()
      await pools.fetchPools(block)
      timings.pools = Date.now() - stepStart

      stepStart = Date.now()
      await staking.processStaking(api, block)
      timings.staking = Date.now() - stepStart

      stepStart = Date.now()
      await staking.processLiquidStaking(api, block)
      timings.liquidStaking = Date.now() - stepStart

      stepStart = Date.now()
      await swaps.processSwapEvents(api, block)
      timings.swaps = Date.now() - stepStart

      stepStart = Date.now()
      await store.saveLatest({
        timestamp: block.timestamp,
        block: block.number,
      })
      timings.saveLatest = Date.now() - stepStart

      stepStart = Date.now()
      // let processedByPoolRates = await poolRatesService.processRates();
      timings.poolRates = Date.now() - stepStart

      stepStart = Date.now()
      // let processedByPriceService = await priceService.processPrices(1);
      timings.priceService = Date.now() - stepStart

      stepStart = Date.now()
      // const mergedProcessed = new Map([...processedByPoolRates])
      // for (const [key, value] of processedByPriceService) {
      //   if (!mergedProcessed.has(key) || value < mergedProcessed.get(key)) {
      //     mergedProcessed.set(key, value)
      //   }
      // }
      // await store.removeUnusedKeys(mergedProcessed)
      timings.removeUnusedKeys = Date.now() - stepStart

      const totalTime = Date.now() - blockStart
      const sortedTimings = Object.entries(timings).sort(([,a], [,b]) => b - a)

      logger.info(`Block ${block.number} processed in ${totalTime}ms`, {
        totalTime,
        timings: Object.fromEntries(sortedTimings)
      })


    } catch (e) {
      logger.error('Error in processing block: ', e)
    }
  })
}
