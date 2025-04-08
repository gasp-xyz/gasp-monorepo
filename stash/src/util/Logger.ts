import { LoggingWinston } from '@google-cloud/logging-winston'
import winston from 'winston'

const loggingWinston = new LoggingWinston({
  // https://github.com/googleapis/nodejs-logging-winston?tab=readme-ov-file#error-handling-with-a-default-callback
  defaultCallback: (err) => {
    if (err) {
      console.log('Error occured: ' + err)
    }
  },
  projectId: process.env.GCP_PROJECT_ID || 'direct-pixel-353917',
  // Needed to fix missing logs in GAE environment
  // Ref: https://github.com/googleapis/nodejs-logging-winston?tab=readme-ov-file#alternative-way-to-ingest-logs-in-google-cloud-managed-environments
  redirectToStdout: true,
})
const logger = winston.createLogger({
  format: winston.format.combine(
    winston.format.splat(),
    winston.format.simple(),
  ),
  level: 'debug',
  transports: [
    process.env.GAE_APPLICATION
      ? loggingWinston
      : new winston.transports.Console(),
  ],
})

export default logger
