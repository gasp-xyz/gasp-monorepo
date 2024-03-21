############################# HELP MESSAGE #############################
# Make sure the help command stays first, so that it's printed by default when `make` is called without arguments
.PHONY: help tests
help:
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

CONTRACTS_REGEX="FinalizerServiceManager|FinalizerTaskManager|AVSDirectory|BLSApkRegistry|RegistryCoordinator|DelegationManager|ERC20Mock|StrategyManager|IStrategy|StakeRegistry"
# CONTRACTS_REGEX=".+"

-----------------------------: ## 

.EXPORT_ALL_VARIABLES:

SKIP_WASM_BUILD=1

ETH_RPC_URL=http://localhost:8545
ETH_WS_URL=ws://localhost:8545
SUBSTRATE_RPC_URL=wss://collator-01-ws-rollup-testnet.mangata.online:443
AVS_RPC_URL=http://localhost:8090
AVS_SERVER_IP_PORT_ADDRESS=localhost:8090

CHAIN_ID=31337
AVS_REGISTRY_COORDINATOR_ADDR=0xa82fF9aFd8f496c3d6ac40E2a0F282E47488CFc9
TESTNET=true

AVS_KICK_PERIOD=5
AVS_UPDATE_STAKE_PERIOD=10

-----------------------------: ## 

___CONTRACTS___: ## 

deploy-eigenlayer-contracts-to-anvil-and-save-state: ## Deploy eigenlayer
	./tests/integration/deploy-eigenlayer-save-anvil-state.sh

deploy-avs-contracts-to-anvil-and-save-state: ## Deploy avs
	./tests/integration/deploy-avs-save-anvil-state.sh

deploy-all-to-anvil-and-save-state: deploy-eigenlayer-contracts-to-anvil-and-save-state deploy-avs-contracts-to-anvil-and-save-state ## deploy eigenlayer and avs contracts 

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
	forge bind --bindings-path ./avs-finalizer/bindings --root ./contracts --crate-name bindings --overwrite --select ${CONTRACTS_REGEX}
	cd ./avs-finalizer && cargo fmt

bindings-json: ## generate JS bindings
	cd ./rolldown-contract && make update-abi
	cd ./contracts && forge build && cp out/FinalizerTaskManager.sol/FinalizerTaskManager.json ../rollup-updater/src/FinalizerTaskManager.json

bindings: bindings-go bindings-rs bindings-json ## generate all bindings

-----------------------------: ## 
# We pipe all zapper logs through https://github.com/maoueh/zap-pretty so make sure to install it
# TODO: piping to zap-pretty only works when zapper environment is set to production, unsure why
____OFFCHAIN_SOFTWARE___: ## 
start-avs-aggregator: ##
	go run avs-aggregator/cmd/main.go \
		--ecdsa-key-file tests/keys/aggregator.ecdsa.key.json \
		2>&1 | zap-pretty

# start-operator: ## 
#	RUST_LOG=avs_finalizer=debug cargo run --manifest-path=avs-finalizer/Cargo.toml -- \
# 		--bls-key-file tests/keys/test.bls.key.json \
# 		--ecdsa-key-file tests/keys/test.ecdsa.key.json

start-avs-finalizer: ## 
	RUST_LOG=avs_finalizer=debug cargo run --manifest-path=avs-finalizer/Cargo.toml -- \
		--ecdsa-ephemeral-key \
		--bls-ephemeral-key \
		--stake 100

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

