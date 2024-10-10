import util from "node:util";
import { Keyring } from "@polkadot/api";
import type { HeaderExtended } from "@polkadot/api-derive/type/types";
import { hexToU8a } from "@polkadot/util";
import "dotenv/config";
import { signTx } from "gasp-sdk";
import "gasp-types";
import {
	TOKENS_TO_TRACK,
	ETH_CHAIN_URL,
	MANGATA_CONTRACT_ADDRESS,
	MANGATA_NODE_URL,
	PRIVATE_KEY,
	TX_COST,
} from "./common/constants.js";

import { Ferry } from "./ferry/index.js";
import { L1Api } from "./l1/index.js";
import { L2Api, getL1ChainType } from "./l2/index.js";
import { getApi, isSuccess, print } from "./utils/index.js";
import { PrivateKeyAccount } from "viem";
import { privateKeyToAccount } from "viem/accounts";

async function main() {
	const api = await getApi(MANGATA_NODE_URL);
	const l2 = new L2Api(api);
	const l1 = new L1Api(ETH_CHAIN_URL);

	if (!(await api.isReady)) {
		throw `Cannot connect to ${MANGATA_NODE_URL}`;
	}


	const acc: PrivateKeyAccount = privateKeyToAccount(PRIVATE_KEY as any);
	const ferry = new Ferry(
		hexToU8a(acc.address),
		l1,
		l2,
		JSON.parse(TOKENS_TO_TRACK),
		TX_COST,
	);

	let inProgress = false;

	const unwatch = await api.derive.chain.subscribeFinalizedHeads(
		async (header: HeaderExtended) => {
			console.info(`New L2 block: #${header.number}`);
			if (inProgress) {
				return;
			}

			if (!(await l1.isRolldownDeployed())) {
				console.info(
					`rolldown contract ${MANGATA_CONTRACT_ADDRESS} not found yet`,
				);
				return;
			}

			inProgress = true;
      const pastWithdrawalsToClose = await ferry.getPendingWithdrawals();
      if (pastWithdrawalsToClose.length > 0) {
        await l1.close(pastWithdrawalsToClose[0], hexToU8a(acc.address));
        inProgress = false;
        return;
      }else{
        const pending = await ferry.getPendingWithdrawals();
        const rated = await ferry.rateWithdrawals(pending);
        if (rated.length == 0) {
          inProgress = false;
          return;
        }
        await l1.ferry(rated[0], hexToU8a(PRIVATE_KEY));
      }
			// const pending = await ferry.getPendingDeposits();
			// const profitable = await ferry.rateDeposits(pending);

			// console.info(`Found ${profitable.length} proffitable deposits`);
			// if (profitable.length > 0) {
			// 	const depositToFerry = profitable[0];
			// 	console.info(`Ferrying deposit ${util.inspect(depositToFerry)}`);
			// 	if (!(await ferry.hasFundsToCoverTxFee())) {
			// 		throw new Error(`Not enough funds to cover tx fee`);
			// 	}
			// 	const status = await signTx(
			// 		api,
			// 		api.tx.rolldown.ferryDeposit(
			// 			getL1ChainType(api),
			// 			{ origin: "L1", id: depositToFerry.requestId },
			// 			depositToFerry.depositRecipient,
			// 			depositToFerry.tokenAddress,
			// 			depositToFerry.amount,
			// 			depositToFerry.timeStamp,
			// 			depositToFerry.ferryTip,
			// 			await l1.getDepostiHash(depositToFerry.requestId),
			// 		),
			// 		keypair,
			// 	);
			//
			// 		print(
			//      if (isSuccess(status)) {
			// 			`Ferrying deposit ${depositToFerry.requestId} to ${depositToFerry.depositRecipient}`,
			// 		);
			// 	} else {
			// 		print(
			// 			`Failed to ferry deposit ${depositToFerry.requestId} to ${depositToFerry.depositRecipient}`,
			// 		);
			// 	}
			// }
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
