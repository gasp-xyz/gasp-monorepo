pub use finalizer_task_manager::*;
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
pub mod finalizer_task_manager {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("THRESHOLD_DENOMINATOR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "THRESHOLD_DENOMINATOR",
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
                    ::std::borrow::ToOwned::to_owned("_validateTaskResponse"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "_validateTaskResponse",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("taskHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("taskType"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum IFinalizerTaskManager.TaskType",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "referenceTaskIndex",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("taskCreatedBlock"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("aggregator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("aggregator"),
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
                    ::std::borrow::ToOwned::to_owned("allTaskHashes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allTaskHashes"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum IFinalizerTaskManager.TaskType",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("allTaskResponses"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allTaskResponses"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum IFinalizerTaskManager.TaskType",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("blsApkRegistry"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("blsApkRegistry"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IBLSApkRegistry"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("blsSignatureChecker"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "blsSignatureChecker",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract BLSSignatureChecker",
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
                    ::std::borrow::ToOwned::to_owned("cancelPendingTasks"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("cancelPendingTasks"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("chainRdBatchNonce"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("chainRdBatchNonce"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
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
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumbers"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "referenceBlockNumber",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
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
                    ::std::borrow::ToOwned::to_owned("createNewOpTask"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createNewOpTask"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "quorumThresholdPercentage",
                                    ),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("createNewRdTask"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createNewRdTask"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("chainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("batchId"),
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
                    ::std::borrow::ToOwned::to_owned("dummyForOperatorStateInfoType"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "dummyForOperatorStateInfoType",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_operatorStateInfo",
                                    ),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("dummyForQuorumStakeTotalsType"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "dummyForQuorumStakeTotalsType",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_quorumStakeTotals",
                                    ),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("forceCancelPendingTasks"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "forceCancelPendingTasks",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("forceCreateNewOpTask"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "forceCreateNewOpTask",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "quorumThresholdPercentage",
                                    ),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("forceRespondToOpTask"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "forceRespondToOpTask",
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
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("generator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("generator"),
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
                    ::std::borrow::ToOwned::to_owned("getBatchOperatorFromId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getBatchOperatorFromId",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "registryCoordinator",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IRegistryCoordinator",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operatorIds"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getBatchOperatorId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getBatchOperatorId"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "registryCoordinator",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IRegistryCoordinator",
                                        ),
                                    ),
                                },
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operatorIds"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getCheckSignaturesIndices"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getCheckSignaturesIndices",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "registryCoordinator",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IRegistryCoordinator",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "referenceBlockNumber",
                                    ),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "nonSignerOperatorIds",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
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
                                            "struct OperatorStateRetriever.CheckSignaturesIndices",
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
                    ::std::borrow::ToOwned::to_owned("getOperatorState"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getOperatorState"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "registryCoordinator",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IRegistryCoordinator",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumbers"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct OperatorStateRetriever.Operator[][]",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getOperatorState"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "registryCoordinator",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IRegistryCoordinator",
                                        ),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct OperatorStateRetriever.Operator[][]",
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
                    ::std::borrow::ToOwned::to_owned("getQuorumBitmapsAtBlockNumber"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getQuorumBitmapsAtBlockNumber",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "registryCoordinator",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IRegistryCoordinator",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operatorIds"),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("idToTaskStatus"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("idToTaskStatus"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum IFinalizerTaskManager.TaskType",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum IFinalizerTaskManager.TaskStatus",
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
                                    name: ::std::borrow::ToOwned::to_owned("_aggregator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_generator"),
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
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_blsSignatureCheckerAddress",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_taskResponseWindowBlock",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_operatorStateRetrieverExtended",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("isTaskPending"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isTaskPending"),
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
                    ::std::borrow::ToOwned::to_owned(
                        "lastCompletedOpTaskCompletedBlock",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "lastCompletedOpTaskCompletedBlock",
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
                    ::std::borrow::ToOwned::to_owned("lastCompletedOpTaskCreatedBlock"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "lastCompletedOpTaskCreatedBlock",
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
                    ::std::borrow::ToOwned::to_owned("lastCompletedOpTaskNum"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "lastCompletedOpTaskNum",
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
                    ::std::borrow::ToOwned::to_owned("lastCompletedOpTaskQuorumNumbers"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "lastCompletedOpTaskQuorumNumbers",
                            ),
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
                    ::std::borrow::ToOwned::to_owned(
                        "lastCompletedOpTaskQuorumThresholdPercentage",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "lastCompletedOpTaskQuorumThresholdPercentage",
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
                        "lastCompletedRdTaskCompletedBlock",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "lastCompletedRdTaskCompletedBlock",
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
                    ::std::borrow::ToOwned::to_owned("lastCompletedRdTaskCreatedBlock"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "lastCompletedRdTaskCreatedBlock",
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
                    ::std::borrow::ToOwned::to_owned("lastCompletedRdTaskNum"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "lastCompletedRdTaskNum",
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
                    ::std::borrow::ToOwned::to_owned("lastOpTaskCreatedBlock"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "lastOpTaskCreatedBlock",
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
                    ::std::borrow::ToOwned::to_owned("lastRdTaskCreatedBlock"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "lastRdTaskCreatedBlock",
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
                    ::std::borrow::ToOwned::to_owned("latestOpTaskNum"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("latestOpTaskNum"),
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
                    ::std::borrow::ToOwned::to_owned("latestRdTaskNum"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("latestRdTaskNum"),
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
                    ::std::borrow::ToOwned::to_owned("operatorStateRetrieverExtended"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "operatorStateRetrieverExtended",
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
                    ::std::borrow::ToOwned::to_owned("operatorsStateInfoHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "operatorsStateInfoHash",
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
                    ::std::borrow::ToOwned::to_owned("respondToOpTask"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("respondToOpTask"),
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
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("respondToRdTask"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("respondToRdTask"),
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
                    ::std::borrow::ToOwned::to_owned("setAggregator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setAggregator"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_aggregator"),
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
                    ::std::borrow::ToOwned::to_owned("setGenerator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setGenerator"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_generator"),
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
                    ::std::borrow::ToOwned::to_owned("stakeRegistry"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("stakeRegistry"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IStakeRegistry"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("taskResponseWindowBlock"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "taskResponseWindowBlock",
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
                    ::std::borrow::ToOwned::to_owned("updateBlsSignatureCheckerAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateBlsSignatureCheckerAddress",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_blsSignatureCheckerAddress",
                                    ),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AggregatorUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AggregatorUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("aggregatorAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "BLSSignatureCheckerAddressUpdated",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BLSSignatureCheckerAddressUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "blsSignatureCheckerAddress",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("GeneratorUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("GeneratorUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("generatorAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
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
                    ::std::borrow::ToOwned::to_owned("NewOpTaskCreated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("NewOpTaskCreated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("taskIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
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
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NewOpTaskForceCreated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NewOpTaskForceCreated",
                            ),
                            inputs: ::std::vec![],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NewRdTaskCreated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("NewRdTaskCreated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("taskIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
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
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OpTaskCancelled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("OpTaskCancelled"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("taskIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OpTaskCompleted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("OpTaskCompleted"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("taskIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("taskResponse"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ],
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OpTaskForceCompleted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OpTaskForceCompleted",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("taskIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("taskResponse"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ],
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OpTaskResponded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("OpTaskResponded"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("taskIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("taskResponse"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ],
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "taskResponseMetadata",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
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
                    ::std::borrow::ToOwned::to_owned("RdTaskCancelled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RdTaskCancelled"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("taskIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RdTaskCompleted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RdTaskCompleted"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("taskIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
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
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RdTaskResponded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RdTaskResponded"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("taskIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
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
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "taskResponseMetadata",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
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
                    ::std::borrow::ToOwned::to_owned("StaleStakesForbiddenUpdate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "StaleStakesForbiddenUpdate",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
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
    pub static FINALIZERTASKMANAGER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa^\x8D\x80b\0\0!`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x03\xC5W`\x005`\xE0\x1C\x80c]\xF4YF\x11a\x01\xFFW\x80c\xA9\x07{\xFB\x11a\x01\x1AW\x80c\xDF\\\xF7#\x11a\0\xADW\x80c\xF5d\x0C\xF8\x11a\0|W\x80c\xF5d\x0C\xF8\x14a\x08\xFCW\x80c\xF9\x12\n\xF6\x14a\t\x0FW\x80c\xFA\xBC\x1C\xBC\x14a\t\"W\x80c\xFD\xC1]\xE8\x14a\t5W`\0\x80\xFD[\x80c\xDF\\\xF7#\x14a\x08\xC9W\x80c\xE7\x0C&#\x14a\x08\xD1W\x80c\xEF\x02DX\x14a\x08\xE1W\x80c\xF2\xFD\xE3\x8B\x14a\x08\xE9W`\0\x80\xFD[\x80c\xCAP\xE5j\x11a\0\xE9W\x80c\xCAP\xE5j\x14a\x08iW\x80c\xCE\xFD\xC1\xD4\x14a\x08\x82W\x80c\xD77\xD9\xF2\x14a\x08\xA3W\x80c\xDECH8\x14a\x08\xB6W`\0\x80\xFD[\x80c\xA9\x07{\xFB\x14a\x07\xFFW\x80c\xAD\xFC\xB0H\x14a\x08\x12W\x80c\xB1\xED\xC8\xB4\x14a\x08\x1BW\x80c\xBF#\x15\xED\x14a\x08.W`\0\x80\xFD[\x80cz\xFA\x1E\xED\x11a\x01\x92W\x80c\x8C\x82\xAF^\x11a\x01aW\x80c\x8C\x82\xAF^\x14a\x07\xA8W\x80c\x8D\xA5\xCB[\x14a\x07\xC0W\x80c\x8F\xC8r\x9A\x14a\x07\xD1W\x80c\xA6\x95c\xA9\x14a\x07\xE8W`\0\x80\xFD[\x80cz\xFA\x1E\xED\x14a\x07XW\x80cz\xFD\xD5K\x14a\x07kW\x80c\x83\x80\xAC\xBD\x14a\x07\x82W\x80c\x88o\x11\x95\x14a\x07\x95W`\0\x80\xFD[\x80cn\x12_\xF4\x11a\x01\xCEW\x80cn\x12_\xF4\x14a\x07\tW\x80cn\xFBF6\x14a\x07\x1CW\x80cqP\x18\xA6\x14a\x07=W\x80cr1\x14\xAB\x14a\x07EW`\0\x80\xFD[\x80c]\xF4YF\x14a\x06\xE9W\x80c` /\xC0\x14a\x06\xF1W\x80ch0H5\x14a\x06\xF9W\x80cm\x14\xA9\x87\x14a\x07\x01W`\0\x80\xFD[\x80c5c\xB0\xD1\x11a\x02\xEFW\x80cMzq\x16\x11a\x02\x82W\x80cY\\jg\x11a\x02QW\x80cY\\jg\x14a\x06\x96W\x80cZ\xC8j\xB7\x14a\x06\x9EW\x80c\\\x15Vb\x14a\x06\xC1W\x80c\\\x97Z\xBB\x14a\x06\xE1W`\0\x80\xFD[\x80cMzq\x16\x14a\x06AW\x80cOs\x9Ft\x14a\x06QW\x80cSz))\x14a\x06qW\x80cT\xD1'\xDE\x14a\x06\x88W`\0\x80\xFD[\x80cAx\x9DW\x11a\x02\xBEW\x80cAx\x9DW\x14a\x05\xE6W\x80cE&[z\x14a\x05\xFDW\x80cJ|~K\x14a\x06\x0EW\x80cM+W\xFE\x14a\x06!W`\0\x80\xFD[\x80c5c\xB0\xD1\x14a\x05\x88W\x80c6\xF7\x8E\xD8\x14a\x05\xA8W\x80c=r\"\xEB\x14a\x05\xBCW\x80c=\x9F\xB0\x0C\x14a\x05\xD3W`\0\x80\xFD[\x80c\x1A\xC2r\x97\x11a\x03gW\x80c&\x12\xFC\xEF\x11a\x036W\x80c&\x12\xFC\xEF\x14a\x05)W\x80c(0\xE8\xF9\x14a\x05<W\x80c,\x11\x01\xDA\x14a\x05QW\x80c1\xB3k\xD9\x14a\x05hW`\0\x80\xFD[\x80c\x1A\xC2r\x97\x14a\x04\xB0W\x80c\x1C\x17\x8E\x9C\x14a\x04\xDBW\x80c!\xE7\x80b\x14a\x05\x06W\x80c$Z{\xFC\x14a\x05\x0EW`\0\x80\xFD[\x80c\x0E\xE0\xFD\xBD\x11a\x03\xA3W\x80c\x0E\xE0\xFD\xBD\x14a\x044W\x80c\x10\xD6z/\x14a\x04QW\x80c\x13d9\xDD\x14a\x04dW\x80c\x13\xF8\x15\xED\x14a\x04wW`\0\x80\xFD[\x80c\t\x80\xF1\xEC\x14a\x03\xCAW\x80c\t\xEC\x19\x06\x14a\x03\xF9W\x80c\x0E\xBF*\x81\x14a\x04\x0EW[`\0\x80\xFD[`\xA3Ta\x03\xDF\x90a\x01\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x04\x0Ca\x04\x076`\x04aA\x80V[a\tHV[\0[a\x03\xDFa\x04\x1C6`\x04aA\xEAV[`\xA1` R`\0\x90\x81R`@\x90 Tc\xFF\xFF\xFF\xFF\x16\x81V[`\xA3Ta\x04A\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x03\xF0V[a\x04\x0Ca\x04_6`\x04aB\x1AV[a\n\xCBV[a\x04\x0Ca\x04r6`\x04aB7V[a\x0B\x84V[a\x04\xA2a\x04\x856`\x04aB|V[`\x99` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[`@Q\x90\x81R` \x01a\x03\xF0V[a\x04\xA2a\x04\xBE6`\x04aB|V[`\x9A` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[`\x97Ta\x04\xEE\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03\xF0V[a\x04\x0Ca\x0C\xC3V[`\x9ETa\x04\xEE\x90d\x01\0\0\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x04\x0Ca\x0576`\x04aB\xB3V[a\rGV[a\x05Da\x0F\xEEV[`@Qa\x03\xF0\x91\x90aC\x1CV[`\xA3Ta\x03\xDF\x90`\x01`H\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x05{a\x05v6`\x04aC\xE3V[a\x10|V[`@Qa\x03\xF0\x91\x90aD\xD1V[a\x05\x9Ba\x05\x966`\x04aD\xE4V[a\x11\x98V[`@Qa\x03\xF0\x91\x90aF?V[`\xA0Ta\x04A\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[`\xA3Ta\x03\xDF\x90`\x01`h\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`\xA0Ta\x04\xEE\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x98Ta\x03\xDF\x90`\x01`\xC0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x04\x0Ca\x06\x0B6`\x04aFRV[PV[a\x04\x0Ca\x06\x1C6`\x04aB\x1AV[a\x160V[a\x064a\x06/6`\x04aF\xF2V[a\x16\x8DV[`@Qa\x03\xF0\x91\x90aGAV[`\x9CTa\x03\xDF\x90c\xFF\xFF\xFF\xFF\x16\x81V[a\x06da\x06_6`\x04aG\xD6V[a\x17\xA2V[`@Qa\x03\xF0\x91\x90aI$V[`\x9CTa\x03\xDF\x90`\x01``\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x04\x0Ca\x06\x0B6`\x04aI\xA2V[a\x04\x0Ca\x1E\xCAV[a\x04Aa\x06\xAC6`\x04aI\xDDV[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[a\x06\xD4a\x06\xCF6`\x04aJ\0V[a\x1F\x91V[`@Qa\x03\xF0\x91\x90aJcV[`fTa\x04\xA2V[a\x04\xEEa!YV[a\x04\x0Ca!\xCCV[a\x04\xEEa!\xD4V[a\x04\xEEa\"\x1EV[a\x04\x0Ca\x07\x176`\x04aJ\x9BV[a\"hV[a\x07/a\x07*6`\x04aN V[a\"\xF3V[`@Qa\x03\xF0\x92\x91\x90aN\xE0V[a\x04\x0Ca#\x93V[a\x04\x0Ca\x07S6`\x04aB\x1AV[a#\xA5V[`\x9FTa\x04\xEE\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x98Ta\x03\xDF\x90`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`\x98Ta\x04\xEE\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`eTa\x04\xEE\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x9CTa\x03\xDF\x90d\x01\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x04\xEEV[`\x9CTa\x03\xDF\x90`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`\x98Ta\x03\xDF\x90`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x04\x0Ca\x08\r6`\x04aO)V[a#\xFBV[a\x04\xA2`\xA2T\x81V[a\x04\x0Ca\x08)6`\x04aO\x9FV[a(\xCDV[a\x08\\a\x08<6`\x04aB|V[`\x9B` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[`@Qa\x03\xF0\x91\x90aP\x06V[`\xA3Ta\x03\xDF\x90e\x01\0\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x08\x95a\x08\x906`\x04aP.V[a+AV[`@Qa\x03\xF0\x92\x91\x90aPeV[a\x04\x0Ca\x08\xB16`\x04aP\x86V[a,\xD3V[a\x04\x0Ca\x08\xC46`\x04aP\xF1V[a/sV[a\x04\xEEa1,V[`\x9ETa\x03\xDF\x90c\xFF\xFF\xFF\xFF\x16\x81V[a\x04\xA2`d\x81V[a\x04\x0Ca\x08\xF76`\x04aB\x1AV[a1vV[a\x04\x0Ca\t\n6`\x04aJ\x9BV[a1\xECV[a\x04\x0Ca\t\x1D6`\x04aB\x1AV[a2GV[a\x04\x0Ca\t06`\x04aB7V[a2\xABV[a\x04\x0Ca\tC6`\x04aB\x1AV[a4\x07V[a\tPa4]V[a\t\x9C\x82`@Q` \x01a\td\x91\x90aR\x1BV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x91\x82\x01 \x90`\0\x90a\t\x8C\x90\x85\x01\x85aS\x0CV[a\x08)`@\x87\x01` \x88\x01aS\x0CV[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`@\x80Q`\x80\x81\x01\x82Rc\xFF\xFF\xFF\xFFC\x16\x81R`\0` \x80\x83\x01\x82\x90R\x84\x81\x01Q\x93\x83\x01\x93\x90\x93R\x83Q``\x83\x01R\x91a\n\"\x90\x83\x90a\t\xF2\x90\x87\x01\x87aS\x0CV[\x86\x84`@Q` \x01a\n\x05\x92\x91\x90aS\x99V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\x04a4\xB7V[a\n0\x84`@\x015\x86a5\x8BV[a\n=` \x85\x01\x85aS\x0CV[c\xFF\xFF\xFF\xFF\x16\x7F\xFF)\x08H=t\xB6\xB7\0S\xDDG2`\xAC\xF1\xB0\x9E\x0B\xA0x\x1B\xF9A\0\xBB\x82wX\x17I\xDE\x85`@Qa\nr\x91\x90aS\xB9V[`@Q\x80\x91\x03\x90\xA2a\n\x87` \x85\x01\x85aS\x0CV[c\xFF\xFF\xFF\xFF\x16\x7F\xDF\"\xF3U\x8EHA\xB6=w\x17\x95F\xB3\xEA\xE6>N4;\xBEu'F\xB0\x93\x16+\xC5&\xBEL\x85`@Qa\n\xBC\x91\x90aS\xB9V[`@Q\x80\x91\x03\x90\xA2PPPPPV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\x1EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0BB\x91\x90aS\xC7V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0B{W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0Br\x90aS\xE4V[`@Q\x80\x91\x03\x90\xFD[a\x06\x0B\x81a6eV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xCCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xF0\x91\x90aT.V[a\x0C\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0Br\x90aTKV[`fT\x81\x81\x16\x14a\x0C\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0BrV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`\x9FT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0C\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0Br\x90aT\x93V[`\xA0T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15\x15`\x01\x14a\r=W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01RnNo task pending`\x88\x1B`D\x82\x01R`d\x01a\x0BrV[a\rEa7\\V[V[`\x9FT`\x01`\x01`\xA0\x1B\x03\x163\x14a\rqW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0Br\x90aT\x93V[`\xA0T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\r\xC2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RsTask already pending``\x1B`D\x82\x01R`d\x01a\x0BrV[`\x9CT`\x01``\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15\x80\x15\x90a\r\xDFWPC\x15\x15[a\x0E\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x13\xDC\x08\x14\xDD\x18]\x19H\x1D[\x9A[\x9A]`\x8A\x1B`D\x82\x01R`d\x01a\x0BrV[`\x98T`@\x80Qa\x01\0\x81\x01\x82R`\x01`\xE0\x1B\x90\x92\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x80\x84R`\x01`\x01`@\x1B\x03\x86\x16` \x85\x01R\x84\x82\x16\x92\x84\x01\x92\x90\x92RC\x81\x16``\x84\x01R`\x9CT`\x01`@\x1B\x81\x04\x82\x16`\x80\x85\x01R`\x01``\x1B\x90\x04\x16`\xA0\x83\x01R`\x9D\x80T\x91\x92`\0\x92\x90\x91`\xC0\x83\x01\x91a\x0E\x96\x90aT\xB2V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0E\xC2\x90aT\xB2V[\x80\x15a\x0F\x0FW\x80`\x1F\x10a\x0E\xE4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0F\x0FV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0E\xF2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x9ETc\xFF\xFF\xFF\xFF\x16` \x91\x82\x01R`@Q\x91\x92Pa\x0F^\x91`\x01\x91\x85\x91a\x0FA\x91\x86\x91\x01aT\xE6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\x01a9KV[C`\x9C`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81c\xFF\xFF\xFF\xFF\x16\x7F\x92[\x9C<\xD9\xE2\xCB\x02v\xA1w\x93\xEBh\x0E\xF1\x9Amq\xDC\x9DU(Hv\xF2\x98\xA0\xD8fcV\x82`@Qa\x0F\xB5\x91\x90aT\xE6V[`@Q\x80\x91\x03\x90\xA2a\x0F\xC8\x82`\x01aU\xA4V[`\x98`\x1Ca\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPPV[`\x9D\x80Ta\x0F\xFB\x90aT\xB2V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x10'\x90aT\xB2V[\x80\x15a\x10tW\x80`\x1F\x10a\x10IWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x10tV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x10WW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[``\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x10\x97Wa\x10\x97aC/V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x10\xC0W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82Q\x81\x10\x15a\x11\x91W\x83`\x01`\x01`\xA0\x1B\x03\x16c\x13T*N\x84\x83\x81Q\x81\x10a\x10\xF0Wa\x10\xF0aU\xCCV[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x11#\x91\x90`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11@W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11d\x91\x90aU\xE2V[\x82\x82\x81Q\x81\x10a\x11vWa\x11vaU\xCCV[` \x90\x81\x02\x91\x90\x91\x01\x01Ra\x11\x8A\x81aU\xFBV[\x90Pa\x10\xC6V[P\x92\x91PPV[```\0\x84`\x01`\x01`\xA0\x1B\x03\x16ch0H5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xDAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xFE\x91\x90aS\xC7V[\x90P`\0\x85`\x01`\x01`\xA0\x1B\x03\x16c\x9E\x99#\xC2`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12@W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12d\x91\x90aS\xC7V[\x90P`\0\x86`\x01`\x01`\xA0\x1B\x03\x16c]\xF4YF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xA6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xCA\x91\x90aS\xC7V[\x90P`\0\x86Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x12\xE7Wa\x12\xE7aC/V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x13\x1AW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x13\x05W\x90P[P\x90P`\0[\x87Q\x81\x10\x15a\x16\"W`\0\x88\x82\x81Q\x81\x10a\x13=Wa\x13=aU\xCCV[\x01` \x01Q`@Qc\x89\x02bE`\xE0\x1B\x81R`\xF8\x91\x90\x91\x1C`\x04\x82\x01\x81\x90Rc\xFF\xFF\xFF\xFF\x8A\x16`$\x83\x01R\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x87\x16\x90c\x89\x02bE\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\x9EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x13\xC6\x91\x90\x81\x01\x90aV\x14V[\x90P\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x13\xE1Wa\x13\xE1aC/V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x14,W\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a\x13\xFFW\x90P[P\x84\x84\x81Q\x81\x10a\x14?Wa\x14?aU\xCCV[` \x02` \x01\x01\x81\x90RP`\0[\x81Q\x81\x10\x15a\x16\x0CW`@Q\x80``\x01`@R\x80\x87`\x01`\x01`\xA0\x1B\x03\x16cG\xB3\x14\xE8\x85\x85\x81Q\x81\x10a\x14\x82Wa\x14\x82aU\xCCV[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14\xA8\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\xC5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\xE9\x91\x90aS\xC7V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x83\x81Q\x81\x10a\x15\tWa\x15\taU\xCCV[` \x02` \x01\x01Q\x81R` \x01\x89`\x01`\x01`\xA0\x1B\x03\x16c\xFA(\xC6'\x85\x85\x81Q\x81\x10a\x157Wa\x157aU\xCCV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x04\x81\x01\x91\x90\x91R`\xFF\x88\x16`$\x82\x01Rc\xFF\xFF\xFF\xFF\x8F\x16`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\x93W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xB7\x91\x90aV\xBBV[`\x01`\x01``\x1B\x03\x16\x81RP\x85\x85\x81Q\x81\x10a\x15\xD5Wa\x15\xD5aU\xCCV[` \x02` \x01\x01Q\x82\x81Q\x81\x10a\x15\xEEWa\x15\xEEaU\xCCV[` \x02` \x01\x01\x81\x90RP\x80\x80a\x16\x04\x90aU\xFBV[\x91PPa\x14MV[PPP\x80\x80a\x16\x1A\x90aU\xFBV[\x91PPa\x13 V[P\x93PPPP[\x93\x92PPPV[a\x168a4]V[`\x9F\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F`\xF5\xACH\xA1?\x8B[\xF4\xB2\x13\xDE\x19\r\xD2\xDE%\x92\xC9\xF6\xF5\xAC}\xC1N=rk\x95\xDE\xD2\xDA\x90` \x01[`@Q\x80\x91\x03\x90\xA1PV[``\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16\xA8Wa\x16\xA8aC/V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x16\xD1W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82Q\x81\x10\x15a\x11\x91W\x83`\x01`\x01`\xA0\x1B\x03\x16c)k\xB0d\x84\x83\x81Q\x81\x10a\x17\x01Wa\x17\x01aU\xCCV[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17'\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17h\x91\x90aS\xC7V[\x82\x82\x81Q\x81\x10a\x17zWa\x17zaU\xCCV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01Ra\x17\x9B\x81aU\xFBV[\x90Pa\x16\xD7V[a\x17\xCD`@Q\x80`\x80\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`\0\x87`\x01`\x01`\xA0\x1B\x03\x16ch0H5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x181\x91\x90aS\xC7V[\x90Pa\x18^`@Q\x80`\x80\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`@Qca\xC8\xA1/`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90c\xC3\x91B^\x90a\x18\x8E\x90\x8B\x90\x89\x90\x89\x90`\x04\x01aV\xD6V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\xABW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x18\xD3\x91\x90\x81\x01\x90aW V[\x81R`@Qc@\xE0:\x81`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\x81\xC0u\x02\x90a\x19\x05\x90\x8B\x90\x8B\x90\x8B\x90`\x04\x01aW\xAEV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\"W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x19J\x91\x90\x81\x01\x90aW V[`@\x82\x01R\x85`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19gWa\x19gaC/V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x19\x9AW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x19\x85W\x90P[P``\x82\x01R`\0[`\xFF\x81\x16\x87\x11\x15a\x1D\xDBW`\0\x85`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19\xC8Wa\x19\xC8aC/V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x19\xF1W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x83``\x01Q\x83`\xFF\x16\x81Q\x81\x10a\x1A\x0BWa\x1A\x0BaU\xCCV[` \x02` \x01\x01\x81\x90RP`\0[\x86\x81\x10\x15a\x1C\xDBW`\0\x8C`\x01`\x01`\xA0\x1B\x03\x16c\x04\xECcQ\x8A\x8A\x85\x81\x81\x10a\x1ADWa\x1ADaU\xCCV[\x90P` \x02\x015\x8E\x88`\0\x01Q\x86\x81Q\x81\x10a\x1AbWa\x1AbaU\xCCV[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1A\x9F\x93\x92\x91\x90\x92\x83Rc\xFF\xFF\xFF\xFF\x91\x82\x16` \x84\x01R\x16`@\x82\x01R``\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\xBCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xE0\x91\x90aW\xCEV[\x90P\x80`\x01`\x01`\xC0\x1B\x03\x16`\0\x03a\x1B\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R\x7FOperatorStateRetriever.getCheckS`D\x82\x01R\x7FignaturesIndices: operator must `d\x82\x01R\x7Fbe registered at blocknumber\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x0BrV[\x8A\x8A\x85`\xFF\x16\x81\x81\x10a\x1B\x9CWa\x1B\x9CaU\xCCV[`\x01`\x01`\x01`\xC0\x1B\x03\x85\x16\x91\x90\x93\x015`\xF8\x1C\x1C\x82\x16\x90\x91\x03\x90Pa\x1C\xC8W\x85`\x01`\x01`\xA0\x1B\x03\x16c\xDD\x98F\xB9\x8A\x8A\x85\x81\x81\x10a\x1B\xDDWa\x1B\xDDaU\xCCV[\x90P` \x02\x015\x8D\x8D\x88`\xFF\x16\x81\x81\x10a\x1B\xF9Wa\x1B\xF9aU\xCCV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x87\x90\x1B\x16\x81R`\x04\x81\x01\x94\x90\x94R\x91\x90\x91\x015`\xF8\x1C`$\x83\x01RPc\xFF\xFF\xFF\xFF\x8F\x16`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1COW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Cs\x91\x90aW\xF7V[\x85``\x01Q\x85`\xFF\x16\x81Q\x81\x10a\x1C\x8CWa\x1C\x8CaU\xCCV[` \x02` \x01\x01Q\x84\x81Q\x81\x10a\x1C\xA5Wa\x1C\xA5aU\xCCV[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x82a\x1C\xC4\x81aU\xFBV[\x93PP[P\x80a\x1C\xD3\x81aU\xFBV[\x91PPa\x1A\x19V[P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1C\xF6Wa\x1C\xF6aC/V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1D\x1FW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a\x1D\xA0W\x84``\x01Q\x84`\xFF\x16\x81Q\x81\x10a\x1DFWa\x1DFaU\xCCV[` \x02` \x01\x01Q\x81\x81Q\x81\x10a\x1D_Wa\x1D_aU\xCCV[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a\x1DyWa\x1DyaU\xCCV[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x1D\x98\x81aU\xFBV[\x91PPa\x1D%V[P\x80\x84``\x01Q\x84`\xFF\x16\x81Q\x81\x10a\x1D\xBBWa\x1D\xBBaU\xCCV[` \x02` \x01\x01\x81\x90RPPP\x80\x80a\x1D\xD3\x90aX\x14V[\x91PPa\x19\xA3V[P`\0\x89`\x01`\x01`\xA0\x1B\x03\x16c]\xF4YF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\x1CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E@\x91\x90aS\xC7V[`@Qc5IR\xA3`\xE2\x1B\x81R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xD5%J\x8C\x90a\x1Es\x90\x8B\x90\x8B\x90\x8E\x90`\x04\x01aX3V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\x90W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1E\xB8\x91\x90\x81\x01\x90aW V[` \x83\x01RP\x98\x97PPPPPPPPV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\x12W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F6\x91\x90aT.V[a\x1FRW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0Br\x90aTKV[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[```\0\x84`\x01`\x01`\xA0\x1B\x03\x16c\xC3\x91B^\x84\x86`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1F\xC3\x92\x91\x90aX]V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\xE0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra \x08\x91\x90\x81\x01\x90aW V[\x90P`\0\x84Q`\x01`\x01`@\x1B\x03\x81\x11\x15a %Wa %aC/V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a NW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x85Q\x81\x10\x15a!OW\x86`\x01`\x01`\xA0\x1B\x03\x16c\x04\xECcQ\x87\x83\x81Q\x81\x10a ~Wa ~aU\xCCV[` \x02` \x01\x01Q\x87\x86\x85\x81Q\x81\x10a \x99Wa \x99aU\xCCV[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a \xD6\x93\x92\x91\x90\x92\x83Rc\xFF\xFF\xFF\xFF\x91\x82\x16` \x84\x01R\x16`@\x82\x01R``\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a \xF3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\x17\x91\x90aW\xCEV[`\x01`\x01`\xC0\x1B\x03\x16\x82\x82\x81Q\x81\x10a!2Wa!2aU\xCCV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a!G\x81aU\xFBV[\x91PPa TV[P\x95\x94PPPPPV[`\x97T`@\x80Qc.\xFA,\xA3`\xE1\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c]\xF4YF\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a!\xA3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\xC7\x91\x90aS\xC7V[\x90P\x90V[a\x0C\xEDa4]V[`\x97T`@\x80Qch0H5`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91ch0H5\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a!\xA3W=`\0\x80>=`\0\xFD[`\x97T`@\x80Qcm\x14\xA9\x87`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cm\x14\xA9\x87\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a!\xA3W=`\0\x80>=`\0\xFD[`\x9FT`\x01`\x01`\xA0\x1B\x03\x163\x14a\"\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0Br\x90aT\x93V[`\xA0T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\"\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RsTask already pending``\x1B`D\x82\x01R`d\x01a\x0BrV[a\"\xEE\x83\x83\x83a:%V[PPPV[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`\x97T`@Qc7}\xA3\x1B`\xE1\x1B\x81R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cn\xFBF6\x90a#@\x90\x8A\x90\x8A\x90\x8A\x90\x8A\x90\x8A\x90`\x04\x01aY\0V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a#]W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra#\x85\x91\x90\x81\x01\x90aZ`V[\x91P\x91P\x95P\x95\x93PPPPV[a#\x9Ba4]V[a\rE`\0a=\rV[a#\xADa4]V[`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x90\x1AeM\xC80\xC9N\x8A\x12\xC9\xA3\xBC\n\x92\xAC\x11\xB5\xCF(\x04l\xA8\xD1\x90i\x1C\xDA\xF5 \x90\x16\x90` \x01a\x16\x82V[`fT\x15a$KW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FPausable: contract is paused\0\0\0\0`D\x82\x01R`d\x01a\x0BrV[`\x9ETd\x01\0\0\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x163\x14a$\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04\x17WF\x83`\xDC\x1B`D\x82\x01R`d\x01a\x0BrV[6`\0a$\xA5`\xC0\x86\x01\x86aZ\xFCV[\x90\x92P\x90P`\0a$\xBDa\x01\0\x87\x01`\xE0\x88\x01aS\x0CV[\x90P`\xA1`\0a$\xD3``\x88\x01`@\x89\x01aA\xEAV[`\x01`\x01`@\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 Tc\xFF\xFF\xFF\xFF\x16\x15\x80a%FWP`\xA1`\0a%\x0E``\x88\x01`@\x89\x01aA\xEAV[`\x01`\x01`@\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 Tc\xFF\xFF\xFF\xFF\x16a%>`\x80\x87\x01``\x88\x01aS\x0CV[c\xFF\xFF\xFF\xFF\x16\x14[a%\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FchainRdBatchNonce mismatch\0\0\0\0\0\0`D\x82\x01R`d\x01a\x0BrV[a%\xDE\x86`@Q` \x01a%\xA6\x91\x90a[BV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x91\x82\x01 \x90`\x01\x90a%\xCE\x90\x89\x01\x89aS\x0CV[a\x08)`\x80\x8B\x01``\x8C\x01aS\x0CV[`\0\x80a&&\x87`@Q` \x01a%\xF5\x91\x90a\\\xA4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 a&\x1D`\xC0\x8B\x01`\xA0\x8C\x01aS\x0CV[\x88\x88\x88\x88a=_V[\x90\x92P\x90Pa&8` \x89\x01\x89aS\x0CV[c\xFF\xFF\xFF\xFF\x16\x7F\x07H\x0B\xC0\x16'N\xBBe\x0C8G\xD7~\xB2\xA5\xAD\x17W9\x8BIXQ\x1AKu\xDCfo\xAE\xC0\x88\x83`@Qa&o\x92\x91\x90a\\\xB3V[`@Q\x80\x91\x03\x90\xA2\x81\x15a&\xA7Wa&\xA2`\x01a&\x8F` \x8A\x01\x8AaS\x0CV[\x89\x84`@Q` \x01a\n\x05\x92\x91\x90a\\\xB3V[a&\xF3V[a&\xE9`\x01a&\xB9` \x8A\x01\x8AaS\x0CV[\x89\x84`@Q` \x01a&\xCC\x92\x91\x90a\\\xB3V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\x03a4\xB7V[PPPPPPPPV[Fa'\x04``\x89\x01`@\x8A\x01aA\xEAV[`\x01`\x01`@\x1B\x03\x16\x03a'\x93W`@\x80Q\x80\x82\x01\x82R`\xA0\x89\x81\x015\x82R`\xC0\x8A\x015` \x83\x01RT\x91Qb#\xD0\xB5`\xE6\x1B\x81R\x90\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x08\xF4-@\x90a'_\x90`\x80\x8C\x015\x90\x85\x90`\x04\x01a\\\xD5V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a'yW`\0\x80\xFD[PZ\xF1\x15\x80\x15a'\x8DW=`\0\x80>=`\0\xFD[PPPPP[a'\xA3`\x80\x88\x01``\x89\x01aS\x0CV[a'\xAE\x90`\x01aU\xA4V[`\xA1`\0a'\xC2``\x8B\x01`@\x8C\x01aA\xEAV[`\x01`\x01`@\x1B\x03\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x93\x90\x93\x16\x92\x90\x92\x17\x90\x91Ua(\x01\x90\x89\x01\x89aS\x0CV[`\xA3\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16a\x01\0\x02d\xFF\xFF\xFF\xFF\0\x19\x90\x92\x16\x91\x90\x91\x17\x90Ua(2`\x80\x89\x01``\x8A\x01aS\x0CV[`\xA3\x80Tp\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\0\x19\x16e\x01\0\0\0\0\0c\xFF\xFF\xFF\xFF\x93\x84\x16\x02c\xFF\xFF\xFF\xFF`h\x1B\x19\x16\x17`\x01`h\x1BC\x93\x90\x93\x16\x92\x90\x92\x02\x91\x90\x91\x17\x90Ua(\x86` \x88\x01\x88aS\x0CV[c\xFF\xFF\xFF\xFF\x16\x7F\x88M\xD2\xB9\xADwz\xE0\xBC7\xBB\t\xE4>3\x90\xA9\xB9=^\x0C\xAC\xEA\x96r\x94v\x11\x16\xBB\x9A$\x88`@Qa(\xBB\x91\x90a\\\xA4V[`@Q\x80\x91\x03\x90\xA2PPPPPPPPV[`\xA0T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15\x15`\x01\x14a)\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01RnNo task pending`\x88\x1B`D\x82\x01R`d\x01a\x0BrV[`\x99`\0\x84`\x01\x81\x11\x15a)3Wa)3aO\xF0V[`\x01\x81\x11\x15a)DWa)DaO\xF0V[\x81R` \x01\x90\x81R` \x01`\0 `\0\x83c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x84\x14a)\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\n\x8C.md\r\xAD.m\xAC.\x8Cm`\x9B\x1B`D\x82\x01R`d\x01a\x0BrV[`\x01`\x9B`\0\x85`\x01\x81\x11\x15a)\xC6Wa)\xC6aO\xF0V[`\x01\x81\x11\x15a)\xD7Wa)\xD7aO\xF0V[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 c\xFF\xFF\xFF\xFF\x87\x16\x82R\x90\x92R\x90 T`\xFF\x16`\x04\x81\x11\x15a*\x0FWa*\x0FaO\xF0V[\x14a*MW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01RmNot Init state`\x90\x1B`D\x82\x01R`d\x01a\x0BrV[`\0`\x9A\x81\x85`\x01\x81\x11\x15a*dWa*daO\xF0V[`\x01\x81\x11\x15a*uWa*uaO\xF0V[\x81R` \x01\x90\x81R` \x01`\0 `\0\x84c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x14a*\xDBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01Ri\x04\x16\xC7&G\x92\x05&W7`\xB4\x1B`D\x82\x01R`d\x01a\x0BrV[`\x98Ta*\xF5\x90`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x82aU\xA4V[c\xFF\xFF\xFF\xFF\x16Cc\xFF\xFF\xFF\xFF\x16\x11\x15a+;W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x08`$\x82\x01RgToo late`\xC0\x1B`D\x82\x01R`d\x01a\x0BrV[PPPPV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91``\x91\x83\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x84\x81`\0\x81Q\x81\x10a+|Wa+|aU\xCCV[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Qca\xC8\xA1/`\xE1\x1B\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90c\xC3\x91B^\x90a+\xB8\x90\x88\x90\x86\x90`\x04\x01aX]V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+\xD5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra+\xFD\x91\x90\x81\x01\x90aW V[`\0\x81Q\x81\x10a,\x0FWa,\x0FaU\xCCV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x88\x90Rc\xFF\xFF\xFF\xFF\x87\x81\x16`$\x83\x01R\x90\x91\x16`D\x82\x01\x81\x90R\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x89\x16\x90c\x04\xECcQ\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,{W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\x9F\x91\x90aW\xCEV[`\x01`\x01`\xC0\x1B\x03\x16\x90P`\0a,\xB5\x82a>\xD5V[\x90P\x81a,\xC3\x8A\x83\x8Aa\x11\x98V[\x95P\x95PPPPP\x93P\x93\x91PPV[`fT\x15a-#W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FPausable: contract is paused\0\0\0\0`D\x82\x01R`d\x01a\x0BrV[`\x9ETd\x01\0\0\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x163\x14a-mW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04\x17WF\x83`\xDC\x1B`D\x82\x01R`d\x01a\x0BrV[`\x9CT`\x01``\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15\x15\x80a-\x8CWP`\xA3T`\xFF\x16[a-\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x1D\\\xD9H\x1C\x9B\xDB\xDD\x08\x1A[\x9A]`\x9A\x1B`D\x82\x01R`d\x01a\x0BrV[6`\0a-\xD8`\xC0\x86\x01\x86aZ\xFCV[\x90\x92P\x90P`\0a-\xF0a\x01\0\x87\x01`\xE0\x88\x01aS\x0CV[\x90Pa.>\x86`@Q` \x01a.\x06\x91\x90aR\x1BV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x91\x82\x01 \x90`\0\x90a..\x90\x89\x01\x89aS\x0CV[a\x08)`@\x8B\x01` \x8C\x01aS\x0CV[`\0\x80a.}\x87`@Q` \x01a.U\x91\x90aS\xB9V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 a&\x1D`\x80\x8B\x01``\x8C\x01aS\x0CV[\x90\x92P\x90Pa.\x8F` \x89\x01\x89aS\x0CV[c\xFF\xFF\xFF\xFF\x16\x7FG\xAD\xAC\xB0\xB6\xBB\xD7&\xAE9\xACl\0l\xCA\x1C \x06\xC9\xAE\xDA\xA8\x82\xDC\xBA|H\x04\xDB|A\xCE\x88\x83`@Qa.\xC6\x92\x91\x90aS\x99V[`@Q\x80\x91\x03\x90\xA2\x81\x15a.\xFEWa.\xF9`\0a.\xE6` \x8A\x01\x8AaS\x0CV[\x89\x84`@Q` \x01a\n\x05\x92\x91\x90aS\x99V[a/#V[a&\xE9`\0a/\x10` \x8A\x01\x8AaS\x0CV[\x89\x84`@Q` \x01a&\xCC\x92\x91\x90aS\x99V[a/1\x87`@\x015\x89a5\x8BV[a/>` \x88\x01\x88aS\x0CV[c\xFF\xFF\xFF\xFF\x16\x7F\xFF)\x08H=t\xB6\xB7\0S\xDDG2`\xAC\xF1\xB0\x9E\x0B\xA0x\x1B\xF9A\0\xBB\x82wX\x17I\xDE\x88`@Qa(\xBB\x91\x90aS\xB9V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a/\x93WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a/\xADWP0;\x15\x80\x15a/\xADWP`\0T`\xFF\x16`\x01\x14[a0\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x0BrV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a03W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a0>\x8A`\0a?\xA1V[a0G\x89a=\rV[`\x9E\x80Td\x01\0\0\0\0`\x01`\xC0\x1B\x03\x19\x16d\x01\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x8B\x81\x16\x91\x90\x91\x02\x91\x90\x91\x17\x90\x91U`\x9F\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x8A\x84\x16\x17\x90\x91U`\xA3\x80T`\xFF\x19\x16\x89\x15\x15\x17\x90U`\x97\x80T\x82\x16\x88\x84\x16\x17\x90U`\x98\x80T\x86\x84\x16`\x01`\x01`\xC0\x1B\x03\x19\x90\x91\x16\x17`\x01`\xA0\x1Bc\xFF\xFF\xFF\xFF\x89\x16\x02\x17\x90U`\xA0\x80T\x90\x91\x16\x91\x84\x16\x91\x90\x91\x17\x90U\x80\x15a1 W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPPPPV[`\x97T`@\x80Qc\xDF\\\xF7#`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xDF\\\xF7#\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a!\xA3W=`\0\x80>=`\0\xFD[a1~a4]V[`\x01`\x01`\xA0\x1B\x03\x81\x16a1\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x0BrV[a\x06\x0B\x81a=\rV[a1\xF4a4]V[`\xA0T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a2\x0EWa2\x0Ea7\\V[a2\x19\x83\x83\x83a:%V[`@Q\x7FN\xE9\x87\xE5\xF1\xBE\x19\xCA\xBF\xB1\xA2C\xE5\xC4#\x88\x9F\x06\x0F3&gS\x95?\xF0\xCF\x9D\xB8\x99f\xAB\x90`\0\x90\xA1PPPV[a2Oa4]V[`\x9E\x80Td\x01\0\0\0\0`\x01`\xC0\x1B\x03\x19\x16d\x01\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x81\x02\x91\x90\x91\x17\x90\x91U`@Q\x90\x81R\x7F`,\xECK\x15\x83\xB0}\x07\x11a\xDA^\xB9X\x94D\xD2E\x92\x01\xE2\xFA\xB7u=\xC9A\xE95\x1C!\x90` \x01a\x16\x82V[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a2\xFEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3\"\x91\x90aS\xC7V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a3RW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0Br\x90aS\xE4V[`fT\x19\x81\x19`fT\x19\x16\x14a3\xD0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0BrV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x0C\xB8V[a4\x0Fa4]V[`\xA0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F/ \xCF\x1B\xDAgs\x90D\xC5\xBFWsS\x97\x0C=\xBC\x18;,rt\xD1\xE8XJ\x10&\x922g\x90` \x01a\x16\x82V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\rEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x0BrV[\x81`\x9A`\0\x86`\x01\x81\x11\x15a4\xCEWa4\xCEaO\xF0V[`\x01\x81\x11\x15a4\xDFWa4\xDFaO\xF0V[\x81R` \x01\x90\x81R` \x01`\0 `\0\x85c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP\x80`\x9B`\0\x86`\x01\x81\x11\x15a5%Wa5%aO\xF0V[`\x01\x81\x11\x15a56Wa56aO\xF0V[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 c\xFF\xFF\xFF\xFF\x88\x16\x82R\x90\x92R\x90 \x80T`\xFF\x19\x16`\x01\x83`\x04\x81\x11\x15a5sWa5saO\xF0V[\x02\x17\x90UPP`\xA0\x80T`\xFF`\xA0\x1B\x19\x16\x90UPPPV[`\xA2\x82\x90Ua5\xA0`@\x82\x01` \x83\x01aS\x0CV[`\x9C\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x01``\x1B\x02c\xFF\xFF\xFF\xFF``\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90Ua5\xD2`\x80\x82\x01\x82aZ\xFCV[a5\xDE\x91`\x9D\x91a@\xBCV[Pa5\xEF`\xC0\x82\x01`\xA0\x83\x01aS\x0CV[`\x9E\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua6\x16` \x82\x01\x82aS\x0CV[`\x9C\x80Tk\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x16`\x01`@\x1Bc\xFF\xFF\xFF\xFF\x93\x84\x16\x02\x17\x90U`\xA3\x80Tl\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\x19\x16`\x01`H\x1BC\x90\x93\x16\x92\x90\x92\x02\x91\x90\x91\x17\x90UPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16a6\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x0BrV[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x98T`\x01`\xC0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15a8LW`\x98T`\0\x90a7\x90\x90`\x01\x90`\x01`\xC0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16a\\\xF3V[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R\x7F\x10\xAF\xAC\x923\xB4\xCC\xC5Md\x04\xFF\xC1\xCF;GQZ+\x8E\xDB\xF6u\xD1^\xDD\xCE\x05\xA0'\xDC\xBD` R`@\x90 T\x90\x91P`\x01\x90`\xFF\x16`\x04\x81\x11\x15a7\xDFWa7\xDFaO\xF0V[\x03a8JWc\xFF\xFF\xFF\xFF\x81\x16`\0\x81\x81R\x7F\x10\xAF\xAC\x923\xB4\xCC\xC5Md\x04\xFF\xC1\xCF;GQZ+\x8E\xDB\xF6u\xD1^\xDD\xCE\x05\xA0'\xDC\xBD` R`@\x80\x82 \x80T`\xFF\x19\x16`\x02\x17\x90UQ\x7F\xD6\xA4\xE0\xFF\x9F:\x057\x08u|z\x12J\xBE\xE3\x1C\xEDa\xF4?\x17\xE6\xE1\xCF\x11\x94>\xC5\x9E`q\x91\x90\xA2[P[`\x98T`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15a9<W`\x98T`\0\x90a8\x80\x90`\x01\x90`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16a\\\xF3V[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R\x7F)\x8C\x80\r\x08\x81\xDD \x8Dp^\xBC\x03\xEB\x18\x18\x9F8\x11\x82Y\xF2}\xD4;L`\xD6\x1C`~\x87` R`@\x90 T\x90\x91P`\x01\x90`\xFF\x16`\x04\x81\x11\x15a8\xCFWa8\xCFaO\xF0V[\x03a9:Wc\xFF\xFF\xFF\xFF\x81\x16`\0\x81\x81R\x7F)\x8C\x80\r\x08\x81\xDD \x8Dp^\xBC\x03\xEB\x18\x18\x9F8\x11\x82Y\xF2}\xD4;L`\xD6\x1C`~\x87` R`@\x80\x82 \x80T`\xFF\x19\x16`\x02\x17\x90UQ\x7F\x0B\xF4k\xFC\xA6\xE2\x13}5\xB8\x93\xC2\x95\xAD\xD8\xC3;\xCF\xBF\xFA\xFD\xEF\x93%,\xB5\x1A\xEDu8\xBA\x0C\x91\x90\xA2[P[`\xA0\x80T`\xFF`\xA0\x1B\x19\x16\x90UV[\x81`\x99`\0\x86`\x01\x81\x11\x15a9bWa9baO\xF0V[`\x01\x81\x11\x15a9sWa9saO\xF0V[\x81R` \x01\x90\x81R` \x01`\0 `\0\x85c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP\x80`\x9B`\0\x86`\x01\x81\x11\x15a9\xB9Wa9\xB9aO\xF0V[`\x01\x81\x11\x15a9\xCAWa9\xCAaO\xF0V[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 c\xFF\xFF\xFF\xFF\x88\x16\x82R\x90\x92R\x90 \x80T`\xFF\x19\x16`\x01\x83`\x04\x81\x11\x15a:\x07Wa:\x07aO\xF0V[\x02\x17\x90UPP`\xA0\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x17\x90UPPPV[`\x9CTc\xFF\xFF\xFF\xFF`\x01``\x1B\x90\x91\x04\x16C\x14\x80\x15\x90a:DWPC\x15\x15[a:\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FCan't in lastCompletedOpTaskCrea`D\x82\x01RgtedBlock`\xC0\x1B`d\x82\x01R`\x84\x01a\x0BrV[`\x98T`@\x80Qa\x01\0\x81\x01\x82R`\0\x81\x83\x01\x81\x90R``\x80\x83\x01\x82\x90R`\x80\x83\x01\x81\x90R`\xC0\x83\x01R`\xE0\x82\x01Rc\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x90\x93\x04\x83\x16\x80\x82RC\x84\x16` \x80\x84\x01\x91\x90\x91R\x93\x87\x16`\xA0\x83\x01R\x82Q`\x1F\x86\x01\x85\x90\x04\x85\x02\x81\x01\x85\x01\x90\x93R\x84\x83R\x92\x90\x91\x90\x85\x90\x85\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x82\x90RP`\x80\x86\x01\x94\x90\x94RPP`\x9CT`\x01``\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x90\x91\x03\x90Pa;\xAAWc\xFF\xFF\xFF\xFF\x80\x83\x16`@\x80\x84\x01\x91\x90\x91RC\x90\x91\x16``\x83\x01R\x80Q` `\x1F\x86\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x84\x81R\x90\x85\x90\x85\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPPP`\xC0\x82\x01Rc\xFF\xFF\xFF\xFF\x85\x16`\xE0\x82\x01Ra<mV[`\x9CTc\xFF\xFF\xFF\xFF`\x01`@\x1B\x82\x04\x81\x16`@\x84\x01R`\x01``\x1B\x90\x91\x04\x16``\x82\x01R`\x9D\x80Ta;\xDB\x90aT\xB2V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta<\x07\x90aT\xB2V[\x80\x15a<TW\x80`\x1F\x10a<)Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a<TV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a<7W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`\xC0\x82\x01R`\x9ETc\xFF\xFF\xFF\xFF\x16`\xE0\x82\x01R[a<\x84`\0\x83\x83`@Q` \x01a\x0FA\x91\x90a]\x18V[`\x9C\x80Tc\xFF\xFF\xFF\xFF\x19\x16Cc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x90\x91\x17\x90\x91U`@Q\x90\x83\x16\x90\x7F\xF3bg'\xFAn5e\xD6\xB23\xE2\x13l.\xE4R\xCF\x96\xE6@OQ\xD8\xCA\xA3G\xDD/{\x91\xF8\x90a<\xD3\x90\x84\x90a]\x18V[`@Q\x80\x91\x03\x90\xA2a<\xE6\x82`\x01aU\xA4V[`\x98`\x18a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPPPV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R``\x82\x84\x01\x81\x90R\x80\x83\x01R`\x97T\x92Qc7}\xA3\x1B`\xE1\x1B\x81R\x90\x92\x83\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cn\xFBF6\x90a=\xBC\x90\x8D\x90\x8B\x90\x8B\x90\x8F\x90\x8F\x90`\x04\x01aY\0V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a=\xD9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra>\x01\x91\x90\x81\x01\x90aZ`V[`@\x80Q`\x80\x81\x01\x82Rc\xFF\xFF\xFF\xFFC\x16\x81R` \x80\x82\x01\x84\x90R\x84\x01Q\x91\x81\x01\x91\x90\x91R\x82Q``\x82\x01R\x91\x93P\x91P`\x01`\0[\x88\x81\x10\x15a>\xC4W\x87`\xFF\x16\x85` \x01Q\x82\x81Q\x81\x10a>YWa>YaU\xCCV[` \x02` \x01\x01Qa>k\x91\x90a]\xD1V[`\x01`\x01``\x1B\x03\x16`d\x86`\0\x01Q\x83\x81Q\x81\x10a>\x8CWa>\x8CaU\xCCV[` \x02` \x01\x01Q`\x01`\x01``\x1B\x03\x16a>\xA7\x91\x90a^\0V[\x10\x15a>\xB2W`\0\x91P[\x80a>\xBC\x81aU\xFBV[\x91PPa>7V[P\x9B\x90\x9AP\x98PPPPPPPPPV[```\0\x80a>\xE3\x84a@\x8BV[a\xFF\xFF\x16`\x01`\x01`@\x1B\x03\x81\x11\x15a>\xFEWa>\xFEaC/V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a?(W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x80[\x82Q\x82\x10\x80\x15a?@WPa\x01\0\x81\x10[\x15a?\x97W`\x01\x81\x1B\x93P\x85\x84\x16\x15a?\x87W\x80`\xF8\x1B\x83\x83\x81Q\x81\x10a?iWa?iaU\xCCV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x81`\x01\x01\x91P[a?\x90\x81aU\xFBV[\x90Pa?/V[P\x90\x94\x93PPPPV[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a?\xC2WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a@DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x0BrV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a@\x87\x82a6eV[PPV[`\0\x80[\x82\x15a@\xB6Wa@\xA0`\x01\x84a^\x1FV[\x90\x92\x16\x91\x80a@\xAE\x81a^6V[\x91PPa@\x8FV[\x92\x91PPV[\x82\x80Ta@\xC8\x90aT\xB2V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82a@\xEAW`\0\x85UaA0V[\x82`\x1F\x10aA\x03W\x82\x80\x01`\xFF\x19\x825\x16\x17\x85UaA0V[\x82\x80\x01`\x01\x01\x85U\x82\x15aA0W\x91\x82\x01[\x82\x81\x11\x15aA0W\x825\x82U\x91` \x01\x91\x90`\x01\x01\x90aA\x15V[PaA<\x92\x91PaA@V[P\x90V[[\x80\x82\x11\x15aA<W`\0\x81U`\x01\x01aAAV[`\0a\x01\0\x82\x84\x03\x12\x15aAhW`\0\x80\xFD[P\x91\x90PV[`\0``\x82\x84\x03\x12\x15aAhW`\0\x80\xFD[`\0\x80`\x80\x83\x85\x03\x12\x15aA\x93W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aA\xA9W`\0\x80\xFD[aA\xB5\x85\x82\x86\x01aAUV[\x92PPaA\xC5\x84` \x85\x01aAnV[\x90P\x92P\x92\x90PV[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14aA\xE5W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15aA\xFCW`\0\x80\xFD[a\x16)\x82aA\xCEV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06\x0BW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aB,W`\0\x80\xFD[\x815a\x16)\x81aB\x05V[`\0` \x82\x84\x03\x12\x15aBIW`\0\x80\xFD[P5\x91\x90PV[\x805`\x02\x81\x10aA\xE5W`\0\x80\xFD[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x06\x0BW`\0\x80\xFD[\x805aA\xE5\x81aB_V[`\0\x80`@\x83\x85\x03\x12\x15aB\x8FW`\0\x80\xFD[aB\x98\x83aBPV[\x91P` \x83\x015aB\xA8\x81aB_V[\x80\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15aB\xC6W`\0\x80\xFD[aB\x98\x83aA\xCEV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15aB\xF5W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01aB\xD9V[\x81\x81\x11\x15aC\x07W`\0` \x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x16)` \x83\x01\x84aB\xCFV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aCgWaCgaC/V[`@R\x90V[`@Qa\x01\0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aCgWaCgaC/V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aC\xB8WaC\xB8aC/V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15aC\xD9WaC\xD9aC/V[P`\x05\x1B` \x01\x90V[`\0\x80`@\x83\x85\x03\x12\x15aC\xF6W`\0\x80\xFD[\x825aD\x01\x81aB\x05V[\x91P` \x83\x81\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aD\x1DW`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13aD.W`\0\x80\xFD[\x805aDAaD<\x82aC\xC0V[aC\x90V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x88\x83\x11\x15aD`W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aD\x87W\x835aDx\x81aB\x05V[\x82R\x92\x84\x01\x92\x90\x84\x01\x90aDeV[\x80\x95PPPPPP\x92P\x92\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aD\xC6W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aD\xAAV[P\x94\x95\x94PPPPPV[` \x81R`\0a\x16)` \x83\x01\x84aD\x96V[`\0\x80`\0``\x84\x86\x03\x12\x15aD\xF9W`\0\x80\xFD[\x835aE\x04\x81aB\x05V[\x92P` \x84\x81\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aE!W`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12aE5W`\0\x80\xFD[\x815\x81\x81\x11\x15aEGWaEGaC/V[aEY`\x1F\x82\x01`\x1F\x19\x16\x85\x01aC\x90V[\x91P\x80\x82R\x88\x84\x82\x85\x01\x01\x11\x15aEoW`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x94PPPPaE\x92`@\x85\x01aBqV[\x90P\x92P\x92P\x92V[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0\x80[\x86\x81\x10\x15aF1W\x83\x85\x03\x8AR\x82Q\x80Q\x80\x87R\x90\x87\x01\x90\x87\x87\x01\x90\x84[\x81\x81\x10\x15aF\x1CW\x83Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x8A\x81\x01Q\x8B\x85\x01R`@\x90\x81\x01Q`\x01`\x01``\x1B\x03\x16\x90\x84\x01R\x92\x89\x01\x92``\x90\x92\x01\x91`\x01\x01aE\xD8V[PP\x9A\x87\x01\x9A\x95PP\x91\x85\x01\x91`\x01\x01aE\xBAV[P\x92\x98\x97PPPPPPPPV[` \x81R`\0a\x16)` \x83\x01\x84aE\x9BV[`\0` \x82\x84\x03\x12\x15aFdW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aFzW`\0\x80\xFD[\x82\x01`@\x81\x85\x03\x12\x15a\x16)W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12aF\x9DW`\0\x80\xFD[\x815` aF\xADaD<\x83aC\xC0V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aF\xCCW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aF\xE7W\x805\x83R\x91\x83\x01\x91\x83\x01aF\xD0V[P\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15aG\x05W`\0\x80\xFD[\x825aG\x10\x81aB\x05V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aG+W`\0\x80\xFD[aG7\x85\x82\x86\x01aF\x8CV[\x91PP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aG\x82W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aG]V[P\x90\x96\x95PPPPPPV[`\0\x80\x83`\x1F\x84\x01\x12aG\xA0W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aG\xB7W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aG\xCFW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\x80\x87\x89\x03\x12\x15aG\xEFW`\0\x80\xFD[\x865aG\xFA\x81aB\x05V[\x95P` \x87\x015aH\n\x81aB_V[\x94P`@\x87\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aH&W`\0\x80\xFD[aH2\x8A\x83\x8B\x01aG\x8EV[\x90\x96P\x94P``\x89\x015\x91P\x80\x82\x11\x15aHKW`\0\x80\xFD[\x81\x89\x01\x91P\x89`\x1F\x83\x01\x12aH_W`\0\x80\xFD[\x815\x81\x81\x11\x15aHnW`\0\x80\xFD[\x8A` \x82`\x05\x1B\x85\x01\x01\x11\x15aH\x83W`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92\x95P\x92\x95P\x92\x95V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aD\xC6W\x81Qc\xFF\xFF\xFF\xFF\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aH\xADV[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0[\x85\x81\x10\x15aI\x17W\x82\x84\x03\x89RaI\x05\x84\x83QaH\x99V[\x98\x85\x01\x98\x93P\x90\x84\x01\x90`\x01\x01aH\xEDV[P\x91\x97\x96PPPPPPPV[` \x81R`\0\x82Q`\x80` \x84\x01RaI@`\xA0\x84\x01\x82aH\x99V[\x90P` \x84\x01Q`\x1F\x19\x80\x85\x84\x03\x01`@\x86\x01RaI^\x83\x83aH\x99V[\x92P`@\x86\x01Q\x91P\x80\x85\x84\x03\x01``\x86\x01RaI{\x83\x83aH\x99V[\x92P``\x86\x01Q\x91P\x80\x85\x84\x03\x01`\x80\x86\x01RPaI\x99\x82\x82aH\xCFV[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15aI\xB4W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aI\xCAW`\0\x80\xFD[\x82\x01a\x01 \x81\x85\x03\x12\x15a\x16)W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aI\xEFW`\0\x80\xFD[\x815`\xFF\x81\x16\x81\x14a\x16)W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15aJ\x15W`\0\x80\xFD[\x835aJ \x81aB\x05V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aJ;W`\0\x80\xFD[aJG\x86\x82\x87\x01aF\x8CV[\x92PP`@\x84\x015aJX\x81aB_V[\x80\x91PP\x92P\x92P\x92V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aG\x82W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aJ\x7FV[`\0\x80`\0`@\x84\x86\x03\x12\x15aJ\xB0W`\0\x80\xFD[\x835aJ\xBB\x81aB_V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aJ\xD6W`\0\x80\xFD[aJ\xE2\x86\x82\x87\x01aG\x8EV[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x82`\x1F\x83\x01\x12aK\0W`\0\x80\xFD[\x815` aK\x10aD<\x83aC\xC0V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aK/W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aF\xE7W\x805aKF\x81aB_V[\x83R\x91\x83\x01\x91\x83\x01aK3V[`\0`@\x82\x84\x03\x12\x15aKeW`\0\x80\xFD[aKmaCEV[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12aK\x94W`\0\x80\xFD[\x815` aK\xA4aD<\x83aC\xC0V[\x82\x81R`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aK\xC3W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aF\xE7WaK\xD9\x88\x82aKSV[\x83R\x91\x83\x01\x91`@\x01aK\xC7V[`\0\x82`\x1F\x83\x01\x12aK\xF8W`\0\x80\xFD[aL\0aCEV[\x80`@\x84\x01\x85\x81\x11\x15aL\x12W`\0\x80\xFD[\x84[\x81\x81\x10\x15aL,W\x805\x84R` \x93\x84\x01\x93\x01aL\x14V[P\x90\x95\x94PPPPPV[`\0`\x80\x82\x84\x03\x12\x15aLIW`\0\x80\xFD[aLQaCEV[\x90PaL]\x83\x83aK\xE7V[\x81RaLl\x83`@\x84\x01aK\xE7V[` \x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12aL\x88W`\0\x80\xFD[\x815` aL\x98aD<\x83aC\xC0V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aL\xB7W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aF\xE7W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aL\xDAW`\0\x80\x81\xFD[aL\xE8\x89\x86\x83\x8B\x01\x01aJ\xEFV[\x84RP\x91\x83\x01\x91\x83\x01aL\xBBV[`\0a\x01\x80\x82\x84\x03\x12\x15aM\tW`\0\x80\xFD[aM\x11aCmV[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aM*W`\0\x80\xFD[aM6\x85\x83\x86\x01aJ\xEFV[\x83R` \x84\x015\x91P\x80\x82\x11\x15aMLW`\0\x80\xFD[aMX\x85\x83\x86\x01aK\x83V[` \x84\x01R`@\x84\x015\x91P\x80\x82\x11\x15aMqW`\0\x80\xFD[aM}\x85\x83\x86\x01aK\x83V[`@\x84\x01RaM\x8F\x85``\x86\x01aL7V[``\x84\x01RaM\xA1\x85`\xE0\x86\x01aKSV[`\x80\x84\x01Ra\x01 \x84\x015\x91P\x80\x82\x11\x15aM\xBBW`\0\x80\xFD[aM\xC7\x85\x83\x86\x01aJ\xEFV[`\xA0\x84\x01Ra\x01@\x84\x015\x91P\x80\x82\x11\x15aM\xE1W`\0\x80\xFD[aM\xED\x85\x83\x86\x01aJ\xEFV[`\xC0\x84\x01Ra\x01`\x84\x015\x91P\x80\x82\x11\x15aN\x07W`\0\x80\xFD[PaN\x14\x84\x82\x85\x01aLwV[`\xE0\x83\x01RP\x92\x91PPV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15aN8W`\0\x80\xFD[\x855\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aNVW`\0\x80\xFD[aNb\x89\x83\x8A\x01aG\x8EV[\x90\x96P\x94P`@\x88\x015\x91PaNw\x82aB_V[\x90\x92P``\x87\x015\x90\x80\x82\x11\x15aN\x8DW`\0\x80\xFD[PaN\x9A\x88\x82\x89\x01aL\xF6V[\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aD\xC6W\x81Q`\x01`\x01``\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aN\xBBV[`@\x81R`\0\x83Q`@\x80\x84\x01RaN\xFB`\x80\x84\x01\x82aN\xA7V[\x90P` \x85\x01Q`?\x19\x84\x83\x03\x01``\x85\x01RaO\x18\x82\x82aN\xA7V[\x92PPP\x82` \x83\x01R\x93\x92PPPV[`\0\x80`\0a\x01@\x84\x86\x03\x12\x15aO?W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aOVW`\0\x80\xFD[aOb\x87\x83\x88\x01aAUV[\x94PaOq\x87` \x88\x01aAUV[\x93Pa\x01 \x86\x015\x91P\x80\x82\x11\x15aO\x88W`\0\x80\xFD[PaO\x95\x86\x82\x87\x01aL\xF6V[\x91PP\x92P\x92P\x92V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aO\xB5W`\0\x80\xFD[\x845\x93PaO\xC5` \x86\x01aBPV[\x92P`@\x85\x015aO\xD5\x81aB_V[\x91P``\x85\x015aO\xE5\x81aB_V[\x93\x96\x92\x95P\x90\x93PPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x05\x83\x10aP(WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15aPCW`\0\x80\xFD[\x835aPN\x81aB\x05V[\x92P` \x84\x015\x91P`@\x84\x015aJX\x81aB_V[\x82\x81R`@` \x82\x01R`\0aP~`@\x83\x01\x84aE\x9BV[\x94\x93PPPPV[`\0\x80`\0`\xA0\x84\x86\x03\x12\x15aP\x9BW`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aP\xB2W`\0\x80\xFD[aP\xBE\x87\x83\x88\x01aAUV[\x94PaP\xCD\x87` \x88\x01aAnV[\x93P`\x80\x86\x015\x91P\x80\x82\x11\x15aO\x88W`\0\x80\xFD[\x80\x15\x15\x81\x14a\x06\x0BW`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0\x80`\0a\x01 \x8A\x8C\x03\x12\x15aQ\x10W`\0\x80\xFD[\x895aQ\x1B\x81aB\x05V[\x98P` \x8A\x015aQ+\x81aB\x05V[\x97P`@\x8A\x015aQ;\x81aB\x05V[\x96P``\x8A\x015aQK\x81aB\x05V[\x95P`\x80\x8A\x015aQ[\x81aP\xE3V[\x94P`\xA0\x8A\x015aQk\x81aB\x05V[\x93P`\xC0\x8A\x015aQ{\x81aB_V[\x92P`\xE0\x8A\x015aQ\x8B\x81aB\x05V[\x91Pa\x01\0\x8A\x015aQ\x9C\x81aB\x05V[\x80\x91PP\x92\x95\x98P\x92\x95\x98P\x92\x95\x98V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aQ\xC4W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aQ\xE3W`\0\x80\xFD[\x806\x03\x83\x13\x15aG\xCFW`\0\x80\xFD[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x81R`\0\x825aR,\x81aB_V[c\xFF\xFF\xFF\xFF\x81\x16` \x84\x01RPaRE` \x84\x01aBqV[c\xFF\xFF\xFF\xFF\x81\x16`@\x84\x01RPaR^`@\x84\x01aBqV[c\xFF\xFF\xFF\xFF\x81\x16``\x84\x01RPaRw``\x84\x01aBqV[c\xFF\xFF\xFF\xFF\x81\x16`\x80\x84\x01RPaR\x91`\x80\x84\x01\x84aQ\xADV[a\x01\0\x80`\xA0\x86\x01RaR\xA9a\x01 \x86\x01\x83\x85aQ\xF2V[\x92PaR\xB7`\xA0\x87\x01aBqV[c\xFF\xFF\xFF\xFF\x81\x16`\xC0\x87\x01R\x91PaR\xD2`\xC0\x87\x01\x87aQ\xADV[\x86\x85\x03`\x1F\x19\x01`\xE0\x88\x01R\x92PaR\xEB\x84\x84\x83aQ\xF2V[\x93PPaR\xFA`\xE0\x87\x01aBqV[c\xFF\xFF\xFF\xFF\x81\x16\x86\x83\x01R\x91Pa?\x97V[`\0` \x82\x84\x03\x12\x15aS\x1EW`\0\x80\xFD[\x815a\x16)\x81aB_V[\x805aS4\x81aB_V[c\xFF\xFF\xFF\xFF\x16\x82R` \x81\x81\x015\x90\x83\x01R`@\x90\x81\x015\x91\x01RV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Q` \x83\x01R`\0`@\x82\x01Q`\x80`@\x85\x01RaS\x80`\x80\x85\x01\x82aN\xA7V[\x90P``\x83\x01Q\x84\x82\x03``\x86\x01RaI\x99\x82\x82aN\xA7V[aS\xA3\x81\x84aS)V[`\x80``\x82\x01R`\0aP~`\x80\x83\x01\x84aSQV[``\x81\x01a@\xB6\x82\x84aS)V[`\0` \x82\x84\x03\x12\x15aS\xD9W`\0\x80\xFD[\x81Qa\x16)\x81aB\x05V[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15aT@W`\0\x80\xFD[\x81Qa\x16)\x81aP\xE3V[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`\x05\x90\x82\x01RdAuth1`\xD8\x1B`@\x82\x01R``\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80aT\xC6W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aAhWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[` \x81Rc\xFF\xFF\xFF\xFF\x82Q\x16` \x82\x01R`\x01`\x01`@\x1B\x03` \x83\x01Q\x16`@\x82\x01R`\0`@\x83\x01QaU#``\x84\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P``\x83\x01Qc\xFF\xFF\xFF\xFF\x81\x16`\x80\x84\x01RP`\x80\x83\x01Qc\xFF\xFF\xFF\xFF\x81\x16`\xA0\x84\x01RP`\xA0\x83\x01Qc\xFF\xFF\xFF\xFF\x81\x16`\xC0\x84\x01RP`\xC0\x83\x01Qa\x01\0\x80`\xE0\x85\x01RaUva\x01 \x85\x01\x83aB\xCFV[\x91P`\xE0\x85\x01Qa?\x97\x82\x86\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aU\xC3WaU\xC3aU\x8EV[\x01\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15aU\xF4W`\0\x80\xFD[PQ\x91\x90PV[`\0`\x01\x82\x01aV\rWaV\raU\x8EV[P`\x01\x01\x90V[`\0` \x80\x83\x85\x03\x12\x15aV'W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aV=W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aVNW`\0\x80\xFD[\x80QaV\\aD<\x82aC\xC0V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15aV{W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aV\x99W\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90aV\x80V[\x97\x96PPPPPPPV[\x80Q`\x01`\x01``\x1B\x03\x81\x16\x81\x14aA\xE5W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aV\xCDW`\0\x80\xFD[a\x16)\x82aV\xA4V[c\xFF\xFF\xFF\xFF\x84\x16\x81R`@` \x82\x01\x81\x90R\x81\x01\x82\x90R`\0`\x01`\x01`\xFB\x1B\x03\x83\x11\x15aW\x03W`\0\x80\xFD[\x82`\x05\x1B\x80\x85``\x85\x017`\0\x92\x01``\x01\x91\x82RP\x93\x92PPPV[`\0` \x80\x83\x85\x03\x12\x15aW3W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aWIW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aWZW`\0\x80\xFD[\x80QaWhaD<\x82aC\xC0V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15aW\x87W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aV\x99W\x83QaW\x9F\x81aB_V[\x82R\x92\x84\x01\x92\x90\x84\x01\x90aW\x8CV[c\xFF\xFF\xFF\xFF\x84\x16\x81R`@` \x82\x01R`\0aI\x99`@\x83\x01\x84\x86aQ\xF2V[`\0` \x82\x84\x03\x12\x15aW\xE0W`\0\x80\xFD[\x81Q`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x14a\x16)W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aX\tW`\0\x80\xFD[\x81Qa\x16)\x81aB_V[`\0`\xFF\x82\x16`\xFF\x81\x03aX*WaX*aU\x8EV[`\x01\x01\x92\x91PPV[`@\x81R`\0aXG`@\x83\x01\x85\x87aQ\xF2V[\x90Pc\xFF\xFF\xFF\xFF\x83\x16` \x83\x01R\x94\x93PPPPV[c\xFF\xFF\xFF\xFF\x83\x16\x81R`@` \x82\x01R`\0aP~`@\x83\x01\x84aD\x96V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aD\xC6WaX\xAD\x87\x83Q\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[`@\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01aX\x90V[\x80`\0[`\x02\x81\x10\x15a+;W\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01aX\xC4V[aX\xEE\x82\x82QaX\xC0V[` \x81\x01Qa\"\xEE`@\x84\x01\x82aX\xC0V[\x85\x81R`\x80` \x82\x01R`\0aY\x1A`\x80\x83\x01\x86\x88aQ\xF2V[c\xFF\xFF\xFF\xFF\x85\x16`@\x84\x01R\x82\x81\x03``\x84\x01Ra\x01\x80\x84Q\x81\x83RaYB\x82\x84\x01\x82aH\x99V[\x91PP` \x85\x01Q\x82\x82\x03` \x84\x01RaY\\\x82\x82aX|V[\x91PP`@\x85\x01Q\x82\x82\x03`@\x84\x01RaYv\x82\x82aX|V[\x91PP``\x85\x01QaY\x8B``\x84\x01\x82aX\xE3V[P`\x80\x85\x01Q\x80Q`\xE0\x84\x01R` \x01Qa\x01\0\x83\x01R`\xA0\x85\x01Q\x82\x82\x03a\x01 \x84\x01RaY\xBA\x82\x82aH\x99V[\x91PP`\xC0\x85\x01Q\x82\x82\x03a\x01@\x84\x01RaY\xD5\x82\x82aH\x99V[\x91PP`\xE0\x85\x01Q\x82\x82\x03a\x01`\x84\x01RaY\xF0\x82\x82aH\xCFV[\x9A\x99PPPPPPPPPPV[`\0\x82`\x1F\x83\x01\x12aZ\x0FW`\0\x80\xFD[\x81Q` aZ\x1FaD<\x83aC\xC0V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aZ>W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aF\xE7WaZS\x81aV\xA4V[\x83R\x91\x83\x01\x91\x83\x01aZBV[`\0\x80`@\x83\x85\x03\x12\x15aZsW`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aZ\x8AW`\0\x80\xFD[\x90\x84\x01\x90`@\x82\x87\x03\x12\x15aZ\x9EW`\0\x80\xFD[aZ\xA6aCEV[\x82Q\x82\x81\x11\x15aZ\xB5W`\0\x80\xFD[aZ\xC1\x88\x82\x86\x01aY\xFEV[\x82RP` \x83\x01Q\x82\x81\x11\x15aZ\xD6W`\0\x80\xFD[aZ\xE2\x88\x82\x86\x01aY\xFEV[` \x83\x01RP\x80\x94PPPP` \x83\x01Q\x90P\x92P\x92\x90PV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a[\x13W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a[-W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15aG\xCFW`\0\x80\xFD[` \x81R`\0\x825a[S\x81aB_V[c\xFF\xFF\xFF\xFF\x81\x16` \x84\x01RPa[l` \x84\x01aA\xCEV[`\x01`\x01`@\x1B\x03\x81\x16`@\x84\x01RPa[\x88`@\x84\x01aBqV[c\xFF\xFF\xFF\xFF\x81\x16``\x84\x01RPa[\xA1``\x84\x01aBqV[c\xFF\xFF\xFF\xFF\x81\x16`\x80\x84\x01RPa[\xBA`\x80\x84\x01aBqV[c\xFF\xFF\xFF\xFF\x81\x16`\xA0\x84\x01RPa[\xD3`\xA0\x84\x01aBqV[c\xFF\xFF\xFF\xFF\x81\x16`\xC0\x84\x01RPa[\xED`\xC0\x84\x01\x84aQ\xADV[a\x01\0\x80`\xE0\x86\x01Ra\\\x05a\x01 \x86\x01\x83\x85aQ\xF2V[\x92PaR\xFA`\xE0\x87\x01aBqV[\x805a\\\x1E\x81aB_V[c\xFF\xFF\xFF\xFF\x80\x82\x16\x84R` \x83\x015` \x85\x01R`\x01`\x01`@\x1B\x03a\\F`@\x85\x01aA\xCEV[\x16`@\x85\x01R``\x83\x015\x91Pa\\\\\x82aB_V[\x16``\x83\x01R`\x80\x81\x81\x015\x90\x83\x01R`\xA0\x80\x82\x015\x90\x83\x01R`\xC0\x80\x82\x015\x90\x83\x01R`\xE0\x81\x015a\\\x8E\x81aB\x05V[`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x92\x90\x92\x01\x91\x90\x91RPV[a\x01\0\x81\x01a@\xB6\x82\x84a\\\x13V[`\0a\x01 a\\\xC2\x83\x86a\\\x13V[\x80a\x01\0\x84\x01RaI\x99\x81\x84\x01\x85aSQV[\x82\x81R``\x81\x01a\x16)` \x83\x01\x84\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[`\0c\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a]\x10Wa]\x10aU\x8EV[\x03\x93\x92PPPV[` \x81Rc\xFF\xFF\xFF\xFF\x82Q\x16` \x82\x01R`\0` \x83\x01Qa]B`@\x84\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P`@\x83\x01Qc\xFF\xFF\xFF\xFF\x81\x16``\x84\x01RP``\x83\x01Qc\xFF\xFF\xFF\xFF\x81\x16`\x80\x84\x01RP`\x80\x83\x01Qa\x01\0\x80`\xA0\x85\x01Ra]\x83a\x01 \x85\x01\x83aB\xCFV[\x91P`\xA0\x85\x01Qa]\x9C`\xC0\x86\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P`\xC0\x85\x01Q\x84\x83\x03`\x1F\x19\x01`\xE0\x86\x01Ra]\xB8\x83\x82aB\xCFV[\x92PP`\xE0\x85\x01Qa?\x97\x82\x86\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15a]\xF7Wa]\xF7aU\x8EV[\x02\x94\x93PPPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a^\x1AWa^\x1AaU\x8EV[P\x02\x90V[`\0\x82\x82\x10\x15a^1Wa^1aU\x8EV[P\x03\x90V[`\0a\xFF\xFF\x80\x83\x16\x81\x81\x03a^MWa^MaU\x8EV[`\x01\x01\x93\x92PPPV\xFE\xA2dipfsX\"\x12 n\x87\xBF\xE1\xAC\x91%+\xBDTjZ\xBB\xA7s\xFF\xF7\x88\x94\xF2\xC9\xADy\x93\x84\x99YV}\xDE\x8E\x1CdsolcC\0\x08\r\x003";
    /// The bytecode of the contract.
    pub static FINALIZERTASKMANAGER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x03\xC5W`\x005`\xE0\x1C\x80c]\xF4YF\x11a\x01\xFFW\x80c\xA9\x07{\xFB\x11a\x01\x1AW\x80c\xDF\\\xF7#\x11a\0\xADW\x80c\xF5d\x0C\xF8\x11a\0|W\x80c\xF5d\x0C\xF8\x14a\x08\xFCW\x80c\xF9\x12\n\xF6\x14a\t\x0FW\x80c\xFA\xBC\x1C\xBC\x14a\t\"W\x80c\xFD\xC1]\xE8\x14a\t5W`\0\x80\xFD[\x80c\xDF\\\xF7#\x14a\x08\xC9W\x80c\xE7\x0C&#\x14a\x08\xD1W\x80c\xEF\x02DX\x14a\x08\xE1W\x80c\xF2\xFD\xE3\x8B\x14a\x08\xE9W`\0\x80\xFD[\x80c\xCAP\xE5j\x11a\0\xE9W\x80c\xCAP\xE5j\x14a\x08iW\x80c\xCE\xFD\xC1\xD4\x14a\x08\x82W\x80c\xD77\xD9\xF2\x14a\x08\xA3W\x80c\xDECH8\x14a\x08\xB6W`\0\x80\xFD[\x80c\xA9\x07{\xFB\x14a\x07\xFFW\x80c\xAD\xFC\xB0H\x14a\x08\x12W\x80c\xB1\xED\xC8\xB4\x14a\x08\x1BW\x80c\xBF#\x15\xED\x14a\x08.W`\0\x80\xFD[\x80cz\xFA\x1E\xED\x11a\x01\x92W\x80c\x8C\x82\xAF^\x11a\x01aW\x80c\x8C\x82\xAF^\x14a\x07\xA8W\x80c\x8D\xA5\xCB[\x14a\x07\xC0W\x80c\x8F\xC8r\x9A\x14a\x07\xD1W\x80c\xA6\x95c\xA9\x14a\x07\xE8W`\0\x80\xFD[\x80cz\xFA\x1E\xED\x14a\x07XW\x80cz\xFD\xD5K\x14a\x07kW\x80c\x83\x80\xAC\xBD\x14a\x07\x82W\x80c\x88o\x11\x95\x14a\x07\x95W`\0\x80\xFD[\x80cn\x12_\xF4\x11a\x01\xCEW\x80cn\x12_\xF4\x14a\x07\tW\x80cn\xFBF6\x14a\x07\x1CW\x80cqP\x18\xA6\x14a\x07=W\x80cr1\x14\xAB\x14a\x07EW`\0\x80\xFD[\x80c]\xF4YF\x14a\x06\xE9W\x80c` /\xC0\x14a\x06\xF1W\x80ch0H5\x14a\x06\xF9W\x80cm\x14\xA9\x87\x14a\x07\x01W`\0\x80\xFD[\x80c5c\xB0\xD1\x11a\x02\xEFW\x80cMzq\x16\x11a\x02\x82W\x80cY\\jg\x11a\x02QW\x80cY\\jg\x14a\x06\x96W\x80cZ\xC8j\xB7\x14a\x06\x9EW\x80c\\\x15Vb\x14a\x06\xC1W\x80c\\\x97Z\xBB\x14a\x06\xE1W`\0\x80\xFD[\x80cMzq\x16\x14a\x06AW\x80cOs\x9Ft\x14a\x06QW\x80cSz))\x14a\x06qW\x80cT\xD1'\xDE\x14a\x06\x88W`\0\x80\xFD[\x80cAx\x9DW\x11a\x02\xBEW\x80cAx\x9DW\x14a\x05\xE6W\x80cE&[z\x14a\x05\xFDW\x80cJ|~K\x14a\x06\x0EW\x80cM+W\xFE\x14a\x06!W`\0\x80\xFD[\x80c5c\xB0\xD1\x14a\x05\x88W\x80c6\xF7\x8E\xD8\x14a\x05\xA8W\x80c=r\"\xEB\x14a\x05\xBCW\x80c=\x9F\xB0\x0C\x14a\x05\xD3W`\0\x80\xFD[\x80c\x1A\xC2r\x97\x11a\x03gW\x80c&\x12\xFC\xEF\x11a\x036W\x80c&\x12\xFC\xEF\x14a\x05)W\x80c(0\xE8\xF9\x14a\x05<W\x80c,\x11\x01\xDA\x14a\x05QW\x80c1\xB3k\xD9\x14a\x05hW`\0\x80\xFD[\x80c\x1A\xC2r\x97\x14a\x04\xB0W\x80c\x1C\x17\x8E\x9C\x14a\x04\xDBW\x80c!\xE7\x80b\x14a\x05\x06W\x80c$Z{\xFC\x14a\x05\x0EW`\0\x80\xFD[\x80c\x0E\xE0\xFD\xBD\x11a\x03\xA3W\x80c\x0E\xE0\xFD\xBD\x14a\x044W\x80c\x10\xD6z/\x14a\x04QW\x80c\x13d9\xDD\x14a\x04dW\x80c\x13\xF8\x15\xED\x14a\x04wW`\0\x80\xFD[\x80c\t\x80\xF1\xEC\x14a\x03\xCAW\x80c\t\xEC\x19\x06\x14a\x03\xF9W\x80c\x0E\xBF*\x81\x14a\x04\x0EW[`\0\x80\xFD[`\xA3Ta\x03\xDF\x90a\x01\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x04\x0Ca\x04\x076`\x04aA\x80V[a\tHV[\0[a\x03\xDFa\x04\x1C6`\x04aA\xEAV[`\xA1` R`\0\x90\x81R`@\x90 Tc\xFF\xFF\xFF\xFF\x16\x81V[`\xA3Ta\x04A\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x03\xF0V[a\x04\x0Ca\x04_6`\x04aB\x1AV[a\n\xCBV[a\x04\x0Ca\x04r6`\x04aB7V[a\x0B\x84V[a\x04\xA2a\x04\x856`\x04aB|V[`\x99` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[`@Q\x90\x81R` \x01a\x03\xF0V[a\x04\xA2a\x04\xBE6`\x04aB|V[`\x9A` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[`\x97Ta\x04\xEE\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03\xF0V[a\x04\x0Ca\x0C\xC3V[`\x9ETa\x04\xEE\x90d\x01\0\0\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x04\x0Ca\x0576`\x04aB\xB3V[a\rGV[a\x05Da\x0F\xEEV[`@Qa\x03\xF0\x91\x90aC\x1CV[`\xA3Ta\x03\xDF\x90`\x01`H\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x05{a\x05v6`\x04aC\xE3V[a\x10|V[`@Qa\x03\xF0\x91\x90aD\xD1V[a\x05\x9Ba\x05\x966`\x04aD\xE4V[a\x11\x98V[`@Qa\x03\xF0\x91\x90aF?V[`\xA0Ta\x04A\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[`\xA3Ta\x03\xDF\x90`\x01`h\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`\xA0Ta\x04\xEE\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x98Ta\x03\xDF\x90`\x01`\xC0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x04\x0Ca\x06\x0B6`\x04aFRV[PV[a\x04\x0Ca\x06\x1C6`\x04aB\x1AV[a\x160V[a\x064a\x06/6`\x04aF\xF2V[a\x16\x8DV[`@Qa\x03\xF0\x91\x90aGAV[`\x9CTa\x03\xDF\x90c\xFF\xFF\xFF\xFF\x16\x81V[a\x06da\x06_6`\x04aG\xD6V[a\x17\xA2V[`@Qa\x03\xF0\x91\x90aI$V[`\x9CTa\x03\xDF\x90`\x01``\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x04\x0Ca\x06\x0B6`\x04aI\xA2V[a\x04\x0Ca\x1E\xCAV[a\x04Aa\x06\xAC6`\x04aI\xDDV[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[a\x06\xD4a\x06\xCF6`\x04aJ\0V[a\x1F\x91V[`@Qa\x03\xF0\x91\x90aJcV[`fTa\x04\xA2V[a\x04\xEEa!YV[a\x04\x0Ca!\xCCV[a\x04\xEEa!\xD4V[a\x04\xEEa\"\x1EV[a\x04\x0Ca\x07\x176`\x04aJ\x9BV[a\"hV[a\x07/a\x07*6`\x04aN V[a\"\xF3V[`@Qa\x03\xF0\x92\x91\x90aN\xE0V[a\x04\x0Ca#\x93V[a\x04\x0Ca\x07S6`\x04aB\x1AV[a#\xA5V[`\x9FTa\x04\xEE\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x98Ta\x03\xDF\x90`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`\x98Ta\x04\xEE\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`eTa\x04\xEE\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x9CTa\x03\xDF\x90d\x01\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x04\xEEV[`\x9CTa\x03\xDF\x90`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`\x98Ta\x03\xDF\x90`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x04\x0Ca\x08\r6`\x04aO)V[a#\xFBV[a\x04\xA2`\xA2T\x81V[a\x04\x0Ca\x08)6`\x04aO\x9FV[a(\xCDV[a\x08\\a\x08<6`\x04aB|V[`\x9B` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[`@Qa\x03\xF0\x91\x90aP\x06V[`\xA3Ta\x03\xDF\x90e\x01\0\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x08\x95a\x08\x906`\x04aP.V[a+AV[`@Qa\x03\xF0\x92\x91\x90aPeV[a\x04\x0Ca\x08\xB16`\x04aP\x86V[a,\xD3V[a\x04\x0Ca\x08\xC46`\x04aP\xF1V[a/sV[a\x04\xEEa1,V[`\x9ETa\x03\xDF\x90c\xFF\xFF\xFF\xFF\x16\x81V[a\x04\xA2`d\x81V[a\x04\x0Ca\x08\xF76`\x04aB\x1AV[a1vV[a\x04\x0Ca\t\n6`\x04aJ\x9BV[a1\xECV[a\x04\x0Ca\t\x1D6`\x04aB\x1AV[a2GV[a\x04\x0Ca\t06`\x04aB7V[a2\xABV[a\x04\x0Ca\tC6`\x04aB\x1AV[a4\x07V[a\tPa4]V[a\t\x9C\x82`@Q` \x01a\td\x91\x90aR\x1BV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x91\x82\x01 \x90`\0\x90a\t\x8C\x90\x85\x01\x85aS\x0CV[a\x08)`@\x87\x01` \x88\x01aS\x0CV[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`@\x80Q`\x80\x81\x01\x82Rc\xFF\xFF\xFF\xFFC\x16\x81R`\0` \x80\x83\x01\x82\x90R\x84\x81\x01Q\x93\x83\x01\x93\x90\x93R\x83Q``\x83\x01R\x91a\n\"\x90\x83\x90a\t\xF2\x90\x87\x01\x87aS\x0CV[\x86\x84`@Q` \x01a\n\x05\x92\x91\x90aS\x99V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\x04a4\xB7V[a\n0\x84`@\x015\x86a5\x8BV[a\n=` \x85\x01\x85aS\x0CV[c\xFF\xFF\xFF\xFF\x16\x7F\xFF)\x08H=t\xB6\xB7\0S\xDDG2`\xAC\xF1\xB0\x9E\x0B\xA0x\x1B\xF9A\0\xBB\x82wX\x17I\xDE\x85`@Qa\nr\x91\x90aS\xB9V[`@Q\x80\x91\x03\x90\xA2a\n\x87` \x85\x01\x85aS\x0CV[c\xFF\xFF\xFF\xFF\x16\x7F\xDF\"\xF3U\x8EHA\xB6=w\x17\x95F\xB3\xEA\xE6>N4;\xBEu'F\xB0\x93\x16+\xC5&\xBEL\x85`@Qa\n\xBC\x91\x90aS\xB9V[`@Q\x80\x91\x03\x90\xA2PPPPPV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\x1EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0BB\x91\x90aS\xC7V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0B{W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0Br\x90aS\xE4V[`@Q\x80\x91\x03\x90\xFD[a\x06\x0B\x81a6eV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xCCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xF0\x91\x90aT.V[a\x0C\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0Br\x90aTKV[`fT\x81\x81\x16\x14a\x0C\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0BrV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`\x9FT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0C\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0Br\x90aT\x93V[`\xA0T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15\x15`\x01\x14a\r=W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01RnNo task pending`\x88\x1B`D\x82\x01R`d\x01a\x0BrV[a\rEa7\\V[V[`\x9FT`\x01`\x01`\xA0\x1B\x03\x163\x14a\rqW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0Br\x90aT\x93V[`\xA0T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\r\xC2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RsTask already pending``\x1B`D\x82\x01R`d\x01a\x0BrV[`\x9CT`\x01``\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15\x80\x15\x90a\r\xDFWPC\x15\x15[a\x0E\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x13\xDC\x08\x14\xDD\x18]\x19H\x1D[\x9A[\x9A]`\x8A\x1B`D\x82\x01R`d\x01a\x0BrV[`\x98T`@\x80Qa\x01\0\x81\x01\x82R`\x01`\xE0\x1B\x90\x92\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x80\x84R`\x01`\x01`@\x1B\x03\x86\x16` \x85\x01R\x84\x82\x16\x92\x84\x01\x92\x90\x92RC\x81\x16``\x84\x01R`\x9CT`\x01`@\x1B\x81\x04\x82\x16`\x80\x85\x01R`\x01``\x1B\x90\x04\x16`\xA0\x83\x01R`\x9D\x80T\x91\x92`\0\x92\x90\x91`\xC0\x83\x01\x91a\x0E\x96\x90aT\xB2V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0E\xC2\x90aT\xB2V[\x80\x15a\x0F\x0FW\x80`\x1F\x10a\x0E\xE4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0F\x0FV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0E\xF2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x9ETc\xFF\xFF\xFF\xFF\x16` \x91\x82\x01R`@Q\x91\x92Pa\x0F^\x91`\x01\x91\x85\x91a\x0FA\x91\x86\x91\x01aT\xE6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\x01a9KV[C`\x9C`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81c\xFF\xFF\xFF\xFF\x16\x7F\x92[\x9C<\xD9\xE2\xCB\x02v\xA1w\x93\xEBh\x0E\xF1\x9Amq\xDC\x9DU(Hv\xF2\x98\xA0\xD8fcV\x82`@Qa\x0F\xB5\x91\x90aT\xE6V[`@Q\x80\x91\x03\x90\xA2a\x0F\xC8\x82`\x01aU\xA4V[`\x98`\x1Ca\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPPV[`\x9D\x80Ta\x0F\xFB\x90aT\xB2V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x10'\x90aT\xB2V[\x80\x15a\x10tW\x80`\x1F\x10a\x10IWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x10tV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x10WW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[``\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x10\x97Wa\x10\x97aC/V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x10\xC0W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82Q\x81\x10\x15a\x11\x91W\x83`\x01`\x01`\xA0\x1B\x03\x16c\x13T*N\x84\x83\x81Q\x81\x10a\x10\xF0Wa\x10\xF0aU\xCCV[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x11#\x91\x90`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11@W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11d\x91\x90aU\xE2V[\x82\x82\x81Q\x81\x10a\x11vWa\x11vaU\xCCV[` \x90\x81\x02\x91\x90\x91\x01\x01Ra\x11\x8A\x81aU\xFBV[\x90Pa\x10\xC6V[P\x92\x91PPV[```\0\x84`\x01`\x01`\xA0\x1B\x03\x16ch0H5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xDAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xFE\x91\x90aS\xC7V[\x90P`\0\x85`\x01`\x01`\xA0\x1B\x03\x16c\x9E\x99#\xC2`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12@W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12d\x91\x90aS\xC7V[\x90P`\0\x86`\x01`\x01`\xA0\x1B\x03\x16c]\xF4YF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xA6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xCA\x91\x90aS\xC7V[\x90P`\0\x86Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x12\xE7Wa\x12\xE7aC/V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x13\x1AW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x13\x05W\x90P[P\x90P`\0[\x87Q\x81\x10\x15a\x16\"W`\0\x88\x82\x81Q\x81\x10a\x13=Wa\x13=aU\xCCV[\x01` \x01Q`@Qc\x89\x02bE`\xE0\x1B\x81R`\xF8\x91\x90\x91\x1C`\x04\x82\x01\x81\x90Rc\xFF\xFF\xFF\xFF\x8A\x16`$\x83\x01R\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x87\x16\x90c\x89\x02bE\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\x9EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x13\xC6\x91\x90\x81\x01\x90aV\x14V[\x90P\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x13\xE1Wa\x13\xE1aC/V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x14,W\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a\x13\xFFW\x90P[P\x84\x84\x81Q\x81\x10a\x14?Wa\x14?aU\xCCV[` \x02` \x01\x01\x81\x90RP`\0[\x81Q\x81\x10\x15a\x16\x0CW`@Q\x80``\x01`@R\x80\x87`\x01`\x01`\xA0\x1B\x03\x16cG\xB3\x14\xE8\x85\x85\x81Q\x81\x10a\x14\x82Wa\x14\x82aU\xCCV[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14\xA8\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\xC5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\xE9\x91\x90aS\xC7V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x83\x81Q\x81\x10a\x15\tWa\x15\taU\xCCV[` \x02` \x01\x01Q\x81R` \x01\x89`\x01`\x01`\xA0\x1B\x03\x16c\xFA(\xC6'\x85\x85\x81Q\x81\x10a\x157Wa\x157aU\xCCV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x04\x81\x01\x91\x90\x91R`\xFF\x88\x16`$\x82\x01Rc\xFF\xFF\xFF\xFF\x8F\x16`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\x93W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xB7\x91\x90aV\xBBV[`\x01`\x01``\x1B\x03\x16\x81RP\x85\x85\x81Q\x81\x10a\x15\xD5Wa\x15\xD5aU\xCCV[` \x02` \x01\x01Q\x82\x81Q\x81\x10a\x15\xEEWa\x15\xEEaU\xCCV[` \x02` \x01\x01\x81\x90RP\x80\x80a\x16\x04\x90aU\xFBV[\x91PPa\x14MV[PPP\x80\x80a\x16\x1A\x90aU\xFBV[\x91PPa\x13 V[P\x93PPPP[\x93\x92PPPV[a\x168a4]V[`\x9F\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F`\xF5\xACH\xA1?\x8B[\xF4\xB2\x13\xDE\x19\r\xD2\xDE%\x92\xC9\xF6\xF5\xAC}\xC1N=rk\x95\xDE\xD2\xDA\x90` \x01[`@Q\x80\x91\x03\x90\xA1PV[``\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16\xA8Wa\x16\xA8aC/V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x16\xD1W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82Q\x81\x10\x15a\x11\x91W\x83`\x01`\x01`\xA0\x1B\x03\x16c)k\xB0d\x84\x83\x81Q\x81\x10a\x17\x01Wa\x17\x01aU\xCCV[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17'\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17h\x91\x90aS\xC7V[\x82\x82\x81Q\x81\x10a\x17zWa\x17zaU\xCCV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01Ra\x17\x9B\x81aU\xFBV[\x90Pa\x16\xD7V[a\x17\xCD`@Q\x80`\x80\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`\0\x87`\x01`\x01`\xA0\x1B\x03\x16ch0H5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x181\x91\x90aS\xC7V[\x90Pa\x18^`@Q\x80`\x80\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`@Qca\xC8\xA1/`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90c\xC3\x91B^\x90a\x18\x8E\x90\x8B\x90\x89\x90\x89\x90`\x04\x01aV\xD6V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\xABW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x18\xD3\x91\x90\x81\x01\x90aW V[\x81R`@Qc@\xE0:\x81`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\x81\xC0u\x02\x90a\x19\x05\x90\x8B\x90\x8B\x90\x8B\x90`\x04\x01aW\xAEV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\"W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x19J\x91\x90\x81\x01\x90aW V[`@\x82\x01R\x85`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19gWa\x19gaC/V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x19\x9AW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x19\x85W\x90P[P``\x82\x01R`\0[`\xFF\x81\x16\x87\x11\x15a\x1D\xDBW`\0\x85`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19\xC8Wa\x19\xC8aC/V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x19\xF1W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x83``\x01Q\x83`\xFF\x16\x81Q\x81\x10a\x1A\x0BWa\x1A\x0BaU\xCCV[` \x02` \x01\x01\x81\x90RP`\0[\x86\x81\x10\x15a\x1C\xDBW`\0\x8C`\x01`\x01`\xA0\x1B\x03\x16c\x04\xECcQ\x8A\x8A\x85\x81\x81\x10a\x1ADWa\x1ADaU\xCCV[\x90P` \x02\x015\x8E\x88`\0\x01Q\x86\x81Q\x81\x10a\x1AbWa\x1AbaU\xCCV[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1A\x9F\x93\x92\x91\x90\x92\x83Rc\xFF\xFF\xFF\xFF\x91\x82\x16` \x84\x01R\x16`@\x82\x01R``\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\xBCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xE0\x91\x90aW\xCEV[\x90P\x80`\x01`\x01`\xC0\x1B\x03\x16`\0\x03a\x1B\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R\x7FOperatorStateRetriever.getCheckS`D\x82\x01R\x7FignaturesIndices: operator must `d\x82\x01R\x7Fbe registered at blocknumber\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x0BrV[\x8A\x8A\x85`\xFF\x16\x81\x81\x10a\x1B\x9CWa\x1B\x9CaU\xCCV[`\x01`\x01`\x01`\xC0\x1B\x03\x85\x16\x91\x90\x93\x015`\xF8\x1C\x1C\x82\x16\x90\x91\x03\x90Pa\x1C\xC8W\x85`\x01`\x01`\xA0\x1B\x03\x16c\xDD\x98F\xB9\x8A\x8A\x85\x81\x81\x10a\x1B\xDDWa\x1B\xDDaU\xCCV[\x90P` \x02\x015\x8D\x8D\x88`\xFF\x16\x81\x81\x10a\x1B\xF9Wa\x1B\xF9aU\xCCV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x87\x90\x1B\x16\x81R`\x04\x81\x01\x94\x90\x94R\x91\x90\x91\x015`\xF8\x1C`$\x83\x01RPc\xFF\xFF\xFF\xFF\x8F\x16`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1COW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Cs\x91\x90aW\xF7V[\x85``\x01Q\x85`\xFF\x16\x81Q\x81\x10a\x1C\x8CWa\x1C\x8CaU\xCCV[` \x02` \x01\x01Q\x84\x81Q\x81\x10a\x1C\xA5Wa\x1C\xA5aU\xCCV[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x82a\x1C\xC4\x81aU\xFBV[\x93PP[P\x80a\x1C\xD3\x81aU\xFBV[\x91PPa\x1A\x19V[P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1C\xF6Wa\x1C\xF6aC/V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1D\x1FW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a\x1D\xA0W\x84``\x01Q\x84`\xFF\x16\x81Q\x81\x10a\x1DFWa\x1DFaU\xCCV[` \x02` \x01\x01Q\x81\x81Q\x81\x10a\x1D_Wa\x1D_aU\xCCV[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a\x1DyWa\x1DyaU\xCCV[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x1D\x98\x81aU\xFBV[\x91PPa\x1D%V[P\x80\x84``\x01Q\x84`\xFF\x16\x81Q\x81\x10a\x1D\xBBWa\x1D\xBBaU\xCCV[` \x02` \x01\x01\x81\x90RPPP\x80\x80a\x1D\xD3\x90aX\x14V[\x91PPa\x19\xA3V[P`\0\x89`\x01`\x01`\xA0\x1B\x03\x16c]\xF4YF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\x1CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E@\x91\x90aS\xC7V[`@Qc5IR\xA3`\xE2\x1B\x81R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xD5%J\x8C\x90a\x1Es\x90\x8B\x90\x8B\x90\x8E\x90`\x04\x01aX3V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\x90W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1E\xB8\x91\x90\x81\x01\x90aW V[` \x83\x01RP\x98\x97PPPPPPPPV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\x12W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F6\x91\x90aT.V[a\x1FRW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0Br\x90aTKV[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[```\0\x84`\x01`\x01`\xA0\x1B\x03\x16c\xC3\x91B^\x84\x86`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1F\xC3\x92\x91\x90aX]V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\xE0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra \x08\x91\x90\x81\x01\x90aW V[\x90P`\0\x84Q`\x01`\x01`@\x1B\x03\x81\x11\x15a %Wa %aC/V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a NW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x85Q\x81\x10\x15a!OW\x86`\x01`\x01`\xA0\x1B\x03\x16c\x04\xECcQ\x87\x83\x81Q\x81\x10a ~Wa ~aU\xCCV[` \x02` \x01\x01Q\x87\x86\x85\x81Q\x81\x10a \x99Wa \x99aU\xCCV[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a \xD6\x93\x92\x91\x90\x92\x83Rc\xFF\xFF\xFF\xFF\x91\x82\x16` \x84\x01R\x16`@\x82\x01R``\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a \xF3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\x17\x91\x90aW\xCEV[`\x01`\x01`\xC0\x1B\x03\x16\x82\x82\x81Q\x81\x10a!2Wa!2aU\xCCV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a!G\x81aU\xFBV[\x91PPa TV[P\x95\x94PPPPPV[`\x97T`@\x80Qc.\xFA,\xA3`\xE1\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c]\xF4YF\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a!\xA3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\xC7\x91\x90aS\xC7V[\x90P\x90V[a\x0C\xEDa4]V[`\x97T`@\x80Qch0H5`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91ch0H5\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a!\xA3W=`\0\x80>=`\0\xFD[`\x97T`@\x80Qcm\x14\xA9\x87`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cm\x14\xA9\x87\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a!\xA3W=`\0\x80>=`\0\xFD[`\x9FT`\x01`\x01`\xA0\x1B\x03\x163\x14a\"\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0Br\x90aT\x93V[`\xA0T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\"\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RsTask already pending``\x1B`D\x82\x01R`d\x01a\x0BrV[a\"\xEE\x83\x83\x83a:%V[PPPV[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`\x97T`@Qc7}\xA3\x1B`\xE1\x1B\x81R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cn\xFBF6\x90a#@\x90\x8A\x90\x8A\x90\x8A\x90\x8A\x90\x8A\x90`\x04\x01aY\0V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a#]W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra#\x85\x91\x90\x81\x01\x90aZ`V[\x91P\x91P\x95P\x95\x93PPPPV[a#\x9Ba4]V[a\rE`\0a=\rV[a#\xADa4]V[`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x90\x1AeM\xC80\xC9N\x8A\x12\xC9\xA3\xBC\n\x92\xAC\x11\xB5\xCF(\x04l\xA8\xD1\x90i\x1C\xDA\xF5 \x90\x16\x90` \x01a\x16\x82V[`fT\x15a$KW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FPausable: contract is paused\0\0\0\0`D\x82\x01R`d\x01a\x0BrV[`\x9ETd\x01\0\0\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x163\x14a$\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04\x17WF\x83`\xDC\x1B`D\x82\x01R`d\x01a\x0BrV[6`\0a$\xA5`\xC0\x86\x01\x86aZ\xFCV[\x90\x92P\x90P`\0a$\xBDa\x01\0\x87\x01`\xE0\x88\x01aS\x0CV[\x90P`\xA1`\0a$\xD3``\x88\x01`@\x89\x01aA\xEAV[`\x01`\x01`@\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 Tc\xFF\xFF\xFF\xFF\x16\x15\x80a%FWP`\xA1`\0a%\x0E``\x88\x01`@\x89\x01aA\xEAV[`\x01`\x01`@\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 Tc\xFF\xFF\xFF\xFF\x16a%>`\x80\x87\x01``\x88\x01aS\x0CV[c\xFF\xFF\xFF\xFF\x16\x14[a%\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FchainRdBatchNonce mismatch\0\0\0\0\0\0`D\x82\x01R`d\x01a\x0BrV[a%\xDE\x86`@Q` \x01a%\xA6\x91\x90a[BV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x91\x82\x01 \x90`\x01\x90a%\xCE\x90\x89\x01\x89aS\x0CV[a\x08)`\x80\x8B\x01``\x8C\x01aS\x0CV[`\0\x80a&&\x87`@Q` \x01a%\xF5\x91\x90a\\\xA4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 a&\x1D`\xC0\x8B\x01`\xA0\x8C\x01aS\x0CV[\x88\x88\x88\x88a=_V[\x90\x92P\x90Pa&8` \x89\x01\x89aS\x0CV[c\xFF\xFF\xFF\xFF\x16\x7F\x07H\x0B\xC0\x16'N\xBBe\x0C8G\xD7~\xB2\xA5\xAD\x17W9\x8BIXQ\x1AKu\xDCfo\xAE\xC0\x88\x83`@Qa&o\x92\x91\x90a\\\xB3V[`@Q\x80\x91\x03\x90\xA2\x81\x15a&\xA7Wa&\xA2`\x01a&\x8F` \x8A\x01\x8AaS\x0CV[\x89\x84`@Q` \x01a\n\x05\x92\x91\x90a\\\xB3V[a&\xF3V[a&\xE9`\x01a&\xB9` \x8A\x01\x8AaS\x0CV[\x89\x84`@Q` \x01a&\xCC\x92\x91\x90a\\\xB3V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\x03a4\xB7V[PPPPPPPPV[Fa'\x04``\x89\x01`@\x8A\x01aA\xEAV[`\x01`\x01`@\x1B\x03\x16\x03a'\x93W`@\x80Q\x80\x82\x01\x82R`\xA0\x89\x81\x015\x82R`\xC0\x8A\x015` \x83\x01RT\x91Qb#\xD0\xB5`\xE6\x1B\x81R\x90\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x08\xF4-@\x90a'_\x90`\x80\x8C\x015\x90\x85\x90`\x04\x01a\\\xD5V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a'yW`\0\x80\xFD[PZ\xF1\x15\x80\x15a'\x8DW=`\0\x80>=`\0\xFD[PPPPP[a'\xA3`\x80\x88\x01``\x89\x01aS\x0CV[a'\xAE\x90`\x01aU\xA4V[`\xA1`\0a'\xC2``\x8B\x01`@\x8C\x01aA\xEAV[`\x01`\x01`@\x1B\x03\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x93\x90\x93\x16\x92\x90\x92\x17\x90\x91Ua(\x01\x90\x89\x01\x89aS\x0CV[`\xA3\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16a\x01\0\x02d\xFF\xFF\xFF\xFF\0\x19\x90\x92\x16\x91\x90\x91\x17\x90Ua(2`\x80\x89\x01``\x8A\x01aS\x0CV[`\xA3\x80Tp\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\0\x19\x16e\x01\0\0\0\0\0c\xFF\xFF\xFF\xFF\x93\x84\x16\x02c\xFF\xFF\xFF\xFF`h\x1B\x19\x16\x17`\x01`h\x1BC\x93\x90\x93\x16\x92\x90\x92\x02\x91\x90\x91\x17\x90Ua(\x86` \x88\x01\x88aS\x0CV[c\xFF\xFF\xFF\xFF\x16\x7F\x88M\xD2\xB9\xADwz\xE0\xBC7\xBB\t\xE4>3\x90\xA9\xB9=^\x0C\xAC\xEA\x96r\x94v\x11\x16\xBB\x9A$\x88`@Qa(\xBB\x91\x90a\\\xA4V[`@Q\x80\x91\x03\x90\xA2PPPPPPPPV[`\xA0T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15\x15`\x01\x14a)\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01RnNo task pending`\x88\x1B`D\x82\x01R`d\x01a\x0BrV[`\x99`\0\x84`\x01\x81\x11\x15a)3Wa)3aO\xF0V[`\x01\x81\x11\x15a)DWa)DaO\xF0V[\x81R` \x01\x90\x81R` \x01`\0 `\0\x83c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x84\x14a)\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\n\x8C.md\r\xAD.m\xAC.\x8Cm`\x9B\x1B`D\x82\x01R`d\x01a\x0BrV[`\x01`\x9B`\0\x85`\x01\x81\x11\x15a)\xC6Wa)\xC6aO\xF0V[`\x01\x81\x11\x15a)\xD7Wa)\xD7aO\xF0V[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 c\xFF\xFF\xFF\xFF\x87\x16\x82R\x90\x92R\x90 T`\xFF\x16`\x04\x81\x11\x15a*\x0FWa*\x0FaO\xF0V[\x14a*MW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01RmNot Init state`\x90\x1B`D\x82\x01R`d\x01a\x0BrV[`\0`\x9A\x81\x85`\x01\x81\x11\x15a*dWa*daO\xF0V[`\x01\x81\x11\x15a*uWa*uaO\xF0V[\x81R` \x01\x90\x81R` \x01`\0 `\0\x84c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x14a*\xDBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01Ri\x04\x16\xC7&G\x92\x05&W7`\xB4\x1B`D\x82\x01R`d\x01a\x0BrV[`\x98Ta*\xF5\x90`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x82aU\xA4V[c\xFF\xFF\xFF\xFF\x16Cc\xFF\xFF\xFF\xFF\x16\x11\x15a+;W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x08`$\x82\x01RgToo late`\xC0\x1B`D\x82\x01R`d\x01a\x0BrV[PPPPV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91``\x91\x83\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x84\x81`\0\x81Q\x81\x10a+|Wa+|aU\xCCV[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Qca\xC8\xA1/`\xE1\x1B\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90c\xC3\x91B^\x90a+\xB8\x90\x88\x90\x86\x90`\x04\x01aX]V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+\xD5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra+\xFD\x91\x90\x81\x01\x90aW V[`\0\x81Q\x81\x10a,\x0FWa,\x0FaU\xCCV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x88\x90Rc\xFF\xFF\xFF\xFF\x87\x81\x16`$\x83\x01R\x90\x91\x16`D\x82\x01\x81\x90R\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x89\x16\x90c\x04\xECcQ\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,{W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\x9F\x91\x90aW\xCEV[`\x01`\x01`\xC0\x1B\x03\x16\x90P`\0a,\xB5\x82a>\xD5V[\x90P\x81a,\xC3\x8A\x83\x8Aa\x11\x98V[\x95P\x95PPPPP\x93P\x93\x91PPV[`fT\x15a-#W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FPausable: contract is paused\0\0\0\0`D\x82\x01R`d\x01a\x0BrV[`\x9ETd\x01\0\0\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x163\x14a-mW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04\x17WF\x83`\xDC\x1B`D\x82\x01R`d\x01a\x0BrV[`\x9CT`\x01``\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15\x15\x80a-\x8CWP`\xA3T`\xFF\x16[a-\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x1D\\\xD9H\x1C\x9B\xDB\xDD\x08\x1A[\x9A]`\x9A\x1B`D\x82\x01R`d\x01a\x0BrV[6`\0a-\xD8`\xC0\x86\x01\x86aZ\xFCV[\x90\x92P\x90P`\0a-\xF0a\x01\0\x87\x01`\xE0\x88\x01aS\x0CV[\x90Pa.>\x86`@Q` \x01a.\x06\x91\x90aR\x1BV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x91\x82\x01 \x90`\0\x90a..\x90\x89\x01\x89aS\x0CV[a\x08)`@\x8B\x01` \x8C\x01aS\x0CV[`\0\x80a.}\x87`@Q` \x01a.U\x91\x90aS\xB9V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 a&\x1D`\x80\x8B\x01``\x8C\x01aS\x0CV[\x90\x92P\x90Pa.\x8F` \x89\x01\x89aS\x0CV[c\xFF\xFF\xFF\xFF\x16\x7FG\xAD\xAC\xB0\xB6\xBB\xD7&\xAE9\xACl\0l\xCA\x1C \x06\xC9\xAE\xDA\xA8\x82\xDC\xBA|H\x04\xDB|A\xCE\x88\x83`@Qa.\xC6\x92\x91\x90aS\x99V[`@Q\x80\x91\x03\x90\xA2\x81\x15a.\xFEWa.\xF9`\0a.\xE6` \x8A\x01\x8AaS\x0CV[\x89\x84`@Q` \x01a\n\x05\x92\x91\x90aS\x99V[a/#V[a&\xE9`\0a/\x10` \x8A\x01\x8AaS\x0CV[\x89\x84`@Q` \x01a&\xCC\x92\x91\x90aS\x99V[a/1\x87`@\x015\x89a5\x8BV[a/>` \x88\x01\x88aS\x0CV[c\xFF\xFF\xFF\xFF\x16\x7F\xFF)\x08H=t\xB6\xB7\0S\xDDG2`\xAC\xF1\xB0\x9E\x0B\xA0x\x1B\xF9A\0\xBB\x82wX\x17I\xDE\x88`@Qa(\xBB\x91\x90aS\xB9V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a/\x93WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a/\xADWP0;\x15\x80\x15a/\xADWP`\0T`\xFF\x16`\x01\x14[a0\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x0BrV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a03W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a0>\x8A`\0a?\xA1V[a0G\x89a=\rV[`\x9E\x80Td\x01\0\0\0\0`\x01`\xC0\x1B\x03\x19\x16d\x01\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x8B\x81\x16\x91\x90\x91\x02\x91\x90\x91\x17\x90\x91U`\x9F\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x8A\x84\x16\x17\x90\x91U`\xA3\x80T`\xFF\x19\x16\x89\x15\x15\x17\x90U`\x97\x80T\x82\x16\x88\x84\x16\x17\x90U`\x98\x80T\x86\x84\x16`\x01`\x01`\xC0\x1B\x03\x19\x90\x91\x16\x17`\x01`\xA0\x1Bc\xFF\xFF\xFF\xFF\x89\x16\x02\x17\x90U`\xA0\x80T\x90\x91\x16\x91\x84\x16\x91\x90\x91\x17\x90U\x80\x15a1 W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPPPPV[`\x97T`@\x80Qc\xDF\\\xF7#`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xDF\\\xF7#\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a!\xA3W=`\0\x80>=`\0\xFD[a1~a4]V[`\x01`\x01`\xA0\x1B\x03\x81\x16a1\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x0BrV[a\x06\x0B\x81a=\rV[a1\xF4a4]V[`\xA0T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a2\x0EWa2\x0Ea7\\V[a2\x19\x83\x83\x83a:%V[`@Q\x7FN\xE9\x87\xE5\xF1\xBE\x19\xCA\xBF\xB1\xA2C\xE5\xC4#\x88\x9F\x06\x0F3&gS\x95?\xF0\xCF\x9D\xB8\x99f\xAB\x90`\0\x90\xA1PPPV[a2Oa4]V[`\x9E\x80Td\x01\0\0\0\0`\x01`\xC0\x1B\x03\x19\x16d\x01\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x81\x02\x91\x90\x91\x17\x90\x91U`@Q\x90\x81R\x7F`,\xECK\x15\x83\xB0}\x07\x11a\xDA^\xB9X\x94D\xD2E\x92\x01\xE2\xFA\xB7u=\xC9A\xE95\x1C!\x90` \x01a\x16\x82V[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a2\xFEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3\"\x91\x90aS\xC7V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a3RW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0Br\x90aS\xE4V[`fT\x19\x81\x19`fT\x19\x16\x14a3\xD0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0BrV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x0C\xB8V[a4\x0Fa4]V[`\xA0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F/ \xCF\x1B\xDAgs\x90D\xC5\xBFWsS\x97\x0C=\xBC\x18;,rt\xD1\xE8XJ\x10&\x922g\x90` \x01a\x16\x82V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\rEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x0BrV[\x81`\x9A`\0\x86`\x01\x81\x11\x15a4\xCEWa4\xCEaO\xF0V[`\x01\x81\x11\x15a4\xDFWa4\xDFaO\xF0V[\x81R` \x01\x90\x81R` \x01`\0 `\0\x85c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP\x80`\x9B`\0\x86`\x01\x81\x11\x15a5%Wa5%aO\xF0V[`\x01\x81\x11\x15a56Wa56aO\xF0V[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 c\xFF\xFF\xFF\xFF\x88\x16\x82R\x90\x92R\x90 \x80T`\xFF\x19\x16`\x01\x83`\x04\x81\x11\x15a5sWa5saO\xF0V[\x02\x17\x90UPP`\xA0\x80T`\xFF`\xA0\x1B\x19\x16\x90UPPPV[`\xA2\x82\x90Ua5\xA0`@\x82\x01` \x83\x01aS\x0CV[`\x9C\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x01``\x1B\x02c\xFF\xFF\xFF\xFF``\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90Ua5\xD2`\x80\x82\x01\x82aZ\xFCV[a5\xDE\x91`\x9D\x91a@\xBCV[Pa5\xEF`\xC0\x82\x01`\xA0\x83\x01aS\x0CV[`\x9E\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua6\x16` \x82\x01\x82aS\x0CV[`\x9C\x80Tk\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x16`\x01`@\x1Bc\xFF\xFF\xFF\xFF\x93\x84\x16\x02\x17\x90U`\xA3\x80Tl\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\x19\x16`\x01`H\x1BC\x90\x93\x16\x92\x90\x92\x02\x91\x90\x91\x17\x90UPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16a6\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x0BrV[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x98T`\x01`\xC0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15a8LW`\x98T`\0\x90a7\x90\x90`\x01\x90`\x01`\xC0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16a\\\xF3V[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R\x7F\x10\xAF\xAC\x923\xB4\xCC\xC5Md\x04\xFF\xC1\xCF;GQZ+\x8E\xDB\xF6u\xD1^\xDD\xCE\x05\xA0'\xDC\xBD` R`@\x90 T\x90\x91P`\x01\x90`\xFF\x16`\x04\x81\x11\x15a7\xDFWa7\xDFaO\xF0V[\x03a8JWc\xFF\xFF\xFF\xFF\x81\x16`\0\x81\x81R\x7F\x10\xAF\xAC\x923\xB4\xCC\xC5Md\x04\xFF\xC1\xCF;GQZ+\x8E\xDB\xF6u\xD1^\xDD\xCE\x05\xA0'\xDC\xBD` R`@\x80\x82 \x80T`\xFF\x19\x16`\x02\x17\x90UQ\x7F\xD6\xA4\xE0\xFF\x9F:\x057\x08u|z\x12J\xBE\xE3\x1C\xEDa\xF4?\x17\xE6\xE1\xCF\x11\x94>\xC5\x9E`q\x91\x90\xA2[P[`\x98T`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15a9<W`\x98T`\0\x90a8\x80\x90`\x01\x90`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16a\\\xF3V[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R\x7F)\x8C\x80\r\x08\x81\xDD \x8Dp^\xBC\x03\xEB\x18\x18\x9F8\x11\x82Y\xF2}\xD4;L`\xD6\x1C`~\x87` R`@\x90 T\x90\x91P`\x01\x90`\xFF\x16`\x04\x81\x11\x15a8\xCFWa8\xCFaO\xF0V[\x03a9:Wc\xFF\xFF\xFF\xFF\x81\x16`\0\x81\x81R\x7F)\x8C\x80\r\x08\x81\xDD \x8Dp^\xBC\x03\xEB\x18\x18\x9F8\x11\x82Y\xF2}\xD4;L`\xD6\x1C`~\x87` R`@\x80\x82 \x80T`\xFF\x19\x16`\x02\x17\x90UQ\x7F\x0B\xF4k\xFC\xA6\xE2\x13}5\xB8\x93\xC2\x95\xAD\xD8\xC3;\xCF\xBF\xFA\xFD\xEF\x93%,\xB5\x1A\xEDu8\xBA\x0C\x91\x90\xA2[P[`\xA0\x80T`\xFF`\xA0\x1B\x19\x16\x90UV[\x81`\x99`\0\x86`\x01\x81\x11\x15a9bWa9baO\xF0V[`\x01\x81\x11\x15a9sWa9saO\xF0V[\x81R` \x01\x90\x81R` \x01`\0 `\0\x85c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP\x80`\x9B`\0\x86`\x01\x81\x11\x15a9\xB9Wa9\xB9aO\xF0V[`\x01\x81\x11\x15a9\xCAWa9\xCAaO\xF0V[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 c\xFF\xFF\xFF\xFF\x88\x16\x82R\x90\x92R\x90 \x80T`\xFF\x19\x16`\x01\x83`\x04\x81\x11\x15a:\x07Wa:\x07aO\xF0V[\x02\x17\x90UPP`\xA0\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x17\x90UPPPV[`\x9CTc\xFF\xFF\xFF\xFF`\x01``\x1B\x90\x91\x04\x16C\x14\x80\x15\x90a:DWPC\x15\x15[a:\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FCan't in lastCompletedOpTaskCrea`D\x82\x01RgtedBlock`\xC0\x1B`d\x82\x01R`\x84\x01a\x0BrV[`\x98T`@\x80Qa\x01\0\x81\x01\x82R`\0\x81\x83\x01\x81\x90R``\x80\x83\x01\x82\x90R`\x80\x83\x01\x81\x90R`\xC0\x83\x01R`\xE0\x82\x01Rc\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x90\x93\x04\x83\x16\x80\x82RC\x84\x16` \x80\x84\x01\x91\x90\x91R\x93\x87\x16`\xA0\x83\x01R\x82Q`\x1F\x86\x01\x85\x90\x04\x85\x02\x81\x01\x85\x01\x90\x93R\x84\x83R\x92\x90\x91\x90\x85\x90\x85\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x82\x90RP`\x80\x86\x01\x94\x90\x94RPP`\x9CT`\x01``\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x90\x91\x03\x90Pa;\xAAWc\xFF\xFF\xFF\xFF\x80\x83\x16`@\x80\x84\x01\x91\x90\x91RC\x90\x91\x16``\x83\x01R\x80Q` `\x1F\x86\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x84\x81R\x90\x85\x90\x85\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPPP`\xC0\x82\x01Rc\xFF\xFF\xFF\xFF\x85\x16`\xE0\x82\x01Ra<mV[`\x9CTc\xFF\xFF\xFF\xFF`\x01`@\x1B\x82\x04\x81\x16`@\x84\x01R`\x01``\x1B\x90\x91\x04\x16``\x82\x01R`\x9D\x80Ta;\xDB\x90aT\xB2V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta<\x07\x90aT\xB2V[\x80\x15a<TW\x80`\x1F\x10a<)Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a<TV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a<7W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`\xC0\x82\x01R`\x9ETc\xFF\xFF\xFF\xFF\x16`\xE0\x82\x01R[a<\x84`\0\x83\x83`@Q` \x01a\x0FA\x91\x90a]\x18V[`\x9C\x80Tc\xFF\xFF\xFF\xFF\x19\x16Cc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x90\x91\x17\x90\x91U`@Q\x90\x83\x16\x90\x7F\xF3bg'\xFAn5e\xD6\xB23\xE2\x13l.\xE4R\xCF\x96\xE6@OQ\xD8\xCA\xA3G\xDD/{\x91\xF8\x90a<\xD3\x90\x84\x90a]\x18V[`@Q\x80\x91\x03\x90\xA2a<\xE6\x82`\x01aU\xA4V[`\x98`\x18a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPPPV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R``\x82\x84\x01\x81\x90R\x80\x83\x01R`\x97T\x92Qc7}\xA3\x1B`\xE1\x1B\x81R\x90\x92\x83\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cn\xFBF6\x90a=\xBC\x90\x8D\x90\x8B\x90\x8B\x90\x8F\x90\x8F\x90`\x04\x01aY\0V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a=\xD9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra>\x01\x91\x90\x81\x01\x90aZ`V[`@\x80Q`\x80\x81\x01\x82Rc\xFF\xFF\xFF\xFFC\x16\x81R` \x80\x82\x01\x84\x90R\x84\x01Q\x91\x81\x01\x91\x90\x91R\x82Q``\x82\x01R\x91\x93P\x91P`\x01`\0[\x88\x81\x10\x15a>\xC4W\x87`\xFF\x16\x85` \x01Q\x82\x81Q\x81\x10a>YWa>YaU\xCCV[` \x02` \x01\x01Qa>k\x91\x90a]\xD1V[`\x01`\x01``\x1B\x03\x16`d\x86`\0\x01Q\x83\x81Q\x81\x10a>\x8CWa>\x8CaU\xCCV[` \x02` \x01\x01Q`\x01`\x01``\x1B\x03\x16a>\xA7\x91\x90a^\0V[\x10\x15a>\xB2W`\0\x91P[\x80a>\xBC\x81aU\xFBV[\x91PPa>7V[P\x9B\x90\x9AP\x98PPPPPPPPPV[```\0\x80a>\xE3\x84a@\x8BV[a\xFF\xFF\x16`\x01`\x01`@\x1B\x03\x81\x11\x15a>\xFEWa>\xFEaC/V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a?(W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x80[\x82Q\x82\x10\x80\x15a?@WPa\x01\0\x81\x10[\x15a?\x97W`\x01\x81\x1B\x93P\x85\x84\x16\x15a?\x87W\x80`\xF8\x1B\x83\x83\x81Q\x81\x10a?iWa?iaU\xCCV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x81`\x01\x01\x91P[a?\x90\x81aU\xFBV[\x90Pa?/V[P\x90\x94\x93PPPPV[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a?\xC2WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a@DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x0BrV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a@\x87\x82a6eV[PPV[`\0\x80[\x82\x15a@\xB6Wa@\xA0`\x01\x84a^\x1FV[\x90\x92\x16\x91\x80a@\xAE\x81a^6V[\x91PPa@\x8FV[\x92\x91PPV[\x82\x80Ta@\xC8\x90aT\xB2V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82a@\xEAW`\0\x85UaA0V[\x82`\x1F\x10aA\x03W\x82\x80\x01`\xFF\x19\x825\x16\x17\x85UaA0V[\x82\x80\x01`\x01\x01\x85U\x82\x15aA0W\x91\x82\x01[\x82\x81\x11\x15aA0W\x825\x82U\x91` \x01\x91\x90`\x01\x01\x90aA\x15V[PaA<\x92\x91PaA@V[P\x90V[[\x80\x82\x11\x15aA<W`\0\x81U`\x01\x01aAAV[`\0a\x01\0\x82\x84\x03\x12\x15aAhW`\0\x80\xFD[P\x91\x90PV[`\0``\x82\x84\x03\x12\x15aAhW`\0\x80\xFD[`\0\x80`\x80\x83\x85\x03\x12\x15aA\x93W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aA\xA9W`\0\x80\xFD[aA\xB5\x85\x82\x86\x01aAUV[\x92PPaA\xC5\x84` \x85\x01aAnV[\x90P\x92P\x92\x90PV[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14aA\xE5W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15aA\xFCW`\0\x80\xFD[a\x16)\x82aA\xCEV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06\x0BW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aB,W`\0\x80\xFD[\x815a\x16)\x81aB\x05V[`\0` \x82\x84\x03\x12\x15aBIW`\0\x80\xFD[P5\x91\x90PV[\x805`\x02\x81\x10aA\xE5W`\0\x80\xFD[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x06\x0BW`\0\x80\xFD[\x805aA\xE5\x81aB_V[`\0\x80`@\x83\x85\x03\x12\x15aB\x8FW`\0\x80\xFD[aB\x98\x83aBPV[\x91P` \x83\x015aB\xA8\x81aB_V[\x80\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15aB\xC6W`\0\x80\xFD[aB\x98\x83aA\xCEV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15aB\xF5W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01aB\xD9V[\x81\x81\x11\x15aC\x07W`\0` \x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x16)` \x83\x01\x84aB\xCFV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aCgWaCgaC/V[`@R\x90V[`@Qa\x01\0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aCgWaCgaC/V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aC\xB8WaC\xB8aC/V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15aC\xD9WaC\xD9aC/V[P`\x05\x1B` \x01\x90V[`\0\x80`@\x83\x85\x03\x12\x15aC\xF6W`\0\x80\xFD[\x825aD\x01\x81aB\x05V[\x91P` \x83\x81\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aD\x1DW`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13aD.W`\0\x80\xFD[\x805aDAaD<\x82aC\xC0V[aC\x90V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x88\x83\x11\x15aD`W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aD\x87W\x835aDx\x81aB\x05V[\x82R\x92\x84\x01\x92\x90\x84\x01\x90aDeV[\x80\x95PPPPPP\x92P\x92\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aD\xC6W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aD\xAAV[P\x94\x95\x94PPPPPV[` \x81R`\0a\x16)` \x83\x01\x84aD\x96V[`\0\x80`\0``\x84\x86\x03\x12\x15aD\xF9W`\0\x80\xFD[\x835aE\x04\x81aB\x05V[\x92P` \x84\x81\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aE!W`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12aE5W`\0\x80\xFD[\x815\x81\x81\x11\x15aEGWaEGaC/V[aEY`\x1F\x82\x01`\x1F\x19\x16\x85\x01aC\x90V[\x91P\x80\x82R\x88\x84\x82\x85\x01\x01\x11\x15aEoW`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x94PPPPaE\x92`@\x85\x01aBqV[\x90P\x92P\x92P\x92V[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0\x80[\x86\x81\x10\x15aF1W\x83\x85\x03\x8AR\x82Q\x80Q\x80\x87R\x90\x87\x01\x90\x87\x87\x01\x90\x84[\x81\x81\x10\x15aF\x1CW\x83Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x8A\x81\x01Q\x8B\x85\x01R`@\x90\x81\x01Q`\x01`\x01``\x1B\x03\x16\x90\x84\x01R\x92\x89\x01\x92``\x90\x92\x01\x91`\x01\x01aE\xD8V[PP\x9A\x87\x01\x9A\x95PP\x91\x85\x01\x91`\x01\x01aE\xBAV[P\x92\x98\x97PPPPPPPPV[` \x81R`\0a\x16)` \x83\x01\x84aE\x9BV[`\0` \x82\x84\x03\x12\x15aFdW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aFzW`\0\x80\xFD[\x82\x01`@\x81\x85\x03\x12\x15a\x16)W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12aF\x9DW`\0\x80\xFD[\x815` aF\xADaD<\x83aC\xC0V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aF\xCCW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aF\xE7W\x805\x83R\x91\x83\x01\x91\x83\x01aF\xD0V[P\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15aG\x05W`\0\x80\xFD[\x825aG\x10\x81aB\x05V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aG+W`\0\x80\xFD[aG7\x85\x82\x86\x01aF\x8CV[\x91PP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aG\x82W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aG]V[P\x90\x96\x95PPPPPPV[`\0\x80\x83`\x1F\x84\x01\x12aG\xA0W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aG\xB7W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aG\xCFW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\x80\x87\x89\x03\x12\x15aG\xEFW`\0\x80\xFD[\x865aG\xFA\x81aB\x05V[\x95P` \x87\x015aH\n\x81aB_V[\x94P`@\x87\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aH&W`\0\x80\xFD[aH2\x8A\x83\x8B\x01aG\x8EV[\x90\x96P\x94P``\x89\x015\x91P\x80\x82\x11\x15aHKW`\0\x80\xFD[\x81\x89\x01\x91P\x89`\x1F\x83\x01\x12aH_W`\0\x80\xFD[\x815\x81\x81\x11\x15aHnW`\0\x80\xFD[\x8A` \x82`\x05\x1B\x85\x01\x01\x11\x15aH\x83W`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92\x95P\x92\x95P\x92\x95V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aD\xC6W\x81Qc\xFF\xFF\xFF\xFF\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aH\xADV[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0[\x85\x81\x10\x15aI\x17W\x82\x84\x03\x89RaI\x05\x84\x83QaH\x99V[\x98\x85\x01\x98\x93P\x90\x84\x01\x90`\x01\x01aH\xEDV[P\x91\x97\x96PPPPPPPV[` \x81R`\0\x82Q`\x80` \x84\x01RaI@`\xA0\x84\x01\x82aH\x99V[\x90P` \x84\x01Q`\x1F\x19\x80\x85\x84\x03\x01`@\x86\x01RaI^\x83\x83aH\x99V[\x92P`@\x86\x01Q\x91P\x80\x85\x84\x03\x01``\x86\x01RaI{\x83\x83aH\x99V[\x92P``\x86\x01Q\x91P\x80\x85\x84\x03\x01`\x80\x86\x01RPaI\x99\x82\x82aH\xCFV[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15aI\xB4W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aI\xCAW`\0\x80\xFD[\x82\x01a\x01 \x81\x85\x03\x12\x15a\x16)W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aI\xEFW`\0\x80\xFD[\x815`\xFF\x81\x16\x81\x14a\x16)W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15aJ\x15W`\0\x80\xFD[\x835aJ \x81aB\x05V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aJ;W`\0\x80\xFD[aJG\x86\x82\x87\x01aF\x8CV[\x92PP`@\x84\x015aJX\x81aB_V[\x80\x91PP\x92P\x92P\x92V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aG\x82W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aJ\x7FV[`\0\x80`\0`@\x84\x86\x03\x12\x15aJ\xB0W`\0\x80\xFD[\x835aJ\xBB\x81aB_V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aJ\xD6W`\0\x80\xFD[aJ\xE2\x86\x82\x87\x01aG\x8EV[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x82`\x1F\x83\x01\x12aK\0W`\0\x80\xFD[\x815` aK\x10aD<\x83aC\xC0V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aK/W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aF\xE7W\x805aKF\x81aB_V[\x83R\x91\x83\x01\x91\x83\x01aK3V[`\0`@\x82\x84\x03\x12\x15aKeW`\0\x80\xFD[aKmaCEV[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12aK\x94W`\0\x80\xFD[\x815` aK\xA4aD<\x83aC\xC0V[\x82\x81R`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aK\xC3W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aF\xE7WaK\xD9\x88\x82aKSV[\x83R\x91\x83\x01\x91`@\x01aK\xC7V[`\0\x82`\x1F\x83\x01\x12aK\xF8W`\0\x80\xFD[aL\0aCEV[\x80`@\x84\x01\x85\x81\x11\x15aL\x12W`\0\x80\xFD[\x84[\x81\x81\x10\x15aL,W\x805\x84R` \x93\x84\x01\x93\x01aL\x14V[P\x90\x95\x94PPPPPV[`\0`\x80\x82\x84\x03\x12\x15aLIW`\0\x80\xFD[aLQaCEV[\x90PaL]\x83\x83aK\xE7V[\x81RaLl\x83`@\x84\x01aK\xE7V[` \x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12aL\x88W`\0\x80\xFD[\x815` aL\x98aD<\x83aC\xC0V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aL\xB7W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aF\xE7W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aL\xDAW`\0\x80\x81\xFD[aL\xE8\x89\x86\x83\x8B\x01\x01aJ\xEFV[\x84RP\x91\x83\x01\x91\x83\x01aL\xBBV[`\0a\x01\x80\x82\x84\x03\x12\x15aM\tW`\0\x80\xFD[aM\x11aCmV[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aM*W`\0\x80\xFD[aM6\x85\x83\x86\x01aJ\xEFV[\x83R` \x84\x015\x91P\x80\x82\x11\x15aMLW`\0\x80\xFD[aMX\x85\x83\x86\x01aK\x83V[` \x84\x01R`@\x84\x015\x91P\x80\x82\x11\x15aMqW`\0\x80\xFD[aM}\x85\x83\x86\x01aK\x83V[`@\x84\x01RaM\x8F\x85``\x86\x01aL7V[``\x84\x01RaM\xA1\x85`\xE0\x86\x01aKSV[`\x80\x84\x01Ra\x01 \x84\x015\x91P\x80\x82\x11\x15aM\xBBW`\0\x80\xFD[aM\xC7\x85\x83\x86\x01aJ\xEFV[`\xA0\x84\x01Ra\x01@\x84\x015\x91P\x80\x82\x11\x15aM\xE1W`\0\x80\xFD[aM\xED\x85\x83\x86\x01aJ\xEFV[`\xC0\x84\x01Ra\x01`\x84\x015\x91P\x80\x82\x11\x15aN\x07W`\0\x80\xFD[PaN\x14\x84\x82\x85\x01aLwV[`\xE0\x83\x01RP\x92\x91PPV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15aN8W`\0\x80\xFD[\x855\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aNVW`\0\x80\xFD[aNb\x89\x83\x8A\x01aG\x8EV[\x90\x96P\x94P`@\x88\x015\x91PaNw\x82aB_V[\x90\x92P``\x87\x015\x90\x80\x82\x11\x15aN\x8DW`\0\x80\xFD[PaN\x9A\x88\x82\x89\x01aL\xF6V[\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aD\xC6W\x81Q`\x01`\x01``\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aN\xBBV[`@\x81R`\0\x83Q`@\x80\x84\x01RaN\xFB`\x80\x84\x01\x82aN\xA7V[\x90P` \x85\x01Q`?\x19\x84\x83\x03\x01``\x85\x01RaO\x18\x82\x82aN\xA7V[\x92PPP\x82` \x83\x01R\x93\x92PPPV[`\0\x80`\0a\x01@\x84\x86\x03\x12\x15aO?W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aOVW`\0\x80\xFD[aOb\x87\x83\x88\x01aAUV[\x94PaOq\x87` \x88\x01aAUV[\x93Pa\x01 \x86\x015\x91P\x80\x82\x11\x15aO\x88W`\0\x80\xFD[PaO\x95\x86\x82\x87\x01aL\xF6V[\x91PP\x92P\x92P\x92V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aO\xB5W`\0\x80\xFD[\x845\x93PaO\xC5` \x86\x01aBPV[\x92P`@\x85\x015aO\xD5\x81aB_V[\x91P``\x85\x015aO\xE5\x81aB_V[\x93\x96\x92\x95P\x90\x93PPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x05\x83\x10aP(WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15aPCW`\0\x80\xFD[\x835aPN\x81aB\x05V[\x92P` \x84\x015\x91P`@\x84\x015aJX\x81aB_V[\x82\x81R`@` \x82\x01R`\0aP~`@\x83\x01\x84aE\x9BV[\x94\x93PPPPV[`\0\x80`\0`\xA0\x84\x86\x03\x12\x15aP\x9BW`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aP\xB2W`\0\x80\xFD[aP\xBE\x87\x83\x88\x01aAUV[\x94PaP\xCD\x87` \x88\x01aAnV[\x93P`\x80\x86\x015\x91P\x80\x82\x11\x15aO\x88W`\0\x80\xFD[\x80\x15\x15\x81\x14a\x06\x0BW`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0\x80`\0a\x01 \x8A\x8C\x03\x12\x15aQ\x10W`\0\x80\xFD[\x895aQ\x1B\x81aB\x05V[\x98P` \x8A\x015aQ+\x81aB\x05V[\x97P`@\x8A\x015aQ;\x81aB\x05V[\x96P``\x8A\x015aQK\x81aB\x05V[\x95P`\x80\x8A\x015aQ[\x81aP\xE3V[\x94P`\xA0\x8A\x015aQk\x81aB\x05V[\x93P`\xC0\x8A\x015aQ{\x81aB_V[\x92P`\xE0\x8A\x015aQ\x8B\x81aB\x05V[\x91Pa\x01\0\x8A\x015aQ\x9C\x81aB\x05V[\x80\x91PP\x92\x95\x98P\x92\x95\x98P\x92\x95\x98V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aQ\xC4W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aQ\xE3W`\0\x80\xFD[\x806\x03\x83\x13\x15aG\xCFW`\0\x80\xFD[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x81R`\0\x825aR,\x81aB_V[c\xFF\xFF\xFF\xFF\x81\x16` \x84\x01RPaRE` \x84\x01aBqV[c\xFF\xFF\xFF\xFF\x81\x16`@\x84\x01RPaR^`@\x84\x01aBqV[c\xFF\xFF\xFF\xFF\x81\x16``\x84\x01RPaRw``\x84\x01aBqV[c\xFF\xFF\xFF\xFF\x81\x16`\x80\x84\x01RPaR\x91`\x80\x84\x01\x84aQ\xADV[a\x01\0\x80`\xA0\x86\x01RaR\xA9a\x01 \x86\x01\x83\x85aQ\xF2V[\x92PaR\xB7`\xA0\x87\x01aBqV[c\xFF\xFF\xFF\xFF\x81\x16`\xC0\x87\x01R\x91PaR\xD2`\xC0\x87\x01\x87aQ\xADV[\x86\x85\x03`\x1F\x19\x01`\xE0\x88\x01R\x92PaR\xEB\x84\x84\x83aQ\xF2V[\x93PPaR\xFA`\xE0\x87\x01aBqV[c\xFF\xFF\xFF\xFF\x81\x16\x86\x83\x01R\x91Pa?\x97V[`\0` \x82\x84\x03\x12\x15aS\x1EW`\0\x80\xFD[\x815a\x16)\x81aB_V[\x805aS4\x81aB_V[c\xFF\xFF\xFF\xFF\x16\x82R` \x81\x81\x015\x90\x83\x01R`@\x90\x81\x015\x91\x01RV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Q` \x83\x01R`\0`@\x82\x01Q`\x80`@\x85\x01RaS\x80`\x80\x85\x01\x82aN\xA7V[\x90P``\x83\x01Q\x84\x82\x03``\x86\x01RaI\x99\x82\x82aN\xA7V[aS\xA3\x81\x84aS)V[`\x80``\x82\x01R`\0aP~`\x80\x83\x01\x84aSQV[``\x81\x01a@\xB6\x82\x84aS)V[`\0` \x82\x84\x03\x12\x15aS\xD9W`\0\x80\xFD[\x81Qa\x16)\x81aB\x05V[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15aT@W`\0\x80\xFD[\x81Qa\x16)\x81aP\xE3V[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`\x05\x90\x82\x01RdAuth1`\xD8\x1B`@\x82\x01R``\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80aT\xC6W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aAhWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[` \x81Rc\xFF\xFF\xFF\xFF\x82Q\x16` \x82\x01R`\x01`\x01`@\x1B\x03` \x83\x01Q\x16`@\x82\x01R`\0`@\x83\x01QaU#``\x84\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P``\x83\x01Qc\xFF\xFF\xFF\xFF\x81\x16`\x80\x84\x01RP`\x80\x83\x01Qc\xFF\xFF\xFF\xFF\x81\x16`\xA0\x84\x01RP`\xA0\x83\x01Qc\xFF\xFF\xFF\xFF\x81\x16`\xC0\x84\x01RP`\xC0\x83\x01Qa\x01\0\x80`\xE0\x85\x01RaUva\x01 \x85\x01\x83aB\xCFV[\x91P`\xE0\x85\x01Qa?\x97\x82\x86\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aU\xC3WaU\xC3aU\x8EV[\x01\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15aU\xF4W`\0\x80\xFD[PQ\x91\x90PV[`\0`\x01\x82\x01aV\rWaV\raU\x8EV[P`\x01\x01\x90V[`\0` \x80\x83\x85\x03\x12\x15aV'W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aV=W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aVNW`\0\x80\xFD[\x80QaV\\aD<\x82aC\xC0V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15aV{W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aV\x99W\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90aV\x80V[\x97\x96PPPPPPPV[\x80Q`\x01`\x01``\x1B\x03\x81\x16\x81\x14aA\xE5W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aV\xCDW`\0\x80\xFD[a\x16)\x82aV\xA4V[c\xFF\xFF\xFF\xFF\x84\x16\x81R`@` \x82\x01\x81\x90R\x81\x01\x82\x90R`\0`\x01`\x01`\xFB\x1B\x03\x83\x11\x15aW\x03W`\0\x80\xFD[\x82`\x05\x1B\x80\x85``\x85\x017`\0\x92\x01``\x01\x91\x82RP\x93\x92PPPV[`\0` \x80\x83\x85\x03\x12\x15aW3W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aWIW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aWZW`\0\x80\xFD[\x80QaWhaD<\x82aC\xC0V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15aW\x87W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aV\x99W\x83QaW\x9F\x81aB_V[\x82R\x92\x84\x01\x92\x90\x84\x01\x90aW\x8CV[c\xFF\xFF\xFF\xFF\x84\x16\x81R`@` \x82\x01R`\0aI\x99`@\x83\x01\x84\x86aQ\xF2V[`\0` \x82\x84\x03\x12\x15aW\xE0W`\0\x80\xFD[\x81Q`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x14a\x16)W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aX\tW`\0\x80\xFD[\x81Qa\x16)\x81aB_V[`\0`\xFF\x82\x16`\xFF\x81\x03aX*WaX*aU\x8EV[`\x01\x01\x92\x91PPV[`@\x81R`\0aXG`@\x83\x01\x85\x87aQ\xF2V[\x90Pc\xFF\xFF\xFF\xFF\x83\x16` \x83\x01R\x94\x93PPPPV[c\xFF\xFF\xFF\xFF\x83\x16\x81R`@` \x82\x01R`\0aP~`@\x83\x01\x84aD\x96V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aD\xC6WaX\xAD\x87\x83Q\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[`@\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01aX\x90V[\x80`\0[`\x02\x81\x10\x15a+;W\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01aX\xC4V[aX\xEE\x82\x82QaX\xC0V[` \x81\x01Qa\"\xEE`@\x84\x01\x82aX\xC0V[\x85\x81R`\x80` \x82\x01R`\0aY\x1A`\x80\x83\x01\x86\x88aQ\xF2V[c\xFF\xFF\xFF\xFF\x85\x16`@\x84\x01R\x82\x81\x03``\x84\x01Ra\x01\x80\x84Q\x81\x83RaYB\x82\x84\x01\x82aH\x99V[\x91PP` \x85\x01Q\x82\x82\x03` \x84\x01RaY\\\x82\x82aX|V[\x91PP`@\x85\x01Q\x82\x82\x03`@\x84\x01RaYv\x82\x82aX|V[\x91PP``\x85\x01QaY\x8B``\x84\x01\x82aX\xE3V[P`\x80\x85\x01Q\x80Q`\xE0\x84\x01R` \x01Qa\x01\0\x83\x01R`\xA0\x85\x01Q\x82\x82\x03a\x01 \x84\x01RaY\xBA\x82\x82aH\x99V[\x91PP`\xC0\x85\x01Q\x82\x82\x03a\x01@\x84\x01RaY\xD5\x82\x82aH\x99V[\x91PP`\xE0\x85\x01Q\x82\x82\x03a\x01`\x84\x01RaY\xF0\x82\x82aH\xCFV[\x9A\x99PPPPPPPPPPV[`\0\x82`\x1F\x83\x01\x12aZ\x0FW`\0\x80\xFD[\x81Q` aZ\x1FaD<\x83aC\xC0V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aZ>W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aF\xE7WaZS\x81aV\xA4V[\x83R\x91\x83\x01\x91\x83\x01aZBV[`\0\x80`@\x83\x85\x03\x12\x15aZsW`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aZ\x8AW`\0\x80\xFD[\x90\x84\x01\x90`@\x82\x87\x03\x12\x15aZ\x9EW`\0\x80\xFD[aZ\xA6aCEV[\x82Q\x82\x81\x11\x15aZ\xB5W`\0\x80\xFD[aZ\xC1\x88\x82\x86\x01aY\xFEV[\x82RP` \x83\x01Q\x82\x81\x11\x15aZ\xD6W`\0\x80\xFD[aZ\xE2\x88\x82\x86\x01aY\xFEV[` \x83\x01RP\x80\x94PPPP` \x83\x01Q\x90P\x92P\x92\x90PV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a[\x13W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a[-W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15aG\xCFW`\0\x80\xFD[` \x81R`\0\x825a[S\x81aB_V[c\xFF\xFF\xFF\xFF\x81\x16` \x84\x01RPa[l` \x84\x01aA\xCEV[`\x01`\x01`@\x1B\x03\x81\x16`@\x84\x01RPa[\x88`@\x84\x01aBqV[c\xFF\xFF\xFF\xFF\x81\x16``\x84\x01RPa[\xA1``\x84\x01aBqV[c\xFF\xFF\xFF\xFF\x81\x16`\x80\x84\x01RPa[\xBA`\x80\x84\x01aBqV[c\xFF\xFF\xFF\xFF\x81\x16`\xA0\x84\x01RPa[\xD3`\xA0\x84\x01aBqV[c\xFF\xFF\xFF\xFF\x81\x16`\xC0\x84\x01RPa[\xED`\xC0\x84\x01\x84aQ\xADV[a\x01\0\x80`\xE0\x86\x01Ra\\\x05a\x01 \x86\x01\x83\x85aQ\xF2V[\x92PaR\xFA`\xE0\x87\x01aBqV[\x805a\\\x1E\x81aB_V[c\xFF\xFF\xFF\xFF\x80\x82\x16\x84R` \x83\x015` \x85\x01R`\x01`\x01`@\x1B\x03a\\F`@\x85\x01aA\xCEV[\x16`@\x85\x01R``\x83\x015\x91Pa\\\\\x82aB_V[\x16``\x83\x01R`\x80\x81\x81\x015\x90\x83\x01R`\xA0\x80\x82\x015\x90\x83\x01R`\xC0\x80\x82\x015\x90\x83\x01R`\xE0\x81\x015a\\\x8E\x81aB\x05V[`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x92\x90\x92\x01\x91\x90\x91RPV[a\x01\0\x81\x01a@\xB6\x82\x84a\\\x13V[`\0a\x01 a\\\xC2\x83\x86a\\\x13V[\x80a\x01\0\x84\x01RaI\x99\x81\x84\x01\x85aSQV[\x82\x81R``\x81\x01a\x16)` \x83\x01\x84\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[`\0c\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a]\x10Wa]\x10aU\x8EV[\x03\x93\x92PPPV[` \x81Rc\xFF\xFF\xFF\xFF\x82Q\x16` \x82\x01R`\0` \x83\x01Qa]B`@\x84\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P`@\x83\x01Qc\xFF\xFF\xFF\xFF\x81\x16``\x84\x01RP``\x83\x01Qc\xFF\xFF\xFF\xFF\x81\x16`\x80\x84\x01RP`\x80\x83\x01Qa\x01\0\x80`\xA0\x85\x01Ra]\x83a\x01 \x85\x01\x83aB\xCFV[\x91P`\xA0\x85\x01Qa]\x9C`\xC0\x86\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P`\xC0\x85\x01Q\x84\x83\x03`\x1F\x19\x01`\xE0\x86\x01Ra]\xB8\x83\x82aB\xCFV[\x92PP`\xE0\x85\x01Qa?\x97\x82\x86\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15a]\xF7Wa]\xF7aU\x8EV[\x02\x94\x93PPPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a^\x1AWa^\x1AaU\x8EV[P\x02\x90V[`\0\x82\x82\x10\x15a^1Wa^1aU\x8EV[P\x03\x90V[`\0a\xFF\xFF\x80\x83\x16\x81\x81\x03a^MWa^MaU\x8EV[`\x01\x01\x93\x92PPPV\xFE\xA2dipfsX\"\x12 n\x87\xBF\xE1\xAC\x91%+\xBDTjZ\xBB\xA7s\xFF\xF7\x88\x94\xF2\xC9\xADy\x93\x84\x99YV}\xDE\x8E\x1CdsolcC\0\x08\r\x003";
    /// The deployed bytecode of the contract.
    pub static FINALIZERTASKMANAGER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct FinalizerTaskManager<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for FinalizerTaskManager<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for FinalizerTaskManager<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for FinalizerTaskManager<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for FinalizerTaskManager<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(FinalizerTaskManager))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> FinalizerTaskManager<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                FINALIZERTASKMANAGER_ABI.clone(),
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
                FINALIZERTASKMANAGER_ABI.clone(),
                FINALIZERTASKMANAGER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `THRESHOLD_DENOMINATOR` (0xef024458) function
        pub fn threshold_denominator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([239, 2, 68, 88], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_validateTaskResponse` (0xb1edc8b4) function
        pub fn validate_task_response(
            &self,
            task_hash: [u8; 32],
            task_type: u8,
            reference_task_index: u32,
            task_created_block: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [177, 237, 200, 180],
                    (
                        task_hash,
                        task_type,
                        reference_task_index,
                        task_created_block,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `aggregator` (0x245a7bfc) function
        pub fn aggregator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([36, 90, 123, 252], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allTaskHashes` (0x13f815ed) function
        pub fn all_task_hashes(
            &self,
            p0: u8,
            p1: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([19, 248, 21, 237], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allTaskResponses` (0x1ac27297) function
        pub fn all_task_responses(
            &self,
            p0: u8,
            p1: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([26, 194, 114, 151], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allowNonRootInit` (0x0ee0fdbd) function
        pub fn allow_non_root_init(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([14, 224, 253, 189], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `blsApkRegistry` (0x5df45946) function
        pub fn bls_apk_registry(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([93, 244, 89, 70], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `blsSignatureChecker` (0x1c178e9c) function
        pub fn bls_signature_checker(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([28, 23, 142, 156], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `cancelPendingTasks` (0x21e78062) function
        pub fn cancel_pending_tasks(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([33, 231, 128, 98], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `chainRdBatchNonce` (0x0ebf2a81) function
        pub fn chain_rd_batch_nonce(
            &self,
            p0: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([14, 191, 42, 129], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkSignatures` (0x6efb4636) function
        pub fn check_signatures(
            &self,
            msg_hash: [u8; 32],
            quorum_numbers: ::ethers::core::types::Bytes,
            reference_block_number: u32,
            non_signer_stakes_and_signature: NonSignerStakesAndSignature,
        ) -> ::ethers::contract::builders::ContractCall<M, (QuorumStakeTotals, [u8; 32])> {
            self.0
                .method_hash(
                    [110, 251, 70, 54],
                    (
                        msg_hash,
                        quorum_numbers,
                        reference_block_number,
                        non_signer_stakes_and_signature,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createNewOpTask` (0x6e125ff4) function
        pub fn create_new_op_task(
            &self,
            quorum_threshold_percentage: u32,
            quorum_numbers: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [110, 18, 95, 244],
                    (quorum_threshold_percentage, quorum_numbers),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createNewRdTask` (0x2612fcef) function
        pub fn create_new_rd_task(
            &self,
            chain_id: u64,
            batch_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([38, 18, 252, 239], (chain_id, batch_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `delegation` (0xdf5cf723) function
        pub fn delegation(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([223, 92, 247, 35], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dummyForOperatorStateInfoType` (0x54d127de) function
        pub fn dummy_for_operator_state_info_type(
            &self,
            operator_state_info: OperatorStateInfo,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([84, 209, 39, 222], (operator_state_info,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dummyForQuorumStakeTotalsType` (0x45265b7a) function
        pub fn dummy_for_quorum_stake_totals_type(
            &self,
            quorum_stake_totals: QuorumStakeTotals,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([69, 38, 91, 122], (quorum_stake_totals,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `forceCancelPendingTasks` (0x60202fc0) function
        pub fn force_cancel_pending_tasks(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([96, 32, 47, 192], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `forceCreateNewOpTask` (0xf5640cf8) function
        pub fn force_create_new_op_task(
            &self,
            quorum_threshold_percentage: u32,
            quorum_numbers: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [245, 100, 12, 248],
                    (quorum_threshold_percentage, quorum_numbers),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `forceRespondToOpTask` (0x09ec1906) function
        pub fn force_respond_to_op_task(
            &self,
            task: OpTask,
            task_response: OpTaskResponse,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([9, 236, 25, 6], (task, task_response))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `generator` (0x7afa1eed) function
        pub fn generator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([122, 250, 30, 237], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBatchOperatorFromId` (0x4d2b57fe) function
        pub fn get_batch_operator_from_id(
            &self,
            registry_coordinator: ::ethers::core::types::Address,
            operator_ids: ::std::vec::Vec<[u8; 32]>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([77, 43, 87, 254], (registry_coordinator, operator_ids))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBatchOperatorId` (0x31b36bd9) function
        pub fn get_batch_operator_id(
            &self,
            registry_coordinator: ::ethers::core::types::Address,
            operators: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 32]>> {
            self.0
                .method_hash([49, 179, 107, 217], (registry_coordinator, operators))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCheckSignaturesIndices` (0x4f739f74) function
        pub fn get_check_signatures_indices(
            &self,
            registry_coordinator: ::ethers::core::types::Address,
            reference_block_number: u32,
            quorum_numbers: ::ethers::core::types::Bytes,
            non_signer_operator_ids: ::std::vec::Vec<[u8; 32]>,
        ) -> ::ethers::contract::builders::ContractCall<M, CheckSignaturesIndices> {
            self.0
                .method_hash(
                    [79, 115, 159, 116],
                    (
                        registry_coordinator,
                        reference_block_number,
                        quorum_numbers,
                        non_signer_operator_ids,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOperatorState` (0x3563b0d1) function
        pub fn get_operator_state(
            &self,
            registry_coordinator: ::ethers::core::types::Address,
            quorum_numbers: ::ethers::core::types::Bytes,
            block_number: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<::std::vec::Vec<Operator>>>
        {
            self.0
                .method_hash(
                    [53, 99, 176, 209],
                    (registry_coordinator, quorum_numbers, block_number),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOperatorState` (0xcefdc1d4) function
        pub fn get_operator_state_with_registry_coordinator_and_operator_id(
            &self,
            registry_coordinator: ::ethers::core::types::Address,
            operator_id: [u8; 32],
            block_number: u32,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::std::vec::Vec<::std::vec::Vec<Operator>>,
            ),
        > {
            self.0
                .method_hash(
                    [206, 253, 193, 212],
                    (registry_coordinator, operator_id, block_number),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getQuorumBitmapsAtBlockNumber` (0x5c155662) function
        pub fn get_quorum_bitmaps_at_block_number(
            &self,
            registry_coordinator: ::ethers::core::types::Address,
            operator_ids: ::std::vec::Vec<[u8; 32]>,
            block_number: u32,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash(
                    [92, 21, 86, 98],
                    (registry_coordinator, operator_ids, block_number),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `idToTaskStatus` (0xbf2315ed) function
        pub fn id_to_task_status(
            &self,
            p0: u8,
            p1: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([191, 35, 21, 237], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0xde434838) function
        pub fn initialize(
            &self,
            pauser_registry: ::ethers::core::types::Address,
            initial_owner: ::ethers::core::types::Address,
            aggregator: ::ethers::core::types::Address,
            generator: ::ethers::core::types::Address,
            allow_non_root_init: bool,
            bls_signature_checker_address: ::ethers::core::types::Address,
            task_response_window_block: u32,
            operator_state_retriever_extended: ::ethers::core::types::Address,
            rolldown: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [222, 67, 72, 56],
                    (
                        pauser_registry,
                        initial_owner,
                        aggregator,
                        generator,
                        allow_non_root_init,
                        bls_signature_checker_address,
                        task_response_window_block,
                        operator_state_retriever_extended,
                        rolldown,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isTaskPending` (0x36f78ed8) function
        pub fn is_task_pending(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([54, 247, 142, 216], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastCompletedOpTaskCompletedBlock` (0x2c1101da) function
        pub fn last_completed_op_task_completed_block(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([44, 17, 1, 218], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastCompletedOpTaskCreatedBlock` (0x537a2929) function
        pub fn last_completed_op_task_created_block(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([83, 122, 41, 41], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastCompletedOpTaskNum` (0x8fc8729a) function
        pub fn last_completed_op_task_num(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([143, 200, 114, 154], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastCompletedOpTaskQuorumNumbers` (0x2830e8f9) function
        pub fn last_completed_op_task_quorum_numbers(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([40, 48, 232, 249], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastCompletedOpTaskQuorumThresholdPercentage` (0xe70c2623) function
        pub fn last_completed_op_task_quorum_threshold_percentage(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([231, 12, 38, 35], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastCompletedRdTaskCompletedBlock` (0x3d7222eb) function
        pub fn last_completed_rd_task_completed_block(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([61, 114, 34, 235], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastCompletedRdTaskCreatedBlock` (0xca50e56a) function
        pub fn last_completed_rd_task_created_block(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([202, 80, 229, 106], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastCompletedRdTaskNum` (0x0980f1ec) function
        pub fn last_completed_rd_task_num(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([9, 128, 241, 236], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastOpTaskCreatedBlock` (0x4d7a7116) function
        pub fn last_op_task_created_block(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([77, 122, 113, 22], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastRdTaskCreatedBlock` (0x8c82af5e) function
        pub fn last_rd_task_created_block(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([140, 130, 175, 94], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `latestOpTaskNum` (0x41789d57) function
        pub fn latest_op_task_num(&self) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([65, 120, 157, 87], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `latestRdTaskNum` (0x7afdd54b) function
        pub fn latest_rd_task_num(&self) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([122, 253, 213, 75], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operatorStateRetrieverExtended` (0x8380acbd) function
        pub fn operator_state_retriever_extended(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([131, 128, 172, 189], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operatorsStateInfoHash` (0xadfcb048) function
        pub fn operators_state_info_hash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([173, 252, 176, 72], ())
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
        ///Calls the contract's `registryCoordinator` (0x6d14a987) function
        pub fn registry_coordinator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([109, 20, 169, 135], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `respondToOpTask` (0xd737d9f2) function
        pub fn respond_to_op_task(
            &self,
            task: OpTask,
            task_response: OpTaskResponse,
            non_signer_stakes_and_signature: NonSignerStakesAndSignature,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [215, 55, 217, 242],
                    (task, task_response, non_signer_stakes_and_signature),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `respondToRdTask` (0xa9077bfb) function
        pub fn respond_to_rd_task(
            &self,
            task: RdTask,
            task_response: RdTaskResponse,
            non_signer_stakes_and_signature: NonSignerStakesAndSignature,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [169, 7, 123, 251],
                    (task, task_response, non_signer_stakes_and_signature),
                )
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
        ///Calls the contract's `setAggregator` (0xf9120af6) function
        pub fn set_aggregator(
            &self,
            aggregator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([249, 18, 10, 246], aggregator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setGenerator` (0x4a7c7e4b) function
        pub fn set_generator(
            &self,
            generator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([74, 124, 126, 75], generator)
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
        ///Calls the contract's `stakeRegistry` (0x68304835) function
        pub fn stake_registry(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([104, 48, 72, 53], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `taskResponseWindowBlock` (0xa69563a9) function
        pub fn task_response_window_block(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([166, 149, 99, 169], ())
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
        ///Calls the contract's `unpause` (0xfabc1cbc) function
        pub fn unpause(
            &self,
            new_paused_status: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 188, 28, 188], new_paused_status)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateBlsSignatureCheckerAddress` (0x723114ab) function
        pub fn update_bls_signature_checker_address(
            &self,
            bls_signature_checker_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([114, 49, 20, 171], bls_signature_checker_address)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AggregatorUpdated` event
        pub fn aggregator_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AggregatorUpdatedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `BLSSignatureCheckerAddressUpdated` event
        pub fn bls_signature_checker_address_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BlssignatureCheckerAddressUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `GeneratorUpdated` event
        pub fn generator_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, GeneratorUpdatedFilter>
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
        ///Gets the contract's `NewOpTaskCreated` event
        pub fn new_op_task_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, NewOpTaskCreatedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `NewOpTaskForceCreated` event
        pub fn new_op_task_force_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, NewOpTaskForceCreatedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `NewRdTaskCreated` event
        pub fn new_rd_task_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, NewRdTaskCreatedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `OpTaskCancelled` event
        pub fn op_task_cancelled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OpTaskCancelledFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `OpTaskCompleted` event
        pub fn op_task_completed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OpTaskCompletedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `OpTaskForceCompleted` event
        pub fn op_task_force_completed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OpTaskForceCompletedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `OpTaskResponded` event
        pub fn op_task_responded_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OpTaskRespondedFilter>
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
        ///Gets the contract's `RdTaskCancelled` event
        pub fn rd_task_cancelled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RdTaskCancelledFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `RdTaskCompleted` event
        pub fn rd_task_completed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RdTaskCompletedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `RdTaskResponded` event
        pub fn rd_task_responded_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RdTaskRespondedFilter>
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
        ///Gets the contract's `StaleStakesForbiddenUpdate` event
        pub fn stale_stakes_forbidden_update_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StaleStakesForbiddenUpdateFilter,
        > {
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
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, FinalizerTaskManagerEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for FinalizerTaskManager<M>
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
    #[ethevent(name = "AggregatorUpdated", abi = "AggregatorUpdated(address)")]
    pub struct AggregatorUpdatedFilter {
        pub aggregator_address: ::ethers::core::types::Address,
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
        name = "BLSSignatureCheckerAddressUpdated",
        abi = "BLSSignatureCheckerAddressUpdated(address)"
    )]
    pub struct BlssignatureCheckerAddressUpdatedFilter {
        pub bls_signature_checker_address: ::ethers::core::types::Address,
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
    #[ethevent(name = "GeneratorUpdated", abi = "GeneratorUpdated(address)")]
    pub struct GeneratorUpdatedFilter {
        pub generator_address: ::ethers::core::types::Address,
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
        name = "NewOpTaskCreated",
        abi = "NewOpTaskCreated(uint32,(uint32,uint32,uint32,uint32,bytes,uint32,bytes,uint32))"
    )]
    pub struct NewOpTaskCreatedFilter {
        #[ethevent(indexed)]
        pub task_index: u32,
        pub task: OpTask,
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
    #[ethevent(name = "NewOpTaskForceCreated", abi = "NewOpTaskForceCreated()")]
    pub struct NewOpTaskForceCreatedFilter;
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
        name = "NewRdTaskCreated",
        abi = "NewRdTaskCreated(uint32,(uint32,uint64,uint32,uint32,uint32,uint32,bytes,uint32))"
    )]
    pub struct NewRdTaskCreatedFilter {
        #[ethevent(indexed)]
        pub task_index: u32,
        pub task: RdTask,
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
    #[ethevent(name = "OpTaskCancelled", abi = "OpTaskCancelled(uint32)")]
    pub struct OpTaskCancelledFilter {
        #[ethevent(indexed)]
        pub task_index: u32,
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
        name = "OpTaskCompleted",
        abi = "OpTaskCompleted(uint32,(uint32,bytes32,bytes32))"
    )]
    pub struct OpTaskCompletedFilter {
        #[ethevent(indexed)]
        pub task_index: u32,
        pub task_response: OpTaskResponse,
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
        name = "OpTaskForceCompleted",
        abi = "OpTaskForceCompleted(uint32,(uint32,bytes32,bytes32))"
    )]
    pub struct OpTaskForceCompletedFilter {
        #[ethevent(indexed)]
        pub task_index: u32,
        pub task_response: OpTaskResponse,
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
        name = "OpTaskResponded",
        abi = "OpTaskResponded(uint32,(uint32,bytes32,bytes32),(uint32,bytes32,uint96[],uint96[]))"
    )]
    pub struct OpTaskRespondedFilter {
        #[ethevent(indexed)]
        pub task_index: u32,
        pub task_response: OpTaskResponse,
        pub task_response_metadata: TaskResponseMetadata,
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
    #[ethevent(name = "RdTaskCancelled", abi = "RdTaskCancelled(uint32)")]
    pub struct RdTaskCancelledFilter {
        #[ethevent(indexed)]
        pub task_index: u32,
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
        name = "RdTaskCompleted",
        abi = "RdTaskCompleted(uint32,(uint32,bytes32,uint64,uint32,bytes32,uint256,uint256,address))"
    )]
    pub struct RdTaskCompletedFilter {
        #[ethevent(indexed)]
        pub task_index: u32,
        pub task_response: RdTaskResponse,
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
        name = "RdTaskResponded",
        abi = "RdTaskResponded(uint32,(uint32,bytes32,uint64,uint32,bytes32,uint256,uint256,address),(uint32,bytes32,uint96[],uint96[]))"
    )]
    pub struct RdTaskRespondedFilter {
        #[ethevent(indexed)]
        pub task_index: u32,
        pub task_response: RdTaskResponse,
        pub task_response_metadata: TaskResponseMetadata,
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
    #[ethevent(
        name = "StaleStakesForbiddenUpdate",
        abi = "StaleStakesForbiddenUpdate(bool)"
    )]
    pub struct StaleStakesForbiddenUpdateFilter {
        pub value: bool,
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
    pub enum FinalizerTaskManagerEvents {
        AggregatorUpdatedFilter(AggregatorUpdatedFilter),
        BlssignatureCheckerAddressUpdatedFilter(BlssignatureCheckerAddressUpdatedFilter),
        GeneratorUpdatedFilter(GeneratorUpdatedFilter),
        InitializedFilter(InitializedFilter),
        NewOpTaskCreatedFilter(NewOpTaskCreatedFilter),
        NewOpTaskForceCreatedFilter(NewOpTaskForceCreatedFilter),
        NewRdTaskCreatedFilter(NewRdTaskCreatedFilter),
        OpTaskCancelledFilter(OpTaskCancelledFilter),
        OpTaskCompletedFilter(OpTaskCompletedFilter),
        OpTaskForceCompletedFilter(OpTaskForceCompletedFilter),
        OpTaskRespondedFilter(OpTaskRespondedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PausedFilter(PausedFilter),
        PauserRegistrySetFilter(PauserRegistrySetFilter),
        RdTaskCancelledFilter(RdTaskCancelledFilter),
        RdTaskCompletedFilter(RdTaskCompletedFilter),
        RdTaskRespondedFilter(RdTaskRespondedFilter),
        RolldownTargetUpdatedFilter(RolldownTargetUpdatedFilter),
        StaleStakesForbiddenUpdateFilter(StaleStakesForbiddenUpdateFilter),
        UnpausedFilter(UnpausedFilter),
    }
    impl ::ethers::contract::EthLogDecode for FinalizerTaskManagerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AggregatorUpdatedFilter::decode_log(log) {
                return Ok(FinalizerTaskManagerEvents::AggregatorUpdatedFilter(decoded));
            }
            if let Ok(decoded) = BlssignatureCheckerAddressUpdatedFilter::decode_log(log) {
                return Ok(
                    FinalizerTaskManagerEvents::BlssignatureCheckerAddressUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = GeneratorUpdatedFilter::decode_log(log) {
                return Ok(FinalizerTaskManagerEvents::GeneratorUpdatedFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(FinalizerTaskManagerEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = NewOpTaskCreatedFilter::decode_log(log) {
                return Ok(FinalizerTaskManagerEvents::NewOpTaskCreatedFilter(decoded));
            }
            if let Ok(decoded) = NewOpTaskForceCreatedFilter::decode_log(log) {
                return Ok(FinalizerTaskManagerEvents::NewOpTaskForceCreatedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = NewRdTaskCreatedFilter::decode_log(log) {
                return Ok(FinalizerTaskManagerEvents::NewRdTaskCreatedFilter(decoded));
            }
            if let Ok(decoded) = OpTaskCancelledFilter::decode_log(log) {
                return Ok(FinalizerTaskManagerEvents::OpTaskCancelledFilter(decoded));
            }
            if let Ok(decoded) = OpTaskCompletedFilter::decode_log(log) {
                return Ok(FinalizerTaskManagerEvents::OpTaskCompletedFilter(decoded));
            }
            if let Ok(decoded) = OpTaskForceCompletedFilter::decode_log(log) {
                return Ok(FinalizerTaskManagerEvents::OpTaskForceCompletedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = OpTaskRespondedFilter::decode_log(log) {
                return Ok(FinalizerTaskManagerEvents::OpTaskRespondedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(FinalizerTaskManagerEvents::OwnershipTransferredFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(FinalizerTaskManagerEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = PauserRegistrySetFilter::decode_log(log) {
                return Ok(FinalizerTaskManagerEvents::PauserRegistrySetFilter(decoded));
            }
            if let Ok(decoded) = RdTaskCancelledFilter::decode_log(log) {
                return Ok(FinalizerTaskManagerEvents::RdTaskCancelledFilter(decoded));
            }
            if let Ok(decoded) = RdTaskCompletedFilter::decode_log(log) {
                return Ok(FinalizerTaskManagerEvents::RdTaskCompletedFilter(decoded));
            }
            if let Ok(decoded) = RdTaskRespondedFilter::decode_log(log) {
                return Ok(FinalizerTaskManagerEvents::RdTaskRespondedFilter(decoded));
            }
            if let Ok(decoded) = RolldownTargetUpdatedFilter::decode_log(log) {
                return Ok(FinalizerTaskManagerEvents::RolldownTargetUpdatedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = StaleStakesForbiddenUpdateFilter::decode_log(log) {
                return Ok(FinalizerTaskManagerEvents::StaleStakesForbiddenUpdateFilter(decoded));
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(FinalizerTaskManagerEvents::UnpausedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for FinalizerTaskManagerEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AggregatorUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::BlssignatureCheckerAddressUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GeneratorUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::NewOpTaskCreatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::NewOpTaskForceCreatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::NewRdTaskCreatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OpTaskCancelledFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OpTaskCompletedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OpTaskForceCompletedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OpTaskRespondedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PausedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauserRegistrySetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RdTaskCancelledFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RdTaskCompletedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RdTaskRespondedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RolldownTargetUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::StaleStakesForbiddenUpdateFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnpausedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AggregatorUpdatedFilter> for FinalizerTaskManagerEvents {
        fn from(value: AggregatorUpdatedFilter) -> Self {
            Self::AggregatorUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<BlssignatureCheckerAddressUpdatedFilter> for FinalizerTaskManagerEvents {
        fn from(value: BlssignatureCheckerAddressUpdatedFilter) -> Self {
            Self::BlssignatureCheckerAddressUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<GeneratorUpdatedFilter> for FinalizerTaskManagerEvents {
        fn from(value: GeneratorUpdatedFilter) -> Self {
            Self::GeneratorUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for FinalizerTaskManagerEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<NewOpTaskCreatedFilter> for FinalizerTaskManagerEvents {
        fn from(value: NewOpTaskCreatedFilter) -> Self {
            Self::NewOpTaskCreatedFilter(value)
        }
    }
    impl ::core::convert::From<NewOpTaskForceCreatedFilter> for FinalizerTaskManagerEvents {
        fn from(value: NewOpTaskForceCreatedFilter) -> Self {
            Self::NewOpTaskForceCreatedFilter(value)
        }
    }
    impl ::core::convert::From<NewRdTaskCreatedFilter> for FinalizerTaskManagerEvents {
        fn from(value: NewRdTaskCreatedFilter) -> Self {
            Self::NewRdTaskCreatedFilter(value)
        }
    }
    impl ::core::convert::From<OpTaskCancelledFilter> for FinalizerTaskManagerEvents {
        fn from(value: OpTaskCancelledFilter) -> Self {
            Self::OpTaskCancelledFilter(value)
        }
    }
    impl ::core::convert::From<OpTaskCompletedFilter> for FinalizerTaskManagerEvents {
        fn from(value: OpTaskCompletedFilter) -> Self {
            Self::OpTaskCompletedFilter(value)
        }
    }
    impl ::core::convert::From<OpTaskForceCompletedFilter> for FinalizerTaskManagerEvents {
        fn from(value: OpTaskForceCompletedFilter) -> Self {
            Self::OpTaskForceCompletedFilter(value)
        }
    }
    impl ::core::convert::From<OpTaskRespondedFilter> for FinalizerTaskManagerEvents {
        fn from(value: OpTaskRespondedFilter) -> Self {
            Self::OpTaskRespondedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for FinalizerTaskManagerEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<PausedFilter> for FinalizerTaskManagerEvents {
        fn from(value: PausedFilter) -> Self {
            Self::PausedFilter(value)
        }
    }
    impl ::core::convert::From<PauserRegistrySetFilter> for FinalizerTaskManagerEvents {
        fn from(value: PauserRegistrySetFilter) -> Self {
            Self::PauserRegistrySetFilter(value)
        }
    }
    impl ::core::convert::From<RdTaskCancelledFilter> for FinalizerTaskManagerEvents {
        fn from(value: RdTaskCancelledFilter) -> Self {
            Self::RdTaskCancelledFilter(value)
        }
    }
    impl ::core::convert::From<RdTaskCompletedFilter> for FinalizerTaskManagerEvents {
        fn from(value: RdTaskCompletedFilter) -> Self {
            Self::RdTaskCompletedFilter(value)
        }
    }
    impl ::core::convert::From<RdTaskRespondedFilter> for FinalizerTaskManagerEvents {
        fn from(value: RdTaskRespondedFilter) -> Self {
            Self::RdTaskRespondedFilter(value)
        }
    }
    impl ::core::convert::From<RolldownTargetUpdatedFilter> for FinalizerTaskManagerEvents {
        fn from(value: RolldownTargetUpdatedFilter) -> Self {
            Self::RolldownTargetUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<StaleStakesForbiddenUpdateFilter> for FinalizerTaskManagerEvents {
        fn from(value: StaleStakesForbiddenUpdateFilter) -> Self {
            Self::StaleStakesForbiddenUpdateFilter(value)
        }
    }
    impl ::core::convert::From<UnpausedFilter> for FinalizerTaskManagerEvents {
        fn from(value: UnpausedFilter) -> Self {
            Self::UnpausedFilter(value)
        }
    }
    ///Container type for all input parameters for the `THRESHOLD_DENOMINATOR` function with signature `THRESHOLD_DENOMINATOR()` and selector `0xef024458`
    #[derive(
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
    #[ethcall(name = "THRESHOLD_DENOMINATOR", abi = "THRESHOLD_DENOMINATOR()")]
    pub struct ThresholdDenominatorCall;
    ///Container type for all input parameters for the `_validateTaskResponse` function with signature `_validateTaskResponse(bytes32,uint8,uint32,uint32)` and selector `0xb1edc8b4`
    #[derive(
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
        name = "_validateTaskResponse",
        abi = "_validateTaskResponse(bytes32,uint8,uint32,uint32)"
    )]
    pub struct ValidateTaskResponseCall {
        pub task_hash: [u8; 32],
        pub task_type: u8,
        pub reference_task_index: u32,
        pub task_created_block: u32,
    }
    ///Container type for all input parameters for the `aggregator` function with signature `aggregator()` and selector `0x245a7bfc`
    #[derive(
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
    #[ethcall(name = "aggregator", abi = "aggregator()")]
    pub struct AggregatorCall;
    ///Container type for all input parameters for the `allTaskHashes` function with signature `allTaskHashes(uint8,uint32)` and selector `0x13f815ed`
    #[derive(
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
    #[ethcall(name = "allTaskHashes", abi = "allTaskHashes(uint8,uint32)")]
    pub struct AllTaskHashesCall(pub u8, pub u32);
    ///Container type for all input parameters for the `allTaskResponses` function with signature `allTaskResponses(uint8,uint32)` and selector `0x1ac27297`
    #[derive(
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
    #[ethcall(name = "allTaskResponses", abi = "allTaskResponses(uint8,uint32)")]
    pub struct AllTaskResponsesCall(pub u8, pub u32);
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
    ///Container type for all input parameters for the `blsApkRegistry` function with signature `blsApkRegistry()` and selector `0x5df45946`
    #[derive(
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
    #[ethcall(name = "blsApkRegistry", abi = "blsApkRegistry()")]
    pub struct BlsApkRegistryCall;
    ///Container type for all input parameters for the `blsSignatureChecker` function with signature `blsSignatureChecker()` and selector `0x1c178e9c`
    #[derive(
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
    #[ethcall(name = "blsSignatureChecker", abi = "blsSignatureChecker()")]
    pub struct BlsSignatureCheckerCall;
    ///Container type for all input parameters for the `cancelPendingTasks` function with signature `cancelPendingTasks()` and selector `0x21e78062`
    #[derive(
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
    #[ethcall(name = "cancelPendingTasks", abi = "cancelPendingTasks()")]
    pub struct CancelPendingTasksCall;
    ///Container type for all input parameters for the `chainRdBatchNonce` function with signature `chainRdBatchNonce(uint64)` and selector `0x0ebf2a81`
    #[derive(
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
    #[ethcall(name = "chainRdBatchNonce", abi = "chainRdBatchNonce(uint64)")]
    pub struct ChainRdBatchNonceCall(pub u64);
    ///Container type for all input parameters for the `checkSignatures` function with signature `checkSignatures(bytes32,bytes,uint32,(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))` and selector `0x6efb4636`
    #[derive(
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
        abi = "checkSignatures(bytes32,bytes,uint32,(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))"
    )]
    pub struct CheckSignaturesCall {
        pub msg_hash: [u8; 32],
        pub quorum_numbers: ::ethers::core::types::Bytes,
        pub reference_block_number: u32,
        pub non_signer_stakes_and_signature: NonSignerStakesAndSignature,
    }
    ///Container type for all input parameters for the `createNewOpTask` function with signature `createNewOpTask(uint32,bytes)` and selector `0x6e125ff4`
    #[derive(
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
    #[ethcall(name = "createNewOpTask", abi = "createNewOpTask(uint32,bytes)")]
    pub struct CreateNewOpTaskCall {
        pub quorum_threshold_percentage: u32,
        pub quorum_numbers: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `createNewRdTask` function with signature `createNewRdTask(uint64,uint32)` and selector `0x2612fcef`
    #[derive(
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
    #[ethcall(name = "createNewRdTask", abi = "createNewRdTask(uint64,uint32)")]
    pub struct CreateNewRdTaskCall {
        pub chain_id: u64,
        pub batch_id: u32,
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
        Hash,
    )]
    #[ethcall(name = "delegation", abi = "delegation()")]
    pub struct DelegationCall;
    ///Container type for all input parameters for the `dummyForOperatorStateInfoType` function with signature `dummyForOperatorStateInfoType((bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]))` and selector `0x54d127de`
    #[derive(
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
        name = "dummyForOperatorStateInfoType",
        abi = "dummyForOperatorStateInfoType((bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]))"
    )]
    pub struct DummyForOperatorStateInfoTypeCall {
        pub operator_state_info: OperatorStateInfo,
    }
    ///Container type for all input parameters for the `dummyForQuorumStakeTotalsType` function with signature `dummyForQuorumStakeTotalsType((uint96[],uint96[]))` and selector `0x45265b7a`
    #[derive(
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
        name = "dummyForQuorumStakeTotalsType",
        abi = "dummyForQuorumStakeTotalsType((uint96[],uint96[]))"
    )]
    pub struct DummyForQuorumStakeTotalsTypeCall {
        pub quorum_stake_totals: QuorumStakeTotals,
    }
    ///Container type for all input parameters for the `forceCancelPendingTasks` function with signature `forceCancelPendingTasks()` and selector `0x60202fc0`
    #[derive(
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
    #[ethcall(name = "forceCancelPendingTasks", abi = "forceCancelPendingTasks()")]
    pub struct ForceCancelPendingTasksCall;
    ///Container type for all input parameters for the `forceCreateNewOpTask` function with signature `forceCreateNewOpTask(uint32,bytes)` and selector `0xf5640cf8`
    #[derive(
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
        name = "forceCreateNewOpTask",
        abi = "forceCreateNewOpTask(uint32,bytes)"
    )]
    pub struct ForceCreateNewOpTaskCall {
        pub quorum_threshold_percentage: u32,
        pub quorum_numbers: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `forceRespondToOpTask` function with signature `forceRespondToOpTask((uint32,uint32,uint32,uint32,bytes,uint32,bytes,uint32),(uint32,bytes32,bytes32))` and selector `0x09ec1906`
    #[derive(
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
        name = "forceRespondToOpTask",
        abi = "forceRespondToOpTask((uint32,uint32,uint32,uint32,bytes,uint32,bytes,uint32),(uint32,bytes32,bytes32))"
    )]
    pub struct ForceRespondToOpTaskCall {
        pub task: OpTask,
        pub task_response: OpTaskResponse,
    }
    ///Container type for all input parameters for the `generator` function with signature `generator()` and selector `0x7afa1eed`
    #[derive(
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
    #[ethcall(name = "generator", abi = "generator()")]
    pub struct GeneratorCall;
    ///Container type for all input parameters for the `getBatchOperatorFromId` function with signature `getBatchOperatorFromId(address,bytes32[])` and selector `0x4d2b57fe`
    #[derive(
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
        name = "getBatchOperatorFromId",
        abi = "getBatchOperatorFromId(address,bytes32[])"
    )]
    pub struct GetBatchOperatorFromIdCall {
        pub registry_coordinator: ::ethers::core::types::Address,
        pub operator_ids: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all input parameters for the `getBatchOperatorId` function with signature `getBatchOperatorId(address,address[])` and selector `0x31b36bd9`
    #[derive(
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
        name = "getBatchOperatorId",
        abi = "getBatchOperatorId(address,address[])"
    )]
    pub struct GetBatchOperatorIdCall {
        pub registry_coordinator: ::ethers::core::types::Address,
        pub operators: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `getCheckSignaturesIndices` function with signature `getCheckSignaturesIndices(address,uint32,bytes,bytes32[])` and selector `0x4f739f74`
    #[derive(
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
        name = "getCheckSignaturesIndices",
        abi = "getCheckSignaturesIndices(address,uint32,bytes,bytes32[])"
    )]
    pub struct GetCheckSignaturesIndicesCall {
        pub registry_coordinator: ::ethers::core::types::Address,
        pub reference_block_number: u32,
        pub quorum_numbers: ::ethers::core::types::Bytes,
        pub non_signer_operator_ids: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all input parameters for the `getOperatorState` function with signature `getOperatorState(address,bytes,uint32)` and selector `0x3563b0d1`
    #[derive(
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
        name = "getOperatorState",
        abi = "getOperatorState(address,bytes,uint32)"
    )]
    pub struct GetOperatorStateCall {
        pub registry_coordinator: ::ethers::core::types::Address,
        pub quorum_numbers: ::ethers::core::types::Bytes,
        pub block_number: u32,
    }
    ///Container type for all input parameters for the `getOperatorState` function with signature `getOperatorState(address,bytes32,uint32)` and selector `0xcefdc1d4`
    #[derive(
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
        name = "getOperatorState",
        abi = "getOperatorState(address,bytes32,uint32)"
    )]
    pub struct GetOperatorStateWithRegistryCoordinatorAndOperatorIdCall {
        pub registry_coordinator: ::ethers::core::types::Address,
        pub operator_id: [u8; 32],
        pub block_number: u32,
    }
    ///Container type for all input parameters for the `getQuorumBitmapsAtBlockNumber` function with signature `getQuorumBitmapsAtBlockNumber(address,bytes32[],uint32)` and selector `0x5c155662`
    #[derive(
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
        name = "getQuorumBitmapsAtBlockNumber",
        abi = "getQuorumBitmapsAtBlockNumber(address,bytes32[],uint32)"
    )]
    pub struct GetQuorumBitmapsAtBlockNumberCall {
        pub registry_coordinator: ::ethers::core::types::Address,
        pub operator_ids: ::std::vec::Vec<[u8; 32]>,
        pub block_number: u32,
    }
    ///Container type for all input parameters for the `idToTaskStatus` function with signature `idToTaskStatus(uint8,uint32)` and selector `0xbf2315ed`
    #[derive(
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
    #[ethcall(name = "idToTaskStatus", abi = "idToTaskStatus(uint8,uint32)")]
    pub struct IdToTaskStatusCall(pub u8, pub u32);
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address,address,address,bool,address,uint32,address,address)` and selector `0xde434838`
    #[derive(
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
        abi = "initialize(address,address,address,address,bool,address,uint32,address,address)"
    )]
    pub struct InitializeCall {
        pub pauser_registry: ::ethers::core::types::Address,
        pub initial_owner: ::ethers::core::types::Address,
        pub aggregator: ::ethers::core::types::Address,
        pub generator: ::ethers::core::types::Address,
        pub allow_non_root_init: bool,
        pub bls_signature_checker_address: ::ethers::core::types::Address,
        pub task_response_window_block: u32,
        pub operator_state_retriever_extended: ::ethers::core::types::Address,
        pub rolldown: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isTaskPending` function with signature `isTaskPending()` and selector `0x36f78ed8`
    #[derive(
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
    #[ethcall(name = "isTaskPending", abi = "isTaskPending()")]
    pub struct IsTaskPendingCall;
    ///Container type for all input parameters for the `lastCompletedOpTaskCompletedBlock` function with signature `lastCompletedOpTaskCompletedBlock()` and selector `0x2c1101da`
    #[derive(
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
        name = "lastCompletedOpTaskCompletedBlock",
        abi = "lastCompletedOpTaskCompletedBlock()"
    )]
    pub struct LastCompletedOpTaskCompletedBlockCall;
    ///Container type for all input parameters for the `lastCompletedOpTaskCreatedBlock` function with signature `lastCompletedOpTaskCreatedBlock()` and selector `0x537a2929`
    #[derive(
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
        name = "lastCompletedOpTaskCreatedBlock",
        abi = "lastCompletedOpTaskCreatedBlock()"
    )]
    pub struct LastCompletedOpTaskCreatedBlockCall;
    ///Container type for all input parameters for the `lastCompletedOpTaskNum` function with signature `lastCompletedOpTaskNum()` and selector `0x8fc8729a`
    #[derive(
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
    #[ethcall(name = "lastCompletedOpTaskNum", abi = "lastCompletedOpTaskNum()")]
    pub struct LastCompletedOpTaskNumCall;
    ///Container type for all input parameters for the `lastCompletedOpTaskQuorumNumbers` function with signature `lastCompletedOpTaskQuorumNumbers()` and selector `0x2830e8f9`
    #[derive(
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
        name = "lastCompletedOpTaskQuorumNumbers",
        abi = "lastCompletedOpTaskQuorumNumbers()"
    )]
    pub struct LastCompletedOpTaskQuorumNumbersCall;
    ///Container type for all input parameters for the `lastCompletedOpTaskQuorumThresholdPercentage` function with signature `lastCompletedOpTaskQuorumThresholdPercentage()` and selector `0xe70c2623`
    #[derive(
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
        name = "lastCompletedOpTaskQuorumThresholdPercentage",
        abi = "lastCompletedOpTaskQuorumThresholdPercentage()"
    )]
    pub struct LastCompletedOpTaskQuorumThresholdPercentageCall;
    ///Container type for all input parameters for the `lastCompletedRdTaskCompletedBlock` function with signature `lastCompletedRdTaskCompletedBlock()` and selector `0x3d7222eb`
    #[derive(
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
        name = "lastCompletedRdTaskCompletedBlock",
        abi = "lastCompletedRdTaskCompletedBlock()"
    )]
    pub struct LastCompletedRdTaskCompletedBlockCall;
    ///Container type for all input parameters for the `lastCompletedRdTaskCreatedBlock` function with signature `lastCompletedRdTaskCreatedBlock()` and selector `0xca50e56a`
    #[derive(
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
        name = "lastCompletedRdTaskCreatedBlock",
        abi = "lastCompletedRdTaskCreatedBlock()"
    )]
    pub struct LastCompletedRdTaskCreatedBlockCall;
    ///Container type for all input parameters for the `lastCompletedRdTaskNum` function with signature `lastCompletedRdTaskNum()` and selector `0x0980f1ec`
    #[derive(
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
    #[ethcall(name = "lastCompletedRdTaskNum", abi = "lastCompletedRdTaskNum()")]
    pub struct LastCompletedRdTaskNumCall;
    ///Container type for all input parameters for the `lastOpTaskCreatedBlock` function with signature `lastOpTaskCreatedBlock()` and selector `0x4d7a7116`
    #[derive(
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
    #[ethcall(name = "lastOpTaskCreatedBlock", abi = "lastOpTaskCreatedBlock()")]
    pub struct LastOpTaskCreatedBlockCall;
    ///Container type for all input parameters for the `lastRdTaskCreatedBlock` function with signature `lastRdTaskCreatedBlock()` and selector `0x8c82af5e`
    #[derive(
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
    #[ethcall(name = "lastRdTaskCreatedBlock", abi = "lastRdTaskCreatedBlock()")]
    pub struct LastRdTaskCreatedBlockCall;
    ///Container type for all input parameters for the `latestOpTaskNum` function with signature `latestOpTaskNum()` and selector `0x41789d57`
    #[derive(
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
    #[ethcall(name = "latestOpTaskNum", abi = "latestOpTaskNum()")]
    pub struct LatestOpTaskNumCall;
    ///Container type for all input parameters for the `latestRdTaskNum` function with signature `latestRdTaskNum()` and selector `0x7afdd54b`
    #[derive(
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
    #[ethcall(name = "latestRdTaskNum", abi = "latestRdTaskNum()")]
    pub struct LatestRdTaskNumCall;
    ///Container type for all input parameters for the `operatorStateRetrieverExtended` function with signature `operatorStateRetrieverExtended()` and selector `0x8380acbd`
    #[derive(
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
        name = "operatorStateRetrieverExtended",
        abi = "operatorStateRetrieverExtended()"
    )]
    pub struct OperatorStateRetrieverExtendedCall;
    ///Container type for all input parameters for the `operatorsStateInfoHash` function with signature `operatorsStateInfoHash()` and selector `0xadfcb048`
    #[derive(
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
    #[ethcall(name = "operatorsStateInfoHash", abi = "operatorsStateInfoHash()")]
    pub struct OperatorsStateInfoHashCall;
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
        Hash,
    )]
    #[ethcall(name = "registryCoordinator", abi = "registryCoordinator()")]
    pub struct RegistryCoordinatorCall;
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
    ///Container type for all input parameters for the `respondToOpTask` function with signature `respondToOpTask((uint32,uint32,uint32,uint32,bytes,uint32,bytes,uint32),(uint32,bytes32,bytes32),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))` and selector `0xd737d9f2`
    #[derive(
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
        name = "respondToOpTask",
        abi = "respondToOpTask((uint32,uint32,uint32,uint32,bytes,uint32,bytes,uint32),(uint32,bytes32,bytes32),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))"
    )]
    pub struct RespondToOpTaskCall {
        pub task: OpTask,
        pub task_response: OpTaskResponse,
        pub non_signer_stakes_and_signature: NonSignerStakesAndSignature,
    }
    ///Container type for all input parameters for the `respondToRdTask` function with signature `respondToRdTask((uint32,uint64,uint32,uint32,uint32,uint32,bytes,uint32),(uint32,bytes32,uint64,uint32,bytes32,uint256,uint256,address),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))` and selector `0xa9077bfb`
    #[derive(
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
        name = "respondToRdTask",
        abi = "respondToRdTask((uint32,uint64,uint32,uint32,uint32,uint32,bytes,uint32),(uint32,bytes32,uint64,uint32,bytes32,uint256,uint256,address),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))"
    )]
    pub struct RespondToRdTaskCall {
        pub task: RdTask,
        pub task_response: RdTaskResponse,
        pub non_signer_stakes_and_signature: NonSignerStakesAndSignature,
    }
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
    ///Container type for all input parameters for the `setAggregator` function with signature `setAggregator(address)` and selector `0xf9120af6`
    #[derive(
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
    #[ethcall(name = "setAggregator", abi = "setAggregator(address)")]
    pub struct SetAggregatorCall {
        pub aggregator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setGenerator` function with signature `setGenerator(address)` and selector `0x4a7c7e4b`
    #[derive(
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
    #[ethcall(name = "setGenerator", abi = "setGenerator(address)")]
    pub struct SetGeneratorCall {
        pub generator: ::ethers::core::types::Address,
    }
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
    ///Container type for all input parameters for the `stakeRegistry` function with signature `stakeRegistry()` and selector `0x68304835`
    #[derive(
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
    #[ethcall(name = "stakeRegistry", abi = "stakeRegistry()")]
    pub struct StakeRegistryCall;
    ///Container type for all input parameters for the `taskResponseWindowBlock` function with signature `taskResponseWindowBlock()` and selector `0xa69563a9`
    #[derive(
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
    #[ethcall(name = "taskResponseWindowBlock", abi = "taskResponseWindowBlock()")]
    pub struct TaskResponseWindowBlockCall;
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
    ///Container type for all input parameters for the `updateBlsSignatureCheckerAddress` function with signature `updateBlsSignatureCheckerAddress(address)` and selector `0x723114ab`
    #[derive(
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
        name = "updateBlsSignatureCheckerAddress",
        abi = "updateBlsSignatureCheckerAddress(address)"
    )]
    pub struct UpdateBlsSignatureCheckerAddressCall {
        pub bls_signature_checker_address: ::ethers::core::types::Address,
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
        Hash,
    )]
    pub enum FinalizerTaskManagerCalls {
        ThresholdDenominator(ThresholdDenominatorCall),
        ValidateTaskResponse(ValidateTaskResponseCall),
        Aggregator(AggregatorCall),
        AllTaskHashes(AllTaskHashesCall),
        AllTaskResponses(AllTaskResponsesCall),
        AllowNonRootInit(AllowNonRootInitCall),
        BlsApkRegistry(BlsApkRegistryCall),
        BlsSignatureChecker(BlsSignatureCheckerCall),
        CancelPendingTasks(CancelPendingTasksCall),
        ChainRdBatchNonce(ChainRdBatchNonceCall),
        CheckSignatures(CheckSignaturesCall),
        CreateNewOpTask(CreateNewOpTaskCall),
        CreateNewRdTask(CreateNewRdTaskCall),
        Delegation(DelegationCall),
        DummyForOperatorStateInfoType(DummyForOperatorStateInfoTypeCall),
        DummyForQuorumStakeTotalsType(DummyForQuorumStakeTotalsTypeCall),
        ForceCancelPendingTasks(ForceCancelPendingTasksCall),
        ForceCreateNewOpTask(ForceCreateNewOpTaskCall),
        ForceRespondToOpTask(ForceRespondToOpTaskCall),
        Generator(GeneratorCall),
        GetBatchOperatorFromId(GetBatchOperatorFromIdCall),
        GetBatchOperatorId(GetBatchOperatorIdCall),
        GetCheckSignaturesIndices(GetCheckSignaturesIndicesCall),
        GetOperatorState(GetOperatorStateCall),
        GetOperatorStateWithRegistryCoordinatorAndOperatorId(
            GetOperatorStateWithRegistryCoordinatorAndOperatorIdCall,
        ),
        GetQuorumBitmapsAtBlockNumber(GetQuorumBitmapsAtBlockNumberCall),
        IdToTaskStatus(IdToTaskStatusCall),
        Initialize(InitializeCall),
        IsTaskPending(IsTaskPendingCall),
        LastCompletedOpTaskCompletedBlock(LastCompletedOpTaskCompletedBlockCall),
        LastCompletedOpTaskCreatedBlock(LastCompletedOpTaskCreatedBlockCall),
        LastCompletedOpTaskNum(LastCompletedOpTaskNumCall),
        LastCompletedOpTaskQuorumNumbers(LastCompletedOpTaskQuorumNumbersCall),
        LastCompletedOpTaskQuorumThresholdPercentage(
            LastCompletedOpTaskQuorumThresholdPercentageCall,
        ),
        LastCompletedRdTaskCompletedBlock(LastCompletedRdTaskCompletedBlockCall),
        LastCompletedRdTaskCreatedBlock(LastCompletedRdTaskCreatedBlockCall),
        LastCompletedRdTaskNum(LastCompletedRdTaskNumCall),
        LastOpTaskCreatedBlock(LastOpTaskCreatedBlockCall),
        LastRdTaskCreatedBlock(LastRdTaskCreatedBlockCall),
        LatestOpTaskNum(LatestOpTaskNumCall),
        LatestRdTaskNum(LatestRdTaskNumCall),
        OperatorStateRetrieverExtended(OperatorStateRetrieverExtendedCall),
        OperatorsStateInfoHash(OperatorsStateInfoHashCall),
        Owner(OwnerCall),
        Pause(PauseCall),
        PauseAll(PauseAllCall),
        PausedWithIndex(PausedWithIndexCall),
        Paused(PausedCall),
        PauserRegistry(PauserRegistryCall),
        RegistryCoordinator(RegistryCoordinatorCall),
        RenounceOwnership(RenounceOwnershipCall),
        RespondToOpTask(RespondToOpTaskCall),
        RespondToRdTask(RespondToRdTaskCall),
        Rolldown(RolldownCall),
        SetAggregator(SetAggregatorCall),
        SetGenerator(SetGeneratorCall),
        SetPauserRegistry(SetPauserRegistryCall),
        SetRolldown(SetRolldownCall),
        StakeRegistry(StakeRegistryCall),
        TaskResponseWindowBlock(TaskResponseWindowBlockCall),
        TransferOwnership(TransferOwnershipCall),
        Unpause(UnpauseCall),
        UpdateBlsSignatureCheckerAddress(UpdateBlsSignatureCheckerAddressCall),
    }
    impl ::ethers::core::abi::AbiDecode for FinalizerTaskManagerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <ThresholdDenominatorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ThresholdDenominator(decoded));
            }
            if let Ok(decoded) =
                <ValidateTaskResponseCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ValidateTaskResponse(decoded));
            }
            if let Ok(decoded) = <AggregatorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Aggregator(decoded));
            }
            if let Ok(decoded) = <AllTaskHashesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AllTaskHashes(decoded));
            }
            if let Ok(decoded) =
                <AllTaskResponsesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AllTaskResponses(decoded));
            }
            if let Ok(decoded) =
                <AllowNonRootInitCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AllowNonRootInit(decoded));
            }
            if let Ok(decoded) =
                <BlsApkRegistryCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BlsApkRegistry(decoded));
            }
            if let Ok(decoded) =
                <BlsSignatureCheckerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BlsSignatureChecker(decoded));
            }
            if let Ok(decoded) =
                <CancelPendingTasksCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CancelPendingTasks(decoded));
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
            if let Ok(decoded) =
                <CreateNewOpTaskCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CreateNewOpTask(decoded));
            }
            if let Ok(decoded) =
                <CreateNewRdTaskCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CreateNewRdTask(decoded));
            }
            if let Ok(decoded) = <DelegationCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Delegation(decoded));
            }
            if let Ok(decoded) =
                <DummyForOperatorStateInfoTypeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DummyForOperatorStateInfoType(decoded));
            }
            if let Ok(decoded) =
                <DummyForQuorumStakeTotalsTypeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DummyForQuorumStakeTotalsType(decoded));
            }
            if let Ok(decoded) =
                <ForceCancelPendingTasksCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ForceCancelPendingTasks(decoded));
            }
            if let Ok(decoded) =
                <ForceCreateNewOpTaskCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ForceCreateNewOpTask(decoded));
            }
            if let Ok(decoded) =
                <ForceRespondToOpTaskCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ForceRespondToOpTask(decoded));
            }
            if let Ok(decoded) = <GeneratorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Generator(decoded));
            }
            if let Ok(decoded) =
                <GetBatchOperatorFromIdCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetBatchOperatorFromId(decoded));
            }
            if let Ok(decoded) =
                <GetBatchOperatorIdCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetBatchOperatorId(decoded));
            }
            if let Ok(decoded) =
                <GetCheckSignaturesIndicesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetCheckSignaturesIndices(decoded));
            }
            if let Ok(decoded) =
                <GetOperatorStateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetOperatorState(decoded));
            }
            if let Ok(decoded) = <GetOperatorStateWithRegistryCoordinatorAndOperatorIdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::GetOperatorStateWithRegistryCoordinatorAndOperatorId(decoded),
                );
            }
            if let Ok(decoded) =
                <GetQuorumBitmapsAtBlockNumberCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetQuorumBitmapsAtBlockNumber(decoded));
            }
            if let Ok(decoded) =
                <IdToTaskStatusCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IdToTaskStatus(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <IsTaskPendingCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IsTaskPending(decoded));
            }
            if let Ok(decoded) =
                <LastCompletedOpTaskCompletedBlockCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::LastCompletedOpTaskCompletedBlock(decoded));
            }
            if let Ok(decoded) =
                <LastCompletedOpTaskCreatedBlockCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::LastCompletedOpTaskCreatedBlock(decoded));
            }
            if let Ok(decoded) =
                <LastCompletedOpTaskNumCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LastCompletedOpTaskNum(decoded));
            }
            if let Ok(decoded) =
                <LastCompletedOpTaskQuorumNumbersCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::LastCompletedOpTaskQuorumNumbers(decoded));
            }
            if let Ok(decoded) = <LastCompletedOpTaskQuorumThresholdPercentageCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LastCompletedOpTaskQuorumThresholdPercentage(decoded));
            }
            if let Ok(decoded) =
                <LastCompletedRdTaskCompletedBlockCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::LastCompletedRdTaskCompletedBlock(decoded));
            }
            if let Ok(decoded) =
                <LastCompletedRdTaskCreatedBlockCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::LastCompletedRdTaskCreatedBlock(decoded));
            }
            if let Ok(decoded) =
                <LastCompletedRdTaskNumCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LastCompletedRdTaskNum(decoded));
            }
            if let Ok(decoded) =
                <LastOpTaskCreatedBlockCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LastOpTaskCreatedBlock(decoded));
            }
            if let Ok(decoded) =
                <LastRdTaskCreatedBlockCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LastRdTaskCreatedBlock(decoded));
            }
            if let Ok(decoded) =
                <LatestOpTaskNumCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LatestOpTaskNum(decoded));
            }
            if let Ok(decoded) =
                <LatestRdTaskNumCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LatestRdTaskNum(decoded));
            }
            if let Ok(decoded) =
                <OperatorStateRetrieverExtendedCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OperatorStateRetrieverExtended(decoded));
            }
            if let Ok(decoded) =
                <OperatorsStateInfoHashCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OperatorsStateInfoHash(decoded));
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
                <RegistryCoordinatorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RegistryCoordinator(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <RespondToOpTaskCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RespondToOpTask(decoded));
            }
            if let Ok(decoded) =
                <RespondToRdTaskCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RespondToRdTask(decoded));
            }
            if let Ok(decoded) = <RolldownCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Rolldown(decoded));
            }
            if let Ok(decoded) = <SetAggregatorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetAggregator(decoded));
            }
            if let Ok(decoded) = <SetGeneratorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetGenerator(decoded));
            }
            if let Ok(decoded) =
                <SetPauserRegistryCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetPauserRegistry(decoded));
            }
            if let Ok(decoded) = <SetRolldownCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetRolldown(decoded));
            }
            if let Ok(decoded) = <StakeRegistryCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::StakeRegistry(decoded));
            }
            if let Ok(decoded) =
                <TaskResponseWindowBlockCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TaskResponseWindowBlock(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <UnpauseCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Unpause(decoded));
            }
            if let Ok(decoded) =
                <UpdateBlsSignatureCheckerAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::UpdateBlsSignatureCheckerAddress(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for FinalizerTaskManagerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ThresholdDenominator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ValidateTaskResponse(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Aggregator(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AllTaskHashes(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AllTaskResponses(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AllowNonRootInit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BlsApkRegistry(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BlsSignatureChecker(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CancelPendingTasks(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChainRdBatchNonce(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CheckSignatures(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CreateNewOpTask(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CreateNewRdTask(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Delegation(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DummyForOperatorStateInfoType(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DummyForQuorumStakeTotalsType(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ForceCancelPendingTasks(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ForceCreateNewOpTask(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ForceRespondToOpTask(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Generator(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetBatchOperatorFromId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetBatchOperatorId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetCheckSignaturesIndices(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOperatorState(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetOperatorStateWithRegistryCoordinatorAndOperatorId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetQuorumBitmapsAtBlockNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IdToTaskStatus(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsTaskPending(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LastCompletedOpTaskCompletedBlock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LastCompletedOpTaskCreatedBlock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LastCompletedOpTaskNum(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LastCompletedOpTaskQuorumNumbers(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LastCompletedOpTaskQuorumThresholdPercentage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LastCompletedRdTaskCompletedBlock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LastCompletedRdTaskCreatedBlock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LastCompletedRdTaskNum(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LastOpTaskCreatedBlock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LastRdTaskCreatedBlock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LatestOpTaskNum(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LatestRdTaskNum(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OperatorStateRetrieverExtended(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OperatorsStateInfoHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PauseAll(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PausedWithIndex(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Paused(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PauserRegistry(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RegistryCoordinator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RespondToOpTask(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RespondToRdTask(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Rolldown(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetAggregator(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetGenerator(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetPauserRegistry(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetRolldown(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StakeRegistry(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TaskResponseWindowBlock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Unpause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateBlsSignatureCheckerAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for FinalizerTaskManagerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ThresholdDenominator(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidateTaskResponse(element) => ::core::fmt::Display::fmt(element, f),
                Self::Aggregator(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllTaskHashes(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllTaskResponses(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllowNonRootInit(element) => ::core::fmt::Display::fmt(element, f),
                Self::BlsApkRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::BlsSignatureChecker(element) => ::core::fmt::Display::fmt(element, f),
                Self::CancelPendingTasks(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChainRdBatchNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckSignatures(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateNewOpTask(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateNewRdTask(element) => ::core::fmt::Display::fmt(element, f),
                Self::Delegation(element) => ::core::fmt::Display::fmt(element, f),
                Self::DummyForOperatorStateInfoType(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DummyForQuorumStakeTotalsType(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ForceCancelPendingTasks(element) => ::core::fmt::Display::fmt(element, f),
                Self::ForceCreateNewOpTask(element) => ::core::fmt::Display::fmt(element, f),
                Self::ForceRespondToOpTask(element) => ::core::fmt::Display::fmt(element, f),
                Self::Generator(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBatchOperatorFromId(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBatchOperatorId(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCheckSignaturesIndices(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOperatorState(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOperatorStateWithRegistryCoordinatorAndOperatorId(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetQuorumBitmapsAtBlockNumber(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IdToTaskStatus(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsTaskPending(element) => ::core::fmt::Display::fmt(element, f),
                Self::LastCompletedOpTaskCompletedBlock(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LastCompletedOpTaskCreatedBlock(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LastCompletedOpTaskNum(element) => ::core::fmt::Display::fmt(element, f),
                Self::LastCompletedOpTaskQuorumNumbers(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LastCompletedOpTaskQuorumThresholdPercentage(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LastCompletedRdTaskCompletedBlock(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LastCompletedRdTaskCreatedBlock(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LastCompletedRdTaskNum(element) => ::core::fmt::Display::fmt(element, f),
                Self::LastOpTaskCreatedBlock(element) => ::core::fmt::Display::fmt(element, f),
                Self::LastRdTaskCreatedBlock(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestOpTaskNum(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestRdTaskNum(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorStateRetrieverExtended(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorsStateInfoHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pause(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauseAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::PausedWithIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::Paused(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauserRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegistryCoordinator(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::RespondToOpTask(element) => ::core::fmt::Display::fmt(element, f),
                Self::RespondToRdTask(element) => ::core::fmt::Display::fmt(element, f),
                Self::Rolldown(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAggregator(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetGenerator(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPauserRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetRolldown(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::TaskResponseWindowBlock(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unpause(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateBlsSignatureCheckerAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ThresholdDenominatorCall> for FinalizerTaskManagerCalls {
        fn from(value: ThresholdDenominatorCall) -> Self {
            Self::ThresholdDenominator(value)
        }
    }
    impl ::core::convert::From<ValidateTaskResponseCall> for FinalizerTaskManagerCalls {
        fn from(value: ValidateTaskResponseCall) -> Self {
            Self::ValidateTaskResponse(value)
        }
    }
    impl ::core::convert::From<AggregatorCall> for FinalizerTaskManagerCalls {
        fn from(value: AggregatorCall) -> Self {
            Self::Aggregator(value)
        }
    }
    impl ::core::convert::From<AllTaskHashesCall> for FinalizerTaskManagerCalls {
        fn from(value: AllTaskHashesCall) -> Self {
            Self::AllTaskHashes(value)
        }
    }
    impl ::core::convert::From<AllTaskResponsesCall> for FinalizerTaskManagerCalls {
        fn from(value: AllTaskResponsesCall) -> Self {
            Self::AllTaskResponses(value)
        }
    }
    impl ::core::convert::From<AllowNonRootInitCall> for FinalizerTaskManagerCalls {
        fn from(value: AllowNonRootInitCall) -> Self {
            Self::AllowNonRootInit(value)
        }
    }
    impl ::core::convert::From<BlsApkRegistryCall> for FinalizerTaskManagerCalls {
        fn from(value: BlsApkRegistryCall) -> Self {
            Self::BlsApkRegistry(value)
        }
    }
    impl ::core::convert::From<BlsSignatureCheckerCall> for FinalizerTaskManagerCalls {
        fn from(value: BlsSignatureCheckerCall) -> Self {
            Self::BlsSignatureChecker(value)
        }
    }
    impl ::core::convert::From<CancelPendingTasksCall> for FinalizerTaskManagerCalls {
        fn from(value: CancelPendingTasksCall) -> Self {
            Self::CancelPendingTasks(value)
        }
    }
    impl ::core::convert::From<ChainRdBatchNonceCall> for FinalizerTaskManagerCalls {
        fn from(value: ChainRdBatchNonceCall) -> Self {
            Self::ChainRdBatchNonce(value)
        }
    }
    impl ::core::convert::From<CheckSignaturesCall> for FinalizerTaskManagerCalls {
        fn from(value: CheckSignaturesCall) -> Self {
            Self::CheckSignatures(value)
        }
    }
    impl ::core::convert::From<CreateNewOpTaskCall> for FinalizerTaskManagerCalls {
        fn from(value: CreateNewOpTaskCall) -> Self {
            Self::CreateNewOpTask(value)
        }
    }
    impl ::core::convert::From<CreateNewRdTaskCall> for FinalizerTaskManagerCalls {
        fn from(value: CreateNewRdTaskCall) -> Self {
            Self::CreateNewRdTask(value)
        }
    }
    impl ::core::convert::From<DelegationCall> for FinalizerTaskManagerCalls {
        fn from(value: DelegationCall) -> Self {
            Self::Delegation(value)
        }
    }
    impl ::core::convert::From<DummyForOperatorStateInfoTypeCall> for FinalizerTaskManagerCalls {
        fn from(value: DummyForOperatorStateInfoTypeCall) -> Self {
            Self::DummyForOperatorStateInfoType(value)
        }
    }
    impl ::core::convert::From<DummyForQuorumStakeTotalsTypeCall> for FinalizerTaskManagerCalls {
        fn from(value: DummyForQuorumStakeTotalsTypeCall) -> Self {
            Self::DummyForQuorumStakeTotalsType(value)
        }
    }
    impl ::core::convert::From<ForceCancelPendingTasksCall> for FinalizerTaskManagerCalls {
        fn from(value: ForceCancelPendingTasksCall) -> Self {
            Self::ForceCancelPendingTasks(value)
        }
    }
    impl ::core::convert::From<ForceCreateNewOpTaskCall> for FinalizerTaskManagerCalls {
        fn from(value: ForceCreateNewOpTaskCall) -> Self {
            Self::ForceCreateNewOpTask(value)
        }
    }
    impl ::core::convert::From<ForceRespondToOpTaskCall> for FinalizerTaskManagerCalls {
        fn from(value: ForceRespondToOpTaskCall) -> Self {
            Self::ForceRespondToOpTask(value)
        }
    }
    impl ::core::convert::From<GeneratorCall> for FinalizerTaskManagerCalls {
        fn from(value: GeneratorCall) -> Self {
            Self::Generator(value)
        }
    }
    impl ::core::convert::From<GetBatchOperatorFromIdCall> for FinalizerTaskManagerCalls {
        fn from(value: GetBatchOperatorFromIdCall) -> Self {
            Self::GetBatchOperatorFromId(value)
        }
    }
    impl ::core::convert::From<GetBatchOperatorIdCall> for FinalizerTaskManagerCalls {
        fn from(value: GetBatchOperatorIdCall) -> Self {
            Self::GetBatchOperatorId(value)
        }
    }
    impl ::core::convert::From<GetCheckSignaturesIndicesCall> for FinalizerTaskManagerCalls {
        fn from(value: GetCheckSignaturesIndicesCall) -> Self {
            Self::GetCheckSignaturesIndices(value)
        }
    }
    impl ::core::convert::From<GetOperatorStateCall> for FinalizerTaskManagerCalls {
        fn from(value: GetOperatorStateCall) -> Self {
            Self::GetOperatorState(value)
        }
    }
    impl ::core::convert::From<GetOperatorStateWithRegistryCoordinatorAndOperatorIdCall>
        for FinalizerTaskManagerCalls
    {
        fn from(value: GetOperatorStateWithRegistryCoordinatorAndOperatorIdCall) -> Self {
            Self::GetOperatorStateWithRegistryCoordinatorAndOperatorId(value)
        }
    }
    impl ::core::convert::From<GetQuorumBitmapsAtBlockNumberCall> for FinalizerTaskManagerCalls {
        fn from(value: GetQuorumBitmapsAtBlockNumberCall) -> Self {
            Self::GetQuorumBitmapsAtBlockNumber(value)
        }
    }
    impl ::core::convert::From<IdToTaskStatusCall> for FinalizerTaskManagerCalls {
        fn from(value: IdToTaskStatusCall) -> Self {
            Self::IdToTaskStatus(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for FinalizerTaskManagerCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<IsTaskPendingCall> for FinalizerTaskManagerCalls {
        fn from(value: IsTaskPendingCall) -> Self {
            Self::IsTaskPending(value)
        }
    }
    impl ::core::convert::From<LastCompletedOpTaskCompletedBlockCall> for FinalizerTaskManagerCalls {
        fn from(value: LastCompletedOpTaskCompletedBlockCall) -> Self {
            Self::LastCompletedOpTaskCompletedBlock(value)
        }
    }
    impl ::core::convert::From<LastCompletedOpTaskCreatedBlockCall> for FinalizerTaskManagerCalls {
        fn from(value: LastCompletedOpTaskCreatedBlockCall) -> Self {
            Self::LastCompletedOpTaskCreatedBlock(value)
        }
    }
    impl ::core::convert::From<LastCompletedOpTaskNumCall> for FinalizerTaskManagerCalls {
        fn from(value: LastCompletedOpTaskNumCall) -> Self {
            Self::LastCompletedOpTaskNum(value)
        }
    }
    impl ::core::convert::From<LastCompletedOpTaskQuorumNumbersCall> for FinalizerTaskManagerCalls {
        fn from(value: LastCompletedOpTaskQuorumNumbersCall) -> Self {
            Self::LastCompletedOpTaskQuorumNumbers(value)
        }
    }
    impl ::core::convert::From<LastCompletedOpTaskQuorumThresholdPercentageCall>
        for FinalizerTaskManagerCalls
    {
        fn from(value: LastCompletedOpTaskQuorumThresholdPercentageCall) -> Self {
            Self::LastCompletedOpTaskQuorumThresholdPercentage(value)
        }
    }
    impl ::core::convert::From<LastCompletedRdTaskCompletedBlockCall> for FinalizerTaskManagerCalls {
        fn from(value: LastCompletedRdTaskCompletedBlockCall) -> Self {
            Self::LastCompletedRdTaskCompletedBlock(value)
        }
    }
    impl ::core::convert::From<LastCompletedRdTaskCreatedBlockCall> for FinalizerTaskManagerCalls {
        fn from(value: LastCompletedRdTaskCreatedBlockCall) -> Self {
            Self::LastCompletedRdTaskCreatedBlock(value)
        }
    }
    impl ::core::convert::From<LastCompletedRdTaskNumCall> for FinalizerTaskManagerCalls {
        fn from(value: LastCompletedRdTaskNumCall) -> Self {
            Self::LastCompletedRdTaskNum(value)
        }
    }
    impl ::core::convert::From<LastOpTaskCreatedBlockCall> for FinalizerTaskManagerCalls {
        fn from(value: LastOpTaskCreatedBlockCall) -> Self {
            Self::LastOpTaskCreatedBlock(value)
        }
    }
    impl ::core::convert::From<LastRdTaskCreatedBlockCall> for FinalizerTaskManagerCalls {
        fn from(value: LastRdTaskCreatedBlockCall) -> Self {
            Self::LastRdTaskCreatedBlock(value)
        }
    }
    impl ::core::convert::From<LatestOpTaskNumCall> for FinalizerTaskManagerCalls {
        fn from(value: LatestOpTaskNumCall) -> Self {
            Self::LatestOpTaskNum(value)
        }
    }
    impl ::core::convert::From<LatestRdTaskNumCall> for FinalizerTaskManagerCalls {
        fn from(value: LatestRdTaskNumCall) -> Self {
            Self::LatestRdTaskNum(value)
        }
    }
    impl ::core::convert::From<OperatorStateRetrieverExtendedCall> for FinalizerTaskManagerCalls {
        fn from(value: OperatorStateRetrieverExtendedCall) -> Self {
            Self::OperatorStateRetrieverExtended(value)
        }
    }
    impl ::core::convert::From<OperatorsStateInfoHashCall> for FinalizerTaskManagerCalls {
        fn from(value: OperatorsStateInfoHashCall) -> Self {
            Self::OperatorsStateInfoHash(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for FinalizerTaskManagerCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PauseCall> for FinalizerTaskManagerCalls {
        fn from(value: PauseCall) -> Self {
            Self::Pause(value)
        }
    }
    impl ::core::convert::From<PauseAllCall> for FinalizerTaskManagerCalls {
        fn from(value: PauseAllCall) -> Self {
            Self::PauseAll(value)
        }
    }
    impl ::core::convert::From<PausedWithIndexCall> for FinalizerTaskManagerCalls {
        fn from(value: PausedWithIndexCall) -> Self {
            Self::PausedWithIndex(value)
        }
    }
    impl ::core::convert::From<PausedCall> for FinalizerTaskManagerCalls {
        fn from(value: PausedCall) -> Self {
            Self::Paused(value)
        }
    }
    impl ::core::convert::From<PauserRegistryCall> for FinalizerTaskManagerCalls {
        fn from(value: PauserRegistryCall) -> Self {
            Self::PauserRegistry(value)
        }
    }
    impl ::core::convert::From<RegistryCoordinatorCall> for FinalizerTaskManagerCalls {
        fn from(value: RegistryCoordinatorCall) -> Self {
            Self::RegistryCoordinator(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for FinalizerTaskManagerCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<RespondToOpTaskCall> for FinalizerTaskManagerCalls {
        fn from(value: RespondToOpTaskCall) -> Self {
            Self::RespondToOpTask(value)
        }
    }
    impl ::core::convert::From<RespondToRdTaskCall> for FinalizerTaskManagerCalls {
        fn from(value: RespondToRdTaskCall) -> Self {
            Self::RespondToRdTask(value)
        }
    }
    impl ::core::convert::From<RolldownCall> for FinalizerTaskManagerCalls {
        fn from(value: RolldownCall) -> Self {
            Self::Rolldown(value)
        }
    }
    impl ::core::convert::From<SetAggregatorCall> for FinalizerTaskManagerCalls {
        fn from(value: SetAggregatorCall) -> Self {
            Self::SetAggregator(value)
        }
    }
    impl ::core::convert::From<SetGeneratorCall> for FinalizerTaskManagerCalls {
        fn from(value: SetGeneratorCall) -> Self {
            Self::SetGenerator(value)
        }
    }
    impl ::core::convert::From<SetPauserRegistryCall> for FinalizerTaskManagerCalls {
        fn from(value: SetPauserRegistryCall) -> Self {
            Self::SetPauserRegistry(value)
        }
    }
    impl ::core::convert::From<SetRolldownCall> for FinalizerTaskManagerCalls {
        fn from(value: SetRolldownCall) -> Self {
            Self::SetRolldown(value)
        }
    }
    impl ::core::convert::From<StakeRegistryCall> for FinalizerTaskManagerCalls {
        fn from(value: StakeRegistryCall) -> Self {
            Self::StakeRegistry(value)
        }
    }
    impl ::core::convert::From<TaskResponseWindowBlockCall> for FinalizerTaskManagerCalls {
        fn from(value: TaskResponseWindowBlockCall) -> Self {
            Self::TaskResponseWindowBlock(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for FinalizerTaskManagerCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UnpauseCall> for FinalizerTaskManagerCalls {
        fn from(value: UnpauseCall) -> Self {
            Self::Unpause(value)
        }
    }
    impl ::core::convert::From<UpdateBlsSignatureCheckerAddressCall> for FinalizerTaskManagerCalls {
        fn from(value: UpdateBlsSignatureCheckerAddressCall) -> Self {
            Self::UpdateBlsSignatureCheckerAddress(value)
        }
    }
    ///Container type for all return fields from the `THRESHOLD_DENOMINATOR` function with signature `THRESHOLD_DENOMINATOR()` and selector `0xef024458`
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
    pub struct ThresholdDenominatorReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `aggregator` function with signature `aggregator()` and selector `0x245a7bfc`
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
    pub struct AggregatorReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `allTaskHashes` function with signature `allTaskHashes(uint8,uint32)` and selector `0x13f815ed`
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
    pub struct AllTaskHashesReturn(pub [u8; 32]);
    ///Container type for all return fields from the `allTaskResponses` function with signature `allTaskResponses(uint8,uint32)` and selector `0x1ac27297`
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
    pub struct AllTaskResponsesReturn(pub [u8; 32]);
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
    ///Container type for all return fields from the `blsApkRegistry` function with signature `blsApkRegistry()` and selector `0x5df45946`
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
    pub struct BlsApkRegistryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `blsSignatureChecker` function with signature `blsSignatureChecker()` and selector `0x1c178e9c`
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
    pub struct BlsSignatureCheckerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `chainRdBatchNonce` function with signature `chainRdBatchNonce(uint64)` and selector `0x0ebf2a81`
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
    ///Container type for all return fields from the `checkSignatures` function with signature `checkSignatures(bytes32,bytes,uint32,(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))` and selector `0x6efb4636`
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
    pub struct CheckSignaturesReturn(pub QuorumStakeTotals, pub [u8; 32]);
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
        Hash,
    )]
    pub struct DelegationReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `generator` function with signature `generator()` and selector `0x7afa1eed`
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
    pub struct GeneratorReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getBatchOperatorFromId` function with signature `getBatchOperatorFromId(address,bytes32[])` and selector `0x4d2b57fe`
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
    pub struct GetBatchOperatorFromIdReturn {
        pub operators: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all return fields from the `getBatchOperatorId` function with signature `getBatchOperatorId(address,address[])` and selector `0x31b36bd9`
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
    pub struct GetBatchOperatorIdReturn {
        pub operator_ids: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all return fields from the `getCheckSignaturesIndices` function with signature `getCheckSignaturesIndices(address,uint32,bytes,bytes32[])` and selector `0x4f739f74`
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
    pub struct GetCheckSignaturesIndicesReturn(pub CheckSignaturesIndices);
    ///Container type for all return fields from the `getOperatorState` function with signature `getOperatorState(address,bytes,uint32)` and selector `0x3563b0d1`
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
    pub struct GetOperatorStateReturn(pub ::std::vec::Vec<::std::vec::Vec<Operator>>);
    ///Container type for all return fields from the `getOperatorState` function with signature `getOperatorState(address,bytes32,uint32)` and selector `0xcefdc1d4`
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
    pub struct GetOperatorStateWithRegistryCoordinatorAndOperatorIdReturn(
        pub ::ethers::core::types::U256,
        pub ::std::vec::Vec<::std::vec::Vec<Operator>>,
    );
    ///Container type for all return fields from the `getQuorumBitmapsAtBlockNumber` function with signature `getQuorumBitmapsAtBlockNumber(address,bytes32[],uint32)` and selector `0x5c155662`
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
    pub struct GetQuorumBitmapsAtBlockNumberReturn(
        pub ::std::vec::Vec<::ethers::core::types::U256>,
    );
    ///Container type for all return fields from the `idToTaskStatus` function with signature `idToTaskStatus(uint8,uint32)` and selector `0xbf2315ed`
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
    pub struct IdToTaskStatusReturn(pub u8);
    ///Container type for all return fields from the `isTaskPending` function with signature `isTaskPending()` and selector `0x36f78ed8`
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
    pub struct IsTaskPendingReturn(pub bool);
    ///Container type for all return fields from the `lastCompletedOpTaskCompletedBlock` function with signature `lastCompletedOpTaskCompletedBlock()` and selector `0x2c1101da`
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
    pub struct LastCompletedOpTaskCompletedBlockReturn(pub u32);
    ///Container type for all return fields from the `lastCompletedOpTaskCreatedBlock` function with signature `lastCompletedOpTaskCreatedBlock()` and selector `0x537a2929`
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
    pub struct LastCompletedOpTaskCreatedBlockReturn(pub u32);
    ///Container type for all return fields from the `lastCompletedOpTaskNum` function with signature `lastCompletedOpTaskNum()` and selector `0x8fc8729a`
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
    pub struct LastCompletedOpTaskNumReturn(pub u32);
    ///Container type for all return fields from the `lastCompletedOpTaskQuorumNumbers` function with signature `lastCompletedOpTaskQuorumNumbers()` and selector `0x2830e8f9`
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
    pub struct LastCompletedOpTaskQuorumNumbersReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `lastCompletedOpTaskQuorumThresholdPercentage` function with signature `lastCompletedOpTaskQuorumThresholdPercentage()` and selector `0xe70c2623`
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
    pub struct LastCompletedOpTaskQuorumThresholdPercentageReturn(pub u32);
    ///Container type for all return fields from the `lastCompletedRdTaskCompletedBlock` function with signature `lastCompletedRdTaskCompletedBlock()` and selector `0x3d7222eb`
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
    pub struct LastCompletedRdTaskCompletedBlockReturn(pub u32);
    ///Container type for all return fields from the `lastCompletedRdTaskCreatedBlock` function with signature `lastCompletedRdTaskCreatedBlock()` and selector `0xca50e56a`
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
    pub struct LastCompletedRdTaskCreatedBlockReturn(pub u32);
    ///Container type for all return fields from the `lastCompletedRdTaskNum` function with signature `lastCompletedRdTaskNum()` and selector `0x0980f1ec`
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
    pub struct LastCompletedRdTaskNumReturn(pub u32);
    ///Container type for all return fields from the `lastOpTaskCreatedBlock` function with signature `lastOpTaskCreatedBlock()` and selector `0x4d7a7116`
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
    pub struct LastOpTaskCreatedBlockReturn(pub u32);
    ///Container type for all return fields from the `lastRdTaskCreatedBlock` function with signature `lastRdTaskCreatedBlock()` and selector `0x8c82af5e`
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
    pub struct LastRdTaskCreatedBlockReturn(pub u32);
    ///Container type for all return fields from the `latestOpTaskNum` function with signature `latestOpTaskNum()` and selector `0x41789d57`
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
    pub struct LatestOpTaskNumReturn(pub u32);
    ///Container type for all return fields from the `latestRdTaskNum` function with signature `latestRdTaskNum()` and selector `0x7afdd54b`
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
    pub struct LatestRdTaskNumReturn(pub u32);
    ///Container type for all return fields from the `operatorStateRetrieverExtended` function with signature `operatorStateRetrieverExtended()` and selector `0x8380acbd`
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
    pub struct OperatorStateRetrieverExtendedReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `operatorsStateInfoHash` function with signature `operatorsStateInfoHash()` and selector `0xadfcb048`
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
    pub struct OperatorsStateInfoHashReturn(pub [u8; 32]);
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
        Hash,
    )]
    pub struct RegistryCoordinatorReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `stakeRegistry` function with signature `stakeRegistry()` and selector `0x68304835`
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
    pub struct StakeRegistryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `taskResponseWindowBlock` function with signature `taskResponseWindowBlock()` and selector `0xa69563a9`
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
    pub struct TaskResponseWindowBlockReturn(pub u32);
    ///`CheckSignaturesIndices(uint32[],uint32[],uint32[],uint32[][])`
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
    pub struct CheckSignaturesIndices {
        pub non_signer_quorum_bitmap_indices: ::std::vec::Vec<u32>,
        pub quorum_apk_indices: ::std::vec::Vec<u32>,
        pub total_stake_indices: ::std::vec::Vec<u32>,
        pub non_signer_stake_indices: ::std::vec::Vec<::std::vec::Vec<u32>>,
    }
    ///`Operator(address,bytes32,uint96)`
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
    pub struct Operator {
        pub operator: ::ethers::core::types::Address,
        pub operator_id: [u8; 32],
        pub stake: u128,
    }
}
