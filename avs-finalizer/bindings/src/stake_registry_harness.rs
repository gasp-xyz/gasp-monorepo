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
                        name: ::std::borrow::ToOwned::to_owned("_delegationManager"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "contract IDelegationManager",
                            ),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
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
                    ::std::borrow::ToOwned::to_owned("addStrategies"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addStrategies"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_strategyParams"),
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
                                            "struct IStakeRegistry.StrategyParams[]",
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
                    ::std::borrow::ToOwned::to_owned("applyDelta"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("applyDelta"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint96"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("delta"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("calculateDelta"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("calculateDelta"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("prev"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint96"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cur"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint96"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                    ::std::borrow::ToOwned::to_owned("getCurrentStake"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getCurrentStake"),
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
                    ::std::borrow::ToOwned::to_owned("getCurrentTotalStake"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getCurrentTotalStake",
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
                    ::std::borrow::ToOwned::to_owned("getLatestStakeUpdate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getLatestStakeUpdate",
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
                                            "struct IStakeRegistry.StakeUpdate",
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
                    ::std::borrow::ToOwned::to_owned("getStakeAtBlockNumber"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getStakeAtBlockNumber",
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
                    ::std::borrow::ToOwned::to_owned("getStakeAtBlockNumberAndIndex"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getStakeAtBlockNumberAndIndex",
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
                    ::std::borrow::ToOwned::to_owned("getStakeHistory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getStakeHistory"),
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
                                            "struct IStakeRegistry.StakeUpdate[]",
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
                    ::std::borrow::ToOwned::to_owned("getStakeHistoryLength"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getStakeHistoryLength",
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
                    ::std::borrow::ToOwned::to_owned("getStakeUpdateAtIndex"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getStakeUpdateAtIndex",
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
                                            "struct IStakeRegistry.StakeUpdate",
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
                    ::std::borrow::ToOwned::to_owned("getStakeUpdateIndexAtBlockNumber"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getStakeUpdateIndexAtBlockNumber",
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
                    ::std::borrow::ToOwned::to_owned("getTotalStakeHistoryLength"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getTotalStakeHistoryLength",
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
                        "getTotalStakeIndicesAtBlockNumber",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getTotalStakeIndicesAtBlockNumber",
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
                    ::std::borrow::ToOwned::to_owned("getTotalStakeUpdateAtIndex"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getTotalStakeUpdateAtIndex",
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
                                            "struct IStakeRegistry.StakeUpdate",
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
                    ::std::borrow::ToOwned::to_owned("initializeQuorum"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initializeQuorum"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_strategyParams"),
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
                                            "struct IStakeRegistry.StrategyParams[]",
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
                    ::std::borrow::ToOwned::to_owned("modifyStrategyParams"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "modifyStrategyParams",
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
                                    name: ::std::borrow::ToOwned::to_owned("newStake"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint96"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
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
                                    name: ::std::borrow::ToOwned::to_owned("stakeDelta"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                                    name: ::std::string::String::new(),
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
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeStrategies"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeStrategies"),
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
                    ::std::borrow::ToOwned::to_owned("strategyParams"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("strategyParams"),
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
                    ::std::borrow::ToOwned::to_owned("strategyParamsByIndex"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "strategyParamsByIndex",
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
                                            "struct IStakeRegistry.StrategyParams",
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
                    ::std::borrow::ToOwned::to_owned("strategyParamsLength"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "strategyParamsLength",
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        192usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint192"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("OperatorStakeUpdate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OperatorStakeUpdate",
                            ),
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
    const __BYTECODE: &[u8] = b"`\xC0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\08]8\x03\x80b\08]\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\0eV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\xA0R\x16`\x80Rb\0\0\xA4V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0bW`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15b\0\0yW`\0\x80\xFD[\x82Qb\0\0\x86\x81b\0\0LV[` \x84\x01Q\x90\x92Pb\0\0\x99\x81b\0\0LV[\x80\x91PP\x92P\x92\x90PV[`\x80Q`\xA0Qa7Tb\0\x01\t`\09`\0\x81\x81a\x03\xAE\x01R\x81\x81a\x06\x9B\x01R\x81\x81a\t\xD8\x01R\x81\x81a\rO\x01R\x81\x81a\x10\xA5\x01R\x81\x81a\x15\xE9\x01R\x81\x81a\x16\xE6\x01R\x81\x81a\x18\n\x01Ra\x1B\xCC\x01R`\0\x81\x81a\x05p\x01Ra\x1Fj\x01Ra7T`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\x06W`\x005`\xE0\x1C\x80c\x81\xC0u\x02\x11a\x01\x1AW\x80c\xC8)LV\x11a\0\xADW\x80c\xF2\xBE\x94\xAE\x11a\0|W\x80c\xF2\xBE\x94\xAE\x14a\x05\x92W\x80c\xF5\tU\x1A\x14a\x05\xA5W\x80c\xF8Q\xE1\x98\x14a\x05\xB8W\x80c\xFA(\xC6'\x14a\x05\xCBW\x80c\xFFiJw\x14a\x05\xDEW`\0\x80\xFD[\x80c\xC8)LV\x14a\x05\x1DW\x80c\xD5\xEC\xCC\x05\x14a\x050W\x80c\xDD\x98F\xB9\x14a\x05CW\x80c\xDF\\\xF7#\x14a\x05kW`\0\x80\xFD[\x80c\xBC\x9A@\xC3\x11a\0\xE9W\x80c\xBC\x9A@\xC3\x14a\x04\xBBW\x80c\xBD)\xB8\xCD\x14a\x04\xCEW\x80c\xC4gx\xA5\x14a\x04\xE1W\x80c\xC6\x01R}\x14a\x05\nW`\0\x80\xFD[\x80c\x81\xC0u\x02\x14a\x04(W\x80c\xACk\xFB\x03\x14a\x04HW\x80c\xAD\xC8\x04\xDA\x14a\x04hW\x80c\xB6\x90Kx\x14a\x04\xA8W`\0\x80\xFD[\x80cK\xD2n\t\x11a\x01\x9DW\x80cf\xAC\xFE\xFE\x11a\x01lW\x80cf\xAC\xFE\xFE\x14a\x03~W\x80cm\x14\xA9\x87\x14a\x03\xA9W\x80ctELm\x14a\x03\xE8W\x80c|\x17#G\x14a\x03\xFBW\x80c\x7FB\x98\"\x14a\x04\x15W`\0\x80\xFD[\x80cK\xD2n\t\x14a\x03\x19W\x80cT\x01\xED'\x14a\x03IW\x80c^Zgu\x14a\x03\\W\x80c_\x1F-w\x14a\x03kW`\0\x80\xFD[\x80c \xB6b\x98\x11a\x01\xD9W\x80c \xB6b\x98\x14a\x02\xA2W\x80c%PGw\x14a\x02\xB5W\x80c,\xD9Y@\x14a\x02\xD6W\x80c<\xA5\xA5\xF5\x14a\x02\xF6W`\0\x80\xFD[\x80c\x03\x90\xA4\xD5\x14a\x02\x0BW\x80c\x04\x91\xB4\x1C\x14a\x02 W\x80c\x08s$a\x14a\x02VW\x80c\x1F\x9Bt\xE0\x14a\x02wW[`\0\x80\xFD[a\x02\x1Ea\x02\x196`\x04a+\xAEV[a\x05\xF1V[\0[a\x02Ca\x02.6`\x04a+\xD8V[`\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02ia\x02d6`\x04a+\xAEV[a\x06\0V[`@Qa\x02M\x92\x91\x90a+\xF3V[a\x02\x8Aa\x02\x856`\x04a,-V[a\x06IV[`@Q`\x01`\x01``\x1B\x03\x90\x91\x16\x81R` \x01a\x02MV[a\x02\x1Ea\x02\xB06`\x04a,\xA8V[a\x06\x99V[a\x02\xC8a\x02\xC36`\x04a-iV[a\t\xCAV[`@Qa\x02M\x92\x91\x90a.\x08V[a\x02\xE9a\x02\xE46`\x04a.-V[a\x0C\x95V[`@Qa\x02M\x91\x90a.YV[a\x02Ca\x03\x046`\x04a+\xD8V[`\xFF\x16`\0\x90\x81R`\x03` R`@\x90 T\x90V[a\x02Ca\x03'6`\x04a.-V[`\0\x91\x82R`\x02` \x90\x81R`@\x80\x84 `\xFF\x93\x90\x93\x16\x84R\x91\x90R\x90 T\x90V[a\x02\x8Aa\x03W6`\x04a.-V[a\r4V[a\x02Cg\r\xE0\xB6\xB3\xA7d\0\0\x81V[a\x02\x1Ea\x03y6`\x04a/bV[a\rMV[a\x03\x91a\x03\x8C6`\x04a-iV[a\x10\x98V[`@Q`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x81R` \x01a\x02MV[a\x03\xD0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02MV[a\x02Ca\x03\xF66`\x04a0!V[a\x11\xF2V[a\x04\x03` \x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02MV[a\x02\x8Aa\x04#6`\x04a0]V[a\x12\tV[a\x04;a\x0466`\x04a0\x8DV[a\x12\x15V[`@Qa\x02M\x91\x90a0\xDFV[a\x04[a\x04V6`\x04a1\x1DV[a\x14FV[`@Qa\x02M\x91\x90a1PV[a\x04{a\x04v6`\x04a+\xAEV[a\x14\xDEV[`@\x80Q\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x92\x83\x01Q`\x01`\x01``\x1B\x03\x16\x92\x81\x01\x92\x90\x92R\x01a\x02MV[a\x04[a\x04\xB66`\x04a+\xAEV[a\x15XV[a\x02\x1Ea\x04\xC96`\x04a1\x85V[a\x15\xE7V[a\x02\x1Ea\x04\xDC6`\x04a1\xAFV[a\x16\xDBV[a\x02\x8Aa\x04\xEF6`\x04a+\xD8V[`\0` \x81\x90R\x90\x81R`@\x90 T`\x01`\x01``\x1B\x03\x16\x81V[a\x02\x1Ea\x05\x186`\x04a2{V[a\x18\x08V[a\x02\x8Aa\x05+6`\x04a2\xC8V[a\x18\xFCV[a\x02\x8Aa\x05>6`\x04a+\xD8V[a\x19zV[a\x05Va\x05Q6`\x04a3\x04V[a\x19\xCDV[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02MV[a\x03\xD0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\x8Aa\x05\xA06`\x04a37V[a\x19\xDAV[a\x02Ca\x05\xB36`\x04a3yV[a\x1AoV[a\x04[a\x05\xC66`\x04a.-V[a\x1A{V[a\x02\x8Aa\x05\xD96`\x04a3\x04V[a\x1B`V[a\x02\x1Ea\x05\xEC6`\x04a3\x95V[a\x1B\xC1V[a\x05\xFB\x82\x82a\x1D,V[PPPV[`\x03` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x06\x1CW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x92P`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90P\x82V[`\xFF\x82\x16`\0\x90\x81R`\x01` R`@\x81 T\x83\x90a\x06\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06z\x90a3\xF2V[`@Q\x80\x91\x03\x90\xFD[`\0a\x06\x8F\x85\x85a\x1E\xA6V[P\x95\x94PPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xF7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x1B\x91\x90a4CV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x07KW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06z\x90a4`V[\x84a\x07g\x81`\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x15\x15\x90V[a\x07\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06z\x90a3\xF2V[\x83\x80a\x07\xF9W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FStakeRegistry.modifyStrategyPara`D\x82\x01R\x7Fms: no strategy indices provided`d\x82\x01R`\x84\x01a\x06zV[\x82\x81\x14a\x08nW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FStakeRegistry.modifyStrategyPara`D\x82\x01R\x7Fms: input length mismatch\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06zV[`\xFF\x87\x16`\0\x90\x81R`\x03` R`@\x81 \x90[\x82\x81\x10\x15a\t\xBFW\x85\x85\x82\x81\x81\x10a\x08\x9CWa\x08\x9Ca4\xDCV[\x90P` \x02\x01` \x81\x01\x90a\x08\xB1\x91\x90a4\xF2V[\x82\x89\x89\x84\x81\x81\x10a\x08\xC4Wa\x08\xC4a4\xDCV[\x90P` \x02\x015\x81T\x81\x10a\x08\xDBWa\x08\xDBa4\xDCV[\x90`\0R` `\0 \x01`\0\x01`\x14a\x01\0\n\x81T\x81`\x01`\x01``\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01``\x1B\x03\x16\x02\x17\x90UP\x88`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x83\x8A\x8A\x85\x81\x81\x10a\tDWa\tDa4\xDCV[\x90P` \x02\x015\x81T\x81\x10a\t[Wa\t[a4\xDCV[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x88\x88\x85\x81\x81\x10a\t\x82Wa\t\x82a4\xDCV[\x90P` \x02\x01` \x81\x01\x90a\t\x97\x91\x90a4\xF2V[`@Qa\t\xA5\x92\x91\x90a+\xF3V[`@Q\x80\x91\x03\x90\xA2\x80a\t\xB7\x81a5#V[\x91PPa\x08\x82V[PPPPPPPPPV[``\x803`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\n\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06z\x90a5>V[`\0\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\n/Wa\n/a.\xD1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\nXW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x84`\x01`\x01`@\x1B\x03\x81\x11\x15a\nuWa\nua.\xD1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\n\x9EW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x85\x81\x10\x15a\x0C\x87W`\0\x87\x87\x83\x81\x81\x10a\n\xC0Wa\n\xC0a4\xDCV[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x01` R`@\x90 T\x90\x92P\x15\x15\x90Pa\x0BHW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FStakeRegistry.registerOperator: `D\x82\x01Rt\x1C][\xDC\x9D[H\x19\x1B\xD9\\\xC8\x1B\x9B\xDD\x08\x19^\x1A\\\xDD`Z\x1B`d\x82\x01R`\x84\x01a\x06zV[`\0\x80a\x0BU\x83\x8Da\x1E\xA6V[\x91P\x91P\x80a\x0B\xF2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`[`$\x82\x01R\x7FStakeRegistry.registerOperator: `D\x82\x01R\x7FOperator does not meet minimum s`d\x82\x01R\x7Ftake requirement for quorum\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06zV[`\0a\x0B\xFF\x8C\x85\x85a \\V[\x90P\x82\x87\x86\x81Q\x81\x10a\x0C\x14Wa\x0C\x14a4\xDCV[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPPa\x0C>\x84\x82a\x1D,V[\x86\x86\x81Q\x81\x10a\x0CPWa\x0CPa4\xDCV[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPPPPPP\x80\x80a\x0C\x7F\x90a5#V[\x91PPa\n\xA4V[P\x90\x97\x90\x96P\x94PPPPPV[`\0\x82\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x85\x16\x84R\x82R\x80\x83 \x80T\x82Q\x81\x85\x02\x81\x01\x85\x01\x90\x93R\x80\x83R``\x94\x92\x93\x91\x92\x90\x91\x84\x01[\x82\x82\x10\x15a\r'W`\0\x84\x81R` \x90\x81\x90 `@\x80Q``\x81\x01\x82R\x91\x85\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x83\x85\x01R`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90\x82\x01R\x82R`\x01\x90\x92\x01\x91\x01a\x0C\xCEV[PPPP\x90P[\x92\x91PPV[`\0\x80a\rA\x84\x84a\x1A{V[`@\x01Q\x94\x93PPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xABW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xCF\x91\x90a4CV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\r\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06z\x90a4`V[\x81a\x0E\x1B\x81`\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x15\x15\x90V[a\x0E7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06z\x90a3\xF2V[\x81Q\x80a\x0E\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FStakeRegistry.removeStrategies: `D\x82\x01R\x7Fno indices to remove provided\0\0\0`d\x82\x01R`\x84\x01a\x06zV[`\xFF\x84\x16`\0\x90\x81R`\x03` R`@\x81 \x90[\x82\x81\x10\x15a\x10\x90W\x85`\xFF\x16\x7F1\xFA.,\xD2\x80\xC97^\x13\xFF\xCF=\x81\xE27\x81\0\x18n@X\xF8\xD3\xDD\xB6\x90\xB8-\xCD1\xF7\x83\x87\x84\x81Q\x81\x10a\x0F\0Wa\x0F\0a4\xDCV[` \x02` \x01\x01Q\x81T\x81\x10a\x0F\x18Wa\x0F\x18a4\xDCV[`\0\x91\x82R` \x91\x82\x90 \x01T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x01`@Q\x80\x91\x03\x90\xA2\x85`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x83\x87\x84\x81Q\x81\x10a\x0FvWa\x0Fva4\xDCV[` \x02` \x01\x01Q\x81T\x81\x10a\x0F\x8EWa\x0F\x8Ea4\xDCV[`\0\x91\x82R` \x80\x83 \x91\x90\x91\x01T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82R\x91\x81\x01\x92\x90\x92R\x01`@Q\x80\x91\x03\x90\xA2\x81T\x82\x90a\x0F\xCE\x90`\x01\x90a5\xB0V[\x81T\x81\x10a\x0F\xDEWa\x0F\xDEa4\xDCV[\x90`\0R` `\0 \x01\x82\x86\x83\x81Q\x81\x10a\x0F\xFBWa\x0F\xFBa4\xDCV[` \x02` \x01\x01Q\x81T\x81\x10a\x10\x13Wa\x10\x13a4\xDCV[`\0\x91\x82R` \x90\x91 \x82T\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x17\x81U\x91T`\x01`\x01``\x1B\x03`\x01`\xA0\x1B\x91\x82\x90\x04\x16\x02\x17\x90U\x81T\x82\x90\x80a\x10fWa\x10fa5\xC7V[`\0\x82\x81R` \x81 \x82\x01`\0\x19\x90\x81\x01\x91\x90\x91U\x01\x90U\x80a\x10\x88\x81a5#V[\x91PPa\x0E\xC0V[PPPPPPV[`\x003`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x10\xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06z\x90a5>V[`\0\x80[\x83\x81\x10\x15a\x06\x8FW`\0\x85\x85\x83\x81\x81\x10a\x11\x02Wa\x11\x02a4\xDCV[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x01` R`@\x90 T\x90\x92P\x15\x15\x90Pa\x11\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FStakeRegistry.updateOperatorStak`D\x82\x01R\x7Fe: quorum does not exist\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06zV[`\0\x80a\x11\x9F\x83\x8Ba\x1E\xA6V[\x91P\x91P\x80a\x11\xC1W`\0\x91P`\x01`\xFF\x84\x16\x1B`\x01`\x01`\xC0\x1B\x03\x86\x16\x17\x94P[`\0a\x11\xCE\x8A\x85\x85a \\V[\x90Pa\x11\xDA\x84\x82a\x1D,V[PPPPP\x80\x80a\x11\xEA\x90a5#V[\x91PPa\x10\xE6V[`\0a\x11\xFF\x84\x84\x84a \\V[\x90P[\x93\x92PPPV[`\0a\x12\x02\x83\x83a\"\xDCV[```\0\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x121Wa\x121a.\xD1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x12ZW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83\x81\x10\x15a\x14=W`\0\x85\x85\x83\x81\x81\x10a\x12|Wa\x12|a4\xDCV[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x01` R`@\x81 \x80T\x92\x94Pc\xFF\xFF\xFF\xFF\x8B\x16\x93P\x91a\x12\xADWa\x12\xADa4\xDCV[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a\x13YW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`[`$\x82\x01R\x7FStakeRegistry.getTotalStakeIndic`D\x82\x01R\x7FesAtBlockNumber: quorum has no s`d\x82\x01R\x7Ftake history at blockNumber\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06zV[`\xFF\x81\x16`\0\x90\x81R`\x01` R`@\x81 T\x90[\x81\x81\x10\x15a\x14'W`\xFF\x83\x16`\0\x90\x81R`\x01` \x81\x90R`@\x90\x91 c\xFF\xFF\xFF\xFF\x8B\x16\x91a\x13\x9D\x84\x86a5\xB0V[a\x13\xA7\x91\x90a5\xB0V[\x81T\x81\x10a\x13\xB7Wa\x13\xB7a4\xDCV[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a\x14\x15W`\x01a\x13\xDA\x82\x84a5\xB0V[a\x13\xE4\x91\x90a5\xB0V[\x85\x85\x81Q\x81\x10a\x13\xF6Wa\x13\xF6a4\xDCV[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPPa\x14'V[\x80a\x14\x1F\x81a5#V[\x91PPa\x13nV[PPP\x80\x80a\x145\x90a5#V[\x91PPa\x12`V[P\x94\x93PPPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R\x85\x82R`\x02\x81R\x83\x82 `\xFF\x88\x16\x83R\x90R\x91\x90\x91 \x80T\x83\x90\x81\x10a\x14\x8BWa\x14\x8Ba4\xDCV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x93\x83\x01\x93\x90\x93R`\x01`@\x1B\x90\x92\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x94\x93PPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\xFF\x83\x16`\0\x90\x81R`\x03` R`@\x90 \x80T\x83\x90\x81\x10a\x15\x16Wa\x15\x16a4\xDCV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R`\xFF\x86\x16\x82R`\x01\x90R\x91\x90\x91 \x80T\x83\x90\x81\x10a\x15\x95Wa\x15\x95a4\xDCV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x93\x83\x01\x93\x90\x93R`\x01`@\x1B\x90\x92\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16i\x91\x90a4CV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x16\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06z\x90a4`V[\x81a\x16\xB5\x81`\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x15\x15\x90V[a\x16\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06z\x90a3\xF2V[a\x05\xFB\x83\x83a#\nV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x17#W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06z\x90a5>V[`\0[\x81\x81\x10\x15a\x18\x02W`\0\x83\x83\x83\x81\x81\x10a\x17BWa\x17Ba4\xDCV[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x01` R`@\x90 T\x90\x92P\x15\x15\x90Pa\x17\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FStakeRegistry.deregisterOperator`D\x82\x01R\x7F: quorum does not exist\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06zV[`\0a\x17\xE0\x86\x83`\0a \\V[\x90Pa\x17\xEC\x82\x82a\x1D,V[PPP\x80\x80a\x17\xFA\x90a5#V[\x91PPa\x17&V[PPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18fW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\x8A\x91\x90a4CV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x18\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06z\x90a4`V[\x81a\x18\xD6\x81`\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x15\x15\x90V[a\x18\xF2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06z\x90a3\xF2V[a\x05\xFB\x83\x83a#sV[`\xFF\x83\x16`\0\x90\x81R`\x01` R`@\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x19#Wa\x19#a4\xDCV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90Pa\rA\x81\x85a'WV[`\xFF\x81\x16`\0\x90\x81R`\x01` \x81\x90R`@\x82 \x80T\x90\x91a\x19\x9B\x91a5\xB0V[\x81T\x81\x10a\x19\xABWa\x19\xABa4\xDCV[`\0\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x92\x91PPV[`\0a\x11\xFF\x84\x84\x84a(\xE2V[`\0\x82\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x88\x16\x84R\x90\x91R\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x1A\x0BWa\x1A\x0Ba4\xDCV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90Pa\x1Ab\x81\x86a'WV[`@\x01Q\x95\x94PPPPPV[`\0a\x12\x02\x83\x83a+\x80V[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R\x86\x82R`\x02\x81R\x84\x82 `\xFF\x87\x16\x83R\x81R\x84\x82 T\x85Q\x93\x84\x01\x86R\x82\x84R\x90\x83\x01\x82\x90R\x93\x82\x01R\x90\x91\x90\x81a\x1A\xD4W\x91Pa\r.\x90PV[`\0\x85\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x88\x16\x84R\x90\x91R\x90 a\x1A\xFB`\x01\x84a5\xB0V[\x81T\x81\x10a\x1B\x0BWa\x1B\x0Ba4\xDCV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x92Pa\r.\x91PPV[`\0\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 a\x1B\x87\x85\x85\x85a(\xE2V[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x1B\x9DWa\x1B\x9Da4\xDCV[`\0\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x94\x93PPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x1C\tW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06z\x90a5>V[`\xFF\x83\x16`\0\x90\x81R`\x01` R`@\x90 T\x15a\x1C\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FStakeRegistry.initializeQuorum: `D\x82\x01Rtquorum already exists`X\x1B`d\x82\x01R`\x84\x01a\x06zV[a\x1C\x91\x83\x82a#sV[a\x1C\x9B\x83\x83a#\nV[PP`\xFF\x16`\0\x90\x81R`\x01` \x81\x81R`@\x80\x84 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x87\x81R\x93\x82\x01\x87\x81R\x83T\x96\x87\x01\x84U\x92\x87R\x93\x90\x95 \x94Q\x94\x90\x93\x01\x80T\x91Q\x93Q`\x01`\x01``\x1B\x03\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x94\x84\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x95\x90\x93\x16\x94\x90\x94\x17\x17\x91\x90\x91\x16\x17\x90UV[`\xFF\x82\x16`\0\x90\x81R`\x01` \x81\x90R`@\x82 \x80T\x91\x83\x91\x90a\x1DP\x90\x84a5\xB0V[\x81T\x81\x10a\x1D`Wa\x1D`a4\xDCV[\x90`\0R` `\0 \x01\x90P\x83`\0\x14\x15a\x1D\x8FWT`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x91Pa\r.\x90PV[\x80T`\0\x90a\x1D\xAE\x90`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x86a\"\xDCV[\x82T\x90\x91PCc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14\x15a\x1D\xEBW\x81T`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x16`\x01`@\x1B`\x01`\x01``\x1B\x03\x83\x16\x02\x17\x82Ua\x1E\x9DV[\x81Tc\xFF\xFF\xFF\xFFC\x81\x16`\x01` \x1B\x81\x81\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x94\x16\x93\x90\x93\x17\x85U`\xFF\x89\x16`\0\x90\x81R`\x01` \x81\x81R`@\x80\x84 \x81Q``\x81\x01\x83R\x95\x86R\x85\x83\x01\x85\x81R`\x01`\x01``\x1B\x03\x80\x8B\x16\x93\x88\x01\x93\x84R\x82T\x95\x86\x01\x83U\x91\x86R\x92\x90\x94 \x94Q\x94\x90\x92\x01\x80T\x91Q\x92Q\x90\x93\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x92\x86\x16\x90\x96\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x91\x16\x93\x90\x94\x16\x92\x90\x92\x17\x92\x90\x92\x17\x16\x91\x90\x91\x17\x90U[\x95\x94PPPPPV[`\0\x80`\0\x80a\x1E\xC5\x86`\xFF\x16`\0\x90\x81R`\x03` R`@\x90 T\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x90\x91P`\0[\x82\x81\x10\x15a )W`\xFF\x88\x16`\0\x90\x81R`\x03` R`@\x90 \x80T\x82\x90\x81\x10a\x1F\x0BWa\x1F\x0Ba4\xDCV[`\0\x91\x82R` \x80\x83 `@\x80Q\x80\x82\x01\x82R\x93\x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x80\x82\x16\x80\x86R`\x01`\xA0\x1B\x90\x92\x04`\x01`\x01``\x1B\x03\x16\x93\x85\x01\x93\x90\x93R\x90Qcw\x8EU\xF3`\xE0\x1B\x81R\x8B\x83\x16`\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x91\x94P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cw\x8EU\xF3\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\xB1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\xD5\x91\x90a5\xDDV[\x90P\x80\x15a \x16Wg\r\xE0\xB6\xB3\xA7d\0\0\x83` \x01Q`\x01`\x01``\x1B\x03\x16\x82a\x1F\xFF\x91\x90a5\xF6V[a \t\x91\x90a6\x15V[a \x13\x90\x86a67V[\x94P[P\x80a !\x81a5#V[\x91PPa\x1E\xDFV[PPP`\xFF\x85\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90\x92P`\x01`\x01``\x1B\x03\x90\x81\x16\x90\x83\x16\x10\x15\x90P[\x92P\x92\x90PV[`\0\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 T\x81\x90\x80a! W`\0\x86\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x82R\x80\x83 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x86\x81R`\x01`\x01``\x1B\x03\x80\x8C\x16\x95\x84\x01\x95\x86R\x84T`\x01\x81\x01\x86U\x94\x88R\x95\x90\x96 \x91Q\x91\x90\x92\x01\x80T\x95Q\x93Q\x90\x94\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x93\x83\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x96\x16\x91\x90\x92\x16\x17\x93\x90\x93\x17\x16\x91\x90\x91\x17\x90Ua\"\x82V[`\0\x86\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x90\x91R\x81 a!G`\x01\x84a5\xB0V[\x81T\x81\x10a!WWa!Wa4\xDCV[`\0\x91\x82R` \x90\x91 \x01\x80T`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x91\x04\x81\x16\x94P\x90\x91P\x85\x16\x83\x14\x15a!\x90W`\0\x93PPPPa\x12\x02V[\x80TCc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14\x15a!\xCAW\x80T`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x16`\x01`@\x1B`\x01`\x01``\x1B\x03\x87\x16\x02\x17\x81Ua\"\x80V[\x80Tg\xFF\xFF\xFF\xFF\0\0\0\0\x19\x16`\x01` \x1BCc\xFF\xFF\xFF\xFF\x90\x81\x16\x82\x81\x02\x93\x90\x93\x17\x84U`\0\x8A\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x8D\x16\x84R\x82R\x80\x83 \x81Q``\x81\x01\x83R\x96\x87R\x86\x83\x01\x84\x81R`\x01`\x01``\x1B\x03\x8D\x81\x16\x93\x89\x01\x93\x84R\x82T`\x01\x81\x01\x84U\x92\x86R\x93\x90\x94 \x96Q\x96\x01\x80T\x93Q\x91Q\x96\x85\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x94\x16\x93\x90\x93\x17\x93\x16\x90\x93\x02\x91\x90\x91\x17`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x16`\x01`@\x1B\x93\x90\x92\x16\x92\x90\x92\x02\x17\x90U[P[`@\x80Q`\xFF\x87\x16\x81R`\x01`\x01``\x1B\x03\x86\x16` \x82\x01R\x87\x91\x7F/R}R~\x95\xD8\xFE@\xAE\xC5Swt;\xB7y\x08}\xA3\xF6\xD0\xD0\x8F\x12\xE3dD\xDAb2}\x91\x01`@Q\x80\x91\x03\x90\xA2a\"\xD2\x82\x85a+\x80V[\x96\x95PPPPPPV[`\0\x80\x82\x12\x15a#\0Wa\"\xEF\x82a6bV[a\"\xF9\x90\x84a6\x7FV[\x90Pa\r.V[a\"\xF9\x82\x84a67V[`\xFF\x82\x16`\0\x81\x81R` \x81\x81R`@\x91\x82\x90 \x80Tk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01``\x1B\x03\x86\x16\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F&\xEE\xCF\xF2\xB7\x0B\nq\x10O\xF4\xD9@\xBAqb\xD2:\x95\xC2Hw\x1F\xC4\x87\xA7\xBE\x17\xA5\x96\xB3\xCF\x91\x01`@Q\x80\x91\x03\x90\xA2PPV[`\0\x81Q\x11a#\xD8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R`\0\x80Q` a6\xFF\x839\x81Q\x91R`D\x82\x01R\x7F: no strategies provided\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06zV[\x80Q`\xFF\x83\x16`\0\x90\x81R`\x03` \x90\x81R`@\x90\x91 T\x90a#\xFB\x83\x83a6\xA7V[\x11\x15a$kW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R`\0\x80Q` a6\xFF\x839\x81Q\x91R`D\x82\x01R\x7F: exceed MAX_WEIGHING_FUNCTION_L`d\x82\x01Rd\x08\xA9\xC8\xEA\x89`\xDB\x1B`\x84\x82\x01R`\xA4\x01a\x06zV[`\0[\x82\x81\x10\x15a'PW`\0[a$\x83\x82\x84a6\xA7V[\x81\x10\x15a%dW\x84\x82\x81Q\x81\x10a$\x9CWa$\x9Ca4\xDCV[` \x02` \x01\x01Q`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x03`\0\x88`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x82\x81T\x81\x10a$\xDBWa$\xDBa4\xDCV[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a%RW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R`\0\x80Q` a6\xFF\x839\x81Q\x91R`D\x82\x01R\x7F: cannot add same strategy 2x\0\0\0`d\x82\x01R`\x84\x01a\x06zV[\x80a%\\\x81a5#V[\x91PPa$yV[P`\0\x84\x82\x81Q\x81\x10a%yWa%ya4\xDCV[` \x02` \x01\x01Q` \x01Q`\x01`\x01``\x1B\x03\x16\x11a%\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`F`$\x82\x01R`\0\x80Q` a6\xFF\x839\x81Q\x91R`D\x82\x01R\x7F: cannot add strategy with zero `d\x82\x01Re\x1D\xD9ZY\xDA\x1D`\xD2\x1B`\x84\x82\x01R`\xA4\x01a\x06zV[`\xFF\x85\x16`\0\x90\x81R`\x03` R`@\x90 \x84Q\x85\x90\x83\x90\x81\x10a&$Wa&$a4\xDCV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82T`\x01\x81\x01\x84U`\0\x93\x84R\x92\x82\x90 \x81Q\x91\x90\x92\x01Q`\x01`\x01``\x1B\x03\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x91\x01U\x83Q`\xFF\x86\x16\x90\x7F\x10V^V\xCA\xCB\xF3.\xCA&yE\xF0T\xFE\xC0.Yu\x002\xD1\x13\xD30!\x82\xAD\x96\x7FT\x04\x90\x86\x90\x84\x90\x81\x10a&\xA1Wa&\xA1a4\xDCV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQ`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x01`@Q\x80\x91\x03\x90\xA2\x84`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x85\x83\x81Q\x81\x10a&\xFEWa&\xFEa4\xDCV[` \x02` \x01\x01Q`\0\x01Q\x86\x84\x81Q\x81\x10a'\x1CWa'\x1Ca4\xDCV[` \x02` \x01\x01Q` \x01Q`@Qa'6\x92\x91\x90a+\xF3V[`@Q\x80\x91\x03\x90\xA2\x80a'H\x81a5#V[\x91PPa$nV[PPPPPV[\x81`\0\x01Qc\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a(\x03W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R```$\x82\x01R\x7FStakeRegistry._validateOperatorS`D\x82\x01R\x7FtakeAtBlockNumber: operatorStake`d\x82\x01R\x7FUpdate is from after blockNumber`\x84\x82\x01R`\xA4\x01a\x06zV[` \x82\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x80a()WP\x81` \x01Qc\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10[a(\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`t`$\x82\x01R\x7FStakeRegistry._validateOperatorS`D\x82\x01R\x7FtakeAtBlockNumber: there is a ne`d\x82\x01R\x7Fwer operatorStakeUpdate availabl`\x84\x82\x01Rs2\x9012\xB37\xB92\x90167\xB1\xB5\xA7:\xB6\xB12\xB9`a\x1B`\xA4\x82\x01R`\xC4\x01a\x06zV[PPV[`\0\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 T\x81[\x81\x81\x10\x15a*\xBBW`\0\x86\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x90\x91R\x90 c\xFF\xFF\xFF\xFF\x85\x16\x90`\x01a)9\x84\x86a5\xB0V[a)C\x91\x90a5\xB0V[\x81T\x81\x10a)SWa)Sa4\xDCV[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a*\xA9W`\0\x86\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x90\x91R\x81 `\x01a)\x92\x84\x86a5\xB0V[a)\x9C\x91\x90a5\xB0V[\x81T\x81\x10a)\xACWa)\xACa4\xDCV[`\0\x91\x82R` \x90\x91 \x01T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x90P\x80\x15\x80a)\xDFWP\x84c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x11[a*\x89W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`i`$\x82\x01R\x7FStakeRegistry._getStakeUpdateInd`D\x82\x01R\x7FexForOperatorAtBlockNumber: oper`d\x82\x01R\x7FatorId has no stake update at bl`\x84\x82\x01Rh7\xB1\xB5\xA7:\xB6\xB12\xB9`\xB9\x1B`\xA4\x82\x01R`\xC4\x01a\x06zV[`\x01a*\x95\x83\x85a5\xB0V[a*\x9F\x91\x90a5\xB0V[\x93PPPPa\x12\x02V[\x80a*\xB3\x81a5#V[\x91PPa)\x01V[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x81`$\x82\x01R\x7FStakeRegistry._getStakeUpdateInd`D\x82\x01R\x7FexForOperatorAtBlockNumber: no s`d\x82\x01R\x7Ftake update found for operatorId`\x84\x82\x01R\x7F and quorumNumber at block numbe`\xA4\x82\x01R`9`\xF9\x1B`\xC4\x82\x01R`\xE4\x01a\x06zV[`\0a\x12\x02`\x01`\x01``\x1B\x03\x80\x85\x16\x90\x84\x16a6\xBFV[\x805`\xFF\x81\x16\x81\x14a+\xA9W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a+\xC1W`\0\x80\xFD[a+\xCA\x83a+\x98V[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a+\xEAW`\0\x80\xFD[a\x12\x02\x82a+\x98V[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R`\x01`\x01``\x1B\x03\x16` \x82\x01R`@\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a,*W`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a,@W`\0\x80\xFD[a,I\x83a+\x98V[\x91P` \x83\x015a,Y\x81a,\x15V[\x80\x91PP\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a,vW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a,\x8DW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a UW`\0\x80\xFD[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a,\xC0W`\0\x80\xFD[a,\xC9\x86a+\x98V[\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a,\xE5W`\0\x80\xFD[a,\xF1\x89\x83\x8A\x01a,dV[\x90\x96P\x94P`@\x88\x015\x91P\x80\x82\x11\x15a-\nW`\0\x80\xFD[Pa-\x17\x88\x82\x89\x01a,dV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0\x80\x83`\x1F\x84\x01\x12a-:W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a-QW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a UW`\0\x80\xFD[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a-\x7FW`\0\x80\xFD[\x845a-\x8A\x81a,\x15V[\x93P` \x85\x015\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a-\xACW`\0\x80\xFD[a-\xB8\x87\x82\x88\x01a-(V[\x95\x98\x94\x97P\x95PPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a-\xFDW\x81Q`\x01`\x01``\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a-\xD8V[P\x94\x95\x94PPPPPV[`@\x81R`\0a.\x1B`@\x83\x01\x85a-\xC4V[\x82\x81\x03` \x84\x01Ra\x1E\x9D\x81\x85a-\xC4V[`\0\x80`@\x83\x85\x03\x12\x15a.@W`\0\x80\xFD[\x825\x91Pa.P` \x84\x01a+\x98V[\x90P\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a.\xC5Wa.\xB2\x83\x85Qc\xFF\xFF\xFF\xFF\x80\x82Q\x16\x83R\x80` \x83\x01Q\x16` \x84\x01RP`\x01`\x01``\x1B\x03`@\x82\x01Q\x16`@\x83\x01RPPV[\x92\x84\x01\x92``\x92\x90\x92\x01\x91`\x01\x01a.uV[P\x90\x96\x95PPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a/\tWa/\ta.\xD1V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a/7Wa/7a.\xD1V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a/XWa/Xa.\xD1V[P`\x05\x1B` \x01\x90V[`\0\x80`@\x83\x85\x03\x12\x15a/uW`\0\x80\xFD[a/~\x83a+\x98V[\x91P` \x80\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a/\x9AW`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a/\xABW`\0\x80\xFD[\x805a/\xBEa/\xB9\x82a/?V[a/\x0FV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x88\x83\x11\x15a/\xDDW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a/\xFBW\x835\x82R\x92\x84\x01\x92\x90\x84\x01\x90a/\xE2V[\x80\x95PPPPPP\x92P\x92\x90PV[\x805`\x01`\x01``\x1B\x03\x81\x16\x81\x14a+\xA9W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a06W`\0\x80\xFD[\x835\x92Pa0F` \x85\x01a+\x98V[\x91Pa0T`@\x85\x01a0\nV[\x90P\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a0pW`\0\x80\xFD[a+\xCA\x83a0\nV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a+\xA9W`\0\x80\xFD[`\0\x80`\0`@\x84\x86\x03\x12\x15a0\xA2W`\0\x80\xFD[a0\xAB\x84a0yV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a0\xC6W`\0\x80\xFD[a0\xD2\x86\x82\x87\x01a-(V[\x94\x97\x90\x96P\x93\x94PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a.\xC5W\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a0\xFBV[`\0\x80`\0``\x84\x86\x03\x12\x15a12W`\0\x80\xFD[a1;\x84a+\x98V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[\x81Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x80\x84\x01Q\x90\x91\x16\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01``\x1B\x03\x16\x90\x82\x01R``\x81\x01a\r.V[`\0\x80`@\x83\x85\x03\x12\x15a1\x98W`\0\x80\xFD[a1\xA1\x83a+\x98V[\x91Pa.P` \x84\x01a0\nV[`\0\x80`\0`@\x84\x86\x03\x12\x15a1\xC4W`\0\x80\xFD[\x835\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a0\xC6W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a1\xF2W`\0\x80\xFD[\x815` a2\x02a/\xB9\x83a/?V[\x82\x81R`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a2!W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a2pW`@\x81\x89\x03\x12\x15a2>W`\0\x80\x81\xFD[a2Fa.\xE7V[\x815a2Q\x81a,\x15V[\x81Ra2^\x82\x86\x01a0\nV[\x81\x86\x01R\x83R\x91\x83\x01\x91`@\x01a2%V[P\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a2\x8EW`\0\x80\xFD[a2\x97\x83a+\x98V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a2\xB2W`\0\x80\xFD[a2\xBE\x85\x82\x86\x01a1\xE1V[\x91PP\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a2\xDDW`\0\x80\xFD[a2\xE6\x84a+\x98V[\x92Pa2\xF4` \x85\x01a0yV[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a3\x19W`\0\x80\xFD[\x835\x92Pa3)` \x85\x01a+\x98V[\x91Pa0T`@\x85\x01a0yV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a3MW`\0\x80\xFD[a3V\x85a+\x98V[\x93Pa3d` \x86\x01a0yV[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[`\0\x80`@\x83\x85\x03\x12\x15a3\x8CW`\0\x80\xFD[a1\xA1\x83a0\nV[`\0\x80`\0``\x84\x86\x03\x12\x15a3\xAAW`\0\x80\xFD[a3\xB3\x84a+\x98V[\x92Pa3\xC1` \x85\x01a0\nV[\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a3\xDCW`\0\x80\xFD[a3\xE8\x86\x82\x87\x01a1\xE1V[\x91PP\x92P\x92P\x92V[` \x80\x82R`1\x90\x82\x01R\x7FStakeRegistry.quorumExists: quor`@\x82\x01Rp\x1D[H\x19\x1B\xD9\\\xC8\x1B\x9B\xDD\x08\x19^\x1A\\\xDD`z\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15a4UW`\0\x80\xFD[\x81Qa\x12\x02\x81a,\x15V[` \x80\x82R`V\x90\x82\x01R\x7FStakeRegistry.onlyCoordinatorOwn`@\x82\x01R\x7Fer: caller is not the owner of t``\x82\x01Ru42\x9092\xB3\xB4\xB9\xBA9<\xA1\xB7\xB7\xB924\xB70\xBA7\xB9`Q\x1B`\x80\x82\x01R`\xA0\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a5\x04W`\0\x80\xFD[a\x12\x02\x82a0\nV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15a57Wa57a5\rV[P`\x01\x01\x90V[` \x80\x82R`L\x90\x82\x01R\x7FStakeRegistry.onlyRegistryCoordi`@\x82\x01R\x7Fnator: caller is not the Registr``\x82\x01Rk<\xA1\xB7\xB7\xB924\xB70\xBA7\xB9`\xA1\x1B`\x80\x82\x01R`\xA0\x01\x90V[`\0\x82\x82\x10\x15a5\xC2Wa5\xC2a5\rV[P\x03\x90V[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a5\xEFW`\0\x80\xFD[PQ\x91\x90PV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a6\x10Wa6\x10a5\rV[P\x02\x90V[`\0\x82a62WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a6YWa6Ya5\rV[\x01\x94\x93PPPPV[`\0`\x01`\xFF\x1B\x82\x14\x15a6xWa6xa5\rV[P`\0\x03\x90V[`\0`\x01`\x01``\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a6\x9FWa6\x9Fa5\rV[\x03\x93\x92PPPV[`\0\x82\x19\x82\x11\x15a6\xBAWa6\xBAa5\rV[P\x01\x90V[`\0\x80\x83\x12\x80\x15`\x01`\xFF\x1B\x85\x01\x84\x12\x16\x15a6\xDDWa6\xDDa5\rV[`\x01`\x01`\xFF\x1B\x03\x84\x01\x83\x13\x81\x16\x15a6\xF8Wa6\xF8a5\rV[PP\x03\x90V\xFEStakeRegistry._addStrategyParams\xA2dipfsX\"\x12 \x08v\x88D\x84\xC2\xAA\xC0\xFC\xDD\xC8\xC3\x0C~2\xB6G\xD3\xB4\xF4\xB5\xBB\xCA\xAB\xB7\xAB\x8Eg\xA6\x82\x96\x92dsolcC\0\x08\x0C\x003";
    /// The bytecode of the contract.
    pub static STAKEREGISTRYHARNESS_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\x06W`\x005`\xE0\x1C\x80c\x81\xC0u\x02\x11a\x01\x1AW\x80c\xC8)LV\x11a\0\xADW\x80c\xF2\xBE\x94\xAE\x11a\0|W\x80c\xF2\xBE\x94\xAE\x14a\x05\x92W\x80c\xF5\tU\x1A\x14a\x05\xA5W\x80c\xF8Q\xE1\x98\x14a\x05\xB8W\x80c\xFA(\xC6'\x14a\x05\xCBW\x80c\xFFiJw\x14a\x05\xDEW`\0\x80\xFD[\x80c\xC8)LV\x14a\x05\x1DW\x80c\xD5\xEC\xCC\x05\x14a\x050W\x80c\xDD\x98F\xB9\x14a\x05CW\x80c\xDF\\\xF7#\x14a\x05kW`\0\x80\xFD[\x80c\xBC\x9A@\xC3\x11a\0\xE9W\x80c\xBC\x9A@\xC3\x14a\x04\xBBW\x80c\xBD)\xB8\xCD\x14a\x04\xCEW\x80c\xC4gx\xA5\x14a\x04\xE1W\x80c\xC6\x01R}\x14a\x05\nW`\0\x80\xFD[\x80c\x81\xC0u\x02\x14a\x04(W\x80c\xACk\xFB\x03\x14a\x04HW\x80c\xAD\xC8\x04\xDA\x14a\x04hW\x80c\xB6\x90Kx\x14a\x04\xA8W`\0\x80\xFD[\x80cK\xD2n\t\x11a\x01\x9DW\x80cf\xAC\xFE\xFE\x11a\x01lW\x80cf\xAC\xFE\xFE\x14a\x03~W\x80cm\x14\xA9\x87\x14a\x03\xA9W\x80ctELm\x14a\x03\xE8W\x80c|\x17#G\x14a\x03\xFBW\x80c\x7FB\x98\"\x14a\x04\x15W`\0\x80\xFD[\x80cK\xD2n\t\x14a\x03\x19W\x80cT\x01\xED'\x14a\x03IW\x80c^Zgu\x14a\x03\\W\x80c_\x1F-w\x14a\x03kW`\0\x80\xFD[\x80c \xB6b\x98\x11a\x01\xD9W\x80c \xB6b\x98\x14a\x02\xA2W\x80c%PGw\x14a\x02\xB5W\x80c,\xD9Y@\x14a\x02\xD6W\x80c<\xA5\xA5\xF5\x14a\x02\xF6W`\0\x80\xFD[\x80c\x03\x90\xA4\xD5\x14a\x02\x0BW\x80c\x04\x91\xB4\x1C\x14a\x02 W\x80c\x08s$a\x14a\x02VW\x80c\x1F\x9Bt\xE0\x14a\x02wW[`\0\x80\xFD[a\x02\x1Ea\x02\x196`\x04a+\xAEV[a\x05\xF1V[\0[a\x02Ca\x02.6`\x04a+\xD8V[`\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02ia\x02d6`\x04a+\xAEV[a\x06\0V[`@Qa\x02M\x92\x91\x90a+\xF3V[a\x02\x8Aa\x02\x856`\x04a,-V[a\x06IV[`@Q`\x01`\x01``\x1B\x03\x90\x91\x16\x81R` \x01a\x02MV[a\x02\x1Ea\x02\xB06`\x04a,\xA8V[a\x06\x99V[a\x02\xC8a\x02\xC36`\x04a-iV[a\t\xCAV[`@Qa\x02M\x92\x91\x90a.\x08V[a\x02\xE9a\x02\xE46`\x04a.-V[a\x0C\x95V[`@Qa\x02M\x91\x90a.YV[a\x02Ca\x03\x046`\x04a+\xD8V[`\xFF\x16`\0\x90\x81R`\x03` R`@\x90 T\x90V[a\x02Ca\x03'6`\x04a.-V[`\0\x91\x82R`\x02` \x90\x81R`@\x80\x84 `\xFF\x93\x90\x93\x16\x84R\x91\x90R\x90 T\x90V[a\x02\x8Aa\x03W6`\x04a.-V[a\r4V[a\x02Cg\r\xE0\xB6\xB3\xA7d\0\0\x81V[a\x02\x1Ea\x03y6`\x04a/bV[a\rMV[a\x03\x91a\x03\x8C6`\x04a-iV[a\x10\x98V[`@Q`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x81R` \x01a\x02MV[a\x03\xD0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02MV[a\x02Ca\x03\xF66`\x04a0!V[a\x11\xF2V[a\x04\x03` \x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02MV[a\x02\x8Aa\x04#6`\x04a0]V[a\x12\tV[a\x04;a\x0466`\x04a0\x8DV[a\x12\x15V[`@Qa\x02M\x91\x90a0\xDFV[a\x04[a\x04V6`\x04a1\x1DV[a\x14FV[`@Qa\x02M\x91\x90a1PV[a\x04{a\x04v6`\x04a+\xAEV[a\x14\xDEV[`@\x80Q\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x92\x83\x01Q`\x01`\x01``\x1B\x03\x16\x92\x81\x01\x92\x90\x92R\x01a\x02MV[a\x04[a\x04\xB66`\x04a+\xAEV[a\x15XV[a\x02\x1Ea\x04\xC96`\x04a1\x85V[a\x15\xE7V[a\x02\x1Ea\x04\xDC6`\x04a1\xAFV[a\x16\xDBV[a\x02\x8Aa\x04\xEF6`\x04a+\xD8V[`\0` \x81\x90R\x90\x81R`@\x90 T`\x01`\x01``\x1B\x03\x16\x81V[a\x02\x1Ea\x05\x186`\x04a2{V[a\x18\x08V[a\x02\x8Aa\x05+6`\x04a2\xC8V[a\x18\xFCV[a\x02\x8Aa\x05>6`\x04a+\xD8V[a\x19zV[a\x05Va\x05Q6`\x04a3\x04V[a\x19\xCDV[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02MV[a\x03\xD0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\x8Aa\x05\xA06`\x04a37V[a\x19\xDAV[a\x02Ca\x05\xB36`\x04a3yV[a\x1AoV[a\x04[a\x05\xC66`\x04a.-V[a\x1A{V[a\x02\x8Aa\x05\xD96`\x04a3\x04V[a\x1B`V[a\x02\x1Ea\x05\xEC6`\x04a3\x95V[a\x1B\xC1V[a\x05\xFB\x82\x82a\x1D,V[PPPV[`\x03` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x06\x1CW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x92P`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90P\x82V[`\xFF\x82\x16`\0\x90\x81R`\x01` R`@\x81 T\x83\x90a\x06\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06z\x90a3\xF2V[`@Q\x80\x91\x03\x90\xFD[`\0a\x06\x8F\x85\x85a\x1E\xA6V[P\x95\x94PPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xF7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x1B\x91\x90a4CV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x07KW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06z\x90a4`V[\x84a\x07g\x81`\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x15\x15\x90V[a\x07\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06z\x90a3\xF2V[\x83\x80a\x07\xF9W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FStakeRegistry.modifyStrategyPara`D\x82\x01R\x7Fms: no strategy indices provided`d\x82\x01R`\x84\x01a\x06zV[\x82\x81\x14a\x08nW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FStakeRegistry.modifyStrategyPara`D\x82\x01R\x7Fms: input length mismatch\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06zV[`\xFF\x87\x16`\0\x90\x81R`\x03` R`@\x81 \x90[\x82\x81\x10\x15a\t\xBFW\x85\x85\x82\x81\x81\x10a\x08\x9CWa\x08\x9Ca4\xDCV[\x90P` \x02\x01` \x81\x01\x90a\x08\xB1\x91\x90a4\xF2V[\x82\x89\x89\x84\x81\x81\x10a\x08\xC4Wa\x08\xC4a4\xDCV[\x90P` \x02\x015\x81T\x81\x10a\x08\xDBWa\x08\xDBa4\xDCV[\x90`\0R` `\0 \x01`\0\x01`\x14a\x01\0\n\x81T\x81`\x01`\x01``\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01``\x1B\x03\x16\x02\x17\x90UP\x88`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x83\x8A\x8A\x85\x81\x81\x10a\tDWa\tDa4\xDCV[\x90P` \x02\x015\x81T\x81\x10a\t[Wa\t[a4\xDCV[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x88\x88\x85\x81\x81\x10a\t\x82Wa\t\x82a4\xDCV[\x90P` \x02\x01` \x81\x01\x90a\t\x97\x91\x90a4\xF2V[`@Qa\t\xA5\x92\x91\x90a+\xF3V[`@Q\x80\x91\x03\x90\xA2\x80a\t\xB7\x81a5#V[\x91PPa\x08\x82V[PPPPPPPPPV[``\x803`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\n\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06z\x90a5>V[`\0\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\n/Wa\n/a.\xD1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\nXW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x84`\x01`\x01`@\x1B\x03\x81\x11\x15a\nuWa\nua.\xD1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\n\x9EW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x85\x81\x10\x15a\x0C\x87W`\0\x87\x87\x83\x81\x81\x10a\n\xC0Wa\n\xC0a4\xDCV[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x01` R`@\x90 T\x90\x92P\x15\x15\x90Pa\x0BHW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FStakeRegistry.registerOperator: `D\x82\x01Rt\x1C][\xDC\x9D[H\x19\x1B\xD9\\\xC8\x1B\x9B\xDD\x08\x19^\x1A\\\xDD`Z\x1B`d\x82\x01R`\x84\x01a\x06zV[`\0\x80a\x0BU\x83\x8Da\x1E\xA6V[\x91P\x91P\x80a\x0B\xF2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`[`$\x82\x01R\x7FStakeRegistry.registerOperator: `D\x82\x01R\x7FOperator does not meet minimum s`d\x82\x01R\x7Ftake requirement for quorum\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06zV[`\0a\x0B\xFF\x8C\x85\x85a \\V[\x90P\x82\x87\x86\x81Q\x81\x10a\x0C\x14Wa\x0C\x14a4\xDCV[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPPa\x0C>\x84\x82a\x1D,V[\x86\x86\x81Q\x81\x10a\x0CPWa\x0CPa4\xDCV[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPPPPPP\x80\x80a\x0C\x7F\x90a5#V[\x91PPa\n\xA4V[P\x90\x97\x90\x96P\x94PPPPPV[`\0\x82\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x85\x16\x84R\x82R\x80\x83 \x80T\x82Q\x81\x85\x02\x81\x01\x85\x01\x90\x93R\x80\x83R``\x94\x92\x93\x91\x92\x90\x91\x84\x01[\x82\x82\x10\x15a\r'W`\0\x84\x81R` \x90\x81\x90 `@\x80Q``\x81\x01\x82R\x91\x85\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x83\x85\x01R`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90\x82\x01R\x82R`\x01\x90\x92\x01\x91\x01a\x0C\xCEV[PPPP\x90P[\x92\x91PPV[`\0\x80a\rA\x84\x84a\x1A{V[`@\x01Q\x94\x93PPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xABW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xCF\x91\x90a4CV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\r\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06z\x90a4`V[\x81a\x0E\x1B\x81`\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x15\x15\x90V[a\x0E7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06z\x90a3\xF2V[\x81Q\x80a\x0E\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FStakeRegistry.removeStrategies: `D\x82\x01R\x7Fno indices to remove provided\0\0\0`d\x82\x01R`\x84\x01a\x06zV[`\xFF\x84\x16`\0\x90\x81R`\x03` R`@\x81 \x90[\x82\x81\x10\x15a\x10\x90W\x85`\xFF\x16\x7F1\xFA.,\xD2\x80\xC97^\x13\xFF\xCF=\x81\xE27\x81\0\x18n@X\xF8\xD3\xDD\xB6\x90\xB8-\xCD1\xF7\x83\x87\x84\x81Q\x81\x10a\x0F\0Wa\x0F\0a4\xDCV[` \x02` \x01\x01Q\x81T\x81\x10a\x0F\x18Wa\x0F\x18a4\xDCV[`\0\x91\x82R` \x91\x82\x90 \x01T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x01`@Q\x80\x91\x03\x90\xA2\x85`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x83\x87\x84\x81Q\x81\x10a\x0FvWa\x0Fva4\xDCV[` \x02` \x01\x01Q\x81T\x81\x10a\x0F\x8EWa\x0F\x8Ea4\xDCV[`\0\x91\x82R` \x80\x83 \x91\x90\x91\x01T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82R\x91\x81\x01\x92\x90\x92R\x01`@Q\x80\x91\x03\x90\xA2\x81T\x82\x90a\x0F\xCE\x90`\x01\x90a5\xB0V[\x81T\x81\x10a\x0F\xDEWa\x0F\xDEa4\xDCV[\x90`\0R` `\0 \x01\x82\x86\x83\x81Q\x81\x10a\x0F\xFBWa\x0F\xFBa4\xDCV[` \x02` \x01\x01Q\x81T\x81\x10a\x10\x13Wa\x10\x13a4\xDCV[`\0\x91\x82R` \x90\x91 \x82T\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x17\x81U\x91T`\x01`\x01``\x1B\x03`\x01`\xA0\x1B\x91\x82\x90\x04\x16\x02\x17\x90U\x81T\x82\x90\x80a\x10fWa\x10fa5\xC7V[`\0\x82\x81R` \x81 \x82\x01`\0\x19\x90\x81\x01\x91\x90\x91U\x01\x90U\x80a\x10\x88\x81a5#V[\x91PPa\x0E\xC0V[PPPPPPV[`\x003`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x10\xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06z\x90a5>V[`\0\x80[\x83\x81\x10\x15a\x06\x8FW`\0\x85\x85\x83\x81\x81\x10a\x11\x02Wa\x11\x02a4\xDCV[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x01` R`@\x90 T\x90\x92P\x15\x15\x90Pa\x11\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FStakeRegistry.updateOperatorStak`D\x82\x01R\x7Fe: quorum does not exist\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06zV[`\0\x80a\x11\x9F\x83\x8Ba\x1E\xA6V[\x91P\x91P\x80a\x11\xC1W`\0\x91P`\x01`\xFF\x84\x16\x1B`\x01`\x01`\xC0\x1B\x03\x86\x16\x17\x94P[`\0a\x11\xCE\x8A\x85\x85a \\V[\x90Pa\x11\xDA\x84\x82a\x1D,V[PPPPP\x80\x80a\x11\xEA\x90a5#V[\x91PPa\x10\xE6V[`\0a\x11\xFF\x84\x84\x84a \\V[\x90P[\x93\x92PPPV[`\0a\x12\x02\x83\x83a\"\xDCV[```\0\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x121Wa\x121a.\xD1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x12ZW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83\x81\x10\x15a\x14=W`\0\x85\x85\x83\x81\x81\x10a\x12|Wa\x12|a4\xDCV[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x01` R`@\x81 \x80T\x92\x94Pc\xFF\xFF\xFF\xFF\x8B\x16\x93P\x91a\x12\xADWa\x12\xADa4\xDCV[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a\x13YW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`[`$\x82\x01R\x7FStakeRegistry.getTotalStakeIndic`D\x82\x01R\x7FesAtBlockNumber: quorum has no s`d\x82\x01R\x7Ftake history at blockNumber\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06zV[`\xFF\x81\x16`\0\x90\x81R`\x01` R`@\x81 T\x90[\x81\x81\x10\x15a\x14'W`\xFF\x83\x16`\0\x90\x81R`\x01` \x81\x90R`@\x90\x91 c\xFF\xFF\xFF\xFF\x8B\x16\x91a\x13\x9D\x84\x86a5\xB0V[a\x13\xA7\x91\x90a5\xB0V[\x81T\x81\x10a\x13\xB7Wa\x13\xB7a4\xDCV[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a\x14\x15W`\x01a\x13\xDA\x82\x84a5\xB0V[a\x13\xE4\x91\x90a5\xB0V[\x85\x85\x81Q\x81\x10a\x13\xF6Wa\x13\xF6a4\xDCV[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPPa\x14'V[\x80a\x14\x1F\x81a5#V[\x91PPa\x13nV[PPP\x80\x80a\x145\x90a5#V[\x91PPa\x12`V[P\x94\x93PPPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R\x85\x82R`\x02\x81R\x83\x82 `\xFF\x88\x16\x83R\x90R\x91\x90\x91 \x80T\x83\x90\x81\x10a\x14\x8BWa\x14\x8Ba4\xDCV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x93\x83\x01\x93\x90\x93R`\x01`@\x1B\x90\x92\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x94\x93PPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\xFF\x83\x16`\0\x90\x81R`\x03` R`@\x90 \x80T\x83\x90\x81\x10a\x15\x16Wa\x15\x16a4\xDCV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R`\xFF\x86\x16\x82R`\x01\x90R\x91\x90\x91 \x80T\x83\x90\x81\x10a\x15\x95Wa\x15\x95a4\xDCV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x93\x83\x01\x93\x90\x93R`\x01`@\x1B\x90\x92\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16i\x91\x90a4CV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x16\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06z\x90a4`V[\x81a\x16\xB5\x81`\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x15\x15\x90V[a\x16\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06z\x90a3\xF2V[a\x05\xFB\x83\x83a#\nV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x17#W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06z\x90a5>V[`\0[\x81\x81\x10\x15a\x18\x02W`\0\x83\x83\x83\x81\x81\x10a\x17BWa\x17Ba4\xDCV[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x01` R`@\x90 T\x90\x92P\x15\x15\x90Pa\x17\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FStakeRegistry.deregisterOperator`D\x82\x01R\x7F: quorum does not exist\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06zV[`\0a\x17\xE0\x86\x83`\0a \\V[\x90Pa\x17\xEC\x82\x82a\x1D,V[PPP\x80\x80a\x17\xFA\x90a5#V[\x91PPa\x17&V[PPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18fW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\x8A\x91\x90a4CV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x18\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06z\x90a4`V[\x81a\x18\xD6\x81`\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x15\x15\x90V[a\x18\xF2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06z\x90a3\xF2V[a\x05\xFB\x83\x83a#sV[`\xFF\x83\x16`\0\x90\x81R`\x01` R`@\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x19#Wa\x19#a4\xDCV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90Pa\rA\x81\x85a'WV[`\xFF\x81\x16`\0\x90\x81R`\x01` \x81\x90R`@\x82 \x80T\x90\x91a\x19\x9B\x91a5\xB0V[\x81T\x81\x10a\x19\xABWa\x19\xABa4\xDCV[`\0\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x92\x91PPV[`\0a\x11\xFF\x84\x84\x84a(\xE2V[`\0\x82\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x88\x16\x84R\x90\x91R\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x1A\x0BWa\x1A\x0Ba4\xDCV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90Pa\x1Ab\x81\x86a'WV[`@\x01Q\x95\x94PPPPPV[`\0a\x12\x02\x83\x83a+\x80V[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R\x86\x82R`\x02\x81R\x84\x82 `\xFF\x87\x16\x83R\x81R\x84\x82 T\x85Q\x93\x84\x01\x86R\x82\x84R\x90\x83\x01\x82\x90R\x93\x82\x01R\x90\x91\x90\x81a\x1A\xD4W\x91Pa\r.\x90PV[`\0\x85\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x88\x16\x84R\x90\x91R\x90 a\x1A\xFB`\x01\x84a5\xB0V[\x81T\x81\x10a\x1B\x0BWa\x1B\x0Ba4\xDCV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x92Pa\r.\x91PPV[`\0\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 a\x1B\x87\x85\x85\x85a(\xE2V[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x1B\x9DWa\x1B\x9Da4\xDCV[`\0\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x94\x93PPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x1C\tW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06z\x90a5>V[`\xFF\x83\x16`\0\x90\x81R`\x01` R`@\x90 T\x15a\x1C\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FStakeRegistry.initializeQuorum: `D\x82\x01Rtquorum already exists`X\x1B`d\x82\x01R`\x84\x01a\x06zV[a\x1C\x91\x83\x82a#sV[a\x1C\x9B\x83\x83a#\nV[PP`\xFF\x16`\0\x90\x81R`\x01` \x81\x81R`@\x80\x84 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x87\x81R\x93\x82\x01\x87\x81R\x83T\x96\x87\x01\x84U\x92\x87R\x93\x90\x95 \x94Q\x94\x90\x93\x01\x80T\x91Q\x93Q`\x01`\x01``\x1B\x03\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x94\x84\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x95\x90\x93\x16\x94\x90\x94\x17\x17\x91\x90\x91\x16\x17\x90UV[`\xFF\x82\x16`\0\x90\x81R`\x01` \x81\x90R`@\x82 \x80T\x91\x83\x91\x90a\x1DP\x90\x84a5\xB0V[\x81T\x81\x10a\x1D`Wa\x1D`a4\xDCV[\x90`\0R` `\0 \x01\x90P\x83`\0\x14\x15a\x1D\x8FWT`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x91Pa\r.\x90PV[\x80T`\0\x90a\x1D\xAE\x90`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x86a\"\xDCV[\x82T\x90\x91PCc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14\x15a\x1D\xEBW\x81T`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x16`\x01`@\x1B`\x01`\x01``\x1B\x03\x83\x16\x02\x17\x82Ua\x1E\x9DV[\x81Tc\xFF\xFF\xFF\xFFC\x81\x16`\x01` \x1B\x81\x81\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x94\x16\x93\x90\x93\x17\x85U`\xFF\x89\x16`\0\x90\x81R`\x01` \x81\x81R`@\x80\x84 \x81Q``\x81\x01\x83R\x95\x86R\x85\x83\x01\x85\x81R`\x01`\x01``\x1B\x03\x80\x8B\x16\x93\x88\x01\x93\x84R\x82T\x95\x86\x01\x83U\x91\x86R\x92\x90\x94 \x94Q\x94\x90\x92\x01\x80T\x91Q\x92Q\x90\x93\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x92\x86\x16\x90\x96\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x91\x16\x93\x90\x94\x16\x92\x90\x92\x17\x92\x90\x92\x17\x16\x91\x90\x91\x17\x90U[\x95\x94PPPPPV[`\0\x80`\0\x80a\x1E\xC5\x86`\xFF\x16`\0\x90\x81R`\x03` R`@\x90 T\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x90\x91P`\0[\x82\x81\x10\x15a )W`\xFF\x88\x16`\0\x90\x81R`\x03` R`@\x90 \x80T\x82\x90\x81\x10a\x1F\x0BWa\x1F\x0Ba4\xDCV[`\0\x91\x82R` \x80\x83 `@\x80Q\x80\x82\x01\x82R\x93\x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x80\x82\x16\x80\x86R`\x01`\xA0\x1B\x90\x92\x04`\x01`\x01``\x1B\x03\x16\x93\x85\x01\x93\x90\x93R\x90Qcw\x8EU\xF3`\xE0\x1B\x81R\x8B\x83\x16`\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x91\x94P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cw\x8EU\xF3\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\xB1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\xD5\x91\x90a5\xDDV[\x90P\x80\x15a \x16Wg\r\xE0\xB6\xB3\xA7d\0\0\x83` \x01Q`\x01`\x01``\x1B\x03\x16\x82a\x1F\xFF\x91\x90a5\xF6V[a \t\x91\x90a6\x15V[a \x13\x90\x86a67V[\x94P[P\x80a !\x81a5#V[\x91PPa\x1E\xDFV[PPP`\xFF\x85\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90\x92P`\x01`\x01``\x1B\x03\x90\x81\x16\x90\x83\x16\x10\x15\x90P[\x92P\x92\x90PV[`\0\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 T\x81\x90\x80a! W`\0\x86\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x82R\x80\x83 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x86\x81R`\x01`\x01``\x1B\x03\x80\x8C\x16\x95\x84\x01\x95\x86R\x84T`\x01\x81\x01\x86U\x94\x88R\x95\x90\x96 \x91Q\x91\x90\x92\x01\x80T\x95Q\x93Q\x90\x94\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x93\x83\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x96\x16\x91\x90\x92\x16\x17\x93\x90\x93\x17\x16\x91\x90\x91\x17\x90Ua\"\x82V[`\0\x86\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x90\x91R\x81 a!G`\x01\x84a5\xB0V[\x81T\x81\x10a!WWa!Wa4\xDCV[`\0\x91\x82R` \x90\x91 \x01\x80T`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x91\x04\x81\x16\x94P\x90\x91P\x85\x16\x83\x14\x15a!\x90W`\0\x93PPPPa\x12\x02V[\x80TCc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14\x15a!\xCAW\x80T`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x16`\x01`@\x1B`\x01`\x01``\x1B\x03\x87\x16\x02\x17\x81Ua\"\x80V[\x80Tg\xFF\xFF\xFF\xFF\0\0\0\0\x19\x16`\x01` \x1BCc\xFF\xFF\xFF\xFF\x90\x81\x16\x82\x81\x02\x93\x90\x93\x17\x84U`\0\x8A\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x8D\x16\x84R\x82R\x80\x83 \x81Q``\x81\x01\x83R\x96\x87R\x86\x83\x01\x84\x81R`\x01`\x01``\x1B\x03\x8D\x81\x16\x93\x89\x01\x93\x84R\x82T`\x01\x81\x01\x84U\x92\x86R\x93\x90\x94 \x96Q\x96\x01\x80T\x93Q\x91Q\x96\x85\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x94\x16\x93\x90\x93\x17\x93\x16\x90\x93\x02\x91\x90\x91\x17`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x16`\x01`@\x1B\x93\x90\x92\x16\x92\x90\x92\x02\x17\x90U[P[`@\x80Q`\xFF\x87\x16\x81R`\x01`\x01``\x1B\x03\x86\x16` \x82\x01R\x87\x91\x7F/R}R~\x95\xD8\xFE@\xAE\xC5Swt;\xB7y\x08}\xA3\xF6\xD0\xD0\x8F\x12\xE3dD\xDAb2}\x91\x01`@Q\x80\x91\x03\x90\xA2a\"\xD2\x82\x85a+\x80V[\x96\x95PPPPPPV[`\0\x80\x82\x12\x15a#\0Wa\"\xEF\x82a6bV[a\"\xF9\x90\x84a6\x7FV[\x90Pa\r.V[a\"\xF9\x82\x84a67V[`\xFF\x82\x16`\0\x81\x81R` \x81\x81R`@\x91\x82\x90 \x80Tk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01``\x1B\x03\x86\x16\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F&\xEE\xCF\xF2\xB7\x0B\nq\x10O\xF4\xD9@\xBAqb\xD2:\x95\xC2Hw\x1F\xC4\x87\xA7\xBE\x17\xA5\x96\xB3\xCF\x91\x01`@Q\x80\x91\x03\x90\xA2PPV[`\0\x81Q\x11a#\xD8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R`\0\x80Q` a6\xFF\x839\x81Q\x91R`D\x82\x01R\x7F: no strategies provided\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06zV[\x80Q`\xFF\x83\x16`\0\x90\x81R`\x03` \x90\x81R`@\x90\x91 T\x90a#\xFB\x83\x83a6\xA7V[\x11\x15a$kW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R`\0\x80Q` a6\xFF\x839\x81Q\x91R`D\x82\x01R\x7F: exceed MAX_WEIGHING_FUNCTION_L`d\x82\x01Rd\x08\xA9\xC8\xEA\x89`\xDB\x1B`\x84\x82\x01R`\xA4\x01a\x06zV[`\0[\x82\x81\x10\x15a'PW`\0[a$\x83\x82\x84a6\xA7V[\x81\x10\x15a%dW\x84\x82\x81Q\x81\x10a$\x9CWa$\x9Ca4\xDCV[` \x02` \x01\x01Q`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x03`\0\x88`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x82\x81T\x81\x10a$\xDBWa$\xDBa4\xDCV[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a%RW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R`\0\x80Q` a6\xFF\x839\x81Q\x91R`D\x82\x01R\x7F: cannot add same strategy 2x\0\0\0`d\x82\x01R`\x84\x01a\x06zV[\x80a%\\\x81a5#V[\x91PPa$yV[P`\0\x84\x82\x81Q\x81\x10a%yWa%ya4\xDCV[` \x02` \x01\x01Q` \x01Q`\x01`\x01``\x1B\x03\x16\x11a%\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`F`$\x82\x01R`\0\x80Q` a6\xFF\x839\x81Q\x91R`D\x82\x01R\x7F: cannot add strategy with zero `d\x82\x01Re\x1D\xD9ZY\xDA\x1D`\xD2\x1B`\x84\x82\x01R`\xA4\x01a\x06zV[`\xFF\x85\x16`\0\x90\x81R`\x03` R`@\x90 \x84Q\x85\x90\x83\x90\x81\x10a&$Wa&$a4\xDCV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82T`\x01\x81\x01\x84U`\0\x93\x84R\x92\x82\x90 \x81Q\x91\x90\x92\x01Q`\x01`\x01``\x1B\x03\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x91\x01U\x83Q`\xFF\x86\x16\x90\x7F\x10V^V\xCA\xCB\xF3.\xCA&yE\xF0T\xFE\xC0.Yu\x002\xD1\x13\xD30!\x82\xAD\x96\x7FT\x04\x90\x86\x90\x84\x90\x81\x10a&\xA1Wa&\xA1a4\xDCV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQ`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x01`@Q\x80\x91\x03\x90\xA2\x84`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x85\x83\x81Q\x81\x10a&\xFEWa&\xFEa4\xDCV[` \x02` \x01\x01Q`\0\x01Q\x86\x84\x81Q\x81\x10a'\x1CWa'\x1Ca4\xDCV[` \x02` \x01\x01Q` \x01Q`@Qa'6\x92\x91\x90a+\xF3V[`@Q\x80\x91\x03\x90\xA2\x80a'H\x81a5#V[\x91PPa$nV[PPPPPV[\x81`\0\x01Qc\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a(\x03W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R```$\x82\x01R\x7FStakeRegistry._validateOperatorS`D\x82\x01R\x7FtakeAtBlockNumber: operatorStake`d\x82\x01R\x7FUpdate is from after blockNumber`\x84\x82\x01R`\xA4\x01a\x06zV[` \x82\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x80a()WP\x81` \x01Qc\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10[a(\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`t`$\x82\x01R\x7FStakeRegistry._validateOperatorS`D\x82\x01R\x7FtakeAtBlockNumber: there is a ne`d\x82\x01R\x7Fwer operatorStakeUpdate availabl`\x84\x82\x01Rs2\x9012\xB37\xB92\x90167\xB1\xB5\xA7:\xB6\xB12\xB9`a\x1B`\xA4\x82\x01R`\xC4\x01a\x06zV[PPV[`\0\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 T\x81[\x81\x81\x10\x15a*\xBBW`\0\x86\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x90\x91R\x90 c\xFF\xFF\xFF\xFF\x85\x16\x90`\x01a)9\x84\x86a5\xB0V[a)C\x91\x90a5\xB0V[\x81T\x81\x10a)SWa)Sa4\xDCV[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a*\xA9W`\0\x86\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x90\x91R\x81 `\x01a)\x92\x84\x86a5\xB0V[a)\x9C\x91\x90a5\xB0V[\x81T\x81\x10a)\xACWa)\xACa4\xDCV[`\0\x91\x82R` \x90\x91 \x01T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x90P\x80\x15\x80a)\xDFWP\x84c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x11[a*\x89W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`i`$\x82\x01R\x7FStakeRegistry._getStakeUpdateInd`D\x82\x01R\x7FexForOperatorAtBlockNumber: oper`d\x82\x01R\x7FatorId has no stake update at bl`\x84\x82\x01Rh7\xB1\xB5\xA7:\xB6\xB12\xB9`\xB9\x1B`\xA4\x82\x01R`\xC4\x01a\x06zV[`\x01a*\x95\x83\x85a5\xB0V[a*\x9F\x91\x90a5\xB0V[\x93PPPPa\x12\x02V[\x80a*\xB3\x81a5#V[\x91PPa)\x01V[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x81`$\x82\x01R\x7FStakeRegistry._getStakeUpdateInd`D\x82\x01R\x7FexForOperatorAtBlockNumber: no s`d\x82\x01R\x7Ftake update found for operatorId`\x84\x82\x01R\x7F and quorumNumber at block numbe`\xA4\x82\x01R`9`\xF9\x1B`\xC4\x82\x01R`\xE4\x01a\x06zV[`\0a\x12\x02`\x01`\x01``\x1B\x03\x80\x85\x16\x90\x84\x16a6\xBFV[\x805`\xFF\x81\x16\x81\x14a+\xA9W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a+\xC1W`\0\x80\xFD[a+\xCA\x83a+\x98V[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a+\xEAW`\0\x80\xFD[a\x12\x02\x82a+\x98V[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R`\x01`\x01``\x1B\x03\x16` \x82\x01R`@\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a,*W`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a,@W`\0\x80\xFD[a,I\x83a+\x98V[\x91P` \x83\x015a,Y\x81a,\x15V[\x80\x91PP\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a,vW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a,\x8DW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a UW`\0\x80\xFD[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a,\xC0W`\0\x80\xFD[a,\xC9\x86a+\x98V[\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a,\xE5W`\0\x80\xFD[a,\xF1\x89\x83\x8A\x01a,dV[\x90\x96P\x94P`@\x88\x015\x91P\x80\x82\x11\x15a-\nW`\0\x80\xFD[Pa-\x17\x88\x82\x89\x01a,dV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0\x80\x83`\x1F\x84\x01\x12a-:W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a-QW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a UW`\0\x80\xFD[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a-\x7FW`\0\x80\xFD[\x845a-\x8A\x81a,\x15V[\x93P` \x85\x015\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a-\xACW`\0\x80\xFD[a-\xB8\x87\x82\x88\x01a-(V[\x95\x98\x94\x97P\x95PPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a-\xFDW\x81Q`\x01`\x01``\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a-\xD8V[P\x94\x95\x94PPPPPV[`@\x81R`\0a.\x1B`@\x83\x01\x85a-\xC4V[\x82\x81\x03` \x84\x01Ra\x1E\x9D\x81\x85a-\xC4V[`\0\x80`@\x83\x85\x03\x12\x15a.@W`\0\x80\xFD[\x825\x91Pa.P` \x84\x01a+\x98V[\x90P\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a.\xC5Wa.\xB2\x83\x85Qc\xFF\xFF\xFF\xFF\x80\x82Q\x16\x83R\x80` \x83\x01Q\x16` \x84\x01RP`\x01`\x01``\x1B\x03`@\x82\x01Q\x16`@\x83\x01RPPV[\x92\x84\x01\x92``\x92\x90\x92\x01\x91`\x01\x01a.uV[P\x90\x96\x95PPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a/\tWa/\ta.\xD1V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a/7Wa/7a.\xD1V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a/XWa/Xa.\xD1V[P`\x05\x1B` \x01\x90V[`\0\x80`@\x83\x85\x03\x12\x15a/uW`\0\x80\xFD[a/~\x83a+\x98V[\x91P` \x80\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a/\x9AW`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a/\xABW`\0\x80\xFD[\x805a/\xBEa/\xB9\x82a/?V[a/\x0FV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x88\x83\x11\x15a/\xDDW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a/\xFBW\x835\x82R\x92\x84\x01\x92\x90\x84\x01\x90a/\xE2V[\x80\x95PPPPPP\x92P\x92\x90PV[\x805`\x01`\x01``\x1B\x03\x81\x16\x81\x14a+\xA9W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a06W`\0\x80\xFD[\x835\x92Pa0F` \x85\x01a+\x98V[\x91Pa0T`@\x85\x01a0\nV[\x90P\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a0pW`\0\x80\xFD[a+\xCA\x83a0\nV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a+\xA9W`\0\x80\xFD[`\0\x80`\0`@\x84\x86\x03\x12\x15a0\xA2W`\0\x80\xFD[a0\xAB\x84a0yV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a0\xC6W`\0\x80\xFD[a0\xD2\x86\x82\x87\x01a-(V[\x94\x97\x90\x96P\x93\x94PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a.\xC5W\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a0\xFBV[`\0\x80`\0``\x84\x86\x03\x12\x15a12W`\0\x80\xFD[a1;\x84a+\x98V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[\x81Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x80\x84\x01Q\x90\x91\x16\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01``\x1B\x03\x16\x90\x82\x01R``\x81\x01a\r.V[`\0\x80`@\x83\x85\x03\x12\x15a1\x98W`\0\x80\xFD[a1\xA1\x83a+\x98V[\x91Pa.P` \x84\x01a0\nV[`\0\x80`\0`@\x84\x86\x03\x12\x15a1\xC4W`\0\x80\xFD[\x835\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a0\xC6W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a1\xF2W`\0\x80\xFD[\x815` a2\x02a/\xB9\x83a/?V[\x82\x81R`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a2!W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a2pW`@\x81\x89\x03\x12\x15a2>W`\0\x80\x81\xFD[a2Fa.\xE7V[\x815a2Q\x81a,\x15V[\x81Ra2^\x82\x86\x01a0\nV[\x81\x86\x01R\x83R\x91\x83\x01\x91`@\x01a2%V[P\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a2\x8EW`\0\x80\xFD[a2\x97\x83a+\x98V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a2\xB2W`\0\x80\xFD[a2\xBE\x85\x82\x86\x01a1\xE1V[\x91PP\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a2\xDDW`\0\x80\xFD[a2\xE6\x84a+\x98V[\x92Pa2\xF4` \x85\x01a0yV[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a3\x19W`\0\x80\xFD[\x835\x92Pa3)` \x85\x01a+\x98V[\x91Pa0T`@\x85\x01a0yV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a3MW`\0\x80\xFD[a3V\x85a+\x98V[\x93Pa3d` \x86\x01a0yV[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[`\0\x80`@\x83\x85\x03\x12\x15a3\x8CW`\0\x80\xFD[a1\xA1\x83a0\nV[`\0\x80`\0``\x84\x86\x03\x12\x15a3\xAAW`\0\x80\xFD[a3\xB3\x84a+\x98V[\x92Pa3\xC1` \x85\x01a0\nV[\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a3\xDCW`\0\x80\xFD[a3\xE8\x86\x82\x87\x01a1\xE1V[\x91PP\x92P\x92P\x92V[` \x80\x82R`1\x90\x82\x01R\x7FStakeRegistry.quorumExists: quor`@\x82\x01Rp\x1D[H\x19\x1B\xD9\\\xC8\x1B\x9B\xDD\x08\x19^\x1A\\\xDD`z\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15a4UW`\0\x80\xFD[\x81Qa\x12\x02\x81a,\x15V[` \x80\x82R`V\x90\x82\x01R\x7FStakeRegistry.onlyCoordinatorOwn`@\x82\x01R\x7Fer: caller is not the owner of t``\x82\x01Ru42\x9092\xB3\xB4\xB9\xBA9<\xA1\xB7\xB7\xB924\xB70\xBA7\xB9`Q\x1B`\x80\x82\x01R`\xA0\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a5\x04W`\0\x80\xFD[a\x12\x02\x82a0\nV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15a57Wa57a5\rV[P`\x01\x01\x90V[` \x80\x82R`L\x90\x82\x01R\x7FStakeRegistry.onlyRegistryCoordi`@\x82\x01R\x7Fnator: caller is not the Registr``\x82\x01Rk<\xA1\xB7\xB7\xB924\xB70\xBA7\xB9`\xA1\x1B`\x80\x82\x01R`\xA0\x01\x90V[`\0\x82\x82\x10\x15a5\xC2Wa5\xC2a5\rV[P\x03\x90V[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a5\xEFW`\0\x80\xFD[PQ\x91\x90PV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a6\x10Wa6\x10a5\rV[P\x02\x90V[`\0\x82a62WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a6YWa6Ya5\rV[\x01\x94\x93PPPPV[`\0`\x01`\xFF\x1B\x82\x14\x15a6xWa6xa5\rV[P`\0\x03\x90V[`\0`\x01`\x01``\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a6\x9FWa6\x9Fa5\rV[\x03\x93\x92PPPV[`\0\x82\x19\x82\x11\x15a6\xBAWa6\xBAa5\rV[P\x01\x90V[`\0\x80\x83\x12\x80\x15`\x01`\xFF\x1B\x85\x01\x84\x12\x16\x15a6\xDDWa6\xDDa5\rV[`\x01`\x01`\xFF\x1B\x03\x84\x01\x83\x13\x81\x16\x15a6\xF8Wa6\xF8a5\rV[PP\x03\x90V\xFEStakeRegistry._addStrategyParams\xA2dipfsX\"\x12 \x08v\x88D\x84\xC2\xAA\xC0\xFC\xDD\xC8\xC3\x0C~2\xB6G\xD3\xB4\xF4\xB5\xBB\xCA\xAB\xB7\xAB\x8Eg\xA6\x82\x96\x92dsolcC\0\x08\x0C\x003";
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
        ///Calls the contract's `addStrategies` (0xc601527d) function
        pub fn add_strategies(
            &self,
            quorum_number: u8,
            strategy_params: ::std::vec::Vec<StrategyParams>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([198, 1, 82, 125], (quorum_number, strategy_params))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `applyDelta` (0x7f429822) function
        pub fn apply_delta(
            &self,
            value: u128,
            delta: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([127, 66, 152, 34], (value, delta))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateDelta` (0xf509551a) function
        pub fn calculate_delta(
            &self,
            prev: u128,
            cur: u128,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([245, 9, 85, 26], (prev, cur))
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
        ///Calls the contract's `getCurrentStake` (0x5401ed27) function
        pub fn get_current_stake(
            &self,
            operator_id: [u8; 32],
            quorum_number: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([84, 1, 237, 39], (operator_id, quorum_number))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCurrentTotalStake` (0xd5eccc05) function
        pub fn get_current_total_stake(
            &self,
            quorum_number: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([213, 236, 204, 5], quorum_number)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLatestStakeUpdate` (0xf851e198) function
        pub fn get_latest_stake_update(
            &self,
            operator_id: [u8; 32],
            quorum_number: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, StakeUpdate> {
            self.0
                .method_hash([248, 81, 225, 152], (operator_id, quorum_number))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStakeAtBlockNumber` (0xfa28c627) function
        pub fn get_stake_at_block_number(
            &self,
            operator_id: [u8; 32],
            quorum_number: u8,
            block_number: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash(
                    [250, 40, 198, 39],
                    (operator_id, quorum_number, block_number),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStakeAtBlockNumberAndIndex` (0xf2be94ae) function
        pub fn get_stake_at_block_number_and_index(
            &self,
            quorum_number: u8,
            block_number: u32,
            operator_id: [u8; 32],
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash(
                    [242, 190, 148, 174],
                    (quorum_number, block_number, operator_id, index),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStakeHistory` (0x2cd95940) function
        pub fn get_stake_history(
            &self,
            operator_id: [u8; 32],
            quorum_number: u8,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<StakeUpdate>,
        > {
            self.0
                .method_hash([44, 217, 89, 64], (operator_id, quorum_number))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStakeHistoryLength` (0x4bd26e09) function
        pub fn get_stake_history_length(
            &self,
            operator_id: [u8; 32],
            quorum_number: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([75, 210, 110, 9], (operator_id, quorum_number))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStakeUpdateAtIndex` (0xac6bfb03) function
        pub fn get_stake_update_at_index(
            &self,
            quorum_number: u8,
            operator_id: [u8; 32],
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, StakeUpdate> {
            self.0
                .method_hash([172, 107, 251, 3], (quorum_number, operator_id, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStakeUpdateIndexAtBlockNumber` (0xdd9846b9) function
        pub fn get_stake_update_index_at_block_number(
            &self,
            operator_id: [u8; 32],
            quorum_number: u8,
            block_number: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash(
                    [221, 152, 70, 185],
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
        ///Calls the contract's `getTotalStakeHistoryLength` (0x0491b41c) function
        pub fn get_total_stake_history_length(
            &self,
            quorum_number: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([4, 145, 180, 28], quorum_number)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTotalStakeIndicesAtBlockNumber` (0x81c07502) function
        pub fn get_total_stake_indices_at_block_number(
            &self,
            block_number: u32,
            quorum_numbers: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<u32>> {
            self.0
                .method_hash([129, 192, 117, 2], (block_number, quorum_numbers))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTotalStakeUpdateAtIndex` (0xb6904b78) function
        pub fn get_total_stake_update_at_index(
            &self,
            quorum_number: u8,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, StakeUpdate> {
            self.0
                .method_hash([182, 144, 75, 120], (quorum_number, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initializeQuorum` (0xff694a77) function
        pub fn initialize_quorum(
            &self,
            quorum_number: u8,
            minimum_stake: u128,
            strategy_params: ::std::vec::Vec<StrategyParams>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [255, 105, 74, 119],
                    (quorum_number, minimum_stake, strategy_params),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `minimumStakeForQuorum` (0xc46778a5) function
        pub fn minimum_stake_for_quorum(
            &self,
            p0: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([196, 103, 120, 165], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `modifyStrategyParams` (0x20b66298) function
        pub fn modify_strategy_params(
            &self,
            quorum_number: u8,
            strategy_indices: ::std::vec::Vec<::ethers::core::types::U256>,
            new_multipliers: ::std::vec::Vec<u128>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [32, 182, 98, 152],
                    (quorum_number, strategy_indices, new_multipliers),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `recordOperatorStakeUpdate` (0x74454c6d) function
        pub fn record_operator_stake_update(
            &self,
            operator_id: [u8; 32],
            quorum_number: u8,
            new_stake: u128,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([116, 69, 76, 109], (operator_id, quorum_number, new_stake))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `recordTotalStakeUpdate` (0x0390a4d5) function
        pub fn record_total_stake_update(
            &self,
            quorum_number: u8,
            stake_delta: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([3, 144, 164, 213], (quorum_number, stake_delta))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerOperator` (0x25504777) function
        pub fn register_operator(
            &self,
            operator: ::ethers::core::types::Address,
            operator_id: [u8; 32],
            quorum_numbers: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::std::vec::Vec<u128>, ::std::vec::Vec<u128>),
        > {
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
        ///Calls the contract's `removeStrategies` (0x5f1f2d77) function
        pub fn remove_strategies(
            &self,
            quorum_number: u8,
            indices_to_remove: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([95, 31, 45, 119], (quorum_number, indices_to_remove))
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
        ///Calls the contract's `strategyParams` (0x08732461) function
        pub fn strategy_params(
            &self,
            p0: u8,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::Address, u128),
        > {
            self.0
                .method_hash([8, 115, 36, 97], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `strategyParamsByIndex` (0xadc804da) function
        pub fn strategy_params_by_index(
            &self,
            quorum_number: u8,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, StrategyParams> {
            self.0
                .method_hash([173, 200, 4, 218], (quorum_number, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `strategyParamsLength` (0x3ca5a5f5) function
        pub fn strategy_params_length(
            &self,
            quorum_number: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([60, 165, 165, 245], quorum_number)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateOperatorStake` (0x66acfefe) function
        pub fn update_operator_stake(
            &self,
            operator: ::ethers::core::types::Address,
            operator_id: [u8; 32],
            quorum_numbers: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [102, 172, 254, 254],
                    (operator, operator_id, quorum_numbers),
                )
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
        ///Gets the contract's `OperatorStakeUpdate` event
        pub fn operator_stake_update_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OperatorStakeUpdateFilter,
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
    #[ethevent(
        name = "OperatorStakeUpdate",
        abi = "OperatorStakeUpdate(bytes32,uint8,uint96)"
    )]
    pub struct OperatorStakeUpdateFilter {
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
        MinimumStakeForQuorumUpdatedFilter(MinimumStakeForQuorumUpdatedFilter),
        OperatorStakeUpdateFilter(OperatorStakeUpdateFilter),
        QuorumCreatedFilter(QuorumCreatedFilter),
        StrategyAddedToQuorumFilter(StrategyAddedToQuorumFilter),
        StrategyMultiplierUpdatedFilter(StrategyMultiplierUpdatedFilter),
        StrategyRemovedFromQuorumFilter(StrategyRemovedFromQuorumFilter),
    }
    impl ::ethers::contract::EthLogDecode for StakeRegistryHarnessEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = MinimumStakeForQuorumUpdatedFilter::decode_log(log) {
                return Ok(
                    StakeRegistryHarnessEvents::MinimumStakeForQuorumUpdatedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = OperatorStakeUpdateFilter::decode_log(log) {
                return Ok(
                    StakeRegistryHarnessEvents::OperatorStakeUpdateFilter(decoded),
                );
            }
            if let Ok(decoded) = QuorumCreatedFilter::decode_log(log) {
                return Ok(StakeRegistryHarnessEvents::QuorumCreatedFilter(decoded));
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
                Self::MinimumStakeForQuorumUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorStakeUpdateFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::QuorumCreatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
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
    impl ::core::convert::From<MinimumStakeForQuorumUpdatedFilter>
    for StakeRegistryHarnessEvents {
        fn from(value: MinimumStakeForQuorumUpdatedFilter) -> Self {
            Self::MinimumStakeForQuorumUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<OperatorStakeUpdateFilter>
    for StakeRegistryHarnessEvents {
        fn from(value: OperatorStakeUpdateFilter) -> Self {
            Self::OperatorStakeUpdateFilter(value)
        }
    }
    impl ::core::convert::From<QuorumCreatedFilter> for StakeRegistryHarnessEvents {
        fn from(value: QuorumCreatedFilter) -> Self {
            Self::QuorumCreatedFilter(value)
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
    ///Container type for all input parameters for the `addStrategies` function with signature `addStrategies(uint8,(address,uint96)[])` and selector `0xc601527d`
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
    #[ethcall(name = "addStrategies", abi = "addStrategies(uint8,(address,uint96)[])")]
    pub struct AddStrategiesCall {
        pub quorum_number: u8,
        pub strategy_params: ::std::vec::Vec<StrategyParams>,
    }
    ///Container type for all input parameters for the `applyDelta` function with signature `applyDelta(uint96,int256)` and selector `0x7f429822`
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
    #[ethcall(name = "applyDelta", abi = "applyDelta(uint96,int256)")]
    pub struct ApplyDeltaCall {
        pub value: u128,
        pub delta: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `calculateDelta` function with signature `calculateDelta(uint96,uint96)` and selector `0xf509551a`
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
    #[ethcall(name = "calculateDelta", abi = "calculateDelta(uint96,uint96)")]
    pub struct CalculateDeltaCall {
        pub prev: u128,
        pub cur: u128,
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
    ///Container type for all input parameters for the `getCurrentStake` function with signature `getCurrentStake(bytes32,uint8)` and selector `0x5401ed27`
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
    #[ethcall(name = "getCurrentStake", abi = "getCurrentStake(bytes32,uint8)")]
    pub struct GetCurrentStakeCall {
        pub operator_id: [u8; 32],
        pub quorum_number: u8,
    }
    ///Container type for all input parameters for the `getCurrentTotalStake` function with signature `getCurrentTotalStake(uint8)` and selector `0xd5eccc05`
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
    #[ethcall(name = "getCurrentTotalStake", abi = "getCurrentTotalStake(uint8)")]
    pub struct GetCurrentTotalStakeCall {
        pub quorum_number: u8,
    }
    ///Container type for all input parameters for the `getLatestStakeUpdate` function with signature `getLatestStakeUpdate(bytes32,uint8)` and selector `0xf851e198`
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
        name = "getLatestStakeUpdate",
        abi = "getLatestStakeUpdate(bytes32,uint8)"
    )]
    pub struct GetLatestStakeUpdateCall {
        pub operator_id: [u8; 32],
        pub quorum_number: u8,
    }
    ///Container type for all input parameters for the `getStakeAtBlockNumber` function with signature `getStakeAtBlockNumber(bytes32,uint8,uint32)` and selector `0xfa28c627`
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
        name = "getStakeAtBlockNumber",
        abi = "getStakeAtBlockNumber(bytes32,uint8,uint32)"
    )]
    pub struct GetStakeAtBlockNumberCall {
        pub operator_id: [u8; 32],
        pub quorum_number: u8,
        pub block_number: u32,
    }
    ///Container type for all input parameters for the `getStakeAtBlockNumberAndIndex` function with signature `getStakeAtBlockNumberAndIndex(uint8,uint32,bytes32,uint256)` and selector `0xf2be94ae`
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
        name = "getStakeAtBlockNumberAndIndex",
        abi = "getStakeAtBlockNumberAndIndex(uint8,uint32,bytes32,uint256)"
    )]
    pub struct GetStakeAtBlockNumberAndIndexCall {
        pub quorum_number: u8,
        pub block_number: u32,
        pub operator_id: [u8; 32],
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getStakeHistory` function with signature `getStakeHistory(bytes32,uint8)` and selector `0x2cd95940`
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
    #[ethcall(name = "getStakeHistory", abi = "getStakeHistory(bytes32,uint8)")]
    pub struct GetStakeHistoryCall {
        pub operator_id: [u8; 32],
        pub quorum_number: u8,
    }
    ///Container type for all input parameters for the `getStakeHistoryLength` function with signature `getStakeHistoryLength(bytes32,uint8)` and selector `0x4bd26e09`
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
        name = "getStakeHistoryLength",
        abi = "getStakeHistoryLength(bytes32,uint8)"
    )]
    pub struct GetStakeHistoryLengthCall {
        pub operator_id: [u8; 32],
        pub quorum_number: u8,
    }
    ///Container type for all input parameters for the `getStakeUpdateAtIndex` function with signature `getStakeUpdateAtIndex(uint8,bytes32,uint256)` and selector `0xac6bfb03`
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
        name = "getStakeUpdateAtIndex",
        abi = "getStakeUpdateAtIndex(uint8,bytes32,uint256)"
    )]
    pub struct GetStakeUpdateAtIndexCall {
        pub quorum_number: u8,
        pub operator_id: [u8; 32],
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getStakeUpdateIndexAtBlockNumber` function with signature `getStakeUpdateIndexAtBlockNumber(bytes32,uint8,uint32)` and selector `0xdd9846b9`
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
        name = "getStakeUpdateIndexAtBlockNumber",
        abi = "getStakeUpdateIndexAtBlockNumber(bytes32,uint8,uint32)"
    )]
    pub struct GetStakeUpdateIndexAtBlockNumberCall {
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
    ///Container type for all input parameters for the `getTotalStakeHistoryLength` function with signature `getTotalStakeHistoryLength(uint8)` and selector `0x0491b41c`
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
        name = "getTotalStakeHistoryLength",
        abi = "getTotalStakeHistoryLength(uint8)"
    )]
    pub struct GetTotalStakeHistoryLengthCall {
        pub quorum_number: u8,
    }
    ///Container type for all input parameters for the `getTotalStakeIndicesAtBlockNumber` function with signature `getTotalStakeIndicesAtBlockNumber(uint32,bytes)` and selector `0x81c07502`
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
        name = "getTotalStakeIndicesAtBlockNumber",
        abi = "getTotalStakeIndicesAtBlockNumber(uint32,bytes)"
    )]
    pub struct GetTotalStakeIndicesAtBlockNumberCall {
        pub block_number: u32,
        pub quorum_numbers: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `getTotalStakeUpdateAtIndex` function with signature `getTotalStakeUpdateAtIndex(uint8,uint256)` and selector `0xb6904b78`
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
        name = "getTotalStakeUpdateAtIndex",
        abi = "getTotalStakeUpdateAtIndex(uint8,uint256)"
    )]
    pub struct GetTotalStakeUpdateAtIndexCall {
        pub quorum_number: u8,
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `initializeQuorum` function with signature `initializeQuorum(uint8,uint96,(address,uint96)[])` and selector `0xff694a77`
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
        name = "initializeQuorum",
        abi = "initializeQuorum(uint8,uint96,(address,uint96)[])"
    )]
    pub struct InitializeQuorumCall {
        pub quorum_number: u8,
        pub minimum_stake: u128,
        pub strategy_params: ::std::vec::Vec<StrategyParams>,
    }
    ///Container type for all input parameters for the `minimumStakeForQuorum` function with signature `minimumStakeForQuorum(uint8)` and selector `0xc46778a5`
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
    #[ethcall(name = "minimumStakeForQuorum", abi = "minimumStakeForQuorum(uint8)")]
    pub struct MinimumStakeForQuorumCall(pub u8);
    ///Container type for all input parameters for the `modifyStrategyParams` function with signature `modifyStrategyParams(uint8,uint256[],uint96[])` and selector `0x20b66298`
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
        name = "modifyStrategyParams",
        abi = "modifyStrategyParams(uint8,uint256[],uint96[])"
    )]
    pub struct ModifyStrategyParamsCall {
        pub quorum_number: u8,
        pub strategy_indices: ::std::vec::Vec<::ethers::core::types::U256>,
        pub new_multipliers: ::std::vec::Vec<u128>,
    }
    ///Container type for all input parameters for the `recordOperatorStakeUpdate` function with signature `recordOperatorStakeUpdate(bytes32,uint8,uint96)` and selector `0x74454c6d`
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
        abi = "recordOperatorStakeUpdate(bytes32,uint8,uint96)"
    )]
    pub struct RecordOperatorStakeUpdateCall {
        pub operator_id: [u8; 32],
        pub quorum_number: u8,
        pub new_stake: u128,
    }
    ///Container type for all input parameters for the `recordTotalStakeUpdate` function with signature `recordTotalStakeUpdate(uint8,int256)` and selector `0x0390a4d5`
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
        abi = "recordTotalStakeUpdate(uint8,int256)"
    )]
    pub struct RecordTotalStakeUpdateCall {
        pub quorum_number: u8,
        pub stake_delta: ::ethers::core::types::I256,
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
    ///Container type for all input parameters for the `removeStrategies` function with signature `removeStrategies(uint8,uint256[])` and selector `0x5f1f2d77`
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
    #[ethcall(name = "removeStrategies", abi = "removeStrategies(uint8,uint256[])")]
    pub struct RemoveStrategiesCall {
        pub quorum_number: u8,
        pub indices_to_remove: ::std::vec::Vec<::ethers::core::types::U256>,
    }
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
    ///Container type for all input parameters for the `strategyParams` function with signature `strategyParams(uint8,uint256)` and selector `0x08732461`
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
    #[ethcall(name = "strategyParams", abi = "strategyParams(uint8,uint256)")]
    pub struct StrategyParamsCall(pub u8, pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `strategyParamsByIndex` function with signature `strategyParamsByIndex(uint8,uint256)` and selector `0xadc804da`
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
        name = "strategyParamsByIndex",
        abi = "strategyParamsByIndex(uint8,uint256)"
    )]
    pub struct StrategyParamsByIndexCall {
        pub quorum_number: u8,
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `strategyParamsLength` function with signature `strategyParamsLength(uint8)` and selector `0x3ca5a5f5`
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
    #[ethcall(name = "strategyParamsLength", abi = "strategyParamsLength(uint8)")]
    pub struct StrategyParamsLengthCall {
        pub quorum_number: u8,
    }
    ///Container type for all input parameters for the `updateOperatorStake` function with signature `updateOperatorStake(address,bytes32,bytes)` and selector `0x66acfefe`
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
        abi = "updateOperatorStake(address,bytes32,bytes)"
    )]
    pub struct UpdateOperatorStakeCall {
        pub operator: ::ethers::core::types::Address,
        pub operator_id: [u8; 32],
        pub quorum_numbers: ::ethers::core::types::Bytes,
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
        MaxWeighingFunctionLength(MaxWeighingFunctionLengthCall),
        WeightingDivisor(WeightingDivisorCall),
        AddStrategies(AddStrategiesCall),
        ApplyDelta(ApplyDeltaCall),
        CalculateDelta(CalculateDeltaCall),
        Delegation(DelegationCall),
        DeregisterOperator(DeregisterOperatorCall),
        GetCurrentStake(GetCurrentStakeCall),
        GetCurrentTotalStake(GetCurrentTotalStakeCall),
        GetLatestStakeUpdate(GetLatestStakeUpdateCall),
        GetStakeAtBlockNumber(GetStakeAtBlockNumberCall),
        GetStakeAtBlockNumberAndIndex(GetStakeAtBlockNumberAndIndexCall),
        GetStakeHistory(GetStakeHistoryCall),
        GetStakeHistoryLength(GetStakeHistoryLengthCall),
        GetStakeUpdateAtIndex(GetStakeUpdateAtIndexCall),
        GetStakeUpdateIndexAtBlockNumber(GetStakeUpdateIndexAtBlockNumberCall),
        GetTotalStakeAtBlockNumberFromIndex(GetTotalStakeAtBlockNumberFromIndexCall),
        GetTotalStakeHistoryLength(GetTotalStakeHistoryLengthCall),
        GetTotalStakeIndicesAtBlockNumber(GetTotalStakeIndicesAtBlockNumberCall),
        GetTotalStakeUpdateAtIndex(GetTotalStakeUpdateAtIndexCall),
        InitializeQuorum(InitializeQuorumCall),
        MinimumStakeForQuorum(MinimumStakeForQuorumCall),
        ModifyStrategyParams(ModifyStrategyParamsCall),
        RecordOperatorStakeUpdate(RecordOperatorStakeUpdateCall),
        RecordTotalStakeUpdate(RecordTotalStakeUpdateCall),
        RegisterOperator(RegisterOperatorCall),
        RegistryCoordinator(RegistryCoordinatorCall),
        RemoveStrategies(RemoveStrategiesCall),
        SetMinimumStakeForQuorum(SetMinimumStakeForQuorumCall),
        StrategyParams(StrategyParamsCall),
        StrategyParamsByIndex(StrategyParamsByIndexCall),
        StrategyParamsLength(StrategyParamsLengthCall),
        UpdateOperatorStake(UpdateOperatorStakeCall),
        WeightOfOperatorForQuorum(WeightOfOperatorForQuorumCall),
    }
    impl ::ethers::core::abi::AbiDecode for StakeRegistryHarnessCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
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
            if let Ok(decoded) = <AddStrategiesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddStrategies(decoded));
            }
            if let Ok(decoded) = <ApplyDeltaCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ApplyDelta(decoded));
            }
            if let Ok(decoded) = <CalculateDeltaCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateDelta(decoded));
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
            if let Ok(decoded) = <GetCurrentStakeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetCurrentStake(decoded));
            }
            if let Ok(decoded) = <GetCurrentTotalStakeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetCurrentTotalStake(decoded));
            }
            if let Ok(decoded) = <GetLatestStakeUpdateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetLatestStakeUpdate(decoded));
            }
            if let Ok(decoded) = <GetStakeAtBlockNumberCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetStakeAtBlockNumber(decoded));
            }
            if let Ok(decoded) = <GetStakeAtBlockNumberAndIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetStakeAtBlockNumberAndIndex(decoded));
            }
            if let Ok(decoded) = <GetStakeHistoryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetStakeHistory(decoded));
            }
            if let Ok(decoded) = <GetStakeHistoryLengthCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetStakeHistoryLength(decoded));
            }
            if let Ok(decoded) = <GetStakeUpdateAtIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetStakeUpdateAtIndex(decoded));
            }
            if let Ok(decoded) = <GetStakeUpdateIndexAtBlockNumberCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetStakeUpdateIndexAtBlockNumber(decoded));
            }
            if let Ok(decoded) = <GetTotalStakeAtBlockNumberFromIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTotalStakeAtBlockNumberFromIndex(decoded));
            }
            if let Ok(decoded) = <GetTotalStakeHistoryLengthCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTotalStakeHistoryLength(decoded));
            }
            if let Ok(decoded) = <GetTotalStakeIndicesAtBlockNumberCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTotalStakeIndicesAtBlockNumber(decoded));
            }
            if let Ok(decoded) = <GetTotalStakeUpdateAtIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTotalStakeUpdateAtIndex(decoded));
            }
            if let Ok(decoded) = <InitializeQuorumCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InitializeQuorum(decoded));
            }
            if let Ok(decoded) = <MinimumStakeForQuorumCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MinimumStakeForQuorum(decoded));
            }
            if let Ok(decoded) = <ModifyStrategyParamsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ModifyStrategyParams(decoded));
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
            if let Ok(decoded) = <RegistryCoordinatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RegistryCoordinator(decoded));
            }
            if let Ok(decoded) = <RemoveStrategiesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveStrategies(decoded));
            }
            if let Ok(decoded) = <SetMinimumStakeForQuorumCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetMinimumStakeForQuorum(decoded));
            }
            if let Ok(decoded) = <StrategyParamsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StrategyParams(decoded));
            }
            if let Ok(decoded) = <StrategyParamsByIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StrategyParamsByIndex(decoded));
            }
            if let Ok(decoded) = <StrategyParamsLengthCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StrategyParamsLength(decoded));
            }
            if let Ok(decoded) = <UpdateOperatorStakeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateOperatorStake(decoded));
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
                Self::MaxWeighingFunctionLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WeightingDivisor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddStrategies(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ApplyDelta(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateDelta(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Delegation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeregisterOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetCurrentStake(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetCurrentTotalStake(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetLatestStakeUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetStakeAtBlockNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetStakeAtBlockNumberAndIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetStakeHistory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetStakeHistoryLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetStakeUpdateAtIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetStakeUpdateIndexAtBlockNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTotalStakeAtBlockNumberFromIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTotalStakeHistoryLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTotalStakeIndicesAtBlockNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTotalStakeUpdateAtIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InitializeQuorum(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MinimumStakeForQuorum(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ModifyStrategyParams(element) => {
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
                Self::RegistryCoordinator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveStrategies(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetMinimumStakeForQuorum(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StrategyParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StrategyParamsByIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StrategyParamsLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateOperatorStake(element) => {
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
                Self::MaxWeighingFunctionLength(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WeightingDivisor(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddStrategies(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApplyDelta(element) => ::core::fmt::Display::fmt(element, f),
                Self::CalculateDelta(element) => ::core::fmt::Display::fmt(element, f),
                Self::Delegation(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeregisterOperator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetCurrentStake(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCurrentTotalStake(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetLatestStakeUpdate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetStakeAtBlockNumber(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetStakeAtBlockNumberAndIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetStakeHistory(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStakeHistoryLength(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetStakeUpdateAtIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetStakeUpdateIndexAtBlockNumber(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTotalStakeAtBlockNumberFromIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTotalStakeHistoryLength(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTotalStakeIndicesAtBlockNumber(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTotalStakeUpdateAtIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializeQuorum(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinimumStakeForQuorum(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ModifyStrategyParams(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RecordOperatorStakeUpdate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RecordTotalStakeUpdate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RegisterOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegistryCoordinator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemoveStrategies(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetMinimumStakeForQuorum(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StrategyParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::StrategyParamsByIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StrategyParamsLength(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateOperatorStake(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WeightOfOperatorForQuorum(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
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
    impl ::core::convert::From<AddStrategiesCall> for StakeRegistryHarnessCalls {
        fn from(value: AddStrategiesCall) -> Self {
            Self::AddStrategies(value)
        }
    }
    impl ::core::convert::From<ApplyDeltaCall> for StakeRegistryHarnessCalls {
        fn from(value: ApplyDeltaCall) -> Self {
            Self::ApplyDelta(value)
        }
    }
    impl ::core::convert::From<CalculateDeltaCall> for StakeRegistryHarnessCalls {
        fn from(value: CalculateDeltaCall) -> Self {
            Self::CalculateDelta(value)
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
    impl ::core::convert::From<GetCurrentStakeCall> for StakeRegistryHarnessCalls {
        fn from(value: GetCurrentStakeCall) -> Self {
            Self::GetCurrentStake(value)
        }
    }
    impl ::core::convert::From<GetCurrentTotalStakeCall> for StakeRegistryHarnessCalls {
        fn from(value: GetCurrentTotalStakeCall) -> Self {
            Self::GetCurrentTotalStake(value)
        }
    }
    impl ::core::convert::From<GetLatestStakeUpdateCall> for StakeRegistryHarnessCalls {
        fn from(value: GetLatestStakeUpdateCall) -> Self {
            Self::GetLatestStakeUpdate(value)
        }
    }
    impl ::core::convert::From<GetStakeAtBlockNumberCall> for StakeRegistryHarnessCalls {
        fn from(value: GetStakeAtBlockNumberCall) -> Self {
            Self::GetStakeAtBlockNumber(value)
        }
    }
    impl ::core::convert::From<GetStakeAtBlockNumberAndIndexCall>
    for StakeRegistryHarnessCalls {
        fn from(value: GetStakeAtBlockNumberAndIndexCall) -> Self {
            Self::GetStakeAtBlockNumberAndIndex(value)
        }
    }
    impl ::core::convert::From<GetStakeHistoryCall> for StakeRegistryHarnessCalls {
        fn from(value: GetStakeHistoryCall) -> Self {
            Self::GetStakeHistory(value)
        }
    }
    impl ::core::convert::From<GetStakeHistoryLengthCall> for StakeRegistryHarnessCalls {
        fn from(value: GetStakeHistoryLengthCall) -> Self {
            Self::GetStakeHistoryLength(value)
        }
    }
    impl ::core::convert::From<GetStakeUpdateAtIndexCall> for StakeRegistryHarnessCalls {
        fn from(value: GetStakeUpdateAtIndexCall) -> Self {
            Self::GetStakeUpdateAtIndex(value)
        }
    }
    impl ::core::convert::From<GetStakeUpdateIndexAtBlockNumberCall>
    for StakeRegistryHarnessCalls {
        fn from(value: GetStakeUpdateIndexAtBlockNumberCall) -> Self {
            Self::GetStakeUpdateIndexAtBlockNumber(value)
        }
    }
    impl ::core::convert::From<GetTotalStakeAtBlockNumberFromIndexCall>
    for StakeRegistryHarnessCalls {
        fn from(value: GetTotalStakeAtBlockNumberFromIndexCall) -> Self {
            Self::GetTotalStakeAtBlockNumberFromIndex(value)
        }
    }
    impl ::core::convert::From<GetTotalStakeHistoryLengthCall>
    for StakeRegistryHarnessCalls {
        fn from(value: GetTotalStakeHistoryLengthCall) -> Self {
            Self::GetTotalStakeHistoryLength(value)
        }
    }
    impl ::core::convert::From<GetTotalStakeIndicesAtBlockNumberCall>
    for StakeRegistryHarnessCalls {
        fn from(value: GetTotalStakeIndicesAtBlockNumberCall) -> Self {
            Self::GetTotalStakeIndicesAtBlockNumber(value)
        }
    }
    impl ::core::convert::From<GetTotalStakeUpdateAtIndexCall>
    for StakeRegistryHarnessCalls {
        fn from(value: GetTotalStakeUpdateAtIndexCall) -> Self {
            Self::GetTotalStakeUpdateAtIndex(value)
        }
    }
    impl ::core::convert::From<InitializeQuorumCall> for StakeRegistryHarnessCalls {
        fn from(value: InitializeQuorumCall) -> Self {
            Self::InitializeQuorum(value)
        }
    }
    impl ::core::convert::From<MinimumStakeForQuorumCall> for StakeRegistryHarnessCalls {
        fn from(value: MinimumStakeForQuorumCall) -> Self {
            Self::MinimumStakeForQuorum(value)
        }
    }
    impl ::core::convert::From<ModifyStrategyParamsCall> for StakeRegistryHarnessCalls {
        fn from(value: ModifyStrategyParamsCall) -> Self {
            Self::ModifyStrategyParams(value)
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
    impl ::core::convert::From<RegistryCoordinatorCall> for StakeRegistryHarnessCalls {
        fn from(value: RegistryCoordinatorCall) -> Self {
            Self::RegistryCoordinator(value)
        }
    }
    impl ::core::convert::From<RemoveStrategiesCall> for StakeRegistryHarnessCalls {
        fn from(value: RemoveStrategiesCall) -> Self {
            Self::RemoveStrategies(value)
        }
    }
    impl ::core::convert::From<SetMinimumStakeForQuorumCall>
    for StakeRegistryHarnessCalls {
        fn from(value: SetMinimumStakeForQuorumCall) -> Self {
            Self::SetMinimumStakeForQuorum(value)
        }
    }
    impl ::core::convert::From<StrategyParamsCall> for StakeRegistryHarnessCalls {
        fn from(value: StrategyParamsCall) -> Self {
            Self::StrategyParams(value)
        }
    }
    impl ::core::convert::From<StrategyParamsByIndexCall> for StakeRegistryHarnessCalls {
        fn from(value: StrategyParamsByIndexCall) -> Self {
            Self::StrategyParamsByIndex(value)
        }
    }
    impl ::core::convert::From<StrategyParamsLengthCall> for StakeRegistryHarnessCalls {
        fn from(value: StrategyParamsLengthCall) -> Self {
            Self::StrategyParamsLength(value)
        }
    }
    impl ::core::convert::From<UpdateOperatorStakeCall> for StakeRegistryHarnessCalls {
        fn from(value: UpdateOperatorStakeCall) -> Self {
            Self::UpdateOperatorStake(value)
        }
    }
    impl ::core::convert::From<WeightOfOperatorForQuorumCall>
    for StakeRegistryHarnessCalls {
        fn from(value: WeightOfOperatorForQuorumCall) -> Self {
            Self::WeightOfOperatorForQuorum(value)
        }
    }
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
    ///Container type for all return fields from the `applyDelta` function with signature `applyDelta(uint96,int256)` and selector `0x7f429822`
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
    pub struct ApplyDeltaReturn(pub u128);
    ///Container type for all return fields from the `calculateDelta` function with signature `calculateDelta(uint96,uint96)` and selector `0xf509551a`
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
    pub struct CalculateDeltaReturn(pub ::ethers::core::types::I256);
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
    ///Container type for all return fields from the `getCurrentStake` function with signature `getCurrentStake(bytes32,uint8)` and selector `0x5401ed27`
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
    pub struct GetCurrentStakeReturn(pub u128);
    ///Container type for all return fields from the `getCurrentTotalStake` function with signature `getCurrentTotalStake(uint8)` and selector `0xd5eccc05`
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
    pub struct GetCurrentTotalStakeReturn(pub u128);
    ///Container type for all return fields from the `getLatestStakeUpdate` function with signature `getLatestStakeUpdate(bytes32,uint8)` and selector `0xf851e198`
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
    pub struct GetLatestStakeUpdateReturn(pub StakeUpdate);
    ///Container type for all return fields from the `getStakeAtBlockNumber` function with signature `getStakeAtBlockNumber(bytes32,uint8,uint32)` and selector `0xfa28c627`
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
    pub struct GetStakeAtBlockNumberReturn(pub u128);
    ///Container type for all return fields from the `getStakeAtBlockNumberAndIndex` function with signature `getStakeAtBlockNumberAndIndex(uint8,uint32,bytes32,uint256)` and selector `0xf2be94ae`
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
    pub struct GetStakeAtBlockNumberAndIndexReturn(pub u128);
    ///Container type for all return fields from the `getStakeHistory` function with signature `getStakeHistory(bytes32,uint8)` and selector `0x2cd95940`
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
    pub struct GetStakeHistoryReturn(pub ::std::vec::Vec<StakeUpdate>);
    ///Container type for all return fields from the `getStakeHistoryLength` function with signature `getStakeHistoryLength(bytes32,uint8)` and selector `0x4bd26e09`
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
    pub struct GetStakeHistoryLengthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getStakeUpdateAtIndex` function with signature `getStakeUpdateAtIndex(uint8,bytes32,uint256)` and selector `0xac6bfb03`
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
    pub struct GetStakeUpdateAtIndexReturn(pub StakeUpdate);
    ///Container type for all return fields from the `getStakeUpdateIndexAtBlockNumber` function with signature `getStakeUpdateIndexAtBlockNumber(bytes32,uint8,uint32)` and selector `0xdd9846b9`
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
    pub struct GetStakeUpdateIndexAtBlockNumberReturn(pub u32);
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
    ///Container type for all return fields from the `getTotalStakeHistoryLength` function with signature `getTotalStakeHistoryLength(uint8)` and selector `0x0491b41c`
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
    pub struct GetTotalStakeHistoryLengthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getTotalStakeIndicesAtBlockNumber` function with signature `getTotalStakeIndicesAtBlockNumber(uint32,bytes)` and selector `0x81c07502`
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
    pub struct GetTotalStakeIndicesAtBlockNumberReturn(pub ::std::vec::Vec<u32>);
    ///Container type for all return fields from the `getTotalStakeUpdateAtIndex` function with signature `getTotalStakeUpdateAtIndex(uint8,uint256)` and selector `0xb6904b78`
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
    pub struct GetTotalStakeUpdateAtIndexReturn(pub StakeUpdate);
    ///Container type for all return fields from the `minimumStakeForQuorum` function with signature `minimumStakeForQuorum(uint8)` and selector `0xc46778a5`
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
    ///Container type for all return fields from the `recordOperatorStakeUpdate` function with signature `recordOperatorStakeUpdate(bytes32,uint8,uint96)` and selector `0x74454c6d`
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
    pub struct RecordOperatorStakeUpdateReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `registerOperator` function with signature `registerOperator(address,bytes32,bytes)` and selector `0x25504777`
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
    pub struct RegisterOperatorReturn(
        pub ::std::vec::Vec<u128>,
        pub ::std::vec::Vec<u128>,
    );
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
    ///Container type for all return fields from the `strategyParams` function with signature `strategyParams(uint8,uint256)` and selector `0x08732461`
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
    pub struct StrategyParamsReturn {
        pub strategy: ::ethers::core::types::Address,
        pub multiplier: u128,
    }
    ///Container type for all return fields from the `strategyParamsByIndex` function with signature `strategyParamsByIndex(uint8,uint256)` and selector `0xadc804da`
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
    pub struct StrategyParamsByIndexReturn(pub StrategyParams);
    ///Container type for all return fields from the `strategyParamsLength` function with signature `strategyParamsLength(uint8)` and selector `0x3ca5a5f5`
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
    pub struct StrategyParamsLengthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `updateOperatorStake` function with signature `updateOperatorStake(address,bytes32,bytes)` and selector `0x66acfefe`
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
    pub struct UpdateOperatorStakeReturn(pub ::ethers::core::types::U256);
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
