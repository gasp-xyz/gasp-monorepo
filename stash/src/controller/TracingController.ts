import { Request, Response } from 'express'
import {
  findTransactionsByAddressAndStatus,
  getStatusByTxHashOrEntityId,
  getTransactionByEntityId,
  getTransactionByTxHash,
  getTransactionsByAddress,
  startTracingTransaction,
} from '../service/TracingService.js'
import {
  getAllTransactionsByAddressAndStatusSchema,
  getAllTransactionsByAddressSchema,
  getStatusByTxHashOrEntityIdSchema,
  getTransactionByEntityIdSchema,
  getTransactionByTxHashSchema,
  startTracingSchema,
} from '../schema/TracingSchema.js'
import * as errorHandler from '../error/Handler.js'

export const startTracing = async (
  body: {
    txHash: string
    address: string
    type: string
    amount: number
    asset_chainId: string
    asset_address: string
  },
  res: Response
): Promise<object> => {
  const { txHash, address, type, amount, asset_chainId, asset_address } = body
  try {
    await startTracingSchema.validate({
      txHash,
      address,
      type,
      amount,
      asset_chainId,
      asset_address,
    })
    const transaction = await startTracingTransaction({
      txHash,
      address,
      type,
      amount,
      asset_chainId,
      asset_address,
    })
    console.log('object from startTracingTransaction', transaction)
    return res.json({ transaction })
  } catch (e) {
    await errorHandler.handle(res, e)
  }
}

export const getTransactionStatusByTxHashOrEntityId = async (
  req: Request,
  res: Response
): Promise<void> => {
  try {
    const { txHashOrEntityId } = req.params
    await getStatusByTxHashOrEntityIdSchema.validate({ txHashOrEntityId })
    const status = await getStatusByTxHashOrEntityId(txHashOrEntityId)
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
  const { address } = req.params
  await getAllTransactionsByAddressSchema.validate({ address })
  try {
    const transactions = await getTransactionsByAddress(address)
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
): Promise<void> => {
  const { address, status } = req.params
  console.log('req.params', req.params)
  try {
    await getAllTransactionsByAddressAndStatusSchema.validate({
      address,
      status,
    })
    const transactions = await findTransactionsByAddressAndStatus(
      address,
      status
    )
    res.status(200).json(transactions)
  } catch (e) {
    await errorHandler.handle(res, e)
  }
}

export const getATransactionByEntityId = async (
  req: Request,
  res: Response
): Promise<void> => {
  const { entityId } = req.params
  await getTransactionByEntityIdSchema.validate({ entityId })
  try {
    const transaction = await getTransactionByEntityId(entityId)
    if (transaction) {
      res.status(200).send({ transaction })
    } else {
      res.status(404).send({ error: 'Transaction not found for this entityId' })
    }
  } catch (e) {
    await errorHandler.handle(res, e)
  }
}

export const getATransactionByTxHash = async (
  req: Request,
  res: Response
): Promise<void> => {
  const { txHash } = req.params
  await getTransactionByTxHashSchema.validate({ txHash })
  try {
    const transaction = await getTransactionByTxHash(txHash)
    if (transaction) {
      res.status(200).send({ transaction })
    } else {
      res.status(404).send({ error: 'Transaction not found for this txHash' })
    }
  } catch (e) {
    await errorHandler.handle(res, e)
  }
}
