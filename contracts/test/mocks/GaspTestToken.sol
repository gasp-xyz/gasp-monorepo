// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {Context} from "@openzeppelin/contracts/utils/Context.sol";

contract MintableERC20 is Context, Ownable, ERC20 {
    constructor(string memory name_, string memory symbol_) Ownable() ERC20(name_, symbol_) {
        _mint(_msgSender(), 1_000_000_000 * 10 ** decimals());
    }

    function mint(address to, uint256 amount) external onlyOwner {
        _mint(to, amount);
    }
}

contract GaspTestToken is MintableERC20 {
    constructor() MintableERC20("GaspV2", "GASPV2") {}
}

contract DevToken is MintableERC20 {
    constructor() MintableERC20("GaspDev", "GASPDEV") {}
}

contract ArbToken is MintableERC20 {
    constructor() MintableERC20("CrossChain Token", "CCXT") {}
}
