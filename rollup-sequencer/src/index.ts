import util from "util";
import { Mangata, signTx, MangataGenericEvent } from "@mangata-finance/sdk";
import "@mangata-finance/types";
import { Keyring } from "@polkadot/api";
import "dotenv/config";
import { createPublicClient, encodeAbiParameters, webSocket } from "viem";
import { keccak256 } from "viem";
import rolldownAbi from "./RollDown.json" assert { type: "json" };
import { countReset } from "console";

type ContractAddress = `0x${string}`;

const mangataContractAddress = process.env
	.MANGATA_CONTRACT_ADDRESS! as ContractAddress;

function sleep_ms(ms: number) {
	return new Promise((resolve) => setTimeout(resolve, ms));
}

function isSuccess(events: MangataGenericEvent[]) {
  return events.some((event) => event.section === "system" && event.method === "ExtrinsicSuccess");
}

function getMinRequestId(l2Update: any) {
  let minId = Math.min.apply(null, l2Update.pendingCancelResultions.map(function(item: any) {
    return Number(item.requestId.id);
  }))

  minId = Math.min.apply(null, l2Update.pendingWithdrawalResolutions.map(function(item: any) {
    return Number(item.requestId.id);
  }))

  minId = Math.min.apply(minId, l2Update.pendingL2UpdatesToRemove.map(function(item: any) {
    return Number(item.requestId.id);
  }))

  minId = Math.min.apply(minId, l2Update.pendingDeposits.map(function(item: any) {
    return Number(item.requestId.id);
  }))

  if (minId === Infinity) {
    return null;
  } else {
    return minId;
  }
}

function getMaxRequestId(l2Update: any) {
  let maxId = Math.max.apply(null, l2Update.pendingCancelResultions.map(function(item: any) {
    return Number(item.requestId.id);
  }))

  maxId = Math.max.apply(maxId, l2Update.pendingWithdrawalResolutions.map(function(item: any) {
    return Number(item.requestId.id);
  }))

  maxId = Math.max.apply(maxId, l2Update.pendingL2UpdatesToRemove.map(function(item: any) {
    return Number(item.requestId.id);
  }))

  maxId = Math.max.apply(maxId, l2Update.pendingDeposits.map(function(item: any) {
    return Number(item.requestId.id);
  }))

  if (maxId === -Infinity) {
    return null;
  } else {
    return maxId;
  }
}


function countRequests(l2Update: any) {
  return l2Update.pendingCancelResultions.length +
    l2Update.pendingWithdrawalResolutions.length +
    l2Update.pendingL2UpdatesToRemove.length +
    l2Update.pendingDeposits.length;

}



function filterUpdates(l2Update: any, lastRequestId: number): any {

  const minId = getMinRequestId(l2Update);
  if (minId == null) {
    return l2Update;
  }
  let firstRequestId = Math.max(minId, lastRequestId + 1);


  while (l2Update.pendingDeposits.length > 0 && l2Update.pendingDeposits[0].requestId.id < firstRequestId) {
    l2Update.pendingDeposits.shift();
  }

  while (l2Update.pendingCancelResultions.length > 0 && l2Update.pendingCancelResultions[0].requestId.id < firstRequestId) {
    l2Update.pendingCancelResultions.shift();
  }

  while (l2Update.pendingWithdrawalResolutions.length > 0 && l2Update.pendingWithdrawalResolutions[0].requestId.id < firstRequestId) {
    l2Update.pendingWithdrawalResolutions.shift();
  }

  while (l2Update.pendingL2UpdatesToRemove.length > 0 && l2Update.pendingL2UpdatesToRemove[0].requestId.id < firstRequestId) {
    l2Update.pendingL2UpdatesToRemove.shift();
  }

  const maxAmountOfUpdates = 500;


  if (maxAmountOfUpdates > 0) {
    let lastRequestId = firstRequestId + maxAmountOfUpdates;

    while (l2Update.pendingDeposits.length > 0 && l2Update.pendingDeposits[l2Update.pendingDeposits.length - 1].requestId.id > lastRequestId) {
      l2Update.pendingDeposits.pop();
    }

    while (l2Update.pendingCancelResultions.length > 0 && l2Update.pendingCancelResultions[l2Update.pendingCancelResultions.length - 1].requestId.id > lastRequestId) {
      l2Update.pendingCancelResultions.pop();
    }

    while (l2Update.pendingWithdrawalResolutions.length > 0 && l2Update.pendingWithdrawalResolutions[l2Update.pendingWithdrawalResolutions.length - 1].requestId.id > lastRequestId) {
      l2Update.pendingWithdrawalResolutions.pop();
    }

    while (l2Update.pendingL2UpdatesToRemove.length > 0 && l2Update.pendingL2UpdatesToRemove[l2Update.pendingL2UpdatesToRemove.length - 1].requestId.id > lastRequestId) {
      l2Update.pendingL2UpdatesToRemove.pop();
    }

    return l2Update;
  } else {
    return l2Update;
  }

}

async function main() {
	let lastSubmitted = "";
	const abi = rolldownAbi.abi;
	const publicClient = createPublicClient({
		transport: webSocket(process.env.ETH_CHAIN_URL, {
			retryCount: 5,
		}),
	});

	while (true) {
		try {
			const data = (await publicClient.readContract({
				address: mangataContractAddress,
				abi: abi,
				functionName: "getUpdateForL2",
			})) as any;
			console.log(util.inspect(data, { depth: null }));
			break;
		} catch (e) {
      console.log(e)
			console.log(`${mangataContractAddress} contract not found`);
			await sleep_ms(1000);
		}
	}

	const api = await Mangata.instance([process.env.MANGATA_NODE_URL!]).api();
	await api.isReady;

	const keyring = new Keyring({ type: "sr25519" });
	const collator = keyring.addFromUri(process.env.MNEMONIC!);
  let lastRequestId = parseInt((await api.query.rolldown.lastProcessedRequestOnL2("Ethereum")).toString());
  let inProgress = false;

	await api.derive.chain.subscribeNewHeads(async (header) => {
		const apiAt = await api.at(header.hash);
		console.log(`block #${header.number} was authored by ${header.author}`);

		if (header.author?.toString() === collator.address) {

      let rights = await api.query.rolldown.sequencerRights(collator.address);
      if (rights.unwrap().readRights.toNumber() === 0) {
        console.log(`no read rights left, skipping...`);
        return;
      }

      if (inProgress) {
        console.log(`in progress, skipping...`);
      }else{
        inProgress = true;
      }

			const data = (await publicClient.readContract({
				address: mangataContractAddress,
				abi: abi,
				functionName: "getUpdateForL2",
			})) as any;


			// @ts-ignore
			const encodedData = encodeAbiParameters(
				abi.find((e: any) => e!.name === "getUpdateForL2")!.outputs!,
				[data],
			);


			const nativeL1Update = (await api.rpc.rolldown.get_native_l1_update(
				encodedData.substring(2),
			)).unwrap();

      let filtered = filterUpdates(nativeL1Update, lastRequestId);
      let requestsCount = countRequests(filtered);
 
			if (requestsCount > 0) {
				let result =  await signTx(
					api,
					api.tx.rolldown.updateL2FromL1(filtered),
					collator,
				);

        if (isSuccess(result)) {

          console.log(`l1update was submitted successfully`);

          if (lastSubmitted == keccak256(encodedData)){
            lastRequestId = getMaxRequestId(filtered)!;
          }else{
            lastSubmitted = keccak256(encodedData);
            lastRequestId = parseInt((await api.query.rolldown.lastProcessedRequestOnL2("Ethereum")).toString());
          }

        }else{
          console.log(`l1update was submitted unsuccessfully`);
        }
			} else {
				console.log(`L1Update was already submitted ${encodedData}`);
			}
      inProgress = false;
		}

		const events = await apiAt.query.system.events();

		const pendingRequestsEvents = events.filter(
			(event) =>
				event.event.section === "rolldown" &&
				event.event.method === "L1ReadStored",
		);

		if (pendingRequestsEvents.length > 0) {
			pendingRequestsEvents.forEach((record) => {
				record.event.data.forEach(async (data, index) => {
					const requestId = (data as unknown as string[])[1];
					const { start, end } = (data as any)[2] as unknown as {
						start: string;
						end: string;
					};

					const contractData = (await publicClient.readContract({
						address: mangataContractAddress,
						abi: abi,
						functionName: "getPendingRequests",
						args: [start, end],
					})) as any;

					// @ts-ignore
					const encodedData = encodeAbiParameters(
						abi.find((e: any) => e!.name === "getPendingRequests")!.outputs!,
						[contractData],
					);

					const verified = await api.rpc.rolldown.verify_pending_requests(
						keccak256(encodedData),
						requestId.toString(),
					);

					if (!verified.toPrimitive()) {
						await signTx(
							api,
							api.tx.rolldown.cancelRequestsFromL1(requestId.toString()),
							collator,
						);
					}
				});
			});
		}
	});
}

main()
	.then(() => {
		console.log("Success");
	})
	.catch((e) => console.error("Something went wrong", e));
