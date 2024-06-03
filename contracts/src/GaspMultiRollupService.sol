// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.9;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/utils/Address.sol";
import "@openzeppelin/contracts/utils/Strings.sol";
import "forge-std/console.sol";

import "@openzeppelin-upgrades/contracts/proxy/utils/Initializable.sol";
import "@openzeppelin-upgrades/contracts/access/OwnableUpgradeable.sol";
import "@eigenlayer/contracts/permissions/Pausable.sol";
import "./GaspMultiRollupServiceStorage.sol";

// We require that the minimum stake for all quorums is 1 to avoid issues with stake being 0 but apk registry having the op's g1PubKey. We may not need this? Update apk based on the opIds returned by getter
// We require that staleStakesForbidden in bls signature checker is true. And that if withdrwal delay block is about 7 days, then task response is atmost about 1day and that on other L1s the staleness is max 3days? But how do we ensure that that other L1s are updated within say 2 days of responding on Eth and if not then brick accordingly? Maybe the updater can do this?
// Maybe we do not need this - We require qourumNumbers to be static - let's just enforce a check in createNewTask - Maybe an extrinsic to use the check or not - Probably not we need to match on the other L1s
// Maybe we do not need this - We require Threshold percentage to be static - Let's enforce that too - Maybe an extrinsic to use the check or not - Probably not we need to match on the other L1s
// Do not init the operator_info service in the finalizer/operator! Do not put all the operator bls keys in the TaskResponse. Only the changes in stuff and the bls keys of the new ops
// Do not double verify if delta is null
contract GaspMultiRollupService is
    Initializable,
    OwnableUpgradeable,
    Pausable,
    GaspMultiRollupServiceStorage
{

    function initialize(IPauserRegistry _pauserRegistry, address initialOwner, address _updater, IRolldown _rolldown)
        public
        initializer
    {
        _initializePauser(_pauserRegistry, UNPAUSE_ALL);
        _transferOwnership(initialOwner);
        updater = _updater;
        rolldown = _rolldown;
    }

    /* MODIFIERS */
    modifier onlyUpdater() {
        require(msg.sender == updater, "Updater must be the caller");
        _;
    }

    function set_updater(address _updater) public onlyOwner {
        updater = _updater;
    }

    function set_rolldown(IRolldown _rolldown) public onlyOwner {
        rolldown = _rolldown;
    }
}