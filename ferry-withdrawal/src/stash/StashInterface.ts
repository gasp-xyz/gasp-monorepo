export interface StashInterface {
	shouldBeClosed(txHash: Uint8Array): Promise<boolean>;
}
