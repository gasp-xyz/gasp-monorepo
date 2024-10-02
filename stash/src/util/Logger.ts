import { createLogger, format, transports } from 'winston'
import { LoggingWinston } from '@google-cloud/logging-winston'

const logger = createLogger({
  format: format.combine(format.splat(), format.simple()),
  transports: [
    process.env.GAE_APPLICATION
      ? new LoggingWinston({
          level: 'debug',
        })
      : new transports.Console({
          level: 'debug',
        }),
    new transports.File({ filename: 'debug.log', level: 'debug' }),
  ],
})

export default logger
