import { Request, Response } from 'express'

import * as errorHandler from '../error/Handler.js'

import {
  bodyHistorySchema,
  pathCurrencySchema,
  pathPairCurrencySchema,
} from '../schema/PriceDiscoverySchema.js'
import * as priceDiscoveryService from '../service/PriceDiscoveryService.js'

export const getPrice = async (req: Request, res: Response): Promise<void> => {
  /*       #swagger.tags = ['Price Discovery']
           #swagger.summary = 'Get the price of the token on Managata DEX.'
           #swagger.description = "The price will be returned in all tokens including USD."
           #swagger.parameters['currencySymbol'] = {
              in: 'path',
              description: 'which token or pool to get',
              type: 'string'
           }
           #swagger.responses[200] = {
               description: 'Successful response',
               content: {
                   'application/json': {
                       schema: {
                           type: 'object',
                           properties: {
                              current_price: {
                                type: 'object',
                                properties: {
                                  "usd": { type: 'string' }
                                }
                              }
                           }
                       }
                   }
               }
           }
        */
  try {
    const id = pathCurrencySchema.validateSync(req.params)

    const response = await priceDiscoveryService.priceDiscovery(
      id.currencySymbol
    )

    res.json(response)
  } catch (e) {
    await errorHandler.handle(res, e)
  }
}

export const getHistoryPair = async (
  req: Request,
  res: Response
): Promise<void> => {
  /*
   #swagger.tags = ['Price History Pair']
   #swagger.summary = 'Get the price history of the token pair.'
   #swagger.description = "The price of base asset will be returned in in the target asset."
   #swagger.parameters['baseCurrencySymbol'] = {
      in: 'path',
      description: 'which token to get as a base',
      type: 'string'
   }
   #swagger.parameters['targetCurrencySymbol'] = {
      in: 'path',
      description: 'which token to get as target',
      type: 'string'
   }
   #swagger.parameters['days'] = {
      in: 'query',
      description: 'how many days into history to fetch, number or max, default is max',
      type: 'string'
   }
   #swagger.parameters['interval'] = {
      in: 'query',
      type: 'string'
   }
   #swagger.responses[200] = {
       description: 'Successful response',
       content: {
           'application/json': {
               example: {
                  "prices": [
                    [
                        1685577600000,
                        "0.000825322989162406"
                    ],
                    [
                        1685664000000,
                        "0.0008134241430865131"
                    ]
                  ]
               }
           }
       }
   }
  */
  try {
    const ids = pathPairCurrencySchema.validateSync(req.params)
    const args = bodyHistorySchema.validateSync(req.query)

    const response = await priceDiscoveryService.priceHistoryPair(
      ids.baseCurrencySymbol,
      ids.targetCurrencySymbol,
      args.days,
      args.interval
    )

    res.json(response)
  } catch (e) {
    await errorHandler.handle(res, e)
  }
}

export const getHistory = async (
  req: Request,
  res: Response
): Promise<void> => {
  /* #swagger.tags = ['Price History']
   #swagger.summary = 'Get the price history of the token.'
   #swagger.description = "The price will be returned in USD."
   #swagger.parameters['currencySymbol'] = {
      in: 'path',
      description: 'which token or pool to get',
      type: 'string'
   }
   #swagger.parameters['days'] = {
      in: 'query',
      description: 'how many days into history to fetch, number or max, default is max',
      type: 'string'
   }
   #swagger.parameters['interval'] = {
      in: 'query',
      type: 'string'
   }
   #swagger.responses[200] = {
       description: 'Successful response',
       content: {
           'application/json': {
               example: {
                  "prices": [
                    [
                        1685577600000,
                        "0.000825322989162406"
                    ],
                    [
                        1685664000000,
                        "0.0008134241430865131"
                    ]
                  ]
               }
           }
       }
   }
*/

  try {
    const id = pathCurrencySchema.validateSync(req.params)
    const args = bodyHistorySchema.validateSync(req.query)

    const response = await priceDiscoveryService.priceHistory(
      id.currencySymbol,
      args.days,
      args.interval
    )

    res.json(response)
  } catch (e) {
    await errorHandler.handle(res, e)
  }
}

export const getVolumeAsset = async (
  req: Request,
  res: Response
): Promise<void> => {
  /*       #swagger.tags = ['TVL History']
         #swagger.summary = 'Get the trade volume history of the token.'
         #swagger.description = "The price will be returned in USD."
         #swagger.parameters['currencySymbol'] = {
            in: 'path',
            description: 'which token or pool to get',
            type: 'string'
         }
         #swagger.parameters['days'] = {
            in: 'query',
            description: 'how many days into history to fetch, number or max, default is max',
            type: 'string'
         }
         #swagger.parameters['interval'] = {
            in: 'query',
            type: 'string'
         }
         #swagger.responses[200] = {
           description: 'Successful response',
           content: {
               'application/json': {
                   example: {
                      "volumes": [
                        [
                            1685577600000,
                            "991.5151718402558"
                        ],
                        [
                            1685664000000,
                            "3094.527263206033"
                        ]
                      ]
                   }
               }
           }
 }
      */
  return getVolume(req, res, false)
}
export const getVolumePool = async (
  req: Request,
  res: Response
): Promise<void> => {
  /*       #swagger.tags = ['TVL History']
         #swagger.summary = 'Get the TVL history of the pool.'
         #swagger.description = "The price will be returned in USD."
         #swagger.parameters['currencySymbol'] = {
            in: 'path',
            description: 'which token or pool to get',
            type: 'string'
         }
         #swagger.parameters['days'] = {
            in: 'query',
            description: 'how many days into history to fetch, number or max, default is max',
            type: 'string'
         }
         #swagger.parameters['interval'] = {
            in: 'query',
            type: 'string'
         }
         #swagger.responses[200] = {
           description: 'Successful response',
           content: {
               'application/json': {
                   example: {
                      "volumes": [
                        [
                            1685577600000,
                            "991.5151718402558"
                        ],
                        [
                            1685664000000,
                            "3094.527263206033"
                        ]
                      ]
                   }
               }
           }
 }
      */
  return getVolume(req, res, true)
}
const getVolume = async (
  req: Request,
  res: Response,
  isPool: boolean
): Promise<void> => {
  try {
    const id = pathCurrencySchema.validateSync(req.params)
    const args = bodyHistorySchema.validateSync(req.query)

    const response = await priceDiscoveryService.volumeHistory(
      id.currencySymbol,
      isPool,
      args.days,
      args.interval
    )

    res.json(response)
  } catch (e) {
    await errorHandler.handle(res, e)
  }
}

export const getTradesAsset = async (
  req: Request,
  res: Response
): Promise<void> => {
  /*       #swagger.tags = ['Volume History']
          #swagger.summary = 'Get the trade volume history of the token.'
          #swagger.description = "The price will be returned in USD."
          #swagger.parameters['currencySymbol'] = {
             in: 'path',
             description: 'which token or pool to get',
             type: 'string'
          }
          #swagger.parameters['days'] = {
             in: 'query',
             description: 'how many days into history to fetch, number or max, default is max',
             type: 'string'
          }
          #swagger.parameters['interval'] = {
             in: 'query',
             type: 'string'
          }
          #swagger.responses[200] = {
            description: 'Successful response',
            content: {
                'application/json': {
                    example: {
                       "volumes": [
                         [
                             1685577600000,
                             "991.5151718402558"
                         ],
                         [
                             1685664000000,
                             "3094.527263206033"
                         ]
                       ]
                    }
                }
            }
  }
       */
  return getTrades(req, res, false)
}
export const getTradesPool = async (
  req: Request,
  res: Response
): Promise<void> => {
  /*       #swagger.tags = ['Volume History']
          #swagger.summary = 'Get the trade volume history of the pool. '
          #swagger.description = "The price will be returned in USD."
          #swagger.parameters['currencySymbol'] = {
             in: 'path',
             description: 'which token or pool to get',
             type: 'string'
          }
          #swagger.parameters['days'] = {
             in: 'query',
             description: 'how many days into history to fetch, number or max, default is max',
             type: 'string'
          }
          #swagger.parameters['interval'] = {
             in: 'query',
             type: 'string'
          }
          #swagger.responses[200] = {
            description: 'Successful response',
            content: {
                'application/json': {
                    example: {
                       "volumes": [
                         [
                             1685577600000,
                             "991.5151718402558"
                         ],
                         [
                             1685664000000,
                             "3094.527263206033"
                         ]
                       ]
                    }
                }
            }
  }
       */
  return getTrades(req, res, true)
}
const getTrades = async (
  req: Request,
  res: Response,
  isPool: boolean
): Promise<void> => {
  try {
    const id = pathCurrencySchema.validateSync(req.params)
    const args = bodyHistorySchema.validateSync(req.query)

    const response = await priceDiscoveryService.tradesHistory(
      id.currencySymbol,
      isPool,
      args.days,
      args.interval
    )

    res.json(response)
  } catch (e) {
    await errorHandler.handle(res, e)
  }
}
