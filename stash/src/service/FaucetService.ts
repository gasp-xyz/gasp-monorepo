import * as process from 'node:process'

import axios from 'axios'
import {
  createPublicClient,
  createWalletClient,
  http,
  publicActions,
} from 'viem'
import { privateKeyToAccount } from 'viem/accounts'

import { redis } from '../connector/RedisConnector.js'
import { ForbiddenException } from '../error/Exception.js'
import Gasp from '../Gasp.json' with { type: 'json' }
import { CONFIG_TO_CHAIN } from '../util/ConfigToChain.js'
import logger from '../util/Logger.js'

interface SimulateTransactionRequest {
  client: any
  account: any
  tokenToSendAddress: any
  toAddress: string
  amount: bigint
}

const VERIFY_URL = 'https://api.hcaptcha.com/siteverify'
const TOKEN_REQUEST_PREFIX = 'token_request:'
const MAX_REQUESTS = 3
const DECIMALS = 19 //18 decimals == 1 token, 19 == 10 tokens
const tokenToSendAddress = '0x' + process.env.GASPV2_TOKEN_ADDRESS

export const verifyCaptcha = async (captchaToken: string): Promise<void> => {
  const payload = {
    secret: process.env.CAPTCHA_SECRET,
    sitekey: process.env.CAPTCHA_SITEKEY,
    response: captchaToken,
  }
  const response = await axios.post(VERIFY_URL, payload, {
    headers: {
      'Content-Type': 'application/x-www-form-urlencoded',
    },
  })
  logger.info('Captcha verification response: ' + JSON.stringify(response.data))
  if (!response.data.success) {
    logger.warn('Captcha verification failed', response.data)
    //hcaptcha returns success as false if the captcha is not valid
    const errorCodes = response.data['error-codes']
    throw new ForbiddenException(
      'Captcha verification failed. Reason: ' + errorCodes,
    )
  }
}

export const sendTokens = async (toAddress: string): Promise<void> => {
  await checkRequestCount(toAddress)
  const amount = BigInt(10 ** DECIMALS) // 10 GASPV2 tokens
  await send(tokenToSendAddress, toAddress, amount)
  logger.info(`Sent ${amount} tokens to ${toAddress}`)
  await incrementTokenRequest(toAddress)
  logger.info(`Incremented token request count for ${toAddress}`)
}

export const checkRequestCount = async (toAddress: string): Promise<void> => {
  const key = `${TOKEN_REQUEST_PREFIX}${toAddress}`
  const requestCount = await redis.client.get(key)
  if (requestCount && Number(requestCount) >= MAX_REQUESTS) {
    throw new ForbiddenException(
      `Address ${toAddress} has requested the token more than ${MAX_REQUESTS} times.`,
    )
  }
}

const incrementTokenRequest = async (address: string): Promise<void> => {
  const key = `${TOKEN_REQUEST_PREFIX}${address}`
  await redis.client.incr(key)
}

const send = async (
  tokenToSendAddress: any,
  toAddress: string,
  amount: bigint,
) => {
  // Create a new wallet client
  const client = createWalletClient({
    chain: CONFIG_TO_CHAIN.get(process.env.ENVIRONMENT + '-ethereum'),
    transport: http(process.env.RPC_ADDRESS),
  }).extend(publicActions)
  const account = privateKeyToAccount(
    `0x${process.env.ORIGIN_ACCOUNT_PRIVATE_KEY}`,
  )
  logger.info('Sending tokens...')
  const transaction: SimulateTransactionRequest = {
    client,
    account,
    tokenToSendAddress,
    toAddress,
    amount,
  }
  const request = await simulateTransaction(transaction)
  //Execute the transaction
  return await client.writeContract(request)
}

const simulateTransaction = async (transaction: SimulateTransactionRequest) => {
  const publicClient = createPublicClient({
    chain: CONFIG_TO_CHAIN.get(process.env.ENVIRONMENT + '-ethereum'),
    transport: http(process.env.RPC_ADDRESS),
  })
  const baseFeeInWei = await publicClient.getGasPrice()
  const maxPriorityFeePerGasInWei =
    await publicClient.estimateMaxPriorityFeePerGas()
  const maxFeeInWei =
    BigInt(2) * BigInt(baseFeeInWei) + BigInt(maxPriorityFeePerGasInWei)

  const { request } = await transaction.client.simulateContract({
    account: transaction.account,
    chain: CONFIG_TO_CHAIN.get(process.env.ENVIRONMENT + '-ethereum'),
    abi: Gasp.abi,
    address: transaction.tokenToSendAddress,
    args: [transaction.toAddress, transaction.amount],
    functionName: 'transfer',
    maxFeePerGas: maxFeeInWei,
    maxPriorityFeePerGas: maxPriorityFeePerGasInWei,
  })
  logger.info('Transaction simulation successful, executing transaction...')
  return request
}
