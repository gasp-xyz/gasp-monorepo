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
  return `\trequestId: ${withdrawal.requestId}` +
         `\n\trecipient: ${u8aToHex(withdrawal.withdrawalRecipient)}` +
          `\n\ttoken: ${u8aToHex(withdrawal.tokenAddress)}` +
          `\n\tamount: ${withdrawal.amount}` +
          `\n\tferryTip: ${withdrawal.ferryTip}` +
          `\n\thash: ${u8aToHex(withdrawal.hash)}`;
}


export { type Withdrawal , toString};
