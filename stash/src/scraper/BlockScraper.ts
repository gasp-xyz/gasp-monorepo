import { ApiPromise } from '@polkadot/api'
import { ApiDecoration } from '@polkadot/api/types'
import { BlockHash, SignedBlock } from '@polkadot/types/interfaces'
import _ from 'lodash'

import * as store from '../repository/ChainRepository.js'
import { CodecOrArray, toHuman } from '../util/Chain.js'
import logger from '../util/Logger.js'
import { Metrics } from '../util/Metrics.js'

class AsyncBlockBuffer {
  private queue: Block[] = []
  private readonly bufferSize: number
  private readonly api: ApiPromise
  private fillInProgress = false
  private currentBlockNumber: number

  constructor(api: ApiPromise, startBlock: number, bufferSize = 5) {
    this.api = api
    this.currentBlockNumber = startBlock
    this.bufferSize = bufferSize
  }

  async initialize() {
    await this.fillBuffer()
  }

  async getNextBlock(): Promise<Block | null> {
    if (this.queue.length === 0) {
      return null
    }

    const block = this.queue.shift()!
    this.fillBuffer()
    return block
  }

  private async fillBuffer() {
    if (this.fillInProgress || this.queue.length >= this.bufferSize) {
      return
    }

    this.fillInProgress = true
    try {
      const blocksToFetch = this.bufferSize - this.queue.length
      const fetchPromises: Promise<Block>[] = []

      for (let i = 0; i < blocksToFetch; i++) {
        const blockNumber = this.currentBlockNumber + i
        fetchPromises.push(getBlockByNumber(this.api, blockNumber))
      }

      const blocks = await Promise.all(fetchPromises)
      this.queue.push(...blocks)
      this.currentBlockNumber += blocks.length
      logger.info(`current block number in buffer: ${this.currentBlockNumber}`)
    } catch (error) {
      logger.error('Error filling block buffer:', error)
    } finally {
      this.fillInProgress = false
    }
  }

  isEmpty(): boolean {
    return this.queue.length === 0
  }

  size(): number {
    return this.queue.length
  }
}

export const BLOCK_TIME = 6000

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
  fn: (b: Block) => Promise<void>,
) => {
  const metrics = new Metrics('BlockScraper')
  let last = 0
  let current = from
  const buffer = new AsyncBlockBuffer(api, from, 5)

  const unsub = await api.rpc.chain.subscribeFinalizedHeads(async (head) => {
    const headBlock = await getBlockByHash(
      api,
      head.hash as unknown as BlockHash,
    )
    last = headBlock.number
    metrics.setBlocks(from, headBlock.number)
    logger.debug(`BlockScraper: new head ${last}`)
  })

  await buffer.initialize()

  // infinite
  while (last >= 0) {
    if (current >= last) {
      await new Promise((f) => setTimeout(f, BLOCK_TIME))
      continue
    }

    store.setBatchMode(current, last)

    let stepStart = Date.now()
    const block = await buffer.getNextBlock()
    if (!block) {
      await new Promise((f) => setTimeout(f, BLOCK_TIME))
      continue
    }
    let timings = Date.now() - stepStart
    logger.debug(`Fetch block time : ${timings}`)


    stepStart = Date.now()
    await fn(block)
    let processingTime = Date.now() - stepStart

    if (block.number % 100 === 0) {
      logger.warn(`Block #${block.number} processed in ${processingTime}ms`)
    }

    metrics.tick()
    current++
  }

  unsub()
}

export const processEvents = async (block: Block) => {
  const events = _.chain(block.events)
    .filter((ev) => filterEvents(ev[1]))
    .groupBy(([idx]) => idx)
    .map((evs) => evs.map(([, ev]) => ev))
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
  logger.debug(`BlockScraper: getBlock by hash - ${hash}`)
  const blockRpc = await api.rpc.chain.getBlock(hash)
  const apiAt: ApiDecoration<'promise'> = await api.at(hash)
  const events = await apiAt.query.system.events()

  return mapper(apiAt, blockRpc, events)
}

const mapper = (
  api: ApiDecoration<'promise'>,
  blockRpc: SignedBlock,
  eventsRpc: CodecOrArray,
): Block => {
  const events: [number, Event][] = toHuman(eventsRpc).map(
    ({ phase, event }) => [phase.ApplyExtrinsic, event],
  )
  const extrinsics: Extrinsic[] = toHuman(blockRpc.block.extrinsics).map(
    ({ method }) => method,
  )
  const timestamp = getTimestamp(extrinsics)

  return {
    number: blockRpc.block.header.number.toNumber(),
    hash: blockRpc.block.hash.toString(),
    parent: blockRpc.block.header.parentHash.toString(),
    events: events,
    extrinsics: extrinsics,
    timestamp,
    api,
  }
}

const filterEvents = (ev: Event) => {
  return !(ev.section === 'system' && ev.method === 'TxsEnqueued')
}

const getTimestamp = (extrinsics: Extrinsic[]): number => {
  const e = extrinsics.filter(
    ({ section, method }) => 'timestamp.set' === `${section}.${method}`,
  )
  return e.length === 1
    ? Number(parseInt(e[0].args['now'].toString().replace(/,/g, '')))
    : 0
}
