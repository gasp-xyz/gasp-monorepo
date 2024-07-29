import { Decimal } from 'decimal.js'
import _ from 'lodash'
import moment from 'moment'
import { getCoinHistory } from '../connector/CoinGecko.js'
import * as chainStore from '../repository/ChainRepository.js'
import { Asset, PoolEntry } from '../repository/ChainRepository.js'
import * as priceStore from '../repository/PriceRepository.js'
import { TimestampedAmount } from '../schema/Models.js'
import { BASE_TOKEN } from '../util/Chain.js'
import logger from '../util/Logger.js'
import { ZERO } from '../util/Misc.js'

const ETHEREUM_ID = 'ethereum'
const ETH_DECIMALS = new Decimal(1e18)

export const initService = async (base: number = BASE_TOKEN) => {
  await prepareKusama()
  await processPrices(base)
}

const processPrices = async (base: number) => {
  // we will price in all assets until head, if an asset can't be priced it is safe to use head as latest
  const head = (await chainStore.getLatest()).timestamp
  const assets = await getAssets(base)
  const pricedIn = [base]
  logger.debug(`PriceService: have assets: ${assets}`)
  for (const asset of assets) {
    // both assets have prices already, we need to traverse twice with base as both
    if (pricedIn.includes(asset.pool[0]) && pricedIn.includes(asset.pool[1])) {
      logger.debug(
        `PriceService: assets ${asset} has both base prices, traverse twice`
      )
      const latest = await priceStore.getLatest(asset.id)
      await processAsset(asset.pool[0], asset, head)
      // reset the latest a process again with other id as base
      await priceStore.saveLatest(asset.id, latest)
      await processAsset(asset.pool[1], asset, head)
      continue
    }
    // get the unpriced asset id
    const id = pricedIn.includes(asset.pool[0])
      ? asset.pool[1]
      : pricedIn.includes(asset.pool[1])
      ? asset.pool[0]
      : -1
    if (id < 0) {
      logger.debug(
        `PriceService: assets ${asset} has no base prices, save latest`
      )
      await priceStore.saveLatest(asset.id, head)
      continue
    }

    await processAsset(id, asset, head)
    pricedIn.push(id)
  }
}

const processAsset = async (id: number, asset: Asset, head: number) => {
  const base = asset.pool[0] === id ? asset.pool[1] : asset.pool[0]
  logger.info(
    `PriceService: processing asset ${asset} with ${base} as base price`
  )
  let pools = []
  let processed = 0
  do {
    const from = await priceStore.getLatest(asset.id)
    pools = await chainStore.getPools(asset.id, from, head)
    logger.debug(
      `PriceService: processing pool ${asset} in ${from}-${head} with ${pools.length} pools`
    )
    processed += await processPools(asset, base, id, pools)
  } while (pools.length === chainStore.LIMIT)

  logger.info(`PriceService: priced in asset ${id} with ${processed} prices`)
}

const processPools = async (
  asset: Asset,
  base: number,
  id: number,
  pools: PoolEntry[]
) => {
  if (pools.length === 0) {
    return 0
  }
  const basePrices = await getPrices(base, pools)
  logger.debug(
    `PriceService: for ${asset} asset & base ${base} have ${basePrices.size} prices`
  )

  let shouldFilter = false
  let prices = pools.map((p) => {
    const basePrice = getPrice(base, p.timestamp, basePrices)
    const ratio = getPoolRatio(base, p)
    if (ratio.eq(ZERO) || basePrice.eq(ZERO)) {
      shouldFilter = true
    }
    return [p.timestamp, basePrice.mul(ratio)] as TimestampedAmount
  })
  if (shouldFilter) {
    prices = prices.filter((p) => !p[1].eq(ZERO))
  }

  logger.debug(`PriceService: processed for ${id} with ${prices.length} prices`)
  await priceStore.save(asset.id, id, prices, _.last(pools).timestamp)
  return prices.length
}

const getPrices = async (id: number, pools: chainStore.PoolEntry[]) => {
  const first = _.first(pools)
  const last = _.last(pools)
  const from =
    id === BASE_TOKEN
      ? moment.utc(first.timestamp).startOf('day').valueOf()
      : first.timestamp
  const to =
    id === BASE_TOKEN
      ? moment.utc(last.timestamp).startOf('day').valueOf()
      : last.timestamp
  const prices = await priceStore.get(id, from, to)
  return new Map(prices)
}

const getPrice = (
  id: number,
  timestamp: number,
  prices: Map<number, Decimal>
) => {
  let key = timestamp
  if (id === BASE_TOKEN) {
    key = _.findLast([...prices.keys()], (tsp) => timestamp >= tsp)
  }
  return prices.has(key) ? prices.get(key) : ZERO
}

const getPoolRatio = (base: number, pool: PoolEntry) => {
  if (pool.amounts[0].eq(ZERO) || pool.amounts[1].eq(ZERO)) {
    return ZERO
  }
  const idx = pool.assets[0] === base ? [0, 1] : [1, 0]
  const ratio = pool.amounts[idx[0]].div(pool.amounts[idx[1]])
  return ratio
}

const prepareKusama = async () => {
  const latestStoredTimestamp = await priceStore.getLatest(BASE_TOKEN)
  const latest =
    latestStoredTimestamp === 0
      ? moment.utc().subtract(360, 'days')
      : moment.utc(await priceStore.getLatest(BASE_TOKEN))

  const current = moment.utc()
  const diff = current.diff(latest, 'days')

  if (diff === 0) {
    return
  }

  const prices = await getCoinHistory(ETHEREUM_ID, diff)

  await priceStore.save(
    BASE_TOKEN,
    BASE_TOKEN,
    prices.map((p) => [p.timestamp, p.price.div(ETH_DECIMALS)]),
    prices.length === 0 ? 0 : _.last(prices).timestamp
  )
  logger.debug(
    `PriceService: fetched 'ethereum'
     ${prices.length} prices`
  )
}

const getAssets = async (base: number) => {
  let stored = await chainStore.getAssets()
  const ids = [base]
  const assets = []
  while (stored.length > 0) {
    const [acc, next] = _.partition(
      stored,
      (a) => ids.includes(a.pool[0]) || ids.includes(a.pool[1])
    )
    if (acc.length > 0) {
      assets.push(...acc)
      ids.push(...acc.map((a) => a.pool).flat())
      stored = next
    } else {
      assets.push(...next)
      break
    }
  }
  return assets
}
