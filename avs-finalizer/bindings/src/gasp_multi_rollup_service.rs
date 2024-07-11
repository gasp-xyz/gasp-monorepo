pub use gasp_multi_rollup_service::*;
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
pub mod gasp_multi_rollup_service {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("checkSignatures"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("checkSignatures"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("msgHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ),
                                                        2usize,
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ),
                                                        2usize,
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IGaspMultiRollupServicePrimitives.NonSignerStakesAndSignatureForOldState",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                                ),
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IBLSSignatureChecker.QuorumStakeTotals",
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
                                    name: ::std::borrow::ToOwned::to_owned("_pauserRegistry"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IPauserRegistry"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("initialOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_updater"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("lastUpdateBlockTimestamp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "lastUpdateBlockTimestamp",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("latestCompletedTaskCreatedBlock"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "latestCompletedTaskCreatedBlock",
                            ),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("latestCompletedTaskNumber"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "latestCompletedTaskNumber",
                            ),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("latestPendingStateHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "latestPendingStateHash",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("operatorAndQuorumToStakes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "operatorAndQuorumToStakes",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
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
                    ::std::borrow::ToOwned::to_owned("operatorIdQuorumCount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "operatorIdQuorumCount",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
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
                    ::std::borrow::ToOwned::to_owned("pause"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pause"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newPausedStatus"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("pauseAll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pauseAll"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("paused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("paused"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("paused"),
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
                    ::std::borrow::ToOwned::to_owned("pauserRegistry"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pauserRegistry"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IPauserRegistry"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("process_eigen_reinit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "process_eigen_reinit",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("task"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IFinalizerTaskManager.Task",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("taskResponse"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IFinalizerTaskManager.TaskResponse",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operatorStateInfo"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                ],
                                                            ),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                ],
                                                            ),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                                ::std::boxed::Box::new(
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                                ),
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                                ::std::boxed::Box::new(
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                                                ),
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                                ::std::boxed::Box::new(
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                                ),
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                                ::std::boxed::Box::new(
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                                                ),
                                                            ),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IGaspMultiRollupServicePrimitives.OperatorStateInfo",
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
                    ::std::borrow::ToOwned::to_owned("process_eigen_update"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "process_eigen_update",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("task"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IFinalizerTaskManager.Task",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("taskResponse"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IFinalizerTaskManager.TaskResponse",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "nonSignerStakesAndSignatureForOldState",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ),
                                                        2usize,
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ),
                                                        2usize,
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IGaspMultiRollupServicePrimitives.NonSignerStakesAndSignatureForOldState",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operatorStateInfo"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                ],
                                                            ),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                ],
                                                            ),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                                ::std::boxed::Box::new(
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                                ),
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                                ::std::boxed::Box::new(
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                                                ),
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                                ::std::boxed::Box::new(
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                                ),
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                                ::std::boxed::Box::new(
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                                                ),
                                                            ),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IGaspMultiRollupServicePrimitives.OperatorStateInfo",
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
                    ::std::borrow::ToOwned::to_owned("qourumApk"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("qourumApk"),
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
                                    name: ::std::borrow::ToOwned::to_owned("X"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("Y"),
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
                    ::std::borrow::ToOwned::to_owned("quorumNumbers"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("quorumNumbers"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("quorumThresholdPercentage"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "quorumThresholdPercentage",
                            ),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("quorumToStakes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("quorumToStakes"),
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
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setPauserRegistry"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setPauserRegistry"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newPauserRegistry"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IPauserRegistry"),
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
                    ::std::borrow::ToOwned::to_owned("set_updater"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("set_updater"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_updater"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("stalled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("stalled"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("trySignatureAndApkVerification"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "trySignatureAndApkVerification",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("msgHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("apk"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct BN254.G1Point"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("apkG2"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                                2usize,
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                                2usize,
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct BN254.G2Point"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sigma"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct BN254.G1Point"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pairingSuccessful"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("siganatureIsValid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unpause"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("unpause"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newPausedStatus"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("updater"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updater"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("EigenReinitProcessed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "EigenReinitProcessed",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("taskNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("taskCreatedBlock"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EigenUpdateProcessed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "EigenUpdateProcessed",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("taskNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("taskCreatedBlock"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
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
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferred",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Paused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Paused"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newPausedStatus"),
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
                    ::std::borrow::ToOwned::to_owned("PauserRegistrySet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("PauserRegistrySet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pauserRegistry"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newPauserRegistry"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Unpaused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Unpaused"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newPausedStatus"),
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
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static GASPMULTIROLLUPSERVICE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[PaI=\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xC4W`\x005`\xE0\x1C\x80c\\\x97Z\xBB\x11a\0\xF9W\x80c\xC9'\xFE\xEF\x11a\0\x97W\x80c\xEDZ\x04\xFE\x11a\0qW\x80c\xEDZ\x04\xFE\x14a\x04zW\x80c\xF2\xFD\xE3\x8B\x14a\x04\x92W\x80c\xFA\xBC\x1C\xBC\x14a\x04\xA5W\x80c\xFCv]\xD5\x14a\x04\xB8W`\0\x80\xFD[\x80c\xC9'\xFE\xEF\x14a\x04KW\x80c\xDF\x03L\xD0\x14a\x04^W\x80c\xE6\x1D\xB1u\x14a\x04qW`\0\x80\xFD[\x80c\x88o\x11\x95\x11a\0\xD3W\x80c\x88o\x11\x95\x14a\x03\xDCW\x80c\x8D\xA5\xCB[\x14a\x04\x07W\x80c\xC0\xC5;\x8B\x14a\x04\x18W\x80c\xC4\xE1\x91L\x14a\x04+W`\0\x80\xFD[\x80c\\\x97Z\xBB\x14a\x03\xA3W\x80cqP\x18\xA6\x14a\x03\xABW\x80cz\xD7Ua\x14a\x03\xB3W`\0\x80\xFD[\x80cC\r;9\x11a\x01fW\x80cM\xEA\xBC!\x11a\x01@W\x80cM\xEA\xBC!\x14a\x03/W\x80cRn>d\x14a\x03TW\x80cY\\jg\x14a\x03xW\x80cZ\xC8j\xB7\x14a\x03\x80W`\0\x80\xFD[\x80cC\r;9\x14a\x02\x97W\x80cI\x9Do\xB6\x14a\x02\xCCW\x80cJ\xE6\xB2\x03\x14a\x03\x18W`\0\x80\xFD[\x80c\x13d9\xDD\x11a\x01\xA2W\x80c\x13d9\xDD\x14a\x022W\x80c\x17\x1F\x1D[\x14a\x02EW\x80c*\x84\x14\xFD\x14a\x02oW\x80c>@I&\x14a\x02\x84W`\0\x80\xFD[\x80c\x03\xD0\x97\xD2\x14a\x01\xC9W\x80c\x10\xD6z/\x14a\x02\nW\x80c\x12FH\xC9\x14a\x02\x1FW[`\0\x80\xFD[a\x01\xF0a\x01\xD76`\x04a7(V[`\x9F` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T\x82V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[a\x02\x1Da\x02\x186`\x04a7_V[a\x04\xC8V[\0[a\x02\x1Da\x02-6`\x04a7_V[a\x05\x84V[a\x02\x1Da\x02@6`\x04a7|V[a\x05\xAEV[a\x02Xa\x02S6`\x04a8\xF9V[a\x06\xEDV[`@\x80Q\x92\x15\x15\x83R\x90\x15\x15` \x83\x01R\x01a\x02\x01V[a\x02wa\x08wV[`@Qa\x02\x01\x91\x90a9JV[a\x02\x1Da\x02\x926`\x04a9\xDDV[a\t\x05V[a\x02\xBAa\x02\xA56`\x04a7|V[`\xA0` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02\x01V[a\x03\0a\x02\xDA6`\x04a:\x80V[`\x9E` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\x01`\x01``\x1B\x03\x16\x81V[`@Q`\x01`\x01``\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x01V[a\x03!`\x99T\x81V[`@Q\x90\x81R` \x01a\x02\x01V[`\x9CTa\x03?\x90c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\x01V[`\x97Ta\x03h\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02\x01V[a\x02\x1Da\x17AV[a\x03ha\x03\x8E6`\x04a7(V[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`fTa\x03!V[a\x02\x1Da\x18\x08V[a\x03\0a\x03\xC16`\x04a7(V[`\x9D` R`\0\x90\x81R`@\x90 T`\x01`\x01``\x1B\x03\x16\x81V[`eTa\x03\xEF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x01V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x03\xEFV[a\x02\x1Da\x04&6`\x04a:\xACV[a\x18\x1CV[a\x04>a\x0496`\x04a;\xD7V[a\x19MV[`@Qa\x02\x01\x91\x90a<aV[a\x02\x1Da\x04Y6`\x04a<\xA3V[a\x1F:V[`\x97Ta\x03\xEF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03!`\x98T\x81V[`\x9ATa\x03?\x90d\x01\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x02\x1Da\x04\xA06`\x04a7_V[a)\xFDV[a\x02\x1Da\x04\xB36`\x04a7|V[a*sV[`\x9ATa\x03?\x90c\xFF\xFF\xFF\xFF\x16\x81V[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x1BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05?\x91\x90a=\x18V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x05xW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05o\x90a=5V[`@Q\x80\x91\x03\x90\xFD[a\x05\x81\x81a+\xCFV[PV[a\x05\x8Ca,\xC6V[`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xF6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x1A\x91\x90a=\x8DV[a\x066W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05o\x90a=\xAAV[`fT\x81\x81\x16\x14a\x06\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05oV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`\0\x80`\0\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x87\x87`\0\x01Q\x88` \x01Q\x88`\0\x01Q`\0`\x02\x81\x10a\x075Wa\x075a=\xF2V[` \x02\x01Q\x89Q`\x01` \x02\x01Q\x8A` \x01Q`\0`\x02\x81\x10a\x07ZWa\x07Za=\xF2V[` \x02\x01Q\x8B` \x01Q`\x01`\x02\x81\x10a\x07vWa\x07va=\xF2V[` \x90\x81\x02\x91\x90\x91\x01Q\x8CQ\x8D\x83\x01Q`@Qa\x07\xD3\x9A\x99\x98\x97\x96\x95\x94\x01\x98\x89R` \x89\x01\x97\x90\x97R`@\x88\x01\x95\x90\x95R``\x87\x01\x93\x90\x93R`\x80\x86\x01\x91\x90\x91R`\xA0\x85\x01R`\xC0\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01Ra\x01 \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\0\x1Ca\x07\xF6\x91\x90a>\x08V[\x90Pa\x08ia\x08\x0Fa\x08\x08\x88\x84a- V[\x86\x90a-\xB7V[a\x08\x17a.KV[a\x08_a\x08P\x85a\x08J`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x01\x82R`\x02\x90\x82\x01R\x90V[\x90a- V[a\x08Y\x8Ca/\x0BV[\x90a-\xB7V[\x88b\x01\xD4\xC0a/\x9BV[\x90\x98\x90\x97P\x95PPPPPPV[`\x9B\x80Ta\x08\x84\x90a>*V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08\xB0\x90a>*V[\x80\x15a\x08\xFDW\x80`\x1F\x10a\x08\xD2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08\xFDV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x08\xE0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\x97T`\x01`\x01`\xA0\x1B\x03\x163\x14a\t_W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FUpdater must be the caller\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05oV[a\to`\x80\x85\x01``\x86\x01a>sV[`\x9ATd\x01\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14a\t\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7Freference block hash mismatch\0\0\0`D\x82\x01R`d\x01a\x05oV[\x83`@Q` \x01a\t\xE2\x91\x90a?\x03V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x83` \x015\x14a\nJW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FreferenceTaskHash hash mismatch\0`D\x82\x01R`d\x01a\x05oV[\x80`@Q` \x01a\n[\x91\x90aDoV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x83`@\x015\x14a\n\xC3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FoperatorStateInfo hash mismatch\0`D\x82\x01R`d\x01a\x05oV[`\x9ATd\x01\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x15a\x0C\x83Wa\n\xE9``\x85\x01`@\x86\x01a>sV[`\x9ATc\xFF\xFF\xFF\xFF\x91\x82\x16\x91a\x0B\n\x91d\x01\0\0\0\0\x90\x04\x16a8@aE\xECV[c\xFF\xFF\xFF\xFF\x16\x11a\x0BMW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x077F\x16\xC6R\x077F\x17FR\x03`\x9C\x1B`D\x82\x01R`d\x01a\x05oV[B`\x98Tb\x03\xF4\x80a\x0B_\x91\x90aF\x14V[\x11a\x0B\x9CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rlstale state 1`\x98\x1B`D\x82\x01R`d\x01a\x05oV[`\0a\x0B\xD2\x84`@Q` \x01a\x0B\xB2\x91\x90aF,V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84a\x049\x90aFyV[`\x9CT\x90\x91Pc\xFF\xFF\xFF\xFF\x16`\0[`\x9B\x80Ta\x0B\xEE\x90a>*V[\x90P\x81\x10\x15a\x0C\x7FW\x81`\xFF\x16\x83` \x01Q\x82\x81Q\x81\x10a\x0C\x11Wa\x0C\x11a=\xF2V[` \x02` \x01\x01Qa\x0C#\x91\x90aF\x85V[`\x01`\x01``\x1B\x03\x16`d\x84`\0\x01Q\x83\x81Q\x81\x10a\x0CDWa\x0CDa=\xF2V[` \x02` \x01\x01Q`\x01`\x01``\x1B\x03\x16a\x0C_\x91\x90aF\xB4V[\x10\x15a\x0CmWPPPa\x17;V[\x80a\x0Cw\x81aF\xD3V[\x91PPa\x0B\xE1V[PPP[`\0[a\x0C\x93` \x83\x01\x83aF\xEEV[\x90P\x81\x10\x15a\rXW`\x9D`\0a\x0C\xAD` \x85\x01\x85aF\xEEV[\x84\x81\x81\x10a\x0C\xBDWa\x0C\xBDa=\xF2V[\x90P` \x02\x01` \x81\x01\x90a\x0C\xD2\x91\x90a7(V[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0\x90\x81 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U`\x9F\x91a\r\x05\x90\x85\x01\x85aF\xEEV[\x84\x81\x81\x10a\r\x15Wa\r\x15a=\xF2V[\x90P` \x02\x01` \x81\x01\x90a\r*\x91\x90a7(V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x81\x81U`\x01\x01U\x80a\rP\x81aF\xD3V[\x91PPa\x0C\x86V[P`\0[a\ri`@\x83\x01\x83aG7V[\x90P\x81\x10\x15a\x0E\xA4Wa\r\x7F`@\x83\x01\x83aG7V[\x82\x81\x81\x10a\r\x8FWa\r\x8Fa=\xF2V[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\r\xA7\x91\x90aG\x80V[`\x9D`\0a\r\xB8`@\x86\x01\x86aG7V[\x85\x81\x81\x10a\r\xC8Wa\r\xC8a=\xF2V[a\r\xDE\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa7(V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x90\x81\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x17\x90\x91Ua\x0E\x1E\x90\x83\x01\x83aG7V[\x82\x81\x81\x10a\x0E.Wa\x0E.a=\xF2V[\x90P`\x80\x02\x01`@\x01`\x9F`\0\x84\x80`@\x01\x90a\x0EK\x91\x90aG7V[\x85\x81\x81\x10a\x0E[Wa\x0E[a=\xF2V[a\x0Eq\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa7(V[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a\x0E\x9C\x81aF\xD3V[\x91PPa\r\\V[P`\0[a\x0E\xB5``\x83\x01\x83aG\x9BV[\x90P\x81\x10\x15a\x0FmWa\x0E\xCB``\x83\x01\x83aG\x9BV[\x82\x81\x81\x10a\x0E\xDBWa\x0E\xDBa=\xF2V[\x90P`@\x02\x01` \x01` \x81\x01\x90a\x0E\xF3\x91\x90aG\x80V[`\x9D`\0a\x0F\x04``\x86\x01\x86aG\x9BV[\x85\x81\x81\x10a\x0F\x14Wa\x0F\x14a=\xF2V[a\x0F*\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa7(V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\x0Fe\x81aF\xD3V[\x91PPa\x0E\xA8V[P`\0[a\x0F~`\x80\x83\x01\x83aG\xE4V[\x90P\x81\x10\x15a\x10\x1AWa\x0F\x94`\x80\x83\x01\x83aG\xE4V[\x82\x81\x81\x10a\x0F\xA4Wa\x0F\xA4a=\xF2V[\x90P``\x02\x01` \x01`\x9F`\0\x84\x80`\x80\x01\x90a\x0F\xC1\x91\x90aG\xE4V[\x85\x81\x81\x10a\x0F\xD1Wa\x0F\xD1a=\xF2V[a\x0F\xE7\x92` ``\x90\x92\x02\x01\x90\x81\x01\x91Pa7(V[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a\x10\x12\x81aF\xD3V[\x91PPa\x0FqV[P`\0[a\x10+`\xA0\x83\x01\x83aF\xEEV[\x90P\x81\x10\x15a\x11QW`\0[`\x9B\x80Ta\x10D\x90a>*V[\x90P\x81\x10\x15a\x10\xFDW`\x9E`\0a\x10^`\xA0\x86\x01\x86aF\xEEV[\x85\x81\x81\x10a\x10nWa\x10na=\xF2V[\x90P` \x02\x015\x81R` \x01\x90\x81R` \x01`\0 `\0`\x9B\x83\x81Ta\x10\x93\x90a>*V[\x81\x10a\x10\xA1Wa\x10\xA1a=\xF2V[\x81T`\x01\x16\x15a\x10\xC0W\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U\x81a\x10\xF5\x81aF\xD3V[\x92PPa\x107V[P`\xA0`\0a\x11\x0E\x84\x83\x01\x85aF\xEEV[\x84\x81\x81\x10a\x11\x1EWa\x11\x1Ea=\xF2V[` \x90\x81\x02\x92\x90\x92\x015\x83RP\x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\xFF\x19\x16\x90U\x80a\x11I\x81aF\xD3V[\x91PPa\x10\x1EV[P`\0[a\x11b`\xC0\x83\x01\x83aF\xEEV[\x90P\x81\x10\x15a\x13\xB2Wa\x11x`\xC0\x83\x01\x83aF\xEEV[\x82\x81\x81\x10a\x11\x88Wa\x11\x88a=\xF2V[\x90P` \x02\x81\x01\x90a\x11\x9A\x91\x90aH,V[a\x11\xAB\x90`\x80\x81\x01\x90``\x01a7(V[`\xA0`\0a\x11\xBC`\xC0\x86\x01\x86aF\xEEV[\x85\x81\x81\x10a\x11\xCCWa\x11\xCCa=\xF2V[\x90P` \x02\x81\x01\x90a\x11\xDE\x91\x90aH,V[`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP`\0[a\x12\x18`\xC0\x84\x01\x84aF\xEEV[\x83\x81\x81\x10a\x12(Wa\x12(a=\xF2V[\x90P` \x02\x81\x01\x90a\x12:\x91\x90aH,V[a\x12H\x90` \x81\x01\x90aF\xEEV[\x90P\x81\x10\x15a\x13\x9FWa\x12^`\xC0\x84\x01\x84aF\xEEV[\x83\x81\x81\x10a\x12nWa\x12na=\xF2V[\x90P` \x02\x81\x01\x90a\x12\x80\x91\x90aH,V[a\x12\x8E\x90`@\x81\x01\x90aF\xEEV[\x82\x81\x81\x10a\x12\x9EWa\x12\x9Ea=\xF2V[\x90P` \x02\x01` \x81\x01\x90a\x12\xB3\x91\x90aG\x80V[`\x9E`\0a\x12\xC4`\xC0\x87\x01\x87aF\xEEV[\x86\x81\x81\x10a\x12\xD4Wa\x12\xD4a=\xF2V[\x90P` \x02\x81\x01\x90a\x12\xE6\x91\x90aH,V[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a\x13\x07`\xC0\x87\x01\x87aF\xEEV[\x86\x81\x81\x10a\x13\x17Wa\x13\x17a=\xF2V[\x90P` \x02\x81\x01\x90a\x13)\x91\x90aH,V[a\x137\x90` \x81\x01\x90aF\xEEV[\x85\x81\x81\x10a\x13GWa\x13Ga=\xF2V[\x90P` \x02\x01` \x81\x01\x90a\x13\\\x91\x90a7(V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\x13\x97\x81aF\xD3V[\x91PPa\x12\x0BV[P\x80a\x13\xAA\x81aF\xD3V[\x91PPa\x11UV[P`\0[a\x13\xC3`\xE0\x83\x01\x83aF\xEEV[\x90P\x81\x10\x15a\x15vW`\0[a\x13\xDC`\xE0\x84\x01\x84aF\xEEV[\x83\x81\x81\x10a\x13\xECWa\x13\xECa=\xF2V[\x90P` \x02\x81\x01\x90a\x13\xFE\x91\x90aHLV[a\x14\x0C\x90` \x81\x01\x90aF\xEEV[\x90P\x81\x10\x15a\x15cWa\x14\"`\xE0\x84\x01\x84aF\xEEV[\x83\x81\x81\x10a\x142Wa\x142a=\xF2V[\x90P` \x02\x81\x01\x90a\x14D\x91\x90aHLV[a\x14R\x90`@\x81\x01\x90aF\xEEV[\x82\x81\x81\x10a\x14bWa\x14ba=\xF2V[\x90P` \x02\x01` \x81\x01\x90a\x14w\x91\x90aG\x80V[`\x9E`\0a\x14\x88`\xE0\x87\x01\x87aF\xEEV[\x86\x81\x81\x10a\x14\x98Wa\x14\x98a=\xF2V[\x90P` \x02\x81\x01\x90a\x14\xAA\x91\x90aHLV[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a\x14\xCB`\xE0\x87\x01\x87aF\xEEV[\x86\x81\x81\x10a\x14\xDBWa\x14\xDBa=\xF2V[\x90P` \x02\x81\x01\x90a\x14\xED\x91\x90aHLV[a\x14\xFB\x90` \x81\x01\x90aF\xEEV[\x85\x81\x81\x10a\x15\x0BWa\x15\x0Ba=\xF2V[\x90P` \x02\x01` \x81\x01\x90a\x15 \x91\x90a7(V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\x15[\x81aF\xD3V[\x91PPa\x13\xCFV[P\x80a\x15n\x81aF\xD3V[\x91PPa\x13\xB6V[P`\0[a\x15\x88a\x01\0\x83\x01\x83aG\x9BV[\x90P\x81\x10\x15a\x16,Wa\x15\x9Fa\x01\0\x83\x01\x83aG\x9BV[\x82\x81\x81\x10a\x15\xAFWa\x15\xAFa=\xF2V[\x90P`@\x02\x01` \x01` \x81\x01\x90a\x15\xC7\x91\x90a7(V[`\xA0`\0a\x15\xD9a\x01\0\x86\x01\x86aG\x9BV[\x85\x81\x81\x10a\x15\xE9Wa\x15\xE9a=\xF2V[\x90P`@\x02\x01`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP\x80\x80a\x16$\x90aF\xD3V[\x91PPa\x15zV[P`\xA0\x83\x015`\x99Ua\x16B` \x85\x01\x85a>sV[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\x16l``\x85\x01`@\x86\x01a>sV[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90UB`\x98Ua\x16\xA4`\x80\x85\x01\x85aHbV[a\x16\xB0\x91`\x9B\x91a5\xA4V[Pa\x16\xC1`\xC0\x85\x01`\xA0\x86\x01a>sV[`\x9C\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x7F\x9A\x12\x8F\xE74\x7F\x1E\x11\xCA\"\xAA\x9D\xEBc.\xC9\xAB\xB0\x96\x08\xC1:\x99L`\xF7zV/F\x01qa\x17\t` \x86\x01\x86a>sV[a\x17\x19``\x87\x01`@\x88\x01a>sV[`@\x80Qc\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01[`@Q\x80\x91\x03\x90\xA1[PPPPV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\x89W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xAD\x91\x90a=\x8DV[a\x17\xC9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05o\x90a=\xAAV[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[a\x18\x10a,\xC6V[a\x18\x1A`\0a1\xBFV[V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x18<WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x18VWP0;\x15\x80\x15a\x18VWP`\0T`\xFF\x16`\x01\x14[a\x18\xB9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x05oV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x18\xDCW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x18\xE7\x84`\0a2\x11V[a\x18\xF0\x83a1\xBFV[`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90U\x80\x15a\x17;W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01a\x172V[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01\x81\x90R`\x9B\x80Ta\x19\x84\x90a>*V[\x90P\x90Pa\x19\xA5`@Q\x80`@\x01`@R\x80``\x81R` \x01``\x81RP\x90V[\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19\xBDWa\x19\xBDa7\x95V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x19\xE6W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P` \x82\x01R\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1A\x04Wa\x1A\x04a7\x95V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1A-W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x81R\x84QQ`\0\x90`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1AMWa\x1AMa7\x95V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1AvW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80[\x87QQ\x81\x10\x15a\x1C1Wa\x1A\xBF\x88`\0\x01Q\x82\x81Q\x81\x10a\x1A\xA0Wa\x1A\xA0a=\xF2V[` \x02` \x01\x01Q\x80Q`\0\x90\x81R` \x91\x82\x01Q\x90\x91R`@\x90 \x90V[\x83\x82\x81Q\x81\x10a\x1A\xD1Wa\x1A\xD1a=\xF2V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80\x15a\x1B\xAFW\x82a\x1A\xEE`\x01\x83aH\xA8V[\x81Q\x81\x10a\x1A\xFEWa\x1A\xFEa=\xF2V[` \x02` \x01\x01Q`\0\x1C\x83\x82\x81Q\x81\x10a\x1B\x1BWa\x1B\x1Ba=\xF2V[` \x02` \x01\x01Q`\0\x1C\x11a\x1B\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`M`$\x82\x01R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: nonSignerG1PubkeysForOldSta`d\x82\x01Rl\x1D\x19H\x1B\x9B\xDD\x08\x1C\xDB\xDC\x9D\x19Y`\x9A\x1B`\x84\x82\x01R`\xA4\x01a\x05oV[a\x1C\x1Da\x1C\x16`\xA0`\0\x86\x85\x81Q\x81\x10a\x1B\xCBWa\x1B\xCBa=\xF2V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82R\x81\x01\x91\x90\x91R`@\x01`\0 T\x8AQ\x80Q`\xFF\x90\x92\x16\x91\x85\x90\x81\x10a\x1C\0Wa\x1C\0a=\xF2V[` \x02` \x01\x01Qa2\xFB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x87\x90a-\xB7V[\x95P\x80a\x1C)\x81aF\xD3V[\x91PPa\x1A}V[Pa\x1C;\x85a3\xDFV[\x94P`\0[\x84\x81\x10\x15a\x1E\x1CW`\x9B\x81\x81Ta\x1CV\x90a>*V[\x81\x10a\x1CdWa\x1Cda=\xF2V[\x81T`\x01\x16\x15a\x1C\x83W\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C`\0\x81\x81R`\x9F` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x01T\x90\x82\x01R\x90\x92Pa\x1C\xC4\x90\x87\x90a-\xB7V[`\xFF\x83\x16`\0\x90\x81R`\x9D` \x90\x81R`@\x90\x91 T\x90\x86\x01Q\x80Q\x92\x98P`\x01`\x01``\x1B\x03\x90\x91\x16\x91\x83\x90\x81\x10a\x1C\xFFWa\x1C\xFFa=\xF2V[`\x01`\x01``\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x82\x01R\x84\x01Q\x80Q\x82\x90\x81\x10a\x1D+Wa\x1D+a=\xF2V[` \x02` \x01\x01Q\x84`\0\x01Q\x82\x81Q\x81\x10a\x1DIWa\x1DIa=\xF2V[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPP`\0[\x88QQ\x81\x10\x15a\x1E\tW`\x9E`\0\x85\x83\x81Q\x81\x10a\x1D\x8CWa\x1D\x8Ca=\xF2V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82R\x81\x81\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 `\xFF\x87\x16\x82R\x90\x92R\x90 T\x85Q\x80Q`\x01`\x01``\x1B\x03\x90\x92\x16\x91\x84\x90\x81\x10a\x1D\xD7Wa\x1D\xD7a=\xF2V[` \x02` \x01\x01\x81\x81Qa\x1D\xEB\x91\x90aH\xBFV[`\x01`\x01``\x1B\x03\x16\x90RP\x80a\x1E\x01\x81aF\xD3V[\x91PPa\x1DlV[P\x80a\x1E\x14\x81aF\xD3V[\x91PPa\x1C@V[P`\0\x80a\x1E4\x8A\x88\x8B` \x01Q\x8C`@\x01Qa\x06\xEDV[\x91P\x91P\x81a\x1E\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: pairing precompile call fai`d\x82\x01Rb\x1B\x19Y`\xEA\x1B`\x84\x82\x01R`\xA4\x01a\x05oV[\x80a\x1F*W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: signature is invalid\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05oV[P\x92\x95PPPPPP[\x92\x91PPV[a\x1FBa,\xC6V[`\0[a\x1FR` \x83\x01\x83aF\xEEV[\x90P\x81\x10\x15a \x17W`\x9D`\0a\x1Fl` \x85\x01\x85aF\xEEV[\x84\x81\x81\x10a\x1F|Wa\x1F|a=\xF2V[\x90P` \x02\x01` \x81\x01\x90a\x1F\x91\x91\x90a7(V[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0\x90\x81 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U`\x9F\x91a\x1F\xC4\x90\x85\x01\x85aF\xEEV[\x84\x81\x81\x10a\x1F\xD4Wa\x1F\xD4a=\xF2V[\x90P` \x02\x01` \x81\x01\x90a\x1F\xE9\x91\x90a7(V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x81\x81U`\x01\x01U\x80a \x0F\x81aF\xD3V[\x91PPa\x1FEV[P`\0[a (`@\x83\x01\x83aG7V[\x90P\x81\x10\x15a!cWa >`@\x83\x01\x83aG7V[\x82\x81\x81\x10a NWa Na=\xF2V[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a f\x91\x90aG\x80V[`\x9D`\0a w`@\x86\x01\x86aG7V[\x85\x81\x81\x10a \x87Wa \x87a=\xF2V[a \x9D\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa7(V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x90\x81\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x17\x90\x91Ua \xDD\x90\x83\x01\x83aG7V[\x82\x81\x81\x10a \xEDWa \xEDa=\xF2V[\x90P`\x80\x02\x01`@\x01`\x9F`\0\x84\x80`@\x01\x90a!\n\x91\x90aG7V[\x85\x81\x81\x10a!\x1AWa!\x1Aa=\xF2V[a!0\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa7(V[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a![\x81aF\xD3V[\x91PPa \x1BV[P`\0[a!t``\x83\x01\x83aG\x9BV[\x90P\x81\x10\x15a\",Wa!\x8A``\x83\x01\x83aG\x9BV[\x82\x81\x81\x10a!\x9AWa!\x9Aa=\xF2V[\x90P`@\x02\x01` \x01` \x81\x01\x90a!\xB2\x91\x90aG\x80V[`\x9D`\0a!\xC3``\x86\x01\x86aG\x9BV[\x85\x81\x81\x10a!\xD3Wa!\xD3a=\xF2V[a!\xE9\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa7(V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\"$\x81aF\xD3V[\x91PPa!gV[P`\0[a\"=`\x80\x83\x01\x83aG\xE4V[\x90P\x81\x10\x15a\"\xD9Wa\"S`\x80\x83\x01\x83aG\xE4V[\x82\x81\x81\x10a\"cWa\"ca=\xF2V[\x90P``\x02\x01` \x01`\x9F`\0\x84\x80`\x80\x01\x90a\"\x80\x91\x90aG\xE4V[\x85\x81\x81\x10a\"\x90Wa\"\x90a=\xF2V[a\"\xA6\x92` ``\x90\x92\x02\x01\x90\x81\x01\x91Pa7(V[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a\"\xD1\x81aF\xD3V[\x91PPa\"0V[P`\0[a\"\xEA`\xA0\x83\x01\x83aF\xEEV[\x90P\x81\x10\x15a$\x10W`\0[`\x9B\x80Ta#\x03\x90a>*V[\x90P\x81\x10\x15a#\xBCW`\x9E`\0a#\x1D`\xA0\x86\x01\x86aF\xEEV[\x85\x81\x81\x10a#-Wa#-a=\xF2V[\x90P` \x02\x015\x81R` \x01\x90\x81R` \x01`\0 `\0`\x9B\x83\x81Ta#R\x90a>*V[\x81\x10a#`Wa#`a=\xF2V[\x81T`\x01\x16\x15a#\x7FW\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U\x81a#\xB4\x81aF\xD3V[\x92PPa\"\xF6V[P`\xA0`\0a#\xCD\x84\x83\x01\x85aF\xEEV[\x84\x81\x81\x10a#\xDDWa#\xDDa=\xF2V[` \x90\x81\x02\x92\x90\x92\x015\x83RP\x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\xFF\x19\x16\x90U\x80a$\x08\x81aF\xD3V[\x91PPa\"\xDDV[P`\0[a$!`\xC0\x83\x01\x83aF\xEEV[\x90P\x81\x10\x15a&qWa$7`\xC0\x83\x01\x83aF\xEEV[\x82\x81\x81\x10a$GWa$Ga=\xF2V[\x90P` \x02\x81\x01\x90a$Y\x91\x90aH,V[a$j\x90`\x80\x81\x01\x90``\x01a7(V[`\xA0`\0a${`\xC0\x86\x01\x86aF\xEEV[\x85\x81\x81\x10a$\x8BWa$\x8Ba=\xF2V[\x90P` \x02\x81\x01\x90a$\x9D\x91\x90aH,V[`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP`\0[a$\xD7`\xC0\x84\x01\x84aF\xEEV[\x83\x81\x81\x10a$\xE7Wa$\xE7a=\xF2V[\x90P` \x02\x81\x01\x90a$\xF9\x91\x90aH,V[a%\x07\x90` \x81\x01\x90aF\xEEV[\x90P\x81\x10\x15a&^Wa%\x1D`\xC0\x84\x01\x84aF\xEEV[\x83\x81\x81\x10a%-Wa%-a=\xF2V[\x90P` \x02\x81\x01\x90a%?\x91\x90aH,V[a%M\x90`@\x81\x01\x90aF\xEEV[\x82\x81\x81\x10a%]Wa%]a=\xF2V[\x90P` \x02\x01` \x81\x01\x90a%r\x91\x90aG\x80V[`\x9E`\0a%\x83`\xC0\x87\x01\x87aF\xEEV[\x86\x81\x81\x10a%\x93Wa%\x93a=\xF2V[\x90P` \x02\x81\x01\x90a%\xA5\x91\x90aH,V[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a%\xC6`\xC0\x87\x01\x87aF\xEEV[\x86\x81\x81\x10a%\xD6Wa%\xD6a=\xF2V[\x90P` \x02\x81\x01\x90a%\xE8\x91\x90aH,V[a%\xF6\x90` \x81\x01\x90aF\xEEV[\x85\x81\x81\x10a&\x06Wa&\x06a=\xF2V[\x90P` \x02\x01` \x81\x01\x90a&\x1B\x91\x90a7(V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a&V\x81aF\xD3V[\x91PPa$\xCAV[P\x80a&i\x81aF\xD3V[\x91PPa$\x14V[P`\0[a&\x82`\xE0\x83\x01\x83aF\xEEV[\x90P\x81\x10\x15a(5W`\0[a&\x9B`\xE0\x84\x01\x84aF\xEEV[\x83\x81\x81\x10a&\xABWa&\xABa=\xF2V[\x90P` \x02\x81\x01\x90a&\xBD\x91\x90aHLV[a&\xCB\x90` \x81\x01\x90aF\xEEV[\x90P\x81\x10\x15a(\"Wa&\xE1`\xE0\x84\x01\x84aF\xEEV[\x83\x81\x81\x10a&\xF1Wa&\xF1a=\xF2V[\x90P` \x02\x81\x01\x90a'\x03\x91\x90aHLV[a'\x11\x90`@\x81\x01\x90aF\xEEV[\x82\x81\x81\x10a'!Wa'!a=\xF2V[\x90P` \x02\x01` \x81\x01\x90a'6\x91\x90aG\x80V[`\x9E`\0a'G`\xE0\x87\x01\x87aF\xEEV[\x86\x81\x81\x10a'WWa'Wa=\xF2V[\x90P` \x02\x81\x01\x90a'i\x91\x90aHLV[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a'\x8A`\xE0\x87\x01\x87aF\xEEV[\x86\x81\x81\x10a'\x9AWa'\x9Aa=\xF2V[\x90P` \x02\x81\x01\x90a'\xAC\x91\x90aHLV[a'\xBA\x90` \x81\x01\x90aF\xEEV[\x85\x81\x81\x10a'\xCAWa'\xCAa=\xF2V[\x90P` \x02\x01` \x81\x01\x90a'\xDF\x91\x90a7(V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a(\x1A\x81aF\xD3V[\x91PPa&\x8EV[P\x80a(-\x81aF\xD3V[\x91PPa&uV[P`\0[a(Ga\x01\0\x83\x01\x83aG\x9BV[\x90P\x81\x10\x15a(\xEBWa(^a\x01\0\x83\x01\x83aG\x9BV[\x82\x81\x81\x10a(nWa(na=\xF2V[\x90P`@\x02\x01` \x01` \x81\x01\x90a(\x86\x91\x90a7(V[`\xA0`\0a(\x98a\x01\0\x86\x01\x86aG\x9BV[\x85\x81\x81\x10a(\xA8Wa(\xA8a=\xF2V[\x90P`@\x02\x01`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP\x80\x80a(\xE3\x90aF\xD3V[\x91PPa(9V[P`\xA0\x82\x015`\x99Ua)\x01` \x84\x01\x84a>sV[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua)+``\x84\x01`@\x85\x01a>sV[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90UB`\x98Ua)c`\x80\x84\x01\x84aHbV[a)o\x91`\x9B\x91a5\xA4V[Pa)\x80`\xC0\x84\x01`\xA0\x85\x01a>sV[`\x9C\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x7F&Ie\xEBk\xC46\xC6\xC4sC\x1D4\xAFV\xE82\xEC4O\xDF\xD4>\xE6\xAFo\xCEm ^\x84\xAFa)\xC8` \x85\x01\x85a>sV[a)\xD8``\x86\x01`@\x87\x01a>sV[`@\x80Qc\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1PPPV[a*\x05a,\xC6V[`\x01`\x01`\xA0\x1B\x03\x81\x16a*jW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x05oV[a\x05\x81\x81a1\xBFV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a*\xC6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*\xEA\x91\x90a=\x18V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a+\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05o\x90a=5V[`fT\x19\x81\x19`fT\x19\x16\x14a+\x98W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05oV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x06\xE2V[`\x01`\x01`\xA0\x1B\x03\x81\x16a,]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x05oV[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x18\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x05oV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra-<a6(V[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x80\x82\x01\x84\x90R`\0\x90\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a-oWa-qV[\xFE[P\x80a-\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCB[][\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x05oV[PP\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra-\xD3a6FV[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R\x83Q`@\x80\x84\x01\x91\x90\x91R\x90\x84\x01Q``\x83\x01R`\0\x90\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a-oWP\x80a-\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCBXY\x19\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x05oV[a.Sa6dV[P`@\x80Q`\x80\x81\x01\x82R\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81\x83\x01\x90\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED``\x83\x01R\x81R\x81Q\x80\x83\x01\x90\x92R\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x81\x01\x91\x90\x91R\x81\x01\x91\x90\x91R\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0\x80\x80a/;`\0\x80Q` aH\xE8\x839\x81Q\x91R\x86a>\x08V[\x90P[a/G\x81a4zV[\x90\x93P\x91P`\0\x80Q` aH\xE8\x839\x81Q\x91R\x82\x83\t\x83\x14\x15a/\x81W`@\x80Q\x80\x82\x01\x90\x91R\x90\x81R` \x81\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80Q` aH\xE8\x839\x81Q\x91R`\x01\x82\x08\x90Pa/>V[`@\x80Q\x80\x82\x01\x82R\x86\x81R` \x80\x82\x01\x86\x90R\x82Q\x80\x84\x01\x90\x93R\x86\x83R\x82\x01\x84\x90R`\0\x91\x82\x91\x90a/\xCDa6\x89V[`\0[`\x02\x81\x10\x15a1\x92W`\0a/\xE6\x82`\x06aF\xB4V[\x90P\x84\x82`\x02\x81\x10a/\xFAWa/\xFAa=\xF2V[` \x02\x01QQ\x83a0\x0C\x83`\0aF\x14V[`\x0C\x81\x10a0\x1CWa0\x1Ca=\xF2V[` \x02\x01R\x84\x82`\x02\x81\x10a03Wa03a=\xF2V[` \x02\x01Q` \x01Q\x83\x82`\x01a0J\x91\x90aF\x14V[`\x0C\x81\x10a0ZWa0Za=\xF2V[` \x02\x01R\x83\x82`\x02\x81\x10a0qWa0qa=\xF2V[` \x02\x01QQQ\x83a0\x84\x83`\x02aF\x14V[`\x0C\x81\x10a0\x94Wa0\x94a=\xF2V[` \x02\x01R\x83\x82`\x02\x81\x10a0\xABWa0\xABa=\xF2V[` \x02\x01QQ`\x01` \x02\x01Q\x83a0\xC4\x83`\x03aF\x14V[`\x0C\x81\x10a0\xD4Wa0\xD4a=\xF2V[` \x02\x01R\x83\x82`\x02\x81\x10a0\xEBWa0\xEBa=\xF2V[` \x02\x01Q` \x01Q`\0`\x02\x81\x10a1\x06Wa1\x06a=\xF2V[` \x02\x01Q\x83a1\x17\x83`\x04aF\x14V[`\x0C\x81\x10a1'Wa1'a=\xF2V[` \x02\x01R\x83\x82`\x02\x81\x10a1>Wa1>a=\xF2V[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a1YWa1Ya=\xF2V[` \x02\x01Q\x83a1j\x83`\x05aF\x14V[`\x0C\x81\x10a1zWa1za=\xF2V[` \x02\x01RP\x80a1\x8A\x81aF\xD3V[\x91PPa/\xD0V[Pa1\x9Ba6\xA8V[`\0` \x82a\x01\x80\x85`\x08\x8C\xFA\x91Q\x91\x9C\x91\x15\x15\x9BP\x90\x99PPPPPPPPPPV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a22WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a2\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x05oV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a2\xF7\x82a+\xCFV[PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x02\0\x82a\xFF\xFF\x16\x10a3WW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Roscalar-too-large`\x80\x1B`D\x82\x01R`d\x01a\x05oV[\x81a\xFF\xFF\x16`\x01\x14\x15a3kWP\x81a\x1F4V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01\x81\x90R\x84\x90`\x01\x90[\x81a\xFF\xFF\x16\x86a\xFF\xFF\x16\x10a3\xD4W`\x01a\xFF\xFF\x87\x16`\xFF\x83\x16\x1C\x81\x16\x14\x15a3\xB7Wa3\xB4\x84\x84a-\xB7V[\x93P[a3\xC1\x83\x84a-\xB7V[\x92Pb\x01\xFF\xFE`\x01\x92\x83\x1B\x16\x91\x01a3\x87V[P\x91\x95\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q\x15\x80\x15a4\x04WP` \x82\x01Q\x15[\x15a4\"WPP`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01`\0\x80Q` aH\xE8\x839\x81Q\x91R\x84` \x01Qa4U\x91\x90a>\x08V[a4m\x90`\0\x80Q` aH\xE8\x839\x81Q\x91RaH\xA8V[\x90R\x92\x91PPV[\x91\x90PV[`\0\x80\x80`\0\x80Q` aH\xE8\x839\x81Q\x91R`\x03`\0\x80Q` aH\xE8\x839\x81Q\x91R\x86`\0\x80Q` aH\xE8\x839\x81Q\x91R\x88\x89\t\t\x08\x90P`\0a4\xF0\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\0\x80Q` aH\xE8\x839\x81Q\x91Ra4\xFCV[\x91\x95\x91\x94P\x90\x92PPPV[`\0\x80a5\x07a6\xA8V[a5\x0Fa6\xC6V[` \x80\x82R\x81\x81\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x88\x90R`\x80\x82\x01\x87\x90R`\xA0\x82\x01\x86\x90R\x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82\x80\x15a-oWP\x82a5\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FBN254.expMod: call failure\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05oV[PQ\x95\x94PPPPPV[\x82\x80Ta5\xB0\x90a>*V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82a5\xD2W`\0\x85Ua6\x18V[\x82`\x1F\x10a5\xEBW\x82\x80\x01`\xFF\x19\x825\x16\x17\x85Ua6\x18V[\x82\x80\x01`\x01\x01\x85U\x82\x15a6\x18W\x91\x82\x01[\x82\x81\x11\x15a6\x18W\x825\x82U\x91` \x01\x91\x90`\x01\x01\x90a5\xFDV[Pa6$\x92\x91Pa6\xE4V[P\x90V[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80a6wa6\xF9V[\x81R` \x01a6\x84a6\xF9V[\x90R\x90V[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[[\x80\x82\x11\x15a6$W`\0\x81U`\x01\x01a6\xE5V[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[\x805`\xFF\x81\x16\x81\x14a4uW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a7:W`\0\x80\xFD[a7C\x82a7\x17V[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05\x81W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a7qW`\0\x80\xFD[\x815a7C\x81a7JV[`\0` \x82\x84\x03\x12\x15a7\x8EW`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a7\xCDWa7\xCDa7\x95V[`@R\x90V[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a7\xCDWa7\xCDa7\x95V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a8\x1DWa8\x1Da7\x95V[`@R\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a87W`\0\x80\xFD[a8?a7\xABV[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a8fW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a8\x88Wa8\x88a7\x95V[\x80`@RP\x80`@\x84\x01\x85\x81\x11\x15a8\x9FW`\0\x80\xFD[\x84[\x81\x81\x10\x15a3\xD4W\x805\x83R` \x92\x83\x01\x92\x01a8\xA1V[`\0`\x80\x82\x84\x03\x12\x15a8\xCBW`\0\x80\xFD[a8\xD3a7\xABV[\x90Pa8\xDF\x83\x83a8UV[\x81Ra8\xEE\x83`@\x84\x01a8UV[` \x82\x01R\x92\x91PPV[`\0\x80`\0\x80a\x01 \x85\x87\x03\x12\x15a9\x10W`\0\x80\xFD[\x845\x93Pa9!\x86` \x87\x01a8%V[\x92Pa90\x86``\x87\x01a8\xB9V[\x91Pa9?\x86`\xE0\x87\x01a8%V[\x90P\x92\x95\x91\x94P\x92PV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a9wW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a9[V[\x81\x81\x11\x15a9\x89W`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[`\0a\x01\0\x82\x84\x03\x12\x15a9\xB2W`\0\x80\xFD[P\x91\x90PV[`\0`\xC0\x82\x84\x03\x12\x15a9\xB2W`\0\x80\xFD[`\0a\x01 \x82\x84\x03\x12\x15a9\xB2W`\0\x80\xFD[`\0\x80`\0\x80a\x01 \x85\x87\x03\x12\x15a9\xF4W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a:\x0BW`\0\x80\xFD[a:\x17\x88\x83\x89\x01a9\x9FV[\x95Pa:&\x88` \x89\x01a9\xB8V[\x94P`\xE0\x87\x015\x91P\x80\x82\x11\x15a:<W`\0\x80\xFD[\x90\x86\x01\x90`\xE0\x82\x89\x03\x12\x15a:PW`\0\x80\xFD[\x90\x92Pa\x01\0\x86\x015\x90\x80\x82\x11\x15a:gW`\0\x80\xFD[Pa:t\x87\x82\x88\x01a9\xCAV[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a:\x93W`\0\x80\xFD[\x825\x91Pa:\xA3` \x84\x01a7\x17V[\x90P\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a:\xC1W`\0\x80\xFD[\x835a:\xCC\x81a7JV[\x92P` \x84\x015a:\xDC\x81a7JV[\x91P`@\x84\x015a:\xEC\x81a7JV[\x80\x91PP\x92P\x92P\x92V[`\0`\xE0\x82\x84\x03\x12\x15a;\tW`\0\x80\xFD[a;\x11a7\xD3V[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a;*W`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a;>W`\0\x80\xFD[\x815` \x82\x82\x11\x15a;RWa;Ra7\x95V[a;`\x81\x83`\x05\x1B\x01a7\xF5V[\x82\x81R\x81\x81\x01\x93P`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x87\x83\x11\x15a;\x80W`\0\x80\xFD[\x93\x81\x01\x93[\x82\x85\x10\x15a;\xA9Wa;\x97\x88\x86a8%V[\x84R\x81\x84\x01\x93P`@\x85\x01\x94Pa;\x85V[\x85Ra;\xB7\x87\x87\x83\x01a8\xB9V[\x81\x86\x01RPPPPa;\xCC\x83`\xA0\x84\x01a8%V[`@\x82\x01R\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a;\xEAW`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a<\x07W`\0\x80\xFD[a<\x13\x85\x82\x86\x01a:\xF7V[\x91PP\x92P\x92\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a<VW\x81Q`\x01`\x01``\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a<1V[P\x94\x95\x94PPPPPV[` \x81R`\0\x82Q`@` \x84\x01Ra<}``\x84\x01\x82a<\x1DV[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01Ra<\x9A\x82\x82a<\x1DV[\x95\x94PPPPPV[`\0\x80`\0a\x01\0\x84\x86\x03\x12\x15a<\xB9W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a<\xD0W`\0\x80\xFD[a<\xDC\x87\x83\x88\x01a9\x9FV[\x94Pa<\xEB\x87` \x88\x01a9\xB8V[\x93P`\xE0\x86\x015\x91P\x80\x82\x11\x15a=\x01W`\0\x80\xFD[Pa=\x0E\x86\x82\x87\x01a9\xCAV[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a=*W`\0\x80\xFD[\x81Qa7C\x81a7JV[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[\x80\x15\x15\x81\x14a\x05\x81W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a=\x9FW`\0\x80\xFD[\x81Qa7C\x81a=\x7FV[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x82a>%WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80a>>W`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15a9\xB2WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a4uW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a>\x85W`\0\x80\xFD[a7C\x82a>_V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a>\xA5W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a>\xC4W`\0\x80\xFD[\x806\x03\x83\x13\x15a>\xD3W`\0\x80\xFD[\x92P\x92\x90PV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x81Rc\xFF\xFF\xFF\xFFa?\x15\x83a>_V[\x16` \x82\x01R` \x82\x015`@\x82\x01R`\0a?3`@\x84\x01a>_V[c\xFF\xFF\xFF\xFF\x81\x16``\x84\x01RPa?L``\x84\x01a>_V[c\xFF\xFF\xFF\xFF\x81\x16`\x80\x84\x01RPa?f`\x80\x84\x01\x84a>\x8EV[a\x01\0\x80`\xA0\x86\x01Ra?~a\x01 \x86\x01\x83\x85a>\xDAV[\x92Pa?\x8C`\xA0\x87\x01a>_V[c\xFF\xFF\xFF\xFF\x81\x16`\xC0\x87\x01R\x91Pa?\xA7`\xC0\x87\x01\x87a>\x8EV[\x86\x85\x03`\x1F\x19\x01`\xE0\x88\x01R\x92Pa?\xC0\x84\x84\x83a>\xDAV[\x93PPa?\xCF`\xE0\x87\x01a>_V[c\xFF\xFF\xFF\xFF\x16\x94\x01\x93\x90\x93R\x93\x92PPPV[\x805a4u\x81a=\x7FV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a@\x04W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a@#W`\0\x80\xFD[\x80`\x05\x1B6\x03\x83\x13\x15a>\xD3W`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a<VW`\xFFa@X\x83a7\x17V[\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a@EV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a@\x82W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a@\xA1W`\0\x80\xFD[\x80`\x07\x1B6\x03\x83\x13\x15a>\xD3W`\0\x80\xFD[\x805`\x01`\x01``\x1B\x03\x81\x16\x81\x14a4uW`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a<VW`\xFFa@\xED\x83a7\x17V[\x16\x87R`\x01`\x01``\x1B\x03aA\x03\x84\x84\x01a@\xB3V[\x16\x83\x88\x01R`@\x82\x81\x015\x90\x88\x01R``\x80\x83\x015\x90\x88\x01R`\x80\x96\x87\x01\x96\x90\x91\x01\x90`\x01\x01a@\xDAV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aAEW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aAdW`\0\x80\xFD[\x80`\x06\x1B6\x03\x83\x13\x15a>\xD3W`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a<VW`\xFFaA\x99\x83a7\x17V[\x16\x87R`\x01`\x01``\x1B\x03aA\xAF\x84\x84\x01a@\xB3V[\x16\x87\x84\x01R`@\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aA\x86V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aA\xDEW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aA\xFDW`\0\x80\xFD[``\x81\x026\x03\x83\x13\x15a>\xD3W`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a<VW`\xFFaB2\x83a7\x17V[\x16\x87RaBM\x83\x88\x01\x84\x84\x01\x805\x82R` \x90\x81\x015\x91\x01RV[``\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aB\x1FV[\x81\x83R`\0`\x01`\x01`\xFB\x1B\x03\x83\x11\x15aByW`\0\x80\xFD[\x82`\x05\x1B\x80\x83` \x87\x017`\0\x94\x01` \x01\x93\x84RP\x91\x92\x91PPV[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a<VW`\x01`\x01``\x1B\x03aB\xBF\x83a@\xB3V[\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aB\xA6V[\x81\x83R`\0` \x80\x85\x01\x80\x81\x96P\x85`\x05\x1B\x81\x01\x91P\x84`\0\x80[\x88\x81\x10\x15aC\x83W\x83\x85\x03\x8AR\x825`~\x19\x896\x03\x01\x81\x12aC\rW\x82\x83\xFD[\x88\x01\x805\x86R`\x80aC!\x88\x83\x01\x83a?\xEDV[\x82\x8A\x8A\x01RaC3\x83\x8A\x01\x82\x84a@5V[\x92PPP`@aCE\x81\x84\x01\x84a?\xEDV[\x89\x84\x03\x83\x8B\x01RaCW\x84\x82\x84aB\x96V[\x93PPPP```\xFFaCk\x82\x85\x01a7\x17V[\x16\x97\x01\x96\x90\x96RP\x98\x85\x01\x98\x91\x85\x01\x91`\x01\x01aB\xEDV[P\x92\x98\x97PPPPPPPPV[\x81\x83R`\0` \x80\x85\x01\x80\x81\x96P\x85`\x05\x1B\x81\x01\x91P\x84`\0\x80[\x88\x81\x10\x15aC\x83W\x83\x85\x03\x8AR\x825`^\x19\x896\x03\x01\x81\x12aC\xCCW\x82\x83\xFD[\x88\x01\x805\x86R``aC\xE0\x88\x83\x01\x83a?\xEDV[\x82\x8A\x8A\x01RaC\xF2\x83\x8A\x01\x82\x84a@5V[\x92PPP`@aD\x04\x81\x84\x01\x84a?\xEDV[\x93P\x88\x83\x03\x82\x8A\x01RaD\x18\x83\x85\x83aB\x96V[\x9D\x8A\x01\x9D\x98PPP\x93\x87\x01\x93PP`\x01\x01aC\xACV[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a<VW\x815\x87R`\xFFaDW\x84\x84\x01a7\x17V[\x16\x87\x84\x01R`@\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aD>V[` \x81RaD\x89` \x82\x01aD\x83\x84a?\xE2V[\x15\x15\x90RV[`\0aD\x98` \x84\x01\x84a?\xEDV[a\x01 \x80`@\x86\x01RaD\xB0a\x01@\x86\x01\x83\x85a@5V[\x92PaD\xBF`@\x87\x01\x87a@kV[\x92P`\x1F\x19\x80\x87\x86\x03\x01``\x88\x01RaD\xD9\x85\x85\x84a@\xCAV[\x94PaD\xE8``\x89\x01\x89aA.V[\x94P\x91P\x80\x87\x86\x03\x01`\x80\x88\x01RaE\x01\x85\x85\x84aAvV[\x94PaE\x10`\x80\x89\x01\x89aA\xC7V[\x94P\x91P\x80\x87\x86\x03\x01`\xA0\x88\x01RaE)\x85\x85\x84aB\x0FV[\x94PaE8`\xA0\x89\x01\x89a?\xEDV[\x94P\x91P\x80\x87\x86\x03\x01`\xC0\x88\x01RaEQ\x85\x85\x84aB`V[\x94PaE``\xC0\x89\x01\x89a?\xEDV[\x94P\x91P\x80\x87\x86\x03\x01`\xE0\x88\x01RaEy\x85\x85\x84aB\xD2V[\x94PaE\x88`\xE0\x89\x01\x89a?\xEDV[\x94P\x91Pa\x01\0\x81\x88\x87\x03\x01\x81\x89\x01RaE\xA3\x86\x86\x85aC\x91V[\x95PaE\xB1\x81\x8A\x01\x8AaA.V[\x95P\x92PP\x80\x87\x86\x03\x01\x83\x88\x01RPaE\xCB\x84\x84\x83aD.V[\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aF\x0BWaF\x0BaE\xD6V[\x01\x94\x93PPPPV[`\0\x82\x19\x82\x11\x15aF'WaF'aE\xD6V[P\x01\x90V[`\xC0\x81\x01c\xFF\xFF\xFF\xFFaF>\x84a>_V[\x16\x82R` \x83\x015` \x83\x01R`@\x83\x015`@\x83\x01R``\x83\x015``\x83\x01R`\x80\x83\x015`\x80\x83\x01R`\xA0\x83\x015`\xA0\x83\x01R\x92\x91PPV[`\0a\x1F46\x83a:\xF7V[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aF\xABWaF\xABaE\xD6V[\x02\x94\x93PPPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15aF\xCEWaF\xCEaE\xD6V[P\x02\x90V[`\0`\0\x19\x82\x14\x15aF\xE7WaF\xE7aE\xD6V[P`\x01\x01\x90V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aG\x05W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aG\x1FW`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a>\xD3W`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aGNW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aGhW`\0\x80\xFD[` \x01\x91P`\x07\x81\x90\x1B6\x03\x82\x13\x15a>\xD3W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aG\x92W`\0\x80\xFD[a7C\x82a@\xB3V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aG\xB2W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aG\xCCW`\0\x80\xFD[` \x01\x91P`\x06\x81\x90\x1B6\x03\x82\x13\x15a>\xD3W`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aG\xFBW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aH\x15W`\0\x80\xFD[` \x01\x91P``\x81\x026\x03\x82\x13\x15a>\xD3W`\0\x80\xFD[`\0\x825`~\x19\x836\x03\x01\x81\x12aHBW`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x825`^\x19\x836\x03\x01\x81\x12aHBW`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aHyW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aH\x93W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a>\xD3W`\0\x80\xFD[`\0\x82\x82\x10\x15aH\xBAWaH\xBAaE\xD6V[P\x03\x90V[`\0`\x01`\x01``\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15aH\xDFWaH\xDFaE\xD6V[\x03\x93\x92PPPV\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 ;@b\x14\xB2\x92\xF3xve\xF4W\xC4 \x87x$\xB4|\x9F\xD5[\xF1J\xF9\xF9\x89\xE1\xC7\xF5\xE4=dsolcC\0\x08\x0C\x003";
    /// The bytecode of the contract.
    pub static GASPMULTIROLLUPSERVICE_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xC4W`\x005`\xE0\x1C\x80c\\\x97Z\xBB\x11a\0\xF9W\x80c\xC9'\xFE\xEF\x11a\0\x97W\x80c\xEDZ\x04\xFE\x11a\0qW\x80c\xEDZ\x04\xFE\x14a\x04zW\x80c\xF2\xFD\xE3\x8B\x14a\x04\x92W\x80c\xFA\xBC\x1C\xBC\x14a\x04\xA5W\x80c\xFCv]\xD5\x14a\x04\xB8W`\0\x80\xFD[\x80c\xC9'\xFE\xEF\x14a\x04KW\x80c\xDF\x03L\xD0\x14a\x04^W\x80c\xE6\x1D\xB1u\x14a\x04qW`\0\x80\xFD[\x80c\x88o\x11\x95\x11a\0\xD3W\x80c\x88o\x11\x95\x14a\x03\xDCW\x80c\x8D\xA5\xCB[\x14a\x04\x07W\x80c\xC0\xC5;\x8B\x14a\x04\x18W\x80c\xC4\xE1\x91L\x14a\x04+W`\0\x80\xFD[\x80c\\\x97Z\xBB\x14a\x03\xA3W\x80cqP\x18\xA6\x14a\x03\xABW\x80cz\xD7Ua\x14a\x03\xB3W`\0\x80\xFD[\x80cC\r;9\x11a\x01fW\x80cM\xEA\xBC!\x11a\x01@W\x80cM\xEA\xBC!\x14a\x03/W\x80cRn>d\x14a\x03TW\x80cY\\jg\x14a\x03xW\x80cZ\xC8j\xB7\x14a\x03\x80W`\0\x80\xFD[\x80cC\r;9\x14a\x02\x97W\x80cI\x9Do\xB6\x14a\x02\xCCW\x80cJ\xE6\xB2\x03\x14a\x03\x18W`\0\x80\xFD[\x80c\x13d9\xDD\x11a\x01\xA2W\x80c\x13d9\xDD\x14a\x022W\x80c\x17\x1F\x1D[\x14a\x02EW\x80c*\x84\x14\xFD\x14a\x02oW\x80c>@I&\x14a\x02\x84W`\0\x80\xFD[\x80c\x03\xD0\x97\xD2\x14a\x01\xC9W\x80c\x10\xD6z/\x14a\x02\nW\x80c\x12FH\xC9\x14a\x02\x1FW[`\0\x80\xFD[a\x01\xF0a\x01\xD76`\x04a7(V[`\x9F` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T\x82V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[a\x02\x1Da\x02\x186`\x04a7_V[a\x04\xC8V[\0[a\x02\x1Da\x02-6`\x04a7_V[a\x05\x84V[a\x02\x1Da\x02@6`\x04a7|V[a\x05\xAEV[a\x02Xa\x02S6`\x04a8\xF9V[a\x06\xEDV[`@\x80Q\x92\x15\x15\x83R\x90\x15\x15` \x83\x01R\x01a\x02\x01V[a\x02wa\x08wV[`@Qa\x02\x01\x91\x90a9JV[a\x02\x1Da\x02\x926`\x04a9\xDDV[a\t\x05V[a\x02\xBAa\x02\xA56`\x04a7|V[`\xA0` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02\x01V[a\x03\0a\x02\xDA6`\x04a:\x80V[`\x9E` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\x01`\x01``\x1B\x03\x16\x81V[`@Q`\x01`\x01``\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x01V[a\x03!`\x99T\x81V[`@Q\x90\x81R` \x01a\x02\x01V[`\x9CTa\x03?\x90c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\x01V[`\x97Ta\x03h\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02\x01V[a\x02\x1Da\x17AV[a\x03ha\x03\x8E6`\x04a7(V[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`fTa\x03!V[a\x02\x1Da\x18\x08V[a\x03\0a\x03\xC16`\x04a7(V[`\x9D` R`\0\x90\x81R`@\x90 T`\x01`\x01``\x1B\x03\x16\x81V[`eTa\x03\xEF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x01V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x03\xEFV[a\x02\x1Da\x04&6`\x04a:\xACV[a\x18\x1CV[a\x04>a\x0496`\x04a;\xD7V[a\x19MV[`@Qa\x02\x01\x91\x90a<aV[a\x02\x1Da\x04Y6`\x04a<\xA3V[a\x1F:V[`\x97Ta\x03\xEF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03!`\x98T\x81V[`\x9ATa\x03?\x90d\x01\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x02\x1Da\x04\xA06`\x04a7_V[a)\xFDV[a\x02\x1Da\x04\xB36`\x04a7|V[a*sV[`\x9ATa\x03?\x90c\xFF\xFF\xFF\xFF\x16\x81V[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x1BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05?\x91\x90a=\x18V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x05xW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05o\x90a=5V[`@Q\x80\x91\x03\x90\xFD[a\x05\x81\x81a+\xCFV[PV[a\x05\x8Ca,\xC6V[`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xF6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x1A\x91\x90a=\x8DV[a\x066W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05o\x90a=\xAAV[`fT\x81\x81\x16\x14a\x06\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05oV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`\0\x80`\0\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x87\x87`\0\x01Q\x88` \x01Q\x88`\0\x01Q`\0`\x02\x81\x10a\x075Wa\x075a=\xF2V[` \x02\x01Q\x89Q`\x01` \x02\x01Q\x8A` \x01Q`\0`\x02\x81\x10a\x07ZWa\x07Za=\xF2V[` \x02\x01Q\x8B` \x01Q`\x01`\x02\x81\x10a\x07vWa\x07va=\xF2V[` \x90\x81\x02\x91\x90\x91\x01Q\x8CQ\x8D\x83\x01Q`@Qa\x07\xD3\x9A\x99\x98\x97\x96\x95\x94\x01\x98\x89R` \x89\x01\x97\x90\x97R`@\x88\x01\x95\x90\x95R``\x87\x01\x93\x90\x93R`\x80\x86\x01\x91\x90\x91R`\xA0\x85\x01R`\xC0\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01Ra\x01 \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\0\x1Ca\x07\xF6\x91\x90a>\x08V[\x90Pa\x08ia\x08\x0Fa\x08\x08\x88\x84a- V[\x86\x90a-\xB7V[a\x08\x17a.KV[a\x08_a\x08P\x85a\x08J`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x01\x82R`\x02\x90\x82\x01R\x90V[\x90a- V[a\x08Y\x8Ca/\x0BV[\x90a-\xB7V[\x88b\x01\xD4\xC0a/\x9BV[\x90\x98\x90\x97P\x95PPPPPPV[`\x9B\x80Ta\x08\x84\x90a>*V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08\xB0\x90a>*V[\x80\x15a\x08\xFDW\x80`\x1F\x10a\x08\xD2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08\xFDV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x08\xE0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\x97T`\x01`\x01`\xA0\x1B\x03\x163\x14a\t_W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FUpdater must be the caller\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05oV[a\to`\x80\x85\x01``\x86\x01a>sV[`\x9ATd\x01\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14a\t\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7Freference block hash mismatch\0\0\0`D\x82\x01R`d\x01a\x05oV[\x83`@Q` \x01a\t\xE2\x91\x90a?\x03V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x83` \x015\x14a\nJW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FreferenceTaskHash hash mismatch\0`D\x82\x01R`d\x01a\x05oV[\x80`@Q` \x01a\n[\x91\x90aDoV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x83`@\x015\x14a\n\xC3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FoperatorStateInfo hash mismatch\0`D\x82\x01R`d\x01a\x05oV[`\x9ATd\x01\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x15a\x0C\x83Wa\n\xE9``\x85\x01`@\x86\x01a>sV[`\x9ATc\xFF\xFF\xFF\xFF\x91\x82\x16\x91a\x0B\n\x91d\x01\0\0\0\0\x90\x04\x16a8@aE\xECV[c\xFF\xFF\xFF\xFF\x16\x11a\x0BMW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x077F\x16\xC6R\x077F\x17FR\x03`\x9C\x1B`D\x82\x01R`d\x01a\x05oV[B`\x98Tb\x03\xF4\x80a\x0B_\x91\x90aF\x14V[\x11a\x0B\x9CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rlstale state 1`\x98\x1B`D\x82\x01R`d\x01a\x05oV[`\0a\x0B\xD2\x84`@Q` \x01a\x0B\xB2\x91\x90aF,V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84a\x049\x90aFyV[`\x9CT\x90\x91Pc\xFF\xFF\xFF\xFF\x16`\0[`\x9B\x80Ta\x0B\xEE\x90a>*V[\x90P\x81\x10\x15a\x0C\x7FW\x81`\xFF\x16\x83` \x01Q\x82\x81Q\x81\x10a\x0C\x11Wa\x0C\x11a=\xF2V[` \x02` \x01\x01Qa\x0C#\x91\x90aF\x85V[`\x01`\x01``\x1B\x03\x16`d\x84`\0\x01Q\x83\x81Q\x81\x10a\x0CDWa\x0CDa=\xF2V[` \x02` \x01\x01Q`\x01`\x01``\x1B\x03\x16a\x0C_\x91\x90aF\xB4V[\x10\x15a\x0CmWPPPa\x17;V[\x80a\x0Cw\x81aF\xD3V[\x91PPa\x0B\xE1V[PPP[`\0[a\x0C\x93` \x83\x01\x83aF\xEEV[\x90P\x81\x10\x15a\rXW`\x9D`\0a\x0C\xAD` \x85\x01\x85aF\xEEV[\x84\x81\x81\x10a\x0C\xBDWa\x0C\xBDa=\xF2V[\x90P` \x02\x01` \x81\x01\x90a\x0C\xD2\x91\x90a7(V[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0\x90\x81 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U`\x9F\x91a\r\x05\x90\x85\x01\x85aF\xEEV[\x84\x81\x81\x10a\r\x15Wa\r\x15a=\xF2V[\x90P` \x02\x01` \x81\x01\x90a\r*\x91\x90a7(V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x81\x81U`\x01\x01U\x80a\rP\x81aF\xD3V[\x91PPa\x0C\x86V[P`\0[a\ri`@\x83\x01\x83aG7V[\x90P\x81\x10\x15a\x0E\xA4Wa\r\x7F`@\x83\x01\x83aG7V[\x82\x81\x81\x10a\r\x8FWa\r\x8Fa=\xF2V[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\r\xA7\x91\x90aG\x80V[`\x9D`\0a\r\xB8`@\x86\x01\x86aG7V[\x85\x81\x81\x10a\r\xC8Wa\r\xC8a=\xF2V[a\r\xDE\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa7(V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x90\x81\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x17\x90\x91Ua\x0E\x1E\x90\x83\x01\x83aG7V[\x82\x81\x81\x10a\x0E.Wa\x0E.a=\xF2V[\x90P`\x80\x02\x01`@\x01`\x9F`\0\x84\x80`@\x01\x90a\x0EK\x91\x90aG7V[\x85\x81\x81\x10a\x0E[Wa\x0E[a=\xF2V[a\x0Eq\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa7(V[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a\x0E\x9C\x81aF\xD3V[\x91PPa\r\\V[P`\0[a\x0E\xB5``\x83\x01\x83aG\x9BV[\x90P\x81\x10\x15a\x0FmWa\x0E\xCB``\x83\x01\x83aG\x9BV[\x82\x81\x81\x10a\x0E\xDBWa\x0E\xDBa=\xF2V[\x90P`@\x02\x01` \x01` \x81\x01\x90a\x0E\xF3\x91\x90aG\x80V[`\x9D`\0a\x0F\x04``\x86\x01\x86aG\x9BV[\x85\x81\x81\x10a\x0F\x14Wa\x0F\x14a=\xF2V[a\x0F*\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa7(V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\x0Fe\x81aF\xD3V[\x91PPa\x0E\xA8V[P`\0[a\x0F~`\x80\x83\x01\x83aG\xE4V[\x90P\x81\x10\x15a\x10\x1AWa\x0F\x94`\x80\x83\x01\x83aG\xE4V[\x82\x81\x81\x10a\x0F\xA4Wa\x0F\xA4a=\xF2V[\x90P``\x02\x01` \x01`\x9F`\0\x84\x80`\x80\x01\x90a\x0F\xC1\x91\x90aG\xE4V[\x85\x81\x81\x10a\x0F\xD1Wa\x0F\xD1a=\xF2V[a\x0F\xE7\x92` ``\x90\x92\x02\x01\x90\x81\x01\x91Pa7(V[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a\x10\x12\x81aF\xD3V[\x91PPa\x0FqV[P`\0[a\x10+`\xA0\x83\x01\x83aF\xEEV[\x90P\x81\x10\x15a\x11QW`\0[`\x9B\x80Ta\x10D\x90a>*V[\x90P\x81\x10\x15a\x10\xFDW`\x9E`\0a\x10^`\xA0\x86\x01\x86aF\xEEV[\x85\x81\x81\x10a\x10nWa\x10na=\xF2V[\x90P` \x02\x015\x81R` \x01\x90\x81R` \x01`\0 `\0`\x9B\x83\x81Ta\x10\x93\x90a>*V[\x81\x10a\x10\xA1Wa\x10\xA1a=\xF2V[\x81T`\x01\x16\x15a\x10\xC0W\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U\x81a\x10\xF5\x81aF\xD3V[\x92PPa\x107V[P`\xA0`\0a\x11\x0E\x84\x83\x01\x85aF\xEEV[\x84\x81\x81\x10a\x11\x1EWa\x11\x1Ea=\xF2V[` \x90\x81\x02\x92\x90\x92\x015\x83RP\x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\xFF\x19\x16\x90U\x80a\x11I\x81aF\xD3V[\x91PPa\x10\x1EV[P`\0[a\x11b`\xC0\x83\x01\x83aF\xEEV[\x90P\x81\x10\x15a\x13\xB2Wa\x11x`\xC0\x83\x01\x83aF\xEEV[\x82\x81\x81\x10a\x11\x88Wa\x11\x88a=\xF2V[\x90P` \x02\x81\x01\x90a\x11\x9A\x91\x90aH,V[a\x11\xAB\x90`\x80\x81\x01\x90``\x01a7(V[`\xA0`\0a\x11\xBC`\xC0\x86\x01\x86aF\xEEV[\x85\x81\x81\x10a\x11\xCCWa\x11\xCCa=\xF2V[\x90P` \x02\x81\x01\x90a\x11\xDE\x91\x90aH,V[`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP`\0[a\x12\x18`\xC0\x84\x01\x84aF\xEEV[\x83\x81\x81\x10a\x12(Wa\x12(a=\xF2V[\x90P` \x02\x81\x01\x90a\x12:\x91\x90aH,V[a\x12H\x90` \x81\x01\x90aF\xEEV[\x90P\x81\x10\x15a\x13\x9FWa\x12^`\xC0\x84\x01\x84aF\xEEV[\x83\x81\x81\x10a\x12nWa\x12na=\xF2V[\x90P` \x02\x81\x01\x90a\x12\x80\x91\x90aH,V[a\x12\x8E\x90`@\x81\x01\x90aF\xEEV[\x82\x81\x81\x10a\x12\x9EWa\x12\x9Ea=\xF2V[\x90P` \x02\x01` \x81\x01\x90a\x12\xB3\x91\x90aG\x80V[`\x9E`\0a\x12\xC4`\xC0\x87\x01\x87aF\xEEV[\x86\x81\x81\x10a\x12\xD4Wa\x12\xD4a=\xF2V[\x90P` \x02\x81\x01\x90a\x12\xE6\x91\x90aH,V[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a\x13\x07`\xC0\x87\x01\x87aF\xEEV[\x86\x81\x81\x10a\x13\x17Wa\x13\x17a=\xF2V[\x90P` \x02\x81\x01\x90a\x13)\x91\x90aH,V[a\x137\x90` \x81\x01\x90aF\xEEV[\x85\x81\x81\x10a\x13GWa\x13Ga=\xF2V[\x90P` \x02\x01` \x81\x01\x90a\x13\\\x91\x90a7(V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\x13\x97\x81aF\xD3V[\x91PPa\x12\x0BV[P\x80a\x13\xAA\x81aF\xD3V[\x91PPa\x11UV[P`\0[a\x13\xC3`\xE0\x83\x01\x83aF\xEEV[\x90P\x81\x10\x15a\x15vW`\0[a\x13\xDC`\xE0\x84\x01\x84aF\xEEV[\x83\x81\x81\x10a\x13\xECWa\x13\xECa=\xF2V[\x90P` \x02\x81\x01\x90a\x13\xFE\x91\x90aHLV[a\x14\x0C\x90` \x81\x01\x90aF\xEEV[\x90P\x81\x10\x15a\x15cWa\x14\"`\xE0\x84\x01\x84aF\xEEV[\x83\x81\x81\x10a\x142Wa\x142a=\xF2V[\x90P` \x02\x81\x01\x90a\x14D\x91\x90aHLV[a\x14R\x90`@\x81\x01\x90aF\xEEV[\x82\x81\x81\x10a\x14bWa\x14ba=\xF2V[\x90P` \x02\x01` \x81\x01\x90a\x14w\x91\x90aG\x80V[`\x9E`\0a\x14\x88`\xE0\x87\x01\x87aF\xEEV[\x86\x81\x81\x10a\x14\x98Wa\x14\x98a=\xF2V[\x90P` \x02\x81\x01\x90a\x14\xAA\x91\x90aHLV[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a\x14\xCB`\xE0\x87\x01\x87aF\xEEV[\x86\x81\x81\x10a\x14\xDBWa\x14\xDBa=\xF2V[\x90P` \x02\x81\x01\x90a\x14\xED\x91\x90aHLV[a\x14\xFB\x90` \x81\x01\x90aF\xEEV[\x85\x81\x81\x10a\x15\x0BWa\x15\x0Ba=\xF2V[\x90P` \x02\x01` \x81\x01\x90a\x15 \x91\x90a7(V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\x15[\x81aF\xD3V[\x91PPa\x13\xCFV[P\x80a\x15n\x81aF\xD3V[\x91PPa\x13\xB6V[P`\0[a\x15\x88a\x01\0\x83\x01\x83aG\x9BV[\x90P\x81\x10\x15a\x16,Wa\x15\x9Fa\x01\0\x83\x01\x83aG\x9BV[\x82\x81\x81\x10a\x15\xAFWa\x15\xAFa=\xF2V[\x90P`@\x02\x01` \x01` \x81\x01\x90a\x15\xC7\x91\x90a7(V[`\xA0`\0a\x15\xD9a\x01\0\x86\x01\x86aG\x9BV[\x85\x81\x81\x10a\x15\xE9Wa\x15\xE9a=\xF2V[\x90P`@\x02\x01`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP\x80\x80a\x16$\x90aF\xD3V[\x91PPa\x15zV[P`\xA0\x83\x015`\x99Ua\x16B` \x85\x01\x85a>sV[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\x16l``\x85\x01`@\x86\x01a>sV[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90UB`\x98Ua\x16\xA4`\x80\x85\x01\x85aHbV[a\x16\xB0\x91`\x9B\x91a5\xA4V[Pa\x16\xC1`\xC0\x85\x01`\xA0\x86\x01a>sV[`\x9C\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x7F\x9A\x12\x8F\xE74\x7F\x1E\x11\xCA\"\xAA\x9D\xEBc.\xC9\xAB\xB0\x96\x08\xC1:\x99L`\xF7zV/F\x01qa\x17\t` \x86\x01\x86a>sV[a\x17\x19``\x87\x01`@\x88\x01a>sV[`@\x80Qc\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01[`@Q\x80\x91\x03\x90\xA1[PPPPV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\x89W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xAD\x91\x90a=\x8DV[a\x17\xC9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05o\x90a=\xAAV[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[a\x18\x10a,\xC6V[a\x18\x1A`\0a1\xBFV[V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x18<WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x18VWP0;\x15\x80\x15a\x18VWP`\0T`\xFF\x16`\x01\x14[a\x18\xB9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x05oV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x18\xDCW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x18\xE7\x84`\0a2\x11V[a\x18\xF0\x83a1\xBFV[`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90U\x80\x15a\x17;W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01a\x172V[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01\x81\x90R`\x9B\x80Ta\x19\x84\x90a>*V[\x90P\x90Pa\x19\xA5`@Q\x80`@\x01`@R\x80``\x81R` \x01``\x81RP\x90V[\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19\xBDWa\x19\xBDa7\x95V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x19\xE6W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P` \x82\x01R\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1A\x04Wa\x1A\x04a7\x95V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1A-W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x81R\x84QQ`\0\x90`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1AMWa\x1AMa7\x95V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1AvW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80[\x87QQ\x81\x10\x15a\x1C1Wa\x1A\xBF\x88`\0\x01Q\x82\x81Q\x81\x10a\x1A\xA0Wa\x1A\xA0a=\xF2V[` \x02` \x01\x01Q\x80Q`\0\x90\x81R` \x91\x82\x01Q\x90\x91R`@\x90 \x90V[\x83\x82\x81Q\x81\x10a\x1A\xD1Wa\x1A\xD1a=\xF2V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80\x15a\x1B\xAFW\x82a\x1A\xEE`\x01\x83aH\xA8V[\x81Q\x81\x10a\x1A\xFEWa\x1A\xFEa=\xF2V[` \x02` \x01\x01Q`\0\x1C\x83\x82\x81Q\x81\x10a\x1B\x1BWa\x1B\x1Ba=\xF2V[` \x02` \x01\x01Q`\0\x1C\x11a\x1B\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`M`$\x82\x01R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: nonSignerG1PubkeysForOldSta`d\x82\x01Rl\x1D\x19H\x1B\x9B\xDD\x08\x1C\xDB\xDC\x9D\x19Y`\x9A\x1B`\x84\x82\x01R`\xA4\x01a\x05oV[a\x1C\x1Da\x1C\x16`\xA0`\0\x86\x85\x81Q\x81\x10a\x1B\xCBWa\x1B\xCBa=\xF2V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82R\x81\x01\x91\x90\x91R`@\x01`\0 T\x8AQ\x80Q`\xFF\x90\x92\x16\x91\x85\x90\x81\x10a\x1C\0Wa\x1C\0a=\xF2V[` \x02` \x01\x01Qa2\xFB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x87\x90a-\xB7V[\x95P\x80a\x1C)\x81aF\xD3V[\x91PPa\x1A}V[Pa\x1C;\x85a3\xDFV[\x94P`\0[\x84\x81\x10\x15a\x1E\x1CW`\x9B\x81\x81Ta\x1CV\x90a>*V[\x81\x10a\x1CdWa\x1Cda=\xF2V[\x81T`\x01\x16\x15a\x1C\x83W\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C`\0\x81\x81R`\x9F` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x01T\x90\x82\x01R\x90\x92Pa\x1C\xC4\x90\x87\x90a-\xB7V[`\xFF\x83\x16`\0\x90\x81R`\x9D` \x90\x81R`@\x90\x91 T\x90\x86\x01Q\x80Q\x92\x98P`\x01`\x01``\x1B\x03\x90\x91\x16\x91\x83\x90\x81\x10a\x1C\xFFWa\x1C\xFFa=\xF2V[`\x01`\x01``\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x82\x01R\x84\x01Q\x80Q\x82\x90\x81\x10a\x1D+Wa\x1D+a=\xF2V[` \x02` \x01\x01Q\x84`\0\x01Q\x82\x81Q\x81\x10a\x1DIWa\x1DIa=\xF2V[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPP`\0[\x88QQ\x81\x10\x15a\x1E\tW`\x9E`\0\x85\x83\x81Q\x81\x10a\x1D\x8CWa\x1D\x8Ca=\xF2V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82R\x81\x81\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 `\xFF\x87\x16\x82R\x90\x92R\x90 T\x85Q\x80Q`\x01`\x01``\x1B\x03\x90\x92\x16\x91\x84\x90\x81\x10a\x1D\xD7Wa\x1D\xD7a=\xF2V[` \x02` \x01\x01\x81\x81Qa\x1D\xEB\x91\x90aH\xBFV[`\x01`\x01``\x1B\x03\x16\x90RP\x80a\x1E\x01\x81aF\xD3V[\x91PPa\x1DlV[P\x80a\x1E\x14\x81aF\xD3V[\x91PPa\x1C@V[P`\0\x80a\x1E4\x8A\x88\x8B` \x01Q\x8C`@\x01Qa\x06\xEDV[\x91P\x91P\x81a\x1E\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: pairing precompile call fai`d\x82\x01Rb\x1B\x19Y`\xEA\x1B`\x84\x82\x01R`\xA4\x01a\x05oV[\x80a\x1F*W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: signature is invalid\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05oV[P\x92\x95PPPPPP[\x92\x91PPV[a\x1FBa,\xC6V[`\0[a\x1FR` \x83\x01\x83aF\xEEV[\x90P\x81\x10\x15a \x17W`\x9D`\0a\x1Fl` \x85\x01\x85aF\xEEV[\x84\x81\x81\x10a\x1F|Wa\x1F|a=\xF2V[\x90P` \x02\x01` \x81\x01\x90a\x1F\x91\x91\x90a7(V[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0\x90\x81 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U`\x9F\x91a\x1F\xC4\x90\x85\x01\x85aF\xEEV[\x84\x81\x81\x10a\x1F\xD4Wa\x1F\xD4a=\xF2V[\x90P` \x02\x01` \x81\x01\x90a\x1F\xE9\x91\x90a7(V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x81\x81U`\x01\x01U\x80a \x0F\x81aF\xD3V[\x91PPa\x1FEV[P`\0[a (`@\x83\x01\x83aG7V[\x90P\x81\x10\x15a!cWa >`@\x83\x01\x83aG7V[\x82\x81\x81\x10a NWa Na=\xF2V[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a f\x91\x90aG\x80V[`\x9D`\0a w`@\x86\x01\x86aG7V[\x85\x81\x81\x10a \x87Wa \x87a=\xF2V[a \x9D\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa7(V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x90\x81\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x17\x90\x91Ua \xDD\x90\x83\x01\x83aG7V[\x82\x81\x81\x10a \xEDWa \xEDa=\xF2V[\x90P`\x80\x02\x01`@\x01`\x9F`\0\x84\x80`@\x01\x90a!\n\x91\x90aG7V[\x85\x81\x81\x10a!\x1AWa!\x1Aa=\xF2V[a!0\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa7(V[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a![\x81aF\xD3V[\x91PPa \x1BV[P`\0[a!t``\x83\x01\x83aG\x9BV[\x90P\x81\x10\x15a\",Wa!\x8A``\x83\x01\x83aG\x9BV[\x82\x81\x81\x10a!\x9AWa!\x9Aa=\xF2V[\x90P`@\x02\x01` \x01` \x81\x01\x90a!\xB2\x91\x90aG\x80V[`\x9D`\0a!\xC3``\x86\x01\x86aG\x9BV[\x85\x81\x81\x10a!\xD3Wa!\xD3a=\xF2V[a!\xE9\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa7(V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\"$\x81aF\xD3V[\x91PPa!gV[P`\0[a\"=`\x80\x83\x01\x83aG\xE4V[\x90P\x81\x10\x15a\"\xD9Wa\"S`\x80\x83\x01\x83aG\xE4V[\x82\x81\x81\x10a\"cWa\"ca=\xF2V[\x90P``\x02\x01` \x01`\x9F`\0\x84\x80`\x80\x01\x90a\"\x80\x91\x90aG\xE4V[\x85\x81\x81\x10a\"\x90Wa\"\x90a=\xF2V[a\"\xA6\x92` ``\x90\x92\x02\x01\x90\x81\x01\x91Pa7(V[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a\"\xD1\x81aF\xD3V[\x91PPa\"0V[P`\0[a\"\xEA`\xA0\x83\x01\x83aF\xEEV[\x90P\x81\x10\x15a$\x10W`\0[`\x9B\x80Ta#\x03\x90a>*V[\x90P\x81\x10\x15a#\xBCW`\x9E`\0a#\x1D`\xA0\x86\x01\x86aF\xEEV[\x85\x81\x81\x10a#-Wa#-a=\xF2V[\x90P` \x02\x015\x81R` \x01\x90\x81R` \x01`\0 `\0`\x9B\x83\x81Ta#R\x90a>*V[\x81\x10a#`Wa#`a=\xF2V[\x81T`\x01\x16\x15a#\x7FW\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U\x81a#\xB4\x81aF\xD3V[\x92PPa\"\xF6V[P`\xA0`\0a#\xCD\x84\x83\x01\x85aF\xEEV[\x84\x81\x81\x10a#\xDDWa#\xDDa=\xF2V[` \x90\x81\x02\x92\x90\x92\x015\x83RP\x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\xFF\x19\x16\x90U\x80a$\x08\x81aF\xD3V[\x91PPa\"\xDDV[P`\0[a$!`\xC0\x83\x01\x83aF\xEEV[\x90P\x81\x10\x15a&qWa$7`\xC0\x83\x01\x83aF\xEEV[\x82\x81\x81\x10a$GWa$Ga=\xF2V[\x90P` \x02\x81\x01\x90a$Y\x91\x90aH,V[a$j\x90`\x80\x81\x01\x90``\x01a7(V[`\xA0`\0a${`\xC0\x86\x01\x86aF\xEEV[\x85\x81\x81\x10a$\x8BWa$\x8Ba=\xF2V[\x90P` \x02\x81\x01\x90a$\x9D\x91\x90aH,V[`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP`\0[a$\xD7`\xC0\x84\x01\x84aF\xEEV[\x83\x81\x81\x10a$\xE7Wa$\xE7a=\xF2V[\x90P` \x02\x81\x01\x90a$\xF9\x91\x90aH,V[a%\x07\x90` \x81\x01\x90aF\xEEV[\x90P\x81\x10\x15a&^Wa%\x1D`\xC0\x84\x01\x84aF\xEEV[\x83\x81\x81\x10a%-Wa%-a=\xF2V[\x90P` \x02\x81\x01\x90a%?\x91\x90aH,V[a%M\x90`@\x81\x01\x90aF\xEEV[\x82\x81\x81\x10a%]Wa%]a=\xF2V[\x90P` \x02\x01` \x81\x01\x90a%r\x91\x90aG\x80V[`\x9E`\0a%\x83`\xC0\x87\x01\x87aF\xEEV[\x86\x81\x81\x10a%\x93Wa%\x93a=\xF2V[\x90P` \x02\x81\x01\x90a%\xA5\x91\x90aH,V[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a%\xC6`\xC0\x87\x01\x87aF\xEEV[\x86\x81\x81\x10a%\xD6Wa%\xD6a=\xF2V[\x90P` \x02\x81\x01\x90a%\xE8\x91\x90aH,V[a%\xF6\x90` \x81\x01\x90aF\xEEV[\x85\x81\x81\x10a&\x06Wa&\x06a=\xF2V[\x90P` \x02\x01` \x81\x01\x90a&\x1B\x91\x90a7(V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a&V\x81aF\xD3V[\x91PPa$\xCAV[P\x80a&i\x81aF\xD3V[\x91PPa$\x14V[P`\0[a&\x82`\xE0\x83\x01\x83aF\xEEV[\x90P\x81\x10\x15a(5W`\0[a&\x9B`\xE0\x84\x01\x84aF\xEEV[\x83\x81\x81\x10a&\xABWa&\xABa=\xF2V[\x90P` \x02\x81\x01\x90a&\xBD\x91\x90aHLV[a&\xCB\x90` \x81\x01\x90aF\xEEV[\x90P\x81\x10\x15a(\"Wa&\xE1`\xE0\x84\x01\x84aF\xEEV[\x83\x81\x81\x10a&\xF1Wa&\xF1a=\xF2V[\x90P` \x02\x81\x01\x90a'\x03\x91\x90aHLV[a'\x11\x90`@\x81\x01\x90aF\xEEV[\x82\x81\x81\x10a'!Wa'!a=\xF2V[\x90P` \x02\x01` \x81\x01\x90a'6\x91\x90aG\x80V[`\x9E`\0a'G`\xE0\x87\x01\x87aF\xEEV[\x86\x81\x81\x10a'WWa'Wa=\xF2V[\x90P` \x02\x81\x01\x90a'i\x91\x90aHLV[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a'\x8A`\xE0\x87\x01\x87aF\xEEV[\x86\x81\x81\x10a'\x9AWa'\x9Aa=\xF2V[\x90P` \x02\x81\x01\x90a'\xAC\x91\x90aHLV[a'\xBA\x90` \x81\x01\x90aF\xEEV[\x85\x81\x81\x10a'\xCAWa'\xCAa=\xF2V[\x90P` \x02\x01` \x81\x01\x90a'\xDF\x91\x90a7(V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a(\x1A\x81aF\xD3V[\x91PPa&\x8EV[P\x80a(-\x81aF\xD3V[\x91PPa&uV[P`\0[a(Ga\x01\0\x83\x01\x83aG\x9BV[\x90P\x81\x10\x15a(\xEBWa(^a\x01\0\x83\x01\x83aG\x9BV[\x82\x81\x81\x10a(nWa(na=\xF2V[\x90P`@\x02\x01` \x01` \x81\x01\x90a(\x86\x91\x90a7(V[`\xA0`\0a(\x98a\x01\0\x86\x01\x86aG\x9BV[\x85\x81\x81\x10a(\xA8Wa(\xA8a=\xF2V[\x90P`@\x02\x01`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP\x80\x80a(\xE3\x90aF\xD3V[\x91PPa(9V[P`\xA0\x82\x015`\x99Ua)\x01` \x84\x01\x84a>sV[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua)+``\x84\x01`@\x85\x01a>sV[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90UB`\x98Ua)c`\x80\x84\x01\x84aHbV[a)o\x91`\x9B\x91a5\xA4V[Pa)\x80`\xC0\x84\x01`\xA0\x85\x01a>sV[`\x9C\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x7F&Ie\xEBk\xC46\xC6\xC4sC\x1D4\xAFV\xE82\xEC4O\xDF\xD4>\xE6\xAFo\xCEm ^\x84\xAFa)\xC8` \x85\x01\x85a>sV[a)\xD8``\x86\x01`@\x87\x01a>sV[`@\x80Qc\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1PPPV[a*\x05a,\xC6V[`\x01`\x01`\xA0\x1B\x03\x81\x16a*jW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x05oV[a\x05\x81\x81a1\xBFV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a*\xC6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*\xEA\x91\x90a=\x18V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a+\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05o\x90a=5V[`fT\x19\x81\x19`fT\x19\x16\x14a+\x98W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05oV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x06\xE2V[`\x01`\x01`\xA0\x1B\x03\x81\x16a,]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x05oV[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x18\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x05oV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra-<a6(V[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x80\x82\x01\x84\x90R`\0\x90\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a-oWa-qV[\xFE[P\x80a-\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCB[][\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x05oV[PP\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra-\xD3a6FV[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R\x83Q`@\x80\x84\x01\x91\x90\x91R\x90\x84\x01Q``\x83\x01R`\0\x90\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a-oWP\x80a-\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCBXY\x19\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x05oV[a.Sa6dV[P`@\x80Q`\x80\x81\x01\x82R\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81\x83\x01\x90\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED``\x83\x01R\x81R\x81Q\x80\x83\x01\x90\x92R\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x81\x01\x91\x90\x91R\x81\x01\x91\x90\x91R\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0\x80\x80a/;`\0\x80Q` aH\xE8\x839\x81Q\x91R\x86a>\x08V[\x90P[a/G\x81a4zV[\x90\x93P\x91P`\0\x80Q` aH\xE8\x839\x81Q\x91R\x82\x83\t\x83\x14\x15a/\x81W`@\x80Q\x80\x82\x01\x90\x91R\x90\x81R` \x81\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80Q` aH\xE8\x839\x81Q\x91R`\x01\x82\x08\x90Pa/>V[`@\x80Q\x80\x82\x01\x82R\x86\x81R` \x80\x82\x01\x86\x90R\x82Q\x80\x84\x01\x90\x93R\x86\x83R\x82\x01\x84\x90R`\0\x91\x82\x91\x90a/\xCDa6\x89V[`\0[`\x02\x81\x10\x15a1\x92W`\0a/\xE6\x82`\x06aF\xB4V[\x90P\x84\x82`\x02\x81\x10a/\xFAWa/\xFAa=\xF2V[` \x02\x01QQ\x83a0\x0C\x83`\0aF\x14V[`\x0C\x81\x10a0\x1CWa0\x1Ca=\xF2V[` \x02\x01R\x84\x82`\x02\x81\x10a03Wa03a=\xF2V[` \x02\x01Q` \x01Q\x83\x82`\x01a0J\x91\x90aF\x14V[`\x0C\x81\x10a0ZWa0Za=\xF2V[` \x02\x01R\x83\x82`\x02\x81\x10a0qWa0qa=\xF2V[` \x02\x01QQQ\x83a0\x84\x83`\x02aF\x14V[`\x0C\x81\x10a0\x94Wa0\x94a=\xF2V[` \x02\x01R\x83\x82`\x02\x81\x10a0\xABWa0\xABa=\xF2V[` \x02\x01QQ`\x01` \x02\x01Q\x83a0\xC4\x83`\x03aF\x14V[`\x0C\x81\x10a0\xD4Wa0\xD4a=\xF2V[` \x02\x01R\x83\x82`\x02\x81\x10a0\xEBWa0\xEBa=\xF2V[` \x02\x01Q` \x01Q`\0`\x02\x81\x10a1\x06Wa1\x06a=\xF2V[` \x02\x01Q\x83a1\x17\x83`\x04aF\x14V[`\x0C\x81\x10a1'Wa1'a=\xF2V[` \x02\x01R\x83\x82`\x02\x81\x10a1>Wa1>a=\xF2V[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a1YWa1Ya=\xF2V[` \x02\x01Q\x83a1j\x83`\x05aF\x14V[`\x0C\x81\x10a1zWa1za=\xF2V[` \x02\x01RP\x80a1\x8A\x81aF\xD3V[\x91PPa/\xD0V[Pa1\x9Ba6\xA8V[`\0` \x82a\x01\x80\x85`\x08\x8C\xFA\x91Q\x91\x9C\x91\x15\x15\x9BP\x90\x99PPPPPPPPPPV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a22WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a2\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x05oV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a2\xF7\x82a+\xCFV[PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x02\0\x82a\xFF\xFF\x16\x10a3WW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Roscalar-too-large`\x80\x1B`D\x82\x01R`d\x01a\x05oV[\x81a\xFF\xFF\x16`\x01\x14\x15a3kWP\x81a\x1F4V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01\x81\x90R\x84\x90`\x01\x90[\x81a\xFF\xFF\x16\x86a\xFF\xFF\x16\x10a3\xD4W`\x01a\xFF\xFF\x87\x16`\xFF\x83\x16\x1C\x81\x16\x14\x15a3\xB7Wa3\xB4\x84\x84a-\xB7V[\x93P[a3\xC1\x83\x84a-\xB7V[\x92Pb\x01\xFF\xFE`\x01\x92\x83\x1B\x16\x91\x01a3\x87V[P\x91\x95\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q\x15\x80\x15a4\x04WP` \x82\x01Q\x15[\x15a4\"WPP`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01`\0\x80Q` aH\xE8\x839\x81Q\x91R\x84` \x01Qa4U\x91\x90a>\x08V[a4m\x90`\0\x80Q` aH\xE8\x839\x81Q\x91RaH\xA8V[\x90R\x92\x91PPV[\x91\x90PV[`\0\x80\x80`\0\x80Q` aH\xE8\x839\x81Q\x91R`\x03`\0\x80Q` aH\xE8\x839\x81Q\x91R\x86`\0\x80Q` aH\xE8\x839\x81Q\x91R\x88\x89\t\t\x08\x90P`\0a4\xF0\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\0\x80Q` aH\xE8\x839\x81Q\x91Ra4\xFCV[\x91\x95\x91\x94P\x90\x92PPPV[`\0\x80a5\x07a6\xA8V[a5\x0Fa6\xC6V[` \x80\x82R\x81\x81\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x88\x90R`\x80\x82\x01\x87\x90R`\xA0\x82\x01\x86\x90R\x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82\x80\x15a-oWP\x82a5\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FBN254.expMod: call failure\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05oV[PQ\x95\x94PPPPPV[\x82\x80Ta5\xB0\x90a>*V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82a5\xD2W`\0\x85Ua6\x18V[\x82`\x1F\x10a5\xEBW\x82\x80\x01`\xFF\x19\x825\x16\x17\x85Ua6\x18V[\x82\x80\x01`\x01\x01\x85U\x82\x15a6\x18W\x91\x82\x01[\x82\x81\x11\x15a6\x18W\x825\x82U\x91` \x01\x91\x90`\x01\x01\x90a5\xFDV[Pa6$\x92\x91Pa6\xE4V[P\x90V[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80a6wa6\xF9V[\x81R` \x01a6\x84a6\xF9V[\x90R\x90V[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[[\x80\x82\x11\x15a6$W`\0\x81U`\x01\x01a6\xE5V[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[\x805`\xFF\x81\x16\x81\x14a4uW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a7:W`\0\x80\xFD[a7C\x82a7\x17V[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05\x81W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a7qW`\0\x80\xFD[\x815a7C\x81a7JV[`\0` \x82\x84\x03\x12\x15a7\x8EW`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a7\xCDWa7\xCDa7\x95V[`@R\x90V[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a7\xCDWa7\xCDa7\x95V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a8\x1DWa8\x1Da7\x95V[`@R\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a87W`\0\x80\xFD[a8?a7\xABV[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a8fW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a8\x88Wa8\x88a7\x95V[\x80`@RP\x80`@\x84\x01\x85\x81\x11\x15a8\x9FW`\0\x80\xFD[\x84[\x81\x81\x10\x15a3\xD4W\x805\x83R` \x92\x83\x01\x92\x01a8\xA1V[`\0`\x80\x82\x84\x03\x12\x15a8\xCBW`\0\x80\xFD[a8\xD3a7\xABV[\x90Pa8\xDF\x83\x83a8UV[\x81Ra8\xEE\x83`@\x84\x01a8UV[` \x82\x01R\x92\x91PPV[`\0\x80`\0\x80a\x01 \x85\x87\x03\x12\x15a9\x10W`\0\x80\xFD[\x845\x93Pa9!\x86` \x87\x01a8%V[\x92Pa90\x86``\x87\x01a8\xB9V[\x91Pa9?\x86`\xE0\x87\x01a8%V[\x90P\x92\x95\x91\x94P\x92PV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a9wW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a9[V[\x81\x81\x11\x15a9\x89W`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[`\0a\x01\0\x82\x84\x03\x12\x15a9\xB2W`\0\x80\xFD[P\x91\x90PV[`\0`\xC0\x82\x84\x03\x12\x15a9\xB2W`\0\x80\xFD[`\0a\x01 \x82\x84\x03\x12\x15a9\xB2W`\0\x80\xFD[`\0\x80`\0\x80a\x01 \x85\x87\x03\x12\x15a9\xF4W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a:\x0BW`\0\x80\xFD[a:\x17\x88\x83\x89\x01a9\x9FV[\x95Pa:&\x88` \x89\x01a9\xB8V[\x94P`\xE0\x87\x015\x91P\x80\x82\x11\x15a:<W`\0\x80\xFD[\x90\x86\x01\x90`\xE0\x82\x89\x03\x12\x15a:PW`\0\x80\xFD[\x90\x92Pa\x01\0\x86\x015\x90\x80\x82\x11\x15a:gW`\0\x80\xFD[Pa:t\x87\x82\x88\x01a9\xCAV[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a:\x93W`\0\x80\xFD[\x825\x91Pa:\xA3` \x84\x01a7\x17V[\x90P\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a:\xC1W`\0\x80\xFD[\x835a:\xCC\x81a7JV[\x92P` \x84\x015a:\xDC\x81a7JV[\x91P`@\x84\x015a:\xEC\x81a7JV[\x80\x91PP\x92P\x92P\x92V[`\0`\xE0\x82\x84\x03\x12\x15a;\tW`\0\x80\xFD[a;\x11a7\xD3V[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a;*W`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a;>W`\0\x80\xFD[\x815` \x82\x82\x11\x15a;RWa;Ra7\x95V[a;`\x81\x83`\x05\x1B\x01a7\xF5V[\x82\x81R\x81\x81\x01\x93P`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x87\x83\x11\x15a;\x80W`\0\x80\xFD[\x93\x81\x01\x93[\x82\x85\x10\x15a;\xA9Wa;\x97\x88\x86a8%V[\x84R\x81\x84\x01\x93P`@\x85\x01\x94Pa;\x85V[\x85Ra;\xB7\x87\x87\x83\x01a8\xB9V[\x81\x86\x01RPPPPa;\xCC\x83`\xA0\x84\x01a8%V[`@\x82\x01R\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a;\xEAW`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a<\x07W`\0\x80\xFD[a<\x13\x85\x82\x86\x01a:\xF7V[\x91PP\x92P\x92\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a<VW\x81Q`\x01`\x01``\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a<1V[P\x94\x95\x94PPPPPV[` \x81R`\0\x82Q`@` \x84\x01Ra<}``\x84\x01\x82a<\x1DV[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01Ra<\x9A\x82\x82a<\x1DV[\x95\x94PPPPPV[`\0\x80`\0a\x01\0\x84\x86\x03\x12\x15a<\xB9W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a<\xD0W`\0\x80\xFD[a<\xDC\x87\x83\x88\x01a9\x9FV[\x94Pa<\xEB\x87` \x88\x01a9\xB8V[\x93P`\xE0\x86\x015\x91P\x80\x82\x11\x15a=\x01W`\0\x80\xFD[Pa=\x0E\x86\x82\x87\x01a9\xCAV[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a=*W`\0\x80\xFD[\x81Qa7C\x81a7JV[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[\x80\x15\x15\x81\x14a\x05\x81W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a=\x9FW`\0\x80\xFD[\x81Qa7C\x81a=\x7FV[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x82a>%WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80a>>W`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15a9\xB2WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a4uW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a>\x85W`\0\x80\xFD[a7C\x82a>_V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a>\xA5W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a>\xC4W`\0\x80\xFD[\x806\x03\x83\x13\x15a>\xD3W`\0\x80\xFD[\x92P\x92\x90PV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x81Rc\xFF\xFF\xFF\xFFa?\x15\x83a>_V[\x16` \x82\x01R` \x82\x015`@\x82\x01R`\0a?3`@\x84\x01a>_V[c\xFF\xFF\xFF\xFF\x81\x16``\x84\x01RPa?L``\x84\x01a>_V[c\xFF\xFF\xFF\xFF\x81\x16`\x80\x84\x01RPa?f`\x80\x84\x01\x84a>\x8EV[a\x01\0\x80`\xA0\x86\x01Ra?~a\x01 \x86\x01\x83\x85a>\xDAV[\x92Pa?\x8C`\xA0\x87\x01a>_V[c\xFF\xFF\xFF\xFF\x81\x16`\xC0\x87\x01R\x91Pa?\xA7`\xC0\x87\x01\x87a>\x8EV[\x86\x85\x03`\x1F\x19\x01`\xE0\x88\x01R\x92Pa?\xC0\x84\x84\x83a>\xDAV[\x93PPa?\xCF`\xE0\x87\x01a>_V[c\xFF\xFF\xFF\xFF\x16\x94\x01\x93\x90\x93R\x93\x92PPPV[\x805a4u\x81a=\x7FV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a@\x04W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a@#W`\0\x80\xFD[\x80`\x05\x1B6\x03\x83\x13\x15a>\xD3W`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a<VW`\xFFa@X\x83a7\x17V[\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a@EV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a@\x82W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a@\xA1W`\0\x80\xFD[\x80`\x07\x1B6\x03\x83\x13\x15a>\xD3W`\0\x80\xFD[\x805`\x01`\x01``\x1B\x03\x81\x16\x81\x14a4uW`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a<VW`\xFFa@\xED\x83a7\x17V[\x16\x87R`\x01`\x01``\x1B\x03aA\x03\x84\x84\x01a@\xB3V[\x16\x83\x88\x01R`@\x82\x81\x015\x90\x88\x01R``\x80\x83\x015\x90\x88\x01R`\x80\x96\x87\x01\x96\x90\x91\x01\x90`\x01\x01a@\xDAV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aAEW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aAdW`\0\x80\xFD[\x80`\x06\x1B6\x03\x83\x13\x15a>\xD3W`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a<VW`\xFFaA\x99\x83a7\x17V[\x16\x87R`\x01`\x01``\x1B\x03aA\xAF\x84\x84\x01a@\xB3V[\x16\x87\x84\x01R`@\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aA\x86V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aA\xDEW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aA\xFDW`\0\x80\xFD[``\x81\x026\x03\x83\x13\x15a>\xD3W`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a<VW`\xFFaB2\x83a7\x17V[\x16\x87RaBM\x83\x88\x01\x84\x84\x01\x805\x82R` \x90\x81\x015\x91\x01RV[``\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aB\x1FV[\x81\x83R`\0`\x01`\x01`\xFB\x1B\x03\x83\x11\x15aByW`\0\x80\xFD[\x82`\x05\x1B\x80\x83` \x87\x017`\0\x94\x01` \x01\x93\x84RP\x91\x92\x91PPV[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a<VW`\x01`\x01``\x1B\x03aB\xBF\x83a@\xB3V[\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aB\xA6V[\x81\x83R`\0` \x80\x85\x01\x80\x81\x96P\x85`\x05\x1B\x81\x01\x91P\x84`\0\x80[\x88\x81\x10\x15aC\x83W\x83\x85\x03\x8AR\x825`~\x19\x896\x03\x01\x81\x12aC\rW\x82\x83\xFD[\x88\x01\x805\x86R`\x80aC!\x88\x83\x01\x83a?\xEDV[\x82\x8A\x8A\x01RaC3\x83\x8A\x01\x82\x84a@5V[\x92PPP`@aCE\x81\x84\x01\x84a?\xEDV[\x89\x84\x03\x83\x8B\x01RaCW\x84\x82\x84aB\x96V[\x93PPPP```\xFFaCk\x82\x85\x01a7\x17V[\x16\x97\x01\x96\x90\x96RP\x98\x85\x01\x98\x91\x85\x01\x91`\x01\x01aB\xEDV[P\x92\x98\x97PPPPPPPPV[\x81\x83R`\0` \x80\x85\x01\x80\x81\x96P\x85`\x05\x1B\x81\x01\x91P\x84`\0\x80[\x88\x81\x10\x15aC\x83W\x83\x85\x03\x8AR\x825`^\x19\x896\x03\x01\x81\x12aC\xCCW\x82\x83\xFD[\x88\x01\x805\x86R``aC\xE0\x88\x83\x01\x83a?\xEDV[\x82\x8A\x8A\x01RaC\xF2\x83\x8A\x01\x82\x84a@5V[\x92PPP`@aD\x04\x81\x84\x01\x84a?\xEDV[\x93P\x88\x83\x03\x82\x8A\x01RaD\x18\x83\x85\x83aB\x96V[\x9D\x8A\x01\x9D\x98PPP\x93\x87\x01\x93PP`\x01\x01aC\xACV[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a<VW\x815\x87R`\xFFaDW\x84\x84\x01a7\x17V[\x16\x87\x84\x01R`@\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aD>V[` \x81RaD\x89` \x82\x01aD\x83\x84a?\xE2V[\x15\x15\x90RV[`\0aD\x98` \x84\x01\x84a?\xEDV[a\x01 \x80`@\x86\x01RaD\xB0a\x01@\x86\x01\x83\x85a@5V[\x92PaD\xBF`@\x87\x01\x87a@kV[\x92P`\x1F\x19\x80\x87\x86\x03\x01``\x88\x01RaD\xD9\x85\x85\x84a@\xCAV[\x94PaD\xE8``\x89\x01\x89aA.V[\x94P\x91P\x80\x87\x86\x03\x01`\x80\x88\x01RaE\x01\x85\x85\x84aAvV[\x94PaE\x10`\x80\x89\x01\x89aA\xC7V[\x94P\x91P\x80\x87\x86\x03\x01`\xA0\x88\x01RaE)\x85\x85\x84aB\x0FV[\x94PaE8`\xA0\x89\x01\x89a?\xEDV[\x94P\x91P\x80\x87\x86\x03\x01`\xC0\x88\x01RaEQ\x85\x85\x84aB`V[\x94PaE``\xC0\x89\x01\x89a?\xEDV[\x94P\x91P\x80\x87\x86\x03\x01`\xE0\x88\x01RaEy\x85\x85\x84aB\xD2V[\x94PaE\x88`\xE0\x89\x01\x89a?\xEDV[\x94P\x91Pa\x01\0\x81\x88\x87\x03\x01\x81\x89\x01RaE\xA3\x86\x86\x85aC\x91V[\x95PaE\xB1\x81\x8A\x01\x8AaA.V[\x95P\x92PP\x80\x87\x86\x03\x01\x83\x88\x01RPaE\xCB\x84\x84\x83aD.V[\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aF\x0BWaF\x0BaE\xD6V[\x01\x94\x93PPPPV[`\0\x82\x19\x82\x11\x15aF'WaF'aE\xD6V[P\x01\x90V[`\xC0\x81\x01c\xFF\xFF\xFF\xFFaF>\x84a>_V[\x16\x82R` \x83\x015` \x83\x01R`@\x83\x015`@\x83\x01R``\x83\x015``\x83\x01R`\x80\x83\x015`\x80\x83\x01R`\xA0\x83\x015`\xA0\x83\x01R\x92\x91PPV[`\0a\x1F46\x83a:\xF7V[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aF\xABWaF\xABaE\xD6V[\x02\x94\x93PPPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15aF\xCEWaF\xCEaE\xD6V[P\x02\x90V[`\0`\0\x19\x82\x14\x15aF\xE7WaF\xE7aE\xD6V[P`\x01\x01\x90V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aG\x05W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aG\x1FW`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a>\xD3W`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aGNW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aGhW`\0\x80\xFD[` \x01\x91P`\x07\x81\x90\x1B6\x03\x82\x13\x15a>\xD3W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aG\x92W`\0\x80\xFD[a7C\x82a@\xB3V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aG\xB2W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aG\xCCW`\0\x80\xFD[` \x01\x91P`\x06\x81\x90\x1B6\x03\x82\x13\x15a>\xD3W`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aG\xFBW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aH\x15W`\0\x80\xFD[` \x01\x91P``\x81\x026\x03\x82\x13\x15a>\xD3W`\0\x80\xFD[`\0\x825`~\x19\x836\x03\x01\x81\x12aHBW`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x825`^\x19\x836\x03\x01\x81\x12aHBW`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aHyW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aH\x93W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a>\xD3W`\0\x80\xFD[`\0\x82\x82\x10\x15aH\xBAWaH\xBAaE\xD6V[P\x03\x90V[`\0`\x01`\x01``\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15aH\xDFWaH\xDFaE\xD6V[\x03\x93\x92PPPV\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 ;@b\x14\xB2\x92\xF3xve\xF4W\xC4 \x87x$\xB4|\x9F\xD5[\xF1J\xF9\xF9\x89\xE1\xC7\xF5\xE4=dsolcC\0\x08\x0C\x003";
    /// The deployed bytecode of the contract.
    pub static GASPMULTIROLLUPSERVICE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct GaspMultiRollupService<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for GaspMultiRollupService<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for GaspMultiRollupService<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for GaspMultiRollupService<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for GaspMultiRollupService<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(GaspMultiRollupService))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> GaspMultiRollupService<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                GASPMULTIROLLUPSERVICE_ABI.clone(),
                client,
            ))
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
                GASPMULTIROLLUPSERVICE_ABI.clone(),
                GASPMULTIROLLUPSERVICE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `checkSignatures` (0xc4e1914c) function
        pub fn check_signatures(
            &self,
            msg_hash: [u8; 32],
            params: NonSignerStakesAndSignatureForOldState,
        ) -> ::ethers::contract::builders::ContractCall<M, QuorumStakeTotals> {
            self.0
                .method_hash([196, 225, 145, 76], (msg_hash, params))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0xc0c53b8b) function
        pub fn initialize(
            &self,
            pauser_registry: ::ethers::core::types::Address,
            initial_owner: ::ethers::core::types::Address,
            updater: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [192, 197, 59, 139],
                    (pauser_registry, initial_owner, updater),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastUpdateBlockTimestamp` (0xe61db175) function
        pub fn last_update_block_timestamp(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([230, 29, 177, 117], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `latestCompletedTaskCreatedBlock` (0xed5a04fe) function
        pub fn latest_completed_task_created_block(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([237, 90, 4, 254], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `latestCompletedTaskNumber` (0xfc765dd5) function
        pub fn latest_completed_task_number(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([252, 118, 93, 213], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `latestPendingStateHash` (0x4ae6b203) function
        pub fn latest_pending_state_hash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([74, 230, 178, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operatorAndQuorumToStakes` (0x499d6fb6) function
        pub fn operator_and_quorum_to_stakes(
            &self,
            p0: [u8; 32],
            p1: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([73, 157, 111, 182], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operatorIdQuorumCount` (0x430d3b39) function
        pub fn operator_id_quorum_count(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([67, 13, 59, 57], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pause` (0x136439dd) function
        pub fn pause(
            &self,
            new_paused_status: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([19, 100, 57, 221], new_paused_status)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pauseAll` (0x595c6a67) function
        pub fn pause_all(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([89, 92, 106, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `paused` (0x5ac86ab7) function
        pub fn paused_with_index(
            &self,
            index: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([90, 200, 106, 183], index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `paused` (0x5c975abb) function
        pub fn paused(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([92, 151, 90, 187], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pauserRegistry` (0x886f1195) function
        pub fn pauser_registry(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([136, 111, 17, 149], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `process_eigen_reinit` (0xc927feef) function
        pub fn process_eigen_reinit(
            &self,
            task: Task,
            task_response: TaskResponse,
            operator_state_info: OperatorStateInfo,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [201, 39, 254, 239],
                    (task, task_response, operator_state_info),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `process_eigen_update` (0x3e404926) function
        pub fn process_eigen_update(
            &self,
            task: Task,
            task_response: TaskResponse,
            non_signer_stakes_and_signature_for_old_state: NonSignerStakesAndSignatureForOldState,
            operator_state_info: OperatorStateInfo,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [62, 64, 73, 38],
                    (
                        task,
                        task_response,
                        non_signer_stakes_and_signature_for_old_state,
                        operator_state_info,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `qourumApk` (0x03d097d2) function
        pub fn qourum_apk(
            &self,
            p0: u8,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([3, 208, 151, 210], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quorumNumbers` (0x2a8414fd) function
        pub fn quorum_numbers(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([42, 132, 20, 253], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quorumThresholdPercentage` (0x4deabc21) function
        pub fn quorum_threshold_percentage(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([77, 234, 188, 33], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quorumToStakes` (0x7ad75561) function
        pub fn quorum_to_stakes(
            &self,
            p0: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([122, 215, 85, 97], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPauserRegistry` (0x10d67a2f) function
        pub fn set_pauser_registry(
            &self,
            new_pauser_registry: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([16, 214, 122, 47], new_pauser_registry)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `set_updater` (0x124648c9) function
        pub fn set_updater(
            &self,
            updater: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([18, 70, 72, 201], updater)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stalled` (0x526e3e64) function
        pub fn stalled(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([82, 110, 62, 100], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `trySignatureAndApkVerification` (0x171f1d5b) function
        pub fn try_signature_and_apk_verification(
            &self,
            msg_hash: [u8; 32],
            apk: G1Point,
            apk_g2: G2Point,
            sigma: G1Point,
        ) -> ::ethers::contract::builders::ContractCall<M, (bool, bool)> {
            self.0
                .method_hash([23, 31, 29, 91], (msg_hash, apk, apk_g2, sigma))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unpause` (0xfabc1cbc) function
        pub fn unpause(
            &self,
            new_paused_status: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 188, 28, 188], new_paused_status)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updater` (0xdf034cd0) function
        pub fn updater(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([223, 3, 76, 208], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `EigenReinitProcessed` event
        pub fn eigen_reinit_processed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, EigenReinitProcessedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `EigenUpdateProcessed` event
        pub fn eigen_update_processed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, EigenUpdateProcessedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Initialized` event
        pub fn initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, InitializedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OwnershipTransferredFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Paused` event
        pub fn paused_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PausedFilter> {
            self.0.event()
        }
        ///Gets the contract's `PauserRegistrySet` event
        pub fn pauser_registry_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PauserRegistrySetFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Unpaused` event
        pub fn unpaused_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, UnpausedFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, GaspMultiRollupServiceEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for GaspMultiRollupService<M>
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
        name = "EigenReinitProcessed",
        abi = "EigenReinitProcessed(uint32,uint32)"
    )]
    pub struct EigenReinitProcessedFilter {
        pub task_number: u32,
        pub task_created_block: u32,
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
        name = "EigenUpdateProcessed",
        abi = "EigenUpdateProcessed(uint32,uint32)"
    )]
    pub struct EigenUpdateProcessedFilter {
        pub task_number: u32,
        pub task_created_block: u32,
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
        Hash,
    )]
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
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
    #[ethevent(name = "Paused", abi = "Paused(address,uint256)")]
    pub struct PausedFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        pub new_paused_status: ::ethers::core::types::U256,
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
    #[ethevent(name = "PauserRegistrySet", abi = "PauserRegistrySet(address,address)")]
    pub struct PauserRegistrySetFilter {
        pub pauser_registry: ::ethers::core::types::Address,
        pub new_pauser_registry: ::ethers::core::types::Address,
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
    #[ethevent(name = "Unpaused", abi = "Unpaused(address,uint256)")]
    pub struct UnpausedFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        pub new_paused_status: ::ethers::core::types::U256,
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
    pub enum GaspMultiRollupServiceEvents {
        EigenReinitProcessedFilter(EigenReinitProcessedFilter),
        EigenUpdateProcessedFilter(EigenUpdateProcessedFilter),
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PausedFilter(PausedFilter),
        PauserRegistrySetFilter(PauserRegistrySetFilter),
        UnpausedFilter(UnpausedFilter),
    }
    impl ::ethers::contract::EthLogDecode for GaspMultiRollupServiceEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = EigenReinitProcessedFilter::decode_log(log) {
                return Ok(GaspMultiRollupServiceEvents::EigenReinitProcessedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = EigenUpdateProcessedFilter::decode_log(log) {
                return Ok(GaspMultiRollupServiceEvents::EigenUpdateProcessedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(GaspMultiRollupServiceEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(GaspMultiRollupServiceEvents::OwnershipTransferredFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(GaspMultiRollupServiceEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = PauserRegistrySetFilter::decode_log(log) {
                return Ok(GaspMultiRollupServiceEvents::PauserRegistrySetFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(GaspMultiRollupServiceEvents::UnpausedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for GaspMultiRollupServiceEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::EigenReinitProcessedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::EigenUpdateProcessedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PausedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauserRegistrySetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnpausedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<EigenReinitProcessedFilter> for GaspMultiRollupServiceEvents {
        fn from(value: EigenReinitProcessedFilter) -> Self {
            Self::EigenReinitProcessedFilter(value)
        }
    }
    impl ::core::convert::From<EigenUpdateProcessedFilter> for GaspMultiRollupServiceEvents {
        fn from(value: EigenUpdateProcessedFilter) -> Self {
            Self::EigenUpdateProcessedFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for GaspMultiRollupServiceEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for GaspMultiRollupServiceEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<PausedFilter> for GaspMultiRollupServiceEvents {
        fn from(value: PausedFilter) -> Self {
            Self::PausedFilter(value)
        }
    }
    impl ::core::convert::From<PauserRegistrySetFilter> for GaspMultiRollupServiceEvents {
        fn from(value: PauserRegistrySetFilter) -> Self {
            Self::PauserRegistrySetFilter(value)
        }
    }
    impl ::core::convert::From<UnpausedFilter> for GaspMultiRollupServiceEvents {
        fn from(value: UnpausedFilter) -> Self {
            Self::UnpausedFilter(value)
        }
    }
    ///Container type for all input parameters for the `checkSignatures` function with signature `checkSignatures(bytes32,((uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256)))` and selector `0xc4e1914c`
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
        name = "checkSignatures",
        abi = "checkSignatures(bytes32,((uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256)))"
    )]
    pub struct CheckSignaturesCall {
        pub msg_hash: [u8; 32],
        pub params: NonSignerStakesAndSignatureForOldState,
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address,address)` and selector `0xc0c53b8b`
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
    #[ethcall(name = "initialize", abi = "initialize(address,address,address)")]
    pub struct InitializeCall {
        pub pauser_registry: ::ethers::core::types::Address,
        pub initial_owner: ::ethers::core::types::Address,
        pub updater: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `lastUpdateBlockTimestamp` function with signature `lastUpdateBlockTimestamp()` and selector `0xe61db175`
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
    #[ethcall(name = "lastUpdateBlockTimestamp", abi = "lastUpdateBlockTimestamp()")]
    pub struct LastUpdateBlockTimestampCall;
    ///Container type for all input parameters for the `latestCompletedTaskCreatedBlock` function with signature `latestCompletedTaskCreatedBlock()` and selector `0xed5a04fe`
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
        name = "latestCompletedTaskCreatedBlock",
        abi = "latestCompletedTaskCreatedBlock()"
    )]
    pub struct LatestCompletedTaskCreatedBlockCall;
    ///Container type for all input parameters for the `latestCompletedTaskNumber` function with signature `latestCompletedTaskNumber()` and selector `0xfc765dd5`
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
        name = "latestCompletedTaskNumber",
        abi = "latestCompletedTaskNumber()"
    )]
    pub struct LatestCompletedTaskNumberCall;
    ///Container type for all input parameters for the `latestPendingStateHash` function with signature `latestPendingStateHash()` and selector `0x4ae6b203`
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
    #[ethcall(name = "latestPendingStateHash", abi = "latestPendingStateHash()")]
    pub struct LatestPendingStateHashCall;
    ///Container type for all input parameters for the `operatorAndQuorumToStakes` function with signature `operatorAndQuorumToStakes(bytes32,uint8)` and selector `0x499d6fb6`
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
        name = "operatorAndQuorumToStakes",
        abi = "operatorAndQuorumToStakes(bytes32,uint8)"
    )]
    pub struct OperatorAndQuorumToStakesCall(pub [u8; 32], pub u8);
    ///Container type for all input parameters for the `operatorIdQuorumCount` function with signature `operatorIdQuorumCount(bytes32)` and selector `0x430d3b39`
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
    #[ethcall(name = "operatorIdQuorumCount", abi = "operatorIdQuorumCount(bytes32)")]
    pub struct OperatorIdQuorumCountCall(pub [u8; 32]);
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `pause` function with signature `pause(uint256)` and selector `0x136439dd`
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
    #[ethcall(name = "pause", abi = "pause(uint256)")]
    pub struct PauseCall {
        pub new_paused_status: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `pauseAll` function with signature `pauseAll()` and selector `0x595c6a67`
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
    #[ethcall(name = "pauseAll", abi = "pauseAll()")]
    pub struct PauseAllCall;
    ///Container type for all input parameters for the `paused` function with signature `paused(uint8)` and selector `0x5ac86ab7`
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
    #[ethcall(name = "paused", abi = "paused(uint8)")]
    pub struct PausedWithIndexCall {
        pub index: u8,
    }
    ///Container type for all input parameters for the `paused` function with signature `paused()` and selector `0x5c975abb`
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
    #[ethcall(name = "paused", abi = "paused()")]
    pub struct PausedCall;
    ///Container type for all input parameters for the `pauserRegistry` function with signature `pauserRegistry()` and selector `0x886f1195`
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
    #[ethcall(name = "pauserRegistry", abi = "pauserRegistry()")]
    pub struct PauserRegistryCall;
    ///Container type for all input parameters for the `process_eigen_reinit` function with signature `process_eigen_reinit((uint32,uint256,uint32,uint32,bytes,uint32,bytes,uint32),(uint32,bytes32,bytes32,bytes32,bytes32,bytes32),(bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]))` and selector `0xc927feef`
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
        name = "process_eigen_reinit",
        abi = "process_eigen_reinit((uint32,uint256,uint32,uint32,bytes,uint32,bytes,uint32),(uint32,bytes32,bytes32,bytes32,bytes32,bytes32),(bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]))"
    )]
    pub struct ProcessEigenReinitCall {
        pub task: Task,
        pub task_response: TaskResponse,
        pub operator_state_info: OperatorStateInfo,
    }
    ///Container type for all input parameters for the `process_eigen_update` function with signature `process_eigen_update((uint32,uint256,uint32,uint32,bytes,uint32,bytes,uint32),(uint32,bytes32,bytes32,bytes32,bytes32,bytes32),((uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256)),(bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]))` and selector `0x3e404926`
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
        name = "process_eigen_update",
        abi = "process_eigen_update((uint32,uint256,uint32,uint32,bytes,uint32,bytes,uint32),(uint32,bytes32,bytes32,bytes32,bytes32,bytes32),((uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256)),(bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]))"
    )]
    pub struct ProcessEigenUpdateCall {
        pub task: Task,
        pub task_response: TaskResponse,
        pub non_signer_stakes_and_signature_for_old_state: NonSignerStakesAndSignatureForOldState,
        pub operator_state_info: OperatorStateInfo,
    }
    ///Container type for all input parameters for the `qourumApk` function with signature `qourumApk(uint8)` and selector `0x03d097d2`
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
    #[ethcall(name = "qourumApk", abi = "qourumApk(uint8)")]
    pub struct QourumApkCall(pub u8);
    ///Container type for all input parameters for the `quorumNumbers` function with signature `quorumNumbers()` and selector `0x2a8414fd`
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
    #[ethcall(name = "quorumNumbers", abi = "quorumNumbers()")]
    pub struct QuorumNumbersCall;
    ///Container type for all input parameters for the `quorumThresholdPercentage` function with signature `quorumThresholdPercentage()` and selector `0x4deabc21`
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
        name = "quorumThresholdPercentage",
        abi = "quorumThresholdPercentage()"
    )]
    pub struct QuorumThresholdPercentageCall;
    ///Container type for all input parameters for the `quorumToStakes` function with signature `quorumToStakes(uint8)` and selector `0x7ad75561`
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
    #[ethcall(name = "quorumToStakes", abi = "quorumToStakes(uint8)")]
    pub struct QuorumToStakesCall(pub u8);
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
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
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `setPauserRegistry` function with signature `setPauserRegistry(address)` and selector `0x10d67a2f`
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
    #[ethcall(name = "setPauserRegistry", abi = "setPauserRegistry(address)")]
    pub struct SetPauserRegistryCall {
        pub new_pauser_registry: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `set_updater` function with signature `set_updater(address)` and selector `0x124648c9`
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
    #[ethcall(name = "set_updater", abi = "set_updater(address)")]
    pub struct SetUpdaterCall {
        pub updater: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `stalled` function with signature `stalled()` and selector `0x526e3e64`
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
    #[ethcall(name = "stalled", abi = "stalled()")]
    pub struct StalledCall;
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
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
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `trySignatureAndApkVerification` function with signature `trySignatureAndApkVerification(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256))` and selector `0x171f1d5b`
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
        name = "trySignatureAndApkVerification",
        abi = "trySignatureAndApkVerification(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256))"
    )]
    pub struct TrySignatureAndApkVerificationCall {
        pub msg_hash: [u8; 32],
        pub apk: G1Point,
        pub apk_g2: G2Point,
        pub sigma: G1Point,
    }
    ///Container type for all input parameters for the `unpause` function with signature `unpause(uint256)` and selector `0xfabc1cbc`
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
    #[ethcall(name = "unpause", abi = "unpause(uint256)")]
    pub struct UnpauseCall {
        pub new_paused_status: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `updater` function with signature `updater()` and selector `0xdf034cd0`
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
    #[ethcall(name = "updater", abi = "updater()")]
    pub struct UpdaterCall;
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
    pub enum GaspMultiRollupServiceCalls {
        CheckSignatures(CheckSignaturesCall),
        Initialize(InitializeCall),
        LastUpdateBlockTimestamp(LastUpdateBlockTimestampCall),
        LatestCompletedTaskCreatedBlock(LatestCompletedTaskCreatedBlockCall),
        LatestCompletedTaskNumber(LatestCompletedTaskNumberCall),
        LatestPendingStateHash(LatestPendingStateHashCall),
        OperatorAndQuorumToStakes(OperatorAndQuorumToStakesCall),
        OperatorIdQuorumCount(OperatorIdQuorumCountCall),
        Owner(OwnerCall),
        Pause(PauseCall),
        PauseAll(PauseAllCall),
        PausedWithIndex(PausedWithIndexCall),
        Paused(PausedCall),
        PauserRegistry(PauserRegistryCall),
        ProcessEigenReinit(ProcessEigenReinitCall),
        ProcessEigenUpdate(ProcessEigenUpdateCall),
        QourumApk(QourumApkCall),
        QuorumNumbers(QuorumNumbersCall),
        QuorumThresholdPercentage(QuorumThresholdPercentageCall),
        QuorumToStakes(QuorumToStakesCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetPauserRegistry(SetPauserRegistryCall),
        SetUpdater(SetUpdaterCall),
        Stalled(StalledCall),
        TransferOwnership(TransferOwnershipCall),
        TrySignatureAndApkVerification(TrySignatureAndApkVerificationCall),
        Unpause(UnpauseCall),
        Updater(UpdaterCall),
    }
    impl ::ethers::core::abi::AbiDecode for GaspMultiRollupServiceCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <CheckSignaturesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CheckSignatures(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) =
                <LastUpdateBlockTimestampCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LastUpdateBlockTimestamp(decoded));
            }
            if let Ok(decoded) =
                <LatestCompletedTaskCreatedBlockCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::LatestCompletedTaskCreatedBlock(decoded));
            }
            if let Ok(decoded) =
                <LatestCompletedTaskNumberCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LatestCompletedTaskNumber(decoded));
            }
            if let Ok(decoded) =
                <LatestPendingStateHashCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LatestPendingStateHash(decoded));
            }
            if let Ok(decoded) =
                <OperatorAndQuorumToStakesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OperatorAndQuorumToStakes(decoded));
            }
            if let Ok(decoded) =
                <OperatorIdQuorumCountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OperatorIdQuorumCount(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <PauseCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Pause(decoded));
            }
            if let Ok(decoded) = <PauseAllCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PauseAll(decoded));
            }
            if let Ok(decoded) =
                <PausedWithIndexCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PausedWithIndex(decoded));
            }
            if let Ok(decoded) = <PausedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Paused(decoded));
            }
            if let Ok(decoded) =
                <PauserRegistryCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PauserRegistry(decoded));
            }
            if let Ok(decoded) =
                <ProcessEigenReinitCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ProcessEigenReinit(decoded));
            }
            if let Ok(decoded) =
                <ProcessEigenUpdateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ProcessEigenUpdate(decoded));
            }
            if let Ok(decoded) = <QourumApkCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::QourumApk(decoded));
            }
            if let Ok(decoded) = <QuorumNumbersCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::QuorumNumbers(decoded));
            }
            if let Ok(decoded) =
                <QuorumThresholdPercentageCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::QuorumThresholdPercentage(decoded));
            }
            if let Ok(decoded) =
                <QuorumToStakesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::QuorumToStakes(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <SetPauserRegistryCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetPauserRegistry(decoded));
            }
            if let Ok(decoded) = <SetUpdaterCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetUpdater(decoded));
            }
            if let Ok(decoded) = <StalledCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Stalled(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <TrySignatureAndApkVerificationCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TrySignatureAndApkVerification(decoded));
            }
            if let Ok(decoded) = <UnpauseCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Unpause(decoded));
            }
            if let Ok(decoded) = <UpdaterCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Updater(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for GaspMultiRollupServiceCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CheckSignatures(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LastUpdateBlockTimestamp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LatestCompletedTaskCreatedBlock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LatestCompletedTaskNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LatestPendingStateHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OperatorAndQuorumToStakes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OperatorIdQuorumCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PauseAll(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PausedWithIndex(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Paused(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PauserRegistry(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProcessEigenReinit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProcessEigenUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QourumApk(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::QuorumNumbers(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::QuorumThresholdPercentage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QuorumToStakes(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetPauserRegistry(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetUpdater(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Stalled(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TrySignatureAndApkVerification(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unpause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Updater(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for GaspMultiRollupServiceCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CheckSignatures(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::LastUpdateBlockTimestamp(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestCompletedTaskCreatedBlock(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LatestCompletedTaskNumber(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestPendingStateHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorAndQuorumToStakes(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorIdQuorumCount(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pause(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauseAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::PausedWithIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::Paused(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauserRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProcessEigenReinit(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProcessEigenUpdate(element) => ::core::fmt::Display::fmt(element, f),
                Self::QourumApk(element) => ::core::fmt::Display::fmt(element, f),
                Self::QuorumNumbers(element) => ::core::fmt::Display::fmt(element, f),
                Self::QuorumThresholdPercentage(element) => ::core::fmt::Display::fmt(element, f),
                Self::QuorumToStakes(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPauserRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetUpdater(element) => ::core::fmt::Display::fmt(element, f),
                Self::Stalled(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::TrySignatureAndApkVerification(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Unpause(element) => ::core::fmt::Display::fmt(element, f),
                Self::Updater(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CheckSignaturesCall> for GaspMultiRollupServiceCalls {
        fn from(value: CheckSignaturesCall) -> Self {
            Self::CheckSignatures(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for GaspMultiRollupServiceCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<LastUpdateBlockTimestampCall> for GaspMultiRollupServiceCalls {
        fn from(value: LastUpdateBlockTimestampCall) -> Self {
            Self::LastUpdateBlockTimestamp(value)
        }
    }
    impl ::core::convert::From<LatestCompletedTaskCreatedBlockCall> for GaspMultiRollupServiceCalls {
        fn from(value: LatestCompletedTaskCreatedBlockCall) -> Self {
            Self::LatestCompletedTaskCreatedBlock(value)
        }
    }
    impl ::core::convert::From<LatestCompletedTaskNumberCall> for GaspMultiRollupServiceCalls {
        fn from(value: LatestCompletedTaskNumberCall) -> Self {
            Self::LatestCompletedTaskNumber(value)
        }
    }
    impl ::core::convert::From<LatestPendingStateHashCall> for GaspMultiRollupServiceCalls {
        fn from(value: LatestPendingStateHashCall) -> Self {
            Self::LatestPendingStateHash(value)
        }
    }
    impl ::core::convert::From<OperatorAndQuorumToStakesCall> for GaspMultiRollupServiceCalls {
        fn from(value: OperatorAndQuorumToStakesCall) -> Self {
            Self::OperatorAndQuorumToStakes(value)
        }
    }
    impl ::core::convert::From<OperatorIdQuorumCountCall> for GaspMultiRollupServiceCalls {
        fn from(value: OperatorIdQuorumCountCall) -> Self {
            Self::OperatorIdQuorumCount(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for GaspMultiRollupServiceCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PauseCall> for GaspMultiRollupServiceCalls {
        fn from(value: PauseCall) -> Self {
            Self::Pause(value)
        }
    }
    impl ::core::convert::From<PauseAllCall> for GaspMultiRollupServiceCalls {
        fn from(value: PauseAllCall) -> Self {
            Self::PauseAll(value)
        }
    }
    impl ::core::convert::From<PausedWithIndexCall> for GaspMultiRollupServiceCalls {
        fn from(value: PausedWithIndexCall) -> Self {
            Self::PausedWithIndex(value)
        }
    }
    impl ::core::convert::From<PausedCall> for GaspMultiRollupServiceCalls {
        fn from(value: PausedCall) -> Self {
            Self::Paused(value)
        }
    }
    impl ::core::convert::From<PauserRegistryCall> for GaspMultiRollupServiceCalls {
        fn from(value: PauserRegistryCall) -> Self {
            Self::PauserRegistry(value)
        }
    }
    impl ::core::convert::From<ProcessEigenReinitCall> for GaspMultiRollupServiceCalls {
        fn from(value: ProcessEigenReinitCall) -> Self {
            Self::ProcessEigenReinit(value)
        }
    }
    impl ::core::convert::From<ProcessEigenUpdateCall> for GaspMultiRollupServiceCalls {
        fn from(value: ProcessEigenUpdateCall) -> Self {
            Self::ProcessEigenUpdate(value)
        }
    }
    impl ::core::convert::From<QourumApkCall> for GaspMultiRollupServiceCalls {
        fn from(value: QourumApkCall) -> Self {
            Self::QourumApk(value)
        }
    }
    impl ::core::convert::From<QuorumNumbersCall> for GaspMultiRollupServiceCalls {
        fn from(value: QuorumNumbersCall) -> Self {
            Self::QuorumNumbers(value)
        }
    }
    impl ::core::convert::From<QuorumThresholdPercentageCall> for GaspMultiRollupServiceCalls {
        fn from(value: QuorumThresholdPercentageCall) -> Self {
            Self::QuorumThresholdPercentage(value)
        }
    }
    impl ::core::convert::From<QuorumToStakesCall> for GaspMultiRollupServiceCalls {
        fn from(value: QuorumToStakesCall) -> Self {
            Self::QuorumToStakes(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for GaspMultiRollupServiceCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SetPauserRegistryCall> for GaspMultiRollupServiceCalls {
        fn from(value: SetPauserRegistryCall) -> Self {
            Self::SetPauserRegistry(value)
        }
    }
    impl ::core::convert::From<SetUpdaterCall> for GaspMultiRollupServiceCalls {
        fn from(value: SetUpdaterCall) -> Self {
            Self::SetUpdater(value)
        }
    }
    impl ::core::convert::From<StalledCall> for GaspMultiRollupServiceCalls {
        fn from(value: StalledCall) -> Self {
            Self::Stalled(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for GaspMultiRollupServiceCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<TrySignatureAndApkVerificationCall> for GaspMultiRollupServiceCalls {
        fn from(value: TrySignatureAndApkVerificationCall) -> Self {
            Self::TrySignatureAndApkVerification(value)
        }
    }
    impl ::core::convert::From<UnpauseCall> for GaspMultiRollupServiceCalls {
        fn from(value: UnpauseCall) -> Self {
            Self::Unpause(value)
        }
    }
    impl ::core::convert::From<UpdaterCall> for GaspMultiRollupServiceCalls {
        fn from(value: UpdaterCall) -> Self {
            Self::Updater(value)
        }
    }
    ///Container type for all return fields from the `checkSignatures` function with signature `checkSignatures(bytes32,((uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256)))` and selector `0xc4e1914c`
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
    pub struct CheckSignaturesReturn(pub QuorumStakeTotals);
    ///Container type for all return fields from the `lastUpdateBlockTimestamp` function with signature `lastUpdateBlockTimestamp()` and selector `0xe61db175`
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
    pub struct LastUpdateBlockTimestampReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `latestCompletedTaskCreatedBlock` function with signature `latestCompletedTaskCreatedBlock()` and selector `0xed5a04fe`
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
    pub struct LatestCompletedTaskCreatedBlockReturn(pub u32);
    ///Container type for all return fields from the `latestCompletedTaskNumber` function with signature `latestCompletedTaskNumber()` and selector `0xfc765dd5`
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
    pub struct LatestCompletedTaskNumberReturn(pub u32);
    ///Container type for all return fields from the `latestPendingStateHash` function with signature `latestPendingStateHash()` and selector `0x4ae6b203`
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
    pub struct LatestPendingStateHashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `operatorAndQuorumToStakes` function with signature `operatorAndQuorumToStakes(bytes32,uint8)` and selector `0x499d6fb6`
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
    pub struct OperatorAndQuorumToStakesReturn(pub u128);
    ///Container type for all return fields from the `operatorIdQuorumCount` function with signature `operatorIdQuorumCount(bytes32)` and selector `0x430d3b39`
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
    pub struct OperatorIdQuorumCountReturn(pub u8);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `paused` function with signature `paused(uint8)` and selector `0x5ac86ab7`
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
    pub struct PausedWithIndexReturn(pub bool);
    ///Container type for all return fields from the `paused` function with signature `paused()` and selector `0x5c975abb`
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
    pub struct PausedReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `pauserRegistry` function with signature `pauserRegistry()` and selector `0x886f1195`
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
    pub struct PauserRegistryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `qourumApk` function with signature `qourumApk(uint8)` and selector `0x03d097d2`
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
    pub struct QourumApkReturn {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `quorumNumbers` function with signature `quorumNumbers()` and selector `0x2a8414fd`
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
    pub struct QuorumNumbersReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `quorumThresholdPercentage` function with signature `quorumThresholdPercentage()` and selector `0x4deabc21`
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
    pub struct QuorumThresholdPercentageReturn(pub u32);
    ///Container type for all return fields from the `quorumToStakes` function with signature `quorumToStakes(uint8)` and selector `0x7ad75561`
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
    pub struct QuorumToStakesReturn(pub u128);
    ///Container type for all return fields from the `stalled` function with signature `stalled()` and selector `0x526e3e64`
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
    pub struct StalledReturn(pub bool);
    ///Container type for all return fields from the `trySignatureAndApkVerification` function with signature `trySignatureAndApkVerification(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256))` and selector `0x171f1d5b`
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
    pub struct TrySignatureAndApkVerificationReturn {
        pub pairing_successful: bool,
        pub siganature_is_valid: bool,
    }
    ///Container type for all return fields from the `updater` function with signature `updater()` and selector `0xdf034cd0`
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
    pub struct UpdaterReturn(pub ::ethers::core::types::Address);
}
