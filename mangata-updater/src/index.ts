import { Mangata } from "@mangata-finance/sdk";
import "dotenv/config";
import { createPublicClient, createWalletClient, webSocket } from "viem";
import { privateKeyToAccount } from "viem/accounts";
import { goerli } from "viem/chains"


import { eigenContractAbi } from "./eigenAbi.js";
import { mangataContractAbi } from "./mangataAbi.js";

type ContractAddress = `0x${string}`;

const eigenContractAddress = process.env
	.EIGEN_CONTRACT_ADDRESS! as ContractAddress;

const mangataContractAddress = process.env
	.MANGATA_CONTRACT_ADDRESS! as ContractAddress;

async function main() {
	const api = await Mangata.instance([process.env.MANGATA_URL!]).api();
	console.log("api", api.isConnected);

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
	});


	const unwatch = publicClient.watchContractEvent({
		address: eigenContractAddress,
		abi: eigenContractAbi,
		eventName: "TaskResponded",
		onLogs: async (logs) => {
			for (const log of logs) {

				const pendingUpdates = await api.query.rolldown.pendingUpdatesU256Array(log.blockNumber) as unknown as bigint[]

				// Executes TX on ETH with all pending_updates with hashes
				// Here we need to write to mangata contract
				const storageHash = await walletClient.writeContract({
					chain: goerli, // TODO: this needs the chain in order to work properly
					abi: mangataContractAbi,
					address: mangataContractAddress,
					functionName: "update_l1_from_l2",
					args: [pendingUpdates],
				});

				await publicClient.waitForTransactionReceipt({ hash: storageHash });
			}
		},
	});

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
