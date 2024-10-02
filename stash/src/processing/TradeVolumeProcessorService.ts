import { Decimal } from 'decimal.js'
import _ from 'lodash'
import moment from 'moment'
import * as chainStore from '../repository/ChainRepository.js'
import * as priceStore from '../repository/PriceRepository.js'
import * as tradeStore from '../repository/TradeVolumeRepository.js'
import { TimestampedAmount, TimestampedAmountPool } from '../schema/Models.js'
import { BASE_TOKEN, parseNumber } from '../util/Chain.js'
import logger from '../util/Logger.js'
import { ZERO } from '../util/Misc.js'

export const initService = async () => {
  await processTrades()
}

const processTrades = async () => {
  const assets = await chainStore.getAssets()
  logger.info(`TradeVolumeService: have assets: ${assets}`)
  for (const asset of assets) {
    logger.info(`TradeVolumeService: processing asset ${asset}`)
    await processAsset(asset)
  }
}

const processAsset = async (asset: chainStore.Asset) => {
  let events = []
  let processed = 0
  do {
    const latest = await tradeStore.getLatest(asset)
    const to = await priceStore.getLatest(asset.id)
    events = await chainStore.getEvents(latest, to)
    logger.debug(
      `TradeVolumeService: processing assets ${asset} in ${latest}-${to} with ${events.length} events`
    )
    processed += await processEvents(asset, events)
  } while (events.length === chainStore.LIMIT)
  logger.info(
    `TradeVolumeService: processed asset ${asset} with ${processed} trades`
  )
}

const processEvents = async (
  asset: chainStore.Asset,
  events: chainStore.EventEntry[]
) => {
  if (events.length === 0) {
    return 0
  }
  const ids = asset.pool
  const swaps = events
    .map((ev) => [ev.timestamp, getSwaps(ev, asset)] as [number, SwapData])
    .filter(([tsp, amounts]) => !(amounts[0].eq(ZERO) || amounts[1].eq(ZERO)))

  const [prices_0, prices_1] = await getPrices(
    ids,
    swaps.map(([tsp]) => tsp)
  )
  if (prices_0.size === 0 || prices_1.size === 0) {
    await tradeStore.save(asset, [], _.last(events).timestamp)
    logger.debug(
      `TradeVolumeService: zero prices for ${ids} with ${swaps.length} swaps`
    )
    return 0
  }
  logger.debug(
    `TradeVolumeService: has ${prices_0.size},${prices_1.size} prices for ${ids} with ${swaps.length} swaps`
  )

  const amounts: TimestampedAmountPool[] = swaps
    .map(([tsp, swapData]) => {
      const price_0 = getPrice(asset.pool[0], tsp, prices_0)
      const price_1 = getPrice(asset.pool[1], tsp, prices_1)
      return [
        tsp,
        [swapData[0].mul(price_0), swapData[1].mul(price_1)],
      ] as TimestampedAmountPool
    })
    .filter(([tsp, amounts]) => !(amounts[0].eq(ZERO) || amounts[1].eq(ZERO)))

  await tradeStore.save(asset, amounts, _.last(events).timestamp)

  logger.debug(
    `TradeVolumeService: processed ${events.length} events with ${amounts.length} trades`
  )
  return amounts.length
}

const getSwaps = (
  event: chainStore.EventEntry,
  asset: chainStore.Asset
): SwapData => {
  // filter swap events and multiswap summary event where data contains more then two ids
  // for mutiswaps we consider partial swap events instead of summaried one
  return event.events
    .flat()
    .filter(
      (e) =>
        e.section === 'xyk' &&
        e.method === 'AssetsSwapped' &&
        (!_.isArray(e.data[1]) ||
          (_.isArray(e.data[1]) && e.data[1].length === 2))
    )
    .map((e) => {
      const ids = _.isArray(e.data[1])
        ? [
            Number.parseInt(_.first(e.data[1])),
            Number.parseInt(_.last(e.data[1])),
          ]
        : [Number.parseInt(e.data[1]), Number.parseInt(e.data[3])]
      const amounts = _.isArray(e.data[1])
        ? [
            new Decimal(parseNumber(e.data[2])),
            new Decimal(parseNumber(e.data[3])),
          ]
        : [
            new Decimal(parseNumber(e.data[2])),
            new Decimal(parseNumber(e.data[4])),
          ]
      return {
        pool: ids,
        amount: amounts,
      }
    })
    .map((e) => {
      if (_.isEqual(e.pool, asset.pool)) {
        return e.amount as SwapData
      } else if (_.isEqual(e.pool, [asset.pool[1], asset.pool[0]])) {
        return [e.amount[1], e.amount[0]] as SwapData
      } else {
        return [ZERO, ZERO] as SwapData
      }
    })
    .reduce((acc, ev) => [acc[0].add(ev[0]), acc[1].add(ev[1])], [ZERO, ZERO])
}

const getPrices = async (ids: [number, number], timestamps: number[]) => {
  const prices: TimestampedAmount[][] = []
  for (const asset of ids) {
    const price = []
    for (const tsp of timestamps) {
      if (asset === BASE_TOKEN) {
        const from = moment.utc(tsp).startOf('day').valueOf()
        if (_.last(price)?.[0] !== from) {
          price.push(await priceStore.get(asset, from, from))
        }
      } else {
        price.push(await priceStore.get(asset, tsp, tsp))
      }
    }
    prices.push(price.flat())
  }
  return [new Map(prices[0]), new Map(prices[1])]
}

const getPrice = (
  id: number,
  timestamp: number,
  prices: Map<number, Decimal>
) => {
  let key = timestamp
  if (id === BASE_TOKEN) {
    key = _.findLast([...prices.keys()], (tsp) => timestamp >= tsp)
  }
  return prices.has(key) ? prices.get(key) : ZERO
}

type SwapData = [Decimal, Decimal]
