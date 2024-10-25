import { Client } from 'redis-om'
import { depositSchema } from '../model/Deposit.js'
import { withdrawalSchema } from '../model/Withdrawal.js'
import { getTimeseriesUrl } from '../connector/RedisConnector.js'
import logger from '../util/Logger.js'

const client = new Client()
await client.open(getTimeseriesUrl())

const depositRepository = client.fetchRepository(depositSchema)
const withdrawalRepository = client.fetchRepository(withdrawalSchema)

try {
  await depositRepository.createIndex()
} catch (error) {
  if (!error.message.includes('Deposit index already exists')) {
    throw error
  }
  logger.log({
    level: 'info',
    message: 'Deposit index already exists, skipping creation.',
  })
}
try {
  await withdrawalRepository.createIndex()
} catch (error) {
  if (!error.message.includes('Withdrawal index already exists')) {
    throw error
  }
  logger.log({
    level: 'info',
    message: 'Withdrawal index already exists, skipping creation.',
  })
}

export { depositRepository, withdrawalRepository }
