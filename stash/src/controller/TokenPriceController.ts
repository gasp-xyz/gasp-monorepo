import { Request, Response } from 'express'

import * as errorHandler from '../error/Handler.js'
import * as tokenPriceService from '../service/TokenPriceService.js'

export const getTokenPrices = async (
  req: Request,
  res: Response,
): Promise<void> => {
  /*
    #swagger.tags = ['Token Prices']
    #swagger.summary = 'Get the price of a specific token.'
    #swagger.description = 'Retrieve the price of a token by providing its ID. Price is returned in USD.'
    #swagger.parameters['tokenId'] = {
        in: 'path',
        description: 'ID of the token to retrieve the price for',
        required: true,
        type: 'string'
    }
    #swagger.responses[200] = {
        description: 'Successful response',
        content: {
            'application/json': {
                example: {
                    tokenId: 'exampleTokenId',
                    price: 123.45
                }
            }
        }
    }
    #swagger.responses[404] = {
        description: 'Token price not found',
        content: {
            'application/json': {
                example: {
                    message: 'Token price not found'
                }
            }
        }
    }
    #swagger.responses[500] = {
        description: 'Internal server error',
        content: {
            'application/json': {
                example: {
                    message: 'An error occurred while retrieving the token price'
                }
            }
        }
    }
*/

  try {
    const tokenId = req.params.tokenId
    const price = await tokenPriceService.getTokenPrice(tokenId)
    if (price !== null && price !== undefined) {
      res.json({ tokenId: tokenId, price: price })
    } else {
      res.status(404).json({ message: 'Token price not found' })
    }
  } catch (e) {
    await errorHandler.handle(res, e)
  }
}
