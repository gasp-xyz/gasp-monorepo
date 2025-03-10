import { fromBN } from 'gasp-sdk'
import moment from 'moment'

import MangataClient from '../connector/MangataNode.js'
import { matchInterval } from './PriceDiscoveryService.js'
import {
  calculateLiquidityInUsd,
  calculatePrice,
  calculateVolumeForTheToken,
  getLiquidityPoolInUsd,
  getPoolVolumeInUsd,
  getTokenInfoBasedOnTheSymbol,
  getTokenPrices,
  poolsWithNonZeroIssuance,
} from '../util/Listing.js'

export const pairs = async () => {
  const assetsInfo = await MangataClient.query.getAssetsInfo()
  const pools = await poolsWithNonZeroIssuance(assetsInfo)
  const pairs = []
  for (const key in pools) {
    // We do not want pools such as KSM-MGX-BNC
    // this is deep liquidity, and they do not support such notation
    if (pools[key].symbol.split('-').length <= 2) {
      const token = pools[key]
      const [base, target] = token.symbol.split('-')

      pairs.push({
        ticker_id: token.symbol.replace('-', '_'),
        base,
        target,
        pool_id: token.id,
      })
    }
  }

  return pairs
}

export const tickers = async () => {
  const assetsInfo = await MangataClient.query.getAssetsInfo()
  // We need to filter out pools that have zero issuance.
  // If we don't filter them out, it will create several problems
  // during the calculation process.
  const pools = await poolsWithNonZeroIssuance(assetsInfo)

  const tickers = []
  await Promise.all(
    pools.map(async (pool) => {
      const [baseToken, targetToken] = pool.symbol.split('-')
      const baseTokenInfo = getTokenInfoBasedOnTheSymbol(assetsInfo, baseToken)
      const targetTokenInfo = getTokenInfoBasedOnTheSymbol(
        assetsInfo,
        targetToken
      )

      const poolBalance = await MangataClient.query.getAmountOfTokensInPool(
        baseTokenInfo.id,
        targetTokenInfo.id
      )
      const baseTokenReserve = poolBalance[0]
      const targetTokenReserve = poolBalance[1]

      const price = await calculatePrice(
        baseTokenReserve,
        targetTokenReserve,
        baseTokenInfo.decimals
      )

      const current = moment.utc()
      const to = current.valueOf()
      const from = current.subtract(1, 'days').valueOf()

      const poolTradeVolumeInUsd = await getPoolVolumeInUsd(
        pool.id,
        from,
        to,
        matchInterval('day')
      )
      const baseTokenPrices = await getTokenPrices(
        baseTokenInfo.id,
        baseTokenInfo.decimals,
        from,
        to,
        matchInterval('day')
      )
      const targetTokenPrices = await getTokenPrices(
        targetTokenInfo.id,
        targetTokenInfo.decimals,
        from,
        to,
        matchInterval('day')
      )
      const liquidityPoolInUsd = await getLiquidityPoolInUsd(
        pool.id,
        from,
        to,
        matchInterval('day')
      )

      const lastPrice = fromBN(price, targetTokenInfo.decimals)
      const baseVolume = calculateVolumeForTheToken(
        poolTradeVolumeInUsd,
        baseTokenPrices
      )
      const targetVolume = calculateVolumeForTheToken(
        poolTradeVolumeInUsd,
        targetTokenPrices
      )
      const liquidityInUsd = calculateLiquidityInUsd(liquidityPoolInUsd)

      tickers.push({
        ticker_id: pool.symbol.replace('-', '_'),
        base_currency: baseToken,
        target_currency: targetToken,
        last_price: lastPrice.toString(),
        base_volume: baseVolume.toString(),
        target_volume: targetVolume.toString(),
        pool_id: pool.id,
        liquidity_in_usd: liquidityInUsd.toString(),
      })
    })
  )

  return tickers
}
