#!/bin/bash

# First import the deployer account into the foundry wallet (named here as "testnet-actual-0")
# Use as `RPC_URL=[InsertUrl] ./script/deploy/deploy-arb-sepolia.sh`
# Run it once without `--broadcast` to simulate execution
# Once sure - run with `--broadcast`

WALLET_NAME=testnet-actual-0
PREFIX=wss://arbitrum-sepolia.gateway.tenderly.co/
if [[ ! $RPC_URL =~ ^"$PREFIX" ]]; then
echo "RPC_URL bad target"
exit 1
fi

# cd to the directory of this script so that this can be run from anywhere
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
# At this point we are in tests/integration
cd "$parent_path" && cd ../../contracts
#

echo "executing script"

# ADD --broadcast to actually execute the script
ENV_SELECTOR=upgrade-rolldown-arbitrum-sepolia forge script script/MultiStage.s.sol --rpc-url $RPC_URL --account $WALLET_NAME -vvvvvvv

