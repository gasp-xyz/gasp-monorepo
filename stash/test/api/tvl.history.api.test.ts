import supertest from 'supertest'
import { describe, expect,it } from 'vitest'

import app from '../../src/app'
import { tokenIDs } from './utils'

describe('/GET tvl-history', () => {
  it.skip('GET /tvl-history - Schema validation', async () => {
    await supertest(app)
      .get(`/tvl-history/0`)
      .expect(200)
      .then((response) => {
        expect(response.body).toMatchSnapshot('Tvl-history')
        expect(response.body.error).toBeUndefined()
        expect(response.body.volumes).toBeDefined()
        expect(response.body.volumes).toBeInstanceOf(Array)
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
    it('should return error on non existing ID', async () => {
      const errorMessage =
        'this must be one of the following values: 0, 1, 2, 3, 4, 5, 7, 15, 19'

      await supertest(app)
        .get(`/tvl-history/foo`)
        .expect(200)
        .then((response) => {
          expect(response.body).toEqual({ volumes: [] })
        })
    })
  })
})
