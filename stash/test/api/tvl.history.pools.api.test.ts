import { chai, describe, expect, it } from "vitest";
chai.should()
import supertest from "supertest";
import app from "../../src/app";
import { MAX_DAYS, MAX_INTERVAL } from "./utils";

const ERROR_MSG_POOL_NOT_FOUND = "this must be one of the following values: GASPV2-L1Asset, L1Asset-GASPV2, GASPV2-L1Asset-GASPV2, L1Asset-GASPV2-GASPV2, ALL";

    describe('APi tests: tvl-history/pools', () => {
        const testPool = "GASPV2-L1Asset"
        const testPoolReversed = "L1Asset-GASPV2"
        it("GET pools/GASPV2-L1Asset returns the same as pools/L1Asset-GASPV2 -> Expect deep equal", async () => {
            const gaspv2L1Asset = await supertest(app)
                .get("/tvl-history/pools/" + testPool)
                .query({
                    interval: MAX_INTERVAL,
                    days: MAX_DAYS
                })
                .expect(200)
            const l1AssetGaspv2 = await supertest(app)
                .get("/tvl-history/pools/" + testPoolReversed)
                .query({
                    interval: MAX_INTERVAL,
                    days: MAX_DAYS
                })
                .expect(200)
            expect(gaspv2L1Asset.body).to.deep.equal(l1AssetGaspv2.body);
            expect(gaspv2L1Asset.body).to.have.property("volumes");
            expect(gaspv2L1Asset.body.volumes).to.be.an("array");
            expect(gaspv2L1Asset.body.volumes[0]).to.have.a.lengthOf(2);
        })
    })

    // These tests will fail if images changes And/Or if bugfixes. Careful when updating!
    describe.todo('Snapshots tests: tvl-history/pools', () => {
        //more tests will come...
    })
    describe('API Errors: tvl-history/pools', () => {
        it("GET pools/foo does not exist Expect validation error", async () => {
            await supertest(app)
                .get("/tvl-history/pools/foo")
                .query({
                    interval: MAX_INTERVAL,
                    days: MAX_DAYS
                })
                .expect(500)
                .then((response) => {
                    const fooResponse = response.body;
                    expect(fooResponse.exceptionName).to.contain("ValidationError")
                    expect(fooResponse.message).to.contain(ERROR_MSG_POOL_NOT_FOUND)
                });
        })

    });
    describe.todo('System Errors: tvl-history/pools', () => {
        //more tests will come...
    });

