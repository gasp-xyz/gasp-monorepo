import { chai, describe, expect, it } from "vitest";
chai.should()
import supertest from "supertest";
import app from "../../src/app";
import { MAX_DAYS, MAX_INTERVAL } from "./utils";

const ERROR_MSG_POOL_NOT_FOUND = "this must be one of the following values: KSM-MGX, MGX-KSM, MGX-TUR, TUR-MGX";

    describe.skip('APi tests: tvl-history/pools', () => {
        const testPool = "KSM-MGX"
        const testPoolReversed = "MGX-KSM"
        it("GET pools/MGX-KSM returns the same as pools/KSM-MGX -> Expect deep equal", async () => {
            const ksmMgx = await supertest(app)
                .get("/tvl-history/pools/" + testPool)
                .query({
                    interval: MAX_INTERVAL,
                    days: MAX_DAYS
                })
                .expect(200)
            const mgxKsm = await supertest(app)
                .get("/tvl-history/pools/" + testPoolReversed)
                .query({
                    interval: MAX_INTERVAL,
                    days: MAX_DAYS
                })
                .expect(200)
            expect(ksmMgx.body).to.deep.equal(mgxKsm.body);
            expect(ksmMgx.body).to.have.property("volumes");
            expect(ksmMgx.body.volumes).to.be.an("array");
            expect(ksmMgx.body.volumes[0]).to.have.a.lengthOf(2);
        })
    })

    // These tests will fail if images changes And/Or if bugfixes. Careful when updating!
    describe.todo('Snapshots tests: tvl-history/pools', () => {
        //more tests will come...
    })
    describe.skip('API Errors: tvl-history/pools', () => {
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

