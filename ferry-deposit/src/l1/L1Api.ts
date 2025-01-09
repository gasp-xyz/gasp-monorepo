import { Deposit } from "../Deposit.js";

import { ABI, MANGATA_CONTRACT_ADDRESS } from "../config.js";

import { createPublicClient } from "viem";
import { type PublicClient, encodeAbiParameters, keccak256 } from "viem";
import { hexToU8a } from "@polkadot/util";
import { L1Interface } from "./L1Interface.js";
import { http, webSocket } from "viem";

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

	async getDepostiHash(requestId: bigint): Promise<Uint8Array> {
		const value = await this.client.readContract({
			address: MANGATA_CONTRACT_ADDRESS,
			abi: ABI,
			functionName: "deposits",
			blockTag: "latest",
			args: [requestId],
		});
		const hash = encodeAbiParameters(
			ABI.find((e: any) => e!.name === "deposits")!.outputs!,
			[...(value as any)],
		);
		return hexToU8a(keccak256(hexToU8a(hash)));
	}

	async getLatestRequestId(): Promise<bigint | null> {
		const blockNumber = await this.client.getBlockNumber({ cacheTime: 0 });
		if (blockNumber < this.delay) {
			return null;
		}
		const blockToReadAt = blockNumber - this.delay;

		const value = await this.client.readContract({
			address: MANGATA_CONTRACT_ADDRESS,
			abi: ABI,
			functionName: "counter",
			blockNumber: BigInt(blockToReadAt),
		});
		const reqId = BigInt(value as any) - 1n;
		if (reqId < 1n) {
			return null;
		}
		return reqId;
	}

	async getDeposits(rangeStart: bigint, rangeEnd: bigint): Promise<Deposit[]> {
		const blockNumber = await this.client.getBlockNumber();
		if (blockNumber < this.delay) {
			return Promise.resolve([]);
		}
		const blockToReadAt = blockNumber - this.delay;

		const selector = "getPendingRequests";
		const contractData = await this.client.readContract({
			address: MANGATA_CONTRACT_ADDRESS,
			abi: ABI,
			functionName: selector,
			args: [rangeStart, rangeEnd],
			blockNumber: blockToReadAt,
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
}

export { L1Interface, L1Api };
