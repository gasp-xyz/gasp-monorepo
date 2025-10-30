import fetch from 'cross-fetch'
import { Decimal } from 'decimal.js'

import {
  BadRequestException,
  HttpResponseException,
} from '../error/Exception.js'

const coinGeckoApi = 'https://pro-api.coingecko.com/api/v3'

// Rate limiting state
let callCount = 0
let lastResetTime = Date.now()
const RATE_LIMIT = 30
const RATE_WINDOW = 60000 // 1 minute in milliseconds

const sleep = (ms: number): Promise<void> =>
  new Promise((resolve) => setTimeout(resolve, ms))

const checkRateLimit = async (): Promise<void> => {
  const now = Date.now()

  // Reset counter if more than 1 minute has passed
  if (now - lastResetTime >= RATE_WINDOW) {
    callCount = 0
    lastResetTime = now
  }

  // If we've hit the limit, wait for the remainder of the window
  if (callCount >= RATE_LIMIT) {
    const timeToWait = RATE_WINDOW - (now - lastResetTime)
    if (timeToWait > 0) {
      console.log(
        `Rate limit reached (${RATE_LIMIT} calls/minute). Waiting ${Math.ceil(timeToWait / 1000)} seconds...`,
      )
      await sleep(timeToWait)
      callCount = 0
      lastResetTime = Date.now()
    }
  }

  callCount++
}

export const getCoinInfo = async (
  tokenId: string,
): Promise<CoinGeckoCoinData> => {
  if (tokenId == null || tokenId.length <= 0)
    throw new BadRequestException('Missing token ID information.')

  await checkRateLimit()

  const url = new URL(coinGeckoApi + '/coins/' + tokenId)
  url.searchParams.append('x_cg_pro_api_key', process.env.COINGECKO_API_KEY!)

  const coinDataResponse = await fetch(url, {
    method: 'get',
    headers: {
      Accept: 'application/json',
    },
  })

  if (coinDataResponse.status !== HttpStatus.OK)
    throw new HttpResponseException(
      'Coin Gecko returned unexpected status. Status: ' +
        coinDataResponse.status,
    )

  return (await coinDataResponse.json()) as CoinGeckoCoinData
}

// gets daily prices at 00:00 utc
export const getCoinHistory = async (
  tokenId: string,
  days: number,
  currency: string = 'usd',
): Promise<CoinGeckoPrice[]> => {
  if (tokenId == null || tokenId.length <= 0)
    throw new BadRequestException('Missing token ID information.')

  await checkRateLimit()

  const url = new URL(`${coinGeckoApi}/coins/${tokenId}/market_chart`)
  url.searchParams.append('vs_currency', currency)
  url.searchParams.append('days', days.toString())
  url.searchParams.append('interval', 'daily')
  url.searchParams.append('x_cg_pro_api_key', process.env.COINGECKO_API_KEY!)

  const headers = {
    Accept: 'application/json',
  }

  const coinDataResponse = await fetch(url.toString(), {
    method: 'get',
    headers: headers,
  })

  if (coinDataResponse.status !== HttpStatus.OK) {
    const errorBody = await coinDataResponse.text()
    throw new HttpResponseException(
      `Coin Gecko returned unexpected status. Status: ${coinDataResponse.status}, Body: ${errorBody}`,
    )
  }

  const prices: CoinGeckoPrice[] = (await coinDataResponse.json()).prices.map(
    ([ts, p]) => {
      return {
        timestamp: ts,
        price: new Decimal(p),
      } as CoinGeckoPrice
    },
  )

  // last one is a price at a call time, not daily
  prices.pop()
  return prices
}

export interface CoinGeckoCoinData {
  id: string
  symbol: string
  name: string
  market_data: {
    current_price: Record<string, number>
  }
}

export interface CoinGeckoPrice {
  timestamp: number
  price: Decimal
}

export abstract class HttpStatus {
  static readonly OK: number = 200
}
