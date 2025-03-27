import supertest from 'supertest'
import { describe, expect, it } from 'vitest'

import app from '../../src/app'
import { MAX_DAYS, MAX_INTERVAL, tokenIDs } from './utils'

describe('APi tests: price-discovery', () => {
  it.each(tokenIDs)(
    'should return current prices for supported token IDs: %s',
    async (tokenID) => {
      const response = await supertest(app)
        .get('/price-discovery/' + tokenID)
        .expect(200)
      expect(response.body).to.have.property('current_price')
      expect(response.body.error).toBeUndefined()
      expect(response.body.current_price).toBeDefined()
      expect(response.body.current_price).toBeInstanceOf(Object)
    }
  )
})

describe('API Errors: price-discovery', () => {
  it('GET price-discovery/foo does not exist Expect validation error', async () => {
    const errorMessage =
      'this must be one of the following values: 0, 1, 2, 3, 4, 5, 7, 15, 19'
    await supertest(app)
      .get('/price-discovery/foo')
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
