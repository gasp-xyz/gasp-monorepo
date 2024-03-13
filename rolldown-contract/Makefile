deploy:
	forge script script/rolldown.s.sol:MyScript --fork-url http://localhost:8545 --broadcast -vvvv
format:
	npx prettier -c .prettierrc.json --write --plugin=prettier-plugin-solidity './src/*.sol'
	npx prettier -c .prettierrc.json --write --plugin=prettier-plugin-solidity './test/*.sol'
