pub use stake_registry::*;
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
pub mod stake_registry {
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
    pub static STAKEREGISTRY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01 `@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@Qb\0@\xA08\x03\x80b\0@\xA0\x839\x81\x01`@\x81\x90Rb\0\x005\x91b\0\x02JV[\x82\x82\x82\x81\x81\x81`\x01`\x01`\xA0\x1B\x03\x16`\xA0\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xDF\\\xF7#`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\0\x93W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\0\xB9\x91\x90b\0\x02\x9EV[`\x01`\x01`\xA0\x1B\x03\x16`\x80\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81`\x01`\x01`\xA0\x1B\x03\x16c\xB14Bq`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x01\x11W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x017\x91\x90b\0\x02\x9EV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xC0R\x81\x16`\xE0Rb\0\x01Tb\0\x01oV[PPPP`\x01`\x01`\xA0\x1B\x03\x16a\x01\0RPb\0\x02\xC5\x91PPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15b\0\x01\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\xFF\x90\x81\x16\x10\x15b\0\x02/W`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x02GW`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15b\0\x02`W`\0\x80\xFD[\x83Qb\0\x02m\x81b\0\x021V[` \x85\x01Q\x90\x93Pb\0\x02\x80\x81b\0\x021V[`@\x85\x01Q\x90\x92Pb\0\x02\x93\x81b\0\x021V[\x80\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15b\0\x02\xB1W`\0\x80\xFD[\x81Qb\0\x02\xBE\x81b\0\x021V[\x93\x92PPPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa=Pb\0\x03P`\09`\0\x81\x81a\x03\x95\x01R\x81\x81a\x08\xDE\x01R\x81\x81a\x17\xFD\x01R\x81\x81a\x1B\xB4\x01Ra\x1C\x94\x01R`\0\x81\x81a\x02\xC4\x01R\x81\x81a\x0B\x9A\x01R\x81\x81a\x0F,\x01R\x81\x81a\x14\xF4\x01R\x81\x81a\x16H\x01Ra\x174\x01R`\0a\x04\xF2\x01R`\0a\x03\x03\x01R`\0\x81\x81a\x05\x9E\x01Ra\x07y\x01Ra=P`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\x1CW`\x005`\xE0\x1C\x80c\x99\xEE\xD4\xEE\x11a\x01%W\x80c\xC8)LV\x11a\0\xADW\x80c\xDF\\\xF7#\x11a\0|W\x80c\xDF\\\xF7#\x14a\x05\x99W\x80c\xE1\x92\xE9\xAD\x14a\x05\xC0W\x80c\xE2T'\xDD\x14a\x05\xE0W\x80c\xE8\x9C\n\0\x14a\x05\xF3W\x80c\xEB\x92\x19\x9C\x14a\x06\x06W`\0\x80\xFD[\x80c\xC8)LV\x14a\x05MW\x80c\xC8\xF79\xD0\x14a\x05`W\x80c\xCD\x05\r\x9C\x14a\x05sW\x80c\xCE\x97~\xC3\x14a\x05\x86W`\0\x80\xFD[\x80c\xA6sFg\x11a\0\xF4W\x80c\xA6sFg\x14a\x04\xE5W\x80c\xB14Bq\x14a\x04\xEDW\x80c\xBB\xA5I\xFA\x14a\x05\x14W\x80c\xBC\x9A@\xC3\x14a\x05'W\x80c\xBD)\xB8\xCD\x14a\x05:W`\0\x80\xFD[\x80c\x99\xEE\xD4\xEE\x14a\x04uW\x80c\x9A\xA1e=\x14a\x04\x88W\x80c\x9E\x8C\xA6 \x14a\x04\xAFW\x80c\xA4<\xDE\x89\x14a\x04\xD2W`\0\x80\xFD[\x80c^Zgu\x11a\x01\xA8W\x80cn\x8F\x03\xCA\x11a\x01wW\x80cn\x8F\x03\xCA\x14a\x03\xB7W\x80c|\x17#G\x14a\x03\xE8W\x80c|\xC0\xD7_\x14a\x04\x02W\x80c~\xD9C\x0F\x14a\x04BW\x80c\x94Dr\xA9\x14a\x04UW`\0\x80\xFD[\x80c^Zgu\x14a\x03MW\x80c_)H\xEC\x14a\x03jW\x80cj\xB58\xD4\x14a\x03}W\x80cm\x14\xA9\x87\x14a\x03\x90W`\0\x80\xFD[\x80c+=\x88\x16\x11a\x01\xEFW\x80c+=\x88\x16\x14a\x02\x99W\x80c,*]+\x14a\x02\xACW\x80c9\x98\xFD\xD3\x14a\x02\xBFW\x80c9\xB7\x0E8\x14a\x02\xFEW\x80cH\x08Xf\x14a\x03%W`\0\x80\xFD[\x80c\x1B2r%\x14a\x02!W\x80c\x1F\x9Bt\xE0\x14a\x02QW\x80c$\x8Des\x14a\x02dW\x80c%PGw\x14a\x02\x84W[`\0\x80\xFD[a\x024a\x02/6`\x04a13V[a\x06'V[`@Q`\x01`\x01``\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x024a\x02_6`\x04a1\x84V[a\x06\x8BV[a\x02wa\x02r6`\x04a1\xBBV[a\x08:V[`@Qa\x02H\x91\x90a1\xEEV[a\x02\x97a\x02\x926`\x04a2kV[a\x08\xD3V[\0[a\x02\x97a\x02\xA76`\x04a3\nV[a\x0B\x98V[a\x02\x97a\x02\xBA6`\x04a3\\V[a\x0F*V[a\x02\xE6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02HV[a\x02\xE6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x038a\x0336`\x04a13V[a\x12`V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02HV[a\x03\\g\r\xE0\xB6\xB3\xA7d\0\0\x81V[`@Q\x90\x81R` \x01a\x02HV[a\x02\x97a\x03x6`\x04a5\xA2V[a\x12uV[a\x024a\x03\x8B6`\x04a6aV[a\x13\x8AV[a\x02\xE6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\\a\x03\xC56`\x04a6aV[`\0\x91\x82Ra\x01\xB3` \x90\x81R`@\x80\x84 `\xFF\x93\x90\x93\x16\x84R\x91\x90R\x90 T\x90V[a\x03\xF0` \x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02HV[a\x04\x15a\x04\x106`\x04a6\x8DV[a\x13\xA5V[`@\x80Q\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x92\x83\x01Q`\x01`\x01``\x1B\x03\x16\x92\x81\x01\x92\x90\x92R\x01a\x02HV[a\x024a\x04P6`\x04a6\xB7V[a\x14\x1FV[a\x04ha\x04c6`\x04a6aV[a\x14SV[`@Qa\x02H\x91\x90a6\xD0V[a\x02\x97a\x04\x836`\x04a7HV[a\x14\xF2V[`\0Ta\x04\x9C\x90b\x01\0\0\x90\x04a\xFF\xFF\x16\x81V[`@Qa\xFF\xFF\x90\x91\x16\x81R` \x01a\x02HV[a\x03\\a\x04\xBD6`\x04a7|V[`\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[a\x024a\x04\xE06`\x04a7\x97V[a\x15\xB0V[a\x03\xF0`\xC0\x81V[a\x02\xE6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\x97a\x05\"6`\x04a7\xD9V[a\x16FV[a\x02\x97a\x0556`\x04a8\x1CV[a\x172V[a\x02\x97a\x05H6`\x04a8FV[a\x17\xF2V[a\x024a\x05[6`\x04a8\x84V[a\x19\x89V[a\x024a\x05n6`\x04a7|V[a\x1A\x18V[a\x02wa\x05\x816`\x04a6aV[a\x1A\x89V[a\x02\x97a\x05\x946`\x04a8\xC0V[a\x1BpV[a\x02\xE6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x05\xD3a\x05\xCE6`\x04a9\x01V[a\x1EPV[`@Qa\x02H\x91\x90a9:V[a\x02wa\x05\xEE6`\x04a6\x8DV[a \xBDV[a\x03\\a\x06\x016`\x04a7|V[a!UV[a\x06\x19a\x06\x146`\x04a6\x8DV[a!vV[`@Qa\x02H\x92\x91\x90a9xV[`\0\x83\x81Ra\x01\xB3` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 a\x06O\x85\x85\x85a!\xBFV[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x06eWa\x06ea9\x9AV[`\0\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90P[\x93\x92PPPV[`\0\x80T\x83\x90b\x01\0\0\x90\x04a\xFF\xFF\x16`\xFF\x82\x16\x10a\x06\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xBC\x90a9\xB0V[`@Q\x80\x91\x03\x90\xFD[`\xFF\x84\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 T\x81Q\x80\x83\x01\x90\x92R\x83\x82R\x91\x81\x01\x83\x90R\x82[\x82\x81\x10\x15a\x08.W`\xFF\x88\x16`\0\x90\x81R`\x01` R`@\x90 \x80T\x82\x90\x81\x10a\x07\x1AWa\x07\x1Aa9\x9AV[`\0\x91\x82R` \x80\x83 `@\x80Q\x80\x82\x01\x82R\x93\x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x80\x82\x16\x80\x86R`\x01`\xA0\x1B\x90\x92\x04`\x01`\x01``\x1B\x03\x16\x93\x85\x01\x93\x90\x93R\x90Qcw\x8EU\xF3`\xE0\x1B\x81R\x8B\x83\x16`\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x91\x94P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cw\x8EU\xF3\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xC0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xE4\x91\x90a:\rV[\x90P\x80\x15a\x08%Wg\r\xE0\xB6\xB3\xA7d\0\0\x83` \x01Q`\x01`\x01``\x1B\x03\x16\x82a\x08\x0E\x91\x90a:<V[a\x08\x18\x91\x90a:[V[a\x08\"\x90\x86a:}V[\x94P[P`\x01\x01a\x06\xEEV[P\x91\x96\x95PPPPPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R\x85\x82Ra\x01\xB3\x81R\x83\x82 `\xFF\x88\x16\x83R\x90R\x91\x90\x91 \x80T\x83\x90\x81\x10a\x08\x80Wa\x08\x80a9\x9AV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x93\x83\x01\x93\x90\x93R`\x01`@\x1B\x90\x92\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x94\x93PPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\t\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xBC\x90a:\xA8V[`\0Tb\x01\0\0\x90\x04a\xFF\xFF\x16\x82\x82a\t5`\x01\x82a;\x1AV[\x81\x81\x10a\tDWa\tDa9\x9AV[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C`\xFF\x16\x10a\t\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`T`$\x82\x01R\x7FStakeRegistry._registerOperator:`D\x82\x01R\x7F greatest quorumNumber must be l`d\x82\x01Rs\x19\\\xDC\xC8\x1D\x1A\x18[\x88\x1C][\xDC\x9D[P\xDB\xDD[\x9D`b\x1B`\x84\x82\x01R`\xA4\x01a\x06\xBCV[`@\x80Q``\x81\x01\x82R`\0` \x82\x01\x81\x90R\x91\x81\x01\x82\x90RCc\xFF\xFF\xFF\xFF\x16\x81R\x90[`\xFF\x81\x16\x83\x11\x15a\x0B\x90W`\0\x84\x84\x83`\xFF\x16\x81\x81\x10a\n*Wa\n*a9\x9AV[\x91\x90\x91\x015`\xF8\x1C\x91P`\0\x90Pa\nC\x88\x88\x84a$\xE6V[\x91PP`\x01`\x01``\x1B\x03\x81\x16a\n\xE8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R\x7FStakeRegistry._registerOperator:`D\x82\x01R\x7F Operator does not meet minimum `d\x82\x01R\x7Fstake requirement for quorum\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06\xBCV[`\0`\xB3\x83`\xFF\x16a\x01\0\x81\x10a\x0B\x01Wa\x0B\x01a9\x9AV[\x01T\x90P\x81\x81\x15a\x0BhW`\xB3\x84`\xFF\x16a\x01\0\x81\x10a\x0B#Wa\x0B#a9\x9AV[\x01a\x0B/`\x01\x84a;\x1AV[\x81T\x81\x10a\x0B?Wa\x0B?a9\x9AV[`\0\x91\x82R` \x90\x91 \x01Ta\x0Be\x90`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x82a:}V[\x90P[`\x01`\x01``\x1B\x03\x81\x16`@\x87\x01Ra\x0B\x81\x84\x87a%\xEBV[\x84`\x01\x01\x94PPPPPa\n\x08V[PPPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xF6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x1A\x91\x90a;1V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0CJW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xBC\x90a;NV[`\0T\x83\x90b\x01\0\0\x90\x04a\xFF\xFF\x16`\xFF\x82\x16\x10a\x0CzW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xBC\x90a9\xB0V[\x81\x80a\r\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`W`$\x82\x01R\x7FVoteWeigherBase.removeStrategies`D\x82\x01R\x7FConsideredAndMultipliers: no ind`d\x82\x01R\x7Fices to remove provided\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06\xBCV[`\0[\x81\x81\x10\x15a\x0B\x90W`\xFF\x86\x16`\0\x81\x81R`\x01` R`@\x90 \x7F1\xFA.,\xD2\x80\xC97^\x13\xFF\xCF=\x81\xE27\x81\0\x18n@X\xF8\xD3\xDD\xB6\x90\xB8-\xCD1\xF7\x90\x87\x87\x85\x81\x81\x10a\reWa\rea9\x9AV[\x90P` \x02\x015\x81T\x81\x10a\r|Wa\r|a9\x9AV[`\0\x91\x82R` \x91\x82\x90 \x01T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x01`@Q\x80\x91\x03\x90\xA2`\xFF\x86\x16`\0\x81\x81R`\x01` R`@\x90 \x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x90\x87\x87\x85\x81\x81\x10a\r\xE8Wa\r\xE8a9\x9AV[\x90P` \x02\x015\x81T\x81\x10a\r\xFFWa\r\xFFa9\x9AV[`\0\x91\x82R` \x80\x83 \x91\x90\x91\x01T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82R\x91\x81\x01\x92\x90\x92R\x01`@Q\x80\x91\x03\x90\xA2`\xFF\x86\x16`\0\x90\x81R`\x01` \x81\x90R`@\x90\x91 \x80T\x90\x91a\x0EQ\x91a;\x1AV[\x81T\x81\x10a\x0EaWa\x0Eaa9\x9AV[`\0\x91\x82R` \x80\x83 `\xFF\x8A\x16\x84R`\x01\x90\x91R`@\x90\x92 \x91\x01\x90\x86\x86\x84\x81\x81\x10a\x0E\x90Wa\x0E\x90a9\x9AV[\x90P` \x02\x015\x81T\x81\x10a\x0E\xA7Wa\x0E\xA7a9\x9AV[`\0\x91\x82R` \x80\x83 \x84T\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92\x83\x17\x81U\x93T`\x01`\x01``\x1B\x03`\x01`\xA0\x1B\x91\x82\x90\x04\x16\x02\x90\x91\x17\x90\x92U`\xFF\x88\x16\x81R`\x01\x90\x91R`@\x90 \x80T\x80a\x0F\nWa\x0F\na;\xCAV[`\0\x82\x81R` \x81 \x82\x01`\0\x19\x90\x81\x01\x91\x90\x91U\x01\x90U`\x01\x01a\r\x17V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x88W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xAC\x91\x90a;1V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0F\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xBC\x90a;NV[`\0T\x85\x90b\x01\0\0\x90\x04a\xFF\xFF\x16`\xFF\x82\x16\x10a\x10\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xBC\x90a9\xB0V[\x83\x80a\x10\x8CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FVoteWeigherBase.modifyStrategyWe`D\x82\x01R\x7Fights: no strategy indices provi`d\x82\x01Rb\x19\x19Y`\xEA\x1B`\x84\x82\x01R`\xA4\x01a\x06\xBCV[\x82\x81\x14a\x11\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R\x7FVoteWeigherBase.modifyStrategyWe`D\x82\x01R\x7Fights: input length mismatch\0\0\0\0`d\x82\x01R`\x84\x01a\x06\xBCV[`\0[\x81\x81\x10\x15a\x12VW\x84\x84\x82\x81\x81\x10a\x11\x1EWa\x11\x1Ea9\x9AV[\x90P` \x02\x01` \x81\x01\x90a\x113\x91\x90a;\xE0V[`\xFF\x89\x16`\0\x90\x81R`\x01` R`@\x90 \x88\x88\x84\x81\x81\x10a\x11WWa\x11Wa9\x9AV[\x90P` \x02\x015\x81T\x81\x10a\x11nWa\x11na9\x9AV[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x80T`\x01`\x01``\x1B\x03\x94\x90\x94\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x93\x90\x93\x17\x90\x92U`\xFF\x8A\x16\x80\x82R`\x01\x90\x92R`@\x90 \x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x90\x89\x89\x85\x81\x81\x10a\x11\xE5Wa\x11\xE5a9\x9AV[\x90P` \x02\x015\x81T\x81\x10a\x11\xFCWa\x11\xFCa9\x9AV[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x87\x87\x85\x81\x81\x10a\x12#Wa\x12#a9\x9AV[\x90P` \x02\x01` \x81\x01\x90a\x128\x91\x90a;\xE0V[`@Qa\x12F\x92\x91\x90a9xV[`@Q\x80\x91\x03\x90\xA2`\x01\x01a\x11\x04V[PPPPPPPPV[`\0a\x12m\x84\x84\x84a!\xBFV[\x94\x93PPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x12\x95WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x12\xAFWP0;\x15\x80\x15a\x12\xAFWP`\0T`\xFF\x16`\x01\x14[a\x13\x12W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x06\xBCV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x135W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x13?\x83\x83a'\x05V[\x80\x15a\x13\x85W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPV[`\0\x80a\x13\x97\x84\x84a\x1A\x89V[`@\x01Q\x91PP[\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\xFF\x83\x16`\0\x90\x81R`\x01` R`@\x90 \x80T\x83\x90\x81\x10a\x13\xDDWa\x13\xDDa9\x9AV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[`3\x81a\x01\0\x81\x10a\x140W`\0\x80\xFD[`\x02\x91\x82\x82\x04\x01\x91\x90\x06`\x0C\x02\x91PT\x90a\x01\0\n\x90\x04`\x01`\x01``\x1B\x03\x16\x81V[`\0\x82\x81Ra\x01\xB3` \x90\x81R`@\x80\x83 `\xFF\x85\x16\x84R\x82R\x80\x83 \x80T\x82Q\x81\x85\x02\x81\x01\x85\x01\x90\x93R\x80\x83R``\x94\x92\x93\x91\x92\x90\x91\x84\x01[\x82\x82\x10\x15a\x14\xE6W`\0\x84\x81R` \x90\x81\x90 `@\x80Q``\x81\x01\x82R\x91\x85\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x83\x85\x01R`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90\x82\x01R\x82R`\x01\x90\x92\x01\x91\x01a\x14\x8DV[PPPP\x90P\x92\x91PPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15PW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15t\x91\x90a;1V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x15\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xBC\x90a;NV[a\x15\xAD\x81a(IV[PV[`\0\x82\x81Ra\x01\xB3` \x90\x81R`@\x80\x83 `\xFF\x88\x16\x84R\x90\x91R\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x15\xE2Wa\x15\xE2a9\x9AV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90Pa\x169\x81\x86a)IV[`@\x01Q\x95\x94PPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xA4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xC8\x91\x90a;1V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x16\xF8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xBC\x90a;NV[`\0T\x82\x90b\x01\0\0\x90\x04a\xFF\xFF\x16`\xFF\x82\x16\x10a\x17(W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xBC\x90a9\xB0V[a\x13\x85\x83\x83a*\xCAV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\x90W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xB4\x91\x90a;1V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x17\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xBC\x90a;NV[a\x17\xEE\x82\x82a.\xFCV[PPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x18:W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xBC\x90a:\xA8V[`@\x80Q``\x80\x82\x01\x83R`\0` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90Rc\xFF\xFF\xFF\xFFC\x16\x80\x85R\x85Q\x93\x84\x01\x86R\x90\x83\x01\x82\x90R\x93\x82\x01\x81\x90R\x92\x81R\x90\x91[`\xFF\x81\x16\x84\x11\x15a\x0B\x90W`\0\x85\x85\x83`\xFF\x16\x81\x81\x10a\x18\x9AWa\x18\x9Aa9\x9AV[\x91\x90\x91\x015`\xF8\x1C\x91P`\0\x90Pa\x18\xB3\x88\x83\x87a/\x91V[\x90P\x80`\xB3\x83`\xFF\x16a\x01\0\x81\x10a\x18\xCDWa\x18\xCDa9\x9AV[\x01`\x01`\xB3\x85`\xFF\x16a\x01\0\x81\x10a\x18\xE7Wa\x18\xE7a9\x9AV[\x01Ta\x18\xF3\x91\x90a;\x1AV[\x81T\x81\x10a\x19\x03Wa\x19\x03a9\x9AV[`\0\x91\x82R` \x90\x91 \x01Ta\x19)\x91\x90`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16a;\xFBV[`\x01`\x01``\x1B\x03\x16`@\x85\x01Ra\x19A\x82\x85a%\xEBV[`@\x80Q`\xFF\x84\x16\x81R`\0` \x82\x01R\x89\x91\x7F\xE7\xC6\x0CRi/\x0E\x0F\xF8\xD1\xC2\x89\xFF\xE6<\xB0.y7?5\xE3\0\x0C\xAF\xE7\xCC\xA8\x98\x88Y4\x91\x01`@Q\x80\x91\x03\x90\xA2PP`\x01\x01a\x18xV[`\0\x80`\xB3\x85`\xFF\x16a\x01\0\x81\x10a\x19\xA3Wa\x19\xA3a9\x9AV[\x01\x83\x81T\x81\x10a\x19\xB5Wa\x19\xB5a9\x9AV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90Pa\x1A\x0C\x81\x85a)IV[`@\x01Q\x94\x93PPPPV[`\0`\xB3\x82`\xFF\x16a\x01\0\x81\x10a\x1A1Wa\x1A1a9\x9AV[\x01`\x01`\xB3\x84`\xFF\x16a\x01\0\x81\x10a\x1AKWa\x1AKa9\x9AV[\x01Ta\x1AW\x91\x90a;\x1AV[\x81T\x81\x10a\x1AgWa\x1Aga9\x9AV[`\0\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x92\x91PPV[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R\x86\x82Ra\x01\xB3\x81R\x84\x82 `\xFF\x87\x16\x83R\x81R\x84\x82 T\x85Q\x93\x84\x01\x86R\x82\x84R\x90\x83\x01\x82\x90R\x93\x82\x01R\x90\x91\x90\x81a\x1A\xE3W\x91Pa\x13\x9F\x90PV[`\0\x85\x81Ra\x01\xB3` \x90\x81R`@\x80\x83 `\xFF\x88\x16\x84R\x90\x91R\x90 a\x1B\x0B`\x01\x84a;\x1AV[\x81T\x81\x10a\x1B\x1BWa\x1B\x1Ba9\x9AV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x92Pa\x13\x9F\x91PPV[`\0[`\0Tb\x01\0\0\x90\x04a\xFF\xFF\x16`\xFF\x82\x16\x10\x15a\x13\x85W`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R\x90[\x83\x81\x10\x15a\x1E/W`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x13T*N\x87\x87\x85\x81\x81\x10a\x1B\xF3Wa\x1B\xF3a9\x9AV[\x90P` \x02\x01` \x81\x01\x90a\x1C\x08\x91\x90a<#V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1CLW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Cp\x91\x90a:\rV[`@Qc41\xAF%`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c41\xAF%\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\xDBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xFF\x91\x90a<@V[\x90P`\x01`\x01`\x01`\xC0\x1B\x03\x82\x16`\xFF\x87\x16\x1C\x81\x16\x14\x15a\x1E%W\x83Qc\xFF\xFF\xFF\xFF\x16a\x1D\xC1W`\xB3\x85`\xFF\x16a\x01\0\x81\x10a\x1D=Wa\x1D=a9\x9AV[\x01`\x01`\xB3\x87`\xFF\x16a\x01\0\x81\x10a\x1DWWa\x1DWa9\x9AV[\x01Ta\x1Dc\x91\x90a;\x1AV[\x81T\x81\x10a\x1DsWa\x1Dsa9\x9AV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x93P[`\0\x80a\x1D\xF5\x89\x89\x87\x81\x81\x10a\x1D\xD9Wa\x1D\xD9a9\x9AV[\x90P` \x02\x01` \x81\x01\x90a\x1D\xEE\x91\x90a<#V[\x85\x89a$\xE6V[\x91P\x91P\x80\x82\x87`@\x01Qa\x1E\n\x91\x90a;\xFBV[a\x1E\x14\x91\x90a:}V[`\x01`\x01``\x1B\x03\x16`@\x87\x01RPP[PP`\x01\x01a\x1B\xA8V[P\x80Qc\xFF\xFF\xFF\xFF\x16\x15a\x1EGWa\x1EG\x82\x82a%\xEBV[P`\x01\x01a\x1BsV[```\0\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1ElWa\x1Ela3\xDCV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1E\x95W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83\x81\x10\x15a \xB4W`\0\x85\x85\x83\x81\x81\x10a\x1E\xB7Wa\x1E\xB7a9\x9AV[\x91\x90\x91\x015`\xF8\x1C\x91PPc\xFF\xFF\xFF\xFF\x87\x16`\xB3\x82a\x01\0\x81\x10a\x1E\xDDWa\x1E\xDDa9\x9AV[\x01`\0\x81T\x81\x10a\x1E\xF0Wa\x1E\xF0a9\x9AV[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a\x1F\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`j`$\x82\x01R\x7FStakeRegistry.getTotalStakeIndic`D\x82\x01R\x7FesByQuorumNumbersAtBlockNumber: `d\x82\x01R\x7Fquorum has no stake history at b`\x84\x82\x01Ri67\xB1\xB5\xA7:\xB6\xB12\xB9`\xB1\x1B`\xA4\x82\x01R`\xC4\x01a\x06\xBCV[`\0`\xB3\x82`\xFF\x16a\x01\0\x81\x10a\x1F\xC8Wa\x1F\xC8a9\x9AV[\x01T\x90P`\0[\x81c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a \x9EW\x88c\xFF\xFF\xFF\xFF\x16`\xB3\x84`\xFF\x16a\x01\0\x81\x10a \x01Wa \x01a9\x9AV[\x01`\x01a \x0E\x84\x86a<iV[a \x18\x91\x90a<iV[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a .Wa .a9\x9AV[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a \x8CW`\x01a Q\x82\x84a<iV[a [\x91\x90a<iV[\x85\x85\x81Q\x81\x10a mWa ma9\x9AV[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPPa \x9EV[\x80a \x96\x81a<\x86V[\x91PPa\x1F\xCFV[PPP\x80\x80a \xAC\x90a<\xAAV[\x91PPa\x1E\x9BV[P\x94\x93PPPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R`\xB3\x83`\xFF\x16a\x01\0\x81\x10a \xF1Wa \xF1a9\x9AV[\x01\x82\x81T\x81\x10a!\x03Wa!\x03a9\x9AV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x93\x83\x01\x93\x90\x93R`\x01`@\x1B\x90\x92\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[`\0`\xB3\x82`\xFF\x16a\x01\0\x81\x10a!nWa!na9\x9AV[\x01T\x92\x91PPV[`\x01` R\x81`\0R`@`\0 \x81\x81T\x81\x10a!\x92W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x92P`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90P\x82V[`\0\x83\x81Ra\x01\xB3` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 T\x81[\x81c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a$\x16W`\0\x86\x81Ra\x01\xB3` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x90\x91R\x90 c\xFF\xFF\xFF\xFF\x85\x16\x90`\x01a\"$\x84\x86a<iV[a\".\x91\x90a<iV[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\"DWa\"Da9\x9AV[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a$\x04W`\0\x86\x81Ra\x01\xB3` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x90\x91R\x90 `\x01a\"\x84\x83\x85a<iV[a\"\x8E\x91\x90a<iV[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\"\xA4Wa\"\xA4a9\x9AV[`\0\x91\x82R` \x90\x91 \x01T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15\x80a#0WP`\0\x86\x81Ra\x01\xB3` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x90\x91R\x90 c\xFF\xFF\xFF\xFF\x85\x16\x90`\x01a\"\xF5\x84\x86a<iV[a\"\xFF\x91\x90a<iV[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a#\x15Wa#\x15a9\x9AV[`\0\x91\x82R` \x90\x91 \x01T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x11[a#\xE5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`t`$\x82\x01R\x7FStakeRegistry._getStakeUpdateInd`D\x82\x01R\x7FexForOperatorIdForQuorumAtBlockN`d\x82\x01R\x7Fumber: operatorId has no stake u`\x84\x82\x01Rs820\xBA2\x900\xBA\x10167\xB1\xB5\xA7:\xB6\xB12\xB9`a\x1B`\xA4\x82\x01R`\xC4\x01a\x06\xBCV[`\x01a#\xF1\x82\x84a<iV[a#\xFB\x91\x90a<iV[\x92PPPa\x06\x84V[\x80a$\x0E\x81a<\x86V[\x91PPa!\xDFV[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x8C`$\x82\x01R\x7FStakeRegistry._getStakeUpdateInd`D\x82\x01R\x7FexForOperatorIdForQuorumAtBlockN`d\x82\x01R\x7Fumber: no stake update found for`\x84\x82\x01R\x7F operatorId and quorumNumber at `\xA4\x82\x01Rk167\xB1\xB5\x907:\xB6\xB12\xB9`\xA1\x1B`\xC4\x82\x01R`\xE4\x01a\x06\xBCV[`@\x80Q``\x81\x01\x82R`\0` \x82\x01\x81\x90R\x91\x81\x01\x82\x90RCc\xFF\xFF\xFF\xFF\x16\x81R\x81\x90a%\x14\x84\x87a\x06\x8BV[`\x01`\x01``\x1B\x03\x16`@\x82\x01R`3`\xFF\x85\x16a\x01\0\x81\x10a%9Wa%9a9\x9AV[`\x02\x91\x82\x82\x04\x01\x91\x90\x06`\x0C\x02\x90T\x90a\x01\0\n\x90\x04`\x01`\x01``\x1B\x03\x16`\x01`\x01``\x1B\x03\x16\x81`@\x01Q`\x01`\x01``\x1B\x03\x16\x10\x15a%}W`\0`@\x82\x01R[`\0a%\x8A\x86\x86\x84a/\x91V[`@\x80\x84\x01Q\x81Q`\xFF\x89\x16\x81R`\x01`\x01``\x1B\x03\x90\x91\x16` \x82\x01R\x91\x92P\x87\x91\x7F\xE7\xC6\x0CRi/\x0E\x0F\xF8\xD1\xC2\x89\xFF\xE6<\xB0.y7?5\xE3\0\x0C\xAF\xE7\xCC\xA8\x98\x88Y4\x91\x01`@Q\x80\x91\x03\x90\xA2`@\x90\x91\x01Q\x90\x92P\x90P\x93P\x93\x91PPV[`\0`\xB3\x83`\xFF\x16a\x01\0\x81\x10a&\x04Wa&\x04a9\x9AV[\x01T\x90P\x80\x15a&nWC`\xB3\x84`\xFF\x16a\x01\0\x81\x10a&&Wa&&a9\x9AV[\x01a&2`\x01\x84a;\x1AV[\x81T\x81\x10a&BWa&Ba9\x9AV[\x90`\0R` `\0 \x01`\0\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP[c\xFF\xFF\xFF\xFFC\x16\x82R`\xB3`\xFF\x84\x16a\x01\0\x81\x10a&\x8EWa&\x8Ea9\x9AV[\x01\x80T`\x01\x81\x01\x82U`\0\x91\x82R` \x91\x82\x90 \x84Q\x91\x01\x80T\x92\x85\x01Q`@\x90\x95\x01Q`\x01`\x01``\x1B\x03\x16`\x01`@\x1B\x02k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x1B\x19c\xFF\xFF\xFF\xFF\x96\x87\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x95\x16\x96\x90\x93\x16\x95\x90\x95\x17\x92\x90\x92\x17\x16\x92\x90\x92\x17\x90\x91UPPV[`\0Ta\x01\0\x90\x04`\xFF\x16a'pW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x06\xBCV[\x80Q\x82Q\x14a'\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`;`$\x82\x01R\x7FRegistry._initialize: minimumSta`D\x82\x01R\x7FkeForQuorum length mismatch\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\xBCV[`\0[\x81Q\x81`\xFF\x16\x10\x15a\x13\x85Wa(\x1C\x81\x84\x83`\xFF\x16\x81Q\x81\x10a(\x0FWa(\x0Fa9\x9AV[` \x02` \x01\x01Qa.\xFCV[a(A\x82\x82`\xFF\x16\x81Q\x81\x10a(4Wa(4a9\x9AV[` \x02` \x01\x01Qa(IV[`\x01\x01a'\xEAV[`\0Tb\x01\0\0\x90\x04a\xFF\xFF\x16`\xC0\x81\x10a(\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`O`$\x82\x01R\x7FVoteWeigherBase._createQuorum: n`D\x82\x01R\x7Fumber of quorums cannot exceed M`d\x82\x01Rn\x10V\x17\xD4US\xD4\x95SW\xD0\xD3\xD5S\x95`\x8A\x1B`\x84\x82\x01R`\xA4\x01a\x06\xBCV[\x80a(\xF0\x81`\x01a<\xC5V[`\0`\x02a\x01\0\n\x81T\x81a\xFF\xFF\x02\x19\x16\x90\x83a\xFF\xFF\x16\x02\x17\x90UPa)\x16\x81\x84a*\xCAV[`@Q`\xFF\x82\x16\x90\x7F\x83\x1A\x9C\x86\xC4[\xB3\x03\xCA\xF3\xF0d\xBE+\xC2\xB9\xFDN\xCF\x19\xE4|J\xC0*a\xE7]\xAB\xFEU\xB4\x90`\0\x90\xA2PPPV[\x81Qc\xFF\xFF\xFF\xFF\x80\x83\x16\x91\x16\x11\x15a)\xEFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R```$\x82\x01R\x7FStakeRegistry._validateOperatorS`D\x82\x01R\x7FtakeAtBlockNumber: operatorStake`d\x82\x01R\x7FUpdate is from after blockNumber`\x84\x82\x01R`\xA4\x01a\x06\xBCV[` \x82\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x80a*\x15WP\x80c\xFF\xFF\xFF\xFF\x16\x82` \x01Qc\xFF\xFF\xFF\xFF\x16\x11[a\x17\xEEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`t`$\x82\x01R\x7FStakeRegistry._validateOperatorS`D\x82\x01R\x7FtakeAtBlockNumber: there is a ne`d\x82\x01R\x7Fwer operatorStakeUpdate availabl`\x84\x82\x01Rs2\x9012\xB37\xB92\x90167\xB1\xB5\xA7:\xB6\xB12\xB9`a\x1B`\xA4\x82\x01R`\xC4\x01a\x06\xBCV[`\0\x81Q\x11a+FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`N`$\x82\x01R`\0\x80Q` a<\xFB\x839\x81Q\x91R`D\x82\x01R\x7FnsideredAndMultipliers: no strat`d\x82\x01Rm\x19Y\xDAY\\\xC8\x1C\x1C\x9B\xDD\x9AY\x19Y`\x92\x1B`\x84\x82\x01R`\xA4\x01a\x06\xBCV[\x80Q`\xFF\x83\x16`\0\x90\x81R`\x01` \x90\x81R`@\x90\x91 T\x90a+i\x83\x83a<\xE2V[\x11\x15a+\xF1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`[`$\x82\x01R`\0\x80Q` a<\xFB\x839\x81Q\x91R`D\x82\x01R\x7FnsideredAndMultipliers: exceed M`d\x82\x01R\x7FAX_WEIGHING_FUNCTION_LENGTH\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06\xBCV[`\0[\x82\x81\x10\x15a.\xF5W`\0[a,\t\x82\x84a<\xE2V[\x81\x10\x15a,\xFCW\x84\x82\x81Q\x81\x10a,\"Wa,\"a9\x9AV[` \x02` \x01\x01Q`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\0\x88`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x82\x81T\x81\x10a,aWa,aa9\x9AV[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a,\xF4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`S`$\x82\x01R`\0\x80Q` a<\xFB\x839\x81Q\x91R`D\x82\x01R\x7FnsideredAndMultipliers: cannot a`d\x82\x01Rr\x0C\x8C\x84\x0El-\xAC\xA4\x0En\x8EL.\x8C\xAC\xEF$\x06O`k\x1B`\x84\x82\x01R`\xA4\x01a\x06\xBCV[`\x01\x01a+\xFFV[P`\0\x84\x82\x81Q\x81\x10a-\x11Wa-\x11a9\x9AV[` \x02` \x01\x01Q` \x01Q`\x01`\x01``\x1B\x03\x16\x11a-\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R`\0\x80Q` a<\xFB\x839\x81Q\x91R`D\x82\x01R\x7FnsideredAndMultipliers: cannot a`d\x82\x01R\x7Fdd strategy with zero weight\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06\xBCV[`\xFF\x85\x16`\0\x90\x81R`\x01` R`@\x90 \x84Q\x85\x90\x83\x90\x81\x10a-\xD3Wa-\xD3a9\x9AV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82T`\x01\x81\x01\x84U`\0\x93\x84R\x92\x82\x90 \x81Q\x91\x90\x92\x01Q`\x01`\x01``\x1B\x03\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x91\x01U\x83Q`\xFF\x86\x16\x90\x7F\x10V^V\xCA\xCB\xF3.\xCA&yE\xF0T\xFE\xC0.Yu\x002\xD1\x13\xD30!\x82\xAD\x96\x7FT\x04\x90\x86\x90\x84\x90\x81\x10a.PWa.Pa9\x9AV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQ`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x01`@Q\x80\x91\x03\x90\xA2\x84`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x85\x83\x81Q\x81\x10a.\xADWa.\xADa9\x9AV[` \x02` \x01\x01Q`\0\x01Q\x86\x84\x81Q\x81\x10a.\xCBWa.\xCBa9\x9AV[` \x02` \x01\x01Q` \x01Q`@Qa.\xE5\x92\x91\x90a9xV[`@Q\x80\x91\x03\x90\xA2`\x01\x01a+\xF4V[PPPPPV[\x80`3\x83`\xFF\x16a\x01\0\x81\x10a/\x14Wa/\x14a9\x9AV[`\x02\x91\x82\x82\x04\x01\x91\x90\x06`\x0C\x02a\x01\0\n\x81T\x81`\x01`\x01``\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01``\x1B\x03\x16\x02\x17\x90UP\x81`\xFF\x16\x7F&\xEE\xCF\xF2\xB7\x0B\nq\x10O\xF4\xD9@\xBAqb\xD2:\x95\xC2Hw\x1F\xC4\x87\xA7\xBE\x17\xA5\x96\xB3\xCF\x82`@Qa/\x85\x91\x90`\x01`\x01``\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PPV[`\0\x83\x81Ra\x01\xB3` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 T\x81\x90\x80\x15a0tW`\0\x86\x81Ra\x01\xB3` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x90\x91R\x90 C\x90a/\xE1`\x01\x84a;\x1AV[\x81T\x81\x10a/\xF1Wa/\xF1a9\x9AV[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x80Tc\xFF\xFF\xFF\xFF\x94\x90\x94\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x94\x16\x93\x90\x93\x17\x90\x92U\x87\x81Ra\x01\xB3\x82R`@\x80\x82 `\xFF\x89\x16\x83R\x90\x92R a0E`\x01\x83a;\x1AV[\x81T\x81\x10a0UWa0Ua9\x9AV[`\0\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x91P[P`\0\x85\x81Ra\x01\xB3` \x90\x81R`@\x80\x83 `\xFF\x88\x16\x84R\x82R\x80\x83 \x80T`\x01\x81\x01\x82U\x90\x84R\x92\x82\x90 \x86Q\x93\x01\x80T\x92\x87\x01Q\x91\x87\x01Q`\x01`\x01``\x1B\x03\x16`\x01`@\x1B\x02k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x1B\x19c\xFF\xFF\xFF\xFF\x93\x84\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x95\x16\x93\x90\x95\x16\x92\x90\x92\x17\x92\x90\x92\x17\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x90P\x93\x92PPPV[\x805`\xFF\x81\x16\x81\x14a1\x1AW`\0\x80\xFD[\x91\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a1\x1AW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a1HW`\0\x80\xFD[\x835\x92Pa1X` \x85\x01a1\tV[\x91Pa1f`@\x85\x01a1\x1FV[\x90P\x92P\x92P\x92V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x15\xADW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a1\x97W`\0\x80\xFD[a1\xA0\x83a1\tV[\x91P` \x83\x015a1\xB0\x81a1oV[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a1\xD0W`\0\x80\xFD[a1\xD9\x84a1\tV[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[\x81Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x80\x84\x01Q\x90\x91\x16\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01``\x1B\x03\x16\x90\x82\x01R``\x81\x01a\x13\x9FV[`\0\x80\x83`\x1F\x84\x01\x12a25W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a2LW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a2dW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a2\x81W`\0\x80\xFD[\x845a2\x8C\x81a1oV[\x93P` \x85\x015\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a2\xAEW`\0\x80\xFD[a2\xBA\x87\x82\x88\x01a2#V[\x95\x98\x94\x97P\x95PPPPV[`\0\x80\x83`\x1F\x84\x01\x12a2\xD8W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a2\xEFW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a2dW`\0\x80\xFD[`\0\x80`\0`@\x84\x86\x03\x12\x15a3\x1FW`\0\x80\xFD[a3(\x84a1\tV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a3CW`\0\x80\xFD[a3O\x86\x82\x87\x01a2\xC6V[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a3tW`\0\x80\xFD[a3}\x86a1\tV[\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a3\x99W`\0\x80\xFD[a3\xA5\x89\x83\x8A\x01a2\xC6V[\x90\x96P\x94P`@\x88\x015\x91P\x80\x82\x11\x15a3\xBEW`\0\x80\xFD[Pa3\xCB\x88\x82\x89\x01a2\xC6V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a4\x14Wa4\x14a3\xDCV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a4BWa4Ba3\xDCV[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a4cWa4ca3\xDCV[P`\x05\x1B` \x01\x90V[\x805`\x01`\x01``\x1B\x03\x81\x16\x81\x14a1\x1AW`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a4\x95W`\0\x80\xFD[\x815` a4\xAAa4\xA5\x83a4JV[a4\x1AV[\x82\x81R`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a4\xC9W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a5\x18W`@\x81\x89\x03\x12\x15a4\xE6W`\0\x80\x81\xFD[a4\xEEa3\xF2V[\x815a4\xF9\x81a1oV[\x81Ra5\x06\x82\x86\x01a4mV[\x81\x86\x01R\x83R\x91\x83\x01\x91`@\x01a4\xCDV[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a54W`\0\x80\xFD[\x815` a5Da4\xA5\x83a4JV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a5cW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a5\x18W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a5\x86W`\0\x80\x81\xFD[a5\x94\x89\x86\x83\x8B\x01\x01a4\x84V[\x84RP\x91\x83\x01\x91\x83\x01a5gV[`\0\x80`@\x83\x85\x03\x12\x15a5\xB5W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a5\xCCW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a5\xE0W`\0\x80\xFD[\x815` a5\xF0a4\xA5\x83a4JV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a6\x0FW`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a64Wa6%\x86a4mV[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a6\x14V[\x96PP\x86\x015\x92PP\x80\x82\x11\x15a6JW`\0\x80\xFD[Pa6W\x85\x82\x86\x01a5#V[\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a6tW`\0\x80\xFD[\x825\x91Pa6\x84` \x84\x01a1\tV[\x90P\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a6\xA0W`\0\x80\xFD[a6\xA9\x83a1\tV[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a6\xC9W`\0\x80\xFD[P5\x91\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a7<Wa7)\x83\x85Qc\xFF\xFF\xFF\xFF\x80\x82Q\x16\x83R\x80` \x83\x01Q\x16` \x84\x01RP`\x01`\x01``\x1B\x03`@\x82\x01Q\x16`@\x83\x01RPPV[\x92\x84\x01\x92``\x92\x90\x92\x01\x91`\x01\x01a6\xECV[P\x90\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a7ZW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a7pW`\0\x80\xFD[a\x12m\x84\x82\x85\x01a4\x84V[`\0` \x82\x84\x03\x12\x15a7\x8EW`\0\x80\xFD[a\x06\x84\x82a1\tV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a7\xADW`\0\x80\xFD[a7\xB6\x85a1\tV[\x93Pa7\xC4` \x86\x01a1\x1FV[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[`\0\x80`@\x83\x85\x03\x12\x15a7\xECW`\0\x80\xFD[a7\xF5\x83a1\tV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a8\x10W`\0\x80\xFD[a6W\x85\x82\x86\x01a4\x84V[`\0\x80`@\x83\x85\x03\x12\x15a8/W`\0\x80\xFD[a88\x83a1\tV[\x91Pa6\x84` \x84\x01a4mV[`\0\x80`\0`@\x84\x86\x03\x12\x15a8[W`\0\x80\xFD[\x835\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a8xW`\0\x80\xFD[a3O\x86\x82\x87\x01a2#V[`\0\x80`\0``\x84\x86\x03\x12\x15a8\x99W`\0\x80\xFD[a8\xA2\x84a1\tV[\x92Pa8\xB0` \x85\x01a1\x1FV[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0\x80` \x83\x85\x03\x12\x15a8\xD3W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a8\xE9W`\0\x80\xFD[a8\xF5\x85\x82\x86\x01a2\xC6V[\x90\x96\x90\x95P\x93PPPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a9\x16W`\0\x80\xFD[a9\x1F\x84a1\x1FV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a8xW`\0\x80\xFD[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a7<W\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a9VV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R`\x01`\x01``\x1B\x03\x16` \x82\x01R`@\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[` \x80\x82R`<\x90\x82\x01R\x7FVoteWeigherBase.validQuorumNumbe`@\x82\x01R\x7Fr: quorumNumber is not valid\0\0\0\0``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15a:\x1FW`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a:VWa:Va:&V[P\x02\x90V[`\0\x82a:xWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a:\x9FWa:\x9Fa:&V[\x01\x94\x93PPPPV[` \x80\x82R`L\x90\x82\x01R\x7FStakeRegistry.onlyRegistryCoordi`@\x82\x01R\x7Fnator: caller is not the Registr``\x82\x01Rk<\xA1\xB7\xB7\xB924\xB70\xBA7\xB9`\xA1\x1B`\x80\x82\x01R`\xA0\x01\x90V[`\0\x82\x82\x10\x15a;,Wa;,a:&V[P\x03\x90V[`\0` \x82\x84\x03\x12\x15a;CW`\0\x80\xFD[\x81Qa\x06\x84\x81a1oV[` \x80\x82R`V\x90\x82\x01R\x7FVoteWeigherBase.onlyServiceManag`@\x82\x01R\x7FerOwner: caller is not the owner``\x82\x01Ru\x107\xB3\x10:42\x909\xB2\xB9;4\xB1\xB2\xA6\xB0\xB70\xB3\xB2\xB9`Q\x1B`\x80\x82\x01R`\xA0\x01\x90V[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a;\xF2W`\0\x80\xFD[a\x06\x84\x82a4mV[`\0`\x01`\x01``\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a<\x1BWa<\x1Ba:&V[\x03\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a<5W`\0\x80\xFD[\x815a\x06\x84\x81a1oV[`\0` \x82\x84\x03\x12\x15a<RW`\0\x80\xFD[\x81Q`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x14a\x06\x84W`\0\x80\xFD[`\0c\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a<\x1BWa<\x1Ba:&V[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x14\x15a<\xA0Wa<\xA0a:&V[`\x01\x01\x93\x92PPPV[`\0`\0\x19\x82\x14\x15a<\xBEWa<\xBEa:&V[P`\x01\x01\x90V[`\0a\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a:\x9FWa:\x9Fa:&V[`\0\x82\x19\x82\x11\x15a<\xF5Wa<\xF5a:&V[P\x01\x90V\xFEVoteWeigherBase._addStrategiesCo\xA2dipfsX\"\x12 \xF5\xED\xCB\xD7\xB8\x06\xA3;\x02>u\xFAT\xE6<\xDC=\xEC\xC5\x99\x11\xB0f\x063\xBE\x9C\x91a\xD9\xB8?dsolcC\0\x08\x0C\x003";
    /// The bytecode of the contract.
    pub static STAKEREGISTRY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\x1CW`\x005`\xE0\x1C\x80c\x99\xEE\xD4\xEE\x11a\x01%W\x80c\xC8)LV\x11a\0\xADW\x80c\xDF\\\xF7#\x11a\0|W\x80c\xDF\\\xF7#\x14a\x05\x99W\x80c\xE1\x92\xE9\xAD\x14a\x05\xC0W\x80c\xE2T'\xDD\x14a\x05\xE0W\x80c\xE8\x9C\n\0\x14a\x05\xF3W\x80c\xEB\x92\x19\x9C\x14a\x06\x06W`\0\x80\xFD[\x80c\xC8)LV\x14a\x05MW\x80c\xC8\xF79\xD0\x14a\x05`W\x80c\xCD\x05\r\x9C\x14a\x05sW\x80c\xCE\x97~\xC3\x14a\x05\x86W`\0\x80\xFD[\x80c\xA6sFg\x11a\0\xF4W\x80c\xA6sFg\x14a\x04\xE5W\x80c\xB14Bq\x14a\x04\xEDW\x80c\xBB\xA5I\xFA\x14a\x05\x14W\x80c\xBC\x9A@\xC3\x14a\x05'W\x80c\xBD)\xB8\xCD\x14a\x05:W`\0\x80\xFD[\x80c\x99\xEE\xD4\xEE\x14a\x04uW\x80c\x9A\xA1e=\x14a\x04\x88W\x80c\x9E\x8C\xA6 \x14a\x04\xAFW\x80c\xA4<\xDE\x89\x14a\x04\xD2W`\0\x80\xFD[\x80c^Zgu\x11a\x01\xA8W\x80cn\x8F\x03\xCA\x11a\x01wW\x80cn\x8F\x03\xCA\x14a\x03\xB7W\x80c|\x17#G\x14a\x03\xE8W\x80c|\xC0\xD7_\x14a\x04\x02W\x80c~\xD9C\x0F\x14a\x04BW\x80c\x94Dr\xA9\x14a\x04UW`\0\x80\xFD[\x80c^Zgu\x14a\x03MW\x80c_)H\xEC\x14a\x03jW\x80cj\xB58\xD4\x14a\x03}W\x80cm\x14\xA9\x87\x14a\x03\x90W`\0\x80\xFD[\x80c+=\x88\x16\x11a\x01\xEFW\x80c+=\x88\x16\x14a\x02\x99W\x80c,*]+\x14a\x02\xACW\x80c9\x98\xFD\xD3\x14a\x02\xBFW\x80c9\xB7\x0E8\x14a\x02\xFEW\x80cH\x08Xf\x14a\x03%W`\0\x80\xFD[\x80c\x1B2r%\x14a\x02!W\x80c\x1F\x9Bt\xE0\x14a\x02QW\x80c$\x8Des\x14a\x02dW\x80c%PGw\x14a\x02\x84W[`\0\x80\xFD[a\x024a\x02/6`\x04a13V[a\x06'V[`@Q`\x01`\x01``\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x024a\x02_6`\x04a1\x84V[a\x06\x8BV[a\x02wa\x02r6`\x04a1\xBBV[a\x08:V[`@Qa\x02H\x91\x90a1\xEEV[a\x02\x97a\x02\x926`\x04a2kV[a\x08\xD3V[\0[a\x02\x97a\x02\xA76`\x04a3\nV[a\x0B\x98V[a\x02\x97a\x02\xBA6`\x04a3\\V[a\x0F*V[a\x02\xE6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02HV[a\x02\xE6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x038a\x0336`\x04a13V[a\x12`V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02HV[a\x03\\g\r\xE0\xB6\xB3\xA7d\0\0\x81V[`@Q\x90\x81R` \x01a\x02HV[a\x02\x97a\x03x6`\x04a5\xA2V[a\x12uV[a\x024a\x03\x8B6`\x04a6aV[a\x13\x8AV[a\x02\xE6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\\a\x03\xC56`\x04a6aV[`\0\x91\x82Ra\x01\xB3` \x90\x81R`@\x80\x84 `\xFF\x93\x90\x93\x16\x84R\x91\x90R\x90 T\x90V[a\x03\xF0` \x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02HV[a\x04\x15a\x04\x106`\x04a6\x8DV[a\x13\xA5V[`@\x80Q\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x92\x83\x01Q`\x01`\x01``\x1B\x03\x16\x92\x81\x01\x92\x90\x92R\x01a\x02HV[a\x024a\x04P6`\x04a6\xB7V[a\x14\x1FV[a\x04ha\x04c6`\x04a6aV[a\x14SV[`@Qa\x02H\x91\x90a6\xD0V[a\x02\x97a\x04\x836`\x04a7HV[a\x14\xF2V[`\0Ta\x04\x9C\x90b\x01\0\0\x90\x04a\xFF\xFF\x16\x81V[`@Qa\xFF\xFF\x90\x91\x16\x81R` \x01a\x02HV[a\x03\\a\x04\xBD6`\x04a7|V[`\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[a\x024a\x04\xE06`\x04a7\x97V[a\x15\xB0V[a\x03\xF0`\xC0\x81V[a\x02\xE6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\x97a\x05\"6`\x04a7\xD9V[a\x16FV[a\x02\x97a\x0556`\x04a8\x1CV[a\x172V[a\x02\x97a\x05H6`\x04a8FV[a\x17\xF2V[a\x024a\x05[6`\x04a8\x84V[a\x19\x89V[a\x024a\x05n6`\x04a7|V[a\x1A\x18V[a\x02wa\x05\x816`\x04a6aV[a\x1A\x89V[a\x02\x97a\x05\x946`\x04a8\xC0V[a\x1BpV[a\x02\xE6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x05\xD3a\x05\xCE6`\x04a9\x01V[a\x1EPV[`@Qa\x02H\x91\x90a9:V[a\x02wa\x05\xEE6`\x04a6\x8DV[a \xBDV[a\x03\\a\x06\x016`\x04a7|V[a!UV[a\x06\x19a\x06\x146`\x04a6\x8DV[a!vV[`@Qa\x02H\x92\x91\x90a9xV[`\0\x83\x81Ra\x01\xB3` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 a\x06O\x85\x85\x85a!\xBFV[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x06eWa\x06ea9\x9AV[`\0\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90P[\x93\x92PPPV[`\0\x80T\x83\x90b\x01\0\0\x90\x04a\xFF\xFF\x16`\xFF\x82\x16\x10a\x06\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xBC\x90a9\xB0V[`@Q\x80\x91\x03\x90\xFD[`\xFF\x84\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 T\x81Q\x80\x83\x01\x90\x92R\x83\x82R\x91\x81\x01\x83\x90R\x82[\x82\x81\x10\x15a\x08.W`\xFF\x88\x16`\0\x90\x81R`\x01` R`@\x90 \x80T\x82\x90\x81\x10a\x07\x1AWa\x07\x1Aa9\x9AV[`\0\x91\x82R` \x80\x83 `@\x80Q\x80\x82\x01\x82R\x93\x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x80\x82\x16\x80\x86R`\x01`\xA0\x1B\x90\x92\x04`\x01`\x01``\x1B\x03\x16\x93\x85\x01\x93\x90\x93R\x90Qcw\x8EU\xF3`\xE0\x1B\x81R\x8B\x83\x16`\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x91\x94P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cw\x8EU\xF3\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xC0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xE4\x91\x90a:\rV[\x90P\x80\x15a\x08%Wg\r\xE0\xB6\xB3\xA7d\0\0\x83` \x01Q`\x01`\x01``\x1B\x03\x16\x82a\x08\x0E\x91\x90a:<V[a\x08\x18\x91\x90a:[V[a\x08\"\x90\x86a:}V[\x94P[P`\x01\x01a\x06\xEEV[P\x91\x96\x95PPPPPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R\x85\x82Ra\x01\xB3\x81R\x83\x82 `\xFF\x88\x16\x83R\x90R\x91\x90\x91 \x80T\x83\x90\x81\x10a\x08\x80Wa\x08\x80a9\x9AV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x93\x83\x01\x93\x90\x93R`\x01`@\x1B\x90\x92\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x94\x93PPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\t\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xBC\x90a:\xA8V[`\0Tb\x01\0\0\x90\x04a\xFF\xFF\x16\x82\x82a\t5`\x01\x82a;\x1AV[\x81\x81\x10a\tDWa\tDa9\x9AV[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C`\xFF\x16\x10a\t\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`T`$\x82\x01R\x7FStakeRegistry._registerOperator:`D\x82\x01R\x7F greatest quorumNumber must be l`d\x82\x01Rs\x19\\\xDC\xC8\x1D\x1A\x18[\x88\x1C][\xDC\x9D[P\xDB\xDD[\x9D`b\x1B`\x84\x82\x01R`\xA4\x01a\x06\xBCV[`@\x80Q``\x81\x01\x82R`\0` \x82\x01\x81\x90R\x91\x81\x01\x82\x90RCc\xFF\xFF\xFF\xFF\x16\x81R\x90[`\xFF\x81\x16\x83\x11\x15a\x0B\x90W`\0\x84\x84\x83`\xFF\x16\x81\x81\x10a\n*Wa\n*a9\x9AV[\x91\x90\x91\x015`\xF8\x1C\x91P`\0\x90Pa\nC\x88\x88\x84a$\xE6V[\x91PP`\x01`\x01``\x1B\x03\x81\x16a\n\xE8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R\x7FStakeRegistry._registerOperator:`D\x82\x01R\x7F Operator does not meet minimum `d\x82\x01R\x7Fstake requirement for quorum\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06\xBCV[`\0`\xB3\x83`\xFF\x16a\x01\0\x81\x10a\x0B\x01Wa\x0B\x01a9\x9AV[\x01T\x90P\x81\x81\x15a\x0BhW`\xB3\x84`\xFF\x16a\x01\0\x81\x10a\x0B#Wa\x0B#a9\x9AV[\x01a\x0B/`\x01\x84a;\x1AV[\x81T\x81\x10a\x0B?Wa\x0B?a9\x9AV[`\0\x91\x82R` \x90\x91 \x01Ta\x0Be\x90`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x82a:}V[\x90P[`\x01`\x01``\x1B\x03\x81\x16`@\x87\x01Ra\x0B\x81\x84\x87a%\xEBV[\x84`\x01\x01\x94PPPPPa\n\x08V[PPPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xF6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x1A\x91\x90a;1V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0CJW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xBC\x90a;NV[`\0T\x83\x90b\x01\0\0\x90\x04a\xFF\xFF\x16`\xFF\x82\x16\x10a\x0CzW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xBC\x90a9\xB0V[\x81\x80a\r\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`W`$\x82\x01R\x7FVoteWeigherBase.removeStrategies`D\x82\x01R\x7FConsideredAndMultipliers: no ind`d\x82\x01R\x7Fices to remove provided\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06\xBCV[`\0[\x81\x81\x10\x15a\x0B\x90W`\xFF\x86\x16`\0\x81\x81R`\x01` R`@\x90 \x7F1\xFA.,\xD2\x80\xC97^\x13\xFF\xCF=\x81\xE27\x81\0\x18n@X\xF8\xD3\xDD\xB6\x90\xB8-\xCD1\xF7\x90\x87\x87\x85\x81\x81\x10a\reWa\rea9\x9AV[\x90P` \x02\x015\x81T\x81\x10a\r|Wa\r|a9\x9AV[`\0\x91\x82R` \x91\x82\x90 \x01T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x01`@Q\x80\x91\x03\x90\xA2`\xFF\x86\x16`\0\x81\x81R`\x01` R`@\x90 \x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x90\x87\x87\x85\x81\x81\x10a\r\xE8Wa\r\xE8a9\x9AV[\x90P` \x02\x015\x81T\x81\x10a\r\xFFWa\r\xFFa9\x9AV[`\0\x91\x82R` \x80\x83 \x91\x90\x91\x01T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82R\x91\x81\x01\x92\x90\x92R\x01`@Q\x80\x91\x03\x90\xA2`\xFF\x86\x16`\0\x90\x81R`\x01` \x81\x90R`@\x90\x91 \x80T\x90\x91a\x0EQ\x91a;\x1AV[\x81T\x81\x10a\x0EaWa\x0Eaa9\x9AV[`\0\x91\x82R` \x80\x83 `\xFF\x8A\x16\x84R`\x01\x90\x91R`@\x90\x92 \x91\x01\x90\x86\x86\x84\x81\x81\x10a\x0E\x90Wa\x0E\x90a9\x9AV[\x90P` \x02\x015\x81T\x81\x10a\x0E\xA7Wa\x0E\xA7a9\x9AV[`\0\x91\x82R` \x80\x83 \x84T\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92\x83\x17\x81U\x93T`\x01`\x01``\x1B\x03`\x01`\xA0\x1B\x91\x82\x90\x04\x16\x02\x90\x91\x17\x90\x92U`\xFF\x88\x16\x81R`\x01\x90\x91R`@\x90 \x80T\x80a\x0F\nWa\x0F\na;\xCAV[`\0\x82\x81R` \x81 \x82\x01`\0\x19\x90\x81\x01\x91\x90\x91U\x01\x90U`\x01\x01a\r\x17V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x88W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xAC\x91\x90a;1V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0F\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xBC\x90a;NV[`\0T\x85\x90b\x01\0\0\x90\x04a\xFF\xFF\x16`\xFF\x82\x16\x10a\x10\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xBC\x90a9\xB0V[\x83\x80a\x10\x8CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FVoteWeigherBase.modifyStrategyWe`D\x82\x01R\x7Fights: no strategy indices provi`d\x82\x01Rb\x19\x19Y`\xEA\x1B`\x84\x82\x01R`\xA4\x01a\x06\xBCV[\x82\x81\x14a\x11\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R\x7FVoteWeigherBase.modifyStrategyWe`D\x82\x01R\x7Fights: input length mismatch\0\0\0\0`d\x82\x01R`\x84\x01a\x06\xBCV[`\0[\x81\x81\x10\x15a\x12VW\x84\x84\x82\x81\x81\x10a\x11\x1EWa\x11\x1Ea9\x9AV[\x90P` \x02\x01` \x81\x01\x90a\x113\x91\x90a;\xE0V[`\xFF\x89\x16`\0\x90\x81R`\x01` R`@\x90 \x88\x88\x84\x81\x81\x10a\x11WWa\x11Wa9\x9AV[\x90P` \x02\x015\x81T\x81\x10a\x11nWa\x11na9\x9AV[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x80T`\x01`\x01``\x1B\x03\x94\x90\x94\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x93\x90\x93\x17\x90\x92U`\xFF\x8A\x16\x80\x82R`\x01\x90\x92R`@\x90 \x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x90\x89\x89\x85\x81\x81\x10a\x11\xE5Wa\x11\xE5a9\x9AV[\x90P` \x02\x015\x81T\x81\x10a\x11\xFCWa\x11\xFCa9\x9AV[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x87\x87\x85\x81\x81\x10a\x12#Wa\x12#a9\x9AV[\x90P` \x02\x01` \x81\x01\x90a\x128\x91\x90a;\xE0V[`@Qa\x12F\x92\x91\x90a9xV[`@Q\x80\x91\x03\x90\xA2`\x01\x01a\x11\x04V[PPPPPPPPV[`\0a\x12m\x84\x84\x84a!\xBFV[\x94\x93PPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x12\x95WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x12\xAFWP0;\x15\x80\x15a\x12\xAFWP`\0T`\xFF\x16`\x01\x14[a\x13\x12W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x06\xBCV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x135W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x13?\x83\x83a'\x05V[\x80\x15a\x13\x85W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPV[`\0\x80a\x13\x97\x84\x84a\x1A\x89V[`@\x01Q\x91PP[\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\xFF\x83\x16`\0\x90\x81R`\x01` R`@\x90 \x80T\x83\x90\x81\x10a\x13\xDDWa\x13\xDDa9\x9AV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[`3\x81a\x01\0\x81\x10a\x140W`\0\x80\xFD[`\x02\x91\x82\x82\x04\x01\x91\x90\x06`\x0C\x02\x91PT\x90a\x01\0\n\x90\x04`\x01`\x01``\x1B\x03\x16\x81V[`\0\x82\x81Ra\x01\xB3` \x90\x81R`@\x80\x83 `\xFF\x85\x16\x84R\x82R\x80\x83 \x80T\x82Q\x81\x85\x02\x81\x01\x85\x01\x90\x93R\x80\x83R``\x94\x92\x93\x91\x92\x90\x91\x84\x01[\x82\x82\x10\x15a\x14\xE6W`\0\x84\x81R` \x90\x81\x90 `@\x80Q``\x81\x01\x82R\x91\x85\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x83\x85\x01R`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90\x82\x01R\x82R`\x01\x90\x92\x01\x91\x01a\x14\x8DV[PPPP\x90P\x92\x91PPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15PW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15t\x91\x90a;1V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x15\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xBC\x90a;NV[a\x15\xAD\x81a(IV[PV[`\0\x82\x81Ra\x01\xB3` \x90\x81R`@\x80\x83 `\xFF\x88\x16\x84R\x90\x91R\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x15\xE2Wa\x15\xE2a9\x9AV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90Pa\x169\x81\x86a)IV[`@\x01Q\x95\x94PPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xA4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xC8\x91\x90a;1V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x16\xF8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xBC\x90a;NV[`\0T\x82\x90b\x01\0\0\x90\x04a\xFF\xFF\x16`\xFF\x82\x16\x10a\x17(W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xBC\x90a9\xB0V[a\x13\x85\x83\x83a*\xCAV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\x90W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xB4\x91\x90a;1V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x17\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xBC\x90a;NV[a\x17\xEE\x82\x82a.\xFCV[PPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x18:W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xBC\x90a:\xA8V[`@\x80Q``\x80\x82\x01\x83R`\0` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90Rc\xFF\xFF\xFF\xFFC\x16\x80\x85R\x85Q\x93\x84\x01\x86R\x90\x83\x01\x82\x90R\x93\x82\x01\x81\x90R\x92\x81R\x90\x91[`\xFF\x81\x16\x84\x11\x15a\x0B\x90W`\0\x85\x85\x83`\xFF\x16\x81\x81\x10a\x18\x9AWa\x18\x9Aa9\x9AV[\x91\x90\x91\x015`\xF8\x1C\x91P`\0\x90Pa\x18\xB3\x88\x83\x87a/\x91V[\x90P\x80`\xB3\x83`\xFF\x16a\x01\0\x81\x10a\x18\xCDWa\x18\xCDa9\x9AV[\x01`\x01`\xB3\x85`\xFF\x16a\x01\0\x81\x10a\x18\xE7Wa\x18\xE7a9\x9AV[\x01Ta\x18\xF3\x91\x90a;\x1AV[\x81T\x81\x10a\x19\x03Wa\x19\x03a9\x9AV[`\0\x91\x82R` \x90\x91 \x01Ta\x19)\x91\x90`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16a;\xFBV[`\x01`\x01``\x1B\x03\x16`@\x85\x01Ra\x19A\x82\x85a%\xEBV[`@\x80Q`\xFF\x84\x16\x81R`\0` \x82\x01R\x89\x91\x7F\xE7\xC6\x0CRi/\x0E\x0F\xF8\xD1\xC2\x89\xFF\xE6<\xB0.y7?5\xE3\0\x0C\xAF\xE7\xCC\xA8\x98\x88Y4\x91\x01`@Q\x80\x91\x03\x90\xA2PP`\x01\x01a\x18xV[`\0\x80`\xB3\x85`\xFF\x16a\x01\0\x81\x10a\x19\xA3Wa\x19\xA3a9\x9AV[\x01\x83\x81T\x81\x10a\x19\xB5Wa\x19\xB5a9\x9AV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90Pa\x1A\x0C\x81\x85a)IV[`@\x01Q\x94\x93PPPPV[`\0`\xB3\x82`\xFF\x16a\x01\0\x81\x10a\x1A1Wa\x1A1a9\x9AV[\x01`\x01`\xB3\x84`\xFF\x16a\x01\0\x81\x10a\x1AKWa\x1AKa9\x9AV[\x01Ta\x1AW\x91\x90a;\x1AV[\x81T\x81\x10a\x1AgWa\x1Aga9\x9AV[`\0\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x92\x91PPV[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R\x86\x82Ra\x01\xB3\x81R\x84\x82 `\xFF\x87\x16\x83R\x81R\x84\x82 T\x85Q\x93\x84\x01\x86R\x82\x84R\x90\x83\x01\x82\x90R\x93\x82\x01R\x90\x91\x90\x81a\x1A\xE3W\x91Pa\x13\x9F\x90PV[`\0\x85\x81Ra\x01\xB3` \x90\x81R`@\x80\x83 `\xFF\x88\x16\x84R\x90\x91R\x90 a\x1B\x0B`\x01\x84a;\x1AV[\x81T\x81\x10a\x1B\x1BWa\x1B\x1Ba9\x9AV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x92Pa\x13\x9F\x91PPV[`\0[`\0Tb\x01\0\0\x90\x04a\xFF\xFF\x16`\xFF\x82\x16\x10\x15a\x13\x85W`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R\x90[\x83\x81\x10\x15a\x1E/W`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x13T*N\x87\x87\x85\x81\x81\x10a\x1B\xF3Wa\x1B\xF3a9\x9AV[\x90P` \x02\x01` \x81\x01\x90a\x1C\x08\x91\x90a<#V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1CLW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Cp\x91\x90a:\rV[`@Qc41\xAF%`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c41\xAF%\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\xDBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xFF\x91\x90a<@V[\x90P`\x01`\x01`\x01`\xC0\x1B\x03\x82\x16`\xFF\x87\x16\x1C\x81\x16\x14\x15a\x1E%W\x83Qc\xFF\xFF\xFF\xFF\x16a\x1D\xC1W`\xB3\x85`\xFF\x16a\x01\0\x81\x10a\x1D=Wa\x1D=a9\x9AV[\x01`\x01`\xB3\x87`\xFF\x16a\x01\0\x81\x10a\x1DWWa\x1DWa9\x9AV[\x01Ta\x1Dc\x91\x90a;\x1AV[\x81T\x81\x10a\x1DsWa\x1Dsa9\x9AV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x93P[`\0\x80a\x1D\xF5\x89\x89\x87\x81\x81\x10a\x1D\xD9Wa\x1D\xD9a9\x9AV[\x90P` \x02\x01` \x81\x01\x90a\x1D\xEE\x91\x90a<#V[\x85\x89a$\xE6V[\x91P\x91P\x80\x82\x87`@\x01Qa\x1E\n\x91\x90a;\xFBV[a\x1E\x14\x91\x90a:}V[`\x01`\x01``\x1B\x03\x16`@\x87\x01RPP[PP`\x01\x01a\x1B\xA8V[P\x80Qc\xFF\xFF\xFF\xFF\x16\x15a\x1EGWa\x1EG\x82\x82a%\xEBV[P`\x01\x01a\x1BsV[```\0\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1ElWa\x1Ela3\xDCV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1E\x95W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83\x81\x10\x15a \xB4W`\0\x85\x85\x83\x81\x81\x10a\x1E\xB7Wa\x1E\xB7a9\x9AV[\x91\x90\x91\x015`\xF8\x1C\x91PPc\xFF\xFF\xFF\xFF\x87\x16`\xB3\x82a\x01\0\x81\x10a\x1E\xDDWa\x1E\xDDa9\x9AV[\x01`\0\x81T\x81\x10a\x1E\xF0Wa\x1E\xF0a9\x9AV[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a\x1F\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`j`$\x82\x01R\x7FStakeRegistry.getTotalStakeIndic`D\x82\x01R\x7FesByQuorumNumbersAtBlockNumber: `d\x82\x01R\x7Fquorum has no stake history at b`\x84\x82\x01Ri67\xB1\xB5\xA7:\xB6\xB12\xB9`\xB1\x1B`\xA4\x82\x01R`\xC4\x01a\x06\xBCV[`\0`\xB3\x82`\xFF\x16a\x01\0\x81\x10a\x1F\xC8Wa\x1F\xC8a9\x9AV[\x01T\x90P`\0[\x81c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a \x9EW\x88c\xFF\xFF\xFF\xFF\x16`\xB3\x84`\xFF\x16a\x01\0\x81\x10a \x01Wa \x01a9\x9AV[\x01`\x01a \x0E\x84\x86a<iV[a \x18\x91\x90a<iV[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a .Wa .a9\x9AV[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a \x8CW`\x01a Q\x82\x84a<iV[a [\x91\x90a<iV[\x85\x85\x81Q\x81\x10a mWa ma9\x9AV[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPPa \x9EV[\x80a \x96\x81a<\x86V[\x91PPa\x1F\xCFV[PPP\x80\x80a \xAC\x90a<\xAAV[\x91PPa\x1E\x9BV[P\x94\x93PPPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R`\xB3\x83`\xFF\x16a\x01\0\x81\x10a \xF1Wa \xF1a9\x9AV[\x01\x82\x81T\x81\x10a!\x03Wa!\x03a9\x9AV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x93\x83\x01\x93\x90\x93R`\x01`@\x1B\x90\x92\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[`\0`\xB3\x82`\xFF\x16a\x01\0\x81\x10a!nWa!na9\x9AV[\x01T\x92\x91PPV[`\x01` R\x81`\0R`@`\0 \x81\x81T\x81\x10a!\x92W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x92P`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90P\x82V[`\0\x83\x81Ra\x01\xB3` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 T\x81[\x81c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a$\x16W`\0\x86\x81Ra\x01\xB3` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x90\x91R\x90 c\xFF\xFF\xFF\xFF\x85\x16\x90`\x01a\"$\x84\x86a<iV[a\".\x91\x90a<iV[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\"DWa\"Da9\x9AV[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a$\x04W`\0\x86\x81Ra\x01\xB3` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x90\x91R\x90 `\x01a\"\x84\x83\x85a<iV[a\"\x8E\x91\x90a<iV[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\"\xA4Wa\"\xA4a9\x9AV[`\0\x91\x82R` \x90\x91 \x01T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15\x80a#0WP`\0\x86\x81Ra\x01\xB3` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x90\x91R\x90 c\xFF\xFF\xFF\xFF\x85\x16\x90`\x01a\"\xF5\x84\x86a<iV[a\"\xFF\x91\x90a<iV[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a#\x15Wa#\x15a9\x9AV[`\0\x91\x82R` \x90\x91 \x01T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x11[a#\xE5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`t`$\x82\x01R\x7FStakeRegistry._getStakeUpdateInd`D\x82\x01R\x7FexForOperatorIdForQuorumAtBlockN`d\x82\x01R\x7Fumber: operatorId has no stake u`\x84\x82\x01Rs820\xBA2\x900\xBA\x10167\xB1\xB5\xA7:\xB6\xB12\xB9`a\x1B`\xA4\x82\x01R`\xC4\x01a\x06\xBCV[`\x01a#\xF1\x82\x84a<iV[a#\xFB\x91\x90a<iV[\x92PPPa\x06\x84V[\x80a$\x0E\x81a<\x86V[\x91PPa!\xDFV[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x8C`$\x82\x01R\x7FStakeRegistry._getStakeUpdateInd`D\x82\x01R\x7FexForOperatorIdForQuorumAtBlockN`d\x82\x01R\x7Fumber: no stake update found for`\x84\x82\x01R\x7F operatorId and quorumNumber at `\xA4\x82\x01Rk167\xB1\xB5\x907:\xB6\xB12\xB9`\xA1\x1B`\xC4\x82\x01R`\xE4\x01a\x06\xBCV[`@\x80Q``\x81\x01\x82R`\0` \x82\x01\x81\x90R\x91\x81\x01\x82\x90RCc\xFF\xFF\xFF\xFF\x16\x81R\x81\x90a%\x14\x84\x87a\x06\x8BV[`\x01`\x01``\x1B\x03\x16`@\x82\x01R`3`\xFF\x85\x16a\x01\0\x81\x10a%9Wa%9a9\x9AV[`\x02\x91\x82\x82\x04\x01\x91\x90\x06`\x0C\x02\x90T\x90a\x01\0\n\x90\x04`\x01`\x01``\x1B\x03\x16`\x01`\x01``\x1B\x03\x16\x81`@\x01Q`\x01`\x01``\x1B\x03\x16\x10\x15a%}W`\0`@\x82\x01R[`\0a%\x8A\x86\x86\x84a/\x91V[`@\x80\x84\x01Q\x81Q`\xFF\x89\x16\x81R`\x01`\x01``\x1B\x03\x90\x91\x16` \x82\x01R\x91\x92P\x87\x91\x7F\xE7\xC6\x0CRi/\x0E\x0F\xF8\xD1\xC2\x89\xFF\xE6<\xB0.y7?5\xE3\0\x0C\xAF\xE7\xCC\xA8\x98\x88Y4\x91\x01`@Q\x80\x91\x03\x90\xA2`@\x90\x91\x01Q\x90\x92P\x90P\x93P\x93\x91PPV[`\0`\xB3\x83`\xFF\x16a\x01\0\x81\x10a&\x04Wa&\x04a9\x9AV[\x01T\x90P\x80\x15a&nWC`\xB3\x84`\xFF\x16a\x01\0\x81\x10a&&Wa&&a9\x9AV[\x01a&2`\x01\x84a;\x1AV[\x81T\x81\x10a&BWa&Ba9\x9AV[\x90`\0R` `\0 \x01`\0\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP[c\xFF\xFF\xFF\xFFC\x16\x82R`\xB3`\xFF\x84\x16a\x01\0\x81\x10a&\x8EWa&\x8Ea9\x9AV[\x01\x80T`\x01\x81\x01\x82U`\0\x91\x82R` \x91\x82\x90 \x84Q\x91\x01\x80T\x92\x85\x01Q`@\x90\x95\x01Q`\x01`\x01``\x1B\x03\x16`\x01`@\x1B\x02k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x1B\x19c\xFF\xFF\xFF\xFF\x96\x87\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x95\x16\x96\x90\x93\x16\x95\x90\x95\x17\x92\x90\x92\x17\x16\x92\x90\x92\x17\x90\x91UPPV[`\0Ta\x01\0\x90\x04`\xFF\x16a'pW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x06\xBCV[\x80Q\x82Q\x14a'\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`;`$\x82\x01R\x7FRegistry._initialize: minimumSta`D\x82\x01R\x7FkeForQuorum length mismatch\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\xBCV[`\0[\x81Q\x81`\xFF\x16\x10\x15a\x13\x85Wa(\x1C\x81\x84\x83`\xFF\x16\x81Q\x81\x10a(\x0FWa(\x0Fa9\x9AV[` \x02` \x01\x01Qa.\xFCV[a(A\x82\x82`\xFF\x16\x81Q\x81\x10a(4Wa(4a9\x9AV[` \x02` \x01\x01Qa(IV[`\x01\x01a'\xEAV[`\0Tb\x01\0\0\x90\x04a\xFF\xFF\x16`\xC0\x81\x10a(\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`O`$\x82\x01R\x7FVoteWeigherBase._createQuorum: n`D\x82\x01R\x7Fumber of quorums cannot exceed M`d\x82\x01Rn\x10V\x17\xD4US\xD4\x95SW\xD0\xD3\xD5S\x95`\x8A\x1B`\x84\x82\x01R`\xA4\x01a\x06\xBCV[\x80a(\xF0\x81`\x01a<\xC5V[`\0`\x02a\x01\0\n\x81T\x81a\xFF\xFF\x02\x19\x16\x90\x83a\xFF\xFF\x16\x02\x17\x90UPa)\x16\x81\x84a*\xCAV[`@Q`\xFF\x82\x16\x90\x7F\x83\x1A\x9C\x86\xC4[\xB3\x03\xCA\xF3\xF0d\xBE+\xC2\xB9\xFDN\xCF\x19\xE4|J\xC0*a\xE7]\xAB\xFEU\xB4\x90`\0\x90\xA2PPPV[\x81Qc\xFF\xFF\xFF\xFF\x80\x83\x16\x91\x16\x11\x15a)\xEFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R```$\x82\x01R\x7FStakeRegistry._validateOperatorS`D\x82\x01R\x7FtakeAtBlockNumber: operatorStake`d\x82\x01R\x7FUpdate is from after blockNumber`\x84\x82\x01R`\xA4\x01a\x06\xBCV[` \x82\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x80a*\x15WP\x80c\xFF\xFF\xFF\xFF\x16\x82` \x01Qc\xFF\xFF\xFF\xFF\x16\x11[a\x17\xEEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`t`$\x82\x01R\x7FStakeRegistry._validateOperatorS`D\x82\x01R\x7FtakeAtBlockNumber: there is a ne`d\x82\x01R\x7Fwer operatorStakeUpdate availabl`\x84\x82\x01Rs2\x9012\xB37\xB92\x90167\xB1\xB5\xA7:\xB6\xB12\xB9`a\x1B`\xA4\x82\x01R`\xC4\x01a\x06\xBCV[`\0\x81Q\x11a+FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`N`$\x82\x01R`\0\x80Q` a<\xFB\x839\x81Q\x91R`D\x82\x01R\x7FnsideredAndMultipliers: no strat`d\x82\x01Rm\x19Y\xDAY\\\xC8\x1C\x1C\x9B\xDD\x9AY\x19Y`\x92\x1B`\x84\x82\x01R`\xA4\x01a\x06\xBCV[\x80Q`\xFF\x83\x16`\0\x90\x81R`\x01` \x90\x81R`@\x90\x91 T\x90a+i\x83\x83a<\xE2V[\x11\x15a+\xF1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`[`$\x82\x01R`\0\x80Q` a<\xFB\x839\x81Q\x91R`D\x82\x01R\x7FnsideredAndMultipliers: exceed M`d\x82\x01R\x7FAX_WEIGHING_FUNCTION_LENGTH\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06\xBCV[`\0[\x82\x81\x10\x15a.\xF5W`\0[a,\t\x82\x84a<\xE2V[\x81\x10\x15a,\xFCW\x84\x82\x81Q\x81\x10a,\"Wa,\"a9\x9AV[` \x02` \x01\x01Q`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\0\x88`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x82\x81T\x81\x10a,aWa,aa9\x9AV[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a,\xF4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`S`$\x82\x01R`\0\x80Q` a<\xFB\x839\x81Q\x91R`D\x82\x01R\x7FnsideredAndMultipliers: cannot a`d\x82\x01Rr\x0C\x8C\x84\x0El-\xAC\xA4\x0En\x8EL.\x8C\xAC\xEF$\x06O`k\x1B`\x84\x82\x01R`\xA4\x01a\x06\xBCV[`\x01\x01a+\xFFV[P`\0\x84\x82\x81Q\x81\x10a-\x11Wa-\x11a9\x9AV[` \x02` \x01\x01Q` \x01Q`\x01`\x01``\x1B\x03\x16\x11a-\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R`\0\x80Q` a<\xFB\x839\x81Q\x91R`D\x82\x01R\x7FnsideredAndMultipliers: cannot a`d\x82\x01R\x7Fdd strategy with zero weight\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06\xBCV[`\xFF\x85\x16`\0\x90\x81R`\x01` R`@\x90 \x84Q\x85\x90\x83\x90\x81\x10a-\xD3Wa-\xD3a9\x9AV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82T`\x01\x81\x01\x84U`\0\x93\x84R\x92\x82\x90 \x81Q\x91\x90\x92\x01Q`\x01`\x01``\x1B\x03\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x91\x01U\x83Q`\xFF\x86\x16\x90\x7F\x10V^V\xCA\xCB\xF3.\xCA&yE\xF0T\xFE\xC0.Yu\x002\xD1\x13\xD30!\x82\xAD\x96\x7FT\x04\x90\x86\x90\x84\x90\x81\x10a.PWa.Pa9\x9AV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQ`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x01`@Q\x80\x91\x03\x90\xA2\x84`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x85\x83\x81Q\x81\x10a.\xADWa.\xADa9\x9AV[` \x02` \x01\x01Q`\0\x01Q\x86\x84\x81Q\x81\x10a.\xCBWa.\xCBa9\x9AV[` \x02` \x01\x01Q` \x01Q`@Qa.\xE5\x92\x91\x90a9xV[`@Q\x80\x91\x03\x90\xA2`\x01\x01a+\xF4V[PPPPPV[\x80`3\x83`\xFF\x16a\x01\0\x81\x10a/\x14Wa/\x14a9\x9AV[`\x02\x91\x82\x82\x04\x01\x91\x90\x06`\x0C\x02a\x01\0\n\x81T\x81`\x01`\x01``\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01``\x1B\x03\x16\x02\x17\x90UP\x81`\xFF\x16\x7F&\xEE\xCF\xF2\xB7\x0B\nq\x10O\xF4\xD9@\xBAqb\xD2:\x95\xC2Hw\x1F\xC4\x87\xA7\xBE\x17\xA5\x96\xB3\xCF\x82`@Qa/\x85\x91\x90`\x01`\x01``\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PPV[`\0\x83\x81Ra\x01\xB3` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 T\x81\x90\x80\x15a0tW`\0\x86\x81Ra\x01\xB3` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x90\x91R\x90 C\x90a/\xE1`\x01\x84a;\x1AV[\x81T\x81\x10a/\xF1Wa/\xF1a9\x9AV[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x80Tc\xFF\xFF\xFF\xFF\x94\x90\x94\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x94\x16\x93\x90\x93\x17\x90\x92U\x87\x81Ra\x01\xB3\x82R`@\x80\x82 `\xFF\x89\x16\x83R\x90\x92R a0E`\x01\x83a;\x1AV[\x81T\x81\x10a0UWa0Ua9\x9AV[`\0\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x91P[P`\0\x85\x81Ra\x01\xB3` \x90\x81R`@\x80\x83 `\xFF\x88\x16\x84R\x82R\x80\x83 \x80T`\x01\x81\x01\x82U\x90\x84R\x92\x82\x90 \x86Q\x93\x01\x80T\x92\x87\x01Q\x91\x87\x01Q`\x01`\x01``\x1B\x03\x16`\x01`@\x1B\x02k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x1B\x19c\xFF\xFF\xFF\xFF\x93\x84\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x95\x16\x93\x90\x95\x16\x92\x90\x92\x17\x92\x90\x92\x17\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x90P\x93\x92PPPV[\x805`\xFF\x81\x16\x81\x14a1\x1AW`\0\x80\xFD[\x91\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a1\x1AW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a1HW`\0\x80\xFD[\x835\x92Pa1X` \x85\x01a1\tV[\x91Pa1f`@\x85\x01a1\x1FV[\x90P\x92P\x92P\x92V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x15\xADW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a1\x97W`\0\x80\xFD[a1\xA0\x83a1\tV[\x91P` \x83\x015a1\xB0\x81a1oV[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a1\xD0W`\0\x80\xFD[a1\xD9\x84a1\tV[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[\x81Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x80\x84\x01Q\x90\x91\x16\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01``\x1B\x03\x16\x90\x82\x01R``\x81\x01a\x13\x9FV[`\0\x80\x83`\x1F\x84\x01\x12a25W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a2LW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a2dW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a2\x81W`\0\x80\xFD[\x845a2\x8C\x81a1oV[\x93P` \x85\x015\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a2\xAEW`\0\x80\xFD[a2\xBA\x87\x82\x88\x01a2#V[\x95\x98\x94\x97P\x95PPPPV[`\0\x80\x83`\x1F\x84\x01\x12a2\xD8W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a2\xEFW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a2dW`\0\x80\xFD[`\0\x80`\0`@\x84\x86\x03\x12\x15a3\x1FW`\0\x80\xFD[a3(\x84a1\tV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a3CW`\0\x80\xFD[a3O\x86\x82\x87\x01a2\xC6V[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a3tW`\0\x80\xFD[a3}\x86a1\tV[\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a3\x99W`\0\x80\xFD[a3\xA5\x89\x83\x8A\x01a2\xC6V[\x90\x96P\x94P`@\x88\x015\x91P\x80\x82\x11\x15a3\xBEW`\0\x80\xFD[Pa3\xCB\x88\x82\x89\x01a2\xC6V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a4\x14Wa4\x14a3\xDCV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a4BWa4Ba3\xDCV[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a4cWa4ca3\xDCV[P`\x05\x1B` \x01\x90V[\x805`\x01`\x01``\x1B\x03\x81\x16\x81\x14a1\x1AW`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a4\x95W`\0\x80\xFD[\x815` a4\xAAa4\xA5\x83a4JV[a4\x1AV[\x82\x81R`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a4\xC9W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a5\x18W`@\x81\x89\x03\x12\x15a4\xE6W`\0\x80\x81\xFD[a4\xEEa3\xF2V[\x815a4\xF9\x81a1oV[\x81Ra5\x06\x82\x86\x01a4mV[\x81\x86\x01R\x83R\x91\x83\x01\x91`@\x01a4\xCDV[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a54W`\0\x80\xFD[\x815` a5Da4\xA5\x83a4JV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a5cW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a5\x18W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a5\x86W`\0\x80\x81\xFD[a5\x94\x89\x86\x83\x8B\x01\x01a4\x84V[\x84RP\x91\x83\x01\x91\x83\x01a5gV[`\0\x80`@\x83\x85\x03\x12\x15a5\xB5W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a5\xCCW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a5\xE0W`\0\x80\xFD[\x815` a5\xF0a4\xA5\x83a4JV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a6\x0FW`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a64Wa6%\x86a4mV[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a6\x14V[\x96PP\x86\x015\x92PP\x80\x82\x11\x15a6JW`\0\x80\xFD[Pa6W\x85\x82\x86\x01a5#V[\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a6tW`\0\x80\xFD[\x825\x91Pa6\x84` \x84\x01a1\tV[\x90P\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a6\xA0W`\0\x80\xFD[a6\xA9\x83a1\tV[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a6\xC9W`\0\x80\xFD[P5\x91\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a7<Wa7)\x83\x85Qc\xFF\xFF\xFF\xFF\x80\x82Q\x16\x83R\x80` \x83\x01Q\x16` \x84\x01RP`\x01`\x01``\x1B\x03`@\x82\x01Q\x16`@\x83\x01RPPV[\x92\x84\x01\x92``\x92\x90\x92\x01\x91`\x01\x01a6\xECV[P\x90\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a7ZW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a7pW`\0\x80\xFD[a\x12m\x84\x82\x85\x01a4\x84V[`\0` \x82\x84\x03\x12\x15a7\x8EW`\0\x80\xFD[a\x06\x84\x82a1\tV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a7\xADW`\0\x80\xFD[a7\xB6\x85a1\tV[\x93Pa7\xC4` \x86\x01a1\x1FV[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[`\0\x80`@\x83\x85\x03\x12\x15a7\xECW`\0\x80\xFD[a7\xF5\x83a1\tV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a8\x10W`\0\x80\xFD[a6W\x85\x82\x86\x01a4\x84V[`\0\x80`@\x83\x85\x03\x12\x15a8/W`\0\x80\xFD[a88\x83a1\tV[\x91Pa6\x84` \x84\x01a4mV[`\0\x80`\0`@\x84\x86\x03\x12\x15a8[W`\0\x80\xFD[\x835\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a8xW`\0\x80\xFD[a3O\x86\x82\x87\x01a2#V[`\0\x80`\0``\x84\x86\x03\x12\x15a8\x99W`\0\x80\xFD[a8\xA2\x84a1\tV[\x92Pa8\xB0` \x85\x01a1\x1FV[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0\x80` \x83\x85\x03\x12\x15a8\xD3W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a8\xE9W`\0\x80\xFD[a8\xF5\x85\x82\x86\x01a2\xC6V[\x90\x96\x90\x95P\x93PPPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a9\x16W`\0\x80\xFD[a9\x1F\x84a1\x1FV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a8xW`\0\x80\xFD[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a7<W\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a9VV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R`\x01`\x01``\x1B\x03\x16` \x82\x01R`@\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[` \x80\x82R`<\x90\x82\x01R\x7FVoteWeigherBase.validQuorumNumbe`@\x82\x01R\x7Fr: quorumNumber is not valid\0\0\0\0``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15a:\x1FW`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a:VWa:Va:&V[P\x02\x90V[`\0\x82a:xWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a:\x9FWa:\x9Fa:&V[\x01\x94\x93PPPPV[` \x80\x82R`L\x90\x82\x01R\x7FStakeRegistry.onlyRegistryCoordi`@\x82\x01R\x7Fnator: caller is not the Registr``\x82\x01Rk<\xA1\xB7\xB7\xB924\xB70\xBA7\xB9`\xA1\x1B`\x80\x82\x01R`\xA0\x01\x90V[`\0\x82\x82\x10\x15a;,Wa;,a:&V[P\x03\x90V[`\0` \x82\x84\x03\x12\x15a;CW`\0\x80\xFD[\x81Qa\x06\x84\x81a1oV[` \x80\x82R`V\x90\x82\x01R\x7FVoteWeigherBase.onlyServiceManag`@\x82\x01R\x7FerOwner: caller is not the owner``\x82\x01Ru\x107\xB3\x10:42\x909\xB2\xB9;4\xB1\xB2\xA6\xB0\xB70\xB3\xB2\xB9`Q\x1B`\x80\x82\x01R`\xA0\x01\x90V[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a;\xF2W`\0\x80\xFD[a\x06\x84\x82a4mV[`\0`\x01`\x01``\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a<\x1BWa<\x1Ba:&V[\x03\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a<5W`\0\x80\xFD[\x815a\x06\x84\x81a1oV[`\0` \x82\x84\x03\x12\x15a<RW`\0\x80\xFD[\x81Q`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x14a\x06\x84W`\0\x80\xFD[`\0c\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a<\x1BWa<\x1Ba:&V[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x14\x15a<\xA0Wa<\xA0a:&V[`\x01\x01\x93\x92PPPV[`\0`\0\x19\x82\x14\x15a<\xBEWa<\xBEa:&V[P`\x01\x01\x90V[`\0a\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a:\x9FWa:\x9Fa:&V[`\0\x82\x19\x82\x11\x15a<\xF5Wa<\xF5a:&V[P\x01\x90V\xFEVoteWeigherBase._addStrategiesCo\xA2dipfsX\"\x12 \xF5\xED\xCB\xD7\xB8\x06\xA3;\x02>u\xFAT\xE6<\xDC=\xEC\xC5\x99\x11\xB0f\x063\xBE\x9C\x91a\xD9\xB8?dsolcC\0\x08\x0C\x003";
    /// The deployed bytecode of the contract.
    pub static STAKEREGISTRY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct StakeRegistry<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for StakeRegistry<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for StakeRegistry<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for StakeRegistry<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for StakeRegistry<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(StakeRegistry))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> StakeRegistry<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    STAKEREGISTRY_ABI.clone(),
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
                STAKEREGISTRY_ABI.clone(),
                STAKEREGISTRY_BYTECODE.clone().into(),
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
            StakeRegistryEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for StakeRegistry<M> {
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
    pub enum StakeRegistryEvents {
        InitializedFilter(InitializedFilter),
        MinimumStakeForQuorumUpdatedFilter(MinimumStakeForQuorumUpdatedFilter),
        QuorumCreatedFilter(QuorumCreatedFilter),
        StakeUpdateFilter(StakeUpdateFilter),
        StrategyAddedToQuorumFilter(StrategyAddedToQuorumFilter),
        StrategyMultiplierUpdatedFilter(StrategyMultiplierUpdatedFilter),
        StrategyRemovedFromQuorumFilter(StrategyRemovedFromQuorumFilter),
    }
    impl ::ethers::contract::EthLogDecode for StakeRegistryEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(StakeRegistryEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = MinimumStakeForQuorumUpdatedFilter::decode_log(log) {
                return Ok(
                    StakeRegistryEvents::MinimumStakeForQuorumUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = QuorumCreatedFilter::decode_log(log) {
                return Ok(StakeRegistryEvents::QuorumCreatedFilter(decoded));
            }
            if let Ok(decoded) = StakeUpdateFilter::decode_log(log) {
                return Ok(StakeRegistryEvents::StakeUpdateFilter(decoded));
            }
            if let Ok(decoded) = StrategyAddedToQuorumFilter::decode_log(log) {
                return Ok(StakeRegistryEvents::StrategyAddedToQuorumFilter(decoded));
            }
            if let Ok(decoded) = StrategyMultiplierUpdatedFilter::decode_log(log) {
                return Ok(StakeRegistryEvents::StrategyMultiplierUpdatedFilter(decoded));
            }
            if let Ok(decoded) = StrategyRemovedFromQuorumFilter::decode_log(log) {
                return Ok(StakeRegistryEvents::StrategyRemovedFromQuorumFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for StakeRegistryEvents {
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
    impl ::core::convert::From<InitializedFilter> for StakeRegistryEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<MinimumStakeForQuorumUpdatedFilter>
    for StakeRegistryEvents {
        fn from(value: MinimumStakeForQuorumUpdatedFilter) -> Self {
            Self::MinimumStakeForQuorumUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<QuorumCreatedFilter> for StakeRegistryEvents {
        fn from(value: QuorumCreatedFilter) -> Self {
            Self::QuorumCreatedFilter(value)
        }
    }
    impl ::core::convert::From<StakeUpdateFilter> for StakeRegistryEvents {
        fn from(value: StakeUpdateFilter) -> Self {
            Self::StakeUpdateFilter(value)
        }
    }
    impl ::core::convert::From<StrategyAddedToQuorumFilter> for StakeRegistryEvents {
        fn from(value: StrategyAddedToQuorumFilter) -> Self {
            Self::StrategyAddedToQuorumFilter(value)
        }
    }
    impl ::core::convert::From<StrategyMultiplierUpdatedFilter> for StakeRegistryEvents {
        fn from(value: StrategyMultiplierUpdatedFilter) -> Self {
            Self::StrategyMultiplierUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<StrategyRemovedFromQuorumFilter> for StakeRegistryEvents {
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
    pub enum StakeRegistryCalls {
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
        RegisterOperator(RegisterOperatorCall),
        RegistryCoordinator(RegistryCoordinatorCall),
        RemoveStrategiesConsideredAndMultipliers(
            RemoveStrategiesConsideredAndMultipliersCall,
        ),
        ServiceManager(ServiceManagerCall),
        SetMinimumStakeForQuorum(SetMinimumStakeForQuorumCall),
        Slasher(SlasherCall),
        StrategiesConsideredAndMultipliers(StrategiesConsideredAndMultipliersCall),
        StrategiesConsideredAndMultipliersLength(
            StrategiesConsideredAndMultipliersLengthCall,
        ),
        StrategyAndWeightingMultiplierForQuorumByIndex(
            StrategyAndWeightingMultiplierForQuorumByIndexCall,
        ),
        StrategyManager(StrategyManagerCall),
        UpdateStakes(UpdateStakesCall),
        WeightOfOperatorForQuorum(WeightOfOperatorForQuorumCall),
    }
    impl ::ethers::core::abi::AbiDecode for StakeRegistryCalls {
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
            if let Ok(decoded) = <RegisterOperatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RegisterOperator(decoded));
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
    impl ::ethers::core::abi::AbiEncode for StakeRegistryCalls {
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
                Self::RegisterOperator(element) => {
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
                Self::UpdateStakes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WeightOfOperatorForQuorum(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for StakeRegistryCalls {
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
                Self::RegisterOperator(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::UpdateStakes(element) => ::core::fmt::Display::fmt(element, f),
                Self::WeightOfOperatorForQuorum(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<MaxQuorumCountCall> for StakeRegistryCalls {
        fn from(value: MaxQuorumCountCall) -> Self {
            Self::MaxQuorumCount(value)
        }
    }
    impl ::core::convert::From<MaxWeighingFunctionLengthCall> for StakeRegistryCalls {
        fn from(value: MaxWeighingFunctionLengthCall) -> Self {
            Self::MaxWeighingFunctionLength(value)
        }
    }
    impl ::core::convert::From<WeightingDivisorCall> for StakeRegistryCalls {
        fn from(value: WeightingDivisorCall) -> Self {
            Self::WeightingDivisor(value)
        }
    }
    impl ::core::convert::From<AddStrategiesConsideredAndMultipliersCall>
    for StakeRegistryCalls {
        fn from(value: AddStrategiesConsideredAndMultipliersCall) -> Self {
            Self::AddStrategiesConsideredAndMultipliers(value)
        }
    }
    impl ::core::convert::From<CreateQuorumCall> for StakeRegistryCalls {
        fn from(value: CreateQuorumCall) -> Self {
            Self::CreateQuorum(value)
        }
    }
    impl ::core::convert::From<DelegationCall> for StakeRegistryCalls {
        fn from(value: DelegationCall) -> Self {
            Self::Delegation(value)
        }
    }
    impl ::core::convert::From<DeregisterOperatorCall> for StakeRegistryCalls {
        fn from(value: DeregisterOperatorCall) -> Self {
            Self::DeregisterOperator(value)
        }
    }
    impl ::core::convert::From<GetCurrentOperatorStakeForQuorumCall>
    for StakeRegistryCalls {
        fn from(value: GetCurrentOperatorStakeForQuorumCall) -> Self {
            Self::GetCurrentOperatorStakeForQuorum(value)
        }
    }
    impl ::core::convert::From<GetCurrentTotalStakeForQuorumCall>
    for StakeRegistryCalls {
        fn from(value: GetCurrentTotalStakeForQuorumCall) -> Self {
            Self::GetCurrentTotalStakeForQuorum(value)
        }
    }
    impl ::core::convert::From<GetLengthOfOperatorIdStakeHistoryForQuorumCall>
    for StakeRegistryCalls {
        fn from(value: GetLengthOfOperatorIdStakeHistoryForQuorumCall) -> Self {
            Self::GetLengthOfOperatorIdStakeHistoryForQuorum(value)
        }
    }
    impl ::core::convert::From<GetLengthOfTotalStakeHistoryForQuorumCall>
    for StakeRegistryCalls {
        fn from(value: GetLengthOfTotalStakeHistoryForQuorumCall) -> Self {
            Self::GetLengthOfTotalStakeHistoryForQuorum(value)
        }
    }
    impl ::core::convert::From<GetMostRecentStakeUpdateByOperatorIdCall>
    for StakeRegistryCalls {
        fn from(value: GetMostRecentStakeUpdateByOperatorIdCall) -> Self {
            Self::GetMostRecentStakeUpdateByOperatorId(value)
        }
    }
    impl ::core::convert::From<GetOperatorIdToStakeHistoryCall> for StakeRegistryCalls {
        fn from(value: GetOperatorIdToStakeHistoryCall) -> Self {
            Self::GetOperatorIdToStakeHistory(value)
        }
    }
    impl ::core::convert::From<GetStakeForOperatorIdForQuorumAtBlockNumberCall>
    for StakeRegistryCalls {
        fn from(value: GetStakeForOperatorIdForQuorumAtBlockNumberCall) -> Self {
            Self::GetStakeForOperatorIdForQuorumAtBlockNumber(value)
        }
    }
    impl ::core::convert::From<GetStakeForQuorumAtBlockNumberFromOperatorIdAndIndexCall>
    for StakeRegistryCalls {
        fn from(
            value: GetStakeForQuorumAtBlockNumberFromOperatorIdAndIndexCall,
        ) -> Self {
            Self::GetStakeForQuorumAtBlockNumberFromOperatorIdAndIndex(value)
        }
    }
    impl ::core::convert::From<GetStakeUpdateForQuorumFromOperatorIdAndIndexCall>
    for StakeRegistryCalls {
        fn from(value: GetStakeUpdateForQuorumFromOperatorIdAndIndexCall) -> Self {
            Self::GetStakeUpdateForQuorumFromOperatorIdAndIndex(value)
        }
    }
    impl ::core::convert::From<
        GetStakeUpdateIndexForOperatorIdForQuorumAtBlockNumberCall,
    > for StakeRegistryCalls {
        fn from(
            value: GetStakeUpdateIndexForOperatorIdForQuorumAtBlockNumberCall,
        ) -> Self {
            Self::GetStakeUpdateIndexForOperatorIdForQuorumAtBlockNumber(value)
        }
    }
    impl ::core::convert::From<GetTotalStakeAtBlockNumberFromIndexCall>
    for StakeRegistryCalls {
        fn from(value: GetTotalStakeAtBlockNumberFromIndexCall) -> Self {
            Self::GetTotalStakeAtBlockNumberFromIndex(value)
        }
    }
    impl ::core::convert::From<GetTotalStakeIndicesByQuorumNumbersAtBlockNumberCall>
    for StakeRegistryCalls {
        fn from(value: GetTotalStakeIndicesByQuorumNumbersAtBlockNumberCall) -> Self {
            Self::GetTotalStakeIndicesByQuorumNumbersAtBlockNumber(value)
        }
    }
    impl ::core::convert::From<GetTotalStakeUpdateForQuorumFromIndexCall>
    for StakeRegistryCalls {
        fn from(value: GetTotalStakeUpdateForQuorumFromIndexCall) -> Self {
            Self::GetTotalStakeUpdateForQuorumFromIndex(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for StakeRegistryCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<MinimumStakeForQuorumCall> for StakeRegistryCalls {
        fn from(value: MinimumStakeForQuorumCall) -> Self {
            Self::MinimumStakeForQuorum(value)
        }
    }
    impl ::core::convert::From<ModifyStrategyWeightsCall> for StakeRegistryCalls {
        fn from(value: ModifyStrategyWeightsCall) -> Self {
            Self::ModifyStrategyWeights(value)
        }
    }
    impl ::core::convert::From<QuorumCountCall> for StakeRegistryCalls {
        fn from(value: QuorumCountCall) -> Self {
            Self::QuorumCount(value)
        }
    }
    impl ::core::convert::From<RegisterOperatorCall> for StakeRegistryCalls {
        fn from(value: RegisterOperatorCall) -> Self {
            Self::RegisterOperator(value)
        }
    }
    impl ::core::convert::From<RegistryCoordinatorCall> for StakeRegistryCalls {
        fn from(value: RegistryCoordinatorCall) -> Self {
            Self::RegistryCoordinator(value)
        }
    }
    impl ::core::convert::From<RemoveStrategiesConsideredAndMultipliersCall>
    for StakeRegistryCalls {
        fn from(value: RemoveStrategiesConsideredAndMultipliersCall) -> Self {
            Self::RemoveStrategiesConsideredAndMultipliers(value)
        }
    }
    impl ::core::convert::From<ServiceManagerCall> for StakeRegistryCalls {
        fn from(value: ServiceManagerCall) -> Self {
            Self::ServiceManager(value)
        }
    }
    impl ::core::convert::From<SetMinimumStakeForQuorumCall> for StakeRegistryCalls {
        fn from(value: SetMinimumStakeForQuorumCall) -> Self {
            Self::SetMinimumStakeForQuorum(value)
        }
    }
    impl ::core::convert::From<SlasherCall> for StakeRegistryCalls {
        fn from(value: SlasherCall) -> Self {
            Self::Slasher(value)
        }
    }
    impl ::core::convert::From<StrategiesConsideredAndMultipliersCall>
    for StakeRegistryCalls {
        fn from(value: StrategiesConsideredAndMultipliersCall) -> Self {
            Self::StrategiesConsideredAndMultipliers(value)
        }
    }
    impl ::core::convert::From<StrategiesConsideredAndMultipliersLengthCall>
    for StakeRegistryCalls {
        fn from(value: StrategiesConsideredAndMultipliersLengthCall) -> Self {
            Self::StrategiesConsideredAndMultipliersLength(value)
        }
    }
    impl ::core::convert::From<StrategyAndWeightingMultiplierForQuorumByIndexCall>
    for StakeRegistryCalls {
        fn from(value: StrategyAndWeightingMultiplierForQuorumByIndexCall) -> Self {
            Self::StrategyAndWeightingMultiplierForQuorumByIndex(value)
        }
    }
    impl ::core::convert::From<StrategyManagerCall> for StakeRegistryCalls {
        fn from(value: StrategyManagerCall) -> Self {
            Self::StrategyManager(value)
        }
    }
    impl ::core::convert::From<UpdateStakesCall> for StakeRegistryCalls {
        fn from(value: UpdateStakesCall) -> Self {
            Self::UpdateStakes(value)
        }
    }
    impl ::core::convert::From<WeightOfOperatorForQuorumCall> for StakeRegistryCalls {
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
