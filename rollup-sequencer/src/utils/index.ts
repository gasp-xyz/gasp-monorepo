import util from "node:util";
import {
	Mangata,
	type MangataGenericEvent,
	signTx,
} from "@mangata-finance/sdk";
import { type ApiPromise, Keyring } from "@polkadot/api";
import type { ApiDecoration } from "@polkadot/api/types";
import type { KeyringPair } from "@polkadot/keyring/types";
import type { Option } from "@polkadot/types-codec";
import type {
	FrameSystemEventRecord,
	PalletRolldownMessagesL1Update,
} from "@polkadot/types/lookup";
import { hexToU8a, u8aToHex } from "@polkadot/util";
import type { KeypairType } from "@polkadot/util-crypto/types";
import { type PublicClient, encodeAbiParameters, keccak256 } from "viem";
import {
	ABI,
	BLOCK_NUMBER_DELAY,
	L1_READ_STORED_EVENT_METHOD,
	LIMIT,
	MANGATA_CONTRACT_ADDRESS,
	ROLLDOWN_EVENT_SECTION,
} from "../common/constants.js";

function sleep(timeInMilliseconds: number): Promise<void> {
	return new Promise((resolve) => setTimeout(resolve, timeInMilliseconds));
}

function getKeyring(type: KeypairType): Keyring {
	return new Keyring({ type });
}

function getCollator(type: KeypairType, mnemonic: string): KeyringPair {
	const keyring = getKeyring(type);
	return keyring.addFromSeed(hexToU8a(mnemonic));
}

async function getApi(nodeUrl: string): Promise<ApiPromise> {
	const api = await Mangata.instance([nodeUrl]).api();
	await api.isReady;
	return api;
}

function getEncodedData(methodName: string, data: any): `0x${string}` {
	return encodeAbiParameters(
		ABI.find((e: any) => e!.name === methodName)!.outputs!,
		[data],
	);
}

async function getNativeL1Update(
	api: ApiPromise,
	encodedData: `0x${string}`,
): Promise<Option<PalletRolldownMessagesL1Update>> {
	return await api.rpc.rolldown.get_native_l1_update(encodedData.substring(2));
}

async function getEvents(
	apiAt: ApiDecoration<"promise">,
	section: string,
	method: string,
): Promise<FrameSystemEventRecord[]> {
	const events = await apiAt.query.system.events();

	return events.filter(
		(event) => event.event.section === section && event.event.method === method,
	);
}

async function initReadContractWithRetry(publicClient: PublicClient) {
	while (true) {
		try {
			return await getUpdateForL2(publicClient);
		} catch (e) {
			print(`${MANGATA_CONTRACT_ADDRESS} contract not found`);
			await sleep(1000);
		}
	}
}

async function processDataForL2Update(
	api: ApiPromise,
	publicClient: PublicClient,
) {
	const latestBlockNumber = await publicClient.getBlockNumber();
	const delayedBlockNumber = latestBlockNumber - BigInt(BLOCK_NUMBER_DELAY);

	print(`Latest Block Number: ${latestBlockNumber.toString()}`);
	print(`Delayed Block Number:  ${delayedBlockNumber.toString()}`);

	const data = await getUpdateForL2(publicClient);
	print(data);

	const encodedData = getEncodedData("getUpdateForL2", data);
	const nativeL1Update = await getNativeL1Update(api, encodedData);
	return {
		encodedData,
		nativeL1Update,
	};
}

async function processPendingRequestsEvents(
	api: ApiPromise,
	publicClient: PublicClient,
	headerHash: Uint8Array,
	collator: KeyringPair,
) {
	const apiAt = await api.at(headerHash);
	const pendingRequestsEvents = await getEvents(
		apiAt,
		ROLLDOWN_EVENT_SECTION,
		L1_READ_STORED_EVENT_METHOD,
	);

	if (pendingRequestsEvents.length > 0) {
		pendingRequestsEvents.forEach(async (record: FrameSystemEventRecord) => {
			record.event.data.forEach(async (data) => {
				const requestId = (data as unknown as string[])[1];
				const { start, end } = (data as any)[2] as unknown as {
					start: string;
					end: string;
				};

				const contractData = await publicClient.readContract({
					address: MANGATA_CONTRACT_ADDRESS,
					abi: ABI,
					functionName: "getPendingRequests",
					args: [start, end],
				});

				const encodedData = getEncodedData("getPendingRequests", contractData);

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
}

async function getSelectedSequencerWithRights(
	api: ApiPromise,
	collatorAddress: string,
	headerHash: Uint8Array,
) {
	const apiAt = await api.at(headerHash);
	const selectedSequencer =
		await apiAt.query.sequencerStaking.selectedSequencer();
	if (selectedSequencer.isSome) {
		const isSequencerSelected =
			u8aToHex(selectedSequencer.unwrap()).toLowerCase() ===
			collatorAddress.toLowerCase();
		const sequencerRights =
			await apiAt.query.rolldown.sequencerRights(collatorAddress);
		const hasSequencerRights =
			sequencerRights.unwrap().readRights.toNumber() > 0;
		return {
			isSequencerSelected,
			hasSequencerRights,
			selectedSequencer: u8aToHex(selectedSequencer.unwrap()).toLowerCase(),
		};
	}
	return {
		isSequencerSelected: false,
		hasSequencerRights: false,
		selectedSequencer: null,
	};
}

function isSuccess(events: MangataGenericEvent[]) {
	return events.some(
		(event) =>
			event.section === "system" && event.method === "ExtrinsicSuccess",
	);
}

function getRequestIds(
	l2update: PalletRolldownMessagesL1Update,
	type:
		| "pendingCancelResolutions"
		| "pendingWithdrawalResolutions"
		| "pendingL2UpdatesToRemove"
		| "pendingDeposits",
) {
	return l2update[type].map((item) => item.requestId.id.toNumber());
}

function getMinMaxRequestId(requestIds: Array<number>, type: "min" | "max") {
	if (type === "min") {
		return Math.min(...requestIds);
	}
	return Math.max(...requestIds);
}

function getMinRequestId(l2Update: PalletRolldownMessagesL1Update) {
	const pendingCancelResolutionsRequestIds = getRequestIds(
		l2Update,
		"pendingCancelResolutions",
	);

	const pendingWithdrawalResolutionsRequestIds = getRequestIds(
		l2Update,
		"pendingWithdrawalResolutions",
	);

	const pendingL2UpdatesToRemoveRequestIds = getRequestIds(
		l2Update,
		"pendingL2UpdatesToRemove",
	);

	const pendingDepositsRequestIds = getRequestIds(l2Update, "pendingDeposits");

	const minId = getMinMaxRequestId(
		[
			Math.min(...pendingCancelResolutionsRequestIds),
			Math.min(...pendingWithdrawalResolutionsRequestIds),
			Math.min(...pendingL2UpdatesToRemoveRequestIds),
			Math.min(...pendingDepositsRequestIds),
		],
		"min",
	);

	return minId === Number.POSITIVE_INFINITY ? null : minId;
}

function getMaxRequestId(l2Update: PalletRolldownMessagesL1Update) {
	const pendingCancelResolutionsRequestIds = getRequestIds(
		l2Update,
		"pendingCancelResolutions",
	);

	const pendingWithdrawalResolutionsRequestIds = getRequestIds(
		l2Update,
		"pendingWithdrawalResolutions",
	);

	const pendingL2UpdatesToRemoveRequestIds = getRequestIds(
		l2Update,
		"pendingL2UpdatesToRemove",
	);

	const pendingDepositsRequestIds = getRequestIds(l2Update, "pendingDeposits");

	const maxId = getMinMaxRequestId(
		[
			Math.max(...pendingCancelResolutionsRequestIds),
			Math.max(...pendingWithdrawalResolutionsRequestIds),
			Math.max(...pendingL2UpdatesToRemoveRequestIds),
			Math.max(...pendingDepositsRequestIds),
		],
		"max",
	);

	return maxId === Number.POSITIVE_INFINITY ? null : maxId;
}

function filterUpdates(
	l2Update: PalletRolldownMessagesL1Update,
	lastRequestId: number,
) {
	const minId = getMinRequestId(l2Update);
	if (minId === null) {
		return l2Update;
	}
	const firstRequestId = Math.max(minId, lastRequestId + 1);

	while (
		l2Update.pendingDeposits.length > 0 &&
		l2Update.pendingDeposits[0].requestId.id.toNumber() < firstRequestId
	) {
		l2Update.pendingDeposits.shift();
	}

	while (
		l2Update.pendingCancelResolutions.length > 0 &&
		l2Update.pendingCancelResolutions[0].requestId.id.toNumber() <
			firstRequestId
	) {
		l2Update.pendingCancelResolutions.shift();
	}

	while (
		l2Update.pendingWithdrawalResolutions.length > 0 &&
		l2Update.pendingWithdrawalResolutions[0].requestId.id.toNumber() <
			firstRequestId
	) {
		l2Update.pendingWithdrawalResolutions.shift();
	}

	while (
		l2Update.pendingL2UpdatesToRemove.length > 0 &&
		l2Update.pendingL2UpdatesToRemove[0].requestId.id.toNumber() <
			firstRequestId
	) {
		l2Update.pendingL2UpdatesToRemove.shift();
	}

	const maxAmountOfUpdates = Number.parseInt(LIMIT);

	if (maxAmountOfUpdates > 0) {
		const lastRequestId = firstRequestId + maxAmountOfUpdates;

		while (
			l2Update.pendingDeposits.length > 0 &&
			l2Update.pendingDeposits[
				l2Update.pendingDeposits.length - 1
			].requestId.id.toNumber() > lastRequestId
		) {
			l2Update.pendingDeposits.pop();
		}

		while (
			l2Update.pendingCancelResolutions.length > 0 &&
			l2Update.pendingCancelResolutions[
				l2Update.pendingCancelResolutions.length - 1
			].requestId.id.toNumber() > lastRequestId
		) {
			l2Update.pendingCancelResolutions.pop();
		}

		while (
			l2Update.pendingWithdrawalResolutions.length > 0 &&
			l2Update.pendingWithdrawalResolutions[
				l2Update.pendingWithdrawalResolutions.length - 1
			].requestId.id.toNumber() > lastRequestId
		) {
			l2Update.pendingWithdrawalResolutions.pop();
		}

		while (
			l2Update.pendingL2UpdatesToRemove.length > 0 &&
			l2Update.pendingL2UpdatesToRemove[
				l2Update.pendingL2UpdatesToRemove.length - 1
			].requestId.id.toNumber() > lastRequestId
		) {
			l2Update.pendingL2UpdatesToRemove.pop();
		}

		return l2Update;
	}
	return l2Update;
}

function countRequests(l2Update: PalletRolldownMessagesL1Update) {
	return (
		l2Update.pendingCancelResolutions.length +
		l2Update.pendingWithdrawalResolutions.length +
		l2Update.pendingL2UpdatesToRemove.length +
		l2Update.pendingDeposits.length
	);
}

async function getLastRequestId(api: ApiPromise) {
	return Number.parseInt(
		(await api.query.rolldown.lastProcessedRequestOnL2("Ethereum")).toString(),
	);
}

function print(data: any) {
	console.log(util.inspect(data, { depth: null }));
}

async function getUpdateForL2(publicClient: PublicClient) {
	const lastProcessed = (await publicClient.readContract({
		address: MANGATA_CONTRACT_ADDRESS,
		abi: ABI,
		functionName: "lastProcessedUpdate_origin_l1",
	})) as bigint;

	const counter = (await publicClient.readContract({
		address: MANGATA_CONTRACT_ADDRESS,
		abi: ABI,
		functionName: "counter",
	})) as bigint;

	const rangeStart = lastProcessed + 1n;
	let rangeEnd = rangeStart + BigInt(LIMIT);
	if (rangeEnd > counter - 1n) {
		rangeEnd = counter - 1n;
	}

	return await publicClient.readContract({
		address: MANGATA_CONTRACT_ADDRESS,
		abi: ABI,
		functionName: "getPendingRequests",
		args: [rangeStart, rangeEnd],
	});
}

export {
	print,
	sleep,
	getApi,
	getEvents,
	isSuccess,
	getCollator,
	countRequests,
	filterUpdates,
	getEncodedData,
	getMaxRequestId,
	getLastRequestId,
	getNativeL1Update,
	processDataForL2Update,
	initReadContractWithRetry,
	processPendingRequestsEvents,
	getSelectedSequencerWithRights,
};