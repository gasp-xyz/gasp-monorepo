import { Mangata, signTx } from "@mangata-finance/sdk";
import "@mangata-finance/types";
import { Keyring } from "@polkadot/api";
import "dotenv/config";
import { createPublicClient, webSocket } from "viem";

import { mangataContractAbi } from "./mangataAbi.js";

type ContractAddress = `0x${string}`;

const mangataContractAddress = process.env
	.MANGATA_CONTRACT_ADDRESS! as ContractAddress;

async function main() {
	const api = await Mangata.instance([process.env.MANGATA_URL!]).api();
	console.log(
		`Connected ${api.isConnected}: Url: ${process.env.MANGATA_URL!}`,
		api.isConnected,
	);

	const keyring = new Keyring({ type: "sr25519" });
	const collator = keyring.addFromMnemonic(process.env.MNEMONIC!);

	const publicClient = createPublicClient({
		transport: webSocket(process.env.ETH_CHAIN_URL, {
			retryCount: 5,
		}),
	});

	await api.derive.chain.subscribeNewHeads(async (header) => {
		console.log(`block #${header.number} was authored by ${header.author}`);

		if (header.author?.toString() === collator.address) {
			const data = await publicClient.readContract({
				address: mangataContractAddress,
				abi: mangataContractAbi,
				functionName: "getUpdateForL2",
			});

			await signTx(api, api.tx.rolldown.updateL2FromL1(data), collator);
		}
	});
}

main()
	.then(() => {
		console.log("Success");
	})
	.catch((e) => console.error("Something went wrong", e));
