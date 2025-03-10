import { describe, expect, it } from 'vitest'
import supertest from 'supertest'
import app from '../../src/app'
import { MAX_DAYS, MAX_INTERVAL } from './utils'

describe('APi tests: tvl-history/pools', () => {
  const testPool = '0-1'
  const testPoolReversed = '1-0'
  it('GET pools/0-1 returns the same as pools/1-0 -> Expect deep equal', async () => {
    const gaspv2L1Asset = await supertest(app)
      .get('/tvl-history/pools/' + testPool)
      .query({
        interval: MAX_INTERVAL,
        days: MAX_DAYS,
      })
      .expect(200)
    const l1AssetGaspv2 = await supertest(app)
      .get('/tvl-history/pools/' + testPoolReversed)
      .query({
        interval: MAX_INTERVAL,
        days: MAX_DAYS,
      })
      .expect(200)
    expect(gaspv2L1Asset.body).to.deep.equal(l1AssetGaspv2.body)
    expect(gaspv2L1Asset.body).to.have.property('volumes')
    expect(gaspv2L1Asset.body.volumes).toBeInstanceOf(Array)
    expect(gaspv2L1Asset.body.volumes[0]).toHaveLength(2)
  })
})

// These tests will fail if images changes And/Or if bugfixes. Careful when updating!
describe.todo('Snapshots tests: tvl-history/pools', () => {
  //more tests will come...
})
describe('API Errors: tvl-history/pools', () => {
  it('GET pools/foo does not exist Expect validation error', async () => {
    const errorMessage =
      'this must be one of the following values: 0-1, 1-0, 3-1, 1-3, 3-0, 0-3, 0-2, 2-0, 7-5, 5-7, 1-7, 7-1, 4-5, 5-4, 3-4, 4-3, 1-5, 5-1, 0-19, 19-0, 19-1, 1-19, 4-0, 0-4, 7-0, 0-7, 1-4, 4-1, 0-5, 5-0, 5-3, 3-5, 1-15, 15-1, ALL'

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
