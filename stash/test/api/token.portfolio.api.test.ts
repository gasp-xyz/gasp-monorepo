import supertest from 'supertest'
import { describe, expect,it } from 'vitest'

import app from '../../src/app'

describe('/GET token-portfolio', () => {
  it('should return token portfolio for provided account', async () => {
    const aliceAcc = '0xf24ff3a9cf04c71dbc94d0b566f7a27b94566cac'
    await supertest(app)
      .get(`/account/${aliceAcc}/token-portfolio`)
      .expect(200)
      .then((response) => {
        expect(response.body.error).toBeUndefined()
        expect(response.body).toBeDefined()
        expect(response.body).toBeInstanceOf(Array)
        response.body.forEach((item) => {
          expect(item).toHaveProperty('tokenId')
          expect(typeof item.tokenId).toBe('string')

          expect(item).toHaveProperty('balance')
          expect(typeof item.balance).toBe('string')

          expect(item).toHaveProperty('balanceInUsd')
          expect(typeof item.balanceInUsd).toBe('string')
        })
      })
  })

  it('should return an EMPTY token portfolio for provided account', async () => {
    const emptyAcc = '0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF'
    await supertest(app)
      .get(`/account/${emptyAcc}/token-portfolio`)
      .expect(200)
      .then((response) => {
        expect(response.body.error).toBeUndefined()
        expect(response.body).toBeDefined()
        expect(response.body).toBeInstanceOf(Array)
        expect(response.body).toHaveLength(0)
      })
  })

  describe('API errors', () => {
    it('should throw an error when using address without 0x prefix', async () => {
      const nonExistingAcc = 'VERYBADACCOUNT'
      const errorMsg =
        'createType(Lookup0):: Expected input with 20 bytes (160 bits), found 14 bytes'
      await supertest(app)
        .get(`/account/${nonExistingAcc}/token-portfolio`)
        .expect(500)
        .then((response) => {
          const errorResponse = response.body
          expect(errorResponse.exceptionName).to.contain('Error')
          expect(errorResponse.message).to.contain(errorMsg)
        })
    })
  })
})
