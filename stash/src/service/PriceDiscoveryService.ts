import { fromBN, TokenInfo } from 'gasp-sdk'
import { BN } from '@polkadot/util'
import { Decimal } from 'decimal.js'
import _ from 'lodash'
import moment from 'moment'
import { StringSchema } from 'yup'
import * as coinGecko from '../connector/CoinGecko.js'
import { CoinGeckoCoinData } from '../connector/CoinGecko.js'
import MangataClient from '../connector/MangataNode.js'
import { NotFoundException } from '../error/Exception.js'
import * as chainStore from '../repository/ChainRepository.js'
import * as ratesStore from '../repository/PoolRatesRepository.js'
import * as priceStore from '../repository/PriceRepository.js'
import * as tradesStore from '../repository/TradeVolumeRepository.js'
import * as volumeStore from '../repository/VolumeRepository.js'
import { ASSET_ALL, TimestampedAmount } from '../schema/Models.js'
import { currencyIdSchemaFn, Interval } from '../schema/PriceDiscoverySchema.js'
import logger from '../util/Logger.js'

const minInSec = 60
const secInMs = 1000
const coinGeckoId = process.env.APP_ENV === 'rollup-dev' ? 'ethereum' : 'kusama'
const tokenForPriceSymbol = process.env.TOKEN_FOR_PRICE_SYMBOL

let baseTokenPrice: CoinGeckoCoinData = null
let assetInfo: Map<string, TokenInfo>
let assetSchema: StringSchema
let poolSchema: StringSchema
let nextPriceRefresh = new Date(Date.now() - 60 * minInSec * secInMs)

function isTimeForRefresh(lastRefreshTime: Date) {
  const intervalMs = 60 * minInSec * secInMs // 60 minutes interval

  // Calculate the current time in milliseconds
  const currentTimeMs = Date.now()

  // Calculate the last refresh time in milliseconds
  const lastRefreshTimeMs = lastRefreshTime.getTime()

  // Check if the difference between current time and last refresh time exceeds the interval
  return currentTimeMs - lastRefreshTimeMs >= intervalMs
}

const refresh = async () => {
  if (
    baseTokenPrice == null ||
    assetInfo == null ||
    isTimeForRefresh(nextPriceRefresh)
  ) {
    baseTokenPrice = await coinGecko.getCoinInfo(coinGeckoId)

    logger.info('Refreshing asset info.')
    assetInfo = new Map()
    const storedPools = await chainStore.getAssets()
    const registeredAssets = Object.values(
      await MangataClient.query.getAssetsInfo()
    )
    const idToAsset = new Map(
      registeredAssets.map((a) => [Number.parseInt(a.id), a])
    )

    // only pools we have stored in DB
    const assetIds = _.uniq(storedPools.map((p) => p.pool).flat())
    const assets = registeredAssets.filter((a) =>
      assetIds.includes(Number.parseInt(a.id))
    )
    assetInfo = new Map(assets.map((a) => [a.symbol, a]))

    // only pools we have stored in DB
    const poolIds = []
    for (const pool of storedPools) {
      const asset = idToAsset.get(pool.id)
      // if we dont have the pool asset in registry, skip
      if (!asset) continue
      const asset_0 = idToAsset.get(pool.pool[0])
      const asset_1 = idToAsset.get(pool.pool[1])
      const keys = [
        `${asset_0.symbol}-${asset_1.symbol}`,
        `${asset_1.symbol}-${asset_0.symbol}`,
      ]

      assetInfo.set(keys[0], asset)
      assetInfo.set(keys[1], asset)
      poolIds.push(keys[0])
      poolIds.push(keys[1])
    }
    // add ALL aggregate
    assetInfo.set(ASSET_ALL.symbol, ASSET_ALL)
    poolIds.push(ASSET_ALL.id)

    assetSchema = currencyIdSchemaFn(assets.map((a) => a.symbol))
    poolSchema = currencyIdSchemaFn(poolIds)

    nextPriceRefresh = new Date()
  }
}

const getAsset = (symbol: string, isPool: boolean) => {
  isPool ? poolSchema.validateSync(symbol) : assetSchema.validateSync(symbol)
  return assetInfo.get(symbol)
}

export const priceDiscovery = async (
  currencySymbol: string
): Promise<PriceDiscoveryDto> => {
  await refresh()

  const calculatedAsset = getAsset(currencySymbol, false)
  const priceTokenAssetInfo = getAsset(tokenForPriceSymbol, false)

  if (calculatedAsset == null)
    throw new NotFoundException('Unknown currency symbol.')

  const buyAmount = new BN(1 + '0'.repeat(calculatedAsset.decimals))
  const buyPriceInPriceToken = await MangataClient.rpc.calculateBuyPriceId(
    priceTokenAssetInfo.id,
    calculatedAsset.id,
    buyAmount
  )
  const priceTokenDecimalValue = new Decimal(
    fromBN(buyPriceInPriceToken, priceTokenAssetInfo.decimals)
  )
  const results: Record<string, string> = {}

  for (const currency in baseTokenPrice.market_data.current_price) {
    results[currency] =
      currencySymbol === 'GETH'
        ? new Decimal(1)
            .times(baseTokenPrice.market_data.current_price[currency])
            .toString()
        : priceTokenDecimalValue
            .times(baseTokenPrice.market_data.current_price[currency])
            .toString()
  }

  return {
    current_price: results,
  }
}

export const priceHistoryPair = async (
  base: string,
  target: string,
  days: number | 'max' = 'max',
  interval: number | Interval = 0
): Promise<PriceHistoryDto> => {
  await refresh()

  const intervalAdjusted = interval === 0 ? adjustInterval(days) : interval
  const baseAsset = getAsset(base, false)
  const targetAsset = getAsset(target, false)
  const exponent = new Decimal(`1e${baseAsset.decimals}`).div(
    new Decimal(`1e${targetAsset.decimals}`)
  )
  const current = moment.utc()
  const to = current.valueOf()
  const from = days === 'max' ? 0 : current.subtract(days, 'days').valueOf()
  const prices = (
    await ratesStore.get(
      baseAsset.id,
      targetAsset.id,
      from,
      to,
      matchInterval(intervalAdjusted)
    )
  ).map(([t, a]) => [t, a.mul(exponent)] as TimestampedAmount)

  return { prices }
}

export const priceHistory = async (
  currencySymbol: string,
  days: number | 'max' = 'max',
  interval: number | Interval = 0
): Promise<PriceHistoryDto> => {
  await refresh()

  const intervalAdjusted = interval === 0 ? adjustInterval(days) : interval
  const asset = getAsset(currencySymbol, false)
  const exponent = new Decimal(`1e${asset.decimals}`)
  const current = moment.utc()
  const to = current.valueOf()
  const from = days === 'max' ? 0 : current.subtract(days, 'days').valueOf()
  const prices = (
    await priceStore.get(asset.id, from, to, matchInterval(intervalAdjusted))
  ).map(([t, a]) => [t, a.mul(exponent)] as TimestampedAmount)

  return { prices }
}

export const volumeHistory = async (
  currencySymbol: string,
  isPool: boolean,
  days: number | 'max' = 'max',
  interval: number | Interval = 0
): Promise<VolumeHistoryDto> => {
  await refresh()

  const intervalAdjusted = interval === 0 ? adjustInterval(days) : interval
  const asset = getAsset(currencySymbol, isPool)
  const current = moment.utc()
  const to = current.valueOf()
  const from = days === 'max' ? 0 : current.subtract(days, 'days').valueOf()
  const volumes = await volumeStore.get(
    asset.id,
    isPool,
    from,
    to,
    matchInterval(intervalAdjusted)
  )

  return { volumes }
}

export const tradesHistory = async (
  currencySymbol: string,
  isPool: boolean,
  days: number | 'max' = 'max',
  interval: number | Interval = 0
): Promise<VolumeHistoryDto> => {
  await refresh()

  const intervalAdjusted = interval === 0 ? adjustInterval(days) : interval
  const asset = getAsset(currencySymbol, isPool)
  const current = moment.utc()
  const to = current.valueOf()
  const from = days === 'max' ? 0 : current.subtract(days, 'days').valueOf()
  const volumes = await tradesStore.get(
    asset.id,
    isPool,
    from,
    to,
    matchInterval(intervalAdjusted)
  )

  return { volumes }
}

export const adjustInterval = (days: number | string): Interval => {
  if (!_.isNumber(days) || days > 90) {
    return 'day'
  }
  if (days <= 5) {
    return 'minute'
  }
  return 'hour'
}

export const matchInterval = (interval: number | Interval): number => {
  if (_.isNumber(interval)) {
    return interval * 1000
  }
  switch (interval) {
    case 'day':
      return 1000 * 60 * 60 * 24
    case '12H':
      return 1000 * 60 * 60 * 12
    case '6H':
      return 1000 * 60 * 60 * 6
    case 'hour':
      return 1000 * 60 * 60
    case '30m':
      return 1000 * 60 * 30
    case '15m':
      return 1000 * 60 * 15
    case 'minute':
      return 1000 * 60
    default:
      return 0
  }
}

interface PriceDiscoveryDto {
  current_price: Record<string, string>
}

interface PriceHistoryDto {
  prices: TimestampedAmount[]
}

interface VolumeHistoryDto {
  volumes: TimestampedAmount[]
}
