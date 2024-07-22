/* eslint-disable @typescript-eslint/no-non-null-assertion */
import { chai, describe, expect, it } from "vitest";
chai.should()
import supertest from "supertest";
import app from "../../src/app";
import Joi from 'joi';
import MangataClient from "../../src/connector/MangataNode";

const tokenSchema = Joi.object({
    tokenId: Joi.string().required(),
    tokenName: Joi.string().required(),
    symbol: Joi.string().required(),
    priceInUSD: Joi.number().precision(18).unsafe().required(),
    volume24hInUSD: Joi.number().precision(18).unsafe().required(),
    liquidity24hInUSD: Joi.number().precision(18).unsafe().required(),
    priceChange24hInPerc: Joi.number().precision(18).unsafe().required(),
    volumeChange24hInPerc: Joi.number().precision(18).unsafe().required(),
});

const listSchema = Joi.array().items(
    tokenSchema
);
//Values can not be tested, since data is old, and this api request l&g values.
describe('APi tests: token stats', () => {
    it("GET /token/list/stats - Schema validation", async () => {
        await supertest(app)
            .get("/token/list/stats")
            .expect(200)
            .then((response) => {
                const validationResult = listSchema.validate(response.body);
                expect(validationResult.error).toBeUndefined();
            });
    })
    it("GET /token/GASPV2/stats - Schema validation", async () => {
        await supertest(app)
            .get("/token/GASPV2/stats")
            .expect(200)
            .then((response) => {
                const validationResult = tokenSchema.validate(response.body);
                expect(validationResult.error).toBeUndefined();
            });
    });
    it("GET list & GET token stat info matches", async () => {
        await supertest(app)
            .get("/token/GASPV2/stats")
            .expect(200)
            .then(async(response) => {
                const allTokens = (await supertest(app).get("/token/list/stats").expect(200)).body;
                const gaspV2Token = response.body;
                const onlyKsm = allTokens.filter( (token: { tokenId: any; }) => token.tokenId === gaspV2Token.tokenId);
                expect(onlyKsm.length).toEqual(1);
                expect(onlyKsm[0]).toEqual(gaspV2Token);
            })
    })
    it("GET /token/list/stats - List matches with all the tokens with pool", async () => {
        const sdk = MangataClient;
        const api = await sdk.api();
        const pools = await api.query.xyk.liquidityPools.entries();
        const allstats = (await supertest(app)
            .get("/token/list/stats")).body;
        pools.forEach( (pool) => {
            const firstTokenId = pool[1].toHuman()[0];
            const secondTokenId = pool[1].toHuman()[1];
            const excludePool = [ "6" , "7" , "8"]; //todo: i changed this to leave only pool with 0 1 token ids (geth and gaspv2)
            const firstToken = allstats.filter( (token: { tokenId: any; }) => token.tokenId === firstTokenId );
            const secondToken = allstats.filter( (token: { tokenId: any; }) => token.tokenId === secondTokenId );
            console.log( `Token ${firstTokenId} , ${secondTokenId} `)
            if (!excludePool.includes(firstTokenId) ||  !excludePool.includes(secondTokenId) ) {
                expect(firstToken.length).toEqual(1);
                expect(secondToken.length).toEqual(1);
            }else{
                console.log( `Skipped:: ${firstTokenId} , ${secondTokenId} `)
            }
        });
    })
})

describe('API Errors: Non existing token', () => {
    it("GET non existing token", async () => {
        await supertest(app)
            .get("/token/Hello/stats")
            .expect(404)
    })
});

