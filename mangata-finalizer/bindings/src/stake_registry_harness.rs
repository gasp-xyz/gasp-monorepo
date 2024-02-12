pub use stake_registry_harness::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod stake_registry_harness {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_registryCoordinator"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "contract IRegistryCoordinator",
                            ),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_strategyManager"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IStrategyManager"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_serviceManager"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IServiceManager"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("MAX_QUORUM_COUNT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("MAX_QUORUM_COUNT"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MAX_WEIGHING_FUNCTION_LENGTH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MAX_WEIGHING_FUNCTION_LENGTH",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WEIGHTING_DIVISOR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("WEIGHTING_DIVISOR"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "addStrategiesConsideredAndMultipliers",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "addStrategiesConsideredAndMultipliers",
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
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_newStrategiesConsideredAndMultipliers",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IVoteWeigher.StrategyAndWeightingMultiplier[]",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("createQuorum"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createQuorum"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_strategiesConsideredAndMultipliers",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IVoteWeigher.StrategyAndWeightingMultiplier[]",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("delegation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("delegation"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IDelegationManager",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deregisterOperator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deregisterOperator"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getCurrentOperatorStakeForQuorum"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getCurrentOperatorStakeForQuorum",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint96"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getCurrentTotalStakeForQuorum"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getCurrentTotalStakeForQuorum",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint96"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "getLengthOfOperatorIdStakeHistoryForQuorum",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getLengthOfOperatorIdStakeHistoryForQuorum",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "getLengthOfTotalStakeHistoryForQuorum",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getLengthOfTotalStakeHistoryForQuorum",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "getMostRecentStakeUpdateByOperatorId",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getMostRecentStakeUpdateByOperatorId",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IStakeRegistry.OperatorStakeUpdate",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getOperatorIdToStakeHistory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getOperatorIdToStakeHistory",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IStakeRegistry.OperatorStakeUpdate[]",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "getStakeForOperatorIdForQuorumAtBlockNumber",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getStakeForOperatorIdForQuorumAtBlockNumber",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint96"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "getStakeForQuorumAtBlockNumberFromOperatorIdAndIndex",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint96"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "getStakeUpdateForQuorumFromOperatorIdAndIndex",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IStakeRegistry.OperatorStakeUpdate",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "getStakeUpdateIndexForOperatorIdForQuorumAtBlockNumber",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getStakeUpdateIndexForOperatorIdForQuorumAtBlockNumber",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "getTotalStakeAtBlockNumberFromIndex",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint96"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "getTotalStakeIndicesByQuorumNumbersAtBlockNumber",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "getTotalStakeUpdateForQuorumFromIndex",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IStakeRegistry.OperatorStakeUpdate",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_minimumStakeForQuorum",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint96[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_quorumStrategiesConsideredAndMultipliers",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IVoteWeigher.StrategyAndWeightingMultiplier[][]",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("minimumStakeForQuorum"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "minimumStakeForQuorum",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint96"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("modifyStrategyWeights"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "modifyStrategyWeights",
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
                                    name: ::std::borrow::ToOwned::to_owned("strategyIndices"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newMultipliers"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint96[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("quorumCount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("quorumCount"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("recordOperatorStakeUpdate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "recordOperatorStakeUpdate",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
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
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "operatorStakeUpdate",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IStakeRegistry.OperatorStakeUpdate",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint96"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("recordTotalStakeUpdate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "recordTotalStakeUpdate",
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
                                    name: ::std::borrow::ToOwned::to_owned("totalStakeUpdate"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IStakeRegistry.OperatorStakeUpdate",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("registerOperator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("registerOperatorNonCoordinator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "registerOperatorNonCoordinator",
                            ),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("registryCoordinator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "registryCoordinator",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IRegistryCoordinator",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "removeStrategiesConsideredAndMultipliers",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "removeStrategiesConsideredAndMultipliers",
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
                                    name: ::std::borrow::ToOwned::to_owned("indicesToRemove"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("serviceManager"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("serviceManager"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IServiceManager"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setMinimumStakeForQuorum"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setMinimumStakeForQuorum",
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
                                    name: ::std::borrow::ToOwned::to_owned("minimumStake"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint96"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setOperatorWeight"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setOperatorWeight"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("weight"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint96"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("slasher"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("slasher"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ISlasher"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "strategiesConsideredAndMultipliers",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "strategiesConsideredAndMultipliers",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("strategy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IStrategy"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("multiplier"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint96"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "strategiesConsideredAndMultipliersLength",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "strategiesConsideredAndMultipliersLength",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "strategyAndWeightingMultiplierForQuorumByIndex",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "strategyAndWeightingMultiplierForQuorumByIndex",
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IVoteWeigher.StrategyAndWeightingMultiplier",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("strategyManager"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("strategyManager"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IStrategyManager",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateOperatorStake"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateOperatorStake",
                            ),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint96"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint96"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateStakes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateStakes"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operators"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("weightOfOperatorForQuorum"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "weightOfOperatorForQuorum",
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
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint96"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Initialized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Initialized"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("version"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MinimumStakeForQuorumUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MinimumStakeForQuorumUpdated",
                            ),
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("QuorumCreated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("QuorumCreated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StakeUpdate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("StakeUpdate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StrategyAddedToQuorum"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "StrategyAddedToQuorum",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("strategy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StrategyMultiplierUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "StrategyMultiplierUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("strategy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("multiplier"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StrategyRemovedFromQuorum"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "StrategyRemovedFromQuorum",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("strategy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static STAKEREGISTRYHARNESS_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01 `@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@Qb\0B\xE48\x03\x80b\0B\xE4\x839\x81\x01`@\x81\x90Rb\0\x005\x91b\0\x02PV[\x82\x82\x82\x82\x82\x82\x81\x81\x81`\x01`\x01`\xA0\x1B\x03\x16`\xA0\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xDF\\\xF7#`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\0\x96W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\0\xBC\x91\x90b\0\x02\xA4V[`\x01`\x01`\xA0\x1B\x03\x16`\x80\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xB14Bq`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x01\x14W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x01:\x91\x90b\0\x02\xA4V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xC0R\x81\x16`\xE0Rb\0\x01Wb\0\x01uV[PPPP`\x01`\x01`\xA0\x1B\x03\x16a\x01\0RPb\0\x02\xCB\x94PPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15b\0\x01\xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\xFF\x90\x81\x16\x10\x15b\0\x025W`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x02MW`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15b\0\x02fW`\0\x80\xFD[\x83Qb\0\x02s\x81b\0\x027V[` \x85\x01Q\x90\x93Pb\0\x02\x86\x81b\0\x027V[`@\x85\x01Q\x90\x92Pb\0\x02\x99\x81b\0\x027V[\x80\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15b\0\x02\xB7W`\0\x80\xFD[\x81Qb\0\x02\xC4\x81b\0\x027V[\x93\x92PPPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa?\x95b\0\x03O`\09`\0\x81\x81a\x04(\x01R\x81\x81a\x08f\x01R\x81\x81a\x16\xCE\x01R\x81\x81a\x1Ay\x01Ra\x1BY\x01R`\0\x81\x81a\x02\xFB\x01R\x81\x81a\n<\x01R\x81\x81a\r\xCE\x01R\x81\x81a\x13\xAF\x01R\x81\x81a\x15\x1D\x01Ra\x16\t\x01R`\0a\x05\xAB\x01R`\0a\x03:\x01R`\0a\x06\x8A\x01Ra?\x95`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02SW`\x005`\xE0\x1C\x80c\x94Dr\xA9\x11a\x01FW\x80c\xBD)\xB8\xCD\x11a\0\xC3W\x80c\xDF\\\xF7#\x11a\0\x87W\x80c\xDF\\\xF7#\x14a\x06\x85W\x80c\xE1\x92\xE9\xAD\x14a\x06\xACW\x80c\xE2T'\xDD\x14a\x06\xCCW\x80c\xE8\x9C\n\0\x14a\x06\xDFW\x80c\xEB\x92\x19\x9C\x14a\x06\xF2W\x80c\xF5\x04X\x94\x14a\x07\x13W`\0\x80\xFD[\x80c\xBD)\xB8\xCD\x14a\x06&W\x80c\xC8)LV\x14a\x069W\x80c\xC8\xF79\xD0\x14a\x06LW\x80c\xCD\x05\r\x9C\x14a\x06_W\x80c\xCE\x97~\xC3\x14a\x06rW`\0\x80\xFD[\x80c\xA6sFg\x11a\x01\nW\x80c\xA6sFg\x14a\x05\x9EW\x80c\xB14Bq\x14a\x05\xA6W\x80c\xB37\x1B\xAA\x14a\x05\xCDW\x80c\xBB\xA5I\xFA\x14a\x06\0W\x80c\xBC\x9A@\xC3\x14a\x06\x13W`\0\x80\xFD[\x80c\x94Dr\xA9\x14a\x05\x0EW\x80c\x99\xEE\xD4\xEE\x14a\x05.W\x80c\x9A\xA1e=\x14a\x05AW\x80c\x9E\x8C\xA6 \x14a\x05hW\x80c\xA4<\xDE\x89\x14a\x05\x8BW`\0\x80\xFD[\x80c^Zgu\x11a\x01\xD4W\x80c|\x17#G\x11a\x01\x98W\x80c|\x17#G\x14a\x04{W\x80c|\xA9\xFF\0\x14a\x04\x95W\x80c|\xC0\xD7_\x14a\x04\xA8W\x80c~\xD9C\x0F\x14a\x04\xE8W\x80c\x90)\x04}\x14a\x04\xFBW`\0\x80\xFD[\x80c^Zgu\x14a\x03\xE0W\x80c_)H\xEC\x14a\x03\xFDW\x80cj\xB58\xD4\x14a\x04\x10W\x80cm\x14\xA9\x87\x14a\x04#W\x80cn\x8F\x03\xCA\x14a\x04JW`\0\x80\xFD[\x80c,*]+\x11a\x02\x1BW\x80c,*]+\x14a\x02\xE3W\x80c9\x98\xFD\xD3\x14a\x02\xF6W\x80c9\xB7\x0E8\x14a\x035W\x80cH\x08Xf\x14a\x03\\W\x80cL\xAFH\xC2\x14a\x03\x84W`\0\x80\xFD[\x80c\x1B2r%\x14a\x02XW\x80c\x1F\x9Bt\xE0\x14a\x02\x88W\x80c$\x8Des\x14a\x02\x9BW\x80c%PGw\x14a\x02\xBBW\x80c+=\x88\x16\x14a\x02\xD0W[`\0\x80\xFD[a\x02ka\x02f6`\x04a1~V[a\x07&V[`@Q`\x01`\x01``\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02ka\x02\x966`\x04a1\xCFV[a\x07\x8AV[a\x02\xAEa\x02\xA96`\x04a2\x06V[a\x07\xC2V[`@Qa\x02\x7F\x91\x90a29V[a\x02\xCEa\x02\xC96`\x04a2\xB6V[a\x08[V[\0[a\x02\xCEa\x02\xDE6`\x04a3UV[a\n:V[a\x02\xCEa\x02\xF16`\x04a3\xA7V[a\r\xCCV[a\x03\x1D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x7FV[a\x03\x1D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03oa\x03j6`\x04a1~V[a\x11\x02V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\x7FV[a\x02\xCEa\x03\x926`\x04a4>V[`\xFF\x90\x92\x16`\0\x90\x81Ra\x01\xF5` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x83R\x92\x90R \x80T`\x01`\x01``\x1B\x03\x90\x92\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x03\xEFg\r\xE0\xB6\xB3\xA7d\0\0\x81V[`@Q\x90\x81R` \x01a\x02\x7FV[a\x02\xCEa\x04\x0B6`\x04a6)V[a\x11\x17V[a\x02ka\x04\x1E6`\x04a6\xE8V[a\x12,V[a\x03\x1D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\xEFa\x04X6`\x04a6\xE8V[`\0\x91\x82Ra\x01\xB3` \x90\x81R`@\x80\x84 `\xFF\x93\x90\x93\x16\x84R\x91\x90R\x90 T\x90V[a\x04\x83` \x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02\x7FV[a\x02ka\x04\xA36`\x04a7\x82V[a\x12EV[a\x04\xBBa\x04\xB66`\x04a7\xB6V[a\x12RV[`@\x80Q\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x92\x83\x01Q`\x01`\x01``\x1B\x03\x16\x92\x81\x01\x92\x90\x92R\x01a\x02\x7FV[a\x02ka\x04\xF66`\x04a7\xE0V[a\x12\xCCV[a\x02\xCEa\x05\t6`\x04a7\xF9V[a\x13\0V[a\x05!a\x05\x1C6`\x04a6\xE8V[a\x13\x0EV[`@Qa\x02\x7F\x91\x90a8$V[a\x02\xCEa\x05<6`\x04a8\x9CV[a\x13\xADV[`\0Ta\x05U\x90b\x01\0\0\x90\x04a\xFF\xFF\x16\x81V[`@Qa\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\x7FV[a\x03\xEFa\x05v6`\x04a8\xD0V[`\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[a\x02ka\x05\x996`\x04a8\xEBV[a\x14kV[a\x04\x83`\xC0\x81V[a\x03\x1D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x05\xE0a\x05\xDB6`\x04a9-V[a\x15\x01V[`@\x80Q`\x01`\x01``\x1B\x03\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01a\x02\x7FV[a\x02\xCEa\x06\x0E6`\x04a9bV[a\x15\x1BV[a\x02\xCEa\x06!6`\x04a9\xA5V[a\x16\x07V[a\x02\xCEa\x0646`\x04a9\xCFV[a\x16\xC3V[a\x02ka\x06G6`\x04a:\rV[a\x18ZV[a\x02ka\x06Z6`\x04a8\xD0V[a\x18\xDDV[a\x02\xAEa\x06m6`\x04a6\xE8V[a\x19NV[a\x02\xCEa\x06\x806`\x04a:IV[a\x1A5V[a\x03\x1D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x06\xBFa\x06\xBA6`\x04a:\x8AV[a\x1D\x15V[`@Qa\x02\x7F\x91\x90a:\xC3V[a\x02\xAEa\x06\xDA6`\x04a7\xB6V[a\x1F\x82V[a\x03\xEFa\x06\xED6`\x04a8\xD0V[a \x1AV[a\x07\x05a\x07\x006`\x04a7\xB6V[a ;V[`@Qa\x02\x7F\x92\x91\x90a;\x01V[a\x02\xCEa\x07!6`\x04a2\xB6V[a \x84V[`\0\x83\x81Ra\x01\xB3` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 a\x07N\x85\x85\x85a\"\nV[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x07dWa\x07da;#V[`\0\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90P[\x93\x92PPPV[`\xFF\x82\x16`\0\x90\x81Ra\x01\xF5` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 T`\x01`\x01``\x1B\x03\x16[\x92\x91PPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R\x85\x82Ra\x01\xB3\x81R\x83\x82 `\xFF\x88\x16\x83R\x90R\x91\x90\x91 \x80T\x83\x90\x81\x10a\x08\x08Wa\x08\x08a;#V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x93\x83\x01\x93\x90\x93R`\x01`@\x1B\x90\x92\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x94\x93PPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x08\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a;9V[`@Q\x80\x91\x03\x90\xFD[`\0Tb\x01\0\0\x90\x04a\xFF\xFF\x16\x82\x82a\x08\xC6`\x01\x82a;\xC1V[\x81\x81\x10a\x08\xD5Wa\x08\xD5a;#V[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C`\xFF\x16\x10a\t\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a;\xD8V[`@\x80Q``\x81\x01\x82R`\0` \x82\x01\x81\x90R\x91\x81\x01\x82\x90RCc\xFF\xFF\xFF\xFF\x16\x81R\x90[`\xFF\x81\x16\x83\x11\x15a\n2W`\0\x84\x84\x83`\xFF\x16\x81\x81\x10a\tHWa\tHa;#V[\x91\x90\x91\x015`\xF8\x1C\x91P`\0\x90Pa\ta\x88\x88\x84a%1V[\x91PP`\x01`\x01``\x1B\x03\x81\x16a\t\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a<RV[`\0`\xB3\x83`\xFF\x16a\x01\0\x81\x10a\t\xA3Wa\t\xA3a;#V[\x01T\x90P\x81\x81\x15a\n\nW`\xB3\x84`\xFF\x16a\x01\0\x81\x10a\t\xC5Wa\t\xC5a;#V[\x01a\t\xD1`\x01\x84a;\xC1V[\x81T\x81\x10a\t\xE1Wa\t\xE1a;#V[`\0\x91\x82R` \x90\x91 \x01Ta\n\x07\x90`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x82a<\xD5V[\x90P[`\x01`\x01``\x1B\x03\x81\x16`@\x87\x01Ra\n#\x84\x87a&6V[\x84`\x01\x01\x94PPPPPa\t&V[PPPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\x98W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xBC\x91\x90a=\0V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\n\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a=\x1DV[`\0T\x83\x90b\x01\0\0\x90\x04a\xFF\xFF\x16`\xFF\x82\x16\x10a\x0B\x1CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a=\x99V[\x81\x80a\x0B\xB6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`W`$\x82\x01R\x7FVoteWeigherBase.removeStrategies`D\x82\x01R\x7FConsideredAndMultipliers: no ind`d\x82\x01R\x7Fices to remove provided\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x08\xA3V[`\0[\x81\x81\x10\x15a\n2W`\xFF\x86\x16`\0\x81\x81R`\x01` R`@\x90 \x7F1\xFA.,\xD2\x80\xC97^\x13\xFF\xCF=\x81\xE27\x81\0\x18n@X\xF8\xD3\xDD\xB6\x90\xB8-\xCD1\xF7\x90\x87\x87\x85\x81\x81\x10a\x0C\x07Wa\x0C\x07a;#V[\x90P` \x02\x015\x81T\x81\x10a\x0C\x1EWa\x0C\x1Ea;#V[`\0\x91\x82R` \x91\x82\x90 \x01T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x01`@Q\x80\x91\x03\x90\xA2`\xFF\x86\x16`\0\x81\x81R`\x01` R`@\x90 \x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x90\x87\x87\x85\x81\x81\x10a\x0C\x8AWa\x0C\x8Aa;#V[\x90P` \x02\x015\x81T\x81\x10a\x0C\xA1Wa\x0C\xA1a;#V[`\0\x91\x82R` \x80\x83 \x91\x90\x91\x01T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82R\x91\x81\x01\x92\x90\x92R\x01`@Q\x80\x91\x03\x90\xA2`\xFF\x86\x16`\0\x90\x81R`\x01` \x81\x90R`@\x90\x91 \x80T\x90\x91a\x0C\xF3\x91a;\xC1V[\x81T\x81\x10a\r\x03Wa\r\x03a;#V[`\0\x91\x82R` \x80\x83 `\xFF\x8A\x16\x84R`\x01\x90\x91R`@\x90\x92 \x91\x01\x90\x86\x86\x84\x81\x81\x10a\r2Wa\r2a;#V[\x90P` \x02\x015\x81T\x81\x10a\rIWa\rIa;#V[`\0\x91\x82R` \x80\x83 \x84T\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92\x83\x17\x81U\x93T`\x01`\x01``\x1B\x03`\x01`\xA0\x1B\x91\x82\x90\x04\x16\x02\x90\x91\x17\x90\x92U`\xFF\x88\x16\x81R`\x01\x90\x91R`@\x90 \x80T\x80a\r\xACWa\r\xACa=\xF6V[`\0\x82\x81R` \x81 \x82\x01`\0\x19\x90\x81\x01\x91\x90\x91U\x01\x90U`\x01\x01a\x0B\xB9V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E*W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0EN\x91\x90a=\0V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0E~W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a=\x1DV[`\0T\x85\x90b\x01\0\0\x90\x04a\xFF\xFF\x16`\xFF\x82\x16\x10a\x0E\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a=\x99V[\x83\x80a\x0F.W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FVoteWeigherBase.modifyStrategyWe`D\x82\x01R\x7Fights: no strategy indices provi`d\x82\x01Rb\x19\x19Y`\xEA\x1B`\x84\x82\x01R`\xA4\x01a\x08\xA3V[\x82\x81\x14a\x0F\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R\x7FVoteWeigherBase.modifyStrategyWe`D\x82\x01R\x7Fights: input length mismatch\0\0\0\0`d\x82\x01R`\x84\x01a\x08\xA3V[`\0[\x81\x81\x10\x15a\x10\xF8W\x84\x84\x82\x81\x81\x10a\x0F\xC0Wa\x0F\xC0a;#V[\x90P` \x02\x01` \x81\x01\x90a\x0F\xD5\x91\x90a>\x0CV[`\xFF\x89\x16`\0\x90\x81R`\x01` R`@\x90 \x88\x88\x84\x81\x81\x10a\x0F\xF9Wa\x0F\xF9a;#V[\x90P` \x02\x015\x81T\x81\x10a\x10\x10Wa\x10\x10a;#V[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x80T`\x01`\x01``\x1B\x03\x94\x90\x94\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x93\x90\x93\x17\x90\x92U`\xFF\x8A\x16\x80\x82R`\x01\x90\x92R`@\x90 \x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x90\x89\x89\x85\x81\x81\x10a\x10\x87Wa\x10\x87a;#V[\x90P` \x02\x015\x81T\x81\x10a\x10\x9EWa\x10\x9Ea;#V[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x87\x87\x85\x81\x81\x10a\x10\xC5Wa\x10\xC5a;#V[\x90P` \x02\x01` \x81\x01\x90a\x10\xDA\x91\x90a>\x0CV[`@Qa\x10\xE8\x92\x91\x90a;\x01V[`@Q\x80\x91\x03\x90\xA2`\x01\x01a\x0F\xA6V[PPPPPPPPV[`\0a\x11\x0F\x84\x84\x84a\"\nV[\x94\x93PPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x117WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x11QWP0;\x15\x80\x15a\x11QWP`\0T`\xFF\x16`\x01\x14[a\x11\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x08\xA3V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x11\xD7W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x11\xE1\x83\x83a'PV[\x80\x15a\x12'W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPV[`\0\x80a\x129\x84\x84a\x19NV[`@\x01Q\x94\x93PPPPV[`\0a\x11\x0F\x84\x84\x84a(\x94V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\xFF\x83\x16`\0\x90\x81R`\x01` R`@\x90 \x80T\x83\x90\x81\x10a\x12\x8AWa\x12\x8Aa;#V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[`3\x81a\x01\0\x81\x10a\x12\xDDW`\0\x80\xFD[`\x02\x91\x82\x82\x04\x01\x91\x90\x06`\x0C\x02\x91PT\x90a\x01\0\n\x90\x04`\x01`\x01``\x1B\x03\x16\x81V[a\x13\n\x82\x82a&6V[PPV[`\0\x82\x81Ra\x01\xB3` \x90\x81R`@\x80\x83 `\xFF\x85\x16\x84R\x82R\x80\x83 \x80T\x82Q\x81\x85\x02\x81\x01\x85\x01\x90\x93R\x80\x83R``\x94\x92\x93\x91\x92\x90\x91\x84\x01[\x82\x82\x10\x15a\x13\xA1W`\0\x84\x81R` \x90\x81\x90 `@\x80Q``\x81\x01\x82R\x91\x85\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x83\x85\x01R`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90\x82\x01R\x82R`\x01\x90\x92\x01\x91\x01a\x13HV[PPPP\x90P\x92\x91PPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\x0BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14/\x91\x90a=\0V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x14_W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a=\x1DV[a\x14h\x81a*\x0CV[PV[`\0\x82\x81Ra\x01\xB3` \x90\x81R`@\x80\x83 `\xFF\x88\x16\x84R\x90\x91R\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x14\x9DWa\x14\x9Da;#V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90Pa\x14\xF4\x81\x86a+\x0CV[`@\x01Q\x95\x94PPPPPV[`\0\x80a\x15\x0F\x85\x85\x85a%1V[\x91P\x91P\x93P\x93\x91PPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15yW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x9D\x91\x90a=\0V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x15\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a=\x1DV[`\0T\x82\x90b\x01\0\0\x90\x04a\xFF\xFF\x16`\xFF\x82\x16\x10a\x15\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a=\x99V[a\x12'\x83\x83a,\x8DV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16eW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\x89\x91\x90a=\0V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x16\xB9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a=\x1DV[a\x13\n\x82\x82a0\xBFV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x17\x0BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a;9V[`@\x80Q``\x80\x82\x01\x83R`\0` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90Rc\xFF\xFF\xFF\xFFC\x16\x80\x85R\x85Q\x93\x84\x01\x86R\x90\x83\x01\x82\x90R\x93\x82\x01\x81\x90R\x92\x81R\x90\x91[`\xFF\x81\x16\x84\x11\x15a\n2W`\0\x85\x85\x83`\xFF\x16\x81\x81\x10a\x17kWa\x17ka;#V[\x91\x90\x91\x015`\xF8\x1C\x91P`\0\x90Pa\x17\x84\x88\x83\x87a(\x94V[\x90P\x80`\xB3\x83`\xFF\x16a\x01\0\x81\x10a\x17\x9EWa\x17\x9Ea;#V[\x01`\x01`\xB3\x85`\xFF\x16a\x01\0\x81\x10a\x17\xB8Wa\x17\xB8a;#V[\x01Ta\x17\xC4\x91\x90a;\xC1V[\x81T\x81\x10a\x17\xD4Wa\x17\xD4a;#V[`\0\x91\x82R` \x90\x91 \x01Ta\x17\xFA\x91\x90`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16a>'V[`\x01`\x01``\x1B\x03\x16`@\x85\x01Ra\x18\x12\x82\x85a&6V[`@\x80Q`\xFF\x84\x16\x81R`\0` \x82\x01R\x89\x91\x7F\xE7\xC6\x0CRi/\x0E\x0F\xF8\xD1\xC2\x89\xFF\xE6<\xB0.y7?5\xE3\0\x0C\xAF\xE7\xCC\xA8\x98\x88Y4\x91\x01`@Q\x80\x91\x03\x90\xA2PP`\x01\x01a\x17IV[`\0\x80`\xB3\x85`\xFF\x16a\x01\0\x81\x10a\x18tWa\x18ta;#V[\x01\x83\x81T\x81\x10a\x18\x86Wa\x18\x86a;#V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90Pa\x129\x81\x85a+\x0CV[`\0`\xB3\x82`\xFF\x16a\x01\0\x81\x10a\x18\xF6Wa\x18\xF6a;#V[\x01`\x01`\xB3\x84`\xFF\x16a\x01\0\x81\x10a\x19\x10Wa\x19\x10a;#V[\x01Ta\x19\x1C\x91\x90a;\xC1V[\x81T\x81\x10a\x19,Wa\x19,a;#V[`\0\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x92\x91PPV[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R\x86\x82Ra\x01\xB3\x81R\x84\x82 `\xFF\x87\x16\x83R\x81R\x84\x82 T\x85Q\x93\x84\x01\x86R\x82\x84R\x90\x83\x01\x82\x90R\x93\x82\x01R\x90\x91\x90\x81a\x19\xA8W\x91Pa\x07\xBC\x90PV[`\0\x85\x81Ra\x01\xB3` \x90\x81R`@\x80\x83 `\xFF\x88\x16\x84R\x90\x91R\x90 a\x19\xD0`\x01\x84a;\xC1V[\x81T\x81\x10a\x19\xE0Wa\x19\xE0a;#V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x92Pa\x07\xBC\x91PPV[`\0[`\0Tb\x01\0\0\x90\x04a\xFF\xFF\x16`\xFF\x82\x16\x10\x15a\x12'W`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R\x90[\x83\x81\x10\x15a\x1C\xF4W`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x13T*N\x87\x87\x85\x81\x81\x10a\x1A\xB8Wa\x1A\xB8a;#V[\x90P` \x02\x01` \x81\x01\x90a\x1A\xCD\x91\x90a>OV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\x11W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B5\x91\x90a>lV[`@Qc41\xAF%`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c41\xAF%\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xA0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xC4\x91\x90a>\x85V[\x90P`\x01`\x01`\x01`\xC0\x1B\x03\x82\x16`\xFF\x87\x16\x1C\x81\x16\x14\x15a\x1C\xEAW\x83Qc\xFF\xFF\xFF\xFF\x16a\x1C\x86W`\xB3\x85`\xFF\x16a\x01\0\x81\x10a\x1C\x02Wa\x1C\x02a;#V[\x01`\x01`\xB3\x87`\xFF\x16a\x01\0\x81\x10a\x1C\x1CWa\x1C\x1Ca;#V[\x01Ta\x1C(\x91\x90a;\xC1V[\x81T\x81\x10a\x1C8Wa\x1C8a;#V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x93P[`\0\x80a\x1C\xBA\x89\x89\x87\x81\x81\x10a\x1C\x9EWa\x1C\x9Ea;#V[\x90P` \x02\x01` \x81\x01\x90a\x1C\xB3\x91\x90a>OV[\x85\x89a%1V[\x91P\x91P\x80\x82\x87`@\x01Qa\x1C\xCF\x91\x90a>'V[a\x1C\xD9\x91\x90a<\xD5V[`\x01`\x01``\x1B\x03\x16`@\x87\x01RPP[PP`\x01\x01a\x1AmV[P\x80Qc\xFF\xFF\xFF\xFF\x16\x15a\x1D\x0CWa\x1D\x0C\x82\x82a&6V[P`\x01\x01a\x1A8V[```\0\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D1Wa\x1D1a4zV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1DZW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83\x81\x10\x15a\x1FyW`\0\x85\x85\x83\x81\x81\x10a\x1D|Wa\x1D|a;#V[\x91\x90\x91\x015`\xF8\x1C\x91PPc\xFF\xFF\xFF\xFF\x87\x16`\xB3\x82a\x01\0\x81\x10a\x1D\xA2Wa\x1D\xA2a;#V[\x01`\0\x81T\x81\x10a\x1D\xB5Wa\x1D\xB5a;#V[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a\x1EtW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`j`$\x82\x01R\x7FStakeRegistry.getTotalStakeIndic`D\x82\x01R\x7FesByQuorumNumbersAtBlockNumber: `d\x82\x01R\x7Fquorum has no stake history at b`\x84\x82\x01Ri67\xB1\xB5\xA7:\xB6\xB12\xB9`\xB1\x1B`\xA4\x82\x01R`\xC4\x01a\x08\xA3V[`\0`\xB3\x82`\xFF\x16a\x01\0\x81\x10a\x1E\x8DWa\x1E\x8Da;#V[\x01T\x90P`\0[\x81c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x1FcW\x88c\xFF\xFF\xFF\xFF\x16`\xB3\x84`\xFF\x16a\x01\0\x81\x10a\x1E\xC6Wa\x1E\xC6a;#V[\x01`\x01a\x1E\xD3\x84\x86a>\xAEV[a\x1E\xDD\x91\x90a>\xAEV[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x1E\xF3Wa\x1E\xF3a;#V[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a\x1FQW`\x01a\x1F\x16\x82\x84a>\xAEV[a\x1F \x91\x90a>\xAEV[\x85\x85\x81Q\x81\x10a\x1F2Wa\x1F2a;#V[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPPa\x1FcV[\x80a\x1F[\x81a>\xCBV[\x91PPa\x1E\x94V[PPP\x80\x80a\x1Fq\x90a>\xEFV[\x91PPa\x1D`V[P\x94\x93PPPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R`\xB3\x83`\xFF\x16a\x01\0\x81\x10a\x1F\xB6Wa\x1F\xB6a;#V[\x01\x82\x81T\x81\x10a\x1F\xC8Wa\x1F\xC8a;#V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x93\x83\x01\x93\x90\x93R`\x01`@\x1B\x90\x92\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[`\0`\xB3\x82`\xFF\x16a\x01\0\x81\x10a 3Wa 3a;#V[\x01T\x92\x91PPV[`\x01` R\x81`\0R`@`\0 \x81\x81T\x81\x10a WW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x92P`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90P\x82V[`\0Tb\x01\0\0\x90\x04a\xFF\xFF\x16\x82\x82a \x9E`\x01\x82a;\xC1V[\x81\x81\x10a \xADWa \xADa;#V[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C`\xFF\x16\x10a \xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a;\xD8V[`@\x80Q``\x81\x01\x82R`\0` \x82\x01\x81\x90R\x91\x81\x01\x82\x90RCc\xFF\xFF\xFF\xFF\x16\x81R\x90[`\xFF\x81\x16\x83\x11\x15a\n2W`\0\x84\x84\x83`\xFF\x16\x81\x81\x10a! Wa! a;#V[\x91\x90\x91\x015`\xF8\x1C\x91P`\0\x90Pa!9\x88\x88\x84a%1V[\x91PP`\x01`\x01``\x1B\x03\x81\x16a!bW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a<RV[`\0`\xB3\x83`\xFF\x16a\x01\0\x81\x10a!{Wa!{a;#V[\x01T\x90P\x81\x81\x15a!\xE2W`\xB3\x84`\xFF\x16a\x01\0\x81\x10a!\x9DWa!\x9Da;#V[\x01a!\xA9`\x01\x84a;\xC1V[\x81T\x81\x10a!\xB9Wa!\xB9a;#V[`\0\x91\x82R` \x90\x91 \x01Ta!\xDF\x90`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x82a<\xD5V[\x90P[`\x01`\x01``\x1B\x03\x81\x16`@\x87\x01Ra!\xFB\x84\x87a&6V[\x84`\x01\x01\x94PPPPPa \xFEV[`\0\x83\x81Ra\x01\xB3` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 T\x81[\x81c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a$aW`\0\x86\x81Ra\x01\xB3` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x90\x91R\x90 c\xFF\xFF\xFF\xFF\x85\x16\x90`\x01a\"o\x84\x86a>\xAEV[a\"y\x91\x90a>\xAEV[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\"\x8FWa\"\x8Fa;#V[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a$OW`\0\x86\x81Ra\x01\xB3` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x90\x91R\x90 `\x01a\"\xCF\x83\x85a>\xAEV[a\"\xD9\x91\x90a>\xAEV[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\"\xEFWa\"\xEFa;#V[`\0\x91\x82R` \x90\x91 \x01T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15\x80a#{WP`\0\x86\x81Ra\x01\xB3` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x90\x91R\x90 c\xFF\xFF\xFF\xFF\x85\x16\x90`\x01a#@\x84\x86a>\xAEV[a#J\x91\x90a>\xAEV[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a#`Wa#`a;#V[`\0\x91\x82R` \x90\x91 \x01T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x11[a$0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`t`$\x82\x01R\x7FStakeRegistry._getStakeUpdateInd`D\x82\x01R\x7FexForOperatorIdForQuorumAtBlockN`d\x82\x01R\x7Fumber: operatorId has no stake u`\x84\x82\x01Rs820\xBA2\x900\xBA\x10167\xB1\xB5\xA7:\xB6\xB12\xB9`a\x1B`\xA4\x82\x01R`\xC4\x01a\x08\xA3V[`\x01a$<\x82\x84a>\xAEV[a$F\x91\x90a>\xAEV[\x92PPPa\x07\x83V[\x80a$Y\x81a>\xCBV[\x91PPa\"*V[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x8C`$\x82\x01R\x7FStakeRegistry._getStakeUpdateInd`D\x82\x01R\x7FexForOperatorIdForQuorumAtBlockN`d\x82\x01R\x7Fumber: no stake update found for`\x84\x82\x01R\x7F operatorId and quorumNumber at `\xA4\x82\x01Rk167\xB1\xB5\x907:\xB6\xB12\xB9`\xA1\x1B`\xC4\x82\x01R`\xE4\x01a\x08\xA3V[`@\x80Q``\x81\x01\x82R`\0` \x82\x01\x81\x90R\x91\x81\x01\x82\x90RCc\xFF\xFF\xFF\xFF\x16\x81R\x81\x90a%_\x84\x87a\x07\x8AV[`\x01`\x01``\x1B\x03\x16`@\x82\x01R`3`\xFF\x85\x16a\x01\0\x81\x10a%\x84Wa%\x84a;#V[`\x02\x91\x82\x82\x04\x01\x91\x90\x06`\x0C\x02\x90T\x90a\x01\0\n\x90\x04`\x01`\x01``\x1B\x03\x16`\x01`\x01``\x1B\x03\x16\x81`@\x01Q`\x01`\x01``\x1B\x03\x16\x10\x15a%\xC8W`\0`@\x82\x01R[`\0a%\xD5\x86\x86\x84a(\x94V[`@\x80\x84\x01Q\x81Q`\xFF\x89\x16\x81R`\x01`\x01``\x1B\x03\x90\x91\x16` \x82\x01R\x91\x92P\x87\x91\x7F\xE7\xC6\x0CRi/\x0E\x0F\xF8\xD1\xC2\x89\xFF\xE6<\xB0.y7?5\xE3\0\x0C\xAF\xE7\xCC\xA8\x98\x88Y4\x91\x01`@Q\x80\x91\x03\x90\xA2`@\x90\x91\x01Q\x90\x92P\x90P\x93P\x93\x91PPV[`\0`\xB3\x83`\xFF\x16a\x01\0\x81\x10a&OWa&Oa;#V[\x01T\x90P\x80\x15a&\xB9WC`\xB3\x84`\xFF\x16a\x01\0\x81\x10a&qWa&qa;#V[\x01a&}`\x01\x84a;\xC1V[\x81T\x81\x10a&\x8DWa&\x8Da;#V[\x90`\0R` `\0 \x01`\0\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP[c\xFF\xFF\xFF\xFFC\x16\x82R`\xB3`\xFF\x84\x16a\x01\0\x81\x10a&\xD9Wa&\xD9a;#V[\x01\x80T`\x01\x81\x01\x82U`\0\x91\x82R` \x91\x82\x90 \x84Q\x91\x01\x80T\x92\x85\x01Q`@\x90\x95\x01Q`\x01`\x01``\x1B\x03\x16`\x01`@\x1B\x02k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x1B\x19c\xFF\xFF\xFF\xFF\x96\x87\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x95\x16\x96\x90\x93\x16\x95\x90\x95\x17\x92\x90\x92\x17\x16\x92\x90\x92\x17\x90\x91UPPV[`\0Ta\x01\0\x90\x04`\xFF\x16a'\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x08\xA3V[\x80Q\x82Q\x14a(2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`;`$\x82\x01R\x7FRegistry._initialize: minimumSta`D\x82\x01R\x7FkeForQuorum length mismatch\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08\xA3V[`\0[\x81Q\x81`\xFF\x16\x10\x15a\x12'Wa(g\x81\x84\x83`\xFF\x16\x81Q\x81\x10a(ZWa(Za;#V[` \x02` \x01\x01Qa0\xBFV[a(\x8C\x82\x82`\xFF\x16\x81Q\x81\x10a(\x7FWa(\x7Fa;#V[` \x02` \x01\x01Qa*\x0CV[`\x01\x01a(5V[`\0\x83\x81Ra\x01\xB3` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 T\x81\x90\x80\x15a)wW`\0\x86\x81Ra\x01\xB3` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x90\x91R\x90 C\x90a(\xE4`\x01\x84a;\xC1V[\x81T\x81\x10a(\xF4Wa(\xF4a;#V[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x80Tc\xFF\xFF\xFF\xFF\x94\x90\x94\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x94\x16\x93\x90\x93\x17\x90\x92U\x87\x81Ra\x01\xB3\x82R`@\x80\x82 `\xFF\x89\x16\x83R\x90\x92R a)H`\x01\x83a;\xC1V[\x81T\x81\x10a)XWa)Xa;#V[`\0\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x91P[P`\0\x85\x81Ra\x01\xB3` \x90\x81R`@\x80\x83 `\xFF\x88\x16\x84R\x82R\x80\x83 \x80T`\x01\x81\x01\x82U\x90\x84R\x92\x82\x90 \x86Q\x93\x01\x80T\x92\x87\x01Q\x91\x87\x01Q`\x01`\x01``\x1B\x03\x16`\x01`@\x1B\x02k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x1B\x19c\xFF\xFF\xFF\xFF\x93\x84\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x95\x16\x93\x90\x95\x16\x92\x90\x92\x17\x92\x90\x92\x17\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x90P\x93\x92PPPV[`\0Tb\x01\0\0\x90\x04a\xFF\xFF\x16`\xC0\x81\x10a*\xA7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`O`$\x82\x01R\x7FVoteWeigherBase._createQuorum: n`D\x82\x01R\x7Fumber of quorums cannot exceed M`d\x82\x01Rn\x10V\x17\xD4US\xD4\x95SW\xD0\xD3\xD5S\x95`\x8A\x1B`\x84\x82\x01R`\xA4\x01a\x08\xA3V[\x80a*\xB3\x81`\x01a?\nV[`\0`\x02a\x01\0\n\x81T\x81a\xFF\xFF\x02\x19\x16\x90\x83a\xFF\xFF\x16\x02\x17\x90UPa*\xD9\x81\x84a,\x8DV[`@Q`\xFF\x82\x16\x90\x7F\x83\x1A\x9C\x86\xC4[\xB3\x03\xCA\xF3\xF0d\xBE+\xC2\xB9\xFDN\xCF\x19\xE4|J\xC0*a\xE7]\xAB\xFEU\xB4\x90`\0\x90\xA2PPPV[\x81Qc\xFF\xFF\xFF\xFF\x80\x83\x16\x91\x16\x11\x15a+\xB2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R```$\x82\x01R\x7FStakeRegistry._validateOperatorS`D\x82\x01R\x7FtakeAtBlockNumber: operatorStake`d\x82\x01R\x7FUpdate is from after blockNumber`\x84\x82\x01R`\xA4\x01a\x08\xA3V[` \x82\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x80a+\xD8WP\x80c\xFF\xFF\xFF\xFF\x16\x82` \x01Qc\xFF\xFF\xFF\xFF\x16\x11[a\x13\nW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`t`$\x82\x01R\x7FStakeRegistry._validateOperatorS`D\x82\x01R\x7FtakeAtBlockNumber: there is a ne`d\x82\x01R\x7Fwer operatorStakeUpdate availabl`\x84\x82\x01Rs2\x9012\xB37\xB92\x90167\xB1\xB5\xA7:\xB6\xB12\xB9`a\x1B`\xA4\x82\x01R`\xC4\x01a\x08\xA3V[`\0\x81Q\x11a-\tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`N`$\x82\x01R`\0\x80Q` a?@\x839\x81Q\x91R`D\x82\x01R\x7FnsideredAndMultipliers: no strat`d\x82\x01Rm\x19Y\xDAY\\\xC8\x1C\x1C\x9B\xDD\x9AY\x19Y`\x92\x1B`\x84\x82\x01R`\xA4\x01a\x08\xA3V[\x80Q`\xFF\x83\x16`\0\x90\x81R`\x01` \x90\x81R`@\x90\x91 T\x90a-,\x83\x83a?'V[\x11\x15a-\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`[`$\x82\x01R`\0\x80Q` a?@\x839\x81Q\x91R`D\x82\x01R\x7FnsideredAndMultipliers: exceed M`d\x82\x01R\x7FAX_WEIGHING_FUNCTION_LENGTH\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x08\xA3V[`\0[\x82\x81\x10\x15a0\xB8W`\0[a-\xCC\x82\x84a?'V[\x81\x10\x15a.\xBFW\x84\x82\x81Q\x81\x10a-\xE5Wa-\xE5a;#V[` \x02` \x01\x01Q`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\0\x88`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x82\x81T\x81\x10a.$Wa.$a;#V[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a.\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`S`$\x82\x01R`\0\x80Q` a?@\x839\x81Q\x91R`D\x82\x01R\x7FnsideredAndMultipliers: cannot a`d\x82\x01Rr\x0C\x8C\x84\x0El-\xAC\xA4\x0En\x8EL.\x8C\xAC\xEF$\x06O`k\x1B`\x84\x82\x01R`\xA4\x01a\x08\xA3V[`\x01\x01a-\xC2V[P`\0\x84\x82\x81Q\x81\x10a.\xD4Wa.\xD4a;#V[` \x02` \x01\x01Q` \x01Q`\x01`\x01``\x1B\x03\x16\x11a/pW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R`\0\x80Q` a?@\x839\x81Q\x91R`D\x82\x01R\x7FnsideredAndMultipliers: cannot a`d\x82\x01R\x7Fdd strategy with zero weight\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x08\xA3V[`\xFF\x85\x16`\0\x90\x81R`\x01` R`@\x90 \x84Q\x85\x90\x83\x90\x81\x10a/\x96Wa/\x96a;#V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82T`\x01\x81\x01\x84U`\0\x93\x84R\x92\x82\x90 \x81Q\x91\x90\x92\x01Q`\x01`\x01``\x1B\x03\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x91\x01U\x83Q`\xFF\x86\x16\x90\x7F\x10V^V\xCA\xCB\xF3.\xCA&yE\xF0T\xFE\xC0.Yu\x002\xD1\x13\xD30!\x82\xAD\x96\x7FT\x04\x90\x86\x90\x84\x90\x81\x10a0\x13Wa0\x13a;#V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQ`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x01`@Q\x80\x91\x03\x90\xA2\x84`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x85\x83\x81Q\x81\x10a0pWa0pa;#V[` \x02` \x01\x01Q`\0\x01Q\x86\x84\x81Q\x81\x10a0\x8EWa0\x8Ea;#V[` \x02` \x01\x01Q` \x01Q`@Qa0\xA8\x92\x91\x90a;\x01V[`@Q\x80\x91\x03\x90\xA2`\x01\x01a-\xB7V[PPPPPV[\x80`3\x83`\xFF\x16a\x01\0\x81\x10a0\xD7Wa0\xD7a;#V[`\x02\x91\x82\x82\x04\x01\x91\x90\x06`\x0C\x02a\x01\0\n\x81T\x81`\x01`\x01``\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01``\x1B\x03\x16\x02\x17\x90UP\x81`\xFF\x16\x7F&\xEE\xCF\xF2\xB7\x0B\nq\x10O\xF4\xD9@\xBAqb\xD2:\x95\xC2Hw\x1F\xC4\x87\xA7\xBE\x17\xA5\x96\xB3\xCF\x82`@Qa1H\x91\x90`\x01`\x01``\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PPV[\x805`\xFF\x81\x16\x81\x14a1eW`\0\x80\xFD[\x91\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a1eW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a1\x93W`\0\x80\xFD[\x835\x92Pa1\xA3` \x85\x01a1TV[\x91Pa1\xB1`@\x85\x01a1jV[\x90P\x92P\x92P\x92V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x14hW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a1\xE2W`\0\x80\xFD[a1\xEB\x83a1TV[\x91P` \x83\x015a1\xFB\x81a1\xBAV[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a2\x1BW`\0\x80\xFD[a2$\x84a1TV[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[\x81Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x80\x84\x01Q\x90\x91\x16\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01``\x1B\x03\x16\x90\x82\x01R``\x81\x01a\x07\xBCV[`\0\x80\x83`\x1F\x84\x01\x12a2\x80W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a2\x97W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a2\xAFW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a2\xCCW`\0\x80\xFD[\x845a2\xD7\x81a1\xBAV[\x93P` \x85\x015\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a2\xF9W`\0\x80\xFD[a3\x05\x87\x82\x88\x01a2nV[\x95\x98\x94\x97P\x95PPPPV[`\0\x80\x83`\x1F\x84\x01\x12a3#W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a3:W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a2\xAFW`\0\x80\xFD[`\0\x80`\0`@\x84\x86\x03\x12\x15a3jW`\0\x80\xFD[a3s\x84a1TV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a3\x8EW`\0\x80\xFD[a3\x9A\x86\x82\x87\x01a3\x11V[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a3\xBFW`\0\x80\xFD[a3\xC8\x86a1TV[\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a3\xE4W`\0\x80\xFD[a3\xF0\x89\x83\x8A\x01a3\x11V[\x90\x96P\x94P`@\x88\x015\x91P\x80\x82\x11\x15a4\tW`\0\x80\xFD[Pa4\x16\x88\x82\x89\x01a3\x11V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[\x805`\x01`\x01``\x1B\x03\x81\x16\x81\x14a1eW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a4SW`\0\x80\xFD[a4\\\x84a1TV[\x92P` \x84\x015a4l\x81a1\xBAV[\x91Pa1\xB1`@\x85\x01a4'V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a4\xB2Wa4\xB2a4zV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a4\xE0Wa4\xE0a4zV[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a5\x01Wa5\x01a4zV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a5\x1CW`\0\x80\xFD[\x815` a51a5,\x83a4\xE8V[a4\xB8V[\x82\x81R`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a5PW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a5\x9FW`@\x81\x89\x03\x12\x15a5mW`\0\x80\x81\xFD[a5ua4\x90V[\x815a5\x80\x81a1\xBAV[\x81Ra5\x8D\x82\x86\x01a4'V[\x81\x86\x01R\x83R\x91\x83\x01\x91`@\x01a5TV[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a5\xBBW`\0\x80\xFD[\x815` a5\xCBa5,\x83a4\xE8V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a5\xEAW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a5\x9FW\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a6\rW`\0\x80\x81\xFD[a6\x1B\x89\x86\x83\x8B\x01\x01a5\x0BV[\x84RP\x91\x83\x01\x91\x83\x01a5\xEEV[`\0\x80`@\x83\x85\x03\x12\x15a6<W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a6SW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a6gW`\0\x80\xFD[\x815` a6wa5,\x83a4\xE8V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a6\x96W`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a6\xBBWa6\xAC\x86a4'V[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a6\x9BV[\x96PP\x86\x015\x92PP\x80\x82\x11\x15a6\xD1W`\0\x80\xFD[Pa6\xDE\x85\x82\x86\x01a5\xAAV[\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a6\xFBW`\0\x80\xFD[\x825\x91Pa7\x0B` \x84\x01a1TV[\x90P\x92P\x92\x90PV[`\0``\x82\x84\x03\x12\x15a7&W`\0\x80\xFD[`@Q``\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a7HWa7Ha4zV[`@R\x90P\x80a7W\x83a1jV[\x81Ra7e` \x84\x01a1jV[` \x82\x01Ra7v`@\x84\x01a4'V[`@\x82\x01RP\x92\x91PPV[`\0\x80`\0`\xA0\x84\x86\x03\x12\x15a7\x97W`\0\x80\xFD[\x835\x92Pa7\xA7` \x85\x01a1TV[\x91Pa1\xB1\x85`@\x86\x01a7\x14V[`\0\x80`@\x83\x85\x03\x12\x15a7\xC9W`\0\x80\xFD[a7\xD2\x83a1TV[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a7\xF2W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\x80\x83\x85\x03\x12\x15a8\x0CW`\0\x80\xFD[a8\x15\x83a1TV[\x91Pa7\x0B\x84` \x85\x01a7\x14V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a8\x90Wa8}\x83\x85Qc\xFF\xFF\xFF\xFF\x80\x82Q\x16\x83R\x80` \x83\x01Q\x16` \x84\x01RP`\x01`\x01``\x1B\x03`@\x82\x01Q\x16`@\x83\x01RPPV[\x92\x84\x01\x92``\x92\x90\x92\x01\x91`\x01\x01a8@V[P\x90\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a8\xAEW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a8\xC4W`\0\x80\xFD[a\x11\x0F\x84\x82\x85\x01a5\x0BV[`\0` \x82\x84\x03\x12\x15a8\xE2W`\0\x80\xFD[a\x07\x83\x82a1TV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a9\x01W`\0\x80\xFD[a9\n\x85a1TV[\x93Pa9\x18` \x86\x01a1jV[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15a9BW`\0\x80\xFD[\x835a9M\x81a1\xBAV[\x92P` \x84\x015\x91Pa1\xB1`@\x85\x01a1TV[`\0\x80`@\x83\x85\x03\x12\x15a9uW`\0\x80\xFD[a9~\x83a1TV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a9\x99W`\0\x80\xFD[a6\xDE\x85\x82\x86\x01a5\x0BV[`\0\x80`@\x83\x85\x03\x12\x15a9\xB8W`\0\x80\xFD[a9\xC1\x83a1TV[\x91Pa7\x0B` \x84\x01a4'V[`\0\x80`\0`@\x84\x86\x03\x12\x15a9\xE4W`\0\x80\xFD[\x835\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a:\x01W`\0\x80\xFD[a3\x9A\x86\x82\x87\x01a2nV[`\0\x80`\0``\x84\x86\x03\x12\x15a:\"W`\0\x80\xFD[a:+\x84a1TV[\x92Pa:9` \x85\x01a1jV[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0\x80` \x83\x85\x03\x12\x15a:\\W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a:rW`\0\x80\xFD[a:~\x85\x82\x86\x01a3\x11V[\x90\x96\x90\x95P\x93PPPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a:\x9FW`\0\x80\xFD[a:\xA8\x84a1jV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a:\x01W`\0\x80\xFD[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a8\x90W\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a:\xDFV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R`\x01`\x01``\x1B\x03\x16` \x82\x01R`@\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[` \x80\x82R`L\x90\x82\x01R\x7FStakeRegistry.onlyRegistryCoordi`@\x82\x01R\x7Fnator: caller is not the Registr``\x82\x01Rk<\xA1\xB7\xB7\xB924\xB70\xBA7\xB9`\xA1\x1B`\x80\x82\x01R`\xA0\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a;\xD3Wa;\xD3a;\xABV[P\x03\x90V[` \x80\x82R`T\x90\x82\x01R\x7FStakeRegistry._registerOperator:`@\x82\x01R\x7F greatest quorumNumber must be l``\x82\x01Rs\x19\\\xDC\xC8\x1D\x1A\x18[\x88\x1C][\xDC\x9D[P\xDB\xDD[\x9D`b\x1B`\x80\x82\x01R`\xA0\x01\x90V[` \x80\x82R`\\\x90\x82\x01R\x7FStakeRegistry._registerOperator:`@\x82\x01R\x7F Operator does not meet minimum ``\x82\x01R\x7Fstake requirement for quorum\0\0\0\0`\x80\x82\x01R`\xA0\x01\x90V[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a<\xF7Wa<\xF7a;\xABV[\x01\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a=\x12W`\0\x80\xFD[\x81Qa\x07\x83\x81a1\xBAV[` \x80\x82R`V\x90\x82\x01R\x7FVoteWeigherBase.onlyServiceManag`@\x82\x01R\x7FerOwner: caller is not the owner``\x82\x01Ru\x107\xB3\x10:42\x909\xB2\xB9;4\xB1\xB2\xA6\xB0\xB70\xB3\xB2\xB9`Q\x1B`\x80\x82\x01R`\xA0\x01\x90V[` \x80\x82R`<\x90\x82\x01R\x7FVoteWeigherBase.validQuorumNumbe`@\x82\x01R\x7Fr: quorumNumber is not valid\0\0\0\0``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a>\x1EW`\0\x80\xFD[a\x07\x83\x82a4'V[`\0`\x01`\x01``\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a>GWa>Ga;\xABV[\x03\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a>aW`\0\x80\xFD[\x815a\x07\x83\x81a1\xBAV[`\0` \x82\x84\x03\x12\x15a>~W`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a>\x97W`\0\x80\xFD[\x81Q`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x14a\x07\x83W`\0\x80\xFD[`\0c\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a>GWa>Ga;\xABV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x14\x15a>\xE5Wa>\xE5a;\xABV[`\x01\x01\x93\x92PPPV[`\0`\0\x19\x82\x14\x15a?\x03Wa?\x03a;\xABV[P`\x01\x01\x90V[`\0a\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a<\xF7Wa<\xF7a;\xABV[`\0\x82\x19\x82\x11\x15a?:Wa?:a;\xABV[P\x01\x90V\xFEVoteWeigherBase._addStrategiesCo\xA2dipfsX\"\x12 oY@\xCF@\x84x\xDE\xA4S\x079Y`\xB7\xB6\xC4\x8F\x83\xABRr\xD2\x10\xDB*\x92\x0F]\xD1\xE1\x16dsolcC\0\x08\x0C\x003";
    /// The bytecode of the contract.
    pub static STAKEREGISTRYHARNESS_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02SW`\x005`\xE0\x1C\x80c\x94Dr\xA9\x11a\x01FW\x80c\xBD)\xB8\xCD\x11a\0\xC3W\x80c\xDF\\\xF7#\x11a\0\x87W\x80c\xDF\\\xF7#\x14a\x06\x85W\x80c\xE1\x92\xE9\xAD\x14a\x06\xACW\x80c\xE2T'\xDD\x14a\x06\xCCW\x80c\xE8\x9C\n\0\x14a\x06\xDFW\x80c\xEB\x92\x19\x9C\x14a\x06\xF2W\x80c\xF5\x04X\x94\x14a\x07\x13W`\0\x80\xFD[\x80c\xBD)\xB8\xCD\x14a\x06&W\x80c\xC8)LV\x14a\x069W\x80c\xC8\xF79\xD0\x14a\x06LW\x80c\xCD\x05\r\x9C\x14a\x06_W\x80c\xCE\x97~\xC3\x14a\x06rW`\0\x80\xFD[\x80c\xA6sFg\x11a\x01\nW\x80c\xA6sFg\x14a\x05\x9EW\x80c\xB14Bq\x14a\x05\xA6W\x80c\xB37\x1B\xAA\x14a\x05\xCDW\x80c\xBB\xA5I\xFA\x14a\x06\0W\x80c\xBC\x9A@\xC3\x14a\x06\x13W`\0\x80\xFD[\x80c\x94Dr\xA9\x14a\x05\x0EW\x80c\x99\xEE\xD4\xEE\x14a\x05.W\x80c\x9A\xA1e=\x14a\x05AW\x80c\x9E\x8C\xA6 \x14a\x05hW\x80c\xA4<\xDE\x89\x14a\x05\x8BW`\0\x80\xFD[\x80c^Zgu\x11a\x01\xD4W\x80c|\x17#G\x11a\x01\x98W\x80c|\x17#G\x14a\x04{W\x80c|\xA9\xFF\0\x14a\x04\x95W\x80c|\xC0\xD7_\x14a\x04\xA8W\x80c~\xD9C\x0F\x14a\x04\xE8W\x80c\x90)\x04}\x14a\x04\xFBW`\0\x80\xFD[\x80c^Zgu\x14a\x03\xE0W\x80c_)H\xEC\x14a\x03\xFDW\x80cj\xB58\xD4\x14a\x04\x10W\x80cm\x14\xA9\x87\x14a\x04#W\x80cn\x8F\x03\xCA\x14a\x04JW`\0\x80\xFD[\x80c,*]+\x11a\x02\x1BW\x80c,*]+\x14a\x02\xE3W\x80c9\x98\xFD\xD3\x14a\x02\xF6W\x80c9\xB7\x0E8\x14a\x035W\x80cH\x08Xf\x14a\x03\\W\x80cL\xAFH\xC2\x14a\x03\x84W`\0\x80\xFD[\x80c\x1B2r%\x14a\x02XW\x80c\x1F\x9Bt\xE0\x14a\x02\x88W\x80c$\x8Des\x14a\x02\x9BW\x80c%PGw\x14a\x02\xBBW\x80c+=\x88\x16\x14a\x02\xD0W[`\0\x80\xFD[a\x02ka\x02f6`\x04a1~V[a\x07&V[`@Q`\x01`\x01``\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02ka\x02\x966`\x04a1\xCFV[a\x07\x8AV[a\x02\xAEa\x02\xA96`\x04a2\x06V[a\x07\xC2V[`@Qa\x02\x7F\x91\x90a29V[a\x02\xCEa\x02\xC96`\x04a2\xB6V[a\x08[V[\0[a\x02\xCEa\x02\xDE6`\x04a3UV[a\n:V[a\x02\xCEa\x02\xF16`\x04a3\xA7V[a\r\xCCV[a\x03\x1D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x7FV[a\x03\x1D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03oa\x03j6`\x04a1~V[a\x11\x02V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\x7FV[a\x02\xCEa\x03\x926`\x04a4>V[`\xFF\x90\x92\x16`\0\x90\x81Ra\x01\xF5` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x83R\x92\x90R \x80T`\x01`\x01``\x1B\x03\x90\x92\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x03\xEFg\r\xE0\xB6\xB3\xA7d\0\0\x81V[`@Q\x90\x81R` \x01a\x02\x7FV[a\x02\xCEa\x04\x0B6`\x04a6)V[a\x11\x17V[a\x02ka\x04\x1E6`\x04a6\xE8V[a\x12,V[a\x03\x1D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\xEFa\x04X6`\x04a6\xE8V[`\0\x91\x82Ra\x01\xB3` \x90\x81R`@\x80\x84 `\xFF\x93\x90\x93\x16\x84R\x91\x90R\x90 T\x90V[a\x04\x83` \x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02\x7FV[a\x02ka\x04\xA36`\x04a7\x82V[a\x12EV[a\x04\xBBa\x04\xB66`\x04a7\xB6V[a\x12RV[`@\x80Q\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x92\x83\x01Q`\x01`\x01``\x1B\x03\x16\x92\x81\x01\x92\x90\x92R\x01a\x02\x7FV[a\x02ka\x04\xF66`\x04a7\xE0V[a\x12\xCCV[a\x02\xCEa\x05\t6`\x04a7\xF9V[a\x13\0V[a\x05!a\x05\x1C6`\x04a6\xE8V[a\x13\x0EV[`@Qa\x02\x7F\x91\x90a8$V[a\x02\xCEa\x05<6`\x04a8\x9CV[a\x13\xADV[`\0Ta\x05U\x90b\x01\0\0\x90\x04a\xFF\xFF\x16\x81V[`@Qa\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\x7FV[a\x03\xEFa\x05v6`\x04a8\xD0V[`\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[a\x02ka\x05\x996`\x04a8\xEBV[a\x14kV[a\x04\x83`\xC0\x81V[a\x03\x1D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x05\xE0a\x05\xDB6`\x04a9-V[a\x15\x01V[`@\x80Q`\x01`\x01``\x1B\x03\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01a\x02\x7FV[a\x02\xCEa\x06\x0E6`\x04a9bV[a\x15\x1BV[a\x02\xCEa\x06!6`\x04a9\xA5V[a\x16\x07V[a\x02\xCEa\x0646`\x04a9\xCFV[a\x16\xC3V[a\x02ka\x06G6`\x04a:\rV[a\x18ZV[a\x02ka\x06Z6`\x04a8\xD0V[a\x18\xDDV[a\x02\xAEa\x06m6`\x04a6\xE8V[a\x19NV[a\x02\xCEa\x06\x806`\x04a:IV[a\x1A5V[a\x03\x1D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x06\xBFa\x06\xBA6`\x04a:\x8AV[a\x1D\x15V[`@Qa\x02\x7F\x91\x90a:\xC3V[a\x02\xAEa\x06\xDA6`\x04a7\xB6V[a\x1F\x82V[a\x03\xEFa\x06\xED6`\x04a8\xD0V[a \x1AV[a\x07\x05a\x07\x006`\x04a7\xB6V[a ;V[`@Qa\x02\x7F\x92\x91\x90a;\x01V[a\x02\xCEa\x07!6`\x04a2\xB6V[a \x84V[`\0\x83\x81Ra\x01\xB3` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 a\x07N\x85\x85\x85a\"\nV[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x07dWa\x07da;#V[`\0\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90P[\x93\x92PPPV[`\xFF\x82\x16`\0\x90\x81Ra\x01\xF5` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 T`\x01`\x01``\x1B\x03\x16[\x92\x91PPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R\x85\x82Ra\x01\xB3\x81R\x83\x82 `\xFF\x88\x16\x83R\x90R\x91\x90\x91 \x80T\x83\x90\x81\x10a\x08\x08Wa\x08\x08a;#V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x93\x83\x01\x93\x90\x93R`\x01`@\x1B\x90\x92\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x94\x93PPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x08\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a;9V[`@Q\x80\x91\x03\x90\xFD[`\0Tb\x01\0\0\x90\x04a\xFF\xFF\x16\x82\x82a\x08\xC6`\x01\x82a;\xC1V[\x81\x81\x10a\x08\xD5Wa\x08\xD5a;#V[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C`\xFF\x16\x10a\t\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a;\xD8V[`@\x80Q``\x81\x01\x82R`\0` \x82\x01\x81\x90R\x91\x81\x01\x82\x90RCc\xFF\xFF\xFF\xFF\x16\x81R\x90[`\xFF\x81\x16\x83\x11\x15a\n2W`\0\x84\x84\x83`\xFF\x16\x81\x81\x10a\tHWa\tHa;#V[\x91\x90\x91\x015`\xF8\x1C\x91P`\0\x90Pa\ta\x88\x88\x84a%1V[\x91PP`\x01`\x01``\x1B\x03\x81\x16a\t\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a<RV[`\0`\xB3\x83`\xFF\x16a\x01\0\x81\x10a\t\xA3Wa\t\xA3a;#V[\x01T\x90P\x81\x81\x15a\n\nW`\xB3\x84`\xFF\x16a\x01\0\x81\x10a\t\xC5Wa\t\xC5a;#V[\x01a\t\xD1`\x01\x84a;\xC1V[\x81T\x81\x10a\t\xE1Wa\t\xE1a;#V[`\0\x91\x82R` \x90\x91 \x01Ta\n\x07\x90`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x82a<\xD5V[\x90P[`\x01`\x01``\x1B\x03\x81\x16`@\x87\x01Ra\n#\x84\x87a&6V[\x84`\x01\x01\x94PPPPPa\t&V[PPPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\x98W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xBC\x91\x90a=\0V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\n\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a=\x1DV[`\0T\x83\x90b\x01\0\0\x90\x04a\xFF\xFF\x16`\xFF\x82\x16\x10a\x0B\x1CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a=\x99V[\x81\x80a\x0B\xB6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`W`$\x82\x01R\x7FVoteWeigherBase.removeStrategies`D\x82\x01R\x7FConsideredAndMultipliers: no ind`d\x82\x01R\x7Fices to remove provided\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x08\xA3V[`\0[\x81\x81\x10\x15a\n2W`\xFF\x86\x16`\0\x81\x81R`\x01` R`@\x90 \x7F1\xFA.,\xD2\x80\xC97^\x13\xFF\xCF=\x81\xE27\x81\0\x18n@X\xF8\xD3\xDD\xB6\x90\xB8-\xCD1\xF7\x90\x87\x87\x85\x81\x81\x10a\x0C\x07Wa\x0C\x07a;#V[\x90P` \x02\x015\x81T\x81\x10a\x0C\x1EWa\x0C\x1Ea;#V[`\0\x91\x82R` \x91\x82\x90 \x01T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x01`@Q\x80\x91\x03\x90\xA2`\xFF\x86\x16`\0\x81\x81R`\x01` R`@\x90 \x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x90\x87\x87\x85\x81\x81\x10a\x0C\x8AWa\x0C\x8Aa;#V[\x90P` \x02\x015\x81T\x81\x10a\x0C\xA1Wa\x0C\xA1a;#V[`\0\x91\x82R` \x80\x83 \x91\x90\x91\x01T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82R\x91\x81\x01\x92\x90\x92R\x01`@Q\x80\x91\x03\x90\xA2`\xFF\x86\x16`\0\x90\x81R`\x01` \x81\x90R`@\x90\x91 \x80T\x90\x91a\x0C\xF3\x91a;\xC1V[\x81T\x81\x10a\r\x03Wa\r\x03a;#V[`\0\x91\x82R` \x80\x83 `\xFF\x8A\x16\x84R`\x01\x90\x91R`@\x90\x92 \x91\x01\x90\x86\x86\x84\x81\x81\x10a\r2Wa\r2a;#V[\x90P` \x02\x015\x81T\x81\x10a\rIWa\rIa;#V[`\0\x91\x82R` \x80\x83 \x84T\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92\x83\x17\x81U\x93T`\x01`\x01``\x1B\x03`\x01`\xA0\x1B\x91\x82\x90\x04\x16\x02\x90\x91\x17\x90\x92U`\xFF\x88\x16\x81R`\x01\x90\x91R`@\x90 \x80T\x80a\r\xACWa\r\xACa=\xF6V[`\0\x82\x81R` \x81 \x82\x01`\0\x19\x90\x81\x01\x91\x90\x91U\x01\x90U`\x01\x01a\x0B\xB9V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E*W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0EN\x91\x90a=\0V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0E~W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a=\x1DV[`\0T\x85\x90b\x01\0\0\x90\x04a\xFF\xFF\x16`\xFF\x82\x16\x10a\x0E\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a=\x99V[\x83\x80a\x0F.W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FVoteWeigherBase.modifyStrategyWe`D\x82\x01R\x7Fights: no strategy indices provi`d\x82\x01Rb\x19\x19Y`\xEA\x1B`\x84\x82\x01R`\xA4\x01a\x08\xA3V[\x82\x81\x14a\x0F\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R\x7FVoteWeigherBase.modifyStrategyWe`D\x82\x01R\x7Fights: input length mismatch\0\0\0\0`d\x82\x01R`\x84\x01a\x08\xA3V[`\0[\x81\x81\x10\x15a\x10\xF8W\x84\x84\x82\x81\x81\x10a\x0F\xC0Wa\x0F\xC0a;#V[\x90P` \x02\x01` \x81\x01\x90a\x0F\xD5\x91\x90a>\x0CV[`\xFF\x89\x16`\0\x90\x81R`\x01` R`@\x90 \x88\x88\x84\x81\x81\x10a\x0F\xF9Wa\x0F\xF9a;#V[\x90P` \x02\x015\x81T\x81\x10a\x10\x10Wa\x10\x10a;#V[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x80T`\x01`\x01``\x1B\x03\x94\x90\x94\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x93\x90\x93\x17\x90\x92U`\xFF\x8A\x16\x80\x82R`\x01\x90\x92R`@\x90 \x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x90\x89\x89\x85\x81\x81\x10a\x10\x87Wa\x10\x87a;#V[\x90P` \x02\x015\x81T\x81\x10a\x10\x9EWa\x10\x9Ea;#V[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x87\x87\x85\x81\x81\x10a\x10\xC5Wa\x10\xC5a;#V[\x90P` \x02\x01` \x81\x01\x90a\x10\xDA\x91\x90a>\x0CV[`@Qa\x10\xE8\x92\x91\x90a;\x01V[`@Q\x80\x91\x03\x90\xA2`\x01\x01a\x0F\xA6V[PPPPPPPPV[`\0a\x11\x0F\x84\x84\x84a\"\nV[\x94\x93PPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x117WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x11QWP0;\x15\x80\x15a\x11QWP`\0T`\xFF\x16`\x01\x14[a\x11\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x08\xA3V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x11\xD7W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x11\xE1\x83\x83a'PV[\x80\x15a\x12'W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPV[`\0\x80a\x129\x84\x84a\x19NV[`@\x01Q\x94\x93PPPPV[`\0a\x11\x0F\x84\x84\x84a(\x94V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\xFF\x83\x16`\0\x90\x81R`\x01` R`@\x90 \x80T\x83\x90\x81\x10a\x12\x8AWa\x12\x8Aa;#V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[`3\x81a\x01\0\x81\x10a\x12\xDDW`\0\x80\xFD[`\x02\x91\x82\x82\x04\x01\x91\x90\x06`\x0C\x02\x91PT\x90a\x01\0\n\x90\x04`\x01`\x01``\x1B\x03\x16\x81V[a\x13\n\x82\x82a&6V[PPV[`\0\x82\x81Ra\x01\xB3` \x90\x81R`@\x80\x83 `\xFF\x85\x16\x84R\x82R\x80\x83 \x80T\x82Q\x81\x85\x02\x81\x01\x85\x01\x90\x93R\x80\x83R``\x94\x92\x93\x91\x92\x90\x91\x84\x01[\x82\x82\x10\x15a\x13\xA1W`\0\x84\x81R` \x90\x81\x90 `@\x80Q``\x81\x01\x82R\x91\x85\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x83\x85\x01R`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90\x82\x01R\x82R`\x01\x90\x92\x01\x91\x01a\x13HV[PPPP\x90P\x92\x91PPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\x0BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14/\x91\x90a=\0V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x14_W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a=\x1DV[a\x14h\x81a*\x0CV[PV[`\0\x82\x81Ra\x01\xB3` \x90\x81R`@\x80\x83 `\xFF\x88\x16\x84R\x90\x91R\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x14\x9DWa\x14\x9Da;#V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90Pa\x14\xF4\x81\x86a+\x0CV[`@\x01Q\x95\x94PPPPPV[`\0\x80a\x15\x0F\x85\x85\x85a%1V[\x91P\x91P\x93P\x93\x91PPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15yW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x9D\x91\x90a=\0V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x15\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a=\x1DV[`\0T\x82\x90b\x01\0\0\x90\x04a\xFF\xFF\x16`\xFF\x82\x16\x10a\x15\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a=\x99V[a\x12'\x83\x83a,\x8DV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16eW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\x89\x91\x90a=\0V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x16\xB9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a=\x1DV[a\x13\n\x82\x82a0\xBFV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x17\x0BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a;9V[`@\x80Q``\x80\x82\x01\x83R`\0` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90Rc\xFF\xFF\xFF\xFFC\x16\x80\x85R\x85Q\x93\x84\x01\x86R\x90\x83\x01\x82\x90R\x93\x82\x01\x81\x90R\x92\x81R\x90\x91[`\xFF\x81\x16\x84\x11\x15a\n2W`\0\x85\x85\x83`\xFF\x16\x81\x81\x10a\x17kWa\x17ka;#V[\x91\x90\x91\x015`\xF8\x1C\x91P`\0\x90Pa\x17\x84\x88\x83\x87a(\x94V[\x90P\x80`\xB3\x83`\xFF\x16a\x01\0\x81\x10a\x17\x9EWa\x17\x9Ea;#V[\x01`\x01`\xB3\x85`\xFF\x16a\x01\0\x81\x10a\x17\xB8Wa\x17\xB8a;#V[\x01Ta\x17\xC4\x91\x90a;\xC1V[\x81T\x81\x10a\x17\xD4Wa\x17\xD4a;#V[`\0\x91\x82R` \x90\x91 \x01Ta\x17\xFA\x91\x90`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16a>'V[`\x01`\x01``\x1B\x03\x16`@\x85\x01Ra\x18\x12\x82\x85a&6V[`@\x80Q`\xFF\x84\x16\x81R`\0` \x82\x01R\x89\x91\x7F\xE7\xC6\x0CRi/\x0E\x0F\xF8\xD1\xC2\x89\xFF\xE6<\xB0.y7?5\xE3\0\x0C\xAF\xE7\xCC\xA8\x98\x88Y4\x91\x01`@Q\x80\x91\x03\x90\xA2PP`\x01\x01a\x17IV[`\0\x80`\xB3\x85`\xFF\x16a\x01\0\x81\x10a\x18tWa\x18ta;#V[\x01\x83\x81T\x81\x10a\x18\x86Wa\x18\x86a;#V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90Pa\x129\x81\x85a+\x0CV[`\0`\xB3\x82`\xFF\x16a\x01\0\x81\x10a\x18\xF6Wa\x18\xF6a;#V[\x01`\x01`\xB3\x84`\xFF\x16a\x01\0\x81\x10a\x19\x10Wa\x19\x10a;#V[\x01Ta\x19\x1C\x91\x90a;\xC1V[\x81T\x81\x10a\x19,Wa\x19,a;#V[`\0\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x92\x91PPV[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R\x86\x82Ra\x01\xB3\x81R\x84\x82 `\xFF\x87\x16\x83R\x81R\x84\x82 T\x85Q\x93\x84\x01\x86R\x82\x84R\x90\x83\x01\x82\x90R\x93\x82\x01R\x90\x91\x90\x81a\x19\xA8W\x91Pa\x07\xBC\x90PV[`\0\x85\x81Ra\x01\xB3` \x90\x81R`@\x80\x83 `\xFF\x88\x16\x84R\x90\x91R\x90 a\x19\xD0`\x01\x84a;\xC1V[\x81T\x81\x10a\x19\xE0Wa\x19\xE0a;#V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x92Pa\x07\xBC\x91PPV[`\0[`\0Tb\x01\0\0\x90\x04a\xFF\xFF\x16`\xFF\x82\x16\x10\x15a\x12'W`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R\x90[\x83\x81\x10\x15a\x1C\xF4W`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x13T*N\x87\x87\x85\x81\x81\x10a\x1A\xB8Wa\x1A\xB8a;#V[\x90P` \x02\x01` \x81\x01\x90a\x1A\xCD\x91\x90a>OV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\x11W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B5\x91\x90a>lV[`@Qc41\xAF%`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c41\xAF%\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xA0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xC4\x91\x90a>\x85V[\x90P`\x01`\x01`\x01`\xC0\x1B\x03\x82\x16`\xFF\x87\x16\x1C\x81\x16\x14\x15a\x1C\xEAW\x83Qc\xFF\xFF\xFF\xFF\x16a\x1C\x86W`\xB3\x85`\xFF\x16a\x01\0\x81\x10a\x1C\x02Wa\x1C\x02a;#V[\x01`\x01`\xB3\x87`\xFF\x16a\x01\0\x81\x10a\x1C\x1CWa\x1C\x1Ca;#V[\x01Ta\x1C(\x91\x90a;\xC1V[\x81T\x81\x10a\x1C8Wa\x1C8a;#V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x93P[`\0\x80a\x1C\xBA\x89\x89\x87\x81\x81\x10a\x1C\x9EWa\x1C\x9Ea;#V[\x90P` \x02\x01` \x81\x01\x90a\x1C\xB3\x91\x90a>OV[\x85\x89a%1V[\x91P\x91P\x80\x82\x87`@\x01Qa\x1C\xCF\x91\x90a>'V[a\x1C\xD9\x91\x90a<\xD5V[`\x01`\x01``\x1B\x03\x16`@\x87\x01RPP[PP`\x01\x01a\x1AmV[P\x80Qc\xFF\xFF\xFF\xFF\x16\x15a\x1D\x0CWa\x1D\x0C\x82\x82a&6V[P`\x01\x01a\x1A8V[```\0\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D1Wa\x1D1a4zV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1DZW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83\x81\x10\x15a\x1FyW`\0\x85\x85\x83\x81\x81\x10a\x1D|Wa\x1D|a;#V[\x91\x90\x91\x015`\xF8\x1C\x91PPc\xFF\xFF\xFF\xFF\x87\x16`\xB3\x82a\x01\0\x81\x10a\x1D\xA2Wa\x1D\xA2a;#V[\x01`\0\x81T\x81\x10a\x1D\xB5Wa\x1D\xB5a;#V[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a\x1EtW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`j`$\x82\x01R\x7FStakeRegistry.getTotalStakeIndic`D\x82\x01R\x7FesByQuorumNumbersAtBlockNumber: `d\x82\x01R\x7Fquorum has no stake history at b`\x84\x82\x01Ri67\xB1\xB5\xA7:\xB6\xB12\xB9`\xB1\x1B`\xA4\x82\x01R`\xC4\x01a\x08\xA3V[`\0`\xB3\x82`\xFF\x16a\x01\0\x81\x10a\x1E\x8DWa\x1E\x8Da;#V[\x01T\x90P`\0[\x81c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x1FcW\x88c\xFF\xFF\xFF\xFF\x16`\xB3\x84`\xFF\x16a\x01\0\x81\x10a\x1E\xC6Wa\x1E\xC6a;#V[\x01`\x01a\x1E\xD3\x84\x86a>\xAEV[a\x1E\xDD\x91\x90a>\xAEV[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x1E\xF3Wa\x1E\xF3a;#V[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a\x1FQW`\x01a\x1F\x16\x82\x84a>\xAEV[a\x1F \x91\x90a>\xAEV[\x85\x85\x81Q\x81\x10a\x1F2Wa\x1F2a;#V[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPPa\x1FcV[\x80a\x1F[\x81a>\xCBV[\x91PPa\x1E\x94V[PPP\x80\x80a\x1Fq\x90a>\xEFV[\x91PPa\x1D`V[P\x94\x93PPPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R`\xB3\x83`\xFF\x16a\x01\0\x81\x10a\x1F\xB6Wa\x1F\xB6a;#V[\x01\x82\x81T\x81\x10a\x1F\xC8Wa\x1F\xC8a;#V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x93\x83\x01\x93\x90\x93R`\x01`@\x1B\x90\x92\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[`\0`\xB3\x82`\xFF\x16a\x01\0\x81\x10a 3Wa 3a;#V[\x01T\x92\x91PPV[`\x01` R\x81`\0R`@`\0 \x81\x81T\x81\x10a WW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x92P`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90P\x82V[`\0Tb\x01\0\0\x90\x04a\xFF\xFF\x16\x82\x82a \x9E`\x01\x82a;\xC1V[\x81\x81\x10a \xADWa \xADa;#V[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C`\xFF\x16\x10a \xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a;\xD8V[`@\x80Q``\x81\x01\x82R`\0` \x82\x01\x81\x90R\x91\x81\x01\x82\x90RCc\xFF\xFF\xFF\xFF\x16\x81R\x90[`\xFF\x81\x16\x83\x11\x15a\n2W`\0\x84\x84\x83`\xFF\x16\x81\x81\x10a! Wa! a;#V[\x91\x90\x91\x015`\xF8\x1C\x91P`\0\x90Pa!9\x88\x88\x84a%1V[\x91PP`\x01`\x01``\x1B\x03\x81\x16a!bW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xA3\x90a<RV[`\0`\xB3\x83`\xFF\x16a\x01\0\x81\x10a!{Wa!{a;#V[\x01T\x90P\x81\x81\x15a!\xE2W`\xB3\x84`\xFF\x16a\x01\0\x81\x10a!\x9DWa!\x9Da;#V[\x01a!\xA9`\x01\x84a;\xC1V[\x81T\x81\x10a!\xB9Wa!\xB9a;#V[`\0\x91\x82R` \x90\x91 \x01Ta!\xDF\x90`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x82a<\xD5V[\x90P[`\x01`\x01``\x1B\x03\x81\x16`@\x87\x01Ra!\xFB\x84\x87a&6V[\x84`\x01\x01\x94PPPPPa \xFEV[`\0\x83\x81Ra\x01\xB3` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 T\x81[\x81c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a$aW`\0\x86\x81Ra\x01\xB3` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x90\x91R\x90 c\xFF\xFF\xFF\xFF\x85\x16\x90`\x01a\"o\x84\x86a>\xAEV[a\"y\x91\x90a>\xAEV[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\"\x8FWa\"\x8Fa;#V[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a$OW`\0\x86\x81Ra\x01\xB3` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x90\x91R\x90 `\x01a\"\xCF\x83\x85a>\xAEV[a\"\xD9\x91\x90a>\xAEV[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\"\xEFWa\"\xEFa;#V[`\0\x91\x82R` \x90\x91 \x01T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15\x80a#{WP`\0\x86\x81Ra\x01\xB3` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x90\x91R\x90 c\xFF\xFF\xFF\xFF\x85\x16\x90`\x01a#@\x84\x86a>\xAEV[a#J\x91\x90a>\xAEV[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a#`Wa#`a;#V[`\0\x91\x82R` \x90\x91 \x01T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x11[a$0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`t`$\x82\x01R\x7FStakeRegistry._getStakeUpdateInd`D\x82\x01R\x7FexForOperatorIdForQuorumAtBlockN`d\x82\x01R\x7Fumber: operatorId has no stake u`\x84\x82\x01Rs820\xBA2\x900\xBA\x10167\xB1\xB5\xA7:\xB6\xB12\xB9`a\x1B`\xA4\x82\x01R`\xC4\x01a\x08\xA3V[`\x01a$<\x82\x84a>\xAEV[a$F\x91\x90a>\xAEV[\x92PPPa\x07\x83V[\x80a$Y\x81a>\xCBV[\x91PPa\"*V[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x8C`$\x82\x01R\x7FStakeRegistry._getStakeUpdateInd`D\x82\x01R\x7FexForOperatorIdForQuorumAtBlockN`d\x82\x01R\x7Fumber: no stake update found for`\x84\x82\x01R\x7F operatorId and quorumNumber at `\xA4\x82\x01Rk167\xB1\xB5\x907:\xB6\xB12\xB9`\xA1\x1B`\xC4\x82\x01R`\xE4\x01a\x08\xA3V[`@\x80Q``\x81\x01\x82R`\0` \x82\x01\x81\x90R\x91\x81\x01\x82\x90RCc\xFF\xFF\xFF\xFF\x16\x81R\x81\x90a%_\x84\x87a\x07\x8AV[`\x01`\x01``\x1B\x03\x16`@\x82\x01R`3`\xFF\x85\x16a\x01\0\x81\x10a%\x84Wa%\x84a;#V[`\x02\x91\x82\x82\x04\x01\x91\x90\x06`\x0C\x02\x90T\x90a\x01\0\n\x90\x04`\x01`\x01``\x1B\x03\x16`\x01`\x01``\x1B\x03\x16\x81`@\x01Q`\x01`\x01``\x1B\x03\x16\x10\x15a%\xC8W`\0`@\x82\x01R[`\0a%\xD5\x86\x86\x84a(\x94V[`@\x80\x84\x01Q\x81Q`\xFF\x89\x16\x81R`\x01`\x01``\x1B\x03\x90\x91\x16` \x82\x01R\x91\x92P\x87\x91\x7F\xE7\xC6\x0CRi/\x0E\x0F\xF8\xD1\xC2\x89\xFF\xE6<\xB0.y7?5\xE3\0\x0C\xAF\xE7\xCC\xA8\x98\x88Y4\x91\x01`@Q\x80\x91\x03\x90\xA2`@\x90\x91\x01Q\x90\x92P\x90P\x93P\x93\x91PPV[`\0`\xB3\x83`\xFF\x16a\x01\0\x81\x10a&OWa&Oa;#V[\x01T\x90P\x80\x15a&\xB9WC`\xB3\x84`\xFF\x16a\x01\0\x81\x10a&qWa&qa;#V[\x01a&}`\x01\x84a;\xC1V[\x81T\x81\x10a&\x8DWa&\x8Da;#V[\x90`\0R` `\0 \x01`\0\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP[c\xFF\xFF\xFF\xFFC\x16\x82R`\xB3`\xFF\x84\x16a\x01\0\x81\x10a&\xD9Wa&\xD9a;#V[\x01\x80T`\x01\x81\x01\x82U`\0\x91\x82R` \x91\x82\x90 \x84Q\x91\x01\x80T\x92\x85\x01Q`@\x90\x95\x01Q`\x01`\x01``\x1B\x03\x16`\x01`@\x1B\x02k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x1B\x19c\xFF\xFF\xFF\xFF\x96\x87\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x95\x16\x96\x90\x93\x16\x95\x90\x95\x17\x92\x90\x92\x17\x16\x92\x90\x92\x17\x90\x91UPPV[`\0Ta\x01\0\x90\x04`\xFF\x16a'\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x08\xA3V[\x80Q\x82Q\x14a(2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`;`$\x82\x01R\x7FRegistry._initialize: minimumSta`D\x82\x01R\x7FkeForQuorum length mismatch\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08\xA3V[`\0[\x81Q\x81`\xFF\x16\x10\x15a\x12'Wa(g\x81\x84\x83`\xFF\x16\x81Q\x81\x10a(ZWa(Za;#V[` \x02` \x01\x01Qa0\xBFV[a(\x8C\x82\x82`\xFF\x16\x81Q\x81\x10a(\x7FWa(\x7Fa;#V[` \x02` \x01\x01Qa*\x0CV[`\x01\x01a(5V[`\0\x83\x81Ra\x01\xB3` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 T\x81\x90\x80\x15a)wW`\0\x86\x81Ra\x01\xB3` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x90\x91R\x90 C\x90a(\xE4`\x01\x84a;\xC1V[\x81T\x81\x10a(\xF4Wa(\xF4a;#V[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x80Tc\xFF\xFF\xFF\xFF\x94\x90\x94\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x94\x16\x93\x90\x93\x17\x90\x92U\x87\x81Ra\x01\xB3\x82R`@\x80\x82 `\xFF\x89\x16\x83R\x90\x92R a)H`\x01\x83a;\xC1V[\x81T\x81\x10a)XWa)Xa;#V[`\0\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x91P[P`\0\x85\x81Ra\x01\xB3` \x90\x81R`@\x80\x83 `\xFF\x88\x16\x84R\x82R\x80\x83 \x80T`\x01\x81\x01\x82U\x90\x84R\x92\x82\x90 \x86Q\x93\x01\x80T\x92\x87\x01Q\x91\x87\x01Q`\x01`\x01``\x1B\x03\x16`\x01`@\x1B\x02k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x1B\x19c\xFF\xFF\xFF\xFF\x93\x84\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x95\x16\x93\x90\x95\x16\x92\x90\x92\x17\x92\x90\x92\x17\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x90P\x93\x92PPPV[`\0Tb\x01\0\0\x90\x04a\xFF\xFF\x16`\xC0\x81\x10a*\xA7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`O`$\x82\x01R\x7FVoteWeigherBase._createQuorum: n`D\x82\x01R\x7Fumber of quorums cannot exceed M`d\x82\x01Rn\x10V\x17\xD4US\xD4\x95SW\xD0\xD3\xD5S\x95`\x8A\x1B`\x84\x82\x01R`\xA4\x01a\x08\xA3V[\x80a*\xB3\x81`\x01a?\nV[`\0`\x02a\x01\0\n\x81T\x81a\xFF\xFF\x02\x19\x16\x90\x83a\xFF\xFF\x16\x02\x17\x90UPa*\xD9\x81\x84a,\x8DV[`@Q`\xFF\x82\x16\x90\x7F\x83\x1A\x9C\x86\xC4[\xB3\x03\xCA\xF3\xF0d\xBE+\xC2\xB9\xFDN\xCF\x19\xE4|J\xC0*a\xE7]\xAB\xFEU\xB4\x90`\0\x90\xA2PPPV[\x81Qc\xFF\xFF\xFF\xFF\x80\x83\x16\x91\x16\x11\x15a+\xB2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R```$\x82\x01R\x7FStakeRegistry._validateOperatorS`D\x82\x01R\x7FtakeAtBlockNumber: operatorStake`d\x82\x01R\x7FUpdate is from after blockNumber`\x84\x82\x01R`\xA4\x01a\x08\xA3V[` \x82\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x80a+\xD8WP\x80c\xFF\xFF\xFF\xFF\x16\x82` \x01Qc\xFF\xFF\xFF\xFF\x16\x11[a\x13\nW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`t`$\x82\x01R\x7FStakeRegistry._validateOperatorS`D\x82\x01R\x7FtakeAtBlockNumber: there is a ne`d\x82\x01R\x7Fwer operatorStakeUpdate availabl`\x84\x82\x01Rs2\x9012\xB37\xB92\x90167\xB1\xB5\xA7:\xB6\xB12\xB9`a\x1B`\xA4\x82\x01R`\xC4\x01a\x08\xA3V[`\0\x81Q\x11a-\tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`N`$\x82\x01R`\0\x80Q` a?@\x839\x81Q\x91R`D\x82\x01R\x7FnsideredAndMultipliers: no strat`d\x82\x01Rm\x19Y\xDAY\\\xC8\x1C\x1C\x9B\xDD\x9AY\x19Y`\x92\x1B`\x84\x82\x01R`\xA4\x01a\x08\xA3V[\x80Q`\xFF\x83\x16`\0\x90\x81R`\x01` \x90\x81R`@\x90\x91 T\x90a-,\x83\x83a?'V[\x11\x15a-\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`[`$\x82\x01R`\0\x80Q` a?@\x839\x81Q\x91R`D\x82\x01R\x7FnsideredAndMultipliers: exceed M`d\x82\x01R\x7FAX_WEIGHING_FUNCTION_LENGTH\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x08\xA3V[`\0[\x82\x81\x10\x15a0\xB8W`\0[a-\xCC\x82\x84a?'V[\x81\x10\x15a.\xBFW\x84\x82\x81Q\x81\x10a-\xE5Wa-\xE5a;#V[` \x02` \x01\x01Q`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\0\x88`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x82\x81T\x81\x10a.$Wa.$a;#V[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a.\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`S`$\x82\x01R`\0\x80Q` a?@\x839\x81Q\x91R`D\x82\x01R\x7FnsideredAndMultipliers: cannot a`d\x82\x01Rr\x0C\x8C\x84\x0El-\xAC\xA4\x0En\x8EL.\x8C\xAC\xEF$\x06O`k\x1B`\x84\x82\x01R`\xA4\x01a\x08\xA3V[`\x01\x01a-\xC2V[P`\0\x84\x82\x81Q\x81\x10a.\xD4Wa.\xD4a;#V[` \x02` \x01\x01Q` \x01Q`\x01`\x01``\x1B\x03\x16\x11a/pW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R`\0\x80Q` a?@\x839\x81Q\x91R`D\x82\x01R\x7FnsideredAndMultipliers: cannot a`d\x82\x01R\x7Fdd strategy with zero weight\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x08\xA3V[`\xFF\x85\x16`\0\x90\x81R`\x01` R`@\x90 \x84Q\x85\x90\x83\x90\x81\x10a/\x96Wa/\x96a;#V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82T`\x01\x81\x01\x84U`\0\x93\x84R\x92\x82\x90 \x81Q\x91\x90\x92\x01Q`\x01`\x01``\x1B\x03\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x91\x01U\x83Q`\xFF\x86\x16\x90\x7F\x10V^V\xCA\xCB\xF3.\xCA&yE\xF0T\xFE\xC0.Yu\x002\xD1\x13\xD30!\x82\xAD\x96\x7FT\x04\x90\x86\x90\x84\x90\x81\x10a0\x13Wa0\x13a;#V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQ`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x01`@Q\x80\x91\x03\x90\xA2\x84`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x85\x83\x81Q\x81\x10a0pWa0pa;#V[` \x02` \x01\x01Q`\0\x01Q\x86\x84\x81Q\x81\x10a0\x8EWa0\x8Ea;#V[` \x02` \x01\x01Q` \x01Q`@Qa0\xA8\x92\x91\x90a;\x01V[`@Q\x80\x91\x03\x90\xA2`\x01\x01a-\xB7V[PPPPPV[\x80`3\x83`\xFF\x16a\x01\0\x81\x10a0\xD7Wa0\xD7a;#V[`\x02\x91\x82\x82\x04\x01\x91\x90\x06`\x0C\x02a\x01\0\n\x81T\x81`\x01`\x01``\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01``\x1B\x03\x16\x02\x17\x90UP\x81`\xFF\x16\x7F&\xEE\xCF\xF2\xB7\x0B\nq\x10O\xF4\xD9@\xBAqb\xD2:\x95\xC2Hw\x1F\xC4\x87\xA7\xBE\x17\xA5\x96\xB3\xCF\x82`@Qa1H\x91\x90`\x01`\x01``\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PPV[\x805`\xFF\x81\x16\x81\x14a1eW`\0\x80\xFD[\x91\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a1eW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a1\x93W`\0\x80\xFD[\x835\x92Pa1\xA3` \x85\x01a1TV[\x91Pa1\xB1`@\x85\x01a1jV[\x90P\x92P\x92P\x92V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x14hW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a1\xE2W`\0\x80\xFD[a1\xEB\x83a1TV[\x91P` \x83\x015a1\xFB\x81a1\xBAV[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a2\x1BW`\0\x80\xFD[a2$\x84a1TV[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[\x81Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x80\x84\x01Q\x90\x91\x16\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01``\x1B\x03\x16\x90\x82\x01R``\x81\x01a\x07\xBCV[`\0\x80\x83`\x1F\x84\x01\x12a2\x80W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a2\x97W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a2\xAFW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a2\xCCW`\0\x80\xFD[\x845a2\xD7\x81a1\xBAV[\x93P` \x85\x015\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a2\xF9W`\0\x80\xFD[a3\x05\x87\x82\x88\x01a2nV[\x95\x98\x94\x97P\x95PPPPV[`\0\x80\x83`\x1F\x84\x01\x12a3#W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a3:W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a2\xAFW`\0\x80\xFD[`\0\x80`\0`@\x84\x86\x03\x12\x15a3jW`\0\x80\xFD[a3s\x84a1TV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a3\x8EW`\0\x80\xFD[a3\x9A\x86\x82\x87\x01a3\x11V[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a3\xBFW`\0\x80\xFD[a3\xC8\x86a1TV[\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a3\xE4W`\0\x80\xFD[a3\xF0\x89\x83\x8A\x01a3\x11V[\x90\x96P\x94P`@\x88\x015\x91P\x80\x82\x11\x15a4\tW`\0\x80\xFD[Pa4\x16\x88\x82\x89\x01a3\x11V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[\x805`\x01`\x01``\x1B\x03\x81\x16\x81\x14a1eW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a4SW`\0\x80\xFD[a4\\\x84a1TV[\x92P` \x84\x015a4l\x81a1\xBAV[\x91Pa1\xB1`@\x85\x01a4'V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a4\xB2Wa4\xB2a4zV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a4\xE0Wa4\xE0a4zV[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a5\x01Wa5\x01a4zV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a5\x1CW`\0\x80\xFD[\x815` a51a5,\x83a4\xE8V[a4\xB8V[\x82\x81R`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a5PW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a5\x9FW`@\x81\x89\x03\x12\x15a5mW`\0\x80\x81\xFD[a5ua4\x90V[\x815a5\x80\x81a1\xBAV[\x81Ra5\x8D\x82\x86\x01a4'V[\x81\x86\x01R\x83R\x91\x83\x01\x91`@\x01a5TV[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a5\xBBW`\0\x80\xFD[\x815` a5\xCBa5,\x83a4\xE8V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a5\xEAW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a5\x9FW\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a6\rW`\0\x80\x81\xFD[a6\x1B\x89\x86\x83\x8B\x01\x01a5\x0BV[\x84RP\x91\x83\x01\x91\x83\x01a5\xEEV[`\0\x80`@\x83\x85\x03\x12\x15a6<W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a6SW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a6gW`\0\x80\xFD[\x815` a6wa5,\x83a4\xE8V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a6\x96W`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a6\xBBWa6\xAC\x86a4'V[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a6\x9BV[\x96PP\x86\x015\x92PP\x80\x82\x11\x15a6\xD1W`\0\x80\xFD[Pa6\xDE\x85\x82\x86\x01a5\xAAV[\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a6\xFBW`\0\x80\xFD[\x825\x91Pa7\x0B` \x84\x01a1TV[\x90P\x92P\x92\x90PV[`\0``\x82\x84\x03\x12\x15a7&W`\0\x80\xFD[`@Q``\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a7HWa7Ha4zV[`@R\x90P\x80a7W\x83a1jV[\x81Ra7e` \x84\x01a1jV[` \x82\x01Ra7v`@\x84\x01a4'V[`@\x82\x01RP\x92\x91PPV[`\0\x80`\0`\xA0\x84\x86\x03\x12\x15a7\x97W`\0\x80\xFD[\x835\x92Pa7\xA7` \x85\x01a1TV[\x91Pa1\xB1\x85`@\x86\x01a7\x14V[`\0\x80`@\x83\x85\x03\x12\x15a7\xC9W`\0\x80\xFD[a7\xD2\x83a1TV[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a7\xF2W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\x80\x83\x85\x03\x12\x15a8\x0CW`\0\x80\xFD[a8\x15\x83a1TV[\x91Pa7\x0B\x84` \x85\x01a7\x14V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a8\x90Wa8}\x83\x85Qc\xFF\xFF\xFF\xFF\x80\x82Q\x16\x83R\x80` \x83\x01Q\x16` \x84\x01RP`\x01`\x01``\x1B\x03`@\x82\x01Q\x16`@\x83\x01RPPV[\x92\x84\x01\x92``\x92\x90\x92\x01\x91`\x01\x01a8@V[P\x90\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a8\xAEW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a8\xC4W`\0\x80\xFD[a\x11\x0F\x84\x82\x85\x01a5\x0BV[`\0` \x82\x84\x03\x12\x15a8\xE2W`\0\x80\xFD[a\x07\x83\x82a1TV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a9\x01W`\0\x80\xFD[a9\n\x85a1TV[\x93Pa9\x18` \x86\x01a1jV[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15a9BW`\0\x80\xFD[\x835a9M\x81a1\xBAV[\x92P` \x84\x015\x91Pa1\xB1`@\x85\x01a1TV[`\0\x80`@\x83\x85\x03\x12\x15a9uW`\0\x80\xFD[a9~\x83a1TV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a9\x99W`\0\x80\xFD[a6\xDE\x85\x82\x86\x01a5\x0BV[`\0\x80`@\x83\x85\x03\x12\x15a9\xB8W`\0\x80\xFD[a9\xC1\x83a1TV[\x91Pa7\x0B` \x84\x01a4'V[`\0\x80`\0`@\x84\x86\x03\x12\x15a9\xE4W`\0\x80\xFD[\x835\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a:\x01W`\0\x80\xFD[a3\x9A\x86\x82\x87\x01a2nV[`\0\x80`\0``\x84\x86\x03\x12\x15a:\"W`\0\x80\xFD[a:+\x84a1TV[\x92Pa:9` \x85\x01a1jV[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0\x80` \x83\x85\x03\x12\x15a:\\W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a:rW`\0\x80\xFD[a:~\x85\x82\x86\x01a3\x11V[\x90\x96\x90\x95P\x93PPPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a:\x9FW`\0\x80\xFD[a:\xA8\x84a1jV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a:\x01W`\0\x80\xFD[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a8\x90W\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a:\xDFV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R`\x01`\x01``\x1B\x03\x16` \x82\x01R`@\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[` \x80\x82R`L\x90\x82\x01R\x7FStakeRegistry.onlyRegistryCoordi`@\x82\x01R\x7Fnator: caller is not the Registr``\x82\x01Rk<\xA1\xB7\xB7\xB924\xB70\xBA7\xB9`\xA1\x1B`\x80\x82\x01R`\xA0\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a;\xD3Wa;\xD3a;\xABV[P\x03\x90V[` \x80\x82R`T\x90\x82\x01R\x7FStakeRegistry._registerOperator:`@\x82\x01R\x7F greatest quorumNumber must be l``\x82\x01Rs\x19\\\xDC\xC8\x1D\x1A\x18[\x88\x1C][\xDC\x9D[P\xDB\xDD[\x9D`b\x1B`\x80\x82\x01R`\xA0\x01\x90V[` \x80\x82R`\\\x90\x82\x01R\x7FStakeRegistry._registerOperator:`@\x82\x01R\x7F Operator does not meet minimum ``\x82\x01R\x7Fstake requirement for quorum\0\0\0\0`\x80\x82\x01R`\xA0\x01\x90V[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a<\xF7Wa<\xF7a;\xABV[\x01\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a=\x12W`\0\x80\xFD[\x81Qa\x07\x83\x81a1\xBAV[` \x80\x82R`V\x90\x82\x01R\x7FVoteWeigherBase.onlyServiceManag`@\x82\x01R\x7FerOwner: caller is not the owner``\x82\x01Ru\x107\xB3\x10:42\x909\xB2\xB9;4\xB1\xB2\xA6\xB0\xB70\xB3\xB2\xB9`Q\x1B`\x80\x82\x01R`\xA0\x01\x90V[` \x80\x82R`<\x90\x82\x01R\x7FVoteWeigherBase.validQuorumNumbe`@\x82\x01R\x7Fr: quorumNumber is not valid\0\0\0\0``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a>\x1EW`\0\x80\xFD[a\x07\x83\x82a4'V[`\0`\x01`\x01``\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a>GWa>Ga;\xABV[\x03\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a>aW`\0\x80\xFD[\x815a\x07\x83\x81a1\xBAV[`\0` \x82\x84\x03\x12\x15a>~W`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a>\x97W`\0\x80\xFD[\x81Q`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x14a\x07\x83W`\0\x80\xFD[`\0c\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a>GWa>Ga;\xABV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x14\x15a>\xE5Wa>\xE5a;\xABV[`\x01\x01\x93\x92PPPV[`\0`\0\x19\x82\x14\x15a?\x03Wa?\x03a;\xABV[P`\x01\x01\x90V[`\0a\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a<\xF7Wa<\xF7a;\xABV[`\0\x82\x19\x82\x11\x15a?:Wa?:a;\xABV[P\x01\x90V\xFEVoteWeigherBase._addStrategiesCo\xA2dipfsX\"\x12 oY@\xCF@\x84x\xDE\xA4S\x079Y`\xB7\xB6\xC4\x8F\x83\xABRr\xD2\x10\xDB*\x92\x0F]\xD1\xE1\x16dsolcC\0\x08\x0C\x003";
    /// The deployed bytecode of the contract.
    pub static STAKEREGISTRYHARNESS_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct StakeRegistryHarness<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for StakeRegistryHarness<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for StakeRegistryHarness<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for StakeRegistryHarness<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for StakeRegistryHarness<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(StakeRegistryHarness))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> StakeRegistryHarness<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    STAKEREGISTRYHARNESS_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                STAKEREGISTRYHARNESS_ABI.clone(),
                STAKEREGISTRYHARNESS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `MAX_QUORUM_COUNT` (0xa6734667) function
        pub fn max_quorum_count(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([166, 115, 70, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MAX_WEIGHING_FUNCTION_LENGTH` (0x7c172347) function
        pub fn max_weighing_function_length(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([124, 23, 35, 71], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `WEIGHTING_DIVISOR` (0x5e5a6775) function
        pub fn weighting_divisor(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([94, 90, 103, 117], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addStrategiesConsideredAndMultipliers` (0xbba549fa) function
        pub fn add_strategies_considered_and_multipliers(
            &self,
            quorum_number: u8,
            new_strategies_considered_and_multipliers: ::std::vec::Vec<
                StrategyAndWeightingMultiplier,
            >,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [187, 165, 73, 250],
                    (quorum_number, new_strategies_considered_and_multipliers),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createQuorum` (0x99eed4ee) function
        pub fn create_quorum(
            &self,
            strategies_considered_and_multipliers: ::std::vec::Vec<
                StrategyAndWeightingMultiplier,
            >,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([153, 238, 212, 238], strategies_considered_and_multipliers)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `delegation` (0xdf5cf723) function
        pub fn delegation(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([223, 92, 247, 35], ())
                .expect("method not found (this should never happen)")
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
        ///Calls the contract's `getLengthOfOperatorIdStakeHistoryForQuorum` (0x6e8f03ca) function
        pub fn get_length_of_operator_id_stake_history_for_quorum(
            &self,
            operator_id: [u8; 32],
            quorum_number: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([110, 143, 3, 202], (operator_id, quorum_number))
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
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<OperatorStakeUpdate>,
        > {
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
                .method_hash(
                    [72, 8, 88, 102],
                    (operator_id, quorum_number, block_number),
                )
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
        ///Calls the contract's `initialize` (0x5f2948ec) function
        pub fn initialize(
            &self,
            minimum_stake_for_quorum: ::std::vec::Vec<u128>,
            quorum_strategies_considered_and_multipliers: ::std::vec::Vec<
                ::std::vec::Vec<StrategyAndWeightingMultiplier>,
            >,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [95, 41, 72, 236],
                    (
                        minimum_stake_for_quorum,
                        quorum_strategies_considered_and_multipliers,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `minimumStakeForQuorum` (0x7ed9430f) function
        pub fn minimum_stake_for_quorum(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([126, 217, 67, 15], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `modifyStrategyWeights` (0x2c2a5d2b) function
        pub fn modify_strategy_weights(
            &self,
            quorum_number: u8,
            strategy_indices: ::std::vec::Vec<::ethers::core::types::U256>,
            new_multipliers: ::std::vec::Vec<u128>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [44, 42, 93, 43],
                    (quorum_number, strategy_indices, new_multipliers),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quorumCount` (0x9aa1653d) function
        pub fn quorum_count(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([154, 161, 101, 61], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `recordOperatorStakeUpdate` (0x7ca9ff00) function
        pub fn record_operator_stake_update(
            &self,
            operator_id: [u8; 32],
            quorum_number: u8,
            operator_stake_update: OperatorStakeUpdate,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash(
                    [124, 169, 255, 0],
                    (operator_id, quorum_number, operator_stake_update),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `recordTotalStakeUpdate` (0x9029047d) function
        pub fn record_total_stake_update(
            &self,
            quorum_number: u8,
            total_stake_update: OperatorStakeUpdate,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([144, 41, 4, 125], (quorum_number, total_stake_update))
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
        ///Calls the contract's `registerOperatorNonCoordinator` (0xf5045894) function
        pub fn register_operator_non_coordinator(
            &self,
            operator: ::ethers::core::types::Address,
            operator_id: [u8; 32],
            quorum_numbers: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([245, 4, 88, 148], (operator, operator_id, quorum_numbers))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registryCoordinator` (0x6d14a987) function
        pub fn registry_coordinator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([109, 20, 169, 135], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeStrategiesConsideredAndMultipliers` (0x2b3d8816) function
        pub fn remove_strategies_considered_and_multipliers(
            &self,
            quorum_number: u8,
            indices_to_remove: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([43, 61, 136, 22], (quorum_number, indices_to_remove))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `serviceManager` (0x3998fdd3) function
        pub fn service_manager(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([57, 152, 253, 211], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setMinimumStakeForQuorum` (0xbc9a40c3) function
        pub fn set_minimum_stake_for_quorum(
            &self,
            quorum_number: u8,
            minimum_stake: u128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([188, 154, 64, 195], (quorum_number, minimum_stake))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setOperatorWeight` (0x4caf48c2) function
        pub fn set_operator_weight(
            &self,
            quorum_number: u8,
            operator: ::ethers::core::types::Address,
            weight: u128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([76, 175, 72, 194], (quorum_number, operator, weight))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `slasher` (0xb1344271) function
        pub fn slasher(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([177, 52, 66, 113], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `strategiesConsideredAndMultipliers` (0xeb92199c) function
        pub fn strategies_considered_and_multipliers(
            &self,
            p0: u8,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::Address, u128),
        > {
            self.0
                .method_hash([235, 146, 25, 156], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `strategiesConsideredAndMultipliersLength` (0x9e8ca620) function
        pub fn strategies_considered_and_multipliers_length(
            &self,
            quorum_number: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([158, 140, 166, 32], quorum_number)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `strategyAndWeightingMultiplierForQuorumByIndex` (0x7cc0d75f) function
        pub fn strategy_and_weighting_multiplier_for_quorum_by_index(
            &self,
            quorum_number: u8,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            StrategyAndWeightingMultiplier,
        > {
            self.0
                .method_hash([124, 192, 215, 95], (quorum_number, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `strategyManager` (0x39b70e38) function
        pub fn strategy_manager(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([57, 183, 14, 56], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateOperatorStake` (0xb3371baa) function
        pub fn update_operator_stake(
            &self,
            operator: ::ethers::core::types::Address,
            operator_id: [u8; 32],
            quorum_number: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, (u128, u128)> {
            self.0
                .method_hash([179, 55, 27, 170], (operator, operator_id, quorum_number))
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
        ///Calls the contract's `weightOfOperatorForQuorum` (0x1f9b74e0) function
        pub fn weight_of_operator_for_quorum(
            &self,
            quorum_number: u8,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([31, 155, 116, 224], (quorum_number, operator))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Initialized` event
        pub fn initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            InitializedFilter,
        > {
            self.0.event()
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
        ///Gets the contract's `QuorumCreated` event
        pub fn quorum_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            QuorumCreatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `StakeUpdate` event
        pub fn stake_update_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StakeUpdateFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `StrategyAddedToQuorum` event
        pub fn strategy_added_to_quorum_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StrategyAddedToQuorumFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `StrategyMultiplierUpdated` event
        pub fn strategy_multiplier_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StrategyMultiplierUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `StrategyRemovedFromQuorum` event
        pub fn strategy_removed_from_quorum_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StrategyRemovedFromQuorumFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StakeRegistryHarnessEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for StakeRegistryHarness<M> {
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
        Hash
    )]
    #[ethevent(name = "Initialized", abi = "Initialized(uint8)")]
    pub struct InitializedFilter {
        pub version: u8,
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
        Hash
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
        Hash
    )]
    #[ethevent(name = "QuorumCreated", abi = "QuorumCreated(uint8)")]
    pub struct QuorumCreatedFilter {
        #[ethevent(indexed)]
        pub quorum_number: u8,
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
        Hash
    )]
    #[ethevent(name = "StakeUpdate", abi = "StakeUpdate(bytes32,uint8,uint96)")]
    pub struct StakeUpdateFilter {
        #[ethevent(indexed)]
        pub operator_id: [u8; 32],
        pub quorum_number: u8,
        pub stake: u128,
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
        Hash
    )]
    #[ethevent(
        name = "StrategyAddedToQuorum",
        abi = "StrategyAddedToQuorum(uint8,address)"
    )]
    pub struct StrategyAddedToQuorumFilter {
        #[ethevent(indexed)]
        pub quorum_number: u8,
        pub strategy: ::ethers::core::types::Address,
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
        Hash
    )]
    #[ethevent(
        name = "StrategyMultiplierUpdated",
        abi = "StrategyMultiplierUpdated(uint8,address,uint256)"
    )]
    pub struct StrategyMultiplierUpdatedFilter {
        #[ethevent(indexed)]
        pub quorum_number: u8,
        pub strategy: ::ethers::core::types::Address,
        pub multiplier: ::ethers::core::types::U256,
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
        Hash
    )]
    #[ethevent(
        name = "StrategyRemovedFromQuorum",
        abi = "StrategyRemovedFromQuorum(uint8,address)"
    )]
    pub struct StrategyRemovedFromQuorumFilter {
        #[ethevent(indexed)]
        pub quorum_number: u8,
        pub strategy: ::ethers::core::types::Address,
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
        Hash
    )]
    pub enum StakeRegistryHarnessEvents {
        InitializedFilter(InitializedFilter),
        MinimumStakeForQuorumUpdatedFilter(MinimumStakeForQuorumUpdatedFilter),
        QuorumCreatedFilter(QuorumCreatedFilter),
        StakeUpdateFilter(StakeUpdateFilter),
        StrategyAddedToQuorumFilter(StrategyAddedToQuorumFilter),
        StrategyMultiplierUpdatedFilter(StrategyMultiplierUpdatedFilter),
        StrategyRemovedFromQuorumFilter(StrategyRemovedFromQuorumFilter),
    }
    impl ::ethers::contract::EthLogDecode for StakeRegistryHarnessEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(StakeRegistryHarnessEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = MinimumStakeForQuorumUpdatedFilter::decode_log(log) {
                return Ok(
                    StakeRegistryHarnessEvents::MinimumStakeForQuorumUpdatedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = QuorumCreatedFilter::decode_log(log) {
                return Ok(StakeRegistryHarnessEvents::QuorumCreatedFilter(decoded));
            }
            if let Ok(decoded) = StakeUpdateFilter::decode_log(log) {
                return Ok(StakeRegistryHarnessEvents::StakeUpdateFilter(decoded));
            }
            if let Ok(decoded) = StrategyAddedToQuorumFilter::decode_log(log) {
                return Ok(
                    StakeRegistryHarnessEvents::StrategyAddedToQuorumFilter(decoded),
                );
            }
            if let Ok(decoded) = StrategyMultiplierUpdatedFilter::decode_log(log) {
                return Ok(
                    StakeRegistryHarnessEvents::StrategyMultiplierUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = StrategyRemovedFromQuorumFilter::decode_log(log) {
                return Ok(
                    StakeRegistryHarnessEvents::StrategyRemovedFromQuorumFilter(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for StakeRegistryHarnessEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinimumStakeForQuorumUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::QuorumCreatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StakeUpdateFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::StrategyAddedToQuorumFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StrategyMultiplierUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StrategyRemovedFromQuorumFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<InitializedFilter> for StakeRegistryHarnessEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<MinimumStakeForQuorumUpdatedFilter>
    for StakeRegistryHarnessEvents {
        fn from(value: MinimumStakeForQuorumUpdatedFilter) -> Self {
            Self::MinimumStakeForQuorumUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<QuorumCreatedFilter> for StakeRegistryHarnessEvents {
        fn from(value: QuorumCreatedFilter) -> Self {
            Self::QuorumCreatedFilter(value)
        }
    }
    impl ::core::convert::From<StakeUpdateFilter> for StakeRegistryHarnessEvents {
        fn from(value: StakeUpdateFilter) -> Self {
            Self::StakeUpdateFilter(value)
        }
    }
    impl ::core::convert::From<StrategyAddedToQuorumFilter>
    for StakeRegistryHarnessEvents {
        fn from(value: StrategyAddedToQuorumFilter) -> Self {
            Self::StrategyAddedToQuorumFilter(value)
        }
    }
    impl ::core::convert::From<StrategyMultiplierUpdatedFilter>
    for StakeRegistryHarnessEvents {
        fn from(value: StrategyMultiplierUpdatedFilter) -> Self {
            Self::StrategyMultiplierUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<StrategyRemovedFromQuorumFilter>
    for StakeRegistryHarnessEvents {
        fn from(value: StrategyRemovedFromQuorumFilter) -> Self {
            Self::StrategyRemovedFromQuorumFilter(value)
        }
    }
    ///Container type for all input parameters for the `MAX_QUORUM_COUNT` function with signature `MAX_QUORUM_COUNT()` and selector `0xa6734667`
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
        Hash
    )]
    #[ethcall(name = "MAX_QUORUM_COUNT", abi = "MAX_QUORUM_COUNT()")]
    pub struct MaxQuorumCountCall;
    ///Container type for all input parameters for the `MAX_WEIGHING_FUNCTION_LENGTH` function with signature `MAX_WEIGHING_FUNCTION_LENGTH()` and selector `0x7c172347`
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
        Hash
    )]
    #[ethcall(
        name = "MAX_WEIGHING_FUNCTION_LENGTH",
        abi = "MAX_WEIGHING_FUNCTION_LENGTH()"
    )]
    pub struct MaxWeighingFunctionLengthCall;
    ///Container type for all input parameters for the `WEIGHTING_DIVISOR` function with signature `WEIGHTING_DIVISOR()` and selector `0x5e5a6775`
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
        Hash
    )]
    #[ethcall(name = "WEIGHTING_DIVISOR", abi = "WEIGHTING_DIVISOR()")]
    pub struct WeightingDivisorCall;
    ///Container type for all input parameters for the `addStrategiesConsideredAndMultipliers` function with signature `addStrategiesConsideredAndMultipliers(uint8,(address,uint96)[])` and selector `0xbba549fa`
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
        Hash
    )]
    #[ethcall(
        name = "addStrategiesConsideredAndMultipliers",
        abi = "addStrategiesConsideredAndMultipliers(uint8,(address,uint96)[])"
    )]
    pub struct AddStrategiesConsideredAndMultipliersCall {
        pub quorum_number: u8,
        pub new_strategies_considered_and_multipliers: ::std::vec::Vec<
            StrategyAndWeightingMultiplier,
        >,
    }
    ///Container type for all input parameters for the `createQuorum` function with signature `createQuorum((address,uint96)[])` and selector `0x99eed4ee`
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
        Hash
    )]
    #[ethcall(name = "createQuorum", abi = "createQuorum((address,uint96)[])")]
    pub struct CreateQuorumCall {
        pub strategies_considered_and_multipliers: ::std::vec::Vec<
            StrategyAndWeightingMultiplier,
        >,
    }
    ///Container type for all input parameters for the `delegation` function with signature `delegation()` and selector `0xdf5cf723`
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
        Hash
    )]
    #[ethcall(name = "delegation", abi = "delegation()")]
    pub struct DelegationCall;
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
        Hash
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
        Hash
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
        Hash
    )]
    #[ethcall(
        name = "getCurrentTotalStakeForQuorum",
        abi = "getCurrentTotalStakeForQuorum(uint8)"
    )]
    pub struct GetCurrentTotalStakeForQuorumCall {
        pub quorum_number: u8,
    }
    ///Container type for all input parameters for the `getLengthOfOperatorIdStakeHistoryForQuorum` function with signature `getLengthOfOperatorIdStakeHistoryForQuorum(bytes32,uint8)` and selector `0x6e8f03ca`
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
        Hash
    )]
    #[ethcall(
        name = "getLengthOfOperatorIdStakeHistoryForQuorum",
        abi = "getLengthOfOperatorIdStakeHistoryForQuorum(bytes32,uint8)"
    )]
    pub struct GetLengthOfOperatorIdStakeHistoryForQuorumCall {
        pub operator_id: [u8; 32],
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
    )]
    #[ethcall(
        name = "getTotalStakeUpdateForQuorumFromIndex",
        abi = "getTotalStakeUpdateForQuorumFromIndex(uint8,uint256)"
    )]
    pub struct GetTotalStakeUpdateForQuorumFromIndexCall {
        pub quorum_number: u8,
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize(uint96[],(address,uint96)[][])` and selector `0x5f2948ec`
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
        Hash
    )]
    #[ethcall(name = "initialize", abi = "initialize(uint96[],(address,uint96)[][])")]
    pub struct InitializeCall {
        pub minimum_stake_for_quorum: ::std::vec::Vec<u128>,
        pub quorum_strategies_considered_and_multipliers: ::std::vec::Vec<
            ::std::vec::Vec<StrategyAndWeightingMultiplier>,
        >,
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
        Hash
    )]
    #[ethcall(name = "minimumStakeForQuorum", abi = "minimumStakeForQuorum(uint256)")]
    pub struct MinimumStakeForQuorumCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `modifyStrategyWeights` function with signature `modifyStrategyWeights(uint8,uint256[],uint96[])` and selector `0x2c2a5d2b`
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
        Hash
    )]
    #[ethcall(
        name = "modifyStrategyWeights",
        abi = "modifyStrategyWeights(uint8,uint256[],uint96[])"
    )]
    pub struct ModifyStrategyWeightsCall {
        pub quorum_number: u8,
        pub strategy_indices: ::std::vec::Vec<::ethers::core::types::U256>,
        pub new_multipliers: ::std::vec::Vec<u128>,
    }
    ///Container type for all input parameters for the `quorumCount` function with signature `quorumCount()` and selector `0x9aa1653d`
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
        Hash
    )]
    #[ethcall(name = "quorumCount", abi = "quorumCount()")]
    pub struct QuorumCountCall;
    ///Container type for all input parameters for the `recordOperatorStakeUpdate` function with signature `recordOperatorStakeUpdate(bytes32,uint8,(uint32,uint32,uint96))` and selector `0x7ca9ff00`
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
        Hash
    )]
    #[ethcall(
        name = "recordOperatorStakeUpdate",
        abi = "recordOperatorStakeUpdate(bytes32,uint8,(uint32,uint32,uint96))"
    )]
    pub struct RecordOperatorStakeUpdateCall {
        pub operator_id: [u8; 32],
        pub quorum_number: u8,
        pub operator_stake_update: OperatorStakeUpdate,
    }
    ///Container type for all input parameters for the `recordTotalStakeUpdate` function with signature `recordTotalStakeUpdate(uint8,(uint32,uint32,uint96))` and selector `0x9029047d`
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
        Hash
    )]
    #[ethcall(
        name = "recordTotalStakeUpdate",
        abi = "recordTotalStakeUpdate(uint8,(uint32,uint32,uint96))"
    )]
    pub struct RecordTotalStakeUpdateCall {
        pub quorum_number: u8,
        pub total_stake_update: OperatorStakeUpdate,
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
        Hash
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
    ///Container type for all input parameters for the `registerOperatorNonCoordinator` function with signature `registerOperatorNonCoordinator(address,bytes32,bytes)` and selector `0xf5045894`
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
        Hash
    )]
    #[ethcall(
        name = "registerOperatorNonCoordinator",
        abi = "registerOperatorNonCoordinator(address,bytes32,bytes)"
    )]
    pub struct RegisterOperatorNonCoordinatorCall {
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
        Hash
    )]
    #[ethcall(name = "registryCoordinator", abi = "registryCoordinator()")]
    pub struct RegistryCoordinatorCall;
    ///Container type for all input parameters for the `removeStrategiesConsideredAndMultipliers` function with signature `removeStrategiesConsideredAndMultipliers(uint8,uint256[])` and selector `0x2b3d8816`
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
        Hash
    )]
    #[ethcall(
        name = "removeStrategiesConsideredAndMultipliers",
        abi = "removeStrategiesConsideredAndMultipliers(uint8,uint256[])"
    )]
    pub struct RemoveStrategiesConsideredAndMultipliersCall {
        pub quorum_number: u8,
        pub indices_to_remove: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all input parameters for the `serviceManager` function with signature `serviceManager()` and selector `0x3998fdd3`
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
        Hash
    )]
    #[ethcall(name = "serviceManager", abi = "serviceManager()")]
    pub struct ServiceManagerCall;
    ///Container type for all input parameters for the `setMinimumStakeForQuorum` function with signature `setMinimumStakeForQuorum(uint8,uint96)` and selector `0xbc9a40c3`
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
        Hash
    )]
    #[ethcall(
        name = "setMinimumStakeForQuorum",
        abi = "setMinimumStakeForQuorum(uint8,uint96)"
    )]
    pub struct SetMinimumStakeForQuorumCall {
        pub quorum_number: u8,
        pub minimum_stake: u128,
    }
    ///Container type for all input parameters for the `setOperatorWeight` function with signature `setOperatorWeight(uint8,address,uint96)` and selector `0x4caf48c2`
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
        Hash
    )]
    #[ethcall(
        name = "setOperatorWeight",
        abi = "setOperatorWeight(uint8,address,uint96)"
    )]
    pub struct SetOperatorWeightCall {
        pub quorum_number: u8,
        pub operator: ::ethers::core::types::Address,
        pub weight: u128,
    }
    ///Container type for all input parameters for the `slasher` function with signature `slasher()` and selector `0xb1344271`
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
        Hash
    )]
    #[ethcall(name = "slasher", abi = "slasher()")]
    pub struct SlasherCall;
    ///Container type for all input parameters for the `strategiesConsideredAndMultipliers` function with signature `strategiesConsideredAndMultipliers(uint8,uint256)` and selector `0xeb92199c`
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
        Hash
    )]
    #[ethcall(
        name = "strategiesConsideredAndMultipliers",
        abi = "strategiesConsideredAndMultipliers(uint8,uint256)"
    )]
    pub struct StrategiesConsideredAndMultipliersCall(
        pub u8,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `strategiesConsideredAndMultipliersLength` function with signature `strategiesConsideredAndMultipliersLength(uint8)` and selector `0x9e8ca620`
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
        Hash
    )]
    #[ethcall(
        name = "strategiesConsideredAndMultipliersLength",
        abi = "strategiesConsideredAndMultipliersLength(uint8)"
    )]
    pub struct StrategiesConsideredAndMultipliersLengthCall {
        pub quorum_number: u8,
    }
    ///Container type for all input parameters for the `strategyAndWeightingMultiplierForQuorumByIndex` function with signature `strategyAndWeightingMultiplierForQuorumByIndex(uint8,uint256)` and selector `0x7cc0d75f`
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
        Hash
    )]
    #[ethcall(
        name = "strategyAndWeightingMultiplierForQuorumByIndex",
        abi = "strategyAndWeightingMultiplierForQuorumByIndex(uint8,uint256)"
    )]
    pub struct StrategyAndWeightingMultiplierForQuorumByIndexCall {
        pub quorum_number: u8,
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `strategyManager` function with signature `strategyManager()` and selector `0x39b70e38`
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
        Hash
    )]
    #[ethcall(name = "strategyManager", abi = "strategyManager()")]
    pub struct StrategyManagerCall;
    ///Container type for all input parameters for the `updateOperatorStake` function with signature `updateOperatorStake(address,bytes32,uint8)` and selector `0xb3371baa`
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
        Hash
    )]
    #[ethcall(
        name = "updateOperatorStake",
        abi = "updateOperatorStake(address,bytes32,uint8)"
    )]
    pub struct UpdateOperatorStakeCall {
        pub operator: ::ethers::core::types::Address,
        pub operator_id: [u8; 32],
        pub quorum_number: u8,
    }
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
        Hash
    )]
    #[ethcall(name = "updateStakes", abi = "updateStakes(address[])")]
    pub struct UpdateStakesCall {
        pub operators: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `weightOfOperatorForQuorum` function with signature `weightOfOperatorForQuorum(uint8,address)` and selector `0x1f9b74e0`
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
        Hash
    )]
    #[ethcall(
        name = "weightOfOperatorForQuorum",
        abi = "weightOfOperatorForQuorum(uint8,address)"
    )]
    pub struct WeightOfOperatorForQuorumCall {
        pub quorum_number: u8,
        pub operator: ::ethers::core::types::Address,
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
        Hash
    )]
    pub enum StakeRegistryHarnessCalls {
        MaxQuorumCount(MaxQuorumCountCall),
        MaxWeighingFunctionLength(MaxWeighingFunctionLengthCall),
        WeightingDivisor(WeightingDivisorCall),
        AddStrategiesConsideredAndMultipliers(AddStrategiesConsideredAndMultipliersCall),
        CreateQuorum(CreateQuorumCall),
        Delegation(DelegationCall),
        DeregisterOperator(DeregisterOperatorCall),
        GetCurrentOperatorStakeForQuorum(GetCurrentOperatorStakeForQuorumCall),
        GetCurrentTotalStakeForQuorum(GetCurrentTotalStakeForQuorumCall),
        GetLengthOfOperatorIdStakeHistoryForQuorum(
            GetLengthOfOperatorIdStakeHistoryForQuorumCall,
        ),
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
        Initialize(InitializeCall),
        MinimumStakeForQuorum(MinimumStakeForQuorumCall),
        ModifyStrategyWeights(ModifyStrategyWeightsCall),
        QuorumCount(QuorumCountCall),
        RecordOperatorStakeUpdate(RecordOperatorStakeUpdateCall),
        RecordTotalStakeUpdate(RecordTotalStakeUpdateCall),
        RegisterOperator(RegisterOperatorCall),
        RegisterOperatorNonCoordinator(RegisterOperatorNonCoordinatorCall),
        RegistryCoordinator(RegistryCoordinatorCall),
        RemoveStrategiesConsideredAndMultipliers(
            RemoveStrategiesConsideredAndMultipliersCall,
        ),
        ServiceManager(ServiceManagerCall),
        SetMinimumStakeForQuorum(SetMinimumStakeForQuorumCall),
        SetOperatorWeight(SetOperatorWeightCall),
        Slasher(SlasherCall),
        StrategiesConsideredAndMultipliers(StrategiesConsideredAndMultipliersCall),
        StrategiesConsideredAndMultipliersLength(
            StrategiesConsideredAndMultipliersLengthCall,
        ),
        StrategyAndWeightingMultiplierForQuorumByIndex(
            StrategyAndWeightingMultiplierForQuorumByIndexCall,
        ),
        StrategyManager(StrategyManagerCall),
        UpdateOperatorStake(UpdateOperatorStakeCall),
        UpdateStakes(UpdateStakesCall),
        WeightOfOperatorForQuorum(WeightOfOperatorForQuorumCall),
    }
    impl ::ethers::core::abi::AbiDecode for StakeRegistryHarnessCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <MaxQuorumCountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxQuorumCount(decoded));
            }
            if let Ok(decoded) = <MaxWeighingFunctionLengthCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxWeighingFunctionLength(decoded));
            }
            if let Ok(decoded) = <WeightingDivisorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WeightingDivisor(decoded));
            }
            if let Ok(decoded) = <AddStrategiesConsideredAndMultipliersCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddStrategiesConsideredAndMultipliers(decoded));
            }
            if let Ok(decoded) = <CreateQuorumCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateQuorum(decoded));
            }
            if let Ok(decoded) = <DelegationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Delegation(decoded));
            }
            if let Ok(decoded) = <DeregisterOperatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeregisterOperator(decoded));
            }
            if let Ok(decoded) = <GetCurrentOperatorStakeForQuorumCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetCurrentOperatorStakeForQuorum(decoded));
            }
            if let Ok(decoded) = <GetCurrentTotalStakeForQuorumCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetCurrentTotalStakeForQuorum(decoded));
            }
            if let Ok(decoded) = <GetLengthOfOperatorIdStakeHistoryForQuorumCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetLengthOfOperatorIdStakeHistoryForQuorum(decoded));
            }
            if let Ok(decoded) = <GetLengthOfTotalStakeHistoryForQuorumCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetLengthOfTotalStakeHistoryForQuorum(decoded));
            }
            if let Ok(decoded) = <GetMostRecentStakeUpdateByOperatorIdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetMostRecentStakeUpdateByOperatorId(decoded));
            }
            if let Ok(decoded) = <GetOperatorIdToStakeHistoryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
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
            if let Ok(decoded) = <GetTotalStakeAtBlockNumberFromIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
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
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <MinimumStakeForQuorumCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MinimumStakeForQuorum(decoded));
            }
            if let Ok(decoded) = <ModifyStrategyWeightsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ModifyStrategyWeights(decoded));
            }
            if let Ok(decoded) = <QuorumCountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::QuorumCount(decoded));
            }
            if let Ok(decoded) = <RecordOperatorStakeUpdateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RecordOperatorStakeUpdate(decoded));
            }
            if let Ok(decoded) = <RecordTotalStakeUpdateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RecordTotalStakeUpdate(decoded));
            }
            if let Ok(decoded) = <RegisterOperatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RegisterOperator(decoded));
            }
            if let Ok(decoded) = <RegisterOperatorNonCoordinatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RegisterOperatorNonCoordinator(decoded));
            }
            if let Ok(decoded) = <RegistryCoordinatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RegistryCoordinator(decoded));
            }
            if let Ok(decoded) = <RemoveStrategiesConsideredAndMultipliersCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveStrategiesConsideredAndMultipliers(decoded));
            }
            if let Ok(decoded) = <ServiceManagerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ServiceManager(decoded));
            }
            if let Ok(decoded) = <SetMinimumStakeForQuorumCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetMinimumStakeForQuorum(decoded));
            }
            if let Ok(decoded) = <SetOperatorWeightCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetOperatorWeight(decoded));
            }
            if let Ok(decoded) = <SlasherCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Slasher(decoded));
            }
            if let Ok(decoded) = <StrategiesConsideredAndMultipliersCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StrategiesConsideredAndMultipliers(decoded));
            }
            if let Ok(decoded) = <StrategiesConsideredAndMultipliersLengthCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StrategiesConsideredAndMultipliersLength(decoded));
            }
            if let Ok(decoded) = <StrategyAndWeightingMultiplierForQuorumByIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StrategyAndWeightingMultiplierForQuorumByIndex(decoded));
            }
            if let Ok(decoded) = <StrategyManagerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StrategyManager(decoded));
            }
            if let Ok(decoded) = <UpdateOperatorStakeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateOperatorStake(decoded));
            }
            if let Ok(decoded) = <UpdateStakesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateStakes(decoded));
            }
            if let Ok(decoded) = <WeightOfOperatorForQuorumCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WeightOfOperatorForQuorum(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for StakeRegistryHarnessCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::MaxQuorumCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxWeighingFunctionLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WeightingDivisor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddStrategiesConsideredAndMultipliers(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateQuorum(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Delegation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeregisterOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetCurrentOperatorStakeForQuorum(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetCurrentTotalStakeForQuorum(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetLengthOfOperatorIdStakeHistoryForQuorum(element) => {
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
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MinimumStakeForQuorum(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ModifyStrategyWeights(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QuorumCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RecordOperatorStakeUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RecordTotalStakeUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterOperatorNonCoordinator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegistryCoordinator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveStrategiesConsideredAndMultipliers(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ServiceManager(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetMinimumStakeForQuorum(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetOperatorWeight(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Slasher(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StrategiesConsideredAndMultipliers(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StrategiesConsideredAndMultipliersLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StrategyAndWeightingMultiplierForQuorumByIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StrategyManager(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateOperatorStake(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateStakes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WeightOfOperatorForQuorum(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for StakeRegistryHarnessCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::MaxQuorumCount(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxWeighingFunctionLength(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WeightingDivisor(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddStrategiesConsideredAndMultipliers(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CreateQuorum(element) => ::core::fmt::Display::fmt(element, f),
                Self::Delegation(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeregisterOperator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetCurrentOperatorStakeForQuorum(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetCurrentTotalStakeForQuorum(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetLengthOfOperatorIdStakeHistoryForQuorum(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetLengthOfTotalStakeHistoryForQuorum(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetMostRecentStakeUpdateByOperatorId(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetOperatorIdToStakeHistory(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
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
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinimumStakeForQuorum(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ModifyStrategyWeights(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::QuorumCount(element) => ::core::fmt::Display::fmt(element, f),
                Self::RecordOperatorStakeUpdate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RecordTotalStakeUpdate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RegisterOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterOperatorNonCoordinator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RegistryCoordinator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemoveStrategiesConsideredAndMultipliers(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ServiceManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetMinimumStakeForQuorum(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetOperatorWeight(element) => ::core::fmt::Display::fmt(element, f),
                Self::Slasher(element) => ::core::fmt::Display::fmt(element, f),
                Self::StrategiesConsideredAndMultipliers(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StrategiesConsideredAndMultipliersLength(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StrategyAndWeightingMultiplierForQuorumByIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StrategyManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateOperatorStake(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateStakes(element) => ::core::fmt::Display::fmt(element, f),
                Self::WeightOfOperatorForQuorum(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<MaxQuorumCountCall> for StakeRegistryHarnessCalls {
        fn from(value: MaxQuorumCountCall) -> Self {
            Self::MaxQuorumCount(value)
        }
    }
    impl ::core::convert::From<MaxWeighingFunctionLengthCall>
    for StakeRegistryHarnessCalls {
        fn from(value: MaxWeighingFunctionLengthCall) -> Self {
            Self::MaxWeighingFunctionLength(value)
        }
    }
    impl ::core::convert::From<WeightingDivisorCall> for StakeRegistryHarnessCalls {
        fn from(value: WeightingDivisorCall) -> Self {
            Self::WeightingDivisor(value)
        }
    }
    impl ::core::convert::From<AddStrategiesConsideredAndMultipliersCall>
    for StakeRegistryHarnessCalls {
        fn from(value: AddStrategiesConsideredAndMultipliersCall) -> Self {
            Self::AddStrategiesConsideredAndMultipliers(value)
        }
    }
    impl ::core::convert::From<CreateQuorumCall> for StakeRegistryHarnessCalls {
        fn from(value: CreateQuorumCall) -> Self {
            Self::CreateQuorum(value)
        }
    }
    impl ::core::convert::From<DelegationCall> for StakeRegistryHarnessCalls {
        fn from(value: DelegationCall) -> Self {
            Self::Delegation(value)
        }
    }
    impl ::core::convert::From<DeregisterOperatorCall> for StakeRegistryHarnessCalls {
        fn from(value: DeregisterOperatorCall) -> Self {
            Self::DeregisterOperator(value)
        }
    }
    impl ::core::convert::From<GetCurrentOperatorStakeForQuorumCall>
    for StakeRegistryHarnessCalls {
        fn from(value: GetCurrentOperatorStakeForQuorumCall) -> Self {
            Self::GetCurrentOperatorStakeForQuorum(value)
        }
    }
    impl ::core::convert::From<GetCurrentTotalStakeForQuorumCall>
    for StakeRegistryHarnessCalls {
        fn from(value: GetCurrentTotalStakeForQuorumCall) -> Self {
            Self::GetCurrentTotalStakeForQuorum(value)
        }
    }
    impl ::core::convert::From<GetLengthOfOperatorIdStakeHistoryForQuorumCall>
    for StakeRegistryHarnessCalls {
        fn from(value: GetLengthOfOperatorIdStakeHistoryForQuorumCall) -> Self {
            Self::GetLengthOfOperatorIdStakeHistoryForQuorum(value)
        }
    }
    impl ::core::convert::From<GetLengthOfTotalStakeHistoryForQuorumCall>
    for StakeRegistryHarnessCalls {
        fn from(value: GetLengthOfTotalStakeHistoryForQuorumCall) -> Self {
            Self::GetLengthOfTotalStakeHistoryForQuorum(value)
        }
    }
    impl ::core::convert::From<GetMostRecentStakeUpdateByOperatorIdCall>
    for StakeRegistryHarnessCalls {
        fn from(value: GetMostRecentStakeUpdateByOperatorIdCall) -> Self {
            Self::GetMostRecentStakeUpdateByOperatorId(value)
        }
    }
    impl ::core::convert::From<GetOperatorIdToStakeHistoryCall>
    for StakeRegistryHarnessCalls {
        fn from(value: GetOperatorIdToStakeHistoryCall) -> Self {
            Self::GetOperatorIdToStakeHistory(value)
        }
    }
    impl ::core::convert::From<GetStakeForOperatorIdForQuorumAtBlockNumberCall>
    for StakeRegistryHarnessCalls {
        fn from(value: GetStakeForOperatorIdForQuorumAtBlockNumberCall) -> Self {
            Self::GetStakeForOperatorIdForQuorumAtBlockNumber(value)
        }
    }
    impl ::core::convert::From<GetStakeForQuorumAtBlockNumberFromOperatorIdAndIndexCall>
    for StakeRegistryHarnessCalls {
        fn from(
            value: GetStakeForQuorumAtBlockNumberFromOperatorIdAndIndexCall,
        ) -> Self {
            Self::GetStakeForQuorumAtBlockNumberFromOperatorIdAndIndex(value)
        }
    }
    impl ::core::convert::From<GetStakeUpdateForQuorumFromOperatorIdAndIndexCall>
    for StakeRegistryHarnessCalls {
        fn from(value: GetStakeUpdateForQuorumFromOperatorIdAndIndexCall) -> Self {
            Self::GetStakeUpdateForQuorumFromOperatorIdAndIndex(value)
        }
    }
    impl ::core::convert::From<
        GetStakeUpdateIndexForOperatorIdForQuorumAtBlockNumberCall,
    > for StakeRegistryHarnessCalls {
        fn from(
            value: GetStakeUpdateIndexForOperatorIdForQuorumAtBlockNumberCall,
        ) -> Self {
            Self::GetStakeUpdateIndexForOperatorIdForQuorumAtBlockNumber(value)
        }
    }
    impl ::core::convert::From<GetTotalStakeAtBlockNumberFromIndexCall>
    for StakeRegistryHarnessCalls {
        fn from(value: GetTotalStakeAtBlockNumberFromIndexCall) -> Self {
            Self::GetTotalStakeAtBlockNumberFromIndex(value)
        }
    }
    impl ::core::convert::From<GetTotalStakeIndicesByQuorumNumbersAtBlockNumberCall>
    for StakeRegistryHarnessCalls {
        fn from(value: GetTotalStakeIndicesByQuorumNumbersAtBlockNumberCall) -> Self {
            Self::GetTotalStakeIndicesByQuorumNumbersAtBlockNumber(value)
        }
    }
    impl ::core::convert::From<GetTotalStakeUpdateForQuorumFromIndexCall>
    for StakeRegistryHarnessCalls {
        fn from(value: GetTotalStakeUpdateForQuorumFromIndexCall) -> Self {
            Self::GetTotalStakeUpdateForQuorumFromIndex(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for StakeRegistryHarnessCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<MinimumStakeForQuorumCall> for StakeRegistryHarnessCalls {
        fn from(value: MinimumStakeForQuorumCall) -> Self {
            Self::MinimumStakeForQuorum(value)
        }
    }
    impl ::core::convert::From<ModifyStrategyWeightsCall> for StakeRegistryHarnessCalls {
        fn from(value: ModifyStrategyWeightsCall) -> Self {
            Self::ModifyStrategyWeights(value)
        }
    }
    impl ::core::convert::From<QuorumCountCall> for StakeRegistryHarnessCalls {
        fn from(value: QuorumCountCall) -> Self {
            Self::QuorumCount(value)
        }
    }
    impl ::core::convert::From<RecordOperatorStakeUpdateCall>
    for StakeRegistryHarnessCalls {
        fn from(value: RecordOperatorStakeUpdateCall) -> Self {
            Self::RecordOperatorStakeUpdate(value)
        }
    }
    impl ::core::convert::From<RecordTotalStakeUpdateCall>
    for StakeRegistryHarnessCalls {
        fn from(value: RecordTotalStakeUpdateCall) -> Self {
            Self::RecordTotalStakeUpdate(value)
        }
    }
    impl ::core::convert::From<RegisterOperatorCall> for StakeRegistryHarnessCalls {
        fn from(value: RegisterOperatorCall) -> Self {
            Self::RegisterOperator(value)
        }
    }
    impl ::core::convert::From<RegisterOperatorNonCoordinatorCall>
    for StakeRegistryHarnessCalls {
        fn from(value: RegisterOperatorNonCoordinatorCall) -> Self {
            Self::RegisterOperatorNonCoordinator(value)
        }
    }
    impl ::core::convert::From<RegistryCoordinatorCall> for StakeRegistryHarnessCalls {
        fn from(value: RegistryCoordinatorCall) -> Self {
            Self::RegistryCoordinator(value)
        }
    }
    impl ::core::convert::From<RemoveStrategiesConsideredAndMultipliersCall>
    for StakeRegistryHarnessCalls {
        fn from(value: RemoveStrategiesConsideredAndMultipliersCall) -> Self {
            Self::RemoveStrategiesConsideredAndMultipliers(value)
        }
    }
    impl ::core::convert::From<ServiceManagerCall> for StakeRegistryHarnessCalls {
        fn from(value: ServiceManagerCall) -> Self {
            Self::ServiceManager(value)
        }
    }
    impl ::core::convert::From<SetMinimumStakeForQuorumCall>
    for StakeRegistryHarnessCalls {
        fn from(value: SetMinimumStakeForQuorumCall) -> Self {
            Self::SetMinimumStakeForQuorum(value)
        }
    }
    impl ::core::convert::From<SetOperatorWeightCall> for StakeRegistryHarnessCalls {
        fn from(value: SetOperatorWeightCall) -> Self {
            Self::SetOperatorWeight(value)
        }
    }
    impl ::core::convert::From<SlasherCall> for StakeRegistryHarnessCalls {
        fn from(value: SlasherCall) -> Self {
            Self::Slasher(value)
        }
    }
    impl ::core::convert::From<StrategiesConsideredAndMultipliersCall>
    for StakeRegistryHarnessCalls {
        fn from(value: StrategiesConsideredAndMultipliersCall) -> Self {
            Self::StrategiesConsideredAndMultipliers(value)
        }
    }
    impl ::core::convert::From<StrategiesConsideredAndMultipliersLengthCall>
    for StakeRegistryHarnessCalls {
        fn from(value: StrategiesConsideredAndMultipliersLengthCall) -> Self {
            Self::StrategiesConsideredAndMultipliersLength(value)
        }
    }
    impl ::core::convert::From<StrategyAndWeightingMultiplierForQuorumByIndexCall>
    for StakeRegistryHarnessCalls {
        fn from(value: StrategyAndWeightingMultiplierForQuorumByIndexCall) -> Self {
            Self::StrategyAndWeightingMultiplierForQuorumByIndex(value)
        }
    }
    impl ::core::convert::From<StrategyManagerCall> for StakeRegistryHarnessCalls {
        fn from(value: StrategyManagerCall) -> Self {
            Self::StrategyManager(value)
        }
    }
    impl ::core::convert::From<UpdateOperatorStakeCall> for StakeRegistryHarnessCalls {
        fn from(value: UpdateOperatorStakeCall) -> Self {
            Self::UpdateOperatorStake(value)
        }
    }
    impl ::core::convert::From<UpdateStakesCall> for StakeRegistryHarnessCalls {
        fn from(value: UpdateStakesCall) -> Self {
            Self::UpdateStakes(value)
        }
    }
    impl ::core::convert::From<WeightOfOperatorForQuorumCall>
    for StakeRegistryHarnessCalls {
        fn from(value: WeightOfOperatorForQuorumCall) -> Self {
            Self::WeightOfOperatorForQuorum(value)
        }
    }
    ///Container type for all return fields from the `MAX_QUORUM_COUNT` function with signature `MAX_QUORUM_COUNT()` and selector `0xa6734667`
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
        Hash
    )]
    pub struct MaxQuorumCountReturn(pub u8);
    ///Container type for all return fields from the `MAX_WEIGHING_FUNCTION_LENGTH` function with signature `MAX_WEIGHING_FUNCTION_LENGTH()` and selector `0x7c172347`
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
        Hash
    )]
    pub struct MaxWeighingFunctionLengthReturn(pub u8);
    ///Container type for all return fields from the `WEIGHTING_DIVISOR` function with signature `WEIGHTING_DIVISOR()` and selector `0x5e5a6775`
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
        Hash
    )]
    pub struct WeightingDivisorReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `delegation` function with signature `delegation()` and selector `0xdf5cf723`
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
        Hash
    )]
    pub struct DelegationReturn(pub ::ethers::core::types::Address);
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
        Hash
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
        Hash
    )]
    pub struct GetCurrentTotalStakeForQuorumReturn(pub u128);
    ///Container type for all return fields from the `getLengthOfOperatorIdStakeHistoryForQuorum` function with signature `getLengthOfOperatorIdStakeHistoryForQuorum(bytes32,uint8)` and selector `0x6e8f03ca`
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
        Hash
    )]
    pub struct GetLengthOfOperatorIdStakeHistoryForQuorumReturn(
        pub ::ethers::core::types::U256,
    );
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
        Hash
    )]
    pub struct GetLengthOfTotalStakeHistoryForQuorumReturn(
        pub ::ethers::core::types::U256,
    );
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
        Hash
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
        Hash
    )]
    pub struct GetOperatorIdToStakeHistoryReturn(
        pub ::std::vec::Vec<OperatorStakeUpdate>,
    );
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
        Hash
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
        Hash
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
        Hash
    )]
    pub struct GetStakeUpdateForQuorumFromOperatorIdAndIndexReturn(
        pub OperatorStakeUpdate,
    );
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
        Hash
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
        Hash
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
        Hash
    )]
    pub struct GetTotalStakeIndicesByQuorumNumbersAtBlockNumberReturn(
        pub ::std::vec::Vec<u32>,
    );
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
        Hash
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
        Hash
    )]
    pub struct MinimumStakeForQuorumReturn(pub u128);
    ///Container type for all return fields from the `quorumCount` function with signature `quorumCount()` and selector `0x9aa1653d`
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
        Hash
    )]
    pub struct QuorumCountReturn(pub u16);
    ///Container type for all return fields from the `recordOperatorStakeUpdate` function with signature `recordOperatorStakeUpdate(bytes32,uint8,(uint32,uint32,uint96))` and selector `0x7ca9ff00`
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
        Hash
    )]
    pub struct RecordOperatorStakeUpdateReturn(pub u128);
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
        Hash
    )]
    pub struct RegistryCoordinatorReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `serviceManager` function with signature `serviceManager()` and selector `0x3998fdd3`
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
        Hash
    )]
    pub struct ServiceManagerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `slasher` function with signature `slasher()` and selector `0xb1344271`
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
        Hash
    )]
    pub struct SlasherReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `strategiesConsideredAndMultipliers` function with signature `strategiesConsideredAndMultipliers(uint8,uint256)` and selector `0xeb92199c`
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
        Hash
    )]
    pub struct StrategiesConsideredAndMultipliersReturn {
        pub strategy: ::ethers::core::types::Address,
        pub multiplier: u128,
    }
    ///Container type for all return fields from the `strategiesConsideredAndMultipliersLength` function with signature `strategiesConsideredAndMultipliersLength(uint8)` and selector `0x9e8ca620`
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
        Hash
    )]
    pub struct StrategiesConsideredAndMultipliersLengthReturn(
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `strategyAndWeightingMultiplierForQuorumByIndex` function with signature `strategyAndWeightingMultiplierForQuorumByIndex(uint8,uint256)` and selector `0x7cc0d75f`
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
        Hash
    )]
    pub struct StrategyAndWeightingMultiplierForQuorumByIndexReturn(
        pub StrategyAndWeightingMultiplier,
    );
    ///Container type for all return fields from the `strategyManager` function with signature `strategyManager()` and selector `0x39b70e38`
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
        Hash
    )]
    pub struct StrategyManagerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `updateOperatorStake` function with signature `updateOperatorStake(address,bytes32,uint8)` and selector `0xb3371baa`
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
        Hash
    )]
    pub struct UpdateOperatorStakeReturn(pub u128, pub u128);
    ///Container type for all return fields from the `weightOfOperatorForQuorum` function with signature `weightOfOperatorForQuorum(uint8,address)` and selector `0x1f9b74e0`
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
        Hash
    )]
    pub struct WeightOfOperatorForQuorumReturn(pub u128);
}
