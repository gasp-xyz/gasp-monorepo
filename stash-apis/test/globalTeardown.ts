import { tearDownBothContainers } from './api/utils'

const globalTeardown = async () => {
  await tearDownBothContainers()
}

export default globalTeardown
