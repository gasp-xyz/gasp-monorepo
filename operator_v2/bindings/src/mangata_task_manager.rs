pub use mangata_task_manager::*;
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
pub mod mangata_task_manager {
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
                                "contract IBLSRegistryCoordinatorWithIndices",
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
                    ::std::borrow::ToOwned::to_owned("TASK_CHALLENGE_WINDOW_BLOCK"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "TASK_CHALLENGE_WINDOW_BLOCK",
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
                    ::std::borrow::ToOwned::to_owned("TASK_RESPONSE_WINDOW_BLOCK"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "TASK_RESPONSE_WINDOW_BLOCK",
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
                    ::std::borrow::ToOwned::to_owned("blsPubkeyRegistry"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("blsPubkeyRegistry"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IBLSPubkeyRegistry",
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
                                            "contract IBLSRegistryCoordinatorWithIndices",
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
                                            "struct BLSOperatorStateRetriever.CheckSignaturesIndices",
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
                                            "contract IBLSRegistryCoordinatorWithIndices",
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
                                            "struct BLSOperatorStateRetriever.Operator[][]",
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
                                            "contract IBLSRegistryCoordinatorWithIndices",
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
                                            "struct BLSOperatorStateRetriever.Operator[][]",
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
                    ::std::borrow::ToOwned::to_owned("getTaskChallangeWindowBlock"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getTaskChallangeWindowBlock",
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
                    ::std::borrow::ToOwned::to_owned("raiseAndResolveChallenge"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "raiseAndResolveChallenge",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("task"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IMangataTaskManager.Task",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("taskResponse"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IMangataTaskManager.TaskResponse",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "taskResponseMetadata",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IMangataTaskManager.TaskResponseMetadata",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "pubkeysOfNonSigningOperators",
                                    ),
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
                                        ::std::borrow::ToOwned::to_owned("struct BN254.G1Point[]"),
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
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IMangataTaskManager.Task",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("taskResponse"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IMangataTaskManager.TaskResponse",
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
                    ::std::borrow::ToOwned::to_owned("taskSuccesfullyChallenged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "taskSuccesfullyChallenged",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("TaskChallengedSuccessfully"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "TaskChallengedSuccessfully",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("taskIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("challenger"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TaskChallengedUnsuccessfully"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "TaskChallengedUnsuccessfully",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("taskIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("challenger"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
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
    pub static MANGATATASKMANAGER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01\0`@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@Qb\0Q\xAA8\x03\x80b\0Q\xAA\x839\x81\x01`@\x81\x90Rb\0\x005\x91b\0\x01iV[\x81\x80`\x01`\x01`\xA0\x1B\x03\x16`\x80\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x80`\x01`\x01`\xA0\x1B\x03\x16ch0H5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\0\x8FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\0\xB5\x91\x90b\0\x01\xB0V[`\x01`\x01`\xA0\x1B\x03\x16`\xA0\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x80`\x01`\x01`\xA0\x1B\x03\x16c5a\xDE\xB1`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x01\rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x013\x91\x90b\0\x01\xB0V[`\x01`\x01`\xA0\x1B\x03\x16`\xC0RPc\xFF\xFF\xFF\xFF\x16`\xE0RPb\0\x01\xD7V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01fW`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15b\0\x01}W`\0\x80\xFD[\x82Qb\0\x01\x8A\x81b\0\x01PV[` \x84\x01Q\x90\x92Pc\xFF\xFF\xFF\xFF\x81\x16\x81\x14b\0\x01\xA5W`\0\x80\xFD[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15b\0\x01\xC3W`\0\x80\xFD[\x81Qb\0\x01\xD0\x81b\0\x01PV[\x93\x92PPPV[`\x80Q`\xA0Q`\xC0Q`\xE0QaOhb\0\x02B`\09`\0\x81\x81a\x02\\\x01R\x81\x81a\x05\x1B\x01Ra\n\xE3\x01R`\0\x81\x81a\x03$\x01R\x81\x81a\x15\x0C\x01Ra\x1F\xC9\x01R`\0\x81\x81a\x04\x04\x01R\x81\x81a%\xCA\x01Ra'k\x01R`\0\x81\x81a\x04>\x01Ra$\x1E\x01RaOh`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xFBW`\x005`\xE0\x1C\x80ch0H5\x11a\x01\x1AW\x80c\x8B\0\xCE|\x11a\0\xADW\x80c\xF2\xFD\xE3\x8B\x11a\0|W\x80c\xF2\xFD\xE3\x8B\x14a\x05\x06W\x80c\xF5\xC9\x89\x9D\x14a\x05\x19W\x80c\xF6<[\xAB\x14a\x05?W\x80c\xF8\xC8v^\x14a\x05GW\x80c\xFA\xBC\x1C\xBC\x14a\x05ZW`\0\x80\xFD[\x80c\x8B\0\xCE|\x14a\x04\xBDW\x80c\x8D\xA5\xCB[\x14a\x04\xCDW\x80c\xCE\xFD\xC1\xD4\x14a\x04\xDEW\x80c\xE3\xB5\xEE\x04\x14a\x04\xFFW`\0\x80\xFD[\x80cqP\x18\xA6\x11a\0\xE9W\x80cqP\x18\xA6\x14a\x04\x81W\x80cr\xD1\x8E\x8D\x14a\x04\x89W\x80cz\xFA\x1E\xED\x14a\x04\x97W\x80c\x88o\x11\x95\x14a\x04\xAAW`\0\x80\xFD[\x80ch0H5\x14a\x03\xFFW\x80ck\x92x~\x14a\x04&W\x80cm\x14\xA9\x87\x14a\x049W\x80cn\xFBF6\x14a\x04`W`\0\x80\xFD[\x80c5a\xDE\xB1\x11a\x01\x92W\x80cY\\jg\x11a\x01aW\x80cY\\jg\x14a\x03\x99W\x80cZ\xC8j\xB7\x14a\x03\xA1W\x80c\\\x97Z\xBB\x14a\x03\xD4W\x80c]\xEC\xC3\xF5\x14a\x03\xDCW`\0\x80\xFD[\x80c5a\xDE\xB1\x14a\x03\x1FW\x80c5c\xB0\xD1\x14a\x03FW\x80c<\xD2\x7Fd\x14a\x03fW\x80cOs\x9Ft\x14a\x03yW`\0\x80\xFD[\x80c$Z{\xFC\x11a\x01\xCEW\x80c$Z{\xFC\x14a\x02\x93W\x80c'|\xB9\x95\x14a\x02\xBEW\x80c,\xB2#\xD5\x14a\x02\xD1W\x80c-\x89\xF6\xFC\x14a\x02\xFFW`\0\x80\xFD[\x80c\x10\xD6z/\x14a\x02\0W\x80c\x13d9\xDD\x14a\x02\x15W\x80c\x17\x1F\x1D[\x14a\x02(W\x80c\x1A\xD41\x89\x14a\x02WW[`\0\x80\xFD[a\x02\x13a\x02\x0E6`\x04a;\xC6V[a\x05mV[\0[a\x02\x13a\x02#6`\x04a;\xE3V[a\x06)V[a\x02;a\x0266`\x04a=aV[a\x07hV[`@\x80Q\x92\x15\x15\x83R\x90\x15\x15` \x83\x01R\x01[`@Q\x80\x91\x03\x90\xF3[a\x02~\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02NV[`\x9BTa\x02\xA6\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02NV[a\x02\x13a\x02\xCC6`\x04a@\x9DV[a\x08\xF2V[a\x02\xF1a\x02\xDF6`\x04aA\x11V[`\x99` R`\0\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01a\x02NV[a\x02\xF1a\x03\r6`\x04aA\x11V[`\x98` R`\0\x90\x81R`@\x90 T\x81V[a\x02\xA6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03Ya\x03T6`\x04aA.V[a\rqV[`@Qa\x02N\x91\x90aBuV[a\x02\x13a\x03t6`\x04aB\x88V[a\x10\xECV[a\x03\x8Ca\x03\x876`\x04aCVV[a\x16\xA6V[`@Qa\x02N\x91\x90aDZV[a\x02\x13a\x1D*V[a\x03\xC4a\x03\xAF6`\x04aE\x15V[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`@Q\x90\x15\x15\x81R` \x01a\x02NV[`fTa\x02\xF1V[a\x03\xC4a\x03\xEA6`\x04aA\x11V[`\x9A` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x02\xA6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\x13a\x0446`\x04aE8V[a\x1D\xF1V[a\x02\xA6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x04sa\x04n6`\x04aE\x93V[a\x1F\x92V[`@Qa\x02N\x92\x91\x90aFSV[a\x02\x13a)\xFEV[`\x97Tc\xFF\xFF\xFF\xFF\x16a\x02~V[`\x9CTa\x02\xA6\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`eTa\x02\xA6\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x97Ta\x02~\x90c\xFF\xFF\xFF\xFF\x16\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x02\xA6V[a\x04\xF1a\x04\xEC6`\x04aF\x9CV[a*\x12V[`@Qa\x02N\x92\x91\x90aF\xDEV[`da\x02~V[a\x02\x13a\x05\x146`\x04a;\xC6V[a+\xA4V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x02~V[a\x02~`d\x81V[a\x02\x13a\x05U6`\x04aF\xFFV[a,\x1AV[a\x02\x13a\x05h6`\x04a;\xE3V[a-kV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xC0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xE4\x91\x90aG[V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x06\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x14\x90aGxV[`@Q\x80\x91\x03\x90\xFD[a\x06&\x81a.\xC7V[PV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06qW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x95\x91\x90aG\xC2V[a\x06\xB1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x14\x90aG\xE4V[`fT\x81\x81\x16\x14a\x07*W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x14V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`\0\x80`\0\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x87\x87`\0\x01Q\x88` \x01Q\x88`\0\x01Q`\0`\x02\x81\x10a\x07\xB0Wa\x07\xB0aH,V[` \x02\x01Q\x89Q`\x01` \x02\x01Q\x8A` \x01Q`\0`\x02\x81\x10a\x07\xD5Wa\x07\xD5aH,V[` \x02\x01Q\x8B` \x01Q`\x01`\x02\x81\x10a\x07\xF1Wa\x07\xF1aH,V[` \x90\x81\x02\x91\x90\x91\x01Q\x8CQ\x8D\x83\x01Q`@Qa\x08N\x9A\x99\x98\x97\x96\x95\x94\x01\x98\x89R` \x89\x01\x97\x90\x97R`@\x88\x01\x95\x90\x95R``\x87\x01\x93\x90\x93R`\x80\x86\x01\x91\x90\x91R`\xA0\x85\x01R`\xC0\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01Ra\x01 \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\0\x1Ca\x08q\x91\x90aHBV[\x90Pa\x08\xE4a\x08\x8Aa\x08\x83\x88\x84a/\xBEV[\x86\x90a0UV[a\x08\x92a0\xE9V[a\x08\xDAa\x08\xCB\x85a\x08\xC5`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x01\x82R`\x02\x90\x82\x01R\x90V[\x90a/\xBEV[a\x08\xD4\x8Ca1\xA9V[\x90a0UV[\x88b\x01\xD4\xC0a29V[\x90\x98\x90\x97P\x95PPPPPPV[`\x9BT`\x01`\x01`\xA0\x1B\x03\x163\x14a\tLW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAggregator must be the caller\0\0\0`D\x82\x01R`d\x01a\x06\x14V[`\0a\t^`@\x85\x01` \x86\x01aA\x11V[\x90P6`\0a\tp`@\x87\x01\x87aHdV[\x90\x92P\x90P`\0a\t\x87`\x80\x88\x01``\x89\x01aA\x11V[\x90P`\x98`\0a\t\x9A` \x89\x01\x89aA\x11V[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x87`@Q` \x01a\t\xC6\x91\x90aH\xD3V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x14a\nOW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7Fsupplied task does not match the`D\x82\x01R\x7F one recorded in the contract\0\0\0`d\x82\x01R`\x84\x01a\x06\x14V[`\0`\x99\x81a\na` \x8A\x01\x8AaA\x11V[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x14a\n\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FAggregator has already responded`D\x82\x01Rk to the task`\xA0\x1B`d\x82\x01R`\x84\x01a\x06\x14V[a\x0B\x08\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85aI\x8AV[c\xFF\xFF\xFF\xFF\x16Cc\xFF\xFF\xFF\xFF\x16\x11\x15a\x0ByW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FAggregator has responded to the `D\x82\x01Rltask too late`\x98\x1B`d\x82\x01R`\x84\x01a\x06\x14V[`\0\x86`@Q` \x01a\x0B\x8C\x91\x90aI\xD0V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x80a\x0B\xB4\x83\x87\x87\x8A\x8Ca\x1F\x92V[\x91P\x91P`\0[\x85\x81\x10\x15a\x0C\xB3W\x84`\xFF\x16\x83` \x01Q\x82\x81Q\x81\x10a\x0B\xDDWa\x0B\xDDaH,V[` \x02` \x01\x01Qa\x0B\xEF\x91\x90aI\xDEV[`\x01`\x01``\x1B\x03\x16`d\x84`\0\x01Q\x83\x81Q\x81\x10a\x0C\x10Wa\x0C\x10aH,V[` \x02` \x01\x01Q`\x01`\x01``\x1B\x03\x16a\x0C+\x91\x90aJ\rV[\x10\x15a\x0C\xA1W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FSignatories do not own at least `D\x82\x01R\x7Fthreshold percentage of a quorum`d\x82\x01R`\x84\x01a\x06\x14V[\x80a\x0C\xAB\x81aJ,V[\x91PPa\x0B\xBBV[P`@\x80Q\x80\x82\x01\x82Rc\xFF\xFF\xFF\xFFC\x16\x81R` \x80\x82\x01\x84\x90R\x91Q\x90\x91a\x0C\xE0\x91\x8C\x91\x84\x91\x01aJGV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\x99`\0\x8C`\0\x01` \x81\x01\x90a\r\r\x91\x90aA\x11V[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP\x7F\xF2\xAF\x11\xFA\xD7=CI\xC9\x9C\xF6/)\x8D3vA\xEA\x0B\xB7\xC0\xF5\xA8\xDB\x92\xA9\x8A'_sOX\x8A\x82`@Qa\r\\\x92\x91\x90aJGV[`@Q\x80\x91\x03\x90\xA1PPPPPPPPPPPV[```\0\x84`\x01`\x01`\xA0\x1B\x03\x16ch0H5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xB3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xD7\x91\x90aG[V[\x90P`\0\x85`\x01`\x01`\xA0\x1B\x03\x16c\x9E\x99#\xC2`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\x19W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E=\x91\x90aG[V[\x90P`\0\x85Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0EZWa\x0EZa;\xFCV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E\x8DW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0ExW\x90P[P\x90P`\0[\x86Q\x81\x10\x15a\x10\xDFW`\0\x87\x82\x81Q\x81\x10a\x0E\xB0Wa\x0E\xB0aH,V[\x01` \x01Q`@Qc\x88\x9A\xE3\xE5`\xE0\x1B\x81R`\xF8\x91\x90\x91\x1C`\x04\x82\x01\x81\x90Rc\xFF\xFF\xFF\xFF\x89\x16`$\x83\x01R\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x86\x16\x90c\x88\x9A\xE3\xE5\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x11W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0F9\x91\x90\x81\x01\x90aJsV[\x90P\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0FTWa\x0FTa;\xFCV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0F\x99W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0FrW\x90P[P\x84\x84\x81Q\x81\x10a\x0F\xACWa\x0F\xACaH,V[` \x02` \x01\x01\x81\x90RP`\0[\x81Q\x81\x10\x15a\x10\xC9W`\0\x82\x82\x81Q\x81\x10a\x0F\xD7Wa\x0F\xD7aH,V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`@\x80Q\x80\x82\x01\x82R\x82\x81R\x90Qc\x1B2r%`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`\xFF\x88\x16`$\x82\x01Rc\xFF\xFF\xFF\xFF\x8E\x16`D\x82\x01R\x91\x93P\x91\x82\x01\x90`\x01`\x01`\xA0\x1B\x03\x8B\x16\x90c\x1B2r%\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10OW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10s\x91\x90aK\x03V[`\x01`\x01``\x1B\x03\x16\x81RP\x86\x86\x81Q\x81\x10a\x10\x91Wa\x10\x91aH,V[` \x02` \x01\x01Q\x83\x81Q\x81\x10a\x10\xAAWa\x10\xAAaH,V[` \x02` \x01\x01\x81\x90RPP\x80\x80a\x10\xC1\x90aJ,V[\x91PPa\x0F\xBAV[PPP\x80\x80a\x10\xD7\x90aJ,V[\x91PPa\x0E\x93V[P\x92PPP[\x93\x92PPPV[`\0a\x10\xFB` \x85\x01\x85aA\x11V[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x99` R`@\x90 T\x90\x91P\x855\x90a\x11mW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FTask hasn't been responded to ye`D\x82\x01R`\x1D`\xFA\x1B`d\x82\x01R`\x84\x01a\x06\x14V[\x84\x84`@Q` \x01a\x11\x80\x92\x91\x90aK,V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 c\xFF\xFF\xFF\xFF\x85\x16`\0\x90\x81R`\x99\x90\x93R\x91 T\x14a\x12\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FTask response does not match the`D\x82\x01R\x7F one recorded in the contract\0\0\0`d\x82\x01R`\x84\x01a\x06\x14V[c\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`\x9A` R`@\x90 T`\xFF\x16\x15a\x12\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FThe response to this task has al`D\x82\x01R\x7Fready been challenged successful`d\x82\x01Rb6<\x97`\xE9\x1B`\x84\x82\x01R`\xA4\x01a\x06\x14V[`da\x12\xC6` \x86\x01\x86aA\x11V[a\x12\xD0\x91\x90aI\x8AV[c\xFF\xFF\xFF\xFF\x16Cc\xFF\xFF\xFF\xFF\x16\x11\x15a\x13QW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FThe challenge period for this ta`D\x82\x01R\x7Fsk has already expired.\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x14V[`\x01`@Q3\x90c\xFF\xFF\xFF\xFF\x85\x16\x90\x7F\xFD>&\xBE\xEBYg\xFCZW\xA0Di\x14\xEA\xBCE\xB4\xAAGLg\xA5\x1BKQ`\xCA\xC6\r\xDB\x05\x90`\0\x90\xA3PPPa\x16\xA0V[\x86Q\x81\x10\x15a\x13\xE8Wa\x13\xB9\x87\x82\x81Q\x81\x10a\x13\xACWa\x13\xACaH,V[` \x02` \x01\x01Qa4]V[\x82\x82\x81Q\x81\x10a\x13\xCBWa\x13\xCBaH,V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a\x13\xE0\x81aJ,V[\x91PPa\x13\x8EV[P`\0a\x13\xFB`@\x8B\x01` \x8C\x01aA\x11V[\x82`@Q` \x01a\x14\r\x92\x91\x90aKGV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x87` \x015\x81\x14a\x14\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`P`$\x82\x01R\x7FThe pubkeys of non-signing opera`D\x82\x01R\x7Ftors supplied by the challenger `d\x82\x01Ro0\xB92\x9077\xBA\x101\xB7\xB992\xB1\xBA\x17`\x81\x1B`\x84\x82\x01R`\xA4\x01a\x06\x14V[`\0\x87Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14\xD2Wa\x14\xD2a;\xFCV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x14\xFBW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x88Q\x81\x10\x15a\x16OW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x18uH\xC8`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15hW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x8C\x91\x90aG[V[`\x01`\x01`\xA0\x1B\x03\x16c\xE8\xBB\x9A\xE6\x85\x83\x81Q\x81\x10a\x15\xACWa\x15\xACaH,V[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x15\xD2\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\xEFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\x13\x91\x90aG[V[\x82\x82\x81Q\x81\x10a\x16%Wa\x16%aH,V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x16G\x81aJ,V[\x91PPa\x15\x01V[Pc\xFF\xFF\xFF\xFF\x87\x16`\0\x81\x81R`\x9A` R`@\x80\x82 \x80T`\xFF\x19\x16`\x01\x17\x90UQ3\x92\x91\x7F\xC2\r\x1B\xB0\xF1b6\x800k\x83\xD4\xFFK\xB9\x9A+\xEB\x9D\x86\xD9x2\xF3\xCA@\xFD\x13\xA2\x9D\xF1\xEC\x91\xA3PPPPPPP[PPPPV[a\x16\xD1`@Q\x80`\x80\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`\0\x87`\x01`\x01`\xA0\x1B\x03\x16ch0H5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\x11W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x175\x91\x90aG[V[\x90Pa\x17b`@Q\x80`\x80\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`@Qc\x85\x02\rI`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90c\x85\x02\rI\x90a\x17\x92\x90\x8B\x90\x89\x90\x89\x90`\x04\x01aK\x8FV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\xAFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x17\xD7\x91\x90\x81\x01\x90aK\xD9V[\x81R`@Qc\xE1\x92\xE9\xAD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xE1\x92\xE9\xAD\x90a\x18\t\x90\x8B\x90\x8B\x90\x8B\x90`\x04\x01aLgV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18&W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x18N\x91\x90\x81\x01\x90aK\xD9V[`@\x82\x01R\x85`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18kWa\x18ka;\xFCV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x18\x9EW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x18\x89W\x90P[P``\x82\x01R`\0[`\xFF\x81\x16\x87\x11\x15a\x1C;W`\0\x85`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18\xCCWa\x18\xCCa;\xFCV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x18\xF5W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x83``\x01Q\x83`\xFF\x16\x81Q\x81\x10a\x19\x0FWa\x19\x0FaH,V[` \x02` \x01\x01\x81\x90RP`\0[\x86\x81\x10\x15a\x1B;W`\0\x8C`\x01`\x01`\xA0\x1B\x03\x16c0db\r\x8A\x8A\x85\x81\x81\x10a\x19HWa\x19HaH,V[\x90P` \x02\x015\x8E\x88`\0\x01Q\x86\x81Q\x81\x10a\x19fWa\x19faH,V[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x19\xA3\x93\x92\x91\x90\x92\x83Rc\xFF\xFF\xFF\xFF\x91\x82\x16` \x84\x01R\x16`@\x82\x01R``\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\xC0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\xE4\x91\x90aL\x90V[\x90P\x8A\x8A\x85`\xFF\x16\x81\x81\x10a\x19\xFBWa\x19\xFBaH,V[`\x01`\x01`\xC0\x1B\x03\x84\x16\x92\x015`\xF8\x1C\x91\x90\x91\x1C`\x01\x90\x81\x16\x14\x15\x90Pa\x1B(W\x85`\x01`\x01`\xA0\x1B\x03\x16cH\x08Xf\x8A\x8A\x85\x81\x81\x10a\x1A=Wa\x1A=aH,V[\x90P` \x02\x015\x8D\x8D\x88`\xFF\x16\x81\x81\x10a\x1AYWa\x1AYaH,V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x87\x90\x1B\x16\x81R`\x04\x81\x01\x94\x90\x94R\x91\x90\x91\x015`\xF8\x1C`$\x83\x01RPc\xFF\xFF\xFF\xFF\x8F\x16`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\xAFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xD3\x91\x90aL\xB9V[\x85``\x01Q\x85`\xFF\x16\x81Q\x81\x10a\x1A\xECWa\x1A\xECaH,V[` \x02` \x01\x01Q\x84\x81Q\x81\x10a\x1B\x05Wa\x1B\x05aH,V[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x82a\x1B$\x81aJ,V[\x93PP[P\x80a\x1B3\x81aJ,V[\x91PPa\x19\x1DV[P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1BVWa\x1BVa;\xFCV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1B\x7FW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a\x1C\0W\x84``\x01Q\x84`\xFF\x16\x81Q\x81\x10a\x1B\xA6Wa\x1B\xA6aH,V[` \x02` \x01\x01Q\x81\x81Q\x81\x10a\x1B\xBFWa\x1B\xBFaH,V[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a\x1B\xD9Wa\x1B\xD9aH,V[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x1B\xF8\x81aJ,V[\x91PPa\x1B\x85V[P\x80\x84``\x01Q\x84`\xFF\x16\x81Q\x81\x10a\x1C\x1BWa\x1C\x1BaH,V[` \x02` \x01\x01\x81\x90RPPP\x80\x80a\x1C3\x90aL\xD6V[\x91PPa\x18\xA7V[P`\0\x89`\x01`\x01`\xA0\x1B\x03\x16c5a\xDE\xB1`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C|W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xA0\x91\x90aG[V[`@Qc\xED\xA1\x07c`\xE0\x1B\x81R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xED\xA1\x07c\x90a\x1C\xD3\x90\x8B\x90\x8B\x90\x8E\x90`\x04\x01aL\xF6V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\xF0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1D\x18\x91\x90\x81\x01\x90aK\xD9V[` \x83\x01RP\x98\x97PPPPPPPPV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1DrW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\x96\x91\x90aG\xC2V[a\x1D\xB2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x14\x90aG\xE4V[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[`\x9CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1EUW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FTask generator must be the calle`D\x82\x01R`9`\xF9\x1B`d\x82\x01R`\x84\x01a\x06\x14V[a\x1E\x8C`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0c\xFF\xFF\xFF\xFF\x16\x81R` \x01``\x81R` \x01`\0c\xFF\xFF\xFF\xFF\x16\x81RP\x90V[\x84\x81Rc\xFF\xFF\xFF\xFFC\x81\x16` \x80\x84\x01\x91\x90\x91R\x90\x85\x16``\x83\x01R`@\x80Q`\x1F\x85\x01\x83\x90\x04\x83\x02\x81\x01\x83\x01\x90\x91R\x83\x81R\x90\x84\x90\x84\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPPP`@\x80\x83\x01\x91\x90\x91RQa\x1E\xF2\x90\x82\x90` \x01aMLV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 `\x97\x80Tc\xFF\xFF\xFF\xFF\x90\x81\x16`\0\x90\x81R`\x98\x90\x94R\x93\x90\x92 UT\x16\x90\x7F\x16\x95\xB8\xD0n\xC8\0\xB4a^t\\\xFB[\xD0\x0C\x1F(ua]B\x92\\;Z\xFAT;\xB2LH\x90a\x1FU\x90\x84\x90aMLV[`@Q\x80\x91\x03\x90\xA2`\x97Ta\x1Fq\x90c\xFF\xFF\xFF\xFF\x16`\x01aI\x8AV[`\x97\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01\x81\x90R\x90\x81[\x86\x81\x10\x15a!\xAFW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xC1\xAFk$\x89\x89\x84\x81\x81\x10a \x08Wa \x08aH,V[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x88\x88`\xA0\x01Q\x85\x81Q\x81\x10a ,Wa ,aH,V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81R`\xFF\x90\x93\x16`\x04\x84\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x84\x01R\x16`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a \x88W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \xAC\x91\x90aM\xB1V[`\x01`\x01`@\x1B\x03\x19\x16a \xCF\x86`@\x01Q\x83\x81Q\x81\x10a\x13\xACWa\x13\xACaH,V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14a!kW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`a`$\x82\x01R`\0\x80Q` aO\x13\x839\x81Q\x91R`D\x82\x01R\x7Fres: quorumApk hash in storage d`d\x82\x01R\x7Foes not match provided quorum ap`\x84\x82\x01R`k`\xF8\x1B`\xA4\x82\x01R`\xC4\x01a\x06\x14V[a!\x9B\x85`@\x01Q\x82\x81Q\x81\x10a!\x84Wa!\x84aH,V[` \x02` \x01\x01Q\x83a0U\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91P\x80a!\xA7\x81aJ,V[\x91PPa\x1F\xBFV[P`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R\x86`\x01`\x01`@\x1B\x03\x81\x11\x15a!\xDCWa!\xDCa;\xFCV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\"\x05W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P` \x82\x01R\x86`\x01`\x01`@\x1B\x03\x81\x11\x15a\"#Wa\"#a;\xFCV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\"LW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x81R` \x85\x01QQ`\0\x90`\x01`\x01`@\x1B\x03\x81\x11\x15a\"oWa\"oa;\xFCV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\"\x98W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x86` \x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\xBAWa\"\xBAa;\xFCV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\"\xE3W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0a#'\x8B\x8B\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa4\xA0\x92PPPV[\x90P`\0[\x88` \x01QQ\x81\x10\x15a%\x92Wa#R\x89` \x01Q\x82\x81Q\x81\x10a\x13\xACWa\x13\xACaH,V[\x84\x82\x81Q\x81\x10a#dWa#daH,V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80\x15a$\x1CW\x83a#\x81`\x01\x83aM\xDCV[\x81Q\x81\x10a#\x91Wa#\x91aH,V[` \x02` \x01\x01Q`\0\x1C\x84\x82\x81Q\x81\x10a#\xAEWa#\xAEaH,V[` \x02` \x01\x01Q`\0\x1C\x11a$\x1CW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R`\0\x80Q` aO\x13\x839\x81Q\x91R`D\x82\x01R\x7Fres: nonSignerPubkeys not sorted`d\x82\x01R`\x84\x01a\x06\x14V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c0db\r\x85\x83\x81Q\x81\x10a$]Wa$]aH,V[` \x02` \x01\x01Q\x8C\x8C`\0\x01Q\x85\x81Q\x81\x10a$|Wa$|aH,V[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a$\xB9\x93\x92\x91\x90\x92\x83Rc\xFF\xFF\xFF\xFF\x91\x82\x16` \x84\x01R\x16`@\x82\x01R``\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a$\xD6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$\xFA\x91\x90aL\x90V[`\x01`\x01`\xC0\x1B\x03\x16\x83\x82\x81Q\x81\x10a%\x15Wa%\x15aH,V[` \x02` \x01\x01\x81\x81RPPa%~a%wa%K\x84\x86\x85\x81Q\x81\x10a%=Wa%=aH,V[` \x02` \x01\x01Q\x16a6\tV[a%q\x8C` \x01Q\x85\x81Q\x81\x10a%dWa%daH,V[` \x02` \x01\x01Qa6:V[\x90a6\xD5V[\x87\x90a0UV[\x95P\x80a%\x8A\x81aJ,V[\x91PPa#,V[PP`\0[`\xFF\x81\x16\x8A\x11\x15a(\xD2W`\0\x8B\x8B\x83`\xFF\x16\x81\x81\x10a%\xB9Wa%\xB9aH,V[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xC8)LV\x82\x8C\x8C`\xC0\x01Q\x86`\xFF\x16\x81Q\x81\x10a&\x12Wa&\x12aH,V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81R`\xFF\x90\x93\x16`\x04\x84\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x84\x01R\x16`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&nW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\x92\x91\x90aK\x03V[\x85` \x01Q\x83`\xFF\x16\x81Q\x81\x10a&\xABWa&\xABaH,V[`\x01`\x01``\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x82\x01R\x85\x01Q\x80Q`\xFF\x84\x16\x90\x81\x10a&\xDAWa&\xDAaH,V[` \x02` \x01\x01Q\x85`\0\x01Q\x83`\xFF\x16\x81Q\x81\x10a&\xFBWa&\xFBaH,V[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPP`\0[\x89` \x01QQ\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a(\xC8W`\0a'd\x85\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a'NWa'NaH,V[` \x02` \x01\x01Q\x84`\xFF\x16\x1C`\x01\x90\x81\x16\x14\x90V[\x15a(\xB5W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xA4<\xDE\x89\x84\x8E\x89\x86c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a'\xB2Wa'\xB2aH,V[` \x02` \x01\x01Q\x8F`\xE0\x01Q\x89`\xFF\x16\x81Q\x81\x10a'\xD3Wa'\xD3aH,V[` \x02` \x01\x01Q\x86c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a'\xF2Wa'\xF2aH,V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x87\x90\x1B\x16\x81R`\xFF\x90\x94\x16`\x04\x85\x01Rc\xFF\xFF\xFF\xFF\x92\x83\x16`$\x85\x01R`D\x84\x01\x91\x90\x91R\x16`d\x82\x01R`\x84\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a(VW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(z\x91\x90aK\x03V[\x87Q\x80Q`\xFF\x87\x16\x90\x81\x10a(\x91Wa(\x91aH,V[` \x02` \x01\x01\x81\x81Qa(\xA5\x91\x90aM\xF3V[`\x01`\x01``\x1B\x03\x16\x90RP`\x01\x01[P\x80a(\xC0\x81aN\x1BV[\x91PPa'\x1EV[PP`\x01\x01a%\x97V[PP`\0\x80a(\xEB\x8C\x86\x8A``\x01Q\x8B`\x80\x01Qa\x07hV[\x91P\x91P\x81a)\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R`\0\x80Q` aO\x13\x839\x81Q\x91R`D\x82\x01R\x7Fres: pairing precompile call fai`d\x82\x01Rb\x1B\x19Y`\xEA\x1B`\x84\x82\x01R`\xA4\x01a\x06\x14V[\x80a)\xBDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R`\0\x80Q` aO\x13\x839\x81Q\x91R`D\x82\x01R\x7Fres: signature is invalid\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x14V[PP`\0\x87\x82`@Q` \x01a)\xD4\x92\x91\x90aKGV[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x92\x9B\x92\x9AP\x91\x98PPPPPPPPPV[a*\x06a7\xBAV[a*\x10`\0a8\x14V[V[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91``\x91\x83\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x84\x81`\0\x81Q\x81\x10a*MWa*MaH,V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Qc\x85\x02\rI`\xE0\x1B\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90c\x85\x02\rI\x90a*\x89\x90\x88\x90\x86\x90`\x04\x01aN?V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a*\xA6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra*\xCE\x91\x90\x81\x01\x90aK\xD9V[`\0\x81Q\x81\x10a*\xE0Wa*\xE0aH,V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Qc0db\r`\xE0\x1B\x81R`\x04\x81\x01\x88\x90Rc\xFF\xFF\xFF\xFF\x87\x81\x16`$\x83\x01R\x90\x91\x16`D\x82\x01\x81\x90R\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x89\x16\x90c0db\r\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+LW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+p\x91\x90aL\x90V[`\x01`\x01`\xC0\x1B\x03\x16\x90P`\0a+\x86\x82a8fV[\x90P\x81a+\x94\x8A\x83\x8Aa\rqV[\x95P\x95PPPPP\x93P\x93\x91PPV[a+\xACa7\xBAV[`\x01`\x01`\xA0\x1B\x03\x81\x16a,\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x06\x14V[a\x06&\x81a8\x14V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a,:WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a,TWP0;\x15\x80\x15a,TWP`\0T`\xFF\x16`\x01\x14[a,\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x06\x14V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a,\xDAW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a,\xE5\x85`\0a8\xC3V[a,\xEE\x84a8\x14V[`\x9B\x80T`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U`\x9C\x80T\x92\x85\x16\x92\x90\x91\x16\x91\x90\x91\x17\x90U\x80\x15a-dW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-\xBEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-\xE2\x91\x90aG[V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a.\x12W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x14\x90aGxV[`fT\x19\x81\x19`fT\x19\x16\x14a.\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x14V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x07]V[`\x01`\x01`\xA0\x1B\x03\x81\x16a/UW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x06\x14V[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra/\xDAa:\xD7V[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x80\x82\x01\x84\x90R`\0\x90\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a0\rWa0\x0FV[\xFE[P\x80a0MW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCB[][\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x06\x14V[PP\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra0qa:\xF5V[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R\x83Q`@\x80\x84\x01\x91\x90\x91R\x90\x84\x01Q``\x83\x01R`\0\x90\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a0\rWP\x80a0MW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCBXY\x19\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x06\x14V[a0\xF1a;\x13V[P`@\x80Q`\x80\x81\x01\x82R\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81\x83\x01\x90\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED``\x83\x01R\x81R\x81Q\x80\x83\x01\x90\x92R\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x81\x01\x91\x90\x91R\x81\x01\x91\x90\x91R\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0\x80\x80a1\xD9`\0\x80Q` aN\xF3\x839\x81Q\x91R\x86aHBV[\x90P[a1\xE5\x81a9\xADV[\x90\x93P\x91P`\0\x80Q` aN\xF3\x839\x81Q\x91R\x82\x83\t\x83\x14\x15a2\x1FW`@\x80Q\x80\x82\x01\x90\x91R\x90\x81R` \x81\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80Q` aN\xF3\x839\x81Q\x91R`\x01\x82\x08\x90Pa1\xDCV[`@\x80Q\x80\x82\x01\x82R\x86\x81R` \x80\x82\x01\x86\x90R\x82Q\x80\x84\x01\x90\x93R\x86\x83R\x82\x01\x84\x90R`\0\x91\x82\x91\x90a2ka;8V[`\0[`\x02\x81\x10\x15a40W`\0a2\x84\x82`\x06aJ\rV[\x90P\x84\x82`\x02\x81\x10a2\x98Wa2\x98aH,V[` \x02\x01QQ\x83a2\xAA\x83`\0aN\x93V[`\x0C\x81\x10a2\xBAWa2\xBAaH,V[` \x02\x01R\x84\x82`\x02\x81\x10a2\xD1Wa2\xD1aH,V[` \x02\x01Q` \x01Q\x83\x82`\x01a2\xE8\x91\x90aN\x93V[`\x0C\x81\x10a2\xF8Wa2\xF8aH,V[` \x02\x01R\x83\x82`\x02\x81\x10a3\x0FWa3\x0FaH,V[` \x02\x01QQQ\x83a3\"\x83`\x02aN\x93V[`\x0C\x81\x10a32Wa32aH,V[` \x02\x01R\x83\x82`\x02\x81\x10a3IWa3IaH,V[` \x02\x01QQ`\x01` \x02\x01Q\x83a3b\x83`\x03aN\x93V[`\x0C\x81\x10a3rWa3raH,V[` \x02\x01R\x83\x82`\x02\x81\x10a3\x89Wa3\x89aH,V[` \x02\x01Q` \x01Q`\0`\x02\x81\x10a3\xA4Wa3\xA4aH,V[` \x02\x01Q\x83a3\xB5\x83`\x04aN\x93V[`\x0C\x81\x10a3\xC5Wa3\xC5aH,V[` \x02\x01R\x83\x82`\x02\x81\x10a3\xDCWa3\xDCaH,V[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a3\xF7Wa3\xF7aH,V[` \x02\x01Q\x83a4\x08\x83`\x05aN\x93V[`\x0C\x81\x10a4\x18Wa4\x18aH,V[` \x02\x01RP\x80a4(\x81aJ,V[\x91PPa2nV[Pa49a;WV[`\0` \x82a\x01\x80\x85`\x08\x8C\xFA\x91Q\x91\x9C\x91\x15\x15\x9BP\x90\x99PPPPPPPPPPV[`\0\x81`\0\x01Q\x82` \x01Q`@Q` \x01a4\x83\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0a\x01\0\x82Q\x11\x15a5\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FBitmapUtils.bytesArrayToBitmap: `D\x82\x01RubytesArray is too long`P\x1B`d\x82\x01R`\x84\x01a\x06\x14V[\x81Qa5\"WP`\0\x91\x90PV[`\0\x80\x83`\0\x81Q\x81\x10a58Wa58aH,V[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x81\x90\x1B\x92P[\x84Q\x81\x10\x15a6\0W\x84\x81\x81Q\x81\x10a5fWa5faH,V[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x1B\x91P\x82\x82\x16\x15a5\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FBitmapUtils.bytesArrayToBitmap: `D\x82\x01R\x7Frepeat entry in bytesArray\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x14V[\x91\x81\x17\x91a5\xF9\x81aJ,V[\x90Pa5KV[P\x90\x93\x92PPPV[`\0\x80[\x82\x15a64Wa6\x1E`\x01\x84aM\xDCV[\x90\x92\x16\x91\x80a6,\x81aN\xABV[\x91PPa6\rV[\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q\x15\x80\x15a6_WP` \x82\x01Q\x15[\x15a6}WPP`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01`\0\x80Q` aN\xF3\x839\x81Q\x91R\x84` \x01Qa6\xB0\x91\x90aHBV[a6\xC8\x90`\0\x80Q` aN\xF3\x839\x81Q\x91RaM\xDCV[\x90R\x92\x91PPV[\x91\x90PV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x02\0\x82a\xFF\xFF\x16\x10a71W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Roscalar-too-large`\x80\x1B`D\x82\x01R`d\x01a\x06\x14V[\x81a\xFF\xFF\x16`\x01\x14\x15a7EWP\x81a64V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01\x81\x90R\x84\x90`\x01\x90[\x81a\xFF\xFF\x16\x86a\xFF\xFF\x16\x11\x15a7\xAFW`\x01a\xFF\xFF\x87\x16`\xFF\x83\x16\x1C\x81\x16\x14\x15a7\x92Wa7\x8F\x84\x84a0UV[\x93P[a7\x9C\x83\x84a0UV[\x92Pb\x01\xFF\xFE`\x01\x92\x83\x1B\x16\x91\x01a7aV[P\x91\x95\x94PPPPPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a*\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x06\x14V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[```\0\x80[a\x01\0\x81\x10\x15a8\xBCW`\x01\x81\x1B\x91P\x83\x82\x16\x15a8\xACW\x82\x81`\xF8\x1B`@Q` \x01a8\x9A\x92\x91\x90aN\xC3V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92P[a8\xB5\x81aJ,V[\x90Pa8lV[PP\x91\x90PV[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a8\xE4WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a9fW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x06\x14V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a9\xA9\x82a.\xC7V[PPV[`\0\x80\x80`\0\x80Q` aN\xF3\x839\x81Q\x91R`\x03`\0\x80Q` aN\xF3\x839\x81Q\x91R\x86`\0\x80Q` aN\xF3\x839\x81Q\x91R\x88\x89\t\t\x08\x90P`\0a:#\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\0\x80Q` aN\xF3\x839\x81Q\x91Ra:/V[\x91\x95\x91\x94P\x90\x92PPPV[`\0\x80a::a;WV[a:Ba;uV[` \x80\x82R\x81\x81\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x88\x90R`\x80\x82\x01\x87\x90R`\xA0\x82\x01\x86\x90R\x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82\x80\x15a0\rWP\x82a:\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FBN254.expMod: call failure\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\x14V[PQ\x95\x94PPPPPV[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80a;&a;\x93V[\x81R` \x01a;3a;\x93V[\x90R\x90V[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06&W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a;\xD8W`\0\x80\xFD[\x815a\x10\xE5\x81a;\xB1V[`\0` \x82\x84\x03\x12\x15a;\xF5W`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a<4Wa<4a;\xFCV[`@R\x90V[`@Qa\x01\0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a<4Wa<4a;\xFCV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a<\x85Wa<\x85a;\xFCV[`@R\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a<\x9FW`\0\x80\xFD[a<\xA7a<\x12V[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a<\xCEW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a<\xF0Wa<\xF0a;\xFCV[\x80`@RP\x80`@\x84\x01\x85\x81\x11\x15a=\x07W`\0\x80\xFD[\x84[\x81\x81\x10\x15a7\xAFW\x805\x83R` \x92\x83\x01\x92\x01a=\tV[`\0`\x80\x82\x84\x03\x12\x15a=3W`\0\x80\xFD[a=;a<\x12V[\x90Pa=G\x83\x83a<\xBDV[\x81Ra=V\x83`@\x84\x01a<\xBDV[` \x82\x01R\x92\x91PPV[`\0\x80`\0\x80a\x01 \x85\x87\x03\x12\x15a=xW`\0\x80\xFD[\x845\x93Pa=\x89\x86` \x87\x01a<\x8DV[\x92Pa=\x98\x86``\x87\x01a=!V[\x91Pa=\xA7\x86`\xE0\x87\x01a<\x8DV[\x90P\x92\x95\x91\x94P\x92PV[`\0`\x80\x82\x84\x03\x12\x15a=\xC4W`\0\x80\xFD[P\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a=\xC4W`\0\x80\xFD[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a=\xF5Wa=\xF5a;\xFCV[P`\x05\x1B` \x01\x90V[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x06&W`\0\x80\xFD[\x805a6\xD0\x81a=\xFFV[`\0\x82`\x1F\x83\x01\x12a>-W`\0\x80\xFD[\x815` a>Ba>=\x83a=\xDCV[a<]V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a>aW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a>\x85W\x805a>x\x81a=\xFFV[\x83R\x91\x83\x01\x91\x83\x01a>eV[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a>\xA1W`\0\x80\xFD[\x815` a>\xB1a>=\x83a=\xDCV[\x82\x81R`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a>\xD0W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a>\x85Wa>\xE6\x88\x82a<\x8DV[\x83R\x91\x83\x01\x91`@\x01a>\xD4V[`\0\x82`\x1F\x83\x01\x12a?\x05W`\0\x80\xFD[\x815` a?\x15a>=\x83a=\xDCV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a?4W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a>\x85W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a?WW`\0\x80\x81\xFD[a?e\x89\x86\x83\x8B\x01\x01a>\x1CV[\x84RP\x91\x83\x01\x91\x83\x01a?8V[`\0a\x01\x80\x82\x84\x03\x12\x15a?\x86W`\0\x80\xFD[a?\x8Ea<:V[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a?\xA7W`\0\x80\xFD[a?\xB3\x85\x83\x86\x01a>\x1CV[\x83R` \x84\x015\x91P\x80\x82\x11\x15a?\xC9W`\0\x80\xFD[a?\xD5\x85\x83\x86\x01a>\x90V[` \x84\x01R`@\x84\x015\x91P\x80\x82\x11\x15a?\xEEW`\0\x80\xFD[a?\xFA\x85\x83\x86\x01a>\x90V[`@\x84\x01Ra@\x0C\x85``\x86\x01a=!V[``\x84\x01Ra@\x1E\x85`\xE0\x86\x01a<\x8DV[`\x80\x84\x01Ra\x01 \x84\x015\x91P\x80\x82\x11\x15a@8W`\0\x80\xFD[a@D\x85\x83\x86\x01a>\x1CV[`\xA0\x84\x01Ra\x01@\x84\x015\x91P\x80\x82\x11\x15a@^W`\0\x80\xFD[a@j\x85\x83\x86\x01a>\x1CV[`\xC0\x84\x01Ra\x01`\x84\x015\x91P\x80\x82\x11\x15a@\x84W`\0\x80\xFD[Pa@\x91\x84\x82\x85\x01a>\xF4V[`\xE0\x83\x01RP\x92\x91PPV[`\0\x80`\0`\x80\x84\x86\x03\x12\x15a@\xB2W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a@\xC9W`\0\x80\xFD[a@\xD5\x87\x83\x88\x01a=\xB2V[\x94Pa@\xE4\x87` \x88\x01a=\xCAV[\x93P``\x86\x015\x91P\x80\x82\x11\x15a@\xFAW`\0\x80\xFD[PaA\x07\x86\x82\x87\x01a?sV[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15aA#W`\0\x80\xFD[\x815a\x10\xE5\x81a=\xFFV[`\0\x80`\0``\x84\x86\x03\x12\x15aACW`\0\x80\xFD[\x835aAN\x81a;\xB1V[\x92P` \x84\x81\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aAkW`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12aA\x7FW`\0\x80\xFD[\x815\x81\x81\x11\x15aA\x91WaA\x91a;\xFCV[aA\xA3`\x1F\x82\x01`\x1F\x19\x16\x85\x01a<]V[\x91P\x80\x82R\x88\x84\x82\x85\x01\x01\x11\x15aA\xB9W`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x94PPPPaA\xDC`@\x85\x01a>\x11V[\x90P\x92P\x92P\x92V[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0\x80[\x86\x81\x10\x15aBgW\x83\x85\x03\x8AR\x82Q\x80Q\x80\x87R\x90\x87\x01\x90\x87\x87\x01\x90\x84[\x81\x81\x10\x15aBRW\x83Q\x80Q\x84R\x8A\x01Q`\x01`\x01``\x1B\x03\x16\x8A\x84\x01R\x92\x89\x01\x92`@\x90\x92\x01\x91`\x01\x01aB\"V[PP\x9A\x87\x01\x9A\x95PP\x91\x85\x01\x91`\x01\x01aB\x04V[P\x92\x98\x97PPPPPPPPV[` \x81R`\0a\x10\xE5` \x83\x01\x84aA\xE5V[`\0\x80`\0\x80`\xC0\x85\x87\x03\x12\x15aB\x9EW`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aB\xB5W`\0\x80\xFD[aB\xC1\x88\x83\x89\x01a=\xB2V[\x95PaB\xD0\x88` \x89\x01a=\xCAV[\x94PaB\xDF\x88``\x89\x01a=\xCAV[\x93P`\xA0\x87\x015\x91P\x80\x82\x11\x15aB\xF5W`\0\x80\xFD[PaC\x02\x87\x82\x88\x01a>\x90V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80\x83`\x1F\x84\x01\x12aC W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aC7W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aCOW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\x80\x87\x89\x03\x12\x15aCoW`\0\x80\xFD[\x865aCz\x81a;\xB1V[\x95P` \x87\x015aC\x8A\x81a=\xFFV[\x94P`@\x87\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aC\xA6W`\0\x80\xFD[aC\xB2\x8A\x83\x8B\x01aC\x0EV[\x90\x96P\x94P``\x89\x015\x91P\x80\x82\x11\x15aC\xCBW`\0\x80\xFD[\x81\x89\x01\x91P\x89`\x1F\x83\x01\x12aC\xDFW`\0\x80\xFD[\x815\x81\x81\x11\x15aC\xEEW`\0\x80\xFD[\x8A` \x82`\x05\x1B\x85\x01\x01\x11\x15aD\x03W`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92\x95P\x92\x95P\x92\x95V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aDOW\x81Qc\xFF\xFF\xFF\xFF\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aD-V[P\x94\x95\x94PPPPPV[`\0` \x80\x83R\x83Q`\x80\x82\x85\x01RaDv`\xA0\x85\x01\x82aD\x19V[\x90P\x81\x85\x01Q`\x1F\x19\x80\x86\x84\x03\x01`@\x87\x01RaD\x93\x83\x83aD\x19V[\x92P`@\x87\x01Q\x91P\x80\x86\x84\x03\x01``\x87\x01RaD\xB0\x83\x83aD\x19V[``\x88\x01Q\x87\x82\x03\x83\x01`\x80\x89\x01R\x80Q\x80\x83R\x91\x94P\x85\x01\x92P\x84\x84\x01\x90`\x05\x81\x90\x1B\x85\x01\x86\x01`\0[\x82\x81\x10\x15aE\x07W\x84\x87\x83\x03\x01\x84RaD\xF5\x82\x87QaD\x19V[\x95\x88\x01\x95\x93\x88\x01\x93\x91P`\x01\x01aD\xDBV[P\x99\x98PPPPPPPPPV[`\0` \x82\x84\x03\x12\x15aE'W`\0\x80\xFD[\x815`\xFF\x81\x16\x81\x14a\x10\xE5W`\0\x80\xFD[`\0\x80`\0\x80``\x85\x87\x03\x12\x15aENW`\0\x80\xFD[\x845\x93P` \x85\x015aE`\x81a=\xFFV[\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aE{W`\0\x80\xFD[aE\x87\x87\x82\x88\x01aC\x0EV[\x95\x98\x94\x97P\x95PPPPV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15aE\xABW`\0\x80\xFD[\x855\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aE\xC9W`\0\x80\xFD[aE\xD5\x89\x83\x8A\x01aC\x0EV[\x90\x96P\x94P`@\x88\x015\x91PaE\xEA\x82a=\xFFV[\x90\x92P``\x87\x015\x90\x80\x82\x11\x15aF\0W`\0\x80\xFD[PaF\r\x88\x82\x89\x01a?sV[\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aDOW\x81Q`\x01`\x01``\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aF.V[`@\x81R`\0\x83Q`@\x80\x84\x01RaFn`\x80\x84\x01\x82aF\x1AV[\x90P` \x85\x01Q`?\x19\x84\x83\x03\x01``\x85\x01RaF\x8B\x82\x82aF\x1AV[\x92PPP\x82` \x83\x01R\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15aF\xB1W`\0\x80\xFD[\x835aF\xBC\x81a;\xB1V[\x92P` \x84\x015\x91P`@\x84\x015aF\xD3\x81a=\xFFV[\x80\x91PP\x92P\x92P\x92V[\x82\x81R`@` \x82\x01R`\0aF\xF7`@\x83\x01\x84aA\xE5V[\x94\x93PPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aG\x15W`\0\x80\xFD[\x845aG \x81a;\xB1V[\x93P` \x85\x015aG0\x81a;\xB1V[\x92P`@\x85\x015aG@\x81a;\xB1V[\x91P``\x85\x015aGP\x81a;\xB1V[\x93\x96\x92\x95P\x90\x93PPV[`\0` \x82\x84\x03\x12\x15aGmW`\0\x80\xFD[\x81Qa\x10\xE5\x81a;\xB1V[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15aG\xD4W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x10\xE5W`\0\x80\xFD[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x82aH_WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aH{W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aH\x95W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15aCOW`\0\x80\xFD[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x81R\x815` \x82\x01R`\0` \x83\x015aH\xEE\x81a=\xFFV[c\xFF\xFF\xFF\xFF\x81\x16`@\x84\x01RP`@\x83\x015`\x1E\x19\x846\x03\x01\x81\x12aI\x12W`\0\x80\xFD[\x83\x01\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aI*W`\0\x80\xFD[\x806\x03\x85\x13\x15aI9W`\0\x80\xFD[`\x80``\x85\x01RaIQ`\xA0\x85\x01\x82` \x85\x01aH\xAAV[\x91PPaI```\x85\x01a>\x11V[c\xFF\xFF\xFF\xFF\x81\x16`\x80\x85\x01RP\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aI\xA9WaI\xA9aItV[\x01\x94\x93PPPPV[\x805aI\xBD\x81a=\xFFV[c\xFF\xFF\xFF\xFF\x16\x82R` \x90\x81\x015\x91\x01RV[`@\x81\x01a64\x82\x84aI\xB2V[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aJ\x04WaJ\x04aItV[\x02\x94\x93PPPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15aJ'WaJ'aItV[P\x02\x90V[`\0`\0\x19\x82\x14\x15aJ@WaJ@aItV[P`\x01\x01\x90V[`\x80\x81\x01aJU\x82\x85aI\xB2V[c\xFF\xFF\xFF\xFF\x83Q\x16`@\x83\x01R` \x83\x01Q``\x83\x01R\x93\x92PPPV[`\0` \x80\x83\x85\x03\x12\x15aJ\x86W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aJ\x9CW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aJ\xADW`\0\x80\xFD[\x80QaJ\xBBa>=\x82a=\xDCV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15aJ\xDAW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aJ\xF8W\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90aJ\xDFV[\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15aK\x15W`\0\x80\xFD[\x81Q`\x01`\x01``\x1B\x03\x81\x16\x81\x14a\x10\xE5W`\0\x80\xFD[`\x80\x81\x01aK:\x82\x85aI\xB2V[a\x10\xE5`@\x83\x01\x84aI\xB2V[c\xFF\xFF\xFF\xFF`\xE0\x1B\x83`\xE0\x1B\x16\x81R`\0`\x04\x82\x01\x83Q` \x80\x86\x01`\0[\x83\x81\x10\x15aK\x82W\x81Q\x85R\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01aKfV[P\x92\x97\x96PPPPPPPV[c\xFF\xFF\xFF\xFF\x84\x16\x81R`@` \x82\x01\x81\x90R\x81\x01\x82\x90R`\0`\x01`\x01`\xFB\x1B\x03\x83\x11\x15aK\xBCW`\0\x80\xFD[\x82`\x05\x1B\x80\x85``\x85\x017`\0\x92\x01``\x01\x91\x82RP\x93\x92PPPV[`\0` \x80\x83\x85\x03\x12\x15aK\xECW`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aL\x02W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aL\x13W`\0\x80\xFD[\x80QaL!a>=\x82a=\xDCV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15aL@W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aJ\xF8W\x83QaLX\x81a=\xFFV[\x82R\x92\x84\x01\x92\x90\x84\x01\x90aLEV[c\xFF\xFF\xFF\xFF\x84\x16\x81R`@` \x82\x01R`\0aL\x87`@\x83\x01\x84\x86aH\xAAV[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15aL\xA2W`\0\x80\xFD[\x81Q`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x14a\x10\xE5W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aL\xCBW`\0\x80\xFD[\x81Qa\x10\xE5\x81a=\xFFV[`\0`\xFF\x82\x16`\xFF\x81\x14\x15aL\xEDWaL\xEDaItV[`\x01\x01\x92\x91PPV[`@\x81R`\0aM\n`@\x83\x01\x85\x87aH\xAAV[\x90Pc\xFF\xFF\xFF\xFF\x83\x16` \x83\x01R\x94\x93PPPPV[`\0[\x83\x81\x10\x15aM;W\x81\x81\x01Q\x83\x82\x01R` \x01aM#V[\x83\x81\x11\x15a\x16\xA0WPP`\0\x91\x01RV[` \x81R\x81Q` \x82\x01R`\0` \x83\x01Qc\xFF\xFF\xFF\xFF\x80\x82\x16`@\x85\x01R`@\x85\x01Q\x91P`\x80``\x85\x01R\x81Q\x80`\xA0\x86\x01RaM\x92\x81`\xC0\x87\x01` \x86\x01aM V[``\x95\x90\x95\x01Q\x16`\x80\x84\x01RPP`\xC0`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15aM\xC3W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16\x81\x14a\x10\xE5W`\0\x80\xFD[`\0\x82\x82\x10\x15aM\xEEWaM\xEEaItV[P\x03\x90V[`\0`\x01`\x01``\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15aN\x13WaN\x13aItV[\x03\x93\x92PPPV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x14\x15aN5WaN5aItV[`\x01\x01\x93\x92PPPV[`\0`@\x82\x01c\xFF\xFF\xFF\xFF\x85\x16\x83R` `@\x81\x85\x01R\x81\x85Q\x80\x84R``\x86\x01\x91P\x82\x87\x01\x93P`\0[\x81\x81\x10\x15aN\x86W\x84Q\x83R\x93\x83\x01\x93\x91\x83\x01\x91`\x01\x01aNjV[P\x90\x97\x96PPPPPPPV[`\0\x82\x19\x82\x11\x15aN\xA6WaN\xA6aItV[P\x01\x90V[`\0a\xFF\xFF\x80\x83\x16\x81\x81\x14\x15aN5WaN5aItV[`\0\x83QaN\xD5\x81\x84` \x88\x01aM V[`\x01`\x01`\xF8\x1B\x03\x19\x93\x90\x93\x16\x91\x90\x92\x01\x90\x81R`\x01\x01\x92\x91PPV\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDGBLSSignatureChecker.checkSignatu\xA2dipfsX\"\x12 \xE2}fu\xA6Fi\x87[U\x96\x91\xCD\xCE\x0F\x04d\xB8\x8D'\0\x0E\xB2^\xCC\x06\xE7|\xFC\x01OAdsolcC\0\x08\x0C\x003";
    /// The bytecode of the contract.
    pub static MANGATATASKMANAGER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xFBW`\x005`\xE0\x1C\x80ch0H5\x11a\x01\x1AW\x80c\x8B\0\xCE|\x11a\0\xADW\x80c\xF2\xFD\xE3\x8B\x11a\0|W\x80c\xF2\xFD\xE3\x8B\x14a\x05\x06W\x80c\xF5\xC9\x89\x9D\x14a\x05\x19W\x80c\xF6<[\xAB\x14a\x05?W\x80c\xF8\xC8v^\x14a\x05GW\x80c\xFA\xBC\x1C\xBC\x14a\x05ZW`\0\x80\xFD[\x80c\x8B\0\xCE|\x14a\x04\xBDW\x80c\x8D\xA5\xCB[\x14a\x04\xCDW\x80c\xCE\xFD\xC1\xD4\x14a\x04\xDEW\x80c\xE3\xB5\xEE\x04\x14a\x04\xFFW`\0\x80\xFD[\x80cqP\x18\xA6\x11a\0\xE9W\x80cqP\x18\xA6\x14a\x04\x81W\x80cr\xD1\x8E\x8D\x14a\x04\x89W\x80cz\xFA\x1E\xED\x14a\x04\x97W\x80c\x88o\x11\x95\x14a\x04\xAAW`\0\x80\xFD[\x80ch0H5\x14a\x03\xFFW\x80ck\x92x~\x14a\x04&W\x80cm\x14\xA9\x87\x14a\x049W\x80cn\xFBF6\x14a\x04`W`\0\x80\xFD[\x80c5a\xDE\xB1\x11a\x01\x92W\x80cY\\jg\x11a\x01aW\x80cY\\jg\x14a\x03\x99W\x80cZ\xC8j\xB7\x14a\x03\xA1W\x80c\\\x97Z\xBB\x14a\x03\xD4W\x80c]\xEC\xC3\xF5\x14a\x03\xDCW`\0\x80\xFD[\x80c5a\xDE\xB1\x14a\x03\x1FW\x80c5c\xB0\xD1\x14a\x03FW\x80c<\xD2\x7Fd\x14a\x03fW\x80cOs\x9Ft\x14a\x03yW`\0\x80\xFD[\x80c$Z{\xFC\x11a\x01\xCEW\x80c$Z{\xFC\x14a\x02\x93W\x80c'|\xB9\x95\x14a\x02\xBEW\x80c,\xB2#\xD5\x14a\x02\xD1W\x80c-\x89\xF6\xFC\x14a\x02\xFFW`\0\x80\xFD[\x80c\x10\xD6z/\x14a\x02\0W\x80c\x13d9\xDD\x14a\x02\x15W\x80c\x17\x1F\x1D[\x14a\x02(W\x80c\x1A\xD41\x89\x14a\x02WW[`\0\x80\xFD[a\x02\x13a\x02\x0E6`\x04a;\xC6V[a\x05mV[\0[a\x02\x13a\x02#6`\x04a;\xE3V[a\x06)V[a\x02;a\x0266`\x04a=aV[a\x07hV[`@\x80Q\x92\x15\x15\x83R\x90\x15\x15` \x83\x01R\x01[`@Q\x80\x91\x03\x90\xF3[a\x02~\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02NV[`\x9BTa\x02\xA6\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02NV[a\x02\x13a\x02\xCC6`\x04a@\x9DV[a\x08\xF2V[a\x02\xF1a\x02\xDF6`\x04aA\x11V[`\x99` R`\0\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01a\x02NV[a\x02\xF1a\x03\r6`\x04aA\x11V[`\x98` R`\0\x90\x81R`@\x90 T\x81V[a\x02\xA6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03Ya\x03T6`\x04aA.V[a\rqV[`@Qa\x02N\x91\x90aBuV[a\x02\x13a\x03t6`\x04aB\x88V[a\x10\xECV[a\x03\x8Ca\x03\x876`\x04aCVV[a\x16\xA6V[`@Qa\x02N\x91\x90aDZV[a\x02\x13a\x1D*V[a\x03\xC4a\x03\xAF6`\x04aE\x15V[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`@Q\x90\x15\x15\x81R` \x01a\x02NV[`fTa\x02\xF1V[a\x03\xC4a\x03\xEA6`\x04aA\x11V[`\x9A` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x02\xA6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\x13a\x0446`\x04aE8V[a\x1D\xF1V[a\x02\xA6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x04sa\x04n6`\x04aE\x93V[a\x1F\x92V[`@Qa\x02N\x92\x91\x90aFSV[a\x02\x13a)\xFEV[`\x97Tc\xFF\xFF\xFF\xFF\x16a\x02~V[`\x9CTa\x02\xA6\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`eTa\x02\xA6\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x97Ta\x02~\x90c\xFF\xFF\xFF\xFF\x16\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x02\xA6V[a\x04\xF1a\x04\xEC6`\x04aF\x9CV[a*\x12V[`@Qa\x02N\x92\x91\x90aF\xDEV[`da\x02~V[a\x02\x13a\x05\x146`\x04a;\xC6V[a+\xA4V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x02~V[a\x02~`d\x81V[a\x02\x13a\x05U6`\x04aF\xFFV[a,\x1AV[a\x02\x13a\x05h6`\x04a;\xE3V[a-kV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xC0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xE4\x91\x90aG[V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x06\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x14\x90aGxV[`@Q\x80\x91\x03\x90\xFD[a\x06&\x81a.\xC7V[PV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06qW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x95\x91\x90aG\xC2V[a\x06\xB1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x14\x90aG\xE4V[`fT\x81\x81\x16\x14a\x07*W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x14V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`\0\x80`\0\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x87\x87`\0\x01Q\x88` \x01Q\x88`\0\x01Q`\0`\x02\x81\x10a\x07\xB0Wa\x07\xB0aH,V[` \x02\x01Q\x89Q`\x01` \x02\x01Q\x8A` \x01Q`\0`\x02\x81\x10a\x07\xD5Wa\x07\xD5aH,V[` \x02\x01Q\x8B` \x01Q`\x01`\x02\x81\x10a\x07\xF1Wa\x07\xF1aH,V[` \x90\x81\x02\x91\x90\x91\x01Q\x8CQ\x8D\x83\x01Q`@Qa\x08N\x9A\x99\x98\x97\x96\x95\x94\x01\x98\x89R` \x89\x01\x97\x90\x97R`@\x88\x01\x95\x90\x95R``\x87\x01\x93\x90\x93R`\x80\x86\x01\x91\x90\x91R`\xA0\x85\x01R`\xC0\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01Ra\x01 \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\0\x1Ca\x08q\x91\x90aHBV[\x90Pa\x08\xE4a\x08\x8Aa\x08\x83\x88\x84a/\xBEV[\x86\x90a0UV[a\x08\x92a0\xE9V[a\x08\xDAa\x08\xCB\x85a\x08\xC5`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x01\x82R`\x02\x90\x82\x01R\x90V[\x90a/\xBEV[a\x08\xD4\x8Ca1\xA9V[\x90a0UV[\x88b\x01\xD4\xC0a29V[\x90\x98\x90\x97P\x95PPPPPPV[`\x9BT`\x01`\x01`\xA0\x1B\x03\x163\x14a\tLW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAggregator must be the caller\0\0\0`D\x82\x01R`d\x01a\x06\x14V[`\0a\t^`@\x85\x01` \x86\x01aA\x11V[\x90P6`\0a\tp`@\x87\x01\x87aHdV[\x90\x92P\x90P`\0a\t\x87`\x80\x88\x01``\x89\x01aA\x11V[\x90P`\x98`\0a\t\x9A` \x89\x01\x89aA\x11V[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x87`@Q` \x01a\t\xC6\x91\x90aH\xD3V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x14a\nOW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7Fsupplied task does not match the`D\x82\x01R\x7F one recorded in the contract\0\0\0`d\x82\x01R`\x84\x01a\x06\x14V[`\0`\x99\x81a\na` \x8A\x01\x8AaA\x11V[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x14a\n\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FAggregator has already responded`D\x82\x01Rk to the task`\xA0\x1B`d\x82\x01R`\x84\x01a\x06\x14V[a\x0B\x08\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85aI\x8AV[c\xFF\xFF\xFF\xFF\x16Cc\xFF\xFF\xFF\xFF\x16\x11\x15a\x0ByW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FAggregator has responded to the `D\x82\x01Rltask too late`\x98\x1B`d\x82\x01R`\x84\x01a\x06\x14V[`\0\x86`@Q` \x01a\x0B\x8C\x91\x90aI\xD0V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0\x80a\x0B\xB4\x83\x87\x87\x8A\x8Ca\x1F\x92V[\x91P\x91P`\0[\x85\x81\x10\x15a\x0C\xB3W\x84`\xFF\x16\x83` \x01Q\x82\x81Q\x81\x10a\x0B\xDDWa\x0B\xDDaH,V[` \x02` \x01\x01Qa\x0B\xEF\x91\x90aI\xDEV[`\x01`\x01``\x1B\x03\x16`d\x84`\0\x01Q\x83\x81Q\x81\x10a\x0C\x10Wa\x0C\x10aH,V[` \x02` \x01\x01Q`\x01`\x01``\x1B\x03\x16a\x0C+\x91\x90aJ\rV[\x10\x15a\x0C\xA1W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FSignatories do not own at least `D\x82\x01R\x7Fthreshold percentage of a quorum`d\x82\x01R`\x84\x01a\x06\x14V[\x80a\x0C\xAB\x81aJ,V[\x91PPa\x0B\xBBV[P`@\x80Q\x80\x82\x01\x82Rc\xFF\xFF\xFF\xFFC\x16\x81R` \x80\x82\x01\x84\x90R\x91Q\x90\x91a\x0C\xE0\x91\x8C\x91\x84\x91\x01aJGV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\x99`\0\x8C`\0\x01` \x81\x01\x90a\r\r\x91\x90aA\x11V[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x81\x90UP\x7F\xF2\xAF\x11\xFA\xD7=CI\xC9\x9C\xF6/)\x8D3vA\xEA\x0B\xB7\xC0\xF5\xA8\xDB\x92\xA9\x8A'_sOX\x8A\x82`@Qa\r\\\x92\x91\x90aJGV[`@Q\x80\x91\x03\x90\xA1PPPPPPPPPPPV[```\0\x84`\x01`\x01`\xA0\x1B\x03\x16ch0H5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xB3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xD7\x91\x90aG[V[\x90P`\0\x85`\x01`\x01`\xA0\x1B\x03\x16c\x9E\x99#\xC2`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\x19W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E=\x91\x90aG[V[\x90P`\0\x85Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0EZWa\x0EZa;\xFCV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E\x8DW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0ExW\x90P[P\x90P`\0[\x86Q\x81\x10\x15a\x10\xDFW`\0\x87\x82\x81Q\x81\x10a\x0E\xB0Wa\x0E\xB0aH,V[\x01` \x01Q`@Qc\x88\x9A\xE3\xE5`\xE0\x1B\x81R`\xF8\x91\x90\x91\x1C`\x04\x82\x01\x81\x90Rc\xFF\xFF\xFF\xFF\x89\x16`$\x83\x01R\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x86\x16\x90c\x88\x9A\xE3\xE5\x90`D\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x11W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0F9\x91\x90\x81\x01\x90aJsV[\x90P\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0FTWa\x0FTa;\xFCV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0F\x99W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0FrW\x90P[P\x84\x84\x81Q\x81\x10a\x0F\xACWa\x0F\xACaH,V[` \x02` \x01\x01\x81\x90RP`\0[\x81Q\x81\x10\x15a\x10\xC9W`\0\x82\x82\x81Q\x81\x10a\x0F\xD7Wa\x0F\xD7aH,V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`@\x80Q\x80\x82\x01\x82R\x82\x81R\x90Qc\x1B2r%`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`\xFF\x88\x16`$\x82\x01Rc\xFF\xFF\xFF\xFF\x8E\x16`D\x82\x01R\x91\x93P\x91\x82\x01\x90`\x01`\x01`\xA0\x1B\x03\x8B\x16\x90c\x1B2r%\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10OW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10s\x91\x90aK\x03V[`\x01`\x01``\x1B\x03\x16\x81RP\x86\x86\x81Q\x81\x10a\x10\x91Wa\x10\x91aH,V[` \x02` \x01\x01Q\x83\x81Q\x81\x10a\x10\xAAWa\x10\xAAaH,V[` \x02` \x01\x01\x81\x90RPP\x80\x80a\x10\xC1\x90aJ,V[\x91PPa\x0F\xBAV[PPP\x80\x80a\x10\xD7\x90aJ,V[\x91PPa\x0E\x93V[P\x92PPP[\x93\x92PPPV[`\0a\x10\xFB` \x85\x01\x85aA\x11V[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x99` R`@\x90 T\x90\x91P\x855\x90a\x11mW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FTask hasn't been responded to ye`D\x82\x01R`\x1D`\xFA\x1B`d\x82\x01R`\x84\x01a\x06\x14V[\x84\x84`@Q` \x01a\x11\x80\x92\x91\x90aK,V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 c\xFF\xFF\xFF\xFF\x85\x16`\0\x90\x81R`\x99\x90\x93R\x91 T\x14a\x12\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FTask response does not match the`D\x82\x01R\x7F one recorded in the contract\0\0\0`d\x82\x01R`\x84\x01a\x06\x14V[c\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`\x9A` R`@\x90 T`\xFF\x16\x15a\x12\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FThe response to this task has al`D\x82\x01R\x7Fready been challenged successful`d\x82\x01Rb6<\x97`\xE9\x1B`\x84\x82\x01R`\xA4\x01a\x06\x14V[`da\x12\xC6` \x86\x01\x86aA\x11V[a\x12\xD0\x91\x90aI\x8AV[c\xFF\xFF\xFF\xFF\x16Cc\xFF\xFF\xFF\xFF\x16\x11\x15a\x13QW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FThe challenge period for this ta`D\x82\x01R\x7Fsk has already expired.\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x14V[`\x01`@Q3\x90c\xFF\xFF\xFF\xFF\x85\x16\x90\x7F\xFD>&\xBE\xEBYg\xFCZW\xA0Di\x14\xEA\xBCE\xB4\xAAGLg\xA5\x1BKQ`\xCA\xC6\r\xDB\x05\x90`\0\x90\xA3PPPa\x16\xA0V[\x86Q\x81\x10\x15a\x13\xE8Wa\x13\xB9\x87\x82\x81Q\x81\x10a\x13\xACWa\x13\xACaH,V[` \x02` \x01\x01Qa4]V[\x82\x82\x81Q\x81\x10a\x13\xCBWa\x13\xCBaH,V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a\x13\xE0\x81aJ,V[\x91PPa\x13\x8EV[P`\0a\x13\xFB`@\x8B\x01` \x8C\x01aA\x11V[\x82`@Q` \x01a\x14\r\x92\x91\x90aKGV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x87` \x015\x81\x14a\x14\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`P`$\x82\x01R\x7FThe pubkeys of non-signing opera`D\x82\x01R\x7Ftors supplied by the challenger `d\x82\x01Ro0\xB92\x9077\xBA\x101\xB7\xB992\xB1\xBA\x17`\x81\x1B`\x84\x82\x01R`\xA4\x01a\x06\x14V[`\0\x87Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14\xD2Wa\x14\xD2a;\xFCV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x14\xFBW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x88Q\x81\x10\x15a\x16OW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x18uH\xC8`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15hW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x8C\x91\x90aG[V[`\x01`\x01`\xA0\x1B\x03\x16c\xE8\xBB\x9A\xE6\x85\x83\x81Q\x81\x10a\x15\xACWa\x15\xACaH,V[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x15\xD2\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\xEFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\x13\x91\x90aG[V[\x82\x82\x81Q\x81\x10a\x16%Wa\x16%aH,V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x16G\x81aJ,V[\x91PPa\x15\x01V[Pc\xFF\xFF\xFF\xFF\x87\x16`\0\x81\x81R`\x9A` R`@\x80\x82 \x80T`\xFF\x19\x16`\x01\x17\x90UQ3\x92\x91\x7F\xC2\r\x1B\xB0\xF1b6\x800k\x83\xD4\xFFK\xB9\x9A+\xEB\x9D\x86\xD9x2\xF3\xCA@\xFD\x13\xA2\x9D\xF1\xEC\x91\xA3PPPPPPP[PPPPV[a\x16\xD1`@Q\x80`\x80\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`\0\x87`\x01`\x01`\xA0\x1B\x03\x16ch0H5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\x11W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x175\x91\x90aG[V[\x90Pa\x17b`@Q\x80`\x80\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`@Qc\x85\x02\rI`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90c\x85\x02\rI\x90a\x17\x92\x90\x8B\x90\x89\x90\x89\x90`\x04\x01aK\x8FV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\xAFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x17\xD7\x91\x90\x81\x01\x90aK\xD9V[\x81R`@Qc\xE1\x92\xE9\xAD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xE1\x92\xE9\xAD\x90a\x18\t\x90\x8B\x90\x8B\x90\x8B\x90`\x04\x01aLgV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18&W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x18N\x91\x90\x81\x01\x90aK\xD9V[`@\x82\x01R\x85`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18kWa\x18ka;\xFCV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x18\x9EW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x18\x89W\x90P[P``\x82\x01R`\0[`\xFF\x81\x16\x87\x11\x15a\x1C;W`\0\x85`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18\xCCWa\x18\xCCa;\xFCV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x18\xF5W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x83``\x01Q\x83`\xFF\x16\x81Q\x81\x10a\x19\x0FWa\x19\x0FaH,V[` \x02` \x01\x01\x81\x90RP`\0[\x86\x81\x10\x15a\x1B;W`\0\x8C`\x01`\x01`\xA0\x1B\x03\x16c0db\r\x8A\x8A\x85\x81\x81\x10a\x19HWa\x19HaH,V[\x90P` \x02\x015\x8E\x88`\0\x01Q\x86\x81Q\x81\x10a\x19fWa\x19faH,V[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x19\xA3\x93\x92\x91\x90\x92\x83Rc\xFF\xFF\xFF\xFF\x91\x82\x16` \x84\x01R\x16`@\x82\x01R``\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\xC0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\xE4\x91\x90aL\x90V[\x90P\x8A\x8A\x85`\xFF\x16\x81\x81\x10a\x19\xFBWa\x19\xFBaH,V[`\x01`\x01`\xC0\x1B\x03\x84\x16\x92\x015`\xF8\x1C\x91\x90\x91\x1C`\x01\x90\x81\x16\x14\x15\x90Pa\x1B(W\x85`\x01`\x01`\xA0\x1B\x03\x16cH\x08Xf\x8A\x8A\x85\x81\x81\x10a\x1A=Wa\x1A=aH,V[\x90P` \x02\x015\x8D\x8D\x88`\xFF\x16\x81\x81\x10a\x1AYWa\x1AYaH,V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x87\x90\x1B\x16\x81R`\x04\x81\x01\x94\x90\x94R\x91\x90\x91\x015`\xF8\x1C`$\x83\x01RPc\xFF\xFF\xFF\xFF\x8F\x16`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\xAFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xD3\x91\x90aL\xB9V[\x85``\x01Q\x85`\xFF\x16\x81Q\x81\x10a\x1A\xECWa\x1A\xECaH,V[` \x02` \x01\x01Q\x84\x81Q\x81\x10a\x1B\x05Wa\x1B\x05aH,V[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x82a\x1B$\x81aJ,V[\x93PP[P\x80a\x1B3\x81aJ,V[\x91PPa\x19\x1DV[P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1BVWa\x1BVa;\xFCV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1B\x7FW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a\x1C\0W\x84``\x01Q\x84`\xFF\x16\x81Q\x81\x10a\x1B\xA6Wa\x1B\xA6aH,V[` \x02` \x01\x01Q\x81\x81Q\x81\x10a\x1B\xBFWa\x1B\xBFaH,V[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a\x1B\xD9Wa\x1B\xD9aH,V[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x1B\xF8\x81aJ,V[\x91PPa\x1B\x85V[P\x80\x84``\x01Q\x84`\xFF\x16\x81Q\x81\x10a\x1C\x1BWa\x1C\x1BaH,V[` \x02` \x01\x01\x81\x90RPPP\x80\x80a\x1C3\x90aL\xD6V[\x91PPa\x18\xA7V[P`\0\x89`\x01`\x01`\xA0\x1B\x03\x16c5a\xDE\xB1`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C|W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xA0\x91\x90aG[V[`@Qc\xED\xA1\x07c`\xE0\x1B\x81R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xED\xA1\x07c\x90a\x1C\xD3\x90\x8B\x90\x8B\x90\x8E\x90`\x04\x01aL\xF6V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\xF0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1D\x18\x91\x90\x81\x01\x90aK\xD9V[` \x83\x01RP\x98\x97PPPPPPPPV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1DrW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\x96\x91\x90aG\xC2V[a\x1D\xB2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x14\x90aG\xE4V[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[`\x9CT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1EUW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FTask generator must be the calle`D\x82\x01R`9`\xF9\x1B`d\x82\x01R`\x84\x01a\x06\x14V[a\x1E\x8C`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0c\xFF\xFF\xFF\xFF\x16\x81R` \x01``\x81R` \x01`\0c\xFF\xFF\xFF\xFF\x16\x81RP\x90V[\x84\x81Rc\xFF\xFF\xFF\xFFC\x81\x16` \x80\x84\x01\x91\x90\x91R\x90\x85\x16``\x83\x01R`@\x80Q`\x1F\x85\x01\x83\x90\x04\x83\x02\x81\x01\x83\x01\x90\x91R\x83\x81R\x90\x84\x90\x84\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPPP`@\x80\x83\x01\x91\x90\x91RQa\x1E\xF2\x90\x82\x90` \x01aMLV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 `\x97\x80Tc\xFF\xFF\xFF\xFF\x90\x81\x16`\0\x90\x81R`\x98\x90\x94R\x93\x90\x92 UT\x16\x90\x7F\x16\x95\xB8\xD0n\xC8\0\xB4a^t\\\xFB[\xD0\x0C\x1F(ua]B\x92\\;Z\xFAT;\xB2LH\x90a\x1FU\x90\x84\x90aMLV[`@Q\x80\x91\x03\x90\xA2`\x97Ta\x1Fq\x90c\xFF\xFF\xFF\xFF\x16`\x01aI\x8AV[`\x97\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01\x81\x90R\x90\x81[\x86\x81\x10\x15a!\xAFW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xC1\xAFk$\x89\x89\x84\x81\x81\x10a \x08Wa \x08aH,V[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x88\x88`\xA0\x01Q\x85\x81Q\x81\x10a ,Wa ,aH,V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81R`\xFF\x90\x93\x16`\x04\x84\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x84\x01R\x16`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a \x88W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \xAC\x91\x90aM\xB1V[`\x01`\x01`@\x1B\x03\x19\x16a \xCF\x86`@\x01Q\x83\x81Q\x81\x10a\x13\xACWa\x13\xACaH,V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x14a!kW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`a`$\x82\x01R`\0\x80Q` aO\x13\x839\x81Q\x91R`D\x82\x01R\x7Fres: quorumApk hash in storage d`d\x82\x01R\x7Foes not match provided quorum ap`\x84\x82\x01R`k`\xF8\x1B`\xA4\x82\x01R`\xC4\x01a\x06\x14V[a!\x9B\x85`@\x01Q\x82\x81Q\x81\x10a!\x84Wa!\x84aH,V[` \x02` \x01\x01Q\x83a0U\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91P\x80a!\xA7\x81aJ,V[\x91PPa\x1F\xBFV[P`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R\x86`\x01`\x01`@\x1B\x03\x81\x11\x15a!\xDCWa!\xDCa;\xFCV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\"\x05W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P` \x82\x01R\x86`\x01`\x01`@\x1B\x03\x81\x11\x15a\"#Wa\"#a;\xFCV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\"LW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x81R` \x85\x01QQ`\0\x90`\x01`\x01`@\x1B\x03\x81\x11\x15a\"oWa\"oa;\xFCV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\"\x98W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x86` \x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\xBAWa\"\xBAa;\xFCV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\"\xE3W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0a#'\x8B\x8B\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa4\xA0\x92PPPV[\x90P`\0[\x88` \x01QQ\x81\x10\x15a%\x92Wa#R\x89` \x01Q\x82\x81Q\x81\x10a\x13\xACWa\x13\xACaH,V[\x84\x82\x81Q\x81\x10a#dWa#daH,V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80\x15a$\x1CW\x83a#\x81`\x01\x83aM\xDCV[\x81Q\x81\x10a#\x91Wa#\x91aH,V[` \x02` \x01\x01Q`\0\x1C\x84\x82\x81Q\x81\x10a#\xAEWa#\xAEaH,V[` \x02` \x01\x01Q`\0\x1C\x11a$\x1CW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R`\0\x80Q` aO\x13\x839\x81Q\x91R`D\x82\x01R\x7Fres: nonSignerPubkeys not sorted`d\x82\x01R`\x84\x01a\x06\x14V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c0db\r\x85\x83\x81Q\x81\x10a$]Wa$]aH,V[` \x02` \x01\x01Q\x8C\x8C`\0\x01Q\x85\x81Q\x81\x10a$|Wa$|aH,V[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a$\xB9\x93\x92\x91\x90\x92\x83Rc\xFF\xFF\xFF\xFF\x91\x82\x16` \x84\x01R\x16`@\x82\x01R``\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a$\xD6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$\xFA\x91\x90aL\x90V[`\x01`\x01`\xC0\x1B\x03\x16\x83\x82\x81Q\x81\x10a%\x15Wa%\x15aH,V[` \x02` \x01\x01\x81\x81RPPa%~a%wa%K\x84\x86\x85\x81Q\x81\x10a%=Wa%=aH,V[` \x02` \x01\x01Q\x16a6\tV[a%q\x8C` \x01Q\x85\x81Q\x81\x10a%dWa%daH,V[` \x02` \x01\x01Qa6:V[\x90a6\xD5V[\x87\x90a0UV[\x95P\x80a%\x8A\x81aJ,V[\x91PPa#,V[PP`\0[`\xFF\x81\x16\x8A\x11\x15a(\xD2W`\0\x8B\x8B\x83`\xFF\x16\x81\x81\x10a%\xB9Wa%\xB9aH,V[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xC8)LV\x82\x8C\x8C`\xC0\x01Q\x86`\xFF\x16\x81Q\x81\x10a&\x12Wa&\x12aH,V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81R`\xFF\x90\x93\x16`\x04\x84\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x84\x01R\x16`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&nW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\x92\x91\x90aK\x03V[\x85` \x01Q\x83`\xFF\x16\x81Q\x81\x10a&\xABWa&\xABaH,V[`\x01`\x01``\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x82\x01R\x85\x01Q\x80Q`\xFF\x84\x16\x90\x81\x10a&\xDAWa&\xDAaH,V[` \x02` \x01\x01Q\x85`\0\x01Q\x83`\xFF\x16\x81Q\x81\x10a&\xFBWa&\xFBaH,V[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPP`\0[\x89` \x01QQ\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a(\xC8W`\0a'd\x85\x83c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a'NWa'NaH,V[` \x02` \x01\x01Q\x84`\xFF\x16\x1C`\x01\x90\x81\x16\x14\x90V[\x15a(\xB5W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xA4<\xDE\x89\x84\x8E\x89\x86c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a'\xB2Wa'\xB2aH,V[` \x02` \x01\x01Q\x8F`\xE0\x01Q\x89`\xFF\x16\x81Q\x81\x10a'\xD3Wa'\xD3aH,V[` \x02` \x01\x01Q\x86c\xFF\xFF\xFF\xFF\x16\x81Q\x81\x10a'\xF2Wa'\xF2aH,V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x87\x90\x1B\x16\x81R`\xFF\x90\x94\x16`\x04\x85\x01Rc\xFF\xFF\xFF\xFF\x92\x83\x16`$\x85\x01R`D\x84\x01\x91\x90\x91R\x16`d\x82\x01R`\x84\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a(VW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(z\x91\x90aK\x03V[\x87Q\x80Q`\xFF\x87\x16\x90\x81\x10a(\x91Wa(\x91aH,V[` \x02` \x01\x01\x81\x81Qa(\xA5\x91\x90aM\xF3V[`\x01`\x01``\x1B\x03\x16\x90RP`\x01\x01[P\x80a(\xC0\x81aN\x1BV[\x91PPa'\x1EV[PP`\x01\x01a%\x97V[PP`\0\x80a(\xEB\x8C\x86\x8A``\x01Q\x8B`\x80\x01Qa\x07hV[\x91P\x91P\x81a)\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R`\0\x80Q` aO\x13\x839\x81Q\x91R`D\x82\x01R\x7Fres: pairing precompile call fai`d\x82\x01Rb\x1B\x19Y`\xEA\x1B`\x84\x82\x01R`\xA4\x01a\x06\x14V[\x80a)\xBDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R`\0\x80Q` aO\x13\x839\x81Q\x91R`D\x82\x01R\x7Fres: signature is invalid\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x14V[PP`\0\x87\x82`@Q` \x01a)\xD4\x92\x91\x90aKGV[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x92\x9B\x92\x9AP\x91\x98PPPPPPPPPV[a*\x06a7\xBAV[a*\x10`\0a8\x14V[V[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91``\x91\x83\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x84\x81`\0\x81Q\x81\x10a*MWa*MaH,V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Qc\x85\x02\rI`\xE0\x1B\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90c\x85\x02\rI\x90a*\x89\x90\x88\x90\x86\x90`\x04\x01aN?V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a*\xA6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra*\xCE\x91\x90\x81\x01\x90aK\xD9V[`\0\x81Q\x81\x10a*\xE0Wa*\xE0aH,V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Qc0db\r`\xE0\x1B\x81R`\x04\x81\x01\x88\x90Rc\xFF\xFF\xFF\xFF\x87\x81\x16`$\x83\x01R\x90\x91\x16`D\x82\x01\x81\x90R\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x89\x16\x90c0db\r\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+LW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+p\x91\x90aL\x90V[`\x01`\x01`\xC0\x1B\x03\x16\x90P`\0a+\x86\x82a8fV[\x90P\x81a+\x94\x8A\x83\x8Aa\rqV[\x95P\x95PPPPP\x93P\x93\x91PPV[a+\xACa7\xBAV[`\x01`\x01`\xA0\x1B\x03\x81\x16a,\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x06\x14V[a\x06&\x81a8\x14V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a,:WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a,TWP0;\x15\x80\x15a,TWP`\0T`\xFF\x16`\x01\x14[a,\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x06\x14V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a,\xDAW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a,\xE5\x85`\0a8\xC3V[a,\xEE\x84a8\x14V[`\x9B\x80T`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U`\x9C\x80T\x92\x85\x16\x92\x90\x91\x16\x91\x90\x91\x17\x90U\x80\x15a-dW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-\xBEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-\xE2\x91\x90aG[V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a.\x12W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x14\x90aGxV[`fT\x19\x81\x19`fT\x19\x16\x14a.\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x14V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x07]V[`\x01`\x01`\xA0\x1B\x03\x81\x16a/UW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x06\x14V[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra/\xDAa:\xD7V[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x80\x82\x01\x84\x90R`\0\x90\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a0\rWa0\x0FV[\xFE[P\x80a0MW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCB[][\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x06\x14V[PP\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra0qa:\xF5V[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R\x83Q`@\x80\x84\x01\x91\x90\x91R\x90\x84\x01Q``\x83\x01R`\0\x90\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a0\rWP\x80a0MW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCBXY\x19\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x06\x14V[a0\xF1a;\x13V[P`@\x80Q`\x80\x81\x01\x82R\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81\x83\x01\x90\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED``\x83\x01R\x81R\x81Q\x80\x83\x01\x90\x92R\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x81\x01\x91\x90\x91R\x81\x01\x91\x90\x91R\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0\x80\x80a1\xD9`\0\x80Q` aN\xF3\x839\x81Q\x91R\x86aHBV[\x90P[a1\xE5\x81a9\xADV[\x90\x93P\x91P`\0\x80Q` aN\xF3\x839\x81Q\x91R\x82\x83\t\x83\x14\x15a2\x1FW`@\x80Q\x80\x82\x01\x90\x91R\x90\x81R` \x81\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80Q` aN\xF3\x839\x81Q\x91R`\x01\x82\x08\x90Pa1\xDCV[`@\x80Q\x80\x82\x01\x82R\x86\x81R` \x80\x82\x01\x86\x90R\x82Q\x80\x84\x01\x90\x93R\x86\x83R\x82\x01\x84\x90R`\0\x91\x82\x91\x90a2ka;8V[`\0[`\x02\x81\x10\x15a40W`\0a2\x84\x82`\x06aJ\rV[\x90P\x84\x82`\x02\x81\x10a2\x98Wa2\x98aH,V[` \x02\x01QQ\x83a2\xAA\x83`\0aN\x93V[`\x0C\x81\x10a2\xBAWa2\xBAaH,V[` \x02\x01R\x84\x82`\x02\x81\x10a2\xD1Wa2\xD1aH,V[` \x02\x01Q` \x01Q\x83\x82`\x01a2\xE8\x91\x90aN\x93V[`\x0C\x81\x10a2\xF8Wa2\xF8aH,V[` \x02\x01R\x83\x82`\x02\x81\x10a3\x0FWa3\x0FaH,V[` \x02\x01QQQ\x83a3\"\x83`\x02aN\x93V[`\x0C\x81\x10a32Wa32aH,V[` \x02\x01R\x83\x82`\x02\x81\x10a3IWa3IaH,V[` \x02\x01QQ`\x01` \x02\x01Q\x83a3b\x83`\x03aN\x93V[`\x0C\x81\x10a3rWa3raH,V[` \x02\x01R\x83\x82`\x02\x81\x10a3\x89Wa3\x89aH,V[` \x02\x01Q` \x01Q`\0`\x02\x81\x10a3\xA4Wa3\xA4aH,V[` \x02\x01Q\x83a3\xB5\x83`\x04aN\x93V[`\x0C\x81\x10a3\xC5Wa3\xC5aH,V[` \x02\x01R\x83\x82`\x02\x81\x10a3\xDCWa3\xDCaH,V[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a3\xF7Wa3\xF7aH,V[` \x02\x01Q\x83a4\x08\x83`\x05aN\x93V[`\x0C\x81\x10a4\x18Wa4\x18aH,V[` \x02\x01RP\x80a4(\x81aJ,V[\x91PPa2nV[Pa49a;WV[`\0` \x82a\x01\x80\x85`\x08\x8C\xFA\x91Q\x91\x9C\x91\x15\x15\x9BP\x90\x99PPPPPPPPPPV[`\0\x81`\0\x01Q\x82` \x01Q`@Q` \x01a4\x83\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0a\x01\0\x82Q\x11\x15a5\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FBitmapUtils.bytesArrayToBitmap: `D\x82\x01RubytesArray is too long`P\x1B`d\x82\x01R`\x84\x01a\x06\x14V[\x81Qa5\"WP`\0\x91\x90PV[`\0\x80\x83`\0\x81Q\x81\x10a58Wa58aH,V[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x81\x90\x1B\x92P[\x84Q\x81\x10\x15a6\0W\x84\x81\x81Q\x81\x10a5fWa5faH,V[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x1B\x91P\x82\x82\x16\x15a5\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FBitmapUtils.bytesArrayToBitmap: `D\x82\x01R\x7Frepeat entry in bytesArray\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x14V[\x91\x81\x17\x91a5\xF9\x81aJ,V[\x90Pa5KV[P\x90\x93\x92PPPV[`\0\x80[\x82\x15a64Wa6\x1E`\x01\x84aM\xDCV[\x90\x92\x16\x91\x80a6,\x81aN\xABV[\x91PPa6\rV[\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q\x15\x80\x15a6_WP` \x82\x01Q\x15[\x15a6}WPP`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01`\0\x80Q` aN\xF3\x839\x81Q\x91R\x84` \x01Qa6\xB0\x91\x90aHBV[a6\xC8\x90`\0\x80Q` aN\xF3\x839\x81Q\x91RaM\xDCV[\x90R\x92\x91PPV[\x91\x90PV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x02\0\x82a\xFF\xFF\x16\x10a71W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Roscalar-too-large`\x80\x1B`D\x82\x01R`d\x01a\x06\x14V[\x81a\xFF\xFF\x16`\x01\x14\x15a7EWP\x81a64V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01\x81\x90R\x84\x90`\x01\x90[\x81a\xFF\xFF\x16\x86a\xFF\xFF\x16\x11\x15a7\xAFW`\x01a\xFF\xFF\x87\x16`\xFF\x83\x16\x1C\x81\x16\x14\x15a7\x92Wa7\x8F\x84\x84a0UV[\x93P[a7\x9C\x83\x84a0UV[\x92Pb\x01\xFF\xFE`\x01\x92\x83\x1B\x16\x91\x01a7aV[P\x91\x95\x94PPPPPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a*\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x06\x14V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[```\0\x80[a\x01\0\x81\x10\x15a8\xBCW`\x01\x81\x1B\x91P\x83\x82\x16\x15a8\xACW\x82\x81`\xF8\x1B`@Q` \x01a8\x9A\x92\x91\x90aN\xC3V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92P[a8\xB5\x81aJ,V[\x90Pa8lV[PP\x91\x90PV[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a8\xE4WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a9fW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x06\x14V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a9\xA9\x82a.\xC7V[PPV[`\0\x80\x80`\0\x80Q` aN\xF3\x839\x81Q\x91R`\x03`\0\x80Q` aN\xF3\x839\x81Q\x91R\x86`\0\x80Q` aN\xF3\x839\x81Q\x91R\x88\x89\t\t\x08\x90P`\0a:#\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\0\x80Q` aN\xF3\x839\x81Q\x91Ra:/V[\x91\x95\x91\x94P\x90\x92PPPV[`\0\x80a::a;WV[a:Ba;uV[` \x80\x82R\x81\x81\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x88\x90R`\x80\x82\x01\x87\x90R`\xA0\x82\x01\x86\x90R\x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82\x80\x15a0\rWP\x82a:\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FBN254.expMod: call failure\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\x14V[PQ\x95\x94PPPPPV[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80a;&a;\x93V[\x81R` \x01a;3a;\x93V[\x90R\x90V[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06&W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a;\xD8W`\0\x80\xFD[\x815a\x10\xE5\x81a;\xB1V[`\0` \x82\x84\x03\x12\x15a;\xF5W`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a<4Wa<4a;\xFCV[`@R\x90V[`@Qa\x01\0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a<4Wa<4a;\xFCV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a<\x85Wa<\x85a;\xFCV[`@R\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a<\x9FW`\0\x80\xFD[a<\xA7a<\x12V[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a<\xCEW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a<\xF0Wa<\xF0a;\xFCV[\x80`@RP\x80`@\x84\x01\x85\x81\x11\x15a=\x07W`\0\x80\xFD[\x84[\x81\x81\x10\x15a7\xAFW\x805\x83R` \x92\x83\x01\x92\x01a=\tV[`\0`\x80\x82\x84\x03\x12\x15a=3W`\0\x80\xFD[a=;a<\x12V[\x90Pa=G\x83\x83a<\xBDV[\x81Ra=V\x83`@\x84\x01a<\xBDV[` \x82\x01R\x92\x91PPV[`\0\x80`\0\x80a\x01 \x85\x87\x03\x12\x15a=xW`\0\x80\xFD[\x845\x93Pa=\x89\x86` \x87\x01a<\x8DV[\x92Pa=\x98\x86``\x87\x01a=!V[\x91Pa=\xA7\x86`\xE0\x87\x01a<\x8DV[\x90P\x92\x95\x91\x94P\x92PV[`\0`\x80\x82\x84\x03\x12\x15a=\xC4W`\0\x80\xFD[P\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a=\xC4W`\0\x80\xFD[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a=\xF5Wa=\xF5a;\xFCV[P`\x05\x1B` \x01\x90V[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x06&W`\0\x80\xFD[\x805a6\xD0\x81a=\xFFV[`\0\x82`\x1F\x83\x01\x12a>-W`\0\x80\xFD[\x815` a>Ba>=\x83a=\xDCV[a<]V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a>aW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a>\x85W\x805a>x\x81a=\xFFV[\x83R\x91\x83\x01\x91\x83\x01a>eV[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a>\xA1W`\0\x80\xFD[\x815` a>\xB1a>=\x83a=\xDCV[\x82\x81R`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a>\xD0W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a>\x85Wa>\xE6\x88\x82a<\x8DV[\x83R\x91\x83\x01\x91`@\x01a>\xD4V[`\0\x82`\x1F\x83\x01\x12a?\x05W`\0\x80\xFD[\x815` a?\x15a>=\x83a=\xDCV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a?4W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a>\x85W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a?WW`\0\x80\x81\xFD[a?e\x89\x86\x83\x8B\x01\x01a>\x1CV[\x84RP\x91\x83\x01\x91\x83\x01a?8V[`\0a\x01\x80\x82\x84\x03\x12\x15a?\x86W`\0\x80\xFD[a?\x8Ea<:V[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a?\xA7W`\0\x80\xFD[a?\xB3\x85\x83\x86\x01a>\x1CV[\x83R` \x84\x015\x91P\x80\x82\x11\x15a?\xC9W`\0\x80\xFD[a?\xD5\x85\x83\x86\x01a>\x90V[` \x84\x01R`@\x84\x015\x91P\x80\x82\x11\x15a?\xEEW`\0\x80\xFD[a?\xFA\x85\x83\x86\x01a>\x90V[`@\x84\x01Ra@\x0C\x85``\x86\x01a=!V[``\x84\x01Ra@\x1E\x85`\xE0\x86\x01a<\x8DV[`\x80\x84\x01Ra\x01 \x84\x015\x91P\x80\x82\x11\x15a@8W`\0\x80\xFD[a@D\x85\x83\x86\x01a>\x1CV[`\xA0\x84\x01Ra\x01@\x84\x015\x91P\x80\x82\x11\x15a@^W`\0\x80\xFD[a@j\x85\x83\x86\x01a>\x1CV[`\xC0\x84\x01Ra\x01`\x84\x015\x91P\x80\x82\x11\x15a@\x84W`\0\x80\xFD[Pa@\x91\x84\x82\x85\x01a>\xF4V[`\xE0\x83\x01RP\x92\x91PPV[`\0\x80`\0`\x80\x84\x86\x03\x12\x15a@\xB2W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a@\xC9W`\0\x80\xFD[a@\xD5\x87\x83\x88\x01a=\xB2V[\x94Pa@\xE4\x87` \x88\x01a=\xCAV[\x93P``\x86\x015\x91P\x80\x82\x11\x15a@\xFAW`\0\x80\xFD[PaA\x07\x86\x82\x87\x01a?sV[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15aA#W`\0\x80\xFD[\x815a\x10\xE5\x81a=\xFFV[`\0\x80`\0``\x84\x86\x03\x12\x15aACW`\0\x80\xFD[\x835aAN\x81a;\xB1V[\x92P` \x84\x81\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aAkW`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12aA\x7FW`\0\x80\xFD[\x815\x81\x81\x11\x15aA\x91WaA\x91a;\xFCV[aA\xA3`\x1F\x82\x01`\x1F\x19\x16\x85\x01a<]V[\x91P\x80\x82R\x88\x84\x82\x85\x01\x01\x11\x15aA\xB9W`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x94PPPPaA\xDC`@\x85\x01a>\x11V[\x90P\x92P\x92P\x92V[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0\x80[\x86\x81\x10\x15aBgW\x83\x85\x03\x8AR\x82Q\x80Q\x80\x87R\x90\x87\x01\x90\x87\x87\x01\x90\x84[\x81\x81\x10\x15aBRW\x83Q\x80Q\x84R\x8A\x01Q`\x01`\x01``\x1B\x03\x16\x8A\x84\x01R\x92\x89\x01\x92`@\x90\x92\x01\x91`\x01\x01aB\"V[PP\x9A\x87\x01\x9A\x95PP\x91\x85\x01\x91`\x01\x01aB\x04V[P\x92\x98\x97PPPPPPPPV[` \x81R`\0a\x10\xE5` \x83\x01\x84aA\xE5V[`\0\x80`\0\x80`\xC0\x85\x87\x03\x12\x15aB\x9EW`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aB\xB5W`\0\x80\xFD[aB\xC1\x88\x83\x89\x01a=\xB2V[\x95PaB\xD0\x88` \x89\x01a=\xCAV[\x94PaB\xDF\x88``\x89\x01a=\xCAV[\x93P`\xA0\x87\x015\x91P\x80\x82\x11\x15aB\xF5W`\0\x80\xFD[PaC\x02\x87\x82\x88\x01a>\x90V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80\x83`\x1F\x84\x01\x12aC W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aC7W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aCOW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\x80\x87\x89\x03\x12\x15aCoW`\0\x80\xFD[\x865aCz\x81a;\xB1V[\x95P` \x87\x015aC\x8A\x81a=\xFFV[\x94P`@\x87\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aC\xA6W`\0\x80\xFD[aC\xB2\x8A\x83\x8B\x01aC\x0EV[\x90\x96P\x94P``\x89\x015\x91P\x80\x82\x11\x15aC\xCBW`\0\x80\xFD[\x81\x89\x01\x91P\x89`\x1F\x83\x01\x12aC\xDFW`\0\x80\xFD[\x815\x81\x81\x11\x15aC\xEEW`\0\x80\xFD[\x8A` \x82`\x05\x1B\x85\x01\x01\x11\x15aD\x03W`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92\x95P\x92\x95P\x92\x95V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aDOW\x81Qc\xFF\xFF\xFF\xFF\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aD-V[P\x94\x95\x94PPPPPV[`\0` \x80\x83R\x83Q`\x80\x82\x85\x01RaDv`\xA0\x85\x01\x82aD\x19V[\x90P\x81\x85\x01Q`\x1F\x19\x80\x86\x84\x03\x01`@\x87\x01RaD\x93\x83\x83aD\x19V[\x92P`@\x87\x01Q\x91P\x80\x86\x84\x03\x01``\x87\x01RaD\xB0\x83\x83aD\x19V[``\x88\x01Q\x87\x82\x03\x83\x01`\x80\x89\x01R\x80Q\x80\x83R\x91\x94P\x85\x01\x92P\x84\x84\x01\x90`\x05\x81\x90\x1B\x85\x01\x86\x01`\0[\x82\x81\x10\x15aE\x07W\x84\x87\x83\x03\x01\x84RaD\xF5\x82\x87QaD\x19V[\x95\x88\x01\x95\x93\x88\x01\x93\x91P`\x01\x01aD\xDBV[P\x99\x98PPPPPPPPPV[`\0` \x82\x84\x03\x12\x15aE'W`\0\x80\xFD[\x815`\xFF\x81\x16\x81\x14a\x10\xE5W`\0\x80\xFD[`\0\x80`\0\x80``\x85\x87\x03\x12\x15aENW`\0\x80\xFD[\x845\x93P` \x85\x015aE`\x81a=\xFFV[\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aE{W`\0\x80\xFD[aE\x87\x87\x82\x88\x01aC\x0EV[\x95\x98\x94\x97P\x95PPPPV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15aE\xABW`\0\x80\xFD[\x855\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aE\xC9W`\0\x80\xFD[aE\xD5\x89\x83\x8A\x01aC\x0EV[\x90\x96P\x94P`@\x88\x015\x91PaE\xEA\x82a=\xFFV[\x90\x92P``\x87\x015\x90\x80\x82\x11\x15aF\0W`\0\x80\xFD[PaF\r\x88\x82\x89\x01a?sV[\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aDOW\x81Q`\x01`\x01``\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aF.V[`@\x81R`\0\x83Q`@\x80\x84\x01RaFn`\x80\x84\x01\x82aF\x1AV[\x90P` \x85\x01Q`?\x19\x84\x83\x03\x01``\x85\x01RaF\x8B\x82\x82aF\x1AV[\x92PPP\x82` \x83\x01R\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15aF\xB1W`\0\x80\xFD[\x835aF\xBC\x81a;\xB1V[\x92P` \x84\x015\x91P`@\x84\x015aF\xD3\x81a=\xFFV[\x80\x91PP\x92P\x92P\x92V[\x82\x81R`@` \x82\x01R`\0aF\xF7`@\x83\x01\x84aA\xE5V[\x94\x93PPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aG\x15W`\0\x80\xFD[\x845aG \x81a;\xB1V[\x93P` \x85\x015aG0\x81a;\xB1V[\x92P`@\x85\x015aG@\x81a;\xB1V[\x91P``\x85\x015aGP\x81a;\xB1V[\x93\x96\x92\x95P\x90\x93PPV[`\0` \x82\x84\x03\x12\x15aGmW`\0\x80\xFD[\x81Qa\x10\xE5\x81a;\xB1V[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15aG\xD4W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x10\xE5W`\0\x80\xFD[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x82aH_WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aH{W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aH\x95W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15aCOW`\0\x80\xFD[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x81R\x815` \x82\x01R`\0` \x83\x015aH\xEE\x81a=\xFFV[c\xFF\xFF\xFF\xFF\x81\x16`@\x84\x01RP`@\x83\x015`\x1E\x19\x846\x03\x01\x81\x12aI\x12W`\0\x80\xFD[\x83\x01\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aI*W`\0\x80\xFD[\x806\x03\x85\x13\x15aI9W`\0\x80\xFD[`\x80``\x85\x01RaIQ`\xA0\x85\x01\x82` \x85\x01aH\xAAV[\x91PPaI```\x85\x01a>\x11V[c\xFF\xFF\xFF\xFF\x81\x16`\x80\x85\x01RP\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aI\xA9WaI\xA9aItV[\x01\x94\x93PPPPV[\x805aI\xBD\x81a=\xFFV[c\xFF\xFF\xFF\xFF\x16\x82R` \x90\x81\x015\x91\x01RV[`@\x81\x01a64\x82\x84aI\xB2V[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aJ\x04WaJ\x04aItV[\x02\x94\x93PPPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15aJ'WaJ'aItV[P\x02\x90V[`\0`\0\x19\x82\x14\x15aJ@WaJ@aItV[P`\x01\x01\x90V[`\x80\x81\x01aJU\x82\x85aI\xB2V[c\xFF\xFF\xFF\xFF\x83Q\x16`@\x83\x01R` \x83\x01Q``\x83\x01R\x93\x92PPPV[`\0` \x80\x83\x85\x03\x12\x15aJ\x86W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aJ\x9CW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aJ\xADW`\0\x80\xFD[\x80QaJ\xBBa>=\x82a=\xDCV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15aJ\xDAW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aJ\xF8W\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90aJ\xDFV[\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15aK\x15W`\0\x80\xFD[\x81Q`\x01`\x01``\x1B\x03\x81\x16\x81\x14a\x10\xE5W`\0\x80\xFD[`\x80\x81\x01aK:\x82\x85aI\xB2V[a\x10\xE5`@\x83\x01\x84aI\xB2V[c\xFF\xFF\xFF\xFF`\xE0\x1B\x83`\xE0\x1B\x16\x81R`\0`\x04\x82\x01\x83Q` \x80\x86\x01`\0[\x83\x81\x10\x15aK\x82W\x81Q\x85R\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01aKfV[P\x92\x97\x96PPPPPPPV[c\xFF\xFF\xFF\xFF\x84\x16\x81R`@` \x82\x01\x81\x90R\x81\x01\x82\x90R`\0`\x01`\x01`\xFB\x1B\x03\x83\x11\x15aK\xBCW`\0\x80\xFD[\x82`\x05\x1B\x80\x85``\x85\x017`\0\x92\x01``\x01\x91\x82RP\x93\x92PPPV[`\0` \x80\x83\x85\x03\x12\x15aK\xECW`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aL\x02W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aL\x13W`\0\x80\xFD[\x80QaL!a>=\x82a=\xDCV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15aL@W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aJ\xF8W\x83QaLX\x81a=\xFFV[\x82R\x92\x84\x01\x92\x90\x84\x01\x90aLEV[c\xFF\xFF\xFF\xFF\x84\x16\x81R`@` \x82\x01R`\0aL\x87`@\x83\x01\x84\x86aH\xAAV[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15aL\xA2W`\0\x80\xFD[\x81Q`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x14a\x10\xE5W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aL\xCBW`\0\x80\xFD[\x81Qa\x10\xE5\x81a=\xFFV[`\0`\xFF\x82\x16`\xFF\x81\x14\x15aL\xEDWaL\xEDaItV[`\x01\x01\x92\x91PPV[`@\x81R`\0aM\n`@\x83\x01\x85\x87aH\xAAV[\x90Pc\xFF\xFF\xFF\xFF\x83\x16` \x83\x01R\x94\x93PPPPV[`\0[\x83\x81\x10\x15aM;W\x81\x81\x01Q\x83\x82\x01R` \x01aM#V[\x83\x81\x11\x15a\x16\xA0WPP`\0\x91\x01RV[` \x81R\x81Q` \x82\x01R`\0` \x83\x01Qc\xFF\xFF\xFF\xFF\x80\x82\x16`@\x85\x01R`@\x85\x01Q\x91P`\x80``\x85\x01R\x81Q\x80`\xA0\x86\x01RaM\x92\x81`\xC0\x87\x01` \x86\x01aM V[``\x95\x90\x95\x01Q\x16`\x80\x84\x01RPP`\xC0`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15aM\xC3W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16\x81\x14a\x10\xE5W`\0\x80\xFD[`\0\x82\x82\x10\x15aM\xEEWaM\xEEaItV[P\x03\x90V[`\0`\x01`\x01``\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15aN\x13WaN\x13aItV[\x03\x93\x92PPPV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x14\x15aN5WaN5aItV[`\x01\x01\x93\x92PPPV[`\0`@\x82\x01c\xFF\xFF\xFF\xFF\x85\x16\x83R` `@\x81\x85\x01R\x81\x85Q\x80\x84R``\x86\x01\x91P\x82\x87\x01\x93P`\0[\x81\x81\x10\x15aN\x86W\x84Q\x83R\x93\x83\x01\x93\x91\x83\x01\x91`\x01\x01aNjV[P\x90\x97\x96PPPPPPPV[`\0\x82\x19\x82\x11\x15aN\xA6WaN\xA6aItV[P\x01\x90V[`\0a\xFF\xFF\x80\x83\x16\x81\x81\x14\x15aN5WaN5aItV[`\0\x83QaN\xD5\x81\x84` \x88\x01aM V[`\x01`\x01`\xF8\x1B\x03\x19\x93\x90\x93\x16\x91\x90\x92\x01\x90\x81R`\x01\x01\x92\x91PPV\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDGBLSSignatureChecker.checkSignatu\xA2dipfsX\"\x12 \xE2}fu\xA6Fi\x87[U\x96\x91\xCD\xCE\x0F\x04d\xB8\x8D'\0\x0E\xB2^\xCC\x06\xE7|\xFC\x01OAdsolcC\0\x08\x0C\x003";
    /// The deployed bytecode of the contract.
    pub static MANGATATASKMANAGER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MangataTaskManager<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MangataTaskManager<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MangataTaskManager<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MangataTaskManager<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MangataTaskManager<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MangataTaskManager))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MangataTaskManager<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MANGATATASKMANAGER_ABI.clone(),
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
                MANGATATASKMANAGER_ABI.clone(),
                MANGATATASKMANAGER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `TASK_CHALLENGE_WINDOW_BLOCK` (0xf63c5bab) function
        pub fn task_challenge_window_block(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([246, 60, 91, 171], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `TASK_RESPONSE_WINDOW_BLOCK` (0x1ad43189) function
        pub fn task_response_window_block(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([26, 212, 49, 137], ())
                .expect("method not found (this should never happen)")
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
        ///Calls the contract's `blsPubkeyRegistry` (0x3561deb1) function
        pub fn bls_pubkey_registry(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([53, 97, 222, 177], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkSignatures` (0x6efb4636) function
        pub fn check_signatures(
            &self,
            msg_hash: [u8; 32],
            quorum_numbers: ::ethers::core::types::Bytes,
            reference_block_number: u32,
            non_signer_stakes_and_signature: NonSignerStakesAndSignature,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (QuorumStakeTotals, [u8; 32]),
        > {
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
        ///Calls the contract's `getTaskChallangeWindowBlock` (0xe3b5ee04) function
        pub fn get_task_challange_window_block(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([227, 181, 238, 4], ())
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
        ///Calls the contract's `raiseAndResolveChallenge` (0x3cd27f64) function
        pub fn raise_and_resolve_challenge(
            &self,
            task: Task,
            task_response: TaskResponse,
            task_response_metadata: TaskResponseMetadata,
            pubkeys_of_non_signing_operators: ::std::vec::Vec<G1Point>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [60, 210, 127, 100],
                    (
                        task,
                        task_response,
                        task_response_metadata,
                        pubkeys_of_non_signing_operators,
                    ),
                )
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
        ///Calls the contract's `respondToTask` (0x277cb995) function
        pub fn respond_to_task(
            &self,
            task: Task,
            task_response: TaskResponse,
            non_signer_stakes_and_signature: NonSignerStakesAndSignature,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [39, 124, 185, 149],
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
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([104, 48, 72, 53], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `taskNumber` (0x72d18e8d) function
        pub fn task_number(&self) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([114, 209, 142, 141], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `taskSuccesfullyChallenged` (0x5decc3f5) function
        pub fn task_succesfully_challenged(
            &self,
            p0: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([93, 236, 195, 245], p0)
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
        ///Gets the contract's `TaskChallengedSuccessfully` event
        pub fn task_challenged_successfully_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TaskChallengedSuccessfullyFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `TaskChallengedUnsuccessfully` event
        pub fn task_challenged_unsuccessfully_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TaskChallengedUnsuccessfullyFilter,
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
            MangataTaskManagerEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MangataTaskManager<M> {
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
        abi = "NewTaskCreated(uint32,(uint256,uint32,bytes,uint32))"
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
        name = "TaskChallengedSuccessfully",
        abi = "TaskChallengedSuccessfully(uint32,address)"
    )]
    pub struct TaskChallengedSuccessfullyFilter {
        #[ethevent(indexed)]
        pub task_index: u32,
        #[ethevent(indexed)]
        pub challenger: ::ethers::core::types::Address,
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
        name = "TaskChallengedUnsuccessfully",
        abi = "TaskChallengedUnsuccessfully(uint32,address)"
    )]
    pub struct TaskChallengedUnsuccessfullyFilter {
        #[ethevent(indexed)]
        pub task_index: u32,
        #[ethevent(indexed)]
        pub challenger: ::ethers::core::types::Address,
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
    #[ethevent(name = "TaskCompleted", abi = "TaskCompleted(uint32)")]
    pub struct TaskCompletedFilter {
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
        Hash
    )]
    #[ethevent(
        name = "TaskResponded",
        abi = "TaskResponded((uint32,bytes32),(uint32,bytes32))"
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
    pub enum MangataTaskManagerEvents {
        InitializedFilter(InitializedFilter),
        NewTaskCreatedFilter(NewTaskCreatedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PausedFilter(PausedFilter),
        PauserRegistrySetFilter(PauserRegistrySetFilter),
        TaskChallengedSuccessfullyFilter(TaskChallengedSuccessfullyFilter),
        TaskChallengedUnsuccessfullyFilter(TaskChallengedUnsuccessfullyFilter),
        TaskCompletedFilter(TaskCompletedFilter),
        TaskRespondedFilter(TaskRespondedFilter),
        UnpausedFilter(UnpausedFilter),
    }
    impl ::ethers::contract::EthLogDecode for MangataTaskManagerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(MangataTaskManagerEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = NewTaskCreatedFilter::decode_log(log) {
                return Ok(MangataTaskManagerEvents::NewTaskCreatedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(MangataTaskManagerEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(MangataTaskManagerEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = PauserRegistrySetFilter::decode_log(log) {
                return Ok(MangataTaskManagerEvents::PauserRegistrySetFilter(decoded));
            }
            if let Ok(decoded) = TaskChallengedSuccessfullyFilter::decode_log(log) {
                return Ok(
                    MangataTaskManagerEvents::TaskChallengedSuccessfullyFilter(decoded),
                );
            }
            if let Ok(decoded) = TaskChallengedUnsuccessfullyFilter::decode_log(log) {
                return Ok(
                    MangataTaskManagerEvents::TaskChallengedUnsuccessfullyFilter(decoded),
                );
            }
            if let Ok(decoded) = TaskCompletedFilter::decode_log(log) {
                return Ok(MangataTaskManagerEvents::TaskCompletedFilter(decoded));
            }
            if let Ok(decoded) = TaskRespondedFilter::decode_log(log) {
                return Ok(MangataTaskManagerEvents::TaskRespondedFilter(decoded));
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(MangataTaskManagerEvents::UnpausedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for MangataTaskManagerEvents {
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
                Self::TaskChallengedSuccessfullyFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TaskChallengedUnsuccessfullyFilter(element) => {
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
    impl ::core::convert::From<InitializedFilter> for MangataTaskManagerEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<NewTaskCreatedFilter> for MangataTaskManagerEvents {
        fn from(value: NewTaskCreatedFilter) -> Self {
            Self::NewTaskCreatedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for MangataTaskManagerEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<PausedFilter> for MangataTaskManagerEvents {
        fn from(value: PausedFilter) -> Self {
            Self::PausedFilter(value)
        }
    }
    impl ::core::convert::From<PauserRegistrySetFilter> for MangataTaskManagerEvents {
        fn from(value: PauserRegistrySetFilter) -> Self {
            Self::PauserRegistrySetFilter(value)
        }
    }
    impl ::core::convert::From<TaskChallengedSuccessfullyFilter>
    for MangataTaskManagerEvents {
        fn from(value: TaskChallengedSuccessfullyFilter) -> Self {
            Self::TaskChallengedSuccessfullyFilter(value)
        }
    }
    impl ::core::convert::From<TaskChallengedUnsuccessfullyFilter>
    for MangataTaskManagerEvents {
        fn from(value: TaskChallengedUnsuccessfullyFilter) -> Self {
            Self::TaskChallengedUnsuccessfullyFilter(value)
        }
    }
    impl ::core::convert::From<TaskCompletedFilter> for MangataTaskManagerEvents {
        fn from(value: TaskCompletedFilter) -> Self {
            Self::TaskCompletedFilter(value)
        }
    }
    impl ::core::convert::From<TaskRespondedFilter> for MangataTaskManagerEvents {
        fn from(value: TaskRespondedFilter) -> Self {
            Self::TaskRespondedFilter(value)
        }
    }
    impl ::core::convert::From<UnpausedFilter> for MangataTaskManagerEvents {
        fn from(value: UnpausedFilter) -> Self {
            Self::UnpausedFilter(value)
        }
    }
    ///Container type for all input parameters for the `TASK_CHALLENGE_WINDOW_BLOCK` function with signature `TASK_CHALLENGE_WINDOW_BLOCK()` and selector `0xf63c5bab`
    #[derive(
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
        name = "TASK_CHALLENGE_WINDOW_BLOCK",
        abi = "TASK_CHALLENGE_WINDOW_BLOCK()"
    )]
    pub struct TaskChallengeWindowBlockCall;
    ///Container type for all input parameters for the `TASK_RESPONSE_WINDOW_BLOCK` function with signature `TASK_RESPONSE_WINDOW_BLOCK()` and selector `0x1ad43189`
    #[derive(
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
    #[ethcall(name = "TASK_RESPONSE_WINDOW_BLOCK", abi = "TASK_RESPONSE_WINDOW_BLOCK()")]
    pub struct TaskResponseWindowBlockCall;
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
    ///Container type for all input parameters for the `blsPubkeyRegistry` function with signature `blsPubkeyRegistry()` and selector `0x3561deb1`
    #[derive(
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
    #[ethcall(name = "blsPubkeyRegistry", abi = "blsPubkeyRegistry()")]
    pub struct BlsPubkeyRegistryCall;
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
        Hash
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
    ///Container type for all input parameters for the `getTaskChallangeWindowBlock` function with signature `getTaskChallangeWindowBlock()` and selector `0xe3b5ee04`
    #[derive(
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
        name = "getTaskChallangeWindowBlock",
        abi = "getTaskChallangeWindowBlock()"
    )]
    pub struct GetTaskChallangeWindowBlockCall;
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
    ///Container type for all input parameters for the `raiseAndResolveChallenge` function with signature `raiseAndResolveChallenge((uint256,uint32,bytes,uint32),(uint32,bytes32),(uint32,bytes32),(uint256,uint256)[])` and selector `0x3cd27f64`
    #[derive(
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
        name = "raiseAndResolveChallenge",
        abi = "raiseAndResolveChallenge((uint256,uint32,bytes,uint32),(uint32,bytes32),(uint32,bytes32),(uint256,uint256)[])"
    )]
    pub struct RaiseAndResolveChallengeCall {
        pub task: Task,
        pub task_response: TaskResponse,
        pub task_response_metadata: TaskResponseMetadata,
        pub pubkeys_of_non_signing_operators: ::std::vec::Vec<G1Point>,
    }
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
    ///Container type for all input parameters for the `respondToTask` function with signature `respondToTask((uint256,uint32,bytes,uint32),(uint32,bytes32),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))` and selector `0x277cb995`
    #[derive(
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
        abi = "respondToTask((uint256,uint32,bytes,uint32),(uint32,bytes32),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))"
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
    ///Container type for all input parameters for the `taskSuccesfullyChallenged` function with signature `taskSuccesfullyChallenged(uint32)` and selector `0x5decc3f5`
    #[derive(
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
        name = "taskSuccesfullyChallenged",
        abi = "taskSuccesfullyChallenged(uint32)"
    )]
    pub struct TaskSuccesfullyChallengedCall(pub u32);
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
    pub enum MangataTaskManagerCalls {
        TaskChallengeWindowBlock(TaskChallengeWindowBlockCall),
        TaskResponseWindowBlock(TaskResponseWindowBlockCall),
        Aggregator(AggregatorCall),
        AllTaskHashes(AllTaskHashesCall),
        AllTaskResponses(AllTaskResponsesCall),
        BlsPubkeyRegistry(BlsPubkeyRegistryCall),
        CheckSignatures(CheckSignaturesCall),
        CreateNewTask(CreateNewTaskCall),
        Generator(GeneratorCall),
        GetCheckSignaturesIndices(GetCheckSignaturesIndicesCall),
        GetOperatorState(GetOperatorStateCall),
        GetOperatorStateWithRegistryCoordinatorAndOperatorId(
            GetOperatorStateWithRegistryCoordinatorAndOperatorIdCall,
        ),
        GetTaskChallangeWindowBlock(GetTaskChallangeWindowBlockCall),
        GetTaskResponseWindowBlock(GetTaskResponseWindowBlockCall),
        Initialize(InitializeCall),
        LatestTaskNum(LatestTaskNumCall),
        Owner(OwnerCall),
        Pause(PauseCall),
        PauseAll(PauseAllCall),
        PausedWithIndex(PausedWithIndexCall),
        Paused(PausedCall),
        PauserRegistry(PauserRegistryCall),
        RaiseAndResolveChallenge(RaiseAndResolveChallengeCall),
        RegistryCoordinator(RegistryCoordinatorCall),
        RenounceOwnership(RenounceOwnershipCall),
        RespondToTask(RespondToTaskCall),
        SetPauserRegistry(SetPauserRegistryCall),
        StakeRegistry(StakeRegistryCall),
        TaskNumber(TaskNumberCall),
        TaskSuccesfullyChallenged(TaskSuccesfullyChallengedCall),
        TransferOwnership(TransferOwnershipCall),
        TrySignatureAndApkVerification(TrySignatureAndApkVerificationCall),
        Unpause(UnpauseCall),
    }
    impl ::ethers::core::abi::AbiDecode for MangataTaskManagerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <TaskChallengeWindowBlockCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TaskChallengeWindowBlock(decoded));
            }
            if let Ok(decoded) = <TaskResponseWindowBlockCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TaskResponseWindowBlock(decoded));
            }
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
            if let Ok(decoded) = <BlsPubkeyRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BlsPubkeyRegistry(decoded));
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
            if let Ok(decoded) = <GetTaskChallangeWindowBlockCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTaskChallangeWindowBlock(decoded));
            }
            if let Ok(decoded) = <GetTaskResponseWindowBlockCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTaskResponseWindowBlock(decoded));
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
            if let Ok(decoded) = <RaiseAndResolveChallengeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RaiseAndResolveChallenge(decoded));
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
            if let Ok(decoded) = <StakeRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StakeRegistry(decoded));
            }
            if let Ok(decoded) = <TaskNumberCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TaskNumber(decoded));
            }
            if let Ok(decoded) = <TaskSuccesfullyChallengedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TaskSuccesfullyChallenged(decoded));
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
    impl ::ethers::core::abi::AbiEncode for MangataTaskManagerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::TaskChallengeWindowBlock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TaskResponseWindowBlock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Aggregator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AllTaskHashes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AllTaskResponses(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BlsPubkeyRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CheckSignatures(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateNewTask(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Generator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetCheckSignaturesIndices(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOperatorState(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOperatorStateWithRegistryCoordinatorAndOperatorId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTaskChallangeWindowBlock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTaskResponseWindowBlock(element) => {
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
                Self::RaiseAndResolveChallenge(element) => {
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
                Self::StakeRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TaskNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TaskSuccesfullyChallenged(element) => {
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
    impl ::core::fmt::Display for MangataTaskManagerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::TaskChallengeWindowBlock(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TaskResponseWindowBlock(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Aggregator(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllTaskHashes(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllTaskResponses(element) => ::core::fmt::Display::fmt(element, f),
                Self::BlsPubkeyRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckSignatures(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateNewTask(element) => ::core::fmt::Display::fmt(element, f),
                Self::Generator(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCheckSignaturesIndices(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetOperatorState(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOperatorStateWithRegistryCoordinatorAndOperatorId(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTaskChallangeWindowBlock(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTaskResponseWindowBlock(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestTaskNum(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pause(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauseAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::PausedWithIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::Paused(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauserRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::RaiseAndResolveChallenge(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RegistryCoordinator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::RespondToTask(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPauserRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::TaskNumber(element) => ::core::fmt::Display::fmt(element, f),
                Self::TaskSuccesfullyChallenged(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::TrySignatureAndApkVerification(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Unpause(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<TaskChallengeWindowBlockCall>
    for MangataTaskManagerCalls {
        fn from(value: TaskChallengeWindowBlockCall) -> Self {
            Self::TaskChallengeWindowBlock(value)
        }
    }
    impl ::core::convert::From<TaskResponseWindowBlockCall> for MangataTaskManagerCalls {
        fn from(value: TaskResponseWindowBlockCall) -> Self {
            Self::TaskResponseWindowBlock(value)
        }
    }
    impl ::core::convert::From<AggregatorCall> for MangataTaskManagerCalls {
        fn from(value: AggregatorCall) -> Self {
            Self::Aggregator(value)
        }
    }
    impl ::core::convert::From<AllTaskHashesCall> for MangataTaskManagerCalls {
        fn from(value: AllTaskHashesCall) -> Self {
            Self::AllTaskHashes(value)
        }
    }
    impl ::core::convert::From<AllTaskResponsesCall> for MangataTaskManagerCalls {
        fn from(value: AllTaskResponsesCall) -> Self {
            Self::AllTaskResponses(value)
        }
    }
    impl ::core::convert::From<BlsPubkeyRegistryCall> for MangataTaskManagerCalls {
        fn from(value: BlsPubkeyRegistryCall) -> Self {
            Self::BlsPubkeyRegistry(value)
        }
    }
    impl ::core::convert::From<CheckSignaturesCall> for MangataTaskManagerCalls {
        fn from(value: CheckSignaturesCall) -> Self {
            Self::CheckSignatures(value)
        }
    }
    impl ::core::convert::From<CreateNewTaskCall> for MangataTaskManagerCalls {
        fn from(value: CreateNewTaskCall) -> Self {
            Self::CreateNewTask(value)
        }
    }
    impl ::core::convert::From<GeneratorCall> for MangataTaskManagerCalls {
        fn from(value: GeneratorCall) -> Self {
            Self::Generator(value)
        }
    }
    impl ::core::convert::From<GetCheckSignaturesIndicesCall>
    for MangataTaskManagerCalls {
        fn from(value: GetCheckSignaturesIndicesCall) -> Self {
            Self::GetCheckSignaturesIndices(value)
        }
    }
    impl ::core::convert::From<GetOperatorStateCall> for MangataTaskManagerCalls {
        fn from(value: GetOperatorStateCall) -> Self {
            Self::GetOperatorState(value)
        }
    }
    impl ::core::convert::From<GetOperatorStateWithRegistryCoordinatorAndOperatorIdCall>
    for MangataTaskManagerCalls {
        fn from(
            value: GetOperatorStateWithRegistryCoordinatorAndOperatorIdCall,
        ) -> Self {
            Self::GetOperatorStateWithRegistryCoordinatorAndOperatorId(value)
        }
    }
    impl ::core::convert::From<GetTaskChallangeWindowBlockCall>
    for MangataTaskManagerCalls {
        fn from(value: GetTaskChallangeWindowBlockCall) -> Self {
            Self::GetTaskChallangeWindowBlock(value)
        }
    }
    impl ::core::convert::From<GetTaskResponseWindowBlockCall>
    for MangataTaskManagerCalls {
        fn from(value: GetTaskResponseWindowBlockCall) -> Self {
            Self::GetTaskResponseWindowBlock(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for MangataTaskManagerCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<LatestTaskNumCall> for MangataTaskManagerCalls {
        fn from(value: LatestTaskNumCall) -> Self {
            Self::LatestTaskNum(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for MangataTaskManagerCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PauseCall> for MangataTaskManagerCalls {
        fn from(value: PauseCall) -> Self {
            Self::Pause(value)
        }
    }
    impl ::core::convert::From<PauseAllCall> for MangataTaskManagerCalls {
        fn from(value: PauseAllCall) -> Self {
            Self::PauseAll(value)
        }
    }
    impl ::core::convert::From<PausedWithIndexCall> for MangataTaskManagerCalls {
        fn from(value: PausedWithIndexCall) -> Self {
            Self::PausedWithIndex(value)
        }
    }
    impl ::core::convert::From<PausedCall> for MangataTaskManagerCalls {
        fn from(value: PausedCall) -> Self {
            Self::Paused(value)
        }
    }
    impl ::core::convert::From<PauserRegistryCall> for MangataTaskManagerCalls {
        fn from(value: PauserRegistryCall) -> Self {
            Self::PauserRegistry(value)
        }
    }
    impl ::core::convert::From<RaiseAndResolveChallengeCall>
    for MangataTaskManagerCalls {
        fn from(value: RaiseAndResolveChallengeCall) -> Self {
            Self::RaiseAndResolveChallenge(value)
        }
    }
    impl ::core::convert::From<RegistryCoordinatorCall> for MangataTaskManagerCalls {
        fn from(value: RegistryCoordinatorCall) -> Self {
            Self::RegistryCoordinator(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for MangataTaskManagerCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<RespondToTaskCall> for MangataTaskManagerCalls {
        fn from(value: RespondToTaskCall) -> Self {
            Self::RespondToTask(value)
        }
    }
    impl ::core::convert::From<SetPauserRegistryCall> for MangataTaskManagerCalls {
        fn from(value: SetPauserRegistryCall) -> Self {
            Self::SetPauserRegistry(value)
        }
    }
    impl ::core::convert::From<StakeRegistryCall> for MangataTaskManagerCalls {
        fn from(value: StakeRegistryCall) -> Self {
            Self::StakeRegistry(value)
        }
    }
    impl ::core::convert::From<TaskNumberCall> for MangataTaskManagerCalls {
        fn from(value: TaskNumberCall) -> Self {
            Self::TaskNumber(value)
        }
    }
    impl ::core::convert::From<TaskSuccesfullyChallengedCall>
    for MangataTaskManagerCalls {
        fn from(value: TaskSuccesfullyChallengedCall) -> Self {
            Self::TaskSuccesfullyChallenged(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for MangataTaskManagerCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<TrySignatureAndApkVerificationCall>
    for MangataTaskManagerCalls {
        fn from(value: TrySignatureAndApkVerificationCall) -> Self {
            Self::TrySignatureAndApkVerification(value)
        }
    }
    impl ::core::convert::From<UnpauseCall> for MangataTaskManagerCalls {
        fn from(value: UnpauseCall) -> Self {
            Self::Unpause(value)
        }
    }
    ///Container type for all return fields from the `TASK_CHALLENGE_WINDOW_BLOCK` function with signature `TASK_CHALLENGE_WINDOW_BLOCK()` and selector `0xf63c5bab`
    #[derive(
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
    pub struct TaskChallengeWindowBlockReturn(pub u32);
    ///Container type for all return fields from the `TASK_RESPONSE_WINDOW_BLOCK` function with signature `TASK_RESPONSE_WINDOW_BLOCK()` and selector `0x1ad43189`
    #[derive(
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
    pub struct TaskResponseWindowBlockReturn(pub u32);
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
    ///Container type for all return fields from the `blsPubkeyRegistry` function with signature `blsPubkeyRegistry()` and selector `0x3561deb1`
    #[derive(
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
    pub struct BlsPubkeyRegistryReturn(pub ::ethers::core::types::Address);
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
        Hash
    )]
    pub struct CheckSignaturesReturn(pub QuorumStakeTotals, pub [u8; 32]);
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
    ///Container type for all return fields from the `getTaskChallangeWindowBlock` function with signature `getTaskChallangeWindowBlock()` and selector `0xe3b5ee04`
    #[derive(
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
    pub struct GetTaskChallangeWindowBlockReturn(pub u32);
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
    ///Container type for all return fields from the `taskSuccesfullyChallenged` function with signature `taskSuccesfullyChallenged(uint32)` and selector `0x5decc3f5`
    #[derive(
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
    pub struct TaskSuccesfullyChallengedReturn(pub bool);
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
    ///`Operator(bytes32,uint96)`
    #[derive(
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
        pub operator_id: [u8; 32],
        pub stake: u128,
    }
}
