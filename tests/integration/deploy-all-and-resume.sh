#!/bin/bash

RPC_URL=http://localhost:8545
PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80

# cd to the directory of this script so that this can be run from anywhere
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
# At this point we are in tests/integration
cd "$parent_path"

# start an empty anvil chain in the background
anvil &

cd ../../contracts
# M2_Deploy_From_Scratch.s.sol prepends "script/" to the configFile passed as input (M2_deploy_from_scratch.anvil.config.json)
forge script script/M2_Deploy_From_Scratch.s.sol \
    --rpc-url $RPC_URL \
    --private-key $PRIVATE_KEY \
    --broadcast \
    --sig "run(string memory configFile)" \
    -- M2_deploy_from_scratch.anvil.config.json

mv script/output/M2_from_scratch_deployment_data.json script/input/31337/eigenlayer_deployment_output.json

# create erc mock token & strategy in eigen
forge script script/0_AnvilSetup.s.sol \
    --rpc-url $RPC_URL \
    --private-key $PRIVATE_KEY \
    --broadcast -v

# deploy AVS contracts
forge script script/1_FinalizerAvsDeployer.s.sol:Deployer \
    --rpc-url $RPC_URL \
    --private-key $PRIVATE_KEY \
    --broadcast -v

# pkill anvil