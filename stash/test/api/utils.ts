import * as crypto from 'crypto'
import { GenericContainer, StartedTestContainer, Wait } from 'testcontainers'

export const REDIS_HOST_DOCKER_IMAGE_NAME = 'gaspxyz/redis-stash-test-data:stable'
export const MAX_DAYS = 'max'
export const MAX_INTERVAL = 'day'
export const tokenIDs = ['0', '1', '2', '3', '4', '5', '7', '15', '19']

let redisContainer: StartedTestContainer

export async function startContainer(image: string) {
  console.warn('Starting container: ' + image)
  return await new GenericContainer(image)
      .withExposedPorts({ container: 6379, host: 6380 })
      .withName('redis-container')
      .withWaitStrategy(Wait.forLogMessage('Ready to accept connections'))
      .start()
}

export async function tearDownBothContainers() {
  console.warn('Tearing down containers')
    await redisContainer.stop()
}
export async function initBothContainers() {
  try {
    redisContainer = await startContainer(REDIS_HOST_DOCKER_IMAGE_NAME)
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
