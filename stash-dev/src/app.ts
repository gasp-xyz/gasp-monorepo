import '@mangata-finance/types'
import express, { Request, Response, json, urlencoded } from 'express'
import swaggerUI from 'swagger-ui-express'
import cors from 'cors'

import * as priceDiscoveryController from './controller/PriceDiscoveryController.js'
import * as tokenController from './controller/TokenController.js'
import * as xcmController from './controller/XcmController.js'
import * as coingeckoController from './controller/coingeckoListingController.js'
import * as coinmarketcapController from './controller/coinmarketcapListingController.js'
import * as tokenListController from './controller/tokenListController.js'
import * as stakingAprController from './controller/stakingAprController.js'
import * as liquidStakingController from './controller/liquidStakingController.js'
import * as networkController from './controller/networkController.js'
import * as tokenNetworkPortfolioController from './controller/tokenNetworkPortfolioController.js'

import { createRequire } from 'module'
const require = createRequire(import.meta.url)
const swaggerFile = require('../swagger-output.json')

const app = express()

// App configuration
app.set('port', process.env.PORT || 8080)
app.use(json())
app.use(urlencoded({ extended: true }))
app.use(cors())

//Root (Health Check)
app.get('/', async (req: Request, res: Response): Promise<void> => {
  res.json({})
})

// Tokens
app.get('/token/order-buckets', tokenController.listTokenOrderBuckets)
// Token list
app.get('/token/list/stats', tokenListController.tokenList)
app.get('/token/:symbol/stats', tokenListController.tokenDetails)

// XCM
app.get('/xcm/channels', xcmController.channels)
app.get('/xcm/network', xcmController.network)
app.get('/xcm/tokens', xcmController.tokens)

app.get(
  '/collator/:collatorAddress/staking/dailyReward',
  stakingAprController.dailyReward
)
app.get('/collator/:collatorAddress/staking/apy', stakingAprController.apy)
app.get('/collators/staking/apy', stakingAprController.collatorsApy)

app.get(
  '/account/:address/liquid-staking/rewards-history/24h/sum',
  liquidStakingController.rewards24hours
)
app.get(
  '/account/:address/liquid-staking/rewards-history/month/sum',
  liquidStakingController.rewardsMonth
)

// Price Discovery API
app.get('/price-discovery/:currencySymbol/', priceDiscoveryController.getPrice)
app.get(
  '/price-history/pair/:baseCurrencySymbol/:targetCurrencySymbol',
  priceDiscoveryController.getHistoryPair
)
app.get('/price-history/:currencySymbol/', priceDiscoveryController.getHistory)
app.get(
  '/volume-history/pools/:currencySymbol/',
  priceDiscoveryController.getTradesPool
)
app.get(
  '/volume-history/:currencySymbol/',
  priceDiscoveryController.getTradesAsset
)
app.get(
  '/tvl-history/pools/:currencySymbol/',
  priceDiscoveryController.getVolumePool
)
app.get(
  '/tvl-history/:currencySymbol/',
  priceDiscoveryController.getVolumeAsset
)

// CoinGecko listing endpoints
app.get('/coingecko/pairs', coingeckoController.pairs)
app.get('/coingecko/tickers', coingeckoController.tickers)

if (process.env.APP_ENV === 'rollup-dev') {
  app.get('/affirmed-network/list', networkController.networkList)
  app.get('/affirmed-token/list', networkController.tokenList)
  app.get(
    '/account/:address/token-portfolio',
    tokenNetworkPortfolioController.tokenNetworkPortfolio
  )
}

// Coinmarketcap listing endpoints
app.get('/coinmarketcap/v1/summary', coinmarketcapController.summary)

app.use('/doc', swaggerUI.serve, swaggerUI.setup(swaggerFile))

export default app
