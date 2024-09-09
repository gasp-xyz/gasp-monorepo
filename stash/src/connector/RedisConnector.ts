import IORedis, { Redis } from 'ioredis'
import Redlock from 'redlock'

export class RedisClient {
  client: Redis
  lock: Redlock
  noRetryLock: Redlock

  constructor(host: string, port: number, pass: string = '') {
    this.client = new IORedis({ host, port, password: pass })
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
  process.env.REDIS_HOST,
  parseInt(process.env.REDIS_PORT),
  process.env.REDIS_PASS
)
export const timeseries = new RedisClient(
  process.env.TIMESERIES_HOST,
  parseInt(process.env.TIMESERIES_PORT),
  process.env.TIMESERIES_PASS
)

export function getTimeseriesUrl(): string {
  const host = process.env.TIMESERIES_HOST
  const port = parseInt(process.env.TIMESERIES_PORT)
  const pass = process.env.TIMESERIES_PASS
  console.log('')
  return `redis://:${pass}@${host}:${port}`
}
