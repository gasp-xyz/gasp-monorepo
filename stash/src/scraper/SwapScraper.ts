import { ApiPromise } from '@polkadot/api'
import _ from 'lodash'

import { redis } from '../connector/RedisConnector.js'
import { swapRepository } from '../repository/TransactionRepository.js'
import { getTokenPrice } from '../service/TokenPriceService.js'
import logger from '../util/Logger.js'
import * as redisUtil from '../util/Redis.js'
import { Block, Event } from './BlockScraper'

export const processSwapEvents = async (api: ApiPromise, block: Block) => {
  const events = _.chain(block.events)
    .filter((ev) => filterEvents(ev[1]))
    .groupBy(([idx]) => idx)
    .map((evs) => evs.map(([, ev]) => ev))
    .value()
  if (events.length > 0) {
    for (const eventGroup of events) {
      for (const event of eventGroup) {
        try {
          await processDataForDashboard(api, event)
        } catch (e) {
          logger.error('Error processing data for dashboard:', e)
        }

        try {
          await processDataForVolumeHistory(api, event)
        } catch (e) {
          logger.error('Error processing data for volume history:', e)
        }

        try {
          await processDataForTVLHistory(api, event)
        } catch (e) {
          logger.error('Error processing data for TVL history:', e)
        }
      }
    }
  }
}

export const processDataForDashboard = async (
  api: ApiPromise,
  event: Event,
) => {
  logger.info('event asset swapped received: ', event)
  const account = (event.data as any).who
  for (const swap of (event.data as any).swaps) {
    try {
      const tokenId = swap.assetOut
      const eventVolume = swap.amountOut
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
        const newVolume =
          decimals !== null
            ? await calculateVolume(tokenId, decimals, eventVolume)
            : 0

        existingRecord.totalVolume = existingRecord.totalVolume + newVolume
        await swapRepository.save(existingRecord)
        logger.info('Existing record updated with data:', existingRecord)
      } else {
        //we got the trade for new account
        logger.info('No records found, creating a new one for a swap')
        const { decimals } = await decimalsFromTokenId(api, tokenId)
        const volume =
          decimals !== null
            ? await calculateVolume(tokenId, decimals, eventVolume)
            : 0
        const newRecord = {
          account: account,
          lastTradeTimestamp: new Date(),
          daysActive: 1,
          totalVolume: volume,
          totalTrades: 1,
        }
        await swapRepository.save(newRecord)
        logger.info('New record saved with data:', newRecord)
      }
    } catch (e) {
      logger.error(
        'Error processing data for a specific swap for dashboard:',
        e,
      )
    }
  }
}

export const processDataForVolumeHistory = async (
  api: ApiPromise,
  event: Event,
) => {
  logger.info('Entered processDataForVolumeHistory')
  // Implementation for processing data for volume history
  for (const swap of (event.data as any).swaps) {
    try {
      logger.info('Swap data:', swap)
      const { assetIn, assetOut, amountIn, amountOut, poolId } = swap
      //update pool volume
      const poolVolumeKey = `trades:pool:${poolId}`
      await checkKey(poolVolumeKey, [
        'pool',
        poolId,
        'asset',
        assetIn,
        'asset',
        assetOut,
      ])
      const [, poolVolumeValueRaw] = (await redis.client.call(
        'TS.GET',
        poolVolumeKey,
      )) as [string, string]
      const poolVolumeValue =
        poolVolumeValueRaw !== undefined ? parseFloat(poolVolumeValueRaw) : 0
      logger.info(
        `Fetched pool volume for ${poolId}, latest value from the database is : ${poolVolumeValue}`,
      )
      const { decimals: decimalsIn } = await decimalsFromTokenId(api, assetIn)
      const volumeInUSD =
        decimalsIn !== null
          ? await calculateVolume(assetIn, decimalsIn, amountIn)
          : 0
      const { decimals: decimalsOut } = await decimalsFromTokenId(api, assetOut)
      const volumeOutUSD =
        decimalsOut !== null
          ? await calculateVolume(assetOut, decimalsOut, amountOut)
          : 0

      const newPoolVolume =
        volumeInUSD === 0 || volumeOutUSD === 0
          ? poolVolumeValue
          : poolVolumeValue + Number(volumeInUSD) + Number(volumeOutUSD)
      redis.client.call('TS.ADD', poolVolumeKey, '*', newPoolVolume)
      logger.info(
        `Formula for poolId ${poolId} new volume is =  ${poolVolumeValue} + ${Number(
          volumeInUSD,
        )} + ${Number(
          volumeOutUSD,
        )} but if the price of one token is 0 pool volume stays unchanged`,
      )
      logger.info(
        `Updated pool volume for ${poolId}, new value in the database is : ${newPoolVolume}`,
      )
      //update pool ALL volume
      const ALLpoolVolumeKey = `trades:pool:ALL`
      await checkKey(ALLpoolVolumeKey, ['pool', 'ALL'])
      const [, ALLpoolVolumeValueRaw] = (await redis.client.call(
        'TS.GET',
        ALLpoolVolumeKey,
      )) as [string, string]
      const ALLpoolVolumeValue =
        ALLpoolVolumeValueRaw !== undefined
          ? parseFloat(ALLpoolVolumeValueRaw)
          : 0
      const ALLnewPoolVolume =
        volumeInUSD === 0 || volumeOutUSD === 0
          ? ALLpoolVolumeValue
          : ALLpoolVolumeValue +
            (volumeInUSD as number) +
            (volumeOutUSD as number)
      redis.client.call('TS.ADD', ALLpoolVolumeKey, '*', ALLnewPoolVolume)
      logger.info(
        `Formula for ALL pools new volume is =  ${ALLpoolVolumeValue} + ${Number(
          volumeInUSD,
        )} + ${Number(
          volumeOutUSD,
        )} but if the price of one token is 0 ALL pool volume stays unchanged`,
      )
      logger.info(`Updated pool volume for ALL: ${ALLnewPoolVolume}`)
      //update assets volume
      const assetInVolumeKey = `trades:asset:${assetIn}`
      await checkKey(assetInVolumeKey, ['asset', assetIn])
      const [, assetInVolumeValueRaw] = (await redis.client.call(
        'TS.GET',
        assetInVolumeKey,
      )) as [string, string]
      const assetInVolumeValue =
        assetInVolumeValueRaw !== undefined
          ? parseFloat(assetInVolumeValueRaw)
          : 0
      logger.info(
        `Fetched volume for asset with id ${assetIn}, value from the database is: ${assetInVolumeValue}`,
      )
      const newAssetInVolume = assetInVolumeValue + (volumeInUSD as number)
      redis.client.call('TS.ADD', assetInVolumeKey, '*', newAssetInVolume)
      logger.info(
        `Formula for assetId ${assetIn} new volume is =  ${assetInVolumeValue} + ${Number(
          volumeInUSD,
        )}`,
      )
      logger.info(
        `Updated volume for asset with id ${assetIn}, new value in the database is: ${newAssetInVolume}`,
      )

      const assetOutVolumeKey = `trades:asset:${assetOut}`
      await checkKey(assetOutVolumeKey, ['asset', assetOut])
      const [, assetOutVolumeValueRaw] = (await redis.client.call(
        'TS.GET',
        assetOutVolumeKey,
      )) as [string, string]
      const assetOutVolumeValue =
        assetOutVolumeValueRaw !== undefined
          ? parseFloat(assetOutVolumeValueRaw)
          : 0
      logger.info(
        `Fetched volume for asset with id ${assetOut}, value from the database is: ${assetOutVolumeValue}`,
      )
      const newAssetOutVolume = assetOutVolumeValue + (volumeOutUSD as number)
      redis.client.call('TS.ADD', assetOutVolumeKey, '*', newAssetOutVolume)
      logger.info(
        `Formula for assetId ${assetOut} new volume is =  ${assetOutVolumeValue} + ${Number(
          volumeOutUSD,
        )}`,
      )

      logger.info(
        `Updated volume for asset with id ${assetOut}, new value in the database is: ${newAssetOutVolume}`,
      )
    } catch (e) {
      logger.error(
        'Error processing data for a specific swap for volume history:',
        e,
      )
    }
  }
}

export async function checkKey(key: string, label: string[]): Promise<void> {
  const exist = await redisUtil.hasKey(redis, key)
  if (!exist) {
    redis.client.call(
      'TS.CREATE',
      key,
      'RETENTION',
      '0',
      'DUPLICATE_POLICY',
      'SUM',
      'LABELS',
      ...label,
    )
  }
}

export const processDataForTVLHistory = async (
  api: ApiPromise,
  event: Event,
) => {
  logger.info('Entered processDataForTVLHistory')
  // Implementation for processing data for TVL history
  for (const swap of (event.data as any).swaps) {
    try {
      logger.info('Swap data:', swap)
      const { assetIn, assetOut, amountIn, amountOut, poolId } = swap
      //update pool TVL
      const poolTVLKey = `volumes:pool:${poolId}`
      await checkKey(poolTVLKey, [
        'pool',
        poolId,
        'asset',
        assetIn,
        'asset',
        assetOut,
      ])
      const [, poolTVLValueRaw] = (await redis.client.call(
        'TS.GET',
        poolTVLKey,
      )) as [string, string]
      const poolTVLValue =
        poolTVLValueRaw !== undefined ? parseFloat(poolTVLValueRaw) : 0
      logger.info(
        `Fetched pool TVL for ${poolId}, value in the database is: ${poolTVLValue}`,
      )
      const { decimals: decimalsIn } = await decimalsFromTokenId(api, assetIn)
      const volumeInUSD =
        decimalsIn !== null
          ? await calculateVolume(assetIn, decimalsIn, amountIn)
          : 0
      const { decimals: decimalsOut } = await decimalsFromTokenId(api, assetOut)
      const volumeOutUSD =
        decimalsOut !== null
          ? await calculateVolume(assetOut, decimalsOut, amountOut)
          : 0
      const newPoolTVL =
        volumeInUSD === 0 || volumeOutUSD === 0
          ? poolTVLValue
          : poolTVLValue + (volumeInUSD as number) - (volumeOutUSD as number)
      redis.client.call('TS.ADD', poolTVLKey, '*', newPoolTVL)
      logger.info(
        `Formula for poolId ${poolId} new TVL is =  ${poolTVLValue} + ${Number(
          volumeInUSD,
        )} - ${Number(
          volumeOutUSD,
        )} but if the price of one token is 0 pool TVL stays unchanged`,
      )
      logger.info(
        `Updated pool TVL for ${poolId}, new value in the database is: ${newPoolTVL}`,
      )
      //update pool ALL TVL
      const ALLpoolTVLKey = `volumes:pool:ALL`
      await checkKey(ALLpoolTVLKey, ['pool', 'ALL'])
      const [, ALLpoolTVLValueRaw] = (await redis.client.call(
        'TS.GET',
        ALLpoolTVLKey,
      )) as [string, string]
      const ALLpoolTVLValue =
        ALLpoolTVLValueRaw !== undefined ? parseFloat(ALLpoolTVLValueRaw) : 0
      logger.info(
        `Fetched pool TVL ALL pools, value in the database is: ${ALLpoolTVLValue}`,
      )
      const ALLnewPoolTVL =
        volumeInUSD === 0 || volumeOutUSD === 0
          ? ALLpoolTVLValue
          : ALLpoolTVLValue + (volumeInUSD as number) - (volumeOutUSD as number)
      redis.client.call('TS.ADD', ALLpoolTVLKey, '*', ALLnewPoolTVL)
      logger.info(
        `Formula for ALL pools new TVL is =  ${ALLpoolTVLValue} + ${Number(
          volumeInUSD,
        )} - ${Number(
          volumeOutUSD,
        )} but if the price of one token is 0 ALL pool TVL stays unchanged`,
      )
      logger.info(
        `Updated pool TVL for ALL, new value in the database is: ${ALLnewPoolTVL}`,
      )
      //update assets TVL
      const assetInTVLKey = `volumes:asset:${assetIn}`
      await checkKey(assetInTVLKey, ['asset', assetIn])
      const [, assetInTVLValueRaw] = (await redis.client.call(
        'TS.GET',
        assetInTVLKey,
      )) as [string, string]
      const assetInTVLValue =
        assetInTVLValueRaw !== undefined ? parseFloat(assetInTVLValueRaw) : 0
      logger.info(
        `Fetched TVL for asset with id ${assetIn}, value in the database is: ${assetInTVLValue}`,
      )
      const newAssetInTVL = assetInTVLValue + (volumeInUSD as number)
      redis.client.call('TS.ADD', assetInTVLKey, '*', newAssetInTVL)
      logger.info(
        `Formula for assetId ${assetIn} new TVL is =  ${assetInTVLValue} + ${Number(
          volumeInUSD,
        )}`,
      )
      logger.info(
        `Updated TVL for asset with id ${assetIn}, new value in the database is: ${newAssetInTVL}`,
      )
      const assetOutTVLKey = `volumes:asset:${assetOut}`
      await checkKey(assetOutTVLKey, ['asset', assetOut])
      const [, assetOutTVLValueRaw] = (await redis.client.call(
        'TS.GET',
        assetOutTVLKey,
      )) as [string, string]
      const assetOutTVLValue =
        assetOutTVLValueRaw !== undefined ? parseFloat(assetOutTVLValueRaw) : 0
      logger.info(
        `Fetched TVL for asset with id ${assetOut}, value in the database is : ${assetOutTVLValue}`,
      )
      const newAssetOutTVL =
        assetOutTVLValue - (volumeOutUSD as number) < 0
          ? 0
          : assetOutTVLValue - (volumeOutUSD as number)
      redis.client.call('TS.ADD', assetOutTVLKey, '*', newAssetOutTVL)
      logger.info(
        `Formula for assetId ${assetOut} new TVL is =  ${assetOutTVLValue} - ${Number(
          volumeOutUSD,
        )} but if the formula value is negative, new TVL value will be 0`,
      )
      logger.info(
        `Updated TVL for asset with id ${assetOut}, new value in the database is: ${newAssetOutTVL}`,
      )
    } catch (e) {
      logger.error(
        'Error processing data for a specific swap for TVL history:',
        e,
      )
    }
  }
}

export const filterEvents = (ev: Event) => {
  return ev.section === 'market' && ev.method === 'AssetsSwapped'
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
  volume: string,
): Promise<number | string> {
  try {
    const price = await getTokenPrice(tokenId)
    if (price == null) {
      throw new Error(`Token price for token id ${tokenId} is null`)
    }
    const currentPrice = price.toString()
    const volumeString = volume.toString().replace(/,/g, '')
    const volumeBigInt = BigInt(volumeString)
    return (
      parseFloat(currentPrice) * (Number(volumeBigInt) / Math.pow(10, decimals))
    )
  } catch (error) {
    logger.error(
      `Error: Unable to retrieve token price data for token id ${tokenId}`,
      error,
    )
    return 0
  }
}
