import { BN } from '@polkadot/util'
import * as tradesStore from '../repository/TradeVolumeRepository.js'
import * as priceStore from '../repository/PriceRepository.js'
import { TimestampedAmount } from '../schema/Models.js'
import { Decimal } from 'decimal.js'
import * as volumeStore from '../repository/VolumeRepository.js'
import MangataClient from '../connector/MangataNode.js'
import { MainTokens } from 'gasp-sdk'

const LAST_INDEX = -1
const SECOND_ELEMENT_INDEX = 1

export type TokenInfoStats = {
  tokenId: string
  tokenName: string
  symbol: string
  priceInUSD: string
  volume24hInUSD: string
  liquidity24hInUSD: string
  priceChange24hInPerc: string
  volumeChange24hInPerc: string
}

export const getPoolVolumeInUsd = async (
  poolId: string,
  from: number,
  to: number,
  interval: number
) => await tradesStore.get(poolId, true, from, to, interval)

export const getTokenVolumeInUsd = async (
  tokenId: string,
  from: number,
  to: number,
  interval: number
) => await tradesStore.get(tokenId, false, from, to, interval)

export const getTokenPrices = async (
  tokenId: string,
  tokenDecimals: number,
  from: number,
  to: number,
  interval: number
) =>
  (await priceStore.get(tokenId, from, to, interval)).map(
    ([t, a]) =>
      [t, a.mul(new Decimal(`1e${tokenDecimals}`))] as TimestampedAmount
  )

export const getLiquidityPoolInUsd = async (
  poolId: string,
  from: number,
  to: number,
  interval: number
) => await volumeStore.get(poolId, true, from, to, interval)

export const getLiquidityTokenInUsd = async (
  tokenId: string,
  from: number,
  to: number,
  interval: number
) => await volumeStore.get(tokenId, false, from, to, interval)

export const calculatePrice = async (
  baseTokenReserve: BN,
  targetTokenReserve: BN,
  decimals: number
) => {
  return await MangataClient.rpc.calculateBuyPrice({
    inputReserve: targetTokenReserve,
    outputReserve: baseTokenReserve,
    amount: new BN(`${10 ** decimals}`),
  })
}

export const getTokenInfoBasedOnTheSymbol = (
  assetsInfo: MainTokens,
  currencySymbol: string
) => {
  return Object.values(assetsInfo).filter(
    (info) => info.symbol === currencySymbol
  )[0]
}

export const poolsWithNonZeroIssuance = async (assetsInfo: MainTokens) => {
  const filteredPools = Object.entries(assetsInfo)
    .map(([id, info]) => ({ id, ...info }))
    .filter((info) => info.name.includes('Liquidity'))
    .filter((info) => info.symbol.split('-').length <= 2)

  return (
    await Promise.all(
      filteredPools.map(async (info) => {
        const issuance = await MangataClient.query.getTotalIssuance(info.id)
        return { ...info, issuance }
      })
    )
  ).filter((pool) => !pool.issuance.isZero())
}

export const calculateVolumeForTheToken = (
  poolTradeVolumeInUsd: TimestampedAmount[],
  tokenPrices: TimestampedAmount[]
) => {
  if (poolTradeVolumeInUsd.length === 0 || tokenPrices.length === 0)
    return new Decimal(0)
  const lastPoolVolumeEntry =
    poolTradeVolumeInUsd.at(LAST_INDEX)[SECOND_ELEMENT_INDEX]
  const lastTokenPrice = tokenPrices.at(LAST_INDEX)[SECOND_ELEMENT_INDEX]
  return lastPoolVolumeEntry.div(lastTokenPrice)
}

export const calculateLiquidityInUsd = (
  liquidityPoolInUsd: TimestampedAmount[]
) => {
  if (liquidityPoolInUsd.length === 0) return new Decimal(0)
  return liquidityPoolInUsd.at(LAST_INDEX)[SECOND_ELEMENT_INDEX]
}

export const calculate24PriceChange = (tokenPrices: TimestampedAmount[]) => {
  if (tokenPrices.length === 0) return new Decimal(0)
  const currentTokenPrice = tokenPrices.at(LAST_INDEX)[SECOND_ELEMENT_INDEX]
  const priceAt24hour = tokenPrices[0][SECOND_ELEMENT_INDEX]
  return currentTokenPrice
    .sub(priceAt24hour)
    .div(priceAt24hour)
    .mul(100)
    .toFixed(2)
}

export const calculate24VolumeChange = (tokenVolumes: TimestampedAmount[]) => {
  if (tokenVolumes.length === 0) return new Decimal(0)
  const currentVolume = tokenVolumes.at(LAST_INDEX)[SECOND_ELEMENT_INDEX]
  const volumeAt24hour = tokenVolumes[0][SECOND_ELEMENT_INDEX]
  return currentVolume
    .sub(volumeAt24hour)
    .div(volumeAt24hour)
    .mul(100)
    .toFixed(2)
}
