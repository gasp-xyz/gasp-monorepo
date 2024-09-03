#!/bin/bash

RPC_URL=http://localhost:8545
PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80

# cd to the directory of this script so that this can be run from anywhere
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
# At this point we are in tests/integration
cd "$parent_path"

# start an empty anvil chain in the background and dump its state to a json file upon exit
anvil --dump-state anvil-state.json &

############################################
# Deploy eigenlayer contracts
############################################
cd "$parent_path" && cd ../../contracts
#
ENV_SELECTOR=ethereum-stub forge script script/MultiStage.s.sol --rpc-url $RPC_URL --private-key $PRIVATE_KEY --broadcast -v

# kill anvil to save its state
pkill anvil
