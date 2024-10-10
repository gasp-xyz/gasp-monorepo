import util from "node:util";
import { type ApiPromise, Keyring } from "@polkadot/api";
import { Mangata, type MangataGenericEvent, signTx } from "gasp-sdk";

function sleep(timeInMilliseconds: number): Promise<void> {
	return new Promise((resolve) => setTimeout(resolve, timeInMilliseconds));
}

async function getApi(nodeUrl: string): Promise<ApiPromise> {
	const api = await Mangata.instance([nodeUrl]).api();
	await api.isReady;
	return api;
}

function isSuccess(events: MangataGenericEvent[]) {
	return events.some(
		(event) =>
			event.section === "system" && event.method === "ExtrinsicSuccess",
	);
}

function isEqual(first: Uint8Array, second: Uint8Array): boolean {
  if(first.length !== second.length) {
    return false;
  }
  return first.every((value, index) => value === second[index]);
}

function print(data: any) {
	console.log(util.inspect(data, { depth: null }));
}

export { print, sleep, getApi, isSuccess, isEqual };
