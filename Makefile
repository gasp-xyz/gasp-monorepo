############################# HELP MESSAGE #############################
# Make sure the help command stays first, so that it's printed by default when `make` is called without arguments
.PHONY: help tests
help:
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

CONTRACTS_REGEX="FinalizerServiceManager|FinalizerTaskManager|AVSDirectory|BLSApkRegistry|RegistryCoordinator|DelegationManager|ERC20Mock|StrategyManager|IStrategy|StakeRegistry|GaspMultiRollupService"
# CONTRACTS_REGEX=".+"

-----------------------------: ## 

.EXPORT_ALL_VARIABLES:

SKIP_WASM_BUILD=1

ETH_RPC_URL=http://localhost:8545
ETH_WS_URL=ws://localhost:8545
SUBSTRATE_RPC_URL=wss://rollup-holesky-rpc.gasp.xyz:443
AVS_RPC_URL=http://localhost:8090
AVS_SERVER_IP_PORT_ADDRESS=localhost:8090

CHAIN_ID=31337
AVS_REGISTRY_COORDINATOR_ADDR=0x851356ae760d987E095750cCeb3bC6014560891C

AVS_BLOCK_VALIDATION_PERIOD=10
AVS_KICK_PERIOD=5
AVS_UPDATE_STAKE_PERIOD=10

-----------------------------: ## 

___CONTRACTS___: ## 

deploy-all-contracts-via-multistage-and-resume: ## Deploy evth with anvil running
	./tests/integration/deploy-all-contracts-via-multistage-and-resume.sh

deploy-eigenlayer-contracts-to-anvil-and-save-state: ## Deploy eigenlayer
	./tests/integration/deploy-eigenlayer-save-anvil-state.sh

deploy-avs-contracts-to-anvil-and-save-state: ## Deploy avs
	./tests/integration/deploy-avs-save-anvil-state.sh

deploy-all-to-anvil-and-save-state: deploy-eigenlayer-contracts-to-anvil-and-save-state deploy-avs-contracts-to-anvil-and-save-state ## deploy eigenlayer and avs contracts 

deploy-alt-l1-contracts-and-save-state:
	./tests/integration/deploy-alt-l1-contracts-and-save-anvil-state.sh

deploy-all-contracts-and-save-state:
	./tests/integration/deploy-all-contracts-and-save-anvil-state.sh

start-anvil-chain-with-el-and-avs-deployed: ## starts anvil from a saved state file (with el and avs contracts deployed)
	anvil --load-state tests/integration/avs-and-eigenlayer-deployed-anvil-state.json

start-anvil-and-deploy: ## starts anvil and deploys EL & avs contracts
	./tests/integration/deploy-all-and-resume.sh

start-anvil-deploy-and-verify: ## starts anvil, deploys EL & avs contracts, and verifies them on local blockscout, check ./tests/integration/readme for more info
	./tests/integration/deploy-all-verify-and-resume.sh

bindings-go: ## generates contract bindings
	cd contracts && ./generate-go-bindings.sh

bindings-rs: ## generates rust bindings
	forge bind --bindings-path ./gasp-avs/bindings --root ./contracts --crate-name bindings --overwrite --select ${CONTRACTS_REGEX}
	cd ./gasp-avs && cargo fmt
	cd ./updater && cargo fmt
	cp -rf ./gasp-avs/bindings ./updater/

bindings-rs-alloy: ## generates rust alloy bindings
	forge --version | grep "forge 0.3.0"
	forge bind --alloy --bindings-path ./sequencer/bindings --root ./contracts --crate-name bindings --overwrite  --select 'Rolldown$$' --select 'RolldownPrimitives$$'
	cd ./sequencer/bindings && cargo fmt

bindings-json: ## generate JS bindings
	forge build --root ./contracts 
	cp contracts/out/Rolldown.sol/Rolldown.json ./ferry-deposit//src/Rolldown.json
	cp contracts/out/Rolldown.sol/Rolldown.json ./ferry-withdrawal/src/Rolldown.json

bindings-gasp:
	rm -rf metadata.scale || true
	rm -rf sequencer/sequencer/src/l2/gasp/gasp_bindings.rs || true
	subxt metadata -f bytes -o metadata.scale --url http://127.0.0.1:9944
	echo "#[allow(non_snake_case)]" >> sequencer/sequencer/src/l2/gasp/gasp_bindings.rs
	subxt codegen --attribute "#[allow(non_snake_case)]" --derive Clone --derive PartialEq --file metadata.scale | rustfmt --edition=2018 --emit=stdout >> sequencer/sequencer/src/l2/gasp/gasp_bindings.rs
	rm -rf metadata.scale


bindings: bindings-go bindings-rs bindings-json bindings-rs-alloy## generate all bindings

-----------------------------: ## 
# We pipe all zapper logs through https://github.com/maoueh/zap-pretty so make sure to install it
# TODO: piping to zap-pretty only works when zapper environment is set to production, unsure why
____OFFCHAIN_SOFTWARE___: ## 
start-avs-aggregator: ##
	cast rpc evm_mine && \
	go run avs-aggregator/cmd/main.go \
		--ecdsa-key-file tests/keys/aggregator.ecdsa.key.json \
		2>&1 | zap-pretty

start-gasp-avs: ## 
	RUST_LOG=gasp_avs=debug cargo run --manifest-path=gasp-avs/Cargo.toml -- \
		--ecdsa-ephemeral-key \
		--bls-ephemeral-key \
		--testnet \
		--stake 50

start-gasp-avs-testkeys: ## 
	RUST_LOG=gasp_avs=debug cargo run --manifest-path=gasp-avs/Cargo.toml -- \
		--ecdsa-key-file tests/keys/test.ecdsa.key.json \
		--bls-key-file tests/keys/test.bls.key.json \
		--opt-in-at-startup

start-blockscout: ## starts blockscout
	cd ops/blockscout-docker-compose && docker-compose up -d

stop-blockscout: ## stops blockscout
	cd ops/blockscout-docker-compose && docker-compose down -v

restart-blockscout: ## restarts blockscout (useful when you want to reset the database)
	cd ops/blockscout-docker-compose && docker-compose down -v && docker-compose up -d
-----------------------------: ## 
_____HELPER_____: ## 
mocks: ## generates mocks for tests
	go install go.uber.org/mock/mockgen@v0.3.0
	go generate ./...

tests-unit: ## runs all unit tests
	go test $$(go list ./... | grep -v /integration) -coverprofile=coverage.out -covermode=atomic --timeout 15s
	go tool cover -html=coverage.out -o coverage.html

tests-contract: ## runs all forge tests
	cd contracts && forge test
