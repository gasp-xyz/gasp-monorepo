import 'dotenv/config'

import app from './app.js'
import * as chainRepository from './repository/ChainRepository.js'
import { BLOCK_TIME } from './scraper/BlockScraper.js'
import * as networkService from './service/NetworkService.js'
import * as blockService from './service/SyncBlockService.js'
import * as syncTransactionsService from './service/SyncTransactionsService.js'
import * as tokenPriceService from './service/TokenPriceService.js'
import * as tokenService from './service/TokenService.js'
import * as xcmService from './service/XcmNetworkService.js'
import * as poolRatesService from './processing/PoolRatesProcessorService.js'
import * as priceService from './processing/PriceProcessorService.js'
import * as store from './repository/ChainRepository.js'
const BASE_TOKEN_ID = 1;
import logger from './util/Logger.js'
// Express Server boot
const server = app.listen(app.get('port'), async () => {
  logger.info(
    'Server started: http://localhost:%d in %s mode',
    app.get('port'),
    app.get('env'),
  )

  await tokenService.initService()

  await networkService.initService()

  await xcmService.initService()

  logger.info('DB initialized')

  await tokenPriceService.refreshTokenPrice()

  blockService.initService()

  syncTransactionsService.initService()

  while (true) {
    await new Promise((f) => setTimeout(f, BLOCK_TIME * 10))
    let processedByPoolRates = await poolRatesService.processRates();
    let processedByPriceService = await priceService.processPrices(BASE_TOKEN_ID);
      const mergedProcessed = new Map([...processedByPoolRates])
      for (const [key, value] of processedByPriceService) {
        if (!mergedProcessed.has(key) || value < mergedProcessed.get(key)) {
          mergedProcessed.set(key, value)
        }
      }
      await store.removeUnusedKeys(mergedProcessed)
  }

})

let isRefreshing = false

const runPeriodically = async () => {
  logger.warn("Refreshing prices")
  if (isRefreshing) {
    return
  }
  isRefreshing = true
  await tokenPriceService.refreshTokenPrice()
  isRefreshing = false
}

setInterval(
  runPeriodically,
  Number(process.env.MINUTES_FOR_TOKEN_REFRESH) * 60 * 1000,
)

export default server
