import 'dotenv/config'

import app from './app.js'
import * as poolRatesService from './processing/PoolRatesProcessorService.js'
import * as priceService from './processing/PriceProcessorService.js'
import { BLOCK_TIME } from './scraper/BlockScraper.js'
import * as networkService from './service/NetworkService.js'
import * as blockService from './service/SyncBlockService.js'
import * as syncTransactionsService from './service/SyncTransactionsService.js'
import * as tokenPriceService from './service/TokenPriceService.js'
import * as tokenService from './service/TokenService.js'
import * as xcmService from './service/XcmNetworkService.js'
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

  blockService.initService()

  syncTransactionsService.initService()

  const run = 1
  while (run) {
    await new Promise((f) => setTimeout(f, BLOCK_TIME * 10))
    await poolRatesService.initService()
    await priceService.initService()
  }
})

let isRefreshing = false

const runPeriodically = async () => {
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
