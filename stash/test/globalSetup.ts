import { REDIS_HOST_DOCKER_IMAGE_NAME, startContainer } from './api/utils'

const globalConfig = async () => {
  await startContainer(REDIS_HOST_DOCKER_IMAGE_NAME)
}

export default globalConfig

//afterAll(async () => {
//  await tearDownBothContainers()
//})
