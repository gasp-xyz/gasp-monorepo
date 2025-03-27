import fetch from 'cross-fetch'

import MangataClient from '../connector/MangataNode.js'
import { tokenPricesRepository } from '../repository/TransactionRepository.js'
import * as priceDiscoveryService from '../service/PriceDiscoveryService.js'
import logger from '../util/Logger.js'

let assets: any = {}
const coinGeckoApi = 'https://pro-api.coingecko.com/api/v3'

class CoinGeckoCoinPriceData {
  [contractAddress: string]: {
    usd: number
    usd_market_cap: number
    usd_24h_vol: number
    usd_24h_change: number
    last_updated_at: number
  }
}

export const refreshTokenPrice = async () => {
  await refreshTokenList()
  await refreshPrices()
  logger.info('Token prices refreshed')
}

async function refreshTokenList() {
  const api = await MangataClient.api()
  const tokens = await api.query.assetRegistry.metadata.entries()
  assets = await tokens.reduce(async (objPromise, [key, value]: [any, any]) => {
    const obj = await objPromise
    const [tokenId] = key.args
    const { decimals, name } = value.unwrap()
    if (
      !Buffer.from(name.toString().slice(2), 'hex')
        .toString('utf-8')
        .startsWith('LiquidityPoolToken') //todo: improve token recognition
    ) {
      const l1Asset = await api.query.assetRegistry.idToL1Asset(tokenId)
      let key = Object.keys(l1Asset.toHuman())[0]
      if (key.toLowerCase() === 'arbitrum') {
        key = 'arbitrum-one' //coingecko uses arbitrum-one
      }
      const value = Object.values(l1Asset.toHuman())[0]

      obj[tokenId.toString()] = {
        id: tokenId.toString(),
        decimals: Number(decimals.toPrimitive()),
        name: Buffer.from(name.toString().slice(2), 'hex').toString('utf-8'),
        chainId: key.toLowerCase(),
        contractAddress: value.toString(),
      }
    }
    return obj
  }, Promise.resolve({}))
  for (const asset of Object.values(assets) as any) {
    const exists = await tokenPricesRepository
      .search()
      .where('tokenId')
      .equals(asset.id)
      .returnFirst()
    if (exists) {
      continue //we have this assset already
    }
    const token = {
      tokenId: asset.id,
      chainId: asset.chainId,
      contractAddress: asset.contractAddress,
    }

    await tokenPricesRepository.save(token)
  }
  logger.info('Newest list of tokens from the node ', assets)
}

async function getCoingeckoPrice(chainId: any, contractAddress: any) {
  const url = new URL(`${coinGeckoApi}/simple/token_price/${chainId}`)
  url.searchParams.append('contract_addresses', contractAddress)
  url.searchParams.append('vs_currencies', 'usd')
  url.searchParams.append('x_cg_pro_api_key', process.env.COINGECKO_API_KEY!)
  const coinPriceResponse = await fetch(url, {
    method: 'get',
    headers: {
      Accept: 'application/json',
    },
  })

  if (coinPriceResponse.status !== HttpStatus.OK) {
    logger.error(
      'Coin Gecko returned unexpected status. Status: ' +
        coinPriceResponse.status +
        coinPriceResponse.statusText
    )
    return null
  }
  const coinPriceData = await coinPriceResponse.json()
  return coinPriceData as CoinGeckoCoinPriceData
}

async function getPoolPrice(tokenId: any) {
  try {
    const response = await priceDiscoveryService.priceDiscovery(tokenId)
    const currentPrice = response.current_price['usd']
    if (currentPrice !== null && parseFloat(currentPrice) !== 0) {
      logger.info(
        `Price found in price discovery for tokenId ${tokenId}:`,
        currentPrice
      )
      return currentPrice
    }
  } catch (error) {
    logger.error('Price discovery did not return the price,:', error)
  }
  try {
    logger.info(
      `Price discovery is 0 for tokenId ${tokenId}, calling price history`
    )
    const priceHistoryData = await priceDiscoveryService.priceHistory(tokenId)
    return priceHistoryData.prices.length > 0
      ? priceHistoryData.prices[
          priceHistoryData.prices.length - 1
        ][1].toString()
      : undefined //if there is no price-history data fallback to undefined
  } catch (error) {
    logger.error(
      'Price history did not return the price for tokenId:',
      tokenId,
      error
    )
    return undefined
  }
}

async function refreshPrices() {
  const tokens = await tokenPricesRepository.search().return.all()

  for (const token of tokens) {
    if (
      new Date(token.timestamp) >
      new Date(
        Date.now() - Number(process.env.MINUTES_FOR_TOKEN_REFRESH) * 60 * 1000
      )
    ) {
      logger.info('Token price is fresh, skipping', token.tokenId)
      continue
    }
    logger.info(
      'Price is stale, refreshing for token',
      token.tokenId,
      token.chainId,
      token.contractAddress
    )
    let price = await getCoingeckoPrice(token.chainId, token.contractAddress)
    if (price && Object.keys(price).length > 0) {
      logger.info(
        'Coingecko returned price for token id',
        token.tokenId,
        price[token.contractAddress].usd
      )
      token.price = price[token.contractAddress].usd
      token.timestamp = new Date()
      await tokenPricesRepository.save(token)
      logger.info('Price refreshed from Coingecko for token id', token.tokenId)
      continue
    }
    logger.info(
      'Price not found on coingecko, trying local memory for token ',
      token.tokenId
    )
    const poolPrice = await getPoolPrice(token.tokenId)
    if (poolPrice && Object.keys(poolPrice).length > 0) {
      logger.info(
        'Price found in local memory for token id',
        token.tokenId,
        poolPrice
      )
      token.price = parseFloat(poolPrice)
      token.timestamp = new Date()
      await tokenPricesRepository.save(token)
    }
    //if we don't have coingecko nor pool price, we don't update the token price
  }
}

export async function getTokenPrice(tokenId: string): Promise<number | null> {
  const token = await tokenPricesRepository
    .search()
    .where('tokenId')
    .equals(tokenId)
    .returnFirst()
  return token ? token.price : null
}

export abstract class HttpStatus {
  static readonly OK: number = 200
}
