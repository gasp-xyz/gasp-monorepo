import { defineConfig } from 'vitest/config'
import * as process from 'node:process'


export default defineConfig({
  test: {
    hookTimeout: 60000,
    testTimeout: 60000,
    globalSetup: 'test/globalSetup.ts',
    globalTeardown: 'test/globalTeardown.ts',
    env: {
      APP_ENV: 'dev',
      MANGATA_NODE_URL: 'wss://kusama-archive.mangata.online',
      REDIS_HOST: 'localhost',
      REDIS_PORT: '6380',
      REDIS_PASS: '',
      TIMESERIES_HOST: 'localhost',
      TIMESERIES_PORT: '6379',
      TIMESERIES_PASS: '',
      COINGECKO_API_KEY: process.env.COINGECKO_API_KEY
    },
    coverage: {
      include: ['src/**/*'],
      reporter: ['text', 'json-summary', 'json'],
    },
  },
})
