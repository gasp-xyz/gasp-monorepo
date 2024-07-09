import _ from 'lodash'
import * as chainStore from '../repository/ChainRepository.js'
import * as ratesStore from '../repository/PoolRatesRepository.js'
import { TimestampedBaseTargetAmount } from '../schema/Models.js'
import logger from '../util/Logger.js'

export const initService = async () => {
  await processRates()
}

const processRates = async () => {
  const head = (await chainStore.getLatest()).timestamp
  const assets = await chainStore.getAssets()
  logger.info(`PoolRatesService: have assets: ${assets}`)
  for (const asset of assets) {
    logger.info(`PoolRatesService: processing asset ${asset}`)
    await processAsset(asset, head)
  }
}

const processAsset = async (asset: chainStore.Asset, head: number) => {
  let pools = []
  let processed = 0
  do {
    const latest = await ratesStore.getLatest(asset.id)
    pools = await chainStore.getPools(asset.id, latest, head)
    logger.debug(
      `PoolRatesService: processing pool ${asset} in ${latest}-${head} with ${pools.length} pools`
    )
    processed += await processPools(asset, pools)
  } while (pools.length === chainStore.LIMIT)
  logger.info(
    `PoolRatesService: processed asset ${asset} with ${processed} volumes`
  )
}

const processPools = async (
  asset: chainStore.Asset,
  pools: chainStore.PoolEntry[]
) => {
  if (pools.length === 0) {
    return 0
  }
  const rates: TimestampedBaseTargetAmount[] = pools.map((p) => [
    p.timestamp,
    p.assets[0],
    p.assets[1],
    p.amounts[1].div(p.amounts[0]),
  ])
  const rates_rev: TimestampedBaseTargetAmount[] = pools.map((p) => [
    p.timestamp,
    p.assets[1],
    p.assets[0],
    p.amounts[0].div(p.amounts[1]),
  ])

  await ratesStore.save(asset, [rates, rates_rev], _.last(pools).timestamp)
  logger.debug(
    `PoolRatesService: processed ${rates.length}/${pools.length} pools`
  )
  return rates.length
}
