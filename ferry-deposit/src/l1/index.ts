
import {
  Deposit
} from "../common/deposit.js";

interface L1Interface {
	isRolldownDeployed(): Promise<boolean>;
	getLatestRequestId(): Promise<bigint | null>;
	getDeposits(rangeStart: bigint, rangeEnd: bigint): Promise<Deposit[]>;
	getDepostiHash(requestId: bigint): Promise<Uint8Array>;
}

export { L1Interface };
