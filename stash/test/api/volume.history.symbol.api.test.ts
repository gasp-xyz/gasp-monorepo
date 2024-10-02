import { describe, expect, it } from "vitest";
import supertest from "supertest";
import app from "../../src/app";
import { ERROR_MSG_PAIR_ASSET_NOT_FOUND } from "./utils";

    describe('APi tests: volume-history/', () => {
        const tokens = ["GASPV2", "L1Asset", "GASPV2-ETH", "L1Asset-GASPV2"];
        it.each(tokens)("should return volumes for supported pools: %s", async (token) => {
            const response = await supertest(app)
                .get("/volume-history/" + token)
                .expect(200);
                expect(response.body).to.have.property("volumes");
                expect(response.body.error).toBeUndefined();
                expect(response.body.volumes).toBeDefined();
                expect(response.body.volumes).toBeInstanceOf(Array);
        })
    })

    describe('API Errors: volume-history/', () => {
        it("GET pools/foo does not exist -> Expect validation error", async () => {
            await supertest(app)
                .get("/volume-history/foo")
                .expect(500)
                .then((response) => {
                    const fooResponse = response.body;
                    expect(fooResponse.exceptionName).to.contain("ValidationError")
                    expect(fooResponse.message).to.contain(ERROR_MSG_PAIR_ASSET_NOT_FOUND)
                });
        })
    });