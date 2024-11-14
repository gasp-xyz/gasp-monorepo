import { ApiPromise } from '@polkadot/api'
import { Block, Event } from './BlockScraper'
import _ from 'lodash'
import { swapRepository } from '../repository/TransactionRepository.js'
import logger from '../util/Logger.js'
import * as priceDiscoveryService from '../service/PriceDiscoveryService.js'

export const processSwapEvents = async (api: ApiPromise, block: Block) => {
  const events = _.chain(block.events)
    .filter((ev) => filterEvents(ev[1]))
    .groupBy(([idx, _]) => idx)
    .map((evs, _) => evs.map(([_, ev]) => ev))
    .value()
  if (events.length > 0) {
    for (const eventGroup of events) {
      for (const event of eventGroup) {
        try {
          logger.info('event asset swapped received: ', event)
          const account = event.data[0]
          const tokenId = event.data[1][1]
          const eventVolume = event.data[3]
          const existingRecord = await swapRepository
            .search()
            .where('account')
            .equals(account)
            .returnFirst()
          if (existingRecord) {
            logger.info('existing record found: ', existingRecord)
            const currentDate = new Date()
            const lastTradeDate = new Date(existingRecord.lastTradeTimestamp)
            existingRecord.daysActive =
              currentDate.toDateString() === lastTradeDate.toDateString()
                ? existingRecord.daysActive
                : existingRecord.daysActive + 1
            existingRecord.lastTradeTimestamp = currentDate
            existingRecord.totalTrades += 1
            const { decimals } = await decimalsFromTokenId(api, tokenId)
            const newTotalVolume = await calculateVolume(
              tokenId,
              decimals,
              eventVolume
            )
            if (newTotalVolume !== null) {
              existingRecord.totalVolume = newTotalVolume
            }
            await swapRepository.save(existingRecord)
          } else {
            //we got the trade for new account
            const { decimals } = await decimalsFromTokenId(api, tokenId)
            const volume = await calculateVolume(tokenId, decimals, eventVolume)
            const newRecord = {
              account: account,
              lastTradeTimestamp: new Date(),
              daysActive: 1,
              totalVolume: volume,
              totalTrades: 1,
            }
            await swapRepository.save(newRecord)
          }
        } catch (e) {
          logger.error('Error processing swap event:', e)
        }
      }
    }
  }
}

export const filterEvents = (ev: Event) => {
  return ev.section === 'xyk' && ev.method === 'AssetsSwapped'
}

export async function decimalsFromTokenId(api: ApiPromise, tokenId: any) {
  const tokens = await api.query.assetRegistry.metadata.entries()
  const assets = tokens.reduce((obj, [key, value]) => {
    const [tokenId] = key.args
    const { decimals } = value.unwrap()
    obj[tokenId.toString()] = {
      id: tokenId.toString(),
      decimals: Number(decimals.toPrimitive()),
    }
    return obj
  }, {})
  return assets[tokenId.toString()] || null
}

export async function calculateVolume(
  tokenId: string,
  decimals: number,
  volume: string
): Promise<number | string> {
  try {
    // const response = await getCoinInfo(tokenSymbol)
    const response = await priceDiscoveryService.priceDiscovery(tokenId)
    const currentPrice = response.current_price['usd']
    const volumeString = volume.toString().replace(/,/g, '')
    const volumeBigInt = BigInt(volumeString)
    const finalValue =
      parseFloat(currentPrice) * (Number(volumeBigInt) / Math.pow(10, decimals))
    console.log('current price: ', currentPrice)
    console.log('decimals: ', decimals)
    console.log('final value in dolars: ', finalValue)

    return finalValue
  } catch (error) {
    logger.error(
      `Error: Unable to retrieve token price data for token id ${tokenId}`,
      error
    )
    return null
  }
}
