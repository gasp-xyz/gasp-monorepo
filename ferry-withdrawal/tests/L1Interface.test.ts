import { describe, test, beforeAll, expect, it } from "vitest";
import { sleep, getApi } from "../src/utils/index.js";
import { L2Interface, L2Api } from "../src/l2";
import { L1Api } from "../src/l1";
import { L1Interface } from "../src/l1/index.js";
import { hexToU8a } from "@polkadot/util";
import { anvil } from "viem/chains";
import {
  ABI,
	MANGATA_CONTRACT_ADDRESS,
} from "../src/common/constants.js";


import {
	http,
	type PrivateKeyAccount,
	createWalletClient,
} from "viem";
import { privateKeyToAccount } from "viem/accounts";
import { TestClient, createPublicClient, createTestClient, webSocket } from "viem";
import util from "node:util";

const timeout = 60000;

const WS_URI = "ws://localhost:8545";
const HTTP_URI = "http://localhost:8545";
const TOKEN_ADDRESS = hexToU8a("0xFD471836031dc5108809D173A067e8486B9047A3", 160);
const ALITH = "0xf24ff3a9cf04c71dbc94d0b566f7a27b94566cac";
const ANVIL_TEST_ACCOUNT = "0x8b3a350cf5c34c9194ca85829a2df0ec3153be0318b5e2d3348e872092edffba";

const properImpl = (key: any, value: any) => {
  if (typeof value === 'bigint') {
    return value.toString();
  }
  return value;
};

async function getBlockNumnber() {
	const transport = webSocket(WS_URI, { retryCount: 5 });
	const publicClient = createPublicClient({
		transport,
	});
  return await publicClient.getBlockNumber({cacheTime: 0});
}

async function dummyDeposit(uri: string) {
	const transport = webSocket(uri, { retryCount: 5 });
	const publicClient = createPublicClient({
		transport,
	});

	const acc: PrivateKeyAccount = privateKeyToAccount(ANVIL_TEST_ACCOUNT);

	const { request } = await publicClient.simulateContract({
		account: acc,
		address: MANGATA_CONTRACT_ADDRESS,
		abi: ABI,
		functionName: "deposit_native",
		value: BigInt(123456789),
	});

	const wc = createWalletClient({
		account: acc,
		chain: anvil,
		transport,
	});
	const txHash = await wc.writeContract(request);
  return await publicClient.waitForTransactionReceipt({ hash: txHash });
}

async function mintBlocks(count: number) {
  const tc = createTestClient({
    mode: "anvil",
    transport: webSocket(WS_URI, { retryCount: 5 })
  });
  await tc.mine({ blocks: count })
}

let l1Api : L1Interface;

describe('L1Interface', () => {
  beforeAll(async () => {
    await dummyDeposit(WS_URI);
    await mintBlocks(1);
    l1Api = new L1Api(WS_URI, 0n);
  });

  it('should successfully connect through websocket', async () => {
    l1Api = new L1Api(WS_URI, 0n);
    expect(await l1Api.isRolldownDeployed()).toBeTruthy()
  });

  it('should successfully connect through http', async () => {
    l1Api = new L1Api(HTTP_URI, 0n);
    expect(await l1Api.isRolldownDeployed()).toBeTruthy()
  });

  it('can fetch latestRequestId', async () => {
    let latestRequestId = await l1Api.getLatestRequestId();
    expect(latestRequestId).toBeGreaterThanOrEqual(0);
    await dummyDeposit(WS_URI);
    expect(await l1Api.getLatestRequestId()).toBeGreaterThan(latestRequestId!);
  });

  it('can fetch erc20 balance of existing token', async () => {
    const value = await l1Api.getBalance(TOKEN_ADDRESS, hexToU8a(ALITH));
    expect(value).toBeGreaterThanOrEqual(0);
  });

  it('can fetch erc20 balance of unexisting token (returns null)', async () => {
    const value = await l1Api.getBalance(hexToU8a("0x8888888888888888888888888888888888888888"), hexToU8a(ALITH));
    expect(value).toBeNull();
  });

  it('can fetch balance of native token (returns null)', async () => {
    const value = await l1Api.getBalance(hexToU8a("0x8888888888888888888888888888888888888888"), hexToU8a(ALITH));
    expect(value).toBeNull();
  });

  it('can fetch balance of native token (returns null)', async () => {
    const value = await l1Api.getBalance(hexToU8a("0x8888888888888888888888888888888888888888"), hexToU8a(ALITH));
    expect(value).toBeNull();
  });

  it('can fetch native token address', async () => {
    const value = await l1Api.getNativeTokenAddress();
    expect(value).toEqual(hexToU8a("0x0000000000000000000000000000000000000001"));
  });

  it('can fetch native token balance', async () => {
    const nativeTokenAddress = await l1Api.getNativeTokenAddress();
    const acc: PrivateKeyAccount = privateKeyToAccount(ANVIL_TEST_ACCOUNT);
    const value = await l1Api.getBalance(nativeTokenAddress, hexToU8a(acc.address));
    expect(value).toBeGreaterThan(0);
  });

  it('isFerried works', async () => {
    const value = await l1Api.isFerried(hexToU8a("0x9191919191919191919191919191919191919191919191919191919191919191"));
    expect(value).toBeFalsy();
  });

  it('isClosed works', async () => {
    const value = await l1Api.isClosed(hexToU8a("0x9191919191919191919191919191919191919191919191919191919191919191"));
    expect(value).toBeFalsy();
  });



});
