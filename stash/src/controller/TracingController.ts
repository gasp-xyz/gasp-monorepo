import { Request, Response } from 'express'
import {
  findTransactionsByAddressAndStatus,
  getByTxHashOrEntityId,
  getTransactionsByAddress,
  startTracingTransaction,
} from '../service/TracingService.js'
import {
  getAllTransactionsByAddressAndStatusSchema,
  getAllTransactionsByAddressSchema,
  getStatusByTxHashOrEntityIdSchema,
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
    startTracingSchema.validateSync({
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

export const getTransactionByTxHashOrEntityId = async (
  req: Request,
  res: Response
) => {
  /*
   #swagger.tags = ['Tracing']
   #swagger.summary = 'Get a transaction by entityId or transaction hash.'
   #swagger.description = "Get a transaction by providing the entity ID  or transaction hash and type of a transaction."
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
      description: 'Successful response with transaction details'
   }
   #swagger.responses[404] = {
      description: 'Transaction not found for this entityId or transaction hash'
   }
   #swagger.responses[500] = {
      description: 'Internal Server Error'
   }
  */
  try {
    const { txHashOrEntityId, type } = req.params
    await getStatusByTxHashOrEntityIdSchema.validateSync({
      txHashOrEntityId,
      type,
    })
    const transaction = await getByTxHashOrEntityId(
      txHashOrEntityId,
      type
    )
    if (transaction && Object.keys(transaction).length > 0) {
      res.status(200).send({ transaction })
    } else {
      res.status(404).send({
        error: 'Transaction not found for this entityId or transaction hash',
      })
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
   #swagger.summary = 'Get all frontend created transactions by address.'
   #swagger.description = "Get all frontend created transactions by providing a specific address and type of a transaction."
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
    getAllTransactionsByAddressSchema.validateSync({ address, type })
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
    getAllTransactionsByAddressAndStatusSchema.validateSync({
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
