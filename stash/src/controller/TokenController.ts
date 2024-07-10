import { Request, Response } from 'express'

import * as errorHandler from '../error/Handler.js'
import * as tokenService from '../service/TokenService.js'

export const listTokenOrderBuckets = async (
  req: Request,
  res: Response
): Promise<void> => {
  /*
          #swagger.tags = ['Token Order Buckets']
          #swagger.summary = 'Get the semantic order configuration of listed tokens on Mangata DEX.'
          #swagger.description = "Get the semantic order configuration of listed tokens on Mangata DEX. Result is ordered by bucket rank specified in the configuration."
          #swagger.responses[200] = {
             description: 'Successful response',
             content: {
                 'application/json': {
                     example: {
                        "buckets": [
                          { "bucket": "stables", "tokens": ["USDT", "USDC", "aUSD"] },
                          { "bucket": "bluechips", "tokens": ["BTC", "ETH"] },
                          { "bucket": "l0", "tokens": ["DOT", "KSM"] },
                          { "bucket": "dextoken", "tokens": ["MGA", "MGX"] },
                          { "bucket": "l1", "tokens": ["MOVR", "BNC", "OAK", "TUR", "IMBU"] },
                          { "bucket": "l2", "tokens": ["MATIC"] },
                          { "bucket": "protocols", "tokens": ["Solar"] },
                          { "bucket": "derivatives", "tokens": ["vKSM", "vsKSM", "vMOVR", "vBNC"] },
                        ]
                     }
                 }
             }
          }
      */
  try {
    const tokenOrderBuckets = await tokenService.listTokenOrderBuckets()

    res.json({ buckets: tokenOrderBuckets })
  } catch (e) {
    await errorHandler.handle(res, e)
  }
}
