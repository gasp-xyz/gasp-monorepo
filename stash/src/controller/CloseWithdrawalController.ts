import { Request, Response } from 'express'

import * as errorHandler from '../error/Handler.js'
import { closeWithdrawalSchema } from '../schema/CloseWithdrawalSchema.js'
import { closeWithdrawal } from '../service/CloseWithdrawalService.js'
import logger from '../util/Logger.js'

export const closeWithdrawalEndpoint = async (req: Request, res: Response) => {
  /*
   #swagger.tags = ['Withdrawal']
   #swagger.summary = 'Close a withdrawal by calling Rolldown contract.'
   #swagger.description = "Close a withdrawal by providing transaction hash. Stash will fetch withdrawal data and call close_withdrawal on Rolldown contract."
   #swagger.parameters['txHash'] = {
     in: 'path',
     description: 'Transaction hash of the withdrawal',
     required: true,
     type: 'string'
   }
   #swagger.responses[200] = {
      description: 'Successful response with transaction hash'
   }
   #swagger.responses[400] = {
      description: 'Validation error'
   }
   #swagger.responses[404] = {
      description: 'Withdrawal not found'
   }
   #swagger.responses[500] = {
      description: 'Internal Server Error'
   }
  */
  const { txHash } = req.params
  
  logger.info(`Received close withdrawal request for txHash: ${txHash}`)
  
  try {
    logger.info(`Validating request parameters for txHash: ${txHash}`)
    closeWithdrawalSchema.validateSync({ txHash })
    
    logger.info(`Starting withdrawal closure process for txHash: ${txHash}`)
    const result = await closeWithdrawal(txHash)
    
    logger.info(`Withdrawal closure completed successfully for txHash: ${txHash}, contractTx: ${result.txHash}`)
    
    return res.json({ 
      success: true,
      txHash: result.txHash,
      message: 'Withdrawal closed successfully'
    })
  } catch (e) {
    logger.error(`Close withdrawal request failed for txHash: ${txHash}: ${e.message}`, {
      error: e.stack,
      txHash
    })
    await errorHandler.handle(res, e)
  }
}