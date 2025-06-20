import fetch from 'cross-fetch'
import { Decimal } from 'decimal.js'

import {
  BadRequestException,
  HttpResponseException,
} from '../error/Exception.js'

const coinGeckoApi = 'https://pro-api.coingecko.com/api/v3'

export const getCoinInfo = async (
  tokenId: string,
): Promise<CoinGeckoCoinData> => {
  if (tokenId == null || tokenId.length <= 0)
    throw new BadRequestException('Missing token ID information.')

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

  if (coinDataResponse.status !== HttpStatus.OK)
    throw new HttpResponseException(
      'Coin Gecko returned unexpected status. Status: ' +
        coinDataResponse.status,
    )

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
