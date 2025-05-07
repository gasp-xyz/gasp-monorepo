 
import { BN } from '@polkadot/util'
import { BN_TEN } from 'gasp-sdk'
import Joi from 'joi'
import moment from 'moment'
import supertest from 'supertest'
import { describe, expect, it } from 'vitest'

import app from '../../src/app'

const apySchema = Joi.array().items(
  Joi.object({
    date: Joi.string().required(),
    apy: Joi.number().precision(18).unsafe().required(),
    token: Joi.string().required(),
    collatorAddress: Joi.string().required(),
    dateFormat: Joi.string().required(),
    timestamp: Joi.date().timestamp().required(),
  })
)

const dailyRewardSchema = Joi.array().items(
  Joi.object({
    tokenId: Joi.string().required(),
    dailyRewards: Joi.number().precision(18).unsafe().required(),
    dateFormat: Joi.string().required(),
    date: Joi.string().required(),
    timestamp: Joi.date().timestamp().required(),
  })
)

function validateReturnedDate(date: string, format: string) {
  const dateFormat = moment(date, format, true)
  const dateFromMoment = dateFormat.toDate()
  expect(dateFromMoment.getDate()).to.equal(parseInt(date.split('/')[0]))
  expect(dateFromMoment.getMonth() + 1).to.equal(parseInt(date.split('/')[1]))
  expect(dateFromMoment.getFullYear()).to.equal(parseInt(date.split('/')[2]))
}

//perhaps those addresses may change when data is upgraded!

const collatorAddress = '0x3cd0a705a2dc65e5b1e1205896baa2be8a07c6e0'
const accountAddress = '0x928f1040adb982d3ab32a62dc8eda57e9b81b4dd'

function validateValidApyRange(body) {
  const apy = parseFloat(body.apy)
  expect(apy).to.be.lessThanOrEqual(300)
  expect(apy).to.be.greaterThan(100)
}

describe('APi tests: Collator apy - dailyRewards', () => {
  it('GET /collators/apy - collator - OK', async () => {
    await supertest(app)
      .get(`/collator/${collatorAddress}/staking/apy`)
      .expect(200)
      .then((response) => {
        const validationResult = apySchema.validate(response.body)
        const body = JSON.parse(JSON.stringify(response.body[0]))
        const date = body.date
        const format = body.dateFormat
        expect(validationResult.error).toBeUndefined()
        expect(body.token).to.equal('0')
        validateReturnedDate(date, format)
        validateValidApyRange(body)
      })
  })
  it('GET /collators/dailyReward - collator - OK', async () => {
    await supertest(app)
      .get(`/collator/${collatorAddress}/staking/dailyReward`)
      .expect(200)
      .then((response) => {
        const validationResult = dailyRewardSchema.validate(response.body)
        expect(validationResult.error).toBeUndefined()
        const body = JSON.parse(JSON.stringify(response.body[0]))
        const date = body.date
        const format = body.dateFormat
        expect(body.tokenId).to.equal('0')
        validateReturnedDate(date, format)
        const dailyRewards = body.dailyRewards
        const rewardValue = new BN(dailyRewards).div(BN_TEN.pow(new BN(18)))
        expect(rewardValue.toNumber()).gt(500000)
      })
  })

  it('GET account/liquid-staking/rewards-history/month/sum - OK', async () => {
    await supertest(app)
      .get(
        `/account/${accountAddress}/liquid-staking/rewards-history/month/sum`
      )
      .expect(200)
      .then((response) => {
        const body = response.body[0]
        expect(body.liquidityTokenId).to.equal('5')
        const amountClaimed = new BN(body.amountClaimed).div(
          BN_TEN.pow(new BN(18))
        )
        expect(amountClaimed.toNumber()).gt(500000)
      })
  })

  it.skip('GET /collators/apy - old - collator - OK', async () => {
    //todo: qa to fix and remove skip
    await supertest(app)
      .get(`/collator/${ethCollatorAddress}/staking/apy`)
      // .expect(200)
      .then((response) => {
        console.log(response)
        const validationResult = apySchema.validate(response.body)
        const body = JSON.parse(JSON.stringify(response.body))[5]
        const date = body.date //body.5.date;
        const format = body.dateFormat //response.body.5.dateFormat;
        expect(validationResult.error).toBeUndefined()
        validateReturnedDate(date, format)

        // special validation about old collators should have old date.
        expect(date).to.equal('15/12/2023')
        validateValidApyRange(body)
      })
  })
  it.skip('GET /collators/dailyReward - old - collator - OK', async () => {
    //todo: qa to fix and remove skip
    await supertest(app)
      .get(`/collator/${oldCollatorAddress}/staking/dailyReward`)
      .expect(200)
      .then((response) => {
        const validationResult = dailyRewardSchema.validate(response.body)
        const body = JSON.parse(JSON.stringify(response.body))[0]
        const date = body.date //body.5.date;
        const format = body.dateFormat //response.body.5.dateFormat;
        expect(validationResult.error).toBeUndefined()
        validateReturnedDate(date, format)

        // special validation about old collators should have old date.
        expect(date).to.equal('15/12/2023')
        const reward = response.body[0].dailyRewards
        const rewardValue = new BN(reward).div(BN_TEN.pow(new BN(18)))
        expect(rewardValue.toNumber()).gt(5000)
      })
  })

  describe('API errors', () => {
    const errorMessage = 'This collator has not received any rewards as of yet.'

    it('GET /collators/dailyReward - no data', async () => {
      await supertest(app)
        .get('/collator/foo/staking/dailyreward')
        .expect(404)
        .then((response) => {
          expect(response.body.message).to.equal(errorMessage)
          expect(response.body.exceptionName).to.equal('NotFoundException')
        })
    })

    it('GET /collators/apy - no data', async () => {
      await supertest(app)
        .get('/collator/foo/staking/apy')
        .expect(404)
        .then((response) => {
          expect(response.body.message).to.equal(errorMessage)
          expect(response.body.exceptionName).to.equal('NotFoundException')
        })
    })
  })
})
