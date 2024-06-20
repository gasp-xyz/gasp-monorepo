import { z } from "zod";

declare global {
    namespace NodeJS {
        interface ProcessEnv extends z.infer<typeof appConfigSchema> {}
    }
}

const appConfigSchema = z.object({
    EIGEN_CONTRACT_ADDRESS: z.string(),
    MANGATA_CONTRACT_ADDRESS: z.string(),
    FINALIZATION_SOURCE: z.string(),
    LIMIT: z.string().default("0"),
    CHAIN: z.string().default("anvil"),
    MANGATA_NODE_URL: z.string(),
    WALLET_PRIVATE_KEY: z.string(),
    ETH_CHAIN_URL: z.string(),
    VERBOSE: z.string().default("false"),
    L1_CHAIN: z.string().default("Ethereum"),
});

type AppConfig = z.infer<typeof appConfigSchema>;

function defineConfig(config: AppConfig) {
    return appConfigSchema.parse(config);
}

function createConfig() {
    return defineConfig({
        EIGEN_CONTRACT_ADDRESS: process.env.EIGEN_CONTRACT_ADDRESS,
        MANGATA_CONTRACT_ADDRESS: process.env.MANGATA_CONTRACT_ADDRESS,
        FINALIZATION_SOURCE: process.env.FINALIZATION_SOURCE,
        LIMIT: process.env.LIMIT,
        CHAIN: process.env.CHAIN,
        MANGATA_NODE_URL: process.env.MANGATA_NODE_URL,
        WALLET_PRIVATE_KEY: process.env.WALLET_PRIVATE_KEY,
        ETH_CHAIN_URL: process.env.ETH_CHAIN_URL,
        VERBOSE: process.env.VERBOSE,
        L1_CHAIN: process.env.L1_CHAIN!,
    });
}

function getConfig() {
    // Here we can distinguish between different
    // configs like - local, test, staging, production
    return createConfig();
}

export const configuration = getConfig();
