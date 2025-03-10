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
  } else if (type === 'withdrawal') {
    try {
      return await processWithdrawalRequest(traceRequest)
    } catch (error) {
      logger.error(`Error in processWithdrawalRequest: ${error.message}`)
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

async function processWithdrawalRequest({
  txHash,
  address,
  type,
  chain,
  amount,
  asset_chainId,
  asset_address,
}: TraceTransactionRequest) {
  //check if it already exists
  const existingWithdrawal = await withdrawalRepository
    .search()
    .where('txHash')
    .equals(txHash)
    .and('type')
    .equals(type)
    .returnFirst()
  if (existingWithdrawal) {
    existingWithdrawal.createdBy = CreatedBy.Frontend
    const timestamp = new Date().toISOString()
    existingWithdrawal.updated = Date.parse(timestamp)
    await withdrawalRepository.save(existingWithdrawal)
    logger.info({
      message: 'Withdrawal updated with createdBy:',
      transaction: existingWithdrawal,
    })
    return {
      ...existingWithdrawal,
    }
  } else {
    //create a new one
    const timestamp = new Date().toISOString()
    const affirmedNetworks = await redis.client.get(NETWORK_LIST_KEY)
    const networks = affirmedNetworks ? JSON.parse(affirmedNetworks) : []
    const network = networks.find((net: Network) => net.key === chain)
    const chainId = network ? network.chainId : 'unknown'

    const withdrawalData = {
      requestId: null,
      txHash: txHash,
      address: address,
      created: Date.parse(timestamp),
      updated: Date.parse(timestamp),
      status: WITHDRAWAL_INITIATED_BY_FE,
      type: type,
      chain: chain,
      amount: amount.replace(/,/g, ''),
      asset_chainId: chainId,
      asset_address: asset_address,
      proof: '',
      calldata: '',
      createdBy: CreatedBy.Frontend,
      closedBy: null,
    }
    const withdrawal = await withdrawalRepository.save(withdrawalData)
    const symbols = Object.getOwnPropertySymbols(withdrawal)
    const entityIdSymbol = symbols.find(
      (symbol) => symbol.toString() === 'Symbol(entityId)'
    )
    const value = withdrawal[entityIdSymbol as any]
    return {
      ...withdrawal,
      entityId: value,
    }
  }
}

export const getTransactionsByAddress = async (
  address: string,
  type: string
): Promise<object[]> => {
  const repository =
    type === 'deposit' ? depositRepository : withdrawalRepository
  return await repository.search().where('address').equals(address).return.all()
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
