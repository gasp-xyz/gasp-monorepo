import 'gasp-types'

import cors from 'cors'
import express, { json, Request, Response, urlencoded } from 'express'
import { createRequire } from 'module'
import swaggerUI from 'swagger-ui-express'

import * as coingeckoController from './controller/coingeckoListingController.js'
import * as coinmarketcapController from './controller/coinmarketcapListingController.js'
import * as faucetController from './controller/FaucetController.js'
import * as keyValueController from './controller/KeyValueController.js'
import * as liquidStakingController from './controller/liquidStakingController.js'
import * as airdropController from './controller/mgxAirdropController.js'
import * as networkController from './controller/networkController.js'
import * as priceDiscoveryController from './controller/PriceDiscoveryController.js'
import * as stakingAprController from './controller/stakingAprController.js'
import * as tokenController from './controller/TokenController.js'
import * as tokenListController from './controller/tokenListController.js'
import * as tokenNetworkPortfolioController from './controller/tokenNetworkPortfolioController.js'
import * as tokenPriceController from './controller/TokenPriceController.js'
import * as tracingController from './controller/TracingController.js'
import * as tradingController from './controller/TradingController.js'
import * as xcmController from './controller/XcmController.js'
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
app.get('/token/:id/stats', tokenListController.tokenDetails)

// XCM
app.get('/xcm/channels', xcmController.channels)
app.get('/xcm/network', xcmController.network)
app.get('/xcm/tokens', xcmController.tokens)

app.get(
  '/collator/:collatorAddress/staking/dailyReward',
  stakingAprController.dailyReward,
)
app.get('/collator/:collatorAddress/staking/apy', stakingAprController.apy)
app.get('/collators/staking/apy', stakingAprController.collatorsApy)

app.get(
  '/account/:address/liquid-staking/rewards-history/24h/sum',
  liquidStakingController.rewards24hours,
)
app.get(
  '/account/:address/liquid-staking/rewards-history/month/sum',
  liquidStakingController.rewardsMonth,
)

// Price Discovery API
app.get('/price-discovery/:currencyId/', priceDiscoveryController.getPrice)
app.get(
  '/price-history/pair/:baseCurrencyId/:targetCurrencyId',
  priceDiscoveryController.getHistoryPair,
)
app.get('/price-history/:currencyId/', priceDiscoveryController.getHistory)
app.get(
  '/volume-history/pools/:currencyId/',
  priceDiscoveryController.getTradesPool,
)
app.get('/volume-history/:currencyId/', priceDiscoveryController.getTradesAsset)
app.get(
  '/tvl-history/pools/:currencyId/',
  priceDiscoveryController.getVolumePool,
)
app.get('/tvl-history/:currencyId/', priceDiscoveryController.getVolumeAsset)

// CoinGecko listing endpoints
app.get('/coingecko/pairs', coingeckoController.pairs)
app.get('/coingecko/tickers', coingeckoController.tickers)

app.get('/affirmed-network/list', networkController.networkList)
app.get('/affirmed-token/list', networkController.tokenList)
app.get(
  '/account/:address/token-portfolio',
  tokenNetworkPortfolioController.tokenNetworkPortfolio,
)

//Faucet endpoint
app.get(
  '/faucet/requestTokens/:toAddress/captcha/:captcha/',
  faucetController.captcha,
)

// Tracing endpoints
app.post('/tracing/tx/start', tracingController.startTracing)

app.get(
  '/tracing/type/:type/tx/:txHashOrEntityId',
  tracingController.getTransactionByTxHashOrEntityId,
)

app.get(
  '/tracing/type/:type/tx/listByAddress/:address',
  tracingController.getAllTransactionsByAddress,
)

app.get(
  '/tracing/type/:type/tx/listByAddress/:address/:status',
  tracingController.getAllTransactionsByAddressAndStatus,
)

//Token prices endpoint
app.get('/price/token/:tokenId', tokenPriceController.getTokenPrices)

// Dashboard endpoint
app.get('/account/:wallet/dashboard', tradingController.getData)

// Airdrop endpoints
app.get('/mgx-airdrop/eligibility/:address', airdropController.checkEligibility)
app.post('/mgx-airdrop/link-address', airdropController.linkAddress)

// Coinmarketcap listing endpoints
app.get('/coinmarketcap/v1/summary', coinmarketcapController.summary)

//key value storage endpoints
app.post('/key-value/store', keyValueController.store)

app.use('/doc', swaggerUI.serve, swaggerUI.setup(swaggerFile))

export default app
