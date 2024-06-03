import { afterAll, beforeAll, chai, describe, expect, it, vi } from 'vitest'
import { GenericContainer, Wait } from 'testcontainers'
// eslint-disable-next-line import/no-named-as-default
import Decimal from 'decimal.js/decimal'
import IORedis from 'ioredis'
chai.should()

describe('Multiswap test: skip summary event', function () {
  let timeseriesContainer

  beforeAll(async () => {
    timeseriesContainer = await new GenericContainer('redis/redis-stack:latest')
      .withExposedPorts({ container: 6379, host: 6381 })
      .withWaitStrategy(Wait.forLogMessage('Ready to accept connections'))
      .start()

    process.env.REDIS_HOST = 'localhost'
    process.env.REDIS_PORT = '6380'
    process.env.REDIS_PASS = ''

    process.env.TIMESERIES_HOST = timeseriesContainer.getHost()
    process.env.TIMESERIES_PORT = '6381'
    process.env.TIMESERIES_PASS = ''

    vi.mock('../src/repository/ChainRepository', async () => {
      const actual = await vi.importActual('../src/repository/ChainRepository')

      return {
        ...actual,
        getEvents: vi.fn().mockImplementation(() => {
          return [
            //this comes from:ZRANGEBYSCORE chain:events 1689269814386 1689269814386
            '{"timestamp":1689269814386,"block":2750652,"events":[[{"method":"Reserved","section":"tokens","index":"0x0a03","data":{"currencyId":"0","who":"5EjiypJ12geC4xhaTi6YFU8BUH9LLzqk4VhmRSeBaSoB4zRA","amount":"50,000,000,000,000,000,000"}},{"method":"FeeLocked","section":"feeLock","index":"0x0f02","data":{"who":"5EjiypJ12geC4xhaTi6YFU8BUH9LLzqk4VhmRSeBaSoB4zRA","lockAmount":"50,000,000,000,000,000,000","totalLocked":"100,000,000,000,000,000,000"}},{"method":"Transfer","section":"tokens","index":"0x0a02","data":{"currencyId":"0","from":"5EjiypJ12geC4xhaTi6YFU8BUH9LLzqk4VhmRSeBaSoB4zRA","to":"5EYCAe5XGPRojsCSi9p1ZZQ5qgeJGFcTxPxrsFRzkASu6bT2","amount":"800,000,000,000,000,001"}},{"method":"Transfer","section":"tokens","index":"0x0a02","data":{"currencyId":"0","from":"5EjiypJ12geC4xhaTi6YFU8BUH9LLzqk4VhmRSeBaSoB4zRA","to":"5EYCAe5ijiYfyeZ2JJCGq56LmPyNRAKzpG4QkoQkkQNB5e6Z","amount":"200,000,000,000,000,001"}},{"method":"Transfer","section":"tokens","index":"0x0a02","data":{"currencyId":"0","from":"5EjiypJ12geC4xhaTi6YFU8BUH9LLzqk4VhmRSeBaSoB4zRA","to":"5EYCAe5ijiYfyeZ2JJezKNMZfdbiFMyQc4YVzxaiMebAZBcm","amount":"200,000,000,000,000,001"}},{"method":"Transfer","section":"tokens","index":"0x0a02","data":{"currencyId":"0","from":"5EjiypJ12geC4xhaTi6YFU8BUH9LLzqk4VhmRSeBaSoB4zRA","to":"5EYCAe5XGPRojsCSi9p1ZZQ5qgeJGFcTxPxrsFRzkASu6bT2","amount":"398,799,999,999,999,999,997"}},{"method":"Transfer","section":"tokens","index":"0x0a02","data":{"currencyId":"4","from":"5EYCAe5XGPRojsCSi9p1ZZQ5qgeJGFcTxPxrsFRzkASu6bT2","to":"5EjiypJ12geC4xhaTi6YFU8BUH9LLzqk4VhmRSeBaSoB4zRA","amount":"10,278,281,175"}},{"method":"AssetsSwapped","section":"xyk","index":"0x0d01","data":["5EjiypJ12geC4xhaTi6YFU8BUH9LLzqk4VhmRSeBaSoB4zRA",["0","4"],"400,000,000,000,000,000,000","10,278,281,175"]},{"method":"Withdrawn","section":"tokens","index":"0x0a08","data":{"currencyId":"0","who":"5EYCAe5ijiYfyeZ2JJezKNMZfdbiFMyQc4YVzxaiMebAZBcm","amount":"200,000,000,000,000,001"}},{"method":"Transfer","section":"tokens","index":"0x0a02","data":{"currencyId":"4","from":"5EjiypJ12geC4xhaTi6YFU8BUH9LLzqk4VhmRSeBaSoB4zRA","to":"5EYCAe5XGPRojsCSi9p1ZZQ5qgeJGFcTxPxrsFRzkASu6bT2","amount":"20,556,563"}},{"method":"Transfer","section":"tokens","index":"0x0a02","data":{"currencyId":"4","from":"5EjiypJ12geC4xhaTi6YFU8BUH9LLzqk4VhmRSeBaSoB4zRA","to":"5EYCAe5ijiYfyeZ2JJCGq56LmPyNRAKzpG4QkoQkkQNB5e6Z","amount":"5,139,141"}},{"method":"Transfer","section":"tokens","index":"0x0a02","data":{"currencyId":"4","from":"5EjiypJ12geC4xhaTi6YFU8BUH9LLzqk4VhmRSeBaSoB4zRA","to":"5EYCAe5ijiYfyeZ2JJezKNMZfdbiFMyQc4YVzxaiMebAZBcm","amount":"5,139,141"}},{"method":"Transfer","section":"tokens","index":"0x0a02","data":{"currencyId":"4","from":"5EjiypJ12geC4xhaTi6YFU8BUH9LLzqk4VhmRSeBaSoB4zRA","to":"5EYCAe5XGPRojsCSi9p1ZZQ5qgeJGFcTxPxrsFRzkASu6bT2","amount":"10,247,446,330"}},{"method":"Transfer","section":"tokens","index":"0x0a02","data":{"currencyId":"30","from":"5EYCAe5XGPRojsCSi9p1ZZQ5qgeJGFcTxPxrsFRzkASu6bT2","to":"5EjiypJ12geC4xhaTi6YFU8BUH9LLzqk4VhmRSeBaSoB4zRA","amount":"252,846"}},{"method":"AssetsSwapped","section":"xyk","index":"0x0d01","data":["5EjiypJ12geC4xhaTi6YFU8BUH9LLzqk4VhmRSeBaSoB4zRA",["4","30"],"10,278,281,175","252,846"]},{"method":"Transfer","section":"tokens","index":"0x0a02","data":{"currencyId":"4","from":"5EYCAe5ijiYfyeZ2JJCGq56LmPyNRAKzpG4QkoQkkQNB5e6Z","to":"5EYCAe5XGPRojsCSi9p1ZZQ5qgeJGFcTxPxrsFRzkASu6bT2","amount":"5,139,141"}},{"method":"Transfer","section":"tokens","index":"0x0a02","data":{"currencyId":"0","from":"5EYCAe5XGPRojsCSi9p1ZZQ5qgeJGFcTxPxrsFRzkASu6bT2","to":"5EYCAe5ijiYfyeZ2JJCGq56LmPyNRAKzpG4QkoQkkQNB5e6Z","amount":"199,400,187,378,441,148"}},{"method":"Transfer","section":"tokens","index":"0x0a02","data":{"currencyId":"4","from":"5EYCAe5ijiYfyeZ2JJezKNMZfdbiFMyQc4YVzxaiMebAZBcm","to":"5EYCAe5XGPRojsCSi9p1ZZQ5qgeJGFcTxPxrsFRzkASu6bT2","amount":"5,139,141"}},{"method":"Withdrawn","section":"tokens","index":"0x0a08","data":{"currencyId":"0","who":"5EYCAe5XGPRojsCSi9p1ZZQ5qgeJGFcTxPxrsFRzkASu6bT2","amount":"199,400,187,378,441,148"}},{"method":"AssetsSwapped","section":"xyk","index":"0x0d01","data":["5EjiypJ12geC4xhaTi6YFU8BUH9LLzqk4VhmRSeBaSoB4zRA",["0","4","30"],"400,000,000,000,000,000,000","252,846"]},{"method":"ExtrinsicSuccess","section":"system","index":"0x0000","data":{"dispatchInfo":{"weight":{"refTime":"3,994,270,020","proofSize":"0"},"class":"Normal","paysFee":"Yes"}}}]]}',
          ]
            .map((s) => JSON.parse(s))
            .map((json) => {
              return {
                block: Number.parseInt(json.block),
                timestamp: Number.parseInt(json.timestamp),
                events: json.events,
              }
            })
        }),
        getLatest: vi.fn().mockReturnValue({ timestamp: 1689269814386 }),
        LIMIT: 100000,
      }
    })
    vi.mock('../src/repository/PriceRepository', async () => {
      const actual = await vi.importActual('../src/repository/PriceRepository')
      return {
        ...actual,
        get: vi.fn().mockImplementation(() => {
          //Dummy price for the timestamp
          return (
            [[1689269814386, '1.E-7']] as unknown as [number, string][]
          ).map(([tsp, price]) => [tsp, new Decimal(price)])
        }),
        getLatest: vi.fn().mockReturnValue(1689269814386),
        LIMIT: 100000,
      }
    })
    vi.mock('../src/repository/TradeVolumeRepository', async () => {
      const actual = await vi.importActual(
        '../src/repository/TradeVolumeRepository'
      )
      return {
        ...actual,
        getLatest: vi.fn().mockReturnValue(1689269814386),
        LIMIT: 100000,
      }
    })
  })

  afterAll(async () => {
    await timeseriesContainer.stop()
  })

  it('[0-30] Event should not be mapped - 1689269814386', async () => {
    const trades = await import('../src/processing/TradeVolumeProcessorService')
    await trades.initService()

    const redisClientTs = new IORedis({
      port: 6381,
      host: timeseriesContainer.getHost(),
    })
    let failed = false
    let err = ''
    await redisClientTs
      .call('TS.RANGE', 'trades:pool:36', '1689269814386', '1689269814386')
      .catch((error) => {
        failed = true
        err = error.message
      })
    expect(failed).to.be.true
    expect(err).to.be.equal('ERR TSDB: the key does not exist')

    const swap1 = await redisClientTs.call(
      'TS.RANGE',
      'trades:pool:32',
      '1689269814386',
      '1689269814386'
    )
    const swap2 = await redisClientTs.call(
      'TS.RANGE',
      'trades:pool:5',
      '1689269814386',
      '1689269814386'
    )
    expect(swap1).not.to.be.empty
    expect(swap2).not.to.be.empty
  })
})
