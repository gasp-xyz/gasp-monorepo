import { type ApiPromise } from "@polkadot/api";
import type { KeyringPair } from "@polkadot/keyring/types";
import type {
	MangataTypesAssetsL1Asset,
	PalletRolldownMessagesChain,
} from "@polkadot/types/lookup";

import { L1_CHAIN } from "../config.js";
import { L2Interface } from "./L2Interface.js";

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
	} else if (chain.isBase) {
		return api.createType("MangataTypesAssetsL1Asset", {
			Base: tokenAddress,
		});
	} else {
		throw new Error(`Unknown chain id ${chain.toHuman()}`);
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
      //TODO : not used, and no point of aligning since we are moving to rust impl
      throw new Error('unimplemented');
      // return BigInt(
      // 	(
      // 		await this.api.rpc.xyk.calculate_sell_price(
      // 			tokenId.unwrap(),
      // 			0,
      // 			amount,
      // 		)
      // 	).toString(),
      // );
    }
	}

	async getNativeTokenAddress(): Promise<Uint8Array> {
		return (await this.api.query.assetRegistry.idToL1Asset(0))
			.unwrap()
			.asEthereum.toU8a();
	}

	async getBalance(address: Uint8Array): Promise<[Uint8Array, bigint][]> {
		const assetMapping =
			await this.api.query.assetRegistry.idToL1Asset.entries();
		const chain = getL1ChainType(this.api);

		const mapping: [bigint, Uint8Array][] = assetMapping
			.filter(
				([_key, value]) =>
					(value.unwrap().isEthereum && chain.isEthereum) ||
					(value.unwrap().isArbitrum && chain.isArbitrum) ||
					(value.unwrap().isBase && chain.isBase),
			)
			.map(([key, value]) => {
				const id = BigInt(key.args[0].toString());
				const address = value.unwrap();
				if (address.isArbitrum && chain.isArbitrum) {
					return [id, value.unwrap().asArbitrum.toU8a()];
				} else if (address.isEthereum && chain.isEthereum) {
					return [id, value.unwrap().asEthereum.toU8a()];
				} else if (address.isBase && chain.isBase) {
					return [id, value.unwrap().asBase.toU8a()];
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

		return values;
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
export { L2Api, getL1ChainType };
