import { describe, expect, it, beforeAll } from "vitest";
import supertest from "supertest";
import app from '../../src/app';
import logger from "../../src/util/Logger";
import { generateRandomAddress, generateRandomHash } from './utils';

let transactionData: any;

describe('TracingController', () => {
    beforeAll(async () => {
        const response = await supertest(app)
            .post('/tracing/tx/start')
            .send({
                txHash: '0x102',
                address: '0x102',
                type: 'deposit',
                chain: 'Ethereum',
                amount: '400000000000000000',
                asset_chainId: '0x106',
                asset_address: '0x107'
            });
        transactionData = response.body;
    });

    it('should start tracing a transaction', () => {
        expect(transactionData).toHaveProperty('transaction');
        expect(transactionData.transaction).toHaveProperty('txHash', transactionData.transaction.address);
        expect(transactionData.transaction).toHaveProperty('address', '0x102');
        expect(transactionData.transaction).toHaveProperty('type', 'deposit');
        expect(transactionData.transaction).toHaveProperty('chain', 'Ethereum');
        expect(transactionData.transaction).toHaveProperty('amount', '400000000000000000');
        expect(transactionData.transaction).toHaveProperty('asset_chainId', '0x106');
        expect(transactionData.transaction).toHaveProperty('asset_address', '0x107');
        expect(transactionData.transaction).toHaveProperty('entityId');
        expect(transactionData.transaction.entityId).toBeTruthy();
        expect(transactionData.transaction).toHaveProperty('created');
        expect(transactionData.transaction.created).toBeTruthy();
        expect(transactionData.transaction).toHaveProperty('updated');
        expect(transactionData.transaction.updated).toBeTruthy(); 
    });

    describe('Query transactions', () => {
        it('should get transaction status by txHash or entityId', async () => {
            try {
                const response = await supertest(app)
                    .get(`/tracing/tx/${transactionData.transaction.txHash}`)
                    .expect(200);
                expect(response.body).toEqual(expect.objectContaining({status: 'L1_INITIATED'}));
            } catch (error) {
                logger.error('Error in get transaction status by txHash or entityId:', error.message);
                throw error;
            }
        });

        it('should return an error when geting tx status using wrong txHash', async () => {
            const wrongTxHash = generateRandomHash()
            const response = await supertest(app)
            .get(`/tracing/tx/${wrongTxHash}`)
            .expect(404);
            expect(response.body).toEqual(expect.objectContaining({error: 'Transaction not found'}));
        });

        it('should return an error when getting all txs using wrong address', async () => {
            const wrongAddress = generateRandomAddress();
            const response = await supertest(app)
            .get(`/tracing/tx/listByAddress/${wrongAddress}`)
            .expect(404);
            expect(response.body).toEqual(expect.objectContaining({error: 'No transactions found for this address'}));
        });

        it.skip('should return an error when getting tx using wrong address and status', async () => {
            const wrongAddress = generateRandomAddress();
            const wrongStatus = 'RANDOM_STATUS';
            const response = await supertest(app)
            .get(`/tracing/tx/listByAddress/${wrongAddress}/${wrongStatus}`)
            .expect(404);
            expect(response.body).toEqual(expect.objectContaining({error: 'No transactions found for this address'}));
        });

        it.skip('should return an error when getting tx using wrong entityId', async () => {
            const wrongEntityId = generateRandomAddress();
            const response = await supertest(app)
            .get(`/tracing/tx/findByEntityId/${wrongEntityId}`)
            .expect(404);
            expect(response.body).toEqual(expect.objectContaining({error: 'No transactions found for this address'}));
        });

        it('should get all transactions by address', async () => {
            try {
                const response = await supertest(app)
                    .get(`/tracing/tx/listByAddress/${transactionData.transaction.address}`)
                    .expect(200);
                expect(response.body).toHaveProperty('transactions');
                expect(response.body.transactions).toBeInstanceOf(Array);
                expect(response.body.transactions[0]).toHaveProperty('address', transactionData.transaction.address);
                expect(response.body.transactions[0]).toHaveProperty('amount', transactionData.transaction.amount);
                expect(response.body.transactions[0]).toHaveProperty('asset_address', transactionData.transaction.asset_address);
                expect(response.body.transactions[0]).toHaveProperty('asset_chainId', transactionData.transaction.asset_chainId);
                expect(response.body.transactions[0]).toHaveProperty('chain', transactionData.transaction.chain);
                expect(response.body.transactions[0]).toHaveProperty('created', transactionData.transaction.created);
                expect(response.body.transactions[0]).toHaveProperty('requestId', transactionData.transaction.requestId);
                expect(response.body.transactions[0]).toHaveProperty('status', transactionData.transaction.status);
                expect(response.body.transactions[0]).toHaveProperty('txHash', transactionData.transaction.txHash);
                expect(response.body.transactions[0]).toHaveProperty('type', transactionData.transaction.type);
                expect(response.body.transactions[0]).toHaveProperty('updated');
            } catch (error) {
                logger.error('Error in get all transactions by address:', error.message);
                throw error;
            }
        });

        it('should get all transactions by address and status', async () => {
            try {
                const response = await supertest(app)
                    .get(`/tracing/tx/listByAddress/${transactionData.transaction.address}/L1_INITIATED`)
                    .expect(200);
                expect(response.body).toHaveProperty('transactions');
                expect(response.body.transactions).toBeInstanceOf(Array);
                expect(response.body.transactions[0]).toHaveProperty('address', transactionData.transaction.address);
                expect(response.body.transactions[0]).toHaveProperty('status', 'L1_INITIATED');
            } catch (error) {
                logger.error('Error in get all transactions by address and status:', error.message);
                throw error;
            }
        });

        it('should find a transaction by entityId', async () => {
            try {
                const response = await supertest(app)
                    .get(`/tracing/tx/findByEntityId/${transactionData.transaction.entityId}`)
                    .expect(200);
                expect(response.body).toHaveProperty('transaction');
            } catch (error) {
                logger.error('Error in find a transaction by entityId:', error.message);
                throw error;
            }
        });
    });
});