import supertest from 'supertest'
import { describe, expect, it } from 'vitest'

import app from '../../src/app'
import { tokenIDs } from './utils'

describe('APi tests: volume-history', () => {
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
    await supertest(app)
      .get('/volume-history/foo')
      .expect(200)
      .then((response) => {
        const fooResponse = response.body
        expect(fooResponse).to.deep.equal({ volumes: [] })
      })
  })
})
