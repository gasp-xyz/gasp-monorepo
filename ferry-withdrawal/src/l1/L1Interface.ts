import { Withdrawal } from "../Withdrawal.js";

export interface L1Interface {
  isRolldownDeployed(): Promise<boolean>;
  getLatestRequestId(): Promise<bigint | null>;
  getBalance(account: Uint8Array, tokenAddress: Uint8Array): Promise<bigint | null>;
  getNativeTokenAddress(): Promise<Uint8Array>;

  isClosed(hash: Uint8Array): Promise<boolean>;
  isFerried(hash: Uint8Array): Promise<boolean>;
  getFerry(hash: Uint8Array): Promise<Uint8Array | null>;

  ferry(withdrawal: Withdrawal, privateKey: Uint8Array): Promise<boolean>;
  close(withdrawal: Withdrawal, privateKey: Uint8Array, proof: Uint8Array[]): Promise<boolean>;
  getMerkleRange(requestId: bigint): Promise<[bigint, bigint] | null>; }
