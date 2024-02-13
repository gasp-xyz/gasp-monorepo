############################# HELP MESSAGE #############################
# Make sure the help command stays first, so that it's printed by default when `make` is called without arguments
.PHONY: help tests
help:
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

CONTRACTS_REGEX="MangataServiceManager|MangataTaskManager|BLSPubkeyRegistry|BLSRegistryCoordinatorWithIndices|DelegationManager|BLSPublicKeyCompendium|Slasher|ERC20Mock|StrategyManager|IStrategy|StakeRegistry"
# CONTRACTS_REGEX=".+"

-----------------------------: ## 

.EXPORT_ALL_VARIABLES:

SKIP_WASM_BUILD=1

ETH_RPC_URL=http://localhost:8545
ETH_WS_URL=ws://localhost:8545
SUBSTRATE_RPC_URL=wss://kusama-archive.mangata.online:443
AVS_RPC_URL=http://localhost:8090
AVS_SERVER_IP_PORT_ADDRESS=localhost:8090

CHAIN_ID=31337
BLS_COMPENDIUM_ADDR=0xc5a5C42992dECbae36851359345FE25997F5C42d
BLS_OPERATOR_STATE_RETRIEVER_ADDR=0x67d269191c92Caf3cD7723F116c85e6E9bf55933
AVS_SERVICE_MANAGER_ADDR=0x9E545E3C0baAB3E08CdfD552C960A1050f373042
TESTNET=true

AVS_KICK_PERIOD=3

-----------------------------: ## 

___CONTRACTS___: ## 

deploy-eigenlayer-contracts-to-anvil-and-save-state: ## Deploy eigenlayer
	./tests/integration/deploy-eigenlayer-save-anvil-state.sh

deploy-shared-avs-contracts-to-anvil-and-save-state: ## Deploy blspubkeycompendium and blsstateoperatorretriever
	./tests/integration/deploy-shared-avs-contracts-save-anvil-state.sh

deploy-avs-contracts-to-anvil-and-save-state: ## Deploy avs
	./tests/integration/deploy-avs-save-anvil-state.sh

deploy-all-to-anvil-and-save-state: deploy-eigenlayer-contracts-to-anvil-and-save-state deploy-shared-avs-contracts-to-anvil-and-save-state deploy-avs-contracts-to-anvil-and-save-state ## deploy eigenlayer, shared avs contracts, and avs contracts 

start-anvil-chain-with-el-and-avs-deployed: ## starts anvil from a saved state file (with el and avs contracts deployed)
	anvil --load-state tests/integration/avs-and-eigenlayer-deployed-anvil-state.json

bindings-go: ## generates contract bindings
	cd contracts && ./generate-go-bindings.sh

bindings-rs: ## generates rust bindings
	forge bind --bindings-path ./mangata-finalizer/bindings --root ./contracts --crate-name bindings --overwrite --select ${CONTRACTS_REGEX} 


-----------------------------: ## 
# We pipe all zapper logs through https://github.com/maoueh/zap-pretty so make sure to install it
# TODO: piping to zap-pretty only works when zapper environment is set to production, unsure why
____OFFCHAIN_SOFTWARE___: ## 
start-aggregator: ##
	go run aggregator/cmd/main.go \
		--ecdsa-key-file tests/keys/aggregator.ecdsa.key.json \
		2>&1 | zap-pretty

# start-operator: ## 
# 	RUST_LOG=mangata_finalizer=debug cargo run --manifest-path=mangata-finalizer/Cargo.toml -- \
# 		--bls-key-file tests/keys/test.bls.key.json \
# 		--ecdsa-key-file tests/keys/test.ecdsa.key.json

start-operator: ## 
	RUST_LOG=mangata_finalizer=debug cargo run --manifest-path=mangata-finalizer/Cargo.toml -- \
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

