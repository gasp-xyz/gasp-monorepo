#!/bin/bash
~/.cargo/bin/subxt codegen --attribute "#[allow(non_snake_case)]" --derive Clone --derive PartialEq --file metadata.scale | rustfmt --edition=2018 --emit=stdout >src/l2/gasp/gasp_bindings.rs
