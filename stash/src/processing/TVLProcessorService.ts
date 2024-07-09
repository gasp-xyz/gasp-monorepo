import { Decimal } from 'decimal.js'
import _ from 'lodash'
import moment from 'moment'
import * as chainStore from '../repository/ChainRepository.js'
import * as priceStore from '../repository/PriceRepository.js'
import * as volumeStore from '../repository/VolumeRepository.js'
import { TimestampedAmount, TimestampedAmountPool } from '../schema/Models.js'
import { BASE_TOKEN } from '../util/Chain.js'
import logger from '../util/Logger.js'
import { ZERO } from '../util/Misc.js'

export const initService = async () => {
  await processVolumes()
}

const processVolumes = async () => {
  const assets = await chainStore.getAssets()
  logger.info(`VolumeService: have assets: ${assets}`)
  for (const asset of assets) {
    logger.info(`VolumeService: processing asset ${asset}`)
    await processAsset(asset)
  }
}

const processAsset = async (asset: chainStore.Asset) => {
  let pools = []
  let processed = 0
  do {
    const latest = await volumeStore.getLatest(asset.id)
    const to = await priceStore.getLatest(asset.id)
    pools = await chainStore.getPools(asset.id, latest, to)
    logger.debug(
      `VolumeService: processing pool ${asset} in ${latest}-${to} with ${pools.length} pools`
    )
    processed += await processPools(asset, pools)
  } while (pools.length === chainStore.LIMIT)
  logger.info(
    `VolumeService: processed asset ${asset} with ${processed} volumes`
  )
}

const processPools = async (
  asset: chainStore.Asset,
  pools: chainStore.PoolEntry[]
) => {
  if (pools.length === 0) {
    return 0
  }
  const ids = asset.pool
  const [prices_0, prices_1] = await getPrices(ids, pools)
  if (prices_0.size === 0 || prices_1.size === 0) {
    await volumeStore.save(asset, [], _.last(pools).timestamp)
    logger.debug(
      `VolumeService: zero prices for ${ids} with ${pools.length} pools`
    )
    return 0
  }
  logger.debug(
    `VolumeService: has ${prices_0.size},${prices_1.size} prices for ${ids}`
  )

  let shouldFilter = false
  let amounts: TimestampedAmountPool[] = pools.map((p) => {
    const price_0 = getPrice(p.assets[0], p.timestamp, prices_0)
    const price_1 = getPrice(p.assets[1], p.timestamp, prices_1)
    if (price_0.eq(ZERO) || price_1.eq(ZERO)) {
      shouldFilter = true
      return [p.timestamp, [ZERO, ZERO]]
    }
    return [p.timestamp, [p.amounts[0].mul(price_0), p.amounts[1].mul(price_1)]]
  })
  if (shouldFilter) {
    amounts = amounts.filter(
      ([tsp, amounts]) => !(amounts[0].eq(ZERO) || amounts[1].eq(ZERO))
    )
  }

  await volumeStore.save(asset, amounts, _.last(pools).timestamp)
  logger.debug(
    `VolumeService: processed ${amounts.length}/${pools.length} pools`
  )
  return amounts.length
}

const getPrices = async (
  ids: [number, number],
  pools: chainStore.PoolEntry[]
) => {
  const first = _.first(pools)
  const last = _.last(pools)
  const prices: TimestampedAmount[][] = []
  for (const asset of ids) {
    const from =
      asset === BASE_TOKEN
        ? moment.utc(first.timestamp).startOf('day').valueOf()
        : first.timestamp
    const to =
      asset === BASE_TOKEN
        ? moment.utc(last.timestamp).startOf('day').valueOf()
        : last.timestamp
    prices.push(await priceStore.get(asset, from, to))
  }
  return [new Map(prices[0]), new Map(prices[1])]
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
