import "@mangata-finance/types";
import { Mangata, signTx } from "@mangata-finance/sdk";
import { Keyring } from "@polkadot/api";
import "dotenv/config";
import {createPublicClient, encodeAbiParameters, webSocket} from "viem";
import rolldownAbi from './RollDown.json' assert {type: 'json'};

type ContractAddress = `0x${string}`;

const mangataContractAddress = process.env.MANGATA_CONTRACT_ADDRESS! as ContractAddress;

function sleep_ms(ms: number) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

async function main() {
  let abi = rolldownAbi.abi;
  const publicClient = createPublicClient({
    transport: webSocket(process.env.ETH_CHAIN_URL, {
      retryCount: 5,
    }),
  });

  while (true) {
    try {
      const data = await publicClient.readContract({
        address: mangataContractAddress,
        abi: abi,
        functionName: "getUpdateForL2",
      }) as any;
      console.log(data)
      break;
    } catch (e) {
      console.log(`${mangataContractAddress} contract not found`)
      await sleep_ms(1000);
    }
  }

  const api = await Mangata.instance([process.env.MANGATA_NODE_URL!]).api();

  const keyring = new Keyring({ type: "sr25519" });
  const collator = keyring.addFromUri(process.env.MNEMONIC!)

  await api.derive.chain.subscribeNewHeads(async (header) => {
    console.log(`block #${header.number} was authored by ${header.author}`);

    if (header.author?.toString() === collator.address) {
      const data = await publicClient.readContract({
        address: mangataContractAddress,
        abi: abi,
        functionName: "getUpdateForL2",
      }) as any;

      console.log(data)

      data.order = data.order.map((e: any) => {
        switch (e) {
          case 0: {
            return "DEPOSIT";
          }
          case 1: {
            return "WITHDRAWAL";
          }
          case 2: {
            return "CANCEL_RESOLUTION";
          }
          case 3: {
            return "L2_UPDATES_TO_REMOVE";
          }
        }
      });

      await signTx(api, api.tx.rolldown.updateL2FromL1(data), collator);
    } else {
      const data = await publicClient.readContract({
        address: mangataContractAddress,
        abi: abi,
        functionName: "getUpdateForL2",
      }) as any;

      // @ts-ignore
      let encodedData = encodeAbiParameters(abi.find((e: any) => e!.name === "getUpdateForL2").outputs, [data]);

      const events = await api.query.system.events()
      const pendingRequestsEvents = events.filter(event => event.event.section === "rolldown" && event.event.method === "PendingRequestStored")
      if (pendingRequestsEvents.length > 0) {
        const eventData= pendingRequestsEvents.map(event => event.event.data.toPrimitive())
        const requestId = eventData[2]!.toString()

        const verified = await api.rpc.rolldown.verify_pending_requests(encodedData, requestId)
        if (!verified) {
          await signTx(api, api.tx.rolldown.cancelRequestsFromL1(requestId), collator);
        }
      }
    }
  });
}

main()
  .then(() => {
    console.log("Success");
  })
  .catch((e) => console.error("Something went wrong", e));
