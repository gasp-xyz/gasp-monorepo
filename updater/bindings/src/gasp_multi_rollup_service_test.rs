pub use gasp_multi_rollup_service_test::*;
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
pub mod gasp_multi_rollup_service_test {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("IS_TEST"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("IS_TEST"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("excludeArtifacts"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("excludeArtifacts"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("excludedArtifacts_",),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::String,
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("excludeContracts"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("excludeContracts"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("excludedContracts_",),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("excludeSelectors"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("excludeSelectors"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("excludedSelectors_",),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                        ::ethers::core::abi::ethabi::ParamType::Array(
                                            ::std::boxed::Box::new(
                                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                                    4usize
                                                ),
                                            ),
                                        ),
                                    ],),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct StdInvariant.FuzzSelector[]",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("excludeSenders"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("excludeSenders"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("excludedSenders_"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("failed"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("failed"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("targetArtifactSelectors"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("targetArtifactSelectors",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("targetedArtifactSelectors_",),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::String,
                                        ::ethers::core::abi::ethabi::ParamType::Array(
                                            ::std::boxed::Box::new(
                                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                                    4usize
                                                ),
                                            ),
                                        ),
                                    ],),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct StdInvariant.FuzzArtifactSelector[]",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("targetArtifacts"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("targetArtifacts"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("targetedArtifacts_",),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::String,
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("targetContracts"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("targetContracts"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("targetedContracts_",),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("targetInterfaces"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("targetInterfaces"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("targetedInterfaces_",),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                        ::ethers::core::abi::ethabi::ParamType::Array(
                                            ::std::boxed::Box::new(
                                                ::ethers::core::abi::ethabi::ParamType::String,
                                            ),
                                        ),
                                    ],),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct StdInvariant.FuzzInterface[]",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("targetSelectors"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("targetSelectors"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("targetedSelectors_",),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                        ::ethers::core::abi::ethabi::ParamType::Array(
                                            ::std::boxed::Box::new(
                                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                                    4usize
                                                ),
                                            ),
                                        ),
                                    ],),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct StdInvariant.FuzzSelector[]",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("targetSenders"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("targetSenders"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("targetedSenders_"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("testGasp"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("testGasp"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("EigenOpUpdateProcessed"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("EigenOpUpdateProcessed",),
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
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EigenRdUpdateProcessed"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("EigenRdUpdateProcessed",),
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
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EigenReinitProcessed"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("EigenReinitProcessed",),
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
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RolldownTargetUpdated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("RolldownTargetUpdated",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("rolldownAddress"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("log"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_address"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("log_address"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_array"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_array"),
                            inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("val"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                ),
                                indexed: false,
                            },],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_array"),
                            inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("val"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    ),
                                ),
                                indexed: false,
                            },],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_array"),
                            inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("val"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                    ),
                                ),
                                indexed: false,
                            },],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_bytes"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("log_bytes"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_bytes32"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("log_bytes32"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_int"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("log_int"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_address"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("log_named_address"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("key"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("val"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_array"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_array"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_array"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                        ),
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_array"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_bytes"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("log_named_bytes"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("key"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("val"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_bytes32"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("log_named_bytes32"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("key"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("val"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_decimal_int"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("log_named_decimal_int",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("key"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("val"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("decimals"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_decimal_uint"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("log_named_decimal_uint",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("key"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("val"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("decimals"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_int"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("log_named_int"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("key"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("val"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_string"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("log_named_string"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("key"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("val"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_uint"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("log_named_uint"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("key"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("val"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_string"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("log_string"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_uint"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("log_uint"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("logs"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("logs"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static GASPMULTIROLLUPSERVICETEST_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\x0C\x80T`\x01`\xFF\x19\x91\x82\x16\x81\x17\x90\x92U`\x1F\x80T\x90\x91\x16\x90\x91\x17\x90U4\x80\x15a\0-W`\0\x80\xFD[Pa\x0F\xFD\x80a\0=`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xCFW`\x005`\xE0\x1C\x80c\x85\"l\x81\x11a\0\x8CW\x80c\xB5P\x8A\xA9\x11a\0fW\x80c\xB5P\x8A\xA9\x14a\x01hW\x80c\xBAAO\xA6\x14a\x01pW\x80c\xE2\x0C\x9Fq\x14a\x01\x88W\x80c\xFAv&\xD4\x14a\x01\x90W`\0\x80\xFD[\x80c\x85\"l\x81\x14a\x016W\x80c\x91j\x17\xC6\x14a\x01KW\x80c\xB0FO\xDC\x14a\x01`W`\0\x80\xFD[\x80c\x1E\xD7\x83\x1C\x14a\0\xD4W\x80c*\xDE8\x80\x14a\0\xF2W\x80c>^<#\x14a\x01\x07W\x80c?r\x86\xF4\x14a\x01\x0FW\x80cT\xF2\xD3T\x14a\x01\x17W\x80cf\xD9\xA9\xA0\x14a\x01!W[`\0\x80\xFD[a\0\xDCa\x01\x9DV[`@Qa\0\xE9\x91\x90a\x0B\x8CV[`@Q\x80\x91\x03\x90\xF3[a\0\xFAa\x01\xFFV[`@Qa\0\xE9\x91\x90a\x0C&V[a\0\xDCa\x03AV[a\0\xDCa\x03\xA1V[a\x01\x1Fa\x04\x01V[\0[a\x01)a\x05\x03V[`@Qa\0\xE9\x91\x90a\r+V[a\x01>a\x06pV[`@Qa\0\xE9\x91\x90a\r\xB0V[a\x01Sa\x07@V[`@Qa\0\xE9\x91\x90a\x0E\x12V[a\x01Sa\x08&V[a\x01>a\t\x0CV[a\x01xa\t\xDCV[`@Q\x90\x15\x15\x81R` \x01a\0\xE9V[a\0\xDCa\n\x80V[`\x1FTa\x01x\x90`\xFF\x16\x81V[```\x16\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x01\xF5W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x01\xD7W[PPPPP\x90P\x90V[```\x1E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x038W`\0\x84\x81R` \x80\x82 `@\x80Q\x80\x82\x01\x82R`\x02\x87\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x95\x91\x94\x86\x81\x01\x94\x91\x92\x90\x84\x01[\x82\x82\x10\x15a\x03!W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x02\x94\x90a\x0E\x81V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02\xC0\x90a\x0E\x81V[\x80\x15a\x03\rW\x80`\x1F\x10a\x02\xE2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03\rV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02\xF0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x02uV[PPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x02#V[PPPP\x90P\x90V[```\x18\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x01\xF5W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x01\xD7WPPPPP\x90P\x90V[```\x17\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x01\xF5W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x01\xD7WPPPPP\x90P\x90V[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91` \x82\x01\x81\x806\x837\x01\x90PP\x90P`\0`\xF8\x1B\x81`\0\x81Q\x81\x10a\x04:Wa\x04:a\x0E\xBBV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`@\x80Q`\0\x80\x82Ra\x01 \x82\x01\x83R`\x01` \x80\x84\x01\x91\x82R`\x1E\x84\x86\x01R``\x84\x01\x83\x90R`\x80\x84\x01\x83\x90R`\xA0\x84\x01\x86\x90R`B`\xC0\x85\x01R`\xE0\x84\x01\x84\x90Ra\x01\0\x84\x01\x83\x90R\x93Q\x92\x93\x90\x92a\x04\xAE\x91\x84\x91\x01a\x0E\xD1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90Pa\x04\xC8\x81a\n\xE0V[`\0\x82`@Q` \x01a\x04\xDB\x91\x90a\x0E\xD1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa\x04\xFC\x81a\x0B&V[PPPPPV[```\x1B\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x038W\x83\x82\x90`\0R` `\0 \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81`\0\x82\x01\x80Ta\x05Z\x90a\x0E\x81V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\x86\x90a\x0E\x81V[\x80\x15a\x05\xD3W\x80`\x1F\x10a\x05\xA8Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\xD3V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\xB6W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x06XW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x06\x1AW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x05'V[```\x1A\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x038W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x06\xB3\x90a\x0E\x81V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\xDF\x90a\x0E\x81V[\x80\x15a\x07,W\x80`\x1F\x10a\x07\x01Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07,V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07\x0FW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x06\x94V[```\x1D\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x038W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x08\x0EW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x07\xD0W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x07dV[```\x1C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x038W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x08\xF4W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x08\xB6W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x08JV[```\x19\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x038W\x83\x82\x90`\0R` `\0 \x01\x80Ta\tO\x90a\x0E\x81V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\t{\x90a\x0E\x81V[\x80\x15a\t\xC8W\x80`\x1F\x10a\t\x9DWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t\xC8V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t\xABW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\t0V[`\x08T`\0\x90`\xFF\x16\x15a\t\xF4WP`\x08T`\xFF\x16\x90V[`@Qc\x06g\xF9\xD7`\xE4\x1B\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\x04\x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B`$\x83\x01R`\0\x91cf\x7F\x9Dp\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\nUW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\ny\x91\x90a\x0F\x94V[\x14\x15\x90P\x90V[```\x15\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x01\xF5W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x01\xD7WPPPPP\x90P\x90V[a\x0B#\x81`@Q`$\x01a\n\xF4\x91\x90a\x0F\xADV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\x05\xF3\xBF\xAB`\xE1\x1B\x17\x90Ra\x0BgV[PV[a\x0B#\x81`@Q`$\x01a\x0B<\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c'\xB7\xCF\x85`\xE0\x1B\x17\x90R[a\x0B#\x81`\0jconsole.log\x90P`\0\x80\x83Q` \x85\x01\x84Z\xFAPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x0B\xCDW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x0B\xA8V[P\x90\x96\x95PPPPPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x0B\xFFW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x0B\xE3V[\x81\x81\x11\x15a\x0C\x11W`\0` \x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90`\x05\x81\x81\x1B\x87\x01\x84\x01\x88\x86\x01\x87\x80[\x85\x81\x10\x15a\x0C\xD6W`?\x19\x8B\x85\x03\x01\x87R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x89\x01Q\x89\x85\x01\x89\x90R\x80Q\x89\x86\x01\x81\x90R\x90\x8A\x01\x90``\x81\x88\x1B\x87\x01\x81\x01\x91\x90\x87\x01\x90\x85[\x81\x81\x10\x15a\x0C\xC0W`_\x19\x89\x85\x03\x01\x83Ra\x0C\xAE\x84\x86Qa\x0B\xD9V[\x94\x8E\x01\x94\x93P\x91\x8D\x01\x91`\x01\x01a\x0C\x92V[PPP\x97\x8A\x01\x97\x94PP\x91\x88\x01\x91`\x01\x01a\x0CMV[P\x91\x9A\x99PPPPPPPPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\r W\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x0C\xFAV[P\x94\x95\x94PPPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P\x82\x86\x01\x91P\x82\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0[\x83\x81\x10\x15a\r\xA2W\x88\x83\x03`?\x19\x01\x85R\x81Q\x80Q\x87\x85Ra\rv\x88\x86\x01\x82a\x0B\xD9V[\x91\x89\x01Q\x85\x83\x03\x86\x8B\x01R\x91\x90Pa\r\x8E\x81\x83a\x0C\xE6V[\x96\x89\x01\x96\x94PPP\x90\x86\x01\x90`\x01\x01a\rRV[P\x90\x98\x97PPPPPPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P\x83\x87\x01`\0[\x82\x81\x10\x15a\x0E\x05W`?\x19\x88\x86\x03\x01\x84Ra\r\xF3\x85\x83Qa\x0B\xD9V[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a\r\xD7V[P\x92\x97\x96PPPPPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P\x82\x86\x01\x91P\x82\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0[\x83\x81\x10\x15a\r\xA2W\x88\x83\x03`?\x19\x01\x85R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x87\x01Q\x87\x84\x01\x87\x90Ra\x0En\x87\x85\x01\x82a\x0C\xE6V[\x95\x88\x01\x95\x93PP\x90\x86\x01\x90`\x01\x01a\x0E9V[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0E\x95W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0E\xB5WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[` \x81Rc\xFF\xFF\xFF\xFF\x82Q\x16` \x82\x01R`\0` \x83\x01Qa\x0E\xFB`@\x84\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P`@\x83\x01Qc\xFF\xFF\xFF\xFF\x81\x16``\x84\x01RP``\x83\x01Qc\xFF\xFF\xFF\xFF\x81\x16`\x80\x84\x01RP`\x80\x83\x01Qa\x01\0\x80`\xA0\x85\x01Ra\x0F<a\x01 \x85\x01\x83a\x0B\xD9V[\x91P`\xA0\x85\x01Qa\x0FU`\xC0\x86\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P`\xC0\x85\x01Q\x84\x83\x03`\x1F\x19\x01`\xE0\x86\x01Ra\x0Fq\x83\x82a\x0B\xD9V[\x92PP`\xE0\x85\x01Qa\x0F\x8A\x82\x86\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P\x90\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x0F\xA6W`\0\x80\xFD[PQ\x91\x90PV[` \x81R`\0a\x0F\xC0` \x83\x01\x84a\x0B\xD9V[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \xEE\xE5\xD9\xEDGp_\xB2\xF7t\xF2\xC3\xD2/\x14\xE6\xA0\xF6@\xEA|\x19\xED\xB7\x80\xCDvAN9\xB3EdsolcC\0\x08\r\x003";
    /// The bytecode of the contract.
    pub static GASPMULTIROLLUPSERVICETEST_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xCFW`\x005`\xE0\x1C\x80c\x85\"l\x81\x11a\0\x8CW\x80c\xB5P\x8A\xA9\x11a\0fW\x80c\xB5P\x8A\xA9\x14a\x01hW\x80c\xBAAO\xA6\x14a\x01pW\x80c\xE2\x0C\x9Fq\x14a\x01\x88W\x80c\xFAv&\xD4\x14a\x01\x90W`\0\x80\xFD[\x80c\x85\"l\x81\x14a\x016W\x80c\x91j\x17\xC6\x14a\x01KW\x80c\xB0FO\xDC\x14a\x01`W`\0\x80\xFD[\x80c\x1E\xD7\x83\x1C\x14a\0\xD4W\x80c*\xDE8\x80\x14a\0\xF2W\x80c>^<#\x14a\x01\x07W\x80c?r\x86\xF4\x14a\x01\x0FW\x80cT\xF2\xD3T\x14a\x01\x17W\x80cf\xD9\xA9\xA0\x14a\x01!W[`\0\x80\xFD[a\0\xDCa\x01\x9DV[`@Qa\0\xE9\x91\x90a\x0B\x8CV[`@Q\x80\x91\x03\x90\xF3[a\0\xFAa\x01\xFFV[`@Qa\0\xE9\x91\x90a\x0C&V[a\0\xDCa\x03AV[a\0\xDCa\x03\xA1V[a\x01\x1Fa\x04\x01V[\0[a\x01)a\x05\x03V[`@Qa\0\xE9\x91\x90a\r+V[a\x01>a\x06pV[`@Qa\0\xE9\x91\x90a\r\xB0V[a\x01Sa\x07@V[`@Qa\0\xE9\x91\x90a\x0E\x12V[a\x01Sa\x08&V[a\x01>a\t\x0CV[a\x01xa\t\xDCV[`@Q\x90\x15\x15\x81R` \x01a\0\xE9V[a\0\xDCa\n\x80V[`\x1FTa\x01x\x90`\xFF\x16\x81V[```\x16\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x01\xF5W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x01\xD7W[PPPPP\x90P\x90V[```\x1E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x038W`\0\x84\x81R` \x80\x82 `@\x80Q\x80\x82\x01\x82R`\x02\x87\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x95\x91\x94\x86\x81\x01\x94\x91\x92\x90\x84\x01[\x82\x82\x10\x15a\x03!W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x02\x94\x90a\x0E\x81V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02\xC0\x90a\x0E\x81V[\x80\x15a\x03\rW\x80`\x1F\x10a\x02\xE2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03\rV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02\xF0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x02uV[PPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x02#V[PPPP\x90P\x90V[```\x18\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x01\xF5W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x01\xD7WPPPPP\x90P\x90V[```\x17\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x01\xF5W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x01\xD7WPPPPP\x90P\x90V[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91` \x82\x01\x81\x806\x837\x01\x90PP\x90P`\0`\xF8\x1B\x81`\0\x81Q\x81\x10a\x04:Wa\x04:a\x0E\xBBV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP`@\x80Q`\0\x80\x82Ra\x01 \x82\x01\x83R`\x01` \x80\x84\x01\x91\x82R`\x1E\x84\x86\x01R``\x84\x01\x83\x90R`\x80\x84\x01\x83\x90R`\xA0\x84\x01\x86\x90R`B`\xC0\x85\x01R`\xE0\x84\x01\x84\x90Ra\x01\0\x84\x01\x83\x90R\x93Q\x92\x93\x90\x92a\x04\xAE\x91\x84\x91\x01a\x0E\xD1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90Pa\x04\xC8\x81a\n\xE0V[`\0\x82`@Q` \x01a\x04\xDB\x91\x90a\x0E\xD1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa\x04\xFC\x81a\x0B&V[PPPPPV[```\x1B\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x038W\x83\x82\x90`\0R` `\0 \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81`\0\x82\x01\x80Ta\x05Z\x90a\x0E\x81V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\x86\x90a\x0E\x81V[\x80\x15a\x05\xD3W\x80`\x1F\x10a\x05\xA8Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\xD3V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\xB6W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x06XW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x06\x1AW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x05'V[```\x1A\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x038W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x06\xB3\x90a\x0E\x81V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\xDF\x90a\x0E\x81V[\x80\x15a\x07,W\x80`\x1F\x10a\x07\x01Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07,V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07\x0FW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x06\x94V[```\x1D\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x038W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x08\x0EW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x07\xD0W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x07dV[```\x1C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x038W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x08\xF4W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x08\xB6W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x08JV[```\x19\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x038W\x83\x82\x90`\0R` `\0 \x01\x80Ta\tO\x90a\x0E\x81V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\t{\x90a\x0E\x81V[\x80\x15a\t\xC8W\x80`\x1F\x10a\t\x9DWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t\xC8V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t\xABW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\t0V[`\x08T`\0\x90`\xFF\x16\x15a\t\xF4WP`\x08T`\xFF\x16\x90V[`@Qc\x06g\xF9\xD7`\xE4\x1B\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\x04\x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B`$\x83\x01R`\0\x91cf\x7F\x9Dp\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\nUW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\ny\x91\x90a\x0F\x94V[\x14\x15\x90P\x90V[```\x15\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x01\xF5W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x01\xD7WPPPPP\x90P\x90V[a\x0B#\x81`@Q`$\x01a\n\xF4\x91\x90a\x0F\xADV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\x05\xF3\xBF\xAB`\xE1\x1B\x17\x90Ra\x0BgV[PV[a\x0B#\x81`@Q`$\x01a\x0B<\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c'\xB7\xCF\x85`\xE0\x1B\x17\x90R[a\x0B#\x81`\0jconsole.log\x90P`\0\x80\x83Q` \x85\x01\x84Z\xFAPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x0B\xCDW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x0B\xA8V[P\x90\x96\x95PPPPPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x0B\xFFW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x0B\xE3V[\x81\x81\x11\x15a\x0C\x11W`\0` \x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90`\x05\x81\x81\x1B\x87\x01\x84\x01\x88\x86\x01\x87\x80[\x85\x81\x10\x15a\x0C\xD6W`?\x19\x8B\x85\x03\x01\x87R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x89\x01Q\x89\x85\x01\x89\x90R\x80Q\x89\x86\x01\x81\x90R\x90\x8A\x01\x90``\x81\x88\x1B\x87\x01\x81\x01\x91\x90\x87\x01\x90\x85[\x81\x81\x10\x15a\x0C\xC0W`_\x19\x89\x85\x03\x01\x83Ra\x0C\xAE\x84\x86Qa\x0B\xD9V[\x94\x8E\x01\x94\x93P\x91\x8D\x01\x91`\x01\x01a\x0C\x92V[PPP\x97\x8A\x01\x97\x94PP\x91\x88\x01\x91`\x01\x01a\x0CMV[P\x91\x9A\x99PPPPPPPPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\r W\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x0C\xFAV[P\x94\x95\x94PPPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P\x82\x86\x01\x91P\x82\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0[\x83\x81\x10\x15a\r\xA2W\x88\x83\x03`?\x19\x01\x85R\x81Q\x80Q\x87\x85Ra\rv\x88\x86\x01\x82a\x0B\xD9V[\x91\x89\x01Q\x85\x83\x03\x86\x8B\x01R\x91\x90Pa\r\x8E\x81\x83a\x0C\xE6V[\x96\x89\x01\x96\x94PPP\x90\x86\x01\x90`\x01\x01a\rRV[P\x90\x98\x97PPPPPPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P\x83\x87\x01`\0[\x82\x81\x10\x15a\x0E\x05W`?\x19\x88\x86\x03\x01\x84Ra\r\xF3\x85\x83Qa\x0B\xD9V[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a\r\xD7V[P\x92\x97\x96PPPPPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P\x82\x86\x01\x91P\x82\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0[\x83\x81\x10\x15a\r\xA2W\x88\x83\x03`?\x19\x01\x85R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x87\x01Q\x87\x84\x01\x87\x90Ra\x0En\x87\x85\x01\x82a\x0C\xE6V[\x95\x88\x01\x95\x93PP\x90\x86\x01\x90`\x01\x01a\x0E9V[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0E\x95W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0E\xB5WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[` \x81Rc\xFF\xFF\xFF\xFF\x82Q\x16` \x82\x01R`\0` \x83\x01Qa\x0E\xFB`@\x84\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P`@\x83\x01Qc\xFF\xFF\xFF\xFF\x81\x16``\x84\x01RP``\x83\x01Qc\xFF\xFF\xFF\xFF\x81\x16`\x80\x84\x01RP`\x80\x83\x01Qa\x01\0\x80`\xA0\x85\x01Ra\x0F<a\x01 \x85\x01\x83a\x0B\xD9V[\x91P`\xA0\x85\x01Qa\x0FU`\xC0\x86\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P`\xC0\x85\x01Q\x84\x83\x03`\x1F\x19\x01`\xE0\x86\x01Ra\x0Fq\x83\x82a\x0B\xD9V[\x92PP`\xE0\x85\x01Qa\x0F\x8A\x82\x86\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P\x90\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x0F\xA6W`\0\x80\xFD[PQ\x91\x90PV[` \x81R`\0a\x0F\xC0` \x83\x01\x84a\x0B\xD9V[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \xEE\xE5\xD9\xEDGp_\xB2\xF7t\xF2\xC3\xD2/\x14\xE6\xA0\xF6@\xEA|\x19\xED\xB7\x80\xCDvAN9\xB3EdsolcC\0\x08\r\x003";
    /// The deployed bytecode of the contract.
    pub static GASPMULTIROLLUPSERVICETEST_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct GaspMultiRollupServiceTest<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for GaspMultiRollupServiceTest<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for GaspMultiRollupServiceTest<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for GaspMultiRollupServiceTest<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for GaspMultiRollupServiceTest<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(GaspMultiRollupServiceTest))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> GaspMultiRollupServiceTest<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                GASPMULTIROLLUPSERVICETEST_ABI.clone(),
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
                GASPMULTIROLLUPSERVICETEST_ABI.clone(),
                GASPMULTIROLLUPSERVICETEST_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `IS_TEST` (0xfa7626d4) function
        pub fn is_test(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([250, 118, 38, 212], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `excludeArtifacts` (0xb5508aa9) function
        pub fn exclude_artifacts(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<::std::string::String>>
        {
            self.0
                .method_hash([181, 80, 138, 169], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `excludeContracts` (0xe20c9f71) function
        pub fn exclude_contracts(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([226, 12, 159, 113], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `excludeSelectors` (0xb0464fdc) function
        pub fn exclude_selectors(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<FuzzSelector>> {
            self.0
                .method_hash([176, 70, 79, 220], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `excludeSenders` (0x1ed7831c) function
        pub fn exclude_senders(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([30, 215, 131, 28], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `failed` (0xba414fa6) function
        pub fn failed(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([186, 65, 79, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `targetArtifactSelectors` (0x66d9a9a0) function
        pub fn target_artifact_selectors(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<FuzzArtifactSelector>>
        {
            self.0
                .method_hash([102, 217, 169, 160], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `targetArtifacts` (0x85226c81) function
        pub fn target_artifacts(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<::std::string::String>>
        {
            self.0
                .method_hash([133, 34, 108, 129], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `targetContracts` (0x3f7286f4) function
        pub fn target_contracts(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([63, 114, 134, 244], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `targetInterfaces` (0x2ade3880) function
        pub fn target_interfaces(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<FuzzInterface>> {
            self.0
                .method_hash([42, 222, 56, 128], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `targetSelectors` (0x916a17c6) function
        pub fn target_selectors(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<FuzzSelector>> {
            self.0
                .method_hash([145, 106, 23, 198], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `targetSenders` (0x3e5e3c23) function
        pub fn target_senders(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([62, 94, 60, 35], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `testGasp` (0x54f2d354) function
        pub fn test_gasp(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([84, 242, 211, 84], ())
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
        ///Gets the contract's `RolldownTargetUpdated` event
        pub fn rolldown_target_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RolldownTargetUpdatedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `log` event
        pub fn log_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogFilter> {
            self.0.event()
        }
        ///Gets the contract's `log_address` event
        pub fn log_address_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogAddressFilter> {
            self.0.event()
        }
        ///Gets the contract's `log_array` event
        pub fn log_array_1_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogArray1Filter> {
            self.0.event()
        }
        ///Gets the contract's `log_array` event
        pub fn log_array_2_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogArray2Filter> {
            self.0.event()
        }
        ///Gets the contract's `log_array` event
        pub fn log_array_3_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogArray3Filter> {
            self.0.event()
        }
        ///Gets the contract's `log_bytes` event
        pub fn log_bytes_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogBytesFilter> {
            self.0.event()
        }
        ///Gets the contract's `log_bytes32` event
        pub fn log_bytes_32_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogBytes32Filter> {
            self.0.event()
        }
        ///Gets the contract's `log_int` event
        pub fn log_int_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogIntFilter> {
            self.0.event()
        }
        ///Gets the contract's `log_named_address` event
        pub fn log_named_address_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogNamedAddressFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `log_named_array` event
        pub fn log_named_array_1_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogNamedArray1Filter>
        {
            self.0.event()
        }
        ///Gets the contract's `log_named_array` event
        pub fn log_named_array_2_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogNamedArray2Filter>
        {
            self.0.event()
        }
        ///Gets the contract's `log_named_array` event
        pub fn log_named_array_3_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogNamedArray3Filter>
        {
            self.0.event()
        }
        ///Gets the contract's `log_named_bytes` event
        pub fn log_named_bytes_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogNamedBytesFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `log_named_bytes32` event
        pub fn log_named_bytes_32_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogNamedBytes32Filter>
        {
            self.0.event()
        }
        ///Gets the contract's `log_named_decimal_int` event
        pub fn log_named_decimal_int_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogNamedDecimalIntFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `log_named_decimal_uint` event
        pub fn log_named_decimal_uint_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogNamedDecimalUintFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `log_named_int` event
        pub fn log_named_int_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogNamedIntFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `log_named_string` event
        pub fn log_named_string_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogNamedStringFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `log_named_uint` event
        pub fn log_named_uint_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogNamedUintFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `log_string` event
        pub fn log_string_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogStringFilter> {
            self.0.event()
        }
        ///Gets the contract's `log_uint` event
        pub fn log_uint_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogUintFilter> {
            self.0.event()
        }
        ///Gets the contract's `logs` event
        pub fn logs_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogsFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            GaspMultiRollupServiceTestEvents,
        > {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for GaspMultiRollupServiceTest<M>
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
    #[ethevent(name = "log", abi = "log(string)")]
    pub struct LogFilter(pub ::std::string::String);
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
    #[ethevent(name = "log_address", abi = "log_address(address)")]
    pub struct LogAddressFilter(pub ::ethers::core::types::Address);
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
    #[ethevent(name = "log_array", abi = "log_array(uint256[])")]
    pub struct LogArray1Filter {
        pub val: ::std::vec::Vec<::ethers::core::types::U256>,
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
    #[ethevent(name = "log_array", abi = "log_array(int256[])")]
    pub struct LogArray2Filter {
        pub val: ::std::vec::Vec<::ethers::core::types::I256>,
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
    #[ethevent(name = "log_array", abi = "log_array(address[])")]
    pub struct LogArray3Filter {
        pub val: ::std::vec::Vec<::ethers::core::types::Address>,
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
    #[ethevent(name = "log_bytes", abi = "log_bytes(bytes)")]
    pub struct LogBytesFilter(pub ::ethers::core::types::Bytes);
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
    #[ethevent(name = "log_bytes32", abi = "log_bytes32(bytes32)")]
    pub struct LogBytes32Filter(pub [u8; 32]);
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
    #[ethevent(name = "log_int", abi = "log_int(int256)")]
    pub struct LogIntFilter(pub ::ethers::core::types::I256);
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
    #[ethevent(name = "log_named_address", abi = "log_named_address(string,address)")]
    pub struct LogNamedAddressFilter {
        pub key: ::std::string::String,
        pub val: ::ethers::core::types::Address,
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
    #[ethevent(name = "log_named_array", abi = "log_named_array(string,uint256[])")]
    pub struct LogNamedArray1Filter {
        pub key: ::std::string::String,
        pub val: ::std::vec::Vec<::ethers::core::types::U256>,
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
    #[ethevent(name = "log_named_array", abi = "log_named_array(string,int256[])")]
    pub struct LogNamedArray2Filter {
        pub key: ::std::string::String,
        pub val: ::std::vec::Vec<::ethers::core::types::I256>,
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
    #[ethevent(name = "log_named_array", abi = "log_named_array(string,address[])")]
    pub struct LogNamedArray3Filter {
        pub key: ::std::string::String,
        pub val: ::std::vec::Vec<::ethers::core::types::Address>,
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
    #[ethevent(name = "log_named_bytes", abi = "log_named_bytes(string,bytes)")]
    pub struct LogNamedBytesFilter {
        pub key: ::std::string::String,
        pub val: ::ethers::core::types::Bytes,
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
    #[ethevent(name = "log_named_bytes32", abi = "log_named_bytes32(string,bytes32)")]
    pub struct LogNamedBytes32Filter {
        pub key: ::std::string::String,
        pub val: [u8; 32],
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
        name = "log_named_decimal_int",
        abi = "log_named_decimal_int(string,int256,uint256)"
    )]
    pub struct LogNamedDecimalIntFilter {
        pub key: ::std::string::String,
        pub val: ::ethers::core::types::I256,
        pub decimals: ::ethers::core::types::U256,
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
        name = "log_named_decimal_uint",
        abi = "log_named_decimal_uint(string,uint256,uint256)"
    )]
    pub struct LogNamedDecimalUintFilter {
        pub key: ::std::string::String,
        pub val: ::ethers::core::types::U256,
        pub decimals: ::ethers::core::types::U256,
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
    #[ethevent(name = "log_named_int", abi = "log_named_int(string,int256)")]
    pub struct LogNamedIntFilter {
        pub key: ::std::string::String,
        pub val: ::ethers::core::types::I256,
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
    #[ethevent(name = "log_named_string", abi = "log_named_string(string,string)")]
    pub struct LogNamedStringFilter {
        pub key: ::std::string::String,
        pub val: ::std::string::String,
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
    #[ethevent(name = "log_named_uint", abi = "log_named_uint(string,uint256)")]
    pub struct LogNamedUintFilter {
        pub key: ::std::string::String,
        pub val: ::ethers::core::types::U256,
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
    #[ethevent(name = "log_string", abi = "log_string(string)")]
    pub struct LogStringFilter(pub ::std::string::String);
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
    #[ethevent(name = "log_uint", abi = "log_uint(uint256)")]
    pub struct LogUintFilter(pub ::ethers::core::types::U256);
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
    #[ethevent(name = "logs", abi = "logs(bytes)")]
    pub struct LogsFilter(pub ::ethers::core::types::Bytes);
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
    pub enum GaspMultiRollupServiceTestEvents {
        EigenOpUpdateProcessedFilter(EigenOpUpdateProcessedFilter),
        EigenRdUpdateProcessedFilter(EigenRdUpdateProcessedFilter),
        EigenReinitProcessedFilter(EigenReinitProcessedFilter),
        RolldownTargetUpdatedFilter(RolldownTargetUpdatedFilter),
        LogFilter(LogFilter),
        LogAddressFilter(LogAddressFilter),
        LogArray1Filter(LogArray1Filter),
        LogArray2Filter(LogArray2Filter),
        LogArray3Filter(LogArray3Filter),
        LogBytesFilter(LogBytesFilter),
        LogBytes32Filter(LogBytes32Filter),
        LogIntFilter(LogIntFilter),
        LogNamedAddressFilter(LogNamedAddressFilter),
        LogNamedArray1Filter(LogNamedArray1Filter),
        LogNamedArray2Filter(LogNamedArray2Filter),
        LogNamedArray3Filter(LogNamedArray3Filter),
        LogNamedBytesFilter(LogNamedBytesFilter),
        LogNamedBytes32Filter(LogNamedBytes32Filter),
        LogNamedDecimalIntFilter(LogNamedDecimalIntFilter),
        LogNamedDecimalUintFilter(LogNamedDecimalUintFilter),
        LogNamedIntFilter(LogNamedIntFilter),
        LogNamedStringFilter(LogNamedStringFilter),
        LogNamedUintFilter(LogNamedUintFilter),
        LogStringFilter(LogStringFilter),
        LogUintFilter(LogUintFilter),
        LogsFilter(LogsFilter),
    }
    impl ::ethers::contract::EthLogDecode for GaspMultiRollupServiceTestEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = EigenOpUpdateProcessedFilter::decode_log(log) {
                return Ok(GaspMultiRollupServiceTestEvents::EigenOpUpdateProcessedFilter(decoded));
            }
            if let Ok(decoded) = EigenRdUpdateProcessedFilter::decode_log(log) {
                return Ok(GaspMultiRollupServiceTestEvents::EigenRdUpdateProcessedFilter(decoded));
            }
            if let Ok(decoded) = EigenReinitProcessedFilter::decode_log(log) {
                return Ok(GaspMultiRollupServiceTestEvents::EigenReinitProcessedFilter(decoded));
            }
            if let Ok(decoded) = RolldownTargetUpdatedFilter::decode_log(log) {
                return Ok(GaspMultiRollupServiceTestEvents::RolldownTargetUpdatedFilter(decoded));
            }
            if let Ok(decoded) = LogFilter::decode_log(log) {
                return Ok(GaspMultiRollupServiceTestEvents::LogFilter(decoded));
            }
            if let Ok(decoded) = LogAddressFilter::decode_log(log) {
                return Ok(GaspMultiRollupServiceTestEvents::LogAddressFilter(decoded));
            }
            if let Ok(decoded) = LogArray1Filter::decode_log(log) {
                return Ok(GaspMultiRollupServiceTestEvents::LogArray1Filter(decoded));
            }
            if let Ok(decoded) = LogArray2Filter::decode_log(log) {
                return Ok(GaspMultiRollupServiceTestEvents::LogArray2Filter(decoded));
            }
            if let Ok(decoded) = LogArray3Filter::decode_log(log) {
                return Ok(GaspMultiRollupServiceTestEvents::LogArray3Filter(decoded));
            }
            if let Ok(decoded) = LogBytesFilter::decode_log(log) {
                return Ok(GaspMultiRollupServiceTestEvents::LogBytesFilter(decoded));
            }
            if let Ok(decoded) = LogBytes32Filter::decode_log(log) {
                return Ok(GaspMultiRollupServiceTestEvents::LogBytes32Filter(decoded));
            }
            if let Ok(decoded) = LogIntFilter::decode_log(log) {
                return Ok(GaspMultiRollupServiceTestEvents::LogIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedAddressFilter::decode_log(log) {
                return Ok(GaspMultiRollupServiceTestEvents::LogNamedAddressFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = LogNamedArray1Filter::decode_log(log) {
                return Ok(GaspMultiRollupServiceTestEvents::LogNamedArray1Filter(
                    decoded,
                ));
            }
            if let Ok(decoded) = LogNamedArray2Filter::decode_log(log) {
                return Ok(GaspMultiRollupServiceTestEvents::LogNamedArray2Filter(
                    decoded,
                ));
            }
            if let Ok(decoded) = LogNamedArray3Filter::decode_log(log) {
                return Ok(GaspMultiRollupServiceTestEvents::LogNamedArray3Filter(
                    decoded,
                ));
            }
            if let Ok(decoded) = LogNamedBytesFilter::decode_log(log) {
                return Ok(GaspMultiRollupServiceTestEvents::LogNamedBytesFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = LogNamedBytes32Filter::decode_log(log) {
                return Ok(GaspMultiRollupServiceTestEvents::LogNamedBytes32Filter(
                    decoded,
                ));
            }
            if let Ok(decoded) = LogNamedDecimalIntFilter::decode_log(log) {
                return Ok(GaspMultiRollupServiceTestEvents::LogNamedDecimalIntFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = LogNamedDecimalUintFilter::decode_log(log) {
                return Ok(GaspMultiRollupServiceTestEvents::LogNamedDecimalUintFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = LogNamedIntFilter::decode_log(log) {
                return Ok(GaspMultiRollupServiceTestEvents::LogNamedIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedStringFilter::decode_log(log) {
                return Ok(GaspMultiRollupServiceTestEvents::LogNamedStringFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = LogNamedUintFilter::decode_log(log) {
                return Ok(GaspMultiRollupServiceTestEvents::LogNamedUintFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = LogStringFilter::decode_log(log) {
                return Ok(GaspMultiRollupServiceTestEvents::LogStringFilter(decoded));
            }
            if let Ok(decoded) = LogUintFilter::decode_log(log) {
                return Ok(GaspMultiRollupServiceTestEvents::LogUintFilter(decoded));
            }
            if let Ok(decoded) = LogsFilter::decode_log(log) {
                return Ok(GaspMultiRollupServiceTestEvents::LogsFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for GaspMultiRollupServiceTestEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::EigenOpUpdateProcessedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EigenRdUpdateProcessedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EigenReinitProcessedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RolldownTargetUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogAddressFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogArray1Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogArray2Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogArray3Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytesFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes32Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogIntFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogNamedAddressFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogNamedArray1Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogNamedArray2Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogNamedArray3Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogNamedBytesFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogNamedBytes32Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogNamedDecimalIntFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogNamedDecimalUintFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogNamedIntFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogNamedStringFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogNamedUintFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogStringFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogUintFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogsFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<EigenOpUpdateProcessedFilter> for GaspMultiRollupServiceTestEvents {
        fn from(value: EigenOpUpdateProcessedFilter) -> Self {
            Self::EigenOpUpdateProcessedFilter(value)
        }
    }
    impl ::core::convert::From<EigenRdUpdateProcessedFilter> for GaspMultiRollupServiceTestEvents {
        fn from(value: EigenRdUpdateProcessedFilter) -> Self {
            Self::EigenRdUpdateProcessedFilter(value)
        }
    }
    impl ::core::convert::From<EigenReinitProcessedFilter> for GaspMultiRollupServiceTestEvents {
        fn from(value: EigenReinitProcessedFilter) -> Self {
            Self::EigenReinitProcessedFilter(value)
        }
    }
    impl ::core::convert::From<RolldownTargetUpdatedFilter> for GaspMultiRollupServiceTestEvents {
        fn from(value: RolldownTargetUpdatedFilter) -> Self {
            Self::RolldownTargetUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<LogFilter> for GaspMultiRollupServiceTestEvents {
        fn from(value: LogFilter) -> Self {
            Self::LogFilter(value)
        }
    }
    impl ::core::convert::From<LogAddressFilter> for GaspMultiRollupServiceTestEvents {
        fn from(value: LogAddressFilter) -> Self {
            Self::LogAddressFilter(value)
        }
    }
    impl ::core::convert::From<LogArray1Filter> for GaspMultiRollupServiceTestEvents {
        fn from(value: LogArray1Filter) -> Self {
            Self::LogArray1Filter(value)
        }
    }
    impl ::core::convert::From<LogArray2Filter> for GaspMultiRollupServiceTestEvents {
        fn from(value: LogArray2Filter) -> Self {
            Self::LogArray2Filter(value)
        }
    }
    impl ::core::convert::From<LogArray3Filter> for GaspMultiRollupServiceTestEvents {
        fn from(value: LogArray3Filter) -> Self {
            Self::LogArray3Filter(value)
        }
    }
    impl ::core::convert::From<LogBytesFilter> for GaspMultiRollupServiceTestEvents {
        fn from(value: LogBytesFilter) -> Self {
            Self::LogBytesFilter(value)
        }
    }
    impl ::core::convert::From<LogBytes32Filter> for GaspMultiRollupServiceTestEvents {
        fn from(value: LogBytes32Filter) -> Self {
            Self::LogBytes32Filter(value)
        }
    }
    impl ::core::convert::From<LogIntFilter> for GaspMultiRollupServiceTestEvents {
        fn from(value: LogIntFilter) -> Self {
            Self::LogIntFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedAddressFilter> for GaspMultiRollupServiceTestEvents {
        fn from(value: LogNamedAddressFilter) -> Self {
            Self::LogNamedAddressFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedArray1Filter> for GaspMultiRollupServiceTestEvents {
        fn from(value: LogNamedArray1Filter) -> Self {
            Self::LogNamedArray1Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedArray2Filter> for GaspMultiRollupServiceTestEvents {
        fn from(value: LogNamedArray2Filter) -> Self {
            Self::LogNamedArray2Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedArray3Filter> for GaspMultiRollupServiceTestEvents {
        fn from(value: LogNamedArray3Filter) -> Self {
            Self::LogNamedArray3Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedBytesFilter> for GaspMultiRollupServiceTestEvents {
        fn from(value: LogNamedBytesFilter) -> Self {
            Self::LogNamedBytesFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedBytes32Filter> for GaspMultiRollupServiceTestEvents {
        fn from(value: LogNamedBytes32Filter) -> Self {
            Self::LogNamedBytes32Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedDecimalIntFilter> for GaspMultiRollupServiceTestEvents {
        fn from(value: LogNamedDecimalIntFilter) -> Self {
            Self::LogNamedDecimalIntFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedDecimalUintFilter> for GaspMultiRollupServiceTestEvents {
        fn from(value: LogNamedDecimalUintFilter) -> Self {
            Self::LogNamedDecimalUintFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedIntFilter> for GaspMultiRollupServiceTestEvents {
        fn from(value: LogNamedIntFilter) -> Self {
            Self::LogNamedIntFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedStringFilter> for GaspMultiRollupServiceTestEvents {
        fn from(value: LogNamedStringFilter) -> Self {
            Self::LogNamedStringFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedUintFilter> for GaspMultiRollupServiceTestEvents {
        fn from(value: LogNamedUintFilter) -> Self {
            Self::LogNamedUintFilter(value)
        }
    }
    impl ::core::convert::From<LogStringFilter> for GaspMultiRollupServiceTestEvents {
        fn from(value: LogStringFilter) -> Self {
            Self::LogStringFilter(value)
        }
    }
    impl ::core::convert::From<LogUintFilter> for GaspMultiRollupServiceTestEvents {
        fn from(value: LogUintFilter) -> Self {
            Self::LogUintFilter(value)
        }
    }
    impl ::core::convert::From<LogsFilter> for GaspMultiRollupServiceTestEvents {
        fn from(value: LogsFilter) -> Self {
            Self::LogsFilter(value)
        }
    }
    ///Container type for all input parameters for the `IS_TEST` function with signature `IS_TEST()` and selector `0xfa7626d4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "IS_TEST", abi = "IS_TEST()")]
    pub struct IsTestCall;
    ///Container type for all input parameters for the `excludeArtifacts` function with signature `excludeArtifacts()` and selector `0xb5508aa9`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "excludeArtifacts", abi = "excludeArtifacts()")]
    pub struct ExcludeArtifactsCall;
    ///Container type for all input parameters for the `excludeContracts` function with signature `excludeContracts()` and selector `0xe20c9f71`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "excludeContracts", abi = "excludeContracts()")]
    pub struct ExcludeContractsCall;
    ///Container type for all input parameters for the `excludeSelectors` function with signature `excludeSelectors()` and selector `0xb0464fdc`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "excludeSelectors", abi = "excludeSelectors()")]
    pub struct ExcludeSelectorsCall;
    ///Container type for all input parameters for the `excludeSenders` function with signature `excludeSenders()` and selector `0x1ed7831c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "excludeSenders", abi = "excludeSenders()")]
    pub struct ExcludeSendersCall;
    ///Container type for all input parameters for the `failed` function with signature `failed()` and selector `0xba414fa6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "failed", abi = "failed()")]
    pub struct FailedCall;
    ///Container type for all input parameters for the `targetArtifactSelectors` function with signature `targetArtifactSelectors()` and selector `0x66d9a9a0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "targetArtifactSelectors", abi = "targetArtifactSelectors()")]
    pub struct TargetArtifactSelectorsCall;
    ///Container type for all input parameters for the `targetArtifacts` function with signature `targetArtifacts()` and selector `0x85226c81`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "targetArtifacts", abi = "targetArtifacts()")]
    pub struct TargetArtifactsCall;
    ///Container type for all input parameters for the `targetContracts` function with signature `targetContracts()` and selector `0x3f7286f4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "targetContracts", abi = "targetContracts()")]
    pub struct TargetContractsCall;
    ///Container type for all input parameters for the `targetInterfaces` function with signature `targetInterfaces()` and selector `0x2ade3880`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "targetInterfaces", abi = "targetInterfaces()")]
    pub struct TargetInterfacesCall;
    ///Container type for all input parameters for the `targetSelectors` function with signature `targetSelectors()` and selector `0x916a17c6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "targetSelectors", abi = "targetSelectors()")]
    pub struct TargetSelectorsCall;
    ///Container type for all input parameters for the `targetSenders` function with signature `targetSenders()` and selector `0x3e5e3c23`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "targetSenders", abi = "targetSenders()")]
    pub struct TargetSendersCall;
    ///Container type for all input parameters for the `testGasp` function with signature `testGasp()` and selector `0x54f2d354`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "testGasp", abi = "testGasp()")]
    pub struct TestGaspCall;
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
    pub enum GaspMultiRollupServiceTestCalls {
        IsTest(IsTestCall),
        ExcludeArtifacts(ExcludeArtifactsCall),
        ExcludeContracts(ExcludeContractsCall),
        ExcludeSelectors(ExcludeSelectorsCall),
        ExcludeSenders(ExcludeSendersCall),
        Failed(FailedCall),
        TargetArtifactSelectors(TargetArtifactSelectorsCall),
        TargetArtifacts(TargetArtifactsCall),
        TargetContracts(TargetContractsCall),
        TargetInterfaces(TargetInterfacesCall),
        TargetSelectors(TargetSelectorsCall),
        TargetSenders(TargetSendersCall),
        TestGasp(TestGaspCall),
    }
    impl ::ethers::core::abi::AbiDecode for GaspMultiRollupServiceTestCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <IsTestCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsTest(decoded));
            }
            if let Ok(decoded) =
                <ExcludeArtifactsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ExcludeArtifacts(decoded));
            }
            if let Ok(decoded) =
                <ExcludeContractsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ExcludeContracts(decoded));
            }
            if let Ok(decoded) =
                <ExcludeSelectorsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ExcludeSelectors(decoded));
            }
            if let Ok(decoded) =
                <ExcludeSendersCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ExcludeSenders(decoded));
            }
            if let Ok(decoded) = <FailedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Failed(decoded));
            }
            if let Ok(decoded) =
                <TargetArtifactSelectorsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TargetArtifactSelectors(decoded));
            }
            if let Ok(decoded) =
                <TargetArtifactsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TargetArtifacts(decoded));
            }
            if let Ok(decoded) =
                <TargetContractsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TargetContracts(decoded));
            }
            if let Ok(decoded) =
                <TargetInterfacesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TargetInterfaces(decoded));
            }
            if let Ok(decoded) =
                <TargetSelectorsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TargetSelectors(decoded));
            }
            if let Ok(decoded) = <TargetSendersCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TargetSenders(decoded));
            }
            if let Ok(decoded) = <TestGaspCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TestGasp(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for GaspMultiRollupServiceTestCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::IsTest(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExcludeArtifacts(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExcludeContracts(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExcludeSelectors(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExcludeSenders(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Failed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TargetArtifactSelectors(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TargetArtifacts(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TargetContracts(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TargetInterfaces(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TargetSelectors(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TargetSenders(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TestGasp(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for GaspMultiRollupServiceTestCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::IsTest(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExcludeArtifacts(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExcludeContracts(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExcludeSelectors(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExcludeSenders(element) => ::core::fmt::Display::fmt(element, f),
                Self::Failed(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetArtifactSelectors(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetArtifacts(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetContracts(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetInterfaces(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetSelectors(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetSenders(element) => ::core::fmt::Display::fmt(element, f),
                Self::TestGasp(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<IsTestCall> for GaspMultiRollupServiceTestCalls {
        fn from(value: IsTestCall) -> Self {
            Self::IsTest(value)
        }
    }
    impl ::core::convert::From<ExcludeArtifactsCall> for GaspMultiRollupServiceTestCalls {
        fn from(value: ExcludeArtifactsCall) -> Self {
            Self::ExcludeArtifacts(value)
        }
    }
    impl ::core::convert::From<ExcludeContractsCall> for GaspMultiRollupServiceTestCalls {
        fn from(value: ExcludeContractsCall) -> Self {
            Self::ExcludeContracts(value)
        }
    }
    impl ::core::convert::From<ExcludeSelectorsCall> for GaspMultiRollupServiceTestCalls {
        fn from(value: ExcludeSelectorsCall) -> Self {
            Self::ExcludeSelectors(value)
        }
    }
    impl ::core::convert::From<ExcludeSendersCall> for GaspMultiRollupServiceTestCalls {
        fn from(value: ExcludeSendersCall) -> Self {
            Self::ExcludeSenders(value)
        }
    }
    impl ::core::convert::From<FailedCall> for GaspMultiRollupServiceTestCalls {
        fn from(value: FailedCall) -> Self {
            Self::Failed(value)
        }
    }
    impl ::core::convert::From<TargetArtifactSelectorsCall> for GaspMultiRollupServiceTestCalls {
        fn from(value: TargetArtifactSelectorsCall) -> Self {
            Self::TargetArtifactSelectors(value)
        }
    }
    impl ::core::convert::From<TargetArtifactsCall> for GaspMultiRollupServiceTestCalls {
        fn from(value: TargetArtifactsCall) -> Self {
            Self::TargetArtifacts(value)
        }
    }
    impl ::core::convert::From<TargetContractsCall> for GaspMultiRollupServiceTestCalls {
        fn from(value: TargetContractsCall) -> Self {
            Self::TargetContracts(value)
        }
    }
    impl ::core::convert::From<TargetInterfacesCall> for GaspMultiRollupServiceTestCalls {
        fn from(value: TargetInterfacesCall) -> Self {
            Self::TargetInterfaces(value)
        }
    }
    impl ::core::convert::From<TargetSelectorsCall> for GaspMultiRollupServiceTestCalls {
        fn from(value: TargetSelectorsCall) -> Self {
            Self::TargetSelectors(value)
        }
    }
    impl ::core::convert::From<TargetSendersCall> for GaspMultiRollupServiceTestCalls {
        fn from(value: TargetSendersCall) -> Self {
            Self::TargetSenders(value)
        }
    }
    impl ::core::convert::From<TestGaspCall> for GaspMultiRollupServiceTestCalls {
        fn from(value: TestGaspCall) -> Self {
            Self::TestGasp(value)
        }
    }
    ///Container type for all return fields from the `IS_TEST` function with signature `IS_TEST()` and selector `0xfa7626d4`
    #[derive(
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
    pub struct IsTestReturn(pub bool);
    ///Container type for all return fields from the `excludeArtifacts` function with signature `excludeArtifacts()` and selector `0xb5508aa9`
    #[derive(
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
    pub struct ExcludeArtifactsReturn {
        pub excluded_artifacts: ::std::vec::Vec<::std::string::String>,
    }
    ///Container type for all return fields from the `excludeContracts` function with signature `excludeContracts()` and selector `0xe20c9f71`
    #[derive(
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
    pub struct ExcludeContractsReturn {
        pub excluded_contracts: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all return fields from the `excludeSelectors` function with signature `excludeSelectors()` and selector `0xb0464fdc`
    #[derive(
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
    pub struct ExcludeSelectorsReturn {
        pub excluded_selectors: ::std::vec::Vec<FuzzSelector>,
    }
    ///Container type for all return fields from the `excludeSenders` function with signature `excludeSenders()` and selector `0x1ed7831c`
    #[derive(
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
    pub struct ExcludeSendersReturn {
        pub excluded_senders: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all return fields from the `failed` function with signature `failed()` and selector `0xba414fa6`
    #[derive(
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
    pub struct FailedReturn(pub bool);
    ///Container type for all return fields from the `targetArtifactSelectors` function with signature `targetArtifactSelectors()` and selector `0x66d9a9a0`
    #[derive(
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
    pub struct TargetArtifactSelectorsReturn {
        pub targeted_artifact_selectors: ::std::vec::Vec<FuzzArtifactSelector>,
    }
    ///Container type for all return fields from the `targetArtifacts` function with signature `targetArtifacts()` and selector `0x85226c81`
    #[derive(
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
    pub struct TargetArtifactsReturn {
        pub targeted_artifacts: ::std::vec::Vec<::std::string::String>,
    }
    ///Container type for all return fields from the `targetContracts` function with signature `targetContracts()` and selector `0x3f7286f4`
    #[derive(
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
    pub struct TargetContractsReturn {
        pub targeted_contracts: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all return fields from the `targetInterfaces` function with signature `targetInterfaces()` and selector `0x2ade3880`
    #[derive(
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
    pub struct TargetInterfacesReturn {
        pub targeted_interfaces: ::std::vec::Vec<FuzzInterface>,
    }
    ///Container type for all return fields from the `targetSelectors` function with signature `targetSelectors()` and selector `0x916a17c6`
    #[derive(
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
    pub struct TargetSelectorsReturn {
        pub targeted_selectors: ::std::vec::Vec<FuzzSelector>,
    }
    ///Container type for all return fields from the `targetSenders` function with signature `targetSenders()` and selector `0x3e5e3c23`
    #[derive(
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
    pub struct TargetSendersReturn {
        pub targeted_senders: ::std::vec::Vec<::ethers::core::types::Address>,
    }
}
