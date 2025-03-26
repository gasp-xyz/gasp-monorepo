import { redis } from '../connector/RedisConnector.js'

export const store = async (
  key: string,
  value: string
): Promise<{ value: string; timestamp: number }> => {
  const now = new Date().toISOString()
  const parsed = Date.parse(now)
  await redis.client.hset(`${key}:${value}`, 'timestamp', parsed)
  return { value: `${key}:${value}`, timestamp: parsed }
}
