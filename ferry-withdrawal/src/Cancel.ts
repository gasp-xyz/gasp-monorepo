
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
  // TODO: fix
  return "sdfsdf"
	// return `rid:${cancel.requestId} range: ${cancel.range[0]} .. ${cancel.range[1]}`;
}

export { type Cancel, toString, isCancel };
