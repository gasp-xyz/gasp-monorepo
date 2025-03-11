// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract Faucet is Ownable {
    IERC20 public token;
    uint256 public withdrawalAmount = 10 * (10 ** 18);
    uint256 public lockTime = 10 minutes;
    uint256 public maxUsageCount = 3;
    mapping(address => uint256) private nextAccessTime;
    mapping(address => uint256) private usageCount;

    event Withdrawal(address indexed to, uint256 indexed amount);

    constructor(address tokenAddress) payable Ownable() {
        token = IERC20(tokenAddress);
    }

    function requestTokens() public {
        require(
            token.balanceOf(address(this)) >= withdrawalAmount, "Insufficient balance in faucet for withdrawal request"
        );
        require(
            block.timestamp >= nextAccessTime[msg.sender],
            "Insufficient time elapsed since last withdrawal - try again later."
        );
        require(usageCount[msg.sender] < maxUsageCount, "Address has already used the faucet maximum 3 times");

        nextAccessTime[msg.sender] = block.timestamp + lockTime;
        usageCount[msg.sender]++;

        token.transfer(msg.sender, withdrawalAmount);
    }

    function getBalance() external view returns (uint256) {
        return token.balanceOf(address(this));
    }

    function setWithdrawalAmount(uint256 amount) public onlyOwner {
        withdrawalAmount = amount * (10 ** 18);
    }

    function setLockTime(uint256 amount) public onlyOwner {
        lockTime = amount * 1 minutes;
    }

    function setMaxUsageCount(uint256 count) public onlyOwner {
        maxUsageCount = count;
    }

    function withdraw() external onlyOwner {
        emit Withdrawal(msg.sender, token.balanceOf(address(this)));
        token.transfer(msg.sender, token.balanceOf(address(this)));
    }
}
