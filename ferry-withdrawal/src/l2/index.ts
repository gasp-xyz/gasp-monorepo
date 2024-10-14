import { type ApiPromise } from "@polkadot/api";
import type { KeyringPair } from "@polkadot/keyring/types";
import type { BTreeMap, u128 } from '@polkadot/types-codec';
import type {
	PalletRolldownMessagesChain,
} from "@polkadot/types/lookup";
import {  PalletRolldownL2Request } from '@polkadot/types/lookup';
import type { H256 } from '@polkadot/types/interfaces/runtime';

import type { Option} from '@polkadot/types-codec';
import type { ITuple } from '@polkadot/types-codec/types';
import {
	L1_CHAIN,
} from "../common/constants.js";
import { Withdrawal } from "../common/withdrawal.js";

function createBigIntArrayFromRange(start:bigint, end:bigint) {
  const result = [];
  for (let i = start; i <= end; i++) {
    result.push(i);
  }
  return result;
}

interface L2Interface {
	getLatestRequestId(): Promise<bigint|null>;
	getLatestRequestIdInPast(delay: number): Promise<bigint|null>;
  getWithdrawals(startRange: bigint, endRange: bigint): Promise<Withdrawal[]>;
	getNativeTokenAddress(): Promise<Uint8Array>;
}

function getL1ChainType(api: ApiPromise): PalletRolldownMessagesChain {
	return api.createType("PalletRolldownMessagesChain", L1_CHAIN);
}

class L2Api implements L2Interface {
	api!: ApiPromise;
	keyring!: KeyringPair;

	constructor(api: ApiPromise) {
		this.api = api;
	}

	async getNativeTokenAddress(): Promise<Uint8Array> {
		return (await this.api.query.assetRegistry.idToL1Asset(0))
			.unwrap()
			.asEthereum.toU8a();
	}


  parseLatestRequestId(nextRequesId: BTreeMap<PalletRolldownMessagesChain, u128>): bigint|null {
    // NOTE: looks like === is not implemented for PalletRolldownMessagesChain
    // therefore its not possible to query valu from map using .get(chain) query ;<
    const chain: PalletRolldownMessagesChain = getL1ChainType(this.api);
    let found = Array.from(nextRequesId.keys()).findIndex( (key) => {
      return key.toString() === chain.toString();
    });

    if (found == -1){
      return null;
    } else {
      return Array.from(nextRequesId.values())[found].toBigInt() - 1n;
    }
  }

  async getLatestRequestId(): Promise<bigint|null> {
    let nextRequesId = await this.api.query.rolldown.l2OriginRequestId();
    return this.parseLatestRequestId(nextRequesId);
  }

  async getLatestRequestIdInPast(blockInPast: number): Promise<bigint|null> {
    const { number } = await this.api.rpc.chain.getHeader();
    const targetBlock = Math.max(0, number.toNumber() - blockInPast);
    const targetBlockHash = await this.api.query.system.blockHash(targetBlock);
    let apiAt = await this.api.at(targetBlockHash);
    let nextRequesId = await apiAt.query.rolldown.l2OriginRequestId();
    return this.parseLatestRequestId(nextRequesId);
  }


  async getWithdrawals(startRange: bigint, endRange: bigint): Promise<Withdrawal[]> {
    const chain = getL1ChainType(this.api);
    let range = createBigIntArrayFromRange(startRange, endRange);
    const requests = await Promise.all(range.map( (idx: bigint) => this.api.query.rolldown.l2Requests(chain,{ origin: "L2", id: idx.toString()})));
    const withdrawals = requests.filter( (elem: Option<ITuple<[PalletRolldownL2Request, H256]>>) => {
      if (elem.isNone) {
        return false;
      }

      let [req, hash] = elem.unwrap();
      return req.isWithdrawal;
    })
    .map( (elem: Option<ITuple<[PalletRolldownL2Request, H256]>> ) => {
      const [req, _hash] = elem.unwrap();
      return {
        requestId: BigInt(req.asWithdrawal.requestId.id.toString()),
        withdrawalRecipient: req.asWithdrawal.withdrawalRecipient,
        tokenAddress: req.asWithdrawal.tokenAddress.toU8a(),
        amount: BigInt(req.asWithdrawal.amount.toString()),
        ferryTip: BigInt(req.asWithdrawal.ferryTip.toString()),
        hash: _hash.toU8a(),
      };
    })
    return withdrawals;
  }

}
export { L2Interface, L2Api, getL1ChainType };
