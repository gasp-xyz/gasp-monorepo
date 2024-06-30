// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import "./IGaspMultiRollupServicePrimitives.sol";

interface IGaspMultiRollupService is IGaspMultiRollupServicePrimitives {
/*
    function withdraw_pending_eth(uint256 amount) external;

    function deposit_eth() external payable;

    function deposit(address tokenAddress, uint256 amount) external;

    function deposit_erc20(address tokenAddress, uint256 amount) external;

    function getUpdateForL2() external view returns (L1Update memory) ;

    function update_l1_from_l2(L2Update calldata inputArray) external;

    function getPendingRequests(
        uint256 start,
        uint256 end
    ) external view returns (L1Update memory);
*/
}


        // BN254.G1Point[] nonSignerPubkeys; // is the G1 pubkeys of all nonsigners. These need to be sorted by their g1hash
        // uint32[] nonSignerPubkeysIndicesforOperatorIdsRemovedForOldState;
        // BN254.G1Point[] nonSignerPubkeysAddedForOldState;
        // BN254.G2Point apkG2forOldState;
        // BN254.G1Point sigmaforOldSate;

        // // With operator keying
        // uint32[] nonSignerPubkeysIndicesforOperatorIdsRemovedForOldState; // We will use new state to query operator to operatorId and remove them from the nonSignerPubkeys. nonSignerPubkeys are used to subtract from quorumsTotalStake and from quorumApks using number of quorums they are part of. BUT how will you remove them it is a plain array? Binary lookup? nonSignerPubkeys needs to be sorted!!
        // // When iterating - if the itr matches the next itr here - May need this check No real need for this check (check that each of these have quorum count 0 in old state and quorum count non zero in the update) - then next on the itr here... Create two apks simultaneously
        // BN254.G1Point[] nonSignerPubkeysAddedForOldState; // To use this actually check that hash ie operatorId's quorum count is non-zero in old and that in the update was infact set to 0. And then use the old quorum count we get to calc the old apk...
        // // Nah!! Just add and remove according to the above and then just get the quorum count and the g1s and then calc the non-signing apks and stake for both old and new... 


        // // For the following two we need to simply get the old state (along with the new one for the above two) and compute them like we compute the above two... 
        // BN254.G2Point apkG2forOldState; // is the aggregate G2 pubkey of all signers
        // BN254.G1Point sigmaforOldSate; // is the aggregate G1 signature of all signers

        // // We do not require a collary for these as we can query the stake and number of quorums directly...
        // uint32[] nonSignerQuorumBitmapIndices; // is the indices of all nonsigner quorum bitmaps
        // uint32[][] nonSignerStakeIndices; // is the indices of each non signers stake within a quorum
        // // We do not require these we can just query
        // BN254.G1Point[] quorumApks; // is the aggregate G1 pubkey of each quorum
        // uint32[] quorumApkIndices; // is the indices of each quorum aggregate pubkey
        // uint32[] totalStakeIndices; // is the indices of each quorums total stake