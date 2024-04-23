import { z } from "zod";

export const appConfigSchema = z.object({
    MANGATA_CONTRACT_ADDRESS: z.string(),
    BLOCK_NUMBER_DELAY: z.string().default("5"),
    ETH_CHAIN_URL: z.string(),
    MANGATA_NODE_URL: z.string(),
    MNEMONIC: z.string(),
});

export type AppConfig = z.infer<typeof appConfigSchema>;