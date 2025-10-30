import { redis } from '../connector/RedisConnector.js'
import logger from '../util/Logger.js'

interface LastCloseableResult {
  network: string
  lastRequestId: number
}

const NETWORKS = ['Ethereum', 'Arbitrum', 'Base', 'Sonic']

const getLastProcessedRequestId = async (
  l1Chain: string,
  type: string,
): Promise<number> => {
  const result = await redis.client.hget(
    `transactions_scanned:${type}:${l1Chain}`,
    'lastRequestId',
  )
  return result ? Number(result) : 0
}

export const getLastCloseableRequestIds = async (): Promise<LastCloseableResult[]> => {
  try {
    const results: LastCloseableResult[] = []
    
    for (const network of NETWORKS) {
      const lastRequestId = await getLastProcessedRequestId(network, 'l2update')
      results.push({
        network,
        lastRequestId,
      })
    }
    
    logger.info('Retrieved last closeable request IDs for all networks', results)
    return results
  } catch (error) {
    logger.error('Error retrieving last closeable request IDs:', error)
    throw error
  }
}