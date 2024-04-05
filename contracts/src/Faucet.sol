// SPDX-License-Identifier: MIT

pragma solidity ^0.8.12;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";


contract TestFaucet is Ownable {
    IERC20 public token;

    uint256 public withdrawalAmount = 10 * (10**18);
    uint256 public lockTime = 10 minutes;

    event Withdrawal(address indexed to, uint256 indexed amount);

    mapping(address => uint256) nextAccessTime;

    // Mapping to keep track of usage count for each address
    mapping(address => uint256) private usageCount;

    // Maximum number of times an address can use the faucet
    uint256 public constant maxUsageCount = 3;

    constructor(address tokenAddress) Ownable() payable {
        token = IERC20(tokenAddress);
    }

    function requestTokens() public {
        require(
            token.balanceOf(address(this)) >= withdrawalAmount,
            "Insufficient balance in faucet for withdrawal request"
        );
        require(
            block.timestamp >= nextAccessTime[msg.sender],
            "Insufficient time elapsed since last withdrawal - try again later."
        );
        require(
            usageCount[msg.sender] < maxUsageCount,
            "Address has already used the faucet maximum 3 times"
        );

        nextAccessTime[msg.sender] = block.timestamp + lockTime;

        token.transfer(msg.sender, withdrawalAmount);
    }

    function getBalance() external view returns (uint256) {
        return token.balanceOf(address(this));
    }

    function setWithdrawalAmount(uint256 amount) public onlyOwner {
        withdrawalAmount = amount * (10**18);
    }

    function setLockTime(uint256 amount) public onlyOwner {
        lockTime = amount * 1 minutes;
    }

    function withdraw() external onlyOwner {
        emit Withdrawal(msg.sender, token.balanceOf(address(this)));
        token.transfer(msg.sender, token.balanceOf(address(this)));
    }
}
