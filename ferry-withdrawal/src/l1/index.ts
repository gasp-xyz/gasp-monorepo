
import {
  Deposit
} from "../common/deposit.js";

import {
	ABI,
	MANGATA_CONTRACT_ADDRESS,
} from "../common/constants.js";

import {  createPublicClient } from "viem";
import { type PublicClient, encodeAbiParameters, keccak256 } from "viem";
import { hexToU8a } from "@polkadot/util";
import {
	http,
	webSocket,
} from "viem";
import { Withdrawal } from "../common/withdrawal.js";

function minBigInt(lhs: bigint, rhs: bigint): bigint {
  return [lhs, rhs].reduce((min, current) => current < min ? current : min);
}

interface L1Interface {
	isRolldownDeployed(): Promise<boolean>;
  getFerriedAndReadyToClose(rangeStart: bigint , rangeEnd: bigint ): Promise<Withdrawal[]>;
  getLatestRequestId(): Promise<bigint| null>;

  closeWithdrawal(withdrawalId: bigint): Promise<void>;
  ferryWithdrawal(withdrawalId: bigint): Promise<void>;
  canBeFerried(withdrawalId: bigint): Promise<boolean>;
  canBeClosed(withdrawalId: bigint): Promise<boolean>;
}

class L1Api implements L1Interface {
	client!: PublicClient;
  delay: bigint;

	constructor(uri: string, delay: bigint) {
    this.delay = delay;
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

	async isRolldownDeployed(): Promise<boolean> {
    const blockNumber = await this.client.getBlockNumber();
    if (blockNumber < this.delay) {
      return false;
    }
    const blockToReadAt = blockNumber - this.delay;

		const code = await this.client.getCode({
			address: MANGATA_CONTRACT_ADDRESS,
			blockNumber: blockToReadAt,
		});
		return code !== undefined && code !== "0x";
	}

	async getLatestRequestId(): Promise<bigint | null> {
		const value = await this.client.readContract({
			address: MANGATA_CONTRACT_ADDRESS,
			abi: ABI,
			functionName: "counter",
			blockTag: "latest"
		});
		const reqId = BigInt(value as any) - 1n;
		if (reqId < 1n) {
			return null;
		}
		return reqId;
	}

  async getFerriedAndReadyToClose(rangeStart: bigint, rangeEnd: bigint): Promise<Withdrawal[]> {
    const latest = await this.getLatestRequestId();
    if (latest == null ){
      return [];
    }

    const end = minBigInt(rangeEnd, latest);
    const value = await this.client.readContract({
      address: MANGATA_CONTRACT_ADDRESS,
      abi: ABI,
      functionName: "counter",
      blockTag: "latest"
    });

    return [];
  }

  async closeWithdrawal(withdrawalId: bigint): Promise<void> {
    throw new Error("Method not implemented.");
  }

  async ferryWithdrawal(withdrawalId: bigint): Promise<void> {
    throw new Error("Method not implemented.");
  }

  canBeFerried(withdrawalId: bigint): Promise<boolean> {
    throw new Error("Method not implemented.");
  }

  canBeClosed(withdrawalId: bigint): Promise<boolean> {
    throw new Error("Method not implemented.");
  }

}

export { L1Interface, L1Api };
