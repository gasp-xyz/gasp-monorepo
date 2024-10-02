
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

interface L1Interface {
	isRolldownDeployed(): Promise<boolean>;
	getLatestRequestId(): Promise<bigint | null>;
	getDeposits(rangeStart: bigint, rangeEnd: bigint): Promise<Deposit[]>;
	getDepostiHash(requestId: bigint): Promise<Uint8Array>;
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
		return code !== undefined && code !== "0x";
	}
}

export { L1Interface, L1Api };
