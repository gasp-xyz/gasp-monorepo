#!/bin/bash

RPC_URL=http://localhost:8545
PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80

# cd to the directory of this script so that this can be run from anywhere
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
# At this point we are in tests/integration
cd "$parent_path"

anvil --chain-id 17000 &

echo "executing script"

forge script S1_U1MX_FinalizerTaskManager.s.sol --sig "run()" --rpc-url $RPC_URL --private-key $PRIVATE_KEY --broadcast -vvvvv

pkill anvil
