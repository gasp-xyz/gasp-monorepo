/* eslint-disable @typescript-eslint/no-non-null-assertion */
import { chai, describe, expect, it } from "vitest";
chai.should()
import supertest from "supertest";
import app from "../../src/app";
import Joi from 'joi';
import moment from 'moment'
import { BN } from '@polkadot/util'
import { BN_TEN } from 'gasp-sdk'

const apySchema =
    Joi.object({
        5: Joi.object({
            date: Joi.string().required(),
            apy: Joi.number().precision(18).unsafe().required(),
            collatorAddress: Joi.string().required(),
            dateFormat: Joi.string().required(),
            timestamp: Joi.date().timestamp().required()
        })}
    );
const dailyRewardSchema =
    Joi.array().items(

    Joi.object({
            tokenId: Joi.string().required(),
            dailyRewards: Joi.number().precision(18).unsafe().required(),
            dateFormat: Joi.string().required(),
            date: Joi.string().required(),
            timestamp: Joi.date().timestamp().required(),
        }
    )
    )

function validateReturnedDate(date :string, format : string) {
    const dateFormat = moment(date, format, true);
    const dateFromMoment = dateFormat.toDate();
    expect(dateFromMoment.getDate()).to.equal(parseInt(date.split('/')[0]))
    expect(dateFromMoment.getMonth() +1 ).to.equal(parseInt(date.split('/')[1]))
    expect(dateFromMoment.getFullYear()).to.equal(parseInt(date.split('/')[2]))
}

//perhaps those addresses may change when data is upgraded!

// const oldCollatorAddress = "5EtT1Psa48f9KSNMZqZgwPj6guNULVeRUzcA9w1Kk8DkP9iZ";
const collatorAddress =  "0xf24ff3a9cf04c71dbc94d0b566f7a27b94566cac";

function validateValidApyRange(body) {
    const apy = parseFloat(body.apy)
    expect(apy).to.be.lessThanOrEqual(40)
    expect(apy).to.be.greaterThan(10)
}

describe('APi tests: Collator apy - dailyRewards', () => { //todo: remove skip once we know new collator addresses
    it("GET /collators/dailyReward - no data", async () => {
        await supertest(app)
            .get("/collator/foo/staking/dailyreward")
            .expect(404)
            .then((response) => {
                expect(response.body.message).to.equal("This collator has not received any rewards as of yet.")
                expect(response.body.exceptionName).to.equal("NotFoundException")
            });
    })
    it("GET /collators/apy - no data", async () => {
        await supertest(app)
            .get("/collator/foo/staking/apy")
            .expect(404)
            .then((response) => {
                expect(response.body.message).to.equal("This collator has not received any rewards as of yet.")
                expect(response.body.exceptionName).to.equal("NotFoundException")
            });
    })
    it.skip("GET /collators/apy - collator - OK", async () => {
        await supertest(app)
            .get(`/collator/${collatorAddress}/staking/apy`)
            .expect(200)
            .then((response) => {
                const validationResult = apySchema.validate(response.body);
                const body = JSON.parse(JSON.stringify(response.body))[5];
                const date =  body.date; //body.5.date;
                const format = body.dateFormat ; //response.body.5.dateFormat;
                expect(validationResult.error).toBeUndefined();
                validateReturnedDate(date, format);
                validateValidApyRange(body);
            });
    })
    it.skip("GET /collators/dailyReward - collator - OK", async () => {
        await supertest(app)
            .get(`/collator/${collatorAddress}/staking/dailyReward`)
            .expect(200)
            .then((response) => {
                const validationResult = dailyRewardSchema.validate(response.body);
                expect(validationResult.error).toBeUndefined()
                const date =  response.body[0].date;
                const format = response.body[0].dateFormat ;
                expect(response.body[0].tokenId).to.equal("5");
                validateReturnedDate(date,format);
                const reward = response.body[0].dailyRewards ;
                const rewardValue = new BN(reward).div(BN_TEN.pow(new BN(18)))
                expect(rewardValue.toNumber()).gt(5000 );
            });
    })
    it.skip("GET /collators/apy - old - collator - OK", async () => { //todo: gonzalo to check
        await supertest(app)
            .get(`/collator/${ethCollatorAddress}/staking/apy`)
            // .expect(200)
            .then((response) => {
              console.log(response)
                const validationResult = apySchema.validate(response.body);
                const body = JSON.parse(JSON.stringify(response.body))[5];
                const date =  body.date; //body.5.date;
                const format = body.dateFormat ; //response.body.5.dateFormat;
                expect(validationResult.error).toBeUndefined();
                validateReturnedDate(date, format);

                // special validation about old collators should have old date.
                expect(date).to.equal('15/12/2023');
                validateValidApyRange(body);
            });
    })
    it.skip("GET /collators/dailyReward - old - collator - OK", async () => { //todo: gonzalo to check
        await supertest(app)
            .get(`/collator/${oldCollatorAddress}/staking/dailyReward`)
            .expect(200)
            .then((response) => {
                const validationResult = dailyRewardSchema.validate(response.body);
                const body = JSON.parse(JSON.stringify(response.body))[0];
                const date =  body.date; //body.5.date;
                const format = body.dateFormat ; //response.body.5.dateFormat;
                expect(validationResult.error).toBeUndefined();
                validateReturnedDate(date, format);

                // special validation about old collators should have old date.
                expect(date).to.equal('15/12/2023');
                const reward = response.body[0].dailyRewards ;
                const rewardValue = new BN(reward).div(BN_TEN.pow(new BN(18)))
                expect(rewardValue.toNumber()).gt(5000 );
            });
    })

})
