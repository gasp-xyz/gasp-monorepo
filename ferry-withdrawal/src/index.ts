import util from "node:util";
import { Keyring } from "@polkadot/api";
import JSONbig  from 'json-bigint';
import type { HeaderExtended } from "@polkadot/api-derive/type/types";
import { hexToU8a, u8aToHex } from "@polkadot/util";
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
  LOOK_BACK_HOURS
} from "./common/constants.js";

import { Ferry } from "./ferry/index.js";
import { L1Api } from "./l1/index.js";
import { L2Api, getL1ChainType } from "./l2/index.js";
import { getApi, isSuccess, print } from "./utils/index.js";
import { PrivateKeyAccount } from "viem";
import { privateKeyToAccount } from "viem/accounts";
import { z } from "zod";
import { Withdrawal } from "./common/withdrawal.js";

export const TOKENS_TO_TRACK_SCHEMA = z.tuple([
	z.string().transform((x) => hexToU8a(x,160)),
	z.bigint(),
	z.bigint(),
]).array();
export type AppConfig = z.infer<typeof TOKENS_TO_TRACK_SCHEMA>;


function printWithdrawal(withdrawal: Withdrawal): void {
  console.log(`requestId: ${withdrawal.requestId}`);
  console.log(`withdrawalRecipient: ${u8aToHex(withdrawal.withdrawalRecipient)}`);
  console.log(`tokenAddress: ${u8aToHex(withdrawal.tokenAddress)}`);
  console.log(`amount: ${withdrawal.amount}`);
  console.log(`ferryTip: ${withdrawal.ferryTip}`);
  console.log(`hash: ${u8aToHex(withdrawal.hash)}`);
}

//TODO: improve logging
//TODO: log level configurable from env
//TODO: automate ci

async function main() {
	const api = await getApi(MANGATA_NODE_URL);
	const l2 = new L2Api(api);
	const l1 = new L1Api(ETH_CHAIN_URL);

	if (!(await api.isReady)) {
		throw `Cannot connect to ${MANGATA_NODE_URL}`;
	}

  const enabled_tokens = TOKENS_TO_TRACK_SCHEMA.parse(JSONbig({ alwaysParseAsBig: true, useNativeBigInt: true}).parse(TOKENS_TO_TRACK));
	const acc: PrivateKeyAccount = privateKeyToAccount(PRIVATE_KEY as any);
  console.info(`Using account ${acc.address}`);
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
      const pastWithdrawalsToClose = await ferry.getPastFerriesReadyToClose(LOOK_BACK_HOURS*5*60);
      if (pastWithdrawalsToClose.length > 0) {
        console.info(`Closing withdrawal`);
        printWithdrawal(pastWithdrawalsToClose[0]);
        await l1.close(pastWithdrawalsToClose[0], hexToU8a(PRIVATE_KEY));
        inProgress = false;
        return;
      }else{
        const pending = await ferry.getPendingWithdrawals();
        console.info(`Found ${pending.length} pending withdrawals`);
        const rated = await ferry.rateWithdrawals(pending);
        if (rated.length == 0) {
          inProgress = false;
          return;
        }
        console.info(`Ferrying withdrawal`);
        printWithdrawal(rated[0]);
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
