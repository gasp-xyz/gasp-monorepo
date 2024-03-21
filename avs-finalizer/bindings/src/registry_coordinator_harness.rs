pub use registry_coordinator_harness::*;
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
pub mod registry_coordinator_harness {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_serviceManager"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IServiceManager"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_stakeRegistry"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IStakeRegistry"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_blsApkRegistry"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IBLSApkRegistry"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_indexRegistry"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IIndexRegistry"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("IS_TEST"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("IS_TEST"),
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
                    ::std::borrow::ToOwned::to_owned("OPERATOR_CHURN_APPROVAL_TYPEHASH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OPERATOR_CHURN_APPROVAL_TYPEHASH",
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
                    ::std::borrow::ToOwned::to_owned("PUBKEY_REGISTRATION_TYPEHASH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PUBKEY_REGISTRATION_TYPEHASH",
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
                    ::std::borrow::ToOwned::to_owned("_deregisterOperatorExternal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "_deregisterOperatorExternal",
                            ),
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("_registerOperatorExternal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "_registerOperatorExternal",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumbers"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("socket"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operatorSignature"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ISignatureUtils.SignatureWithSaltAndExpiry",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("results"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                ),
                                            ),
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
                                            "struct RegistryCoordinator.RegisterResults",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("_updateOperatorBitmapExternal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "_updateOperatorBitmapExternal",
                            ),
                            inputs: ::std::vec![
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
                                    name: ::std::borrow::ToOwned::to_owned("quorumBitmap"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        192usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint192"),
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
                    ::std::borrow::ToOwned::to_owned("_updateOperatorExternal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "_updateOperatorExternal",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operatorInfo"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IRegistryCoordinator.OperatorInfo",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("quorumsToUpdate"),
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
                    ::std::borrow::ToOwned::to_owned(
                        "calculateOperatorChurnApprovalDigestHash",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateOperatorChurnApprovalDigestHash",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "registeringOperator",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "registeringOperatorId",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "operatorKickParams",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IRegistryCoordinator.OperatorKickParam[]",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("salt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("expiry"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("churnApprover"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("churnApprover"),
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
                    ::std::borrow::ToOwned::to_owned("createQuorum"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createQuorum"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operatorSetParams"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IRegistryCoordinator.OperatorSetParam",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("minimumStake"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint96"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("strategyParams"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IStakeRegistry.StrategyParams[]",
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
                    ::std::borrow::ToOwned::to_owned("deregisterOperator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deregisterOperator"),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("ejectOperator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ejectOperator"),
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ejector"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ejector"),
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
                    ::std::borrow::ToOwned::to_owned("excludeArtifacts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("excludeArtifacts"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "excludedArtifacts_",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("excludeContracts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("excludeContracts"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "excludedContracts_",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("excludeSenders"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("excludeSenders"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("excludedSenders_"),
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
                    ::std::borrow::ToOwned::to_owned("failed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("failed"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getCurrentQuorumBitmap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getCurrentQuorumBitmap",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operatorId"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        192usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint192"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getOperator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getOperator"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IRegistryCoordinator.OperatorInfo",
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
                    ::std::borrow::ToOwned::to_owned("getOperatorFromId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getOperatorFromId"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operatorId"),
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
                    ::std::borrow::ToOwned::to_owned("getOperatorId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getOperatorId"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("getOperatorSetParams"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getOperatorSetParams",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IRegistryCoordinator.OperatorSetParam",
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
                    ::std::borrow::ToOwned::to_owned("getOperatorStatus"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getOperatorStatus"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum IRegistryCoordinator.OperatorStatus",
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
                    ::std::borrow::ToOwned::to_owned(
                        "getQuorumBitmapAtBlockNumberByIndex",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getQuorumBitmapAtBlockNumberByIndex",
                            ),
                            inputs: ::std::vec![
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        192usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint192"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getQuorumBitmapHistoryLength"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getQuorumBitmapHistoryLength",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operatorId"),
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
                        "getQuorumBitmapIndicesAtBlockNumber",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getQuorumBitmapIndicesAtBlockNumber",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("blockNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getQuorumBitmapUpdateByIndex"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getQuorumBitmapUpdateByIndex",
                            ),
                            inputs: ::std::vec![
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
                                    name: ::std::borrow::ToOwned::to_owned("index"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(192usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IRegistryCoordinator.QuorumBitmapUpdate",
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
                    ::std::borrow::ToOwned::to_owned("indexRegistry"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("indexRegistry"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IIndexRegistry"),
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
                                    name: ::std::borrow::ToOwned::to_owned("_initialOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_churnApprover"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_ejector"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_pauserRegistry"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IPauserRegistry"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_initialPausedStatus",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_operatorSetParams",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IRegistryCoordinator.OperatorSetParam[]",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_minimumStakes"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint96[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_strategyParams"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IStakeRegistry.StrategyParams[][]",
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
                    ::std::borrow::ToOwned::to_owned("isChurnApproverSaltUsed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "isChurnApproverSaltUsed",
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
                    ::std::borrow::ToOwned::to_owned("numRegistries"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("numRegistries"),
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
                    ::std::borrow::ToOwned::to_owned("pubkeyRegistrationMessageHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "pubkeyRegistrationMessageHash",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("quorumCount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("quorumCount"),
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
                    ::std::borrow::ToOwned::to_owned("quorumUpdateBlockNumber"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "quorumUpdateBlockNumber",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("registerOperator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("registerOperator"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumbers"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("socket"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
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
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IBLSApkRegistry.PubkeyRegistrationParams",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operatorSignature"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ISignatureUtils.SignatureWithSaltAndExpiry",
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
                    ::std::borrow::ToOwned::to_owned("registerOperatorWithChurn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "registerOperatorWithChurn",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumbers"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("socket"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
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
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IBLSApkRegistry.PubkeyRegistrationParams",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "operatorKickParams",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IRegistryCoordinator.OperatorKickParam[]",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "churnApproverSignature",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ISignatureUtils.SignatureWithSaltAndExpiry",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operatorSignature"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ISignatureUtils.SignatureWithSaltAndExpiry",
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
                    ::std::borrow::ToOwned::to_owned("registries"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("registries"),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("serviceManager"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("serviceManager"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IServiceManager"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setChurnApprover"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setChurnApprover"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_churnApprover"),
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
                    ::std::borrow::ToOwned::to_owned("setEjector"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setEjector"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_ejector"),
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
                    ::std::borrow::ToOwned::to_owned("setOperatorId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setOperatorId"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setOperatorSetParams"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setOperatorSetParams",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operatorSetParams"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IRegistryCoordinator.OperatorSetParam",
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
                    ::std::borrow::ToOwned::to_owned("setQuorumCount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setQuorumCount"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("count"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
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
                    ::std::borrow::ToOwned::to_owned("targetArtifactSelectors"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "targetArtifactSelectors",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "targetedArtifactSelectors_",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct StdInvariant.FuzzSelector[]",
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
                    ::std::borrow::ToOwned::to_owned("targetArtifacts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("targetArtifacts"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "targetedArtifacts_",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("targetContracts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("targetContracts"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "targetedContracts_",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("targetInterfaces"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("targetInterfaces"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "targetedInterfaces_",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::String,
                                                        ),
                                                    ),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct StdInvariant.FuzzInterface[]",
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
                    ::std::borrow::ToOwned::to_owned("targetSelectors"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("targetSelectors"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "targetedSelectors_",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct StdInvariant.FuzzSelector[]",
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
                    ::std::borrow::ToOwned::to_owned("targetSenders"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("targetSenders"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("targetedSenders_"),
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
                    ::std::borrow::ToOwned::to_owned("updateOperators"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateOperators"),
                            inputs: ::std::vec![
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateOperatorsForQuorum"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateOperatorsForQuorum",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "operatorsPerQuorum",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ),
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[][]"),
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
                    ::std::borrow::ToOwned::to_owned("updateSocket"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateSocket"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("socket"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                    ::std::borrow::ToOwned::to_owned("ChurnApproverUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ChurnApproverUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("prevChurnApprover"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newChurnApprover"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EjectorUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("EjectorUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("prevEjector"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newEjector"),
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
                    ::std::borrow::ToOwned::to_owned("OperatorDeregistered"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OperatorDeregistered",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("operatorId"),
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
                    ::std::borrow::ToOwned::to_owned("OperatorRegistered"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("OperatorRegistered"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("operatorId"),
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
                    ::std::borrow::ToOwned::to_owned("OperatorSetParamsUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OperatorSetParamsUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("operatorSetParams"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
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
                    ::std::borrow::ToOwned::to_owned("OperatorSocketUpdate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OperatorSocketUpdate",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("socket"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
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
                    ::std::borrow::ToOwned::to_owned("QuorumBlockNumberUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "QuorumBlockNumberUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("blocknumber"),
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
                (
                    ::std::borrow::ToOwned::to_owned("log"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_address"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_address"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_array"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_array"),
                            inputs: ::std::vec![
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
                            name: ::std::borrow::ToOwned::to_owned("log_array"),
                            inputs: ::std::vec![
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
                            name: ::std::borrow::ToOwned::to_owned("log_array"),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("log_bytes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_bytes"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_bytes32"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_bytes32"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_int"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_int"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_address"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
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
                        },
                    ],
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
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_bytes32"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_bytes32"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_decimal_int"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "log_named_decimal_int",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("log_named_decimal_uint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "log_named_decimal_uint",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("decimals"),
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
                    ::std::borrow::ToOwned::to_owned("log_named_int"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_string"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_named_uint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_named_uint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("val"),
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
                    ::std::borrow::ToOwned::to_owned("log_string"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_string"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("log_uint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("log_uint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("logs"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("logs"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
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
    pub static REGISTRYCOORDINATORHARNESS_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01\xC0`@R`\xCF\x80T`\x01`\xFF\x19\x91\x82\x16\x81\x17\x90\x92U`\xD3\x80T\x90\x91\x16\x90\x91\x17\x90U4\x80\x15b\0\0/W`\0\x80\xFD[P`@Qb\0rL8\x03\x80b\0rL\x839\x81\x01`@\x81\x90Rb\0\0R\x91b\0\x02\xC0V[`@\x80Q\x80\x82\x01\x82R`\x16\x81R\x7FAVSRegistryCoordinator\0\0\0\0\0\0\0\0\0\0` \x80\x83\x01\x91\x82R\x83Q\x80\x85\x01\x85R`\x06\x81Rev0.0.1`\xD0\x1B\x90\x82\x01R\x91Q\x90 `\xE0\x81\x90R\x7Fk\xDA~?8^H\x84\x10H9\x04D\xCC\xED\\\xC7\x95\xAF\x87u\x8A\xF6v\"\xE5\xF4\xF0\x88,J\x99a\x01\0\x81\x90RF`\xA0\x81\x81R\x85Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81\x87\x01\x81\x90R\x81\x88\x01\x95\x90\x95R``\x81\x01\x93\x90\x93R`\x80\x80\x84\x01\x92\x90\x92R0\x83\x82\x01\x81\x90R\x86Q\x80\x85\x03\x90\x92\x01\x82R`\xC0\x93\x84\x01\x90\x96R\x80Q\x94\x01\x93\x90\x93 \x90\x92R\x91\x90Ra\x01 R`\x01`\x01`\xA0\x1B\x03\x80\x85\x16a\x01@R\x80\x84\x16a\x01\x80R\x80\x83\x16a\x01`R\x81\x16a\x01\xA0R\x83\x83\x83\x83b\0\x01tb\0\x01\x93V[PPPPb\0\x01\x893b\0\x02U` \x1B` \x1CV[PPPPb\0\x03(V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15b\0\x02\0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\xFF\x90\x81\x16\x10\x15b\0\x02SW`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`d\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x02\xBDW`\0\x80\xFD[PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15b\0\x02\xD7W`\0\x80\xFD[\x84Qb\0\x02\xE4\x81b\0\x02\xA7V[` \x86\x01Q\x90\x94Pb\0\x02\xF7\x81b\0\x02\xA7V[`@\x86\x01Q\x90\x93Pb\0\x03\n\x81b\0\x02\xA7V[``\x86\x01Q\x90\x92Pb\0\x03\x1D\x81b\0\x02\xA7V[\x93\x96\x92\x95P\x90\x93PPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x01@Qa\x01`Qa\x01\x80Qa\x01\xA0Qan\x1Cb\0\x040`\09`\0\x81\x81a\x087\x01R\x81\x81a\x16\xBE\x01R\x81\x81a*Z\x01R\x81\x81a5\xAD\x01R\x81\x81a?\xDF\x01RaH\xAD\x01R`\0\x81\x81a\x07L\x01R\x81\x81a)\xE5\x01R\x81\x81a.\xF3\x01R\x81\x81a5\x04\x01R\x81\x81a?_\x01R\x81\x81aD\x82\x01RaH,\x01R`\0\x81\x81a\x06\xFD\x01R\x81\x81a\x11\xDE\x01R\x81\x81a*#\x01R\x81\x81a4\x84\x01R\x81\x81a>\xE1\x01R\x81\x81a@\xC7\x01R\x81\x81aA=\x01RaI)\x01R`\0\x81\x81a\x061\x01R\x81\x81a3\xCC\x01Ra>7\x01R`\0aK,\x01R`\0aK{\x01R`\0aKV\x01R`\0aJ\xAF\x01R`\0aJ\xD9\x01R`\0aK\x03\x01Ran\x1C`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x03\xCFW`\x005`\xE0\x1C\x80cf\xD9\xA9\xA0\x11a\x01\xFFW\x80c\xB5P\x8A\xA9\x11a\x01\x1AW\x80c\xD9,\xBB\x84\x11a\0\xADW\x80c\xF2\xFD\xE3\x8B\x11a\0|W\x80c\xF2\xFD\xE3\x8B\x14a\n$W\x80c\xFAv&\xD4\x14a\n7W\x80c\xFA\xBC\x1C\xBC\x14a\nDW\x80c\xFD9\x10Z\x14a\nWW`\0\x80\xFD[\x80c\xD9,\xBB\x84\x14a\t<W\x80c\xDD\x82\x83\xF3\x14a\tfW\x80c\xE2\x0C\x9Fq\x14a\tyW\x80c\xE6W\x97\xAD\x14a\t\x81W`\0\x80\xFD[\x80c\xCA\r\xE8\x82\x11a\0\xE9W\x80c\xCA\r\xE8\x82\x14a\x08\xE7W\x80c\xCAO-\x97\x14a\t\x0EW\x80c\xD7-\x8D\xD6\x14a\t!W\x80c\xD7[L\x88\x14a\t)W`\0\x80\xFD[\x80c\xB5P\x8A\xA9\x14a\x08\x93W\x80c\xBAAO\xA6\x14a\x08\x9BW\x80c\xC3\x91B^\x14a\x08\xA3W\x80c\xC4\t}^\x14a\x08\xC3W`\0\x80\xFD[\x80c\x88o\x11\x95\x11a\x01\x92W\x80c\x9B]\x17{\x11a\x01aW\x80c\x9B]\x17{\x14a\x08\x1FW\x80c\x9E\x99#\xC2\x14a\x082W\x80c\x9F\xEA\xB8Y\x14a\x08YW\x80c\xA5\x08W\xBF\x14a\x08\x80W`\0\x80\xFD[\x80c\x88o\x11\x95\x14a\x07\xD7W\x80c\x8D\xA5\xCB[\x14a\x07\xF0W\x80c\x91j\x17\xC6\x14a\x07\xF8W\x80c\x9A\xA1e=\x14a\x08\0W`\0\x80\xFD[\x80c\x83\x10\xFE\xF6\x11a\x01\xCEW\x80c\x83\x10\xFE\xF6\x14a\x07\x89W\x80c\x84\xCAR\x13\x14a\x07\x9CW\x80c\x85\"l\x81\x14a\x07\xAFW\x80c\x87\x1E\xF0I\x14a\x07\xC4W`\0\x80\xFD[\x80cf\xD9\xA9\xA0\x14a\x072W\x80ch0H5\x14a\x07GW\x80cn;\x17\xDB\x14a\x07nW\x80cqP\x18\xA6\x14a\x07\x81W`\0\x80\xFD[\x80c)k\xB0d\x11a\x02\xEFW\x80cQ@\xA5H\x11a\x02\x82W\x80c[\x0B\x82\x9F\x11a\x02QW\x80c[\x0B\x82\x9F\x14a\x06\xDDW\x80c\\\x97Z\xBB\x14a\x06\xF0W\x80c]\xF4YF\x14a\x06\xF8W\x80ccG\xC9\0\x14a\x07\x1FW`\0\x80\xFD[\x80cQ@\xA5H\x14a\x06\x83W\x80cXe\xC6\x0C\x14a\x06\x96W\x80cY\\jg\x14a\x06\xB6W\x80cZ\xC8j\xB7\x14a\x06\xBEW`\0\x80\xFD[\x80c9\x98\xFD\xD3\x11a\x02\xBEW\x80c9\x98\xFD\xD3\x14a\x06,W\x80c<*\x7FL\x14a\x06SW\x80c>^<#\x14a\x06sW\x80c?r\x86\xF4\x14a\x06{W`\0\x80\xFD[\x80c)k\xB0d\x14a\x05\xDEW\x80c)\xD1\xE0\xC3\x14a\x05\xF1W\x80c*\xDE8\x80\x14a\x06\x04W\x80c,\xDD\x1E\x86\x14a\x06\x19W`\0\x80\xFD[\x80c\x14x\x85\x1F\x11a\x03gW\x80c$\x9A\x0CB\x11a\x036W\x80c$\x9A\x0CB\x14a\x05\x85W\x80c'\xE7\x92\x88\x14a\x05\xA5W\x80c(\xF6\x1B1\x14a\x05\xB8W\x80c)ST|\x14a\x05\xCBW`\0\x80\xFD[\x80c\x14x\x85\x1F\x14a\x04\xD4W\x80c\x1A\xB2WO\x14a\x05\x07W\x80c\x1E\xB8\x12\xDA\x14a\x05'W\x80c\x1E\xD7\x83\x1C\x14a\x05pW`\0\x80\xFD[\x80c\x0C\xF4\xB7g\x11a\x03\xA3W\x80c\x0C\xF4\xB7g\x14a\x04rW\x80c\x10\xD6z/\x14a\x04\x85W\x80c\x13T*N\x14a\x04\x98W\x80c\x13d9\xDD\x14a\x04\xC1W`\0\x80\xFD[\x80b\xCF*\xB5\x14a\x03\xD4W\x80c\x03\xFD4\x92\x14a\x03\xE9W\x80c\x04\xECcQ\x14a\x04\x1CW\x80c\x05C\x10\xE6\x14a\x04GW[`\0\x80\xFD[a\x03\xE7a\x03\xE26`\x04aTlV[a\n\x93V[\0[a\x04\ta\x03\xF76`\x04aT\xADV[`\0\x90\x81R`\x98` R`@\x90 T\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x04/a\x04*6`\x04aT\xD8V[a\x0B\xA9V[`@Q`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x81R` \x01a\x04\x13V[`\x9DTa\x04Z\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x04\x13V[a\x03\xE7a\x04\x806`\x04aV\x0FV[a\r\x9FV[a\x03\xE7a\x04\x936`\x04aVkV[a\x0E\x87V[a\x04\ta\x04\xA66`\x04aVkV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x99` R`@\x90 T\x90V[a\x03\xE7a\x04\xCF6`\x04aT\xADV[a\x0F:V[a\x04\xF7a\x04\xE26`\x04aT\xADV[`\x9A` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x04\x13V[a\x05\x1Aa\x05\x156`\x04aW$V[a\x10wV[`@Qa\x04\x13\x91\x90aX\x10V[a\x05:a\x0556`\x04aX\x98V[a\x10\xB4V[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x80\x85\x01Q\x90\x91\x16\x90\x82\x01R\x91\x81\x01Q`\x01`\x01`\xC0\x1B\x03\x16\x90\x82\x01R``\x01a\x04\x13V[a\x05xa\x11EV[`@Qa\x04\x13\x91\x90aX\xBAV[a\x04\ta\x05\x936`\x04aY\x18V[`\x9B` R`\0\x90\x81R`@\x90 T\x81V[a\x03\xE7a\x05\xB36`\x04aYHV[a\x11\xA7V[`\x9ETa\x04Z\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03\xE7a\x05\xD96`\x04aYxV[a\x11\xB5V[a\x04Za\x05\xEC6`\x04aT\xADV[a\x11\xC5V[a\x03\xE7a\x05\xFF6`\x04aVkV[a\x12QV[a\x06\x0Ca\x12bV[`@Qa\x04\x13\x91\x90aZ\xB2V[a\x03\xE7a\x06'6`\x04aVkV[a\x13\xA4V[a\x04Z\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x06fa\x06a6`\x04aVkV[a\x13\xB5V[`@Qa\x04\x13\x91\x90a[/V[a\x05xa\x144V[a\x05xa\x14\x94V[a\x03\xE7a\x06\x916`\x04a[FV[a\x14\xF4V[a\x06\xA9a\x06\xA46`\x04aVkV[a\x1A~V[`@Qa\x04\x13\x91\x90a[\xE9V[a\x03\xE7a\x1A\xF2V[a\x04\xF7a\x06\xCC6`\x04aY\x18V[`\x01\x80T`\xFF\x90\x92\x16\x1B\x90\x81\x16\x14\x90V[a\x03\xE7a\x06\xEB6`\x04a\\nV[a\x1B\xBEV[`\x01Ta\x04\tV[a\x04Z\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x04Za\x07-6`\x04aT\xADV[a\x1CPV[a\x07:a\x1CzV[`@Qa\x04\x13\x91\x90a\\\xA2V[a\x04Z\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\xE7a\x07|6`\x04a]UV[a\x1D`V[a\x03\xE7a\x1E V[a\x03\xE7a\x07\x976`\x04a]UV[a\x1D\xE0V[a\x04\ta\x07\xAA6`\x04a^\x0CV[a\x1E4V[a\x07\xB7a\x1E~V[`@Qa\x04\x13\x91\x90a^\xD9V[a\x04/a\x07\xD26`\x04aT\xADV[a\x1FNV[`\0Ta\x04Z\x90b\x01\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x04Za\x1FYV[a\x07:a\x1FrV[`\x96Ta\x08\r\x90`\xFF\x16\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x04\x13V[a\x03\xE7a\x08-6`\x04a_CV[a XV[a\x04Z\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x04\t\x7F+\xD8!$\x05\x7F\t\x13\xBC;w,\xE7\xB8>\x80W\xC1\xAD\x1F5\x10\xFC\x83w\x8B\xE2\x0F\x10\xEC]\xE6\x81V[a\x03\xE7a\x08\x8E6`\x04a`<V[a#\x90V[a\x07\xB7a%\x14V[a\x04\xF7a%\xE4V[a\x08\xB6a\x08\xB16`\x04a`\xCAV[a'\x11V[`@Qa\x04\x13\x91\x90aaoV[a\x03\xE7a\x08\xD16`\x04aY\x18V[`\x96\x80T`\xFF\x19\x16`\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x04\t\x7F;\xFES\x90\xD1h\x8D.\x05n\x0E\xE4\xB6\x8CT\x19\xF4-\x9B\xD0\xC7\x92\xF0\x1A\xCB\x83\x88g\x14\xAD\xEBV\x81V[a\x03\xE7a\t\x1C6`\x04aa\xADV[a'\xCAV[`\x9CTa\x04\tV[a\x03\xE7a\t76`\x04ab\x93V[a(1V[a\x03\xE7a\tJ6`\x04ab\xE9V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\0\x90\x81R`\x99` R`@\x90 UV[a\x03\xE7a\tt6`\x04adhV[a(DV[a\x05xa+HV[a\t\xF0a\t\x8F6`\x04aY\x18V[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x92\x84\x01\x81\x90R`\xFF\x94\x90\x94\x16\x84R`\x97\x82R\x92\x82\x90 \x82Q\x93\x84\x01\x83RTc\xFF\xFF\xFF\xFF\x81\x16\x84Ra\xFF\xFF`\x01` \x1B\x82\x04\x81\x16\x92\x85\x01\x92\x90\x92R`\x01`0\x1B\x90\x04\x16\x90\x82\x01R\x90V[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x16\x81R` \x80\x84\x01Qa\xFF\xFF\x90\x81\x16\x91\x83\x01\x91\x90\x91R\x92\x82\x01Q\x90\x92\x16\x90\x82\x01R``\x01a\x04\x13V[a\x03\xE7a\n26`\x04aVkV[a+\xA8V[`\xCFTa\x04\xF7\x90`\xFF\x16\x81V[a\x03\xE7a\nR6`\x04aT\xADV[a,\x1EV[a\n\x86a\ne6`\x04aVkV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T`\xFF\x16\x90V[`@Qa\x04\x13\x91\x90ae<V[`\x01T`\x02\x90`\x04\x90\x81\x16\x14\x15a\n\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\xBC\x90aeJV[`@Q\x80\x91\x03\x90\xFD[`\0[\x82\x81\x10\x15a\x0B\xA3W`\0\x84\x84\x83\x81\x81\x10a\n\xE4Wa\n\xE4ae\x81V[\x90P` \x02\x01` \x81\x01\x90a\n\xF9\x91\x90aVkV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x99` \x90\x81R`@\x80\x83 \x81Q\x80\x83\x01\x90\x92R\x80T\x82R`\x01\x81\x01T\x94\x95P\x92\x93\x90\x92\x91\x83\x01\x90`\xFF\x16`\x02\x81\x11\x15a\x0BDWa\x0BDa[\xB1V[`\x02\x81\x11\x15a\x0BUWa\x0BUa[\xB1V[\x90RP\x80Q\x90\x91P`\0a\x0Bh\x82a-zV[\x90P`\0a\x0B~\x82`\x01`\x01`\xC0\x1B\x03\x16a-\xE9V[\x90Pa\x0B\x8B\x85\x85\x83a.\xB5V[PPPPP\x80\x80a\x0B\x9B\x90ae\xADV[\x91PPa\n\xC8V[PPPPV[`\0\x83\x81R`\x98` R`@\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x0B\xCCWa\x0B\xCCae\x81V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x85R`\x01` \x1B\x83\x04\x82\x16\x95\x85\x01\x95\x90\x95R`\x01`@\x1B\x90\x91\x04`\x01`\x01`\xC0\x1B\x03\x16\x91\x83\x01\x91\x90\x91R\x90\x92P\x85\x16\x10\x15a\x0C\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`e`$\x82\x01R\x7FRegistryCoordinator.getQuorumBit`D\x82\x01R\x7FmapAtBlockNumberByIndex: quorumB`d\x82\x01R\x7FitmapUpdate is from after blockN`\x84\x82\x01Rd:\xB6\xB12\xB9`\xD9\x1B`\xA4\x82\x01R`\xC4\x01a\n\xBCV[` \x81\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x80a\x0C\xECWP\x80` \x01Qc\xFF\xFF\xFF\xFF\x16\x84c\xFF\xFF\xFF\xFF\x16\x10[a\r\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`f`$\x82\x01R\x7FRegistryCoordinator.getQuorumBit`D\x82\x01R\x7FmapAtBlockNumberByIndex: quorumB`d\x82\x01R\x7FitmapUpdate is from before block`\x84\x82\x01Re':\xB6\xB12\xB9`\xD1\x1B`\xA4\x82\x01R`\xC4\x01a\n\xBCV[`@\x01Q\x94\x93PPPPV[`\x013`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T`\xFF\x16`\x02\x81\x11\x15a\r\xC8Wa\r\xC8a[\xB1V[\x14a\x0E;W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R\x7FRegistryCoordinator.updateSocket`D\x82\x01R\x7F: operator is not registered\0\0\0\0`d\x82\x01R`\x84\x01a\n\xBCV[3`\0\x90\x81R`\x99` R`@\x90\x81\x90 T\x90Q\x7F\xEC)c\xAB!\xC1\xE5\x0E\x1EX*\xA5B\xAF.K\xF7\xBF8\xE6\xE1@<'\xB4.\x1C]nb\x1E\xAA\x90a\x0E|\x90\x84\x90ae\xC8V[`@Q\x80\x91\x03\x90\xA2PV[`\0`\x02\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xDAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xFE\x91\x90ae\xDBV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0F.W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\xBC\x90ae\xF8V[a\x0F7\x81a/\xA2V[PV[`\0T`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01Rb\x01\0\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x87W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xAB\x91\x90afBV[a\x0F\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\xBC\x90afdV[`\x01T\x81\x81\x16\x14a\x10@W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\n\xBCV[`\x01\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01a\x0E|V[a\x10\x9B`@Q\x80``\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81RP\x90V[a\x10\xA9\x87\x87\x87\x87\x87\x87a0\xA7V[\x97\x96PPPPPPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R`\0\x83\x81R`\x98` R`@\x90 \x80T\x83\x90\x81\x10a\x10\xF1Wa\x10\xF1ae\x81V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01`\xC0\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90P[\x92\x91PPV[```\xDC\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x11\x9DW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x11\x7FW[PPPPP\x90P\x90V[a\x11\xB1\x82\x82a6;V[PPV[a\x11\xC0\x83\x83\x83a.\xB5V[PPPV[`@Qc\x08\xF6b\x9D`\xE3\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cG\xB3\x14\xE8\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12-W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11?\x91\x90ae\xDBV[a\x12Ya7\xFBV[a\x0F7\x81a8ZV[```\xE3\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x13\x9BW`\0\x84\x81R` \x80\x82 `@\x80Q\x80\x82\x01\x82R`\x02\x87\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x95\x91\x94\x86\x81\x01\x94\x91\x92\x90\x84\x01[\x82\x82\x10\x15a\x13\x84W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x12\xF7\x90af\xACV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x13#\x90af\xACV[\x80\x15a\x13pW\x80`\x1F\x10a\x13EWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x13pV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x13SW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x12\xD8V[PPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x12\x86V[PPPP\x90P\x90V[a\x13\xACa7\xFBV[a\x0F7\x81a8\xC3V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x11?a\x14/\x7F+\xD8!$\x05\x7F\t\x13\xBC;w,\xE7\xB8>\x80W\xC1\xAD\x1F5\x10\xFC\x83w\x8B\xE2\x0F\x10\xEC]\xE6\x84`@Q` \x01a\x14\x14\x92\x91\x90\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a9,V[a9zV[```\xDE\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x11\x9DW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x11\x7FWPPPPP\x90P\x90V[```\xDD\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x11\x9DW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x11\x7FWPPPPP\x90P\x90V[`\x01T`\x02\x90`\x04\x90\x81\x16\x14\x15a\x15\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\xBC\x90aeJV[`\0a\x15e\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`\x96T`\xFF\x16\x91Pa:\n\x90PV[\x90Pa\x15p\x81a:\xC3V[a\x15\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R`\0\x80Q` amg\x839\x81Q\x91R`D\x82\x01R\x7ForsForQuorum: some quorums do no`d\x82\x01Rf\x1D\x08\x19^\x1A\\\xDD`\xCA\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[\x84\x83\x14a\x16OW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R`\0\x80Q` amg\x839\x81Q\x91R`D\x82\x01R\x7ForsForQuorum: input length misma`d\x82\x01Rb\x0E\x8Cm`\xEB\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[`\0[\x83\x81\x10\x15a\x1AuW`\0\x85\x85\x83\x81\x81\x10a\x16nWa\x16nae\x81V[\x91\x90\x91\x015`\xF8\x1C\x91P6\x90P`\0\x89\x89\x85\x81\x81\x10a\x16\x8FWa\x16\x8Fae\x81V[\x90P` \x02\x81\x01\x90a\x16\xA1\x91\x90af\xE1V[`@Qcy\xA0\x84\x91`\xE1\x1B\x81R`\xFF\x86\x16`\x04\x82\x01R\x91\x93P\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xF3A\t\"\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x171\x91\x90ag*V[c\xFF\xFF\xFF\xFF\x16\x81\x14a\x17\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`e`$\x82\x01R`\0\x80Q` amg\x839\x81Q\x91R`D\x82\x01R\x7ForsForQuorum: number of updated `d\x82\x01R\x7Foperators does not match quorum `\x84\x82\x01Rd\x1D\x1B\xDD\x18[`\xDA\x1B`\xA4\x82\x01R`\xC4\x01a\n\xBCV[`\0\x80[\x82\x81\x10\x15a\x1A\x14W`\0\x84\x84\x83\x81\x81\x10a\x17\xEDWa\x17\xEDae\x81V[\x90P` \x02\x01` \x81\x01\x90a\x18\x02\x91\x90aVkV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x99` \x90\x81R`@\x80\x83 \x81Q\x80\x83\x01\x90\x92R\x80T\x82R`\x01\x81\x01T\x94\x95P\x92\x93\x90\x92\x91\x83\x01\x90`\xFF\x16`\x02\x81\x11\x15a\x18MWa\x18Ma[\xB1V[`\x02\x81\x11\x15a\x18^Wa\x18^a[\xB1V[\x90RP\x80Q\x90\x91P`\0a\x18q\x82a-zV[\x90P`\x01`\x01`\x01`\xC0\x1B\x03\x82\x16`\xFF\x8B\x16\x1C\x81\x16\x14a\x18\xF5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R`\0\x80Q` amg\x839\x81Q\x91R\x90\x82\x01R\x7ForsForQuorum: operator not in qu`d\x82\x01Rcorum`\xE0\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[\x85`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x11a\x19\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`g`$\x82\x01R`\0\x80Q` amg\x839\x81Q\x91R`D\x82\x01R\x7ForsForQuorum: operators array mu`d\x82\x01R\x7Fst be sorted in ascending addres`\x84\x82\x01Rf9\x907\xB922\xB9`\xC9\x1B`\xA4\x82\x01R`\xC4\x01a\n\xBCV[Pa\x19\xFE\x83\x83\x8F\x8F\x8D\x90\x8E`\x01a\x19\xB7\x91\x90agGV[\x92a\x19\xC4\x93\x92\x91\x90ag_V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa.\xB5\x92PPPV[P\x90\x92Pa\x1A\r\x90P\x81ae\xADV[\x90Pa\x17\xD1V[P`\xFF\x84\x16`\0\x81\x81R`\x9B` \x90\x81R`@\x91\x82\x90 C\x90\x81\x90U\x91Q\x91\x82R\x7FF\x07}U3\x07c\xF1bi\xFDu\xE5v\x16c\xF4\x19-'\x91t|\x01\x89\xB1j\xD3\x1D\xB0}\xB4\x91\x01`@Q\x80\x91\x03\x90\xA2PPPP\x80a\x1An\x90ae\xADV[\x90Pa\x16RV[PPPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x99` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x81\x01T\x90\x91\x83\x01\x90`\xFF\x16`\x02\x81\x11\x15a\x1A\xD8Wa\x1A\xD8a[\xB1V[`\x02\x81\x11\x15a\x1A\xE9Wa\x1A\xE9a[\xB1V[\x90RP\x92\x91PPV[`\0T`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01Rb\x01\0\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B?W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Bc\x91\x90afBV[a\x1B\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\xBC\x90afdV[`\0\x19`\x01\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[a\x1B\xC6a7\xFBV[`\x96T\x82\x90`\xFF\x90\x81\x16\x90\x82\x16\x10a\x1CFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FRegistryCoordinator.quorumExists`D\x82\x01R\x7F: quorum does not exist\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\n\xBCV[a\x11\xC0\x83\x83a:\xF6V[`\x9C\x81\x81T\x81\x10a\x1C`W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x81V[```\xE1\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x13\x9BW`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x1DHW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x1D\nW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x1C\x9EV[`\x9ET`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1D\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FRegistryCoordinator.onlyEjector:`D\x82\x01R\x7F caller is not the ejector\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\n\xBCV[a\x11\xC0\x83\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa;\xA3\x92PPPV[a\x1E(a7\xFBV[a\x1E2`\0a@SV[V[`\0a\x1Et\x7F;\xFES\x90\xD1h\x8D.\x05n\x0E\xE4\xB6\x8CT\x19\xF4-\x9B\xD0\xC7\x92\xF0\x1A\xCB\x83\x88g\x14\xAD\xEBV\x87\x87\x87\x87\x87`@Q` \x01a\x14\x14\x96\x95\x94\x93\x92\x91\x90ag\x89V[\x96\x95PPPPPPV[```\xE0\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x13\x9BW\x83\x82\x90`\0R` `\0 \x01\x80Ta\x1E\xC1\x90af\xACV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1E\xED\x90af\xACV[\x80\x15a\x1F:W\x80`\x1F\x10a\x1F\x0FWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1F:V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1F\x1DW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x1E\xA2V[`\0a\x11?\x82a-zV[`\0a\x1Fm`dT`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x90P\x90V[```\xE2\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x13\x9BW`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a @W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a \x02W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x1F\x96V[`\x01\x80T`\0\x91\x90\x81\x16\x14\x15a \x80W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\xBC\x90aeJV[\x83\x89\x14a!\x03W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FRegistryCoordinator.registerOper\x90\x82\x01R\x7FatorWithChurn: input length mism`d\x82\x01Rc\x0C.\x8Cm`\xE3\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[`\0a!\x0F3\x88a@\xA5V[\x90Pa!o3\x82\x88\x88\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a!dWa!U`@\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90ah\x0EV[\x81R` \x01\x90`\x01\x01\x90a!8V[PPPPP\x87aA\xD6V[`\0a!\xB63\x83\x8E\x8E\x8E\x8E\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8C\x92Pa0\xA7\x91PPV[\x90P`\0[\x8B\x81\x10\x15a#\x81W`\0`\x97`\0\x8F\x8F\x85\x81\x81\x10a!\xDBWa!\xDBae\x81V[\x91\x90\x91\x015`\xF8\x1C\x82RP` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0 \x81Q``\x81\x01\x83R\x90Tc\xFF\xFF\xFF\xFF\x81\x16\x80\x83Ra\xFF\xFF`\x01` \x1B\x83\x04\x81\x16\x95\x84\x01\x95\x90\x95R`\x01`0\x1B\x90\x91\x04\x90\x93\x16\x91\x81\x01\x91\x90\x91R\x84Q\x80Q\x91\x93P\x90\x84\x90\x81\x10a\"HWa\"Hae\x81V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a#nWa\"\xE9\x8E\x8E\x84\x81\x81\x10a\"qWa\"qae\x81V[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x84`@\x01Q\x84\x81Q\x81\x10a\"\x94Wa\"\x94ae\x81V[` \x02` \x01\x01Q3\x86` \x01Q\x86\x81Q\x81\x10a\"\xB3Wa\"\xB3ae\x81V[` \x02` \x01\x01Q\x8D\x8D\x88\x81\x81\x10a\"\xCDWa\"\xCDae\x81V[\x90P`@\x02\x01\x806\x03\x81\x01\x90a\"\xE3\x91\x90ah\x0EV[\x86aCcV[a#n\x89\x89\x84\x81\x81\x10a\"\xFEWa\"\xFEae\x81V[\x90P`@\x02\x01` \x01` \x81\x01\x90a#\x16\x91\x90aVkV[\x8F\x8F\x85\x90\x86`\x01a#'\x91\x90agGV[\x92a#4\x93\x92\x91\x90ag_V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa;\xA3\x92PPPV[P\x80a#y\x81ae\xADV[\x91PPa!\xBBV[PPPPPPPPPPPPPV[`\x01\x80T`\0\x91\x90\x81\x16\x14\x15a#\xB8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\xBC\x90aeJV[`\0a#\xC43\x85a@\xA5V[\x90P`\0a$\r3\x83\x8B\x8B\x8B\x8B\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8C\x92Pa0\xA7\x91PPV[Q\x90P`\0[\x88\x81\x10\x15a%\x08W`\0\x8A\x8A\x83\x81\x81\x10a$/Wa$/ae\x81V[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x97` R`@\x90 T\x85Q\x91\x93Pc\xFF\xFF\xFF\xFF\x16\x91P\x84\x90\x84\x90\x81\x10a$eWa$eae\x81V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a$\xF5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FRegistryCoordinator.registerOper\x90\x82\x01R\x7Fator: operator count exceeds max`d\x82\x01Rcimum`\xE0\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[P\x80a%\0\x81ae\xADV[\x91PPa$\x13V[PPPPPPPPPPV[```\xDF\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x13\x9BW\x83\x82\x90`\0R` `\0 \x01\x80Ta%W\x90af\xACV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta%\x83\x90af\xACV[\x80\x15a%\xD0W\x80`\x1F\x10a%\xA5Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a%\xD0V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a%\xB3W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a%8V[`\xCFT`\0\x90a\x01\0\x90\x04`\xFF\x16\x15a&\x06WP`\xCFTa\x01\0\x90\x04`\xFF\x16\x90V[`\0sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a'\x0CW`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R`\0\x92\x90\x91a&\x94\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01ah*V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra&\xAE\x91ah[V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a&\xEBW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a&\xF0V[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90a'\x08\x91\x90afBV[\x91PP[\x91\x90PV[```\0\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a'.Wa'.aU\x10V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a'WW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a'\xC2Wa'\x89\x85\x85\x83\x81Q\x81\x10a'|Wa'|ae\x81V[` \x02` \x01\x01QaF8V[\x82\x82\x81Q\x81\x10a'\x9BWa'\x9Bae\x81V[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a'\xBA\x81ae\xADV[\x91PPa']V[P\x93\x92PPPV[`\x01\x80T`\x02\x90\x81\x16\x14\x15a'\xF1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\xBC\x90aeJV[a\x11\xC03\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa;\xA3\x92PPPV[a(9a7\xFBV[a\x11\xC0\x83\x83\x83aGtV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a(dWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a(~WP0;\x15\x80\x15a(~WP`\0T`\xFF\x16`\x01\x14[a(\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\n\xBCV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a)\x04W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[\x82Q\x84Q\x14\x80\x15a)\x16WP\x81Q\x83Q\x14[a)\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FRegistryCoordinator.initialize: `D\x82\x01Rt\r-\xCE\x0E\xAE\x84\r\x8C\xAD\xCC\xEE\x8D\x04\r\xAD.m\xAC.\x8Cm`[\x1B`d\x82\x01R`\x84\x01a\n\xBCV[a)\x89\x89a@SV[a)\x93\x86\x86aI\x8BV[a)\x9C\x88a8ZV[a)\xA5\x87a8\xC3V[`\x9C\x80T`\x01\x81\x81\x01\x83U`\0\x83\x81R\x7F\xAF\x85\xB9\x07\x1D\xFA\xFE\xAC\x14\t\xD3\xF1\xD1\x9B\xAF\xC9\xBC|7\x97L\xDE\x8D\xF0\xEEah\xF0\x08nS\x9C\x92\x83\x01\x80T`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U\x85T\x80\x85\x01\x87U\x85\x01\x80T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x16\x90\x83\x16\x17\x90U\x85T\x93\x84\x01\x90\x95U\x91\x90\x92\x01\x80T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x16\x91\x90\x93\x16\x17\x90\x91U[\x84Q\x81\x10\x15a*\xF6Wa*\xE4\x85\x82\x81Q\x81\x10a*\xA3Wa*\xA3ae\x81V[` \x02` \x01\x01Q\x85\x83\x81Q\x81\x10a*\xBDWa*\xBDae\x81V[` \x02` \x01\x01Q\x85\x84\x81Q\x81\x10a*\xD7Wa*\xD7ae\x81V[` \x02` \x01\x01QaGtV[\x80a*\xEE\x81ae\xADV[\x91PPa*\x85V[P\x80\x15a+=W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPPPV[```\xDB\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x11\x9DW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x11\x7FWPPPPP\x90P\x90V[a+\xB0a7\xFBV[`\x01`\x01`\xA0\x1B\x03\x81\x16a,\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\n\xBCV[a\x0F7\x81a@SV[`\0`\x02\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,qW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\x95\x91\x90ae\xDBV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a,\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\xBC\x90ae\xF8V[`\x01T\x19\x81\x19`\x01T\x19\x16\x14a-CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\n\xBCV[`\x01\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x0E|V[`\0\x81\x81R`\x98` R`@\x81 T\x80a-\x97WP`\0\x92\x91PPV[`\0\x83\x81R`\x98` R`@\x90 a-\xB0`\x01\x83ahwV[\x81T\x81\x10a-\xC0Wa-\xC0ae\x81V[`\0\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01`\xC0\x1B\x03\x16\x93\x92PPPV[P\x91\x90PV[```\0\x80a-\xF7\x84aJwV[a\xFF\xFF\x16`\x01`\x01`@\x1B\x03\x81\x11\x15a.\x12Wa.\x12aU\x10V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a.<W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x80[\x82Q\x82\x10\x80\x15a.TWPa\x01\0\x81\x10[\x15a.\xABW`\x01\x81\x1B\x93P\x85\x84\x16\x15a.\x9BW\x80`\xF8\x1B\x83\x83\x81Q\x81\x10a.}Wa.}ae\x81V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x81`\x01\x01\x91P[a.\xA4\x81ae\xADV[\x90Pa.CV[P\x90\x94\x93PPPPV[`\x01\x82` \x01Q`\x02\x81\x11\x15a.\xCDWa.\xCDa[\xB1V[\x14a.\xD7WPPPV[\x81Q`@Qc3V\x7F\x7F`\xE1\x1B\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cf\xAC\xFE\xFE\x90a/,\x90\x88\x90\x86\x90\x88\x90`\x04\x01ah\x8EV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a/KW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a/o\x91\x90ah\xBEV[\x90P`\x01`\x01`\xC0\x1B\x03\x81\x16\x15a/\x9BWa/\x9B\x85a/\x96\x83`\x01`\x01`\xC0\x1B\x03\x16a-\xE9V[a;\xA3V[PPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16a00W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[`\0T`@\x80Q`\x01`\x01`\xA0\x1B\x03b\x01\0\0\x90\x93\x04\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`\0\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16b\x01\0\0\x02b\x01\0\0`\x01`\xB0\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[a0\xCB`@Q\x80``\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`\0a1\x13\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`\x96T`\xFF\x16\x91Pa:\n\x90PV[\x90P`\0a1 \x88a-zV[\x90P`\x01`\x01`\xC0\x1B\x03\x82\x16a1\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FRegistryCoordinator._registerOpe`D\x82\x01R\x7Frator: bitmap cannot be 0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\n\xBCV[a1\xA7\x82a:\xC3V[a2\x1BW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FRegistryCoordinator._registerOpe`D\x82\x01R\x7Frator: some quorums do not exist`d\x82\x01R`\x84\x01a\n\xBCV[\x80\x82\x16`\x01`\x01`\xC0\x1B\x03\x16\x15a2\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`h`$\x82\x01R\x7FRegistryCoordinator._registerOpe`D\x82\x01R\x7Frator: operator already register`d\x82\x01R\x7Fed for some quorums being regist`\x84\x82\x01Rg2\xB92\xB2\x1037\xB9`\xC1\x1B`\xA4\x82\x01R`\xC4\x01a\n\xBCV[`\x01`\x01`\xC0\x1B\x03\x81\x81\x16\x90\x83\x16\x17a2\xEA\x89\x82a6;V[\x88\x7F\xEC)c\xAB!\xC1\xE5\x0E\x1EX*\xA5B\xAF.K\xF7\xBF8\xE6\xE1@<'\xB4.\x1C]nb\x1E\xAA\x87`@Qa3\x1A\x91\x90ae\xC8V[`@Q\x80\x91\x03\x90\xA2`\x01`\x01`\x01`\xA0\x1B\x03\x8B\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T`\xFF\x16`\x02\x81\x11\x15a3TWa3Ta[\xB1V[\x14a4mW`@\x80Q\x80\x82\x01\x82R\x8A\x81R`\x01` \x80\x83\x01\x82\x81R`\x01`\x01`\xA0\x1B\x03\x8F\x16`\0\x90\x81R`\x99\x90\x92R\x93\x90 \x82Q\x81U\x92Q\x83\x82\x01\x80T\x93\x94\x93\x91\x92\x90\x91`\xFF\x19\x16\x90\x83`\x02\x81\x11\x15a3\xAFWa3\xAFa[\xB1V[\x02\x17\x90UPP`@Qc\x99&\xEE}`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91Pc\x99&\xEE}\x90a4\x04\x90\x8D\x90\x89\x90`\x04\x01ah\xDBV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a4\x1EW`\0\x80\xFD[PZ\xF1\x15\x80\x15a42W=`\0\x80>=`\0\xFD[PP`@Q\x8B\x92P`\x01`\x01`\xA0\x1B\x03\x8D\x16\x91P\x7F\xE8\xE6\x8C\xEF\x1C:v\x1E\xD7\xBE~\x84c\xA3u\xF2\x7F{\xC35\xE5\x18$\"<\xAC\xCEcn\xC5\xC3\xFE\x90`\0\x90\xA3[`@Qc\x1F\xD9<\xA9`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c?\xB2yR\x90a4\xBD\x90\x8D\x90\x8C\x90\x8C\x90`\x04\x01aiOV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a4\xD7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a4\xEBW=`\0\x80>=`\0\xFD[PP`@Qc%PGw`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92Pc%PGw\x91Pa5A\x90\x8D\x90\x8D\x90\x8D\x90\x8D\x90`\x04\x01aitV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a5`W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra5\x88\x91\x90\x81\x01\x90aj\0V[`@\x80\x87\x01\x91\x90\x91R` \x86\x01\x91\x90\x91RQb\xBF\xF0M`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90b\xBF\xF0M\x90a5\xE5\x90\x8C\x90\x8C\x90\x8C\x90`\x04\x01ajcV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a6\x04W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra6,\x91\x90\x81\x01\x90aj}V[\x84RPPP\x96\x95PPPPPPV[`\0\x82\x81R`\x98` R`@\x90 T\x80a6\xE0W`\0\x83\x81R`\x98` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x86\x81R`\x01`\x01`\xC0\x1B\x03\x80\x8A\x16\x95\x84\x01\x95\x86R\x84T`\x01\x81\x01\x86U\x94\x88R\x95\x90\x96 \x91Q\x91\x90\x92\x01\x80T\x95Q\x93Q\x90\x94\x16`\x01`@\x1B\x02`\x01`\x01`@\x1B\x03\x93\x83\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x96\x16\x91\x90\x92\x16\x17\x93\x90\x93\x17\x16\x91\x90\x91\x17\x90UPPPV[`\0\x83\x81R`\x98` R`@\x81 a6\xF9`\x01\x84ahwV[\x81T\x81\x10a7\tWa7\tae\x81V[`\0\x91\x82R` \x90\x91 \x01\x80T\x90\x91PCc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14\x15a7MW\x80T`\x01`\x01`@\x1B\x03\x16`\x01`@\x1B`\x01`\x01`\xC0\x1B\x03\x85\x16\x02\x17\x81Ua\x0B\xA3V[\x80Tc\xFF\xFF\xFF\xFFC\x81\x16`\x01` \x1B\x81\x81\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x94\x16\x93\x90\x93\x17\x84U`\0\x87\x81R`\x98` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83R\x94\x85R\x84\x83\x01\x84\x81R`\x01`\x01`\xC0\x1B\x03\x80\x8C\x16\x93\x87\x01\x93\x84R\x82T`\x01\x81\x01\x84U\x92\x86R\x93\x90\x94 \x94Q\x94\x01\x80T\x93Q\x91Q\x90\x92\x16`\x01`@\x1B\x02`\x01`\x01`@\x1B\x03\x91\x86\x16\x90\x96\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x93\x90\x94\x16\x92\x90\x92\x17\x17\x91\x90\x91\x16\x91\x90\x91\x17\x90UPPPPV[3a8\x04a\x1FYV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1E2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\n\xBCV[`\x9DT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F1TW\xD8\xA8\xFE`\xF0J\xF1|\x16\xE2\xF5\xA5\xE1\xDBa+1d\x8EX\x03\x03`u\x9E\xF8\xF3R\x8C\x91\x01`@Q\x80\x91\x03\x90\xA1`\x9D\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x9ET`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F\x8F0\xAB\t\xF4:l\x15}\x7F\xCE~\n\x13\xC0\x03\x04,\x1C\x95\xE8\xA7.z\x14j!\xC0\xCA\xA2M\xC9\x91\x01`@Q\x80\x91\x03\x90\xA1`\x9E\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0a\x11?a99aJ\xA2V[\x83`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x83\x90R`B\x81\x01\x82\x90R`\0\x90`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0\x80\x80a9\xAA`\0\x80Q` am\xA7\x839\x81Q\x91R\x86ak!V[\x90P[a9\xB6\x81aK\xC9V[\x90\x93P\x91P`\0\x80Q` am\xA7\x839\x81Q\x91R\x82\x83\t\x83\x14\x15a9\xF0W`@\x80Q\x80\x82\x01\x90\x91R\x90\x81R` \x81\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80Q` am\xA7\x839\x81Q\x91R`\x01\x82\x08\x90Pa9\xADV[`\0\x80a:\x16\x84aLKV[\x90P\x80\x15a:\xBCW\x82`\xFF\x16\x84`\x01\x86Qa:1\x91\x90ahwV[\x81Q\x81\x10a:AWa:Aae\x81V[\x01` \x01Q`\xF8\x1C\x10a:\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`?`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: bitmap exceeds max value\0`d\x82\x01R`\x84\x01a\n\xBCV[\x93\x92PPPV[`\x96T`\0\x90\x81\x90a:\xDD\x90`\x01\x90`\xFF\x16\x81\x90\x1BahwV[\x90Pa:\xBC`\x01`\x01`\xC0\x1B\x03\x84\x81\x16\x90\x83\x16\x81\x16\x14\x90V[`\xFF\x82\x16`\0\x81\x81R`\x97` \x90\x81R`@\x91\x82\x90 \x84Q\x81T\x86\x84\x01\x80Q\x88\x87\x01\x80Qc\xFF\xFF\xFF\xFF\x90\x95\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x94\x16\x84\x17`\x01` \x1Ba\xFF\xFF\x93\x84\x16\x02\x17g\xFF\xFF\0\0\0\0\0\0\x19\x16`\x01`0\x1B\x95\x83\x16\x95\x90\x95\x02\x94\x90\x94\x17\x90\x94U\x85Q\x91\x82RQ\x83\x16\x93\x81\x01\x93\x90\x93RQ\x16\x91\x81\x01\x91\x90\x91R\x7F>\xE6\xFE\x8DTa\x02D\xC3\xE9\xD3\xC0f\xAEJ\xEE\x99x\x84\xAA(\xF1\x06\x16\xAE\x82\x19%@\x13\x18\xAC\x90``\x01`@Q\x80\x91\x03\x90\xA2PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x99` R`@\x90 \x80T`\x01\x80\x83\x01T`\xFF\x16`\x02\x81\x11\x15a;\xD7Wa;\xD7a[\xB1V[\x14a<DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R`\0\x80Q` am\xC7\x839\x81Q\x91R`D\x82\x01R\x7Fperator: operator is not registe`d\x82\x01Rb\x1C\x99Y`\xEA\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[`\x96T`\0\x90a<X\x90\x85\x90`\xFF\x16a:\nV[\x90P`\0a<e\x83a-zV[\x90P`\x01`\x01`\xC0\x1B\x03\x82\x16a<\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`;`$\x82\x01R`\0\x80Q` am\xC7\x839\x81Q\x91R`D\x82\x01R\x7Fperator: bitmap cannot be 0\0\0\0\0\0`d\x82\x01R`\x84\x01a\n\xBCV[a<\xDA\x82a:\xC3V[a=EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`B`$\x82\x01R`\0\x80Q` am\xC7\x839\x81Q\x91R`D\x82\x01R\x7Fperator: some quorums do not exi`d\x82\x01Ra\x1C\xDD`\xF2\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[a=\\`\x01`\x01`\xC0\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x16\x14\x90V[a=\xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`Y`$\x82\x01R`\0\x80Q` am\xC7\x839\x81Q\x91R`D\x82\x01R\x7Fperator: operator is not registe`d\x82\x01R\x7Fred for specified quorums\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\n\xBCV[`\x01`\x01`\xC0\x1B\x03\x82\x81\x16\x19\x82\x16\x16a=\xFB\x84\x82a6;V[`\x01`\x01`\xC0\x1B\x03\x81\x16a>\xCAW`\x01\x85\x01\x80T`\xFF\x19\x16`\x02\x17\x90U`@QcQ\xB2zm`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA3d\xF4\xDA\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a>{W`\0\x80\xFD[PZ\xF1\x15\x80\x15a>\x8FW=`\0\x80>=`\0\xFD[PP`@Q\x86\x92P`\x01`\x01`\xA0\x1B\x03\x8A\x16\x91P\x7F9o\xDC\xB1\x80\xCB\x0F\xEA&\x92\x81\x13\xFB\x0F\xD1\xC3T\x98c\xF9\xCDV>j\x18O\x1DW\x81\x16\xC8\xE4\x90`\0\x90\xA3[`@Qc\xF4\xE2O\xE5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xF4\xE2O\xE5\x90a?\x18\x90\x8A\x90\x8A\x90`\x04\x01ak5V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a?2W`\0\x80\xFD[PZ\xF1\x15\x80\x15a?FW=`\0\x80>=`\0\xFD[PP`@Qc\xBD)\xB8\xCD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92Pc\xBD)\xB8\xCD\x91Pa?\x98\x90\x87\x90\x8A\x90`\x04\x01akYV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a?\xB2W`\0\x80\xFD[PZ\xF1\x15\x80\x15a?\xC6W=`\0\x80>=`\0\xFD[PP`@Qc\xBD)\xB8\xCD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92Pc\xBD)\xB8\xCD\x91Pa@\x18\x90\x87\x90\x8A\x90`\x04\x01akYV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a@2W`\0\x80\xFD[PZ\xF1\x15\x80\x15a@FW=`\0\x80>=`\0\xFD[PPPPPPPPPPPV[`d\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\x13T*N\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aA\x10W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aA4\x91\x90akrV[\x90P\x80a\x11?W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xBFy\xCEX\x84\x84aAu\x87a\x13\xB5V[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aA\x93\x93\x92\x91\x90ak\x8BV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15aA\xB2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a:\xBC\x91\x90akrV[` \x80\x82\x01Q`\0\x90\x81R`\x9A\x90\x91R`@\x90 T`\xFF\x16\x15aB|W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`R`$\x82\x01R\x7FRegistryCoordinator._verifyChurn`D\x82\x01R\x7FApproverSignature: churnApprover`d\x82\x01Rq\x08\x1C\xD8[\x1D\x08\x18[\x1C\x99XY\x1EH\x1D\\\xD9Y`r\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[B\x81`@\x01Q\x10\x15aC\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`R`$\x82\x01R\x7FRegistryCoordinator._verifyChurn`D\x82\x01R\x7FApproverSignature: churnApprover`d\x82\x01Rq\x08\x1C\xDAY\xDB\x98]\x1D\\\x99H\x19^\x1C\x1A\\\x99Y`r\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[` \x80\x82\x01\x80Q`\0\x90\x81R`\x9A\x90\x92R`@\x91\x82\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U`\x9DT\x90Q\x91\x83\x01Qa\x0B\xA3\x92`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91aC\\\x91\x88\x91\x88\x91\x88\x91\x90a\x1E4V[\x83QaM\xD8V[` \x80\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`\0\x81\x81R`\x99\x90\x94R`@\x90\x93 T\x91\x92\x90\x87\x16\x14\x15aC\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R`\0\x80Q` am\x87\x839\x81Q\x91R`D\x82\x01Rt97\x1D\x101\xB0\xB777\xBA\x101\xB4:\xB97\x109\xB2\xB63`Y\x1B`d\x82\x01R`\x84\x01a\n\xBCV[\x87`\xFF\x16\x84`\0\x01Q`\xFF\x16\x14aD`W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R`\0\x80Q` am\x87\x839\x81Q\x91R`D\x82\x01R\x7Frn: quorumNumber not the same as`d\x82\x01Rf\x08\x1C\xDAY\xDB\x99Y`\xCA\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[`@QcT\x01\xED'`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`\xFF\x89\x16`$\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cT\x01\xED'\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aD\xD1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aD\xF5\x91\x90al\nV[\x90PaE\x01\x81\x85aO\x92V[`\x01`\x01``\x1B\x03\x16\x86`\x01`\x01``\x1B\x03\x16\x11aE\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`V`$\x82\x01R`\0\x80Q` am\x87\x839\x81Q\x91R`D\x82\x01R\x7Frn: incoming operator has insuff`d\x82\x01Ru4\xB1\xB4\xB2\xB7:\x109\xBA0\xB5\xB2\x9037\xB9\x101\xB4:\xB97`Q\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[aE\x9E\x88\x85aO\xB6V[`\x01`\x01``\x1B\x03\x16\x81`\x01`\x01``\x1B\x03\x16\x10a+=W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R`\0\x80Q` am\x87\x839\x81Q\x91R`D\x82\x01R\x7Frn: cannot kick operator with mo`d\x82\x01R\x7Fre than kickBIPsOfTotalStake\0\0\0\0`\x84\x82\x01R`\xA4\x01a\n\xBCV[`\0\x81\x81R`\x98` R`@\x81 T\x81[\x81\x81\x10\x15aF\xCAW`\x01aF]\x82\x84ahwV[aFg\x91\x90ahwV[\x92P\x84c\xFF\xFF\xFF\xFF\x16`\x98`\0\x86\x81R` \x01\x90\x81R` \x01`\0 \x84c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10aF\x9AWaF\x9Aae\x81V[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11aF\xB8WPPa\x11?V[\x80aF\xC2\x81ae\xADV[\x91PPaFIV[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`l`$\x82\x01R\x7FRegistryCoordinator.getQuorumBit`D\x82\x01R\x7FmapIndexAtBlockNumber: no bitmap`d\x82\x01R\x7F update found for operatorId at `\x84\x82\x01Rk167\xB1\xB5\x907:\xB6\xB12\xB9`\xA1\x1B`\xA4\x82\x01R`\xC4\x01a\n\xBCV[`\x96T`\xFF\x16`\xC0\x81\x10aG\xE8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FRegistryCoordinator.createQuorum`D\x82\x01Rt\x0E\x88\x1BX^\x08\x1C][\xDC\x9D[\\\xC8\x1C\x99XX\xDA\x19Y`Z\x1B`d\x82\x01R`\x84\x01a\n\xBCV[aG\xF3\x81`\x01al'V[`\x96\x80T`\xFF\x19\x16`\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80aH\x12\x81\x86a:\xF6V[`@Q`\x01b\x96\xB5\x89`\xE0\x1B\x03\x19\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xFFiJw\x90aHe\x90\x84\x90\x88\x90\x88\x90`\x04\x01alLV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aH\x7FW`\0\x80\xFD[PZ\xF1\x15\x80\x15aH\x93W=`\0\x80>=`\0\xFD[PP`@Qc\x13l\xA0\xF9`\xE1\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x92Pc&\xD9A\xF2\x91P`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aH\xFBW`\0\x80\xFD[PZ\xF1\x15\x80\x15aI\x0FW=`\0\x80>=`\0\xFD[PP`@Qc\x13l\xA0\xF9`\xE1\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x92Pc&\xD9A\xF2\x91P`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aIwW`\0\x80\xFD[PZ\xF1\x15\x80\x15a+=W=`\0\x80>=`\0\xFD[`\0Tb\x01\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15aI\xB2WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[aJ4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[`\x01\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a\x11\xB1\x82a/\xA2V[`\0\x80[\x82\x15a\x11?WaJ\x8C`\x01\x84ahwV[\x90\x92\x16\x91\x80aJ\x9A\x81al\xC5V[\x91PPaJ{V[`\x000`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80\x15aJ\xFBWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14[\x15aK%WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[P`@\x80Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x80\x83\x01\x91\x90\x91R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x84\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x83\x01RF`\x80\x83\x01R0`\xA0\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\xC0\x90\x92\x01\x90\x92R\x80Q\x91\x01 \x90V[`\0\x80\x80`\0\x80Q` am\xA7\x839\x81Q\x91R`\x03`\0\x80Q` am\xA7\x839\x81Q\x91R\x86`\0\x80Q` am\xA7\x839\x81Q\x91R\x88\x89\t\t\x08\x90P`\0aL?\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\0\x80Q` am\xA7\x839\x81Q\x91RaO\xD0V[\x91\x95\x91\x94P\x90\x92PPPV[`\0a\x01\0\x82Q\x11\x15aL\xD4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FBitmapUtils.orderedBytesArrayToB\x90\x82\x01R\x7Fitmap: orderedBytesArray is too `d\x82\x01Rclong`\xE0\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[\x81QaL\xE2WP`\0\x91\x90PV[`\0\x80\x83`\0\x81Q\x81\x10aL\xF8WaL\xF8ae\x81V[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x81\x90\x1B\x92P[\x84Q\x81\x10\x15aM\xCFW\x84\x81\x81Q\x81\x10aM&WaM&ae\x81V[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x1B\x91P\x82\x82\x11aM\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: orderedBytesArray is not `d\x82\x01Rf\x1B\xDC\x99\x19\\\x99Y`\xCA\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[\x91\x81\x17\x91aM\xC8\x81ae\xADV[\x90PaM\x0BV[P\x90\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16;\x15aN\xF2W`@Qc\x0B\x13]?`\xE1\x1B\x80\x82R\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\x16&\xBA~\x90aN\x18\x90\x86\x90\x86\x90`\x04\x01akYV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aN5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aNY\x91\x90al\xE7V[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14a\x11\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`S`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: ERC1271 signature `d\x82\x01Rr\x1D\x99\\\x9AY\x9AX\xD8]\x1A[\xDB\x88\x19\x98Z[\x19Y`j\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[\x82`\x01`\x01`\xA0\x1B\x03\x16aO\x06\x83\x83aP\x7FV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x11\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: signature not from`d\x82\x01Rf\x109\xB4\xB3\xB72\xB9`\xC9\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[` \x81\x01Q`\0\x90a'\x10\x90aO\xAC\x90a\xFF\xFF\x16\x85am\x11V[a:\xBC\x91\x90am@V[`@\x81\x01Q`\0\x90a'\x10\x90aO\xAC\x90a\xFF\xFF\x16\x85am\x11V[`\0\x80aO\xDBaS\xECV[aO\xE3aT\nV[` \x80\x82R\x81\x81\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x88\x90R`\x80\x82\x01\x87\x90R`\xA0\x82\x01\x86\x90R\x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82\x80\x15aP$WaP&V[\xFE[P\x82aPtW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FBN254.expMod: call failure\0\0\0\0\0\0`D\x82\x01R`d\x01a\n\xBCV[PQ\x95\x94PPPPPV[`\0\x80`\0aP\x8E\x85\x85aP\x9BV[\x91P\x91Pa'\xC2\x81aQ\x0BV[`\0\x80\x82Q`A\x14\x15aP\xD2W` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1AaP\xC6\x87\x82\x85\x85aR\xC6V[\x94P\x94PPPPaQ\x04V[\x82Q`@\x14\x15aP\xFCW` \x83\x01Q`@\x84\x01QaP\xF1\x86\x83\x83aS\xB3V[\x93P\x93PPPaQ\x04V[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0\x81`\x04\x81\x11\x15aQ\x1FWaQ\x1Fa[\xB1V[\x14\x15aQ(WPV[`\x01\x81`\x04\x81\x11\x15aQ<WaQ<a[\xB1V[\x14\x15aQ\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\n\xBCV[`\x02\x81`\x04\x81\x11\x15aQ\x9EWaQ\x9Ea[\xB1V[\x14\x15aQ\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\n\xBCV[`\x03\x81`\x04\x81\x11\x15aR\0WaR\0a[\xB1V[\x14\x15aRYW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\n\xBCV[`\x04\x81`\x04\x81\x11\x15aRmWaRma[\xB1V[\x14\x15a\x0F7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 'v' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\n\xBCV[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15aR\xFDWP`\0\x90P`\x03aS\xAAV[\x84`\xFF\x16`\x1B\x14\x15\x80\x15aS\x15WP\x84`\xFF\x16`\x1C\x14\x15[\x15aS&WP`\0\x90P`\x04aS\xAAV[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15aSzW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16aS\xA3W`\0`\x01\x92P\x92PPaS\xAAV[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x80`\x01`\x01`\xFF\x1B\x03\x83\x16\x81aS\xD0`\xFF\x86\x90\x1C`\x1BagGV[\x90PaS\xDE\x87\x82\x88\x85aR\xC6V[\x93P\x93PPP\x93P\x93\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`\0\x80\x83`\x1F\x84\x01\x12aT:W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aTQW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15aQ\x04W`\0\x80\xFD[`\0\x80` \x83\x85\x03\x12\x15aT\x7FW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aT\x95W`\0\x80\xFD[aT\xA1\x85\x82\x86\x01aT(V[\x90\x96\x90\x95P\x93PPPPV[`\0` \x82\x84\x03\x12\x15aT\xBFW`\0\x80\xFD[P5\x91\x90PV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0F7W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15aT\xEDW`\0\x80\xFD[\x835\x92P` \x84\x015aT\xFF\x81aT\xC6V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aUHWaUHaU\x10V[`@R\x90V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aUHWaUHaU\x10V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aU\x98WaU\x98aU\x10V[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12aU\xB1W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aU\xCAWaU\xCAaU\x10V[aU\xDD`\x1F\x82\x01`\x1F\x19\x16` \x01aUpV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15aU\xF2W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15aV!W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aV7W`\0\x80\xFD[aVC\x84\x82\x85\x01aU\xA0V[\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0F7W`\0\x80\xFD[\x805a'\x0C\x81aVKV[`\0` \x82\x84\x03\x12\x15aV}W`\0\x80\xFD[\x815a:\xBC\x81aVKV[`\0\x80\x83`\x1F\x84\x01\x12aV\x9AW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aV\xB1W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aQ\x04W`\0\x80\xFD[`\0``\x82\x84\x03\x12\x15aV\xDBW`\0\x80\xFD[aV\xE3aU&V[\x90P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aV\xFBW`\0\x80\xFD[aW\x07\x84\x82\x85\x01aU\xA0V[\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\xA0\x87\x89\x03\x12\x15aW=W`\0\x80\xFD[\x865aWH\x81aVKV[\x95P` \x87\x015\x94P`@\x87\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aWkW`\0\x80\xFD[aWw\x8A\x83\x8B\x01aV\x88V[\x90\x96P\x94P``\x89\x015\x91P\x80\x82\x11\x15aW\x90W`\0\x80\xFD[aW\x9C\x8A\x83\x8B\x01aU\xA0V[\x93P`\x80\x89\x015\x91P\x80\x82\x11\x15aW\xB2W`\0\x80\xFD[PaW\xBF\x89\x82\x8A\x01aV\xC9V[\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aX\x05W\x81Q`\x01`\x01``\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aW\xE0V[P\x94\x95\x94PPPPPV[` \x80\x82R\x82Q``\x83\x83\x01R\x80Q`\x80\x84\x01\x81\x90R`\0\x92\x91\x82\x01\x90\x83\x90`\xA0\x86\x01\x90[\x80\x83\x10\x15aX[W\x83Qc\xFF\xFF\xFF\xFF\x16\x82R\x92\x84\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x84\x01\x90aX5V[P\x83\x87\x01Q\x93P`\x1F\x19\x92P\x82\x86\x82\x03\x01`@\x87\x01RaX{\x81\x85aW\xCCV[\x93PPP`@\x85\x01Q\x81\x85\x84\x03\x01``\x86\x01Ra\x1Et\x83\x82aW\xCCV[`\0\x80`@\x83\x85\x03\x12\x15aX\xABW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aX\xFBW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aX\xD6V[P\x90\x96\x95PPPPPPV[\x805`\xFF\x81\x16\x81\x14a'\x0CW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aY*W`\0\x80\xFD[a:\xBC\x82aY\x07V[`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x14a\x0F7W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15aY[W`\0\x80\xFD[\x825\x91P` \x83\x015aYm\x81aY3V[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0\x83\x85\x03`\x80\x81\x12\x15aY\x8EW`\0\x80\xFD[\x845aY\x99\x81aVKV[\x93P`@`\x1F\x19\x82\x01\x12\x15aY\xADW`\0\x80\xFD[PaY\xB6aUNV[` \x85\x015\x81R`@\x85\x015`\x03\x81\x10aY\xCFW`\0\x80\xFD[` \x82\x01R\x91P``\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aY\xEFW`\0\x80\xFD[aY\xFB\x86\x82\x87\x01aU\xA0V[\x91PP\x92P\x92P\x92V[`\0[\x83\x81\x10\x15aZ W\x81\x81\x01Q\x83\x82\x01R` \x01aZ\x08V[\x83\x81\x11\x15a\x0B\xA3WPP`\0\x91\x01RV[`\0\x81Q\x80\x84RaZI\x81` \x86\x01` \x86\x01aZ\x05V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0[\x85\x81\x10\x15aZ\xA5W\x82\x84\x03\x89RaZ\x93\x84\x83QaZ1V[\x98\x85\x01\x98\x93P\x90\x84\x01\x90`\x01\x01aZ{V[P\x91\x97\x96PPPPPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P\x82\x86\x01\x91P\x82\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0[\x83\x81\x10\x15a[!W\x88\x83\x03`?\x19\x01\x85R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x87\x01Q\x87\x84\x01\x87\x90Ra[\x0E\x87\x85\x01\x82aZ]V[\x95\x88\x01\x95\x93PP\x90\x86\x01\x90`\x01\x01aZ\xD9V[P\x90\x98\x97PPPPPPPPV[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x81\x01a\x11?V[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a[\\W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a[sW`\0\x80\xFD[a[\x7F\x88\x83\x89\x01aT(V[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a[\x98W`\0\x80\xFD[Pa[\xA5\x87\x82\x88\x01aV\x88V[\x95\x98\x94\x97P\x95PPPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x03\x81\x10a[\xE5WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[\x81Q\x81R` \x80\x83\x01Q`@\x83\x01\x91a\\\x04\x90\x84\x01\x82a[\xC7V[P\x92\x91PPV[\x805a\xFF\xFF\x81\x16\x81\x14a'\x0CW`\0\x80\xFD[`\0``\x82\x84\x03\x12\x15a\\/W`\0\x80\xFD[a\\7aU&V[\x90P\x815a\\D\x81aT\xC6V[\x81Ra\\R` \x83\x01a\\\x0BV[` \x82\x01Ra\\c`@\x83\x01a\\\x0BV[`@\x82\x01R\x92\x91PPV[`\0\x80`\x80\x83\x85\x03\x12\x15a\\\x81W`\0\x80\xFD[a\\\x8A\x83aY\x07V[\x91Pa\\\x99\x84` \x85\x01a\\\x1DV[\x90P\x92P\x92\x90PV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P\x82\x86\x01\x91P\x82\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0\x80[\x84\x81\x10\x15a]FW\x89\x84\x03`?\x19\x01\x86R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x88\x01Q\x88\x85\x01\x88\x90R\x80Q\x88\x86\x01\x81\x90R\x90\x89\x01\x90\x83\x90``\x87\x01\x90[\x80\x83\x10\x15a]1W\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x82R\x92\x8B\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x8B\x01\x90a]\x07V[P\x97\x8A\x01\x97\x95PPP\x91\x87\x01\x91`\x01\x01a\\\xCAV[P\x91\x99\x98PPPPPPPPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a]jW`\0\x80\xFD[\x835a]u\x81aVKV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a]\x90W`\0\x80\xFD[a]\x9C\x86\x82\x87\x01aV\x88V[\x94\x97\x90\x96P\x93\x94PPPPV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a]\xC2Wa]\xC2aU\x10V[P`\x05\x1B` \x01\x90V[`\0`@\x82\x84\x03\x12\x15a]\xDEW`\0\x80\xFD[a]\xE6aUNV[\x90Pa]\xF1\x82aY\x07V[\x81R` \x82\x015a^\x01\x81aVKV[` \x82\x01R\x92\x91PPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a^$W`\0\x80\xFD[\x855a^/\x81aVKV[\x94P` \x86\x81\x015\x94P`@\x80\x88\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a^SW`\0\x80\xFD[\x88\x01`\x1F\x81\x01\x8A\x13a^dW`\0\x80\xFD[\x805a^wa^r\x82a]\xA9V[aUpV[\x81\x81R`\x06\x91\x90\x91\x1B\x82\x01\x84\x01\x90\x84\x81\x01\x90\x8C\x83\x11\x15a^\x96W`\0\x80\xFD[\x92\x85\x01\x92[\x82\x84\x10\x15a^\xBCWa^\xAD\x8D\x85a]\xCCV[\x82R\x92\x84\x01\x92\x90\x85\x01\x90a^\x9BV[\x99\x9C\x98\x9BP\x98\x99``\x81\x015\x99P`\x80\x015\x97\x96PPPPPPPV[` \x81R`\0a:\xBC` \x83\x01\x84aZ]V[`\0a\x01\0\x82\x84\x03\x12\x15a-\xE3W`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12a_\x11W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a_(W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x06\x1B\x85\x01\x01\x11\x15aQ\x04W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0\x80`\0a\x01\xA0\x8A\x8C\x03\x12\x15a_bW`\0\x80\xFD[\x895`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a_yW`\0\x80\xFD[a_\x85\x8D\x83\x8E\x01aV\x88V[\x90\x9BP\x99P` \x8C\x015\x91P\x80\x82\x11\x15a_\x9EW`\0\x80\xFD[a_\xAA\x8D\x83\x8E\x01aV\x88V[\x90\x99P\x97P\x87\x91Pa_\xBF\x8D`@\x8E\x01a^\xECV[\x96Pa\x01@\x8C\x015\x91P\x80\x82\x11\x15a_\xD6W`\0\x80\xFD[a_\xE2\x8D\x83\x8E\x01a^\xFFV[\x90\x96P\x94Pa\x01`\x8C\x015\x91P\x80\x82\x11\x15a_\xFCW`\0\x80\xFD[a`\x08\x8D\x83\x8E\x01aV\xC9V[\x93Pa\x01\x80\x8C\x015\x91P\x80\x82\x11\x15a`\x1FW`\0\x80\xFD[Pa`,\x8C\x82\x8D\x01aV\xC9V[\x91PP\x92\x95\x98P\x92\x95\x98P\x92\x95\x98V[`\0\x80`\0\x80`\0\x80a\x01`\x87\x89\x03\x12\x15a`VW`\0\x80\xFD[\x865`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a`mW`\0\x80\xFD[a`y\x8A\x83\x8B\x01aV\x88V[\x90\x98P\x96P` \x89\x015\x91P\x80\x82\x11\x15a`\x92W`\0\x80\xFD[a`\x9E\x8A\x83\x8B\x01aV\x88V[\x90\x96P\x94P\x84\x91Pa`\xB3\x8A`@\x8B\x01a^\xECV[\x93Pa\x01@\x89\x015\x91P\x80\x82\x11\x15aW\xB2W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a`\xDDW`\0\x80\xFD[\x825a`\xE8\x81aT\xC6V[\x91P` \x83\x81\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aa\x04W`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13aa\x15W`\0\x80\xFD[\x805aa#a^r\x82a]\xA9V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x88\x83\x11\x15aaBW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aa`W\x835\x82R\x92\x84\x01\x92\x90\x84\x01\x90aaGV[\x80\x95PPPPPP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aX\xFBW\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aa\x8BV[`\0\x80` \x83\x85\x03\x12\x15aa\xC0W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aa\xD6W`\0\x80\xFD[aT\xA1\x85\x82\x86\x01aV\x88V[`\x01`\x01``\x1B\x03\x81\x16\x81\x14a\x0F7W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12ab\x08W`\0\x80\xFD[\x815` ab\x18a^r\x83a]\xA9V[\x82\x81R`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15ab7W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15ab\x88W`@\x81\x89\x03\x12\x15abTW`\0\x80\x81\xFD[ab\\aUNV[\x815abg\x81aVKV[\x81R\x81\x85\x015abv\x81aa\xE2V[\x81\x86\x01R\x83R\x91\x83\x01\x91`@\x01ab;V[P\x96\x95PPPPPPV[`\0\x80`\0`\xA0\x84\x86\x03\x12\x15ab\xA8W`\0\x80\xFD[ab\xB2\x85\x85a\\\x1DV[\x92P``\x84\x015ab\xC2\x81aa\xE2V[\x91P`\x80\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15ab\xDDW`\0\x80\xFD[aY\xFB\x86\x82\x87\x01aa\xF7V[`\0\x80`@\x83\x85\x03\x12\x15ab\xFCW`\0\x80\xFD[\x825ac\x07\x81aVKV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x82`\x1F\x83\x01\x12ac&W`\0\x80\xFD[\x815` ac6a^r\x83a]\xA9V[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15acUW`\0\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15acxWack\x89\x82a\\\x1DV[\x84R\x92\x84\x01\x92\x81\x01acYV[P\x90\x97\x96PPPPPPPV[`\0\x82`\x1F\x83\x01\x12ac\x96W`\0\x80\xFD[\x815` ac\xA6a^r\x83a]\xA9V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15ac\xC5W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15ab\x88W\x805ac\xDC\x81aa\xE2V[\x83R\x91\x83\x01\x91\x83\x01ac\xC9V[`\0\x82`\x1F\x83\x01\x12ac\xFAW`\0\x80\xFD[\x815` ad\na^r\x83a]\xA9V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15ad)W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15ab\x88W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15adLW`\0\x80\x81\xFD[adZ\x89\x86\x83\x8B\x01\x01aa\xF7V[\x84RP\x91\x83\x01\x91\x83\x01ad-V[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15ad\x85W`\0\x80\xFD[ad\x8E\x89aV`V[\x97Pad\x9C` \x8A\x01aV`V[\x96Pad\xAA`@\x8A\x01aV`V[\x95Pad\xB8``\x8A\x01aV`V[\x94P`\x80\x89\x015\x93P`\xA0\x89\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15ad\xDBW`\0\x80\xFD[ad\xE7\x8C\x83\x8D\x01ac\x15V[\x94P`\xC0\x8B\x015\x91P\x80\x82\x11\x15ad\xFDW`\0\x80\xFD[ae\t\x8C\x83\x8D\x01ac\x85V[\x93P`\xE0\x8B\x015\x91P\x80\x82\x11\x15ae\x1FW`\0\x80\xFD[Pae,\x8B\x82\x8C\x01ac\xE9V[\x91PP\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[` \x81\x01a\x11?\x82\x84a[\xC7V[` \x80\x82R`\x19\x90\x82\x01R\x7FPausable: index is paused\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15ae\xC1Wae\xC1ae\x97V[P`\x01\x01\x90V[` \x81R`\0a:\xBC` \x83\x01\x84aZ1V[`\0` \x82\x84\x03\x12\x15ae\xEDW`\0\x80\xFD[\x81Qa:\xBC\x81aVKV[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15afTW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a:\xBCW`\0\x80\xFD[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80af\xC0W`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15a-\xE3WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12af\xF8W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15ag\x12W`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15aQ\x04W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15ag<W`\0\x80\xFD[\x81Qa:\xBC\x81aT\xC6V[`\0\x82\x19\x82\x11\x15agZWagZae\x97V[P\x01\x90V[`\0\x80\x85\x85\x11\x15agoW`\0\x80\xFD[\x83\x86\x11\x15ag|W`\0\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[`\0`\xC0\x82\x01\x88\x83R` `\x01\x80`\xA0\x1B\x03\x80\x8A\x16\x82\x86\x01R`@\x89\x81\x87\x01R`\xC0``\x87\x01R\x83\x89Q\x80\x86R`\xE0\x88\x01\x91P\x84\x8B\x01\x95P`\0[\x81\x81\x10\x15ag\xEEW\x86Q\x80Q`\xFF\x16\x84R\x86\x01Q\x85\x16\x86\x84\x01R\x95\x85\x01\x95\x91\x83\x01\x91`\x01\x01ag\xC4V[PP`\x80\x87\x01\x98\x90\x98RPPPP`\xA0\x90\x91\x01\x91\x90\x91RP\x94\x93PPPPV[`\0`@\x82\x84\x03\x12\x15ah W`\0\x80\xFD[a:\xBC\x83\x83a]\xCCV[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x81R\x81Q`\0\x90ahM\x81`\x04\x85\x01` \x87\x01aZ\x05V[\x91\x90\x91\x01`\x04\x01\x93\x92PPPV[`\0\x82Qahm\x81\x84` \x87\x01aZ\x05V[\x91\x90\x91\x01\x92\x91PPV[`\0\x82\x82\x10\x15ah\x89Wah\x89ae\x97V[P\x03\x90V[`\x01\x80`\xA0\x1B\x03\x84\x16\x81R\x82` \x82\x01R```@\x82\x01R`\0ah\xB5``\x83\x01\x84aZ1V[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15ah\xD0W`\0\x80\xFD[\x81Qa:\xBC\x81aY3V[`\x01\x80`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R`\0\x82Q```@\x84\x01Rai\x05`\xA0\x84\x01\x82aZ1V[\x90P` \x84\x01Q``\x84\x01R`@\x84\x01Q`\x80\x84\x01R\x80\x91PP\x93\x92PPPV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R`@` \x82\x01\x81\x90R`\0\x90ah\xB5\x90\x83\x01\x84\x86ai&V[`\x01\x80`\xA0\x1B\x03\x85\x16\x81R\x83` \x82\x01R```@\x82\x01R`\0a\x1Et``\x83\x01\x84\x86ai&V[`\0\x82`\x1F\x83\x01\x12ai\xADW`\0\x80\xFD[\x81Q` ai\xBDa^r\x83a]\xA9V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15ai\xDCW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15ab\x88W\x80Qai\xF3\x81aa\xE2V[\x83R\x91\x83\x01\x91\x83\x01ai\xE0V[`\0\x80`@\x83\x85\x03\x12\x15aj\x13W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aj*W`\0\x80\xFD[aj6\x86\x83\x87\x01ai\x9CV[\x93P` \x85\x01Q\x91P\x80\x82\x11\x15ajLW`\0\x80\xFD[PajY\x85\x82\x86\x01ai\x9CV[\x91PP\x92P\x92\x90PV[\x83\x81R`@` \x82\x01R`\0ah\xB5`@\x83\x01\x84\x86ai&V[`\0` \x80\x83\x85\x03\x12\x15aj\x90W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aj\xA6W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aj\xB7W`\0\x80\xFD[\x80Qaj\xC5a^r\x82a]\xA9V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15aj\xE4W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x10\xA9W\x83Qaj\xFC\x81aT\xC6V[\x82R\x92\x84\x01\x92\x90\x84\x01\x90aj\xE9V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82ak0Wak0ak\x0BV[P\x06\x90V[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R`\0\x90aVC\x90\x83\x01\x84aZ1V[\x82\x81R`@` \x82\x01R`\0aVC`@\x83\x01\x84aZ1V[`\0` \x82\x84\x03\x12\x15ak\x84W`\0\x80\xFD[PQ\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81Ra\x01`\x81\x01ak\xB3` \x83\x01\x85\x805\x82R` \x90\x81\x015\x91\x01RV[ak\xCD``\x83\x01`@\x86\x01\x805\x82R` \x90\x81\x015\x91\x01RV[`@`\x80\x85\x01`\xA0\x84\x017`\xE0\x82\x01`\0\x81R`@`\xC0\x86\x01\x827P`\0a\x01 \x83\x01\x90\x81R\x83Q\x90R` \x90\x92\x01Qa\x01@\x90\x91\x01R\x92\x91PPV[`\0` \x82\x84\x03\x12\x15al\x1CW`\0\x80\xFD[\x81Qa:\xBC\x81aa\xE2V[`\0`\xFF\x82\x16`\xFF\x84\x16\x80`\xFF\x03\x82\x11\x15alDWalDae\x97V[\x01\x93\x92PPPV[`\0``\x82\x01`\xFF\x86\x16\x83R` `\x01`\x01``\x1B\x03\x80\x87\x16\x82\x86\x01R`@``\x81\x87\x01R\x83\x87Q\x80\x86R`\x80\x88\x01\x91P\x84\x89\x01\x95P`\0[\x81\x81\x10\x15al\xB5W\x86Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x86\x01Q\x85\x16\x86\x84\x01R\x95\x85\x01\x95\x91\x83\x01\x91`\x01\x01al\x85V[P\x90\x9A\x99PPPPPPPPPPV[`\0a\xFF\xFF\x80\x83\x16\x81\x81\x14\x15al\xDDWal\xDDae\x97V[`\x01\x01\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15al\xF9W`\0\x80\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a:\xBCW`\0\x80\xFD[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15am7Wam7ae\x97V[\x02\x94\x93PPPPV[`\0`\x01`\x01``\x1B\x03\x80\x84\x16\x80amZWamZak\x0BV[\x92\x16\x91\x90\x91\x04\x92\x91PPV\xFERegistryCoordinator.updateOperatRegistryCoordinator._validateChu0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDGRegistryCoordinator._deregisterO\xA2dipfsX\"\x12 \xCF\xE7\xB4$(.\x97\x8Dq\x0Fk\xE0g\xAA\xE6\xE9\xF2S~,\xA7YM\x9B\x04\xBB\x14JW\x93:ldsolcC\0\x08\x0C\x003";
    /// The bytecode of the contract.
    pub static REGISTRYCOORDINATORHARNESS_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x03\xCFW`\x005`\xE0\x1C\x80cf\xD9\xA9\xA0\x11a\x01\xFFW\x80c\xB5P\x8A\xA9\x11a\x01\x1AW\x80c\xD9,\xBB\x84\x11a\0\xADW\x80c\xF2\xFD\xE3\x8B\x11a\0|W\x80c\xF2\xFD\xE3\x8B\x14a\n$W\x80c\xFAv&\xD4\x14a\n7W\x80c\xFA\xBC\x1C\xBC\x14a\nDW\x80c\xFD9\x10Z\x14a\nWW`\0\x80\xFD[\x80c\xD9,\xBB\x84\x14a\t<W\x80c\xDD\x82\x83\xF3\x14a\tfW\x80c\xE2\x0C\x9Fq\x14a\tyW\x80c\xE6W\x97\xAD\x14a\t\x81W`\0\x80\xFD[\x80c\xCA\r\xE8\x82\x11a\0\xE9W\x80c\xCA\r\xE8\x82\x14a\x08\xE7W\x80c\xCAO-\x97\x14a\t\x0EW\x80c\xD7-\x8D\xD6\x14a\t!W\x80c\xD7[L\x88\x14a\t)W`\0\x80\xFD[\x80c\xB5P\x8A\xA9\x14a\x08\x93W\x80c\xBAAO\xA6\x14a\x08\x9BW\x80c\xC3\x91B^\x14a\x08\xA3W\x80c\xC4\t}^\x14a\x08\xC3W`\0\x80\xFD[\x80c\x88o\x11\x95\x11a\x01\x92W\x80c\x9B]\x17{\x11a\x01aW\x80c\x9B]\x17{\x14a\x08\x1FW\x80c\x9E\x99#\xC2\x14a\x082W\x80c\x9F\xEA\xB8Y\x14a\x08YW\x80c\xA5\x08W\xBF\x14a\x08\x80W`\0\x80\xFD[\x80c\x88o\x11\x95\x14a\x07\xD7W\x80c\x8D\xA5\xCB[\x14a\x07\xF0W\x80c\x91j\x17\xC6\x14a\x07\xF8W\x80c\x9A\xA1e=\x14a\x08\0W`\0\x80\xFD[\x80c\x83\x10\xFE\xF6\x11a\x01\xCEW\x80c\x83\x10\xFE\xF6\x14a\x07\x89W\x80c\x84\xCAR\x13\x14a\x07\x9CW\x80c\x85\"l\x81\x14a\x07\xAFW\x80c\x87\x1E\xF0I\x14a\x07\xC4W`\0\x80\xFD[\x80cf\xD9\xA9\xA0\x14a\x072W\x80ch0H5\x14a\x07GW\x80cn;\x17\xDB\x14a\x07nW\x80cqP\x18\xA6\x14a\x07\x81W`\0\x80\xFD[\x80c)k\xB0d\x11a\x02\xEFW\x80cQ@\xA5H\x11a\x02\x82W\x80c[\x0B\x82\x9F\x11a\x02QW\x80c[\x0B\x82\x9F\x14a\x06\xDDW\x80c\\\x97Z\xBB\x14a\x06\xF0W\x80c]\xF4YF\x14a\x06\xF8W\x80ccG\xC9\0\x14a\x07\x1FW`\0\x80\xFD[\x80cQ@\xA5H\x14a\x06\x83W\x80cXe\xC6\x0C\x14a\x06\x96W\x80cY\\jg\x14a\x06\xB6W\x80cZ\xC8j\xB7\x14a\x06\xBEW`\0\x80\xFD[\x80c9\x98\xFD\xD3\x11a\x02\xBEW\x80c9\x98\xFD\xD3\x14a\x06,W\x80c<*\x7FL\x14a\x06SW\x80c>^<#\x14a\x06sW\x80c?r\x86\xF4\x14a\x06{W`\0\x80\xFD[\x80c)k\xB0d\x14a\x05\xDEW\x80c)\xD1\xE0\xC3\x14a\x05\xF1W\x80c*\xDE8\x80\x14a\x06\x04W\x80c,\xDD\x1E\x86\x14a\x06\x19W`\0\x80\xFD[\x80c\x14x\x85\x1F\x11a\x03gW\x80c$\x9A\x0CB\x11a\x036W\x80c$\x9A\x0CB\x14a\x05\x85W\x80c'\xE7\x92\x88\x14a\x05\xA5W\x80c(\xF6\x1B1\x14a\x05\xB8W\x80c)ST|\x14a\x05\xCBW`\0\x80\xFD[\x80c\x14x\x85\x1F\x14a\x04\xD4W\x80c\x1A\xB2WO\x14a\x05\x07W\x80c\x1E\xB8\x12\xDA\x14a\x05'W\x80c\x1E\xD7\x83\x1C\x14a\x05pW`\0\x80\xFD[\x80c\x0C\xF4\xB7g\x11a\x03\xA3W\x80c\x0C\xF4\xB7g\x14a\x04rW\x80c\x10\xD6z/\x14a\x04\x85W\x80c\x13T*N\x14a\x04\x98W\x80c\x13d9\xDD\x14a\x04\xC1W`\0\x80\xFD[\x80b\xCF*\xB5\x14a\x03\xD4W\x80c\x03\xFD4\x92\x14a\x03\xE9W\x80c\x04\xECcQ\x14a\x04\x1CW\x80c\x05C\x10\xE6\x14a\x04GW[`\0\x80\xFD[a\x03\xE7a\x03\xE26`\x04aTlV[a\n\x93V[\0[a\x04\ta\x03\xF76`\x04aT\xADV[`\0\x90\x81R`\x98` R`@\x90 T\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x04/a\x04*6`\x04aT\xD8V[a\x0B\xA9V[`@Q`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x81R` \x01a\x04\x13V[`\x9DTa\x04Z\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x04\x13V[a\x03\xE7a\x04\x806`\x04aV\x0FV[a\r\x9FV[a\x03\xE7a\x04\x936`\x04aVkV[a\x0E\x87V[a\x04\ta\x04\xA66`\x04aVkV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x99` R`@\x90 T\x90V[a\x03\xE7a\x04\xCF6`\x04aT\xADV[a\x0F:V[a\x04\xF7a\x04\xE26`\x04aT\xADV[`\x9A` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x04\x13V[a\x05\x1Aa\x05\x156`\x04aW$V[a\x10wV[`@Qa\x04\x13\x91\x90aX\x10V[a\x05:a\x0556`\x04aX\x98V[a\x10\xB4V[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x80\x85\x01Q\x90\x91\x16\x90\x82\x01R\x91\x81\x01Q`\x01`\x01`\xC0\x1B\x03\x16\x90\x82\x01R``\x01a\x04\x13V[a\x05xa\x11EV[`@Qa\x04\x13\x91\x90aX\xBAV[a\x04\ta\x05\x936`\x04aY\x18V[`\x9B` R`\0\x90\x81R`@\x90 T\x81V[a\x03\xE7a\x05\xB36`\x04aYHV[a\x11\xA7V[`\x9ETa\x04Z\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03\xE7a\x05\xD96`\x04aYxV[a\x11\xB5V[a\x04Za\x05\xEC6`\x04aT\xADV[a\x11\xC5V[a\x03\xE7a\x05\xFF6`\x04aVkV[a\x12QV[a\x06\x0Ca\x12bV[`@Qa\x04\x13\x91\x90aZ\xB2V[a\x03\xE7a\x06'6`\x04aVkV[a\x13\xA4V[a\x04Z\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x06fa\x06a6`\x04aVkV[a\x13\xB5V[`@Qa\x04\x13\x91\x90a[/V[a\x05xa\x144V[a\x05xa\x14\x94V[a\x03\xE7a\x06\x916`\x04a[FV[a\x14\xF4V[a\x06\xA9a\x06\xA46`\x04aVkV[a\x1A~V[`@Qa\x04\x13\x91\x90a[\xE9V[a\x03\xE7a\x1A\xF2V[a\x04\xF7a\x06\xCC6`\x04aY\x18V[`\x01\x80T`\xFF\x90\x92\x16\x1B\x90\x81\x16\x14\x90V[a\x03\xE7a\x06\xEB6`\x04a\\nV[a\x1B\xBEV[`\x01Ta\x04\tV[a\x04Z\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x04Za\x07-6`\x04aT\xADV[a\x1CPV[a\x07:a\x1CzV[`@Qa\x04\x13\x91\x90a\\\xA2V[a\x04Z\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\xE7a\x07|6`\x04a]UV[a\x1D`V[a\x03\xE7a\x1E V[a\x03\xE7a\x07\x976`\x04a]UV[a\x1D\xE0V[a\x04\ta\x07\xAA6`\x04a^\x0CV[a\x1E4V[a\x07\xB7a\x1E~V[`@Qa\x04\x13\x91\x90a^\xD9V[a\x04/a\x07\xD26`\x04aT\xADV[a\x1FNV[`\0Ta\x04Z\x90b\x01\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x04Za\x1FYV[a\x07:a\x1FrV[`\x96Ta\x08\r\x90`\xFF\x16\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x04\x13V[a\x03\xE7a\x08-6`\x04a_CV[a XV[a\x04Z\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x04\t\x7F+\xD8!$\x05\x7F\t\x13\xBC;w,\xE7\xB8>\x80W\xC1\xAD\x1F5\x10\xFC\x83w\x8B\xE2\x0F\x10\xEC]\xE6\x81V[a\x03\xE7a\x08\x8E6`\x04a`<V[a#\x90V[a\x07\xB7a%\x14V[a\x04\xF7a%\xE4V[a\x08\xB6a\x08\xB16`\x04a`\xCAV[a'\x11V[`@Qa\x04\x13\x91\x90aaoV[a\x03\xE7a\x08\xD16`\x04aY\x18V[`\x96\x80T`\xFF\x19\x16`\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x04\t\x7F;\xFES\x90\xD1h\x8D.\x05n\x0E\xE4\xB6\x8CT\x19\xF4-\x9B\xD0\xC7\x92\xF0\x1A\xCB\x83\x88g\x14\xAD\xEBV\x81V[a\x03\xE7a\t\x1C6`\x04aa\xADV[a'\xCAV[`\x9CTa\x04\tV[a\x03\xE7a\t76`\x04ab\x93V[a(1V[a\x03\xE7a\tJ6`\x04ab\xE9V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\0\x90\x81R`\x99` R`@\x90 UV[a\x03\xE7a\tt6`\x04adhV[a(DV[a\x05xa+HV[a\t\xF0a\t\x8F6`\x04aY\x18V[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x92\x84\x01\x81\x90R`\xFF\x94\x90\x94\x16\x84R`\x97\x82R\x92\x82\x90 \x82Q\x93\x84\x01\x83RTc\xFF\xFF\xFF\xFF\x81\x16\x84Ra\xFF\xFF`\x01` \x1B\x82\x04\x81\x16\x92\x85\x01\x92\x90\x92R`\x01`0\x1B\x90\x04\x16\x90\x82\x01R\x90V[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x16\x81R` \x80\x84\x01Qa\xFF\xFF\x90\x81\x16\x91\x83\x01\x91\x90\x91R\x92\x82\x01Q\x90\x92\x16\x90\x82\x01R``\x01a\x04\x13V[a\x03\xE7a\n26`\x04aVkV[a+\xA8V[`\xCFTa\x04\xF7\x90`\xFF\x16\x81V[a\x03\xE7a\nR6`\x04aT\xADV[a,\x1EV[a\n\x86a\ne6`\x04aVkV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T`\xFF\x16\x90V[`@Qa\x04\x13\x91\x90ae<V[`\x01T`\x02\x90`\x04\x90\x81\x16\x14\x15a\n\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\xBC\x90aeJV[`@Q\x80\x91\x03\x90\xFD[`\0[\x82\x81\x10\x15a\x0B\xA3W`\0\x84\x84\x83\x81\x81\x10a\n\xE4Wa\n\xE4ae\x81V[\x90P` \x02\x01` \x81\x01\x90a\n\xF9\x91\x90aVkV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x99` \x90\x81R`@\x80\x83 \x81Q\x80\x83\x01\x90\x92R\x80T\x82R`\x01\x81\x01T\x94\x95P\x92\x93\x90\x92\x91\x83\x01\x90`\xFF\x16`\x02\x81\x11\x15a\x0BDWa\x0BDa[\xB1V[`\x02\x81\x11\x15a\x0BUWa\x0BUa[\xB1V[\x90RP\x80Q\x90\x91P`\0a\x0Bh\x82a-zV[\x90P`\0a\x0B~\x82`\x01`\x01`\xC0\x1B\x03\x16a-\xE9V[\x90Pa\x0B\x8B\x85\x85\x83a.\xB5V[PPPPP\x80\x80a\x0B\x9B\x90ae\xADV[\x91PPa\n\xC8V[PPPPV[`\0\x83\x81R`\x98` R`@\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x0B\xCCWa\x0B\xCCae\x81V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x85R`\x01` \x1B\x83\x04\x82\x16\x95\x85\x01\x95\x90\x95R`\x01`@\x1B\x90\x91\x04`\x01`\x01`\xC0\x1B\x03\x16\x91\x83\x01\x91\x90\x91R\x90\x92P\x85\x16\x10\x15a\x0C\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`e`$\x82\x01R\x7FRegistryCoordinator.getQuorumBit`D\x82\x01R\x7FmapAtBlockNumberByIndex: quorumB`d\x82\x01R\x7FitmapUpdate is from after blockN`\x84\x82\x01Rd:\xB6\xB12\xB9`\xD9\x1B`\xA4\x82\x01R`\xC4\x01a\n\xBCV[` \x81\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x80a\x0C\xECWP\x80` \x01Qc\xFF\xFF\xFF\xFF\x16\x84c\xFF\xFF\xFF\xFF\x16\x10[a\r\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`f`$\x82\x01R\x7FRegistryCoordinator.getQuorumBit`D\x82\x01R\x7FmapAtBlockNumberByIndex: quorumB`d\x82\x01R\x7FitmapUpdate is from before block`\x84\x82\x01Re':\xB6\xB12\xB9`\xD1\x1B`\xA4\x82\x01R`\xC4\x01a\n\xBCV[`@\x01Q\x94\x93PPPPV[`\x013`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T`\xFF\x16`\x02\x81\x11\x15a\r\xC8Wa\r\xC8a[\xB1V[\x14a\x0E;W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R\x7FRegistryCoordinator.updateSocket`D\x82\x01R\x7F: operator is not registered\0\0\0\0`d\x82\x01R`\x84\x01a\n\xBCV[3`\0\x90\x81R`\x99` R`@\x90\x81\x90 T\x90Q\x7F\xEC)c\xAB!\xC1\xE5\x0E\x1EX*\xA5B\xAF.K\xF7\xBF8\xE6\xE1@<'\xB4.\x1C]nb\x1E\xAA\x90a\x0E|\x90\x84\x90ae\xC8V[`@Q\x80\x91\x03\x90\xA2PV[`\0`\x02\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xDAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xFE\x91\x90ae\xDBV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0F.W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\xBC\x90ae\xF8V[a\x0F7\x81a/\xA2V[PV[`\0T`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01Rb\x01\0\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x87W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xAB\x91\x90afBV[a\x0F\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\xBC\x90afdV[`\x01T\x81\x81\x16\x14a\x10@W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\n\xBCV[`\x01\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01a\x0E|V[a\x10\x9B`@Q\x80``\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81RP\x90V[a\x10\xA9\x87\x87\x87\x87\x87\x87a0\xA7V[\x97\x96PPPPPPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R`\0\x83\x81R`\x98` R`@\x90 \x80T\x83\x90\x81\x10a\x10\xF1Wa\x10\xF1ae\x81V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01`\xC0\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90P[\x92\x91PPV[```\xDC\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x11\x9DW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x11\x7FW[PPPPP\x90P\x90V[a\x11\xB1\x82\x82a6;V[PPV[a\x11\xC0\x83\x83\x83a.\xB5V[PPPV[`@Qc\x08\xF6b\x9D`\xE3\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cG\xB3\x14\xE8\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12-W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11?\x91\x90ae\xDBV[a\x12Ya7\xFBV[a\x0F7\x81a8ZV[```\xE3\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x13\x9BW`\0\x84\x81R` \x80\x82 `@\x80Q\x80\x82\x01\x82R`\x02\x87\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x95\x91\x94\x86\x81\x01\x94\x91\x92\x90\x84\x01[\x82\x82\x10\x15a\x13\x84W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x12\xF7\x90af\xACV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x13#\x90af\xACV[\x80\x15a\x13pW\x80`\x1F\x10a\x13EWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x13pV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x13SW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x12\xD8V[PPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x12\x86V[PPPP\x90P\x90V[a\x13\xACa7\xFBV[a\x0F7\x81a8\xC3V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x11?a\x14/\x7F+\xD8!$\x05\x7F\t\x13\xBC;w,\xE7\xB8>\x80W\xC1\xAD\x1F5\x10\xFC\x83w\x8B\xE2\x0F\x10\xEC]\xE6\x84`@Q` \x01a\x14\x14\x92\x91\x90\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a9,V[a9zV[```\xDE\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x11\x9DW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x11\x7FWPPPPP\x90P\x90V[```\xDD\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x11\x9DW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x11\x7FWPPPPP\x90P\x90V[`\x01T`\x02\x90`\x04\x90\x81\x16\x14\x15a\x15\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\xBC\x90aeJV[`\0a\x15e\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`\x96T`\xFF\x16\x91Pa:\n\x90PV[\x90Pa\x15p\x81a:\xC3V[a\x15\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R`\0\x80Q` amg\x839\x81Q\x91R`D\x82\x01R\x7ForsForQuorum: some quorums do no`d\x82\x01Rf\x1D\x08\x19^\x1A\\\xDD`\xCA\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[\x84\x83\x14a\x16OW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R`\0\x80Q` amg\x839\x81Q\x91R`D\x82\x01R\x7ForsForQuorum: input length misma`d\x82\x01Rb\x0E\x8Cm`\xEB\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[`\0[\x83\x81\x10\x15a\x1AuW`\0\x85\x85\x83\x81\x81\x10a\x16nWa\x16nae\x81V[\x91\x90\x91\x015`\xF8\x1C\x91P6\x90P`\0\x89\x89\x85\x81\x81\x10a\x16\x8FWa\x16\x8Fae\x81V[\x90P` \x02\x81\x01\x90a\x16\xA1\x91\x90af\xE1V[`@Qcy\xA0\x84\x91`\xE1\x1B\x81R`\xFF\x86\x16`\x04\x82\x01R\x91\x93P\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xF3A\t\"\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x171\x91\x90ag*V[c\xFF\xFF\xFF\xFF\x16\x81\x14a\x17\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`e`$\x82\x01R`\0\x80Q` amg\x839\x81Q\x91R`D\x82\x01R\x7ForsForQuorum: number of updated `d\x82\x01R\x7Foperators does not match quorum `\x84\x82\x01Rd\x1D\x1B\xDD\x18[`\xDA\x1B`\xA4\x82\x01R`\xC4\x01a\n\xBCV[`\0\x80[\x82\x81\x10\x15a\x1A\x14W`\0\x84\x84\x83\x81\x81\x10a\x17\xEDWa\x17\xEDae\x81V[\x90P` \x02\x01` \x81\x01\x90a\x18\x02\x91\x90aVkV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x99` \x90\x81R`@\x80\x83 \x81Q\x80\x83\x01\x90\x92R\x80T\x82R`\x01\x81\x01T\x94\x95P\x92\x93\x90\x92\x91\x83\x01\x90`\xFF\x16`\x02\x81\x11\x15a\x18MWa\x18Ma[\xB1V[`\x02\x81\x11\x15a\x18^Wa\x18^a[\xB1V[\x90RP\x80Q\x90\x91P`\0a\x18q\x82a-zV[\x90P`\x01`\x01`\x01`\xC0\x1B\x03\x82\x16`\xFF\x8B\x16\x1C\x81\x16\x14a\x18\xF5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R`\0\x80Q` amg\x839\x81Q\x91R\x90\x82\x01R\x7ForsForQuorum: operator not in qu`d\x82\x01Rcorum`\xE0\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[\x85`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x11a\x19\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`g`$\x82\x01R`\0\x80Q` amg\x839\x81Q\x91R`D\x82\x01R\x7ForsForQuorum: operators array mu`d\x82\x01R\x7Fst be sorted in ascending addres`\x84\x82\x01Rf9\x907\xB922\xB9`\xC9\x1B`\xA4\x82\x01R`\xC4\x01a\n\xBCV[Pa\x19\xFE\x83\x83\x8F\x8F\x8D\x90\x8E`\x01a\x19\xB7\x91\x90agGV[\x92a\x19\xC4\x93\x92\x91\x90ag_V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa.\xB5\x92PPPV[P\x90\x92Pa\x1A\r\x90P\x81ae\xADV[\x90Pa\x17\xD1V[P`\xFF\x84\x16`\0\x81\x81R`\x9B` \x90\x81R`@\x91\x82\x90 C\x90\x81\x90U\x91Q\x91\x82R\x7FF\x07}U3\x07c\xF1bi\xFDu\xE5v\x16c\xF4\x19-'\x91t|\x01\x89\xB1j\xD3\x1D\xB0}\xB4\x91\x01`@Q\x80\x91\x03\x90\xA2PPPP\x80a\x1An\x90ae\xADV[\x90Pa\x16RV[PPPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x99` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x81\x01T\x90\x91\x83\x01\x90`\xFF\x16`\x02\x81\x11\x15a\x1A\xD8Wa\x1A\xD8a[\xB1V[`\x02\x81\x11\x15a\x1A\xE9Wa\x1A\xE9a[\xB1V[\x90RP\x92\x91PPV[`\0T`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01Rb\x01\0\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B?W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Bc\x91\x90afBV[a\x1B\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\xBC\x90afdV[`\0\x19`\x01\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[a\x1B\xC6a7\xFBV[`\x96T\x82\x90`\xFF\x90\x81\x16\x90\x82\x16\x10a\x1CFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FRegistryCoordinator.quorumExists`D\x82\x01R\x7F: quorum does not exist\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\n\xBCV[a\x11\xC0\x83\x83a:\xF6V[`\x9C\x81\x81T\x81\x10a\x1C`W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x81V[```\xE1\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x13\x9BW`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x1DHW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x1D\nW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x1C\x9EV[`\x9ET`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1D\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FRegistryCoordinator.onlyEjector:`D\x82\x01R\x7F caller is not the ejector\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\n\xBCV[a\x11\xC0\x83\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa;\xA3\x92PPPV[a\x1E(a7\xFBV[a\x1E2`\0a@SV[V[`\0a\x1Et\x7F;\xFES\x90\xD1h\x8D.\x05n\x0E\xE4\xB6\x8CT\x19\xF4-\x9B\xD0\xC7\x92\xF0\x1A\xCB\x83\x88g\x14\xAD\xEBV\x87\x87\x87\x87\x87`@Q` \x01a\x14\x14\x96\x95\x94\x93\x92\x91\x90ag\x89V[\x96\x95PPPPPPV[```\xE0\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x13\x9BW\x83\x82\x90`\0R` `\0 \x01\x80Ta\x1E\xC1\x90af\xACV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1E\xED\x90af\xACV[\x80\x15a\x1F:W\x80`\x1F\x10a\x1F\x0FWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1F:V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1F\x1DW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x1E\xA2V[`\0a\x11?\x82a-zV[`\0a\x1Fm`dT`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x90P\x90V[```\xE2\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x13\x9BW`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a @W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a \x02W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x1F\x96V[`\x01\x80T`\0\x91\x90\x81\x16\x14\x15a \x80W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\xBC\x90aeJV[\x83\x89\x14a!\x03W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FRegistryCoordinator.registerOper\x90\x82\x01R\x7FatorWithChurn: input length mism`d\x82\x01Rc\x0C.\x8Cm`\xE3\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[`\0a!\x0F3\x88a@\xA5V[\x90Pa!o3\x82\x88\x88\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a!dWa!U`@\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90ah\x0EV[\x81R` \x01\x90`\x01\x01\x90a!8V[PPPPP\x87aA\xD6V[`\0a!\xB63\x83\x8E\x8E\x8E\x8E\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8C\x92Pa0\xA7\x91PPV[\x90P`\0[\x8B\x81\x10\x15a#\x81W`\0`\x97`\0\x8F\x8F\x85\x81\x81\x10a!\xDBWa!\xDBae\x81V[\x91\x90\x91\x015`\xF8\x1C\x82RP` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0 \x81Q``\x81\x01\x83R\x90Tc\xFF\xFF\xFF\xFF\x81\x16\x80\x83Ra\xFF\xFF`\x01` \x1B\x83\x04\x81\x16\x95\x84\x01\x95\x90\x95R`\x01`0\x1B\x90\x91\x04\x90\x93\x16\x91\x81\x01\x91\x90\x91R\x84Q\x80Q\x91\x93P\x90\x84\x90\x81\x10a\"HWa\"Hae\x81V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a#nWa\"\xE9\x8E\x8E\x84\x81\x81\x10a\"qWa\"qae\x81V[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x84`@\x01Q\x84\x81Q\x81\x10a\"\x94Wa\"\x94ae\x81V[` \x02` \x01\x01Q3\x86` \x01Q\x86\x81Q\x81\x10a\"\xB3Wa\"\xB3ae\x81V[` \x02` \x01\x01Q\x8D\x8D\x88\x81\x81\x10a\"\xCDWa\"\xCDae\x81V[\x90P`@\x02\x01\x806\x03\x81\x01\x90a\"\xE3\x91\x90ah\x0EV[\x86aCcV[a#n\x89\x89\x84\x81\x81\x10a\"\xFEWa\"\xFEae\x81V[\x90P`@\x02\x01` \x01` \x81\x01\x90a#\x16\x91\x90aVkV[\x8F\x8F\x85\x90\x86`\x01a#'\x91\x90agGV[\x92a#4\x93\x92\x91\x90ag_V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa;\xA3\x92PPPV[P\x80a#y\x81ae\xADV[\x91PPa!\xBBV[PPPPPPPPPPPPPV[`\x01\x80T`\0\x91\x90\x81\x16\x14\x15a#\xB8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\xBC\x90aeJV[`\0a#\xC43\x85a@\xA5V[\x90P`\0a$\r3\x83\x8B\x8B\x8B\x8B\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8C\x92Pa0\xA7\x91PPV[Q\x90P`\0[\x88\x81\x10\x15a%\x08W`\0\x8A\x8A\x83\x81\x81\x10a$/Wa$/ae\x81V[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x97` R`@\x90 T\x85Q\x91\x93Pc\xFF\xFF\xFF\xFF\x16\x91P\x84\x90\x84\x90\x81\x10a$eWa$eae\x81V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a$\xF5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FRegistryCoordinator.registerOper\x90\x82\x01R\x7Fator: operator count exceeds max`d\x82\x01Rcimum`\xE0\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[P\x80a%\0\x81ae\xADV[\x91PPa$\x13V[PPPPPPPPPPV[```\xDF\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x13\x9BW\x83\x82\x90`\0R` `\0 \x01\x80Ta%W\x90af\xACV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta%\x83\x90af\xACV[\x80\x15a%\xD0W\x80`\x1F\x10a%\xA5Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a%\xD0V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a%\xB3W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a%8V[`\xCFT`\0\x90a\x01\0\x90\x04`\xFF\x16\x15a&\x06WP`\xCFTa\x01\0\x90\x04`\xFF\x16\x90V[`\0sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a'\x0CW`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R`\0\x92\x90\x91a&\x94\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01ah*V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra&\xAE\x91ah[V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a&\xEBW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a&\xF0V[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90a'\x08\x91\x90afBV[\x91PP[\x91\x90PV[```\0\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a'.Wa'.aU\x10V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a'WW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a'\xC2Wa'\x89\x85\x85\x83\x81Q\x81\x10a'|Wa'|ae\x81V[` \x02` \x01\x01QaF8V[\x82\x82\x81Q\x81\x10a'\x9BWa'\x9Bae\x81V[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a'\xBA\x81ae\xADV[\x91PPa']V[P\x93\x92PPPV[`\x01\x80T`\x02\x90\x81\x16\x14\x15a'\xF1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\xBC\x90aeJV[a\x11\xC03\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa;\xA3\x92PPPV[a(9a7\xFBV[a\x11\xC0\x83\x83\x83aGtV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a(dWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a(~WP0;\x15\x80\x15a(~WP`\0T`\xFF\x16`\x01\x14[a(\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\n\xBCV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a)\x04W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[\x82Q\x84Q\x14\x80\x15a)\x16WP\x81Q\x83Q\x14[a)\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FRegistryCoordinator.initialize: `D\x82\x01Rt\r-\xCE\x0E\xAE\x84\r\x8C\xAD\xCC\xEE\x8D\x04\r\xAD.m\xAC.\x8Cm`[\x1B`d\x82\x01R`\x84\x01a\n\xBCV[a)\x89\x89a@SV[a)\x93\x86\x86aI\x8BV[a)\x9C\x88a8ZV[a)\xA5\x87a8\xC3V[`\x9C\x80T`\x01\x81\x81\x01\x83U`\0\x83\x81R\x7F\xAF\x85\xB9\x07\x1D\xFA\xFE\xAC\x14\t\xD3\xF1\xD1\x9B\xAF\xC9\xBC|7\x97L\xDE\x8D\xF0\xEEah\xF0\x08nS\x9C\x92\x83\x01\x80T`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U\x85T\x80\x85\x01\x87U\x85\x01\x80T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x16\x90\x83\x16\x17\x90U\x85T\x93\x84\x01\x90\x95U\x91\x90\x92\x01\x80T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x16\x91\x90\x93\x16\x17\x90\x91U[\x84Q\x81\x10\x15a*\xF6Wa*\xE4\x85\x82\x81Q\x81\x10a*\xA3Wa*\xA3ae\x81V[` \x02` \x01\x01Q\x85\x83\x81Q\x81\x10a*\xBDWa*\xBDae\x81V[` \x02` \x01\x01Q\x85\x84\x81Q\x81\x10a*\xD7Wa*\xD7ae\x81V[` \x02` \x01\x01QaGtV[\x80a*\xEE\x81ae\xADV[\x91PPa*\x85V[P\x80\x15a+=W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPPPV[```\xDB\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x11\x9DW` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x11\x7FWPPPPP\x90P\x90V[a+\xB0a7\xFBV[`\x01`\x01`\xA0\x1B\x03\x81\x16a,\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\n\xBCV[a\x0F7\x81a@SV[`\0`\x02\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,qW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\x95\x91\x90ae\xDBV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a,\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\xBC\x90ae\xF8V[`\x01T\x19\x81\x19`\x01T\x19\x16\x14a-CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\n\xBCV[`\x01\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x0E|V[`\0\x81\x81R`\x98` R`@\x81 T\x80a-\x97WP`\0\x92\x91PPV[`\0\x83\x81R`\x98` R`@\x90 a-\xB0`\x01\x83ahwV[\x81T\x81\x10a-\xC0Wa-\xC0ae\x81V[`\0\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01`\xC0\x1B\x03\x16\x93\x92PPPV[P\x91\x90PV[```\0\x80a-\xF7\x84aJwV[a\xFF\xFF\x16`\x01`\x01`@\x1B\x03\x81\x11\x15a.\x12Wa.\x12aU\x10V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a.<W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x80[\x82Q\x82\x10\x80\x15a.TWPa\x01\0\x81\x10[\x15a.\xABW`\x01\x81\x1B\x93P\x85\x84\x16\x15a.\x9BW\x80`\xF8\x1B\x83\x83\x81Q\x81\x10a.}Wa.}ae\x81V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x81`\x01\x01\x91P[a.\xA4\x81ae\xADV[\x90Pa.CV[P\x90\x94\x93PPPPV[`\x01\x82` \x01Q`\x02\x81\x11\x15a.\xCDWa.\xCDa[\xB1V[\x14a.\xD7WPPPV[\x81Q`@Qc3V\x7F\x7F`\xE1\x1B\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cf\xAC\xFE\xFE\x90a/,\x90\x88\x90\x86\x90\x88\x90`\x04\x01ah\x8EV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a/KW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a/o\x91\x90ah\xBEV[\x90P`\x01`\x01`\xC0\x1B\x03\x81\x16\x15a/\x9BWa/\x9B\x85a/\x96\x83`\x01`\x01`\xC0\x1B\x03\x16a-\xE9V[a;\xA3V[PPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16a00W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[`\0T`@\x80Q`\x01`\x01`\xA0\x1B\x03b\x01\0\0\x90\x93\x04\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`\0\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16b\x01\0\0\x02b\x01\0\0`\x01`\xB0\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[a0\xCB`@Q\x80``\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`\0a1\x13\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`\x96T`\xFF\x16\x91Pa:\n\x90PV[\x90P`\0a1 \x88a-zV[\x90P`\x01`\x01`\xC0\x1B\x03\x82\x16a1\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FRegistryCoordinator._registerOpe`D\x82\x01R\x7Frator: bitmap cannot be 0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\n\xBCV[a1\xA7\x82a:\xC3V[a2\x1BW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FRegistryCoordinator._registerOpe`D\x82\x01R\x7Frator: some quorums do not exist`d\x82\x01R`\x84\x01a\n\xBCV[\x80\x82\x16`\x01`\x01`\xC0\x1B\x03\x16\x15a2\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`h`$\x82\x01R\x7FRegistryCoordinator._registerOpe`D\x82\x01R\x7Frator: operator already register`d\x82\x01R\x7Fed for some quorums being regist`\x84\x82\x01Rg2\xB92\xB2\x1037\xB9`\xC1\x1B`\xA4\x82\x01R`\xC4\x01a\n\xBCV[`\x01`\x01`\xC0\x1B\x03\x81\x81\x16\x90\x83\x16\x17a2\xEA\x89\x82a6;V[\x88\x7F\xEC)c\xAB!\xC1\xE5\x0E\x1EX*\xA5B\xAF.K\xF7\xBF8\xE6\xE1@<'\xB4.\x1C]nb\x1E\xAA\x87`@Qa3\x1A\x91\x90ae\xC8V[`@Q\x80\x91\x03\x90\xA2`\x01`\x01`\x01`\xA0\x1B\x03\x8B\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T`\xFF\x16`\x02\x81\x11\x15a3TWa3Ta[\xB1V[\x14a4mW`@\x80Q\x80\x82\x01\x82R\x8A\x81R`\x01` \x80\x83\x01\x82\x81R`\x01`\x01`\xA0\x1B\x03\x8F\x16`\0\x90\x81R`\x99\x90\x92R\x93\x90 \x82Q\x81U\x92Q\x83\x82\x01\x80T\x93\x94\x93\x91\x92\x90\x91`\xFF\x19\x16\x90\x83`\x02\x81\x11\x15a3\xAFWa3\xAFa[\xB1V[\x02\x17\x90UPP`@Qc\x99&\xEE}`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91Pc\x99&\xEE}\x90a4\x04\x90\x8D\x90\x89\x90`\x04\x01ah\xDBV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a4\x1EW`\0\x80\xFD[PZ\xF1\x15\x80\x15a42W=`\0\x80>=`\0\xFD[PP`@Q\x8B\x92P`\x01`\x01`\xA0\x1B\x03\x8D\x16\x91P\x7F\xE8\xE6\x8C\xEF\x1C:v\x1E\xD7\xBE~\x84c\xA3u\xF2\x7F{\xC35\xE5\x18$\"<\xAC\xCEcn\xC5\xC3\xFE\x90`\0\x90\xA3[`@Qc\x1F\xD9<\xA9`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c?\xB2yR\x90a4\xBD\x90\x8D\x90\x8C\x90\x8C\x90`\x04\x01aiOV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a4\xD7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a4\xEBW=`\0\x80>=`\0\xFD[PP`@Qc%PGw`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92Pc%PGw\x91Pa5A\x90\x8D\x90\x8D\x90\x8D\x90\x8D\x90`\x04\x01aitV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a5`W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra5\x88\x91\x90\x81\x01\x90aj\0V[`@\x80\x87\x01\x91\x90\x91R` \x86\x01\x91\x90\x91RQb\xBF\xF0M`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90b\xBF\xF0M\x90a5\xE5\x90\x8C\x90\x8C\x90\x8C\x90`\x04\x01ajcV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a6\x04W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra6,\x91\x90\x81\x01\x90aj}V[\x84RPPP\x96\x95PPPPPPV[`\0\x82\x81R`\x98` R`@\x90 T\x80a6\xE0W`\0\x83\x81R`\x98` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x86\x81R`\x01`\x01`\xC0\x1B\x03\x80\x8A\x16\x95\x84\x01\x95\x86R\x84T`\x01\x81\x01\x86U\x94\x88R\x95\x90\x96 \x91Q\x91\x90\x92\x01\x80T\x95Q\x93Q\x90\x94\x16`\x01`@\x1B\x02`\x01`\x01`@\x1B\x03\x93\x83\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x96\x16\x91\x90\x92\x16\x17\x93\x90\x93\x17\x16\x91\x90\x91\x17\x90UPPPV[`\0\x83\x81R`\x98` R`@\x81 a6\xF9`\x01\x84ahwV[\x81T\x81\x10a7\tWa7\tae\x81V[`\0\x91\x82R` \x90\x91 \x01\x80T\x90\x91PCc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14\x15a7MW\x80T`\x01`\x01`@\x1B\x03\x16`\x01`@\x1B`\x01`\x01`\xC0\x1B\x03\x85\x16\x02\x17\x81Ua\x0B\xA3V[\x80Tc\xFF\xFF\xFF\xFFC\x81\x16`\x01` \x1B\x81\x81\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x94\x16\x93\x90\x93\x17\x84U`\0\x87\x81R`\x98` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83R\x94\x85R\x84\x83\x01\x84\x81R`\x01`\x01`\xC0\x1B\x03\x80\x8C\x16\x93\x87\x01\x93\x84R\x82T`\x01\x81\x01\x84U\x92\x86R\x93\x90\x94 \x94Q\x94\x01\x80T\x93Q\x91Q\x90\x92\x16`\x01`@\x1B\x02`\x01`\x01`@\x1B\x03\x91\x86\x16\x90\x96\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x93\x90\x94\x16\x92\x90\x92\x17\x17\x91\x90\x91\x16\x91\x90\x91\x17\x90UPPPPV[3a8\x04a\x1FYV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1E2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\n\xBCV[`\x9DT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F1TW\xD8\xA8\xFE`\xF0J\xF1|\x16\xE2\xF5\xA5\xE1\xDBa+1d\x8EX\x03\x03`u\x9E\xF8\xF3R\x8C\x91\x01`@Q\x80\x91\x03\x90\xA1`\x9D\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x9ET`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F\x8F0\xAB\t\xF4:l\x15}\x7F\xCE~\n\x13\xC0\x03\x04,\x1C\x95\xE8\xA7.z\x14j!\xC0\xCA\xA2M\xC9\x91\x01`@Q\x80\x91\x03\x90\xA1`\x9E\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0a\x11?a99aJ\xA2V[\x83`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x83\x90R`B\x81\x01\x82\x90R`\0\x90`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0\x80\x80a9\xAA`\0\x80Q` am\xA7\x839\x81Q\x91R\x86ak!V[\x90P[a9\xB6\x81aK\xC9V[\x90\x93P\x91P`\0\x80Q` am\xA7\x839\x81Q\x91R\x82\x83\t\x83\x14\x15a9\xF0W`@\x80Q\x80\x82\x01\x90\x91R\x90\x81R` \x81\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80Q` am\xA7\x839\x81Q\x91R`\x01\x82\x08\x90Pa9\xADV[`\0\x80a:\x16\x84aLKV[\x90P\x80\x15a:\xBCW\x82`\xFF\x16\x84`\x01\x86Qa:1\x91\x90ahwV[\x81Q\x81\x10a:AWa:Aae\x81V[\x01` \x01Q`\xF8\x1C\x10a:\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`?`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: bitmap exceeds max value\0`d\x82\x01R`\x84\x01a\n\xBCV[\x93\x92PPPV[`\x96T`\0\x90\x81\x90a:\xDD\x90`\x01\x90`\xFF\x16\x81\x90\x1BahwV[\x90Pa:\xBC`\x01`\x01`\xC0\x1B\x03\x84\x81\x16\x90\x83\x16\x81\x16\x14\x90V[`\xFF\x82\x16`\0\x81\x81R`\x97` \x90\x81R`@\x91\x82\x90 \x84Q\x81T\x86\x84\x01\x80Q\x88\x87\x01\x80Qc\xFF\xFF\xFF\xFF\x90\x95\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x94\x16\x84\x17`\x01` \x1Ba\xFF\xFF\x93\x84\x16\x02\x17g\xFF\xFF\0\0\0\0\0\0\x19\x16`\x01`0\x1B\x95\x83\x16\x95\x90\x95\x02\x94\x90\x94\x17\x90\x94U\x85Q\x91\x82RQ\x83\x16\x93\x81\x01\x93\x90\x93RQ\x16\x91\x81\x01\x91\x90\x91R\x7F>\xE6\xFE\x8DTa\x02D\xC3\xE9\xD3\xC0f\xAEJ\xEE\x99x\x84\xAA(\xF1\x06\x16\xAE\x82\x19%@\x13\x18\xAC\x90``\x01`@Q\x80\x91\x03\x90\xA2PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x99` R`@\x90 \x80T`\x01\x80\x83\x01T`\xFF\x16`\x02\x81\x11\x15a;\xD7Wa;\xD7a[\xB1V[\x14a<DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R`\0\x80Q` am\xC7\x839\x81Q\x91R`D\x82\x01R\x7Fperator: operator is not registe`d\x82\x01Rb\x1C\x99Y`\xEA\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[`\x96T`\0\x90a<X\x90\x85\x90`\xFF\x16a:\nV[\x90P`\0a<e\x83a-zV[\x90P`\x01`\x01`\xC0\x1B\x03\x82\x16a<\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`;`$\x82\x01R`\0\x80Q` am\xC7\x839\x81Q\x91R`D\x82\x01R\x7Fperator: bitmap cannot be 0\0\0\0\0\0`d\x82\x01R`\x84\x01a\n\xBCV[a<\xDA\x82a:\xC3V[a=EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`B`$\x82\x01R`\0\x80Q` am\xC7\x839\x81Q\x91R`D\x82\x01R\x7Fperator: some quorums do not exi`d\x82\x01Ra\x1C\xDD`\xF2\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[a=\\`\x01`\x01`\xC0\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x16\x14\x90V[a=\xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`Y`$\x82\x01R`\0\x80Q` am\xC7\x839\x81Q\x91R`D\x82\x01R\x7Fperator: operator is not registe`d\x82\x01R\x7Fred for specified quorums\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\n\xBCV[`\x01`\x01`\xC0\x1B\x03\x82\x81\x16\x19\x82\x16\x16a=\xFB\x84\x82a6;V[`\x01`\x01`\xC0\x1B\x03\x81\x16a>\xCAW`\x01\x85\x01\x80T`\xFF\x19\x16`\x02\x17\x90U`@QcQ\xB2zm`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA3d\xF4\xDA\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a>{W`\0\x80\xFD[PZ\xF1\x15\x80\x15a>\x8FW=`\0\x80>=`\0\xFD[PP`@Q\x86\x92P`\x01`\x01`\xA0\x1B\x03\x8A\x16\x91P\x7F9o\xDC\xB1\x80\xCB\x0F\xEA&\x92\x81\x13\xFB\x0F\xD1\xC3T\x98c\xF9\xCDV>j\x18O\x1DW\x81\x16\xC8\xE4\x90`\0\x90\xA3[`@Qc\xF4\xE2O\xE5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xF4\xE2O\xE5\x90a?\x18\x90\x8A\x90\x8A\x90`\x04\x01ak5V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a?2W`\0\x80\xFD[PZ\xF1\x15\x80\x15a?FW=`\0\x80>=`\0\xFD[PP`@Qc\xBD)\xB8\xCD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92Pc\xBD)\xB8\xCD\x91Pa?\x98\x90\x87\x90\x8A\x90`\x04\x01akYV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a?\xB2W`\0\x80\xFD[PZ\xF1\x15\x80\x15a?\xC6W=`\0\x80>=`\0\xFD[PP`@Qc\xBD)\xB8\xCD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92Pc\xBD)\xB8\xCD\x91Pa@\x18\x90\x87\x90\x8A\x90`\x04\x01akYV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a@2W`\0\x80\xFD[PZ\xF1\x15\x80\x15a@FW=`\0\x80>=`\0\xFD[PPPPPPPPPPPV[`d\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\x13T*N\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aA\x10W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aA4\x91\x90akrV[\x90P\x80a\x11?W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xBFy\xCEX\x84\x84aAu\x87a\x13\xB5V[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aA\x93\x93\x92\x91\x90ak\x8BV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15aA\xB2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a:\xBC\x91\x90akrV[` \x80\x82\x01Q`\0\x90\x81R`\x9A\x90\x91R`@\x90 T`\xFF\x16\x15aB|W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`R`$\x82\x01R\x7FRegistryCoordinator._verifyChurn`D\x82\x01R\x7FApproverSignature: churnApprover`d\x82\x01Rq\x08\x1C\xD8[\x1D\x08\x18[\x1C\x99XY\x1EH\x1D\\\xD9Y`r\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[B\x81`@\x01Q\x10\x15aC\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`R`$\x82\x01R\x7FRegistryCoordinator._verifyChurn`D\x82\x01R\x7FApproverSignature: churnApprover`d\x82\x01Rq\x08\x1C\xDAY\xDB\x98]\x1D\\\x99H\x19^\x1C\x1A\\\x99Y`r\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[` \x80\x82\x01\x80Q`\0\x90\x81R`\x9A\x90\x92R`@\x91\x82\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U`\x9DT\x90Q\x91\x83\x01Qa\x0B\xA3\x92`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91aC\\\x91\x88\x91\x88\x91\x88\x91\x90a\x1E4V[\x83QaM\xD8V[` \x80\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`\0\x81\x81R`\x99\x90\x94R`@\x90\x93 T\x91\x92\x90\x87\x16\x14\x15aC\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R`\0\x80Q` am\x87\x839\x81Q\x91R`D\x82\x01Rt97\x1D\x101\xB0\xB777\xBA\x101\xB4:\xB97\x109\xB2\xB63`Y\x1B`d\x82\x01R`\x84\x01a\n\xBCV[\x87`\xFF\x16\x84`\0\x01Q`\xFF\x16\x14aD`W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R`\0\x80Q` am\x87\x839\x81Q\x91R`D\x82\x01R\x7Frn: quorumNumber not the same as`d\x82\x01Rf\x08\x1C\xDAY\xDB\x99Y`\xCA\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[`@QcT\x01\xED'`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`\xFF\x89\x16`$\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cT\x01\xED'\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aD\xD1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aD\xF5\x91\x90al\nV[\x90PaE\x01\x81\x85aO\x92V[`\x01`\x01``\x1B\x03\x16\x86`\x01`\x01``\x1B\x03\x16\x11aE\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`V`$\x82\x01R`\0\x80Q` am\x87\x839\x81Q\x91R`D\x82\x01R\x7Frn: incoming operator has insuff`d\x82\x01Ru4\xB1\xB4\xB2\xB7:\x109\xBA0\xB5\xB2\x9037\xB9\x101\xB4:\xB97`Q\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[aE\x9E\x88\x85aO\xB6V[`\x01`\x01``\x1B\x03\x16\x81`\x01`\x01``\x1B\x03\x16\x10a+=W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R`\0\x80Q` am\x87\x839\x81Q\x91R`D\x82\x01R\x7Frn: cannot kick operator with mo`d\x82\x01R\x7Fre than kickBIPsOfTotalStake\0\0\0\0`\x84\x82\x01R`\xA4\x01a\n\xBCV[`\0\x81\x81R`\x98` R`@\x81 T\x81[\x81\x81\x10\x15aF\xCAW`\x01aF]\x82\x84ahwV[aFg\x91\x90ahwV[\x92P\x84c\xFF\xFF\xFF\xFF\x16`\x98`\0\x86\x81R` \x01\x90\x81R` \x01`\0 \x84c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10aF\x9AWaF\x9Aae\x81V[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11aF\xB8WPPa\x11?V[\x80aF\xC2\x81ae\xADV[\x91PPaFIV[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`l`$\x82\x01R\x7FRegistryCoordinator.getQuorumBit`D\x82\x01R\x7FmapIndexAtBlockNumber: no bitmap`d\x82\x01R\x7F update found for operatorId at `\x84\x82\x01Rk167\xB1\xB5\x907:\xB6\xB12\xB9`\xA1\x1B`\xA4\x82\x01R`\xC4\x01a\n\xBCV[`\x96T`\xFF\x16`\xC0\x81\x10aG\xE8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FRegistryCoordinator.createQuorum`D\x82\x01Rt\x0E\x88\x1BX^\x08\x1C][\xDC\x9D[\\\xC8\x1C\x99XX\xDA\x19Y`Z\x1B`d\x82\x01R`\x84\x01a\n\xBCV[aG\xF3\x81`\x01al'V[`\x96\x80T`\xFF\x19\x16`\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80aH\x12\x81\x86a:\xF6V[`@Q`\x01b\x96\xB5\x89`\xE0\x1B\x03\x19\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xFFiJw\x90aHe\x90\x84\x90\x88\x90\x88\x90`\x04\x01alLV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aH\x7FW`\0\x80\xFD[PZ\xF1\x15\x80\x15aH\x93W=`\0\x80>=`\0\xFD[PP`@Qc\x13l\xA0\xF9`\xE1\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x92Pc&\xD9A\xF2\x91P`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aH\xFBW`\0\x80\xFD[PZ\xF1\x15\x80\x15aI\x0FW=`\0\x80>=`\0\xFD[PP`@Qc\x13l\xA0\xF9`\xE1\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x92Pc&\xD9A\xF2\x91P`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aIwW`\0\x80\xFD[PZ\xF1\x15\x80\x15a+=W=`\0\x80>=`\0\xFD[`\0Tb\x01\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15aI\xB2WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[aJ4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[`\x01\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a\x11\xB1\x82a/\xA2V[`\0\x80[\x82\x15a\x11?WaJ\x8C`\x01\x84ahwV[\x90\x92\x16\x91\x80aJ\x9A\x81al\xC5V[\x91PPaJ{V[`\x000`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80\x15aJ\xFBWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14[\x15aK%WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[P`@\x80Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x80\x83\x01\x91\x90\x91R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x84\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x83\x01RF`\x80\x83\x01R0`\xA0\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\xC0\x90\x92\x01\x90\x92R\x80Q\x91\x01 \x90V[`\0\x80\x80`\0\x80Q` am\xA7\x839\x81Q\x91R`\x03`\0\x80Q` am\xA7\x839\x81Q\x91R\x86`\0\x80Q` am\xA7\x839\x81Q\x91R\x88\x89\t\t\x08\x90P`\0aL?\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\0\x80Q` am\xA7\x839\x81Q\x91RaO\xD0V[\x91\x95\x91\x94P\x90\x92PPPV[`\0a\x01\0\x82Q\x11\x15aL\xD4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FBitmapUtils.orderedBytesArrayToB\x90\x82\x01R\x7Fitmap: orderedBytesArray is too `d\x82\x01Rclong`\xE0\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[\x81QaL\xE2WP`\0\x91\x90PV[`\0\x80\x83`\0\x81Q\x81\x10aL\xF8WaL\xF8ae\x81V[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x81\x90\x1B\x92P[\x84Q\x81\x10\x15aM\xCFW\x84\x81\x81Q\x81\x10aM&WaM&ae\x81V[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x1B\x91P\x82\x82\x11aM\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: orderedBytesArray is not `d\x82\x01Rf\x1B\xDC\x99\x19\\\x99Y`\xCA\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[\x91\x81\x17\x91aM\xC8\x81ae\xADV[\x90PaM\x0BV[P\x90\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16;\x15aN\xF2W`@Qc\x0B\x13]?`\xE1\x1B\x80\x82R\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\x16&\xBA~\x90aN\x18\x90\x86\x90\x86\x90`\x04\x01akYV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aN5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aNY\x91\x90al\xE7V[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14a\x11\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`S`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: ERC1271 signature `d\x82\x01Rr\x1D\x99\\\x9AY\x9AX\xD8]\x1A[\xDB\x88\x19\x98Z[\x19Y`j\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[\x82`\x01`\x01`\xA0\x1B\x03\x16aO\x06\x83\x83aP\x7FV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x11\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: signature not from`d\x82\x01Rf\x109\xB4\xB3\xB72\xB9`\xC9\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[` \x81\x01Q`\0\x90a'\x10\x90aO\xAC\x90a\xFF\xFF\x16\x85am\x11V[a:\xBC\x91\x90am@V[`@\x81\x01Q`\0\x90a'\x10\x90aO\xAC\x90a\xFF\xFF\x16\x85am\x11V[`\0\x80aO\xDBaS\xECV[aO\xE3aT\nV[` \x80\x82R\x81\x81\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x88\x90R`\x80\x82\x01\x87\x90R`\xA0\x82\x01\x86\x90R\x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82\x80\x15aP$WaP&V[\xFE[P\x82aPtW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FBN254.expMod: call failure\0\0\0\0\0\0`D\x82\x01R`d\x01a\n\xBCV[PQ\x95\x94PPPPPV[`\0\x80`\0aP\x8E\x85\x85aP\x9BV[\x91P\x91Pa'\xC2\x81aQ\x0BV[`\0\x80\x82Q`A\x14\x15aP\xD2W` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1AaP\xC6\x87\x82\x85\x85aR\xC6V[\x94P\x94PPPPaQ\x04V[\x82Q`@\x14\x15aP\xFCW` \x83\x01Q`@\x84\x01QaP\xF1\x86\x83\x83aS\xB3V[\x93P\x93PPPaQ\x04V[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0\x81`\x04\x81\x11\x15aQ\x1FWaQ\x1Fa[\xB1V[\x14\x15aQ(WPV[`\x01\x81`\x04\x81\x11\x15aQ<WaQ<a[\xB1V[\x14\x15aQ\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\n\xBCV[`\x02\x81`\x04\x81\x11\x15aQ\x9EWaQ\x9Ea[\xB1V[\x14\x15aQ\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\n\xBCV[`\x03\x81`\x04\x81\x11\x15aR\0WaR\0a[\xB1V[\x14\x15aRYW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\n\xBCV[`\x04\x81`\x04\x81\x11\x15aRmWaRma[\xB1V[\x14\x15a\x0F7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 'v' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\n\xBCV[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15aR\xFDWP`\0\x90P`\x03aS\xAAV[\x84`\xFF\x16`\x1B\x14\x15\x80\x15aS\x15WP\x84`\xFF\x16`\x1C\x14\x15[\x15aS&WP`\0\x90P`\x04aS\xAAV[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15aSzW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16aS\xA3W`\0`\x01\x92P\x92PPaS\xAAV[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x80`\x01`\x01`\xFF\x1B\x03\x83\x16\x81aS\xD0`\xFF\x86\x90\x1C`\x1BagGV[\x90PaS\xDE\x87\x82\x88\x85aR\xC6V[\x93P\x93PPP\x93P\x93\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`\0\x80\x83`\x1F\x84\x01\x12aT:W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aTQW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15aQ\x04W`\0\x80\xFD[`\0\x80` \x83\x85\x03\x12\x15aT\x7FW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aT\x95W`\0\x80\xFD[aT\xA1\x85\x82\x86\x01aT(V[\x90\x96\x90\x95P\x93PPPPV[`\0` \x82\x84\x03\x12\x15aT\xBFW`\0\x80\xFD[P5\x91\x90PV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0F7W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15aT\xEDW`\0\x80\xFD[\x835\x92P` \x84\x015aT\xFF\x81aT\xC6V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aUHWaUHaU\x10V[`@R\x90V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aUHWaUHaU\x10V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aU\x98WaU\x98aU\x10V[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12aU\xB1W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aU\xCAWaU\xCAaU\x10V[aU\xDD`\x1F\x82\x01`\x1F\x19\x16` \x01aUpV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15aU\xF2W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15aV!W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aV7W`\0\x80\xFD[aVC\x84\x82\x85\x01aU\xA0V[\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0F7W`\0\x80\xFD[\x805a'\x0C\x81aVKV[`\0` \x82\x84\x03\x12\x15aV}W`\0\x80\xFD[\x815a:\xBC\x81aVKV[`\0\x80\x83`\x1F\x84\x01\x12aV\x9AW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aV\xB1W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aQ\x04W`\0\x80\xFD[`\0``\x82\x84\x03\x12\x15aV\xDBW`\0\x80\xFD[aV\xE3aU&V[\x90P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aV\xFBW`\0\x80\xFD[aW\x07\x84\x82\x85\x01aU\xA0V[\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\xA0\x87\x89\x03\x12\x15aW=W`\0\x80\xFD[\x865aWH\x81aVKV[\x95P` \x87\x015\x94P`@\x87\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aWkW`\0\x80\xFD[aWw\x8A\x83\x8B\x01aV\x88V[\x90\x96P\x94P``\x89\x015\x91P\x80\x82\x11\x15aW\x90W`\0\x80\xFD[aW\x9C\x8A\x83\x8B\x01aU\xA0V[\x93P`\x80\x89\x015\x91P\x80\x82\x11\x15aW\xB2W`\0\x80\xFD[PaW\xBF\x89\x82\x8A\x01aV\xC9V[\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aX\x05W\x81Q`\x01`\x01``\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aW\xE0V[P\x94\x95\x94PPPPPV[` \x80\x82R\x82Q``\x83\x83\x01R\x80Q`\x80\x84\x01\x81\x90R`\0\x92\x91\x82\x01\x90\x83\x90`\xA0\x86\x01\x90[\x80\x83\x10\x15aX[W\x83Qc\xFF\xFF\xFF\xFF\x16\x82R\x92\x84\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x84\x01\x90aX5V[P\x83\x87\x01Q\x93P`\x1F\x19\x92P\x82\x86\x82\x03\x01`@\x87\x01RaX{\x81\x85aW\xCCV[\x93PPP`@\x85\x01Q\x81\x85\x84\x03\x01``\x86\x01Ra\x1Et\x83\x82aW\xCCV[`\0\x80`@\x83\x85\x03\x12\x15aX\xABW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aX\xFBW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aX\xD6V[P\x90\x96\x95PPPPPPV[\x805`\xFF\x81\x16\x81\x14a'\x0CW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aY*W`\0\x80\xFD[a:\xBC\x82aY\x07V[`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x14a\x0F7W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15aY[W`\0\x80\xFD[\x825\x91P` \x83\x015aYm\x81aY3V[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0\x83\x85\x03`\x80\x81\x12\x15aY\x8EW`\0\x80\xFD[\x845aY\x99\x81aVKV[\x93P`@`\x1F\x19\x82\x01\x12\x15aY\xADW`\0\x80\xFD[PaY\xB6aUNV[` \x85\x015\x81R`@\x85\x015`\x03\x81\x10aY\xCFW`\0\x80\xFD[` \x82\x01R\x91P``\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aY\xEFW`\0\x80\xFD[aY\xFB\x86\x82\x87\x01aU\xA0V[\x91PP\x92P\x92P\x92V[`\0[\x83\x81\x10\x15aZ W\x81\x81\x01Q\x83\x82\x01R` \x01aZ\x08V[\x83\x81\x11\x15a\x0B\xA3WPP`\0\x91\x01RV[`\0\x81Q\x80\x84RaZI\x81` \x86\x01` \x86\x01aZ\x05V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0[\x85\x81\x10\x15aZ\xA5W\x82\x84\x03\x89RaZ\x93\x84\x83QaZ1V[\x98\x85\x01\x98\x93P\x90\x84\x01\x90`\x01\x01aZ{V[P\x91\x97\x96PPPPPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P\x82\x86\x01\x91P\x82\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0[\x83\x81\x10\x15a[!W\x88\x83\x03`?\x19\x01\x85R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x87\x01Q\x87\x84\x01\x87\x90Ra[\x0E\x87\x85\x01\x82aZ]V[\x95\x88\x01\x95\x93PP\x90\x86\x01\x90`\x01\x01aZ\xD9V[P\x90\x98\x97PPPPPPPPV[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x81\x01a\x11?V[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a[\\W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a[sW`\0\x80\xFD[a[\x7F\x88\x83\x89\x01aT(V[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a[\x98W`\0\x80\xFD[Pa[\xA5\x87\x82\x88\x01aV\x88V[\x95\x98\x94\x97P\x95PPPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x03\x81\x10a[\xE5WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[\x81Q\x81R` \x80\x83\x01Q`@\x83\x01\x91a\\\x04\x90\x84\x01\x82a[\xC7V[P\x92\x91PPV[\x805a\xFF\xFF\x81\x16\x81\x14a'\x0CW`\0\x80\xFD[`\0``\x82\x84\x03\x12\x15a\\/W`\0\x80\xFD[a\\7aU&V[\x90P\x815a\\D\x81aT\xC6V[\x81Ra\\R` \x83\x01a\\\x0BV[` \x82\x01Ra\\c`@\x83\x01a\\\x0BV[`@\x82\x01R\x92\x91PPV[`\0\x80`\x80\x83\x85\x03\x12\x15a\\\x81W`\0\x80\xFD[a\\\x8A\x83aY\x07V[\x91Pa\\\x99\x84` \x85\x01a\\\x1DV[\x90P\x92P\x92\x90PV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P\x82\x86\x01\x91P\x82\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0\x80[\x84\x81\x10\x15a]FW\x89\x84\x03`?\x19\x01\x86R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x88\x01Q\x88\x85\x01\x88\x90R\x80Q\x88\x86\x01\x81\x90R\x90\x89\x01\x90\x83\x90``\x87\x01\x90[\x80\x83\x10\x15a]1W\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x82R\x92\x8B\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x8B\x01\x90a]\x07V[P\x97\x8A\x01\x97\x95PPP\x91\x87\x01\x91`\x01\x01a\\\xCAV[P\x91\x99\x98PPPPPPPPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a]jW`\0\x80\xFD[\x835a]u\x81aVKV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a]\x90W`\0\x80\xFD[a]\x9C\x86\x82\x87\x01aV\x88V[\x94\x97\x90\x96P\x93\x94PPPPV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a]\xC2Wa]\xC2aU\x10V[P`\x05\x1B` \x01\x90V[`\0`@\x82\x84\x03\x12\x15a]\xDEW`\0\x80\xFD[a]\xE6aUNV[\x90Pa]\xF1\x82aY\x07V[\x81R` \x82\x015a^\x01\x81aVKV[` \x82\x01R\x92\x91PPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a^$W`\0\x80\xFD[\x855a^/\x81aVKV[\x94P` \x86\x81\x015\x94P`@\x80\x88\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a^SW`\0\x80\xFD[\x88\x01`\x1F\x81\x01\x8A\x13a^dW`\0\x80\xFD[\x805a^wa^r\x82a]\xA9V[aUpV[\x81\x81R`\x06\x91\x90\x91\x1B\x82\x01\x84\x01\x90\x84\x81\x01\x90\x8C\x83\x11\x15a^\x96W`\0\x80\xFD[\x92\x85\x01\x92[\x82\x84\x10\x15a^\xBCWa^\xAD\x8D\x85a]\xCCV[\x82R\x92\x84\x01\x92\x90\x85\x01\x90a^\x9BV[\x99\x9C\x98\x9BP\x98\x99``\x81\x015\x99P`\x80\x015\x97\x96PPPPPPPV[` \x81R`\0a:\xBC` \x83\x01\x84aZ]V[`\0a\x01\0\x82\x84\x03\x12\x15a-\xE3W`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12a_\x11W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a_(W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x06\x1B\x85\x01\x01\x11\x15aQ\x04W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0\x80`\0a\x01\xA0\x8A\x8C\x03\x12\x15a_bW`\0\x80\xFD[\x895`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a_yW`\0\x80\xFD[a_\x85\x8D\x83\x8E\x01aV\x88V[\x90\x9BP\x99P` \x8C\x015\x91P\x80\x82\x11\x15a_\x9EW`\0\x80\xFD[a_\xAA\x8D\x83\x8E\x01aV\x88V[\x90\x99P\x97P\x87\x91Pa_\xBF\x8D`@\x8E\x01a^\xECV[\x96Pa\x01@\x8C\x015\x91P\x80\x82\x11\x15a_\xD6W`\0\x80\xFD[a_\xE2\x8D\x83\x8E\x01a^\xFFV[\x90\x96P\x94Pa\x01`\x8C\x015\x91P\x80\x82\x11\x15a_\xFCW`\0\x80\xFD[a`\x08\x8D\x83\x8E\x01aV\xC9V[\x93Pa\x01\x80\x8C\x015\x91P\x80\x82\x11\x15a`\x1FW`\0\x80\xFD[Pa`,\x8C\x82\x8D\x01aV\xC9V[\x91PP\x92\x95\x98P\x92\x95\x98P\x92\x95\x98V[`\0\x80`\0\x80`\0\x80a\x01`\x87\x89\x03\x12\x15a`VW`\0\x80\xFD[\x865`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a`mW`\0\x80\xFD[a`y\x8A\x83\x8B\x01aV\x88V[\x90\x98P\x96P` \x89\x015\x91P\x80\x82\x11\x15a`\x92W`\0\x80\xFD[a`\x9E\x8A\x83\x8B\x01aV\x88V[\x90\x96P\x94P\x84\x91Pa`\xB3\x8A`@\x8B\x01a^\xECV[\x93Pa\x01@\x89\x015\x91P\x80\x82\x11\x15aW\xB2W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a`\xDDW`\0\x80\xFD[\x825a`\xE8\x81aT\xC6V[\x91P` \x83\x81\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aa\x04W`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13aa\x15W`\0\x80\xFD[\x805aa#a^r\x82a]\xA9V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x88\x83\x11\x15aaBW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aa`W\x835\x82R\x92\x84\x01\x92\x90\x84\x01\x90aaGV[\x80\x95PPPPPP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aX\xFBW\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aa\x8BV[`\0\x80` \x83\x85\x03\x12\x15aa\xC0W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aa\xD6W`\0\x80\xFD[aT\xA1\x85\x82\x86\x01aV\x88V[`\x01`\x01``\x1B\x03\x81\x16\x81\x14a\x0F7W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12ab\x08W`\0\x80\xFD[\x815` ab\x18a^r\x83a]\xA9V[\x82\x81R`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15ab7W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15ab\x88W`@\x81\x89\x03\x12\x15abTW`\0\x80\x81\xFD[ab\\aUNV[\x815abg\x81aVKV[\x81R\x81\x85\x015abv\x81aa\xE2V[\x81\x86\x01R\x83R\x91\x83\x01\x91`@\x01ab;V[P\x96\x95PPPPPPV[`\0\x80`\0`\xA0\x84\x86\x03\x12\x15ab\xA8W`\0\x80\xFD[ab\xB2\x85\x85a\\\x1DV[\x92P``\x84\x015ab\xC2\x81aa\xE2V[\x91P`\x80\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15ab\xDDW`\0\x80\xFD[aY\xFB\x86\x82\x87\x01aa\xF7V[`\0\x80`@\x83\x85\x03\x12\x15ab\xFCW`\0\x80\xFD[\x825ac\x07\x81aVKV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x82`\x1F\x83\x01\x12ac&W`\0\x80\xFD[\x815` ac6a^r\x83a]\xA9V[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15acUW`\0\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15acxWack\x89\x82a\\\x1DV[\x84R\x92\x84\x01\x92\x81\x01acYV[P\x90\x97\x96PPPPPPPV[`\0\x82`\x1F\x83\x01\x12ac\x96W`\0\x80\xFD[\x815` ac\xA6a^r\x83a]\xA9V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15ac\xC5W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15ab\x88W\x805ac\xDC\x81aa\xE2V[\x83R\x91\x83\x01\x91\x83\x01ac\xC9V[`\0\x82`\x1F\x83\x01\x12ac\xFAW`\0\x80\xFD[\x815` ad\na^r\x83a]\xA9V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15ad)W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15ab\x88W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15adLW`\0\x80\x81\xFD[adZ\x89\x86\x83\x8B\x01\x01aa\xF7V[\x84RP\x91\x83\x01\x91\x83\x01ad-V[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15ad\x85W`\0\x80\xFD[ad\x8E\x89aV`V[\x97Pad\x9C` \x8A\x01aV`V[\x96Pad\xAA`@\x8A\x01aV`V[\x95Pad\xB8``\x8A\x01aV`V[\x94P`\x80\x89\x015\x93P`\xA0\x89\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15ad\xDBW`\0\x80\xFD[ad\xE7\x8C\x83\x8D\x01ac\x15V[\x94P`\xC0\x8B\x015\x91P\x80\x82\x11\x15ad\xFDW`\0\x80\xFD[ae\t\x8C\x83\x8D\x01ac\x85V[\x93P`\xE0\x8B\x015\x91P\x80\x82\x11\x15ae\x1FW`\0\x80\xFD[Pae,\x8B\x82\x8C\x01ac\xE9V[\x91PP\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[` \x81\x01a\x11?\x82\x84a[\xC7V[` \x80\x82R`\x19\x90\x82\x01R\x7FPausable: index is paused\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15ae\xC1Wae\xC1ae\x97V[P`\x01\x01\x90V[` \x81R`\0a:\xBC` \x83\x01\x84aZ1V[`\0` \x82\x84\x03\x12\x15ae\xEDW`\0\x80\xFD[\x81Qa:\xBC\x81aVKV[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15afTW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a:\xBCW`\0\x80\xFD[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80af\xC0W`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15a-\xE3WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12af\xF8W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15ag\x12W`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15aQ\x04W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15ag<W`\0\x80\xFD[\x81Qa:\xBC\x81aT\xC6V[`\0\x82\x19\x82\x11\x15agZWagZae\x97V[P\x01\x90V[`\0\x80\x85\x85\x11\x15agoW`\0\x80\xFD[\x83\x86\x11\x15ag|W`\0\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[`\0`\xC0\x82\x01\x88\x83R` `\x01\x80`\xA0\x1B\x03\x80\x8A\x16\x82\x86\x01R`@\x89\x81\x87\x01R`\xC0``\x87\x01R\x83\x89Q\x80\x86R`\xE0\x88\x01\x91P\x84\x8B\x01\x95P`\0[\x81\x81\x10\x15ag\xEEW\x86Q\x80Q`\xFF\x16\x84R\x86\x01Q\x85\x16\x86\x84\x01R\x95\x85\x01\x95\x91\x83\x01\x91`\x01\x01ag\xC4V[PP`\x80\x87\x01\x98\x90\x98RPPPP`\xA0\x90\x91\x01\x91\x90\x91RP\x94\x93PPPPV[`\0`@\x82\x84\x03\x12\x15ah W`\0\x80\xFD[a:\xBC\x83\x83a]\xCCV[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x81R\x81Q`\0\x90ahM\x81`\x04\x85\x01` \x87\x01aZ\x05V[\x91\x90\x91\x01`\x04\x01\x93\x92PPPV[`\0\x82Qahm\x81\x84` \x87\x01aZ\x05V[\x91\x90\x91\x01\x92\x91PPV[`\0\x82\x82\x10\x15ah\x89Wah\x89ae\x97V[P\x03\x90V[`\x01\x80`\xA0\x1B\x03\x84\x16\x81R\x82` \x82\x01R```@\x82\x01R`\0ah\xB5``\x83\x01\x84aZ1V[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15ah\xD0W`\0\x80\xFD[\x81Qa:\xBC\x81aY3V[`\x01\x80`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R`\0\x82Q```@\x84\x01Rai\x05`\xA0\x84\x01\x82aZ1V[\x90P` \x84\x01Q``\x84\x01R`@\x84\x01Q`\x80\x84\x01R\x80\x91PP\x93\x92PPPV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R`@` \x82\x01\x81\x90R`\0\x90ah\xB5\x90\x83\x01\x84\x86ai&V[`\x01\x80`\xA0\x1B\x03\x85\x16\x81R\x83` \x82\x01R```@\x82\x01R`\0a\x1Et``\x83\x01\x84\x86ai&V[`\0\x82`\x1F\x83\x01\x12ai\xADW`\0\x80\xFD[\x81Q` ai\xBDa^r\x83a]\xA9V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15ai\xDCW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15ab\x88W\x80Qai\xF3\x81aa\xE2V[\x83R\x91\x83\x01\x91\x83\x01ai\xE0V[`\0\x80`@\x83\x85\x03\x12\x15aj\x13W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aj*W`\0\x80\xFD[aj6\x86\x83\x87\x01ai\x9CV[\x93P` \x85\x01Q\x91P\x80\x82\x11\x15ajLW`\0\x80\xFD[PajY\x85\x82\x86\x01ai\x9CV[\x91PP\x92P\x92\x90PV[\x83\x81R`@` \x82\x01R`\0ah\xB5`@\x83\x01\x84\x86ai&V[`\0` \x80\x83\x85\x03\x12\x15aj\x90W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aj\xA6W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aj\xB7W`\0\x80\xFD[\x80Qaj\xC5a^r\x82a]\xA9V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15aj\xE4W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x10\xA9W\x83Qaj\xFC\x81aT\xC6V[\x82R\x92\x84\x01\x92\x90\x84\x01\x90aj\xE9V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82ak0Wak0ak\x0BV[P\x06\x90V[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R`\0\x90aVC\x90\x83\x01\x84aZ1V[\x82\x81R`@` \x82\x01R`\0aVC`@\x83\x01\x84aZ1V[`\0` \x82\x84\x03\x12\x15ak\x84W`\0\x80\xFD[PQ\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81Ra\x01`\x81\x01ak\xB3` \x83\x01\x85\x805\x82R` \x90\x81\x015\x91\x01RV[ak\xCD``\x83\x01`@\x86\x01\x805\x82R` \x90\x81\x015\x91\x01RV[`@`\x80\x85\x01`\xA0\x84\x017`\xE0\x82\x01`\0\x81R`@`\xC0\x86\x01\x827P`\0a\x01 \x83\x01\x90\x81R\x83Q\x90R` \x90\x92\x01Qa\x01@\x90\x91\x01R\x92\x91PPV[`\0` \x82\x84\x03\x12\x15al\x1CW`\0\x80\xFD[\x81Qa:\xBC\x81aa\xE2V[`\0`\xFF\x82\x16`\xFF\x84\x16\x80`\xFF\x03\x82\x11\x15alDWalDae\x97V[\x01\x93\x92PPPV[`\0``\x82\x01`\xFF\x86\x16\x83R` `\x01`\x01``\x1B\x03\x80\x87\x16\x82\x86\x01R`@``\x81\x87\x01R\x83\x87Q\x80\x86R`\x80\x88\x01\x91P\x84\x89\x01\x95P`\0[\x81\x81\x10\x15al\xB5W\x86Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x86\x01Q\x85\x16\x86\x84\x01R\x95\x85\x01\x95\x91\x83\x01\x91`\x01\x01al\x85V[P\x90\x9A\x99PPPPPPPPPPV[`\0a\xFF\xFF\x80\x83\x16\x81\x81\x14\x15al\xDDWal\xDDae\x97V[`\x01\x01\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15al\xF9W`\0\x80\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a:\xBCW`\0\x80\xFD[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15am7Wam7ae\x97V[\x02\x94\x93PPPPV[`\0`\x01`\x01``\x1B\x03\x80\x84\x16\x80amZWamZak\x0BV[\x92\x16\x91\x90\x91\x04\x92\x91PPV\xFERegistryCoordinator.updateOperatRegistryCoordinator._validateChu0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDGRegistryCoordinator._deregisterO\xA2dipfsX\"\x12 \xCF\xE7\xB4$(.\x97\x8Dq\x0Fk\xE0g\xAA\xE6\xE9\xF2S~,\xA7YM\x9B\x04\xBB\x14JW\x93:ldsolcC\0\x08\x0C\x003";
    /// The deployed bytecode of the contract.
    pub static REGISTRYCOORDINATORHARNESS_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct RegistryCoordinatorHarness<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for RegistryCoordinatorHarness<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for RegistryCoordinatorHarness<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for RegistryCoordinatorHarness<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for RegistryCoordinatorHarness<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(RegistryCoordinatorHarness))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> RegistryCoordinatorHarness<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    REGISTRYCOORDINATORHARNESS_ABI.clone(),
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
                REGISTRYCOORDINATORHARNESS_ABI.clone(),
                REGISTRYCOORDINATORHARNESS_BYTECODE.clone().into(),
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
        ///Calls the contract's `OPERATOR_CHURN_APPROVAL_TYPEHASH` (0xca0de882) function
        pub fn operator_churn_approval_typehash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([202, 13, 232, 130], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PUBKEY_REGISTRATION_TYPEHASH` (0x9feab859) function
        pub fn pubkey_registration_typehash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([159, 234, 184, 89], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_deregisterOperatorExternal` (0x8310fef6) function
        pub fn deregister_operator_external(
            &self,
            operator: ::ethers::core::types::Address,
            quorum_numbers: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([131, 16, 254, 246], (operator, quorum_numbers))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_registerOperatorExternal` (0x1ab2574f) function
        pub fn register_operator_external(
            &self,
            operator: ::ethers::core::types::Address,
            operator_id: [u8; 32],
            quorum_numbers: ::ethers::core::types::Bytes,
            socket: ::std::string::String,
            operator_signature: SignatureWithSaltAndExpiry,
        ) -> ::ethers::contract::builders::ContractCall<M, RegisterResults> {
            self.0
                .method_hash(
                    [26, 178, 87, 79],
                    (operator, operator_id, quorum_numbers, socket, operator_signature),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_updateOperatorBitmapExternal` (0x27e79288) function
        pub fn update_operator_bitmap_external(
            &self,
            operator_id: [u8; 32],
            quorum_bitmap: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([39, 231, 146, 136], (operator_id, quorum_bitmap))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_updateOperatorExternal` (0x2953547c) function
        pub fn update_operator_external(
            &self,
            operator: ::ethers::core::types::Address,
            operator_info: OperatorInfo,
            quorums_to_update: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [41, 83, 84, 124],
                    (operator, operator_info, quorums_to_update),
                )
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
        ///Calls the contract's `calculateOperatorChurnApprovalDigestHash` (0x84ca5213) function
        pub fn calculate_operator_churn_approval_digest_hash(
            &self,
            registering_operator: ::ethers::core::types::Address,
            registering_operator_id: [u8; 32],
            operator_kick_params: ::std::vec::Vec<OperatorKickParam>,
            salt: [u8; 32],
            expiry: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [132, 202, 82, 19],
                    (
                        registering_operator,
                        registering_operator_id,
                        operator_kick_params,
                        salt,
                        expiry,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `churnApprover` (0x054310e6) function
        pub fn churn_approver(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([5, 67, 16, 230], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createQuorum` (0xd75b4c88) function
        pub fn create_quorum(
            &self,
            operator_set_params: OperatorSetParam,
            minimum_stake: u128,
            strategy_params: ::std::vec::Vec<StrategyParams>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [215, 91, 76, 136],
                    (operator_set_params, minimum_stake, strategy_params),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deregisterOperator` (0xca4f2d97) function
        pub fn deregister_operator(
            &self,
            quorum_numbers: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([202, 79, 45, 151], quorum_numbers)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ejectOperator` (0x6e3b17db) function
        pub fn eject_operator(
            &self,
            operator: ::ethers::core::types::Address,
            quorum_numbers: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([110, 59, 23, 219], (operator, quorum_numbers))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ejector` (0x28f61b31) function
        pub fn ejector(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([40, 246, 27, 49], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `excludeArtifacts` (0xb5508aa9) function
        pub fn exclude_artifacts(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::std::string::String>,
        > {
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
        ///Calls the contract's `getCurrentQuorumBitmap` (0x871ef049) function
        pub fn get_current_quorum_bitmap(
            &self,
            operator_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([135, 30, 240, 73], operator_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOperator` (0x5865c60c) function
        pub fn get_operator(
            &self,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, OperatorInfo> {
            self.0
                .method_hash([88, 101, 198, 12], operator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOperatorFromId` (0x296bb064) function
        pub fn get_operator_from_id(
            &self,
            operator_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([41, 107, 176, 100], operator_id)
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
        ///Calls the contract's `getOperatorSetParams` (0xe65797ad) function
        pub fn get_operator_set_params(
            &self,
            quorum_number: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, OperatorSetParam> {
            self.0
                .method_hash([230, 87, 151, 173], quorum_number)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOperatorStatus` (0xfd39105a) function
        pub fn get_operator_status(
            &self,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([253, 57, 16, 90], operator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getQuorumBitmapAtBlockNumberByIndex` (0x04ec6351) function
        pub fn get_quorum_bitmap_at_block_number_by_index(
            &self,
            operator_id: [u8; 32],
            block_number: u32,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([4, 236, 99, 81], (operator_id, block_number, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getQuorumBitmapHistoryLength` (0x03fd3492) function
        pub fn get_quorum_bitmap_history_length(
            &self,
            operator_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([3, 253, 52, 146], operator_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getQuorumBitmapIndicesAtBlockNumber` (0xc391425e) function
        pub fn get_quorum_bitmap_indices_at_block_number(
            &self,
            block_number: u32,
            operator_ids: ::std::vec::Vec<[u8; 32]>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<u32>> {
            self.0
                .method_hash([195, 145, 66, 94], (block_number, operator_ids))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getQuorumBitmapUpdateByIndex` (0x1eb812da) function
        pub fn get_quorum_bitmap_update_by_index(
            &self,
            operator_id: [u8; 32],
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, QuorumBitmapUpdate> {
            self.0
                .method_hash([30, 184, 18, 218], (operator_id, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `indexRegistry` (0x9e9923c2) function
        pub fn index_registry(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([158, 153, 35, 194], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0xdd8283f3) function
        pub fn initialize(
            &self,
            initial_owner: ::ethers::core::types::Address,
            churn_approver: ::ethers::core::types::Address,
            ejector: ::ethers::core::types::Address,
            pauser_registry: ::ethers::core::types::Address,
            initial_paused_status: ::ethers::core::types::U256,
            operator_set_params: ::std::vec::Vec<OperatorSetParam>,
            minimum_stakes: ::std::vec::Vec<u128>,
            strategy_params: ::std::vec::Vec<::std::vec::Vec<StrategyParams>>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [221, 130, 131, 243],
                    (
                        initial_owner,
                        churn_approver,
                        ejector,
                        pauser_registry,
                        initial_paused_status,
                        operator_set_params,
                        minimum_stakes,
                        strategy_params,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isChurnApproverSaltUsed` (0x1478851f) function
        pub fn is_churn_approver_salt_used(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([20, 120, 133, 31], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `numRegistries` (0xd72d8dd6) function
        pub fn num_registries(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([215, 45, 141, 214], ())
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
        ///Calls the contract's `pubkeyRegistrationMessageHash` (0x3c2a7f4c) function
        pub fn pubkey_registration_message_hash(
            &self,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, G1Point> {
            self.0
                .method_hash([60, 42, 127, 76], operator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quorumCount` (0x9aa1653d) function
        pub fn quorum_count(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([154, 161, 101, 61], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quorumUpdateBlockNumber` (0x249a0c42) function
        pub fn quorum_update_block_number(
            &self,
            p0: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([36, 154, 12, 66], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerOperator` (0xa50857bf) function
        pub fn register_operator(
            &self,
            quorum_numbers: ::ethers::core::types::Bytes,
            socket: ::std::string::String,
            params: PubkeyRegistrationParams,
            operator_signature: SignatureWithSaltAndExpiry,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [165, 8, 87, 191],
                    (quorum_numbers, socket, params, operator_signature),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerOperatorWithChurn` (0x9b5d177b) function
        pub fn register_operator_with_churn(
            &self,
            quorum_numbers: ::ethers::core::types::Bytes,
            socket: ::std::string::String,
            params: PubkeyRegistrationParams,
            operator_kick_params: ::std::vec::Vec<OperatorKickParam>,
            churn_approver_signature: SignatureWithSaltAndExpiry,
            operator_signature: SignatureWithSaltAndExpiry,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [155, 93, 23, 123],
                    (
                        quorum_numbers,
                        socket,
                        params,
                        operator_kick_params,
                        churn_approver_signature,
                        operator_signature,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registries` (0x6347c900) function
        pub fn registries(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([99, 71, 201, 0], p0)
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
        ///Calls the contract's `serviceManager` (0x3998fdd3) function
        pub fn service_manager(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([57, 152, 253, 211], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setChurnApprover` (0x29d1e0c3) function
        pub fn set_churn_approver(
            &self,
            churn_approver: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([41, 209, 224, 195], churn_approver)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setEjector` (0x2cdd1e86) function
        pub fn set_ejector(
            &self,
            ejector: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([44, 221, 30, 134], ejector)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setOperatorId` (0xd92cbb84) function
        pub fn set_operator_id(
            &self,
            operator: ::ethers::core::types::Address,
            operator_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([217, 44, 187, 132], (operator, operator_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setOperatorSetParams` (0x5b0b829f) function
        pub fn set_operator_set_params(
            &self,
            quorum_number: u8,
            operator_set_params: OperatorSetParam,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([91, 11, 130, 159], (quorum_number, operator_set_params))
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
        ///Calls the contract's `setQuorumCount` (0xc4097d5e) function
        pub fn set_quorum_count(
            &self,
            count: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 9, 125, 94], count)
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
        ///Calls the contract's `targetArtifactSelectors` (0x66d9a9a0) function
        pub fn target_artifact_selectors(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<FuzzSelector>,
        > {
            self.0
                .method_hash([102, 217, 169, 160], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `targetArtifacts` (0x85226c81) function
        pub fn target_artifacts(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::std::string::String>,
        > {
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
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<FuzzInterface>,
        > {
            self.0
                .method_hash([42, 222, 56, 128], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `targetSelectors` (0x916a17c6) function
        pub fn target_selectors(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<FuzzSelector>,
        > {
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
        ///Calls the contract's `updateOperators` (0x00cf2ab5) function
        pub fn update_operators(
            &self,
            operators: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([0, 207, 42, 181], operators)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateOperatorsForQuorum` (0x5140a548) function
        pub fn update_operators_for_quorum(
            &self,
            operators_per_quorum: ::std::vec::Vec<
                ::std::vec::Vec<::ethers::core::types::Address>,
            >,
            quorum_numbers: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([81, 64, 165, 72], (operators_per_quorum, quorum_numbers))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateSocket` (0x0cf4b767) function
        pub fn update_socket(
            &self,
            socket: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([12, 244, 183, 103], socket)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `ChurnApproverUpdated` event
        pub fn churn_approver_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ChurnApproverUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `EjectorUpdated` event
        pub fn ejector_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EjectorUpdatedFilter,
        > {
            self.0.event()
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
        ///Gets the contract's `OperatorDeregistered` event
        pub fn operator_deregistered_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OperatorDeregisteredFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OperatorRegistered` event
        pub fn operator_registered_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OperatorRegisteredFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OperatorSetParamsUpdated` event
        pub fn operator_set_params_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OperatorSetParamsUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OperatorSocketUpdate` event
        pub fn operator_socket_update_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OperatorSocketUpdateFilter,
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
        ///Gets the contract's `QuorumBlockNumberUpdated` event
        pub fn quorum_block_number_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            QuorumBlockNumberUpdatedFilter,
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
        ///Gets the contract's `log` event
        pub fn log_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LogFilter> {
            self.0.event()
        }
        ///Gets the contract's `log_address` event
        pub fn log_address_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogAddressFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_array` event
        pub fn log_array_1_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogArray1Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_array` event
        pub fn log_array_2_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogArray2Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_array` event
        pub fn log_array_3_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogArray3Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_bytes` event
        pub fn log_bytes_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogBytesFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_bytes32` event
        pub fn log_bytes_32_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogBytes32Filter,
        > {
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
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedAddressFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_array` event
        pub fn log_named_array_1_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedArray1Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_array` event
        pub fn log_named_array_2_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedArray2Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_array` event
        pub fn log_named_array_3_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedArray3Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_bytes` event
        pub fn log_named_bytes_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedBytesFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_bytes32` event
        pub fn log_named_bytes_32_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedBytes32Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_decimal_int` event
        pub fn log_named_decimal_int_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedDecimalIntFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_decimal_uint` event
        pub fn log_named_decimal_uint_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedDecimalUintFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_int` event
        pub fn log_named_int_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedIntFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_string` event
        pub fn log_named_string_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedStringFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_named_uint` event
        pub fn log_named_uint_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogNamedUintFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `log_string` event
        pub fn log_string_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LogStringFilter,
        > {
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
            RegistryCoordinatorHarnessEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for RegistryCoordinatorHarness<M> {
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
    #[ethevent(
        name = "ChurnApproverUpdated",
        abi = "ChurnApproverUpdated(address,address)"
    )]
    pub struct ChurnApproverUpdatedFilter {
        pub prev_churn_approver: ::ethers::core::types::Address,
        pub new_churn_approver: ::ethers::core::types::Address,
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
    #[ethevent(name = "EjectorUpdated", abi = "EjectorUpdated(address,address)")]
    pub struct EjectorUpdatedFilter {
        pub prev_ejector: ::ethers::core::types::Address,
        pub new_ejector: ::ethers::core::types::Address,
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
        name = "OperatorDeregistered",
        abi = "OperatorDeregistered(address,bytes32)"
    )]
    pub struct OperatorDeregisteredFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub operator_id: [u8; 32],
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
    #[ethevent(name = "OperatorRegistered", abi = "OperatorRegistered(address,bytes32)")]
    pub struct OperatorRegisteredFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub operator_id: [u8; 32],
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
        name = "OperatorSetParamsUpdated",
        abi = "OperatorSetParamsUpdated(uint8,(uint32,uint16,uint16))"
    )]
    pub struct OperatorSetParamsUpdatedFilter {
        #[ethevent(indexed)]
        pub quorum_number: u8,
        pub operator_set_params: OperatorSetParam,
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
        name = "OperatorSocketUpdate",
        abi = "OperatorSocketUpdate(bytes32,string)"
    )]
    pub struct OperatorSocketUpdateFilter {
        #[ethevent(indexed)]
        pub operator_id: [u8; 32],
        pub socket: ::std::string::String,
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
        name = "QuorumBlockNumberUpdated",
        abi = "QuorumBlockNumberUpdated(uint8,uint256)"
    )]
    pub struct QuorumBlockNumberUpdatedFilter {
        #[ethevent(indexed)]
        pub quorum_number: u8,
        pub blocknumber: ::ethers::core::types::U256,
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
    )]
    pub enum RegistryCoordinatorHarnessEvents {
        ChurnApproverUpdatedFilter(ChurnApproverUpdatedFilter),
        EjectorUpdatedFilter(EjectorUpdatedFilter),
        InitializedFilter(InitializedFilter),
        OperatorDeregisteredFilter(OperatorDeregisteredFilter),
        OperatorRegisteredFilter(OperatorRegisteredFilter),
        OperatorSetParamsUpdatedFilter(OperatorSetParamsUpdatedFilter),
        OperatorSocketUpdateFilter(OperatorSocketUpdateFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PausedFilter(PausedFilter),
        PauserRegistrySetFilter(PauserRegistrySetFilter),
        QuorumBlockNumberUpdatedFilter(QuorumBlockNumberUpdatedFilter),
        UnpausedFilter(UnpausedFilter),
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
    impl ::ethers::contract::EthLogDecode for RegistryCoordinatorHarnessEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ChurnApproverUpdatedFilter::decode_log(log) {
                return Ok(
                    RegistryCoordinatorHarnessEvents::ChurnApproverUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = EjectorUpdatedFilter::decode_log(log) {
                return Ok(
                    RegistryCoordinatorHarnessEvents::EjectorUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(RegistryCoordinatorHarnessEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OperatorDeregisteredFilter::decode_log(log) {
                return Ok(
                    RegistryCoordinatorHarnessEvents::OperatorDeregisteredFilter(decoded),
                );
            }
            if let Ok(decoded) = OperatorRegisteredFilter::decode_log(log) {
                return Ok(
                    RegistryCoordinatorHarnessEvents::OperatorRegisteredFilter(decoded),
                );
            }
            if let Ok(decoded) = OperatorSetParamsUpdatedFilter::decode_log(log) {
                return Ok(
                    RegistryCoordinatorHarnessEvents::OperatorSetParamsUpdatedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = OperatorSocketUpdateFilter::decode_log(log) {
                return Ok(
                    RegistryCoordinatorHarnessEvents::OperatorSocketUpdateFilter(decoded),
                );
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(
                    RegistryCoordinatorHarnessEvents::OwnershipTransferredFilter(decoded),
                );
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(RegistryCoordinatorHarnessEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = PauserRegistrySetFilter::decode_log(log) {
                return Ok(
                    RegistryCoordinatorHarnessEvents::PauserRegistrySetFilter(decoded),
                );
            }
            if let Ok(decoded) = QuorumBlockNumberUpdatedFilter::decode_log(log) {
                return Ok(
                    RegistryCoordinatorHarnessEvents::QuorumBlockNumberUpdatedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(RegistryCoordinatorHarnessEvents::UnpausedFilter(decoded));
            }
            if let Ok(decoded) = LogFilter::decode_log(log) {
                return Ok(RegistryCoordinatorHarnessEvents::LogFilter(decoded));
            }
            if let Ok(decoded) = LogAddressFilter::decode_log(log) {
                return Ok(RegistryCoordinatorHarnessEvents::LogAddressFilter(decoded));
            }
            if let Ok(decoded) = LogArray1Filter::decode_log(log) {
                return Ok(RegistryCoordinatorHarnessEvents::LogArray1Filter(decoded));
            }
            if let Ok(decoded) = LogArray2Filter::decode_log(log) {
                return Ok(RegistryCoordinatorHarnessEvents::LogArray2Filter(decoded));
            }
            if let Ok(decoded) = LogArray3Filter::decode_log(log) {
                return Ok(RegistryCoordinatorHarnessEvents::LogArray3Filter(decoded));
            }
            if let Ok(decoded) = LogBytesFilter::decode_log(log) {
                return Ok(RegistryCoordinatorHarnessEvents::LogBytesFilter(decoded));
            }
            if let Ok(decoded) = LogBytes32Filter::decode_log(log) {
                return Ok(RegistryCoordinatorHarnessEvents::LogBytes32Filter(decoded));
            }
            if let Ok(decoded) = LogIntFilter::decode_log(log) {
                return Ok(RegistryCoordinatorHarnessEvents::LogIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedAddressFilter::decode_log(log) {
                return Ok(
                    RegistryCoordinatorHarnessEvents::LogNamedAddressFilter(decoded),
                );
            }
            if let Ok(decoded) = LogNamedArray1Filter::decode_log(log) {
                return Ok(
                    RegistryCoordinatorHarnessEvents::LogNamedArray1Filter(decoded),
                );
            }
            if let Ok(decoded) = LogNamedArray2Filter::decode_log(log) {
                return Ok(
                    RegistryCoordinatorHarnessEvents::LogNamedArray2Filter(decoded),
                );
            }
            if let Ok(decoded) = LogNamedArray3Filter::decode_log(log) {
                return Ok(
                    RegistryCoordinatorHarnessEvents::LogNamedArray3Filter(decoded),
                );
            }
            if let Ok(decoded) = LogNamedBytesFilter::decode_log(log) {
                return Ok(
                    RegistryCoordinatorHarnessEvents::LogNamedBytesFilter(decoded),
                );
            }
            if let Ok(decoded) = LogNamedBytes32Filter::decode_log(log) {
                return Ok(
                    RegistryCoordinatorHarnessEvents::LogNamedBytes32Filter(decoded),
                );
            }
            if let Ok(decoded) = LogNamedDecimalIntFilter::decode_log(log) {
                return Ok(
                    RegistryCoordinatorHarnessEvents::LogNamedDecimalIntFilter(decoded),
                );
            }
            if let Ok(decoded) = LogNamedDecimalUintFilter::decode_log(log) {
                return Ok(
                    RegistryCoordinatorHarnessEvents::LogNamedDecimalUintFilter(decoded),
                );
            }
            if let Ok(decoded) = LogNamedIntFilter::decode_log(log) {
                return Ok(RegistryCoordinatorHarnessEvents::LogNamedIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedStringFilter::decode_log(log) {
                return Ok(
                    RegistryCoordinatorHarnessEvents::LogNamedStringFilter(decoded),
                );
            }
            if let Ok(decoded) = LogNamedUintFilter::decode_log(log) {
                return Ok(RegistryCoordinatorHarnessEvents::LogNamedUintFilter(decoded));
            }
            if let Ok(decoded) = LogStringFilter::decode_log(log) {
                return Ok(RegistryCoordinatorHarnessEvents::LogStringFilter(decoded));
            }
            if let Ok(decoded) = LogUintFilter::decode_log(log) {
                return Ok(RegistryCoordinatorHarnessEvents::LogUintFilter(decoded));
            }
            if let Ok(decoded) = LogsFilter::decode_log(log) {
                return Ok(RegistryCoordinatorHarnessEvents::LogsFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for RegistryCoordinatorHarnessEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ChurnApproverUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EjectorUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorDeregisteredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorRegisteredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorSetParamsUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorSocketUpdateFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PausedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauserRegistrySetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::QuorumBlockNumberUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnpausedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogAddressFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogArray1Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogArray2Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogArray3Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytesFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes32Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogIntFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogNamedAddressFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedArray1Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedArray2Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedArray3Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedBytesFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedBytes32Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedDecimalIntFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedDecimalUintFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedIntFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogNamedStringFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogNamedUintFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LogStringFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogUintFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogsFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ChurnApproverUpdatedFilter>
    for RegistryCoordinatorHarnessEvents {
        fn from(value: ChurnApproverUpdatedFilter) -> Self {
            Self::ChurnApproverUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<EjectorUpdatedFilter>
    for RegistryCoordinatorHarnessEvents {
        fn from(value: EjectorUpdatedFilter) -> Self {
            Self::EjectorUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for RegistryCoordinatorHarnessEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OperatorDeregisteredFilter>
    for RegistryCoordinatorHarnessEvents {
        fn from(value: OperatorDeregisteredFilter) -> Self {
            Self::OperatorDeregisteredFilter(value)
        }
    }
    impl ::core::convert::From<OperatorRegisteredFilter>
    for RegistryCoordinatorHarnessEvents {
        fn from(value: OperatorRegisteredFilter) -> Self {
            Self::OperatorRegisteredFilter(value)
        }
    }
    impl ::core::convert::From<OperatorSetParamsUpdatedFilter>
    for RegistryCoordinatorHarnessEvents {
        fn from(value: OperatorSetParamsUpdatedFilter) -> Self {
            Self::OperatorSetParamsUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<OperatorSocketUpdateFilter>
    for RegistryCoordinatorHarnessEvents {
        fn from(value: OperatorSocketUpdateFilter) -> Self {
            Self::OperatorSocketUpdateFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter>
    for RegistryCoordinatorHarnessEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<PausedFilter> for RegistryCoordinatorHarnessEvents {
        fn from(value: PausedFilter) -> Self {
            Self::PausedFilter(value)
        }
    }
    impl ::core::convert::From<PauserRegistrySetFilter>
    for RegistryCoordinatorHarnessEvents {
        fn from(value: PauserRegistrySetFilter) -> Self {
            Self::PauserRegistrySetFilter(value)
        }
    }
    impl ::core::convert::From<QuorumBlockNumberUpdatedFilter>
    for RegistryCoordinatorHarnessEvents {
        fn from(value: QuorumBlockNumberUpdatedFilter) -> Self {
            Self::QuorumBlockNumberUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<UnpausedFilter> for RegistryCoordinatorHarnessEvents {
        fn from(value: UnpausedFilter) -> Self {
            Self::UnpausedFilter(value)
        }
    }
    impl ::core::convert::From<LogFilter> for RegistryCoordinatorHarnessEvents {
        fn from(value: LogFilter) -> Self {
            Self::LogFilter(value)
        }
    }
    impl ::core::convert::From<LogAddressFilter> for RegistryCoordinatorHarnessEvents {
        fn from(value: LogAddressFilter) -> Self {
            Self::LogAddressFilter(value)
        }
    }
    impl ::core::convert::From<LogArray1Filter> for RegistryCoordinatorHarnessEvents {
        fn from(value: LogArray1Filter) -> Self {
            Self::LogArray1Filter(value)
        }
    }
    impl ::core::convert::From<LogArray2Filter> for RegistryCoordinatorHarnessEvents {
        fn from(value: LogArray2Filter) -> Self {
            Self::LogArray2Filter(value)
        }
    }
    impl ::core::convert::From<LogArray3Filter> for RegistryCoordinatorHarnessEvents {
        fn from(value: LogArray3Filter) -> Self {
            Self::LogArray3Filter(value)
        }
    }
    impl ::core::convert::From<LogBytesFilter> for RegistryCoordinatorHarnessEvents {
        fn from(value: LogBytesFilter) -> Self {
            Self::LogBytesFilter(value)
        }
    }
    impl ::core::convert::From<LogBytes32Filter> for RegistryCoordinatorHarnessEvents {
        fn from(value: LogBytes32Filter) -> Self {
            Self::LogBytes32Filter(value)
        }
    }
    impl ::core::convert::From<LogIntFilter> for RegistryCoordinatorHarnessEvents {
        fn from(value: LogIntFilter) -> Self {
            Self::LogIntFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedAddressFilter>
    for RegistryCoordinatorHarnessEvents {
        fn from(value: LogNamedAddressFilter) -> Self {
            Self::LogNamedAddressFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedArray1Filter>
    for RegistryCoordinatorHarnessEvents {
        fn from(value: LogNamedArray1Filter) -> Self {
            Self::LogNamedArray1Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedArray2Filter>
    for RegistryCoordinatorHarnessEvents {
        fn from(value: LogNamedArray2Filter) -> Self {
            Self::LogNamedArray2Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedArray3Filter>
    for RegistryCoordinatorHarnessEvents {
        fn from(value: LogNamedArray3Filter) -> Self {
            Self::LogNamedArray3Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedBytesFilter>
    for RegistryCoordinatorHarnessEvents {
        fn from(value: LogNamedBytesFilter) -> Self {
            Self::LogNamedBytesFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedBytes32Filter>
    for RegistryCoordinatorHarnessEvents {
        fn from(value: LogNamedBytes32Filter) -> Self {
            Self::LogNamedBytes32Filter(value)
        }
    }
    impl ::core::convert::From<LogNamedDecimalIntFilter>
    for RegistryCoordinatorHarnessEvents {
        fn from(value: LogNamedDecimalIntFilter) -> Self {
            Self::LogNamedDecimalIntFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedDecimalUintFilter>
    for RegistryCoordinatorHarnessEvents {
        fn from(value: LogNamedDecimalUintFilter) -> Self {
            Self::LogNamedDecimalUintFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedIntFilter> for RegistryCoordinatorHarnessEvents {
        fn from(value: LogNamedIntFilter) -> Self {
            Self::LogNamedIntFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedStringFilter>
    for RegistryCoordinatorHarnessEvents {
        fn from(value: LogNamedStringFilter) -> Self {
            Self::LogNamedStringFilter(value)
        }
    }
    impl ::core::convert::From<LogNamedUintFilter> for RegistryCoordinatorHarnessEvents {
        fn from(value: LogNamedUintFilter) -> Self {
            Self::LogNamedUintFilter(value)
        }
    }
    impl ::core::convert::From<LogStringFilter> for RegistryCoordinatorHarnessEvents {
        fn from(value: LogStringFilter) -> Self {
            Self::LogStringFilter(value)
        }
    }
    impl ::core::convert::From<LogUintFilter> for RegistryCoordinatorHarnessEvents {
        fn from(value: LogUintFilter) -> Self {
            Self::LogUintFilter(value)
        }
    }
    impl ::core::convert::From<LogsFilter> for RegistryCoordinatorHarnessEvents {
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
        Hash
    )]
    #[ethcall(name = "IS_TEST", abi = "IS_TEST()")]
    pub struct IsTestCall;
    ///Container type for all input parameters for the `OPERATOR_CHURN_APPROVAL_TYPEHASH` function with signature `OPERATOR_CHURN_APPROVAL_TYPEHASH()` and selector `0xca0de882`
    #[derive(
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
        name = "OPERATOR_CHURN_APPROVAL_TYPEHASH",
        abi = "OPERATOR_CHURN_APPROVAL_TYPEHASH()"
    )]
    pub struct OperatorChurnApprovalTypehashCall;
    ///Container type for all input parameters for the `PUBKEY_REGISTRATION_TYPEHASH` function with signature `PUBKEY_REGISTRATION_TYPEHASH()` and selector `0x9feab859`
    #[derive(
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
        name = "PUBKEY_REGISTRATION_TYPEHASH",
        abi = "PUBKEY_REGISTRATION_TYPEHASH()"
    )]
    pub struct PubkeyRegistrationTypehashCall;
    ///Container type for all input parameters for the `_deregisterOperatorExternal` function with signature `_deregisterOperatorExternal(address,bytes)` and selector `0x8310fef6`
    #[derive(
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
        name = "_deregisterOperatorExternal",
        abi = "_deregisterOperatorExternal(address,bytes)"
    )]
    pub struct DeregisterOperatorExternalCall {
        pub operator: ::ethers::core::types::Address,
        pub quorum_numbers: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `_registerOperatorExternal` function with signature `_registerOperatorExternal(address,bytes32,bytes,string,(bytes,bytes32,uint256))` and selector `0x1ab2574f`
    #[derive(
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
        name = "_registerOperatorExternal",
        abi = "_registerOperatorExternal(address,bytes32,bytes,string,(bytes,bytes32,uint256))"
    )]
    pub struct RegisterOperatorExternalCall {
        pub operator: ::ethers::core::types::Address,
        pub operator_id: [u8; 32],
        pub quorum_numbers: ::ethers::core::types::Bytes,
        pub socket: ::std::string::String,
        pub operator_signature: SignatureWithSaltAndExpiry,
    }
    ///Container type for all input parameters for the `_updateOperatorBitmapExternal` function with signature `_updateOperatorBitmapExternal(bytes32,uint192)` and selector `0x27e79288`
    #[derive(
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
        name = "_updateOperatorBitmapExternal",
        abi = "_updateOperatorBitmapExternal(bytes32,uint192)"
    )]
    pub struct UpdateOperatorBitmapExternalCall {
        pub operator_id: [u8; 32],
        pub quorum_bitmap: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `_updateOperatorExternal` function with signature `_updateOperatorExternal(address,(bytes32,uint8),bytes)` and selector `0x2953547c`
    #[derive(
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
        name = "_updateOperatorExternal",
        abi = "_updateOperatorExternal(address,(bytes32,uint8),bytes)"
    )]
    pub struct UpdateOperatorExternalCall {
        pub operator: ::ethers::core::types::Address,
        pub operator_info: OperatorInfo,
        pub quorums_to_update: ::ethers::core::types::Bytes,
    }
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
    ///Container type for all input parameters for the `calculateOperatorChurnApprovalDigestHash` function with signature `calculateOperatorChurnApprovalDigestHash(address,bytes32,(uint8,address)[],bytes32,uint256)` and selector `0x84ca5213`
    #[derive(
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
        name = "calculateOperatorChurnApprovalDigestHash",
        abi = "calculateOperatorChurnApprovalDigestHash(address,bytes32,(uint8,address)[],bytes32,uint256)"
    )]
    pub struct CalculateOperatorChurnApprovalDigestHashCall {
        pub registering_operator: ::ethers::core::types::Address,
        pub registering_operator_id: [u8; 32],
        pub operator_kick_params: ::std::vec::Vec<OperatorKickParam>,
        pub salt: [u8; 32],
        pub expiry: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `churnApprover` function with signature `churnApprover()` and selector `0x054310e6`
    #[derive(
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
    #[ethcall(name = "churnApprover", abi = "churnApprover()")]
    pub struct ChurnApproverCall;
    ///Container type for all input parameters for the `createQuorum` function with signature `createQuorum((uint32,uint16,uint16),uint96,(address,uint96)[])` and selector `0xd75b4c88`
    #[derive(
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
        name = "createQuorum",
        abi = "createQuorum((uint32,uint16,uint16),uint96,(address,uint96)[])"
    )]
    pub struct CreateQuorumCall {
        pub operator_set_params: OperatorSetParam,
        pub minimum_stake: u128,
        pub strategy_params: ::std::vec::Vec<StrategyParams>,
    }
    ///Container type for all input parameters for the `deregisterOperator` function with signature `deregisterOperator(bytes)` and selector `0xca4f2d97`
    #[derive(
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
    #[ethcall(name = "deregisterOperator", abi = "deregisterOperator(bytes)")]
    pub struct DeregisterOperatorCall {
        pub quorum_numbers: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `ejectOperator` function with signature `ejectOperator(address,bytes)` and selector `0x6e3b17db`
    #[derive(
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
    #[ethcall(name = "ejectOperator", abi = "ejectOperator(address,bytes)")]
    pub struct EjectOperatorCall {
        pub operator: ::ethers::core::types::Address,
        pub quorum_numbers: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `ejector` function with signature `ejector()` and selector `0x28f61b31`
    #[derive(
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
    #[ethcall(name = "ejector", abi = "ejector()")]
    pub struct EjectorCall;
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
        Hash
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
        Hash
    )]
    #[ethcall(name = "excludeContracts", abi = "excludeContracts()")]
    pub struct ExcludeContractsCall;
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
        Hash
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
        Hash
    )]
    #[ethcall(name = "failed", abi = "failed()")]
    pub struct FailedCall;
    ///Container type for all input parameters for the `getCurrentQuorumBitmap` function with signature `getCurrentQuorumBitmap(bytes32)` and selector `0x871ef049`
    #[derive(
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
    #[ethcall(name = "getCurrentQuorumBitmap", abi = "getCurrentQuorumBitmap(bytes32)")]
    pub struct GetCurrentQuorumBitmapCall {
        pub operator_id: [u8; 32],
    }
    ///Container type for all input parameters for the `getOperator` function with signature `getOperator(address)` and selector `0x5865c60c`
    #[derive(
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
    #[ethcall(name = "getOperator", abi = "getOperator(address)")]
    pub struct GetOperatorCall {
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getOperatorFromId` function with signature `getOperatorFromId(bytes32)` and selector `0x296bb064`
    #[derive(
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
    #[ethcall(name = "getOperatorFromId", abi = "getOperatorFromId(bytes32)")]
    pub struct GetOperatorFromIdCall {
        pub operator_id: [u8; 32],
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
        Hash
    )]
    #[ethcall(name = "getOperatorId", abi = "getOperatorId(address)")]
    pub struct GetOperatorIdCall {
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getOperatorSetParams` function with signature `getOperatorSetParams(uint8)` and selector `0xe65797ad`
    #[derive(
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
    #[ethcall(name = "getOperatorSetParams", abi = "getOperatorSetParams(uint8)")]
    pub struct GetOperatorSetParamsCall {
        pub quorum_number: u8,
    }
    ///Container type for all input parameters for the `getOperatorStatus` function with signature `getOperatorStatus(address)` and selector `0xfd39105a`
    #[derive(
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
    #[ethcall(name = "getOperatorStatus", abi = "getOperatorStatus(address)")]
    pub struct GetOperatorStatusCall {
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getQuorumBitmapAtBlockNumberByIndex` function with signature `getQuorumBitmapAtBlockNumberByIndex(bytes32,uint32,uint256)` and selector `0x04ec6351`
    #[derive(
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
        name = "getQuorumBitmapAtBlockNumberByIndex",
        abi = "getQuorumBitmapAtBlockNumberByIndex(bytes32,uint32,uint256)"
    )]
    pub struct GetQuorumBitmapAtBlockNumberByIndexCall {
        pub operator_id: [u8; 32],
        pub block_number: u32,
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getQuorumBitmapHistoryLength` function with signature `getQuorumBitmapHistoryLength(bytes32)` and selector `0x03fd3492`
    #[derive(
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
        name = "getQuorumBitmapHistoryLength",
        abi = "getQuorumBitmapHistoryLength(bytes32)"
    )]
    pub struct GetQuorumBitmapHistoryLengthCall {
        pub operator_id: [u8; 32],
    }
    ///Container type for all input parameters for the `getQuorumBitmapIndicesAtBlockNumber` function with signature `getQuorumBitmapIndicesAtBlockNumber(uint32,bytes32[])` and selector `0xc391425e`
    #[derive(
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
        name = "getQuorumBitmapIndicesAtBlockNumber",
        abi = "getQuorumBitmapIndicesAtBlockNumber(uint32,bytes32[])"
    )]
    pub struct GetQuorumBitmapIndicesAtBlockNumberCall {
        pub block_number: u32,
        pub operator_ids: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all input parameters for the `getQuorumBitmapUpdateByIndex` function with signature `getQuorumBitmapUpdateByIndex(bytes32,uint256)` and selector `0x1eb812da`
    #[derive(
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
        name = "getQuorumBitmapUpdateByIndex",
        abi = "getQuorumBitmapUpdateByIndex(bytes32,uint256)"
    )]
    pub struct GetQuorumBitmapUpdateByIndexCall {
        pub operator_id: [u8; 32],
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `indexRegistry` function with signature `indexRegistry()` and selector `0x9e9923c2`
    #[derive(
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
    #[ethcall(name = "indexRegistry", abi = "indexRegistry()")]
    pub struct IndexRegistryCall;
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address,address,address,uint256,(uint32,uint16,uint16)[],uint96[],(address,uint96)[][])` and selector `0xdd8283f3`
    #[derive(
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
        name = "initialize",
        abi = "initialize(address,address,address,address,uint256,(uint32,uint16,uint16)[],uint96[],(address,uint96)[][])"
    )]
    pub struct InitializeCall {
        pub initial_owner: ::ethers::core::types::Address,
        pub churn_approver: ::ethers::core::types::Address,
        pub ejector: ::ethers::core::types::Address,
        pub pauser_registry: ::ethers::core::types::Address,
        pub initial_paused_status: ::ethers::core::types::U256,
        pub operator_set_params: ::std::vec::Vec<OperatorSetParam>,
        pub minimum_stakes: ::std::vec::Vec<u128>,
        pub strategy_params: ::std::vec::Vec<::std::vec::Vec<StrategyParams>>,
    }
    ///Container type for all input parameters for the `isChurnApproverSaltUsed` function with signature `isChurnApproverSaltUsed(bytes32)` and selector `0x1478851f`
    #[derive(
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
        name = "isChurnApproverSaltUsed",
        abi = "isChurnApproverSaltUsed(bytes32)"
    )]
    pub struct IsChurnApproverSaltUsedCall(pub [u8; 32]);
    ///Container type for all input parameters for the `numRegistries` function with signature `numRegistries()` and selector `0xd72d8dd6`
    #[derive(
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
    #[ethcall(name = "numRegistries", abi = "numRegistries()")]
    pub struct NumRegistriesCall;
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
    ///Container type for all input parameters for the `pubkeyRegistrationMessageHash` function with signature `pubkeyRegistrationMessageHash(address)` and selector `0x3c2a7f4c`
    #[derive(
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
        name = "pubkeyRegistrationMessageHash",
        abi = "pubkeyRegistrationMessageHash(address)"
    )]
    pub struct PubkeyRegistrationMessageHashCall {
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `quorumCount` function with signature `quorumCount()` and selector `0x9aa1653d`
    #[derive(
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
    #[ethcall(name = "quorumCount", abi = "quorumCount()")]
    pub struct QuorumCountCall;
    ///Container type for all input parameters for the `quorumUpdateBlockNumber` function with signature `quorumUpdateBlockNumber(uint8)` and selector `0x249a0c42`
    #[derive(
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
    #[ethcall(name = "quorumUpdateBlockNumber", abi = "quorumUpdateBlockNumber(uint8)")]
    pub struct QuorumUpdateBlockNumberCall(pub u8);
    ///Container type for all input parameters for the `registerOperator` function with signature `registerOperator(bytes,string,((uint256,uint256),(uint256,uint256),(uint256[2],uint256[2])),(bytes,bytes32,uint256))` and selector `0xa50857bf`
    #[derive(
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
        name = "registerOperator",
        abi = "registerOperator(bytes,string,((uint256,uint256),(uint256,uint256),(uint256[2],uint256[2])),(bytes,bytes32,uint256))"
    )]
    pub struct RegisterOperatorCall {
        pub quorum_numbers: ::ethers::core::types::Bytes,
        pub socket: ::std::string::String,
        pub params: PubkeyRegistrationParams,
        pub operator_signature: SignatureWithSaltAndExpiry,
    }
    ///Container type for all input parameters for the `registerOperatorWithChurn` function with signature `registerOperatorWithChurn(bytes,string,((uint256,uint256),(uint256,uint256),(uint256[2],uint256[2])),(uint8,address)[],(bytes,bytes32,uint256),(bytes,bytes32,uint256))` and selector `0x9b5d177b`
    #[derive(
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
        name = "registerOperatorWithChurn",
        abi = "registerOperatorWithChurn(bytes,string,((uint256,uint256),(uint256,uint256),(uint256[2],uint256[2])),(uint8,address)[],(bytes,bytes32,uint256),(bytes,bytes32,uint256))"
    )]
    pub struct RegisterOperatorWithChurnCall {
        pub quorum_numbers: ::ethers::core::types::Bytes,
        pub socket: ::std::string::String,
        pub params: PubkeyRegistrationParams,
        pub operator_kick_params: ::std::vec::Vec<OperatorKickParam>,
        pub churn_approver_signature: SignatureWithSaltAndExpiry,
        pub operator_signature: SignatureWithSaltAndExpiry,
    }
    ///Container type for all input parameters for the `registries` function with signature `registries(uint256)` and selector `0x6347c900`
    #[derive(
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
    #[ethcall(name = "registries", abi = "registries(uint256)")]
    pub struct RegistriesCall(pub ::ethers::core::types::U256);
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
    ///Container type for all input parameters for the `serviceManager` function with signature `serviceManager()` and selector `0x3998fdd3`
    #[derive(
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
    #[ethcall(name = "serviceManager", abi = "serviceManager()")]
    pub struct ServiceManagerCall;
    ///Container type for all input parameters for the `setChurnApprover` function with signature `setChurnApprover(address)` and selector `0x29d1e0c3`
    #[derive(
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
    #[ethcall(name = "setChurnApprover", abi = "setChurnApprover(address)")]
    pub struct SetChurnApproverCall {
        pub churn_approver: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setEjector` function with signature `setEjector(address)` and selector `0x2cdd1e86`
    #[derive(
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
    #[ethcall(name = "setEjector", abi = "setEjector(address)")]
    pub struct SetEjectorCall {
        pub ejector: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setOperatorId` function with signature `setOperatorId(address,bytes32)` and selector `0xd92cbb84`
    #[derive(
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
    #[ethcall(name = "setOperatorId", abi = "setOperatorId(address,bytes32)")]
    pub struct SetOperatorIdCall {
        pub operator: ::ethers::core::types::Address,
        pub operator_id: [u8; 32],
    }
    ///Container type for all input parameters for the `setOperatorSetParams` function with signature `setOperatorSetParams(uint8,(uint32,uint16,uint16))` and selector `0x5b0b829f`
    #[derive(
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
        name = "setOperatorSetParams",
        abi = "setOperatorSetParams(uint8,(uint32,uint16,uint16))"
    )]
    pub struct SetOperatorSetParamsCall {
        pub quorum_number: u8,
        pub operator_set_params: OperatorSetParam,
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
    ///Container type for all input parameters for the `setQuorumCount` function with signature `setQuorumCount(uint8)` and selector `0xc4097d5e`
    #[derive(
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
    #[ethcall(name = "setQuorumCount", abi = "setQuorumCount(uint8)")]
    pub struct SetQuorumCountCall {
        pub count: u8,
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
    )]
    #[ethcall(name = "targetSenders", abi = "targetSenders()")]
    pub struct TargetSendersCall;
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
    ///Container type for all input parameters for the `updateOperators` function with signature `updateOperators(address[])` and selector `0x00cf2ab5`
    #[derive(
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
    #[ethcall(name = "updateOperators", abi = "updateOperators(address[])")]
    pub struct UpdateOperatorsCall {
        pub operators: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `updateOperatorsForQuorum` function with signature `updateOperatorsForQuorum(address[][],bytes)` and selector `0x5140a548`
    #[derive(
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
        name = "updateOperatorsForQuorum",
        abi = "updateOperatorsForQuorum(address[][],bytes)"
    )]
    pub struct UpdateOperatorsForQuorumCall {
        pub operators_per_quorum: ::std::vec::Vec<
            ::std::vec::Vec<::ethers::core::types::Address>,
        >,
        pub quorum_numbers: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `updateSocket` function with signature `updateSocket(string)` and selector `0x0cf4b767`
    #[derive(
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
    #[ethcall(name = "updateSocket", abi = "updateSocket(string)")]
    pub struct UpdateSocketCall {
        pub socket: ::std::string::String,
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
    pub enum RegistryCoordinatorHarnessCalls {
        IsTest(IsTestCall),
        OperatorChurnApprovalTypehash(OperatorChurnApprovalTypehashCall),
        PubkeyRegistrationTypehash(PubkeyRegistrationTypehashCall),
        DeregisterOperatorExternal(DeregisterOperatorExternalCall),
        RegisterOperatorExternal(RegisterOperatorExternalCall),
        UpdateOperatorBitmapExternal(UpdateOperatorBitmapExternalCall),
        UpdateOperatorExternal(UpdateOperatorExternalCall),
        BlsApkRegistry(BlsApkRegistryCall),
        CalculateOperatorChurnApprovalDigestHash(
            CalculateOperatorChurnApprovalDigestHashCall,
        ),
        ChurnApprover(ChurnApproverCall),
        CreateQuorum(CreateQuorumCall),
        DeregisterOperator(DeregisterOperatorCall),
        EjectOperator(EjectOperatorCall),
        Ejector(EjectorCall),
        ExcludeArtifacts(ExcludeArtifactsCall),
        ExcludeContracts(ExcludeContractsCall),
        ExcludeSenders(ExcludeSendersCall),
        Failed(FailedCall),
        GetCurrentQuorumBitmap(GetCurrentQuorumBitmapCall),
        GetOperator(GetOperatorCall),
        GetOperatorFromId(GetOperatorFromIdCall),
        GetOperatorId(GetOperatorIdCall),
        GetOperatorSetParams(GetOperatorSetParamsCall),
        GetOperatorStatus(GetOperatorStatusCall),
        GetQuorumBitmapAtBlockNumberByIndex(GetQuorumBitmapAtBlockNumberByIndexCall),
        GetQuorumBitmapHistoryLength(GetQuorumBitmapHistoryLengthCall),
        GetQuorumBitmapIndicesAtBlockNumber(GetQuorumBitmapIndicesAtBlockNumberCall),
        GetQuorumBitmapUpdateByIndex(GetQuorumBitmapUpdateByIndexCall),
        IndexRegistry(IndexRegistryCall),
        Initialize(InitializeCall),
        IsChurnApproverSaltUsed(IsChurnApproverSaltUsedCall),
        NumRegistries(NumRegistriesCall),
        Owner(OwnerCall),
        Pause(PauseCall),
        PauseAll(PauseAllCall),
        PausedWithIndex(PausedWithIndexCall),
        Paused(PausedCall),
        PauserRegistry(PauserRegistryCall),
        PubkeyRegistrationMessageHash(PubkeyRegistrationMessageHashCall),
        QuorumCount(QuorumCountCall),
        QuorumUpdateBlockNumber(QuorumUpdateBlockNumberCall),
        RegisterOperator(RegisterOperatorCall),
        RegisterOperatorWithChurn(RegisterOperatorWithChurnCall),
        Registries(RegistriesCall),
        RenounceOwnership(RenounceOwnershipCall),
        ServiceManager(ServiceManagerCall),
        SetChurnApprover(SetChurnApproverCall),
        SetEjector(SetEjectorCall),
        SetOperatorId(SetOperatorIdCall),
        SetOperatorSetParams(SetOperatorSetParamsCall),
        SetPauserRegistry(SetPauserRegistryCall),
        SetQuorumCount(SetQuorumCountCall),
        StakeRegistry(StakeRegistryCall),
        TargetArtifactSelectors(TargetArtifactSelectorsCall),
        TargetArtifacts(TargetArtifactsCall),
        TargetContracts(TargetContractsCall),
        TargetInterfaces(TargetInterfacesCall),
        TargetSelectors(TargetSelectorsCall),
        TargetSenders(TargetSendersCall),
        TransferOwnership(TransferOwnershipCall),
        Unpause(UnpauseCall),
        UpdateOperators(UpdateOperatorsCall),
        UpdateOperatorsForQuorum(UpdateOperatorsForQuorumCall),
        UpdateSocket(UpdateSocketCall),
    }
    impl ::ethers::core::abi::AbiDecode for RegistryCoordinatorHarnessCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <IsTestCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsTest(decoded));
            }
            if let Ok(decoded) = <OperatorChurnApprovalTypehashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OperatorChurnApprovalTypehash(decoded));
            }
            if let Ok(decoded) = <PubkeyRegistrationTypehashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PubkeyRegistrationTypehash(decoded));
            }
            if let Ok(decoded) = <DeregisterOperatorExternalCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeregisterOperatorExternal(decoded));
            }
            if let Ok(decoded) = <RegisterOperatorExternalCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RegisterOperatorExternal(decoded));
            }
            if let Ok(decoded) = <UpdateOperatorBitmapExternalCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateOperatorBitmapExternal(decoded));
            }
            if let Ok(decoded) = <UpdateOperatorExternalCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateOperatorExternal(decoded));
            }
            if let Ok(decoded) = <BlsApkRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BlsApkRegistry(decoded));
            }
            if let Ok(decoded) = <CalculateOperatorChurnApprovalDigestHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateOperatorChurnApprovalDigestHash(decoded));
            }
            if let Ok(decoded) = <ChurnApproverCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ChurnApprover(decoded));
            }
            if let Ok(decoded) = <CreateQuorumCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateQuorum(decoded));
            }
            if let Ok(decoded) = <DeregisterOperatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeregisterOperator(decoded));
            }
            if let Ok(decoded) = <EjectOperatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EjectOperator(decoded));
            }
            if let Ok(decoded) = <EjectorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Ejector(decoded));
            }
            if let Ok(decoded) = <ExcludeArtifactsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExcludeArtifacts(decoded));
            }
            if let Ok(decoded) = <ExcludeContractsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExcludeContracts(decoded));
            }
            if let Ok(decoded) = <ExcludeSendersCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExcludeSenders(decoded));
            }
            if let Ok(decoded) = <FailedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Failed(decoded));
            }
            if let Ok(decoded) = <GetCurrentQuorumBitmapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetCurrentQuorumBitmap(decoded));
            }
            if let Ok(decoded) = <GetOperatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetOperator(decoded));
            }
            if let Ok(decoded) = <GetOperatorFromIdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetOperatorFromId(decoded));
            }
            if let Ok(decoded) = <GetOperatorIdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetOperatorId(decoded));
            }
            if let Ok(decoded) = <GetOperatorSetParamsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetOperatorSetParams(decoded));
            }
            if let Ok(decoded) = <GetOperatorStatusCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetOperatorStatus(decoded));
            }
            if let Ok(decoded) = <GetQuorumBitmapAtBlockNumberByIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetQuorumBitmapAtBlockNumberByIndex(decoded));
            }
            if let Ok(decoded) = <GetQuorumBitmapHistoryLengthCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetQuorumBitmapHistoryLength(decoded));
            }
            if let Ok(decoded) = <GetQuorumBitmapIndicesAtBlockNumberCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetQuorumBitmapIndicesAtBlockNumber(decoded));
            }
            if let Ok(decoded) = <GetQuorumBitmapUpdateByIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetQuorumBitmapUpdateByIndex(decoded));
            }
            if let Ok(decoded) = <IndexRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IndexRegistry(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <IsChurnApproverSaltUsedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsChurnApproverSaltUsed(decoded));
            }
            if let Ok(decoded) = <NumRegistriesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NumRegistries(decoded));
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
            if let Ok(decoded) = <PubkeyRegistrationMessageHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PubkeyRegistrationMessageHash(decoded));
            }
            if let Ok(decoded) = <QuorumCountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::QuorumCount(decoded));
            }
            if let Ok(decoded) = <QuorumUpdateBlockNumberCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::QuorumUpdateBlockNumber(decoded));
            }
            if let Ok(decoded) = <RegisterOperatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RegisterOperator(decoded));
            }
            if let Ok(decoded) = <RegisterOperatorWithChurnCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RegisterOperatorWithChurn(decoded));
            }
            if let Ok(decoded) = <RegistriesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Registries(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <ServiceManagerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ServiceManager(decoded));
            }
            if let Ok(decoded) = <SetChurnApproverCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetChurnApprover(decoded));
            }
            if let Ok(decoded) = <SetEjectorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetEjector(decoded));
            }
            if let Ok(decoded) = <SetOperatorIdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetOperatorId(decoded));
            }
            if let Ok(decoded) = <SetOperatorSetParamsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetOperatorSetParams(decoded));
            }
            if let Ok(decoded) = <SetPauserRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetPauserRegistry(decoded));
            }
            if let Ok(decoded) = <SetQuorumCountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetQuorumCount(decoded));
            }
            if let Ok(decoded) = <StakeRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StakeRegistry(decoded));
            }
            if let Ok(decoded) = <TargetArtifactSelectorsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TargetArtifactSelectors(decoded));
            }
            if let Ok(decoded) = <TargetArtifactsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TargetArtifacts(decoded));
            }
            if let Ok(decoded) = <TargetContractsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TargetContracts(decoded));
            }
            if let Ok(decoded) = <TargetInterfacesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TargetInterfaces(decoded));
            }
            if let Ok(decoded) = <TargetSelectorsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TargetSelectors(decoded));
            }
            if let Ok(decoded) = <TargetSendersCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TargetSenders(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <UnpauseCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Unpause(decoded));
            }
            if let Ok(decoded) = <UpdateOperatorsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateOperators(decoded));
            }
            if let Ok(decoded) = <UpdateOperatorsForQuorumCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateOperatorsForQuorum(decoded));
            }
            if let Ok(decoded) = <UpdateSocketCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateSocket(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for RegistryCoordinatorHarnessCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::IsTest(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OperatorChurnApprovalTypehash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PubkeyRegistrationTypehash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeregisterOperatorExternal(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterOperatorExternal(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateOperatorBitmapExternal(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateOperatorExternal(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BlsApkRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateOperatorChurnApprovalDigestHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChurnApprover(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateQuorum(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeregisterOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EjectOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Ejector(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExcludeArtifacts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExcludeContracts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExcludeSenders(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Failed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetCurrentQuorumBitmap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOperatorFromId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOperatorId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOperatorSetParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOperatorStatus(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetQuorumBitmapAtBlockNumberByIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetQuorumBitmapHistoryLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetQuorumBitmapIndicesAtBlockNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetQuorumBitmapUpdateByIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IndexRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsChurnApproverSaltUsed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NumRegistries(element) => {
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
                Self::PubkeyRegistrationMessageHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QuorumCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QuorumUpdateBlockNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterOperatorWithChurn(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Registries(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ServiceManager(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetChurnApprover(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetEjector(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetOperatorId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetOperatorSetParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPauserRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetQuorumCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StakeRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TargetArtifactSelectors(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TargetArtifacts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TargetContracts(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TargetInterfaces(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TargetSelectors(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TargetSenders(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unpause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateOperators(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateOperatorsForQuorum(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateSocket(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for RegistryCoordinatorHarnessCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::IsTest(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorChurnApprovalTypehash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PubkeyRegistrationTypehash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DeregisterOperatorExternal(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RegisterOperatorExternal(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateOperatorBitmapExternal(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateOperatorExternal(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BlsApkRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::CalculateOperatorChurnApprovalDigestHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChurnApprover(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateQuorum(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeregisterOperator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EjectOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::Ejector(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExcludeArtifacts(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExcludeContracts(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExcludeSenders(element) => ::core::fmt::Display::fmt(element, f),
                Self::Failed(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCurrentQuorumBitmap(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOperatorFromId(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOperatorId(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOperatorSetParams(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetOperatorStatus(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetQuorumBitmapAtBlockNumberByIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetQuorumBitmapHistoryLength(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetQuorumBitmapIndicesAtBlockNumber(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetQuorumBitmapUpdateByIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IndexRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsChurnApproverSaltUsed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NumRegistries(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pause(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauseAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::PausedWithIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::Paused(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauserRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::PubkeyRegistrationMessageHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::QuorumCount(element) => ::core::fmt::Display::fmt(element, f),
                Self::QuorumUpdateBlockNumber(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RegisterOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterOperatorWithChurn(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Registries(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::ServiceManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetChurnApprover(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetEjector(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetOperatorId(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetOperatorSetParams(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetPauserRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetQuorumCount(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetArtifactSelectors(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TargetArtifacts(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetContracts(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetInterfaces(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetSelectors(element) => ::core::fmt::Display::fmt(element, f),
                Self::TargetSenders(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unpause(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateOperators(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateOperatorsForQuorum(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateSocket(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<IsTestCall> for RegistryCoordinatorHarnessCalls {
        fn from(value: IsTestCall) -> Self {
            Self::IsTest(value)
        }
    }
    impl ::core::convert::From<OperatorChurnApprovalTypehashCall>
    for RegistryCoordinatorHarnessCalls {
        fn from(value: OperatorChurnApprovalTypehashCall) -> Self {
            Self::OperatorChurnApprovalTypehash(value)
        }
    }
    impl ::core::convert::From<PubkeyRegistrationTypehashCall>
    for RegistryCoordinatorHarnessCalls {
        fn from(value: PubkeyRegistrationTypehashCall) -> Self {
            Self::PubkeyRegistrationTypehash(value)
        }
    }
    impl ::core::convert::From<DeregisterOperatorExternalCall>
    for RegistryCoordinatorHarnessCalls {
        fn from(value: DeregisterOperatorExternalCall) -> Self {
            Self::DeregisterOperatorExternal(value)
        }
    }
    impl ::core::convert::From<RegisterOperatorExternalCall>
    for RegistryCoordinatorHarnessCalls {
        fn from(value: RegisterOperatorExternalCall) -> Self {
            Self::RegisterOperatorExternal(value)
        }
    }
    impl ::core::convert::From<UpdateOperatorBitmapExternalCall>
    for RegistryCoordinatorHarnessCalls {
        fn from(value: UpdateOperatorBitmapExternalCall) -> Self {
            Self::UpdateOperatorBitmapExternal(value)
        }
    }
    impl ::core::convert::From<UpdateOperatorExternalCall>
    for RegistryCoordinatorHarnessCalls {
        fn from(value: UpdateOperatorExternalCall) -> Self {
            Self::UpdateOperatorExternal(value)
        }
    }
    impl ::core::convert::From<BlsApkRegistryCall> for RegistryCoordinatorHarnessCalls {
        fn from(value: BlsApkRegistryCall) -> Self {
            Self::BlsApkRegistry(value)
        }
    }
    impl ::core::convert::From<CalculateOperatorChurnApprovalDigestHashCall>
    for RegistryCoordinatorHarnessCalls {
        fn from(value: CalculateOperatorChurnApprovalDigestHashCall) -> Self {
            Self::CalculateOperatorChurnApprovalDigestHash(value)
        }
    }
    impl ::core::convert::From<ChurnApproverCall> for RegistryCoordinatorHarnessCalls {
        fn from(value: ChurnApproverCall) -> Self {
            Self::ChurnApprover(value)
        }
    }
    impl ::core::convert::From<CreateQuorumCall> for RegistryCoordinatorHarnessCalls {
        fn from(value: CreateQuorumCall) -> Self {
            Self::CreateQuorum(value)
        }
    }
    impl ::core::convert::From<DeregisterOperatorCall>
    for RegistryCoordinatorHarnessCalls {
        fn from(value: DeregisterOperatorCall) -> Self {
            Self::DeregisterOperator(value)
        }
    }
    impl ::core::convert::From<EjectOperatorCall> for RegistryCoordinatorHarnessCalls {
        fn from(value: EjectOperatorCall) -> Self {
            Self::EjectOperator(value)
        }
    }
    impl ::core::convert::From<EjectorCall> for RegistryCoordinatorHarnessCalls {
        fn from(value: EjectorCall) -> Self {
            Self::Ejector(value)
        }
    }
    impl ::core::convert::From<ExcludeArtifactsCall>
    for RegistryCoordinatorHarnessCalls {
        fn from(value: ExcludeArtifactsCall) -> Self {
            Self::ExcludeArtifacts(value)
        }
    }
    impl ::core::convert::From<ExcludeContractsCall>
    for RegistryCoordinatorHarnessCalls {
        fn from(value: ExcludeContractsCall) -> Self {
            Self::ExcludeContracts(value)
        }
    }
    impl ::core::convert::From<ExcludeSendersCall> for RegistryCoordinatorHarnessCalls {
        fn from(value: ExcludeSendersCall) -> Self {
            Self::ExcludeSenders(value)
        }
    }
    impl ::core::convert::From<FailedCall> for RegistryCoordinatorHarnessCalls {
        fn from(value: FailedCall) -> Self {
            Self::Failed(value)
        }
    }
    impl ::core::convert::From<GetCurrentQuorumBitmapCall>
    for RegistryCoordinatorHarnessCalls {
        fn from(value: GetCurrentQuorumBitmapCall) -> Self {
            Self::GetCurrentQuorumBitmap(value)
        }
    }
    impl ::core::convert::From<GetOperatorCall> for RegistryCoordinatorHarnessCalls {
        fn from(value: GetOperatorCall) -> Self {
            Self::GetOperator(value)
        }
    }
    impl ::core::convert::From<GetOperatorFromIdCall>
    for RegistryCoordinatorHarnessCalls {
        fn from(value: GetOperatorFromIdCall) -> Self {
            Self::GetOperatorFromId(value)
        }
    }
    impl ::core::convert::From<GetOperatorIdCall> for RegistryCoordinatorHarnessCalls {
        fn from(value: GetOperatorIdCall) -> Self {
            Self::GetOperatorId(value)
        }
    }
    impl ::core::convert::From<GetOperatorSetParamsCall>
    for RegistryCoordinatorHarnessCalls {
        fn from(value: GetOperatorSetParamsCall) -> Self {
            Self::GetOperatorSetParams(value)
        }
    }
    impl ::core::convert::From<GetOperatorStatusCall>
    for RegistryCoordinatorHarnessCalls {
        fn from(value: GetOperatorStatusCall) -> Self {
            Self::GetOperatorStatus(value)
        }
    }
    impl ::core::convert::From<GetQuorumBitmapAtBlockNumberByIndexCall>
    for RegistryCoordinatorHarnessCalls {
        fn from(value: GetQuorumBitmapAtBlockNumberByIndexCall) -> Self {
            Self::GetQuorumBitmapAtBlockNumberByIndex(value)
        }
    }
    impl ::core::convert::From<GetQuorumBitmapHistoryLengthCall>
    for RegistryCoordinatorHarnessCalls {
        fn from(value: GetQuorumBitmapHistoryLengthCall) -> Self {
            Self::GetQuorumBitmapHistoryLength(value)
        }
    }
    impl ::core::convert::From<GetQuorumBitmapIndicesAtBlockNumberCall>
    for RegistryCoordinatorHarnessCalls {
        fn from(value: GetQuorumBitmapIndicesAtBlockNumberCall) -> Self {
            Self::GetQuorumBitmapIndicesAtBlockNumber(value)
        }
    }
    impl ::core::convert::From<GetQuorumBitmapUpdateByIndexCall>
    for RegistryCoordinatorHarnessCalls {
        fn from(value: GetQuorumBitmapUpdateByIndexCall) -> Self {
            Self::GetQuorumBitmapUpdateByIndex(value)
        }
    }
    impl ::core::convert::From<IndexRegistryCall> for RegistryCoordinatorHarnessCalls {
        fn from(value: IndexRegistryCall) -> Self {
            Self::IndexRegistry(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for RegistryCoordinatorHarnessCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<IsChurnApproverSaltUsedCall>
    for RegistryCoordinatorHarnessCalls {
        fn from(value: IsChurnApproverSaltUsedCall) -> Self {
            Self::IsChurnApproverSaltUsed(value)
        }
    }
    impl ::core::convert::From<NumRegistriesCall> for RegistryCoordinatorHarnessCalls {
        fn from(value: NumRegistriesCall) -> Self {
            Self::NumRegistries(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for RegistryCoordinatorHarnessCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PauseCall> for RegistryCoordinatorHarnessCalls {
        fn from(value: PauseCall) -> Self {
            Self::Pause(value)
        }
    }
    impl ::core::convert::From<PauseAllCall> for RegistryCoordinatorHarnessCalls {
        fn from(value: PauseAllCall) -> Self {
            Self::PauseAll(value)
        }
    }
    impl ::core::convert::From<PausedWithIndexCall> for RegistryCoordinatorHarnessCalls {
        fn from(value: PausedWithIndexCall) -> Self {
            Self::PausedWithIndex(value)
        }
    }
    impl ::core::convert::From<PausedCall> for RegistryCoordinatorHarnessCalls {
        fn from(value: PausedCall) -> Self {
            Self::Paused(value)
        }
    }
    impl ::core::convert::From<PauserRegistryCall> for RegistryCoordinatorHarnessCalls {
        fn from(value: PauserRegistryCall) -> Self {
            Self::PauserRegistry(value)
        }
    }
    impl ::core::convert::From<PubkeyRegistrationMessageHashCall>
    for RegistryCoordinatorHarnessCalls {
        fn from(value: PubkeyRegistrationMessageHashCall) -> Self {
            Self::PubkeyRegistrationMessageHash(value)
        }
    }
    impl ::core::convert::From<QuorumCountCall> for RegistryCoordinatorHarnessCalls {
        fn from(value: QuorumCountCall) -> Self {
            Self::QuorumCount(value)
        }
    }
    impl ::core::convert::From<QuorumUpdateBlockNumberCall>
    for RegistryCoordinatorHarnessCalls {
        fn from(value: QuorumUpdateBlockNumberCall) -> Self {
            Self::QuorumUpdateBlockNumber(value)
        }
    }
    impl ::core::convert::From<RegisterOperatorCall>
    for RegistryCoordinatorHarnessCalls {
        fn from(value: RegisterOperatorCall) -> Self {
            Self::RegisterOperator(value)
        }
    }
    impl ::core::convert::From<RegisterOperatorWithChurnCall>
    for RegistryCoordinatorHarnessCalls {
        fn from(value: RegisterOperatorWithChurnCall) -> Self {
            Self::RegisterOperatorWithChurn(value)
        }
    }
    impl ::core::convert::From<RegistriesCall> for RegistryCoordinatorHarnessCalls {
        fn from(value: RegistriesCall) -> Self {
            Self::Registries(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall>
    for RegistryCoordinatorHarnessCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<ServiceManagerCall> for RegistryCoordinatorHarnessCalls {
        fn from(value: ServiceManagerCall) -> Self {
            Self::ServiceManager(value)
        }
    }
    impl ::core::convert::From<SetChurnApproverCall>
    for RegistryCoordinatorHarnessCalls {
        fn from(value: SetChurnApproverCall) -> Self {
            Self::SetChurnApprover(value)
        }
    }
    impl ::core::convert::From<SetEjectorCall> for RegistryCoordinatorHarnessCalls {
        fn from(value: SetEjectorCall) -> Self {
            Self::SetEjector(value)
        }
    }
    impl ::core::convert::From<SetOperatorIdCall> for RegistryCoordinatorHarnessCalls {
        fn from(value: SetOperatorIdCall) -> Self {
            Self::SetOperatorId(value)
        }
    }
    impl ::core::convert::From<SetOperatorSetParamsCall>
    for RegistryCoordinatorHarnessCalls {
        fn from(value: SetOperatorSetParamsCall) -> Self {
            Self::SetOperatorSetParams(value)
        }
    }
    impl ::core::convert::From<SetPauserRegistryCall>
    for RegistryCoordinatorHarnessCalls {
        fn from(value: SetPauserRegistryCall) -> Self {
            Self::SetPauserRegistry(value)
        }
    }
    impl ::core::convert::From<SetQuorumCountCall> for RegistryCoordinatorHarnessCalls {
        fn from(value: SetQuorumCountCall) -> Self {
            Self::SetQuorumCount(value)
        }
    }
    impl ::core::convert::From<StakeRegistryCall> for RegistryCoordinatorHarnessCalls {
        fn from(value: StakeRegistryCall) -> Self {
            Self::StakeRegistry(value)
        }
    }
    impl ::core::convert::From<TargetArtifactSelectorsCall>
    for RegistryCoordinatorHarnessCalls {
        fn from(value: TargetArtifactSelectorsCall) -> Self {
            Self::TargetArtifactSelectors(value)
        }
    }
    impl ::core::convert::From<TargetArtifactsCall> for RegistryCoordinatorHarnessCalls {
        fn from(value: TargetArtifactsCall) -> Self {
            Self::TargetArtifacts(value)
        }
    }
    impl ::core::convert::From<TargetContractsCall> for RegistryCoordinatorHarnessCalls {
        fn from(value: TargetContractsCall) -> Self {
            Self::TargetContracts(value)
        }
    }
    impl ::core::convert::From<TargetInterfacesCall>
    for RegistryCoordinatorHarnessCalls {
        fn from(value: TargetInterfacesCall) -> Self {
            Self::TargetInterfaces(value)
        }
    }
    impl ::core::convert::From<TargetSelectorsCall> for RegistryCoordinatorHarnessCalls {
        fn from(value: TargetSelectorsCall) -> Self {
            Self::TargetSelectors(value)
        }
    }
    impl ::core::convert::From<TargetSendersCall> for RegistryCoordinatorHarnessCalls {
        fn from(value: TargetSendersCall) -> Self {
            Self::TargetSenders(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall>
    for RegistryCoordinatorHarnessCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UnpauseCall> for RegistryCoordinatorHarnessCalls {
        fn from(value: UnpauseCall) -> Self {
            Self::Unpause(value)
        }
    }
    impl ::core::convert::From<UpdateOperatorsCall> for RegistryCoordinatorHarnessCalls {
        fn from(value: UpdateOperatorsCall) -> Self {
            Self::UpdateOperators(value)
        }
    }
    impl ::core::convert::From<UpdateOperatorsForQuorumCall>
    for RegistryCoordinatorHarnessCalls {
        fn from(value: UpdateOperatorsForQuorumCall) -> Self {
            Self::UpdateOperatorsForQuorum(value)
        }
    }
    impl ::core::convert::From<UpdateSocketCall> for RegistryCoordinatorHarnessCalls {
        fn from(value: UpdateSocketCall) -> Self {
            Self::UpdateSocket(value)
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
        Hash
    )]
    pub struct IsTestReturn(pub bool);
    ///Container type for all return fields from the `OPERATOR_CHURN_APPROVAL_TYPEHASH` function with signature `OPERATOR_CHURN_APPROVAL_TYPEHASH()` and selector `0xca0de882`
    #[derive(
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
    pub struct OperatorChurnApprovalTypehashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `PUBKEY_REGISTRATION_TYPEHASH` function with signature `PUBKEY_REGISTRATION_TYPEHASH()` and selector `0x9feab859`
    #[derive(
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
    pub struct PubkeyRegistrationTypehashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `_registerOperatorExternal` function with signature `_registerOperatorExternal(address,bytes32,bytes,string,(bytes,bytes32,uint256))` and selector `0x1ab2574f`
    #[derive(
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
    pub struct RegisterOperatorExternalReturn {
        pub results: RegisterResults,
    }
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
    ///Container type for all return fields from the `calculateOperatorChurnApprovalDigestHash` function with signature `calculateOperatorChurnApprovalDigestHash(address,bytes32,(uint8,address)[],bytes32,uint256)` and selector `0x84ca5213`
    #[derive(
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
    pub struct CalculateOperatorChurnApprovalDigestHashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `churnApprover` function with signature `churnApprover()` and selector `0x054310e6`
    #[derive(
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
    pub struct ChurnApproverReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `ejector` function with signature `ejector()` and selector `0x28f61b31`
    #[derive(
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
    pub struct EjectorReturn(pub ::ethers::core::types::Address);
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
        Hash
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
        Hash
    )]
    pub struct ExcludeContractsReturn {
        pub excluded_contracts: ::std::vec::Vec<::ethers::core::types::Address>,
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
        Hash
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
        Hash
    )]
    pub struct FailedReturn(pub bool);
    ///Container type for all return fields from the `getCurrentQuorumBitmap` function with signature `getCurrentQuorumBitmap(bytes32)` and selector `0x871ef049`
    #[derive(
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
    pub struct GetCurrentQuorumBitmapReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getOperator` function with signature `getOperator(address)` and selector `0x5865c60c`
    #[derive(
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
    pub struct GetOperatorReturn(pub OperatorInfo);
    ///Container type for all return fields from the `getOperatorFromId` function with signature `getOperatorFromId(bytes32)` and selector `0x296bb064`
    #[derive(
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
    pub struct GetOperatorFromIdReturn(pub ::ethers::core::types::Address);
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
        Hash
    )]
    pub struct GetOperatorIdReturn(pub [u8; 32]);
    ///Container type for all return fields from the `getOperatorSetParams` function with signature `getOperatorSetParams(uint8)` and selector `0xe65797ad`
    #[derive(
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
    pub struct GetOperatorSetParamsReturn(pub OperatorSetParam);
    ///Container type for all return fields from the `getOperatorStatus` function with signature `getOperatorStatus(address)` and selector `0xfd39105a`
    #[derive(
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
    pub struct GetOperatorStatusReturn(pub u8);
    ///Container type for all return fields from the `getQuorumBitmapAtBlockNumberByIndex` function with signature `getQuorumBitmapAtBlockNumberByIndex(bytes32,uint32,uint256)` and selector `0x04ec6351`
    #[derive(
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
    pub struct GetQuorumBitmapAtBlockNumberByIndexReturn(
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `getQuorumBitmapHistoryLength` function with signature `getQuorumBitmapHistoryLength(bytes32)` and selector `0x03fd3492`
    #[derive(
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
    pub struct GetQuorumBitmapHistoryLengthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getQuorumBitmapIndicesAtBlockNumber` function with signature `getQuorumBitmapIndicesAtBlockNumber(uint32,bytes32[])` and selector `0xc391425e`
    #[derive(
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
    pub struct GetQuorumBitmapIndicesAtBlockNumberReturn(pub ::std::vec::Vec<u32>);
    ///Container type for all return fields from the `getQuorumBitmapUpdateByIndex` function with signature `getQuorumBitmapUpdateByIndex(bytes32,uint256)` and selector `0x1eb812da`
    #[derive(
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
    pub struct GetQuorumBitmapUpdateByIndexReturn(pub QuorumBitmapUpdate);
    ///Container type for all return fields from the `indexRegistry` function with signature `indexRegistry()` and selector `0x9e9923c2`
    #[derive(
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
    pub struct IndexRegistryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `isChurnApproverSaltUsed` function with signature `isChurnApproverSaltUsed(bytes32)` and selector `0x1478851f`
    #[derive(
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
    pub struct IsChurnApproverSaltUsedReturn(pub bool);
    ///Container type for all return fields from the `numRegistries` function with signature `numRegistries()` and selector `0xd72d8dd6`
    #[derive(
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
    pub struct NumRegistriesReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `pubkeyRegistrationMessageHash` function with signature `pubkeyRegistrationMessageHash(address)` and selector `0x3c2a7f4c`
    #[derive(
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
    pub struct PubkeyRegistrationMessageHashReturn(pub G1Point);
    ///Container type for all return fields from the `quorumCount` function with signature `quorumCount()` and selector `0x9aa1653d`
    #[derive(
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
    pub struct QuorumCountReturn(pub u8);
    ///Container type for all return fields from the `quorumUpdateBlockNumber` function with signature `quorumUpdateBlockNumber(uint8)` and selector `0x249a0c42`
    #[derive(
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
    pub struct QuorumUpdateBlockNumberReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `registries` function with signature `registries(uint256)` and selector `0x6347c900`
    #[derive(
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
    pub struct RegistriesReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `serviceManager` function with signature `serviceManager()` and selector `0x3998fdd3`
    #[derive(
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
    pub struct ServiceManagerReturn(pub ::ethers::core::types::Address);
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
        Hash
    )]
    pub struct TargetArtifactSelectorsReturn {
        pub targeted_artifact_selectors: ::std::vec::Vec<FuzzSelector>,
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
    )]
    pub struct TargetSendersReturn {
        pub targeted_senders: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///`RegisterResults(uint32[],uint96[],uint96[])`
    #[derive(
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
    pub struct RegisterResults {
        pub num_operators_per_quorum: ::std::vec::Vec<u32>,
        pub operator_stakes: ::std::vec::Vec<u128>,
        pub total_stakes: ::std::vec::Vec<u128>,
    }
}
