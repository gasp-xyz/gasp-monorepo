export interface L2Update {
    cancels: Cancel[];
    withdrawals: Withdrawal[];
    results: RequestResult[];
}

export interface Cancel {
    requestId: RequestId;
    range: Range;
    hash: string;
}

export interface RequestId {
    origin: Origin;
    id: number;
}

export enum Origin {
    L1,
    L2
}

export interface Range {
    start: number;
    end: number;
}

export interface Withdrawal {
    requestId: RequestId;
    withdrawalRecipient: string;
    tokenAddress: string;
    amount: number;
}

export interface RequestResult {
    requestId: RequestId;
    originRequestId: number;
    updateType: UpdateType;
    status: boolean;
}

export enum UpdateType {
    DEPOSIT,
    WITHDRAWAL,
    WITHDRAWAL_RESOLUTION,
    INDEX_UPDATE,
    CANCEL,
    CANCEL_RESOLUTION
}
