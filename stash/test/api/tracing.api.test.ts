import { describe, expect, it, beforeAll } from 'vitest'
import supertest from 'supertest'
import app from '../../src/app'
import logger from '../../src/util/Logger'
import { generateRandomAddress, generateRandomHash } from './utils'

let transactionData: any

describe('TracingController', () => {
  beforeAll(async () => {
    const response = await supertest(app).post('/tracing/tx/start').send({
      txHash: '0x102',
      address: '0x102',
      type: 'deposit',
      chain: 'Ethereum',
      amount: '400000000000000000',
      asset_chainId: '0x106',
      asset_address: '0x107',
    })
    transactionData = response.body
  })

  it('should start tracing a transaction', () => {
    expect(transactionData).toHaveProperty('transaction')
    expect(transactionData.transaction).toHaveProperty(
      'txHash',
      transactionData.transaction.address
    )
    expect(transactionData.transaction).toHaveProperty('address', '0x102')
    expect(transactionData.transaction).toHaveProperty('type', 'deposit')
    expect(transactionData.transaction).toHaveProperty('chain', 'Ethereum')
    expect(transactionData.transaction).toHaveProperty(
      'amount',
      '400000000000000000'
    )
    expect(transactionData.transaction).toHaveProperty('asset_chainId', '0x106')
    expect(transactionData.transaction).toHaveProperty('asset_address', '0x107')
    expect(transactionData.transaction).toHaveProperty('entityId')
    expect(transactionData.transaction.entityId).toBeTruthy()
    expect(transactionData.transaction).toHaveProperty('created')
    expect(transactionData.transaction.created).toBeTruthy()
    expect(transactionData.transaction).toHaveProperty('updated')
    expect(transactionData.transaction.updated).toBeTruthy()
  })

  describe('Query transactions', () => {
    it('should get transaction status by txHash or entityId', async () => {
      try {
        const response = await supertest(app)
          .get(`/tracing/tx/${transactionData.transaction.txHash}`)
          .expect(200)
        expect(response.body).toEqual(
          expect.objectContaining({ status: 'PendingOnL1' })
        )
      } catch (error) {
        logger.error(
          'Error in get transaction status by txHash or entityId:',
          error.message
        )
        throw error
      }
    })

    it('should get all transactions by address', async () => {
      try {
        const response = await supertest(app)
          .get(
            `/tracing/tx/listByAddress/${transactionData.transaction.address}`
          )
          .expect(200)

        const expectedProperties = {
          address: transactionData.transaction.address,
          amount: transactionData.transaction.amount,
          asset_address: transactionData.transaction.asset_address,
          asset_chainId: transactionData.transaction.asset_chainId,
          chain: transactionData.transaction.chain,
          created: transactionData.transaction.created,
          requestId: transactionData.transaction.requestId,
          status: transactionData.transaction.status,
          txHash: transactionData.transaction.txHash,
          type: transactionData.transaction.type,
        }
        expect(response.body).toHaveProperty('transactions')
        expect(response.body.transactions).toBeInstanceOf(Array)
        expect(response.body.transactions[0]).toHaveProperty('updated')

        Object.entries(expectedProperties).forEach(([key, value]) => {
          expect(response.body.transactions[0]).toHaveProperty(key, value)
        })
      } catch (error) {
        logger.error('Error in get all transactions by address:', error.message)
        throw error
      }
    })

    it('should get all transactions by address and status', async () => {
      try {
        const response = await supertest(app)
          .get(
            `/tracing/tx/listByAddress/${transactionData.transaction.address}/PendingOnL1`
          )
          .expect(200)
        expect(response.body).toHaveProperty('transactions')
        expect(response.body.transactions).toBeInstanceOf(Array)
        expect(response.body.transactions[0]).toHaveProperty(
          'address',
          transactionData.transaction.address
        )
        expect(response.body.transactions[0]).toHaveProperty(
          'status',
          'PendingOnL1'
        )
      } catch (error) {
        logger.error(
          'Error in get all transactions by address and status:',
          error.message
        )
        throw error
      }
    })

    it('should find a transaction by entityId', async () => {
      try {
        const response = await supertest(app)
          .get(
            `/tracing/tx/findByEntityId/${transactionData.transaction.entityId}`
          )
          .expect(200)
        expect(response.body).toHaveProperty('transaction')
      } catch (error) {
        logger.error('Error in find a transaction by entityId:', error.message)
        throw error
      }
    })
  })

  describe('API errors', () => {
    it('should return an error when geting tx status using wrong txHash', async () => {
      const errorMessage = 'Transaction not found'
      const wrongTxHash = generateRandomHash()
      const response = await supertest(app)
        .get(`/tracing/tx/${wrongTxHash}`)
        .expect(404)
      expect(response.body).toEqual(
        expect.objectContaining({ error: errorMessage })
      )
    })

    it('should return an error when getting all txs using wrong address', async () => {
      const errorMessage = 'No transactions found for this address'
      const wrongAddress = generateRandomAddress()
      const response = await supertest(app)
        .get(`/tracing/tx/listByAddress/${wrongAddress}`)
        .expect(404)
      expect(response.body).toEqual(
        expect.objectContaining({
          error: errorMessage,
        })
      )
    })

    it('should return an error when getting tx using wrong address and correct status', async () => {
      const errorMessage = 'No transactions found for this address'
      const wrongAddress = generateRandomAddress()
      const response = await supertest(app)
        .get(`/tracing/tx/listByAddress/${wrongAddress}/PendingOnL1`)
        .expect(404)
      expect(response.body).toEqual(
        expect.objectContaining({
          error: errorMessage,
        })
      )
    })

    it('should return an error when getting tx using wrong entityId', async () => {
      const errorMessage = 'Transaction not found for this entityId'
      const wrongEntityId = '00000000001111111111WWWWWW'
      const response = await supertest(app)
        .get(`/tracing/tx/findByEntityId/${wrongEntityId}`)
        .expect(404)
      expect(response.body).toEqual(
        expect.objectContaining({
          error: errorMessage,
        })
      )
    })
  })
})
