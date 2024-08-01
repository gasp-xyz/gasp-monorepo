import axios from 'axios'
import * as process from 'node:process'
import { redis } from '../connector/RedisConnector.js'
import logger from '../util/Logger.js'
import { createWalletClient, http, publicActions } from 'viem'
import { holesky } from 'viem/chains'
import { privateKeyToAccount } from 'viem/accounts'
import Gasp from '../Gasp.json' assert { type: 'json' }
import { ForbiddenException } from '../error/Exception.js'

const VERIFY_URL = 'https://api.hcaptcha.com/siteverify'
//the two enums below you can use with token 10000000-aaaa-bbbb-cccc-000000000001 to have a test example of success
const TEST_SECRET = '0x0000000000000000000000000000000000000000'
const TEST_SITEKEY = '10000000-ffff-ffff-ffff-000000000001'
const TOKEN_REQUEST_PREFIX = 'token_request:'
const MAX_REQUESTS = 3
const DECIMALS = 19 //18 decimals == 1 token, 19 == 10 tokens
const tokenToSendAddress = process.env.GASPV2_TOKEN_ADDRESS

export const verifyCaptcha = async (captchaToken: string): Promise<void> => {
  const payload = {
    // secret: process.env.CAPTCHA_SECRET, //comment out this, and uncomment the next line to test with the test token
    secret: TEST_SECRET,
    // sitekey: process.env.CAPTCHA_SITEKEY, //comment out this, and uncomment the next line to test with the test token
    sitekey: TEST_SITEKEY,
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
      'Captcha verification failed. Reason: ' + errorCodes
    )
  }
}

export const sendTokens = async (toAddress: string): Promise<void> => {
  //Check if the address has requested the token more than MAX_REQUESTS times
  const key = `${TOKEN_REQUEST_PREFIX}${toAddress}`
  const requestCount = await redis.client.get(key)
  if (requestCount && Number(requestCount) >= MAX_REQUESTS) {
    throw new ForbiddenException(
      `Address ${toAddress} has requested the token more than ${MAX_REQUESTS} times.`
    )
  }
  const amount = BigInt(10 ** DECIMALS) // 10 GASPV2 tokens
  await send(tokenToSendAddress, toAddress, amount)
  logger.info(`Sent ${amount} tokens to ${toAddress}`)
  await incrementTokenRequest(toAddress)
  logger.info(`Incremented token request count for ${toAddress}`)
}

export const incrementTokenRequest = async (address: string): Promise<void> => {
  const key = `${TOKEN_REQUEST_PREFIX}${address}`
  await redis.client.incr(key)
}

async function send(
  tokenToSendAddress: any,
  toAddress: string,
  amount: bigint
) {
  // Create a new wallet client
  const client = createWalletClient({
    chain: holesky,
    transport: http(process.env.HOLESKY_ADDRESS),
  }).extend(publicActions)
  const account = privateKeyToAccount(
    `0x${process.env.ORIGIN_ACCOUNT_PRIVATE_KEY}`
  )
  logger.info('Sending tokens...')
  //Simulate the transaction
  const { request } = await client.simulateContract({
    account: account,
    chain: holesky,
    abi: Gasp.abi,
    address: tokenToSendAddress, //tokenToSendAddress is the address of the GASPV2 token
    args: [toAddress, amount], //toAddress is the address of the account that will receive the tokens
    functionName: 'transfer',
  })
  logger.info('Transaction simulation successful, executing transaction...')
  //Execute the transaction
  return await client.writeContract(request)
}
