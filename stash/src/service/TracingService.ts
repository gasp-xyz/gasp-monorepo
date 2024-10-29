import logger from '../util/Logger.js'
import { depositRepository } from '../repository/TransactionRepository.js'

interface TraceTransactionRequest {
  txHash: string
  address: string
  amount: string
  asset_address: string
  type: string
  chain: string
  asset_chainId: string
}

export const startTracingTransaction = async (
  traceRequest: TraceTransactionRequest
): Promise<object> => {
  const { txHash, address, type, chain, amount, asset_chainId, asset_address } =
    traceRequest
  const timestamp = new Date().toISOString()
  let status: string
  if (type === 'deposit') {
    status = 'PendingOnL1'
  } else if (type === 'withdrawal') {
    status = 'L2_INITIATED'
  } else {
    status = 'UNKNOWN'
  }

  const transactionData = {
    requestId: null, //requestId we get from L1, empty when we start tracing
    txHash,
    address,
    created: Date.parse(timestamp),
    updated: Date.parse(timestamp),
    status,
    type,
    chain,
    amount,
    asset_chainId,
    asset_address,
    closedBy: null,
  }

  try {
    const existingTransactions = await depositRepository
      .search()
      .where('address')
      .equals(address)
      .and('txHash')
      .equals(txHash)
      .return.all()

    if (existingTransactions.length > 0) {
      logger.info(
        `Transaction already exists for address: ${address} and txHash: ${txHash}`
      )
      return existingTransactions[0]
    } else {
      const transaction = await depositRepository.save(transactionData)
      logger.info(
        `Transaction tracing started for address: ${address} and txHash: ${txHash}`
      )
      const symbols = Object.getOwnPropertySymbols(transaction)
      const entityIdSymbol = symbols.find(
        (symbol) => symbol.toString() === 'Symbol(entityId)'
      )
      const value = transaction[entityIdSymbol as any]

      return {
        ...transaction,
        entityId: value,
      }
    }
  } catch (error) {
    logger.error(
      `Error storing or retrieving transaction data: ${error.message}`
    )
    throw error
  }
}

export const getTransactionsByAddress = async (
  address: string
): Promise<object[]> => {
  return await depositRepository
    .search()
    .where('address')
    .equals(address)
    .return.all()
}

export const getStatusByTxHashOrEntityId = async (
  identifier: string
): Promise<string | null> => {
  const transactions = await depositRepository
    .search()
    .where('txHash')
    .equals(identifier)
    .return.all()

  if (transactions.length === 0) {
    const transaction = await depositRepository.fetch(identifier)
    if (transaction) {
      return transaction.status
    }
  }

  if (transactions.length > 0) {
    return transactions[0].status
  }
  return null
}

export const getTransactionByEntityId = async (
  entityId: string
): Promise<object | null> => {
  const transactionsByEntityId = await depositRepository.fetch(entityId)
  if (transactionsByEntityId && 'txHash' in transactionsByEntityId) {
    return transactionsByEntityId
  }
  return null
}

export const getTransactionByTxHash = async (
  txHash: string
): Promise<object | null> => {
  const transactionsByTxHash = await depositRepository
    .search()
    .where('txHash')
    .equals(txHash)
    .return.all()

  if (transactionsByTxHash.length > 0) {
    return transactionsByTxHash[0]
  }
  return null
}

export async function findTransactionsByAddressAndStatus(
  address: string,
  status: string
) {
  const transactions = await depositRepository
    .search()
    .where('address')
    .equals(address)
    .and('status')
    .equals(status)
    .return.all()
  logger.info(
    `Found ${transactions.length} transactions for address: ${address} and status: ${status}`
  )
  return transactions
}
