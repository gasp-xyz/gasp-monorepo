import { fromBN } from '@mangata-finance/sdk'
import moment from 'moment'
import MangataClient from '../connector/MangataNode.js'
import { matchInterval } from './PriceDiscoveryService.js'
import {
  calculatePrice,
  calculateVolumeForTheToken,
  getPoolVolumeInUsd,
  getTokenInfoBasedOnTheSymbol,
  getTokenPrices,
  poolsWithNonZeroIssuance,
} from '../util/Listing.js'

export const summary = async () => {
  const assetsInfo = await MangataClient.query.getAssetsInfo()

  // We need to filter out pools that have zero issuance.
  // If we don't filter them out, it will create several problems
  // during the calculation process.
  const pools = await poolsWithNonZeroIssuance(assetsInfo)

  const summaryMap = {}
  await Promise.all(
    pools.map(async (pool) => {
      const [baseToken, targetToken] = pool.symbol.split('-')
      const baseTokenInfo = getTokenInfoBasedOnTheSymbol(assetsInfo, baseToken)
      const targetTokenInfo = getTokenInfoBasedOnTheSymbol(
        assetsInfo,
        targetToken
      )

      const [baseTokenReserve, targetTokenReserve] =
        await MangataClient.query.getAmountOfTokensInPool(
          baseTokenInfo.id,
          targetTokenInfo.id
        )

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
        matchInterval('minute')
      )
      const baseTokenPrices = await getTokenPrices(
        baseTokenInfo.id,
        baseTokenInfo.decimals,
        from,
        to,
        matchInterval('minute')
      )
      const targetTokenPrices = await getTokenPrices(
        targetTokenInfo.id,
        targetTokenInfo.decimals,
        from,
        to,
        matchInterval('minute')
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

      summaryMap[pool.symbol.replace('-', '_')] = {
        base_id: baseTokenInfo.id,
        base_name: baseTokenInfo.name,
        base_symbol: baseTokenInfo.symbol,
        quote_id: targetTokenInfo.id,
        quote_name: targetTokenInfo.name,
        quote_symbol: targetTokenInfo.symbol,
        last_price: lastPrice,
        base_volume: baseVolume.toString(),
        quote_volume: targetVolume.toString(),
      }
    })
  )

  return summaryMap
}
