import { describe, beforeAll, expect, it } from "vitest";
import { isEqual} from "../src/utils.js";
import { L1Api , withdrawalToViemFormat } from "../src/l1/L1Api.js";
import { L1Interface } from "../src/l1/L1Interface.js";
import { hexToU8a, u8aToHex } from "@polkadot/util";
import { anvil } from "viem/chains";
import {
  ABI,
	MANGATA_CONTRACT_ADDRESS,
} from "../src/Config.js";
import {
	type PrivateKeyAccount,
	createWalletClient,
  encodeAbiParameters,
  keccak256,
} from "viem";
import { privateKeyToAccount } from "viem/accounts";
import {  createPublicClient, createTestClient, webSocket } from "viem";
import { Withdrawal } from "../src/Withdrawal.js";

function getRandomInt (min: number, max: number) : number {
    return Math.floor(Math.random() * (max - min + 1)) + min;
}

function getRandomUintArray(length: number) {
    const arr = new Uint8Array(length);
    for (let i = 0; i < length; i++) {
        arr[i] = getRandomInt(0, 255);
    }
    return arr;
}

const WS_URI = "ws://localhost:8545";
const HTTP_URI = "http://localhost:8545";
const TOKEN_ADDRESS = hexToU8a("0xc351628EB244ec633d5f21fBD6621e1a683B1181", 160);
const ALITH = "0xf24ff3a9cf04c71dbc94d0b566f7a27b94566cac";
const ANVIL_TEST_ACCOUNT = "0x8b3a350cf5c34c9194ca85829a2df0ec3153be0318b5e2d3348e872092edffba";


function hashWithdrawal(withdrawal: Withdrawal) {
  const encoded = encodeAbiParameters(
    ABI.find((e: any) => e!.name === "ferry_withdrawal")!.inputs!,
    [withdrawalToViemFormat(withdrawal)]
  );
  const input = new Uint8Array([...hexToU8a("0x0000000000000000000000000000000000000000000000000000000000000000"), ...hexToU8a(encoded.toString())])
  return hexToU8a(keccak256(input));
}

async function updateUpdaterAccount(uri: string) {
	const transport = webSocket(uri, { retryCount: 5 });
	const publicClient = createPublicClient({
		transport,
	});

	const acc: PrivateKeyAccount = privateKeyToAccount("0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80");

	const { request } = await publicClient.simulateContract({
		account: acc,
		address: MANGATA_CONTRACT_ADDRESS,
		abi: ABI,
		functionName: "setUpdater",
		args: [acc.address]
	});

	const wc = createWalletClient({
		account: acc,
		chain: anvil,
		transport,
	});
	const txHash = await wc.writeContract(request);
  return await publicClient.waitForTransactionReceipt({ hash: txHash });
}

async function injectMerkleRoot(uri: string, merkleRoot: Uint8Array, startRange: bigint, endRange: bigint) {
	const transport = webSocket(uri, { retryCount: 5 });
	const publicClient = createPublicClient({
		transport,
	});

	const acc: PrivateKeyAccount = privateKeyToAccount("0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80");

	const { request } = await publicClient.simulateContract({
		account: acc,
		address: MANGATA_CONTRACT_ADDRESS,
		abi: ABI,
		functionName: "updateL1FromL2",
		args: [u8aToHex(merkleRoot), [startRange, endRange]]
	});

	const wc = createWalletClient({
		account: acc,
		chain: anvil,
		transport,
	});
	const txHash = await wc.writeContract(request);
  const result = await publicClient.waitForTransactionReceipt({ hash: txHash });
  return result.status === "success";
}

async function transfer(uri: string, to: Uint8Array, value: bigint) {
	const transport = webSocket(uri, { retryCount: 5 });
	const publicClient = createPublicClient({
		transport,
	});

	const acc: PrivateKeyAccount = privateKeyToAccount("0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80");

	const { request } = await publicClient.simulateContract({
		account: acc,
		address: u8aToHex(TOKEN_ADDRESS),
		abi: [{
          "constant": true,
          "inputs": [{ "name": "to", "type": "address" }, { "name": "value", "type": "uint256" }],
          "name": "mint",
          "outputs": [],
          "type": "function",
        }],
		functionName: "mint",
		args: [u8aToHex(to), 10000],
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
    await mintBlocks(1);
    l1Api = new L1Api(WS_URI);
  });

  it('should successfully connect through websocket', async () => {
    l1Api = new L1Api(WS_URI);
    expect(await l1Api.isRolldownDeployed(0n)).toBeTruthy()
  });

  it('should successfully connect through http', async () => {
    l1Api = new L1Api(HTTP_URI);
    expect(await l1Api.isRolldownDeployed(0n)).toBeTruthy()
  });

  it('can fetch latestRequestId', async () => {
    await updateUpdaterAccount(WS_URI);
    const rangeStart = 1n;
    const rangeEnd = 10n;
    // try catch in case test is run multiple times
    try {
      await injectMerkleRoot(WS_URI, hexToU8a("0x8888888888888888888888888888888888888888888888888888888888888888"),rangeStart, rangeEnd);
    } catch (e) {
      console.info("update already injected");
    }
    let latestRequestId = await l1Api.getLatestRequestId(0n);
    expect(latestRequestId).toBeGreaterThanOrEqual(rangeEnd);
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

  it('ferryWithdrawal works', async () => {
    const randomAddress = getRandomUintArray(20);

    const withdrawal: Withdrawal = {
        requestId: 1n,
        withdrawalRecipient: randomAddress,
        tokenAddress: TOKEN_ADDRESS,
        amount: 1n,
        ferryTip: 0n,
        hash: hexToU8a("0x0000000000000000000000000000000000000000000000000000000000000000", 32),
    };

    const privateKey = hexToU8a("0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80");
    expect(await l1Api.isFerried(hashWithdrawal(withdrawal))).toBeFalsy();
    await l1Api.ferry(withdrawal, privateKey);
    expect(await l1Api.isFerried(hashWithdrawal(withdrawal))).toBeTruthy();
    }, {timeout: 10000});

  it('non native ferryWithdrawal works doest not transfer native token to contract', async () => {
	const publicClient = createPublicClient({
		transport: webSocket(WS_URI, { retryCount: 5 }),
	});
    const randomAddress = getRandomUintArray(20);

    const contactBalanceBefore = await publicClient.getBalance({ address: MANGATA_CONTRACT_ADDRESS });
    const withdrawal: Withdrawal = {
        requestId: 1n,
        withdrawalRecipient: randomAddress,
        tokenAddress: TOKEN_ADDRESS,
        amount: 1n,
        ferryTip: 0n,
        hash: hexToU8a("0x0000000000000000000000000000000000000000000000000000000000000000", 32),
    };

    const privateKey = hexToU8a("0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80");
    expect(await l1Api.isFerried(hashWithdrawal(withdrawal))).toBeFalsy();
    await l1Api.ferry(withdrawal, privateKey);
    const contactBalanceAfter = await publicClient.getBalance({ address: MANGATA_CONTRACT_ADDRESS });
    expect(await l1Api.isFerried(hashWithdrawal(withdrawal))).toBeTruthy();
    expect(contactBalanceBefore).toEqual(contactBalanceAfter);
    }, {timeout: 10000});


  it('native ferryWithdrawal works', async () => {
    const randomAddress = getRandomUintArray(20);

    const nativeTokenAddress = await l1Api.getNativeTokenAddress();

    const withdrawal: Withdrawal = {
        requestId: 1n,
        withdrawalRecipient: randomAddress,
        tokenAddress: nativeTokenAddress,
        amount: 1n,
        ferryTip: 0n,
        hash: hexToU8a("0x0000000000000000000000000000000000000000000000000000000000000000", 32),
    };

    const privateKey = hexToU8a("0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80");
    expect(await l1Api.isFerried(hashWithdrawal(withdrawal))).toBeFalsy();
    await l1Api.ferry(withdrawal, privateKey);
    expect(await l1Api.isFerried(hashWithdrawal(withdrawal))).toBeTruthy();
    }, {timeout: 10000});

  it('getMerkleRange works', async () => {
    await updateUpdaterAccount(WS_URI);
    await transfer(WS_URI, hexToU8a(MANGATA_CONTRACT_ADDRESS), 10000n);
    const randomAddress = getRandomUintArray(20);
    const lastRequestId = await l1Api.getLatestRequestId(0n);

    const withdrawal = {
        requestId: lastRequestId! + 1n,
        withdrawalRecipient: randomAddress,
        tokenAddress: TOKEN_ADDRESS,
        amount: 1n,
        ferryTip: 0n,
        hash: hexToU8a("0x0000000000000000000000000000000000000000000000000000000000000000", 256),
    };
    withdrawal.hash = hashWithdrawal(withdrawal);

    await injectMerkleRoot(WS_URI, withdrawal.hash, withdrawal.requestId, withdrawal.requestId);
    const result = await l1Api.getMerkleRange(withdrawal.requestId);
    expect(result).not.toBeNull();
    }, {timeout: 30000});

  it('closeWithdrawal works', async () => {
    await updateUpdaterAccount(WS_URI);
    await transfer(WS_URI, hexToU8a(MANGATA_CONTRACT_ADDRESS), 10000n);
    const randomAddress = getRandomUintArray(20);
    const lastRequestId = await l1Api.getLatestRequestId(0n);

    let withdrawal = {
        requestId: lastRequestId! + 1n,
        withdrawalRecipient: randomAddress,
        tokenAddress: TOKEN_ADDRESS,
        amount: 1n,
        ferryTip: 0n,
        hash: hexToU8a("0x0000000000000000000000000000000000000000000000000000000000000000", 256),
    };
    withdrawal.hash = hashWithdrawal(withdrawal);

    await injectMerkleRoot(WS_URI, withdrawal.hash, withdrawal.requestId, withdrawal.requestId);
    const privateKey = hexToU8a("0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80");
    expect(await l1Api.isFerried(hashWithdrawal(withdrawal))).toBeFalsy();
    expect(await l1Api.isClosed(hashWithdrawal(withdrawal))).toBeFalsy();
    const proof: Uint8Array[] = [];
    await l1Api.closeWithdrawal(withdrawal, withdrawal.hash, proof, privateKey);
    }, {timeout: 30000});

  it('getFerry works', async () => {
    await updateUpdaterAccount(WS_URI);
    await transfer(WS_URI, hexToU8a(MANGATA_CONTRACT_ADDRESS), 10000n);
    const randomAddress = getRandomUintArray(20);
    const lastRequestId = await l1Api.getLatestRequestId(0n);

    const withdrawal1 = {
        requestId: lastRequestId! + 1n,
        withdrawalRecipient: randomAddress,
        tokenAddress: TOKEN_ADDRESS,
        amount: 1n,
        ferryTip: 0n,
        hash: hexToU8a("0x0000000000000000000000000000000000000000000000000000000000000000", 256),
    };
    withdrawal1.hash = hashWithdrawal(withdrawal1);

    const privateKey = hexToU8a("0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80");
    const acc: PrivateKeyAccount = privateKeyToAccount(u8aToHex(privateKey));
    await l1Api.ferry(withdrawal1, privateKey)
    await l1Api.isFerried(withdrawal1.hash);
    const ferryAddress = await l1Api.getFerry(withdrawal1.hash);
    expect(ferryAddress).not.toBeNull();
    expect(isEqual(ferryAddress!, hexToU8a(acc.address))).toBeTruthy();

    }, {timeout: 10000});

});
