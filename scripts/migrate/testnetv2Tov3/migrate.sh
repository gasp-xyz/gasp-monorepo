#!/bin/bash

set -o errexit -o nounset -o pipefail

source .env
yarn
yarn build

yarn execute