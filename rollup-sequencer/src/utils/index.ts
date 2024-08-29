import util from "node:util";
import {
	Mangata,
	type MangataGenericEvent,
	signTx,
} from "gasp-sdk";
import { type ApiPromise, Keyring } from "@polkadot/api";
import type { ApiDecoration } from "@polkadot/api/types";
import type { KeyringPair } from "@polkadot/keyring/types";
import { Option } from "@polkadot/types-codec";
import { Vec } from "@polkadot/types-codec";
import {
	FrameSystemEventRecord,
	PalletRolldownMessagesL1Update,
	PalletRolldownMessagesChain,
} from "@polkadot/types/lookup";
import { hexToU8a, u8aToHex } from "@polkadot/util";
import type { KeypairType } from "@polkadot/util-crypto/types";
import { type PublicClient, encodeAbiParameters, keccak256, UnauthorizedProviderError } from "viem";
import {
	ABI,
	BLOCK_NUMBER_DELAY,
	L1_CHAIN,
	L1_READ_STORED_EVENT_METHOD,
	LIMIT,
	MANGATA_CONTRACT_ADDRESS,
	ROLLDOWN_EVENT_SECTION,
} from "../common/constants.js";


function getL1ChainType(
	api: ApiPromise,
): PalletRolldownMessagesChain {
  return api.createType('PalletRolldownMessagesChain', L1_CHAIN)
}

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
  return await api.rpc.rolldown.get_native_sequencer_update(encodedData.substring(2));
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

async function initReadContractWithRetry(
	publicClient: PublicClient,
	api: ApiPromise,
) {

	while (true) {
    const latestBlockNumber = await publicClient.getBlockNumber();
    const delayedBlockNumber = latestBlockNumber - BigInt(BLOCK_NUMBER_DELAY);
    const code = await publicClient.getCode({address: MANGATA_CONTRACT_ADDRESS, blockNumber: delayedBlockNumber});
    console.log(`code ${code}`)
    if (code !== undefined){
      break;
    }else{
      print(`contract ${MANGATA_CONTRACT_ADDRESS} does not exists yet at block(${delayedBlockNumber}) with BLOCK_NUMBER_DELAY set to ${BLOCK_NUMBER_DELAY}- retrying in 5 seconds`);
      await sleep(5000);
    }
  }


}

async function processDataForL2Update(
	api: ApiPromise,
	publicClient: PublicClient,
) {
	const latestBlockNumber = await publicClient.getBlockNumber();
	print(`Latest Block Number: ${latestBlockNumber.toString()}`);

  if (latestBlockNumber < BigInt(BLOCK_NUMBER_DELAY)) {
    print("not enought block - returning none");
    return null
  }
 
  const delayedBlockNumber = latestBlockNumber - BigInt(BLOCK_NUMBER_DELAY);
	const data = await getUpdateForL2(publicClient, api, delayedBlockNumber);
	print(`ETH native data : ${util.inspect(data, { depth: null })}`);

	const encodedData = getEncodedData("getUpdateForL2", data);
	return (await getNativeL1Update(api, encodedData)).unwrap();
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
				const chain = (data as unknown as string[])[0];
				const requestId = (data as unknown as string[])[2];
				const { start, end } = (data as any)[3] as unknown as {
					start: string;
					end: string;
				};
        if (chain.toString() !== L1_CHAIN){
          console.log(`ignoring event ${data.toString()} for differnet chain`)
          return
        }

        const latestBlockNumber = await publicClient.getBlockNumber();

				const contractData = await publicClient.readContract({
					address: MANGATA_CONTRACT_ADDRESS,
					abi: ABI,
					functionName: "getPendingRequests",
					args: [start, end],
          // blockNumber: 
				});

				const encodedData = getEncodedData("getPendingRequests", contractData);

				const verified = await api.rpc.rolldown.verify_sequencer_update(
          L1_CHAIN,
					keccak256(encodedData),
					requestId.toString(),
				);

				if (!verified.toPrimitive()) {
					await signTx(
						api,
						api.tx.rolldown.cancelRequestsFromL1(L1_CHAIN as any, requestId.toString()),
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
	const selectedSequencerMap =
    await apiAt.query.sequencerStaking.selectedSequencer();
  const selectedMap = JSON.parse(selectedSequencerMap.toString());
  const selectedSequencerRaw = selectedMap[L1_CHAIN];

  var selectedSequencer = null;
  var isSequencerSelected = false;
  var hasReadRights = false;
  var hasCancelRights = false;

  if (selectedSequencerRaw !== undefined) {
	selectedSequencer = selectedSequencerRaw.toLowerCase();
	isSequencerSelected = selectedSequencer === collatorAddress.toLowerCase();
	const sequencerRights = await apiAt.query.rolldown.sequencersRights(L1_CHAIN);
	let rights = JSON.parse(sequencerRights.toString())[collatorAddress.toLowerCase()]
	hasReadRights = rights.readRights > 0;
	hasCancelRights = rights.cancelRights > 0;
  }

  return {
    isSequencerSelected,
    selectedSequencer,
    hasReadRights,
    hasCancelRights
  };

}

function isSuccess(events: MangataGenericEvent[]) {
	return events.some(
		(event) =>
			event.section === "system" && event.method === "ExtrinsicSuccess",
	);
}

function maxBigInt(...args: bigint[]) {
    return args.reduce((max, current) => current > max ? current : max);
}


function getMaxRequestId(l2Update: PalletRolldownMessagesL1Update) {
	const pendingCancelResolutionsRequestIds = l2Update.pendingCancelResolutions.map((r) => BigInt(r.requestId.id.toString()));
	const pendingDepositsRequestIds = l2Update.pendingDeposits.map((r) => BigInt(r.requestId.id.toString()));

  return maxBigInt( 0n , ...pendingCancelResolutionsRequestIds, ...pendingDepositsRequestIds);
}

function countRequests(l2Update: PalletRolldownMessagesL1Update | null) {
  if (l2Update === null){
    return 0
  }

	return(
		l2Update.pendingDeposits.length +
		l2Update.pendingCancelResolutions.length
	);
}

async function getLastRequestId(api: ApiPromise) {
	return BigInt(
		(await api.query.rolldown.lastProcessedRequestOnL2(L1_CHAIN)).toString(),
	);
}

function print(data: any) {
	console.log(util.inspect(data, { depth: null }));
}

async function getUpdateForL2(publicClient: PublicClient, api: ApiPromise, blockNumber: bigint) {
	const lastProcessed =
		await api.query.rolldown.lastProcessedRequestOnL2(L1_CHAIN);
  console.log(`BLOCK_NUMBER ${blockNumber}`)
	const counter = (await publicClient.readContract({
		address: MANGATA_CONTRACT_ADDRESS,
		abi: ABI,
		functionName: "counter",
    blockNumber
	})) as bigint;

	const rangeStart = BigInt(lastProcessed.toString()) + 1n;

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
	getEncodedData,
	getMaxRequestId,
	getLastRequestId,
	getNativeL1Update,
	processDataForL2Update,
	initReadContractWithRetry,
	processPendingRequestsEvents,
	getSelectedSequencerWithRights,
};
