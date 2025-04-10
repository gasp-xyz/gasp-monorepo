import IORedis, { Redis } from 'ioredis'
import Redlock from 'redlock'

export class RedisClient {
  client: Redis
  lock: Redlock
  noRetryLock: Redlock
  name: string

  constructor(name: string, host: string, port: number, pass: string = '') {
    this.name = name
    console.log(
      `[${name}] Connecting to redis at ${host}:${port} ${
        pass ? 'with password' : 'without password'
      }`,
    )

    this.client = new IORedis({ host, port, password: pass })

    this.client.on('connect', () => console.log(`[${name}] Connected to Redis`))
    this.client.on('error', (err) =>
      console.error(`[${name}] Redis connection error:`, err),
    )
    this.client.on('ready', () => console.log(`[${name}] Redis client ready`))

    this.lock = new Redlock([this.client], {
      driftFactor: 0.01,
      retryCount: 10,
      retryDelay: 200, // time in ms
      retryJitter: 200, // time in ms
      automaticExtensionThreshold: 500, // time in ms
    })
    this.noRetryLock = new Redlock([this.client], {
      driftFactor: 0.01,
      retryCount: 0,
    })
  }
}

export const redis = new RedisClient(
  'main',
  process.env.REDIS_HOST,
  parseInt(process.env.REDIS_PORT),
  process.env.REDIS_PASS,
)

export function getRedisUrl(): string {
  const host = process.env.REDIS_HOST
  const port = parseInt(process.env.REDIS_PORT)
  const pass = process.env.REDIS_PASS
  return pass ? `redis://:${pass}@${host}:${port}` : `redis://${host}:${port}`
}
