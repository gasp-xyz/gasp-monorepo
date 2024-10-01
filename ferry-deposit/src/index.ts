import { Keyring } from "@polkadot/api";
import type { HeaderExtended } from "@polkadot/api-derive/type/types";
import type { KeyringPair } from "@polkadot/keyring/types";
import "dotenv/config";
import { signTx } from "gasp-sdk";
import "gasp-types";
import { BaseError, keccak256 } from "viem";
import {
	ETH_CHAIN_URL,
	MANGATA_CONTRACT_ADDRESS,
	MANGATA_NODE_URL,
	MIN_PROFIT,
	MNEMONIC,
	TX_COST,
} from "./common/constants.js";
import {
	Ferry,
	L1Api,
	L2Api,
	getApi,
	isSuccess,
	print,
} from "./utils/index.js";

async function main() {
	const api = await getApi(MANGATA_NODE_URL);
	const l2 = new L2Api(api);
	const l1 = new L1Api(ETH_CHAIN_URL);
	if (await l1.isRolldownDeployed()) {
		throw `Rolldown contract ${MANGATA_CONTRACT_ADDRESS} is not deployed on L1`;
	}

	if (await api.isReady) {
		throw `Cannot connect to ${MANGATA_NODE_URL}`;
	}

	const keyring = new Keyring({ type: "ethereum" });
	const keypair = keyring.createFromUri(MNEMONIC);
	const ferry = new Ferry(
		hexToU8a(keypair.address),
		l1,
		l2,
		TX_COST,
		MIN_PROFIT,
	);

	let inProgress = false;

	const unwatch = await api.derive.chain.subscribeFinalizedHeads(
		async (header: HeaderExtended) => {
			if (inProgress) {
				return;
			}

			inProgress = true;
			const pending = await ferry.getPendingDeposits();
			const profitable = await ferry.rateDeposits(pending);
			if (profitable.length > 0) {
				const depositToFerry = profitable[0];
				const status = await signTx(
					api,
					api.tx.rolldown.ferryDeposit(
						"Ethereum",
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
	.then(() => {
		print("Success");
	})
	.catch((e) => {
		console.error("Something went wrong", e);
		process.exit(1);
	});
function hexToU8a(address: string): Uint8Array {
	throw new Error("Function not implemented.");
}
