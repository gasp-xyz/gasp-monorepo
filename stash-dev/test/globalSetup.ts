import { REDIS_HOST_DOCKER_IMAGE_NAME, startContainer, TIMESERIES_HOST_DOCKER_IMAGE_NAME } from './api/utils'

const globalConfig = async () => {
  await startContainer(TIMESERIES_HOST_DOCKER_IMAGE_NAME)
  await startContainer(REDIS_HOST_DOCKER_IMAGE_NAME)
}

export default globalConfig

//afterAll(async () => {
//  await tearDownBothContainers()
//})
