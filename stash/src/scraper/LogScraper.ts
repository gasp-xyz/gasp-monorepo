import { type PublicClientConfig, createPublicClient, http } from 'viem'
import Rolldown from '../Rolldown.json' assert { type: 'json' }
import { transactionRepository } from '../repository/TransactionRepository.js'
import process from 'node:process'
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

  const unwatch = publicClient.watchContractEvent({
    address: `0x${process.env.CONTRACT_ADDRESS}` as `0x${string}`,
    abi: Rolldown.abi,
    eventName: 'DepositAcceptedIntoQueue',
    onLogs: async (logs) => {
      console.log('Received DepositAcceptedIntoQueue event')
      for (const log of logs) {
        const { transactionHash } = log
        const existingTransaction = await transactionRepository
          .search()
          .where('txHash')
          .equals(transactionHash)
          .returnFirst() //todo: we should have only one transaction with the same hash
        if (existingTransaction) {
          existingTransaction.status = L1_CONFIRMED_STATUS
          existingTransaction.requestId = (log as any).args.requestId.toString()
          await transactionRepository.save(existingTransaction)
          console.log('Transaction status updated:', existingTransaction)
          return
        }
        console.log('Accepted log object:', log)
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
