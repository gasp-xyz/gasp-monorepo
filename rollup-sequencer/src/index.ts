import util from "util";
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

const blockNumberDelay = process.env.BLOCK_NUMBER_DELAY! || 5;

function sleep_ms(ms: number) {
	return new Promise((resolve) => setTimeout(resolve, ms));
}

async function main() {
	let lastSubmitted = "";
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

			try {
				const latestBlockNumber = await publicClient.getBlockNumber()
				const delayedBlockNumber = latestBlockNumber - BigInt(blockNumberDelay)

				console.log("Latest Block Number: " + latestBlockNumber.toString())
				console.log("Delayed Block Number: " + delayedBlockNumber.toString())

				const data = (await publicClient.readContract({
					address: mangataContractAddress,
					abi: abi,
					functionName: "getUpdateForL2",
					blockNumber: delayedBlockNumber,
				})) as any;

				console.log(util.inspect(data, { depth: null }));

				// @ts-ignore
				const encodedData = encodeAbiParameters(
					abi.find((e: any) => e!.name === "getUpdateForL2")!.outputs!,
					[data],
				);

				const nativeL1Update = await api.rpc.rolldown.get_native_l1_update(
					encodedData.substring(2),
				);

				if (lastSubmitted !== keccak256(encodedData)) {
					await signTx(
						api,
						api.tx.rolldown.updateL2FromL1(nativeL1Update.unwrap()),
						collator,
					);
					lastSubmitted = keccak256(encodedData);
				} else {
					console.log(`L1Update was already submitted ${encodedData}`);
				}
			} catch (e) {
				// Do nothing with error
				// Error only appear when we have block where there are no data for getUpdateForL2 at all.
				// This is only in the very beginning
				// ContractFunctionExecutionError: The contract function "getUpdateForL2" returned no data ("0x").
			}
		}

		const events = await apiAt.query.system.events();

		const pendingRequestsEvents = events.filter(
			(event) =>
				event.event.section === "rolldown" &&
				event.event.method === "L1ReadStored",
		);

		if (pendingRequestsEvents.length > 0) {
			pendingRequestsEvents.forEach((record) => {
				record.event.data.forEach(async (data, index) => {
					const requestId = (data as unknown as string[])[1];
					const { start, end } = (data as any)[2] as unknown as {
						start: string;
						end: string;
					};

					const contractData = (await publicClient.readContract({
						address: mangataContractAddress,
						abi: abi,
						functionName: "getPendingRequests",
						args: [start, end],
					})) as any;

					// @ts-ignore
					const encodedData = encodeAbiParameters(
						abi.find((e: any) => e!.name === "getPendingRequests")!.outputs!,
						[contractData],
					);

					const verified = await api.rpc.rolldown.verify_pending_requests(
						keccak256(encodedData),
						requestId.toString(),
					);

					if (!verified.toPrimitive()) {
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
