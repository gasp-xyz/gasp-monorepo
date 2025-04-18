 
import Joi from 'joi'
import supertest from 'supertest'
import { beforeAll, describe, expect, it } from 'vitest'

import app from '../../src/app'
import mangataNode from '../../src/connector/MangataNode'
import { redis } from '../../src/connector/RedisConnector'

const bucketsSchema = Joi.object({
  buckets: Joi.array().items(
    Joi.object({
      bucket: Joi.string().required(),
      rank: Joi.number().required(),
      tokens: Joi.array().min(0).items(Joi.string().optional()),
    })
  ),
})

const expectedBuckets = [
  'stables',
  'bluechips',
  'l0',
  'dextoken',
  'l1',
  'l2',
  'protocols',
  'derivatives',
]

const expectedTokens = [
  'USDT',
  'USDC',
  'aUSD',
  'BTC',
  'ETH',
  'DOT',
  'KSM',
  'MGA',
  '0',
  'GETH',
  'L1Asset',
  'MOVR',
  'BNC',
  'OAK',
  'TUR',
  'IMBU',
  'ZLK',
  'RMRK',
  'vKSM',
  'vsKSM',
  'vMOVR',
  'vBNC',
]

beforeAll(async () => {
  // remove all the inserts when new snapshot is created
  await new Promise((f) => setTimeout(f, 1000))
  const TOKEN_ORDER_BUCKETS_KEY = 'token_order_buckets'
  await redis.client.hset(TOKEN_ORDER_BUCKETS_KEY, {
    ['stables']: JSON.stringify({
      bucket: 'stables',
      rank: 1,
      tokens: ['USDT', 'USDC', 'aUSD'],
    }),
  })
  await redis.client.hset(TOKEN_ORDER_BUCKETS_KEY, {
    ['bluechips']: JSON.stringify({
      bucket: 'bluechips',
      rank: 2,
      tokens: ['BTC', 'ETH'],
    }),
  })
  await redis.client.hset(TOKEN_ORDER_BUCKETS_KEY, {
    ['l0']: JSON.stringify({ bucket: 'l0', rank: 3, tokens: ['DOT', 'KSM'] }),
  })
  await redis.client.hset(TOKEN_ORDER_BUCKETS_KEY, {
    ['dextoken']: JSON.stringify({
      bucket: 'dextoken',
      rank: 4,
      tokens: ['MGA', '0', 'GETH', 'L1Asset'],
    }),
  })
  await redis.client.hset(TOKEN_ORDER_BUCKETS_KEY, {
    ['l1']: JSON.stringify({
      bucket: 'l1',
      rank: 5,
      tokens: ['MOVR', 'BNC', 'OAK', 'TUR', 'IMBU', 'ZLK', 'RMRK'],
    }),
  })
  await redis.client.hset(TOKEN_ORDER_BUCKETS_KEY, {
    ['l2']: JSON.stringify({ bucket: 'l2', rank: 6, tokens: [] }),
  })
  await redis.client.hset(TOKEN_ORDER_BUCKETS_KEY, {
    ['protocols']: JSON.stringify({ bucket: 'protocols', rank: 7, tokens: [] }),
  })
  await redis.client.hset(TOKEN_ORDER_BUCKETS_KEY, {
    ['derivatives']: JSON.stringify({
      bucket: 'derivatives',
      rank: 8,
      tokens: ['vKSM', 'vsKSM', 'vMOVR', 'vBNC'],
    }),
  })
  await new Promise((f) => setTimeout(f, 2000))
})

describe('APi tests: Buckets', () => {
  it('GET token/order-buckets - Schema validation', async () => {
    await supertest(app)
      .get('/token/order-buckets')
      .expect(200)
      .then((response) => {
        const validationResult = bucketsSchema.validate(response.body)
        expect(validationResult.error).toBeUndefined()
      })
  })
  it('GET token/order-buckets - Every token is listed', async () => {
    //now using production, but perhaps using a local setup or chops would help.
    await supertest(app)
      .get('/token/order-buckets')
      .expect(200)
      .then(async (response) => {
        const allTokens = await (
          await mangataNode.api()
        ).query.tokens.totalIssuance.entries()
        const liqTokens = await (
          await mangataNode.api()
        ).query.xyk.liquidityAssets.entries()

        const bucketsFromResponse = response.body.buckets.map((r) => r.bucket)
        expect(bucketsFromResponse).toEqual(expectedBuckets)

        const tokensFromResponse = response.body.buckets.map((r) => r.tokens)
        expect(tokensFromResponse.flat()).toEqual(
          expect.arrayContaining(expectedTokens)
        )

        //Exclude liquidity tokens from all tokens
        const onlyAssets = allTokens
          //Exclude liquidity tokens from all tokens
          .filter(
            (token) =>
              !liqTokens.find(
                (liqToken) =>
                  liqToken[1].toString() === token[0].toHuman()[0].toString()
              )
          )
          //Exclude tokens with 0 balance
          .filter((token) => token[1].toHuman() !== '0')
          .map((token) => token[0].toHuman()[0].toString())
        for (let i = 0; i < onlyAssets.length; i++) {
          const tokenInfo = await mangataNode.query.getTokenInfo(onlyAssets[i])
          console.log(
            'Validating :' + JSON.stringify(tokenInfo) + ' :: ' + onlyAssets[i]
          )
          if (tokenInfo.name.includes('Liquidity')) {
            console.log('Skipping liquidity token: ' + tokenInfo.id)
            continue
          }
          const found = response.body.buckets.filter(
            (bucket: { tokens: string | string[] }) =>
              bucket.tokens.includes(tokenInfo.id)
          )
          console.log('Validating :' + tokenInfo.symbol)
          expect(found).toBeDefined()

          //tokens must only exist on one bucket.
          expect(found.length).toBe(1)
        }
      })
  })
})
