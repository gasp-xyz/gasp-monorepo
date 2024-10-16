interface Deposit {
	readonly requestId: bigint;
	readonly depositRecipient: Uint8Array;
	readonly tokenAddress: Uint8Array;
	readonly amount: bigint;
	readonly timeStamp: bigint;
	readonly ferryTip: bigint;
}

export type { Deposit };
