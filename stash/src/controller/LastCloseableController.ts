import { Request, Response } from 'express'

import * as errorHandler from '../error/Handler.js'
import { getLastCloseableRequestIds } from '../service/LastCloseableService.js'

export const lastCloseable = async (req: Request, res: Response) => {
  /*
   #swagger.tags = ['Last Closeable']
   #swagger.summary = 'Get last closeable request IDs for all networks.'
   #swagger.description = "Retrieve the last processed L2 update request IDs for all supported networks."
   #swagger.responses[200] = {
      description: 'Successful response with last closeable request IDs',
      schema: {
        type: 'array',
        items: {
          type: 'object',
          properties: {
            network: { type: 'string', example: 'Ethereum' },
            lastRequestId: { type: 'number', example: 12345 }
          }
        }
      }
   }
   #swagger.responses[500] = {
      description: 'Internal Server Error'
   }
  */
  try {
    const result = await getLastCloseableRequestIds()
    res.json(result)
  } catch (error) {
    errorHandler.handle(res, error)
  }
}