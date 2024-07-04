import MangataClient from '../connector/MangataNode.js'
import * as store from '../repository/ChainRepository.js'
import * as blocks from '../scraper/BlockScraper.js'
import * as pools from '../scraper/PoolsScraper.js'
import * as staking from '../scraper/StakingScraper.js'

export const initService = async () => {
  const api = await MangataClient.api()

  const latestBlock = (await store.getLatest()).block
  // const latestBlock = 3719278
  await blocks.withBlocks(api, latestBlock, async (block) => {
    console.log('finished with blocks')
    await blocks.processEvents(block)
    console.log('finished with process events')

    await pools.fetchPools(block)
    await staking.processStaking(api, block)
    await staking.processLiquidStaking(api, block)
    await store.saveLatest({ timestamp: block.timestamp, block: block.number })
  })
}
