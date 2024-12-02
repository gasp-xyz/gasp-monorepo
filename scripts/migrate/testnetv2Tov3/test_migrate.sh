#!/bin/bash

source .env.example
yarn
yarn build

# REPO_ROOT=$(readlink -f $(dirname $(readlink -f $0))/../../..)
# NODE_BIN_PATH="$REPO_ROOT/../mangata_node"

# # TODO
# # We should make some alterations to the source node before executing
# $NODE_BIN_PATH/target/release/rollup-node --alith --tmp --force-authoring --ws-port=$SOURCE_URL &
# $NODE_BIN_PATH/target/release/rollup-node --alith --tmp --force-authoring --ws-port=$TARGET_URL &

yarn execute