import rolldownAbi from "./Rolldown.json" assert { type: "json" };
import { hexToU8a } from "@polkadot/util";
import JSONbig from "json-bigint";
import { z } from "zod";
import { anvil, holesky, arbitrumSepolia, hardhat } from "viem/chains";

const tokensToTrackSchema = z
	.tuple([
		z.string().transform((x) => hexToU8a(x, 160)),
		z.bigint(),
		z.bigint(),
	])
	.array();

const SUPPORTED_CHAINS = new Map<string, string>([
	["anvil-arbitrum", "arbitrum"],
	["anvil-ethereum", "ethereum"],
	["holesky", "ethereum"],
	["ethereum", "ethereum"],
	["base-sepolia", "base"],
	["base", "base"],
	["arbitrum", "arbitrum"],
	["arbitrum-sepolia", "arbitrum"],
	["reth-arbitrum", "arbitrum"],
	["reth-ethereum", "ethereum"],
	["reth-base", "base"],
]);

const cliConfigSchemat = z.object({
	MANGATA_CONTRACT_ADDRESS: z.string(),
	ETH_CHAIN_URL: z.string(),
	STASH_URL: z.string(),
	MANGATA_NODE_URL: z.string(),
	PRIVATE_KEY: z.string(),
	L1_CHAIN: z.string().refine((chain) => SUPPORTED_CHAINS.has(chain), {
		message: `env::L1_CHAIN needs to be one of ${Array.from(
			SUPPORTED_CHAINS.keys(),
		)}`,
	}),
	TOKENS_TO_TRACK: z
		.string()
		.transform((elem) =>
			tokensToTrackSchema.parse(
				JSONbig({ alwaysParseAsBig: true, useNativeBigInt: true }).parse(elem),
			),
		),
	TX_COST: z.bigint(),
	LOOK_BACK_HOURS: z.number().default(24),
	LOG: z.string().default("info"),
	DELAY: z.bigint().default(0n),
});

function createCliConfig() {
	return cliConfigSchemat.parse({
		MANGATA_CONTRACT_ADDRESS: process.env.MANGATA_CONTRACT_ADDRESS!,
		ETH_CHAIN_URL: process.env.ETH_CHAIN_URL!,
		MANGATA_NODE_URL: process.env.MANGATA_NODE_URL!,
		STASH_URL: process.env.STASH_URL!,
		PRIVATE_KEY: process.env.PRIVATE_KEY!,
		L1_CHAIN: process.env.L1_CHAIN!,
		TOKENS_TO_TRACK: process.env.TOKENS_TO_TRACK!,
		TX_COST: BigInt(process.env.TX_COST!),
		LOOK_BACK_HOURS: process.env.LOOK_BACK_HOURS
			? Number(process.env.LOOK_BACK_HOURS)
			: undefined,
		LOG: process.env.LOG,
		DELAY: process.env.DELAY ? BigInt(process.env.DELAY) : undefined,
	});
}

const configuration = createCliConfig();

export const MANGATA_CONTRACT_ADDRESS =
	configuration.MANGATA_CONTRACT_ADDRESS as `0x${string}`;
export const ETH_CHAIN_URL = configuration.ETH_CHAIN_URL;
export const STASH_URL = configuration.STASH_URL;
export const MANGATA_NODE_URL = configuration.MANGATA_NODE_URL;
export const PRIVATE_KEY = configuration.PRIVATE_KEY;
export const L1_CHAIN = configuration.L1_CHAIN;
export const L2_CHAIN = SUPPORTED_CHAINS.get(L1_CHAIN)!;
export const TOKENS_TO_TRACK = configuration.TOKENS_TO_TRACK;
export const TX_COST = configuration.TX_COST;
export const LOOK_BACK_HOURS = configuration.LOOK_BACK_HOURS;
export const LOG = configuration.LOG;
export const ABI = rolldownAbi.abi;
export const DELAY = configuration.DELAY;
