import { Mangata } from "@mangata-finance/sdk";
import "@mangata-finance/types";
import "@mangata-finance/types";
import { ApiPromise } from "@polkadot/api";
import "dotenv/config";
import {
	createPublicClient,
	createWalletClient,
	webSocket,
} from "viem";
import { defineChain } from "viem";
import { decodeAbiParameters } from "viem";
import { privateKeyToAccount } from "viem/accounts";
import { holesky } from "viem/chains";
import {print} from "./util/utils.js";
import "./util/polyfill.js"
import {
	ROLLDOWN_ABI,
	EIGEN_ABI,
	MANGATA_CONTRACT_ADDRESS,
	EIGEN_CONTRACT_ADDRESS,
	LIMIT,
	VERBOSE, FINALIZATION_SOURCE, MANGATA_NODE_URL, WALLET_PRIVATE_KEY, ETH_CHAIN_URL
} from "./common/constants.js";
import {getChain, getPublicClient, getWalletClient, webSocketTransport, ethAccount} from "./viem/client.js";



function getMinRequestId(l2Update: any) {
	let minId = Math.min.apply(null,
		[
			l2Update[0].withdrawals,
			l2Update[0].cancels,
			l2Update[0].results
		].flat()
			.map(function(item: any) {
				return Number(item.requestId.id);
			}))

	if (minId === Infinity) {
		return null;
	} else {
		return minId;
	}
}


function getMaxRequestId(l2Update: any) {
	let maxId = Math.max.apply(null,
		[
			l2Update[0].withdrawals,
			l2Update[0].cancels,
			l2Update[0].results
		].flat()
			.map(function(item: any) {
				return Number(item.requestId.id);
			}));

	if (maxId === -Infinity) {
		return null;
	} else {
		return maxId;
	}
}

function filterUpdates(l2Update: any, lastSubmittedId: number): any {

	l2Update[0].withdrawals = l2Update[0].withdrawals.filter((elem: any) => {
		return elem.requestId.id > lastSubmittedId;
	});
	l2Update[0].cancels = l2Update[0].cancels.filter((elem: any) => {
		return elem.requestId.id > lastSubmittedId;
	});
	l2Update[0].results = l2Update[0].results.filter((elem: any) => {
		return elem.requestId.id > lastSubmittedId;
	});

	const minId = getMinRequestId(l2Update);
	if (minId == null) {
		return l2Update;
	}

	const maxAmountOfUpdates = LIMIT
	if (maxAmountOfUpdates > 0) {
		l2Update[0].withdrawals = l2Update[0].withdrawals.filter((elem: any) => {
			return elem.requestId.id < BigInt(minId + maxAmountOfUpdates);
		});
		l2Update[0].cancels = l2Update[0].cancels.filter((elem: any) => {
			return elem.requestId.id < BigInt(minId + maxAmountOfUpdates);
		});
		l2Update[0].results = l2Update[0].results.filter((elem: any) => {
			return elem.requestId.id < BigInt(minId + maxAmountOfUpdates);
		});
		return l2Update;
	} else {
		return l2Update;
	}

}


async function sendUpdateToL1(
	api: ApiPromise,
	walletClient: any,
	publicClient: any,
	blockHash: any,
) {
	print(`HASH: ${blockHash} `);
	const pendingUpdates = await api.rpc.rolldown.pending_updates(blockHash);

	const l2Update = decodeAbiParameters(
		ROLLDOWN_ABI.find((e: any) => e.name === "update_l1_from_l2")!.inputs,
		pendingUpdates.toHex(),
	);

	if (VERBOSE) {
		print(`l2Update:  ${JSON.stringify(l2Update, null, 2)}`);
	}

	const lastSubmittedId = (await publicClient.readContract({
		address: MANGATA_CONTRACT_ADDRESS,
		abi: ROLLDOWN_ABI,
		functionName: "lastProcessedUpdate_origin_l2",
	})) as number;

	print(`L1::lastSubmittedId: ${lastSubmittedId}`);
	const update = filterUpdates(l2Update, lastSubmittedId);
	if (VERBOSE) {
		console.log(`filtered l2Update:  ${JSON.stringify(update, null, 2)}`);
	}

	const reqCount =
		update[0].withdrawals.length +
		update[0].cancels.length +
		update[0].results.length;

	if (reqCount == 0) {
		return null;
	}

	const storageHash = await walletClient.writeContract({
		chain: getChain(),
		abi: ROLLDOWN_ABI,
		address: MANGATA_CONTRACT_ADDRESS,
		functionName: "update_l1_from_l2",
		args: update,
		// gas: 9999999n,
	});

	return storageHash;
}

async function main() {
	const api = await Mangata.instance([MANGATA_NODE_URL]).api();
	const publicClient = getPublicClient({transport: webSocketTransport, chain: getChain()})
	const walletClient = getWalletClient({
		account: ethAccount,
		transport: webSocketTransport
	})

	let unwatch: any;
	let inProgress = false;

	if (FINALIZATION_SOURCE === "relay") {
		unwatch = await api.rpc.chain.subscribeFinalizedHeads(async (header) => {
			if (inProgress === false) {
				inProgress = true;
				print(`Chain is at block: #${header.number}`);

				const txHash = await sendUpdateToL1(api, walletClient, publicClient, header.hash);
				if (txHash) {
					const result = await publicClient.waitForTransactionReceipt({
						hash: txHash,
					});
					print(`#${result.blockNumber} ${result.transactionHash} : ${result.status}`);
				}
				inProgress = false;
			} else {
				print(`Chain is at block: #${header.number} - tx pending`);
			}
		});
	} else {
		print("Subscribing to eth events");
		unwatch = publicClient.watchContractEvent({
			address: EIGEN_CONTRACT_ADDRESS,
			abi: EIGEN_ABI,
			eventName: "TaskCompleted",
			onLogs: async (logs) => {
				print("Received task notification from L1");
				for (const log of logs) {
					const txHash = await sendUpdateToL1(
						api,
						walletClient,
						publicClient,
						(log as any).args.blockHash,
					);
					if (txHash) {
						const result = await publicClient.waitForTransactionReceipt({
							hash: txHash,
						});
						print(`#${result.blockNumber} ${result.transactionHash} : ${result.status}`,);
					}
				}
			},
		});
	}

	// We need to  unwatch if server is down or restarted to prevent leaks
	process.on("SIGINT", async () => {
		console.error("Stopping the process..., SIGINT signal");
		unwatch(); // Unsubscribe from event watching
		process.exit();
	});

	process.on("SIGTERM", async () => {
		console.error("Stopping the process..., SIGTERM signal");
		unwatch(); // Unsubscribe from event watching
		process.exit();
	});

	process.on("SIGHUP", async () => {
		console.error("Stopping the process..., SIGHUP signal");
		unwatch(); // Unsubscribe from event watching
		process.exit();
	});
}

main()
	.then(() => {
		print("Success");
	})
	.catch((e) => console.error("Something went wrong", e));
