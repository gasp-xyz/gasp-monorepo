#!/bin/bash

RPC_URL=http://localhost:8545
PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80

# cd to the directory of this script so that this can be run from anywhere
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
# At this point we are in tests/integration
cd "$parent_path"

# start an empty anvil chain in the background and dump its state to a json file upon exit
anvil --dump-state eigenlayer-deployed-anvil-state.json &

cd ../../contracts/lib/eigenlayer-middleware/lib/eigenlayer-contracts
# deployment overwrites this file, so we save it as backup, because we want that output in our local files, and not in the eigenlayer-contracts submodule files
mv script/output/M2_from_scratch_deployment_data.json script/output/M2_from_scratch_deployment_data.json.bak
# M2_Deploy_From_Scratch.s.sol prepends "script/testing/" to the configFile passed as input (M2_deploy_from_scratch.anvil.config.json)
forge script script/testing/M2_Deploy_From_Scratch.s.sol --rpc-url $RPC_URL --private-key $PRIVATE_KEY --broadcast --sig "run(string memory configFile)" -- M2_deploy_from_scratch.anvil.config.json
mv script/output/M2_from_scratch_deployment_data.json ../../../../script/input/31337/eigenlayer_deployment_output.json
mv script/output/M2_from_scratch_deployment_data.json.bak script/output/M2_from_scratch_deployment_data.json

# create strategy and deposit
cast send 0x860B6912C2d0337ef05bbC89b0C2CB6CbAEAB4A5 --value 10ether --private-key $PRIVATE_KEY

cd "$parent_path"
cd ../../contracts
forge script script/0_AnvilSetup.s.sol --rpc-url $RPC_URL --private-key $PRIVATE_KEY --broadcast -v

# kill anvil to save its state
pkill anvil
