pub use registry_coordinator::*;
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
pub mod registry_coordinator {
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
                    ::std::borrow::ToOwned::to_owned("ejectionCooldown"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ejectionCooldown"),
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
                    ::std::borrow::ToOwned::to_owned("lastEjectionTimestamp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "lastEjectionTimestamp",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("setEjectionCooldown"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setEjectionCooldown",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_ejectionCooldown"),
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
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static REGISTRYCOORDINATOR_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01\xC0`@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@Qb\0c\x868\x03\x80b\0c\x86\x839\x81\x01`@\x81\x90Rb\0\x005\x91b\0\x02TV[`@\x80Q\x80\x82\x01\x82R`\x16\x81R\x7FAVSRegistryCoordinator\0\0\0\0\0\0\0\0\0\0` \x80\x83\x01\x91\x82R\x83Q\x80\x85\x01\x90\x94R`\x06\x84Rev0.0.1`\xD0\x1B\x90\x84\x01R\x81Q\x90 `\xE0\x81\x90R\x7Fk\xDA~?8^H\x84\x10H9\x04D\xCC\xED\\\xC7\x95\xAF\x87u\x8A\xF6v\"\xE5\xF4\xF0\x88,J\x99a\x01\0\x81\x90RF`\xA0R\x87\x93\x87\x93\x87\x93\x87\x93\x91\x92\x91\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0Fb\0\x015\x81\x84\x84`@\x80Q` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90RF`\x80\x82\x01R0`\xA0\x82\x01R`\0\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x93\x92PPPV[`\x80R0`\xC0Ra\x01 RPPPP`\x01`\x01`\xA0\x1B\x03\x93\x84\x16a\x01@R\x91\x83\x16a\x01\x80R\x82\x16a\x01`R\x16a\x01\xA0Rb\0\x01ob\0\x01yV[PPPPb\0\x02\xBCV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15b\0\x01\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\xFF\x90\x81\x16\x10\x15b\0\x029W`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x02QW`\0\x80\xFD[PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15b\0\x02kW`\0\x80\xFD[\x84Qb\0\x02x\x81b\0\x02;V[` \x86\x01Q\x90\x94Pb\0\x02\x8B\x81b\0\x02;V[`@\x86\x01Q\x90\x93Pb\0\x02\x9E\x81b\0\x02;V[``\x86\x01Q\x90\x92Pb\0\x02\xB1\x81b\0\x02;V[\x93\x96\x92\x95P\x90\x93PPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x01@Qa\x01`Qa\x01\x80Qa\x01\xA0Qa_\xC2b\0\x03\xC4`\09`\0\x81\x81a\x06\xAB\x01R\x81\x81a\x11\x9B\x01R\x81\x81a \x80\x01R\x81\x81a.\xB1\x01R\x81\x81a7l\x01Ra=C\x01R`\0\x81\x81a\x05\xF0\x01R\x81\x81a \x0B\x01R\x81\x81a$\xB5\x01R\x81\x81a.1\x01R\x81\x81a6\xC3\x01R\x81\x81a9\x18\x01Ra<\xC2\x01R`\0\x81\x81a\x05\xB6\x01R\x81\x81a\x0F7\x01R\x81\x81a I\x01R\x81\x81a-\xB3\x01R\x81\x81a/\x99\x01R\x81\x81a0\x13\x01R\x81\x81a6C\x01Ra=\xBF\x01R`\0\x81\x81a\x04\xFA\x01R\x81\x81a-\t\x01Ra5\x8B\x01R`\0a?\xC6\x01R`\0a@\x15\x01R`\0a?\xF0\x01R`\0a?I\x01R`\0a?s\x01R`\0a?\x9D\x01Ra_\xC2`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\xD5W`\x005`\xE0\x1C\x80c]\xF4YF\x11a\x01\x82W\x80c\x9F\xEA\xB8Y\x11a\0\xE9W\x80c\xD7-\x8D\xD6\x11a\0\xA2W\x80c\xE6W\x97\xAD\x11a\0|W\x80c\xE6W\x97\xAD\x14a\x07\x98W\x80c\xF2\xFD\xE3\x8B\x14a\x08;W\x80c\xFA\xBC\x1C\xBC\x14a\x08NW\x80c\xFD9\x10Z\x14a\x08aW`\0\x80\xFD[\x80c\xD7-\x8D\xD6\x14a\x07jW\x80c\xD7[L\x88\x14a\x07rW\x80c\xDD\x82\x83\xF3\x14a\x07\x85W`\0\x80\xFD[\x80c\x9F\xEA\xB8Y\x14a\x06\xCDW\x80c\xA5\x08W\xBF\x14a\x06\xF4W\x80c\xA9ox>\x14a\x07\x07W\x80c\xC3\x91B^\x14a\x07\x10W\x80c\xCA\r\xE8\x82\x14a\x070W\x80c\xCAO-\x97\x14a\x07WW`\0\x80\xFD[\x80c\x87\x1E\xF0I\x11a\x01;W\x80c\x87\x1E\xF0I\x14a\x06@W\x80c\x88o\x11\x95\x14a\x06SW\x80c\x8D\xA5\xCB[\x14a\x06lW\x80c\x9A\xA1e=\x14a\x06tW\x80c\x9B]\x17{\x14a\x06\x93W\x80c\x9E\x99#\xC2\x14a\x06\xA6W`\0\x80\xFD[\x80c]\xF4YF\x14a\x05\xB1W\x80ccG\xC9\0\x14a\x05\xD8W\x80ch0H5\x14a\x05\xEBW\x80cn;\x17\xDB\x14a\x06\x12W\x80cqP\x18\xA6\x14a\x06%W\x80c\x84\xCAR\x13\x14a\x06-W`\0\x80\xFD[\x80c$\x9A\x0CB\x11a\x02AW\x80c<*\x7FL\x11a\x01\xFAW\x80cY\\jg\x11a\x01\xD4W\x80cY\\jg\x14a\x05oW\x80cZ\xC8j\xB7\x14a\x05wW\x80c[\x0B\x82\x9F\x14a\x05\x96W\x80c\\\x97Z\xBB\x14a\x05\xA9W`\0\x80\xFD[\x80c<*\x7FL\x14a\x05\x1CW\x80cQ@\xA5H\x14a\x05<W\x80cXe\xC6\x0C\x14a\x05OW`\0\x80\xFD[\x80c$\x9A\x0CB\x14a\x04\x89W\x80c(\xF6\x1B1\x14a\x04\xA9W\x80c)k\xB0d\x14a\x04\xBCW\x80c)\xD1\xE0\xC3\x14a\x04\xCFW\x80c,\xDD\x1E\x86\x14a\x04\xE2W\x80c9\x98\xFD\xD3\x14a\x04\xF5W`\0\x80\xFD[\x80c\x10\xD6z/\x11a\x02\x93W\x80c\x10\xD6z/\x14a\x03\x9EW\x80c\x12^\x05\x84\x14a\x03\xB1W\x80c\x13T*N\x14a\x03\xD1W\x80c\x13d9\xDD\x14a\x03\xFAW\x80c\x14x\x85\x1F\x14a\x04\rW\x80c\x1E\xB8\x12\xDA\x14a\x04@W`\0\x80\xFD[\x80b\xCF*\xB5\x14a\x02\xDAW\x80c\x03\xFD4\x92\x14a\x02\xEFW\x80c\x04\xECcQ\x14a\x03\"W\x80c\x05C\x10\xE6\x14a\x03MW\x80c\x0C\xF4\xB7g\x14a\x03xW\x80c\r?!4\x14a\x03\x8BW[`\0\x80\xFD[a\x02\xEDa\x02\xE86`\x04aJ\xBDV[a\x08\x9DV[\0[a\x03\x0Fa\x02\xFD6`\x04aJ\xFEV[`\0\x90\x81R`\x98` R`@\x90 T\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x035a\x0306`\x04aK)V[a\t\xB2V[`@Q`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x81R` \x01a\x03\x19V[`\x9DTa\x03`\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03\x19V[a\x02\xEDa\x03\x866`\x04aLHV[a\x0B\xA8V[a\x02\xEDa\x03\x996`\x04aJ\xFEV[a\x0C\x90V[a\x02\xEDa\x03\xAC6`\x04aL\xBDV[a\x0C\x9DV[a\x03\x0Fa\x03\xBF6`\x04aL\xBDV[`\x9F` R`\0\x90\x81R`@\x90 T\x81V[a\x03\x0Fa\x03\xDF6`\x04aL\xBDV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x99` R`@\x90 T\x90V[a\x02\xEDa\x04\x086`\x04aJ\xFEV[a\rPV[a\x040a\x04\x1B6`\x04aJ\xFEV[`\x9A` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x03\x19V[a\x04Sa\x04N6`\x04aL\xDAV[a\x0E\x8DV[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x80\x85\x01Q\x90\x91\x16\x90\x82\x01R\x91\x81\x01Q`\x01`\x01`\xC0\x1B\x03\x16\x90\x82\x01R``\x01a\x03\x19V[a\x03\x0Fa\x04\x976`\x04aM\rV[`\x9B` R`\0\x90\x81R`@\x90 T\x81V[`\x9ETa\x03`\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03`a\x04\xCA6`\x04aJ\xFEV[a\x0F\x1EV[a\x02\xEDa\x04\xDD6`\x04aL\xBDV[a\x0F\xAAV[a\x02\xEDa\x04\xF06`\x04aL\xBDV[a\x0F\xBBV[a\x03`\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x05/a\x05*6`\x04aL\xBDV[a\x0F\xCCV[`@Qa\x03\x19\x91\x90aM(V[a\x02\xEDa\x05J6`\x04aM\x80V[a\x10KV[a\x05ba\x05]6`\x04aL\xBDV[a\x15[V[`@Qa\x03\x19\x91\x90aN#V[a\x02\xEDa\x15\xCFV[a\x040a\x05\x856`\x04aM\rV[`\x01\x80T`\xFF\x90\x92\x16\x1B\x90\x81\x16\x14\x90V[a\x02\xEDa\x05\xA46`\x04aN\xA8V[a\x16\x9BV[`\x01Ta\x03\x0FV[a\x03`\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03`a\x05\xE66`\x04aJ\xFEV[a\x16\xBCV[a\x03`\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xEDa\x06 6`\x04aN\xDCV[a\x16\xE6V[a\x02\xEDa\x17\xFCV[a\x03\x0Fa\x06;6`\x04aO\x93V[a\x18\x10V[a\x035a\x06N6`\x04aJ\xFEV[a\x18ZV[`\0Ta\x03`\x90b\x01\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03`a\x18eV[`\x96Ta\x06\x81\x90`\xFF\x16\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x03\x19V[a\x02\xEDa\x06\xA16`\x04aQ,V[a\x18~V[a\x03`\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\x0F\x7F+\xD8!$\x05\x7F\t\x13\xBC;w,\xE7\xB8>\x80W\xC1\xAD\x1F5\x10\xFC\x83w\x8B\xE2\x0F\x10\xEC]\xE6\x81V[a\x02\xEDa\x07\x026`\x04aR%V[a\x1B\xB5V[a\x03\x0F`\xA0T\x81V[a\x07#a\x07\x1E6`\x04aR\xCDV[a\x1D8V[`@Qa\x03\x19\x91\x90aSrV[a\x03\x0F\x7FM@N2v\xE7\xAC!c\xD8\xEEGj\xFAjA\xD1\xF6\x8F\xB7\x1F-\x8BeF\xB2NU\xCE\x01\xB7*\x81V[a\x02\xEDa\x07e6`\x04aS\xBCV[a\x1D\xF1V[`\x9CTa\x03\x0FV[a\x02\xEDa\x07\x806`\x04aT\xA2V[a\x1EWV[a\x02\xEDa\x07\x936`\x04aVUV[a\x1EjV[a\x08\x07a\x07\xA66`\x04aM\rV[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x92\x84\x01\x81\x90R`\xFF\x94\x90\x94\x16\x84R`\x97\x82R\x92\x82\x90 \x82Q\x93\x84\x01\x83RTc\xFF\xFF\xFF\xFF\x81\x16\x84Ra\xFF\xFF`\x01` \x1B\x82\x04\x81\x16\x92\x85\x01\x92\x90\x92R`\x01`0\x1B\x90\x04\x16\x90\x82\x01R\x90V[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x16\x81R` \x80\x84\x01Qa\xFF\xFF\x90\x81\x16\x91\x83\x01\x91\x90\x91R\x92\x82\x01Q\x90\x92\x16\x90\x82\x01R``\x01a\x03\x19V[a\x02\xEDa\x08I6`\x04aL\xBDV[a!nV[a\x02\xEDa\x08\\6`\x04aJ\xFEV[a!\xE4V[a\x08\x90a\x08o6`\x04aL\xBDV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T`\xFF\x16\x90V[`@Qa\x03\x19\x91\x90aW)V[`\x01T`\x02\x90`\x04\x90\x81\x16\x03a\x08\xCEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xC5\x90aW7V[`@Q\x80\x91\x03\x90\xFD[`\0[\x82\x81\x10\x15a\t\xACW`\0\x84\x84\x83\x81\x81\x10a\x08\xEDWa\x08\xEDaWnV[\x90P` \x02\x01` \x81\x01\x90a\t\x02\x91\x90aL\xBDV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x99` \x90\x81R`@\x80\x83 \x81Q\x80\x83\x01\x90\x92R\x80T\x82R`\x01\x81\x01T\x94\x95P\x92\x93\x90\x92\x91\x83\x01\x90`\xFF\x16`\x02\x81\x11\x15a\tMWa\tMaM\xEBV[`\x02\x81\x11\x15a\t^Wa\t^aM\xEBV[\x90RP\x80Q\x90\x91P`\0a\tq\x82a#@V[\x90P`\0a\t\x87\x82`\x01`\x01`\xC0\x1B\x03\x16a#\xABV[\x90Pa\t\x94\x85\x85\x83a$wV[PPPPP\x80\x80a\t\xA4\x90aW\x9AV[\x91PPa\x08\xD1V[PPPPV[`\0\x83\x81R`\x98` R`@\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\t\xD5Wa\t\xD5aWnV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x85R`\x01` \x1B\x83\x04\x82\x16\x95\x85\x01\x95\x90\x95R`\x01`@\x1B\x90\x91\x04`\x01`\x01`\xC0\x1B\x03\x16\x91\x83\x01\x91\x90\x91R\x90\x92P\x85\x16\x10\x15a\n\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`e`$\x82\x01R\x7FRegistryCoordinator.getQuorumBit`D\x82\x01R\x7FmapAtBlockNumberByIndex: quorumB`d\x82\x01R\x7FitmapUpdate is from after blockN`\x84\x82\x01Rd:\xB6\xB12\xB9`\xD9\x1B`\xA4\x82\x01R`\xC4\x01a\x08\xC5V[` \x81\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x80a\n\xF5WP\x80` \x01Qc\xFF\xFF\xFF\xFF\x16\x84c\xFF\xFF\xFF\xFF\x16\x10[a\x0B\x9CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`f`$\x82\x01R\x7FRegistryCoordinator.getQuorumBit`D\x82\x01R\x7FmapAtBlockNumberByIndex: quorumB`d\x82\x01R\x7FitmapUpdate is from before block`\x84\x82\x01Re':\xB6\xB12\xB9`\xD1\x1B`\xA4\x82\x01R`\xC4\x01a\x08\xC5V[`@\x01Q\x94\x93PPPPV[`\x013`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T`\xFF\x16`\x02\x81\x11\x15a\x0B\xD1Wa\x0B\xD1aM\xEBV[\x14a\x0CDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R\x7FRegistryCoordinator.updateSocket`D\x82\x01R\x7F: operator is not registered\0\0\0\0`d\x82\x01R`\x84\x01a\x08\xC5V[3`\0\x90\x81R`\x99` R`@\x90\x81\x90 T\x90Q\x7F\xEC)c\xAB!\xC1\xE5\x0E\x1EX*\xA5B\xAF.K\xF7\xBF8\xE6\xE1@<'\xB4.\x1C]nb\x1E\xAA\x90a\x0C\x85\x90\x84\x90aX\0V[`@Q\x80\x91\x03\x90\xA2PV[a\x0C\x98a%dV[`\xA0UV[`\0`\x02\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xF0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x14\x91\x90aX\x13V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\rDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xC5\x90aX0V[a\rM\x81a%\xC3V[PV[`\0T`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01Rb\x01\0\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\x9DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xC1\x91\x90aXzV[a\r\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xC5\x90aX\x9CV[`\x01T\x81\x81\x16\x14a\x0EVW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08\xC5V[`\x01\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01a\x0C\x85V[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R`\0\x83\x81R`\x98` R`@\x90 \x80T\x83\x90\x81\x10a\x0E\xCAWa\x0E\xCAaWnV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01`\xC0\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90P[\x92\x91PPV[`@Qc\x08\xF6b\x9D`\xE3\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cG\xB3\x14\xE8\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x86W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x18\x91\x90aX\x13V[a\x0F\xB2a%dV[a\rM\x81a&\xC8V[a\x0F\xC3a%dV[a\rM\x81a'1V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x0F\x18a\x10F\x7F+\xD8!$\x05\x7F\t\x13\xBC;w,\xE7\xB8>\x80W\xC1\xAD\x1F5\x10\xFC\x83w\x8B\xE2\x0F\x10\xEC]\xE6\x84`@Q` \x01a\x10+\x92\x91\x90\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a'\x9AV[a'\xE8V[`\x01T`\x02\x90`\x04\x90\x81\x16\x03a\x10sW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xC5\x90aW7V[`\0a\x10\xBB\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`\x96T`\xFF\x16\x91Pa(w\x90PV[\x90P\x84\x83\x14a\x11,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R`\0\x80Q` a_-\x839\x81Q\x91R`D\x82\x01R\x7ForsForQuorum: input length misma`d\x82\x01Rb\x0E\x8Cm`\xEB\x1B`\x84\x82\x01R`\xA4\x01a\x08\xC5V[`\0[\x83\x81\x10\x15a\x15RW`\0\x85\x85\x83\x81\x81\x10a\x11KWa\x11KaWnV[\x91\x90\x91\x015`\xF8\x1C\x91P6\x90P`\0\x89\x89\x85\x81\x81\x10a\x11lWa\x11laWnV[\x90P` \x02\x81\x01\x90a\x11~\x91\x90aX\xE4V[`@Qcy\xA0\x84\x91`\xE1\x1B\x81R`\xFF\x86\x16`\x04\x82\x01R\x91\x93P\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xF3A\t\"\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xEAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\x0E\x91\x90aY-V[c\xFF\xFF\xFF\xFF\x16\x81\x14a\x12\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`e`$\x82\x01R`\0\x80Q` a_-\x839\x81Q\x91R`D\x82\x01R\x7ForsForQuorum: number of updated `d\x82\x01R\x7Foperators does not match quorum `\x84\x82\x01Rd\x1D\x1B\xDD\x18[`\xDA\x1B`\xA4\x82\x01R`\xC4\x01a\x08\xC5V[`\0\x80[\x82\x81\x10\x15a\x14\xF1W`\0\x84\x84\x83\x81\x81\x10a\x12\xCAWa\x12\xCAaWnV[\x90P` \x02\x01` \x81\x01\x90a\x12\xDF\x91\x90aL\xBDV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x99` \x90\x81R`@\x80\x83 \x81Q\x80\x83\x01\x90\x92R\x80T\x82R`\x01\x81\x01T\x94\x95P\x92\x93\x90\x92\x91\x83\x01\x90`\xFF\x16`\x02\x81\x11\x15a\x13*Wa\x13*aM\xEBV[`\x02\x81\x11\x15a\x13;Wa\x13;aM\xEBV[\x90RP\x80Q\x90\x91P`\0a\x13N\x82a#@V[\x90P`\x01`\x01`\x01`\xC0\x1B\x03\x82\x16`\xFF\x8B\x16\x1C\x81\x16\x14a\x13\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R`\0\x80Q` a_-\x839\x81Q\x91R\x90\x82\x01R\x7ForsForQuorum: operator not in qu`d\x82\x01Rcorum`\xE0\x1B`\x84\x82\x01R`\xA4\x01a\x08\xC5V[\x85`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x11a\x14}W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`g`$\x82\x01R`\0\x80Q` a_-\x839\x81Q\x91R`D\x82\x01R\x7ForsForQuorum: operators array mu`d\x82\x01R\x7Fst be sorted in ascending addres`\x84\x82\x01Rf9\x907\xB922\xB9`\xC9\x1B`\xA4\x82\x01R`\xC4\x01a\x08\xC5V[Pa\x14\xDB\x83\x83\x8F\x8F\x8D\x90\x8E`\x01a\x14\x94\x91\x90aYJV[\x92a\x14\xA1\x93\x92\x91\x90aYbV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa$w\x92PPPV[P\x90\x92Pa\x14\xEA\x90P\x81aW\x9AV[\x90Pa\x12\xAEV[P`\xFF\x84\x16`\0\x81\x81R`\x9B` \x90\x81R`@\x91\x82\x90 C\x90\x81\x90U\x91Q\x91\x82R\x7FF\x07}U3\x07c\xF1bi\xFDu\xE5v\x16c\xF4\x19-'\x91t|\x01\x89\xB1j\xD3\x1D\xB0}\xB4\x91\x01`@Q\x80\x91\x03\x90\xA2PPPP\x80a\x15K\x90aW\x9AV[\x90Pa\x11/V[PPPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x99` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x81\x01T\x90\x91\x83\x01\x90`\xFF\x16`\x02\x81\x11\x15a\x15\xB5Wa\x15\xB5aM\xEBV[`\x02\x81\x11\x15a\x15\xC6Wa\x15\xC6aM\xEBV[\x90RP\x92\x91PPV[`\0T`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01Rb\x01\0\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\x1CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16@\x91\x90aXzV[a\x16\\W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xC5\x90aX\x9CV[`\0\x19`\x01\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[a\x16\xA3a%dV[\x81a\x16\xAD\x81a)\x08V[a\x16\xB7\x83\x83a)\x86V[PPPV[`\x9C\x81\x81T\x81\x10a\x16\xCCW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x81V[a\x16\xEEa*3V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x9F` \x90\x81R`@\x80\x83 B\x90U`\x99\x82R\x80\x83 \x80T\x82Q`\x1F\x87\x01\x85\x90\x04\x85\x02\x81\x01\x85\x01\x90\x93R\x85\x83R\x90\x93\x90\x92\x90\x91a\x17[\x91\x87\x90\x87\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`\x96T`\xFF\x16\x91Pa(w\x90PV[\x90P`\0a\x17h\x83a#@V[\x90P`\x01\x80\x85\x01T`\xFF\x16`\x02\x81\x11\x15a\x17\x84Wa\x17\x84aM\xEBV[\x14\x80\x15a\x17\x99WP`\x01`\x01`\xC0\x1B\x03\x82\x16\x15\x15[\x80\x15a\x17\xB7WPa\x17\xB7`\x01`\x01`\xC0\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x16\x14\x90V[\x15a\x15RWa\x15R\x87\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa*\xB3\x92PPPV[a\x18\x04a%dV[a\x18\x0E`\0a/%V[V[`\0a\x18P\x7FM@N2v\xE7\xAC!c\xD8\xEEGj\xFAjA\xD1\xF6\x8F\xB7\x1F-\x8BeF\xB2NU\xCE\x01\xB7*\x87\x87\x87\x87\x87`@Q` \x01a\x10+\x96\x95\x94\x93\x92\x91\x90aY\x8CV[\x96\x95PPPPPPV[`\0a\x0F\x18\x82a#@V[`\0a\x18y`dT`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x90P\x90V[`\x01\x80T`\0\x91\x90\x81\x16\x03a\x18\xA5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xC5\x90aW7V[\x83\x89\x14a\x19(W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FRegistryCoordinator.registerOper\x90\x82\x01R\x7FatorWithChurn: input length mism`d\x82\x01Rc\x0C.\x8Cm`\xE3\x1B`\x84\x82\x01R`\xA4\x01a\x08\xC5V[`\0a\x1943\x88a/wV[\x90Pa\x19\x943\x82\x88\x88\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x19\x89Wa\x19z`@\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aZ\x11V[\x81R` \x01\x90`\x01\x01\x90a\x19]V[PPPPP\x87a0\xACV[`\0a\x19\xDB3\x83\x8E\x8E\x8E\x8E\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8C\x92Pa29\x91PPV[\x90P`\0[\x8B\x81\x10\x15a\x1B\xA6W`\0`\x97`\0\x8F\x8F\x85\x81\x81\x10a\x1A\0Wa\x1A\0aWnV[\x91\x90\x91\x015`\xF8\x1C\x82RP` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0 \x81Q``\x81\x01\x83R\x90Tc\xFF\xFF\xFF\xFF\x81\x16\x80\x83Ra\xFF\xFF`\x01` \x1B\x83\x04\x81\x16\x95\x84\x01\x95\x90\x95R`\x01`0\x1B\x90\x91\x04\x90\x93\x16\x91\x81\x01\x91\x90\x91R\x84Q\x80Q\x91\x93P\x90\x84\x90\x81\x10a\x1AmWa\x1AmaWnV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\x1B\x93Wa\x1B\x0E\x8E\x8E\x84\x81\x81\x10a\x1A\x96Wa\x1A\x96aWnV[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x84`@\x01Q\x84\x81Q\x81\x10a\x1A\xB9Wa\x1A\xB9aWnV[` \x02` \x01\x01Q3\x86` \x01Q\x86\x81Q\x81\x10a\x1A\xD8Wa\x1A\xD8aWnV[` \x02` \x01\x01Q\x8D\x8D\x88\x81\x81\x10a\x1A\xF2Wa\x1A\xF2aWnV[\x90P`@\x02\x01\x806\x03\x81\x01\x90a\x1B\x08\x91\x90aZ\x11V[\x86a7\xFAV[a\x1B\x93\x89\x89\x84\x81\x81\x10a\x1B#Wa\x1B#aWnV[\x90P`@\x02\x01` \x01` \x81\x01\x90a\x1B;\x91\x90aL\xBDV[\x8F\x8F\x85\x90\x86`\x01a\x1BL\x91\x90aYJV[\x92a\x1BY\x93\x92\x91\x90aYbV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa*\xB3\x92PPPV[P\x80a\x1B\x9E\x81aW\x9AV[\x91PPa\x19\xE0V[PPPPPPPPPPPPPV[`\x01\x80T`\0\x91\x90\x81\x16\x03a\x1B\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xC5\x90aW7V[`\0a\x1B\xE83\x85a/wV[\x90P`\0a\x1C13\x83\x8B\x8B\x8B\x8B\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8C\x92Pa29\x91PPV[Q\x90P`\0[\x88\x81\x10\x15a\x1D,W`\0\x8A\x8A\x83\x81\x81\x10a\x1CSWa\x1CSaWnV[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x97` R`@\x90 T\x85Q\x91\x93Pc\xFF\xFF\xFF\xFF\x16\x91P\x84\x90\x84\x90\x81\x10a\x1C\x89Wa\x1C\x89aWnV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\x1D\x19W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FRegistryCoordinator.registerOper\x90\x82\x01R\x7Fator: operator count exceeds max`d\x82\x01Rcimum`\xE0\x1B`\x84\x82\x01R`\xA4\x01a\x08\xC5V[P\x80a\x1D$\x81aW\x9AV[\x91PPa\x1C7V[PPPPPPPPPPV[```\0\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1DUWa\x1DUaKaV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1D~W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\x1D\xE9Wa\x1D\xB0\x85\x85\x83\x81Q\x81\x10a\x1D\xA3Wa\x1D\xA3aWnV[` \x02` \x01\x01Qa:\xCEV[\x82\x82\x81Q\x81\x10a\x1D\xC2Wa\x1D\xC2aWnV[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x1D\xE1\x81aW\x9AV[\x91PPa\x1D\x84V[P\x93\x92PPPV[`\x01\x80T`\x02\x90\x81\x16\x03a\x1E\x17W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xC5\x90aW7V[a\x16\xB73\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa*\xB3\x92PPPV[a\x1E_a%dV[a\x16\xB7\x83\x83\x83a<\nV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x1E\x8AWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x1E\xA4WP0;\x15\x80\x15a\x1E\xA4WP`\0T`\xFF\x16`\x01\x14[a\x1F\x07W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x08\xC5V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x1F*W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[\x82Q\x84Q\x14\x80\x15a\x1F<WP\x81Q\x83Q\x14[a\x1F\xA6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FRegistryCoordinator.initialize: `D\x82\x01Rt\r-\xCE\x0E\xAE\x84\r\x8C\xAD\xCC\xEE\x8D\x04\r\xAD.m\xAC.\x8Cm`[\x1B`d\x82\x01R`\x84\x01a\x08\xC5V[a\x1F\xAF\x89a/%V[a\x1F\xB9\x86\x86a>!V[a\x1F\xC2\x88a&\xC8V[a\x1F\xCB\x87a'1V[`\x9C\x80T`\x01\x81\x81\x01\x83U`\0\x83\x81R\x7F\xAF\x85\xB9\x07\x1D\xFA\xFE\xAC\x14\t\xD3\xF1\xD1\x9B\xAF\xC9\xBC|7\x97L\xDE\x8D\xF0\xEEah\xF0\x08nS\x9C\x92\x83\x01\x80T`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U\x85T\x80\x85\x01\x87U\x85\x01\x80T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x16\x90\x83\x16\x17\x90U\x85T\x93\x84\x01\x90\x95U\x91\x90\x92\x01\x80T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x16\x91\x90\x93\x16\x17\x90\x91U[\x84Q\x81\x10\x15a!\x1CWa!\n\x85\x82\x81Q\x81\x10a \xC9Wa \xC9aWnV[` \x02` \x01\x01Q\x85\x83\x81Q\x81\x10a \xE3Wa \xE3aWnV[` \x02` \x01\x01Q\x85\x84\x81Q\x81\x10a \xFDWa \xFDaWnV[` \x02` \x01\x01Qa<\nV[\x80a!\x14\x81aW\x9AV[\x91PPa \xABV[P\x80\x15a!cW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPPPV[a!va%dV[`\x01`\x01`\xA0\x1B\x03\x81\x16a!\xDBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x08\xC5V[a\rM\x81a/%V[`\0`\x02\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"[\x91\x90aX\x13V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\"\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xC5\x90aX0V[`\x01T\x19\x81\x19`\x01T\x19\x16\x14a#\tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08\xC5V[`\x01\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x0C\x85V[`\0\x81\x81R`\x98` R`@\x81 T\x80\x82\x03a#_WP`\0\x92\x91PPV[`\0\x83\x81R`\x98` R`@\x90 a#x`\x01\x83aZ-V[\x81T\x81\x10a#\x88Wa#\x88aWnV[`\0\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01`\xC0\x1B\x03\x16\x93\x92PPPV[```\0\x80a#\xB9\x84a?\x11V[a\xFF\xFF\x16`\x01`\x01`@\x1B\x03\x81\x11\x15a#\xD4Wa#\xD4aKaV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a#\xFEW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x80[\x82Q\x82\x10\x80\x15a$\x16WPa\x01\0\x81\x10[\x15a$mW`\x01\x81\x1B\x93P\x85\x84\x16\x15a$]W\x80`\xF8\x1B\x83\x83\x81Q\x81\x10a$?Wa$?aWnV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x81`\x01\x01\x91P[a$f\x81aW\x9AV[\x90Pa$\x05V[P\x90\x94\x93PPPPV[`\x01\x82` \x01Q`\x02\x81\x11\x15a$\x8FWa$\x8FaM\xEBV[\x14a$\x99WPPPV[\x81Q`@Qc3V\x7F\x7F`\xE1\x1B\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cf\xAC\xFE\xFE\x90a$\xEE\x90\x88\x90\x86\x90\x88\x90`\x04\x01aZDV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a%\rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%1\x91\x90aZtV[\x90P`\x01`\x01`\xC0\x1B\x03\x81\x16\x15a%]Wa%]\x85a%X\x83`\x01`\x01`\xC0\x1B\x03\x16a#\xABV[a*\xB3V[PPPPPV[3a%ma\x18eV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x18\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x08\xC5V[`\x01`\x01`\xA0\x1B\x03\x81\x16a&QW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x08\xC5V[`\0T`@\x80Q`\x01`\x01`\xA0\x1B\x03b\x01\0\0\x90\x93\x04\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`\0\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16b\x01\0\0\x02b\x01\0\0`\x01`\xB0\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x9DT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F1TW\xD8\xA8\xFE`\xF0J\xF1|\x16\xE2\xF5\xA5\xE1\xDBa+1d\x8EX\x03\x03`u\x9E\xF8\xF3R\x8C\x91\x01`@Q\x80\x91\x03\x90\xA1`\x9D\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x9ET`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F\x8F0\xAB\t\xF4:l\x15}\x7F\xCE~\n\x13\xC0\x03\x04,\x1C\x95\xE8\xA7.z\x14j!\xC0\xCA\xA2M\xC9\x91\x01`@Q\x80\x91\x03\x90\xA1`\x9E\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0a\x0F\x18a'\xA7a?<V[\x83`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x83\x90R`B\x81\x01\x82\x90R`\0\x90`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0\x80\x80a(\x18`\0\x80Q` a_m\x839\x81Q\x91R\x86aZ\xB3V[\x90P[a($\x81a@cV[\x90\x93P\x91P`\0\x80Q` a_m\x839\x81Q\x91R\x82\x83\t\x83\x03a(]W`@\x80Q\x80\x82\x01\x90\x91R\x90\x81R` \x81\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80Q` a_m\x839\x81Q\x91R`\x01\x82\x08\x90Pa(\x1BV[`\0\x80a(\x83\x84a@\xE5V[\x90P\x80\x83`\xFF\x16`\x01\x90\x1B\x11a)\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`?`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: bitmap exceeds max value\0`d\x82\x01R`\x84\x01a\x08\xC5V[\x93\x92PPPV[`\x96T`\xFF\x90\x81\x16\x90\x82\x16\x10a\rMW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FRegistryCoordinator.quorumExists`D\x82\x01R\x7F: quorum does not exist\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08\xC5V[`\xFF\x82\x16`\0\x81\x81R`\x97` \x90\x81R`@\x91\x82\x90 \x84Q\x81T\x86\x84\x01\x80Q\x88\x87\x01\x80Qc\xFF\xFF\xFF\xFF\x90\x95\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x94\x16\x84\x17`\x01` \x1Ba\xFF\xFF\x93\x84\x16\x02\x17g\xFF\xFF\0\0\0\0\0\0\x19\x16`\x01`0\x1B\x95\x83\x16\x95\x90\x95\x02\x94\x90\x94\x17\x90\x94U\x85Q\x91\x82RQ\x83\x16\x93\x81\x01\x93\x90\x93RQ\x16\x91\x81\x01\x91\x90\x91R\x7F>\xE6\xFE\x8DTa\x02D\xC3\xE9\xD3\xC0f\xAEJ\xEE\x99x\x84\xAA(\xF1\x06\x16\xAE\x82\x19%@\x13\x18\xAC\x90``\x01`@Q\x80\x91\x03\x90\xA2PPV[`\x9ET`\x01`\x01`\xA0\x1B\x03\x163\x14a\x18\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FRegistryCoordinator.onlyEjector:`D\x82\x01R\x7F caller is not the ejector\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08\xC5V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x99` R`@\x90 \x80T`\x01\x80\x83\x01T`\xFF\x16`\x02\x81\x11\x15a*\xE7Wa*\xE7aM\xEBV[\x14a+fW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FRegistryCoordinator._deregisterO`D\x82\x01R\x7Fperator: operator is not registe`d\x82\x01Rb\x1C\x99Y`\xEA\x1B`\x84\x82\x01R`\xA4\x01a\x08\xC5V[`\x96T`\0\x90a+z\x90\x85\x90`\xFF\x16a(wV[\x90P`\0a+\x87\x83a#@V[\x90P`\x01`\x01`\xC0\x1B\x03\x82\x16a,\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`;`$\x82\x01R\x7FRegistryCoordinator._deregisterO`D\x82\x01R\x7Fperator: bitmap cannot be 0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08\xC5V[a,\x1C`\x01`\x01`\xC0\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x16\x14\x90V[a,\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`Y`$\x82\x01R\x7FRegistryCoordinator._deregisterO`D\x82\x01R\x7Fperator: operator is not registe`d\x82\x01R\x7Fred for specified quorums\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x08\xC5V[`\x01`\x01`\xC0\x1B\x03\x82\x81\x16\x19\x82\x16\x16a,\xCD\x84\x82aBuV[`\x01`\x01`\xC0\x1B\x03\x81\x16a-\x9CW`\x01\x85\x01\x80T`\xFF\x19\x16`\x02\x17\x90U`@QcQ\xB2zm`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA3d\xF4\xDA\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a-MW`\0\x80\xFD[PZ\xF1\x15\x80\x15a-aW=`\0\x80>=`\0\xFD[PP`@Q\x86\x92P`\x01`\x01`\xA0\x1B\x03\x8A\x16\x91P\x7F9o\xDC\xB1\x80\xCB\x0F\xEA&\x92\x81\x13\xFB\x0F\xD1\xC3T\x98c\xF9\xCDV>j\x18O\x1DW\x81\x16\xC8\xE4\x90`\0\x90\xA3[`@Qc\xF4\xE2O\xE5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xF4\xE2O\xE5\x90a-\xEA\x90\x8A\x90\x8A\x90`\x04\x01aZ\xC7V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a.\x04W`\0\x80\xFD[PZ\xF1\x15\x80\x15a.\x18W=`\0\x80>=`\0\xFD[PP`@Qc\xBD)\xB8\xCD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92Pc\xBD)\xB8\xCD\x91Pa.j\x90\x87\x90\x8A\x90`\x04\x01aZ\xEBV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a.\x84W`\0\x80\xFD[PZ\xF1\x15\x80\x15a.\x98W=`\0\x80>=`\0\xFD[PP`@Qc\xBD)\xB8\xCD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92Pc\xBD)\xB8\xCD\x91Pa.\xEA\x90\x87\x90\x8A\x90`\x04\x01aZ\xEBV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a/\x04W`\0\x80\xFD[PZ\xF1\x15\x80\x15a/\x18W=`\0\x80>=`\0\xFD[PPPPPPPPPPPV[`d\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\x13T*N\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a/\xE2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a0\x06\x91\x90a[\x04V[\x90P`\0\x81\x90\x03a\x0F\x18W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xBFy\xCEX\x84\x84a0K\x87a\x0F\xCCV[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a0i\x93\x92\x91\x90a[\x1DV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a0\x88W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)\x01\x91\x90a[\x04V[` \x80\x82\x01Q`\0\x90\x81R`\x9A\x90\x91R`@\x90 T`\xFF\x16\x15a1RW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`R`$\x82\x01R\x7FRegistryCoordinator._verifyChurn`D\x82\x01R\x7FApproverSignature: churnApprover`d\x82\x01Rq\x08\x1C\xD8[\x1D\x08\x18[\x1C\x99XY\x1EH\x1D\\\xD9Y`r\x1B`\x84\x82\x01R`\xA4\x01a\x08\xC5V[B\x81`@\x01Q\x10\x15a1\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`R`$\x82\x01R\x7FRegistryCoordinator._verifyChurn`D\x82\x01R\x7FApproverSignature: churnApprover`d\x82\x01Rq\x08\x1C\xDAY\xDB\x98]\x1D\\\x99H\x19^\x1C\x1A\\\x99Y`r\x1B`\x84\x82\x01R`\xA4\x01a\x08\xC5V[` \x80\x82\x01\x80Q`\0\x90\x81R`\x9A\x90\x92R`@\x91\x82\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U`\x9DT\x90Q\x91\x83\x01Qa\t\xAC\x92`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91a22\x91\x88\x91\x88\x91\x88\x91\x90a\x18\x10V[\x83QaD6V[a2]`@Q\x80``\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`\0a2\xA5\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`\x96T`\xFF\x16\x91Pa(w\x90PV[\x90P`\0a2\xB2\x88a#@V[\x90P`\x01`\x01`\xC0\x1B\x03\x82\x16a30W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FRegistryCoordinator._registerOpe`D\x82\x01R\x7Frator: bitmap cannot be 0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08\xC5V[\x80\x82\x16`\x01`\x01`\xC0\x1B\x03\x16\x15a3\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`h`$\x82\x01R\x7FRegistryCoordinator._registerOpe`D\x82\x01R\x7Frator: operator already register`d\x82\x01R\x7Fed for some quorums being regist`\x84\x82\x01Rg2\xB92\xB2\x1037\xB9`\xC1\x1B`\xA4\x82\x01R`\xC4\x01a\x08\xC5V[`\xA0T`\x01`\x01`\xA0\x1B\x03\x8A\x16`\0\x90\x81R`\x9F` R`@\x90 T`\x01`\x01`\xC0\x1B\x03\x83\x81\x16\x90\x85\x16\x17\x91B\x91a4\x1E\x91\x90aYJV[\x10a4\x9FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R\x7FRegistryCoordinator._registerOpe`D\x82\x01R\x7Frator: operator cannot reregiste`d\x82\x01Rd\x1C\x88\x1EY]`\xDA\x1B`\x84\x82\x01R`\xA4\x01a\x08\xC5V[a4\xA9\x89\x82aBuV[\x88\x7F\xEC)c\xAB!\xC1\xE5\x0E\x1EX*\xA5B\xAF.K\xF7\xBF8\xE6\xE1@<'\xB4.\x1C]nb\x1E\xAA\x87`@Qa4\xD9\x91\x90aX\0V[`@Q\x80\x91\x03\x90\xA2`\x01`\x01`\x01`\xA0\x1B\x03\x8B\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T`\xFF\x16`\x02\x81\x11\x15a5\x13Wa5\x13aM\xEBV[\x14a6,W`@\x80Q\x80\x82\x01\x82R\x8A\x81R`\x01` \x80\x83\x01\x82\x81R`\x01`\x01`\xA0\x1B\x03\x8F\x16`\0\x90\x81R`\x99\x90\x92R\x93\x90 \x82Q\x81U\x92Q\x83\x82\x01\x80T\x93\x94\x93\x91\x92\x90\x91`\xFF\x19\x16\x90\x83`\x02\x81\x11\x15a5nWa5naM\xEBV[\x02\x17\x90UPP`@Qc\x99&\xEE}`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91Pc\x99&\xEE}\x90a5\xC3\x90\x8D\x90\x89\x90`\x04\x01a[\x96V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a5\xDDW`\0\x80\xFD[PZ\xF1\x15\x80\x15a5\xF1W=`\0\x80>=`\0\xFD[PP`@Q\x8B\x92P`\x01`\x01`\xA0\x1B\x03\x8D\x16\x91P\x7F\xE8\xE6\x8C\xEF\x1C:v\x1E\xD7\xBE~\x84c\xA3u\xF2\x7F{\xC35\xE5\x18$\"<\xAC\xCEcn\xC5\xC3\xFE\x90`\0\x90\xA3[`@Qc\x1F\xD9<\xA9`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c?\xB2yR\x90a6|\x90\x8D\x90\x8C\x90\x8C\x90`\x04\x01a\\\nV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a6\x96W`\0\x80\xFD[PZ\xF1\x15\x80\x15a6\xAAW=`\0\x80>=`\0\xFD[PP`@Qc%PGw`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92Pc%PGw\x91Pa7\0\x90\x8D\x90\x8D\x90\x8D\x90\x8D\x90`\x04\x01a\\/V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a7\x1FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra7G\x91\x90\x81\x01\x90a\\\xBBV[`@\x80\x87\x01\x91\x90\x91R` \x86\x01\x91\x90\x91RQb\xBF\xF0M`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90b\xBF\xF0M\x90a7\xA4\x90\x8C\x90\x8C\x90\x8C\x90`\x04\x01a]\x1EV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a7\xC3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra7\xEB\x91\x90\x81\x01\x90a]8V[\x84RPPP\x96\x95PPPPPPV[` \x80\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`\0\x81\x81R`\x99\x90\x94R`@\x90\x93 T\x91\x92\x90\x87\x16\x03a8yW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R`\0\x80Q` a_M\x839\x81Q\x91R`D\x82\x01Rt97\x1D\x101\xB0\xB777\xBA\x101\xB4:\xB97\x109\xB2\xB63`Y\x1B`d\x82\x01R`\x84\x01a\x08\xC5V[\x87`\xFF\x16\x84`\0\x01Q`\xFF\x16\x14a8\xF6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R`\0\x80Q` a_M\x839\x81Q\x91R`D\x82\x01R\x7Frn: quorumNumber not the same as`d\x82\x01Rf\x08\x1C\xDAY\xDB\x99Y`\xCA\x1B`\x84\x82\x01R`\xA4\x01a\x08\xC5V[`@QcT\x01\xED'`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`\xFF\x89\x16`$\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cT\x01\xED'\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a9gW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a9\x8B\x91\x90a]\xD1V[\x90Pa9\x97\x81\x85aE\xF0V[`\x01`\x01``\x1B\x03\x16\x86`\x01`\x01``\x1B\x03\x16\x11a:*W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`V`$\x82\x01R`\0\x80Q` a_M\x839\x81Q\x91R`D\x82\x01R\x7Frn: incoming operator has insuff`d\x82\x01Ru4\xB1\xB4\xB2\xB7:\x109\xBA0\xB5\xB2\x9037\xB9\x101\xB4:\xB97`Q\x1B`\x84\x82\x01R`\xA4\x01a\x08\xC5V[a:4\x88\x85aF\x14V[`\x01`\x01``\x1B\x03\x16\x81`\x01`\x01``\x1B\x03\x16\x10a!cW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R`\0\x80Q` a_M\x839\x81Q\x91R`D\x82\x01R\x7Frn: cannot kick operator with mo`d\x82\x01R\x7Fre than kickBIPsOfTotalStake\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x08\xC5V[`\0\x81\x81R`\x98` R`@\x81 T\x81[\x81\x81\x10\x15a;`W`\x01a:\xF3\x82\x84aZ-V[a:\xFD\x91\x90aZ-V[\x92P\x84c\xFF\xFF\xFF\xFF\x16`\x98`\0\x86\x81R` \x01\x90\x81R` \x01`\0 \x84c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a;0Wa;0aWnV[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a;NWPPa\x0F\x18V[\x80a;X\x81aW\x9AV[\x91PPa:\xDFV[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`l`$\x82\x01R\x7FRegistryCoordinator.getQuorumBit`D\x82\x01R\x7FmapIndexAtBlockNumber: no bitmap`d\x82\x01R\x7F update found for operatorId at `\x84\x82\x01Rk167\xB1\xB5\x907:\xB6\xB12\xB9`\xA1\x1B`\xA4\x82\x01R`\xC4\x01a\x08\xC5V[`\x96T`\xFF\x16`\xC0\x81\x10a<~W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FRegistryCoordinator.createQuorum`D\x82\x01Rt\x0E\x88\x1BX^\x08\x1C][\xDC\x9D[\\\xC8\x1C\x99XX\xDA\x19Y`Z\x1B`d\x82\x01R`\x84\x01a\x08\xC5V[a<\x89\x81`\x01a]\xEEV[`\x96\x80T`\xFF\x19\x16`\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a<\xA8\x81\x86a)\x86V[`@Q`\x01b\x96\xB5\x89`\xE0\x1B\x03\x19\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xFFiJw\x90a<\xFB\x90\x84\x90\x88\x90\x88\x90`\x04\x01a^\x13V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a=\x15W`\0\x80\xFD[PZ\xF1\x15\x80\x15a=)W=`\0\x80>=`\0\xFD[PP`@Qc\x13l\xA0\xF9`\xE1\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x92Pc&\xD9A\xF2\x91P`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a=\x91W`\0\x80\xFD[PZ\xF1\x15\x80\x15a=\xA5W=`\0\x80>=`\0\xFD[PP`@Qc\x13l\xA0\xF9`\xE1\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x92Pc&\xD9A\xF2\x91P`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a>\rW`\0\x80\xFD[PZ\xF1\x15\x80\x15a!cW=`\0\x80>=`\0\xFD[`\0Tb\x01\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a>HWP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a>\xCAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x08\xC5V[`\x01\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a?\r\x82a%\xC3V[PPV[`\0\x80[\x82\x15a\x0F\x18Wa?&`\x01\x84aZ-V[\x90\x92\x16\x91\x80a?4\x81a^\x8CV[\x91PPa?\x15V[`\x000`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80\x15a?\x95WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14[\x15a?\xBFWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[P`@\x80Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x80\x83\x01\x91\x90\x91R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x84\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x83\x01RF`\x80\x83\x01R0`\xA0\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\xC0\x90\x92\x01\x90\x92R\x80Q\x91\x01 \x90V[`\0\x80\x80`\0\x80Q` a_m\x839\x81Q\x91R`\x03`\0\x80Q` a_m\x839\x81Q\x91R\x86`\0\x80Q` a_m\x839\x81Q\x91R\x88\x89\t\t\x08\x90P`\0a@\xD9\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\0\x80Q` a_m\x839\x81Q\x91RaF.V[\x91\x95\x91\x94P\x90\x92PPPV[`\0a\x01\0\x82Q\x11\x15aAnW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FBitmapUtils.orderedBytesArrayToB\x90\x82\x01R\x7Fitmap: orderedBytesArray is too `d\x82\x01Rclong`\xE0\x1B`\x84\x82\x01R`\xA4\x01a\x08\xC5V[\x81Q`\0\x03aA\x7FWP`\0\x91\x90PV[`\0\x80\x83`\0\x81Q\x81\x10aA\x95WaA\x95aWnV[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x81\x90\x1B\x92P[\x84Q\x81\x10\x15aBlW\x84\x81\x81Q\x81\x10aA\xC3WaA\xC3aWnV[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x1B\x91P\x82\x82\x11aBXW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: orderedBytesArray is not `d\x82\x01Rf\x1B\xDC\x99\x19\\\x99Y`\xCA\x1B`\x84\x82\x01R`\xA4\x01a\x08\xC5V[\x91\x81\x17\x91aBe\x81aW\x9AV[\x90PaA\xA8V[P\x90\x93\x92PPPV[`\0\x82\x81R`\x98` R`@\x81 T\x90\x81\x90\x03aC\x1DW`\0\x83\x81R`\x98` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x86\x81R`\x01`\x01`\xC0\x1B\x03\x80\x8A\x16\x95\x84\x01\x95\x86R\x84T`\x01\x81\x01\x86U\x94\x88R\x95\x90\x96 \x91Q\x91\x90\x92\x01\x80T\x95Q\x93Q\x90\x94\x16`\x01`@\x1B\x02`\x01`\x01`@\x1B\x03\x93\x83\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x96\x16\x91\x90\x92\x16\x17\x93\x90\x93\x17\x16\x91\x90\x91\x17\x90UPPPV[`\0\x83\x81R`\x98` R`@\x81 aC6`\x01\x84aZ-V[\x81T\x81\x10aCFWaCFaWnV[`\0\x91\x82R` \x90\x91 \x01\x80T\x90\x91Pc\xFF\xFF\xFF\xFFC\x81\x16\x91\x16\x03aC\x88W\x80T`\x01`\x01`@\x1B\x03\x16`\x01`@\x1B`\x01`\x01`\xC0\x1B\x03\x85\x16\x02\x17\x81Ua\t\xACV[\x80Tc\xFF\xFF\xFF\xFFC\x81\x16`\x01` \x1B\x81\x81\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x94\x16\x93\x90\x93\x17\x84U`\0\x87\x81R`\x98` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83R\x94\x85R\x84\x83\x01\x84\x81R`\x01`\x01`\xC0\x1B\x03\x80\x8C\x16\x93\x87\x01\x93\x84R\x82T`\x01\x81\x01\x84U\x92\x86R\x93\x90\x94 \x94Q\x94\x01\x80T\x93Q\x91Q\x90\x92\x16`\x01`@\x1B\x02`\x01`\x01`@\x1B\x03\x91\x86\x16\x90\x96\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x93\x90\x94\x16\x92\x90\x92\x17\x17\x91\x90\x91\x16\x91\x90\x91\x17\x90UPPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16;\x15aEPW`@Qc\x0B\x13]?`\xE1\x1B\x80\x82R\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\x16&\xBA~\x90aDv\x90\x86\x90\x86\x90`\x04\x01aZ\xEBV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aD\x93W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aD\xB7\x91\x90a^\xADV[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14a\x16\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`S`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: ERC1271 signature `d\x82\x01Rr\x1D\x99\\\x9AY\x9AX\xD8]\x1A[\xDB\x88\x19\x98Z[\x19Y`j\x1B`\x84\x82\x01R`\xA4\x01a\x08\xC5V[\x82`\x01`\x01`\xA0\x1B\x03\x16aEd\x83\x83aF\xD7V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x16\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: signature not from`d\x82\x01Rf\x109\xB4\xB3\xB72\xB9`\xC9\x1B`\x84\x82\x01R`\xA4\x01a\x08\xC5V[` \x81\x01Q`\0\x90a'\x10\x90aF\n\x90a\xFF\xFF\x16\x85a^\xD7V[a)\x01\x91\x90a_\x06V[`@\x81\x01Q`\0\x90a'\x10\x90aF\n\x90a\xFF\xFF\x16\x85a^\xD7V[`\0\x80aF9aJ=V[aFAaJ[V[` \x80\x82R\x81\x81\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x88\x90R`\x80\x82\x01\x87\x90R`\xA0\x82\x01\x86\x90R\x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82\x80aF~W\xFE[P\x82aF\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FBN254.expMod: call failure\0\0\0\0\0\0`D\x82\x01R`d\x01a\x08\xC5V[PQ\x95\x94PPPPPV[`\0\x80`\0aF\xE6\x85\x85aF\xF3V[\x91P\x91Pa\x1D\xE9\x81aGaV[`\0\x80\x82Q`A\x03aG)W` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1AaG\x1D\x87\x82\x85\x85aI\x17V[\x94P\x94PPPPaGZV[\x82Q`@\x03aGRW` \x83\x01Q`@\x84\x01QaGG\x86\x83\x83aJ\x04V[\x93P\x93PPPaGZV[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0\x81`\x04\x81\x11\x15aGuWaGuaM\xEBV[\x03aG}WPV[`\x01\x81`\x04\x81\x11\x15aG\x91WaG\x91aM\xEBV[\x03aG\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x08\xC5V[`\x02\x81`\x04\x81\x11\x15aG\xF2WaG\xF2aM\xEBV[\x03aH?W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x08\xC5V[`\x03\x81`\x04\x81\x11\x15aHSWaHSaM\xEBV[\x03aH\xABW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x08\xC5V[`\x04\x81`\x04\x81\x11\x15aH\xBFWaH\xBFaM\xEBV[\x03a\rMW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 'v' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x08\xC5V[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15aINWP`\0\x90P`\x03aI\xFBV[\x84`\xFF\x16`\x1B\x14\x15\x80\x15aIfWP\x84`\xFF\x16`\x1C\x14\x15[\x15aIwWP`\0\x90P`\x04aI\xFBV[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15aI\xCBW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16aI\xF4W`\0`\x01\x92P\x92PPaI\xFBV[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x80`\x01`\x01`\xFF\x1B\x03\x83\x16\x81aJ!`\xFF\x86\x90\x1C`\x1BaYJV[\x90PaJ/\x87\x82\x88\x85aI\x17V[\x93P\x93PPP\x93P\x93\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`\0\x80\x83`\x1F\x84\x01\x12aJ\x8BW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aJ\xA2W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15aGZW`\0\x80\xFD[`\0\x80` \x83\x85\x03\x12\x15aJ\xD0W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aJ\xE6W`\0\x80\xFD[aJ\xF2\x85\x82\x86\x01aJyV[\x90\x96\x90\x95P\x93PPPPV[`\0` \x82\x84\x03\x12\x15aK\x10W`\0\x80\xFD[P5\x91\x90PV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\rMW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15aK>W`\0\x80\xFD[\x835\x92P` \x84\x015aKP\x81aK\x17V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aK\x99WaK\x99aKaV[`@R\x90V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aK\x99WaK\x99aKaV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aK\xE9WaK\xE9aKaV[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x83\x11\x15aL\nWaL\naKaV[aL\x1D`\x1F\x84\x01`\x1F\x19\x16` \x01aK\xC1V[\x90P\x82\x81R\x83\x83\x83\x01\x11\x15aL1W`\0\x80\xFD[\x82\x82` \x83\x017`\0` \x84\x83\x01\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15aLZW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aLpW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13aL\x81W`\0\x80\xFD[aL\x90\x84\x825` \x84\x01aK\xF1V[\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\rMW`\0\x80\xFD[\x805aL\xB8\x81aL\x98V[\x91\x90PV[`\0` \x82\x84\x03\x12\x15aL\xCFW`\0\x80\xFD[\x815a)\x01\x81aL\x98V[`\0\x80`@\x83\x85\x03\x12\x15aL\xEDW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[\x805`\xFF\x81\x16\x81\x14aL\xB8W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aM\x1FW`\0\x80\xFD[a)\x01\x82aL\xFCV[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x81\x01a\x0F\x18V[`\0\x80\x83`\x1F\x84\x01\x12aMQW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aMhW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aGZW`\0\x80\xFD[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15aM\x96W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aM\xADW`\0\x80\xFD[aM\xB9\x88\x83\x89\x01aJyV[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15aM\xD2W`\0\x80\xFD[PaM\xDF\x87\x82\x88\x01aM?V[\x95\x98\x94\x97P\x95PPPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x03\x81\x10aN\x1FWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[\x81Q\x81R` \x80\x83\x01Q`@\x83\x01\x91aN>\x90\x84\x01\x82aN\x01V[P\x92\x91PPV[\x805a\xFF\xFF\x81\x16\x81\x14aL\xB8W`\0\x80\xFD[`\0``\x82\x84\x03\x12\x15aNiW`\0\x80\xFD[aNqaKwV[\x90P\x815aN~\x81aK\x17V[\x81RaN\x8C` \x83\x01aNEV[` \x82\x01RaN\x9D`@\x83\x01aNEV[`@\x82\x01R\x92\x91PPV[`\0\x80`\x80\x83\x85\x03\x12\x15aN\xBBW`\0\x80\xFD[aN\xC4\x83aL\xFCV[\x91PaN\xD3\x84` \x85\x01aNWV[\x90P\x92P\x92\x90PV[`\0\x80`\0`@\x84\x86\x03\x12\x15aN\xF1W`\0\x80\xFD[\x835aN\xFC\x81aL\x98V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aO\x17W`\0\x80\xFD[aO#\x86\x82\x87\x01aM?V[\x94\x97\x90\x96P\x93\x94PPPPV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15aOIWaOIaKaV[P`\x05\x1B` \x01\x90V[`\0`@\x82\x84\x03\x12\x15aOeW`\0\x80\xFD[aOmaK\x9FV[\x90PaOx\x82aL\xFCV[\x81R` \x82\x015aO\x88\x81aL\x98V[` \x82\x01R\x92\x91PPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15aO\xABW`\0\x80\xFD[\x855aO\xB6\x81aL\x98V[\x94P` \x86\x81\x015\x94P`@\x80\x88\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aO\xDAW`\0\x80\xFD[\x88\x01`\x1F\x81\x01\x8A\x13aO\xEBW`\0\x80\xFD[\x805aO\xFEaO\xF9\x82aO0V[aK\xC1V[\x81\x81R`\x06\x91\x90\x91\x1B\x82\x01\x84\x01\x90\x84\x81\x01\x90\x8C\x83\x11\x15aP\x1DW`\0\x80\xFD[\x92\x85\x01\x92[\x82\x84\x10\x15aPCWaP4\x8D\x85aOSV[\x82R\x92\x84\x01\x92\x90\x85\x01\x90aP\"V[\x99\x9C\x98\x9BP\x98\x99``\x81\x015\x99P`\x80\x015\x97\x96PPPPPPPV[`\0a\x01\0\x82\x84\x03\x12\x15aPsW`\0\x80\xFD[P\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12aP\x8BW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aP\xA2W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x06\x1B\x85\x01\x01\x11\x15aGZW`\0\x80\xFD[`\0``\x82\x84\x03\x12\x15aP\xCFW`\0\x80\xFD[aP\xD7aKwV[\x90P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aP\xEFW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13aQ\0W`\0\x80\xFD[aQ\x0F\x84\x825` \x84\x01aK\xF1V[\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\0\x80`\0a\x01\xA0\x8A\x8C\x03\x12\x15aQKW`\0\x80\xFD[\x895`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aQbW`\0\x80\xFD[aQn\x8D\x83\x8E\x01aM?V[\x90\x9BP\x99P` \x8C\x015\x91P\x80\x82\x11\x15aQ\x87W`\0\x80\xFD[aQ\x93\x8D\x83\x8E\x01aM?V[\x90\x99P\x97P\x87\x91PaQ\xA8\x8D`@\x8E\x01aP`V[\x96Pa\x01@\x8C\x015\x91P\x80\x82\x11\x15aQ\xBFW`\0\x80\xFD[aQ\xCB\x8D\x83\x8E\x01aPyV[\x90\x96P\x94Pa\x01`\x8C\x015\x91P\x80\x82\x11\x15aQ\xE5W`\0\x80\xFD[aQ\xF1\x8D\x83\x8E\x01aP\xBDV[\x93Pa\x01\x80\x8C\x015\x91P\x80\x82\x11\x15aR\x08W`\0\x80\xFD[PaR\x15\x8C\x82\x8D\x01aP\xBDV[\x91PP\x92\x95\x98P\x92\x95\x98P\x92\x95\x98V[`\0\x80`\0\x80`\0\x80a\x01`\x87\x89\x03\x12\x15aR?W`\0\x80\xFD[\x865`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aRVW`\0\x80\xFD[aRb\x8A\x83\x8B\x01aM?V[\x90\x98P\x96P` \x89\x015\x91P\x80\x82\x11\x15aR{W`\0\x80\xFD[aR\x87\x8A\x83\x8B\x01aM?V[\x90\x96P\x94P\x84\x91PaR\x9C\x8A`@\x8B\x01aP`V[\x93Pa\x01@\x89\x015\x91P\x80\x82\x11\x15aR\xB3W`\0\x80\xFD[PaR\xC0\x89\x82\x8A\x01aP\xBDV[\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0\x80`@\x83\x85\x03\x12\x15aR\xE0W`\0\x80\xFD[\x825aR\xEB\x81aK\x17V[\x91P` \x83\x81\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aS\x07W`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13aS\x18W`\0\x80\xFD[\x805aS&aO\xF9\x82aO0V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x88\x83\x11\x15aSEW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aScW\x835\x82R\x92\x84\x01\x92\x90\x84\x01\x90aSJV[\x80\x95PPPPPP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aS\xB0W\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aS\x8EV[P\x90\x96\x95PPPPPPV[`\0\x80` \x83\x85\x03\x12\x15aS\xCFW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aS\xE5W`\0\x80\xFD[aJ\xF2\x85\x82\x86\x01aM?V[`\x01`\x01``\x1B\x03\x81\x16\x81\x14a\rMW`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12aT\x17W`\0\x80\xFD[\x815` aT'aO\xF9\x83aO0V[\x82\x81R`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aTFW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aT\x97W`@\x81\x89\x03\x12\x15aTcW`\0\x80\x81\xFD[aTkaK\x9FV[\x815aTv\x81aL\x98V[\x81R\x81\x85\x015aT\x85\x81aS\xF1V[\x81\x86\x01R\x83R\x91\x83\x01\x91`@\x01aTJV[P\x96\x95PPPPPPV[`\0\x80`\0`\xA0\x84\x86\x03\x12\x15aT\xB7W`\0\x80\xFD[aT\xC1\x85\x85aNWV[\x92P``\x84\x015aT\xD1\x81aS\xF1V[\x91P`\x80\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aT\xECW`\0\x80\xFD[aT\xF8\x86\x82\x87\x01aT\x06V[\x91PP\x92P\x92P\x92V[`\0\x82`\x1F\x83\x01\x12aU\x13W`\0\x80\xFD[\x815` aU#aO\xF9\x83aO0V[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15aUBW`\0\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15aUeWaUX\x89\x82aNWV[\x84R\x92\x84\x01\x92\x81\x01aUFV[P\x90\x97\x96PPPPPPPV[`\0\x82`\x1F\x83\x01\x12aU\x83W`\0\x80\xFD[\x815` aU\x93aO\xF9\x83aO0V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aU\xB2W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aT\x97W\x805aU\xC9\x81aS\xF1V[\x83R\x91\x83\x01\x91\x83\x01aU\xB6V[`\0\x82`\x1F\x83\x01\x12aU\xE7W`\0\x80\xFD[\x815` aU\xF7aO\xF9\x83aO0V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aV\x16W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aT\x97W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aV9W`\0\x80\x81\xFD[aVG\x89\x86\x83\x8B\x01\x01aT\x06V[\x84RP\x91\x83\x01\x91\x83\x01aV\x1AV[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15aVrW`\0\x80\xFD[aV{\x89aL\xADV[\x97PaV\x89` \x8A\x01aL\xADV[\x96PaV\x97`@\x8A\x01aL\xADV[\x95PaV\xA5``\x8A\x01aL\xADV[\x94P`\x80\x89\x015\x93P`\xA0\x89\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aV\xC8W`\0\x80\xFD[aV\xD4\x8C\x83\x8D\x01aU\x02V[\x94P`\xC0\x8B\x015\x91P\x80\x82\x11\x15aV\xEAW`\0\x80\xFD[aV\xF6\x8C\x83\x8D\x01aUrV[\x93P`\xE0\x8B\x015\x91P\x80\x82\x11\x15aW\x0CW`\0\x80\xFD[PaW\x19\x8B\x82\x8C\x01aU\xD6V[\x91PP\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[` \x81\x01a\x0F\x18\x82\x84aN\x01V[` \x80\x82R`\x19\x90\x82\x01R\x7FPausable: index is paused\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01aW\xACWaW\xACaW\x84V[P`\x01\x01\x90V[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15aW\xD9W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01aW\xBDV[\x81\x81\x11\x15aW\xEBW`\0` \x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a)\x01` \x83\x01\x84aW\xB3V[`\0` \x82\x84\x03\x12\x15aX%W`\0\x80\xFD[\x81Qa)\x01\x81aL\x98V[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15aX\x8CW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a)\x01W`\0\x80\xFD[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aX\xFBW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aY\x15W`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15aGZW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aY?W`\0\x80\xFD[\x81Qa)\x01\x81aK\x17V[`\0\x82\x19\x82\x11\x15aY]WaY]aW\x84V[P\x01\x90V[`\0\x80\x85\x85\x11\x15aYrW`\0\x80\xFD[\x83\x86\x11\x15aY\x7FW`\0\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[`\0`\xC0\x82\x01\x88\x83R` `\x01\x80`\xA0\x1B\x03\x80\x8A\x16\x82\x86\x01R`@\x89\x81\x87\x01R`\xC0``\x87\x01R\x83\x89Q\x80\x86R`\xE0\x88\x01\x91P\x84\x8B\x01\x95P`\0[\x81\x81\x10\x15aY\xF1W\x86Q\x80Q`\xFF\x16\x84R\x86\x01Q\x85\x16\x86\x84\x01R\x95\x85\x01\x95\x91\x83\x01\x91`\x01\x01aY\xC7V[PP`\x80\x87\x01\x98\x90\x98RPPPP`\xA0\x90\x91\x01\x91\x90\x91RP\x94\x93PPPPV[`\0`@\x82\x84\x03\x12\x15aZ#W`\0\x80\xFD[a)\x01\x83\x83aOSV[`\0\x82\x82\x10\x15aZ?WaZ?aW\x84V[P\x03\x90V[`\x01\x80`\xA0\x1B\x03\x84\x16\x81R\x82` \x82\x01R```@\x82\x01R`\0aZk``\x83\x01\x84aW\xB3V[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15aZ\x86W`\0\x80\xFD[\x81Q`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x14a)\x01W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82aZ\xC2WaZ\xC2aZ\x9DV[P\x06\x90V[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R`\0\x90aL\x90\x90\x83\x01\x84aW\xB3V[\x82\x81R`@` \x82\x01R`\0aL\x90`@\x83\x01\x84aW\xB3V[`\0` \x82\x84\x03\x12\x15a[\x16W`\0\x80\xFD[PQ\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81Ra\x01`\x81\x01a[E` \x83\x01\x85\x805\x82R` \x90\x81\x015\x91\x01RV[a[_``\x83\x01`@\x86\x01\x805\x82R` \x90\x81\x015\x91\x01RV[`@`\x80\x85\x01`\xA0\x84\x017`@`\xC0\x85\x01`\xE0\x84\x017`\0a\x01 \x83\x01\x90\x81R\x83Q\x90R` \x90\x92\x01Qa\x01@\x90\x91\x01R\x92\x91PPV[`\x01\x80`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R`\0\x82Q```@\x84\x01Ra[\xC0`\xA0\x84\x01\x82aW\xB3V[\x90P` \x84\x01Q``\x84\x01R`@\x84\x01Q`\x80\x84\x01R\x80\x91PP\x93\x92PPPV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R`@` \x82\x01\x81\x90R`\0\x90aZk\x90\x83\x01\x84\x86a[\xE1V[`\x01\x80`\xA0\x1B\x03\x85\x16\x81R\x83` \x82\x01R```@\x82\x01R`\0a\x18P``\x83\x01\x84\x86a[\xE1V[`\0\x82`\x1F\x83\x01\x12a\\hW`\0\x80\xFD[\x81Q` a\\xaO\xF9\x83aO0V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\\\x97W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aT\x97W\x80Qa\\\xAE\x81aS\xF1V[\x83R\x91\x83\x01\x91\x83\x01a\\\x9BV[`\0\x80`@\x83\x85\x03\x12\x15a\\\xCEW`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\\\xE5W`\0\x80\xFD[a\\\xF1\x86\x83\x87\x01a\\WV[\x93P` \x85\x01Q\x91P\x80\x82\x11\x15a]\x07W`\0\x80\xFD[Pa]\x14\x85\x82\x86\x01a\\WV[\x91PP\x92P\x92\x90PV[\x83\x81R`@` \x82\x01R`\0aZk`@\x83\x01\x84\x86a[\xE1V[`\0` \x80\x83\x85\x03\x12\x15a]KW`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a]aW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a]rW`\0\x80\xFD[\x80Qa]\x80aO\xF9\x82aO0V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a]\x9FW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a]\xC6W\x83Qa]\xB7\x81aK\x17V[\x82R\x92\x84\x01\x92\x90\x84\x01\x90a]\xA4V[\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a]\xE3W`\0\x80\xFD[\x81Qa)\x01\x81aS\xF1V[`\0`\xFF\x82\x16`\xFF\x84\x16\x80`\xFF\x03\x82\x11\x15a^\x0BWa^\x0BaW\x84V[\x01\x93\x92PPPV[`\0``\x82\x01`\xFF\x86\x16\x83R` `\x01`\x01``\x1B\x03\x80\x87\x16\x82\x86\x01R`@``\x81\x87\x01R\x83\x87Q\x80\x86R`\x80\x88\x01\x91P\x84\x89\x01\x95P`\0[\x81\x81\x10\x15a^|W\x86Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x86\x01Q\x85\x16\x86\x84\x01R\x95\x85\x01\x95\x91\x83\x01\x91`\x01\x01a^LV[P\x90\x9A\x99PPPPPPPPPPV[`\0a\xFF\xFF\x80\x83\x16\x81\x81\x03a^\xA3Wa^\xA3aW\x84V[`\x01\x01\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a^\xBFW`\0\x80\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a)\x01W`\0\x80\xFD[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15a^\xFDWa^\xFDaW\x84V[\x02\x94\x93PPPPV[`\0`\x01`\x01``\x1B\x03\x80\x84\x16\x80a_ Wa_ aZ\x9DV[\x92\x16\x91\x90\x91\x04\x92\x91PPV\xFERegistryCoordinator.updateOperatRegistryCoordinator._validateChu0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 B)\xC6\xF0\x16\xBCq\xCF0?Y\xD7\xD3mf\\\xAA>\x1Cx~\x01h*=\t\xC2\x7F\xCCB\x9F\xBEdsolcC\0\x08\r\x003";
    /// The bytecode of the contract.
    pub static REGISTRYCOORDINATOR_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\xD5W`\x005`\xE0\x1C\x80c]\xF4YF\x11a\x01\x82W\x80c\x9F\xEA\xB8Y\x11a\0\xE9W\x80c\xD7-\x8D\xD6\x11a\0\xA2W\x80c\xE6W\x97\xAD\x11a\0|W\x80c\xE6W\x97\xAD\x14a\x07\x98W\x80c\xF2\xFD\xE3\x8B\x14a\x08;W\x80c\xFA\xBC\x1C\xBC\x14a\x08NW\x80c\xFD9\x10Z\x14a\x08aW`\0\x80\xFD[\x80c\xD7-\x8D\xD6\x14a\x07jW\x80c\xD7[L\x88\x14a\x07rW\x80c\xDD\x82\x83\xF3\x14a\x07\x85W`\0\x80\xFD[\x80c\x9F\xEA\xB8Y\x14a\x06\xCDW\x80c\xA5\x08W\xBF\x14a\x06\xF4W\x80c\xA9ox>\x14a\x07\x07W\x80c\xC3\x91B^\x14a\x07\x10W\x80c\xCA\r\xE8\x82\x14a\x070W\x80c\xCAO-\x97\x14a\x07WW`\0\x80\xFD[\x80c\x87\x1E\xF0I\x11a\x01;W\x80c\x87\x1E\xF0I\x14a\x06@W\x80c\x88o\x11\x95\x14a\x06SW\x80c\x8D\xA5\xCB[\x14a\x06lW\x80c\x9A\xA1e=\x14a\x06tW\x80c\x9B]\x17{\x14a\x06\x93W\x80c\x9E\x99#\xC2\x14a\x06\xA6W`\0\x80\xFD[\x80c]\xF4YF\x14a\x05\xB1W\x80ccG\xC9\0\x14a\x05\xD8W\x80ch0H5\x14a\x05\xEBW\x80cn;\x17\xDB\x14a\x06\x12W\x80cqP\x18\xA6\x14a\x06%W\x80c\x84\xCAR\x13\x14a\x06-W`\0\x80\xFD[\x80c$\x9A\x0CB\x11a\x02AW\x80c<*\x7FL\x11a\x01\xFAW\x80cY\\jg\x11a\x01\xD4W\x80cY\\jg\x14a\x05oW\x80cZ\xC8j\xB7\x14a\x05wW\x80c[\x0B\x82\x9F\x14a\x05\x96W\x80c\\\x97Z\xBB\x14a\x05\xA9W`\0\x80\xFD[\x80c<*\x7FL\x14a\x05\x1CW\x80cQ@\xA5H\x14a\x05<W\x80cXe\xC6\x0C\x14a\x05OW`\0\x80\xFD[\x80c$\x9A\x0CB\x14a\x04\x89W\x80c(\xF6\x1B1\x14a\x04\xA9W\x80c)k\xB0d\x14a\x04\xBCW\x80c)\xD1\xE0\xC3\x14a\x04\xCFW\x80c,\xDD\x1E\x86\x14a\x04\xE2W\x80c9\x98\xFD\xD3\x14a\x04\xF5W`\0\x80\xFD[\x80c\x10\xD6z/\x11a\x02\x93W\x80c\x10\xD6z/\x14a\x03\x9EW\x80c\x12^\x05\x84\x14a\x03\xB1W\x80c\x13T*N\x14a\x03\xD1W\x80c\x13d9\xDD\x14a\x03\xFAW\x80c\x14x\x85\x1F\x14a\x04\rW\x80c\x1E\xB8\x12\xDA\x14a\x04@W`\0\x80\xFD[\x80b\xCF*\xB5\x14a\x02\xDAW\x80c\x03\xFD4\x92\x14a\x02\xEFW\x80c\x04\xECcQ\x14a\x03\"W\x80c\x05C\x10\xE6\x14a\x03MW\x80c\x0C\xF4\xB7g\x14a\x03xW\x80c\r?!4\x14a\x03\x8BW[`\0\x80\xFD[a\x02\xEDa\x02\xE86`\x04aJ\xBDV[a\x08\x9DV[\0[a\x03\x0Fa\x02\xFD6`\x04aJ\xFEV[`\0\x90\x81R`\x98` R`@\x90 T\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x035a\x0306`\x04aK)V[a\t\xB2V[`@Q`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x81R` \x01a\x03\x19V[`\x9DTa\x03`\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03\x19V[a\x02\xEDa\x03\x866`\x04aLHV[a\x0B\xA8V[a\x02\xEDa\x03\x996`\x04aJ\xFEV[a\x0C\x90V[a\x02\xEDa\x03\xAC6`\x04aL\xBDV[a\x0C\x9DV[a\x03\x0Fa\x03\xBF6`\x04aL\xBDV[`\x9F` R`\0\x90\x81R`@\x90 T\x81V[a\x03\x0Fa\x03\xDF6`\x04aL\xBDV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x99` R`@\x90 T\x90V[a\x02\xEDa\x04\x086`\x04aJ\xFEV[a\rPV[a\x040a\x04\x1B6`\x04aJ\xFEV[`\x9A` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x03\x19V[a\x04Sa\x04N6`\x04aL\xDAV[a\x0E\x8DV[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x80\x85\x01Q\x90\x91\x16\x90\x82\x01R\x91\x81\x01Q`\x01`\x01`\xC0\x1B\x03\x16\x90\x82\x01R``\x01a\x03\x19V[a\x03\x0Fa\x04\x976`\x04aM\rV[`\x9B` R`\0\x90\x81R`@\x90 T\x81V[`\x9ETa\x03`\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03`a\x04\xCA6`\x04aJ\xFEV[a\x0F\x1EV[a\x02\xEDa\x04\xDD6`\x04aL\xBDV[a\x0F\xAAV[a\x02\xEDa\x04\xF06`\x04aL\xBDV[a\x0F\xBBV[a\x03`\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x05/a\x05*6`\x04aL\xBDV[a\x0F\xCCV[`@Qa\x03\x19\x91\x90aM(V[a\x02\xEDa\x05J6`\x04aM\x80V[a\x10KV[a\x05ba\x05]6`\x04aL\xBDV[a\x15[V[`@Qa\x03\x19\x91\x90aN#V[a\x02\xEDa\x15\xCFV[a\x040a\x05\x856`\x04aM\rV[`\x01\x80T`\xFF\x90\x92\x16\x1B\x90\x81\x16\x14\x90V[a\x02\xEDa\x05\xA46`\x04aN\xA8V[a\x16\x9BV[`\x01Ta\x03\x0FV[a\x03`\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03`a\x05\xE66`\x04aJ\xFEV[a\x16\xBCV[a\x03`\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xEDa\x06 6`\x04aN\xDCV[a\x16\xE6V[a\x02\xEDa\x17\xFCV[a\x03\x0Fa\x06;6`\x04aO\x93V[a\x18\x10V[a\x035a\x06N6`\x04aJ\xFEV[a\x18ZV[`\0Ta\x03`\x90b\x01\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03`a\x18eV[`\x96Ta\x06\x81\x90`\xFF\x16\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x03\x19V[a\x02\xEDa\x06\xA16`\x04aQ,V[a\x18~V[a\x03`\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\x0F\x7F+\xD8!$\x05\x7F\t\x13\xBC;w,\xE7\xB8>\x80W\xC1\xAD\x1F5\x10\xFC\x83w\x8B\xE2\x0F\x10\xEC]\xE6\x81V[a\x02\xEDa\x07\x026`\x04aR%V[a\x1B\xB5V[a\x03\x0F`\xA0T\x81V[a\x07#a\x07\x1E6`\x04aR\xCDV[a\x1D8V[`@Qa\x03\x19\x91\x90aSrV[a\x03\x0F\x7FM@N2v\xE7\xAC!c\xD8\xEEGj\xFAjA\xD1\xF6\x8F\xB7\x1F-\x8BeF\xB2NU\xCE\x01\xB7*\x81V[a\x02\xEDa\x07e6`\x04aS\xBCV[a\x1D\xF1V[`\x9CTa\x03\x0FV[a\x02\xEDa\x07\x806`\x04aT\xA2V[a\x1EWV[a\x02\xEDa\x07\x936`\x04aVUV[a\x1EjV[a\x08\x07a\x07\xA66`\x04aM\rV[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x92\x84\x01\x81\x90R`\xFF\x94\x90\x94\x16\x84R`\x97\x82R\x92\x82\x90 \x82Q\x93\x84\x01\x83RTc\xFF\xFF\xFF\xFF\x81\x16\x84Ra\xFF\xFF`\x01` \x1B\x82\x04\x81\x16\x92\x85\x01\x92\x90\x92R`\x01`0\x1B\x90\x04\x16\x90\x82\x01R\x90V[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x16\x81R` \x80\x84\x01Qa\xFF\xFF\x90\x81\x16\x91\x83\x01\x91\x90\x91R\x92\x82\x01Q\x90\x92\x16\x90\x82\x01R``\x01a\x03\x19V[a\x02\xEDa\x08I6`\x04aL\xBDV[a!nV[a\x02\xEDa\x08\\6`\x04aJ\xFEV[a!\xE4V[a\x08\x90a\x08o6`\x04aL\xBDV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T`\xFF\x16\x90V[`@Qa\x03\x19\x91\x90aW)V[`\x01T`\x02\x90`\x04\x90\x81\x16\x03a\x08\xCEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xC5\x90aW7V[`@Q\x80\x91\x03\x90\xFD[`\0[\x82\x81\x10\x15a\t\xACW`\0\x84\x84\x83\x81\x81\x10a\x08\xEDWa\x08\xEDaWnV[\x90P` \x02\x01` \x81\x01\x90a\t\x02\x91\x90aL\xBDV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x99` \x90\x81R`@\x80\x83 \x81Q\x80\x83\x01\x90\x92R\x80T\x82R`\x01\x81\x01T\x94\x95P\x92\x93\x90\x92\x91\x83\x01\x90`\xFF\x16`\x02\x81\x11\x15a\tMWa\tMaM\xEBV[`\x02\x81\x11\x15a\t^Wa\t^aM\xEBV[\x90RP\x80Q\x90\x91P`\0a\tq\x82a#@V[\x90P`\0a\t\x87\x82`\x01`\x01`\xC0\x1B\x03\x16a#\xABV[\x90Pa\t\x94\x85\x85\x83a$wV[PPPPP\x80\x80a\t\xA4\x90aW\x9AV[\x91PPa\x08\xD1V[PPPPV[`\0\x83\x81R`\x98` R`@\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\t\xD5Wa\t\xD5aWnV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x85R`\x01` \x1B\x83\x04\x82\x16\x95\x85\x01\x95\x90\x95R`\x01`@\x1B\x90\x91\x04`\x01`\x01`\xC0\x1B\x03\x16\x91\x83\x01\x91\x90\x91R\x90\x92P\x85\x16\x10\x15a\n\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`e`$\x82\x01R\x7FRegistryCoordinator.getQuorumBit`D\x82\x01R\x7FmapAtBlockNumberByIndex: quorumB`d\x82\x01R\x7FitmapUpdate is from after blockN`\x84\x82\x01Rd:\xB6\xB12\xB9`\xD9\x1B`\xA4\x82\x01R`\xC4\x01a\x08\xC5V[` \x81\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x80a\n\xF5WP\x80` \x01Qc\xFF\xFF\xFF\xFF\x16\x84c\xFF\xFF\xFF\xFF\x16\x10[a\x0B\x9CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`f`$\x82\x01R\x7FRegistryCoordinator.getQuorumBit`D\x82\x01R\x7FmapAtBlockNumberByIndex: quorumB`d\x82\x01R\x7FitmapUpdate is from before block`\x84\x82\x01Re':\xB6\xB12\xB9`\xD1\x1B`\xA4\x82\x01R`\xC4\x01a\x08\xC5V[`@\x01Q\x94\x93PPPPV[`\x013`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T`\xFF\x16`\x02\x81\x11\x15a\x0B\xD1Wa\x0B\xD1aM\xEBV[\x14a\x0CDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R\x7FRegistryCoordinator.updateSocket`D\x82\x01R\x7F: operator is not registered\0\0\0\0`d\x82\x01R`\x84\x01a\x08\xC5V[3`\0\x90\x81R`\x99` R`@\x90\x81\x90 T\x90Q\x7F\xEC)c\xAB!\xC1\xE5\x0E\x1EX*\xA5B\xAF.K\xF7\xBF8\xE6\xE1@<'\xB4.\x1C]nb\x1E\xAA\x90a\x0C\x85\x90\x84\x90aX\0V[`@Q\x80\x91\x03\x90\xA2PV[a\x0C\x98a%dV[`\xA0UV[`\0`\x02\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xF0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x14\x91\x90aX\x13V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\rDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xC5\x90aX0V[a\rM\x81a%\xC3V[PV[`\0T`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01Rb\x01\0\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\x9DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xC1\x91\x90aXzV[a\r\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xC5\x90aX\x9CV[`\x01T\x81\x81\x16\x14a\x0EVW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08\xC5V[`\x01\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01a\x0C\x85V[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R`\0\x83\x81R`\x98` R`@\x90 \x80T\x83\x90\x81\x10a\x0E\xCAWa\x0E\xCAaWnV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01`\xC0\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90P[\x92\x91PPV[`@Qc\x08\xF6b\x9D`\xE3\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cG\xB3\x14\xE8\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x86W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x18\x91\x90aX\x13V[a\x0F\xB2a%dV[a\rM\x81a&\xC8V[a\x0F\xC3a%dV[a\rM\x81a'1V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x0F\x18a\x10F\x7F+\xD8!$\x05\x7F\t\x13\xBC;w,\xE7\xB8>\x80W\xC1\xAD\x1F5\x10\xFC\x83w\x8B\xE2\x0F\x10\xEC]\xE6\x84`@Q` \x01a\x10+\x92\x91\x90\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a'\x9AV[a'\xE8V[`\x01T`\x02\x90`\x04\x90\x81\x16\x03a\x10sW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xC5\x90aW7V[`\0a\x10\xBB\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`\x96T`\xFF\x16\x91Pa(w\x90PV[\x90P\x84\x83\x14a\x11,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R`\0\x80Q` a_-\x839\x81Q\x91R`D\x82\x01R\x7ForsForQuorum: input length misma`d\x82\x01Rb\x0E\x8Cm`\xEB\x1B`\x84\x82\x01R`\xA4\x01a\x08\xC5V[`\0[\x83\x81\x10\x15a\x15RW`\0\x85\x85\x83\x81\x81\x10a\x11KWa\x11KaWnV[\x91\x90\x91\x015`\xF8\x1C\x91P6\x90P`\0\x89\x89\x85\x81\x81\x10a\x11lWa\x11laWnV[\x90P` \x02\x81\x01\x90a\x11~\x91\x90aX\xE4V[`@Qcy\xA0\x84\x91`\xE1\x1B\x81R`\xFF\x86\x16`\x04\x82\x01R\x91\x93P\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xF3A\t\"\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xEAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\x0E\x91\x90aY-V[c\xFF\xFF\xFF\xFF\x16\x81\x14a\x12\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`e`$\x82\x01R`\0\x80Q` a_-\x839\x81Q\x91R`D\x82\x01R\x7ForsForQuorum: number of updated `d\x82\x01R\x7Foperators does not match quorum `\x84\x82\x01Rd\x1D\x1B\xDD\x18[`\xDA\x1B`\xA4\x82\x01R`\xC4\x01a\x08\xC5V[`\0\x80[\x82\x81\x10\x15a\x14\xF1W`\0\x84\x84\x83\x81\x81\x10a\x12\xCAWa\x12\xCAaWnV[\x90P` \x02\x01` \x81\x01\x90a\x12\xDF\x91\x90aL\xBDV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x99` \x90\x81R`@\x80\x83 \x81Q\x80\x83\x01\x90\x92R\x80T\x82R`\x01\x81\x01T\x94\x95P\x92\x93\x90\x92\x91\x83\x01\x90`\xFF\x16`\x02\x81\x11\x15a\x13*Wa\x13*aM\xEBV[`\x02\x81\x11\x15a\x13;Wa\x13;aM\xEBV[\x90RP\x80Q\x90\x91P`\0a\x13N\x82a#@V[\x90P`\x01`\x01`\x01`\xC0\x1B\x03\x82\x16`\xFF\x8B\x16\x1C\x81\x16\x14a\x13\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R`\0\x80Q` a_-\x839\x81Q\x91R\x90\x82\x01R\x7ForsForQuorum: operator not in qu`d\x82\x01Rcorum`\xE0\x1B`\x84\x82\x01R`\xA4\x01a\x08\xC5V[\x85`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x11a\x14}W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`g`$\x82\x01R`\0\x80Q` a_-\x839\x81Q\x91R`D\x82\x01R\x7ForsForQuorum: operators array mu`d\x82\x01R\x7Fst be sorted in ascending addres`\x84\x82\x01Rf9\x907\xB922\xB9`\xC9\x1B`\xA4\x82\x01R`\xC4\x01a\x08\xC5V[Pa\x14\xDB\x83\x83\x8F\x8F\x8D\x90\x8E`\x01a\x14\x94\x91\x90aYJV[\x92a\x14\xA1\x93\x92\x91\x90aYbV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa$w\x92PPPV[P\x90\x92Pa\x14\xEA\x90P\x81aW\x9AV[\x90Pa\x12\xAEV[P`\xFF\x84\x16`\0\x81\x81R`\x9B` \x90\x81R`@\x91\x82\x90 C\x90\x81\x90U\x91Q\x91\x82R\x7FF\x07}U3\x07c\xF1bi\xFDu\xE5v\x16c\xF4\x19-'\x91t|\x01\x89\xB1j\xD3\x1D\xB0}\xB4\x91\x01`@Q\x80\x91\x03\x90\xA2PPPP\x80a\x15K\x90aW\x9AV[\x90Pa\x11/V[PPPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x99` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x81\x01T\x90\x91\x83\x01\x90`\xFF\x16`\x02\x81\x11\x15a\x15\xB5Wa\x15\xB5aM\xEBV[`\x02\x81\x11\x15a\x15\xC6Wa\x15\xC6aM\xEBV[\x90RP\x92\x91PPV[`\0T`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01Rb\x01\0\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\x1CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16@\x91\x90aXzV[a\x16\\W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xC5\x90aX\x9CV[`\0\x19`\x01\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[a\x16\xA3a%dV[\x81a\x16\xAD\x81a)\x08V[a\x16\xB7\x83\x83a)\x86V[PPPV[`\x9C\x81\x81T\x81\x10a\x16\xCCW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x81V[a\x16\xEEa*3V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x9F` \x90\x81R`@\x80\x83 B\x90U`\x99\x82R\x80\x83 \x80T\x82Q`\x1F\x87\x01\x85\x90\x04\x85\x02\x81\x01\x85\x01\x90\x93R\x85\x83R\x90\x93\x90\x92\x90\x91a\x17[\x91\x87\x90\x87\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`\x96T`\xFF\x16\x91Pa(w\x90PV[\x90P`\0a\x17h\x83a#@V[\x90P`\x01\x80\x85\x01T`\xFF\x16`\x02\x81\x11\x15a\x17\x84Wa\x17\x84aM\xEBV[\x14\x80\x15a\x17\x99WP`\x01`\x01`\xC0\x1B\x03\x82\x16\x15\x15[\x80\x15a\x17\xB7WPa\x17\xB7`\x01`\x01`\xC0\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x16\x14\x90V[\x15a\x15RWa\x15R\x87\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa*\xB3\x92PPPV[a\x18\x04a%dV[a\x18\x0E`\0a/%V[V[`\0a\x18P\x7FM@N2v\xE7\xAC!c\xD8\xEEGj\xFAjA\xD1\xF6\x8F\xB7\x1F-\x8BeF\xB2NU\xCE\x01\xB7*\x87\x87\x87\x87\x87`@Q` \x01a\x10+\x96\x95\x94\x93\x92\x91\x90aY\x8CV[\x96\x95PPPPPPV[`\0a\x0F\x18\x82a#@V[`\0a\x18y`dT`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x90P\x90V[`\x01\x80T`\0\x91\x90\x81\x16\x03a\x18\xA5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xC5\x90aW7V[\x83\x89\x14a\x19(W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FRegistryCoordinator.registerOper\x90\x82\x01R\x7FatorWithChurn: input length mism`d\x82\x01Rc\x0C.\x8Cm`\xE3\x1B`\x84\x82\x01R`\xA4\x01a\x08\xC5V[`\0a\x1943\x88a/wV[\x90Pa\x19\x943\x82\x88\x88\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x19\x89Wa\x19z`@\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aZ\x11V[\x81R` \x01\x90`\x01\x01\x90a\x19]V[PPPPP\x87a0\xACV[`\0a\x19\xDB3\x83\x8E\x8E\x8E\x8E\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8C\x92Pa29\x91PPV[\x90P`\0[\x8B\x81\x10\x15a\x1B\xA6W`\0`\x97`\0\x8F\x8F\x85\x81\x81\x10a\x1A\0Wa\x1A\0aWnV[\x91\x90\x91\x015`\xF8\x1C\x82RP` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0 \x81Q``\x81\x01\x83R\x90Tc\xFF\xFF\xFF\xFF\x81\x16\x80\x83Ra\xFF\xFF`\x01` \x1B\x83\x04\x81\x16\x95\x84\x01\x95\x90\x95R`\x01`0\x1B\x90\x91\x04\x90\x93\x16\x91\x81\x01\x91\x90\x91R\x84Q\x80Q\x91\x93P\x90\x84\x90\x81\x10a\x1AmWa\x1AmaWnV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\x1B\x93Wa\x1B\x0E\x8E\x8E\x84\x81\x81\x10a\x1A\x96Wa\x1A\x96aWnV[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x84`@\x01Q\x84\x81Q\x81\x10a\x1A\xB9Wa\x1A\xB9aWnV[` \x02` \x01\x01Q3\x86` \x01Q\x86\x81Q\x81\x10a\x1A\xD8Wa\x1A\xD8aWnV[` \x02` \x01\x01Q\x8D\x8D\x88\x81\x81\x10a\x1A\xF2Wa\x1A\xF2aWnV[\x90P`@\x02\x01\x806\x03\x81\x01\x90a\x1B\x08\x91\x90aZ\x11V[\x86a7\xFAV[a\x1B\x93\x89\x89\x84\x81\x81\x10a\x1B#Wa\x1B#aWnV[\x90P`@\x02\x01` \x01` \x81\x01\x90a\x1B;\x91\x90aL\xBDV[\x8F\x8F\x85\x90\x86`\x01a\x1BL\x91\x90aYJV[\x92a\x1BY\x93\x92\x91\x90aYbV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa*\xB3\x92PPPV[P\x80a\x1B\x9E\x81aW\x9AV[\x91PPa\x19\xE0V[PPPPPPPPPPPPPV[`\x01\x80T`\0\x91\x90\x81\x16\x03a\x1B\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xC5\x90aW7V[`\0a\x1B\xE83\x85a/wV[\x90P`\0a\x1C13\x83\x8B\x8B\x8B\x8B\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8C\x92Pa29\x91PPV[Q\x90P`\0[\x88\x81\x10\x15a\x1D,W`\0\x8A\x8A\x83\x81\x81\x10a\x1CSWa\x1CSaWnV[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x97` R`@\x90 T\x85Q\x91\x93Pc\xFF\xFF\xFF\xFF\x16\x91P\x84\x90\x84\x90\x81\x10a\x1C\x89Wa\x1C\x89aWnV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\x1D\x19W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FRegistryCoordinator.registerOper\x90\x82\x01R\x7Fator: operator count exceeds max`d\x82\x01Rcimum`\xE0\x1B`\x84\x82\x01R`\xA4\x01a\x08\xC5V[P\x80a\x1D$\x81aW\x9AV[\x91PPa\x1C7V[PPPPPPPPPPV[```\0\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1DUWa\x1DUaKaV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1D~W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\x1D\xE9Wa\x1D\xB0\x85\x85\x83\x81Q\x81\x10a\x1D\xA3Wa\x1D\xA3aWnV[` \x02` \x01\x01Qa:\xCEV[\x82\x82\x81Q\x81\x10a\x1D\xC2Wa\x1D\xC2aWnV[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x1D\xE1\x81aW\x9AV[\x91PPa\x1D\x84V[P\x93\x92PPPV[`\x01\x80T`\x02\x90\x81\x16\x03a\x1E\x17W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xC5\x90aW7V[a\x16\xB73\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa*\xB3\x92PPPV[a\x1E_a%dV[a\x16\xB7\x83\x83\x83a<\nV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x1E\x8AWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x1E\xA4WP0;\x15\x80\x15a\x1E\xA4WP`\0T`\xFF\x16`\x01\x14[a\x1F\x07W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x08\xC5V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x1F*W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[\x82Q\x84Q\x14\x80\x15a\x1F<WP\x81Q\x83Q\x14[a\x1F\xA6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FRegistryCoordinator.initialize: `D\x82\x01Rt\r-\xCE\x0E\xAE\x84\r\x8C\xAD\xCC\xEE\x8D\x04\r\xAD.m\xAC.\x8Cm`[\x1B`d\x82\x01R`\x84\x01a\x08\xC5V[a\x1F\xAF\x89a/%V[a\x1F\xB9\x86\x86a>!V[a\x1F\xC2\x88a&\xC8V[a\x1F\xCB\x87a'1V[`\x9C\x80T`\x01\x81\x81\x01\x83U`\0\x83\x81R\x7F\xAF\x85\xB9\x07\x1D\xFA\xFE\xAC\x14\t\xD3\xF1\xD1\x9B\xAF\xC9\xBC|7\x97L\xDE\x8D\xF0\xEEah\xF0\x08nS\x9C\x92\x83\x01\x80T`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U\x85T\x80\x85\x01\x87U\x85\x01\x80T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x16\x90\x83\x16\x17\x90U\x85T\x93\x84\x01\x90\x95U\x91\x90\x92\x01\x80T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x16\x91\x90\x93\x16\x17\x90\x91U[\x84Q\x81\x10\x15a!\x1CWa!\n\x85\x82\x81Q\x81\x10a \xC9Wa \xC9aWnV[` \x02` \x01\x01Q\x85\x83\x81Q\x81\x10a \xE3Wa \xE3aWnV[` \x02` \x01\x01Q\x85\x84\x81Q\x81\x10a \xFDWa \xFDaWnV[` \x02` \x01\x01Qa<\nV[\x80a!\x14\x81aW\x9AV[\x91PPa \xABV[P\x80\x15a!cW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPPPV[a!va%dV[`\x01`\x01`\xA0\x1B\x03\x81\x16a!\xDBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x08\xC5V[a\rM\x81a/%V[`\0`\x02\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"[\x91\x90aX\x13V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\"\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xC5\x90aX0V[`\x01T\x19\x81\x19`\x01T\x19\x16\x14a#\tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08\xC5V[`\x01\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x0C\x85V[`\0\x81\x81R`\x98` R`@\x81 T\x80\x82\x03a#_WP`\0\x92\x91PPV[`\0\x83\x81R`\x98` R`@\x90 a#x`\x01\x83aZ-V[\x81T\x81\x10a#\x88Wa#\x88aWnV[`\0\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01`\xC0\x1B\x03\x16\x93\x92PPPV[```\0\x80a#\xB9\x84a?\x11V[a\xFF\xFF\x16`\x01`\x01`@\x1B\x03\x81\x11\x15a#\xD4Wa#\xD4aKaV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a#\xFEW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x80[\x82Q\x82\x10\x80\x15a$\x16WPa\x01\0\x81\x10[\x15a$mW`\x01\x81\x1B\x93P\x85\x84\x16\x15a$]W\x80`\xF8\x1B\x83\x83\x81Q\x81\x10a$?Wa$?aWnV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x81`\x01\x01\x91P[a$f\x81aW\x9AV[\x90Pa$\x05V[P\x90\x94\x93PPPPV[`\x01\x82` \x01Q`\x02\x81\x11\x15a$\x8FWa$\x8FaM\xEBV[\x14a$\x99WPPPV[\x81Q`@Qc3V\x7F\x7F`\xE1\x1B\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cf\xAC\xFE\xFE\x90a$\xEE\x90\x88\x90\x86\x90\x88\x90`\x04\x01aZDV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a%\rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%1\x91\x90aZtV[\x90P`\x01`\x01`\xC0\x1B\x03\x81\x16\x15a%]Wa%]\x85a%X\x83`\x01`\x01`\xC0\x1B\x03\x16a#\xABV[a*\xB3V[PPPPPV[3a%ma\x18eV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x18\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x08\xC5V[`\x01`\x01`\xA0\x1B\x03\x81\x16a&QW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x08\xC5V[`\0T`@\x80Q`\x01`\x01`\xA0\x1B\x03b\x01\0\0\x90\x93\x04\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`\0\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16b\x01\0\0\x02b\x01\0\0`\x01`\xB0\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x9DT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F1TW\xD8\xA8\xFE`\xF0J\xF1|\x16\xE2\xF5\xA5\xE1\xDBa+1d\x8EX\x03\x03`u\x9E\xF8\xF3R\x8C\x91\x01`@Q\x80\x91\x03\x90\xA1`\x9D\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x9ET`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F\x8F0\xAB\t\xF4:l\x15}\x7F\xCE~\n\x13\xC0\x03\x04,\x1C\x95\xE8\xA7.z\x14j!\xC0\xCA\xA2M\xC9\x91\x01`@Q\x80\x91\x03\x90\xA1`\x9E\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0a\x0F\x18a'\xA7a?<V[\x83`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x83\x90R`B\x81\x01\x82\x90R`\0\x90`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0\x80\x80a(\x18`\0\x80Q` a_m\x839\x81Q\x91R\x86aZ\xB3V[\x90P[a($\x81a@cV[\x90\x93P\x91P`\0\x80Q` a_m\x839\x81Q\x91R\x82\x83\t\x83\x03a(]W`@\x80Q\x80\x82\x01\x90\x91R\x90\x81R` \x81\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80Q` a_m\x839\x81Q\x91R`\x01\x82\x08\x90Pa(\x1BV[`\0\x80a(\x83\x84a@\xE5V[\x90P\x80\x83`\xFF\x16`\x01\x90\x1B\x11a)\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`?`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: bitmap exceeds max value\0`d\x82\x01R`\x84\x01a\x08\xC5V[\x93\x92PPPV[`\x96T`\xFF\x90\x81\x16\x90\x82\x16\x10a\rMW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FRegistryCoordinator.quorumExists`D\x82\x01R\x7F: quorum does not exist\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08\xC5V[`\xFF\x82\x16`\0\x81\x81R`\x97` \x90\x81R`@\x91\x82\x90 \x84Q\x81T\x86\x84\x01\x80Q\x88\x87\x01\x80Qc\xFF\xFF\xFF\xFF\x90\x95\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x94\x16\x84\x17`\x01` \x1Ba\xFF\xFF\x93\x84\x16\x02\x17g\xFF\xFF\0\0\0\0\0\0\x19\x16`\x01`0\x1B\x95\x83\x16\x95\x90\x95\x02\x94\x90\x94\x17\x90\x94U\x85Q\x91\x82RQ\x83\x16\x93\x81\x01\x93\x90\x93RQ\x16\x91\x81\x01\x91\x90\x91R\x7F>\xE6\xFE\x8DTa\x02D\xC3\xE9\xD3\xC0f\xAEJ\xEE\x99x\x84\xAA(\xF1\x06\x16\xAE\x82\x19%@\x13\x18\xAC\x90``\x01`@Q\x80\x91\x03\x90\xA2PPV[`\x9ET`\x01`\x01`\xA0\x1B\x03\x163\x14a\x18\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FRegistryCoordinator.onlyEjector:`D\x82\x01R\x7F caller is not the ejector\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08\xC5V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x99` R`@\x90 \x80T`\x01\x80\x83\x01T`\xFF\x16`\x02\x81\x11\x15a*\xE7Wa*\xE7aM\xEBV[\x14a+fW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FRegistryCoordinator._deregisterO`D\x82\x01R\x7Fperator: operator is not registe`d\x82\x01Rb\x1C\x99Y`\xEA\x1B`\x84\x82\x01R`\xA4\x01a\x08\xC5V[`\x96T`\0\x90a+z\x90\x85\x90`\xFF\x16a(wV[\x90P`\0a+\x87\x83a#@V[\x90P`\x01`\x01`\xC0\x1B\x03\x82\x16a,\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`;`$\x82\x01R\x7FRegistryCoordinator._deregisterO`D\x82\x01R\x7Fperator: bitmap cannot be 0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08\xC5V[a,\x1C`\x01`\x01`\xC0\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x16\x14\x90V[a,\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`Y`$\x82\x01R\x7FRegistryCoordinator._deregisterO`D\x82\x01R\x7Fperator: operator is not registe`d\x82\x01R\x7Fred for specified quorums\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x08\xC5V[`\x01`\x01`\xC0\x1B\x03\x82\x81\x16\x19\x82\x16\x16a,\xCD\x84\x82aBuV[`\x01`\x01`\xC0\x1B\x03\x81\x16a-\x9CW`\x01\x85\x01\x80T`\xFF\x19\x16`\x02\x17\x90U`@QcQ\xB2zm`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA3d\xF4\xDA\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a-MW`\0\x80\xFD[PZ\xF1\x15\x80\x15a-aW=`\0\x80>=`\0\xFD[PP`@Q\x86\x92P`\x01`\x01`\xA0\x1B\x03\x8A\x16\x91P\x7F9o\xDC\xB1\x80\xCB\x0F\xEA&\x92\x81\x13\xFB\x0F\xD1\xC3T\x98c\xF9\xCDV>j\x18O\x1DW\x81\x16\xC8\xE4\x90`\0\x90\xA3[`@Qc\xF4\xE2O\xE5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xF4\xE2O\xE5\x90a-\xEA\x90\x8A\x90\x8A\x90`\x04\x01aZ\xC7V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a.\x04W`\0\x80\xFD[PZ\xF1\x15\x80\x15a.\x18W=`\0\x80>=`\0\xFD[PP`@Qc\xBD)\xB8\xCD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92Pc\xBD)\xB8\xCD\x91Pa.j\x90\x87\x90\x8A\x90`\x04\x01aZ\xEBV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a.\x84W`\0\x80\xFD[PZ\xF1\x15\x80\x15a.\x98W=`\0\x80>=`\0\xFD[PP`@Qc\xBD)\xB8\xCD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92Pc\xBD)\xB8\xCD\x91Pa.\xEA\x90\x87\x90\x8A\x90`\x04\x01aZ\xEBV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a/\x04W`\0\x80\xFD[PZ\xF1\x15\x80\x15a/\x18W=`\0\x80>=`\0\xFD[PPPPPPPPPPPV[`d\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\x13T*N\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a/\xE2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a0\x06\x91\x90a[\x04V[\x90P`\0\x81\x90\x03a\x0F\x18W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xBFy\xCEX\x84\x84a0K\x87a\x0F\xCCV[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a0i\x93\x92\x91\x90a[\x1DV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a0\x88W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)\x01\x91\x90a[\x04V[` \x80\x82\x01Q`\0\x90\x81R`\x9A\x90\x91R`@\x90 T`\xFF\x16\x15a1RW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`R`$\x82\x01R\x7FRegistryCoordinator._verifyChurn`D\x82\x01R\x7FApproverSignature: churnApprover`d\x82\x01Rq\x08\x1C\xD8[\x1D\x08\x18[\x1C\x99XY\x1EH\x1D\\\xD9Y`r\x1B`\x84\x82\x01R`\xA4\x01a\x08\xC5V[B\x81`@\x01Q\x10\x15a1\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`R`$\x82\x01R\x7FRegistryCoordinator._verifyChurn`D\x82\x01R\x7FApproverSignature: churnApprover`d\x82\x01Rq\x08\x1C\xDAY\xDB\x98]\x1D\\\x99H\x19^\x1C\x1A\\\x99Y`r\x1B`\x84\x82\x01R`\xA4\x01a\x08\xC5V[` \x80\x82\x01\x80Q`\0\x90\x81R`\x9A\x90\x92R`@\x91\x82\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U`\x9DT\x90Q\x91\x83\x01Qa\t\xAC\x92`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91a22\x91\x88\x91\x88\x91\x88\x91\x90a\x18\x10V[\x83QaD6V[a2]`@Q\x80``\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`\0a2\xA5\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`\x96T`\xFF\x16\x91Pa(w\x90PV[\x90P`\0a2\xB2\x88a#@V[\x90P`\x01`\x01`\xC0\x1B\x03\x82\x16a30W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FRegistryCoordinator._registerOpe`D\x82\x01R\x7Frator: bitmap cannot be 0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08\xC5V[\x80\x82\x16`\x01`\x01`\xC0\x1B\x03\x16\x15a3\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`h`$\x82\x01R\x7FRegistryCoordinator._registerOpe`D\x82\x01R\x7Frator: operator already register`d\x82\x01R\x7Fed for some quorums being regist`\x84\x82\x01Rg2\xB92\xB2\x1037\xB9`\xC1\x1B`\xA4\x82\x01R`\xC4\x01a\x08\xC5V[`\xA0T`\x01`\x01`\xA0\x1B\x03\x8A\x16`\0\x90\x81R`\x9F` R`@\x90 T`\x01`\x01`\xC0\x1B\x03\x83\x81\x16\x90\x85\x16\x17\x91B\x91a4\x1E\x91\x90aYJV[\x10a4\x9FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R\x7FRegistryCoordinator._registerOpe`D\x82\x01R\x7Frator: operator cannot reregiste`d\x82\x01Rd\x1C\x88\x1EY]`\xDA\x1B`\x84\x82\x01R`\xA4\x01a\x08\xC5V[a4\xA9\x89\x82aBuV[\x88\x7F\xEC)c\xAB!\xC1\xE5\x0E\x1EX*\xA5B\xAF.K\xF7\xBF8\xE6\xE1@<'\xB4.\x1C]nb\x1E\xAA\x87`@Qa4\xD9\x91\x90aX\0V[`@Q\x80\x91\x03\x90\xA2`\x01`\x01`\x01`\xA0\x1B\x03\x8B\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T`\xFF\x16`\x02\x81\x11\x15a5\x13Wa5\x13aM\xEBV[\x14a6,W`@\x80Q\x80\x82\x01\x82R\x8A\x81R`\x01` \x80\x83\x01\x82\x81R`\x01`\x01`\xA0\x1B\x03\x8F\x16`\0\x90\x81R`\x99\x90\x92R\x93\x90 \x82Q\x81U\x92Q\x83\x82\x01\x80T\x93\x94\x93\x91\x92\x90\x91`\xFF\x19\x16\x90\x83`\x02\x81\x11\x15a5nWa5naM\xEBV[\x02\x17\x90UPP`@Qc\x99&\xEE}`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91Pc\x99&\xEE}\x90a5\xC3\x90\x8D\x90\x89\x90`\x04\x01a[\x96V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a5\xDDW`\0\x80\xFD[PZ\xF1\x15\x80\x15a5\xF1W=`\0\x80>=`\0\xFD[PP`@Q\x8B\x92P`\x01`\x01`\xA0\x1B\x03\x8D\x16\x91P\x7F\xE8\xE6\x8C\xEF\x1C:v\x1E\xD7\xBE~\x84c\xA3u\xF2\x7F{\xC35\xE5\x18$\"<\xAC\xCEcn\xC5\xC3\xFE\x90`\0\x90\xA3[`@Qc\x1F\xD9<\xA9`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c?\xB2yR\x90a6|\x90\x8D\x90\x8C\x90\x8C\x90`\x04\x01a\\\nV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a6\x96W`\0\x80\xFD[PZ\xF1\x15\x80\x15a6\xAAW=`\0\x80>=`\0\xFD[PP`@Qc%PGw`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92Pc%PGw\x91Pa7\0\x90\x8D\x90\x8D\x90\x8D\x90\x8D\x90`\x04\x01a\\/V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a7\x1FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra7G\x91\x90\x81\x01\x90a\\\xBBV[`@\x80\x87\x01\x91\x90\x91R` \x86\x01\x91\x90\x91RQb\xBF\xF0M`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90b\xBF\xF0M\x90a7\xA4\x90\x8C\x90\x8C\x90\x8C\x90`\x04\x01a]\x1EV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a7\xC3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra7\xEB\x91\x90\x81\x01\x90a]8V[\x84RPPP\x96\x95PPPPPPV[` \x80\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`\0\x81\x81R`\x99\x90\x94R`@\x90\x93 T\x91\x92\x90\x87\x16\x03a8yW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R`\0\x80Q` a_M\x839\x81Q\x91R`D\x82\x01Rt97\x1D\x101\xB0\xB777\xBA\x101\xB4:\xB97\x109\xB2\xB63`Y\x1B`d\x82\x01R`\x84\x01a\x08\xC5V[\x87`\xFF\x16\x84`\0\x01Q`\xFF\x16\x14a8\xF6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R`\0\x80Q` a_M\x839\x81Q\x91R`D\x82\x01R\x7Frn: quorumNumber not the same as`d\x82\x01Rf\x08\x1C\xDAY\xDB\x99Y`\xCA\x1B`\x84\x82\x01R`\xA4\x01a\x08\xC5V[`@QcT\x01\xED'`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`\xFF\x89\x16`$\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cT\x01\xED'\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a9gW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a9\x8B\x91\x90a]\xD1V[\x90Pa9\x97\x81\x85aE\xF0V[`\x01`\x01``\x1B\x03\x16\x86`\x01`\x01``\x1B\x03\x16\x11a:*W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`V`$\x82\x01R`\0\x80Q` a_M\x839\x81Q\x91R`D\x82\x01R\x7Frn: incoming operator has insuff`d\x82\x01Ru4\xB1\xB4\xB2\xB7:\x109\xBA0\xB5\xB2\x9037\xB9\x101\xB4:\xB97`Q\x1B`\x84\x82\x01R`\xA4\x01a\x08\xC5V[a:4\x88\x85aF\x14V[`\x01`\x01``\x1B\x03\x16\x81`\x01`\x01``\x1B\x03\x16\x10a!cW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R`\0\x80Q` a_M\x839\x81Q\x91R`D\x82\x01R\x7Frn: cannot kick operator with mo`d\x82\x01R\x7Fre than kickBIPsOfTotalStake\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x08\xC5V[`\0\x81\x81R`\x98` R`@\x81 T\x81[\x81\x81\x10\x15a;`W`\x01a:\xF3\x82\x84aZ-V[a:\xFD\x91\x90aZ-V[\x92P\x84c\xFF\xFF\xFF\xFF\x16`\x98`\0\x86\x81R` \x01\x90\x81R` \x01`\0 \x84c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a;0Wa;0aWnV[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a;NWPPa\x0F\x18V[\x80a;X\x81aW\x9AV[\x91PPa:\xDFV[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`l`$\x82\x01R\x7FRegistryCoordinator.getQuorumBit`D\x82\x01R\x7FmapIndexAtBlockNumber: no bitmap`d\x82\x01R\x7F update found for operatorId at `\x84\x82\x01Rk167\xB1\xB5\x907:\xB6\xB12\xB9`\xA1\x1B`\xA4\x82\x01R`\xC4\x01a\x08\xC5V[`\x96T`\xFF\x16`\xC0\x81\x10a<~W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FRegistryCoordinator.createQuorum`D\x82\x01Rt\x0E\x88\x1BX^\x08\x1C][\xDC\x9D[\\\xC8\x1C\x99XX\xDA\x19Y`Z\x1B`d\x82\x01R`\x84\x01a\x08\xC5V[a<\x89\x81`\x01a]\xEEV[`\x96\x80T`\xFF\x19\x16`\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a<\xA8\x81\x86a)\x86V[`@Q`\x01b\x96\xB5\x89`\xE0\x1B\x03\x19\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xFFiJw\x90a<\xFB\x90\x84\x90\x88\x90\x88\x90`\x04\x01a^\x13V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a=\x15W`\0\x80\xFD[PZ\xF1\x15\x80\x15a=)W=`\0\x80>=`\0\xFD[PP`@Qc\x13l\xA0\xF9`\xE1\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x92Pc&\xD9A\xF2\x91P`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a=\x91W`\0\x80\xFD[PZ\xF1\x15\x80\x15a=\xA5W=`\0\x80>=`\0\xFD[PP`@Qc\x13l\xA0\xF9`\xE1\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x92Pc&\xD9A\xF2\x91P`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a>\rW`\0\x80\xFD[PZ\xF1\x15\x80\x15a!cW=`\0\x80>=`\0\xFD[`\0Tb\x01\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a>HWP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a>\xCAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x08\xC5V[`\x01\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a?\r\x82a%\xC3V[PPV[`\0\x80[\x82\x15a\x0F\x18Wa?&`\x01\x84aZ-V[\x90\x92\x16\x91\x80a?4\x81a^\x8CV[\x91PPa?\x15V[`\x000`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80\x15a?\x95WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14[\x15a?\xBFWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[P`@\x80Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x80\x83\x01\x91\x90\x91R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x84\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x83\x01RF`\x80\x83\x01R0`\xA0\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\xC0\x90\x92\x01\x90\x92R\x80Q\x91\x01 \x90V[`\0\x80\x80`\0\x80Q` a_m\x839\x81Q\x91R`\x03`\0\x80Q` a_m\x839\x81Q\x91R\x86`\0\x80Q` a_m\x839\x81Q\x91R\x88\x89\t\t\x08\x90P`\0a@\xD9\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\0\x80Q` a_m\x839\x81Q\x91RaF.V[\x91\x95\x91\x94P\x90\x92PPPV[`\0a\x01\0\x82Q\x11\x15aAnW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FBitmapUtils.orderedBytesArrayToB\x90\x82\x01R\x7Fitmap: orderedBytesArray is too `d\x82\x01Rclong`\xE0\x1B`\x84\x82\x01R`\xA4\x01a\x08\xC5V[\x81Q`\0\x03aA\x7FWP`\0\x91\x90PV[`\0\x80\x83`\0\x81Q\x81\x10aA\x95WaA\x95aWnV[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x81\x90\x1B\x92P[\x84Q\x81\x10\x15aBlW\x84\x81\x81Q\x81\x10aA\xC3WaA\xC3aWnV[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x1B\x91P\x82\x82\x11aBXW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: orderedBytesArray is not `d\x82\x01Rf\x1B\xDC\x99\x19\\\x99Y`\xCA\x1B`\x84\x82\x01R`\xA4\x01a\x08\xC5V[\x91\x81\x17\x91aBe\x81aW\x9AV[\x90PaA\xA8V[P\x90\x93\x92PPPV[`\0\x82\x81R`\x98` R`@\x81 T\x90\x81\x90\x03aC\x1DW`\0\x83\x81R`\x98` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x86\x81R`\x01`\x01`\xC0\x1B\x03\x80\x8A\x16\x95\x84\x01\x95\x86R\x84T`\x01\x81\x01\x86U\x94\x88R\x95\x90\x96 \x91Q\x91\x90\x92\x01\x80T\x95Q\x93Q\x90\x94\x16`\x01`@\x1B\x02`\x01`\x01`@\x1B\x03\x93\x83\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x96\x16\x91\x90\x92\x16\x17\x93\x90\x93\x17\x16\x91\x90\x91\x17\x90UPPPV[`\0\x83\x81R`\x98` R`@\x81 aC6`\x01\x84aZ-V[\x81T\x81\x10aCFWaCFaWnV[`\0\x91\x82R` \x90\x91 \x01\x80T\x90\x91Pc\xFF\xFF\xFF\xFFC\x81\x16\x91\x16\x03aC\x88W\x80T`\x01`\x01`@\x1B\x03\x16`\x01`@\x1B`\x01`\x01`\xC0\x1B\x03\x85\x16\x02\x17\x81Ua\t\xACV[\x80Tc\xFF\xFF\xFF\xFFC\x81\x16`\x01` \x1B\x81\x81\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x94\x16\x93\x90\x93\x17\x84U`\0\x87\x81R`\x98` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83R\x94\x85R\x84\x83\x01\x84\x81R`\x01`\x01`\xC0\x1B\x03\x80\x8C\x16\x93\x87\x01\x93\x84R\x82T`\x01\x81\x01\x84U\x92\x86R\x93\x90\x94 \x94Q\x94\x01\x80T\x93Q\x91Q\x90\x92\x16`\x01`@\x1B\x02`\x01`\x01`@\x1B\x03\x91\x86\x16\x90\x96\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x93\x90\x94\x16\x92\x90\x92\x17\x17\x91\x90\x91\x16\x91\x90\x91\x17\x90UPPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16;\x15aEPW`@Qc\x0B\x13]?`\xE1\x1B\x80\x82R\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\x16&\xBA~\x90aDv\x90\x86\x90\x86\x90`\x04\x01aZ\xEBV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aD\x93W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aD\xB7\x91\x90a^\xADV[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14a\x16\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`S`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: ERC1271 signature `d\x82\x01Rr\x1D\x99\\\x9AY\x9AX\xD8]\x1A[\xDB\x88\x19\x98Z[\x19Y`j\x1B`\x84\x82\x01R`\xA4\x01a\x08\xC5V[\x82`\x01`\x01`\xA0\x1B\x03\x16aEd\x83\x83aF\xD7V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x16\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: signature not from`d\x82\x01Rf\x109\xB4\xB3\xB72\xB9`\xC9\x1B`\x84\x82\x01R`\xA4\x01a\x08\xC5V[` \x81\x01Q`\0\x90a'\x10\x90aF\n\x90a\xFF\xFF\x16\x85a^\xD7V[a)\x01\x91\x90a_\x06V[`@\x81\x01Q`\0\x90a'\x10\x90aF\n\x90a\xFF\xFF\x16\x85a^\xD7V[`\0\x80aF9aJ=V[aFAaJ[V[` \x80\x82R\x81\x81\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x88\x90R`\x80\x82\x01\x87\x90R`\xA0\x82\x01\x86\x90R\x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82\x80aF~W\xFE[P\x82aF\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FBN254.expMod: call failure\0\0\0\0\0\0`D\x82\x01R`d\x01a\x08\xC5V[PQ\x95\x94PPPPPV[`\0\x80`\0aF\xE6\x85\x85aF\xF3V[\x91P\x91Pa\x1D\xE9\x81aGaV[`\0\x80\x82Q`A\x03aG)W` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1AaG\x1D\x87\x82\x85\x85aI\x17V[\x94P\x94PPPPaGZV[\x82Q`@\x03aGRW` \x83\x01Q`@\x84\x01QaGG\x86\x83\x83aJ\x04V[\x93P\x93PPPaGZV[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0\x81`\x04\x81\x11\x15aGuWaGuaM\xEBV[\x03aG}WPV[`\x01\x81`\x04\x81\x11\x15aG\x91WaG\x91aM\xEBV[\x03aG\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x08\xC5V[`\x02\x81`\x04\x81\x11\x15aG\xF2WaG\xF2aM\xEBV[\x03aH?W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x08\xC5V[`\x03\x81`\x04\x81\x11\x15aHSWaHSaM\xEBV[\x03aH\xABW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x08\xC5V[`\x04\x81`\x04\x81\x11\x15aH\xBFWaH\xBFaM\xEBV[\x03a\rMW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 'v' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x08\xC5V[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15aINWP`\0\x90P`\x03aI\xFBV[\x84`\xFF\x16`\x1B\x14\x15\x80\x15aIfWP\x84`\xFF\x16`\x1C\x14\x15[\x15aIwWP`\0\x90P`\x04aI\xFBV[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15aI\xCBW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16aI\xF4W`\0`\x01\x92P\x92PPaI\xFBV[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x80`\x01`\x01`\xFF\x1B\x03\x83\x16\x81aJ!`\xFF\x86\x90\x1C`\x1BaYJV[\x90PaJ/\x87\x82\x88\x85aI\x17V[\x93P\x93PPP\x93P\x93\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`\0\x80\x83`\x1F\x84\x01\x12aJ\x8BW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aJ\xA2W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15aGZW`\0\x80\xFD[`\0\x80` \x83\x85\x03\x12\x15aJ\xD0W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aJ\xE6W`\0\x80\xFD[aJ\xF2\x85\x82\x86\x01aJyV[\x90\x96\x90\x95P\x93PPPPV[`\0` \x82\x84\x03\x12\x15aK\x10W`\0\x80\xFD[P5\x91\x90PV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\rMW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15aK>W`\0\x80\xFD[\x835\x92P` \x84\x015aKP\x81aK\x17V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aK\x99WaK\x99aKaV[`@R\x90V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aK\x99WaK\x99aKaV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aK\xE9WaK\xE9aKaV[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x83\x11\x15aL\nWaL\naKaV[aL\x1D`\x1F\x84\x01`\x1F\x19\x16` \x01aK\xC1V[\x90P\x82\x81R\x83\x83\x83\x01\x11\x15aL1W`\0\x80\xFD[\x82\x82` \x83\x017`\0` \x84\x83\x01\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15aLZW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aLpW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13aL\x81W`\0\x80\xFD[aL\x90\x84\x825` \x84\x01aK\xF1V[\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\rMW`\0\x80\xFD[\x805aL\xB8\x81aL\x98V[\x91\x90PV[`\0` \x82\x84\x03\x12\x15aL\xCFW`\0\x80\xFD[\x815a)\x01\x81aL\x98V[`\0\x80`@\x83\x85\x03\x12\x15aL\xEDW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[\x805`\xFF\x81\x16\x81\x14aL\xB8W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aM\x1FW`\0\x80\xFD[a)\x01\x82aL\xFCV[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x81\x01a\x0F\x18V[`\0\x80\x83`\x1F\x84\x01\x12aMQW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aMhW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aGZW`\0\x80\xFD[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15aM\x96W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aM\xADW`\0\x80\xFD[aM\xB9\x88\x83\x89\x01aJyV[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15aM\xD2W`\0\x80\xFD[PaM\xDF\x87\x82\x88\x01aM?V[\x95\x98\x94\x97P\x95PPPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x03\x81\x10aN\x1FWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[\x81Q\x81R` \x80\x83\x01Q`@\x83\x01\x91aN>\x90\x84\x01\x82aN\x01V[P\x92\x91PPV[\x805a\xFF\xFF\x81\x16\x81\x14aL\xB8W`\0\x80\xFD[`\0``\x82\x84\x03\x12\x15aNiW`\0\x80\xFD[aNqaKwV[\x90P\x815aN~\x81aK\x17V[\x81RaN\x8C` \x83\x01aNEV[` \x82\x01RaN\x9D`@\x83\x01aNEV[`@\x82\x01R\x92\x91PPV[`\0\x80`\x80\x83\x85\x03\x12\x15aN\xBBW`\0\x80\xFD[aN\xC4\x83aL\xFCV[\x91PaN\xD3\x84` \x85\x01aNWV[\x90P\x92P\x92\x90PV[`\0\x80`\0`@\x84\x86\x03\x12\x15aN\xF1W`\0\x80\xFD[\x835aN\xFC\x81aL\x98V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aO\x17W`\0\x80\xFD[aO#\x86\x82\x87\x01aM?V[\x94\x97\x90\x96P\x93\x94PPPPV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15aOIWaOIaKaV[P`\x05\x1B` \x01\x90V[`\0`@\x82\x84\x03\x12\x15aOeW`\0\x80\xFD[aOmaK\x9FV[\x90PaOx\x82aL\xFCV[\x81R` \x82\x015aO\x88\x81aL\x98V[` \x82\x01R\x92\x91PPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15aO\xABW`\0\x80\xFD[\x855aO\xB6\x81aL\x98V[\x94P` \x86\x81\x015\x94P`@\x80\x88\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aO\xDAW`\0\x80\xFD[\x88\x01`\x1F\x81\x01\x8A\x13aO\xEBW`\0\x80\xFD[\x805aO\xFEaO\xF9\x82aO0V[aK\xC1V[\x81\x81R`\x06\x91\x90\x91\x1B\x82\x01\x84\x01\x90\x84\x81\x01\x90\x8C\x83\x11\x15aP\x1DW`\0\x80\xFD[\x92\x85\x01\x92[\x82\x84\x10\x15aPCWaP4\x8D\x85aOSV[\x82R\x92\x84\x01\x92\x90\x85\x01\x90aP\"V[\x99\x9C\x98\x9BP\x98\x99``\x81\x015\x99P`\x80\x015\x97\x96PPPPPPPV[`\0a\x01\0\x82\x84\x03\x12\x15aPsW`\0\x80\xFD[P\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12aP\x8BW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aP\xA2W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x06\x1B\x85\x01\x01\x11\x15aGZW`\0\x80\xFD[`\0``\x82\x84\x03\x12\x15aP\xCFW`\0\x80\xFD[aP\xD7aKwV[\x90P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aP\xEFW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13aQ\0W`\0\x80\xFD[aQ\x0F\x84\x825` \x84\x01aK\xF1V[\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\0\x80`\0a\x01\xA0\x8A\x8C\x03\x12\x15aQKW`\0\x80\xFD[\x895`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aQbW`\0\x80\xFD[aQn\x8D\x83\x8E\x01aM?V[\x90\x9BP\x99P` \x8C\x015\x91P\x80\x82\x11\x15aQ\x87W`\0\x80\xFD[aQ\x93\x8D\x83\x8E\x01aM?V[\x90\x99P\x97P\x87\x91PaQ\xA8\x8D`@\x8E\x01aP`V[\x96Pa\x01@\x8C\x015\x91P\x80\x82\x11\x15aQ\xBFW`\0\x80\xFD[aQ\xCB\x8D\x83\x8E\x01aPyV[\x90\x96P\x94Pa\x01`\x8C\x015\x91P\x80\x82\x11\x15aQ\xE5W`\0\x80\xFD[aQ\xF1\x8D\x83\x8E\x01aP\xBDV[\x93Pa\x01\x80\x8C\x015\x91P\x80\x82\x11\x15aR\x08W`\0\x80\xFD[PaR\x15\x8C\x82\x8D\x01aP\xBDV[\x91PP\x92\x95\x98P\x92\x95\x98P\x92\x95\x98V[`\0\x80`\0\x80`\0\x80a\x01`\x87\x89\x03\x12\x15aR?W`\0\x80\xFD[\x865`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aRVW`\0\x80\xFD[aRb\x8A\x83\x8B\x01aM?V[\x90\x98P\x96P` \x89\x015\x91P\x80\x82\x11\x15aR{W`\0\x80\xFD[aR\x87\x8A\x83\x8B\x01aM?V[\x90\x96P\x94P\x84\x91PaR\x9C\x8A`@\x8B\x01aP`V[\x93Pa\x01@\x89\x015\x91P\x80\x82\x11\x15aR\xB3W`\0\x80\xFD[PaR\xC0\x89\x82\x8A\x01aP\xBDV[\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0\x80`@\x83\x85\x03\x12\x15aR\xE0W`\0\x80\xFD[\x825aR\xEB\x81aK\x17V[\x91P` \x83\x81\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aS\x07W`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13aS\x18W`\0\x80\xFD[\x805aS&aO\xF9\x82aO0V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x88\x83\x11\x15aSEW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aScW\x835\x82R\x92\x84\x01\x92\x90\x84\x01\x90aSJV[\x80\x95PPPPPP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aS\xB0W\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aS\x8EV[P\x90\x96\x95PPPPPPV[`\0\x80` \x83\x85\x03\x12\x15aS\xCFW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aS\xE5W`\0\x80\xFD[aJ\xF2\x85\x82\x86\x01aM?V[`\x01`\x01``\x1B\x03\x81\x16\x81\x14a\rMW`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12aT\x17W`\0\x80\xFD[\x815` aT'aO\xF9\x83aO0V[\x82\x81R`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aTFW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aT\x97W`@\x81\x89\x03\x12\x15aTcW`\0\x80\x81\xFD[aTkaK\x9FV[\x815aTv\x81aL\x98V[\x81R\x81\x85\x015aT\x85\x81aS\xF1V[\x81\x86\x01R\x83R\x91\x83\x01\x91`@\x01aTJV[P\x96\x95PPPPPPV[`\0\x80`\0`\xA0\x84\x86\x03\x12\x15aT\xB7W`\0\x80\xFD[aT\xC1\x85\x85aNWV[\x92P``\x84\x015aT\xD1\x81aS\xF1V[\x91P`\x80\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aT\xECW`\0\x80\xFD[aT\xF8\x86\x82\x87\x01aT\x06V[\x91PP\x92P\x92P\x92V[`\0\x82`\x1F\x83\x01\x12aU\x13W`\0\x80\xFD[\x815` aU#aO\xF9\x83aO0V[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15aUBW`\0\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15aUeWaUX\x89\x82aNWV[\x84R\x92\x84\x01\x92\x81\x01aUFV[P\x90\x97\x96PPPPPPPV[`\0\x82`\x1F\x83\x01\x12aU\x83W`\0\x80\xFD[\x815` aU\x93aO\xF9\x83aO0V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aU\xB2W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aT\x97W\x805aU\xC9\x81aS\xF1V[\x83R\x91\x83\x01\x91\x83\x01aU\xB6V[`\0\x82`\x1F\x83\x01\x12aU\xE7W`\0\x80\xFD[\x815` aU\xF7aO\xF9\x83aO0V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aV\x16W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aT\x97W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aV9W`\0\x80\x81\xFD[aVG\x89\x86\x83\x8B\x01\x01aT\x06V[\x84RP\x91\x83\x01\x91\x83\x01aV\x1AV[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15aVrW`\0\x80\xFD[aV{\x89aL\xADV[\x97PaV\x89` \x8A\x01aL\xADV[\x96PaV\x97`@\x8A\x01aL\xADV[\x95PaV\xA5``\x8A\x01aL\xADV[\x94P`\x80\x89\x015\x93P`\xA0\x89\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aV\xC8W`\0\x80\xFD[aV\xD4\x8C\x83\x8D\x01aU\x02V[\x94P`\xC0\x8B\x015\x91P\x80\x82\x11\x15aV\xEAW`\0\x80\xFD[aV\xF6\x8C\x83\x8D\x01aUrV[\x93P`\xE0\x8B\x015\x91P\x80\x82\x11\x15aW\x0CW`\0\x80\xFD[PaW\x19\x8B\x82\x8C\x01aU\xD6V[\x91PP\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[` \x81\x01a\x0F\x18\x82\x84aN\x01V[` \x80\x82R`\x19\x90\x82\x01R\x7FPausable: index is paused\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01aW\xACWaW\xACaW\x84V[P`\x01\x01\x90V[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15aW\xD9W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01aW\xBDV[\x81\x81\x11\x15aW\xEBW`\0` \x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a)\x01` \x83\x01\x84aW\xB3V[`\0` \x82\x84\x03\x12\x15aX%W`\0\x80\xFD[\x81Qa)\x01\x81aL\x98V[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15aX\x8CW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a)\x01W`\0\x80\xFD[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aX\xFBW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aY\x15W`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15aGZW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aY?W`\0\x80\xFD[\x81Qa)\x01\x81aK\x17V[`\0\x82\x19\x82\x11\x15aY]WaY]aW\x84V[P\x01\x90V[`\0\x80\x85\x85\x11\x15aYrW`\0\x80\xFD[\x83\x86\x11\x15aY\x7FW`\0\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[`\0`\xC0\x82\x01\x88\x83R` `\x01\x80`\xA0\x1B\x03\x80\x8A\x16\x82\x86\x01R`@\x89\x81\x87\x01R`\xC0``\x87\x01R\x83\x89Q\x80\x86R`\xE0\x88\x01\x91P\x84\x8B\x01\x95P`\0[\x81\x81\x10\x15aY\xF1W\x86Q\x80Q`\xFF\x16\x84R\x86\x01Q\x85\x16\x86\x84\x01R\x95\x85\x01\x95\x91\x83\x01\x91`\x01\x01aY\xC7V[PP`\x80\x87\x01\x98\x90\x98RPPPP`\xA0\x90\x91\x01\x91\x90\x91RP\x94\x93PPPPV[`\0`@\x82\x84\x03\x12\x15aZ#W`\0\x80\xFD[a)\x01\x83\x83aOSV[`\0\x82\x82\x10\x15aZ?WaZ?aW\x84V[P\x03\x90V[`\x01\x80`\xA0\x1B\x03\x84\x16\x81R\x82` \x82\x01R```@\x82\x01R`\0aZk``\x83\x01\x84aW\xB3V[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15aZ\x86W`\0\x80\xFD[\x81Q`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x14a)\x01W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82aZ\xC2WaZ\xC2aZ\x9DV[P\x06\x90V[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R`\0\x90aL\x90\x90\x83\x01\x84aW\xB3V[\x82\x81R`@` \x82\x01R`\0aL\x90`@\x83\x01\x84aW\xB3V[`\0` \x82\x84\x03\x12\x15a[\x16W`\0\x80\xFD[PQ\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81Ra\x01`\x81\x01a[E` \x83\x01\x85\x805\x82R` \x90\x81\x015\x91\x01RV[a[_``\x83\x01`@\x86\x01\x805\x82R` \x90\x81\x015\x91\x01RV[`@`\x80\x85\x01`\xA0\x84\x017`@`\xC0\x85\x01`\xE0\x84\x017`\0a\x01 \x83\x01\x90\x81R\x83Q\x90R` \x90\x92\x01Qa\x01@\x90\x91\x01R\x92\x91PPV[`\x01\x80`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R`\0\x82Q```@\x84\x01Ra[\xC0`\xA0\x84\x01\x82aW\xB3V[\x90P` \x84\x01Q``\x84\x01R`@\x84\x01Q`\x80\x84\x01R\x80\x91PP\x93\x92PPPV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R`@` \x82\x01\x81\x90R`\0\x90aZk\x90\x83\x01\x84\x86a[\xE1V[`\x01\x80`\xA0\x1B\x03\x85\x16\x81R\x83` \x82\x01R```@\x82\x01R`\0a\x18P``\x83\x01\x84\x86a[\xE1V[`\0\x82`\x1F\x83\x01\x12a\\hW`\0\x80\xFD[\x81Q` a\\xaO\xF9\x83aO0V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\\\x97W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aT\x97W\x80Qa\\\xAE\x81aS\xF1V[\x83R\x91\x83\x01\x91\x83\x01a\\\x9BV[`\0\x80`@\x83\x85\x03\x12\x15a\\\xCEW`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\\\xE5W`\0\x80\xFD[a\\\xF1\x86\x83\x87\x01a\\WV[\x93P` \x85\x01Q\x91P\x80\x82\x11\x15a]\x07W`\0\x80\xFD[Pa]\x14\x85\x82\x86\x01a\\WV[\x91PP\x92P\x92\x90PV[\x83\x81R`@` \x82\x01R`\0aZk`@\x83\x01\x84\x86a[\xE1V[`\0` \x80\x83\x85\x03\x12\x15a]KW`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a]aW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a]rW`\0\x80\xFD[\x80Qa]\x80aO\xF9\x82aO0V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a]\x9FW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a]\xC6W\x83Qa]\xB7\x81aK\x17V[\x82R\x92\x84\x01\x92\x90\x84\x01\x90a]\xA4V[\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a]\xE3W`\0\x80\xFD[\x81Qa)\x01\x81aS\xF1V[`\0`\xFF\x82\x16`\xFF\x84\x16\x80`\xFF\x03\x82\x11\x15a^\x0BWa^\x0BaW\x84V[\x01\x93\x92PPPV[`\0``\x82\x01`\xFF\x86\x16\x83R` `\x01`\x01``\x1B\x03\x80\x87\x16\x82\x86\x01R`@``\x81\x87\x01R\x83\x87Q\x80\x86R`\x80\x88\x01\x91P\x84\x89\x01\x95P`\0[\x81\x81\x10\x15a^|W\x86Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x86\x01Q\x85\x16\x86\x84\x01R\x95\x85\x01\x95\x91\x83\x01\x91`\x01\x01a^LV[P\x90\x9A\x99PPPPPPPPPPV[`\0a\xFF\xFF\x80\x83\x16\x81\x81\x03a^\xA3Wa^\xA3aW\x84V[`\x01\x01\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a^\xBFW`\0\x80\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a)\x01W`\0\x80\xFD[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15a^\xFDWa^\xFDaW\x84V[\x02\x94\x93PPPPV[`\0`\x01`\x01``\x1B\x03\x80\x84\x16\x80a_ Wa_ aZ\x9DV[\x92\x16\x91\x90\x91\x04\x92\x91PPV\xFERegistryCoordinator.updateOperatRegistryCoordinator._validateChu0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 B)\xC6\xF0\x16\xBCq\xCF0?Y\xD7\xD3mf\\\xAA>\x1Cx~\x01h*=\t\xC2\x7F\xCCB\x9F\xBEdsolcC\0\x08\r\x003";
    /// The deployed bytecode of the contract.
    pub static REGISTRYCOORDINATOR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct RegistryCoordinator<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for RegistryCoordinator<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for RegistryCoordinator<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for RegistryCoordinator<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for RegistryCoordinator<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(RegistryCoordinator))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> RegistryCoordinator<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                REGISTRYCOORDINATOR_ABI.clone(),
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
                REGISTRYCOORDINATOR_ABI.clone(),
                REGISTRYCOORDINATOR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
        ///Calls the contract's `blsApkRegistry` (0x5df45946) function
        pub fn bls_apk_registry(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
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
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
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
        ///Calls the contract's `ejectionCooldown` (0xa96f783e) function
        pub fn ejection_cooldown(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([169, 111, 120, 62], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ejector` (0x28f61b31) function
        pub fn ejector(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([40, 246, 27, 49], ())
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
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
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
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
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
        ///Calls the contract's `lastEjectionTimestamp` (0x125e0584) function
        pub fn last_ejection_timestamp(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([18, 94, 5, 132], p0)
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
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([99, 71, 201, 0], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `serviceManager` (0x3998fdd3) function
        pub fn service_manager(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
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
        ///Calls the contract's `setEjectionCooldown` (0x0d3f2134) function
        pub fn set_ejection_cooldown(
            &self,
            ejection_cooldown: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([13, 63, 33, 52], ejection_cooldown)
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
        ///Calls the contract's `stakeRegistry` (0x68304835) function
        pub fn stake_registry(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([104, 48, 72, 53], ())
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
            operators_per_quorum: ::std::vec::Vec<::std::vec::Vec<::ethers::core::types::Address>>,
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
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ChurnApproverUpdatedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `EjectorUpdated` event
        pub fn ejector_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, EjectorUpdatedFilter>
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
        ///Gets the contract's `OperatorDeregistered` event
        pub fn operator_deregistered_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OperatorDeregisteredFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `OperatorRegistered` event
        pub fn operator_registered_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OperatorRegisteredFilter>
        {
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
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OperatorSocketUpdateFilter>
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
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, UnpausedFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RegistryCoordinatorEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for RegistryCoordinator<M>
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
        Hash,
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
        Hash,
    )]
    #[ethevent(
        name = "OperatorRegistered",
        abi = "OperatorRegistered(address,bytes32)"
    )]
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
        Hash,
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
        Hash,
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
    pub enum RegistryCoordinatorEvents {
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
    }
    impl ::ethers::contract::EthLogDecode for RegistryCoordinatorEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ChurnApproverUpdatedFilter::decode_log(log) {
                return Ok(RegistryCoordinatorEvents::ChurnApproverUpdatedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = EjectorUpdatedFilter::decode_log(log) {
                return Ok(RegistryCoordinatorEvents::EjectorUpdatedFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(RegistryCoordinatorEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OperatorDeregisteredFilter::decode_log(log) {
                return Ok(RegistryCoordinatorEvents::OperatorDeregisteredFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = OperatorRegisteredFilter::decode_log(log) {
                return Ok(RegistryCoordinatorEvents::OperatorRegisteredFilter(decoded));
            }
            if let Ok(decoded) = OperatorSetParamsUpdatedFilter::decode_log(log) {
                return Ok(RegistryCoordinatorEvents::OperatorSetParamsUpdatedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = OperatorSocketUpdateFilter::decode_log(log) {
                return Ok(RegistryCoordinatorEvents::OperatorSocketUpdateFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(RegistryCoordinatorEvents::OwnershipTransferredFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(RegistryCoordinatorEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = PauserRegistrySetFilter::decode_log(log) {
                return Ok(RegistryCoordinatorEvents::PauserRegistrySetFilter(decoded));
            }
            if let Ok(decoded) = QuorumBlockNumberUpdatedFilter::decode_log(log) {
                return Ok(RegistryCoordinatorEvents::QuorumBlockNumberUpdatedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(RegistryCoordinatorEvents::UnpausedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for RegistryCoordinatorEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ChurnApproverUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::EjectorUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorDeregisteredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorRegisteredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorSetParamsUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorSocketUpdateFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PausedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauserRegistrySetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::QuorumBlockNumberUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnpausedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ChurnApproverUpdatedFilter> for RegistryCoordinatorEvents {
        fn from(value: ChurnApproverUpdatedFilter) -> Self {
            Self::ChurnApproverUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<EjectorUpdatedFilter> for RegistryCoordinatorEvents {
        fn from(value: EjectorUpdatedFilter) -> Self {
            Self::EjectorUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for RegistryCoordinatorEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OperatorDeregisteredFilter> for RegistryCoordinatorEvents {
        fn from(value: OperatorDeregisteredFilter) -> Self {
            Self::OperatorDeregisteredFilter(value)
        }
    }
    impl ::core::convert::From<OperatorRegisteredFilter> for RegistryCoordinatorEvents {
        fn from(value: OperatorRegisteredFilter) -> Self {
            Self::OperatorRegisteredFilter(value)
        }
    }
    impl ::core::convert::From<OperatorSetParamsUpdatedFilter> for RegistryCoordinatorEvents {
        fn from(value: OperatorSetParamsUpdatedFilter) -> Self {
            Self::OperatorSetParamsUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<OperatorSocketUpdateFilter> for RegistryCoordinatorEvents {
        fn from(value: OperatorSocketUpdateFilter) -> Self {
            Self::OperatorSocketUpdateFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for RegistryCoordinatorEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<PausedFilter> for RegistryCoordinatorEvents {
        fn from(value: PausedFilter) -> Self {
            Self::PausedFilter(value)
        }
    }
    impl ::core::convert::From<PauserRegistrySetFilter> for RegistryCoordinatorEvents {
        fn from(value: PauserRegistrySetFilter) -> Self {
            Self::PauserRegistrySetFilter(value)
        }
    }
    impl ::core::convert::From<QuorumBlockNumberUpdatedFilter> for RegistryCoordinatorEvents {
        fn from(value: QuorumBlockNumberUpdatedFilter) -> Self {
            Self::QuorumBlockNumberUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<UnpausedFilter> for RegistryCoordinatorEvents {
        fn from(value: UnpausedFilter) -> Self {
            Self::UnpausedFilter(value)
        }
    }
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
        Hash,
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
        Hash,
    )]
    #[ethcall(
        name = "PUBKEY_REGISTRATION_TYPEHASH",
        abi = "PUBKEY_REGISTRATION_TYPEHASH()"
    )]
    pub struct PubkeyRegistrationTypehashCall;
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    #[ethcall(name = "ejectOperator", abi = "ejectOperator(address,bytes)")]
    pub struct EjectOperatorCall {
        pub operator: ::ethers::core::types::Address,
        pub quorum_numbers: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `ejectionCooldown` function with signature `ejectionCooldown()` and selector `0xa96f783e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "ejectionCooldown", abi = "ejectionCooldown()")]
    pub struct EjectionCooldownCall;
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
        Hash,
    )]
    #[ethcall(name = "ejector", abi = "ejector()")]
    pub struct EjectorCall;
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
        Hash,
    )]
    #[ethcall(
        name = "getCurrentQuorumBitmap",
        abi = "getCurrentQuorumBitmap(bytes32)"
    )]
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    #[ethcall(
        name = "isChurnApproverSaltUsed",
        abi = "isChurnApproverSaltUsed(bytes32)"
    )]
    pub struct IsChurnApproverSaltUsedCall(pub [u8; 32]);
    ///Container type for all input parameters for the `lastEjectionTimestamp` function with signature `lastEjectionTimestamp(address)` and selector `0x125e0584`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "lastEjectionTimestamp", abi = "lastEjectionTimestamp(address)")]
    pub struct LastEjectionTimestampCall(pub ::ethers::core::types::Address);
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    #[ethcall(
        name = "quorumUpdateBlockNumber",
        abi = "quorumUpdateBlockNumber(uint8)"
    )]
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    #[ethcall(name = "setChurnApprover", abi = "setChurnApprover(address)")]
    pub struct SetChurnApproverCall {
        pub churn_approver: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setEjectionCooldown` function with signature `setEjectionCooldown(uint256)` and selector `0x0d3f2134`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setEjectionCooldown", abi = "setEjectionCooldown(uint256)")]
    pub struct SetEjectionCooldownCall {
        pub ejection_cooldown: ::ethers::core::types::U256,
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
        Hash,
    )]
    #[ethcall(name = "setEjector", abi = "setEjector(address)")]
    pub struct SetEjectorCall {
        pub ejector: ::ethers::core::types::Address,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    #[ethcall(
        name = "updateOperatorsForQuorum",
        abi = "updateOperatorsForQuorum(address[][],bytes)"
    )]
    pub struct UpdateOperatorsForQuorumCall {
        pub operators_per_quorum: ::std::vec::Vec<::std::vec::Vec<::ethers::core::types::Address>>,
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
        Hash,
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
        Hash,
    )]
    pub enum RegistryCoordinatorCalls {
        OperatorChurnApprovalTypehash(OperatorChurnApprovalTypehashCall),
        PubkeyRegistrationTypehash(PubkeyRegistrationTypehashCall),
        BlsApkRegistry(BlsApkRegistryCall),
        CalculateOperatorChurnApprovalDigestHash(CalculateOperatorChurnApprovalDigestHashCall),
        ChurnApprover(ChurnApproverCall),
        CreateQuorum(CreateQuorumCall),
        DeregisterOperator(DeregisterOperatorCall),
        EjectOperator(EjectOperatorCall),
        EjectionCooldown(EjectionCooldownCall),
        Ejector(EjectorCall),
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
        LastEjectionTimestamp(LastEjectionTimestampCall),
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
        SetEjectionCooldown(SetEjectionCooldownCall),
        SetEjector(SetEjectorCall),
        SetOperatorSetParams(SetOperatorSetParamsCall),
        SetPauserRegistry(SetPauserRegistryCall),
        StakeRegistry(StakeRegistryCall),
        TransferOwnership(TransferOwnershipCall),
        Unpause(UnpauseCall),
        UpdateOperators(UpdateOperatorsCall),
        UpdateOperatorsForQuorum(UpdateOperatorsForQuorumCall),
        UpdateSocket(UpdateSocketCall),
    }
    impl ::ethers::core::abi::AbiDecode for RegistryCoordinatorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <OperatorChurnApprovalTypehashCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OperatorChurnApprovalTypehash(decoded));
            }
            if let Ok(decoded) =
                <PubkeyRegistrationTypehashCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PubkeyRegistrationTypehash(decoded));
            }
            if let Ok(decoded) =
                <BlsApkRegistryCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BlsApkRegistry(decoded));
            }
            if let Ok(decoded) = <CalculateOperatorChurnApprovalDigestHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateOperatorChurnApprovalDigestHash(decoded));
            }
            if let Ok(decoded) = <ChurnApproverCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ChurnApprover(decoded));
            }
            if let Ok(decoded) = <CreateQuorumCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CreateQuorum(decoded));
            }
            if let Ok(decoded) =
                <DeregisterOperatorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DeregisterOperator(decoded));
            }
            if let Ok(decoded) = <EjectOperatorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EjectOperator(decoded));
            }
            if let Ok(decoded) =
                <EjectionCooldownCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EjectionCooldown(decoded));
            }
            if let Ok(decoded) = <EjectorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Ejector(decoded));
            }
            if let Ok(decoded) =
                <GetCurrentQuorumBitmapCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetCurrentQuorumBitmap(decoded));
            }
            if let Ok(decoded) = <GetOperatorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetOperator(decoded));
            }
            if let Ok(decoded) =
                <GetOperatorFromIdCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetOperatorFromId(decoded));
            }
            if let Ok(decoded) = <GetOperatorIdCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetOperatorId(decoded));
            }
            if let Ok(decoded) =
                <GetOperatorSetParamsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetOperatorSetParams(decoded));
            }
            if let Ok(decoded) =
                <GetOperatorStatusCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetOperatorStatus(decoded));
            }
            if let Ok(decoded) =
                <GetQuorumBitmapAtBlockNumberByIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetQuorumBitmapAtBlockNumberByIndex(decoded));
            }
            if let Ok(decoded) =
                <GetQuorumBitmapHistoryLengthCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetQuorumBitmapHistoryLength(decoded));
            }
            if let Ok(decoded) =
                <GetQuorumBitmapIndicesAtBlockNumberCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetQuorumBitmapIndicesAtBlockNumber(decoded));
            }
            if let Ok(decoded) =
                <GetQuorumBitmapUpdateByIndexCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetQuorumBitmapUpdateByIndex(decoded));
            }
            if let Ok(decoded) = <IndexRegistryCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IndexRegistry(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) =
                <IsChurnApproverSaltUsedCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IsChurnApproverSaltUsed(decoded));
            }
            if let Ok(decoded) =
                <LastEjectionTimestampCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LastEjectionTimestamp(decoded));
            }
            if let Ok(decoded) = <NumRegistriesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NumRegistries(decoded));
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
                <PubkeyRegistrationMessageHashCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PubkeyRegistrationMessageHash(decoded));
            }
            if let Ok(decoded) = <QuorumCountCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::QuorumCount(decoded));
            }
            if let Ok(decoded) =
                <QuorumUpdateBlockNumberCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::QuorumUpdateBlockNumber(decoded));
            }
            if let Ok(decoded) =
                <RegisterOperatorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RegisterOperator(decoded));
            }
            if let Ok(decoded) =
                <RegisterOperatorWithChurnCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RegisterOperatorWithChurn(decoded));
            }
            if let Ok(decoded) = <RegistriesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Registries(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <ServiceManagerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ServiceManager(decoded));
            }
            if let Ok(decoded) =
                <SetChurnApproverCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetChurnApprover(decoded));
            }
            if let Ok(decoded) =
                <SetEjectionCooldownCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetEjectionCooldown(decoded));
            }
            if let Ok(decoded) = <SetEjectorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetEjector(decoded));
            }
            if let Ok(decoded) =
                <SetOperatorSetParamsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetOperatorSetParams(decoded));
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
                <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <UnpauseCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Unpause(decoded));
            }
            if let Ok(decoded) =
                <UpdateOperatorsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateOperators(decoded));
            }
            if let Ok(decoded) =
                <UpdateOperatorsForQuorumCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateOperatorsForQuorum(decoded));
            }
            if let Ok(decoded) = <UpdateSocketCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateSocket(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for RegistryCoordinatorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::OperatorChurnApprovalTypehash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PubkeyRegistrationTypehash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BlsApkRegistry(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CalculateOperatorChurnApprovalDigestHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChurnApprover(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CreateQuorum(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DeregisterOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EjectOperator(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EjectionCooldown(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Ejector(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetCurrentQuorumBitmap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOperator(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetOperatorFromId(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetOperatorId(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetOperatorSetParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOperatorStatus(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::IndexRegistry(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsChurnApproverSaltUsed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LastEjectionTimestamp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NumRegistries(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PauseAll(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PausedWithIndex(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Paused(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PauserRegistry(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PubkeyRegistrationMessageHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QuorumCount(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::QuorumUpdateBlockNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterOperator(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RegisterOperatorWithChurn(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Registries(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ServiceManager(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetChurnApprover(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetEjectionCooldown(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetEjector(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetOperatorSetParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPauserRegistry(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StakeRegistry(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Unpause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateOperators(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateOperatorsForQuorum(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateSocket(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for RegistryCoordinatorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::OperatorChurnApprovalTypehash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PubkeyRegistrationTypehash(element) => ::core::fmt::Display::fmt(element, f),
                Self::BlsApkRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::CalculateOperatorChurnApprovalDigestHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChurnApprover(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateQuorum(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeregisterOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::EjectOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::EjectionCooldown(element) => ::core::fmt::Display::fmt(element, f),
                Self::Ejector(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCurrentQuorumBitmap(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOperatorFromId(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOperatorId(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOperatorSetParams(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::IsChurnApproverSaltUsed(element) => ::core::fmt::Display::fmt(element, f),
                Self::LastEjectionTimestamp(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::QuorumUpdateBlockNumber(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterOperatorWithChurn(element) => ::core::fmt::Display::fmt(element, f),
                Self::Registries(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::ServiceManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetChurnApprover(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetEjectionCooldown(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetEjector(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetOperatorSetParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPauserRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unpause(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateOperators(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateOperatorsForQuorum(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateSocket(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<OperatorChurnApprovalTypehashCall> for RegistryCoordinatorCalls {
        fn from(value: OperatorChurnApprovalTypehashCall) -> Self {
            Self::OperatorChurnApprovalTypehash(value)
        }
    }
    impl ::core::convert::From<PubkeyRegistrationTypehashCall> for RegistryCoordinatorCalls {
        fn from(value: PubkeyRegistrationTypehashCall) -> Self {
            Self::PubkeyRegistrationTypehash(value)
        }
    }
    impl ::core::convert::From<BlsApkRegistryCall> for RegistryCoordinatorCalls {
        fn from(value: BlsApkRegistryCall) -> Self {
            Self::BlsApkRegistry(value)
        }
    }
    impl ::core::convert::From<CalculateOperatorChurnApprovalDigestHashCall>
        for RegistryCoordinatorCalls
    {
        fn from(value: CalculateOperatorChurnApprovalDigestHashCall) -> Self {
            Self::CalculateOperatorChurnApprovalDigestHash(value)
        }
    }
    impl ::core::convert::From<ChurnApproverCall> for RegistryCoordinatorCalls {
        fn from(value: ChurnApproverCall) -> Self {
            Self::ChurnApprover(value)
        }
    }
    impl ::core::convert::From<CreateQuorumCall> for RegistryCoordinatorCalls {
        fn from(value: CreateQuorumCall) -> Self {
            Self::CreateQuorum(value)
        }
    }
    impl ::core::convert::From<DeregisterOperatorCall> for RegistryCoordinatorCalls {
        fn from(value: DeregisterOperatorCall) -> Self {
            Self::DeregisterOperator(value)
        }
    }
    impl ::core::convert::From<EjectOperatorCall> for RegistryCoordinatorCalls {
        fn from(value: EjectOperatorCall) -> Self {
            Self::EjectOperator(value)
        }
    }
    impl ::core::convert::From<EjectionCooldownCall> for RegistryCoordinatorCalls {
        fn from(value: EjectionCooldownCall) -> Self {
            Self::EjectionCooldown(value)
        }
    }
    impl ::core::convert::From<EjectorCall> for RegistryCoordinatorCalls {
        fn from(value: EjectorCall) -> Self {
            Self::Ejector(value)
        }
    }
    impl ::core::convert::From<GetCurrentQuorumBitmapCall> for RegistryCoordinatorCalls {
        fn from(value: GetCurrentQuorumBitmapCall) -> Self {
            Self::GetCurrentQuorumBitmap(value)
        }
    }
    impl ::core::convert::From<GetOperatorCall> for RegistryCoordinatorCalls {
        fn from(value: GetOperatorCall) -> Self {
            Self::GetOperator(value)
        }
    }
    impl ::core::convert::From<GetOperatorFromIdCall> for RegistryCoordinatorCalls {
        fn from(value: GetOperatorFromIdCall) -> Self {
            Self::GetOperatorFromId(value)
        }
    }
    impl ::core::convert::From<GetOperatorIdCall> for RegistryCoordinatorCalls {
        fn from(value: GetOperatorIdCall) -> Self {
            Self::GetOperatorId(value)
        }
    }
    impl ::core::convert::From<GetOperatorSetParamsCall> for RegistryCoordinatorCalls {
        fn from(value: GetOperatorSetParamsCall) -> Self {
            Self::GetOperatorSetParams(value)
        }
    }
    impl ::core::convert::From<GetOperatorStatusCall> for RegistryCoordinatorCalls {
        fn from(value: GetOperatorStatusCall) -> Self {
            Self::GetOperatorStatus(value)
        }
    }
    impl ::core::convert::From<GetQuorumBitmapAtBlockNumberByIndexCall> for RegistryCoordinatorCalls {
        fn from(value: GetQuorumBitmapAtBlockNumberByIndexCall) -> Self {
            Self::GetQuorumBitmapAtBlockNumberByIndex(value)
        }
    }
    impl ::core::convert::From<GetQuorumBitmapHistoryLengthCall> for RegistryCoordinatorCalls {
        fn from(value: GetQuorumBitmapHistoryLengthCall) -> Self {
            Self::GetQuorumBitmapHistoryLength(value)
        }
    }
    impl ::core::convert::From<GetQuorumBitmapIndicesAtBlockNumberCall> for RegistryCoordinatorCalls {
        fn from(value: GetQuorumBitmapIndicesAtBlockNumberCall) -> Self {
            Self::GetQuorumBitmapIndicesAtBlockNumber(value)
        }
    }
    impl ::core::convert::From<GetQuorumBitmapUpdateByIndexCall> for RegistryCoordinatorCalls {
        fn from(value: GetQuorumBitmapUpdateByIndexCall) -> Self {
            Self::GetQuorumBitmapUpdateByIndex(value)
        }
    }
    impl ::core::convert::From<IndexRegistryCall> for RegistryCoordinatorCalls {
        fn from(value: IndexRegistryCall) -> Self {
            Self::IndexRegistry(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for RegistryCoordinatorCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<IsChurnApproverSaltUsedCall> for RegistryCoordinatorCalls {
        fn from(value: IsChurnApproverSaltUsedCall) -> Self {
            Self::IsChurnApproverSaltUsed(value)
        }
    }
    impl ::core::convert::From<LastEjectionTimestampCall> for RegistryCoordinatorCalls {
        fn from(value: LastEjectionTimestampCall) -> Self {
            Self::LastEjectionTimestamp(value)
        }
    }
    impl ::core::convert::From<NumRegistriesCall> for RegistryCoordinatorCalls {
        fn from(value: NumRegistriesCall) -> Self {
            Self::NumRegistries(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for RegistryCoordinatorCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PauseCall> for RegistryCoordinatorCalls {
        fn from(value: PauseCall) -> Self {
            Self::Pause(value)
        }
    }
    impl ::core::convert::From<PauseAllCall> for RegistryCoordinatorCalls {
        fn from(value: PauseAllCall) -> Self {
            Self::PauseAll(value)
        }
    }
    impl ::core::convert::From<PausedWithIndexCall> for RegistryCoordinatorCalls {
        fn from(value: PausedWithIndexCall) -> Self {
            Self::PausedWithIndex(value)
        }
    }
    impl ::core::convert::From<PausedCall> for RegistryCoordinatorCalls {
        fn from(value: PausedCall) -> Self {
            Self::Paused(value)
        }
    }
    impl ::core::convert::From<PauserRegistryCall> for RegistryCoordinatorCalls {
        fn from(value: PauserRegistryCall) -> Self {
            Self::PauserRegistry(value)
        }
    }
    impl ::core::convert::From<PubkeyRegistrationMessageHashCall> for RegistryCoordinatorCalls {
        fn from(value: PubkeyRegistrationMessageHashCall) -> Self {
            Self::PubkeyRegistrationMessageHash(value)
        }
    }
    impl ::core::convert::From<QuorumCountCall> for RegistryCoordinatorCalls {
        fn from(value: QuorumCountCall) -> Self {
            Self::QuorumCount(value)
        }
    }
    impl ::core::convert::From<QuorumUpdateBlockNumberCall> for RegistryCoordinatorCalls {
        fn from(value: QuorumUpdateBlockNumberCall) -> Self {
            Self::QuorumUpdateBlockNumber(value)
        }
    }
    impl ::core::convert::From<RegisterOperatorCall> for RegistryCoordinatorCalls {
        fn from(value: RegisterOperatorCall) -> Self {
            Self::RegisterOperator(value)
        }
    }
    impl ::core::convert::From<RegisterOperatorWithChurnCall> for RegistryCoordinatorCalls {
        fn from(value: RegisterOperatorWithChurnCall) -> Self {
            Self::RegisterOperatorWithChurn(value)
        }
    }
    impl ::core::convert::From<RegistriesCall> for RegistryCoordinatorCalls {
        fn from(value: RegistriesCall) -> Self {
            Self::Registries(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for RegistryCoordinatorCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<ServiceManagerCall> for RegistryCoordinatorCalls {
        fn from(value: ServiceManagerCall) -> Self {
            Self::ServiceManager(value)
        }
    }
    impl ::core::convert::From<SetChurnApproverCall> for RegistryCoordinatorCalls {
        fn from(value: SetChurnApproverCall) -> Self {
            Self::SetChurnApprover(value)
        }
    }
    impl ::core::convert::From<SetEjectionCooldownCall> for RegistryCoordinatorCalls {
        fn from(value: SetEjectionCooldownCall) -> Self {
            Self::SetEjectionCooldown(value)
        }
    }
    impl ::core::convert::From<SetEjectorCall> for RegistryCoordinatorCalls {
        fn from(value: SetEjectorCall) -> Self {
            Self::SetEjector(value)
        }
    }
    impl ::core::convert::From<SetOperatorSetParamsCall> for RegistryCoordinatorCalls {
        fn from(value: SetOperatorSetParamsCall) -> Self {
            Self::SetOperatorSetParams(value)
        }
    }
    impl ::core::convert::From<SetPauserRegistryCall> for RegistryCoordinatorCalls {
        fn from(value: SetPauserRegistryCall) -> Self {
            Self::SetPauserRegistry(value)
        }
    }
    impl ::core::convert::From<StakeRegistryCall> for RegistryCoordinatorCalls {
        fn from(value: StakeRegistryCall) -> Self {
            Self::StakeRegistry(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for RegistryCoordinatorCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UnpauseCall> for RegistryCoordinatorCalls {
        fn from(value: UnpauseCall) -> Self {
            Self::Unpause(value)
        }
    }
    impl ::core::convert::From<UpdateOperatorsCall> for RegistryCoordinatorCalls {
        fn from(value: UpdateOperatorsCall) -> Self {
            Self::UpdateOperators(value)
        }
    }
    impl ::core::convert::From<UpdateOperatorsForQuorumCall> for RegistryCoordinatorCalls {
        fn from(value: UpdateOperatorsForQuorumCall) -> Self {
            Self::UpdateOperatorsForQuorum(value)
        }
    }
    impl ::core::convert::From<UpdateSocketCall> for RegistryCoordinatorCalls {
        fn from(value: UpdateSocketCall) -> Self {
            Self::UpdateSocket(value)
        }
    }
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
        Hash,
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
        Hash,
    )]
    pub struct PubkeyRegistrationTypehashReturn(pub [u8; 32]);
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
        Hash,
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
        Hash,
    )]
    pub struct ChurnApproverReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `ejectionCooldown` function with signature `ejectionCooldown()` and selector `0xa96f783e`
    #[derive(
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
    pub struct EjectionCooldownReturn(pub ::ethers::core::types::U256);
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
        Hash,
    )]
    pub struct EjectorReturn(pub ::ethers::core::types::Address);
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    pub struct GetQuorumBitmapAtBlockNumberByIndexReturn(pub ::ethers::core::types::U256);
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    pub struct IsChurnApproverSaltUsedReturn(pub bool);
    ///Container type for all return fields from the `lastEjectionTimestamp` function with signature `lastEjectionTimestamp(address)` and selector `0x125e0584`
    #[derive(
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
    pub struct LastEjectionTimestampReturn(pub ::ethers::core::types::U256);
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    pub struct StakeRegistryReturn(pub ::ethers::core::types::Address);
    ///`OperatorKickParam(uint8,address)`
    #[derive(
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
    pub struct OperatorKickParam {
        pub quorum_number: u8,
        pub operator: ::ethers::core::types::Address,
    }
}
