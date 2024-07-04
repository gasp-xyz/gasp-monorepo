import { Decimal } from 'decimal.js'
import * as store from '../repository/ChainRepository.js'
import { Asset, PoolEntry } from '../repository/ChainRepository.js'
import logger from '../util/Logger.js'
import { ZERO } from '../util/Misc.js'
import { Block } from './BlockScraper.js'
import { StorageKey } from '@polkadot/types'
import { ITuple } from '@polkadot/types-codec/types/interfaces'
import { u32, u128 } from '@polkadot/types-codec/primitive'
import { Option } from '@polkadot/types-codec/base'
import { BN } from '@polkadot/util'

let assets: Map<string, Asset> = new Map()

export const fetchPools = async (block: Block): Promise<void> => {
  await initAssets(block)
  const pools = await getPools(block)
  if (pools !== null && pools !== undefined && pools.length > 0) {
    await store.savePools(pools)
  }
}

const getPools = async (block: Block): Promise<PoolEntry[]> => {
  const pools: [StorageKey<[ITuple<[u32, u32]>]>, ITuple<[u128, u128]>][] =
    await block.api.query.xyk.pools.entries()

  const liquidityTokenIds = (
    await block.api.query.xyk.liquidityAssets.entries()
  ).map(([_, tokenId]) => {
    return tokenId.toString()
  })

  const filteredPools = pools.map(async ([storageKey, poolAssetsAmount]) => {
    const amounts = poolAssetsAmount.toHuman() as [string, string]
    const liquidityAssetsInPool = storageKey.args[0].toHuman() as [
      string,
      string
    ]
    const liquidityPoolId = await block.api.query.xyk.liquidityAssets([
      liquidityAssetsInPool[0].toString(),
      liquidityAssetsInPool[1].toString(),
    ])

    const liquidityPoolIdUnwrapped = JSON.parse(
      JSON.stringify(liquidityPoolId)
    ).toNumber()
    const entry: PoolEntry = {
      id: liquidityPoolIdUnwrapped,
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

  const poolEntries = (await Promise.all(filteredPools)).filter(
    (p) => p.amounts[0].gt(ZERO) && p.amounts[1].gt(ZERO)
  )

  for (const key of liquidityTokenIds) {
    await checkHasAsset(block, key)
  }

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
      } at block ${block.number}`
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
      string
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
