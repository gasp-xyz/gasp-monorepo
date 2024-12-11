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
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum IRolldownPrimitives.ChainId",
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_chainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum IRolldownPrimitives.ChainId",
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
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
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
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
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
                                    name: ::std::borrow::ToOwned::to_owned("lastBatchId"),
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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[PaMc\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\x06W`\x005`\xE0\x1C\x80co\x0C0\xA4\x11a\x01\x1AW\x80c\xD0:\x07\xB2\x11a\0\xADW\x80c\xED9\xE5\x02\x11a\0|W\x80c\xED9\xE5\x02\x14a\x05DW\x80c\xF2\xFD\xE3\x8B\x14a\x05WW\x80c\xF8N\x91\xFC\x14a\x05jW\x80c\xFA\xBC\x1C\xBC\x14a\x05sW\x80c\xFD\xC1]\xE8\x14a\x05\x86W`\0\x80\xFD[\x80c\xD0:\x07\xB2\x14a\x04\xF2W\x80c\xDE\xB4\x03}\x14a\x05\x02W\x80c\xDF\x03L\xD0\x14a\x05\x19W\x80c\xE2\xA7\xCBf\x14a\x05,W`\0\x80\xFD[\x80c\x88o\x11\x95\x11a\0\xE9W\x80c\x88o\x11\x95\x14a\x04\x9AW\x80c\x8D\xA5\xCB[\x14a\x04\xADW\x80c\x9A\x8A\x05\x92\x14a\x04\xBEW\x80c\x9DT\xF4\x19\x14a\x04\xDFW`\0\x80\xFD[\x80co\x0C0\xA4\x14a\x042W\x80cqP\x18\xA6\x14a\x04IW\x80cz\xD7Ua\x14a\x04QW\x80c}\x97\x88\x97\x14a\x04zW`\0\x80\xFD[\x80c4\xFA\xDB\xEA\x11a\x01\x9DW\x80cM\xEA\xBC!\x11a\x01lW\x80cM\xEA\xBC!\x14a\x03\xBCW\x80cRn>d\x14a\x03\xE1W\x80cY\\jg\x14a\x03\xF5W\x80cZ\xC8j\xB7\x14a\x03\xFDW\x80c\\\x97Z\xBB\x14a\x04 W`\0\x80\xFD[\x80c4\xFA\xDB\xEA\x14a\x02\xFDW\x80c=\x9F\xB0\x0C\x14a\x03\x10W\x80cC\r;9\x14a\x03;W\x80cI\x9Do\xB6\x14a\x03pW`\0\x80\xFD[\x80c\x17\x1F\x1D[\x11a\x01\xD9W\x80c\x17\x1F\x1D[\x14a\x02\x98W\x80c&5\xE7N\x14a\x02\xC2W\x80c*\x84\x14\xFD\x14a\x02\xD5W\x80c0\xC4}\x8E\x14a\x02\xEAW`\0\x80\xFD[\x80c\x03\xD0\x97\xD2\x14a\x02\x0BW\x80c\x0E\xE0\xFD\xBD\x14a\x02LW\x80c\x10\xD6z/\x14a\x02pW\x80c\x13d9\xDD\x14a\x02\x85W[`\0\x80\xFD[a\x022a\x02\x196`\x04a6}V[`\x9F` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T\x82V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[`\x98Ta\x02`\x90`\x01`\xA8\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02CV[a\x02\x83a\x02~6`\x04a6\xADV[a\x05\x99V[\0[a\x02\x83a\x02\x936`\x04a6\xCAV[a\x06UV[a\x02\xABa\x02\xA66`\x04a8HV[a\x07\x94V[`@\x80Q\x92\x15\x15\x83R\x90\x15\x15` \x83\x01R\x01a\x02CV[a\x02\x83a\x02\xD06`\x04a9#V[a\t\x1EV[a\x02\xDDa\x0ByV[`@Qa\x02C\x91\x90a:\x1DV[a\x02\x83a\x02\xF86`\x04a:\x9AV[a\x0C\x07V[a\x02\x83a\x03\x0B6`\x04a;+V[a\r\xAFV[`\x97Ta\x03#\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02CV[a\x03^a\x03I6`\x04a6\xCAV[`\xA0` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02CV[a\x03\xA4a\x03~6`\x04a;\xCFV[`\x9E` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\x01`\x01``\x1B\x03\x16\x81V[`@Q`\x01`\x01``\x1B\x03\x90\x91\x16\x81R` \x01a\x02CV[`\x9CTa\x03\xCC\x90c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02CV[`\x98Ta\x02`\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[a\x02\x83a\x12\xB4V[a\x02`a\x04\x0B6`\x04a6}V[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`fT[`@Q\x90\x81R` \x01a\x02CV[`\x9ATa\x03\xCC\x90`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x02\x83a\x13{V[a\x03\xA4a\x04_6`\x04a6}V[`\x9D` R`\0\x90\x81R`@\x90 T`\x01`\x01``\x1B\x03\x16\x81V[a\x04\x8Da\x04\x886`\x04a>\x9DV[a\x13\x8FV[`@Qa\x02C\x91\x90a?'V[`eTa\x03#\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x03#V[`\x97Ta\x04\xD2\x90`\x01`\xC0\x1B\x90\x04`\xFF\x16\x81V[`@Qa\x02C\x91\x90a?\xA1V[a\x02\x83a\x04\xED6`\x04a6\xADV[a\x19yV[`\x9ATa\x03\xCC\x90c\xFF\xFF\xFF\xFF\x16\x81V[`\x97Ta\x03\xCC\x90`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`\x98Ta\x03#\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x9ATa\x03\xCC\x90d\x01\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x02\x83a\x05R6`\x04a?\xAFV[a\x19\xA3V[a\x02\x83a\x05e6`\x04a6\xADV[a\x1FQV[a\x04$`\x99T\x81V[a\x02\x83a\x05\x816`\x04a6\xCAV[a\x1F\xC7V[a\x02\x83a\x05\x946`\x04a6\xADV[a!#V[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xECW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x10\x91\x90a@1V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x06IW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06@\x90a@NV[`@Q\x80\x91\x03\x90\xFD[a\x06R\x81a!\x7FV[PV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x9DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xC1\x91\x90a@\x98V[a\x06\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06@\x90a@\xB5V[`fT\x81\x81\x16\x14a\x07VW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06@V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`\0\x80`\0\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x87\x87`\0\x01Q\x88` \x01Q\x88`\0\x01Q`\0`\x02\x81\x10a\x07\xDCWa\x07\xDCa@\xFDV[` \x02\x01Q\x89Q`\x01` \x02\x01Q\x8A` \x01Q`\0`\x02\x81\x10a\x08\x01Wa\x08\x01a@\xFDV[` \x02\x01Q\x8B` \x01Q`\x01`\x02\x81\x10a\x08\x1DWa\x08\x1Da@\xFDV[` \x90\x81\x02\x91\x90\x91\x01Q\x8CQ\x8D\x83\x01Q`@Qa\x08z\x9A\x99\x98\x97\x96\x95\x94\x01\x98\x89R` \x89\x01\x97\x90\x97R`@\x88\x01\x95\x90\x95R``\x87\x01\x93\x90\x93R`\x80\x86\x01\x91\x90\x91R`\xA0\x85\x01R`\xC0\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01Ra\x01 \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\0\x1Ca\x08\x9D\x91\x90aA\x13V[\x90Pa\t\x10a\x08\xB6a\x08\xAF\x88\x84a\"vV[\x86\x90a#\x07V[a\x08\xBEa#\x9CV[a\t\x06a\x08\xF7\x85a\x08\xF1`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x01\x82R`\x02\x90\x82\x01R\x90V[\x90a\"vV[a\t\0\x8Ca$\\V[\x90a#\x07V[\x88b\x01\xD4\xC0a$\xEBV[\x90\x98\x90\x97P\x95PPPPPPV[a\t&a'\x0FV[\x83\x82\x14a\tuW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FrdUpdate info length mismatch\0\0\0`D\x82\x01R`d\x01a\x06@V[a\t~\x86a'iV[a\t\x8B` \x88\x01\x88aA5V[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90Ua\t\xC2`@\x88\x01` \x89\x01aA5V[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x01`@\x1B\x02c\xFF\xFF\xFF\xFF`@\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90UB`\x99Ua\t\xF8``\x88\x01\x88aAPV[a\n\x04\x91`\x9B\x91a4\xF9V[Pa\n\x15`\xA0\x88\x01`\x80\x89\x01aA5V[`\x9C\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U`\0[\x84\x81\x10\x15a\n\xDEW`\x97T`\x01`\x01`\xA0\x1B\x03\x16c\x08\xF4-@\x87\x87\x84\x81\x81\x10a\n]Wa\n]a@\xFDV[\x90P` \x02\x015\x86\x86\x85\x81\x81\x10a\nvWa\nva@\xFDV[\x90P`@\x02\x01`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n\x99\x92\x91\x90aA\x96V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\n\xB3W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\n\xC7W=`\0\x80>=`\0\xFD[PPPP\x80\x80a\n\xD6\x90aA\xCAV[\x91PPa\n2V[P\x83\x15a\x0B\x11Wa\n\xF0\x81`\x01aA\xE3V[`\x97`\x14a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP[\x7F&Ie\xEBk\xC46\xC6\xC4sC\x1D4\xAFV\xE82\xEC4O\xDF\xD4>\xE6\xAFo\xCEm ^\x84\xAFa\x0B?` \x89\x01\x89aA5V[a\x0BO`@\x8A\x01` \x8B\x01aA5V[`@\x80Qc\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01[`@Q\x80\x91\x03\x90\xA1PPPPPPPV[`\x9B\x80Ta\x0B\x86\x90aB\x0BV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B\xB2\x90aB\x0BV[\x80\x15a\x0B\xFFW\x80`\x1F\x10a\x0B\xD4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B\xFFV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\xE2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x0C'WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x0CAWP0;\x15\x80\x15a\x0CAWP`\0T`\xFF\x16`\x01\x14[a\x0C\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x06@V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x0C\xC7W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x0C\xD2\x87`\0a1\x16V[a\x0C\xDB\x86a1\xFCV[`\x98\x80T\x85\x15\x15`\x01`\xA8\x1B\x02`\x01a\xFF\x01`\xA0\x1B\x03\x19\x90\x91\x16`\x01`\x01`\xA0\x1B\x03\x80\x89\x16\x91\x90\x91\x17\x91\x90\x91\x17\x90\x91U`\x97\x80T\x91\x85\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x82U\x84\x92`\x01d\xFF\0\0\0\x01`\xA0\x1B\x03\x19\x16\x17`\x01`\xC0\x1B\x83`\x02\x81\x11\x15a\rIWa\rIa?iV[\x02\x17\x90UP`\x97\x80Tc\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x17\x90U\x80\x15a\r\xA6W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01a\x0BhV[PPPPPPPV[`fT\x15a\r\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FPausable: contract is paused\0\0\0\0`D\x82\x01R`d\x01a\x06@V[`\x98T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0EYW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FUpdater must be the caller\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06@V[`\x9AT`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x80\x15\x15\x80a\x0E\x80WP`\x98T`\x01`\xA8\x1B\x90\x04`\xFF\x16[\x15a\x0E\xCCW`\x98T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0E\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04\x17WF\x83`\xDC\x1B`D\x82\x01R`d\x01a\x06@V[a\x0F\x0EV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0F\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdAuth1`\xD8\x1B`D\x82\x01R`d\x01a\x06@V[\x84`@Q` \x01a\x0F\x1F\x91\x90aB\xADV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x015\x14a\x0F\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FreferenceTaskHash hash mismatch\0`D\x82\x01R`d\x01a\x06@V[\x81`@Q` \x01a\x0F\x98\x91\x90aG\xF2V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84`@\x015\x14a\x10\0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FoperatorStateInfo hash mismatch\0`D\x82\x01R`d\x01a\x06@V[c\xFF\xFF\xFF\xFF\x81\x16\x15a\x11\x95Wa\x10\x1C``\x86\x01`@\x87\x01aA5V[c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x14a\x10vW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7Freference block mismatch\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06@V[`\0a\x10\xAC\x85`@Q` \x01a\x10\x8C\x91\x90aIYV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x85a\x04\x88\x90aI\x88V[`\x9CT\x90\x91Pc\xFF\xFF\xFF\xFF\x16`\0[`\x9B\x80Ta\x10\xC8\x90aB\x0BV[\x90P\x81\x10\x15a\x11\x91W\x81`\xFF\x16\x83` \x01Q\x82\x81Q\x81\x10a\x10\xEBWa\x10\xEBa@\xFDV[` \x02` \x01\x01Qa\x10\xFD\x91\x90aI\x94V[`\x01`\x01``\x1B\x03\x16`d\x84`\0\x01Q\x83\x81Q\x81\x10a\x11\x1EWa\x11\x1Ea@\xFDV[` \x02` \x01\x01Q`\x01`\x01``\x1B\x03\x16a\x119\x91\x90aI\xC3V[\x10\x15a\x11\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtFailed to meet quorum`X\x1B`D\x82\x01R`d\x01a\x06@V[\x80a\x11\x89\x81aA\xCAV[\x91PPa\x10\xBBV[PPP[a\x11\x9E\x82a'iV[a\x11\xAB` \x86\x01\x86aA5V[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90Ua\x11\xE2`@\x86\x01` \x87\x01aA5V[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x01`@\x1B\x02c\xFF\xFF\xFF\xFF`@\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90UB`\x99Ua\x12\x18``\x86\x01\x86aAPV[a\x12$\x91`\x9B\x91a4\xF9V[Pa\x125`\xA0\x86\x01`\x80\x87\x01aA5V[`\x9C\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x7F6\xA1\xFD{\xD5T\xF5\xC4(\xC9\x82\x9C\t\xC6`kL\x89;\x1F\xAD\xC8sZz\x12yW\x97D}\xEDa\x12}` \x87\x01\x87aA5V[a\x12\x8D`@\x88\x01` \x89\x01aA5V[`@\x80Qc\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1PPPPPV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xFCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13 \x91\x90a@\x98V[a\x13<W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06@\x90a@\xB5V[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[a\x13\x83a'\x0FV[a\x13\x8D`\0a1\xFCV[V[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01\x81\x90R`\x9B\x80Ta\x13\xC6\x90aB\x0BV[\x90P\x90Pa\x13\xE7`@Q\x80`@\x01`@R\x80``\x81R` \x01``\x81RP\x90V[\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x13\xFFWa\x13\xFFa6\xE3V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x14(W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P` \x82\x01R\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14FWa\x14Fa6\xE3V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x14oW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x81R` \x85\x01QQ`\0\x90`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14\x92Wa\x14\x92a6\xE3V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x14\xBBW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80[\x87` \x01QQ\x81\x10\x15a\x16mWa\x15\x07\x88` \x01Q\x82\x81Q\x81\x10a\x14\xE8Wa\x14\xE8a@\xFDV[` \x02` \x01\x01Q\x80Q`\0\x90\x81R` \x91\x82\x01Q\x90\x91R`@\x90 \x90V[\x83\x82\x81Q\x81\x10a\x15\x19Wa\x15\x19a@\xFDV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80\x15a\x15\xE3W\x82a\x156`\x01\x83aI\xE2V[\x81Q\x81\x10a\x15FWa\x15Fa@\xFDV[` \x02` \x01\x01Q`\0\x1C\x83\x82\x81Q\x81\x10a\x15cWa\x15ca@\xFDV[` \x02` \x01\x01Q`\0\x1C\x11a\x15\xE3W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: nonSignerPubkeys not sorted`d\x82\x01R`\x84\x01a\x06@V[a\x16Ya\x16R`\xA0`\0\x86\x85\x81Q\x81\x10a\x15\xFFWa\x15\xFFa@\xFDV[` \x02` \x01\x01Q\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\xFF\x16\x8A` \x01Q\x84\x81Q\x81\x10a\x16<Wa\x16<a@\xFDV[` \x02` \x01\x01Qa2N\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x87\x90a#\x07V[\x95P\x80a\x16e\x81aA\xCAV[\x91PPa\x14\xC2V[Pa\x16w\x85a31V[\x94P`\0[\x84\x81\x10\x15a\x18[W`\x9B\x81\x81Ta\x16\x92\x90aB\x0BV[\x81\x10a\x16\xA0Wa\x16\xA0a@\xFDV[\x81T`\x01\x16\x15a\x16\xBFW\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C`\0\x81\x81R`\x9F` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x01T\x90\x82\x01R\x90\x92Pa\x17\0\x90\x87\x90a#\x07V[`\xFF\x83\x16`\0\x90\x81R`\x9D` \x90\x81R`@\x90\x91 T\x90\x86\x01Q\x80Q\x92\x98P`\x01`\x01``\x1B\x03\x90\x91\x16\x91\x83\x90\x81\x10a\x17;Wa\x17;a@\xFDV[`\x01`\x01``\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x82\x01R\x84\x01Q\x80Q\x82\x90\x81\x10a\x17gWa\x17ga@\xFDV[` \x02` \x01\x01Q\x84`\0\x01Q\x82\x81Q\x81\x10a\x17\x85Wa\x17\x85a@\xFDV[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPP`\0[\x88` \x01QQ\x81\x10\x15a\x18HW`\x9E`\0\x85\x83\x81Q\x81\x10a\x17\xCBWa\x17\xCBa@\xFDV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82R\x81\x81\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 `\xFF\x87\x16\x82R\x90\x92R\x90 T\x85Q\x80Q`\x01`\x01``\x1B\x03\x90\x92\x16\x91\x84\x90\x81\x10a\x18\x16Wa\x18\x16a@\xFDV[` \x02` \x01\x01\x81\x81Qa\x18*\x91\x90aI\xF9V[`\x01`\x01``\x1B\x03\x16\x90RP\x80a\x18@\x81aA\xCAV[\x91PPa\x17\xA8V[P\x80a\x18S\x81aA\xCAV[\x91PPa\x16|V[P`\0\x80a\x18s\x8A\x88\x8B``\x01Q\x8C`\x80\x01Qa\x07\x94V[\x91P\x91P\x81a\x18\xF6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: pairing precompile call fai`d\x82\x01Rb\x1B\x19Y`\xEA\x1B`\x84\x82\x01R`\xA4\x01a\x06@V[\x80a\x19iW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: signature is invalid\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06@V[P\x92\x95PPPPPP[\x92\x91PPV[a\x19\x81a'\x0FV[`\x98\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`fT\x15a\x19\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FPausable: contract is paused\0\0\0\0`D\x82\x01R`d\x01a\x06@V[`\x98T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1AMW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FUpdater must be the caller\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06@V[`\x97T`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16a\x1Am`\x80\x84\x01``\x85\x01aA5V[c\xFF\xFF\xFF\xFF\x16\x14a\x1A\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FchainRdBatchNonce mismatch\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06@V[a\x1A\xD0`@\x84\x01` \x85\x01aJ!V[`\x02\x81\x11\x15a\x1A\xE1Wa\x1A\xE1a?iV[`\x97T`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x1A\xFFWa\x1A\xFFa?iV[\x14a\x1B<W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x15\xDC\x9B\xDB\x99\xC8\x18\xDA\x18Z[\x92Y`\x9A\x1B`D\x82\x01R`d\x01a\x06@V[`\x9ATc\xFF\xFF\xFF\xFF\x16\x15\x80a\x1BhWPa\x1BY` \x84\x01\x84aA5V[`\x9ATc\xFF\xFF\xFF\xFF\x91\x82\x16\x91\x16\x10[a\x1B\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkStale RdTask`\xA0\x1B`D\x82\x01R`d\x01a\x06@V[`\x9AT`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16`\0\x03a\x1B\xF4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x13\xDC\x08\x1C\xDD\x18]\x19H\x1D[\x9A[\x9A]`\x8A\x1B`D\x82\x01R`d\x01a\x06@V[a\x1C\x04`\xA0\x84\x01`\x80\x85\x01aA5V[`\x9AT`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14a\x1CeW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7Freference block hash mismatch\0\0\0`D\x82\x01R`d\x01a\x06@V[\x82`@Q` \x01a\x1Cv\x91\x90aJ<V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82` \x015\x14a\x1C\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FreferenceTaskHash hash mismatch\0`D\x82\x01R`d\x01a\x06@V[`\0a\x1D\x14\x83`@Q` \x01a\x1C\xF4\x91\x90aJ\xEEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x83a\x04\x88\x90aI\x88V[`\x9CT\x90\x91Pc\xFF\xFF\xFF\xFF\x16`\0[`\x9B\x80Ta\x1D0\x90aB\x0BV[\x90P\x81\x10\x15a\x1D\xF9W\x81`\xFF\x16\x83` \x01Q\x82\x81Q\x81\x10a\x1DSWa\x1DSa@\xFDV[` \x02` \x01\x01Qa\x1De\x91\x90aI\x94V[`\x01`\x01``\x1B\x03\x16`d\x84`\0\x01Q\x83\x81Q\x81\x10a\x1D\x86Wa\x1D\x86a@\xFDV[` \x02` \x01\x01Q`\x01`\x01``\x1B\x03\x16a\x1D\xA1\x91\x90aI\xC3V[\x10\x15a\x1D\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtFailed to meet quorum`X\x1B`D\x82\x01R`d\x01a\x06@V[\x80a\x1D\xF1\x81aA\xCAV[\x91PPa\x1D#V[P`@\x80Q\x80\x82\x01\x82R`\xA0\x86\x015\x81R`\xC0\x86\x015` \x82\x01\x90\x81R`\x97T\x92Qb#\xD0\xB5`\xE6\x1B\x81R`\x80\x88\x015`\x04\x82\x01R\x82Q`$\x82\x01R\x90Q`D\x82\x01R\x90\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x08\xF4-@\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1EkW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1E\x7FW=`\0\x80>=`\0\xFD[Pa\x1E\x94\x92PPP`\x80\x86\x01``\x87\x01aA5V[a\x1E\x9F\x90`\x01aA\xE3V[`\x97\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x01`\xA0\x1B\x02c\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90Ua\x1E\xD1` \x87\x01\x87aA5V[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x7F\xECh\xDB9\x18y\xB0\xF9\xF4 \xD1\xCD\xF3Gj\xFB\xDF\x08Z$b\xBFM+\x11\xDFxFb\x95\xCB\x17a\x1F\x19` \x88\x01\x88aA5V[a\x1F)`\x80\x89\x01``\x8A\x01aA5V[`@\x80Qc\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1PPPPPPV[a\x1FYa'\x0FV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1F\xBEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x06@V[a\x06R\x81a1\xFCV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a \x1AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a >\x91\x90a@1V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a nW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06@\x90a@NV[`fT\x19\x81\x19`fT\x19\x16\x14a \xECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06@V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x07\x89V[a!+a'\x0FV[`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F/ \xCF\x1B\xDAgs\x90D\xC5\xBFWsS\x97\x0C=\xBC\x18;,rt\xD1\xE8XJ\x10&\x922g\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\"\rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x06@V[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\"\x92a5}V[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x80\x82\x01\x84\x90R`\0\x90\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80a\"\xC1W\xFE[P\x80a\"\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCB[][\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x06@V[PP\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra##a5\x9BV[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R\x83Q`@\x80\x84\x01\x91\x90\x91R\x90\x84\x01Q``\x83\x01R`\0\x90\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80\x80a#^W\xFE[P\x80a\"\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCBXY\x19\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x06@V[a#\xA4a5\xB9V[P`@\x80Q`\x80\x81\x01\x82R\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81\x83\x01\x90\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED``\x83\x01R\x81R\x81Q\x80\x83\x01\x90\x92R\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x81\x01\x91\x90\x91R\x81\x01\x91\x90\x91R\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0\x80\x80a$\x8C`\0\x80Q` aM\x0E\x839\x81Q\x91R\x86aA\x13V[\x90P[a$\x98\x81a3\xCCV[\x90\x93P\x91P`\0\x80Q` aM\x0E\x839\x81Q\x91R\x82\x83\t\x83\x03a$\xD1W`@\x80Q\x80\x82\x01\x90\x91R\x90\x81R` \x81\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80Q` aM\x0E\x839\x81Q\x91R`\x01\x82\x08\x90Pa$\x8FV[`@\x80Q\x80\x82\x01\x82R\x86\x81R` \x80\x82\x01\x86\x90R\x82Q\x80\x84\x01\x90\x93R\x86\x83R\x82\x01\x84\x90R`\0\x91\x82\x91\x90a%\x1Da5\xDEV[`\0[`\x02\x81\x10\x15a&\xE2W`\0a%6\x82`\x06aI\xC3V[\x90P\x84\x82`\x02\x81\x10a%JWa%Ja@\xFDV[` \x02\x01QQ\x83a%\\\x83`\0aK\x81V[`\x0C\x81\x10a%lWa%la@\xFDV[` \x02\x01R\x84\x82`\x02\x81\x10a%\x83Wa%\x83a@\xFDV[` \x02\x01Q` \x01Q\x83\x82`\x01a%\x9A\x91\x90aK\x81V[`\x0C\x81\x10a%\xAAWa%\xAAa@\xFDV[` \x02\x01R\x83\x82`\x02\x81\x10a%\xC1Wa%\xC1a@\xFDV[` \x02\x01QQQ\x83a%\xD4\x83`\x02aK\x81V[`\x0C\x81\x10a%\xE4Wa%\xE4a@\xFDV[` \x02\x01R\x83\x82`\x02\x81\x10a%\xFBWa%\xFBa@\xFDV[` \x02\x01QQ`\x01` \x02\x01Q\x83a&\x14\x83`\x03aK\x81V[`\x0C\x81\x10a&$Wa&$a@\xFDV[` \x02\x01R\x83\x82`\x02\x81\x10a&;Wa&;a@\xFDV[` \x02\x01Q` \x01Q`\0`\x02\x81\x10a&VWa&Va@\xFDV[` \x02\x01Q\x83a&g\x83`\x04aK\x81V[`\x0C\x81\x10a&wWa&wa@\xFDV[` \x02\x01R\x83\x82`\x02\x81\x10a&\x8EWa&\x8Ea@\xFDV[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a&\xA9Wa&\xA9a@\xFDV[` \x02\x01Q\x83a&\xBA\x83`\x05aK\x81V[`\x0C\x81\x10a&\xCAWa&\xCAa@\xFDV[` \x02\x01RP\x80a&\xDA\x81aA\xCAV[\x91PPa% V[Pa&\xEBa5\xFDV[`\0` \x82a\x01\x80\x85`\x08\x8C\xFA\x91Q\x91\x9C\x91\x15\x15\x9BP\x90\x99PPPPPPPPPPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x13\x8DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x06@V[`\0[a'y` \x83\x01\x83aK\x99V[\x90P\x81\x10\x15a(>W`\x9D`\0a'\x93` \x85\x01\x85aK\x99V[\x84\x81\x81\x10a'\xA3Wa'\xA3a@\xFDV[\x90P` \x02\x01` \x81\x01\x90a'\xB8\x91\x90a6}V[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0\x90\x81 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U`\x9F\x91a'\xEB\x90\x85\x01\x85aK\x99V[\x84\x81\x81\x10a'\xFBWa'\xFBa@\xFDV[\x90P` \x02\x01` \x81\x01\x90a(\x10\x91\x90a6}V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x81\x81U`\x01\x01U\x80a(6\x81aA\xCAV[\x91PPa'lV[P`\0[a(O`@\x83\x01\x83aK\xE2V[\x90P\x81\x10\x15a)\x8AWa(e`@\x83\x01\x83aK\xE2V[\x82\x81\x81\x10a(uWa(ua@\xFDV[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a(\x8D\x91\x90aL+V[`\x9D`\0a(\x9E`@\x86\x01\x86aK\xE2V[\x85\x81\x81\x10a(\xAEWa(\xAEa@\xFDV[a(\xC4\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa6}V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x90\x81\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x17\x90\x91Ua)\x04\x90\x83\x01\x83aK\xE2V[\x82\x81\x81\x10a)\x14Wa)\x14a@\xFDV[\x90P`\x80\x02\x01`@\x01`\x9F`\0\x84\x80`@\x01\x90a)1\x91\x90aK\xE2V[\x85\x81\x81\x10a)AWa)Aa@\xFDV[a)W\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa6}V[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a)\x82\x81aA\xCAV[\x91PPa(BV[P`\0[a)\x9B``\x83\x01\x83aLFV[\x90P\x81\x10\x15a*SWa)\xB1``\x83\x01\x83aLFV[\x82\x81\x81\x10a)\xC1Wa)\xC1a@\xFDV[\x90P`@\x02\x01` \x01` \x81\x01\x90a)\xD9\x91\x90aL+V[`\x9D`\0a)\xEA``\x86\x01\x86aLFV[\x85\x81\x81\x10a)\xFAWa)\xFAa@\xFDV[a*\x10\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa6}V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a*K\x81aA\xCAV[\x91PPa)\x8EV[P`\0[a*d`\x80\x83\x01\x83aL\x8FV[\x90P\x81\x10\x15a+\0Wa*z`\x80\x83\x01\x83aL\x8FV[\x82\x81\x81\x10a*\x8AWa*\x8Aa@\xFDV[\x90P``\x02\x01` \x01`\x9F`\0\x84\x80`\x80\x01\x90a*\xA7\x91\x90aL\x8FV[\x85\x81\x81\x10a*\xB7Wa*\xB7a@\xFDV[a*\xCD\x92` ``\x90\x92\x02\x01\x90\x81\x01\x91Pa6}V[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a*\xF8\x81aA\xCAV[\x91PPa*WV[P`\0[a+\x11`\xA0\x83\x01\x83aK\x99V[\x90P\x81\x10\x15a,7W`\0[`\x9B\x80Ta+*\x90aB\x0BV[\x90P\x81\x10\x15a+\xE3W`\x9E`\0a+D`\xA0\x86\x01\x86aK\x99V[\x85\x81\x81\x10a+TWa+Ta@\xFDV[\x90P` \x02\x015\x81R` \x01\x90\x81R` \x01`\0 `\0`\x9B\x83\x81Ta+y\x90aB\x0BV[\x81\x10a+\x87Wa+\x87a@\xFDV[\x81T`\x01\x16\x15a+\xA6W\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U\x80a+\xDB\x81aA\xCAV[\x91PPa+\x1DV[P`\xA0`\0a+\xF4\x84\x83\x01\x85aK\x99V[\x84\x81\x81\x10a,\x04Wa,\x04a@\xFDV[` \x90\x81\x02\x92\x90\x92\x015\x83RP\x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\xFF\x19\x16\x90U\x80a,/\x81aA\xCAV[\x91PPa+\x04V[P`\0[a,H`\xC0\x83\x01\x83aK\x99V[\x90P\x81\x10\x15a.\x98Wa,^`\xC0\x83\x01\x83aK\x99V[\x82\x81\x81\x10a,nWa,na@\xFDV[\x90P` \x02\x81\x01\x90a,\x80\x91\x90aL\xD7V[a,\x91\x90`\x80\x81\x01\x90``\x01a6}V[`\xA0`\0a,\xA2`\xC0\x86\x01\x86aK\x99V[\x85\x81\x81\x10a,\xB2Wa,\xB2a@\xFDV[\x90P` \x02\x81\x01\x90a,\xC4\x91\x90aL\xD7V[`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP`\0[a,\xFE`\xC0\x84\x01\x84aK\x99V[\x83\x81\x81\x10a-\x0EWa-\x0Ea@\xFDV[\x90P` \x02\x81\x01\x90a- \x91\x90aL\xD7V[a-.\x90` \x81\x01\x90aK\x99V[\x90P\x81\x10\x15a.\x85Wa-D`\xC0\x84\x01\x84aK\x99V[\x83\x81\x81\x10a-TWa-Ta@\xFDV[\x90P` \x02\x81\x01\x90a-f\x91\x90aL\xD7V[a-t\x90`@\x81\x01\x90aK\x99V[\x82\x81\x81\x10a-\x84Wa-\x84a@\xFDV[\x90P` \x02\x01` \x81\x01\x90a-\x99\x91\x90aL+V[`\x9E`\0a-\xAA`\xC0\x87\x01\x87aK\x99V[\x86\x81\x81\x10a-\xBAWa-\xBAa@\xFDV[\x90P` \x02\x81\x01\x90a-\xCC\x91\x90aL\xD7V[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a-\xED`\xC0\x87\x01\x87aK\x99V[\x86\x81\x81\x10a-\xFDWa-\xFDa@\xFDV[\x90P` \x02\x81\x01\x90a.\x0F\x91\x90aL\xD7V[a.\x1D\x90` \x81\x01\x90aK\x99V[\x85\x81\x81\x10a.-Wa.-a@\xFDV[\x90P` \x02\x01` \x81\x01\x90a.B\x91\x90a6}V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a.}\x81aA\xCAV[\x91PPa,\xF1V[P\x80a.\x90\x81aA\xCAV[\x91PPa,;V[P`\0[a.\xA9`\xE0\x83\x01\x83aK\x99V[\x90P\x81\x10\x15a0\\W`\0[a.\xC2`\xE0\x84\x01\x84aK\x99V[\x83\x81\x81\x10a.\xD2Wa.\xD2a@\xFDV[\x90P` \x02\x81\x01\x90a.\xE4\x91\x90aL\xF7V[a.\xF2\x90` \x81\x01\x90aK\x99V[\x90P\x81\x10\x15a0IWa/\x08`\xE0\x84\x01\x84aK\x99V[\x83\x81\x81\x10a/\x18Wa/\x18a@\xFDV[\x90P` \x02\x81\x01\x90a/*\x91\x90aL\xF7V[a/8\x90`@\x81\x01\x90aK\x99V[\x82\x81\x81\x10a/HWa/Ha@\xFDV[\x90P` \x02\x01` \x81\x01\x90a/]\x91\x90aL+V[`\x9E`\0a/n`\xE0\x87\x01\x87aK\x99V[\x86\x81\x81\x10a/~Wa/~a@\xFDV[\x90P` \x02\x81\x01\x90a/\x90\x91\x90aL\xF7V[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a/\xB1`\xE0\x87\x01\x87aK\x99V[\x86\x81\x81\x10a/\xC1Wa/\xC1a@\xFDV[\x90P` \x02\x81\x01\x90a/\xD3\x91\x90aL\xF7V[a/\xE1\x90` \x81\x01\x90aK\x99V[\x85\x81\x81\x10a/\xF1Wa/\xF1a@\xFDV[\x90P` \x02\x01` \x81\x01\x90a0\x06\x91\x90a6}V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a0A\x81aA\xCAV[\x91PPa.\xB5V[P\x80a0T\x81aA\xCAV[\x91PPa.\x9CV[P`\0[a0na\x01\0\x83\x01\x83aLFV[\x90P\x81\x10\x15a1\x12Wa0\x85a\x01\0\x83\x01\x83aLFV[\x82\x81\x81\x10a0\x95Wa0\x95a@\xFDV[\x90P`@\x02\x01` \x01` \x81\x01\x90a0\xAD\x91\x90a6}V[`\xA0`\0a0\xBFa\x01\0\x86\x01\x86aLFV[\x85\x81\x81\x10a0\xCFWa0\xCFa@\xFDV[\x90P`@\x02\x01`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP\x80\x80a1\n\x90aA\xCAV[\x91PPa0`V[PPV[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a17WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a1\xB9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x06@V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a1\x12\x82a!\x7FV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x02\0\x82a\xFF\xFF\x16\x10a2\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Roscalar-too-large`\x80\x1B`D\x82\x01R`d\x01a\x06@V[\x81a\xFF\xFF\x16`\x01\x03a2\xBDWP\x81a\x19sV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01\x81\x90R\x84\x90`\x01\x90[\x81a\xFF\xFF\x16\x86a\xFF\xFF\x16\x10a3&W`\x01a\xFF\xFF\x87\x16`\xFF\x83\x16\x1C\x81\x16\x90\x03a3\tWa3\x06\x84\x84a#\x07V[\x93P[a3\x13\x83\x84a#\x07V[\x92Pb\x01\xFF\xFE`\x01\x92\x83\x1B\x16\x91\x01a2\xD9V[P\x91\x95\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q\x15\x80\x15a3VWP` \x82\x01Q\x15[\x15a3tWPP`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01`\0\x80Q` aM\x0E\x839\x81Q\x91R\x84` \x01Qa3\xA7\x91\x90aA\x13V[a3\xBF\x90`\0\x80Q` aM\x0E\x839\x81Q\x91RaI\xE2V[\x90R\x92\x91PPV[\x91\x90PV[`\0\x80\x80`\0\x80Q` aM\x0E\x839\x81Q\x91R`\x03`\0\x80Q` aM\x0E\x839\x81Q\x91R\x86`\0\x80Q` aM\x0E\x839\x81Q\x91R\x88\x89\t\t\x08\x90P`\0a4B\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\0\x80Q` aM\x0E\x839\x81Q\x91Ra4NV[\x91\x95\x91\x94P\x90\x92PPPV[`\0\x80a4Ya5\xFDV[a4aa6\x1BV[` \x80\x82R\x81\x81\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x88\x90R`\x80\x82\x01\x87\x90R`\xA0\x82\x01\x86\x90R\x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82\x80a4\x9EW\xFE[P\x82a4\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FBN254.expMod: call failure\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06@V[PQ\x91PP[\x93\x92PPPV[\x82\x80Ta5\x05\x90aB\x0BV[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82a5'W`\0\x85Ua5mV[\x82`\x1F\x10a5@W\x82\x80\x01`\xFF\x19\x825\x16\x17\x85Ua5mV[\x82\x80\x01`\x01\x01\x85U\x82\x15a5mW\x91\x82\x01[\x82\x81\x11\x15a5mW\x825\x82U\x91` \x01\x91\x90`\x01\x01\x90a5RV[Pa5y\x92\x91Pa69V[P\x90V[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80a5\xCCa6NV[\x81R` \x01a5\xD9a6NV[\x90R\x90V[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[[\x80\x82\x11\x15a5yW`\0\x81U`\x01\x01a6:V[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[\x805`\xFF\x81\x16\x81\x14a3\xC7W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a6\x8FW`\0\x80\xFD[a4\xF2\x82a6lV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06RW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a6\xBFW`\0\x80\xFD[\x815a4\xF2\x81a6\x98V[`\0` \x82\x84\x03\x12\x15a6\xDCW`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a7\x1BWa7\x1Ba6\xE3V[`@R\x90V[`@Qa\x01\0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a7\x1BWa7\x1Ba6\xE3V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a7lWa7la6\xE3V[`@R\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a7\x86W`\0\x80\xFD[a7\x8Ea6\xF9V[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a7\xB5W`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a7\xD7Wa7\xD7a6\xE3V[\x80`@RP\x80`@\x84\x01\x85\x81\x11\x15a7\xEEW`\0\x80\xFD[\x84[\x81\x81\x10\x15a3&W\x805\x83R` \x92\x83\x01\x92\x01a7\xF0V[`\0`\x80\x82\x84\x03\x12\x15a8\x1AW`\0\x80\xFD[a8\"a6\xF9V[\x90Pa8.\x83\x83a7\xA4V[\x81Ra8=\x83`@\x84\x01a7\xA4V[` \x82\x01R\x92\x91PPV[`\0\x80`\0\x80a\x01 \x85\x87\x03\x12\x15a8_W`\0\x80\xFD[\x845\x93Pa8p\x86` \x87\x01a7tV[\x92Pa8\x7F\x86``\x87\x01a8\x08V[\x91Pa8\x8E\x86`\xE0\x87\x01a7tV[\x90P\x92\x95\x91\x94P\x92PV[`\0`\xE0\x82\x84\x03\x12\x15a8\xABW`\0\x80\xFD[P\x91\x90PV[`\0a\x01 \x82\x84\x03\x12\x15a8\xABW`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12a8\xD6W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a8\xEDW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x06\x1B\x85\x01\x01\x11\x15a9\x08W`\0\x80\xFD[\x92P\x92\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a3\xC7W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0`\xA0\x88\x8A\x03\x12\x15a9>W`\0\x80\xFD[\x875`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a9UW`\0\x80\xFD[a9a\x8B\x83\x8C\x01a8\x99V[\x98P` \x8A\x015\x91P\x80\x82\x11\x15a9wW`\0\x80\xFD[a9\x83\x8B\x83\x8C\x01a8\xB1V[\x97P`@\x8A\x015\x91P\x80\x82\x11\x15a9\x99W`\0\x80\xFD[\x81\x8A\x01\x91P\x8A`\x1F\x83\x01\x12a9\xADW`\0\x80\xFD[\x815\x81\x81\x11\x15a9\xBCW`\0\x80\xFD[\x8B` \x82`\x05\x1B\x85\x01\x01\x11\x15a9\xD1W`\0\x80\xFD[` \x83\x01\x97P\x80\x96PP``\x8A\x015\x91P\x80\x82\x11\x15a9\xEFW`\0\x80\xFD[Pa9\xFC\x8A\x82\x8B\x01a8\xC4V[\x90\x94P\x92Pa:\x0F\x90P`\x80\x89\x01a9\x0FV[\x90P\x92\x95\x98\x91\x94\x97P\x92\x95PV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a:JW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a:.V[\x81\x81\x11\x15a:\\W`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[\x80\x15\x15\x81\x14a\x06RW`\0\x80\xFD[\x805a3\xC7\x81a:rV[\x805`\x03\x81\x10a3\xC7W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a:\xB3W`\0\x80\xFD[\x865a:\xBE\x81a6\x98V[\x95P` \x87\x015a:\xCE\x81a6\x98V[\x94P`@\x87\x015a:\xDE\x81a6\x98V[\x93P``\x87\x015a:\xEE\x81a:rV[\x92P`\x80\x87\x015a:\xFE\x81a6\x98V[\x91Pa;\x0C`\xA0\x88\x01a:\x8BV[\x90P\x92\x95P\x92\x95P\x92\x95V[`\0a\x01\x80\x82\x84\x03\x12\x15a8\xABW`\0\x80\xFD[`\0\x80`\0\x80\x84\x86\x03`\xC0\x81\x12\x15a;BW`\0\x80\xFD[\x855`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a;YW`\0\x80\xFD[a;e\x89\x83\x8A\x01a8\x99V[\x96P```\x1F\x19\x84\x01\x12\x15a;yW`\0\x80\xFD[` \x88\x01\x95P`\x80\x88\x015\x92P\x80\x83\x11\x15a;\x93W`\0\x80\xFD[a;\x9F\x89\x84\x8A\x01a;\x18V[\x94P`\xA0\x88\x015\x92P\x80\x83\x11\x15a;\xB5W`\0\x80\xFD[PPa;\xC3\x87\x82\x88\x01a8\xB1V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a;\xE2W`\0\x80\xFD[\x825\x91Pa;\xF2` \x84\x01a6lV[\x90P\x92P\x92\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a<\x14Wa<\x14a6\xE3V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a</W`\0\x80\xFD[\x815` a<Da<?\x83a;\xFBV[a7DV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a<cW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a<\x85Wa<x\x81a9\x0FV[\x83R\x91\x83\x01\x91\x83\x01a<gV[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a<\xA1W`\0\x80\xFD[\x815` a<\xB1a<?\x83a;\xFBV[\x82\x81R`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a<\xD0W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a<\x85Wa<\xE6\x88\x82a7tV[\x83R\x91\x83\x01\x91`@\x01a<\xD4V[`\0\x82`\x1F\x83\x01\x12a=\x05W`\0\x80\xFD[\x815` a=\x15a<?\x83a;\xFBV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a=4W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a<\x85W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a=WW`\0\x80\x81\xFD[a=e\x89\x86\x83\x8B\x01\x01a<\x1EV[\x84RP\x91\x83\x01\x91\x83\x01a=8V[`\0a\x01\x80\x82\x84\x03\x12\x15a=\x86W`\0\x80\xFD[a=\x8Ea7!V[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a=\xA7W`\0\x80\xFD[a=\xB3\x85\x83\x86\x01a<\x1EV[\x83R` \x84\x015\x91P\x80\x82\x11\x15a=\xC9W`\0\x80\xFD[a=\xD5\x85\x83\x86\x01a<\x90V[` \x84\x01R`@\x84\x015\x91P\x80\x82\x11\x15a=\xEEW`\0\x80\xFD[a=\xFA\x85\x83\x86\x01a<\x90V[`@\x84\x01Ra>\x0C\x85``\x86\x01a8\x08V[``\x84\x01Ra>\x1E\x85`\xE0\x86\x01a7tV[`\x80\x84\x01Ra\x01 \x84\x015\x91P\x80\x82\x11\x15a>8W`\0\x80\xFD[a>D\x85\x83\x86\x01a<\x1EV[`\xA0\x84\x01Ra\x01@\x84\x015\x91P\x80\x82\x11\x15a>^W`\0\x80\xFD[a>j\x85\x83\x86\x01a<\x1EV[`\xC0\x84\x01Ra\x01`\x84\x015\x91P\x80\x82\x11\x15a>\x84W`\0\x80\xFD[Pa>\x91\x84\x82\x85\x01a<\xF4V[`\xE0\x83\x01RP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a>\xB0W`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a>\xCDW`\0\x80\xFD[a>\xD9\x85\x82\x86\x01a=sV[\x91PP\x92P\x92\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a?\x1CW\x81Q`\x01`\x01``\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a>\xF7V[P\x94\x95\x94PPPPPV[` \x81R`\0\x82Q`@` \x84\x01Ra?C``\x84\x01\x82a>\xE3V[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01Ra?`\x82\x82a>\xE3V[\x95\x94PPPPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x03\x81\x10a?\x9DWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[` \x81\x01a\x19s\x82\x84a?\x7FV[`\0\x80`\0\x83\x85\x03a\x01@\x81\x12\x15a?\xC6W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a?\xDDW`\0\x80\xFD[a?\xE9\x88\x83\x89\x01a8\x99V[\x95Pa\x01\0`\x1F\x19\x84\x01\x12\x15a?\xFEW`\0\x80\xFD[` \x87\x01\x94Pa\x01 \x87\x015\x92P\x80\x83\x11\x15a@\x19W`\0\x80\xFD[PPa@'\x86\x82\x87\x01a;\x18V[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a@CW`\0\x80\xFD[\x81Qa4\xF2\x81a6\x98V[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15a@\xAAW`\0\x80\xFD[\x81Qa4\xF2\x81a:rV[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x82aA0WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\0` \x82\x84\x03\x12\x15aAGW`\0\x80\xFD[a4\xF2\x82a9\x0FV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aAgW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aA\x81W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a9\x08W`\0\x80\xFD[\x82\x81R``\x81\x01a4\xF2` \x83\x01\x84\x805\x82R` \x90\x81\x015\x91\x01RV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01aA\xDCWaA\xDCaA\xB4V[P`\x01\x01\x90V[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aB\x02WaB\x02aA\xB4V[\x01\x94\x93PPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80aB\x1FW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a8\xABWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aBVW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aBuW`\0\x80\xFD[\x806\x03\x83\x13\x15a9\x08W`\0\x80\xFD[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x81R`\0c\xFF\xFF\xFF\xFF\x80aB\xC2\x85a9\x0FV[\x16` \x84\x01R\x80aB\xD5` \x86\x01a9\x0FV[\x16`@\x84\x01R\x80aB\xE8`@\x86\x01a9\x0FV[\x16``\x84\x01RPaB\xFC``\x84\x01\x84aB?V[`\xE0`\x80\x85\x01RaC\x12a\x01\0\x85\x01\x82\x84aB\x84V[\x91PPaC!`\x80\x85\x01a9\x0FV[c\xFF\xFF\xFF\xFF\x81\x16`\xA0\x85\x01RPaC;`\xA0\x85\x01\x85aB?V[\x84\x83\x03`\x1F\x19\x01`\xC0\x86\x01RaCR\x83\x82\x84aB\x84V[\x92PPPaCb`\xC0\x85\x01a9\x0FV[c\xFF\xFF\xFF\xFF\x81\x16`\xE0\x85\x01RP\x93\x92PPPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aC\x8DW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aC\xACW`\0\x80\xFD[\x80`\x05\x1B6\x03\x83\x13\x15a9\x08W`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a?\x1CW`\xFFaC\xE1\x83a6lV[\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aC\xCEV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aD\x0BW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aD*W`\0\x80\xFD[\x80`\x07\x1B6\x03\x83\x13\x15a9\x08W`\0\x80\xFD[\x805`\x01`\x01``\x1B\x03\x81\x16\x81\x14a3\xC7W`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a?\x1CW`\xFFaDv\x83a6lV[\x16\x87R`\x01`\x01``\x1B\x03aD\x8C\x84\x84\x01aD<V[\x16\x83\x88\x01R`@\x82\x81\x015\x90\x88\x01R``\x80\x83\x015\x90\x88\x01R`\x80\x96\x87\x01\x96\x90\x91\x01\x90`\x01\x01aDcV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aD\xCEW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aD\xEDW`\0\x80\xFD[\x80`\x06\x1B6\x03\x83\x13\x15a9\x08W`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a?\x1CW`\xFFaE\"\x83a6lV[\x16\x87R`\x01`\x01``\x1B\x03aE8\x84\x84\x01aD<V[\x16\x87\x84\x01R`@\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aE\x0FV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aEgW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aE\x86W`\0\x80\xFD[``\x81\x026\x03\x83\x13\x15a9\x08W`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a?\x1CW`\xFFaE\xBB\x83a6lV[\x16\x87RaE\xD6\x83\x88\x01\x84\x84\x01\x805\x82R` \x90\x81\x015\x91\x01RV[``\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aE\xA8V[\x81\x83R`\0`\x01`\x01`\xFB\x1B\x03\x83\x11\x15aF\x02W`\0\x80\xFD[\x82`\x05\x1B\x80\x83` \x87\x017`\0\x94\x01` \x01\x93\x84RP\x91\x92\x91PPV[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a?\x1CW`\x01`\x01``\x1B\x03aFH\x83aD<V[\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aF/V[\x81\x83R`\0` \x80\x85\x01\x94P\x84\x84`\x05\x1B\x86\x01\x84`\0\x80[\x88\x81\x10\x15aG\tW\x84\x84\x03\x8AR\x825`~\x19\x896\x03\x01\x81\x12aF\x93W\x82\x83\xFD[\x88\x01\x805\x85R`\x80aF\xA7\x88\x83\x01\x83aCvV[\x82\x8A\x89\x01RaF\xB9\x83\x89\x01\x82\x84aC\xBEV[\x92PPP`@aF\xCB\x81\x84\x01\x84aCvV[\x88\x84\x03\x83\x8A\x01RaF\xDD\x84\x82\x84aF\x1FV[\x93PPPP```\xFFaF\xF1\x82\x85\x01a6lV[\x16\x96\x01\x95\x90\x95RP\x98\x85\x01\x98\x91\x85\x01\x91`\x01\x01aFsV[P\x91\x98\x97PPPPPPPPV[\x81\x83R`\0` \x80\x85\x01\x94P\x84\x84`\x05\x1B\x86\x01\x84`\0\x80[\x88\x81\x10\x15aG\tW\x84\x84\x03\x8AR\x825`^\x19\x896\x03\x01\x81\x12aGOW\x82\x83\xFD[\x88\x01\x805\x85R``aGc\x88\x83\x01\x83aCvV[\x82\x8A\x89\x01RaGu\x83\x89\x01\x82\x84aC\xBEV[\x92PPP`@aG\x87\x81\x84\x01\x84aCvV[\x93P\x87\x83\x03\x82\x89\x01RaG\x9B\x83\x85\x83aF\x1FV[\x9D\x8A\x01\x9D\x97PPP\x93\x87\x01\x93PP`\x01\x01aG/V[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a?\x1CW\x815\x87R`\xFFaG\xDA\x84\x84\x01a6lV[\x16\x87\x84\x01R`@\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aG\xC1V[` \x81RaH\x0C` \x82\x01aH\x06\x84a:\x80V[\x15\x15\x90RV[`\0aH\x1B` \x84\x01\x84aCvV[a\x01 \x80`@\x86\x01RaH3a\x01@\x86\x01\x83\x85aC\xBEV[\x92PaHB`@\x87\x01\x87aC\xF4V[\x92P`\x1F\x19\x80\x87\x86\x03\x01``\x88\x01RaH\\\x85\x85\x84aDSV[\x94PaHk``\x89\x01\x89aD\xB7V[\x94P\x91P\x80\x87\x86\x03\x01`\x80\x88\x01RaH\x84\x85\x85\x84aD\xFFV[\x94PaH\x93`\x80\x89\x01\x89aEPV[\x94P\x91P\x80\x87\x86\x03\x01`\xA0\x88\x01RaH\xAC\x85\x85\x84aE\x98V[\x94PaH\xBB`\xA0\x89\x01\x89aCvV[\x94P\x91P\x80\x87\x86\x03\x01`\xC0\x88\x01RaH\xD4\x85\x85\x84aE\xE9V[\x94PaH\xE3`\xC0\x89\x01\x89aCvV[\x94P\x91P\x80\x87\x86\x03\x01`\xE0\x88\x01RaH\xFC\x85\x85\x84aF[V[\x94PaI\x0B`\xE0\x89\x01\x89aCvV[\x94P\x91Pa\x01\0\x81\x88\x87\x03\x01\x81\x89\x01RaI&\x86\x86\x85aG\x17V[\x95PaI4\x81\x8A\x01\x8AaD\xB7V[\x95P\x92PP\x80\x87\x86\x03\x01\x83\x88\x01RPaIN\x84\x84\x83aG\xB1V[\x97\x96PPPPPPPV[``\x81\x01c\xFF\xFF\xFF\xFFaIk\x84a9\x0FV[\x16\x82R` \x83\x015` \x83\x01R`@\x83\x015`@\x83\x01R\x92\x91PPV[`\0a\x19s6\x83a=sV[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aI\xBAWaI\xBAaA\xB4V[\x02\x94\x93PPPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15aI\xDDWaI\xDDaA\xB4V[P\x02\x90V[`\0\x82\x82\x10\x15aI\xF4WaI\xF4aA\xB4V[P\x03\x90V[`\0`\x01`\x01``\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15aJ\x19WaJ\x19aA\xB4V[\x03\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15aJ3W`\0\x80\xFD[a4\xF2\x82a:\x8BV[` \x81R`\0c\xFF\xFF\xFF\xFF\x80aJQ\x85a9\x0FV[\x16` \x84\x01RaJc` \x85\x01a:\x8BV[aJp`@\x85\x01\x82a?\x7FV[P\x80aJ~`@\x86\x01a9\x0FV[\x16``\x84\x01R\x80aJ\x91``\x86\x01a9\x0FV[\x16`\x80\x84\x01R\x80aJ\xA4`\x80\x86\x01a9\x0FV[\x16`\xA0\x84\x01RaJ\xB7`\xA0\x85\x01\x85aB?V[`\xE0`\xC0\x86\x01RaJ\xCDa\x01\0\x86\x01\x82\x84aB\x84V[\x91PP\x81aJ\xDD`\xC0\x87\x01a9\x0FV[\x16`\xE0\x85\x01R\x80\x92PPP\x92\x91PPV[a\x01\0\x81\x01c\xFF\xFF\xFF\xFF\x80aK\x02\x85a9\x0FV[\x16\x83R` \x84\x015` \x84\x01RaK\x1B`@\x85\x01a:\x8BV[aK(`@\x85\x01\x82a?\x7FV[P\x80aK6``\x86\x01a9\x0FV[\x16``\x84\x01RP`\x80\x83\x015`\x80\x83\x01R`\xA0\x83\x015`\xA0\x83\x01R`\xC0\x83\x015`\xC0\x83\x01R`\xE0\x83\x015aKi\x81a6\x98V[`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x92\x90\x92\x01\x91\x90\x91R\x91\x90PV[`\0\x82\x19\x82\x11\x15aK\x94WaK\x94aA\xB4V[P\x01\x90V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aK\xB0W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aK\xCAW`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a9\x08W`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aK\xF9W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aL\x13W`\0\x80\xFD[` \x01\x91P`\x07\x81\x90\x1B6\x03\x82\x13\x15a9\x08W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aL=W`\0\x80\xFD[a4\xF2\x82aD<V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aL]W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aLwW`\0\x80\xFD[` \x01\x91P`\x06\x81\x90\x1B6\x03\x82\x13\x15a9\x08W`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aL\xA6W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aL\xC0W`\0\x80\xFD[` \x01\x91P``\x81\x026\x03\x82\x13\x15a9\x08W`\0\x80\xFD[`\0\x825`~\x19\x836\x03\x01\x81\x12aL\xEDW`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x825`^\x19\x836\x03\x01\x81\x12aL\xEDW`\0\x80\xFD\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 \xD2\x80\x1A\xC5\xE3\x14\xD7D\xED\x19x\x15\xF9\xBA\x11\x9B\xEB\x82\x11D\x11\xDD\xF3a\xBF\xC4\xC5\xD3v\xB5\xE5\xAFdsolcC\0\x08\r\x003";
    /// The bytecode of the contract.
    pub static GASPMULTIROLLUPSERVICE_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\x06W`\x005`\xE0\x1C\x80co\x0C0\xA4\x11a\x01\x1AW\x80c\xD0:\x07\xB2\x11a\0\xADW\x80c\xED9\xE5\x02\x11a\0|W\x80c\xED9\xE5\x02\x14a\x05DW\x80c\xF2\xFD\xE3\x8B\x14a\x05WW\x80c\xF8N\x91\xFC\x14a\x05jW\x80c\xFA\xBC\x1C\xBC\x14a\x05sW\x80c\xFD\xC1]\xE8\x14a\x05\x86W`\0\x80\xFD[\x80c\xD0:\x07\xB2\x14a\x04\xF2W\x80c\xDE\xB4\x03}\x14a\x05\x02W\x80c\xDF\x03L\xD0\x14a\x05\x19W\x80c\xE2\xA7\xCBf\x14a\x05,W`\0\x80\xFD[\x80c\x88o\x11\x95\x11a\0\xE9W\x80c\x88o\x11\x95\x14a\x04\x9AW\x80c\x8D\xA5\xCB[\x14a\x04\xADW\x80c\x9A\x8A\x05\x92\x14a\x04\xBEW\x80c\x9DT\xF4\x19\x14a\x04\xDFW`\0\x80\xFD[\x80co\x0C0\xA4\x14a\x042W\x80cqP\x18\xA6\x14a\x04IW\x80cz\xD7Ua\x14a\x04QW\x80c}\x97\x88\x97\x14a\x04zW`\0\x80\xFD[\x80c4\xFA\xDB\xEA\x11a\x01\x9DW\x80cM\xEA\xBC!\x11a\x01lW\x80cM\xEA\xBC!\x14a\x03\xBCW\x80cRn>d\x14a\x03\xE1W\x80cY\\jg\x14a\x03\xF5W\x80cZ\xC8j\xB7\x14a\x03\xFDW\x80c\\\x97Z\xBB\x14a\x04 W`\0\x80\xFD[\x80c4\xFA\xDB\xEA\x14a\x02\xFDW\x80c=\x9F\xB0\x0C\x14a\x03\x10W\x80cC\r;9\x14a\x03;W\x80cI\x9Do\xB6\x14a\x03pW`\0\x80\xFD[\x80c\x17\x1F\x1D[\x11a\x01\xD9W\x80c\x17\x1F\x1D[\x14a\x02\x98W\x80c&5\xE7N\x14a\x02\xC2W\x80c*\x84\x14\xFD\x14a\x02\xD5W\x80c0\xC4}\x8E\x14a\x02\xEAW`\0\x80\xFD[\x80c\x03\xD0\x97\xD2\x14a\x02\x0BW\x80c\x0E\xE0\xFD\xBD\x14a\x02LW\x80c\x10\xD6z/\x14a\x02pW\x80c\x13d9\xDD\x14a\x02\x85W[`\0\x80\xFD[a\x022a\x02\x196`\x04a6}V[`\x9F` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T\x82V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[`\x98Ta\x02`\x90`\x01`\xA8\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02CV[a\x02\x83a\x02~6`\x04a6\xADV[a\x05\x99V[\0[a\x02\x83a\x02\x936`\x04a6\xCAV[a\x06UV[a\x02\xABa\x02\xA66`\x04a8HV[a\x07\x94V[`@\x80Q\x92\x15\x15\x83R\x90\x15\x15` \x83\x01R\x01a\x02CV[a\x02\x83a\x02\xD06`\x04a9#V[a\t\x1EV[a\x02\xDDa\x0ByV[`@Qa\x02C\x91\x90a:\x1DV[a\x02\x83a\x02\xF86`\x04a:\x9AV[a\x0C\x07V[a\x02\x83a\x03\x0B6`\x04a;+V[a\r\xAFV[`\x97Ta\x03#\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02CV[a\x03^a\x03I6`\x04a6\xCAV[`\xA0` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02CV[a\x03\xA4a\x03~6`\x04a;\xCFV[`\x9E` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\x01`\x01``\x1B\x03\x16\x81V[`@Q`\x01`\x01``\x1B\x03\x90\x91\x16\x81R` \x01a\x02CV[`\x9CTa\x03\xCC\x90c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02CV[`\x98Ta\x02`\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[a\x02\x83a\x12\xB4V[a\x02`a\x04\x0B6`\x04a6}V[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`fT[`@Q\x90\x81R` \x01a\x02CV[`\x9ATa\x03\xCC\x90`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x02\x83a\x13{V[a\x03\xA4a\x04_6`\x04a6}V[`\x9D` R`\0\x90\x81R`@\x90 T`\x01`\x01``\x1B\x03\x16\x81V[a\x04\x8Da\x04\x886`\x04a>\x9DV[a\x13\x8FV[`@Qa\x02C\x91\x90a?'V[`eTa\x03#\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x03#V[`\x97Ta\x04\xD2\x90`\x01`\xC0\x1B\x90\x04`\xFF\x16\x81V[`@Qa\x02C\x91\x90a?\xA1V[a\x02\x83a\x04\xED6`\x04a6\xADV[a\x19yV[`\x9ATa\x03\xCC\x90c\xFF\xFF\xFF\xFF\x16\x81V[`\x97Ta\x03\xCC\x90`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`\x98Ta\x03#\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x9ATa\x03\xCC\x90d\x01\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x02\x83a\x05R6`\x04a?\xAFV[a\x19\xA3V[a\x02\x83a\x05e6`\x04a6\xADV[a\x1FQV[a\x04$`\x99T\x81V[a\x02\x83a\x05\x816`\x04a6\xCAV[a\x1F\xC7V[a\x02\x83a\x05\x946`\x04a6\xADV[a!#V[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xECW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x10\x91\x90a@1V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x06IW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06@\x90a@NV[`@Q\x80\x91\x03\x90\xFD[a\x06R\x81a!\x7FV[PV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x9DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xC1\x91\x90a@\x98V[a\x06\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06@\x90a@\xB5V[`fT\x81\x81\x16\x14a\x07VW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06@V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`\0\x80`\0\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x87\x87`\0\x01Q\x88` \x01Q\x88`\0\x01Q`\0`\x02\x81\x10a\x07\xDCWa\x07\xDCa@\xFDV[` \x02\x01Q\x89Q`\x01` \x02\x01Q\x8A` \x01Q`\0`\x02\x81\x10a\x08\x01Wa\x08\x01a@\xFDV[` \x02\x01Q\x8B` \x01Q`\x01`\x02\x81\x10a\x08\x1DWa\x08\x1Da@\xFDV[` \x90\x81\x02\x91\x90\x91\x01Q\x8CQ\x8D\x83\x01Q`@Qa\x08z\x9A\x99\x98\x97\x96\x95\x94\x01\x98\x89R` \x89\x01\x97\x90\x97R`@\x88\x01\x95\x90\x95R``\x87\x01\x93\x90\x93R`\x80\x86\x01\x91\x90\x91R`\xA0\x85\x01R`\xC0\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01Ra\x01 \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\0\x1Ca\x08\x9D\x91\x90aA\x13V[\x90Pa\t\x10a\x08\xB6a\x08\xAF\x88\x84a\"vV[\x86\x90a#\x07V[a\x08\xBEa#\x9CV[a\t\x06a\x08\xF7\x85a\x08\xF1`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x01\x82R`\x02\x90\x82\x01R\x90V[\x90a\"vV[a\t\0\x8Ca$\\V[\x90a#\x07V[\x88b\x01\xD4\xC0a$\xEBV[\x90\x98\x90\x97P\x95PPPPPPV[a\t&a'\x0FV[\x83\x82\x14a\tuW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FrdUpdate info length mismatch\0\0\0`D\x82\x01R`d\x01a\x06@V[a\t~\x86a'iV[a\t\x8B` \x88\x01\x88aA5V[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90Ua\t\xC2`@\x88\x01` \x89\x01aA5V[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x01`@\x1B\x02c\xFF\xFF\xFF\xFF`@\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90UB`\x99Ua\t\xF8``\x88\x01\x88aAPV[a\n\x04\x91`\x9B\x91a4\xF9V[Pa\n\x15`\xA0\x88\x01`\x80\x89\x01aA5V[`\x9C\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U`\0[\x84\x81\x10\x15a\n\xDEW`\x97T`\x01`\x01`\xA0\x1B\x03\x16c\x08\xF4-@\x87\x87\x84\x81\x81\x10a\n]Wa\n]a@\xFDV[\x90P` \x02\x015\x86\x86\x85\x81\x81\x10a\nvWa\nva@\xFDV[\x90P`@\x02\x01`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n\x99\x92\x91\x90aA\x96V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\n\xB3W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\n\xC7W=`\0\x80>=`\0\xFD[PPPP\x80\x80a\n\xD6\x90aA\xCAV[\x91PPa\n2V[P\x83\x15a\x0B\x11Wa\n\xF0\x81`\x01aA\xE3V[`\x97`\x14a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP[\x7F&Ie\xEBk\xC46\xC6\xC4sC\x1D4\xAFV\xE82\xEC4O\xDF\xD4>\xE6\xAFo\xCEm ^\x84\xAFa\x0B?` \x89\x01\x89aA5V[a\x0BO`@\x8A\x01` \x8B\x01aA5V[`@\x80Qc\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01[`@Q\x80\x91\x03\x90\xA1PPPPPPPV[`\x9B\x80Ta\x0B\x86\x90aB\x0BV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B\xB2\x90aB\x0BV[\x80\x15a\x0B\xFFW\x80`\x1F\x10a\x0B\xD4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B\xFFV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\xE2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x0C'WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x0CAWP0;\x15\x80\x15a\x0CAWP`\0T`\xFF\x16`\x01\x14[a\x0C\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x06@V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x0C\xC7W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x0C\xD2\x87`\0a1\x16V[a\x0C\xDB\x86a1\xFCV[`\x98\x80T\x85\x15\x15`\x01`\xA8\x1B\x02`\x01a\xFF\x01`\xA0\x1B\x03\x19\x90\x91\x16`\x01`\x01`\xA0\x1B\x03\x80\x89\x16\x91\x90\x91\x17\x91\x90\x91\x17\x90\x91U`\x97\x80T\x91\x85\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x82U\x84\x92`\x01d\xFF\0\0\0\x01`\xA0\x1B\x03\x19\x16\x17`\x01`\xC0\x1B\x83`\x02\x81\x11\x15a\rIWa\rIa?iV[\x02\x17\x90UP`\x97\x80Tc\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x17\x90U\x80\x15a\r\xA6W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01a\x0BhV[PPPPPPPV[`fT\x15a\r\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FPausable: contract is paused\0\0\0\0`D\x82\x01R`d\x01a\x06@V[`\x98T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0EYW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FUpdater must be the caller\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06@V[`\x9AT`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x80\x15\x15\x80a\x0E\x80WP`\x98T`\x01`\xA8\x1B\x90\x04`\xFF\x16[\x15a\x0E\xCCW`\x98T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0E\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04\x17WF\x83`\xDC\x1B`D\x82\x01R`d\x01a\x06@V[a\x0F\x0EV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0F\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdAuth1`\xD8\x1B`D\x82\x01R`d\x01a\x06@V[\x84`@Q` \x01a\x0F\x1F\x91\x90aB\xADV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x015\x14a\x0F\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FreferenceTaskHash hash mismatch\0`D\x82\x01R`d\x01a\x06@V[\x81`@Q` \x01a\x0F\x98\x91\x90aG\xF2V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84`@\x015\x14a\x10\0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FoperatorStateInfo hash mismatch\0`D\x82\x01R`d\x01a\x06@V[c\xFF\xFF\xFF\xFF\x81\x16\x15a\x11\x95Wa\x10\x1C``\x86\x01`@\x87\x01aA5V[c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x14a\x10vW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7Freference block mismatch\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06@V[`\0a\x10\xAC\x85`@Q` \x01a\x10\x8C\x91\x90aIYV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x85a\x04\x88\x90aI\x88V[`\x9CT\x90\x91Pc\xFF\xFF\xFF\xFF\x16`\0[`\x9B\x80Ta\x10\xC8\x90aB\x0BV[\x90P\x81\x10\x15a\x11\x91W\x81`\xFF\x16\x83` \x01Q\x82\x81Q\x81\x10a\x10\xEBWa\x10\xEBa@\xFDV[` \x02` \x01\x01Qa\x10\xFD\x91\x90aI\x94V[`\x01`\x01``\x1B\x03\x16`d\x84`\0\x01Q\x83\x81Q\x81\x10a\x11\x1EWa\x11\x1Ea@\xFDV[` \x02` \x01\x01Q`\x01`\x01``\x1B\x03\x16a\x119\x91\x90aI\xC3V[\x10\x15a\x11\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtFailed to meet quorum`X\x1B`D\x82\x01R`d\x01a\x06@V[\x80a\x11\x89\x81aA\xCAV[\x91PPa\x10\xBBV[PPP[a\x11\x9E\x82a'iV[a\x11\xAB` \x86\x01\x86aA5V[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90Ua\x11\xE2`@\x86\x01` \x87\x01aA5V[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x01`@\x1B\x02c\xFF\xFF\xFF\xFF`@\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90UB`\x99Ua\x12\x18``\x86\x01\x86aAPV[a\x12$\x91`\x9B\x91a4\xF9V[Pa\x125`\xA0\x86\x01`\x80\x87\x01aA5V[`\x9C\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x7F6\xA1\xFD{\xD5T\xF5\xC4(\xC9\x82\x9C\t\xC6`kL\x89;\x1F\xAD\xC8sZz\x12yW\x97D}\xEDa\x12}` \x87\x01\x87aA5V[a\x12\x8D`@\x88\x01` \x89\x01aA5V[`@\x80Qc\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1PPPPPV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xFCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13 \x91\x90a@\x98V[a\x13<W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06@\x90a@\xB5V[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[a\x13\x83a'\x0FV[a\x13\x8D`\0a1\xFCV[V[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01\x81\x90R`\x9B\x80Ta\x13\xC6\x90aB\x0BV[\x90P\x90Pa\x13\xE7`@Q\x80`@\x01`@R\x80``\x81R` \x01``\x81RP\x90V[\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x13\xFFWa\x13\xFFa6\xE3V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x14(W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P` \x82\x01R\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14FWa\x14Fa6\xE3V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x14oW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x81R` \x85\x01QQ`\0\x90`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14\x92Wa\x14\x92a6\xE3V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x14\xBBW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80[\x87` \x01QQ\x81\x10\x15a\x16mWa\x15\x07\x88` \x01Q\x82\x81Q\x81\x10a\x14\xE8Wa\x14\xE8a@\xFDV[` \x02` \x01\x01Q\x80Q`\0\x90\x81R` \x91\x82\x01Q\x90\x91R`@\x90 \x90V[\x83\x82\x81Q\x81\x10a\x15\x19Wa\x15\x19a@\xFDV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80\x15a\x15\xE3W\x82a\x156`\x01\x83aI\xE2V[\x81Q\x81\x10a\x15FWa\x15Fa@\xFDV[` \x02` \x01\x01Q`\0\x1C\x83\x82\x81Q\x81\x10a\x15cWa\x15ca@\xFDV[` \x02` \x01\x01Q`\0\x1C\x11a\x15\xE3W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: nonSignerPubkeys not sorted`d\x82\x01R`\x84\x01a\x06@V[a\x16Ya\x16R`\xA0`\0\x86\x85\x81Q\x81\x10a\x15\xFFWa\x15\xFFa@\xFDV[` \x02` \x01\x01Q\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\xFF\x16\x8A` \x01Q\x84\x81Q\x81\x10a\x16<Wa\x16<a@\xFDV[` \x02` \x01\x01Qa2N\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x87\x90a#\x07V[\x95P\x80a\x16e\x81aA\xCAV[\x91PPa\x14\xC2V[Pa\x16w\x85a31V[\x94P`\0[\x84\x81\x10\x15a\x18[W`\x9B\x81\x81Ta\x16\x92\x90aB\x0BV[\x81\x10a\x16\xA0Wa\x16\xA0a@\xFDV[\x81T`\x01\x16\x15a\x16\xBFW\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C`\0\x81\x81R`\x9F` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x01T\x90\x82\x01R\x90\x92Pa\x17\0\x90\x87\x90a#\x07V[`\xFF\x83\x16`\0\x90\x81R`\x9D` \x90\x81R`@\x90\x91 T\x90\x86\x01Q\x80Q\x92\x98P`\x01`\x01``\x1B\x03\x90\x91\x16\x91\x83\x90\x81\x10a\x17;Wa\x17;a@\xFDV[`\x01`\x01``\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x82\x01R\x84\x01Q\x80Q\x82\x90\x81\x10a\x17gWa\x17ga@\xFDV[` \x02` \x01\x01Q\x84`\0\x01Q\x82\x81Q\x81\x10a\x17\x85Wa\x17\x85a@\xFDV[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPP`\0[\x88` \x01QQ\x81\x10\x15a\x18HW`\x9E`\0\x85\x83\x81Q\x81\x10a\x17\xCBWa\x17\xCBa@\xFDV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82R\x81\x81\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 `\xFF\x87\x16\x82R\x90\x92R\x90 T\x85Q\x80Q`\x01`\x01``\x1B\x03\x90\x92\x16\x91\x84\x90\x81\x10a\x18\x16Wa\x18\x16a@\xFDV[` \x02` \x01\x01\x81\x81Qa\x18*\x91\x90aI\xF9V[`\x01`\x01``\x1B\x03\x16\x90RP\x80a\x18@\x81aA\xCAV[\x91PPa\x17\xA8V[P\x80a\x18S\x81aA\xCAV[\x91PPa\x16|V[P`\0\x80a\x18s\x8A\x88\x8B``\x01Q\x8C`\x80\x01Qa\x07\x94V[\x91P\x91P\x81a\x18\xF6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: pairing precompile call fai`d\x82\x01Rb\x1B\x19Y`\xEA\x1B`\x84\x82\x01R`\xA4\x01a\x06@V[\x80a\x19iW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: signature is invalid\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06@V[P\x92\x95PPPPPP[\x92\x91PPV[a\x19\x81a'\x0FV[`\x98\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`fT\x15a\x19\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FPausable: contract is paused\0\0\0\0`D\x82\x01R`d\x01a\x06@V[`\x98T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1AMW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FUpdater must be the caller\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06@V[`\x97T`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16a\x1Am`\x80\x84\x01``\x85\x01aA5V[c\xFF\xFF\xFF\xFF\x16\x14a\x1A\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FchainRdBatchNonce mismatch\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06@V[a\x1A\xD0`@\x84\x01` \x85\x01aJ!V[`\x02\x81\x11\x15a\x1A\xE1Wa\x1A\xE1a?iV[`\x97T`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x1A\xFFWa\x1A\xFFa?iV[\x14a\x1B<W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x15\xDC\x9B\xDB\x99\xC8\x18\xDA\x18Z[\x92Y`\x9A\x1B`D\x82\x01R`d\x01a\x06@V[`\x9ATc\xFF\xFF\xFF\xFF\x16\x15\x80a\x1BhWPa\x1BY` \x84\x01\x84aA5V[`\x9ATc\xFF\xFF\xFF\xFF\x91\x82\x16\x91\x16\x10[a\x1B\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkStale RdTask`\xA0\x1B`D\x82\x01R`d\x01a\x06@V[`\x9AT`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16`\0\x03a\x1B\xF4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x13\xDC\x08\x1C\xDD\x18]\x19H\x1D[\x9A[\x9A]`\x8A\x1B`D\x82\x01R`d\x01a\x06@V[a\x1C\x04`\xA0\x84\x01`\x80\x85\x01aA5V[`\x9AT`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14a\x1CeW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7Freference block hash mismatch\0\0\0`D\x82\x01R`d\x01a\x06@V[\x82`@Q` \x01a\x1Cv\x91\x90aJ<V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82` \x015\x14a\x1C\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FreferenceTaskHash hash mismatch\0`D\x82\x01R`d\x01a\x06@V[`\0a\x1D\x14\x83`@Q` \x01a\x1C\xF4\x91\x90aJ\xEEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x83a\x04\x88\x90aI\x88V[`\x9CT\x90\x91Pc\xFF\xFF\xFF\xFF\x16`\0[`\x9B\x80Ta\x1D0\x90aB\x0BV[\x90P\x81\x10\x15a\x1D\xF9W\x81`\xFF\x16\x83` \x01Q\x82\x81Q\x81\x10a\x1DSWa\x1DSa@\xFDV[` \x02` \x01\x01Qa\x1De\x91\x90aI\x94V[`\x01`\x01``\x1B\x03\x16`d\x84`\0\x01Q\x83\x81Q\x81\x10a\x1D\x86Wa\x1D\x86a@\xFDV[` \x02` \x01\x01Q`\x01`\x01``\x1B\x03\x16a\x1D\xA1\x91\x90aI\xC3V[\x10\x15a\x1D\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtFailed to meet quorum`X\x1B`D\x82\x01R`d\x01a\x06@V[\x80a\x1D\xF1\x81aA\xCAV[\x91PPa\x1D#V[P`@\x80Q\x80\x82\x01\x82R`\xA0\x86\x015\x81R`\xC0\x86\x015` \x82\x01\x90\x81R`\x97T\x92Qb#\xD0\xB5`\xE6\x1B\x81R`\x80\x88\x015`\x04\x82\x01R\x82Q`$\x82\x01R\x90Q`D\x82\x01R\x90\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x08\xF4-@\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1EkW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1E\x7FW=`\0\x80>=`\0\xFD[Pa\x1E\x94\x92PPP`\x80\x86\x01``\x87\x01aA5V[a\x1E\x9F\x90`\x01aA\xE3V[`\x97\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x01`\xA0\x1B\x02c\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90Ua\x1E\xD1` \x87\x01\x87aA5V[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x7F\xECh\xDB9\x18y\xB0\xF9\xF4 \xD1\xCD\xF3Gj\xFB\xDF\x08Z$b\xBFM+\x11\xDFxFb\x95\xCB\x17a\x1F\x19` \x88\x01\x88aA5V[a\x1F)`\x80\x89\x01``\x8A\x01aA5V[`@\x80Qc\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1PPPPPPV[a\x1FYa'\x0FV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1F\xBEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x06@V[a\x06R\x81a1\xFCV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a \x1AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a >\x91\x90a@1V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a nW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06@\x90a@NV[`fT\x19\x81\x19`fT\x19\x16\x14a \xECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06@V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x07\x89V[a!+a'\x0FV[`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F/ \xCF\x1B\xDAgs\x90D\xC5\xBFWsS\x97\x0C=\xBC\x18;,rt\xD1\xE8XJ\x10&\x922g\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\"\rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x06@V[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\"\x92a5}V[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x80\x82\x01\x84\x90R`\0\x90\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80a\"\xC1W\xFE[P\x80a\"\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCB[][\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x06@V[PP\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra##a5\x9BV[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R\x83Q`@\x80\x84\x01\x91\x90\x91R\x90\x84\x01Q``\x83\x01R`\0\x90\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80\x80a#^W\xFE[P\x80a\"\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCBXY\x19\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x06@V[a#\xA4a5\xB9V[P`@\x80Q`\x80\x81\x01\x82R\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81\x83\x01\x90\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED``\x83\x01R\x81R\x81Q\x80\x83\x01\x90\x92R\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x81\x01\x91\x90\x91R\x81\x01\x91\x90\x91R\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0\x80\x80a$\x8C`\0\x80Q` aM\x0E\x839\x81Q\x91R\x86aA\x13V[\x90P[a$\x98\x81a3\xCCV[\x90\x93P\x91P`\0\x80Q` aM\x0E\x839\x81Q\x91R\x82\x83\t\x83\x03a$\xD1W`@\x80Q\x80\x82\x01\x90\x91R\x90\x81R` \x81\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80Q` aM\x0E\x839\x81Q\x91R`\x01\x82\x08\x90Pa$\x8FV[`@\x80Q\x80\x82\x01\x82R\x86\x81R` \x80\x82\x01\x86\x90R\x82Q\x80\x84\x01\x90\x93R\x86\x83R\x82\x01\x84\x90R`\0\x91\x82\x91\x90a%\x1Da5\xDEV[`\0[`\x02\x81\x10\x15a&\xE2W`\0a%6\x82`\x06aI\xC3V[\x90P\x84\x82`\x02\x81\x10a%JWa%Ja@\xFDV[` \x02\x01QQ\x83a%\\\x83`\0aK\x81V[`\x0C\x81\x10a%lWa%la@\xFDV[` \x02\x01R\x84\x82`\x02\x81\x10a%\x83Wa%\x83a@\xFDV[` \x02\x01Q` \x01Q\x83\x82`\x01a%\x9A\x91\x90aK\x81V[`\x0C\x81\x10a%\xAAWa%\xAAa@\xFDV[` \x02\x01R\x83\x82`\x02\x81\x10a%\xC1Wa%\xC1a@\xFDV[` \x02\x01QQQ\x83a%\xD4\x83`\x02aK\x81V[`\x0C\x81\x10a%\xE4Wa%\xE4a@\xFDV[` \x02\x01R\x83\x82`\x02\x81\x10a%\xFBWa%\xFBa@\xFDV[` \x02\x01QQ`\x01` \x02\x01Q\x83a&\x14\x83`\x03aK\x81V[`\x0C\x81\x10a&$Wa&$a@\xFDV[` \x02\x01R\x83\x82`\x02\x81\x10a&;Wa&;a@\xFDV[` \x02\x01Q` \x01Q`\0`\x02\x81\x10a&VWa&Va@\xFDV[` \x02\x01Q\x83a&g\x83`\x04aK\x81V[`\x0C\x81\x10a&wWa&wa@\xFDV[` \x02\x01R\x83\x82`\x02\x81\x10a&\x8EWa&\x8Ea@\xFDV[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a&\xA9Wa&\xA9a@\xFDV[` \x02\x01Q\x83a&\xBA\x83`\x05aK\x81V[`\x0C\x81\x10a&\xCAWa&\xCAa@\xFDV[` \x02\x01RP\x80a&\xDA\x81aA\xCAV[\x91PPa% V[Pa&\xEBa5\xFDV[`\0` \x82a\x01\x80\x85`\x08\x8C\xFA\x91Q\x91\x9C\x91\x15\x15\x9BP\x90\x99PPPPPPPPPPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x13\x8DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x06@V[`\0[a'y` \x83\x01\x83aK\x99V[\x90P\x81\x10\x15a(>W`\x9D`\0a'\x93` \x85\x01\x85aK\x99V[\x84\x81\x81\x10a'\xA3Wa'\xA3a@\xFDV[\x90P` \x02\x01` \x81\x01\x90a'\xB8\x91\x90a6}V[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0\x90\x81 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U`\x9F\x91a'\xEB\x90\x85\x01\x85aK\x99V[\x84\x81\x81\x10a'\xFBWa'\xFBa@\xFDV[\x90P` \x02\x01` \x81\x01\x90a(\x10\x91\x90a6}V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x81\x81U`\x01\x01U\x80a(6\x81aA\xCAV[\x91PPa'lV[P`\0[a(O`@\x83\x01\x83aK\xE2V[\x90P\x81\x10\x15a)\x8AWa(e`@\x83\x01\x83aK\xE2V[\x82\x81\x81\x10a(uWa(ua@\xFDV[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a(\x8D\x91\x90aL+V[`\x9D`\0a(\x9E`@\x86\x01\x86aK\xE2V[\x85\x81\x81\x10a(\xAEWa(\xAEa@\xFDV[a(\xC4\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa6}V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x90\x81\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x17\x90\x91Ua)\x04\x90\x83\x01\x83aK\xE2V[\x82\x81\x81\x10a)\x14Wa)\x14a@\xFDV[\x90P`\x80\x02\x01`@\x01`\x9F`\0\x84\x80`@\x01\x90a)1\x91\x90aK\xE2V[\x85\x81\x81\x10a)AWa)Aa@\xFDV[a)W\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa6}V[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a)\x82\x81aA\xCAV[\x91PPa(BV[P`\0[a)\x9B``\x83\x01\x83aLFV[\x90P\x81\x10\x15a*SWa)\xB1``\x83\x01\x83aLFV[\x82\x81\x81\x10a)\xC1Wa)\xC1a@\xFDV[\x90P`@\x02\x01` \x01` \x81\x01\x90a)\xD9\x91\x90aL+V[`\x9D`\0a)\xEA``\x86\x01\x86aLFV[\x85\x81\x81\x10a)\xFAWa)\xFAa@\xFDV[a*\x10\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa6}V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a*K\x81aA\xCAV[\x91PPa)\x8EV[P`\0[a*d`\x80\x83\x01\x83aL\x8FV[\x90P\x81\x10\x15a+\0Wa*z`\x80\x83\x01\x83aL\x8FV[\x82\x81\x81\x10a*\x8AWa*\x8Aa@\xFDV[\x90P``\x02\x01` \x01`\x9F`\0\x84\x80`\x80\x01\x90a*\xA7\x91\x90aL\x8FV[\x85\x81\x81\x10a*\xB7Wa*\xB7a@\xFDV[a*\xCD\x92` ``\x90\x92\x02\x01\x90\x81\x01\x91Pa6}V[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a*\xF8\x81aA\xCAV[\x91PPa*WV[P`\0[a+\x11`\xA0\x83\x01\x83aK\x99V[\x90P\x81\x10\x15a,7W`\0[`\x9B\x80Ta+*\x90aB\x0BV[\x90P\x81\x10\x15a+\xE3W`\x9E`\0a+D`\xA0\x86\x01\x86aK\x99V[\x85\x81\x81\x10a+TWa+Ta@\xFDV[\x90P` \x02\x015\x81R` \x01\x90\x81R` \x01`\0 `\0`\x9B\x83\x81Ta+y\x90aB\x0BV[\x81\x10a+\x87Wa+\x87a@\xFDV[\x81T`\x01\x16\x15a+\xA6W\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U\x80a+\xDB\x81aA\xCAV[\x91PPa+\x1DV[P`\xA0`\0a+\xF4\x84\x83\x01\x85aK\x99V[\x84\x81\x81\x10a,\x04Wa,\x04a@\xFDV[` \x90\x81\x02\x92\x90\x92\x015\x83RP\x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\xFF\x19\x16\x90U\x80a,/\x81aA\xCAV[\x91PPa+\x04V[P`\0[a,H`\xC0\x83\x01\x83aK\x99V[\x90P\x81\x10\x15a.\x98Wa,^`\xC0\x83\x01\x83aK\x99V[\x82\x81\x81\x10a,nWa,na@\xFDV[\x90P` \x02\x81\x01\x90a,\x80\x91\x90aL\xD7V[a,\x91\x90`\x80\x81\x01\x90``\x01a6}V[`\xA0`\0a,\xA2`\xC0\x86\x01\x86aK\x99V[\x85\x81\x81\x10a,\xB2Wa,\xB2a@\xFDV[\x90P` \x02\x81\x01\x90a,\xC4\x91\x90aL\xD7V[`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP`\0[a,\xFE`\xC0\x84\x01\x84aK\x99V[\x83\x81\x81\x10a-\x0EWa-\x0Ea@\xFDV[\x90P` \x02\x81\x01\x90a- \x91\x90aL\xD7V[a-.\x90` \x81\x01\x90aK\x99V[\x90P\x81\x10\x15a.\x85Wa-D`\xC0\x84\x01\x84aK\x99V[\x83\x81\x81\x10a-TWa-Ta@\xFDV[\x90P` \x02\x81\x01\x90a-f\x91\x90aL\xD7V[a-t\x90`@\x81\x01\x90aK\x99V[\x82\x81\x81\x10a-\x84Wa-\x84a@\xFDV[\x90P` \x02\x01` \x81\x01\x90a-\x99\x91\x90aL+V[`\x9E`\0a-\xAA`\xC0\x87\x01\x87aK\x99V[\x86\x81\x81\x10a-\xBAWa-\xBAa@\xFDV[\x90P` \x02\x81\x01\x90a-\xCC\x91\x90aL\xD7V[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a-\xED`\xC0\x87\x01\x87aK\x99V[\x86\x81\x81\x10a-\xFDWa-\xFDa@\xFDV[\x90P` \x02\x81\x01\x90a.\x0F\x91\x90aL\xD7V[a.\x1D\x90` \x81\x01\x90aK\x99V[\x85\x81\x81\x10a.-Wa.-a@\xFDV[\x90P` \x02\x01` \x81\x01\x90a.B\x91\x90a6}V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a.}\x81aA\xCAV[\x91PPa,\xF1V[P\x80a.\x90\x81aA\xCAV[\x91PPa,;V[P`\0[a.\xA9`\xE0\x83\x01\x83aK\x99V[\x90P\x81\x10\x15a0\\W`\0[a.\xC2`\xE0\x84\x01\x84aK\x99V[\x83\x81\x81\x10a.\xD2Wa.\xD2a@\xFDV[\x90P` \x02\x81\x01\x90a.\xE4\x91\x90aL\xF7V[a.\xF2\x90` \x81\x01\x90aK\x99V[\x90P\x81\x10\x15a0IWa/\x08`\xE0\x84\x01\x84aK\x99V[\x83\x81\x81\x10a/\x18Wa/\x18a@\xFDV[\x90P` \x02\x81\x01\x90a/*\x91\x90aL\xF7V[a/8\x90`@\x81\x01\x90aK\x99V[\x82\x81\x81\x10a/HWa/Ha@\xFDV[\x90P` \x02\x01` \x81\x01\x90a/]\x91\x90aL+V[`\x9E`\0a/n`\xE0\x87\x01\x87aK\x99V[\x86\x81\x81\x10a/~Wa/~a@\xFDV[\x90P` \x02\x81\x01\x90a/\x90\x91\x90aL\xF7V[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a/\xB1`\xE0\x87\x01\x87aK\x99V[\x86\x81\x81\x10a/\xC1Wa/\xC1a@\xFDV[\x90P` \x02\x81\x01\x90a/\xD3\x91\x90aL\xF7V[a/\xE1\x90` \x81\x01\x90aK\x99V[\x85\x81\x81\x10a/\xF1Wa/\xF1a@\xFDV[\x90P` \x02\x01` \x81\x01\x90a0\x06\x91\x90a6}V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a0A\x81aA\xCAV[\x91PPa.\xB5V[P\x80a0T\x81aA\xCAV[\x91PPa.\x9CV[P`\0[a0na\x01\0\x83\x01\x83aLFV[\x90P\x81\x10\x15a1\x12Wa0\x85a\x01\0\x83\x01\x83aLFV[\x82\x81\x81\x10a0\x95Wa0\x95a@\xFDV[\x90P`@\x02\x01` \x01` \x81\x01\x90a0\xAD\x91\x90a6}V[`\xA0`\0a0\xBFa\x01\0\x86\x01\x86aLFV[\x85\x81\x81\x10a0\xCFWa0\xCFa@\xFDV[\x90P`@\x02\x01`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP\x80\x80a1\n\x90aA\xCAV[\x91PPa0`V[PPV[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a17WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a1\xB9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x06@V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a1\x12\x82a!\x7FV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x02\0\x82a\xFF\xFF\x16\x10a2\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Roscalar-too-large`\x80\x1B`D\x82\x01R`d\x01a\x06@V[\x81a\xFF\xFF\x16`\x01\x03a2\xBDWP\x81a\x19sV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01\x81\x90R\x84\x90`\x01\x90[\x81a\xFF\xFF\x16\x86a\xFF\xFF\x16\x10a3&W`\x01a\xFF\xFF\x87\x16`\xFF\x83\x16\x1C\x81\x16\x90\x03a3\tWa3\x06\x84\x84a#\x07V[\x93P[a3\x13\x83\x84a#\x07V[\x92Pb\x01\xFF\xFE`\x01\x92\x83\x1B\x16\x91\x01a2\xD9V[P\x91\x95\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q\x15\x80\x15a3VWP` \x82\x01Q\x15[\x15a3tWPP`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01`\0\x80Q` aM\x0E\x839\x81Q\x91R\x84` \x01Qa3\xA7\x91\x90aA\x13V[a3\xBF\x90`\0\x80Q` aM\x0E\x839\x81Q\x91RaI\xE2V[\x90R\x92\x91PPV[\x91\x90PV[`\0\x80\x80`\0\x80Q` aM\x0E\x839\x81Q\x91R`\x03`\0\x80Q` aM\x0E\x839\x81Q\x91R\x86`\0\x80Q` aM\x0E\x839\x81Q\x91R\x88\x89\t\t\x08\x90P`\0a4B\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\0\x80Q` aM\x0E\x839\x81Q\x91Ra4NV[\x91\x95\x91\x94P\x90\x92PPPV[`\0\x80a4Ya5\xFDV[a4aa6\x1BV[` \x80\x82R\x81\x81\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x88\x90R`\x80\x82\x01\x87\x90R`\xA0\x82\x01\x86\x90R\x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82\x80a4\x9EW\xFE[P\x82a4\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FBN254.expMod: call failure\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06@V[PQ\x91PP[\x93\x92PPPV[\x82\x80Ta5\x05\x90aB\x0BV[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82a5'W`\0\x85Ua5mV[\x82`\x1F\x10a5@W\x82\x80\x01`\xFF\x19\x825\x16\x17\x85Ua5mV[\x82\x80\x01`\x01\x01\x85U\x82\x15a5mW\x91\x82\x01[\x82\x81\x11\x15a5mW\x825\x82U\x91` \x01\x91\x90`\x01\x01\x90a5RV[Pa5y\x92\x91Pa69V[P\x90V[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80a5\xCCa6NV[\x81R` \x01a5\xD9a6NV[\x90R\x90V[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[[\x80\x82\x11\x15a5yW`\0\x81U`\x01\x01a6:V[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[\x805`\xFF\x81\x16\x81\x14a3\xC7W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a6\x8FW`\0\x80\xFD[a4\xF2\x82a6lV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06RW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a6\xBFW`\0\x80\xFD[\x815a4\xF2\x81a6\x98V[`\0` \x82\x84\x03\x12\x15a6\xDCW`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a7\x1BWa7\x1Ba6\xE3V[`@R\x90V[`@Qa\x01\0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a7\x1BWa7\x1Ba6\xE3V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a7lWa7la6\xE3V[`@R\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a7\x86W`\0\x80\xFD[a7\x8Ea6\xF9V[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a7\xB5W`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a7\xD7Wa7\xD7a6\xE3V[\x80`@RP\x80`@\x84\x01\x85\x81\x11\x15a7\xEEW`\0\x80\xFD[\x84[\x81\x81\x10\x15a3&W\x805\x83R` \x92\x83\x01\x92\x01a7\xF0V[`\0`\x80\x82\x84\x03\x12\x15a8\x1AW`\0\x80\xFD[a8\"a6\xF9V[\x90Pa8.\x83\x83a7\xA4V[\x81Ra8=\x83`@\x84\x01a7\xA4V[` \x82\x01R\x92\x91PPV[`\0\x80`\0\x80a\x01 \x85\x87\x03\x12\x15a8_W`\0\x80\xFD[\x845\x93Pa8p\x86` \x87\x01a7tV[\x92Pa8\x7F\x86``\x87\x01a8\x08V[\x91Pa8\x8E\x86`\xE0\x87\x01a7tV[\x90P\x92\x95\x91\x94P\x92PV[`\0`\xE0\x82\x84\x03\x12\x15a8\xABW`\0\x80\xFD[P\x91\x90PV[`\0a\x01 \x82\x84\x03\x12\x15a8\xABW`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12a8\xD6W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a8\xEDW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x06\x1B\x85\x01\x01\x11\x15a9\x08W`\0\x80\xFD[\x92P\x92\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a3\xC7W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0`\xA0\x88\x8A\x03\x12\x15a9>W`\0\x80\xFD[\x875`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a9UW`\0\x80\xFD[a9a\x8B\x83\x8C\x01a8\x99V[\x98P` \x8A\x015\x91P\x80\x82\x11\x15a9wW`\0\x80\xFD[a9\x83\x8B\x83\x8C\x01a8\xB1V[\x97P`@\x8A\x015\x91P\x80\x82\x11\x15a9\x99W`\0\x80\xFD[\x81\x8A\x01\x91P\x8A`\x1F\x83\x01\x12a9\xADW`\0\x80\xFD[\x815\x81\x81\x11\x15a9\xBCW`\0\x80\xFD[\x8B` \x82`\x05\x1B\x85\x01\x01\x11\x15a9\xD1W`\0\x80\xFD[` \x83\x01\x97P\x80\x96PP``\x8A\x015\x91P\x80\x82\x11\x15a9\xEFW`\0\x80\xFD[Pa9\xFC\x8A\x82\x8B\x01a8\xC4V[\x90\x94P\x92Pa:\x0F\x90P`\x80\x89\x01a9\x0FV[\x90P\x92\x95\x98\x91\x94\x97P\x92\x95PV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a:JW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a:.V[\x81\x81\x11\x15a:\\W`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[\x80\x15\x15\x81\x14a\x06RW`\0\x80\xFD[\x805a3\xC7\x81a:rV[\x805`\x03\x81\x10a3\xC7W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a:\xB3W`\0\x80\xFD[\x865a:\xBE\x81a6\x98V[\x95P` \x87\x015a:\xCE\x81a6\x98V[\x94P`@\x87\x015a:\xDE\x81a6\x98V[\x93P``\x87\x015a:\xEE\x81a:rV[\x92P`\x80\x87\x015a:\xFE\x81a6\x98V[\x91Pa;\x0C`\xA0\x88\x01a:\x8BV[\x90P\x92\x95P\x92\x95P\x92\x95V[`\0a\x01\x80\x82\x84\x03\x12\x15a8\xABW`\0\x80\xFD[`\0\x80`\0\x80\x84\x86\x03`\xC0\x81\x12\x15a;BW`\0\x80\xFD[\x855`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a;YW`\0\x80\xFD[a;e\x89\x83\x8A\x01a8\x99V[\x96P```\x1F\x19\x84\x01\x12\x15a;yW`\0\x80\xFD[` \x88\x01\x95P`\x80\x88\x015\x92P\x80\x83\x11\x15a;\x93W`\0\x80\xFD[a;\x9F\x89\x84\x8A\x01a;\x18V[\x94P`\xA0\x88\x015\x92P\x80\x83\x11\x15a;\xB5W`\0\x80\xFD[PPa;\xC3\x87\x82\x88\x01a8\xB1V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a;\xE2W`\0\x80\xFD[\x825\x91Pa;\xF2` \x84\x01a6lV[\x90P\x92P\x92\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a<\x14Wa<\x14a6\xE3V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a</W`\0\x80\xFD[\x815` a<Da<?\x83a;\xFBV[a7DV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a<cW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a<\x85Wa<x\x81a9\x0FV[\x83R\x91\x83\x01\x91\x83\x01a<gV[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a<\xA1W`\0\x80\xFD[\x815` a<\xB1a<?\x83a;\xFBV[\x82\x81R`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a<\xD0W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a<\x85Wa<\xE6\x88\x82a7tV[\x83R\x91\x83\x01\x91`@\x01a<\xD4V[`\0\x82`\x1F\x83\x01\x12a=\x05W`\0\x80\xFD[\x815` a=\x15a<?\x83a;\xFBV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a=4W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a<\x85W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a=WW`\0\x80\x81\xFD[a=e\x89\x86\x83\x8B\x01\x01a<\x1EV[\x84RP\x91\x83\x01\x91\x83\x01a=8V[`\0a\x01\x80\x82\x84\x03\x12\x15a=\x86W`\0\x80\xFD[a=\x8Ea7!V[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a=\xA7W`\0\x80\xFD[a=\xB3\x85\x83\x86\x01a<\x1EV[\x83R` \x84\x015\x91P\x80\x82\x11\x15a=\xC9W`\0\x80\xFD[a=\xD5\x85\x83\x86\x01a<\x90V[` \x84\x01R`@\x84\x015\x91P\x80\x82\x11\x15a=\xEEW`\0\x80\xFD[a=\xFA\x85\x83\x86\x01a<\x90V[`@\x84\x01Ra>\x0C\x85``\x86\x01a8\x08V[``\x84\x01Ra>\x1E\x85`\xE0\x86\x01a7tV[`\x80\x84\x01Ra\x01 \x84\x015\x91P\x80\x82\x11\x15a>8W`\0\x80\xFD[a>D\x85\x83\x86\x01a<\x1EV[`\xA0\x84\x01Ra\x01@\x84\x015\x91P\x80\x82\x11\x15a>^W`\0\x80\xFD[a>j\x85\x83\x86\x01a<\x1EV[`\xC0\x84\x01Ra\x01`\x84\x015\x91P\x80\x82\x11\x15a>\x84W`\0\x80\xFD[Pa>\x91\x84\x82\x85\x01a<\xF4V[`\xE0\x83\x01RP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a>\xB0W`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a>\xCDW`\0\x80\xFD[a>\xD9\x85\x82\x86\x01a=sV[\x91PP\x92P\x92\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a?\x1CW\x81Q`\x01`\x01``\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a>\xF7V[P\x94\x95\x94PPPPPV[` \x81R`\0\x82Q`@` \x84\x01Ra?C``\x84\x01\x82a>\xE3V[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01Ra?`\x82\x82a>\xE3V[\x95\x94PPPPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x03\x81\x10a?\x9DWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[` \x81\x01a\x19s\x82\x84a?\x7FV[`\0\x80`\0\x83\x85\x03a\x01@\x81\x12\x15a?\xC6W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a?\xDDW`\0\x80\xFD[a?\xE9\x88\x83\x89\x01a8\x99V[\x95Pa\x01\0`\x1F\x19\x84\x01\x12\x15a?\xFEW`\0\x80\xFD[` \x87\x01\x94Pa\x01 \x87\x015\x92P\x80\x83\x11\x15a@\x19W`\0\x80\xFD[PPa@'\x86\x82\x87\x01a;\x18V[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a@CW`\0\x80\xFD[\x81Qa4\xF2\x81a6\x98V[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15a@\xAAW`\0\x80\xFD[\x81Qa4\xF2\x81a:rV[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x82aA0WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\0` \x82\x84\x03\x12\x15aAGW`\0\x80\xFD[a4\xF2\x82a9\x0FV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aAgW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aA\x81W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a9\x08W`\0\x80\xFD[\x82\x81R``\x81\x01a4\xF2` \x83\x01\x84\x805\x82R` \x90\x81\x015\x91\x01RV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01aA\xDCWaA\xDCaA\xB4V[P`\x01\x01\x90V[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aB\x02WaB\x02aA\xB4V[\x01\x94\x93PPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80aB\x1FW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a8\xABWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aBVW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aBuW`\0\x80\xFD[\x806\x03\x83\x13\x15a9\x08W`\0\x80\xFD[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x81R`\0c\xFF\xFF\xFF\xFF\x80aB\xC2\x85a9\x0FV[\x16` \x84\x01R\x80aB\xD5` \x86\x01a9\x0FV[\x16`@\x84\x01R\x80aB\xE8`@\x86\x01a9\x0FV[\x16``\x84\x01RPaB\xFC``\x84\x01\x84aB?V[`\xE0`\x80\x85\x01RaC\x12a\x01\0\x85\x01\x82\x84aB\x84V[\x91PPaC!`\x80\x85\x01a9\x0FV[c\xFF\xFF\xFF\xFF\x81\x16`\xA0\x85\x01RPaC;`\xA0\x85\x01\x85aB?V[\x84\x83\x03`\x1F\x19\x01`\xC0\x86\x01RaCR\x83\x82\x84aB\x84V[\x92PPPaCb`\xC0\x85\x01a9\x0FV[c\xFF\xFF\xFF\xFF\x81\x16`\xE0\x85\x01RP\x93\x92PPPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aC\x8DW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aC\xACW`\0\x80\xFD[\x80`\x05\x1B6\x03\x83\x13\x15a9\x08W`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a?\x1CW`\xFFaC\xE1\x83a6lV[\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aC\xCEV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aD\x0BW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aD*W`\0\x80\xFD[\x80`\x07\x1B6\x03\x83\x13\x15a9\x08W`\0\x80\xFD[\x805`\x01`\x01``\x1B\x03\x81\x16\x81\x14a3\xC7W`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a?\x1CW`\xFFaDv\x83a6lV[\x16\x87R`\x01`\x01``\x1B\x03aD\x8C\x84\x84\x01aD<V[\x16\x83\x88\x01R`@\x82\x81\x015\x90\x88\x01R``\x80\x83\x015\x90\x88\x01R`\x80\x96\x87\x01\x96\x90\x91\x01\x90`\x01\x01aDcV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aD\xCEW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aD\xEDW`\0\x80\xFD[\x80`\x06\x1B6\x03\x83\x13\x15a9\x08W`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a?\x1CW`\xFFaE\"\x83a6lV[\x16\x87R`\x01`\x01``\x1B\x03aE8\x84\x84\x01aD<V[\x16\x87\x84\x01R`@\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aE\x0FV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aEgW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aE\x86W`\0\x80\xFD[``\x81\x026\x03\x83\x13\x15a9\x08W`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a?\x1CW`\xFFaE\xBB\x83a6lV[\x16\x87RaE\xD6\x83\x88\x01\x84\x84\x01\x805\x82R` \x90\x81\x015\x91\x01RV[``\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aE\xA8V[\x81\x83R`\0`\x01`\x01`\xFB\x1B\x03\x83\x11\x15aF\x02W`\0\x80\xFD[\x82`\x05\x1B\x80\x83` \x87\x017`\0\x94\x01` \x01\x93\x84RP\x91\x92\x91PPV[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a?\x1CW`\x01`\x01``\x1B\x03aFH\x83aD<V[\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aF/V[\x81\x83R`\0` \x80\x85\x01\x94P\x84\x84`\x05\x1B\x86\x01\x84`\0\x80[\x88\x81\x10\x15aG\tW\x84\x84\x03\x8AR\x825`~\x19\x896\x03\x01\x81\x12aF\x93W\x82\x83\xFD[\x88\x01\x805\x85R`\x80aF\xA7\x88\x83\x01\x83aCvV[\x82\x8A\x89\x01RaF\xB9\x83\x89\x01\x82\x84aC\xBEV[\x92PPP`@aF\xCB\x81\x84\x01\x84aCvV[\x88\x84\x03\x83\x8A\x01RaF\xDD\x84\x82\x84aF\x1FV[\x93PPPP```\xFFaF\xF1\x82\x85\x01a6lV[\x16\x96\x01\x95\x90\x95RP\x98\x85\x01\x98\x91\x85\x01\x91`\x01\x01aFsV[P\x91\x98\x97PPPPPPPPV[\x81\x83R`\0` \x80\x85\x01\x94P\x84\x84`\x05\x1B\x86\x01\x84`\0\x80[\x88\x81\x10\x15aG\tW\x84\x84\x03\x8AR\x825`^\x19\x896\x03\x01\x81\x12aGOW\x82\x83\xFD[\x88\x01\x805\x85R``aGc\x88\x83\x01\x83aCvV[\x82\x8A\x89\x01RaGu\x83\x89\x01\x82\x84aC\xBEV[\x92PPP`@aG\x87\x81\x84\x01\x84aCvV[\x93P\x87\x83\x03\x82\x89\x01RaG\x9B\x83\x85\x83aF\x1FV[\x9D\x8A\x01\x9D\x97PPP\x93\x87\x01\x93PP`\x01\x01aG/V[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a?\x1CW\x815\x87R`\xFFaG\xDA\x84\x84\x01a6lV[\x16\x87\x84\x01R`@\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aG\xC1V[` \x81RaH\x0C` \x82\x01aH\x06\x84a:\x80V[\x15\x15\x90RV[`\0aH\x1B` \x84\x01\x84aCvV[a\x01 \x80`@\x86\x01RaH3a\x01@\x86\x01\x83\x85aC\xBEV[\x92PaHB`@\x87\x01\x87aC\xF4V[\x92P`\x1F\x19\x80\x87\x86\x03\x01``\x88\x01RaH\\\x85\x85\x84aDSV[\x94PaHk``\x89\x01\x89aD\xB7V[\x94P\x91P\x80\x87\x86\x03\x01`\x80\x88\x01RaH\x84\x85\x85\x84aD\xFFV[\x94PaH\x93`\x80\x89\x01\x89aEPV[\x94P\x91P\x80\x87\x86\x03\x01`\xA0\x88\x01RaH\xAC\x85\x85\x84aE\x98V[\x94PaH\xBB`\xA0\x89\x01\x89aCvV[\x94P\x91P\x80\x87\x86\x03\x01`\xC0\x88\x01RaH\xD4\x85\x85\x84aE\xE9V[\x94PaH\xE3`\xC0\x89\x01\x89aCvV[\x94P\x91P\x80\x87\x86\x03\x01`\xE0\x88\x01RaH\xFC\x85\x85\x84aF[V[\x94PaI\x0B`\xE0\x89\x01\x89aCvV[\x94P\x91Pa\x01\0\x81\x88\x87\x03\x01\x81\x89\x01RaI&\x86\x86\x85aG\x17V[\x95PaI4\x81\x8A\x01\x8AaD\xB7V[\x95P\x92PP\x80\x87\x86\x03\x01\x83\x88\x01RPaIN\x84\x84\x83aG\xB1V[\x97\x96PPPPPPPV[``\x81\x01c\xFF\xFF\xFF\xFFaIk\x84a9\x0FV[\x16\x82R` \x83\x015` \x83\x01R`@\x83\x015`@\x83\x01R\x92\x91PPV[`\0a\x19s6\x83a=sV[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aI\xBAWaI\xBAaA\xB4V[\x02\x94\x93PPPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15aI\xDDWaI\xDDaA\xB4V[P\x02\x90V[`\0\x82\x82\x10\x15aI\xF4WaI\xF4aA\xB4V[P\x03\x90V[`\0`\x01`\x01``\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15aJ\x19WaJ\x19aA\xB4V[\x03\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15aJ3W`\0\x80\xFD[a4\xF2\x82a:\x8BV[` \x81R`\0c\xFF\xFF\xFF\xFF\x80aJQ\x85a9\x0FV[\x16` \x84\x01RaJc` \x85\x01a:\x8BV[aJp`@\x85\x01\x82a?\x7FV[P\x80aJ~`@\x86\x01a9\x0FV[\x16``\x84\x01R\x80aJ\x91``\x86\x01a9\x0FV[\x16`\x80\x84\x01R\x80aJ\xA4`\x80\x86\x01a9\x0FV[\x16`\xA0\x84\x01RaJ\xB7`\xA0\x85\x01\x85aB?V[`\xE0`\xC0\x86\x01RaJ\xCDa\x01\0\x86\x01\x82\x84aB\x84V[\x91PP\x81aJ\xDD`\xC0\x87\x01a9\x0FV[\x16`\xE0\x85\x01R\x80\x92PPP\x92\x91PPV[a\x01\0\x81\x01c\xFF\xFF\xFF\xFF\x80aK\x02\x85a9\x0FV[\x16\x83R` \x84\x015` \x84\x01RaK\x1B`@\x85\x01a:\x8BV[aK(`@\x85\x01\x82a?\x7FV[P\x80aK6``\x86\x01a9\x0FV[\x16``\x84\x01RP`\x80\x83\x015`\x80\x83\x01R`\xA0\x83\x015`\xA0\x83\x01R`\xC0\x83\x015`\xC0\x83\x01R`\xE0\x83\x015aKi\x81a6\x98V[`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x92\x90\x92\x01\x91\x90\x91R\x91\x90PV[`\0\x82\x19\x82\x11\x15aK\x94WaK\x94aA\xB4V[P\x01\x90V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aK\xB0W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aK\xCAW`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a9\x08W`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aK\xF9W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aL\x13W`\0\x80\xFD[` \x01\x91P`\x07\x81\x90\x1B6\x03\x82\x13\x15a9\x08W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aL=W`\0\x80\xFD[a4\xF2\x82aD<V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aL]W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aLwW`\0\x80\xFD[` \x01\x91P`\x06\x81\x90\x1B6\x03\x82\x13\x15a9\x08W`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aL\xA6W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aL\xC0W`\0\x80\xFD[` \x01\x91P``\x81\x026\x03\x82\x13\x15a9\x08W`\0\x80\xFD[`\0\x825`~\x19\x836\x03\x01\x81\x12aL\xEDW`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x825`^\x19\x836\x03\x01\x81\x12aL\xEDW`\0\x80\xFD\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 \xD2\x80\x1A\xC5\xE3\x14\xD7D\xED\x19x\x15\xF9\xBA\x11\x9B\xEB\x82\x11D\x11\xDD\xF3a\xBF\xC4\xC5\xD3v\xB5\xE5\xAFdsolcC\0\x08\r\x003";
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
        ///Calls the contract's `initialize` (0x30c47d8e) function
        pub fn initialize(
            &self,
            pauser_registry: ::ethers::core::types::Address,
            initial_owner: ::ethers::core::types::Address,
            updater: ::ethers::core::types::Address,
            allow_non_root_init: bool,
            rolldown: ::ethers::core::types::Address,
            chain_id: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [48, 196, 125, 142],
                    (
                        pauser_registry,
                        initial_owner,
                        updater,
                        allow_non_root_init,
                        rolldown,
                        chain_id,
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
        ///Calls the contract's `processEigenOpUpdate` (0x34fadbea) function
        pub fn process_eigen_op_update(
            &self,
            task: OpTask,
            task_response: OpTaskResponse,
            non_signer_stakes_and_signature: NonSignerStakesAndSignature,
            operator_state_info: OperatorStateInfo,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [52, 250, 219, 234],
                    (
                        task,
                        task_response,
                        non_signer_stakes_and_signature,
                        operator_state_info,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `processEigenRdUpdate` (0xed39e502) function
        pub fn process_eigen_rd_update(
            &self,
            task: RdTask,
            task_response: RdTaskResponse,
            non_signer_stakes_and_signature: NonSignerStakesAndSignature,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [237, 57, 229, 2],
                    (task, task_response, non_signer_stakes_and_signature),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `processEigenReinit` (0x2635e74e) function
        pub fn process_eigen_reinit(
            &self,
            task: OpTask,
            operator_state_info: OperatorStateInfo,
            merkle_roots: ::std::vec::Vec<[u8; 32]>,
            ranges: ::std::vec::Vec<Range>,
            last_batch_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [38, 53, 231, 78],
                    (
                        task,
                        operator_state_info,
                        merkle_roots,
                        ranges,
                        last_batch_id,
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
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address,address,bool,address,uint8)` and selector `0x30c47d8e`
    #[derive(
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
        abi = "initialize(address,address,address,bool,address,uint8)"
    )]
    pub struct InitializeCall {
        pub pauser_registry: ::ethers::core::types::Address,
        pub initial_owner: ::ethers::core::types::Address,
        pub updater: ::ethers::core::types::Address,
        pub allow_non_root_init: bool,
        pub rolldown: ::ethers::core::types::Address,
        pub chain_id: u8,
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
    ///Container type for all input parameters for the `processEigenOpUpdate` function with signature `processEigenOpUpdate((uint32,uint32,uint32,bytes,uint32,bytes,uint32),(uint32,bytes32,bytes32),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]),(bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]))` and selector `0x34fadbea`
    #[derive(
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
        abi = "processEigenOpUpdate((uint32,uint32,uint32,bytes,uint32,bytes,uint32),(uint32,bytes32,bytes32),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]),(bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]))"
    )]
    pub struct ProcessEigenOpUpdateCall {
        pub task: OpTask,
        pub task_response: OpTaskResponse,
        pub non_signer_stakes_and_signature: NonSignerStakesAndSignature,
        pub operator_state_info: OperatorStateInfo,
    }
    ///Container type for all input parameters for the `processEigenRdUpdate` function with signature `processEigenRdUpdate((uint32,uint8,uint32,uint32,uint32,bytes,uint32),(uint32,bytes32,uint8,uint32,bytes32,uint256,uint256,address),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))` and selector `0xed39e502`
    #[derive(
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
        abi = "processEigenRdUpdate((uint32,uint8,uint32,uint32,uint32,bytes,uint32),(uint32,bytes32,uint8,uint32,bytes32,uint256,uint256,address),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))"
    )]
    pub struct ProcessEigenRdUpdateCall {
        pub task: RdTask,
        pub task_response: RdTaskResponse,
        pub non_signer_stakes_and_signature: NonSignerStakesAndSignature,
    }
    ///Container type for all input parameters for the `processEigenReinit` function with signature `processEigenReinit((uint32,uint32,uint32,bytes,uint32,bytes,uint32),(bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]),bytes32[],(uint256,uint256)[],uint32)` and selector `0x2635e74e`
    #[derive(
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
        abi = "processEigenReinit((uint32,uint32,uint32,bytes,uint32,bytes,uint32),(bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]),bytes32[],(uint256,uint256)[],uint32)"
    )]
    pub struct ProcessEigenReinitCall {
        pub task: OpTask,
        pub operator_state_info: OperatorStateInfo,
        pub merkle_roots: ::std::vec::Vec<[u8; 32]>,
        pub ranges: ::std::vec::Vec<Range>,
        pub last_batch_id: u32,
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
