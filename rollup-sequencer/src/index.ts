import { Mangata, signTx } from "@mangata-finance/sdk";
import "@mangata-finance/types";
import { Keyring } from "@polkadot/api";
import "dotenv/config";
import { createPublicClient, encodeAbiParameters, webSocket } from "viem";
import { keccak256 } from "viem";
import rolldownAbi from "./RollDown.json" assert { type: "json" };

type ContractAddress = `0x${string}`;

const mangataContractAddress = process.env
	.MANGATA_CONTRACT_ADDRESS! as ContractAddress;

function sleep_ms(ms: number) {
	return new Promise((resolve) => setTimeout(resolve, ms));
}

async function main() {
	const abi = rolldownAbi.abi;
	const publicClient = createPublicClient({
		transport: webSocket(process.env.ETH_CHAIN_URL, {
			retryCount: 5,
		}),
	});

	while (true) {
		try {
			const data = (await publicClient.readContract({
				address: mangataContractAddress,
				abi: abi,
				functionName: "getUpdateForL2",
			})) as any;
			console.log(data);
			break;
		} catch (e) {
			console.log(`${mangataContractAddress} contract not found`);
			await sleep_ms(1000);
		}
	}

	const api = await Mangata.instance([process.env.MANGATA_NODE_URL!]).api();
	await api.isReady;

	const keyring = new Keyring({ type: "sr25519" });
	const collator = keyring.addFromUri(process.env.MNEMONIC!);

	await api.derive.chain.subscribeNewHeads(async (header) => {
		const apiAt = await api.at(header.hash);
		console.log(`block #${header.number} was authored by ${header.author}`);

		if (header.author?.toString() === collator.address) {
			const data = (await publicClient.readContract({
				address: mangataContractAddress,
				abi: abi,
				functionName: "getUpdateForL2",
			})) as any;

			console.log(data);

			data.order = data.order.map((e: any) => {
				switch (e) {
					case 0: {
						return "DEPOSIT";
					}
					case 1: {
						return "WITHDRAWAL";
					}
					case 2: {
						return "CANCEL_RESOLUTION";
					}
					case 3: {
						return "L2_UPDATES_TO_REMOVE";
					}
				}
			});

			await signTx(api, api.tx.rolldown.updateL2FromL1(data), collator);
		}
		const events = await apiAt.query.system.events();
		const pendingRequestsEvents = events.filter(
			(event) =>
				event.event.section === "rolldown" &&
				event.event.method === "PendingRequestStored",
		);

		if (pendingRequestsEvents.length > 0) {
			pendingRequestsEvents.forEach((record) => {
				record.event.data.forEach(async (data, index) => {
					const requestId = (data as unknown as string[])[1];
					const contractData = (await publicClient.readContract({
						address: mangataContractAddress,
						abi: abi,
						functionName: "getUpdateForL2",
					})) as any;
					// @ts-ignore
					const encodedData = encodeAbiParameters(
						abi.find((e: any) => e!.name === "getUpdateForL2")!.outputs!,
						[contractData],
					);

					const verified = await api.rpc.rolldown.verify_pending_requests(
						keccak256(encodedData),
						requestId.toString(),
					);

					const isVerified = Boolean(verified.toString());

					if (!isVerified) {
						await signTx(
							api,
							api.tx.rolldown.cancelRequestsFromL1(requestId.toString()),
							collator,
						);
					}
				});
			});
		}
	});
}

main()
	.then(() => {
		console.log("Success");
	})
	.catch((e) => console.error("Something went wrong", e));
