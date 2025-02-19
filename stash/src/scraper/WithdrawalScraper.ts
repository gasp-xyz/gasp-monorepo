import { Block, Event } from './BlockScraper'
import _ from 'lodash'
import { withdrawalRepository } from '../repository/TransactionRepository.js'
import { ApiPromise } from '@polkadot/api'
import { redis } from '../connector/RedisConnector.js'
import logger from '../util/Logger.js'

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

export const processWithdrawalEvents = async (
  api: ApiPromise,
  block: Block
) => {
  const events = _.chain(block.events)
    .filter((ev) => filterEvents(ev[1]))
    .groupBy(([idx, _]) => idx)
    .map((evs, _) => evs.map(([_, ev]) => ev))
    .value()
  if (events.length > 0) {
    for (const eventGroup of events) {
      for (const event of eventGroup) {
        if (event.method === 'WithdrawalRequestCreated') {
          const existingWithdrawal = await withdrawalRepository
            .search()
            .where('txHash')
            .equals((event.data as any).hash_)
            .and('type')
            .equals('withdrawal')
            .returnFirst()

          if (existingWithdrawal) {
            await updateWithdrawal(api, existingWithdrawal, event.data)
          } else {
            const withdrawalData = await startTracingWithdrawal(api, event.data)
            logger.info('Tracing started for withdrawal', withdrawalData)
          }
        } else if (event.method === 'TxBatchCreated') {
          await updateWithdrawalsWhenBatchCreated(api, event.data)
        }
      }
    }
  }
}

export const updateWithdrawal = async (
  api: ApiPromise,
  existingWithdrawal: any,
  eventData: any
) => {
  existingWithdrawal.requestId = Number(
    eventData.requestId.id.replace(/,/g, '')
  )
  existingWithdrawal.updated = Date.parse(new Date().toISOString())
  existingWithdrawal.status = WITHDRAWAL_PENDING_ON_L2
  existingWithdrawal.proof = ''
  const calldata = await api.rpc.rolldown.get_abi_encoded_l2_request(
    eventData.chain,
    eventData.requestId.id.replace(/,/g, '')
  )
  existingWithdrawal.calldata = calldata.toHex()
  existingWithdrawal.closedBy = null
  await withdrawalRepository.save(existingWithdrawal)
  logger.info(
    'Existing withdrawal updated with event WithdrawalRequestCreated',
    existingWithdrawal
  )
}

export const startTracingWithdrawal = async (
  api: ApiPromise,
  eventData: any
): Promise<object> => {
  const timestamp = new Date().toISOString()
  const calldata = await api.rpc.rolldown.get_abi_encoded_l2_request(
    eventData.chain,
    eventData.requestId.id.replace(/,/g, '')
  )
  const affirmedNetworks = await redis.client.get(NETWORK_LIST_KEY)
  const networks = affirmedNetworks ? JSON.parse(affirmedNetworks) : []
  const network = networks.find((net: Network) => net.key === eventData.chain)
  const chainId = network ? network.chainId : 'unknown'

  const withdrawalData = {
    requestId: Number(eventData.requestId.id.replace(/,/g, '')),
    txHash: eventData.hash_,
    address: eventData.recipient,
    created: Date.parse(timestamp),
    updated: Date.parse(timestamp),
    status: WITHDRAWAL_PENDING_ON_L2,
    type: type,
    chain: eventData.chain,
    amount: eventData.amount.replace(/,/g, ''),
    asset_chainId: chainId,
    asset_address: eventData.tokenAddress,
    proof: '',
    calldata: calldata.toHex(),
    createdBy: CreatedBy.Other,
    closedBy: null,
  }
  return withdrawalRepository.save(withdrawalData)
}

export const updateWithdrawalsWhenBatchCreated = async (
  api: ApiPromise,
  eventData: any
): Promise<void> => {
  const updateTimestamp = new Date().toISOString()
  const firstElement = Number(eventData.range[0].replace(/,/g, ''))
  const lastElement = Number(
    eventData.range[eventData.range.length - 1].replace(/,/g, '')
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
