pub use i_stake_registry::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod i_stake_registry {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("deregisterOperator"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("deregisterOperator"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("quorumNumbers"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getCurrentOperatorStakeForQuorum"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getCurrentOperatorStakeForQuorum",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint96"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getCurrentTotalStakeForQuorum"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getCurrentTotalStakeForQuorum",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint8"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint96"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getLengthOfTotalStakeHistoryForQuorum"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "getLengthOfTotalStakeHistoryForQuorum",
                        ),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint8"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getMostRecentStakeUpdateByOperatorId"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "getMostRecentStakeUpdateByOperatorId",
                        ),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IStakeRegistry.OperatorStakeUpdate",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getOperatorIdToStakeHistory"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getOperatorIdToStakeHistory",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                    ],),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IStakeRegistry.OperatorStakeUpdate[]",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getStakeForOperatorIdForQuorumAtBlockNumber"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "getStakeForOperatorIdForQuorumAtBlockNumber",
                        ),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("blockNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint96"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "getStakeForQuorumAtBlockNumberFromOperatorIdAndIndex",
                    ),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "getStakeForQuorumAtBlockNumberFromOperatorIdAndIndex",
                        ),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("blockNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("index"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint96"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "getStakeUpdateForQuorumFromOperatorIdAndIndex",
                    ),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "getStakeUpdateForQuorumFromOperatorIdAndIndex",
                        ),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("index"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IStakeRegistry.OperatorStakeUpdate",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "getStakeUpdateIndexForOperatorIdForQuorumAtBlockNumber",
                    ),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "getStakeUpdateIndexForOperatorIdForQuorumAtBlockNumber",
                        ),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("blockNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getTotalStakeAtBlockNumberFromIndex"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "getTotalStakeAtBlockNumberFromIndex",
                        ),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("blockNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("index"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint96"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "getTotalStakeIndicesByQuorumNumbersAtBlockNumber",
                    ),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "getTotalStakeIndicesByQuorumNumbersAtBlockNumber",
                        ),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("blockNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("quorumNumbers"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getTotalStakeUpdateForQuorumFromIndex"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "getTotalStakeUpdateForQuorumFromIndex",
                        ),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("index"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IStakeRegistry.OperatorStakeUpdate",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("minimumStakeForQuorum"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("minimumStakeForQuorum",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint96"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("registerOperator"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("registerOperator"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("quorumNumbers"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("registryCoordinator"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("registryCoordinator",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract IRegistryCoordinator",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateStakes"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updateStakes"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("operators"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address[]"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("MinimumStakeForQuorumUpdated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("MinimumStakeForQuorumUpdated",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("minimumStake"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StakeUpdate"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("StakeUpdate"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("stake"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ISTAKEREGISTRY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    pub struct IStakeRegistry<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IStakeRegistry<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IStakeRegistry<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IStakeRegistry<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IStakeRegistry<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IStakeRegistry))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IStakeRegistry<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                ISTAKEREGISTRY_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `deregisterOperator` (0xbd29b8cd) function
        pub fn deregister_operator(
            &self,
            operator_id: [u8; 32],
            quorum_numbers: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([189, 41, 184, 205], (operator_id, quorum_numbers))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCurrentOperatorStakeForQuorum` (0x6ab538d4) function
        pub fn get_current_operator_stake_for_quorum(
            &self,
            operator_id: [u8; 32],
            quorum_number: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([106, 181, 56, 212], (operator_id, quorum_number))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCurrentTotalStakeForQuorum` (0xc8f739d0) function
        pub fn get_current_total_stake_for_quorum(
            &self,
            quorum_number: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([200, 247, 57, 208], quorum_number)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLengthOfTotalStakeHistoryForQuorum` (0xe89c0a00) function
        pub fn get_length_of_total_stake_history_for_quorum(
            &self,
            quorum_number: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([232, 156, 10, 0], quorum_number)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMostRecentStakeUpdateByOperatorId` (0xcd050d9c) function
        pub fn get_most_recent_stake_update_by_operator_id(
            &self,
            operator_id: [u8; 32],
            quorum_number: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, OperatorStakeUpdate> {
            self.0
                .method_hash([205, 5, 13, 156], (operator_id, quorum_number))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOperatorIdToStakeHistory` (0x944472a9) function
        pub fn get_operator_id_to_stake_history(
            &self,
            operator_id: [u8; 32],
            quorum_number: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<OperatorStakeUpdate>>
        {
            self.0
                .method_hash([148, 68, 114, 169], (operator_id, quorum_number))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStakeForOperatorIdForQuorumAtBlockNumber` (0x1b327225) function
        pub fn get_stake_for_operator_id_for_quorum_at_block_number(
            &self,
            operator_id: [u8; 32],
            quorum_number: u8,
            block_number: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash(
                    [27, 50, 114, 37],
                    (operator_id, quorum_number, block_number),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStakeForQuorumAtBlockNumberFromOperatorIdAndIndex` (0xa43cde89) function
        pub fn get_stake_for_quorum_at_block_number_from_operator_id_and_index(
            &self,
            quorum_number: u8,
            block_number: u32,
            operator_id: [u8; 32],
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash(
                    [164, 60, 222, 137],
                    (quorum_number, block_number, operator_id, index),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStakeUpdateForQuorumFromOperatorIdAndIndex` (0x248d6573) function
        pub fn get_stake_update_for_quorum_from_operator_id_and_index(
            &self,
            quorum_number: u8,
            operator_id: [u8; 32],
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, OperatorStakeUpdate> {
            self.0
                .method_hash([36, 141, 101, 115], (quorum_number, operator_id, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStakeUpdateIndexForOperatorIdForQuorumAtBlockNumber` (0x48085866) function
        pub fn get_stake_update_index_for_operator_id_for_quorum_at_block_number(
            &self,
            operator_id: [u8; 32],
            quorum_number: u8,
            block_number: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([72, 8, 88, 102], (operator_id, quorum_number, block_number))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTotalStakeAtBlockNumberFromIndex` (0xc8294c56) function
        pub fn get_total_stake_at_block_number_from_index(
            &self,
            quorum_number: u8,
            block_number: u32,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([200, 41, 76, 86], (quorum_number, block_number, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTotalStakeIndicesByQuorumNumbersAtBlockNumber` (0xe192e9ad) function
        pub fn get_total_stake_indices_by_quorum_numbers_at_block_number(
            &self,
            block_number: u32,
            quorum_numbers: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<u32>> {
            self.0
                .method_hash([225, 146, 233, 173], (block_number, quorum_numbers))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTotalStakeUpdateForQuorumFromIndex` (0xe25427dd) function
        pub fn get_total_stake_update_for_quorum_from_index(
            &self,
            quorum_number: u8,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, OperatorStakeUpdate> {
            self.0
                .method_hash([226, 84, 39, 221], (quorum_number, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `minimumStakeForQuorum` (0x7ed9430f) function
        pub fn minimum_stake_for_quorum(
            &self,
            quorum_number: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([126, 217, 67, 15], quorum_number)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerOperator` (0x25504777) function
        pub fn register_operator(
            &self,
            operator: ::ethers::core::types::Address,
            operator_id: [u8; 32],
            quorum_numbers: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([37, 80, 71, 119], (operator, operator_id, quorum_numbers))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registryCoordinator` (0x6d14a987) function
        pub fn registry_coordinator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([109, 20, 169, 135], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateStakes` (0xce977ec3) function
        pub fn update_stakes(
            &self,
            operators: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([206, 151, 126, 195], operators)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `MinimumStakeForQuorumUpdated` event
        pub fn minimum_stake_for_quorum_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MinimumStakeForQuorumUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `StakeUpdate` event
        pub fn stake_update_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, StakeUpdateFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, IStakeRegistryEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for IStakeRegistry<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "MinimumStakeForQuorumUpdated",
        abi = "MinimumStakeForQuorumUpdated(uint8,uint96)"
    )]
    pub struct MinimumStakeForQuorumUpdatedFilter {
        #[ethevent(indexed)]
        pub quorum_number: u8,
        pub minimum_stake: u128,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "StakeUpdate", abi = "StakeUpdate(bytes32,uint8,uint96)")]
    pub struct StakeUpdateFilter {
        #[ethevent(indexed)]
        pub operator_id: [u8; 32],
        pub quorum_number: u8,
        pub stake: u128,
    }
    ///Container type for all of the contract's events
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub enum IStakeRegistryEvents {
        MinimumStakeForQuorumUpdatedFilter(MinimumStakeForQuorumUpdatedFilter),
        StakeUpdateFilter(StakeUpdateFilter),
    }
    impl ::ethers::contract::EthLogDecode for IStakeRegistryEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = MinimumStakeForQuorumUpdatedFilter::decode_log(log) {
                return Ok(IStakeRegistryEvents::MinimumStakeForQuorumUpdatedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = StakeUpdateFilter::decode_log(log) {
                return Ok(IStakeRegistryEvents::StakeUpdateFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IStakeRegistryEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::MinimumStakeForQuorumUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StakeUpdateFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<MinimumStakeForQuorumUpdatedFilter> for IStakeRegistryEvents {
        fn from(value: MinimumStakeForQuorumUpdatedFilter) -> Self {
            Self::MinimumStakeForQuorumUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<StakeUpdateFilter> for IStakeRegistryEvents {
        fn from(value: StakeUpdateFilter) -> Self {
            Self::StakeUpdateFilter(value)
        }
    }
    ///Container type for all input parameters for the `deregisterOperator` function with signature `deregisterOperator(bytes32,bytes)` and selector `0xbd29b8cd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "deregisterOperator", abi = "deregisterOperator(bytes32,bytes)")]
    pub struct DeregisterOperatorCall {
        pub operator_id: [u8; 32],
        pub quorum_numbers: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `getCurrentOperatorStakeForQuorum` function with signature `getCurrentOperatorStakeForQuorum(bytes32,uint8)` and selector `0x6ab538d4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "getCurrentOperatorStakeForQuorum",
        abi = "getCurrentOperatorStakeForQuorum(bytes32,uint8)"
    )]
    pub struct GetCurrentOperatorStakeForQuorumCall {
        pub operator_id: [u8; 32],
        pub quorum_number: u8,
    }
    ///Container type for all input parameters for the `getCurrentTotalStakeForQuorum` function with signature `getCurrentTotalStakeForQuorum(uint8)` and selector `0xc8f739d0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "getCurrentTotalStakeForQuorum",
        abi = "getCurrentTotalStakeForQuorum(uint8)"
    )]
    pub struct GetCurrentTotalStakeForQuorumCall {
        pub quorum_number: u8,
    }
    ///Container type for all input parameters for the `getLengthOfTotalStakeHistoryForQuorum` function with signature `getLengthOfTotalStakeHistoryForQuorum(uint8)` and selector `0xe89c0a00`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "getLengthOfTotalStakeHistoryForQuorum",
        abi = "getLengthOfTotalStakeHistoryForQuorum(uint8)"
    )]
    pub struct GetLengthOfTotalStakeHistoryForQuorumCall {
        pub quorum_number: u8,
    }
    ///Container type for all input parameters for the `getMostRecentStakeUpdateByOperatorId` function with signature `getMostRecentStakeUpdateByOperatorId(bytes32,uint8)` and selector `0xcd050d9c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "getMostRecentStakeUpdateByOperatorId",
        abi = "getMostRecentStakeUpdateByOperatorId(bytes32,uint8)"
    )]
    pub struct GetMostRecentStakeUpdateByOperatorIdCall {
        pub operator_id: [u8; 32],
        pub quorum_number: u8,
    }
    ///Container type for all input parameters for the `getOperatorIdToStakeHistory` function with signature `getOperatorIdToStakeHistory(bytes32,uint8)` and selector `0x944472a9`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "getOperatorIdToStakeHistory",
        abi = "getOperatorIdToStakeHistory(bytes32,uint8)"
    )]
    pub struct GetOperatorIdToStakeHistoryCall {
        pub operator_id: [u8; 32],
        pub quorum_number: u8,
    }
    ///Container type for all input parameters for the `getStakeForOperatorIdForQuorumAtBlockNumber` function with signature `getStakeForOperatorIdForQuorumAtBlockNumber(bytes32,uint8,uint32)` and selector `0x1b327225`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "getStakeForOperatorIdForQuorumAtBlockNumber",
        abi = "getStakeForOperatorIdForQuorumAtBlockNumber(bytes32,uint8,uint32)"
    )]
    pub struct GetStakeForOperatorIdForQuorumAtBlockNumberCall {
        pub operator_id: [u8; 32],
        pub quorum_number: u8,
        pub block_number: u32,
    }
    ///Container type for all input parameters for the `getStakeForQuorumAtBlockNumberFromOperatorIdAndIndex` function with signature `getStakeForQuorumAtBlockNumberFromOperatorIdAndIndex(uint8,uint32,bytes32,uint256)` and selector `0xa43cde89`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "getStakeForQuorumAtBlockNumberFromOperatorIdAndIndex",
        abi = "getStakeForQuorumAtBlockNumberFromOperatorIdAndIndex(uint8,uint32,bytes32,uint256)"
    )]
    pub struct GetStakeForQuorumAtBlockNumberFromOperatorIdAndIndexCall {
        pub quorum_number: u8,
        pub block_number: u32,
        pub operator_id: [u8; 32],
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getStakeUpdateForQuorumFromOperatorIdAndIndex` function with signature `getStakeUpdateForQuorumFromOperatorIdAndIndex(uint8,bytes32,uint256)` and selector `0x248d6573`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "getStakeUpdateForQuorumFromOperatorIdAndIndex",
        abi = "getStakeUpdateForQuorumFromOperatorIdAndIndex(uint8,bytes32,uint256)"
    )]
    pub struct GetStakeUpdateForQuorumFromOperatorIdAndIndexCall {
        pub quorum_number: u8,
        pub operator_id: [u8; 32],
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getStakeUpdateIndexForOperatorIdForQuorumAtBlockNumber` function with signature `getStakeUpdateIndexForOperatorIdForQuorumAtBlockNumber(bytes32,uint8,uint32)` and selector `0x48085866`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "getStakeUpdateIndexForOperatorIdForQuorumAtBlockNumber",
        abi = "getStakeUpdateIndexForOperatorIdForQuorumAtBlockNumber(bytes32,uint8,uint32)"
    )]
    pub struct GetStakeUpdateIndexForOperatorIdForQuorumAtBlockNumberCall {
        pub operator_id: [u8; 32],
        pub quorum_number: u8,
        pub block_number: u32,
    }
    ///Container type for all input parameters for the `getTotalStakeAtBlockNumberFromIndex` function with signature `getTotalStakeAtBlockNumberFromIndex(uint8,uint32,uint256)` and selector `0xc8294c56`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "getTotalStakeAtBlockNumberFromIndex",
        abi = "getTotalStakeAtBlockNumberFromIndex(uint8,uint32,uint256)"
    )]
    pub struct GetTotalStakeAtBlockNumberFromIndexCall {
        pub quorum_number: u8,
        pub block_number: u32,
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getTotalStakeIndicesByQuorumNumbersAtBlockNumber` function with signature `getTotalStakeIndicesByQuorumNumbersAtBlockNumber(uint32,bytes)` and selector `0xe192e9ad`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "getTotalStakeIndicesByQuorumNumbersAtBlockNumber",
        abi = "getTotalStakeIndicesByQuorumNumbersAtBlockNumber(uint32,bytes)"
    )]
    pub struct GetTotalStakeIndicesByQuorumNumbersAtBlockNumberCall {
        pub block_number: u32,
        pub quorum_numbers: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `getTotalStakeUpdateForQuorumFromIndex` function with signature `getTotalStakeUpdateForQuorumFromIndex(uint8,uint256)` and selector `0xe25427dd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "getTotalStakeUpdateForQuorumFromIndex",
        abi = "getTotalStakeUpdateForQuorumFromIndex(uint8,uint256)"
    )]
    pub struct GetTotalStakeUpdateForQuorumFromIndexCall {
        pub quorum_number: u8,
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `minimumStakeForQuorum` function with signature `minimumStakeForQuorum(uint256)` and selector `0x7ed9430f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "minimumStakeForQuorum", abi = "minimumStakeForQuorum(uint256)")]
    pub struct MinimumStakeForQuorumCall {
        pub quorum_number: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `registerOperator` function with signature `registerOperator(address,bytes32,bytes)` and selector `0x25504777`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "registerOperator",
        abi = "registerOperator(address,bytes32,bytes)"
    )]
    pub struct RegisterOperatorCall {
        pub operator: ::ethers::core::types::Address,
        pub operator_id: [u8; 32],
        pub quorum_numbers: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `registryCoordinator` function with signature `registryCoordinator()` and selector `0x6d14a987`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "registryCoordinator", abi = "registryCoordinator()")]
    pub struct RegistryCoordinatorCall;
    ///Container type for all input parameters for the `updateStakes` function with signature `updateStakes(address[])` and selector `0xce977ec3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "updateStakes", abi = "updateStakes(address[])")]
    pub struct UpdateStakesCall {
        pub operators: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub enum IStakeRegistryCalls {
        DeregisterOperator(DeregisterOperatorCall),
        GetCurrentOperatorStakeForQuorum(GetCurrentOperatorStakeForQuorumCall),
        GetCurrentTotalStakeForQuorum(GetCurrentTotalStakeForQuorumCall),
        GetLengthOfTotalStakeHistoryForQuorum(GetLengthOfTotalStakeHistoryForQuorumCall),
        GetMostRecentStakeUpdateByOperatorId(GetMostRecentStakeUpdateByOperatorIdCall),
        GetOperatorIdToStakeHistory(GetOperatorIdToStakeHistoryCall),
        GetStakeForOperatorIdForQuorumAtBlockNumber(
            GetStakeForOperatorIdForQuorumAtBlockNumberCall,
        ),
        GetStakeForQuorumAtBlockNumberFromOperatorIdAndIndex(
            GetStakeForQuorumAtBlockNumberFromOperatorIdAndIndexCall,
        ),
        GetStakeUpdateForQuorumFromOperatorIdAndIndex(
            GetStakeUpdateForQuorumFromOperatorIdAndIndexCall,
        ),
        GetStakeUpdateIndexForOperatorIdForQuorumAtBlockNumber(
            GetStakeUpdateIndexForOperatorIdForQuorumAtBlockNumberCall,
        ),
        GetTotalStakeAtBlockNumberFromIndex(GetTotalStakeAtBlockNumberFromIndexCall),
        GetTotalStakeIndicesByQuorumNumbersAtBlockNumber(
            GetTotalStakeIndicesByQuorumNumbersAtBlockNumberCall,
        ),
        GetTotalStakeUpdateForQuorumFromIndex(GetTotalStakeUpdateForQuorumFromIndexCall),
        MinimumStakeForQuorum(MinimumStakeForQuorumCall),
        RegisterOperator(RegisterOperatorCall),
        RegistryCoordinator(RegistryCoordinatorCall),
        UpdateStakes(UpdateStakesCall),
    }
    impl ::ethers::core::abi::AbiDecode for IStakeRegistryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <DeregisterOperatorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DeregisterOperator(decoded));
            }
            if let Ok(decoded) =
                <GetCurrentOperatorStakeForQuorumCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetCurrentOperatorStakeForQuorum(decoded));
            }
            if let Ok(decoded) =
                <GetCurrentTotalStakeForQuorumCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetCurrentTotalStakeForQuorum(decoded));
            }
            if let Ok(decoded) = <GetLengthOfTotalStakeHistoryForQuorumCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetLengthOfTotalStakeHistoryForQuorum(decoded));
            }
            if let Ok(decoded) =
                <GetMostRecentStakeUpdateByOperatorIdCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetMostRecentStakeUpdateByOperatorId(decoded));
            }
            if let Ok(decoded) =
                <GetOperatorIdToStakeHistoryCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetOperatorIdToStakeHistory(decoded));
            }
            if let Ok(decoded) = <GetStakeForOperatorIdForQuorumAtBlockNumberCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetStakeForOperatorIdForQuorumAtBlockNumber(decoded));
            }
            if let Ok(decoded) = <GetStakeForQuorumAtBlockNumberFromOperatorIdAndIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::GetStakeForQuorumAtBlockNumberFromOperatorIdAndIndex(decoded),
                );
            }
            if let Ok(decoded) = <GetStakeUpdateForQuorumFromOperatorIdAndIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetStakeUpdateForQuorumFromOperatorIdAndIndex(decoded));
            }
            if let Ok(decoded) = <GetStakeUpdateIndexForOperatorIdForQuorumAtBlockNumberCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::GetStakeUpdateIndexForOperatorIdForQuorumAtBlockNumber(decoded),
                );
            }
            if let Ok(decoded) =
                <GetTotalStakeAtBlockNumberFromIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetTotalStakeAtBlockNumberFromIndex(decoded));
            }
            if let Ok(decoded) = <GetTotalStakeIndicesByQuorumNumbersAtBlockNumberCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::GetTotalStakeIndicesByQuorumNumbersAtBlockNumber(decoded),
                );
            }
            if let Ok(decoded) = <GetTotalStakeUpdateForQuorumFromIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTotalStakeUpdateForQuorumFromIndex(decoded));
            }
            if let Ok(decoded) =
                <MinimumStakeForQuorumCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MinimumStakeForQuorum(decoded));
            }
            if let Ok(decoded) =
                <RegisterOperatorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RegisterOperator(decoded));
            }
            if let Ok(decoded) =
                <RegistryCoordinatorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RegistryCoordinator(decoded));
            }
            if let Ok(decoded) = <UpdateStakesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateStakes(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IStakeRegistryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DeregisterOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetCurrentOperatorStakeForQuorum(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetCurrentTotalStakeForQuorum(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetLengthOfTotalStakeHistoryForQuorum(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetMostRecentStakeUpdateByOperatorId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOperatorIdToStakeHistory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetStakeForOperatorIdForQuorumAtBlockNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetStakeForQuorumAtBlockNumberFromOperatorIdAndIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetStakeUpdateForQuorumFromOperatorIdAndIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetStakeUpdateIndexForOperatorIdForQuorumAtBlockNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTotalStakeAtBlockNumberFromIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTotalStakeIndicesByQuorumNumbersAtBlockNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTotalStakeUpdateForQuorumFromIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MinimumStakeForQuorum(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterOperator(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RegistryCoordinator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateStakes(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for IStakeRegistryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DeregisterOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCurrentOperatorStakeForQuorum(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetCurrentTotalStakeForQuorum(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetLengthOfTotalStakeHistoryForQuorum(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetMostRecentStakeUpdateByOperatorId(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetOperatorIdToStakeHistory(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStakeForOperatorIdForQuorumAtBlockNumber(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetStakeForQuorumAtBlockNumberFromOperatorIdAndIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetStakeUpdateForQuorumFromOperatorIdAndIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetStakeUpdateIndexForOperatorIdForQuorumAtBlockNumber(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTotalStakeAtBlockNumberFromIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTotalStakeIndicesByQuorumNumbersAtBlockNumber(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTotalStakeUpdateForQuorumFromIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MinimumStakeForQuorum(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegistryCoordinator(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateStakes(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DeregisterOperatorCall> for IStakeRegistryCalls {
        fn from(value: DeregisterOperatorCall) -> Self {
            Self::DeregisterOperator(value)
        }
    }
    impl ::core::convert::From<GetCurrentOperatorStakeForQuorumCall> for IStakeRegistryCalls {
        fn from(value: GetCurrentOperatorStakeForQuorumCall) -> Self {
            Self::GetCurrentOperatorStakeForQuorum(value)
        }
    }
    impl ::core::convert::From<GetCurrentTotalStakeForQuorumCall> for IStakeRegistryCalls {
        fn from(value: GetCurrentTotalStakeForQuorumCall) -> Self {
            Self::GetCurrentTotalStakeForQuorum(value)
        }
    }
    impl ::core::convert::From<GetLengthOfTotalStakeHistoryForQuorumCall> for IStakeRegistryCalls {
        fn from(value: GetLengthOfTotalStakeHistoryForQuorumCall) -> Self {
            Self::GetLengthOfTotalStakeHistoryForQuorum(value)
        }
    }
    impl ::core::convert::From<GetMostRecentStakeUpdateByOperatorIdCall> for IStakeRegistryCalls {
        fn from(value: GetMostRecentStakeUpdateByOperatorIdCall) -> Self {
            Self::GetMostRecentStakeUpdateByOperatorId(value)
        }
    }
    impl ::core::convert::From<GetOperatorIdToStakeHistoryCall> for IStakeRegistryCalls {
        fn from(value: GetOperatorIdToStakeHistoryCall) -> Self {
            Self::GetOperatorIdToStakeHistory(value)
        }
    }
    impl ::core::convert::From<GetStakeForOperatorIdForQuorumAtBlockNumberCall>
        for IStakeRegistryCalls
    {
        fn from(value: GetStakeForOperatorIdForQuorumAtBlockNumberCall) -> Self {
            Self::GetStakeForOperatorIdForQuorumAtBlockNumber(value)
        }
    }
    impl ::core::convert::From<GetStakeForQuorumAtBlockNumberFromOperatorIdAndIndexCall>
        for IStakeRegistryCalls
    {
        fn from(value: GetStakeForQuorumAtBlockNumberFromOperatorIdAndIndexCall) -> Self {
            Self::GetStakeForQuorumAtBlockNumberFromOperatorIdAndIndex(value)
        }
    }
    impl ::core::convert::From<GetStakeUpdateForQuorumFromOperatorIdAndIndexCall>
        for IStakeRegistryCalls
    {
        fn from(value: GetStakeUpdateForQuorumFromOperatorIdAndIndexCall) -> Self {
            Self::GetStakeUpdateForQuorumFromOperatorIdAndIndex(value)
        }
    }
    impl ::core::convert::From<GetStakeUpdateIndexForOperatorIdForQuorumAtBlockNumberCall>
        for IStakeRegistryCalls
    {
        fn from(value: GetStakeUpdateIndexForOperatorIdForQuorumAtBlockNumberCall) -> Self {
            Self::GetStakeUpdateIndexForOperatorIdForQuorumAtBlockNumber(value)
        }
    }
    impl ::core::convert::From<GetTotalStakeAtBlockNumberFromIndexCall> for IStakeRegistryCalls {
        fn from(value: GetTotalStakeAtBlockNumberFromIndexCall) -> Self {
            Self::GetTotalStakeAtBlockNumberFromIndex(value)
        }
    }
    impl ::core::convert::From<GetTotalStakeIndicesByQuorumNumbersAtBlockNumberCall>
        for IStakeRegistryCalls
    {
        fn from(value: GetTotalStakeIndicesByQuorumNumbersAtBlockNumberCall) -> Self {
            Self::GetTotalStakeIndicesByQuorumNumbersAtBlockNumber(value)
        }
    }
    impl ::core::convert::From<GetTotalStakeUpdateForQuorumFromIndexCall> for IStakeRegistryCalls {
        fn from(value: GetTotalStakeUpdateForQuorumFromIndexCall) -> Self {
            Self::GetTotalStakeUpdateForQuorumFromIndex(value)
        }
    }
    impl ::core::convert::From<MinimumStakeForQuorumCall> for IStakeRegistryCalls {
        fn from(value: MinimumStakeForQuorumCall) -> Self {
            Self::MinimumStakeForQuorum(value)
        }
    }
    impl ::core::convert::From<RegisterOperatorCall> for IStakeRegistryCalls {
        fn from(value: RegisterOperatorCall) -> Self {
            Self::RegisterOperator(value)
        }
    }
    impl ::core::convert::From<RegistryCoordinatorCall> for IStakeRegistryCalls {
        fn from(value: RegistryCoordinatorCall) -> Self {
            Self::RegistryCoordinator(value)
        }
    }
    impl ::core::convert::From<UpdateStakesCall> for IStakeRegistryCalls {
        fn from(value: UpdateStakesCall) -> Self {
            Self::UpdateStakes(value)
        }
    }
    ///Container type for all return fields from the `getCurrentOperatorStakeForQuorum` function with signature `getCurrentOperatorStakeForQuorum(bytes32,uint8)` and selector `0x6ab538d4`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetCurrentOperatorStakeForQuorumReturn(pub u128);
    ///Container type for all return fields from the `getCurrentTotalStakeForQuorum` function with signature `getCurrentTotalStakeForQuorum(uint8)` and selector `0xc8f739d0`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetCurrentTotalStakeForQuorumReturn(pub u128);
    ///Container type for all return fields from the `getLengthOfTotalStakeHistoryForQuorum` function with signature `getLengthOfTotalStakeHistoryForQuorum(uint8)` and selector `0xe89c0a00`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetLengthOfTotalStakeHistoryForQuorumReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getMostRecentStakeUpdateByOperatorId` function with signature `getMostRecentStakeUpdateByOperatorId(bytes32,uint8)` and selector `0xcd050d9c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetMostRecentStakeUpdateByOperatorIdReturn(pub OperatorStakeUpdate);
    ///Container type for all return fields from the `getOperatorIdToStakeHistory` function with signature `getOperatorIdToStakeHistory(bytes32,uint8)` and selector `0x944472a9`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetOperatorIdToStakeHistoryReturn(pub ::std::vec::Vec<OperatorStakeUpdate>);
    ///Container type for all return fields from the `getStakeForOperatorIdForQuorumAtBlockNumber` function with signature `getStakeForOperatorIdForQuorumAtBlockNumber(bytes32,uint8,uint32)` and selector `0x1b327225`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetStakeForOperatorIdForQuorumAtBlockNumberReturn(pub u128);
    ///Container type for all return fields from the `getStakeForQuorumAtBlockNumberFromOperatorIdAndIndex` function with signature `getStakeForQuorumAtBlockNumberFromOperatorIdAndIndex(uint8,uint32,bytes32,uint256)` and selector `0xa43cde89`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetStakeForQuorumAtBlockNumberFromOperatorIdAndIndexReturn(pub u128);
    ///Container type for all return fields from the `getStakeUpdateForQuorumFromOperatorIdAndIndex` function with signature `getStakeUpdateForQuorumFromOperatorIdAndIndex(uint8,bytes32,uint256)` and selector `0x248d6573`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetStakeUpdateForQuorumFromOperatorIdAndIndexReturn(pub OperatorStakeUpdate);
    ///Container type for all return fields from the `getStakeUpdateIndexForOperatorIdForQuorumAtBlockNumber` function with signature `getStakeUpdateIndexForOperatorIdForQuorumAtBlockNumber(bytes32,uint8,uint32)` and selector `0x48085866`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetStakeUpdateIndexForOperatorIdForQuorumAtBlockNumberReturn(pub u32);
    ///Container type for all return fields from the `getTotalStakeAtBlockNumberFromIndex` function with signature `getTotalStakeAtBlockNumberFromIndex(uint8,uint32,uint256)` and selector `0xc8294c56`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetTotalStakeAtBlockNumberFromIndexReturn(pub u128);
    ///Container type for all return fields from the `getTotalStakeIndicesByQuorumNumbersAtBlockNumber` function with signature `getTotalStakeIndicesByQuorumNumbersAtBlockNumber(uint32,bytes)` and selector `0xe192e9ad`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetTotalStakeIndicesByQuorumNumbersAtBlockNumberReturn(pub ::std::vec::Vec<u32>);
    ///Container type for all return fields from the `getTotalStakeUpdateForQuorumFromIndex` function with signature `getTotalStakeUpdateForQuorumFromIndex(uint8,uint256)` and selector `0xe25427dd`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetTotalStakeUpdateForQuorumFromIndexReturn(pub OperatorStakeUpdate);
    ///Container type for all return fields from the `minimumStakeForQuorum` function with signature `minimumStakeForQuorum(uint256)` and selector `0x7ed9430f`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct MinimumStakeForQuorumReturn(pub u128);
    ///Container type for all return fields from the `registryCoordinator` function with signature `registryCoordinator()` and selector `0x6d14a987`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct RegistryCoordinatorReturn(pub ::ethers::core::types::Address);
}
