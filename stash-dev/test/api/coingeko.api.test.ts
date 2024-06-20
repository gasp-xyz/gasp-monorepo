/* eslint-disable @typescript-eslint/no-non-null-assertion */
import { chai, describe, expect, it } from "vitest";
chai.should()
import supertest from "supertest";
import app from "../../src/app";
import Joi from 'joi';

const pairsSchema = Joi.array().items(
    Joi.object({
        ticker_id: Joi.string().required(),
        base: Joi.string().required(),
        target: Joi.string().required(),
        pool_id: Joi.number().required(),
    })
);
const tickersSchema = Joi.array().items(
    Joi.object({
        ticker_id : Joi.string().required(),
        base_currency : Joi.string().required(),
        target_currency : Joi.string().required(),
        last_price : Joi.number().precision(18).unsafe().required(),
        base_volume : Joi.number().precision(18).unsafe().required(),
        target_volume : Joi.number().precision(18).unsafe().required(),
        pool_id : Joi.number().precision(0).required(),
        liquidity_in_usd : Joi.number().precision(18).unsafe().required(),
    })
);

describe('APi tests: Coingeko', () => {
    it("GET /pairs - Schema validation", async () => {
        await supertest(app)
            .get("/coingecko/pairs")
            .expect(200)
            .then((response) => {
                const validationResult = pairsSchema.validate(response.body);
                expect(validationResult.error).toBeUndefined();
            });
    })
    it("GET /tickers - Schema validation", async () => {
        await supertest(app)
            .get("/coingecko/tickers")
            .expect(200)
            .then((response) => {
                const validationResult = tickersSchema.validate(response.body);
                expect(validationResult.error).toBeUndefined();
            })
    });
    it("GET tickers & GET pairs pool info matches", async () => {
        await supertest(app)
            .get("/coingecko/tickers")
            .expect(200)
            .then(async(response) => {
                const pairs = (await supertest(app).get("/coingecko/pairs").expect(200)).body;
                const tickers = response.body;
                expect( tickers.length).toEqual(pairs.length);
                tickers.forEach((ticker) => {
                    expect(
                        pairs.filter
                        ( (pair) =>
                            pair.pool_id === ticker.pool_id &&
                            ticker.ticker_id === pair.ticker_id &&
                            ticker.base_currency === pair.base &&
                            ticker.target_currency === pair.target
                        )
                    ).toHaveLength(1)
                });
            })
    })
})

// These tests will fail if images changes And/Or if bugfixes. Careful when updating!
// UPDATE:
//    /pairs connects to Kusama network, so toMatchSnapshot may fail from
//    time to time until we do MGX-792
describe('Snapshots tests: Coingeko', () => {
    it("GET /pairs - Schema validation", async () => {
        await supertest(app)
            .get("/coingecko/pairs")
            .expect(200)
            .then((response) => {
                expect(response.body).toMatchSnapshot("Pairs");
            });
    })
    it("GET /tickers - Schema validation", async () => {
        await supertest(app)
            .get("/coingecko/tickers")
            .expect(200)
            .then((response) => {
                response.body.sort( (a, b) => a.pool_id - b.pool_id ).forEach((ticker) => {
                    expect(ticker).toMatchSnapshot(
                        {
                            liquidity_in_usd: expect.any(String),
                            target_volume: expect.any(String),
                            base_volume: expect.any(String),
                            last_price: expect.any(String),
                        });
                })

            })
    });
})
describe('API Errors: Coingeko', () => {
    it("GET non existing path", async () => {
        await supertest(app)
            .get("/coingecko/foo")
            .expect(404)
    })
    it("POST /tickers ", async () => {
        await supertest(app)
            .post("/coingecko/tickers")
            .expect(404)
    });
});

//Need to skip this, since we re not able to stop the container.
// Unless we use a different container for each test suite we need to skip this.
// Using a different container for each test suite is not possible,
// since resolution of redis-host is done in the import.
describe.skip('System Errors: Coingeko', () => {
    it("GET /tickers  with redis down", async () => {
        // await container.stop();
        await supertest(app)
            .get("/coingecko/tickers")
            .expect(500)
    });
})
