import { Mangata } from "@mangata-finance/sdk";
import "@mangata-finance/types";
import "@mangata-finance/types";
import type { ApiPromise } from "@polkadot/api";
import "dotenv/config";
import {
	TestClient,
	WalletClient,
	createPublicClient,
	createWalletClient,
	keccak256,
	webSocket,
} from "viem";
import { defineChain } from "viem";
import { decodeAbiParameters } from "viem";
import { privateKeyToAccount } from "viem/accounts";
import eigenContractAbi from "./FinalizerTaskManager.json" assert {
	type: "json",
};
import rolldownAbi from "./RollDown.json" assert { type: "json" };

type ContractAddress = `0x${string}`;

const eigenContractAddress = process.env
	.EIGEN_CONTRACT_ADDRESS! as ContractAddress;

const mangataContractAddress = process.env
	.MANGATA_CONTRACT_ADDRESS! as ContractAddress;

const finalizationSource = process.env.FINALIZATION_SOURCE;
const verbose = process.env.VERBOSE;
const anvil = defineChain({
	id: 31337,
	name: "anvil",
	network: "Anvil",
	nativeCurrency: {
		decimals: 18,
		name: "Ether",
		symbol: "ETH",
	},
	rpcUrls: {
		public: {
			http: ["ws://127.0.0.1:8545"],
		},
		default: {
			http: ["ws://127.0.0.1:8545"],
		},
	},
});

let pendingUpdatesStored = "";

async function sendUpdateToL1(
	api: ApiPromise,
	walletClient: any,
	abi: any,
	blockHash: any,
) {
	console.log(`HASH ${blockHash} `);
	const pendingUpdates = await api.rpc.rolldown.pending_updates(blockHash);

	if (pendingUpdates.isEmpty){
		console.log("pendingUpdates is empty");
		return null;
	}

	if (verbose) {
		console.log(
			"----------------------------------------------------------------",
		);
		console.log("Fetched pending updates", keccak256(pendingUpdates.toHex()));
		console.log("Stored pending updates", pendingUpdatesStored);
		console.log(
			"----------------------------------------------------------------",
		);
	}

	if (pendingUpdatesStored === keccak256(pendingUpdates.toHex())) {
		console.log("Duplicate updates: ", keccak256(pendingUpdates.toHex()));
		return null;
	} else {
		pendingUpdatesStored = keccak256(pendingUpdates.toHex());
		const l2Update = decodeAbiParameters(
			abi.find((e: any) => e.name === "update_l1_from_l2")!.inputs,
			pendingUpdates.toHex(),
		);

		const reqCount =
			l2Update[0].withdrawals.length +
			l2Update[0].cancels.length +
			l2Update[0].results.length;

		if (verbose) {
			console.log(`l2Update:  ${JSON.stringify(l2Update, null, 2)}`);
		}

		if (reqCount > 0) {
			const storageHash = await walletClient.writeContract({
				chain: anvil, // TODO: this needs the chain in order to work properly
				abi: abi,
				address: mangataContractAddress,
				functionName: "update_l1_from_l2",
				args: l2Update as any,
				gas: 999999n,
			});
			return storageHash;
		}

		console.log("no updates available");
		return null;
	}
}

async function main() {
	const api = await Mangata.instance([process.env.MANGATA_NODE_URL!]).api();
	const abi = rolldownAbi.abi;

	// Ethereum private key
	// We need this to write to Mangata contract
	const account = privateKeyToAccount(`0x${process.env.WALLET_PRIVATE_KEY!}`);

	const transport = webSocket(process.env.ETH_CHAIN_URL, {
		retryCount: 5,
	});

	// We need wallet client in order to write to contract
	const walletClient = createWalletClient({
		account,
		transport,
	});

	// We need public client in order to read and subscribe to contract
	const publicClient = createPublicClient({
		transport,
		chain: anvil,
	});
	(BigInt.prototype as any).toJSON = function () {
		return this.toString();
	};

	let unwatch: any;

	if (finalizationSource === "relay") {
		unwatch = await api.rpc.chain.subscribeFinalizedHeads(async (header) => {
			console.log(`Chain is at block: #${header.number}`);
			const txHash = await sendUpdateToL1(api, walletClient, abi, header.hash);
			if (txHash) {
				const result = await publicClient.waitForTransactionReceipt({
					hash: txHash,
				});
				console.log(
					`#${result.blockNumber} ${result.transactionHash} : ${result.status}`,
				);
			}
		});
	} else {
		console.log("subscribing to eth events");
		unwatch = publicClient.watchContractEvent({
			address: eigenContractAddress,
			abi: eigenContractAbi.abi,
			eventName: "TaskCompleted",
			onLogs: async (logs) => {
				for (const log of logs) {
					const txHash = await sendUpdateToL1(
						api,
						walletClient,
						abi,
						(log as any).args.blockHash,
					);
					if (txHash) {
						const result = await publicClient.waitForTransactionReceipt({
							hash: txHash,
						});
						console.log(
							`#${result.blockNumber} ${result.transactionHash} : ${result.status}`,
						);
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
		console.log("Success");
	})
	.catch((e) => console.error("Something went wrong", e));
