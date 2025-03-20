import MangataClient from '../connector/MangataNode.js'
import {
  watchDepositAcceptedIntoQueue,
  processRequests,
  watchWithdrawalClosed,
} from '../scraper/L1LogScraper.js'
import logger from '../util/Logger.js'
import { CONFIG_TO_CHAIN } from '../util/ConfigToChain.js'
const ETH_CHAIN = 'Ethereum'
const ARB_CHAIN = 'Arbitrum'
const BASE_CHAIN = 'Base'
const MONAD_CHAIN = 'Monad'
const MEGAETH_CHAIN = 'Megaeth'
export const initService = async () => {
  const api = await MangataClient.api()

  Promise.allSettled([
    watchDepositAcceptedIntoQueue(
      api,
      process.env.ETH_CHAIN_URL,
      CONFIG_TO_CHAIN.get(process.env.ENVIRONMENT + '-ethereum'),
      ETH_CHAIN,
      process.env.CONTRACT_ADDRESS_ETH
    ),
    watchDepositAcceptedIntoQueue(
      api,
      process.env.ARBITRUM_SEPOLIA_CHAIN_URL,
      CONFIG_TO_CHAIN.get(process.env.ENVIRONMENT + '-arbitrum'),
      ARB_CHAIN,
      process.env.CONTRACT_ADDRESS_ARB
    ),
    watchDepositAcceptedIntoQueue(
      api,
      process.env.BASE_CHAIN_URL,
      CONFIG_TO_CHAIN.get(process.env.ENVIRONMENT + '-base'),
      BASE_CHAIN,
      process.env.CONTRACT_ADDRESS_BASE
    ),
    watchDepositAcceptedIntoQueue(
      api,
      process.env.MONAD_CHAIN_URL,
      CONFIG_TO_CHAIN.get(process.env.ENVIRONMENT + '-monad'),
      MONAD_CHAIN,
      process.env.CONTRACT_ADDRESS_MONAD
    ),
    new Promise((resolve) => {
      setTimeout(() => {
        watchWithdrawalClosed(
          api,
          process.env.ETH_CHAIN_URL,
          CONFIG_TO_CHAIN.get(process.env.ENVIRONMENT + '-ethereum'),
          ETH_CHAIN,
          process.env.CONTRACT_ADDRESS_ETH
        ).then(resolve)
      }, 10000) // Delay of 10000 milliseconds (10 seconds) to allow past withdrawals to be started and confirmed first
    }),
    new Promise((resolve) => {
      setTimeout(() => {
        watchWithdrawalClosed(
          api,
          process.env.ARBITRUM_SEPOLIA_CHAIN_URL,
          CONFIG_TO_CHAIN.get(process.env.ENVIRONMENT + '-arbitrum'),
          ARB_CHAIN,
          process.env.CONTRACT_ADDRESS_ARB
        ).then(resolve)
      }, 10000)
    }),
    new Promise((resolve) => {
      setTimeout(() => {
        watchWithdrawalClosed(
          api,
          process.env.BASE_CHAIN_URL,
          CONFIG_TO_CHAIN.get(process.env.ENVIRONMENT + '-base'),
          BASE_CHAIN,
          process.env.CONTRACT_ADDRESS_BASE
        ).then(resolve)
      }, 10000)
    }),
    new Promise((resolve) => {
      setTimeout(() => {
        watchWithdrawalClosed(
          api,
          process.env.MONAD_CHAIN_URL,
          CONFIG_TO_CHAIN.get(process.env.ENVIRONMENT + '-monad'),
          MONAD_CHAIN,
          process.env.CONTRACT_ADDRESS_MONAD
        ).then(resolve)
      }, 10000)
    }),
    processRequests(api, 'Arbitrum'),
    processRequests(api, 'Ethereum'),
    processRequests(api, 'Base'),
    processRequests(api, 'Monad'),
  ]).then((results) => {
    results.forEach((result) => {
      if (result.status === 'fulfilled') {
        logger.log('Promise fulfilled:', result.value)
      } else {
        logger.error('Promise rejected:', result.reason)
      }
    })
  })
}
