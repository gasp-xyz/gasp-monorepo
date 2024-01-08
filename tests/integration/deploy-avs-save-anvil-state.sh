#!/bin/bash

RPC_URL=http://localhost:8545
PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80

# cd to the directory of this script so that this can be run from anywhere
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
cd "$parent_path"

# start an anvil instance in the background that has eigenlayer contracts deployed
anvil --load-state eigenlayer-and-shared-avs-contracts-deployed-anvil-state.json --dump-state avs-and-eigenlayer-deployed-anvil-state.json &
cd ../../contracts
forge script script/1_MangataAvsDeployer.s.sol:Deployer --rpc-url $RPC_URL --private-key $PRIVATE_KEY --broadcast -v
# kill anvil to save its state
pkill anvil
