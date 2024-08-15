import { type PublicClientConfig, createPublicClient, webSocket } from 'viem'
import Rolldown from '../Rolldown.json' assert { type: 'json' }
import { transactionRepository } from '../repository/TransactionRepository.js'
import logger from '../util/Logger.js'
export const L1_CONFIRMED_STATUS = 'L1_CONFIRMED'
const contractAddress = '0x${process.env.CONTRACT_ADDRESS}'

export const watchDepositAcceptedIntoQueue = async (
  api: any,
  chainUrl: string,
  chain: any
) => {
  const webSocketTransport = webSocket(chainUrl, { retryCount: 5 })

  const publicClient = getPublicClient({
    transport: webSocketTransport,
    chain: chain,
  })

  const unwatch = publicClient.watchContractEvent({
    address: contractAddress,
    abi: Rolldown.abi,
    eventName: 'DepositAcceptedIntoQueue',
    onLogs: async (logs: any) => {
      logger.info('Received DepositAcceptedIntoQueue event')
      for (const log of logs) {
        const { transactionHash } = log
        logger.info('transactionHash is:', transactionHash)
        const existingTransaction = await transactionRepository
          .search()
          .where('txHash')
          .equals(transactionHash)
          .returnFirst() //todo: we should have only one transaction with the same hash
        if (existingTransaction.length > 0) {
          existingTransaction.status = L1_CONFIRMED_STATUS
          await transactionRepository.save(existingTransaction)
          logger.info('Transaction status updated:', existingTransaction)
          return
        }
        logger.info('Accepted log object:', log)
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
