import { defineConfig } from 'vitest/config'
import * as process from 'node:process'

export default defineConfig({
  test: {
    hookTimeout: 60000,
    testTimeout: 60000,
    globalSetup: 'test/globalSetup.ts',
    globalTeardown: 'test/globalTeardown.ts',
    env: {
      MANGATA_NODE_URL: 'wss://collator-01-ws-rollup-frontend.gasp.xyz',
      OLD_MANGATA_NODE_URL: 'wss://kusama-archive.mangata.online',
      REDIS_HOST: 'localhost',
      REDIS_PORT: '6380',
      REDIS_PASS: '',
      COINGECKO_API_KEY: process.env.COINGECKO_API_KEY,
      CAPTCHA_SITEKEY: '10000000-ffff-ffff-ffff-000000000001',
      CAPTCHA_SECRET: '0x0000000000000000000000000000000000000000',
      TOKEN_FOR_PRICE_ID: '1',
      MGX_TOKEN_ID: '0',
      MGX_AIRDROP_SNAPSHOTS: '123',
    },
    coverage: {
      include: ['src/**/*'],
      reporter: ['text', 'json-summary', 'json'],
    },
  },
})
