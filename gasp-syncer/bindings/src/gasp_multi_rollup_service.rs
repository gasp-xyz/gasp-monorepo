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
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                ),
                                            ),
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
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IBLSSignatureChecker.NonSignerStakesAndSignature",
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
                    ::std::borrow::ToOwned::to_owned("latestCompletedRdTaskNumber"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "latestCompletedRdTaskNumber",
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
                                        "nonSignerStakesAndSignature",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                ),
                                            ),
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
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IBLSSignatureChecker.NonSignerStakesAndSignature",
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
                                        "nonSignerStakesAndSignature",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                ),
                                            ),
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
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IBLSSignatureChecker.NonSignerStakesAndSignature",
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
    pub static GASPMULTIROLLUPSERVICE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[PaP\xF3\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xE5W`\x005`\xE0\x1C\x80c\\\x97Z\xBB\x11a\x01\x0FW\x80c\xD0:\x07\xB2\x11a\0\xA2W\x80c\xF2\xFD\xE3\x8B\x11a\0qW\x80c\xF2\xFD\xE3\x8B\x14a\x04\xDDW\x80c\xF8N\x91\xFC\x14a\x04\xF0W\x80c\xFA\xBC\x1C\xBC\x14a\x04\xF9W\x80c\xFE\xCF\x974\x14a\x05\x0CW`\0\x80\xFD[\x80c\xD0:\x07\xB2\x14a\x04\x97W\x80c\xD0\x93\x86\x7F\x14a\x04\xA7W\x80c\xDF\x03L\xD0\x14a\x04\xBAW\x80c\xE2\xA7\xCBf\x14a\x04\xCDW`\0\x80\xFD[\x80cz\xD7Ua\x11a\0\xDEW\x80cz\xD7Ua\x14a\x04\x12W\x80c}\x97\x88\x97\x14a\x04;W\x80c\x88o\x11\x95\x14a\x04[W\x80c\x8D\xA5\xCB[\x14a\x04\x86W`\0\x80\xFD[\x80c\\\x97Z\xBB\x14a\x03\xD8W\x80co\x0C0\xA4\x14a\x03\xE0W\x80cqP\x18\xA6\x14a\x03\xF7W\x80cy\xA0\xA8S\x14a\x03\xFFW`\0\x80\xFD[\x80c*\x84\x14\xFD\x11a\x01\x87W\x80cM\xEA\xBC!\x11a\x01VW\x80cM\xEA\xBC!\x14a\x03tW\x80cRn>d\x14a\x03\x99W\x80cY\\jg\x14a\x03\xADW\x80cZ\xC8j\xB7\x14a\x03\xB5W`\0\x80\xFD[\x80c*\x84\x14\xFD\x14a\x02\xC7W\x80cC\r;9\x14a\x02\xDCW\x80cI\x9Do\xB6\x14a\x03\x11W\x80cJ\xE6\xB2\x03\x14a\x03]W`\0\x80\xFD[\x80c\x12FH\xC9\x11a\x01\xC3W\x80c\x12FH\xC9\x14a\x02dW\x80c\x13d9\xDD\x14a\x02wW\x80c\x17\x1F\x1D[\x14a\x02\x8AW\x80c#+\x8E\x98\x14a\x02\xB4W`\0\x80\xFD[\x80c\x03\xD0\x97\xD2\x14a\x01\xEAW\x80c\x0E\xE0\xFD\xBD\x14a\x02+W\x80c\x10\xD6z/\x14a\x02OW[`\0\x80\xFD[a\x02\x11a\x01\xF86`\x04a;\xF3V[`\xA0` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T\x82V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[`\x97Ta\x02?\x90`\x01`\xA8\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02\"V[a\x02ba\x02]6`\x04a<*V[a\x05\x1FV[\0[a\x02ba\x02r6`\x04a<*V[a\x05\xDBV[a\x02ba\x02\x856`\x04a<GV[a\x06\x05V[a\x02\x9Da\x02\x986`\x04a=\xC5V[a\x07DV[`@\x80Q\x92\x15\x15\x83R\x90\x15\x15` \x83\x01R\x01a\x02\"V[a\x02ba\x02\xC26`\x04a>TV[a\x08\xCEV[a\x02\xCFa\x17\x8BV[`@Qa\x02\"\x91\x90a>\xF8V[a\x02\xFFa\x02\xEA6`\x04a<GV[`\xA1` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02\"V[a\x03Ea\x03\x1F6`\x04a?MV[`\x9F` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\x01`\x01``\x1B\x03\x16\x81V[`@Q`\x01`\x01``\x1B\x03\x90\x91\x16\x81R` \x01a\x02\"V[a\x03f`\x9AT\x81V[`@Q\x90\x81R` \x01a\x02\"V[`\x9DTa\x03\x84\x90c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\"V[`\x97Ta\x02?\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[a\x02ba\x18\x19V[a\x02?a\x03\xC36`\x04a;\xF3V[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`fTa\x03fV[`\x9BTa\x03\x84\x90`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x02ba\x18\xE0V[a\x02ba\x04\r6`\x04a?yV[a\x18\xF4V[a\x03Ea\x04 6`\x04a;\xF3V[`\x9E` R`\0\x90\x81R`@\x90 T`\x01`\x01``\x1B\x03\x16\x81V[a\x04Na\x04I6`\x04aB\xB7V[a\x1C\xD9V[`@Qa\x02\"\x91\x90aCAV[`eTa\x04n\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\"V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x04nV[`\x99Ta\x03\x84\x90c\xFF\xFF\xFF\xFF\x16\x81V[a\x02ba\x04\xB56`\x04aC\x83V[a\"\xC3V[`\x97Ta\x04n\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x9BTa\x03\x84\x90c\xFF\xFF\xFF\xFF\x16\x81V[a\x02ba\x04\xEB6`\x04a<*V[a-\x83V[a\x03f`\x98T\x81V[a\x02ba\x05\x076`\x04a<GV[a-\xF9V[a\x02ba\x05\x1A6`\x04aD\x08V[a/UV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x96\x91\x90aDdV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x05\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xC6\x90aD\x81V[`@Q\x80\x91\x03\x90\xFD[a\x05\xD8\x81a0\x9AV[PV[a\x05\xE3a1\x91V[`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06MW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06q\x91\x90aD\xCBV[a\x06\x8DW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xC6\x90aD\xE8V[`fT\x81\x81\x16\x14a\x07\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\xC6V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`\0\x80`\0\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x87\x87`\0\x01Q\x88` \x01Q\x88`\0\x01Q`\0`\x02\x81\x10a\x07\x8CWa\x07\x8CaE0V[` \x02\x01Q\x89Q`\x01` \x02\x01Q\x8A` \x01Q`\0`\x02\x81\x10a\x07\xB1Wa\x07\xB1aE0V[` \x02\x01Q\x8B` \x01Q`\x01`\x02\x81\x10a\x07\xCDWa\x07\xCDaE0V[` \x90\x81\x02\x91\x90\x91\x01Q\x8CQ\x8D\x83\x01Q`@Qa\x08*\x9A\x99\x98\x97\x96\x95\x94\x01\x98\x89R` \x89\x01\x97\x90\x97R`@\x88\x01\x95\x90\x95R``\x87\x01\x93\x90\x93R`\x80\x86\x01\x91\x90\x91R`\xA0\x85\x01R`\xC0\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01Ra\x01 \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\0\x1Ca\x08M\x91\x90aEFV[\x90Pa\x08\xC0a\x08fa\x08_\x88\x84a1\xEBV[\x86\x90a2\x82V[a\x08na3\x16V[a\x08\xB6a\x08\xA7\x85a\x08\xA1`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x01\x82R`\x02\x90\x82\x01R\x90V[\x90a1\xEBV[a\x08\xB0\x8Ca3\xD6V[\x90a2\x82V[\x88b\x01\xD4\xC0a4fV[\x90\x98\x90\x97P\x95PPPPPPV[`\x9BT`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15\x80\x15a\t\x80W`\x97T`\x01`\xA8\x1B\x90\x04`\xFF\x16\x15a\t>W`\x97T`\x01`\x01`\xA0\x1B\x03\x163\x14a\t9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04\x17WF\x83`\xDC\x1B`D\x82\x01R`d\x01a\x05\xC6V[a\t\xC2V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\t9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd \xBA\xBA4\x19`\xD9\x1B`D\x82\x01R`d\x01a\x05\xC6V[`\x97T`\x01`\x01`\xA0\x1B\x03\x163\x14a\t\xC2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04\x17WF\x83`\xDC\x1B`D\x82\x01R`d\x01a\x05\xC6V[\x84`@Q` \x01a\t\xD3\x91\x90aE\xDDV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x015\x14a\n;W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FreferenceTaskHash hash mismatch\0`D\x82\x01R`d\x01a\x05\xC6V[\x81`@Q` \x01a\nL\x91\x90aK\"V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84`@\x015\x14a\n\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FoperatorStateInfo hash mismatch\0`D\x82\x01R`d\x01a\x05\xC6V[\x80a\x0C\xD4Wa\n\xC9``\x86\x01`@\x87\x01aL\x89V[`\x9BT`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14a\x0B*W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7Freference block mismatch\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\xC6V[a\x0B:`@\x86\x01` \x87\x01aL\x89V[`\x9BTc\xFF\xFF\xFF\xFF\x91\x82\x16\x91a\x0BZ\x91`\x01` \x1B\x90\x04\x16a8@aL\xBAV[c\xFF\xFF\xFF\xFF\x16\x11a\x0B\x9DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x077F\x16\xC6R\x077F\x17FR\x03`\x9C\x1B`D\x82\x01R`d\x01a\x05\xC6V[B`\x98Tb\x03\xF4\x80a\x0B\xAF\x91\x90aL\xE2V[\x11a\x0B\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rlstale state 1`\x98\x1B`D\x82\x01R`d\x01a\x05\xC6V[`\0a\x0C\"\x85`@Q` \x01a\x0C\x02\x91\x90aL\xFAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x85a\x04I\x90aM)V[`\x9DT\x90\x91Pc\xFF\xFF\xFF\xFF\x16`\0[`\x9C\x80Ta\x0C>\x90aM5V[\x90P\x81\x10\x15a\x0C\xD0W\x81`\xFF\x16\x83` \x01Q\x82\x81Q\x81\x10a\x0CaWa\x0CaaE0V[` \x02` \x01\x01Qa\x0Cs\x91\x90aMjV[`\x01`\x01``\x1B\x03\x16`d\x84`\0\x01Q\x83\x81Q\x81\x10a\x0C\x94Wa\x0C\x94aE0V[` \x02` \x01\x01Q`\x01`\x01``\x1B\x03\x16a\x0C\xAF\x91\x90aM\x99V[\x10\x15a\x0C\xBEWPPPPa\x17\x85V[\x80a\x0C\xC8\x81aM\xB8V[\x91PPa\x0C1V[PPP[`\0[a\x0C\xE4` \x84\x01\x84aM\xD3V[\x90P\x81\x10\x15a\r\xA9W`\x9E`\0a\x0C\xFE` \x86\x01\x86aM\xD3V[\x84\x81\x81\x10a\r\x0EWa\r\x0EaE0V[\x90P` \x02\x01` \x81\x01\x90a\r#\x91\x90a;\xF3V[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0\x90\x81 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U`\xA0\x91a\rV\x90\x86\x01\x86aM\xD3V[\x84\x81\x81\x10a\rfWa\rfaE0V[\x90P` \x02\x01` \x81\x01\x90a\r{\x91\x90a;\xF3V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x81\x81U`\x01\x01U\x80a\r\xA1\x81aM\xB8V[\x91PPa\x0C\xD7V[P`\0[a\r\xBA`@\x84\x01\x84aN\x1CV[\x90P\x81\x10\x15a\x0E\xF5Wa\r\xD0`@\x84\x01\x84aN\x1CV[\x82\x81\x81\x10a\r\xE0Wa\r\xE0aE0V[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\r\xF8\x91\x90aNeV[`\x9E`\0a\x0E\t`@\x87\x01\x87aN\x1CV[\x85\x81\x81\x10a\x0E\x19Wa\x0E\x19aE0V[a\x0E/\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa;\xF3V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x90\x81\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x17\x90\x91Ua\x0Eo\x90\x84\x01\x84aN\x1CV[\x82\x81\x81\x10a\x0E\x7FWa\x0E\x7FaE0V[\x90P`\x80\x02\x01`@\x01`\xA0`\0\x85\x80`@\x01\x90a\x0E\x9C\x91\x90aN\x1CV[\x85\x81\x81\x10a\x0E\xACWa\x0E\xACaE0V[a\x0E\xC2\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa;\xF3V[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a\x0E\xED\x81aM\xB8V[\x91PPa\r\xADV[P`\0[a\x0F\x06``\x84\x01\x84aN\x80V[\x90P\x81\x10\x15a\x0F\xBEWa\x0F\x1C``\x84\x01\x84aN\x80V[\x82\x81\x81\x10a\x0F,Wa\x0F,aE0V[\x90P`@\x02\x01` \x01` \x81\x01\x90a\x0FD\x91\x90aNeV[`\x9E`\0a\x0FU``\x87\x01\x87aN\x80V[\x85\x81\x81\x10a\x0FeWa\x0FeaE0V[a\x0F{\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa;\xF3V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\x0F\xB6\x81aM\xB8V[\x91PPa\x0E\xF9V[P`\0[a\x0F\xCF`\x80\x84\x01\x84aN\xC9V[\x90P\x81\x10\x15a\x10kWa\x0F\xE5`\x80\x84\x01\x84aN\xC9V[\x82\x81\x81\x10a\x0F\xF5Wa\x0F\xF5aE0V[\x90P``\x02\x01` \x01`\xA0`\0\x85\x80`\x80\x01\x90a\x10\x12\x91\x90aN\xC9V[\x85\x81\x81\x10a\x10\"Wa\x10\"aE0V[a\x108\x92` ``\x90\x92\x02\x01\x90\x81\x01\x91Pa;\xF3V[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a\x10c\x81aM\xB8V[\x91PPa\x0F\xC2V[P`\0[a\x10|`\xA0\x84\x01\x84aM\xD3V[\x90P\x81\x10\x15a\x11\xA3W`\0[`\x9C\x80Ta\x10\x95\x90aM5V[\x90P\x81\x10\x15a\x11NW`\x9F`\0a\x10\xAF`\xA0\x87\x01\x87aM\xD3V[\x85\x81\x81\x10a\x10\xBFWa\x10\xBFaE0V[\x90P` \x02\x015\x81R` \x01\x90\x81R` \x01`\0 `\0`\x9C\x83\x81Ta\x10\xE4\x90aM5V[\x81\x10a\x10\xF2Wa\x10\xF2aE0V[\x81T`\x01\x16\x15a\x11\x11W\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U\x81a\x11F\x81aM\xB8V[\x92PPa\x10\x88V[P`\xA1`\0a\x11``\xA0\x86\x01\x86aM\xD3V[\x84\x81\x81\x10a\x11pWa\x11paE0V[` \x90\x81\x02\x92\x90\x92\x015\x83RP\x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\xFF\x19\x16\x90U\x80a\x11\x9B\x81aM\xB8V[\x91PPa\x10oV[P`\0[a\x11\xB4`\xC0\x84\x01\x84aM\xD3V[\x90P\x81\x10\x15a\x14\x04Wa\x11\xCA`\xC0\x84\x01\x84aM\xD3V[\x82\x81\x81\x10a\x11\xDAWa\x11\xDAaE0V[\x90P` \x02\x81\x01\x90a\x11\xEC\x91\x90aO\x11V[a\x11\xFD\x90`\x80\x81\x01\x90``\x01a;\xF3V[`\xA1`\0a\x12\x0E`\xC0\x87\x01\x87aM\xD3V[\x85\x81\x81\x10a\x12\x1EWa\x12\x1EaE0V[\x90P` \x02\x81\x01\x90a\x120\x91\x90aO\x11V[`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP`\0[a\x12j`\xC0\x85\x01\x85aM\xD3V[\x83\x81\x81\x10a\x12zWa\x12zaE0V[\x90P` \x02\x81\x01\x90a\x12\x8C\x91\x90aO\x11V[a\x12\x9A\x90` \x81\x01\x90aM\xD3V[\x90P\x81\x10\x15a\x13\xF1Wa\x12\xB0`\xC0\x85\x01\x85aM\xD3V[\x83\x81\x81\x10a\x12\xC0Wa\x12\xC0aE0V[\x90P` \x02\x81\x01\x90a\x12\xD2\x91\x90aO\x11V[a\x12\xE0\x90`@\x81\x01\x90aM\xD3V[\x82\x81\x81\x10a\x12\xF0Wa\x12\xF0aE0V[\x90P` \x02\x01` \x81\x01\x90a\x13\x05\x91\x90aNeV[`\x9F`\0a\x13\x16`\xC0\x88\x01\x88aM\xD3V[\x86\x81\x81\x10a\x13&Wa\x13&aE0V[\x90P` \x02\x81\x01\x90a\x138\x91\x90aO\x11V[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a\x13Y`\xC0\x88\x01\x88aM\xD3V[\x86\x81\x81\x10a\x13iWa\x13iaE0V[\x90P` \x02\x81\x01\x90a\x13{\x91\x90aO\x11V[a\x13\x89\x90` \x81\x01\x90aM\xD3V[\x85\x81\x81\x10a\x13\x99Wa\x13\x99aE0V[\x90P` \x02\x01` \x81\x01\x90a\x13\xAE\x91\x90a;\xF3V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\x13\xE9\x81aM\xB8V[\x91PPa\x12]V[P\x80a\x13\xFC\x81aM\xB8V[\x91PPa\x11\xA7V[P`\0[a\x14\x15`\xE0\x84\x01\x84aM\xD3V[\x90P\x81\x10\x15a\x15\xC8W`\0[a\x14.`\xE0\x85\x01\x85aM\xD3V[\x83\x81\x81\x10a\x14>Wa\x14>aE0V[\x90P` \x02\x81\x01\x90a\x14P\x91\x90aO1V[a\x14^\x90` \x81\x01\x90aM\xD3V[\x90P\x81\x10\x15a\x15\xB5Wa\x14t`\xE0\x85\x01\x85aM\xD3V[\x83\x81\x81\x10a\x14\x84Wa\x14\x84aE0V[\x90P` \x02\x81\x01\x90a\x14\x96\x91\x90aO1V[a\x14\xA4\x90`@\x81\x01\x90aM\xD3V[\x82\x81\x81\x10a\x14\xB4Wa\x14\xB4aE0V[\x90P` \x02\x01` \x81\x01\x90a\x14\xC9\x91\x90aNeV[`\x9F`\0a\x14\xDA`\xE0\x88\x01\x88aM\xD3V[\x86\x81\x81\x10a\x14\xEAWa\x14\xEAaE0V[\x90P` \x02\x81\x01\x90a\x14\xFC\x91\x90aO1V[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a\x15\x1D`\xE0\x88\x01\x88aM\xD3V[\x86\x81\x81\x10a\x15-Wa\x15-aE0V[\x90P` \x02\x81\x01\x90a\x15?\x91\x90aO1V[a\x15M\x90` \x81\x01\x90aM\xD3V[\x85\x81\x81\x10a\x15]Wa\x15]aE0V[\x90P` \x02\x01` \x81\x01\x90a\x15r\x91\x90a;\xF3V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\x15\xAD\x81aM\xB8V[\x91PPa\x14!V[P\x80a\x15\xC0\x81aM\xB8V[\x91PPa\x14\x08V[P`\0[a\x15\xDAa\x01\0\x84\x01\x84aN\x80V[\x90P\x81\x10\x15a\x16~Wa\x15\xF1a\x01\0\x84\x01\x84aN\x80V[\x82\x81\x81\x10a\x16\x01Wa\x16\x01aE0V[\x90P`@\x02\x01` \x01` \x81\x01\x90a\x16\x19\x91\x90a;\xF3V[`\xA1`\0a\x16+a\x01\0\x87\x01\x87aN\x80V[\x85\x81\x81\x10a\x16;Wa\x16;aE0V[\x90P`@\x02\x01`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP\x80\x80a\x16v\x90aM\xB8V[\x91PPa\x15\xCCV[Pa\x16\x8C` \x86\x01\x86aL\x89V[`\x9B\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\x16\xB6`@\x86\x01` \x87\x01aL\x89V[`\x9B\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90UB`\x98Ua\x16\xED``\x86\x01\x86aOGV[a\x16\xF9\x91`\x9C\x91a:oV[Pa\x17\n`\xA0\x86\x01`\x80\x87\x01aL\x89V[`\x9D\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x7F6\xA1\xFD{\xD5T\xF5\xC4(\xC9\x82\x9C\t\xC6`kL\x89;\x1F\xAD\xC8sZz\x12yW\x97D}\xEDa\x17R` \x87\x01\x87aL\x89V[a\x17b`@\x88\x01` \x89\x01aL\x89V[`@\x80Qc\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01[`@Q\x80\x91\x03\x90\xA1P[PPPPV[`\x9C\x80Ta\x17\x98\x90aM5V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x17\xC4\x90aM5V[\x80\x15a\x18\x11W\x80`\x1F\x10a\x17\xE6Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x18\x11V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x17\xF4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18aW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\x85\x91\x90aD\xCBV[a\x18\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xC6\x90aD\xE8V[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[a\x18\xE8a1\x91V[a\x18\xF2`\0a6\x8AV[V[`\x97T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x19NW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FUpdater must be the caller\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\xC6V[`\x99Tc\xFF\xFF\xFF\xFF\x16\x15\x80a\x19zWPa\x19k` \x84\x01\x84aL\x89V[`\x99Tc\xFF\xFF\xFF\xFF\x91\x82\x16\x91\x16\x10[a\x19\xB5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkStale RdTask`\xA0\x1B`D\x82\x01R`d\x01a\x05\xC6V[`\x9BT`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16a\x1A\x03W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x13\xDC\x08\x1C\xDD\x18]\x19H\x1D[\x9A[\x9A]`\x8A\x1B`D\x82\x01R`d\x01a\x05\xC6V[a\x1A\x13`\x80\x84\x01``\x85\x01aL\x89V[`\x9BT`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14a\x1AtW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7Freference block hash mismatch\0\0\0`D\x82\x01R`d\x01a\x05\xC6V[\x82`@Q` \x01a\x1A\x85\x91\x90aO\x8DV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82` \x015\x14a\x1A\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FreferenceTaskHash hash mismatch\0`D\x82\x01R`d\x01a\x05\xC6V[a\x1A\xFD``\x84\x01`@\x85\x01aL\x89V[`\x9BTc\xFF\xFF\xFF\xFF\x91\x82\x16\x91a\x1B\x1D\x91`\x01` \x1B\x90\x04\x16a8@aL\xBAV[c\xFF\xFF\xFF\xFF\x16\x11a\x1B`W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x077F\x16\xC6R\x077F\x17FR\x03`\x9C\x1B`D\x82\x01R`d\x01a\x05\xC6V[B`\x98Tb\x03\xF4\x80a\x1Br\x91\x90aL\xE2V[\x11a\x1B\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rlstale state 1`\x98\x1B`D\x82\x01R`d\x01a\x05\xC6V[`\0a\x1B\xE5\x83`@Q` \x01a\x1B\xC5\x91\x90aP\x1BV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x83a\x04I\x90aM)V[`\x9DT\x90\x91Pc\xFF\xFF\xFF\xFF\x16`\0[`\x9C\x80Ta\x1C\x01\x90aM5V[\x90P\x81\x10\x15a\x1C\x92W\x81`\xFF\x16\x83` \x01Q\x82\x81Q\x81\x10a\x1C$Wa\x1C$aE0V[` \x02` \x01\x01Qa\x1C6\x91\x90aMjV[`\x01`\x01``\x1B\x03\x16`d\x84`\0\x01Q\x83\x81Q\x81\x10a\x1CWWa\x1CWaE0V[` \x02` \x01\x01Q`\x01`\x01``\x1B\x03\x16a\x1Cr\x91\x90aM\x99V[\x10\x15a\x1C\x80WPPPPPPV[\x80a\x1C\x8A\x81aM\xB8V[\x91PPa\x1B\xF4V[P`\x80\x84\x015`\x9AU\x7F\xECh\xDB9\x18y\xB0\xF9\xF4 \xD1\xCD\xF3Gj\xFB\xDF\x08Z$b\xBFM+\x11\xDFxFb\x95\xCB\x17a\x1C\xC9` \x87\x01\x87aL\x89V[a\x17b``\x88\x01`@\x89\x01aL\x89V[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01\x81\x90R`\x9C\x80Ta\x1D\x10\x90aM5V[\x90P\x90Pa\x1D1`@Q\x80`@\x01`@R\x80``\x81R` \x01``\x81RP\x90V[\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1DIWa\x1DIa<`V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1DrW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P` \x82\x01R\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\x90Wa\x1D\x90a<`V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1D\xB9W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x81R` \x85\x01QQ`\0\x90`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\xDCWa\x1D\xDCa<`V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1E\x05W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80[\x87` \x01QQ\x81\x10\x15a\x1F\xB7Wa\x1EQ\x88` \x01Q\x82\x81Q\x81\x10a\x1E2Wa\x1E2aE0V[` \x02` \x01\x01Q\x80Q`\0\x90\x81R` \x91\x82\x01Q\x90\x91R`@\x90 \x90V[\x83\x82\x81Q\x81\x10a\x1EcWa\x1EcaE0V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80\x15a\x1F-W\x82a\x1E\x80`\x01\x83aP^V[\x81Q\x81\x10a\x1E\x90Wa\x1E\x90aE0V[` \x02` \x01\x01Q`\0\x1C\x83\x82\x81Q\x81\x10a\x1E\xADWa\x1E\xADaE0V[` \x02` \x01\x01Q`\0\x1C\x11a\x1F-W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: nonSignerPubkeys not sorted`d\x82\x01R`\x84\x01a\x05\xC6V[a\x1F\xA3a\x1F\x9C`\xA1`\0\x86\x85\x81Q\x81\x10a\x1FIWa\x1FIaE0V[` \x02` \x01\x01Q\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\xFF\x16\x8A` \x01Q\x84\x81Q\x81\x10a\x1F\x86Wa\x1F\x86aE0V[` \x02` \x01\x01Qa6\xDC\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x87\x90a2\x82V[\x95P\x80a\x1F\xAF\x81aM\xB8V[\x91PPa\x1E\x0CV[Pa\x1F\xC1\x85a7\xC0V[\x94P`\0[\x84\x81\x10\x15a!\xA5W`\x9C\x81\x81Ta\x1F\xDC\x90aM5V[\x81\x10a\x1F\xEAWa\x1F\xEAaE0V[\x81T`\x01\x16\x15a \tW\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C`\0\x81\x81R`\xA0` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x01T\x90\x82\x01R\x90\x92Pa J\x90\x87\x90a2\x82V[`\xFF\x83\x16`\0\x90\x81R`\x9E` \x90\x81R`@\x90\x91 T\x90\x86\x01Q\x80Q\x92\x98P`\x01`\x01``\x1B\x03\x90\x91\x16\x91\x83\x90\x81\x10a \x85Wa \x85aE0V[`\x01`\x01``\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x82\x01R\x84\x01Q\x80Q\x82\x90\x81\x10a \xB1Wa \xB1aE0V[` \x02` \x01\x01Q\x84`\0\x01Q\x82\x81Q\x81\x10a \xCFWa \xCFaE0V[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPP`\0[\x88` \x01QQ\x81\x10\x15a!\x92W`\x9F`\0\x85\x83\x81Q\x81\x10a!\x15Wa!\x15aE0V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82R\x81\x81\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 `\xFF\x87\x16\x82R\x90\x92R\x90 T\x85Q\x80Q`\x01`\x01``\x1B\x03\x90\x92\x16\x91\x84\x90\x81\x10a!`Wa!`aE0V[` \x02` \x01\x01\x81\x81Qa!t\x91\x90aPuV[`\x01`\x01``\x1B\x03\x16\x90RP\x80a!\x8A\x81aM\xB8V[\x91PPa \xF2V[P\x80a!\x9D\x81aM\xB8V[\x91PPa\x1F\xC6V[P`\0\x80a!\xBD\x8A\x88\x8B``\x01Q\x8C`\x80\x01Qa\x07DV[\x91P\x91P\x81a\"@W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: pairing precompile call fai`d\x82\x01Rb\x1B\x19Y`\xEA\x1B`\x84\x82\x01R`\xA4\x01a\x05\xC6V[\x80a\"\xB3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: signature is invalid\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\xC6V[P\x92\x95PPPPPP[\x92\x91PPV[a\"\xCBa1\x91V[`\0[a\"\xDB` \x84\x01\x84aM\xD3V[\x90P\x81\x10\x15a#\xA0W`\x9E`\0a\"\xF5` \x86\x01\x86aM\xD3V[\x84\x81\x81\x10a#\x05Wa#\x05aE0V[\x90P` \x02\x01` \x81\x01\x90a#\x1A\x91\x90a;\xF3V[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0\x90\x81 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U`\xA0\x91a#M\x90\x86\x01\x86aM\xD3V[\x84\x81\x81\x10a#]Wa#]aE0V[\x90P` \x02\x01` \x81\x01\x90a#r\x91\x90a;\xF3V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x81\x81U`\x01\x01U\x80a#\x98\x81aM\xB8V[\x91PPa\"\xCEV[P`\0[a#\xB1`@\x84\x01\x84aN\x1CV[\x90P\x81\x10\x15a$\xECWa#\xC7`@\x84\x01\x84aN\x1CV[\x82\x81\x81\x10a#\xD7Wa#\xD7aE0V[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a#\xEF\x91\x90aNeV[`\x9E`\0a$\0`@\x87\x01\x87aN\x1CV[\x85\x81\x81\x10a$\x10Wa$\x10aE0V[a$&\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa;\xF3V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x90\x81\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x17\x90\x91Ua$f\x90\x84\x01\x84aN\x1CV[\x82\x81\x81\x10a$vWa$vaE0V[\x90P`\x80\x02\x01`@\x01`\xA0`\0\x85\x80`@\x01\x90a$\x93\x91\x90aN\x1CV[\x85\x81\x81\x10a$\xA3Wa$\xA3aE0V[a$\xB9\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa;\xF3V[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a$\xE4\x81aM\xB8V[\x91PPa#\xA4V[P`\0[a$\xFD``\x84\x01\x84aN\x80V[\x90P\x81\x10\x15a%\xB5Wa%\x13``\x84\x01\x84aN\x80V[\x82\x81\x81\x10a%#Wa%#aE0V[\x90P`@\x02\x01` \x01` \x81\x01\x90a%;\x91\x90aNeV[`\x9E`\0a%L``\x87\x01\x87aN\x80V[\x85\x81\x81\x10a%\\Wa%\\aE0V[a%r\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa;\xF3V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a%\xAD\x81aM\xB8V[\x91PPa$\xF0V[P`\0[a%\xC6`\x80\x84\x01\x84aN\xC9V[\x90P\x81\x10\x15a&bWa%\xDC`\x80\x84\x01\x84aN\xC9V[\x82\x81\x81\x10a%\xECWa%\xECaE0V[\x90P``\x02\x01` \x01`\xA0`\0\x85\x80`\x80\x01\x90a&\t\x91\x90aN\xC9V[\x85\x81\x81\x10a&\x19Wa&\x19aE0V[a&/\x92` ``\x90\x92\x02\x01\x90\x81\x01\x91Pa;\xF3V[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a&Z\x81aM\xB8V[\x91PPa%\xB9V[P`\0[a&s`\xA0\x84\x01\x84aM\xD3V[\x90P\x81\x10\x15a'\x9AW`\0[`\x9C\x80Ta&\x8C\x90aM5V[\x90P\x81\x10\x15a'EW`\x9F`\0a&\xA6`\xA0\x87\x01\x87aM\xD3V[\x85\x81\x81\x10a&\xB6Wa&\xB6aE0V[\x90P` \x02\x015\x81R` \x01\x90\x81R` \x01`\0 `\0`\x9C\x83\x81Ta&\xDB\x90aM5V[\x81\x10a&\xE9Wa&\xE9aE0V[\x81T`\x01\x16\x15a'\x08W\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U\x81a'=\x81aM\xB8V[\x92PPa&\x7FV[P`\xA1`\0a'W`\xA0\x86\x01\x86aM\xD3V[\x84\x81\x81\x10a'gWa'gaE0V[` \x90\x81\x02\x92\x90\x92\x015\x83RP\x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\xFF\x19\x16\x90U\x80a'\x92\x81aM\xB8V[\x91PPa&fV[P`\0[a'\xAB`\xC0\x84\x01\x84aM\xD3V[\x90P\x81\x10\x15a)\xFBWa'\xC1`\xC0\x84\x01\x84aM\xD3V[\x82\x81\x81\x10a'\xD1Wa'\xD1aE0V[\x90P` \x02\x81\x01\x90a'\xE3\x91\x90aO\x11V[a'\xF4\x90`\x80\x81\x01\x90``\x01a;\xF3V[`\xA1`\0a(\x05`\xC0\x87\x01\x87aM\xD3V[\x85\x81\x81\x10a(\x15Wa(\x15aE0V[\x90P` \x02\x81\x01\x90a('\x91\x90aO\x11V[`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP`\0[a(a`\xC0\x85\x01\x85aM\xD3V[\x83\x81\x81\x10a(qWa(qaE0V[\x90P` \x02\x81\x01\x90a(\x83\x91\x90aO\x11V[a(\x91\x90` \x81\x01\x90aM\xD3V[\x90P\x81\x10\x15a)\xE8Wa(\xA7`\xC0\x85\x01\x85aM\xD3V[\x83\x81\x81\x10a(\xB7Wa(\xB7aE0V[\x90P` \x02\x81\x01\x90a(\xC9\x91\x90aO\x11V[a(\xD7\x90`@\x81\x01\x90aM\xD3V[\x82\x81\x81\x10a(\xE7Wa(\xE7aE0V[\x90P` \x02\x01` \x81\x01\x90a(\xFC\x91\x90aNeV[`\x9F`\0a)\r`\xC0\x88\x01\x88aM\xD3V[\x86\x81\x81\x10a)\x1DWa)\x1DaE0V[\x90P` \x02\x81\x01\x90a)/\x91\x90aO\x11V[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a)P`\xC0\x88\x01\x88aM\xD3V[\x86\x81\x81\x10a)`Wa)`aE0V[\x90P` \x02\x81\x01\x90a)r\x91\x90aO\x11V[a)\x80\x90` \x81\x01\x90aM\xD3V[\x85\x81\x81\x10a)\x90Wa)\x90aE0V[\x90P` \x02\x01` \x81\x01\x90a)\xA5\x91\x90a;\xF3V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a)\xE0\x81aM\xB8V[\x91PPa(TV[P\x80a)\xF3\x81aM\xB8V[\x91PPa'\x9EV[P`\0[a*\x0C`\xE0\x84\x01\x84aM\xD3V[\x90P\x81\x10\x15a+\xBFW`\0[a*%`\xE0\x85\x01\x85aM\xD3V[\x83\x81\x81\x10a*5Wa*5aE0V[\x90P` \x02\x81\x01\x90a*G\x91\x90aO1V[a*U\x90` \x81\x01\x90aM\xD3V[\x90P\x81\x10\x15a+\xACWa*k`\xE0\x85\x01\x85aM\xD3V[\x83\x81\x81\x10a*{Wa*{aE0V[\x90P` \x02\x81\x01\x90a*\x8D\x91\x90aO1V[a*\x9B\x90`@\x81\x01\x90aM\xD3V[\x82\x81\x81\x10a*\xABWa*\xABaE0V[\x90P` \x02\x01` \x81\x01\x90a*\xC0\x91\x90aNeV[`\x9F`\0a*\xD1`\xE0\x88\x01\x88aM\xD3V[\x86\x81\x81\x10a*\xE1Wa*\xE1aE0V[\x90P` \x02\x81\x01\x90a*\xF3\x91\x90aO1V[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a+\x14`\xE0\x88\x01\x88aM\xD3V[\x86\x81\x81\x10a+$Wa+$aE0V[\x90P` \x02\x81\x01\x90a+6\x91\x90aO1V[a+D\x90` \x81\x01\x90aM\xD3V[\x85\x81\x81\x10a+TWa+TaE0V[\x90P` \x02\x01` \x81\x01\x90a+i\x91\x90a;\xF3V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a+\xA4\x81aM\xB8V[\x91PPa*\x18V[P\x80a+\xB7\x81aM\xB8V[\x91PPa)\xFFV[P`\0[a+\xD1a\x01\0\x84\x01\x84aN\x80V[\x90P\x81\x10\x15a,uWa+\xE8a\x01\0\x84\x01\x84aN\x80V[\x82\x81\x81\x10a+\xF8Wa+\xF8aE0V[\x90P`@\x02\x01` \x01` \x81\x01\x90a,\x10\x91\x90a;\xF3V[`\xA1`\0a,\"a\x01\0\x87\x01\x87aN\x80V[\x85\x81\x81\x10a,2Wa,2aE0V[\x90P`@\x02\x01`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP\x80\x80a,m\x90aM\xB8V[\x91PPa+\xC3V[P`\x9A\x81\x90Ua,\x88` \x84\x01\x84aL\x89V[`\x9B\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua,\xB2`@\x84\x01` \x85\x01aL\x89V[`\x9B\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90UB`\x98Ua,\xE9``\x84\x01\x84aOGV[a,\xF5\x91`\x9C\x91a:oV[Pa-\x06`\xA0\x84\x01`\x80\x85\x01aL\x89V[`\x9D\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x7F&Ie\xEBk\xC46\xC6\xC4sC\x1D4\xAFV\xE82\xEC4O\xDF\xD4>\xE6\xAFo\xCEm ^\x84\xAFa-N` \x85\x01\x85aL\x89V[a-^`@\x86\x01` \x87\x01aL\x89V[`@\x80Qc\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1PPPV[a-\x8Ba1\x91V[`\x01`\x01`\xA0\x1B\x03\x81\x16a-\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x05\xC6V[a\x05\xD8\x81a6\x8AV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a.LW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.p\x91\x90aDdV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a.\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xC6\x90aD\x81V[`fT\x19\x81\x19`fT\x19\x16\x14a/\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\xC6V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x079V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a/uWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a/\x8FWP0;\x15\x80\x15a/\x8FWP`\0T`\xFF\x16`\x01\x14[a/\xF2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x05\xC6V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a0\x15W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a0 \x85`\0a8[V[a0)\x84a6\x8AV[`\x97\x80T\x83\x15\x15`\x01`\xA8\x1B\x02`\x01a\xFF\x01`\xA0\x1B\x03\x19\x90\x91\x16`\x01`\x01`\xA0\x1B\x03\x86\x16\x17\x17\x90U\x80\x15a0\x93W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01a\x17{V[PPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16a1(W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x05\xC6V[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x18\xF2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x05\xC6V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra2\x07a:\xF3V[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x80\x82\x01\x84\x90R`\0\x90\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a2:Wa2<V[\xFE[P\x80a2zW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCB[][\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x05\xC6V[PP\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra2\x9Ea;\x11V[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R\x83Q`@\x80\x84\x01\x91\x90\x91R\x90\x84\x01Q``\x83\x01R`\0\x90\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a2:WP\x80a2zW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCBXY\x19\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x05\xC6V[a3\x1Ea;/V[P`@\x80Q`\x80\x81\x01\x82R\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81\x83\x01\x90\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED``\x83\x01R\x81R\x81Q\x80\x83\x01\x90\x92R\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x81\x01\x91\x90\x91R\x81\x01\x91\x90\x91R\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0\x80\x80a4\x06`\0\x80Q` aP\x9E\x839\x81Q\x91R\x86aEFV[\x90P[a4\x12\x81a9EV[\x90\x93P\x91P`\0\x80Q` aP\x9E\x839\x81Q\x91R\x82\x83\t\x83\x14\x15a4LW`@\x80Q\x80\x82\x01\x90\x91R\x90\x81R` \x81\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80Q` aP\x9E\x839\x81Q\x91R`\x01\x82\x08\x90Pa4\tV[`@\x80Q\x80\x82\x01\x82R\x86\x81R` \x80\x82\x01\x86\x90R\x82Q\x80\x84\x01\x90\x93R\x86\x83R\x82\x01\x84\x90R`\0\x91\x82\x91\x90a4\x98a;TV[`\0[`\x02\x81\x10\x15a6]W`\0a4\xB1\x82`\x06aM\x99V[\x90P\x84\x82`\x02\x81\x10a4\xC5Wa4\xC5aE0V[` \x02\x01QQ\x83a4\xD7\x83`\0aL\xE2V[`\x0C\x81\x10a4\xE7Wa4\xE7aE0V[` \x02\x01R\x84\x82`\x02\x81\x10a4\xFEWa4\xFEaE0V[` \x02\x01Q` \x01Q\x83\x82`\x01a5\x15\x91\x90aL\xE2V[`\x0C\x81\x10a5%Wa5%aE0V[` \x02\x01R\x83\x82`\x02\x81\x10a5<Wa5<aE0V[` \x02\x01QQQ\x83a5O\x83`\x02aL\xE2V[`\x0C\x81\x10a5_Wa5_aE0V[` \x02\x01R\x83\x82`\x02\x81\x10a5vWa5vaE0V[` \x02\x01QQ`\x01` \x02\x01Q\x83a5\x8F\x83`\x03aL\xE2V[`\x0C\x81\x10a5\x9FWa5\x9FaE0V[` \x02\x01R\x83\x82`\x02\x81\x10a5\xB6Wa5\xB6aE0V[` \x02\x01Q` \x01Q`\0`\x02\x81\x10a5\xD1Wa5\xD1aE0V[` \x02\x01Q\x83a5\xE2\x83`\x04aL\xE2V[`\x0C\x81\x10a5\xF2Wa5\xF2aE0V[` \x02\x01R\x83\x82`\x02\x81\x10a6\tWa6\taE0V[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a6$Wa6$aE0V[` \x02\x01Q\x83a65\x83`\x05aL\xE2V[`\x0C\x81\x10a6EWa6EaE0V[` \x02\x01RP\x80a6U\x81aM\xB8V[\x91PPa4\x9BV[Pa6fa;sV[`\0` \x82a\x01\x80\x85`\x08\x8C\xFA\x91Q\x91\x9C\x91\x15\x15\x9BP\x90\x99PPPPPPPPPPV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x02\0\x82a\xFF\xFF\x16\x10a78W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Roscalar-too-large`\x80\x1B`D\x82\x01R`d\x01a\x05\xC6V[\x81a\xFF\xFF\x16`\x01\x14\x15a7LWP\x81a\"\xBDV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01\x81\x90R\x84\x90`\x01\x90[\x81a\xFF\xFF\x16\x86a\xFF\xFF\x16\x10a7\xB5W`\x01a\xFF\xFF\x87\x16`\xFF\x83\x16\x1C\x81\x16\x14\x15a7\x98Wa7\x95\x84\x84a2\x82V[\x93P[a7\xA2\x83\x84a2\x82V[\x92Pb\x01\xFF\xFE`\x01\x92\x83\x1B\x16\x91\x01a7hV[P\x91\x95\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q\x15\x80\x15a7\xE5WP` \x82\x01Q\x15[\x15a8\x03WPP`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01`\0\x80Q` aP\x9E\x839\x81Q\x91R\x84` \x01Qa86\x91\x90aEFV[a8N\x90`\0\x80Q` aP\x9E\x839\x81Q\x91RaP^V[\x90R\x92\x91PPV[\x91\x90PV[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a8|WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a8\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x05\xC6V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a9A\x82a0\x9AV[PPV[`\0\x80\x80`\0\x80Q` aP\x9E\x839\x81Q\x91R`\x03`\0\x80Q` aP\x9E\x839\x81Q\x91R\x86`\0\x80Q` aP\x9E\x839\x81Q\x91R\x88\x89\t\t\x08\x90P`\0a9\xBB\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\0\x80Q` aP\x9E\x839\x81Q\x91Ra9\xC7V[\x91\x95\x91\x94P\x90\x92PPPV[`\0\x80a9\xD2a;sV[a9\xDAa;\x91V[` \x80\x82R\x81\x81\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x88\x90R`\x80\x82\x01\x87\x90R`\xA0\x82\x01\x86\x90R\x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82\x80\x15a2:WP\x82a:dW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FBN254.expMod: call failure\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\xC6V[PQ\x95\x94PPPPPV[\x82\x80Ta:{\x90aM5V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82a:\x9DW`\0\x85Ua:\xE3V[\x82`\x1F\x10a:\xB6W\x82\x80\x01`\xFF\x19\x825\x16\x17\x85Ua:\xE3V[\x82\x80\x01`\x01\x01\x85U\x82\x15a:\xE3W\x91\x82\x01[\x82\x81\x11\x15a:\xE3W\x825\x82U\x91` \x01\x91\x90`\x01\x01\x90a:\xC8V[Pa:\xEF\x92\x91Pa;\xAFV[P\x90V[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80a;Ba;\xC4V[\x81R` \x01a;Oa;\xC4V[\x90R\x90V[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[[\x80\x82\x11\x15a:\xEFW`\0\x81U`\x01\x01a;\xB0V[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[\x805`\xFF\x81\x16\x81\x14a8VW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a<\x05W`\0\x80\xFD[a<\x0E\x82a;\xE2V[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05\xD8W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a<<W`\0\x80\xFD[\x815a<\x0E\x81a<\x15V[`\0` \x82\x84\x03\x12\x15a<YW`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a<\x98Wa<\x98a<`V[`@R\x90V[`@Qa\x01\0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a<\x98Wa<\x98a<`V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a<\xE9Wa<\xE9a<`V[`@R\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a=\x03W`\0\x80\xFD[a=\x0Ba<vV[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a=2W`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a=TWa=Ta<`V[\x80`@RP\x80`@\x84\x01\x85\x81\x11\x15a=kW`\0\x80\xFD[\x84[\x81\x81\x10\x15a7\xB5W\x805\x83R` \x92\x83\x01\x92\x01a=mV[`\0`\x80\x82\x84\x03\x12\x15a=\x97W`\0\x80\xFD[a=\x9Fa<vV[\x90Pa=\xAB\x83\x83a=!V[\x81Ra=\xBA\x83`@\x84\x01a=!V[` \x82\x01R\x92\x91PPV[`\0\x80`\0\x80a\x01 \x85\x87\x03\x12\x15a=\xDCW`\0\x80\xFD[\x845\x93Pa=\xED\x86` \x87\x01a<\xF1V[\x92Pa=\xFC\x86``\x87\x01a=\x85V[\x91Pa>\x0B\x86`\xE0\x87\x01a<\xF1V[\x90P\x92\x95\x91\x94P\x92PV[`\0`\xE0\x82\x84\x03\x12\x15a>(W`\0\x80\xFD[P\x91\x90PV[`\0a\x01\x80\x82\x84\x03\x12\x15a>(W`\0\x80\xFD[`\0a\x01 \x82\x84\x03\x12\x15a>(W`\0\x80\xFD[`\0\x80`\0\x80\x84\x86\x03`\xC0\x81\x12\x15a>kW`\0\x80\xFD[\x855`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a>\x82W`\0\x80\xFD[a>\x8E\x89\x83\x8A\x01a>\x16V[\x96P```\x1F\x19\x84\x01\x12\x15a>\xA2W`\0\x80\xFD[` \x88\x01\x95P`\x80\x88\x015\x92P\x80\x83\x11\x15a>\xBCW`\0\x80\xFD[a>\xC8\x89\x84\x8A\x01a>.V[\x94P`\xA0\x88\x015\x92P\x80\x83\x11\x15a>\xDEW`\0\x80\xFD[PPa>\xEC\x87\x82\x88\x01a>AV[\x91PP\x92\x95\x91\x94P\x92PV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a?%W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a?\tV[\x81\x81\x11\x15a?7W`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a?`W`\0\x80\xFD[\x825\x91Pa?p` \x84\x01a;\xE2V[\x90P\x92P\x92\x90PV[`\0\x80`\0\x83\x85\x03`\xE0\x81\x12\x15a?\x8FW`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a?\xA6W`\0\x80\xFD[\x90\x86\x01\x90`\xC0\x82\x89\x03\x12\x15a?\xBAW`\0\x80\xFD[\x81\x95P`\xA0`\x1F\x19\x84\x01\x12\x15a?\xCFW`\0\x80\xFD[` \x87\x01\x94P`\xC0\x87\x015\x92P\x80\x83\x11\x15a?\xE9W`\0\x80\xFD[PPa?\xF7\x86\x82\x87\x01a>.V[\x91PP\x92P\x92P\x92V[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a@\x1AWa@\x1Aa<`V[P`\x05\x1B` \x01\x90V[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a8VW`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a@IW`\0\x80\xFD[\x815` a@^a@Y\x83a@\x01V[a<\xC1V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a@}W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a@\x9FWa@\x92\x81a@$V[\x83R\x91\x83\x01\x91\x83\x01a@\x81V[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a@\xBBW`\0\x80\xFD[\x815` a@\xCBa@Y\x83a@\x01V[\x82\x81R`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a@\xEAW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a@\x9FWaA\0\x88\x82a<\xF1V[\x83R\x91\x83\x01\x91`@\x01a@\xEEV[`\0\x82`\x1F\x83\x01\x12aA\x1FW`\0\x80\xFD[\x815` aA/a@Y\x83a@\x01V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aANW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a@\x9FW\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aAqW`\0\x80\x81\xFD[aA\x7F\x89\x86\x83\x8B\x01\x01a@8V[\x84RP\x91\x83\x01\x91\x83\x01aARV[`\0a\x01\x80\x82\x84\x03\x12\x15aA\xA0W`\0\x80\xFD[aA\xA8a<\x9EV[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aA\xC1W`\0\x80\xFD[aA\xCD\x85\x83\x86\x01a@8V[\x83R` \x84\x015\x91P\x80\x82\x11\x15aA\xE3W`\0\x80\xFD[aA\xEF\x85\x83\x86\x01a@\xAAV[` \x84\x01R`@\x84\x015\x91P\x80\x82\x11\x15aB\x08W`\0\x80\xFD[aB\x14\x85\x83\x86\x01a@\xAAV[`@\x84\x01RaB&\x85``\x86\x01a=\x85V[``\x84\x01RaB8\x85`\xE0\x86\x01a<\xF1V[`\x80\x84\x01Ra\x01 \x84\x015\x91P\x80\x82\x11\x15aBRW`\0\x80\xFD[aB^\x85\x83\x86\x01a@8V[`\xA0\x84\x01Ra\x01@\x84\x015\x91P\x80\x82\x11\x15aBxW`\0\x80\xFD[aB\x84\x85\x83\x86\x01a@8V[`\xC0\x84\x01Ra\x01`\x84\x015\x91P\x80\x82\x11\x15aB\x9EW`\0\x80\xFD[PaB\xAB\x84\x82\x85\x01aA\x0EV[`\xE0\x83\x01RP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15aB\xCAW`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aB\xE7W`\0\x80\xFD[aB\xF3\x85\x82\x86\x01aA\x8DV[\x91PP\x92P\x92\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aC6W\x81Q`\x01`\x01``\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aC\x11V[P\x94\x95\x94PPPPPV[` \x81R`\0\x82Q`@` \x84\x01RaC]``\x84\x01\x82aB\xFDV[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01RaCz\x82\x82aB\xFDV[\x95\x94PPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15aC\x98W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aC\xAFW`\0\x80\xFD[aC\xBB\x87\x83\x88\x01a>\x16V[\x94P` \x86\x015\x91P\x80\x82\x11\x15aC\xD1W`\0\x80\xFD[PaC\xDE\x86\x82\x87\x01a>AV[\x92PP`@\x84\x015\x90P\x92P\x92P\x92V[\x80\x15\x15\x81\x14a\x05\xD8W`\0\x80\xFD[\x805a8V\x81aC\xEFV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aD\x1EW`\0\x80\xFD[\x845aD)\x81a<\x15V[\x93P` \x85\x015aD9\x81a<\x15V[\x92P`@\x85\x015aDI\x81a<\x15V[\x91P``\x85\x015aDY\x81aC\xEFV[\x93\x96\x92\x95P\x90\x93PPV[`\0` \x82\x84\x03\x12\x15aDvW`\0\x80\xFD[\x81Qa<\x0E\x81a<\x15V[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15aD\xDDW`\0\x80\xFD[\x81Qa<\x0E\x81aC\xEFV[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x82aEcWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aE\x7FW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aE\x9EW`\0\x80\xFD[\x806\x03\x83\x13\x15aE\xADW`\0\x80\xFD[\x92P\x92\x90PV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x81R`\0c\xFF\xFF\xFF\xFF\x80aE\xF2\x85a@$V[\x16` \x84\x01R\x80aF\x05` \x86\x01a@$V[\x16`@\x84\x01R\x80aF\x18`@\x86\x01a@$V[\x16``\x84\x01RPaF,``\x84\x01\x84aEhV[`\xE0`\x80\x85\x01RaFBa\x01\0\x85\x01\x82\x84aE\xB4V[\x91PPaFQ`\x80\x85\x01a@$V[c\xFF\xFF\xFF\xFF\x81\x16`\xA0\x85\x01RPaFk`\xA0\x85\x01\x85aEhV[\x84\x83\x03`\x1F\x19\x01`\xC0\x86\x01RaF\x82\x83\x82\x84aE\xB4V[\x92PPPaF\x92`\xC0\x85\x01a@$V[c\xFF\xFF\xFF\xFF\x81\x16`\xE0\x85\x01RP\x93\x92PPPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aF\xBDW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aF\xDCW`\0\x80\xFD[\x80`\x05\x1B6\x03\x83\x13\x15aE\xADW`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15aC6W`\xFFaG\x11\x83a;\xE2V[\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aF\xFEV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aG;W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aGZW`\0\x80\xFD[\x80`\x07\x1B6\x03\x83\x13\x15aE\xADW`\0\x80\xFD[\x805`\x01`\x01``\x1B\x03\x81\x16\x81\x14a8VW`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15aC6W`\xFFaG\xA6\x83a;\xE2V[\x16\x87R`\x01`\x01``\x1B\x03aG\xBC\x84\x84\x01aGlV[\x16\x83\x88\x01R`@\x82\x81\x015\x90\x88\x01R``\x80\x83\x015\x90\x88\x01R`\x80\x96\x87\x01\x96\x90\x91\x01\x90`\x01\x01aG\x93V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aG\xFEW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aH\x1DW`\0\x80\xFD[\x80`\x06\x1B6\x03\x83\x13\x15aE\xADW`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15aC6W`\xFFaHR\x83a;\xE2V[\x16\x87R`\x01`\x01``\x1B\x03aHh\x84\x84\x01aGlV[\x16\x87\x84\x01R`@\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aH?V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aH\x97W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aH\xB6W`\0\x80\xFD[``\x81\x026\x03\x83\x13\x15aE\xADW`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15aC6W`\xFFaH\xEB\x83a;\xE2V[\x16\x87RaI\x06\x83\x88\x01\x84\x84\x01\x805\x82R` \x90\x81\x015\x91\x01RV[``\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aH\xD8V[\x81\x83R`\0`\x01`\x01`\xFB\x1B\x03\x83\x11\x15aI2W`\0\x80\xFD[\x82`\x05\x1B\x80\x83` \x87\x017`\0\x94\x01` \x01\x93\x84RP\x91\x92\x91PPV[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15aC6W`\x01`\x01``\x1B\x03aIx\x83aGlV[\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aI_V[\x81\x83R`\0` \x80\x85\x01\x94P\x84\x84`\x05\x1B\x86\x01\x84`\0\x80[\x88\x81\x10\x15aJ9W\x84\x84\x03\x8AR\x825`~\x19\x896\x03\x01\x81\x12aI\xC3W\x82\x83\xFD[\x88\x01\x805\x85R`\x80aI\xD7\x88\x83\x01\x83aF\xA6V[\x82\x8A\x89\x01RaI\xE9\x83\x89\x01\x82\x84aF\xEEV[\x92PPP`@aI\xFB\x81\x84\x01\x84aF\xA6V[\x88\x84\x03\x83\x8A\x01RaJ\r\x84\x82\x84aIOV[\x93PPPP```\xFFaJ!\x82\x85\x01a;\xE2V[\x16\x96\x01\x95\x90\x95RP\x98\x85\x01\x98\x91\x85\x01\x91`\x01\x01aI\xA3V[P\x91\x98\x97PPPPPPPPV[\x81\x83R`\0` \x80\x85\x01\x94P\x84\x84`\x05\x1B\x86\x01\x84`\0\x80[\x88\x81\x10\x15aJ9W\x84\x84\x03\x8AR\x825`^\x19\x896\x03\x01\x81\x12aJ\x7FW\x82\x83\xFD[\x88\x01\x805\x85R``aJ\x93\x88\x83\x01\x83aF\xA6V[\x82\x8A\x89\x01RaJ\xA5\x83\x89\x01\x82\x84aF\xEEV[\x92PPP`@aJ\xB7\x81\x84\x01\x84aF\xA6V[\x93P\x87\x83\x03\x82\x89\x01RaJ\xCB\x83\x85\x83aIOV[\x9D\x8A\x01\x9D\x97PPP\x93\x87\x01\x93PP`\x01\x01aJ_V[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15aC6W\x815\x87R`\xFFaK\n\x84\x84\x01a;\xE2V[\x16\x87\x84\x01R`@\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aJ\xF1V[` \x81RaK<` \x82\x01aK6\x84aC\xFDV[\x15\x15\x90RV[`\0aKK` \x84\x01\x84aF\xA6V[a\x01 \x80`@\x86\x01RaKca\x01@\x86\x01\x83\x85aF\xEEV[\x92PaKr`@\x87\x01\x87aG$V[\x92P`\x1F\x19\x80\x87\x86\x03\x01``\x88\x01RaK\x8C\x85\x85\x84aG\x83V[\x94PaK\x9B``\x89\x01\x89aG\xE7V[\x94P\x91P\x80\x87\x86\x03\x01`\x80\x88\x01RaK\xB4\x85\x85\x84aH/V[\x94PaK\xC3`\x80\x89\x01\x89aH\x80V[\x94P\x91P\x80\x87\x86\x03\x01`\xA0\x88\x01RaK\xDC\x85\x85\x84aH\xC8V[\x94PaK\xEB`\xA0\x89\x01\x89aF\xA6V[\x94P\x91P\x80\x87\x86\x03\x01`\xC0\x88\x01RaL\x04\x85\x85\x84aI\x19V[\x94PaL\x13`\xC0\x89\x01\x89aF\xA6V[\x94P\x91P\x80\x87\x86\x03\x01`\xE0\x88\x01RaL,\x85\x85\x84aI\x8BV[\x94PaL;`\xE0\x89\x01\x89aF\xA6V[\x94P\x91Pa\x01\0\x81\x88\x87\x03\x01\x81\x89\x01RaLV\x86\x86\x85aJGV[\x95PaLd\x81\x8A\x01\x8AaG\xE7V[\x95P\x92PP\x80\x87\x86\x03\x01\x83\x88\x01RPaL~\x84\x84\x83aJ\xE1V[\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15aL\x9BW`\0\x80\xFD[a<\x0E\x82a@$V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aL\xD9WaL\xD9aL\xA4V[\x01\x94\x93PPPPV[`\0\x82\x19\x82\x11\x15aL\xF5WaL\xF5aL\xA4V[P\x01\x90V[``\x81\x01c\xFF\xFF\xFF\xFFaM\x0C\x84a@$V[\x16\x82R` \x83\x015` \x83\x01R`@\x83\x015`@\x83\x01R\x92\x91PPV[`\0a\"\xBD6\x83aA\x8DV[`\x01\x81\x81\x1C\x90\x82\x16\x80aMIW`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15a>(WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aM\x90WaM\x90aL\xA4V[\x02\x94\x93PPPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15aM\xB3WaM\xB3aL\xA4V[P\x02\x90V[`\0`\0\x19\x82\x14\x15aM\xCCWaM\xCCaL\xA4V[P`\x01\x01\x90V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aM\xEAW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aN\x04W`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15aE\xADW`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aN3W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aNMW`\0\x80\xFD[` \x01\x91P`\x07\x81\x90\x1B6\x03\x82\x13\x15aE\xADW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aNwW`\0\x80\xFD[a<\x0E\x82aGlV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aN\x97W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aN\xB1W`\0\x80\xFD[` \x01\x91P`\x06\x81\x90\x1B6\x03\x82\x13\x15aE\xADW`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aN\xE0W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aN\xFAW`\0\x80\xFD[` \x01\x91P``\x81\x026\x03\x82\x13\x15aE\xADW`\0\x80\xFD[`\0\x825`~\x19\x836\x03\x01\x81\x12aO'W`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x825`^\x19\x836\x03\x01\x81\x12aO'W`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aO^W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aOxW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15aE\xADW`\0\x80\xFD[` \x81R`\0c\xFF\xFF\xFF\xFF\x80aO\xA2\x85a@$V[\x16` \x84\x01R` \x84\x015`@\x84\x01R\x80aO\xBF`@\x86\x01a@$V[\x16``\x84\x01R\x80aO\xD2``\x86\x01a@$V[\x16`\x80\x84\x01RaO\xE5`\x80\x85\x01\x85aEhV[`\xC0`\xA0\x86\x01RaO\xFA`\xE0\x86\x01\x82\x84aE\xB4V[\x91PP\x81aP\n`\xA0\x87\x01a@$V[\x16`\xC0\x85\x01R\x80\x92PPP\x92\x91PPV[`\xA0\x81\x01c\xFF\xFF\xFF\xFFaP-\x84a@$V[\x16\x82R` \x83\x015` \x83\x01R`@\x83\x015`@\x83\x01R``\x83\x015``\x83\x01R`\x80\x83\x015`\x80\x83\x01R\x92\x91PPV[`\0\x82\x82\x10\x15aPpWaPpaL\xA4V[P\x03\x90V[`\0`\x01`\x01``\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15aP\x95WaP\x95aL\xA4V[\x03\x93\x92PPPV\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 \xE0\x19\xE8\x80t\x9B}N\xFFV\xDE\xF0Le\xA3(Fe\xFA9\xDAP\x1A;b\x02\xC1\xA0\x9C\x8E\xC2edsolcC\0\x08\x0C\x003";
    /// The bytecode of the contract.
    pub static GASPMULTIROLLUPSERVICE_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xE5W`\x005`\xE0\x1C\x80c\\\x97Z\xBB\x11a\x01\x0FW\x80c\xD0:\x07\xB2\x11a\0\xA2W\x80c\xF2\xFD\xE3\x8B\x11a\0qW\x80c\xF2\xFD\xE3\x8B\x14a\x04\xDDW\x80c\xF8N\x91\xFC\x14a\x04\xF0W\x80c\xFA\xBC\x1C\xBC\x14a\x04\xF9W\x80c\xFE\xCF\x974\x14a\x05\x0CW`\0\x80\xFD[\x80c\xD0:\x07\xB2\x14a\x04\x97W\x80c\xD0\x93\x86\x7F\x14a\x04\xA7W\x80c\xDF\x03L\xD0\x14a\x04\xBAW\x80c\xE2\xA7\xCBf\x14a\x04\xCDW`\0\x80\xFD[\x80cz\xD7Ua\x11a\0\xDEW\x80cz\xD7Ua\x14a\x04\x12W\x80c}\x97\x88\x97\x14a\x04;W\x80c\x88o\x11\x95\x14a\x04[W\x80c\x8D\xA5\xCB[\x14a\x04\x86W`\0\x80\xFD[\x80c\\\x97Z\xBB\x14a\x03\xD8W\x80co\x0C0\xA4\x14a\x03\xE0W\x80cqP\x18\xA6\x14a\x03\xF7W\x80cy\xA0\xA8S\x14a\x03\xFFW`\0\x80\xFD[\x80c*\x84\x14\xFD\x11a\x01\x87W\x80cM\xEA\xBC!\x11a\x01VW\x80cM\xEA\xBC!\x14a\x03tW\x80cRn>d\x14a\x03\x99W\x80cY\\jg\x14a\x03\xADW\x80cZ\xC8j\xB7\x14a\x03\xB5W`\0\x80\xFD[\x80c*\x84\x14\xFD\x14a\x02\xC7W\x80cC\r;9\x14a\x02\xDCW\x80cI\x9Do\xB6\x14a\x03\x11W\x80cJ\xE6\xB2\x03\x14a\x03]W`\0\x80\xFD[\x80c\x12FH\xC9\x11a\x01\xC3W\x80c\x12FH\xC9\x14a\x02dW\x80c\x13d9\xDD\x14a\x02wW\x80c\x17\x1F\x1D[\x14a\x02\x8AW\x80c#+\x8E\x98\x14a\x02\xB4W`\0\x80\xFD[\x80c\x03\xD0\x97\xD2\x14a\x01\xEAW\x80c\x0E\xE0\xFD\xBD\x14a\x02+W\x80c\x10\xD6z/\x14a\x02OW[`\0\x80\xFD[a\x02\x11a\x01\xF86`\x04a;\xF3V[`\xA0` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T\x82V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[`\x97Ta\x02?\x90`\x01`\xA8\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02\"V[a\x02ba\x02]6`\x04a<*V[a\x05\x1FV[\0[a\x02ba\x02r6`\x04a<*V[a\x05\xDBV[a\x02ba\x02\x856`\x04a<GV[a\x06\x05V[a\x02\x9Da\x02\x986`\x04a=\xC5V[a\x07DV[`@\x80Q\x92\x15\x15\x83R\x90\x15\x15` \x83\x01R\x01a\x02\"V[a\x02ba\x02\xC26`\x04a>TV[a\x08\xCEV[a\x02\xCFa\x17\x8BV[`@Qa\x02\"\x91\x90a>\xF8V[a\x02\xFFa\x02\xEA6`\x04a<GV[`\xA1` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02\"V[a\x03Ea\x03\x1F6`\x04a?MV[`\x9F` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\x01`\x01``\x1B\x03\x16\x81V[`@Q`\x01`\x01``\x1B\x03\x90\x91\x16\x81R` \x01a\x02\"V[a\x03f`\x9AT\x81V[`@Q\x90\x81R` \x01a\x02\"V[`\x9DTa\x03\x84\x90c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\"V[`\x97Ta\x02?\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[a\x02ba\x18\x19V[a\x02?a\x03\xC36`\x04a;\xF3V[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`fTa\x03fV[`\x9BTa\x03\x84\x90`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x02ba\x18\xE0V[a\x02ba\x04\r6`\x04a?yV[a\x18\xF4V[a\x03Ea\x04 6`\x04a;\xF3V[`\x9E` R`\0\x90\x81R`@\x90 T`\x01`\x01``\x1B\x03\x16\x81V[a\x04Na\x04I6`\x04aB\xB7V[a\x1C\xD9V[`@Qa\x02\"\x91\x90aCAV[`eTa\x04n\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\"V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x04nV[`\x99Ta\x03\x84\x90c\xFF\xFF\xFF\xFF\x16\x81V[a\x02ba\x04\xB56`\x04aC\x83V[a\"\xC3V[`\x97Ta\x04n\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x9BTa\x03\x84\x90c\xFF\xFF\xFF\xFF\x16\x81V[a\x02ba\x04\xEB6`\x04a<*V[a-\x83V[a\x03f`\x98T\x81V[a\x02ba\x05\x076`\x04a<GV[a-\xF9V[a\x02ba\x05\x1A6`\x04aD\x08V[a/UV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x96\x91\x90aDdV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x05\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xC6\x90aD\x81V[`@Q\x80\x91\x03\x90\xFD[a\x05\xD8\x81a0\x9AV[PV[a\x05\xE3a1\x91V[`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06MW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06q\x91\x90aD\xCBV[a\x06\x8DW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xC6\x90aD\xE8V[`fT\x81\x81\x16\x14a\x07\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\xC6V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`\0\x80`\0\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x87\x87`\0\x01Q\x88` \x01Q\x88`\0\x01Q`\0`\x02\x81\x10a\x07\x8CWa\x07\x8CaE0V[` \x02\x01Q\x89Q`\x01` \x02\x01Q\x8A` \x01Q`\0`\x02\x81\x10a\x07\xB1Wa\x07\xB1aE0V[` \x02\x01Q\x8B` \x01Q`\x01`\x02\x81\x10a\x07\xCDWa\x07\xCDaE0V[` \x90\x81\x02\x91\x90\x91\x01Q\x8CQ\x8D\x83\x01Q`@Qa\x08*\x9A\x99\x98\x97\x96\x95\x94\x01\x98\x89R` \x89\x01\x97\x90\x97R`@\x88\x01\x95\x90\x95R``\x87\x01\x93\x90\x93R`\x80\x86\x01\x91\x90\x91R`\xA0\x85\x01R`\xC0\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01Ra\x01 \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\0\x1Ca\x08M\x91\x90aEFV[\x90Pa\x08\xC0a\x08fa\x08_\x88\x84a1\xEBV[\x86\x90a2\x82V[a\x08na3\x16V[a\x08\xB6a\x08\xA7\x85a\x08\xA1`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x01\x82R`\x02\x90\x82\x01R\x90V[\x90a1\xEBV[a\x08\xB0\x8Ca3\xD6V[\x90a2\x82V[\x88b\x01\xD4\xC0a4fV[\x90\x98\x90\x97P\x95PPPPPPV[`\x9BT`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15\x80\x15a\t\x80W`\x97T`\x01`\xA8\x1B\x90\x04`\xFF\x16\x15a\t>W`\x97T`\x01`\x01`\xA0\x1B\x03\x163\x14a\t9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04\x17WF\x83`\xDC\x1B`D\x82\x01R`d\x01a\x05\xC6V[a\t\xC2V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\t9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd \xBA\xBA4\x19`\xD9\x1B`D\x82\x01R`d\x01a\x05\xC6V[`\x97T`\x01`\x01`\xA0\x1B\x03\x163\x14a\t\xC2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04\x17WF\x83`\xDC\x1B`D\x82\x01R`d\x01a\x05\xC6V[\x84`@Q` \x01a\t\xD3\x91\x90aE\xDDV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x015\x14a\n;W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FreferenceTaskHash hash mismatch\0`D\x82\x01R`d\x01a\x05\xC6V[\x81`@Q` \x01a\nL\x91\x90aK\"V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84`@\x015\x14a\n\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FoperatorStateInfo hash mismatch\0`D\x82\x01R`d\x01a\x05\xC6V[\x80a\x0C\xD4Wa\n\xC9``\x86\x01`@\x87\x01aL\x89V[`\x9BT`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14a\x0B*W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7Freference block mismatch\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\xC6V[a\x0B:`@\x86\x01` \x87\x01aL\x89V[`\x9BTc\xFF\xFF\xFF\xFF\x91\x82\x16\x91a\x0BZ\x91`\x01` \x1B\x90\x04\x16a8@aL\xBAV[c\xFF\xFF\xFF\xFF\x16\x11a\x0B\x9DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x077F\x16\xC6R\x077F\x17FR\x03`\x9C\x1B`D\x82\x01R`d\x01a\x05\xC6V[B`\x98Tb\x03\xF4\x80a\x0B\xAF\x91\x90aL\xE2V[\x11a\x0B\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rlstale state 1`\x98\x1B`D\x82\x01R`d\x01a\x05\xC6V[`\0a\x0C\"\x85`@Q` \x01a\x0C\x02\x91\x90aL\xFAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x85a\x04I\x90aM)V[`\x9DT\x90\x91Pc\xFF\xFF\xFF\xFF\x16`\0[`\x9C\x80Ta\x0C>\x90aM5V[\x90P\x81\x10\x15a\x0C\xD0W\x81`\xFF\x16\x83` \x01Q\x82\x81Q\x81\x10a\x0CaWa\x0CaaE0V[` \x02` \x01\x01Qa\x0Cs\x91\x90aMjV[`\x01`\x01``\x1B\x03\x16`d\x84`\0\x01Q\x83\x81Q\x81\x10a\x0C\x94Wa\x0C\x94aE0V[` \x02` \x01\x01Q`\x01`\x01``\x1B\x03\x16a\x0C\xAF\x91\x90aM\x99V[\x10\x15a\x0C\xBEWPPPPa\x17\x85V[\x80a\x0C\xC8\x81aM\xB8V[\x91PPa\x0C1V[PPP[`\0[a\x0C\xE4` \x84\x01\x84aM\xD3V[\x90P\x81\x10\x15a\r\xA9W`\x9E`\0a\x0C\xFE` \x86\x01\x86aM\xD3V[\x84\x81\x81\x10a\r\x0EWa\r\x0EaE0V[\x90P` \x02\x01` \x81\x01\x90a\r#\x91\x90a;\xF3V[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0\x90\x81 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U`\xA0\x91a\rV\x90\x86\x01\x86aM\xD3V[\x84\x81\x81\x10a\rfWa\rfaE0V[\x90P` \x02\x01` \x81\x01\x90a\r{\x91\x90a;\xF3V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x81\x81U`\x01\x01U\x80a\r\xA1\x81aM\xB8V[\x91PPa\x0C\xD7V[P`\0[a\r\xBA`@\x84\x01\x84aN\x1CV[\x90P\x81\x10\x15a\x0E\xF5Wa\r\xD0`@\x84\x01\x84aN\x1CV[\x82\x81\x81\x10a\r\xE0Wa\r\xE0aE0V[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\r\xF8\x91\x90aNeV[`\x9E`\0a\x0E\t`@\x87\x01\x87aN\x1CV[\x85\x81\x81\x10a\x0E\x19Wa\x0E\x19aE0V[a\x0E/\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa;\xF3V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x90\x81\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x17\x90\x91Ua\x0Eo\x90\x84\x01\x84aN\x1CV[\x82\x81\x81\x10a\x0E\x7FWa\x0E\x7FaE0V[\x90P`\x80\x02\x01`@\x01`\xA0`\0\x85\x80`@\x01\x90a\x0E\x9C\x91\x90aN\x1CV[\x85\x81\x81\x10a\x0E\xACWa\x0E\xACaE0V[a\x0E\xC2\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa;\xF3V[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a\x0E\xED\x81aM\xB8V[\x91PPa\r\xADV[P`\0[a\x0F\x06``\x84\x01\x84aN\x80V[\x90P\x81\x10\x15a\x0F\xBEWa\x0F\x1C``\x84\x01\x84aN\x80V[\x82\x81\x81\x10a\x0F,Wa\x0F,aE0V[\x90P`@\x02\x01` \x01` \x81\x01\x90a\x0FD\x91\x90aNeV[`\x9E`\0a\x0FU``\x87\x01\x87aN\x80V[\x85\x81\x81\x10a\x0FeWa\x0FeaE0V[a\x0F{\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa;\xF3V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\x0F\xB6\x81aM\xB8V[\x91PPa\x0E\xF9V[P`\0[a\x0F\xCF`\x80\x84\x01\x84aN\xC9V[\x90P\x81\x10\x15a\x10kWa\x0F\xE5`\x80\x84\x01\x84aN\xC9V[\x82\x81\x81\x10a\x0F\xF5Wa\x0F\xF5aE0V[\x90P``\x02\x01` \x01`\xA0`\0\x85\x80`\x80\x01\x90a\x10\x12\x91\x90aN\xC9V[\x85\x81\x81\x10a\x10\"Wa\x10\"aE0V[a\x108\x92` ``\x90\x92\x02\x01\x90\x81\x01\x91Pa;\xF3V[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a\x10c\x81aM\xB8V[\x91PPa\x0F\xC2V[P`\0[a\x10|`\xA0\x84\x01\x84aM\xD3V[\x90P\x81\x10\x15a\x11\xA3W`\0[`\x9C\x80Ta\x10\x95\x90aM5V[\x90P\x81\x10\x15a\x11NW`\x9F`\0a\x10\xAF`\xA0\x87\x01\x87aM\xD3V[\x85\x81\x81\x10a\x10\xBFWa\x10\xBFaE0V[\x90P` \x02\x015\x81R` \x01\x90\x81R` \x01`\0 `\0`\x9C\x83\x81Ta\x10\xE4\x90aM5V[\x81\x10a\x10\xF2Wa\x10\xF2aE0V[\x81T`\x01\x16\x15a\x11\x11W\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U\x81a\x11F\x81aM\xB8V[\x92PPa\x10\x88V[P`\xA1`\0a\x11``\xA0\x86\x01\x86aM\xD3V[\x84\x81\x81\x10a\x11pWa\x11paE0V[` \x90\x81\x02\x92\x90\x92\x015\x83RP\x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\xFF\x19\x16\x90U\x80a\x11\x9B\x81aM\xB8V[\x91PPa\x10oV[P`\0[a\x11\xB4`\xC0\x84\x01\x84aM\xD3V[\x90P\x81\x10\x15a\x14\x04Wa\x11\xCA`\xC0\x84\x01\x84aM\xD3V[\x82\x81\x81\x10a\x11\xDAWa\x11\xDAaE0V[\x90P` \x02\x81\x01\x90a\x11\xEC\x91\x90aO\x11V[a\x11\xFD\x90`\x80\x81\x01\x90``\x01a;\xF3V[`\xA1`\0a\x12\x0E`\xC0\x87\x01\x87aM\xD3V[\x85\x81\x81\x10a\x12\x1EWa\x12\x1EaE0V[\x90P` \x02\x81\x01\x90a\x120\x91\x90aO\x11V[`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP`\0[a\x12j`\xC0\x85\x01\x85aM\xD3V[\x83\x81\x81\x10a\x12zWa\x12zaE0V[\x90P` \x02\x81\x01\x90a\x12\x8C\x91\x90aO\x11V[a\x12\x9A\x90` \x81\x01\x90aM\xD3V[\x90P\x81\x10\x15a\x13\xF1Wa\x12\xB0`\xC0\x85\x01\x85aM\xD3V[\x83\x81\x81\x10a\x12\xC0Wa\x12\xC0aE0V[\x90P` \x02\x81\x01\x90a\x12\xD2\x91\x90aO\x11V[a\x12\xE0\x90`@\x81\x01\x90aM\xD3V[\x82\x81\x81\x10a\x12\xF0Wa\x12\xF0aE0V[\x90P` \x02\x01` \x81\x01\x90a\x13\x05\x91\x90aNeV[`\x9F`\0a\x13\x16`\xC0\x88\x01\x88aM\xD3V[\x86\x81\x81\x10a\x13&Wa\x13&aE0V[\x90P` \x02\x81\x01\x90a\x138\x91\x90aO\x11V[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a\x13Y`\xC0\x88\x01\x88aM\xD3V[\x86\x81\x81\x10a\x13iWa\x13iaE0V[\x90P` \x02\x81\x01\x90a\x13{\x91\x90aO\x11V[a\x13\x89\x90` \x81\x01\x90aM\xD3V[\x85\x81\x81\x10a\x13\x99Wa\x13\x99aE0V[\x90P` \x02\x01` \x81\x01\x90a\x13\xAE\x91\x90a;\xF3V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\x13\xE9\x81aM\xB8V[\x91PPa\x12]V[P\x80a\x13\xFC\x81aM\xB8V[\x91PPa\x11\xA7V[P`\0[a\x14\x15`\xE0\x84\x01\x84aM\xD3V[\x90P\x81\x10\x15a\x15\xC8W`\0[a\x14.`\xE0\x85\x01\x85aM\xD3V[\x83\x81\x81\x10a\x14>Wa\x14>aE0V[\x90P` \x02\x81\x01\x90a\x14P\x91\x90aO1V[a\x14^\x90` \x81\x01\x90aM\xD3V[\x90P\x81\x10\x15a\x15\xB5Wa\x14t`\xE0\x85\x01\x85aM\xD3V[\x83\x81\x81\x10a\x14\x84Wa\x14\x84aE0V[\x90P` \x02\x81\x01\x90a\x14\x96\x91\x90aO1V[a\x14\xA4\x90`@\x81\x01\x90aM\xD3V[\x82\x81\x81\x10a\x14\xB4Wa\x14\xB4aE0V[\x90P` \x02\x01` \x81\x01\x90a\x14\xC9\x91\x90aNeV[`\x9F`\0a\x14\xDA`\xE0\x88\x01\x88aM\xD3V[\x86\x81\x81\x10a\x14\xEAWa\x14\xEAaE0V[\x90P` \x02\x81\x01\x90a\x14\xFC\x91\x90aO1V[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a\x15\x1D`\xE0\x88\x01\x88aM\xD3V[\x86\x81\x81\x10a\x15-Wa\x15-aE0V[\x90P` \x02\x81\x01\x90a\x15?\x91\x90aO1V[a\x15M\x90` \x81\x01\x90aM\xD3V[\x85\x81\x81\x10a\x15]Wa\x15]aE0V[\x90P` \x02\x01` \x81\x01\x90a\x15r\x91\x90a;\xF3V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\x15\xAD\x81aM\xB8V[\x91PPa\x14!V[P\x80a\x15\xC0\x81aM\xB8V[\x91PPa\x14\x08V[P`\0[a\x15\xDAa\x01\0\x84\x01\x84aN\x80V[\x90P\x81\x10\x15a\x16~Wa\x15\xF1a\x01\0\x84\x01\x84aN\x80V[\x82\x81\x81\x10a\x16\x01Wa\x16\x01aE0V[\x90P`@\x02\x01` \x01` \x81\x01\x90a\x16\x19\x91\x90a;\xF3V[`\xA1`\0a\x16+a\x01\0\x87\x01\x87aN\x80V[\x85\x81\x81\x10a\x16;Wa\x16;aE0V[\x90P`@\x02\x01`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP\x80\x80a\x16v\x90aM\xB8V[\x91PPa\x15\xCCV[Pa\x16\x8C` \x86\x01\x86aL\x89V[`\x9B\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\x16\xB6`@\x86\x01` \x87\x01aL\x89V[`\x9B\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90UB`\x98Ua\x16\xED``\x86\x01\x86aOGV[a\x16\xF9\x91`\x9C\x91a:oV[Pa\x17\n`\xA0\x86\x01`\x80\x87\x01aL\x89V[`\x9D\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x7F6\xA1\xFD{\xD5T\xF5\xC4(\xC9\x82\x9C\t\xC6`kL\x89;\x1F\xAD\xC8sZz\x12yW\x97D}\xEDa\x17R` \x87\x01\x87aL\x89V[a\x17b`@\x88\x01` \x89\x01aL\x89V[`@\x80Qc\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01[`@Q\x80\x91\x03\x90\xA1P[PPPPV[`\x9C\x80Ta\x17\x98\x90aM5V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x17\xC4\x90aM5V[\x80\x15a\x18\x11W\x80`\x1F\x10a\x17\xE6Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x18\x11V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x17\xF4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18aW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\x85\x91\x90aD\xCBV[a\x18\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xC6\x90aD\xE8V[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[a\x18\xE8a1\x91V[a\x18\xF2`\0a6\x8AV[V[`\x97T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x19NW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FUpdater must be the caller\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\xC6V[`\x99Tc\xFF\xFF\xFF\xFF\x16\x15\x80a\x19zWPa\x19k` \x84\x01\x84aL\x89V[`\x99Tc\xFF\xFF\xFF\xFF\x91\x82\x16\x91\x16\x10[a\x19\xB5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkStale RdTask`\xA0\x1B`D\x82\x01R`d\x01a\x05\xC6V[`\x9BT`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16a\x1A\x03W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x13\xDC\x08\x1C\xDD\x18]\x19H\x1D[\x9A[\x9A]`\x8A\x1B`D\x82\x01R`d\x01a\x05\xC6V[a\x1A\x13`\x80\x84\x01``\x85\x01aL\x89V[`\x9BT`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14a\x1AtW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7Freference block hash mismatch\0\0\0`D\x82\x01R`d\x01a\x05\xC6V[\x82`@Q` \x01a\x1A\x85\x91\x90aO\x8DV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82` \x015\x14a\x1A\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FreferenceTaskHash hash mismatch\0`D\x82\x01R`d\x01a\x05\xC6V[a\x1A\xFD``\x84\x01`@\x85\x01aL\x89V[`\x9BTc\xFF\xFF\xFF\xFF\x91\x82\x16\x91a\x1B\x1D\x91`\x01` \x1B\x90\x04\x16a8@aL\xBAV[c\xFF\xFF\xFF\xFF\x16\x11a\x1B`W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x077F\x16\xC6R\x077F\x17FR\x03`\x9C\x1B`D\x82\x01R`d\x01a\x05\xC6V[B`\x98Tb\x03\xF4\x80a\x1Br\x91\x90aL\xE2V[\x11a\x1B\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rlstale state 1`\x98\x1B`D\x82\x01R`d\x01a\x05\xC6V[`\0a\x1B\xE5\x83`@Q` \x01a\x1B\xC5\x91\x90aP\x1BV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x83a\x04I\x90aM)V[`\x9DT\x90\x91Pc\xFF\xFF\xFF\xFF\x16`\0[`\x9C\x80Ta\x1C\x01\x90aM5V[\x90P\x81\x10\x15a\x1C\x92W\x81`\xFF\x16\x83` \x01Q\x82\x81Q\x81\x10a\x1C$Wa\x1C$aE0V[` \x02` \x01\x01Qa\x1C6\x91\x90aMjV[`\x01`\x01``\x1B\x03\x16`d\x84`\0\x01Q\x83\x81Q\x81\x10a\x1CWWa\x1CWaE0V[` \x02` \x01\x01Q`\x01`\x01``\x1B\x03\x16a\x1Cr\x91\x90aM\x99V[\x10\x15a\x1C\x80WPPPPPPV[\x80a\x1C\x8A\x81aM\xB8V[\x91PPa\x1B\xF4V[P`\x80\x84\x015`\x9AU\x7F\xECh\xDB9\x18y\xB0\xF9\xF4 \xD1\xCD\xF3Gj\xFB\xDF\x08Z$b\xBFM+\x11\xDFxFb\x95\xCB\x17a\x1C\xC9` \x87\x01\x87aL\x89V[a\x17b``\x88\x01`@\x89\x01aL\x89V[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01\x81\x90R`\x9C\x80Ta\x1D\x10\x90aM5V[\x90P\x90Pa\x1D1`@Q\x80`@\x01`@R\x80``\x81R` \x01``\x81RP\x90V[\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1DIWa\x1DIa<`V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1DrW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P` \x82\x01R\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\x90Wa\x1D\x90a<`V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1D\xB9W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x81R` \x85\x01QQ`\0\x90`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\xDCWa\x1D\xDCa<`V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1E\x05W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80[\x87` \x01QQ\x81\x10\x15a\x1F\xB7Wa\x1EQ\x88` \x01Q\x82\x81Q\x81\x10a\x1E2Wa\x1E2aE0V[` \x02` \x01\x01Q\x80Q`\0\x90\x81R` \x91\x82\x01Q\x90\x91R`@\x90 \x90V[\x83\x82\x81Q\x81\x10a\x1EcWa\x1EcaE0V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80\x15a\x1F-W\x82a\x1E\x80`\x01\x83aP^V[\x81Q\x81\x10a\x1E\x90Wa\x1E\x90aE0V[` \x02` \x01\x01Q`\0\x1C\x83\x82\x81Q\x81\x10a\x1E\xADWa\x1E\xADaE0V[` \x02` \x01\x01Q`\0\x1C\x11a\x1F-W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: nonSignerPubkeys not sorted`d\x82\x01R`\x84\x01a\x05\xC6V[a\x1F\xA3a\x1F\x9C`\xA1`\0\x86\x85\x81Q\x81\x10a\x1FIWa\x1FIaE0V[` \x02` \x01\x01Q\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\xFF\x16\x8A` \x01Q\x84\x81Q\x81\x10a\x1F\x86Wa\x1F\x86aE0V[` \x02` \x01\x01Qa6\xDC\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x87\x90a2\x82V[\x95P\x80a\x1F\xAF\x81aM\xB8V[\x91PPa\x1E\x0CV[Pa\x1F\xC1\x85a7\xC0V[\x94P`\0[\x84\x81\x10\x15a!\xA5W`\x9C\x81\x81Ta\x1F\xDC\x90aM5V[\x81\x10a\x1F\xEAWa\x1F\xEAaE0V[\x81T`\x01\x16\x15a \tW\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C`\0\x81\x81R`\xA0` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x01T\x90\x82\x01R\x90\x92Pa J\x90\x87\x90a2\x82V[`\xFF\x83\x16`\0\x90\x81R`\x9E` \x90\x81R`@\x90\x91 T\x90\x86\x01Q\x80Q\x92\x98P`\x01`\x01``\x1B\x03\x90\x91\x16\x91\x83\x90\x81\x10a \x85Wa \x85aE0V[`\x01`\x01``\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x82\x01R\x84\x01Q\x80Q\x82\x90\x81\x10a \xB1Wa \xB1aE0V[` \x02` \x01\x01Q\x84`\0\x01Q\x82\x81Q\x81\x10a \xCFWa \xCFaE0V[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPP`\0[\x88` \x01QQ\x81\x10\x15a!\x92W`\x9F`\0\x85\x83\x81Q\x81\x10a!\x15Wa!\x15aE0V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82R\x81\x81\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 `\xFF\x87\x16\x82R\x90\x92R\x90 T\x85Q\x80Q`\x01`\x01``\x1B\x03\x90\x92\x16\x91\x84\x90\x81\x10a!`Wa!`aE0V[` \x02` \x01\x01\x81\x81Qa!t\x91\x90aPuV[`\x01`\x01``\x1B\x03\x16\x90RP\x80a!\x8A\x81aM\xB8V[\x91PPa \xF2V[P\x80a!\x9D\x81aM\xB8V[\x91PPa\x1F\xC6V[P`\0\x80a!\xBD\x8A\x88\x8B``\x01Q\x8C`\x80\x01Qa\x07DV[\x91P\x91P\x81a\"@W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: pairing precompile call fai`d\x82\x01Rb\x1B\x19Y`\xEA\x1B`\x84\x82\x01R`\xA4\x01a\x05\xC6V[\x80a\"\xB3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: signature is invalid\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\xC6V[P\x92\x95PPPPPP[\x92\x91PPV[a\"\xCBa1\x91V[`\0[a\"\xDB` \x84\x01\x84aM\xD3V[\x90P\x81\x10\x15a#\xA0W`\x9E`\0a\"\xF5` \x86\x01\x86aM\xD3V[\x84\x81\x81\x10a#\x05Wa#\x05aE0V[\x90P` \x02\x01` \x81\x01\x90a#\x1A\x91\x90a;\xF3V[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0\x90\x81 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U`\xA0\x91a#M\x90\x86\x01\x86aM\xD3V[\x84\x81\x81\x10a#]Wa#]aE0V[\x90P` \x02\x01` \x81\x01\x90a#r\x91\x90a;\xF3V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x81\x81U`\x01\x01U\x80a#\x98\x81aM\xB8V[\x91PPa\"\xCEV[P`\0[a#\xB1`@\x84\x01\x84aN\x1CV[\x90P\x81\x10\x15a$\xECWa#\xC7`@\x84\x01\x84aN\x1CV[\x82\x81\x81\x10a#\xD7Wa#\xD7aE0V[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a#\xEF\x91\x90aNeV[`\x9E`\0a$\0`@\x87\x01\x87aN\x1CV[\x85\x81\x81\x10a$\x10Wa$\x10aE0V[a$&\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa;\xF3V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x90\x81\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x17\x90\x91Ua$f\x90\x84\x01\x84aN\x1CV[\x82\x81\x81\x10a$vWa$vaE0V[\x90P`\x80\x02\x01`@\x01`\xA0`\0\x85\x80`@\x01\x90a$\x93\x91\x90aN\x1CV[\x85\x81\x81\x10a$\xA3Wa$\xA3aE0V[a$\xB9\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa;\xF3V[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a$\xE4\x81aM\xB8V[\x91PPa#\xA4V[P`\0[a$\xFD``\x84\x01\x84aN\x80V[\x90P\x81\x10\x15a%\xB5Wa%\x13``\x84\x01\x84aN\x80V[\x82\x81\x81\x10a%#Wa%#aE0V[\x90P`@\x02\x01` \x01` \x81\x01\x90a%;\x91\x90aNeV[`\x9E`\0a%L``\x87\x01\x87aN\x80V[\x85\x81\x81\x10a%\\Wa%\\aE0V[a%r\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa;\xF3V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a%\xAD\x81aM\xB8V[\x91PPa$\xF0V[P`\0[a%\xC6`\x80\x84\x01\x84aN\xC9V[\x90P\x81\x10\x15a&bWa%\xDC`\x80\x84\x01\x84aN\xC9V[\x82\x81\x81\x10a%\xECWa%\xECaE0V[\x90P``\x02\x01` \x01`\xA0`\0\x85\x80`\x80\x01\x90a&\t\x91\x90aN\xC9V[\x85\x81\x81\x10a&\x19Wa&\x19aE0V[a&/\x92` ``\x90\x92\x02\x01\x90\x81\x01\x91Pa;\xF3V[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a&Z\x81aM\xB8V[\x91PPa%\xB9V[P`\0[a&s`\xA0\x84\x01\x84aM\xD3V[\x90P\x81\x10\x15a'\x9AW`\0[`\x9C\x80Ta&\x8C\x90aM5V[\x90P\x81\x10\x15a'EW`\x9F`\0a&\xA6`\xA0\x87\x01\x87aM\xD3V[\x85\x81\x81\x10a&\xB6Wa&\xB6aE0V[\x90P` \x02\x015\x81R` \x01\x90\x81R` \x01`\0 `\0`\x9C\x83\x81Ta&\xDB\x90aM5V[\x81\x10a&\xE9Wa&\xE9aE0V[\x81T`\x01\x16\x15a'\x08W\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U\x81a'=\x81aM\xB8V[\x92PPa&\x7FV[P`\xA1`\0a'W`\xA0\x86\x01\x86aM\xD3V[\x84\x81\x81\x10a'gWa'gaE0V[` \x90\x81\x02\x92\x90\x92\x015\x83RP\x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\xFF\x19\x16\x90U\x80a'\x92\x81aM\xB8V[\x91PPa&fV[P`\0[a'\xAB`\xC0\x84\x01\x84aM\xD3V[\x90P\x81\x10\x15a)\xFBWa'\xC1`\xC0\x84\x01\x84aM\xD3V[\x82\x81\x81\x10a'\xD1Wa'\xD1aE0V[\x90P` \x02\x81\x01\x90a'\xE3\x91\x90aO\x11V[a'\xF4\x90`\x80\x81\x01\x90``\x01a;\xF3V[`\xA1`\0a(\x05`\xC0\x87\x01\x87aM\xD3V[\x85\x81\x81\x10a(\x15Wa(\x15aE0V[\x90P` \x02\x81\x01\x90a('\x91\x90aO\x11V[`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP`\0[a(a`\xC0\x85\x01\x85aM\xD3V[\x83\x81\x81\x10a(qWa(qaE0V[\x90P` \x02\x81\x01\x90a(\x83\x91\x90aO\x11V[a(\x91\x90` \x81\x01\x90aM\xD3V[\x90P\x81\x10\x15a)\xE8Wa(\xA7`\xC0\x85\x01\x85aM\xD3V[\x83\x81\x81\x10a(\xB7Wa(\xB7aE0V[\x90P` \x02\x81\x01\x90a(\xC9\x91\x90aO\x11V[a(\xD7\x90`@\x81\x01\x90aM\xD3V[\x82\x81\x81\x10a(\xE7Wa(\xE7aE0V[\x90P` \x02\x01` \x81\x01\x90a(\xFC\x91\x90aNeV[`\x9F`\0a)\r`\xC0\x88\x01\x88aM\xD3V[\x86\x81\x81\x10a)\x1DWa)\x1DaE0V[\x90P` \x02\x81\x01\x90a)/\x91\x90aO\x11V[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a)P`\xC0\x88\x01\x88aM\xD3V[\x86\x81\x81\x10a)`Wa)`aE0V[\x90P` \x02\x81\x01\x90a)r\x91\x90aO\x11V[a)\x80\x90` \x81\x01\x90aM\xD3V[\x85\x81\x81\x10a)\x90Wa)\x90aE0V[\x90P` \x02\x01` \x81\x01\x90a)\xA5\x91\x90a;\xF3V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a)\xE0\x81aM\xB8V[\x91PPa(TV[P\x80a)\xF3\x81aM\xB8V[\x91PPa'\x9EV[P`\0[a*\x0C`\xE0\x84\x01\x84aM\xD3V[\x90P\x81\x10\x15a+\xBFW`\0[a*%`\xE0\x85\x01\x85aM\xD3V[\x83\x81\x81\x10a*5Wa*5aE0V[\x90P` \x02\x81\x01\x90a*G\x91\x90aO1V[a*U\x90` \x81\x01\x90aM\xD3V[\x90P\x81\x10\x15a+\xACWa*k`\xE0\x85\x01\x85aM\xD3V[\x83\x81\x81\x10a*{Wa*{aE0V[\x90P` \x02\x81\x01\x90a*\x8D\x91\x90aO1V[a*\x9B\x90`@\x81\x01\x90aM\xD3V[\x82\x81\x81\x10a*\xABWa*\xABaE0V[\x90P` \x02\x01` \x81\x01\x90a*\xC0\x91\x90aNeV[`\x9F`\0a*\xD1`\xE0\x88\x01\x88aM\xD3V[\x86\x81\x81\x10a*\xE1Wa*\xE1aE0V[\x90P` \x02\x81\x01\x90a*\xF3\x91\x90aO1V[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a+\x14`\xE0\x88\x01\x88aM\xD3V[\x86\x81\x81\x10a+$Wa+$aE0V[\x90P` \x02\x81\x01\x90a+6\x91\x90aO1V[a+D\x90` \x81\x01\x90aM\xD3V[\x85\x81\x81\x10a+TWa+TaE0V[\x90P` \x02\x01` \x81\x01\x90a+i\x91\x90a;\xF3V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a+\xA4\x81aM\xB8V[\x91PPa*\x18V[P\x80a+\xB7\x81aM\xB8V[\x91PPa)\xFFV[P`\0[a+\xD1a\x01\0\x84\x01\x84aN\x80V[\x90P\x81\x10\x15a,uWa+\xE8a\x01\0\x84\x01\x84aN\x80V[\x82\x81\x81\x10a+\xF8Wa+\xF8aE0V[\x90P`@\x02\x01` \x01` \x81\x01\x90a,\x10\x91\x90a;\xF3V[`\xA1`\0a,\"a\x01\0\x87\x01\x87aN\x80V[\x85\x81\x81\x10a,2Wa,2aE0V[\x90P`@\x02\x01`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP\x80\x80a,m\x90aM\xB8V[\x91PPa+\xC3V[P`\x9A\x81\x90Ua,\x88` \x84\x01\x84aL\x89V[`\x9B\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua,\xB2`@\x84\x01` \x85\x01aL\x89V[`\x9B\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90UB`\x98Ua,\xE9``\x84\x01\x84aOGV[a,\xF5\x91`\x9C\x91a:oV[Pa-\x06`\xA0\x84\x01`\x80\x85\x01aL\x89V[`\x9D\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x7F&Ie\xEBk\xC46\xC6\xC4sC\x1D4\xAFV\xE82\xEC4O\xDF\xD4>\xE6\xAFo\xCEm ^\x84\xAFa-N` \x85\x01\x85aL\x89V[a-^`@\x86\x01` \x87\x01aL\x89V[`@\x80Qc\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1PPPV[a-\x8Ba1\x91V[`\x01`\x01`\xA0\x1B\x03\x81\x16a-\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x05\xC6V[a\x05\xD8\x81a6\x8AV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a.LW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.p\x91\x90aDdV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a.\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xC6\x90aD\x81V[`fT\x19\x81\x19`fT\x19\x16\x14a/\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\xC6V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x079V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a/uWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a/\x8FWP0;\x15\x80\x15a/\x8FWP`\0T`\xFF\x16`\x01\x14[a/\xF2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x05\xC6V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a0\x15W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a0 \x85`\0a8[V[a0)\x84a6\x8AV[`\x97\x80T\x83\x15\x15`\x01`\xA8\x1B\x02`\x01a\xFF\x01`\xA0\x1B\x03\x19\x90\x91\x16`\x01`\x01`\xA0\x1B\x03\x86\x16\x17\x17\x90U\x80\x15a0\x93W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01a\x17{V[PPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16a1(W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x05\xC6V[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x18\xF2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x05\xC6V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra2\x07a:\xF3V[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x80\x82\x01\x84\x90R`\0\x90\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a2:Wa2<V[\xFE[P\x80a2zW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCB[][\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x05\xC6V[PP\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra2\x9Ea;\x11V[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R\x83Q`@\x80\x84\x01\x91\x90\x91R\x90\x84\x01Q``\x83\x01R`\0\x90\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a2:WP\x80a2zW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCBXY\x19\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x05\xC6V[a3\x1Ea;/V[P`@\x80Q`\x80\x81\x01\x82R\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81\x83\x01\x90\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED``\x83\x01R\x81R\x81Q\x80\x83\x01\x90\x92R\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x81\x01\x91\x90\x91R\x81\x01\x91\x90\x91R\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0\x80\x80a4\x06`\0\x80Q` aP\x9E\x839\x81Q\x91R\x86aEFV[\x90P[a4\x12\x81a9EV[\x90\x93P\x91P`\0\x80Q` aP\x9E\x839\x81Q\x91R\x82\x83\t\x83\x14\x15a4LW`@\x80Q\x80\x82\x01\x90\x91R\x90\x81R` \x81\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80Q` aP\x9E\x839\x81Q\x91R`\x01\x82\x08\x90Pa4\tV[`@\x80Q\x80\x82\x01\x82R\x86\x81R` \x80\x82\x01\x86\x90R\x82Q\x80\x84\x01\x90\x93R\x86\x83R\x82\x01\x84\x90R`\0\x91\x82\x91\x90a4\x98a;TV[`\0[`\x02\x81\x10\x15a6]W`\0a4\xB1\x82`\x06aM\x99V[\x90P\x84\x82`\x02\x81\x10a4\xC5Wa4\xC5aE0V[` \x02\x01QQ\x83a4\xD7\x83`\0aL\xE2V[`\x0C\x81\x10a4\xE7Wa4\xE7aE0V[` \x02\x01R\x84\x82`\x02\x81\x10a4\xFEWa4\xFEaE0V[` \x02\x01Q` \x01Q\x83\x82`\x01a5\x15\x91\x90aL\xE2V[`\x0C\x81\x10a5%Wa5%aE0V[` \x02\x01R\x83\x82`\x02\x81\x10a5<Wa5<aE0V[` \x02\x01QQQ\x83a5O\x83`\x02aL\xE2V[`\x0C\x81\x10a5_Wa5_aE0V[` \x02\x01R\x83\x82`\x02\x81\x10a5vWa5vaE0V[` \x02\x01QQ`\x01` \x02\x01Q\x83a5\x8F\x83`\x03aL\xE2V[`\x0C\x81\x10a5\x9FWa5\x9FaE0V[` \x02\x01R\x83\x82`\x02\x81\x10a5\xB6Wa5\xB6aE0V[` \x02\x01Q` \x01Q`\0`\x02\x81\x10a5\xD1Wa5\xD1aE0V[` \x02\x01Q\x83a5\xE2\x83`\x04aL\xE2V[`\x0C\x81\x10a5\xF2Wa5\xF2aE0V[` \x02\x01R\x83\x82`\x02\x81\x10a6\tWa6\taE0V[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a6$Wa6$aE0V[` \x02\x01Q\x83a65\x83`\x05aL\xE2V[`\x0C\x81\x10a6EWa6EaE0V[` \x02\x01RP\x80a6U\x81aM\xB8V[\x91PPa4\x9BV[Pa6fa;sV[`\0` \x82a\x01\x80\x85`\x08\x8C\xFA\x91Q\x91\x9C\x91\x15\x15\x9BP\x90\x99PPPPPPPPPPV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x02\0\x82a\xFF\xFF\x16\x10a78W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Roscalar-too-large`\x80\x1B`D\x82\x01R`d\x01a\x05\xC6V[\x81a\xFF\xFF\x16`\x01\x14\x15a7LWP\x81a\"\xBDV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01\x81\x90R\x84\x90`\x01\x90[\x81a\xFF\xFF\x16\x86a\xFF\xFF\x16\x10a7\xB5W`\x01a\xFF\xFF\x87\x16`\xFF\x83\x16\x1C\x81\x16\x14\x15a7\x98Wa7\x95\x84\x84a2\x82V[\x93P[a7\xA2\x83\x84a2\x82V[\x92Pb\x01\xFF\xFE`\x01\x92\x83\x1B\x16\x91\x01a7hV[P\x91\x95\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q\x15\x80\x15a7\xE5WP` \x82\x01Q\x15[\x15a8\x03WPP`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01`\0\x80Q` aP\x9E\x839\x81Q\x91R\x84` \x01Qa86\x91\x90aEFV[a8N\x90`\0\x80Q` aP\x9E\x839\x81Q\x91RaP^V[\x90R\x92\x91PPV[\x91\x90PV[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a8|WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a8\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x05\xC6V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a9A\x82a0\x9AV[PPV[`\0\x80\x80`\0\x80Q` aP\x9E\x839\x81Q\x91R`\x03`\0\x80Q` aP\x9E\x839\x81Q\x91R\x86`\0\x80Q` aP\x9E\x839\x81Q\x91R\x88\x89\t\t\x08\x90P`\0a9\xBB\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\0\x80Q` aP\x9E\x839\x81Q\x91Ra9\xC7V[\x91\x95\x91\x94P\x90\x92PPPV[`\0\x80a9\xD2a;sV[a9\xDAa;\x91V[` \x80\x82R\x81\x81\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x88\x90R`\x80\x82\x01\x87\x90R`\xA0\x82\x01\x86\x90R\x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82\x80\x15a2:WP\x82a:dW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FBN254.expMod: call failure\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\xC6V[PQ\x95\x94PPPPPV[\x82\x80Ta:{\x90aM5V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82a:\x9DW`\0\x85Ua:\xE3V[\x82`\x1F\x10a:\xB6W\x82\x80\x01`\xFF\x19\x825\x16\x17\x85Ua:\xE3V[\x82\x80\x01`\x01\x01\x85U\x82\x15a:\xE3W\x91\x82\x01[\x82\x81\x11\x15a:\xE3W\x825\x82U\x91` \x01\x91\x90`\x01\x01\x90a:\xC8V[Pa:\xEF\x92\x91Pa;\xAFV[P\x90V[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80a;Ba;\xC4V[\x81R` \x01a;Oa;\xC4V[\x90R\x90V[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[[\x80\x82\x11\x15a:\xEFW`\0\x81U`\x01\x01a;\xB0V[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[\x805`\xFF\x81\x16\x81\x14a8VW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a<\x05W`\0\x80\xFD[a<\x0E\x82a;\xE2V[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05\xD8W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a<<W`\0\x80\xFD[\x815a<\x0E\x81a<\x15V[`\0` \x82\x84\x03\x12\x15a<YW`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a<\x98Wa<\x98a<`V[`@R\x90V[`@Qa\x01\0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a<\x98Wa<\x98a<`V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a<\xE9Wa<\xE9a<`V[`@R\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a=\x03W`\0\x80\xFD[a=\x0Ba<vV[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a=2W`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a=TWa=Ta<`V[\x80`@RP\x80`@\x84\x01\x85\x81\x11\x15a=kW`\0\x80\xFD[\x84[\x81\x81\x10\x15a7\xB5W\x805\x83R` \x92\x83\x01\x92\x01a=mV[`\0`\x80\x82\x84\x03\x12\x15a=\x97W`\0\x80\xFD[a=\x9Fa<vV[\x90Pa=\xAB\x83\x83a=!V[\x81Ra=\xBA\x83`@\x84\x01a=!V[` \x82\x01R\x92\x91PPV[`\0\x80`\0\x80a\x01 \x85\x87\x03\x12\x15a=\xDCW`\0\x80\xFD[\x845\x93Pa=\xED\x86` \x87\x01a<\xF1V[\x92Pa=\xFC\x86``\x87\x01a=\x85V[\x91Pa>\x0B\x86`\xE0\x87\x01a<\xF1V[\x90P\x92\x95\x91\x94P\x92PV[`\0`\xE0\x82\x84\x03\x12\x15a>(W`\0\x80\xFD[P\x91\x90PV[`\0a\x01\x80\x82\x84\x03\x12\x15a>(W`\0\x80\xFD[`\0a\x01 \x82\x84\x03\x12\x15a>(W`\0\x80\xFD[`\0\x80`\0\x80\x84\x86\x03`\xC0\x81\x12\x15a>kW`\0\x80\xFD[\x855`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a>\x82W`\0\x80\xFD[a>\x8E\x89\x83\x8A\x01a>\x16V[\x96P```\x1F\x19\x84\x01\x12\x15a>\xA2W`\0\x80\xFD[` \x88\x01\x95P`\x80\x88\x015\x92P\x80\x83\x11\x15a>\xBCW`\0\x80\xFD[a>\xC8\x89\x84\x8A\x01a>.V[\x94P`\xA0\x88\x015\x92P\x80\x83\x11\x15a>\xDEW`\0\x80\xFD[PPa>\xEC\x87\x82\x88\x01a>AV[\x91PP\x92\x95\x91\x94P\x92PV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a?%W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a?\tV[\x81\x81\x11\x15a?7W`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a?`W`\0\x80\xFD[\x825\x91Pa?p` \x84\x01a;\xE2V[\x90P\x92P\x92\x90PV[`\0\x80`\0\x83\x85\x03`\xE0\x81\x12\x15a?\x8FW`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a?\xA6W`\0\x80\xFD[\x90\x86\x01\x90`\xC0\x82\x89\x03\x12\x15a?\xBAW`\0\x80\xFD[\x81\x95P`\xA0`\x1F\x19\x84\x01\x12\x15a?\xCFW`\0\x80\xFD[` \x87\x01\x94P`\xC0\x87\x015\x92P\x80\x83\x11\x15a?\xE9W`\0\x80\xFD[PPa?\xF7\x86\x82\x87\x01a>.V[\x91PP\x92P\x92P\x92V[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a@\x1AWa@\x1Aa<`V[P`\x05\x1B` \x01\x90V[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a8VW`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a@IW`\0\x80\xFD[\x815` a@^a@Y\x83a@\x01V[a<\xC1V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a@}W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a@\x9FWa@\x92\x81a@$V[\x83R\x91\x83\x01\x91\x83\x01a@\x81V[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a@\xBBW`\0\x80\xFD[\x815` a@\xCBa@Y\x83a@\x01V[\x82\x81R`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a@\xEAW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a@\x9FWaA\0\x88\x82a<\xF1V[\x83R\x91\x83\x01\x91`@\x01a@\xEEV[`\0\x82`\x1F\x83\x01\x12aA\x1FW`\0\x80\xFD[\x815` aA/a@Y\x83a@\x01V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aANW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a@\x9FW\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aAqW`\0\x80\x81\xFD[aA\x7F\x89\x86\x83\x8B\x01\x01a@8V[\x84RP\x91\x83\x01\x91\x83\x01aARV[`\0a\x01\x80\x82\x84\x03\x12\x15aA\xA0W`\0\x80\xFD[aA\xA8a<\x9EV[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aA\xC1W`\0\x80\xFD[aA\xCD\x85\x83\x86\x01a@8V[\x83R` \x84\x015\x91P\x80\x82\x11\x15aA\xE3W`\0\x80\xFD[aA\xEF\x85\x83\x86\x01a@\xAAV[` \x84\x01R`@\x84\x015\x91P\x80\x82\x11\x15aB\x08W`\0\x80\xFD[aB\x14\x85\x83\x86\x01a@\xAAV[`@\x84\x01RaB&\x85``\x86\x01a=\x85V[``\x84\x01RaB8\x85`\xE0\x86\x01a<\xF1V[`\x80\x84\x01Ra\x01 \x84\x015\x91P\x80\x82\x11\x15aBRW`\0\x80\xFD[aB^\x85\x83\x86\x01a@8V[`\xA0\x84\x01Ra\x01@\x84\x015\x91P\x80\x82\x11\x15aBxW`\0\x80\xFD[aB\x84\x85\x83\x86\x01a@8V[`\xC0\x84\x01Ra\x01`\x84\x015\x91P\x80\x82\x11\x15aB\x9EW`\0\x80\xFD[PaB\xAB\x84\x82\x85\x01aA\x0EV[`\xE0\x83\x01RP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15aB\xCAW`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aB\xE7W`\0\x80\xFD[aB\xF3\x85\x82\x86\x01aA\x8DV[\x91PP\x92P\x92\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aC6W\x81Q`\x01`\x01``\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aC\x11V[P\x94\x95\x94PPPPPV[` \x81R`\0\x82Q`@` \x84\x01RaC]``\x84\x01\x82aB\xFDV[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01RaCz\x82\x82aB\xFDV[\x95\x94PPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15aC\x98W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aC\xAFW`\0\x80\xFD[aC\xBB\x87\x83\x88\x01a>\x16V[\x94P` \x86\x015\x91P\x80\x82\x11\x15aC\xD1W`\0\x80\xFD[PaC\xDE\x86\x82\x87\x01a>AV[\x92PP`@\x84\x015\x90P\x92P\x92P\x92V[\x80\x15\x15\x81\x14a\x05\xD8W`\0\x80\xFD[\x805a8V\x81aC\xEFV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aD\x1EW`\0\x80\xFD[\x845aD)\x81a<\x15V[\x93P` \x85\x015aD9\x81a<\x15V[\x92P`@\x85\x015aDI\x81a<\x15V[\x91P``\x85\x015aDY\x81aC\xEFV[\x93\x96\x92\x95P\x90\x93PPV[`\0` \x82\x84\x03\x12\x15aDvW`\0\x80\xFD[\x81Qa<\x0E\x81a<\x15V[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15aD\xDDW`\0\x80\xFD[\x81Qa<\x0E\x81aC\xEFV[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x82aEcWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aE\x7FW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aE\x9EW`\0\x80\xFD[\x806\x03\x83\x13\x15aE\xADW`\0\x80\xFD[\x92P\x92\x90PV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x81R`\0c\xFF\xFF\xFF\xFF\x80aE\xF2\x85a@$V[\x16` \x84\x01R\x80aF\x05` \x86\x01a@$V[\x16`@\x84\x01R\x80aF\x18`@\x86\x01a@$V[\x16``\x84\x01RPaF,``\x84\x01\x84aEhV[`\xE0`\x80\x85\x01RaFBa\x01\0\x85\x01\x82\x84aE\xB4V[\x91PPaFQ`\x80\x85\x01a@$V[c\xFF\xFF\xFF\xFF\x81\x16`\xA0\x85\x01RPaFk`\xA0\x85\x01\x85aEhV[\x84\x83\x03`\x1F\x19\x01`\xC0\x86\x01RaF\x82\x83\x82\x84aE\xB4V[\x92PPPaF\x92`\xC0\x85\x01a@$V[c\xFF\xFF\xFF\xFF\x81\x16`\xE0\x85\x01RP\x93\x92PPPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aF\xBDW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aF\xDCW`\0\x80\xFD[\x80`\x05\x1B6\x03\x83\x13\x15aE\xADW`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15aC6W`\xFFaG\x11\x83a;\xE2V[\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aF\xFEV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aG;W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aGZW`\0\x80\xFD[\x80`\x07\x1B6\x03\x83\x13\x15aE\xADW`\0\x80\xFD[\x805`\x01`\x01``\x1B\x03\x81\x16\x81\x14a8VW`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15aC6W`\xFFaG\xA6\x83a;\xE2V[\x16\x87R`\x01`\x01``\x1B\x03aG\xBC\x84\x84\x01aGlV[\x16\x83\x88\x01R`@\x82\x81\x015\x90\x88\x01R``\x80\x83\x015\x90\x88\x01R`\x80\x96\x87\x01\x96\x90\x91\x01\x90`\x01\x01aG\x93V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aG\xFEW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aH\x1DW`\0\x80\xFD[\x80`\x06\x1B6\x03\x83\x13\x15aE\xADW`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15aC6W`\xFFaHR\x83a;\xE2V[\x16\x87R`\x01`\x01``\x1B\x03aHh\x84\x84\x01aGlV[\x16\x87\x84\x01R`@\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aH?V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aH\x97W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aH\xB6W`\0\x80\xFD[``\x81\x026\x03\x83\x13\x15aE\xADW`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15aC6W`\xFFaH\xEB\x83a;\xE2V[\x16\x87RaI\x06\x83\x88\x01\x84\x84\x01\x805\x82R` \x90\x81\x015\x91\x01RV[``\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aH\xD8V[\x81\x83R`\0`\x01`\x01`\xFB\x1B\x03\x83\x11\x15aI2W`\0\x80\xFD[\x82`\x05\x1B\x80\x83` \x87\x017`\0\x94\x01` \x01\x93\x84RP\x91\x92\x91PPV[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15aC6W`\x01`\x01``\x1B\x03aIx\x83aGlV[\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aI_V[\x81\x83R`\0` \x80\x85\x01\x94P\x84\x84`\x05\x1B\x86\x01\x84`\0\x80[\x88\x81\x10\x15aJ9W\x84\x84\x03\x8AR\x825`~\x19\x896\x03\x01\x81\x12aI\xC3W\x82\x83\xFD[\x88\x01\x805\x85R`\x80aI\xD7\x88\x83\x01\x83aF\xA6V[\x82\x8A\x89\x01RaI\xE9\x83\x89\x01\x82\x84aF\xEEV[\x92PPP`@aI\xFB\x81\x84\x01\x84aF\xA6V[\x88\x84\x03\x83\x8A\x01RaJ\r\x84\x82\x84aIOV[\x93PPPP```\xFFaJ!\x82\x85\x01a;\xE2V[\x16\x96\x01\x95\x90\x95RP\x98\x85\x01\x98\x91\x85\x01\x91`\x01\x01aI\xA3V[P\x91\x98\x97PPPPPPPPV[\x81\x83R`\0` \x80\x85\x01\x94P\x84\x84`\x05\x1B\x86\x01\x84`\0\x80[\x88\x81\x10\x15aJ9W\x84\x84\x03\x8AR\x825`^\x19\x896\x03\x01\x81\x12aJ\x7FW\x82\x83\xFD[\x88\x01\x805\x85R``aJ\x93\x88\x83\x01\x83aF\xA6V[\x82\x8A\x89\x01RaJ\xA5\x83\x89\x01\x82\x84aF\xEEV[\x92PPP`@aJ\xB7\x81\x84\x01\x84aF\xA6V[\x93P\x87\x83\x03\x82\x89\x01RaJ\xCB\x83\x85\x83aIOV[\x9D\x8A\x01\x9D\x97PPP\x93\x87\x01\x93PP`\x01\x01aJ_V[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15aC6W\x815\x87R`\xFFaK\n\x84\x84\x01a;\xE2V[\x16\x87\x84\x01R`@\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aJ\xF1V[` \x81RaK<` \x82\x01aK6\x84aC\xFDV[\x15\x15\x90RV[`\0aKK` \x84\x01\x84aF\xA6V[a\x01 \x80`@\x86\x01RaKca\x01@\x86\x01\x83\x85aF\xEEV[\x92PaKr`@\x87\x01\x87aG$V[\x92P`\x1F\x19\x80\x87\x86\x03\x01``\x88\x01RaK\x8C\x85\x85\x84aG\x83V[\x94PaK\x9B``\x89\x01\x89aG\xE7V[\x94P\x91P\x80\x87\x86\x03\x01`\x80\x88\x01RaK\xB4\x85\x85\x84aH/V[\x94PaK\xC3`\x80\x89\x01\x89aH\x80V[\x94P\x91P\x80\x87\x86\x03\x01`\xA0\x88\x01RaK\xDC\x85\x85\x84aH\xC8V[\x94PaK\xEB`\xA0\x89\x01\x89aF\xA6V[\x94P\x91P\x80\x87\x86\x03\x01`\xC0\x88\x01RaL\x04\x85\x85\x84aI\x19V[\x94PaL\x13`\xC0\x89\x01\x89aF\xA6V[\x94P\x91P\x80\x87\x86\x03\x01`\xE0\x88\x01RaL,\x85\x85\x84aI\x8BV[\x94PaL;`\xE0\x89\x01\x89aF\xA6V[\x94P\x91Pa\x01\0\x81\x88\x87\x03\x01\x81\x89\x01RaLV\x86\x86\x85aJGV[\x95PaLd\x81\x8A\x01\x8AaG\xE7V[\x95P\x92PP\x80\x87\x86\x03\x01\x83\x88\x01RPaL~\x84\x84\x83aJ\xE1V[\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15aL\x9BW`\0\x80\xFD[a<\x0E\x82a@$V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aL\xD9WaL\xD9aL\xA4V[\x01\x94\x93PPPPV[`\0\x82\x19\x82\x11\x15aL\xF5WaL\xF5aL\xA4V[P\x01\x90V[``\x81\x01c\xFF\xFF\xFF\xFFaM\x0C\x84a@$V[\x16\x82R` \x83\x015` \x83\x01R`@\x83\x015`@\x83\x01R\x92\x91PPV[`\0a\"\xBD6\x83aA\x8DV[`\x01\x81\x81\x1C\x90\x82\x16\x80aMIW`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15a>(WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aM\x90WaM\x90aL\xA4V[\x02\x94\x93PPPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15aM\xB3WaM\xB3aL\xA4V[P\x02\x90V[`\0`\0\x19\x82\x14\x15aM\xCCWaM\xCCaL\xA4V[P`\x01\x01\x90V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aM\xEAW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aN\x04W`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15aE\xADW`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aN3W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aNMW`\0\x80\xFD[` \x01\x91P`\x07\x81\x90\x1B6\x03\x82\x13\x15aE\xADW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aNwW`\0\x80\xFD[a<\x0E\x82aGlV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aN\x97W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aN\xB1W`\0\x80\xFD[` \x01\x91P`\x06\x81\x90\x1B6\x03\x82\x13\x15aE\xADW`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aN\xE0W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aN\xFAW`\0\x80\xFD[` \x01\x91P``\x81\x026\x03\x82\x13\x15aE\xADW`\0\x80\xFD[`\0\x825`~\x19\x836\x03\x01\x81\x12aO'W`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x825`^\x19\x836\x03\x01\x81\x12aO'W`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aO^W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aOxW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15aE\xADW`\0\x80\xFD[` \x81R`\0c\xFF\xFF\xFF\xFF\x80aO\xA2\x85a@$V[\x16` \x84\x01R` \x84\x015`@\x84\x01R\x80aO\xBF`@\x86\x01a@$V[\x16``\x84\x01R\x80aO\xD2``\x86\x01a@$V[\x16`\x80\x84\x01RaO\xE5`\x80\x85\x01\x85aEhV[`\xC0`\xA0\x86\x01RaO\xFA`\xE0\x86\x01\x82\x84aE\xB4V[\x91PP\x81aP\n`\xA0\x87\x01a@$V[\x16`\xC0\x85\x01R\x80\x92PPP\x92\x91PPV[`\xA0\x81\x01c\xFF\xFF\xFF\xFFaP-\x84a@$V[\x16\x82R` \x83\x015` \x83\x01R`@\x83\x015`@\x83\x01R``\x83\x015``\x83\x01R`\x80\x83\x015`\x80\x83\x01R\x92\x91PPV[`\0\x82\x82\x10\x15aPpWaPpaL\xA4V[P\x03\x90V[`\0`\x01`\x01``\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15aP\x95WaP\x95aL\xA4V[\x03\x93\x92PPPV\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 \xE0\x19\xE8\x80t\x9B}N\xFFV\xDE\xF0Le\xA3(Fe\xFA9\xDAP\x1A;b\x02\xC1\xA0\x9C\x8E\xC2edsolcC\0\x08\x0C\x003";
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
        ///Calls the contract's `allowNonRootInit` (0x0ee0fdbd) function
        pub fn allow_non_root_init(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([14, 224, 253, 189], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkSignatures` (0x7d978897) function
        pub fn check_signatures(
            &self,
            msg_hash: [u8; 32],
            params: NonSignerStakesAndSignature,
        ) -> ::ethers::contract::builders::ContractCall<M, QuorumStakeTotals> {
            self.0
                .method_hash([125, 151, 136, 151], (msg_hash, params))
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
        ///Calls the contract's `latestCompletedRdTaskNumber` (0xd03a07b2) function
        pub fn latest_completed_rd_task_number(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([208, 58, 7, 178], ())
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
        ///Calls the contract's `process_eigen_op_update` (0x232b8e98) function
        pub fn process_eigen_op_update(
            &self,
            task: OpTask,
            task_response: OpTaskResponse,
            non_signer_stakes_and_signature: NonSignerStakesAndSignature,
            operator_state_info: OperatorStateInfo,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [35, 43, 142, 152],
                    (
                        task,
                        task_response,
                        non_signer_stakes_and_signature,
                        operator_state_info,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `process_eigen_rd_update` (0x79a0a853) function
        pub fn process_eigen_rd_update(
            &self,
            task: RdTask,
            task_response: RdTaskResponse,
            non_signer_stakes_and_signature: NonSignerStakesAndSignature,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [121, 160, 168, 83],
                    (task, task_response, non_signer_stakes_and_signature),
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
        ///Gets the contract's `EigenOpUpdateProcessed` event
        pub fn eigen_op_update_processed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, EigenOpUpdateProcessedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `EigenRdUpdateProcessed` event
        pub fn eigen_rd_update_processed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, EigenRdUpdateProcessedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `EigenReinitProcessed` event
        pub fn eigen_reinit_processed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, EigenReinitProcessedFilter>
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
        Hash,
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
                return Ok(GaspMultiRollupServiceEvents::EigenOpUpdateProcessedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = EigenRdUpdateProcessedFilter::decode_log(log) {
                return Ok(GaspMultiRollupServiceEvents::EigenRdUpdateProcessedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = EigenReinitProcessedFilter::decode_log(log) {
                return Ok(GaspMultiRollupServiceEvents::EigenReinitProcessedFilter(
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
                Self::EigenOpUpdateProcessedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EigenRdUpdateProcessedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EigenReinitProcessedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PausedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauserRegistrySetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnpausedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<EigenOpUpdateProcessedFilter> for GaspMultiRollupServiceEvents {
        fn from(value: EigenOpUpdateProcessedFilter) -> Self {
            Self::EigenOpUpdateProcessedFilter(value)
        }
    }
    impl ::core::convert::From<EigenRdUpdateProcessedFilter> for GaspMultiRollupServiceEvents {
        fn from(value: EigenRdUpdateProcessedFilter) -> Self {
            Self::EigenRdUpdateProcessedFilter(value)
        }
    }
    impl ::core::convert::From<EigenReinitProcessedFilter> for GaspMultiRollupServiceEvents {
        fn from(value: EigenReinitProcessedFilter) -> Self {
            Self::EigenReinitProcessedFilter(value)
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
        Hash,
    )]
    #[ethcall(name = "allowNonRootInit", abi = "allowNonRootInit()")]
    pub struct AllowNonRootInitCall;
    ///Container type for all input parameters for the `checkSignatures` function with signature `checkSignatures(bytes32,(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))` and selector `0x7d978897`
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
        abi = "checkSignatures(bytes32,(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))"
    )]
    pub struct CheckSignaturesCall {
        pub msg_hash: [u8; 32],
        pub params: NonSignerStakesAndSignature,
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
        Hash,
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
        Hash,
    )]
    #[ethcall(
        name = "lastOpUpdateBlockTimestamp",
        abi = "lastOpUpdateBlockTimestamp()"
    )]
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
        Hash,
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
        Hash,
    )]
    #[ethcall(
        name = "latestCompletedOpTaskNumber",
        abi = "latestCompletedOpTaskNumber()"
    )]
    pub struct LatestCompletedOpTaskNumberCall;
    ///Container type for all input parameters for the `latestCompletedRdTaskNumber` function with signature `latestCompletedRdTaskNumber()` and selector `0xd03a07b2`
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
        name = "latestCompletedRdTaskNumber",
        abi = "latestCompletedRdTaskNumber()"
    )]
    pub struct LatestCompletedRdTaskNumberCall;
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
    ///Container type for all input parameters for the `process_eigen_op_update` function with signature `process_eigen_op_update((uint32,uint32,uint32,bytes,uint32,bytes,uint32),(uint32,bytes32,bytes32),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]),(bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]))` and selector `0x232b8e98`
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
        name = "process_eigen_op_update",
        abi = "process_eigen_op_update((uint32,uint32,uint32,bytes,uint32,bytes,uint32),(uint32,bytes32,bytes32),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]),(bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]))"
    )]
    pub struct ProcessEigenOpUpdateCall {
        pub task: OpTask,
        pub task_response: OpTaskResponse,
        pub non_signer_stakes_and_signature: NonSignerStakesAndSignature,
        pub operator_state_info: OperatorStateInfo,
    }
    ///Container type for all input parameters for the `process_eigen_rd_update` function with signature `process_eigen_rd_update((uint32,uint256,uint32,uint32,bytes,uint32),(uint32,bytes32,bytes32,bytes32,bytes32),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))` and selector `0x79a0a853`
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
        name = "process_eigen_rd_update",
        abi = "process_eigen_rd_update((uint32,uint256,uint32,uint32,bytes,uint32),(uint32,bytes32,bytes32,bytes32,bytes32),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))"
    )]
    pub struct ProcessEigenRdUpdateCall {
        pub task: RdTask,
        pub task_response: RdTaskResponse,
        pub non_signer_stakes_and_signature: NonSignerStakesAndSignature,
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
        Hash,
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
        AllowNonRootInit(AllowNonRootInitCall),
        CheckSignatures(CheckSignaturesCall),
        Initialize(InitializeCall),
        LastOpUpdateBlockTimestamp(LastOpUpdateBlockTimestampCall),
        LatestCompletedOpTaskCreatedBlock(LatestCompletedOpTaskCreatedBlockCall),
        LatestCompletedOpTaskNumber(LatestCompletedOpTaskNumberCall),
        LatestCompletedRdTaskNumber(LatestCompletedRdTaskNumberCall),
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
            if let Ok(decoded) =
                <AllowNonRootInitCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AllowNonRootInit(decoded));
            }
            if let Ok(decoded) =
                <CheckSignaturesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CheckSignatures(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) =
                <LastOpUpdateBlockTimestampCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LastOpUpdateBlockTimestamp(decoded));
            }
            if let Ok(decoded) =
                <LatestCompletedOpTaskCreatedBlockCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::LatestCompletedOpTaskCreatedBlock(decoded));
            }
            if let Ok(decoded) =
                <LatestCompletedOpTaskNumberCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LatestCompletedOpTaskNumber(decoded));
            }
            if let Ok(decoded) =
                <LatestCompletedRdTaskNumberCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LatestCompletedRdTaskNumber(decoded));
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
                <ProcessEigenOpUpdateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ProcessEigenOpUpdate(decoded));
            }
            if let Ok(decoded) =
                <ProcessEigenRdUpdateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ProcessEigenRdUpdate(decoded));
            }
            if let Ok(decoded) =
                <ProcessEigenReinitCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ProcessEigenReinit(decoded));
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
                Self::AllowNonRootInit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CheckSignatures(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LastOpUpdateBlockTimestamp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LatestCompletedOpTaskCreatedBlock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LatestCompletedOpTaskNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LatestCompletedRdTaskNumber(element) => {
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
                Self::ProcessEigenOpUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProcessEigenRdUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProcessEigenReinit(element) => {
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
                Self::AllowNonRootInit(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckSignatures(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::LastOpUpdateBlockTimestamp(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestCompletedOpTaskCreatedBlock(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LatestCompletedOpTaskNumber(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestCompletedRdTaskNumber(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestPendingStateHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorAndQuorumToStakes(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorIdQuorumCount(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pause(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauseAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::PausedWithIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::Paused(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauserRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProcessEigenOpUpdate(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProcessEigenRdUpdate(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProcessEigenReinit(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<LastOpUpdateBlockTimestampCall> for GaspMultiRollupServiceCalls {
        fn from(value: LastOpUpdateBlockTimestampCall) -> Self {
            Self::LastOpUpdateBlockTimestamp(value)
        }
    }
    impl ::core::convert::From<LatestCompletedOpTaskCreatedBlockCall> for GaspMultiRollupServiceCalls {
        fn from(value: LatestCompletedOpTaskCreatedBlockCall) -> Self {
            Self::LatestCompletedOpTaskCreatedBlock(value)
        }
    }
    impl ::core::convert::From<LatestCompletedOpTaskNumberCall> for GaspMultiRollupServiceCalls {
        fn from(value: LatestCompletedOpTaskNumberCall) -> Self {
            Self::LatestCompletedOpTaskNumber(value)
        }
    }
    impl ::core::convert::From<LatestCompletedRdTaskNumberCall> for GaspMultiRollupServiceCalls {
        fn from(value: LatestCompletedRdTaskNumberCall) -> Self {
            Self::LatestCompletedRdTaskNumber(value)
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
    impl ::core::convert::From<ProcessEigenOpUpdateCall> for GaspMultiRollupServiceCalls {
        fn from(value: ProcessEigenOpUpdateCall) -> Self {
            Self::ProcessEigenOpUpdate(value)
        }
    }
    impl ::core::convert::From<ProcessEigenRdUpdateCall> for GaspMultiRollupServiceCalls {
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
        Hash,
    )]
    pub struct AllowNonRootInitReturn(pub bool);
    ///Container type for all return fields from the `checkSignatures` function with signature `checkSignatures(bytes32,(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))` and selector `0x7d978897`
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    pub struct LatestCompletedOpTaskNumberReturn(pub u32);
    ///Container type for all return fields from the `latestCompletedRdTaskNumber` function with signature `latestCompletedRdTaskNumber()` and selector `0xd03a07b2`
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
    pub struct LatestCompletedRdTaskNumberReturn(pub u32);
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
