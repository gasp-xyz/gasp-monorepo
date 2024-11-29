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
    uint256 private _gasPrice = 20 gwei;

    function run() external {
        _gasEndFerryWithdrawal();
        _gasEndUpdateL1FromL2();
        _gasEndCloseWithdrawal();
    }

    function _setUp() private {
        users = Users({
            admin: 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266,
            updater: 0x70997970C51812dc3A010C7d01b50e0d17dc79C8,
            withdrawer: 0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC,
            recipient: 0x90F79bf6EB2c4f870365E785982E1f101E93b906
        });

        Rolldown implementation = new Rolldown();
        ProxyAdmin proxyAdmin = new ProxyAdmin();
        TransparentUpgradeableProxy proxy = new TransparentUpgradeableProxy(
            address(implementation),
            address(proxyAdmin),
            abi.encodeCall(Rolldown.initialize, (users.admin, IRolldownPrimitives.ChainId.Ethereum, users.updater))
        );
        rolldown = Rolldown(address(proxy));

        vm.prank(users.admin);
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

    function _gasEndFerryWithdrawal() private {
        _setUp();

        vm.txGasPrice(_gasPrice);
        uint256 gasStart = gasleft();

        vm.startBroadcast(users.withdrawer);
        rolldown.ferryWithdrawal{value: _amount}(_withdrawalA);
        vm.stopBroadcast();

        uint256 gasEnd = gasleft();
        uint256 gasUsed = (gasStart - gasEnd) * tx.gasprice;

        console.log("ferryWithdrawal gas used: %s", gasUsed);
        console.log("-------------------------------------------------------------------");
    }

    function _gasEndUpdateL1FromL2() private {
        _setUp();

        vm.txGasPrice(_gasPrice);
        uint256 gasStart = gasleft();

        vm.startBroadcast(users.updater);
        rolldown.updateL1FromL2(_merkleRoot, _range);
        vm.stopBroadcast();

        uint256 gasEnd = gasleft();
        uint256 gasUsed = (gasStart - gasEnd) * tx.gasprice;

        console.log("updateL1FromL2 gas used: %s", gasUsed);
        console.log("-------------------------------------------------------------------");
    }

    function _gasEndCloseWithdrawal() private {
        _setUp();

        vm.txGasPrice(_gasPrice);
        uint256 gasStart = gasleft();

        vm.prank(users.updater);
        rolldown.updateL1FromL2(_merkleRoot, _range);

        vm.startBroadcast(users.withdrawer);
        rolldown.closeWithdrawal(_withdrawalA, _merkleRoot, _merkleProof);
        vm.stopBroadcast();

        uint256 gasEnd = gasleft();
        uint256 gasUsed = (gasStart - gasEnd) * tx.gasprice;

        console.log("closeWithdrawal gas used: %s", gasUsed);
        console.log("-------------------------------------------------------------------");
    }

    function _createRequestId(IRolldownPrimitives.Origin origin)
        private
        view
        returns (IRolldownPrimitives.RequestId memory)
    {
        return IRolldownPrimitives.RequestId({origin: origin, id: rolldown.counter()});
    }
}
