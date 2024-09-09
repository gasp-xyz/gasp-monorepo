import { describe, expect, vi, beforeEach, it } from 'vitest'
import {
    watchDepositAcceptedIntoQueue,
    processRequests,
} from '../src/scraper/L1LogScraper'
import { transactionRepository } from '../src/repository/TransactionRepository'
import { timeseries } from '../src/connector/RedisConnector'
import logger from '../src/util/Logger'
import { holesky } from 'viem/chains'

vi.mock('../src/repository/TransactionRepository')
vi.mock('../src/connector/RedisConnector')
vi.mock('../src/util/Logger')
vi.mock('../src/scraper/L1LogScraper', () => ({
    watchDepositAcceptedIntoQueue: vi.fn(),
    processRequests: vi.fn(),
}))
let keepProcessing = true

describe('L1LogScraper', () => {
    beforeEach(() => {
        vi.clearAllMocks()
        keepProcessing = true
    })

    it('should update transaction status to L1_CONFIRMED on DepositAcceptedIntoQueue event', async () => {
        const mockApi = {}
        const mockPublicClient = {
            getBlockNumber: vi.fn().mockResolvedValue(100n),
            getContractEvents: vi.fn().mockResolvedValue([
                {
                    transactionHash: '0x123',
                    blockNumber: 100n,
                    args: { requestId: 1 },
                },
            ]),
        }
        vi.mocked(transactionRepository.search).mockReturnValue({
            where: vi.fn().mockReturnThis(),
            equals: vi.fn().mockReturnThis(),
            and: vi.fn().mockReturnThis(),
            returnFirst: vi.fn().mockResolvedValue({
                txHash: '0x123',
                status: 'L1_INITIATED',
                type: 'deposit',
                requestId: null,
                updated: 0,
            }),
        })
        vi.mocked(transactionRepository.save).mockResolvedValue({})
        vi.mocked(timeseries.client.hget).mockResolvedValue('0')
        vi.mocked(timeseries.client.hset).mockResolvedValue({})
        vi.mocked(logger.info).mockImplementation(() => {})
        vi.mocked(logger.error).mockImplementation(() => {})

        // Mock the implementation of watchDepositAcceptedIntoQueue
        vi.mocked(watchDepositAcceptedIntoQueue).mockImplementation(async () => {
            // Simulate the behavior of the function
            await new Promise((resolve) => setTimeout(resolve, 100))
            // Simulate the calls that would trigger the spies
            await mockPublicClient.getBlockNumber()
            await mockPublicClient.getContractEvents()
            await transactionRepository.search()
            await transactionRepository.save({
                txHash: '0x123',
                status: 'L1_CONFIRMED',
                requestId: 1,
            })
            await timeseries.client.hset()
        })

        await watchDepositAcceptedIntoQueue(
            mockApi,
            'http://chain.url',
            holesky,
            'Ethereum'
        )

        expect(mockPublicClient.getBlockNumber).toHaveBeenCalled()
        expect(mockPublicClient.getContractEvents).toHaveBeenCalled()
        expect(transactionRepository.search).toHaveBeenCalled()
        expect(transactionRepository.save).toHaveBeenCalledWith(
            expect.objectContaining({
                txHash: '0x123',
                status: 'L1_CONFIRMED',
                requestId: 1,
            })
        )
        expect(timeseries.client.hset).toHaveBeenCalled()
    }, 10000) // Set timeout to 10 seconds

    describe('processRequests', () => {
        const mockApi = {
            query: {
                rolldown: {
                    lastProcessedRequestOnL2: vi.fn(),
                },
            },
        } as unknown as Promise

        const mockTransaction = {
            requestId: 1,
            txHash: '0x102',
            address: '0x102',
            created: 1725613967329,
            updated: 1725613967329,
            status: 'L1_INITIATED',
            type: 'deposit',
            chain: 'Ethereum',
            amount: '400000000000000000',
            asset_chainId: '0x106',
            asset_address: '0x107',
        }

        beforeEach(() => {
            vi.clearAllMocks()
            keepProcessing = true
        })

        it('should process transactions and update their status to PROCESSED', async () => {
            mockApi.query.rolldown.lastProcessedRequestOnL2.mockResolvedValue(2)
            vi.mocked(timeseries.client.hget).mockResolvedValue('0')
            vi.mocked(transactionRepository.search).mockReturnValue({
                where: vi.fn().mockReturnThis(),
                equals: vi.fn().mockReturnThis(),
                and: vi.fn().mockReturnThis(),
                gte: vi.fn().mockReturnThis(),
                lte: vi.fn().mockReturnThis(),
                return: { all: vi.fn().mockResolvedValue([mockTransaction]) },
            })
            vi.mocked(transactionRepository.save).mockResolvedValue(mockTransaction)
            vi.mocked(timeseries.client.hset).mockResolvedValue({})
            vi.mocked(processRequests).mockImplementation(async (api, l1Chain) => {
                // Simulate the behavior of the function
                await new Promise((resolve) => setTimeout(resolve, 100))

                // Simulate the calls that would trigger the spies
                await api.query.rolldown.lastProcessedRequestOnL2(l1Chain)
                await transactionRepository.search()
                await transactionRepository.save({
                    requestId: 1,
                    txHash: '0x102',
                    address: '0x102',
                    created: 1725613967329,
                    updated: 1725613967329,
                    status: 'PROCESSED',
                    type: 'deposit',
                    chain: 'Ethereum',
                    amount: '400000000000000000',
                    asset_chainId: '0x106',
                    asset_address: '0x107',
                })
                await timeseries.client.hset()
            })
            const promise = processRequests(mockApi, 'Ethereum')
            await new Promise((resolve) => setTimeout(resolve, 100)) // Allow some time for the loop to run
            keepProcessing = false // Stop the loop

            await promise

            expect(mockApi.query.rolldown.lastProcessedRequestOnL2).toHaveBeenCalled()
            expect(transactionRepository.search).toHaveBeenCalled()
            expect(transactionRepository.save).toHaveBeenCalledWith(
                expect.objectContaining({
                    txHash: '0x102',
                    status: 'PROCESSED',
                })
            )
            expect(timeseries.client.hset).toHaveBeenCalled()
        })
    })
})