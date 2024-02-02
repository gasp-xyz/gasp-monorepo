// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts@5.0.1/token/ERC20/ERC20.sol";

contract Mangata is ERC20 {
    constructor() ERC20("Mangata", "MGA") {
        _mint(msg.sender, 1000000000 * 10 ** decimals());
    }
}
