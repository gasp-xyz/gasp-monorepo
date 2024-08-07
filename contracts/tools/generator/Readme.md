# Test cases generator

The purpose of this tool is to generate merkle proof test cases for solidity. Since there is number of implementation details that may affect the merkle root calculation (e.g. prehashing keys, node promotion etc) to make sure that the implementation is compatible with what we use on the GASP this tool was invented.

When run it generates number of merkle trees of different number of nodes, together with merkle roots and proof for every leave node. Merkle tree is hashed using `keccak256` algorithm and produced trees are both balanced and unbalanced. Output file(json) is used as an input for solidity tests of Rolldown contract.
