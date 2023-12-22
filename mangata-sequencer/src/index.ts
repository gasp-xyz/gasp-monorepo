import { Mangata } from "@mangata-finance/sdk";
import "@mangata-finance/types";
import "dotenv/config";
import { createPublicClient, webSocket } from "viem";

import { mangataContractAbi } from "./mangataAbi.js";

type ContractAddress = `0x${string}`;

const mangataContractAddress = process.env
	.MANGATA_CONTRACT_ADDRESS! as ContractAddress;

async function main() {
	const api = await Mangata.instance([process.env.MANGATA_URL!]).api();
	console.log("api", api.isConnected);

	const transport = webSocket(process.env.ETH_CHAIN_URL, {
		retryCount: 5,
	});

	// We need public client in order to read contract
	const publicClient = createPublicClient({
		transport,
	});

	// subscription for new heads
	await api.derive.chain.subscribeNewHeads(async (header) => {
		console.log(`Block author: #${header.author}`);
		console.log(`Chain is at block hash: #${header.hash}`);

		// TODO: we need to get somehow the collator address which is running this sequencer
		// TODO: and we read contract only when the collator is building the block
		// TODO: and of course sending TX to mangata node
		if (header.author?.toString() === "") {
			const data = await publicClient.readContract({
				address: mangataContractAddress,
				abi: mangataContractAbi,
				functionName: "getUpdateForL2",
			});

			console.log("data", data);

			// TODO: we need to send TX to magnata with those data
		}
	});
}

main()
	.then(() => {
		console.log("Success");
	})
	.catch((e) => console.error("Something went wrong", e));
