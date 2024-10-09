interface Withdrawal {
	readonly requestId: bigint;
	readonly withdrawalRecipient: Uint8Array;
	readonly tokenAddress: Uint8Array;
	readonly amount: bigint;
	readonly ferryTip: bigint;
	readonly hash: Uint8Array;
}

export type { Withdrawal };
