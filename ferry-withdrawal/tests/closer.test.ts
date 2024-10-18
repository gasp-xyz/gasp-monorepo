import { describe, beforeAll, expect, it, vi, beforeEach } from "vitest";
import { L2Interface } from "../src/l2/L2Interface.js";
import { hexToU8a } from "@polkadot/util";
import { L1Interface } from "../src/l1/L1Interface.js";
import { Ferry } from "../src/Ferry.js";
import 'dotenv/config'
import { CloserService } from "../src/CloserService.js";

const ALITH = "0xf24ff3a9cf04c71dbc94d0b566f7a27b94566cac";
const NATIVE_TOKEN = hexToU8a("0x0000000000000000000000000000000000000001", 160);
const ENABLED_TOKEN = hexToU8a("0x1111111111111111111111111111111111111111", 160);
const NOT_ENABLED_TOKEN = hexToU8a("0x2222222222222222222222222222222222222222", 160);
const DEFAULT_BALANCE = 1000000n;
const TOKENS_TO_CLOSE: [Uint8Array, bigint, bigint][] = [
  [ENABLED_TOKEN, 0n, 1n],
];

let closer: CloserService;
let l1Mock: L1Interface;
let l2Mock: L2Interface;

describe('Closer Service', () => {

  beforeEach(async () => {

    l1Mock = {
      isRolldownDeployed: vi.fn().mockImplementation(() => { throw new Error("Unexpected mock called") }),
      getNativeTokenAddress: vi.fn().mockImplementation(() => NATIVE_TOKEN),
      getLatestRequestId: vi.fn().mockImplementation(() => { throw new Error("Unexpected mock called") }),
      isClosed: vi.fn().mockImplementation(() => { throw new Error("Unexpected mock called") }),
      isFerried: vi.fn().mockImplementation(() => { throw new Error("Unexpected mock called") }),
      getFerry: vi.fn().mockImplementation(() => { throw new Error("Unexpected mock called") }),
      ferry: vi.fn().mockImplementation(() => { throw new Error("Unexpected mock called") }),
      close: vi.fn().mockImplementation(() => { throw new Error("Unexpected mock called") }),
      getBalance: vi.fn().mockImplementation(() => DEFAULT_BALANCE),
    };

    l2Mock = {
      getLatestRequestId: vi.fn().mockImplementation(() => { throw new Error("Unexpcted mock called") }),
      getLatestRequestIdInPast: vi.fn().mockImplementation(() => { throw new Error("Unexpcted mock called") }),
      getWithdrawals: vi.fn().mockImplementation(() => { throw new Error("Unexpcted mock called") }),
      getNativeTokenAddress: vi.fn().mockImplementation(() => { throw new Error("Unexpected mock called") }),
    };

    closer = new CloserService(l1Mock, l2Mock, TOKENS_TO_CLOSE, 0n);
  });

  it('should fetch all withdrawalas at once from small range', async () => {

    l1Mock.getLatestRequestId = vi.fn().mockResolvedValue(123n);
    l2Mock.getWithdrawals = vi.fn().mockImplementationOnce(async (arg1, arg2) => { return []; })
    closer.findWithdrawalsToClose();


  });

  it('should fetch withdrawals in batches', async () => {

    l1Mock.getLatestRequestId = vi.fn().mockResolvedValue(123n);
    l2Mock.getWithdrawals = vi.fn().mockResolvedValue([])
    l2Mock.getWithdrawals = vi.fn().mockImplementation(async (arg1, arg2) => { return []; })

    const batchSize = 1000n;
    closer = new CloserService(l1Mock, l2Mock, TOKENS_TO_CLOSE, batchSize);

    await closer.findWithdrawalsToClose();

    expect(l2Mock.getWithdrawals).toHaveBeenCalledTimes(1);
    expect(l2Mock.getWithdrawals).toHaveBeenNthCalledWith(1, 1n, 123n);

  });

  it('should fetch withdrawals in batches 2', async () => {

    l1Mock.getLatestRequestId = vi.fn().mockResolvedValue(2000n);
    l2Mock.getWithdrawals = vi.fn().mockResolvedValue([])
    l2Mock.getWithdrawals = vi.fn().mockImplementation(async (arg1, arg2) => { return []; })

    const batchSize = 1000n;
    closer = new CloserService(l1Mock, l2Mock, TOKENS_TO_CLOSE, batchSize);

    await closer.findWithdrawalsToClose();

    expect(l2Mock.getWithdrawals).toHaveBeenCalledTimes(2);
    expect(l2Mock.getWithdrawals).toHaveBeenCalledWith(1n, 1001n);
    expect(l2Mock.getWithdrawals).toHaveBeenCalledWith(1001n, 2000n);

  });

  it('should fetch withdrawals in batches 3', async () => {

    l2Mock.getWithdrawals = vi.fn().mockResolvedValue([])
    l1Mock.getLatestRequestId = vi.fn().mockResolvedValue(2002);
    l2Mock.getWithdrawals = vi.fn().mockImplementation(async (arg1, arg2) => { return []; })

    const batchSize = 1000n;
    closer = new CloserService(l1Mock, l2Mock, TOKENS_TO_CLOSE, batchSize);
    await closer.findWithdrawalsToClose();

    expect(l2Mock.getWithdrawals).toHaveBeenCalledTimes(3);
    expect(l2Mock.getWithdrawals).toHaveBeenCalledWith(1n, 1001n);
    expect(l2Mock.getWithdrawals).toHaveBeenCalledWith(1001n, 2001n);
    expect(l2Mock.getWithdrawals).toHaveBeenCalledWith(2001n, 2002n);
  });

  it('fetches withdrawals properly when they come one by one', async () => {

    l2Mock.getWithdrawals = vi.fn().mockResolvedValue([])
    l1Mock.getLatestRequestId = vi.fn()
      .mockResolvedValueOnce(null)
      .mockResolvedValueOnce(1n);

    l2Mock.getWithdrawals = vi.fn().mockImplementation(async (arg1, arg2) => { return []; })
    closer = new CloserService(l1Mock, l2Mock, TOKENS_TO_CLOSE, 1000n);
    await closer.findWithdrawalsToClose();
    expect(l2Mock.getWithdrawals).toHaveBeenCalledTimes(0);

    await closer.findWithdrawalsToClose();
    expect(l2Mock.getWithdrawals).toHaveBeenCalledTimes(1);
    expect(l2Mock.getWithdrawals).toHaveBeenCalledWith(1n, 1n);
  });


  it('should not fetch new batch until there are some withdrawals to consume', async () => {

    l1Mock.getLatestRequestId = vi.fn().mockResolvedValue(2001n);
    l2Mock.getWithdrawals = vi.fn().mockResolvedValue([{
      requestId: 1n,
      withdrawalRecipient: hexToU8a("0x0000000000000000000000000000000000000000", 20),
      tokenAddress: ENABLED_TOKEN,
      amount: 1n,
      ferryTip: 0n,
      hash: hexToU8a("0x0000000000000000000000000000000000000000000000000000000000000000", 32),
    }])
    l1Mock.isClosed = vi.fn().mockResolvedValue(false);
    l1Mock.isFerried = vi.fn().mockResolvedValue(false);

    const batchSize = 1000n;
    closer = new CloserService(l1Mock, l2Mock, TOKENS_TO_CLOSE, batchSize);
    await closer.findWithdrawalsToClose();
    await closer.findWithdrawalsToClose();
    await closer.findWithdrawalsToClose();
    await closer.findWithdrawalsToClose();
    await closer.findWithdrawalsToClose();

    expect(l2Mock.getWithdrawals).toHaveBeenCalledTimes(1);
    expect(l2Mock.getWithdrawals).toHaveBeenCalledWith(1n, 1001n);
  });

  it('should fetch new batch once all the txs from the previous one are consumed', async () => {
    l1Mock.getLatestRequestId = vi.fn().mockResolvedValue(2001n);
    l2Mock.getWithdrawals = vi.fn().mockResolvedValue([{
      requestId: 1n,
      withdrawalRecipient: hexToU8a("0x0000000000000000000000000000000000000000", 20),
      tokenAddress: ENABLED_TOKEN,
      amount: 1n,
      ferryTip: 0n,
      hash: hexToU8a("0x0000000000000000000000000000000000000000000000000000000000000000", 32),
    }])
    l1Mock.isClosed = vi.fn().mockResolvedValue(false);
    l1Mock.isFerried = vi.fn().mockResolvedValue(false);

    const batchSize = 1000n;
    closer = new CloserService(l1Mock, l2Mock, TOKENS_TO_CLOSE, batchSize);
    await closer.findWithdrawalsToClose();
    await closer.findWithdrawalsToClose();
    await closer.findWithdrawalsToClose();
    await closer.findWithdrawalsToClose();
    await closer.findWithdrawalsToClose();


    expect(await closer.getNextWithdrawalToClose()).not.toBeNull();
    expect(await closer.getNextWithdrawalToClose()).toBeNull();

    expect(l2Mock.getWithdrawals).toHaveBeenCalledTimes(1);
    expect(l2Mock.getWithdrawals).toHaveBeenCalledWith(1n, 1001n);

    await closer.findWithdrawalsToClose();
    expect(l2Mock.getWithdrawals).toHaveBeenCalledTimes(2);
    expect(l2Mock.getWithdrawals).toHaveBeenCalledWith(1001n, 2001n);
  });

  it('do not fetch previous withdrawals once everything is processed', async () => {
    l1Mock.getLatestRequestId = vi.fn().mockResolvedValue(500n);
    l2Mock.getWithdrawals = vi.fn().mockResolvedValue([])
    l1Mock.isClosed = vi.fn().mockResolvedValue(false);
    l1Mock.isFerried = vi.fn().mockResolvedValue(false);

    const batchSize = 1000n;
    closer = new CloserService(l1Mock, l2Mock, TOKENS_TO_CLOSE, batchSize);
    await closer.findWithdrawalsToClose();
    expect(l2Mock.getWithdrawals).toHaveBeenCalledTimes(1);
    expect(l2Mock.getWithdrawals).toHaveBeenCalledWith(1n, 500n);

    await closer.findWithdrawalsToClose();
    await closer.findWithdrawalsToClose();
    await closer.findWithdrawalsToClose();
    await closer.findWithdrawalsToClose();
    expect(l2Mock.getWithdrawals).toHaveBeenCalledTimes(1);

    l1Mock.getLatestRequestId = vi.fn().mockResolvedValue(1200n);
    await closer.findWithdrawalsToClose();
    expect(l2Mock.getWithdrawals).toHaveBeenCalledTimes(2);
    expect(l2Mock.getWithdrawals).toHaveBeenCalledWith(500n, 1200n);
  });


  it('only considers enabled tokens', async () => {

    const TOKENS_TO_CLOSE: [Uint8Array, bigint, bigint][] = [
      [ENABLED_TOKEN, 0n, 1n],
    ];
    l1Mock.isClosed = vi.fn().mockResolvedValue(false);
    l1Mock.isFerried = vi.fn().mockResolvedValue(false);

    l1Mock.getLatestRequestId = vi.fn().mockResolvedValue(500n);
    const closeableWithdrawal = {
          requestId: 1n,
          withdrawalRecipient: hexToU8a("0x0000000000000000000000000000000000000000", 20),
          tokenAddress: ENABLED_TOKEN,
          amount: 1n,
          ferryTip: 0n,
          hash: hexToU8a("0x0000000000000000000000000000000000000000000000000000000000000000", 32),
        };
    const ignoredWithdrawal = {
          requestId: 2n,
          withdrawalRecipient: hexToU8a("0x0000000000000000000000000000000000000000", 20),
          tokenAddress: NOT_ENABLED_TOKEN,
          amount: 1n,
          ferryTip: 0n,
          hash: hexToU8a("0x0000000000000000000000000000000000000000000000000000000000000000", 32),
        };

    l2Mock.getWithdrawals = vi.fn().mockResolvedValue([closeableWithdrawal, ignoredWithdrawal]), 
    closer = new CloserService(l1Mock, l2Mock, TOKENS_TO_CLOSE, 1000n);

    await closer.findWithdrawalsToClose();

    expect(await closer.getNextWithdrawalToClose()).toStrictEqual(closeableWithdrawal);
    expect(await closer.getNextWithdrawalToClose()).toBeNull();


  });

  it('only considers enabled tokens above the threshold', async () => {
    const TOKENS_TO_CLOSE: [Uint8Array, bigint, bigint][] = [
      [ENABLED_TOKEN, 100n, 1n],
    ];
    l1Mock.isClosed = vi.fn().mockResolvedValue(false);
    l1Mock.isFerried = vi.fn().mockResolvedValue(false);

    l1Mock.getLatestRequestId = vi.fn().mockResolvedValue(500n);
    const closeableWithdrawal = {
          requestId: 1n,
          withdrawalRecipient: hexToU8a("0x0000000000000000000000000000000000000000", 20),
          tokenAddress: ENABLED_TOKEN,
          amount: 100n,
          ferryTip: 100n,
          hash: hexToU8a("0x0000000000000000000000000000000000000000000000000000000000000000", 32),
        };
    const ignoredWithdrawal = {
          requestId: 2n,
          withdrawalRecipient: hexToU8a("0x0000000000000000000000000000000000000000", 20),
          tokenAddress: ENABLED_TOKEN,
          amount: 99n,
          ferryTip: 99n,
          hash: hexToU8a("0x0000000000000000000000000000000000000000000000000000000000000000", 32),
        };

    l2Mock.getWithdrawals = vi.fn().mockResolvedValue([closeableWithdrawal, ignoredWithdrawal]), 
    closer = new CloserService(l1Mock, l2Mock, TOKENS_TO_CLOSE, 1000n);

    await closer.findWithdrawalsToClose();

    expect(await closer.getNextWithdrawalToClose()).toStrictEqual(closeableWithdrawal);
    expect(await closer.getNextWithdrawalToClose()).toBeNull();
  });
})
