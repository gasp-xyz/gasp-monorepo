import { Request, Response } from 'express'
import {
  findTransactionsByAddressAndStatus,
  getStatusByTxHashOrEntityId,
  getTransactionByEntityId,
  getTransactionsByAddress,
  startTracingTransaction,
} from '../service/TracingService.js'
import {
  getAllTransactionsByAddressAndStatusSchema,
  getAllTransactionsByAddressSchema,
  getStatusByTxHashOrEntityIdSchema,
  getTransactionByEntityIdSchema,
  startTracingSchema,
} from '../schema/TracingSchema.js'
import * as errorHandler from '../error/Handler.js'

export const startTracing = async (req: Request, res: Response) => {
  /*
   #swagger.tags = ['Tracing']
   #swagger.summary = 'Start tracing a transaction.'
   #swagger.description = "Start tracing a transaction by providing transaction details."
   #swagger.parameters['body'] = {
     in: 'body',
     description: 'Transaction details',
     required: true,
     schema: {
       txHash: 'string',
       address: 'string',
       type: 'string',
       chain: 'string',
       amount: 'number',
       asset_chainId: 'string',
       asset_address: 'string'
     }
   }
   #swagger.responses[200] = {
      description: 'Successful response with transaction details'
   }
   #swagger.responses[400] = {
      description: 'Validation error'
   }
   #swagger.responses[500] = {
      description: 'Internal Server Error'
   }
  */
  const { txHash, address, type, chain, amount, asset_chainId, asset_address } =
    req.body
  try {
    startTracingSchema.validate({
      txHash,
      address,
      type,
      chain,
      amount,
      asset_chainId,
      asset_address,
    })
    const transaction = await startTracingTransaction({
      txHash,
      address,
      type,
      chain,
      amount,
      asset_chainId,
      asset_address,
    })
    return res.json({ transaction })
  } catch (e) {
    await errorHandler.handle(res, e)
  }
}

export const getTransactionStatusByTxHashOrEntityId = async (
  req: Request,
  res: Response
) => {
  /*
   #swagger.tags = ['Tracing']
   #swagger.summary = 'Get transaction status by txHash or entityId.'
   #swagger.description = "Get the status of a transaction by providing the transaction hash or entity ID and type of the transaction."
   #swagger.parameters['txHashOrEntityId'] = {
     in: 'path',
     description: 'Transaction hash or entity ID',
     required: true,
     type: 'string'
   }
   #swagger.parameters['type'] = {
    in: 'path',
    description: 'Type of the transaction',
    required: true,
    type: 'string',
    enum: ['deposit', 'withdrawal']
}
   #swagger.responses[200] = {
      description: 'Successful response with transaction status'
   }
   #swagger.responses[404] = {
      description: 'Transaction not found'
   }
   #swagger.responses[500] = {
      description: 'Internal Server Error'
   }
  */
  try {
    const { txHashOrEntityId, type } = req.params
    await getStatusByTxHashOrEntityIdSchema.validate({ txHashOrEntityId, type })
    const status = await getStatusByTxHashOrEntityId(txHashOrEntityId, type)
    if (status) {
      res.status(200).send({ status })
    } else {
      res.status(404).send({ error: 'Transaction not found' })
    }
  } catch (e) {
    await errorHandler.handle(res, e)
  }
}

export const getAllTransactionsByAddress = async (
  req: Request,
  res: Response
): Promise<void> => {
  /*
   #swagger.tags = ['Tracing']
   #swagger.summary = 'Get all transactions by address.'
   #swagger.description = "Get all transactions by providing a specific address and type of a transaction."
   #swagger.parameters['address'] = {
     in: 'path',
     description: 'Address to get transactions for',
     required: true,
     type: 'string'
   }
   #swagger.parameters['type'] = {
    in: 'path',
    description: 'Type of the transaction',
    required: true,
    type: 'string',
    enum: ['deposit', 'withdrawal']
}
   #swagger.responses[200] = {
      description: 'Successful response with transactions'
   }
   #swagger.responses[404] = {
      description: 'No transactions found for this address'
   }
   #swagger.responses[500] = {
      description: 'Internal Server Error'
   }
  */
  try {
    const { address, type } = req.params
    getAllTransactionsByAddressSchema.validate({ address, type })
    const transactions = await getTransactionsByAddress(address, type)
    if (transactions.length > 0) {
      res.status(200).send({ transactions })
    } else {
      res.status(404).send({ error: 'No transactions found for this address' })
    }
  } catch (e) {
    await errorHandler.handle(res, e)
  }
}

export const getAllTransactionsByAddressAndStatus = async (
  req: Request,
  res: Response
) => {
  /*
   #swagger.tags = ['Tracing']
   #swagger.summary = 'Get all transactions by address and status.'
#swagger.description = "Retrieve all transactions for a specific address and status by providing the address, status, and transaction type."
   #swagger.parameters['address'] = {
     in: 'path',
     description: 'Address to get transactions for',
     required: true,
     type: 'string'
   }
   #swagger.parameters['status'] = {
     in: 'path',
     description: 'Status of the transactions to filter by',
     required: true,
     type: 'string'
   }
   #swagger.parameters['type'] = {
    in: 'path',
    description: 'Type of the transaction',
    required: true,
    type: 'string',
    enum: ['deposit', 'withdrawal']
}
   #swagger.responses[200] = {
      description: 'Successful response with transactions'
   }
   #swagger.responses[500] = {
      description: 'Internal Server Error'
   }
  */
  try {
    const { address, status, type } = req.params
    getAllTransactionsByAddressAndStatusSchema.validate({
      address,
      status,
      type,
    })
    const transactions = await findTransactionsByAddressAndStatus(
      address,
      status,
      type
    )
    if (transactions.length > 0) {
      res.status(200).send({ transactions })
    } else {
      res.status(404).send({ error: 'No transactions found for this address' })
    }
  } catch (e) {
    await errorHandler.handle(res, e)
  }
}

export const getATransactionByEntityId = async (
  req: Request,
  res: Response
) => {
  /*
   #swagger.tags = ['Tracing']
   #swagger.summary = 'Get a transaction by entityId.'
   #swagger.description = "Get a specific transaction by providing the entity ID and type of a transaction."
   #swagger.parameters['entityId'] = {
     in: 'path',
     description: 'Entity ID of the transaction',
     required: true,
     type: 'string'
   }
   #swagger.parameters['type'] = {
    in: 'path',
    description: 'Type of the transaction',
    required: true,
    type: 'string',
    enum: ['deposit', 'withdrawal']
}
   #swagger.responses[200] = {
      description: 'Successful response with transaction details'
   }
   #swagger.responses[404] = {
      description: 'Transaction not found for this entityId'
   }
   #swagger.responses[500] = {
      description: 'Internal Server Error'
   }
  */
  try {
    const { entityId, type } = req.params
    getTransactionByEntityIdSchema.validateSync({ entityId, type })
    const transaction = await getTransactionByEntityId(entityId, type)
    if (transaction) {
      res.status(200).send({ transaction })
    } else {
      res.status(404).send({ error: 'Transaction not found for this entityId' })
    }
  } catch (e) {
    await errorHandler.handle(res, e)
  }
}
