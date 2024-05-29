import { signTx } from "@mangata-finance/sdk";
import "@mangata-finance/types";
import type { HeaderExtended } from "@polkadot/api-derive/type/types";
import "dotenv/config";
import { keccak256 } from "viem";
import { MANGATA_NODE_URL, MNEMONIC } from "./common/constants.js";
import {
	countRequests,
	filterUpdates,
	getApi,
	getCollator,
	getLastRequestId,
	getMaxRequestId,
	getSelectedSequencerWithRights,
	initReadContractWithRetry,
	isSuccess,
	print,
	processDataForL2Update,
	processPendingRequestsEvents,
} from "./utils/index.js";
import { getPublicClient } from "./viem/client.js";
import { webSocketTransport } from "./viem/transport.js";

async function main() {
	let lastSubmitted = "";
	let inProgress = false;

	const publicClient = getPublicClient({ transport: webSocketTransport });

	const api = await getApi(MANGATA_NODE_URL);

	await initReadContractWithRetry(publicClient, api);

	let lastRequestId = await getLastRequestId(api);

	await api.derive.chain.subscribeNewHeads(async (header: HeaderExtended) => {
		const collator = getCollator("ethereum", MNEMONIC);
    print(`collator address: ${collator.address}`)

		print(`block #${header.number} was authored by ${header.author}`);
		const { isSequencerSelected, hasSequencerRights, selectedSequencer } =
			await getSelectedSequencerWithRights(api, collator.address, header.hash);
    print(`is selected ${isSequencerSelected}`);
    print(`rights : ${hasSequencerRights}`);
		if (isSequencerSelected && hasSequencerRights) {
			print(`Sequencer selected: ${selectedSequencer}`);
			try {
				if (inProgress) {
					print("In progress, skipping...");
				} else {
					inProgress = true;
				}
				const { encodedData, nativeL1Update } = await processDataForL2Update(
					api,
					publicClient,
				);

				const filteredUpdates = filterUpdates(
					nativeL1Update.unwrap(),
					lastRequestId,
				);
				const requestsCount = countRequests(filteredUpdates);

				if (requestsCount > 0) {
					const result = await signTx(
						api,
						api.tx.rolldown.updateL2FromL1(filteredUpdates),
						collator,
					);

					if (isSuccess(result)) {
						print("L1update was submitted successfully");

						if (lastSubmitted === keccak256(encodedData)) {
							lastRequestId = getMaxRequestId(filteredUpdates)!;
						} else {
							lastSubmitted = keccak256(encodedData);
							lastRequestId = await getLastRequestId(api);
						}
					} else {
						print("L1update was submitted unsuccessfully");
					}
				} else {
					print(`L1Update was already submitted ${encodedData}`);
				}
			} catch (e) {
				print(e);
				print("The contract function getUpdateForL2 returned no data");
				// Do nothing with error
				// Error only appear when we have block where there are no data for getUpdateForL2 at all.
				// This is only in the very beginning
				// ContractFunctionExecutionError: The contract function "getUpdateForL2" returned no data ("0x").
			}
			inProgress = false;
		}

		await processPendingRequestsEvents(
			api,
			publicClient,
			header.hash,
			collator,
		);
	});
}

main()
	.then(() => {
		print("Success");
	})
	.catch((e) => console.error("Something went wrong", e));
