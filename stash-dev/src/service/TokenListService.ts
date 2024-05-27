import MangataClient from '../connector/MangataNode.js'
import moment from 'moment'
import {
  calculate24PriceChange,
  calculate24VolumeChange,
  getLiquidityTokenInUsd,
  getTokenPrices,
  getTokenVolumeInUsd,
  TokenInfoStats,
} from '../util/Listing.js'
import { matchInterval } from './PriceDiscoveryService.js'
import Decimal from 'decimal.js'
import { NotFoundException } from '../error/Exception.js'

const FILTER_LIQUIDITY_TOKENS = 'Liquidity'

export const tokenDetails = async (
  tokenSymbol: string
): Promise<TokenInfoStats> => {
  const assetsInfo = await MangataClient.query.getAssetsInfo()
  const tokenInfo = Object.values(assetsInfo)
    .filter((asset) => {
      if (process.env.APP_ENV === 'rollup-dev') {
        return asset.name !== 'L1Asset' // we do not want to have L1Asset there
      } else {
        return true
      }
    })
    .find((item) => item.symbol === tokenSymbol)
  if (!tokenInfo) throw new NotFoundException('Unknown currency symbol.')

  const current = moment.utc()
  const to = current.valueOf()
  const from = current.subtract(1, 'days').valueOf()

  const [tokenPrice, tokenVolume, tokenLiquidityInUsd] = await Promise.all([
    getTokenPrices(
      tokenInfo.id,
      tokenInfo.decimals,
      current.subtract(2, 'days').valueOf(),
      to,
      matchInterval('day')
    ),
    getTokenVolumeInUsd(tokenInfo.id, from, to, matchInterval('day')),
    getLiquidityTokenInUsd(tokenInfo.id, from, to, matchInterval('day')),
  ])

  // Calculate changes concurrently
  const [priceChange, volumeChange] = await Promise.all([
    calculate24PriceChange(tokenPrice),
    calculate24VolumeChange(tokenVolume),
  ])

  return {
    tokenId: tokenInfo.id,
    tokenName: tokenInfo.name,
    symbol: tokenInfo.symbol,
    priceInUSD:
      tokenPrice.length === 0
        ? new Decimal(0).toString()
        : tokenPrice.at(-1)[1].toString(),
    volume24hInUSD:
      tokenVolume.length === 0
        ? new Decimal(0).toString()
        : tokenVolume.at(-1)[1].toString(),
    liquidity24hInUSD:
      tokenLiquidityInUsd.length === 0
        ? new Decimal(0).toString()
        : tokenLiquidityInUsd.at(-1)[1].toString(),
    priceChange24hInPerc: priceChange.toString(),
    volumeChange24hInPerc: volumeChange.toString(),
  } as TokenInfoStats
}

export const tokenList = async (): Promise<TokenInfoStats[]> => {
  const assetsInfo = await MangataClient.query.getAssetsInfo()
  const listedTokens = Object.entries(assetsInfo)
    .map(([id, info]) => ({ id, ...info }))
    .filter((asset) => !asset.name.includes(FILTER_LIQUIDITY_TOKENS))
    .filter((asset) => {
      if (process.env.APP_ENV === 'rollup-dev') {
        return asset.name !== 'L1Asset' // we do not want to have L1Asset there
      } else {
        return true
      }
    })

  const tokens: TokenInfoStats[] = []
  await Promise.all(
    listedTokens.map(async (token) => {
      const current = moment.utc()
      const to = current.valueOf()
      const from = current.subtract(1, 'days').valueOf()
      const tokenPrice = await getTokenPrices(
        token.id,
        token.decimals,
        from,
        to,
        matchInterval('hour')
      )
      const tokenVolume = await getTokenVolumeInUsd(
        token.id,
        from,
        to,
        matchInterval('day')
      )
      const tokenLiquidityInUsd = await getLiquidityTokenInUsd(
        token.id,
        from,
        to,
        matchInterval('day')
      )
      const priceChange = calculate24PriceChange(tokenPrice)
      const volumeChange = calculate24VolumeChange(tokenVolume)

      tokens.push({
        tokenId: token.id,
        tokenName: token.name,
        symbol: token.symbol,
        priceInUSD:
          tokenPrice.length === 0
            ? new Decimal(0).toString()
            : tokenPrice.at(-1)[1].toString(),
        volume24hInUSD:
          tokenVolume.length === 0
            ? new Decimal(0).toString()
            : tokenVolume.at(-1)[1].toString(),
        liquidity24hInUSD:
          tokenLiquidityInUsd.length === 0
            ? new Decimal(0).toString()
            : tokenLiquidityInUsd.at(-1)[1].toString(),
        priceChange24hInPerc: priceChange.toString(),
        volumeChange24hInPerc: volumeChange.toString(),
      })
    })
  )
  return tokens
}
