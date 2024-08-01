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
                    ::std::borrow::ToOwned::to_owned("allowNonRootInit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allowNonRootInit"),
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
                                    name: ::std::borrow::ToOwned::to_owned("_allowNonRootInit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                    ::std::borrow::ToOwned::to_owned("lastOpUpdateBlockTimestamp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "lastOpUpdateBlockTimestamp",
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
                    ::std::borrow::ToOwned::to_owned(
                        "latestCompletedOpTaskCreatedBlock",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "latestCompletedOpTaskCreatedBlock",
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
                    ::std::borrow::ToOwned::to_owned("latestCompletedOpTaskNumber"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "latestCompletedOpTaskNumber",
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
                    ::std::borrow::ToOwned::to_owned("process_eigen_op_update"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "process_eigen_op_update",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("task"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
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
                                            "struct IFinalizerTaskManager.OpTask",
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
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IFinalizerTaskManager.OpTaskResponse",
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
                    ::std::borrow::ToOwned::to_owned("process_eigen_rd_update"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "process_eigen_rd_update",
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
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IFinalizerTaskManager.RdTask",
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
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IFinalizerTaskManager.RdTaskResponse",
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
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                                            "struct IFinalizerTaskManager.OpTask",
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pendingStateHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("EigenOpUpdateProcessed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "EigenOpUpdateProcessed",
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
                    ::std::borrow::ToOwned::to_owned("EigenRdUpdateProcessed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "EigenRdUpdateProcessed",
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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[PaN\xA7\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xDAW`\x005`\xE0\x1C\x80co\x0C0\xA4\x11a\x01\x04W\x80c\xD0\x93\x86\x7F\x11a\0\xA2W\x80c\xF2\xFD\xE3\x8B\x11a\0qW\x80c\xF2\xFD\xE3\x8B\x14a\x04\xC2W\x80c\xF8N\x91\xFC\x14a\x04\xD5W\x80c\xFA\xBC\x1C\xBC\x14a\x04\xDEW\x80c\xFE\xCF\x974\x14a\x04\xF1W`\0\x80\xFD[\x80c\xD0\x93\x86\x7F\x14a\x04yW\x80c\xDBS\rc\x14a\x04\x8CW\x80c\xDF\x03L\xD0\x14a\x04\x9FW\x80c\xE2\xA7\xCBf\x14a\x04\xB2W`\0\x80\xFD[\x80c\x88o\x11\x95\x11a\0\xDEW\x80c\x88o\x11\x95\x14a\x04\nW\x80c\x8D\xA5\xCB[\x14a\x045W\x80c\xA1\x1D\xF1L\x14a\x04FW\x80c\xC4\xE1\x91L\x14a\x04YW`\0\x80\xFD[\x80co\x0C0\xA4\x14a\x03\xC2W\x80cqP\x18\xA6\x14a\x03\xD9W\x80cz\xD7Ua\x14a\x03\xE1W`\0\x80\xFD[\x80cC\r;9\x11a\x01|W\x80cRn>d\x11a\x01KW\x80cRn>d\x14a\x03{W\x80cY\\jg\x14a\x03\x8FW\x80cZ\xC8j\xB7\x14a\x03\x97W\x80c\\\x97Z\xBB\x14a\x03\xBAW`\0\x80\xFD[\x80cC\r;9\x14a\x02\xBEW\x80cI\x9Do\xB6\x14a\x02\xF3W\x80cJ\xE6\xB2\x03\x14a\x03?W\x80cM\xEA\xBC!\x14a\x03VW`\0\x80\xFD[\x80c\x12FH\xC9\x11a\x01\xB8W\x80c\x12FH\xC9\x14a\x02YW\x80c\x13d9\xDD\x14a\x02lW\x80c\x17\x1F\x1D[\x14a\x02\x7FW\x80c*\x84\x14\xFD\x14a\x02\xA9W`\0\x80\xFD[\x80c\x03\xD0\x97\xD2\x14a\x01\xDFW\x80c\x0E\xE0\xFD\xBD\x14a\x02 W\x80c\x10\xD6z/\x14a\x02DW[`\0\x80\xFD[a\x02\x06a\x01\xED6`\x04a;wV[`\x9F` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T\x82V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[`\x97Ta\x024\x90`\x01`\xA8\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02\x17V[a\x02Wa\x02R6`\x04a;\xAEV[a\x05\x04V[\0[a\x02Wa\x02g6`\x04a;\xAEV[a\x05\xC0V[a\x02Wa\x02z6`\x04a;\xCBV[a\x05\xEAV[a\x02\x92a\x02\x8D6`\x04a=HV[a\x07)V[`@\x80Q\x92\x15\x15\x83R\x90\x15\x15` \x83\x01R\x01a\x02\x17V[a\x02\xB1a\x08\xB3V[`@Qa\x02\x17\x91\x90a=\x99V[a\x02\xE1a\x02\xCC6`\x04a;\xCBV[`\xA0` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02\x17V[a\x03'a\x03\x016`\x04a=\xEEV[`\x9E` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\x01`\x01``\x1B\x03\x16\x81V[`@Q`\x01`\x01``\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x17V[a\x03H`\x99T\x81V[`@Q\x90\x81R` \x01a\x02\x17V[`\x9CTa\x03f\x90c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\x17V[`\x97Ta\x024\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[a\x02Wa\tAV[a\x024a\x03\xA56`\x04a;wV[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`fTa\x03HV[`\x9ATa\x03f\x90`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x02Wa\n\x08V[a\x03'a\x03\xEF6`\x04a;wV[`\x9D` R`\0\x90\x81R`@\x90 T`\x01`\x01``\x1B\x03\x16\x81V[`eTa\x04\x1D\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x17V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x04\x1DV[a\x02Wa\x04T6`\x04a>2V[a\n\x1CV[a\x04la\x04g6`\x04a?\x9AV[a\r\xC2V[`@Qa\x02\x17\x91\x90a@$V[a\x02Wa\x04\x876`\x04a@yV[a\x13\xAFV[a\x02Wa\x04\x9A6`\x04a@\xE5V[a\x1EnV[`\x97Ta\x04\x1D\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x9ATa\x03f\x90c\xFF\xFF\xFF\xFF\x16\x81V[a\x02Wa\x04\xD06`\x04a;\xAEV[a-\x07V[a\x03H`\x98T\x81V[a\x02Wa\x04\xEC6`\x04a;\xCBV[a-}V[a\x02Wa\x04\xFF6`\x04aA\xA2V[a.\xD9V[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05WW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05{\x91\x90aA\xFEV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x05\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xAB\x90aB\x1BV[`@Q\x80\x91\x03\x90\xFD[a\x05\xBD\x81a0\x1EV[PV[a\x05\xC8a1\x15V[`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x062W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06V\x91\x90aBeV[a\x06rW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xAB\x90aB\x82V[`fT\x81\x81\x16\x14a\x06\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\xABV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`\0\x80`\0\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x87\x87`\0\x01Q\x88` \x01Q\x88`\0\x01Q`\0`\x02\x81\x10a\x07qWa\x07qaB\xCAV[` \x02\x01Q\x89Q`\x01` \x02\x01Q\x8A` \x01Q`\0`\x02\x81\x10a\x07\x96Wa\x07\x96aB\xCAV[` \x02\x01Q\x8B` \x01Q`\x01`\x02\x81\x10a\x07\xB2Wa\x07\xB2aB\xCAV[` \x90\x81\x02\x91\x90\x91\x01Q\x8CQ\x8D\x83\x01Q`@Qa\x08\x0F\x9A\x99\x98\x97\x96\x95\x94\x01\x98\x89R` \x89\x01\x97\x90\x97R`@\x88\x01\x95\x90\x95R``\x87\x01\x93\x90\x93R`\x80\x86\x01\x91\x90\x91R`\xA0\x85\x01R`\xC0\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01Ra\x01 \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\0\x1Ca\x082\x91\x90aB\xE0V[\x90Pa\x08\xA5a\x08Ka\x08D\x88\x84a1oV[\x86\x90a2\x06V[a\x08Sa2\x9AV[a\x08\x9Ba\x08\x8C\x85a\x08\x86`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x01\x82R`\x02\x90\x82\x01R\x90V[\x90a1oV[a\x08\x95\x8Ca3ZV[\x90a2\x06V[\x88b\x01\xD4\xC0a3\xEAV[\x90\x98\x90\x97P\x95PPPPPPV[`\x9B\x80Ta\x08\xC0\x90aC\x02V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08\xEC\x90aC\x02V[\x80\x15a\t9W\x80`\x1F\x10a\t\x0EWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t9V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t\x1CW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\x89W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xAD\x91\x90aBeV[a\t\xC9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xAB\x90aB\x82V[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[a\n\x10a1\x15V[a\n\x1A`\0a6\x0EV[V[`\x97T`\x01`\x01`\xA0\x1B\x03\x163\x14a\nvW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FUpdater must be the caller\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\xABV[`\x9AT`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16a\n\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x13\xDC\x08\x1C\xDD\x18]\x19H\x1D[\x9A[\x9A]`\x8A\x1B`D\x82\x01R`d\x01a\x05\xABV[a\n\xD4`\x80\x84\x01``\x85\x01aCKV[`\x9AT`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14a\x0B5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7Freference block hash mismatch\0\0\0`D\x82\x01R`d\x01a\x05\xABV[\x82`@Q` \x01a\x0BF\x91\x90aC\xDBV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82` \x015\x14a\x0B\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FreferenceTaskHash hash mismatch\0`D\x82\x01R`d\x01a\x05\xABV[a\x0B\xBE``\x84\x01`@\x85\x01aCKV[`\x9ATc\xFF\xFF\xFF\xFF\x91\x82\x16\x91a\x0B\xDE\x91`\x01` \x1B\x90\x04\x16a8@aD\x7FV[c\xFF\xFF\xFF\xFF\x16\x11a\x0C!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x077F\x16\xC6R\x077F\x17FR\x03`\x9C\x1B`D\x82\x01R`d\x01a\x05\xABV[B`\x98Tb\x03\xF4\x80a\x0C3\x91\x90aD\xA7V[\x11a\x0CpW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rlstale state 1`\x98\x1B`D\x82\x01R`d\x01a\x05\xABV[`\0a\x0C\xA6\x83`@Q` \x01a\x0C\x86\x91\x90aD\xBFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x83a\x04g\x90aE\x02V[`\x9CT\x90\x91Pc\xFF\xFF\xFF\xFF\x16`\0[`\x9B\x80Ta\x0C\xC2\x90aC\x02V[\x90P\x81\x10\x15a\rSW\x81`\xFF\x16\x83` \x01Q\x82\x81Q\x81\x10a\x0C\xE5Wa\x0C\xE5aB\xCAV[` \x02` \x01\x01Qa\x0C\xF7\x91\x90aE\x0EV[`\x01`\x01``\x1B\x03\x16`d\x84`\0\x01Q\x83\x81Q\x81\x10a\r\x18Wa\r\x18aB\xCAV[` \x02` \x01\x01Q`\x01`\x01``\x1B\x03\x16a\r3\x91\x90aE=V[\x10\x15a\rAWPPPPPPV[\x80a\rK\x81aE\\V[\x91PPa\x0C\xB5V[P`\x80\x84\x015`\x99U\x7F\xECh\xDB9\x18y\xB0\xF9\xF4 \xD1\xCD\xF3Gj\xFB\xDF\x08Z$b\xBFM+\x11\xDFxFb\x95\xCB\x17a\r\x8A` \x87\x01\x87aCKV[a\r\x9A``\x88\x01`@\x89\x01aCKV[`@\x80Qc\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01[`@Q\x80\x91\x03\x90\xA1PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01\x81\x90R`\x9B\x80Ta\r\xF9\x90aC\x02V[\x90P\x90Pa\x0E\x1A`@Q\x80`@\x01`@R\x80``\x81R` \x01``\x81RP\x90V[\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0E2Wa\x0E2a;\xE4V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E[W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P` \x82\x01R\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0EyWa\x0Eya;\xE4V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E\xA2W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x81R\x84QQ`\0\x90`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0E\xC2Wa\x0E\xC2a;\xE4V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E\xEBW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80[\x87QQ\x81\x10\x15a\x10\xA6Wa\x0F4\x88`\0\x01Q\x82\x81Q\x81\x10a\x0F\x15Wa\x0F\x15aB\xCAV[` \x02` \x01\x01Q\x80Q`\0\x90\x81R` \x91\x82\x01Q\x90\x91R`@\x90 \x90V[\x83\x82\x81Q\x81\x10a\x0FFWa\x0FFaB\xCAV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80\x15a\x10$W\x82a\x0Fc`\x01\x83aEwV[\x81Q\x81\x10a\x0FsWa\x0FsaB\xCAV[` \x02` \x01\x01Q`\0\x1C\x83\x82\x81Q\x81\x10a\x0F\x90Wa\x0F\x90aB\xCAV[` \x02` \x01\x01Q`\0\x1C\x11a\x10$W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`M`$\x82\x01R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: nonSignerG1PubkeysForOldSta`d\x82\x01Rl\x1D\x19H\x1B\x9B\xDD\x08\x1C\xDB\xDC\x9D\x19Y`\x9A\x1B`\x84\x82\x01R`\xA4\x01a\x05\xABV[a\x10\x92a\x10\x8B`\xA0`\0\x86\x85\x81Q\x81\x10a\x10@Wa\x10@aB\xCAV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82R\x81\x01\x91\x90\x91R`@\x01`\0 T\x8AQ\x80Q`\xFF\x90\x92\x16\x91\x85\x90\x81\x10a\x10uWa\x10uaB\xCAV[` \x02` \x01\x01Qa6`\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x87\x90a2\x06V[\x95P\x80a\x10\x9E\x81aE\\V[\x91PPa\x0E\xF2V[Pa\x10\xB0\x85a7DV[\x94P`\0[\x84\x81\x10\x15a\x12\x91W`\x9B\x81\x81Ta\x10\xCB\x90aC\x02V[\x81\x10a\x10\xD9Wa\x10\xD9aB\xCAV[\x81T`\x01\x16\x15a\x10\xF8W\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C`\0\x81\x81R`\x9F` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x01T\x90\x82\x01R\x90\x92Pa\x119\x90\x87\x90a2\x06V[`\xFF\x83\x16`\0\x90\x81R`\x9D` \x90\x81R`@\x90\x91 T\x90\x86\x01Q\x80Q\x92\x98P`\x01`\x01``\x1B\x03\x90\x91\x16\x91\x83\x90\x81\x10a\x11tWa\x11taB\xCAV[`\x01`\x01``\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x82\x01R\x84\x01Q\x80Q\x82\x90\x81\x10a\x11\xA0Wa\x11\xA0aB\xCAV[` \x02` \x01\x01Q\x84`\0\x01Q\x82\x81Q\x81\x10a\x11\xBEWa\x11\xBEaB\xCAV[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPP`\0[\x88QQ\x81\x10\x15a\x12~W`\x9E`\0\x85\x83\x81Q\x81\x10a\x12\x01Wa\x12\x01aB\xCAV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82R\x81\x81\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 `\xFF\x87\x16\x82R\x90\x92R\x90 T\x85Q\x80Q`\x01`\x01``\x1B\x03\x90\x92\x16\x91\x84\x90\x81\x10a\x12LWa\x12LaB\xCAV[` \x02` \x01\x01\x81\x81Qa\x12`\x91\x90aE\x8EV[`\x01`\x01``\x1B\x03\x16\x90RP\x80a\x12v\x81aE\\V[\x91PPa\x11\xE1V[P\x80a\x12\x89\x81aE\\V[\x91PPa\x10\xB5V[P`\0\x80a\x12\xA9\x8A\x88\x8B` \x01Q\x8C`@\x01Qa\x07)V[\x91P\x91P\x81a\x13,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: pairing precompile call fai`d\x82\x01Rb\x1B\x19Y`\xEA\x1B`\x84\x82\x01R`\xA4\x01a\x05\xABV[\x80a\x13\x9FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: signature is invalid\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\xABV[P\x92\x95PPPPPP[\x92\x91PPV[a\x13\xB7a1\x15V[`\0[a\x13\xC7` \x84\x01\x84aE\xB6V[\x90P\x81\x10\x15a\x14\x8CW`\x9D`\0a\x13\xE1` \x86\x01\x86aE\xB6V[\x84\x81\x81\x10a\x13\xF1Wa\x13\xF1aB\xCAV[\x90P` \x02\x01` \x81\x01\x90a\x14\x06\x91\x90a;wV[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0\x90\x81 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U`\x9F\x91a\x149\x90\x86\x01\x86aE\xB6V[\x84\x81\x81\x10a\x14IWa\x14IaB\xCAV[\x90P` \x02\x01` \x81\x01\x90a\x14^\x91\x90a;wV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x81\x81U`\x01\x01U\x80a\x14\x84\x81aE\\V[\x91PPa\x13\xBAV[P`\0[a\x14\x9D`@\x84\x01\x84aE\xFFV[\x90P\x81\x10\x15a\x15\xD8Wa\x14\xB3`@\x84\x01\x84aE\xFFV[\x82\x81\x81\x10a\x14\xC3Wa\x14\xC3aB\xCAV[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\x14\xDB\x91\x90aF_V[`\x9D`\0a\x14\xEC`@\x87\x01\x87aE\xFFV[\x85\x81\x81\x10a\x14\xFCWa\x14\xFCaB\xCAV[a\x15\x12\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa;wV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x90\x81\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x17\x90\x91Ua\x15R\x90\x84\x01\x84aE\xFFV[\x82\x81\x81\x10a\x15bWa\x15baB\xCAV[\x90P`\x80\x02\x01`@\x01`\x9F`\0\x85\x80`@\x01\x90a\x15\x7F\x91\x90aE\xFFV[\x85\x81\x81\x10a\x15\x8FWa\x15\x8FaB\xCAV[a\x15\xA5\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa;wV[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a\x15\xD0\x81aE\\V[\x91PPa\x14\x90V[P`\0[a\x15\xE9``\x84\x01\x84aFzV[\x90P\x81\x10\x15a\x16\xA1Wa\x15\xFF``\x84\x01\x84aFzV[\x82\x81\x81\x10a\x16\x0FWa\x16\x0FaB\xCAV[\x90P`@\x02\x01` \x01` \x81\x01\x90a\x16'\x91\x90aF_V[`\x9D`\0a\x168``\x87\x01\x87aFzV[\x85\x81\x81\x10a\x16HWa\x16HaB\xCAV[a\x16^\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa;wV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\x16\x99\x81aE\\V[\x91PPa\x15\xDCV[P`\0[a\x16\xB2`\x80\x84\x01\x84aF\xC3V[\x90P\x81\x10\x15a\x17NWa\x16\xC8`\x80\x84\x01\x84aF\xC3V[\x82\x81\x81\x10a\x16\xD8Wa\x16\xD8aB\xCAV[\x90P``\x02\x01` \x01`\x9F`\0\x85\x80`\x80\x01\x90a\x16\xF5\x91\x90aF\xC3V[\x85\x81\x81\x10a\x17\x05Wa\x17\x05aB\xCAV[a\x17\x1B\x92` ``\x90\x92\x02\x01\x90\x81\x01\x91Pa;wV[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a\x17F\x81aE\\V[\x91PPa\x16\xA5V[P`\0[a\x17_`\xA0\x84\x01\x84aE\xB6V[\x90P\x81\x10\x15a\x18\x85W`\0[`\x9B\x80Ta\x17x\x90aC\x02V[\x90P\x81\x10\x15a\x181W`\x9E`\0a\x17\x92`\xA0\x87\x01\x87aE\xB6V[\x85\x81\x81\x10a\x17\xA2Wa\x17\xA2aB\xCAV[\x90P` \x02\x015\x81R` \x01\x90\x81R` \x01`\0 `\0`\x9B\x83\x81Ta\x17\xC7\x90aC\x02V[\x81\x10a\x17\xD5Wa\x17\xD5aB\xCAV[\x81T`\x01\x16\x15a\x17\xF4W\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U\x81a\x18)\x81aE\\V[\x92PPa\x17kV[P`\xA0`\0a\x18B\x85\x83\x01\x86aE\xB6V[\x84\x81\x81\x10a\x18RWa\x18RaB\xCAV[` \x90\x81\x02\x92\x90\x92\x015\x83RP\x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\xFF\x19\x16\x90U\x80a\x18}\x81aE\\V[\x91PPa\x17RV[P`\0[a\x18\x96`\xC0\x84\x01\x84aE\xB6V[\x90P\x81\x10\x15a\x1A\xE6Wa\x18\xAC`\xC0\x84\x01\x84aE\xB6V[\x82\x81\x81\x10a\x18\xBCWa\x18\xBCaB\xCAV[\x90P` \x02\x81\x01\x90a\x18\xCE\x91\x90aG\x0BV[a\x18\xDF\x90`\x80\x81\x01\x90``\x01a;wV[`\xA0`\0a\x18\xF0`\xC0\x87\x01\x87aE\xB6V[\x85\x81\x81\x10a\x19\0Wa\x19\0aB\xCAV[\x90P` \x02\x81\x01\x90a\x19\x12\x91\x90aG\x0BV[`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP`\0[a\x19L`\xC0\x85\x01\x85aE\xB6V[\x83\x81\x81\x10a\x19\\Wa\x19\\aB\xCAV[\x90P` \x02\x81\x01\x90a\x19n\x91\x90aG\x0BV[a\x19|\x90` \x81\x01\x90aE\xB6V[\x90P\x81\x10\x15a\x1A\xD3Wa\x19\x92`\xC0\x85\x01\x85aE\xB6V[\x83\x81\x81\x10a\x19\xA2Wa\x19\xA2aB\xCAV[\x90P` \x02\x81\x01\x90a\x19\xB4\x91\x90aG\x0BV[a\x19\xC2\x90`@\x81\x01\x90aE\xB6V[\x82\x81\x81\x10a\x19\xD2Wa\x19\xD2aB\xCAV[\x90P` \x02\x01` \x81\x01\x90a\x19\xE7\x91\x90aF_V[`\x9E`\0a\x19\xF8`\xC0\x88\x01\x88aE\xB6V[\x86\x81\x81\x10a\x1A\x08Wa\x1A\x08aB\xCAV[\x90P` \x02\x81\x01\x90a\x1A\x1A\x91\x90aG\x0BV[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a\x1A;`\xC0\x88\x01\x88aE\xB6V[\x86\x81\x81\x10a\x1AKWa\x1AKaB\xCAV[\x90P` \x02\x81\x01\x90a\x1A]\x91\x90aG\x0BV[a\x1Ak\x90` \x81\x01\x90aE\xB6V[\x85\x81\x81\x10a\x1A{Wa\x1A{aB\xCAV[\x90P` \x02\x01` \x81\x01\x90a\x1A\x90\x91\x90a;wV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\x1A\xCB\x81aE\\V[\x91PPa\x19?V[P\x80a\x1A\xDE\x81aE\\V[\x91PPa\x18\x89V[P`\0[a\x1A\xF7`\xE0\x84\x01\x84aE\xB6V[\x90P\x81\x10\x15a\x1C\xAAW`\0[a\x1B\x10`\xE0\x85\x01\x85aE\xB6V[\x83\x81\x81\x10a\x1B Wa\x1B aB\xCAV[\x90P` \x02\x81\x01\x90a\x1B2\x91\x90aG+V[a\x1B@\x90` \x81\x01\x90aE\xB6V[\x90P\x81\x10\x15a\x1C\x97Wa\x1BV`\xE0\x85\x01\x85aE\xB6V[\x83\x81\x81\x10a\x1BfWa\x1BfaB\xCAV[\x90P` \x02\x81\x01\x90a\x1Bx\x91\x90aG+V[a\x1B\x86\x90`@\x81\x01\x90aE\xB6V[\x82\x81\x81\x10a\x1B\x96Wa\x1B\x96aB\xCAV[\x90P` \x02\x01` \x81\x01\x90a\x1B\xAB\x91\x90aF_V[`\x9E`\0a\x1B\xBC`\xE0\x88\x01\x88aE\xB6V[\x86\x81\x81\x10a\x1B\xCCWa\x1B\xCCaB\xCAV[\x90P` \x02\x81\x01\x90a\x1B\xDE\x91\x90aG+V[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a\x1B\xFF`\xE0\x88\x01\x88aE\xB6V[\x86\x81\x81\x10a\x1C\x0FWa\x1C\x0FaB\xCAV[\x90P` \x02\x81\x01\x90a\x1C!\x91\x90aG+V[a\x1C/\x90` \x81\x01\x90aE\xB6V[\x85\x81\x81\x10a\x1C?Wa\x1C?aB\xCAV[\x90P` \x02\x01` \x81\x01\x90a\x1CT\x91\x90a;wV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\x1C\x8F\x81aE\\V[\x91PPa\x1B\x03V[P\x80a\x1C\xA2\x81aE\\V[\x91PPa\x1A\xEAV[P`\0[a\x1C\xBCa\x01\0\x84\x01\x84aFzV[\x90P\x81\x10\x15a\x1D`Wa\x1C\xD3a\x01\0\x84\x01\x84aFzV[\x82\x81\x81\x10a\x1C\xE3Wa\x1C\xE3aB\xCAV[\x90P`@\x02\x01` \x01` \x81\x01\x90a\x1C\xFB\x91\x90a;wV[`\xA0`\0a\x1D\ra\x01\0\x87\x01\x87aFzV[\x85\x81\x81\x10a\x1D\x1DWa\x1D\x1DaB\xCAV[\x90P`@\x02\x01`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP\x80\x80a\x1DX\x90aE\\V[\x91PPa\x1C\xAEV[P`\x99\x81\x90Ua\x1Ds` \x84\x01\x84aCKV[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\x1D\x9D`@\x84\x01` \x85\x01aCKV[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90UB`\x98Ua\x1D\xD4``\x84\x01\x84aGAV[a\x1D\xE0\x91`\x9B\x91a9\xF3V[Pa\x1D\xF1`\xA0\x84\x01`\x80\x85\x01aCKV[`\x9C\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x7F&Ie\xEBk\xC46\xC6\xC4sC\x1D4\xAFV\xE82\xEC4O\xDF\xD4>\xE6\xAFo\xCEm ^\x84\xAFa\x1E9` \x85\x01\x85aCKV[a\x1EI`@\x86\x01` \x87\x01aCKV[`@\x80Qc\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1PPPV[`\x9AT`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15\x80\x15a\x1F W`\x97T`\x01`\xA8\x1B\x90\x04`\xFF\x16\x15a\x1E\xDEW`\x97T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1E\xD9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04\x17WF\x83`\xDC\x1B`D\x82\x01R`d\x01a\x05\xABV[a\x1FbV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1E\xD9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd \xBA\xBA4\x19`\xD9\x1B`D\x82\x01R`d\x01a\x05\xABV[`\x97T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1FbW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04\x17WF\x83`\xDC\x1B`D\x82\x01R`d\x01a\x05\xABV[\x84`@Q` \x01a\x1Fs\x91\x90aG\x87V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x015\x14a\x1F\xDBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FreferenceTaskHash hash mismatch\0`D\x82\x01R`d\x01a\x05\xABV[\x81`@Q` \x01a\x1F\xEC\x91\x90aL\xBBV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84`@\x015\x14a TW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FoperatorStateInfo hash mismatch\0`D\x82\x01R`d\x01a\x05\xABV[\x80a\"tWa i``\x86\x01`@\x87\x01aCKV[`\x9AT`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14a \xCAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7Freference block mismatch\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\xABV[a \xDA`@\x86\x01` \x87\x01aCKV[`\x9ATc\xFF\xFF\xFF\xFF\x91\x82\x16\x91a \xFA\x91`\x01` \x1B\x90\x04\x16a8@aD\x7FV[c\xFF\xFF\xFF\xFF\x16\x11a!=W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x077F\x16\xC6R\x077F\x17FR\x03`\x9C\x1B`D\x82\x01R`d\x01a\x05\xABV[B`\x98Tb\x03\xF4\x80a!O\x91\x90aD\xA7V[\x11a!\x8CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rlstale state 1`\x98\x1B`D\x82\x01R`d\x01a\x05\xABV[`\0a!\xC2\x85`@Q` \x01a!\xA2\x91\x90aN\"V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x85a\x04g\x90aE\x02V[`\x9CT\x90\x91Pc\xFF\xFF\xFF\xFF\x16`\0[`\x9B\x80Ta!\xDE\x90aC\x02V[\x90P\x81\x10\x15a\"pW\x81`\xFF\x16\x83` \x01Q\x82\x81Q\x81\x10a\"\x01Wa\"\x01aB\xCAV[` \x02` \x01\x01Qa\"\x13\x91\x90aE\x0EV[`\x01`\x01``\x1B\x03\x16`d\x84`\0\x01Q\x83\x81Q\x81\x10a\"4Wa\"4aB\xCAV[` \x02` \x01\x01Q`\x01`\x01``\x1B\x03\x16a\"O\x91\x90aE=V[\x10\x15a\"^WPPPPa-\x01V[\x80a\"h\x81aE\\V[\x91PPa!\xD1V[PPP[`\0[a\"\x84` \x84\x01\x84aE\xB6V[\x90P\x81\x10\x15a#IW`\x9D`\0a\"\x9E` \x86\x01\x86aE\xB6V[\x84\x81\x81\x10a\"\xAEWa\"\xAEaB\xCAV[\x90P` \x02\x01` \x81\x01\x90a\"\xC3\x91\x90a;wV[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0\x90\x81 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U`\x9F\x91a\"\xF6\x90\x86\x01\x86aE\xB6V[\x84\x81\x81\x10a#\x06Wa#\x06aB\xCAV[\x90P` \x02\x01` \x81\x01\x90a#\x1B\x91\x90a;wV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x81\x81U`\x01\x01U\x80a#A\x81aE\\V[\x91PPa\"wV[P`\0[a#Z`@\x84\x01\x84aE\xFFV[\x90P\x81\x10\x15a$\x95Wa#p`@\x84\x01\x84aE\xFFV[\x82\x81\x81\x10a#\x80Wa#\x80aB\xCAV[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a#\x98\x91\x90aF_V[`\x9D`\0a#\xA9`@\x87\x01\x87aE\xFFV[\x85\x81\x81\x10a#\xB9Wa#\xB9aB\xCAV[a#\xCF\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa;wV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x90\x81\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x17\x90\x91Ua$\x0F\x90\x84\x01\x84aE\xFFV[\x82\x81\x81\x10a$\x1FWa$\x1FaB\xCAV[\x90P`\x80\x02\x01`@\x01`\x9F`\0\x85\x80`@\x01\x90a$<\x91\x90aE\xFFV[\x85\x81\x81\x10a$LWa$LaB\xCAV[a$b\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa;wV[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a$\x8D\x81aE\\V[\x91PPa#MV[P`\0[a$\xA6``\x84\x01\x84aFzV[\x90P\x81\x10\x15a%^Wa$\xBC``\x84\x01\x84aFzV[\x82\x81\x81\x10a$\xCCWa$\xCCaB\xCAV[\x90P`@\x02\x01` \x01` \x81\x01\x90a$\xE4\x91\x90aF_V[`\x9D`\0a$\xF5``\x87\x01\x87aFzV[\x85\x81\x81\x10a%\x05Wa%\x05aB\xCAV[a%\x1B\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa;wV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a%V\x81aE\\V[\x91PPa$\x99V[P`\0[a%o`\x80\x84\x01\x84aF\xC3V[\x90P\x81\x10\x15a&\x0BWa%\x85`\x80\x84\x01\x84aF\xC3V[\x82\x81\x81\x10a%\x95Wa%\x95aB\xCAV[\x90P``\x02\x01` \x01`\x9F`\0\x85\x80`\x80\x01\x90a%\xB2\x91\x90aF\xC3V[\x85\x81\x81\x10a%\xC2Wa%\xC2aB\xCAV[a%\xD8\x92` ``\x90\x92\x02\x01\x90\x81\x01\x91Pa;wV[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a&\x03\x81aE\\V[\x91PPa%bV[P`\0[a&\x1C`\xA0\x84\x01\x84aE\xB6V[\x90P\x81\x10\x15a'BW`\0[`\x9B\x80Ta&5\x90aC\x02V[\x90P\x81\x10\x15a&\xEEW`\x9E`\0a&O`\xA0\x87\x01\x87aE\xB6V[\x85\x81\x81\x10a&_Wa&_aB\xCAV[\x90P` \x02\x015\x81R` \x01\x90\x81R` \x01`\0 `\0`\x9B\x83\x81Ta&\x84\x90aC\x02V[\x81\x10a&\x92Wa&\x92aB\xCAV[\x81T`\x01\x16\x15a&\xB1W\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U\x81a&\xE6\x81aE\\V[\x92PPa&(V[P`\xA0`\0a&\xFF\x85\x83\x01\x86aE\xB6V[\x84\x81\x81\x10a'\x0FWa'\x0FaB\xCAV[` \x90\x81\x02\x92\x90\x92\x015\x83RP\x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\xFF\x19\x16\x90U\x80a':\x81aE\\V[\x91PPa&\x0FV[P`\0[a'S`\xC0\x84\x01\x84aE\xB6V[\x90P\x81\x10\x15a)\xA3Wa'i`\xC0\x84\x01\x84aE\xB6V[\x82\x81\x81\x10a'yWa'yaB\xCAV[\x90P` \x02\x81\x01\x90a'\x8B\x91\x90aG\x0BV[a'\x9C\x90`\x80\x81\x01\x90``\x01a;wV[`\xA0`\0a'\xAD`\xC0\x87\x01\x87aE\xB6V[\x85\x81\x81\x10a'\xBDWa'\xBDaB\xCAV[\x90P` \x02\x81\x01\x90a'\xCF\x91\x90aG\x0BV[`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP`\0[a(\t`\xC0\x85\x01\x85aE\xB6V[\x83\x81\x81\x10a(\x19Wa(\x19aB\xCAV[\x90P` \x02\x81\x01\x90a(+\x91\x90aG\x0BV[a(9\x90` \x81\x01\x90aE\xB6V[\x90P\x81\x10\x15a)\x90Wa(O`\xC0\x85\x01\x85aE\xB6V[\x83\x81\x81\x10a(_Wa(_aB\xCAV[\x90P` \x02\x81\x01\x90a(q\x91\x90aG\x0BV[a(\x7F\x90`@\x81\x01\x90aE\xB6V[\x82\x81\x81\x10a(\x8FWa(\x8FaB\xCAV[\x90P` \x02\x01` \x81\x01\x90a(\xA4\x91\x90aF_V[`\x9E`\0a(\xB5`\xC0\x88\x01\x88aE\xB6V[\x86\x81\x81\x10a(\xC5Wa(\xC5aB\xCAV[\x90P` \x02\x81\x01\x90a(\xD7\x91\x90aG\x0BV[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a(\xF8`\xC0\x88\x01\x88aE\xB6V[\x86\x81\x81\x10a)\x08Wa)\x08aB\xCAV[\x90P` \x02\x81\x01\x90a)\x1A\x91\x90aG\x0BV[a)(\x90` \x81\x01\x90aE\xB6V[\x85\x81\x81\x10a)8Wa)8aB\xCAV[\x90P` \x02\x01` \x81\x01\x90a)M\x91\x90a;wV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a)\x88\x81aE\\V[\x91PPa'\xFCV[P\x80a)\x9B\x81aE\\V[\x91PPa'FV[P`\0[a)\xB4`\xE0\x84\x01\x84aE\xB6V[\x90P\x81\x10\x15a+gW`\0[a)\xCD`\xE0\x85\x01\x85aE\xB6V[\x83\x81\x81\x10a)\xDDWa)\xDDaB\xCAV[\x90P` \x02\x81\x01\x90a)\xEF\x91\x90aG+V[a)\xFD\x90` \x81\x01\x90aE\xB6V[\x90P\x81\x10\x15a+TWa*\x13`\xE0\x85\x01\x85aE\xB6V[\x83\x81\x81\x10a*#Wa*#aB\xCAV[\x90P` \x02\x81\x01\x90a*5\x91\x90aG+V[a*C\x90`@\x81\x01\x90aE\xB6V[\x82\x81\x81\x10a*SWa*SaB\xCAV[\x90P` \x02\x01` \x81\x01\x90a*h\x91\x90aF_V[`\x9E`\0a*y`\xE0\x88\x01\x88aE\xB6V[\x86\x81\x81\x10a*\x89Wa*\x89aB\xCAV[\x90P` \x02\x81\x01\x90a*\x9B\x91\x90aG+V[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a*\xBC`\xE0\x88\x01\x88aE\xB6V[\x86\x81\x81\x10a*\xCCWa*\xCCaB\xCAV[\x90P` \x02\x81\x01\x90a*\xDE\x91\x90aG+V[a*\xEC\x90` \x81\x01\x90aE\xB6V[\x85\x81\x81\x10a*\xFCWa*\xFCaB\xCAV[\x90P` \x02\x01` \x81\x01\x90a+\x11\x91\x90a;wV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a+L\x81aE\\V[\x91PPa)\xC0V[P\x80a+_\x81aE\\V[\x91PPa)\xA7V[P`\0[a+ya\x01\0\x84\x01\x84aFzV[\x90P\x81\x10\x15a,\x1DWa+\x90a\x01\0\x84\x01\x84aFzV[\x82\x81\x81\x10a+\xA0Wa+\xA0aB\xCAV[\x90P`@\x02\x01` \x01` \x81\x01\x90a+\xB8\x91\x90a;wV[`\xA0`\0a+\xCAa\x01\0\x87\x01\x87aFzV[\x85\x81\x81\x10a+\xDAWa+\xDAaB\xCAV[\x90P`@\x02\x01`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP\x80\x80a,\x15\x90aE\\V[\x91PPa+kV[Pa,+` \x86\x01\x86aCKV[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua,U`@\x86\x01` \x87\x01aCKV[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90UB`\x98Ua,\x8C``\x86\x01\x86aGAV[a,\x98\x91`\x9B\x91a9\xF3V[Pa,\xA9`\xA0\x86\x01`\x80\x87\x01aCKV[`\x9C\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x7F6\xA1\xFD{\xD5T\xF5\xC4(\xC9\x82\x9C\t\xC6`kL\x89;\x1F\xAD\xC8sZz\x12yW\x97D}\xEDa,\xF1` \x87\x01\x87aCKV[a\r\x9A`@\x88\x01` \x89\x01aCKV[PPPPV[a-\x0Fa1\x15V[`\x01`\x01`\xA0\x1B\x03\x81\x16a-tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x05\xABV[a\x05\xBD\x81a6\x0EV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-\xD0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-\xF4\x91\x90aA\xFEV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a.$W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xAB\x90aB\x1BV[`fT\x19\x81\x19`fT\x19\x16\x14a.\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\xABV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x07\x1EV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a.\xF9WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a/\x13WP0;\x15\x80\x15a/\x13WP`\0T`\xFF\x16`\x01\x14[a/vW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x05\xABV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a/\x99W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a/\xA4\x85`\0a7\xDFV[a/\xAD\x84a6\x0EV[`\x97\x80T\x83\x15\x15`\x01`\xA8\x1B\x02`\x01a\xFF\x01`\xA0\x1B\x03\x19\x90\x91\x16`\x01`\x01`\xA0\x1B\x03\x86\x16\x17\x17\x90U\x80\x15a0\x17W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01a\r\xB3V[PPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16a0\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x05\xABV[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\n\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x05\xABV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra1\x8Ba:wV[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x80\x82\x01\x84\x90R`\0\x90\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a1\xBEWa1\xC0V[\xFE[P\x80a1\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCB[][\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x05\xABV[PP\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra2\"a:\x95V[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R\x83Q`@\x80\x84\x01\x91\x90\x91R\x90\x84\x01Q``\x83\x01R`\0\x90\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a1\xBEWP\x80a1\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCBXY\x19\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x05\xABV[a2\xA2a:\xB3V[P`@\x80Q`\x80\x81\x01\x82R\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81\x83\x01\x90\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED``\x83\x01R\x81R\x81Q\x80\x83\x01\x90\x92R\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x81\x01\x91\x90\x91R\x81\x01\x91\x90\x91R\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0\x80\x80a3\x8A`\0\x80Q` aNR\x839\x81Q\x91R\x86aB\xE0V[\x90P[a3\x96\x81a8\xC9V[\x90\x93P\x91P`\0\x80Q` aNR\x839\x81Q\x91R\x82\x83\t\x83\x14\x15a3\xD0W`@\x80Q\x80\x82\x01\x90\x91R\x90\x81R` \x81\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80Q` aNR\x839\x81Q\x91R`\x01\x82\x08\x90Pa3\x8DV[`@\x80Q\x80\x82\x01\x82R\x86\x81R` \x80\x82\x01\x86\x90R\x82Q\x80\x84\x01\x90\x93R\x86\x83R\x82\x01\x84\x90R`\0\x91\x82\x91\x90a4\x1Ca:\xD8V[`\0[`\x02\x81\x10\x15a5\xE1W`\0a45\x82`\x06aE=V[\x90P\x84\x82`\x02\x81\x10a4IWa4IaB\xCAV[` \x02\x01QQ\x83a4[\x83`\0aD\xA7V[`\x0C\x81\x10a4kWa4kaB\xCAV[` \x02\x01R\x84\x82`\x02\x81\x10a4\x82Wa4\x82aB\xCAV[` \x02\x01Q` \x01Q\x83\x82`\x01a4\x99\x91\x90aD\xA7V[`\x0C\x81\x10a4\xA9Wa4\xA9aB\xCAV[` \x02\x01R\x83\x82`\x02\x81\x10a4\xC0Wa4\xC0aB\xCAV[` \x02\x01QQQ\x83a4\xD3\x83`\x02aD\xA7V[`\x0C\x81\x10a4\xE3Wa4\xE3aB\xCAV[` \x02\x01R\x83\x82`\x02\x81\x10a4\xFAWa4\xFAaB\xCAV[` \x02\x01QQ`\x01` \x02\x01Q\x83a5\x13\x83`\x03aD\xA7V[`\x0C\x81\x10a5#Wa5#aB\xCAV[` \x02\x01R\x83\x82`\x02\x81\x10a5:Wa5:aB\xCAV[` \x02\x01Q` \x01Q`\0`\x02\x81\x10a5UWa5UaB\xCAV[` \x02\x01Q\x83a5f\x83`\x04aD\xA7V[`\x0C\x81\x10a5vWa5vaB\xCAV[` \x02\x01R\x83\x82`\x02\x81\x10a5\x8DWa5\x8DaB\xCAV[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a5\xA8Wa5\xA8aB\xCAV[` \x02\x01Q\x83a5\xB9\x83`\x05aD\xA7V[`\x0C\x81\x10a5\xC9Wa5\xC9aB\xCAV[` \x02\x01RP\x80a5\xD9\x81aE\\V[\x91PPa4\x1FV[Pa5\xEAa:\xF7V[`\0` \x82a\x01\x80\x85`\x08\x8C\xFA\x91Q\x91\x9C\x91\x15\x15\x9BP\x90\x99PPPPPPPPPPV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x02\0\x82a\xFF\xFF\x16\x10a6\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Roscalar-too-large`\x80\x1B`D\x82\x01R`d\x01a\x05\xABV[\x81a\xFF\xFF\x16`\x01\x14\x15a6\xD0WP\x81a\x13\xA9V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01\x81\x90R\x84\x90`\x01\x90[\x81a\xFF\xFF\x16\x86a\xFF\xFF\x16\x10a79W`\x01a\xFF\xFF\x87\x16`\xFF\x83\x16\x1C\x81\x16\x14\x15a7\x1CWa7\x19\x84\x84a2\x06V[\x93P[a7&\x83\x84a2\x06V[\x92Pb\x01\xFF\xFE`\x01\x92\x83\x1B\x16\x91\x01a6\xECV[P\x91\x95\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q\x15\x80\x15a7iWP` \x82\x01Q\x15[\x15a7\x87WPP`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01`\0\x80Q` aNR\x839\x81Q\x91R\x84` \x01Qa7\xBA\x91\x90aB\xE0V[a7\xD2\x90`\0\x80Q` aNR\x839\x81Q\x91RaEwV[\x90R\x92\x91PPV[\x91\x90PV[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a8\0WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a8\x82W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x05\xABV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a8\xC5\x82a0\x1EV[PPV[`\0\x80\x80`\0\x80Q` aNR\x839\x81Q\x91R`\x03`\0\x80Q` aNR\x839\x81Q\x91R\x86`\0\x80Q` aNR\x839\x81Q\x91R\x88\x89\t\t\x08\x90P`\0a9?\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\0\x80Q` aNR\x839\x81Q\x91Ra9KV[\x91\x95\x91\x94P\x90\x92PPPV[`\0\x80a9Va:\xF7V[a9^a;\x15V[` \x80\x82R\x81\x81\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x88\x90R`\x80\x82\x01\x87\x90R`\xA0\x82\x01\x86\x90R\x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82\x80\x15a1\xBEWP\x82a9\xE8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FBN254.expMod: call failure\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\xABV[PQ\x95\x94PPPPPV[\x82\x80Ta9\xFF\x90aC\x02V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82a:!W`\0\x85Ua:gV[\x82`\x1F\x10a::W\x82\x80\x01`\xFF\x19\x825\x16\x17\x85Ua:gV[\x82\x80\x01`\x01\x01\x85U\x82\x15a:gW\x91\x82\x01[\x82\x81\x11\x15a:gW\x825\x82U\x91` \x01\x91\x90`\x01\x01\x90a:LV[Pa:s\x92\x91Pa;3V[P\x90V[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80a:\xC6a;HV[\x81R` \x01a:\xD3a;HV[\x90R\x90V[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[[\x80\x82\x11\x15a:sW`\0\x81U`\x01\x01a;4V[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[\x805`\xFF\x81\x16\x81\x14a7\xDAW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a;\x89W`\0\x80\xFD[a;\x92\x82a;fV[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05\xBDW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a;\xC0W`\0\x80\xFD[\x815a;\x92\x81a;\x99V[`\0` \x82\x84\x03\x12\x15a;\xDDW`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a<\x1CWa<\x1Ca;\xE4V[`@R\x90V[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a<\x1CWa<\x1Ca;\xE4V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a<lWa<la;\xE4V[`@R\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a<\x86W`\0\x80\xFD[a<\x8Ea;\xFAV[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a<\xB5W`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a<\xD7Wa<\xD7a;\xE4V[\x80`@RP\x80`@\x84\x01\x85\x81\x11\x15a<\xEEW`\0\x80\xFD[\x84[\x81\x81\x10\x15a79W\x805\x83R` \x92\x83\x01\x92\x01a<\xF0V[`\0`\x80\x82\x84\x03\x12\x15a=\x1AW`\0\x80\xFD[a=\"a;\xFAV[\x90Pa=.\x83\x83a<\xA4V[\x81Ra==\x83`@\x84\x01a<\xA4V[` \x82\x01R\x92\x91PPV[`\0\x80`\0\x80a\x01 \x85\x87\x03\x12\x15a=_W`\0\x80\xFD[\x845\x93Pa=p\x86` \x87\x01a<tV[\x92Pa=\x7F\x86``\x87\x01a=\x08V[\x91Pa=\x8E\x86`\xE0\x87\x01a<tV[\x90P\x92\x95\x91\x94P\x92PV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a=\xC6W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a=\xAAV[\x81\x81\x11\x15a=\xD8W`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a>\x01W`\0\x80\xFD[\x825\x91Pa>\x11` \x84\x01a;fV[\x90P\x92P\x92\x90PV[`\0`\xE0\x82\x84\x03\x12\x15a>,W`\0\x80\xFD[P\x91\x90PV[`\0\x80`\0\x83\x85\x03`\xE0\x81\x12\x15a>HW`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a>_W`\0\x80\xFD[\x90\x86\x01\x90`\xC0\x82\x89\x03\x12\x15a>sW`\0\x80\xFD[\x81\x95P`\xA0`\x1F\x19\x84\x01\x12\x15a>\x88W`\0\x80\xFD[` \x87\x01\x94P`\xC0\x87\x015\x92P\x80\x83\x11\x15a>\xA2W`\0\x80\xFD[PPa>\xB0\x86\x82\x87\x01a>\x1AV[\x91PP\x92P\x92P\x92V[`\0`\xE0\x82\x84\x03\x12\x15a>\xCCW`\0\x80\xFD[a>\xD4a<\"V[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a>\xEDW`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a?\x01W`\0\x80\xFD[\x815` \x82\x82\x11\x15a?\x15Wa?\x15a;\xE4V[a?#\x81\x83`\x05\x1B\x01a<DV[\x82\x81R\x81\x81\x01\x93P`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x87\x83\x11\x15a?CW`\0\x80\xFD[\x93\x81\x01\x93[\x82\x85\x10\x15a?lWa?Z\x88\x86a<tV[\x84R\x81\x84\x01\x93P`@\x85\x01\x94Pa?HV[\x85Ra?z\x87\x87\x83\x01a=\x08V[\x81\x86\x01RPPPPa?\x8F\x83`\xA0\x84\x01a<tV[`@\x82\x01R\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a?\xADW`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a?\xCAW`\0\x80\xFD[a?\xD6\x85\x82\x86\x01a>\xBAV[\x91PP\x92P\x92\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a@\x19W\x81Q`\x01`\x01``\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a?\xF4V[P\x94\x95\x94PPPPPV[` \x81R`\0\x82Q`@` \x84\x01Ra@@``\x84\x01\x82a?\xE0V[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01Ra@]\x82\x82a?\xE0V[\x95\x94PPPPPV[`\0a\x01 \x82\x84\x03\x12\x15a>,W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a@\x8EW`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a@\xA5W`\0\x80\xFD[a@\xB1\x87\x83\x88\x01a>\x1AV[\x94P` \x86\x015\x91P\x80\x82\x11\x15a@\xC7W`\0\x80\xFD[Pa@\xD4\x86\x82\x87\x01a@fV[\x92PP`@\x84\x015\x90P\x92P\x92P\x92V[`\0\x80`\0\x80\x84\x86\x03`\xC0\x81\x12\x15a@\xFCW`\0\x80\xFD[\x855`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aA\x13W`\0\x80\xFD[aA\x1F\x89\x83\x8A\x01a>\x1AV[\x96P```\x1F\x19\x84\x01\x12\x15aA3W`\0\x80\xFD[` \x88\x01\x95P`\x80\x88\x015\x92P\x80\x83\x11\x15aAMW`\0\x80\xFD[aAY\x89\x84\x8A\x01a>\x1AV[\x94P`\xA0\x88\x015\x92P\x80\x83\x11\x15aAoW`\0\x80\xFD[PPaA}\x87\x82\x88\x01a@fV[\x91PP\x92\x95\x91\x94P\x92PV[\x80\x15\x15\x81\x14a\x05\xBDW`\0\x80\xFD[\x805a7\xDA\x81aA\x89V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aA\xB8W`\0\x80\xFD[\x845aA\xC3\x81a;\x99V[\x93P` \x85\x015aA\xD3\x81a;\x99V[\x92P`@\x85\x015aA\xE3\x81a;\x99V[\x91P``\x85\x015aA\xF3\x81aA\x89V[\x93\x96\x92\x95P\x90\x93PPV[`\0` \x82\x84\x03\x12\x15aB\x10W`\0\x80\xFD[\x81Qa;\x92\x81a;\x99V[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15aBwW`\0\x80\xFD[\x81Qa;\x92\x81aA\x89V[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x82aB\xFDWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80aC\x16W`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15a>,WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a7\xDAW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aC]W`\0\x80\xFD[a;\x92\x82aC7V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aC}W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aC\x9CW`\0\x80\xFD[\x806\x03\x83\x13\x15aC\xABW`\0\x80\xFD[\x92P\x92\x90PV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x81R`\0c\xFF\xFF\xFF\xFF\x80aC\xF0\x85aC7V[\x16` \x84\x01R` \x84\x015`@\x84\x01R\x80aD\r`@\x86\x01aC7V[\x16``\x84\x01R\x80aD ``\x86\x01aC7V[\x16`\x80\x84\x01RaD3`\x80\x85\x01\x85aCfV[`\xC0`\xA0\x86\x01RaDH`\xE0\x86\x01\x82\x84aC\xB2V[\x91PP\x81aDX`\xA0\x87\x01aC7V[\x16`\xC0\x85\x01R\x80\x92PPP\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aD\x9EWaD\x9EaDiV[\x01\x94\x93PPPPV[`\0\x82\x19\x82\x11\x15aD\xBAWaD\xBAaDiV[P\x01\x90V[`\xA0\x81\x01c\xFF\xFF\xFF\xFFaD\xD1\x84aC7V[\x16\x82R` \x83\x015` \x83\x01R`@\x83\x015`@\x83\x01R``\x83\x015``\x83\x01R`\x80\x83\x015`\x80\x83\x01R\x92\x91PPV[`\0a\x13\xA96\x83a>\xBAV[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aE4WaE4aDiV[\x02\x94\x93PPPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15aEWWaEWaDiV[P\x02\x90V[`\0`\0\x19\x82\x14\x15aEpWaEpaDiV[P`\x01\x01\x90V[`\0\x82\x82\x10\x15aE\x89WaE\x89aDiV[P\x03\x90V[`\0`\x01`\x01``\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15aE\xAEWaE\xAEaDiV[\x03\x93\x92PPPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aE\xCDW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aE\xE7W`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15aC\xABW`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aF\x16W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aF0W`\0\x80\xFD[` \x01\x91P`\x07\x81\x90\x1B6\x03\x82\x13\x15aC\xABW`\0\x80\xFD[\x805`\x01`\x01``\x1B\x03\x81\x16\x81\x14a7\xDAW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aFqW`\0\x80\xFD[a;\x92\x82aFHV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aF\x91W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aF\xABW`\0\x80\xFD[` \x01\x91P`\x06\x81\x90\x1B6\x03\x82\x13\x15aC\xABW`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aF\xDAW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aF\xF4W`\0\x80\xFD[` \x01\x91P``\x81\x026\x03\x82\x13\x15aC\xABW`\0\x80\xFD[`\0\x825`~\x19\x836\x03\x01\x81\x12aG!W`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x825`^\x19\x836\x03\x01\x81\x12aG!W`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aGXW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aGrW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15aC\xABW`\0\x80\xFD[` \x81R`\0c\xFF\xFF\xFF\xFF\x80aG\x9C\x85aC7V[\x16` \x84\x01R\x80aG\xAF` \x86\x01aC7V[\x16`@\x84\x01R\x80aG\xC2`@\x86\x01aC7V[\x16``\x84\x01RPaG\xD6``\x84\x01\x84aCfV[`\xE0`\x80\x85\x01RaG\xECa\x01\0\x85\x01\x82\x84aC\xB2V[\x91PPaG\xFB`\x80\x85\x01aC7V[c\xFF\xFF\xFF\xFF\x81\x16`\xA0\x85\x01RPaH\x15`\xA0\x85\x01\x85aCfV[\x84\x83\x03`\x1F\x19\x01`\xC0\x86\x01RaH,\x83\x82\x84aC\xB2V[\x92PPPaH<`\xC0\x85\x01aC7V[c\xFF\xFF\xFF\xFF\x81\x16`\xE0\x85\x01RP\x93\x92PPPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aHgW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aH\x86W`\0\x80\xFD[\x80`\x05\x1B6\x03\x83\x13\x15aC\xABW`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a@\x19W`\xFFaH\xBB\x83a;fV[\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aH\xA8V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aH\xE5W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aI\x04W`\0\x80\xFD[\x80`\x07\x1B6\x03\x83\x13\x15aC\xABW`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a@\x19W`\xFFaI9\x83a;fV[\x16\x87R`\x01`\x01``\x1B\x03aIO\x84\x84\x01aFHV[\x16\x83\x88\x01R`@\x82\x81\x015\x90\x88\x01R``\x80\x83\x015\x90\x88\x01R`\x80\x96\x87\x01\x96\x90\x91\x01\x90`\x01\x01aI&V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aI\x91W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aI\xB0W`\0\x80\xFD[\x80`\x06\x1B6\x03\x83\x13\x15aC\xABW`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a@\x19W`\xFFaI\xE5\x83a;fV[\x16\x87R`\x01`\x01``\x1B\x03aI\xFB\x84\x84\x01aFHV[\x16\x87\x84\x01R`@\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aI\xD2V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aJ*W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aJIW`\0\x80\xFD[``\x81\x026\x03\x83\x13\x15aC\xABW`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a@\x19W`\xFFaJ~\x83a;fV[\x16\x87RaJ\x99\x83\x88\x01\x84\x84\x01\x805\x82R` \x90\x81\x015\x91\x01RV[``\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aJkV[\x81\x83R`\0`\x01`\x01`\xFB\x1B\x03\x83\x11\x15aJ\xC5W`\0\x80\xFD[\x82`\x05\x1B\x80\x83` \x87\x017`\0\x94\x01` \x01\x93\x84RP\x91\x92\x91PPV[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a@\x19W`\x01`\x01``\x1B\x03aK\x0B\x83aFHV[\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aJ\xF2V[\x81\x83R`\0` \x80\x85\x01\x80\x81\x96P\x85`\x05\x1B\x81\x01\x91P\x84`\0\x80[\x88\x81\x10\x15aK\xCFW\x83\x85\x03\x8AR\x825`~\x19\x896\x03\x01\x81\x12aKYW\x82\x83\xFD[\x88\x01\x805\x86R`\x80aKm\x88\x83\x01\x83aHPV[\x82\x8A\x8A\x01RaK\x7F\x83\x8A\x01\x82\x84aH\x98V[\x92PPP`@aK\x91\x81\x84\x01\x84aHPV[\x89\x84\x03\x83\x8B\x01RaK\xA3\x84\x82\x84aJ\xE2V[\x93PPPP```\xFFaK\xB7\x82\x85\x01a;fV[\x16\x97\x01\x96\x90\x96RP\x98\x85\x01\x98\x91\x85\x01\x91`\x01\x01aK9V[P\x92\x98\x97PPPPPPPPV[\x81\x83R`\0` \x80\x85\x01\x80\x81\x96P\x85`\x05\x1B\x81\x01\x91P\x84`\0\x80[\x88\x81\x10\x15aK\xCFW\x83\x85\x03\x8AR\x825`^\x19\x896\x03\x01\x81\x12aL\x18W\x82\x83\xFD[\x88\x01\x805\x86R``aL,\x88\x83\x01\x83aHPV[\x82\x8A\x8A\x01RaL>\x83\x8A\x01\x82\x84aH\x98V[\x92PPP`@aLP\x81\x84\x01\x84aHPV[\x93P\x88\x83\x03\x82\x8A\x01RaLd\x83\x85\x83aJ\xE2V[\x9D\x8A\x01\x9D\x98PPP\x93\x87\x01\x93PP`\x01\x01aK\xF8V[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a@\x19W\x815\x87R`\xFFaL\xA3\x84\x84\x01a;fV[\x16\x87\x84\x01R`@\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aL\x8AV[` \x81RaL\xD5` \x82\x01aL\xCF\x84aA\x97V[\x15\x15\x90RV[`\0aL\xE4` \x84\x01\x84aHPV[a\x01 \x80`@\x86\x01RaL\xFCa\x01@\x86\x01\x83\x85aH\x98V[\x92PaM\x0B`@\x87\x01\x87aH\xCEV[\x92P`\x1F\x19\x80\x87\x86\x03\x01``\x88\x01RaM%\x85\x85\x84aI\x16V[\x94PaM4``\x89\x01\x89aIzV[\x94P\x91P\x80\x87\x86\x03\x01`\x80\x88\x01RaMM\x85\x85\x84aI\xC2V[\x94PaM\\`\x80\x89\x01\x89aJ\x13V[\x94P\x91P\x80\x87\x86\x03\x01`\xA0\x88\x01RaMu\x85\x85\x84aJ[V[\x94PaM\x84`\xA0\x89\x01\x89aHPV[\x94P\x91P\x80\x87\x86\x03\x01`\xC0\x88\x01RaM\x9D\x85\x85\x84aJ\xACV[\x94PaM\xAC`\xC0\x89\x01\x89aHPV[\x94P\x91P\x80\x87\x86\x03\x01`\xE0\x88\x01RaM\xC5\x85\x85\x84aK\x1EV[\x94PaM\xD4`\xE0\x89\x01\x89aHPV[\x94P\x91Pa\x01\0\x81\x88\x87\x03\x01\x81\x89\x01RaM\xEF\x86\x86\x85aK\xDDV[\x95PaM\xFD\x81\x8A\x01\x8AaIzV[\x95P\x92PP\x80\x87\x86\x03\x01\x83\x88\x01RPaN\x17\x84\x84\x83aLzV[\x97\x96PPPPPPPV[``\x81\x01c\xFF\xFF\xFF\xFFaN4\x84aC7V[\x16\x82R` \x83\x015` \x83\x01R`@\x83\x015`@\x83\x01R\x92\x91PPV\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 e't\x04\x93?\x01/'\xFC\xFEQ\x03E\xB5\xC4LeJ!l&\"\xE8\xF9\xE9\xA4\xEDL\xE3\x96\xCAdsolcC\0\x08\x0C\x003";
    /// The bytecode of the contract.
    pub static GASPMULTIROLLUPSERVICE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xDAW`\x005`\xE0\x1C\x80co\x0C0\xA4\x11a\x01\x04W\x80c\xD0\x93\x86\x7F\x11a\0\xA2W\x80c\xF2\xFD\xE3\x8B\x11a\0qW\x80c\xF2\xFD\xE3\x8B\x14a\x04\xC2W\x80c\xF8N\x91\xFC\x14a\x04\xD5W\x80c\xFA\xBC\x1C\xBC\x14a\x04\xDEW\x80c\xFE\xCF\x974\x14a\x04\xF1W`\0\x80\xFD[\x80c\xD0\x93\x86\x7F\x14a\x04yW\x80c\xDBS\rc\x14a\x04\x8CW\x80c\xDF\x03L\xD0\x14a\x04\x9FW\x80c\xE2\xA7\xCBf\x14a\x04\xB2W`\0\x80\xFD[\x80c\x88o\x11\x95\x11a\0\xDEW\x80c\x88o\x11\x95\x14a\x04\nW\x80c\x8D\xA5\xCB[\x14a\x045W\x80c\xA1\x1D\xF1L\x14a\x04FW\x80c\xC4\xE1\x91L\x14a\x04YW`\0\x80\xFD[\x80co\x0C0\xA4\x14a\x03\xC2W\x80cqP\x18\xA6\x14a\x03\xD9W\x80cz\xD7Ua\x14a\x03\xE1W`\0\x80\xFD[\x80cC\r;9\x11a\x01|W\x80cRn>d\x11a\x01KW\x80cRn>d\x14a\x03{W\x80cY\\jg\x14a\x03\x8FW\x80cZ\xC8j\xB7\x14a\x03\x97W\x80c\\\x97Z\xBB\x14a\x03\xBAW`\0\x80\xFD[\x80cC\r;9\x14a\x02\xBEW\x80cI\x9Do\xB6\x14a\x02\xF3W\x80cJ\xE6\xB2\x03\x14a\x03?W\x80cM\xEA\xBC!\x14a\x03VW`\0\x80\xFD[\x80c\x12FH\xC9\x11a\x01\xB8W\x80c\x12FH\xC9\x14a\x02YW\x80c\x13d9\xDD\x14a\x02lW\x80c\x17\x1F\x1D[\x14a\x02\x7FW\x80c*\x84\x14\xFD\x14a\x02\xA9W`\0\x80\xFD[\x80c\x03\xD0\x97\xD2\x14a\x01\xDFW\x80c\x0E\xE0\xFD\xBD\x14a\x02 W\x80c\x10\xD6z/\x14a\x02DW[`\0\x80\xFD[a\x02\x06a\x01\xED6`\x04a;wV[`\x9F` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T\x82V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[`\x97Ta\x024\x90`\x01`\xA8\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02\x17V[a\x02Wa\x02R6`\x04a;\xAEV[a\x05\x04V[\0[a\x02Wa\x02g6`\x04a;\xAEV[a\x05\xC0V[a\x02Wa\x02z6`\x04a;\xCBV[a\x05\xEAV[a\x02\x92a\x02\x8D6`\x04a=HV[a\x07)V[`@\x80Q\x92\x15\x15\x83R\x90\x15\x15` \x83\x01R\x01a\x02\x17V[a\x02\xB1a\x08\xB3V[`@Qa\x02\x17\x91\x90a=\x99V[a\x02\xE1a\x02\xCC6`\x04a;\xCBV[`\xA0` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02\x17V[a\x03'a\x03\x016`\x04a=\xEEV[`\x9E` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\x01`\x01``\x1B\x03\x16\x81V[`@Q`\x01`\x01``\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x17V[a\x03H`\x99T\x81V[`@Q\x90\x81R` \x01a\x02\x17V[`\x9CTa\x03f\x90c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\x17V[`\x97Ta\x024\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[a\x02Wa\tAV[a\x024a\x03\xA56`\x04a;wV[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`fTa\x03HV[`\x9ATa\x03f\x90`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x02Wa\n\x08V[a\x03'a\x03\xEF6`\x04a;wV[`\x9D` R`\0\x90\x81R`@\x90 T`\x01`\x01``\x1B\x03\x16\x81V[`eTa\x04\x1D\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x17V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x04\x1DV[a\x02Wa\x04T6`\x04a>2V[a\n\x1CV[a\x04la\x04g6`\x04a?\x9AV[a\r\xC2V[`@Qa\x02\x17\x91\x90a@$V[a\x02Wa\x04\x876`\x04a@yV[a\x13\xAFV[a\x02Wa\x04\x9A6`\x04a@\xE5V[a\x1EnV[`\x97Ta\x04\x1D\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x9ATa\x03f\x90c\xFF\xFF\xFF\xFF\x16\x81V[a\x02Wa\x04\xD06`\x04a;\xAEV[a-\x07V[a\x03H`\x98T\x81V[a\x02Wa\x04\xEC6`\x04a;\xCBV[a-}V[a\x02Wa\x04\xFF6`\x04aA\xA2V[a.\xD9V[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05WW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05{\x91\x90aA\xFEV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x05\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xAB\x90aB\x1BV[`@Q\x80\x91\x03\x90\xFD[a\x05\xBD\x81a0\x1EV[PV[a\x05\xC8a1\x15V[`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x062W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06V\x91\x90aBeV[a\x06rW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xAB\x90aB\x82V[`fT\x81\x81\x16\x14a\x06\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\xABV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`\0\x80`\0\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x87\x87`\0\x01Q\x88` \x01Q\x88`\0\x01Q`\0`\x02\x81\x10a\x07qWa\x07qaB\xCAV[` \x02\x01Q\x89Q`\x01` \x02\x01Q\x8A` \x01Q`\0`\x02\x81\x10a\x07\x96Wa\x07\x96aB\xCAV[` \x02\x01Q\x8B` \x01Q`\x01`\x02\x81\x10a\x07\xB2Wa\x07\xB2aB\xCAV[` \x90\x81\x02\x91\x90\x91\x01Q\x8CQ\x8D\x83\x01Q`@Qa\x08\x0F\x9A\x99\x98\x97\x96\x95\x94\x01\x98\x89R` \x89\x01\x97\x90\x97R`@\x88\x01\x95\x90\x95R``\x87\x01\x93\x90\x93R`\x80\x86\x01\x91\x90\x91R`\xA0\x85\x01R`\xC0\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01Ra\x01 \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\0\x1Ca\x082\x91\x90aB\xE0V[\x90Pa\x08\xA5a\x08Ka\x08D\x88\x84a1oV[\x86\x90a2\x06V[a\x08Sa2\x9AV[a\x08\x9Ba\x08\x8C\x85a\x08\x86`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x01\x82R`\x02\x90\x82\x01R\x90V[\x90a1oV[a\x08\x95\x8Ca3ZV[\x90a2\x06V[\x88b\x01\xD4\xC0a3\xEAV[\x90\x98\x90\x97P\x95PPPPPPV[`\x9B\x80Ta\x08\xC0\x90aC\x02V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08\xEC\x90aC\x02V[\x80\x15a\t9W\x80`\x1F\x10a\t\x0EWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t9V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t\x1CW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\x89W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xAD\x91\x90aBeV[a\t\xC9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xAB\x90aB\x82V[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[a\n\x10a1\x15V[a\n\x1A`\0a6\x0EV[V[`\x97T`\x01`\x01`\xA0\x1B\x03\x163\x14a\nvW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FUpdater must be the caller\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\xABV[`\x9AT`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16a\n\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x13\xDC\x08\x1C\xDD\x18]\x19H\x1D[\x9A[\x9A]`\x8A\x1B`D\x82\x01R`d\x01a\x05\xABV[a\n\xD4`\x80\x84\x01``\x85\x01aCKV[`\x9AT`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14a\x0B5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7Freference block hash mismatch\0\0\0`D\x82\x01R`d\x01a\x05\xABV[\x82`@Q` \x01a\x0BF\x91\x90aC\xDBV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82` \x015\x14a\x0B\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FreferenceTaskHash hash mismatch\0`D\x82\x01R`d\x01a\x05\xABV[a\x0B\xBE``\x84\x01`@\x85\x01aCKV[`\x9ATc\xFF\xFF\xFF\xFF\x91\x82\x16\x91a\x0B\xDE\x91`\x01` \x1B\x90\x04\x16a8@aD\x7FV[c\xFF\xFF\xFF\xFF\x16\x11a\x0C!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x077F\x16\xC6R\x077F\x17FR\x03`\x9C\x1B`D\x82\x01R`d\x01a\x05\xABV[B`\x98Tb\x03\xF4\x80a\x0C3\x91\x90aD\xA7V[\x11a\x0CpW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rlstale state 1`\x98\x1B`D\x82\x01R`d\x01a\x05\xABV[`\0a\x0C\xA6\x83`@Q` \x01a\x0C\x86\x91\x90aD\xBFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x83a\x04g\x90aE\x02V[`\x9CT\x90\x91Pc\xFF\xFF\xFF\xFF\x16`\0[`\x9B\x80Ta\x0C\xC2\x90aC\x02V[\x90P\x81\x10\x15a\rSW\x81`\xFF\x16\x83` \x01Q\x82\x81Q\x81\x10a\x0C\xE5Wa\x0C\xE5aB\xCAV[` \x02` \x01\x01Qa\x0C\xF7\x91\x90aE\x0EV[`\x01`\x01``\x1B\x03\x16`d\x84`\0\x01Q\x83\x81Q\x81\x10a\r\x18Wa\r\x18aB\xCAV[` \x02` \x01\x01Q`\x01`\x01``\x1B\x03\x16a\r3\x91\x90aE=V[\x10\x15a\rAWPPPPPPV[\x80a\rK\x81aE\\V[\x91PPa\x0C\xB5V[P`\x80\x84\x015`\x99U\x7F\xECh\xDB9\x18y\xB0\xF9\xF4 \xD1\xCD\xF3Gj\xFB\xDF\x08Z$b\xBFM+\x11\xDFxFb\x95\xCB\x17a\r\x8A` \x87\x01\x87aCKV[a\r\x9A``\x88\x01`@\x89\x01aCKV[`@\x80Qc\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01[`@Q\x80\x91\x03\x90\xA1PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01\x81\x90R`\x9B\x80Ta\r\xF9\x90aC\x02V[\x90P\x90Pa\x0E\x1A`@Q\x80`@\x01`@R\x80``\x81R` \x01``\x81RP\x90V[\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0E2Wa\x0E2a;\xE4V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E[W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P` \x82\x01R\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0EyWa\x0Eya;\xE4V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E\xA2W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x81R\x84QQ`\0\x90`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0E\xC2Wa\x0E\xC2a;\xE4V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E\xEBW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80[\x87QQ\x81\x10\x15a\x10\xA6Wa\x0F4\x88`\0\x01Q\x82\x81Q\x81\x10a\x0F\x15Wa\x0F\x15aB\xCAV[` \x02` \x01\x01Q\x80Q`\0\x90\x81R` \x91\x82\x01Q\x90\x91R`@\x90 \x90V[\x83\x82\x81Q\x81\x10a\x0FFWa\x0FFaB\xCAV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80\x15a\x10$W\x82a\x0Fc`\x01\x83aEwV[\x81Q\x81\x10a\x0FsWa\x0FsaB\xCAV[` \x02` \x01\x01Q`\0\x1C\x83\x82\x81Q\x81\x10a\x0F\x90Wa\x0F\x90aB\xCAV[` \x02` \x01\x01Q`\0\x1C\x11a\x10$W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`M`$\x82\x01R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: nonSignerG1PubkeysForOldSta`d\x82\x01Rl\x1D\x19H\x1B\x9B\xDD\x08\x1C\xDB\xDC\x9D\x19Y`\x9A\x1B`\x84\x82\x01R`\xA4\x01a\x05\xABV[a\x10\x92a\x10\x8B`\xA0`\0\x86\x85\x81Q\x81\x10a\x10@Wa\x10@aB\xCAV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82R\x81\x01\x91\x90\x91R`@\x01`\0 T\x8AQ\x80Q`\xFF\x90\x92\x16\x91\x85\x90\x81\x10a\x10uWa\x10uaB\xCAV[` \x02` \x01\x01Qa6`\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x87\x90a2\x06V[\x95P\x80a\x10\x9E\x81aE\\V[\x91PPa\x0E\xF2V[Pa\x10\xB0\x85a7DV[\x94P`\0[\x84\x81\x10\x15a\x12\x91W`\x9B\x81\x81Ta\x10\xCB\x90aC\x02V[\x81\x10a\x10\xD9Wa\x10\xD9aB\xCAV[\x81T`\x01\x16\x15a\x10\xF8W\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C`\0\x81\x81R`\x9F` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x01T\x90\x82\x01R\x90\x92Pa\x119\x90\x87\x90a2\x06V[`\xFF\x83\x16`\0\x90\x81R`\x9D` \x90\x81R`@\x90\x91 T\x90\x86\x01Q\x80Q\x92\x98P`\x01`\x01``\x1B\x03\x90\x91\x16\x91\x83\x90\x81\x10a\x11tWa\x11taB\xCAV[`\x01`\x01``\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x82\x01R\x84\x01Q\x80Q\x82\x90\x81\x10a\x11\xA0Wa\x11\xA0aB\xCAV[` \x02` \x01\x01Q\x84`\0\x01Q\x82\x81Q\x81\x10a\x11\xBEWa\x11\xBEaB\xCAV[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPP`\0[\x88QQ\x81\x10\x15a\x12~W`\x9E`\0\x85\x83\x81Q\x81\x10a\x12\x01Wa\x12\x01aB\xCAV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82R\x81\x81\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 `\xFF\x87\x16\x82R\x90\x92R\x90 T\x85Q\x80Q`\x01`\x01``\x1B\x03\x90\x92\x16\x91\x84\x90\x81\x10a\x12LWa\x12LaB\xCAV[` \x02` \x01\x01\x81\x81Qa\x12`\x91\x90aE\x8EV[`\x01`\x01``\x1B\x03\x16\x90RP\x80a\x12v\x81aE\\V[\x91PPa\x11\xE1V[P\x80a\x12\x89\x81aE\\V[\x91PPa\x10\xB5V[P`\0\x80a\x12\xA9\x8A\x88\x8B` \x01Q\x8C`@\x01Qa\x07)V[\x91P\x91P\x81a\x13,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: pairing precompile call fai`d\x82\x01Rb\x1B\x19Y`\xEA\x1B`\x84\x82\x01R`\xA4\x01a\x05\xABV[\x80a\x13\x9FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: signature is invalid\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\xABV[P\x92\x95PPPPPP[\x92\x91PPV[a\x13\xB7a1\x15V[`\0[a\x13\xC7` \x84\x01\x84aE\xB6V[\x90P\x81\x10\x15a\x14\x8CW`\x9D`\0a\x13\xE1` \x86\x01\x86aE\xB6V[\x84\x81\x81\x10a\x13\xF1Wa\x13\xF1aB\xCAV[\x90P` \x02\x01` \x81\x01\x90a\x14\x06\x91\x90a;wV[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0\x90\x81 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U`\x9F\x91a\x149\x90\x86\x01\x86aE\xB6V[\x84\x81\x81\x10a\x14IWa\x14IaB\xCAV[\x90P` \x02\x01` \x81\x01\x90a\x14^\x91\x90a;wV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x81\x81U`\x01\x01U\x80a\x14\x84\x81aE\\V[\x91PPa\x13\xBAV[P`\0[a\x14\x9D`@\x84\x01\x84aE\xFFV[\x90P\x81\x10\x15a\x15\xD8Wa\x14\xB3`@\x84\x01\x84aE\xFFV[\x82\x81\x81\x10a\x14\xC3Wa\x14\xC3aB\xCAV[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\x14\xDB\x91\x90aF_V[`\x9D`\0a\x14\xEC`@\x87\x01\x87aE\xFFV[\x85\x81\x81\x10a\x14\xFCWa\x14\xFCaB\xCAV[a\x15\x12\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa;wV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x90\x81\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x17\x90\x91Ua\x15R\x90\x84\x01\x84aE\xFFV[\x82\x81\x81\x10a\x15bWa\x15baB\xCAV[\x90P`\x80\x02\x01`@\x01`\x9F`\0\x85\x80`@\x01\x90a\x15\x7F\x91\x90aE\xFFV[\x85\x81\x81\x10a\x15\x8FWa\x15\x8FaB\xCAV[a\x15\xA5\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa;wV[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a\x15\xD0\x81aE\\V[\x91PPa\x14\x90V[P`\0[a\x15\xE9``\x84\x01\x84aFzV[\x90P\x81\x10\x15a\x16\xA1Wa\x15\xFF``\x84\x01\x84aFzV[\x82\x81\x81\x10a\x16\x0FWa\x16\x0FaB\xCAV[\x90P`@\x02\x01` \x01` \x81\x01\x90a\x16'\x91\x90aF_V[`\x9D`\0a\x168``\x87\x01\x87aFzV[\x85\x81\x81\x10a\x16HWa\x16HaB\xCAV[a\x16^\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa;wV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\x16\x99\x81aE\\V[\x91PPa\x15\xDCV[P`\0[a\x16\xB2`\x80\x84\x01\x84aF\xC3V[\x90P\x81\x10\x15a\x17NWa\x16\xC8`\x80\x84\x01\x84aF\xC3V[\x82\x81\x81\x10a\x16\xD8Wa\x16\xD8aB\xCAV[\x90P``\x02\x01` \x01`\x9F`\0\x85\x80`\x80\x01\x90a\x16\xF5\x91\x90aF\xC3V[\x85\x81\x81\x10a\x17\x05Wa\x17\x05aB\xCAV[a\x17\x1B\x92` ``\x90\x92\x02\x01\x90\x81\x01\x91Pa;wV[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a\x17F\x81aE\\V[\x91PPa\x16\xA5V[P`\0[a\x17_`\xA0\x84\x01\x84aE\xB6V[\x90P\x81\x10\x15a\x18\x85W`\0[`\x9B\x80Ta\x17x\x90aC\x02V[\x90P\x81\x10\x15a\x181W`\x9E`\0a\x17\x92`\xA0\x87\x01\x87aE\xB6V[\x85\x81\x81\x10a\x17\xA2Wa\x17\xA2aB\xCAV[\x90P` \x02\x015\x81R` \x01\x90\x81R` \x01`\0 `\0`\x9B\x83\x81Ta\x17\xC7\x90aC\x02V[\x81\x10a\x17\xD5Wa\x17\xD5aB\xCAV[\x81T`\x01\x16\x15a\x17\xF4W\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U\x81a\x18)\x81aE\\V[\x92PPa\x17kV[P`\xA0`\0a\x18B\x85\x83\x01\x86aE\xB6V[\x84\x81\x81\x10a\x18RWa\x18RaB\xCAV[` \x90\x81\x02\x92\x90\x92\x015\x83RP\x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\xFF\x19\x16\x90U\x80a\x18}\x81aE\\V[\x91PPa\x17RV[P`\0[a\x18\x96`\xC0\x84\x01\x84aE\xB6V[\x90P\x81\x10\x15a\x1A\xE6Wa\x18\xAC`\xC0\x84\x01\x84aE\xB6V[\x82\x81\x81\x10a\x18\xBCWa\x18\xBCaB\xCAV[\x90P` \x02\x81\x01\x90a\x18\xCE\x91\x90aG\x0BV[a\x18\xDF\x90`\x80\x81\x01\x90``\x01a;wV[`\xA0`\0a\x18\xF0`\xC0\x87\x01\x87aE\xB6V[\x85\x81\x81\x10a\x19\0Wa\x19\0aB\xCAV[\x90P` \x02\x81\x01\x90a\x19\x12\x91\x90aG\x0BV[`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP`\0[a\x19L`\xC0\x85\x01\x85aE\xB6V[\x83\x81\x81\x10a\x19\\Wa\x19\\aB\xCAV[\x90P` \x02\x81\x01\x90a\x19n\x91\x90aG\x0BV[a\x19|\x90` \x81\x01\x90aE\xB6V[\x90P\x81\x10\x15a\x1A\xD3Wa\x19\x92`\xC0\x85\x01\x85aE\xB6V[\x83\x81\x81\x10a\x19\xA2Wa\x19\xA2aB\xCAV[\x90P` \x02\x81\x01\x90a\x19\xB4\x91\x90aG\x0BV[a\x19\xC2\x90`@\x81\x01\x90aE\xB6V[\x82\x81\x81\x10a\x19\xD2Wa\x19\xD2aB\xCAV[\x90P` \x02\x01` \x81\x01\x90a\x19\xE7\x91\x90aF_V[`\x9E`\0a\x19\xF8`\xC0\x88\x01\x88aE\xB6V[\x86\x81\x81\x10a\x1A\x08Wa\x1A\x08aB\xCAV[\x90P` \x02\x81\x01\x90a\x1A\x1A\x91\x90aG\x0BV[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a\x1A;`\xC0\x88\x01\x88aE\xB6V[\x86\x81\x81\x10a\x1AKWa\x1AKaB\xCAV[\x90P` \x02\x81\x01\x90a\x1A]\x91\x90aG\x0BV[a\x1Ak\x90` \x81\x01\x90aE\xB6V[\x85\x81\x81\x10a\x1A{Wa\x1A{aB\xCAV[\x90P` \x02\x01` \x81\x01\x90a\x1A\x90\x91\x90a;wV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\x1A\xCB\x81aE\\V[\x91PPa\x19?V[P\x80a\x1A\xDE\x81aE\\V[\x91PPa\x18\x89V[P`\0[a\x1A\xF7`\xE0\x84\x01\x84aE\xB6V[\x90P\x81\x10\x15a\x1C\xAAW`\0[a\x1B\x10`\xE0\x85\x01\x85aE\xB6V[\x83\x81\x81\x10a\x1B Wa\x1B aB\xCAV[\x90P` \x02\x81\x01\x90a\x1B2\x91\x90aG+V[a\x1B@\x90` \x81\x01\x90aE\xB6V[\x90P\x81\x10\x15a\x1C\x97Wa\x1BV`\xE0\x85\x01\x85aE\xB6V[\x83\x81\x81\x10a\x1BfWa\x1BfaB\xCAV[\x90P` \x02\x81\x01\x90a\x1Bx\x91\x90aG+V[a\x1B\x86\x90`@\x81\x01\x90aE\xB6V[\x82\x81\x81\x10a\x1B\x96Wa\x1B\x96aB\xCAV[\x90P` \x02\x01` \x81\x01\x90a\x1B\xAB\x91\x90aF_V[`\x9E`\0a\x1B\xBC`\xE0\x88\x01\x88aE\xB6V[\x86\x81\x81\x10a\x1B\xCCWa\x1B\xCCaB\xCAV[\x90P` \x02\x81\x01\x90a\x1B\xDE\x91\x90aG+V[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a\x1B\xFF`\xE0\x88\x01\x88aE\xB6V[\x86\x81\x81\x10a\x1C\x0FWa\x1C\x0FaB\xCAV[\x90P` \x02\x81\x01\x90a\x1C!\x91\x90aG+V[a\x1C/\x90` \x81\x01\x90aE\xB6V[\x85\x81\x81\x10a\x1C?Wa\x1C?aB\xCAV[\x90P` \x02\x01` \x81\x01\x90a\x1CT\x91\x90a;wV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\x1C\x8F\x81aE\\V[\x91PPa\x1B\x03V[P\x80a\x1C\xA2\x81aE\\V[\x91PPa\x1A\xEAV[P`\0[a\x1C\xBCa\x01\0\x84\x01\x84aFzV[\x90P\x81\x10\x15a\x1D`Wa\x1C\xD3a\x01\0\x84\x01\x84aFzV[\x82\x81\x81\x10a\x1C\xE3Wa\x1C\xE3aB\xCAV[\x90P`@\x02\x01` \x01` \x81\x01\x90a\x1C\xFB\x91\x90a;wV[`\xA0`\0a\x1D\ra\x01\0\x87\x01\x87aFzV[\x85\x81\x81\x10a\x1D\x1DWa\x1D\x1DaB\xCAV[\x90P`@\x02\x01`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP\x80\x80a\x1DX\x90aE\\V[\x91PPa\x1C\xAEV[P`\x99\x81\x90Ua\x1Ds` \x84\x01\x84aCKV[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\x1D\x9D`@\x84\x01` \x85\x01aCKV[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90UB`\x98Ua\x1D\xD4``\x84\x01\x84aGAV[a\x1D\xE0\x91`\x9B\x91a9\xF3V[Pa\x1D\xF1`\xA0\x84\x01`\x80\x85\x01aCKV[`\x9C\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x7F&Ie\xEBk\xC46\xC6\xC4sC\x1D4\xAFV\xE82\xEC4O\xDF\xD4>\xE6\xAFo\xCEm ^\x84\xAFa\x1E9` \x85\x01\x85aCKV[a\x1EI`@\x86\x01` \x87\x01aCKV[`@\x80Qc\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1PPPV[`\x9AT`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15\x80\x15a\x1F W`\x97T`\x01`\xA8\x1B\x90\x04`\xFF\x16\x15a\x1E\xDEW`\x97T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1E\xD9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04\x17WF\x83`\xDC\x1B`D\x82\x01R`d\x01a\x05\xABV[a\x1FbV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1E\xD9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd \xBA\xBA4\x19`\xD9\x1B`D\x82\x01R`d\x01a\x05\xABV[`\x97T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1FbW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04\x17WF\x83`\xDC\x1B`D\x82\x01R`d\x01a\x05\xABV[\x84`@Q` \x01a\x1Fs\x91\x90aG\x87V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x015\x14a\x1F\xDBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FreferenceTaskHash hash mismatch\0`D\x82\x01R`d\x01a\x05\xABV[\x81`@Q` \x01a\x1F\xEC\x91\x90aL\xBBV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84`@\x015\x14a TW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FoperatorStateInfo hash mismatch\0`D\x82\x01R`d\x01a\x05\xABV[\x80a\"tWa i``\x86\x01`@\x87\x01aCKV[`\x9AT`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14a \xCAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7Freference block mismatch\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\xABV[a \xDA`@\x86\x01` \x87\x01aCKV[`\x9ATc\xFF\xFF\xFF\xFF\x91\x82\x16\x91a \xFA\x91`\x01` \x1B\x90\x04\x16a8@aD\x7FV[c\xFF\xFF\xFF\xFF\x16\x11a!=W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x077F\x16\xC6R\x077F\x17FR\x03`\x9C\x1B`D\x82\x01R`d\x01a\x05\xABV[B`\x98Tb\x03\xF4\x80a!O\x91\x90aD\xA7V[\x11a!\x8CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rlstale state 1`\x98\x1B`D\x82\x01R`d\x01a\x05\xABV[`\0a!\xC2\x85`@Q` \x01a!\xA2\x91\x90aN\"V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x85a\x04g\x90aE\x02V[`\x9CT\x90\x91Pc\xFF\xFF\xFF\xFF\x16`\0[`\x9B\x80Ta!\xDE\x90aC\x02V[\x90P\x81\x10\x15a\"pW\x81`\xFF\x16\x83` \x01Q\x82\x81Q\x81\x10a\"\x01Wa\"\x01aB\xCAV[` \x02` \x01\x01Qa\"\x13\x91\x90aE\x0EV[`\x01`\x01``\x1B\x03\x16`d\x84`\0\x01Q\x83\x81Q\x81\x10a\"4Wa\"4aB\xCAV[` \x02` \x01\x01Q`\x01`\x01``\x1B\x03\x16a\"O\x91\x90aE=V[\x10\x15a\"^WPPPPa-\x01V[\x80a\"h\x81aE\\V[\x91PPa!\xD1V[PPP[`\0[a\"\x84` \x84\x01\x84aE\xB6V[\x90P\x81\x10\x15a#IW`\x9D`\0a\"\x9E` \x86\x01\x86aE\xB6V[\x84\x81\x81\x10a\"\xAEWa\"\xAEaB\xCAV[\x90P` \x02\x01` \x81\x01\x90a\"\xC3\x91\x90a;wV[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0\x90\x81 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U`\x9F\x91a\"\xF6\x90\x86\x01\x86aE\xB6V[\x84\x81\x81\x10a#\x06Wa#\x06aB\xCAV[\x90P` \x02\x01` \x81\x01\x90a#\x1B\x91\x90a;wV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x81\x81U`\x01\x01U\x80a#A\x81aE\\V[\x91PPa\"wV[P`\0[a#Z`@\x84\x01\x84aE\xFFV[\x90P\x81\x10\x15a$\x95Wa#p`@\x84\x01\x84aE\xFFV[\x82\x81\x81\x10a#\x80Wa#\x80aB\xCAV[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a#\x98\x91\x90aF_V[`\x9D`\0a#\xA9`@\x87\x01\x87aE\xFFV[\x85\x81\x81\x10a#\xB9Wa#\xB9aB\xCAV[a#\xCF\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa;wV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x90\x81\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x17\x90\x91Ua$\x0F\x90\x84\x01\x84aE\xFFV[\x82\x81\x81\x10a$\x1FWa$\x1FaB\xCAV[\x90P`\x80\x02\x01`@\x01`\x9F`\0\x85\x80`@\x01\x90a$<\x91\x90aE\xFFV[\x85\x81\x81\x10a$LWa$LaB\xCAV[a$b\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa;wV[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a$\x8D\x81aE\\V[\x91PPa#MV[P`\0[a$\xA6``\x84\x01\x84aFzV[\x90P\x81\x10\x15a%^Wa$\xBC``\x84\x01\x84aFzV[\x82\x81\x81\x10a$\xCCWa$\xCCaB\xCAV[\x90P`@\x02\x01` \x01` \x81\x01\x90a$\xE4\x91\x90aF_V[`\x9D`\0a$\xF5``\x87\x01\x87aFzV[\x85\x81\x81\x10a%\x05Wa%\x05aB\xCAV[a%\x1B\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa;wV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a%V\x81aE\\V[\x91PPa$\x99V[P`\0[a%o`\x80\x84\x01\x84aF\xC3V[\x90P\x81\x10\x15a&\x0BWa%\x85`\x80\x84\x01\x84aF\xC3V[\x82\x81\x81\x10a%\x95Wa%\x95aB\xCAV[\x90P``\x02\x01` \x01`\x9F`\0\x85\x80`\x80\x01\x90a%\xB2\x91\x90aF\xC3V[\x85\x81\x81\x10a%\xC2Wa%\xC2aB\xCAV[a%\xD8\x92` ``\x90\x92\x02\x01\x90\x81\x01\x91Pa;wV[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a&\x03\x81aE\\V[\x91PPa%bV[P`\0[a&\x1C`\xA0\x84\x01\x84aE\xB6V[\x90P\x81\x10\x15a'BW`\0[`\x9B\x80Ta&5\x90aC\x02V[\x90P\x81\x10\x15a&\xEEW`\x9E`\0a&O`\xA0\x87\x01\x87aE\xB6V[\x85\x81\x81\x10a&_Wa&_aB\xCAV[\x90P` \x02\x015\x81R` \x01\x90\x81R` \x01`\0 `\0`\x9B\x83\x81Ta&\x84\x90aC\x02V[\x81\x10a&\x92Wa&\x92aB\xCAV[\x81T`\x01\x16\x15a&\xB1W\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U\x81a&\xE6\x81aE\\V[\x92PPa&(V[P`\xA0`\0a&\xFF\x85\x83\x01\x86aE\xB6V[\x84\x81\x81\x10a'\x0FWa'\x0FaB\xCAV[` \x90\x81\x02\x92\x90\x92\x015\x83RP\x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\xFF\x19\x16\x90U\x80a':\x81aE\\V[\x91PPa&\x0FV[P`\0[a'S`\xC0\x84\x01\x84aE\xB6V[\x90P\x81\x10\x15a)\xA3Wa'i`\xC0\x84\x01\x84aE\xB6V[\x82\x81\x81\x10a'yWa'yaB\xCAV[\x90P` \x02\x81\x01\x90a'\x8B\x91\x90aG\x0BV[a'\x9C\x90`\x80\x81\x01\x90``\x01a;wV[`\xA0`\0a'\xAD`\xC0\x87\x01\x87aE\xB6V[\x85\x81\x81\x10a'\xBDWa'\xBDaB\xCAV[\x90P` \x02\x81\x01\x90a'\xCF\x91\x90aG\x0BV[`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP`\0[a(\t`\xC0\x85\x01\x85aE\xB6V[\x83\x81\x81\x10a(\x19Wa(\x19aB\xCAV[\x90P` \x02\x81\x01\x90a(+\x91\x90aG\x0BV[a(9\x90` \x81\x01\x90aE\xB6V[\x90P\x81\x10\x15a)\x90Wa(O`\xC0\x85\x01\x85aE\xB6V[\x83\x81\x81\x10a(_Wa(_aB\xCAV[\x90P` \x02\x81\x01\x90a(q\x91\x90aG\x0BV[a(\x7F\x90`@\x81\x01\x90aE\xB6V[\x82\x81\x81\x10a(\x8FWa(\x8FaB\xCAV[\x90P` \x02\x01` \x81\x01\x90a(\xA4\x91\x90aF_V[`\x9E`\0a(\xB5`\xC0\x88\x01\x88aE\xB6V[\x86\x81\x81\x10a(\xC5Wa(\xC5aB\xCAV[\x90P` \x02\x81\x01\x90a(\xD7\x91\x90aG\x0BV[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a(\xF8`\xC0\x88\x01\x88aE\xB6V[\x86\x81\x81\x10a)\x08Wa)\x08aB\xCAV[\x90P` \x02\x81\x01\x90a)\x1A\x91\x90aG\x0BV[a)(\x90` \x81\x01\x90aE\xB6V[\x85\x81\x81\x10a)8Wa)8aB\xCAV[\x90P` \x02\x01` \x81\x01\x90a)M\x91\x90a;wV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a)\x88\x81aE\\V[\x91PPa'\xFCV[P\x80a)\x9B\x81aE\\V[\x91PPa'FV[P`\0[a)\xB4`\xE0\x84\x01\x84aE\xB6V[\x90P\x81\x10\x15a+gW`\0[a)\xCD`\xE0\x85\x01\x85aE\xB6V[\x83\x81\x81\x10a)\xDDWa)\xDDaB\xCAV[\x90P` \x02\x81\x01\x90a)\xEF\x91\x90aG+V[a)\xFD\x90` \x81\x01\x90aE\xB6V[\x90P\x81\x10\x15a+TWa*\x13`\xE0\x85\x01\x85aE\xB6V[\x83\x81\x81\x10a*#Wa*#aB\xCAV[\x90P` \x02\x81\x01\x90a*5\x91\x90aG+V[a*C\x90`@\x81\x01\x90aE\xB6V[\x82\x81\x81\x10a*SWa*SaB\xCAV[\x90P` \x02\x01` \x81\x01\x90a*h\x91\x90aF_V[`\x9E`\0a*y`\xE0\x88\x01\x88aE\xB6V[\x86\x81\x81\x10a*\x89Wa*\x89aB\xCAV[\x90P` \x02\x81\x01\x90a*\x9B\x91\x90aG+V[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a*\xBC`\xE0\x88\x01\x88aE\xB6V[\x86\x81\x81\x10a*\xCCWa*\xCCaB\xCAV[\x90P` \x02\x81\x01\x90a*\xDE\x91\x90aG+V[a*\xEC\x90` \x81\x01\x90aE\xB6V[\x85\x81\x81\x10a*\xFCWa*\xFCaB\xCAV[\x90P` \x02\x01` \x81\x01\x90a+\x11\x91\x90a;wV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a+L\x81aE\\V[\x91PPa)\xC0V[P\x80a+_\x81aE\\V[\x91PPa)\xA7V[P`\0[a+ya\x01\0\x84\x01\x84aFzV[\x90P\x81\x10\x15a,\x1DWa+\x90a\x01\0\x84\x01\x84aFzV[\x82\x81\x81\x10a+\xA0Wa+\xA0aB\xCAV[\x90P`@\x02\x01` \x01` \x81\x01\x90a+\xB8\x91\x90a;wV[`\xA0`\0a+\xCAa\x01\0\x87\x01\x87aFzV[\x85\x81\x81\x10a+\xDAWa+\xDAaB\xCAV[\x90P`@\x02\x01`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP\x80\x80a,\x15\x90aE\\V[\x91PPa+kV[Pa,+` \x86\x01\x86aCKV[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua,U`@\x86\x01` \x87\x01aCKV[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90UB`\x98Ua,\x8C``\x86\x01\x86aGAV[a,\x98\x91`\x9B\x91a9\xF3V[Pa,\xA9`\xA0\x86\x01`\x80\x87\x01aCKV[`\x9C\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x7F6\xA1\xFD{\xD5T\xF5\xC4(\xC9\x82\x9C\t\xC6`kL\x89;\x1F\xAD\xC8sZz\x12yW\x97D}\xEDa,\xF1` \x87\x01\x87aCKV[a\r\x9A`@\x88\x01` \x89\x01aCKV[PPPPV[a-\x0Fa1\x15V[`\x01`\x01`\xA0\x1B\x03\x81\x16a-tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x05\xABV[a\x05\xBD\x81a6\x0EV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-\xD0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-\xF4\x91\x90aA\xFEV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a.$W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xAB\x90aB\x1BV[`fT\x19\x81\x19`fT\x19\x16\x14a.\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\xABV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x07\x1EV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a.\xF9WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a/\x13WP0;\x15\x80\x15a/\x13WP`\0T`\xFF\x16`\x01\x14[a/vW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x05\xABV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a/\x99W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a/\xA4\x85`\0a7\xDFV[a/\xAD\x84a6\x0EV[`\x97\x80T\x83\x15\x15`\x01`\xA8\x1B\x02`\x01a\xFF\x01`\xA0\x1B\x03\x19\x90\x91\x16`\x01`\x01`\xA0\x1B\x03\x86\x16\x17\x17\x90U\x80\x15a0\x17W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01a\r\xB3V[PPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16a0\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x05\xABV[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\n\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x05\xABV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra1\x8Ba:wV[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x80\x82\x01\x84\x90R`\0\x90\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a1\xBEWa1\xC0V[\xFE[P\x80a1\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCB[][\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x05\xABV[PP\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra2\"a:\x95V[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R\x83Q`@\x80\x84\x01\x91\x90\x91R\x90\x84\x01Q``\x83\x01R`\0\x90\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a1\xBEWP\x80a1\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCBXY\x19\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x05\xABV[a2\xA2a:\xB3V[P`@\x80Q`\x80\x81\x01\x82R\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81\x83\x01\x90\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED``\x83\x01R\x81R\x81Q\x80\x83\x01\x90\x92R\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x81\x01\x91\x90\x91R\x81\x01\x91\x90\x91R\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0\x80\x80a3\x8A`\0\x80Q` aNR\x839\x81Q\x91R\x86aB\xE0V[\x90P[a3\x96\x81a8\xC9V[\x90\x93P\x91P`\0\x80Q` aNR\x839\x81Q\x91R\x82\x83\t\x83\x14\x15a3\xD0W`@\x80Q\x80\x82\x01\x90\x91R\x90\x81R` \x81\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80Q` aNR\x839\x81Q\x91R`\x01\x82\x08\x90Pa3\x8DV[`@\x80Q\x80\x82\x01\x82R\x86\x81R` \x80\x82\x01\x86\x90R\x82Q\x80\x84\x01\x90\x93R\x86\x83R\x82\x01\x84\x90R`\0\x91\x82\x91\x90a4\x1Ca:\xD8V[`\0[`\x02\x81\x10\x15a5\xE1W`\0a45\x82`\x06aE=V[\x90P\x84\x82`\x02\x81\x10a4IWa4IaB\xCAV[` \x02\x01QQ\x83a4[\x83`\0aD\xA7V[`\x0C\x81\x10a4kWa4kaB\xCAV[` \x02\x01R\x84\x82`\x02\x81\x10a4\x82Wa4\x82aB\xCAV[` \x02\x01Q` \x01Q\x83\x82`\x01a4\x99\x91\x90aD\xA7V[`\x0C\x81\x10a4\xA9Wa4\xA9aB\xCAV[` \x02\x01R\x83\x82`\x02\x81\x10a4\xC0Wa4\xC0aB\xCAV[` \x02\x01QQQ\x83a4\xD3\x83`\x02aD\xA7V[`\x0C\x81\x10a4\xE3Wa4\xE3aB\xCAV[` \x02\x01R\x83\x82`\x02\x81\x10a4\xFAWa4\xFAaB\xCAV[` \x02\x01QQ`\x01` \x02\x01Q\x83a5\x13\x83`\x03aD\xA7V[`\x0C\x81\x10a5#Wa5#aB\xCAV[` \x02\x01R\x83\x82`\x02\x81\x10a5:Wa5:aB\xCAV[` \x02\x01Q` \x01Q`\0`\x02\x81\x10a5UWa5UaB\xCAV[` \x02\x01Q\x83a5f\x83`\x04aD\xA7V[`\x0C\x81\x10a5vWa5vaB\xCAV[` \x02\x01R\x83\x82`\x02\x81\x10a5\x8DWa5\x8DaB\xCAV[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a5\xA8Wa5\xA8aB\xCAV[` \x02\x01Q\x83a5\xB9\x83`\x05aD\xA7V[`\x0C\x81\x10a5\xC9Wa5\xC9aB\xCAV[` \x02\x01RP\x80a5\xD9\x81aE\\V[\x91PPa4\x1FV[Pa5\xEAa:\xF7V[`\0` \x82a\x01\x80\x85`\x08\x8C\xFA\x91Q\x91\x9C\x91\x15\x15\x9BP\x90\x99PPPPPPPPPPV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x02\0\x82a\xFF\xFF\x16\x10a6\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Roscalar-too-large`\x80\x1B`D\x82\x01R`d\x01a\x05\xABV[\x81a\xFF\xFF\x16`\x01\x14\x15a6\xD0WP\x81a\x13\xA9V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01\x81\x90R\x84\x90`\x01\x90[\x81a\xFF\xFF\x16\x86a\xFF\xFF\x16\x10a79W`\x01a\xFF\xFF\x87\x16`\xFF\x83\x16\x1C\x81\x16\x14\x15a7\x1CWa7\x19\x84\x84a2\x06V[\x93P[a7&\x83\x84a2\x06V[\x92Pb\x01\xFF\xFE`\x01\x92\x83\x1B\x16\x91\x01a6\xECV[P\x91\x95\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q\x15\x80\x15a7iWP` \x82\x01Q\x15[\x15a7\x87WPP`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01`\0\x80Q` aNR\x839\x81Q\x91R\x84` \x01Qa7\xBA\x91\x90aB\xE0V[a7\xD2\x90`\0\x80Q` aNR\x839\x81Q\x91RaEwV[\x90R\x92\x91PPV[\x91\x90PV[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a8\0WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a8\x82W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x05\xABV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a8\xC5\x82a0\x1EV[PPV[`\0\x80\x80`\0\x80Q` aNR\x839\x81Q\x91R`\x03`\0\x80Q` aNR\x839\x81Q\x91R\x86`\0\x80Q` aNR\x839\x81Q\x91R\x88\x89\t\t\x08\x90P`\0a9?\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\0\x80Q` aNR\x839\x81Q\x91Ra9KV[\x91\x95\x91\x94P\x90\x92PPPV[`\0\x80a9Va:\xF7V[a9^a;\x15V[` \x80\x82R\x81\x81\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x88\x90R`\x80\x82\x01\x87\x90R`\xA0\x82\x01\x86\x90R\x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82\x80\x15a1\xBEWP\x82a9\xE8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FBN254.expMod: call failure\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\xABV[PQ\x95\x94PPPPPV[\x82\x80Ta9\xFF\x90aC\x02V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82a:!W`\0\x85Ua:gV[\x82`\x1F\x10a::W\x82\x80\x01`\xFF\x19\x825\x16\x17\x85Ua:gV[\x82\x80\x01`\x01\x01\x85U\x82\x15a:gW\x91\x82\x01[\x82\x81\x11\x15a:gW\x825\x82U\x91` \x01\x91\x90`\x01\x01\x90a:LV[Pa:s\x92\x91Pa;3V[P\x90V[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80a:\xC6a;HV[\x81R` \x01a:\xD3a;HV[\x90R\x90V[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[[\x80\x82\x11\x15a:sW`\0\x81U`\x01\x01a;4V[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[\x805`\xFF\x81\x16\x81\x14a7\xDAW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a;\x89W`\0\x80\xFD[a;\x92\x82a;fV[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05\xBDW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a;\xC0W`\0\x80\xFD[\x815a;\x92\x81a;\x99V[`\0` \x82\x84\x03\x12\x15a;\xDDW`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a<\x1CWa<\x1Ca;\xE4V[`@R\x90V[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a<\x1CWa<\x1Ca;\xE4V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a<lWa<la;\xE4V[`@R\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a<\x86W`\0\x80\xFD[a<\x8Ea;\xFAV[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a<\xB5W`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a<\xD7Wa<\xD7a;\xE4V[\x80`@RP\x80`@\x84\x01\x85\x81\x11\x15a<\xEEW`\0\x80\xFD[\x84[\x81\x81\x10\x15a79W\x805\x83R` \x92\x83\x01\x92\x01a<\xF0V[`\0`\x80\x82\x84\x03\x12\x15a=\x1AW`\0\x80\xFD[a=\"a;\xFAV[\x90Pa=.\x83\x83a<\xA4V[\x81Ra==\x83`@\x84\x01a<\xA4V[` \x82\x01R\x92\x91PPV[`\0\x80`\0\x80a\x01 \x85\x87\x03\x12\x15a=_W`\0\x80\xFD[\x845\x93Pa=p\x86` \x87\x01a<tV[\x92Pa=\x7F\x86``\x87\x01a=\x08V[\x91Pa=\x8E\x86`\xE0\x87\x01a<tV[\x90P\x92\x95\x91\x94P\x92PV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a=\xC6W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a=\xAAV[\x81\x81\x11\x15a=\xD8W`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a>\x01W`\0\x80\xFD[\x825\x91Pa>\x11` \x84\x01a;fV[\x90P\x92P\x92\x90PV[`\0`\xE0\x82\x84\x03\x12\x15a>,W`\0\x80\xFD[P\x91\x90PV[`\0\x80`\0\x83\x85\x03`\xE0\x81\x12\x15a>HW`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a>_W`\0\x80\xFD[\x90\x86\x01\x90`\xC0\x82\x89\x03\x12\x15a>sW`\0\x80\xFD[\x81\x95P`\xA0`\x1F\x19\x84\x01\x12\x15a>\x88W`\0\x80\xFD[` \x87\x01\x94P`\xC0\x87\x015\x92P\x80\x83\x11\x15a>\xA2W`\0\x80\xFD[PPa>\xB0\x86\x82\x87\x01a>\x1AV[\x91PP\x92P\x92P\x92V[`\0`\xE0\x82\x84\x03\x12\x15a>\xCCW`\0\x80\xFD[a>\xD4a<\"V[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a>\xEDW`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a?\x01W`\0\x80\xFD[\x815` \x82\x82\x11\x15a?\x15Wa?\x15a;\xE4V[a?#\x81\x83`\x05\x1B\x01a<DV[\x82\x81R\x81\x81\x01\x93P`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x87\x83\x11\x15a?CW`\0\x80\xFD[\x93\x81\x01\x93[\x82\x85\x10\x15a?lWa?Z\x88\x86a<tV[\x84R\x81\x84\x01\x93P`@\x85\x01\x94Pa?HV[\x85Ra?z\x87\x87\x83\x01a=\x08V[\x81\x86\x01RPPPPa?\x8F\x83`\xA0\x84\x01a<tV[`@\x82\x01R\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a?\xADW`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a?\xCAW`\0\x80\xFD[a?\xD6\x85\x82\x86\x01a>\xBAV[\x91PP\x92P\x92\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a@\x19W\x81Q`\x01`\x01``\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a?\xF4V[P\x94\x95\x94PPPPPV[` \x81R`\0\x82Q`@` \x84\x01Ra@@``\x84\x01\x82a?\xE0V[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01Ra@]\x82\x82a?\xE0V[\x95\x94PPPPPV[`\0a\x01 \x82\x84\x03\x12\x15a>,W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a@\x8EW`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a@\xA5W`\0\x80\xFD[a@\xB1\x87\x83\x88\x01a>\x1AV[\x94P` \x86\x015\x91P\x80\x82\x11\x15a@\xC7W`\0\x80\xFD[Pa@\xD4\x86\x82\x87\x01a@fV[\x92PP`@\x84\x015\x90P\x92P\x92P\x92V[`\0\x80`\0\x80\x84\x86\x03`\xC0\x81\x12\x15a@\xFCW`\0\x80\xFD[\x855`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aA\x13W`\0\x80\xFD[aA\x1F\x89\x83\x8A\x01a>\x1AV[\x96P```\x1F\x19\x84\x01\x12\x15aA3W`\0\x80\xFD[` \x88\x01\x95P`\x80\x88\x015\x92P\x80\x83\x11\x15aAMW`\0\x80\xFD[aAY\x89\x84\x8A\x01a>\x1AV[\x94P`\xA0\x88\x015\x92P\x80\x83\x11\x15aAoW`\0\x80\xFD[PPaA}\x87\x82\x88\x01a@fV[\x91PP\x92\x95\x91\x94P\x92PV[\x80\x15\x15\x81\x14a\x05\xBDW`\0\x80\xFD[\x805a7\xDA\x81aA\x89V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aA\xB8W`\0\x80\xFD[\x845aA\xC3\x81a;\x99V[\x93P` \x85\x015aA\xD3\x81a;\x99V[\x92P`@\x85\x015aA\xE3\x81a;\x99V[\x91P``\x85\x015aA\xF3\x81aA\x89V[\x93\x96\x92\x95P\x90\x93PPV[`\0` \x82\x84\x03\x12\x15aB\x10W`\0\x80\xFD[\x81Qa;\x92\x81a;\x99V[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15aBwW`\0\x80\xFD[\x81Qa;\x92\x81aA\x89V[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x82aB\xFDWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80aC\x16W`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15a>,WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a7\xDAW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aC]W`\0\x80\xFD[a;\x92\x82aC7V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aC}W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aC\x9CW`\0\x80\xFD[\x806\x03\x83\x13\x15aC\xABW`\0\x80\xFD[\x92P\x92\x90PV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x81R`\0c\xFF\xFF\xFF\xFF\x80aC\xF0\x85aC7V[\x16` \x84\x01R` \x84\x015`@\x84\x01R\x80aD\r`@\x86\x01aC7V[\x16``\x84\x01R\x80aD ``\x86\x01aC7V[\x16`\x80\x84\x01RaD3`\x80\x85\x01\x85aCfV[`\xC0`\xA0\x86\x01RaDH`\xE0\x86\x01\x82\x84aC\xB2V[\x91PP\x81aDX`\xA0\x87\x01aC7V[\x16`\xC0\x85\x01R\x80\x92PPP\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aD\x9EWaD\x9EaDiV[\x01\x94\x93PPPPV[`\0\x82\x19\x82\x11\x15aD\xBAWaD\xBAaDiV[P\x01\x90V[`\xA0\x81\x01c\xFF\xFF\xFF\xFFaD\xD1\x84aC7V[\x16\x82R` \x83\x015` \x83\x01R`@\x83\x015`@\x83\x01R``\x83\x015``\x83\x01R`\x80\x83\x015`\x80\x83\x01R\x92\x91PPV[`\0a\x13\xA96\x83a>\xBAV[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aE4WaE4aDiV[\x02\x94\x93PPPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15aEWWaEWaDiV[P\x02\x90V[`\0`\0\x19\x82\x14\x15aEpWaEpaDiV[P`\x01\x01\x90V[`\0\x82\x82\x10\x15aE\x89WaE\x89aDiV[P\x03\x90V[`\0`\x01`\x01``\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15aE\xAEWaE\xAEaDiV[\x03\x93\x92PPPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aE\xCDW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aE\xE7W`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15aC\xABW`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aF\x16W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aF0W`\0\x80\xFD[` \x01\x91P`\x07\x81\x90\x1B6\x03\x82\x13\x15aC\xABW`\0\x80\xFD[\x805`\x01`\x01``\x1B\x03\x81\x16\x81\x14a7\xDAW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aFqW`\0\x80\xFD[a;\x92\x82aFHV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aF\x91W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aF\xABW`\0\x80\xFD[` \x01\x91P`\x06\x81\x90\x1B6\x03\x82\x13\x15aC\xABW`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aF\xDAW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aF\xF4W`\0\x80\xFD[` \x01\x91P``\x81\x026\x03\x82\x13\x15aC\xABW`\0\x80\xFD[`\0\x825`~\x19\x836\x03\x01\x81\x12aG!W`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x825`^\x19\x836\x03\x01\x81\x12aG!W`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aGXW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aGrW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15aC\xABW`\0\x80\xFD[` \x81R`\0c\xFF\xFF\xFF\xFF\x80aG\x9C\x85aC7V[\x16` \x84\x01R\x80aG\xAF` \x86\x01aC7V[\x16`@\x84\x01R\x80aG\xC2`@\x86\x01aC7V[\x16``\x84\x01RPaG\xD6``\x84\x01\x84aCfV[`\xE0`\x80\x85\x01RaG\xECa\x01\0\x85\x01\x82\x84aC\xB2V[\x91PPaG\xFB`\x80\x85\x01aC7V[c\xFF\xFF\xFF\xFF\x81\x16`\xA0\x85\x01RPaH\x15`\xA0\x85\x01\x85aCfV[\x84\x83\x03`\x1F\x19\x01`\xC0\x86\x01RaH,\x83\x82\x84aC\xB2V[\x92PPPaH<`\xC0\x85\x01aC7V[c\xFF\xFF\xFF\xFF\x81\x16`\xE0\x85\x01RP\x93\x92PPPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aHgW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aH\x86W`\0\x80\xFD[\x80`\x05\x1B6\x03\x83\x13\x15aC\xABW`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a@\x19W`\xFFaH\xBB\x83a;fV[\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aH\xA8V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aH\xE5W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aI\x04W`\0\x80\xFD[\x80`\x07\x1B6\x03\x83\x13\x15aC\xABW`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a@\x19W`\xFFaI9\x83a;fV[\x16\x87R`\x01`\x01``\x1B\x03aIO\x84\x84\x01aFHV[\x16\x83\x88\x01R`@\x82\x81\x015\x90\x88\x01R``\x80\x83\x015\x90\x88\x01R`\x80\x96\x87\x01\x96\x90\x91\x01\x90`\x01\x01aI&V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aI\x91W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aI\xB0W`\0\x80\xFD[\x80`\x06\x1B6\x03\x83\x13\x15aC\xABW`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a@\x19W`\xFFaI\xE5\x83a;fV[\x16\x87R`\x01`\x01``\x1B\x03aI\xFB\x84\x84\x01aFHV[\x16\x87\x84\x01R`@\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aI\xD2V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aJ*W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aJIW`\0\x80\xFD[``\x81\x026\x03\x83\x13\x15aC\xABW`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a@\x19W`\xFFaJ~\x83a;fV[\x16\x87RaJ\x99\x83\x88\x01\x84\x84\x01\x805\x82R` \x90\x81\x015\x91\x01RV[``\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aJkV[\x81\x83R`\0`\x01`\x01`\xFB\x1B\x03\x83\x11\x15aJ\xC5W`\0\x80\xFD[\x82`\x05\x1B\x80\x83` \x87\x017`\0\x94\x01` \x01\x93\x84RP\x91\x92\x91PPV[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a@\x19W`\x01`\x01``\x1B\x03aK\x0B\x83aFHV[\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aJ\xF2V[\x81\x83R`\0` \x80\x85\x01\x80\x81\x96P\x85`\x05\x1B\x81\x01\x91P\x84`\0\x80[\x88\x81\x10\x15aK\xCFW\x83\x85\x03\x8AR\x825`~\x19\x896\x03\x01\x81\x12aKYW\x82\x83\xFD[\x88\x01\x805\x86R`\x80aKm\x88\x83\x01\x83aHPV[\x82\x8A\x8A\x01RaK\x7F\x83\x8A\x01\x82\x84aH\x98V[\x92PPP`@aK\x91\x81\x84\x01\x84aHPV[\x89\x84\x03\x83\x8B\x01RaK\xA3\x84\x82\x84aJ\xE2V[\x93PPPP```\xFFaK\xB7\x82\x85\x01a;fV[\x16\x97\x01\x96\x90\x96RP\x98\x85\x01\x98\x91\x85\x01\x91`\x01\x01aK9V[P\x92\x98\x97PPPPPPPPV[\x81\x83R`\0` \x80\x85\x01\x80\x81\x96P\x85`\x05\x1B\x81\x01\x91P\x84`\0\x80[\x88\x81\x10\x15aK\xCFW\x83\x85\x03\x8AR\x825`^\x19\x896\x03\x01\x81\x12aL\x18W\x82\x83\xFD[\x88\x01\x805\x86R``aL,\x88\x83\x01\x83aHPV[\x82\x8A\x8A\x01RaL>\x83\x8A\x01\x82\x84aH\x98V[\x92PPP`@aLP\x81\x84\x01\x84aHPV[\x93P\x88\x83\x03\x82\x8A\x01RaLd\x83\x85\x83aJ\xE2V[\x9D\x8A\x01\x9D\x98PPP\x93\x87\x01\x93PP`\x01\x01aK\xF8V[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a@\x19W\x815\x87R`\xFFaL\xA3\x84\x84\x01a;fV[\x16\x87\x84\x01R`@\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aL\x8AV[` \x81RaL\xD5` \x82\x01aL\xCF\x84aA\x97V[\x15\x15\x90RV[`\0aL\xE4` \x84\x01\x84aHPV[a\x01 \x80`@\x86\x01RaL\xFCa\x01@\x86\x01\x83\x85aH\x98V[\x92PaM\x0B`@\x87\x01\x87aH\xCEV[\x92P`\x1F\x19\x80\x87\x86\x03\x01``\x88\x01RaM%\x85\x85\x84aI\x16V[\x94PaM4``\x89\x01\x89aIzV[\x94P\x91P\x80\x87\x86\x03\x01`\x80\x88\x01RaMM\x85\x85\x84aI\xC2V[\x94PaM\\`\x80\x89\x01\x89aJ\x13V[\x94P\x91P\x80\x87\x86\x03\x01`\xA0\x88\x01RaMu\x85\x85\x84aJ[V[\x94PaM\x84`\xA0\x89\x01\x89aHPV[\x94P\x91P\x80\x87\x86\x03\x01`\xC0\x88\x01RaM\x9D\x85\x85\x84aJ\xACV[\x94PaM\xAC`\xC0\x89\x01\x89aHPV[\x94P\x91P\x80\x87\x86\x03\x01`\xE0\x88\x01RaM\xC5\x85\x85\x84aK\x1EV[\x94PaM\xD4`\xE0\x89\x01\x89aHPV[\x94P\x91Pa\x01\0\x81\x88\x87\x03\x01\x81\x89\x01RaM\xEF\x86\x86\x85aK\xDDV[\x95PaM\xFD\x81\x8A\x01\x8AaIzV[\x95P\x92PP\x80\x87\x86\x03\x01\x83\x88\x01RPaN\x17\x84\x84\x83aLzV[\x97\x96PPPPPPPV[``\x81\x01c\xFF\xFF\xFF\xFFaN4\x84aC7V[\x16\x82R` \x83\x015` \x83\x01R`@\x83\x015`@\x83\x01R\x92\x91PPV\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 e't\x04\x93?\x01/'\xFC\xFEQ\x03E\xB5\xC4LeJ!l&\"\xE8\xF9\xE9\xA4\xEDL\xE3\x96\xCAdsolcC\0\x08\x0C\x003";
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
        ///Calls the contract's `allowNonRootInit` (0x0ee0fdbd) function
        pub fn allow_non_root_init(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([14, 224, 253, 189], ())
                .expect("method not found (this should never happen)")
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
        ///Calls the contract's `initialize` (0xfecf9734) function
        pub fn initialize(
            &self,
            pauser_registry: ::ethers::core::types::Address,
            initial_owner: ::ethers::core::types::Address,
            updater: ::ethers::core::types::Address,
            allow_non_root_init: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [254, 207, 151, 52],
                    (pauser_registry, initial_owner, updater, allow_non_root_init),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastOpUpdateBlockTimestamp` (0xf84e91fc) function
        pub fn last_op_update_block_timestamp(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([248, 78, 145, 252], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `latestCompletedOpTaskCreatedBlock` (0x6f0c30a4) function
        pub fn latest_completed_op_task_created_block(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([111, 12, 48, 164], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `latestCompletedOpTaskNumber` (0xe2a7cb66) function
        pub fn latest_completed_op_task_number(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([226, 167, 203, 102], ())
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
        ///Calls the contract's `process_eigen_op_update` (0xdb530d63) function
        pub fn process_eigen_op_update(
            &self,
            task: OpTask,
            task_response: OpTaskResponse,
            non_signer_stakes_and_signature_for_old_state: NonSignerStakesAndSignatureForOldState,
            operator_state_info: OperatorStateInfo,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [219, 83, 13, 99],
                    (
                        task,
                        task_response,
                        non_signer_stakes_and_signature_for_old_state,
                        operator_state_info,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `process_eigen_rd_update` (0xa11df14c) function
        pub fn process_eigen_rd_update(
            &self,
            task: RdTask,
            task_response: RdTaskResponse,
            non_signer_stakes_and_signature_for_old_state: NonSignerStakesAndSignatureForOldState,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [161, 29, 241, 76],
                    (task, task_response, non_signer_stakes_and_signature_for_old_state),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `process_eigen_reinit` (0xd093867f) function
        pub fn process_eigen_reinit(
            &self,
            task: OpTask,
            operator_state_info: OperatorStateInfo,
            pending_state_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [208, 147, 134, 127],
                    (task, operator_state_info, pending_state_hash),
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
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([223, 3, 76, 208], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `EigenOpUpdateProcessed` event
        pub fn eigen_op_update_processed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EigenOpUpdateProcessedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `EigenRdUpdateProcessed` event
        pub fn eigen_rd_update_processed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EigenRdUpdateProcessedFilter,
        > {
            self.0.event()
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
        name = "EigenOpUpdateProcessed",
        abi = "EigenOpUpdateProcessed(uint32,uint32)"
    )]
    pub struct EigenOpUpdateProcessedFilter {
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
        name = "EigenRdUpdateProcessed",
        abi = "EigenRdUpdateProcessed(uint32,uint32)"
    )]
    pub struct EigenRdUpdateProcessedFilter {
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
        EigenOpUpdateProcessedFilter(EigenOpUpdateProcessedFilter),
        EigenRdUpdateProcessedFilter(EigenRdUpdateProcessedFilter),
        EigenReinitProcessedFilter(EigenReinitProcessedFilter),
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
            if let Ok(decoded) = EigenOpUpdateProcessedFilter::decode_log(log) {
                return Ok(
                    GaspMultiRollupServiceEvents::EigenOpUpdateProcessedFilter(decoded),
                );
            }
            if let Ok(decoded) = EigenRdUpdateProcessedFilter::decode_log(log) {
                return Ok(
                    GaspMultiRollupServiceEvents::EigenRdUpdateProcessedFilter(decoded),
                );
            }
            if let Ok(decoded) = EigenReinitProcessedFilter::decode_log(log) {
                return Ok(
                    GaspMultiRollupServiceEvents::EigenReinitProcessedFilter(decoded),
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
                Self::EigenOpUpdateProcessedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EigenRdUpdateProcessedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EigenReinitProcessedFilter(element) => {
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
    impl ::core::convert::From<EigenOpUpdateProcessedFilter>
    for GaspMultiRollupServiceEvents {
        fn from(value: EigenOpUpdateProcessedFilter) -> Self {
            Self::EigenOpUpdateProcessedFilter(value)
        }
    }
    impl ::core::convert::From<EigenRdUpdateProcessedFilter>
    for GaspMultiRollupServiceEvents {
        fn from(value: EigenRdUpdateProcessedFilter) -> Self {
            Self::EigenRdUpdateProcessedFilter(value)
        }
    }
    impl ::core::convert::From<EigenReinitProcessedFilter>
    for GaspMultiRollupServiceEvents {
        fn from(value: EigenReinitProcessedFilter) -> Self {
            Self::EigenReinitProcessedFilter(value)
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
    ///Container type for all input parameters for the `allowNonRootInit` function with signature `allowNonRootInit()` and selector `0x0ee0fdbd`
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
    #[ethcall(name = "allowNonRootInit", abi = "allowNonRootInit()")]
    pub struct AllowNonRootInitCall;
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
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address,address,bool)` and selector `0xfecf9734`
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
    #[ethcall(name = "initialize", abi = "initialize(address,address,address,bool)")]
    pub struct InitializeCall {
        pub pauser_registry: ::ethers::core::types::Address,
        pub initial_owner: ::ethers::core::types::Address,
        pub updater: ::ethers::core::types::Address,
        pub allow_non_root_init: bool,
    }
    ///Container type for all input parameters for the `lastOpUpdateBlockTimestamp` function with signature `lastOpUpdateBlockTimestamp()` and selector `0xf84e91fc`
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
    #[ethcall(name = "lastOpUpdateBlockTimestamp", abi = "lastOpUpdateBlockTimestamp()")]
    pub struct LastOpUpdateBlockTimestampCall;
    ///Container type for all input parameters for the `latestCompletedOpTaskCreatedBlock` function with signature `latestCompletedOpTaskCreatedBlock()` and selector `0x6f0c30a4`
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
        name = "latestCompletedOpTaskCreatedBlock",
        abi = "latestCompletedOpTaskCreatedBlock()"
    )]
    pub struct LatestCompletedOpTaskCreatedBlockCall;
    ///Container type for all input parameters for the `latestCompletedOpTaskNumber` function with signature `latestCompletedOpTaskNumber()` and selector `0xe2a7cb66`
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
        name = "latestCompletedOpTaskNumber",
        abi = "latestCompletedOpTaskNumber()"
    )]
    pub struct LatestCompletedOpTaskNumberCall;
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
    ///Container type for all input parameters for the `process_eigen_op_update` function with signature `process_eigen_op_update((uint32,uint32,uint32,bytes,uint32,bytes,uint32),(uint32,bytes32,bytes32),((uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256)),(bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]))` and selector `0xdb530d63`
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
        name = "process_eigen_op_update",
        abi = "process_eigen_op_update((uint32,uint32,uint32,bytes,uint32,bytes,uint32),(uint32,bytes32,bytes32),((uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256)),(bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]))"
    )]
    pub struct ProcessEigenOpUpdateCall {
        pub task: OpTask,
        pub task_response: OpTaskResponse,
        pub non_signer_stakes_and_signature_for_old_state: NonSignerStakesAndSignatureForOldState,
        pub operator_state_info: OperatorStateInfo,
    }
    ///Container type for all input parameters for the `process_eigen_rd_update` function with signature `process_eigen_rd_update((uint32,uint256,uint32,uint32,bytes,uint32),(uint32,bytes32,bytes32,bytes32,bytes32),((uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256)))` and selector `0xa11df14c`
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
        name = "process_eigen_rd_update",
        abi = "process_eigen_rd_update((uint32,uint256,uint32,uint32,bytes,uint32),(uint32,bytes32,bytes32,bytes32,bytes32),((uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256)))"
    )]
    pub struct ProcessEigenRdUpdateCall {
        pub task: RdTask,
        pub task_response: RdTaskResponse,
        pub non_signer_stakes_and_signature_for_old_state: NonSignerStakesAndSignatureForOldState,
    }
    ///Container type for all input parameters for the `process_eigen_reinit` function with signature `process_eigen_reinit((uint32,uint32,uint32,bytes,uint32,bytes,uint32),(bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]),bytes32)` and selector `0xd093867f`
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
        abi = "process_eigen_reinit((uint32,uint32,uint32,bytes,uint32,bytes,uint32),(bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]),bytes32)"
    )]
    pub struct ProcessEigenReinitCall {
        pub task: OpTask,
        pub operator_state_info: OperatorStateInfo,
        pub pending_state_hash: [u8; 32],
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
        AllowNonRootInit(AllowNonRootInitCall),
        CheckSignatures(CheckSignaturesCall),
        Initialize(InitializeCall),
        LastOpUpdateBlockTimestamp(LastOpUpdateBlockTimestampCall),
        LatestCompletedOpTaskCreatedBlock(LatestCompletedOpTaskCreatedBlockCall),
        LatestCompletedOpTaskNumber(LatestCompletedOpTaskNumberCall),
        LatestPendingStateHash(LatestPendingStateHashCall),
        OperatorAndQuorumToStakes(OperatorAndQuorumToStakesCall),
        OperatorIdQuorumCount(OperatorIdQuorumCountCall),
        Owner(OwnerCall),
        Pause(PauseCall),
        PauseAll(PauseAllCall),
        PausedWithIndex(PausedWithIndexCall),
        Paused(PausedCall),
        PauserRegistry(PauserRegistryCall),
        ProcessEigenOpUpdate(ProcessEigenOpUpdateCall),
        ProcessEigenRdUpdate(ProcessEigenRdUpdateCall),
        ProcessEigenReinit(ProcessEigenReinitCall),
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
            if let Ok(decoded) = <AllowNonRootInitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AllowNonRootInit(decoded));
            }
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
            if let Ok(decoded) = <LastOpUpdateBlockTimestampCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LastOpUpdateBlockTimestamp(decoded));
            }
            if let Ok(decoded) = <LatestCompletedOpTaskCreatedBlockCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LatestCompletedOpTaskCreatedBlock(decoded));
            }
            if let Ok(decoded) = <LatestCompletedOpTaskNumberCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LatestCompletedOpTaskNumber(decoded));
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
            if let Ok(decoded) = <ProcessEigenOpUpdateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProcessEigenOpUpdate(decoded));
            }
            if let Ok(decoded) = <ProcessEigenRdUpdateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProcessEigenRdUpdate(decoded));
            }
            if let Ok(decoded) = <ProcessEigenReinitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProcessEigenReinit(decoded));
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
            if let Ok(decoded) = <SetPauserRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetPauserRegistry(decoded));
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
                Self::AllowNonRootInit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CheckSignatures(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LastOpUpdateBlockTimestamp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LatestCompletedOpTaskCreatedBlock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LatestCompletedOpTaskNumber(element) => {
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
                Self::ProcessEigenOpUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProcessEigenRdUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProcessEigenReinit(element) => {
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
                Self::SetPauserRegistry(element) => {
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
                Self::AllowNonRootInit(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckSignatures(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::LastOpUpdateBlockTimestamp(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LatestCompletedOpTaskCreatedBlock(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LatestCompletedOpTaskNumber(element) => {
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
                Self::ProcessEigenOpUpdate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProcessEigenRdUpdate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProcessEigenReinit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::QourumApk(element) => ::core::fmt::Display::fmt(element, f),
                Self::QuorumNumbers(element) => ::core::fmt::Display::fmt(element, f),
                Self::QuorumThresholdPercentage(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
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
    impl ::core::convert::From<AllowNonRootInitCall> for GaspMultiRollupServiceCalls {
        fn from(value: AllowNonRootInitCall) -> Self {
            Self::AllowNonRootInit(value)
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
    impl ::core::convert::From<LastOpUpdateBlockTimestampCall>
    for GaspMultiRollupServiceCalls {
        fn from(value: LastOpUpdateBlockTimestampCall) -> Self {
            Self::LastOpUpdateBlockTimestamp(value)
        }
    }
    impl ::core::convert::From<LatestCompletedOpTaskCreatedBlockCall>
    for GaspMultiRollupServiceCalls {
        fn from(value: LatestCompletedOpTaskCreatedBlockCall) -> Self {
            Self::LatestCompletedOpTaskCreatedBlock(value)
        }
    }
    impl ::core::convert::From<LatestCompletedOpTaskNumberCall>
    for GaspMultiRollupServiceCalls {
        fn from(value: LatestCompletedOpTaskNumberCall) -> Self {
            Self::LatestCompletedOpTaskNumber(value)
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
    impl ::core::convert::From<ProcessEigenOpUpdateCall>
    for GaspMultiRollupServiceCalls {
        fn from(value: ProcessEigenOpUpdateCall) -> Self {
            Self::ProcessEigenOpUpdate(value)
        }
    }
    impl ::core::convert::From<ProcessEigenRdUpdateCall>
    for GaspMultiRollupServiceCalls {
        fn from(value: ProcessEigenRdUpdateCall) -> Self {
            Self::ProcessEigenRdUpdate(value)
        }
    }
    impl ::core::convert::From<ProcessEigenReinitCall> for GaspMultiRollupServiceCalls {
        fn from(value: ProcessEigenReinitCall) -> Self {
            Self::ProcessEigenReinit(value)
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
    ///Container type for all return fields from the `allowNonRootInit` function with signature `allowNonRootInit()` and selector `0x0ee0fdbd`
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
    pub struct AllowNonRootInitReturn(pub bool);
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
    ///Container type for all return fields from the `lastOpUpdateBlockTimestamp` function with signature `lastOpUpdateBlockTimestamp()` and selector `0xf84e91fc`
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
    pub struct LastOpUpdateBlockTimestampReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `latestCompletedOpTaskCreatedBlock` function with signature `latestCompletedOpTaskCreatedBlock()` and selector `0x6f0c30a4`
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
    pub struct LatestCompletedOpTaskCreatedBlockReturn(pub u32);
    ///Container type for all return fields from the `latestCompletedOpTaskNumber` function with signature `latestCompletedOpTaskNumber()` and selector `0xe2a7cb66`
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
    pub struct LatestCompletedOpTaskNumberReturn(pub u32);
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
    ///`NonSignerStakesAndSignatureForOldState((uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256))`
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
    pub struct NonSignerStakesAndSignatureForOldState {
        pub non_signer_g1_pubkeys_for_old_state: ::std::vec::Vec<G1Point>,
        pub apk_g2_for_old_state: G2Point,
        pub sigma_for_old_state: G1Point,
    }
}
