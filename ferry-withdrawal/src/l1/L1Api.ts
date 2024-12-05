import { ABI, MANGATA_CONTRACT_ADDRESS } from "../Config.js";

import {
	BlockTag,
	PrivateKeyAccount,
	UnauthorizedProviderError,
	createPublicClient,
	createWalletClient,
} from "viem";
import { type PublicClient } from "viem";
import { hexToU8a, u8aToHex } from "@polkadot/util";
import { http, webSocket } from "viem";
import { Withdrawal } from "../Withdrawal.js";
import { Cancel } from "../Cancel.js";
import { privateKeyToAccount } from "viem/accounts";
import {
	anvil,
	holesky,
	arbitrumSepolia,
	localhost,
	type Chain,
} from "viem/chains";
import { isEqual } from "../utils.js";
import { estimateMaxPriorityFeePerGas } from "viem/actions";
import { L1Interface } from "./L1Interface.js";
import { logger } from "../logger.js";
import { L1_CHAIN } from "../Config.js";

async function estimateGasInWei(publicClient: PublicClient) {
	// https://www.blocknative.com/blog/eip-1559-fees
	// We do not want VIEM estimate we would like to make our own estimate
	// based on this equation: Max Fee = (2 * Base Fee) + Max Priority Fee

	// Max Fee = maxFeePerGas (viem)
	// Max Priority Fee = maxPriorityFeePerGas (viem)

	const baseFeeInWei = await publicClient.getGasPrice();
	const maxPriorityFeePerGasInWei =
		await estimateMaxPriorityFeePerGas(publicClient);
	const maxFeeInWei =
		BigInt(2) * BigInt(baseFeeInWei) + BigInt(maxPriorityFeePerGasInWei);
	return { maxFeeInWei, maxPriorityFeePerGasInWei };
}

function withdrawalToViemFormat(withdrawal: Withdrawal): unknown[] {
	return [
		[1, withdrawal.requestId],
		u8aToHex(withdrawal.withdrawalRecipient, 160),
		u8aToHex(withdrawal.tokenAddress, 160),
		withdrawal.amount,
		withdrawal.ferryTip,
	];
}

function cancelToViemFormat(cancel: Cancel): unknown[] {
	return [
		[1, cancel.requestId],
		[cancel.startRange, cancel.endRange],
		u8aToHex(cancel.properHash),
	];
}

const CONFIG_TO_CHAIN = new Map<string, Chain>([
	["anvil-arbitrum", anvil],
	["anvil-ethereum", anvil],
	["holesky", holesky],
	["arbitrum-sepolia", arbitrumSepolia],
	["reth-arbitrum", localhost],
	["reth-ethereum", localhost],
]);

class L1Api implements L1Interface {
	client!: PublicClient;
	transport: any;

	constructor(uri: string) {
		if (!CONFIG_TO_CHAIN.has(L1_CHAIN)) {
			throw new Error(`unknown chain '${L1_CHAIN}'`);
		}

		const chain = CONFIG_TO_CHAIN.get(L1_CHAIN);
		if (uri.startsWith("ws")) {
			this.transport = webSocket(uri, { retryCount: 5 });
			this.client = createPublicClient({
				transport: this.transport,
				chain,
			});
		} else if (uri.startsWith("http")) {
			this.transport = http(uri, { retryCount: 5 });
			this.client = createPublicClient({
				transport: this.transport,
				chain,
			});
		} else {
			throw new Error("Invalid uri");
		}
	}

	async getMerkleRange(
		requestId: bigint,
	): Promise<{ root: Uint8Array; range: [bigint, bigint] }> {
		const root = (await this.client.readContract({
			address: MANGATA_CONTRACT_ADDRESS,
			abi: ABI,
			functionName: "find_l2_batch",
			args: [requestId],
			blockTag: "latest",
		})) as any;

		const range = await this.client.readContract({
			address: MANGATA_CONTRACT_ADDRESS,
			abi: ABI,
			functionName: "merkleRootRange",
			args: [root],
			blockTag: "latest",
		});

		return {
			root: hexToU8a(root),
			range: range as [bigint, bigint],
		};
	}

	async getFerry(hash: Uint8Array): Promise<Uint8Array | null> {
		const closedStatus = u8aToHex(await this.getClosedStatus());
		const zeros = "0x0000000000000000000000000000000000000000";

		const status = (await this.client.readContract({
			address: MANGATA_CONTRACT_ADDRESS,
			abi: ABI,
			functionName: "processedL2Requests",
			args: [u8aToHex(hash)],
			blockTag: "latest",
		})) as string;

		return status !== null && status !== closedStatus && status !== zeros
			? hexToU8a(status)
			: null;
	}

	async getNativeTokenAddress(): Promise<Uint8Array> {
		let val = await this.client.readContract({
			address: MANGATA_CONTRACT_ADDRESS,
			abi: ABI,
			functionName: "NATIVE_TOKEN_ADDRESS",
			blockTag: "latest",
		});
		return hexToU8a(val as any);
	}

	async getBalance(
		tokenAddress: Uint8Array,
		account: Uint8Array,
	): Promise<bigint | null> {
		const nativeTokenAddress = await this.getNativeTokenAddress();
		if (isEqual(nativeTokenAddress, tokenAddress)) {
			return this.client.getBalance({ address: u8aToHex(account) });
		} else {
			try {
				return BigInt(
					(await this.client.readContract({
						address: u8aToHex(tokenAddress),
						abi: [
							{
								constant: true,
								inputs: [{ name: "owner", type: "address" }],
								name: "balanceOf",
								outputs: [{ name: "balance", type: "uint256" }],
								type: "function",
							},
						],
						functionName: "balanceOf",
						args: [u8aToHex(account)],
						blockTag: "latest",
					})) as any,
				);
			} catch (e) {
				return null;
			}
		}
	}

	async getClosedStatus(): Promise<Uint8Array> {
		let closedStatus = await this.client.readContract({
			address: MANGATA_CONTRACT_ADDRESS,
			abi: ABI,
			functionName: "CLOSED",
			blockTag: "latest",
		});
		return hexToU8a(closedStatus as string);
	}

	async isClosed(hash: Uint8Array): Promise<boolean> {
		const closedStatus = u8aToHex(await this.getClosedStatus());
		let status = await this.client.readContract({
			address: MANGATA_CONTRACT_ADDRESS,
			abi: ABI,
			functionName: "processedL2Requests",
			args: [u8aToHex(hash)],
			blockTag: "latest",
		});
		return status === closedStatus;
	}

	async isFerried(hash: Uint8Array): Promise<boolean> {
		const closedStatus = u8aToHex(await this.getClosedStatus());
		const zeros = "0x0000000000000000000000000000000000000000";

		let status = await this.client.readContract({
			address: MANGATA_CONTRACT_ADDRESS,
			abi: ABI,
			functionName: "processedL2Requests",
			args: [u8aToHex(hash)],
			blockTag: "latest",
		});

		return status !== closedStatus && status !== zeros;
	}

	async isRolldownDeployed(delay: bigint): Promise<boolean> {
		let currentBlock = await this.client.getBlockNumber();
		let atBlock: bigint;

		if (delay > currentBlock) {
			atBlock = 1n;
		} else {
			atBlock = currentBlock - delay;
		}

		if (delay > 0n) {
			logger.info(`Checking if rolldown is deployed at past block #${atBlock}`);
		} else {
			logger.info(
				`Checking if rolldown is deployed at current block #${currentBlock}`,
			);
		}

		let val = await this.client.readContract({
			address: MANGATA_CONTRACT_ADDRESS,
			abi: ABI,
			functionName: "NATIVE_TOKEN_ADDRESS",
			blockNumber: atBlock,
		});
		return (val as string) != "0x00000000000000000000000000000000000000000";
	}

	async getLatestRequestId(delay: bigint = 0n): Promise<bigint | null> {
		let blockTag: BlockTag | undefined = "latest";
		let blockNumber: bigint | undefined = undefined;

		if (delay > 0n) {
			let currentBlock = await this.client.getBlockNumber();
			if (currentBlock < delay) {
				blockTag = undefined;
				blockNumber = 0n;
			} else {
				blockTag = undefined;
				blockNumber = currentBlock - delay;
			}
		}

		const value = BigInt(
			(await this.client.readContract({
				address: MANGATA_CONTRACT_ADDRESS,
				abi: ABI,
				functionName: "getMerkleRootsLength",
				blockTag,
				blockNumber,
			})) as any,
		);

		if (value === 0n) {
			return null;
		} else {
			const lastHash = await this.client.readContract({
				address: MANGATA_CONTRACT_ADDRESS,
				abi: ABI,
				functionName: "roots",
				args: [value - 1n],
				blockTag,
				blockNumber,
			});

			const range = await this.client.readContract({
				address: MANGATA_CONTRACT_ADDRESS,
				abi: ABI,
				functionName: "merkleRootRange",
				args: [lastHash],
				blockTag,
				blockNumber,
			});
			return (range as any)[1];
		}
	}

	async ferry(
		withdrawal: Withdrawal,
		privateKey: Uint8Array,
	): Promise<boolean> {
		const acc: PrivateKeyAccount = privateKeyToAccount(
			u8aToHex(privateKey) as `0x{string}`,
		);
		const wc = createWalletClient({
			account: acc,
			chain: this.client.chain,
			transport: this.transport,
		});

		// TODO: submit as a batch
		const approveRequest = await this.client.simulateContract({
			account: acc,
			address: u8aToHex(withdrawal.tokenAddress),
			abi: [
				{
					constant: true,
					inputs: [
						{ name: "spender", type: "address" },
						{ name: "amount", type: "uint256" },
					],
					name: "approve",
					outputs: [{ name: "status", type: "bool" }],
					type: "function",
				},
			],
			functionName: "approve",
			args: [MANGATA_CONTRACT_ADDRESS, withdrawal.amount],
		});
		const approvetxHash = await wc.writeContract(approveRequest.request);
		await this.client.waitForTransactionReceipt({ hash: approvetxHash });

		const { maxFeeInWei, maxPriorityFeePerGasInWei } = await estimateGasInWei(
			this.client,
		);
		const ferryRequest = await this.client.simulateContract({
			account: acc,
			address: MANGATA_CONTRACT_ADDRESS,
			abi: ABI,
			functionName: "ferry_withdrawal",
			args: [withdrawalToViemFormat(withdrawal)],
			maxFeePerGas: maxFeeInWei,
			maxPriorityFeePerGas: maxPriorityFeePerGasInWei,
		});

		const ferrytxHash = await wc.writeContract(ferryRequest.request);
		const status = await this.client.waitForTransactionReceipt({
			hash: ferrytxHash,
		});
		return status.status === "success";
	}

	async closeWithdrawal(
		withdrawal: Withdrawal,
		merkleRoot: Uint8Array,
		proof: Uint8Array[],
		privateKey: Uint8Array,
	): Promise<boolean> {
		const acc: PrivateKeyAccount = privateKeyToAccount(
			u8aToHex(privateKey) as `0x{string}`,
		);
		const wc = createWalletClient({
			account: acc,
			chain: this.client.chain,
			transport: this.transport,
		});

		const { maxFeeInWei, maxPriorityFeePerGasInWei } = await estimateGasInWei(
			this.client,
		);
		const ferryRequest = await this.client.simulateContract({
			account: acc,
			address: MANGATA_CONTRACT_ADDRESS,
			abi: ABI,
			functionName: "close_withdrawal",
			args: [
				withdrawalToViemFormat(withdrawal),
				u8aToHex(merkleRoot),
				proof.map((p) => u8aToHex(p)),
			],
			maxFeePerGas: maxFeeInWei,
			maxPriorityFeePerGas: maxPriorityFeePerGasInWei,
		});

		const ferrytxHash = await wc.writeContract(ferryRequest.request);
		const status = await this.client.waitForTransactionReceipt({
			hash: ferrytxHash,
		});
		return status.status === "success";
	}

	async closeCancel(
		cancel: Cancel,
		merkleRoot: Uint8Array,
		proof: Uint8Array[],
		privateKey: Uint8Array,
	): Promise<boolean> {
		const acc: PrivateKeyAccount = privateKeyToAccount(
			u8aToHex(privateKey) as `0x{string}`,
		);
		const wc = createWalletClient({
			account: acc,
			chain: this.client.chain,
			transport: this.transport,
		});

		const { maxFeeInWei, maxPriorityFeePerGasInWei } = await estimateGasInWei(
			this.client,
		);
		const ferryRequest = await this.client.simulateContract({
			account: acc,
			address: MANGATA_CONTRACT_ADDRESS,
			abi: ABI,
			functionName: "close_cancel",
			args: [
				cancelToViemFormat(cancel),
				u8aToHex(merkleRoot),
				proof.map((p) => u8aToHex(p)),
			],
			maxFeePerGas: maxFeeInWei,
			maxPriorityFeePerGas: maxPriorityFeePerGasInWei,
		});

		const ferrytxHash = await wc.writeContract(ferryRequest.request);
		const status = await this.client.waitForTransactionReceipt({
			hash: ferrytxHash,
		});
		return status.status === "success";
	}
}

export { L1Interface, L1Api, withdrawalToViemFormat, cancelToViemFormat };
