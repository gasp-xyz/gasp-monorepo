// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

import {Rolldown} from "../../src/Rolldown.sol";

contract RollDownUpg is Rolldown {
    function imUpgraded() external pure returns (bool) {
      return true;
    }
}