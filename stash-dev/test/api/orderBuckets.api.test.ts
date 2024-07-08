/* eslint-disable @typescript-eslint/no-non-null-assertion */
import { beforeAll, chai, describe, expect, it } from "vitest";
chai.should()
import supertest from "supertest";
import Joi from 'joi';
import mangataNode from "../../src/connector/MangataNode";
import app from "../../src/app";
import { redis } from "../../src/connector/RedisConnector";

const bucketsSchema =
    Joi.object({
        buckets: Joi.array().items(
            Joi.object({
                bucket: Joi.string().required(),
                rank: Joi.number().required(),
                tokens: Joi.array().min(0).items(
                    Joi.string().optional()
                ),
            })
        ),
    });


beforeAll(async () => {
    // remove all the inserts when new snapshot is created
    await new Promise((f) => setTimeout(f, 1000))
    const TOKEN_ORDER_BUCKETS_KEY = 'token_order_buckets'
    await redis.client.hset(TOKEN_ORDER_BUCKETS_KEY, { ["stables"]: JSON.stringify({"bucket":"stables","rank":1,"tokens":["USDT","USDC","aUSD"]}) })
    await redis.client.hset(TOKEN_ORDER_BUCKETS_KEY, { ["bluechips"]: JSON.stringify({ "bucket": "bluechips", "rank": 2, "tokens": [ "BTC", "ETH" ] }) })
    await redis.client.hset(TOKEN_ORDER_BUCKETS_KEY, { ["l0"]: JSON.stringify({ "bucket": "l0", "rank": 3, "tokens": [ "DOT", "KSM" ] }) })
    await redis.client.hset(TOKEN_ORDER_BUCKETS_KEY, { ["dextoken"]: JSON.stringify({ "bucket": "dextoken", "rank": 4, "tokens": [ "MGA", "MGX" ] }) })
    await redis.client.hset(TOKEN_ORDER_BUCKETS_KEY, { ["l1"]: JSON.stringify({ "bucket": "l1", "rank": 5, "tokens": [ "MOVR", "BNC", "OAK", "TUR", "IMBU", "ZLK", "RMRK" ] }) })
    await redis.client.hset(TOKEN_ORDER_BUCKETS_KEY, { ["l2"]: JSON.stringify({ "bucket": "l2", "rank": 6, "tokens": [] }) })
    await redis.client.hset(TOKEN_ORDER_BUCKETS_KEY, { ["protocols"]: JSON.stringify({ "bucket": "protocols", "rank": 7, "tokens": [] }) })
    await redis.client.hset(TOKEN_ORDER_BUCKETS_KEY, { ["derivatives"]: JSON.stringify({ "bucket": "derivatives", "rank": 8, "tokens": [ "vKSM", "vsKSM", "vMOVR", "vBNC" ] }) })
    await new Promise((f) => setTimeout(f, 2000))
});

describe('APi tests: Buckets', () => {
    it("GET token/order-buckets - Schema validation", async () => {
        await supertest(app)
            .get("/token/order-buckets")
            .expect(200)
            .then((response) => {
                const validationResult = bucketsSchema.validate(response.body);
                expect(validationResult.error).toBeUndefined();
            });
    });
    it("GET token/order-buckets - Every token is listed", async () => {
        //now using production, but perhaps using a local setup or chops would help.
        await supertest(app)
            .get("/token/order-buckets")
            .expect(200)
            .then(async(response) => {
                const allTokens = await ( await mangataNode.api()).query.tokens.totalIssuance.entries();
                const liqTokens = await ( await mangataNode.api()).query.xyk.liquidityAssets.entries();
                //Exclude liquidity tokens from all tokens
                const onlyAssets =
                    allTokens
                        //Exclude liquidity tokens from all tokens
                        .filter((token) =>
                            !liqTokens
                                .find((liqToken) =>
                                    liqToken[1].toString() === token[0].toHuman()[0].toString()))
                        //Exclude tokens with 0 balance
                        .filter((token) => token[1].toHuman() !== "0")
                        //Exclude tokens with id 2 (dummy token)
                        .filter((token) => token[0].toHuman()[0].toString() !== "2")
                        .map( (token) => token[0].toHuman()[0].toString());
                console.log(onlyAssets);
                for (let i = 0; i < onlyAssets.length ; i++) {
                    const tokenInfo = ( await mangataNode.query.getTokenInfo(onlyAssets[i]));
                    console.log("Validating :" + JSON.stringify(tokenInfo)  + " :: "  + onlyAssets[i]);
                    if(tokenInfo.name.includes("Liquidity")){
                        console.log("Skipping liquidity token: " + tokenInfo.id)
                        continue;
                    }
                    const found = response.body.buckets.filter((bucket: { tokens: string | string[]; }) => bucket.tokens.includes(tokenInfo.symbol));
                    console.log("Validating :" + tokenInfo.symbol );
                    expect(found).toBeDefined();

                    //tokens must only exist on one bucket.
                    expect(found.length).toBe(1);
                }
            })
    })
})
