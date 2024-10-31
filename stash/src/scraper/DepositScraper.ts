import { Block, Event } from './BlockScraper'
import _ from 'lodash'
import { depositRepository } from '../repository/TransactionRepository.js'
import { ApiPromise } from '@polkadot/api'
import logger from '../util/Logger.js'

export const processFerriedDepositEvents = async (
  api: ApiPromise,
  block: Block
) => {
  const events = _.chain(block.events)
    .filter((ev) => filterEvents(ev[1]))
    .groupBy(([idx, _]) => idx)
    .map((evs, _) => evs.map(([_, ev]) => ev))
    .value()
  if (events.length > 0) {
    for (const eventGroup of events) {
      for (const event of eventGroup) {
        const ferriedDepositData = await processFerriedDeposit(api, event.data)
        logger.info('Ferried deposit processed', ferriedDepositData)
      }
    }
  }
}

export const processFerriedDeposit = async (
  api: ApiPromise,
  eventData: any
): Promise<object> => {
  const transactionsToProcess = await depositRepository
    .search()
    .where('chain')
    .equals(eventData.chain)
    .and('type')
    .equals('deposit')
    .and('requestId')
    .equals(Number(eventData.deposit.requestId.id))
    .returnFirst()
  if (transactionsToProcess) {
    transactionsToProcess.status = 'Processed'
    const timestamp = new Date().toISOString()
    transactionsToProcess.updated = Date.parse(timestamp)
    transactionsToProcess.closedBy = 'ferry'
    return await depositRepository.save(transactionsToProcess)
  }
}

const filterEvents = (ev: Event) => {
  return ev.section === 'rolldown' && ev.method === 'DepositFerried'
}
