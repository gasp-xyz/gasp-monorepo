import { Mangata, signTx } from "@mangata-finance/sdk";
import "@mangata-finance/types";
import { Keyring } from "@polkadot/api";
import "dotenv/config";
import { createPublicClient, webSocket } from "viem";

import * as fs from "fs";
import path from "node:path";
import { fileURLToPath } from "url";
import { mangataContractAbi } from "./mangataAbi.js";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

type ContractAddress = `0x${string}`;

const mangataContractAddress = process.env
	.MANGATA_CONTRACT_ADDRESS! as ContractAddress;

async function main() {
	const api = await Mangata.instance([process.env.MANGATA_URL!]).api();
	console.log("api", api.isConnected);

	// ----------------KEYPAIR for Secret KEY--------------------------
	// TODO: We need to think about how to add secret key in order to make transaction
	const file = fs.readFileSync(path.resolve(__dirname, "./polkadot.json"));
	const keyring = new Keyring({ type: "sr25519" });
	// @ts-ignore
	const user = keyring.createFromJson(JSON.parse(file));
	keyring.addPair(user);
	keyring.pairs[0].decodePkcs8(process.env.ACCOUNT_SECRET_PASSWORD!);
	// ----------------------------------------------------------------

	/*
	* Public client - for reading the contract
	* */
	const publicClient = createPublicClient({
		transport: webSocket(process.env.ETH_CHAIN_URL, {
			retryCount: 5,
		}),
	});
	await api.derive.chain.subscribeNewHeads(async (header) => {
		console.log(`Block author: #${header.author}`);

		// TODO: we need to get somehow the collator address which is running this sequencer
		// TODO: and we read contract only when the collator is building the block
		// TODO: and of course sending TX to mangata node
		if (header.author?.toString() === "") {
			const data = await publicClient.readContract({
				address: mangataContractAddress,
				abi: mangataContractAbi,
				functionName: "getUpdateForL2",
			});

			await signTx(api, api.tx.rolldown.updateL2FromL1(data), user);
		}
	});
}

main()
	.then(() => {
		console.log("Success");
	})
	.catch((e) => console.error("Something went wrong", e));
