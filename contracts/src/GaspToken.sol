// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MintableERC20 is ERC20 {
    address public owner;
    constructor(string memory name_, string memory symbol_) ERC20(name_, symbol_) {
        owner = msg.sender;
        _mint(msg.sender, 1000000000 * 10 ** decimals());
    }

    function mint(address account, uint256 amount) public {
      require((owner == msg.sender), "Only one who deployed contract can mint tokens");
      _mint(account, amount);
    }
}


contract Gasp is MintableERC20 {
    constructor() MintableERC20("GaspV2", "GASPV2") {}
}

contract DevToken is MintableERC20 {
    constructor() MintableERC20("GaspDev", "GASPDEV") {}
}

contract ArbToken is MintableERC20 {
    constructor() MintableERC20("CrossChain Token", "CCXT") {}
}