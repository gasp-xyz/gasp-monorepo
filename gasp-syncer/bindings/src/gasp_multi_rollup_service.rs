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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa;\xA3\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xA9W`\x005`\xE0\x1C\x80cRn>d\x11a\0\xF9W\x80c\x8D\xA5\xCB[\x11a\0\x97W\x80c\xEDZ\x04\xFE\x11a\0qW\x80c\xEDZ\x04\xFE\x14a\x04VW\x80c\xF2\xFD\xE3\x8B\x14a\x04fW\x80c\xF8\xC8v^\x14a\x04yW\x80c\xFA\xBC\x1C\xBC\x14a\x04\x8CW`\0\x80\xFD[\x80c\x8D\xA5\xCB[\x14a\x04\x12W\x80c\xC4\xE1\x91L\x14a\x04#W\x80c\xDF\x03L\xD0\x14a\x04CW`\0\x80\xFD[\x80c\\\x97Z\xBB\x11a\0\xD3W\x80c\\\x97Z\xBB\x14a\x03\xC6W\x80cqP\x18\xA6\x14a\x03\xCEW\x80cz\xD7Ua\x14a\x03\xD6W\x80c\x88o\x11\x95\x14a\x03\xFFW`\0\x80\xFD[\x80cRn>d\x14a\x03wW\x80cY\\jg\x14a\x03\x9BW\x80cZ\xC8j\xB7\x14a\x03\xA3W`\0\x80\xFD[\x80c\x1E-K\xF7\x11a\x01fW\x80cC\r;9\x11a\x01@W\x80cC\r;9\x14a\x02\xBAW\x80cI\x9Do\xB6\x14a\x02\xEFW\x80cJ\xE6\xB2\x03\x14a\x03;W\x80cM\xEA\xBC!\x14a\x03RW`\0\x80\xFD[\x80c\x1E-K\xF7\x14a\x02gW\x80c*\x84\x14\xFD\x14a\x02zW\x80c=\x9F\xB0\x0C\x14a\x02\x8FW`\0\x80\xFD[\x80c\x03R\x03U\x14a\x01\xAEW\x80c\x03\xD0\x97\xD2\x14a\x01\xC3W\x80c\x10\xD6z/\x14a\x02\x04W\x80c\x12FH\xC9\x14a\x02\x17W\x80c\x13d9\xDD\x14a\x02*W\x80c\x17\x1F\x1D[\x14a\x02=W[`\0\x80\xFD[a\x01\xC1a\x01\xBC6`\x04a*\xA7V[a\x04\x9FV[\0[a\x01\xEAa\x01\xD16`\x04a+dV[`\x9F` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T\x82V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xC1a\x02\x126`\x04a+\x9BV[a\x10\xFEV[a\x01\xC1a\x02%6`\x04a+\x9BV[a\x11\xB1V[a\x01\xC1a\x0286`\x04a+\xB8V[a\x11\xDBV[a\x02Pa\x02K6`\x04a-5V[a\x13\x1AV[`@\x80Q\x92\x15\x15\x83R\x90\x15\x15` \x83\x01R\x01a\x01\xFBV[a\x01\xC1a\x02u6`\x04a+\x9BV[a\x14\xA4V[a\x02\x82a\x14\xCEV[`@Qa\x01\xFB\x91\x90a-\x86V[`\x98Ta\x02\xA2\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xFBV[a\x02\xDDa\x02\xC86`\x04a+\xB8V[`\xA0` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x01\xFBV[a\x03#a\x02\xFD6`\x04a-\xDBV[`\x9E` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\x01`\x01``\x1B\x03\x16\x81V[`@Q`\x01`\x01``\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xFBV[a\x03D`\x99T\x81V[`@Q\x90\x81R` \x01a\x01\xFBV[`\x9CTa\x03b\x90c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xFBV[`\x98Ta\x03\x8B\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01\xFBV[a\x01\xC1a\x15\\V[a\x03\x8Ba\x03\xB16`\x04a+dV[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`fTa\x03DV[a\x01\xC1a\x16#V[a\x03#a\x03\xE46`\x04a+dV[`\x9D` R`\0\x90\x81R`@\x90 T`\x01`\x01``\x1B\x03\x16\x81V[`eTa\x02\xA2\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x02\xA2V[a\x046a\x0416`\x04a.\xE7V[a\x167V[`@Qa\x01\xFB\x91\x90a/qV[`\x97Ta\x02\xA2\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x9ATa\x03b\x90c\xFF\xFF\xFF\xFF\x16\x81V[a\x01\xC1a\x04t6`\x04a+\x9BV[a\x1C$V[a\x01\xC1a\x04\x876`\x04a/\xB3V[a\x1C\x9AV[a\x01\xC1a\x04\x9A6`\x04a+\xB8V[a\x1D\xEBV[a\x04\xAF``\x85\x01`@\x86\x01a0\x18V[`\x9ATc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14a\x05\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7Freference block hash mismatch\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[\x83`@Q` \x01a\x05\x1F\x91\x90a0\xA8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x83` \x015\x14a\x05\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FreferenceTaskHash hash mismatch\0`D\x82\x01R`d\x01a\x05\x05V[\x80`@Q` \x01a\x05\x98\x91\x90a5\xFAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x83`@\x015\x14a\x06\0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FoperatorStateInfo hash mismatch\0`D\x82\x01R`d\x01a\x05\x05V[`\x9ATc\xFF\xFF\xFF\xFF\x16\x15a\x06\xD2W`\0a\x06!` \x85\x015a\x041\x85a7xV[`\x9CT\x90\x91Pc\xFF\xFF\xFF\xFF\x16`\0[`\x9B\x80Ta\x06=\x90a7\x84V[\x90P\x81\x10\x15a\x06\xCEW\x81`\xFF\x16\x83` \x01Q\x82\x81Q\x81\x10a\x06`Wa\x06`a7\xB9V[` \x02` \x01\x01Qa\x06r\x91\x90a7\xE5V[`\x01`\x01``\x1B\x03\x16`d\x84`\0\x01Q\x83\x81Q\x81\x10a\x06\x93Wa\x06\x93a7\xB9V[` \x02` \x01\x01Q`\x01`\x01``\x1B\x03\x16a\x06\xAE\x91\x90a8\x14V[\x10\x15a\x06\xBCWPPPa\x10\xF8V[\x80a\x06\xC6\x81a83V[\x91PPa\x060V[PPP[`\0[a\x06\xE2`@\x83\x01\x83a8NV[\x90P\x81\x10\x15a\x07\xA8W`\x9D`\0a\x06\xFC`@\x85\x01\x85a8NV[\x84\x81\x81\x10a\x07\x0CWa\x07\x0Ca7\xB9V[\x90P` \x02\x01` \x81\x01\x90a\x07!\x91\x90a+dV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x90\x81\x01`\0\x90\x81 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U`\x9F\x91a\x07U\x90\x85\x01\x85a8NV[\x84\x81\x81\x10a\x07eWa\x07ea7\xB9V[\x90P` \x02\x01` \x81\x01\x90a\x07z\x91\x90a+dV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x81\x81U`\x01\x01U\x80a\x07\xA0\x81a83V[\x91PPa\x06\xD5V[P`\0[a\x07\xB9``\x83\x01\x83a8\x97V[\x90P\x81\x10\x15a\x08\xF2Wa\x07\xCF``\x83\x01\x83a8\x97V[\x82\x81\x81\x10a\x07\xDFWa\x07\xDFa7\xB9V[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\x07\xF7\x91\x90a8\xE0V[`\x9D`\0a\x08\x08``\x86\x01\x86a8\x97V[\x85\x81\x81\x10a\x08\x18Wa\x08\x18a7\xB9V[a\x08.\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa+dV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\x08l``\x83\x01\x83a8\x97V[\x82\x81\x81\x10a\x08|Wa\x08|a7\xB9V[\x90P`\x80\x02\x01`@\x01`\x9F`\0\x84\x80``\x01\x90a\x08\x99\x91\x90a8\x97V[\x85\x81\x81\x10a\x08\xA9Wa\x08\xA9a7\xB9V[a\x08\xBF\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa+dV[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a\x08\xEA\x81a83V[\x91PPa\x07\xACV[P`\0[a\t\x03`\x80\x83\x01\x83a8\xFBV[\x90P\x81\x10\x15a\t\xBBWa\t\x19`\x80\x83\x01\x83a8\xFBV[\x82\x81\x81\x10a\t)Wa\t)a7\xB9V[\x90P`@\x02\x01` \x01` \x81\x01\x90a\tA\x91\x90a8\xE0V[`\x9D`\0a\tR`\x80\x86\x01\x86a8\xFBV[\x85\x81\x81\x10a\tbWa\tba7\xB9V[a\tx\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa+dV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\t\xB3\x81a83V[\x91PPa\x08\xF6V[P`\0[a\t\xCC`\xA0\x83\x01\x83a9DV[\x90P\x81\x10\x15a\nhWa\t\xE2`\xA0\x83\x01\x83a9DV[\x82\x81\x81\x10a\t\xF2Wa\t\xF2a7\xB9V[\x90P``\x02\x01` \x01`\x9F`\0\x84\x80`\xA0\x01\x90a\n\x0F\x91\x90a9DV[\x85\x81\x81\x10a\n\x1FWa\n\x1Fa7\xB9V[a\n5\x92` ``\x90\x92\x02\x01\x90\x81\x01\x91Pa+dV[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a\n`\x81a83V[\x91PPa\t\xBFV[P`\0[a\ny`\xC0\x83\x01\x83a8NV[\x90P\x81\x10\x15a\x0B\xA0W`\0[`\x9B\x80Ta\n\x92\x90a7\x84V[\x90P\x81\x10\x15a\x0BKW`\x9E`\0a\n\xAC`\xC0\x86\x01\x86a8NV[\x85\x81\x81\x10a\n\xBCWa\n\xBCa7\xB9V[\x90P` \x02\x015\x81R` \x01\x90\x81R` \x01`\0 `\0`\x9B\x83\x81Ta\n\xE1\x90a7\x84V[\x81\x10a\n\xEFWa\n\xEFa7\xB9V[\x81T`\x01\x16\x15a\x0B\x0EW\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U\x81a\x0BC\x81a83V[\x92PPa\n\x85V[P`\xA0`\0a\x0B]`\xC0\x85\x01\x85a8NV[\x84\x81\x81\x10a\x0BmWa\x0Bma7\xB9V[` \x90\x81\x02\x92\x90\x92\x015\x83RP\x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\xFF\x19\x16\x90U\x80a\x0B\x98\x81a83V[\x91PPa\nlV[P`\0[a\x0B\xB1`\xE0\x83\x01\x83a8NV[\x90P\x81\x10\x15a\x0E\x01Wa\x0B\xC7`\xE0\x83\x01\x83a8NV[\x82\x81\x81\x10a\x0B\xD7Wa\x0B\xD7a7\xB9V[\x90P` \x02\x81\x01\x90a\x0B\xE9\x91\x90a9\x8CV[a\x0B\xFA\x90`\x80\x81\x01\x90``\x01a+dV[`\xA0`\0a\x0C\x0B`\xE0\x86\x01\x86a8NV[\x85\x81\x81\x10a\x0C\x1BWa\x0C\x1Ba7\xB9V[\x90P` \x02\x81\x01\x90a\x0C-\x91\x90a9\x8CV[`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP`\0[a\x0Cg`\xE0\x84\x01\x84a8NV[\x83\x81\x81\x10a\x0CwWa\x0Cwa7\xB9V[\x90P` \x02\x81\x01\x90a\x0C\x89\x91\x90a9\x8CV[a\x0C\x97\x90` \x81\x01\x90a8NV[\x90P\x81\x10\x15a\r\xEEWa\x0C\xAD`\xE0\x84\x01\x84a8NV[\x83\x81\x81\x10a\x0C\xBDWa\x0C\xBDa7\xB9V[\x90P` \x02\x81\x01\x90a\x0C\xCF\x91\x90a9\x8CV[a\x0C\xDD\x90`@\x81\x01\x90a8NV[\x82\x81\x81\x10a\x0C\xEDWa\x0C\xEDa7\xB9V[\x90P` \x02\x01` \x81\x01\x90a\r\x02\x91\x90a8\xE0V[`\x9E`\0a\r\x13`\xE0\x87\x01\x87a8NV[\x86\x81\x81\x10a\r#Wa\r#a7\xB9V[\x90P` \x02\x81\x01\x90a\r5\x91\x90a9\x8CV[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a\rV`\xE0\x87\x01\x87a8NV[\x86\x81\x81\x10a\rfWa\rfa7\xB9V[\x90P` \x02\x81\x01\x90a\rx\x91\x90a9\x8CV[a\r\x86\x90` \x81\x01\x90a8NV[\x85\x81\x81\x10a\r\x96Wa\r\x96a7\xB9V[\x90P` \x02\x01` \x81\x01\x90a\r\xAB\x91\x90a+dV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\r\xE6\x81a83V[\x91PPa\x0CZV[P\x80a\r\xF9\x81a83V[\x91PPa\x0B\xA4V[P`\0[a\x0E\x13a\x01\0\x83\x01\x83a8NV[\x90P\x81\x10\x15a\x0F\xCAW`\0[a\x0E-a\x01\0\x84\x01\x84a8NV[\x83\x81\x81\x10a\x0E=Wa\x0E=a7\xB9V[\x90P` \x02\x81\x01\x90a\x0EO\x91\x90a9\xACV[a\x0E]\x90` \x81\x01\x90a8NV[\x90P\x81\x10\x15a\x0F\xB7Wa\x0Eta\x01\0\x84\x01\x84a8NV[\x83\x81\x81\x10a\x0E\x84Wa\x0E\x84a7\xB9V[\x90P` \x02\x81\x01\x90a\x0E\x96\x91\x90a9\xACV[a\x0E\xA4\x90`@\x81\x01\x90a8NV[\x82\x81\x81\x10a\x0E\xB4Wa\x0E\xB4a7\xB9V[\x90P` \x02\x01` \x81\x01\x90a\x0E\xC9\x91\x90a8\xE0V[`\x9E`\0a\x0E\xDBa\x01\0\x87\x01\x87a8NV[\x86\x81\x81\x10a\x0E\xEBWa\x0E\xEBa7\xB9V[\x90P` \x02\x81\x01\x90a\x0E\xFD\x91\x90a9\xACV[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a\x0F\x1Fa\x01\0\x87\x01\x87a8NV[\x86\x81\x81\x10a\x0F/Wa\x0F/a7\xB9V[\x90P` \x02\x81\x01\x90a\x0FA\x91\x90a9\xACV[a\x0FO\x90` \x81\x01\x90a8NV[\x85\x81\x81\x10a\x0F_Wa\x0F_a7\xB9V[\x90P` \x02\x01` \x81\x01\x90a\x0Ft\x91\x90a+dV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\x0F\xAF\x81a83V[\x91PPa\x0E\x1FV[P\x80a\x0F\xC2\x81a83V[\x91PPa\x0E\x05V[P`\0[a\x0F\xDCa\x01 \x83\x01\x83a8\xFBV[\x90P\x81\x10\x15a\x10\x80Wa\x0F\xF3a\x01 \x83\x01\x83a8\xFBV[\x82\x81\x81\x10a\x10\x03Wa\x10\x03a7\xB9V[\x90P`@\x02\x01` \x01` \x81\x01\x90a\x10\x1B\x91\x90a+dV[`\xA0`\0a\x10-a\x01 \x86\x01\x86a8\xFBV[\x85\x81\x81\x10a\x10=Wa\x10=a7\xB9V[\x90P`@\x02\x01`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP\x80\x80a\x10x\x90a83V[\x91PPa\x0F\xCEV[P`\xA0\x83\x015`\x99Ua\x10\x99`@\x85\x01` \x86\x01a0\x18V[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\x10\xC0``\x85\x01\x85a9\xC2V[a\x10\xCC\x91`\x9B\x91a)\x1CV[Pa\x10\xDD`\xA0\x85\x01`\x80\x86\x01a0\x18V[`\x9C\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U[PPPPV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11QW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11u\x91\x90a:\x08V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x11\xA5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x05\x90a:%V[a\x11\xAE\x81a\x1FGV[PV[a\x11\xB9a >V[`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12#W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12G\x91\x90a:oV[a\x12cW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x05\x90a:\x8CV[`fT\x81\x81\x16\x14a\x12\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\x05V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`\0\x80`\0\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x87\x87`\0\x01Q\x88` \x01Q\x88`\0\x01Q`\0`\x02\x81\x10a\x13bWa\x13ba7\xB9V[` \x02\x01Q\x89Q`\x01` \x02\x01Q\x8A` \x01Q`\0`\x02\x81\x10a\x13\x87Wa\x13\x87a7\xB9V[` \x02\x01Q\x8B` \x01Q`\x01`\x02\x81\x10a\x13\xA3Wa\x13\xA3a7\xB9V[` \x90\x81\x02\x91\x90\x91\x01Q\x8CQ\x8D\x83\x01Q`@Qa\x14\0\x9A\x99\x98\x97\x96\x95\x94\x01\x98\x89R` \x89\x01\x97\x90\x97R`@\x88\x01\x95\x90\x95R``\x87\x01\x93\x90\x93R`\x80\x86\x01\x91\x90\x91R`\xA0\x85\x01R`\xC0\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01Ra\x01 \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\0\x1Ca\x14#\x91\x90a:\xD4V[\x90Pa\x14\x96a\x14<a\x145\x88\x84a \x98V[\x86\x90a!/V[a\x14Da!\xC3V[a\x14\x8Ca\x14}\x85a\x14w`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x01\x82R`\x02\x90\x82\x01R\x90V[\x90a \x98V[a\x14\x86\x8Ca\"\x83V[\x90a!/V[\x88b\x01\xD4\xC0a#\x13V[\x90\x98\x90\x97P\x95PPPPPPV[a\x14\xACa >V[`\x98\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x9B\x80Ta\x14\xDB\x90a7\x84V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x15\x07\x90a7\x84V[\x80\x15a\x15TW\x80`\x1F\x10a\x15)Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15TV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x157W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\xA4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xC8\x91\x90a:oV[a\x15\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x05\x90a:\x8CV[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[a\x16+a >V[a\x165`\0a%7V[V[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01\x81\x90R`\x9B\x80Ta\x16n\x90a7\x84V[\x90P\x90Pa\x16\x8F`@Q\x80`@\x01`@R\x80``\x81R` \x01``\x81RP\x90V[\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16\xA7Wa\x16\xA7a+\xD1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x16\xD0W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P` \x82\x01R\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16\xEEWa\x16\xEEa+\xD1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x17\x17W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x81R\x84QQ`\0\x90`\x01`\x01`@\x1B\x03\x81\x11\x15a\x177Wa\x177a+\xD1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x17`W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80[\x87QQ\x81\x10\x15a\x19\x1BWa\x17\xA9\x88`\0\x01Q\x82\x81Q\x81\x10a\x17\x8AWa\x17\x8Aa7\xB9V[` \x02` \x01\x01Q\x80Q`\0\x90\x81R` \x91\x82\x01Q\x90\x91R`@\x90 \x90V[\x83\x82\x81Q\x81\x10a\x17\xBBWa\x17\xBBa7\xB9V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80\x15a\x18\x99W\x82a\x17\xD8`\x01\x83a:\xF6V[\x81Q\x81\x10a\x17\xE8Wa\x17\xE8a7\xB9V[` \x02` \x01\x01Q`\0\x1C\x83\x82\x81Q\x81\x10a\x18\x05Wa\x18\x05a7\xB9V[` \x02` \x01\x01Q`\0\x1C\x11a\x18\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`M`$\x82\x01R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: nonSignerG1PubkeysForOldSta`d\x82\x01Rl\x1D\x19H\x1B\x9B\xDD\x08\x1C\xDB\xDC\x9D\x19Y`\x9A\x1B`\x84\x82\x01R`\xA4\x01a\x05\x05V[a\x19\x07a\x19\0`\xA0`\0\x86\x85\x81Q\x81\x10a\x18\xB5Wa\x18\xB5a7\xB9V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82R\x81\x01\x91\x90\x91R`@\x01`\0 T\x8AQ\x80Q`\xFF\x90\x92\x16\x91\x85\x90\x81\x10a\x18\xEAWa\x18\xEAa7\xB9V[` \x02` \x01\x01Qa%\x89\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x87\x90a!/V[\x95P\x80a\x19\x13\x81a83V[\x91PPa\x17gV[Pa\x19%\x85a&mV[\x94P`\0[\x84\x81\x10\x15a\x1B\x06W`\x9B\x81\x81Ta\x19@\x90a7\x84V[\x81\x10a\x19NWa\x19Na7\xB9V[\x81T`\x01\x16\x15a\x19mW\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C`\0\x81\x81R`\x9F` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x01T\x90\x82\x01R\x90\x92Pa\x19\xAE\x90\x87\x90a!/V[`\xFF\x83\x16`\0\x90\x81R`\x9D` \x90\x81R`@\x90\x91 T\x90\x86\x01Q\x80Q\x92\x98P`\x01`\x01``\x1B\x03\x90\x91\x16\x91\x83\x90\x81\x10a\x19\xE9Wa\x19\xE9a7\xB9V[`\x01`\x01``\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x82\x01R\x84\x01Q\x80Q\x82\x90\x81\x10a\x1A\x15Wa\x1A\x15a7\xB9V[` \x02` \x01\x01Q\x84`\0\x01Q\x82\x81Q\x81\x10a\x1A3Wa\x1A3a7\xB9V[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPP`\0[\x88QQ\x81\x10\x15a\x1A\xF3W`\x9E`\0\x85\x83\x81Q\x81\x10a\x1AvWa\x1Ava7\xB9V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82R\x81\x81\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 `\xFF\x87\x16\x82R\x90\x92R\x90 T\x85Q\x80Q`\x01`\x01``\x1B\x03\x90\x92\x16\x91\x84\x90\x81\x10a\x1A\xC1Wa\x1A\xC1a7\xB9V[` \x02` \x01\x01\x81\x81Qa\x1A\xD5\x91\x90a;\rV[`\x01`\x01``\x1B\x03\x16\x90RP\x80a\x1A\xEB\x81a83V[\x91PPa\x1AVV[P\x80a\x1A\xFE\x81a83V[\x91PPa\x19*V[P`\0\x80a\x1B\x1E\x8A\x88\x8B` \x01Q\x8C`@\x01Qa\x13\x1AV[\x91P\x91P\x81a\x1B\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: pairing precompile call fai`d\x82\x01Rb\x1B\x19Y`\xEA\x1B`\x84\x82\x01R`\xA4\x01a\x05\x05V[\x80a\x1C\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: signature is invalid\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\x05V[P\x92\x95PPPPPP[\x92\x91PPV[a\x1C,a >V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1C\x91W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x05\x05V[a\x11\xAE\x81a%7V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x1C\xBAWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x1C\xD4WP0;\x15\x80\x15a\x1C\xD4WP`\0T`\xFF\x16`\x01\x14[a\x1D7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x05\x05V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x1DZW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x1De\x85`\0a'\x08V[a\x1Dn\x84a%7V[`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U`\x98\x80T\x92\x85\x16\x92\x90\x91\x16\x91\x90\x91\x17\x90U\x80\x15a\x1D\xE4W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E>W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Eb\x91\x90a:\x08V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1E\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x05\x90a:%V[`fT\x19\x81\x19`fT\x19\x16\x14a\x1F\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\x05V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x13\x0FV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1F\xD5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x05\x05V[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x165W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x05\x05V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra \xB4a)\xA0V[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x80\x82\x01\x84\x90R`\0\x90\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a \xE7Wa \xE9V[\xFE[P\x80a!'W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCB[][\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x05\x05V[PP\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra!Ka)\xBEV[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R\x83Q`@\x80\x84\x01\x91\x90\x91R\x90\x84\x01Q``\x83\x01R`\0\x90\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a \xE7WP\x80a!'W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCBXY\x19\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x05\x05V[a!\xCBa)\xDCV[P`@\x80Q`\x80\x81\x01\x82R\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81\x83\x01\x90\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED``\x83\x01R\x81R\x81Q\x80\x83\x01\x90\x92R\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x81\x01\x91\x90\x91R\x81\x01\x91\x90\x91R\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0\x80\x80a\"\xB3`\0\x80Q` a;N\x839\x81Q\x91R\x86a:\xD4V[\x90P[a\"\xBF\x81a'\xF2V[\x90\x93P\x91P`\0\x80Q` a;N\x839\x81Q\x91R\x82\x83\t\x83\x14\x15a\"\xF9W`@\x80Q\x80\x82\x01\x90\x91R\x90\x81R` \x81\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80Q` a;N\x839\x81Q\x91R`\x01\x82\x08\x90Pa\"\xB6V[`@\x80Q\x80\x82\x01\x82R\x86\x81R` \x80\x82\x01\x86\x90R\x82Q\x80\x84\x01\x90\x93R\x86\x83R\x82\x01\x84\x90R`\0\x91\x82\x91\x90a#Ea*\x01V[`\0[`\x02\x81\x10\x15a%\nW`\0a#^\x82`\x06a8\x14V[\x90P\x84\x82`\x02\x81\x10a#rWa#ra7\xB9V[` \x02\x01QQ\x83a#\x84\x83`\0a;5V[`\x0C\x81\x10a#\x94Wa#\x94a7\xB9V[` \x02\x01R\x84\x82`\x02\x81\x10a#\xABWa#\xABa7\xB9V[` \x02\x01Q` \x01Q\x83\x82`\x01a#\xC2\x91\x90a;5V[`\x0C\x81\x10a#\xD2Wa#\xD2a7\xB9V[` \x02\x01R\x83\x82`\x02\x81\x10a#\xE9Wa#\xE9a7\xB9V[` \x02\x01QQQ\x83a#\xFC\x83`\x02a;5V[`\x0C\x81\x10a$\x0CWa$\x0Ca7\xB9V[` \x02\x01R\x83\x82`\x02\x81\x10a$#Wa$#a7\xB9V[` \x02\x01QQ`\x01` \x02\x01Q\x83a$<\x83`\x03a;5V[`\x0C\x81\x10a$LWa$La7\xB9V[` \x02\x01R\x83\x82`\x02\x81\x10a$cWa$ca7\xB9V[` \x02\x01Q` \x01Q`\0`\x02\x81\x10a$~Wa$~a7\xB9V[` \x02\x01Q\x83a$\x8F\x83`\x04a;5V[`\x0C\x81\x10a$\x9FWa$\x9Fa7\xB9V[` \x02\x01R\x83\x82`\x02\x81\x10a$\xB6Wa$\xB6a7\xB9V[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a$\xD1Wa$\xD1a7\xB9V[` \x02\x01Q\x83a$\xE2\x83`\x05a;5V[`\x0C\x81\x10a$\xF2Wa$\xF2a7\xB9V[` \x02\x01RP\x80a%\x02\x81a83V[\x91PPa#HV[Pa%\x13a* V[`\0` \x82a\x01\x80\x85`\x08\x8C\xFA\x91Q\x91\x9C\x91\x15\x15\x9BP\x90\x99PPPPPPPPPPV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x02\0\x82a\xFF\xFF\x16\x10a%\xE5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Roscalar-too-large`\x80\x1B`D\x82\x01R`d\x01a\x05\x05V[\x81a\xFF\xFF\x16`\x01\x14\x15a%\xF9WP\x81a\x1C\x1EV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01\x81\x90R\x84\x90`\x01\x90[\x81a\xFF\xFF\x16\x86a\xFF\xFF\x16\x10a&bW`\x01a\xFF\xFF\x87\x16`\xFF\x83\x16\x1C\x81\x16\x14\x15a&EWa&B\x84\x84a!/V[\x93P[a&O\x83\x84a!/V[\x92Pb\x01\xFF\xFE`\x01\x92\x83\x1B\x16\x91\x01a&\x15V[P\x91\x95\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q\x15\x80\x15a&\x92WP` \x82\x01Q\x15[\x15a&\xB0WPP`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01`\0\x80Q` a;N\x839\x81Q\x91R\x84` \x01Qa&\xE3\x91\x90a:\xD4V[a&\xFB\x90`\0\x80Q` a;N\x839\x81Q\x91Ra:\xF6V[\x90R\x92\x91PPV[\x91\x90PV[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a')WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a'\xABW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x05\x05V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a'\xEE\x82a\x1FGV[PPV[`\0\x80\x80`\0\x80Q` a;N\x839\x81Q\x91R`\x03`\0\x80Q` a;N\x839\x81Q\x91R\x86`\0\x80Q` a;N\x839\x81Q\x91R\x88\x89\t\t\x08\x90P`\0a(h\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\0\x80Q` a;N\x839\x81Q\x91Ra(tV[\x91\x95\x91\x94P\x90\x92PPPV[`\0\x80a(\x7Fa* V[a(\x87a*>V[` \x80\x82R\x81\x81\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x88\x90R`\x80\x82\x01\x87\x90R`\xA0\x82\x01\x86\x90R\x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82\x80\x15a \xE7WP\x82a)\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FBN254.expMod: call failure\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x05V[PQ\x95\x94PPPPPV[\x82\x80Ta)(\x90a7\x84V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82a)JW`\0\x85Ua)\x90V[\x82`\x1F\x10a)cW\x82\x80\x01`\xFF\x19\x825\x16\x17\x85Ua)\x90V[\x82\x80\x01`\x01\x01\x85U\x82\x15a)\x90W\x91\x82\x01[\x82\x81\x11\x15a)\x90W\x825\x82U\x91` \x01\x91\x90`\x01\x01\x90a)uV[Pa)\x9C\x92\x91Pa*\\V[P\x90V[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80a)\xEFa*qV[\x81R` \x01a)\xFCa*qV[\x90R\x90V[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[[\x80\x82\x11\x15a)\x9CW`\0\x81U`\x01\x01a*]V[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`\0`\xE0\x82\x84\x03\x12\x15a*\xA1W`\0\x80\xFD[P\x91\x90PV[`\0\x80`\0\x80\x84\x86\x03a\x01 \x81\x12\x15a*\xBFW`\0\x80\xFD[\x855`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a*\xD6W`\0\x80\xFD[a*\xE2\x89\x83\x8A\x01a*\x8FV[\x96P`\xC0`\x1F\x19\x84\x01\x12\x15a*\xF6W`\0\x80\xFD[` \x88\x01\x95P`\xE0\x88\x015\x92P\x80\x83\x11\x15a+\x10W`\0\x80\xFD[a+\x1C\x89\x84\x8A\x01a*\x8FV[\x94Pa\x01\0\x88\x015\x92P\x80\x83\x11\x15a+3W`\0\x80\xFD[PP\x85\x01a\x01@\x81\x88\x03\x12\x15a+HW`\0\x80\xFD[\x93\x96\x92\x95P\x90\x93PPV[\x805`\xFF\x81\x16\x81\x14a'\x03W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a+vW`\0\x80\xFD[a+\x7F\x82a+SV[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x11\xAEW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a+\xADW`\0\x80\xFD[\x815a+\x7F\x81a+\x86V[`\0` \x82\x84\x03\x12\x15a+\xCAW`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a,\tWa,\ta+\xD1V[`@R\x90V[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a,\tWa,\ta+\xD1V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a,YWa,Ya+\xD1V[`@R\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a,sW`\0\x80\xFD[a,{a+\xE7V[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a,\xA2W`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a,\xC4Wa,\xC4a+\xD1V[\x80`@RP\x80`@\x84\x01\x85\x81\x11\x15a,\xDBW`\0\x80\xFD[\x84[\x81\x81\x10\x15a&bW\x805\x83R` \x92\x83\x01\x92\x01a,\xDDV[`\0`\x80\x82\x84\x03\x12\x15a-\x07W`\0\x80\xFD[a-\x0Fa+\xE7V[\x90Pa-\x1B\x83\x83a,\x91V[\x81Ra-*\x83`@\x84\x01a,\x91V[` \x82\x01R\x92\x91PPV[`\0\x80`\0\x80a\x01 \x85\x87\x03\x12\x15a-LW`\0\x80\xFD[\x845\x93Pa-]\x86` \x87\x01a,aV[\x92Pa-l\x86``\x87\x01a,\xF5V[\x91Pa-{\x86`\xE0\x87\x01a,aV[\x90P\x92\x95\x91\x94P\x92PV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a-\xB3W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a-\x97V[\x81\x81\x11\x15a-\xC5W`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a-\xEEW`\0\x80\xFD[\x825\x91Pa-\xFE` \x84\x01a+SV[\x90P\x92P\x92\x90PV[`\0`\xE0\x82\x84\x03\x12\x15a.\x19W`\0\x80\xFD[a.!a,\x0FV[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a.:W`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a.NW`\0\x80\xFD[\x815` \x82\x82\x11\x15a.bWa.ba+\xD1V[a.p\x81\x83`\x05\x1B\x01a,1V[\x82\x81R\x81\x81\x01\x93P`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x87\x83\x11\x15a.\x90W`\0\x80\xFD[\x93\x81\x01\x93[\x82\x85\x10\x15a.\xB9Wa.\xA7\x88\x86a,aV[\x84R\x81\x84\x01\x93P`@\x85\x01\x94Pa.\x95V[\x85Ra.\xC7\x87\x87\x83\x01a,\xF5V[\x81\x86\x01RPPPPa.\xDC\x83`\xA0\x84\x01a,aV[`@\x82\x01R\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a.\xFAW`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a/\x17W`\0\x80\xFD[a/#\x85\x82\x86\x01a.\x07V[\x91PP\x92P\x92\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a/fW\x81Q`\x01`\x01``\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a/AV[P\x94\x95\x94PPPPPV[` \x81R`\0\x82Q`@` \x84\x01Ra/\x8D``\x84\x01\x82a/-V[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01Ra/\xAA\x82\x82a/-V[\x95\x94PPPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a/\xC9W`\0\x80\xFD[\x845a/\xD4\x81a+\x86V[\x93P` \x85\x015a/\xE4\x81a+\x86V[\x92P`@\x85\x015a/\xF4\x81a+\x86V[\x91P``\x85\x015a+H\x81a+\x86V[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a'\x03W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a0*W`\0\x80\xFD[a+\x7F\x82a0\x04V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a0JW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a0iW`\0\x80\xFD[\x806\x03\x83\x13\x15a0xW`\0\x80\xFD[\x92P\x92\x90PV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x81R\x815` \x82\x01R`\0a0\xC1` \x84\x01a0\x04V[c\xFF\xFF\xFF\xFF\x80\x82\x16`@\x85\x01R\x80a0\xDB`@\x87\x01a0\x04V[\x16``\x85\x01Ra0\xEE``\x86\x01\x86a03V[\x92P`\xE0`\x80\x86\x01Ra1\x06a\x01\0\x86\x01\x84\x83a0\x7FV[\x92PP\x80a1\x16`\x80\x87\x01a0\x04V[\x16`\xA0\x85\x01RPa1*`\xA0\x85\x01\x85a03V[\x84\x83\x03`\x1F\x19\x01`\xC0\x86\x01Ra1A\x83\x82\x84a0\x7FV[\x92PPPa1Q`\xC0\x85\x01a0\x04V[c\xFF\xFF\xFF\xFF\x81\x16`\xE0\x85\x01RP\x93\x92PPPV[\x80\x15\x15\x81\x14a\x11\xAEW`\0\x80\xFD[\x805a'\x03\x81a1eV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a1\x95W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a1\xB4W`\0\x80\xFD[\x80`\x05\x1B6\x03\x83\x13\x15a0xW`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a/fW`\xFFa1\xE9\x83a+SV[\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a1\xD6V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a2\x13W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a22W`\0\x80\xFD[\x80`\x07\x1B6\x03\x83\x13\x15a0xW`\0\x80\xFD[\x805`\x01`\x01``\x1B\x03\x81\x16\x81\x14a'\x03W`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a/fW`\xFFa2~\x83a+SV[\x16\x87R`\x01`\x01``\x1B\x03a2\x94\x84\x84\x01a2DV[\x16\x83\x88\x01R`@\x82\x81\x015\x90\x88\x01R``\x80\x83\x015\x90\x88\x01R`\x80\x96\x87\x01\x96\x90\x91\x01\x90`\x01\x01a2kV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a2\xD6W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a2\xF5W`\0\x80\xFD[\x80`\x06\x1B6\x03\x83\x13\x15a0xW`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a/fW`\xFFa3*\x83a+SV[\x16\x87R`\x01`\x01``\x1B\x03a3@\x84\x84\x01a2DV[\x16\x87\x84\x01R`@\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01a3\x17V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a3oW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a3\x8EW`\0\x80\xFD[``\x81\x026\x03\x83\x13\x15a0xW`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a/fW`\xFFa3\xC3\x83a+SV[\x16\x87Ra3\xDE\x83\x88\x01\x84\x84\x01\x805\x82R` \x90\x81\x015\x91\x01RV[``\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01a3\xB0V[\x81\x83R`\0`\x01`\x01`\xFB\x1B\x03\x83\x11\x15a4\nW`\0\x80\xFD[\x82`\x05\x1B\x80\x83` \x87\x017`\0\x94\x01` \x01\x93\x84RP\x91\x92\x91PPV[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a/fW`\x01`\x01``\x1B\x03a4P\x83a2DV[\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a47V[\x81\x83R`\0` \x80\x85\x01\x94P\x84\x84`\x05\x1B\x86\x01\x84`\0\x80[\x88\x81\x10\x15a5\x11W\x84\x84\x03\x8AR\x825`~\x19\x896\x03\x01\x81\x12a4\x9BW\x82\x83\xFD[\x88\x01\x805\x85R`\x80a4\xAF\x88\x83\x01\x83a1~V[\x82\x8A\x89\x01Ra4\xC1\x83\x89\x01\x82\x84a1\xC6V[\x92PPP`@a4\xD3\x81\x84\x01\x84a1~V[\x88\x84\x03\x83\x8A\x01Ra4\xE5\x84\x82\x84a4'V[\x93PPPP```\xFFa4\xF9\x82\x85\x01a+SV[\x16\x96\x01\x95\x90\x95RP\x98\x85\x01\x98\x91\x85\x01\x91`\x01\x01a4{V[P\x91\x98\x97PPPPPPPPV[\x81\x83R`\0` \x80\x85\x01\x94P\x84\x84`\x05\x1B\x86\x01\x84`\0\x80[\x88\x81\x10\x15a5\x11W\x84\x84\x03\x8AR\x825`^\x19\x896\x03\x01\x81\x12a5WW\x82\x83\xFD[\x88\x01\x805\x85R``a5k\x88\x83\x01\x83a1~V[\x82\x8A\x89\x01Ra5}\x83\x89\x01\x82\x84a1\xC6V[\x92PPP`@a5\x8F\x81\x84\x01\x84a1~V[\x93P\x87\x83\x03\x82\x89\x01Ra5\xA3\x83\x85\x83a4'V[\x9D\x8A\x01\x9D\x97PPP\x93\x87\x01\x93PP`\x01\x01a57V[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a/fW\x815\x87R`\xFFa5\xE2\x84\x84\x01a+SV[\x16\x87\x84\x01R`@\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01a5\xC9V[` \x81Ra6\x14` \x82\x01a6\x0E\x84a1sV[\x15\x15\x90RV[`\0a6\"` \x84\x01a1sV[\x80\x15\x15`@\x84\x01RPa68`@\x84\x01\x84a1~V[a\x01@\x80``\x86\x01Ra6Pa\x01`\x86\x01\x83\x85a1\xC6V[\x92Pa6_``\x87\x01\x87a1\xFCV[\x92P`\x1F\x19\x80\x87\x86\x03\x01`\x80\x88\x01Ra6y\x85\x85\x84a2[V[\x94Pa6\x88`\x80\x89\x01\x89a2\xBFV[\x94P\x91P\x80\x87\x86\x03\x01`\xA0\x88\x01Ra6\xA1\x85\x85\x84a3\x07V[\x94Pa6\xB0`\xA0\x89\x01\x89a3XV[\x94P\x91P\x80\x87\x86\x03\x01`\xC0\x88\x01Ra6\xC9\x85\x85\x84a3\xA0V[\x94Pa6\xD8`\xC0\x89\x01\x89a1~V[\x94P\x91P\x80\x87\x86\x03\x01`\xE0\x88\x01Ra6\xF1\x85\x85\x84a3\xF1V[\x94Pa7\0`\xE0\x89\x01\x89a1~V[\x94P\x91Pa\x01\0\x81\x88\x87\x03\x01\x81\x89\x01Ra7\x1B\x86\x86\x85a4cV[\x95Pa7)\x81\x8A\x01\x8Aa1~V[\x95P\x92PPa\x01 \x81\x88\x87\x03\x01\x81\x89\x01Ra7E\x86\x86\x85a5\x1FV[\x95Pa7S\x81\x8A\x01\x8Aa2\xBFV[\x95P\x92PP\x80\x87\x86\x03\x01\x83\x88\x01RPa7m\x84\x84\x83a5\xB9V[\x97\x96PPPPPPPV[`\0a\x1C\x1E6\x83a.\x07V[`\x01\x81\x81\x1C\x90\x82\x16\x80a7\x98W`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15a*\xA1WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15a8\x0BWa8\x0Ba7\xCFV[\x02\x94\x93PPPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a8.Wa8.a7\xCFV[P\x02\x90V[`\0`\0\x19\x82\x14\x15a8GWa8Ga7\xCFV[P`\x01\x01\x90V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a8eW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a8\x7FW`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a0xW`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a8\xAEW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a8\xC8W`\0\x80\xFD[` \x01\x91P`\x07\x81\x90\x1B6\x03\x82\x13\x15a0xW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a8\xF2W`\0\x80\xFD[a+\x7F\x82a2DV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a9\x12W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a9,W`\0\x80\xFD[` \x01\x91P`\x06\x81\x90\x1B6\x03\x82\x13\x15a0xW`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a9[W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a9uW`\0\x80\xFD[` \x01\x91P``\x81\x026\x03\x82\x13\x15a0xW`\0\x80\xFD[`\0\x825`~\x19\x836\x03\x01\x81\x12a9\xA2W`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x825`^\x19\x836\x03\x01\x81\x12a9\xA2W`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a9\xD9W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a9\xF3W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a0xW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a:\x1AW`\0\x80\xFD[\x81Qa+\x7F\x81a+\x86V[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15a:\x81W`\0\x80\xFD[\x81Qa+\x7F\x81a1eV[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[`\0\x82a:\xF1WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\0\x82\x82\x10\x15a;\x08Wa;\x08a7\xCFV[P\x03\x90V[`\0`\x01`\x01``\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a;-Wa;-a7\xCFV[\x03\x93\x92PPPV[`\0\x82\x19\x82\x11\x15a;HWa;Ha7\xCFV[P\x01\x90V\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 \x0B\x1C\x11\xD9Z\x92\xCFtv[\xF0X\x87\x0B\x8E\xCEA:\xCF\x88>\xCDZ\xCDf\x82\xF9AL!\x17\xD4dsolcC\0\x08\x0C\x003";
    /// The bytecode of the contract.
    pub static GASPMULTIROLLUPSERVICE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xA9W`\x005`\xE0\x1C\x80cRn>d\x11a\0\xF9W\x80c\x8D\xA5\xCB[\x11a\0\x97W\x80c\xEDZ\x04\xFE\x11a\0qW\x80c\xEDZ\x04\xFE\x14a\x04VW\x80c\xF2\xFD\xE3\x8B\x14a\x04fW\x80c\xF8\xC8v^\x14a\x04yW\x80c\xFA\xBC\x1C\xBC\x14a\x04\x8CW`\0\x80\xFD[\x80c\x8D\xA5\xCB[\x14a\x04\x12W\x80c\xC4\xE1\x91L\x14a\x04#W\x80c\xDF\x03L\xD0\x14a\x04CW`\0\x80\xFD[\x80c\\\x97Z\xBB\x11a\0\xD3W\x80c\\\x97Z\xBB\x14a\x03\xC6W\x80cqP\x18\xA6\x14a\x03\xCEW\x80cz\xD7Ua\x14a\x03\xD6W\x80c\x88o\x11\x95\x14a\x03\xFFW`\0\x80\xFD[\x80cRn>d\x14a\x03wW\x80cY\\jg\x14a\x03\x9BW\x80cZ\xC8j\xB7\x14a\x03\xA3W`\0\x80\xFD[\x80c\x1E-K\xF7\x11a\x01fW\x80cC\r;9\x11a\x01@W\x80cC\r;9\x14a\x02\xBAW\x80cI\x9Do\xB6\x14a\x02\xEFW\x80cJ\xE6\xB2\x03\x14a\x03;W\x80cM\xEA\xBC!\x14a\x03RW`\0\x80\xFD[\x80c\x1E-K\xF7\x14a\x02gW\x80c*\x84\x14\xFD\x14a\x02zW\x80c=\x9F\xB0\x0C\x14a\x02\x8FW`\0\x80\xFD[\x80c\x03R\x03U\x14a\x01\xAEW\x80c\x03\xD0\x97\xD2\x14a\x01\xC3W\x80c\x10\xD6z/\x14a\x02\x04W\x80c\x12FH\xC9\x14a\x02\x17W\x80c\x13d9\xDD\x14a\x02*W\x80c\x17\x1F\x1D[\x14a\x02=W[`\0\x80\xFD[a\x01\xC1a\x01\xBC6`\x04a*\xA7V[a\x04\x9FV[\0[a\x01\xEAa\x01\xD16`\x04a+dV[`\x9F` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T\x82V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xC1a\x02\x126`\x04a+\x9BV[a\x10\xFEV[a\x01\xC1a\x02%6`\x04a+\x9BV[a\x11\xB1V[a\x01\xC1a\x0286`\x04a+\xB8V[a\x11\xDBV[a\x02Pa\x02K6`\x04a-5V[a\x13\x1AV[`@\x80Q\x92\x15\x15\x83R\x90\x15\x15` \x83\x01R\x01a\x01\xFBV[a\x01\xC1a\x02u6`\x04a+\x9BV[a\x14\xA4V[a\x02\x82a\x14\xCEV[`@Qa\x01\xFB\x91\x90a-\x86V[`\x98Ta\x02\xA2\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xFBV[a\x02\xDDa\x02\xC86`\x04a+\xB8V[`\xA0` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x01\xFBV[a\x03#a\x02\xFD6`\x04a-\xDBV[`\x9E` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\x01`\x01``\x1B\x03\x16\x81V[`@Q`\x01`\x01``\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xFBV[a\x03D`\x99T\x81V[`@Q\x90\x81R` \x01a\x01\xFBV[`\x9CTa\x03b\x90c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xFBV[`\x98Ta\x03\x8B\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01\xFBV[a\x01\xC1a\x15\\V[a\x03\x8Ba\x03\xB16`\x04a+dV[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`fTa\x03DV[a\x01\xC1a\x16#V[a\x03#a\x03\xE46`\x04a+dV[`\x9D` R`\0\x90\x81R`@\x90 T`\x01`\x01``\x1B\x03\x16\x81V[`eTa\x02\xA2\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x02\xA2V[a\x046a\x0416`\x04a.\xE7V[a\x167V[`@Qa\x01\xFB\x91\x90a/qV[`\x97Ta\x02\xA2\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x9ATa\x03b\x90c\xFF\xFF\xFF\xFF\x16\x81V[a\x01\xC1a\x04t6`\x04a+\x9BV[a\x1C$V[a\x01\xC1a\x04\x876`\x04a/\xB3V[a\x1C\x9AV[a\x01\xC1a\x04\x9A6`\x04a+\xB8V[a\x1D\xEBV[a\x04\xAF``\x85\x01`@\x86\x01a0\x18V[`\x9ATc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14a\x05\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7Freference block hash mismatch\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[\x83`@Q` \x01a\x05\x1F\x91\x90a0\xA8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x83` \x015\x14a\x05\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FreferenceTaskHash hash mismatch\0`D\x82\x01R`d\x01a\x05\x05V[\x80`@Q` \x01a\x05\x98\x91\x90a5\xFAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x83`@\x015\x14a\x06\0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FoperatorStateInfo hash mismatch\0`D\x82\x01R`d\x01a\x05\x05V[`\x9ATc\xFF\xFF\xFF\xFF\x16\x15a\x06\xD2W`\0a\x06!` \x85\x015a\x041\x85a7xV[`\x9CT\x90\x91Pc\xFF\xFF\xFF\xFF\x16`\0[`\x9B\x80Ta\x06=\x90a7\x84V[\x90P\x81\x10\x15a\x06\xCEW\x81`\xFF\x16\x83` \x01Q\x82\x81Q\x81\x10a\x06`Wa\x06`a7\xB9V[` \x02` \x01\x01Qa\x06r\x91\x90a7\xE5V[`\x01`\x01``\x1B\x03\x16`d\x84`\0\x01Q\x83\x81Q\x81\x10a\x06\x93Wa\x06\x93a7\xB9V[` \x02` \x01\x01Q`\x01`\x01``\x1B\x03\x16a\x06\xAE\x91\x90a8\x14V[\x10\x15a\x06\xBCWPPPa\x10\xF8V[\x80a\x06\xC6\x81a83V[\x91PPa\x060V[PPP[`\0[a\x06\xE2`@\x83\x01\x83a8NV[\x90P\x81\x10\x15a\x07\xA8W`\x9D`\0a\x06\xFC`@\x85\x01\x85a8NV[\x84\x81\x81\x10a\x07\x0CWa\x07\x0Ca7\xB9V[\x90P` \x02\x01` \x81\x01\x90a\x07!\x91\x90a+dV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x90\x81\x01`\0\x90\x81 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U`\x9F\x91a\x07U\x90\x85\x01\x85a8NV[\x84\x81\x81\x10a\x07eWa\x07ea7\xB9V[\x90P` \x02\x01` \x81\x01\x90a\x07z\x91\x90a+dV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x81\x81U`\x01\x01U\x80a\x07\xA0\x81a83V[\x91PPa\x06\xD5V[P`\0[a\x07\xB9``\x83\x01\x83a8\x97V[\x90P\x81\x10\x15a\x08\xF2Wa\x07\xCF``\x83\x01\x83a8\x97V[\x82\x81\x81\x10a\x07\xDFWa\x07\xDFa7\xB9V[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\x07\xF7\x91\x90a8\xE0V[`\x9D`\0a\x08\x08``\x86\x01\x86a8\x97V[\x85\x81\x81\x10a\x08\x18Wa\x08\x18a7\xB9V[a\x08.\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa+dV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\x08l``\x83\x01\x83a8\x97V[\x82\x81\x81\x10a\x08|Wa\x08|a7\xB9V[\x90P`\x80\x02\x01`@\x01`\x9F`\0\x84\x80``\x01\x90a\x08\x99\x91\x90a8\x97V[\x85\x81\x81\x10a\x08\xA9Wa\x08\xA9a7\xB9V[a\x08\xBF\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa+dV[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a\x08\xEA\x81a83V[\x91PPa\x07\xACV[P`\0[a\t\x03`\x80\x83\x01\x83a8\xFBV[\x90P\x81\x10\x15a\t\xBBWa\t\x19`\x80\x83\x01\x83a8\xFBV[\x82\x81\x81\x10a\t)Wa\t)a7\xB9V[\x90P`@\x02\x01` \x01` \x81\x01\x90a\tA\x91\x90a8\xE0V[`\x9D`\0a\tR`\x80\x86\x01\x86a8\xFBV[\x85\x81\x81\x10a\tbWa\tba7\xB9V[a\tx\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa+dV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\t\xB3\x81a83V[\x91PPa\x08\xF6V[P`\0[a\t\xCC`\xA0\x83\x01\x83a9DV[\x90P\x81\x10\x15a\nhWa\t\xE2`\xA0\x83\x01\x83a9DV[\x82\x81\x81\x10a\t\xF2Wa\t\xF2a7\xB9V[\x90P``\x02\x01` \x01`\x9F`\0\x84\x80`\xA0\x01\x90a\n\x0F\x91\x90a9DV[\x85\x81\x81\x10a\n\x1FWa\n\x1Fa7\xB9V[a\n5\x92` ``\x90\x92\x02\x01\x90\x81\x01\x91Pa+dV[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a\n`\x81a83V[\x91PPa\t\xBFV[P`\0[a\ny`\xC0\x83\x01\x83a8NV[\x90P\x81\x10\x15a\x0B\xA0W`\0[`\x9B\x80Ta\n\x92\x90a7\x84V[\x90P\x81\x10\x15a\x0BKW`\x9E`\0a\n\xAC`\xC0\x86\x01\x86a8NV[\x85\x81\x81\x10a\n\xBCWa\n\xBCa7\xB9V[\x90P` \x02\x015\x81R` \x01\x90\x81R` \x01`\0 `\0`\x9B\x83\x81Ta\n\xE1\x90a7\x84V[\x81\x10a\n\xEFWa\n\xEFa7\xB9V[\x81T`\x01\x16\x15a\x0B\x0EW\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U\x81a\x0BC\x81a83V[\x92PPa\n\x85V[P`\xA0`\0a\x0B]`\xC0\x85\x01\x85a8NV[\x84\x81\x81\x10a\x0BmWa\x0Bma7\xB9V[` \x90\x81\x02\x92\x90\x92\x015\x83RP\x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\xFF\x19\x16\x90U\x80a\x0B\x98\x81a83V[\x91PPa\nlV[P`\0[a\x0B\xB1`\xE0\x83\x01\x83a8NV[\x90P\x81\x10\x15a\x0E\x01Wa\x0B\xC7`\xE0\x83\x01\x83a8NV[\x82\x81\x81\x10a\x0B\xD7Wa\x0B\xD7a7\xB9V[\x90P` \x02\x81\x01\x90a\x0B\xE9\x91\x90a9\x8CV[a\x0B\xFA\x90`\x80\x81\x01\x90``\x01a+dV[`\xA0`\0a\x0C\x0B`\xE0\x86\x01\x86a8NV[\x85\x81\x81\x10a\x0C\x1BWa\x0C\x1Ba7\xB9V[\x90P` \x02\x81\x01\x90a\x0C-\x91\x90a9\x8CV[`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP`\0[a\x0Cg`\xE0\x84\x01\x84a8NV[\x83\x81\x81\x10a\x0CwWa\x0Cwa7\xB9V[\x90P` \x02\x81\x01\x90a\x0C\x89\x91\x90a9\x8CV[a\x0C\x97\x90` \x81\x01\x90a8NV[\x90P\x81\x10\x15a\r\xEEWa\x0C\xAD`\xE0\x84\x01\x84a8NV[\x83\x81\x81\x10a\x0C\xBDWa\x0C\xBDa7\xB9V[\x90P` \x02\x81\x01\x90a\x0C\xCF\x91\x90a9\x8CV[a\x0C\xDD\x90`@\x81\x01\x90a8NV[\x82\x81\x81\x10a\x0C\xEDWa\x0C\xEDa7\xB9V[\x90P` \x02\x01` \x81\x01\x90a\r\x02\x91\x90a8\xE0V[`\x9E`\0a\r\x13`\xE0\x87\x01\x87a8NV[\x86\x81\x81\x10a\r#Wa\r#a7\xB9V[\x90P` \x02\x81\x01\x90a\r5\x91\x90a9\x8CV[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a\rV`\xE0\x87\x01\x87a8NV[\x86\x81\x81\x10a\rfWa\rfa7\xB9V[\x90P` \x02\x81\x01\x90a\rx\x91\x90a9\x8CV[a\r\x86\x90` \x81\x01\x90a8NV[\x85\x81\x81\x10a\r\x96Wa\r\x96a7\xB9V[\x90P` \x02\x01` \x81\x01\x90a\r\xAB\x91\x90a+dV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\r\xE6\x81a83V[\x91PPa\x0CZV[P\x80a\r\xF9\x81a83V[\x91PPa\x0B\xA4V[P`\0[a\x0E\x13a\x01\0\x83\x01\x83a8NV[\x90P\x81\x10\x15a\x0F\xCAW`\0[a\x0E-a\x01\0\x84\x01\x84a8NV[\x83\x81\x81\x10a\x0E=Wa\x0E=a7\xB9V[\x90P` \x02\x81\x01\x90a\x0EO\x91\x90a9\xACV[a\x0E]\x90` \x81\x01\x90a8NV[\x90P\x81\x10\x15a\x0F\xB7Wa\x0Eta\x01\0\x84\x01\x84a8NV[\x83\x81\x81\x10a\x0E\x84Wa\x0E\x84a7\xB9V[\x90P` \x02\x81\x01\x90a\x0E\x96\x91\x90a9\xACV[a\x0E\xA4\x90`@\x81\x01\x90a8NV[\x82\x81\x81\x10a\x0E\xB4Wa\x0E\xB4a7\xB9V[\x90P` \x02\x01` \x81\x01\x90a\x0E\xC9\x91\x90a8\xE0V[`\x9E`\0a\x0E\xDBa\x01\0\x87\x01\x87a8NV[\x86\x81\x81\x10a\x0E\xEBWa\x0E\xEBa7\xB9V[\x90P` \x02\x81\x01\x90a\x0E\xFD\x91\x90a9\xACV[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a\x0F\x1Fa\x01\0\x87\x01\x87a8NV[\x86\x81\x81\x10a\x0F/Wa\x0F/a7\xB9V[\x90P` \x02\x81\x01\x90a\x0FA\x91\x90a9\xACV[a\x0FO\x90` \x81\x01\x90a8NV[\x85\x81\x81\x10a\x0F_Wa\x0F_a7\xB9V[\x90P` \x02\x01` \x81\x01\x90a\x0Ft\x91\x90a+dV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\x0F\xAF\x81a83V[\x91PPa\x0E\x1FV[P\x80a\x0F\xC2\x81a83V[\x91PPa\x0E\x05V[P`\0[a\x0F\xDCa\x01 \x83\x01\x83a8\xFBV[\x90P\x81\x10\x15a\x10\x80Wa\x0F\xF3a\x01 \x83\x01\x83a8\xFBV[\x82\x81\x81\x10a\x10\x03Wa\x10\x03a7\xB9V[\x90P`@\x02\x01` \x01` \x81\x01\x90a\x10\x1B\x91\x90a+dV[`\xA0`\0a\x10-a\x01 \x86\x01\x86a8\xFBV[\x85\x81\x81\x10a\x10=Wa\x10=a7\xB9V[\x90P`@\x02\x01`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP\x80\x80a\x10x\x90a83V[\x91PPa\x0F\xCEV[P`\xA0\x83\x015`\x99Ua\x10\x99`@\x85\x01` \x86\x01a0\x18V[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\x10\xC0``\x85\x01\x85a9\xC2V[a\x10\xCC\x91`\x9B\x91a)\x1CV[Pa\x10\xDD`\xA0\x85\x01`\x80\x86\x01a0\x18V[`\x9C\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U[PPPPV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11QW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11u\x91\x90a:\x08V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x11\xA5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x05\x90a:%V[a\x11\xAE\x81a\x1FGV[PV[a\x11\xB9a >V[`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12#W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12G\x91\x90a:oV[a\x12cW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x05\x90a:\x8CV[`fT\x81\x81\x16\x14a\x12\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\x05V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`\0\x80`\0\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x87\x87`\0\x01Q\x88` \x01Q\x88`\0\x01Q`\0`\x02\x81\x10a\x13bWa\x13ba7\xB9V[` \x02\x01Q\x89Q`\x01` \x02\x01Q\x8A` \x01Q`\0`\x02\x81\x10a\x13\x87Wa\x13\x87a7\xB9V[` \x02\x01Q\x8B` \x01Q`\x01`\x02\x81\x10a\x13\xA3Wa\x13\xA3a7\xB9V[` \x90\x81\x02\x91\x90\x91\x01Q\x8CQ\x8D\x83\x01Q`@Qa\x14\0\x9A\x99\x98\x97\x96\x95\x94\x01\x98\x89R` \x89\x01\x97\x90\x97R`@\x88\x01\x95\x90\x95R``\x87\x01\x93\x90\x93R`\x80\x86\x01\x91\x90\x91R`\xA0\x85\x01R`\xC0\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01Ra\x01 \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\0\x1Ca\x14#\x91\x90a:\xD4V[\x90Pa\x14\x96a\x14<a\x145\x88\x84a \x98V[\x86\x90a!/V[a\x14Da!\xC3V[a\x14\x8Ca\x14}\x85a\x14w`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x01\x82R`\x02\x90\x82\x01R\x90V[\x90a \x98V[a\x14\x86\x8Ca\"\x83V[\x90a!/V[\x88b\x01\xD4\xC0a#\x13V[\x90\x98\x90\x97P\x95PPPPPPV[a\x14\xACa >V[`\x98\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x9B\x80Ta\x14\xDB\x90a7\x84V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x15\x07\x90a7\x84V[\x80\x15a\x15TW\x80`\x1F\x10a\x15)Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15TV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x157W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\xA4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xC8\x91\x90a:oV[a\x15\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x05\x90a:\x8CV[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[a\x16+a >V[a\x165`\0a%7V[V[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01\x81\x90R`\x9B\x80Ta\x16n\x90a7\x84V[\x90P\x90Pa\x16\x8F`@Q\x80`@\x01`@R\x80``\x81R` \x01``\x81RP\x90V[\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16\xA7Wa\x16\xA7a+\xD1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x16\xD0W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P` \x82\x01R\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16\xEEWa\x16\xEEa+\xD1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x17\x17W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x81R\x84QQ`\0\x90`\x01`\x01`@\x1B\x03\x81\x11\x15a\x177Wa\x177a+\xD1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x17`W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80[\x87QQ\x81\x10\x15a\x19\x1BWa\x17\xA9\x88`\0\x01Q\x82\x81Q\x81\x10a\x17\x8AWa\x17\x8Aa7\xB9V[` \x02` \x01\x01Q\x80Q`\0\x90\x81R` \x91\x82\x01Q\x90\x91R`@\x90 \x90V[\x83\x82\x81Q\x81\x10a\x17\xBBWa\x17\xBBa7\xB9V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80\x15a\x18\x99W\x82a\x17\xD8`\x01\x83a:\xF6V[\x81Q\x81\x10a\x17\xE8Wa\x17\xE8a7\xB9V[` \x02` \x01\x01Q`\0\x1C\x83\x82\x81Q\x81\x10a\x18\x05Wa\x18\x05a7\xB9V[` \x02` \x01\x01Q`\0\x1C\x11a\x18\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`M`$\x82\x01R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: nonSignerG1PubkeysForOldSta`d\x82\x01Rl\x1D\x19H\x1B\x9B\xDD\x08\x1C\xDB\xDC\x9D\x19Y`\x9A\x1B`\x84\x82\x01R`\xA4\x01a\x05\x05V[a\x19\x07a\x19\0`\xA0`\0\x86\x85\x81Q\x81\x10a\x18\xB5Wa\x18\xB5a7\xB9V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82R\x81\x01\x91\x90\x91R`@\x01`\0 T\x8AQ\x80Q`\xFF\x90\x92\x16\x91\x85\x90\x81\x10a\x18\xEAWa\x18\xEAa7\xB9V[` \x02` \x01\x01Qa%\x89\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x87\x90a!/V[\x95P\x80a\x19\x13\x81a83V[\x91PPa\x17gV[Pa\x19%\x85a&mV[\x94P`\0[\x84\x81\x10\x15a\x1B\x06W`\x9B\x81\x81Ta\x19@\x90a7\x84V[\x81\x10a\x19NWa\x19Na7\xB9V[\x81T`\x01\x16\x15a\x19mW\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C`\0\x81\x81R`\x9F` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x01T\x90\x82\x01R\x90\x92Pa\x19\xAE\x90\x87\x90a!/V[`\xFF\x83\x16`\0\x90\x81R`\x9D` \x90\x81R`@\x90\x91 T\x90\x86\x01Q\x80Q\x92\x98P`\x01`\x01``\x1B\x03\x90\x91\x16\x91\x83\x90\x81\x10a\x19\xE9Wa\x19\xE9a7\xB9V[`\x01`\x01``\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x82\x01R\x84\x01Q\x80Q\x82\x90\x81\x10a\x1A\x15Wa\x1A\x15a7\xB9V[` \x02` \x01\x01Q\x84`\0\x01Q\x82\x81Q\x81\x10a\x1A3Wa\x1A3a7\xB9V[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPP`\0[\x88QQ\x81\x10\x15a\x1A\xF3W`\x9E`\0\x85\x83\x81Q\x81\x10a\x1AvWa\x1Ava7\xB9V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82R\x81\x81\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 `\xFF\x87\x16\x82R\x90\x92R\x90 T\x85Q\x80Q`\x01`\x01``\x1B\x03\x90\x92\x16\x91\x84\x90\x81\x10a\x1A\xC1Wa\x1A\xC1a7\xB9V[` \x02` \x01\x01\x81\x81Qa\x1A\xD5\x91\x90a;\rV[`\x01`\x01``\x1B\x03\x16\x90RP\x80a\x1A\xEB\x81a83V[\x91PPa\x1AVV[P\x80a\x1A\xFE\x81a83V[\x91PPa\x19*V[P`\0\x80a\x1B\x1E\x8A\x88\x8B` \x01Q\x8C`@\x01Qa\x13\x1AV[\x91P\x91P\x81a\x1B\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: pairing precompile call fai`d\x82\x01Rb\x1B\x19Y`\xEA\x1B`\x84\x82\x01R`\xA4\x01a\x05\x05V[\x80a\x1C\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: signature is invalid\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\x05V[P\x92\x95PPPPPP[\x92\x91PPV[a\x1C,a >V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1C\x91W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x05\x05V[a\x11\xAE\x81a%7V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x1C\xBAWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x1C\xD4WP0;\x15\x80\x15a\x1C\xD4WP`\0T`\xFF\x16`\x01\x14[a\x1D7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x05\x05V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x1DZW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x1De\x85`\0a'\x08V[a\x1Dn\x84a%7V[`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U`\x98\x80T\x92\x85\x16\x92\x90\x91\x16\x91\x90\x91\x17\x90U\x80\x15a\x1D\xE4W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E>W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Eb\x91\x90a:\x08V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1E\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x05\x90a:%V[`fT\x19\x81\x19`fT\x19\x16\x14a\x1F\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\x05V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x13\x0FV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1F\xD5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x05\x05V[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x165W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x05\x05V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra \xB4a)\xA0V[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x80\x82\x01\x84\x90R`\0\x90\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a \xE7Wa \xE9V[\xFE[P\x80a!'W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCB[][\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x05\x05V[PP\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra!Ka)\xBEV[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R\x83Q`@\x80\x84\x01\x91\x90\x91R\x90\x84\x01Q``\x83\x01R`\0\x90\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a \xE7WP\x80a!'W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCBXY\x19\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x05\x05V[a!\xCBa)\xDCV[P`@\x80Q`\x80\x81\x01\x82R\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81\x83\x01\x90\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED``\x83\x01R\x81R\x81Q\x80\x83\x01\x90\x92R\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x81\x01\x91\x90\x91R\x81\x01\x91\x90\x91R\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0\x80\x80a\"\xB3`\0\x80Q` a;N\x839\x81Q\x91R\x86a:\xD4V[\x90P[a\"\xBF\x81a'\xF2V[\x90\x93P\x91P`\0\x80Q` a;N\x839\x81Q\x91R\x82\x83\t\x83\x14\x15a\"\xF9W`@\x80Q\x80\x82\x01\x90\x91R\x90\x81R` \x81\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80Q` a;N\x839\x81Q\x91R`\x01\x82\x08\x90Pa\"\xB6V[`@\x80Q\x80\x82\x01\x82R\x86\x81R` \x80\x82\x01\x86\x90R\x82Q\x80\x84\x01\x90\x93R\x86\x83R\x82\x01\x84\x90R`\0\x91\x82\x91\x90a#Ea*\x01V[`\0[`\x02\x81\x10\x15a%\nW`\0a#^\x82`\x06a8\x14V[\x90P\x84\x82`\x02\x81\x10a#rWa#ra7\xB9V[` \x02\x01QQ\x83a#\x84\x83`\0a;5V[`\x0C\x81\x10a#\x94Wa#\x94a7\xB9V[` \x02\x01R\x84\x82`\x02\x81\x10a#\xABWa#\xABa7\xB9V[` \x02\x01Q` \x01Q\x83\x82`\x01a#\xC2\x91\x90a;5V[`\x0C\x81\x10a#\xD2Wa#\xD2a7\xB9V[` \x02\x01R\x83\x82`\x02\x81\x10a#\xE9Wa#\xE9a7\xB9V[` \x02\x01QQQ\x83a#\xFC\x83`\x02a;5V[`\x0C\x81\x10a$\x0CWa$\x0Ca7\xB9V[` \x02\x01R\x83\x82`\x02\x81\x10a$#Wa$#a7\xB9V[` \x02\x01QQ`\x01` \x02\x01Q\x83a$<\x83`\x03a;5V[`\x0C\x81\x10a$LWa$La7\xB9V[` \x02\x01R\x83\x82`\x02\x81\x10a$cWa$ca7\xB9V[` \x02\x01Q` \x01Q`\0`\x02\x81\x10a$~Wa$~a7\xB9V[` \x02\x01Q\x83a$\x8F\x83`\x04a;5V[`\x0C\x81\x10a$\x9FWa$\x9Fa7\xB9V[` \x02\x01R\x83\x82`\x02\x81\x10a$\xB6Wa$\xB6a7\xB9V[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a$\xD1Wa$\xD1a7\xB9V[` \x02\x01Q\x83a$\xE2\x83`\x05a;5V[`\x0C\x81\x10a$\xF2Wa$\xF2a7\xB9V[` \x02\x01RP\x80a%\x02\x81a83V[\x91PPa#HV[Pa%\x13a* V[`\0` \x82a\x01\x80\x85`\x08\x8C\xFA\x91Q\x91\x9C\x91\x15\x15\x9BP\x90\x99PPPPPPPPPPV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x02\0\x82a\xFF\xFF\x16\x10a%\xE5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Roscalar-too-large`\x80\x1B`D\x82\x01R`d\x01a\x05\x05V[\x81a\xFF\xFF\x16`\x01\x14\x15a%\xF9WP\x81a\x1C\x1EV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01\x81\x90R\x84\x90`\x01\x90[\x81a\xFF\xFF\x16\x86a\xFF\xFF\x16\x10a&bW`\x01a\xFF\xFF\x87\x16`\xFF\x83\x16\x1C\x81\x16\x14\x15a&EWa&B\x84\x84a!/V[\x93P[a&O\x83\x84a!/V[\x92Pb\x01\xFF\xFE`\x01\x92\x83\x1B\x16\x91\x01a&\x15V[P\x91\x95\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q\x15\x80\x15a&\x92WP` \x82\x01Q\x15[\x15a&\xB0WPP`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01`\0\x80Q` a;N\x839\x81Q\x91R\x84` \x01Qa&\xE3\x91\x90a:\xD4V[a&\xFB\x90`\0\x80Q` a;N\x839\x81Q\x91Ra:\xF6V[\x90R\x92\x91PPV[\x91\x90PV[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a')WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a'\xABW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x05\x05V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a'\xEE\x82a\x1FGV[PPV[`\0\x80\x80`\0\x80Q` a;N\x839\x81Q\x91R`\x03`\0\x80Q` a;N\x839\x81Q\x91R\x86`\0\x80Q` a;N\x839\x81Q\x91R\x88\x89\t\t\x08\x90P`\0a(h\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\0\x80Q` a;N\x839\x81Q\x91Ra(tV[\x91\x95\x91\x94P\x90\x92PPPV[`\0\x80a(\x7Fa* V[a(\x87a*>V[` \x80\x82R\x81\x81\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x88\x90R`\x80\x82\x01\x87\x90R`\xA0\x82\x01\x86\x90R\x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82\x80\x15a \xE7WP\x82a)\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FBN254.expMod: call failure\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\x05V[PQ\x95\x94PPPPPV[\x82\x80Ta)(\x90a7\x84V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82a)JW`\0\x85Ua)\x90V[\x82`\x1F\x10a)cW\x82\x80\x01`\xFF\x19\x825\x16\x17\x85Ua)\x90V[\x82\x80\x01`\x01\x01\x85U\x82\x15a)\x90W\x91\x82\x01[\x82\x81\x11\x15a)\x90W\x825\x82U\x91` \x01\x91\x90`\x01\x01\x90a)uV[Pa)\x9C\x92\x91Pa*\\V[P\x90V[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80a)\xEFa*qV[\x81R` \x01a)\xFCa*qV[\x90R\x90V[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[[\x80\x82\x11\x15a)\x9CW`\0\x81U`\x01\x01a*]V[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`\0`\xE0\x82\x84\x03\x12\x15a*\xA1W`\0\x80\xFD[P\x91\x90PV[`\0\x80`\0\x80\x84\x86\x03a\x01 \x81\x12\x15a*\xBFW`\0\x80\xFD[\x855`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a*\xD6W`\0\x80\xFD[a*\xE2\x89\x83\x8A\x01a*\x8FV[\x96P`\xC0`\x1F\x19\x84\x01\x12\x15a*\xF6W`\0\x80\xFD[` \x88\x01\x95P`\xE0\x88\x015\x92P\x80\x83\x11\x15a+\x10W`\0\x80\xFD[a+\x1C\x89\x84\x8A\x01a*\x8FV[\x94Pa\x01\0\x88\x015\x92P\x80\x83\x11\x15a+3W`\0\x80\xFD[PP\x85\x01a\x01@\x81\x88\x03\x12\x15a+HW`\0\x80\xFD[\x93\x96\x92\x95P\x90\x93PPV[\x805`\xFF\x81\x16\x81\x14a'\x03W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a+vW`\0\x80\xFD[a+\x7F\x82a+SV[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x11\xAEW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a+\xADW`\0\x80\xFD[\x815a+\x7F\x81a+\x86V[`\0` \x82\x84\x03\x12\x15a+\xCAW`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a,\tWa,\ta+\xD1V[`@R\x90V[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a,\tWa,\ta+\xD1V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a,YWa,Ya+\xD1V[`@R\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a,sW`\0\x80\xFD[a,{a+\xE7V[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a,\xA2W`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a,\xC4Wa,\xC4a+\xD1V[\x80`@RP\x80`@\x84\x01\x85\x81\x11\x15a,\xDBW`\0\x80\xFD[\x84[\x81\x81\x10\x15a&bW\x805\x83R` \x92\x83\x01\x92\x01a,\xDDV[`\0`\x80\x82\x84\x03\x12\x15a-\x07W`\0\x80\xFD[a-\x0Fa+\xE7V[\x90Pa-\x1B\x83\x83a,\x91V[\x81Ra-*\x83`@\x84\x01a,\x91V[` \x82\x01R\x92\x91PPV[`\0\x80`\0\x80a\x01 \x85\x87\x03\x12\x15a-LW`\0\x80\xFD[\x845\x93Pa-]\x86` \x87\x01a,aV[\x92Pa-l\x86``\x87\x01a,\xF5V[\x91Pa-{\x86`\xE0\x87\x01a,aV[\x90P\x92\x95\x91\x94P\x92PV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a-\xB3W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a-\x97V[\x81\x81\x11\x15a-\xC5W`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a-\xEEW`\0\x80\xFD[\x825\x91Pa-\xFE` \x84\x01a+SV[\x90P\x92P\x92\x90PV[`\0`\xE0\x82\x84\x03\x12\x15a.\x19W`\0\x80\xFD[a.!a,\x0FV[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a.:W`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a.NW`\0\x80\xFD[\x815` \x82\x82\x11\x15a.bWa.ba+\xD1V[a.p\x81\x83`\x05\x1B\x01a,1V[\x82\x81R\x81\x81\x01\x93P`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x87\x83\x11\x15a.\x90W`\0\x80\xFD[\x93\x81\x01\x93[\x82\x85\x10\x15a.\xB9Wa.\xA7\x88\x86a,aV[\x84R\x81\x84\x01\x93P`@\x85\x01\x94Pa.\x95V[\x85Ra.\xC7\x87\x87\x83\x01a,\xF5V[\x81\x86\x01RPPPPa.\xDC\x83`\xA0\x84\x01a,aV[`@\x82\x01R\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a.\xFAW`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a/\x17W`\0\x80\xFD[a/#\x85\x82\x86\x01a.\x07V[\x91PP\x92P\x92\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a/fW\x81Q`\x01`\x01``\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a/AV[P\x94\x95\x94PPPPPV[` \x81R`\0\x82Q`@` \x84\x01Ra/\x8D``\x84\x01\x82a/-V[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01Ra/\xAA\x82\x82a/-V[\x95\x94PPPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a/\xC9W`\0\x80\xFD[\x845a/\xD4\x81a+\x86V[\x93P` \x85\x015a/\xE4\x81a+\x86V[\x92P`@\x85\x015a/\xF4\x81a+\x86V[\x91P``\x85\x015a+H\x81a+\x86V[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a'\x03W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a0*W`\0\x80\xFD[a+\x7F\x82a0\x04V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a0JW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a0iW`\0\x80\xFD[\x806\x03\x83\x13\x15a0xW`\0\x80\xFD[\x92P\x92\x90PV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x81R\x815` \x82\x01R`\0a0\xC1` \x84\x01a0\x04V[c\xFF\xFF\xFF\xFF\x80\x82\x16`@\x85\x01R\x80a0\xDB`@\x87\x01a0\x04V[\x16``\x85\x01Ra0\xEE``\x86\x01\x86a03V[\x92P`\xE0`\x80\x86\x01Ra1\x06a\x01\0\x86\x01\x84\x83a0\x7FV[\x92PP\x80a1\x16`\x80\x87\x01a0\x04V[\x16`\xA0\x85\x01RPa1*`\xA0\x85\x01\x85a03V[\x84\x83\x03`\x1F\x19\x01`\xC0\x86\x01Ra1A\x83\x82\x84a0\x7FV[\x92PPPa1Q`\xC0\x85\x01a0\x04V[c\xFF\xFF\xFF\xFF\x81\x16`\xE0\x85\x01RP\x93\x92PPPV[\x80\x15\x15\x81\x14a\x11\xAEW`\0\x80\xFD[\x805a'\x03\x81a1eV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a1\x95W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a1\xB4W`\0\x80\xFD[\x80`\x05\x1B6\x03\x83\x13\x15a0xW`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a/fW`\xFFa1\xE9\x83a+SV[\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a1\xD6V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a2\x13W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a22W`\0\x80\xFD[\x80`\x07\x1B6\x03\x83\x13\x15a0xW`\0\x80\xFD[\x805`\x01`\x01``\x1B\x03\x81\x16\x81\x14a'\x03W`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a/fW`\xFFa2~\x83a+SV[\x16\x87R`\x01`\x01``\x1B\x03a2\x94\x84\x84\x01a2DV[\x16\x83\x88\x01R`@\x82\x81\x015\x90\x88\x01R``\x80\x83\x015\x90\x88\x01R`\x80\x96\x87\x01\x96\x90\x91\x01\x90`\x01\x01a2kV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a2\xD6W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a2\xF5W`\0\x80\xFD[\x80`\x06\x1B6\x03\x83\x13\x15a0xW`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a/fW`\xFFa3*\x83a+SV[\x16\x87R`\x01`\x01``\x1B\x03a3@\x84\x84\x01a2DV[\x16\x87\x84\x01R`@\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01a3\x17V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a3oW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a3\x8EW`\0\x80\xFD[``\x81\x026\x03\x83\x13\x15a0xW`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a/fW`\xFFa3\xC3\x83a+SV[\x16\x87Ra3\xDE\x83\x88\x01\x84\x84\x01\x805\x82R` \x90\x81\x015\x91\x01RV[``\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01a3\xB0V[\x81\x83R`\0`\x01`\x01`\xFB\x1B\x03\x83\x11\x15a4\nW`\0\x80\xFD[\x82`\x05\x1B\x80\x83` \x87\x017`\0\x94\x01` \x01\x93\x84RP\x91\x92\x91PPV[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a/fW`\x01`\x01``\x1B\x03a4P\x83a2DV[\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a47V[\x81\x83R`\0` \x80\x85\x01\x94P\x84\x84`\x05\x1B\x86\x01\x84`\0\x80[\x88\x81\x10\x15a5\x11W\x84\x84\x03\x8AR\x825`~\x19\x896\x03\x01\x81\x12a4\x9BW\x82\x83\xFD[\x88\x01\x805\x85R`\x80a4\xAF\x88\x83\x01\x83a1~V[\x82\x8A\x89\x01Ra4\xC1\x83\x89\x01\x82\x84a1\xC6V[\x92PPP`@a4\xD3\x81\x84\x01\x84a1~V[\x88\x84\x03\x83\x8A\x01Ra4\xE5\x84\x82\x84a4'V[\x93PPPP```\xFFa4\xF9\x82\x85\x01a+SV[\x16\x96\x01\x95\x90\x95RP\x98\x85\x01\x98\x91\x85\x01\x91`\x01\x01a4{V[P\x91\x98\x97PPPPPPPPV[\x81\x83R`\0` \x80\x85\x01\x94P\x84\x84`\x05\x1B\x86\x01\x84`\0\x80[\x88\x81\x10\x15a5\x11W\x84\x84\x03\x8AR\x825`^\x19\x896\x03\x01\x81\x12a5WW\x82\x83\xFD[\x88\x01\x805\x85R``a5k\x88\x83\x01\x83a1~V[\x82\x8A\x89\x01Ra5}\x83\x89\x01\x82\x84a1\xC6V[\x92PPP`@a5\x8F\x81\x84\x01\x84a1~V[\x93P\x87\x83\x03\x82\x89\x01Ra5\xA3\x83\x85\x83a4'V[\x9D\x8A\x01\x9D\x97PPP\x93\x87\x01\x93PP`\x01\x01a57V[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a/fW\x815\x87R`\xFFa5\xE2\x84\x84\x01a+SV[\x16\x87\x84\x01R`@\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01a5\xC9V[` \x81Ra6\x14` \x82\x01a6\x0E\x84a1sV[\x15\x15\x90RV[`\0a6\"` \x84\x01a1sV[\x80\x15\x15`@\x84\x01RPa68`@\x84\x01\x84a1~V[a\x01@\x80``\x86\x01Ra6Pa\x01`\x86\x01\x83\x85a1\xC6V[\x92Pa6_``\x87\x01\x87a1\xFCV[\x92P`\x1F\x19\x80\x87\x86\x03\x01`\x80\x88\x01Ra6y\x85\x85\x84a2[V[\x94Pa6\x88`\x80\x89\x01\x89a2\xBFV[\x94P\x91P\x80\x87\x86\x03\x01`\xA0\x88\x01Ra6\xA1\x85\x85\x84a3\x07V[\x94Pa6\xB0`\xA0\x89\x01\x89a3XV[\x94P\x91P\x80\x87\x86\x03\x01`\xC0\x88\x01Ra6\xC9\x85\x85\x84a3\xA0V[\x94Pa6\xD8`\xC0\x89\x01\x89a1~V[\x94P\x91P\x80\x87\x86\x03\x01`\xE0\x88\x01Ra6\xF1\x85\x85\x84a3\xF1V[\x94Pa7\0`\xE0\x89\x01\x89a1~V[\x94P\x91Pa\x01\0\x81\x88\x87\x03\x01\x81\x89\x01Ra7\x1B\x86\x86\x85a4cV[\x95Pa7)\x81\x8A\x01\x8Aa1~V[\x95P\x92PPa\x01 \x81\x88\x87\x03\x01\x81\x89\x01Ra7E\x86\x86\x85a5\x1FV[\x95Pa7S\x81\x8A\x01\x8Aa2\xBFV[\x95P\x92PP\x80\x87\x86\x03\x01\x83\x88\x01RPa7m\x84\x84\x83a5\xB9V[\x97\x96PPPPPPPV[`\0a\x1C\x1E6\x83a.\x07V[`\x01\x81\x81\x1C\x90\x82\x16\x80a7\x98W`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15a*\xA1WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15a8\x0BWa8\x0Ba7\xCFV[\x02\x94\x93PPPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a8.Wa8.a7\xCFV[P\x02\x90V[`\0`\0\x19\x82\x14\x15a8GWa8Ga7\xCFV[P`\x01\x01\x90V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a8eW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a8\x7FW`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a0xW`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a8\xAEW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a8\xC8W`\0\x80\xFD[` \x01\x91P`\x07\x81\x90\x1B6\x03\x82\x13\x15a0xW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a8\xF2W`\0\x80\xFD[a+\x7F\x82a2DV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a9\x12W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a9,W`\0\x80\xFD[` \x01\x91P`\x06\x81\x90\x1B6\x03\x82\x13\x15a0xW`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a9[W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a9uW`\0\x80\xFD[` \x01\x91P``\x81\x026\x03\x82\x13\x15a0xW`\0\x80\xFD[`\0\x825`~\x19\x836\x03\x01\x81\x12a9\xA2W`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x825`^\x19\x836\x03\x01\x81\x12a9\xA2W`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a9\xD9W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a9\xF3W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a0xW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a:\x1AW`\0\x80\xFD[\x81Qa+\x7F\x81a+\x86V[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15a:\x81W`\0\x80\xFD[\x81Qa+\x7F\x81a1eV[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[`\0\x82a:\xF1WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\0\x82\x82\x10\x15a;\x08Wa;\x08a7\xCFV[P\x03\x90V[`\0`\x01`\x01``\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a;-Wa;-a7\xCFV[\x03\x93\x92PPPV[`\0\x82\x19\x82\x11\x15a;HWa;Ha7\xCFV[P\x01\x90V\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 \x0B\x1C\x11\xD9Z\x92\xCFtv[\xF0X\x87\x0B\x8E\xCEA:\xCF\x88>\xCDZ\xCDf\x82\xF9AL!\x17\xD4dsolcC\0\x08\x0C\x003";
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
        ///Calls the contract's `latestCompletedTaskCreatedBlock` (0xed5a04fe) function
        pub fn latest_completed_task_created_block(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([237, 90, 4, 254], ())
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
        ///Calls the contract's `process_eigen_update` (0x03520355) function
        pub fn process_eigen_update(
            &self,
            task: Task,
            task_response: TaskResponse,
            non_signer_stakes_and_signature_for_old_state: NonSignerStakesAndSignatureForOldState,
            operator_state_info: OperatorStateInfo,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [3, 82, 3, 85],
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
    ///Container type for all input parameters for the `process_eigen_update` function with signature `process_eigen_update((uint256,uint32,uint32,bytes,uint32,bytes,uint32),(uint32,bytes32,bytes32,bytes32,bytes32,bytes32),((uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256)),(bool,bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]))` and selector `0x03520355`
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
        abi = "process_eigen_update((uint256,uint32,uint32,bytes,uint32,bytes,uint32),(uint32,bytes32,bytes32,bytes32,bytes32,bytes32),((uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256)),(bool,bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]))"
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
        LatestCompletedTaskCreatedBlock(LatestCompletedTaskCreatedBlockCall),
        LatestPendingStateHash(LatestPendingStateHashCall),
        OperatorAndQuorumToStakes(OperatorAndQuorumToStakesCall),
        OperatorIdQuorumCount(OperatorIdQuorumCountCall),
        Owner(OwnerCall),
        Pause(PauseCall),
        PauseAll(PauseAllCall),
        PausedWithIndex(PausedWithIndexCall),
        Paused(PausedCall),
        PauserRegistry(PauserRegistryCall),
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
            if let Ok(decoded) = <LatestCompletedTaskCreatedBlockCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LatestCompletedTaskCreatedBlock(decoded));
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
                Self::LatestCompletedTaskCreatedBlock(element) => {
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
                Self::LatestCompletedTaskCreatedBlock(element) => {
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
    impl ::core::convert::From<LatestCompletedTaskCreatedBlockCall>
    for GaspMultiRollupServiceCalls {
        fn from(value: LatestCompletedTaskCreatedBlockCall) -> Self {
            Self::LatestCompletedTaskCreatedBlock(value)
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
