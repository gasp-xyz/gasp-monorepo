import { u8aToHex } from "@polkadot/util";

interface Deposit {
	readonly requestId: bigint;
	readonly depositRecipient: Uint8Array;
	readonly tokenAddress: Uint8Array;
	readonly amount: bigint;
	readonly timeStamp: bigint;
	readonly ferryTip: bigint;
}

function toString(withdrawal: Deposit): string {
	return `rid:${withdrawal.requestId} recipient:${u8aToHex(withdrawal.depositRecipient)} token:${u8aToHex(withdrawal.tokenAddress)} amount:${withdrawal.amount} tip:${withdrawal.ferryTip}`;
}
export { type Deposit, toString };
