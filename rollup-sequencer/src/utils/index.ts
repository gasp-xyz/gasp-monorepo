import util from "node:util";
import { Mangata, signTx } from "@mangata-finance/sdk";
import { type ApiPromise, Keyring } from "@polkadot/api";
import type { ApiDecoration } from "@polkadot/api/types";
import type { KeyringPair } from "@polkadot/keyring/types";
import type { Option } from "@polkadot/types-codec";
import type {
	FrameSystemEventRecord,
	PalletRolldownMessagesL1Update,
} from "@polkadot/types/lookup";
import { hexToU8a } from "@polkadot/util";
import type { KeypairType } from "@polkadot/util-crypto/types";
import { type PublicClient, encodeAbiParameters, keccak256 } from "viem";
import {
	ABI,
	BLOCK_NUMBER_DELAY,
	L1_READ_STORED_EVENT_METHOD,
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
			return await publicClient.readContract({
				address: MANGATA_CONTRACT_ADDRESS,
				abi: ABI,
				functionName: "getUpdateForL2",
			});
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

	const data = await publicClient.readContract({
		address: MANGATA_CONTRACT_ADDRESS,
		abi: ABI,
		functionName: "getUpdateForL2",
		blockNumber: delayedBlockNumber,
	});

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

function print(data: any) {
	console.log(util.inspect(data, { depth: null }));
}

export {
	print,
	sleep,
	getApi,
	getEvents,
	getCollator,
	getEncodedData,
	getNativeL1Update,
	processDataForL2Update,
	initReadContractWithRetry,
	processPendingRequestsEvents,
};
