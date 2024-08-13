import { Client } from 'redis-om'
import { transactionSchema } from '../model/Transaction.js'
import { getRedisUrl } from '../connector/RedisConnector.js'

const client = new Client()
await client.open(getRedisUrl())

const transactionRepository = client.fetchRepository(transactionSchema)

try {
  await transactionRepository.createIndex()
} catch (error) {
  if (error.message.includes('Index already exists')) {
    console.log('Index already exists, skipping creation.')
  } else {
    throw error
  }
}

export { transactionRepository }
