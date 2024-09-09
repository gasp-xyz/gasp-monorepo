import MangataClient from '../connector/MangataNode.js'
import {
  watchDepositAcceptedIntoQueue,
  processRequests,
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
      ETH_CHAIN
    ),
    watchDepositAcceptedIntoQueue(
      api,
      process.env.ARBITRUM_SEPOLIA_CHAIN_URL,
      arbitrumSepolia,
      ARB_CHAIN
    ),
    processRequests(api, 'Arbitrum'),
    processRequests(api, 'Ethereum'),
  ]).then((results) => {
    results.forEach((result) => {
      if (result.status === 'fulfilled') {
        console.log('Promise fulfilled:', result.value)
      } else {
        console.error('Promise rejected:', result.reason)
      }
    })
  })
}
