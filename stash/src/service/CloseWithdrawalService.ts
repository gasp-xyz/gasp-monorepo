import { createPublicClient, createWalletClient, http } from 'viem'
import { privateKeyToAccount } from 'viem/accounts'

import rolldownAbi from '../Rolldown.json' with { type: 'json' }
import { withdrawalRepository } from '../repository/TransactionRepository.js'
import { CONFIG_TO_CHAIN } from '../util/ConfigToChain.js'
import logger from '../util/Logger.js'

interface WithdrawalData {
  requestId: {
    origin: number
    id: bigint
  }
  recipient: string
  tokenAddress: string
  amount: bigint
  ferryTip: bigint
}

interface CloseWithdrawalResult {
  txHash: string
}

export const closeWithdrawal = async (txHash: string): Promise<CloseWithdrawalResult> => {
  logger.info(`Starting close withdrawal process for txHash: ${txHash}`)
  
  try {
    // 1. Fetch withdrawal data from database
    logger.info(`Fetching withdrawal data from database for txHash: ${txHash}`)
    const withdrawal = await getWithdrawalByTxHash(txHash)
    if (!withdrawal) {
      logger.error(`Withdrawal not found for txHash: ${txHash}`)
      throw new Error(`Withdrawal not found for txHash: ${txHash}`)
    }

    logger.info(`Found withdrawal: requestId=${withdrawal.requestId}, chain=${withdrawal.chain}, status=${withdrawal.status}`)

    // 2. Extract withdrawal data from database record
    logger.info(`Building withdrawal data structure for contract call`)
    const withdrawalData = buildWithdrawalData(withdrawal)
    const merkleRoot = extractMerkleRoot(withdrawal)
    const proof = extractProof(withdrawal)

    logger.info(`Withdrawal data prepared:`, {
      requestId: withdrawalData.requestId,
      recipient: withdrawalData.recipient,
      tokenAddress: withdrawalData.tokenAddress,
      amount: withdrawalData.amount.toString(),
      ferryTip: withdrawalData.ferryTip.toString(),
      merkleRoot,
      proofLength: proof.length
    })

    // 3. Call close_withdrawal on Rolldown contract
    logger.info(`Calling close_withdrawal on Rolldown contract for chain: ${withdrawal.chain}`)
    const contractTxHash = await callCloseWithdrawal(withdrawalData, merkleRoot, proof, withdrawal.chain)

    // 4. Update withdrawal status in database
    logger.info(`Updating withdrawal status to 'Processed' for entityId: ${withdrawal.entityId}`)
    await updateWithdrawalStatus(withdrawal.entityId, 'Processed', 'other')

    logger.info(`Withdrawal ${txHash} closed successfully with contract tx: ${contractTxHash}`)
    
    return { txHash: contractTxHash }
  } catch (error) {
    logger.error(`Error closing withdrawal ${txHash}: ${error.message}`, { error: error.stack })
    throw error
  }
}

async function getWithdrawalByTxHash(txHash: string): Promise<any | null> {
  const withdrawals = await withdrawalRepository
    .search()
    .where('txHash')
    .equals(txHash)
    .and('type')
    .equals('withdrawal')
    .return.all()

  if (withdrawals.length === 0) {
    return null
  }

  // Get entityId for the withdrawal
  const withdrawal = withdrawals[0] as any
  const symbols = Object.getOwnPropertySymbols(withdrawal)
  const entityIdSymbol = symbols.find(
    (symbol) => symbol.toString() === 'Symbol(entityId)',
  )
  const entityId = withdrawal[entityIdSymbol as any]

  return {
    ...withdrawal,
    entityId
  }
}

function buildWithdrawalData(withdrawal: any): WithdrawalData {
  return {
    requestId: {
      origin: 1,
      id: BigInt(withdrawal.requestId || '0')
    },
    recipient: withdrawal.recipient,
    tokenAddress: withdrawal.asset_address,
    amount: BigInt(withdrawal.amount),
    ferryTip: BigInt(withdrawal.ferryTip || '0')
  }
}

function extractMerkleRoot(withdrawal: any): string {
  return withdrawal.root || '0x0000000000000000000000000000000000000000000000000000000000000000'
}

function extractProof(withdrawal: any): string[] {
  return withdrawal.proof || []
}

function getChainConfig(chain: string) {
  const chainKey = `${process.env.NODE_ENV || 'localhost'}-${chain.toLowerCase()}`
  logger.info(`Looking up chain config for key: ${chainKey}`)
  const chainConfig = CONFIG_TO_CHAIN.get(chainKey)
  
  if (!chainConfig) {
    logger.error(`Unsupported chain: ${chain}, chainKey: ${chainKey}`)
    throw new Error(`Unsupported chain: ${chain}`)
  }
  
  logger.info(`Found chain config for: ${chainConfig.name}`)
  return chainConfig
}

function getContractAddress(chain: string): string {
  const chainUpper = chain.toUpperCase()
  const envVar = `CONTRACT_ADDRESS_${chainUpper}`
  logger.info(`Looking up contract address from env var: ${envVar}`)
  const contractAddress = process.env[envVar]
  
  if (!contractAddress) {
    logger.error(`${envVar} environment variable not set`)
    throw new Error(`${envVar} environment variable not set`)
  }
  
  logger.info(`Found contract address: ${contractAddress}`)
  return contractAddress
}

function getPrivateKey(chain: string): string {
  const chainUpper = chain.toUpperCase()
  const envVar = `CLOSE_WITHDRAWAL_PRIVATE_KEY_${chainUpper}`
  logger.info(`Looking up private key from env var: ${envVar}`)
  const privateKey = process.env[envVar]
  
  if (!privateKey) {
    logger.error(`${envVar} environment variable not set`)
    throw new Error(`${envVar} environment variable not set`)
  }
  
  logger.info(`Found private key for chain: ${chain}`)
  return privateKey
}

async function callCloseWithdrawal(
  withdrawalData: WithdrawalData,
  merkleRoot: string,
  proof: string[],
  chain: string
): Promise<string> {
  logger.info(`Getting configuration for chain: ${chain}`)
  const privateKey = getPrivateKey(chain)
  const rolldownAddress = getContractAddress(chain)
  const chainConfig = getChainConfig(chain)

  logger.info(`Contract address: ${rolldownAddress}, Chain: ${chainConfig.name}`)

  const account = privateKeyToAccount(privateKey as `0x${string}`)
  logger.info(`Using account: ${account.address}`)
  
  const publicClient = createPublicClient({
    chain: chainConfig,
    transport: http()
  })

  const walletClient = createWalletClient({
    account,
    chain: chainConfig,
    transport: http()
  })

  // Extract the close_withdrawal function ABI
  const closeWithdrawalAbi = rolldownAbi.abi.find(
    (item: any) => item.name === 'close_withdrawal' && item.type === 'function'
  )

  if (!closeWithdrawalAbi) {
    logger.error('close_withdrawal function not found in Rolldown ABI')
    throw new Error('close_withdrawal function not found in Rolldown ABI')
  }

  logger.info('Found close_withdrawal function in ABI')

  try {
    // Simulate the transaction first
    logger.info('Simulating contract transaction...')
    const { request } = await publicClient.simulateContract({
      address: rolldownAddress as `0x${string}`,
      abi: [closeWithdrawalAbi],
      functionName: 'close_withdrawal',
      args: [withdrawalData, merkleRoot, proof],
      account: account.address
    })

    logger.info('Transaction simulation successful, executing transaction...')

    // Execute the transaction
    const txHash = await walletClient.writeContract(request)
    
    logger.info(`Close withdrawal transaction sent: ${txHash}`)
    
    // Wait for transaction confirmation
    logger.info(`Waiting for transaction confirmation: ${txHash}`)
    const receipt = await publicClient.waitForTransactionReceipt({
      hash: txHash,
      confirmations: 1
    })

    logger.info(`Transaction confirmed with status: ${receipt.status}, gas used: ${receipt.gasUsed}`)

    if (receipt.status === 'reverted') {
      logger.error(`Transaction reverted: ${txHash}`)
      throw new Error(`Transaction reverted: ${txHash}`)
    }

    logger.info(`Transaction successful: ${txHash}`)
    return txHash
  } catch (error) {
    logger.error(`Contract call failed for chain ${chain}: ${error.message}`, { 
      error: error.stack,
      rolldownAddress,
      accountAddress: account.address
    })
    throw error
  }
}

async function updateWithdrawalStatus(entityId: string, status: string, closedBy: string) {
  logger.info(`Updating withdrawal status: entityId=${entityId}, status=${status}, closedBy=${closedBy}`)
  
  try {
    const withdrawal = await withdrawalRepository.fetch(entityId)
    if (withdrawal) {
      const oldStatus = withdrawal.status
      withdrawal.status = status
      withdrawal.closedBy = closedBy
      withdrawal.updated = Date.now()
      await withdrawalRepository.save(withdrawal)
      
      logger.info(`Withdrawal status updated successfully: ${oldStatus} -> ${status}`)
    } else {
      logger.error(`Withdrawal not found for entityId: ${entityId}`)
    }
  } catch (error) {
    logger.error(`Failed to update withdrawal status: ${error.message}`, { 
      entityId, 
      status, 
      closedBy, 
      error: error.stack 
    })
    throw error
  }
}