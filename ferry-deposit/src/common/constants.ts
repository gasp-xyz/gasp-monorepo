import rolldownAbi from "../Rolldown.json" assert { type: "json" };
import { configuration } from "../config/index.js";

export const MANGATA_CONTRACT_ADDRESS =
	configuration.MANGATA_CONTRACT_ADDRESS as `0x${string}`;

export const ETH_CHAIN_URL = configuration.ETH_CHAIN_URL;
export const MANGATA_NODE_URL = configuration.MANGATA_NODE_URL;
export const MNEMONIC = configuration.MNEMONIC;
export const L1_CHAIN = configuration.L1_CHAIN;
export const MIN_PROFIT = configuration.MIN_PROFIT;
export const TX_COST = configuration.TX_COST;
export const BLOCK_DELAY = configuration.BLOCK_DELAY;

export const ABI = rolldownAbi.abi;
