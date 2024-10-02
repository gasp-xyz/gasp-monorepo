/* eslint-disable @typescript-eslint/no-non-null-assertion */
import { describe, expect, it } from "vitest";
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
                const onlyGaspV2 = allTokens.filter( (token: { tokenId: any; }) => token.tokenId === gaspV2Token.tokenId);
                expect(onlyGaspV2.length).toEqual(1);
                expect(onlyGaspV2[0]).toEqual(gaspV2Token);
            })
    })
    it("GET /token/list/stats - List matches with all the tokens with pool", async () => {
        const sdk = MangataClient;
        const api = await sdk.api();
        const pools = await api.query.xyk.liquidityPools.entries();
        console.log('Pools:', pools);
        const allstats = (await supertest(app)
            .get("/token/list/stats")).body;
        pools.forEach( (pool) => {
            const firstTokenId = pool[1].toHuman()[0];
            const secondTokenId = pool[1].toHuman()[1];
            const excludePool = [ "2" , "3", "9", "15", "16"]; // leave only pool with 0 1 token ids (eth and gaspv2)
            const firstToken = allstats.filter( (token: { tokenId: any; }) => token.tokenId === firstTokenId );
            const secondToken = allstats.filter( (token: { tokenId: any; }) => token.tokenId === secondTokenId );
            console.log( `Token ${firstTokenId} , ${secondTokenId} `)
            if (excludePool.includes(firstTokenId) ||  excludePool.includes(secondTokenId) ) {
                console.log( `Skipped:: ${firstTokenId} , ${secondTokenId} `)

            }
            else{
                expect(firstToken.length).toEqual(1);
                expect(secondToken.length).toEqual(1);
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

