
import { u8aToHex } from "@polkadot/util";

interface Cancel {
	readonly requestId: bigint;
	readonly startRange: bigint;
	readonly endRange: bigint;
	readonly properHash: Uint8Array;
	readonly hash: Uint8Array;
}

function isCancel(obj:any): obj is Cancel {
  return "properHash" in  obj;
}

function toString(cancel: Cancel): string {
	return `rid:${cancel.requestId} range: ${cancel.startRange} .. ${cancel.endRange} properUpdateHash: ${u8aToHex(cancel.properHash)} tx hash: ${u8aToHex(cancel.hash)}`;
}

export { type Cancel, toString, isCancel };
