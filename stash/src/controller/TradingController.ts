import { Request, Response } from 'express'
import { getDataByWallet } from '../service/TradingService.js'
import { getDataByWalletSchema } from '../schema/TradingSchema.js'
import * as errorHandler from '../error/Handler.js'

export const getData = async (req: Request, res: Response): Promise<void> => {
  /*
   #swagger.tags = ['Trading']
   #swagger.summary = 'Get dashboard data.'
   #swagger.description = "Retrieve the dashboard data."
   #swagger.parameters['wallet'] = {
     in: 'path',
     description: 'Wallet to get dashboard data for',
     required: true,
     type: 'string'
   }
   #swagger.responses[200] = {
      description: 'Successful response with dashboard data'
   }
   #swagger.responses[500] = {
      description: 'Internal Server Error'
   }
  */
  try {
    const { wallet } = req.params
    getDataByWalletSchema.validateSync({ wallet })
    const data = (await getDataByWallet(wallet)) || {}
    res.json(data)
  } catch (e) {
    await errorHandler.handle(res, e)
  }
}
