import { afterAll, beforeAll, chai, describe, it, vi } from 'vitest'
import { GenericContainer, Wait } from 'testcontainers'
import Decimal from 'decimal.js'
import IORedis from 'ioredis'
chai.should()

describe.skip('Integration Test using Test Containers', function () {
  let redisClientTs
  let container
  let timeseriesContainer

  beforeAll(async () => {
    //perhaps we should pull this separatetdly, its a 2GB image. `docker pull p1k1m4n/stash:1`
    container = await new GenericContainer('mangatasolutions/redis-test-stash:latest')
      .withWorkingDir('/')
      .withEntrypoint(['redis-server'])
      .withExposedPorts(6379)
      .withNetworkMode('host')
      .withWaitStrategy(Wait.forLogMessage('Ready to accept connections'))
      .start()
    timeseriesContainer = await new GenericContainer('redis/redis-stack:latest')
      .withExposedPorts({ container: 6379, host: 6380 })
      .withWaitStrategy(Wait.forLogMessage('Ready to accept connections'))
      .start()

    process.env.REDIS_HOST = container.getHost()
    process.env.REDIS_PORT = '6379'
    process.env.REDIS_PASS = ''

    process.env.TIMESERIES_HOST = timeseriesContainer.getHost()
    process.env.TIMESERIES_PORT = '6380'
    process.env.TIMESERIES_PASS = ''

    vi.mock('../src/repository/ChainRepository', async () => {
      const actual = await vi.importActual('../src/repository/ChainRepository')
      return {
        ...actual,
        getPools: vi.fn().mockImplementation((id, from, to) => {
          return [
            '{"id":5,"amounts":["13565789249280502","3.90486697526747697749232102e+26"],"assets":[4,0],"block":2288098,"timestamp":1683581142721}',
            '{"id":5,"amounts":["13565789249280502","3.90486697526747697749232102e+26"],"assets":[4,0],"block":2288099,"timestamp":1683581154821}',
            '{"id":5,"amounts":["13564252884199447","3.90531015013489811424011064e+26"],"assets":[4,0],"block":2288100,"timestamp":1683581166793}',
          ]
            .map((s) => JSON.parse(s))
            .map((p) => {
              return {
                id: Number.parseInt(p.id),
                assets: [Number.parseInt(p.assets[0]), Number.parseInt(p.assets[1])],
                block: Number.parseInt(p.block),
                timestamp: Number.parseInt(p.timestamp),
                amounts: [new Decimal(p.amounts[0]), new Decimal(p.amounts[1])],
              }
            })
        }),
        getLatest: vi.fn().mockReturnValue({ timestamp: 1683581166793 }),
        LIMIT: 100000,
      }
    })
  })

  afterAll(async () => {
    await container.stop()
  })

  it('After a big swap, assets must be impacted', async () => {
    const prices = await import('../src/processing/PriceProcessorService')
    await prices.initService()
    //TODO: check why the repo returns empty.
    //const pricesStore = await import('../src/repository/PriceRepository')
    //const results = await pricesStore.get('price:asset:0', 1683581142721, 1683581166793)

    redisClientTs = new IORedis({
      port: 6380,
      host: timeseriesContainer.getHost(),
    })
    const res = (await redisClientTs.call('TS.RANGE', 'price:asset:0', '1683581142721', '1683581166793')) as [
      number,
      string
    ][]
    res.length.should.be.equal(3)
    const storeValues = res.map(([tsp, price]) => [tsp, new Decimal(price)])
    storeValues[2][1].toNumber().should.be.greaterThan(storeValues[0][1].toNumber())
  })
})
