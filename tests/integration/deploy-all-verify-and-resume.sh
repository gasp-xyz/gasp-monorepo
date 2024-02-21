#!/bin/bash

RPC_URL=http://localhost:8545
PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80

# cd to the directory of this script so that this can be run from anywhere
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
# At this point we are in tests/integration
cd "$parent_path"

# start an empty anvil chain in the background
anvil &

cd ../../contracts/lib/eigenlayer-middleware/lib/eigenlayer-contracts
# deployment overwrites this file, so we save it as backup, because we want that output in our local files, and not in the eigenlayer-contracts submodule files
mv script/output/M2_from_scratch_deployment_data.json script/output/M2_from_scratch_deployment_data.json.bak
# M2_Deploy_From_Scratch.s.sol prepends "script/testing/" to the configFile passed as input (M2_deploy_from_scratch.anvil.config.json)
forge script script/testing/M2_Deploy_From_Scratch.s.sol \
    --rpc-url $RPC_URL \
    --private-key $PRIVATE_KEY \
    --broadcast \
    --verify \
    --verifier blockscout \
    --verifier-url http://localhost:80/api \
    --sig "run(string memory configFile)" \
    -- M2_deploy_from_scratch.anvil.config.json

mv script/output/M2_from_scratch_deployment_data.json ../../../../script/input/31337/eigenlayer_deployment_output.json
mv script/output/M2_from_scratch_deployment_data.json.bak script/output/M2_from_scratch_deployment_data.json

# create erc mock token & strategy in eigen
cd "$parent_path"
cd ../../contracts
forge script script/0_AnvilSetup.s.sol \
    --rpc-url $RPC_URL \
    --private-key $PRIVATE_KEY \
    --broadcast -v \
    --verify \
    --verifier blockscout \
    --verifier-url http://localhost:80/api

# deploy AVS contracts
forge script script/1_FinalizerAvsDeployer.s.sol:Deployer \
    --rpc-url $RPC_URL \
    --private-key $PRIVATE_KEY \
    --broadcast -v  \
    --verify \
    --verifier blockscout \
    --verifier-url http://localhost:80/api

# pkill anvil
