import 'dotenv/config';
import { createPublicClient, createWalletClient, webSocket } from 'viem';
import { privateKeyToAccount } from 'viem/accounts';
import { goerli } from 'viem/chains';
import { Mangata } from '@mangata-finance/sdk';

import { eigenContractAbi } from './eigenAbi.js';

type ContractAddress = `0x${string}`;

const eigenContractAddress = process.env
  .EIGEN_CONTRACT_ADDRESS! as ContractAddress;
const mangataContractAddress = process.env
  .MANGATA_CONTRACT_ADDRESS! as ContractAddress;

async function main() {
  const api = await Mangata.instance([process.env.MANGATA_URL!]).api();
  console.log('api', api.isConnected);

  // Ethereum private key
  // We need this to write to Mangata contract
  const account = privateKeyToAccount(`0x${process.env.WALLET_PRIVATE_KEY!}`);

  const transport = webSocket(process.env.ETH_CHAIN_URL, {
    retryCount: 5
  });

  // We need wallet client in order to write to contract
  const walletClient = createWalletClient({
    account,
    transport
  });

  // We need public client in order to read and subscribe to contract
  const publicClient = createPublicClient({
    transport
  });

  // TODO: Will be implemented. This is if the server is down or restarted
  // TODO: to fetch all past events from certain block number when server was down to the latest block
  // TODO: and process all the updates and then start listening for new events
  // eslint-disable-next-line no-constant-condition
  if (true) {
    const pastEvents = await publicClient.getContractEvents({
      address: eigenContractAddress,
      abi: eigenContractAbi,
      eventName: 'TaskResponded',
      fromBlock: BigInt(0), // TODO: This will be changed based on the failed block
      toBlock: 'latest'
    });

    // TODO: do something with those events
    for (const pastEvent of pastEvents) {
      console.log('PastEvent', pastEvent.blockNumber);
      console.log('PastEvent', pastEvent.transactionIndex);
    }
  }

  const unwatch = publicClient.watchContractEvent({
    address: eigenContractAddress,
    abi: eigenContractAbi,
    eventName: 'TaskResponded',
    onLogs: async (logs) => {
      for (const log of logs) {
        console.log('Watch block number', log.blockNumber);
        console.log('Index Tx', log.transactionIndex);

        // TODO: Executes TX on ETH with all pending_updates with hashes
        // TODO: Here we need to write to mangata contract
        const storageHash = await walletClient.writeContract({
          chain: goerli, // TODO: this needs the chain in order to work properly
          abi: eigenContractAbi, // TODO: Mangata contract ABI
          address: mangataContractAddress,
          functionName: 'pause', // TODO: Function name will be from Mangata contract
          args: [BigInt(123666666666)]
        });

        await publicClient.waitForTransactionReceipt({ hash: storageHash });
      }
    }
  });

  // We need to  unwatch if server is down or restarted to prevent leaks
  process.on('SIGINT', async () => {
    console.log('Stopping the process..., SIGINT signal');
    unwatch(); // Unsubscribe from event watching
    process.exit();
  });

  process.on('SIGTERM', async () => {
    console.log('Stopping the process..., SIGTERM signal');
    unwatch(); // Unsubscribe from event watching
    process.exit();
  });

  process.on('SIGHUP', async () => {
    console.log('Stopping the process..., SIGHUP signal');
    unwatch(); // Unsubscribe from event watching
    process.exit();
  });
}

main()
  .then(() => {
    console.log('Success');
  })
  .catch((e) => console.error('Something went wrong', e));
