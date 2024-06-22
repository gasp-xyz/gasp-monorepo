pub use finalizer_task_manager::*;
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
pub mod finalizer_task_manager {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_registryCoordinator"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "contract IRegistryCoordinator",
                            ),
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
                ],
            }),
            functions: ::core::convert::From::from([
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
                    ::std::borrow::ToOwned::to_owned("createNewTask"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createNewTask"),
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
                                            "struct IFinalizerTaskManager.OperatorStateInfo",
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
                    ::std::borrow::ToOwned::to_owned("getLatestPendingStateHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getLatestPendingStateHash",
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
                    ::std::borrow::ToOwned::to_owned("getTaskResponseWindowBlock"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getTaskResponseWindowBlock",
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
                    ::std::borrow::ToOwned::to_owned("indexToTaskStatus"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("indexToTaskStatus"),
                            inputs: ::std::vec![
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
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("latestTaskNum"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("latestTaskNum"),
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
                    ::std::borrow::ToOwned::to_owned("respondToTask"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("respondToTask"),
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
                    ::std::borrow::ToOwned::to_owned("setStaleStakesForbidden"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setStaleStakesForbidden",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
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
                    ::std::borrow::ToOwned::to_owned("staleStakesForbidden"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "staleStakesForbidden",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("taskNumber"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("taskNumber"),
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
                    ::std::borrow::ToOwned::to_owned("NewTaskCreated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("NewTaskCreated"),
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
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
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
                    ::std::borrow::ToOwned::to_owned("TaskCompleted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("TaskCompleted"),
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
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TaskResponded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("TaskResponded"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
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
    pub static FINALIZERTASKMANAGER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01 `@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@Qb\0]\x8B8\x03\x80b\0]\x8B\x839\x81\x01`@\x81\x90Rb\0\x005\x91b\0\x01\xF7V[\x81\x80`\x01`\x01`\xA0\x1B\x03\x16`\x80\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x80`\x01`\x01`\xA0\x1B\x03\x16ch0H5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\0\x8FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\0\xB5\x91\x90b\0\x02>V[`\x01`\x01`\xA0\x1B\x03\x16`\xA0\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x80`\x01`\x01`\xA0\x1B\x03\x16c]\xF4YF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x01\rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x013\x91\x90b\0\x02>V[`\x01`\x01`\xA0\x1B\x03\x16`\xC0\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP`\xA0Q`\x01`\x01`\xA0\x1B\x03\x16c\xDF\\\xF7#`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x01\x8DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x01\xB3\x91\x90b\0\x02>V[`\x01`\x01`\xA0\x1B\x03\x16`\xE0RP`\x97\x80T`\xFF\x19\x16`\x01\x17\x90Uc\xFF\xFF\xFF\xFF\x16a\x01\0RPb\0\x02eV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01\xF4W`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15b\0\x02\x0BW`\0\x80\xFD[\x82Qb\0\x02\x18\x81b\0\x01\xDEV[` \x84\x01Q\x90\x92Pc\xFF\xFF\xFF\xFF\x81\x16\x81\x14b\0\x023W`\0\x80\xFD[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15b\0\x02QW`\0\x80\xFD[\x81Qb\0\x02^\x81b\0\x01\xDEV[\x93\x92PPPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0QaZ\xA2b\0\x02\xE9`\09`\0\x81\x81a\x05}\x01Ra\x1EH\x01R`\0\x81\x81a\x05F\x01Ra)\xFB\x01R`\0\x81\x81a\x03\xAD\x01Ra+\xDE\x01R`\0\x81\x81a\x03\xD4\x01R\x81\x81a-\xB4\x01Ra/v\x01R`\0\x81\x81a\x04\x0E\x01R\x81\x81a\r\xE1\x01R\x81\x81a&\xC6\x01R\x81\x81a(^\x01Ra*\x98\x01RaZ\xA2`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\x11W`\x005`\xE0\x1C\x80cm\x14\xA9\x87\x11a\x01%W\x80c\xC3\xAF\x83\x13\x11a\0\xADW\x80c\xDF\\\xF7#\x11a\0|W\x80c\xDF\\\xF7#\x14a\x05AW\x80c\xF2\xFD\xE3\x8B\x14a\x05hW\x80c\xF5\xC9\x89\x9D\x14a\x05{W\x80c\xF8\xC8v^\x14a\x05\xA1W\x80c\xFA\xBC\x1C\xBC\x14a\x05\xB4W`\0\x80\xFD[\x80c\xC3\xAF\x83\x13\x14a\x04\xDBW\x80c\xCA\xB0\xE6\xFD\x14a\x04\xECW\x80c\xCE\xFD\xC1\xD4\x14a\x04\xFFW\x80c\xDBg\xF7w\x14a\x05 W`\0\x80\xFD[\x80c\x88o\x11\x95\x11a\0\xF4W\x80c\x88o\x11\x95\x14a\x04jW\x80c\x8B\0\xCE|\x14a\x04}W\x80c\x8D\xA5\xCB[\x14a\x04\x8DW\x80c\x99\xDB\xA0\xC4\x14a\x04\x9EW\x80c\xB9\x8D\t\x08\x14a\x04\xCEW`\0\x80\xFD[\x80cm\x14\xA9\x87\x14a\x04\tW\x80cqP\x18\xA6\x14a\x040W\x80cr\xD1\x8E\x8D\x14a\x048W\x80cz\xFA\x1E\xED\x14a\x04WW`\0\x80\xFD[\x80cAl~^\x11a\x01\xA8W\x80c\\\x15Vb\x11a\x01wW\x80c\\\x15Vb\x14a\x03\x80W\x80c\\\x97Z\xBB\x14a\x03\xA0W\x80c]\xF4YF\x14a\x03\xA8W\x80ch0H5\x14a\x03\xCFW\x80ck\x92x~\x14a\x03\xF6W`\0\x80\xFD[\x80cAl~^\x14a\x03\x12W\x80cOs\x9Ft\x14a\x03%W\x80cY\\jg\x14a\x03EW\x80cZ\xC8j\xB7\x14a\x03MW`\0\x80\xFD[\x80c$Z{\xFC\x11a\x01\xE4W\x80c$Z{\xFC\x14a\x02\x7FW\x80c,\xB2#\xD5\x14a\x02\xB2W\x80c-\x89\xF6\xFC\x14a\x02\xD2W\x80c5c\xB0\xD1\x14a\x02\xF2W`\0\x80\xFD[\x80c\x03s@\x8D\x14a\x02\x16W\x80c\x10\xD6z/\x14a\x02-W\x80c\x13d9\xDD\x14a\x02BW\x80c\x17\x1F\x1D[\x14a\x02UW[`\0\x80\xFD[`\xD1T[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02@a\x02;6`\x04aC\xD7V[a\x05\xC7V[\0[a\x02@a\x02P6`\x04aC\xF4V[a\x06\x80V[a\x02ha\x02c6`\x04aErV[a\x07\xBFV[`@\x80Q\x92\x15\x15\x83R\x90\x15\x15` \x83\x01R\x01a\x02$V[`\xCFTa\x02\x9A\x90d\x01\0\0\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02$V[a\x02\x1Aa\x02\xC06`\x04aE\xE0V[`\xCB` R`\0\x90\x81R`@\x90 T\x81V[a\x02\x1Aa\x02\xE06`\x04aE\xE0V[`\xCA` R`\0\x90\x81R`@\x90 T\x81V[a\x03\x05a\x03\x006`\x04aE\xFDV[a\tIV[`@Qa\x02$\x91\x90aGXV[a\x02@a\x03 6`\x04aG\x80V[a\r\xDFV[a\x038a\x0336`\x04aG\xE5V[a\x0FTV[`@Qa\x02$\x91\x90aH\xE9V[a\x02@a\x16zV[a\x03pa\x03[6`\x04aI\xB3V[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`@Q\x90\x15\x15\x81R` \x01a\x02$V[a\x03\x93a\x03\x8E6`\x04aI\xF3V[a\x17AV[`@Qa\x02$\x91\x90aJ\xA4V[`fTa\x02\x1AV[a\x02\x9A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\x9A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02@a\x04\x046`\x04aJ\xE8V[a\x19\tV[a\x02\x9A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02@a\x1C=V[`\xC9Tc\xFF\xFF\xFF\xFF\x16[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02$V[`\xD0Ta\x02\x9A\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`eTa\x02\x9A\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\xC9Ta\x04B\x90c\xFF\xFF\xFF\xFF\x16\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x02\x9AV[a\x04\xC1a\x04\xAC6`\x04aE\xE0V[`\xCC` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Qa\x02$\x91\x90aKYV[`\x97Ta\x03p\x90`\xFF\x16\x81V[a\x02@a\x04\xE96`\x04aK\x81V[PV[a\x02@a\x04\xFA6`\x04aN\xAAV[a\x1CQV[a\x05\x12a\x05\r6`\x04aO3V[a!\x81V[`@Qa\x02$\x92\x91\x90aOuV[a\x053a\x05.6`\x04aO\x96V[a#\x13V[`@Qa\x02$\x92\x91\x90aPVV[a\x02\x9A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02@a\x05v6`\x04aC\xD7V[a2+V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04BV[a\x02@a\x05\xAF6`\x04aP\x9FV[a2\xA1V[a\x02@a\x05\xC26`\x04aC\xF4V[a4\x07V[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x1AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06>\x91\x90aP\xFBV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x06wW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06n\x90aQ\x18V[`@Q\x80\x91\x03\x90\xFD[a\x04\xE9\x81a5cV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xC8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xEC\x91\x90aQbV[a\x07\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06n\x90aQ\x7FV[`fT\x81\x81\x16\x14a\x07\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06nV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`\0\x80`\0\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x87\x87`\0\x01Q\x88` \x01Q\x88`\0\x01Q`\0`\x02\x81\x10a\x08\x07Wa\x08\x07aQ\xC7V[` \x02\x01Q\x89Q`\x01` \x02\x01Q\x8A` \x01Q`\0`\x02\x81\x10a\x08,Wa\x08,aQ\xC7V[` \x02\x01Q\x8B` \x01Q`\x01`\x02\x81\x10a\x08HWa\x08HaQ\xC7V[` \x90\x81\x02\x91\x90\x91\x01Q\x8CQ\x8D\x83\x01Q`@Qa\x08\xA5\x9A\x99\x98\x97\x96\x95\x94\x01\x98\x89R` \x89\x01\x97\x90\x97R`@\x88\x01\x95\x90\x95R``\x87\x01\x93\x90\x93R`\x80\x86\x01\x91\x90\x91R`\xA0\x85\x01R`\xC0\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01Ra\x01 \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\0\x1Ca\x08\xC8\x91\x90aQ\xDDV[\x90Pa\t;a\x08\xE1a\x08\xDA\x88\x84a6ZV[\x86\x90a6\xF1V[a\x08\xE9a7\x85V[a\t1a\t\"\x85a\t\x1C`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x01\x82R`\x02\x90\x82\x01R\x90V[\x90a6ZV[a\t+\x8Ca8EV[\x90a6\xF1V[\x88b\x01\xD4\xC0a8\xD5V[\x90\x98\x90\x97P\x95PPPPPPV[```\0\x84`\x01`\x01`\xA0\x1B\x03\x16ch0H5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\x8BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xAF\x91\x90aP\xFBV[\x90P`\0\x85`\x01`\x01`\xA0\x1B\x03\x16c\x9E\x99#\xC2`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xF1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x15\x91\x90aP\xFBV[\x90P`\0\x86`\x01`\x01`\xA0\x1B\x03\x16c]\xF4YF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\nWW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n{\x91\x90aP\xFBV[\x90P`\0\x86Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\x98Wa\n\x98aD\rV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\n\xCBW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\n\xB6W\x90P[P\x90P`\0[\x87Q\x81\x10\x15a\r\xD3W`\0\x88\x82\x81Q\x81\x10a\n\xEEWa\n\xEEaQ\xC7V[\x01` \x01Q`@Qc\x89\x02bE`\xE0\x1B\x81R`\xF8\x91\x90\x91\x1C`\x04\x82\x01\x81\x90Rc\xFF\xFF\xFF\xFF\x8A\x16`$\x83\x01R\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x87\x16\x90c\x89\x02bE\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0BOW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0Bw\x91\x90\x81\x01\x90aQ\xFFV[\x90P\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0B\x92Wa\x0B\x92aD\rV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0B\xDDW\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a\x0B\xB0W\x90P[P\x84\x84\x81Q\x81\x10a\x0B\xF0Wa\x0B\xF0aQ\xC7V[` \x02` \x01\x01\x81\x90RP`\0[\x81Q\x81\x10\x15a\r\xBDW`@Q\x80``\x01`@R\x80\x87`\x01`\x01`\xA0\x1B\x03\x16cG\xB3\x14\xE8\x85\x85\x81Q\x81\x10a\x0C3Wa\x0C3aQ\xC7V[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0CY\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CvW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x9A\x91\x90aP\xFBV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x83\x81Q\x81\x10a\x0C\xBAWa\x0C\xBAaQ\xC7V[` \x02` \x01\x01Q\x81R` \x01\x89`\x01`\x01`\xA0\x1B\x03\x16c\xFA(\xC6'\x85\x85\x81Q\x81\x10a\x0C\xE8Wa\x0C\xE8aQ\xC7V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x04\x81\x01\x91\x90\x91R`\xFF\x88\x16`$\x82\x01Rc\xFF\xFF\xFF\xFF\x8F\x16`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\rDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\rh\x91\x90aR\x8FV[`\x01`\x01``\x1B\x03\x16\x81RP\x85\x85\x81Q\x81\x10a\r\x86Wa\r\x86aQ\xC7V[` \x02` \x01\x01Q\x82\x81Q\x81\x10a\r\x9FWa\r\x9FaQ\xC7V[` \x02` \x01\x01\x81\x90RP\x80\x80a\r\xB5\x90aR\xCEV[\x91PPa\x0B\xFEV[PPP\x80\x80a\r\xCB\x90aR\xCEV[\x91PPa\n\xD1V[P\x97\x96PPPPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E=W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Ea\x91\x90aP\xFBV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0F\rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R\x7FBLSSignatureChecker.onlyCoordina`D\x82\x01R\x7FtorOwner: caller is not the owne`d\x82\x01R\x7Fr of the registryCoordinator\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06nV[`\x97\x80T`\xFF\x19\x16\x82\x15\x15\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F@\xE4\xED\x88\n)\xE0\xF6\xDD\xCE0tW\xFBu\xCD\xDFO\xEE\xF7\xD3\xEC\xB00\x1B\xFD\xF4\x97j\x0E-\xFC\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[a\x0F\x7F`@Q\x80`\x80\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`\0\x87`\x01`\x01`\xA0\x1B\x03\x16ch0H5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xBFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xE3\x91\x90aP\xFBV[\x90Pa\x10\x10`@Q\x80`\x80\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`@Qca\xC8\xA1/`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90c\xC3\x91B^\x90a\x10@\x90\x8B\x90\x89\x90\x89\x90`\x04\x01aR\xE9V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10]W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x10\x85\x91\x90\x81\x01\x90aS3V[\x81R`@Qc@\xE0:\x81`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\x81\xC0u\x02\x90a\x10\xB7\x90\x8B\x90\x8B\x90\x8B\x90`\x04\x01aS\xEAV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xD4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x10\xFC\x91\x90\x81\x01\x90aS3V[`@\x82\x01R\x85`\x01`\x01`@\x1B\x03\x81\x11\x15a\x11\x19Wa\x11\x19aD\rV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x11LW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x117W\x90P[P``\x82\x01R`\0[`\xFF\x81\x16\x87\x11\x15a\x15\x8BW`\0\x85`\x01`\x01`@\x1B\x03\x81\x11\x15a\x11zWa\x11zaD\rV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x11\xA3W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x83``\x01Q\x83`\xFF\x16\x81Q\x81\x10a\x11\xBDWa\x11\xBDaQ\xC7V[` \x02` \x01\x01\x81\x90RP`\0[\x86\x81\x10\x15a\x14\x8BW`\0\x8C`\x01`\x01`\xA0\x1B\x03\x16c\x04\xECcQ\x8A\x8A\x85\x81\x81\x10a\x11\xF6Wa\x11\xF6aQ\xC7V[\x90P` \x02\x015\x8E\x88`\0\x01Q\x86\x81Q\x81\x10a\x12\x14Wa\x12\x14aQ\xC7V[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x12Q\x93\x92\x91\x90\x92\x83Rc\xFF\xFF\xFF\xFF\x91\x82\x16` \x84\x01R\x16`@\x82\x01R``\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12nW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\x92\x91\x90aT\x13V[\x90P`\x01`\x01`\xC0\x1B\x03\x81\x16a\x136W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R\x7FOperatorStateRetriever.getCheckS`D\x82\x01R\x7FignaturesIndices: operator must `d\x82\x01R\x7Fbe registered at blocknumber\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06nV[\x8A\x8A\x85`\xFF\x16\x81\x81\x10a\x13KWa\x13KaQ\xC7V[`\x01`\x01`\xC0\x1B\x03\x84\x16\x92\x015`\xF8\x1C\x91\x90\x91\x1C`\x01\x90\x81\x16\x14\x15\x90Pa\x14xW\x85`\x01`\x01`\xA0\x1B\x03\x16c\xDD\x98F\xB9\x8A\x8A\x85\x81\x81\x10a\x13\x8DWa\x13\x8DaQ\xC7V[\x90P` \x02\x015\x8D\x8D\x88`\xFF\x16\x81\x81\x10a\x13\xA9Wa\x13\xA9aQ\xC7V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x87\x90\x1B\x16\x81R`\x04\x81\x01\x94\x90\x94R\x91\x90\x91\x015`\xF8\x1C`$\x83\x01RPc\xFF\xFF\xFF\xFF\x8F\x16`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xFFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14#\x91\x90aT<V[\x85``\x01Q\x85`\xFF\x16\x81Q\x81\x10a\x14<Wa\x14<aQ\xC7V[` \x02` \x01\x01Q\x84\x81Q\x81\x10a\x14UWa\x14UaQ\xC7V[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x82a\x14t\x81aR\xCEV[\x93PP[P\x80a\x14\x83\x81aR\xCEV[\x91PPa\x11\xCBV[P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14\xA6Wa\x14\xA6aD\rV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x14\xCFW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a\x15PW\x84``\x01Q\x84`\xFF\x16\x81Q\x81\x10a\x14\xF6Wa\x14\xF6aQ\xC7V[` \x02` \x01\x01Q\x81\x81Q\x81\x10a\x15\x0FWa\x15\x0FaQ\xC7V[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a\x15)Wa\x15)aQ\xC7V[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x15H\x81aR\xCEV[\x91PPa\x14\xD5V[P\x80\x84``\x01Q\x84`\xFF\x16\x81Q\x81\x10a\x15kWa\x15kaQ\xC7V[` \x02` \x01\x01\x81\x90RPPP\x80\x80a\x15\x83\x90aTYV[\x91PPa\x11UV[P`\0\x89`\x01`\x01`\xA0\x1B\x03\x16c]\xF4YF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\xCCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xF0\x91\x90aP\xFBV[`@Qc5IR\xA3`\xE2\x1B\x81R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xD5%J\x8C\x90a\x16#\x90\x8B\x90\x8B\x90\x8E\x90`\x04\x01aTyV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16@W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x16h\x91\x90\x81\x01\x90aS3V[` \x83\x01RP\x98\x97PPPPPPPPV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xC2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xE6\x91\x90aQbV[a\x17\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06n\x90aQ\x7FV[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[```\0\x84`\x01`\x01`\xA0\x1B\x03\x16c\xC3\x91B^\x84\x86`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17s\x92\x91\x90aT\xA3V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\x90W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x17\xB8\x91\x90\x81\x01\x90aS3V[\x90P`\0\x84Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17\xD5Wa\x17\xD5aD\rV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x17\xFEW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x85Q\x81\x10\x15a\x18\xFFW\x86`\x01`\x01`\xA0\x1B\x03\x16c\x04\xECcQ\x87\x83\x81Q\x81\x10a\x18.Wa\x18.aQ\xC7V[` \x02` \x01\x01Q\x87\x86\x85\x81Q\x81\x10a\x18IWa\x18IaQ\xC7V[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18\x86\x93\x92\x91\x90\x92\x83Rc\xFF\xFF\xFF\xFF\x91\x82\x16` \x84\x01R\x16`@\x82\x01R``\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\xA3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\xC7\x91\x90aT\x13V[`\x01`\x01`\xC0\x1B\x03\x16\x82\x82\x81Q\x81\x10a\x18\xE2Wa\x18\xE2aQ\xC7V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a\x18\xF7\x81aR\xCEV[\x91PPa\x18\x04V[P\x95\x94PPPPPV[`\xD0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x19KW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdAuth1`\xD8\x1B`D\x82\x01R`d\x01a\x06nV[`\xCDTc\xFF\xFF\xFF\xFF\x16C\x14\x80\x15\x90a\x19bWPC\x15\x15[a\x19\xD4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FCan't create a task in the same `D\x82\x01R\x7Fblock as a completed task\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06nV[`@\x80Q`\xE0\x81\x01\x82R`\0\x81\x83\x01\x81\x90R``\x80\x83\x01\x81\x90R`\xA0\x83\x01R`\xC0\x82\x01R\x85\x81Rc\xFF\xFF\xFF\xFFC\x81\x16` \x80\x84\x01\x91\x90\x91R\x90\x86\x16`\x80\x83\x01R\x82Q`\x1F\x85\x01\x82\x90\x04\x82\x02\x81\x01\x82\x01\x90\x93R\x83\x83R\x90\x91\x90\x84\x90\x84\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPPP``\x82\x01R`\xCDTc\xFF\xFF\xFF\xFF\x16`@\x82\x01R`\xCE\x80Ta\x1Af\x90aT\xF7V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1A\x92\x90aT\xF7V[\x80\x15a\x1A\xDFW\x80`\x1F\x10a\x1A\xB4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1A\xDFV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1A\xC2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`\xA0\x82\x01R`\xCFTc\xFF\xFF\xFF\xFF\x90\x81\x16`\xC0\x83\x01R`\xC9T\x16\x15a\x1BqW`\xC9T`\0\x90a\x1B\x1A\x90`\x01\x90c\xFF\xFF\xFF\xFF\x16aU2V[\x90P`\x01c\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`\xCC` R`@\x90 T`\xFF\x16`\x03\x81\x11\x15a\x1BHWa\x1BHaKCV[\x14\x15a\x1BoWc\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\xCC` R`@\x90 \x80T`\xFF\x19\x16`\x02\x17\x90U[P[\x80`@Q` \x01a\x1B\x82\x91\x90aU\xA4V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 `\xC9\x80Tc\xFF\xFF\xFF\xFF\x90\x81\x16`\0\x90\x81R`\xCA\x85R\x85\x81 \x93\x90\x93U\x81T\x81\x16\x83R`\xCC\x90\x93R\x92\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U\x90T\x16\x90\x7F\x93?\xBD\xF3\x90\xBAqs\xF9\xBF\x83\xF5E\x9E\xDF\x1A\xE4\x1A\xE5\xED\xBD\xE6\xC8\xF9\xB3t\xD9y\xD3\t\0\xBE\x90a\x1C\0\x90\x84\x90aU\xA4V[`@Q\x80\x91\x03\x90\xA2`\xC9Ta\x1C\x1C\x90c\xFF\xFF\xFF\xFF\x16`\x01aV7V[`\xC9\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UPPPPPV[a\x1CEa:\xF9V[a\x1CO`\0a;SV[V[`\xCFTd\x01\0\0\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1C\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04\x17WF\x83`\xDC\x1B`D\x82\x01R`d\x01a\x06nV[`\0a\x1C\xAD`@\x85\x01` \x86\x01aE\xE0V[\x90P6`\0a\x1C\xBF``\x87\x01\x87aV_V[\x90\x92P\x90P`\0a\x1C\xD6`\xA0\x88\x01`\x80\x89\x01aE\xE0V[\x90P`\xCA`\0a\x1C\xE9` \x89\x01\x89aE\xE0V[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x87`@Q` \x01a\x1D\x15\x91\x90aV\xEAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x14a\x1D\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7Fsupplied task does not match the`D\x82\x01R\x7F one recorded in the contract\0\0\0`d\x82\x01R`\x84\x01a\x06nV[`\x01`\xCC`\0a\x1D\xB1` \x8A\x01\x8AaE\xE0V[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16`\x03\x81\x11\x15a\x1D\xDCWa\x1D\xDCaKCV[\x14a\x1D\xF9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06n\x90aW\xB1V[`\0`\xCB\x81a\x1E\x0B` \x8A\x01\x8AaE\xE0V[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x14a\x1ECW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06n\x90aW\xB1V[a\x1Em\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85aV7V[c\xFF\xFF\xFF\xFF\x16Cc\xFF\xFF\xFF\xFF\x16\x11\x15a\x1E\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FAggregator has responded to the `D\x82\x01Rltask too late`\x98\x1B`d\x82\x01R`\x84\x01a\x06nV[`\0\x86`@Q` \x01a\x1E\xF1\x91\x90aXHV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x80a\x1F\x19\x83\x87\x87\x8A\x8Ca#\x13V[`@\x80Q`\x80\x81\x01\x82Rc\xFF\xFF\xFF\xFFC\x16\x81R` \x80\x82\x01\x84\x90R\x80\x85\x01Q\x82\x84\x01R\x84Q``\x83\x01R\x91Q\x93\x95P\x91\x93P\x90\x91a\x1F[\x91\x8C\x91\x84\x91\x01aXVV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\xCB`\0\x8C`\0\x01` \x81\x01\x90a\x1F\x88\x91\x90aE\xE0V[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP\x7F\xF9\x86b;\x08\x03\x1C)\xEF\xCB\xF1\t\x81\x18A-\x9Cq\xEF}2\xF6<p\x19\x88\xD6\xBB\xA2Q\xCEf\x8A\x82`@Qa\x1F\xD7\x92\x91\x90aXVV[`@Q\x80\x91\x03\x90\xA1`\0[\x86\x81\x10\x15a xW\x85`\xFF\x16\x84` \x01Q\x82\x81Q\x81\x10a \x04Wa \x04aQ\xC7V[` \x02` \x01\x01Qa \x16\x91\x90aX\xC4V[`\x01`\x01``\x1B\x03\x16`d\x85`\0\x01Q\x83\x81Q\x81\x10a 7Wa 7aQ\xC7V[` \x02` \x01\x01Q`\x01`\x01``\x1B\x03\x16a R\x91\x90aX\xF3V[\x10\x15a fWPPPPPPPPPPPPV[\x80a p\x81aR\xCEV[\x91PPa\x1F\xE2V[P`\xA0\x8A\x015`\xD1U`\x03`\xCC`\0a \x94` \x8E\x01\x8EaE\xE0V[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16`\x03\x81\x11\x15a \xBFWa \xBFaKCV[Pa \xD2\x90P`@\x8C\x01` \x8D\x01aE\xE0V[`\xCD\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua \xF9``\x8C\x01\x8CaV_V[a!\x05\x91`\xCE\x91aBOV[Pa!\x16`\xA0\x8C\x01`\x80\x8D\x01aE\xE0V[`\xCF\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U``\x8A\x015a!B` \x8C\x01\x8CaE\xE0V[c\xFF\xFF\xFF\xFF\x16\x7F\x83x\xBE\x8A3\xCF:I9\x10\xA1n'\\\xD9j\xF4\xF0H\xC5\xEB\x1A,)b\xD4\x06ni\x7F\xEA\x80`@Q`@Q\x80\x91\x03\x90\xA3PPPPPPPPPPPV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91``\x91\x83\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x84\x81`\0\x81Q\x81\x10a!\xBCWa!\xBCaQ\xC7V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Qca\xC8\xA1/`\xE1\x1B\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90c\xC3\x91B^\x90a!\xF8\x90\x88\x90\x86\x90`\x04\x01aT\xA3V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"\x15W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\"=\x91\x90\x81\x01\x90aS3V[`\0\x81Q\x81\x10a\"OWa\"OaQ\xC7V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x88\x90Rc\xFF\xFF\xFF\xFF\x87\x81\x16`$\x83\x01R\x90\x91\x16`D\x82\x01\x81\x90R\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x89\x16\x90c\x04\xECcQ\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"\xBBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"\xDF\x91\x90aT\x13V[`\x01`\x01`\xC0\x1B\x03\x16\x90P`\0a\"\xF5\x82a;\xA5V[\x90P\x81a#\x03\x8A\x83\x8Aa\tIV[\x95P\x95PPPPP\x93P\x93\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`\0\x84a#\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R`\0\x80Q` aZM\x839\x81Q\x91R`D\x82\x01R\x7Fres: empty quorum input\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06nV[`@\x83\x01QQ\x85\x14\x80\x15a#\xA2WP`\xA0\x83\x01QQ\x85\x14[\x80\x15a#\xB2WP`\xC0\x83\x01QQ\x85\x14[\x80\x15a#\xC2WP`\xE0\x83\x01QQ\x85\x14[a$,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R`\0\x80Q` aZM\x839\x81Q\x91R`D\x82\x01R\x7Fres: input quorum length mismatc`d\x82\x01R`\r`\xFB\x1B`\x84\x82\x01R`\xA4\x01a\x06nV[\x82QQ` \x84\x01QQ\x14a$\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R`\0\x80Q` aZM\x839\x81Q\x91R\x90\x82\x01R\x7Fres: input nonsigner length mism`d\x82\x01Rc\x0C.\x8Cm`\xE3\x1B`\x84\x82\x01R`\xA4\x01a\x06nV[Cc\xFF\xFF\xFF\xFF\x16\x84c\xFF\xFF\xFF\xFF\x16\x10a%\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R`\0\x80Q` aZM\x839\x81Q\x91R`D\x82\x01R\x7Fres: invalid reference block\0\0\0\0`d\x82\x01R`\x84\x01a\x06nV[`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x80\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x90\x93R``\x80\x84R\x90\x83\x01R\x90\x86`\x01`\x01`@\x1B\x03\x81\x11\x15a%TWa%TaD\rV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a%}W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P` \x82\x01R\x86`\x01`\x01`@\x1B\x03\x81\x11\x15a%\x9BWa%\x9BaD\rV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a%\xC4W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x81R`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R\x85` \x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a%\xF8Wa%\xF8aD\rV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a&!W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x81R` \x86\x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a&AWa&AaD\rV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a&jW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x81` \x01\x81\x90RP`\0a'<\x8A\x8A\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`@\x80Qc\x9A\xA1e=`\xE0\x1B\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x93Pc\x9A\xA1e=\x92P`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a'\x13W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'7\x91\x90aY\x12V[a<qV[\x90P`\0[\x87` \x01QQ\x81\x10\x15a)\xD7Wa'\x86\x88` \x01Q\x82\x81Q\x81\x10a'gWa'gaQ\xC7V[` \x02` \x01\x01Q\x80Q`\0\x90\x81R` \x91\x82\x01Q\x90\x91R`@\x90 \x90V[\x83` \x01Q\x82\x81Q\x81\x10a'\x9CWa'\x9CaQ\xC7V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80\x15a(\\W` \x83\x01Qa'\xBD`\x01\x83aY/V[\x81Q\x81\x10a'\xCDWa'\xCDaQ\xC7V[` \x02` \x01\x01Q`\0\x1C\x83` \x01Q\x82\x81Q\x81\x10a'\xEEWa'\xEEaQ\xC7V[` \x02` \x01\x01Q`\0\x1C\x11a(\\W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R`\0\x80Q` aZM\x839\x81Q\x91R`D\x82\x01R\x7Fres: nonSignerPubkeys not sorted`d\x82\x01R`\x84\x01a\x06nV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x04\xECcQ\x84` \x01Q\x83\x81Q\x81\x10a(\xA1Wa(\xA1aQ\xC7V[` \x02` \x01\x01Q\x8B\x8B`\0\x01Q\x85\x81Q\x81\x10a(\xC0Wa(\xC0aQ\xC7V[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a(\xFD\x93\x92\x91\x90\x92\x83Rc\xFF\xFF\xFF\xFF\x91\x82\x16` \x84\x01R\x16`@\x82\x01R``\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a)\x1AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)>\x91\x90aT\x13V[`\x01`\x01`\xC0\x1B\x03\x16\x83`\0\x01Q\x82\x81Q\x81\x10a)]Wa)]aQ\xC7V[` \x02` \x01\x01\x81\x81RPPa)\xC3a\x08\xDAa)\x97\x84\x86`\0\x01Q\x85\x81Q\x81\x10a)\x89Wa)\x89aQ\xC7V[` \x02` \x01\x01Q\x16a=\x04V[\x8A` \x01Q\x84\x81Q\x81\x10a)\xADWa)\xADaQ\xC7V[` \x02` \x01\x01Qa=/\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x94P\x80a)\xCF\x81aR\xCEV[\x91PPa'AV[PPa)\xE2\x83a>\x13V[`\x97T\x90\x93P`\xFF\x16`\0\x81a)\xF9W`\0a*{V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xC4H\xFE\xB8`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a*WW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*{\x91\x90aYFV[\x90P`\0[\x8A\x81\x10\x15a0\xFAW\x82\x15a+\xDCW\x89c\xFF\xFF\xFF\xFF\x16\x82\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c$\x9A\x0CB\x8F\x8F\x86\x81\x81\x10a*\xD7Wa*\xD7aQ\xC7V[`@Q`\xE0\x85\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R\x92\x015`\xF8\x1C`\x04\x83\x01RP`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+\x17W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+;\x91\x90aYFV[a+E\x91\x90aY_V[\x10\x15a+\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`f`$\x82\x01R`\0\x80Q` aZM\x839\x81Q\x91R`D\x82\x01R\x7Fres: StakeRegistry updates must `d\x82\x01R\x7Fbe within withdrawalDelayBlocks `\x84\x82\x01Rewindow`\xD0\x1B`\xA4\x82\x01R`\xC4\x01a\x06nV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16ch\xBC\xCA\xAC\x8D\x8D\x84\x81\x81\x10a,\x1DWa,\x1DaQ\xC7V[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x8C\x8C`\xA0\x01Q\x85\x81Q\x81\x10a,AWa,AaQ\xC7V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81R`\xFF\x90\x93\x16`\x04\x84\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x84\x01R\x16`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,\x9DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\xC1\x91\x90aYwV[`\x01`\x01`@\x1B\x03\x19\x16a,\xE4\x8A`@\x01Q\x83\x81Q\x81\x10a'gWa'gaQ\xC7V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14a-\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`a`$\x82\x01R`\0\x80Q` aZM\x839\x81Q\x91R`D\x82\x01R\x7Fres: quorumApk hash in storage d`d\x82\x01R\x7Foes not match provided quorum ap`\x84\x82\x01R`k`\xF8\x1B`\xA4\x82\x01R`\xC4\x01a\x06nV[a-\xB0\x89`@\x01Q\x82\x81Q\x81\x10a-\x99Wa-\x99aQ\xC7V[` \x02` \x01\x01Q\x87a6\xF1\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x95P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xC8)LV\x8D\x8D\x84\x81\x81\x10a-\xF3Wa-\xF3aQ\xC7V[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x8C\x8C`\xC0\x01Q\x85\x81Q\x81\x10a.\x17Wa.\x17aQ\xC7V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81R`\xFF\x90\x93\x16`\x04\x84\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x84\x01R\x16`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a.sW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.\x97\x91\x90aR\x8FV[\x85` \x01Q\x82\x81Q\x81\x10a.\xADWa.\xADaQ\xC7V[`\x01`\x01``\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x82\x01R\x85\x01Q\x80Q\x82\x90\x81\x10a.\xD9Wa.\xD9aQ\xC7V[` \x02` \x01\x01Q\x85`\0\x01Q\x82\x81Q\x81\x10a.\xF7Wa.\xF7aQ\xC7V[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPP`\0\x80[\x8A` \x01QQ\x81\x10\x15a0\xE5Wa/o\x86`\0\x01Q\x82\x81Q\x81\x10a/AWa/AaQ\xC7V[` \x02` \x01\x01Q\x8F\x8F\x86\x81\x81\x10a/[Wa/[aQ\xC7V[`\x01\x92\x015`\xF8\x1C\x92\x90\x92\x1C\x81\x16\x14\x91\x90PV[\x15a0\xD3W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xF2\xBE\x94\xAE\x8F\x8F\x86\x81\x81\x10a/\xB5Wa/\xB5aQ\xC7V[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x8E\x89` \x01Q\x85\x81Q\x81\x10a/\xD9Wa/\xD9aQ\xC7V[` \x02` \x01\x01Q\x8F`\xE0\x01Q\x88\x81Q\x81\x10a/\xF7Wa/\xF7aQ\xC7V[` \x02` \x01\x01Q\x87\x81Q\x81\x10a0\x10Wa0\x10aQ\xC7V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x87\x90\x1B\x16\x81R`\xFF\x90\x94\x16`\x04\x85\x01Rc\xFF\xFF\xFF\xFF\x92\x83\x16`$\x85\x01R`D\x84\x01\x91\x90\x91R\x16`d\x82\x01R`\x84\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a0tW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a0\x98\x91\x90aR\x8FV[\x87Q\x80Q\x85\x90\x81\x10a0\xACWa0\xACaQ\xC7V[` \x02` \x01\x01\x81\x81Qa0\xC0\x91\x90aY\xA2V[`\x01`\x01``\x1B\x03\x16\x90RP`\x01\x90\x91\x01\x90[\x80a0\xDD\x81aR\xCEV[\x91PPa/\x1BV[PP\x80\x80a0\xF2\x90aR\xCEV[\x91PPa*\x80V[PPP`\0\x80a1\x14\x8C\x86\x8A``\x01Q\x8B`\x80\x01Qa\x07\xBFV[\x91P\x91P\x81a1\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R`\0\x80Q` aZM\x839\x81Q\x91R`D\x82\x01R\x7Fres: pairing precompile call fai`d\x82\x01Rb\x1B\x19Y`\xEA\x1B`\x84\x82\x01R`\xA4\x01a\x06nV[\x80a1\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R`\0\x80Q` aZM\x839\x81Q\x91R`D\x82\x01R\x7Fres: signature is invalid\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06nV[PP`\0\x87\x82` \x01Q`@Q` \x01a2\x01\x92\x91\x90aY\xC2V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x92\x9B\x92\x9AP\x91\x98PPPPPPPPPV[a23a:\xF9V[`\x01`\x01`\xA0\x1B\x03\x81\x16a2\x98W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x06nV[a\x04\xE9\x81a;SV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a2\xC1WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a2\xDBWP0;\x15\x80\x15a2\xDBWP`\0T`\xFF\x16`\x01\x14[a3>W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x06nV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a3aW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a3l\x85`\0a>\xAEV[a3u\x84a;SV[`\xCF\x80Td\x01\0\0\0\0`\x01`\xC0\x1B\x03\x19\x16d\x01\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x90\x91\x02\x91\x90\x91\x17\x90\x91U`\xD0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x84\x16\x91\x90\x91\x17\x90U\x80\x15a4\0W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a4ZW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a4~\x91\x90aP\xFBV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a4\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06n\x90aQ\x18V[`fT\x19\x81\x19`fT\x19\x16\x14a5,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06nV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x07\xB4V[`\x01`\x01`\xA0\x1B\x03\x81\x16a5\xF1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x06nV[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra6vaB\xD3V[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x80\x82\x01\x84\x90R`\0\x90\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a6\xA9Wa6\xABV[\xFE[P\x80a6\xE9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCB[][\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x06nV[PP\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra7\raB\xF1V[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R\x83Q`@\x80\x84\x01\x91\x90\x91R\x90\x84\x01Q``\x83\x01R`\0\x90\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a6\xA9WP\x80a6\xE9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCBXY\x19\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x06nV[a7\x8DaC\x0FV[P`@\x80Q`\x80\x81\x01\x82R\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81\x83\x01\x90\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED``\x83\x01R\x81R\x81Q\x80\x83\x01\x90\x92R\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x81\x01\x91\x90\x91R\x81\x01\x91\x90\x91R\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0\x80\x80a8u`\0\x80Q` aZ-\x839\x81Q\x91R\x86aQ\xDDV[\x90P[a8\x81\x81a?\x98V[\x90\x93P\x91P`\0\x80Q` aZ-\x839\x81Q\x91R\x82\x83\t\x83\x14\x15a8\xBBW`@\x80Q\x80\x82\x01\x90\x91R\x90\x81R` \x81\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80Q` aZ-\x839\x81Q\x91R`\x01\x82\x08\x90Pa8xV[`@\x80Q\x80\x82\x01\x82R\x86\x81R` \x80\x82\x01\x86\x90R\x82Q\x80\x84\x01\x90\x93R\x86\x83R\x82\x01\x84\x90R`\0\x91\x82\x91\x90a9\x07aC4V[`\0[`\x02\x81\x10\x15a:\xCCW`\0a9 \x82`\x06aX\xF3V[\x90P\x84\x82`\x02\x81\x10a94Wa94aQ\xC7V[` \x02\x01QQ\x83a9F\x83`\0aY_V[`\x0C\x81\x10a9VWa9VaQ\xC7V[` \x02\x01R\x84\x82`\x02\x81\x10a9mWa9maQ\xC7V[` \x02\x01Q` \x01Q\x83\x82`\x01a9\x84\x91\x90aY_V[`\x0C\x81\x10a9\x94Wa9\x94aQ\xC7V[` \x02\x01R\x83\x82`\x02\x81\x10a9\xABWa9\xABaQ\xC7V[` \x02\x01QQQ\x83a9\xBE\x83`\x02aY_V[`\x0C\x81\x10a9\xCEWa9\xCEaQ\xC7V[` \x02\x01R\x83\x82`\x02\x81\x10a9\xE5Wa9\xE5aQ\xC7V[` \x02\x01QQ`\x01` \x02\x01Q\x83a9\xFE\x83`\x03aY_V[`\x0C\x81\x10a:\x0EWa:\x0EaQ\xC7V[` \x02\x01R\x83\x82`\x02\x81\x10a:%Wa:%aQ\xC7V[` \x02\x01Q` \x01Q`\0`\x02\x81\x10a:@Wa:@aQ\xC7V[` \x02\x01Q\x83a:Q\x83`\x04aY_V[`\x0C\x81\x10a:aWa:aaQ\xC7V[` \x02\x01R\x83\x82`\x02\x81\x10a:xWa:xaQ\xC7V[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a:\x93Wa:\x93aQ\xC7V[` \x02\x01Q\x83a:\xA4\x83`\x05aY_V[`\x0C\x81\x10a:\xB4Wa:\xB4aQ\xC7V[` \x02\x01RP\x80a:\xC4\x81aR\xCEV[\x91PPa9\nV[Pa:\xD5aCSV[`\0` \x82a\x01\x80\x85`\x08\x8C\xFA\x91Q\x91\x9C\x91\x15\x15\x9BP\x90\x99PPPPPPPPPPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1COW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x06nV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[```\0\x80a;\xB3\x84a=\x04V[a\xFF\xFF\x16`\x01`\x01`@\x1B\x03\x81\x11\x15a;\xCEWa;\xCEaD\rV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a;\xF8W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x80[\x82Q\x82\x10\x80\x15a<\x10WPa\x01\0\x81\x10[\x15a<gW`\x01\x81\x1B\x93P\x85\x84\x16\x15a<WW\x80`\xF8\x1B\x83\x83\x81Q\x81\x10a<9Wa<9aQ\xC7V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x81`\x01\x01\x91P[a<`\x81aR\xCEV[\x90Pa;\xFFV[P\x90\x94\x93PPPPV[`\0\x80a<}\x84a@\x1AV[\x90P\x80\x83`\xFF\x16`\x01\x90\x1B\x11a<\xFBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`?`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: bitmap exceeds max value\0`d\x82\x01R`\x84\x01a\x06nV[\x90P[\x92\x91PPV[`\0\x80[\x82\x15a<\xFEWa=\x19`\x01\x84aY/V[\x90\x92\x16\x91\x80a='\x81aZ\nV[\x91PPa=\x08V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x02\0\x82a\xFF\xFF\x16\x10a=\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Roscalar-too-large`\x80\x1B`D\x82\x01R`d\x01a\x06nV[\x81a\xFF\xFF\x16`\x01\x14\x15a=\x9FWP\x81a<\xFEV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01\x81\x90R\x84\x90`\x01\x90[\x81a\xFF\xFF\x16\x86a\xFF\xFF\x16\x10a>\x08W`\x01a\xFF\xFF\x87\x16`\xFF\x83\x16\x1C\x81\x16\x14\x15a=\xEBWa=\xE8\x84\x84a6\xF1V[\x93P[a=\xF5\x83\x84a6\xF1V[\x92Pb\x01\xFF\xFE`\x01\x92\x83\x1B\x16\x91\x01a=\xBBV[P\x91\x95\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q\x15\x80\x15a>8WP` \x82\x01Q\x15[\x15a>VWPP`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01`\0\x80Q` aZ-\x839\x81Q\x91R\x84` \x01Qa>\x89\x91\x90aQ\xDDV[a>\xA1\x90`\0\x80Q` aZ-\x839\x81Q\x91RaY/V[\x90R\x92\x91PPV[\x91\x90PV[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a>\xCFWP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a?QW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x06nV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a?\x94\x82a5cV[PPV[`\0\x80\x80`\0\x80Q` aZ-\x839\x81Q\x91R`\x03`\0\x80Q` aZ-\x839\x81Q\x91R\x86`\0\x80Q` aZ-\x839\x81Q\x91R\x88\x89\t\t\x08\x90P`\0a@\x0E\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\0\x80Q` aZ-\x839\x81Q\x91RaA\xA7V[\x91\x95\x91\x94P\x90\x92PPPV[`\0a\x01\0\x82Q\x11\x15a@\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FBitmapUtils.orderedBytesArrayToB\x90\x82\x01R\x7Fitmap: orderedBytesArray is too `d\x82\x01Rclong`\xE0\x1B`\x84\x82\x01R`\xA4\x01a\x06nV[\x81Qa@\xB1WP`\0\x91\x90PV[`\0\x80\x83`\0\x81Q\x81\x10a@\xC7Wa@\xC7aQ\xC7V[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x81\x90\x1B\x92P[\x84Q\x81\x10\x15aA\x9EW\x84\x81\x81Q\x81\x10a@\xF5Wa@\xF5aQ\xC7V[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x1B\x91P\x82\x82\x11aA\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: orderedBytesArray is not `d\x82\x01Rf\x1B\xDC\x99\x19\\\x99Y`\xCA\x1B`\x84\x82\x01R`\xA4\x01a\x06nV[\x91\x81\x17\x91aA\x97\x81aR\xCEV[\x90Pa@\xDAV[P\x90\x93\x92PPPV[`\0\x80aA\xB2aCSV[aA\xBAaCqV[` \x80\x82R\x81\x81\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x88\x90R`\x80\x82\x01\x87\x90R`\xA0\x82\x01\x86\x90R\x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82\x80\x15a6\xA9WP\x82aBDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FBN254.expMod: call failure\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06nV[PQ\x95\x94PPPPPV[\x82\x80TaB[\x90aT\xF7V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82aB}W`\0\x85UaB\xC3V[\x82`\x1F\x10aB\x96W\x82\x80\x01`\xFF\x19\x825\x16\x17\x85UaB\xC3V[\x82\x80\x01`\x01\x01\x85U\x82\x15aB\xC3W\x91\x82\x01[\x82\x81\x11\x15aB\xC3W\x825\x82U\x91` \x01\x91\x90`\x01\x01\x90aB\xA8V[PaB\xCF\x92\x91PaC\x8FV[P\x90V[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80aC\"aC\xA4V[\x81R` \x01aC/aC\xA4V[\x90R\x90V[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[[\x80\x82\x11\x15aB\xCFW`\0\x81U`\x01\x01aC\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04\xE9W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aC\xE9W`\0\x80\xFD[\x815a<\xFB\x81aC\xC2V[`\0` \x82\x84\x03\x12\x15aD\x06W`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aDEWaDEaD\rV[`@R\x90V[`@Qa\x01\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aDEWaDEaD\rV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aD\x96WaD\x96aD\rV[`@R\x91\x90PV[`\0`@\x82\x84\x03\x12\x15aD\xB0W`\0\x80\xFD[aD\xB8aD#V[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12aD\xDFW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15aE\x01WaE\x01aD\rV[\x80`@RP\x80`@\x84\x01\x85\x81\x11\x15aE\x18W`\0\x80\xFD[\x84[\x81\x81\x10\x15a>\x08W\x805\x83R` \x92\x83\x01\x92\x01aE\x1AV[`\0`\x80\x82\x84\x03\x12\x15aEDW`\0\x80\xFD[aELaD#V[\x90PaEX\x83\x83aD\xCEV[\x81RaEg\x83`@\x84\x01aD\xCEV[` \x82\x01R\x92\x91PPV[`\0\x80`\0\x80a\x01 \x85\x87\x03\x12\x15aE\x89W`\0\x80\xFD[\x845\x93PaE\x9A\x86` \x87\x01aD\x9EV[\x92PaE\xA9\x86``\x87\x01aE2V[\x91PaE\xB8\x86`\xE0\x87\x01aD\x9EV[\x90P\x92\x95\x91\x94P\x92PV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x04\xE9W`\0\x80\xFD[\x805a>\xA9\x81aE\xC3V[`\0` \x82\x84\x03\x12\x15aE\xF2W`\0\x80\xFD[\x815a<\xFB\x81aE\xC3V[`\0\x80`\0``\x84\x86\x03\x12\x15aF\x12W`\0\x80\xFD[\x835aF\x1D\x81aC\xC2V[\x92P` \x84\x81\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aF:W`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12aFNW`\0\x80\xFD[\x815\x81\x81\x11\x15aF`WaF`aD\rV[aFr`\x1F\x82\x01`\x1F\x19\x16\x85\x01aDnV[\x91P\x80\x82R\x88\x84\x82\x85\x01\x01\x11\x15aF\x88W`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x94PPPPaF\xAB`@\x85\x01aE\xD5V[\x90P\x92P\x92P\x92V[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0\x80[\x86\x81\x10\x15aGJW\x83\x85\x03\x8AR\x82Q\x80Q\x80\x87R\x90\x87\x01\x90\x87\x87\x01\x90\x84[\x81\x81\x10\x15aG5W\x83Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x8A\x81\x01Q\x8B\x85\x01R`@\x90\x81\x01Q`\x01`\x01``\x1B\x03\x16\x90\x84\x01R\x92\x89\x01\x92``\x90\x92\x01\x91`\x01\x01aF\xF1V[PP\x9A\x87\x01\x9A\x95PP\x91\x85\x01\x91`\x01\x01aF\xD3V[P\x92\x98\x97PPPPPPPPV[` \x81R`\0aGk` \x83\x01\x84aF\xB4V[\x93\x92PPPV[\x80\x15\x15\x81\x14a\x04\xE9W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aG\x92W`\0\x80\xFD[\x815a<\xFB\x81aGrV[`\0\x80\x83`\x1F\x84\x01\x12aG\xAFW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aG\xC6W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aG\xDEW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\x80\x87\x89\x03\x12\x15aG\xFEW`\0\x80\xFD[\x865aH\t\x81aC\xC2V[\x95P` \x87\x015aH\x19\x81aE\xC3V[\x94P`@\x87\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aH5W`\0\x80\xFD[aHA\x8A\x83\x8B\x01aG\x9DV[\x90\x96P\x94P``\x89\x015\x91P\x80\x82\x11\x15aHZW`\0\x80\xFD[\x81\x89\x01\x91P\x89`\x1F\x83\x01\x12aHnW`\0\x80\xFD[\x815\x81\x81\x11\x15aH}W`\0\x80\xFD[\x8A` \x82`\x05\x1B\x85\x01\x01\x11\x15aH\x92W`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92\x95P\x92\x95P\x92\x95V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aH\xDEW\x81Qc\xFF\xFF\xFF\xFF\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aH\xBCV[P\x94\x95\x94PPPPPV[`\0` \x80\x83R\x83Q`\x80\x82\x85\x01RaI\x05`\xA0\x85\x01\x82aH\xA8V[\x90P\x81\x85\x01Q`\x1F\x19\x80\x86\x84\x03\x01`@\x87\x01RaI\"\x83\x83aH\xA8V[\x92P`@\x87\x01Q\x91P\x80\x86\x84\x03\x01``\x87\x01RaI?\x83\x83aH\xA8V[``\x88\x01Q\x87\x82\x03\x83\x01`\x80\x89\x01R\x80Q\x80\x83R\x91\x94P\x85\x01\x92P\x84\x84\x01\x90`\x05\x81\x90\x1B\x85\x01\x86\x01`\0[\x82\x81\x10\x15aI\x96W\x84\x87\x83\x03\x01\x84RaI\x84\x82\x87QaH\xA8V[\x95\x88\x01\x95\x93\x88\x01\x93\x91P`\x01\x01aIjV[P\x99\x98PPPPPPPPPV[`\xFF\x81\x16\x81\x14a\x04\xE9W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aI\xC5W`\0\x80\xFD[\x815a<\xFB\x81aI\xA4V[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15aI\xE9WaI\xE9aD\rV[P`\x05\x1B` \x01\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15aJ\x08W`\0\x80\xFD[\x835aJ\x13\x81aC\xC2V[\x92P` \x84\x81\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aJ/W`\0\x80\xFD[\x85\x01`\x1F\x81\x01\x87\x13aJ@W`\0\x80\xFD[\x805aJSaJN\x82aI\xD0V[aDnV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x89\x83\x11\x15aJrW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aJ\x90W\x835\x82R\x92\x84\x01\x92\x90\x84\x01\x90aJwV[\x80\x96PPPPPPaF\xAB`@\x85\x01aE\xD5V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aJ\xDCW\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aJ\xC0V[P\x90\x96\x95PPPPPPV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15aJ\xFEW`\0\x80\xFD[\x845\x93P` \x85\x015aK\x10\x81aE\xC3V[\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aK+W`\0\x80\xFD[aK7\x87\x82\x88\x01aG\x9DV[\x95\x98\x94\x97P\x95PPPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x04\x83\x10aK{WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`\0` \x82\x84\x03\x12\x15aK\x93W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aK\xA9W`\0\x80\xFD[\x82\x01a\x01@\x81\x85\x03\x12\x15a<\xFBW`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12aK\xCDW`\0\x80\xFD[\x815` aK\xDDaJN\x83aI\xD0V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aK\xFCW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aL W\x805aL\x13\x81aE\xC3V[\x83R\x91\x83\x01\x91\x83\x01aL\0V[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12aL<W`\0\x80\xFD[\x815` aLLaJN\x83aI\xD0V[\x82\x81R`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aLkW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aL WaL\x81\x88\x82aD\x9EV[\x83R\x91\x83\x01\x91`@\x01aLoV[`\0\x82`\x1F\x83\x01\x12aL\xA0W`\0\x80\xFD[\x815` aL\xB0aJN\x83aI\xD0V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aL\xCFW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aL W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aL\xF2W`\0\x80\x81\xFD[aM\0\x89\x86\x83\x8B\x01\x01aK\xBCV[\x84RP\x91\x83\x01\x91\x83\x01aL\xD3V[`\0a\x02\x80\x82\x84\x03\x12\x15aM!W`\0\x80\xFD[aM)aDKV[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aMBW`\0\x80\xFD[aMN\x85\x83\x86\x01aK\xBCV[\x83R` \x84\x015\x91P\x80\x82\x11\x15aMdW`\0\x80\xFD[aMp\x85\x83\x86\x01aL+V[` \x84\x01R`@\x84\x015\x91P\x80\x82\x11\x15aM\x89W`\0\x80\xFD[aM\x95\x85\x83\x86\x01aL+V[`@\x84\x01RaM\xA7\x85``\x86\x01aE2V[``\x84\x01RaM\xB9\x85`\xE0\x86\x01aD\x9EV[`\x80\x84\x01Ra\x01 \x91P\x81\x84\x015\x81\x81\x11\x15aM\xD4W`\0\x80\xFD[aM\xE0\x86\x82\x87\x01aK\xBCV[`\xA0\x85\x01RPa\x01@\x80\x85\x015\x82\x81\x11\x15aM\xFAW`\0\x80\xFD[aN\x06\x87\x82\x88\x01aK\xBCV[`\xC0\x86\x01RPa\x01`\x80\x86\x015\x83\x81\x11\x15aN W`\0\x80\xFD[aN,\x88\x82\x89\x01aL\x8FV[`\xE0\x87\x01RPa\x01\x80\x86\x015\x83\x81\x11\x15aNEW`\0\x80\xFD[aNQ\x88\x82\x89\x01aK\xBCV[a\x01\0\x87\x01RPa\x01\xA0\x86\x015\x83\x81\x11\x15aNkW`\0\x80\xFD[aNw\x88\x82\x89\x01aL+V[\x85\x87\x01RPaN\x8A\x87a\x01\xC0\x88\x01aE2V[\x82\x86\x01RaN\x9C\x87a\x02@\x88\x01aD\x9EV[\x81\x86\x01RPPPP\x92\x91PPV[`\0\x80`\0\x83\x85\x03a\x01\0\x81\x12\x15aN\xC1W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aN\xD8W`\0\x80\xFD[\x90\x86\x01\x90`\xE0\x82\x89\x03\x12\x15aN\xECW`\0\x80\xFD[\x81\x95P`\xC0`\x1F\x19\x84\x01\x12\x15aO\x01W`\0\x80\xFD[` \x87\x01\x94P`\xE0\x87\x015\x92P\x80\x83\x11\x15aO\x1BW`\0\x80\xFD[PPaO)\x86\x82\x87\x01aM\x0EV[\x91PP\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15aOHW`\0\x80\xFD[\x835aOS\x81aC\xC2V[\x92P` \x84\x015\x91P`@\x84\x015aOj\x81aE\xC3V[\x80\x91PP\x92P\x92P\x92V[\x82\x81R`@` \x82\x01R`\0aO\x8E`@\x83\x01\x84aF\xB4V[\x94\x93PPPPV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15aO\xAEW`\0\x80\xFD[\x855\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aO\xCCW`\0\x80\xFD[aO\xD8\x89\x83\x8A\x01aG\x9DV[\x90\x96P\x94P`@\x88\x015\x91PaO\xED\x82aE\xC3V[\x90\x92P``\x87\x015\x90\x80\x82\x11\x15aP\x03W`\0\x80\xFD[PaP\x10\x88\x82\x89\x01aM\x0EV[\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aH\xDEW\x81Q`\x01`\x01``\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aP1V[`@\x81R`\0\x83Q`@\x80\x84\x01RaPq`\x80\x84\x01\x82aP\x1DV[\x90P` \x85\x01Q`?\x19\x84\x83\x03\x01``\x85\x01RaP\x8E\x82\x82aP\x1DV[\x92PPP\x82` \x83\x01R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aP\xB5W`\0\x80\xFD[\x845aP\xC0\x81aC\xC2V[\x93P` \x85\x015aP\xD0\x81aC\xC2V[\x92P`@\x85\x015aP\xE0\x81aC\xC2V[\x91P``\x85\x015aP\xF0\x81aC\xC2V[\x93\x96\x92\x95P\x90\x93PPV[`\0` \x82\x84\x03\x12\x15aQ\rW`\0\x80\xFD[\x81Qa<\xFB\x81aC\xC2V[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15aQtW`\0\x80\xFD[\x81Qa<\xFB\x81aGrV[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x82aQ\xFAWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\0` \x80\x83\x85\x03\x12\x15aR\x12W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aR(W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aR9W`\0\x80\xFD[\x80QaRGaJN\x82aI\xD0V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15aRfW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aR\x84W\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90aRkV[\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15aR\xA1W`\0\x80\xFD[\x81Q`\x01`\x01``\x1B\x03\x81\x16\x81\x14a<\xFBW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15aR\xE2WaR\xE2aR\xB8V[P`\x01\x01\x90V[c\xFF\xFF\xFF\xFF\x84\x16\x81R`@` \x82\x01\x81\x90R\x81\x01\x82\x90R`\0`\x01`\x01`\xFB\x1B\x03\x83\x11\x15aS\x16W`\0\x80\xFD[\x82`\x05\x1B\x80\x85``\x85\x017`\0\x92\x01``\x01\x91\x82RP\x93\x92PPPV[`\0` \x80\x83\x85\x03\x12\x15aSFW`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aS\\W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aSmW`\0\x80\xFD[\x80QaS{aJN\x82aI\xD0V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15aS\x9AW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aR\x84W\x83QaS\xB2\x81aE\xC3V[\x82R\x92\x84\x01\x92\x90\x84\x01\x90aS\x9FV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[c\xFF\xFF\xFF\xFF\x84\x16\x81R`@` \x82\x01R`\0aT\n`@\x83\x01\x84\x86aS\xC1V[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15aT%W`\0\x80\xFD[\x81Q`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x14a<\xFBW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aTNW`\0\x80\xFD[\x81Qa<\xFB\x81aE\xC3V[`\0`\xFF\x82\x16`\xFF\x81\x14\x15aTpWaTpaR\xB8V[`\x01\x01\x92\x91PPV[`@\x81R`\0aT\x8D`@\x83\x01\x85\x87aS\xC1V[\x90Pc\xFF\xFF\xFF\xFF\x83\x16` \x83\x01R\x94\x93PPPPV[`\0`@\x82\x01c\xFF\xFF\xFF\xFF\x85\x16\x83R` `@\x81\x85\x01R\x81\x85Q\x80\x84R``\x86\x01\x91P\x82\x87\x01\x93P`\0[\x81\x81\x10\x15aT\xEAW\x84Q\x83R\x93\x83\x01\x93\x91\x83\x01\x91`\x01\x01aT\xCEV[P\x90\x97\x96PPPPPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80aU\x0BW`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15aU,WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0c\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15aUOWaUOaR\xB8V[\x03\x93\x92PPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15aU}W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01aUaV[\x81\x81\x11\x15aU\x8FW`\0` \x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R\x81Q` \x82\x01R`\0` \x83\x01Qc\xFF\xFF\xFF\xFF\x80\x82\x16`@\x85\x01R\x80`@\x86\x01Q\x16``\x85\x01R``\x85\x01Q\x91P`\xE0`\x80\x85\x01RaU\xEBa\x01\0\x85\x01\x83aUWV[\x91P\x80`\x80\x86\x01Q\x16`\xA0\x85\x01RP`\xA0\x84\x01Q`\x1F\x19\x84\x83\x03\x01`\xC0\x85\x01RaV\x15\x82\x82aUWV[\x91PP`\xC0\x84\x01QaV/`\xE0\x85\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P\x93\x92PPPV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aVVWaVVaR\xB8V[\x01\x94\x93PPPPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aVvW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aV\x90W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15aG\xDEW`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aV\xBCW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aV\xDBW`\0\x80\xFD[\x806\x03\x83\x13\x15aG\xDEW`\0\x80\xFD[` \x81R\x815` \x82\x01R`\0` \x83\x015aW\x05\x81aE\xC3V[c\xFF\xFF\xFF\xFF\x81\x16`@\x84\x01RP`@\x83\x015aW \x81aE\xC3V[c\xFF\xFF\xFF\xFF\x81\x16``\x84\x01RPaW:``\x84\x01\x84aV\xA5V[`\xE0`\x80\x85\x01RaWPa\x01\0\x85\x01\x82\x84aS\xC1V[\x91PPaW_`\x80\x85\x01aE\xD5V[c\xFF\xFF\xFF\xFF\x81\x16`\xA0\x85\x01RPaWy`\xA0\x85\x01\x85aV\xA5V[\x84\x83\x03`\x1F\x19\x01`\xC0\x86\x01RaW\x90\x83\x82\x84aS\xC1V[\x92PPPaW\xA0`\xC0\x85\x01aE\xD5V[c\xFF\xFF\xFF\xFF\x81\x16`\xE0\x85\x01RaV/V[` \x80\x82R`,\x90\x82\x01R\x7FAggregator has already responded`@\x82\x01Rk to the task`\xA0\x1B``\x82\x01R`\x80\x01\x90V[\x805aX\x08\x81aE\xC3V[c\xFF\xFF\xFF\xFF\x81\x16\x83RP` \x81\x015` \x83\x01R`@\x81\x015`@\x83\x01R``\x81\x015``\x83\x01R`\x80\x81\x015`\x80\x83\x01R`\xA0\x81\x015`\xA0\x83\x01RPPV[`\xC0\x81\x01a<\xFE\x82\x84aW\xFDV[aX`\x81\x84aW\xFDV[`\xE0`\xC0\x82\x01Rc\xFF\xFF\xFF\xFF\x82Q\x16`\xE0\x82\x01R` \x82\x01Qa\x01\0\x82\x01R`\0`@\x83\x01Q`\x80a\x01 \x84\x01RaX\x9Ca\x01`\x84\x01\x82aP\x1DV[\x90P``\x84\x01Q`\xDF\x19\x84\x83\x03\x01a\x01@\x85\x01RaX\xBA\x82\x82aP\x1DV[\x96\x95PPPPPPV[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aX\xEAWaX\xEAaR\xB8V[\x02\x94\x93PPPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15aY\rWaY\raR\xB8V[P\x02\x90V[`\0` \x82\x84\x03\x12\x15aY$W`\0\x80\xFD[\x81Qa<\xFB\x81aI\xA4V[`\0\x82\x82\x10\x15aYAWaYAaR\xB8V[P\x03\x90V[`\0` \x82\x84\x03\x12\x15aYXW`\0\x80\xFD[PQ\x91\x90PV[`\0\x82\x19\x82\x11\x15aYrWaYraR\xB8V[P\x01\x90V[`\0` \x82\x84\x03\x12\x15aY\x89W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16\x81\x14a<\xFBW`\0\x80\xFD[`\0`\x01`\x01``\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15aUOWaUOaR\xB8V[c\xFF\xFF\xFF\xFF`\xE0\x1B\x83`\xE0\x1B\x16\x81R`\0`\x04\x82\x01\x83Q` \x80\x86\x01`\0[\x83\x81\x10\x15aY\xFDW\x81Q\x85R\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01aY\xE1V[P\x92\x97\x96PPPPPPPV[`\0a\xFF\xFF\x80\x83\x16\x81\x81\x14\x15aZ\"WaZ\"aR\xB8V[`\x01\x01\x93\x92PPPV\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDGBLSSignatureChecker.checkSignatu\xA2dipfsX\"\x12 \xB2\xBF\xFEk\xEDN\x9A\xC4\xB0`\xD0\x1DJ0\xF1\x98D\xF0\x93{v|\xC0\x95\xFF\xD7\xE6$\xEDP\xE6\x07dsolcC\0\x08\x0C\x003";
    /// The bytecode of the contract.
    pub static FINALIZERTASKMANAGER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\x11W`\x005`\xE0\x1C\x80cm\x14\xA9\x87\x11a\x01%W\x80c\xC3\xAF\x83\x13\x11a\0\xADW\x80c\xDF\\\xF7#\x11a\0|W\x80c\xDF\\\xF7#\x14a\x05AW\x80c\xF2\xFD\xE3\x8B\x14a\x05hW\x80c\xF5\xC9\x89\x9D\x14a\x05{W\x80c\xF8\xC8v^\x14a\x05\xA1W\x80c\xFA\xBC\x1C\xBC\x14a\x05\xB4W`\0\x80\xFD[\x80c\xC3\xAF\x83\x13\x14a\x04\xDBW\x80c\xCA\xB0\xE6\xFD\x14a\x04\xECW\x80c\xCE\xFD\xC1\xD4\x14a\x04\xFFW\x80c\xDBg\xF7w\x14a\x05 W`\0\x80\xFD[\x80c\x88o\x11\x95\x11a\0\xF4W\x80c\x88o\x11\x95\x14a\x04jW\x80c\x8B\0\xCE|\x14a\x04}W\x80c\x8D\xA5\xCB[\x14a\x04\x8DW\x80c\x99\xDB\xA0\xC4\x14a\x04\x9EW\x80c\xB9\x8D\t\x08\x14a\x04\xCEW`\0\x80\xFD[\x80cm\x14\xA9\x87\x14a\x04\tW\x80cqP\x18\xA6\x14a\x040W\x80cr\xD1\x8E\x8D\x14a\x048W\x80cz\xFA\x1E\xED\x14a\x04WW`\0\x80\xFD[\x80cAl~^\x11a\x01\xA8W\x80c\\\x15Vb\x11a\x01wW\x80c\\\x15Vb\x14a\x03\x80W\x80c\\\x97Z\xBB\x14a\x03\xA0W\x80c]\xF4YF\x14a\x03\xA8W\x80ch0H5\x14a\x03\xCFW\x80ck\x92x~\x14a\x03\xF6W`\0\x80\xFD[\x80cAl~^\x14a\x03\x12W\x80cOs\x9Ft\x14a\x03%W\x80cY\\jg\x14a\x03EW\x80cZ\xC8j\xB7\x14a\x03MW`\0\x80\xFD[\x80c$Z{\xFC\x11a\x01\xE4W\x80c$Z{\xFC\x14a\x02\x7FW\x80c,\xB2#\xD5\x14a\x02\xB2W\x80c-\x89\xF6\xFC\x14a\x02\xD2W\x80c5c\xB0\xD1\x14a\x02\xF2W`\0\x80\xFD[\x80c\x03s@\x8D\x14a\x02\x16W\x80c\x10\xD6z/\x14a\x02-W\x80c\x13d9\xDD\x14a\x02BW\x80c\x17\x1F\x1D[\x14a\x02UW[`\0\x80\xFD[`\xD1T[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02@a\x02;6`\x04aC\xD7V[a\x05\xC7V[\0[a\x02@a\x02P6`\x04aC\xF4V[a\x06\x80V[a\x02ha\x02c6`\x04aErV[a\x07\xBFV[`@\x80Q\x92\x15\x15\x83R\x90\x15\x15` \x83\x01R\x01a\x02$V[`\xCFTa\x02\x9A\x90d\x01\0\0\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02$V[a\x02\x1Aa\x02\xC06`\x04aE\xE0V[`\xCB` R`\0\x90\x81R`@\x90 T\x81V[a\x02\x1Aa\x02\xE06`\x04aE\xE0V[`\xCA` R`\0\x90\x81R`@\x90 T\x81V[a\x03\x05a\x03\x006`\x04aE\xFDV[a\tIV[`@Qa\x02$\x91\x90aGXV[a\x02@a\x03 6`\x04aG\x80V[a\r\xDFV[a\x038a\x0336`\x04aG\xE5V[a\x0FTV[`@Qa\x02$\x91\x90aH\xE9V[a\x02@a\x16zV[a\x03pa\x03[6`\x04aI\xB3V[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`@Q\x90\x15\x15\x81R` \x01a\x02$V[a\x03\x93a\x03\x8E6`\x04aI\xF3V[a\x17AV[`@Qa\x02$\x91\x90aJ\xA4V[`fTa\x02\x1AV[a\x02\x9A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\x9A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02@a\x04\x046`\x04aJ\xE8V[a\x19\tV[a\x02\x9A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02@a\x1C=V[`\xC9Tc\xFF\xFF\xFF\xFF\x16[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02$V[`\xD0Ta\x02\x9A\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`eTa\x02\x9A\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\xC9Ta\x04B\x90c\xFF\xFF\xFF\xFF\x16\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x02\x9AV[a\x04\xC1a\x04\xAC6`\x04aE\xE0V[`\xCC` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Qa\x02$\x91\x90aKYV[`\x97Ta\x03p\x90`\xFF\x16\x81V[a\x02@a\x04\xE96`\x04aK\x81V[PV[a\x02@a\x04\xFA6`\x04aN\xAAV[a\x1CQV[a\x05\x12a\x05\r6`\x04aO3V[a!\x81V[`@Qa\x02$\x92\x91\x90aOuV[a\x053a\x05.6`\x04aO\x96V[a#\x13V[`@Qa\x02$\x92\x91\x90aPVV[a\x02\x9A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02@a\x05v6`\x04aC\xD7V[a2+V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x04BV[a\x02@a\x05\xAF6`\x04aP\x9FV[a2\xA1V[a\x02@a\x05\xC26`\x04aC\xF4V[a4\x07V[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x1AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06>\x91\x90aP\xFBV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x06wW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06n\x90aQ\x18V[`@Q\x80\x91\x03\x90\xFD[a\x04\xE9\x81a5cV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xC8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xEC\x91\x90aQbV[a\x07\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06n\x90aQ\x7FV[`fT\x81\x81\x16\x14a\x07\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06nV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`\0\x80`\0\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x87\x87`\0\x01Q\x88` \x01Q\x88`\0\x01Q`\0`\x02\x81\x10a\x08\x07Wa\x08\x07aQ\xC7V[` \x02\x01Q\x89Q`\x01` \x02\x01Q\x8A` \x01Q`\0`\x02\x81\x10a\x08,Wa\x08,aQ\xC7V[` \x02\x01Q\x8B` \x01Q`\x01`\x02\x81\x10a\x08HWa\x08HaQ\xC7V[` \x90\x81\x02\x91\x90\x91\x01Q\x8CQ\x8D\x83\x01Q`@Qa\x08\xA5\x9A\x99\x98\x97\x96\x95\x94\x01\x98\x89R` \x89\x01\x97\x90\x97R`@\x88\x01\x95\x90\x95R``\x87\x01\x93\x90\x93R`\x80\x86\x01\x91\x90\x91R`\xA0\x85\x01R`\xC0\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01Ra\x01 \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\0\x1Ca\x08\xC8\x91\x90aQ\xDDV[\x90Pa\t;a\x08\xE1a\x08\xDA\x88\x84a6ZV[\x86\x90a6\xF1V[a\x08\xE9a7\x85V[a\t1a\t\"\x85a\t\x1C`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x01\x82R`\x02\x90\x82\x01R\x90V[\x90a6ZV[a\t+\x8Ca8EV[\x90a6\xF1V[\x88b\x01\xD4\xC0a8\xD5V[\x90\x98\x90\x97P\x95PPPPPPV[```\0\x84`\x01`\x01`\xA0\x1B\x03\x16ch0H5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\x8BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xAF\x91\x90aP\xFBV[\x90P`\0\x85`\x01`\x01`\xA0\x1B\x03\x16c\x9E\x99#\xC2`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xF1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x15\x91\x90aP\xFBV[\x90P`\0\x86`\x01`\x01`\xA0\x1B\x03\x16c]\xF4YF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\nWW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n{\x91\x90aP\xFBV[\x90P`\0\x86Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\x98Wa\n\x98aD\rV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\n\xCBW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\n\xB6W\x90P[P\x90P`\0[\x87Q\x81\x10\x15a\r\xD3W`\0\x88\x82\x81Q\x81\x10a\n\xEEWa\n\xEEaQ\xC7V[\x01` \x01Q`@Qc\x89\x02bE`\xE0\x1B\x81R`\xF8\x91\x90\x91\x1C`\x04\x82\x01\x81\x90Rc\xFF\xFF\xFF\xFF\x8A\x16`$\x83\x01R\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x87\x16\x90c\x89\x02bE\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0BOW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0Bw\x91\x90\x81\x01\x90aQ\xFFV[\x90P\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0B\x92Wa\x0B\x92aD\rV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0B\xDDW\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a\x0B\xB0W\x90P[P\x84\x84\x81Q\x81\x10a\x0B\xF0Wa\x0B\xF0aQ\xC7V[` \x02` \x01\x01\x81\x90RP`\0[\x81Q\x81\x10\x15a\r\xBDW`@Q\x80``\x01`@R\x80\x87`\x01`\x01`\xA0\x1B\x03\x16cG\xB3\x14\xE8\x85\x85\x81Q\x81\x10a\x0C3Wa\x0C3aQ\xC7V[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0CY\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CvW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x9A\x91\x90aP\xFBV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x83\x81Q\x81\x10a\x0C\xBAWa\x0C\xBAaQ\xC7V[` \x02` \x01\x01Q\x81R` \x01\x89`\x01`\x01`\xA0\x1B\x03\x16c\xFA(\xC6'\x85\x85\x81Q\x81\x10a\x0C\xE8Wa\x0C\xE8aQ\xC7V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x04\x81\x01\x91\x90\x91R`\xFF\x88\x16`$\x82\x01Rc\xFF\xFF\xFF\xFF\x8F\x16`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\rDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\rh\x91\x90aR\x8FV[`\x01`\x01``\x1B\x03\x16\x81RP\x85\x85\x81Q\x81\x10a\r\x86Wa\r\x86aQ\xC7V[` \x02` \x01\x01Q\x82\x81Q\x81\x10a\r\x9FWa\r\x9FaQ\xC7V[` \x02` \x01\x01\x81\x90RP\x80\x80a\r\xB5\x90aR\xCEV[\x91PPa\x0B\xFEV[PPP\x80\x80a\r\xCB\x90aR\xCEV[\x91PPa\n\xD1V[P\x97\x96PPPPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E=W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Ea\x91\x90aP\xFBV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0F\rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R\x7FBLSSignatureChecker.onlyCoordina`D\x82\x01R\x7FtorOwner: caller is not the owne`d\x82\x01R\x7Fr of the registryCoordinator\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06nV[`\x97\x80T`\xFF\x19\x16\x82\x15\x15\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F@\xE4\xED\x88\n)\xE0\xF6\xDD\xCE0tW\xFBu\xCD\xDFO\xEE\xF7\xD3\xEC\xB00\x1B\xFD\xF4\x97j\x0E-\xFC\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[a\x0F\x7F`@Q\x80`\x80\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`\0\x87`\x01`\x01`\xA0\x1B\x03\x16ch0H5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xBFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xE3\x91\x90aP\xFBV[\x90Pa\x10\x10`@Q\x80`\x80\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`@Qca\xC8\xA1/`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90c\xC3\x91B^\x90a\x10@\x90\x8B\x90\x89\x90\x89\x90`\x04\x01aR\xE9V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10]W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x10\x85\x91\x90\x81\x01\x90aS3V[\x81R`@Qc@\xE0:\x81`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\x81\xC0u\x02\x90a\x10\xB7\x90\x8B\x90\x8B\x90\x8B\x90`\x04\x01aS\xEAV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xD4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x10\xFC\x91\x90\x81\x01\x90aS3V[`@\x82\x01R\x85`\x01`\x01`@\x1B\x03\x81\x11\x15a\x11\x19Wa\x11\x19aD\rV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x11LW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x117W\x90P[P``\x82\x01R`\0[`\xFF\x81\x16\x87\x11\x15a\x15\x8BW`\0\x85`\x01`\x01`@\x1B\x03\x81\x11\x15a\x11zWa\x11zaD\rV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x11\xA3W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x83``\x01Q\x83`\xFF\x16\x81Q\x81\x10a\x11\xBDWa\x11\xBDaQ\xC7V[` \x02` \x01\x01\x81\x90RP`\0[\x86\x81\x10\x15a\x14\x8BW`\0\x8C`\x01`\x01`\xA0\x1B\x03\x16c\x04\xECcQ\x8A\x8A\x85\x81\x81\x10a\x11\xF6Wa\x11\xF6aQ\xC7V[\x90P` \x02\x015\x8E\x88`\0\x01Q\x86\x81Q\x81\x10a\x12\x14Wa\x12\x14aQ\xC7V[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x12Q\x93\x92\x91\x90\x92\x83Rc\xFF\xFF\xFF\xFF\x91\x82\x16` \x84\x01R\x16`@\x82\x01R``\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12nW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\x92\x91\x90aT\x13V[\x90P`\x01`\x01`\xC0\x1B\x03\x81\x16a\x136W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R\x7FOperatorStateRetriever.getCheckS`D\x82\x01R\x7FignaturesIndices: operator must `d\x82\x01R\x7Fbe registered at blocknumber\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06nV[\x8A\x8A\x85`\xFF\x16\x81\x81\x10a\x13KWa\x13KaQ\xC7V[`\x01`\x01`\xC0\x1B\x03\x84\x16\x92\x015`\xF8\x1C\x91\x90\x91\x1C`\x01\x90\x81\x16\x14\x15\x90Pa\x14xW\x85`\x01`\x01`\xA0\x1B\x03\x16c\xDD\x98F\xB9\x8A\x8A\x85\x81\x81\x10a\x13\x8DWa\x13\x8DaQ\xC7V[\x90P` \x02\x015\x8D\x8D\x88`\xFF\x16\x81\x81\x10a\x13\xA9Wa\x13\xA9aQ\xC7V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x87\x90\x1B\x16\x81R`\x04\x81\x01\x94\x90\x94R\x91\x90\x91\x015`\xF8\x1C`$\x83\x01RPc\xFF\xFF\xFF\xFF\x8F\x16`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xFFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14#\x91\x90aT<V[\x85``\x01Q\x85`\xFF\x16\x81Q\x81\x10a\x14<Wa\x14<aQ\xC7V[` \x02` \x01\x01Q\x84\x81Q\x81\x10a\x14UWa\x14UaQ\xC7V[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x82a\x14t\x81aR\xCEV[\x93PP[P\x80a\x14\x83\x81aR\xCEV[\x91PPa\x11\xCBV[P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14\xA6Wa\x14\xA6aD\rV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x14\xCFW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a\x15PW\x84``\x01Q\x84`\xFF\x16\x81Q\x81\x10a\x14\xF6Wa\x14\xF6aQ\xC7V[` \x02` \x01\x01Q\x81\x81Q\x81\x10a\x15\x0FWa\x15\x0FaQ\xC7V[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a\x15)Wa\x15)aQ\xC7V[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x15H\x81aR\xCEV[\x91PPa\x14\xD5V[P\x80\x84``\x01Q\x84`\xFF\x16\x81Q\x81\x10a\x15kWa\x15kaQ\xC7V[` \x02` \x01\x01\x81\x90RPPP\x80\x80a\x15\x83\x90aTYV[\x91PPa\x11UV[P`\0\x89`\x01`\x01`\xA0\x1B\x03\x16c]\xF4YF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\xCCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xF0\x91\x90aP\xFBV[`@Qc5IR\xA3`\xE2\x1B\x81R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xD5%J\x8C\x90a\x16#\x90\x8B\x90\x8B\x90\x8E\x90`\x04\x01aTyV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16@W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x16h\x91\x90\x81\x01\x90aS3V[` \x83\x01RP\x98\x97PPPPPPPPV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xC2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xE6\x91\x90aQbV[a\x17\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06n\x90aQ\x7FV[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[```\0\x84`\x01`\x01`\xA0\x1B\x03\x16c\xC3\x91B^\x84\x86`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17s\x92\x91\x90aT\xA3V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\x90W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x17\xB8\x91\x90\x81\x01\x90aS3V[\x90P`\0\x84Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17\xD5Wa\x17\xD5aD\rV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x17\xFEW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x85Q\x81\x10\x15a\x18\xFFW\x86`\x01`\x01`\xA0\x1B\x03\x16c\x04\xECcQ\x87\x83\x81Q\x81\x10a\x18.Wa\x18.aQ\xC7V[` \x02` \x01\x01Q\x87\x86\x85\x81Q\x81\x10a\x18IWa\x18IaQ\xC7V[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18\x86\x93\x92\x91\x90\x92\x83Rc\xFF\xFF\xFF\xFF\x91\x82\x16` \x84\x01R\x16`@\x82\x01R``\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\xA3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\xC7\x91\x90aT\x13V[`\x01`\x01`\xC0\x1B\x03\x16\x82\x82\x81Q\x81\x10a\x18\xE2Wa\x18\xE2aQ\xC7V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a\x18\xF7\x81aR\xCEV[\x91PPa\x18\x04V[P\x95\x94PPPPPV[`\xD0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x19KW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdAuth1`\xD8\x1B`D\x82\x01R`d\x01a\x06nV[`\xCDTc\xFF\xFF\xFF\xFF\x16C\x14\x80\x15\x90a\x19bWPC\x15\x15[a\x19\xD4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FCan't create a task in the same `D\x82\x01R\x7Fblock as a completed task\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06nV[`@\x80Q`\xE0\x81\x01\x82R`\0\x81\x83\x01\x81\x90R``\x80\x83\x01\x81\x90R`\xA0\x83\x01R`\xC0\x82\x01R\x85\x81Rc\xFF\xFF\xFF\xFFC\x81\x16` \x80\x84\x01\x91\x90\x91R\x90\x86\x16`\x80\x83\x01R\x82Q`\x1F\x85\x01\x82\x90\x04\x82\x02\x81\x01\x82\x01\x90\x93R\x83\x83R\x90\x91\x90\x84\x90\x84\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPPP``\x82\x01R`\xCDTc\xFF\xFF\xFF\xFF\x16`@\x82\x01R`\xCE\x80Ta\x1Af\x90aT\xF7V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1A\x92\x90aT\xF7V[\x80\x15a\x1A\xDFW\x80`\x1F\x10a\x1A\xB4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1A\xDFV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1A\xC2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`\xA0\x82\x01R`\xCFTc\xFF\xFF\xFF\xFF\x90\x81\x16`\xC0\x83\x01R`\xC9T\x16\x15a\x1BqW`\xC9T`\0\x90a\x1B\x1A\x90`\x01\x90c\xFF\xFF\xFF\xFF\x16aU2V[\x90P`\x01c\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`\xCC` R`@\x90 T`\xFF\x16`\x03\x81\x11\x15a\x1BHWa\x1BHaKCV[\x14\x15a\x1BoWc\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\xCC` R`@\x90 \x80T`\xFF\x19\x16`\x02\x17\x90U[P[\x80`@Q` \x01a\x1B\x82\x91\x90aU\xA4V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 `\xC9\x80Tc\xFF\xFF\xFF\xFF\x90\x81\x16`\0\x90\x81R`\xCA\x85R\x85\x81 \x93\x90\x93U\x81T\x81\x16\x83R`\xCC\x90\x93R\x92\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U\x90T\x16\x90\x7F\x93?\xBD\xF3\x90\xBAqs\xF9\xBF\x83\xF5E\x9E\xDF\x1A\xE4\x1A\xE5\xED\xBD\xE6\xC8\xF9\xB3t\xD9y\xD3\t\0\xBE\x90a\x1C\0\x90\x84\x90aU\xA4V[`@Q\x80\x91\x03\x90\xA2`\xC9Ta\x1C\x1C\x90c\xFF\xFF\xFF\xFF\x16`\x01aV7V[`\xC9\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UPPPPPV[a\x1CEa:\xF9V[a\x1CO`\0a;SV[V[`\xCFTd\x01\0\0\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1C\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04\x17WF\x83`\xDC\x1B`D\x82\x01R`d\x01a\x06nV[`\0a\x1C\xAD`@\x85\x01` \x86\x01aE\xE0V[\x90P6`\0a\x1C\xBF``\x87\x01\x87aV_V[\x90\x92P\x90P`\0a\x1C\xD6`\xA0\x88\x01`\x80\x89\x01aE\xE0V[\x90P`\xCA`\0a\x1C\xE9` \x89\x01\x89aE\xE0V[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x87`@Q` \x01a\x1D\x15\x91\x90aV\xEAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x14a\x1D\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7Fsupplied task does not match the`D\x82\x01R\x7F one recorded in the contract\0\0\0`d\x82\x01R`\x84\x01a\x06nV[`\x01`\xCC`\0a\x1D\xB1` \x8A\x01\x8AaE\xE0V[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16`\x03\x81\x11\x15a\x1D\xDCWa\x1D\xDCaKCV[\x14a\x1D\xF9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06n\x90aW\xB1V[`\0`\xCB\x81a\x1E\x0B` \x8A\x01\x8AaE\xE0V[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x14a\x1ECW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06n\x90aW\xB1V[a\x1Em\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85aV7V[c\xFF\xFF\xFF\xFF\x16Cc\xFF\xFF\xFF\xFF\x16\x11\x15a\x1E\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FAggregator has responded to the `D\x82\x01Rltask too late`\x98\x1B`d\x82\x01R`\x84\x01a\x06nV[`\0\x86`@Q` \x01a\x1E\xF1\x91\x90aXHV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x80a\x1F\x19\x83\x87\x87\x8A\x8Ca#\x13V[`@\x80Q`\x80\x81\x01\x82Rc\xFF\xFF\xFF\xFFC\x16\x81R` \x80\x82\x01\x84\x90R\x80\x85\x01Q\x82\x84\x01R\x84Q``\x83\x01R\x91Q\x93\x95P\x91\x93P\x90\x91a\x1F[\x91\x8C\x91\x84\x91\x01aXVV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\xCB`\0\x8C`\0\x01` \x81\x01\x90a\x1F\x88\x91\x90aE\xE0V[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP\x7F\xF9\x86b;\x08\x03\x1C)\xEF\xCB\xF1\t\x81\x18A-\x9Cq\xEF}2\xF6<p\x19\x88\xD6\xBB\xA2Q\xCEf\x8A\x82`@Qa\x1F\xD7\x92\x91\x90aXVV[`@Q\x80\x91\x03\x90\xA1`\0[\x86\x81\x10\x15a xW\x85`\xFF\x16\x84` \x01Q\x82\x81Q\x81\x10a \x04Wa \x04aQ\xC7V[` \x02` \x01\x01Qa \x16\x91\x90aX\xC4V[`\x01`\x01``\x1B\x03\x16`d\x85`\0\x01Q\x83\x81Q\x81\x10a 7Wa 7aQ\xC7V[` \x02` \x01\x01Q`\x01`\x01``\x1B\x03\x16a R\x91\x90aX\xF3V[\x10\x15a fWPPPPPPPPPPPPV[\x80a p\x81aR\xCEV[\x91PPa\x1F\xE2V[P`\xA0\x8A\x015`\xD1U`\x03`\xCC`\0a \x94` \x8E\x01\x8EaE\xE0V[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16`\x03\x81\x11\x15a \xBFWa \xBFaKCV[Pa \xD2\x90P`@\x8C\x01` \x8D\x01aE\xE0V[`\xCD\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua \xF9``\x8C\x01\x8CaV_V[a!\x05\x91`\xCE\x91aBOV[Pa!\x16`\xA0\x8C\x01`\x80\x8D\x01aE\xE0V[`\xCF\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U``\x8A\x015a!B` \x8C\x01\x8CaE\xE0V[c\xFF\xFF\xFF\xFF\x16\x7F\x83x\xBE\x8A3\xCF:I9\x10\xA1n'\\\xD9j\xF4\xF0H\xC5\xEB\x1A,)b\xD4\x06ni\x7F\xEA\x80`@Q`@Q\x80\x91\x03\x90\xA3PPPPPPPPPPPV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91``\x91\x83\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x84\x81`\0\x81Q\x81\x10a!\xBCWa!\xBCaQ\xC7V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Qca\xC8\xA1/`\xE1\x1B\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90c\xC3\x91B^\x90a!\xF8\x90\x88\x90\x86\x90`\x04\x01aT\xA3V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"\x15W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\"=\x91\x90\x81\x01\x90aS3V[`\0\x81Q\x81\x10a\"OWa\"OaQ\xC7V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x88\x90Rc\xFF\xFF\xFF\xFF\x87\x81\x16`$\x83\x01R\x90\x91\x16`D\x82\x01\x81\x90R\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x89\x16\x90c\x04\xECcQ\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"\xBBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"\xDF\x91\x90aT\x13V[`\x01`\x01`\xC0\x1B\x03\x16\x90P`\0a\"\xF5\x82a;\xA5V[\x90P\x81a#\x03\x8A\x83\x8Aa\tIV[\x95P\x95PPPPP\x93P\x93\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`\0\x84a#\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R`\0\x80Q` aZM\x839\x81Q\x91R`D\x82\x01R\x7Fres: empty quorum input\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06nV[`@\x83\x01QQ\x85\x14\x80\x15a#\xA2WP`\xA0\x83\x01QQ\x85\x14[\x80\x15a#\xB2WP`\xC0\x83\x01QQ\x85\x14[\x80\x15a#\xC2WP`\xE0\x83\x01QQ\x85\x14[a$,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R`\0\x80Q` aZM\x839\x81Q\x91R`D\x82\x01R\x7Fres: input quorum length mismatc`d\x82\x01R`\r`\xFB\x1B`\x84\x82\x01R`\xA4\x01a\x06nV[\x82QQ` \x84\x01QQ\x14a$\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R`\0\x80Q` aZM\x839\x81Q\x91R\x90\x82\x01R\x7Fres: input nonsigner length mism`d\x82\x01Rc\x0C.\x8Cm`\xE3\x1B`\x84\x82\x01R`\xA4\x01a\x06nV[Cc\xFF\xFF\xFF\xFF\x16\x84c\xFF\xFF\xFF\xFF\x16\x10a%\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R`\0\x80Q` aZM\x839\x81Q\x91R`D\x82\x01R\x7Fres: invalid reference block\0\0\0\0`d\x82\x01R`\x84\x01a\x06nV[`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x80\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x90\x93R``\x80\x84R\x90\x83\x01R\x90\x86`\x01`\x01`@\x1B\x03\x81\x11\x15a%TWa%TaD\rV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a%}W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P` \x82\x01R\x86`\x01`\x01`@\x1B\x03\x81\x11\x15a%\x9BWa%\x9BaD\rV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a%\xC4W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x81R`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R\x85` \x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a%\xF8Wa%\xF8aD\rV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a&!W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x81R` \x86\x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a&AWa&AaD\rV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a&jW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x81` \x01\x81\x90RP`\0a'<\x8A\x8A\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`@\x80Qc\x9A\xA1e=`\xE0\x1B\x81R\x90Q`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x93Pc\x9A\xA1e=\x92P`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a'\x13W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'7\x91\x90aY\x12V[a<qV[\x90P`\0[\x87` \x01QQ\x81\x10\x15a)\xD7Wa'\x86\x88` \x01Q\x82\x81Q\x81\x10a'gWa'gaQ\xC7V[` \x02` \x01\x01Q\x80Q`\0\x90\x81R` \x91\x82\x01Q\x90\x91R`@\x90 \x90V[\x83` \x01Q\x82\x81Q\x81\x10a'\x9CWa'\x9CaQ\xC7V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80\x15a(\\W` \x83\x01Qa'\xBD`\x01\x83aY/V[\x81Q\x81\x10a'\xCDWa'\xCDaQ\xC7V[` \x02` \x01\x01Q`\0\x1C\x83` \x01Q\x82\x81Q\x81\x10a'\xEEWa'\xEEaQ\xC7V[` \x02` \x01\x01Q`\0\x1C\x11a(\\W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R`\0\x80Q` aZM\x839\x81Q\x91R`D\x82\x01R\x7Fres: nonSignerPubkeys not sorted`d\x82\x01R`\x84\x01a\x06nV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x04\xECcQ\x84` \x01Q\x83\x81Q\x81\x10a(\xA1Wa(\xA1aQ\xC7V[` \x02` \x01\x01Q\x8B\x8B`\0\x01Q\x85\x81Q\x81\x10a(\xC0Wa(\xC0aQ\xC7V[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a(\xFD\x93\x92\x91\x90\x92\x83Rc\xFF\xFF\xFF\xFF\x91\x82\x16` \x84\x01R\x16`@\x82\x01R``\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a)\x1AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)>\x91\x90aT\x13V[`\x01`\x01`\xC0\x1B\x03\x16\x83`\0\x01Q\x82\x81Q\x81\x10a)]Wa)]aQ\xC7V[` \x02` \x01\x01\x81\x81RPPa)\xC3a\x08\xDAa)\x97\x84\x86`\0\x01Q\x85\x81Q\x81\x10a)\x89Wa)\x89aQ\xC7V[` \x02` \x01\x01Q\x16a=\x04V[\x8A` \x01Q\x84\x81Q\x81\x10a)\xADWa)\xADaQ\xC7V[` \x02` \x01\x01Qa=/\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x94P\x80a)\xCF\x81aR\xCEV[\x91PPa'AV[PPa)\xE2\x83a>\x13V[`\x97T\x90\x93P`\xFF\x16`\0\x81a)\xF9W`\0a*{V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xC4H\xFE\xB8`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a*WW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*{\x91\x90aYFV[\x90P`\0[\x8A\x81\x10\x15a0\xFAW\x82\x15a+\xDCW\x89c\xFF\xFF\xFF\xFF\x16\x82\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c$\x9A\x0CB\x8F\x8F\x86\x81\x81\x10a*\xD7Wa*\xD7aQ\xC7V[`@Q`\xE0\x85\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R\x92\x015`\xF8\x1C`\x04\x83\x01RP`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+\x17W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+;\x91\x90aYFV[a+E\x91\x90aY_V[\x10\x15a+\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`f`$\x82\x01R`\0\x80Q` aZM\x839\x81Q\x91R`D\x82\x01R\x7Fres: StakeRegistry updates must `d\x82\x01R\x7Fbe within withdrawalDelayBlocks `\x84\x82\x01Rewindow`\xD0\x1B`\xA4\x82\x01R`\xC4\x01a\x06nV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16ch\xBC\xCA\xAC\x8D\x8D\x84\x81\x81\x10a,\x1DWa,\x1DaQ\xC7V[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x8C\x8C`\xA0\x01Q\x85\x81Q\x81\x10a,AWa,AaQ\xC7V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81R`\xFF\x90\x93\x16`\x04\x84\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x84\x01R\x16`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,\x9DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\xC1\x91\x90aYwV[`\x01`\x01`@\x1B\x03\x19\x16a,\xE4\x8A`@\x01Q\x83\x81Q\x81\x10a'gWa'gaQ\xC7V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14a-\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`a`$\x82\x01R`\0\x80Q` aZM\x839\x81Q\x91R`D\x82\x01R\x7Fres: quorumApk hash in storage d`d\x82\x01R\x7Foes not match provided quorum ap`\x84\x82\x01R`k`\xF8\x1B`\xA4\x82\x01R`\xC4\x01a\x06nV[a-\xB0\x89`@\x01Q\x82\x81Q\x81\x10a-\x99Wa-\x99aQ\xC7V[` \x02` \x01\x01Q\x87a6\xF1\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x95P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xC8)LV\x8D\x8D\x84\x81\x81\x10a-\xF3Wa-\xF3aQ\xC7V[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x8C\x8C`\xC0\x01Q\x85\x81Q\x81\x10a.\x17Wa.\x17aQ\xC7V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81R`\xFF\x90\x93\x16`\x04\x84\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x84\x01R\x16`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a.sW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.\x97\x91\x90aR\x8FV[\x85` \x01Q\x82\x81Q\x81\x10a.\xADWa.\xADaQ\xC7V[`\x01`\x01``\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x82\x01R\x85\x01Q\x80Q\x82\x90\x81\x10a.\xD9Wa.\xD9aQ\xC7V[` \x02` \x01\x01Q\x85`\0\x01Q\x82\x81Q\x81\x10a.\xF7Wa.\xF7aQ\xC7V[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPP`\0\x80[\x8A` \x01QQ\x81\x10\x15a0\xE5Wa/o\x86`\0\x01Q\x82\x81Q\x81\x10a/AWa/AaQ\xC7V[` \x02` \x01\x01Q\x8F\x8F\x86\x81\x81\x10a/[Wa/[aQ\xC7V[`\x01\x92\x015`\xF8\x1C\x92\x90\x92\x1C\x81\x16\x14\x91\x90PV[\x15a0\xD3W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xF2\xBE\x94\xAE\x8F\x8F\x86\x81\x81\x10a/\xB5Wa/\xB5aQ\xC7V[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x8E\x89` \x01Q\x85\x81Q\x81\x10a/\xD9Wa/\xD9aQ\xC7V[` \x02` \x01\x01Q\x8F`\xE0\x01Q\x88\x81Q\x81\x10a/\xF7Wa/\xF7aQ\xC7V[` \x02` \x01\x01Q\x87\x81Q\x81\x10a0\x10Wa0\x10aQ\xC7V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x87\x90\x1B\x16\x81R`\xFF\x90\x94\x16`\x04\x85\x01Rc\xFF\xFF\xFF\xFF\x92\x83\x16`$\x85\x01R`D\x84\x01\x91\x90\x91R\x16`d\x82\x01R`\x84\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a0tW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a0\x98\x91\x90aR\x8FV[\x87Q\x80Q\x85\x90\x81\x10a0\xACWa0\xACaQ\xC7V[` \x02` \x01\x01\x81\x81Qa0\xC0\x91\x90aY\xA2V[`\x01`\x01``\x1B\x03\x16\x90RP`\x01\x90\x91\x01\x90[\x80a0\xDD\x81aR\xCEV[\x91PPa/\x1BV[PP\x80\x80a0\xF2\x90aR\xCEV[\x91PPa*\x80V[PPP`\0\x80a1\x14\x8C\x86\x8A``\x01Q\x8B`\x80\x01Qa\x07\xBFV[\x91P\x91P\x81a1\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R`\0\x80Q` aZM\x839\x81Q\x91R`D\x82\x01R\x7Fres: pairing precompile call fai`d\x82\x01Rb\x1B\x19Y`\xEA\x1B`\x84\x82\x01R`\xA4\x01a\x06nV[\x80a1\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R`\0\x80Q` aZM\x839\x81Q\x91R`D\x82\x01R\x7Fres: signature is invalid\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06nV[PP`\0\x87\x82` \x01Q`@Q` \x01a2\x01\x92\x91\x90aY\xC2V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x92\x9B\x92\x9AP\x91\x98PPPPPPPPPV[a23a:\xF9V[`\x01`\x01`\xA0\x1B\x03\x81\x16a2\x98W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x06nV[a\x04\xE9\x81a;SV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a2\xC1WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a2\xDBWP0;\x15\x80\x15a2\xDBWP`\0T`\xFF\x16`\x01\x14[a3>W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x06nV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a3aW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a3l\x85`\0a>\xAEV[a3u\x84a;SV[`\xCF\x80Td\x01\0\0\0\0`\x01`\xC0\x1B\x03\x19\x16d\x01\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x90\x91\x02\x91\x90\x91\x17\x90\x91U`\xD0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x84\x16\x91\x90\x91\x17\x90U\x80\x15a4\0W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a4ZW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a4~\x91\x90aP\xFBV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a4\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06n\x90aQ\x18V[`fT\x19\x81\x19`fT\x19\x16\x14a5,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06nV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x07\xB4V[`\x01`\x01`\xA0\x1B\x03\x81\x16a5\xF1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x06nV[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra6vaB\xD3V[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x80\x82\x01\x84\x90R`\0\x90\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a6\xA9Wa6\xABV[\xFE[P\x80a6\xE9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCB[][\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x06nV[PP\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra7\raB\xF1V[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R\x83Q`@\x80\x84\x01\x91\x90\x91R\x90\x84\x01Q``\x83\x01R`\0\x90\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a6\xA9WP\x80a6\xE9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCBXY\x19\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x06nV[a7\x8DaC\x0FV[P`@\x80Q`\x80\x81\x01\x82R\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81\x83\x01\x90\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED``\x83\x01R\x81R\x81Q\x80\x83\x01\x90\x92R\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x81\x01\x91\x90\x91R\x81\x01\x91\x90\x91R\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0\x80\x80a8u`\0\x80Q` aZ-\x839\x81Q\x91R\x86aQ\xDDV[\x90P[a8\x81\x81a?\x98V[\x90\x93P\x91P`\0\x80Q` aZ-\x839\x81Q\x91R\x82\x83\t\x83\x14\x15a8\xBBW`@\x80Q\x80\x82\x01\x90\x91R\x90\x81R` \x81\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80Q` aZ-\x839\x81Q\x91R`\x01\x82\x08\x90Pa8xV[`@\x80Q\x80\x82\x01\x82R\x86\x81R` \x80\x82\x01\x86\x90R\x82Q\x80\x84\x01\x90\x93R\x86\x83R\x82\x01\x84\x90R`\0\x91\x82\x91\x90a9\x07aC4V[`\0[`\x02\x81\x10\x15a:\xCCW`\0a9 \x82`\x06aX\xF3V[\x90P\x84\x82`\x02\x81\x10a94Wa94aQ\xC7V[` \x02\x01QQ\x83a9F\x83`\0aY_V[`\x0C\x81\x10a9VWa9VaQ\xC7V[` \x02\x01R\x84\x82`\x02\x81\x10a9mWa9maQ\xC7V[` \x02\x01Q` \x01Q\x83\x82`\x01a9\x84\x91\x90aY_V[`\x0C\x81\x10a9\x94Wa9\x94aQ\xC7V[` \x02\x01R\x83\x82`\x02\x81\x10a9\xABWa9\xABaQ\xC7V[` \x02\x01QQQ\x83a9\xBE\x83`\x02aY_V[`\x0C\x81\x10a9\xCEWa9\xCEaQ\xC7V[` \x02\x01R\x83\x82`\x02\x81\x10a9\xE5Wa9\xE5aQ\xC7V[` \x02\x01QQ`\x01` \x02\x01Q\x83a9\xFE\x83`\x03aY_V[`\x0C\x81\x10a:\x0EWa:\x0EaQ\xC7V[` \x02\x01R\x83\x82`\x02\x81\x10a:%Wa:%aQ\xC7V[` \x02\x01Q` \x01Q`\0`\x02\x81\x10a:@Wa:@aQ\xC7V[` \x02\x01Q\x83a:Q\x83`\x04aY_V[`\x0C\x81\x10a:aWa:aaQ\xC7V[` \x02\x01R\x83\x82`\x02\x81\x10a:xWa:xaQ\xC7V[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a:\x93Wa:\x93aQ\xC7V[` \x02\x01Q\x83a:\xA4\x83`\x05aY_V[`\x0C\x81\x10a:\xB4Wa:\xB4aQ\xC7V[` \x02\x01RP\x80a:\xC4\x81aR\xCEV[\x91PPa9\nV[Pa:\xD5aCSV[`\0` \x82a\x01\x80\x85`\x08\x8C\xFA\x91Q\x91\x9C\x91\x15\x15\x9BP\x90\x99PPPPPPPPPPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1COW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x06nV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[```\0\x80a;\xB3\x84a=\x04V[a\xFF\xFF\x16`\x01`\x01`@\x1B\x03\x81\x11\x15a;\xCEWa;\xCEaD\rV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a;\xF8W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x80[\x82Q\x82\x10\x80\x15a<\x10WPa\x01\0\x81\x10[\x15a<gW`\x01\x81\x1B\x93P\x85\x84\x16\x15a<WW\x80`\xF8\x1B\x83\x83\x81Q\x81\x10a<9Wa<9aQ\xC7V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x81`\x01\x01\x91P[a<`\x81aR\xCEV[\x90Pa;\xFFV[P\x90\x94\x93PPPPV[`\0\x80a<}\x84a@\x1AV[\x90P\x80\x83`\xFF\x16`\x01\x90\x1B\x11a<\xFBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`?`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: bitmap exceeds max value\0`d\x82\x01R`\x84\x01a\x06nV[\x90P[\x92\x91PPV[`\0\x80[\x82\x15a<\xFEWa=\x19`\x01\x84aY/V[\x90\x92\x16\x91\x80a='\x81aZ\nV[\x91PPa=\x08V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x02\0\x82a\xFF\xFF\x16\x10a=\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Roscalar-too-large`\x80\x1B`D\x82\x01R`d\x01a\x06nV[\x81a\xFF\xFF\x16`\x01\x14\x15a=\x9FWP\x81a<\xFEV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01\x81\x90R\x84\x90`\x01\x90[\x81a\xFF\xFF\x16\x86a\xFF\xFF\x16\x10a>\x08W`\x01a\xFF\xFF\x87\x16`\xFF\x83\x16\x1C\x81\x16\x14\x15a=\xEBWa=\xE8\x84\x84a6\xF1V[\x93P[a=\xF5\x83\x84a6\xF1V[\x92Pb\x01\xFF\xFE`\x01\x92\x83\x1B\x16\x91\x01a=\xBBV[P\x91\x95\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q\x15\x80\x15a>8WP` \x82\x01Q\x15[\x15a>VWPP`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01`\0\x80Q` aZ-\x839\x81Q\x91R\x84` \x01Qa>\x89\x91\x90aQ\xDDV[a>\xA1\x90`\0\x80Q` aZ-\x839\x81Q\x91RaY/V[\x90R\x92\x91PPV[\x91\x90PV[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a>\xCFWP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a?QW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x06nV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a?\x94\x82a5cV[PPV[`\0\x80\x80`\0\x80Q` aZ-\x839\x81Q\x91R`\x03`\0\x80Q` aZ-\x839\x81Q\x91R\x86`\0\x80Q` aZ-\x839\x81Q\x91R\x88\x89\t\t\x08\x90P`\0a@\x0E\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\0\x80Q` aZ-\x839\x81Q\x91RaA\xA7V[\x91\x95\x91\x94P\x90\x92PPPV[`\0a\x01\0\x82Q\x11\x15a@\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FBitmapUtils.orderedBytesArrayToB\x90\x82\x01R\x7Fitmap: orderedBytesArray is too `d\x82\x01Rclong`\xE0\x1B`\x84\x82\x01R`\xA4\x01a\x06nV[\x81Qa@\xB1WP`\0\x91\x90PV[`\0\x80\x83`\0\x81Q\x81\x10a@\xC7Wa@\xC7aQ\xC7V[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x81\x90\x1B\x92P[\x84Q\x81\x10\x15aA\x9EW\x84\x81\x81Q\x81\x10a@\xF5Wa@\xF5aQ\xC7V[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x1B\x91P\x82\x82\x11aA\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: orderedBytesArray is not `d\x82\x01Rf\x1B\xDC\x99\x19\\\x99Y`\xCA\x1B`\x84\x82\x01R`\xA4\x01a\x06nV[\x91\x81\x17\x91aA\x97\x81aR\xCEV[\x90Pa@\xDAV[P\x90\x93\x92PPPV[`\0\x80aA\xB2aCSV[aA\xBAaCqV[` \x80\x82R\x81\x81\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x88\x90R`\x80\x82\x01\x87\x90R`\xA0\x82\x01\x86\x90R\x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82\x80\x15a6\xA9WP\x82aBDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FBN254.expMod: call failure\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06nV[PQ\x95\x94PPPPPV[\x82\x80TaB[\x90aT\xF7V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82aB}W`\0\x85UaB\xC3V[\x82`\x1F\x10aB\x96W\x82\x80\x01`\xFF\x19\x825\x16\x17\x85UaB\xC3V[\x82\x80\x01`\x01\x01\x85U\x82\x15aB\xC3W\x91\x82\x01[\x82\x81\x11\x15aB\xC3W\x825\x82U\x91` \x01\x91\x90`\x01\x01\x90aB\xA8V[PaB\xCF\x92\x91PaC\x8FV[P\x90V[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80aC\"aC\xA4V[\x81R` \x01aC/aC\xA4V[\x90R\x90V[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[[\x80\x82\x11\x15aB\xCFW`\0\x81U`\x01\x01aC\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04\xE9W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aC\xE9W`\0\x80\xFD[\x815a<\xFB\x81aC\xC2V[`\0` \x82\x84\x03\x12\x15aD\x06W`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aDEWaDEaD\rV[`@R\x90V[`@Qa\x01\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aDEWaDEaD\rV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aD\x96WaD\x96aD\rV[`@R\x91\x90PV[`\0`@\x82\x84\x03\x12\x15aD\xB0W`\0\x80\xFD[aD\xB8aD#V[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12aD\xDFW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15aE\x01WaE\x01aD\rV[\x80`@RP\x80`@\x84\x01\x85\x81\x11\x15aE\x18W`\0\x80\xFD[\x84[\x81\x81\x10\x15a>\x08W\x805\x83R` \x92\x83\x01\x92\x01aE\x1AV[`\0`\x80\x82\x84\x03\x12\x15aEDW`\0\x80\xFD[aELaD#V[\x90PaEX\x83\x83aD\xCEV[\x81RaEg\x83`@\x84\x01aD\xCEV[` \x82\x01R\x92\x91PPV[`\0\x80`\0\x80a\x01 \x85\x87\x03\x12\x15aE\x89W`\0\x80\xFD[\x845\x93PaE\x9A\x86` \x87\x01aD\x9EV[\x92PaE\xA9\x86``\x87\x01aE2V[\x91PaE\xB8\x86`\xE0\x87\x01aD\x9EV[\x90P\x92\x95\x91\x94P\x92PV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x04\xE9W`\0\x80\xFD[\x805a>\xA9\x81aE\xC3V[`\0` \x82\x84\x03\x12\x15aE\xF2W`\0\x80\xFD[\x815a<\xFB\x81aE\xC3V[`\0\x80`\0``\x84\x86\x03\x12\x15aF\x12W`\0\x80\xFD[\x835aF\x1D\x81aC\xC2V[\x92P` \x84\x81\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aF:W`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12aFNW`\0\x80\xFD[\x815\x81\x81\x11\x15aF`WaF`aD\rV[aFr`\x1F\x82\x01`\x1F\x19\x16\x85\x01aDnV[\x91P\x80\x82R\x88\x84\x82\x85\x01\x01\x11\x15aF\x88W`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x94PPPPaF\xAB`@\x85\x01aE\xD5V[\x90P\x92P\x92P\x92V[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0\x80[\x86\x81\x10\x15aGJW\x83\x85\x03\x8AR\x82Q\x80Q\x80\x87R\x90\x87\x01\x90\x87\x87\x01\x90\x84[\x81\x81\x10\x15aG5W\x83Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x8A\x81\x01Q\x8B\x85\x01R`@\x90\x81\x01Q`\x01`\x01``\x1B\x03\x16\x90\x84\x01R\x92\x89\x01\x92``\x90\x92\x01\x91`\x01\x01aF\xF1V[PP\x9A\x87\x01\x9A\x95PP\x91\x85\x01\x91`\x01\x01aF\xD3V[P\x92\x98\x97PPPPPPPPV[` \x81R`\0aGk` \x83\x01\x84aF\xB4V[\x93\x92PPPV[\x80\x15\x15\x81\x14a\x04\xE9W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aG\x92W`\0\x80\xFD[\x815a<\xFB\x81aGrV[`\0\x80\x83`\x1F\x84\x01\x12aG\xAFW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aG\xC6W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aG\xDEW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\x80\x87\x89\x03\x12\x15aG\xFEW`\0\x80\xFD[\x865aH\t\x81aC\xC2V[\x95P` \x87\x015aH\x19\x81aE\xC3V[\x94P`@\x87\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aH5W`\0\x80\xFD[aHA\x8A\x83\x8B\x01aG\x9DV[\x90\x96P\x94P``\x89\x015\x91P\x80\x82\x11\x15aHZW`\0\x80\xFD[\x81\x89\x01\x91P\x89`\x1F\x83\x01\x12aHnW`\0\x80\xFD[\x815\x81\x81\x11\x15aH}W`\0\x80\xFD[\x8A` \x82`\x05\x1B\x85\x01\x01\x11\x15aH\x92W`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92\x95P\x92\x95P\x92\x95V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aH\xDEW\x81Qc\xFF\xFF\xFF\xFF\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aH\xBCV[P\x94\x95\x94PPPPPV[`\0` \x80\x83R\x83Q`\x80\x82\x85\x01RaI\x05`\xA0\x85\x01\x82aH\xA8V[\x90P\x81\x85\x01Q`\x1F\x19\x80\x86\x84\x03\x01`@\x87\x01RaI\"\x83\x83aH\xA8V[\x92P`@\x87\x01Q\x91P\x80\x86\x84\x03\x01``\x87\x01RaI?\x83\x83aH\xA8V[``\x88\x01Q\x87\x82\x03\x83\x01`\x80\x89\x01R\x80Q\x80\x83R\x91\x94P\x85\x01\x92P\x84\x84\x01\x90`\x05\x81\x90\x1B\x85\x01\x86\x01`\0[\x82\x81\x10\x15aI\x96W\x84\x87\x83\x03\x01\x84RaI\x84\x82\x87QaH\xA8V[\x95\x88\x01\x95\x93\x88\x01\x93\x91P`\x01\x01aIjV[P\x99\x98PPPPPPPPPV[`\xFF\x81\x16\x81\x14a\x04\xE9W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aI\xC5W`\0\x80\xFD[\x815a<\xFB\x81aI\xA4V[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15aI\xE9WaI\xE9aD\rV[P`\x05\x1B` \x01\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15aJ\x08W`\0\x80\xFD[\x835aJ\x13\x81aC\xC2V[\x92P` \x84\x81\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aJ/W`\0\x80\xFD[\x85\x01`\x1F\x81\x01\x87\x13aJ@W`\0\x80\xFD[\x805aJSaJN\x82aI\xD0V[aDnV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x89\x83\x11\x15aJrW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aJ\x90W\x835\x82R\x92\x84\x01\x92\x90\x84\x01\x90aJwV[\x80\x96PPPPPPaF\xAB`@\x85\x01aE\xD5V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aJ\xDCW\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aJ\xC0V[P\x90\x96\x95PPPPPPV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15aJ\xFEW`\0\x80\xFD[\x845\x93P` \x85\x015aK\x10\x81aE\xC3V[\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aK+W`\0\x80\xFD[aK7\x87\x82\x88\x01aG\x9DV[\x95\x98\x94\x97P\x95PPPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x04\x83\x10aK{WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`\0` \x82\x84\x03\x12\x15aK\x93W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aK\xA9W`\0\x80\xFD[\x82\x01a\x01@\x81\x85\x03\x12\x15a<\xFBW`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12aK\xCDW`\0\x80\xFD[\x815` aK\xDDaJN\x83aI\xD0V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aK\xFCW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aL W\x805aL\x13\x81aE\xC3V[\x83R\x91\x83\x01\x91\x83\x01aL\0V[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12aL<W`\0\x80\xFD[\x815` aLLaJN\x83aI\xD0V[\x82\x81R`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aLkW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aL WaL\x81\x88\x82aD\x9EV[\x83R\x91\x83\x01\x91`@\x01aLoV[`\0\x82`\x1F\x83\x01\x12aL\xA0W`\0\x80\xFD[\x815` aL\xB0aJN\x83aI\xD0V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aL\xCFW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aL W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aL\xF2W`\0\x80\x81\xFD[aM\0\x89\x86\x83\x8B\x01\x01aK\xBCV[\x84RP\x91\x83\x01\x91\x83\x01aL\xD3V[`\0a\x02\x80\x82\x84\x03\x12\x15aM!W`\0\x80\xFD[aM)aDKV[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aMBW`\0\x80\xFD[aMN\x85\x83\x86\x01aK\xBCV[\x83R` \x84\x015\x91P\x80\x82\x11\x15aMdW`\0\x80\xFD[aMp\x85\x83\x86\x01aL+V[` \x84\x01R`@\x84\x015\x91P\x80\x82\x11\x15aM\x89W`\0\x80\xFD[aM\x95\x85\x83\x86\x01aL+V[`@\x84\x01RaM\xA7\x85``\x86\x01aE2V[``\x84\x01RaM\xB9\x85`\xE0\x86\x01aD\x9EV[`\x80\x84\x01Ra\x01 \x91P\x81\x84\x015\x81\x81\x11\x15aM\xD4W`\0\x80\xFD[aM\xE0\x86\x82\x87\x01aK\xBCV[`\xA0\x85\x01RPa\x01@\x80\x85\x015\x82\x81\x11\x15aM\xFAW`\0\x80\xFD[aN\x06\x87\x82\x88\x01aK\xBCV[`\xC0\x86\x01RPa\x01`\x80\x86\x015\x83\x81\x11\x15aN W`\0\x80\xFD[aN,\x88\x82\x89\x01aL\x8FV[`\xE0\x87\x01RPa\x01\x80\x86\x015\x83\x81\x11\x15aNEW`\0\x80\xFD[aNQ\x88\x82\x89\x01aK\xBCV[a\x01\0\x87\x01RPa\x01\xA0\x86\x015\x83\x81\x11\x15aNkW`\0\x80\xFD[aNw\x88\x82\x89\x01aL+V[\x85\x87\x01RPaN\x8A\x87a\x01\xC0\x88\x01aE2V[\x82\x86\x01RaN\x9C\x87a\x02@\x88\x01aD\x9EV[\x81\x86\x01RPPPP\x92\x91PPV[`\0\x80`\0\x83\x85\x03a\x01\0\x81\x12\x15aN\xC1W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aN\xD8W`\0\x80\xFD[\x90\x86\x01\x90`\xE0\x82\x89\x03\x12\x15aN\xECW`\0\x80\xFD[\x81\x95P`\xC0`\x1F\x19\x84\x01\x12\x15aO\x01W`\0\x80\xFD[` \x87\x01\x94P`\xE0\x87\x015\x92P\x80\x83\x11\x15aO\x1BW`\0\x80\xFD[PPaO)\x86\x82\x87\x01aM\x0EV[\x91PP\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15aOHW`\0\x80\xFD[\x835aOS\x81aC\xC2V[\x92P` \x84\x015\x91P`@\x84\x015aOj\x81aE\xC3V[\x80\x91PP\x92P\x92P\x92V[\x82\x81R`@` \x82\x01R`\0aO\x8E`@\x83\x01\x84aF\xB4V[\x94\x93PPPPV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15aO\xAEW`\0\x80\xFD[\x855\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aO\xCCW`\0\x80\xFD[aO\xD8\x89\x83\x8A\x01aG\x9DV[\x90\x96P\x94P`@\x88\x015\x91PaO\xED\x82aE\xC3V[\x90\x92P``\x87\x015\x90\x80\x82\x11\x15aP\x03W`\0\x80\xFD[PaP\x10\x88\x82\x89\x01aM\x0EV[\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aH\xDEW\x81Q`\x01`\x01``\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aP1V[`@\x81R`\0\x83Q`@\x80\x84\x01RaPq`\x80\x84\x01\x82aP\x1DV[\x90P` \x85\x01Q`?\x19\x84\x83\x03\x01``\x85\x01RaP\x8E\x82\x82aP\x1DV[\x92PPP\x82` \x83\x01R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aP\xB5W`\0\x80\xFD[\x845aP\xC0\x81aC\xC2V[\x93P` \x85\x015aP\xD0\x81aC\xC2V[\x92P`@\x85\x015aP\xE0\x81aC\xC2V[\x91P``\x85\x015aP\xF0\x81aC\xC2V[\x93\x96\x92\x95P\x90\x93PPV[`\0` \x82\x84\x03\x12\x15aQ\rW`\0\x80\xFD[\x81Qa<\xFB\x81aC\xC2V[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15aQtW`\0\x80\xFD[\x81Qa<\xFB\x81aGrV[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x82aQ\xFAWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\0` \x80\x83\x85\x03\x12\x15aR\x12W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aR(W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aR9W`\0\x80\xFD[\x80QaRGaJN\x82aI\xD0V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15aRfW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aR\x84W\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90aRkV[\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15aR\xA1W`\0\x80\xFD[\x81Q`\x01`\x01``\x1B\x03\x81\x16\x81\x14a<\xFBW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15aR\xE2WaR\xE2aR\xB8V[P`\x01\x01\x90V[c\xFF\xFF\xFF\xFF\x84\x16\x81R`@` \x82\x01\x81\x90R\x81\x01\x82\x90R`\0`\x01`\x01`\xFB\x1B\x03\x83\x11\x15aS\x16W`\0\x80\xFD[\x82`\x05\x1B\x80\x85``\x85\x017`\0\x92\x01``\x01\x91\x82RP\x93\x92PPPV[`\0` \x80\x83\x85\x03\x12\x15aSFW`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aS\\W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aSmW`\0\x80\xFD[\x80QaS{aJN\x82aI\xD0V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15aS\x9AW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aR\x84W\x83QaS\xB2\x81aE\xC3V[\x82R\x92\x84\x01\x92\x90\x84\x01\x90aS\x9FV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[c\xFF\xFF\xFF\xFF\x84\x16\x81R`@` \x82\x01R`\0aT\n`@\x83\x01\x84\x86aS\xC1V[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15aT%W`\0\x80\xFD[\x81Q`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x14a<\xFBW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aTNW`\0\x80\xFD[\x81Qa<\xFB\x81aE\xC3V[`\0`\xFF\x82\x16`\xFF\x81\x14\x15aTpWaTpaR\xB8V[`\x01\x01\x92\x91PPV[`@\x81R`\0aT\x8D`@\x83\x01\x85\x87aS\xC1V[\x90Pc\xFF\xFF\xFF\xFF\x83\x16` \x83\x01R\x94\x93PPPPV[`\0`@\x82\x01c\xFF\xFF\xFF\xFF\x85\x16\x83R` `@\x81\x85\x01R\x81\x85Q\x80\x84R``\x86\x01\x91P\x82\x87\x01\x93P`\0[\x81\x81\x10\x15aT\xEAW\x84Q\x83R\x93\x83\x01\x93\x91\x83\x01\x91`\x01\x01aT\xCEV[P\x90\x97\x96PPPPPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80aU\x0BW`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15aU,WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0c\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15aUOWaUOaR\xB8V[\x03\x93\x92PPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15aU}W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01aUaV[\x81\x81\x11\x15aU\x8FW`\0` \x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R\x81Q` \x82\x01R`\0` \x83\x01Qc\xFF\xFF\xFF\xFF\x80\x82\x16`@\x85\x01R\x80`@\x86\x01Q\x16``\x85\x01R``\x85\x01Q\x91P`\xE0`\x80\x85\x01RaU\xEBa\x01\0\x85\x01\x83aUWV[\x91P\x80`\x80\x86\x01Q\x16`\xA0\x85\x01RP`\xA0\x84\x01Q`\x1F\x19\x84\x83\x03\x01`\xC0\x85\x01RaV\x15\x82\x82aUWV[\x91PP`\xC0\x84\x01QaV/`\xE0\x85\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P\x93\x92PPPV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aVVWaVVaR\xB8V[\x01\x94\x93PPPPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aVvW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aV\x90W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15aG\xDEW`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aV\xBCW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aV\xDBW`\0\x80\xFD[\x806\x03\x83\x13\x15aG\xDEW`\0\x80\xFD[` \x81R\x815` \x82\x01R`\0` \x83\x015aW\x05\x81aE\xC3V[c\xFF\xFF\xFF\xFF\x81\x16`@\x84\x01RP`@\x83\x015aW \x81aE\xC3V[c\xFF\xFF\xFF\xFF\x81\x16``\x84\x01RPaW:``\x84\x01\x84aV\xA5V[`\xE0`\x80\x85\x01RaWPa\x01\0\x85\x01\x82\x84aS\xC1V[\x91PPaW_`\x80\x85\x01aE\xD5V[c\xFF\xFF\xFF\xFF\x81\x16`\xA0\x85\x01RPaWy`\xA0\x85\x01\x85aV\xA5V[\x84\x83\x03`\x1F\x19\x01`\xC0\x86\x01RaW\x90\x83\x82\x84aS\xC1V[\x92PPPaW\xA0`\xC0\x85\x01aE\xD5V[c\xFF\xFF\xFF\xFF\x81\x16`\xE0\x85\x01RaV/V[` \x80\x82R`,\x90\x82\x01R\x7FAggregator has already responded`@\x82\x01Rk to the task`\xA0\x1B``\x82\x01R`\x80\x01\x90V[\x805aX\x08\x81aE\xC3V[c\xFF\xFF\xFF\xFF\x81\x16\x83RP` \x81\x015` \x83\x01R`@\x81\x015`@\x83\x01R``\x81\x015``\x83\x01R`\x80\x81\x015`\x80\x83\x01R`\xA0\x81\x015`\xA0\x83\x01RPPV[`\xC0\x81\x01a<\xFE\x82\x84aW\xFDV[aX`\x81\x84aW\xFDV[`\xE0`\xC0\x82\x01Rc\xFF\xFF\xFF\xFF\x82Q\x16`\xE0\x82\x01R` \x82\x01Qa\x01\0\x82\x01R`\0`@\x83\x01Q`\x80a\x01 \x84\x01RaX\x9Ca\x01`\x84\x01\x82aP\x1DV[\x90P``\x84\x01Q`\xDF\x19\x84\x83\x03\x01a\x01@\x85\x01RaX\xBA\x82\x82aP\x1DV[\x96\x95PPPPPPV[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aX\xEAWaX\xEAaR\xB8V[\x02\x94\x93PPPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15aY\rWaY\raR\xB8V[P\x02\x90V[`\0` \x82\x84\x03\x12\x15aY$W`\0\x80\xFD[\x81Qa<\xFB\x81aI\xA4V[`\0\x82\x82\x10\x15aYAWaYAaR\xB8V[P\x03\x90V[`\0` \x82\x84\x03\x12\x15aYXW`\0\x80\xFD[PQ\x91\x90PV[`\0\x82\x19\x82\x11\x15aYrWaYraR\xB8V[P\x01\x90V[`\0` \x82\x84\x03\x12\x15aY\x89W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16\x81\x14a<\xFBW`\0\x80\xFD[`\0`\x01`\x01``\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15aUOWaUOaR\xB8V[c\xFF\xFF\xFF\xFF`\xE0\x1B\x83`\xE0\x1B\x16\x81R`\0`\x04\x82\x01\x83Q` \x80\x86\x01`\0[\x83\x81\x10\x15aY\xFDW\x81Q\x85R\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01aY\xE1V[P\x92\x97\x96PPPPPPPV[`\0a\xFF\xFF\x80\x83\x16\x81\x81\x14\x15aZ\"WaZ\"aR\xB8V[`\x01\x01\x93\x92PPPV\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDGBLSSignatureChecker.checkSignatu\xA2dipfsX\"\x12 \xB2\xBF\xFEk\xEDN\x9A\xC4\xB0`\xD0\x1DJ0\xF1\x98D\xF0\x93{v|\xC0\x95\xFF\xD7\xE6$\xEDP\xE6\x07dsolcC\0\x08\x0C\x003";
    /// The deployed bytecode of the contract.
    pub static FINALIZERTASKMANAGER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
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
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    FINALIZERTASKMANAGER_ABI.clone(),
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
                FINALIZERTASKMANAGER_ABI.clone(),
                FINALIZERTASKMANAGER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `aggregator` (0x245a7bfc) function
        pub fn aggregator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([36, 90, 123, 252], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allTaskHashes` (0x2d89f6fc) function
        pub fn all_task_hashes(
            &self,
            p0: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([45, 137, 246, 252], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allTaskResponses` (0x2cb223d5) function
        pub fn all_task_responses(
            &self,
            p0: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([44, 178, 35, 213], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `blsApkRegistry` (0x5df45946) function
        pub fn bls_apk_registry(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([93, 244, 89, 70], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkSignatures` (0xdb67f777) function
        pub fn check_signatures(
            &self,
            msg_hash: [u8; 32],
            quorum_numbers: ::ethers::core::types::Bytes,
            reference_block_number: u32,
            params: NonSignerStakesAndSignature,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (QuorumStakeTotals, [u8; 32]),
        > {
            self.0
                .method_hash(
                    [219, 103, 247, 119],
                    (msg_hash, quorum_numbers, reference_block_number, params),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createNewTask` (0x6b92787e) function
        pub fn create_new_task(
            &self,
            block_number: ::ethers::core::types::U256,
            quorum_threshold_percentage: u32,
            quorum_numbers: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [107, 146, 120, 126],
                    (block_number, quorum_threshold_percentage, quorum_numbers),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `delegation` (0xdf5cf723) function
        pub fn delegation(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([223, 92, 247, 35], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dummyForOperatorStateInfoType` (0xc3af8313) function
        pub fn dummy_for_operator_state_info_type(
            &self,
            operator_state_info: OperatorStateInfo,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([195, 175, 131, 19], (operator_state_info,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `generator` (0x7afa1eed) function
        pub fn generator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
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
        ///Calls the contract's `getLatestPendingStateHash` (0x0373408d) function
        pub fn get_latest_pending_state_hash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([3, 115, 64, 141], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOperatorState` (0x3563b0d1) function
        pub fn get_operator_state(
            &self,
            registry_coordinator: ::ethers::core::types::Address,
            quorum_numbers: ::ethers::core::types::Bytes,
            block_number: u32,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::std::vec::Vec<Operator>>,
        > {
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
            (::ethers::core::types::U256, ::std::vec::Vec<::std::vec::Vec<Operator>>),
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
        ///Calls the contract's `getTaskResponseWindowBlock` (0xf5c9899d) function
        pub fn get_task_response_window_block(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([245, 201, 137, 157], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `indexToTaskStatus` (0x99dba0c4) function
        pub fn index_to_task_status(
            &self,
            p0: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([153, 219, 160, 196], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0xf8c8765e) function
        pub fn initialize(
            &self,
            pauser_registry: ::ethers::core::types::Address,
            initial_owner: ::ethers::core::types::Address,
            aggregator: ::ethers::core::types::Address,
            generator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [248, 200, 118, 94],
                    (pauser_registry, initial_owner, aggregator, generator),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `latestTaskNum` (0x8b00ce7c) function
        pub fn latest_task_num(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([139, 0, 206, 124], ())
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
        ///Calls the contract's `registryCoordinator` (0x6d14a987) function
        pub fn registry_coordinator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([109, 20, 169, 135], ())
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
        ///Calls the contract's `respondToTask` (0xcab0e6fd) function
        pub fn respond_to_task(
            &self,
            task: Task,
            task_response: TaskResponse,
            non_signer_stakes_and_signature: NonSignerStakesAndSignature,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [202, 176, 230, 253],
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
        ///Calls the contract's `setStaleStakesForbidden` (0x416c7e5e) function
        pub fn set_stale_stakes_forbidden(
            &self,
            value: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([65, 108, 126, 94], value)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stakeRegistry` (0x68304835) function
        pub fn stake_registry(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([104, 48, 72, 53], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `staleStakesForbidden` (0xb98d0908) function
        pub fn stale_stakes_forbidden(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([185, 141, 9, 8], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `taskNumber` (0x72d18e8d) function
        pub fn task_number(&self) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([114, 209, 142, 141], ())
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
        ///Gets the contract's `NewTaskCreated` event
        pub fn new_task_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NewTaskCreatedFilter,
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
        ///Gets the contract's `TaskCompleted` event
        pub fn task_completed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TaskCompletedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `TaskResponded` event
        pub fn task_responded_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TaskRespondedFilter,
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
            FinalizerTaskManagerEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for FinalizerTaskManager<M> {
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
        name = "NewTaskCreated",
        abi = "NewTaskCreated(uint32,(uint256,uint32,uint32,bytes,uint32,bytes,uint32))"
    )]
    pub struct NewTaskCreatedFilter {
        #[ethevent(indexed)]
        pub task_index: u32,
        pub task: Task,
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
        Hash
    )]
    #[ethevent(name = "TaskCompleted", abi = "TaskCompleted(uint32,bytes32)")]
    pub struct TaskCompletedFilter {
        #[ethevent(indexed)]
        pub task_index: u32,
        #[ethevent(indexed)]
        pub block_hash: [u8; 32],
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
        name = "TaskResponded",
        abi = "TaskResponded((uint32,bytes32,bytes32,bytes32,bytes32,bytes32),(uint32,bytes32,uint96[],uint96[]))"
    )]
    pub struct TaskRespondedFilter {
        pub task_response: TaskResponse,
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
    pub enum FinalizerTaskManagerEvents {
        InitializedFilter(InitializedFilter),
        NewTaskCreatedFilter(NewTaskCreatedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PausedFilter(PausedFilter),
        PauserRegistrySetFilter(PauserRegistrySetFilter),
        StaleStakesForbiddenUpdateFilter(StaleStakesForbiddenUpdateFilter),
        TaskCompletedFilter(TaskCompletedFilter),
        TaskRespondedFilter(TaskRespondedFilter),
        UnpausedFilter(UnpausedFilter),
    }
    impl ::ethers::contract::EthLogDecode for FinalizerTaskManagerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(FinalizerTaskManagerEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = NewTaskCreatedFilter::decode_log(log) {
                return Ok(FinalizerTaskManagerEvents::NewTaskCreatedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(
                    FinalizerTaskManagerEvents::OwnershipTransferredFilter(decoded),
                );
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(FinalizerTaskManagerEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = PauserRegistrySetFilter::decode_log(log) {
                return Ok(FinalizerTaskManagerEvents::PauserRegistrySetFilter(decoded));
            }
            if let Ok(decoded) = StaleStakesForbiddenUpdateFilter::decode_log(log) {
                return Ok(
                    FinalizerTaskManagerEvents::StaleStakesForbiddenUpdateFilter(decoded),
                );
            }
            if let Ok(decoded) = TaskCompletedFilter::decode_log(log) {
                return Ok(FinalizerTaskManagerEvents::TaskCompletedFilter(decoded));
            }
            if let Ok(decoded) = TaskRespondedFilter::decode_log(log) {
                return Ok(FinalizerTaskManagerEvents::TaskRespondedFilter(decoded));
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
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::NewTaskCreatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PausedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauserRegistrySetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StaleStakesForbiddenUpdateFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TaskCompletedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TaskRespondedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnpausedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<InitializedFilter> for FinalizerTaskManagerEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<NewTaskCreatedFilter> for FinalizerTaskManagerEvents {
        fn from(value: NewTaskCreatedFilter) -> Self {
            Self::NewTaskCreatedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter>
    for FinalizerTaskManagerEvents {
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
    impl ::core::convert::From<StaleStakesForbiddenUpdateFilter>
    for FinalizerTaskManagerEvents {
        fn from(value: StaleStakesForbiddenUpdateFilter) -> Self {
            Self::StaleStakesForbiddenUpdateFilter(value)
        }
    }
    impl ::core::convert::From<TaskCompletedFilter> for FinalizerTaskManagerEvents {
        fn from(value: TaskCompletedFilter) -> Self {
            Self::TaskCompletedFilter(value)
        }
    }
    impl ::core::convert::From<TaskRespondedFilter> for FinalizerTaskManagerEvents {
        fn from(value: TaskRespondedFilter) -> Self {
            Self::TaskRespondedFilter(value)
        }
    }
    impl ::core::convert::From<UnpausedFilter> for FinalizerTaskManagerEvents {
        fn from(value: UnpausedFilter) -> Self {
            Self::UnpausedFilter(value)
        }
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
        Hash
    )]
    #[ethcall(name = "aggregator", abi = "aggregator()")]
    pub struct AggregatorCall;
    ///Container type for all input parameters for the `allTaskHashes` function with signature `allTaskHashes(uint32)` and selector `0x2d89f6fc`
    #[derive(
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
    #[ethcall(name = "allTaskHashes", abi = "allTaskHashes(uint32)")]
    pub struct AllTaskHashesCall(pub u32);
    ///Container type for all input parameters for the `allTaskResponses` function with signature `allTaskResponses(uint32)` and selector `0x2cb223d5`
    #[derive(
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
    #[ethcall(name = "allTaskResponses", abi = "allTaskResponses(uint32)")]
    pub struct AllTaskResponsesCall(pub u32);
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
        Hash
    )]
    #[ethcall(name = "blsApkRegistry", abi = "blsApkRegistry()")]
    pub struct BlsApkRegistryCall;
    ///Container type for all input parameters for the `checkSignatures` function with signature `checkSignatures(bytes32,bytes,uint32,(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][],uint32[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256)))` and selector `0xdb67f777`
    #[derive(
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
        abi = "checkSignatures(bytes32,bytes,uint32,(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][],uint32[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256)))"
    )]
    pub struct CheckSignaturesCall {
        pub msg_hash: [u8; 32],
        pub quorum_numbers: ::ethers::core::types::Bytes,
        pub reference_block_number: u32,
        pub params: NonSignerStakesAndSignature,
    }
    ///Container type for all input parameters for the `createNewTask` function with signature `createNewTask(uint256,uint32,bytes)` and selector `0x6b92787e`
    #[derive(
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
    #[ethcall(name = "createNewTask", abi = "createNewTask(uint256,uint32,bytes)")]
    pub struct CreateNewTaskCall {
        pub block_number: ::ethers::core::types::U256,
        pub quorum_threshold_percentage: u32,
        pub quorum_numbers: ::ethers::core::types::Bytes,
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
        Hash
    )]
    #[ethcall(name = "delegation", abi = "delegation()")]
    pub struct DelegationCall;
    ///Container type for all input parameters for the `dummyForOperatorStateInfoType` function with signature `dummyForOperatorStateInfoType((bool,bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]))` and selector `0xc3af8313`
    #[derive(
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
        name = "dummyForOperatorStateInfoType",
        abi = "dummyForOperatorStateInfoType((bool,bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]))"
    )]
    pub struct DummyForOperatorStateInfoTypeCall {
        pub operator_state_info: OperatorStateInfo,
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
        Hash
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
        Hash
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
    ///Container type for all input parameters for the `getLatestPendingStateHash` function with signature `getLatestPendingStateHash()` and selector `0x0373408d`
    #[derive(
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
    #[ethcall(name = "getLatestPendingStateHash", abi = "getLatestPendingStateHash()")]
    pub struct GetLatestPendingStateHashCall;
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
        Hash
    )]
    #[ethcall(name = "getOperatorState", abi = "getOperatorState(address,bytes,uint32)")]
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
        Hash
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
        Hash
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
    ///Container type for all input parameters for the `getTaskResponseWindowBlock` function with signature `getTaskResponseWindowBlock()` and selector `0xf5c9899d`
    #[derive(
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
    #[ethcall(name = "getTaskResponseWindowBlock", abi = "getTaskResponseWindowBlock()")]
    pub struct GetTaskResponseWindowBlockCall;
    ///Container type for all input parameters for the `indexToTaskStatus` function with signature `indexToTaskStatus(uint32)` and selector `0x99dba0c4`
    #[derive(
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
    #[ethcall(name = "indexToTaskStatus", abi = "indexToTaskStatus(uint32)")]
    pub struct IndexToTaskStatusCall(pub u32);
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
        pub aggregator: ::ethers::core::types::Address,
        pub generator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `latestTaskNum` function with signature `latestTaskNum()` and selector `0x8b00ce7c`
    #[derive(
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
    #[ethcall(name = "latestTaskNum", abi = "latestTaskNum()")]
    pub struct LatestTaskNumCall;
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
        Hash
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
        Hash
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `respondToTask` function with signature `respondToTask((uint256,uint32,uint32,bytes,uint32,bytes,uint32),(uint32,bytes32,bytes32,bytes32,bytes32,bytes32),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][],uint32[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256)))` and selector `0xcab0e6fd`
    #[derive(
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
        name = "respondToTask",
        abi = "respondToTask((uint256,uint32,uint32,bytes,uint32,bytes,uint32),(uint32,bytes32,bytes32,bytes32,bytes32,bytes32),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][],uint32[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256)))"
    )]
    pub struct RespondToTaskCall {
        pub task: Task,
        pub task_response: TaskResponse,
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
        Hash
    )]
    #[ethcall(name = "setPauserRegistry", abi = "setPauserRegistry(address)")]
    pub struct SetPauserRegistryCall {
        pub new_pauser_registry: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setStaleStakesForbidden` function with signature `setStaleStakesForbidden(bool)` and selector `0x416c7e5e`
    #[derive(
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
    #[ethcall(name = "setStaleStakesForbidden", abi = "setStaleStakesForbidden(bool)")]
    pub struct SetStaleStakesForbiddenCall {
        pub value: bool,
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
        Hash
    )]
    #[ethcall(name = "stakeRegistry", abi = "stakeRegistry()")]
    pub struct StakeRegistryCall;
    ///Container type for all input parameters for the `staleStakesForbidden` function with signature `staleStakesForbidden()` and selector `0xb98d0908`
    #[derive(
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
    #[ethcall(name = "staleStakesForbidden", abi = "staleStakesForbidden()")]
    pub struct StaleStakesForbiddenCall;
    ///Container type for all input parameters for the `taskNumber` function with signature `taskNumber()` and selector `0x72d18e8d`
    #[derive(
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
    #[ethcall(name = "taskNumber", abi = "taskNumber()")]
    pub struct TaskNumberCall;
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
    pub enum FinalizerTaskManagerCalls {
        Aggregator(AggregatorCall),
        AllTaskHashes(AllTaskHashesCall),
        AllTaskResponses(AllTaskResponsesCall),
        BlsApkRegistry(BlsApkRegistryCall),
        CheckSignatures(CheckSignaturesCall),
        CreateNewTask(CreateNewTaskCall),
        Delegation(DelegationCall),
        DummyForOperatorStateInfoType(DummyForOperatorStateInfoTypeCall),
        Generator(GeneratorCall),
        GetCheckSignaturesIndices(GetCheckSignaturesIndicesCall),
        GetLatestPendingStateHash(GetLatestPendingStateHashCall),
        GetOperatorState(GetOperatorStateCall),
        GetOperatorStateWithRegistryCoordinatorAndOperatorId(
            GetOperatorStateWithRegistryCoordinatorAndOperatorIdCall,
        ),
        GetQuorumBitmapsAtBlockNumber(GetQuorumBitmapsAtBlockNumberCall),
        GetTaskResponseWindowBlock(GetTaskResponseWindowBlockCall),
        IndexToTaskStatus(IndexToTaskStatusCall),
        Initialize(InitializeCall),
        LatestTaskNum(LatestTaskNumCall),
        Owner(OwnerCall),
        Pause(PauseCall),
        PauseAll(PauseAllCall),
        PausedWithIndex(PausedWithIndexCall),
        Paused(PausedCall),
        PauserRegistry(PauserRegistryCall),
        RegistryCoordinator(RegistryCoordinatorCall),
        RenounceOwnership(RenounceOwnershipCall),
        RespondToTask(RespondToTaskCall),
        SetPauserRegistry(SetPauserRegistryCall),
        SetStaleStakesForbidden(SetStaleStakesForbiddenCall),
        StakeRegistry(StakeRegistryCall),
        StaleStakesForbidden(StaleStakesForbiddenCall),
        TaskNumber(TaskNumberCall),
        TransferOwnership(TransferOwnershipCall),
        TrySignatureAndApkVerification(TrySignatureAndApkVerificationCall),
        Unpause(UnpauseCall),
    }
    impl ::ethers::core::abi::AbiDecode for FinalizerTaskManagerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AggregatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Aggregator(decoded));
            }
            if let Ok(decoded) = <AllTaskHashesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AllTaskHashes(decoded));
            }
            if let Ok(decoded) = <AllTaskResponsesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AllTaskResponses(decoded));
            }
            if let Ok(decoded) = <BlsApkRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BlsApkRegistry(decoded));
            }
            if let Ok(decoded) = <CheckSignaturesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CheckSignatures(decoded));
            }
            if let Ok(decoded) = <CreateNewTaskCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateNewTask(decoded));
            }
            if let Ok(decoded) = <DelegationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Delegation(decoded));
            }
            if let Ok(decoded) = <DummyForOperatorStateInfoTypeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DummyForOperatorStateInfoType(decoded));
            }
            if let Ok(decoded) = <GeneratorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Generator(decoded));
            }
            if let Ok(decoded) = <GetCheckSignaturesIndicesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetCheckSignaturesIndices(decoded));
            }
            if let Ok(decoded) = <GetLatestPendingStateHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetLatestPendingStateHash(decoded));
            }
            if let Ok(decoded) = <GetOperatorStateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetOperatorState(decoded));
            }
            if let Ok(decoded) = <GetOperatorStateWithRegistryCoordinatorAndOperatorIdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::GetOperatorStateWithRegistryCoordinatorAndOperatorId(decoded),
                );
            }
            if let Ok(decoded) = <GetQuorumBitmapsAtBlockNumberCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetQuorumBitmapsAtBlockNumber(decoded));
            }
            if let Ok(decoded) = <GetTaskResponseWindowBlockCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTaskResponseWindowBlock(decoded));
            }
            if let Ok(decoded) = <IndexToTaskStatusCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IndexToTaskStatus(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <LatestTaskNumCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LatestTaskNum(decoded));
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
            if let Ok(decoded) = <RegistryCoordinatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RegistryCoordinator(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <RespondToTaskCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RespondToTask(decoded));
            }
            if let Ok(decoded) = <SetPauserRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetPauserRegistry(decoded));
            }
            if let Ok(decoded) = <SetStaleStakesForbiddenCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetStaleStakesForbidden(decoded));
            }
            if let Ok(decoded) = <StakeRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StakeRegistry(decoded));
            }
            if let Ok(decoded) = <StaleStakesForbiddenCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StaleStakesForbidden(decoded));
            }
            if let Ok(decoded) = <TaskNumberCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TaskNumber(decoded));
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
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for FinalizerTaskManagerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Aggregator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AllTaskHashes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AllTaskResponses(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BlsApkRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CheckSignatures(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateNewTask(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Delegation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DummyForOperatorStateInfoType(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Generator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetCheckSignaturesIndices(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetLatestPendingStateHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOperatorState(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOperatorStateWithRegistryCoordinatorAndOperatorId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetQuorumBitmapsAtBlockNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTaskResponseWindowBlock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IndexToTaskStatus(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LatestTaskNum(element) => {
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
                Self::RegistryCoordinator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RespondToTask(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPauserRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetStaleStakesForbidden(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StakeRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StaleStakesForbidden(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TaskNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TrySignatureAndApkVerification(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unpause(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for FinalizerTaskManagerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Aggregator(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllTaskHashes(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllTaskResponses(element) => ::core::fmt::Display::fmt(element, f),
                Self::BlsApkRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckSignatures(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateNewTask(element) => ::core::fmt::Display::fmt(element, f),
                Self::Delegation(element) => ::core::fmt::Display::fmt(element, f),
                Self::DummyForOperatorStateInfoType(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Generator(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCheckSignaturesIndices(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetLatestPendingStateHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetOperatorState(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOperatorStateWithRegistryCoordinatorAndOperatorId(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetQuorumBitmapsAtBlockNumber(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTaskResponseWindowBlock(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IndexToTaskStatus(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestTaskNum(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pause(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauseAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::PausedWithIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::Paused(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauserRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegistryCoordinator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::RespondToTask(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPauserRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetStaleStakesForbidden(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StakeRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::StaleStakesForbidden(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TaskNumber(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::TrySignatureAndApkVerification(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Unpause(element) => ::core::fmt::Display::fmt(element, f),
            }
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
    impl ::core::convert::From<BlsApkRegistryCall> for FinalizerTaskManagerCalls {
        fn from(value: BlsApkRegistryCall) -> Self {
            Self::BlsApkRegistry(value)
        }
    }
    impl ::core::convert::From<CheckSignaturesCall> for FinalizerTaskManagerCalls {
        fn from(value: CheckSignaturesCall) -> Self {
            Self::CheckSignatures(value)
        }
    }
    impl ::core::convert::From<CreateNewTaskCall> for FinalizerTaskManagerCalls {
        fn from(value: CreateNewTaskCall) -> Self {
            Self::CreateNewTask(value)
        }
    }
    impl ::core::convert::From<DelegationCall> for FinalizerTaskManagerCalls {
        fn from(value: DelegationCall) -> Self {
            Self::Delegation(value)
        }
    }
    impl ::core::convert::From<DummyForOperatorStateInfoTypeCall>
    for FinalizerTaskManagerCalls {
        fn from(value: DummyForOperatorStateInfoTypeCall) -> Self {
            Self::DummyForOperatorStateInfoType(value)
        }
    }
    impl ::core::convert::From<GeneratorCall> for FinalizerTaskManagerCalls {
        fn from(value: GeneratorCall) -> Self {
            Self::Generator(value)
        }
    }
    impl ::core::convert::From<GetCheckSignaturesIndicesCall>
    for FinalizerTaskManagerCalls {
        fn from(value: GetCheckSignaturesIndicesCall) -> Self {
            Self::GetCheckSignaturesIndices(value)
        }
    }
    impl ::core::convert::From<GetLatestPendingStateHashCall>
    for FinalizerTaskManagerCalls {
        fn from(value: GetLatestPendingStateHashCall) -> Self {
            Self::GetLatestPendingStateHash(value)
        }
    }
    impl ::core::convert::From<GetOperatorStateCall> for FinalizerTaskManagerCalls {
        fn from(value: GetOperatorStateCall) -> Self {
            Self::GetOperatorState(value)
        }
    }
    impl ::core::convert::From<GetOperatorStateWithRegistryCoordinatorAndOperatorIdCall>
    for FinalizerTaskManagerCalls {
        fn from(
            value: GetOperatorStateWithRegistryCoordinatorAndOperatorIdCall,
        ) -> Self {
            Self::GetOperatorStateWithRegistryCoordinatorAndOperatorId(value)
        }
    }
    impl ::core::convert::From<GetQuorumBitmapsAtBlockNumberCall>
    for FinalizerTaskManagerCalls {
        fn from(value: GetQuorumBitmapsAtBlockNumberCall) -> Self {
            Self::GetQuorumBitmapsAtBlockNumber(value)
        }
    }
    impl ::core::convert::From<GetTaskResponseWindowBlockCall>
    for FinalizerTaskManagerCalls {
        fn from(value: GetTaskResponseWindowBlockCall) -> Self {
            Self::GetTaskResponseWindowBlock(value)
        }
    }
    impl ::core::convert::From<IndexToTaskStatusCall> for FinalizerTaskManagerCalls {
        fn from(value: IndexToTaskStatusCall) -> Self {
            Self::IndexToTaskStatus(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for FinalizerTaskManagerCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<LatestTaskNumCall> for FinalizerTaskManagerCalls {
        fn from(value: LatestTaskNumCall) -> Self {
            Self::LatestTaskNum(value)
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
    impl ::core::convert::From<RespondToTaskCall> for FinalizerTaskManagerCalls {
        fn from(value: RespondToTaskCall) -> Self {
            Self::RespondToTask(value)
        }
    }
    impl ::core::convert::From<SetPauserRegistryCall> for FinalizerTaskManagerCalls {
        fn from(value: SetPauserRegistryCall) -> Self {
            Self::SetPauserRegistry(value)
        }
    }
    impl ::core::convert::From<SetStaleStakesForbiddenCall>
    for FinalizerTaskManagerCalls {
        fn from(value: SetStaleStakesForbiddenCall) -> Self {
            Self::SetStaleStakesForbidden(value)
        }
    }
    impl ::core::convert::From<StakeRegistryCall> for FinalizerTaskManagerCalls {
        fn from(value: StakeRegistryCall) -> Self {
            Self::StakeRegistry(value)
        }
    }
    impl ::core::convert::From<StaleStakesForbiddenCall> for FinalizerTaskManagerCalls {
        fn from(value: StaleStakesForbiddenCall) -> Self {
            Self::StaleStakesForbidden(value)
        }
    }
    impl ::core::convert::From<TaskNumberCall> for FinalizerTaskManagerCalls {
        fn from(value: TaskNumberCall) -> Self {
            Self::TaskNumber(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for FinalizerTaskManagerCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<TrySignatureAndApkVerificationCall>
    for FinalizerTaskManagerCalls {
        fn from(value: TrySignatureAndApkVerificationCall) -> Self {
            Self::TrySignatureAndApkVerification(value)
        }
    }
    impl ::core::convert::From<UnpauseCall> for FinalizerTaskManagerCalls {
        fn from(value: UnpauseCall) -> Self {
            Self::Unpause(value)
        }
    }
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
        Hash
    )]
    pub struct AggregatorReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `allTaskHashes` function with signature `allTaskHashes(uint32)` and selector `0x2d89f6fc`
    #[derive(
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
    pub struct AllTaskHashesReturn(pub [u8; 32]);
    ///Container type for all return fields from the `allTaskResponses` function with signature `allTaskResponses(uint32)` and selector `0x2cb223d5`
    #[derive(
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
    pub struct AllTaskResponsesReturn(pub [u8; 32]);
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
        Hash
    )]
    pub struct BlsApkRegistryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `checkSignatures` function with signature `checkSignatures(bytes32,bytes,uint32,(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][],uint32[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256)))` and selector `0xdb67f777`
    #[derive(
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
        Hash
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
        Hash
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
        Hash
    )]
    pub struct GetCheckSignaturesIndicesReturn(pub CheckSignaturesIndices);
    ///Container type for all return fields from the `getLatestPendingStateHash` function with signature `getLatestPendingStateHash()` and selector `0x0373408d`
    #[derive(
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
    pub struct GetLatestPendingStateHashReturn(pub [u8; 32]);
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
        Hash
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
        Hash
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
        Hash
    )]
    pub struct GetQuorumBitmapsAtBlockNumberReturn(
        pub ::std::vec::Vec<::ethers::core::types::U256>,
    );
    ///Container type for all return fields from the `getTaskResponseWindowBlock` function with signature `getTaskResponseWindowBlock()` and selector `0xf5c9899d`
    #[derive(
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
    pub struct GetTaskResponseWindowBlockReturn(pub u32);
    ///Container type for all return fields from the `indexToTaskStatus` function with signature `indexToTaskStatus(uint32)` and selector `0x99dba0c4`
    #[derive(
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
    pub struct IndexToTaskStatusReturn(pub u8);
    ///Container type for all return fields from the `latestTaskNum` function with signature `latestTaskNum()` and selector `0x8b00ce7c`
    #[derive(
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
    pub struct LatestTaskNumReturn(pub u32);
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
        Hash
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
        Hash
    )]
    pub struct StakeRegistryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `staleStakesForbidden` function with signature `staleStakesForbidden()` and selector `0xb98d0908`
    #[derive(
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
    pub struct StaleStakesForbiddenReturn(pub bool);
    ///Container type for all return fields from the `taskNumber` function with signature `taskNumber()` and selector `0x72d18e8d`
    #[derive(
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
    pub struct TaskNumberReturn(pub u32);
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
    ///`NonSignerStakesAndSignature(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][],uint32[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256))`
    #[derive(
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
    pub struct NonSignerStakesAndSignature {
        pub non_signer_quorum_bitmap_indices: ::std::vec::Vec<u32>,
        pub non_signer_pubkeys: ::std::vec::Vec<G1Point>,
        pub quorum_apks: ::std::vec::Vec<G1Point>,
        pub apk_g2: G2Point,
        pub sigma: G1Point,
        pub quorum_apk_indices: ::std::vec::Vec<u32>,
        pub total_stake_indices: ::std::vec::Vec<u32>,
        pub non_signer_stake_indices: ::std::vec::Vec<::std::vec::Vec<u32>>,
        pub non_signer_pubkeys_indicesfor_operator_ids_removed_for_old_state: ::std::vec::Vec<
            u32,
        >,
        pub non_signer_pubkeys_added_for_old_state: ::std::vec::Vec<G1Point>,
        pub apk_g_2for_old_state: G2Point,
        pub sigmafor_old_sate: G1Point,
    }
    ///`QuorumStakeTotals(uint96[],uint96[])`
    #[derive(
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
    pub struct QuorumStakeTotals {
        pub signed_stake_for_quorum: ::std::vec::Vec<u128>,
        pub total_stake_for_quorum: ::std::vec::Vec<u128>,
    }
    ///`OperatorStateInfo(bool,bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[])`
    #[derive(
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
    pub struct OperatorStateInfo {
        pub operators_state_changed: bool,
        pub operators_state_provided: bool,
        pub quorums_removed: ::std::vec::Vec<u8>,
        pub quorums_added: ::std::vec::Vec<QuorumsAdded>,
        pub quorums_stake_update: ::std::vec::Vec<QuorumsStakeUpdate>,
        pub quorums_apk_update: ::std::vec::Vec<QuorumsApkUpdate>,
        pub operators_removed: ::std::vec::Vec<[u8; 32]>,
        pub operators_added: ::std::vec::Vec<OperatorsAdded>,
        pub operators_stake_update: ::std::vec::Vec<OperatorsStakeUpdate>,
        pub operators_quorum_count_update: ::std::vec::Vec<OperatorsQuorumCountUpdate>,
    }
    ///`OperatorsAdded(bytes32,uint8[],uint96[],uint8)`
    #[derive(
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
    pub struct OperatorsAdded {
        pub operator_id: [u8; 32],
        pub quorum_for_stakes: ::std::vec::Vec<u8>,
        pub quorum_stakes: ::std::vec::Vec<u128>,
        pub quorum_count: u8,
    }
    ///`OperatorsQuorumCountUpdate(bytes32,uint8)`
    #[derive(
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
    pub struct OperatorsQuorumCountUpdate {
        pub operator_id: [u8; 32],
        pub quorum_count: u8,
    }
    ///`OperatorsStakeUpdate(bytes32,uint8[],uint96[])`
    #[derive(
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
    pub struct OperatorsStakeUpdate {
        pub operator_id: [u8; 32],
        pub quorum_for_stakes: ::std::vec::Vec<u8>,
        pub quorum_stakes: ::std::vec::Vec<u128>,
    }
    ///`QuorumsAdded(uint8,uint96,(uint256,uint256))`
    #[derive(
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
    pub struct QuorumsAdded {
        pub quorum_number: u8,
        pub quorum_stake: u128,
        pub quorum_apk: G1Point,
    }
    ///`QuorumsApkUpdate(uint8,(uint256,uint256))`
    #[derive(
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
    pub struct QuorumsApkUpdate {
        pub quorum_number: u8,
        pub quorum_apk: G1Point,
    }
    ///`QuorumsStakeUpdate(uint8,uint96)`
    #[derive(
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
    pub struct QuorumsStakeUpdate {
        pub quorum_number: u8,
        pub quorum_stake: u128,
    }
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
        Hash
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
        Hash
    )]
    pub struct Operator {
        pub operator: ::ethers::core::types::Address,
        pub operator_id: [u8; 32],
        pub stake: u128,
    }
}
