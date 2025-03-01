// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract AetherForgeRewards {
    mapping(address => uint256) public rewards;
    address[] public miners;

    function distributeRewards(address miner, uint256 amount) public {
        rewards[miner] += amount;
        miners.push(miner);
    }

    function claimRewards() public payable {
        uint256 reward = rewards[msg.sender];
        require(reward > 0, "No rewards to claim");
        rewards[msg.sender] = 0;
        (bool success, ) = msg.sender.call{value: reward}("");
        require(success, "Transfer failed.");
    }
}