import { signTx } from "gasp-sdk";
import "gasp-types";
import type { HeaderExtended } from "@polkadot/api-derive/type/types";
import "dotenv/config";
import { keccak256, BaseError } from "viem";
import { MANGATA_NODE_URL, MNEMONIC } from "./common/constants.js";
import {
	getApi,
	getCollator,
	getLastRequestId,
	getMaxRequestId,
	getSelectedSequencerWithRights,
	initReadContractWithRetry,
	isSuccess,
	print,
	processDataForL2Update,
	processPendingRequestsEvents,
} from "./utils/index.js";
import { getPublicClient } from "./viem/client.js";
import { webSocketTransport } from "./viem/transport.js";

async function main() {
	let inProgress = false;

	const publicClient = getPublicClient({ transport: webSocketTransport });

	const api = await getApi(MANGATA_NODE_URL);

	await initReadContractWithRetry(publicClient, api);

	let lastRequestId = await getLastRequestId(api);

  await api.derive.chain.subscribeNewHeads(async (header: HeaderExtended) => {
    const collator = getCollator("ethereum", MNEMONIC);
    print(`collator address: ${collator.address}`)
    print(`block #${header.number} was authored by ${header.author}`);
    const { isSequencerSelected, hasReadRights, hasCancelRights, selectedSequencer } =
      await getSelectedSequencerWithRights(api, collator.address, header.hash);
    print(`me ${collator.address}`);
    print(`selected : ${selectedSequencer}`);
    print(`is selected ${isSequencerSelected}`);
    print(`has read rights : ${hasReadRights}`);
    print(`has cancel rights : ${hasCancelRights}`);
    if (isSequencerSelected && hasReadRights) {
        if (inProgress) {
          return;
        }else{
					print("In progress, skipping...");
          inProgress = true;
        }
        const nativeL1Update = await processDataForL2Update(
          api,
          publicClient,
        );

        if (nativeL1Update === null) {
          inProgress = false;
          return;
        }

        let maxRequestId = getMaxRequestId(nativeL1Update);

        console.log(`maxRequestId: ${maxRequestId}`);
				if (maxRequestId > lastRequestId) {
					const result = await signTx(
						api,
						api.tx.rolldown.updateL2FromL1(nativeL1Update),
						collator,
					);

					if (isSuccess(result)) {
						print("L1update was submitted successfully");
							lastRequestId = maxRequestId;
					} else {
						print("L1update was submitted unsuccessfully");
					}
				} else {
					print(`L1Update with max id == ${lastRequestId} was already submitted`);
				}
			
			inProgress = false;
		}

  if (hasCancelRights && !inProgress){
    inProgress = true;
		await processPendingRequestsEvents(
			api,
			publicClient,
			collator,
		);
    inProgress = false;
  }
	});
}

main()
  .then(() => {
    print("Success");
  })
  .catch((e) => {
    console.error("Something went wrong", e);
    process.exit( 1); 
  })
