import { Withdrawal } from "../Withdrawal.js";

export interface L2Interface {
	getLatestRequestId(): Promise<bigint|null>;
	getLatestRequestIdInPast(delay: number): Promise<bigint|null>;
  getWithdrawals(startRange: bigint, endRange: bigint): Promise<Withdrawal[]>;
	getNativeTokenAddress(): Promise<Uint8Array>;
	getMerkleProof(startRange: bigint, endRange: bigint, txId: bigint): Promise<Uint8Array[]>;
}
