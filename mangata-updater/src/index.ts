import { Mangata } from '@mangata-finance/sdk';
import { createPublicClient, http } from 'viem';
import { goerli } from 'viem/chains';

async function main() {
  const mangata = Mangata.instance(['wss://kusama-archive.mangata.online']);
  const api = await mangata.api();
  console.log('api', api.isConnected);

  const client = createPublicClient({
    chain: goerli,
    transport: http()
  });

  const block = await client.getBlock();
  console.log('block', block);
}

main()
  .then(() => {
    console.log('Success');
    process.exit(0);
  })
  .catch(() => console.error('Something went wrong'));
