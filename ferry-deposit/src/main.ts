import util from "node:util";
import { Keyring } from "@polkadot/api";
import type { HeaderExtended } from "@polkadot/api-derive/type/types";
import { hexToU8a, u8aToHex } from "@polkadot/util";
import "dotenv/config";
import { signTx } from "gasp-sdk";
import "gasp-types";
import {
	BLOCK_DELAY,
	ETH_CHAIN_URL,
	MANGATA_CONTRACT_ADDRESS,
	MANGATA_NODE_URL,
	TOKENS_TO_TRACK,
	MNEMONIC,
	TX_COST,
	LOG,
} from "./config.js";

import { Ferry } from "./Ferry.js";
import { L1Api } from "./l1/L1Api.js";
import { L2Api, getL1ChainType } from "./l2/L2Api.js";
import { getApi, isSuccess, print } from "./utils.js";
import { logger } from "./logger.js";

async function main() {
	const api = await getApi(MANGATA_NODE_URL);
	const l2 = new L2Api(api);
	const l1 = new L1Api(ETH_CHAIN_URL, BLOCK_DELAY);

	logger.info(`Ferry Deposit`);
	logger.info(`L1             : ${ETH_CHAIN_URL}`);
	logger.info(`L1 Block Delay : ${ETH_CHAIN_URL}`);
	logger.info(`L2             : ${MANGATA_NODE_URL}`);
	logger.info(`Log level      : ${LOG}`);

	TOKENS_TO_TRACK.forEach((token) => {
		logger.info(
			`Token to ferry ${u8aToHex(token[0])} minimal profit : ${
				token[1]
			} weight : ${token[2]}`,
		);
	});

	if (!(await api.isReady)) {
		throw `Cannot connect to ${MANGATA_NODE_URL}`;
	}

	const keyring = new Keyring({ type: "ethereum" });
	const keypair = keyring.createFromUri(MNEMONIC);
	const ferry = new Ferry(
		hexToU8a(keypair.address),
		l1,
		l2,
		TX_COST,
		TOKENS_TO_TRACK,
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
			const pending = await ferry.getPendingDeposits();
			const profitable = await ferry.rateDeposits(pending);

			logger.info(`Found ${profitable.length} proffitable deposits`);
			if (profitable.length > 0) {
				const depositToFerry = profitable[0];
				logger.info(`Ferrying deposit ${util.inspect(depositToFerry)}`);
				if (!(await ferry.hasFundsToCoverTxFee())) {
					throw new Error(`Not enough funds to cover tx fee`);
				}
				const status = await signTx(
					api,
					api.tx.rolldown.ferryDeposit(
						getL1ChainType(api),
						{ origin: "L1", id: depositToFerry.requestId },
						depositToFerry.depositRecipient,
						depositToFerry.tokenAddress,
						depositToFerry.amount,
						depositToFerry.timeStamp,
						depositToFerry.ferryTip,
						await l1.getDepostiHash(depositToFerry.requestId),
					),
					keypair,
				);

				if (isSuccess(status)) {
					print(
						`Ferrying deposit ${depositToFerry.requestId} to ${depositToFerry.depositRecipient}`,
					);
				} else {
					print(
						`Failed to ferry deposit ${depositToFerry.requestId} to ${depositToFerry.depositRecipient}`,
					);
				}
			}
			inProgress = false;
		},
	);
}

main()
	.then(() => {})
	.catch((e) => {
		logger.error("Something went wrong", e);
		process.exit(1);
	});
