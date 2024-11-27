import { Withdrawal } from "../Withdrawal.js";
import { Cancel } from "../Cancel.js";

export interface L2Interface {
	getLatestRequestId(): Promise<bigint | null>;
	getLatestRequestIdInPast(delay: number): Promise<bigint | null>;
	getWithdrawals(startRange: bigint, endRange: bigint): Promise<Withdrawal[]>;
	getRequests(
		startRange: bigint,
		endRange: bigint,
	): Promise<(Withdrawal | Cancel)[]>;
	getNativeTokenAddress(): Promise<Uint8Array>;
	getMerkleProof(
		startRange: bigint,
		endRange: bigint,
		txId: bigint,
	): Promise<Uint8Array[]>;
}
