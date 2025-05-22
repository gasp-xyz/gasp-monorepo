import { describe, expect, it, beforeAll, vi } from 'vitest'
import supertest from 'supertest'
import app from '../../src/app'
import logger from '../../src/util/Logger'
import { generateRandomAddress, generateRandomHash } from './utils'
import { withdrawalRepository } from '../../src/repository/TransactionRepository'

const wrongType = 'deposittt'
const type = 'withdrawal'

const withdrawalData = {
  requestId: 2,
  txHash: '0x102',
  address: '0x102',
  created: 1731069089582,
  updated: 1731069095606,
  status: 'WITHDRAWAL_PENDING_ON_L2',
  type: 'withdrawal',
  chain: 'Ethereum',
  amount: '400000000',
  asset_chainId: '0x106',
  asset_address: '0x107',
  proof: '',
  calldata: '0x0000000',
  closedBy: null,
}

describe('TracingController', () => {
  beforeAll(async () => {
    await withdrawalRepository.save(withdrawalData)
  })

  describe('Query transactions', () => {
    it('should get transaction status by txHash or entityId', async () => {
      try {
        const response = await supertest(app)
          .get(`/tracing/type/${type}/tx/${withdrawalData.txHash}`)
          .expect(200)
        expect(response.body).toEqual(
          expect.objectContaining({ status: 'WITHDRAWAL_PENDING_ON_L2' })
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
            `/tracing/type/${type}/tx/listByAddress/${withdrawalData.address}`
          )
          .expect(200)

        const expectedProperties = {
          address: withdrawalData.address,
          amount: withdrawalData.amount,
          asset_address: withdrawalData.asset_address,
          asset_chainId: withdrawalData.asset_chainId,
          chain: withdrawalData.chain,
          created: withdrawalData.created,
          requestId: withdrawalData.requestId,
          status: withdrawalData.status,
          txHash: withdrawalData.txHash,
          type: withdrawalData.type,
          closedBy: withdrawalData.closedBy,
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
            `/tracing/type/${type}/tx/listByAddress/${withdrawalData.address}/WITHDRAWAL_PENDING_ON_L2`
          )
          .expect(200)
        expect(response.body).toHaveProperty('transactions')
        expect(response.body.transactions).toBeInstanceOf(Array)
        expect(response.body.transactions[0]).toHaveProperty(
          'address',
          withdrawalData.address
        )
        expect(response.body.transactions[0]).toHaveProperty(
          'status',
          'WITHDRAWAL_PENDING_ON_L2'
        )
        expect(response.body.transactions[0]).toHaveProperty('closedBy')
      } catch (error) {
        logger.error(
          'Error in get all transactions by address and status:',
          error.message
        )
        throw error
      }
    })
  })

  describe('API errors', () => {
    it('should return an error when geting tx status using wrong txHash', async () => {
      const errorMessage = 'Transaction not found'
      const wrongTxHash = generateRandomHash()
      const response = await supertest(app)
        .get(`/tracing/type/${type}/tx/${wrongTxHash}`)
        .expect(404)
      expect(response.body).toEqual(
        expect.objectContaining({ error: errorMessage })
      )
    })

    it('should return an error when getting all txs using wrong address', async () => {
      const errorMessage = 'No transactions found for this address'
      const wrongAddress = generateRandomAddress()
      const response = await supertest(app)
        .get(`/tracing/type/${type}/tx/listByAddress/${wrongAddress}`)
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
        .get(
          `/tracing/type/${type}/tx/listByAddress/${wrongAddress}/WITHDRAWAL_PENDING_ON_L2`
        )
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
        .get(`/tracing/type/${type}/tx/findByEntityId/${wrongEntityId}`)
        .expect(404)
      expect(response.body).toEqual(
        expect.objectContaining({
          error: errorMessage,
        })
      )
    })

    it('should get transaction status by txHash or entityId', async () => {
      const errorMessage = "type must be either 'deposit' or 'withdrawal'"
      const response = await supertest(app)
        .get(`/tracing/type/${wrongType}/tx/${withdrawalData.txHash}`)
        .expect(500)
      expect(response.body).toEqual(
        expect.objectContaining({
          exceptionName: 'ValidationError',
          message: errorMessage,
        })
      )
    })

    it('should get all transactions by address', async () => {
      const errorMessage = "type must be either 'deposit' or 'withdrawal'"
      const response = await supertest(app)
        .get(
          `/tracing/type/${wrongType}/tx/listByAddress/${withdrawalData.address}`
        )
        .expect(500)
      expect(response.body).toEqual(
        expect.objectContaining({
          exceptionName: 'ValidationError',
          message: errorMessage,
        })
      )
    })

    it('should get all transactions by address and status', async () => {
      const errorMessage = "type must be either 'deposit' or 'withdrawal'"
      const response = await supertest(app)
        .get(
          `/tracing/type/${wrongType}/tx/listByAddress/${withdrawalData.address}/WITHDRAWAL_PENDING_ON_L2`
        )
        .expect(500)
      expect(response.body).toEqual(
        expect.objectContaining({
          exceptionName: 'ValidationError',
          message: errorMessage,
        })
      )
    })

    it('should find a transaction by entityId', async () => {
      const errorMessage = "type must be either 'deposit' or 'withdrawal'"
      const response = await supertest(app)
        .get(
          `/tracing/type/${wrongType}/tx/findByEntityId/01JC5W45SGTYKRAGY1X1QP2SEQ`
        )
        .expect(500)
      expect(response.body).toEqual(
        expect.objectContaining({
          exceptionName: 'ValidationError',
          message: errorMessage,
        })
      )
    })
  })
})
