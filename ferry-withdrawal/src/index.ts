import type { HeaderExtended } from "@polkadot/api-derive/type/types";
import { hexToU8a, u8aToHex } from "@polkadot/util";
import "dotenv/config";
import "gasp-types";
import JSONbig from "json-bigint";
import {
	ETH_CHAIN_URL,
	LOG,
	LOOK_BACK_HOURS,
	MANGATA_CONTRACT_ADDRESS,
	MANGATA_NODE_URL,
	PRIVATE_KEY,
	TOKENS_TO_TRACK,
	TX_COST,
} from "./common/constants.js";

import type { PrivateKeyAccount } from "viem";
import { privateKeyToAccount } from "viem/accounts";
import { z } from "zod";
import { toString } from "./common/withdrawal.js";
import { Ferry } from "./ferry/index.js";
import { L1Api } from "./l1/L1Api.js";
import { L2Api } from "./l2/index.js";
import { getApi, logger, print } from "./utils/index.js";

export const TOKENS_TO_TRACK_SCHEMA = z
	.tuple([
		z.string().transform((x) => hexToU8a(x, 160)),
		z.bigint(),
		z.bigint(),
	])
	.array();
export type AppConfig = z.infer<typeof TOKENS_TO_TRACK_SCHEMA>;

//TODO: improve logging

async function main() {
	const api = await getApi(MANGATA_NODE_URL);
	const l2 = new L2Api(api);
	const l1 = new L1Api(ETH_CHAIN_URL);

	if (!(await api.isReady)) {
		throw `Cannot connect to ${MANGATA_NODE_URL}`;
	}

	const enabled_tokens = TOKENS_TO_TRACK_SCHEMA.parse(
		JSONbig({ alwaysParseAsBig: true, useNativeBigInt: true }).parse(
			TOKENS_TO_TRACK,
		),
	);
	const acc: PrivateKeyAccount = privateKeyToAccount(PRIVATE_KEY as any);

	logger.info(`Account      : ${acc.address}`);
	logger.info(`L1           : ${ETH_CHAIN_URL}`);
	logger.info(`L2           : ${MANGATA_NODE_URL}`);
	logger.info(`Rolldown     : ${MANGATA_CONTRACT_ADDRESS}`);
	logger.info(`Close period : ${LOOK_BACK_HOURS} hours`);
	logger.info(`Log level    : ${LOG}`);

	enabled_tokens.forEach((token) => {
		logger.info(
			`Token to ferry ${u8aToHex(token[0])} minimal profit : ${
				token[1]
			} weight : ${token[2]}`,
		);
	});

	const ferry = new Ferry(
		hexToU8a(acc.address),
		l1,
		l2,
		enabled_tokens,
		TX_COST,
	);

	let inProgress = false;

	const unwatch = await api.derive.chain.subscribeFinalizedHeads(
		async (header: HeaderExtended) => {
			logger.info(`New L2 block: #${header.number}`);
			if (inProgress) {
				return;
			}

			if (!(await l1.isRolldownDeployed())) {
				logger.info(
					`rolldown contract ${MANGATA_CONTRACT_ADDRESS} not found yet`,
				);
				return;
			}

			inProgress = true;
			const pastWithdrawalsToClose = await ferry.getPastFerriesReadyToClose(
				LOOK_BACK_HOURS * 5 * 60,
			);
			logger.debug(
				`Found ${pastWithdrawalsToClose.length} past withdrawals ready to close `,
			);
			if (pastWithdrawalsToClose.length > 0) {
				logger.info(
					`Closing withdrawal ${toString(pastWithdrawalsToClose[0])}`,
				);
				await l1.close(pastWithdrawalsToClose[0], hexToU8a(PRIVATE_KEY));
				inProgress = false;
				return;
			} else {
				const pending = await ferry.getPendingWithdrawals();
				logger.info(`Found ${pending.length} pending withdrawals`);
				const rated = await ferry.rateWithdrawals(pending);
				if (rated.length === 0) {
					inProgress = false;
					return;
				}
				logger.info(`Ferry withdrawal ${toString(rated[0])}`);
				await l1.ferry(rated[0], hexToU8a(PRIVATE_KEY));
			}
			inProgress = false;
		},
	);
}

main()
	.then(() => {
		print("Success");
	})
	.catch((e) => {
		console.error("Something went wrong", e);
		process.exit(1);
	});
