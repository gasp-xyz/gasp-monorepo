#!/bin/bash

# First import the deployer account into the foundry wallet (named here as "mainnet-actual-0")
# Use as `RPC_URL=[InsertUrl] ETHERSCAN_API_KEY=[InsertApiKey] [InsertScriptPath]`
# Run it once without `--broadcast` to simulate execution
# Once sure - run with `--broadcast`

# To upgrade the proxy via L1 council
# Use forge cast calldata and target to the proxyAdmin address

ANVIL_DEFAULT_ACCOUNT=anvil-default-0 # cast wallet import anvil-default-0 --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
WALLET_NAME=mainnet-actual-0
PREFIX=wss://holesky.gateway.tenderly.co/
if [[ ! $RPC_URL =~ ^"$PREFIX" ]]; then
echo "RPC_URL bad target"
exit 1
fi

# cd to the directory of this script so that this can be run from anywhere
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
# At this point we are in tests/integration
cd "$parent_path"
#

echo "executing script"

# ADD --broadcast to actually execute the script (if using without broadcast use ANVIL_DEFAULT_ACCOUNT instead of WALLET_NAME)
# !!!!!!!!!!
# Add --resume when re-running after it has failed
forge script S1_U1MX_FinalizerTaskManager.s.sol --sig "run()" --rpc-url $RPC_URL --account $WALLET_NAME --broadcast -vvvvv --verify --etherscan-api-key $ETHERSCAN_API_KEY

# forge script S1_U1MX_FinalizerTaskManager.s.sol --sig "verify()" --rpc-url $RPC_URL --account $WALLET_NAME -vvvvv
