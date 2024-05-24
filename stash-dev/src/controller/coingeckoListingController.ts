import { Request, Response } from 'express'
import * as errorHandler from '../error/Handler.js'
import * as coingeckoListingService from '../service/CoingeckoListingService.js'

export const pairs = async (_: Request, res: Response) => {
  /* #swagger.tags = ['coingecko']
           #swagger.summary = 'Details on cryptoassets traded on an exchange.'
           #swagger.description = "This endpoint retrieves information about trading pairs from Exchange"
           #swagger.responses[200] = {
               description: 'Successful response',
               content: {
                   'application/json': {
                       schema: {
                           type: 'array',
                           items: {
                               type: 'object',
                               properties: {
                                   ticker_id: { type: 'string' },
                                   base: { type: 'string' },
                                   target: { type: 'string' },
                                   pool_id: { type: 'string' }
                               }
                           }
                       },
                       example: [
                           {
                              "ticker_id": "KSM_MGX",
                              "base": "KSM",
                              "target": "MGX",
                              "pool_id": "5"
                           },
                           {
                              "ticker_id": "MGX_TUR",
                              "base": "MGX",
                              "target": "TUR",
                              "pool_id": "8"
                           }
                       ]
                   }
               }
           }
        */
  try {
    const pairs = await coingeckoListingService.pairs()
    res.json(pairs)
  } catch (e) {
    await errorHandler.handle(res, e)
  }
}

export const tickers = async (_: Request, res: Response) => {
  /*        #swagger.tags = ['coingecko']
            #swagger.summary = 'The endpoint provides 24-hour pricing and volume information on each market pair available on an exchange.'
            #swagger.description = "This endpoint retrieves information about pricing and volume information from Exchange"
            #swagger.responses[200] = {
               description: 'Successful response',
               content: {
                   'application/json': {
                       schema: {
                           type: 'array',
                           items: {
                               type: 'object',
                               properties: {
                                   ticker_id: { type: 'string' },
                                   base_currency: { type: 'string' },
                                   target_currency: { type: 'string' },
                                   last_price: { type: 'string' },
                                   base_volume: { type: 'string' },
                                   target_volume: { type: 'string' },
                                   pool_id: { type: 'string' },
                                   liquidity_in_usd: { type: 'string' }
                               }
                           }
                       },
                       example: [
                           {
                              "ticker_id": "KSM_MGX",
                              "base_currency": "KSM",
                              "target_currency": "MGX",
                              "last_price": "41344.060279360743591417",
                              "base_volume": "37.25709886091373313",
                              "target_volume": "1027054.0278098086651",
                              "pool_id": "5",
                              "liquidity_in_usd": "667688.4072373228"
                           },
                           {
                              "ticker_id": "MGX_TUR",
                              "base_currency": "MGX",
                              "target_currency": "TUR",
                              "last_price": "0.139133601",
                              "base_volume": "8813.7065083928515899",
                              "target_volume": "1205.6596913620300336",
                              "pool_id": "8",
                              "liquidity_in_usd": "85617.00427908385"
                           }
                       ]
                   }
               }
           }

*/

  try {
    const tickers = await coingeckoListingService.tickers()
    res.json(tickers)
  } catch (e) {
    await errorHandler.handle(res, e)
  }
}
