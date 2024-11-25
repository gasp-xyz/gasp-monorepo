import { Withdrawal } from "../Withdrawal.js";
import { Cancel } from "../Cancel.js";

export interface L1Interface {
  isRolldownDeployed(): Promise<boolean>;
  getLatestRequestId(delay: bigint): Promise<bigint | null>;
  getBalance(account: Uint8Array, tokenAddress: Uint8Array): Promise<bigint | null>;
  getNativeTokenAddress(): Promise<Uint8Array>;

  isClosed(hash: Uint8Array): Promise<boolean>;
  isFerried(hash: Uint8Array): Promise<boolean>;
  getFerry(hash: Uint8Array): Promise<Uint8Array | null>;

  ferry(withdrawal: Withdrawal, privateKey: Uint8Array): Promise<boolean>;
  closeWithdrawal(withdrawal: Withdrawal, merkleRoot: Uint8Array, proof: Uint8Array[], privateKey: Uint8Array): Promise<boolean>;
  closeCancel(cancel: Cancel, merkleRoot: Uint8Array, proof: Uint8Array[], privateKey: Uint8Array): Promise<boolean>;
  getMerkleRange(requestId: bigint): Promise<{ root: Uint8Array, range: [bigint, bigint] }>; 
}
