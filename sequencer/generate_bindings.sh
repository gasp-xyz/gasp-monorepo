#!/bin/bash

echo "fetching metadata"
~/.cargo/bin/subxt metadata -f bytes -o metadata.scale --url http://127.0.0.1:9944
~/.cargo/bin/subxt codegen --attribute "#[allow(non_snake_case)]" --derive Clone --derive PartialEq --file metadata.scale | rustfmt --edition=2018 --emit=stdout >sequencer/src/l2/gasp/gasp_bindings.rs
