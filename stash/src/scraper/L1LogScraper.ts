import { type PublicClientConfig, createPublicClient, http } from 'viem'
import Rolldown from '../Rolldown.json' assert { type: 'json' }
import { transactionRepository } from '../repository/TransactionRepository.js'
import process from 'node:process'
import { ApiPromise } from '@polkadot/api'
import { redis } from '../connector/RedisConnector.js'

export const L1_CONFIRMED_STATUS = 'L1_CONFIRMED'

export const watchDepositAcceptedIntoQueue = async (
  api: any,
  chainUrl: string,
  chain: any
) => {
  console.log('Received DepositAcceptedIntoQueue event')
  console.log('scanning events:, chainUrl:', chainUrl, chain)

  const publicClient = getPublicClient({
    transport: http(chainUrl),
    chain: chain,
  })

  const fromBlock = await getLastProcessedRequestId(chain, 'deposit')
  const toBlock = await api.rpc.chain
    .getHeader()
    .then((header) => header.number.toNumber())

  const unwatch = publicClient.getContractEvents({
    address: `0x${process.env.CONTRACT_ADDRESS}` as `0x${string}`,
    abi: Rolldown.abi,
    eventName: 'DepositAcceptedIntoQueue',
    fromBlock,
    toBlock,
    onLogs: async (logs) => {
      console.log('Received DepositAcceptedIntoQueue event')
      for (const log of logs) {
        const { transactionHash } = log
        const existingTransaction = await transactionRepository
          .search()
          .where('txHash')
          .equals(transactionHash)
          .and('type')
          .equals('deposit')
          .returnFirst() //todo: we should have only one transaction with the same hash

        if (existingTransaction) {
          existingTransaction.status = L1_CONFIRMED_STATUS
          existingTransaction.requestId = Number((log as any).args.requestId)
          const timestamp = new Date().toISOString()
          existingTransaction.updated = Date.parse(timestamp)
          await transactionRepository.save(existingTransaction)
          console.log('Transaction status updated:', existingTransaction)
          return
        }
        // console.log('Accepted log object:', log)
      }
    },
  })

  // Handle process termination to unwatch events
  process.on('SIGINT', async () => {
    console.error('Stopping the process..., SIGINT signal')
    unwatch() // Unsubscribe from event watching
    process.exit()
  })

  process.on('SIGTERM', async () => {
    console.error('Stopping the process..., SIGTERM signal')
    unwatch() // Unsubscribe from event watching
    process.exit()
  })

  process.on('SIGHUP', async () => {
    console.error('Stopping the process..., SIGHUP signal')
    unwatch() // Unsubscribe from event watching
    process.exit()
  })
}

export const getPublicClient = (options: PublicClientConfig) => {
  return createPublicClient({ ...options })
}

//Query last processed request on L2 for deposits

let keepProcessing = true

export const stopProcessingRequests = () => {
  keepProcessing = false
}

export const processRequests = async (api: ApiPromise, l1Chain: string) => {
  const unwatch = () => {
    // Add logic to unwatch events if necessary
  }

  // Handle process termination to unwatch events
  process.on('SIGINT', async () => {
    console.error('Stopping the L2 query process..., SIGINT signal')
    stopProcessingRequests()
    unwatch() // Unsubscribe from event watching
    process.exit()
  })

  process.on('SIGTERM', async () => {
    console.error('Stopping the L2 query process..., SIGTERM signal')
    stopProcessingRequests()
    unwatch() // Unsubscribe from event watching
    process.exit()
  })

  process.on('SIGHUP', async () => {
    console.error('Stopping the L2 query process..., SIGHUP signal')
    stopProcessingRequests()
    unwatch() // Unsubscribe from event watching
    process.exit()
  })

  while (keepProcessing) {
    //info: this stays the same
    try {
      const lastProcessedRequestId = Number.parseInt(
        (await api.query.rolldown.lastProcessedRequestOnL2(l1Chain)).toString()
      )
      const lastSavedProcessedRequestId = await getLastProcessedRequestId(
        //we separate the last processed request id by type (deposit, withdrawal) and l1 chain
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
        // Save the lastProcessedRequestId in the database
      }
      await saveLastProcessedRequestId(
        l1Chain,
        lastProcessedRequestId,
        'deposit'
      ) //we are saving the last processed request id unrelated if we had transactions to update

      // console.log(
      //   `Processed ${transactionsToProcess.length} requests from ID ${lastSavedProcessedRequestId} to ID ${lastProcessedRequestId} on chain ${l1Chain}`
      // )
    } catch (error) {
      console.error('Error processing requests:', error)
    }

    // Delay to avoid overwhelming the system
    await new Promise((resolve) => setTimeout(resolve, 5000))
  }
}
const saveLastProcessedRequestId = async (
  l1Chain: string,
  lastProcessedRequestId: number,
  type: string
) => {
  await redis.client.set(
    `transaction:${type}:${l1Chain}:latest`,
    lastProcessedRequestId.toString()
  )
}

const getLastProcessedRequestId = async (
  l1Chain: string,
  type: string
): Promise<number | null> => {
  const result = await redis.client.get(`transaction:${type}:${l1Chain}:latest`)
  return result ? Number(result) : 0 // Return 0 and start updating statuses from the block 0
}
