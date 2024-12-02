# Ferry deposit service
Service responsible for ferrying deposits from `L1` to `Gasp`.  

## Configuration
Service is configurable through env file supporting below parameters:
 * `MANGATA_NODE_URL` - L2 websocket address
 * `ETH_CHAIN_URL` -  L1 websocket address
 * `MANGATA_CONTRACT_ADDRESS` - Rolldown contract address
 * `MNEMONIC` - Polkadot account mnemonic or raw private key
 * `TOKENS_TO_TRACK` - List of tokens to track in json format. Input is expected to have following format: 
```
 '[["0xc351628EB244ec633d5f21fBD6621e1a683B1181", 100000000000000, 1]]'
      TOKEN ADDRESS                               MINIMUM PROFIT   WEIGHT

```
 * `L1_CHAIN` - `Ethereum` or `Arbitrum`
 * `TX_COST` - Estimated tx cost in gwei (to take into account when considering native token ferries)
 * `BLOCK_DELAY` - Delay against the latest block on L1 (reorg prevention)
 * `LOG` - log level as in `winston` (`error`,`info`, `debug`, `silly`)
