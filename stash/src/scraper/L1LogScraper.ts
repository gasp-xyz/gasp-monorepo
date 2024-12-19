import { type PublicClientConfig, createPublicClient, http } from 'viem'
import RolldownContract from '../Rolldown.json' assert { type: 'json' }
import {
  depositRepository,
  withdrawalRepository,
} from '../repository/TransactionRepository.js'
import process from 'node:process'
import { ApiPromise } from '@polkadot/api'
import { timeseries } from '../connector/RedisConnector.js'
import { setTimeout } from 'timers/promises'
import logger from '../util/Logger.js'

export const DEPOSIT_SUBMITTED_TO_L2 = 'SubmittedToL2'
export const DEPOSIT_PENDING_ON_L1 = 'PendingOnL1'
export const WITHDRAWAL_BATCHED_FOR_L1 = 'BatchedForL1'
export const PROCESSED_STATUS = 'Processed'

const keepProcessing = true

export const watchDepositAcceptedIntoQueue = async (
  api: any,
  chainUrl: string,
  chain: any,
  chainName: string,
  contractAddress: string
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
      logger.info({
        message: `Deposit: chainName: ${chainName}, fromBlock: ${fromBlock}, toBlock: ${toBlock}`,
      })
      const logs = await publicClient.getContractEvents({
        address: `0x${contractAddress}` as `0x${string}`,
        abi: RolldownContract.abi,
        eventName: 'DepositAcceptedIntoQueue',
        fromBlock,
        toBlock,
      })

      for (const log of logs) {
        logger.info({
          message: 'Processing deposit log:',
          log: log,
        })
        const { transactionHash, blockNumber } = log
        const existingTransaction = await depositRepository
          .search()
          .where('txHash')
          .equals(transactionHash)
          .and('type')
          .equals('deposit')
          .and('status')
          .equals(DEPOSIT_PENDING_ON_L1)
          .returnFirst()

        if (existingTransaction) {
          existingTransaction.status = DEPOSIT_SUBMITTED_TO_L2
          existingTransaction.requestId = Number((log as any).args.requestId)
          const timestamp = new Date().toISOString()
          existingTransaction.updated = Date.parse(timestamp)
          await depositRepository.save(existingTransaction)
          logger.info({
            message: 'Deposit status updated:',
            transaction: existingTransaction,
          })
        }
        await saveLastProcessedBlock(chainName, blockNumber, 'deposit') //saving the last processed block
      }
      await saveLastProcessedBlock(chainName, toBlock, 'deposit') //even if in the range of fromBlock and toBlock we didn't find any event, we save the last block
    } catch (error) {
      logger.error({
        message: 'Error in watchDepositAcceptedIntoQueue loop:',
        error: error,
      })
    }
    await setTimeout(5000)
  }
}

export const watchWithdrawalClosed = async (
  api: any,
  chainUrl: string,
  chain: any,
  chainName: string,
  contractAddress: string
) => {
  const publicClient = getPublicClient({
    transport: http(chainUrl),
    chain: chain,
  })

  while (keepProcessing) {
    try {
      const toBlock = await publicClient.getBlockNumber()
      let fromBlock = await getLastProcessedBlock(chainName, 'withdrawal')
      if (fromBlock === 0n) {
        fromBlock = toBlock
      }
      logger.info({
        message: ` Withdrawal: chainName: ${chainName}, fromBlock: ${fromBlock}, toBlock: ${toBlock}`,
      })
      const eventsFerried = await publicClient.getContractEvents({
        address: `0x${contractAddress}` as `0x${string}`,
        abi: RolldownContract.abi,
        eventName: 'FerriedWithdrawalClosed',
        fromBlock,
        toBlock,
      })
      const eventsNotFerried = await publicClient.getContractEvents({
        address: `0x${contractAddress}` as `0x${string}`,
        abi: RolldownContract.abi,
        eventName: 'WithdrawalClosed',
        fromBlock,
        toBlock,
      })
      const combinedEvents = [...eventsFerried, ...eventsNotFerried]
      for (const event of combinedEvents) {
        logger.info({
          message: 'Processing withdrawal event:',
          event: event,
        })
        const {
          blockNumber,
          eventName,
          args: { requestId, withdrawalHash },
        } = event as any
        const existingTransaction = await withdrawalRepository
          .search()
          .where('requestId')
          .equals(requestId)
          .and('txHash')
          .equals(withdrawalHash)
          .and('type')
          .equals('withdrawal')
          .and('status')
          .equals(WITHDRAWAL_BATCHED_FOR_L1)
          .and('chain')
          .equals(chainName)
          .returnFirst()
        logger.info(
          'Existing withdrawal found to be updated:',
          existingTransaction
        )
        if (existingTransaction) {
          existingTransaction.status = PROCESSED_STATUS
          existingTransaction.closedBy =
            eventName === 'FerriedWithdrawalClosed' ? 'ferry' : 'regular'
          const timestamp = new Date().toISOString()
          existingTransaction.updated = Date.parse(timestamp)
          await withdrawalRepository.save(existingTransaction)
          logger.info({
            message: 'Withdrawal status updated:',
            transaction: existingTransaction,
          })
        }
        await saveLastProcessedBlock(chainName, blockNumber, 'withdrawal')
      }
      await saveLastProcessedBlock(chainName, toBlock, 'withdrawal') //even if in the range of fromBlock and toBlock we didn't find any event, we save the last block
    } catch (error) {
      logger.error({
        message: 'Error in withdrawal closure loop:',
        error: error,
      })
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
      const transactionsToProcess = await depositRepository
        .search()
        .where('chain')
        .equals(l1Chain)
        .and('type')
        .equals('deposit')
        .and('requestId')
        .gte(lastSavedProcessedRequestId)
        .and('requestId')
        .lte(lastProcessedRequestId)
        .and('status')
        .not.equals('Processed') //we want to skip ferried, already processed
        .return.all()

      for (const transaction of transactionsToProcess) {
        transaction.status = 'Processed'
        const timestamp = new Date().toISOString()
        transaction.updated = Date.parse(timestamp)
        transaction.closedBy = 'regular'
        await depositRepository.save(transaction)
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
  await timeseries.client.hset(
    `transactions_scanned:${type}:${l1Chain}`,
    'lastRequestId',
    lastProcessedRequestId.toString()
  )
  logger.info(
    `${type} : Last processed requestId ${lastProcessedRequestId} chain ${l1Chain} saved`
  )
}

const getLastProcessedRequestId = async (
  l1Chain: string,
  type: string
): Promise<number | null> => {
  const result = await timeseries.client.hget(
    `transactions_scanned:${type}:${l1Chain}`,
    'lastRequestId'
  )
  return result ? Number(result) : 0
}

const saveLastProcessedBlock = async (
  l1Chain: string,
  lastProcessedBlock: bigint,
  type: string
) => {
  await timeseries.client.hset(
    `transactions_scanned:${type}:${l1Chain}`,
    'lastBlock',
    lastProcessedBlock.toString()
  )
  logger.info(
    `${type} : Last processed block ${lastProcessedBlock} saved for chain ${l1Chain}`
  )
}

const getLastProcessedBlock = async (
  l1Chain: string,
  type: string
): Promise<bigint | null> => {
  const result = await timeseries.client.hget(
    `transactions_scanned:${type}:${l1Chain}`,
    'lastBlock'
  )
  return result ? BigInt(result) : 0n
}
