import axios from 'axios'
import { describe, it, expect } from 'vitest'

const BASE_URL =
  'https://gasp-stash-prod-dot-direct-pixel-353917.oa.r.appspot.com'

const endpoints = [
  '/affirmed-network/list',
  '/affirmed-token/list',
  '/coingecko/pairs',
  '/price-discovery/0',
  '/price-history/0',
  '/price-history/pair/0/1',
  '/tvl-history/0',
  '/token/list/stats',
  '/token/order-buckets',
  '/account/0x3cd0a705a2dc65e5b1e1205896baa2be8a07c6e0/token-portfolio',
  '/token/0/stats',
  '/volume-history/0',
  '/volume-history/pools/0-1',
  '/collators/staking/apy',
  '/price/token/0',
  '/account/0x3cd0a705a2dc65e5b1e1205896baa2be8a07c6e0/dashboard',
]
describe('PROD Stash health check', () => {
  endpoints.forEach((endpoint) => {
    it(`GET ${endpoint} should return 200`, async () => {
      const res = await axios.get(`${BASE_URL}${endpoint}`)
      expect(res.status).toEqual(200)
    })
  })
})
