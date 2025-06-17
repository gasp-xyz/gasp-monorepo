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
