import type { HeaderExtended } from "@polkadot/api-derive/type/types";
import "dotenv/config";
import { signTx } from "gasp-sdk";
import "gasp-types";
import { BaseError, keccak256 } from "viem";
import {
	MANGATA_NODE_URL,
	MNEMONIC,
	WATCHDOG_PERIOD,
} from "./common/constants.js";
import {
	WatchDog,
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
	const SECONDS_30 = 30 * 1000;
	const SECONDS_1 = 1 * 1000;
	const watchdogL1 = new WatchDog("L1", WATCHDOG_PERIOD);
	const watchdogL2 = new WatchDog("L1", WATCHDOG_PERIOD);
	let inProgress = false;

	const publicClient = getPublicClient({ transport: webSocketTransport });
	const api = await getApi(MANGATA_NODE_URL);
	await initReadContractWithRetry(publicClient, api);
	let lastRequestId = await getLastRequestId(api);

	setInterval(async () => {
		watchdogL1.check();
		watchdogL2.check();
	}, SECONDS_1);

	setInterval(async () => {
		await api.query.system
			.number()
			.then((blockNr) => watchdogL1.feed(blockNr.toString()));
	}, SECONDS_30);

	setInterval(async () => {
		await publicClient
			.getBlockNumber()
			.then((block) => watchdogL2.feed(block.toString()));
	}, SECONDS_30);

	await api.derive.chain.subscribeNewHeads(async (header: HeaderExtended) => {
		const collator = getCollator("ethereum", MNEMONIC);
		print(`collator address: ${collator.address}`);
		print(`block #${header.number} was authored by ${header.author}`);
		const {
			isSequencerSelected,
			hasReadRights,
			hasCancelRights,
			selectedSequencer,
		} = await getSelectedSequencerWithRights(
			api,
			collator.address,
			header.hash,
		);
		print(`me ${collator.address}`);
		print(`selected : ${selectedSequencer}`);
		print(`is selected ${isSequencerSelected}`);
		print(`has read rights : ${hasReadRights}`);
		print(`has cancel rights : ${hasCancelRights}`);
		if (isSequencerSelected && hasReadRights) {
			if (inProgress) {
				return;
			} else {
				print("In progress, skipping...");
				inProgress = true;
			}
			const nativeL1Update = await processDataForL2Update(api, publicClient);

			if (nativeL1Update === null) {
				inProgress = false;
				return;
			}

			const maxRequestId = getMaxRequestId(nativeL1Update);

			console.log(`maxRequestId: ${maxRequestId}`);
			if (maxRequestId > lastRequestId) {
				const result = await signTx(
					api,
					api.tx.rolldown.updateL2FromL1(nativeL1Update),
					collator,
				);

				if (isSuccess(result)) {
					print("L1update was submitted successfully");
					lastRequestId = maxRequestId;
				} else {
					print("L1update was submitted unsuccessfully");
				}
			} else {
				print(`L1Update with max id == ${lastRequestId} was already submitted`);
			}

			inProgress = false;
		}

		if (hasCancelRights && !inProgress) {
			inProgress = true;
			await processPendingRequestsEvents(api, publicClient, collator);
			inProgress = false;
		}
	});
}

main()
	.then(() => {
		print("Success");
	})
	.catch((e) => {
		console.error("Something went wrong", e);
		process.exit(1);
	});
