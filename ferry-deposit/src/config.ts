import rolldownAbi from "./Rolldown.json" assert { type: "json" };
import { z } from "zod";
import { hexToU8a } from "@polkadot/util";
import JSONbig from "json-bigint";

const tokensToTrackSchema = z
	.tuple([
		z.string().transform((x) => hexToU8a(x, 160)),
		z.bigint(),
		z.bigint(),
	])
	.array();

export const appConfigSchema = z.object({
	MANGATA_CONTRACT_ADDRESS: z.string(),
	ETH_CHAIN_URL: z.string(),
	MANGATA_NODE_URL: z.string(),
	MNEMONIC: z.string(),
	L1_CHAIN: z.string(),
	TX_COST: z.bigint(),
	BLOCK_DELAY: z.bigint(),
	TOKENS_TO_TRACK: z.string().transform(elem => tokensToTrackSchema.parse(JSONbig({ alwaysParseAsBig: true, useNativeBigInt: true }).parse(elem))),
  LOG: z.string().default("info"),
});

const configuration = appConfigSchema.parse({
		MANGATA_CONTRACT_ADDRESS: process.env.MANGATA_CONTRACT_ADDRESS!,
		ETH_CHAIN_URL: process.env.ETH_CHAIN_URL!,
		MANGATA_NODE_URL: process.env.MANGATA_NODE_URL!,
		MNEMONIC: process.env.MNEMONIC!,
		L1_CHAIN: process.env.L1_CHAIN!,
		TX_COST: BigInt(process.env.TX_COST!),
		BLOCK_DELAY: BigInt(process.env.BLOCK_DELAY!),
    TOKENS_TO_TRACK: process.env.TOKENS_TO_TRACK!,
    LOG: process.env.LOG,
	});


export const MANGATA_CONTRACT_ADDRESS = configuration.MANGATA_CONTRACT_ADDRESS as `0x${string}`;
export const ETH_CHAIN_URL = configuration.ETH_CHAIN_URL;
export const MANGATA_NODE_URL = configuration.MANGATA_NODE_URL;
export const MNEMONIC = configuration.MNEMONIC;
export const L1_CHAIN = configuration.L1_CHAIN;
export const TX_COST = configuration.TX_COST;
export const BLOCK_DELAY = configuration.BLOCK_DELAY;
export const TOKENS_TO_TRACK = configuration.TOKENS_TO_TRACK;
export const LOG = configuration.LOG;

export const ABI = rolldownAbi.abi;
