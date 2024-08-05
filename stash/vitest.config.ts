import { defineConfig } from 'vitest/config'
import * as process from 'node:process'

export default defineConfig({
  test: {
    hookTimeout: 60000,
    testTimeout: 60000,
    globalSetup: 'test/globalSetup.ts',
    globalTeardown: 'test/globalTeardown.ts',
    env: {
      APP_ENV: 'rollup-dev',
      MANGATA_NODE_URL: 'wss://collator-01-ws-rollup-frontend.gasp.xyz',
      REDIS_HOST: 'localhost',
      REDIS_PORT: '6380',
      REDIS_PASS: '',
      TIMESERIES_HOST: 'localhost',
      TIMESERIES_PORT: '6379',
      TIMESERIES_PASS: '',
      COINGECKO_API_KEY: process.env.COINGECKO_API_KEY,
      CAPTCHA_SITEKEY: '10000000-ffff-ffff-ffff-000000000001',
      CAPTCHA_SECRET: '0x0000000000000000000000000000000000000000',
    },
    coverage: {
      include: ['src/**/*'],
      reporter: ['text', 'json-summary', 'json'],
    },
  },
})
