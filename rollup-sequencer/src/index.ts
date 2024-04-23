import util from "node:util";
import { Mangata, signTx } from "@mangata-finance/sdk";
import "@mangata-finance/types";
import "dotenv/config";
import {encodeAbiParameters} from "viem";
import { keccak256 } from "viem";
import {MANGATA_CONTRACT_ADDRESS, BLOCK_NUMBER_DELAY, ABI, getCollatorAddress, sleep} from "./utils"
import {getPublicClient} from "./viem/client";
import {webSocketTransport} from "./viem/transport";
import {configuration} from "./config";



async function main() {
	let lastSubmitted = "";

	const publicClient = getPublicClient({ transport: webSocketTransport})

	while (true) {
		try {

			const data = await publicClient.readContract({
				address: MANGATA_CONTRACT_ADDRESS,
				abi: ABI,
				functionName: "getUpdateForL2",
			})

			console.log(util.inspect(data, { depth: null }));

			break;
		} catch (e) {

			console.log(`${MANGATA_CONTRACT_ADDRESS} contract not found`);

			await sleep(1000);
		}
	}

	const api = await Mangata.instance([process.env.MANGATA_NODE_URL!]).api();
	await api.isReady;

	const collator = getCollatorAddress("ethereum", configuration.MNEMONIC)

	await api.derive.chain.subscribeNewHeads(async (header) => {
		const apiAt = await api.at(header.hash);
		console.log(`block #${header.number} was authored by ${header.author}`);

		try {
			const latestBlockNumber = await publicClient.getBlockNumber();
			const delayedBlockNumber = latestBlockNumber - BigInt(BLOCK_NUMBER_DELAY);

			console.log(`Latest Block Number: ${latestBlockNumber.toString()}`);
			console.log(`Delayed Block Number:  ${delayedBlockNumber.toString()}`);

			const data = (await publicClient.readContract({
				address: MANGATA_CONTRACT_ADDRESS,
				abi: ABI,
				functionName: "getUpdateForL2",
				blockNumber: delayedBlockNumber,
			})) as any;

			console.log(util.inspect(data, { depth: null }));

			// @ts-ignore
			const encodedData = encodeAbiParameters(
				ABI.find((e: any) => e!.name === "getUpdateForL2")!.outputs!,
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
						address: MANGATA_CONTRACT_ADDRESS,
						abi: ABI,
						functionName: "getPendingRequests",
						args: [start, end],
					})) as any;

					// @ts-ignore
					const encodedData = encodeAbiParameters(
						ABI.find((e: any) => e!.name === "getPendingRequests")!.outputs!,
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
