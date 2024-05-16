import rolldownAbi from "../RollDown.json" assert { type: "json" };
import { configuration } from "../config/index.js";

export const MANGATA_CONTRACT_ADDRESS =
	configuration.MANGATA_CONTRACT_ADDRESS as `0x${string}`;

export const BLOCK_NUMBER_DELAY = configuration.BLOCK_NUMBER_DELAY;

export const ETH_CHAIN_URL = configuration.ETH_CHAIN_URL;

export const MANGATA_NODE_URL = configuration.MANGATA_NODE_URL;

export const MNEMONIC = configuration.MNEMONIC;

export const ROLLDOWN_EVENT_SECTION = "rolldown";
export const L1_READ_STORED_EVENT_METHOD = "L1ReadStored";
export const LIMIT = configuration.LIMIT;
export const L1_CHAIN = configuration.L1_CHAIN;

export const ABI = rolldownAbi.abi;
