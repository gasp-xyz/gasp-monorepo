import { GenericContainer, StartedTestContainer, Wait } from 'testcontainers'
import * as crypto from 'crypto'

export const TIMESERIES_HOST_DOCKER_IMAGE_NAME =
  'mangatasolutions/redis-test-stash_ts:5'
export const REDIS_HOST_DOCKER_IMAGE_NAME = 'p1k1m4n/redis-test-stash:5'
export const MAX_DAYS = 'max'
export const MAX_INTERVAL = 'day'
export const tokenIDs = ['0', '1', '2', '3', '4', '5', '7', '15', '19']

let redisContainer: StartedTestContainer
let timeSeriesContainer: StartedTestContainer

export async function startContainer(image: string) {
  console.warn('Starting container: ' + image)
  if (image === TIMESERIES_HOST_DOCKER_IMAGE_NAME) {
    return await new GenericContainer(image)
      .withWorkingDir('/')
      .withEntrypoint(['./entrypoint.sh'])
      .withExposedPorts({ container: 6379, host: 6379 })
      .withWaitStrategy(Wait.forLogMessage('Ready to accept connections'))
      .start()
  } else {
    return await new GenericContainer(image)
      .withWorkingDir('/')
      .withEntrypoint(['redis-server', '--protected-mode no'])
      .withExposedPorts({ container: 6379, host: 6380 })
      .withName('redis-container')
      .withWaitStrategy(Wait.forLogMessage('Ready to accept connections'))
      .start()
  }
}
export async function tearDownBothContainers() {
  console.warn('Tearing down containers')
  if (redisContainer) {
    await redisContainer.stop()
  }
  if (timeSeriesContainer) {
    await timeSeriesContainer.stop()
  }
}
export async function initBothContainers() {
  try {
    redisContainer = await startContainer(REDIS_HOST_DOCKER_IMAGE_NAME)
    timeSeriesContainer = await startContainer(
      TIMESERIES_HOST_DOCKER_IMAGE_NAME
    )
  } catch (e) {
    console.error(e)
    throw e
  }
}

export function generateRandomHash(): string {
  const randomBytes = crypto.randomBytes(32)
  const hash =
    '0x' + crypto.createHash('sha256').update(randomBytes).digest('hex')

  return hash
}

export function generateRandomAddress(): string {
  const randomBytes = crypto.randomBytes(20)
  const address = '0x' + randomBytes.toString('hex')

  return address
}
