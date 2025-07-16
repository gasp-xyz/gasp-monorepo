 
import supertest from 'supertest'
import { describe, expect, it } from 'vitest'

import app from '../../src/app'
import { MAX_DAYS, MAX_INTERVAL } from './utils'

// These tests will fail if images changes And/Or if bugfixes. Careful when updating!
describe.todo('Snapshots tests: volume-history/pools', () => {
  //more tests will come...
})
describe('API Errors: volume-history/pools', () => {
  it('GET pools/foo does not exist -> Expect validation error', async () => {
    const errorMessage =
      'this must be one of the following values: 0-1, 1-0, 3-1, 1-3, 3-0, 0-3, 0-2, 2-0, 7-5, 5-7, 1-7, 7-1, 4-5, 5-4, 3-4, 4-3, 1-5, 5-1, 0-19, 19-0, 19-1, 1-19, 4-0, 0-4, 7-0, 0-7, 1-4, 4-1, 0-5, 5-0, 5-3, 3-5, 1-15, 15-1, ALL'
    await supertest(app)
      .get('/volume-history/pools/foo')
      .query({
        interval: MAX_INTERVAL,
        days: MAX_DAYS,
      })
      .expect(200)
      .then((response) => {
        const fooResponse = response.body
        expect(fooResponse).to.deep.equal({ volumes: [] })
      })
  })
})
describe.todo('System Errors: volume-history/pools', () => {
  //more tests will come...
})
