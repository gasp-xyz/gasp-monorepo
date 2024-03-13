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

