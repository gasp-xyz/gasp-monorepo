import { Mangata } from "gasp-sdk";
import "gasp-types";
import "dotenv/config";
import {
	EIGEN_ABI,
	EIGEN_CONTRACT_ADDRESS,
	FINALIZATION_SOURCE,
	MANGATA_NODE_URL,
  DEV_MODE,
} from "./common/constants.js";
import "./util/polyfill.js";
import { print, sendUpdateToL1, closeWithdrawals, getLatestRequestIdSubmittedToL1} from "./util/utils.js";
import {
	ethAccount,
	getChain,
	getPublicClient,
	getWalletClient,
	webSocketTransport,
} from "./viem/client.js";


async function main() {
	const api = await Mangata.instance([MANGATA_NODE_URL]).api();
	const publicClient = getPublicClient({
		transport: webSocketTransport,
		chain: getChain(),
	});
	const walletClient = getWalletClient({
		account: ethAccount,
		transport: webSocketTransport,
	});

	let unwatch: any;
	let inProgress = false;
  let latestRequestIdSubmittedToL1 = 0n;

	if (FINALIZATION_SOURCE === "relay") {
		unwatch = await api.rpc.chain.subscribeFinalizedHeads(async (header) => {
			if (!inProgress) {
				inProgress = true;
				print(`Chain is at block: #${header.number}`);

        if (DEV_MODE !== "false") {
          console.log("DEV_MODE is enabled. Withdrawals will be autoamtically executed");
          latestRequestIdSubmittedToL1 = await closeWithdrawals(api, walletClient, publicClient, latestRequestIdSubmittedToL1);
        }

				await sendUpdateToL1(
					api,
					walletClient,
					publicClient,
					header.hash,
				);
				inProgress = false;
			} else {
				print(`Chain is at block: #${header.number} - tx pending`);
			}
		});
	} else {

		print("Subscribing to eth events");
		unwatch = publicClient.watchContractEvent({
			address: EIGEN_CONTRACT_ADDRESS,
			abi: EIGEN_ABI,
			eventName: "TaskCompleted",
			onLogs: async (logs) => {
				print("Received task notification from L1");
				for (const log of logs) {
					 await sendUpdateToL1(
						api,
						walletClient,
						publicClient,
						(log as any).args.blockHash,
					);
				}
			},
		});
	}

	// We need to  unwatch if server is down or restarted to prevent leaks
	process.on("SIGINT", async () => {
		console.error("Stopping the process..., SIGINT signal");
		unwatch(); // Unsubscribe from event watching
		process.exit();
	});

	process.on("SIGTERM", async () => {
		console.error("Stopping the process..., SIGTERM signal");
		unwatch(); // Unsubscribe from event watching
		process.exit();
	});

	process.on("SIGHUP", async () => {
		console.error("Stopping the process..., SIGHUP signal");
		unwatch(); // Unsubscribe from event watching
		process.exit();
	});
}

main()
	.then(() => print("Success"))
	.catch((e) => {
      console.error("Something went wrong", e)
      process.exit(1);
    }
  );
