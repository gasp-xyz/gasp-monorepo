pub use gasp_multi_rollup_service::*;
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_rolldown"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IRolldown"),
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
                    ::std::borrow::ToOwned::to_owned("rolldown"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rolldown"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IRolldown"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                    ::std::borrow::ToOwned::to_owned("set_rolldown"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("set_rolldown"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_rolldown"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IRolldown"),
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
    pub static GASPMULTIROLLUPSERVICE_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[PaIe\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xDAW`\x005`\xE0\x1C\x80cZ\xC8j\xB7\x11a\x01\x04W\x80c\xC9'\xFE\xEF\x11a\0\xA2W\x80c\xF2\xFD\xE3\x8B\x11a\0qW\x80c\xF2\xFD\xE3\x8B\x14a\x04\xBBW\x80c\xF8\xC8v^\x14a\x04\xCEW\x80c\xFA\xBC\x1C\xBC\x14a\x04\xE1W\x80c\xFCv]\xD5\x14a\x04\xF4W`\0\x80\xFD[\x80c\xC9'\xFE\xEF\x14a\x04tW\x80c\xDF\x03L\xD0\x14a\x04\x87W\x80c\xE6\x1D\xB1u\x14a\x04\x9AW\x80c\xEDZ\x04\xFE\x14a\x04\xA3W`\0\x80\xFD[\x80cz\xD7Ua\x11a\0\xDEW\x80cz\xD7Ua\x14a\x04\x07W\x80c\x88o\x11\x95\x14a\x040W\x80c\x8D\xA5\xCB[\x14a\x04CW\x80c\xC4\xE1\x91L\x14a\x04TW`\0\x80\xFD[\x80cZ\xC8j\xB7\x14a\x03\xD4W\x80c\\\x97Z\xBB\x14a\x03\xF7W\x80cqP\x18\xA6\x14a\x03\xFFW`\0\x80\xFD[\x80c=\x9F\xB0\x0C\x11a\x01|W\x80cJ\xE6\xB2\x03\x11a\x01KW\x80cJ\xE6\xB2\x03\x14a\x03lW\x80cM\xEA\xBC!\x14a\x03\x83W\x80cRn>d\x14a\x03\xA8W\x80cY\\jg\x14a\x03\xCCW`\0\x80\xFD[\x80c=\x9F\xB0\x0C\x14a\x02\xADW\x80c>@I&\x14a\x02\xD8W\x80cC\r;9\x14a\x02\xEBW\x80cI\x9Do\xB6\x14a\x03 W`\0\x80\xFD[\x80c\x13d9\xDD\x11a\x01\xB8W\x80c\x13d9\xDD\x14a\x02HW\x80c\x17\x1F\x1D[\x14a\x02[W\x80c\x1E-K\xF7\x14a\x02\x85W\x80c*\x84\x14\xFD\x14a\x02\x98W`\0\x80\xFD[\x80c\x03\xD0\x97\xD2\x14a\x01\xDFW\x80c\x10\xD6z/\x14a\x02 W\x80c\x12FH\xC9\x14a\x025W[`\0\x80\xFD[a\x02\x06a\x01\xED6`\x04a7\x8CV[`\xA0` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T\x82V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[a\x023a\x02.6`\x04a7\xC3V[a\x05\x04V[\0[a\x023a\x02C6`\x04a7\xC3V[a\x05\xC0V[a\x023a\x02V6`\x04a7\xE0V[a\x05\xEAV[a\x02na\x02i6`\x04a9]V[a\x07)V[`@\x80Q\x92\x15\x15\x83R\x90\x15\x15` \x83\x01R\x01a\x02\x17V[a\x023a\x02\x936`\x04a7\xC3V[a\x08\xB3V[a\x02\xA0a\x08\xDDV[`@Qa\x02\x17\x91\x90a9\xAEV[`\x98Ta\x02\xC0\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x17V[a\x023a\x02\xE66`\x04a:AV[a\tkV[a\x03\x0Ea\x02\xF96`\x04a7\xE0V[`\xA1` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02\x17V[a\x03Ta\x03.6`\x04a:\xE4V[`\x9F` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\x01`\x01``\x1B\x03\x16\x81V[`@Q`\x01`\x01``\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x17V[a\x03u`\x9AT\x81V[`@Q\x90\x81R` \x01a\x02\x17V[`\x9DTa\x03\x93\x90c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\x17V[`\x98Ta\x03\xBC\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02\x17V[a\x023a\x17\x84V[a\x03\xBCa\x03\xE26`\x04a7\x8CV[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`fTa\x03uV[a\x023a\x18KV[a\x03Ta\x04\x156`\x04a7\x8CV[`\x9E` R`\0\x90\x81R`@\x90 T`\x01`\x01``\x1B\x03\x16\x81V[`eTa\x02\xC0\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x02\xC0V[a\x04ga\x04b6`\x04a;\xF0V[a\x18_V[`@Qa\x02\x17\x91\x90a<zV[a\x023a\x04\x826`\x04a<\xBCV[a\x1ELV[`\x97Ta\x02\xC0\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03u`\x99T\x81V[`\x9BTa\x03\x93\x90d\x01\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x023a\x04\xC96`\x04a7\xC3V[a)\x10V[a\x023a\x04\xDC6`\x04a=1V[a)\x86V[a\x023a\x04\xEF6`\x04a7\xE0V[a*\xD7V[`\x9BTa\x03\x93\x90c\xFF\xFF\xFF\xFF\x16\x81V[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05WW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05{\x91\x90a=\x8DV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x05\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xAB\x90a=\xAAV[`@Q\x80\x91\x03\x90\xFD[a\x05\xBD\x81a,3V[PV[a\x05\xC8a-*V[`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x062W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06V\x91\x90a>\x02V[a\x06rW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xAB\x90a>\x1FV[`fT\x81\x81\x16\x14a\x06\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\xABV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`\0\x80`\0\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x87\x87`\0\x01Q\x88` \x01Q\x88`\0\x01Q`\0`\x02\x81\x10a\x07qWa\x07qa>gV[` \x02\x01Q\x89Q`\x01` \x02\x01Q\x8A` \x01Q`\0`\x02\x81\x10a\x07\x96Wa\x07\x96a>gV[` \x02\x01Q\x8B` \x01Q`\x01`\x02\x81\x10a\x07\xB2Wa\x07\xB2a>gV[` \x90\x81\x02\x91\x90\x91\x01Q\x8CQ\x8D\x83\x01Q`@Qa\x08\x0F\x9A\x99\x98\x97\x96\x95\x94\x01\x98\x89R` \x89\x01\x97\x90\x97R`@\x88\x01\x95\x90\x95R``\x87\x01\x93\x90\x93R`\x80\x86\x01\x91\x90\x91R`\xA0\x85\x01R`\xC0\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01Ra\x01 \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\0\x1Ca\x082\x91\x90a>}V[\x90Pa\x08\xA5a\x08Ka\x08D\x88\x84a-\x84V[\x86\x90a.\x1BV[a\x08Sa.\xAFV[a\x08\x9Ba\x08\x8C\x85a\x08\x86`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x01\x82R`\x02\x90\x82\x01R\x90V[\x90a-\x84V[a\x08\x95\x8Ca/oV[\x90a.\x1BV[\x88b\x01\xD4\xC0a/\xFFV[\x90\x98\x90\x97P\x95PPPPPPV[a\x08\xBBa-*V[`\x98\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x9C\x80Ta\x08\xEA\x90a>\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\t\x16\x90a>\x9FV[\x80\x15a\tcW\x80`\x1F\x10a\t8Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\tcV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\tFW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\x97T`\x01`\x01`\xA0\x1B\x03\x163\x14a\t\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FUpdater must be the caller\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\xABV[a\t\xD5`\x80\x85\x01``\x86\x01a>\xE8V[`\x9BTd\x01\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14a\n7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7Freference block hash mismatch\0\0\0`D\x82\x01R`d\x01a\x05\xABV[\x83`@Q` \x01a\nH\x91\x90a?xV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x83` \x015\x14a\n\xB0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FreferenceTaskHash hash mismatch\0`D\x82\x01R`d\x01a\x05\xABV[\x80`@Q` \x01a\n\xC1\x91\x90aD\xE4V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x83`@\x015\x14a\x0B)W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FoperatorStateInfo hash mismatch\0`D\x82\x01R`d\x01a\x05\xABV[a\x0B9``\x85\x01`@\x86\x01a>\xE8V[`\x9BTc\xFF\xFF\xFF\xFF\x91\x82\x16\x91a\x0BZ\x91d\x01\0\0\0\0\x90\x04\x16a8@aFaV[c\xFF\xFF\xFF\xFF\x16\x11a\x0B\x9DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x077F\x16\xC6R\x077F\x17FR\x03`\x9C\x1B`D\x82\x01R`d\x01a\x05\xABV[B`\x99Tb\x03\xF4\x80a\x0B\xAF\x91\x90aF\x89V[\x11a\x0B\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rlstale state 1`\x98\x1B`D\x82\x01R`d\x01a\x05\xABV[`\x9BTd\x01\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x15a\x0C\xC6W`\0a\x0C\x15` \x85\x015a\x04b\x85aF\xA1V[`\x9DT\x90\x91Pc\xFF\xFF\xFF\xFF\x16`\0[`\x9C\x80Ta\x0C1\x90a>\x9FV[\x90P\x81\x10\x15a\x0C\xC2W\x81`\xFF\x16\x83` \x01Q\x82\x81Q\x81\x10a\x0CTWa\x0CTa>gV[` \x02` \x01\x01Qa\x0Cf\x91\x90aF\xADV[`\x01`\x01``\x1B\x03\x16`d\x84`\0\x01Q\x83\x81Q\x81\x10a\x0C\x87Wa\x0C\x87a>gV[` \x02` \x01\x01Q`\x01`\x01``\x1B\x03\x16a\x0C\xA2\x91\x90aF\xDCV[\x10\x15a\x0C\xB0WPPPa\x17~V[\x80a\x0C\xBA\x81aF\xFBV[\x91PPa\x0C$V[PPP[`\0[a\x0C\xD6` \x83\x01\x83aG\x16V[\x90P\x81\x10\x15a\r\x9BW`\x9E`\0a\x0C\xF0` \x85\x01\x85aG\x16V[\x84\x81\x81\x10a\r\0Wa\r\0a>gV[\x90P` \x02\x01` \x81\x01\x90a\r\x15\x91\x90a7\x8CV[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0\x90\x81 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U`\xA0\x91a\rH\x90\x85\x01\x85aG\x16V[\x84\x81\x81\x10a\rXWa\rXa>gV[\x90P` \x02\x01` \x81\x01\x90a\rm\x91\x90a7\x8CV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x81\x81U`\x01\x01U\x80a\r\x93\x81aF\xFBV[\x91PPa\x0C\xC9V[P`\0[a\r\xAC`@\x83\x01\x83aG_V[\x90P\x81\x10\x15a\x0E\xE7Wa\r\xC2`@\x83\x01\x83aG_V[\x82\x81\x81\x10a\r\xD2Wa\r\xD2a>gV[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\r\xEA\x91\x90aG\xA8V[`\x9E`\0a\r\xFB`@\x86\x01\x86aG_V[\x85\x81\x81\x10a\x0E\x0BWa\x0E\x0Ba>gV[a\x0E!\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa7\x8CV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x90\x81\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x17\x90\x91Ua\x0Ea\x90\x83\x01\x83aG_V[\x82\x81\x81\x10a\x0EqWa\x0Eqa>gV[\x90P`\x80\x02\x01`@\x01`\xA0`\0\x84\x80`@\x01\x90a\x0E\x8E\x91\x90aG_V[\x85\x81\x81\x10a\x0E\x9EWa\x0E\x9Ea>gV[a\x0E\xB4\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa7\x8CV[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a\x0E\xDF\x81aF\xFBV[\x91PPa\r\x9FV[P`\0[a\x0E\xF8``\x83\x01\x83aG\xC3V[\x90P\x81\x10\x15a\x0F\xB0Wa\x0F\x0E``\x83\x01\x83aG\xC3V[\x82\x81\x81\x10a\x0F\x1EWa\x0F\x1Ea>gV[\x90P`@\x02\x01` \x01` \x81\x01\x90a\x0F6\x91\x90aG\xA8V[`\x9E`\0a\x0FG``\x86\x01\x86aG\xC3V[\x85\x81\x81\x10a\x0FWWa\x0FWa>gV[a\x0Fm\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa7\x8CV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\x0F\xA8\x81aF\xFBV[\x91PPa\x0E\xEBV[P`\0[a\x0F\xC1`\x80\x83\x01\x83aH\x0CV[\x90P\x81\x10\x15a\x10]Wa\x0F\xD7`\x80\x83\x01\x83aH\x0CV[\x82\x81\x81\x10a\x0F\xE7Wa\x0F\xE7a>gV[\x90P``\x02\x01` \x01`\xA0`\0\x84\x80`\x80\x01\x90a\x10\x04\x91\x90aH\x0CV[\x85\x81\x81\x10a\x10\x14Wa\x10\x14a>gV[a\x10*\x92` ``\x90\x92\x02\x01\x90\x81\x01\x91Pa7\x8CV[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a\x10U\x81aF\xFBV[\x91PPa\x0F\xB4V[P`\0[a\x10n`\xA0\x83\x01\x83aG\x16V[\x90P\x81\x10\x15a\x11\x95W`\0[`\x9C\x80Ta\x10\x87\x90a>\x9FV[\x90P\x81\x10\x15a\x11@W`\x9F`\0a\x10\xA1`\xA0\x86\x01\x86aG\x16V[\x85\x81\x81\x10a\x10\xB1Wa\x10\xB1a>gV[\x90P` \x02\x015\x81R` \x01\x90\x81R` \x01`\0 `\0`\x9C\x83\x81Ta\x10\xD6\x90a>\x9FV[\x81\x10a\x10\xE4Wa\x10\xE4a>gV[\x81T`\x01\x16\x15a\x11\x03W\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U\x81a\x118\x81aF\xFBV[\x92PPa\x10zV[P`\xA1`\0a\x11R`\xA0\x85\x01\x85aG\x16V[\x84\x81\x81\x10a\x11bWa\x11ba>gV[` \x90\x81\x02\x92\x90\x92\x015\x83RP\x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\xFF\x19\x16\x90U\x80a\x11\x8D\x81aF\xFBV[\x91PPa\x10aV[P`\0[a\x11\xA6`\xC0\x83\x01\x83aG\x16V[\x90P\x81\x10\x15a\x13\xF6Wa\x11\xBC`\xC0\x83\x01\x83aG\x16V[\x82\x81\x81\x10a\x11\xCCWa\x11\xCCa>gV[\x90P` \x02\x81\x01\x90a\x11\xDE\x91\x90aHTV[a\x11\xEF\x90`\x80\x81\x01\x90``\x01a7\x8CV[`\xA1`\0a\x12\0`\xC0\x86\x01\x86aG\x16V[\x85\x81\x81\x10a\x12\x10Wa\x12\x10a>gV[\x90P` \x02\x81\x01\x90a\x12\"\x91\x90aHTV[`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP`\0[a\x12\\`\xC0\x84\x01\x84aG\x16V[\x83\x81\x81\x10a\x12lWa\x12la>gV[\x90P` \x02\x81\x01\x90a\x12~\x91\x90aHTV[a\x12\x8C\x90` \x81\x01\x90aG\x16V[\x90P\x81\x10\x15a\x13\xE3Wa\x12\xA2`\xC0\x84\x01\x84aG\x16V[\x83\x81\x81\x10a\x12\xB2Wa\x12\xB2a>gV[\x90P` \x02\x81\x01\x90a\x12\xC4\x91\x90aHTV[a\x12\xD2\x90`@\x81\x01\x90aG\x16V[\x82\x81\x81\x10a\x12\xE2Wa\x12\xE2a>gV[\x90P` \x02\x01` \x81\x01\x90a\x12\xF7\x91\x90aG\xA8V[`\x9F`\0a\x13\x08`\xC0\x87\x01\x87aG\x16V[\x86\x81\x81\x10a\x13\x18Wa\x13\x18a>gV[\x90P` \x02\x81\x01\x90a\x13*\x91\x90aHTV[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a\x13K`\xC0\x87\x01\x87aG\x16V[\x86\x81\x81\x10a\x13[Wa\x13[a>gV[\x90P` \x02\x81\x01\x90a\x13m\x91\x90aHTV[a\x13{\x90` \x81\x01\x90aG\x16V[\x85\x81\x81\x10a\x13\x8BWa\x13\x8Ba>gV[\x90P` \x02\x01` \x81\x01\x90a\x13\xA0\x91\x90a7\x8CV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\x13\xDB\x81aF\xFBV[\x91PPa\x12OV[P\x80a\x13\xEE\x81aF\xFBV[\x91PPa\x11\x99V[P`\0[a\x14\x07`\xE0\x83\x01\x83aG\x16V[\x90P\x81\x10\x15a\x15\xBAW`\0[a\x14 `\xE0\x84\x01\x84aG\x16V[\x83\x81\x81\x10a\x140Wa\x140a>gV[\x90P` \x02\x81\x01\x90a\x14B\x91\x90aHtV[a\x14P\x90` \x81\x01\x90aG\x16V[\x90P\x81\x10\x15a\x15\xA7Wa\x14f`\xE0\x84\x01\x84aG\x16V[\x83\x81\x81\x10a\x14vWa\x14va>gV[\x90P` \x02\x81\x01\x90a\x14\x88\x91\x90aHtV[a\x14\x96\x90`@\x81\x01\x90aG\x16V[\x82\x81\x81\x10a\x14\xA6Wa\x14\xA6a>gV[\x90P` \x02\x01` \x81\x01\x90a\x14\xBB\x91\x90aG\xA8V[`\x9F`\0a\x14\xCC`\xE0\x87\x01\x87aG\x16V[\x86\x81\x81\x10a\x14\xDCWa\x14\xDCa>gV[\x90P` \x02\x81\x01\x90a\x14\xEE\x91\x90aHtV[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a\x15\x0F`\xE0\x87\x01\x87aG\x16V[\x86\x81\x81\x10a\x15\x1FWa\x15\x1Fa>gV[\x90P` \x02\x81\x01\x90a\x151\x91\x90aHtV[a\x15?\x90` \x81\x01\x90aG\x16V[\x85\x81\x81\x10a\x15OWa\x15Oa>gV[\x90P` \x02\x01` \x81\x01\x90a\x15d\x91\x90a7\x8CV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\x15\x9F\x81aF\xFBV[\x91PPa\x14\x13V[P\x80a\x15\xB2\x81aF\xFBV[\x91PPa\x13\xFAV[P`\0[a\x15\xCCa\x01\0\x83\x01\x83aG\xC3V[\x90P\x81\x10\x15a\x16pWa\x15\xE3a\x01\0\x83\x01\x83aG\xC3V[\x82\x81\x81\x10a\x15\xF3Wa\x15\xF3a>gV[\x90P`@\x02\x01` \x01` \x81\x01\x90a\x16\x0B\x91\x90a7\x8CV[`\xA1`\0a\x16\x1Da\x01\0\x86\x01\x86aG\xC3V[\x85\x81\x81\x10a\x16-Wa\x16-a>gV[\x90P`@\x02\x01`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP\x80\x80a\x16h\x90aF\xFBV[\x91PPa\x15\xBEV[P`\xA0\x83\x015`\x9AUa\x16\x86` \x85\x01\x85a>\xE8V[`\x9B\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\x16\xB0``\x85\x01`@\x86\x01a>\xE8V[`\x9B\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90UB`\x99Ua\x16\xE8`\x80\x85\x01\x85aH\x8AV[a\x16\xF4\x91`\x9C\x91a6\x08V[Pa\x17\x05`\xC0\x85\x01`\xA0\x86\x01a>\xE8V[`\x9D\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x7F\x9A\x12\x8F\xE74\x7F\x1E\x11\xCA\"\xAA\x9D\xEBc.\xC9\xAB\xB0\x96\x08\xC1:\x99L`\xF7zV/F\x01qa\x17M` \x86\x01\x86a>\xE8V[a\x17]``\x87\x01`@\x88\x01a>\xE8V[`@\x80Qc\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1[PPPPV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\xCCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xF0\x91\x90a>\x02V[a\x18\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xAB\x90a>\x1FV[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[a\x18Sa-*V[a\x18]`\0a2#V[V[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01\x81\x90R`\x9C\x80Ta\x18\x96\x90a>\x9FV[\x90P\x90Pa\x18\xB7`@Q\x80`@\x01`@R\x80``\x81R` \x01``\x81RP\x90V[\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18\xCFWa\x18\xCFa7\xF9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x18\xF8W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P` \x82\x01R\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19\x16Wa\x19\x16a7\xF9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x19?W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x81R\x84QQ`\0\x90`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19_Wa\x19_a7\xF9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x19\x88W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80[\x87QQ\x81\x10\x15a\x1BCWa\x19\xD1\x88`\0\x01Q\x82\x81Q\x81\x10a\x19\xB2Wa\x19\xB2a>gV[` \x02` \x01\x01Q\x80Q`\0\x90\x81R` \x91\x82\x01Q\x90\x91R`@\x90 \x90V[\x83\x82\x81Q\x81\x10a\x19\xE3Wa\x19\xE3a>gV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80\x15a\x1A\xC1W\x82a\x1A\0`\x01\x83aH\xD0V[\x81Q\x81\x10a\x1A\x10Wa\x1A\x10a>gV[` \x02` \x01\x01Q`\0\x1C\x83\x82\x81Q\x81\x10a\x1A-Wa\x1A-a>gV[` \x02` \x01\x01Q`\0\x1C\x11a\x1A\xC1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`M`$\x82\x01R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: nonSignerG1PubkeysForOldSta`d\x82\x01Rl\x1D\x19H\x1B\x9B\xDD\x08\x1C\xDB\xDC\x9D\x19Y`\x9A\x1B`\x84\x82\x01R`\xA4\x01a\x05\xABV[a\x1B/a\x1B(`\xA1`\0\x86\x85\x81Q\x81\x10a\x1A\xDDWa\x1A\xDDa>gV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82R\x81\x01\x91\x90\x91R`@\x01`\0 T\x8AQ\x80Q`\xFF\x90\x92\x16\x91\x85\x90\x81\x10a\x1B\x12Wa\x1B\x12a>gV[` \x02` \x01\x01Qa2u\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x87\x90a.\x1BV[\x95P\x80a\x1B;\x81aF\xFBV[\x91PPa\x19\x8FV[Pa\x1BM\x85a3YV[\x94P`\0[\x84\x81\x10\x15a\x1D.W`\x9C\x81\x81Ta\x1Bh\x90a>\x9FV[\x81\x10a\x1BvWa\x1Bva>gV[\x81T`\x01\x16\x15a\x1B\x95W\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C`\0\x81\x81R`\xA0` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x01T\x90\x82\x01R\x90\x92Pa\x1B\xD6\x90\x87\x90a.\x1BV[`\xFF\x83\x16`\0\x90\x81R`\x9E` \x90\x81R`@\x90\x91 T\x90\x86\x01Q\x80Q\x92\x98P`\x01`\x01``\x1B\x03\x90\x91\x16\x91\x83\x90\x81\x10a\x1C\x11Wa\x1C\x11a>gV[`\x01`\x01``\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x82\x01R\x84\x01Q\x80Q\x82\x90\x81\x10a\x1C=Wa\x1C=a>gV[` \x02` \x01\x01Q\x84`\0\x01Q\x82\x81Q\x81\x10a\x1C[Wa\x1C[a>gV[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPP`\0[\x88QQ\x81\x10\x15a\x1D\x1BW`\x9F`\0\x85\x83\x81Q\x81\x10a\x1C\x9EWa\x1C\x9Ea>gV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82R\x81\x81\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 `\xFF\x87\x16\x82R\x90\x92R\x90 T\x85Q\x80Q`\x01`\x01``\x1B\x03\x90\x92\x16\x91\x84\x90\x81\x10a\x1C\xE9Wa\x1C\xE9a>gV[` \x02` \x01\x01\x81\x81Qa\x1C\xFD\x91\x90aH\xE7V[`\x01`\x01``\x1B\x03\x16\x90RP\x80a\x1D\x13\x81aF\xFBV[\x91PPa\x1C~V[P\x80a\x1D&\x81aF\xFBV[\x91PPa\x1BRV[P`\0\x80a\x1DF\x8A\x88\x8B` \x01Q\x8C`@\x01Qa\x07)V[\x91P\x91P\x81a\x1D\xC9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: pairing precompile call fai`d\x82\x01Rb\x1B\x19Y`\xEA\x1B`\x84\x82\x01R`\xA4\x01a\x05\xABV[\x80a\x1E<W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: signature is invalid\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\xABV[P\x92\x95PPPPPP[\x92\x91PPV[a\x1ETa-*V[`\0[a\x1Ed` \x83\x01\x83aG\x16V[\x90P\x81\x10\x15a\x1F)W`\x9E`\0a\x1E~` \x85\x01\x85aG\x16V[\x84\x81\x81\x10a\x1E\x8EWa\x1E\x8Ea>gV[\x90P` \x02\x01` \x81\x01\x90a\x1E\xA3\x91\x90a7\x8CV[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0\x90\x81 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U`\xA0\x91a\x1E\xD6\x90\x85\x01\x85aG\x16V[\x84\x81\x81\x10a\x1E\xE6Wa\x1E\xE6a>gV[\x90P` \x02\x01` \x81\x01\x90a\x1E\xFB\x91\x90a7\x8CV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x81\x81U`\x01\x01U\x80a\x1F!\x81aF\xFBV[\x91PPa\x1EWV[P`\0[a\x1F:`@\x83\x01\x83aG_V[\x90P\x81\x10\x15a uWa\x1FP`@\x83\x01\x83aG_V[\x82\x81\x81\x10a\x1F`Wa\x1F`a>gV[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\x1Fx\x91\x90aG\xA8V[`\x9E`\0a\x1F\x89`@\x86\x01\x86aG_V[\x85\x81\x81\x10a\x1F\x99Wa\x1F\x99a>gV[a\x1F\xAF\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa7\x8CV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x90\x81\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x17\x90\x91Ua\x1F\xEF\x90\x83\x01\x83aG_V[\x82\x81\x81\x10a\x1F\xFFWa\x1F\xFFa>gV[\x90P`\x80\x02\x01`@\x01`\xA0`\0\x84\x80`@\x01\x90a \x1C\x91\x90aG_V[\x85\x81\x81\x10a ,Wa ,a>gV[a B\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa7\x8CV[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a m\x81aF\xFBV[\x91PPa\x1F-V[P`\0[a \x86``\x83\x01\x83aG\xC3V[\x90P\x81\x10\x15a!>Wa \x9C``\x83\x01\x83aG\xC3V[\x82\x81\x81\x10a \xACWa \xACa>gV[\x90P`@\x02\x01` \x01` \x81\x01\x90a \xC4\x91\x90aG\xA8V[`\x9E`\0a \xD5``\x86\x01\x86aG\xC3V[\x85\x81\x81\x10a \xE5Wa \xE5a>gV[a \xFB\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa7\x8CV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a!6\x81aF\xFBV[\x91PPa yV[P`\0[a!O`\x80\x83\x01\x83aH\x0CV[\x90P\x81\x10\x15a!\xEBWa!e`\x80\x83\x01\x83aH\x0CV[\x82\x81\x81\x10a!uWa!ua>gV[\x90P``\x02\x01` \x01`\xA0`\0\x84\x80`\x80\x01\x90a!\x92\x91\x90aH\x0CV[\x85\x81\x81\x10a!\xA2Wa!\xA2a>gV[a!\xB8\x92` ``\x90\x92\x02\x01\x90\x81\x01\x91Pa7\x8CV[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a!\xE3\x81aF\xFBV[\x91PPa!BV[P`\0[a!\xFC`\xA0\x83\x01\x83aG\x16V[\x90P\x81\x10\x15a##W`\0[`\x9C\x80Ta\"\x15\x90a>\x9FV[\x90P\x81\x10\x15a\"\xCEW`\x9F`\0a\"/`\xA0\x86\x01\x86aG\x16V[\x85\x81\x81\x10a\"?Wa\"?a>gV[\x90P` \x02\x015\x81R` \x01\x90\x81R` \x01`\0 `\0`\x9C\x83\x81Ta\"d\x90a>\x9FV[\x81\x10a\"rWa\"ra>gV[\x81T`\x01\x16\x15a\"\x91W\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U\x81a\"\xC6\x81aF\xFBV[\x92PPa\"\x08V[P`\xA1`\0a\"\xE0`\xA0\x85\x01\x85aG\x16V[\x84\x81\x81\x10a\"\xF0Wa\"\xF0a>gV[` \x90\x81\x02\x92\x90\x92\x015\x83RP\x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\xFF\x19\x16\x90U\x80a#\x1B\x81aF\xFBV[\x91PPa!\xEFV[P`\0[a#4`\xC0\x83\x01\x83aG\x16V[\x90P\x81\x10\x15a%\x84Wa#J`\xC0\x83\x01\x83aG\x16V[\x82\x81\x81\x10a#ZWa#Za>gV[\x90P` \x02\x81\x01\x90a#l\x91\x90aHTV[a#}\x90`\x80\x81\x01\x90``\x01a7\x8CV[`\xA1`\0a#\x8E`\xC0\x86\x01\x86aG\x16V[\x85\x81\x81\x10a#\x9EWa#\x9Ea>gV[\x90P` \x02\x81\x01\x90a#\xB0\x91\x90aHTV[`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP`\0[a#\xEA`\xC0\x84\x01\x84aG\x16V[\x83\x81\x81\x10a#\xFAWa#\xFAa>gV[\x90P` \x02\x81\x01\x90a$\x0C\x91\x90aHTV[a$\x1A\x90` \x81\x01\x90aG\x16V[\x90P\x81\x10\x15a%qWa$0`\xC0\x84\x01\x84aG\x16V[\x83\x81\x81\x10a$@Wa$@a>gV[\x90P` \x02\x81\x01\x90a$R\x91\x90aHTV[a$`\x90`@\x81\x01\x90aG\x16V[\x82\x81\x81\x10a$pWa$pa>gV[\x90P` \x02\x01` \x81\x01\x90a$\x85\x91\x90aG\xA8V[`\x9F`\0a$\x96`\xC0\x87\x01\x87aG\x16V[\x86\x81\x81\x10a$\xA6Wa$\xA6a>gV[\x90P` \x02\x81\x01\x90a$\xB8\x91\x90aHTV[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a$\xD9`\xC0\x87\x01\x87aG\x16V[\x86\x81\x81\x10a$\xE9Wa$\xE9a>gV[\x90P` \x02\x81\x01\x90a$\xFB\x91\x90aHTV[a%\t\x90` \x81\x01\x90aG\x16V[\x85\x81\x81\x10a%\x19Wa%\x19a>gV[\x90P` \x02\x01` \x81\x01\x90a%.\x91\x90a7\x8CV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a%i\x81aF\xFBV[\x91PPa#\xDDV[P\x80a%|\x81aF\xFBV[\x91PPa#'V[P`\0[a%\x95`\xE0\x83\x01\x83aG\x16V[\x90P\x81\x10\x15a'HW`\0[a%\xAE`\xE0\x84\x01\x84aG\x16V[\x83\x81\x81\x10a%\xBEWa%\xBEa>gV[\x90P` \x02\x81\x01\x90a%\xD0\x91\x90aHtV[a%\xDE\x90` \x81\x01\x90aG\x16V[\x90P\x81\x10\x15a'5Wa%\xF4`\xE0\x84\x01\x84aG\x16V[\x83\x81\x81\x10a&\x04Wa&\x04a>gV[\x90P` \x02\x81\x01\x90a&\x16\x91\x90aHtV[a&$\x90`@\x81\x01\x90aG\x16V[\x82\x81\x81\x10a&4Wa&4a>gV[\x90P` \x02\x01` \x81\x01\x90a&I\x91\x90aG\xA8V[`\x9F`\0a&Z`\xE0\x87\x01\x87aG\x16V[\x86\x81\x81\x10a&jWa&ja>gV[\x90P` \x02\x81\x01\x90a&|\x91\x90aHtV[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a&\x9D`\xE0\x87\x01\x87aG\x16V[\x86\x81\x81\x10a&\xADWa&\xADa>gV[\x90P` \x02\x81\x01\x90a&\xBF\x91\x90aHtV[a&\xCD\x90` \x81\x01\x90aG\x16V[\x85\x81\x81\x10a&\xDDWa&\xDDa>gV[\x90P` \x02\x01` \x81\x01\x90a&\xF2\x91\x90a7\x8CV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a'-\x81aF\xFBV[\x91PPa%\xA1V[P\x80a'@\x81aF\xFBV[\x91PPa%\x88V[P`\0[a'Za\x01\0\x83\x01\x83aG\xC3V[\x90P\x81\x10\x15a'\xFEWa'qa\x01\0\x83\x01\x83aG\xC3V[\x82\x81\x81\x10a'\x81Wa'\x81a>gV[\x90P`@\x02\x01` \x01` \x81\x01\x90a'\x99\x91\x90a7\x8CV[`\xA1`\0a'\xABa\x01\0\x86\x01\x86aG\xC3V[\x85\x81\x81\x10a'\xBBWa'\xBBa>gV[\x90P`@\x02\x01`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP\x80\x80a'\xF6\x90aF\xFBV[\x91PPa'LV[P`\xA0\x82\x015`\x9AUa(\x14` \x84\x01\x84a>\xE8V[`\x9B\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua(>``\x84\x01`@\x85\x01a>\xE8V[`\x9B\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90UB`\x99Ua(v`\x80\x84\x01\x84aH\x8AV[a(\x82\x91`\x9C\x91a6\x08V[Pa(\x93`\xC0\x84\x01`\xA0\x85\x01a>\xE8V[`\x9D\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x7F&Ie\xEBk\xC46\xC6\xC4sC\x1D4\xAFV\xE82\xEC4O\xDF\xD4>\xE6\xAFo\xCEm ^\x84\xAFa(\xDB` \x85\x01\x85a>\xE8V[a(\xEB``\x86\x01`@\x87\x01a>\xE8V[`@\x80Qc\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1PPPV[a)\x18a-*V[`\x01`\x01`\xA0\x1B\x03\x81\x16a)}W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x05\xABV[a\x05\xBD\x81a2#V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a)\xA6WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a)\xC0WP0;\x15\x80\x15a)\xC0WP`\0T`\xFF\x16`\x01\x14[a*#W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x05\xABV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a*FW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a*Q\x85`\0a3\xF4V[a*Z\x84a2#V[`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U`\x98\x80T\x92\x85\x16\x92\x90\x91\x16\x91\x90\x91\x17\x90U\x80\x15a*\xD0W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+*W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+N\x91\x90a=\x8DV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a+~W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xAB\x90a=\xAAV[`fT\x19\x81\x19`fT\x19\x16\x14a+\xFCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\xABV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x07\x1EV[`\x01`\x01`\xA0\x1B\x03\x81\x16a,\xC1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x05\xABV[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x18]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x05\xABV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra-\xA0a6\x8CV[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x80\x82\x01\x84\x90R`\0\x90\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a-\xD3Wa-\xD5V[\xFE[P\x80a.\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCB[][\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x05\xABV[PP\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra.7a6\xAAV[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R\x83Q`@\x80\x84\x01\x91\x90\x91R\x90\x84\x01Q``\x83\x01R`\0\x90\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a-\xD3WP\x80a.\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCBXY\x19\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x05\xABV[a.\xB7a6\xC8V[P`@\x80Q`\x80\x81\x01\x82R\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81\x83\x01\x90\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED``\x83\x01R\x81R\x81Q\x80\x83\x01\x90\x92R\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x81\x01\x91\x90\x91R\x81\x01\x91\x90\x91R\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0\x80\x80a/\x9F`\0\x80Q` aI\x10\x839\x81Q\x91R\x86a>}V[\x90P[a/\xAB\x81a4\xDEV[\x90\x93P\x91P`\0\x80Q` aI\x10\x839\x81Q\x91R\x82\x83\t\x83\x14\x15a/\xE5W`@\x80Q\x80\x82\x01\x90\x91R\x90\x81R` \x81\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80Q` aI\x10\x839\x81Q\x91R`\x01\x82\x08\x90Pa/\xA2V[`@\x80Q\x80\x82\x01\x82R\x86\x81R` \x80\x82\x01\x86\x90R\x82Q\x80\x84\x01\x90\x93R\x86\x83R\x82\x01\x84\x90R`\0\x91\x82\x91\x90a01a6\xEDV[`\0[`\x02\x81\x10\x15a1\xF6W`\0a0J\x82`\x06aF\xDCV[\x90P\x84\x82`\x02\x81\x10a0^Wa0^a>gV[` \x02\x01QQ\x83a0p\x83`\0aF\x89V[`\x0C\x81\x10a0\x80Wa0\x80a>gV[` \x02\x01R\x84\x82`\x02\x81\x10a0\x97Wa0\x97a>gV[` \x02\x01Q` \x01Q\x83\x82`\x01a0\xAE\x91\x90aF\x89V[`\x0C\x81\x10a0\xBEWa0\xBEa>gV[` \x02\x01R\x83\x82`\x02\x81\x10a0\xD5Wa0\xD5a>gV[` \x02\x01QQQ\x83a0\xE8\x83`\x02aF\x89V[`\x0C\x81\x10a0\xF8Wa0\xF8a>gV[` \x02\x01R\x83\x82`\x02\x81\x10a1\x0FWa1\x0Fa>gV[` \x02\x01QQ`\x01` \x02\x01Q\x83a1(\x83`\x03aF\x89V[`\x0C\x81\x10a18Wa18a>gV[` \x02\x01R\x83\x82`\x02\x81\x10a1OWa1Oa>gV[` \x02\x01Q` \x01Q`\0`\x02\x81\x10a1jWa1ja>gV[` \x02\x01Q\x83a1{\x83`\x04aF\x89V[`\x0C\x81\x10a1\x8BWa1\x8Ba>gV[` \x02\x01R\x83\x82`\x02\x81\x10a1\xA2Wa1\xA2a>gV[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a1\xBDWa1\xBDa>gV[` \x02\x01Q\x83a1\xCE\x83`\x05aF\x89V[`\x0C\x81\x10a1\xDEWa1\xDEa>gV[` \x02\x01RP\x80a1\xEE\x81aF\xFBV[\x91PPa04V[Pa1\xFFa7\x0CV[`\0` \x82a\x01\x80\x85`\x08\x8C\xFA\x91Q\x91\x9C\x91\x15\x15\x9BP\x90\x99PPPPPPPPPPV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x02\0\x82a\xFF\xFF\x16\x10a2\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Roscalar-too-large`\x80\x1B`D\x82\x01R`d\x01a\x05\xABV[\x81a\xFF\xFF\x16`\x01\x14\x15a2\xE5WP\x81a\x1EFV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01\x81\x90R\x84\x90`\x01\x90[\x81a\xFF\xFF\x16\x86a\xFF\xFF\x16\x10a3NW`\x01a\xFF\xFF\x87\x16`\xFF\x83\x16\x1C\x81\x16\x14\x15a31Wa3.\x84\x84a.\x1BV[\x93P[a3;\x83\x84a.\x1BV[\x92Pb\x01\xFF\xFE`\x01\x92\x83\x1B\x16\x91\x01a3\x01V[P\x91\x95\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q\x15\x80\x15a3~WP` \x82\x01Q\x15[\x15a3\x9CWPP`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01`\0\x80Q` aI\x10\x839\x81Q\x91R\x84` \x01Qa3\xCF\x91\x90a>}V[a3\xE7\x90`\0\x80Q` aI\x10\x839\x81Q\x91RaH\xD0V[\x90R\x92\x91PPV[\x91\x90PV[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a4\x15WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a4\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x05\xABV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a4\xDA\x82a,3V[PPV[`\0\x80\x80`\0\x80Q` aI\x10\x839\x81Q\x91R`\x03`\0\x80Q` aI\x10\x839\x81Q\x91R\x86`\0\x80Q` aI\x10\x839\x81Q\x91R\x88\x89\t\t\x08\x90P`\0a5T\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\0\x80Q` aI\x10\x839\x81Q\x91Ra5`V[\x91\x95\x91\x94P\x90\x92PPPV[`\0\x80a5ka7\x0CV[a5sa7*V[` \x80\x82R\x81\x81\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x88\x90R`\x80\x82\x01\x87\x90R`\xA0\x82\x01\x86\x90R\x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82\x80\x15a-\xD3WP\x82a5\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FBN254.expMod: call failure\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\xABV[PQ\x95\x94PPPPPV[\x82\x80Ta6\x14\x90a>\x9FV[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82a66W`\0\x85Ua6|V[\x82`\x1F\x10a6OW\x82\x80\x01`\xFF\x19\x825\x16\x17\x85Ua6|V[\x82\x80\x01`\x01\x01\x85U\x82\x15a6|W\x91\x82\x01[\x82\x81\x11\x15a6|W\x825\x82U\x91` \x01\x91\x90`\x01\x01\x90a6aV[Pa6\x88\x92\x91Pa7HV[P\x90V[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80a6\xDBa7]V[\x81R` \x01a6\xE8a7]V[\x90R\x90V[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[[\x80\x82\x11\x15a6\x88W`\0\x81U`\x01\x01a7IV[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[\x805`\xFF\x81\x16\x81\x14a3\xEFW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a7\x9EW`\0\x80\xFD[a7\xA7\x82a7{V[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05\xBDW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a7\xD5W`\0\x80\xFD[\x815a7\xA7\x81a7\xAEV[`\0` \x82\x84\x03\x12\x15a7\xF2W`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a81Wa81a7\xF9V[`@R\x90V[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a81Wa81a7\xF9V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a8\x81Wa8\x81a7\xF9V[`@R\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a8\x9BW`\0\x80\xFD[a8\xA3a8\x0FV[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a8\xCAW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a8\xECWa8\xECa7\xF9V[\x80`@RP\x80`@\x84\x01\x85\x81\x11\x15a9\x03W`\0\x80\xFD[\x84[\x81\x81\x10\x15a3NW\x805\x83R` \x92\x83\x01\x92\x01a9\x05V[`\0`\x80\x82\x84\x03\x12\x15a9/W`\0\x80\xFD[a97a8\x0FV[\x90Pa9C\x83\x83a8\xB9V[\x81Ra9R\x83`@\x84\x01a8\xB9V[` \x82\x01R\x92\x91PPV[`\0\x80`\0\x80a\x01 \x85\x87\x03\x12\x15a9tW`\0\x80\xFD[\x845\x93Pa9\x85\x86` \x87\x01a8\x89V[\x92Pa9\x94\x86``\x87\x01a9\x1DV[\x91Pa9\xA3\x86`\xE0\x87\x01a8\x89V[\x90P\x92\x95\x91\x94P\x92PV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a9\xDBW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a9\xBFV[\x81\x81\x11\x15a9\xEDW`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[`\0a\x01\0\x82\x84\x03\x12\x15a:\x16W`\0\x80\xFD[P\x91\x90PV[`\0`\xC0\x82\x84\x03\x12\x15a:\x16W`\0\x80\xFD[`\0a\x01 \x82\x84\x03\x12\x15a:\x16W`\0\x80\xFD[`\0\x80`\0\x80a\x01 \x85\x87\x03\x12\x15a:XW`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a:oW`\0\x80\xFD[a:{\x88\x83\x89\x01a:\x03V[\x95Pa:\x8A\x88` \x89\x01a:\x1CV[\x94P`\xE0\x87\x015\x91P\x80\x82\x11\x15a:\xA0W`\0\x80\xFD[\x90\x86\x01\x90`\xE0\x82\x89\x03\x12\x15a:\xB4W`\0\x80\xFD[\x90\x92Pa\x01\0\x86\x015\x90\x80\x82\x11\x15a:\xCBW`\0\x80\xFD[Pa:\xD8\x87\x82\x88\x01a:.V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a:\xF7W`\0\x80\xFD[\x825\x91Pa;\x07` \x84\x01a7{V[\x90P\x92P\x92\x90PV[`\0`\xE0\x82\x84\x03\x12\x15a;\"W`\0\x80\xFD[a;*a87V[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a;CW`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a;WW`\0\x80\xFD[\x815` \x82\x82\x11\x15a;kWa;ka7\xF9V[a;y\x81\x83`\x05\x1B\x01a8YV[\x82\x81R\x81\x81\x01\x93P`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x87\x83\x11\x15a;\x99W`\0\x80\xFD[\x93\x81\x01\x93[\x82\x85\x10\x15a;\xC2Wa;\xB0\x88\x86a8\x89V[\x84R\x81\x84\x01\x93P`@\x85\x01\x94Pa;\x9EV[\x85Ra;\xD0\x87\x87\x83\x01a9\x1DV[\x81\x86\x01RPPPPa;\xE5\x83`\xA0\x84\x01a8\x89V[`@\x82\x01R\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a<\x03W`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a< W`\0\x80\xFD[a<,\x85\x82\x86\x01a;\x10V[\x91PP\x92P\x92\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a<oW\x81Q`\x01`\x01``\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a<JV[P\x94\x95\x94PPPPPV[` \x81R`\0\x82Q`@` \x84\x01Ra<\x96``\x84\x01\x82a<6V[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01Ra<\xB3\x82\x82a<6V[\x95\x94PPPPPV[`\0\x80`\0a\x01\0\x84\x86\x03\x12\x15a<\xD2W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a<\xE9W`\0\x80\xFD[a<\xF5\x87\x83\x88\x01a:\x03V[\x94Pa=\x04\x87` \x88\x01a:\x1CV[\x93P`\xE0\x86\x015\x91P\x80\x82\x11\x15a=\x1AW`\0\x80\xFD[Pa='\x86\x82\x87\x01a:.V[\x91PP\x92P\x92P\x92V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a=GW`\0\x80\xFD[\x845a=R\x81a7\xAEV[\x93P` \x85\x015a=b\x81a7\xAEV[\x92P`@\x85\x015a=r\x81a7\xAEV[\x91P``\x85\x015a=\x82\x81a7\xAEV[\x93\x96\x92\x95P\x90\x93PPV[`\0` \x82\x84\x03\x12\x15a=\x9FW`\0\x80\xFD[\x81Qa7\xA7\x81a7\xAEV[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[\x80\x15\x15\x81\x14a\x05\xBDW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a>\x14W`\0\x80\xFD[\x81Qa7\xA7\x81a=\xF4V[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x82a>\x9AWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80a>\xB3W`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15a:\x16WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a3\xEFW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a>\xFAW`\0\x80\xFD[a7\xA7\x82a>\xD4V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a?\x1AW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a?9W`\0\x80\xFD[\x806\x03\x83\x13\x15a?HW`\0\x80\xFD[\x92P\x92\x90PV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x81Rc\xFF\xFF\xFF\xFFa?\x8A\x83a>\xD4V[\x16` \x82\x01R` \x82\x015`@\x82\x01R`\0a?\xA8`@\x84\x01a>\xD4V[c\xFF\xFF\xFF\xFF\x81\x16``\x84\x01RPa?\xC1``\x84\x01a>\xD4V[c\xFF\xFF\xFF\xFF\x81\x16`\x80\x84\x01RPa?\xDB`\x80\x84\x01\x84a?\x03V[a\x01\0\x80`\xA0\x86\x01Ra?\xF3a\x01 \x86\x01\x83\x85a?OV[\x92Pa@\x01`\xA0\x87\x01a>\xD4V[c\xFF\xFF\xFF\xFF\x81\x16`\xC0\x87\x01R\x91Pa@\x1C`\xC0\x87\x01\x87a?\x03V[\x86\x85\x03`\x1F\x19\x01`\xE0\x88\x01R\x92Pa@5\x84\x84\x83a?OV[\x93PPa@D`\xE0\x87\x01a>\xD4V[c\xFF\xFF\xFF\xFF\x16\x94\x01\x93\x90\x93R\x93\x92PPPV[\x805a3\xEF\x81a=\xF4V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a@yW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a@\x98W`\0\x80\xFD[\x80`\x05\x1B6\x03\x83\x13\x15a?HW`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a<oW`\xFFa@\xCD\x83a7{V[\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a@\xBAV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a@\xF7W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aA\x16W`\0\x80\xFD[\x80`\x07\x1B6\x03\x83\x13\x15a?HW`\0\x80\xFD[\x805`\x01`\x01``\x1B\x03\x81\x16\x81\x14a3\xEFW`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a<oW`\xFFaAb\x83a7{V[\x16\x87R`\x01`\x01``\x1B\x03aAx\x84\x84\x01aA(V[\x16\x83\x88\x01R`@\x82\x81\x015\x90\x88\x01R``\x80\x83\x015\x90\x88\x01R`\x80\x96\x87\x01\x96\x90\x91\x01\x90`\x01\x01aAOV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aA\xBAW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aA\xD9W`\0\x80\xFD[\x80`\x06\x1B6\x03\x83\x13\x15a?HW`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a<oW`\xFFaB\x0E\x83a7{V[\x16\x87R`\x01`\x01``\x1B\x03aB$\x84\x84\x01aA(V[\x16\x87\x84\x01R`@\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aA\xFBV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aBSW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aBrW`\0\x80\xFD[``\x81\x026\x03\x83\x13\x15a?HW`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a<oW`\xFFaB\xA7\x83a7{V[\x16\x87RaB\xC2\x83\x88\x01\x84\x84\x01\x805\x82R` \x90\x81\x015\x91\x01RV[``\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aB\x94V[\x81\x83R`\0`\x01`\x01`\xFB\x1B\x03\x83\x11\x15aB\xEEW`\0\x80\xFD[\x82`\x05\x1B\x80\x83` \x87\x017`\0\x94\x01` \x01\x93\x84RP\x91\x92\x91PPV[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a<oW`\x01`\x01``\x1B\x03aC4\x83aA(V[\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aC\x1BV[\x81\x83R`\0` \x80\x85\x01\x80\x81\x96P\x85`\x05\x1B\x81\x01\x91P\x84`\0\x80[\x88\x81\x10\x15aC\xF8W\x83\x85\x03\x8AR\x825`~\x19\x896\x03\x01\x81\x12aC\x82W\x82\x83\xFD[\x88\x01\x805\x86R`\x80aC\x96\x88\x83\x01\x83a@bV[\x82\x8A\x8A\x01RaC\xA8\x83\x8A\x01\x82\x84a@\xAAV[\x92PPP`@aC\xBA\x81\x84\x01\x84a@bV[\x89\x84\x03\x83\x8B\x01RaC\xCC\x84\x82\x84aC\x0BV[\x93PPPP```\xFFaC\xE0\x82\x85\x01a7{V[\x16\x97\x01\x96\x90\x96RP\x98\x85\x01\x98\x91\x85\x01\x91`\x01\x01aCbV[P\x92\x98\x97PPPPPPPPV[\x81\x83R`\0` \x80\x85\x01\x80\x81\x96P\x85`\x05\x1B\x81\x01\x91P\x84`\0\x80[\x88\x81\x10\x15aC\xF8W\x83\x85\x03\x8AR\x825`^\x19\x896\x03\x01\x81\x12aDAW\x82\x83\xFD[\x88\x01\x805\x86R``aDU\x88\x83\x01\x83a@bV[\x82\x8A\x8A\x01RaDg\x83\x8A\x01\x82\x84a@\xAAV[\x92PPP`@aDy\x81\x84\x01\x84a@bV[\x93P\x88\x83\x03\x82\x8A\x01RaD\x8D\x83\x85\x83aC\x0BV[\x9D\x8A\x01\x9D\x98PPP\x93\x87\x01\x93PP`\x01\x01aD!V[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a<oW\x815\x87R`\xFFaD\xCC\x84\x84\x01a7{V[\x16\x87\x84\x01R`@\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aD\xB3V[` \x81RaD\xFE` \x82\x01aD\xF8\x84a@WV[\x15\x15\x90RV[`\0aE\r` \x84\x01\x84a@bV[a\x01 \x80`@\x86\x01RaE%a\x01@\x86\x01\x83\x85a@\xAAV[\x92PaE4`@\x87\x01\x87a@\xE0V[\x92P`\x1F\x19\x80\x87\x86\x03\x01``\x88\x01RaEN\x85\x85\x84aA?V[\x94PaE]``\x89\x01\x89aA\xA3V[\x94P\x91P\x80\x87\x86\x03\x01`\x80\x88\x01RaEv\x85\x85\x84aA\xEBV[\x94PaE\x85`\x80\x89\x01\x89aB<V[\x94P\x91P\x80\x87\x86\x03\x01`\xA0\x88\x01RaE\x9E\x85\x85\x84aB\x84V[\x94PaE\xAD`\xA0\x89\x01\x89a@bV[\x94P\x91P\x80\x87\x86\x03\x01`\xC0\x88\x01RaE\xC6\x85\x85\x84aB\xD5V[\x94PaE\xD5`\xC0\x89\x01\x89a@bV[\x94P\x91P\x80\x87\x86\x03\x01`\xE0\x88\x01RaE\xEE\x85\x85\x84aCGV[\x94PaE\xFD`\xE0\x89\x01\x89a@bV[\x94P\x91Pa\x01\0\x81\x88\x87\x03\x01\x81\x89\x01RaF\x18\x86\x86\x85aD\x06V[\x95PaF&\x81\x8A\x01\x8AaA\xA3V[\x95P\x92PP\x80\x87\x86\x03\x01\x83\x88\x01RPaF@\x84\x84\x83aD\xA3V[\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aF\x80WaF\x80aFKV[\x01\x94\x93PPPPV[`\0\x82\x19\x82\x11\x15aF\x9CWaF\x9CaFKV[P\x01\x90V[`\0a\x1EF6\x83a;\x10V[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aF\xD3WaF\xD3aFKV[\x02\x94\x93PPPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15aF\xF6WaF\xF6aFKV[P\x02\x90V[`\0`\0\x19\x82\x14\x15aG\x0FWaG\x0FaFKV[P`\x01\x01\x90V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aG-W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aGGW`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a?HW`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aGvW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aG\x90W`\0\x80\xFD[` \x01\x91P`\x07\x81\x90\x1B6\x03\x82\x13\x15a?HW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aG\xBAW`\0\x80\xFD[a7\xA7\x82aA(V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aG\xDAW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aG\xF4W`\0\x80\xFD[` \x01\x91P`\x06\x81\x90\x1B6\x03\x82\x13\x15a?HW`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aH#W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aH=W`\0\x80\xFD[` \x01\x91P``\x81\x026\x03\x82\x13\x15a?HW`\0\x80\xFD[`\0\x825`~\x19\x836\x03\x01\x81\x12aHjW`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x825`^\x19\x836\x03\x01\x81\x12aHjW`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aH\xA1W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aH\xBBW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a?HW`\0\x80\xFD[`\0\x82\x82\x10\x15aH\xE2WaH\xE2aFKV[P\x03\x90V[`\0`\x01`\x01``\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15aI\x07WaI\x07aFKV[\x03\x93\x92PPPV\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 \xD9>\xE2_`\xBE\xC0C\x85.\x81\x88Y_\x02.M\x13\xCEP\xECu\x13s\xA7\x06\xA4\x19]\x08\xC6odsolcC\0\x08\x0C\x003";
    /// The bytecode of the contract.
    pub static GASPMULTIROLLUPSERVICE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xDAW`\x005`\xE0\x1C\x80cZ\xC8j\xB7\x11a\x01\x04W\x80c\xC9'\xFE\xEF\x11a\0\xA2W\x80c\xF2\xFD\xE3\x8B\x11a\0qW\x80c\xF2\xFD\xE3\x8B\x14a\x04\xBBW\x80c\xF8\xC8v^\x14a\x04\xCEW\x80c\xFA\xBC\x1C\xBC\x14a\x04\xE1W\x80c\xFCv]\xD5\x14a\x04\xF4W`\0\x80\xFD[\x80c\xC9'\xFE\xEF\x14a\x04tW\x80c\xDF\x03L\xD0\x14a\x04\x87W\x80c\xE6\x1D\xB1u\x14a\x04\x9AW\x80c\xEDZ\x04\xFE\x14a\x04\xA3W`\0\x80\xFD[\x80cz\xD7Ua\x11a\0\xDEW\x80cz\xD7Ua\x14a\x04\x07W\x80c\x88o\x11\x95\x14a\x040W\x80c\x8D\xA5\xCB[\x14a\x04CW\x80c\xC4\xE1\x91L\x14a\x04TW`\0\x80\xFD[\x80cZ\xC8j\xB7\x14a\x03\xD4W\x80c\\\x97Z\xBB\x14a\x03\xF7W\x80cqP\x18\xA6\x14a\x03\xFFW`\0\x80\xFD[\x80c=\x9F\xB0\x0C\x11a\x01|W\x80cJ\xE6\xB2\x03\x11a\x01KW\x80cJ\xE6\xB2\x03\x14a\x03lW\x80cM\xEA\xBC!\x14a\x03\x83W\x80cRn>d\x14a\x03\xA8W\x80cY\\jg\x14a\x03\xCCW`\0\x80\xFD[\x80c=\x9F\xB0\x0C\x14a\x02\xADW\x80c>@I&\x14a\x02\xD8W\x80cC\r;9\x14a\x02\xEBW\x80cI\x9Do\xB6\x14a\x03 W`\0\x80\xFD[\x80c\x13d9\xDD\x11a\x01\xB8W\x80c\x13d9\xDD\x14a\x02HW\x80c\x17\x1F\x1D[\x14a\x02[W\x80c\x1E-K\xF7\x14a\x02\x85W\x80c*\x84\x14\xFD\x14a\x02\x98W`\0\x80\xFD[\x80c\x03\xD0\x97\xD2\x14a\x01\xDFW\x80c\x10\xD6z/\x14a\x02 W\x80c\x12FH\xC9\x14a\x025W[`\0\x80\xFD[a\x02\x06a\x01\xED6`\x04a7\x8CV[`\xA0` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T\x82V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[a\x023a\x02.6`\x04a7\xC3V[a\x05\x04V[\0[a\x023a\x02C6`\x04a7\xC3V[a\x05\xC0V[a\x023a\x02V6`\x04a7\xE0V[a\x05\xEAV[a\x02na\x02i6`\x04a9]V[a\x07)V[`@\x80Q\x92\x15\x15\x83R\x90\x15\x15` \x83\x01R\x01a\x02\x17V[a\x023a\x02\x936`\x04a7\xC3V[a\x08\xB3V[a\x02\xA0a\x08\xDDV[`@Qa\x02\x17\x91\x90a9\xAEV[`\x98Ta\x02\xC0\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x17V[a\x023a\x02\xE66`\x04a:AV[a\tkV[a\x03\x0Ea\x02\xF96`\x04a7\xE0V[`\xA1` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02\x17V[a\x03Ta\x03.6`\x04a:\xE4V[`\x9F` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\x01`\x01``\x1B\x03\x16\x81V[`@Q`\x01`\x01``\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x17V[a\x03u`\x9AT\x81V[`@Q\x90\x81R` \x01a\x02\x17V[`\x9DTa\x03\x93\x90c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\x17V[`\x98Ta\x03\xBC\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02\x17V[a\x023a\x17\x84V[a\x03\xBCa\x03\xE26`\x04a7\x8CV[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`fTa\x03uV[a\x023a\x18KV[a\x03Ta\x04\x156`\x04a7\x8CV[`\x9E` R`\0\x90\x81R`@\x90 T`\x01`\x01``\x1B\x03\x16\x81V[`eTa\x02\xC0\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x02\xC0V[a\x04ga\x04b6`\x04a;\xF0V[a\x18_V[`@Qa\x02\x17\x91\x90a<zV[a\x023a\x04\x826`\x04a<\xBCV[a\x1ELV[`\x97Ta\x02\xC0\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03u`\x99T\x81V[`\x9BTa\x03\x93\x90d\x01\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x023a\x04\xC96`\x04a7\xC3V[a)\x10V[a\x023a\x04\xDC6`\x04a=1V[a)\x86V[a\x023a\x04\xEF6`\x04a7\xE0V[a*\xD7V[`\x9BTa\x03\x93\x90c\xFF\xFF\xFF\xFF\x16\x81V[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05WW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05{\x91\x90a=\x8DV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x05\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xAB\x90a=\xAAV[`@Q\x80\x91\x03\x90\xFD[a\x05\xBD\x81a,3V[PV[a\x05\xC8a-*V[`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x062W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06V\x91\x90a>\x02V[a\x06rW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xAB\x90a>\x1FV[`fT\x81\x81\x16\x14a\x06\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\xABV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`\0\x80`\0\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x87\x87`\0\x01Q\x88` \x01Q\x88`\0\x01Q`\0`\x02\x81\x10a\x07qWa\x07qa>gV[` \x02\x01Q\x89Q`\x01` \x02\x01Q\x8A` \x01Q`\0`\x02\x81\x10a\x07\x96Wa\x07\x96a>gV[` \x02\x01Q\x8B` \x01Q`\x01`\x02\x81\x10a\x07\xB2Wa\x07\xB2a>gV[` \x90\x81\x02\x91\x90\x91\x01Q\x8CQ\x8D\x83\x01Q`@Qa\x08\x0F\x9A\x99\x98\x97\x96\x95\x94\x01\x98\x89R` \x89\x01\x97\x90\x97R`@\x88\x01\x95\x90\x95R``\x87\x01\x93\x90\x93R`\x80\x86\x01\x91\x90\x91R`\xA0\x85\x01R`\xC0\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01Ra\x01 \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\0\x1Ca\x082\x91\x90a>}V[\x90Pa\x08\xA5a\x08Ka\x08D\x88\x84a-\x84V[\x86\x90a.\x1BV[a\x08Sa.\xAFV[a\x08\x9Ba\x08\x8C\x85a\x08\x86`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x01\x82R`\x02\x90\x82\x01R\x90V[\x90a-\x84V[a\x08\x95\x8Ca/oV[\x90a.\x1BV[\x88b\x01\xD4\xC0a/\xFFV[\x90\x98\x90\x97P\x95PPPPPPV[a\x08\xBBa-*V[`\x98\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x9C\x80Ta\x08\xEA\x90a>\x9FV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\t\x16\x90a>\x9FV[\x80\x15a\tcW\x80`\x1F\x10a\t8Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\tcV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\tFW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\x97T`\x01`\x01`\xA0\x1B\x03\x163\x14a\t\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FUpdater must be the caller\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\xABV[a\t\xD5`\x80\x85\x01``\x86\x01a>\xE8V[`\x9BTd\x01\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14a\n7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7Freference block hash mismatch\0\0\0`D\x82\x01R`d\x01a\x05\xABV[\x83`@Q` \x01a\nH\x91\x90a?xV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x83` \x015\x14a\n\xB0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FreferenceTaskHash hash mismatch\0`D\x82\x01R`d\x01a\x05\xABV[\x80`@Q` \x01a\n\xC1\x91\x90aD\xE4V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x83`@\x015\x14a\x0B)W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FoperatorStateInfo hash mismatch\0`D\x82\x01R`d\x01a\x05\xABV[a\x0B9``\x85\x01`@\x86\x01a>\xE8V[`\x9BTc\xFF\xFF\xFF\xFF\x91\x82\x16\x91a\x0BZ\x91d\x01\0\0\0\0\x90\x04\x16a8@aFaV[c\xFF\xFF\xFF\xFF\x16\x11a\x0B\x9DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x077F\x16\xC6R\x077F\x17FR\x03`\x9C\x1B`D\x82\x01R`d\x01a\x05\xABV[B`\x99Tb\x03\xF4\x80a\x0B\xAF\x91\x90aF\x89V[\x11a\x0B\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rlstale state 1`\x98\x1B`D\x82\x01R`d\x01a\x05\xABV[`\x9BTd\x01\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x15a\x0C\xC6W`\0a\x0C\x15` \x85\x015a\x04b\x85aF\xA1V[`\x9DT\x90\x91Pc\xFF\xFF\xFF\xFF\x16`\0[`\x9C\x80Ta\x0C1\x90a>\x9FV[\x90P\x81\x10\x15a\x0C\xC2W\x81`\xFF\x16\x83` \x01Q\x82\x81Q\x81\x10a\x0CTWa\x0CTa>gV[` \x02` \x01\x01Qa\x0Cf\x91\x90aF\xADV[`\x01`\x01``\x1B\x03\x16`d\x84`\0\x01Q\x83\x81Q\x81\x10a\x0C\x87Wa\x0C\x87a>gV[` \x02` \x01\x01Q`\x01`\x01``\x1B\x03\x16a\x0C\xA2\x91\x90aF\xDCV[\x10\x15a\x0C\xB0WPPPa\x17~V[\x80a\x0C\xBA\x81aF\xFBV[\x91PPa\x0C$V[PPP[`\0[a\x0C\xD6` \x83\x01\x83aG\x16V[\x90P\x81\x10\x15a\r\x9BW`\x9E`\0a\x0C\xF0` \x85\x01\x85aG\x16V[\x84\x81\x81\x10a\r\0Wa\r\0a>gV[\x90P` \x02\x01` \x81\x01\x90a\r\x15\x91\x90a7\x8CV[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0\x90\x81 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U`\xA0\x91a\rH\x90\x85\x01\x85aG\x16V[\x84\x81\x81\x10a\rXWa\rXa>gV[\x90P` \x02\x01` \x81\x01\x90a\rm\x91\x90a7\x8CV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x81\x81U`\x01\x01U\x80a\r\x93\x81aF\xFBV[\x91PPa\x0C\xC9V[P`\0[a\r\xAC`@\x83\x01\x83aG_V[\x90P\x81\x10\x15a\x0E\xE7Wa\r\xC2`@\x83\x01\x83aG_V[\x82\x81\x81\x10a\r\xD2Wa\r\xD2a>gV[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\r\xEA\x91\x90aG\xA8V[`\x9E`\0a\r\xFB`@\x86\x01\x86aG_V[\x85\x81\x81\x10a\x0E\x0BWa\x0E\x0Ba>gV[a\x0E!\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa7\x8CV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x90\x81\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x17\x90\x91Ua\x0Ea\x90\x83\x01\x83aG_V[\x82\x81\x81\x10a\x0EqWa\x0Eqa>gV[\x90P`\x80\x02\x01`@\x01`\xA0`\0\x84\x80`@\x01\x90a\x0E\x8E\x91\x90aG_V[\x85\x81\x81\x10a\x0E\x9EWa\x0E\x9Ea>gV[a\x0E\xB4\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa7\x8CV[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a\x0E\xDF\x81aF\xFBV[\x91PPa\r\x9FV[P`\0[a\x0E\xF8``\x83\x01\x83aG\xC3V[\x90P\x81\x10\x15a\x0F\xB0Wa\x0F\x0E``\x83\x01\x83aG\xC3V[\x82\x81\x81\x10a\x0F\x1EWa\x0F\x1Ea>gV[\x90P`@\x02\x01` \x01` \x81\x01\x90a\x0F6\x91\x90aG\xA8V[`\x9E`\0a\x0FG``\x86\x01\x86aG\xC3V[\x85\x81\x81\x10a\x0FWWa\x0FWa>gV[a\x0Fm\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa7\x8CV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\x0F\xA8\x81aF\xFBV[\x91PPa\x0E\xEBV[P`\0[a\x0F\xC1`\x80\x83\x01\x83aH\x0CV[\x90P\x81\x10\x15a\x10]Wa\x0F\xD7`\x80\x83\x01\x83aH\x0CV[\x82\x81\x81\x10a\x0F\xE7Wa\x0F\xE7a>gV[\x90P``\x02\x01` \x01`\xA0`\0\x84\x80`\x80\x01\x90a\x10\x04\x91\x90aH\x0CV[\x85\x81\x81\x10a\x10\x14Wa\x10\x14a>gV[a\x10*\x92` ``\x90\x92\x02\x01\x90\x81\x01\x91Pa7\x8CV[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a\x10U\x81aF\xFBV[\x91PPa\x0F\xB4V[P`\0[a\x10n`\xA0\x83\x01\x83aG\x16V[\x90P\x81\x10\x15a\x11\x95W`\0[`\x9C\x80Ta\x10\x87\x90a>\x9FV[\x90P\x81\x10\x15a\x11@W`\x9F`\0a\x10\xA1`\xA0\x86\x01\x86aG\x16V[\x85\x81\x81\x10a\x10\xB1Wa\x10\xB1a>gV[\x90P` \x02\x015\x81R` \x01\x90\x81R` \x01`\0 `\0`\x9C\x83\x81Ta\x10\xD6\x90a>\x9FV[\x81\x10a\x10\xE4Wa\x10\xE4a>gV[\x81T`\x01\x16\x15a\x11\x03W\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U\x81a\x118\x81aF\xFBV[\x92PPa\x10zV[P`\xA1`\0a\x11R`\xA0\x85\x01\x85aG\x16V[\x84\x81\x81\x10a\x11bWa\x11ba>gV[` \x90\x81\x02\x92\x90\x92\x015\x83RP\x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\xFF\x19\x16\x90U\x80a\x11\x8D\x81aF\xFBV[\x91PPa\x10aV[P`\0[a\x11\xA6`\xC0\x83\x01\x83aG\x16V[\x90P\x81\x10\x15a\x13\xF6Wa\x11\xBC`\xC0\x83\x01\x83aG\x16V[\x82\x81\x81\x10a\x11\xCCWa\x11\xCCa>gV[\x90P` \x02\x81\x01\x90a\x11\xDE\x91\x90aHTV[a\x11\xEF\x90`\x80\x81\x01\x90``\x01a7\x8CV[`\xA1`\0a\x12\0`\xC0\x86\x01\x86aG\x16V[\x85\x81\x81\x10a\x12\x10Wa\x12\x10a>gV[\x90P` \x02\x81\x01\x90a\x12\"\x91\x90aHTV[`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP`\0[a\x12\\`\xC0\x84\x01\x84aG\x16V[\x83\x81\x81\x10a\x12lWa\x12la>gV[\x90P` \x02\x81\x01\x90a\x12~\x91\x90aHTV[a\x12\x8C\x90` \x81\x01\x90aG\x16V[\x90P\x81\x10\x15a\x13\xE3Wa\x12\xA2`\xC0\x84\x01\x84aG\x16V[\x83\x81\x81\x10a\x12\xB2Wa\x12\xB2a>gV[\x90P` \x02\x81\x01\x90a\x12\xC4\x91\x90aHTV[a\x12\xD2\x90`@\x81\x01\x90aG\x16V[\x82\x81\x81\x10a\x12\xE2Wa\x12\xE2a>gV[\x90P` \x02\x01` \x81\x01\x90a\x12\xF7\x91\x90aG\xA8V[`\x9F`\0a\x13\x08`\xC0\x87\x01\x87aG\x16V[\x86\x81\x81\x10a\x13\x18Wa\x13\x18a>gV[\x90P` \x02\x81\x01\x90a\x13*\x91\x90aHTV[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a\x13K`\xC0\x87\x01\x87aG\x16V[\x86\x81\x81\x10a\x13[Wa\x13[a>gV[\x90P` \x02\x81\x01\x90a\x13m\x91\x90aHTV[a\x13{\x90` \x81\x01\x90aG\x16V[\x85\x81\x81\x10a\x13\x8BWa\x13\x8Ba>gV[\x90P` \x02\x01` \x81\x01\x90a\x13\xA0\x91\x90a7\x8CV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\x13\xDB\x81aF\xFBV[\x91PPa\x12OV[P\x80a\x13\xEE\x81aF\xFBV[\x91PPa\x11\x99V[P`\0[a\x14\x07`\xE0\x83\x01\x83aG\x16V[\x90P\x81\x10\x15a\x15\xBAW`\0[a\x14 `\xE0\x84\x01\x84aG\x16V[\x83\x81\x81\x10a\x140Wa\x140a>gV[\x90P` \x02\x81\x01\x90a\x14B\x91\x90aHtV[a\x14P\x90` \x81\x01\x90aG\x16V[\x90P\x81\x10\x15a\x15\xA7Wa\x14f`\xE0\x84\x01\x84aG\x16V[\x83\x81\x81\x10a\x14vWa\x14va>gV[\x90P` \x02\x81\x01\x90a\x14\x88\x91\x90aHtV[a\x14\x96\x90`@\x81\x01\x90aG\x16V[\x82\x81\x81\x10a\x14\xA6Wa\x14\xA6a>gV[\x90P` \x02\x01` \x81\x01\x90a\x14\xBB\x91\x90aG\xA8V[`\x9F`\0a\x14\xCC`\xE0\x87\x01\x87aG\x16V[\x86\x81\x81\x10a\x14\xDCWa\x14\xDCa>gV[\x90P` \x02\x81\x01\x90a\x14\xEE\x91\x90aHtV[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a\x15\x0F`\xE0\x87\x01\x87aG\x16V[\x86\x81\x81\x10a\x15\x1FWa\x15\x1Fa>gV[\x90P` \x02\x81\x01\x90a\x151\x91\x90aHtV[a\x15?\x90` \x81\x01\x90aG\x16V[\x85\x81\x81\x10a\x15OWa\x15Oa>gV[\x90P` \x02\x01` \x81\x01\x90a\x15d\x91\x90a7\x8CV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\x15\x9F\x81aF\xFBV[\x91PPa\x14\x13V[P\x80a\x15\xB2\x81aF\xFBV[\x91PPa\x13\xFAV[P`\0[a\x15\xCCa\x01\0\x83\x01\x83aG\xC3V[\x90P\x81\x10\x15a\x16pWa\x15\xE3a\x01\0\x83\x01\x83aG\xC3V[\x82\x81\x81\x10a\x15\xF3Wa\x15\xF3a>gV[\x90P`@\x02\x01` \x01` \x81\x01\x90a\x16\x0B\x91\x90a7\x8CV[`\xA1`\0a\x16\x1Da\x01\0\x86\x01\x86aG\xC3V[\x85\x81\x81\x10a\x16-Wa\x16-a>gV[\x90P`@\x02\x01`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP\x80\x80a\x16h\x90aF\xFBV[\x91PPa\x15\xBEV[P`\xA0\x83\x015`\x9AUa\x16\x86` \x85\x01\x85a>\xE8V[`\x9B\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\x16\xB0``\x85\x01`@\x86\x01a>\xE8V[`\x9B\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90UB`\x99Ua\x16\xE8`\x80\x85\x01\x85aH\x8AV[a\x16\xF4\x91`\x9C\x91a6\x08V[Pa\x17\x05`\xC0\x85\x01`\xA0\x86\x01a>\xE8V[`\x9D\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x7F\x9A\x12\x8F\xE74\x7F\x1E\x11\xCA\"\xAA\x9D\xEBc.\xC9\xAB\xB0\x96\x08\xC1:\x99L`\xF7zV/F\x01qa\x17M` \x86\x01\x86a>\xE8V[a\x17]``\x87\x01`@\x88\x01a>\xE8V[`@\x80Qc\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1[PPPPV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\xCCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xF0\x91\x90a>\x02V[a\x18\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xAB\x90a>\x1FV[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[a\x18Sa-*V[a\x18]`\0a2#V[V[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01\x81\x90R`\x9C\x80Ta\x18\x96\x90a>\x9FV[\x90P\x90Pa\x18\xB7`@Q\x80`@\x01`@R\x80``\x81R` \x01``\x81RP\x90V[\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18\xCFWa\x18\xCFa7\xF9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x18\xF8W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P` \x82\x01R\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19\x16Wa\x19\x16a7\xF9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x19?W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x81R\x84QQ`\0\x90`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19_Wa\x19_a7\xF9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x19\x88W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80[\x87QQ\x81\x10\x15a\x1BCWa\x19\xD1\x88`\0\x01Q\x82\x81Q\x81\x10a\x19\xB2Wa\x19\xB2a>gV[` \x02` \x01\x01Q\x80Q`\0\x90\x81R` \x91\x82\x01Q\x90\x91R`@\x90 \x90V[\x83\x82\x81Q\x81\x10a\x19\xE3Wa\x19\xE3a>gV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80\x15a\x1A\xC1W\x82a\x1A\0`\x01\x83aH\xD0V[\x81Q\x81\x10a\x1A\x10Wa\x1A\x10a>gV[` \x02` \x01\x01Q`\0\x1C\x83\x82\x81Q\x81\x10a\x1A-Wa\x1A-a>gV[` \x02` \x01\x01Q`\0\x1C\x11a\x1A\xC1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`M`$\x82\x01R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: nonSignerG1PubkeysForOldSta`d\x82\x01Rl\x1D\x19H\x1B\x9B\xDD\x08\x1C\xDB\xDC\x9D\x19Y`\x9A\x1B`\x84\x82\x01R`\xA4\x01a\x05\xABV[a\x1B/a\x1B(`\xA1`\0\x86\x85\x81Q\x81\x10a\x1A\xDDWa\x1A\xDDa>gV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82R\x81\x01\x91\x90\x91R`@\x01`\0 T\x8AQ\x80Q`\xFF\x90\x92\x16\x91\x85\x90\x81\x10a\x1B\x12Wa\x1B\x12a>gV[` \x02` \x01\x01Qa2u\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x87\x90a.\x1BV[\x95P\x80a\x1B;\x81aF\xFBV[\x91PPa\x19\x8FV[Pa\x1BM\x85a3YV[\x94P`\0[\x84\x81\x10\x15a\x1D.W`\x9C\x81\x81Ta\x1Bh\x90a>\x9FV[\x81\x10a\x1BvWa\x1Bva>gV[\x81T`\x01\x16\x15a\x1B\x95W\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C`\0\x81\x81R`\xA0` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x01T\x90\x82\x01R\x90\x92Pa\x1B\xD6\x90\x87\x90a.\x1BV[`\xFF\x83\x16`\0\x90\x81R`\x9E` \x90\x81R`@\x90\x91 T\x90\x86\x01Q\x80Q\x92\x98P`\x01`\x01``\x1B\x03\x90\x91\x16\x91\x83\x90\x81\x10a\x1C\x11Wa\x1C\x11a>gV[`\x01`\x01``\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x82\x01R\x84\x01Q\x80Q\x82\x90\x81\x10a\x1C=Wa\x1C=a>gV[` \x02` \x01\x01Q\x84`\0\x01Q\x82\x81Q\x81\x10a\x1C[Wa\x1C[a>gV[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPP`\0[\x88QQ\x81\x10\x15a\x1D\x1BW`\x9F`\0\x85\x83\x81Q\x81\x10a\x1C\x9EWa\x1C\x9Ea>gV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82R\x81\x81\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 `\xFF\x87\x16\x82R\x90\x92R\x90 T\x85Q\x80Q`\x01`\x01``\x1B\x03\x90\x92\x16\x91\x84\x90\x81\x10a\x1C\xE9Wa\x1C\xE9a>gV[` \x02` \x01\x01\x81\x81Qa\x1C\xFD\x91\x90aH\xE7V[`\x01`\x01``\x1B\x03\x16\x90RP\x80a\x1D\x13\x81aF\xFBV[\x91PPa\x1C~V[P\x80a\x1D&\x81aF\xFBV[\x91PPa\x1BRV[P`\0\x80a\x1DF\x8A\x88\x8B` \x01Q\x8C`@\x01Qa\x07)V[\x91P\x91P\x81a\x1D\xC9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: pairing precompile call fai`d\x82\x01Rb\x1B\x19Y`\xEA\x1B`\x84\x82\x01R`\xA4\x01a\x05\xABV[\x80a\x1E<W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: signature is invalid\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\xABV[P\x92\x95PPPPPP[\x92\x91PPV[a\x1ETa-*V[`\0[a\x1Ed` \x83\x01\x83aG\x16V[\x90P\x81\x10\x15a\x1F)W`\x9E`\0a\x1E~` \x85\x01\x85aG\x16V[\x84\x81\x81\x10a\x1E\x8EWa\x1E\x8Ea>gV[\x90P` \x02\x01` \x81\x01\x90a\x1E\xA3\x91\x90a7\x8CV[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0\x90\x81 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U`\xA0\x91a\x1E\xD6\x90\x85\x01\x85aG\x16V[\x84\x81\x81\x10a\x1E\xE6Wa\x1E\xE6a>gV[\x90P` \x02\x01` \x81\x01\x90a\x1E\xFB\x91\x90a7\x8CV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x81\x81U`\x01\x01U\x80a\x1F!\x81aF\xFBV[\x91PPa\x1EWV[P`\0[a\x1F:`@\x83\x01\x83aG_V[\x90P\x81\x10\x15a uWa\x1FP`@\x83\x01\x83aG_V[\x82\x81\x81\x10a\x1F`Wa\x1F`a>gV[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\x1Fx\x91\x90aG\xA8V[`\x9E`\0a\x1F\x89`@\x86\x01\x86aG_V[\x85\x81\x81\x10a\x1F\x99Wa\x1F\x99a>gV[a\x1F\xAF\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa7\x8CV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x90\x81\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x17\x90\x91Ua\x1F\xEF\x90\x83\x01\x83aG_V[\x82\x81\x81\x10a\x1F\xFFWa\x1F\xFFa>gV[\x90P`\x80\x02\x01`@\x01`\xA0`\0\x84\x80`@\x01\x90a \x1C\x91\x90aG_V[\x85\x81\x81\x10a ,Wa ,a>gV[a B\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa7\x8CV[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a m\x81aF\xFBV[\x91PPa\x1F-V[P`\0[a \x86``\x83\x01\x83aG\xC3V[\x90P\x81\x10\x15a!>Wa \x9C``\x83\x01\x83aG\xC3V[\x82\x81\x81\x10a \xACWa \xACa>gV[\x90P`@\x02\x01` \x01` \x81\x01\x90a \xC4\x91\x90aG\xA8V[`\x9E`\0a \xD5``\x86\x01\x86aG\xC3V[\x85\x81\x81\x10a \xE5Wa \xE5a>gV[a \xFB\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa7\x8CV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a!6\x81aF\xFBV[\x91PPa yV[P`\0[a!O`\x80\x83\x01\x83aH\x0CV[\x90P\x81\x10\x15a!\xEBWa!e`\x80\x83\x01\x83aH\x0CV[\x82\x81\x81\x10a!uWa!ua>gV[\x90P``\x02\x01` \x01`\xA0`\0\x84\x80`\x80\x01\x90a!\x92\x91\x90aH\x0CV[\x85\x81\x81\x10a!\xA2Wa!\xA2a>gV[a!\xB8\x92` ``\x90\x92\x02\x01\x90\x81\x01\x91Pa7\x8CV[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a!\xE3\x81aF\xFBV[\x91PPa!BV[P`\0[a!\xFC`\xA0\x83\x01\x83aG\x16V[\x90P\x81\x10\x15a##W`\0[`\x9C\x80Ta\"\x15\x90a>\x9FV[\x90P\x81\x10\x15a\"\xCEW`\x9F`\0a\"/`\xA0\x86\x01\x86aG\x16V[\x85\x81\x81\x10a\"?Wa\"?a>gV[\x90P` \x02\x015\x81R` \x01\x90\x81R` \x01`\0 `\0`\x9C\x83\x81Ta\"d\x90a>\x9FV[\x81\x10a\"rWa\"ra>gV[\x81T`\x01\x16\x15a\"\x91W\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U\x81a\"\xC6\x81aF\xFBV[\x92PPa\"\x08V[P`\xA1`\0a\"\xE0`\xA0\x85\x01\x85aG\x16V[\x84\x81\x81\x10a\"\xF0Wa\"\xF0a>gV[` \x90\x81\x02\x92\x90\x92\x015\x83RP\x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\xFF\x19\x16\x90U\x80a#\x1B\x81aF\xFBV[\x91PPa!\xEFV[P`\0[a#4`\xC0\x83\x01\x83aG\x16V[\x90P\x81\x10\x15a%\x84Wa#J`\xC0\x83\x01\x83aG\x16V[\x82\x81\x81\x10a#ZWa#Za>gV[\x90P` \x02\x81\x01\x90a#l\x91\x90aHTV[a#}\x90`\x80\x81\x01\x90``\x01a7\x8CV[`\xA1`\0a#\x8E`\xC0\x86\x01\x86aG\x16V[\x85\x81\x81\x10a#\x9EWa#\x9Ea>gV[\x90P` \x02\x81\x01\x90a#\xB0\x91\x90aHTV[`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP`\0[a#\xEA`\xC0\x84\x01\x84aG\x16V[\x83\x81\x81\x10a#\xFAWa#\xFAa>gV[\x90P` \x02\x81\x01\x90a$\x0C\x91\x90aHTV[a$\x1A\x90` \x81\x01\x90aG\x16V[\x90P\x81\x10\x15a%qWa$0`\xC0\x84\x01\x84aG\x16V[\x83\x81\x81\x10a$@Wa$@a>gV[\x90P` \x02\x81\x01\x90a$R\x91\x90aHTV[a$`\x90`@\x81\x01\x90aG\x16V[\x82\x81\x81\x10a$pWa$pa>gV[\x90P` \x02\x01` \x81\x01\x90a$\x85\x91\x90aG\xA8V[`\x9F`\0a$\x96`\xC0\x87\x01\x87aG\x16V[\x86\x81\x81\x10a$\xA6Wa$\xA6a>gV[\x90P` \x02\x81\x01\x90a$\xB8\x91\x90aHTV[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a$\xD9`\xC0\x87\x01\x87aG\x16V[\x86\x81\x81\x10a$\xE9Wa$\xE9a>gV[\x90P` \x02\x81\x01\x90a$\xFB\x91\x90aHTV[a%\t\x90` \x81\x01\x90aG\x16V[\x85\x81\x81\x10a%\x19Wa%\x19a>gV[\x90P` \x02\x01` \x81\x01\x90a%.\x91\x90a7\x8CV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a%i\x81aF\xFBV[\x91PPa#\xDDV[P\x80a%|\x81aF\xFBV[\x91PPa#'V[P`\0[a%\x95`\xE0\x83\x01\x83aG\x16V[\x90P\x81\x10\x15a'HW`\0[a%\xAE`\xE0\x84\x01\x84aG\x16V[\x83\x81\x81\x10a%\xBEWa%\xBEa>gV[\x90P` \x02\x81\x01\x90a%\xD0\x91\x90aHtV[a%\xDE\x90` \x81\x01\x90aG\x16V[\x90P\x81\x10\x15a'5Wa%\xF4`\xE0\x84\x01\x84aG\x16V[\x83\x81\x81\x10a&\x04Wa&\x04a>gV[\x90P` \x02\x81\x01\x90a&\x16\x91\x90aHtV[a&$\x90`@\x81\x01\x90aG\x16V[\x82\x81\x81\x10a&4Wa&4a>gV[\x90P` \x02\x01` \x81\x01\x90a&I\x91\x90aG\xA8V[`\x9F`\0a&Z`\xE0\x87\x01\x87aG\x16V[\x86\x81\x81\x10a&jWa&ja>gV[\x90P` \x02\x81\x01\x90a&|\x91\x90aHtV[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a&\x9D`\xE0\x87\x01\x87aG\x16V[\x86\x81\x81\x10a&\xADWa&\xADa>gV[\x90P` \x02\x81\x01\x90a&\xBF\x91\x90aHtV[a&\xCD\x90` \x81\x01\x90aG\x16V[\x85\x81\x81\x10a&\xDDWa&\xDDa>gV[\x90P` \x02\x01` \x81\x01\x90a&\xF2\x91\x90a7\x8CV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a'-\x81aF\xFBV[\x91PPa%\xA1V[P\x80a'@\x81aF\xFBV[\x91PPa%\x88V[P`\0[a'Za\x01\0\x83\x01\x83aG\xC3V[\x90P\x81\x10\x15a'\xFEWa'qa\x01\0\x83\x01\x83aG\xC3V[\x82\x81\x81\x10a'\x81Wa'\x81a>gV[\x90P`@\x02\x01` \x01` \x81\x01\x90a'\x99\x91\x90a7\x8CV[`\xA1`\0a'\xABa\x01\0\x86\x01\x86aG\xC3V[\x85\x81\x81\x10a'\xBBWa'\xBBa>gV[\x90P`@\x02\x01`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP\x80\x80a'\xF6\x90aF\xFBV[\x91PPa'LV[P`\xA0\x82\x015`\x9AUa(\x14` \x84\x01\x84a>\xE8V[`\x9B\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua(>``\x84\x01`@\x85\x01a>\xE8V[`\x9B\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90UB`\x99Ua(v`\x80\x84\x01\x84aH\x8AV[a(\x82\x91`\x9C\x91a6\x08V[Pa(\x93`\xC0\x84\x01`\xA0\x85\x01a>\xE8V[`\x9D\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x7F&Ie\xEBk\xC46\xC6\xC4sC\x1D4\xAFV\xE82\xEC4O\xDF\xD4>\xE6\xAFo\xCEm ^\x84\xAFa(\xDB` \x85\x01\x85a>\xE8V[a(\xEB``\x86\x01`@\x87\x01a>\xE8V[`@\x80Qc\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1PPPV[a)\x18a-*V[`\x01`\x01`\xA0\x1B\x03\x81\x16a)}W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x05\xABV[a\x05\xBD\x81a2#V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a)\xA6WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a)\xC0WP0;\x15\x80\x15a)\xC0WP`\0T`\xFF\x16`\x01\x14[a*#W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x05\xABV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a*FW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a*Q\x85`\0a3\xF4V[a*Z\x84a2#V[`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U`\x98\x80T\x92\x85\x16\x92\x90\x91\x16\x91\x90\x91\x17\x90U\x80\x15a*\xD0W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+*W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+N\x91\x90a=\x8DV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a+~W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xAB\x90a=\xAAV[`fT\x19\x81\x19`fT\x19\x16\x14a+\xFCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\xABV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x07\x1EV[`\x01`\x01`\xA0\x1B\x03\x81\x16a,\xC1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x05\xABV[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x18]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x05\xABV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra-\xA0a6\x8CV[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x80\x82\x01\x84\x90R`\0\x90\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a-\xD3Wa-\xD5V[\xFE[P\x80a.\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCB[][\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x05\xABV[PP\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra.7a6\xAAV[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R\x83Q`@\x80\x84\x01\x91\x90\x91R\x90\x84\x01Q``\x83\x01R`\0\x90\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a-\xD3WP\x80a.\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCBXY\x19\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x05\xABV[a.\xB7a6\xC8V[P`@\x80Q`\x80\x81\x01\x82R\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81\x83\x01\x90\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED``\x83\x01R\x81R\x81Q\x80\x83\x01\x90\x92R\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x81\x01\x91\x90\x91R\x81\x01\x91\x90\x91R\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0\x80\x80a/\x9F`\0\x80Q` aI\x10\x839\x81Q\x91R\x86a>}V[\x90P[a/\xAB\x81a4\xDEV[\x90\x93P\x91P`\0\x80Q` aI\x10\x839\x81Q\x91R\x82\x83\t\x83\x14\x15a/\xE5W`@\x80Q\x80\x82\x01\x90\x91R\x90\x81R` \x81\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80Q` aI\x10\x839\x81Q\x91R`\x01\x82\x08\x90Pa/\xA2V[`@\x80Q\x80\x82\x01\x82R\x86\x81R` \x80\x82\x01\x86\x90R\x82Q\x80\x84\x01\x90\x93R\x86\x83R\x82\x01\x84\x90R`\0\x91\x82\x91\x90a01a6\xEDV[`\0[`\x02\x81\x10\x15a1\xF6W`\0a0J\x82`\x06aF\xDCV[\x90P\x84\x82`\x02\x81\x10a0^Wa0^a>gV[` \x02\x01QQ\x83a0p\x83`\0aF\x89V[`\x0C\x81\x10a0\x80Wa0\x80a>gV[` \x02\x01R\x84\x82`\x02\x81\x10a0\x97Wa0\x97a>gV[` \x02\x01Q` \x01Q\x83\x82`\x01a0\xAE\x91\x90aF\x89V[`\x0C\x81\x10a0\xBEWa0\xBEa>gV[` \x02\x01R\x83\x82`\x02\x81\x10a0\xD5Wa0\xD5a>gV[` \x02\x01QQQ\x83a0\xE8\x83`\x02aF\x89V[`\x0C\x81\x10a0\xF8Wa0\xF8a>gV[` \x02\x01R\x83\x82`\x02\x81\x10a1\x0FWa1\x0Fa>gV[` \x02\x01QQ`\x01` \x02\x01Q\x83a1(\x83`\x03aF\x89V[`\x0C\x81\x10a18Wa18a>gV[` \x02\x01R\x83\x82`\x02\x81\x10a1OWa1Oa>gV[` \x02\x01Q` \x01Q`\0`\x02\x81\x10a1jWa1ja>gV[` \x02\x01Q\x83a1{\x83`\x04aF\x89V[`\x0C\x81\x10a1\x8BWa1\x8Ba>gV[` \x02\x01R\x83\x82`\x02\x81\x10a1\xA2Wa1\xA2a>gV[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a1\xBDWa1\xBDa>gV[` \x02\x01Q\x83a1\xCE\x83`\x05aF\x89V[`\x0C\x81\x10a1\xDEWa1\xDEa>gV[` \x02\x01RP\x80a1\xEE\x81aF\xFBV[\x91PPa04V[Pa1\xFFa7\x0CV[`\0` \x82a\x01\x80\x85`\x08\x8C\xFA\x91Q\x91\x9C\x91\x15\x15\x9BP\x90\x99PPPPPPPPPPV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x02\0\x82a\xFF\xFF\x16\x10a2\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Roscalar-too-large`\x80\x1B`D\x82\x01R`d\x01a\x05\xABV[\x81a\xFF\xFF\x16`\x01\x14\x15a2\xE5WP\x81a\x1EFV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01\x81\x90R\x84\x90`\x01\x90[\x81a\xFF\xFF\x16\x86a\xFF\xFF\x16\x10a3NW`\x01a\xFF\xFF\x87\x16`\xFF\x83\x16\x1C\x81\x16\x14\x15a31Wa3.\x84\x84a.\x1BV[\x93P[a3;\x83\x84a.\x1BV[\x92Pb\x01\xFF\xFE`\x01\x92\x83\x1B\x16\x91\x01a3\x01V[P\x91\x95\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q\x15\x80\x15a3~WP` \x82\x01Q\x15[\x15a3\x9CWPP`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01`\0\x80Q` aI\x10\x839\x81Q\x91R\x84` \x01Qa3\xCF\x91\x90a>}V[a3\xE7\x90`\0\x80Q` aI\x10\x839\x81Q\x91RaH\xD0V[\x90R\x92\x91PPV[\x91\x90PV[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a4\x15WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a4\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x05\xABV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a4\xDA\x82a,3V[PPV[`\0\x80\x80`\0\x80Q` aI\x10\x839\x81Q\x91R`\x03`\0\x80Q` aI\x10\x839\x81Q\x91R\x86`\0\x80Q` aI\x10\x839\x81Q\x91R\x88\x89\t\t\x08\x90P`\0a5T\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\0\x80Q` aI\x10\x839\x81Q\x91Ra5`V[\x91\x95\x91\x94P\x90\x92PPPV[`\0\x80a5ka7\x0CV[a5sa7*V[` \x80\x82R\x81\x81\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x88\x90R`\x80\x82\x01\x87\x90R`\xA0\x82\x01\x86\x90R\x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82\x80\x15a-\xD3WP\x82a5\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FBN254.expMod: call failure\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\xABV[PQ\x95\x94PPPPPV[\x82\x80Ta6\x14\x90a>\x9FV[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82a66W`\0\x85Ua6|V[\x82`\x1F\x10a6OW\x82\x80\x01`\xFF\x19\x825\x16\x17\x85Ua6|V[\x82\x80\x01`\x01\x01\x85U\x82\x15a6|W\x91\x82\x01[\x82\x81\x11\x15a6|W\x825\x82U\x91` \x01\x91\x90`\x01\x01\x90a6aV[Pa6\x88\x92\x91Pa7HV[P\x90V[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80a6\xDBa7]V[\x81R` \x01a6\xE8a7]V[\x90R\x90V[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[[\x80\x82\x11\x15a6\x88W`\0\x81U`\x01\x01a7IV[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[\x805`\xFF\x81\x16\x81\x14a3\xEFW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a7\x9EW`\0\x80\xFD[a7\xA7\x82a7{V[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05\xBDW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a7\xD5W`\0\x80\xFD[\x815a7\xA7\x81a7\xAEV[`\0` \x82\x84\x03\x12\x15a7\xF2W`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a81Wa81a7\xF9V[`@R\x90V[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a81Wa81a7\xF9V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a8\x81Wa8\x81a7\xF9V[`@R\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a8\x9BW`\0\x80\xFD[a8\xA3a8\x0FV[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a8\xCAW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a8\xECWa8\xECa7\xF9V[\x80`@RP\x80`@\x84\x01\x85\x81\x11\x15a9\x03W`\0\x80\xFD[\x84[\x81\x81\x10\x15a3NW\x805\x83R` \x92\x83\x01\x92\x01a9\x05V[`\0`\x80\x82\x84\x03\x12\x15a9/W`\0\x80\xFD[a97a8\x0FV[\x90Pa9C\x83\x83a8\xB9V[\x81Ra9R\x83`@\x84\x01a8\xB9V[` \x82\x01R\x92\x91PPV[`\0\x80`\0\x80a\x01 \x85\x87\x03\x12\x15a9tW`\0\x80\xFD[\x845\x93Pa9\x85\x86` \x87\x01a8\x89V[\x92Pa9\x94\x86``\x87\x01a9\x1DV[\x91Pa9\xA3\x86`\xE0\x87\x01a8\x89V[\x90P\x92\x95\x91\x94P\x92PV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a9\xDBW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a9\xBFV[\x81\x81\x11\x15a9\xEDW`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[`\0a\x01\0\x82\x84\x03\x12\x15a:\x16W`\0\x80\xFD[P\x91\x90PV[`\0`\xC0\x82\x84\x03\x12\x15a:\x16W`\0\x80\xFD[`\0a\x01 \x82\x84\x03\x12\x15a:\x16W`\0\x80\xFD[`\0\x80`\0\x80a\x01 \x85\x87\x03\x12\x15a:XW`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a:oW`\0\x80\xFD[a:{\x88\x83\x89\x01a:\x03V[\x95Pa:\x8A\x88` \x89\x01a:\x1CV[\x94P`\xE0\x87\x015\x91P\x80\x82\x11\x15a:\xA0W`\0\x80\xFD[\x90\x86\x01\x90`\xE0\x82\x89\x03\x12\x15a:\xB4W`\0\x80\xFD[\x90\x92Pa\x01\0\x86\x015\x90\x80\x82\x11\x15a:\xCBW`\0\x80\xFD[Pa:\xD8\x87\x82\x88\x01a:.V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a:\xF7W`\0\x80\xFD[\x825\x91Pa;\x07` \x84\x01a7{V[\x90P\x92P\x92\x90PV[`\0`\xE0\x82\x84\x03\x12\x15a;\"W`\0\x80\xFD[a;*a87V[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a;CW`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a;WW`\0\x80\xFD[\x815` \x82\x82\x11\x15a;kWa;ka7\xF9V[a;y\x81\x83`\x05\x1B\x01a8YV[\x82\x81R\x81\x81\x01\x93P`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x87\x83\x11\x15a;\x99W`\0\x80\xFD[\x93\x81\x01\x93[\x82\x85\x10\x15a;\xC2Wa;\xB0\x88\x86a8\x89V[\x84R\x81\x84\x01\x93P`@\x85\x01\x94Pa;\x9EV[\x85Ra;\xD0\x87\x87\x83\x01a9\x1DV[\x81\x86\x01RPPPPa;\xE5\x83`\xA0\x84\x01a8\x89V[`@\x82\x01R\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a<\x03W`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a< W`\0\x80\xFD[a<,\x85\x82\x86\x01a;\x10V[\x91PP\x92P\x92\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a<oW\x81Q`\x01`\x01``\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a<JV[P\x94\x95\x94PPPPPV[` \x81R`\0\x82Q`@` \x84\x01Ra<\x96``\x84\x01\x82a<6V[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01Ra<\xB3\x82\x82a<6V[\x95\x94PPPPPV[`\0\x80`\0a\x01\0\x84\x86\x03\x12\x15a<\xD2W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a<\xE9W`\0\x80\xFD[a<\xF5\x87\x83\x88\x01a:\x03V[\x94Pa=\x04\x87` \x88\x01a:\x1CV[\x93P`\xE0\x86\x015\x91P\x80\x82\x11\x15a=\x1AW`\0\x80\xFD[Pa='\x86\x82\x87\x01a:.V[\x91PP\x92P\x92P\x92V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a=GW`\0\x80\xFD[\x845a=R\x81a7\xAEV[\x93P` \x85\x015a=b\x81a7\xAEV[\x92P`@\x85\x015a=r\x81a7\xAEV[\x91P``\x85\x015a=\x82\x81a7\xAEV[\x93\x96\x92\x95P\x90\x93PPV[`\0` \x82\x84\x03\x12\x15a=\x9FW`\0\x80\xFD[\x81Qa7\xA7\x81a7\xAEV[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[\x80\x15\x15\x81\x14a\x05\xBDW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a>\x14W`\0\x80\xFD[\x81Qa7\xA7\x81a=\xF4V[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x82a>\x9AWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80a>\xB3W`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15a:\x16WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a3\xEFW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a>\xFAW`\0\x80\xFD[a7\xA7\x82a>\xD4V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a?\x1AW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a?9W`\0\x80\xFD[\x806\x03\x83\x13\x15a?HW`\0\x80\xFD[\x92P\x92\x90PV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x81Rc\xFF\xFF\xFF\xFFa?\x8A\x83a>\xD4V[\x16` \x82\x01R` \x82\x015`@\x82\x01R`\0a?\xA8`@\x84\x01a>\xD4V[c\xFF\xFF\xFF\xFF\x81\x16``\x84\x01RPa?\xC1``\x84\x01a>\xD4V[c\xFF\xFF\xFF\xFF\x81\x16`\x80\x84\x01RPa?\xDB`\x80\x84\x01\x84a?\x03V[a\x01\0\x80`\xA0\x86\x01Ra?\xF3a\x01 \x86\x01\x83\x85a?OV[\x92Pa@\x01`\xA0\x87\x01a>\xD4V[c\xFF\xFF\xFF\xFF\x81\x16`\xC0\x87\x01R\x91Pa@\x1C`\xC0\x87\x01\x87a?\x03V[\x86\x85\x03`\x1F\x19\x01`\xE0\x88\x01R\x92Pa@5\x84\x84\x83a?OV[\x93PPa@D`\xE0\x87\x01a>\xD4V[c\xFF\xFF\xFF\xFF\x16\x94\x01\x93\x90\x93R\x93\x92PPPV[\x805a3\xEF\x81a=\xF4V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a@yW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a@\x98W`\0\x80\xFD[\x80`\x05\x1B6\x03\x83\x13\x15a?HW`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a<oW`\xFFa@\xCD\x83a7{V[\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a@\xBAV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a@\xF7W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aA\x16W`\0\x80\xFD[\x80`\x07\x1B6\x03\x83\x13\x15a?HW`\0\x80\xFD[\x805`\x01`\x01``\x1B\x03\x81\x16\x81\x14a3\xEFW`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a<oW`\xFFaAb\x83a7{V[\x16\x87R`\x01`\x01``\x1B\x03aAx\x84\x84\x01aA(V[\x16\x83\x88\x01R`@\x82\x81\x015\x90\x88\x01R``\x80\x83\x015\x90\x88\x01R`\x80\x96\x87\x01\x96\x90\x91\x01\x90`\x01\x01aAOV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aA\xBAW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aA\xD9W`\0\x80\xFD[\x80`\x06\x1B6\x03\x83\x13\x15a?HW`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a<oW`\xFFaB\x0E\x83a7{V[\x16\x87R`\x01`\x01``\x1B\x03aB$\x84\x84\x01aA(V[\x16\x87\x84\x01R`@\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aA\xFBV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aBSW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aBrW`\0\x80\xFD[``\x81\x026\x03\x83\x13\x15a?HW`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a<oW`\xFFaB\xA7\x83a7{V[\x16\x87RaB\xC2\x83\x88\x01\x84\x84\x01\x805\x82R` \x90\x81\x015\x91\x01RV[``\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aB\x94V[\x81\x83R`\0`\x01`\x01`\xFB\x1B\x03\x83\x11\x15aB\xEEW`\0\x80\xFD[\x82`\x05\x1B\x80\x83` \x87\x017`\0\x94\x01` \x01\x93\x84RP\x91\x92\x91PPV[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a<oW`\x01`\x01``\x1B\x03aC4\x83aA(V[\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aC\x1BV[\x81\x83R`\0` \x80\x85\x01\x80\x81\x96P\x85`\x05\x1B\x81\x01\x91P\x84`\0\x80[\x88\x81\x10\x15aC\xF8W\x83\x85\x03\x8AR\x825`~\x19\x896\x03\x01\x81\x12aC\x82W\x82\x83\xFD[\x88\x01\x805\x86R`\x80aC\x96\x88\x83\x01\x83a@bV[\x82\x8A\x8A\x01RaC\xA8\x83\x8A\x01\x82\x84a@\xAAV[\x92PPP`@aC\xBA\x81\x84\x01\x84a@bV[\x89\x84\x03\x83\x8B\x01RaC\xCC\x84\x82\x84aC\x0BV[\x93PPPP```\xFFaC\xE0\x82\x85\x01a7{V[\x16\x97\x01\x96\x90\x96RP\x98\x85\x01\x98\x91\x85\x01\x91`\x01\x01aCbV[P\x92\x98\x97PPPPPPPPV[\x81\x83R`\0` \x80\x85\x01\x80\x81\x96P\x85`\x05\x1B\x81\x01\x91P\x84`\0\x80[\x88\x81\x10\x15aC\xF8W\x83\x85\x03\x8AR\x825`^\x19\x896\x03\x01\x81\x12aDAW\x82\x83\xFD[\x88\x01\x805\x86R``aDU\x88\x83\x01\x83a@bV[\x82\x8A\x8A\x01RaDg\x83\x8A\x01\x82\x84a@\xAAV[\x92PPP`@aDy\x81\x84\x01\x84a@bV[\x93P\x88\x83\x03\x82\x8A\x01RaD\x8D\x83\x85\x83aC\x0BV[\x9D\x8A\x01\x9D\x98PPP\x93\x87\x01\x93PP`\x01\x01aD!V[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a<oW\x815\x87R`\xFFaD\xCC\x84\x84\x01a7{V[\x16\x87\x84\x01R`@\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aD\xB3V[` \x81RaD\xFE` \x82\x01aD\xF8\x84a@WV[\x15\x15\x90RV[`\0aE\r` \x84\x01\x84a@bV[a\x01 \x80`@\x86\x01RaE%a\x01@\x86\x01\x83\x85a@\xAAV[\x92PaE4`@\x87\x01\x87a@\xE0V[\x92P`\x1F\x19\x80\x87\x86\x03\x01``\x88\x01RaEN\x85\x85\x84aA?V[\x94PaE]``\x89\x01\x89aA\xA3V[\x94P\x91P\x80\x87\x86\x03\x01`\x80\x88\x01RaEv\x85\x85\x84aA\xEBV[\x94PaE\x85`\x80\x89\x01\x89aB<V[\x94P\x91P\x80\x87\x86\x03\x01`\xA0\x88\x01RaE\x9E\x85\x85\x84aB\x84V[\x94PaE\xAD`\xA0\x89\x01\x89a@bV[\x94P\x91P\x80\x87\x86\x03\x01`\xC0\x88\x01RaE\xC6\x85\x85\x84aB\xD5V[\x94PaE\xD5`\xC0\x89\x01\x89a@bV[\x94P\x91P\x80\x87\x86\x03\x01`\xE0\x88\x01RaE\xEE\x85\x85\x84aCGV[\x94PaE\xFD`\xE0\x89\x01\x89a@bV[\x94P\x91Pa\x01\0\x81\x88\x87\x03\x01\x81\x89\x01RaF\x18\x86\x86\x85aD\x06V[\x95PaF&\x81\x8A\x01\x8AaA\xA3V[\x95P\x92PP\x80\x87\x86\x03\x01\x83\x88\x01RPaF@\x84\x84\x83aD\xA3V[\x97\x96PPPPPPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aF\x80WaF\x80aFKV[\x01\x94\x93PPPPV[`\0\x82\x19\x82\x11\x15aF\x9CWaF\x9CaFKV[P\x01\x90V[`\0a\x1EF6\x83a;\x10V[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aF\xD3WaF\xD3aFKV[\x02\x94\x93PPPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15aF\xF6WaF\xF6aFKV[P\x02\x90V[`\0`\0\x19\x82\x14\x15aG\x0FWaG\x0FaFKV[P`\x01\x01\x90V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aG-W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aGGW`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a?HW`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aGvW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aG\x90W`\0\x80\xFD[` \x01\x91P`\x07\x81\x90\x1B6\x03\x82\x13\x15a?HW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aG\xBAW`\0\x80\xFD[a7\xA7\x82aA(V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aG\xDAW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aG\xF4W`\0\x80\xFD[` \x01\x91P`\x06\x81\x90\x1B6\x03\x82\x13\x15a?HW`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aH#W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aH=W`\0\x80\xFD[` \x01\x91P``\x81\x026\x03\x82\x13\x15a?HW`\0\x80\xFD[`\0\x825`~\x19\x836\x03\x01\x81\x12aHjW`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x825`^\x19\x836\x03\x01\x81\x12aHjW`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aH\xA1W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aH\xBBW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a?HW`\0\x80\xFD[`\0\x82\x82\x10\x15aH\xE2WaH\xE2aFKV[P\x03\x90V[`\0`\x01`\x01``\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15aI\x07WaI\x07aFKV[\x03\x93\x92PPPV\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 \xD9>\xE2_`\xBE\xC0C\x85.\x81\x88Y_\x02.M\x13\xCEP\xECu\x13s\xA7\x06\xA4\x19]\x08\xC6odsolcC\0\x08\x0C\x003";
    /// The deployed bytecode of the contract.
    pub static GASPMULTIROLLUPSERVICE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
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
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    GASPMULTIROLLUPSERVICE_ABI.clone(),
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
        ///Calls the contract's `initialize` (0xf8c8765e) function
        pub fn initialize(
            &self,
            pauser_registry: ::ethers::core::types::Address,
            initial_owner: ::ethers::core::types::Address,
            updater: ::ethers::core::types::Address,
            rolldown: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [248, 200, 118, 94],
                    (pauser_registry, initial_owner, updater, rolldown),
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
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
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
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
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
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
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
        pub fn renounce_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rolldown` (0x3d9fb00c) function
        pub fn rolldown(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([61, 159, 176, 12], ())
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
        ///Calls the contract's `set_rolldown` (0x1e2d4bf7) function
        pub fn set_rolldown(
            &self,
            rolldown: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([30, 45, 75, 247], rolldown)
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
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([223, 3, 76, 208], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `EigenReinitProcessed` event
        pub fn eigen_reinit_processed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EigenReinitProcessedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `EigenUpdateProcessed` event
        pub fn eigen_update_processed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EigenUpdateProcessedFilter,
        > {
            self.0.event()
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
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
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
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PauserRegistrySetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Unpaused` event
        pub fn unpaused_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UnpausedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            GaspMultiRollupServiceEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for GaspMultiRollupService<M> {
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
                return Ok(
                    GaspMultiRollupServiceEvents::EigenReinitProcessedFilter(decoded),
                );
            }
            if let Ok(decoded) = EigenUpdateProcessedFilter::decode_log(log) {
                return Ok(
                    GaspMultiRollupServiceEvents::EigenUpdateProcessedFilter(decoded),
                );
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(GaspMultiRollupServiceEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(
                    GaspMultiRollupServiceEvents::OwnershipTransferredFilter(decoded),
                );
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(GaspMultiRollupServiceEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = PauserRegistrySetFilter::decode_log(log) {
                return Ok(
                    GaspMultiRollupServiceEvents::PauserRegistrySetFilter(decoded),
                );
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
                Self::EigenReinitProcessedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EigenUpdateProcessedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PausedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauserRegistrySetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnpausedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<EigenReinitProcessedFilter>
    for GaspMultiRollupServiceEvents {
        fn from(value: EigenReinitProcessedFilter) -> Self {
            Self::EigenReinitProcessedFilter(value)
        }
    }
    impl ::core::convert::From<EigenUpdateProcessedFilter>
    for GaspMultiRollupServiceEvents {
        fn from(value: EigenUpdateProcessedFilter) -> Self {
            Self::EigenUpdateProcessedFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for GaspMultiRollupServiceEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter>
    for GaspMultiRollupServiceEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<PausedFilter> for GaspMultiRollupServiceEvents {
        fn from(value: PausedFilter) -> Self {
            Self::PausedFilter(value)
        }
    }
    impl ::core::convert::From<PauserRegistrySetFilter>
    for GaspMultiRollupServiceEvents {
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
        Hash
    )]
    #[ethcall(
        name = "checkSignatures",
        abi = "checkSignatures(bytes32,((uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256)))"
    )]
    pub struct CheckSignaturesCall {
        pub msg_hash: [u8; 32],
        pub params: NonSignerStakesAndSignatureForOldState,
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address,address,address)` and selector `0xf8c8765e`
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
    #[ethcall(name = "initialize", abi = "initialize(address,address,address,address)")]
    pub struct InitializeCall {
        pub pauser_registry: ::ethers::core::types::Address,
        pub initial_owner: ::ethers::core::types::Address,
        pub updater: ::ethers::core::types::Address,
        pub rolldown: ::ethers::core::types::Address,
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
        Hash
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
        Hash
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
        Hash
    )]
    #[ethcall(name = "latestCompletedTaskNumber", abi = "latestCompletedTaskNumber()")]
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
    )]
    #[ethcall(name = "quorumThresholdPercentage", abi = "quorumThresholdPercentage()")]
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
        Hash
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
        Hash
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `rolldown` function with signature `rolldown()` and selector `0x3d9fb00c`
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
    #[ethcall(name = "rolldown", abi = "rolldown()")]
    pub struct RolldownCall;
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
        Hash
    )]
    #[ethcall(name = "setPauserRegistry", abi = "setPauserRegistry(address)")]
    pub struct SetPauserRegistryCall {
        pub new_pauser_registry: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `set_rolldown` function with signature `set_rolldown(address)` and selector `0x1e2d4bf7`
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
    #[ethcall(name = "set_rolldown", abi = "set_rolldown(address)")]
    pub struct SetRolldownCall {
        pub rolldown: ::ethers::core::types::Address,
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Rolldown(RolldownCall),
        SetPauserRegistry(SetPauserRegistryCall),
        SetRolldown(SetRolldownCall),
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
            if let Ok(decoded) = <CheckSignaturesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CheckSignatures(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <LastUpdateBlockTimestampCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LastUpdateBlockTimestamp(decoded));
            }
            if let Ok(decoded) = <LatestCompletedTaskCreatedBlockCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LatestCompletedTaskCreatedBlock(decoded));
            }
            if let Ok(decoded) = <LatestCompletedTaskNumberCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LatestCompletedTaskNumber(decoded));
            }
            if let Ok(decoded) = <LatestPendingStateHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LatestPendingStateHash(decoded));
            }
            if let Ok(decoded) = <OperatorAndQuorumToStakesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OperatorAndQuorumToStakes(decoded));
            }
            if let Ok(decoded) = <OperatorIdQuorumCountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OperatorIdQuorumCount(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <PauseCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Pause(decoded));
            }
            if let Ok(decoded) = <PauseAllCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PauseAll(decoded));
            }
            if let Ok(decoded) = <PausedWithIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PausedWithIndex(decoded));
            }
            if let Ok(decoded) = <PausedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Paused(decoded));
            }
            if let Ok(decoded) = <PauserRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PauserRegistry(decoded));
            }
            if let Ok(decoded) = <ProcessEigenReinitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProcessEigenReinit(decoded));
            }
            if let Ok(decoded) = <ProcessEigenUpdateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProcessEigenUpdate(decoded));
            }
            if let Ok(decoded) = <QourumApkCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::QourumApk(decoded));
            }
            if let Ok(decoded) = <QuorumNumbersCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::QuorumNumbers(decoded));
            }
            if let Ok(decoded) = <QuorumThresholdPercentageCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::QuorumThresholdPercentage(decoded));
            }
            if let Ok(decoded) = <QuorumToStakesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::QuorumToStakes(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <RolldownCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Rolldown(decoded));
            }
            if let Ok(decoded) = <SetPauserRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetPauserRegistry(decoded));
            }
            if let Ok(decoded) = <SetRolldownCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetRolldown(decoded));
            }
            if let Ok(decoded) = <SetUpdaterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetUpdater(decoded));
            }
            if let Ok(decoded) = <StalledCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Stalled(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <TrySignatureAndApkVerificationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TrySignatureAndApkVerification(decoded));
            }
            if let Ok(decoded) = <UnpauseCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Unpause(decoded));
            }
            if let Ok(decoded) = <UpdaterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Updater(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for GaspMultiRollupServiceCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CheckSignatures(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
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
                Self::PauseAll(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PausedWithIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Paused(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PauserRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProcessEigenReinit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProcessEigenUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QourumApk(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QuorumNumbers(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QuorumThresholdPercentage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QuorumToStakes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Rolldown(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPauserRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetRolldown(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetUpdater(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Stalled(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
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
                Self::LastUpdateBlockTimestamp(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LatestCompletedTaskCreatedBlock(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LatestCompletedTaskNumber(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LatestPendingStateHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorAndQuorumToStakes(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorIdQuorumCount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pause(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauseAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::PausedWithIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::Paused(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauserRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProcessEigenReinit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProcessEigenUpdate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::QourumApk(element) => ::core::fmt::Display::fmt(element, f),
                Self::QuorumNumbers(element) => ::core::fmt::Display::fmt(element, f),
                Self::QuorumThresholdPercentage(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::QuorumToStakes(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::Rolldown(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPauserRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetRolldown(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<LastUpdateBlockTimestampCall>
    for GaspMultiRollupServiceCalls {
        fn from(value: LastUpdateBlockTimestampCall) -> Self {
            Self::LastUpdateBlockTimestamp(value)
        }
    }
    impl ::core::convert::From<LatestCompletedTaskCreatedBlockCall>
    for GaspMultiRollupServiceCalls {
        fn from(value: LatestCompletedTaskCreatedBlockCall) -> Self {
            Self::LatestCompletedTaskCreatedBlock(value)
        }
    }
    impl ::core::convert::From<LatestCompletedTaskNumberCall>
    for GaspMultiRollupServiceCalls {
        fn from(value: LatestCompletedTaskNumberCall) -> Self {
            Self::LatestCompletedTaskNumber(value)
        }
    }
    impl ::core::convert::From<LatestPendingStateHashCall>
    for GaspMultiRollupServiceCalls {
        fn from(value: LatestPendingStateHashCall) -> Self {
            Self::LatestPendingStateHash(value)
        }
    }
    impl ::core::convert::From<OperatorAndQuorumToStakesCall>
    for GaspMultiRollupServiceCalls {
        fn from(value: OperatorAndQuorumToStakesCall) -> Self {
            Self::OperatorAndQuorumToStakes(value)
        }
    }
    impl ::core::convert::From<OperatorIdQuorumCountCall>
    for GaspMultiRollupServiceCalls {
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
    impl ::core::convert::From<QuorumThresholdPercentageCall>
    for GaspMultiRollupServiceCalls {
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
    impl ::core::convert::From<RolldownCall> for GaspMultiRollupServiceCalls {
        fn from(value: RolldownCall) -> Self {
            Self::Rolldown(value)
        }
    }
    impl ::core::convert::From<SetPauserRegistryCall> for GaspMultiRollupServiceCalls {
        fn from(value: SetPauserRegistryCall) -> Self {
            Self::SetPauserRegistry(value)
        }
    }
    impl ::core::convert::From<SetRolldownCall> for GaspMultiRollupServiceCalls {
        fn from(value: SetRolldownCall) -> Self {
            Self::SetRolldown(value)
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
    impl ::core::convert::From<TrySignatureAndApkVerificationCall>
    for GaspMultiRollupServiceCalls {
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
    )]
    pub struct QuorumToStakesReturn(pub u128);
    ///Container type for all return fields from the `rolldown` function with signature `rolldown()` and selector `0x3d9fb00c`
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
    pub struct RolldownReturn(pub ::ethers::core::types::Address);
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
        Hash
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
        Hash
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
        Hash
    )]
    pub struct UpdaterReturn(pub ::ethers::core::types::Address);
}
