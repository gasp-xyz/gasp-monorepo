import { createPublicClient, http, encodeFunctionData } from 'viem'

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

    // 3. Call close_withdrawal on Rolldown contract (validation only)
    logger.info(`Validating withdrawal can be closed on Rolldown contract for chain: ${withdrawal.chain}`)
    const validationResult = await callCloseWithdrawal(withdrawalData, merkleRoot, proof, withdrawal.chain)

    // 4. Update withdrawal status in database
    logger.info(`Updating withdrawal status to 'Processed' for entityId: ${withdrawal.entityId}`)
    await updateWithdrawalStatus(withdrawal.entityId, 'Processed', 'other')

    logger.info(`Withdrawal ${txHash} closed successfully with validation result: ${validationResult}`)
    
    return { txHash: validationResult }
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
  logger.info(`Extracting proof from withdrawal:`, { 
    proof: withdrawal.proof, 
    proofType: typeof withdrawal.proof,
    proofLength: withdrawal.proof?.length 
  })
  
  if (!withdrawal.proof) {
    logger.info('No proof found, returning empty array')
    return []
  }
  
  // If proof is stored as a hex string, we need to parse it
  if (typeof withdrawal.proof === 'string') {
    // If it's a single hex value like "0x00", convert to empty array
    if (withdrawal.proof === '0x00' || withdrawal.proof === '0x' || withdrawal.proof === '') {
      logger.info('Proof is empty hex string, returning empty array')
      return []
    }
    
    // If it's a comma-separated string of hex values, split it
    if (withdrawal.proof.includes(',')) {
      const proofArray = withdrawal.proof.split(',').filter((p: string) => p && p !== '0x00')
      logger.info(`Proof split by comma:`, proofArray)
      return proofArray
    }
    
    // If it's a single hex string longer than 66 chars (32 bytes), it might be concatenated proofs
    // For now, treat single hex string as one proof element
    logger.info(`Proof is single hex string, returning as array:`, [withdrawal.proof])
    return [withdrawal.proof]
  }
  
  // If it's already an array, return as is
  if (Array.isArray(withdrawal.proof)) {
    const filteredProof = withdrawal.proof.filter((p: string) => p && p !== '0x00')
    logger.info(`Proof is array, filtered:`, filteredProof)
    return filteredProof
  }
  
  logger.info('Proof format unknown, returning empty array')
  return []
}

function getChainConfig(chain: string) {
  const chainKey = `${process.env.NODE_ENV || 'localhost'}-${chain.toLowerCase()}`
  logger.info(`Looking up chain config for key: ${chainKey}`)
  let chainConfig = CONFIG_TO_CHAIN.get(chainKey)
  
  if (!chainConfig) {
    logger.error(`Unsupported chain: ${chain}, chainKey: ${chainKey}`)
    throw new Error(`Unsupported chain: ${chain}`)
  }
  
  // Override RPC URLs with Tenderly endpoints if available
  const chainUpper = chain.toUpperCase()
  let rpcUrl: string | undefined
  
  switch (chainUpper) {
    case 'ETHEREUM':
      rpcUrl = process.env.ETH_CHAIN_URL
      break
    case 'ARBITRUM':
      rpcUrl = process.env.ARBITRUM_SEPOLIA_CHAIN_URL
      break
    case 'BASE':
      rpcUrl = process.env.BASE_CHAIN_URL
      break
    case 'SONIC':
      rpcUrl = process.env.SONIC_CHAIN_URL
      break
  }
  
  if (rpcUrl) {
    logger.info(`Using custom RPC URL for ${chain}: ${rpcUrl}`)
    // Create a new chain config with the custom RPC URL
    chainConfig = {
      ...chainConfig,
      rpcUrls: {
        ...chainConfig.rpcUrls,
        default: {
          http: [rpcUrl]
        }
      }
    }
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
  
  // Ensure the address has 0x prefix
  const formattedAddress = contractAddress.startsWith('0x') ? contractAddress : `0x${contractAddress}`
  
  logger.info(`Found contract address: ${formattedAddress}`)
  return formattedAddress
}


async function callCloseWithdrawal(
  withdrawalData: WithdrawalData,
  merkleRoot: string,
  proof: string[],
  chain: string
): Promise<string> {
  logger.info(`Getting configuration for chain: ${chain}`)
  const rolldownAddress = getContractAddress(chain)
  const chainConfig = getChainConfig(chain)

  logger.info(`Contract address: ${rolldownAddress}, Chain: ${chainConfig.name}`)
  
  const publicClient = createPublicClient({
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

  logger.info('Found close_withdrawal function in ABI:', {
    name: closeWithdrawalAbi.name,
    inputs: closeWithdrawalAbi.inputs,
    stateMutability: closeWithdrawalAbi.stateMutability
  })

  try {
    // Log all the parameters being sent
    logger.info('Parameters for close_withdrawal call:', {
      withdrawalData: {
        requestId: {
          origin: withdrawalData.requestId.origin,
          id: withdrawalData.requestId.id.toString()
        },
        recipient: withdrawalData.recipient,
        tokenAddress: withdrawalData.tokenAddress,
        amount: withdrawalData.amount.toString(),
        ferryTip: withdrawalData.ferryTip.toString()
      },
      merkleRoot,
      proof,
      contractAddress: rolldownAddress
    })

    // Encode the function call data
    logger.info('Encoding close_withdrawal function call...')
    const data = encodeFunctionData({
      abi: [closeWithdrawalAbi],
      functionName: 'close_withdrawal',
      args: [withdrawalData, merkleRoot, proof]
    })

    logger.info(`Encoded call data: ${data}`)

    // Try to get more detailed error information by using trace_call if available
    try {
      logger.info('Attempting trace_call for detailed error...')
      const traceResult = await publicClient.request({
        method: 'trace_call' as any,
        params: [
          {
            to: rolldownAddress,
            data: data
          },
          ['trace']
        ]
      })
      logger.info('Trace result:', traceResult)
    } catch (traceError) {
      logger.info('trace_call not available, continuing with regular call')
    }

    // Call the contract function
    logger.info('Calling close_withdrawal function...')
    const result = await publicClient.call({
      to: rolldownAddress as `0x${string}`,
      data: data
    })

    logger.info(`Close withdrawal call completed successfully`, { result })
    
    return result.data || 'success'
  } catch (error) {
    logger.error(`Contract call failed for chain ${chain}:`, { 
      error: error.message,
      stack: error.stack,
      rolldownAddress,
      callData: encodeFunctionData({
        abi: [closeWithdrawalAbi],
        functionName: 'close_withdrawal',
        args: [withdrawalData, merkleRoot, proof]
      })
    })

    // Try to decode the revert reason if possible
    if (error.data) {
      logger.error('Revert data:', error.data)
    }

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