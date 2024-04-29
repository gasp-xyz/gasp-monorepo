import { signTx } from "@mangata-finance/sdk";
import "@mangata-finance/types";
import type { HeaderExtended } from "@polkadot/api-derive/type/types";
import "dotenv/config";
import { keccak256 } from "viem";
import { MANGATA_NODE_URL, MNEMONIC } from "./common/constants.js";
import {
	getApi,
	getCollator, getSelectedSequencerWithRights,
	initReadContractWithRetry,
	print,
	processDataForL2Update,
	processPendingRequestsEvents,
} from "./utils/index.js";
import { getPublicClient } from "./viem/client.js";
import { webSocketTransport } from "./viem/transport.js";

async function main() {
	let lastSubmitted = "";

	const publicClient = getPublicClient({ transport: webSocketTransport });

	const api = await getApi(MANGATA_NODE_URL);

	await initReadContractWithRetry(publicClient);

	await api.derive.chain.subscribeNewHeads(async (header: HeaderExtended) => {
		const collator = getCollator("ethereum", MNEMONIC);

		print(`block #${header.number} was authored by ${header.author}`);
		const {isSequencerSelected, hasSequencerRights, selectedSequencer} = await getSelectedSequencerWithRights(api, collator.address, header.hash)
		if (isSequencerSelected && hasSequencerRights) {
			print(`Sequencer selected: ${selectedSequencer}`)
			try {

				const { encodedData, nativeL1Update } = await processDataForL2Update(
					api,
					publicClient,
				);

				if (lastSubmitted !== keccak256(encodedData)) {
					await signTx(
						api,
						api.tx.rolldown.updateL2FromL1(nativeL1Update.unwrap()),
						collator,
					);
					lastSubmitted = keccak256(encodedData);
				} else {
					print(`L1Update was already submitted ${encodedData}`);
				}
			} catch (e) {
				print("The contract function getUpdateForL2 returned no data");
				// Do nothing with error
				// Error only appear when we have block where there are no data for getUpdateForL2 at all.
				// This is only in the very beginning
				// ContractFunctionExecutionError: The contract function "getUpdateForL2" returned no data ("0x").
			}
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
