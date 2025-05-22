import logger from '../util/Logger.js'
import {
  depositRepository,
  withdrawalRepository,
} from '../repository/TransactionRepository.js'
import { redis } from '../connector/RedisConnector.js'
const WITHDRAWAL_INITIATED_BY_FE = 'InitiatedByFrontend'
const NETWORK_LIST_KEY = 'affirmed_networks_list'

export enum CreatedBy {
  Frontend = 'frontend',
  Other = 'other',
}

interface Network {
  key: string
  chainId: string
}

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
  const { type } = traceRequest
  const timestamp = new Date().toISOString()
  let status: string
  if (type === 'deposit') {
    try {
      return await startTracingDeposit(traceRequest)
    } catch (error) {
      logger.error(`Error in startTracingDeposit: ${error.message}`)
      throw error
    }
  } else {
    status = 'UNKNOWN'
  }
}

async function startTracingDeposit({
  txHash,
  address,
  type,
  chain,
  amount,
  asset_chainId,
  asset_address,
}: TraceTransactionRequest) {
  const timestamp = new Date().toISOString()
  const status = 'PendingOnL1'
  const transactionData = {
    requestId: null, //requestId we get from L1, empty when we start tracing
    txHash: txHash,
    address: address,
    recipient: address, //recipient is the same as address for deposits
    created: Date.parse(timestamp),
    updated: Date.parse(timestamp),
    status: status,
    type: type,
    chain: chain,
    amount: amount,
    asset_chainId: asset_chainId,
    asset_address: asset_address,
    createdBy: CreatedBy.Frontend,
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
  address: string,
  type: string
): Promise<object[]> => {
  const repository =
    type === 'deposit' ? depositRepository : withdrawalRepository
  return await repository.search().where('address').equals(address).and('createdBy').equals(CreatedBy.Frontend).all()
}

export const getByTxHashOrEntityId = async (
  identifier: string,
  type: string
): Promise<object | null> => {
  const repository =
    type === 'deposit' ? depositRepository : withdrawalRepository
  const transactions = await repository
    .search()
    .where('txHash')
    .equals(identifier)
    .return.all()

  if (transactions.length === 0) {
    const transaction = await repository.fetch(identifier)
    if (transaction) {
      return transaction
    }
  }

  if (transactions.length > 0) {
    return transactions[0]
  }
  return null
}

export async function findTransactionsByAddressAndStatus(
  address: string,
  status: string,
  type: string
) {
  const repository =
    type === 'deposit' ? depositRepository : withdrawalRepository
  const transactions = await repository
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
