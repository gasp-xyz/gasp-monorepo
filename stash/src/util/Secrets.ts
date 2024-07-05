import logger from './Logger.js'
import dotenv from 'dotenv'
import fs from 'fs'

if (fs.existsSync('.env')) {
  logger.info('Using .env for environment variables')
  dotenv.config({ path: '.env' })
} else {
  logger.info('Using .env for environment variables')
  dotenv.config({ path: '.env' }) // you can delete this after you create your own .env file!
}
