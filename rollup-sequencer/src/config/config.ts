import { z } from "zod";

export const appConfigSchema = z.object({
	MANGATA_CONTRACT_ADDRESS: z.string(),
	BLOCK_NUMBER_DELAY: z.string().default("5"),
	ETH_CHAIN_URL: z.string(),
	MANGATA_NODE_URL: z.string(),
	MNEMONIC: z.string(),
	LIMIT: z.string().default("10"),
	L1_CHAIN: z.string().default("Ethereum"),
});

export type AppConfig = z.infer<typeof appConfigSchema>;

export function defineConfig(config: AppConfig) {
	return appConfigSchema.parse(config);
}

export function createConfig() {
	return defineConfig({
		MANGATA_CONTRACT_ADDRESS: process.env.MANGATA_CONTRACT_ADDRESS!,
		BLOCK_NUMBER_DELAY: process.env.BLOCK_NUMBER_DELAY!,
		ETH_CHAIN_URL: process.env.ETH_CHAIN_URL!,
		MANGATA_NODE_URL: process.env.MANGATA_NODE_URL!,
		MNEMONIC: process.env.MNEMONIC!,
		LIMIT: process.env.LIMIT!,
		L1_CHAIN: process.env.L1_CHAIN!,
	});
}
