// SPDX-License-Identifier: MIT
pragma solidity ^0.8.12;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract Mangata is ERC20 {
    constructor() ERC20("Mangata", "MGA") {
        _mint(msg.sender, 1000000000 * 10 ** decimals());
    }
}
