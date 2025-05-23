#!/bin/bash
REPO_ROOT=$(dirname $(readlink -f $0))/../

rm -rf ./benchmarks
mkdir ./benchmarks

benchmarks=(
    "frame_system"
    "pallet_session"
    "pallet_timestamp"
    "orml_tokens"
    "parachain_staking"
    "pallet_xyk"
    "orml_asset_registry"
    "pallet_treasury"
    "pallet_collective_mangata"
    "pallet_crowdloan_rewards"
    "pallet_utility_mangata"
    "pallet_vesting_mangata"
    "pallet_issuance"
    "pallet_bootstrap"
    "pallet_multipurpose_liquidity"
    "pallet_fee_lock"
    "pallet_proof_of_stake"
    "pallet_rolldown"
    "pallet_market"
)

for bench in ${benchmarks[@]}; do
    ${REPO_ROOT}/scripts/run_benchmark.sh $bench
done

# ${REPO_ROOT}/scripts/run_benchmark_overhead.sh
