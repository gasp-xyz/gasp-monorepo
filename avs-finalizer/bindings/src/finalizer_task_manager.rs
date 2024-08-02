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
                                            "enum FinalizerTaskManager.TaskType",
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
                                            "enum FinalizerTaskManager.TaskType",
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
                                    name: ::std::borrow::ToOwned::to_owned("blockNumber"),
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
                                            "enum FinalizerTaskManager.TaskType",
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
                                            "enum FinalizerTaskManager.TaskStatus",
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
                                        "_minOpTaskResponseWindowBlock",
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
                    ::std::borrow::ToOwned::to_owned("minOpTaskResponseWindowBlock"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "minOpTaskResponseWindowBlock",
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
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
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
                                    name: ::std::borrow::ToOwned::to_owned("blockHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
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
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa]!\x80b\0\0!`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\xA0W`\x005`\xE0\x1C\x80c]\xF4YF\x11a\x01gW\x80c\xA6\x95c\xA9\x11a\0\xCEW\x80c\xDF\\\xF7#\x11a\0\x87W\x80c\xDF\\\xF7#\x14a\x06rW\x80c\xE7\x0C&#\x14a\x06zW\x80c\xEF\x02DX\x14a\x06\x8AW\x80c\xF2\xFD\xE3\x8B\x14a\x06\x92W\x80c\xF5d\x0C\xF8\x14a\x06\xA5W\x80c\xFA\xBC\x1C\xBC\x14a\x06\xB8W`\0\x80\xFD[\x80c\xA6\x95c\xA9\x14a\x05\xCCW\x80c\xAD\xFC\xB0H\x14a\x05\xE3W\x80c\xB3\xEA\x18N\x14a\x05\xECW\x80c\xBF#\x15\xED\x14a\x05\xFFW\x80c\xC9\x87\xDE\x8E\x14a\x06:W\x80c\xCE\xFD\xC1\xD4\x14a\x06QW`\0\x80\xFD[\x80cr1\x14\xAB\x11a\x01 W\x80cr1\x14\xAB\x14a\x05[W\x80cz\xFA\x1E\xED\x14a\x05nW\x80cz\xFD\xD5K\x14a\x05\x81W\x80c\x88o\x11\x95\x14a\x05\x91W\x80c\x8D\xA5\xCB[\x14a\x05\xA4W\x80c\x8F\xC8r\x9A\x14a\x05\xB5W`\0\x80\xFD[\x80c]\xF4YF\x14a\x05\x07W\x80ch0H5\x14a\x05\x0FW\x80cm\x14\xA9\x87\x14a\x05\x17W\x80cn\x12_\xF4\x14a\x05\x1FW\x80cn\xFBF6\x14a\x052W\x80cqP\x18\xA6\x14a\x05SW`\0\x80\xFD[\x80cAx\x9DW\x11a\x02\x0BW\x80cSz))\x11a\x01\xC4W\x80cSz))\x14a\x04\x8FW\x80cT\xD1'\xDE\x14a\x04\xA6W\x80cY\\jg\x14a\x04\xB4W\x80cZ\xC8j\xB7\x14a\x04\xBCW\x80c\\\x15Vb\x14a\x04\xDFW\x80c\\\x97Z\xBB\x14a\x04\xFFW`\0\x80\xFD[\x80cAx\x9DW\x14a\x04\x06W\x80cE&[z\x14a\x042W\x80cJ\xE6\xB2\x03\x14a\x04CW\x80cMzq\x16\x14a\x04LW\x80cOs\x9Ft\x14a\x04\\W\x80cQjr'\x14a\x04|W`\0\x80\xFD[\x80c\x1A\xC2r\x97\x11a\x02]W\x80c\x1A\xC2r\x97\x14a\x03NW\x80c\x1C\x17\x8E\x9C\x14a\x03yW\x80c$Z{\xFC\x14a\x03\xA4W\x80c(0\xE8\xF9\x14a\x03\xBEW\x80c-\xB2\xE3\x9B\x14a\x03\xD3W\x80c5c\xB0\xD1\x14a\x03\xE6W`\0\x80\xFD[\x80c\x01\xA3\xF0\x13\x14a\x02\xA5W\x80c\x05\xC6\xB6c\x14a\x02\xBAW\x80c\x0E\xE0\xFD\xBD\x14a\x02\xCDW\x80c\x10\xD6z/\x14a\x02\xEFW\x80c\x13d9\xDD\x14a\x03\x02W\x80c\x13\xF8\x15\xED\x14a\x03\x15W[`\0\x80\xFD[a\x02\xB8a\x02\xB36`\x04aAzV[a\x06\xCBV[\0[a\x02\xB8a\x02\xC86`\x04aE\xDFV[a\x0B\\V[`\xA2Ta\x02\xDA\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\xB8a\x02\xFD6`\x04aF|V[a\x11!V[a\x02\xB8a\x03\x106`\x04aF\xA0V[a\x11\xD1V[a\x03@a\x03#6`\x04aF\xB9V[`\x99` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[`@Q\x90\x81R` \x01a\x02\xE6V[a\x03@a\x03\\6`\x04aF\xB9V[`\x9A` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[`\x97Ta\x03\x8C\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xE6V[`\x9ETa\x03\x8C\x90`\x01` \x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03\xC6a\x13\x10V[`@Qa\x02\xE6\x91\x90aGCV[a\x02\xB8a\x03\xE16`\x04aGdV[a\x13\x9EV[a\x03\xF9a\x03\xF46`\x04aH\rV[a\x15WV[`@Qa\x02\xE6\x91\x90aIhV[`\x97Ta\x04\x1D\x90`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\xE6V[a\x02\xB8a\x04@6`\x04aI{V[PV[a\x03@`\xA0T\x81V[`\x9CTa\x04\x1D\x90c\xFF\xFF\xFF\xFF\x16\x81V[a\x04oa\x04j6`\x04aI\xFDV[a\x19\xEDV[`@Qa\x02\xE6\x91\x90aKVV[a\x02\xB8a\x04\x8A6`\x04aK\xD4V[a!\x13V[`\x9CTa\x04\x1D\x90`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x02\xB8a\x04@6`\x04aL>V[a\x02\xB8a(\x0BV[a\x02\xDAa\x04\xCA6`\x04aLyV[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[a\x04\xF2a\x04\xED6`\x04aL\x9CV[a(\xD2V[`@Qa\x02\xE6\x91\x90aMHV[`fTa\x03@V[a\x03\x8Ca*\x9AV[a\x03\x8Ca+\rV[a\x03\x8Ca+WV[a\x02\xB8a\x05-6`\x04aM\x8CV[a+\xA1V[a\x05Ea\x05@6`\x04aM\xE0V[a0\x15V[`@Qa\x02\xE6\x92\x91\x90aN\xA0V[a\x02\xB8a0\xB5V[a\x02\xB8a\x05i6`\x04aF|V[a0\xC9V[`\x9FTa\x03\x8C\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x98Ta\x04\x1D\x90c\xFF\xFF\xFF\xFF\x16\x81V[`eTa\x03\x8C\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x03\x8CV[`\x9CTa\x04\x1D\x90`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`\x97Ta\x04\x1D\x90`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x03@`\xA1T\x81V[a\x02\xB8a\x05\xFA6`\x04aF\xA0V[a1%V[a\x06-a\x06\r6`\x04aF\xB9V[`\x9B` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[`@Qa\x02\xE6\x91\x90aN\xFFV[`\x97Ta\x04\x1D\x90`\x01`\xC0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x06da\x06_6`\x04aO'V[a5>V[`@Qa\x02\xE6\x92\x91\x90aOiV[a\x03\x8Ca6\xD0V[`\x9ETa\x04\x1D\x90c\xFF\xFF\xFF\xFF\x16\x81V[a\x03@`d\x81V[a\x02\xB8a\x06\xA06`\x04aF|V[a7\x1AV[a\x02\xB8a\x06\xB36`\x04aM\x8CV[a7\x90V[a\x02\xB8a\x06\xC66`\x04aF\xA0V[a;\xD1V[a\x06\xD3a=-V[`\0a\x06\xE2` \x83\x01\x83aO\x8AV[\x90P`\0a\x06\xF6``\x85\x01`@\x86\x01aO\x8AV[\x90P`\0a\x07\n`@\x86\x01` \x87\x01aO\x8AV[\x90P6`\0a\x07\x1C`\xA0\x88\x01\x88aO\xA7V[\x90\x92P\x90P`\0a\x073`\xE0\x89\x01`\xC0\x8A\x01aO\x8AV[`\0\x80\x80R`\x99` \x90\x81R\x91\x92P`\0\x80Q` a\\\xCC\x839\x81Q\x91R\x91a\x07^\x90\x8A\x01\x8AaO\x8AV[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x88`@Q` \x01a\x07\x8A\x91\x90aP[V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x14a\x07\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xBD\x90aQ5V[`@Q\x80\x91\x03\x90\xFD[`\0\x80\x80R`\x9B` \x90\x81R`\x01\x91`\0\x80Q` a\\\x8C\x839\x81Q\x91R\x91a\x07\xF1\x90\x8B\x01\x8BaO\x8AV[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16`\x04\x81\x11\x15a\x08\x1CWa\x08\x1CaN\xE9V[\x14a\x089W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xBD\x90aQ\x92V[`\0\x80\x80R`\x9A` \x90\x81R\x7F\xBEf \xBD3F\xE5\xD7\xF88\x7F\xBE\xC0\x98\x1A\xA0\xD6(\x9D\"\xEF\xA7\xC95\xF9\xEFhA\xBF*\x98\xC7\x90\x82\x90a\x08u\x90\x8B\x01\x8BaO\x8AV[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x14a\x08\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xBD\x90aQ\x92V[`\x97Ta\x08\xC7\x90`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x85aQ\xF4V[c\xFF\xFF\xFF\xFF\x16Cc\xFF\xFF\xFF\xFF\x16\x11\x15a\x08\xF2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xBD\x90aR\x1CV[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`@\x80Q`\x80\x81\x01\x82Rc\xFF\xFF\xFF\xFFC\x16\x81R`\0` \x80\x83\x01\x82\x90R\x84\x81\x01Q\x83\x85\x01R\x84Q``\x84\x01R\x92Q\x90\x92a\tD\x91\x8C\x91\x84\x91\x01aR\xD9V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x91\x82\x01 `\0\x80\x80R`\x9A\x83R\x90\x91\x7F\xBEf \xBD3F\xE5\xD7\xF88\x7F\xBE\xC0\x98\x1A\xA0\xD6(\x9D\"\xEF\xA7\xC95\xF9\xEFhA\xBF*\x98\xC7\x91\x90a\t\x96\x90\x8E\x01\x8EaO\x8AV[c\xFF\xFF\xFF\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 \x93\x90\x93U\x8C\x015`\xA1U\x81\x80R`\x9B\x81R`\x04\x91`\0\x80Q` a\\\x8C\x839\x81Q\x91R\x91a\t\xE1\x90\x8E\x01\x8EaO\x8AV[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16`\x04\x81\x11\x15a\n\x0CWa\n\x0CaN\xE9V[Pa\n\x1F\x90P`@\x8C\x01` \x8D\x01aO\x8AV[`\x9C\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x01`@\x1B\x02c\xFF\xFF\xFF\xFF`@\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90Ua\nQ``\x8C\x01\x8CaO\xA7V[a\n]\x91`\x9D\x91a@\xB7V[Pa\nn`\xA0\x8C\x01`\x80\x8D\x01aO\x8AV[`\x9E\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\n\x95` \x8C\x01\x8CaO\x8AV[`\x9C\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90Ua\n\xC8` \x8B\x01\x8BaO\x8AV[c\xFF\xFF\xFF\xFF\x16\x7F\xFF)\x08H=t\xB6\xB7\0S\xDDG2`\xAC\xF1\xB0\x9E\x0B\xA0x\x1B\xF9A\0\xBB\x82wX\x17I\xDE\x8B`@Qa\n\xFD\x91\x90aR\xF9V[`@Q\x80\x91\x03\x90\xA2a\x0B\x12` \x8B\x01\x8BaO\x8AV[c\xFF\xFF\xFF\xFF\x16\x7F\xDF\"\xF3U\x8EHA\xB6=w\x17\x95F\xB3\xEA\xE6>N4;\xBEu'F\xB0\x93\x16+\xC5&\xBEL\x8B`@Qa\x0BG\x91\x90aR\xF9V[`@Q\x80\x91\x03\x90\xA2PPPPPPPPPPPV[`\x9ET`\x01` \x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0B\x8DW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xBD\x90aS\x07V[`\0a\x0B\x9F`\x80\x85\x01``\x86\x01aO\x8AV[\x90P`\0a\x0B\xB3``\x86\x01`@\x87\x01aO\x8AV[\x90P6`\0a\x0B\xC5`\x80\x88\x01\x88aO\xA7V[\x90\x92P\x90P`\0a\x0B\xDC`\xC0\x89\x01`\xA0\x8A\x01aO\x8AV[`\x01`\0\x90\x81R`\x99` \x90\x81R\x91\x92P\x7F\xBB\x86\xFB\xC04\xF4\xE3\x82\x92\x99t\xBC\xD8A\x9E\xD6&\xB0\xEAd\x7F\x96-\x89\xBA/\xB6\xBD(xZ\xB9\x91a\x0C\x1B\x90\x8A\x01\x8AaO\x8AV[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x88`@Q` \x01a\x0CG\x91\x90aS&V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x14a\x0CzW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xBD\x90aQ5V[`\x01`\0\x81\x81R`\x9B` \x90\x81R`\0\x80Q` a\\\xAC\x839\x81Q\x91R\x91\x90a\x0C\xA5\x90\x8B\x01\x8BaO\x8AV[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16`\x04\x81\x11\x15a\x0C\xD0Wa\x0C\xD0aN\xE9V[\x14a\x0C\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xBD\x90aQ\x92V[`\x01`\0\x90\x81R`\x9A` \x90\x81R\x7F[T+R\x98\x1CO/\xA9\x96U\x14\xD5\xBB\x7F7\xF1\xB7\xBC\t\x02\xA6\xA4\xDCk\x04\xDC\x05\xBE\x85Xk\x90\x82\x90a\r+\x90\x8B\x01\x8BaO\x8AV[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x14a\rcW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xBD\x90aQ\x92V[`\x97Ta\r}\x90`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x85aQ\xF4V[c\xFF\xFF\xFF\xFF\x16Cc\xFF\xFF\xFF\xFF\x16\x11\x15a\r\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xBD\x90aR\x1CV[`\0\x87`@Q` \x01a\r\xBB\x91\x90aT\0V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x80`\x97`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cn\xFBF6\x84\x88\x88\x8C\x8E`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\x1F\x95\x94\x93\x92\x91\x90aT\x98V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E<W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0Ed\x91\x90\x81\x01\x90aV\x0FV[`@\x80Q`\x80\x81\x01\x82Rc\xFF\xFF\xFF\xFFC\x16\x81R` \x80\x82\x01\x84\x90R\x80\x85\x01Q\x82\x84\x01R\x84Q``\x83\x01R\x91Q\x93\x95P\x91\x93P\x90\x91a\x0E\xA6\x91\x8D\x91\x84\x91\x01aV\xABV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x91\x82\x01 `\x01`\0\x90\x81R`\x9A\x83R\x90\x91\x7F[T+R\x98\x1CO/\xA9\x96U\x14\xD5\xBB\x7F7\xF1\xB7\xBC\t\x02\xA6\xA4\xDCk\x04\xDC\x05\xBE\x85Xk\x91\x90a\x0E\xFA\x90\x8F\x01\x8FaO\x8AV[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 U`\x03`\x9B`\0`\x01\x80\x81\x11\x15a\x0F)Wa\x0F)aN\xE9V[\x81R` \x01\x90\x81R` \x01`\0 `\0\x8D`\0\x01` \x81\x01\x90a\x0FL\x91\x90aO\x8AV[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16`\x04\x81\x11\x15a\x0FwWa\x0FwaN\xE9V[Pa\x0F\x87\x90P` \x8D\x01\x8DaO\x8AV[c\xFF\xFF\xFF\xFF\x16\x7F\x07\xBA\x8D|B\t\xC1Nm.\xD5\xFDEB\xE9\xD8\xF4~\x1A\xF3\xDA\x0C\xBD\x9CP\xE2\xD0\xD3\xE8}\xFF\xF0\x8C\x83`@Qa\x0F\xBE\x92\x91\x90aV\xABV[`@Q\x80\x91\x03\x90\xA2`\0[\x86\x81\x10\x15a\x10`W\x85`\xFF\x16\x84` \x01Q\x82\x81Q\x81\x10a\x0F\xEBWa\x0F\xEBaV\xCBV[` \x02` \x01\x01Qa\x0F\xFD\x91\x90aV\xE1V[`\x01`\x01``\x1B\x03\x16`d\x85`\0\x01Q\x83\x81Q\x81\x10a\x10\x1EWa\x10\x1EaV\xCBV[` \x02` \x01\x01Q`\x01`\x01``\x1B\x03\x16a\x109\x91\x90aW\x10V[\x10\x15a\x10NWPPPPPPPPPPPPPV[\x80a\x10X\x81aW/V[\x91PPa\x0F\xC9V[P`\x80\x8B\x015`\xA0U`\x01`\0\x90\x81R`\x9B` \x90\x81R`\x04\x91`\0\x80Q` a\\\xAC\x839\x81Q\x91R\x91a\x10\x96\x90\x8F\x01\x8FaO\x8AV[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16`\x04\x81\x11\x15a\x10\xC1Wa\x10\xC1aN\xE9V[PP`@\x8B\x015a\x10\xD5` \x8D\x01\x8DaO\x8AV[c\xFF\xFF\xFF\xFF\x16\x7F\x02\x04gs\xDF\xD0\xE0\"\xABk\xF6\x8C\xB0}s\xF0\x80\xF8zK\x82\xF7\xA4\xFC\xCF\xE7\xD9\x19\xD6\xC4\xEC\xB4\x8D`@Qa\x11\n\x91\x90aT\0V[`@Q\x80\x91\x03\x90\xA3PPPPPPPPP[PPPV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11tW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x98\x91\x90aWJV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x11\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xBD\x90aWgV[a\x04@\x81a=\x87V[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\x19W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12=\x91\x90aW\xB1V[a\x12YW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xBD\x90aW\xCEV[`fT\x81\x81\x16\x14a\x12\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07\xBDV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`\x9D\x80Ta\x13\x1D\x90aX\x16V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x13I\x90aX\x16V[\x80\x15a\x13\x96W\x80`\x1F\x10a\x13kWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x13\x96V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x13yW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x13\xBEWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x13\xD8WP0;\x15\x80\x15a\x13\xD8WP`\0T`\xFF\x16`\x01\x14[a\x14;W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x07\xBDV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x14^W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x14i\x89`\0a>~V[a\x14r\x88a?hV[`\x9E\x80T`\x01`\x01`\xA0\x1B\x03\x80\x8A\x16`\x01` \x1B\x02d\x01\0\0\0\0`\x01`\xC0\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90\x91U`\x9F\x80T\x88\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x90\x91\x16\x17\x90U`\xA2\x80T\x87\x15\x15`\xFF\x19\x90\x91\x16\x17\x90U`\x97\x80Tc\xFF\xFF\xFF\xFF\x85\x81\x16`\x01`\xC0\x1B\x02c\xFF\xFF\xFF\xFF`\xC0\x1B\x19\x91\x88\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xC0\x1B\x03\x19\x90\x93\x16\x94\x89\x16\x94\x90\x94\x17\x91\x90\x91\x17\x16\x91\x90\x91\x17\x90U\x80\x15a\x15LW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPPPV[```\0\x84`\x01`\x01`\xA0\x1B\x03\x16ch0H5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\x99W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xBD\x91\x90aWJV[\x90P`\0\x85`\x01`\x01`\xA0\x1B\x03\x16c\x9E\x99#\xC2`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\xFFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16#\x91\x90aWJV[\x90P`\0\x86`\x01`\x01`\xA0\x1B\x03\x16c]\xF4YF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16eW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\x89\x91\x90aWJV[\x90P`\0\x86Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16\xA6Wa\x16\xA6aA\xC8V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x16\xD9W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x16\xC4W\x90P[P\x90P`\0[\x87Q\x81\x10\x15a\x19\xE1W`\0\x88\x82\x81Q\x81\x10a\x16\xFCWa\x16\xFCaV\xCBV[\x01` \x01Q`@Qc\x89\x02bE`\xE0\x1B\x81R`\xF8\x91\x90\x91\x1C`\x04\x82\x01\x81\x90Rc\xFF\xFF\xFF\xFF\x8A\x16`$\x83\x01R\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x87\x16\x90c\x89\x02bE\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17]W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x17\x85\x91\x90\x81\x01\x90aXKV[\x90P\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17\xA0Wa\x17\xA0aA\xC8V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x17\xEBW\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a\x17\xBEW\x90P[P\x84\x84\x81Q\x81\x10a\x17\xFEWa\x17\xFEaV\xCBV[` \x02` \x01\x01\x81\x90RP`\0[\x81Q\x81\x10\x15a\x19\xCBW`@Q\x80``\x01`@R\x80\x87`\x01`\x01`\xA0\x1B\x03\x16cG\xB3\x14\xE8\x85\x85\x81Q\x81\x10a\x18AWa\x18AaV\xCBV[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18g\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\x84W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\xA8\x91\x90aWJV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x83\x81Q\x81\x10a\x18\xC8Wa\x18\xC8aV\xCBV[` \x02` \x01\x01Q\x81R` \x01\x89`\x01`\x01`\xA0\x1B\x03\x16c\xFA(\xC6'\x85\x85\x81Q\x81\x10a\x18\xF6Wa\x18\xF6aV\xCBV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x04\x81\x01\x91\x90\x91R`\xFF\x88\x16`$\x82\x01Rc\xFF\xFF\xFF\xFF\x8F\x16`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19RW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19v\x91\x90aX\xDBV[`\x01`\x01``\x1B\x03\x16\x81RP\x85\x85\x81Q\x81\x10a\x19\x94Wa\x19\x94aV\xCBV[` \x02` \x01\x01Q\x82\x81Q\x81\x10a\x19\xADWa\x19\xADaV\xCBV[` \x02` \x01\x01\x81\x90RP\x80\x80a\x19\xC3\x90aW/V[\x91PPa\x18\x0CV[PPP\x80\x80a\x19\xD9\x90aW/V[\x91PPa\x16\xDFV[P\x97\x96PPPPPPPV[a\x1A\x18`@Q\x80`\x80\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`\0\x87`\x01`\x01`\xA0\x1B\x03\x16ch0H5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1AXW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A|\x91\x90aWJV[\x90Pa\x1A\xA9`@Q\x80`\x80\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`@Qca\xC8\xA1/`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90c\xC3\x91B^\x90a\x1A\xD9\x90\x8B\x90\x89\x90\x89\x90`\x04\x01aX\xF6V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\xF6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1B\x1E\x91\x90\x81\x01\x90aY@V[\x81R`@Qc@\xE0:\x81`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\x81\xC0u\x02\x90a\x1BP\x90\x8B\x90\x8B\x90\x8B\x90`\x04\x01aY\xCEV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1BmW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1B\x95\x91\x90\x81\x01\x90aY@V[`@\x82\x01R\x85`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1B\xB2Wa\x1B\xB2aA\xC8V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1B\xE5W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1B\xD0W\x90P[P``\x82\x01R`\0[`\xFF\x81\x16\x87\x11\x15a $W`\0\x85`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1C\x13Wa\x1C\x13aA\xC8V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1C<W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x83``\x01Q\x83`\xFF\x16\x81Q\x81\x10a\x1CVWa\x1CVaV\xCBV[` \x02` \x01\x01\x81\x90RP`\0[\x86\x81\x10\x15a\x1F$W`\0\x8C`\x01`\x01`\xA0\x1B\x03\x16c\x04\xECcQ\x8A\x8A\x85\x81\x81\x10a\x1C\x8FWa\x1C\x8FaV\xCBV[\x90P` \x02\x015\x8E\x88`\0\x01Q\x86\x81Q\x81\x10a\x1C\xADWa\x1C\xADaV\xCBV[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1C\xEA\x93\x92\x91\x90\x92\x83Rc\xFF\xFF\xFF\xFF\x91\x82\x16` \x84\x01R\x16`@\x82\x01R``\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\x07W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D+\x91\x90aY\xEEV[\x90P`\x01`\x01`\xC0\x1B\x03\x81\x16a\x1D\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R\x7FOperatorStateRetriever.getCheckS`D\x82\x01R\x7FignaturesIndices: operator must `d\x82\x01R\x7Fbe registered at blocknumber\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x07\xBDV[\x8A\x8A\x85`\xFF\x16\x81\x81\x10a\x1D\xE4Wa\x1D\xE4aV\xCBV[`\x01`\x01`\xC0\x1B\x03\x84\x16\x92\x015`\xF8\x1C\x91\x90\x91\x1C`\x01\x90\x81\x16\x14\x15\x90Pa\x1F\x11W\x85`\x01`\x01`\xA0\x1B\x03\x16c\xDD\x98F\xB9\x8A\x8A\x85\x81\x81\x10a\x1E&Wa\x1E&aV\xCBV[\x90P` \x02\x015\x8D\x8D\x88`\xFF\x16\x81\x81\x10a\x1EBWa\x1EBaV\xCBV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x87\x90\x1B\x16\x81R`\x04\x81\x01\x94\x90\x94R\x91\x90\x91\x015`\xF8\x1C`$\x83\x01RPc\xFF\xFF\xFF\xFF\x8F\x16`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\x98W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\xBC\x91\x90aZ\x17V[\x85``\x01Q\x85`\xFF\x16\x81Q\x81\x10a\x1E\xD5Wa\x1E\xD5aV\xCBV[` \x02` \x01\x01Q\x84\x81Q\x81\x10a\x1E\xEEWa\x1E\xEEaV\xCBV[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x82a\x1F\r\x81aW/V[\x93PP[P\x80a\x1F\x1C\x81aW/V[\x91PPa\x1CdV[P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F?Wa\x1F?aA\xC8V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1FhW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a\x1F\xE9W\x84``\x01Q\x84`\xFF\x16\x81Q\x81\x10a\x1F\x8FWa\x1F\x8FaV\xCBV[` \x02` \x01\x01Q\x81\x81Q\x81\x10a\x1F\xA8Wa\x1F\xA8aV\xCBV[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a\x1F\xC2Wa\x1F\xC2aV\xCBV[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x1F\xE1\x81aW/V[\x91PPa\x1FnV[P\x80\x84``\x01Q\x84`\xFF\x16\x81Q\x81\x10a \x04Wa \x04aV\xCBV[` \x02` \x01\x01\x81\x90RPPP\x80\x80a \x1C\x90aZ4V[\x91PPa\x1B\xEEV[P`\0\x89`\x01`\x01`\xA0\x1B\x03\x16c]\xF4YF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a eW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \x89\x91\x90aWJV[`@Qc5IR\xA3`\xE2\x1B\x81R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xD5%J\x8C\x90a \xBC\x90\x8B\x90\x8B\x90\x8E\x90`\x04\x01aZTV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a \xD9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra!\x01\x91\x90\x81\x01\x90aY@V[` \x83\x01RP\x98\x97PPPPPPPPV[`\x9CT`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15`\0a!6``\x86\x01`@\x87\x01aO\x8AV[\x90P\x81\x15a!\xC1W`\xA2T`\xFF\x16\x15a!\x7FW`\x9ET`\x01` \x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x163\x14a!zW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xBD\x90aS\x07V[a!\xF2V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a!zW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd \xBA\xBA4\x19`\xD9\x1B`D\x82\x01R`d\x01a\x07\xBDV[`\x9ET`\x01` \x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x163\x14a!\xF2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xBD\x90aS\x07V[`\0a\"\x04`@\x87\x01` \x88\x01aO\x8AV[\x90P6`\0a\"\x16`\xA0\x89\x01\x89aO\xA7V[\x90\x92P\x90P`\0a\"-`\xE0\x8A\x01`\xC0\x8B\x01aO\x8AV[`\0\x80\x80R`\x99` \x90\x81R\x91\x92P`\0\x80Q` a\\\xCC\x839\x81Q\x91R\x91a\"X\x90\x8B\x01\x8BaO\x8AV[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x89`@Q` \x01a\"\x84\x91\x90aP[V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x14a\"\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xBD\x90aQ5V[`\0\x80\x80R`\x9B` \x90\x81R`\x01\x91`\0\x80Q` a\\\x8C\x839\x81Q\x91R\x91a\"\xE2\x90\x8C\x01\x8CaO\x8AV[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16`\x04\x81\x11\x15a#\rWa#\raN\xE9V[\x14a#*W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xBD\x90aQ\x92V[`\0\x80\x80R`\x9A` \x90\x81R\x7F\xBEf \xBD3F\xE5\xD7\xF88\x7F\xBE\xC0\x98\x1A\xA0\xD6(\x9D\"\xEF\xA7\xC95\xF9\xEFhA\xBF*\x98\xC7\x90\x82\x90a#f\x90\x8C\x01\x8CaO\x8AV[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x14a#\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xBD\x90aQ\x92V[`\x97Ta#\xB8\x90`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x85aQ\xF4V[c\xFF\xFF\xFF\xFF\x16Cc\xFF\xFF\xFF\xFF\x16\x11\x15a#\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xBD\x90aR\x1CV[`\0\x88`@Q` \x01a#\xF6\x91\x90aR\xF9V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x83\x83\x01\x90\x92R``\x80\x84R\x90\x83\x01R\x91P`\0\x88\x15a$\xBBW`\x97`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cn\xFBF6\x84\x88\x88\x8C\x8F`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a$p\x95\x94\x93\x92\x91\x90aT\x98V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a$\x8DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra$\xB5\x91\x90\x81\x01\x90aV\x0FV[\x90\x92P\x90P[`@\x80Q`\x80\x81\x01\x82Rc\xFF\xFF\xFF\xFFC\x16\x81R` \x80\x82\x01\x84\x90R\x84\x81\x01Q\x82\x84\x01R\x84Q``\x83\x01R\x91Q\x90\x91a$\xF7\x91\x8E\x91\x84\x91\x01aR\xD9V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 `\x9A`\0\x80\x81R` \x01\x90\x81R` \x01`\0 `\0\x8E`\0\x01` \x81\x01\x90a%7\x91\x90aO\x8AV[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 U`\x03`\x9B`\0\x80`\x01\x81\x11\x15a%fWa%faN\xE9V[\x81R` \x01\x90\x81R` \x01`\0 `\0\x8E`\0\x01` \x81\x01\x90a%\x89\x91\x90aO\x8AV[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16`\x04\x81\x11\x15a%\xB4Wa%\xB4aN\xE9V[Pa%\xC4\x90P` \x8E\x01\x8EaO\x8AV[c\xFF\xFF\xFF\xFF\x16\x7FG\xAD\xAC\xB0\xB6\xBB\xD7&\xAE9\xACl\0l\xCA\x1C \x06\xC9\xAE\xDA\xA8\x82\xDC\xBA|H\x04\xDB|A\xCE\x8D\x83`@Qa%\xFB\x92\x91\x90aR\xD9V[`@Q\x80\x91\x03\x90\xA2\x89\x15a&\xA6W`\0[\x86\x81\x10\x15a&\xA4W\x85`\xFF\x16\x84` \x01Q\x82\x81Q\x81\x10a&.Wa&.aV\xCBV[` \x02` \x01\x01Qa&@\x91\x90aV\xE1V[`\x01`\x01``\x1B\x03\x16`d\x85`\0\x01Q\x83\x81Q\x81\x10a&aWa&aaV\xCBV[` \x02` \x01\x01Q`\x01`\x01``\x1B\x03\x16a&|\x91\x90aW\x10V[\x10\x15a&\x92WPPPPPPPPPPPPPPV[\x80a&\x9C\x81aW/V[\x91PPa&\x0CV[P[`@\x8C\x015`\xA1U`\x04`\x9B`\0\x80\x81R` \x01\x90\x81R` \x01`\0 `\0\x8E`\0\x01` \x81\x01\x90a&\xD8\x91\x90aO\x8AV[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16`\x04\x81\x11\x15a'\x03Wa'\x03aN\xE9V[Pa'\x16\x90P`@\x8E\x01` \x8F\x01aO\x8AV[`\x9C\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x01`@\x1B\x02c\xFF\xFF\xFF\xFF`@\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90Ua'H``\x8E\x01\x8EaO\xA7V[a'T\x91`\x9D\x91a@\xB7V[Pa'e`\xA0\x8E\x01`\x80\x8F\x01aO\x8AV[`\x9E\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua'\x8C` \x8E\x01\x8EaO\x8AV[`\x9C\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90Ua'\xBF` \x8D\x01\x8DaO\x8AV[c\xFF\xFF\xFF\xFF\x16\x7F\xFF)\x08H=t\xB6\xB7\0S\xDDG2`\xAC\xF1\xB0\x9E\x0B\xA0x\x1B\xF9A\0\xBB\x82wX\x17I\xDE\x8D`@Qa'\xF4\x91\x90aR\xF9V[`@Q\x80\x91\x03\x90\xA2PPPPPPPPPPPPPV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a(SW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(w\x91\x90aW\xB1V[a(\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xBD\x90aW\xCEV[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[```\0\x84`\x01`\x01`\xA0\x1B\x03\x16c\xC3\x91B^\x84\x86`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a)\x04\x92\x91\x90aZ~V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a)!W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra)I\x91\x90\x81\x01\x90aY@V[\x90P`\0\x84Q`\x01`\x01`@\x1B\x03\x81\x11\x15a)fWa)faA\xC8V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a)\x8FW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x85Q\x81\x10\x15a*\x90W\x86`\x01`\x01`\xA0\x1B\x03\x16c\x04\xECcQ\x87\x83\x81Q\x81\x10a)\xBFWa)\xBFaV\xCBV[` \x02` \x01\x01Q\x87\x86\x85\x81Q\x81\x10a)\xDAWa)\xDAaV\xCBV[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a*\x17\x93\x92\x91\x90\x92\x83Rc\xFF\xFF\xFF\xFF\x91\x82\x16` \x84\x01R\x16`@\x82\x01R``\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a*4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*X\x91\x90aY\xEEV[`\x01`\x01`\xC0\x1B\x03\x16\x82\x82\x81Q\x81\x10a*sWa*saV\xCBV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a*\x88\x81aW/V[\x91PPa)\x95V[P\x95\x94PPPPPV[`\x97T`@\x80Qc.\xFA,\xA3`\xE1\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c]\xF4YF\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a*\xE4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+\x08\x91\x90aWJV[\x90P\x90V[`\x97T`@\x80Qch0H5`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91ch0H5\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a*\xE4W=`\0\x80>=`\0\xFD[`\x97T`@\x80Qcm\x14\xA9\x87`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cm\x14\xA9\x87\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a*\xE4W=`\0\x80>=`\0\xFD[`\x9FT`\x01`\x01`\xA0\x1B\x03\x163\x14a+\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdAuth1`\xD8\x1B`D\x82\x01R`d\x01a\x07\xBDV[`\x9CTc\xFF\xFF\xFF\xFF`\x01`@\x1B\x90\x91\x04\x16C\x14\x80\x15\x90a,\x02WPC\x15\x15[a,\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xBD\x90aZ\xD2V[`@\x80Q`\xE0\x81\x01\x82R`\0\x81\x83\x01\x81\x90R``\x80\x83\x01\x81\x90R`\xA0\x83\x01R`\xC0\x82\x01R`\x97T`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x82RC\x81\x16` \x80\x84\x01\x91\x90\x91R\x90\x86\x16`\x80\x83\x01R\x82Q`\x1F\x85\x01\x82\x90\x04\x82\x02\x81\x01\x82\x01\x90\x93R\x83\x83R\x90\x91\x90\x84\x90\x84\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPPP``\x82\x01R`\x9CT`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16a-\rWc\xFF\xFF\xFF\xFFC\x16`@\x80\x83\x01\x91\x90\x91R\x80Q` `\x1F\x85\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x83\x81R\x90\x84\x90\x84\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPPP`\xA0\x82\x01Rc\xFF\xFF\xFF\xFF\x84\x16`\xC0\x82\x01Ra-\xC1V[`\x9CT`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16`@\x82\x01R`\x9D\x80Ta-/\x90aX\x16V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta-[\x90aX\x16V[\x80\x15a-\xA8W\x80`\x1F\x10a-}Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a-\xA8V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a-\x8BW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`\xA0\x82\x01R`\x9ETc\xFF\xFF\xFF\xFF\x16`\xC0\x82\x01R[`\x97T`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15a.hW`\x97T`\0\x90a-\xF5\x90`\x01\x90`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16a[/V[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\0\x80Q` a\\\x8C\x839\x81Q\x91R` R`@\x90 T\x90\x91P`\x01\x90`\xFF\x16`\x04\x81\x11\x15a.2Wa.2aN\xE9V[\x14\x15a.fWc\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\0\x80Q` a\\\x8C\x839\x81Q\x91R` R`@\x90 \x80T`\xFF\x19\x16`\x02\x17\x90U[P[`\x98Tc\xFF\xFF\xFF\xFF\x16\x15a/\x01W`\x98T`\0\x90a.\x8E\x90`\x01\x90c\xFF\xFF\xFF\xFF\x16a[/V[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\0\x80Q` a\\\xAC\x839\x81Q\x91R` R`@\x90 T\x90\x91P`\x01\x90`\xFF\x16`\x04\x81\x11\x15a.\xCBWa.\xCBaN\xE9V[\x14\x15a.\xFFWc\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\0\x80Q` a\\\xAC\x839\x81Q\x91R` R`@\x90 \x80T`\xFF\x19\x16`\x02\x17\x90U[P[\x80`@Q` \x01a/\x12\x91\x90a[TV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 `\x97\x80Tc\xFF\xFF\xFF\xFF`\x01`\xE0\x1B\x91\x82\x90\x04\x81\x16`\0\x90\x81R`\0\x80Q` a\\\xCC\x839\x81Q\x91R\x86R\x86\x81 \x94\x90\x94U\x82T\x82\x90\x04\x81\x16\x84R`\0\x80Q` a\\\x8C\x839\x81Q\x91R\x90\x94R\x93\x90\x91 \x80T`\x01`\xFF\x19\x90\x91\x16\x17\x90U`\x9C\x80Tc\xFF\xFF\xFF\xFF\x19\x16C\x84\x16\x17\x90UT\x91\x90\x91\x04\x16\x90\x7F\xFA\xF4\xB2\x05Dy\xD0\xF8>\x90\x9Bs\xCD\xE2\xA6\xCB\x18\xEC*\x93\xBA\x8A\xD5\xA6#)\0\x1C\x86\xB1\xF3\xEA\x90a/\xCC\x90\x84\x90a[TV[`@Q\x80\x91\x03\x90\xA2`\x97Ta/\xEF\x90`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16`\x01aQ\xF4V[`\x97`\x1Ca\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`\x97T`@Qc7}\xA3\x1B`\xE1\x1B\x81R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cn\xFBF6\x90a0b\x90\x8A\x90\x8A\x90\x8A\x90\x8A\x90\x8A\x90`\x04\x01aT\x98V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a0\x7FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra0\xA7\x91\x90\x81\x01\x90aV\x0FV[\x91P\x91P\x95P\x95\x93PPPPV[a0\xBDa=-V[a0\xC7`\0a?hV[V[a0\xD1a=-V[`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x90\x1AeM\xC80\xC9N\x8A\x12\xC9\xA3\xBC\n\x92\xAC\x11\xB5\xCF(\x04l\xA8\xD1\x90i\x1C\xDA\xF5 \x90\x16\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`\x9FT`\x01`\x01`\xA0\x1B\x03\x163\x14a1gW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdAuth1`\xD8\x1B`D\x82\x01R`d\x01a\x07\xBDV[`\x9CT`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15\x80\x15\x90a1\x84WPC\x15\x15[a1\xC2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x13\xDC\x08\x14\xDD\x18]\x19H\x1D[\x9A[\x9A]`\x8A\x1B`D\x82\x01R`d\x01a\x07\xBDV[`\x98T`\x97T`@\x80Q`\xC0\x81\x01\x82R`\0``\x80\x83\x01\x91\x90\x91R`\x80\x82\x01Rc\xFF\xFF\xFF\xFF\x93\x84\x16\x80\x82R` \x82\x01\x86\x90RC\x85\x16\x92\x82\x01\x92\x90\x92R`\x9ET\x84\x16`\xA0\x82\x01R`\x9D\x80T\x92\x94`\x01`\xE0\x1B\x90\x94\x04\x90\x93\x16\x92\x90\x91a2%\x90aX\x16V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta2Q\x90aX\x16V[\x80\x15a2\x9EW\x80`\x1F\x10a2sWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a2\x9EV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a2\x81W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`\x80\x82\x01R`\x9CT`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x90\x81\x16``\x83\x01R\x83\x16\x15a3FW`\0a2\xD3`\x01\x85a[/V[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\0\x80Q` a\\\xAC\x839\x81Q\x91R` R`@\x90 T\x90\x91P`\x01\x90`\xFF\x16`\x04\x81\x11\x15a3\x10Wa3\x10aN\xE9V[\x14\x15a3DWc\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\0\x80Q` a\\\xAC\x839\x81Q\x91R` R`@\x90 \x80T`\xFF\x19\x16`\x02\x17\x90U[P[c\xFF\xFF\xFF\xFF\x82\x16\x15a4]W`\0a3_`\x01\x84a[/V[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\0\x80Q` a\\\x8C\x839\x81Q\x91R` R`@\x90 T\x90\x91P`\x01\x90`\xFF\x16`\x04\x81\x11\x15a3\x9CWa3\x9CaN\xE9V[\x14\x15a4[W`\x97T`\x9CTa3\xC2\x91c\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x90\x91\x04\x81\x16\x91\x16aQ\xF4V[c\xFF\xFF\xFF\xFF\x16Cc\xFF\xFF\xFF\xFF\x16\x11\x15a4-W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FRdTask can't yet cancel the init`D\x82\x01Rf OpTask`\xC8\x1B`d\x82\x01R`\x84\x01a\x07\xBDV[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\0\x80Q` a\\\x8C\x839\x81Q\x91R` R`@\x90 \x80T`\xFF\x19\x16`\x02\x17\x90U[P[\x80`@Q` \x01a4n\x91\x90a[\xEAV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 c\xFF\xFF\xFF\xFF\x87\x16`\0\x81\x81R\x7F\xBB\x86\xFB\xC04\xF4\xE3\x82\x92\x99t\xBC\xD8A\x9E\xD6&\xB0\xEAd\x7F\x96-\x89\xBA/\xB6\xBD(xZ\xB9\x84R\x84\x81 \x92\x90\x92U`\0\x80Q` a\\\xAC\x839\x81Q\x91R\x90\x92R\x91\x90\x91 \x80T`\xFF\x19\x16`\x01\x17\x90U\x90\x7F\x16\x1F\xAA]\x92\xF6\xBF\x02b\x13\x9CW\xC8|\xA6\xCC`\xE5\xB3\xC9\xC34\x1D\xC1u\xCE\xC9z%\x16\xCC}\x90a5\x0B\x90\x84\x90a[\xEAV[`@Q\x80\x91\x03\x90\xA2a5\x1E\x83`\x01aQ\xF4V[`\x98\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UPPPPV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91``\x91\x83\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x84\x81`\0\x81Q\x81\x10a5yWa5yaV\xCBV[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Qca\xC8\xA1/`\xE1\x1B\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90c\xC3\x91B^\x90a5\xB5\x90\x88\x90\x86\x90`\x04\x01aZ~V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a5\xD2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra5\xFA\x91\x90\x81\x01\x90aY@V[`\0\x81Q\x81\x10a6\x0CWa6\x0CaV\xCBV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x88\x90Rc\xFF\xFF\xFF\xFF\x87\x81\x16`$\x83\x01R\x90\x91\x16`D\x82\x01\x81\x90R\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x89\x16\x90c\x04\xECcQ\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a6xW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a6\x9C\x91\x90aY\xEEV[`\x01`\x01`\xC0\x1B\x03\x16\x90P`\0a6\xB2\x82a?\xBAV[\x90P\x81a6\xC0\x8A\x83\x8Aa\x15WV[\x95P\x95PPPPP\x93P\x93\x91PPV[`\x97T`@\x80Qc\xDF\\\xF7#`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xDF\\\xF7#\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a*\xE4W=`\0\x80>=`\0\xFD[a7\"a=-V[`\x01`\x01`\xA0\x1B\x03\x81\x16a7\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x07\xBDV[a\x04@\x81a?hV[a7\x98a=-V[`\x9CTc\xFF\xFF\xFF\xFF`\x01`@\x1B\x90\x91\x04\x16C\x14\x80\x15\x90a7\xB7WPC\x15\x15[a7\xD3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xBD\x90aZ\xD2V[`@\x80Q`\xE0\x81\x01\x82R`\0\x81\x83\x01\x81\x90R``\x80\x83\x01\x81\x90R`\xA0\x83\x01R`\xC0\x82\x01R`\x97T`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x82RC\x81\x16` \x80\x84\x01\x91\x90\x91R\x90\x86\x16`\x80\x83\x01R\x82Q`\x1F\x85\x01\x82\x90\x04\x82\x02\x81\x01\x82\x01\x90\x93R\x83\x83R\x90\x91\x90\x84\x90\x84\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPPP``\x82\x01R`\x9CT`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16a8\xC2Wc\xFF\xFF\xFF\xFFC\x16`@\x80\x83\x01\x91\x90\x91R\x80Q` `\x1F\x85\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x83\x81R\x90\x84\x90\x84\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPPP`\xA0\x82\x01Rc\xFF\xFF\xFF\xFF\x84\x16`\xC0\x82\x01Ra9vV[`\x9CT`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16`@\x82\x01R`\x9D\x80Ta8\xE4\x90aX\x16V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta9\x10\x90aX\x16V[\x80\x15a9]W\x80`\x1F\x10a92Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a9]V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a9@W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`\xA0\x82\x01R`\x9ETc\xFF\xFF\xFF\xFF\x16`\xC0\x82\x01R[`\x97T`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15a:\x1DW`\x97T`\0\x90a9\xAA\x90`\x01\x90`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16a[/V[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\0\x80Q` a\\\x8C\x839\x81Q\x91R` R`@\x90 T\x90\x91P`\x01\x90`\xFF\x16`\x04\x81\x11\x15a9\xE7Wa9\xE7aN\xE9V[\x14\x15a:\x1BWc\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\0\x80Q` a\\\x8C\x839\x81Q\x91R` R`@\x90 \x80T`\xFF\x19\x16`\x02\x17\x90U[P[`\x98Tc\xFF\xFF\xFF\xFF\x16\x15a:\xB6W`\x98T`\0\x90a:C\x90`\x01\x90c\xFF\xFF\xFF\xFF\x16a[/V[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\0\x80Q` a\\\xAC\x839\x81Q\x91R` R`@\x90 T\x90\x91P`\x01\x90`\xFF\x16`\x04\x81\x11\x15a:\x80Wa:\x80aN\xE9V[\x14\x15a:\xB4Wc\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\0\x80Q` a\\\xAC\x839\x81Q\x91R` R`@\x90 \x80T`\xFF\x19\x16`\x02\x17\x90U[P[\x80`@Q` \x01a:\xC7\x91\x90a[TV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 `\x97\x80Tc\xFF\xFF\xFF\xFF`\x01`\xE0\x1B\x91\x82\x90\x04\x81\x16`\0\x90\x81R`\0\x80Q` a\\\xCC\x839\x81Q\x91R\x86R\x86\x81 \x94\x90\x94U\x82T\x82\x90\x04\x81\x16\x84R`\0\x80Q` a\\\x8C\x839\x81Q\x91R\x90\x94R\x93\x90\x91 \x80T`\x01`\xFF\x19\x90\x91\x16\x17\x90U`\x9C\x80Tc\xFF\xFF\xFF\xFF\x19\x16C\x84\x16\x17\x90UT\x91\x90\x91\x04\x16\x90\x7F\xFA\xF4\xB2\x05Dy\xD0\xF8>\x90\x9Bs\xCD\xE2\xA6\xCB\x18\xEC*\x93\xBA\x8A\xD5\xA6#)\0\x1C\x86\xB1\xF3\xEA\x90a;\x81\x90\x84\x90a[TV[`@Q\x80\x91\x03\x90\xA2`\x97`\x1C\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x7F\xDC[\xA3\xB6n\xC6I\x1BX[?\x13i\x8E\xD7\x11\xAA\x82\x90q\xF43\0\xD6\xED\xE6\xDB\xA6~(\xD7_\x82`@Qa/\xCC\x91\x90a[TV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a<$W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a<H\x91\x90aWJV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a<xW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xBD\x90aWgV[`fT\x19\x81\x19`fT\x19\x16\x14a<\xF6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07\xBDV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x13\x05V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a0\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x07\xBDV[`\x01`\x01`\xA0\x1B\x03\x81\x16a>\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x07\xBDV[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a>\x9FWP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a?!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x07\xBDV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a?d\x82a=\x87V[PPV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[```\0\x80a?\xC8\x84a@\x86V[a\xFF\xFF\x16`\x01`\x01`@\x1B\x03\x81\x11\x15a?\xE3Wa?\xE3aA\xC8V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a@\rW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x80[\x82Q\x82\x10\x80\x15a@%WPa\x01\0\x81\x10[\x15a@|W`\x01\x81\x1B\x93P\x85\x84\x16\x15a@lW\x80`\xF8\x1B\x83\x83\x81Q\x81\x10a@NWa@NaV\xCBV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x81`\x01\x01\x91P[a@u\x81aW/V[\x90Pa@\x14V[P\x90\x94\x93PPPPV[`\0\x80[\x82\x15a@\xB1Wa@\x9B`\x01\x84a\\RV[\x90\x92\x16\x91\x80a@\xA9\x81a\\iV[\x91PPa@\x8AV[\x92\x91PPV[\x82\x80Ta@\xC3\x90aX\x16V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82a@\xE5W`\0\x85UaA+V[\x82`\x1F\x10a@\xFEW\x82\x80\x01`\xFF\x19\x825\x16\x17\x85UaA+V[\x82\x80\x01`\x01\x01\x85U\x82\x15aA+W\x91\x82\x01[\x82\x81\x11\x15aA+W\x825\x82U\x91` \x01\x91\x90`\x01\x01\x90aA\x10V[PaA7\x92\x91PaA;V[P\x90V[[\x80\x82\x11\x15aA7W`\0\x81U`\x01\x01aA<V[`\0`\xE0\x82\x84\x03\x12\x15aAbW`\0\x80\xFD[P\x91\x90PV[`\0``\x82\x84\x03\x12\x15aAbW`\0\x80\xFD[`\0\x80`\x80\x83\x85\x03\x12\x15aA\x8DW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aA\xA3W`\0\x80\xFD[aA\xAF\x85\x82\x86\x01aAPV[\x92PPaA\xBF\x84` \x85\x01aAhV[\x90P\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aB\0WaB\0aA\xC8V[`@R\x90V[`@Qa\x01\0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aB\0WaB\0aA\xC8V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aBQWaBQaA\xC8V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15aBrWaBraA\xC8V[P`\x05\x1B` \x01\x90V[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x04@W`\0\x80\xFD[\x805aB\x99\x81aB|V[\x91\x90PV[`\0\x82`\x1F\x83\x01\x12aB\xAFW`\0\x80\xFD[\x815` aB\xC4aB\xBF\x83aBYV[aB)V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aB\xE3W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aC\x07W\x805aB\xFA\x81aB|V[\x83R\x91\x83\x01\x91\x83\x01aB\xE7V[P\x96\x95PPPPPPV[`\0`@\x82\x84\x03\x12\x15aC$W`\0\x80\xFD[aC,aA\xDEV[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12aCSW`\0\x80\xFD[\x815` aCcaB\xBF\x83aBYV[\x82\x81R`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aC\x82W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aC\x07WaC\x98\x88\x82aC\x12V[\x83R\x91\x83\x01\x91`@\x01aC\x86V[`\0\x82`\x1F\x83\x01\x12aC\xB7W`\0\x80\xFD[aC\xBFaA\xDEV[\x80`@\x84\x01\x85\x81\x11\x15aC\xD1W`\0\x80\xFD[\x84[\x81\x81\x10\x15aC\xEBW\x805\x84R` \x93\x84\x01\x93\x01aC\xD3V[P\x90\x95\x94PPPPPV[`\0`\x80\x82\x84\x03\x12\x15aD\x08W`\0\x80\xFD[aD\x10aA\xDEV[\x90PaD\x1C\x83\x83aC\xA6V[\x81RaD+\x83`@\x84\x01aC\xA6V[` \x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12aDGW`\0\x80\xFD[\x815` aDWaB\xBF\x83aBYV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aDvW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aC\x07W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aD\x99W`\0\x80\x81\xFD[aD\xA7\x89\x86\x83\x8B\x01\x01aB\x9EV[\x84RP\x91\x83\x01\x91\x83\x01aDzV[`\0a\x01\x80\x82\x84\x03\x12\x15aD\xC8W`\0\x80\xFD[aD\xD0aB\x06V[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aD\xE9W`\0\x80\xFD[aD\xF5\x85\x83\x86\x01aB\x9EV[\x83R` \x84\x015\x91P\x80\x82\x11\x15aE\x0BW`\0\x80\xFD[aE\x17\x85\x83\x86\x01aCBV[` \x84\x01R`@\x84\x015\x91P\x80\x82\x11\x15aE0W`\0\x80\xFD[aE<\x85\x83\x86\x01aCBV[`@\x84\x01RaEN\x85``\x86\x01aC\xF6V[``\x84\x01RaE`\x85`\xE0\x86\x01aC\x12V[`\x80\x84\x01Ra\x01 \x84\x015\x91P\x80\x82\x11\x15aEzW`\0\x80\xFD[aE\x86\x85\x83\x86\x01aB\x9EV[`\xA0\x84\x01Ra\x01@\x84\x015\x91P\x80\x82\x11\x15aE\xA0W`\0\x80\xFD[aE\xAC\x85\x83\x86\x01aB\x9EV[`\xC0\x84\x01Ra\x01`\x84\x015\x91P\x80\x82\x11\x15aE\xC6W`\0\x80\xFD[PaE\xD3\x84\x82\x85\x01aD6V[`\xE0\x83\x01RP\x92\x91PPV[`\0\x80`\0\x83\x85\x03`\xE0\x81\x12\x15aE\xF5W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aF\x0CW`\0\x80\xFD[\x90\x86\x01\x90`\xC0\x82\x89\x03\x12\x15aF W`\0\x80\xFD[\x81\x95P`\xA0`\x1F\x19\x84\x01\x12\x15aF5W`\0\x80\xFD[` \x87\x01\x94P`\xC0\x87\x015\x92P\x80\x83\x11\x15aFOW`\0\x80\xFD[PPaF]\x86\x82\x87\x01aD\xB5V[\x91PP\x92P\x92P\x92V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04@W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aF\x8EW`\0\x80\xFD[\x815aF\x99\x81aFgV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15aF\xB2W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15aF\xCCW`\0\x80\xFD[\x825`\x02\x81\x10aF\xDBW`\0\x80\xFD[\x91P` \x83\x015aF\xEB\x81aB|V[\x80\x91PP\x92P\x92\x90PV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15aG\x1CW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01aG\0V[\x81\x81\x11\x15aG.W`\0` \x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0aF\x99` \x83\x01\x84aF\xF6V[\x80\x15\x15\x81\x14a\x04@W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15aG\x81W`\0\x80\xFD[\x885aG\x8C\x81aFgV[\x97P` \x89\x015aG\x9C\x81aFgV[\x96P`@\x89\x015aG\xAC\x81aFgV[\x95P``\x89\x015aG\xBC\x81aFgV[\x94P`\x80\x89\x015aG\xCC\x81aGVV[\x93P`\xA0\x89\x015aG\xDC\x81aFgV[\x92P`\xC0\x89\x015aG\xEC\x81aB|V[\x91P`\xE0\x89\x015aG\xFC\x81aB|V[\x80\x91PP\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[`\0\x80`\0``\x84\x86\x03\x12\x15aH\"W`\0\x80\xFD[\x835aH-\x81aFgV[\x92P` \x84\x81\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aHJW`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12aH^W`\0\x80\xFD[\x815\x81\x81\x11\x15aHpWaHpaA\xC8V[aH\x82`\x1F\x82\x01`\x1F\x19\x16\x85\x01aB)V[\x91P\x80\x82R\x88\x84\x82\x85\x01\x01\x11\x15aH\x98W`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x94PPPPaH\xBB`@\x85\x01aB\x8EV[\x90P\x92P\x92P\x92V[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0\x80[\x86\x81\x10\x15aIZW\x83\x85\x03\x8AR\x82Q\x80Q\x80\x87R\x90\x87\x01\x90\x87\x87\x01\x90\x84[\x81\x81\x10\x15aIEW\x83Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x8A\x81\x01Q\x8B\x85\x01R`@\x90\x81\x01Q`\x01`\x01``\x1B\x03\x16\x90\x84\x01R\x92\x89\x01\x92``\x90\x92\x01\x91`\x01\x01aI\x01V[PP\x9A\x87\x01\x9A\x95PP\x91\x85\x01\x91`\x01\x01aH\xE3V[P\x92\x98\x97PPPPPPPPV[` \x81R`\0aF\x99` \x83\x01\x84aH\xC4V[`\0` \x82\x84\x03\x12\x15aI\x8DW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aI\xA3W`\0\x80\xFD[\x82\x01`@\x81\x85\x03\x12\x15aF\x99W`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12aI\xC7W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aI\xDEW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aI\xF6W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\x80\x87\x89\x03\x12\x15aJ\x16W`\0\x80\xFD[\x865aJ!\x81aFgV[\x95P` \x87\x015aJ1\x81aB|V[\x94P`@\x87\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aJMW`\0\x80\xFD[aJY\x8A\x83\x8B\x01aI\xB5V[\x90\x96P\x94P``\x89\x015\x91P\x80\x82\x11\x15aJrW`\0\x80\xFD[\x81\x89\x01\x91P\x89`\x1F\x83\x01\x12aJ\x86W`\0\x80\xFD[\x815\x81\x81\x11\x15aJ\x95W`\0\x80\xFD[\x8A` \x82`\x05\x1B\x85\x01\x01\x11\x15aJ\xAAW`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92\x95P\x92\x95P\x92\x95V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aJ\xF6W\x81Qc\xFF\xFF\xFF\xFF\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aJ\xD4V[P\x94\x95\x94PPPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0[\x85\x81\x10\x15aKIW\x82\x84\x03\x89RaK7\x84\x83QaJ\xC0V[\x98\x85\x01\x98\x93P\x90\x84\x01\x90`\x01\x01aK\x1FV[P\x91\x97\x96PPPPPPPV[` \x81R`\0\x82Q`\x80` \x84\x01RaKr`\xA0\x84\x01\x82aJ\xC0V[\x90P` \x84\x01Q`\x1F\x19\x80\x85\x84\x03\x01`@\x86\x01RaK\x90\x83\x83aJ\xC0V[\x92P`@\x86\x01Q\x91P\x80\x85\x84\x03\x01``\x86\x01RaK\xAD\x83\x83aJ\xC0V[\x92P``\x86\x01Q\x91P\x80\x85\x84\x03\x01`\x80\x86\x01RPaK\xCB\x82\x82aK\x01V[\x95\x94PPPPPV[`\0\x80`\0`\xA0\x84\x86\x03\x12\x15aK\xE9W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aL\0W`\0\x80\xFD[aL\x0C\x87\x83\x88\x01aAPV[\x94PaL\x1B\x87` \x88\x01aAhV[\x93P`\x80\x86\x015\x91P\x80\x82\x11\x15aL1W`\0\x80\xFD[PaF]\x86\x82\x87\x01aD\xB5V[`\0` \x82\x84\x03\x12\x15aLPW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aLfW`\0\x80\xFD[\x82\x01a\x01 \x81\x85\x03\x12\x15aF\x99W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aL\x8BW`\0\x80\xFD[\x815`\xFF\x81\x16\x81\x14aF\x99W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15aL\xB1W`\0\x80\xFD[\x835aL\xBC\x81aFgV[\x92P` \x84\x81\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aL\xD8W`\0\x80\xFD[\x85\x01`\x1F\x81\x01\x87\x13aL\xE9W`\0\x80\xFD[\x805aL\xF7aB\xBF\x82aBYV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x89\x83\x11\x15aM\x16W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aM4W\x835\x82R\x92\x84\x01\x92\x90\x84\x01\x90aM\x1BV[\x80\x96PPPPPPaH\xBB`@\x85\x01aB\x8EV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aM\x80W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aMdV[P\x90\x96\x95PPPPPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15aM\xA1W`\0\x80\xFD[\x835aM\xAC\x81aB|V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aM\xC7W`\0\x80\xFD[aM\xD3\x86\x82\x87\x01aI\xB5V[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15aM\xF8W`\0\x80\xFD[\x855\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aN\x16W`\0\x80\xFD[aN\"\x89\x83\x8A\x01aI\xB5V[\x90\x96P\x94P`@\x88\x015\x91PaN7\x82aB|V[\x90\x92P``\x87\x015\x90\x80\x82\x11\x15aNMW`\0\x80\xFD[PaNZ\x88\x82\x89\x01aD\xB5V[\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aJ\xF6W\x81Q`\x01`\x01``\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aN{V[`@\x81R`\0\x83Q`@\x80\x84\x01RaN\xBB`\x80\x84\x01\x82aNgV[\x90P` \x85\x01Q`?\x19\x84\x83\x03\x01``\x85\x01RaN\xD8\x82\x82aNgV[\x92PPP\x82` \x83\x01R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x05\x83\x10aO!WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15aO<W`\0\x80\xFD[\x835aOG\x81aFgV[\x92P` \x84\x015\x91P`@\x84\x015aO^\x81aB|V[\x80\x91PP\x92P\x92P\x92V[\x82\x81R`@` \x82\x01R`\0aO\x82`@\x83\x01\x84aH\xC4V[\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15aO\x9CW`\0\x80\xFD[\x815aF\x99\x81aB|V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aO\xBEW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aO\xD8W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15aI\xF6W`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aP\x04W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aP#W`\0\x80\xFD[\x806\x03\x83\x13\x15aI\xF6W`\0\x80\xFD[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x81R`\0\x825aPl\x81aB|V[c\xFF\xFF\xFF\xFF\x81\x16` \x84\x01RP` \x83\x015aP\x87\x81aB|V[c\xFF\xFF\xFF\xFF\x81\x16`@\x84\x01RPaP\xA0`@\x84\x01aB\x8EV[c\xFF\xFF\xFF\xFF\x81\x16``\x84\x01RPaP\xBA``\x84\x01\x84aO\xEDV[`\xE0`\x80\x85\x01RaP\xD0a\x01\0\x85\x01\x82\x84aP2V[\x91PPaP\xDF`\x80\x85\x01aB\x8EV[c\xFF\xFF\xFF\xFF\x81\x16`\xA0\x85\x01RPaP\xF9`\xA0\x85\x01\x85aO\xEDV[\x84\x83\x03`\x1F\x19\x01`\xC0\x86\x01RaQ\x10\x83\x82\x84aP2V[\x92PPPaQ `\xC0\x85\x01aB\x8EV[c\xFF\xFF\xFF\xFF\x81\x16`\xE0\x85\x01R[P\x93\x92PPPV[` \x80\x82R`=\x90\x82\x01R\x7Fsupplied task does not match the`@\x82\x01R\x7F one recorded in the contract\0\0\0``\x82\x01R`\x80\x01\x90V[` \x80\x82R`,\x90\x82\x01R\x7FAggregator has already responded`@\x82\x01Rk to the task`\xA0\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aR\x13WaR\x13aQ\xDEV[\x01\x94\x93PPPPV[` \x80\x82R`-\x90\x82\x01R\x7FAggregator has responded to the `@\x82\x01Rltask too late`\x98\x1B``\x82\x01R`\x80\x01\x90V[\x805aRt\x81aB|V[c\xFF\xFF\xFF\xFF\x16\x82R` \x81\x81\x015\x90\x83\x01R`@\x90\x81\x015\x91\x01RV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Q` \x83\x01R`\0`@\x82\x01Q`\x80`@\x85\x01RaR\xC0`\x80\x85\x01\x82aNgV[\x90P``\x83\x01Q\x84\x82\x03``\x86\x01RaK\xCB\x82\x82aNgV[aR\xE3\x81\x84aRiV[`\x80``\x82\x01R`\0aO\x82`\x80\x83\x01\x84aR\x91V[``\x81\x01a@\xB1\x82\x84aRiV[` \x80\x82R`\x05\x90\x82\x01Rd\x04\x17WF\x83`\xDC\x1B`@\x82\x01R``\x01\x90V[` \x81R`\0\x825aS7\x81aB|V[c\xFF\xFF\xFF\xFF\x80\x82\x16` \x85\x01R` \x85\x015`@\x85\x01R`@\x85\x015\x91PaS^\x82aB|V[\x80\x82\x16``\x85\x01R``\x85\x015\x91PaSv\x82aB|V[\x80\x82\x16`\x80\x85\x01RaS\x8B`\x80\x86\x01\x86aO\xEDV[\x92P`\xC0`\xA0\x86\x01RaS\xA2`\xE0\x86\x01\x84\x83aP2V[\x92PP`\xA0\x85\x015aS\xB3\x81aB|V[\x16`\xC0\x93\x90\x93\x01\x92\x90\x92RP\x91\x90PV[\x805aS\xCF\x81aB|V[c\xFF\xFF\xFF\xFF\x16\x82R` \x81\x81\x015\x90\x83\x01R`@\x80\x82\x015\x90\x83\x01R``\x80\x82\x015\x90\x83\x01R`\x80\x90\x81\x015\x91\x01RV[`\xA0\x81\x01a@\xB1\x82\x84aS\xC4V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aJ\xF6WaT?\x87\x83Q\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[`@\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01aT\"V[\x80`\0[`\x02\x81\x10\x15aTuW\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01aTVV[PPPPV[aT\x86\x82\x82QaTRV[` \x81\x01Qa\x11\x1C`@\x84\x01\x82aTRV[\x85\x81R`\x80` \x82\x01R`\0aT\xB2`\x80\x83\x01\x86\x88aP2V[c\xFF\xFF\xFF\xFF\x85\x16`@\x84\x01R\x82\x81\x03``\x84\x01Ra\x01\x80\x84Q\x81\x83RaT\xDA\x82\x84\x01\x82aJ\xC0V[\x91PP` \x85\x01Q\x82\x82\x03` \x84\x01RaT\xF4\x82\x82aT\x0EV[\x91PP`@\x85\x01Q\x82\x82\x03`@\x84\x01RaU\x0E\x82\x82aT\x0EV[\x91PP``\x85\x01QaU#``\x84\x01\x82aT{V[P`\x80\x85\x01Q\x80Q`\xE0\x84\x01R` \x01Qa\x01\0\x83\x01R`\xA0\x85\x01Q\x82\x82\x03a\x01 \x84\x01RaUR\x82\x82aJ\xC0V[\x91PP`\xC0\x85\x01Q\x82\x82\x03a\x01@\x84\x01RaUm\x82\x82aJ\xC0V[\x91PP`\xE0\x85\x01Q\x82\x82\x03a\x01`\x84\x01RaU\x88\x82\x82aK\x01V[\x9A\x99PPPPPPPPPPV[\x80Q`\x01`\x01``\x1B\x03\x81\x16\x81\x14aB\x99W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12aU\xBEW`\0\x80\xFD[\x81Q` aU\xCEaB\xBF\x83aBYV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aU\xEDW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aC\x07WaV\x02\x81aU\x96V[\x83R\x91\x83\x01\x91\x83\x01aU\xF1V[`\0\x80`@\x83\x85\x03\x12\x15aV\"W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aV9W`\0\x80\xFD[\x90\x84\x01\x90`@\x82\x87\x03\x12\x15aVMW`\0\x80\xFD[aVUaA\xDEV[\x82Q\x82\x81\x11\x15aVdW`\0\x80\xFD[aVp\x88\x82\x86\x01aU\xADV[\x82RP` \x83\x01Q\x82\x81\x11\x15aV\x85W`\0\x80\xFD[aV\x91\x88\x82\x86\x01aU\xADV[` \x83\x01RP\x80\x94PPPP` \x83\x01Q\x90P\x92P\x92\x90PV[aV\xB5\x81\x84aS\xC4V[`\xC0`\xA0\x82\x01R`\0aO\x82`\xC0\x83\x01\x84aR\x91V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aW\x07WaW\x07aQ\xDEV[\x02\x94\x93PPPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15aW*WaW*aQ\xDEV[P\x02\x90V[`\0`\0\x19\x82\x14\x15aWCWaWCaQ\xDEV[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15aW\\W`\0\x80\xFD[\x81QaF\x99\x81aFgV[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15aW\xC3W`\0\x80\xFD[\x81QaF\x99\x81aGVV[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80aX*W`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15aAbWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[`\0` \x80\x83\x85\x03\x12\x15aX^W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aXtW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aX\x85W`\0\x80\xFD[\x80QaX\x93aB\xBF\x82aBYV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15aX\xB2W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aX\xD0W\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90aX\xB7V[\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15aX\xEDW`\0\x80\xFD[aF\x99\x82aU\x96V[c\xFF\xFF\xFF\xFF\x84\x16\x81R`@` \x82\x01\x81\x90R\x81\x01\x82\x90R`\0`\x01`\x01`\xFB\x1B\x03\x83\x11\x15aY#W`\0\x80\xFD[\x82`\x05\x1B\x80\x85``\x85\x017`\0\x92\x01``\x01\x91\x82RP\x93\x92PPPV[`\0` \x80\x83\x85\x03\x12\x15aYSW`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aYiW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aYzW`\0\x80\xFD[\x80QaY\x88aB\xBF\x82aBYV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15aY\xA7W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aX\xD0W\x83QaY\xBF\x81aB|V[\x82R\x92\x84\x01\x92\x90\x84\x01\x90aY\xACV[c\xFF\xFF\xFF\xFF\x84\x16\x81R`@` \x82\x01R`\0aK\xCB`@\x83\x01\x84\x86aP2V[`\0` \x82\x84\x03\x12\x15aZ\0W`\0\x80\xFD[\x81Q`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x14aF\x99W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aZ)W`\0\x80\xFD[\x81QaF\x99\x81aB|V[`\0`\xFF\x82\x16`\xFF\x81\x14\x15aZKWaZKaQ\xDEV[`\x01\x01\x92\x91PPV[`@\x81R`\0aZh`@\x83\x01\x85\x87aP2V[\x90Pc\xFF\xFF\xFF\xFF\x83\x16` \x83\x01R\x94\x93PPPPV[`\0`@\x82\x01c\xFF\xFF\xFF\xFF\x85\x16\x83R` `@\x81\x85\x01R\x81\x85Q\x80\x84R``\x86\x01\x91P\x82\x87\x01\x93P`\0[\x81\x81\x10\x15aZ\xC5W\x84Q\x83R\x93\x83\x01\x93\x91\x83\x01\x91`\x01\x01aZ\xA9V[P\x90\x97\x96PPPPPPPV[` \x80\x82R`9\x90\x82\x01R\x7FCan't create a task in the same `@\x82\x01R\x7Fblock as a completed task\0\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[`\0c\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a[LWa[LaQ\xDEV[\x03\x93\x92PPPV[` \x81R`\0c\xFF\xFF\xFF\xFF\x80\x84Q\x16` \x84\x01R\x80` \x85\x01Q\x16`@\x84\x01R\x80`@\x85\x01Q\x16``\x84\x01RP``\x83\x01Q`\xE0`\x80\x84\x01Ra[\x9Ba\x01\0\x84\x01\x82aF\xF6V[\x90P`\x80\x84\x01Qa[\xB4`\xA0\x85\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P`\xA0\x84\x01Q\x83\x82\x03`\x1F\x19\x01`\xC0\x85\x01Ra[\xD0\x82\x82aF\xF6V[\x91PP`\xC0\x84\x01QaQ-`\xE0\x85\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[` \x81R`\0c\xFF\xFF\xFF\xFF\x80\x84Q\x16` \x84\x01R` \x84\x01Q`@\x84\x01R\x80`@\x85\x01Q\x16``\x84\x01R\x80``\x85\x01Q\x16`\x80\x84\x01R`\x80\x84\x01Q`\xC0`\xA0\x85\x01Ra\\9`\xE0\x85\x01\x82aF\xF6V[\x90P\x81`\xA0\x86\x01Q\x16`\xC0\x85\x01R\x80\x92PPP\x92\x91PPV[`\0\x82\x82\x10\x15a\\dWa\\daQ\xDEV[P\x03\x90V[`\0a\xFF\xFF\x80\x83\x16\x81\x81\x14\x15a\\\x81Wa\\\x81aQ\xDEV[`\x01\x01\x93\x92PPPV\xFE\x10\xAF\xAC\x923\xB4\xCC\xC5Md\x04\xFF\xC1\xCF;GQZ+\x8E\xDB\xF6u\xD1^\xDD\xCE\x05\xA0'\xDC\xBD)\x8C\x80\r\x08\x81\xDD \x8Dp^\xBC\x03\xEB\x18\x18\x9F8\x11\x82Y\xF2}\xD4;L`\xD6\x1C`~\x87#]b\x9D\xC8\x02\x03}\xED\x8Ca\xCB'\xFB)\xE4\x0F\xA0\x1B)\x97\x19\xD8\xF9\x91\xFF\xE2\x0B\xDC\xC5\x9FO\xA2dipfsX\"\x12 \x19W\x16\xB8h\xAEI\xC0\xAE\x9A\xA6}\x15\x85\xE0\xE2Q=\x82x1\x8A\x0B\xD3\x82\xEC\xDFI\x95\x17\r6dsolcC\0\x08\x0C\x003";
    /// The bytecode of the contract.
    pub static FINALIZERTASKMANAGER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\xA0W`\x005`\xE0\x1C\x80c]\xF4YF\x11a\x01gW\x80c\xA6\x95c\xA9\x11a\0\xCEW\x80c\xDF\\\xF7#\x11a\0\x87W\x80c\xDF\\\xF7#\x14a\x06rW\x80c\xE7\x0C&#\x14a\x06zW\x80c\xEF\x02DX\x14a\x06\x8AW\x80c\xF2\xFD\xE3\x8B\x14a\x06\x92W\x80c\xF5d\x0C\xF8\x14a\x06\xA5W\x80c\xFA\xBC\x1C\xBC\x14a\x06\xB8W`\0\x80\xFD[\x80c\xA6\x95c\xA9\x14a\x05\xCCW\x80c\xAD\xFC\xB0H\x14a\x05\xE3W\x80c\xB3\xEA\x18N\x14a\x05\xECW\x80c\xBF#\x15\xED\x14a\x05\xFFW\x80c\xC9\x87\xDE\x8E\x14a\x06:W\x80c\xCE\xFD\xC1\xD4\x14a\x06QW`\0\x80\xFD[\x80cr1\x14\xAB\x11a\x01 W\x80cr1\x14\xAB\x14a\x05[W\x80cz\xFA\x1E\xED\x14a\x05nW\x80cz\xFD\xD5K\x14a\x05\x81W\x80c\x88o\x11\x95\x14a\x05\x91W\x80c\x8D\xA5\xCB[\x14a\x05\xA4W\x80c\x8F\xC8r\x9A\x14a\x05\xB5W`\0\x80\xFD[\x80c]\xF4YF\x14a\x05\x07W\x80ch0H5\x14a\x05\x0FW\x80cm\x14\xA9\x87\x14a\x05\x17W\x80cn\x12_\xF4\x14a\x05\x1FW\x80cn\xFBF6\x14a\x052W\x80cqP\x18\xA6\x14a\x05SW`\0\x80\xFD[\x80cAx\x9DW\x11a\x02\x0BW\x80cSz))\x11a\x01\xC4W\x80cSz))\x14a\x04\x8FW\x80cT\xD1'\xDE\x14a\x04\xA6W\x80cY\\jg\x14a\x04\xB4W\x80cZ\xC8j\xB7\x14a\x04\xBCW\x80c\\\x15Vb\x14a\x04\xDFW\x80c\\\x97Z\xBB\x14a\x04\xFFW`\0\x80\xFD[\x80cAx\x9DW\x14a\x04\x06W\x80cE&[z\x14a\x042W\x80cJ\xE6\xB2\x03\x14a\x04CW\x80cMzq\x16\x14a\x04LW\x80cOs\x9Ft\x14a\x04\\W\x80cQjr'\x14a\x04|W`\0\x80\xFD[\x80c\x1A\xC2r\x97\x11a\x02]W\x80c\x1A\xC2r\x97\x14a\x03NW\x80c\x1C\x17\x8E\x9C\x14a\x03yW\x80c$Z{\xFC\x14a\x03\xA4W\x80c(0\xE8\xF9\x14a\x03\xBEW\x80c-\xB2\xE3\x9B\x14a\x03\xD3W\x80c5c\xB0\xD1\x14a\x03\xE6W`\0\x80\xFD[\x80c\x01\xA3\xF0\x13\x14a\x02\xA5W\x80c\x05\xC6\xB6c\x14a\x02\xBAW\x80c\x0E\xE0\xFD\xBD\x14a\x02\xCDW\x80c\x10\xD6z/\x14a\x02\xEFW\x80c\x13d9\xDD\x14a\x03\x02W\x80c\x13\xF8\x15\xED\x14a\x03\x15W[`\0\x80\xFD[a\x02\xB8a\x02\xB36`\x04aAzV[a\x06\xCBV[\0[a\x02\xB8a\x02\xC86`\x04aE\xDFV[a\x0B\\V[`\xA2Ta\x02\xDA\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\xB8a\x02\xFD6`\x04aF|V[a\x11!V[a\x02\xB8a\x03\x106`\x04aF\xA0V[a\x11\xD1V[a\x03@a\x03#6`\x04aF\xB9V[`\x99` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[`@Q\x90\x81R` \x01a\x02\xE6V[a\x03@a\x03\\6`\x04aF\xB9V[`\x9A` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[`\x97Ta\x03\x8C\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xE6V[`\x9ETa\x03\x8C\x90`\x01` \x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03\xC6a\x13\x10V[`@Qa\x02\xE6\x91\x90aGCV[a\x02\xB8a\x03\xE16`\x04aGdV[a\x13\x9EV[a\x03\xF9a\x03\xF46`\x04aH\rV[a\x15WV[`@Qa\x02\xE6\x91\x90aIhV[`\x97Ta\x04\x1D\x90`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\xE6V[a\x02\xB8a\x04@6`\x04aI{V[PV[a\x03@`\xA0T\x81V[`\x9CTa\x04\x1D\x90c\xFF\xFF\xFF\xFF\x16\x81V[a\x04oa\x04j6`\x04aI\xFDV[a\x19\xEDV[`@Qa\x02\xE6\x91\x90aKVV[a\x02\xB8a\x04\x8A6`\x04aK\xD4V[a!\x13V[`\x9CTa\x04\x1D\x90`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x02\xB8a\x04@6`\x04aL>V[a\x02\xB8a(\x0BV[a\x02\xDAa\x04\xCA6`\x04aLyV[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[a\x04\xF2a\x04\xED6`\x04aL\x9CV[a(\xD2V[`@Qa\x02\xE6\x91\x90aMHV[`fTa\x03@V[a\x03\x8Ca*\x9AV[a\x03\x8Ca+\rV[a\x03\x8Ca+WV[a\x02\xB8a\x05-6`\x04aM\x8CV[a+\xA1V[a\x05Ea\x05@6`\x04aM\xE0V[a0\x15V[`@Qa\x02\xE6\x92\x91\x90aN\xA0V[a\x02\xB8a0\xB5V[a\x02\xB8a\x05i6`\x04aF|V[a0\xC9V[`\x9FTa\x03\x8C\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x98Ta\x04\x1D\x90c\xFF\xFF\xFF\xFF\x16\x81V[`eTa\x03\x8C\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x03\x8CV[`\x9CTa\x04\x1D\x90`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`\x97Ta\x04\x1D\x90`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x03@`\xA1T\x81V[a\x02\xB8a\x05\xFA6`\x04aF\xA0V[a1%V[a\x06-a\x06\r6`\x04aF\xB9V[`\x9B` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[`@Qa\x02\xE6\x91\x90aN\xFFV[`\x97Ta\x04\x1D\x90`\x01`\xC0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x06da\x06_6`\x04aO'V[a5>V[`@Qa\x02\xE6\x92\x91\x90aOiV[a\x03\x8Ca6\xD0V[`\x9ETa\x04\x1D\x90c\xFF\xFF\xFF\xFF\x16\x81V[a\x03@`d\x81V[a\x02\xB8a\x06\xA06`\x04aF|V[a7\x1AV[a\x02\xB8a\x06\xB36`\x04aM\x8CV[a7\x90V[a\x02\xB8a\x06\xC66`\x04aF\xA0V[a;\xD1V[a\x06\xD3a=-V[`\0a\x06\xE2` \x83\x01\x83aO\x8AV[\x90P`\0a\x06\xF6``\x85\x01`@\x86\x01aO\x8AV[\x90P`\0a\x07\n`@\x86\x01` \x87\x01aO\x8AV[\x90P6`\0a\x07\x1C`\xA0\x88\x01\x88aO\xA7V[\x90\x92P\x90P`\0a\x073`\xE0\x89\x01`\xC0\x8A\x01aO\x8AV[`\0\x80\x80R`\x99` \x90\x81R\x91\x92P`\0\x80Q` a\\\xCC\x839\x81Q\x91R\x91a\x07^\x90\x8A\x01\x8AaO\x8AV[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x88`@Q` \x01a\x07\x8A\x91\x90aP[V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x14a\x07\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xBD\x90aQ5V[`@Q\x80\x91\x03\x90\xFD[`\0\x80\x80R`\x9B` \x90\x81R`\x01\x91`\0\x80Q` a\\\x8C\x839\x81Q\x91R\x91a\x07\xF1\x90\x8B\x01\x8BaO\x8AV[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16`\x04\x81\x11\x15a\x08\x1CWa\x08\x1CaN\xE9V[\x14a\x089W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xBD\x90aQ\x92V[`\0\x80\x80R`\x9A` \x90\x81R\x7F\xBEf \xBD3F\xE5\xD7\xF88\x7F\xBE\xC0\x98\x1A\xA0\xD6(\x9D\"\xEF\xA7\xC95\xF9\xEFhA\xBF*\x98\xC7\x90\x82\x90a\x08u\x90\x8B\x01\x8BaO\x8AV[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x14a\x08\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xBD\x90aQ\x92V[`\x97Ta\x08\xC7\x90`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x85aQ\xF4V[c\xFF\xFF\xFF\xFF\x16Cc\xFF\xFF\xFF\xFF\x16\x11\x15a\x08\xF2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xBD\x90aR\x1CV[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`@\x80Q`\x80\x81\x01\x82Rc\xFF\xFF\xFF\xFFC\x16\x81R`\0` \x80\x83\x01\x82\x90R\x84\x81\x01Q\x83\x85\x01R\x84Q``\x84\x01R\x92Q\x90\x92a\tD\x91\x8C\x91\x84\x91\x01aR\xD9V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x91\x82\x01 `\0\x80\x80R`\x9A\x83R\x90\x91\x7F\xBEf \xBD3F\xE5\xD7\xF88\x7F\xBE\xC0\x98\x1A\xA0\xD6(\x9D\"\xEF\xA7\xC95\xF9\xEFhA\xBF*\x98\xC7\x91\x90a\t\x96\x90\x8E\x01\x8EaO\x8AV[c\xFF\xFF\xFF\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 \x93\x90\x93U\x8C\x015`\xA1U\x81\x80R`\x9B\x81R`\x04\x91`\0\x80Q` a\\\x8C\x839\x81Q\x91R\x91a\t\xE1\x90\x8E\x01\x8EaO\x8AV[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16`\x04\x81\x11\x15a\n\x0CWa\n\x0CaN\xE9V[Pa\n\x1F\x90P`@\x8C\x01` \x8D\x01aO\x8AV[`\x9C\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x01`@\x1B\x02c\xFF\xFF\xFF\xFF`@\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90Ua\nQ``\x8C\x01\x8CaO\xA7V[a\n]\x91`\x9D\x91a@\xB7V[Pa\nn`\xA0\x8C\x01`\x80\x8D\x01aO\x8AV[`\x9E\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\n\x95` \x8C\x01\x8CaO\x8AV[`\x9C\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90Ua\n\xC8` \x8B\x01\x8BaO\x8AV[c\xFF\xFF\xFF\xFF\x16\x7F\xFF)\x08H=t\xB6\xB7\0S\xDDG2`\xAC\xF1\xB0\x9E\x0B\xA0x\x1B\xF9A\0\xBB\x82wX\x17I\xDE\x8B`@Qa\n\xFD\x91\x90aR\xF9V[`@Q\x80\x91\x03\x90\xA2a\x0B\x12` \x8B\x01\x8BaO\x8AV[c\xFF\xFF\xFF\xFF\x16\x7F\xDF\"\xF3U\x8EHA\xB6=w\x17\x95F\xB3\xEA\xE6>N4;\xBEu'F\xB0\x93\x16+\xC5&\xBEL\x8B`@Qa\x0BG\x91\x90aR\xF9V[`@Q\x80\x91\x03\x90\xA2PPPPPPPPPPPV[`\x9ET`\x01` \x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0B\x8DW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xBD\x90aS\x07V[`\0a\x0B\x9F`\x80\x85\x01``\x86\x01aO\x8AV[\x90P`\0a\x0B\xB3``\x86\x01`@\x87\x01aO\x8AV[\x90P6`\0a\x0B\xC5`\x80\x88\x01\x88aO\xA7V[\x90\x92P\x90P`\0a\x0B\xDC`\xC0\x89\x01`\xA0\x8A\x01aO\x8AV[`\x01`\0\x90\x81R`\x99` \x90\x81R\x91\x92P\x7F\xBB\x86\xFB\xC04\xF4\xE3\x82\x92\x99t\xBC\xD8A\x9E\xD6&\xB0\xEAd\x7F\x96-\x89\xBA/\xB6\xBD(xZ\xB9\x91a\x0C\x1B\x90\x8A\x01\x8AaO\x8AV[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x88`@Q` \x01a\x0CG\x91\x90aS&V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x14a\x0CzW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xBD\x90aQ5V[`\x01`\0\x81\x81R`\x9B` \x90\x81R`\0\x80Q` a\\\xAC\x839\x81Q\x91R\x91\x90a\x0C\xA5\x90\x8B\x01\x8BaO\x8AV[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16`\x04\x81\x11\x15a\x0C\xD0Wa\x0C\xD0aN\xE9V[\x14a\x0C\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xBD\x90aQ\x92V[`\x01`\0\x90\x81R`\x9A` \x90\x81R\x7F[T+R\x98\x1CO/\xA9\x96U\x14\xD5\xBB\x7F7\xF1\xB7\xBC\t\x02\xA6\xA4\xDCk\x04\xDC\x05\xBE\x85Xk\x90\x82\x90a\r+\x90\x8B\x01\x8BaO\x8AV[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x14a\rcW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xBD\x90aQ\x92V[`\x97Ta\r}\x90`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x85aQ\xF4V[c\xFF\xFF\xFF\xFF\x16Cc\xFF\xFF\xFF\xFF\x16\x11\x15a\r\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xBD\x90aR\x1CV[`\0\x87`@Q` \x01a\r\xBB\x91\x90aT\0V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x80`\x97`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cn\xFBF6\x84\x88\x88\x8C\x8E`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\x1F\x95\x94\x93\x92\x91\x90aT\x98V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E<W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0Ed\x91\x90\x81\x01\x90aV\x0FV[`@\x80Q`\x80\x81\x01\x82Rc\xFF\xFF\xFF\xFFC\x16\x81R` \x80\x82\x01\x84\x90R\x80\x85\x01Q\x82\x84\x01R\x84Q``\x83\x01R\x91Q\x93\x95P\x91\x93P\x90\x91a\x0E\xA6\x91\x8D\x91\x84\x91\x01aV\xABV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x91\x82\x01 `\x01`\0\x90\x81R`\x9A\x83R\x90\x91\x7F[T+R\x98\x1CO/\xA9\x96U\x14\xD5\xBB\x7F7\xF1\xB7\xBC\t\x02\xA6\xA4\xDCk\x04\xDC\x05\xBE\x85Xk\x91\x90a\x0E\xFA\x90\x8F\x01\x8FaO\x8AV[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 U`\x03`\x9B`\0`\x01\x80\x81\x11\x15a\x0F)Wa\x0F)aN\xE9V[\x81R` \x01\x90\x81R` \x01`\0 `\0\x8D`\0\x01` \x81\x01\x90a\x0FL\x91\x90aO\x8AV[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16`\x04\x81\x11\x15a\x0FwWa\x0FwaN\xE9V[Pa\x0F\x87\x90P` \x8D\x01\x8DaO\x8AV[c\xFF\xFF\xFF\xFF\x16\x7F\x07\xBA\x8D|B\t\xC1Nm.\xD5\xFDEB\xE9\xD8\xF4~\x1A\xF3\xDA\x0C\xBD\x9CP\xE2\xD0\xD3\xE8}\xFF\xF0\x8C\x83`@Qa\x0F\xBE\x92\x91\x90aV\xABV[`@Q\x80\x91\x03\x90\xA2`\0[\x86\x81\x10\x15a\x10`W\x85`\xFF\x16\x84` \x01Q\x82\x81Q\x81\x10a\x0F\xEBWa\x0F\xEBaV\xCBV[` \x02` \x01\x01Qa\x0F\xFD\x91\x90aV\xE1V[`\x01`\x01``\x1B\x03\x16`d\x85`\0\x01Q\x83\x81Q\x81\x10a\x10\x1EWa\x10\x1EaV\xCBV[` \x02` \x01\x01Q`\x01`\x01``\x1B\x03\x16a\x109\x91\x90aW\x10V[\x10\x15a\x10NWPPPPPPPPPPPPPV[\x80a\x10X\x81aW/V[\x91PPa\x0F\xC9V[P`\x80\x8B\x015`\xA0U`\x01`\0\x90\x81R`\x9B` \x90\x81R`\x04\x91`\0\x80Q` a\\\xAC\x839\x81Q\x91R\x91a\x10\x96\x90\x8F\x01\x8FaO\x8AV[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16`\x04\x81\x11\x15a\x10\xC1Wa\x10\xC1aN\xE9V[PP`@\x8B\x015a\x10\xD5` \x8D\x01\x8DaO\x8AV[c\xFF\xFF\xFF\xFF\x16\x7F\x02\x04gs\xDF\xD0\xE0\"\xABk\xF6\x8C\xB0}s\xF0\x80\xF8zK\x82\xF7\xA4\xFC\xCF\xE7\xD9\x19\xD6\xC4\xEC\xB4\x8D`@Qa\x11\n\x91\x90aT\0V[`@Q\x80\x91\x03\x90\xA3PPPPPPPPP[PPPV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11tW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x98\x91\x90aWJV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x11\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xBD\x90aWgV[a\x04@\x81a=\x87V[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\x19W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12=\x91\x90aW\xB1V[a\x12YW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xBD\x90aW\xCEV[`fT\x81\x81\x16\x14a\x12\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07\xBDV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`\x9D\x80Ta\x13\x1D\x90aX\x16V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x13I\x90aX\x16V[\x80\x15a\x13\x96W\x80`\x1F\x10a\x13kWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x13\x96V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x13yW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x13\xBEWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x13\xD8WP0;\x15\x80\x15a\x13\xD8WP`\0T`\xFF\x16`\x01\x14[a\x14;W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x07\xBDV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x14^W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x14i\x89`\0a>~V[a\x14r\x88a?hV[`\x9E\x80T`\x01`\x01`\xA0\x1B\x03\x80\x8A\x16`\x01` \x1B\x02d\x01\0\0\0\0`\x01`\xC0\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90\x91U`\x9F\x80T\x88\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x90\x91\x16\x17\x90U`\xA2\x80T\x87\x15\x15`\xFF\x19\x90\x91\x16\x17\x90U`\x97\x80Tc\xFF\xFF\xFF\xFF\x85\x81\x16`\x01`\xC0\x1B\x02c\xFF\xFF\xFF\xFF`\xC0\x1B\x19\x91\x88\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xC0\x1B\x03\x19\x90\x93\x16\x94\x89\x16\x94\x90\x94\x17\x91\x90\x91\x17\x16\x91\x90\x91\x17\x90U\x80\x15a\x15LW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPPPV[```\0\x84`\x01`\x01`\xA0\x1B\x03\x16ch0H5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\x99W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xBD\x91\x90aWJV[\x90P`\0\x85`\x01`\x01`\xA0\x1B\x03\x16c\x9E\x99#\xC2`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\xFFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16#\x91\x90aWJV[\x90P`\0\x86`\x01`\x01`\xA0\x1B\x03\x16c]\xF4YF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16eW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\x89\x91\x90aWJV[\x90P`\0\x86Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16\xA6Wa\x16\xA6aA\xC8V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x16\xD9W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x16\xC4W\x90P[P\x90P`\0[\x87Q\x81\x10\x15a\x19\xE1W`\0\x88\x82\x81Q\x81\x10a\x16\xFCWa\x16\xFCaV\xCBV[\x01` \x01Q`@Qc\x89\x02bE`\xE0\x1B\x81R`\xF8\x91\x90\x91\x1C`\x04\x82\x01\x81\x90Rc\xFF\xFF\xFF\xFF\x8A\x16`$\x83\x01R\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x87\x16\x90c\x89\x02bE\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17]W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x17\x85\x91\x90\x81\x01\x90aXKV[\x90P\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17\xA0Wa\x17\xA0aA\xC8V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x17\xEBW\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a\x17\xBEW\x90P[P\x84\x84\x81Q\x81\x10a\x17\xFEWa\x17\xFEaV\xCBV[` \x02` \x01\x01\x81\x90RP`\0[\x81Q\x81\x10\x15a\x19\xCBW`@Q\x80``\x01`@R\x80\x87`\x01`\x01`\xA0\x1B\x03\x16cG\xB3\x14\xE8\x85\x85\x81Q\x81\x10a\x18AWa\x18AaV\xCBV[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18g\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\x84W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\xA8\x91\x90aWJV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x83\x81Q\x81\x10a\x18\xC8Wa\x18\xC8aV\xCBV[` \x02` \x01\x01Q\x81R` \x01\x89`\x01`\x01`\xA0\x1B\x03\x16c\xFA(\xC6'\x85\x85\x81Q\x81\x10a\x18\xF6Wa\x18\xF6aV\xCBV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x04\x81\x01\x91\x90\x91R`\xFF\x88\x16`$\x82\x01Rc\xFF\xFF\xFF\xFF\x8F\x16`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19RW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19v\x91\x90aX\xDBV[`\x01`\x01``\x1B\x03\x16\x81RP\x85\x85\x81Q\x81\x10a\x19\x94Wa\x19\x94aV\xCBV[` \x02` \x01\x01Q\x82\x81Q\x81\x10a\x19\xADWa\x19\xADaV\xCBV[` \x02` \x01\x01\x81\x90RP\x80\x80a\x19\xC3\x90aW/V[\x91PPa\x18\x0CV[PPP\x80\x80a\x19\xD9\x90aW/V[\x91PPa\x16\xDFV[P\x97\x96PPPPPPPV[a\x1A\x18`@Q\x80`\x80\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`\0\x87`\x01`\x01`\xA0\x1B\x03\x16ch0H5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1AXW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A|\x91\x90aWJV[\x90Pa\x1A\xA9`@Q\x80`\x80\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`@Qca\xC8\xA1/`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90c\xC3\x91B^\x90a\x1A\xD9\x90\x8B\x90\x89\x90\x89\x90`\x04\x01aX\xF6V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\xF6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1B\x1E\x91\x90\x81\x01\x90aY@V[\x81R`@Qc@\xE0:\x81`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\x81\xC0u\x02\x90a\x1BP\x90\x8B\x90\x8B\x90\x8B\x90`\x04\x01aY\xCEV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1BmW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1B\x95\x91\x90\x81\x01\x90aY@V[`@\x82\x01R\x85`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1B\xB2Wa\x1B\xB2aA\xC8V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1B\xE5W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1B\xD0W\x90P[P``\x82\x01R`\0[`\xFF\x81\x16\x87\x11\x15a $W`\0\x85`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1C\x13Wa\x1C\x13aA\xC8V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1C<W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x83``\x01Q\x83`\xFF\x16\x81Q\x81\x10a\x1CVWa\x1CVaV\xCBV[` \x02` \x01\x01\x81\x90RP`\0[\x86\x81\x10\x15a\x1F$W`\0\x8C`\x01`\x01`\xA0\x1B\x03\x16c\x04\xECcQ\x8A\x8A\x85\x81\x81\x10a\x1C\x8FWa\x1C\x8FaV\xCBV[\x90P` \x02\x015\x8E\x88`\0\x01Q\x86\x81Q\x81\x10a\x1C\xADWa\x1C\xADaV\xCBV[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1C\xEA\x93\x92\x91\x90\x92\x83Rc\xFF\xFF\xFF\xFF\x91\x82\x16` \x84\x01R\x16`@\x82\x01R``\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\x07W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D+\x91\x90aY\xEEV[\x90P`\x01`\x01`\xC0\x1B\x03\x81\x16a\x1D\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R\x7FOperatorStateRetriever.getCheckS`D\x82\x01R\x7FignaturesIndices: operator must `d\x82\x01R\x7Fbe registered at blocknumber\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x07\xBDV[\x8A\x8A\x85`\xFF\x16\x81\x81\x10a\x1D\xE4Wa\x1D\xE4aV\xCBV[`\x01`\x01`\xC0\x1B\x03\x84\x16\x92\x015`\xF8\x1C\x91\x90\x91\x1C`\x01\x90\x81\x16\x14\x15\x90Pa\x1F\x11W\x85`\x01`\x01`\xA0\x1B\x03\x16c\xDD\x98F\xB9\x8A\x8A\x85\x81\x81\x10a\x1E&Wa\x1E&aV\xCBV[\x90P` \x02\x015\x8D\x8D\x88`\xFF\x16\x81\x81\x10a\x1EBWa\x1EBaV\xCBV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x87\x90\x1B\x16\x81R`\x04\x81\x01\x94\x90\x94R\x91\x90\x91\x015`\xF8\x1C`$\x83\x01RPc\xFF\xFF\xFF\xFF\x8F\x16`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\x98W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\xBC\x91\x90aZ\x17V[\x85``\x01Q\x85`\xFF\x16\x81Q\x81\x10a\x1E\xD5Wa\x1E\xD5aV\xCBV[` \x02` \x01\x01Q\x84\x81Q\x81\x10a\x1E\xEEWa\x1E\xEEaV\xCBV[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x82a\x1F\r\x81aW/V[\x93PP[P\x80a\x1F\x1C\x81aW/V[\x91PPa\x1CdV[P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F?Wa\x1F?aA\xC8V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1FhW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a\x1F\xE9W\x84``\x01Q\x84`\xFF\x16\x81Q\x81\x10a\x1F\x8FWa\x1F\x8FaV\xCBV[` \x02` \x01\x01Q\x81\x81Q\x81\x10a\x1F\xA8Wa\x1F\xA8aV\xCBV[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a\x1F\xC2Wa\x1F\xC2aV\xCBV[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x1F\xE1\x81aW/V[\x91PPa\x1FnV[P\x80\x84``\x01Q\x84`\xFF\x16\x81Q\x81\x10a \x04Wa \x04aV\xCBV[` \x02` \x01\x01\x81\x90RPPP\x80\x80a \x1C\x90aZ4V[\x91PPa\x1B\xEEV[P`\0\x89`\x01`\x01`\xA0\x1B\x03\x16c]\xF4YF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a eW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \x89\x91\x90aWJV[`@Qc5IR\xA3`\xE2\x1B\x81R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xD5%J\x8C\x90a \xBC\x90\x8B\x90\x8B\x90\x8E\x90`\x04\x01aZTV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a \xD9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra!\x01\x91\x90\x81\x01\x90aY@V[` \x83\x01RP\x98\x97PPPPPPPPV[`\x9CT`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15`\0a!6``\x86\x01`@\x87\x01aO\x8AV[\x90P\x81\x15a!\xC1W`\xA2T`\xFF\x16\x15a!\x7FW`\x9ET`\x01` \x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x163\x14a!zW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xBD\x90aS\x07V[a!\xF2V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a!zW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd \xBA\xBA4\x19`\xD9\x1B`D\x82\x01R`d\x01a\x07\xBDV[`\x9ET`\x01` \x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x163\x14a!\xF2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xBD\x90aS\x07V[`\0a\"\x04`@\x87\x01` \x88\x01aO\x8AV[\x90P6`\0a\"\x16`\xA0\x89\x01\x89aO\xA7V[\x90\x92P\x90P`\0a\"-`\xE0\x8A\x01`\xC0\x8B\x01aO\x8AV[`\0\x80\x80R`\x99` \x90\x81R\x91\x92P`\0\x80Q` a\\\xCC\x839\x81Q\x91R\x91a\"X\x90\x8B\x01\x8BaO\x8AV[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x89`@Q` \x01a\"\x84\x91\x90aP[V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x14a\"\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xBD\x90aQ5V[`\0\x80\x80R`\x9B` \x90\x81R`\x01\x91`\0\x80Q` a\\\x8C\x839\x81Q\x91R\x91a\"\xE2\x90\x8C\x01\x8CaO\x8AV[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16`\x04\x81\x11\x15a#\rWa#\raN\xE9V[\x14a#*W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xBD\x90aQ\x92V[`\0\x80\x80R`\x9A` \x90\x81R\x7F\xBEf \xBD3F\xE5\xD7\xF88\x7F\xBE\xC0\x98\x1A\xA0\xD6(\x9D\"\xEF\xA7\xC95\xF9\xEFhA\xBF*\x98\xC7\x90\x82\x90a#f\x90\x8C\x01\x8CaO\x8AV[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x14a#\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xBD\x90aQ\x92V[`\x97Ta#\xB8\x90`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x85aQ\xF4V[c\xFF\xFF\xFF\xFF\x16Cc\xFF\xFF\xFF\xFF\x16\x11\x15a#\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xBD\x90aR\x1CV[`\0\x88`@Q` \x01a#\xF6\x91\x90aR\xF9V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x83\x83\x01\x90\x92R``\x80\x84R\x90\x83\x01R\x91P`\0\x88\x15a$\xBBW`\x97`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cn\xFBF6\x84\x88\x88\x8C\x8F`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a$p\x95\x94\x93\x92\x91\x90aT\x98V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a$\x8DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra$\xB5\x91\x90\x81\x01\x90aV\x0FV[\x90\x92P\x90P[`@\x80Q`\x80\x81\x01\x82Rc\xFF\xFF\xFF\xFFC\x16\x81R` \x80\x82\x01\x84\x90R\x84\x81\x01Q\x82\x84\x01R\x84Q``\x83\x01R\x91Q\x90\x91a$\xF7\x91\x8E\x91\x84\x91\x01aR\xD9V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 `\x9A`\0\x80\x81R` \x01\x90\x81R` \x01`\0 `\0\x8E`\0\x01` \x81\x01\x90a%7\x91\x90aO\x8AV[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 U`\x03`\x9B`\0\x80`\x01\x81\x11\x15a%fWa%faN\xE9V[\x81R` \x01\x90\x81R` \x01`\0 `\0\x8E`\0\x01` \x81\x01\x90a%\x89\x91\x90aO\x8AV[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16`\x04\x81\x11\x15a%\xB4Wa%\xB4aN\xE9V[Pa%\xC4\x90P` \x8E\x01\x8EaO\x8AV[c\xFF\xFF\xFF\xFF\x16\x7FG\xAD\xAC\xB0\xB6\xBB\xD7&\xAE9\xACl\0l\xCA\x1C \x06\xC9\xAE\xDA\xA8\x82\xDC\xBA|H\x04\xDB|A\xCE\x8D\x83`@Qa%\xFB\x92\x91\x90aR\xD9V[`@Q\x80\x91\x03\x90\xA2\x89\x15a&\xA6W`\0[\x86\x81\x10\x15a&\xA4W\x85`\xFF\x16\x84` \x01Q\x82\x81Q\x81\x10a&.Wa&.aV\xCBV[` \x02` \x01\x01Qa&@\x91\x90aV\xE1V[`\x01`\x01``\x1B\x03\x16`d\x85`\0\x01Q\x83\x81Q\x81\x10a&aWa&aaV\xCBV[` \x02` \x01\x01Q`\x01`\x01``\x1B\x03\x16a&|\x91\x90aW\x10V[\x10\x15a&\x92WPPPPPPPPPPPPPPV[\x80a&\x9C\x81aW/V[\x91PPa&\x0CV[P[`@\x8C\x015`\xA1U`\x04`\x9B`\0\x80\x81R` \x01\x90\x81R` \x01`\0 `\0\x8E`\0\x01` \x81\x01\x90a&\xD8\x91\x90aO\x8AV[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16`\x04\x81\x11\x15a'\x03Wa'\x03aN\xE9V[Pa'\x16\x90P`@\x8E\x01` \x8F\x01aO\x8AV[`\x9C\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x01`@\x1B\x02c\xFF\xFF\xFF\xFF`@\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90Ua'H``\x8E\x01\x8EaO\xA7V[a'T\x91`\x9D\x91a@\xB7V[Pa'e`\xA0\x8E\x01`\x80\x8F\x01aO\x8AV[`\x9E\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua'\x8C` \x8E\x01\x8EaO\x8AV[`\x9C\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90Ua'\xBF` \x8D\x01\x8DaO\x8AV[c\xFF\xFF\xFF\xFF\x16\x7F\xFF)\x08H=t\xB6\xB7\0S\xDDG2`\xAC\xF1\xB0\x9E\x0B\xA0x\x1B\xF9A\0\xBB\x82wX\x17I\xDE\x8D`@Qa'\xF4\x91\x90aR\xF9V[`@Q\x80\x91\x03\x90\xA2PPPPPPPPPPPPPV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a(SW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(w\x91\x90aW\xB1V[a(\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xBD\x90aW\xCEV[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[```\0\x84`\x01`\x01`\xA0\x1B\x03\x16c\xC3\x91B^\x84\x86`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a)\x04\x92\x91\x90aZ~V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a)!W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra)I\x91\x90\x81\x01\x90aY@V[\x90P`\0\x84Q`\x01`\x01`@\x1B\x03\x81\x11\x15a)fWa)faA\xC8V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a)\x8FW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x85Q\x81\x10\x15a*\x90W\x86`\x01`\x01`\xA0\x1B\x03\x16c\x04\xECcQ\x87\x83\x81Q\x81\x10a)\xBFWa)\xBFaV\xCBV[` \x02` \x01\x01Q\x87\x86\x85\x81Q\x81\x10a)\xDAWa)\xDAaV\xCBV[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a*\x17\x93\x92\x91\x90\x92\x83Rc\xFF\xFF\xFF\xFF\x91\x82\x16` \x84\x01R\x16`@\x82\x01R``\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a*4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*X\x91\x90aY\xEEV[`\x01`\x01`\xC0\x1B\x03\x16\x82\x82\x81Q\x81\x10a*sWa*saV\xCBV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a*\x88\x81aW/V[\x91PPa)\x95V[P\x95\x94PPPPPV[`\x97T`@\x80Qc.\xFA,\xA3`\xE1\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c]\xF4YF\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a*\xE4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+\x08\x91\x90aWJV[\x90P\x90V[`\x97T`@\x80Qch0H5`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91ch0H5\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a*\xE4W=`\0\x80>=`\0\xFD[`\x97T`@\x80Qcm\x14\xA9\x87`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cm\x14\xA9\x87\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a*\xE4W=`\0\x80>=`\0\xFD[`\x9FT`\x01`\x01`\xA0\x1B\x03\x163\x14a+\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdAuth1`\xD8\x1B`D\x82\x01R`d\x01a\x07\xBDV[`\x9CTc\xFF\xFF\xFF\xFF`\x01`@\x1B\x90\x91\x04\x16C\x14\x80\x15\x90a,\x02WPC\x15\x15[a,\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xBD\x90aZ\xD2V[`@\x80Q`\xE0\x81\x01\x82R`\0\x81\x83\x01\x81\x90R``\x80\x83\x01\x81\x90R`\xA0\x83\x01R`\xC0\x82\x01R`\x97T`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x82RC\x81\x16` \x80\x84\x01\x91\x90\x91R\x90\x86\x16`\x80\x83\x01R\x82Q`\x1F\x85\x01\x82\x90\x04\x82\x02\x81\x01\x82\x01\x90\x93R\x83\x83R\x90\x91\x90\x84\x90\x84\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPPP``\x82\x01R`\x9CT`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16a-\rWc\xFF\xFF\xFF\xFFC\x16`@\x80\x83\x01\x91\x90\x91R\x80Q` `\x1F\x85\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x83\x81R\x90\x84\x90\x84\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPPP`\xA0\x82\x01Rc\xFF\xFF\xFF\xFF\x84\x16`\xC0\x82\x01Ra-\xC1V[`\x9CT`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16`@\x82\x01R`\x9D\x80Ta-/\x90aX\x16V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta-[\x90aX\x16V[\x80\x15a-\xA8W\x80`\x1F\x10a-}Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a-\xA8V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a-\x8BW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`\xA0\x82\x01R`\x9ETc\xFF\xFF\xFF\xFF\x16`\xC0\x82\x01R[`\x97T`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15a.hW`\x97T`\0\x90a-\xF5\x90`\x01\x90`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16a[/V[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\0\x80Q` a\\\x8C\x839\x81Q\x91R` R`@\x90 T\x90\x91P`\x01\x90`\xFF\x16`\x04\x81\x11\x15a.2Wa.2aN\xE9V[\x14\x15a.fWc\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\0\x80Q` a\\\x8C\x839\x81Q\x91R` R`@\x90 \x80T`\xFF\x19\x16`\x02\x17\x90U[P[`\x98Tc\xFF\xFF\xFF\xFF\x16\x15a/\x01W`\x98T`\0\x90a.\x8E\x90`\x01\x90c\xFF\xFF\xFF\xFF\x16a[/V[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\0\x80Q` a\\\xAC\x839\x81Q\x91R` R`@\x90 T\x90\x91P`\x01\x90`\xFF\x16`\x04\x81\x11\x15a.\xCBWa.\xCBaN\xE9V[\x14\x15a.\xFFWc\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\0\x80Q` a\\\xAC\x839\x81Q\x91R` R`@\x90 \x80T`\xFF\x19\x16`\x02\x17\x90U[P[\x80`@Q` \x01a/\x12\x91\x90a[TV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 `\x97\x80Tc\xFF\xFF\xFF\xFF`\x01`\xE0\x1B\x91\x82\x90\x04\x81\x16`\0\x90\x81R`\0\x80Q` a\\\xCC\x839\x81Q\x91R\x86R\x86\x81 \x94\x90\x94U\x82T\x82\x90\x04\x81\x16\x84R`\0\x80Q` a\\\x8C\x839\x81Q\x91R\x90\x94R\x93\x90\x91 \x80T`\x01`\xFF\x19\x90\x91\x16\x17\x90U`\x9C\x80Tc\xFF\xFF\xFF\xFF\x19\x16C\x84\x16\x17\x90UT\x91\x90\x91\x04\x16\x90\x7F\xFA\xF4\xB2\x05Dy\xD0\xF8>\x90\x9Bs\xCD\xE2\xA6\xCB\x18\xEC*\x93\xBA\x8A\xD5\xA6#)\0\x1C\x86\xB1\xF3\xEA\x90a/\xCC\x90\x84\x90a[TV[`@Q\x80\x91\x03\x90\xA2`\x97Ta/\xEF\x90`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16`\x01aQ\xF4V[`\x97`\x1Ca\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`\x97T`@Qc7}\xA3\x1B`\xE1\x1B\x81R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cn\xFBF6\x90a0b\x90\x8A\x90\x8A\x90\x8A\x90\x8A\x90\x8A\x90`\x04\x01aT\x98V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a0\x7FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra0\xA7\x91\x90\x81\x01\x90aV\x0FV[\x91P\x91P\x95P\x95\x93PPPPV[a0\xBDa=-V[a0\xC7`\0a?hV[V[a0\xD1a=-V[`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x90\x1AeM\xC80\xC9N\x8A\x12\xC9\xA3\xBC\n\x92\xAC\x11\xB5\xCF(\x04l\xA8\xD1\x90i\x1C\xDA\xF5 \x90\x16\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`\x9FT`\x01`\x01`\xA0\x1B\x03\x163\x14a1gW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdAuth1`\xD8\x1B`D\x82\x01R`d\x01a\x07\xBDV[`\x9CT`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15\x80\x15\x90a1\x84WPC\x15\x15[a1\xC2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x13\xDC\x08\x14\xDD\x18]\x19H\x1D[\x9A[\x9A]`\x8A\x1B`D\x82\x01R`d\x01a\x07\xBDV[`\x98T`\x97T`@\x80Q`\xC0\x81\x01\x82R`\0``\x80\x83\x01\x91\x90\x91R`\x80\x82\x01Rc\xFF\xFF\xFF\xFF\x93\x84\x16\x80\x82R` \x82\x01\x86\x90RC\x85\x16\x92\x82\x01\x92\x90\x92R`\x9ET\x84\x16`\xA0\x82\x01R`\x9D\x80T\x92\x94`\x01`\xE0\x1B\x90\x94\x04\x90\x93\x16\x92\x90\x91a2%\x90aX\x16V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta2Q\x90aX\x16V[\x80\x15a2\x9EW\x80`\x1F\x10a2sWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a2\x9EV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a2\x81W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`\x80\x82\x01R`\x9CT`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x90\x81\x16``\x83\x01R\x83\x16\x15a3FW`\0a2\xD3`\x01\x85a[/V[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\0\x80Q` a\\\xAC\x839\x81Q\x91R` R`@\x90 T\x90\x91P`\x01\x90`\xFF\x16`\x04\x81\x11\x15a3\x10Wa3\x10aN\xE9V[\x14\x15a3DWc\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\0\x80Q` a\\\xAC\x839\x81Q\x91R` R`@\x90 \x80T`\xFF\x19\x16`\x02\x17\x90U[P[c\xFF\xFF\xFF\xFF\x82\x16\x15a4]W`\0a3_`\x01\x84a[/V[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\0\x80Q` a\\\x8C\x839\x81Q\x91R` R`@\x90 T\x90\x91P`\x01\x90`\xFF\x16`\x04\x81\x11\x15a3\x9CWa3\x9CaN\xE9V[\x14\x15a4[W`\x97T`\x9CTa3\xC2\x91c\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x90\x91\x04\x81\x16\x91\x16aQ\xF4V[c\xFF\xFF\xFF\xFF\x16Cc\xFF\xFF\xFF\xFF\x16\x11\x15a4-W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FRdTask can't yet cancel the init`D\x82\x01Rf OpTask`\xC8\x1B`d\x82\x01R`\x84\x01a\x07\xBDV[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\0\x80Q` a\\\x8C\x839\x81Q\x91R` R`@\x90 \x80T`\xFF\x19\x16`\x02\x17\x90U[P[\x80`@Q` \x01a4n\x91\x90a[\xEAV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 c\xFF\xFF\xFF\xFF\x87\x16`\0\x81\x81R\x7F\xBB\x86\xFB\xC04\xF4\xE3\x82\x92\x99t\xBC\xD8A\x9E\xD6&\xB0\xEAd\x7F\x96-\x89\xBA/\xB6\xBD(xZ\xB9\x84R\x84\x81 \x92\x90\x92U`\0\x80Q` a\\\xAC\x839\x81Q\x91R\x90\x92R\x91\x90\x91 \x80T`\xFF\x19\x16`\x01\x17\x90U\x90\x7F\x16\x1F\xAA]\x92\xF6\xBF\x02b\x13\x9CW\xC8|\xA6\xCC`\xE5\xB3\xC9\xC34\x1D\xC1u\xCE\xC9z%\x16\xCC}\x90a5\x0B\x90\x84\x90a[\xEAV[`@Q\x80\x91\x03\x90\xA2a5\x1E\x83`\x01aQ\xF4V[`\x98\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UPPPPV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91``\x91\x83\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x84\x81`\0\x81Q\x81\x10a5yWa5yaV\xCBV[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Qca\xC8\xA1/`\xE1\x1B\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90c\xC3\x91B^\x90a5\xB5\x90\x88\x90\x86\x90`\x04\x01aZ~V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a5\xD2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra5\xFA\x91\x90\x81\x01\x90aY@V[`\0\x81Q\x81\x10a6\x0CWa6\x0CaV\xCBV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x88\x90Rc\xFF\xFF\xFF\xFF\x87\x81\x16`$\x83\x01R\x90\x91\x16`D\x82\x01\x81\x90R\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x89\x16\x90c\x04\xECcQ\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a6xW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a6\x9C\x91\x90aY\xEEV[`\x01`\x01`\xC0\x1B\x03\x16\x90P`\0a6\xB2\x82a?\xBAV[\x90P\x81a6\xC0\x8A\x83\x8Aa\x15WV[\x95P\x95PPPPP\x93P\x93\x91PPV[`\x97T`@\x80Qc\xDF\\\xF7#`\xE0\x1B\x81R\x90Q`\0\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xDF\\\xF7#\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a*\xE4W=`\0\x80>=`\0\xFD[a7\"a=-V[`\x01`\x01`\xA0\x1B\x03\x81\x16a7\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x07\xBDV[a\x04@\x81a?hV[a7\x98a=-V[`\x9CTc\xFF\xFF\xFF\xFF`\x01`@\x1B\x90\x91\x04\x16C\x14\x80\x15\x90a7\xB7WPC\x15\x15[a7\xD3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xBD\x90aZ\xD2V[`@\x80Q`\xE0\x81\x01\x82R`\0\x81\x83\x01\x81\x90R``\x80\x83\x01\x81\x90R`\xA0\x83\x01R`\xC0\x82\x01R`\x97T`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x82RC\x81\x16` \x80\x84\x01\x91\x90\x91R\x90\x86\x16`\x80\x83\x01R\x82Q`\x1F\x85\x01\x82\x90\x04\x82\x02\x81\x01\x82\x01\x90\x93R\x83\x83R\x90\x91\x90\x84\x90\x84\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPPP``\x82\x01R`\x9CT`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16a8\xC2Wc\xFF\xFF\xFF\xFFC\x16`@\x80\x83\x01\x91\x90\x91R\x80Q` `\x1F\x85\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x83\x81R\x90\x84\x90\x84\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPPP`\xA0\x82\x01Rc\xFF\xFF\xFF\xFF\x84\x16`\xC0\x82\x01Ra9vV[`\x9CT`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16`@\x82\x01R`\x9D\x80Ta8\xE4\x90aX\x16V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta9\x10\x90aX\x16V[\x80\x15a9]W\x80`\x1F\x10a92Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a9]V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a9@W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`\xA0\x82\x01R`\x9ETc\xFF\xFF\xFF\xFF\x16`\xC0\x82\x01R[`\x97T`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15a:\x1DW`\x97T`\0\x90a9\xAA\x90`\x01\x90`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16a[/V[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\0\x80Q` a\\\x8C\x839\x81Q\x91R` R`@\x90 T\x90\x91P`\x01\x90`\xFF\x16`\x04\x81\x11\x15a9\xE7Wa9\xE7aN\xE9V[\x14\x15a:\x1BWc\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\0\x80Q` a\\\x8C\x839\x81Q\x91R` R`@\x90 \x80T`\xFF\x19\x16`\x02\x17\x90U[P[`\x98Tc\xFF\xFF\xFF\xFF\x16\x15a:\xB6W`\x98T`\0\x90a:C\x90`\x01\x90c\xFF\xFF\xFF\xFF\x16a[/V[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\0\x80Q` a\\\xAC\x839\x81Q\x91R` R`@\x90 T\x90\x91P`\x01\x90`\xFF\x16`\x04\x81\x11\x15a:\x80Wa:\x80aN\xE9V[\x14\x15a:\xB4Wc\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\0\x80Q` a\\\xAC\x839\x81Q\x91R` R`@\x90 \x80T`\xFF\x19\x16`\x02\x17\x90U[P[\x80`@Q` \x01a:\xC7\x91\x90a[TV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 `\x97\x80Tc\xFF\xFF\xFF\xFF`\x01`\xE0\x1B\x91\x82\x90\x04\x81\x16`\0\x90\x81R`\0\x80Q` a\\\xCC\x839\x81Q\x91R\x86R\x86\x81 \x94\x90\x94U\x82T\x82\x90\x04\x81\x16\x84R`\0\x80Q` a\\\x8C\x839\x81Q\x91R\x90\x94R\x93\x90\x91 \x80T`\x01`\xFF\x19\x90\x91\x16\x17\x90U`\x9C\x80Tc\xFF\xFF\xFF\xFF\x19\x16C\x84\x16\x17\x90UT\x91\x90\x91\x04\x16\x90\x7F\xFA\xF4\xB2\x05Dy\xD0\xF8>\x90\x9Bs\xCD\xE2\xA6\xCB\x18\xEC*\x93\xBA\x8A\xD5\xA6#)\0\x1C\x86\xB1\xF3\xEA\x90a;\x81\x90\x84\x90a[TV[`@Q\x80\x91\x03\x90\xA2`\x97`\x1C\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x7F\xDC[\xA3\xB6n\xC6I\x1BX[?\x13i\x8E\xD7\x11\xAA\x82\x90q\xF43\0\xD6\xED\xE6\xDB\xA6~(\xD7_\x82`@Qa/\xCC\x91\x90a[TV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a<$W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a<H\x91\x90aWJV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a<xW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xBD\x90aWgV[`fT\x19\x81\x19`fT\x19\x16\x14a<\xF6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07\xBDV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x13\x05V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a0\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x07\xBDV[`\x01`\x01`\xA0\x1B\x03\x81\x16a>\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x07\xBDV[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a>\x9FWP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a?!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x07\xBDV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a?d\x82a=\x87V[PPV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[```\0\x80a?\xC8\x84a@\x86V[a\xFF\xFF\x16`\x01`\x01`@\x1B\x03\x81\x11\x15a?\xE3Wa?\xE3aA\xC8V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a@\rW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x80[\x82Q\x82\x10\x80\x15a@%WPa\x01\0\x81\x10[\x15a@|W`\x01\x81\x1B\x93P\x85\x84\x16\x15a@lW\x80`\xF8\x1B\x83\x83\x81Q\x81\x10a@NWa@NaV\xCBV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x81`\x01\x01\x91P[a@u\x81aW/V[\x90Pa@\x14V[P\x90\x94\x93PPPPV[`\0\x80[\x82\x15a@\xB1Wa@\x9B`\x01\x84a\\RV[\x90\x92\x16\x91\x80a@\xA9\x81a\\iV[\x91PPa@\x8AV[\x92\x91PPV[\x82\x80Ta@\xC3\x90aX\x16V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82a@\xE5W`\0\x85UaA+V[\x82`\x1F\x10a@\xFEW\x82\x80\x01`\xFF\x19\x825\x16\x17\x85UaA+V[\x82\x80\x01`\x01\x01\x85U\x82\x15aA+W\x91\x82\x01[\x82\x81\x11\x15aA+W\x825\x82U\x91` \x01\x91\x90`\x01\x01\x90aA\x10V[PaA7\x92\x91PaA;V[P\x90V[[\x80\x82\x11\x15aA7W`\0\x81U`\x01\x01aA<V[`\0`\xE0\x82\x84\x03\x12\x15aAbW`\0\x80\xFD[P\x91\x90PV[`\0``\x82\x84\x03\x12\x15aAbW`\0\x80\xFD[`\0\x80`\x80\x83\x85\x03\x12\x15aA\x8DW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aA\xA3W`\0\x80\xFD[aA\xAF\x85\x82\x86\x01aAPV[\x92PPaA\xBF\x84` \x85\x01aAhV[\x90P\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aB\0WaB\0aA\xC8V[`@R\x90V[`@Qa\x01\0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aB\0WaB\0aA\xC8V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aBQWaBQaA\xC8V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15aBrWaBraA\xC8V[P`\x05\x1B` \x01\x90V[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x04@W`\0\x80\xFD[\x805aB\x99\x81aB|V[\x91\x90PV[`\0\x82`\x1F\x83\x01\x12aB\xAFW`\0\x80\xFD[\x815` aB\xC4aB\xBF\x83aBYV[aB)V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aB\xE3W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aC\x07W\x805aB\xFA\x81aB|V[\x83R\x91\x83\x01\x91\x83\x01aB\xE7V[P\x96\x95PPPPPPV[`\0`@\x82\x84\x03\x12\x15aC$W`\0\x80\xFD[aC,aA\xDEV[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12aCSW`\0\x80\xFD[\x815` aCcaB\xBF\x83aBYV[\x82\x81R`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aC\x82W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aC\x07WaC\x98\x88\x82aC\x12V[\x83R\x91\x83\x01\x91`@\x01aC\x86V[`\0\x82`\x1F\x83\x01\x12aC\xB7W`\0\x80\xFD[aC\xBFaA\xDEV[\x80`@\x84\x01\x85\x81\x11\x15aC\xD1W`\0\x80\xFD[\x84[\x81\x81\x10\x15aC\xEBW\x805\x84R` \x93\x84\x01\x93\x01aC\xD3V[P\x90\x95\x94PPPPPV[`\0`\x80\x82\x84\x03\x12\x15aD\x08W`\0\x80\xFD[aD\x10aA\xDEV[\x90PaD\x1C\x83\x83aC\xA6V[\x81RaD+\x83`@\x84\x01aC\xA6V[` \x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12aDGW`\0\x80\xFD[\x815` aDWaB\xBF\x83aBYV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aDvW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aC\x07W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aD\x99W`\0\x80\x81\xFD[aD\xA7\x89\x86\x83\x8B\x01\x01aB\x9EV[\x84RP\x91\x83\x01\x91\x83\x01aDzV[`\0a\x01\x80\x82\x84\x03\x12\x15aD\xC8W`\0\x80\xFD[aD\xD0aB\x06V[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aD\xE9W`\0\x80\xFD[aD\xF5\x85\x83\x86\x01aB\x9EV[\x83R` \x84\x015\x91P\x80\x82\x11\x15aE\x0BW`\0\x80\xFD[aE\x17\x85\x83\x86\x01aCBV[` \x84\x01R`@\x84\x015\x91P\x80\x82\x11\x15aE0W`\0\x80\xFD[aE<\x85\x83\x86\x01aCBV[`@\x84\x01RaEN\x85``\x86\x01aC\xF6V[``\x84\x01RaE`\x85`\xE0\x86\x01aC\x12V[`\x80\x84\x01Ra\x01 \x84\x015\x91P\x80\x82\x11\x15aEzW`\0\x80\xFD[aE\x86\x85\x83\x86\x01aB\x9EV[`\xA0\x84\x01Ra\x01@\x84\x015\x91P\x80\x82\x11\x15aE\xA0W`\0\x80\xFD[aE\xAC\x85\x83\x86\x01aB\x9EV[`\xC0\x84\x01Ra\x01`\x84\x015\x91P\x80\x82\x11\x15aE\xC6W`\0\x80\xFD[PaE\xD3\x84\x82\x85\x01aD6V[`\xE0\x83\x01RP\x92\x91PPV[`\0\x80`\0\x83\x85\x03`\xE0\x81\x12\x15aE\xF5W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aF\x0CW`\0\x80\xFD[\x90\x86\x01\x90`\xC0\x82\x89\x03\x12\x15aF W`\0\x80\xFD[\x81\x95P`\xA0`\x1F\x19\x84\x01\x12\x15aF5W`\0\x80\xFD[` \x87\x01\x94P`\xC0\x87\x015\x92P\x80\x83\x11\x15aFOW`\0\x80\xFD[PPaF]\x86\x82\x87\x01aD\xB5V[\x91PP\x92P\x92P\x92V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04@W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aF\x8EW`\0\x80\xFD[\x815aF\x99\x81aFgV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15aF\xB2W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15aF\xCCW`\0\x80\xFD[\x825`\x02\x81\x10aF\xDBW`\0\x80\xFD[\x91P` \x83\x015aF\xEB\x81aB|V[\x80\x91PP\x92P\x92\x90PV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15aG\x1CW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01aG\0V[\x81\x81\x11\x15aG.W`\0` \x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0aF\x99` \x83\x01\x84aF\xF6V[\x80\x15\x15\x81\x14a\x04@W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15aG\x81W`\0\x80\xFD[\x885aG\x8C\x81aFgV[\x97P` \x89\x015aG\x9C\x81aFgV[\x96P`@\x89\x015aG\xAC\x81aFgV[\x95P``\x89\x015aG\xBC\x81aFgV[\x94P`\x80\x89\x015aG\xCC\x81aGVV[\x93P`\xA0\x89\x015aG\xDC\x81aFgV[\x92P`\xC0\x89\x015aG\xEC\x81aB|V[\x91P`\xE0\x89\x015aG\xFC\x81aB|V[\x80\x91PP\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[`\0\x80`\0``\x84\x86\x03\x12\x15aH\"W`\0\x80\xFD[\x835aH-\x81aFgV[\x92P` \x84\x81\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aHJW`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12aH^W`\0\x80\xFD[\x815\x81\x81\x11\x15aHpWaHpaA\xC8V[aH\x82`\x1F\x82\x01`\x1F\x19\x16\x85\x01aB)V[\x91P\x80\x82R\x88\x84\x82\x85\x01\x01\x11\x15aH\x98W`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x94PPPPaH\xBB`@\x85\x01aB\x8EV[\x90P\x92P\x92P\x92V[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0\x80[\x86\x81\x10\x15aIZW\x83\x85\x03\x8AR\x82Q\x80Q\x80\x87R\x90\x87\x01\x90\x87\x87\x01\x90\x84[\x81\x81\x10\x15aIEW\x83Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x8A\x81\x01Q\x8B\x85\x01R`@\x90\x81\x01Q`\x01`\x01``\x1B\x03\x16\x90\x84\x01R\x92\x89\x01\x92``\x90\x92\x01\x91`\x01\x01aI\x01V[PP\x9A\x87\x01\x9A\x95PP\x91\x85\x01\x91`\x01\x01aH\xE3V[P\x92\x98\x97PPPPPPPPV[` \x81R`\0aF\x99` \x83\x01\x84aH\xC4V[`\0` \x82\x84\x03\x12\x15aI\x8DW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aI\xA3W`\0\x80\xFD[\x82\x01`@\x81\x85\x03\x12\x15aF\x99W`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12aI\xC7W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aI\xDEW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aI\xF6W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\x80\x87\x89\x03\x12\x15aJ\x16W`\0\x80\xFD[\x865aJ!\x81aFgV[\x95P` \x87\x015aJ1\x81aB|V[\x94P`@\x87\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aJMW`\0\x80\xFD[aJY\x8A\x83\x8B\x01aI\xB5V[\x90\x96P\x94P``\x89\x015\x91P\x80\x82\x11\x15aJrW`\0\x80\xFD[\x81\x89\x01\x91P\x89`\x1F\x83\x01\x12aJ\x86W`\0\x80\xFD[\x815\x81\x81\x11\x15aJ\x95W`\0\x80\xFD[\x8A` \x82`\x05\x1B\x85\x01\x01\x11\x15aJ\xAAW`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92\x95P\x92\x95P\x92\x95V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aJ\xF6W\x81Qc\xFF\xFF\xFF\xFF\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aJ\xD4V[P\x94\x95\x94PPPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0[\x85\x81\x10\x15aKIW\x82\x84\x03\x89RaK7\x84\x83QaJ\xC0V[\x98\x85\x01\x98\x93P\x90\x84\x01\x90`\x01\x01aK\x1FV[P\x91\x97\x96PPPPPPPV[` \x81R`\0\x82Q`\x80` \x84\x01RaKr`\xA0\x84\x01\x82aJ\xC0V[\x90P` \x84\x01Q`\x1F\x19\x80\x85\x84\x03\x01`@\x86\x01RaK\x90\x83\x83aJ\xC0V[\x92P`@\x86\x01Q\x91P\x80\x85\x84\x03\x01``\x86\x01RaK\xAD\x83\x83aJ\xC0V[\x92P``\x86\x01Q\x91P\x80\x85\x84\x03\x01`\x80\x86\x01RPaK\xCB\x82\x82aK\x01V[\x95\x94PPPPPV[`\0\x80`\0`\xA0\x84\x86\x03\x12\x15aK\xE9W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aL\0W`\0\x80\xFD[aL\x0C\x87\x83\x88\x01aAPV[\x94PaL\x1B\x87` \x88\x01aAhV[\x93P`\x80\x86\x015\x91P\x80\x82\x11\x15aL1W`\0\x80\xFD[PaF]\x86\x82\x87\x01aD\xB5V[`\0` \x82\x84\x03\x12\x15aLPW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aLfW`\0\x80\xFD[\x82\x01a\x01 \x81\x85\x03\x12\x15aF\x99W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aL\x8BW`\0\x80\xFD[\x815`\xFF\x81\x16\x81\x14aF\x99W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15aL\xB1W`\0\x80\xFD[\x835aL\xBC\x81aFgV[\x92P` \x84\x81\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aL\xD8W`\0\x80\xFD[\x85\x01`\x1F\x81\x01\x87\x13aL\xE9W`\0\x80\xFD[\x805aL\xF7aB\xBF\x82aBYV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x89\x83\x11\x15aM\x16W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aM4W\x835\x82R\x92\x84\x01\x92\x90\x84\x01\x90aM\x1BV[\x80\x96PPPPPPaH\xBB`@\x85\x01aB\x8EV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aM\x80W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aMdV[P\x90\x96\x95PPPPPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15aM\xA1W`\0\x80\xFD[\x835aM\xAC\x81aB|V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aM\xC7W`\0\x80\xFD[aM\xD3\x86\x82\x87\x01aI\xB5V[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15aM\xF8W`\0\x80\xFD[\x855\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aN\x16W`\0\x80\xFD[aN\"\x89\x83\x8A\x01aI\xB5V[\x90\x96P\x94P`@\x88\x015\x91PaN7\x82aB|V[\x90\x92P``\x87\x015\x90\x80\x82\x11\x15aNMW`\0\x80\xFD[PaNZ\x88\x82\x89\x01aD\xB5V[\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aJ\xF6W\x81Q`\x01`\x01``\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aN{V[`@\x81R`\0\x83Q`@\x80\x84\x01RaN\xBB`\x80\x84\x01\x82aNgV[\x90P` \x85\x01Q`?\x19\x84\x83\x03\x01``\x85\x01RaN\xD8\x82\x82aNgV[\x92PPP\x82` \x83\x01R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x05\x83\x10aO!WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15aO<W`\0\x80\xFD[\x835aOG\x81aFgV[\x92P` \x84\x015\x91P`@\x84\x015aO^\x81aB|V[\x80\x91PP\x92P\x92P\x92V[\x82\x81R`@` \x82\x01R`\0aO\x82`@\x83\x01\x84aH\xC4V[\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15aO\x9CW`\0\x80\xFD[\x815aF\x99\x81aB|V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aO\xBEW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aO\xD8W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15aI\xF6W`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aP\x04W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aP#W`\0\x80\xFD[\x806\x03\x83\x13\x15aI\xF6W`\0\x80\xFD[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x81R`\0\x825aPl\x81aB|V[c\xFF\xFF\xFF\xFF\x81\x16` \x84\x01RP` \x83\x015aP\x87\x81aB|V[c\xFF\xFF\xFF\xFF\x81\x16`@\x84\x01RPaP\xA0`@\x84\x01aB\x8EV[c\xFF\xFF\xFF\xFF\x81\x16``\x84\x01RPaP\xBA``\x84\x01\x84aO\xEDV[`\xE0`\x80\x85\x01RaP\xD0a\x01\0\x85\x01\x82\x84aP2V[\x91PPaP\xDF`\x80\x85\x01aB\x8EV[c\xFF\xFF\xFF\xFF\x81\x16`\xA0\x85\x01RPaP\xF9`\xA0\x85\x01\x85aO\xEDV[\x84\x83\x03`\x1F\x19\x01`\xC0\x86\x01RaQ\x10\x83\x82\x84aP2V[\x92PPPaQ `\xC0\x85\x01aB\x8EV[c\xFF\xFF\xFF\xFF\x81\x16`\xE0\x85\x01R[P\x93\x92PPPV[` \x80\x82R`=\x90\x82\x01R\x7Fsupplied task does not match the`@\x82\x01R\x7F one recorded in the contract\0\0\0``\x82\x01R`\x80\x01\x90V[` \x80\x82R`,\x90\x82\x01R\x7FAggregator has already responded`@\x82\x01Rk to the task`\xA0\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aR\x13WaR\x13aQ\xDEV[\x01\x94\x93PPPPV[` \x80\x82R`-\x90\x82\x01R\x7FAggregator has responded to the `@\x82\x01Rltask too late`\x98\x1B``\x82\x01R`\x80\x01\x90V[\x805aRt\x81aB|V[c\xFF\xFF\xFF\xFF\x16\x82R` \x81\x81\x015\x90\x83\x01R`@\x90\x81\x015\x91\x01RV[c\xFF\xFF\xFF\xFF\x81Q\x16\x82R` \x81\x01Q` \x83\x01R`\0`@\x82\x01Q`\x80`@\x85\x01RaR\xC0`\x80\x85\x01\x82aNgV[\x90P``\x83\x01Q\x84\x82\x03``\x86\x01RaK\xCB\x82\x82aNgV[aR\xE3\x81\x84aRiV[`\x80``\x82\x01R`\0aO\x82`\x80\x83\x01\x84aR\x91V[``\x81\x01a@\xB1\x82\x84aRiV[` \x80\x82R`\x05\x90\x82\x01Rd\x04\x17WF\x83`\xDC\x1B`@\x82\x01R``\x01\x90V[` \x81R`\0\x825aS7\x81aB|V[c\xFF\xFF\xFF\xFF\x80\x82\x16` \x85\x01R` \x85\x015`@\x85\x01R`@\x85\x015\x91PaS^\x82aB|V[\x80\x82\x16``\x85\x01R``\x85\x015\x91PaSv\x82aB|V[\x80\x82\x16`\x80\x85\x01RaS\x8B`\x80\x86\x01\x86aO\xEDV[\x92P`\xC0`\xA0\x86\x01RaS\xA2`\xE0\x86\x01\x84\x83aP2V[\x92PP`\xA0\x85\x015aS\xB3\x81aB|V[\x16`\xC0\x93\x90\x93\x01\x92\x90\x92RP\x91\x90PV[\x805aS\xCF\x81aB|V[c\xFF\xFF\xFF\xFF\x16\x82R` \x81\x81\x015\x90\x83\x01R`@\x80\x82\x015\x90\x83\x01R``\x80\x82\x015\x90\x83\x01R`\x80\x90\x81\x015\x91\x01RV[`\xA0\x81\x01a@\xB1\x82\x84aS\xC4V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aJ\xF6WaT?\x87\x83Q\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[`@\x96\x90\x96\x01\x95\x90\x82\x01\x90`\x01\x01aT\"V[\x80`\0[`\x02\x81\x10\x15aTuW\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01aTVV[PPPPV[aT\x86\x82\x82QaTRV[` \x81\x01Qa\x11\x1C`@\x84\x01\x82aTRV[\x85\x81R`\x80` \x82\x01R`\0aT\xB2`\x80\x83\x01\x86\x88aP2V[c\xFF\xFF\xFF\xFF\x85\x16`@\x84\x01R\x82\x81\x03``\x84\x01Ra\x01\x80\x84Q\x81\x83RaT\xDA\x82\x84\x01\x82aJ\xC0V[\x91PP` \x85\x01Q\x82\x82\x03` \x84\x01RaT\xF4\x82\x82aT\x0EV[\x91PP`@\x85\x01Q\x82\x82\x03`@\x84\x01RaU\x0E\x82\x82aT\x0EV[\x91PP``\x85\x01QaU#``\x84\x01\x82aT{V[P`\x80\x85\x01Q\x80Q`\xE0\x84\x01R` \x01Qa\x01\0\x83\x01R`\xA0\x85\x01Q\x82\x82\x03a\x01 \x84\x01RaUR\x82\x82aJ\xC0V[\x91PP`\xC0\x85\x01Q\x82\x82\x03a\x01@\x84\x01RaUm\x82\x82aJ\xC0V[\x91PP`\xE0\x85\x01Q\x82\x82\x03a\x01`\x84\x01RaU\x88\x82\x82aK\x01V[\x9A\x99PPPPPPPPPPV[\x80Q`\x01`\x01``\x1B\x03\x81\x16\x81\x14aB\x99W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12aU\xBEW`\0\x80\xFD[\x81Q` aU\xCEaB\xBF\x83aBYV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aU\xEDW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aC\x07WaV\x02\x81aU\x96V[\x83R\x91\x83\x01\x91\x83\x01aU\xF1V[`\0\x80`@\x83\x85\x03\x12\x15aV\"W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aV9W`\0\x80\xFD[\x90\x84\x01\x90`@\x82\x87\x03\x12\x15aVMW`\0\x80\xFD[aVUaA\xDEV[\x82Q\x82\x81\x11\x15aVdW`\0\x80\xFD[aVp\x88\x82\x86\x01aU\xADV[\x82RP` \x83\x01Q\x82\x81\x11\x15aV\x85W`\0\x80\xFD[aV\x91\x88\x82\x86\x01aU\xADV[` \x83\x01RP\x80\x94PPPP` \x83\x01Q\x90P\x92P\x92\x90PV[aV\xB5\x81\x84aS\xC4V[`\xC0`\xA0\x82\x01R`\0aO\x82`\xC0\x83\x01\x84aR\x91V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aW\x07WaW\x07aQ\xDEV[\x02\x94\x93PPPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15aW*WaW*aQ\xDEV[P\x02\x90V[`\0`\0\x19\x82\x14\x15aWCWaWCaQ\xDEV[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15aW\\W`\0\x80\xFD[\x81QaF\x99\x81aFgV[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15aW\xC3W`\0\x80\xFD[\x81QaF\x99\x81aGVV[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80aX*W`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15aAbWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[`\0` \x80\x83\x85\x03\x12\x15aX^W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aXtW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aX\x85W`\0\x80\xFD[\x80QaX\x93aB\xBF\x82aBYV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15aX\xB2W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aX\xD0W\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90aX\xB7V[\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15aX\xEDW`\0\x80\xFD[aF\x99\x82aU\x96V[c\xFF\xFF\xFF\xFF\x84\x16\x81R`@` \x82\x01\x81\x90R\x81\x01\x82\x90R`\0`\x01`\x01`\xFB\x1B\x03\x83\x11\x15aY#W`\0\x80\xFD[\x82`\x05\x1B\x80\x85``\x85\x017`\0\x92\x01``\x01\x91\x82RP\x93\x92PPPV[`\0` \x80\x83\x85\x03\x12\x15aYSW`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aYiW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aYzW`\0\x80\xFD[\x80QaY\x88aB\xBF\x82aBYV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15aY\xA7W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aX\xD0W\x83QaY\xBF\x81aB|V[\x82R\x92\x84\x01\x92\x90\x84\x01\x90aY\xACV[c\xFF\xFF\xFF\xFF\x84\x16\x81R`@` \x82\x01R`\0aK\xCB`@\x83\x01\x84\x86aP2V[`\0` \x82\x84\x03\x12\x15aZ\0W`\0\x80\xFD[\x81Q`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x14aF\x99W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aZ)W`\0\x80\xFD[\x81QaF\x99\x81aB|V[`\0`\xFF\x82\x16`\xFF\x81\x14\x15aZKWaZKaQ\xDEV[`\x01\x01\x92\x91PPV[`@\x81R`\0aZh`@\x83\x01\x85\x87aP2V[\x90Pc\xFF\xFF\xFF\xFF\x83\x16` \x83\x01R\x94\x93PPPPV[`\0`@\x82\x01c\xFF\xFF\xFF\xFF\x85\x16\x83R` `@\x81\x85\x01R\x81\x85Q\x80\x84R``\x86\x01\x91P\x82\x87\x01\x93P`\0[\x81\x81\x10\x15aZ\xC5W\x84Q\x83R\x93\x83\x01\x93\x91\x83\x01\x91`\x01\x01aZ\xA9V[P\x90\x97\x96PPPPPPPV[` \x80\x82R`9\x90\x82\x01R\x7FCan't create a task in the same `@\x82\x01R\x7Fblock as a completed task\0\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[`\0c\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a[LWa[LaQ\xDEV[\x03\x93\x92PPPV[` \x81R`\0c\xFF\xFF\xFF\xFF\x80\x84Q\x16` \x84\x01R\x80` \x85\x01Q\x16`@\x84\x01R\x80`@\x85\x01Q\x16``\x84\x01RP``\x83\x01Q`\xE0`\x80\x84\x01Ra[\x9Ba\x01\0\x84\x01\x82aF\xF6V[\x90P`\x80\x84\x01Qa[\xB4`\xA0\x85\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P`\xA0\x84\x01Q\x83\x82\x03`\x1F\x19\x01`\xC0\x85\x01Ra[\xD0\x82\x82aF\xF6V[\x91PP`\xC0\x84\x01QaQ-`\xE0\x85\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[` \x81R`\0c\xFF\xFF\xFF\xFF\x80\x84Q\x16` \x84\x01R` \x84\x01Q`@\x84\x01R\x80`@\x85\x01Q\x16``\x84\x01R\x80``\x85\x01Q\x16`\x80\x84\x01R`\x80\x84\x01Q`\xC0`\xA0\x85\x01Ra\\9`\xE0\x85\x01\x82aF\xF6V[\x90P\x81`\xA0\x86\x01Q\x16`\xC0\x85\x01R\x80\x92PPP\x92\x91PPV[`\0\x82\x82\x10\x15a\\dWa\\daQ\xDEV[P\x03\x90V[`\0a\xFF\xFF\x80\x83\x16\x81\x81\x14\x15a\\\x81Wa\\\x81aQ\xDEV[`\x01\x01\x93\x92PPPV\xFE\x10\xAF\xAC\x923\xB4\xCC\xC5Md\x04\xFF\xC1\xCF;GQZ+\x8E\xDB\xF6u\xD1^\xDD\xCE\x05\xA0'\xDC\xBD)\x8C\x80\r\x08\x81\xDD \x8Dp^\xBC\x03\xEB\x18\x18\x9F8\x11\x82Y\xF2}\xD4;L`\xD6\x1C`~\x87#]b\x9D\xC8\x02\x03}\xED\x8Ca\xCB'\xFB)\xE4\x0F\xA0\x1B)\x97\x19\xD8\xF9\x91\xFF\xE2\x0B\xDC\xC5\x9FO\xA2dipfsX\"\x12 \x19W\x16\xB8h\xAEI\xC0\xAE\x9A\xA6}\x15\x85\xE0\xE2Q=\x82x1\x8A\x0B\xD3\x82\xEC\xDFI\x95\x17\r6dsolcC\0\x08\x0C\x003";
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
        ///Calls the contract's `createNewRdTask` (0xb3ea184e) function
        pub fn create_new_rd_task(
            &self,
            block_number: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([179, 234, 24, 78], block_number)
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
        ///Calls the contract's `forceRespondToOpTask` (0x01a3f013) function
        pub fn force_respond_to_op_task(
            &self,
            task: OpTask,
            task_response: OpTaskResponse,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([1, 163, 240, 19], (task, task_response))
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
        ///Calls the contract's `initialize` (0x2db2e39b) function
        pub fn initialize(
            &self,
            pauser_registry: ::ethers::core::types::Address,
            initial_owner: ::ethers::core::types::Address,
            aggregator: ::ethers::core::types::Address,
            generator: ::ethers::core::types::Address,
            allow_non_root_init: bool,
            bls_signature_checker_address: ::ethers::core::types::Address,
            task_response_window_block: u32,
            min_op_task_response_window_block: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [45, 178, 227, 155],
                    (
                        pauser_registry,
                        initial_owner,
                        aggregator,
                        generator,
                        allow_non_root_init,
                        bls_signature_checker_address,
                        task_response_window_block,
                        min_op_task_response_window_block,
                    ),
                )
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
        ///Calls the contract's `lastOpTaskCreatedBlock` (0x4d7a7116) function
        pub fn last_op_task_created_block(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([77, 122, 113, 22], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `latestOpTaskNum` (0x41789d57) function
        pub fn latest_op_task_num(&self) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([65, 120, 157, 87], ())
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
        ///Calls the contract's `latestRdTaskNum` (0x7afdd54b) function
        pub fn latest_rd_task_num(&self) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([122, 253, 213, 75], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `minOpTaskResponseWindowBlock` (0xc987de8e) function
        pub fn min_op_task_response_window_block(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([201, 135, 222, 142], ())
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
        ///Calls the contract's `respondToOpTask` (0x516a7227) function
        pub fn respond_to_op_task(
            &self,
            task: OpTask,
            task_response: OpTaskResponse,
            non_signer_stakes_and_signature: NonSignerStakesAndSignature,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [81, 106, 114, 39],
                    (task, task_response, non_signer_stakes_and_signature),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `respondToRdTask` (0x05c6b663) function
        pub fn respond_to_rd_task(
            &self,
            task: RdTask,
            task_response: RdTaskResponse,
            non_signer_stakes_and_signature: NonSignerStakesAndSignature,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [5, 198, 182, 99],
                    (task, task_response, non_signer_stakes_and_signature),
                )
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
        abi = "NewOpTaskCreated(uint32,(uint32,uint32,uint32,bytes,uint32,bytes,uint32))"
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
    #[ethevent(
        name = "NewOpTaskForceCreated",
        abi = "NewOpTaskForceCreated(uint32,(uint32,uint32,uint32,bytes,uint32,bytes,uint32))"
    )]
    pub struct NewOpTaskForceCreatedFilter {
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
    #[ethevent(
        name = "NewRdTaskCreated",
        abi = "NewRdTaskCreated(uint32,(uint32,uint256,uint32,uint32,bytes,uint32))"
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
    #[ethevent(
        name = "RdTaskCompleted",
        abi = "RdTaskCompleted(uint32,bytes32,(uint32,bytes32,bytes32,bytes32,bytes32))"
    )]
    pub struct RdTaskCompletedFilter {
        #[ethevent(indexed)]
        pub task_index: u32,
        #[ethevent(indexed)]
        pub block_hash: [u8; 32],
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
        abi = "RdTaskResponded(uint32,(uint32,bytes32,bytes32,bytes32,bytes32),(uint32,bytes32,uint96[],uint96[]))"
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
        BlssignatureCheckerAddressUpdatedFilter(BlssignatureCheckerAddressUpdatedFilter),
        InitializedFilter(InitializedFilter),
        NewOpTaskCreatedFilter(NewOpTaskCreatedFilter),
        NewOpTaskForceCreatedFilter(NewOpTaskForceCreatedFilter),
        NewRdTaskCreatedFilter(NewRdTaskCreatedFilter),
        OpTaskCompletedFilter(OpTaskCompletedFilter),
        OpTaskForceCompletedFilter(OpTaskForceCompletedFilter),
        OpTaskRespondedFilter(OpTaskRespondedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PausedFilter(PausedFilter),
        PauserRegistrySetFilter(PauserRegistrySetFilter),
        RdTaskCompletedFilter(RdTaskCompletedFilter),
        RdTaskRespondedFilter(RdTaskRespondedFilter),
        StaleStakesForbiddenUpdateFilter(StaleStakesForbiddenUpdateFilter),
        UnpausedFilter(UnpausedFilter),
    }
    impl ::ethers::contract::EthLogDecode for FinalizerTaskManagerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = BlssignatureCheckerAddressUpdatedFilter::decode_log(log) {
                return Ok(
                    FinalizerTaskManagerEvents::BlssignatureCheckerAddressUpdatedFilter(decoded),
                );
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
            if let Ok(decoded) = RdTaskCompletedFilter::decode_log(log) {
                return Ok(FinalizerTaskManagerEvents::RdTaskCompletedFilter(decoded));
            }
            if let Ok(decoded) = RdTaskRespondedFilter::decode_log(log) {
                return Ok(FinalizerTaskManagerEvents::RdTaskRespondedFilter(decoded));
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
                Self::BlssignatureCheckerAddressUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::NewOpTaskCreatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::NewOpTaskForceCreatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::NewRdTaskCreatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OpTaskCompletedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OpTaskForceCompletedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OpTaskRespondedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PausedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauserRegistrySetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RdTaskCompletedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RdTaskRespondedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::StaleStakesForbiddenUpdateFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnpausedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BlssignatureCheckerAddressUpdatedFilter> for FinalizerTaskManagerEvents {
        fn from(value: BlssignatureCheckerAddressUpdatedFilter) -> Self {
            Self::BlssignatureCheckerAddressUpdatedFilter(value)
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
    ///Container type for all input parameters for the `createNewRdTask` function with signature `createNewRdTask(uint256)` and selector `0xb3ea184e`
    #[derive(
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
    #[ethcall(name = "createNewRdTask", abi = "createNewRdTask(uint256)")]
    pub struct CreateNewRdTaskCall {
        pub block_number: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `forceRespondToOpTask` function with signature `forceRespondToOpTask((uint32,uint32,uint32,bytes,uint32,bytes,uint32),(uint32,bytes32,bytes32))` and selector `0x01a3f013`
    #[derive(
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
        abi = "forceRespondToOpTask((uint32,uint32,uint32,bytes,uint32,bytes,uint32),(uint32,bytes32,bytes32))"
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
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address,address,address,bool,address,uint32,uint32)` and selector `0x2db2e39b`
    #[derive(
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
        abi = "initialize(address,address,address,address,bool,address,uint32,uint32)"
    )]
    pub struct InitializeCall {
        pub pauser_registry: ::ethers::core::types::Address,
        pub initial_owner: ::ethers::core::types::Address,
        pub aggregator: ::ethers::core::types::Address,
        pub generator: ::ethers::core::types::Address,
        pub allow_non_root_init: bool,
        pub bls_signature_checker_address: ::ethers::core::types::Address,
        pub task_response_window_block: u32,
        pub min_op_task_response_window_block: u32,
    }
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
    ///Container type for all input parameters for the `minOpTaskResponseWindowBlock` function with signature `minOpTaskResponseWindowBlock()` and selector `0xc987de8e`
    #[derive(
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
        name = "minOpTaskResponseWindowBlock",
        abi = "minOpTaskResponseWindowBlock()"
    )]
    pub struct MinOpTaskResponseWindowBlockCall;
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
    ///Container type for all input parameters for the `respondToOpTask` function with signature `respondToOpTask((uint32,uint32,uint32,bytes,uint32,bytes,uint32),(uint32,bytes32,bytes32),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))` and selector `0x516a7227`
    #[derive(
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
        abi = "respondToOpTask((uint32,uint32,uint32,bytes,uint32,bytes,uint32),(uint32,bytes32,bytes32),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))"
    )]
    pub struct RespondToOpTaskCall {
        pub task: OpTask,
        pub task_response: OpTaskResponse,
        pub non_signer_stakes_and_signature: NonSignerStakesAndSignature,
    }
    ///Container type for all input parameters for the `respondToRdTask` function with signature `respondToRdTask((uint32,uint256,uint32,uint32,bytes,uint32),(uint32,bytes32,bytes32,bytes32,bytes32),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))` and selector `0x05c6b663`
    #[derive(
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
        abi = "respondToRdTask((uint32,uint256,uint32,uint32,bytes,uint32),(uint32,bytes32,bytes32,bytes32,bytes32),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))"
    )]
    pub struct RespondToRdTaskCall {
        pub task: RdTask,
        pub task_response: RdTaskResponse,
        pub non_signer_stakes_and_signature: NonSignerStakesAndSignature,
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
        Aggregator(AggregatorCall),
        AllTaskHashes(AllTaskHashesCall),
        AllTaskResponses(AllTaskResponsesCall),
        AllowNonRootInit(AllowNonRootInitCall),
        BlsApkRegistry(BlsApkRegistryCall),
        BlsSignatureChecker(BlsSignatureCheckerCall),
        CheckSignatures(CheckSignaturesCall),
        CreateNewOpTask(CreateNewOpTaskCall),
        CreateNewRdTask(CreateNewRdTaskCall),
        Delegation(DelegationCall),
        DummyForOperatorStateInfoType(DummyForOperatorStateInfoTypeCall),
        DummyForQuorumStakeTotalsType(DummyForQuorumStakeTotalsTypeCall),
        ForceCreateNewOpTask(ForceCreateNewOpTaskCall),
        ForceRespondToOpTask(ForceRespondToOpTaskCall),
        Generator(GeneratorCall),
        GetCheckSignaturesIndices(GetCheckSignaturesIndicesCall),
        GetOperatorState(GetOperatorStateCall),
        GetOperatorStateWithRegistryCoordinatorAndOperatorId(
            GetOperatorStateWithRegistryCoordinatorAndOperatorIdCall,
        ),
        GetQuorumBitmapsAtBlockNumber(GetQuorumBitmapsAtBlockNumberCall),
        IdToTaskStatus(IdToTaskStatusCall),
        Initialize(InitializeCall),
        LastCompletedOpTaskCreatedBlock(LastCompletedOpTaskCreatedBlockCall),
        LastCompletedOpTaskNum(LastCompletedOpTaskNumCall),
        LastCompletedOpTaskQuorumNumbers(LastCompletedOpTaskQuorumNumbersCall),
        LastCompletedOpTaskQuorumThresholdPercentage(
            LastCompletedOpTaskQuorumThresholdPercentageCall,
        ),
        LastOpTaskCreatedBlock(LastOpTaskCreatedBlockCall),
        LatestOpTaskNum(LatestOpTaskNumCall),
        LatestPendingStateHash(LatestPendingStateHashCall),
        LatestRdTaskNum(LatestRdTaskNumCall),
        MinOpTaskResponseWindowBlock(MinOpTaskResponseWindowBlockCall),
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
        SetPauserRegistry(SetPauserRegistryCall),
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
                <LastOpTaskCreatedBlockCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LastOpTaskCreatedBlock(decoded));
            }
            if let Ok(decoded) =
                <LatestOpTaskNumCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LatestOpTaskNum(decoded));
            }
            if let Ok(decoded) =
                <LatestPendingStateHashCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LatestPendingStateHash(decoded));
            }
            if let Ok(decoded) =
                <LatestRdTaskNumCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LatestRdTaskNum(decoded));
            }
            if let Ok(decoded) =
                <MinOpTaskResponseWindowBlockCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MinOpTaskResponseWindowBlock(decoded));
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
            if let Ok(decoded) =
                <SetPauserRegistryCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetPauserRegistry(decoded));
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
                Self::Aggregator(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AllTaskHashes(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AllTaskResponses(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AllowNonRootInit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BlsApkRegistry(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BlsSignatureChecker(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
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
                Self::ForceCreateNewOpTask(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ForceRespondToOpTask(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Generator(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::LastOpTaskCreatedBlock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LatestOpTaskNum(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LatestPendingStateHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LatestRdTaskNum(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MinOpTaskResponseWindowBlock(element) => {
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
                Self::SetPauserRegistry(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::Aggregator(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllTaskHashes(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllTaskResponses(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllowNonRootInit(element) => ::core::fmt::Display::fmt(element, f),
                Self::BlsApkRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::BlsSignatureChecker(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::ForceCreateNewOpTask(element) => ::core::fmt::Display::fmt(element, f),
                Self::ForceRespondToOpTask(element) => ::core::fmt::Display::fmt(element, f),
                Self::Generator(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::LastOpTaskCreatedBlock(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestOpTaskNum(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestPendingStateHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestRdTaskNum(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinOpTaskResponseWindowBlock(element) => {
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
                Self::SetPauserRegistry(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<LastOpTaskCreatedBlockCall> for FinalizerTaskManagerCalls {
        fn from(value: LastOpTaskCreatedBlockCall) -> Self {
            Self::LastOpTaskCreatedBlock(value)
        }
    }
    impl ::core::convert::From<LatestOpTaskNumCall> for FinalizerTaskManagerCalls {
        fn from(value: LatestOpTaskNumCall) -> Self {
            Self::LatestOpTaskNum(value)
        }
    }
    impl ::core::convert::From<LatestPendingStateHashCall> for FinalizerTaskManagerCalls {
        fn from(value: LatestPendingStateHashCall) -> Self {
            Self::LatestPendingStateHash(value)
        }
    }
    impl ::core::convert::From<LatestRdTaskNumCall> for FinalizerTaskManagerCalls {
        fn from(value: LatestRdTaskNumCall) -> Self {
            Self::LatestRdTaskNum(value)
        }
    }
    impl ::core::convert::From<MinOpTaskResponseWindowBlockCall> for FinalizerTaskManagerCalls {
        fn from(value: MinOpTaskResponseWindowBlockCall) -> Self {
            Self::MinOpTaskResponseWindowBlock(value)
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
    impl ::core::convert::From<SetPauserRegistryCall> for FinalizerTaskManagerCalls {
        fn from(value: SetPauserRegistryCall) -> Self {
            Self::SetPauserRegistry(value)
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
    ///Container type for all return fields from the `minOpTaskResponseWindowBlock` function with signature `minOpTaskResponseWindowBlock()` and selector `0xc987de8e`
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
    pub struct MinOpTaskResponseWindowBlockReturn(pub u32);
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
