import { describe, beforeAll, expect, it , vi} from "vitest";
import { hexToU8a } from "@polkadot/util";
import { Deposit } from "../src/Deposit.js";
import { L2Interface } from "../src/l2/L2Interface.js";
import { L1Interface } from "../src/l1/L1Interface.js";
import { Ferry } from "../src/Ferry.js";


const ALITH = "0xf24ff3a9cf04c71dbc94d0b566f7a27b94566cac";
const DUMMY_TOKEN = hexToU8a("0x1111111111111111111111111111111111111111");
const NATIVE_TOKEN = hexToU8a("0x2222222222222222222222222222222222222222");
const TOKENS_TO_TRACK: [Uint8Array, bigint, bigint][] = [[DUMMY_TOKEN, 0n, 1n]];

let ferry: Ferry;
let l1Mock: L1Interface;
let l2Mock: L2Interface;

describe('Ferry Service', () => {

  beforeAll(async () => {

    l1Mock = {
        isRolldownDeployed: vi.fn().mockImplementation(() => {throw new Error("Unexpcted mock called")}),
        getLatestRequestId: vi.fn().mockImplementation(() => {throw new Error("Unexpcted mock called")}),
        getDeposits: vi.fn().mockImplementation(() => {throw new Error("Unexpcted mock called")}),
        getDepostiHash: vi.fn().mockImplementation(() => {throw new Error("Unexpcted mock called")}),
    };

    l2Mock = {
        getBalance: vi.fn().mockImplementation(() => {throw new Error("Unexpcted mock called")}),
        getLastProcessedRequestId: vi.fn().mockImplementation(() => {throw new Error("Unexpcted mock called")}),
        isExecuted: vi.fn().mockImplementation(() => {throw new Error("Unexpcted mock called")}),
        isFerried: vi.fn().mockImplementation(() => {throw new Error("Unexpcted mock called")}),
        getNativeTokenAddress: vi.fn().mockResolvedValue(NATIVE_TOKEN),
        valutateToken: vi.fn().mockImplementation( (_address, amount) => amount),
    };
    ferry = new Ferry(hexToU8a(ALITH), l1Mock, l2Mock, 0n, TOKENS_TO_TRACK);

  });

  it('works fine when no deposits to ferry', async () => {

    l1Mock.getLatestRequestId = vi.fn().mockResolvedValue(0n);
    l2Mock.getLastProcessedRequestId = vi.fn().mockResolvedValue(0n);
    l1Mock.getDeposits = vi.fn().mockResolvedValue([]);

    expect(await ferry.getPendingDeposits()).toHaveLength(0);
  });


  it('ignores ferried deposits', async () => {

    const deposits: Deposit[] = [
      {
        requestId: 1n,
        depositRecipient: new Uint8Array(20).fill(1),
        tokenAddress: new Uint8Array(20).fill(1),
        amount: 1n, 
        timeStamp: 1n,
        ferryTip: 0n,
      }
    ];

    l1Mock.getLatestRequestId = vi.fn().mockResolvedValue(0n);
    l2Mock.getLastProcessedRequestId = vi.fn().mockResolvedValue(0n);
    l1Mock.getDeposits = vi.fn().mockResolvedValue(deposits);
    l1Mock.getDepostiHash = vi.fn().mockResolvedValue(new Uint8Array(32));
    l2Mock.isExecuted = vi.fn().mockResolvedValue(false);
    l2Mock.isFerried = vi.fn().mockResolvedValue(true);

    expect(await ferry.getPendingDeposits()).toHaveLength(0);
  });


  it('ignores executed deposits', async () => {

    const deposits: Deposit[] = [
      {
        requestId: 1n,
        depositRecipient: new Uint8Array(20).fill(1),
        tokenAddress: new Uint8Array(20).fill(1),
        amount: 1n, 
        timeStamp: 1n,
        ferryTip: 0n,
      }
    ];

    l1Mock.getLatestRequestId = vi.fn().mockResolvedValue(0n);
    l2Mock.getLastProcessedRequestId = vi.fn().mockResolvedValue(0n);
    l1Mock.getDeposits = vi.fn().mockResolvedValue(deposits);
    l1Mock.getDepostiHash = vi.fn().mockResolvedValue(new Uint8Array(32));
    l2Mock.isExecuted = vi.fn().mockResolvedValue(true);
    l2Mock.isFerried = vi.fn().mockResolvedValue(false);

    expect(await ferry.getPendingDeposits()).toHaveLength(0);
  });

  it('considers only not ferried and not executed deposits', async () => {

    const deposits: Deposit[] = [
      {
        requestId: 1n,
        depositRecipient: new Uint8Array(20).fill(1),
        tokenAddress: new Uint8Array(20).fill(1),
        amount: 1n, 
        timeStamp: 1n,
        ferryTip: 0n,
      }
    ];

    l1Mock.getLatestRequestId = vi.fn().mockResolvedValue(0n);
    l2Mock.getLastProcessedRequestId = vi.fn().mockResolvedValue(0n);
    l1Mock.getDeposits = vi.fn().mockResolvedValue(deposits);
    l1Mock.getDepostiHash = vi.fn().mockResolvedValue(new Uint8Array(32));
    l2Mock.isExecuted = vi.fn().mockResolvedValue(false);
    l2Mock.isFerried = vi.fn().mockResolvedValue(false);

    expect(await ferry.getPendingDeposits()).toHaveLength(1);
  });

  it('rates deposits based on fee', async () => {
    l2Mock.getBalance = vi.fn().mockResolvedValue([[DUMMY_TOKEN, 1000n]]);

    const depositWithFee: Deposit = {
      requestId: 1n,
      depositRecipient: new Uint8Array(20).fill(1),
      tokenAddress: DUMMY_TOKEN,
      amount: 10n, 
      timeStamp: 1n,
      ferryTip: 10n,
    };

    const depositWithoutFee: Deposit = {
      requestId: 2n,
      depositRecipient: new Uint8Array(20).fill(1),
      tokenAddress: DUMMY_TOKEN,
      amount: 10n, 
      timeStamp: 1n,
      ferryTip: 0n,
    };

    expect(await ferry.rateDeposits(
      [depositWithoutFee, depositWithFee],
    )).toStrictEqual([depositWithFee, depositWithoutFee]);

  });

  it('rates deposits based on fee & amount', async () => {
    l2Mock.getBalance = vi.fn().mockResolvedValue([[DUMMY_TOKEN, 1000n]]);

    const depositWithFeeAndLeastTranferValue: Deposit = {
      requestId: 1n,
      depositRecipient: new Uint8Array(20).fill(1),
      tokenAddress: DUMMY_TOKEN,
      amount: 10n, 
      timeStamp: 1n,
      ferryTip: 10n,
    };


    const depositWithFee: Deposit = {
      requestId: 1n,
      depositRecipient: new Uint8Array(20).fill(1),
      tokenAddress: DUMMY_TOKEN,
      amount: 20n, 
      timeStamp: 1n,
      ferryTip: 10n,
    };

    const depositWithoutFee: Deposit = {
      requestId: 2n,
      depositRecipient: new Uint8Array(20).fill(1),
      tokenAddress: DUMMY_TOKEN,
      amount: 1n, 
      timeStamp: 1n,
      ferryTip: 0n,
    };

    expect(await ferry.rateDeposits(
      [depositWithoutFee, depositWithFeeAndLeastTranferValue, depositWithFee],
    )).toStrictEqual([depositWithFeeAndLeastTranferValue, depositWithFee, depositWithoutFee]);

  });

  it('filters out deposits that can not afford', async () => {
    l2Mock.getBalance = vi.fn().mockResolvedValue([[DUMMY_TOKEN, 10n]]);


    const depositWithTooMuchAmount: Deposit = {
      requestId: 1n,
      depositRecipient: new Uint8Array(20).fill(1),
      tokenAddress: DUMMY_TOKEN,
      amount: 11n, 
      timeStamp: 1n,
      ferryTip: 0n,
    };

    const depositWithTooMuchAmount2: Deposit = {
      requestId: 2n,
      depositRecipient: new Uint8Array(20).fill(1),
      tokenAddress: DUMMY_TOKEN,
      amount: 25n, 
      timeStamp: 1n,
      ferryTip: 0n,
    };

    expect(await ferry.rateDeposits(
      [depositWithTooMuchAmount, depositWithTooMuchAmount2]
    )).toStrictEqual([]);

  });

  it('accepts the deposits that can be afforded XXX', async () => {
    l2Mock.getBalance = vi.fn().mockResolvedValue([[DUMMY_TOKEN, 10n]]);


    const depositWithTooMuchAmount: Deposit = {
      requestId: 1n,
      depositRecipient: new Uint8Array(20).fill(1),
      tokenAddress: DUMMY_TOKEN,
      amount: 11n, 
      timeStamp: 1n,
      ferryTip: 0n,
    };

    const depositWihoutFee: Deposit = {
      requestId: 2n,
      depositRecipient: new Uint8Array(20).fill(1),
      tokenAddress: DUMMY_TOKEN,
      amount: 10n, 
      timeStamp: 1n,
      ferryTip: 0n,
    };

    const depositWihFee: Deposit = {
      requestId: 2n,
      depositRecipient: new Uint8Array(20).fill(1),
      tokenAddress: DUMMY_TOKEN,
      amount: 25n, 
      timeStamp: 1n,
      ferryTip: 15n,
    };


    expect(await ferry.rateDeposits(
      [depositWithTooMuchAmount, depositWihoutFee, depositWihFee]
    )).toStrictEqual([depositWihFee, depositWihoutFee]);

  });

  it('accepts the deposits that can be afforded', async () => {
    l2Mock.getBalance = vi.fn().mockResolvedValue([[DUMMY_TOKEN, 10n]]);


    const depositWithTooMuchAmount: Deposit = {
      requestId: 1n,
      depositRecipient: new Uint8Array(20).fill(1),
      tokenAddress: DUMMY_TOKEN,
      amount: 11n, 
      timeStamp: 1n,
      ferryTip: 0n,
    };

    const depositWihoutFee: Deposit = {
      requestId: 2n,
      depositRecipient: new Uint8Array(20).fill(1),
      tokenAddress: DUMMY_TOKEN,
      amount: 10n, 
      timeStamp: 1n,
      ferryTip: 0n,
    };

    const depositWihFee: Deposit = {
      requestId: 2n,
      depositRecipient: new Uint8Array(20).fill(1),
      tokenAddress: DUMMY_TOKEN,
      amount: 25n, 
      timeStamp: 1n,
      ferryTip: 15n,
    };


    expect(await ferry.rateDeposits(
      [depositWithTooMuchAmount, depositWihoutFee, depositWihFee]
    )).toStrictEqual([depositWihFee, depositWihoutFee]);

  });

  it('ignores invalid deposits', async () => {
    l2Mock.getBalance = vi.fn().mockResolvedValue([[DUMMY_TOKEN, 10n]]);

    const invalidDeposit: Deposit = {
      requestId: 1n,
      depositRecipient: new Uint8Array(20).fill(1),
      tokenAddress: DUMMY_TOKEN,
      amount: 10n, 
      timeStamp: 1n,
      ferryTip: 100n,
    };


    expect(await ferry.rateDeposits(
      [invalidDeposit]
    )).toStrictEqual([]);
  });


  it('tx cost applies only for native deposits', async () => {
    const txCost = 100n;
    ferry = new Ferry(hexToU8a(ALITH), l1Mock, l2Mock, txCost, [[DUMMY_TOKEN, 0n, 1n], [NATIVE_TOKEN, 0n, 1n]]);

    l2Mock.getBalance = vi.fn().mockResolvedValue([
      [NATIVE_TOKEN, 200n],
      [DUMMY_TOKEN, 10n]
    ]);

    const nativeDeposit: Deposit = {
      requestId: 1n,
      depositRecipient: new Uint8Array(20).fill(1),
      tokenAddress: NATIVE_TOKEN,
      amount: 200n, 
      timeStamp: 1n,
      ferryTip: 99n,
    };

    const nativeDeposit2: Deposit = {
      requestId: 1n,
      depositRecipient: new Uint8Array(20).fill(1),
      tokenAddress: NATIVE_TOKEN,
      amount: 200n, 
      timeStamp: 1n,
      ferryTip: 100n,
    };

    const dummyDeposit: Deposit = {
      requestId: 1n,
      depositRecipient: new Uint8Array(20).fill(1),
      tokenAddress: DUMMY_TOKEN,
      amount: 10n, 
      timeStamp: 1n,
      ferryTip: 1n,
    };

    const result = await ferry.rateDeposits(
      [nativeDeposit, nativeDeposit2, dummyDeposit]
    );

    expect(result).toHaveLength(2);
    expect(result).toStrictEqual([nativeDeposit2, dummyDeposit]);

  });


  it('highest reward:input ratio deposits are preffered', async () => {

    l2Mock.getBalance = vi.fn().mockResolvedValue( [[DUMMY_TOKEN, 10000n]]);

    const depositBetter: Deposit = {
      requestId: 1n,
      depositRecipient: new Uint8Array(20).fill(1),
      tokenAddress: DUMMY_TOKEN,
      amount: 1000n, 
      timeStamp: 1n,
      ferryTip: 100n,
    };

    const depositWorse: Deposit = {
      requestId: 1n,
      depositRecipient: new Uint8Array(20).fill(1),
      tokenAddress: DUMMY_TOKEN,
      amount: 1500n, 
      timeStamp: 1n,
      ferryTip: 100n,
    };

    expect(await ferry.rateDeposits(
      [depositWorse, depositBetter],
    )).toStrictEqual([depositBetter, depositWorse]);

  });

  it('ignores deposits with fee valuation lower than min', async () => {

    const tokensToTrack:[Uint8Array, bigint, bigint][] = [[DUMMY_TOKEN, 100n, 1n]];
    ferry = new Ferry(hexToU8a(ALITH), l1Mock, l2Mock, 0n, tokensToTrack);
    l2Mock.getBalance = vi.fn().mockResolvedValue([[DUMMY_TOKEN, 10000n]]);

    const depositBelowProfitThreshold: Deposit = {
      requestId: 1n,
      depositRecipient: new Uint8Array(20).fill(1),
      tokenAddress: DUMMY_TOKEN,
      amount: 1000n, 
      timeStamp: 1n,
      ferryTip: 99n,
    };

    const depositAboveProfitThreshold: Deposit = {
      requestId: 1n,
      depositRecipient: new Uint8Array(20).fill(1),
      tokenAddress: DUMMY_TOKEN,
      amount: 1500n, 
      timeStamp: 1n,
      ferryTip: 100n,
    };

    expect(await ferry.rateDeposits(
      [depositBelowProfitThreshold, depositAboveProfitThreshold],
    )).toStrictEqual([depositAboveProfitThreshold]);

  });

})
