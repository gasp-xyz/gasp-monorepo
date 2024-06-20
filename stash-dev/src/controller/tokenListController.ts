import { Request, Response } from 'express'
import * as tokenListService from '../service/TokenListService.js'
import * as errorHandler from '../error/Handler.js'
import { tokenSymbolSchema } from '../schema/PriceDiscoverySchema.js'

export const tokenList = async (_: Request, res: Response) => {
  /* #swagger.tags = ['Token List']
           #swagger.summary = 'List of Tokens listed on Mangata with their stats'
           #swagger.description = "This endpoint retrieves information about tokens listed on Mangata with their stats"
           #swagger.responses[200] = {
               description: 'Successful response',
               content: {
                   'application/json': {
                       schema: {
                           type: 'array',
                           items: {
                               type: 'object',
                               properties: {
                                   symbol: { type: 'string' },
                                   priceInUSD: { type: 'string' },
                                   volume24hInUSD: { type: 'string' },
                                   liquidity24hInUSD: { type: 'string' },
                                   priceChange24hInPerc: { type: 'string' },
                                   volumeChange24hInPerc: { type: 'string' }
                               }
                           }
                       },
                       example: [
                           {
                              "symbol": "KSM",
                              "priceInUSD": "19.17",
                              "volume24hInUSD": "1000.23",
                              "liquidity24hInUSD": "100000.567",
                              "priceChange24hInPerc": "2.22",
                              "volumeChange24hInPerc": "3.33"
                           },
                           {
                              "symbol": "MGX",
                              "priceInUSD": "0.00045776",
                              "volume24hInUSD": "1000.23",
                              "liquidity24hInUSD": "1000.567",
                              "priceChange24hInPerc": "0.22",
                              "volumeChange24hInPerc": "1.33"
                           }
                       ]
                   }
               }
           }
        */
  try {
    const tokenList = await tokenListService.tokenList()
    res.json(tokenList)
  } catch (e) {
    await errorHandler.handle(res, e)
  }
}

export const tokenDetails = async (req: Request, res: Response) => {
  /* #swagger.tags = ['Token List']
           #swagger.summary = 'Detail of token listed on Mangata with its stats'
           #swagger.description = "This endpoint retrieves detail information about token listed on Mangata with its stats"
           #swagger.responses[200] = {
               description: 'Successful response',
               content: {
                   'application/json': {
                       schema: {
                           type: 'object',
                               properties: {
                                   symbol: { type: 'string' },
                                   priceInUSD: { type: 'string' },
                                   volume24hInUSD: { type: 'string' },
                                   liquidity24hInUSD: { type: 'string' },
                                   priceChange24hInPerc: { type: 'string' },
                                   volumeChange24hInPerc: { type: 'string' }
                               }
                       },
                       example: {
                              "symbol": "KSM",
                              "priceInUSD": "19.17",
                              "volume24hInUSD": "1000.23",
                              "liquidity24hInUSD": "100000.567",
                              "priceChange24hInPerc": "2.22",
                              "volumeChange24hInPerc": "3.33"
                           }
                   }
               }
           }
        */
  try {
    const { symbol } = tokenSymbolSchema.validateSync(req.params)
    const tokenDetails = await tokenListService.tokenDetails(symbol)
    res.json(tokenDetails)
  } catch (e) {
    await errorHandler.handle(res, e)
  }
}
