# Withdrawal ferry service
Service responsible for ferrying withdrawals from `GASP` to `L1`. To speed up the withdrawal time. While it make require significant amount of time for withdrawal to go through *EigenLayer* and be validated ferry service aims to lend tokens to the users and claim tokens back from the rolldown contract whenever particular withdrawal is ready to be closed. Ferry find all ferryable withdrawals by checking state of L1 & L2 blockchains, then rates them based on `TOKENS_TO_TRACK` taking into account available balance, expected minimum profit and assigned weight. 

## Configuration
Service is configurable through env file supporting below parameters:
 * `MANGATA_NODE_URL` - L2 websocket address
 * `ETH_CHAIN_URL` -  L1 websocket address
 * `MANGATA_CONTRACT_ADDRESS` - Rolldown contract address
 * `PRIVATE_KEY` - Ethereum private key(used to sign ferry txs)
 * `TOKENS_TO_TRACK` - List of tokens to track in json format. Input is expected to have following format: 
```
 '[["0xFD471836031dc5108809D173A067e8486B9047A3", 100000000000000, 1]]'
      TOKEN ADDRESS                               MINIMUM PROFIT   WEIGHT

```
 * `L1_CHAIN` - `Ethereum` or `Arbitrum`
 * `TX_COST` - Estimated tx cost in gwei (to take into account when considering native token ferries)
 * `LOOK_BACK_HOURS` - period of time in past that ferry should be looking for past withdrawals that might be ready to close
 * `LOG` - log level as in `winston` (`error`,`info`, `debug`, `silly`)

# Closer service
Service responsible for closing txs that went through eigen layer and are able to be closed. Ferry tip is transfered as incentivication to whoever is closing the withdrawal.

## Configuration
Service is configurable through env file supporting below parameters:
 * `MANGATA_NODE_URL` - L2 websocket address
 * `ETH_CHAIN_URL` -  L1 websocket address
 * `MANGATA_CONTRACT_ADDRESS` - Rolldown contract address
 * `PRIVATE_KEY` - Ethereum private key(used to sign closeWithdrawal contract calls)
 * `TOKENS_TO_TRACK` - List of tokens to track in json format. Input is expected to have following format: 
```
 '[["0xFD471836031dc5108809D173A067e8486B9047A3", 100000000000000, 1]]'
      TOKEN ADDRESS                               MINIMUM PROFIT   WEIGHT
```
 * `L1_CHAIN` - `Ethereum` or `Arbitrum`
 * `TX_COST` - Estimated tx cost in gwei (to take into account when considering native token ferries)
 * `LOOK_BACK_HOURS` - in case of Closer service should be set to 0 (unused)
 * `LOG` - log level as in `winston` (`error`,`info`, `debug`, `silly`)
