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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum IRolldownPrimitives.ChainId",
                                        ),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum IRolldownPrimitives.ChainId",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("pauseTrackingOpState"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "pauseTrackingOpState",
                            ),
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
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
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
                    ::std::borrow::ToOwned::to_owned("resumeTrackingQuorums"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "resumeTrackingQuorums",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "resetTrackedQuorums",
                                    ),
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
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
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
                    ::std::borrow::ToOwned::to_owned("PauseTrackingOpState"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PauseTrackingOpState",
                            ),
                            inputs: ::std::vec![],
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
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
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
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
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
                    ::std::borrow::ToOwned::to_owned("ResumeTrackingOpState"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ResumeTrackingOpState",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "resetTrackedQuorums",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa_\xB2\x80b\0\0!`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x03\xDBW`\x005`\xE0\x1C\x80ch0H5\x11a\x02\nW\x80c\x93\x03\x90\xD9\x11a\x01%W\x80c\xDECH8\x11a\0\xB8W\x80c\xF2\xFD\xE3\x8B\x11a\0\x87W\x80c\xF2\xFD\xE3\x8B\x14a\t\x1AW\x80c\xF5d\x0C\xF8\x14a\t-W\x80c\xF9\x12\n\xF6\x14a\t@W\x80c\xFA\xBC\x1C\xBC\x14a\tSW\x80c\xFD\xC1]\xE8\x14a\tfW`\0\x80\xFD[\x80c\xDECH8\x14a\x08\xE7W\x80c\xDF\\\xF7#\x14a\x08\xFAW\x80c\xE7\x0C&#\x14a\t\x02W\x80c\xEF\x02DX\x14a\t\x12W`\0\x80\xFD[\x80c\xBF#\x15\xED\x11a\0\xF4W\x80c\xBF#\x15\xED\x14a\x08_W\x80c\xCAP\xE5j\x14a\x08\x9AW\x80c\xCE\xFD\xC1\xD4\x14a\x08\xB3W\x80c\xD77\xD9\xF2\x14a\x08\xD4W`\0\x80\xFD[\x80c\x93\x03\x90\xD9\x14a\x08\x06W\x80c\xA6\x95c\xA9\x14a\x08,W\x80c\xAD\xFC\xB0H\x14a\x08CW\x80c\xB1\xED\xC8\xB4\x14a\x08LW`\0\x80\xFD[\x80cz\xFA\x1E\xED\x11a\x01\x9DW\x80c\x8C\x82\xAF^\x11a\x01lW\x80c\x8C\x82\xAF^\x14a\x07\xB3W\x80c\x8D\xA5\xCB[\x14a\x07\xCBW\x80c\x8F\xC8r\x9A\x14a\x07\xDCW\x80c\x92\xDD\xB1\x9C\x14a\x07\xF3W`\0\x80\xFD[\x80cz\xFA\x1E\xED\x14a\x07cW\x80cz\xFD\xD5K\x14a\x07vW\x80c\x83\x80\xAC\xBD\x14a\x07\x8DW\x80c\x88o\x11\x95\x14a\x07\xA0W`\0\x80\xFD[\x80co%H\x19\x11a\x01\xD9W\x80co%H\x19\x14a\x07-W\x80cqP\x18\xA6\x14a\x07@W\x80cr1\x14\xAB\x14a\x07HW\x80cy\xBA\xDFs\x14a\x07[W`\0\x80\xFD[\x80ch0H5\x14a\x06\xE9W\x80cm\x14\xA9\x87\x14a\x06\xF1W\x80cn\x12_\xF4\x14a\x06\xF9W\x80cn\xFBF6\x14a\x07\x0CW`\0\x80\xFD[\x80c=r\"\xEB\x11a\x02\xFAW\x80cSz))\x11a\x02\x8DW\x80c\\\x15Vb\x11a\x02\\W\x80c\\\x15Vb\x14a\x06\xB1W\x80c\\\x97Z\xBB\x14a\x06\xD1W\x80c]\xF4YF\x14a\x06\xD9W\x80c` /\xC0\x14a\x06\xE1W`\0\x80\xFD[\x80cSz))\x14a\x06aW\x80cT\xD1'\xDE\x14a\x06xW\x80cY\\jg\x14a\x06\x86W\x80cZ\xC8j\xB7\x14a\x06\x8EW`\0\x80\xFD[\x80cJ|~K\x11a\x02\xC9W\x80cJ|~K\x14a\x05\xFEW\x80cM+W\xFE\x14a\x06\x11W\x80cMzq\x16\x14a\x061W\x80cOs\x9Ft\x14a\x06AW`\0\x80\xFD[\x80c=r\"\xEB\x14a\x05\xACW\x80c=\x9F\xB0\x0C\x14a\x05\xC3W\x80cAx\x9DW\x14a\x05\xD6W\x80cE&[z\x14a\x05\xEDW`\0\x80\xFD[\x80c\x1C\x17\x8E\x9C\x11a\x03rW\x80c,\x11\x01\xDA\x11a\x03AW\x80c,\x11\x01\xDA\x14a\x05AW\x80c1\xB3k\xD9\x14a\x05XW\x80c5c\xB0\xD1\x14a\x05xW\x80c6\xF7\x8E\xD8\x14a\x05\x98W`\0\x80\xFD[\x80c\x1C\x17\x8E\x9C\x14a\x04\xDEW\x80c!\xE7\x80b\x14a\x05\tW\x80c$Z{\xFC\x14a\x05\x11W\x80c(0\xE8\xF9\x14a\x05,W`\0\x80\xFD[\x80c\x13d9\xDD\x11a\x03\xAEW\x80c\x13d9\xDD\x14a\x04TW\x80c\x13\xF8\x15\xED\x14a\x04gW\x80c\x19\x1A\xACz\x14a\x04\xA0W\x80c\x1A\xC2r\x97\x14a\x04\xB3W`\0\x80\xFD[\x80c\t\x80\xF1\xEC\x14a\x03\xE0W\x80c\t\xEC\x19\x06\x14a\x04\x0FW\x80c\x0E\xE0\xFD\xBD\x14a\x04$W\x80c\x10\xD6z/\x14a\x04AW[`\0\x80\xFD[`\xA3Ta\x03\xF5\x90a\x01\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x04\"a\x04\x1D6`\x04aB\x8FV[a\tyV[\0[`\xA3Ta\x041\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x04\x06V[a\x04\"a\x04O6`\x04aB\xF2V[a\n\xFCV[a\x04\"a\x04b6`\x04aC\x0FV[a\x0B\xB5V[a\x04\x92a\x04u6`\x04aCYV[`\x99` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[`@Q\x90\x81R` \x01a\x04\x06V[a\x04\"a\x04\xAE6`\x04aC\x9EV[a\x0C\xF4V[a\x04\x92a\x04\xC16`\x04aCYV[`\x9A` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[`\x97Ta\x04\xF1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x04\x06V[a\x04\"a\r5V[`\x9ETa\x04\xF1\x90d\x01\0\0\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x054a\r\xB9V[`@Qa\x04\x06\x91\x90aD\x08V[`\xA3Ta\x03\xF5\x90`\x01`H\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x05ka\x05f6`\x04aD\xCFV[a\x0EGV[`@Qa\x04\x06\x91\x90aE\xBDV[a\x05\x8Ba\x05\x866`\x04aE\xD0V[a\x0FcV[`@Qa\x04\x06\x91\x90aG+V[`\xA0Ta\x041\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[`\xA3Ta\x03\xF5\x90`\x01`h\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`\xA0Ta\x04\xF1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x98Ta\x03\xF5\x90`\x01`\xC0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x04\"a\x05\xFB6`\x04aG>V[PV[a\x04\"a\x06\x0C6`\x04aB\xF2V[a\x13\xFBV[a\x06$a\x06\x1F6`\x04aG\xDEV[a\x14QV[`@Qa\x04\x06\x91\x90aH-V[`\x9CTa\x03\xF5\x90c\xFF\xFF\xFF\xFF\x16\x81V[a\x06Ta\x06O6`\x04aH\xC2V[a\x15fV[`@Qa\x04\x06\x91\x90aJ\x10V[`\x9CTa\x03\xF5\x90`\x01``\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x04\"a\x05\xFB6`\x04aJ\x8EV[a\x04\"a\x1C\x8EV[a\x041a\x06\x9C6`\x04aJ\xC9V[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[a\x06\xC4a\x06\xBF6`\x04aJ\xECV[a\x1DUV[`@Qa\x04\x06\x91\x90aKOV[`fTa\x04\x92V[a\x04\xF1a\x1F\x1DV[a\x04\"a\x1F\x90V[a\x04\xF1a\x1F\x98V[a\x04\xF1a\x1F\xE2V[a\x04\"a\x07\x076`\x04aK\x87V[a ,V[a\x07\x1Fa\x07\x1A6`\x04aO\x0CV[a \xB7V[`@Qa\x04\x06\x92\x91\x90aO\xCCV[a\x04\"a\x07;6`\x04aP$V[a!WV[a\x04\"a$\x07V[a\x04\"a\x07V6`\x04aB\xF2V[a$\x19V[a\x04\"a$oV[`\x9FTa\x04\xF1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x98Ta\x03\xF5\x90`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`\x98Ta\x04\xF1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`eTa\x04\xF1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x9CTa\x03\xF5\x90d\x01\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x04\xF1V[`\x9CTa\x03\xF5\x90`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x04\"a\x08\x016`\x04aP@V[a$\xA2V[a\x03\xF5a\x08\x146`\x04aP\xB6V[`\xA1` R`\0\x90\x81R`@\x90 Tc\xFF\xFF\xFF\xFF\x16\x81V[`\x98Ta\x03\xF5\x90`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x04\x92`\xA2T\x81V[a\x04\"a\x08Z6`\x04aP\xD1V[a)\xCAV[a\x08\x8Da\x08m6`\x04aCYV[`\x9B` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[`@Qa\x04\x06\x91\x90aQ8V[`\xA3Ta\x03\xF5\x90e\x01\0\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x08\xC6a\x08\xC16`\x04aQRV[a,>V[`@Qa\x04\x06\x92\x91\x90aQ\x89V[a\x04\"a\x08\xE26`\x04aQ\xAAV[a-\xD0V[a\x04\"a\x08\xF56`\x04aR\x07V[a0\x82V[a\x04\xF1a2;V[`\x9ETa\x03\xF5\x90c\xFF\xFF\xFF\xFF\x16\x81V[a\x04\x92`d\x81V[a\x04\"a\t(6`\x04aB\xF2V[a2\x85V[a\x04\"a\t;6`\x04aK\x87V[a2\xFBV[a\x04\"a\tN6`\x04aB\xF2V[a3VV[a\x04\"a\ta6`\x04aC\x0FV[a3\xBAV[a\x04\"a\tt6`\x04aB\xF2V[a5\x16V[a\t\x81a5lV[a\t\xCD\x82`@Q` \x01a\t\x95\x91\x90aS1V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x91\x82\x01 \x90`\0\x90a\t\xBD\x90\x85\x01\x85aT\"V[a\x08Z`@\x87\x01` \x88\x01aT\"V[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`@\x80Q`\x80\x81\x01\x82Rc\xFF\xFF\xFF\xFFC\x16\x81R`\0` \x80\x83\x01\x82\x90R\x84\x81\x01Q\x93\x83\x01\x93\x90\x93R\x83Q``\x83\x01R\x91a\nS\x90\x83\x90a\n#\x90\x87\x01\x87aT\"V[\x86\x84`@Q` \x01a\n6\x92\x91\x90aT\xAFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\x04a5\xC6V[a\na\x84`@\x015\x86a6\x9AV[a\nn` \x85\x01\x85aT\"V[c\xFF\xFF\xFF\xFF\x16\x7F\xFF)\x08H=t\xB6\xB7\0S\xDDG2`\xAC\xF1\xB0\x9E\x0B\xA0x\x1B\xF9A\0\xBB\x82wX\x17I\xDE\x85`@Qa\n\xA3\x91\x90aT\xCFV[`@Q\x80\x91\x03\x90\xA2a\n\xB8` \x85\x01\x85aT\"V[c\xFF\xFF\xFF\xFF\x16\x7F\xDF\"\xF3U\x8EHA\xB6=w\x17\x95F\xB3\xEA\xE6>N4;\xBEu'F\xB0\x93\x16+\xC5&\xBEL\x85`@Qa\n\xED\x91\x90aT\xCFV[`@Q\x80\x91\x03\x90\xA2PPPPPV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0BOW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Bs\x91\x90aT\xDDV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0B\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xA3\x90aT\xFAV[`@Q\x80\x91\x03\x90\xFD[a\x05\xFB\x81a7tV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xFDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C!\x91\x90aUDV[a\x0C=W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xA3\x90aUaV[`fT\x81\x81\x16\x14a\x0C\xB6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\xA3V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[a\x0C\xFCa5lV[`@Q\x81\x15\x15\x81R\x7Fj\xF4\xAE\x1FH\x1A\xFF \xCEW\x1A\xBDe7[g\xB2#Y\x88:\x82=\x1D\xDFK\xD8\xF2\x87\x9F\xF7\xBA\x90` \x01[`@Q\x80\x91\x03\x90\xA1PV[`\x9FT`\x01`\x01`\xA0\x1B\x03\x163\x14a\r_W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xA3\x90aU\xA9V[`\xA0T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15\x15`\x01\x14a\r\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01RnNo task pending`\x88\x1B`D\x82\x01R`d\x01a\x0B\xA3V[a\r\xB7a8kV[V[`\x9D\x80Ta\r\xC6\x90aU\xC8V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\r\xF2\x90aU\xC8V[\x80\x15a\x0E?W\x80`\x1F\x10a\x0E\x14Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0E?V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0E\"W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[``\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0EbWa\x0EbaD\x1BV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E\x8BW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82Q\x81\x10\x15a\x0F\\W\x83`\x01`\x01`\xA0\x1B\x03\x16c\x13T*N\x84\x83\x81Q\x81\x10a\x0E\xBBWa\x0E\xBBaU\xFCV[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\xEE\x91\x90`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x0BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F/\x91\x90aV\x12V[\x82\x82\x81Q\x81\x10a\x0FAWa\x0FAaU\xFCV[` \x90\x81\x02\x91\x90\x91\x01\x01Ra\x0FU\x81aVAV[\x90Pa\x0E\x91V[P\x92\x91PPV[```\0\x84`\x01`\x01`\xA0\x1B\x03\x16ch0H5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xA5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xC9\x91\x90aT\xDDV[\x90P`\0\x85`\x01`\x01`\xA0\x1B\x03\x16c\x9E\x99#\xC2`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\x0BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10/\x91\x90aT\xDDV[\x90P`\0\x86`\x01`\x01`\xA0\x1B\x03\x16c]\xF4YF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10qW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x95\x91\x90aT\xDDV[\x90P`\0\x86Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x10\xB2Wa\x10\xB2aD\x1BV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x10\xE5W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x10\xD0W\x90P[P\x90P`\0[\x87Q\x81\x10\x15a\x13\xEDW`\0\x88\x82\x81Q\x81\x10a\x11\x08Wa\x11\x08aU\xFCV[\x01` \x01Q`@Qc\x89\x02bE`\xE0\x1B\x81R`\xF8\x91\x90\x91\x1C`\x04\x82\x01\x81\x90Rc\xFF\xFF\xFF\xFF\x8A\x16`$\x83\x01R\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x87\x16\x90c\x89\x02bE\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11iW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x11\x91\x91\x90\x81\x01\x90aVZV[\x90P\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x11\xACWa\x11\xACaD\x1BV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x11\xF7W\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a\x11\xCAW\x90P[P\x84\x84\x81Q\x81\x10a\x12\nWa\x12\naU\xFCV[` \x02` \x01\x01\x81\x90RP`\0[\x81Q\x81\x10\x15a\x13\xD7W`@Q\x80``\x01`@R\x80\x87`\x01`\x01`\xA0\x1B\x03\x16cG\xB3\x14\xE8\x85\x85\x81Q\x81\x10a\x12MWa\x12MaU\xFCV[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x12s\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\x90W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xB4\x91\x90aT\xDDV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x83\x81Q\x81\x10a\x12\xD4Wa\x12\xD4aU\xFCV[` \x02` \x01\x01Q\x81R` \x01\x89`\x01`\x01`\xA0\x1B\x03\x16c\xFA(\xC6'\x85\x85\x81Q\x81\x10a\x13\x02Wa\x13\x02aU\xFCV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x04\x81\x01\x91\x90\x91R`\xFF\x88\x16`$\x82\x01Rc\xFF\xFF\xFF\xFF\x8F\x16`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13^W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x82\x91\x90aW\x01V[`\x01`\x01``\x1B\x03\x16\x81RP\x85\x85\x81Q\x81\x10a\x13\xA0Wa\x13\xA0aU\xFCV[` \x02` \x01\x01Q\x82\x81Q\x81\x10a\x13\xB9Wa\x13\xB9aU\xFCV[` \x02` \x01\x01\x81\x90RP\x80\x80a\x13\xCF\x90aVAV[\x91PPa\x12\x18V[PPP\x80\x80a\x13\xE5\x90aVAV[\x91PPa\x10\xEBV[P\x93PPPP[\x93\x92PPPV[a\x14\x03a5lV[`\x9F\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F`\xF5\xACH\xA1?\x8B[\xF4\xB2\x13\xDE\x19\r\xD2\xDE%\x92\xC9\xF6\xF5\xAC}\xC1N=rk\x95\xDE\xD2\xDA\x90` \x01a\r*V[``\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14lWa\x14laD\x1BV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x14\x95W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82Q\x81\x10\x15a\x0F\\W\x83`\x01`\x01`\xA0\x1B\x03\x16c)k\xB0d\x84\x83\x81Q\x81\x10a\x14\xC5Wa\x14\xC5aU\xFCV[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14\xEB\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\x08W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15,\x91\x90aT\xDDV[\x82\x82\x81Q\x81\x10a\x15>Wa\x15>aU\xFCV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01Ra\x15_\x81aVAV[\x90Pa\x14\x9BV[a\x15\x91`@Q\x80`\x80\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`\0\x87`\x01`\x01`\xA0\x1B\x03\x16ch0H5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\xD1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xF5\x91\x90aT\xDDV[\x90Pa\x16\"`@Q\x80`\x80\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`@Qca\xC8\xA1/`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90c\xC3\x91B^\x90a\x16R\x90\x8B\x90\x89\x90\x89\x90`\x04\x01aW\x1CV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16oW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x16\x97\x91\x90\x81\x01\x90aWfV[\x81R`@Qc@\xE0:\x81`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\x81\xC0u\x02\x90a\x16\xC9\x90\x8B\x90\x8B\x90\x8B\x90`\x04\x01aW\xF4V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xE6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x17\x0E\x91\x90\x81\x01\x90aWfV[`@\x82\x01R\x85`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17+Wa\x17+aD\x1BV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x17^W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x17IW\x90P[P``\x82\x01R`\0[`\xFF\x81\x16\x87\x11\x15a\x1B\x9FW`\0\x85`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17\x8CWa\x17\x8CaD\x1BV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x17\xB5W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x83``\x01Q\x83`\xFF\x16\x81Q\x81\x10a\x17\xCFWa\x17\xCFaU\xFCV[` \x02` \x01\x01\x81\x90RP`\0[\x86\x81\x10\x15a\x1A\x9FW`\0\x8C`\x01`\x01`\xA0\x1B\x03\x16c\x04\xECcQ\x8A\x8A\x85\x81\x81\x10a\x18\x08Wa\x18\x08aU\xFCV[\x90P` \x02\x015\x8E\x88`\0\x01Q\x86\x81Q\x81\x10a\x18&Wa\x18&aU\xFCV[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18c\x93\x92\x91\x90\x92\x83Rc\xFF\xFF\xFF\xFF\x91\x82\x16` \x84\x01R\x16`@\x82\x01R``\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\x80W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\xA4\x91\x90aX\x14V[\x90P\x80`\x01`\x01`\xC0\x1B\x03\x16`\0\x03a\x19KW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R\x7FOperatorStateRetriever.getCheckS`D\x82\x01R\x7FignaturesIndices: operator must `d\x82\x01R\x7Fbe registered at blocknumber\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x0B\xA3V[\x8A\x8A\x85`\xFF\x16\x81\x81\x10a\x19`Wa\x19`aU\xFCV[`\x01`\x01`\x01`\xC0\x1B\x03\x85\x16\x91\x90\x93\x015`\xF8\x1C\x1C\x82\x16\x90\x91\x03\x90Pa\x1A\x8CW\x85`\x01`\x01`\xA0\x1B\x03\x16c\xDD\x98F\xB9\x8A\x8A\x85\x81\x81\x10a\x19\xA1Wa\x19\xA1aU\xFCV[\x90P` \x02\x015\x8D\x8D\x88`\xFF\x16\x81\x81\x10a\x19\xBDWa\x19\xBDaU\xFCV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x87\x90\x1B\x16\x81R`\x04\x81\x01\x94\x90\x94R\x91\x90\x91\x015`\xF8\x1C`$\x83\x01RPc\xFF\xFF\xFF\xFF\x8F\x16`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\x13W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A7\x91\x90aX=V[\x85``\x01Q\x85`\xFF\x16\x81Q\x81\x10a\x1APWa\x1APaU\xFCV[` \x02` \x01\x01Q\x84\x81Q\x81\x10a\x1AiWa\x1AiaU\xFCV[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x82a\x1A\x88\x81aVAV[\x93PP[P\x80a\x1A\x97\x81aVAV[\x91PPa\x17\xDDV[P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1A\xBAWa\x1A\xBAaD\x1BV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1A\xE3W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a\x1BdW\x84``\x01Q\x84`\xFF\x16\x81Q\x81\x10a\x1B\nWa\x1B\naU\xFCV[` \x02` \x01\x01Q\x81\x81Q\x81\x10a\x1B#Wa\x1B#aU\xFCV[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a\x1B=Wa\x1B=aU\xFCV[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x1B\\\x81aVAV[\x91PPa\x1A\xE9V[P\x80\x84``\x01Q\x84`\xFF\x16\x81Q\x81\x10a\x1B\x7FWa\x1B\x7FaU\xFCV[` \x02` \x01\x01\x81\x90RPPP\x80\x80a\x1B\x97\x90aXZV[\x91PPa\x17gV[P`\0\x89`\x01`\x01`\xA0\x1B\x03\x16c]\xF4YF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xE0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\x04\x91\x90aT\xDDV[`@Qc5IR\xA3`\xE2\x1B\x81R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xD5%J\x8C\x90a\x1C7\x90\x8B\x90\x8B\x90\x8E\x90`\x04\x01aXyV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1CTW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1C|\x91\x90\x81\x01\x90aWfV[` \x83\x01RP\x98\x97PPPPPPPPV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\xD6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xFA\x91\x90aUDV[a\x1D\x16W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xA3\x90aUaV[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[```\0\x84`\x01`\x01`\xA0\x1B\x03\x16c\xC3\x91B^\x84\x86`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1D\x87\x92\x91\x90aX\xA3V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\xA4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1D\xCC\x91\x90\x81\x01\x90aWfV[\x90P`\0\x84Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\xE9Wa\x1D\xE9aD\x1BV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1E\x12W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x85Q\x81\x10\x15a\x1F\x13W\x86`\x01`\x01`\xA0\x1B\x03\x16c\x04\xECcQ\x87\x83\x81Q\x81\x10a\x1EBWa\x1EBaU\xFCV[` \x02` \x01\x01Q\x87\x86\x85\x81Q\x81\x10a\x1E]Wa\x1E]aU\xFCV[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1E\x9A\x93\x92\x91\x90\x92\x83Rc\xFF\xFF\xFF\xFF\x91\x82\x16` \x84\x01R\x16`@\x82\x01R``\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\xB7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\xDB\x91\x90aX\x14V[`\x01`\x01`\xC0\x1B\x03\x16\x82\x82\x81Q\x81\x10a\x1E\xF6Wa\x1E\xF6aU\xFCV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a\x1F\x0B\x81aVAV[\x91PPa\x1E\x18V[P\x95\x94PPPPPV[`\x97T`@\x80Qc.\xFA,\xA3`\xE1\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c]\xF4YF\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1FgW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\x8B\x91\x90aT\xDDV[\x90P\x90V[a\r_a5lV[`\x97T`@\x80Qch0H5`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91ch0H5\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1FgW=`\0\x80>=`\0\xFD[`\x97T`@\x80Qcm\x14\xA9\x87`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cm\x14\xA9\x87\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1FgW=`\0\x80>=`\0\xFD[`\x9FT`\x01`\x01`\xA0\x1B\x03\x163\x14a VW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xA3\x90aU\xA9V[`\xA0T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a \xA7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RsTask already pending``\x1B`D\x82\x01R`d\x01a\x0B\xA3V[a \xB2\x83\x83\x83a:ZV[PPPV[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`\x97T`@Qc7}\xA3\x1B`\xE1\x1B\x81R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cn\xFBF6\x90a!\x04\x90\x8A\x90\x8A\x90\x8A\x90\x8A\x90\x8A\x90`\x04\x01aYFV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!!W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra!I\x91\x90\x81\x01\x90aZ\xA6V[\x91P\x91P\x95P\x95\x93PPPPV[`\x9FT`\x01`\x01`\xA0\x1B\x03\x163\x14a!\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xA3\x90aU\xA9V[`\xA0T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a!\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RsTask already pending``\x1B`D\x82\x01R`d\x01a\x0B\xA3V[`\x9CT`\x01``\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15\x80\x15\x90a!\xEFWPC\x15\x15[a\"-W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x13\xDC\x08\x14\xDD\x18]\x19H\x1D[\x9A[\x9A]`\x8A\x1B`D\x82\x01R`d\x01a\x0B\xA3V[`\x98T`@\x80Qa\x01\0\x81\x01\x90\x91R`\x01`\xE0\x1B\x90\x91\x04c\xFF\xFF\xFF\xFF\x16\x80\x82R\x90`\0\x90` \x81\x01\x85`\x02\x81\x11\x15a\"gWa\"gaQ\"V[\x81Rc\xFF\xFF\xFF\xFF\x80\x86\x16` \x83\x01RC\x81\x16`@\x83\x01R`\x9CT`\x01`@\x1B\x81\x04\x82\x16``\x84\x01R`\x01``\x1B\x90\x04\x16`\x80\x82\x01R`\x9D\x80T`\xA0\x90\x92\x01\x91a\"\xAF\x90aU\xC8V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\"\xDB\x90aU\xC8V[\x80\x15a#(W\x80`\x1F\x10a\"\xFDWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a#(V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a#\x0BW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x9ETc\xFF\xFF\xFF\xFF\x16` \x91\x82\x01R`@Q\x91\x92Pa#w\x91`\x01\x91\x85\x91a#Z\x91\x86\x91\x01a[VV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\x01a=BV[C`\x9C`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81c\xFF\xFF\xFF\xFF\x16\x7F\x0E\xB7s\xA9\xA1\x92k\t\x9CS\x01vk\xFA@\xAC\x9E>s\x94\xE5i\xEE\x8E\x01\xC6\xA3\x92\xC4K\x17\x98\x82`@Qa#\xCE\x91\x90a[VV[`@Q\x80\x91\x03\x90\xA2a#\xE1\x82`\x01a[\xFAV[`\x98`\x1Ca\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPPV[a$\x0Fa5lV[a\r\xB7`\0a>\x1CV[a$!a5lV[`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x90\x1AeM\xC80\xC9N\x8A\x12\xC9\xA3\xBC\n\x92\xAC\x11\xB5\xCF(\x04l\xA8\xD1\x90i\x1C\xDA\xF5 \x90\x16\x90` \x01a\r*V[a$wa5lV[`@Q\x7FM`\x15Bf\xB2\xEA\x0C\x8F\t\x1D%~\xACZ\xBC\x94\x1CF\xCBT\xD0\xC3\x06\x9A\x83\x0Fc9\xFE\x1D\xA1\x90`\0\x90\xA1V[`fT\x15a$\xF2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FPausable: contract is paused\0\0\0\0`D\x82\x01R`d\x01a\x0B\xA3V[`\x9ETd\x01\0\0\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x163\x14a%<W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04\x17WF\x83`\xDC\x1B`D\x82\x01R`d\x01a\x0B\xA3V[6`\0a%L`\xC0\x86\x01\x86a\\\"V[\x90\x92P\x90P`\0a%da\x01\0\x87\x01`\xE0\x88\x01aT\"V[\x90P`\xA1`\0a%z``\x88\x01`@\x89\x01aP\xB6V[`\x02\x81\x11\x15a%\x8BWa%\x8BaQ\"V[`\x02\x81\x11\x15a%\x9CWa%\x9CaQ\"V[\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 Tc\xFF\xFF\xFF\xFF\x16\x15\x80a&\x1FWP`\xA1`\0a%\xCE``\x88\x01`@\x89\x01aP\xB6V[`\x02\x81\x11\x15a%\xDFWa%\xDFaQ\"V[`\x02\x81\x11\x15a%\xF0Wa%\xF0aQ\"V[\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 Tc\xFF\xFF\xFF\xFF\x16a&\x17`\x80\x87\x01``\x88\x01aT\"V[c\xFF\xFF\xFF\xFF\x16\x14[a&kW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FchainRdBatchNonce mismatch\0\0\0\0\0\0`D\x82\x01R`d\x01a\x0B\xA3V[a&\xB7\x86`@Q` \x01a&\x7F\x91\x90a\\hV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x91\x82\x01 \x90`\x01\x90a&\xA7\x90\x89\x01\x89aT\"V[a\x08Z`\x80\x8B\x01``\x8C\x01aT\"V[`\0\x80a&\xFF\x87`@Q` \x01a&\xCE\x91\x90a]\xC9V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 a&\xF6`\xC0\x8B\x01`\xA0\x8C\x01aT\"V[\x88\x88\x88\x88a>nV[\x90\x92P\x90Pa'\x11` \x89\x01\x89aT\"V[c\xFF\xFF\xFF\xFF\x16\x7F\x82\xE5\xC8\xE9Du\x10\xB8g\xD2H\xC8\x928[\xA3O\xA6\xC2\xD4\xC4\xC2o\xF6\x86\x84\x99\xAE@'\xF2\xC6\x88\x83`@Qa'H\x92\x91\x90a]\xD8V[`@Q\x80\x91\x03\x90\xA2\x81\x15a'\x80Wa'{`\x01a'h` \x8A\x01\x8AaT\"V[\x89\x84`@Q` \x01a\n6\x92\x91\x90a]\xD8V[a'\xCCV[a'\xC2`\x01a'\x92` \x8A\x01\x8AaT\"V[\x89\x84`@Q` \x01a'\xA5\x92\x91\x90a]\xD8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\x03a5\xC6V[PPPPPPPPV[`\0\x80a'\xDF``\x8A\x01`@\x8B\x01aP\xB6V[`\x02\x81\x11\x15a'\xF0Wa'\xF0aQ\"V[\x03a(vW`@\x80Q\x80\x82\x01\x82R`\xA0\x8A\x81\x015\x82R`\xC0\x8B\x015` \x83\x01RT\x91Qb#\xD0\xB5`\xE6\x1B\x81R\x90\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x08\xF4-@\x90a(B\x90`\x80\x8D\x015\x90\x85\x90`\x04\x01a]\xFAV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a(\\W`\0\x80\xFD[PZ\xF1\x15\x80\x15a(pW=`\0\x80>=`\0\xFD[PPPPP[a(\x86`\x80\x89\x01``\x8A\x01aT\"V[a(\x91\x90`\x01a[\xFAV[`\xA1`\0a(\xA5``\x8C\x01`@\x8D\x01aP\xB6V[`\x02\x81\x11\x15a(\xB6Wa(\xB6aQ\"V[`\x02\x81\x11\x15a(\xC7Wa(\xC7aQ\"V[\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x93\x90\x93\x16\x92\x90\x92\x17\x90\x91Ua(\xFD\x90\x8A\x01\x8AaT\"V[`\xA3\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16a\x01\0\x02d\xFF\xFF\xFF\xFF\0\x19\x90\x92\x16\x91\x90\x91\x17\x90Ua).`\x80\x8A\x01``\x8B\x01aT\"V[`\xA3\x80Tp\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\0\x19\x16e\x01\0\0\0\0\0c\xFF\xFF\xFF\xFF\x93\x84\x16\x02c\xFF\xFF\xFF\xFF`h\x1B\x19\x16\x17`\x01`h\x1BC\x93\x90\x93\x16\x92\x90\x92\x02\x91\x90\x91\x17\x90Ua)\x82` \x89\x01\x89aT\"V[c\xFF\xFF\xFF\xFF\x16\x7F\x17\x97\xCAY\xE0n\xA4\xA0\xEF\xE1\n\xC0\xFBQ\xB5\x8C\x8A\xCF\\\xFE\xDB\xC1_\xAEQ\xC1\0!\xDC\xB9\x06\xE6\x89`@Qa)\xB7\x91\x90a]\xC9V[`@Q\x80\x91\x03\x90\xA2PPPPPPPPPV[`\xA0T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15\x15`\x01\x14a*\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01RnNo task pending`\x88\x1B`D\x82\x01R`d\x01a\x0B\xA3V[`\x99`\0\x84`\x01\x81\x11\x15a*0Wa*0aQ\"V[`\x01\x81\x11\x15a*AWa*AaQ\"V[\x81R` \x01\x90\x81R` \x01`\0 `\0\x83c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x84\x14a*\xABW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\n\x8C.md\r\xAD.m\xAC.\x8Cm`\x9B\x1B`D\x82\x01R`d\x01a\x0B\xA3V[`\x01`\x9B`\0\x85`\x01\x81\x11\x15a*\xC3Wa*\xC3aQ\"V[`\x01\x81\x11\x15a*\xD4Wa*\xD4aQ\"V[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 c\xFF\xFF\xFF\xFF\x87\x16\x82R\x90\x92R\x90 T`\xFF\x16`\x04\x81\x11\x15a+\x0CWa+\x0CaQ\"V[\x14a+JW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01RmNot Init state`\x90\x1B`D\x82\x01R`d\x01a\x0B\xA3V[`\0`\x9A\x81\x85`\x01\x81\x11\x15a+aWa+aaQ\"V[`\x01\x81\x11\x15a+rWa+raQ\"V[\x81R` \x01\x90\x81R` \x01`\0 `\0\x84c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x14a+\xD8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01Ri\x04\x16\xC7&G\x92\x05&W7`\xB4\x1B`D\x82\x01R`d\x01a\x0B\xA3V[`\x98Ta+\xF2\x90`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x82a[\xFAV[c\xFF\xFF\xFF\xFF\x16Cc\xFF\xFF\xFF\xFF\x16\x11\x15a,8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x08`$\x82\x01RgToo late`\xC0\x1B`D\x82\x01R`d\x01a\x0B\xA3V[PPPPV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91``\x91\x83\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x84\x81`\0\x81Q\x81\x10a,yWa,yaU\xFCV[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Qca\xC8\xA1/`\xE1\x1B\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90c\xC3\x91B^\x90a,\xB5\x90\x88\x90\x86\x90`\x04\x01aX\xA3V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,\xD2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra,\xFA\x91\x90\x81\x01\x90aWfV[`\0\x81Q\x81\x10a-\x0CWa-\x0CaU\xFCV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x88\x90Rc\xFF\xFF\xFF\xFF\x87\x81\x16`$\x83\x01R\x90\x91\x16`D\x82\x01\x81\x90R\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x89\x16\x90c\x04\xECcQ\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-xW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-\x9C\x91\x90aX\x14V[`\x01`\x01`\xC0\x1B\x03\x16\x90P`\0a-\xB2\x82a?\xE4V[\x90P\x81a-\xC0\x8A\x83\x8Aa\x0FcV[\x95P\x95PPPPP\x93P\x93\x91PPV[`fT\x15a. W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FPausable: contract is paused\0\0\0\0`D\x82\x01R`d\x01a\x0B\xA3V[`\x9ETd\x01\0\0\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x163\x14a.jW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04\x17WF\x83`\xDC\x1B`D\x82\x01R`d\x01a\x0B\xA3V[`\x9CT`\x01``\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15\x15\x80a.\x89WP`\xA3T`\xFF\x16[a.\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x1D\\\xD9H\x1C\x9B\xDB\xDD\x08\x1A[\x9A]`\x9A\x1B`D\x82\x01R`d\x01a\x0B\xA3V[6`\0a.\xD5`\xC0\x86\x01\x86a\\\"V[\x90\x92P\x90P`\0a.\xEDa\x01\0\x87\x01`\xE0\x88\x01aT\"V[\x90Pa/;\x86`@Q` \x01a/\x03\x91\x90aS1V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x91\x82\x01 \x90`\0\x90a/+\x90\x89\x01\x89aT\"V[a\x08Z`@\x8B\x01` \x8C\x01aT\"V[`\0\x80a/z\x87`@Q` \x01a/R\x91\x90aT\xCFV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 a&\xF6`\x80\x8B\x01``\x8C\x01aT\"V[\x90\x92P\x90Pa/\x8C` \x89\x01\x89aT\"V[c\xFF\xFF\xFF\xFF\x16\x7FG\xAD\xAC\xB0\xB6\xBB\xD7&\xAE9\xACl\0l\xCA\x1C \x06\xC9\xAE\xDA\xA8\x82\xDC\xBA|H\x04\xDB|A\xCE\x88\x83`@Qa/\xC3\x92\x91\x90aT\xAFV[`@Q\x80\x91\x03\x90\xA2\x81\x15a/\xFBWa/\xF6`\0a/\xE3` \x8A\x01\x8AaT\"V[\x89\x84`@Q` \x01a\n6\x92\x91\x90aT\xAFV[a0 V[a'\xC2`\0a0\r` \x8A\x01\x8AaT\"V[\x89\x84`@Q` \x01a'\xA5\x92\x91\x90aT\xAFV[a0.\x87`@\x015\x89a6\x9AV[a0;` \x88\x01\x88aT\"V[c\xFF\xFF\xFF\xFF\x16\x7F\xFF)\x08H=t\xB6\xB7\0S\xDDG2`\xAC\xF1\xB0\x9E\x0B\xA0x\x1B\xF9A\0\xBB\x82wX\x17I\xDE\x88`@Qa0p\x91\x90aT\xCFV[`@Q\x80\x91\x03\x90\xA2PPPPPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a0\xA2WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a0\xBCWP0;\x15\x80\x15a0\xBCWP`\0T`\xFF\x16`\x01\x14[a1\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x0B\xA3V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a1BW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a1M\x8A`\0a@\xB0V[a1V\x89a>\x1CV[`\x9E\x80Td\x01\0\0\0\0`\x01`\xC0\x1B\x03\x19\x16d\x01\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x8B\x81\x16\x91\x90\x91\x02\x91\x90\x91\x17\x90\x91U`\x9F\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x8A\x84\x16\x17\x90\x91U`\xA3\x80T`\xFF\x19\x16\x89\x15\x15\x17\x90U`\x97\x80T\x82\x16\x88\x84\x16\x17\x90U`\x98\x80T\x86\x84\x16`\x01`\x01`\xC0\x1B\x03\x19\x90\x91\x16\x17`\x01`\xA0\x1Bc\xFF\xFF\xFF\xFF\x89\x16\x02\x17\x90U`\xA0\x80T\x90\x91\x16\x91\x84\x16\x91\x90\x91\x17\x90U\x80\x15a2/W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPPPPV[`\x97T`@\x80Qc\xDF\\\xF7#`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xDF\\\xF7#\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1FgW=`\0\x80>=`\0\xFD[a2\x8Da5lV[`\x01`\x01`\xA0\x1B\x03\x81\x16a2\xF2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x0B\xA3V[a\x05\xFB\x81a>\x1CV[a3\x03a5lV[`\xA0T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a3\x1DWa3\x1Da8kV[a3(\x83\x83\x83a:ZV[`@Q\x7FN\xE9\x87\xE5\xF1\xBE\x19\xCA\xBF\xB1\xA2C\xE5\xC4#\x88\x9F\x06\x0F3&gS\x95?\xF0\xCF\x9D\xB8\x99f\xAB\x90`\0\x90\xA1PPPV[a3^a5lV[`\x9E\x80Td\x01\0\0\0\0`\x01`\xC0\x1B\x03\x19\x16d\x01\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x81\x02\x91\x90\x91\x17\x90\x91U`@Q\x90\x81R\x7F`,\xECK\x15\x83\xB0}\x07\x11a\xDA^\xB9X\x94D\xD2E\x92\x01\xE2\xFA\xB7u=\xC9A\xE95\x1C!\x90` \x01a\r*V[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a4\rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a41\x91\x90aT\xDDV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a4aW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xA3\x90aT\xFAV[`fT\x19\x81\x19`fT\x19\x16\x14a4\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\xA3V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x0C\xE9V[a5\x1Ea5lV[`\xA0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F/ \xCF\x1B\xDAgs\x90D\xC5\xBFWsS\x97\x0C=\xBC\x18;,rt\xD1\xE8XJ\x10&\x922g\x90` \x01a\r*V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\r\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x0B\xA3V[\x81`\x9A`\0\x86`\x01\x81\x11\x15a5\xDDWa5\xDDaQ\"V[`\x01\x81\x11\x15a5\xEEWa5\xEEaQ\"V[\x81R` \x01\x90\x81R` \x01`\0 `\0\x85c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP\x80`\x9B`\0\x86`\x01\x81\x11\x15a64Wa64aQ\"V[`\x01\x81\x11\x15a6EWa6EaQ\"V[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 c\xFF\xFF\xFF\xFF\x88\x16\x82R\x90\x92R\x90 \x80T`\xFF\x19\x16`\x01\x83`\x04\x81\x11\x15a6\x82Wa6\x82aQ\"V[\x02\x17\x90UPP`\xA0\x80T`\xFF`\xA0\x1B\x19\x16\x90UPPPV[`\xA2\x82\x90Ua6\xAF`@\x82\x01` \x83\x01aT\"V[`\x9C\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x01``\x1B\x02c\xFF\xFF\xFF\xFF``\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90Ua6\xE1`\x80\x82\x01\x82a\\\"V[a6\xED\x91`\x9D\x91aA\xCBV[Pa6\xFE`\xC0\x82\x01`\xA0\x83\x01aT\"V[`\x9E\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua7%` \x82\x01\x82aT\"V[`\x9C\x80Tk\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x16`\x01`@\x1Bc\xFF\xFF\xFF\xFF\x93\x84\x16\x02\x17\x90U`\xA3\x80Tl\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\x19\x16`\x01`H\x1BC\x90\x93\x16\x92\x90\x92\x02\x91\x90\x91\x17\x90UPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16a8\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x0B\xA3V[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x98T`\x01`\xC0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15a9[W`\x98T`\0\x90a8\x9F\x90`\x01\x90`\x01`\xC0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16a^\x18V[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R\x7F\x10\xAF\xAC\x923\xB4\xCC\xC5Md\x04\xFF\xC1\xCF;GQZ+\x8E\xDB\xF6u\xD1^\xDD\xCE\x05\xA0'\xDC\xBD` R`@\x90 T\x90\x91P`\x01\x90`\xFF\x16`\x04\x81\x11\x15a8\xEEWa8\xEEaQ\"V[\x03a9YWc\xFF\xFF\xFF\xFF\x81\x16`\0\x81\x81R\x7F\x10\xAF\xAC\x923\xB4\xCC\xC5Md\x04\xFF\xC1\xCF;GQZ+\x8E\xDB\xF6u\xD1^\xDD\xCE\x05\xA0'\xDC\xBD` R`@\x80\x82 \x80T`\xFF\x19\x16`\x02\x17\x90UQ\x7F\xD6\xA4\xE0\xFF\x9F:\x057\x08u|z\x12J\xBE\xE3\x1C\xEDa\xF4?\x17\xE6\xE1\xCF\x11\x94>\xC5\x9E`q\x91\x90\xA2[P[`\x98T`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15a:KW`\x98T`\0\x90a9\x8F\x90`\x01\x90`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16a^\x18V[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R\x7F)\x8C\x80\r\x08\x81\xDD \x8Dp^\xBC\x03\xEB\x18\x18\x9F8\x11\x82Y\xF2}\xD4;L`\xD6\x1C`~\x87` R`@\x90 T\x90\x91P`\x01\x90`\xFF\x16`\x04\x81\x11\x15a9\xDEWa9\xDEaQ\"V[\x03a:IWc\xFF\xFF\xFF\xFF\x81\x16`\0\x81\x81R\x7F)\x8C\x80\r\x08\x81\xDD \x8Dp^\xBC\x03\xEB\x18\x18\x9F8\x11\x82Y\xF2}\xD4;L`\xD6\x1C`~\x87` R`@\x80\x82 \x80T`\xFF\x19\x16`\x02\x17\x90UQ\x7F\x0B\xF4k\xFC\xA6\xE2\x13}5\xB8\x93\xC2\x95\xAD\xD8\xC3;\xCF\xBF\xFA\xFD\xEF\x93%,\xB5\x1A\xEDu8\xBA\x0C\x91\x90\xA2[P[`\xA0\x80T`\xFF`\xA0\x1B\x19\x16\x90UV[`\x9CTc\xFF\xFF\xFF\xFF`\x01``\x1B\x90\x91\x04\x16C\x14\x80\x15\x90a:yWPC\x15\x15[a:\xD6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FCan't in lastCompletedOpTaskCrea`D\x82\x01RgtedBlock`\xC0\x1B`d\x82\x01R`\x84\x01a\x0B\xA3V[`\x98T`@\x80Qa\x01\0\x81\x01\x82R`\0\x81\x83\x01\x81\x90R``\x80\x83\x01\x82\x90R`\x80\x83\x01\x81\x90R`\xC0\x83\x01R`\xE0\x82\x01Rc\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x90\x93\x04\x83\x16\x80\x82RC\x84\x16` \x80\x84\x01\x91\x90\x91R\x93\x87\x16`\xA0\x83\x01R\x82Q`\x1F\x86\x01\x85\x90\x04\x85\x02\x81\x01\x85\x01\x90\x93R\x84\x83R\x92\x90\x91\x90\x85\x90\x85\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x82\x90RP`\x80\x86\x01\x94\x90\x94RPP`\x9CT`\x01``\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x90\x91\x03\x90Pa;\xDFWc\xFF\xFF\xFF\xFF\x80\x83\x16`@\x80\x84\x01\x91\x90\x91RC\x90\x91\x16``\x83\x01R\x80Q` `\x1F\x86\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x84\x81R\x90\x85\x90\x85\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPPP`\xC0\x82\x01Rc\xFF\xFF\xFF\xFF\x85\x16`\xE0\x82\x01Ra<\xA2V[`\x9CTc\xFF\xFF\xFF\xFF`\x01`@\x1B\x82\x04\x81\x16`@\x84\x01R`\x01``\x1B\x90\x91\x04\x16``\x82\x01R`\x9D\x80Ta<\x10\x90aU\xC8V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta<<\x90aU\xC8V[\x80\x15a<\x89W\x80`\x1F\x10a<^Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a<\x89V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a<lW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`\xC0\x82\x01R`\x9ETc\xFF\xFF\xFF\xFF\x16`\xE0\x82\x01R[a<\xB9`\0\x83\x83`@Q` \x01a#Z\x91\x90a^=V[`\x9C\x80Tc\xFF\xFF\xFF\xFF\x19\x16Cc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x90\x91\x17\x90\x91U`@Q\x90\x83\x16\x90\x7F\xF3bg'\xFAn5e\xD6\xB23\xE2\x13l.\xE4R\xCF\x96\xE6@OQ\xD8\xCA\xA3G\xDD/{\x91\xF8\x90a=\x08\x90\x84\x90a^=V[`@Q\x80\x91\x03\x90\xA2a=\x1B\x82`\x01a[\xFAV[`\x98`\x18a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPPPV[\x81`\x99`\0\x86`\x01\x81\x11\x15a=YWa=YaQ\"V[`\x01\x81\x11\x15a=jWa=jaQ\"V[\x81R` \x01\x90\x81R` \x01`\0 `\0\x85c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP\x80`\x9B`\0\x86`\x01\x81\x11\x15a=\xB0Wa=\xB0aQ\"V[`\x01\x81\x11\x15a=\xC1Wa=\xC1aQ\"V[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 c\xFF\xFF\xFF\xFF\x88\x16\x82R\x90\x92R\x90 \x80T`\xFF\x19\x16`\x01\x83`\x04\x81\x11\x15a=\xFEWa=\xFEaQ\"V[\x02\x17\x90UPP`\xA0\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x17\x90UPPPV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R``\x82\x84\x01\x81\x90R\x80\x83\x01R`\x97T\x92Qc7}\xA3\x1B`\xE1\x1B\x81R\x90\x92\x83\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cn\xFBF6\x90a>\xCB\x90\x8D\x90\x8B\x90\x8B\x90\x8F\x90\x8F\x90`\x04\x01aYFV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a>\xE8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra?\x10\x91\x90\x81\x01\x90aZ\xA6V[`@\x80Q`\x80\x81\x01\x82Rc\xFF\xFF\xFF\xFFC\x16\x81R` \x80\x82\x01\x84\x90R\x84\x01Q\x91\x81\x01\x91\x90\x91R\x82Q``\x82\x01R\x91\x93P\x91P`\x01`\0[\x88\x81\x10\x15a?\xD3W\x87`\xFF\x16\x85` \x01Q\x82\x81Q\x81\x10a?hWa?haU\xFCV[` \x02` \x01\x01Qa?z\x91\x90a^\xF6V[`\x01`\x01``\x1B\x03\x16`d\x86`\0\x01Q\x83\x81Q\x81\x10a?\x9BWa?\x9BaU\xFCV[` \x02` \x01\x01Q`\x01`\x01``\x1B\x03\x16a?\xB6\x91\x90a_%V[\x10\x15a?\xC1W`\0\x91P[\x80a?\xCB\x81aVAV[\x91PPa?FV[P\x9B\x90\x9AP\x98PPPPPPPPPV[```\0\x80a?\xF2\x84aA\x9AV[a\xFF\xFF\x16`\x01`\x01`@\x1B\x03\x81\x11\x15a@\rWa@\raD\x1BV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a@7W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x80[\x82Q\x82\x10\x80\x15a@OWPa\x01\0\x81\x10[\x15a@\xA6W`\x01\x81\x1B\x93P\x85\x84\x16\x15a@\x96W\x80`\xF8\x1B\x83\x83\x81Q\x81\x10a@xWa@xaU\xFCV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x81`\x01\x01\x91P[a@\x9F\x81aVAV[\x90Pa@>V[P\x90\x94\x93PPPPV[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a@\xD1WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[aASW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x0B\xA3V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2aA\x96\x82a7tV[PPV[`\0\x80[\x82\x15aA\xC5WaA\xAF`\x01\x84a_DV[\x90\x92\x16\x91\x80aA\xBD\x81a_[V[\x91PPaA\x9EV[\x92\x91PPV[\x82\x80TaA\xD7\x90aU\xC8V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82aA\xF9W`\0\x85UaB?V[\x82`\x1F\x10aB\x12W\x82\x80\x01`\xFF\x19\x825\x16\x17\x85UaB?V[\x82\x80\x01`\x01\x01\x85U\x82\x15aB?W\x91\x82\x01[\x82\x81\x11\x15aB?W\x825\x82U\x91` \x01\x91\x90`\x01\x01\x90aB$V[PaBK\x92\x91PaBOV[P\x90V[[\x80\x82\x11\x15aBKW`\0\x81U`\x01\x01aBPV[`\0a\x01\0\x82\x84\x03\x12\x15aBwW`\0\x80\xFD[P\x91\x90PV[`\0``\x82\x84\x03\x12\x15aBwW`\0\x80\xFD[`\0\x80`\x80\x83\x85\x03\x12\x15aB\xA2W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aB\xB8W`\0\x80\xFD[aB\xC4\x85\x82\x86\x01aBdV[\x92PPaB\xD4\x84` \x85\x01aB}V[\x90P\x92P\x92\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05\xFBW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aC\x04W`\0\x80\xFD[\x815a\x13\xF4\x81aB\xDDV[`\0` \x82\x84\x03\x12\x15aC!W`\0\x80\xFD[P5\x91\x90PV[\x805`\x02\x81\x10aC7W`\0\x80\xFD[\x91\x90PV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x05\xFBW`\0\x80\xFD[\x805aC7\x81aC<V[`\0\x80`@\x83\x85\x03\x12\x15aClW`\0\x80\xFD[aCu\x83aC(V[\x91P` \x83\x015aC\x85\x81aC<V[\x80\x91PP\x92P\x92\x90PV[\x80\x15\x15\x81\x14a\x05\xFBW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aC\xB0W`\0\x80\xFD[\x815a\x13\xF4\x81aC\x90V[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15aC\xE1W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01aC\xC5V[\x81\x81\x11\x15aC\xF3W`\0` \x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x13\xF4` \x83\x01\x84aC\xBBV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aDSWaDSaD\x1BV[`@R\x90V[`@Qa\x01\0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aDSWaDSaD\x1BV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aD\xA4WaD\xA4aD\x1BV[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15aD\xC5WaD\xC5aD\x1BV[P`\x05\x1B` \x01\x90V[`\0\x80`@\x83\x85\x03\x12\x15aD\xE2W`\0\x80\xFD[\x825aD\xED\x81aB\xDDV[\x91P` \x83\x81\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aE\tW`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13aE\x1AW`\0\x80\xFD[\x805aE-aE(\x82aD\xACV[aD|V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x88\x83\x11\x15aELW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aEsW\x835aEd\x81aB\xDDV[\x82R\x92\x84\x01\x92\x90\x84\x01\x90aEQV[\x80\x95PPPPPP\x92P\x92\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aE\xB2W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aE\x96V[P\x94\x95\x94PPPPPV[` \x81R`\0a\x13\xF4` \x83\x01\x84aE\x82V[`\0\x80`\0``\x84\x86\x03\x12\x15aE\xE5W`\0\x80\xFD[\x835aE\xF0\x81aB\xDDV[\x92P` \x84\x81\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aF\rW`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12aF!W`\0\x80\xFD[\x815\x81\x81\x11\x15aF3WaF3aD\x1BV[aFE`\x1F\x82\x01`\x1F\x19\x16\x85\x01aD|V[\x91P\x80\x82R\x88\x84\x82\x85\x01\x01\x11\x15aF[W`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x94PPPPaF~`@\x85\x01aCNV[\x90P\x92P\x92P\x92V[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0\x80[\x86\x81\x10\x15aG\x1DW\x83\x85\x03\x8AR\x82Q\x80Q\x80\x87R\x90\x87\x01\x90\x87\x87\x01\x90\x84[\x81\x81\x10\x15aG\x08W\x83Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x8A\x81\x01Q\x8B\x85\x01R`@\x90\x81\x01Q`\x01`\x01``\x1B\x03\x16\x90\x84\x01R\x92\x89\x01\x92``\x90\x92\x01\x91`\x01\x01aF\xC4V[PP\x9A\x87\x01\x9A\x95PP\x91\x85\x01\x91`\x01\x01aF\xA6V[P\x92\x98\x97PPPPPPPPV[` \x81R`\0a\x13\xF4` \x83\x01\x84aF\x87V[`\0` \x82\x84\x03\x12\x15aGPW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aGfW`\0\x80\xFD[\x82\x01`@\x81\x85\x03\x12\x15a\x13\xF4W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12aG\x89W`\0\x80\xFD[\x815` aG\x99aE(\x83aD\xACV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aG\xB8W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aG\xD3W\x805\x83R\x91\x83\x01\x91\x83\x01aG\xBCV[P\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15aG\xF1W`\0\x80\xFD[\x825aG\xFC\x81aB\xDDV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aH\x17W`\0\x80\xFD[aH#\x85\x82\x86\x01aGxV[\x91PP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aHnW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aHIV[P\x90\x96\x95PPPPPPV[`\0\x80\x83`\x1F\x84\x01\x12aH\x8CW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aH\xA3W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aH\xBBW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\x80\x87\x89\x03\x12\x15aH\xDBW`\0\x80\xFD[\x865aH\xE6\x81aB\xDDV[\x95P` \x87\x015aH\xF6\x81aC<V[\x94P`@\x87\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aI\x12W`\0\x80\xFD[aI\x1E\x8A\x83\x8B\x01aHzV[\x90\x96P\x94P``\x89\x015\x91P\x80\x82\x11\x15aI7W`\0\x80\xFD[\x81\x89\x01\x91P\x89`\x1F\x83\x01\x12aIKW`\0\x80\xFD[\x815\x81\x81\x11\x15aIZW`\0\x80\xFD[\x8A` \x82`\x05\x1B\x85\x01\x01\x11\x15aIoW`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92\x95P\x92\x95P\x92\x95V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aE\xB2W\x81Qc\xFF\xFF\xFF\xFF\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aI\x99V[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0[\x85\x81\x10\x15aJ\x03W\x82\x84\x03\x89RaI\xF1\x84\x83QaI\x85V[\x98\x85\x01\x98\x93P\x90\x84\x01\x90`\x01\x01aI\xD9V[P\x91\x97\x96PPPPPPPV[` \x81R`\0\x82Q`\x80` \x84\x01RaJ,`\xA0\x84\x01\x82aI\x85V[\x90P` \x84\x01Q`\x1F\x19\x80\x85\x84\x03\x01`@\x86\x01RaJJ\x83\x83aI\x85V[\x92P`@\x86\x01Q\x91P\x80\x85\x84\x03\x01``\x86\x01RaJg\x83\x83aI\x85V[\x92P``\x86\x01Q\x91P\x80\x85\x84\x03\x01`\x80\x86\x01RPaJ\x85\x82\x82aI\xBBV[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15aJ\xA0W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aJ\xB6W`\0\x80\xFD[\x82\x01a\x01 \x81\x85\x03\x12\x15a\x13\xF4W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aJ\xDBW`\0\x80\xFD[\x815`\xFF\x81\x16\x81\x14a\x13\xF4W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15aK\x01W`\0\x80\xFD[\x835aK\x0C\x81aB\xDDV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aK'W`\0\x80\xFD[aK3\x86\x82\x87\x01aGxV[\x92PP`@\x84\x015aKD\x81aC<V[\x80\x91PP\x92P\x92P\x92V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aHnW\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aKkV[`\0\x80`\0`@\x84\x86\x03\x12\x15aK\x9CW`\0\x80\xFD[\x835aK\xA7\x81aC<V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aK\xC2W`\0\x80\xFD[aK\xCE\x86\x82\x87\x01aHzV[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x82`\x1F\x83\x01\x12aK\xECW`\0\x80\xFD[\x815` aK\xFCaE(\x83aD\xACV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aL\x1BW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aG\xD3W\x805aL2\x81aC<V[\x83R\x91\x83\x01\x91\x83\x01aL\x1FV[`\0`@\x82\x84\x03\x12\x15aLQW`\0\x80\xFD[aLYaD1V[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12aL\x80W`\0\x80\xFD[\x815` aL\x90aE(\x83aD\xACV[\x82\x81R`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aL\xAFW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aG\xD3WaL\xC5\x88\x82aL?V[\x83R\x91\x83\x01\x91`@\x01aL\xB3V[`\0\x82`\x1F\x83\x01\x12aL\xE4W`\0\x80\xFD[aL\xECaD1V[\x80`@\x84\x01\x85\x81\x11\x15aL\xFEW`\0\x80\xFD[\x84[\x81\x81\x10\x15aM\x18W\x805\x84R` \x93\x84\x01\x93\x01aM\0V[P\x90\x95\x94PPPPPV[`\0`\x80\x82\x84\x03\x12\x15aM5W`\0\x80\xFD[aM=aD1V[\x90PaMI\x83\x83aL\xD3V[\x81RaMX\x83`@\x84\x01aL\xD3V[` \x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12aMtW`\0\x80\xFD[\x815` aM\x84aE(\x83aD\xACV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aM\xA3W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aG\xD3W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aM\xC6W`\0\x80\x81\xFD[aM\xD4\x89\x86\x83\x8B\x01\x01aK\xDBV[\x84RP\x91\x83\x01\x91\x83\x01aM\xA7V[`\0a\x01\x80\x82\x84\x03\x12\x15aM\xF5W`\0\x80\xFD[aM\xFDaDYV[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aN\x16W`\0\x80\xFD[aN\"\x85\x83\x86\x01aK\xDBV[\x83R` \x84\x015\x91P\x80\x82\x11\x15aN8W`\0\x80\xFD[aND\x85\x83\x86\x01aLoV[` \x84\x01R`@\x84\x015\x91P\x80\x82\x11\x15aN]W`\0\x80\xFD[aNi\x85\x83\x86\x01aLoV[`@\x84\x01RaN{\x85``\x86\x01aM#V[``\x84\x01RaN\x8D\x85`\xE0\x86\x01aL?V[`\x80\x84\x01Ra\x01 \x84\x015\x91P\x80\x82\x11\x15aN\xA7W`\0\x80\xFD[aN\xB3\x85\x83\x86\x01aK\xDBV[`\xA0\x84\x01Ra\x01@\x84\x015\x91P\x80\x82\x11\x15aN\xCDW`\0\x80\xFD[aN\xD9\x85\x83\x86\x01aK\xDBV[`\xC0\x84\x01Ra\x01`\x84\x015\x91P\x80\x82\x11\x15aN\xF3W`\0\x80\xFD[PaO\0\x84\x82\x85\x01aMcV[`\xE0\x83\x01RP\x92\x91PPV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15aO$W`\0\x80\xFD[\x855\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aOBW`\0\x80\xFD[aON\x89\x83\x8A\x01aHzV[\x90\x96P\x94P`@\x88\x015\x91PaOc\x82aC<V[\x90\x92P``\x87\x015\x90\x80\x82\x11\x15aOyW`\0\x80\xFD[PaO\x86\x88\x82\x89\x01aM\xE2V[\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aE\xB2W\x81Q`\x01`\x01``\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aO\xA7V[`@\x81R`\0\x83Q`@\x80\x84\x01RaO\xE7`\x80\x84\x01\x82aO\x93V[\x90P` \x85\x01Q`?\x19\x84\x83\x03\x01``\x85\x01RaP\x04\x82\x82aO\x93V[\x92PPP\x82` \x83\x01R\x93\x92PPPV[\x805`\x03\x81\x10aC7W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15aP7W`\0\x80\xFD[aCu\x83aP\x15V[`\0\x80`\0a\x01@\x84\x86\x03\x12\x15aPVW`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aPmW`\0\x80\xFD[aPy\x87\x83\x88\x01aBdV[\x94PaP\x88\x87` \x88\x01aBdV[\x93Pa\x01 \x86\x015\x91P\x80\x82\x11\x15aP\x9FW`\0\x80\xFD[PaP\xAC\x86\x82\x87\x01aM\xE2V[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15aP\xC8W`\0\x80\xFD[a\x13\xF4\x82aP\x15V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aP\xE7W`\0\x80\xFD[\x845\x93PaP\xF7` \x86\x01aC(V[\x92P`@\x85\x015aQ\x07\x81aC<V[\x91P``\x85\x015aQ\x17\x81aC<V[\x93\x96\x92\x95P\x90\x93PPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x05\x83\x10aQLWaQLaQ\"V[\x91\x90R\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15aQgW`\0\x80\xFD[\x835aQr\x81aB\xDDV[\x92P` \x84\x015\x91P`@\x84\x015aKD\x81aC<V[\x82\x81R`@` \x82\x01R`\0aQ\xA2`@\x83\x01\x84aF\x87V[\x94\x93PPPPV[`\0\x80`\0`\xA0\x84\x86\x03\x12\x15aQ\xBFW`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aQ\xD6W`\0\x80\xFD[aQ\xE2\x87\x83\x88\x01aBdV[\x94PaQ\xF1\x87` \x88\x01aB}V[\x93P`\x80\x86\x015\x91P\x80\x82\x11\x15aP\x9FW`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0\x80`\0a\x01 \x8A\x8C\x03\x12\x15aR&W`\0\x80\xFD[\x895aR1\x81aB\xDDV[\x98P` \x8A\x015aRA\x81aB\xDDV[\x97P`@\x8A\x015aRQ\x81aB\xDDV[\x96P``\x8A\x015aRa\x81aB\xDDV[\x95P`\x80\x8A\x015aRq\x81aC\x90V[\x94P`\xA0\x8A\x015aR\x81\x81aB\xDDV[\x93P`\xC0\x8A\x015aR\x91\x81aC<V[\x92P`\xE0\x8A\x015aR\xA1\x81aB\xDDV[\x91Pa\x01\0\x8A\x015aR\xB2\x81aB\xDDV[\x80\x91PP\x92\x95\x98P\x92\x95\x98P\x92\x95\x98V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aR\xDAW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aR\xF9W`\0\x80\xFD[\x806\x03\x83\x13\x15aH\xBBW`\0\x80\xFD[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x81R`\0\x825aSB\x81aC<V[c\xFF\xFF\xFF\xFF\x81\x16` \x84\x01RPaS[` \x84\x01aCNV[c\xFF\xFF\xFF\xFF\x81\x16`@\x84\x01RPaSt`@\x84\x01aCNV[c\xFF\xFF\xFF\xFF\x81\x16``\x84\x01RPaS\x8D``\x84\x01aCNV[c\xFF\xFF\xFF\xFF\x81\x16`\x80\x84\x01RPaS\xA7`\x80\x84\x01\x84aR\xC3V[a\x01\0\x80`\xA0\x86\x01RaS\xBFa\x01 \x86\x01\x83\x85aS\x08V[\x92PaS\xCD`\xA0\x87\x01aCNV[c\xFF\xFF\xFF\xFF\x81\x16`\xC0\x87\x01R\x91PaS\xE8`\xC0\x87\x01\x87aR\xC3V[\x86\x85\x03`\x1F\x19\x01`\xE0\x88\x01R\x92PaT\x01\x84\x84\x83aS\x08V[\x93PPaT\x10`\xE0\x87\x01aCNV[c\xFF\xFF\xFF\xFF\x81\x16\x86\x83\x01R\x91Pa@\xA6V[`\0` \x82\x84\x03\x12\x15aT4W`\0\x80\xFD[\x815a\x13\xF4\x81aC<V[\x805aTJ\x81aC<V[c\xFF\xFF\xFF\xFF\x16\x82R` \x81\x81\x015\x90\x83\x01R`@\x90\x81\x015\x91\x01RV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Q` \x83\x01R`\0`@\x82\x01Q`\x80`@\x85\x01RaT\x96`\x80\x85\x01\x82aO\x93V[\x90P``\x83\x01Q\x84\x82\x03``\x86\x01RaJ\x85\x82\x82aO\x93V[aT\xB9\x81\x84aT?V[`\x80``\x82\x01R`\0aQ\xA2`\x80\x83\x01\x84aTgV[``\x81\x01aA\xC5\x82\x84aT?V[`\0` \x82\x84\x03\x12\x15aT\xEFW`\0\x80\xFD[\x81Qa\x13\xF4\x81aB\xDDV[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15aUVW`\0\x80\xFD[\x81Qa\x13\xF4\x81aC\x90V[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`\x05\x90\x82\x01RdAuth1`\xD8\x1B`@\x82\x01R``\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80aU\xDCW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aBwWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15aV$W`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01aVSWaVSaV+V[P`\x01\x01\x90V[`\0` \x80\x83\x85\x03\x12\x15aVmW`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aV\x83W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aV\x94W`\0\x80\xFD[\x80QaV\xA2aE(\x82aD\xACV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15aV\xC1W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aV\xDFW\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90aV\xC6V[\x97\x96PPPPPPPV[\x80Q`\x01`\x01``\x1B\x03\x81\x16\x81\x14aC7W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aW\x13W`\0\x80\xFD[a\x13\xF4\x82aV\xEAV[c\xFF\xFF\xFF\xFF\x84\x16\x81R`@` \x82\x01\x81\x90R\x81\x01\x82\x90R`\0`\x01`\x01`\xFB\x1B\x03\x83\x11\x15aWIW`\0\x80\xFD[\x82`\x05\x1B\x80\x85``\x85\x017`\0\x92\x01``\x01\x91\x82RP\x93\x92PPPV[`\0` \x80\x83\x85\x03\x12\x15aWyW`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aW\x8FW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aW\xA0W`\0\x80\xFD[\x80QaW\xAEaE(\x82aD\xACV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15aW\xCDW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aV\xDFW\x83QaW\xE5\x81aC<V[\x82R\x92\x84\x01\x92\x90\x84\x01\x90aW\xD2V[c\xFF\xFF\xFF\xFF\x84\x16\x81R`@` \x82\x01R`\0aJ\x85`@\x83\x01\x84\x86aS\x08V[`\0` \x82\x84\x03\x12\x15aX&W`\0\x80\xFD[\x81Q`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x14a\x13\xF4W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aXOW`\0\x80\xFD[\x81Qa\x13\xF4\x81aC<V[`\0`\xFF\x82\x16`\xFF\x81\x03aXpWaXpaV+V[`\x01\x01\x92\x91PPV[`@\x81R`\0aX\x8D`@\x83\x01\x85\x87aS\x08V[\x90Pc\xFF\xFF\xFF\xFF\x83\x16` \x83\x01R\x94\x93PPPPV[c\xFF\xFF\xFF\xFF\x83\x16\x81R`@` \x82\x01R`\0aQ\xA2`@\x83\x01\x84aE\x82V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aE\xB2WaX\xF3\x87\x83Q\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[`@\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01aX\xD6V[\x80`\0[`\x02\x81\x10\x15a,8W\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01aY\nV[aY4\x82\x82QaY\x06V[` \x81\x01Qa \xB2`@\x84\x01\x82aY\x06V[\x85\x81R`\x80` \x82\x01R`\0aY``\x80\x83\x01\x86\x88aS\x08V[c\xFF\xFF\xFF\xFF\x85\x16`@\x84\x01R\x82\x81\x03``\x84\x01Ra\x01\x80\x84Q\x81\x83RaY\x88\x82\x84\x01\x82aI\x85V[\x91PP` \x85\x01Q\x82\x82\x03` \x84\x01RaY\xA2\x82\x82aX\xC2V[\x91PP`@\x85\x01Q\x82\x82\x03`@\x84\x01RaY\xBC\x82\x82aX\xC2V[\x91PP``\x85\x01QaY\xD1``\x84\x01\x82aY)V[P`\x80\x85\x01Q\x80Q`\xE0\x84\x01R` \x01Qa\x01\0\x83\x01R`\xA0\x85\x01Q\x82\x82\x03a\x01 \x84\x01RaZ\0\x82\x82aI\x85V[\x91PP`\xC0\x85\x01Q\x82\x82\x03a\x01@\x84\x01RaZ\x1B\x82\x82aI\x85V[\x91PP`\xE0\x85\x01Q\x82\x82\x03a\x01`\x84\x01RaZ6\x82\x82aI\xBBV[\x9A\x99PPPPPPPPPPV[`\0\x82`\x1F\x83\x01\x12aZUW`\0\x80\xFD[\x81Q` aZeaE(\x83aD\xACV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aZ\x84W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aG\xD3WaZ\x99\x81aV\xEAV[\x83R\x91\x83\x01\x91\x83\x01aZ\x88V[`\0\x80`@\x83\x85\x03\x12\x15aZ\xB9W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aZ\xD0W`\0\x80\xFD[\x90\x84\x01\x90`@\x82\x87\x03\x12\x15aZ\xE4W`\0\x80\xFD[aZ\xECaD1V[\x82Q\x82\x81\x11\x15aZ\xFBW`\0\x80\xFD[a[\x07\x88\x82\x86\x01aZDV[\x82RP` \x83\x01Q\x82\x81\x11\x15a[\x1CW`\0\x80\xFD[a[(\x88\x82\x86\x01aZDV[` \x83\x01RP\x80\x94PPPP` \x83\x01Q\x90P\x92P\x92\x90PV[`\x03\x81\x10a[RWa[RaQ\"V[\x90RV[` \x81R`\0c\xFF\xFF\xFF\xFF\x80\x84Q\x16` \x84\x01R` \x84\x01Qa[|`@\x85\x01\x82a[BV[P\x80`@\x85\x01Q\x16``\x84\x01RP``\x83\x01Qa[\xA1`\x80\x84\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P`\x80\x83\x01Qc\xFF\xFF\xFF\xFF\x81\x16`\xA0\x84\x01RP`\xA0\x83\x01Qc\xFF\xFF\xFF\xFF\x81\x16`\xC0\x84\x01RP`\xC0\x83\x01Qa\x01\0\x80`\xE0\x85\x01Ra[\xE2a\x01 \x85\x01\x83aC\xBBV[\x91P`\xE0\x85\x01Qa@\xA6\x82\x86\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a\\\x19Wa\\\x19aV+V[\x01\x94\x93PPPPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\\9W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\\SW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15aH\xBBW`\0\x80\xFD[` \x81R`\0\x825a\\y\x81aC<V[c\xFF\xFF\xFF\xFF\x81\x16` \x84\x01RPa\\\x92` \x84\x01aP\x15V[a\\\x9F`@\x84\x01\x82a[BV[Pa\\\xAC`@\x84\x01aCNV[c\xFF\xFF\xFF\xFF\x81\x16``\x84\x01RPa\\\xC5``\x84\x01aCNV[c\xFF\xFF\xFF\xFF\x81\x16`\x80\x84\x01RPa\\\xDE`\x80\x84\x01aCNV[c\xFF\xFF\xFF\xFF\x81\x16`\xA0\x84\x01RPa\\\xF7`\xA0\x84\x01aCNV[c\xFF\xFF\xFF\xFF\x81\x16`\xC0\x84\x01RPa]\x11`\xC0\x84\x01\x84aR\xC3V[a\x01\0\x80`\xE0\x86\x01Ra])a\x01 \x86\x01\x83\x85aS\x08V[\x92PaT\x10`\xE0\x87\x01aCNV[\x805a]B\x81aC<V[c\xFF\xFF\xFF\xFF\x80\x82\x16\x84R` \x83\x015` \x85\x01Ra]b`@\x84\x01aP\x15V[\x91Pa]q`@\x85\x01\x83a[BV[``\x83\x015\x91Pa]\x81\x82aC<V[\x16``\x83\x01R`\x80\x81\x81\x015\x90\x83\x01R`\xA0\x80\x82\x015\x90\x83\x01R`\xC0\x80\x82\x015\x90\x83\x01R`\xE0\x81\x015a]\xB3\x81aB\xDDV[`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x92\x90\x92\x01\x91\x90\x91RPV[a\x01\0\x81\x01aA\xC5\x82\x84a]7V[`\0a\x01 a]\xE7\x83\x86a]7V[\x80a\x01\0\x84\x01RaJ\x85\x81\x84\x01\x85aTgV[\x82\x81R``\x81\x01a\x13\xF4` \x83\x01\x84\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[`\0c\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a^5Wa^5aV+V[\x03\x93\x92PPPV[` \x81Rc\xFF\xFF\xFF\xFF\x82Q\x16` \x82\x01R`\0` \x83\x01Qa^g`@\x84\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P`@\x83\x01Qc\xFF\xFF\xFF\xFF\x81\x16``\x84\x01RP``\x83\x01Qc\xFF\xFF\xFF\xFF\x81\x16`\x80\x84\x01RP`\x80\x83\x01Qa\x01\0\x80`\xA0\x85\x01Ra^\xA8a\x01 \x85\x01\x83aC\xBBV[\x91P`\xA0\x85\x01Qa^\xC1`\xC0\x86\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P`\xC0\x85\x01Q\x84\x83\x03`\x1F\x19\x01`\xE0\x86\x01Ra^\xDD\x83\x82aC\xBBV[\x92PP`\xE0\x85\x01Qa@\xA6\x82\x86\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15a_\x1CWa_\x1CaV+V[\x02\x94\x93PPPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a_?Wa_?aV+V[P\x02\x90V[`\0\x82\x82\x10\x15a_VWa_VaV+V[P\x03\x90V[`\0a\xFF\xFF\x80\x83\x16\x81\x81\x03a_rWa_raV+V[`\x01\x01\x93\x92PPPV\xFE\xA2dipfsX\"\x12 `\xC8\xA9\x045\xE2,\x1E1\x0C-\xF1q\x96\x89G\x96\xA8\x7F\xF5\x81W\xEDUV|\x1B\x86:\x9A\x86\x15dsolcC\0\x08\r\x003";
    /// The bytecode of the contract.
    pub static FINALIZERTASKMANAGER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x03\xDBW`\x005`\xE0\x1C\x80ch0H5\x11a\x02\nW\x80c\x93\x03\x90\xD9\x11a\x01%W\x80c\xDECH8\x11a\0\xB8W\x80c\xF2\xFD\xE3\x8B\x11a\0\x87W\x80c\xF2\xFD\xE3\x8B\x14a\t\x1AW\x80c\xF5d\x0C\xF8\x14a\t-W\x80c\xF9\x12\n\xF6\x14a\t@W\x80c\xFA\xBC\x1C\xBC\x14a\tSW\x80c\xFD\xC1]\xE8\x14a\tfW`\0\x80\xFD[\x80c\xDECH8\x14a\x08\xE7W\x80c\xDF\\\xF7#\x14a\x08\xFAW\x80c\xE7\x0C&#\x14a\t\x02W\x80c\xEF\x02DX\x14a\t\x12W`\0\x80\xFD[\x80c\xBF#\x15\xED\x11a\0\xF4W\x80c\xBF#\x15\xED\x14a\x08_W\x80c\xCAP\xE5j\x14a\x08\x9AW\x80c\xCE\xFD\xC1\xD4\x14a\x08\xB3W\x80c\xD77\xD9\xF2\x14a\x08\xD4W`\0\x80\xFD[\x80c\x93\x03\x90\xD9\x14a\x08\x06W\x80c\xA6\x95c\xA9\x14a\x08,W\x80c\xAD\xFC\xB0H\x14a\x08CW\x80c\xB1\xED\xC8\xB4\x14a\x08LW`\0\x80\xFD[\x80cz\xFA\x1E\xED\x11a\x01\x9DW\x80c\x8C\x82\xAF^\x11a\x01lW\x80c\x8C\x82\xAF^\x14a\x07\xB3W\x80c\x8D\xA5\xCB[\x14a\x07\xCBW\x80c\x8F\xC8r\x9A\x14a\x07\xDCW\x80c\x92\xDD\xB1\x9C\x14a\x07\xF3W`\0\x80\xFD[\x80cz\xFA\x1E\xED\x14a\x07cW\x80cz\xFD\xD5K\x14a\x07vW\x80c\x83\x80\xAC\xBD\x14a\x07\x8DW\x80c\x88o\x11\x95\x14a\x07\xA0W`\0\x80\xFD[\x80co%H\x19\x11a\x01\xD9W\x80co%H\x19\x14a\x07-W\x80cqP\x18\xA6\x14a\x07@W\x80cr1\x14\xAB\x14a\x07HW\x80cy\xBA\xDFs\x14a\x07[W`\0\x80\xFD[\x80ch0H5\x14a\x06\xE9W\x80cm\x14\xA9\x87\x14a\x06\xF1W\x80cn\x12_\xF4\x14a\x06\xF9W\x80cn\xFBF6\x14a\x07\x0CW`\0\x80\xFD[\x80c=r\"\xEB\x11a\x02\xFAW\x80cSz))\x11a\x02\x8DW\x80c\\\x15Vb\x11a\x02\\W\x80c\\\x15Vb\x14a\x06\xB1W\x80c\\\x97Z\xBB\x14a\x06\xD1W\x80c]\xF4YF\x14a\x06\xD9W\x80c` /\xC0\x14a\x06\xE1W`\0\x80\xFD[\x80cSz))\x14a\x06aW\x80cT\xD1'\xDE\x14a\x06xW\x80cY\\jg\x14a\x06\x86W\x80cZ\xC8j\xB7\x14a\x06\x8EW`\0\x80\xFD[\x80cJ|~K\x11a\x02\xC9W\x80cJ|~K\x14a\x05\xFEW\x80cM+W\xFE\x14a\x06\x11W\x80cMzq\x16\x14a\x061W\x80cOs\x9Ft\x14a\x06AW`\0\x80\xFD[\x80c=r\"\xEB\x14a\x05\xACW\x80c=\x9F\xB0\x0C\x14a\x05\xC3W\x80cAx\x9DW\x14a\x05\xD6W\x80cE&[z\x14a\x05\xEDW`\0\x80\xFD[\x80c\x1C\x17\x8E\x9C\x11a\x03rW\x80c,\x11\x01\xDA\x11a\x03AW\x80c,\x11\x01\xDA\x14a\x05AW\x80c1\xB3k\xD9\x14a\x05XW\x80c5c\xB0\xD1\x14a\x05xW\x80c6\xF7\x8E\xD8\x14a\x05\x98W`\0\x80\xFD[\x80c\x1C\x17\x8E\x9C\x14a\x04\xDEW\x80c!\xE7\x80b\x14a\x05\tW\x80c$Z{\xFC\x14a\x05\x11W\x80c(0\xE8\xF9\x14a\x05,W`\0\x80\xFD[\x80c\x13d9\xDD\x11a\x03\xAEW\x80c\x13d9\xDD\x14a\x04TW\x80c\x13\xF8\x15\xED\x14a\x04gW\x80c\x19\x1A\xACz\x14a\x04\xA0W\x80c\x1A\xC2r\x97\x14a\x04\xB3W`\0\x80\xFD[\x80c\t\x80\xF1\xEC\x14a\x03\xE0W\x80c\t\xEC\x19\x06\x14a\x04\x0FW\x80c\x0E\xE0\xFD\xBD\x14a\x04$W\x80c\x10\xD6z/\x14a\x04AW[`\0\x80\xFD[`\xA3Ta\x03\xF5\x90a\x01\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x04\"a\x04\x1D6`\x04aB\x8FV[a\tyV[\0[`\xA3Ta\x041\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x04\x06V[a\x04\"a\x04O6`\x04aB\xF2V[a\n\xFCV[a\x04\"a\x04b6`\x04aC\x0FV[a\x0B\xB5V[a\x04\x92a\x04u6`\x04aCYV[`\x99` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[`@Q\x90\x81R` \x01a\x04\x06V[a\x04\"a\x04\xAE6`\x04aC\x9EV[a\x0C\xF4V[a\x04\x92a\x04\xC16`\x04aCYV[`\x9A` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[`\x97Ta\x04\xF1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x04\x06V[a\x04\"a\r5V[`\x9ETa\x04\xF1\x90d\x01\0\0\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x054a\r\xB9V[`@Qa\x04\x06\x91\x90aD\x08V[`\xA3Ta\x03\xF5\x90`\x01`H\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x05ka\x05f6`\x04aD\xCFV[a\x0EGV[`@Qa\x04\x06\x91\x90aE\xBDV[a\x05\x8Ba\x05\x866`\x04aE\xD0V[a\x0FcV[`@Qa\x04\x06\x91\x90aG+V[`\xA0Ta\x041\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[`\xA3Ta\x03\xF5\x90`\x01`h\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`\xA0Ta\x04\xF1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x98Ta\x03\xF5\x90`\x01`\xC0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x04\"a\x05\xFB6`\x04aG>V[PV[a\x04\"a\x06\x0C6`\x04aB\xF2V[a\x13\xFBV[a\x06$a\x06\x1F6`\x04aG\xDEV[a\x14QV[`@Qa\x04\x06\x91\x90aH-V[`\x9CTa\x03\xF5\x90c\xFF\xFF\xFF\xFF\x16\x81V[a\x06Ta\x06O6`\x04aH\xC2V[a\x15fV[`@Qa\x04\x06\x91\x90aJ\x10V[`\x9CTa\x03\xF5\x90`\x01``\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x04\"a\x05\xFB6`\x04aJ\x8EV[a\x04\"a\x1C\x8EV[a\x041a\x06\x9C6`\x04aJ\xC9V[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[a\x06\xC4a\x06\xBF6`\x04aJ\xECV[a\x1DUV[`@Qa\x04\x06\x91\x90aKOV[`fTa\x04\x92V[a\x04\xF1a\x1F\x1DV[a\x04\"a\x1F\x90V[a\x04\xF1a\x1F\x98V[a\x04\xF1a\x1F\xE2V[a\x04\"a\x07\x076`\x04aK\x87V[a ,V[a\x07\x1Fa\x07\x1A6`\x04aO\x0CV[a \xB7V[`@Qa\x04\x06\x92\x91\x90aO\xCCV[a\x04\"a\x07;6`\x04aP$V[a!WV[a\x04\"a$\x07V[a\x04\"a\x07V6`\x04aB\xF2V[a$\x19V[a\x04\"a$oV[`\x9FTa\x04\xF1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x98Ta\x03\xF5\x90`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`\x98Ta\x04\xF1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`eTa\x04\xF1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x9CTa\x03\xF5\x90d\x01\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x04\xF1V[`\x9CTa\x03\xF5\x90`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x04\"a\x08\x016`\x04aP@V[a$\xA2V[a\x03\xF5a\x08\x146`\x04aP\xB6V[`\xA1` R`\0\x90\x81R`@\x90 Tc\xFF\xFF\xFF\xFF\x16\x81V[`\x98Ta\x03\xF5\x90`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x04\x92`\xA2T\x81V[a\x04\"a\x08Z6`\x04aP\xD1V[a)\xCAV[a\x08\x8Da\x08m6`\x04aCYV[`\x9B` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[`@Qa\x04\x06\x91\x90aQ8V[`\xA3Ta\x03\xF5\x90e\x01\0\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x08\xC6a\x08\xC16`\x04aQRV[a,>V[`@Qa\x04\x06\x92\x91\x90aQ\x89V[a\x04\"a\x08\xE26`\x04aQ\xAAV[a-\xD0V[a\x04\"a\x08\xF56`\x04aR\x07V[a0\x82V[a\x04\xF1a2;V[`\x9ETa\x03\xF5\x90c\xFF\xFF\xFF\xFF\x16\x81V[a\x04\x92`d\x81V[a\x04\"a\t(6`\x04aB\xF2V[a2\x85V[a\x04\"a\t;6`\x04aK\x87V[a2\xFBV[a\x04\"a\tN6`\x04aB\xF2V[a3VV[a\x04\"a\ta6`\x04aC\x0FV[a3\xBAV[a\x04\"a\tt6`\x04aB\xF2V[a5\x16V[a\t\x81a5lV[a\t\xCD\x82`@Q` \x01a\t\x95\x91\x90aS1V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x91\x82\x01 \x90`\0\x90a\t\xBD\x90\x85\x01\x85aT\"V[a\x08Z`@\x87\x01` \x88\x01aT\"V[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`@\x80Q`\x80\x81\x01\x82Rc\xFF\xFF\xFF\xFFC\x16\x81R`\0` \x80\x83\x01\x82\x90R\x84\x81\x01Q\x93\x83\x01\x93\x90\x93R\x83Q``\x83\x01R\x91a\nS\x90\x83\x90a\n#\x90\x87\x01\x87aT\"V[\x86\x84`@Q` \x01a\n6\x92\x91\x90aT\xAFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\x04a5\xC6V[a\na\x84`@\x015\x86a6\x9AV[a\nn` \x85\x01\x85aT\"V[c\xFF\xFF\xFF\xFF\x16\x7F\xFF)\x08H=t\xB6\xB7\0S\xDDG2`\xAC\xF1\xB0\x9E\x0B\xA0x\x1B\xF9A\0\xBB\x82wX\x17I\xDE\x85`@Qa\n\xA3\x91\x90aT\xCFV[`@Q\x80\x91\x03\x90\xA2a\n\xB8` \x85\x01\x85aT\"V[c\xFF\xFF\xFF\xFF\x16\x7F\xDF\"\xF3U\x8EHA\xB6=w\x17\x95F\xB3\xEA\xE6>N4;\xBEu'F\xB0\x93\x16+\xC5&\xBEL\x85`@Qa\n\xED\x91\x90aT\xCFV[`@Q\x80\x91\x03\x90\xA2PPPPPV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0BOW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Bs\x91\x90aT\xDDV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0B\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xA3\x90aT\xFAV[`@Q\x80\x91\x03\x90\xFD[a\x05\xFB\x81a7tV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xFDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C!\x91\x90aUDV[a\x0C=W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xA3\x90aUaV[`fT\x81\x81\x16\x14a\x0C\xB6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\xA3V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[a\x0C\xFCa5lV[`@Q\x81\x15\x15\x81R\x7Fj\xF4\xAE\x1FH\x1A\xFF \xCEW\x1A\xBDe7[g\xB2#Y\x88:\x82=\x1D\xDFK\xD8\xF2\x87\x9F\xF7\xBA\x90` \x01[`@Q\x80\x91\x03\x90\xA1PV[`\x9FT`\x01`\x01`\xA0\x1B\x03\x163\x14a\r_W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xA3\x90aU\xA9V[`\xA0T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15\x15`\x01\x14a\r\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01RnNo task pending`\x88\x1B`D\x82\x01R`d\x01a\x0B\xA3V[a\r\xB7a8kV[V[`\x9D\x80Ta\r\xC6\x90aU\xC8V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\r\xF2\x90aU\xC8V[\x80\x15a\x0E?W\x80`\x1F\x10a\x0E\x14Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0E?V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0E\"W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[``\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0EbWa\x0EbaD\x1BV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E\x8BW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82Q\x81\x10\x15a\x0F\\W\x83`\x01`\x01`\xA0\x1B\x03\x16c\x13T*N\x84\x83\x81Q\x81\x10a\x0E\xBBWa\x0E\xBBaU\xFCV[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\xEE\x91\x90`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x0BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F/\x91\x90aV\x12V[\x82\x82\x81Q\x81\x10a\x0FAWa\x0FAaU\xFCV[` \x90\x81\x02\x91\x90\x91\x01\x01Ra\x0FU\x81aVAV[\x90Pa\x0E\x91V[P\x92\x91PPV[```\0\x84`\x01`\x01`\xA0\x1B\x03\x16ch0H5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xA5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xC9\x91\x90aT\xDDV[\x90P`\0\x85`\x01`\x01`\xA0\x1B\x03\x16c\x9E\x99#\xC2`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\x0BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10/\x91\x90aT\xDDV[\x90P`\0\x86`\x01`\x01`\xA0\x1B\x03\x16c]\xF4YF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10qW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x95\x91\x90aT\xDDV[\x90P`\0\x86Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x10\xB2Wa\x10\xB2aD\x1BV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x10\xE5W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x10\xD0W\x90P[P\x90P`\0[\x87Q\x81\x10\x15a\x13\xEDW`\0\x88\x82\x81Q\x81\x10a\x11\x08Wa\x11\x08aU\xFCV[\x01` \x01Q`@Qc\x89\x02bE`\xE0\x1B\x81R`\xF8\x91\x90\x91\x1C`\x04\x82\x01\x81\x90Rc\xFF\xFF\xFF\xFF\x8A\x16`$\x83\x01R\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x87\x16\x90c\x89\x02bE\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11iW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x11\x91\x91\x90\x81\x01\x90aVZV[\x90P\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x11\xACWa\x11\xACaD\x1BV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x11\xF7W\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a\x11\xCAW\x90P[P\x84\x84\x81Q\x81\x10a\x12\nWa\x12\naU\xFCV[` \x02` \x01\x01\x81\x90RP`\0[\x81Q\x81\x10\x15a\x13\xD7W`@Q\x80``\x01`@R\x80\x87`\x01`\x01`\xA0\x1B\x03\x16cG\xB3\x14\xE8\x85\x85\x81Q\x81\x10a\x12MWa\x12MaU\xFCV[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x12s\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\x90W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xB4\x91\x90aT\xDDV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x83\x81Q\x81\x10a\x12\xD4Wa\x12\xD4aU\xFCV[` \x02` \x01\x01Q\x81R` \x01\x89`\x01`\x01`\xA0\x1B\x03\x16c\xFA(\xC6'\x85\x85\x81Q\x81\x10a\x13\x02Wa\x13\x02aU\xFCV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x04\x81\x01\x91\x90\x91R`\xFF\x88\x16`$\x82\x01Rc\xFF\xFF\xFF\xFF\x8F\x16`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13^W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x82\x91\x90aW\x01V[`\x01`\x01``\x1B\x03\x16\x81RP\x85\x85\x81Q\x81\x10a\x13\xA0Wa\x13\xA0aU\xFCV[` \x02` \x01\x01Q\x82\x81Q\x81\x10a\x13\xB9Wa\x13\xB9aU\xFCV[` \x02` \x01\x01\x81\x90RP\x80\x80a\x13\xCF\x90aVAV[\x91PPa\x12\x18V[PPP\x80\x80a\x13\xE5\x90aVAV[\x91PPa\x10\xEBV[P\x93PPPP[\x93\x92PPPV[a\x14\x03a5lV[`\x9F\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F`\xF5\xACH\xA1?\x8B[\xF4\xB2\x13\xDE\x19\r\xD2\xDE%\x92\xC9\xF6\xF5\xAC}\xC1N=rk\x95\xDE\xD2\xDA\x90` \x01a\r*V[``\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14lWa\x14laD\x1BV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x14\x95W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82Q\x81\x10\x15a\x0F\\W\x83`\x01`\x01`\xA0\x1B\x03\x16c)k\xB0d\x84\x83\x81Q\x81\x10a\x14\xC5Wa\x14\xC5aU\xFCV[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14\xEB\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\x08W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15,\x91\x90aT\xDDV[\x82\x82\x81Q\x81\x10a\x15>Wa\x15>aU\xFCV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01Ra\x15_\x81aVAV[\x90Pa\x14\x9BV[a\x15\x91`@Q\x80`\x80\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`\0\x87`\x01`\x01`\xA0\x1B\x03\x16ch0H5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\xD1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xF5\x91\x90aT\xDDV[\x90Pa\x16\"`@Q\x80`\x80\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`@Qca\xC8\xA1/`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90c\xC3\x91B^\x90a\x16R\x90\x8B\x90\x89\x90\x89\x90`\x04\x01aW\x1CV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16oW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x16\x97\x91\x90\x81\x01\x90aWfV[\x81R`@Qc@\xE0:\x81`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\x81\xC0u\x02\x90a\x16\xC9\x90\x8B\x90\x8B\x90\x8B\x90`\x04\x01aW\xF4V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xE6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x17\x0E\x91\x90\x81\x01\x90aWfV[`@\x82\x01R\x85`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17+Wa\x17+aD\x1BV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x17^W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x17IW\x90P[P``\x82\x01R`\0[`\xFF\x81\x16\x87\x11\x15a\x1B\x9FW`\0\x85`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17\x8CWa\x17\x8CaD\x1BV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x17\xB5W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x83``\x01Q\x83`\xFF\x16\x81Q\x81\x10a\x17\xCFWa\x17\xCFaU\xFCV[` \x02` \x01\x01\x81\x90RP`\0[\x86\x81\x10\x15a\x1A\x9FW`\0\x8C`\x01`\x01`\xA0\x1B\x03\x16c\x04\xECcQ\x8A\x8A\x85\x81\x81\x10a\x18\x08Wa\x18\x08aU\xFCV[\x90P` \x02\x015\x8E\x88`\0\x01Q\x86\x81Q\x81\x10a\x18&Wa\x18&aU\xFCV[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18c\x93\x92\x91\x90\x92\x83Rc\xFF\xFF\xFF\xFF\x91\x82\x16` \x84\x01R\x16`@\x82\x01R``\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\x80W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\xA4\x91\x90aX\x14V[\x90P\x80`\x01`\x01`\xC0\x1B\x03\x16`\0\x03a\x19KW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R\x7FOperatorStateRetriever.getCheckS`D\x82\x01R\x7FignaturesIndices: operator must `d\x82\x01R\x7Fbe registered at blocknumber\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x0B\xA3V[\x8A\x8A\x85`\xFF\x16\x81\x81\x10a\x19`Wa\x19`aU\xFCV[`\x01`\x01`\x01`\xC0\x1B\x03\x85\x16\x91\x90\x93\x015`\xF8\x1C\x1C\x82\x16\x90\x91\x03\x90Pa\x1A\x8CW\x85`\x01`\x01`\xA0\x1B\x03\x16c\xDD\x98F\xB9\x8A\x8A\x85\x81\x81\x10a\x19\xA1Wa\x19\xA1aU\xFCV[\x90P` \x02\x015\x8D\x8D\x88`\xFF\x16\x81\x81\x10a\x19\xBDWa\x19\xBDaU\xFCV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x87\x90\x1B\x16\x81R`\x04\x81\x01\x94\x90\x94R\x91\x90\x91\x015`\xF8\x1C`$\x83\x01RPc\xFF\xFF\xFF\xFF\x8F\x16`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\x13W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A7\x91\x90aX=V[\x85``\x01Q\x85`\xFF\x16\x81Q\x81\x10a\x1APWa\x1APaU\xFCV[` \x02` \x01\x01Q\x84\x81Q\x81\x10a\x1AiWa\x1AiaU\xFCV[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x82a\x1A\x88\x81aVAV[\x93PP[P\x80a\x1A\x97\x81aVAV[\x91PPa\x17\xDDV[P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1A\xBAWa\x1A\xBAaD\x1BV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1A\xE3W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a\x1BdW\x84``\x01Q\x84`\xFF\x16\x81Q\x81\x10a\x1B\nWa\x1B\naU\xFCV[` \x02` \x01\x01Q\x81\x81Q\x81\x10a\x1B#Wa\x1B#aU\xFCV[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a\x1B=Wa\x1B=aU\xFCV[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x1B\\\x81aVAV[\x91PPa\x1A\xE9V[P\x80\x84``\x01Q\x84`\xFF\x16\x81Q\x81\x10a\x1B\x7FWa\x1B\x7FaU\xFCV[` \x02` \x01\x01\x81\x90RPPP\x80\x80a\x1B\x97\x90aXZV[\x91PPa\x17gV[P`\0\x89`\x01`\x01`\xA0\x1B\x03\x16c]\xF4YF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xE0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\x04\x91\x90aT\xDDV[`@Qc5IR\xA3`\xE2\x1B\x81R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xD5%J\x8C\x90a\x1C7\x90\x8B\x90\x8B\x90\x8E\x90`\x04\x01aXyV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1CTW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1C|\x91\x90\x81\x01\x90aWfV[` \x83\x01RP\x98\x97PPPPPPPPV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\xD6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xFA\x91\x90aUDV[a\x1D\x16W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xA3\x90aUaV[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[```\0\x84`\x01`\x01`\xA0\x1B\x03\x16c\xC3\x91B^\x84\x86`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1D\x87\x92\x91\x90aX\xA3V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\xA4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1D\xCC\x91\x90\x81\x01\x90aWfV[\x90P`\0\x84Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\xE9Wa\x1D\xE9aD\x1BV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1E\x12W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x85Q\x81\x10\x15a\x1F\x13W\x86`\x01`\x01`\xA0\x1B\x03\x16c\x04\xECcQ\x87\x83\x81Q\x81\x10a\x1EBWa\x1EBaU\xFCV[` \x02` \x01\x01Q\x87\x86\x85\x81Q\x81\x10a\x1E]Wa\x1E]aU\xFCV[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1E\x9A\x93\x92\x91\x90\x92\x83Rc\xFF\xFF\xFF\xFF\x91\x82\x16` \x84\x01R\x16`@\x82\x01R``\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\xB7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\xDB\x91\x90aX\x14V[`\x01`\x01`\xC0\x1B\x03\x16\x82\x82\x81Q\x81\x10a\x1E\xF6Wa\x1E\xF6aU\xFCV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a\x1F\x0B\x81aVAV[\x91PPa\x1E\x18V[P\x95\x94PPPPPV[`\x97T`@\x80Qc.\xFA,\xA3`\xE1\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c]\xF4YF\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1FgW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\x8B\x91\x90aT\xDDV[\x90P\x90V[a\r_a5lV[`\x97T`@\x80Qch0H5`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91ch0H5\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1FgW=`\0\x80>=`\0\xFD[`\x97T`@\x80Qcm\x14\xA9\x87`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cm\x14\xA9\x87\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1FgW=`\0\x80>=`\0\xFD[`\x9FT`\x01`\x01`\xA0\x1B\x03\x163\x14a VW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xA3\x90aU\xA9V[`\xA0T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a \xA7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RsTask already pending``\x1B`D\x82\x01R`d\x01a\x0B\xA3V[a \xB2\x83\x83\x83a:ZV[PPPV[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`\x97T`@Qc7}\xA3\x1B`\xE1\x1B\x81R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cn\xFBF6\x90a!\x04\x90\x8A\x90\x8A\x90\x8A\x90\x8A\x90\x8A\x90`\x04\x01aYFV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!!W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra!I\x91\x90\x81\x01\x90aZ\xA6V[\x91P\x91P\x95P\x95\x93PPPPV[`\x9FT`\x01`\x01`\xA0\x1B\x03\x163\x14a!\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xA3\x90aU\xA9V[`\xA0T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a!\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RsTask already pending``\x1B`D\x82\x01R`d\x01a\x0B\xA3V[`\x9CT`\x01``\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15\x80\x15\x90a!\xEFWPC\x15\x15[a\"-W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x13\xDC\x08\x14\xDD\x18]\x19H\x1D[\x9A[\x9A]`\x8A\x1B`D\x82\x01R`d\x01a\x0B\xA3V[`\x98T`@\x80Qa\x01\0\x81\x01\x90\x91R`\x01`\xE0\x1B\x90\x91\x04c\xFF\xFF\xFF\xFF\x16\x80\x82R\x90`\0\x90` \x81\x01\x85`\x02\x81\x11\x15a\"gWa\"gaQ\"V[\x81Rc\xFF\xFF\xFF\xFF\x80\x86\x16` \x83\x01RC\x81\x16`@\x83\x01R`\x9CT`\x01`@\x1B\x81\x04\x82\x16``\x84\x01R`\x01``\x1B\x90\x04\x16`\x80\x82\x01R`\x9D\x80T`\xA0\x90\x92\x01\x91a\"\xAF\x90aU\xC8V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\"\xDB\x90aU\xC8V[\x80\x15a#(W\x80`\x1F\x10a\"\xFDWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a#(V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a#\x0BW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x9ETc\xFF\xFF\xFF\xFF\x16` \x91\x82\x01R`@Q\x91\x92Pa#w\x91`\x01\x91\x85\x91a#Z\x91\x86\x91\x01a[VV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\x01a=BV[C`\x9C`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81c\xFF\xFF\xFF\xFF\x16\x7F\x0E\xB7s\xA9\xA1\x92k\t\x9CS\x01vk\xFA@\xAC\x9E>s\x94\xE5i\xEE\x8E\x01\xC6\xA3\x92\xC4K\x17\x98\x82`@Qa#\xCE\x91\x90a[VV[`@Q\x80\x91\x03\x90\xA2a#\xE1\x82`\x01a[\xFAV[`\x98`\x1Ca\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPPV[a$\x0Fa5lV[a\r\xB7`\0a>\x1CV[a$!a5lV[`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x90\x1AeM\xC80\xC9N\x8A\x12\xC9\xA3\xBC\n\x92\xAC\x11\xB5\xCF(\x04l\xA8\xD1\x90i\x1C\xDA\xF5 \x90\x16\x90` \x01a\r*V[a$wa5lV[`@Q\x7FM`\x15Bf\xB2\xEA\x0C\x8F\t\x1D%~\xACZ\xBC\x94\x1CF\xCBT\xD0\xC3\x06\x9A\x83\x0Fc9\xFE\x1D\xA1\x90`\0\x90\xA1V[`fT\x15a$\xF2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FPausable: contract is paused\0\0\0\0`D\x82\x01R`d\x01a\x0B\xA3V[`\x9ETd\x01\0\0\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x163\x14a%<W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04\x17WF\x83`\xDC\x1B`D\x82\x01R`d\x01a\x0B\xA3V[6`\0a%L`\xC0\x86\x01\x86a\\\"V[\x90\x92P\x90P`\0a%da\x01\0\x87\x01`\xE0\x88\x01aT\"V[\x90P`\xA1`\0a%z``\x88\x01`@\x89\x01aP\xB6V[`\x02\x81\x11\x15a%\x8BWa%\x8BaQ\"V[`\x02\x81\x11\x15a%\x9CWa%\x9CaQ\"V[\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 Tc\xFF\xFF\xFF\xFF\x16\x15\x80a&\x1FWP`\xA1`\0a%\xCE``\x88\x01`@\x89\x01aP\xB6V[`\x02\x81\x11\x15a%\xDFWa%\xDFaQ\"V[`\x02\x81\x11\x15a%\xF0Wa%\xF0aQ\"V[\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 Tc\xFF\xFF\xFF\xFF\x16a&\x17`\x80\x87\x01``\x88\x01aT\"V[c\xFF\xFF\xFF\xFF\x16\x14[a&kW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FchainRdBatchNonce mismatch\0\0\0\0\0\0`D\x82\x01R`d\x01a\x0B\xA3V[a&\xB7\x86`@Q` \x01a&\x7F\x91\x90a\\hV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x91\x82\x01 \x90`\x01\x90a&\xA7\x90\x89\x01\x89aT\"V[a\x08Z`\x80\x8B\x01``\x8C\x01aT\"V[`\0\x80a&\xFF\x87`@Q` \x01a&\xCE\x91\x90a]\xC9V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 a&\xF6`\xC0\x8B\x01`\xA0\x8C\x01aT\"V[\x88\x88\x88\x88a>nV[\x90\x92P\x90Pa'\x11` \x89\x01\x89aT\"V[c\xFF\xFF\xFF\xFF\x16\x7F\x82\xE5\xC8\xE9Du\x10\xB8g\xD2H\xC8\x928[\xA3O\xA6\xC2\xD4\xC4\xC2o\xF6\x86\x84\x99\xAE@'\xF2\xC6\x88\x83`@Qa'H\x92\x91\x90a]\xD8V[`@Q\x80\x91\x03\x90\xA2\x81\x15a'\x80Wa'{`\x01a'h` \x8A\x01\x8AaT\"V[\x89\x84`@Q` \x01a\n6\x92\x91\x90a]\xD8V[a'\xCCV[a'\xC2`\x01a'\x92` \x8A\x01\x8AaT\"V[\x89\x84`@Q` \x01a'\xA5\x92\x91\x90a]\xD8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\x03a5\xC6V[PPPPPPPPV[`\0\x80a'\xDF``\x8A\x01`@\x8B\x01aP\xB6V[`\x02\x81\x11\x15a'\xF0Wa'\xF0aQ\"V[\x03a(vW`@\x80Q\x80\x82\x01\x82R`\xA0\x8A\x81\x015\x82R`\xC0\x8B\x015` \x83\x01RT\x91Qb#\xD0\xB5`\xE6\x1B\x81R\x90\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x08\xF4-@\x90a(B\x90`\x80\x8D\x015\x90\x85\x90`\x04\x01a]\xFAV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a(\\W`\0\x80\xFD[PZ\xF1\x15\x80\x15a(pW=`\0\x80>=`\0\xFD[PPPPP[a(\x86`\x80\x89\x01``\x8A\x01aT\"V[a(\x91\x90`\x01a[\xFAV[`\xA1`\0a(\xA5``\x8C\x01`@\x8D\x01aP\xB6V[`\x02\x81\x11\x15a(\xB6Wa(\xB6aQ\"V[`\x02\x81\x11\x15a(\xC7Wa(\xC7aQ\"V[\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x93\x90\x93\x16\x92\x90\x92\x17\x90\x91Ua(\xFD\x90\x8A\x01\x8AaT\"V[`\xA3\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16a\x01\0\x02d\xFF\xFF\xFF\xFF\0\x19\x90\x92\x16\x91\x90\x91\x17\x90Ua).`\x80\x8A\x01``\x8B\x01aT\"V[`\xA3\x80Tp\xFF\xFF\xFF\xFF\0\0\0\0\xFF\xFF\xFF\xFF\0\0\0\0\0\x19\x16e\x01\0\0\0\0\0c\xFF\xFF\xFF\xFF\x93\x84\x16\x02c\xFF\xFF\xFF\xFF`h\x1B\x19\x16\x17`\x01`h\x1BC\x93\x90\x93\x16\x92\x90\x92\x02\x91\x90\x91\x17\x90Ua)\x82` \x89\x01\x89aT\"V[c\xFF\xFF\xFF\xFF\x16\x7F\x17\x97\xCAY\xE0n\xA4\xA0\xEF\xE1\n\xC0\xFBQ\xB5\x8C\x8A\xCF\\\xFE\xDB\xC1_\xAEQ\xC1\0!\xDC\xB9\x06\xE6\x89`@Qa)\xB7\x91\x90a]\xC9V[`@Q\x80\x91\x03\x90\xA2PPPPPPPPPV[`\xA0T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15\x15`\x01\x14a*\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01RnNo task pending`\x88\x1B`D\x82\x01R`d\x01a\x0B\xA3V[`\x99`\0\x84`\x01\x81\x11\x15a*0Wa*0aQ\"V[`\x01\x81\x11\x15a*AWa*AaQ\"V[\x81R` \x01\x90\x81R` \x01`\0 `\0\x83c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x84\x14a*\xABW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\n\x8C.md\r\xAD.m\xAC.\x8Cm`\x9B\x1B`D\x82\x01R`d\x01a\x0B\xA3V[`\x01`\x9B`\0\x85`\x01\x81\x11\x15a*\xC3Wa*\xC3aQ\"V[`\x01\x81\x11\x15a*\xD4Wa*\xD4aQ\"V[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 c\xFF\xFF\xFF\xFF\x87\x16\x82R\x90\x92R\x90 T`\xFF\x16`\x04\x81\x11\x15a+\x0CWa+\x0CaQ\"V[\x14a+JW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01RmNot Init state`\x90\x1B`D\x82\x01R`d\x01a\x0B\xA3V[`\0`\x9A\x81\x85`\x01\x81\x11\x15a+aWa+aaQ\"V[`\x01\x81\x11\x15a+rWa+raQ\"V[\x81R` \x01\x90\x81R` \x01`\0 `\0\x84c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x14a+\xD8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\n`$\x82\x01Ri\x04\x16\xC7&G\x92\x05&W7`\xB4\x1B`D\x82\x01R`d\x01a\x0B\xA3V[`\x98Ta+\xF2\x90`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x82a[\xFAV[c\xFF\xFF\xFF\xFF\x16Cc\xFF\xFF\xFF\xFF\x16\x11\x15a,8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x08`$\x82\x01RgToo late`\xC0\x1B`D\x82\x01R`d\x01a\x0B\xA3V[PPPPV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91``\x91\x83\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x84\x81`\0\x81Q\x81\x10a,yWa,yaU\xFCV[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Qca\xC8\xA1/`\xE1\x1B\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90c\xC3\x91B^\x90a,\xB5\x90\x88\x90\x86\x90`\x04\x01aX\xA3V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,\xD2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra,\xFA\x91\x90\x81\x01\x90aWfV[`\0\x81Q\x81\x10a-\x0CWa-\x0CaU\xFCV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x88\x90Rc\xFF\xFF\xFF\xFF\x87\x81\x16`$\x83\x01R\x90\x91\x16`D\x82\x01\x81\x90R\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x89\x16\x90c\x04\xECcQ\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-xW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-\x9C\x91\x90aX\x14V[`\x01`\x01`\xC0\x1B\x03\x16\x90P`\0a-\xB2\x82a?\xE4V[\x90P\x81a-\xC0\x8A\x83\x8Aa\x0FcV[\x95P\x95PPPPP\x93P\x93\x91PPV[`fT\x15a. W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FPausable: contract is paused\0\0\0\0`D\x82\x01R`d\x01a\x0B\xA3V[`\x9ETd\x01\0\0\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x163\x14a.jW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04\x17WF\x83`\xDC\x1B`D\x82\x01R`d\x01a\x0B\xA3V[`\x9CT`\x01``\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15\x15\x80a.\x89WP`\xA3T`\xFF\x16[a.\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x1D\\\xD9H\x1C\x9B\xDB\xDD\x08\x1A[\x9A]`\x9A\x1B`D\x82\x01R`d\x01a\x0B\xA3V[6`\0a.\xD5`\xC0\x86\x01\x86a\\\"V[\x90\x92P\x90P`\0a.\xEDa\x01\0\x87\x01`\xE0\x88\x01aT\"V[\x90Pa/;\x86`@Q` \x01a/\x03\x91\x90aS1V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x91\x82\x01 \x90`\0\x90a/+\x90\x89\x01\x89aT\"V[a\x08Z`@\x8B\x01` \x8C\x01aT\"V[`\0\x80a/z\x87`@Q` \x01a/R\x91\x90aT\xCFV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 a&\xF6`\x80\x8B\x01``\x8C\x01aT\"V[\x90\x92P\x90Pa/\x8C` \x89\x01\x89aT\"V[c\xFF\xFF\xFF\xFF\x16\x7FG\xAD\xAC\xB0\xB6\xBB\xD7&\xAE9\xACl\0l\xCA\x1C \x06\xC9\xAE\xDA\xA8\x82\xDC\xBA|H\x04\xDB|A\xCE\x88\x83`@Qa/\xC3\x92\x91\x90aT\xAFV[`@Q\x80\x91\x03\x90\xA2\x81\x15a/\xFBWa/\xF6`\0a/\xE3` \x8A\x01\x8AaT\"V[\x89\x84`@Q` \x01a\n6\x92\x91\x90aT\xAFV[a0 V[a'\xC2`\0a0\r` \x8A\x01\x8AaT\"V[\x89\x84`@Q` \x01a'\xA5\x92\x91\x90aT\xAFV[a0.\x87`@\x015\x89a6\x9AV[a0;` \x88\x01\x88aT\"V[c\xFF\xFF\xFF\xFF\x16\x7F\xFF)\x08H=t\xB6\xB7\0S\xDDG2`\xAC\xF1\xB0\x9E\x0B\xA0x\x1B\xF9A\0\xBB\x82wX\x17I\xDE\x88`@Qa0p\x91\x90aT\xCFV[`@Q\x80\x91\x03\x90\xA2PPPPPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a0\xA2WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a0\xBCWP0;\x15\x80\x15a0\xBCWP`\0T`\xFF\x16`\x01\x14[a1\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x0B\xA3V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a1BW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a1M\x8A`\0a@\xB0V[a1V\x89a>\x1CV[`\x9E\x80Td\x01\0\0\0\0`\x01`\xC0\x1B\x03\x19\x16d\x01\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x8B\x81\x16\x91\x90\x91\x02\x91\x90\x91\x17\x90\x91U`\x9F\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x8A\x84\x16\x17\x90\x91U`\xA3\x80T`\xFF\x19\x16\x89\x15\x15\x17\x90U`\x97\x80T\x82\x16\x88\x84\x16\x17\x90U`\x98\x80T\x86\x84\x16`\x01`\x01`\xC0\x1B\x03\x19\x90\x91\x16\x17`\x01`\xA0\x1Bc\xFF\xFF\xFF\xFF\x89\x16\x02\x17\x90U`\xA0\x80T\x90\x91\x16\x91\x84\x16\x91\x90\x91\x17\x90U\x80\x15a2/W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPPPPV[`\x97T`@\x80Qc\xDF\\\xF7#`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xDF\\\xF7#\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1FgW=`\0\x80>=`\0\xFD[a2\x8Da5lV[`\x01`\x01`\xA0\x1B\x03\x81\x16a2\xF2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x0B\xA3V[a\x05\xFB\x81a>\x1CV[a3\x03a5lV[`\xA0T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a3\x1DWa3\x1Da8kV[a3(\x83\x83\x83a:ZV[`@Q\x7FN\xE9\x87\xE5\xF1\xBE\x19\xCA\xBF\xB1\xA2C\xE5\xC4#\x88\x9F\x06\x0F3&gS\x95?\xF0\xCF\x9D\xB8\x99f\xAB\x90`\0\x90\xA1PPPV[a3^a5lV[`\x9E\x80Td\x01\0\0\0\0`\x01`\xC0\x1B\x03\x19\x16d\x01\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x81\x02\x91\x90\x91\x17\x90\x91U`@Q\x90\x81R\x7F`,\xECK\x15\x83\xB0}\x07\x11a\xDA^\xB9X\x94D\xD2E\x92\x01\xE2\xFA\xB7u=\xC9A\xE95\x1C!\x90` \x01a\r*V[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a4\rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a41\x91\x90aT\xDDV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a4aW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xA3\x90aT\xFAV[`fT\x19\x81\x19`fT\x19\x16\x14a4\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\xA3V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x0C\xE9V[a5\x1Ea5lV[`\xA0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F/ \xCF\x1B\xDAgs\x90D\xC5\xBFWsS\x97\x0C=\xBC\x18;,rt\xD1\xE8XJ\x10&\x922g\x90` \x01a\r*V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\r\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x0B\xA3V[\x81`\x9A`\0\x86`\x01\x81\x11\x15a5\xDDWa5\xDDaQ\"V[`\x01\x81\x11\x15a5\xEEWa5\xEEaQ\"V[\x81R` \x01\x90\x81R` \x01`\0 `\0\x85c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP\x80`\x9B`\0\x86`\x01\x81\x11\x15a64Wa64aQ\"V[`\x01\x81\x11\x15a6EWa6EaQ\"V[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 c\xFF\xFF\xFF\xFF\x88\x16\x82R\x90\x92R\x90 \x80T`\xFF\x19\x16`\x01\x83`\x04\x81\x11\x15a6\x82Wa6\x82aQ\"V[\x02\x17\x90UPP`\xA0\x80T`\xFF`\xA0\x1B\x19\x16\x90UPPPV[`\xA2\x82\x90Ua6\xAF`@\x82\x01` \x83\x01aT\"V[`\x9C\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x01``\x1B\x02c\xFF\xFF\xFF\xFF``\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90Ua6\xE1`\x80\x82\x01\x82a\\\"V[a6\xED\x91`\x9D\x91aA\xCBV[Pa6\xFE`\xC0\x82\x01`\xA0\x83\x01aT\"V[`\x9E\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua7%` \x82\x01\x82aT\"V[`\x9C\x80Tk\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x16`\x01`@\x1Bc\xFF\xFF\xFF\xFF\x93\x84\x16\x02\x17\x90U`\xA3\x80Tl\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\x19\x16`\x01`H\x1BC\x90\x93\x16\x92\x90\x92\x02\x91\x90\x91\x17\x90UPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16a8\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x0B\xA3V[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x98T`\x01`\xC0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15a9[W`\x98T`\0\x90a8\x9F\x90`\x01\x90`\x01`\xC0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16a^\x18V[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R\x7F\x10\xAF\xAC\x923\xB4\xCC\xC5Md\x04\xFF\xC1\xCF;GQZ+\x8E\xDB\xF6u\xD1^\xDD\xCE\x05\xA0'\xDC\xBD` R`@\x90 T\x90\x91P`\x01\x90`\xFF\x16`\x04\x81\x11\x15a8\xEEWa8\xEEaQ\"V[\x03a9YWc\xFF\xFF\xFF\xFF\x81\x16`\0\x81\x81R\x7F\x10\xAF\xAC\x923\xB4\xCC\xC5Md\x04\xFF\xC1\xCF;GQZ+\x8E\xDB\xF6u\xD1^\xDD\xCE\x05\xA0'\xDC\xBD` R`@\x80\x82 \x80T`\xFF\x19\x16`\x02\x17\x90UQ\x7F\xD6\xA4\xE0\xFF\x9F:\x057\x08u|z\x12J\xBE\xE3\x1C\xEDa\xF4?\x17\xE6\xE1\xCF\x11\x94>\xC5\x9E`q\x91\x90\xA2[P[`\x98T`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15a:KW`\x98T`\0\x90a9\x8F\x90`\x01\x90`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16a^\x18V[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R\x7F)\x8C\x80\r\x08\x81\xDD \x8Dp^\xBC\x03\xEB\x18\x18\x9F8\x11\x82Y\xF2}\xD4;L`\xD6\x1C`~\x87` R`@\x90 T\x90\x91P`\x01\x90`\xFF\x16`\x04\x81\x11\x15a9\xDEWa9\xDEaQ\"V[\x03a:IWc\xFF\xFF\xFF\xFF\x81\x16`\0\x81\x81R\x7F)\x8C\x80\r\x08\x81\xDD \x8Dp^\xBC\x03\xEB\x18\x18\x9F8\x11\x82Y\xF2}\xD4;L`\xD6\x1C`~\x87` R`@\x80\x82 \x80T`\xFF\x19\x16`\x02\x17\x90UQ\x7F\x0B\xF4k\xFC\xA6\xE2\x13}5\xB8\x93\xC2\x95\xAD\xD8\xC3;\xCF\xBF\xFA\xFD\xEF\x93%,\xB5\x1A\xEDu8\xBA\x0C\x91\x90\xA2[P[`\xA0\x80T`\xFF`\xA0\x1B\x19\x16\x90UV[`\x9CTc\xFF\xFF\xFF\xFF`\x01``\x1B\x90\x91\x04\x16C\x14\x80\x15\x90a:yWPC\x15\x15[a:\xD6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FCan't in lastCompletedOpTaskCrea`D\x82\x01RgtedBlock`\xC0\x1B`d\x82\x01R`\x84\x01a\x0B\xA3V[`\x98T`@\x80Qa\x01\0\x81\x01\x82R`\0\x81\x83\x01\x81\x90R``\x80\x83\x01\x82\x90R`\x80\x83\x01\x81\x90R`\xC0\x83\x01R`\xE0\x82\x01Rc\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x90\x93\x04\x83\x16\x80\x82RC\x84\x16` \x80\x84\x01\x91\x90\x91R\x93\x87\x16`\xA0\x83\x01R\x82Q`\x1F\x86\x01\x85\x90\x04\x85\x02\x81\x01\x85\x01\x90\x93R\x84\x83R\x92\x90\x91\x90\x85\x90\x85\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x82\x90RP`\x80\x86\x01\x94\x90\x94RPP`\x9CT`\x01``\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x90\x91\x03\x90Pa;\xDFWc\xFF\xFF\xFF\xFF\x80\x83\x16`@\x80\x84\x01\x91\x90\x91RC\x90\x91\x16``\x83\x01R\x80Q` `\x1F\x86\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x84\x81R\x90\x85\x90\x85\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPPP`\xC0\x82\x01Rc\xFF\xFF\xFF\xFF\x85\x16`\xE0\x82\x01Ra<\xA2V[`\x9CTc\xFF\xFF\xFF\xFF`\x01`@\x1B\x82\x04\x81\x16`@\x84\x01R`\x01``\x1B\x90\x91\x04\x16``\x82\x01R`\x9D\x80Ta<\x10\x90aU\xC8V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta<<\x90aU\xC8V[\x80\x15a<\x89W\x80`\x1F\x10a<^Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a<\x89V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a<lW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`\xC0\x82\x01R`\x9ETc\xFF\xFF\xFF\xFF\x16`\xE0\x82\x01R[a<\xB9`\0\x83\x83`@Q` \x01a#Z\x91\x90a^=V[`\x9C\x80Tc\xFF\xFF\xFF\xFF\x19\x16Cc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x90\x91\x17\x90\x91U`@Q\x90\x83\x16\x90\x7F\xF3bg'\xFAn5e\xD6\xB23\xE2\x13l.\xE4R\xCF\x96\xE6@OQ\xD8\xCA\xA3G\xDD/{\x91\xF8\x90a=\x08\x90\x84\x90a^=V[`@Q\x80\x91\x03\x90\xA2a=\x1B\x82`\x01a[\xFAV[`\x98`\x18a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPPPV[\x81`\x99`\0\x86`\x01\x81\x11\x15a=YWa=YaQ\"V[`\x01\x81\x11\x15a=jWa=jaQ\"V[\x81R` \x01\x90\x81R` \x01`\0 `\0\x85c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP\x80`\x9B`\0\x86`\x01\x81\x11\x15a=\xB0Wa=\xB0aQ\"V[`\x01\x81\x11\x15a=\xC1Wa=\xC1aQ\"V[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 c\xFF\xFF\xFF\xFF\x88\x16\x82R\x90\x92R\x90 \x80T`\xFF\x19\x16`\x01\x83`\x04\x81\x11\x15a=\xFEWa=\xFEaQ\"V[\x02\x17\x90UPP`\xA0\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x17\x90UPPPV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R``\x82\x84\x01\x81\x90R\x80\x83\x01R`\x97T\x92Qc7}\xA3\x1B`\xE1\x1B\x81R\x90\x92\x83\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cn\xFBF6\x90a>\xCB\x90\x8D\x90\x8B\x90\x8B\x90\x8F\x90\x8F\x90`\x04\x01aYFV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a>\xE8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra?\x10\x91\x90\x81\x01\x90aZ\xA6V[`@\x80Q`\x80\x81\x01\x82Rc\xFF\xFF\xFF\xFFC\x16\x81R` \x80\x82\x01\x84\x90R\x84\x01Q\x91\x81\x01\x91\x90\x91R\x82Q``\x82\x01R\x91\x93P\x91P`\x01`\0[\x88\x81\x10\x15a?\xD3W\x87`\xFF\x16\x85` \x01Q\x82\x81Q\x81\x10a?hWa?haU\xFCV[` \x02` \x01\x01Qa?z\x91\x90a^\xF6V[`\x01`\x01``\x1B\x03\x16`d\x86`\0\x01Q\x83\x81Q\x81\x10a?\x9BWa?\x9BaU\xFCV[` \x02` \x01\x01Q`\x01`\x01``\x1B\x03\x16a?\xB6\x91\x90a_%V[\x10\x15a?\xC1W`\0\x91P[\x80a?\xCB\x81aVAV[\x91PPa?FV[P\x9B\x90\x9AP\x98PPPPPPPPPV[```\0\x80a?\xF2\x84aA\x9AV[a\xFF\xFF\x16`\x01`\x01`@\x1B\x03\x81\x11\x15a@\rWa@\raD\x1BV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a@7W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x80[\x82Q\x82\x10\x80\x15a@OWPa\x01\0\x81\x10[\x15a@\xA6W`\x01\x81\x1B\x93P\x85\x84\x16\x15a@\x96W\x80`\xF8\x1B\x83\x83\x81Q\x81\x10a@xWa@xaU\xFCV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x81`\x01\x01\x91P[a@\x9F\x81aVAV[\x90Pa@>V[P\x90\x94\x93PPPPV[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a@\xD1WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[aASW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x0B\xA3V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2aA\x96\x82a7tV[PPV[`\0\x80[\x82\x15aA\xC5WaA\xAF`\x01\x84a_DV[\x90\x92\x16\x91\x80aA\xBD\x81a_[V[\x91PPaA\x9EV[\x92\x91PPV[\x82\x80TaA\xD7\x90aU\xC8V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82aA\xF9W`\0\x85UaB?V[\x82`\x1F\x10aB\x12W\x82\x80\x01`\xFF\x19\x825\x16\x17\x85UaB?V[\x82\x80\x01`\x01\x01\x85U\x82\x15aB?W\x91\x82\x01[\x82\x81\x11\x15aB?W\x825\x82U\x91` \x01\x91\x90`\x01\x01\x90aB$V[PaBK\x92\x91PaBOV[P\x90V[[\x80\x82\x11\x15aBKW`\0\x81U`\x01\x01aBPV[`\0a\x01\0\x82\x84\x03\x12\x15aBwW`\0\x80\xFD[P\x91\x90PV[`\0``\x82\x84\x03\x12\x15aBwW`\0\x80\xFD[`\0\x80`\x80\x83\x85\x03\x12\x15aB\xA2W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aB\xB8W`\0\x80\xFD[aB\xC4\x85\x82\x86\x01aBdV[\x92PPaB\xD4\x84` \x85\x01aB}V[\x90P\x92P\x92\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05\xFBW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aC\x04W`\0\x80\xFD[\x815a\x13\xF4\x81aB\xDDV[`\0` \x82\x84\x03\x12\x15aC!W`\0\x80\xFD[P5\x91\x90PV[\x805`\x02\x81\x10aC7W`\0\x80\xFD[\x91\x90PV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x05\xFBW`\0\x80\xFD[\x805aC7\x81aC<V[`\0\x80`@\x83\x85\x03\x12\x15aClW`\0\x80\xFD[aCu\x83aC(V[\x91P` \x83\x015aC\x85\x81aC<V[\x80\x91PP\x92P\x92\x90PV[\x80\x15\x15\x81\x14a\x05\xFBW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aC\xB0W`\0\x80\xFD[\x815a\x13\xF4\x81aC\x90V[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15aC\xE1W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01aC\xC5V[\x81\x81\x11\x15aC\xF3W`\0` \x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x13\xF4` \x83\x01\x84aC\xBBV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aDSWaDSaD\x1BV[`@R\x90V[`@Qa\x01\0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aDSWaDSaD\x1BV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aD\xA4WaD\xA4aD\x1BV[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15aD\xC5WaD\xC5aD\x1BV[P`\x05\x1B` \x01\x90V[`\0\x80`@\x83\x85\x03\x12\x15aD\xE2W`\0\x80\xFD[\x825aD\xED\x81aB\xDDV[\x91P` \x83\x81\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aE\tW`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13aE\x1AW`\0\x80\xFD[\x805aE-aE(\x82aD\xACV[aD|V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x88\x83\x11\x15aELW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aEsW\x835aEd\x81aB\xDDV[\x82R\x92\x84\x01\x92\x90\x84\x01\x90aEQV[\x80\x95PPPPPP\x92P\x92\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aE\xB2W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aE\x96V[P\x94\x95\x94PPPPPV[` \x81R`\0a\x13\xF4` \x83\x01\x84aE\x82V[`\0\x80`\0``\x84\x86\x03\x12\x15aE\xE5W`\0\x80\xFD[\x835aE\xF0\x81aB\xDDV[\x92P` \x84\x81\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aF\rW`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12aF!W`\0\x80\xFD[\x815\x81\x81\x11\x15aF3WaF3aD\x1BV[aFE`\x1F\x82\x01`\x1F\x19\x16\x85\x01aD|V[\x91P\x80\x82R\x88\x84\x82\x85\x01\x01\x11\x15aF[W`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x94PPPPaF~`@\x85\x01aCNV[\x90P\x92P\x92P\x92V[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0\x80[\x86\x81\x10\x15aG\x1DW\x83\x85\x03\x8AR\x82Q\x80Q\x80\x87R\x90\x87\x01\x90\x87\x87\x01\x90\x84[\x81\x81\x10\x15aG\x08W\x83Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x8A\x81\x01Q\x8B\x85\x01R`@\x90\x81\x01Q`\x01`\x01``\x1B\x03\x16\x90\x84\x01R\x92\x89\x01\x92``\x90\x92\x01\x91`\x01\x01aF\xC4V[PP\x9A\x87\x01\x9A\x95PP\x91\x85\x01\x91`\x01\x01aF\xA6V[P\x92\x98\x97PPPPPPPPV[` \x81R`\0a\x13\xF4` \x83\x01\x84aF\x87V[`\0` \x82\x84\x03\x12\x15aGPW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aGfW`\0\x80\xFD[\x82\x01`@\x81\x85\x03\x12\x15a\x13\xF4W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12aG\x89W`\0\x80\xFD[\x815` aG\x99aE(\x83aD\xACV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aG\xB8W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aG\xD3W\x805\x83R\x91\x83\x01\x91\x83\x01aG\xBCV[P\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15aG\xF1W`\0\x80\xFD[\x825aG\xFC\x81aB\xDDV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aH\x17W`\0\x80\xFD[aH#\x85\x82\x86\x01aGxV[\x91PP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aHnW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aHIV[P\x90\x96\x95PPPPPPV[`\0\x80\x83`\x1F\x84\x01\x12aH\x8CW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aH\xA3W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aH\xBBW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\x80\x87\x89\x03\x12\x15aH\xDBW`\0\x80\xFD[\x865aH\xE6\x81aB\xDDV[\x95P` \x87\x015aH\xF6\x81aC<V[\x94P`@\x87\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aI\x12W`\0\x80\xFD[aI\x1E\x8A\x83\x8B\x01aHzV[\x90\x96P\x94P``\x89\x015\x91P\x80\x82\x11\x15aI7W`\0\x80\xFD[\x81\x89\x01\x91P\x89`\x1F\x83\x01\x12aIKW`\0\x80\xFD[\x815\x81\x81\x11\x15aIZW`\0\x80\xFD[\x8A` \x82`\x05\x1B\x85\x01\x01\x11\x15aIoW`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92\x95P\x92\x95P\x92\x95V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aE\xB2W\x81Qc\xFF\xFF\xFF\xFF\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aI\x99V[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0[\x85\x81\x10\x15aJ\x03W\x82\x84\x03\x89RaI\xF1\x84\x83QaI\x85V[\x98\x85\x01\x98\x93P\x90\x84\x01\x90`\x01\x01aI\xD9V[P\x91\x97\x96PPPPPPPV[` \x81R`\0\x82Q`\x80` \x84\x01RaJ,`\xA0\x84\x01\x82aI\x85V[\x90P` \x84\x01Q`\x1F\x19\x80\x85\x84\x03\x01`@\x86\x01RaJJ\x83\x83aI\x85V[\x92P`@\x86\x01Q\x91P\x80\x85\x84\x03\x01``\x86\x01RaJg\x83\x83aI\x85V[\x92P``\x86\x01Q\x91P\x80\x85\x84\x03\x01`\x80\x86\x01RPaJ\x85\x82\x82aI\xBBV[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15aJ\xA0W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aJ\xB6W`\0\x80\xFD[\x82\x01a\x01 \x81\x85\x03\x12\x15a\x13\xF4W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aJ\xDBW`\0\x80\xFD[\x815`\xFF\x81\x16\x81\x14a\x13\xF4W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15aK\x01W`\0\x80\xFD[\x835aK\x0C\x81aB\xDDV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aK'W`\0\x80\xFD[aK3\x86\x82\x87\x01aGxV[\x92PP`@\x84\x015aKD\x81aC<V[\x80\x91PP\x92P\x92P\x92V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aHnW\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aKkV[`\0\x80`\0`@\x84\x86\x03\x12\x15aK\x9CW`\0\x80\xFD[\x835aK\xA7\x81aC<V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aK\xC2W`\0\x80\xFD[aK\xCE\x86\x82\x87\x01aHzV[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x82`\x1F\x83\x01\x12aK\xECW`\0\x80\xFD[\x815` aK\xFCaE(\x83aD\xACV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aL\x1BW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aG\xD3W\x805aL2\x81aC<V[\x83R\x91\x83\x01\x91\x83\x01aL\x1FV[`\0`@\x82\x84\x03\x12\x15aLQW`\0\x80\xFD[aLYaD1V[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12aL\x80W`\0\x80\xFD[\x815` aL\x90aE(\x83aD\xACV[\x82\x81R`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aL\xAFW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aG\xD3WaL\xC5\x88\x82aL?V[\x83R\x91\x83\x01\x91`@\x01aL\xB3V[`\0\x82`\x1F\x83\x01\x12aL\xE4W`\0\x80\xFD[aL\xECaD1V[\x80`@\x84\x01\x85\x81\x11\x15aL\xFEW`\0\x80\xFD[\x84[\x81\x81\x10\x15aM\x18W\x805\x84R` \x93\x84\x01\x93\x01aM\0V[P\x90\x95\x94PPPPPV[`\0`\x80\x82\x84\x03\x12\x15aM5W`\0\x80\xFD[aM=aD1V[\x90PaMI\x83\x83aL\xD3V[\x81RaMX\x83`@\x84\x01aL\xD3V[` \x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12aMtW`\0\x80\xFD[\x815` aM\x84aE(\x83aD\xACV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aM\xA3W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aG\xD3W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aM\xC6W`\0\x80\x81\xFD[aM\xD4\x89\x86\x83\x8B\x01\x01aK\xDBV[\x84RP\x91\x83\x01\x91\x83\x01aM\xA7V[`\0a\x01\x80\x82\x84\x03\x12\x15aM\xF5W`\0\x80\xFD[aM\xFDaDYV[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aN\x16W`\0\x80\xFD[aN\"\x85\x83\x86\x01aK\xDBV[\x83R` \x84\x015\x91P\x80\x82\x11\x15aN8W`\0\x80\xFD[aND\x85\x83\x86\x01aLoV[` \x84\x01R`@\x84\x015\x91P\x80\x82\x11\x15aN]W`\0\x80\xFD[aNi\x85\x83\x86\x01aLoV[`@\x84\x01RaN{\x85``\x86\x01aM#V[``\x84\x01RaN\x8D\x85`\xE0\x86\x01aL?V[`\x80\x84\x01Ra\x01 \x84\x015\x91P\x80\x82\x11\x15aN\xA7W`\0\x80\xFD[aN\xB3\x85\x83\x86\x01aK\xDBV[`\xA0\x84\x01Ra\x01@\x84\x015\x91P\x80\x82\x11\x15aN\xCDW`\0\x80\xFD[aN\xD9\x85\x83\x86\x01aK\xDBV[`\xC0\x84\x01Ra\x01`\x84\x015\x91P\x80\x82\x11\x15aN\xF3W`\0\x80\xFD[PaO\0\x84\x82\x85\x01aMcV[`\xE0\x83\x01RP\x92\x91PPV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15aO$W`\0\x80\xFD[\x855\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aOBW`\0\x80\xFD[aON\x89\x83\x8A\x01aHzV[\x90\x96P\x94P`@\x88\x015\x91PaOc\x82aC<V[\x90\x92P``\x87\x015\x90\x80\x82\x11\x15aOyW`\0\x80\xFD[PaO\x86\x88\x82\x89\x01aM\xE2V[\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aE\xB2W\x81Q`\x01`\x01``\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aO\xA7V[`@\x81R`\0\x83Q`@\x80\x84\x01RaO\xE7`\x80\x84\x01\x82aO\x93V[\x90P` \x85\x01Q`?\x19\x84\x83\x03\x01``\x85\x01RaP\x04\x82\x82aO\x93V[\x92PPP\x82` \x83\x01R\x93\x92PPPV[\x805`\x03\x81\x10aC7W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15aP7W`\0\x80\xFD[aCu\x83aP\x15V[`\0\x80`\0a\x01@\x84\x86\x03\x12\x15aPVW`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aPmW`\0\x80\xFD[aPy\x87\x83\x88\x01aBdV[\x94PaP\x88\x87` \x88\x01aBdV[\x93Pa\x01 \x86\x015\x91P\x80\x82\x11\x15aP\x9FW`\0\x80\xFD[PaP\xAC\x86\x82\x87\x01aM\xE2V[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15aP\xC8W`\0\x80\xFD[a\x13\xF4\x82aP\x15V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aP\xE7W`\0\x80\xFD[\x845\x93PaP\xF7` \x86\x01aC(V[\x92P`@\x85\x015aQ\x07\x81aC<V[\x91P``\x85\x015aQ\x17\x81aC<V[\x93\x96\x92\x95P\x90\x93PPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x05\x83\x10aQLWaQLaQ\"V[\x91\x90R\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15aQgW`\0\x80\xFD[\x835aQr\x81aB\xDDV[\x92P` \x84\x015\x91P`@\x84\x015aKD\x81aC<V[\x82\x81R`@` \x82\x01R`\0aQ\xA2`@\x83\x01\x84aF\x87V[\x94\x93PPPPV[`\0\x80`\0`\xA0\x84\x86\x03\x12\x15aQ\xBFW`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aQ\xD6W`\0\x80\xFD[aQ\xE2\x87\x83\x88\x01aBdV[\x94PaQ\xF1\x87` \x88\x01aB}V[\x93P`\x80\x86\x015\x91P\x80\x82\x11\x15aP\x9FW`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0\x80`\0a\x01 \x8A\x8C\x03\x12\x15aR&W`\0\x80\xFD[\x895aR1\x81aB\xDDV[\x98P` \x8A\x015aRA\x81aB\xDDV[\x97P`@\x8A\x015aRQ\x81aB\xDDV[\x96P``\x8A\x015aRa\x81aB\xDDV[\x95P`\x80\x8A\x015aRq\x81aC\x90V[\x94P`\xA0\x8A\x015aR\x81\x81aB\xDDV[\x93P`\xC0\x8A\x015aR\x91\x81aC<V[\x92P`\xE0\x8A\x015aR\xA1\x81aB\xDDV[\x91Pa\x01\0\x8A\x015aR\xB2\x81aB\xDDV[\x80\x91PP\x92\x95\x98P\x92\x95\x98P\x92\x95\x98V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aR\xDAW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aR\xF9W`\0\x80\xFD[\x806\x03\x83\x13\x15aH\xBBW`\0\x80\xFD[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x81R`\0\x825aSB\x81aC<V[c\xFF\xFF\xFF\xFF\x81\x16` \x84\x01RPaS[` \x84\x01aCNV[c\xFF\xFF\xFF\xFF\x81\x16`@\x84\x01RPaSt`@\x84\x01aCNV[c\xFF\xFF\xFF\xFF\x81\x16``\x84\x01RPaS\x8D``\x84\x01aCNV[c\xFF\xFF\xFF\xFF\x81\x16`\x80\x84\x01RPaS\xA7`\x80\x84\x01\x84aR\xC3V[a\x01\0\x80`\xA0\x86\x01RaS\xBFa\x01 \x86\x01\x83\x85aS\x08V[\x92PaS\xCD`\xA0\x87\x01aCNV[c\xFF\xFF\xFF\xFF\x81\x16`\xC0\x87\x01R\x91PaS\xE8`\xC0\x87\x01\x87aR\xC3V[\x86\x85\x03`\x1F\x19\x01`\xE0\x88\x01R\x92PaT\x01\x84\x84\x83aS\x08V[\x93PPaT\x10`\xE0\x87\x01aCNV[c\xFF\xFF\xFF\xFF\x81\x16\x86\x83\x01R\x91Pa@\xA6V[`\0` \x82\x84\x03\x12\x15aT4W`\0\x80\xFD[\x815a\x13\xF4\x81aC<V[\x805aTJ\x81aC<V[c\xFF\xFF\xFF\xFF\x16\x82R` \x81\x81\x015\x90\x83\x01R`@\x90\x81\x015\x91\x01RV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Q` \x83\x01R`\0`@\x82\x01Q`\x80`@\x85\x01RaT\x96`\x80\x85\x01\x82aO\x93V[\x90P``\x83\x01Q\x84\x82\x03``\x86\x01RaJ\x85\x82\x82aO\x93V[aT\xB9\x81\x84aT?V[`\x80``\x82\x01R`\0aQ\xA2`\x80\x83\x01\x84aTgV[``\x81\x01aA\xC5\x82\x84aT?V[`\0` \x82\x84\x03\x12\x15aT\xEFW`\0\x80\xFD[\x81Qa\x13\xF4\x81aB\xDDV[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15aUVW`\0\x80\xFD[\x81Qa\x13\xF4\x81aC\x90V[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`\x05\x90\x82\x01RdAuth1`\xD8\x1B`@\x82\x01R``\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80aU\xDCW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aBwWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15aV$W`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01aVSWaVSaV+V[P`\x01\x01\x90V[`\0` \x80\x83\x85\x03\x12\x15aVmW`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aV\x83W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aV\x94W`\0\x80\xFD[\x80QaV\xA2aE(\x82aD\xACV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15aV\xC1W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aV\xDFW\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90aV\xC6V[\x97\x96PPPPPPPV[\x80Q`\x01`\x01``\x1B\x03\x81\x16\x81\x14aC7W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aW\x13W`\0\x80\xFD[a\x13\xF4\x82aV\xEAV[c\xFF\xFF\xFF\xFF\x84\x16\x81R`@` \x82\x01\x81\x90R\x81\x01\x82\x90R`\0`\x01`\x01`\xFB\x1B\x03\x83\x11\x15aWIW`\0\x80\xFD[\x82`\x05\x1B\x80\x85``\x85\x017`\0\x92\x01``\x01\x91\x82RP\x93\x92PPPV[`\0` \x80\x83\x85\x03\x12\x15aWyW`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aW\x8FW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aW\xA0W`\0\x80\xFD[\x80QaW\xAEaE(\x82aD\xACV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15aW\xCDW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aV\xDFW\x83QaW\xE5\x81aC<V[\x82R\x92\x84\x01\x92\x90\x84\x01\x90aW\xD2V[c\xFF\xFF\xFF\xFF\x84\x16\x81R`@` \x82\x01R`\0aJ\x85`@\x83\x01\x84\x86aS\x08V[`\0` \x82\x84\x03\x12\x15aX&W`\0\x80\xFD[\x81Q`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x14a\x13\xF4W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aXOW`\0\x80\xFD[\x81Qa\x13\xF4\x81aC<V[`\0`\xFF\x82\x16`\xFF\x81\x03aXpWaXpaV+V[`\x01\x01\x92\x91PPV[`@\x81R`\0aX\x8D`@\x83\x01\x85\x87aS\x08V[\x90Pc\xFF\xFF\xFF\xFF\x83\x16` \x83\x01R\x94\x93PPPPV[c\xFF\xFF\xFF\xFF\x83\x16\x81R`@` \x82\x01R`\0aQ\xA2`@\x83\x01\x84aE\x82V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aE\xB2WaX\xF3\x87\x83Q\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[`@\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01aX\xD6V[\x80`\0[`\x02\x81\x10\x15a,8W\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01aY\nV[aY4\x82\x82QaY\x06V[` \x81\x01Qa \xB2`@\x84\x01\x82aY\x06V[\x85\x81R`\x80` \x82\x01R`\0aY``\x80\x83\x01\x86\x88aS\x08V[c\xFF\xFF\xFF\xFF\x85\x16`@\x84\x01R\x82\x81\x03``\x84\x01Ra\x01\x80\x84Q\x81\x83RaY\x88\x82\x84\x01\x82aI\x85V[\x91PP` \x85\x01Q\x82\x82\x03` \x84\x01RaY\xA2\x82\x82aX\xC2V[\x91PP`@\x85\x01Q\x82\x82\x03`@\x84\x01RaY\xBC\x82\x82aX\xC2V[\x91PP``\x85\x01QaY\xD1``\x84\x01\x82aY)V[P`\x80\x85\x01Q\x80Q`\xE0\x84\x01R` \x01Qa\x01\0\x83\x01R`\xA0\x85\x01Q\x82\x82\x03a\x01 \x84\x01RaZ\0\x82\x82aI\x85V[\x91PP`\xC0\x85\x01Q\x82\x82\x03a\x01@\x84\x01RaZ\x1B\x82\x82aI\x85V[\x91PP`\xE0\x85\x01Q\x82\x82\x03a\x01`\x84\x01RaZ6\x82\x82aI\xBBV[\x9A\x99PPPPPPPPPPV[`\0\x82`\x1F\x83\x01\x12aZUW`\0\x80\xFD[\x81Q` aZeaE(\x83aD\xACV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aZ\x84W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aG\xD3WaZ\x99\x81aV\xEAV[\x83R\x91\x83\x01\x91\x83\x01aZ\x88V[`\0\x80`@\x83\x85\x03\x12\x15aZ\xB9W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aZ\xD0W`\0\x80\xFD[\x90\x84\x01\x90`@\x82\x87\x03\x12\x15aZ\xE4W`\0\x80\xFD[aZ\xECaD1V[\x82Q\x82\x81\x11\x15aZ\xFBW`\0\x80\xFD[a[\x07\x88\x82\x86\x01aZDV[\x82RP` \x83\x01Q\x82\x81\x11\x15a[\x1CW`\0\x80\xFD[a[(\x88\x82\x86\x01aZDV[` \x83\x01RP\x80\x94PPPP` \x83\x01Q\x90P\x92P\x92\x90PV[`\x03\x81\x10a[RWa[RaQ\"V[\x90RV[` \x81R`\0c\xFF\xFF\xFF\xFF\x80\x84Q\x16` \x84\x01R` \x84\x01Qa[|`@\x85\x01\x82a[BV[P\x80`@\x85\x01Q\x16``\x84\x01RP``\x83\x01Qa[\xA1`\x80\x84\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P`\x80\x83\x01Qc\xFF\xFF\xFF\xFF\x81\x16`\xA0\x84\x01RP`\xA0\x83\x01Qc\xFF\xFF\xFF\xFF\x81\x16`\xC0\x84\x01RP`\xC0\x83\x01Qa\x01\0\x80`\xE0\x85\x01Ra[\xE2a\x01 \x85\x01\x83aC\xBBV[\x91P`\xE0\x85\x01Qa@\xA6\x82\x86\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a\\\x19Wa\\\x19aV+V[\x01\x94\x93PPPPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\\9W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\\SW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15aH\xBBW`\0\x80\xFD[` \x81R`\0\x825a\\y\x81aC<V[c\xFF\xFF\xFF\xFF\x81\x16` \x84\x01RPa\\\x92` \x84\x01aP\x15V[a\\\x9F`@\x84\x01\x82a[BV[Pa\\\xAC`@\x84\x01aCNV[c\xFF\xFF\xFF\xFF\x81\x16``\x84\x01RPa\\\xC5``\x84\x01aCNV[c\xFF\xFF\xFF\xFF\x81\x16`\x80\x84\x01RPa\\\xDE`\x80\x84\x01aCNV[c\xFF\xFF\xFF\xFF\x81\x16`\xA0\x84\x01RPa\\\xF7`\xA0\x84\x01aCNV[c\xFF\xFF\xFF\xFF\x81\x16`\xC0\x84\x01RPa]\x11`\xC0\x84\x01\x84aR\xC3V[a\x01\0\x80`\xE0\x86\x01Ra])a\x01 \x86\x01\x83\x85aS\x08V[\x92PaT\x10`\xE0\x87\x01aCNV[\x805a]B\x81aC<V[c\xFF\xFF\xFF\xFF\x80\x82\x16\x84R` \x83\x015` \x85\x01Ra]b`@\x84\x01aP\x15V[\x91Pa]q`@\x85\x01\x83a[BV[``\x83\x015\x91Pa]\x81\x82aC<V[\x16``\x83\x01R`\x80\x81\x81\x015\x90\x83\x01R`\xA0\x80\x82\x015\x90\x83\x01R`\xC0\x80\x82\x015\x90\x83\x01R`\xE0\x81\x015a]\xB3\x81aB\xDDV[`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x92\x90\x92\x01\x91\x90\x91RPV[a\x01\0\x81\x01aA\xC5\x82\x84a]7V[`\0a\x01 a]\xE7\x83\x86a]7V[\x80a\x01\0\x84\x01RaJ\x85\x81\x84\x01\x85aTgV[\x82\x81R``\x81\x01a\x13\xF4` \x83\x01\x84\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[`\0c\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a^5Wa^5aV+V[\x03\x93\x92PPPV[` \x81Rc\xFF\xFF\xFF\xFF\x82Q\x16` \x82\x01R`\0` \x83\x01Qa^g`@\x84\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P`@\x83\x01Qc\xFF\xFF\xFF\xFF\x81\x16``\x84\x01RP``\x83\x01Qc\xFF\xFF\xFF\xFF\x81\x16`\x80\x84\x01RP`\x80\x83\x01Qa\x01\0\x80`\xA0\x85\x01Ra^\xA8a\x01 \x85\x01\x83aC\xBBV[\x91P`\xA0\x85\x01Qa^\xC1`\xC0\x86\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P`\xC0\x85\x01Q\x84\x83\x03`\x1F\x19\x01`\xE0\x86\x01Ra^\xDD\x83\x82aC\xBBV[\x92PP`\xE0\x85\x01Qa@\xA6\x82\x86\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15a_\x1CWa_\x1CaV+V[\x02\x94\x93PPPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a_?Wa_?aV+V[P\x02\x90V[`\0\x82\x82\x10\x15a_VWa_VaV+V[P\x03\x90V[`\0a\xFF\xFF\x80\x83\x16\x81\x81\x03a_rWa_raV+V[`\x01\x01\x93\x92PPPV\xFE\xA2dipfsX\"\x12 `\xC8\xA9\x045\xE2,\x1E1\x0C-\xF1q\x96\x89G\x96\xA8\x7F\xF5\x81W\xEDUV|\x1B\x86:\x9A\x86\x15dsolcC\0\x08\r\x003";
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
        ///Calls the contract's `chainRdBatchNonce` (0x930390d9) function
        pub fn chain_rd_batch_nonce(
            &self,
            p0: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([147, 3, 144, 217], p0)
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
        ///Calls the contract's `createNewRdTask` (0x6f254819) function
        pub fn create_new_rd_task(
            &self,
            chain_id: u8,
            batch_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([111, 37, 72, 25], (chain_id, batch_id))
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
        ///Calls the contract's `pauseTrackingOpState` (0x79badf73) function
        pub fn pause_tracking_op_state(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([121, 186, 223, 115], ())
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
        ///Calls the contract's `respondToRdTask` (0x92ddb19c) function
        pub fn respond_to_rd_task(
            &self,
            task: RdTask,
            task_response: RdTaskResponse,
            non_signer_stakes_and_signature: NonSignerStakesAndSignature,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [146, 221, 177, 156],
                    (task, task_response, non_signer_stakes_and_signature),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `resumeTrackingQuorums` (0x191aac7a) function
        pub fn resume_tracking_quorums(
            &self,
            reset_tracked_quorums: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([25, 26, 172, 122], reset_tracked_quorums)
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
        ///Gets the contract's `PauseTrackingOpState` event
        pub fn pause_tracking_op_state_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PauseTrackingOpStateFilter>
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
        ///Gets the contract's `ResumeTrackingOpState` event
        pub fn resume_tracking_op_state_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ResumeTrackingOpStateFilter>
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
        abi = "NewRdTaskCreated(uint32,(uint32,uint8,uint32,uint32,uint32,uint32,bytes,uint32))"
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
    #[ethevent(name = "PauseTrackingOpState", abi = "PauseTrackingOpState()")]
    pub struct PauseTrackingOpStateFilter;
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
        abi = "RdTaskCompleted(uint32,(uint32,bytes32,uint8,uint32,bytes32,uint256,uint256,address))"
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
        abi = "RdTaskResponded(uint32,(uint32,bytes32,uint8,uint32,bytes32,uint256,uint256,address),(uint32,bytes32,uint96[],uint96[]))"
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
    #[ethevent(name = "ResumeTrackingOpState", abi = "ResumeTrackingOpState(bool)")]
    pub struct ResumeTrackingOpStateFilter {
        pub reset_tracked_quorums: bool,
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
        PauseTrackingOpStateFilter(PauseTrackingOpStateFilter),
        PausedFilter(PausedFilter),
        PauserRegistrySetFilter(PauserRegistrySetFilter),
        RdTaskCancelledFilter(RdTaskCancelledFilter),
        RdTaskCompletedFilter(RdTaskCompletedFilter),
        RdTaskRespondedFilter(RdTaskRespondedFilter),
        ResumeTrackingOpStateFilter(ResumeTrackingOpStateFilter),
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
            if let Ok(decoded) = PauseTrackingOpStateFilter::decode_log(log) {
                return Ok(FinalizerTaskManagerEvents::PauseTrackingOpStateFilter(
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
            if let Ok(decoded) = ResumeTrackingOpStateFilter::decode_log(log) {
                return Ok(FinalizerTaskManagerEvents::ResumeTrackingOpStateFilter(
                    decoded,
                ));
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
                Self::PauseTrackingOpStateFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PausedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauserRegistrySetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RdTaskCancelledFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RdTaskCompletedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RdTaskRespondedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ResumeTrackingOpStateFilter(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<PauseTrackingOpStateFilter> for FinalizerTaskManagerEvents {
        fn from(value: PauseTrackingOpStateFilter) -> Self {
            Self::PauseTrackingOpStateFilter(value)
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
    impl ::core::convert::From<ResumeTrackingOpStateFilter> for FinalizerTaskManagerEvents {
        fn from(value: ResumeTrackingOpStateFilter) -> Self {
            Self::ResumeTrackingOpStateFilter(value)
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
    ///Container type for all input parameters for the `chainRdBatchNonce` function with signature `chainRdBatchNonce(uint8)` and selector `0x930390d9`
    #[derive(
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
    #[ethcall(name = "chainRdBatchNonce", abi = "chainRdBatchNonce(uint8)")]
    pub struct ChainRdBatchNonceCall(pub u8);
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
    ///Container type for all input parameters for the `createNewRdTask` function with signature `createNewRdTask(uint8,uint32)` and selector `0x6f254819`
    #[derive(
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
    #[ethcall(name = "createNewRdTask", abi = "createNewRdTask(uint8,uint32)")]
    pub struct CreateNewRdTaskCall {
        pub chain_id: u8,
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
    ///Container type for all input parameters for the `pauseTrackingOpState` function with signature `pauseTrackingOpState()` and selector `0x79badf73`
    #[derive(
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
    #[ethcall(name = "pauseTrackingOpState", abi = "pauseTrackingOpState()")]
    pub struct PauseTrackingOpStateCall;
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
    ///Container type for all input parameters for the `respondToRdTask` function with signature `respondToRdTask((uint32,uint8,uint32,uint32,uint32,uint32,bytes,uint32),(uint32,bytes32,uint8,uint32,bytes32,uint256,uint256,address),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))` and selector `0x92ddb19c`
    #[derive(
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
        abi = "respondToRdTask((uint32,uint8,uint32,uint32,uint32,uint32,bytes,uint32),(uint32,bytes32,uint8,uint32,bytes32,uint256,uint256,address),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))"
    )]
    pub struct RespondToRdTaskCall {
        pub task: RdTask,
        pub task_response: RdTaskResponse,
        pub non_signer_stakes_and_signature: NonSignerStakesAndSignature,
    }
    ///Container type for all input parameters for the `resumeTrackingQuorums` function with signature `resumeTrackingQuorums(bool)` and selector `0x191aac7a`
    #[derive(
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
    #[ethcall(name = "resumeTrackingQuorums", abi = "resumeTrackingQuorums(bool)")]
    pub struct ResumeTrackingQuorumsCall {
        pub reset_tracked_quorums: bool,
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
        PauseTrackingOpState(PauseTrackingOpStateCall),
        PausedWithIndex(PausedWithIndexCall),
        Paused(PausedCall),
        PauserRegistry(PauserRegistryCall),
        RegistryCoordinator(RegistryCoordinatorCall),
        RenounceOwnership(RenounceOwnershipCall),
        RespondToOpTask(RespondToOpTaskCall),
        RespondToRdTask(RespondToRdTaskCall),
        ResumeTrackingQuorums(ResumeTrackingQuorumsCall),
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
                <PauseTrackingOpStateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PauseTrackingOpState(decoded));
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
            if let Ok(decoded) =
                <ResumeTrackingQuorumsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ResumeTrackingQuorums(decoded));
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
                Self::PauseTrackingOpState(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PausedWithIndex(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Paused(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PauserRegistry(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RegistryCoordinator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RespondToOpTask(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RespondToRdTask(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ResumeTrackingQuorums(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
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
                Self::PauseTrackingOpState(element) => ::core::fmt::Display::fmt(element, f),
                Self::PausedWithIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::Paused(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauserRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegistryCoordinator(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::RespondToOpTask(element) => ::core::fmt::Display::fmt(element, f),
                Self::RespondToRdTask(element) => ::core::fmt::Display::fmt(element, f),
                Self::ResumeTrackingQuorums(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<PauseTrackingOpStateCall> for FinalizerTaskManagerCalls {
        fn from(value: PauseTrackingOpStateCall) -> Self {
            Self::PauseTrackingOpState(value)
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
    impl ::core::convert::From<ResumeTrackingQuorumsCall> for FinalizerTaskManagerCalls {
        fn from(value: ResumeTrackingQuorumsCall) -> Self {
            Self::ResumeTrackingQuorums(value)
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
    ///Container type for all return fields from the `chainRdBatchNonce` function with signature `chainRdBatchNonce(uint8)` and selector `0x930390d9`
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
