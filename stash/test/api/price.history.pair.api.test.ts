import Joi from 'joi'
import supertest from 'supertest'
import { describe, expect, it } from 'vitest'

import app from '../../src/app'
import { MAX_DAYS, MAX_INTERVAL } from './utils'

const pricesSchema = Joi.object({
  prices: Joi.array().items(
    Joi.array().items(Joi.number().required(), Joi.string().required())
  ),
})

describe('APi tests: price-history/pair', () => {
  const pair = '0/1'
  const reversedPair = '1/0'
  it('GET pair 0/2 validate schema', async () => {
    const ksmMgx = await supertest(app)
      .get('/price-history/pair/' + pair)
      .query({
        interval: MAX_INTERVAL,
        days: MAX_DAYS,
      })
      .expect(200)
    const validationResult = pricesSchema.validate(ksmMgx.body)
    expect(validationResult.error).toBeUndefined()
  })

  it('GET pair 0/2 returns same as pair 2/0', async () => {
    const ksmMgx = await supertest(app)
      .get('/price-history/pair/' + pair)
      .query({
        interval: MAX_INTERVAL,
        days: MAX_DAYS,
      })
      .expect(200)
    const mgxKsm = await supertest(app)
      .get('/price-history/pair/' + reversedPair)
      .query({
        interval: MAX_INTERVAL,
        days: MAX_DAYS,
      })
      .expect(200)
    expect(ksmMgx.body.prices).toHaveLength(mgxKsm.body.prices.length)
    const timestamps1 = ksmMgx.body.prices.map((p: any) => p[0])
    const timestamps2 = mgxKsm.body.prices.map((p: any) => p[0])
    expect(timestamps1).toStrictEqual(timestamps2)
  })

  it('GET pools/0/0: pool that does not exist expect empty', async () => {
    await supertest(app)
      .get('/price-history/pair/0/0')
      .query({
        interval: MAX_INTERVAL,
        days: MAX_DAYS,
      })
      .expect(200)
      .then((response) => {
        const poolDoesNotExist = response.body
        expect(poolDoesNotExist.prices).to.be.empty
      })
  })
})

describe('API Errors: price-history/pair', () => {
  const errorMessage =
    'this must be one of the following values: 0, 1, 2, 3, 4, 5, 7, 15, 19'

  it('GET pools/foo: token does not exist Expect validation error', async () => {
    await supertest(app)
      .get('/price-history/pair/L1Asset/foo')
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
