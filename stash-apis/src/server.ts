import 'dotenv/config'
import app from './app.js'
import * as tokenService from './service/TokenService.js'
import * as xcmService from './service/XcmNetworkService.js'
import * as networkService from './service/NetworkService.js'
import logger from './util/Logger.js'

// Express Server boot
const server = app.listen(app.get('port'), async () => {
  logger.info(
    'Server started: http://localhost:%d in %s mode',
    app.get('port'),
    app.get('env')
  )

  await tokenService.initService()

  await networkService.initService()

  await xcmService.initService()

  logger.info('DB initialized')
})

export default server
