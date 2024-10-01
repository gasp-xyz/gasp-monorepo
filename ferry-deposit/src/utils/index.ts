import util from "node:util";
import { type ApiPromise, Keyring } from "@polkadot/api";
import type { ApiDecoration } from "@polkadot/api/types";
import type { KeyringPair } from "@polkadot/keyring/types";
import type { StorageKey } from "@polkadot/types";
import type { Option, u32, u128 } from "@polkadot/types-codec";
import type {
	FrameSystemEventRecord,
	MangataTypesAssetsL1Asset,
	OrmlTokensAccountData,
	PalletRolldownMessagesChain,
	PalletRolldownMessagesDeposit,
	PalletRolldownMessagesL1Update,
	SpRuntimeAccountAccountId20,
} from "@polkadot/types/lookup";
import type { ITuple } from "@polkadot/types/types";
import { Mangata, type MangataGenericEvent, signTx } from "gasp-sdk";
import {
	http,
	type PrivateKeyAccount,
	createWalletClient,
	webSocket,
} from "viem";
import { type PublicClientConfig, createPublicClient } from "viem";
import { privateKeyToAccount } from "viem/accounts";
import { anvil } from "viem/chains";

import "gasp-types";

import type { H256 } from "@polkadot/types/interfaces/runtime";
import { hexToU8a, u8aToHex } from "@polkadot/util";
import type { KeypairType } from "@polkadot/util-crypto/types";
import { type PublicClient, encodeAbiParameters, keccak256 } from "viem";
import {
	ABI,
	L1_CHAIN,
	MANGATA_CONTRACT_ADDRESS,
	MIN_PROFIT,
} from "../common/constants.js";

function getL1ChainType(api: ApiPromise): PalletRolldownMessagesChain {
	return api.createType("PalletRolldownMessagesChain", L1_CHAIN);
}

function createL1Asset(
	api: ApiPromise,
	tokenAddress: Uint8Array,
): MangataTypesAssetsL1Asset {
	const chain: PalletRolldownMessagesChain = getL1ChainType(api);
	if (chain.isEthereum) {
		return api.createType("MangataTypesAssetsL1Asset", {
			Ethereum: tokenAddress,
		});
	} else if (chain.isArbitrum) {
		return api.createType("MangataTypesAssetsL1Asset", {
			Arbitrum: tokenAddress,
		});
	} else {
		throw new Error(`Unknown chain id ${chain.toHuman()}`);
	}
}

async function asyncFilter(arr: Deposit[], predicate: any) {
	const results = await Promise.all(arr.map(predicate));
	return arr.filter((v: any, index: any) => {
		return results[index];
	});
}

interface Deposit {
	readonly requestId: bigint;
	readonly depositRecipient: Uint8Array;
	readonly tokenAddress: Uint8Array;
	readonly amount: bigint;
	readonly timeStamp: bigint;
	readonly ferryTip: bigint;
}

class Ferry {
	l1: L1Interface;
	l2: L2Interface;
	me: Uint8Array;
	txCost: bigint;
	minProfit: bigint;

	constructor(
		me: Uint8Array,
		l1: L1Interface,
		l2: L2Interface,
		txCost: bigint,
		minProfit: bigint,
	) {
		this.me = me;
		this.l1 = l1;
		this.l2 = l2;
		this.txCost = txCost;
		this.minProfit = minProfit;
	}

	logFilteredOut(before: Deposit[], after: Deposit[], message: string) {
		const diff = before.length - after.length;
		if (diff > 0) {
			console.info(`filtered out ${diff} : ${message}`);
		}
	}

	async getPendingDeposits(): Promise<Deposit[]> {
		const end = await this.l1.getLatestRequestId();
		let start = await this.l2.getLastProcessedRequestId();
		if (end === null) {
			return Promise.resolve([]);
		}
		if (start === 0n) {
			start = 1n;
		}
		const deposits = await this.l1.getDeposits(start, end);

		const ferryableDeposits = await asyncFilter(
			deposits,
			async (elem: Deposit) => {
				const hash = await this.l1.getDepostiHash(elem.requestId);
				const isFerried = await this.l2.isFerried(hash);
				const isExecuted = await this.l2.isExecuted(elem.requestId);
				return !isFerried && !isExecuted;
			},
		);
		return ferryableDeposits;
	}

	async rateDeposits(deposits: Deposit[]): Promise<Deposit[]> {
		const nativeTokenAddress = await this.l2.getNativeTokenAddress();
		const balances = await this.l2.getBalances(this.me);

		const tokenAddressToBalance = new Map<string, bigint>(
			Array.from(balances, ([k, v]) => [u8aToHex(k), v]),
		);

		const validDeposits = deposits.filter((deposit) => {
			return deposit.amount > deposit.ferryTip;
		});
		this.logFilteredOut(deposits, validDeposits, "invalid deposit");

		const affordableDeposits = validDeposits.filter((deposit) => {
			const transferAmount = deposit.amount - deposit.ferryTip;
			const tokenAddress = u8aToHex(deposit.tokenAddress);

			if (tokenAddressToBalance.has(tokenAddress)) {
				if (deposit.tokenAddress === nativeTokenAddress) {
					return (
						tokenAddressToBalance.get(tokenAddress)! >=
						transferAmount + this.txCost
					);
				} else {
					return tokenAddressToBalance.get(tokenAddress)! >= transferAmount;
				}
			} else {
				return false;
			}
		});

		this.logFilteredOut(
			validDeposits,
			affordableDeposits,
			`not enought tokens to cover Account: ${u8aToHex(this.me)}`,
		);

		const ratedDeposits = affordableDeposits.sort((a, b) => {
			if (a.ferryTip > b.ferryTip) {
				return -1;
			} else if (a.ferryTip === b.ferryTip) {
				if (a.amount > b.amount) {
					return 1;
				} else if (a.amount > b.amount) {
					return 0;
				} else {
					return -1;
				}
			} else {
				return 1;
			}
		});

		const aboveProfitThreshold = await asyncFilter(
			ratedDeposits,
			async (elem: Deposit) => {
				return (
					(await this.l2.valutateToken(elem.tokenAddress, elem.ferryTip)) >=
					this.minProfit
				);
			},
		);

		this.logFilteredOut(
			affordableDeposits,
			aboveProfitThreshold,
			"profit below expected threshold",
		);

		return aboveProfitThreshold;
	}
}

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
	return await wc.writeContract(request);
}

interface L1Interface {
	isRolldownDeployed(): Promise<boolean>;
	getLatestRequestId(): Promise<bigint | null>;
	getDeposits(rangeStart: bigint, rangeEnd: bigint): Promise<Deposit[]>;
	getDepostiHash(requestId: bigint): Promise<Uint8Array>;
}

interface L2Interface {
	getBalances(address: Uint8Array): Promise<Map<Uint8Array, bigint>>;
	getLastProcessedRequestId(): Promise<bigint>;
	getLastProcessedRequestId(): Promise<bigint>;
	isExecuted(depositId: bigint): Promise<boolean>;
	isFerried(depositId: Uint8Array): Promise<boolean>;
	getNativeTokenAddress(): Promise<Uint8Array>;
	valutateToken(tokenAddress: Uint8Array, amount: bigint): Promise<bigint>;
}

class L1Api implements L1Interface {
	client!: PublicClient;

	constructor(uri: string) {
		if (uri.startsWith("ws")) {
			this.client = createPublicClient({
				transport: webSocket(uri, { retryCount: 5 }),
			});
		} else if (uri.startsWith("http")) {
			this.client = createPublicClient({
				transport: http(uri, { retryCount: 5 }),
			});
		} else {
			throw new Error("Invalid uri");
		}
	}

	async getDepostiHash(requestId: bigint): Promise<Uint8Array> {
		const value = await this.client.readContract({
			address: MANGATA_CONTRACT_ADDRESS,
			abi: ABI,
			functionName: "deposits",
			blockTag: "finalized",
			args: [requestId],
		});
		const hash = encodeAbiParameters(
			ABI.find((e: any) => e!.name === "deposits")!.outputs!,
			[...(value as any)],
		);
		return hexToU8a(keccak256(hexToU8a(hash)));
	}

	async getLatestRequestId(): Promise<bigint | null> {
		const value = await this.client.readContract({
			address: MANGATA_CONTRACT_ADDRESS,
			abi: ABI,
			functionName: "counter",
			blockTag: "finalized",
		});
		const reqId = BigInt(value as any) - 1n;
		if (reqId < 1n) {
			return null;
		}
		return reqId;
	}

	async getDeposits(rangeStart: bigint, rangeEnd: bigint): Promise<Deposit[]> {
		const selector = "getPendingRequests";
		const contractData = await this.client.readContract({
			address: MANGATA_CONTRACT_ADDRESS,
			abi: ABI,
			functionName: selector,
			args: [rangeStart, rangeEnd],
		});

		return (contractData as any).pendingDeposits.map((deposit: any) => {
			return {
				requestId: deposit.requestId.id,
				depositRecipient: hexToU8a(deposit.depositRecipient),
				tokenAddress: hexToU8a(deposit.tokenAddress),
				amount: deposit.amount,
				timeStamp: deposit.timeStamp,
				ferryTip: deposit.ferryTip,
			};
		});
	}

	async isRolldownDeployed(): Promise<boolean> {
		const code = await this.client.getCode({
			address: MANGATA_CONTRACT_ADDRESS,
			blockTag: "finalized",
		});
		return code !== "0x";
	}
}

class L2Api implements L2Interface {
	api!: ApiPromise;
	keyring!: KeyringPair;

	constructor(api: ApiPromise) {
		this.api = api;
	}

	async valutateToken(
		tokenAddress: Uint8Array,
		amount: bigint,
	): Promise<bigint> {
		const asset: MangataTypesAssetsL1Asset = createL1Asset(
			this.api,
			tokenAddress,
		);
		const tokenId = await this.api.query.assetRegistry.l1AssetToId(asset);
		if (tokenId.isNone) {
			return 0n;
		}

		if (tokenId.unwrap().toNumber() == 0) {
			return amount;
		} else {
			return BigInt(
				(
					await this.api.rpc.xyk.calculate_sell_price(
						tokenId.unwrap(),
						0,
						amount,
					)
				).toString(),
			);
		}
	}

	async getNativeTokenAddress(): Promise<Uint8Array> {
		return (await this.api.query.assetRegistry.idToL1Asset(0))
			.unwrap()
			.asEthereum.toU8a();
	}

	async getBalances(address: Uint8Array): Promise<Map<Uint8Array, bigint>> {
		const assetMapping =
			await this.api.query.assetRegistry.idToL1Asset.entries();
		const chain = getL1ChainType(this.api);

		const mapping: [bigint, Uint8Array][] = assetMapping
			.filter(
				([_key, value]) =>
					(value.unwrap().isEthereum && chain.isEthereum) ||
					(value.unwrap().isArbitrum && chain.isArbitrum),
			)
			.map(([key, value]) => {
				const id = BigInt(key.args[0].toString());
				const address = value.unwrap();
				if (address.isArbitrum && chain.isArbitrum) {
					return [id, value.unwrap().asArbitrum.toU8a()];
				} else if (address.isEthereum && chain.isEthereum) {
					return [id, value.unwrap().asEthereum.toU8a()];
				}
        throw new Error("Invalid chain type");
			});

		const idToL1Asset = new Map<bigint, Uint8Array>(mapping);

		const balances = await this.api.query.tokens.accounts.entries(address);
		const values: [Uint8Array, bigint][] = balances
			.filter(([key, _value]) =>
				idToL1Asset.has(BigInt(key.args[1].toString())),
			)
			.map(([key, value]) => {
				const tokenAddress = idToL1Asset.get(BigInt(key.args[1].toString()))!;
				return [tokenAddress, BigInt(value.free.toString())];
			});

		return new Map<Uint8Array, bigint>(values);
	}

	async getLastProcessedRequestId(): Promise<bigint> {
		const last =
			await this.api.query.rolldown.lastProcessedRequestOnL2(L1_CHAIN);
		return BigInt(last.toString());
	}

	async isExecuted(depositId: bigint): Promise<boolean> {
		return depositId <= (await this.getLastProcessedRequestId());
	}

	async isFerried(depositHash: Uint8Array): Promise<boolean> {
		if (depositHash.length !== 32) {
			throw new Error("depositHash must be exactly 32 bytes long.");
		}
		const chain = getL1ChainType(this.api);
		const status = await this.api.query.rolldown.ferriedDeposits([
			chain,
			depositHash,
		]);
		return status.isSome;
	}
}

function sleep(timeInMilliseconds: number): Promise<void> {
	return new Promise((resolve) => setTimeout(resolve, timeInMilliseconds));
}

function getKeyring(type: KeypairType): Keyring {
	return new Keyring({ type });
}

function getCollator(type: KeypairType, mnemonic: string): KeyringPair {
	const keyring = getKeyring(type);
	return keyring.addFromSeed(hexToU8a(mnemonic));
}

async function getApi(nodeUrl: string): Promise<ApiPromise> {
	const api = await Mangata.instance([nodeUrl]).api();
	await api.isReady;
	return api;
}

async function getNativeL1Update(
	api: ApiPromise,
	encodedData: `0x${string}`,
): Promise<Option<PalletRolldownMessagesL1Update>> {
	return await api.rpc.rolldown.get_native_sequencer_update(
		encodedData.substring(2),
	);
}

async function getEvents(
	apiAt: ApiDecoration<"promise">,
	section: string,
	method: string,
): Promise<FrameSystemEventRecord[]> {
	const events = await apiAt.query.system.events();

	return events.filter(
		(event) => event.event.section === section && event.event.method === method,
	);
}

function isSuccess(events: MangataGenericEvent[]) {
	return events.some(
		(event) =>
			event.section === "system" && event.method === "ExtrinsicSuccess",
	);
}

function print(data: any) {
	console.log(util.inspect(data, { depth: null }));
}

export {
	print,
	sleep,
	getApi,
	getEvents,
	isSuccess,
	getCollator,
	getNativeL1Update,
	L1Api,
	type L1Interface,
	L2Api,
	type L2Interface,
	dummyDeposit,
	Ferry,
	type Deposit,
	getL1ChainType,
};
