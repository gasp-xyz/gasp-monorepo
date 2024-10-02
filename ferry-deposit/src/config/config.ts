import { z } from "zod";

export const appConfigSchema = z.object({
	MANGATA_CONTRACT_ADDRESS: z.string(),
	ETH_CHAIN_URL: z.string(),
	MANGATA_NODE_URL: z.string(),
	MNEMONIC: z.string(),
	L1_CHAIN: z.string(),
	MIN_PROFIT: z.bigint(),
	TX_COST: z.bigint(),
	BLOCK_DELAY: z.bigint(),
});

export type AppConfig = z.infer<typeof appConfigSchema>;

export function defineConfig(config: AppConfig) {
	return appConfigSchema.parse(config);
}

export function createConfig() {
	return defineConfig({
		MANGATA_CONTRACT_ADDRESS: process.env.MANGATA_CONTRACT_ADDRESS!,
		ETH_CHAIN_URL: process.env.ETH_CHAIN_URL!,
		MANGATA_NODE_URL: process.env.MANGATA_NODE_URL!,
		MNEMONIC: process.env.MNEMONIC!,
		L1_CHAIN: process.env.L1_CHAIN!,
		MIN_PROFIT: BigInt(process.env.MIN_PROFIT!),
		TX_COST: BigInt(process.env.TX_COST!),
		BLOCK_DELAY: BigInt(process.env.BLOCK_DELAY!),
	});
}
