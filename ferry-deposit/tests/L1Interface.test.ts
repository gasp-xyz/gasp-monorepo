import { describe, beforeAll, expect, it } from "vitest";
import { L1Api } from "../src/l1/L1Api.js";
import { L1Interface } from "../src/l1/L1Interface.js";
import { anvil } from "viem/chains";
import {
  ABI,
	MANGATA_CONTRACT_ADDRESS,
} from "../src/config.js";


import {
	type PrivateKeyAccount,
	createWalletClient,
} from "viem";
import { privateKeyToAccount } from "viem/accounts";
import { createPublicClient, createTestClient, webSocket } from "viem";

const WS_URI = "ws://localhost:8545";

async function dummyDeposit(uri: string) {
	const ANVIL_TEST_ACCOUNT =
		"0x8b3a350cf5c34c9194ca85829a2df0ec3153be0318b5e2d3348e872092edffba";
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
    expect(await l1Api.isRolldownDeployed()).toBeTruthy()
  });

  it('should successfully connect through http', async () => {
    expect(await l1Api.isRolldownDeployed()).toBeTruthy()
  });

  it('can fetch latestRequestId', async () => {
    let latestRequestId = await l1Api.getLatestRequestId();
    expect(latestRequestId).toBeGreaterThanOrEqual(0);
    await dummyDeposit(WS_URI);
    expect(await l1Api.getLatestRequestId()).toBeGreaterThan(latestRequestId!);
  });

  it('can fetch deposits', async () => {
    await dummyDeposit(WS_URI);
    let latestRequestId = await l1Api.getLatestRequestId();
    const deposits = await l1Api.getDeposits(1n, latestRequestId!);
    expect(deposits.length).toBeGreaterThan(0);
  });

  it('can fetch deposits hash', async () => {

    const firstId = (await l1Api.getLatestRequestId())!;
    await dummyDeposit(WS_URI);

    const secondId = (await l1Api.getLatestRequestId())!;
    expect(firstId).not.toEqual(secondId);

    const firstHash = await l1Api.getDepostiHash(firstId);
    const secondHash = await l1Api.getDepostiHash(secondId);
    expect(firstHash).not.toEqual(secondHash);

  });

  it('block delay works for getLatestRequestId', async () => {
    l1Api = new L1Api(WS_URI, 10n);
    await mintBlocks(10);
    const firstId = (await l1Api.getLatestRequestId())!;

    await dummyDeposit(WS_URI);

    const secondId = (await l1Api.getLatestRequestId())!;
    expect(firstId).to.be.equal(secondId);

    await mintBlocks(10);
    const thirdId = (await l1Api.getLatestRequestId())!;
    expect(thirdId).to.be.equal(firstId + 1n);
  });

  it('block delay works for getDeposits', async () => {
    l1Api = new L1Api(WS_URI, 10n);
    await mintBlocks(10);

    const from = (await l1Api.getLatestRequestId())!;
    await dummyDeposit(WS_URI);
    expect(l1Api.getDeposits(from + 1n , from + 1n)).rejects.toThrow();

    await mintBlocks(10);
    const after = (await l1Api.getLatestRequestId())!;
    expect(after).to.be.equal(from + 1n);
    expect((await l1Api.getDeposits(after, after)).length).to.be.equal(1);
  });



});
