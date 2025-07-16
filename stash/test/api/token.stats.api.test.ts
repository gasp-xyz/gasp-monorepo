 
import Joi from 'joi'
import supertest from 'supertest'
import { describe, expect, it } from 'vitest'

import app from '../../src/app'
import MangataClient from '../../src/connector/MangataNode'

const tokenSchema = Joi.object({
  tokenId: Joi.string().required(),
  tokenName: Joi.string().required(),
  symbol: Joi.string().required(),
  priceInUSD: Joi.number().precision(18).unsafe().required(),
  volume24hInUSD: Joi.number().precision(18).unsafe().required(),
  liquidity24hInUSD: Joi.number().precision(18).unsafe().required(),
  priceChange24hInPerc: Joi.number().precision(18).unsafe().required(),
  volumeChange24hInPerc: Joi.number().precision(18).unsafe().required(),
})

const tokenId = '0'
const tokenName = 'Gasp'
const symbol = 'GASP'

const listSchema = Joi.array().items(tokenSchema)
//Values can not be tested, since data is old, and this api request l&g values.
describe('APi tests: token stats', () => {
  it('GET /token/list/stats - Schema validation', async () => {
    await supertest(app)
      .get('/token/list/stats')
      .expect(200)
      .then((response) => {
        const validationResult = listSchema.validate(response.body)
        expect(validationResult.error).toBeUndefined()
      })
  })
  it('GET /token/{tokenId}/stats - Schema validation', async () => {
    await supertest(app)
      .get(`/token/${tokenId}/stats`)
      .expect(200)
      .then((response) => {
        const validationResult = tokenSchema.validate(response.body)
        expect(validationResult.value).toBeDefined()
        expect(validationResult.value).toEqual(
          expect.objectContaining({
            tokenId: tokenId,
            tokenName: tokenName,
            symbol: symbol,
          })
        )
        expect(validationResult.error).toBeUndefined()
      })
  })
  it('GET list & GET token stat info matches', async () => {
    await supertest(app)
      .get(`/token/${tokenId}/stats`)
      .expect(200)
      .then(async (response) => {
        const allTokens = (
          await supertest(app).get('/token/list/stats').expect(200)
        ).body
        const gaspV2Token = response.body
        const onlyGaspV2 = allTokens.filter(
          (token: { tokenId: number }) => token.tokenId === gaspV2Token.tokenId
        )
        expect(onlyGaspV2.length).toEqual(1)
        expect(onlyGaspV2[0]).toEqual(gaspV2Token)
      })
  })
  it.skip('GET /token/list/stats - List matches with all the tokens with pool', async () => { //to fix data on the frontend node
    const sdk = MangataClient
    const api = await sdk.api()
    const pools = await api.query.xyk.liquidityPools.entries()
    const allstats = (await supertest(app).get('/token/list/stats')).body
    pools.forEach((pool) => {
      const firstTokenId = pool[1].toHuman()[0]
      const secondTokenId = pool[1].toHuman()[1]
      const firstToken = allstats.filter(
        (token: { tokenId: number }) => token.tokenId === firstTokenId
      )
      const secondToken = allstats.filter(
        (token: { tokenId: number }) => token.tokenId === secondTokenId
      )
      expect(firstToken.length).toEqual(1)
      expect(secondToken.length).toEqual(1)
    })
  })
})

describe('API Errors', () => {
  it('GET non existing token ID', async () => {
    const errorMsg = 'Unknown currency id.'
    const randomID = 2736
    await supertest(app)
      .get(`/token/${randomID}/stats`)
      .expect(404)
      .then((response) => {
        const errorResponse = response.body
        expect(errorResponse.exceptionName).to.contain('NotFoundException')
        expect(errorResponse.message).to.contain(errorMsg)
      })
  })

  it('GET token symbol instead of token ID', async () => {
    const errorMsg =
      'id must be a `number` type, but the final value was: `NaN` (cast from the value `"GASP"`).'
    await supertest(app)
      .get(`/token/${symbol}/stats`)
      .expect(500)
      .then((response) => {
        const errorResponse = response.body
        expect(errorResponse.exceptionName).to.contain('ValidationError')
        expect(errorResponse.message).to.contain(errorMsg)
      })
  })
})
