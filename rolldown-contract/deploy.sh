#!/bin/bash -e

ACCOUNT=0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
PRIV=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80

forge build
forge create RollDown --private-key $PRIV | tee /tmp/rolldown.txt
forge create TestToken --private-key $PRIV --constructor-args TestToken TST | tee /tmp/token.txt

ROLLDOWN_CONTRACT=$(grep "Deployed to" /tmp/rolldown.txt | cut -d ":" -f 2 | tr -d " ")
TOKEN_CONTRACT=$(grep "Deployed to" /tmp/token.txt | cut -d ":" -f 2 | tr -d " ")

echo
echo "RollDown: $ROLLDOWN_CONTRACT"
echo "TestToken: $TOKEN_CONTRACT"

cast send $ROLLDOWN_CONTRACT "deposit(address, uint256)" "$TOKEN_CONTRACT" "200" --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 >/dev/null
cast send $ROLLDOWN_CONTRACT "deposit(address, uint256)" "$TOKEN_CONTRACT" "300" --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 >/dev/null
cast send $ROLLDOWN_CONTRACT "deposit(address, uint256)" "$TOKEN_CONTRACT" "300" --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 >/dev/null
cast send $ROLLDOWN_CONTRACT "deposit(address, uint256)" "$TOKEN_CONTRACT" "300" --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 >/dev/null
cast send $ROLLDOWN_CONTRACT "deposit(address, uint256)" "$TOKEN_CONTRACT" "300" --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 >/dev/null
cast send $ROLLDOWN_CONTRACT "deposit(address, uint256)" "$TOKEN_CONTRACT" "300" --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 >/dev/null
cast send $ROLLDOWN_CONTRACT "deposit(address, uint256)" "$TOKEN_CONTRACT" "300" --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 >/dev/null

cast send $ROLLDOWN_CONTRACT "withdraw(address, uint256)" "$TOKEN_CONTRACT" "100" --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 >/dev/null

echo
echo "FINAL"
echo
RESULT=$(cast call $ROLLDOWN_CONTRACT "getUpdateForL2()" "$ROLLDOWN_CONTRACT")
echo $RESULT
# cast abi-decode "getUpdateForL2()" $RESULT
