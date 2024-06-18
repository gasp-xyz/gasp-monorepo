import {
  BadRequestException,
  HttpResponseException,
} from '../error/Exception.js'
import fetch from 'cross-fetch'
import { Decimal } from 'decimal.js'

const coinGeckoApi = 'https://pro-api.coingecko.com/api/v3'

export const getCoinInfo = async (
  tokenId: string
): Promise<CoinGeckoCoinData> => {
  if (tokenId == null || tokenId.length <= 0)
    throw new BadRequestException('Missing token ID information.')

  const coinDataResponse = await fetch(coinGeckoApi + '/coins/' + tokenId, {
    method: 'get',
    headers: {
      Accept: 'application/json',
      'x-cg-demo-api-key': process.env.COINGECKO_API_KEY!,
    },
  })

  if (coinDataResponse.status !== HttpStatus.OK)
    throw new HttpResponseException(
      'Coin Gecko returned unexpected status. Status: ' +
        coinDataResponse.status
    )

  return (await coinDataResponse.json()) as CoinGeckoCoinData
}

// gets daily prices at 00:00 utc
export const getCoinHistory = async (
  tokenId: string,
  days: number,
  currency: string = 'usd'
): Promise<CoinGeckoPrice[]> => {
  if (tokenId == null || tokenId.length <= 0)
    throw new BadRequestException('Missing token ID information.')

  const coinDataResponse = await fetch(
    `${coinGeckoApi}/coins/${tokenId}/market_chart?vs_currency=${currency}&days=${days}&interval=daily`,
    {
      method: 'get',
      headers: {
        Accept: 'application/json',
        'x-cg-demo-api-key': process.env.COINGECKO_API_KEY!,
      },
    }
  )

  if (coinDataResponse.status !== HttpStatus.OK)
    throw new HttpResponseException(
      'Coin Gecko returned unexpected status. Status: ' +
        coinDataResponse.status
    )

  const prices: CoinGeckoPrice[] = (await coinDataResponse.json()).prices.map(
    ([ts, p]) => {
      return {
        timestamp: ts,
        price: new Decimal(p),
      } as CoinGeckoPrice
    }
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
