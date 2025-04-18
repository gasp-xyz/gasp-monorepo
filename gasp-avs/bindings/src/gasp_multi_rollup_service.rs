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
                    ::std::borrow::ToOwned::to_owned("chainId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("chainId"),
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
                    ::std::borrow::ToOwned::to_owned("chainRdBatchNonce"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("chainRdBatchNonce"),
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
                    ::std::borrow::ToOwned::to_owned(
                        "latestCompletedRdTaskCreatedBlock",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "latestCompletedRdTaskCreatedBlock",
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
                    ::std::borrow::ToOwned::to_owned("processEigenOpUpdate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "processEigenOpUpdate",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("task"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
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
                    ::std::borrow::ToOwned::to_owned("processEigenRdUpdate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "processEigenRdUpdate",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("task"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
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
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
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
                    ::std::borrow::ToOwned::to_owned("processEigenReinit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("processEigenReinit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("task"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
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
                                    name: ::std::borrow::ToOwned::to_owned("merkleRoots"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ranges"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IRolldownPrimitives.Range[]",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_chainRdBatchNonce",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_latestCompletedRdTaskNumber",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_latestCompletedRdTaskCreatedBlock",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
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
                    ::std::borrow::ToOwned::to_owned("setRolldown"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setRolldown"),
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
                    ::std::borrow::ToOwned::to_owned("setUpdater"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setUpdater"),
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
                    ::std::borrow::ToOwned::to_owned("RolldownTargetUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RolldownTargetUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("rolldownAddress"),
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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[PaMm\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\x11W`\x005`\xE0\x1C\x80co\x0C0\xA4\x11a\x01%W\x80c\xD0:\x07\xB2\x11a\0\xADW\x80c\xE2\xA7\xCBf\x11a\0|W\x80c\xE2\xA7\xCBf\x14a\x05MW\x80c\xF2\xFD\xE3\x8B\x14a\x05eW\x80c\xF8N\x91\xFC\x14a\x05xW\x80c\xFA\xBC\x1C\xBC\x14a\x05\x81W\x80c\xFD\xC1]\xE8\x14a\x05\x94W`\0\x80\xFD[\x80c\xD0:\x07\xB2\x14a\x05\0W\x80c\xD3\x02p\xEF\x14a\x05\x10W\x80c\xDE\xB4\x03}\x14a\x05#W\x80c\xDF\x03L\xD0\x14a\x05:W`\0\x80\xFD[\x80c}\x97\x88\x97\x11a\0\xF4W\x80c}\x97\x88\x97\x14a\x04\x95W\x80c\x88o\x11\x95\x14a\x04\xB5W\x80c\x8D\xA5\xCB[\x14a\x04\xC8W\x80c\x9A\x8A\x05\x92\x14a\x04\xD9W\x80c\x9DT\xF4\x19\x14a\x04\xEDW`\0\x80\xFD[\x80co\x0C0\xA4\x14a\x04:W\x80cqP\x18\xA6\x14a\x04QW\x80cs\x84\x91\xD9\x14a\x04YW\x80cz\xD7Ua\x14a\x04lW`\0\x80\xFD[\x80c=\x9F\xB0\x0C\x11a\x01\xA8W\x80cRn>d\x11a\x01wW\x80cRn>d\x14a\x03\xD6W\x80cV\xB93\xD9\x14a\x03\xEAW\x80cY\\jg\x14a\x03\xFDW\x80cZ\xC8j\xB7\x14a\x04\x05W\x80c\\\x97Z\xBB\x14a\x04(W`\0\x80\xFD[\x80c=\x9F\xB0\x0C\x14a\x03\x1AW\x80cC\r;9\x14a\x03EW\x80cI\x9Do\xB6\x14a\x03zW\x80cM\xEA\xBC!\x14a\x03\xC6W`\0\x80\xFD[\x80c\x13d9\xDD\x11a\x01\xE4W\x80c\x13d9\xDD\x14a\x02\xB5W\x80c\x17\x1F\x1D[\x14a\x02\xC8W\x80c*\x84\x14\xFD\x14a\x02\xF2W\x80c9\x0Fup\x14a\x03\x07W`\0\x80\xFD[\x80c\x03\xD0\x97\xD2\x14a\x02\x16W\x80c\x0B\xF1d\x10\x14a\x02WW\x80c\x0E\xE0\xFD\xBD\x14a\x02|W\x80c\x10\xD6z/\x14a\x02\xA0W[`\0\x80\xFD[a\x02=a\x02$6`\x04a6eV[`\x9F` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T\x82V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[`\xA1Ta\x02g\x90c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02NV[`\x98Ta\x02\x90\x90`\x01`\xA8\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02NV[a\x02\xB3a\x02\xAE6`\x04a6\x95V[a\x05\xA7V[\0[a\x02\xB3a\x02\xC36`\x04a6\xB2V[a\x06cV[a\x02\xDBa\x02\xD66`\x04a80V[a\x07\xA2V[`@\x80Q\x92\x15\x15\x83R\x90\x15\x15` \x83\x01R\x01a\x02NV[a\x02\xFAa\t,V[`@Qa\x02N\x91\x90a8\x81V[a\x02\xB3a\x03\x156`\x04a8\xEFV[a\t\xBAV[`\x97Ta\x03-\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02NV[a\x03ha\x03S6`\x04a6\xB2V[`\xA0` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02NV[a\x03\xAEa\x03\x886`\x04a9`V[`\x9E` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\x01`\x01``\x1B\x03\x16\x81V[`@Q`\x01`\x01``\x1B\x03\x90\x91\x16\x81R` \x01a\x02NV[`\x9CTa\x02g\x90c\xFF\xFF\xFF\xFF\x16\x81V[`\x98Ta\x02\x90\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[a\x02\xB3a\x03\xF86`\x04a:[V[a\x0B)V[a\x02\xB3a\r\x9DV[a\x02\x90a\x04\x136`\x04a6eV[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`fT[`@Q\x90\x81R` \x01a\x02NV[`\x9ATa\x02g\x90`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x02\xB3a\x0EdV[a\x02\xB3a\x04g6`\x04a;WV[a\x0ExV[a\x03\xAEa\x04z6`\x04a6eV[`\x9D` R`\0\x90\x81R`@\x90 T`\x01`\x01``\x1B\x03\x16\x81V[a\x04\xA8a\x04\xA36`\x04a>oV[a\x14 V[`@Qa\x02N\x91\x90a>\xF9V[`eTa\x03-\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x03-V[`\x97Ta\x03h\x90`\x01`\xC0\x1B\x90\x04`\xFF\x16\x81V[a\x02\xB3a\x04\xFB6`\x04a6\x95V[a\x1A\nV[`\x9ATa\x02g\x90c\xFF\xFF\xFF\xFF\x16\x81V[a\x02\xB3a\x05\x1E6`\x04a?;V[a\x1A4V[`\x97Ta\x02g\x90`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`\x98Ta\x03-\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x9ATa\x02g\x90d\x01\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x02\xB3a\x05s6`\x04a6\x95V[a\x1F9V[a\x04,`\x99T\x81V[a\x02\xB3a\x05\x8F6`\x04a6\xB2V[a\x1F\xAFV[a\x02\xB3a\x05\xA26`\x04a6\x95V[a!\x0BV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xFAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x1E\x91\x90a?\xDFV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x06WW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06N\x90a?\xFCV[`@Q\x80\x91\x03\x90\xFD[a\x06`\x81a!gV[PV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xABW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xCF\x91\x90a@FV[a\x06\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06N\x90a@cV[`fT\x81\x81\x16\x14a\x07dW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06NV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`\0\x80`\0\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x87\x87`\0\x01Q\x88` \x01Q\x88`\0\x01Q`\0`\x02\x81\x10a\x07\xEAWa\x07\xEAa@\xABV[` \x02\x01Q\x89Q`\x01` \x02\x01Q\x8A` \x01Q`\0`\x02\x81\x10a\x08\x0FWa\x08\x0Fa@\xABV[` \x02\x01Q\x8B` \x01Q`\x01`\x02\x81\x10a\x08+Wa\x08+a@\xABV[` \x90\x81\x02\x91\x90\x91\x01Q\x8CQ\x8D\x83\x01Q`@Qa\x08\x88\x9A\x99\x98\x97\x96\x95\x94\x01\x98\x89R` \x89\x01\x97\x90\x97R`@\x88\x01\x95\x90\x95R``\x87\x01\x93\x90\x93R`\x80\x86\x01\x91\x90\x91R`\xA0\x85\x01R`\xC0\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01Ra\x01 \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\0\x1Ca\x08\xAB\x91\x90a@\xC1V[\x90Pa\t\x1Ea\x08\xC4a\x08\xBD\x88\x84a\"^V[\x86\x90a\"\xEFV[a\x08\xCCa#\x84V[a\t\x14a\t\x05\x85a\x08\xFF`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x01\x82R`\x02\x90\x82\x01R\x90V[\x90a\"^V[a\t\x0E\x8Ca$DV[\x90a\"\xEFV[\x88b\x01\xD4\xC0a$\xD3V[\x90\x98\x90\x97P\x95PPPPPPV[`\x9B\x80Ta\t9\x90a@\xE3V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\te\x90a@\xE3V[\x80\x15a\t\xB2W\x80`\x1F\x10a\t\x87Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t\xB2V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t\x95W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\t\xDAWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\t\xF4WP0;\x15\x80\x15a\t\xF4WP`\0T`\xFF\x16`\x01\x14[a\nWW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x06NV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\nzW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\n\x85\x86`\0a&\xF7V[a\n\x8E\x85a'\xE1V[`\x98\x80T`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x01a\xFF\x01`\xA0\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17`\x01`\xA8\x1B\x86\x15\x15\x02\x17\x90\x91U`\x97\x80T`\x01`\xA0\x1B\x92\x85\x16`\x01`\x01`\xC0\x1B\x03\x19\x90\x91\x16\x17\x91\x90\x91\x17\x90U\x80\x15a\x0B!W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01[`@Q\x80\x91\x03\x90\xA1[PPPPPPV[a\x0B1a(3V[\x85\x84\x14a\x0B\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FrdUpdate info length mismatch\0\0\0`D\x82\x01R`d\x01a\x06NV[a\x0B\x89\x88a(\x8DV[a\x0B\x96` \x8A\x01\x8AaA\x17V[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90Ua\x0B\xCD`@\x8A\x01` \x8B\x01aA\x17V[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x01`@\x1B\x02c\xFF\xFF\xFF\xFF`@\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90UB`\x99Ua\x0C\x03`\x80\x8A\x01\x8AaA2V[a\x0C\x0F\x91`\x9B\x91a4\xE1V[Pa\x0C `\xC0\x8A\x01`\xA0\x8B\x01aA\x17V[`\x9C\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U`\0[\x86\x81\x10\x15a\x0C\xE9W`\x97T`\x01`\x01`\xA0\x1B\x03\x16c\x08\xF4-@\x89\x89\x84\x81\x81\x10a\x0ChWa\x0Cha@\xABV[\x90P` \x02\x015\x88\x88\x85\x81\x81\x10a\x0C\x81Wa\x0C\x81a@\xABV[\x90P`@\x02\x01`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\xA4\x92\x91\x90aAxV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0C\xBEW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0C\xD2W=`\0\x80>=`\0\xFD[PPPP\x80\x80a\x0C\xE1\x90aA\xACV[\x91PPa\x0C=V[P`\x97\x80Tc\xFF\xFF\xFF\xFF\x80\x86\x16`\x01`\xA0\x1B\x02c\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90\x91U`\x9A\x80T\x84\x83\x16c\xFF\xFF\xFF\xFF\x19\x91\x82\x16\x17\x90\x91U`\xA1\x80T\x92\x84\x16\x92\x90\x91\x16\x91\x90\x91\x17\x90U\x7F&Ie\xEBk\xC46\xC6\xC4sC\x1D4\xAFV\xE82\xEC4O\xDF\xD4>\xE6\xAFo\xCEm ^\x84\xAFa\rb` \x8B\x01\x8BaA\x17V[a\rr`@\x8C\x01` \x8D\x01aA\x17V[`@\x80Qc\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1PPPPPPPPPV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xE5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\t\x91\x90a@FV[a\x0E%W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06N\x90a@cV[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[a\x0Ela(3V[a\x0Ev`\0a'\xE1V[V[`fT\x15a\x0E\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FPausable: contract is paused\0\0\0\0`D\x82\x01R`d\x01a\x06NV[`\x98T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0F\"W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FUpdater must be the caller\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06NV[`\x97T`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16a\x0FB`\x80\x84\x01``\x85\x01aA\x17V[c\xFF\xFF\xFF\xFF\x16\x14a\x0F\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FchainRdBatchNonce mismatch\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06NV[a\x0F\xA5`@\x84\x01` \x85\x01aA\xDCV[`\x01`\x01`@\x1B\x03\x16F\x14a\x0F\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x15\xDC\x9B\xDB\x99\xC8\x18\xDA\x18Z[\x92Y`\x9A\x1B`D\x82\x01R`d\x01a\x06NV[`\x9ATc\xFF\xFF\xFF\xFF\x16\x15\x80a\x10\x18WPa\x10\t` \x84\x01\x84aA\x17V[`\x9ATc\xFF\xFF\xFF\xFF\x91\x82\x16\x91\x16\x10[a\x10SW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkStale RdTask`\xA0\x1B`D\x82\x01R`d\x01a\x06NV[`\x9AT`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16`\0\x03a\x10\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x13\xDC\x08\x1C\xDD\x18]\x19H\x1D[\x9A[\x9A]`\x8A\x1B`D\x82\x01R`d\x01a\x06NV[a\x10\xB4`\xC0\x84\x01`\xA0\x85\x01aA\x17V[`\x9AT`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14a\x11\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7Freference block hash mismatch\0\0\0`D\x82\x01R`d\x01a\x06NV[\x82`@Q` \x01a\x11&\x91\x90aBeV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82` \x015\x14a\x11\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FreferenceTaskHash hash mismatch\0`D\x82\x01R`d\x01a\x06NV[`\0a\x11\xC4\x83`@Q` \x01a\x11\xA4\x91\x90aCCV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x83a\x04\xA3\x90aC\xD6V[`\x9CT\x90\x91Pc\xFF\xFF\xFF\xFF\x16`\0[`\x9B\x80Ta\x11\xE0\x90a@\xE3V[\x90P\x81\x10\x15a\x12\xA9W\x81`\xFF\x16\x83` \x01Q\x82\x81Q\x81\x10a\x12\x03Wa\x12\x03a@\xABV[` \x02` \x01\x01Qa\x12\x15\x91\x90aC\xE2V[`\x01`\x01``\x1B\x03\x16`d\x84`\0\x01Q\x83\x81Q\x81\x10a\x126Wa\x126a@\xABV[` \x02` \x01\x01Q`\x01`\x01``\x1B\x03\x16a\x12Q\x91\x90aD\x11V[\x10\x15a\x12\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtFailed to meet quorum`X\x1B`D\x82\x01R`d\x01a\x06NV[\x80a\x12\xA1\x81aA\xACV[\x91PPa\x11\xD3V[P`@\x80Q\x80\x82\x01\x82R`\xA0\x86\x015\x81R`\xC0\x86\x015` \x82\x01\x90\x81R`\x97T\x92Qb#\xD0\xB5`\xE6\x1B\x81R`\x80\x88\x015`\x04\x82\x01R\x82Q`$\x82\x01R\x90Q`D\x82\x01R\x90\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x08\xF4-@\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x13\x1BW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x13/W=`\0\x80>=`\0\xFD[Pa\x13D\x92PPP`\x80\x86\x01``\x87\x01aA\x17V[a\x13O\x90`\x01aD0V[`\x97\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x01`\xA0\x1B\x02c\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90Ua\x13\x81` \x87\x01\x87aA\x17V[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\x13\xAB`\x80\x87\x01``\x88\x01aA\x17V[`\xA1\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x7F\xECh\xDB9\x18y\xB0\xF9\xF4 \xD1\xCD\xF3Gj\xFB\xDF\x08Z$b\xBFM+\x11\xDFxFb\x95\xCB\x17a\x13\xF3` \x88\x01\x88aA\x17V[a\x14\x03`\x80\x89\x01``\x8A\x01aA\x17V[`@\x80Qc\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01a\x0B\x18V[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01\x81\x90R`\x9B\x80Ta\x14W\x90a@\xE3V[\x90P\x90Pa\x14x`@Q\x80`@\x01`@R\x80``\x81R` \x01``\x81RP\x90V[\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14\x90Wa\x14\x90a6\xCBV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x14\xB9W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P` \x82\x01R\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14\xD7Wa\x14\xD7a6\xCBV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x15\0W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x81R` \x85\x01QQ`\0\x90`\x01`\x01`@\x1B\x03\x81\x11\x15a\x15#Wa\x15#a6\xCBV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x15LW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80[\x87` \x01QQ\x81\x10\x15a\x16\xFEWa\x15\x98\x88` \x01Q\x82\x81Q\x81\x10a\x15yWa\x15ya@\xABV[` \x02` \x01\x01Q\x80Q`\0\x90\x81R` \x91\x82\x01Q\x90\x91R`@\x90 \x90V[\x83\x82\x81Q\x81\x10a\x15\xAAWa\x15\xAAa@\xABV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80\x15a\x16tW\x82a\x15\xC7`\x01\x83aDXV[\x81Q\x81\x10a\x15\xD7Wa\x15\xD7a@\xABV[` \x02` \x01\x01Q`\0\x1C\x83\x82\x81Q\x81\x10a\x15\xF4Wa\x15\xF4a@\xABV[` \x02` \x01\x01Q`\0\x1C\x11a\x16tW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: nonSignerPubkeys not sorted`d\x82\x01R`\x84\x01a\x06NV[a\x16\xEAa\x16\xE3`\xA0`\0\x86\x85\x81Q\x81\x10a\x16\x90Wa\x16\x90a@\xABV[` \x02` \x01\x01Q\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\xFF\x16\x8A` \x01Q\x84\x81Q\x81\x10a\x16\xCDWa\x16\xCDa@\xABV[` \x02` \x01\x01Qa26\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x87\x90a\"\xEFV[\x95P\x80a\x16\xF6\x81aA\xACV[\x91PPa\x15SV[Pa\x17\x08\x85a3\x19V[\x94P`\0[\x84\x81\x10\x15a\x18\xECW`\x9B\x81\x81Ta\x17#\x90a@\xE3V[\x81\x10a\x171Wa\x171a@\xABV[\x81T`\x01\x16\x15a\x17PW\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C`\0\x81\x81R`\x9F` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x01T\x90\x82\x01R\x90\x92Pa\x17\x91\x90\x87\x90a\"\xEFV[`\xFF\x83\x16`\0\x90\x81R`\x9D` \x90\x81R`@\x90\x91 T\x90\x86\x01Q\x80Q\x92\x98P`\x01`\x01``\x1B\x03\x90\x91\x16\x91\x83\x90\x81\x10a\x17\xCCWa\x17\xCCa@\xABV[`\x01`\x01``\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x82\x01R\x84\x01Q\x80Q\x82\x90\x81\x10a\x17\xF8Wa\x17\xF8a@\xABV[` \x02` \x01\x01Q\x84`\0\x01Q\x82\x81Q\x81\x10a\x18\x16Wa\x18\x16a@\xABV[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPP`\0[\x88` \x01QQ\x81\x10\x15a\x18\xD9W`\x9E`\0\x85\x83\x81Q\x81\x10a\x18\\Wa\x18\\a@\xABV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82R\x81\x81\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 `\xFF\x87\x16\x82R\x90\x92R\x90 T\x85Q\x80Q`\x01`\x01``\x1B\x03\x90\x92\x16\x91\x84\x90\x81\x10a\x18\xA7Wa\x18\xA7a@\xABV[` \x02` \x01\x01\x81\x81Qa\x18\xBB\x91\x90aDoV[`\x01`\x01``\x1B\x03\x16\x90RP\x80a\x18\xD1\x81aA\xACV[\x91PPa\x189V[P\x80a\x18\xE4\x81aA\xACV[\x91PPa\x17\rV[P`\0\x80a\x19\x04\x8A\x88\x8B``\x01Q\x8C`\x80\x01Qa\x07\xA2V[\x91P\x91P\x81a\x19\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: pairing precompile call fai`d\x82\x01Rb\x1B\x19Y`\xEA\x1B`\x84\x82\x01R`\xA4\x01a\x06NV[\x80a\x19\xFAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: signature is invalid\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06NV[P\x92\x95PPPPPP[\x92\x91PPV[a\x1A\x12a(3V[`\x98\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`fT\x15a\x1A\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FPausable: contract is paused\0\0\0\0`D\x82\x01R`d\x01a\x06NV[`\x98T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1A\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FUpdater must be the caller\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06NV[`\x9AT`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x80\x15\x15\x80a\x1B\x05WP`\x98T`\x01`\xA8\x1B\x90\x04`\xFF\x16[\x15a\x1BQW`\x98T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1BLW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04\x17WF\x83`\xDC\x1B`D\x82\x01R`d\x01a\x06NV[a\x1B\x93V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1B\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdAuth1`\xD8\x1B`D\x82\x01R`d\x01a\x06NV[\x84`@Q` \x01a\x1B\xA4\x91\x90aD\x97V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x015\x14a\x1C\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FreferenceTaskHash hash mismatch\0`D\x82\x01R`d\x01a\x06NV[\x81`@Q` \x01a\x1C\x1D\x91\x90aI\xF5V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84`@\x015\x14a\x1C\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FoperatorStateInfo hash mismatch\0`D\x82\x01R`d\x01a\x06NV[c\xFF\xFF\xFF\xFF\x81\x16\x15a\x1E\x1AWa\x1C\xA1`\x80\x86\x01``\x87\x01aA\x17V[c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x14a\x1C\xFBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7Freference block mismatch\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06NV[`\0a\x1D1\x85`@Q` \x01a\x1D\x11\x91\x90aK\\V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x85a\x04\xA3\x90aC\xD6V[`\x9CT\x90\x91Pc\xFF\xFF\xFF\xFF\x16`\0[`\x9B\x80Ta\x1DM\x90a@\xE3V[\x90P\x81\x10\x15a\x1E\x16W\x81`\xFF\x16\x83` \x01Q\x82\x81Q\x81\x10a\x1DpWa\x1Dpa@\xABV[` \x02` \x01\x01Qa\x1D\x82\x91\x90aC\xE2V[`\x01`\x01``\x1B\x03\x16`d\x84`\0\x01Q\x83\x81Q\x81\x10a\x1D\xA3Wa\x1D\xA3a@\xABV[` \x02` \x01\x01Q`\x01`\x01``\x1B\x03\x16a\x1D\xBE\x91\x90aD\x11V[\x10\x15a\x1E\x04W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtFailed to meet quorum`X\x1B`D\x82\x01R`d\x01a\x06NV[\x80a\x1E\x0E\x81aA\xACV[\x91PPa\x1D@V[PPP[a\x1E#\x82a(\x8DV[a\x1E0` \x86\x01\x86aA\x17V[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90Ua\x1Eg`@\x86\x01` \x87\x01aA\x17V[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x01`@\x1B\x02c\xFF\xFF\xFF\xFF`@\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90UB`\x99Ua\x1E\x9D`\x80\x86\x01\x86aA2V[a\x1E\xA9\x91`\x9B\x91a4\xE1V[Pa\x1E\xBA`\xC0\x86\x01`\xA0\x87\x01aA\x17V[`\x9C\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x7F6\xA1\xFD{\xD5T\xF5\xC4(\xC9\x82\x9C\t\xC6`kL\x89;\x1F\xAD\xC8sZz\x12yW\x97D}\xEDa\x1F\x02` \x87\x01\x87aA\x17V[a\x1F\x12`@\x88\x01` \x89\x01aA\x17V[`@\x80Qc\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1PPPPPV[a\x1FAa(3V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1F\xA6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x06NV[a\x06`\x81a'\xE1V[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a \x02W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a &\x91\x90a?\xDFV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a VW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06N\x90a?\xFCV[`fT\x19\x81\x19`fT\x19\x16\x14a \xD4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06NV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x07\x97V[a!\x13a(3V[`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F/ \xCF\x1B\xDAgs\x90D\xC5\xBFWsS\x97\x0C=\xBC\x18;,rt\xD1\xE8XJ\x10&\x922g\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`\x01`\x01`\xA0\x1B\x03\x81\x16a!\xF5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x06NV[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\"za5eV[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x80\x82\x01\x84\x90R`\0\x90\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80a\"\xA9W\xFE[P\x80a\"\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCB[][\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x06NV[PP\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra#\x0Ba5\x83V[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R\x83Q`@\x80\x84\x01\x91\x90\x91R\x90\x84\x01Q``\x83\x01R`\0\x90\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80\x80a#FW\xFE[P\x80a\"\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCBXY\x19\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x06NV[a#\x8Ca5\xA1V[P`@\x80Q`\x80\x81\x01\x82R\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81\x83\x01\x90\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED``\x83\x01R\x81R\x81Q\x80\x83\x01\x90\x92R\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x81\x01\x91\x90\x91R\x81\x01\x91\x90\x91R\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0\x80\x80a$t`\0\x80Q` aM\x18\x839\x81Q\x91R\x86a@\xC1V[\x90P[a$\x80\x81a3\xB4V[\x90\x93P\x91P`\0\x80Q` aM\x18\x839\x81Q\x91R\x82\x83\t\x83\x03a$\xB9W`@\x80Q\x80\x82\x01\x90\x91R\x90\x81R` \x81\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80Q` aM\x18\x839\x81Q\x91R`\x01\x82\x08\x90Pa$wV[`@\x80Q\x80\x82\x01\x82R\x86\x81R` \x80\x82\x01\x86\x90R\x82Q\x80\x84\x01\x90\x93R\x86\x83R\x82\x01\x84\x90R`\0\x91\x82\x91\x90a%\x05a5\xC6V[`\0[`\x02\x81\x10\x15a&\xCAW`\0a%\x1E\x82`\x06aD\x11V[\x90P\x84\x82`\x02\x81\x10a%2Wa%2a@\xABV[` \x02\x01QQ\x83a%D\x83`\0aK\x8BV[`\x0C\x81\x10a%TWa%Ta@\xABV[` \x02\x01R\x84\x82`\x02\x81\x10a%kWa%ka@\xABV[` \x02\x01Q` \x01Q\x83\x82`\x01a%\x82\x91\x90aK\x8BV[`\x0C\x81\x10a%\x92Wa%\x92a@\xABV[` \x02\x01R\x83\x82`\x02\x81\x10a%\xA9Wa%\xA9a@\xABV[` \x02\x01QQQ\x83a%\xBC\x83`\x02aK\x8BV[`\x0C\x81\x10a%\xCCWa%\xCCa@\xABV[` \x02\x01R\x83\x82`\x02\x81\x10a%\xE3Wa%\xE3a@\xABV[` \x02\x01QQ`\x01` \x02\x01Q\x83a%\xFC\x83`\x03aK\x8BV[`\x0C\x81\x10a&\x0CWa&\x0Ca@\xABV[` \x02\x01R\x83\x82`\x02\x81\x10a&#Wa&#a@\xABV[` \x02\x01Q` \x01Q`\0`\x02\x81\x10a&>Wa&>a@\xABV[` \x02\x01Q\x83a&O\x83`\x04aK\x8BV[`\x0C\x81\x10a&_Wa&_a@\xABV[` \x02\x01R\x83\x82`\x02\x81\x10a&vWa&va@\xABV[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a&\x91Wa&\x91a@\xABV[` \x02\x01Q\x83a&\xA2\x83`\x05aK\x8BV[`\x0C\x81\x10a&\xB2Wa&\xB2a@\xABV[` \x02\x01RP\x80a&\xC2\x81aA\xACV[\x91PPa%\x08V[Pa&\xD3a5\xE5V[`\0` \x82a\x01\x80\x85`\x08\x8C\xFA\x91Q\x91\x9C\x91\x15\x15\x9BP\x90\x99PPPPPPPPPPV[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a'\x18WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a'\x9AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x06NV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a'\xDD\x82a!gV[PPV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0EvW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x06NV[`\0[a(\x9D` \x83\x01\x83aK\xA3V[\x90P\x81\x10\x15a)bW`\x9D`\0a(\xB7` \x85\x01\x85aK\xA3V[\x84\x81\x81\x10a(\xC7Wa(\xC7a@\xABV[\x90P` \x02\x01` \x81\x01\x90a(\xDC\x91\x90a6eV[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0\x90\x81 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U`\x9F\x91a)\x0F\x90\x85\x01\x85aK\xA3V[\x84\x81\x81\x10a)\x1FWa)\x1Fa@\xABV[\x90P` \x02\x01` \x81\x01\x90a)4\x91\x90a6eV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x81\x81U`\x01\x01U\x80a)Z\x81aA\xACV[\x91PPa(\x90V[P`\0[a)s`@\x83\x01\x83aK\xECV[\x90P\x81\x10\x15a*\xAEWa)\x89`@\x83\x01\x83aK\xECV[\x82\x81\x81\x10a)\x99Wa)\x99a@\xABV[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a)\xB1\x91\x90aL5V[`\x9D`\0a)\xC2`@\x86\x01\x86aK\xECV[\x85\x81\x81\x10a)\xD2Wa)\xD2a@\xABV[a)\xE8\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa6eV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x90\x81\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x17\x90\x91Ua*(\x90\x83\x01\x83aK\xECV[\x82\x81\x81\x10a*8Wa*8a@\xABV[\x90P`\x80\x02\x01`@\x01`\x9F`\0\x84\x80`@\x01\x90a*U\x91\x90aK\xECV[\x85\x81\x81\x10a*eWa*ea@\xABV[a*{\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa6eV[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a*\xA6\x81aA\xACV[\x91PPa)fV[P`\0[a*\xBF``\x83\x01\x83aLPV[\x90P\x81\x10\x15a+wWa*\xD5``\x83\x01\x83aLPV[\x82\x81\x81\x10a*\xE5Wa*\xE5a@\xABV[\x90P`@\x02\x01` \x01` \x81\x01\x90a*\xFD\x91\x90aL5V[`\x9D`\0a+\x0E``\x86\x01\x86aLPV[\x85\x81\x81\x10a+\x1EWa+\x1Ea@\xABV[a+4\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa6eV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a+o\x81aA\xACV[\x91PPa*\xB2V[P`\0[a+\x88`\x80\x83\x01\x83aL\x99V[\x90P\x81\x10\x15a,$Wa+\x9E`\x80\x83\x01\x83aL\x99V[\x82\x81\x81\x10a+\xAEWa+\xAEa@\xABV[\x90P``\x02\x01` \x01`\x9F`\0\x84\x80`\x80\x01\x90a+\xCB\x91\x90aL\x99V[\x85\x81\x81\x10a+\xDBWa+\xDBa@\xABV[a+\xF1\x92` ``\x90\x92\x02\x01\x90\x81\x01\x91Pa6eV[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a,\x1C\x81aA\xACV[\x91PPa+{V[P`\0[a,5`\xA0\x83\x01\x83aK\xA3V[\x90P\x81\x10\x15a-[W`\0[`\x9B\x80Ta,N\x90a@\xE3V[\x90P\x81\x10\x15a-\x07W`\x9E`\0a,h`\xA0\x86\x01\x86aK\xA3V[\x85\x81\x81\x10a,xWa,xa@\xABV[\x90P` \x02\x015\x81R` \x01\x90\x81R` \x01`\0 `\0`\x9B\x83\x81Ta,\x9D\x90a@\xE3V[\x81\x10a,\xABWa,\xABa@\xABV[\x81T`\x01\x16\x15a,\xCAW\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U\x80a,\xFF\x81aA\xACV[\x91PPa,AV[P`\xA0`\0a-\x18\x84\x83\x01\x85aK\xA3V[\x84\x81\x81\x10a-(Wa-(a@\xABV[` \x90\x81\x02\x92\x90\x92\x015\x83RP\x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\xFF\x19\x16\x90U\x80a-S\x81aA\xACV[\x91PPa,(V[P`\0[a-l`\xC0\x83\x01\x83aK\xA3V[\x90P\x81\x10\x15a/\xBCWa-\x82`\xC0\x83\x01\x83aK\xA3V[\x82\x81\x81\x10a-\x92Wa-\x92a@\xABV[\x90P` \x02\x81\x01\x90a-\xA4\x91\x90aL\xE1V[a-\xB5\x90`\x80\x81\x01\x90``\x01a6eV[`\xA0`\0a-\xC6`\xC0\x86\x01\x86aK\xA3V[\x85\x81\x81\x10a-\xD6Wa-\xD6a@\xABV[\x90P` \x02\x81\x01\x90a-\xE8\x91\x90aL\xE1V[`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP`\0[a.\"`\xC0\x84\x01\x84aK\xA3V[\x83\x81\x81\x10a.2Wa.2a@\xABV[\x90P` \x02\x81\x01\x90a.D\x91\x90aL\xE1V[a.R\x90` \x81\x01\x90aK\xA3V[\x90P\x81\x10\x15a/\xA9Wa.h`\xC0\x84\x01\x84aK\xA3V[\x83\x81\x81\x10a.xWa.xa@\xABV[\x90P` \x02\x81\x01\x90a.\x8A\x91\x90aL\xE1V[a.\x98\x90`@\x81\x01\x90aK\xA3V[\x82\x81\x81\x10a.\xA8Wa.\xA8a@\xABV[\x90P` \x02\x01` \x81\x01\x90a.\xBD\x91\x90aL5V[`\x9E`\0a.\xCE`\xC0\x87\x01\x87aK\xA3V[\x86\x81\x81\x10a.\xDEWa.\xDEa@\xABV[\x90P` \x02\x81\x01\x90a.\xF0\x91\x90aL\xE1V[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a/\x11`\xC0\x87\x01\x87aK\xA3V[\x86\x81\x81\x10a/!Wa/!a@\xABV[\x90P` \x02\x81\x01\x90a/3\x91\x90aL\xE1V[a/A\x90` \x81\x01\x90aK\xA3V[\x85\x81\x81\x10a/QWa/Qa@\xABV[\x90P` \x02\x01` \x81\x01\x90a/f\x91\x90a6eV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a/\xA1\x81aA\xACV[\x91PPa.\x15V[P\x80a/\xB4\x81aA\xACV[\x91PPa-_V[P`\0[a/\xCD`\xE0\x83\x01\x83aK\xA3V[\x90P\x81\x10\x15a1\x80W`\0[a/\xE6`\xE0\x84\x01\x84aK\xA3V[\x83\x81\x81\x10a/\xF6Wa/\xF6a@\xABV[\x90P` \x02\x81\x01\x90a0\x08\x91\x90aM\x01V[a0\x16\x90` \x81\x01\x90aK\xA3V[\x90P\x81\x10\x15a1mWa0,`\xE0\x84\x01\x84aK\xA3V[\x83\x81\x81\x10a0<Wa0<a@\xABV[\x90P` \x02\x81\x01\x90a0N\x91\x90aM\x01V[a0\\\x90`@\x81\x01\x90aK\xA3V[\x82\x81\x81\x10a0lWa0la@\xABV[\x90P` \x02\x01` \x81\x01\x90a0\x81\x91\x90aL5V[`\x9E`\0a0\x92`\xE0\x87\x01\x87aK\xA3V[\x86\x81\x81\x10a0\xA2Wa0\xA2a@\xABV[\x90P` \x02\x81\x01\x90a0\xB4\x91\x90aM\x01V[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a0\xD5`\xE0\x87\x01\x87aK\xA3V[\x86\x81\x81\x10a0\xE5Wa0\xE5a@\xABV[\x90P` \x02\x81\x01\x90a0\xF7\x91\x90aM\x01V[a1\x05\x90` \x81\x01\x90aK\xA3V[\x85\x81\x81\x10a1\x15Wa1\x15a@\xABV[\x90P` \x02\x01` \x81\x01\x90a1*\x91\x90a6eV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a1e\x81aA\xACV[\x91PPa/\xD9V[P\x80a1x\x81aA\xACV[\x91PPa/\xC0V[P`\0[a1\x92a\x01\0\x83\x01\x83aLPV[\x90P\x81\x10\x15a'\xDDWa1\xA9a\x01\0\x83\x01\x83aLPV[\x82\x81\x81\x10a1\xB9Wa1\xB9a@\xABV[\x90P`@\x02\x01` \x01` \x81\x01\x90a1\xD1\x91\x90a6eV[`\xA0`\0a1\xE3a\x01\0\x86\x01\x86aLPV[\x85\x81\x81\x10a1\xF3Wa1\xF3a@\xABV[\x90P`@\x02\x01`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP\x80\x80a2.\x90aA\xACV[\x91PPa1\x84V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x02\0\x82a\xFF\xFF\x16\x10a2\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Roscalar-too-large`\x80\x1B`D\x82\x01R`d\x01a\x06NV[\x81a\xFF\xFF\x16`\x01\x03a2\xA5WP\x81a\x1A\x04V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01\x81\x90R\x84\x90`\x01\x90[\x81a\xFF\xFF\x16\x86a\xFF\xFF\x16\x10a3\x0EW`\x01a\xFF\xFF\x87\x16`\xFF\x83\x16\x1C\x81\x16\x90\x03a2\xF1Wa2\xEE\x84\x84a\"\xEFV[\x93P[a2\xFB\x83\x84a\"\xEFV[\x92Pb\x01\xFF\xFE`\x01\x92\x83\x1B\x16\x91\x01a2\xC1V[P\x91\x95\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q\x15\x80\x15a3>WP` \x82\x01Q\x15[\x15a3\\WPP`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01`\0\x80Q` aM\x18\x839\x81Q\x91R\x84` \x01Qa3\x8F\x91\x90a@\xC1V[a3\xA7\x90`\0\x80Q` aM\x18\x839\x81Q\x91RaDXV[\x90R\x92\x91PPV[\x91\x90PV[`\0\x80\x80`\0\x80Q` aM\x18\x839\x81Q\x91R`\x03`\0\x80Q` aM\x18\x839\x81Q\x91R\x86`\0\x80Q` aM\x18\x839\x81Q\x91R\x88\x89\t\t\x08\x90P`\0a4*\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\0\x80Q` aM\x18\x839\x81Q\x91Ra46V[\x91\x95\x91\x94P\x90\x92PPPV[`\0\x80a4Aa5\xE5V[a4Ia6\x03V[` \x80\x82R\x81\x81\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x88\x90R`\x80\x82\x01\x87\x90R`\xA0\x82\x01\x86\x90R\x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82\x80a4\x86W\xFE[P\x82a4\xD4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FBN254.expMod: call failure\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06NV[PQ\x91PP[\x93\x92PPPV[\x82\x80Ta4\xED\x90a@\xE3V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82a5\x0FW`\0\x85Ua5UV[\x82`\x1F\x10a5(W\x82\x80\x01`\xFF\x19\x825\x16\x17\x85Ua5UV[\x82\x80\x01`\x01\x01\x85U\x82\x15a5UW\x91\x82\x01[\x82\x81\x11\x15a5UW\x825\x82U\x91` \x01\x91\x90`\x01\x01\x90a5:V[Pa5a\x92\x91Pa6!V[P\x90V[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80a5\xB4a66V[\x81R` \x01a5\xC1a66V[\x90R\x90V[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[[\x80\x82\x11\x15a5aW`\0\x81U`\x01\x01a6\"V[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[\x805`\xFF\x81\x16\x81\x14a3\xAFW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a6wW`\0\x80\xFD[a4\xDA\x82a6TV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06`W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a6\xA7W`\0\x80\xFD[\x815a4\xDA\x81a6\x80V[`\0` \x82\x84\x03\x12\x15a6\xC4W`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a7\x03Wa7\x03a6\xCBV[`@R\x90V[`@Qa\x01\0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a7\x03Wa7\x03a6\xCBV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a7TWa7Ta6\xCBV[`@R\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a7nW`\0\x80\xFD[a7va6\xE1V[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a7\x9DW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a7\xBFWa7\xBFa6\xCBV[\x80`@RP\x80`@\x84\x01\x85\x81\x11\x15a7\xD6W`\0\x80\xFD[\x84[\x81\x81\x10\x15a3\x0EW\x805\x83R` \x92\x83\x01\x92\x01a7\xD8V[`\0`\x80\x82\x84\x03\x12\x15a8\x02W`\0\x80\xFD[a8\na6\xE1V[\x90Pa8\x16\x83\x83a7\x8CV[\x81Ra8%\x83`@\x84\x01a7\x8CV[` \x82\x01R\x92\x91PPV[`\0\x80`\0\x80a\x01 \x85\x87\x03\x12\x15a8GW`\0\x80\xFD[\x845\x93Pa8X\x86` \x87\x01a7\\V[\x92Pa8g\x86``\x87\x01a7\xF0V[\x91Pa8v\x86`\xE0\x87\x01a7\\V[\x90P\x92\x95\x91\x94P\x92PV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a8\xAEW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a8\x92V[\x81\x81\x11\x15a8\xC0W`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[\x80\x15\x15\x81\x14a\x06`W`\0\x80\xFD[\x805a3\xAF\x81a8\xD6V[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a9\x07W`\0\x80\xFD[\x855a9\x12\x81a6\x80V[\x94P` \x86\x015a9\"\x81a6\x80V[\x93P`@\x86\x015a92\x81a6\x80V[\x92P``\x86\x015a9B\x81a8\xD6V[\x91P`\x80\x86\x015a9R\x81a6\x80V[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x80`@\x83\x85\x03\x12\x15a9sW`\0\x80\xFD[\x825\x91Pa9\x83` \x84\x01a6TV[\x90P\x92P\x92\x90PV[`\0a\x01\0\x82\x84\x03\x12\x15a9\x9FW`\0\x80\xFD[P\x91\x90PV[`\0a\x01 \x82\x84\x03\x12\x15a9\x9FW`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12a9\xCAW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a9\xE1W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a9\xFCW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a:\x15W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a:,W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x06\x1B\x85\x01\x01\x11\x15a9\xFCW`\0\x80\xFD[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a3\xAFW`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0\x80`\0`\xE0\x8A\x8C\x03\x12\x15a:yW`\0\x80\xFD[\x895`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a:\x90W`\0\x80\xFD[a:\x9C\x8D\x83\x8E\x01a9\x8CV[\x9AP` \x8C\x015\x91P\x80\x82\x11\x15a:\xB2W`\0\x80\xFD[a:\xBE\x8D\x83\x8E\x01a9\xA5V[\x99P`@\x8C\x015\x91P\x80\x82\x11\x15a:\xD4W`\0\x80\xFD[a:\xE0\x8D\x83\x8E\x01a9\xB8V[\x90\x99P\x97P``\x8C\x015\x91P\x80\x82\x11\x15a:\xF9W`\0\x80\xFD[Pa;\x06\x8C\x82\x8D\x01a:\x03V[\x90\x96P\x94Pa;\x19\x90P`\x80\x8B\x01a:GV[\x92Pa;'`\xA0\x8B\x01a:GV[\x91Pa;5`\xC0\x8B\x01a:GV[\x90P\x92\x95\x98P\x92\x95\x98P\x92\x95\x98V[`\0a\x01\x80\x82\x84\x03\x12\x15a9\x9FW`\0\x80\xFD[`\0\x80`\0a\x01@\x84\x86\x03\x12\x15a;mW`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a;\x84W`\0\x80\xFD[a;\x90\x87\x83\x88\x01a9\x8CV[\x94Pa;\x9F\x87` \x88\x01a9\x8CV[\x93Pa\x01 \x86\x015\x91P\x80\x82\x11\x15a;\xB6W`\0\x80\xFD[Pa;\xC3\x86\x82\x87\x01a;DV[\x91PP\x92P\x92P\x92V[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a;\xE6Wa;\xE6a6\xCBV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a<\x01W`\0\x80\xFD[\x815` a<\x16a<\x11\x83a;\xCDV[a7,V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a<5W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a<WWa<J\x81a:GV[\x83R\x91\x83\x01\x91\x83\x01a<9V[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a<sW`\0\x80\xFD[\x815` a<\x83a<\x11\x83a;\xCDV[\x82\x81R`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a<\xA2W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a<WWa<\xB8\x88\x82a7\\V[\x83R\x91\x83\x01\x91`@\x01a<\xA6V[`\0\x82`\x1F\x83\x01\x12a<\xD7W`\0\x80\xFD[\x815` a<\xE7a<\x11\x83a;\xCDV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a=\x06W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a<WW\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a=)W`\0\x80\x81\xFD[a=7\x89\x86\x83\x8B\x01\x01a;\xF0V[\x84RP\x91\x83\x01\x91\x83\x01a=\nV[`\0a\x01\x80\x82\x84\x03\x12\x15a=XW`\0\x80\xFD[a=`a7\tV[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a=yW`\0\x80\xFD[a=\x85\x85\x83\x86\x01a;\xF0V[\x83R` \x84\x015\x91P\x80\x82\x11\x15a=\x9BW`\0\x80\xFD[a=\xA7\x85\x83\x86\x01a<bV[` \x84\x01R`@\x84\x015\x91P\x80\x82\x11\x15a=\xC0W`\0\x80\xFD[a=\xCC\x85\x83\x86\x01a<bV[`@\x84\x01Ra=\xDE\x85``\x86\x01a7\xF0V[``\x84\x01Ra=\xF0\x85`\xE0\x86\x01a7\\V[`\x80\x84\x01Ra\x01 \x84\x015\x91P\x80\x82\x11\x15a>\nW`\0\x80\xFD[a>\x16\x85\x83\x86\x01a;\xF0V[`\xA0\x84\x01Ra\x01@\x84\x015\x91P\x80\x82\x11\x15a>0W`\0\x80\xFD[a><\x85\x83\x86\x01a;\xF0V[`\xC0\x84\x01Ra\x01`\x84\x015\x91P\x80\x82\x11\x15a>VW`\0\x80\xFD[Pa>c\x84\x82\x85\x01a<\xC6V[`\xE0\x83\x01RP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a>\x82W`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a>\x9FW`\0\x80\xFD[a>\xAB\x85\x82\x86\x01a=EV[\x91PP\x92P\x92\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a>\xEEW\x81Q`\x01`\x01``\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a>\xC9V[P\x94\x95\x94PPPPPV[` \x81R`\0\x82Q`@` \x84\x01Ra?\x15``\x84\x01\x82a>\xB5V[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01Ra?2\x82\x82a>\xB5V[\x95\x94PPPPPV[`\0\x80`\0\x80\x84\x86\x03`\xC0\x81\x12\x15a?RW`\0\x80\xFD[\x855`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a?iW`\0\x80\xFD[a?u\x89\x83\x8A\x01a9\x8CV[\x96P```\x1F\x19\x84\x01\x12\x15a?\x89W`\0\x80\xFD[` \x88\x01\x95P`\x80\x88\x015\x92P\x80\x83\x11\x15a?\xA3W`\0\x80\xFD[a?\xAF\x89\x84\x8A\x01a;DV[\x94P`\xA0\x88\x015\x92P\x80\x83\x11\x15a?\xC5W`\0\x80\xFD[PPa?\xD3\x87\x82\x88\x01a9\xA5V[\x91PP\x92\x95\x91\x94P\x92PV[`\0` \x82\x84\x03\x12\x15a?\xF1W`\0\x80\xFD[\x81Qa4\xDA\x81a6\x80V[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15a@XW`\0\x80\xFD[\x81Qa4\xDA\x81a8\xD6V[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x82a@\xDEWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80a@\xF7W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a9\x9FWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15aA)W`\0\x80\xFD[a4\xDA\x82a:GV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aAIW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aAcW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a9\xFCW`\0\x80\xFD[\x82\x81R``\x81\x01a4\xDA` \x83\x01\x84\x805\x82R` \x90\x81\x015\x91\x01RV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01aA\xBEWaA\xBEaA\x96V[P`\x01\x01\x90V[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a3\xAFW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aA\xEEW`\0\x80\xFD[a4\xDA\x82aA\xC5V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aB\x0EW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aB-W`\0\x80\xFD[\x806\x03\x83\x13\x15a9\xFCW`\0\x80\xFD[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x81Rc\xFF\xFF\xFF\xFFaBw\x83a:GV[\x16` \x82\x01R`\x01`\x01`@\x1B\x03aB\x91` \x84\x01aA\xC5V[\x16`@\x82\x01R`\0aB\xA5`@\x84\x01a:GV[c\xFF\xFF\xFF\xFF\x81\x16``\x84\x01RPaB\xBE``\x84\x01a:GV[c\xFF\xFF\xFF\xFF\x81\x16`\x80\x84\x01RPaB\xD7`\x80\x84\x01a:GV[c\xFF\xFF\xFF\xFF\x81\x16`\xA0\x84\x01RPaB\xF0`\xA0\x84\x01a:GV[c\xFF\xFF\xFF\xFF\x81\x16`\xC0\x84\x01RPaC\n`\xC0\x84\x01\x84aA\xF7V[a\x01\0\x80`\xE0\x86\x01RaC\"a\x01 \x86\x01\x83\x85aB<V[\x92PaC0`\xE0\x87\x01a:GV[c\xFF\xFF\xFF\xFF\x16\x94\x01\x93\x90\x93R\x93\x92PPPV[a\x01\0\x81\x01c\xFF\xFF\xFF\xFF\x80aCW\x85a:GV[\x16\x83R` \x84\x015` \x84\x01R`\x01`\x01`@\x1B\x03aCx`@\x86\x01aA\xC5V[\x16`@\x84\x01R\x80aC\x8B``\x86\x01a:GV[\x16``\x84\x01RP`\x80\x83\x015`\x80\x83\x01R`\xA0\x83\x015`\xA0\x83\x01R`\xC0\x83\x015`\xC0\x83\x01R`\xE0\x83\x015aC\xBE\x81a6\x80V[`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x92\x90\x92\x01\x91\x90\x91R\x91\x90PV[`\0a\x1A\x046\x83a=EV[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aD\x08WaD\x08aA\x96V[\x02\x94\x93PPPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15aD+WaD+aA\x96V[P\x02\x90V[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aDOWaDOaA\x96V[\x01\x94\x93PPPPV[`\0\x82\x82\x10\x15aDjWaDjaA\x96V[P\x03\x90V[`\0`\x01`\x01``\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15aD\x8FWaD\x8FaA\x96V[\x03\x93\x92PPPV[` \x81Rc\xFF\xFF\xFF\xFFaD\xA9\x83a:GV[\x16` \x82\x01R`\0aD\xBD` \x84\x01a:GV[c\xFF\xFF\xFF\xFF\x81\x16`@\x84\x01RPaD\xD6`@\x84\x01a:GV[c\xFF\xFF\xFF\xFF\x81\x16``\x84\x01RPaD\xEF``\x84\x01a:GV[c\xFF\xFF\xFF\xFF\x81\x16`\x80\x84\x01RPaE\t`\x80\x84\x01\x84aA\xF7V[a\x01\0\x80`\xA0\x86\x01RaE!a\x01 \x86\x01\x83\x85aB<V[\x92PaE/`\xA0\x87\x01a:GV[c\xFF\xFF\xFF\xFF\x81\x16`\xC0\x87\x01R\x91PaEJ`\xC0\x87\x01\x87aA\xF7V[\x86\x85\x03`\x1F\x19\x01`\xE0\x88\x01R\x92PaEc\x84\x84\x83aB<V[\x93PPaC0`\xE0\x87\x01a:GV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aE\x89W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aE\xA8W`\0\x80\xFD[\x80`\x05\x1B6\x03\x83\x13\x15a9\xFCW`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a>\xEEW`\xFFaE\xDD\x83a6TV[\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aE\xCAV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aF\x07W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aF&W`\0\x80\xFD[\x80`\x07\x1B6\x03\x83\x13\x15a9\xFCW`\0\x80\xFD[\x805`\x01`\x01``\x1B\x03\x81\x16\x81\x14a3\xAFW`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a>\xEEW`\xFFaFr\x83a6TV[\x16\x87R`\x01`\x01``\x1B\x03aF\x88\x84\x84\x01aF8V[\x16\x83\x88\x01R`@\x82\x81\x015\x90\x88\x01R``\x80\x83\x015\x90\x88\x01R`\x80\x96\x87\x01\x96\x90\x91\x01\x90`\x01\x01aF_V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aF\xCAW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aF\xE9W`\0\x80\xFD[\x80`\x06\x1B6\x03\x83\x13\x15a9\xFCW`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a>\xEEW`\xFFaG\x1E\x83a6TV[\x16\x87R`\x01`\x01``\x1B\x03aG4\x84\x84\x01aF8V[\x16\x87\x84\x01R`@\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aG\x0BV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aGcW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aG\x82W`\0\x80\xFD[``\x81\x026\x03\x83\x13\x15a9\xFCW`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a>\xEEW`\xFFaG\xB7\x83a6TV[\x16\x87RaG\xD2\x83\x88\x01\x84\x84\x01\x805\x82R` \x90\x81\x015\x91\x01RV[``\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aG\xA4V[\x81\x83R`\0`\x01`\x01`\xFB\x1B\x03\x83\x11\x15aG\xFEW`\0\x80\xFD[\x82`\x05\x1B\x80\x83` \x87\x017`\0\x94\x01` \x01\x93\x84RP\x91\x92\x91PPV[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a>\xEEW`\x01`\x01``\x1B\x03aHD\x83aF8V[\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aH+V[\x81\x83R`\0` \x80\x85\x01\x94P\x84\x84`\x05\x1B\x86\x01\x84`\0\x80[\x88\x81\x10\x15aI\x05W\x84\x84\x03\x8AR\x825`~\x19\x896\x03\x01\x81\x12aH\x8FW\x82\x83\xFD[\x88\x01\x805\x85R`\x80aH\xA3\x88\x83\x01\x83aErV[\x82\x8A\x89\x01RaH\xB5\x83\x89\x01\x82\x84aE\xBAV[\x92PPP`@aH\xC7\x81\x84\x01\x84aErV[\x88\x84\x03\x83\x8A\x01RaH\xD9\x84\x82\x84aH\x1BV[\x93PPPP```\xFFaH\xED\x82\x85\x01a6TV[\x16\x96\x01\x95\x90\x95RP\x98\x85\x01\x98\x91\x85\x01\x91`\x01\x01aHoV[P\x91\x98\x97PPPPPPPPV[`\0\x83\x83\x85R` \x80\x86\x01\x95P\x80\x85`\x05\x1B\x83\x01\x01\x84`\0\x80[\x88\x81\x10\x15aI\x05W\x85\x84\x03`\x1F\x19\x01\x8AR\x8256\x89\x90\x03`^\x19\x01\x81\x12aIRW\x82\x83\xFD[\x88\x01\x805\x85R``aIf\x87\x83\x01\x83aErV[\x82\x89\x89\x01RaIx\x83\x89\x01\x82\x84aE\xBAV[\x92PPP`@aI\x8A\x81\x84\x01\x84aErV[\x93P\x87\x83\x03\x82\x89\x01RaI\x9E\x83\x85\x83aH\x1BV[\x9D\x89\x01\x9D\x97PPP\x93\x86\x01\x93PP`\x01\x01aI-V[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a>\xEEW\x815\x87R`\xFFaI\xDD\x84\x84\x01a6TV[\x16\x87\x84\x01R`@\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aI\xC4V[` \x81RaJ\x0F` \x82\x01aJ\t\x84a8\xE4V[\x15\x15\x90RV[`\0aJ\x1E` \x84\x01\x84aErV[a\x01 \x80`@\x86\x01RaJ6a\x01@\x86\x01\x83\x85aE\xBAV[\x92PaJE`@\x87\x01\x87aE\xF0V[\x92P`\x1F\x19\x80\x87\x86\x03\x01``\x88\x01RaJ_\x85\x85\x84aFOV[\x94PaJn``\x89\x01\x89aF\xB3V[\x94P\x91P\x80\x87\x86\x03\x01`\x80\x88\x01RaJ\x87\x85\x85\x84aF\xFBV[\x94PaJ\x96`\x80\x89\x01\x89aGLV[\x94P\x91P\x80\x87\x86\x03\x01`\xA0\x88\x01RaJ\xAF\x85\x85\x84aG\x94V[\x94PaJ\xBE`\xA0\x89\x01\x89aErV[\x94P\x91P\x80\x87\x86\x03\x01`\xC0\x88\x01RaJ\xD7\x85\x85\x84aG\xE5V[\x94PaJ\xE6`\xC0\x89\x01\x89aErV[\x94P\x91P\x80\x87\x86\x03\x01`\xE0\x88\x01RaJ\xFF\x85\x85\x84aHWV[\x94PaK\x0E`\xE0\x89\x01\x89aErV[\x94P\x91Pa\x01\0\x81\x88\x87\x03\x01\x81\x89\x01RaK)\x86\x86\x85aI\x13V[\x95PaK7\x81\x8A\x01\x8AaF\xB3V[\x95P\x92PP\x80\x87\x86\x03\x01\x83\x88\x01RPaKQ\x84\x84\x83aI\xB4V[\x97\x96PPPPPPPV[``\x81\x01c\xFF\xFF\xFF\xFFaKn\x84a:GV[\x16\x82R` \x83\x015` \x83\x01R`@\x83\x015`@\x83\x01R\x92\x91PPV[`\0\x82\x19\x82\x11\x15aK\x9EWaK\x9EaA\x96V[P\x01\x90V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aK\xBAW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aK\xD4W`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a9\xFCW`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aL\x03W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aL\x1DW`\0\x80\xFD[` \x01\x91P`\x07\x81\x90\x1B6\x03\x82\x13\x15a9\xFCW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aLGW`\0\x80\xFD[a4\xDA\x82aF8V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aLgW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aL\x81W`\0\x80\xFD[` \x01\x91P`\x06\x81\x90\x1B6\x03\x82\x13\x15a9\xFCW`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aL\xB0W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aL\xCAW`\0\x80\xFD[` \x01\x91P``\x81\x026\x03\x82\x13\x15a9\xFCW`\0\x80\xFD[`\0\x825`~\x19\x836\x03\x01\x81\x12aL\xF7W`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x825`^\x19\x836\x03\x01\x81\x12aL\xF7W`\0\x80\xFD\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 E\xBF\xCB\xF7\xCD\x88\xAB+\x9E}y\x84\x955\x1F\x9E\xEE\xFDQ}V\xE5NSB\x85yT;\x02L;dsolcC\0\x08\r\x003";
    /// The bytecode of the contract.
    pub static GASPMULTIROLLUPSERVICE_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\x11W`\x005`\xE0\x1C\x80co\x0C0\xA4\x11a\x01%W\x80c\xD0:\x07\xB2\x11a\0\xADW\x80c\xE2\xA7\xCBf\x11a\0|W\x80c\xE2\xA7\xCBf\x14a\x05MW\x80c\xF2\xFD\xE3\x8B\x14a\x05eW\x80c\xF8N\x91\xFC\x14a\x05xW\x80c\xFA\xBC\x1C\xBC\x14a\x05\x81W\x80c\xFD\xC1]\xE8\x14a\x05\x94W`\0\x80\xFD[\x80c\xD0:\x07\xB2\x14a\x05\0W\x80c\xD3\x02p\xEF\x14a\x05\x10W\x80c\xDE\xB4\x03}\x14a\x05#W\x80c\xDF\x03L\xD0\x14a\x05:W`\0\x80\xFD[\x80c}\x97\x88\x97\x11a\0\xF4W\x80c}\x97\x88\x97\x14a\x04\x95W\x80c\x88o\x11\x95\x14a\x04\xB5W\x80c\x8D\xA5\xCB[\x14a\x04\xC8W\x80c\x9A\x8A\x05\x92\x14a\x04\xD9W\x80c\x9DT\xF4\x19\x14a\x04\xEDW`\0\x80\xFD[\x80co\x0C0\xA4\x14a\x04:W\x80cqP\x18\xA6\x14a\x04QW\x80cs\x84\x91\xD9\x14a\x04YW\x80cz\xD7Ua\x14a\x04lW`\0\x80\xFD[\x80c=\x9F\xB0\x0C\x11a\x01\xA8W\x80cRn>d\x11a\x01wW\x80cRn>d\x14a\x03\xD6W\x80cV\xB93\xD9\x14a\x03\xEAW\x80cY\\jg\x14a\x03\xFDW\x80cZ\xC8j\xB7\x14a\x04\x05W\x80c\\\x97Z\xBB\x14a\x04(W`\0\x80\xFD[\x80c=\x9F\xB0\x0C\x14a\x03\x1AW\x80cC\r;9\x14a\x03EW\x80cI\x9Do\xB6\x14a\x03zW\x80cM\xEA\xBC!\x14a\x03\xC6W`\0\x80\xFD[\x80c\x13d9\xDD\x11a\x01\xE4W\x80c\x13d9\xDD\x14a\x02\xB5W\x80c\x17\x1F\x1D[\x14a\x02\xC8W\x80c*\x84\x14\xFD\x14a\x02\xF2W\x80c9\x0Fup\x14a\x03\x07W`\0\x80\xFD[\x80c\x03\xD0\x97\xD2\x14a\x02\x16W\x80c\x0B\xF1d\x10\x14a\x02WW\x80c\x0E\xE0\xFD\xBD\x14a\x02|W\x80c\x10\xD6z/\x14a\x02\xA0W[`\0\x80\xFD[a\x02=a\x02$6`\x04a6eV[`\x9F` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T\x82V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[`\xA1Ta\x02g\x90c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02NV[`\x98Ta\x02\x90\x90`\x01`\xA8\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02NV[a\x02\xB3a\x02\xAE6`\x04a6\x95V[a\x05\xA7V[\0[a\x02\xB3a\x02\xC36`\x04a6\xB2V[a\x06cV[a\x02\xDBa\x02\xD66`\x04a80V[a\x07\xA2V[`@\x80Q\x92\x15\x15\x83R\x90\x15\x15` \x83\x01R\x01a\x02NV[a\x02\xFAa\t,V[`@Qa\x02N\x91\x90a8\x81V[a\x02\xB3a\x03\x156`\x04a8\xEFV[a\t\xBAV[`\x97Ta\x03-\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02NV[a\x03ha\x03S6`\x04a6\xB2V[`\xA0` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02NV[a\x03\xAEa\x03\x886`\x04a9`V[`\x9E` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\x01`\x01``\x1B\x03\x16\x81V[`@Q`\x01`\x01``\x1B\x03\x90\x91\x16\x81R` \x01a\x02NV[`\x9CTa\x02g\x90c\xFF\xFF\xFF\xFF\x16\x81V[`\x98Ta\x02\x90\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[a\x02\xB3a\x03\xF86`\x04a:[V[a\x0B)V[a\x02\xB3a\r\x9DV[a\x02\x90a\x04\x136`\x04a6eV[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`fT[`@Q\x90\x81R` \x01a\x02NV[`\x9ATa\x02g\x90`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x02\xB3a\x0EdV[a\x02\xB3a\x04g6`\x04a;WV[a\x0ExV[a\x03\xAEa\x04z6`\x04a6eV[`\x9D` R`\0\x90\x81R`@\x90 T`\x01`\x01``\x1B\x03\x16\x81V[a\x04\xA8a\x04\xA36`\x04a>oV[a\x14 V[`@Qa\x02N\x91\x90a>\xF9V[`eTa\x03-\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x03-V[`\x97Ta\x03h\x90`\x01`\xC0\x1B\x90\x04`\xFF\x16\x81V[a\x02\xB3a\x04\xFB6`\x04a6\x95V[a\x1A\nV[`\x9ATa\x02g\x90c\xFF\xFF\xFF\xFF\x16\x81V[a\x02\xB3a\x05\x1E6`\x04a?;V[a\x1A4V[`\x97Ta\x02g\x90`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`\x98Ta\x03-\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x9ATa\x02g\x90d\x01\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x02\xB3a\x05s6`\x04a6\x95V[a\x1F9V[a\x04,`\x99T\x81V[a\x02\xB3a\x05\x8F6`\x04a6\xB2V[a\x1F\xAFV[a\x02\xB3a\x05\xA26`\x04a6\x95V[a!\x0BV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xFAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x1E\x91\x90a?\xDFV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x06WW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06N\x90a?\xFCV[`@Q\x80\x91\x03\x90\xFD[a\x06`\x81a!gV[PV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xABW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xCF\x91\x90a@FV[a\x06\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06N\x90a@cV[`fT\x81\x81\x16\x14a\x07dW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06NV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`\0\x80`\0\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x87\x87`\0\x01Q\x88` \x01Q\x88`\0\x01Q`\0`\x02\x81\x10a\x07\xEAWa\x07\xEAa@\xABV[` \x02\x01Q\x89Q`\x01` \x02\x01Q\x8A` \x01Q`\0`\x02\x81\x10a\x08\x0FWa\x08\x0Fa@\xABV[` \x02\x01Q\x8B` \x01Q`\x01`\x02\x81\x10a\x08+Wa\x08+a@\xABV[` \x90\x81\x02\x91\x90\x91\x01Q\x8CQ\x8D\x83\x01Q`@Qa\x08\x88\x9A\x99\x98\x97\x96\x95\x94\x01\x98\x89R` \x89\x01\x97\x90\x97R`@\x88\x01\x95\x90\x95R``\x87\x01\x93\x90\x93R`\x80\x86\x01\x91\x90\x91R`\xA0\x85\x01R`\xC0\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01Ra\x01 \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\0\x1Ca\x08\xAB\x91\x90a@\xC1V[\x90Pa\t\x1Ea\x08\xC4a\x08\xBD\x88\x84a\"^V[\x86\x90a\"\xEFV[a\x08\xCCa#\x84V[a\t\x14a\t\x05\x85a\x08\xFF`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x01\x82R`\x02\x90\x82\x01R\x90V[\x90a\"^V[a\t\x0E\x8Ca$DV[\x90a\"\xEFV[\x88b\x01\xD4\xC0a$\xD3V[\x90\x98\x90\x97P\x95PPPPPPV[`\x9B\x80Ta\t9\x90a@\xE3V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\te\x90a@\xE3V[\x80\x15a\t\xB2W\x80`\x1F\x10a\t\x87Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t\xB2V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t\x95W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\t\xDAWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\t\xF4WP0;\x15\x80\x15a\t\xF4WP`\0T`\xFF\x16`\x01\x14[a\nWW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x06NV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\nzW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\n\x85\x86`\0a&\xF7V[a\n\x8E\x85a'\xE1V[`\x98\x80T`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x01a\xFF\x01`\xA0\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17`\x01`\xA8\x1B\x86\x15\x15\x02\x17\x90\x91U`\x97\x80T`\x01`\xA0\x1B\x92\x85\x16`\x01`\x01`\xC0\x1B\x03\x19\x90\x91\x16\x17\x91\x90\x91\x17\x90U\x80\x15a\x0B!W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01[`@Q\x80\x91\x03\x90\xA1[PPPPPPV[a\x0B1a(3V[\x85\x84\x14a\x0B\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FrdUpdate info length mismatch\0\0\0`D\x82\x01R`d\x01a\x06NV[a\x0B\x89\x88a(\x8DV[a\x0B\x96` \x8A\x01\x8AaA\x17V[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90Ua\x0B\xCD`@\x8A\x01` \x8B\x01aA\x17V[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x01`@\x1B\x02c\xFF\xFF\xFF\xFF`@\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90UB`\x99Ua\x0C\x03`\x80\x8A\x01\x8AaA2V[a\x0C\x0F\x91`\x9B\x91a4\xE1V[Pa\x0C `\xC0\x8A\x01`\xA0\x8B\x01aA\x17V[`\x9C\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U`\0[\x86\x81\x10\x15a\x0C\xE9W`\x97T`\x01`\x01`\xA0\x1B\x03\x16c\x08\xF4-@\x89\x89\x84\x81\x81\x10a\x0ChWa\x0Cha@\xABV[\x90P` \x02\x015\x88\x88\x85\x81\x81\x10a\x0C\x81Wa\x0C\x81a@\xABV[\x90P`@\x02\x01`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\xA4\x92\x91\x90aAxV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0C\xBEW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0C\xD2W=`\0\x80>=`\0\xFD[PPPP\x80\x80a\x0C\xE1\x90aA\xACV[\x91PPa\x0C=V[P`\x97\x80Tc\xFF\xFF\xFF\xFF\x80\x86\x16`\x01`\xA0\x1B\x02c\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90\x91U`\x9A\x80T\x84\x83\x16c\xFF\xFF\xFF\xFF\x19\x91\x82\x16\x17\x90\x91U`\xA1\x80T\x92\x84\x16\x92\x90\x91\x16\x91\x90\x91\x17\x90U\x7F&Ie\xEBk\xC46\xC6\xC4sC\x1D4\xAFV\xE82\xEC4O\xDF\xD4>\xE6\xAFo\xCEm ^\x84\xAFa\rb` \x8B\x01\x8BaA\x17V[a\rr`@\x8C\x01` \x8D\x01aA\x17V[`@\x80Qc\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1PPPPPPPPPV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xE5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\t\x91\x90a@FV[a\x0E%W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06N\x90a@cV[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[a\x0Ela(3V[a\x0Ev`\0a'\xE1V[V[`fT\x15a\x0E\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FPausable: contract is paused\0\0\0\0`D\x82\x01R`d\x01a\x06NV[`\x98T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0F\"W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FUpdater must be the caller\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06NV[`\x97T`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16a\x0FB`\x80\x84\x01``\x85\x01aA\x17V[c\xFF\xFF\xFF\xFF\x16\x14a\x0F\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FchainRdBatchNonce mismatch\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06NV[a\x0F\xA5`@\x84\x01` \x85\x01aA\xDCV[`\x01`\x01`@\x1B\x03\x16F\x14a\x0F\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x15\xDC\x9B\xDB\x99\xC8\x18\xDA\x18Z[\x92Y`\x9A\x1B`D\x82\x01R`d\x01a\x06NV[`\x9ATc\xFF\xFF\xFF\xFF\x16\x15\x80a\x10\x18WPa\x10\t` \x84\x01\x84aA\x17V[`\x9ATc\xFF\xFF\xFF\xFF\x91\x82\x16\x91\x16\x10[a\x10SW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkStale RdTask`\xA0\x1B`D\x82\x01R`d\x01a\x06NV[`\x9AT`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16`\0\x03a\x10\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x13\xDC\x08\x1C\xDD\x18]\x19H\x1D[\x9A[\x9A]`\x8A\x1B`D\x82\x01R`d\x01a\x06NV[a\x10\xB4`\xC0\x84\x01`\xA0\x85\x01aA\x17V[`\x9AT`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14a\x11\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7Freference block hash mismatch\0\0\0`D\x82\x01R`d\x01a\x06NV[\x82`@Q` \x01a\x11&\x91\x90aBeV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82` \x015\x14a\x11\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FreferenceTaskHash hash mismatch\0`D\x82\x01R`d\x01a\x06NV[`\0a\x11\xC4\x83`@Q` \x01a\x11\xA4\x91\x90aCCV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x83a\x04\xA3\x90aC\xD6V[`\x9CT\x90\x91Pc\xFF\xFF\xFF\xFF\x16`\0[`\x9B\x80Ta\x11\xE0\x90a@\xE3V[\x90P\x81\x10\x15a\x12\xA9W\x81`\xFF\x16\x83` \x01Q\x82\x81Q\x81\x10a\x12\x03Wa\x12\x03a@\xABV[` \x02` \x01\x01Qa\x12\x15\x91\x90aC\xE2V[`\x01`\x01``\x1B\x03\x16`d\x84`\0\x01Q\x83\x81Q\x81\x10a\x126Wa\x126a@\xABV[` \x02` \x01\x01Q`\x01`\x01``\x1B\x03\x16a\x12Q\x91\x90aD\x11V[\x10\x15a\x12\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtFailed to meet quorum`X\x1B`D\x82\x01R`d\x01a\x06NV[\x80a\x12\xA1\x81aA\xACV[\x91PPa\x11\xD3V[P`@\x80Q\x80\x82\x01\x82R`\xA0\x86\x015\x81R`\xC0\x86\x015` \x82\x01\x90\x81R`\x97T\x92Qb#\xD0\xB5`\xE6\x1B\x81R`\x80\x88\x015`\x04\x82\x01R\x82Q`$\x82\x01R\x90Q`D\x82\x01R\x90\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x08\xF4-@\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x13\x1BW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x13/W=`\0\x80>=`\0\xFD[Pa\x13D\x92PPP`\x80\x86\x01``\x87\x01aA\x17V[a\x13O\x90`\x01aD0V[`\x97\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x01`\xA0\x1B\x02c\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90Ua\x13\x81` \x87\x01\x87aA\x17V[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\x13\xAB`\x80\x87\x01``\x88\x01aA\x17V[`\xA1\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x7F\xECh\xDB9\x18y\xB0\xF9\xF4 \xD1\xCD\xF3Gj\xFB\xDF\x08Z$b\xBFM+\x11\xDFxFb\x95\xCB\x17a\x13\xF3` \x88\x01\x88aA\x17V[a\x14\x03`\x80\x89\x01``\x8A\x01aA\x17V[`@\x80Qc\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01a\x0B\x18V[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01\x81\x90R`\x9B\x80Ta\x14W\x90a@\xE3V[\x90P\x90Pa\x14x`@Q\x80`@\x01`@R\x80``\x81R` \x01``\x81RP\x90V[\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14\x90Wa\x14\x90a6\xCBV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x14\xB9W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P` \x82\x01R\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14\xD7Wa\x14\xD7a6\xCBV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x15\0W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x81R` \x85\x01QQ`\0\x90`\x01`\x01`@\x1B\x03\x81\x11\x15a\x15#Wa\x15#a6\xCBV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x15LW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80[\x87` \x01QQ\x81\x10\x15a\x16\xFEWa\x15\x98\x88` \x01Q\x82\x81Q\x81\x10a\x15yWa\x15ya@\xABV[` \x02` \x01\x01Q\x80Q`\0\x90\x81R` \x91\x82\x01Q\x90\x91R`@\x90 \x90V[\x83\x82\x81Q\x81\x10a\x15\xAAWa\x15\xAAa@\xABV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80\x15a\x16tW\x82a\x15\xC7`\x01\x83aDXV[\x81Q\x81\x10a\x15\xD7Wa\x15\xD7a@\xABV[` \x02` \x01\x01Q`\0\x1C\x83\x82\x81Q\x81\x10a\x15\xF4Wa\x15\xF4a@\xABV[` \x02` \x01\x01Q`\0\x1C\x11a\x16tW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: nonSignerPubkeys not sorted`d\x82\x01R`\x84\x01a\x06NV[a\x16\xEAa\x16\xE3`\xA0`\0\x86\x85\x81Q\x81\x10a\x16\x90Wa\x16\x90a@\xABV[` \x02` \x01\x01Q\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\xFF\x16\x8A` \x01Q\x84\x81Q\x81\x10a\x16\xCDWa\x16\xCDa@\xABV[` \x02` \x01\x01Qa26\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x87\x90a\"\xEFV[\x95P\x80a\x16\xF6\x81aA\xACV[\x91PPa\x15SV[Pa\x17\x08\x85a3\x19V[\x94P`\0[\x84\x81\x10\x15a\x18\xECW`\x9B\x81\x81Ta\x17#\x90a@\xE3V[\x81\x10a\x171Wa\x171a@\xABV[\x81T`\x01\x16\x15a\x17PW\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C`\0\x81\x81R`\x9F` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x01T\x90\x82\x01R\x90\x92Pa\x17\x91\x90\x87\x90a\"\xEFV[`\xFF\x83\x16`\0\x90\x81R`\x9D` \x90\x81R`@\x90\x91 T\x90\x86\x01Q\x80Q\x92\x98P`\x01`\x01``\x1B\x03\x90\x91\x16\x91\x83\x90\x81\x10a\x17\xCCWa\x17\xCCa@\xABV[`\x01`\x01``\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x82\x01R\x84\x01Q\x80Q\x82\x90\x81\x10a\x17\xF8Wa\x17\xF8a@\xABV[` \x02` \x01\x01Q\x84`\0\x01Q\x82\x81Q\x81\x10a\x18\x16Wa\x18\x16a@\xABV[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPP`\0[\x88` \x01QQ\x81\x10\x15a\x18\xD9W`\x9E`\0\x85\x83\x81Q\x81\x10a\x18\\Wa\x18\\a@\xABV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82R\x81\x81\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 `\xFF\x87\x16\x82R\x90\x92R\x90 T\x85Q\x80Q`\x01`\x01``\x1B\x03\x90\x92\x16\x91\x84\x90\x81\x10a\x18\xA7Wa\x18\xA7a@\xABV[` \x02` \x01\x01\x81\x81Qa\x18\xBB\x91\x90aDoV[`\x01`\x01``\x1B\x03\x16\x90RP\x80a\x18\xD1\x81aA\xACV[\x91PPa\x189V[P\x80a\x18\xE4\x81aA\xACV[\x91PPa\x17\rV[P`\0\x80a\x19\x04\x8A\x88\x8B``\x01Q\x8C`\x80\x01Qa\x07\xA2V[\x91P\x91P\x81a\x19\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: pairing precompile call fai`d\x82\x01Rb\x1B\x19Y`\xEA\x1B`\x84\x82\x01R`\xA4\x01a\x06NV[\x80a\x19\xFAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: signature is invalid\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06NV[P\x92\x95PPPPPP[\x92\x91PPV[a\x1A\x12a(3V[`\x98\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`fT\x15a\x1A\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FPausable: contract is paused\0\0\0\0`D\x82\x01R`d\x01a\x06NV[`\x98T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1A\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FUpdater must be the caller\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06NV[`\x9AT`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x80\x15\x15\x80a\x1B\x05WP`\x98T`\x01`\xA8\x1B\x90\x04`\xFF\x16[\x15a\x1BQW`\x98T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1BLW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04\x17WF\x83`\xDC\x1B`D\x82\x01R`d\x01a\x06NV[a\x1B\x93V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1B\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdAuth1`\xD8\x1B`D\x82\x01R`d\x01a\x06NV[\x84`@Q` \x01a\x1B\xA4\x91\x90aD\x97V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x015\x14a\x1C\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FreferenceTaskHash hash mismatch\0`D\x82\x01R`d\x01a\x06NV[\x81`@Q` \x01a\x1C\x1D\x91\x90aI\xF5V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84`@\x015\x14a\x1C\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FoperatorStateInfo hash mismatch\0`D\x82\x01R`d\x01a\x06NV[c\xFF\xFF\xFF\xFF\x81\x16\x15a\x1E\x1AWa\x1C\xA1`\x80\x86\x01``\x87\x01aA\x17V[c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x14a\x1C\xFBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7Freference block mismatch\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06NV[`\0a\x1D1\x85`@Q` \x01a\x1D\x11\x91\x90aK\\V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x85a\x04\xA3\x90aC\xD6V[`\x9CT\x90\x91Pc\xFF\xFF\xFF\xFF\x16`\0[`\x9B\x80Ta\x1DM\x90a@\xE3V[\x90P\x81\x10\x15a\x1E\x16W\x81`\xFF\x16\x83` \x01Q\x82\x81Q\x81\x10a\x1DpWa\x1Dpa@\xABV[` \x02` \x01\x01Qa\x1D\x82\x91\x90aC\xE2V[`\x01`\x01``\x1B\x03\x16`d\x84`\0\x01Q\x83\x81Q\x81\x10a\x1D\xA3Wa\x1D\xA3a@\xABV[` \x02` \x01\x01Q`\x01`\x01``\x1B\x03\x16a\x1D\xBE\x91\x90aD\x11V[\x10\x15a\x1E\x04W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtFailed to meet quorum`X\x1B`D\x82\x01R`d\x01a\x06NV[\x80a\x1E\x0E\x81aA\xACV[\x91PPa\x1D@V[PPP[a\x1E#\x82a(\x8DV[a\x1E0` \x86\x01\x86aA\x17V[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90Ua\x1Eg`@\x86\x01` \x87\x01aA\x17V[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x01`@\x1B\x02c\xFF\xFF\xFF\xFF`@\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90UB`\x99Ua\x1E\x9D`\x80\x86\x01\x86aA2V[a\x1E\xA9\x91`\x9B\x91a4\xE1V[Pa\x1E\xBA`\xC0\x86\x01`\xA0\x87\x01aA\x17V[`\x9C\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x7F6\xA1\xFD{\xD5T\xF5\xC4(\xC9\x82\x9C\t\xC6`kL\x89;\x1F\xAD\xC8sZz\x12yW\x97D}\xEDa\x1F\x02` \x87\x01\x87aA\x17V[a\x1F\x12`@\x88\x01` \x89\x01aA\x17V[`@\x80Qc\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1PPPPPV[a\x1FAa(3V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1F\xA6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x06NV[a\x06`\x81a'\xE1V[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a \x02W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a &\x91\x90a?\xDFV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a VW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06N\x90a?\xFCV[`fT\x19\x81\x19`fT\x19\x16\x14a \xD4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06NV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x07\x97V[a!\x13a(3V[`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F/ \xCF\x1B\xDAgs\x90D\xC5\xBFWsS\x97\x0C=\xBC\x18;,rt\xD1\xE8XJ\x10&\x922g\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`\x01`\x01`\xA0\x1B\x03\x81\x16a!\xF5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x06NV[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\"za5eV[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x80\x82\x01\x84\x90R`\0\x90\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80a\"\xA9W\xFE[P\x80a\"\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCB[][\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x06NV[PP\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra#\x0Ba5\x83V[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R\x83Q`@\x80\x84\x01\x91\x90\x91R\x90\x84\x01Q``\x83\x01R`\0\x90\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80\x80a#FW\xFE[P\x80a\"\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCBXY\x19\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x06NV[a#\x8Ca5\xA1V[P`@\x80Q`\x80\x81\x01\x82R\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81\x83\x01\x90\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED``\x83\x01R\x81R\x81Q\x80\x83\x01\x90\x92R\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x81\x01\x91\x90\x91R\x81\x01\x91\x90\x91R\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0\x80\x80a$t`\0\x80Q` aM\x18\x839\x81Q\x91R\x86a@\xC1V[\x90P[a$\x80\x81a3\xB4V[\x90\x93P\x91P`\0\x80Q` aM\x18\x839\x81Q\x91R\x82\x83\t\x83\x03a$\xB9W`@\x80Q\x80\x82\x01\x90\x91R\x90\x81R` \x81\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80Q` aM\x18\x839\x81Q\x91R`\x01\x82\x08\x90Pa$wV[`@\x80Q\x80\x82\x01\x82R\x86\x81R` \x80\x82\x01\x86\x90R\x82Q\x80\x84\x01\x90\x93R\x86\x83R\x82\x01\x84\x90R`\0\x91\x82\x91\x90a%\x05a5\xC6V[`\0[`\x02\x81\x10\x15a&\xCAW`\0a%\x1E\x82`\x06aD\x11V[\x90P\x84\x82`\x02\x81\x10a%2Wa%2a@\xABV[` \x02\x01QQ\x83a%D\x83`\0aK\x8BV[`\x0C\x81\x10a%TWa%Ta@\xABV[` \x02\x01R\x84\x82`\x02\x81\x10a%kWa%ka@\xABV[` \x02\x01Q` \x01Q\x83\x82`\x01a%\x82\x91\x90aK\x8BV[`\x0C\x81\x10a%\x92Wa%\x92a@\xABV[` \x02\x01R\x83\x82`\x02\x81\x10a%\xA9Wa%\xA9a@\xABV[` \x02\x01QQQ\x83a%\xBC\x83`\x02aK\x8BV[`\x0C\x81\x10a%\xCCWa%\xCCa@\xABV[` \x02\x01R\x83\x82`\x02\x81\x10a%\xE3Wa%\xE3a@\xABV[` \x02\x01QQ`\x01` \x02\x01Q\x83a%\xFC\x83`\x03aK\x8BV[`\x0C\x81\x10a&\x0CWa&\x0Ca@\xABV[` \x02\x01R\x83\x82`\x02\x81\x10a&#Wa&#a@\xABV[` \x02\x01Q` \x01Q`\0`\x02\x81\x10a&>Wa&>a@\xABV[` \x02\x01Q\x83a&O\x83`\x04aK\x8BV[`\x0C\x81\x10a&_Wa&_a@\xABV[` \x02\x01R\x83\x82`\x02\x81\x10a&vWa&va@\xABV[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a&\x91Wa&\x91a@\xABV[` \x02\x01Q\x83a&\xA2\x83`\x05aK\x8BV[`\x0C\x81\x10a&\xB2Wa&\xB2a@\xABV[` \x02\x01RP\x80a&\xC2\x81aA\xACV[\x91PPa%\x08V[Pa&\xD3a5\xE5V[`\0` \x82a\x01\x80\x85`\x08\x8C\xFA\x91Q\x91\x9C\x91\x15\x15\x9BP\x90\x99PPPPPPPPPPV[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a'\x18WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a'\x9AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x06NV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a'\xDD\x82a!gV[PPV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0EvW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x06NV[`\0[a(\x9D` \x83\x01\x83aK\xA3V[\x90P\x81\x10\x15a)bW`\x9D`\0a(\xB7` \x85\x01\x85aK\xA3V[\x84\x81\x81\x10a(\xC7Wa(\xC7a@\xABV[\x90P` \x02\x01` \x81\x01\x90a(\xDC\x91\x90a6eV[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0\x90\x81 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U`\x9F\x91a)\x0F\x90\x85\x01\x85aK\xA3V[\x84\x81\x81\x10a)\x1FWa)\x1Fa@\xABV[\x90P` \x02\x01` \x81\x01\x90a)4\x91\x90a6eV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x81\x81U`\x01\x01U\x80a)Z\x81aA\xACV[\x91PPa(\x90V[P`\0[a)s`@\x83\x01\x83aK\xECV[\x90P\x81\x10\x15a*\xAEWa)\x89`@\x83\x01\x83aK\xECV[\x82\x81\x81\x10a)\x99Wa)\x99a@\xABV[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a)\xB1\x91\x90aL5V[`\x9D`\0a)\xC2`@\x86\x01\x86aK\xECV[\x85\x81\x81\x10a)\xD2Wa)\xD2a@\xABV[a)\xE8\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa6eV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x90\x81\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x17\x90\x91Ua*(\x90\x83\x01\x83aK\xECV[\x82\x81\x81\x10a*8Wa*8a@\xABV[\x90P`\x80\x02\x01`@\x01`\x9F`\0\x84\x80`@\x01\x90a*U\x91\x90aK\xECV[\x85\x81\x81\x10a*eWa*ea@\xABV[a*{\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa6eV[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a*\xA6\x81aA\xACV[\x91PPa)fV[P`\0[a*\xBF``\x83\x01\x83aLPV[\x90P\x81\x10\x15a+wWa*\xD5``\x83\x01\x83aLPV[\x82\x81\x81\x10a*\xE5Wa*\xE5a@\xABV[\x90P`@\x02\x01` \x01` \x81\x01\x90a*\xFD\x91\x90aL5V[`\x9D`\0a+\x0E``\x86\x01\x86aLPV[\x85\x81\x81\x10a+\x1EWa+\x1Ea@\xABV[a+4\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa6eV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a+o\x81aA\xACV[\x91PPa*\xB2V[P`\0[a+\x88`\x80\x83\x01\x83aL\x99V[\x90P\x81\x10\x15a,$Wa+\x9E`\x80\x83\x01\x83aL\x99V[\x82\x81\x81\x10a+\xAEWa+\xAEa@\xABV[\x90P``\x02\x01` \x01`\x9F`\0\x84\x80`\x80\x01\x90a+\xCB\x91\x90aL\x99V[\x85\x81\x81\x10a+\xDBWa+\xDBa@\xABV[a+\xF1\x92` ``\x90\x92\x02\x01\x90\x81\x01\x91Pa6eV[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a,\x1C\x81aA\xACV[\x91PPa+{V[P`\0[a,5`\xA0\x83\x01\x83aK\xA3V[\x90P\x81\x10\x15a-[W`\0[`\x9B\x80Ta,N\x90a@\xE3V[\x90P\x81\x10\x15a-\x07W`\x9E`\0a,h`\xA0\x86\x01\x86aK\xA3V[\x85\x81\x81\x10a,xWa,xa@\xABV[\x90P` \x02\x015\x81R` \x01\x90\x81R` \x01`\0 `\0`\x9B\x83\x81Ta,\x9D\x90a@\xE3V[\x81\x10a,\xABWa,\xABa@\xABV[\x81T`\x01\x16\x15a,\xCAW\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U\x80a,\xFF\x81aA\xACV[\x91PPa,AV[P`\xA0`\0a-\x18\x84\x83\x01\x85aK\xA3V[\x84\x81\x81\x10a-(Wa-(a@\xABV[` \x90\x81\x02\x92\x90\x92\x015\x83RP\x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\xFF\x19\x16\x90U\x80a-S\x81aA\xACV[\x91PPa,(V[P`\0[a-l`\xC0\x83\x01\x83aK\xA3V[\x90P\x81\x10\x15a/\xBCWa-\x82`\xC0\x83\x01\x83aK\xA3V[\x82\x81\x81\x10a-\x92Wa-\x92a@\xABV[\x90P` \x02\x81\x01\x90a-\xA4\x91\x90aL\xE1V[a-\xB5\x90`\x80\x81\x01\x90``\x01a6eV[`\xA0`\0a-\xC6`\xC0\x86\x01\x86aK\xA3V[\x85\x81\x81\x10a-\xD6Wa-\xD6a@\xABV[\x90P` \x02\x81\x01\x90a-\xE8\x91\x90aL\xE1V[`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP`\0[a.\"`\xC0\x84\x01\x84aK\xA3V[\x83\x81\x81\x10a.2Wa.2a@\xABV[\x90P` \x02\x81\x01\x90a.D\x91\x90aL\xE1V[a.R\x90` \x81\x01\x90aK\xA3V[\x90P\x81\x10\x15a/\xA9Wa.h`\xC0\x84\x01\x84aK\xA3V[\x83\x81\x81\x10a.xWa.xa@\xABV[\x90P` \x02\x81\x01\x90a.\x8A\x91\x90aL\xE1V[a.\x98\x90`@\x81\x01\x90aK\xA3V[\x82\x81\x81\x10a.\xA8Wa.\xA8a@\xABV[\x90P` \x02\x01` \x81\x01\x90a.\xBD\x91\x90aL5V[`\x9E`\0a.\xCE`\xC0\x87\x01\x87aK\xA3V[\x86\x81\x81\x10a.\xDEWa.\xDEa@\xABV[\x90P` \x02\x81\x01\x90a.\xF0\x91\x90aL\xE1V[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a/\x11`\xC0\x87\x01\x87aK\xA3V[\x86\x81\x81\x10a/!Wa/!a@\xABV[\x90P` \x02\x81\x01\x90a/3\x91\x90aL\xE1V[a/A\x90` \x81\x01\x90aK\xA3V[\x85\x81\x81\x10a/QWa/Qa@\xABV[\x90P` \x02\x01` \x81\x01\x90a/f\x91\x90a6eV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a/\xA1\x81aA\xACV[\x91PPa.\x15V[P\x80a/\xB4\x81aA\xACV[\x91PPa-_V[P`\0[a/\xCD`\xE0\x83\x01\x83aK\xA3V[\x90P\x81\x10\x15a1\x80W`\0[a/\xE6`\xE0\x84\x01\x84aK\xA3V[\x83\x81\x81\x10a/\xF6Wa/\xF6a@\xABV[\x90P` \x02\x81\x01\x90a0\x08\x91\x90aM\x01V[a0\x16\x90` \x81\x01\x90aK\xA3V[\x90P\x81\x10\x15a1mWa0,`\xE0\x84\x01\x84aK\xA3V[\x83\x81\x81\x10a0<Wa0<a@\xABV[\x90P` \x02\x81\x01\x90a0N\x91\x90aM\x01V[a0\\\x90`@\x81\x01\x90aK\xA3V[\x82\x81\x81\x10a0lWa0la@\xABV[\x90P` \x02\x01` \x81\x01\x90a0\x81\x91\x90aL5V[`\x9E`\0a0\x92`\xE0\x87\x01\x87aK\xA3V[\x86\x81\x81\x10a0\xA2Wa0\xA2a@\xABV[\x90P` \x02\x81\x01\x90a0\xB4\x91\x90aM\x01V[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a0\xD5`\xE0\x87\x01\x87aK\xA3V[\x86\x81\x81\x10a0\xE5Wa0\xE5a@\xABV[\x90P` \x02\x81\x01\x90a0\xF7\x91\x90aM\x01V[a1\x05\x90` \x81\x01\x90aK\xA3V[\x85\x81\x81\x10a1\x15Wa1\x15a@\xABV[\x90P` \x02\x01` \x81\x01\x90a1*\x91\x90a6eV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a1e\x81aA\xACV[\x91PPa/\xD9V[P\x80a1x\x81aA\xACV[\x91PPa/\xC0V[P`\0[a1\x92a\x01\0\x83\x01\x83aLPV[\x90P\x81\x10\x15a'\xDDWa1\xA9a\x01\0\x83\x01\x83aLPV[\x82\x81\x81\x10a1\xB9Wa1\xB9a@\xABV[\x90P`@\x02\x01` \x01` \x81\x01\x90a1\xD1\x91\x90a6eV[`\xA0`\0a1\xE3a\x01\0\x86\x01\x86aLPV[\x85\x81\x81\x10a1\xF3Wa1\xF3a@\xABV[\x90P`@\x02\x01`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP\x80\x80a2.\x90aA\xACV[\x91PPa1\x84V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x02\0\x82a\xFF\xFF\x16\x10a2\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Roscalar-too-large`\x80\x1B`D\x82\x01R`d\x01a\x06NV[\x81a\xFF\xFF\x16`\x01\x03a2\xA5WP\x81a\x1A\x04V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01\x81\x90R\x84\x90`\x01\x90[\x81a\xFF\xFF\x16\x86a\xFF\xFF\x16\x10a3\x0EW`\x01a\xFF\xFF\x87\x16`\xFF\x83\x16\x1C\x81\x16\x90\x03a2\xF1Wa2\xEE\x84\x84a\"\xEFV[\x93P[a2\xFB\x83\x84a\"\xEFV[\x92Pb\x01\xFF\xFE`\x01\x92\x83\x1B\x16\x91\x01a2\xC1V[P\x91\x95\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q\x15\x80\x15a3>WP` \x82\x01Q\x15[\x15a3\\WPP`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01`\0\x80Q` aM\x18\x839\x81Q\x91R\x84` \x01Qa3\x8F\x91\x90a@\xC1V[a3\xA7\x90`\0\x80Q` aM\x18\x839\x81Q\x91RaDXV[\x90R\x92\x91PPV[\x91\x90PV[`\0\x80\x80`\0\x80Q` aM\x18\x839\x81Q\x91R`\x03`\0\x80Q` aM\x18\x839\x81Q\x91R\x86`\0\x80Q` aM\x18\x839\x81Q\x91R\x88\x89\t\t\x08\x90P`\0a4*\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\0\x80Q` aM\x18\x839\x81Q\x91Ra46V[\x91\x95\x91\x94P\x90\x92PPPV[`\0\x80a4Aa5\xE5V[a4Ia6\x03V[` \x80\x82R\x81\x81\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x88\x90R`\x80\x82\x01\x87\x90R`\xA0\x82\x01\x86\x90R\x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82\x80a4\x86W\xFE[P\x82a4\xD4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FBN254.expMod: call failure\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06NV[PQ\x91PP[\x93\x92PPPV[\x82\x80Ta4\xED\x90a@\xE3V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82a5\x0FW`\0\x85Ua5UV[\x82`\x1F\x10a5(W\x82\x80\x01`\xFF\x19\x825\x16\x17\x85Ua5UV[\x82\x80\x01`\x01\x01\x85U\x82\x15a5UW\x91\x82\x01[\x82\x81\x11\x15a5UW\x825\x82U\x91` \x01\x91\x90`\x01\x01\x90a5:V[Pa5a\x92\x91Pa6!V[P\x90V[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80a5\xB4a66V[\x81R` \x01a5\xC1a66V[\x90R\x90V[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[[\x80\x82\x11\x15a5aW`\0\x81U`\x01\x01a6\"V[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[\x805`\xFF\x81\x16\x81\x14a3\xAFW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a6wW`\0\x80\xFD[a4\xDA\x82a6TV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06`W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a6\xA7W`\0\x80\xFD[\x815a4\xDA\x81a6\x80V[`\0` \x82\x84\x03\x12\x15a6\xC4W`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a7\x03Wa7\x03a6\xCBV[`@R\x90V[`@Qa\x01\0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a7\x03Wa7\x03a6\xCBV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a7TWa7Ta6\xCBV[`@R\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a7nW`\0\x80\xFD[a7va6\xE1V[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a7\x9DW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a7\xBFWa7\xBFa6\xCBV[\x80`@RP\x80`@\x84\x01\x85\x81\x11\x15a7\xD6W`\0\x80\xFD[\x84[\x81\x81\x10\x15a3\x0EW\x805\x83R` \x92\x83\x01\x92\x01a7\xD8V[`\0`\x80\x82\x84\x03\x12\x15a8\x02W`\0\x80\xFD[a8\na6\xE1V[\x90Pa8\x16\x83\x83a7\x8CV[\x81Ra8%\x83`@\x84\x01a7\x8CV[` \x82\x01R\x92\x91PPV[`\0\x80`\0\x80a\x01 \x85\x87\x03\x12\x15a8GW`\0\x80\xFD[\x845\x93Pa8X\x86` \x87\x01a7\\V[\x92Pa8g\x86``\x87\x01a7\xF0V[\x91Pa8v\x86`\xE0\x87\x01a7\\V[\x90P\x92\x95\x91\x94P\x92PV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a8\xAEW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a8\x92V[\x81\x81\x11\x15a8\xC0W`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[\x80\x15\x15\x81\x14a\x06`W`\0\x80\xFD[\x805a3\xAF\x81a8\xD6V[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a9\x07W`\0\x80\xFD[\x855a9\x12\x81a6\x80V[\x94P` \x86\x015a9\"\x81a6\x80V[\x93P`@\x86\x015a92\x81a6\x80V[\x92P``\x86\x015a9B\x81a8\xD6V[\x91P`\x80\x86\x015a9R\x81a6\x80V[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x80`@\x83\x85\x03\x12\x15a9sW`\0\x80\xFD[\x825\x91Pa9\x83` \x84\x01a6TV[\x90P\x92P\x92\x90PV[`\0a\x01\0\x82\x84\x03\x12\x15a9\x9FW`\0\x80\xFD[P\x91\x90PV[`\0a\x01 \x82\x84\x03\x12\x15a9\x9FW`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12a9\xCAW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a9\xE1W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a9\xFCW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a:\x15W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a:,W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x06\x1B\x85\x01\x01\x11\x15a9\xFCW`\0\x80\xFD[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a3\xAFW`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0\x80`\0`\xE0\x8A\x8C\x03\x12\x15a:yW`\0\x80\xFD[\x895`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a:\x90W`\0\x80\xFD[a:\x9C\x8D\x83\x8E\x01a9\x8CV[\x9AP` \x8C\x015\x91P\x80\x82\x11\x15a:\xB2W`\0\x80\xFD[a:\xBE\x8D\x83\x8E\x01a9\xA5V[\x99P`@\x8C\x015\x91P\x80\x82\x11\x15a:\xD4W`\0\x80\xFD[a:\xE0\x8D\x83\x8E\x01a9\xB8V[\x90\x99P\x97P``\x8C\x015\x91P\x80\x82\x11\x15a:\xF9W`\0\x80\xFD[Pa;\x06\x8C\x82\x8D\x01a:\x03V[\x90\x96P\x94Pa;\x19\x90P`\x80\x8B\x01a:GV[\x92Pa;'`\xA0\x8B\x01a:GV[\x91Pa;5`\xC0\x8B\x01a:GV[\x90P\x92\x95\x98P\x92\x95\x98P\x92\x95\x98V[`\0a\x01\x80\x82\x84\x03\x12\x15a9\x9FW`\0\x80\xFD[`\0\x80`\0a\x01@\x84\x86\x03\x12\x15a;mW`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a;\x84W`\0\x80\xFD[a;\x90\x87\x83\x88\x01a9\x8CV[\x94Pa;\x9F\x87` \x88\x01a9\x8CV[\x93Pa\x01 \x86\x015\x91P\x80\x82\x11\x15a;\xB6W`\0\x80\xFD[Pa;\xC3\x86\x82\x87\x01a;DV[\x91PP\x92P\x92P\x92V[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a;\xE6Wa;\xE6a6\xCBV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a<\x01W`\0\x80\xFD[\x815` a<\x16a<\x11\x83a;\xCDV[a7,V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a<5W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a<WWa<J\x81a:GV[\x83R\x91\x83\x01\x91\x83\x01a<9V[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a<sW`\0\x80\xFD[\x815` a<\x83a<\x11\x83a;\xCDV[\x82\x81R`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a<\xA2W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a<WWa<\xB8\x88\x82a7\\V[\x83R\x91\x83\x01\x91`@\x01a<\xA6V[`\0\x82`\x1F\x83\x01\x12a<\xD7W`\0\x80\xFD[\x815` a<\xE7a<\x11\x83a;\xCDV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a=\x06W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a<WW\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a=)W`\0\x80\x81\xFD[a=7\x89\x86\x83\x8B\x01\x01a;\xF0V[\x84RP\x91\x83\x01\x91\x83\x01a=\nV[`\0a\x01\x80\x82\x84\x03\x12\x15a=XW`\0\x80\xFD[a=`a7\tV[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a=yW`\0\x80\xFD[a=\x85\x85\x83\x86\x01a;\xF0V[\x83R` \x84\x015\x91P\x80\x82\x11\x15a=\x9BW`\0\x80\xFD[a=\xA7\x85\x83\x86\x01a<bV[` \x84\x01R`@\x84\x015\x91P\x80\x82\x11\x15a=\xC0W`\0\x80\xFD[a=\xCC\x85\x83\x86\x01a<bV[`@\x84\x01Ra=\xDE\x85``\x86\x01a7\xF0V[``\x84\x01Ra=\xF0\x85`\xE0\x86\x01a7\\V[`\x80\x84\x01Ra\x01 \x84\x015\x91P\x80\x82\x11\x15a>\nW`\0\x80\xFD[a>\x16\x85\x83\x86\x01a;\xF0V[`\xA0\x84\x01Ra\x01@\x84\x015\x91P\x80\x82\x11\x15a>0W`\0\x80\xFD[a><\x85\x83\x86\x01a;\xF0V[`\xC0\x84\x01Ra\x01`\x84\x015\x91P\x80\x82\x11\x15a>VW`\0\x80\xFD[Pa>c\x84\x82\x85\x01a<\xC6V[`\xE0\x83\x01RP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a>\x82W`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a>\x9FW`\0\x80\xFD[a>\xAB\x85\x82\x86\x01a=EV[\x91PP\x92P\x92\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a>\xEEW\x81Q`\x01`\x01``\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a>\xC9V[P\x94\x95\x94PPPPPV[` \x81R`\0\x82Q`@` \x84\x01Ra?\x15``\x84\x01\x82a>\xB5V[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01Ra?2\x82\x82a>\xB5V[\x95\x94PPPPPV[`\0\x80`\0\x80\x84\x86\x03`\xC0\x81\x12\x15a?RW`\0\x80\xFD[\x855`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a?iW`\0\x80\xFD[a?u\x89\x83\x8A\x01a9\x8CV[\x96P```\x1F\x19\x84\x01\x12\x15a?\x89W`\0\x80\xFD[` \x88\x01\x95P`\x80\x88\x015\x92P\x80\x83\x11\x15a?\xA3W`\0\x80\xFD[a?\xAF\x89\x84\x8A\x01a;DV[\x94P`\xA0\x88\x015\x92P\x80\x83\x11\x15a?\xC5W`\0\x80\xFD[PPa?\xD3\x87\x82\x88\x01a9\xA5V[\x91PP\x92\x95\x91\x94P\x92PV[`\0` \x82\x84\x03\x12\x15a?\xF1W`\0\x80\xFD[\x81Qa4\xDA\x81a6\x80V[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15a@XW`\0\x80\xFD[\x81Qa4\xDA\x81a8\xD6V[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x82a@\xDEWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80a@\xF7W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a9\x9FWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15aA)W`\0\x80\xFD[a4\xDA\x82a:GV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aAIW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aAcW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a9\xFCW`\0\x80\xFD[\x82\x81R``\x81\x01a4\xDA` \x83\x01\x84\x805\x82R` \x90\x81\x015\x91\x01RV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01aA\xBEWaA\xBEaA\x96V[P`\x01\x01\x90V[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a3\xAFW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aA\xEEW`\0\x80\xFD[a4\xDA\x82aA\xC5V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aB\x0EW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aB-W`\0\x80\xFD[\x806\x03\x83\x13\x15a9\xFCW`\0\x80\xFD[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x81Rc\xFF\xFF\xFF\xFFaBw\x83a:GV[\x16` \x82\x01R`\x01`\x01`@\x1B\x03aB\x91` \x84\x01aA\xC5V[\x16`@\x82\x01R`\0aB\xA5`@\x84\x01a:GV[c\xFF\xFF\xFF\xFF\x81\x16``\x84\x01RPaB\xBE``\x84\x01a:GV[c\xFF\xFF\xFF\xFF\x81\x16`\x80\x84\x01RPaB\xD7`\x80\x84\x01a:GV[c\xFF\xFF\xFF\xFF\x81\x16`\xA0\x84\x01RPaB\xF0`\xA0\x84\x01a:GV[c\xFF\xFF\xFF\xFF\x81\x16`\xC0\x84\x01RPaC\n`\xC0\x84\x01\x84aA\xF7V[a\x01\0\x80`\xE0\x86\x01RaC\"a\x01 \x86\x01\x83\x85aB<V[\x92PaC0`\xE0\x87\x01a:GV[c\xFF\xFF\xFF\xFF\x16\x94\x01\x93\x90\x93R\x93\x92PPPV[a\x01\0\x81\x01c\xFF\xFF\xFF\xFF\x80aCW\x85a:GV[\x16\x83R` \x84\x015` \x84\x01R`\x01`\x01`@\x1B\x03aCx`@\x86\x01aA\xC5V[\x16`@\x84\x01R\x80aC\x8B``\x86\x01a:GV[\x16``\x84\x01RP`\x80\x83\x015`\x80\x83\x01R`\xA0\x83\x015`\xA0\x83\x01R`\xC0\x83\x015`\xC0\x83\x01R`\xE0\x83\x015aC\xBE\x81a6\x80V[`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x92\x90\x92\x01\x91\x90\x91R\x91\x90PV[`\0a\x1A\x046\x83a=EV[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aD\x08WaD\x08aA\x96V[\x02\x94\x93PPPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15aD+WaD+aA\x96V[P\x02\x90V[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aDOWaDOaA\x96V[\x01\x94\x93PPPPV[`\0\x82\x82\x10\x15aDjWaDjaA\x96V[P\x03\x90V[`\0`\x01`\x01``\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15aD\x8FWaD\x8FaA\x96V[\x03\x93\x92PPPV[` \x81Rc\xFF\xFF\xFF\xFFaD\xA9\x83a:GV[\x16` \x82\x01R`\0aD\xBD` \x84\x01a:GV[c\xFF\xFF\xFF\xFF\x81\x16`@\x84\x01RPaD\xD6`@\x84\x01a:GV[c\xFF\xFF\xFF\xFF\x81\x16``\x84\x01RPaD\xEF``\x84\x01a:GV[c\xFF\xFF\xFF\xFF\x81\x16`\x80\x84\x01RPaE\t`\x80\x84\x01\x84aA\xF7V[a\x01\0\x80`\xA0\x86\x01RaE!a\x01 \x86\x01\x83\x85aB<V[\x92PaE/`\xA0\x87\x01a:GV[c\xFF\xFF\xFF\xFF\x81\x16`\xC0\x87\x01R\x91PaEJ`\xC0\x87\x01\x87aA\xF7V[\x86\x85\x03`\x1F\x19\x01`\xE0\x88\x01R\x92PaEc\x84\x84\x83aB<V[\x93PPaC0`\xE0\x87\x01a:GV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aE\x89W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aE\xA8W`\0\x80\xFD[\x80`\x05\x1B6\x03\x83\x13\x15a9\xFCW`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a>\xEEW`\xFFaE\xDD\x83a6TV[\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aE\xCAV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aF\x07W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aF&W`\0\x80\xFD[\x80`\x07\x1B6\x03\x83\x13\x15a9\xFCW`\0\x80\xFD[\x805`\x01`\x01``\x1B\x03\x81\x16\x81\x14a3\xAFW`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a>\xEEW`\xFFaFr\x83a6TV[\x16\x87R`\x01`\x01``\x1B\x03aF\x88\x84\x84\x01aF8V[\x16\x83\x88\x01R`@\x82\x81\x015\x90\x88\x01R``\x80\x83\x015\x90\x88\x01R`\x80\x96\x87\x01\x96\x90\x91\x01\x90`\x01\x01aF_V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aF\xCAW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aF\xE9W`\0\x80\xFD[\x80`\x06\x1B6\x03\x83\x13\x15a9\xFCW`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a>\xEEW`\xFFaG\x1E\x83a6TV[\x16\x87R`\x01`\x01``\x1B\x03aG4\x84\x84\x01aF8V[\x16\x87\x84\x01R`@\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aG\x0BV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aGcW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aG\x82W`\0\x80\xFD[``\x81\x026\x03\x83\x13\x15a9\xFCW`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a>\xEEW`\xFFaG\xB7\x83a6TV[\x16\x87RaG\xD2\x83\x88\x01\x84\x84\x01\x805\x82R` \x90\x81\x015\x91\x01RV[``\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aG\xA4V[\x81\x83R`\0`\x01`\x01`\xFB\x1B\x03\x83\x11\x15aG\xFEW`\0\x80\xFD[\x82`\x05\x1B\x80\x83` \x87\x017`\0\x94\x01` \x01\x93\x84RP\x91\x92\x91PPV[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a>\xEEW`\x01`\x01``\x1B\x03aHD\x83aF8V[\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aH+V[\x81\x83R`\0` \x80\x85\x01\x94P\x84\x84`\x05\x1B\x86\x01\x84`\0\x80[\x88\x81\x10\x15aI\x05W\x84\x84\x03\x8AR\x825`~\x19\x896\x03\x01\x81\x12aH\x8FW\x82\x83\xFD[\x88\x01\x805\x85R`\x80aH\xA3\x88\x83\x01\x83aErV[\x82\x8A\x89\x01RaH\xB5\x83\x89\x01\x82\x84aE\xBAV[\x92PPP`@aH\xC7\x81\x84\x01\x84aErV[\x88\x84\x03\x83\x8A\x01RaH\xD9\x84\x82\x84aH\x1BV[\x93PPPP```\xFFaH\xED\x82\x85\x01a6TV[\x16\x96\x01\x95\x90\x95RP\x98\x85\x01\x98\x91\x85\x01\x91`\x01\x01aHoV[P\x91\x98\x97PPPPPPPPV[`\0\x83\x83\x85R` \x80\x86\x01\x95P\x80\x85`\x05\x1B\x83\x01\x01\x84`\0\x80[\x88\x81\x10\x15aI\x05W\x85\x84\x03`\x1F\x19\x01\x8AR\x8256\x89\x90\x03`^\x19\x01\x81\x12aIRW\x82\x83\xFD[\x88\x01\x805\x85R``aIf\x87\x83\x01\x83aErV[\x82\x89\x89\x01RaIx\x83\x89\x01\x82\x84aE\xBAV[\x92PPP`@aI\x8A\x81\x84\x01\x84aErV[\x93P\x87\x83\x03\x82\x89\x01RaI\x9E\x83\x85\x83aH\x1BV[\x9D\x89\x01\x9D\x97PPP\x93\x86\x01\x93PP`\x01\x01aI-V[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a>\xEEW\x815\x87R`\xFFaI\xDD\x84\x84\x01a6TV[\x16\x87\x84\x01R`@\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aI\xC4V[` \x81RaJ\x0F` \x82\x01aJ\t\x84a8\xE4V[\x15\x15\x90RV[`\0aJ\x1E` \x84\x01\x84aErV[a\x01 \x80`@\x86\x01RaJ6a\x01@\x86\x01\x83\x85aE\xBAV[\x92PaJE`@\x87\x01\x87aE\xF0V[\x92P`\x1F\x19\x80\x87\x86\x03\x01``\x88\x01RaJ_\x85\x85\x84aFOV[\x94PaJn``\x89\x01\x89aF\xB3V[\x94P\x91P\x80\x87\x86\x03\x01`\x80\x88\x01RaJ\x87\x85\x85\x84aF\xFBV[\x94PaJ\x96`\x80\x89\x01\x89aGLV[\x94P\x91P\x80\x87\x86\x03\x01`\xA0\x88\x01RaJ\xAF\x85\x85\x84aG\x94V[\x94PaJ\xBE`\xA0\x89\x01\x89aErV[\x94P\x91P\x80\x87\x86\x03\x01`\xC0\x88\x01RaJ\xD7\x85\x85\x84aG\xE5V[\x94PaJ\xE6`\xC0\x89\x01\x89aErV[\x94P\x91P\x80\x87\x86\x03\x01`\xE0\x88\x01RaJ\xFF\x85\x85\x84aHWV[\x94PaK\x0E`\xE0\x89\x01\x89aErV[\x94P\x91Pa\x01\0\x81\x88\x87\x03\x01\x81\x89\x01RaK)\x86\x86\x85aI\x13V[\x95PaK7\x81\x8A\x01\x8AaF\xB3V[\x95P\x92PP\x80\x87\x86\x03\x01\x83\x88\x01RPaKQ\x84\x84\x83aI\xB4V[\x97\x96PPPPPPPV[``\x81\x01c\xFF\xFF\xFF\xFFaKn\x84a:GV[\x16\x82R` \x83\x015` \x83\x01R`@\x83\x015`@\x83\x01R\x92\x91PPV[`\0\x82\x19\x82\x11\x15aK\x9EWaK\x9EaA\x96V[P\x01\x90V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aK\xBAW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aK\xD4W`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a9\xFCW`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aL\x03W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aL\x1DW`\0\x80\xFD[` \x01\x91P`\x07\x81\x90\x1B6\x03\x82\x13\x15a9\xFCW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aLGW`\0\x80\xFD[a4\xDA\x82aF8V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aLgW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aL\x81W`\0\x80\xFD[` \x01\x91P`\x06\x81\x90\x1B6\x03\x82\x13\x15a9\xFCW`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aL\xB0W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aL\xCAW`\0\x80\xFD[` \x01\x91P``\x81\x026\x03\x82\x13\x15a9\xFCW`\0\x80\xFD[`\0\x825`~\x19\x836\x03\x01\x81\x12aL\xF7W`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x825`^\x19\x836\x03\x01\x81\x12aL\xF7W`\0\x80\xFD\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 E\xBF\xCB\xF7\xCD\x88\xAB+\x9E}y\x84\x955\x1F\x9E\xEE\xFDQ}V\xE5NSB\x85yT;\x02L;dsolcC\0\x08\r\x003";
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
        ///Calls the contract's `chainId` (0x9a8a0592) function
        pub fn chain_id(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([154, 138, 5, 146], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `chainRdBatchNonce` (0xdeb4037d) function
        pub fn chain_rd_batch_nonce(&self) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([222, 180, 3, 125], ())
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
        ///Calls the contract's `initialize` (0x390f7570) function
        pub fn initialize(
            &self,
            pauser_registry: ::ethers::core::types::Address,
            initial_owner: ::ethers::core::types::Address,
            updater: ::ethers::core::types::Address,
            allow_non_root_init: bool,
            rolldown: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [57, 15, 117, 112],
                    (
                        pauser_registry,
                        initial_owner,
                        updater,
                        allow_non_root_init,
                        rolldown,
                    ),
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
        ///Calls the contract's `latestCompletedRdTaskCreatedBlock` (0x0bf16410) function
        pub fn latest_completed_rd_task_created_block(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([11, 241, 100, 16], ())
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
        ///Calls the contract's `processEigenOpUpdate` (0xd30270ef) function
        pub fn process_eigen_op_update(
            &self,
            task: OpTask,
            task_response: OpTaskResponse,
            non_signer_stakes_and_signature: NonSignerStakesAndSignature,
            operator_state_info: OperatorStateInfo,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [211, 2, 112, 239],
                    (
                        task,
                        task_response,
                        non_signer_stakes_and_signature,
                        operator_state_info,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `processEigenRdUpdate` (0x738491d9) function
        pub fn process_eigen_rd_update(
            &self,
            task: RdTask,
            task_response: RdTaskResponse,
            non_signer_stakes_and_signature: NonSignerStakesAndSignature,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [115, 132, 145, 217],
                    (task, task_response, non_signer_stakes_and_signature),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `processEigenReinit` (0x56b933d9) function
        pub fn process_eigen_reinit(
            &self,
            task: OpTask,
            operator_state_info: OperatorStateInfo,
            merkle_roots: ::std::vec::Vec<[u8; 32]>,
            ranges: ::std::vec::Vec<Range>,
            chain_rd_batch_nonce: u32,
            latest_completed_rd_task_number: u32,
            latest_completed_rd_task_created_block: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [86, 185, 51, 217],
                    (
                        task,
                        operator_state_info,
                        merkle_roots,
                        ranges,
                        chain_rd_batch_nonce,
                        latest_completed_rd_task_number,
                        latest_completed_rd_task_created_block,
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
        ///Calls the contract's `rolldown` (0x3d9fb00c) function
        pub fn rolldown(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
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
        ///Calls the contract's `setRolldown` (0xfdc15de8) function
        pub fn set_rolldown(
            &self,
            rolldown: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([253, 193, 93, 232], rolldown)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setUpdater` (0x9d54f419) function
        pub fn set_updater(
            &self,
            updater: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([157, 84, 244, 25], updater)
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
        ///Gets the contract's `RolldownTargetUpdated` event
        pub fn rolldown_target_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RolldownTargetUpdatedFilter>
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
    #[ethevent(name = "RolldownTargetUpdated", abi = "RolldownTargetUpdated(address)")]
    pub struct RolldownTargetUpdatedFilter {
        pub rolldown_address: ::ethers::core::types::Address,
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
        RolldownTargetUpdatedFilter(RolldownTargetUpdatedFilter),
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
            if let Ok(decoded) = RolldownTargetUpdatedFilter::decode_log(log) {
                return Ok(GaspMultiRollupServiceEvents::RolldownTargetUpdatedFilter(
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
                Self::RolldownTargetUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<RolldownTargetUpdatedFilter> for GaspMultiRollupServiceEvents {
        fn from(value: RolldownTargetUpdatedFilter) -> Self {
            Self::RolldownTargetUpdatedFilter(value)
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
    ///Container type for all input parameters for the `chainId` function with signature `chainId()` and selector `0x9a8a0592`
    #[derive(
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
    #[ethcall(name = "chainId", abi = "chainId()")]
    pub struct ChainIdCall;
    ///Container type for all input parameters for the `chainRdBatchNonce` function with signature `chainRdBatchNonce()` and selector `0xdeb4037d`
    #[derive(
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
    #[ethcall(name = "chainRdBatchNonce", abi = "chainRdBatchNonce()")]
    pub struct ChainRdBatchNonceCall;
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
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address,address,bool,address)` and selector `0x390f7570`
    #[derive(
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
        name = "initialize",
        abi = "initialize(address,address,address,bool,address)"
    )]
    pub struct InitializeCall {
        pub pauser_registry: ::ethers::core::types::Address,
        pub initial_owner: ::ethers::core::types::Address,
        pub updater: ::ethers::core::types::Address,
        pub allow_non_root_init: bool,
        pub rolldown: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `latestCompletedRdTaskCreatedBlock` function with signature `latestCompletedRdTaskCreatedBlock()` and selector `0x0bf16410`
    #[derive(
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
        name = "latestCompletedRdTaskCreatedBlock",
        abi = "latestCompletedRdTaskCreatedBlock()"
    )]
    pub struct LatestCompletedRdTaskCreatedBlockCall;
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
    ///Container type for all input parameters for the `processEigenOpUpdate` function with signature `processEigenOpUpdate((uint32,uint32,uint32,uint32,bytes,uint32,bytes,uint32),(uint32,bytes32,bytes32),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]),(bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]))` and selector `0xd30270ef`
    #[derive(
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
        name = "processEigenOpUpdate",
        abi = "processEigenOpUpdate((uint32,uint32,uint32,uint32,bytes,uint32,bytes,uint32),(uint32,bytes32,bytes32),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]),(bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]))"
    )]
    pub struct ProcessEigenOpUpdateCall {
        pub task: OpTask,
        pub task_response: OpTaskResponse,
        pub non_signer_stakes_and_signature: NonSignerStakesAndSignature,
        pub operator_state_info: OperatorStateInfo,
    }
    ///Container type for all input parameters for the `processEigenRdUpdate` function with signature `processEigenRdUpdate((uint32,uint64,uint32,uint32,uint32,uint32,bytes,uint32),(uint32,bytes32,uint64,uint32,bytes32,uint256,uint256,address),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))` and selector `0x738491d9`
    #[derive(
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
        name = "processEigenRdUpdate",
        abi = "processEigenRdUpdate((uint32,uint64,uint32,uint32,uint32,uint32,bytes,uint32),(uint32,bytes32,uint64,uint32,bytes32,uint256,uint256,address),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))"
    )]
    pub struct ProcessEigenRdUpdateCall {
        pub task: RdTask,
        pub task_response: RdTaskResponse,
        pub non_signer_stakes_and_signature: NonSignerStakesAndSignature,
    }
    ///Container type for all input parameters for the `processEigenReinit` function with signature `processEigenReinit((uint32,uint32,uint32,uint32,bytes,uint32,bytes,uint32),(bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]),bytes32[],(uint256,uint256)[],uint32,uint32,uint32)` and selector `0x56b933d9`
    #[derive(
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
        name = "processEigenReinit",
        abi = "processEigenReinit((uint32,uint32,uint32,uint32,bytes,uint32,bytes,uint32),(bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]),bytes32[],(uint256,uint256)[],uint32,uint32,uint32)"
    )]
    pub struct ProcessEigenReinitCall {
        pub task: OpTask,
        pub operator_state_info: OperatorStateInfo,
        pub merkle_roots: ::std::vec::Vec<[u8; 32]>,
        pub ranges: ::std::vec::Vec<Range>,
        pub chain_rd_batch_nonce: u32,
        pub latest_completed_rd_task_number: u32,
        pub latest_completed_rd_task_created_block: u32,
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
        Hash,
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
        Hash,
    )]
    #[ethcall(name = "setPauserRegistry", abi = "setPauserRegistry(address)")]
    pub struct SetPauserRegistryCall {
        pub new_pauser_registry: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setRolldown` function with signature `setRolldown(address)` and selector `0xfdc15de8`
    #[derive(
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
    #[ethcall(name = "setRolldown", abi = "setRolldown(address)")]
    pub struct SetRolldownCall {
        pub rolldown: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setUpdater` function with signature `setUpdater(address)` and selector `0x9d54f419`
    #[derive(
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
    #[ethcall(name = "setUpdater", abi = "setUpdater(address)")]
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
        ChainId(ChainIdCall),
        ChainRdBatchNonce(ChainRdBatchNonceCall),
        CheckSignatures(CheckSignaturesCall),
        Initialize(InitializeCall),
        LastOpUpdateBlockTimestamp(LastOpUpdateBlockTimestampCall),
        LatestCompletedOpTaskCreatedBlock(LatestCompletedOpTaskCreatedBlockCall),
        LatestCompletedOpTaskNumber(LatestCompletedOpTaskNumberCall),
        LatestCompletedRdTaskCreatedBlock(LatestCompletedRdTaskCreatedBlockCall),
        LatestCompletedRdTaskNumber(LatestCompletedRdTaskNumberCall),
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
            if let Ok(decoded) =
                <AllowNonRootInitCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AllowNonRootInit(decoded));
            }
            if let Ok(decoded) = <ChainIdCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ChainId(decoded));
            }
            if let Ok(decoded) =
                <ChainRdBatchNonceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ChainRdBatchNonce(decoded));
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
                <LatestCompletedRdTaskCreatedBlockCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::LatestCompletedRdTaskCreatedBlock(decoded));
            }
            if let Ok(decoded) =
                <LatestCompletedRdTaskNumberCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LatestCompletedRdTaskNumber(decoded));
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
            if let Ok(decoded) = <RolldownCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Rolldown(decoded));
            }
            if let Ok(decoded) =
                <SetPauserRegistryCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetPauserRegistry(decoded));
            }
            if let Ok(decoded) = <SetRolldownCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetRolldown(decoded));
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
                Self::ChainId(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ChainRdBatchNonce(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::LatestCompletedRdTaskCreatedBlock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LatestCompletedRdTaskNumber(element) => {
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
                Self::Rolldown(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetPauserRegistry(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetRolldown(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::ChainId(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChainRdBatchNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckSignatures(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::LastOpUpdateBlockTimestamp(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestCompletedOpTaskCreatedBlock(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LatestCompletedOpTaskNumber(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestCompletedRdTaskCreatedBlock(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LatestCompletedRdTaskNumber(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<AllowNonRootInitCall> for GaspMultiRollupServiceCalls {
        fn from(value: AllowNonRootInitCall) -> Self {
            Self::AllowNonRootInit(value)
        }
    }
    impl ::core::convert::From<ChainIdCall> for GaspMultiRollupServiceCalls {
        fn from(value: ChainIdCall) -> Self {
            Self::ChainId(value)
        }
    }
    impl ::core::convert::From<ChainRdBatchNonceCall> for GaspMultiRollupServiceCalls {
        fn from(value: ChainRdBatchNonceCall) -> Self {
            Self::ChainRdBatchNonce(value)
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
    impl ::core::convert::From<LatestCompletedRdTaskCreatedBlockCall> for GaspMultiRollupServiceCalls {
        fn from(value: LatestCompletedRdTaskCreatedBlockCall) -> Self {
            Self::LatestCompletedRdTaskCreatedBlock(value)
        }
    }
    impl ::core::convert::From<LatestCompletedRdTaskNumberCall> for GaspMultiRollupServiceCalls {
        fn from(value: LatestCompletedRdTaskNumberCall) -> Self {
            Self::LatestCompletedRdTaskNumber(value)
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
    ///Container type for all return fields from the `chainId` function with signature `chainId()` and selector `0x9a8a0592`
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
    pub struct ChainIdReturn(pub u8);
    ///Container type for all return fields from the `chainRdBatchNonce` function with signature `chainRdBatchNonce()` and selector `0xdeb4037d`
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
    pub struct ChainRdBatchNonceReturn(pub u32);
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
    ///Container type for all return fields from the `latestCompletedRdTaskCreatedBlock` function with signature `latestCompletedRdTaskCreatedBlock()` and selector `0x0bf16410`
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
    pub struct LatestCompletedRdTaskCreatedBlockReturn(pub u32);
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
        Hash,
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
    ///`Range(uint256,uint256)`
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
    pub struct Range {
        pub start: ::ethers::core::types::U256,
        pub end: ::ethers::core::types::U256,
    }
}
