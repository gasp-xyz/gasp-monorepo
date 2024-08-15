import MangataClient from '../connector/MangataNode.js'
import * as store from '../repository/ChainRepository.js'
import * as blocks from '../scraper/BlockScraper.js'
import * as pools from '../scraper/PoolsScraper.js'
import * as staking from '../scraper/StakingScraper.js'
import { watchDepositAcceptedIntoQueue } from '../scraper/LogScraper.js'
import logger from '../util/Logger.js'
import { holesky, arbitrumSepolia } from 'viem/chains'
export const initService = async () => {
  const api = await MangataClient.api()

  // Start watching for DepositAcceptedIntoQueue events
  await watchDepositAcceptedIntoQueue(api, process.env.ETH_CHAIN_URL, holesky)
  await watchDepositAcceptedIntoQueue(
    api,
    process.env.ARBITRUM_SEPOLIA_CHAIN_URL,
    arbitrumSepolia
  )

  const latestBlock = (await store.getLatest()).block
  // const latestBlock = 3719278
  await blocks.withBlocks(api, latestBlock, async (block) => {
    try {
      await blocks.processEvents(block)
      await pools.fetchPools(block)
      await staking.processStaking(api, block)
      await staking.processLiquidStaking(api, block)
      await store.saveLatest({
        timestamp: block.timestamp,
        block: block.number,
      })
    } catch (e) {
      logger.error('Error in processing block: ', e)
    }
  })
}
