import { Client } from 'redis-om'
import { transactionSchema } from '../model/Transaction.js'
import { withdrawalSchema } from '../model/Withdrawal.js'
import { getTimeseriesUrl } from '../connector/RedisConnector.js'
import logger from '../util/Logger.js'

const client = new Client()
await client.open(getTimeseriesUrl())

const transactionRepository = client.fetchRepository(transactionSchema)
const withdrawalRepository = client.fetchRepository(withdrawalSchema)

try {
  await transactionRepository.createIndex()
} catch (error) {
  if (error.message.includes('Deposit index already exists')) {
    logger.log({
      level: 'info',
      message: 'Deposit index already exists, skipping creation.',
    })
  } else {
    throw error
  }
}
try {
  await withdrawalRepository.createIndex()
} catch (error) {
  if (error.message.includes('Withdrawal index already exists')) {
    logger.log({
      level: 'info',
      message: 'Withdrawal index already exists, skipping creation.',
    })
  } else {
    throw error
  }
}

export { transactionRepository, withdrawalRepository }
