import { chai, describe, expect, it } from "vitest";
chai.should()
import supertest from "supertest";
import app from "../../src/app";
import { MAX_DAYS, MAX_INTERVAL } from "./utils";
import Joi from "joi";

const ERROR_MSG_ASSET_NOT_FOUND = "this must be one of the following values: GASPV2, GETH, L1Asset";
const pricesSchema =
Joi.object({
    prices: Joi.array().items(
        Joi.array().items(
            Joi.number().required(),
            Joi.string().required(),
        )
    )
})

    describe('APi tests: price-history/pair', () => {
        const pair = "GASPV2/L1Asset"
        const reversedPair = "L1Asset/GASPV2"
        it("GET pair GASPV2/L1Asset validate schema", async () => {
            const ksmMgx = await supertest(app)
                .get("/price-history/pair/" + pair)
                .query({
                    interval: MAX_INTERVAL,
                    days: MAX_DAYS
                })
                .expect(200)
            const validationResult = pricesSchema.validate(ksmMgx.body);
            expect(validationResult.error).toBeUndefined();

        })
        it("GET pair GASPV2/L1Asset returns same as pair L1Asset/GASPV2", async () => {
            const ksmMgx = await supertest(app)
                .get("/price-history/pair/" + pair)
                .query({
                    interval: MAX_INTERVAL,
                    days: MAX_DAYS
                })
                .expect(200)
            const mgxKsm = await supertest(app)
                .get("/price-history/pair/" + reversedPair)
                .query({
                    interval: MAX_INTERVAL,
                    days: MAX_DAYS
                })
                .expect(200)
            expect(ksmMgx.body.prices).toHaveLength(mgxKsm.body.prices.length);
            const timestamps1 = ksmMgx.body.prices.map((p: any) => p[0]);
            const timestamps2 = mgxKsm.body.prices.map((p: any) => p[0]);
            expect(timestamps1).toStrictEqual(timestamps2);
        })
    })

    describe('API Errors: price-history/pair', () => {
        it("GET pools/foo: token does not exist Expect validation error", async () => {
            await supertest(app)
                .get("/price-history/pair/L1Asset/foo")
                .query({
                    interval: MAX_INTERVAL,
                    days: MAX_DAYS
                })
                .expect(500)
                .then((response) => {
                    const fooResponse = response.body;
                    expect(fooResponse.exceptionName).to.contain("ValidationError")
                    expect(fooResponse.message).to.contain(ERROR_MSG_ASSET_NOT_FOUND)
                });
        })
        it("GET pools/vsKSM/RMRK: pool that does not exist expect empty", async () => {
            await supertest(app)
                .get("/price-history/pair/vsKSM/RMRK")
                .query({
                    interval: MAX_INTERVAL,
                    days: MAX_DAYS
                })
                .expect(200)
                .then((response) => {
                    const poolDoesNotExist = response.body;
                    expect(poolDoesNotExist.prices).to.be.empty; //todo: ovde dobijam validation error: {"exceptionName":"ValidationError","message":"this must be one of the following values: GASPV2, GETH, L1Asset"}
                });
        })
    });


