
import { hexToU8a, u8aToHex } from "@polkadot/util";
import type { PrivateKeyAccount } from "viem";
import { privateKeyToAccount } from "viem/accounts";
import type { HeaderExtended } from "@polkadot/api-derive/type/types";
import "dotenv/config";
import "gasp-types";

import { toString } from "./Withdrawal.js";
import { L1Api } from "./l1/L1Api.js";
import { L2Api, getApi } from "./l2/L2Api.js";
import { logger } from "./logger.js";
import {
	ETH_CHAIN_URL,
	LOG,
	LOOK_BACK_HOURS,
	MANGATA_CONTRACT_ADDRESS,
	MANGATA_NODE_URL,
	PRIVATE_KEY,
	TOKENS_TO_TRACK,
	TX_COST,
} from "./Config.js";
import { CloserService } from "./CloserService.js";

const BATCH_SIZE = 100n;

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
			`Token to ferry ${u8aToHex(token[0])} minimal profit : ${
				token[1]
			} weight : ${token[2]}`,
		);
	});

	const closerService = new CloserService(
		l1,
		l2,
		TOKENS_TO_TRACK,
		TX_COST,
	);

	let inProgress = false;

	const unwatch = await api.derive.chain.subscribeFinalizedHeads(
		async (header: HeaderExtended) => {
			inProgress = true;
      logger.info(`#${header.number} updating withdrawals to close`);
      await closerService.findWithdrawalsToClose();
      const withdrawal = await closerService.getNextWithdrawalToClose();
      if (withdrawal) { 
        logger.info(`#${header.number} Closing withdrawal ${toString(withdrawal)}`);
        await closerService.closeWithdrawal(withdrawal, hexToU8a(PRIVATE_KEY));
      }else{
        logger.info(`#${header.number} no withdrawals left`);
      }
			inProgress = false;
		},
	);
}

main()
	.then(() => {
	})
	.catch((e) => {
		console.error("Something went wrong", e);
		process.exit(1);
	});
