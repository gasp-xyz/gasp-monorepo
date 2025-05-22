import { Block, Event } from './BlockScraper'
import _ from 'lodash'
import { withdrawalRepository } from '../repository/TransactionRepository.js'
import { ApiPromise } from '@polkadot/api'
import { redis } from '../connector/RedisConnector.js'
import logger from '../util/Logger.js'
import { AnyTuple } from '@polkadot/types-codec/types'
import { GenericExtrinsic } from '@polkadot/types'

const NETWORK_LIST_KEY = 'affirmed_networks_list'
const WITHDRAWAL_PENDING_ON_L2 = 'PendingOnL2'
const WITHDRAWAL_BATCHED_FOR_L1 = 'BatchedForL1'
const type = 'withdrawal'

interface Network {
  key: string
  chainId: string
}

export enum CreatedBy {
  Frontend = 'frontend',
  Other = 'other',
}

export async function extractExtrinsicHashAndAnAddressFromBlock(
  api: ApiPromise,
  phaseApplyExtrinsic: number,
  block: Block
) {
  let extrinsicHash, address
  try {
    const blockHash = await api.rpc.chain.getBlockHash(block.number)
    const blockHeader = await api.rpc.chain.getHeader(blockHash)
    const extinsics: GenericExtrinsic<AnyTuple>[] = (
      await api.rpc.chain.getBlock(blockHeader.hash)
    ).block.extrinsics
    let extrinsic = extinsics[phaseApplyExtrinsic]
    extrinsicHash = extrinsic.hash.toString()
    address = extrinsic.signer.toString()
    console.log('Extrinsic Hash:', extrinsicHash, 'Address:', address)
    return { extrinsicHash, address }
  } catch (error) {
    logger.error('Error extracting extrinsic hash and address:', error)
    return { extrinsicHash: '', address: '' }
  }
}

export const processWithdrawalEvents = async (
  api: ApiPromise,
  block: Block
) => {
  const events = _.chain(block.events)
    .filter((ev) => filterEvents(ev[1]))
    .groupBy(([idx, _]) => idx)
    .map((evs, idx) =>
      evs.map(([phaseApplyExtrinsic, ev]) => ({ phaseApplyExtrinsic, ev }))
    )
    .value()
  if (events.length > 0) {
    for (const eventGroup of events) {
      for (const event of eventGroup) {
        if (event.ev.method === 'WithdrawalRequestCreated') {
          try {
            const existingWithdrawal = await withdrawalRepository
              .search()
              .where('requestId')
              .equals(
                Number(String((event.ev.data as any).requestId.id).replace(/,/g, ''))
              )
              .and('chain')
              .equals((event.ev.data as any).chain)
              .and('txHash')
              .equals((event.ev.data as any).hash_)
              .returnFirst()
            if (existingWithdrawal) {
              logger.info(
                'Existing withdrawal found, skipping the WithdrawalRequestCreated data: ',
                existingWithdrawal
              )
              continue
            }
            const withdrawalData = await startTracingWithdrawal(
              api,
              event.ev.data,
              event.phaseApplyExtrinsic,
              block
            )
            logger.info('Tracing started for withdrawal', withdrawalData)
          } catch (error) {
            logger.error('Error tracing withdrawal:', error)
          }
        } else if (event.ev.method === 'TxBatchCreated') {
          await updateWithdrawalsWhenBatchCreated(api, event.ev.data)
        }
      }
    }
  }
}

export const startTracingWithdrawal = async (
  api: ApiPromise,
  eventData: any,
  phaseApplyExtrinsic: number,
  block: Block
): Promise<object> => {
  const timestamp = new Date().toISOString()
  const calldata = await api.rpc.rolldown.get_abi_encoded_l2_request(
    eventData.chain,
    Number(String(eventData.requestId.id).replace(/,/g, ''))
  )
  const affirmedNetworks = await redis.client.get(NETWORK_LIST_KEY)
  const networks = affirmedNetworks ? JSON.parse(affirmedNetworks) : []
  const network = networks.find((net: Network) => net.key === eventData.chain)
  const chainId = network ? network.chainId : 'unknown'

  const { extrinsicHash, address } =
    await extractExtrinsicHashAndAnAddressFromBlock(
      api,
      phaseApplyExtrinsic,
      block
    )
  const redisKey = `withdrawal:${extrinsicHash}`
  const keyExists = await redis.client.exists(redisKey)
  console.log('Key Exists:', keyExists)

  const withdrawalData = {
    requestId: Number(String(eventData.requestId.id).replace(/,/g, '')),
    txHash: eventData.hash_,
    address: address,
    recipient: eventData.recipient,
    created: Date.parse(timestamp),
    updated: Date.parse(timestamp),
    status: WITHDRAWAL_PENDING_ON_L2,
    type: type,
    chain: eventData.chain,
    amount: String(eventData.amount).replace(/,/g, ''),
    asset_chainId: chainId,
    asset_address: eventData.tokenAddress,
    proof: '',
    calldata: calldata.toHex(),
    createdBy: keyExists ? CreatedBy.Frontend : CreatedBy.Other,
    closedBy: null,
  }
  return withdrawalRepository.save(withdrawalData)
}

export const updateWithdrawalsWhenBatchCreated = async (
  api: ApiPromise,
  eventData: any
): Promise<void> => {
  const updateTimestamp = new Date().toISOString()
  const firstElement = Number(String(eventData.range[0]).replace(/,/g, ''))
  const lastElement = Number(
    String(eventData.range[eventData.range.length - 1]).replace(/,/g, '')
  )
  const existingWithdrawal = await withdrawalRepository
    .search()
    .where('requestId')
    .lte(lastElement)
    .and('requestId')
    .gte(firstElement)
    .and('chain')
    .equals(eventData.chain)
    .and('status')
    .equals(WITHDRAWAL_PENDING_ON_L2)
    .returnAll()
  if (existingWithdrawal.length > 0) {
    for (const withdrawal of existingWithdrawal) {
      const chain = api.createType('Chain', withdrawal.chain)
      let proof = await api.rpc.rolldown.get_merkle_proof(
        chain,
        [firstElement, lastElement],
        withdrawal.requestId
      )
      let root = await api.rpc.rolldown.get_merkle_root(chain, [
        firstElement,
        lastElement,
      ])
      logger.info('Root:', root.toHuman())
      withdrawal.updated = Date.parse(updateTimestamp)
      withdrawal.status = WITHDRAWAL_BATCHED_FOR_L1
      withdrawal.proof = proof.toHex()
      await withdrawalRepository.save(withdrawal)
      logger.info('Withdrawal batch created and status updated', withdrawal)
    }
  }
}

const filterEvents = (ev: Event) => {
  return (
    ev.section === 'rolldown' &&
    (ev.method === 'WithdrawalRequestCreated' || ev.method === 'TxBatchCreated')
  )
}
