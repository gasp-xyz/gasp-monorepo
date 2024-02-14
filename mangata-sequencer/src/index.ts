import { Mangata, signTx } from "@mangata-finance/sdk";
import "@mangata-finance/types";
import { Keyring } from "@polkadot/api";
import "dotenv/config";
import { createPublicClient, webSocket } from "viem";

import * as fs from 'fs';

type ContractAddress = `0x${string}`;

const mangataContractAddress = process.env.MANGATA_CONTRACT_ADDRESS! as ContractAddress;

async function main() {
  let abi = JSON.parse(fs.readFileSync('./../rolldown-contract/out/rolldown.sol/RollDown.json', 'utf8'))["abi"] as unknown[];

  const api = await Mangata.instance([process.env.MANGATA_URL!]).api();

  const keyring = new Keyring({ type: "sr25519" });
  const collator = keyring.addFromUri(process.env.MNEMONIC!)

  const publicClient = createPublicClient({
    transport: webSocket(process.env.ETH_CHAIN_URL, {
      retryCount: 5,
    }),
  });

  await api.derive.chain.subscribeNewHeads(async (header) => {
    console.log(`block #${header.number} was authored by ${header.author}`);

    if (header.author?.toString() === collator.address) {
      const data = await publicClient.readContract({
        address: mangataContractAddress,
        abi: abi,
        functionName: "getUpdateForL2",
      }) as any;

      data.order = data.order.map((e: any) => {
        switch (e) {
          case 1: {
            return "WITHDRAWAL";
          }
          case 0: {
            return "DEPOSIT";
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
    }
  });
}

main()
  .then(() => {
    console.log("Success");
  })
  .catch((e) => console.error("Something went wrong", e));
