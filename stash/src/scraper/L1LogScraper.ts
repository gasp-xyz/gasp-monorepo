import { type PublicClientConfig, createPublicClient, http } from 'viem'
import RolldownContract from '../Rolldown.json' assert { type: 'json' }
import { transactionRepository } from '../repository/TransactionRepository.js'
import process from 'node:process'
import { ApiPromise } from '@polkadot/api'
import { redis } from '../connector/RedisConnector.js'
import { setTimeout } from 'timers/promises'
import logger from '../util/Logger.js'

export const L1_CONFIRMED_STATUS = 'L1_CONFIRMED'
export const L1_INITIATED_STATUS = 'L1_INITIATED'

const keepProcessing = true

export const watchDepositAcceptedIntoQueue = async (
  api: any,
  chainUrl: string,
  chain: any,
  chainName: string
) => {
  const publicClient = getPublicClient({
    transport: http(chainUrl),
    chain: chain,
  })

  while (keepProcessing) {
    try {
      const toBlock = await publicClient.getBlockNumber()
      let fromBlock = await getLastProcessedBlock(chainName, 'deposit')
      if (fromBlock === 0n) {
        fromBlock = toBlock
      }
      logger.info(
        `chainName: ${chainName}, fromBlock: ${fromBlock}, toBlock: ${toBlock}`
      )
      const logs = await publicClient.getContractEvents({
        address: `0x${process.env.CONTRACT_ADDRESS}` as `0x${string}`,
        abi: RolldownContract.abi,
        eventName: 'DepositAcceptedIntoQueue',
        fromBlock,
        toBlock,
      })
      for (const log of logs) {
        const { transactionHash, blockNumber } = log
        const existingTransaction = await transactionRepository
          .search()
          .where('txHash')
          .equals(transactionHash)
          .and('type')
          .equals('deposit')
          .and('status')
          .equals(L1_INITIATED_STATUS)
          .returnFirst()

        if (existingTransaction) {
          existingTransaction.status = L1_CONFIRMED_STATUS
          existingTransaction.requestId = Number((log as any).args.requestId)
          const timestamp = new Date().toISOString()
          existingTransaction.updated = Date.parse(timestamp)
          await transactionRepository.save(existingTransaction)
          logger.info('Transaction status updated:', existingTransaction)
        }
        await saveLastProcessedBlock(chainName, blockNumber, 'deposit') //saving the last processed block
      }
      await saveLastProcessedBlock(chainName, toBlock, 'deposit') //if in the range of fromBlock and toBlock we didn't find any event, we save the last block
    } catch (error) {
      logger.error('Error in watchDepositAcceptedIntoQueue loop:', error)
    }
    await setTimeout(5000)
  }
}

export const processRequests = async (api: ApiPromise, l1Chain: string) => {
  while (keepProcessing) {
    try {
      const lastProcessedRequestId = Number.parseInt(
        (await api.query.rolldown.lastProcessedRequestOnL2(l1Chain)).toString()
      )
      const lastSavedProcessedRequestId = await getLastProcessedRequestId(
        l1Chain,
        'deposit'
      )
      const transactionsToProcess = await transactionRepository
        .search()
        .where('chain')
        .equals(l1Chain)
        .and('type')
        .equals('deposit')
        .and('requestId')
        .gte(lastSavedProcessedRequestId)
        .and('requestId')
        .lte(lastProcessedRequestId)
        .return.all()

      for (const transaction of transactionsToProcess) {
        transaction.status = 'PROCESSED'
        const timestamp = new Date().toISOString()
        transaction.updated = Date.parse(timestamp)
        await transactionRepository.save(transaction)
      }
      await saveLastProcessedRequestId(
        //even if we don't have any transaction to process, we save the last processed request id
        l1Chain,
        lastProcessedRequestId,
        'deposit'
      )
    } catch (error) {
      logger.error('Error processing requests:', error)
    }

    // Delay to avoid overwhelming the system
    await setTimeout(5000)
  }
}

export const getPublicClient = (options: PublicClientConfig) => {
  return createPublicClient({ ...options })
}
const saveLastProcessedRequestId = async (
  l1Chain: string,
  lastProcessedRequestId: number,
  type: string
) => {
  await redis.client.hset(
    `tx:${type}:${l1Chain}`,
    'lastRequestId',
    lastProcessedRequestId.toString()
  )
  logger.info(
    `Last processed requestId ${lastProcessedRequestId} for chain ${l1Chain} saved`
  )
}

const getLastProcessedRequestId = async (
  l1Chain: string,
  type: string
): Promise<number | null> => {
  const result = await redis.client.hget(
    `tx:${type}:${l1Chain}`,
    'lastRequestId'
  )
  return result ? Number(result) : 0
}

const saveLastProcessedBlock = async (
  l1Chain: string,
  lastProcessedBlock: bigint,
  type: string
) => {
  await redis.client.hset(
    `tx:${type}:${l1Chain}`,
    'lastBlock',
    lastProcessedBlock.toString()
  )
  logger.info(
    `Last processed block ${lastProcessedBlock} saved for chain ${l1Chain}`
  )
}

const getLastProcessedBlock = async (
  l1Chain: string,
  type: string
): Promise<bigint | null> => {
  const result = await redis.client.hget(`tx:${type}:${l1Chain}`, 'lastBlock')
  return result ? BigInt(result) : 0n
}
