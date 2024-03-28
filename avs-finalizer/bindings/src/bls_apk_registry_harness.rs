pub use bls_apk_registry_harness::*;
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
pub mod bls_apk_registry_harness {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                    name: ::std::borrow::ToOwned::to_owned("_registryCoordinator"),
                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                    internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                        "contract IRegistryCoordinator",
                    ),),
                },],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("apkHistory"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("apkHistory"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("apkHash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(24usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes24"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("updateBlockNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("nextUpdateBlockNumber",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("currentApk"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("currentApk"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint8"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("X"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("Y"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deregisterOperator"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("deregisterOperator"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
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
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getApk"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getApk"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint8"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct BN254.G1Point"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getApkHashAtBlockNumberAndIndex"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getApkHashAtBlockNumberAndIndex",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("blockNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("index"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(24usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes24"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getApkHistoryLength"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getApkHistoryLength",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint8"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getApkIndicesAtBlockNumber"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getApkIndicesAtBlockNumber",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("quorumNumbers"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("blockNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getApkUpdateAtIndex"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getApkUpdateAtIndex",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("index"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(24usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IBLSApkRegistry.ApkUpdate",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getOperatorFromPubkeyHash"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getOperatorFromPubkeyHash",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("pubkeyHash"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getOperatorId"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getOperatorId"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("operator"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getRegisteredPubkey"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getRegisteredPubkey",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("operator"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct BN254.G1Point"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initializeQuorum"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("initializeQuorum"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint8"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("operatorToPubkey"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("operatorToPubkey"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("X"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("Y"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("operatorToPubkeyHash"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("operatorToPubkeyHash",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pubkeyHashToOperator"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pubkeyHashToOperator",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("registerBLSPublicKey"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("registerBLSPublicKey",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("params"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ],),
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ],),
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                            ::std::boxed::Box::new(
                                                ::ethers::core::abi::ethabi::ParamType::Uint(
                                                    256usize
                                                ),
                                            ),
                                            2usize,
                                        ),
                                        ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                            ::std::boxed::Box::new(
                                                ::ethers::core::abi::ethabi::ParamType::Uint(
                                                    256usize
                                                ),
                                            ),
                                            2usize,
                                        ),
                                    ],),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct IBLSApkRegistry.PubkeyRegistrationParams",
                                    ),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned(
                                    "pubkeyRegistrationMessageHash",
                                ),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct BN254.G1Point"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("operatorId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("registerOperator"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("registerOperator"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
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
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("registryCoordinator"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("registryCoordinator",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setBLSPublicKey"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setBLSPublicKey"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("pk"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct BN254.G1Point"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Initialized"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Initialized"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("version"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NewPubkeyRegistration"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("NewPubkeyRegistration",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("pubkeyG1"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("pubkeyG2"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
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
                                ],),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OperatorAddedToQuorums"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("OperatorAddedToQuorums",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("quorumNumbers"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OperatorRemovedFromQuorums"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("OperatorRemovedFromQuorums",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("quorumNumbers"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                indexed: false,
                            },
                        ],
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
    pub static BLSAPKREGISTRYHARNESS_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0!\xD28\x03\x80b\0!\xD2\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x01\x18V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\x80R\x80\x80b\0\0Mb\0\0VV[PPPb\0\x01JV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15b\0\0\xC3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\xFF\x90\x81\x16\x10\x15b\0\x01\x16W`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\0` \x82\x84\x03\x12\x15b\0\x01+W`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01CW`\0\x80\xFD[\x93\x92PPPV[`\x80Qa Pb\0\x01\x82`\09`\0\x81\x81a\x03\x1A\x01R\x81\x81a\x04\xDB\x01R\x81\x81a\x064\x01R\x81\x81a\n:\x01Ra\x10\xA6\x01Ra P`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01 W`\x005`\xE0\x1C\x80cm\x14\xA9\x87\x11a\0\xADW\x80c\xD5%J\x8C\x11a\0qW\x80c\xD5%J\x8C\x14a\x03\xEAW\x80c\xDE)\xFA\xC0\x14a\x04\nW\x80c\xDFM\t\xE0\x14a\x04*W\x80c\xE8\xBB\x9A\xE6\x14a\x04\x94W\x80c\xF4\xE2O\xE5\x14a\x04\xBDW`\0\x80\xFD[\x80cm\x14\xA9\x87\x14a\x03\x15W\x80cy\x16\xCE\xA6\x14a\x03<W\x80c\x7F\xF8\x1A\x87\x14a\x03}W\x80c\xA3\xDB\x80\xE2\x14a\x03\xB0W\x80c\xBFy\xCEX\x14a\x03\xD7W`\0\x80\xFD[\x80c?\xB2yR\x11a\0\xF4W\x80c?\xB2yR\x14a\x01\xEAW\x80cG\xB3\x14\xE8\x14a\x01\xFDW\x80c_a\xA8\x84\x14a\x02>W\x80c`WG\xD5\x14a\x02\x9AW\x80ch\xBC\xCA\xAC\x14a\x02\xE8W`\0\x80\xFD[\x80b\xA1\xF4\xCB\x14a\x01%W\x80c\x13T*N\x14a\x01fW\x80c&\xD9A\xF2\x14a\x01\x9DW\x80c7~\xD9\x9D\x14a\x01\xB2W[`\0\x80\xFD[a\x01La\x0136`\x04a\x19yV[`\x03` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T\x82V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x8Fa\x01t6`\x04a\x19yV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[`@Q\x90\x81R` \x01a\x01]V[a\x01\xB0a\x01\xAB6`\x04a\x19\xACV[a\x04\xD0V[\0[a\x01\xD5a\x01\xC06`\x04a\x19\xACV[`\xFF\x16`\0\x90\x81R`\x04` R`@\x90 T\x90V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01]V[a\x01\xB0a\x01\xF86`\x04a\x1A7V[a\x06)V[a\x02&a\x02\x0B6`\x04a\x1A\xDDV[`\0\x90\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01]V[a\x02\x8Da\x02L6`\x04a\x19\xACV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01RP`\xFF\x16`\0\x90\x81R`\x05` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x01T\x90\x82\x01R\x90V[`@Qa\x01]\x91\x90a\x1A\xF6V[a\x02\xADa\x02\xA86`\x04a\x1B\rV[a\x06\xE7V[`@\x80Q\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x80\x84\x01Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x83\x01\x91\x90\x91R\x92\x82\x01Q\x90\x92\x16\x90\x82\x01R``\x01a\x01]V[a\x02\xFBa\x02\xF66`\x04a\x1B7V[a\x07zV[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x91\x16\x81R` \x01a\x01]V[a\x02&\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03Oa\x03J6`\x04a\x1B\rV[a\t\x15V[`@\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x94\x16\x84Rc\xFF\xFF\xFF\xFF\x92\x83\x16` \x85\x01R\x91\x16\x90\x82\x01R``\x01a\x01]V[a\x03\x90a\x03\x8B6`\x04a\x19yV[a\t`V[`@\x80Q\x83Q\x81R` \x93\x84\x01Q\x93\x81\x01\x93\x90\x93R\x82\x01R``\x01a\x01]V[a\x01La\x03\xBE6`\x04a\x19\xACV[`\x05` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T\x82V[a\x01\x8Fa\x03\xE56`\x04a\x1B\x7FV[a\n-V[a\x03\xFDa\x03\xF86`\x04a\x1B\xDCV[a\x0E\x81V[`@Qa\x01]\x91\x90a\x1CTV[a\x01\x8Fa\x04\x186`\x04a\x19yV[`\x01` R`\0\x90\x81R`@\x90 T\x81V[a\x01\xB0a\x0486`\x04a\x1C\xCEV[\x80Q`\0\x90\x81R` \x80\x83\x01\x80Q\x82R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x96\x90\x96\x16\x80\x85R`\x01\x80\x85R\x82\x86 \x88\x90U\x96\x85R`\x02\x84R\x81\x85 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x82\x17\x90U\x84R`\x03\x90\x92R\x91 \x91Q\x82UQ\x91\x01UV[a\x02&a\x04\xA26`\x04a\x1A\xDDV[`\x02` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xB0a\x04\xCB6`\x04a\x1A7V[a\x10\x9BV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x05!W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x18\x90a\x1D\x02V[`@Q\x80\x91\x03\x90\xFD[`\xFF\x81\x16`\0\x90\x81R`\x04` R`@\x90 T\x15a\x05\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FBLSApkRegistry.initializeQuorum:`D\x82\x01Ru quorum already exists`P\x1B`d\x82\x01R`\x84\x01a\x05\x18V[`\xFF\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83R\x84\x81Rc\xFF\xFF\xFF\xFFC\x81\x16\x82\x86\x01\x90\x81R\x82\x85\x01\x87\x81R\x84T`\x01\x81\x01\x86U\x94\x88R\x95\x90\x96 \x91Q\x91\x90\x92\x01\x80T\x95Q\x94Q\x83\x16`\x01`\xE0\x1B\x02`\x01`\x01`\xE0\x1B\x03\x95\x90\x93\x16`\x01`\xC0\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x96\x16\x91\x90\x93\x1C\x17\x93\x90\x93\x17\x91\x90\x91\x16\x91\x90\x91\x17\x90UV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x06qW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x18\x90a\x1D\x02V[`\0a\x06|\x83a\t`V[P\x90Pa\x06\x89\x82\x82a\x11DV[\x7Fs\xA2\xB7\xFB\x84G$\xB9q\x80*\xE9\xB1]\xB0\x94\xD4\xB7\x19-\xF9\xD75\x0E\x14\xEBFk\x9B\"\xEBN\x83a\x06\xCA\x85`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[\x84`@Qa\x06\xDA\x93\x92\x91\x90a\x1DvV[`@Q\x80\x91\x03\x90\xA1PPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R`\xFF\x86\x16\x82R`\x04\x90R\x91\x90\x91 \x80T\x83\x90\x81\x10a\x07$Wa\x07$a\x1D\xE2V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x84\x1B\x16\x82Rc\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x82\x04\x81\x16\x94\x83\x01\x94\x90\x94R`\x01`\xE0\x1B\x90\x04\x90\x92\x16\x90\x82\x01R\x90P[\x92\x91PPV[`\xFF\x83\x16`\0\x90\x81R`\x04` R`@\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x07\xA1Wa\x07\xA1a\x1D\xE2V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x84\x1B\x16\x82Rc\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x82\x04\x81\x16\x94\x83\x01\x85\x90R`\x01`\xE0\x1B\x90\x91\x04\x81\x16\x92\x82\x01\x92\x90\x92R\x92P\x85\x16\x10\x15a\x08hW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FBLSApkRegistry._validateApkHashA`D\x82\x01R\x7FtBlockNumber: index too recent\0\0`d\x82\x01R`\x84\x01a\x05\x18V[`@\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x80a\x08\x8EWP\x80`@\x01Qc\xFF\xFF\xFF\xFF\x16\x84c\xFF\xFF\xFF\xFF\x16\x10[a\t\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FBLSApkRegistry._validateApkHashA`D\x82\x01R\x7FtBlockNumber: not latest apk upd`d\x82\x01Rbate`\xE8\x1B`\x84\x82\x01R`\xA4\x01a\x05\x18V[Q\x94\x93PPPPV[`\x04` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\t1W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`@\x81\x90\x1B\x92Pc\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x82\x04\x81\x16\x92P`\x01`\xE0\x1B\x90\x91\x04\x16\x83V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x81Q\x80\x83\x01\x83R\x81T\x81R`\x01\x91\x82\x01T\x81\x85\x01R\x94\x84R\x90\x91R\x81 T\x90\x91\x90\x80a\n#W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FBLSApkRegistry.getRegisteredPubk`D\x82\x01R\x7Fey: operator is not registered\0\0`d\x82\x01R`\x84\x01a\x05\x18V[\x90\x94\x90\x93P\x91PPV[`\x003`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\nwW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x18\x90a\x1D\x02V[`\0a\n\xA5a\n\x8E6\x86\x90\x03\x86\x01`@\x87\x01a\x1D\xF8V[\x80Q`\0\x90\x81R` \x91\x82\x01Q\x90\x91R`@\x90 \x90V[\x90P\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5\x81\x14\x15a\x0B-W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R`\0\x80Q` a\x1F\xFB\x839\x81Q\x91R`D\x82\x01R\x7FKey: cannot register zero pubkey`d\x82\x01R`\x84\x01a\x05\x18V[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x01` R`@\x90 T\x15a\x0B\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R`\0\x80Q` a\x1F\xFB\x839\x81Q\x91R`D\x82\x01R\x7FKey: operator already registered`d\x82\x01Rf pubkey`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x05\x18V[`\0\x81\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x0C;W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`B`$\x82\x01R`\0\x80Q` a\x1F\xFB\x839\x81Q\x91R`D\x82\x01R\x7FKey: public key already register`d\x82\x01Ra\x19Y`\xF2\x1B`\x84\x82\x01R`\xA4\x01a\x05\x18V[`@\x80Q`\0\x91\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x91a\x0C\x94\x91\x885\x91` \x80\x8B\x015\x92\x8B\x015\x91``\x8C\x015\x91`\x80\x8D\x01\x91`\xC0\x8E\x01\x91\x8D5\x91\x8E\x82\x015\x91\x01a\x1E\x14V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\0\x1Ca\x0C\xB7\x91\x90a\x1E_V[\x90Pa\rQa\x0C\xF0a\x0C\xDB\x83a\x0C\xD56\x8A\x90\x03\x8A\x01`@\x8B\x01a\x1D\xF8V[\x90a\x13\x8FV[a\x0C\xEA6\x89\x90\x03\x89\x01\x89a\x1D\xF8V[\x90a\x14&V[a\x0C\xF8a\x14\xBAV[a\r:a\r+\x85a\x0C\xD5`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x01\x82R`\x02\x90\x82\x01R\x90V[a\x0C\xEA6\x8A\x90\x03\x8A\x01\x8Aa\x1D\xF8V[a\rL6\x8A\x90\x03\x8A\x01`\x80\x8B\x01a\x1E\xF1V[a\x15zV[a\r\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`l`$\x82\x01R`\0\x80Q` a\x1F\xFB\x839\x81Q\x91R`D\x82\x01R\x7FKey: either the G1 signature is `d\x82\x01R\x7Fwrong, or G1 and G2 private key `\x84\x82\x01Rk\x0C\x8D\xE4\r\xCD\xEE\x84\r\xAC.\x8Cm`\xA3\x1B`\xA4\x82\x01R`\xC4\x01a\x05\x18V[`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x89\x82\x01\x805\x82U``\x8B\x015`\x01\x92\x83\x01U\x90\x83R\x81\x84 \x87\x90U\x86\x84R`\x02\x90\x92R\x91\x82\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x84\x17\x90U\x90Q\x7F\xE3\xFBf\x13\xAF.\x890\xCF\x85\xD4\x7F\xCFm\xB1\x01\x92\"Jd\xC6\xCB\xE8\x02>\x0E\xEE\x1B\xA3\x82\x80A\x91a\x0Ep\x91`\x80\x8A\x01\x90a\x1F0V[`@Q\x80\x91\x03\x90\xA2P\x94\x93PPPPV[```\0\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\x9EWa\x0E\x9Ea\x19\xC7V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E\xC7W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x84\x81\x10\x15a\x10\x92W`\0\x86\x86\x83\x81\x81\x10a\x0E\xE9Wa\x0E\xE9a\x1D\xE2V[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x04` R`@\x90 T\x90\x92P\x90P\x80\x15\x80a\x0FLWP`\xFF\x82\x16`\0\x90\x81R`\x04` R`@\x81 \x80T\x90\x91\x90a\x0F0Wa\x0F0a\x1D\xE2V[`\0\x91\x82R` \x90\x91 \x01T`\x01`\xC0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x86\x10[\x15a\x0F\xD9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`Q`$\x82\x01R\x7FBLSApkRegistry.getApkIndicesAtBl`D\x82\x01R\x7FockNumber: blockNumber is before`d\x82\x01Rp the first update`x\x1B`\x84\x82\x01R`\xA4\x01a\x05\x18V[\x80[\x80\x15a\x10|W`\xFF\x83\x16`\0\x90\x81R`\x04` R`@\x90 \x87\x90a\x10\0`\x01\x84a\x1FzV[\x81T\x81\x10a\x10\x10Wa\x10\x10a\x1D\xE2V[`\0\x91\x82R` \x90\x91 \x01T`\x01`\xC0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x11a\x10jWa\x109`\x01\x82a\x1FzV[\x85\x85\x81Q\x81\x10a\x10KWa\x10Ka\x1D\xE2V[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPPa\x10|V[\x80a\x10t\x81a\x1F\x91V[\x91PPa\x0F\xDBV[PPP\x80\x80a\x10\x8A\x90a\x1F\xA8V[\x91PPa\x0E\xCDV[P\x94\x93PPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x10\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x18\x90a\x1D\x02V[`\0a\x10\xEE\x83a\t`V[P\x90Pa\x11\x03\x82a\x10\xFE\x83a\x17\xE7V[a\x11DV[\x7F\xF8C\xEC\xD5:V6u\xE6!\x07\xBE\x14\x94\xFD\xDEJ=I\xAE\xED\xAF\x8D\x88\xC6\x16\xD8SF\xE3P\x0E\x83a\x06\xCA\x85`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0[\x83Q\x81\x10\x15a\x13\x89W`\0\x84\x82\x81Q\x81\x10a\x11xWa\x11xa\x1D\xE2V[\x01` \x90\x81\x01Q`\xF8\x1C`\0\x81\x81R`\x04\x90\x92R`@\x90\x91 T\x90\x91P\x80a\x12\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FBLSApkRegistry._processQuorumApk`D\x82\x01R\x7FUpdate: quorum does not exist\0\0\0`d\x82\x01R`\x84\x01a\x05\x18V[`\xFF\x82\x16`\0\x90\x81R`\x05` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x01T\x90\x82\x01Ra\x12<\x90\x86a\x14&V[`\xFF\x83\x16`\0\x81\x81R`\x05` \x90\x81R`@\x80\x83 \x85Q\x80\x82U\x86\x84\x01\x80Q`\x01\x93\x84\x01U\x90\x85RQ\x83R\x81\x84 \x94\x84R`\x04\x90\x92R\x82 \x93\x97P\x91\x92\x90\x91a\x12\x85\x90\x85a\x1FzV[\x81T\x81\x10a\x12\x95Wa\x12\x95a\x1D\xE2V[`\0\x91\x82R` \x90\x91 \x01\x80T\x90\x91PCc\xFF\xFF\xFF\xFF\x90\x81\x16`\x01`\xC0\x1B\x90\x92\x04\x16\x14\x15a\x12\xD6W\x80T`\x01`\x01`\xC0\x1B\x03\x19\x16`@\x83\x90\x1C\x17\x81Ua\x13rV[\x80Tc\xFF\xFF\xFF\xFFC\x81\x16`\x01`\xE0\x1B\x81\x81\x02`\x01`\x01`\xE0\x1B\x03\x94\x85\x16\x17\x85U`\xFF\x88\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x8B\x16\x81R\x80\x84\x01\x96\x87R\x80\x83\x01\x85\x81R\x82T`\x01\x81\x01\x84U\x92\x86R\x93\x90\x94 \x93Q\x93\x01\x80T\x95Q\x92Q\x87\x16\x90\x94\x02\x91\x90\x95\x16`\x01`\xC0\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x94\x90\x94\x16\x91\x90\x94\x1C\x17\x91\x90\x91\x17\x90\x92\x16\x17\x90U[PPPP\x80\x80a\x13\x81\x90a\x1F\xA8V[\x91PPa\x11[V[PPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x13\xABa\x18\xA6V[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x80\x82\x01\x84\x90R`\0\x90\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a\x13\xDEWa\x13\xE0V[\xFE[P\x80a\x14\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCB[][\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x05\x18V[PP\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x14Ba\x18\xC4V[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R\x83Q`@\x80\x84\x01\x91\x90\x91R\x90\x84\x01Q``\x83\x01R`\0\x90\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a\x13\xDEWP\x80a\x14\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCBXY\x19\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x05\x18V[a\x14\xC2a\x18\xE2V[P`@\x80Q`\x80\x81\x01\x82R\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81\x83\x01\x90\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED``\x83\x01R\x81R\x81Q\x80\x83\x01\x90\x92R\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x81\x01\x91\x90\x91R\x81\x01\x91\x90\x91R\x90V[`@\x80Q\x80\x82\x01\x82R\x85\x81R` \x80\x82\x01\x85\x90R\x82Q\x80\x84\x01\x90\x93R\x85\x83R\x82\x01\x83\x90R`\0\x91a\x15\xA9a\x19\x07V[`\0[`\x02\x81\x10\x15a\x17nW`\0a\x15\xC2\x82`\x06a\x1F\xC3V[\x90P\x84\x82`\x02\x81\x10a\x15\xD6Wa\x15\xD6a\x1D\xE2V[` \x02\x01QQ\x83a\x15\xE8\x83`\0a\x1F\xE2V[`\x0C\x81\x10a\x15\xF8Wa\x15\xF8a\x1D\xE2V[` \x02\x01R\x84\x82`\x02\x81\x10a\x16\x0FWa\x16\x0Fa\x1D\xE2V[` \x02\x01Q` \x01Q\x83\x82`\x01a\x16&\x91\x90a\x1F\xE2V[`\x0C\x81\x10a\x166Wa\x166a\x1D\xE2V[` \x02\x01R\x83\x82`\x02\x81\x10a\x16MWa\x16Ma\x1D\xE2V[` \x02\x01QQQ\x83a\x16`\x83`\x02a\x1F\xE2V[`\x0C\x81\x10a\x16pWa\x16pa\x1D\xE2V[` \x02\x01R\x83\x82`\x02\x81\x10a\x16\x87Wa\x16\x87a\x1D\xE2V[` \x02\x01QQ`\x01` \x02\x01Q\x83a\x16\xA0\x83`\x03a\x1F\xE2V[`\x0C\x81\x10a\x16\xB0Wa\x16\xB0a\x1D\xE2V[` \x02\x01R\x83\x82`\x02\x81\x10a\x16\xC7Wa\x16\xC7a\x1D\xE2V[` \x02\x01Q` \x01Q`\0`\x02\x81\x10a\x16\xE2Wa\x16\xE2a\x1D\xE2V[` \x02\x01Q\x83a\x16\xF3\x83`\x04a\x1F\xE2V[`\x0C\x81\x10a\x17\x03Wa\x17\x03a\x1D\xE2V[` \x02\x01R\x83\x82`\x02\x81\x10a\x17\x1AWa\x17\x1Aa\x1D\xE2V[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a\x175Wa\x175a\x1D\xE2V[` \x02\x01Q\x83a\x17F\x83`\x05a\x1F\xE2V[`\x0C\x81\x10a\x17VWa\x17Va\x1D\xE2V[` \x02\x01RP\x80a\x17f\x81a\x1F\xA8V[\x91PPa\x15\xACV[Pa\x17wa\x19&V[`\0` \x82a\x01\x80\x85`\x08a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a\x13\xDEWP\x80a\x17\xD7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x1C\x18Z\\\x9A[\x99\xCB[\xDC\x18\xDB\xD9\x19KY\x98Z[\x19Y`Z\x1B`D\x82\x01R`d\x01a\x05\x18V[PQ\x15\x15\x98\x97PPPPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q\x15\x80\x15a\x18\x0CWP` \x82\x01Q\x15[\x15a\x18*WPP`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x84` \x01Qa\x18o\x91\x90a\x1E_V[a\x18\x99\x90\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDGa\x1FzV[\x90R\x92\x91PPV[\x91\x90PV[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80a\x18\xF5a\x19DV[\x81R` \x01a\x19\x02a\x19DV[\x90R\x90V[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x18\xA1W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x19\x8BW`\0\x80\xFD[a\x19\x94\x82a\x19bV[\x93\x92PPPV[\x805`\xFF\x81\x16\x81\x14a\x18\xA1W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x19\xBEW`\0\x80\xFD[a\x19\x94\x82a\x19\x9BV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1A\0Wa\x1A\0a\x19\xC7V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1A/Wa\x1A/a\x19\xC7V[`@R\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x1AJW`\0\x80\xFD[a\x1AS\x83a\x19bV[\x91P` \x80\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1AqW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x1A\x85W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x1A\x97Wa\x1A\x97a\x19\xC7V[a\x1A\xA9`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\x1A\x06V[\x91P\x80\x82R\x87\x84\x82\x85\x01\x01\x11\x15a\x1A\xBFW`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x93PPPP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x1A\xEFW`\0\x80\xFD[P5\x91\x90PV[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x81\x01a\x07tV[`\0\x80`@\x83\x85\x03\x12\x15a\x1B W`\0\x80\xFD[a\x1B)\x83a\x19\x9BV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1BLW`\0\x80\xFD[a\x1BU\x84a\x19\x9BV[\x92P` \x84\x015c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1BnW`\0\x80\xFD[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80`\0\x83\x85\x03a\x01`\x81\x12\x15a\x1B\x96W`\0\x80\xFD[a\x1B\x9F\x85a\x19bV[\x93Pa\x01\0`\x1F\x19\x82\x01\x12\x15a\x1B\xB4W`\0\x80\xFD[` \x85\x01\x92P`@a\x01\x1F\x19\x82\x01\x12\x15a\x1B\xCDW`\0\x80\xFD[Pa\x01 \x84\x01\x90P\x92P\x92P\x92V[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x1B\xF1W`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1C\tW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x1C\x1DW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x1C,W`\0\x80\xFD[\x87` \x82\x85\x01\x01\x11\x15a\x1C>W`\0\x80\xFD[` \x92\x83\x01\x98\x90\x97P\x95\x90\x91\x015\x94\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x1C\x92W\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x1CpV[P\x90\x96\x95PPPPPPV[`\0`@\x82\x84\x03\x12\x15a\x1C\xB0W`\0\x80\xFD[a\x1C\xB8a\x19\xDDV[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\0\x80``\x83\x85\x03\x12\x15a\x1C\xE1W`\0\x80\xFD[a\x1C\xEA\x83a\x19bV[\x91Pa\x1C\xF9\x84` \x85\x01a\x1C\x9EV[\x90P\x92P\x92\x90PV[` \x80\x82R`N\x90\x82\x01R\x7FBLSApkRegistry.onlyRegistryCoord`@\x82\x01R\x7Finator: caller is not the regist``\x82\x01Rm9<\x901\xB7\xB7\xB924\xB70\xBA7\xB9`\x91\x1B`\x80\x82\x01R`\xA0\x01\x90V[`\x01\x80`\xA0\x1B\x03\x84\x16\x81R`\0` \x84\x81\x84\x01R```@\x84\x01R\x83Q\x80``\x85\x01R`\0[\x81\x81\x10\x15a\x1D\xB8W\x85\x81\x01\x83\x01Q\x85\x82\x01`\x80\x01R\x82\x01a\x1D\x9CV[\x81\x81\x11\x15a\x1D\xCAW`\0`\x80\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`\x80\x01\x95\x94PPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`@\x82\x84\x03\x12\x15a\x1E\nW`\0\x80\xFD[a\x19\x94\x83\x83a\x1C\x9EV[\x88\x81R\x87` \x82\x01R\x86`@\x82\x01R\x85``\x82\x01R`@\x85`\x80\x83\x017`\0`\xC0\x82\x01`\0\x81R`@\x86\x827PPa\x01\0\x81\x01\x92\x90\x92Ra\x01 \x82\x01Ra\x01@\x01\x96\x95PPPPPPV[`\0\x82a\x1E|WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\0\x82`\x1F\x83\x01\x12a\x1E\x92W`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x1E\xB5Wa\x1E\xB5a\x19\xC7V[\x80`@RP\x80`@\x84\x01\x85\x81\x11\x15a\x1E\xCCW`\0\x80\xFD[\x84[\x81\x81\x10\x15a\x1E\xE6W\x805\x83R` \x92\x83\x01\x92\x01a\x1E\xCEV[P\x91\x95\x94PPPPPV[`\0`\x80\x82\x84\x03\x12\x15a\x1F\x03W`\0\x80\xFD[a\x1F\x0Ba\x19\xDDV[a\x1F\x15\x84\x84a\x1E\x81V[\x81Ra\x1F$\x84`@\x85\x01a\x1E\x81V[` \x82\x01R\x93\x92PPPV[\x825\x81R` \x80\x84\x015\x90\x82\x01R`\xC0\x81\x01`@\x83\x81\x84\x017`\x80\x82\x01`\0\x81R`@\x80\x85\x01\x827P`\0\x81R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a\x1F\x8CWa\x1F\x8Ca\x1FdV[P\x03\x90V[`\0\x81a\x1F\xA0Wa\x1F\xA0a\x1FdV[P`\0\x19\x01\x90V[`\0`\0\x19\x82\x14\x15a\x1F\xBCWa\x1F\xBCa\x1FdV[P`\x01\x01\x90V[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a\x1F\xDDWa\x1F\xDDa\x1FdV[P\x02\x90V[`\0\x82\x19\x82\x11\x15a\x1F\xF5Wa\x1F\xF5a\x1FdV[P\x01\x90V\xFEBLSApkRegistry.registerBLSPublic\xA2dipfsX\"\x12 \x17\xA1\xAB\xD7\xACc\xE4tg\xFB\xBAr5\x1B\xCD$\xF6f\x83\xE5\xA8\xA39\xF7\xD4R\x99\x1D\xC3B\x9AXdsolcC\0\x08\x0C\x003";
    /// The bytecode of the contract.
    pub static BLSAPKREGISTRYHARNESS_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01 W`\x005`\xE0\x1C\x80cm\x14\xA9\x87\x11a\0\xADW\x80c\xD5%J\x8C\x11a\0qW\x80c\xD5%J\x8C\x14a\x03\xEAW\x80c\xDE)\xFA\xC0\x14a\x04\nW\x80c\xDFM\t\xE0\x14a\x04*W\x80c\xE8\xBB\x9A\xE6\x14a\x04\x94W\x80c\xF4\xE2O\xE5\x14a\x04\xBDW`\0\x80\xFD[\x80cm\x14\xA9\x87\x14a\x03\x15W\x80cy\x16\xCE\xA6\x14a\x03<W\x80c\x7F\xF8\x1A\x87\x14a\x03}W\x80c\xA3\xDB\x80\xE2\x14a\x03\xB0W\x80c\xBFy\xCEX\x14a\x03\xD7W`\0\x80\xFD[\x80c?\xB2yR\x11a\0\xF4W\x80c?\xB2yR\x14a\x01\xEAW\x80cG\xB3\x14\xE8\x14a\x01\xFDW\x80c_a\xA8\x84\x14a\x02>W\x80c`WG\xD5\x14a\x02\x9AW\x80ch\xBC\xCA\xAC\x14a\x02\xE8W`\0\x80\xFD[\x80b\xA1\xF4\xCB\x14a\x01%W\x80c\x13T*N\x14a\x01fW\x80c&\xD9A\xF2\x14a\x01\x9DW\x80c7~\xD9\x9D\x14a\x01\xB2W[`\0\x80\xFD[a\x01La\x0136`\x04a\x19yV[`\x03` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T\x82V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x8Fa\x01t6`\x04a\x19yV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[`@Q\x90\x81R` \x01a\x01]V[a\x01\xB0a\x01\xAB6`\x04a\x19\xACV[a\x04\xD0V[\0[a\x01\xD5a\x01\xC06`\x04a\x19\xACV[`\xFF\x16`\0\x90\x81R`\x04` R`@\x90 T\x90V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01]V[a\x01\xB0a\x01\xF86`\x04a\x1A7V[a\x06)V[a\x02&a\x02\x0B6`\x04a\x1A\xDDV[`\0\x90\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01]V[a\x02\x8Da\x02L6`\x04a\x19\xACV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01RP`\xFF\x16`\0\x90\x81R`\x05` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x01T\x90\x82\x01R\x90V[`@Qa\x01]\x91\x90a\x1A\xF6V[a\x02\xADa\x02\xA86`\x04a\x1B\rV[a\x06\xE7V[`@\x80Q\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x80\x84\x01Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x83\x01\x91\x90\x91R\x92\x82\x01Q\x90\x92\x16\x90\x82\x01R``\x01a\x01]V[a\x02\xFBa\x02\xF66`\x04a\x1B7V[a\x07zV[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x91\x16\x81R` \x01a\x01]V[a\x02&\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03Oa\x03J6`\x04a\x1B\rV[a\t\x15V[`@\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x94\x16\x84Rc\xFF\xFF\xFF\xFF\x92\x83\x16` \x85\x01R\x91\x16\x90\x82\x01R``\x01a\x01]V[a\x03\x90a\x03\x8B6`\x04a\x19yV[a\t`V[`@\x80Q\x83Q\x81R` \x93\x84\x01Q\x93\x81\x01\x93\x90\x93R\x82\x01R``\x01a\x01]V[a\x01La\x03\xBE6`\x04a\x19\xACV[`\x05` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T\x82V[a\x01\x8Fa\x03\xE56`\x04a\x1B\x7FV[a\n-V[a\x03\xFDa\x03\xF86`\x04a\x1B\xDCV[a\x0E\x81V[`@Qa\x01]\x91\x90a\x1CTV[a\x01\x8Fa\x04\x186`\x04a\x19yV[`\x01` R`\0\x90\x81R`@\x90 T\x81V[a\x01\xB0a\x0486`\x04a\x1C\xCEV[\x80Q`\0\x90\x81R` \x80\x83\x01\x80Q\x82R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x96\x90\x96\x16\x80\x85R`\x01\x80\x85R\x82\x86 \x88\x90U\x96\x85R`\x02\x84R\x81\x85 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x82\x17\x90U\x84R`\x03\x90\x92R\x91 \x91Q\x82UQ\x91\x01UV[a\x02&a\x04\xA26`\x04a\x1A\xDDV[`\x02` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xB0a\x04\xCB6`\x04a\x1A7V[a\x10\x9BV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x05!W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x18\x90a\x1D\x02V[`@Q\x80\x91\x03\x90\xFD[`\xFF\x81\x16`\0\x90\x81R`\x04` R`@\x90 T\x15a\x05\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FBLSApkRegistry.initializeQuorum:`D\x82\x01Ru quorum already exists`P\x1B`d\x82\x01R`\x84\x01a\x05\x18V[`\xFF\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83R\x84\x81Rc\xFF\xFF\xFF\xFFC\x81\x16\x82\x86\x01\x90\x81R\x82\x85\x01\x87\x81R\x84T`\x01\x81\x01\x86U\x94\x88R\x95\x90\x96 \x91Q\x91\x90\x92\x01\x80T\x95Q\x94Q\x83\x16`\x01`\xE0\x1B\x02`\x01`\x01`\xE0\x1B\x03\x95\x90\x93\x16`\x01`\xC0\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x96\x16\x91\x90\x93\x1C\x17\x93\x90\x93\x17\x91\x90\x91\x16\x91\x90\x91\x17\x90UV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x06qW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x18\x90a\x1D\x02V[`\0a\x06|\x83a\t`V[P\x90Pa\x06\x89\x82\x82a\x11DV[\x7Fs\xA2\xB7\xFB\x84G$\xB9q\x80*\xE9\xB1]\xB0\x94\xD4\xB7\x19-\xF9\xD75\x0E\x14\xEBFk\x9B\"\xEBN\x83a\x06\xCA\x85`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[\x84`@Qa\x06\xDA\x93\x92\x91\x90a\x1DvV[`@Q\x80\x91\x03\x90\xA1PPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R`\xFF\x86\x16\x82R`\x04\x90R\x91\x90\x91 \x80T\x83\x90\x81\x10a\x07$Wa\x07$a\x1D\xE2V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x84\x1B\x16\x82Rc\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x82\x04\x81\x16\x94\x83\x01\x94\x90\x94R`\x01`\xE0\x1B\x90\x04\x90\x92\x16\x90\x82\x01R\x90P[\x92\x91PPV[`\xFF\x83\x16`\0\x90\x81R`\x04` R`@\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x07\xA1Wa\x07\xA1a\x1D\xE2V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x84\x1B\x16\x82Rc\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x82\x04\x81\x16\x94\x83\x01\x85\x90R`\x01`\xE0\x1B\x90\x91\x04\x81\x16\x92\x82\x01\x92\x90\x92R\x92P\x85\x16\x10\x15a\x08hW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FBLSApkRegistry._validateApkHashA`D\x82\x01R\x7FtBlockNumber: index too recent\0\0`d\x82\x01R`\x84\x01a\x05\x18V[`@\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x80a\x08\x8EWP\x80`@\x01Qc\xFF\xFF\xFF\xFF\x16\x84c\xFF\xFF\xFF\xFF\x16\x10[a\t\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FBLSApkRegistry._validateApkHashA`D\x82\x01R\x7FtBlockNumber: not latest apk upd`d\x82\x01Rbate`\xE8\x1B`\x84\x82\x01R`\xA4\x01a\x05\x18V[Q\x94\x93PPPPV[`\x04` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\t1W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`@\x81\x90\x1B\x92Pc\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x82\x04\x81\x16\x92P`\x01`\xE0\x1B\x90\x91\x04\x16\x83V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x81Q\x80\x83\x01\x83R\x81T\x81R`\x01\x91\x82\x01T\x81\x85\x01R\x94\x84R\x90\x91R\x81 T\x90\x91\x90\x80a\n#W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FBLSApkRegistry.getRegisteredPubk`D\x82\x01R\x7Fey: operator is not registered\0\0`d\x82\x01R`\x84\x01a\x05\x18V[\x90\x94\x90\x93P\x91PPV[`\x003`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\nwW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x18\x90a\x1D\x02V[`\0a\n\xA5a\n\x8E6\x86\x90\x03\x86\x01`@\x87\x01a\x1D\xF8V[\x80Q`\0\x90\x81R` \x91\x82\x01Q\x90\x91R`@\x90 \x90V[\x90P\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5\x81\x14\x15a\x0B-W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R`\0\x80Q` a\x1F\xFB\x839\x81Q\x91R`D\x82\x01R\x7FKey: cannot register zero pubkey`d\x82\x01R`\x84\x01a\x05\x18V[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x01` R`@\x90 T\x15a\x0B\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R`\0\x80Q` a\x1F\xFB\x839\x81Q\x91R`D\x82\x01R\x7FKey: operator already registered`d\x82\x01Rf pubkey`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x05\x18V[`\0\x81\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x0C;W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`B`$\x82\x01R`\0\x80Q` a\x1F\xFB\x839\x81Q\x91R`D\x82\x01R\x7FKey: public key already register`d\x82\x01Ra\x19Y`\xF2\x1B`\x84\x82\x01R`\xA4\x01a\x05\x18V[`@\x80Q`\0\x91\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x91a\x0C\x94\x91\x885\x91` \x80\x8B\x015\x92\x8B\x015\x91``\x8C\x015\x91`\x80\x8D\x01\x91`\xC0\x8E\x01\x91\x8D5\x91\x8E\x82\x015\x91\x01a\x1E\x14V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\0\x1Ca\x0C\xB7\x91\x90a\x1E_V[\x90Pa\rQa\x0C\xF0a\x0C\xDB\x83a\x0C\xD56\x8A\x90\x03\x8A\x01`@\x8B\x01a\x1D\xF8V[\x90a\x13\x8FV[a\x0C\xEA6\x89\x90\x03\x89\x01\x89a\x1D\xF8V[\x90a\x14&V[a\x0C\xF8a\x14\xBAV[a\r:a\r+\x85a\x0C\xD5`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x01\x82R`\x02\x90\x82\x01R\x90V[a\x0C\xEA6\x8A\x90\x03\x8A\x01\x8Aa\x1D\xF8V[a\rL6\x8A\x90\x03\x8A\x01`\x80\x8B\x01a\x1E\xF1V[a\x15zV[a\r\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`l`$\x82\x01R`\0\x80Q` a\x1F\xFB\x839\x81Q\x91R`D\x82\x01R\x7FKey: either the G1 signature is `d\x82\x01R\x7Fwrong, or G1 and G2 private key `\x84\x82\x01Rk\x0C\x8D\xE4\r\xCD\xEE\x84\r\xAC.\x8Cm`\xA3\x1B`\xA4\x82\x01R`\xC4\x01a\x05\x18V[`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x89\x82\x01\x805\x82U``\x8B\x015`\x01\x92\x83\x01U\x90\x83R\x81\x84 \x87\x90U\x86\x84R`\x02\x90\x92R\x91\x82\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x84\x17\x90U\x90Q\x7F\xE3\xFBf\x13\xAF.\x890\xCF\x85\xD4\x7F\xCFm\xB1\x01\x92\"Jd\xC6\xCB\xE8\x02>\x0E\xEE\x1B\xA3\x82\x80A\x91a\x0Ep\x91`\x80\x8A\x01\x90a\x1F0V[`@Q\x80\x91\x03\x90\xA2P\x94\x93PPPPV[```\0\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\x9EWa\x0E\x9Ea\x19\xC7V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E\xC7W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x84\x81\x10\x15a\x10\x92W`\0\x86\x86\x83\x81\x81\x10a\x0E\xE9Wa\x0E\xE9a\x1D\xE2V[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x04` R`@\x90 T\x90\x92P\x90P\x80\x15\x80a\x0FLWP`\xFF\x82\x16`\0\x90\x81R`\x04` R`@\x81 \x80T\x90\x91\x90a\x0F0Wa\x0F0a\x1D\xE2V[`\0\x91\x82R` \x90\x91 \x01T`\x01`\xC0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x86\x10[\x15a\x0F\xD9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`Q`$\x82\x01R\x7FBLSApkRegistry.getApkIndicesAtBl`D\x82\x01R\x7FockNumber: blockNumber is before`d\x82\x01Rp the first update`x\x1B`\x84\x82\x01R`\xA4\x01a\x05\x18V[\x80[\x80\x15a\x10|W`\xFF\x83\x16`\0\x90\x81R`\x04` R`@\x90 \x87\x90a\x10\0`\x01\x84a\x1FzV[\x81T\x81\x10a\x10\x10Wa\x10\x10a\x1D\xE2V[`\0\x91\x82R` \x90\x91 \x01T`\x01`\xC0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x11a\x10jWa\x109`\x01\x82a\x1FzV[\x85\x85\x81Q\x81\x10a\x10KWa\x10Ka\x1D\xE2V[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPPa\x10|V[\x80a\x10t\x81a\x1F\x91V[\x91PPa\x0F\xDBV[PPP\x80\x80a\x10\x8A\x90a\x1F\xA8V[\x91PPa\x0E\xCDV[P\x94\x93PPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x10\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\x18\x90a\x1D\x02V[`\0a\x10\xEE\x83a\t`V[P\x90Pa\x11\x03\x82a\x10\xFE\x83a\x17\xE7V[a\x11DV[\x7F\xF8C\xEC\xD5:V6u\xE6!\x07\xBE\x14\x94\xFD\xDEJ=I\xAE\xED\xAF\x8D\x88\xC6\x16\xD8SF\xE3P\x0E\x83a\x06\xCA\x85`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0[\x83Q\x81\x10\x15a\x13\x89W`\0\x84\x82\x81Q\x81\x10a\x11xWa\x11xa\x1D\xE2V[\x01` \x90\x81\x01Q`\xF8\x1C`\0\x81\x81R`\x04\x90\x92R`@\x90\x91 T\x90\x91P\x80a\x12\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FBLSApkRegistry._processQuorumApk`D\x82\x01R\x7FUpdate: quorum does not exist\0\0\0`d\x82\x01R`\x84\x01a\x05\x18V[`\xFF\x82\x16`\0\x90\x81R`\x05` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x01T\x90\x82\x01Ra\x12<\x90\x86a\x14&V[`\xFF\x83\x16`\0\x81\x81R`\x05` \x90\x81R`@\x80\x83 \x85Q\x80\x82U\x86\x84\x01\x80Q`\x01\x93\x84\x01U\x90\x85RQ\x83R\x81\x84 \x94\x84R`\x04\x90\x92R\x82 \x93\x97P\x91\x92\x90\x91a\x12\x85\x90\x85a\x1FzV[\x81T\x81\x10a\x12\x95Wa\x12\x95a\x1D\xE2V[`\0\x91\x82R` \x90\x91 \x01\x80T\x90\x91PCc\xFF\xFF\xFF\xFF\x90\x81\x16`\x01`\xC0\x1B\x90\x92\x04\x16\x14\x15a\x12\xD6W\x80T`\x01`\x01`\xC0\x1B\x03\x19\x16`@\x83\x90\x1C\x17\x81Ua\x13rV[\x80Tc\xFF\xFF\xFF\xFFC\x81\x16`\x01`\xE0\x1B\x81\x81\x02`\x01`\x01`\xE0\x1B\x03\x94\x85\x16\x17\x85U`\xFF\x88\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x8B\x16\x81R\x80\x84\x01\x96\x87R\x80\x83\x01\x85\x81R\x82T`\x01\x81\x01\x84U\x92\x86R\x93\x90\x94 \x93Q\x93\x01\x80T\x95Q\x92Q\x87\x16\x90\x94\x02\x91\x90\x95\x16`\x01`\xC0\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x94\x90\x94\x16\x91\x90\x94\x1C\x17\x91\x90\x91\x17\x90\x92\x16\x17\x90U[PPPP\x80\x80a\x13\x81\x90a\x1F\xA8V[\x91PPa\x11[V[PPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x13\xABa\x18\xA6V[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x80\x82\x01\x84\x90R`\0\x90\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a\x13\xDEWa\x13\xE0V[\xFE[P\x80a\x14\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCB[][\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x05\x18V[PP\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x14Ba\x18\xC4V[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R\x83Q`@\x80\x84\x01\x91\x90\x91R\x90\x84\x01Q``\x83\x01R`\0\x90\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a\x13\xDEWP\x80a\x14\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCBXY\x19\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x05\x18V[a\x14\xC2a\x18\xE2V[P`@\x80Q`\x80\x81\x01\x82R\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81\x83\x01\x90\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED``\x83\x01R\x81R\x81Q\x80\x83\x01\x90\x92R\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x81\x01\x91\x90\x91R\x81\x01\x91\x90\x91R\x90V[`@\x80Q\x80\x82\x01\x82R\x85\x81R` \x80\x82\x01\x85\x90R\x82Q\x80\x84\x01\x90\x93R\x85\x83R\x82\x01\x83\x90R`\0\x91a\x15\xA9a\x19\x07V[`\0[`\x02\x81\x10\x15a\x17nW`\0a\x15\xC2\x82`\x06a\x1F\xC3V[\x90P\x84\x82`\x02\x81\x10a\x15\xD6Wa\x15\xD6a\x1D\xE2V[` \x02\x01QQ\x83a\x15\xE8\x83`\0a\x1F\xE2V[`\x0C\x81\x10a\x15\xF8Wa\x15\xF8a\x1D\xE2V[` \x02\x01R\x84\x82`\x02\x81\x10a\x16\x0FWa\x16\x0Fa\x1D\xE2V[` \x02\x01Q` \x01Q\x83\x82`\x01a\x16&\x91\x90a\x1F\xE2V[`\x0C\x81\x10a\x166Wa\x166a\x1D\xE2V[` \x02\x01R\x83\x82`\x02\x81\x10a\x16MWa\x16Ma\x1D\xE2V[` \x02\x01QQQ\x83a\x16`\x83`\x02a\x1F\xE2V[`\x0C\x81\x10a\x16pWa\x16pa\x1D\xE2V[` \x02\x01R\x83\x82`\x02\x81\x10a\x16\x87Wa\x16\x87a\x1D\xE2V[` \x02\x01QQ`\x01` \x02\x01Q\x83a\x16\xA0\x83`\x03a\x1F\xE2V[`\x0C\x81\x10a\x16\xB0Wa\x16\xB0a\x1D\xE2V[` \x02\x01R\x83\x82`\x02\x81\x10a\x16\xC7Wa\x16\xC7a\x1D\xE2V[` \x02\x01Q` \x01Q`\0`\x02\x81\x10a\x16\xE2Wa\x16\xE2a\x1D\xE2V[` \x02\x01Q\x83a\x16\xF3\x83`\x04a\x1F\xE2V[`\x0C\x81\x10a\x17\x03Wa\x17\x03a\x1D\xE2V[` \x02\x01R\x83\x82`\x02\x81\x10a\x17\x1AWa\x17\x1Aa\x1D\xE2V[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a\x175Wa\x175a\x1D\xE2V[` \x02\x01Q\x83a\x17F\x83`\x05a\x1F\xE2V[`\x0C\x81\x10a\x17VWa\x17Va\x1D\xE2V[` \x02\x01RP\x80a\x17f\x81a\x1F\xA8V[\x91PPa\x15\xACV[Pa\x17wa\x19&V[`\0` \x82a\x01\x80\x85`\x08a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a\x13\xDEWP\x80a\x17\xD7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x1C\x18Z\\\x9A[\x99\xCB[\xDC\x18\xDB\xD9\x19KY\x98Z[\x19Y`Z\x1B`D\x82\x01R`d\x01a\x05\x18V[PQ\x15\x15\x98\x97PPPPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q\x15\x80\x15a\x18\x0CWP` \x82\x01Q\x15[\x15a\x18*WPP`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x84` \x01Qa\x18o\x91\x90a\x1E_V[a\x18\x99\x90\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDGa\x1FzV[\x90R\x92\x91PPV[\x91\x90PV[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80a\x18\xF5a\x19DV[\x81R` \x01a\x19\x02a\x19DV[\x90R\x90V[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x18\xA1W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x19\x8BW`\0\x80\xFD[a\x19\x94\x82a\x19bV[\x93\x92PPPV[\x805`\xFF\x81\x16\x81\x14a\x18\xA1W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x19\xBEW`\0\x80\xFD[a\x19\x94\x82a\x19\x9BV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1A\0Wa\x1A\0a\x19\xC7V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1A/Wa\x1A/a\x19\xC7V[`@R\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x1AJW`\0\x80\xFD[a\x1AS\x83a\x19bV[\x91P` \x80\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1AqW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x1A\x85W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x1A\x97Wa\x1A\x97a\x19\xC7V[a\x1A\xA9`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\x1A\x06V[\x91P\x80\x82R\x87\x84\x82\x85\x01\x01\x11\x15a\x1A\xBFW`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x93PPPP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x1A\xEFW`\0\x80\xFD[P5\x91\x90PV[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x81\x01a\x07tV[`\0\x80`@\x83\x85\x03\x12\x15a\x1B W`\0\x80\xFD[a\x1B)\x83a\x19\x9BV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1BLW`\0\x80\xFD[a\x1BU\x84a\x19\x9BV[\x92P` \x84\x015c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1BnW`\0\x80\xFD[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80`\0\x83\x85\x03a\x01`\x81\x12\x15a\x1B\x96W`\0\x80\xFD[a\x1B\x9F\x85a\x19bV[\x93Pa\x01\0`\x1F\x19\x82\x01\x12\x15a\x1B\xB4W`\0\x80\xFD[` \x85\x01\x92P`@a\x01\x1F\x19\x82\x01\x12\x15a\x1B\xCDW`\0\x80\xFD[Pa\x01 \x84\x01\x90P\x92P\x92P\x92V[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x1B\xF1W`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1C\tW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x1C\x1DW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x1C,W`\0\x80\xFD[\x87` \x82\x85\x01\x01\x11\x15a\x1C>W`\0\x80\xFD[` \x92\x83\x01\x98\x90\x97P\x95\x90\x91\x015\x94\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x1C\x92W\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x1CpV[P\x90\x96\x95PPPPPPV[`\0`@\x82\x84\x03\x12\x15a\x1C\xB0W`\0\x80\xFD[a\x1C\xB8a\x19\xDDV[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\0\x80``\x83\x85\x03\x12\x15a\x1C\xE1W`\0\x80\xFD[a\x1C\xEA\x83a\x19bV[\x91Pa\x1C\xF9\x84` \x85\x01a\x1C\x9EV[\x90P\x92P\x92\x90PV[` \x80\x82R`N\x90\x82\x01R\x7FBLSApkRegistry.onlyRegistryCoord`@\x82\x01R\x7Finator: caller is not the regist``\x82\x01Rm9<\x901\xB7\xB7\xB924\xB70\xBA7\xB9`\x91\x1B`\x80\x82\x01R`\xA0\x01\x90V[`\x01\x80`\xA0\x1B\x03\x84\x16\x81R`\0` \x84\x81\x84\x01R```@\x84\x01R\x83Q\x80``\x85\x01R`\0[\x81\x81\x10\x15a\x1D\xB8W\x85\x81\x01\x83\x01Q\x85\x82\x01`\x80\x01R\x82\x01a\x1D\x9CV[\x81\x81\x11\x15a\x1D\xCAW`\0`\x80\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`\x80\x01\x95\x94PPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`@\x82\x84\x03\x12\x15a\x1E\nW`\0\x80\xFD[a\x19\x94\x83\x83a\x1C\x9EV[\x88\x81R\x87` \x82\x01R\x86`@\x82\x01R\x85``\x82\x01R`@\x85`\x80\x83\x017`\0`\xC0\x82\x01`\0\x81R`@\x86\x827PPa\x01\0\x81\x01\x92\x90\x92Ra\x01 \x82\x01Ra\x01@\x01\x96\x95PPPPPPV[`\0\x82a\x1E|WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\0\x82`\x1F\x83\x01\x12a\x1E\x92W`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x1E\xB5Wa\x1E\xB5a\x19\xC7V[\x80`@RP\x80`@\x84\x01\x85\x81\x11\x15a\x1E\xCCW`\0\x80\xFD[\x84[\x81\x81\x10\x15a\x1E\xE6W\x805\x83R` \x92\x83\x01\x92\x01a\x1E\xCEV[P\x91\x95\x94PPPPPV[`\0`\x80\x82\x84\x03\x12\x15a\x1F\x03W`\0\x80\xFD[a\x1F\x0Ba\x19\xDDV[a\x1F\x15\x84\x84a\x1E\x81V[\x81Ra\x1F$\x84`@\x85\x01a\x1E\x81V[` \x82\x01R\x93\x92PPPV[\x825\x81R` \x80\x84\x015\x90\x82\x01R`\xC0\x81\x01`@\x83\x81\x84\x017`\x80\x82\x01`\0\x81R`@\x80\x85\x01\x827P`\0\x81R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a\x1F\x8CWa\x1F\x8Ca\x1FdV[P\x03\x90V[`\0\x81a\x1F\xA0Wa\x1F\xA0a\x1FdV[P`\0\x19\x01\x90V[`\0`\0\x19\x82\x14\x15a\x1F\xBCWa\x1F\xBCa\x1FdV[P`\x01\x01\x90V[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a\x1F\xDDWa\x1F\xDDa\x1FdV[P\x02\x90V[`\0\x82\x19\x82\x11\x15a\x1F\xF5Wa\x1F\xF5a\x1FdV[P\x01\x90V\xFEBLSApkRegistry.registerBLSPublic\xA2dipfsX\"\x12 \x17\xA1\xAB\xD7\xACc\xE4tg\xFB\xBAr5\x1B\xCD$\xF6f\x83\xE5\xA8\xA39\xF7\xD4R\x99\x1D\xC3B\x9AXdsolcC\0\x08\x0C\x003";
    /// The deployed bytecode of the contract.
    pub static BLSAPKREGISTRYHARNESS_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct BLSApkRegistryHarness<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for BLSApkRegistryHarness<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for BLSApkRegistryHarness<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for BLSApkRegistryHarness<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for BLSApkRegistryHarness<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(BLSApkRegistryHarness))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> BLSApkRegistryHarness<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                BLSAPKREGISTRYHARNESS_ABI.clone(),
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
                BLSAPKREGISTRYHARNESS_ABI.clone(),
                BLSAPKREGISTRYHARNESS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `apkHistory` (0x7916cea6) function
        pub fn apk_history(
            &self,
            p0: u8,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ([u8; 24], u32, u32)> {
            self.0
                .method_hash([121, 22, 206, 166], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `currentApk` (0xa3db80e2) function
        pub fn current_apk(
            &self,
            p0: u8,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([163, 219, 128, 226], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deregisterOperator` (0xf4e24fe5) function
        pub fn deregister_operator(
            &self,
            operator: ::ethers::core::types::Address,
            quorum_numbers: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([244, 226, 79, 229], (operator, quorum_numbers))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getApk` (0x5f61a884) function
        pub fn get_apk(
            &self,
            quorum_number: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, G1Point> {
            self.0
                .method_hash([95, 97, 168, 132], quorum_number)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getApkHashAtBlockNumberAndIndex` (0x68bccaac) function
        pub fn get_apk_hash_at_block_number_and_index(
            &self,
            quorum_number: u8,
            block_number: u32,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 24]> {
            self.0
                .method_hash([104, 188, 202, 172], (quorum_number, block_number, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getApkHistoryLength` (0x377ed99d) function
        pub fn get_apk_history_length(
            &self,
            quorum_number: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([55, 126, 217, 157], quorum_number)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getApkIndicesAtBlockNumber` (0xd5254a8c) function
        pub fn get_apk_indices_at_block_number(
            &self,
            quorum_numbers: ::ethers::core::types::Bytes,
            block_number: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<u32>> {
            self.0
                .method_hash([213, 37, 74, 140], (quorum_numbers, block_number))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getApkUpdateAtIndex` (0x605747d5) function
        pub fn get_apk_update_at_index(
            &self,
            quorum_number: u8,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ApkUpdate> {
            self.0
                .method_hash([96, 87, 71, 213], (quorum_number, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOperatorFromPubkeyHash` (0x47b314e8) function
        pub fn get_operator_from_pubkey_hash(
            &self,
            pubkey_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([71, 179, 20, 232], pubkey_hash)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOperatorId` (0x13542a4e) function
        pub fn get_operator_id(
            &self,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([19, 84, 42, 78], operator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRegisteredPubkey` (0x7ff81a87) function
        pub fn get_registered_pubkey(
            &self,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, (G1Point, [u8; 32])> {
            self.0
                .method_hash([127, 248, 26, 135], operator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initializeQuorum` (0x26d941f2) function
        pub fn initialize_quorum(
            &self,
            quorum_number: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([38, 217, 65, 242], quorum_number)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operatorToPubkey` (0x00a1f4cb) function
        pub fn operator_to_pubkey(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([0, 161, 244, 203], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operatorToPubkeyHash` (0xde29fac0) function
        pub fn operator_to_pubkey_hash(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([222, 41, 250, 192], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pubkeyHashToOperator` (0xe8bb9ae6) function
        pub fn pubkey_hash_to_operator(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([232, 187, 154, 230], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerBLSPublicKey` (0xbf79ce58) function
        pub fn register_bls_public_key(
            &self,
            operator: ::ethers::core::types::Address,
            params: PubkeyRegistrationParams,
            pubkey_registration_message_hash: G1Point,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [191, 121, 206, 88],
                    (operator, params, pubkey_registration_message_hash),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerOperator` (0x3fb27952) function
        pub fn register_operator(
            &self,
            operator: ::ethers::core::types::Address,
            quorum_numbers: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([63, 178, 121, 82], (operator, quorum_numbers))
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
        ///Calls the contract's `setBLSPublicKey` (0xdf4d09e0) function
        pub fn set_bls_public_key(
            &self,
            account: ::ethers::core::types::Address,
            pk: G1Point,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([223, 77, 9, 224], (account, pk))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Initialized` event
        pub fn initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, InitializedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `NewPubkeyRegistration` event
        pub fn new_pubkey_registration_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, NewPubkeyRegistrationFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `OperatorAddedToQuorums` event
        pub fn operator_added_to_quorums_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OperatorAddedToQuorumsFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `OperatorRemovedFromQuorums` event
        pub fn operator_removed_from_quorums_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OperatorRemovedFromQuorumsFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, BLSApkRegistryHarnessEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for BLSApkRegistryHarness<M>
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
        name = "NewPubkeyRegistration",
        abi = "NewPubkeyRegistration(address,(uint256,uint256),(uint256[2],uint256[2]))"
    )]
    pub struct NewPubkeyRegistrationFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        pub pubkey_g1: G1Point,
        pub pubkey_g2: G2Point,
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
        name = "OperatorAddedToQuorums",
        abi = "OperatorAddedToQuorums(address,bytes32,bytes)"
    )]
    pub struct OperatorAddedToQuorumsFilter {
        pub operator: ::ethers::core::types::Address,
        pub operator_id: [u8; 32],
        pub quorum_numbers: ::ethers::core::types::Bytes,
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
        name = "OperatorRemovedFromQuorums",
        abi = "OperatorRemovedFromQuorums(address,bytes32,bytes)"
    )]
    pub struct OperatorRemovedFromQuorumsFilter {
        pub operator: ::ethers::core::types::Address,
        pub operator_id: [u8; 32],
        pub quorum_numbers: ::ethers::core::types::Bytes,
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
    pub enum BLSApkRegistryHarnessEvents {
        InitializedFilter(InitializedFilter),
        NewPubkeyRegistrationFilter(NewPubkeyRegistrationFilter),
        OperatorAddedToQuorumsFilter(OperatorAddedToQuorumsFilter),
        OperatorRemovedFromQuorumsFilter(OperatorRemovedFromQuorumsFilter),
    }
    impl ::ethers::contract::EthLogDecode for BLSApkRegistryHarnessEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(BLSApkRegistryHarnessEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = NewPubkeyRegistrationFilter::decode_log(log) {
                return Ok(BLSApkRegistryHarnessEvents::NewPubkeyRegistrationFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = OperatorAddedToQuorumsFilter::decode_log(log) {
                return Ok(BLSApkRegistryHarnessEvents::OperatorAddedToQuorumsFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = OperatorRemovedFromQuorumsFilter::decode_log(log) {
                return Ok(BLSApkRegistryHarnessEvents::OperatorRemovedFromQuorumsFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for BLSApkRegistryHarnessEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::NewPubkeyRegistrationFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorAddedToQuorumsFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorRemovedFromQuorumsFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<InitializedFilter> for BLSApkRegistryHarnessEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<NewPubkeyRegistrationFilter> for BLSApkRegistryHarnessEvents {
        fn from(value: NewPubkeyRegistrationFilter) -> Self {
            Self::NewPubkeyRegistrationFilter(value)
        }
    }
    impl ::core::convert::From<OperatorAddedToQuorumsFilter> for BLSApkRegistryHarnessEvents {
        fn from(value: OperatorAddedToQuorumsFilter) -> Self {
            Self::OperatorAddedToQuorumsFilter(value)
        }
    }
    impl ::core::convert::From<OperatorRemovedFromQuorumsFilter> for BLSApkRegistryHarnessEvents {
        fn from(value: OperatorRemovedFromQuorumsFilter) -> Self {
            Self::OperatorRemovedFromQuorumsFilter(value)
        }
    }
    ///Container type for all input parameters for the `apkHistory` function with signature `apkHistory(uint8,uint256)` and selector `0x7916cea6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "apkHistory", abi = "apkHistory(uint8,uint256)")]
    pub struct ApkHistoryCall(pub u8, pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `currentApk` function with signature `currentApk(uint8)` and selector `0xa3db80e2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "currentApk", abi = "currentApk(uint8)")]
    pub struct CurrentApkCall(pub u8);
    ///Container type for all input parameters for the `deregisterOperator` function with signature `deregisterOperator(address,bytes)` and selector `0xf4e24fe5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "deregisterOperator", abi = "deregisterOperator(address,bytes)")]
    pub struct DeregisterOperatorCall {
        pub operator: ::ethers::core::types::Address,
        pub quorum_numbers: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `getApk` function with signature `getApk(uint8)` and selector `0x5f61a884`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getApk", abi = "getApk(uint8)")]
    pub struct GetApkCall {
        pub quorum_number: u8,
    }
    ///Container type for all input parameters for the `getApkHashAtBlockNumberAndIndex` function with signature `getApkHashAtBlockNumberAndIndex(uint8,uint32,uint256)` and selector `0x68bccaac`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "getApkHashAtBlockNumberAndIndex",
        abi = "getApkHashAtBlockNumberAndIndex(uint8,uint32,uint256)"
    )]
    pub struct GetApkHashAtBlockNumberAndIndexCall {
        pub quorum_number: u8,
        pub block_number: u32,
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getApkHistoryLength` function with signature `getApkHistoryLength(uint8)` and selector `0x377ed99d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getApkHistoryLength", abi = "getApkHistoryLength(uint8)")]
    pub struct GetApkHistoryLengthCall {
        pub quorum_number: u8,
    }
    ///Container type for all input parameters for the `getApkIndicesAtBlockNumber` function with signature `getApkIndicesAtBlockNumber(bytes,uint256)` and selector `0xd5254a8c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "getApkIndicesAtBlockNumber",
        abi = "getApkIndicesAtBlockNumber(bytes,uint256)"
    )]
    pub struct GetApkIndicesAtBlockNumberCall {
        pub quorum_numbers: ::ethers::core::types::Bytes,
        pub block_number: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getApkUpdateAtIndex` function with signature `getApkUpdateAtIndex(uint8,uint256)` and selector `0x605747d5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "getApkUpdateAtIndex",
        abi = "getApkUpdateAtIndex(uint8,uint256)"
    )]
    pub struct GetApkUpdateAtIndexCall {
        pub quorum_number: u8,
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getOperatorFromPubkeyHash` function with signature `getOperatorFromPubkeyHash(bytes32)` and selector `0x47b314e8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "getOperatorFromPubkeyHash",
        abi = "getOperatorFromPubkeyHash(bytes32)"
    )]
    pub struct GetOperatorFromPubkeyHashCall {
        pub pubkey_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `getOperatorId` function with signature `getOperatorId(address)` and selector `0x13542a4e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getOperatorId", abi = "getOperatorId(address)")]
    pub struct GetOperatorIdCall {
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getRegisteredPubkey` function with signature `getRegisteredPubkey(address)` and selector `0x7ff81a87`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getRegisteredPubkey", abi = "getRegisteredPubkey(address)")]
    pub struct GetRegisteredPubkeyCall {
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `initializeQuorum` function with signature `initializeQuorum(uint8)` and selector `0x26d941f2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "initializeQuorum", abi = "initializeQuorum(uint8)")]
    pub struct InitializeQuorumCall {
        pub quorum_number: u8,
    }
    ///Container type for all input parameters for the `operatorToPubkey` function with signature `operatorToPubkey(address)` and selector `0x00a1f4cb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "operatorToPubkey", abi = "operatorToPubkey(address)")]
    pub struct OperatorToPubkeyCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `operatorToPubkeyHash` function with signature `operatorToPubkeyHash(address)` and selector `0xde29fac0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "operatorToPubkeyHash", abi = "operatorToPubkeyHash(address)")]
    pub struct OperatorToPubkeyHashCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `pubkeyHashToOperator` function with signature `pubkeyHashToOperator(bytes32)` and selector `0xe8bb9ae6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "pubkeyHashToOperator", abi = "pubkeyHashToOperator(bytes32)")]
    pub struct PubkeyHashToOperatorCall(pub [u8; 32]);
    ///Container type for all input parameters for the `registerBLSPublicKey` function with signature `registerBLSPublicKey(address,((uint256,uint256),(uint256,uint256),(uint256[2],uint256[2])),(uint256,uint256))` and selector `0xbf79ce58`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "registerBLSPublicKey",
        abi = "registerBLSPublicKey(address,((uint256,uint256),(uint256,uint256),(uint256[2],uint256[2])),(uint256,uint256))"
    )]
    pub struct RegisterBLSPublicKeyCall {
        pub operator: ::ethers::core::types::Address,
        pub params: PubkeyRegistrationParams,
        pub pubkey_registration_message_hash: G1Point,
    }
    ///Container type for all input parameters for the `registerOperator` function with signature `registerOperator(address,bytes)` and selector `0x3fb27952`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "registerOperator", abi = "registerOperator(address,bytes)")]
    pub struct RegisterOperatorCall {
        pub operator: ::ethers::core::types::Address,
        pub quorum_numbers: ::ethers::core::types::Bytes,
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
        Hash,
    )]
    #[ethcall(name = "registryCoordinator", abi = "registryCoordinator()")]
    pub struct RegistryCoordinatorCall;
    ///Container type for all input parameters for the `setBLSPublicKey` function with signature `setBLSPublicKey(address,(uint256,uint256))` and selector `0xdf4d09e0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "setBLSPublicKey",
        abi = "setBLSPublicKey(address,(uint256,uint256))"
    )]
    pub struct SetBLSPublicKeyCall {
        pub account: ::ethers::core::types::Address,
        pub pk: G1Point,
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
    pub enum BLSApkRegistryHarnessCalls {
        ApkHistory(ApkHistoryCall),
        CurrentApk(CurrentApkCall),
        DeregisterOperator(DeregisterOperatorCall),
        GetApk(GetApkCall),
        GetApkHashAtBlockNumberAndIndex(GetApkHashAtBlockNumberAndIndexCall),
        GetApkHistoryLength(GetApkHistoryLengthCall),
        GetApkIndicesAtBlockNumber(GetApkIndicesAtBlockNumberCall),
        GetApkUpdateAtIndex(GetApkUpdateAtIndexCall),
        GetOperatorFromPubkeyHash(GetOperatorFromPubkeyHashCall),
        GetOperatorId(GetOperatorIdCall),
        GetRegisteredPubkey(GetRegisteredPubkeyCall),
        InitializeQuorum(InitializeQuorumCall),
        OperatorToPubkey(OperatorToPubkeyCall),
        OperatorToPubkeyHash(OperatorToPubkeyHashCall),
        PubkeyHashToOperator(PubkeyHashToOperatorCall),
        RegisterBLSPublicKey(RegisterBLSPublicKeyCall),
        RegisterOperator(RegisterOperatorCall),
        RegistryCoordinator(RegistryCoordinatorCall),
        SetBLSPublicKey(SetBLSPublicKeyCall),
    }
    impl ::ethers::core::abi::AbiDecode for BLSApkRegistryHarnessCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ApkHistoryCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ApkHistory(decoded));
            }
            if let Ok(decoded) = <CurrentApkCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CurrentApk(decoded));
            }
            if let Ok(decoded) =
                <DeregisterOperatorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DeregisterOperator(decoded));
            }
            if let Ok(decoded) = <GetApkCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetApk(decoded));
            }
            if let Ok(decoded) =
                <GetApkHashAtBlockNumberAndIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetApkHashAtBlockNumberAndIndex(decoded));
            }
            if let Ok(decoded) =
                <GetApkHistoryLengthCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetApkHistoryLength(decoded));
            }
            if let Ok(decoded) =
                <GetApkIndicesAtBlockNumberCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetApkIndicesAtBlockNumber(decoded));
            }
            if let Ok(decoded) =
                <GetApkUpdateAtIndexCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetApkUpdateAtIndex(decoded));
            }
            if let Ok(decoded) =
                <GetOperatorFromPubkeyHashCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetOperatorFromPubkeyHash(decoded));
            }
            if let Ok(decoded) = <GetOperatorIdCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetOperatorId(decoded));
            }
            if let Ok(decoded) =
                <GetRegisteredPubkeyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetRegisteredPubkey(decoded));
            }
            if let Ok(decoded) =
                <InitializeQuorumCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InitializeQuorum(decoded));
            }
            if let Ok(decoded) =
                <OperatorToPubkeyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OperatorToPubkey(decoded));
            }
            if let Ok(decoded) =
                <OperatorToPubkeyHashCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OperatorToPubkeyHash(decoded));
            }
            if let Ok(decoded) =
                <PubkeyHashToOperatorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PubkeyHashToOperator(decoded));
            }
            if let Ok(decoded) =
                <RegisterBLSPublicKeyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RegisterBLSPublicKey(decoded));
            }
            if let Ok(decoded) =
                <RegisterOperatorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RegisterOperator(decoded));
            }
            if let Ok(decoded) =
                <RegistryCoordinatorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RegistryCoordinator(decoded));
            }
            if let Ok(decoded) =
                <SetBLSPublicKeyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetBLSPublicKey(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for BLSApkRegistryHarnessCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ApkHistory(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CurrentApk(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DeregisterOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetApk(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetApkHashAtBlockNumberAndIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetApkHistoryLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetApkIndicesAtBlockNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetApkUpdateAtIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOperatorFromPubkeyHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOperatorId(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetRegisteredPubkey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InitializeQuorum(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OperatorToPubkey(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OperatorToPubkeyHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PubkeyHashToOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterBLSPublicKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterOperator(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RegistryCoordinator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetBLSPublicKey(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for BLSApkRegistryHarnessCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApkHistory(element) => ::core::fmt::Display::fmt(element, f),
                Self::CurrentApk(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeregisterOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetApk(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetApkHashAtBlockNumberAndIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetApkHistoryLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetApkIndicesAtBlockNumber(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetApkUpdateAtIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOperatorFromPubkeyHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOperatorId(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRegisteredPubkey(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitializeQuorum(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorToPubkey(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorToPubkeyHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::PubkeyHashToOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterBLSPublicKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegistryCoordinator(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetBLSPublicKey(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApkHistoryCall> for BLSApkRegistryHarnessCalls {
        fn from(value: ApkHistoryCall) -> Self {
            Self::ApkHistory(value)
        }
    }
    impl ::core::convert::From<CurrentApkCall> for BLSApkRegistryHarnessCalls {
        fn from(value: CurrentApkCall) -> Self {
            Self::CurrentApk(value)
        }
    }
    impl ::core::convert::From<DeregisterOperatorCall> for BLSApkRegistryHarnessCalls {
        fn from(value: DeregisterOperatorCall) -> Self {
            Self::DeregisterOperator(value)
        }
    }
    impl ::core::convert::From<GetApkCall> for BLSApkRegistryHarnessCalls {
        fn from(value: GetApkCall) -> Self {
            Self::GetApk(value)
        }
    }
    impl ::core::convert::From<GetApkHashAtBlockNumberAndIndexCall> for BLSApkRegistryHarnessCalls {
        fn from(value: GetApkHashAtBlockNumberAndIndexCall) -> Self {
            Self::GetApkHashAtBlockNumberAndIndex(value)
        }
    }
    impl ::core::convert::From<GetApkHistoryLengthCall> for BLSApkRegistryHarnessCalls {
        fn from(value: GetApkHistoryLengthCall) -> Self {
            Self::GetApkHistoryLength(value)
        }
    }
    impl ::core::convert::From<GetApkIndicesAtBlockNumberCall> for BLSApkRegistryHarnessCalls {
        fn from(value: GetApkIndicesAtBlockNumberCall) -> Self {
            Self::GetApkIndicesAtBlockNumber(value)
        }
    }
    impl ::core::convert::From<GetApkUpdateAtIndexCall> for BLSApkRegistryHarnessCalls {
        fn from(value: GetApkUpdateAtIndexCall) -> Self {
            Self::GetApkUpdateAtIndex(value)
        }
    }
    impl ::core::convert::From<GetOperatorFromPubkeyHashCall> for BLSApkRegistryHarnessCalls {
        fn from(value: GetOperatorFromPubkeyHashCall) -> Self {
            Self::GetOperatorFromPubkeyHash(value)
        }
    }
    impl ::core::convert::From<GetOperatorIdCall> for BLSApkRegistryHarnessCalls {
        fn from(value: GetOperatorIdCall) -> Self {
            Self::GetOperatorId(value)
        }
    }
    impl ::core::convert::From<GetRegisteredPubkeyCall> for BLSApkRegistryHarnessCalls {
        fn from(value: GetRegisteredPubkeyCall) -> Self {
            Self::GetRegisteredPubkey(value)
        }
    }
    impl ::core::convert::From<InitializeQuorumCall> for BLSApkRegistryHarnessCalls {
        fn from(value: InitializeQuorumCall) -> Self {
            Self::InitializeQuorum(value)
        }
    }
    impl ::core::convert::From<OperatorToPubkeyCall> for BLSApkRegistryHarnessCalls {
        fn from(value: OperatorToPubkeyCall) -> Self {
            Self::OperatorToPubkey(value)
        }
    }
    impl ::core::convert::From<OperatorToPubkeyHashCall> for BLSApkRegistryHarnessCalls {
        fn from(value: OperatorToPubkeyHashCall) -> Self {
            Self::OperatorToPubkeyHash(value)
        }
    }
    impl ::core::convert::From<PubkeyHashToOperatorCall> for BLSApkRegistryHarnessCalls {
        fn from(value: PubkeyHashToOperatorCall) -> Self {
            Self::PubkeyHashToOperator(value)
        }
    }
    impl ::core::convert::From<RegisterBLSPublicKeyCall> for BLSApkRegistryHarnessCalls {
        fn from(value: RegisterBLSPublicKeyCall) -> Self {
            Self::RegisterBLSPublicKey(value)
        }
    }
    impl ::core::convert::From<RegisterOperatorCall> for BLSApkRegistryHarnessCalls {
        fn from(value: RegisterOperatorCall) -> Self {
            Self::RegisterOperator(value)
        }
    }
    impl ::core::convert::From<RegistryCoordinatorCall> for BLSApkRegistryHarnessCalls {
        fn from(value: RegistryCoordinatorCall) -> Self {
            Self::RegistryCoordinator(value)
        }
    }
    impl ::core::convert::From<SetBLSPublicKeyCall> for BLSApkRegistryHarnessCalls {
        fn from(value: SetBLSPublicKeyCall) -> Self {
            Self::SetBLSPublicKey(value)
        }
    }
    ///Container type for all return fields from the `apkHistory` function with signature `apkHistory(uint8,uint256)` and selector `0x7916cea6`
    #[derive(
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
    pub struct ApkHistoryReturn {
        pub apk_hash: [u8; 24],
        pub update_block_number: u32,
        pub next_update_block_number: u32,
    }
    ///Container type for all return fields from the `currentApk` function with signature `currentApk(uint8)` and selector `0xa3db80e2`
    #[derive(
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
    pub struct CurrentApkReturn {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getApk` function with signature `getApk(uint8)` and selector `0x5f61a884`
    #[derive(
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
    pub struct GetApkReturn(pub G1Point);
    ///Container type for all return fields from the `getApkHashAtBlockNumberAndIndex` function with signature `getApkHashAtBlockNumberAndIndex(uint8,uint32,uint256)` and selector `0x68bccaac`
    #[derive(
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
    pub struct GetApkHashAtBlockNumberAndIndexReturn(pub [u8; 24]);
    ///Container type for all return fields from the `getApkHistoryLength` function with signature `getApkHistoryLength(uint8)` and selector `0x377ed99d`
    #[derive(
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
    pub struct GetApkHistoryLengthReturn(pub u32);
    ///Container type for all return fields from the `getApkIndicesAtBlockNumber` function with signature `getApkIndicesAtBlockNumber(bytes,uint256)` and selector `0xd5254a8c`
    #[derive(
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
    pub struct GetApkIndicesAtBlockNumberReturn(pub ::std::vec::Vec<u32>);
    ///Container type for all return fields from the `getApkUpdateAtIndex` function with signature `getApkUpdateAtIndex(uint8,uint256)` and selector `0x605747d5`
    #[derive(
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
    pub struct GetApkUpdateAtIndexReturn(pub ApkUpdate);
    ///Container type for all return fields from the `getOperatorFromPubkeyHash` function with signature `getOperatorFromPubkeyHash(bytes32)` and selector `0x47b314e8`
    #[derive(
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
    pub struct GetOperatorFromPubkeyHashReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getOperatorId` function with signature `getOperatorId(address)` and selector `0x13542a4e`
    #[derive(
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
    pub struct GetOperatorIdReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getRegisteredPubkey` function with signature `getRegisteredPubkey(address)` and selector `0x7ff81a87`
    #[derive(
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
    pub struct GetRegisteredPubkeyReturn(pub G1Point, pub [u8; 32]);
    ///Container type for all return fields from the `operatorToPubkey` function with signature `operatorToPubkey(address)` and selector `0x00a1f4cb`
    #[derive(
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
    pub struct OperatorToPubkeyReturn {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `operatorToPubkeyHash` function with signature `operatorToPubkeyHash(address)` and selector `0xde29fac0`
    #[derive(
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
    pub struct OperatorToPubkeyHashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `pubkeyHashToOperator` function with signature `pubkeyHashToOperator(bytes32)` and selector `0xe8bb9ae6`
    #[derive(
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
    pub struct PubkeyHashToOperatorReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `registerBLSPublicKey` function with signature `registerBLSPublicKey(address,((uint256,uint256),(uint256,uint256),(uint256[2],uint256[2])),(uint256,uint256))` and selector `0xbf79ce58`
    #[derive(
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
    pub struct RegisterBLSPublicKeyReturn {
        pub operator_id: [u8; 32],
    }
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
}
