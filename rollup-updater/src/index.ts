import { Mangata } from "@mangata-finance/sdk";
import "@mangata-finance/types";
import "@mangata-finance/types";
import { ApiPromise } from "@polkadot/api";
import "dotenv/config";
import {
  TestClient,
  WalletClient,
  createPublicClient,
  createWalletClient,
  keccak256,
  webSocket,
} from "viem";
import { defineChain } from "viem";
import { decodeAbiParameters } from "viem";
import { privateKeyToAccount } from "viem/accounts";
import { holesky } from "viem/chains";
import eigenContractAbi from "./FinalizerTaskManager.json" assert {
  type: "json",
};
import rolldownAbi from "./RollDown.json" assert { type: "json" };

type ContractAddress = `0x${string}`;

const eigenContractAddress = process.env
  .EIGEN_CONTRACT_ADDRESS! as ContractAddress;

const mangataContractAddress = process.env
  .MANGATA_CONTRACT_ADDRESS! as ContractAddress;

const finalizationSource = process.env.FINALIZATION_SOURCE;
const verbose = process.env.VERBOSE;
const limit = process.env.LIMIT! || "0";


let lastStoredUpdateHash = "";
let lastSubmittedId = 0;


function getMinRequestId(l2Update: any) {
  let minId = Math.min.apply(null,
    [
      l2Update[0].withdrawals,
      l2Update[0].cancels,
      l2Update[0].results
    ].flat()
    .map(function(item: any) {
    return Number(item.requestId.id);
  }))

  if (minId === Infinity) {
    return null;
  } else {
    return minId;
  }
}


function getMaxRequestId(l2Update: any) {
  let maxId = Math.max.apply(null,
    [
      l2Update[0].withdrawals,
      l2Update[0].cancels,
      l2Update[0].results
    ].flat()
    .map(function(item: any) {
    return Number(item.requestId.id);
  }));

  if (maxId === -Infinity) {
    return null;
  } else {
    return maxId;
  }
}

function filterUpdates(l2Update: any): any {

  l2Update[0].withdrawals = l2Update[0].withdrawals.filter((elem: any) => {
    return elem.requestId.id > lastSubmittedId;
  });
  l2Update[0].cancels = l2Update[0].cancels.filter((elem: any) => {
    return elem.requestId.id > lastSubmittedId;
  });
  l2Update[0].results = l2Update[0].results.filter((elem: any) => {
    return elem.requestId.id > lastSubmittedId;
  });

  const minId = getMinRequestId(l2Update);
  if (minId == null) {
    return l2Update;
  }

  const maxAmountOfUpdates = parseInt(limit);
  console.log(`minID: ${minId}`);
  console.log(`maxAmountOfUpdates ${maxAmountOfUpdates}`);
  if (maxAmountOfUpdates > 0) {
    l2Update[0].withdrawals = l2Update[0].withdrawals.filter((elem: any) => {
      return elem.requestId.id < BigInt(minId + maxAmountOfUpdates);
    });
    l2Update[0].cancels = l2Update[0].cancels.filter((elem: any) => {
      return elem.requestId.id < BigInt(minId + maxAmountOfUpdates);
    });
    l2Update[0].results = l2Update[0].results.filter((elem: any) => {
      return elem.requestId.id < BigInt(minId + maxAmountOfUpdates);
    });
    return l2Update;
  } else {
    return l2Update;
  }

}


function getChain() {
  if (process.env.CHAIN == "holesky") {
    return holesky
  } else {
    return defineChain({
      id: 31337,
      name: "anvil",
      network: "Anvil",
      nativeCurrency: {
        decimals: 18,
        name: "Ether",
        symbol: "ETH",
      },
      rpcUrls: {
        public: {
          http: ["ws://127.0.0.1:8545"],
        },
        default: {
          http: ["ws://127.0.0.1:8545"],
        },
      },
    });
  }
}

async function sendUpdateToL1(
  api: ApiPromise,
  walletClient: any,
  abi: any,
  blockHash: any,
) {
  console.log(`HASH ${blockHash} `);
  const pendingUpdates = await api.rpc.rolldown.pending_updates(blockHash);

  const offset = 0;
  const updateHash = keccak256(pendingUpdates.toHex());

  const l2Update = decodeAbiParameters(
    abi.find((e: any) => e.name === "update_l1_from_l2")!.inputs,
    pendingUpdates.toHex(),
  );

  if (verbose) {
    console.log(`l2Update:  ${JSON.stringify(l2Update, null, 2)}`);
  }

  const update = filterUpdates(l2Update);
  if (verbose) {
    console.log(`filtered l2Update:  ${JSON.stringify(update, null, 2)}`);
  }

  const reqCount =
    update[0].withdrawals.length +
    update[0].cancels.length +
    update[0].results.length;

  if (reqCount == 0) {
    return null;
  }

  const storageHash = await walletClient.writeContract({
    chain: getChain(),
    abi: abi,
    address: mangataContractAddress,
    functionName: "update_l1_from_l2",
    args: update,
    // gas: 9999999n,
  });

  lastStoredUpdateHash = updateHash;
  let maxId = getMaxRequestId(update);
  if (maxId !== null) {
    lastSubmittedId = maxId;
  }

  return storageHash;
}

async function main() {
  const api = await Mangata.instance([process.env.MANGATA_NODE_URL!]).api();
  const abi = rolldownAbi.abi;

  // Ethereum private key
  // We need this to write to Mangata contract
  const account = privateKeyToAccount(`0x${process.env.WALLET_PRIVATE_KEY!}`);

  const transport = webSocket(process.env.ETH_CHAIN_URL, {
    retryCount: 5,
  });

  // We need wallet client in order to write to contract
  const walletClient = createWalletClient({
    account,
    transport,
  });

  // We need public client in order to read and subscribe to contract
  const publicClient = createPublicClient({
    transport,
    chain: getChain(),
  });
  (BigInt.prototype as any).toJSON = function() {
    return this.toString();
  };

  let unwatch: any;
  let inProgress = false;

  if (finalizationSource === "relay") {
    unwatch = await api.rpc.chain.subscribeFinalizedHeads(async (header) => {
      if (inProgress === false) {
        inProgress = true;
        console.log(`Chain is at block: #${header.number}`);
        const txHash = await sendUpdateToL1(api, walletClient, abi, header.hash);
        if (txHash) {
          const result = await publicClient.waitForTransactionReceipt({
            hash: txHash,
          });
          console.log(
            `#${result.blockNumber} ${result.transactionHash} : ${result.status}`,
          );
        }
        inProgress = false;
      } else {
        console.log(`Chain is at block: #${header.number} - tx pending`);
      }
    });
  } else {
    console.log("subscribing to eth events");
    unwatch = publicClient.watchContractEvent({
      address: eigenContractAddress,
      abi: eigenContractAbi.abi,
      eventName: "TaskCompleted",
      onLogs: async (logs) => {
        console.log("received task notification from L1");
        for (const log of logs) {
          const txHash = await sendUpdateToL1(
            api,
            walletClient,
            abi,
            (log as any).args.blockHash,
          );
          if (txHash) {
            const result = await publicClient.waitForTransactionReceipt({
              hash: txHash,
            });
            console.log(
              `#${result.blockNumber} ${result.transactionHash} : ${result.status}`,
            );
          }
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
  .then(() => {
    console.log("Success");
  })
  .catch((e) => console.error("Something went wrong", e));

