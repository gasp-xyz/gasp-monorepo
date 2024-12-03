import MangataClient from '../connector/MangataNode.js'
import {
  watchDepositAcceptedIntoQueue,
  processRequests,
  watchWithdrawalClosed,
} from '../scraper/L1LogScraper.js'
import logger from '../util/Logger.js'
import { holesky, arbitrumSepolia } from 'viem/chains'
const ETH_CHAIN = 'Ethereum'
const ARB_CHAIN = 'Arbitrum'
export const initService = async () => {
  const api = await MangataClient.api()

  Promise.allSettled([
    watchDepositAcceptedIntoQueue(
      api,
      process.env.ETH_CHAIN_URL,
      holesky,
      ETH_CHAIN,
      process.env.CONTRACT_ADDRESS_ETH
    ),
    watchDepositAcceptedIntoQueue(
      api,
      process.env.ARBITRUM_SEPOLIA_CHAIN_URL,
      arbitrumSepolia,
      ARB_CHAIN,
      process.env.CONTRACT_ADDRESS_ARB
    ),
    new Promise((resolve) => {
      setTimeout(() => {
        watchWithdrawalClosed(
          api,
          process.env.ETH_CHAIN_URL,
          holesky,
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
          arbitrumSepolia,
          ARB_CHAIN,
          process.env.CONTRACT_ADDRESS_ARB
        ).then(resolve)
      }, 10000)
    }),
    processRequests(api, 'Arbitrum'),
    processRequests(api, 'Ethereum'),
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
