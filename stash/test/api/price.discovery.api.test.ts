import { describe, expect, it } from 'vitest'
import supertest from 'supertest'
import app from '../../src/app'
import { MAX_DAYS, MAX_INTERVAL } from './utils'
import { compactStripLength } from '@polkadot/util'

describe('APi tests: price-discovery', () => {
  it.only('should return current prices for given currency ID', async () => {
    await supertest(app)
      .get(`/price-discovery/0`)
      .expect(200)
      .then((response) => {
        console.dir(response)
        // expect(response.body.error).toBeUndefined()
        // expect(response.body.prices).toBeDefined()
        // expect(response.body.prices).toBeInstanceOf(Array)
      })
  })
})

describe('API Errors: tvl-history/pools', () => {
  it('GET pools/foo does not exist Expect validation error', async () => {
    const errorMessage =
      'this must be one of the following values: 0-2, 2-0, 4-0, 0-4, 0-7, 7-0, 4-7, 7-4, 5-0, 0-5, 0-11, 11-0, 11-4, 4-11, 0-14, 14-0, 16-0, 0-16, 16-4, 4-16, ALL'
    await supertest(app)
      .get('/tvl-history/pools/foo')
      .query({
        interval: MAX_INTERVAL,
        days: MAX_DAYS,
      })
      .expect(500)
      .then((response) => {
        const fooResponse = response.body
        expect(fooResponse.exceptionName).to.contain('ValidationError')
        expect(fooResponse.message).to.contain(errorMessage)
      })
  })
})
describe.todo('System Errors: tvl-history/pools', () => {
  //more tests will come...
})
