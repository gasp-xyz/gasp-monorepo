import supertest from 'supertest'
import { describe, expect,it } from 'vitest'

import app from '../../src/app'
import { tokenIDs } from './utils'

describe('/GET prices', () => {
  it('GET /price-history - Schema validation', async () => {
    await supertest(app)
      .get(`/price-history/0`)
      .expect(200)
      .then((response) => {
        expect(response.body).toMatchSnapshot('Prices')
        expect(response.body.error).toBeUndefined()
        expect(response.body.prices).toBeDefined()
        expect(response.body.prices).toBeInstanceOf(Array)
      })
  })

  it.each(tokenIDs)(
    'should return prices for supported pools: %s',
    async (tokenID) => {
      const response = await supertest(app)
        .get('/price-history/' + tokenID)
        .expect(200)
      expect(response.body).to.have.property('prices')
      expect(response.body.error).toBeUndefined()
      expect(response.body.prices).toBeDefined()
      expect(response.body.prices).toBeInstanceOf(Array)
    }
  )

  describe('API errors', () => {
    it('[MGX-597] - pools should not be returned on prices', async () => {
      const errorMessage =
        'this must be one of the following values: 0, 1, 2, 3, 4, 5, 7, 15, 19'

      await supertest(app)
        .get(`/price-history/0-2?interval=day&days=300`)
        .expect(500)
        .then((response) => {
          const invalidTokenNameResponse = response.body
          expect(invalidTokenNameResponse.exceptionName).to.contain(
            'ValidationError'
          )
          expect(invalidTokenNameResponse.message).to.contain(errorMessage)
        })
    })
  })
})
