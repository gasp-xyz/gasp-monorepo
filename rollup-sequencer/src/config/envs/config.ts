import { defineConfig } from "../defineConfig";

export function createConfig() {
    return defineConfig({
        MANGATA_CONTRACT_ADDRESS: process.env.MANGATA_CONTRACT_ADDRESS!,
        BLOCK_NUMBER_DELAY: process.env.BLOCK_NUMBER_DELAY!,
        ETH_CHAIN_URL: process.env.ETH_CHAIN_URL!,
        MANGATA_NODE_URL: process.env.MANGATA_NODE_URL!,
        MNEMONIC: process.env.MNEMONIC!
    });
}