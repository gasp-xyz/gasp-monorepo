// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.13;

library LMerkleTree {
    function calculateRoot(bytes32 leaveHash, uint256 leaveIdx, bytes32[] memory proof, uint256 leaveCount)
        internal
        pure
        returns (bytes32)
    {
        uint256 levels;
        uint256 tmp = leaveCount;
        while (tmp > 0) {
            tmp = tmp / 2;
            levels += 1;
        }

        return calculateRootImpl(levels, leaveIdx, leaveHash, proof, 0, leaveCount - 1);
    }

    function calculateRootImpl(
        uint256 level,
        uint256 pos,
        bytes32 hash,
        bytes32[] memory proofs,
        uint256 proofIdx,
        uint256 maxIdx
    ) internal pure returns (bytes32) {
        if (pos % 2 == 0) {
            if (pos == maxIdx) {
                // promoted node
            } else {
                hash = keccak256(abi.encodePacked(hash, proofs[proofIdx++]));
            }
        } else {
            hash = keccak256(abi.encodePacked(proofs[proofIdx++], hash));
        }

        return level == 1 ? hash : calculateRootImpl(level - 1, pos / 2, hash, proofs, proofIdx, maxIdx / 2);
    }
}
