### Sequencer

This service is responsible for transferring data from L1 to the GASP node. For each new L2 block, a series of actions are performed in the following order:

- Check for malicious updates (if cancel rights are available).
- Check for new updates waiting on the L1 contract and submit them to L2 (if read rights are available).
- Check for any cancel disputes involving the sequencer and close them on L1 (if the account balance on L1 is greater than the configurable `TX_COST`).

### Configuration

The service is configurable through environment variables:

- `ETH_CHAIN_URL` - EVM RPC endpoint.
- `PRIVATE_KEY` - Private key of the EVM account used for closing cancel disputes.
- `MANGATA_NODE_URL` - GASP node endpoint.
- `MNEMONIC` - Mnemonic or private key of the GASP account used to submit updates.
- `L1_CHAIN` - Specifies the L1 chain: `ethereum`, `arbitrum`, or `base`.
- `MANGATA_CONTRACT_ADDRESS` - Address of the `ROLLDOWN` contract on the EVM chain.
- `LIMIT` - Maximum number of updates that can be submitted in a single block (default is 10k).
- `WATCHDOG` - Time in seconds that the watchdog will wait for a new block. If no new block is found, the watchdog will restart the service.
- `TX_COST` - Approximate cost (in gwei) for submitting a transaction to the L1 chain. This value is used to check if there are enough funds to close the dispute. **If set to `0`, the service will not close disputes.**
- `RUST_LOG` - Configurable log level.

