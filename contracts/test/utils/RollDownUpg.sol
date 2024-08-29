import {Rolldown} from "../../src/Rolldown.sol";
contract RollDownUpg is Rolldown {
    function imUpgraded() public returns (bool) {
      return true;
    }
}