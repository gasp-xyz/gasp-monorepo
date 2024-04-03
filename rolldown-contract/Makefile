deploy:
	forge script script/rolldown.s.sol:MyScript --fork-url http://localhost:8545 --broadcast -vvvv
build:
	forge build
format:
	npx prettier -c .prettierrc.json --write --plugin=prettier-plugin-solidity './src/*.sol'
	npx prettier -c .prettierrc.json --write --plugin=prettier-plugin-solidity './test/*.sol'
update-abi: build
	cp out/rolldown.sol/RollDown.json ../rollup-sequencer/src
	cp out/rolldown.sol/RollDown.json ../rollup-updater/src
verify:
	docker run --network=host -it mangatasolutions/rolldown-contract:local verify-contract --verifier blockscout --verifier-url http://127.0.0.1/api 0x5f3f1dBD7B74C6B46e8c44f98792A1dAf8d69154 RollDown
