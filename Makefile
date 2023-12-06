############################# HELP MESSAGE #############################
# Make sure the help command stays first, so that it's printed by default when `make` is called without arguments
.PHONY: help tests
help:
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

AGGREGATOR_ECDSA_PRIV_KEY=0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6
CHALLENGER_ECDSA_PRIV_KEY=0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a

CHAINID=31337
# Make sure to update this if the strategy address changes
# check in contracts/script/output/${CHAINID}/credible_squaring_avs_deployment_output.json
STRATEGY_ADDRESS=0x7a2088a1bFc9d81c55368AE168C2C02570cB814F
DEPLOYMENT_FILES_DIR=contracts/script/output/${CHAINID}

-----------------------------: ## 

___CONTRACTS___: ## 

deploy-eigenlayer-contracts-to-anvil-and-save-state: ## Deploy eigenlayer
	./tests/integration/deploy-eigenlayer-save-anvil-state.sh

deploy-shared-avs-contracts-to-anvil-and-save-state: ## Deploy blspubkeycompendium and blsstateoperatorretriever
	./tests/integration/deploy-shared-avs-contracts-save-anvil-state.sh

deploy-incredible-squaring-contracts-to-anvil-and-save-state: ## Deploy avs
	./tests/integration/deploy-avs-save-anvil-state.sh

deploy-all-to-anvil-and-save-state: deploy-eigenlayer-contracts-to-anvil-and-save-state deploy-shared-avs-contracts-to-anvil-and-save-state deploy-incredible-squaring-contracts-to-anvil-and-save-state ## deploy eigenlayer, shared avs contracts, and inc-sq contracts 

start-anvil-chain-with-el-and-avs-deployed: ## starts anvil from a saved state file (with el and avs contracts deployed)
	anvil --load-state tests/integration/avs-and-eigenlayer-deployed-anvil-state.json

bindings: ## generates contract bindings
	cd contracts && ./generate-go-bindings.sh

___DOCKER___: ## 
docker-build-and-publish-images: ## builds and publishes operator and aggregator docker images using Ko
	docker buildx build --load --platform linux/amd64,linux/arm64 -t mangatasolutions/aggregator:latest -f aggregator/Dockerfile .
	docker buildx build --load --platform linux/amd64,linux/arm64 -t mangatasolutions/operator:latest -f operator/Dockerfile .
	# KO_DOCKER_REPO=mangatasolutions/aggregator ko build --bare aggregator/cmd/main.go --platform=linux/arm64,linux/amd64
	# KO_DOCKER_REPO=mangatasolutions/operator ko build --bare operator/cmd/main.go --platform=linux/arm64,linux/amd64
docker-start-everything: docker-build-and-publish-images ## starts aggregator and operator docker containers
	docker compose pull && docker compose up

__CLI__: ## 

cli-setup-operator: send-fund cli-register-operator-with-eigenlayer cli-register-operator-bls-pubkeys cli-deposit-into-mocktoken-strategy cli-register-operator-with-avs ## registers operator with eigenlayer and avs

cli-register-operator-with-eigenlayer: ## registers operator with delegationManager
	go run cli/main.go --config config-files/operator.anvil.yaml register-operator-with-eigenlayer

cli-register-operator-bls-pubkeys: ## registers operator's bls public keys with blsPublicKeyCompendium
	go run cli/main.go --config config-files/operator.anvil.yaml register-operator-bls-pubkeys

cli-deposit-into-mocktoken-strategy: ## 
	go run cli/main.go --config config-files/operator.anvil.yaml deposit-into-strategy --strategy-addr ${STRATEGY_ADDRESS} --amount 100

cli-register-operator-with-avs: ## 
	go run cli/main.go --config config-files/operator.anvil.yaml register-operator-with-avs

cli-deregister-operator-with-avs: ## 
	go run cli/main.go --config config-files/operator.anvil.yaml deregister-operator-with-avs

cli-print-operator-status: ## 
	go run cli/main.go --config config-files/operator.anvil.yaml print-operator-status

send-fund: ## sends fund to the operator saved in tests/keys/test.ecdsa.key.json
	cast send 0x860B6912C2d0337ef05bbC89b0C2CB6CbAEAB4A5 --value 10ether --private-key 0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6

-----------------------------: ## 
# We pipe all zapper logs through https://github.com/maoueh/zap-pretty so make sure to install it
# TODO: piping to zap-pretty only works when zapper environment is set to production, unsure why
____OFFCHAIN_SOFTWARE___: ## 
start-aggregator: ## 
	go run aggregator/cmd/main.go --config config-files/aggregator.yaml \
		--credible-squaring-deployment ${DEPLOYMENT_FILES_DIR}/credible_squaring_avs_deployment_output.json \
		--shared-avs-contracts-deployment ${DEPLOYMENT_FILES_DIR}/shared_avs_contracts_deployment_output.json \
		--ecdsa-private-key ${AGGREGATOR_ECDSA_PRIV_KEY} \
		2>&1 | zap-pretty

start-operator: ## 
	go run operator/cmd/main.go --config config-files/operator.anvil.yaml \
		2>&1 | zap-pretty

start-challenger: ## 
	go run challenger/cmd/main.go --config config-files/challenger.yaml \
		--credible-squaring-deployment ${DEPLOYMENT_FILES_DIR}/credible_squaring_avs_deployment_output.json \
		--shared-avs-contracts-deployment ${DEPLOYMENT_FILES_DIR}/shared_avs_contracts_deployment_output.json \
		--ecdsa-private-key ${CHALLENGER_ECDSA_PRIV_KEY} \
		2>&1 | zap-pretty

run-plugin: ## 
	go run plugin/cmd/main.go --config config-files/operator.anvil.yaml
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

tests-integration: ## runs all integration tests
	go test ./tests/integration/... -v -count=1

