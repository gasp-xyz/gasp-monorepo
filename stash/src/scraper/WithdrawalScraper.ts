import { Block, Event } from './BlockScraper'
import _ from 'lodash'
import { withdrawalRepository } from '../repository/TransactionRepository.js'
import { ApiPromise } from '@polkadot/api'
import { keccak256 } from 'viem'
import { redis } from '../connector/RedisConnector.js'

const NETWORK_LIST_KEY = 'affirmed_networks_list'
const L2_INITIATED = 'L2_INITIATED'
const L2_CONFIRMED = 'L2_CONFIRMED'
const type = 'withdrawal'

interface Network {
  key: string
  chainId: string
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
        console.log('Method:', event.method)
        console.log('Section:', event.section)
        console.log('Index:', event.index)
        console.log('Data:', event.data)
        if (event.method === 'WithdrawalRequestCreated') {
          console.log('Event accepted at:', new Date().toISOString())
          const withdrawalData = await startTracingWithdrawal(api, event.data)
          console.log('Tracing started for withdrawal', withdrawalData)
        } else if (event.method === 'TxBatchCreated') {
          await updateWithdrawalsWhenBatchCreated(api, event.data)
        }
      }
    }
  }
}

const startTracingWithdrawal = async (
  api: ApiPromise,
  eventData: any
): Promise<object> => {
  const timestamp = new Date().toISOString()
  const calldata = await api.rpc.rolldown.get_abi_encoded_l2_request(
    eventData.chain,
    eventData.requestId.id
  )
  console.log(
    'Withdrawal hash and keccak256 match:',
    keccak256(calldata.toHex()) === eventData.hash_,
    'Calculated keccak:',
    keccak256(calldata.toHex()),
    'Event hash:',
    eventData.hash_
  )
  // Fetch affirmed networks from Redis
  const affirmedNetworks = await redis.client.get(NETWORK_LIST_KEY)
  const networks = affirmedNetworks ? JSON.parse(affirmedNetworks) : []

  // Find the network with the matching key
  const network = networks.find((net: Network) => net.key === eventData.chain)

  // Check the chainId of the found network
  const chainId = network ? network.chainId : 'unknown'
  const withdrawalData = {
    requestId: Number(eventData.requestId.id),
    txHash: eventData.hash_,
    address: eventData.recipient,
    created: Date.parse(timestamp),
    updated: Date.parse(timestamp),
    status: L2_INITIATED,
    type: type,
    chain: eventData.chain,
    amount: eventData.amount,
    asset_chainId: chainId,
    asset_address: eventData.tokenAddress,
    proof: '',
    calldata: calldata.toHex(),
  }
  return withdrawalRepository.save(withdrawalData)
}

const updateWithdrawalsWhenBatchCreated = async (
  api: ApiPromise,
  eventData: any
): Promise<void> => {
  const updateTimestamp = new Date().toISOString()
  const firstElement = Number(eventData.range[0])
  const lastElement = Number(eventData.range[eventData.range.length - 1])
  const existingWithdrawal = await withdrawalRepository
    .search()
    .where('requestId')
    .lte(lastElement)
    .and('requestId')
    .gte(firstElement)
    .and('chain')
    .equals(eventData.chain)
    .and('status')
    .equals(L2_INITIATED)
    .returnAll()
  if (existingWithdrawal.length > 0) {
    for (const withdrawal of existingWithdrawal) {
      const chain = api.createType('Chain', withdrawal.chain)
      let proof = await api.rpc.rolldown.get_merkle_proof(
        chain,
        [firstElement, lastElement],
        withdrawal.requestId
      )
      console.log('Proof:', proof.toHuman())
      let root = await api.rpc.rolldown.get_merkle_root(chain, [
        firstElement,
        lastElement,
      ])
      console.log('Root:', root.toHuman())
      withdrawal.updated = Date.parse(updateTimestamp)
      withdrawal.status = L2_CONFIRMED
      withdrawal.proof = proof.toHex()
      await withdrawalRepository.save(withdrawal)
      console.log('Withdrawal batch created and status updated', withdrawal)
    }
  }
}

const filterEvents = (ev: Event) => {
  return ev.section === 'rolldown' &&
      (ev.method === 'WithdrawalRequestCreated' || ev.method === 'TxBatchCreated')
}
