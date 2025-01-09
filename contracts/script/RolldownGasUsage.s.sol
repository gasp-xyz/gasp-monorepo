// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

import {ProxyAdmin} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import {TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {console} from "forge-std/console.sol";
import {Script} from "forge-std/Script.sol";
import {IRolldown} from "../src/IRolldown.sol";
import {IRolldownPrimitives} from "../src/IRolldownPrimitives.sol";
import {Rolldown} from "../src/Rolldown.sol";

contract RolldownGasUsage is Script {
    struct Users {
        address admin;
        address depositor;
        address updater;
        address withdrawer;
        address recipient;
    }

    Users public users;
    Rolldown public rolldown;

    IRolldownPrimitives.Range private _range = IRolldownPrimitives.Range(1, 2);
    IRolldownPrimitives.Withdrawal private _withdrawalA;
    IRolldownPrimitives.Withdrawal private _withdrawalB;
    bytes32 private _merkleRoot;
    bytes32[] private _merkleProof;
    uint256 private _amount = 1 ether;
    uint256 txGasPrice = vm.envUint("BENCHMARKS_GAS_PRICE_GWEI") * 10 ** 9;

    function run() external {
        console.log("Rolldown contract");
        console.log("Gas price (wei): %s", txGasPrice);
        console.log("-------------------------------------------------------------------");

        _calculateGasToFerryWithdrawall();
        _calculateGasToUpdateL1FromL2();
        _calculateGasToCloseWithdrawal();
    }

    function _setUp() private {
        vm.txGasPrice(txGasPrice);

        string memory mnemonic = vm.envString("MNEMONIC");

        users = Users({
            admin: vm.rememberKey(vm.deriveKey(mnemonic, 0)),
            depositor: vm.rememberKey(vm.deriveKey(mnemonic, 1)),
            updater: vm.rememberKey(vm.deriveKey(mnemonic, 2)),
            withdrawer: vm.rememberKey(vm.deriveKey(mnemonic, 3)),
            recipient: vm.rememberKey(vm.deriveKey(mnemonic, 4))
        });

        vm.startBroadcast(users.admin);

        Rolldown implementation = new Rolldown();
        ProxyAdmin proxyAdmin = new ProxyAdmin();
        TransparentUpgradeableProxy proxy = new TransparentUpgradeableProxy(
            address(implementation),
            address(proxyAdmin),
            abi.encodeCall(Rolldown.initialize, (users.admin, IRolldownPrimitives.ChainId.Ethereum, users.updater))
        );

        vm.stopBroadcast();

        rolldown = Rolldown(payable(address(proxy)));

        vm.broadcast(users.depositor);
        rolldown.depositNative{value: _amount}();

        address nativeTokenAddress = rolldown.NATIVE_TOKEN_ADDRESS();

        _withdrawalA = IRolldownPrimitives.Withdrawal(
            _createRequestId(IRolldownPrimitives.Origin.L2), users.recipient, nativeTokenAddress, _amount, 0
        );
        bytes32 _withdrawalHashA = rolldown.hashWithdrawal(_withdrawalA);
        _withdrawalB = IRolldownPrimitives.Withdrawal(
            _createRequestId(IRolldownPrimitives.Origin.L2), users.recipient, nativeTokenAddress, _amount, 0
        );
        bytes32 _withdrawalHashB = rolldown.hashWithdrawal(_withdrawalB);

        _merkleRoot = keccak256(abi.encodePacked(_withdrawalHashA, _withdrawalHashB));
        _merkleProof = new bytes32[](1);
        _merkleProof[0] = _withdrawalHashB;
    }

    function _calculateGasToFerryWithdrawall() private {
        console.log("Call ferryWithdrawal method");
        _setUp();

        uint256 gasStart = gasleft();

        vm.broadcast(users.withdrawer);
        rolldown.ferryWithdrawal{value: _amount}(_withdrawalA);

        uint256 gasEnd = gasleft();
        _printMethodCallReport(gasStart - gasEnd);
    }

    function _calculateGasToUpdateL1FromL2() private {
        console.log("Call updateL1FromL2");
        _setUp();

        uint256 gasStart = gasleft();

        vm.broadcast(users.updater);
        rolldown.updateL1FromL2(_merkleRoot, _range);

        uint256 gasEnd = gasleft();
        _printMethodCallReport(gasStart - gasEnd);
    }

    function _calculateGasToCloseWithdrawal() private {
        console.log("Call closeWithdrawal method");
        _setUp();

        vm.broadcast(users.updater);
        rolldown.updateL1FromL2(_merkleRoot, _range);

        uint256 gasStart = gasleft();

        vm.broadcast(users.withdrawer);
        rolldown.closeWithdrawal(_withdrawalA, _merkleRoot, _merkleProof);

        uint256 gasEnd = gasleft();
        _printMethodCallReport(gasStart - gasEnd);
    }

    function _createRequestId(IRolldownPrimitives.Origin origin)
        private
        view
        returns (IRolldownPrimitives.RequestId memory)
    {
        return IRolldownPrimitives.RequestId({origin: origin, id: rolldown.counter()});
    }

    function _printMethodCallReport(uint256 gasUsed) private view {
        console.log("Gas used (wei): %d", gasUsed);
        console.log("Transaction cost (wei): %d", gasUsed * txGasPrice);
        console.log("-------------------------------------------------------------------");
    }
}
