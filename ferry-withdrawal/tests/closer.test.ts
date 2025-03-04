import { describe, beforeAll, expect, it, vi, beforeEach } from "vitest";
import { L2Interface } from "../src/l2/L2Interface.js";
import { hexToU8a, u8aToHex } from "@polkadot/util";
import { L1Interface } from "../src/l1/L1Interface.js";
import { Ferry } from "../src/Ferry.js";
import 'dotenv/config'
import { CloserService } from "../src/CloserService.js";
import { StashInterface } from "../src/stash/StashInterface.js";

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
let stashMock: StashInterface;

describe('Closer Service', () => {

  beforeEach(async () => {

    l1Mock = {
      getMerkleRange: vi.fn().mockImplementation(() => { throw new Error("Unexpected mock called") }),
      isRolldownDeployed: vi.fn().mockImplementation(() => { throw new Error("Unexpected mock called") }),
      getNativeTokenAddress: vi.fn().mockImplementation(() => NATIVE_TOKEN),
      getLatestRequestId: vi.fn().mockImplementation(() => { throw new Error("Unexpected mock called") }),
      isClosed: vi.fn().mockImplementation(() => { throw new Error("Unexpected mock called") }),
      isFerried: vi.fn().mockImplementation(() => { throw new Error("Unexpected mock called") }),
      getFerry: vi.fn().mockImplementation(() => { throw new Error("Unexpected mock called") }),
      ferry: vi.fn().mockImplementation(() => { throw new Error("Unexpected mock called") }),
      closeWithdrawal: vi.fn().mockImplementation(() => { throw new Error("Unexpected mock called") }),
      closeCancel: vi.fn().mockImplementation(() => { throw new Error("Unexpected mock called") }),
      getBalance: vi.fn().mockImplementation(() => DEFAULT_BALANCE),
    };

    l2Mock = {
      getMerkleProof: vi.fn().mockImplementation(() => { throw new Error("Unexpcted mock called") }),
      getLatestRequestId: vi.fn().mockImplementation(() => { throw new Error("Unexpcted mock called") }),
      getLatestRequestIdInPast: vi.fn().mockImplementation(() => { throw new Error("Unexpcted mock called") }),
      getWithdrawals: vi.fn().mockImplementation(() => { throw new Error("Unexpcted mock called") }),
      getRequests: vi.fn().mockImplementation(() => { throw new Error("Unexpcted mock called") }),
      getNativeTokenAddress: vi.fn().mockImplementation(() => { throw new Error("Unexpected mock called") }),
    };

    stashMock = {
      shouldBeClosed: vi.fn().mockResolvedValue(false),
    };


    closer = new CloserService(l1Mock, l2Mock, stashMock, TOKENS_TO_CLOSE, 0n);
  });

  it('should fetch all withdrawalas at once from small range', async () => {

    l1Mock.getLatestRequestId = vi.fn().mockResolvedValue(123n);
    l2Mock.getRequests = vi.fn().mockImplementationOnce(async (arg1, arg2) => { return []; })
    closer.findRequestToClose();


  });

  it('should fetch withdrawals in batches', async () => {

    l1Mock.getLatestRequestId = vi.fn().mockResolvedValue(123n);
    l2Mock.getRequests = vi.fn().mockResolvedValue([])
    l2Mock.getRequests = vi.fn().mockImplementation(async (arg1, arg2) => { return []; })

    const batchSize = 1000n;
    closer = new CloserService(l1Mock, l2Mock, stashMock, TOKENS_TO_CLOSE, batchSize);

    await closer.findRequestToClose();

    expect(l2Mock.getRequests).toHaveBeenCalledTimes(1);
    expect(l2Mock.getRequests).toHaveBeenNthCalledWith(1, 1n, 123n);

  });

  it('should fetch withdrawals in batches 2', async () => {

    l1Mock.getLatestRequestId = vi.fn().mockResolvedValue(2000n);
    l2Mock.getRequests = vi.fn().mockResolvedValue([])
    l2Mock.getRequests = vi.fn().mockImplementation(async (arg1, arg2) => { return []; })

    const batchSize = 1000n;
    closer = new CloserService(l1Mock, l2Mock, stashMock, TOKENS_TO_CLOSE, batchSize);

    await closer.findRequestToClose();

    expect(l2Mock.getRequests).toHaveBeenCalledTimes(2);
    expect(l2Mock.getRequests).toHaveBeenCalledWith(1n, 1001n);
    expect(l2Mock.getRequests).toHaveBeenCalledWith(1001n, 2000n);

  });

  it('should fetch withdrawals in batches 3', async () => {

    l2Mock.getRequests = vi.fn().mockResolvedValue([])
    l1Mock.getLatestRequestId = vi.fn().mockResolvedValue(2002);
    l2Mock.getRequests = vi.fn().mockImplementation(async (arg1, arg2) => { return []; })

    const batchSize = 1000n;
    closer = new CloserService(l1Mock, l2Mock, stashMock, TOKENS_TO_CLOSE, batchSize);
    await closer.findRequestToClose();

    expect(l2Mock.getRequests).toHaveBeenCalledTimes(3);
    expect(l2Mock.getRequests).toHaveBeenCalledWith(1n, 1001n);
    expect(l2Mock.getRequests).toHaveBeenCalledWith(1001n, 2001n);
    expect(l2Mock.getRequests).toHaveBeenCalledWith(2001n, 2002n);
  });

  it('fetches withdrawals properly when they come one by one', async () => {

    l2Mock.getRequests = vi.fn().mockResolvedValue([])
    l1Mock.getLatestRequestId = vi.fn()
      .mockResolvedValueOnce(null)
      .mockResolvedValueOnce(1n)
      .mockResolvedValueOnce(1n);

    l2Mock.getRequests = vi.fn().mockImplementation(async (arg1, arg2) => { return []; })
    closer = new CloserService(l1Mock, l2Mock, stashMock, TOKENS_TO_CLOSE, 1000n);
    await closer.findRequestToClose();
    expect(l2Mock.getRequests).toHaveBeenCalledTimes(0);

    await closer.findRequestToClose();
    expect(l2Mock.getRequests).toHaveBeenCalledTimes(1);
    expect(l2Mock.getRequests).toHaveBeenCalledWith(1n, 1n);
  });


  it('should not fetch new batch until there are some withdrawals to consume', async () => {

    l1Mock.getLatestRequestId = vi.fn().mockResolvedValue(2001n);
    l2Mock.getRequests = vi.fn().mockResolvedValue([{
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
    closer = new CloserService(l1Mock, l2Mock, stashMock, TOKENS_TO_CLOSE, batchSize);
    await closer.findRequestToClose();
    await closer.findRequestToClose();
    await closer.findRequestToClose();
    await closer.findRequestToClose();
    await closer.findRequestToClose();

    expect(l2Mock.getRequests).toHaveBeenCalledTimes(1);
    expect(l2Mock.getRequests).toHaveBeenCalledWith(1n, 1001n);
  });

  it('should fetch new batch once all the txs from the previous one are consumed', async () => {
    l1Mock.getLatestRequestId = vi.fn().mockResolvedValue(2001n);
    l2Mock.getRequests = vi.fn().mockResolvedValue([{
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
    closer = new CloserService(l1Mock, l2Mock, stashMock, TOKENS_TO_CLOSE, batchSize);
    await closer.findRequestToClose();
    await closer.findRequestToClose();
    await closer.findRequestToClose();
    await closer.findRequestToClose();
    await closer.findRequestToClose();


    expect(await closer.getNextRequestToClose()).not.toBeNull();
    expect(await closer.getNextRequestToClose()).toBeNull();

    expect(l2Mock.getRequests).toHaveBeenCalledTimes(1);
    expect(l2Mock.getRequests).toHaveBeenCalledWith(1n, 1001n);

    await closer.findRequestToClose();
    expect(l2Mock.getRequests).toHaveBeenCalledTimes(2);
    expect(l2Mock.getRequests).toHaveBeenCalledWith(1001n, 2001n);
  });

  it('do not fetch previous withdrawals once everything is processed', async () => {
    l1Mock.getLatestRequestId = vi.fn().mockResolvedValue(500n);
    l2Mock.getRequests = vi.fn().mockResolvedValue([])
    l1Mock.isClosed = vi.fn().mockResolvedValue(false);
    l1Mock.isFerried = vi.fn().mockResolvedValue(false);

    const batchSize = 1000n;
    closer = new CloserService(l1Mock, l2Mock, stashMock, TOKENS_TO_CLOSE, batchSize);
    await closer.findRequestToClose();
    expect(l2Mock.getRequests).toHaveBeenCalledTimes(1);
    expect(l2Mock.getRequests).toHaveBeenCalledWith(1n, 500n);

    await closer.findRequestToClose();
    await closer.findRequestToClose();
    await closer.findRequestToClose();
    await closer.findRequestToClose();
    expect(l2Mock.getRequests).toHaveBeenCalledTimes(1);

    l1Mock.getLatestRequestId = vi.fn().mockResolvedValue(1200n);
    await closer.findRequestToClose();
    expect(l2Mock.getRequests).toHaveBeenCalledTimes(2);
    expect(l2Mock.getRequests).toHaveBeenCalledWith(500n, 1200n);
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

    l2Mock.getRequests = vi.fn().mockResolvedValue([closeableWithdrawal, ignoredWithdrawal]), 
    closer = new CloserService(l1Mock, l2Mock, stashMock, TOKENS_TO_CLOSE, 1000n);

    await closer.findRequestToClose();

    expect(await closer.getNextRequestToClose()).toStrictEqual(closeableWithdrawal);
    expect(await closer.getNextRequestToClose()).toBeNull();


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

    l2Mock.getRequests = vi.fn().mockResolvedValue([closeableWithdrawal, ignoredWithdrawal]), 
    closer = new CloserService(l1Mock, l2Mock, stashMock, TOKENS_TO_CLOSE, 1000n);

    await closer.findRequestToClose();

    expect(await closer.getNextRequestToClose()).toStrictEqual(closeableWithdrawal);
    expect(await closer.getNextRequestToClose()).toBeNull();
  });

  it('considers txs reported by stash (initiated by frontend)', async () => {
    const TOKENS_TO_CLOSE: [Uint8Array, bigint, bigint][] = [
    ];
    l1Mock.isClosed = vi.fn().mockResolvedValue(false);
    l1Mock.isFerried = vi.fn().mockResolvedValue(false);
    l1Mock.getLatestRequestId = vi.fn().mockResolvedValue(500n);
    const closableWtihdrawalHash = hexToU8a("0x1111111111111111111111111111111111111111111111111111111111111111", 32);
    const ignoredWtihdrawalHash = hexToU8a("0x0000000000000000000000000000000000000000000000000000000000000000", 32);

    stashMock.shouldBeClosed = vi.fn().mockImplementation((hash) => 
      u8aToHex(closableWtihdrawalHash) === u8aToHex(hash) ? true : false
    )

    const closeableWithdrawal = {
          requestId: 1n,
          withdrawalRecipient: hexToU8a("0x0000000000000000000000000000000000000000", 20),
          tokenAddress: ENABLED_TOKEN,
          amount: 100n,
          ferryTip: 100n,
          hash: closableWtihdrawalHash,
        };
    const ignoredWithdrawal = {
          requestId: 2n,
          withdrawalRecipient: hexToU8a("0x0000000000000000000000000000000000000000", 20),
          tokenAddress: ENABLED_TOKEN,
          amount: 99n,
          ferryTip: 99n,
          hash: ignoredWtihdrawalHash,
        };

    l2Mock.getRequests = vi.fn().mockResolvedValue([closeableWithdrawal, ignoredWithdrawal]), 
    closer = new CloserService(l1Mock, l2Mock, stashMock, TOKENS_TO_CLOSE, 1000n);

    await closer.findRequestToClose();

    expect(await closer.getNextRequestToClose()).toStrictEqual(closeableWithdrawal);
    expect(await closer.getNextRequestToClose()).toBeNull();
  });

  it('ignores txs in replica mode', async () => {
    const TOKENS_TO_CLOSE: [Uint8Array, bigint, bigint][] = [
      [ENABLED_TOKEN, 100n, 1n],
    ];
    l1Mock.isClosed = vi.fn().mockResolvedValue(false);
    l1Mock.isFerried = vi.fn().mockResolvedValue(false);

    l1Mock.getLatestRequestId = vi.fn().mockResolvedValue(500n);
    const closeableWithdrawal1 = {
          requestId: 1n,
          withdrawalRecipient: hexToU8a("0x0000000000000000000000000000000000000000", 20),
          tokenAddress: ENABLED_TOKEN,
          amount: 100n,
          ferryTip: 100n,
          hash: hexToU8a("0x0000000000000000000000000000000000000000000000000000000000000000", 32),
        };
    const ignoredWithdrawal1 = {
          requestId: 2n,
          withdrawalRecipient: hexToU8a("0x0000000000000000000000000000000000000000", 20),
          tokenAddress: ENABLED_TOKEN,
          amount: 99n,
          ferryTip: 99n,
          hash: hexToU8a("0x0000000000000000000000000000000000000000000000000000000000000000", 32),
        };
    const ignoredWithdrawal2 = {
          requestId: 3n,
          withdrawalRecipient: hexToU8a("0x0000000000000000000000000000000000000000", 20),
          tokenAddress: ENABLED_TOKEN,
          amount: 99n,
          ferryTip: 99n,
          hash: hexToU8a("0x0000000000000000000000000000000000000000000000000000000000000000", 32),
        };
    const ignoredWithdrawal3 = {
          requestId: 4n,
          withdrawalRecipient: hexToU8a("0x0000000000000000000000000000000000000000", 20),
          tokenAddress: ENABLED_TOKEN,
          amount: 99n,
          ferryTip: 99n,
          hash: hexToU8a("0x0000000000000000000000000000000000000000000000000000000000000000", 32),
        };
    const closeableWithdrawal2 = {
          requestId: 5n,
          withdrawalRecipient: hexToU8a("0x0000000000000000000000000000000000000000", 20),
          tokenAddress: ENABLED_TOKEN,
          amount: 100n,
          ferryTip: 100n,
          hash: hexToU8a("0x0000000000000000000000000000000000000000000000000000000000000000", 32),
        };

    l2Mock.getRequests = vi.fn().mockResolvedValue(
      [closeableWithdrawal1, ignoredWithdrawal1, ignoredWithdrawal2, ignoredWithdrawal3, closeableWithdrawal2 ]
    ), 
    closer = new CloserService(l1Mock, l2Mock, stashMock, TOKENS_TO_CLOSE, 1000n, 1000n, 4n, 1n);

    await closer.findRequestToClose();

    expect(await closer.getNextRequestToClose()).toStrictEqual(closeableWithdrawal1);
    expect(await closer.getNextRequestToClose()).toStrictEqual(closeableWithdrawal2);
    expect(await closer.getNextRequestToClose()).toBeNull();
  });

  it('ignores txs below threshold', async () => {
    const TOKENS_TO_CLOSE: [Uint8Array, bigint, bigint][] = [
      [ENABLED_TOKEN, 100n, 1n],
    ];
    l1Mock.isClosed = vi.fn().mockResolvedValue(false);
    l1Mock.isFerried = vi.fn().mockResolvedValue(false);

    l1Mock.getLatestRequestId = vi.fn().mockResolvedValue(500n);
    const closeableWithdrawal1 = {
          requestId: 150n,
          withdrawalRecipient: hexToU8a("0x0000000000000000000000000000000000000000", 20),
          tokenAddress: ENABLED_TOKEN,
          amount: 100n,
          ferryTip: 100n,
          hash: hexToU8a("0x0000000000000000000000000000000000000000000000000000000000000000", 32),
        };
    const closeableWithdrawal2 = {
          requestId: 151,
          withdrawalRecipient: hexToU8a("0x0000000000000000000000000000000000000000", 20),
          tokenAddress: ENABLED_TOKEN,
          amount: 100n,
          ferryTip: 100n,
          hash: hexToU8a("0x0000000000000000000000000000000000000000000000000000000000000000", 32),
        };
    const ignoredWithdrawal1 = {
          requestId: 152n,
          withdrawalRecipient: hexToU8a("0x0000000000000000000000000000000000000000", 20),
          tokenAddress: ENABLED_TOKEN,
          amount: 99n,
          ferryTip: 99n,
          hash: hexToU8a("0x0000000000000000000000000000000000000000000000000000000000000000", 32),
        };
    const ignoredWithdrawal2 = {
          requestId: 153n,
          withdrawalRecipient: hexToU8a("0x0000000000000000000000000000000000000000", 20),
          tokenAddress: ENABLED_TOKEN,
          amount: 99n,
          ferryTip: 99n,
          hash: hexToU8a("0x0000000000000000000000000000000000000000000000000000000000000000", 32),
        };
    const ignoredWithdrawal3 = {
          requestId: 154n,
          withdrawalRecipient: hexToU8a("0x0000000000000000000000000000000000000000", 20),
          tokenAddress: ENABLED_TOKEN,
          amount: 99n,
          ferryTip: 99n,
          hash: hexToU8a("0x0000000000000000000000000000000000000000000000000000000000000000", 32),
        };

    l2Mock.getRequests = vi.fn().mockResolvedValue(
      [closeableWithdrawal1, ignoredWithdrawal1, ignoredWithdrawal2, ignoredWithdrawal3, closeableWithdrawal2 ]
    ), 
    closer = new CloserService(l1Mock, l2Mock, stashMock, TOKENS_TO_CLOSE, 1000n, 1000n, 0n, 0n, 150n);

    await closer.findRequestToClose();

    expect(await closer.getNextRequestToClose()).toStrictEqual(closeableWithdrawal1);
    expect(await closer.getNextRequestToClose()).toStrictEqual(closeableWithdrawal2);
    expect(await closer.getNextRequestToClose()).toBeNull();
  });
})
