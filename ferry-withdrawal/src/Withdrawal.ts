import { u8aToHex } from "@polkadot/util";

interface Withdrawal {
	readonly requestId: bigint;
	readonly withdrawalRecipient: Uint8Array;
	readonly tokenAddress: Uint8Array;
	readonly amount: bigint;
	readonly ferryTip: bigint;
	readonly hash: Uint8Array;
}

function toString(withdrawal: Withdrawal): string {
	return `rid:${withdrawal.requestId} recipient:${u8aToHex(withdrawal.withdrawalRecipient)} token:${u8aToHex(withdrawal.tokenAddress)} amount:${withdrawal.amount} tip:${withdrawal.ferryTip} hash:${u8aToHex(withdrawal.hash)}`;
}

function isWithdrawal(obj:any): obj is Withdrawal {
  return "ferryTip" in obj;
}

export { type Withdrawal, toString, isWithdrawal };
