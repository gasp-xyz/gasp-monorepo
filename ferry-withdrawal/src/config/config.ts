import { z } from "zod";

export const appConfigSchema = z.object({
	MANGATA_CONTRACT_ADDRESS: z.string(),
	ETH_CHAIN_URL: z.string(),
	MANGATA_NODE_URL: z.string(),
	PRIVATE_KEY: z.string(),
	L1_CHAIN: z.string(),
	TOKENS_TO_TRACK: z.string(),
	TX_COST: z.bigint(),
	LOOK_BACK_HOURS: z.number(),
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
		PRIVATE_KEY: process.env.PRIVATE_KEY!,
		L1_CHAIN: process.env.L1_CHAIN!,
		TOKENS_TO_TRACK: process.env.TOKENS_TO_TRACK!,
		TX_COST: BigInt(process.env.TX_COST!),
		LOOK_BACK_HOURS: Number(process.env.LOOK_BACK_HOURS!),
	});
}
