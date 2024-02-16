#!/usr/bin/env node

import { Web3, Contract } from 'web3';
import { Keyring, ApiPromise, WsProvider } from '@polkadot/api';
import _yargs from 'yargs';
import { hideBin } from 'yargs/helpers';
const yargs = _yargs(hideBin(process.argv));

import * as fs from 'fs';


function normalize(obj) {
  if (Array.isArray(obj)) {
    return obj.map(normalize);
  } else if (typeof obj === 'object') {
    const newObj = {};
    for (const key in obj) {
      if (!Number.isNaN(Number(key))) {
        continue; // Skip numeric indexes
      }

      if (key === "__length__") {
        continue;
      }
      newObj[key] = normalize(obj[key]);
    }
    return newObj;
  } else {
    return obj;
  }
}

const main = async () => {
  const cli = yargs
    .usage("Usage: $0 -l1 ws://localhost:8545 -l2 ws://127.0.0.1:9946 -a /path/to/contract/abi -type [sequencer|updater]")
    .option("a", {
      alias: "abi",
      demandOption: "The uri is required.",
      type: "string",
      nargs: 1,
      default: "/home/dev/mangata-eth/out/rolldown.sol/RollDown.json",
    })
    .option("l2", {
      alias: "l2-uri",
      demandOption: "The uri is required.",
      type: "string",
      nargs: 1,
      default: "ws://127.0.0.1:9946",
    })
    .option("l1", {
      alias: "l1-uri",
      type: "string",
      demandOption: "The uri is required.",
      nargs: 1,
      default: "ws://localhost:8545",
    })
    .option("seq", {
      type: "boolean",
      describe: "Send L1 update to L2",
      nargs: 0,
    })
    .option("up", {
      type: "boolean",
      describe: "Send L2 update to L1",
      nargs: 0,
    })
    .describe("help", "Show help.")

  const { l1, l2, seq, up } = cli.argv;

  let abi = JSON.parse(fs.readFileSync('/home/dev/mangata-eth/out/rolldown.sol/RollDown.json', 'utf8'));
  let web3 = new Web3(l2);
  const api = new ApiPromise(
    {
      provider: new WsProvider(l1),
      rpc: {
        rolldown: {
          pending_updates: {
            description: "",
            params: [
              {
                name: 'at',
                type: 'Hash',
                isOptional: true
              }
            ],
            type: "Vec<u8>"
          }
        }
      },
      types: {
        ShufflingSeed: {
          seed: "H256",
          proof: "H512"
        },
        Header: {
          parentHash: "Hash",
          number: "Compact<BlockNumber>",
          stateRoot: "Hash",
          extrinsicsRoot: "Hash",
          digest: "Digest",
          seed: "ShufflingSeed",
          count: "BlockNumber"
        },
      }
    }
  );
  await api.isReady;
  let keyring = new Keyring({ type: 'sr25519' });
  let alice = keyring.createFromUri("//Alice");

  const contract = new Contract(abi.abi, "0x5FbDB2315678afecb367f032d93F642f64180aa3");
  contract.setProvider(web3.currentProvider);

  let requests = await contract.methods.getUpdateForL2.call().call();
  requests.order = requests.order.map((e) => {
    switch (e) {
      case 1n: {
        return "WITHDRAWAL";
      }
      case 0n: {
        return "DEPOSIT";
      }
      case 2n: {
        return "CANCEL_RESOLUTION";
      }
      case 3n: {
        return "L2_UPDATES_TO_REMOVE";
      }
    }
  });


  let blockHash = await api.rpc.chain.getBlockHash();
  let pending_updates = await api.rpc.rolldown.pending_updates(blockHash);
  const updates = web3.eth.abi.decodeParameters(
    abi.abi.find((e) => e.name === "update_l1_from_l2").inputs,
    pending_updates.toHex()
  );

  BigInt.prototype.toJSON = function() { return this.toString() }

  console.log("##########################################")
  console.log("            requests on l1                ")
  console.log("##########################################")
  console.log(normalize(requests))

  console.log()
  console.log("##########################################")
  console.log("            updates on l2                 ")
  console.log("##########################################")
  console.log(JSON.stringify(normalize(updates), null, 2))


  console.log()
  console.log()
  if (seq) {
    console.log("acting as sequencer")
    let tx = api.tx.rolldown.updateL2FromL1(requests)
    await tx.signAndSend(alice);
  }

  if (up) {
    console.log("acting as updater")
    const privateKey = '0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80';
    const account = web3.eth.accounts.wallet.add(privateKey).get(0); // contract.methods.setValue(newValue).send
    let final = await contract.methods.update_l1_from_l2(updates.inputArray).send({ from: account.address });
    console.log(final);
  }

  process.exit(0);
}

await main();
