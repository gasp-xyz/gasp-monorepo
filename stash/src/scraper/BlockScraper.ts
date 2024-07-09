import { ApiPromise } from '@polkadot/api'
import { ApiDecoration } from '@polkadot/api/types'
import { BlockHash, SignedBlock } from '@polkadot/types/interfaces'
import _ from 'lodash'
import * as store from '../repository/ChainRepository.js'
import { CodecOrArray, parseNumber, toHuman } from '../util/Chain.js'
import logger from '../util/Logger.js'
import { Metrics } from '../util/Metrics.js'

export const BLOCK_TIME = 12000

export interface Block {
  number: number
  timestamp: number
  hash: string
  parent: string
  extrinsics: Extrinsic[]
  events: [number, Event][]
  api: ApiDecoration<'promise'>
}

export interface Extrinsic {
  method: string
  section: string
  args: object
}

export interface Event {
  method: string
  section: string
  data: object
  index: string
}

export const withBlocks = async (
  api: ApiPromise,
  from: number,
  fn: (b: Block) => Promise<void>
) => {
  const metrics = new Metrics('BlockScraper')
  let last = 0
  let current = from
  const unsub = await api.rpc.chain.subscribeFinalizedHeads(async (head) => {
    const headBlock = await getBlockByHash(api, head.hash)
    last = headBlock.number
    metrics.setBlocks(from, headBlock.number)
    logger.debug(`BlockScraper: new head ${last}`)
  })

  // infinite
  while (last >= 0) {
    if (current >= last) {
      await new Promise((f) => setTimeout(f, BLOCK_TIME))
    }

    store.setBatchMode(current, last)
    const block = await getBlockByNumber(api, current)
    await fn(block)

    metrics.tick()
    current++
  }

  unsub()
}

export const processEvents = async (block: Block) => {
  const events = _.chain(block.events)
    .filter((ev) => filterEvents(ev[1]))
    .groupBy(([idx, _]) => idx)
    .map((evs, _) => evs.map(([_, ev]) => ev))
    .filter((evs) => evs.length > 1)
    .value()

  if (events.length > 0) {
    await store.saveEvents({
      timestamp: block.timestamp,
      block: block.number,
      events,
    })
  }
}

const getBlockByNumber = async (api: ApiPromise, n: number) => {
  const hash = await api.rpc.chain.getBlockHash(n)
  return await getBlockByHash(api, hash)
}

const getBlockByHash = async (api: ApiPromise, hash: BlockHash) => {
  const blockRpc = await api.rpc.chain.getBlock(hash)
  const apiAt: ApiDecoration<'promise'> = await api.at(hash)
  const events = await apiAt.query.system.events()

  return mapper(apiAt, blockRpc, events)
}

const mapper = (
  api: ApiDecoration<'promise'>,
  blockRpc: SignedBlock,
  eventsRpc: CodecOrArray
): Block => {
  const events: [number, Event][] = toHuman(eventsRpc).map(
    ({ phase, event }) => [phase.ApplyExtrinsic, event]
  )
  const extrinsics: Extrinsic[] = toHuman(blockRpc.block.extrinsics).map(
    ({ method }) => method
  )
  const timestamp = getTimestamp(extrinsics)

  const block: Block = {
    number: blockRpc.block.header.number.toNumber(),
    hash: blockRpc.block.hash.toString(),
    parent: blockRpc.block.header.parentHash.toString(),
    events: events,
    extrinsics: extrinsics,
    timestamp,
    api,
  }

  return block
}

const filterEvents = (ev: Event) => {
  return !(ev.section === 'system' && ev.method === 'TxsEnqueued')
}

const getTimestamp = (extrinsics: Extrinsic[]): number => {
  const e = extrinsics.filter(
    ({ section, method }) => 'timestamp.set' === `${section}.${method}`
  )
  return e.length === 1 ? parseNumber(e[0].args['now']) : 0
}
