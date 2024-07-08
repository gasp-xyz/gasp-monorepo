import MangataClient from '../connector/MangataNode.js'
import * as store from '../repository/ChainRepository.js'
import * as blocks from '../scraper/BlockScraper.js'
import * as pools from '../scraper/PoolsScraper.js'
import * as staking from '../scraper/StakingScraper.js'
import logger from '../util/Logger.js'

export const initService = async () => {
  console.log('init service called')
  const api = await MangataClient.api()

  const latestBlock = (await store.getLatest()).block
  // const latestBlock = 3719278
  await blocks.withBlocks(api, latestBlock, async (block) => {
    try {
      // await blocks.processEvents(block)
      // await pools.fetchPools(block)
      // await staking.processStaking(api, block)
      // await staking.processLiquidStaking(api, block)
      // await store.saveLatest({
      //   timestamp: block.timestamp,
      //   block: block.number,
      // })
    } catch (e) {
      logger.error('Error in processing block: ', e)
    }
  })
}
