import { StorageKey } from '@polkadot/types'
import { Option } from '@polkadot/types-codec/base'
import { u32, u128 } from '@polkadot/types-codec/primitive'
import { ITuple } from '@polkadot/types-codec/types/interfaces'
import { BN } from '@polkadot/util'
import { Decimal } from 'decimal.js'

import * as store from '../repository/ChainRepository.js'
import { Asset, PoolEntry } from '../repository/ChainRepository.js'
import { toHuman } from '../util/Chain.js'
import logger from '../util/Logger.js'
import { ZERO } from '../util/Misc.js'
import { Block } from './BlockScraper.js'

let assets: Map<string, Asset> = new Map()

export const fetchPools = async (block: Block): Promise<void> => {
  const initAssetsStart = performance.now()
  await initAssets(block)
  const initAssetsEnd = performance.now()
  logger.info(
    `initAssets benchmark: ${(initAssetsEnd - initAssetsStart).toFixed(2)}ms for block ${block.number}`,
  )

  const getPoolsStart = performance.now()
  const pools = await getPools(block)
  const getPoolsEnd = performance.now()
  logger.info(
    `getPools benchmark: ${(getPoolsEnd - getPoolsStart).toFixed(2)}ms for block ${block.number}, found ${pools?.length || 0} pools`,
  )

  if (pools !== null && pools !== undefined && pools.length > 0) {
    // await store.savePools(pools)
  }
}

const getPools = async (block: Block): Promise<PoolEntry[]> => {
  const poolsQueryStart = performance.now()
  const pools: [StorageKey<[ITuple<[u32, u32]>]>, ITuple<[u128, u128]>][] =
    await block.api.query.xyk.pools.entries()
  const poolsQueryEnd = performance.now()
  logger.info(
    `getPools - pools query: ${(poolsQueryEnd - poolsQueryStart).toFixed(2)}ms for block ${block.number}`,
  )

  const liquidityAssetsQueryStart = performance.now()
  // const liquidityTokenIds = (
  //   await block.api.query.xyk.liquidityAssets.entries()
  // ).map(([, tokenId]) => {
  //   return tokenId.toString()
  // })

  const liquidityTokenIds2 = (
    await block.api.query.xyk.liquidityAssets.entries()
  ).map(([key, tokenId]) => {
    let first = key.args[0][0].toString()
    let second = key.args[0][1].toString()
    return [`${first}:${second}`, tokenId] as [string, BN]
  })

  let mapping = new Map(liquidityTokenIds2)

  const liquidityAssetsQueryEnd = performance.now()
  logger.info(
    `getPools - liquidity assets query: ${(liquidityAssetsQueryEnd - liquidityAssetsQueryStart).toFixed(2)}ms for block ${block.number}`,
  )

  const poolMappingStart = performance.now()
  const filteredPools = pools.map(([storageKey, poolAssetsAmount]) => {
    const amounts = poolAssetsAmount.toHuman() as [string, string]
    const liquidityAssetsInPool = storageKey.args[0].toHuman() as [
      string,
      string,
    ]
    const liquidityPoolId = mapping.get(
      `${liquidityAssetsInPool[0].toString()}:${liquidityAssetsInPool[1].toString()}`,
    )
    // const liquidityPoolId = await block.api.query.xyk.liquidityAssets([
    //   liquidityAssetsInPool[0].toString(),
    //   liquidityAssetsInPool[1].toString(),
    // ])
    const humanLiquidityPoolId = toHuman(liquidityPoolId)
    const numberLiquidityPoolId = Number(humanLiquidityPoolId)
    const entry: PoolEntry = {
      id: numberLiquidityPoolId,
      amounts: [
        new Decimal(amounts[0].replace(/,/g, '').toString()),
        new Decimal(amounts[1].replace(/,/g, '').toString()),
      ],
      assets: [
        Number.parseInt(liquidityAssetsInPool[0]),
        Number.parseInt(liquidityAssetsInPool[1]),
      ],
      block: block.number,
      timestamp: block.timestamp,
    }
    return entry
  })

  const poolEntriesProcessStart = performance.now()
  const poolEntries = (await Promise.all(filteredPools)).filter(
    (p) => p.amounts[0].gt(ZERO) && p.amounts[1].gt(ZERO),
  )
  const poolEntriesProcessEnd = performance.now()
  logger.info(
    `getPools - pool entries processing: ${(poolEntriesProcessEnd - poolEntriesProcessStart).toFixed(2)}ms for block ${block.number}`,
  )
  const poolMappingEnd = performance.now()
  logger.info(
    `getPools - pool mapping total: ${(poolMappingEnd - poolMappingStart).toFixed(2)}ms for block ${block.number}`,
  )

  const assetCheckStart = performance.now()
  for (const [, tokenId] of liquidityTokenIds2) {
    await checkHasAsset(block, tokenId.toString())
  }
  const assetCheckEnd = performance.now()
  logger.info(
    `getPools - asset checks: ${(assetCheckEnd - assetCheckStart).toFixed(2)}ms for block ${block.number}`,
  )

  return poolEntries
}

const initAssets = async (block: Block) => {
  if (assets.size === 0) {
    const stored = await store.getAssets()
    assets = new Map(stored.map((a) => [a.id.toString(), a]))
  }
  if (assets.size === 0) {
    const newAssets = await fetchAssets(block)
    assets = new Map(newAssets.map((a) => [a.id.toString(), a]))
    await store.saveAssets([...assets.values()])
  }
}

const checkHasAsset = async (block: Block, key: string) => {
  if (assets.get(key) === undefined) {
    const newAssets = await fetchAssets(block)
    const asset = newAssets.find((a) => a.id.toString() === key)
    assets.set(asset.id.toString(), asset)
    await store.saveAssets([...assets.values()])
    logger.debug(
      `PoolsScraper: fetch asset ${asset.id.toString()}->${
        asset.pool
      } at block ${block.number}`,
    )
  }
}

const fetchAssets = async (block: Block): Promise<Asset[]> => {
  const liquidityAssets: [StorageKey<[ITuple<[u32, u32]>]>, Option<u32>][] =
    await block.api.query.xyk.liquidityAssets.entries()

  return liquidityAssets.map(([storageKey, optionValue]) => {
    const poolId = optionValue.unwrap() as BN
    const liquidityAssetsInPool = storageKey.args[0].toHuman() as [
      string,
      string,
    ]
    return {
      id: poolId.toNumber(),
      idx: '',
      pool: [
        Number.parseInt(liquidityAssetsInPool[0]),
        Number.parseInt(liquidityAssetsInPool[1]),
      ],
      poolx: '',
      createdAt: block.timestamp,
    }
  })
}
