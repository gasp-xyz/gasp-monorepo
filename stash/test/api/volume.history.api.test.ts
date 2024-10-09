import { describe, expect, it } from 'vitest'
import supertest from 'supertest'
import app from '../../src/app'

describe('APi tests: volume-history', () => {
  const tokenIDs = ['0', '1', '2', '3', '4', '5', '7', '15', '19']
  it.each(tokenIDs)(
    'should return volumes for supported pools: %s',
    async (tokenID) => {
      const response = await supertest(app)
        .get('/volume-history/' + tokenID)
        .expect(200)
      expect(response.body).to.have.property('volumes')
      expect(response.body.error).toBeUndefined()
      expect(response.body.volumes).toBeDefined()
      expect(response.body.volumes).toBeInstanceOf(Array)
    }
  )
})

describe('API Errors: volume-history/', () => {
  it('GET pools/foo does not exist -> Expect validation error', async () => {
    const errorMessage =
      'this must be one of the following values: 0, 1, 2, 3, 4, 5, 7, 15, 19'
    await supertest(app)
      .get('/volume-history/foo')
      .expect(500)
      .then((response) => {
        const fooResponse = response.body
        expect(fooResponse.exceptionName).to.contain('ValidationError')
        expect(fooResponse.message).to.contain(errorMessage)
      })
  })
})
