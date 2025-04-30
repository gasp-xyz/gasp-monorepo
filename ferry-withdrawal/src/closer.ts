import { hexToU8a, u8aToHex } from "@polkadot/util";
import type { PrivateKeyAccount, ReadContractErrorType } from "viem";
import { privateKeyToAccount } from "viem/accounts";
import type { HeaderExtended } from "@polkadot/api-derive/type/types";
import "dotenv/config";
import "gasp-types";

import { Withdrawal, isWithdrawal, toString } from "./Withdrawal.js";
import { Cancel, isCancel, toString as cancelToString } from "./Cancel.js";
import { L1Api } from "./l1/L1Api.js";
import { StashApi } from "./stash/StashApi.js";
import { L2Api, getApi } from "./l2/L2Api.js";
import { logger } from "./logger.js";
import {
	ETH_CHAIN_URL,
	LOG,
	MANGATA_CONTRACT_ADDRESS,
	MANGATA_NODE_URL,
	PRIVATE_KEY,
	TOKENS_TO_TRACK,
	TX_COST,
	STASH_URL,
	DELAY,
	BATCH_SIZE,
	REPLICA_COUNT,
	REPLICA_ID,
	MIN_REQUEST_ID,
	SKIP_STASH,
} from "./Config.js";
import { CloserService } from "./CloserService.js";
import { reportBalance, serveMetrics } from "./metrics.js";

async function main() {
	const api = await getApi(MANGATA_NODE_URL);
	const l1 = new L1Api(ETH_CHAIN_URL);
	const l2 = new L2Api(api, l1.chain.id);
	const stash = SKIP_STASH ? undefined : new StashApi(STASH_URL);

	logger.info(`Closer Serivce`);

	if (!(await api.isReady)) {
		throw `Cannot connect to ${MANGATA_NODE_URL}`;
	}

	const acc: PrivateKeyAccount = privateKeyToAccount(PRIVATE_KEY as any);
	serveMetrics();
	reportBalance(hexToU8a(acc.address), l1);

	logger.info(`Account         : ${acc.address}`);
	logger.info(`L1              : ${ETH_CHAIN_URL}`);
	logger.info(`L2              : ${MANGATA_NODE_URL}`);
	logger.info(`Rolldown        : ${MANGATA_CONTRACT_ADDRESS}`);
	logger.info(`Log level       : ${LOG}`);
	logger.info(`Skip stash      : ${SKIP_STASH}`);
	logger.info(`Replica Id      : ${REPLICA_ID} / ${REPLICA_COUNT}`);
	logger.info(`Min req id      : ${MIN_REQUEST_ID}`);

	if (REPLICA_ID > REPLICA_COUNT) {
		throw new Error(
			`replica id (${REPLICA_ID}) cannot be greater than replica count ${REPLICA_COUNT}`,
		);
	}

	TOKENS_TO_TRACK.forEach((token) => {
		logger.info(
			`Withdrawals to close ${u8aToHex(token[0])} minimal profit : ${token[1]}`,
		);
	});

	await l1.isRolldownDeployed(0n);
	await l1.isRolldownDeployed(DELAY);

	const native = await l1.getNativeTokenAddress();
	const balance = await l1.getBalance(native, hexToU8a(acc.address));

	logger.info(`Closer Balance: : ${balance}`);

	const closerService = new CloserService(
		l1,
		l2,
		stash,
		TOKENS_TO_TRACK,
		TX_COST,
		BATCH_SIZE,
		REPLICA_ID,
		REPLICA_COUNT,
		MIN_REQUEST_ID,
	);

	let inProgress = false;

	const unwatch = await api.derive.chain.subscribeFinalizedHeads(
		async (header: HeaderExtended) => {
			if (!inProgress) {
				inProgress = true;
				logger.info(`#${header.number} updating withdrawals to close`);
				await closerService.findRequestToClose(DELAY);
				const req = await closerService.getNextRequestToClose();
				if (req) {
					if (isWithdrawal(req)) {
						await closerService.closeWithdrawal(req, hexToU8a(PRIVATE_KEY));
					} else if (isCancel(req)) {
						await closerService.closeCancel(req, hexToU8a(PRIVATE_KEY));
					}
				} else {
					logger.debug(`#${header.number} nothing to close`);
				}
				inProgress = false;
			}
		},
	);
}

main().catch((e) => {
	console.error("Something went wrong", e.metaMessages || e);
	process.exit(1);
});
