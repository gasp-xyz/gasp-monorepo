import { Request, Response } from 'express'
import * as errorHandler from '../error/Handler.js'
import * as coinmarketcapListingService from '../service/CoinmarketcapListingService.js'

export const summary = async (_: Request, res: Response) => {
  /*        #swagger.tags = ['Coinmarketcap']
            #swagger.summary = 'The endpoint provides summary information on each market pair available on an exchange.'
            #swagger.description = "This endpoint retrieves information about each market pair information from Exchange"
            #swagger.responses[200] = {
               description: 'Successful response',
               content: {
                   'application/json': {
                       example: {
                            "KSM_MGX": {
                              "base_id": "4",
                              "base_name": "Kusama Native",
                              "base_symbol": "KSM",
                              "quote_id": "0",
                              "quote_name": "Mangata",
                              "quote_symbol": "MGX",
                              "last_price": "41325.990830608673576993",
                              "base_volume": "4.2321231321",
                              "quote_volume": "4111135.2773368904695"
                           }
                       }
                   }
               }
           }

*/
  try {
    const summary = await coinmarketcapListingService.summary()
    res.json(summary)
  } catch (e) {
    await errorHandler.handle(res, e)
  }
}
