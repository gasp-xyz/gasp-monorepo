import { BTreeMap, u64, u128 } from "@polkadot/types-codec";
import type { H256 } from "@polkadot/types/interfaces/runtime";
import type { ITuple } from "@polkadot/types-codec/types";
import type { KeyringPair } from "@polkadot/keyring/types";
import type { L2Interface } from "./L2Interface.js";
import type { Option } from "@polkadot/types-codec";
import { PalletRolldownL2Request } from "@polkadot/types/lookup";
import { Mangata } from "gasp-sdk";
import { Withdrawal } from "../Withdrawal.js";
import { Cancel } from "../Cancel.js";
import { type ApiPromise } from "@polkadot/api";
import { logger } from "../logger.js";

async function getApi(nodeUrl: string): Promise<ApiPromise> {
	const api = await Mangata.instance([nodeUrl]).api();
	await api.isReady;
	return api;
}

function createBigIntArrayFromRange(start: bigint, end: bigint) {
	const result = [];
	for (let i = start; i <= end; i++) {
		result.push(i);
	}
	return result;
}

function getL1ChainType(api: ApiPromise, chainId: number): u64 {
	return api.createType("u64", chainId);
}

class L2Api implements L2Interface {
	api!: ApiPromise;
	keyring!: KeyringPair;
	chainId: number;

	constructor(api: ApiPromise, chainId: number) {
		this.api = api;
		this.chainId = chainId;
	}

	async getMerkleProof(
		startRange: bigint,
		endRange: bigint,
		txId: bigint,
	): Promise<Uint8Array[]> {
		const chain: u64 = getL1ChainType(this.api, this.chainId);
		const result = await this.api.rpc.rolldown.get_merkle_proof(
			chain,
			[startRange, endRange],
			txId,
		);
		return result.map((elem: Uint8Array) => elem);
	}

	async getNativeTokenAddress(): Promise<Uint8Array> {
		return (await this.api.query.assetRegistry.idToL1Asset(0))
			.unwrap()[1].toU8a();
	}

	parseLatestRequestId(
		nextRequesId: BTreeMap<u64, u128>,
	): bigint | null {
		// NOTE: looks like === is not implemented for PalletRolldownMessagesChain
		// therefore its not possible to query valu from map using .get(chain) query ;<
		const chain: u64 = getL1ChainType(this.api, this.chainId);
		let found = Array.from(nextRequesId.keys()).findIndex((key) => {
			return key.toString() === chain.toString();
		});

		if (found == -1) {
			return null;
		} else {
			return Array.from(nextRequesId.values())[found].toBigInt() - 1n;
		}
	}

	async getLatestRequestId(): Promise<bigint | null> {
		let nextRequesId = await this.api.query.rolldown.l2OriginRequestId();
		return this.parseLatestRequestId(nextRequesId);
	}

	async getLatestRequestIdInPast(blockInPast: number): Promise<bigint | null> {
		const { number } = await this.api.rpc.chain.getHeader();
		const targetBlock = Math.max(0, number.toNumber() - blockInPast);
		const targetBlockHash = await this.api.query.system.blockHash(targetBlock);
		let apiAt = await this.api.at(targetBlockHash);
		let nextRequesId = await apiAt.query.rolldown.l2OriginRequestId();
		return this.parseLatestRequestId(nextRequesId);
	}

	async getWithdrawals(
		startRange: bigint,
		endRange: bigint,
	): Promise<Withdrawal[]> {
		const requests: (Withdrawal | Cancel)[] = await this.getRequests(
			startRange,
			endRange,
		);
		return requests.filter((item): item is Withdrawal => {
			return (item as Withdrawal).withdrawalRecipient !== undefined;
		});
	}

	//TODO: rename
	async getRequests(
		startRange: bigint,
		endRange: bigint,
	): Promise<(Withdrawal | Cancel)[]> {
		const chain = getL1ChainType(this.api, this.chainId);
		let range = createBigIntArrayFromRange(startRange, endRange);
		const requests = await Promise.all(
			range.map((idx: bigint) =>
				this.api.query.rolldown.l2Requests(chain, {
					origin: "L2",
					id: idx.toString(),
				}),
			),
		);
		const withdrawalsOrCancels = requests
			.filter((elem: Option<ITuple<[PalletRolldownL2Request, H256]>>) => {
				if (elem.isNone) {
					return false;
				}

				let [req, _] = elem.unwrap();
				return req.isWithdrawal || req.isCancel;
			})
			.map((elem: Option<ITuple<[PalletRolldownL2Request, H256]>>) => {
				const [req, requestHash] = elem.unwrap();
				if (req.isWithdrawal) {
					return {
						requestId: BigInt(req.asWithdrawal.requestId.id.toString()),
						withdrawalRecipient: req.asWithdrawal.withdrawalRecipient,
						tokenAddress: req.asWithdrawal.tokenAddress.toU8a(),
						amount: BigInt(req.asWithdrawal.amount.toString()),
						ferryTip: BigInt(req.asWithdrawal.ferryTip.toString()),
						hash: requestHash.toU8a(),
					};
				} else {
					return {
						requestId: BigInt(req.asCancel.requestId.id.toString()),
						startRange: BigInt(req.asCancel.range.start.toString()),
						endRange: BigInt(req.asCancel.range.end.toString()),
						properHash: req.asCancel.hash_.toU8a(),
						hash: requestHash,
					};
				}
			});
		return withdrawalsOrCancels;
	}
}
export { L2Api, getL1ChainType, getApi };
