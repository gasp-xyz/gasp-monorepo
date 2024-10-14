export interface L2Interface {
	getBalances(address: Uint8Array): Promise<Map<Uint8Array, bigint>>;
	getLastProcessedRequestId(): Promise<bigint>;
	getLastProcessedRequestId(): Promise<bigint>;
	isExecuted(depositId: bigint): Promise<boolean>;
	isFerried(depositId: Uint8Array): Promise<boolean>;
	getNativeTokenAddress(): Promise<Uint8Array>;
	valutateToken(tokenAddress: Uint8Array, amount: bigint): Promise<bigint>;
}
