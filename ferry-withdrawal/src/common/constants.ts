import rolldownAbi from "../Rolldown.json" assert { type: "json" };
import { configuration } from "../config/index.js";

export const MANGATA_CONTRACT_ADDRESS =
	configuration.MANGATA_CONTRACT_ADDRESS as `0x${string}`;

export const ETH_CHAIN_URL = configuration.ETH_CHAIN_URL;
export const MANGATA_NODE_URL = configuration.MANGATA_NODE_URL;
export const PRIVATE_KEY = configuration.PRIVATE_KEY;
export const L1_CHAIN = configuration.L1_CHAIN;
export const TOKENS_TO_TRACK = configuration.TOKENS_TO_TRACK;
export const TX_COST = configuration.TX_COST;
export const LOOK_BACK_HOURS = configuration.LOOK_BACK_HOURS;
export const LOG = configuration.LOG;
export const ABI = rolldownAbi.abi;
