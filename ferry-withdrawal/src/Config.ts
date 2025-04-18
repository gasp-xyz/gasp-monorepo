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
	// anvil
	["anvil-ethereum", "ethereum"],
	["anvil-arbitrum", "arbitrum"],
	["anvil-base", "arbitrum"],
	["anvil-monad", "monad"],
	["anvil-megaeth", "megaeth"],
	["anvil-monad", "monad"],
	["anvil-sonic", "sonic"],
	// mainnet
	["ethereum", "ethereum"],
	["arbitrum", "arbitrum"],
	["base", "base"],
	["sonic", "sonic"],
	// testnet
	["holesky", "ethereum"],
	["arbitrum-sepolia", "arbitrum"],
	["base-sepolia", "base"],
	["monad", "monad"],
	["megaeth", "megaeth"],
	["sonic-testnet", "sonic"],
	// reth
	["reth-arbitrum", "arbitrum"],
	["reth-ethereum", "ethereum"],
	["reth-base", "base"],
	["reth-monad", "monad"],
	["reth-megaeth", "megaeth"],
	["reth-sonic", "sonic"],
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
	BATCH_SIZE: z.bigint().default(500n),
	REPLICA_COUNT: z.bigint().default(0n),
	REPLICA_ID: z.bigint().default(0n),
	MIN_REQUEST_ID: z.bigint().default(0n),
	SKIP_STASH: z.boolean().default(false),
	METRICS_PORT: z.number().default(8080),
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
		BATCH_SIZE: process.env.BATCH_SIZE
			? BigInt(process.env.BATCH_SIZE)
			: undefined,
		REPLICA_COUNT: process.env.REPLICA_COUNT
			? BigInt(process.env.REPLICA_COUNT)
			: undefined,
		REPLICA_ID: process.env.REPLICA_ID
			? BigInt(process.env.REPLICA_ID)
			: undefined,
		MIN_REQUEST_ID: process.env.MIN_REQUEST_ID
			? BigInt(process.env.MIN_REQUEST_ID)
			: undefined,
		SKIP_STASH: process.env.SKIP_STASH
			? Boolean(process.env.SKIP_STASH)
			: undefined,
		METRICS_PORT: process.env.METRICS_PORT
			? parseInt(process.env.METRICS_PORT)
			: undefined,
	});
}

const configuration = createCliConfig();

export const METRICS_PORT = configuration.METRICS_PORT;
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
export const BATCH_SIZE = configuration.BATCH_SIZE;
export const REPLICA_COUNT = configuration.REPLICA_COUNT;
export const REPLICA_ID = configuration.REPLICA_ID;
export const MIN_REQUEST_ID = configuration.MIN_REQUEST_ID;
export const SKIP_STASH = configuration.SKIP_STASH;
