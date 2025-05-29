import supertest from 'supertest'
import { describe, expect, it } from 'vitest'

import app from '../../src/app'
import { MAX_DAYS, MAX_INTERVAL } from './utils'


// These tests will fail if images changes And/Or if bugfixes. Careful when updating!
describe.todo('Snapshots tests: tvl-history/pools', () => {
  //more tests will come...
})
describe('API Errors: tvl-history/pools', () => {
  it('GET pools/foo does not exist Expect validation error', async () => {
    await supertest(app)
      .get('/tvl-history/pools/foo')
      .query({
        interval: MAX_INTERVAL,
        days: MAX_DAYS,
      })
      .expect(200)
      .then((response) => {
        expect(response.body).toEqual({ volumes: [] })
      })
  })
})
describe.todo('System Errors: tvl-history/pools', () => {
  //more tests will come...
})
