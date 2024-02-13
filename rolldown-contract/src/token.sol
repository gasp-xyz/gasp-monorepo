// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract TestToken is ERC20{

    constructor () ERC20 ("DevToken","DEV"){
      //_mint(msg.sender, 100000000000000000000000000);
    }

    function mint(address to, uint256 amount) public virtual {
        _mint(to,amount);
    }

    function burn(address form, uint amount) public virtual {
        _burn(form, amount);
    }
}
