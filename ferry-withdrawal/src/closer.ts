import { hexToU8a, u8aToHex } from "@polkadot/util";
import type { PrivateKeyAccount, ReadContractErrorType } from "viem";
import { privateKeyToAccount } from "viem/accounts";
import type { HeaderExtended } from "@polkadot/api-derive/type/types";
import "dotenv/config";
import "gasp-types";

import { Withdrawal, isWithdrawal, toString } from "./Withdrawal.js";
import { Cancel, isCancel, toString as cancelToString } from "./Cancel.js";
import { L1Api } from "./l1/L1Api.js";
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
	DELAY,
} from "./Config.js";
import { CloserService } from "./CloserService.js";

const BATCH_SIZE = 1000n;

async function main() {
	const api = await getApi(MANGATA_NODE_URL);
	const l2 = new L2Api(api);
	const l1 = new L1Api(ETH_CHAIN_URL);

	logger.info(`Closer Serivce`);

	if (!(await api.isReady)) {
		throw `Cannot connect to ${MANGATA_NODE_URL}`;
	}

	const acc: PrivateKeyAccount = privateKeyToAccount(PRIVATE_KEY as any);

	logger.info(`Account      : ${acc.address}`);
	logger.info(`L1           : ${ETH_CHAIN_URL}`);
	logger.info(`L2           : ${MANGATA_NODE_URL}`);
	logger.info(`Rolldown     : ${MANGATA_CONTRACT_ADDRESS}`);
	logger.info(`Log level    : ${LOG}`);

	TOKENS_TO_TRACK.forEach((token) => {
		logger.info(
			`Withdrawals to close ${u8aToHex(token[0])} minimal profit : ${token[1]}`,
		);
	});

	await l1.isRolldownDeployed(0n);
	await l1.isRolldownDeployed(DELAY);

	const closerService = new CloserService(
		l1,
		l2,
		TOKENS_TO_TRACK,
		TX_COST,
		BATCH_SIZE,
	);

	let inProgress = false;

	const unwatch = await api.derive.chain.subscribeFinalizedHeads(
		async (header: HeaderExtended) => {
			inProgress = true;
			logger.info(`#${header.number} updating withdrawals to close`);
			await closerService.findRequestToClose(DELAY);
			const req = await closerService.getNextRequestToClose();
			if (req) {
				if (isWithdrawal(req)) {
					logger.info(`#${header.number} Closing withdrawal ${toString(req)}`);
					await closerService.closeWithdrawal(req, hexToU8a(PRIVATE_KEY));
				} else if (isCancel(req)) {
					logger.info(
						`#${header.number} Closing withdrawal ${cancelToString(req)}`,
					);
					await closerService.closeCancel(req, hexToU8a(PRIVATE_KEY));
				}
			} else {
				logger.debug(`#${header.number} nothing to close`);
			}
			inProgress = false;
		},
	);
}

main().catch((e) => {
	console.error("Something went wrong", e.metaMessages || e);
	process.exit(1);
});
