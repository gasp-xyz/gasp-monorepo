pub use registry_coordinator::*;
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
    pub static REGISTRYCOORDINATOR_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01\xC0`@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@Qb\0b\x8C8\x03\x80b\0b\x8C\x839\x81\x01`@\x81\x90Rb\0\x005\x91b\0\x02TV[`@\x80Q\x80\x82\x01\x82R`\x16\x81R\x7FAVSRegistryCoordinator\0\0\0\0\0\0\0\0\0\0` \x80\x83\x01\x91\x82R\x83Q\x80\x85\x01\x90\x94R`\x06\x84Rev0.0.1`\xD0\x1B\x90\x84\x01R\x81Q\x90 `\xE0\x81\x90R\x7Fk\xDA~?8^H\x84\x10H9\x04D\xCC\xED\\\xC7\x95\xAF\x87u\x8A\xF6v\"\xE5\xF4\xF0\x88,J\x99a\x01\0\x81\x90RF`\xA0R\x87\x93\x87\x93\x87\x93\x87\x93\x91\x92\x91\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0Fb\0\x015\x81\x84\x84`@\x80Q` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90RF`\x80\x82\x01R0`\xA0\x82\x01R`\0\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x93\x92PPPV[`\x80R0`\xC0Ra\x01 RPPPP`\x01`\x01`\xA0\x1B\x03\x93\x84\x16a\x01@R\x91\x83\x16a\x01\x80R\x82\x16a\x01`R\x16a\x01\xA0Rb\0\x01ob\0\x01yV[PPPPb\0\x02\xBCV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15b\0\x01\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\xFF\x90\x81\x16\x10\x15b\0\x029W`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x02QW`\0\x80\xFD[PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15b\0\x02kW`\0\x80\xFD[\x84Qb\0\x02x\x81b\0\x02;V[` \x86\x01Q\x90\x94Pb\0\x02\x8B\x81b\0\x02;V[`@\x86\x01Q\x90\x93Pb\0\x02\x9E\x81b\0\x02;V[``\x86\x01Q\x90\x92Pb\0\x02\xB1\x81b\0\x02;V[\x93\x96\x92\x95P\x90\x93PPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x01@Qa\x01`Qa\x01\x80Qa\x01\xA0Qa^\xC8b\0\x03\xC4`\09`\0\x81\x81a\x06$\x01R\x81\x81a\x11\x8C\x01R\x81\x81a I\x01R\x81\x81a-\xEC\x01R\x81\x81a6s\x01Ra<K\x01R`\0\x81\x81a\x05|\x01R\x81\x81a\x1F\xD4\x01R\x81\x81a$T\x01R\x81\x81a-l\x01R\x81\x81a5\xCA\x01R\x81\x81a8 \x01Ra;\xCA\x01R`\0\x81\x81a\x05B\x01R\x81\x81a\x0E\xAE\x01R\x81\x81a \x12\x01R\x81\x81a,\xEE\x01R\x81\x81a.\xD4\x01R\x81\x81a/J\x01R\x81\x81a5J\x01Ra<\xC7\x01R`\0\x81\x81a\x04\x86\x01R\x81\x81a,D\x01Ra4\x92\x01R`\0a>\xA3\x01R`\0a>\xF2\x01R`\0a>\xCD\x01R`\0a>&\x01R`\0a>P\x01R`\0a>z\x01Ra^\xC8`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\x94W`\x005`\xE0\x1C\x80c]\xF4YF\x11a\x01gW\x80c\xA5\x08W\xBF\x11a\0\xCEW\x80c\xDD\x82\x83\xF3\x11a\0\x87W\x80c\xDD\x82\x83\xF3\x14a\x06\xF5W\x80c\xE6W\x97\xAD\x14a\x07\x08W\x80c\xF2\xFD\xE3\x8B\x14a\x07\xABW\x80c\xF8X\x11\x91\x14a\x07\xBEW\x80c\xFA\xBC\x1C\xBC\x14a\x07\xD1W\x80c\xFD9\x10Z\x14a\x07\xE4W`\0\x80\xFD[\x80c\xA5\x08W\xBF\x14a\x06mW\x80c\xC3\x91B^\x14a\x06\x80W\x80c\xCA\r\xE8\x82\x14a\x06\xA0W\x80c\xCAO-\x97\x14a\x06\xC7W\x80c\xD7-\x8D\xD6\x14a\x06\xDAW\x80c\xD7[L\x88\x14a\x06\xE2W`\0\x80\xFD[\x80c\x88o\x11\x95\x11a\x01 W\x80c\x88o\x11\x95\x14a\x05\xCCW\x80c\x8D\xA5\xCB[\x14a\x05\xE5W\x80c\x9A\xA1e=\x14a\x05\xEDW\x80c\x9B]\x17{\x14a\x06\x0CW\x80c\x9E\x99#\xC2\x14a\x06\x1FW\x80c\x9F\xEA\xB8Y\x14a\x06FW`\0\x80\xFD[\x80c]\xF4YF\x14a\x05=W\x80ccG\xC9\0\x14a\x05dW\x80ch0H5\x14a\x05wW\x80cn;\x17\xDB\x14a\x05\x9EW\x80cqP\x18\xA6\x14a\x05\xB1W\x80c\x87\x1E\xF0I\x14a\x05\xB9W`\0\x80\xFD[\x80c(\xF6\x1B1\x11a\x02\x0BW\x80cQ@\xA5H\x11a\x01\xC4W\x80cQ@\xA5H\x14a\x04\xC8W\x80cXe\xC6\x0C\x14a\x04\xDBW\x80cY\\jg\x14a\x04\xFBW\x80cZ\xC8j\xB7\x14a\x05\x03W\x80c[\x0B\x82\x9F\x14a\x05\"W\x80c\\\x97Z\xBB\x14a\x055W`\0\x80\xFD[\x80c(\xF6\x1B1\x14a\x045W\x80c)k\xB0d\x14a\x04HW\x80c)\xD1\xE0\xC3\x14a\x04[W\x80c,\xDD\x1E\x86\x14a\x04nW\x80c9\x98\xFD\xD3\x14a\x04\x81W\x80c<*\x7FL\x14a\x04\xA8W`\0\x80\xFD[\x80c\x10\xD6z/\x11a\x02]W\x80c\x10\xD6z/\x14a\x03JW\x80c\x13T*N\x14a\x03]W\x80c\x13d9\xDD\x14a\x03\x86W\x80c\x14x\x85\x1F\x14a\x03\x99W\x80c\x1E\xB8\x12\xDA\x14a\x03\xCCW\x80c$\x9A\x0CB\x14a\x04\x15W`\0\x80\xFD[\x80b\xCF*\xB5\x14a\x02\x99W\x80c\x03\xFD4\x92\x14a\x02\xAEW\x80c\x04\xECcQ\x14a\x02\xE1W\x80c\x05C\x10\xE6\x14a\x03\x0CW\x80c\x0C\xF4\xB7g\x14a\x037W[`\0\x80\xFD[a\x02\xACa\x02\xA76`\x04aI\xA3V[a\x08 V[\0[a\x02\xCEa\x02\xBC6`\x04aI\xE4V[`\0\x90\x81R`\x98` R`@\x90 T\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\xF4a\x02\xEF6`\x04aJ\x0FV[a\t6V[`@Q`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xD8V[`\x9DTa\x03\x1F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xD8V[a\x02\xACa\x03E6`\x04aK.V[a\x0B,V[a\x02\xACa\x03X6`\x04aK\xA3V[a\x0C\x14V[a\x02\xCEa\x03k6`\x04aK\xA3V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x99` R`@\x90 T\x90V[a\x02\xACa\x03\x946`\x04aI\xE4V[a\x0C\xC7V[a\x03\xBCa\x03\xA76`\x04aI\xE4V[`\x9A` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02\xD8V[a\x03\xDFa\x03\xDA6`\x04aK\xC0V[a\x0E\x04V[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x80\x85\x01Q\x90\x91\x16\x90\x82\x01R\x91\x81\x01Q`\x01`\x01`\xC0\x1B\x03\x16\x90\x82\x01R``\x01a\x02\xD8V[a\x02\xCEa\x04#6`\x04aK\xF3V[`\x9B` R`\0\x90\x81R`@\x90 T\x81V[`\x9ETa\x03\x1F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03\x1Fa\x04V6`\x04aI\xE4V[a\x0E\x95V[a\x02\xACa\x04i6`\x04aK\xA3V[a\x0F!V[a\x02\xACa\x04|6`\x04aK\xA3V[a\x0F2V[a\x03\x1F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x04\xBBa\x04\xB66`\x04aK\xA3V[a\x0FCV[`@Qa\x02\xD8\x91\x90aL\x0EV[a\x02\xACa\x04\xD66`\x04aLfV[a\x0F\xC2V[a\x04\xEEa\x04\xE96`\x04aK\xA3V[a\x15LV[`@Qa\x02\xD8\x91\x90aM\tV[a\x02\xACa\x15\xC0V[a\x03\xBCa\x05\x116`\x04aK\xF3V[`\x01\x80T`\xFF\x90\x92\x16\x1B\x90\x81\x16\x14\x90V[a\x02\xACa\x0506`\x04aM\x8EV[a\x16\x8CV[`\x01Ta\x02\xCEV[a\x03\x1F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\x1Fa\x05r6`\x04aI\xE4V[a\x17#V[a\x03\x1F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xACa\x05\xAC6`\x04aM\xC2V[a\x17MV[a\x02\xACa\x18\rV[a\x02\xF4a\x05\xC76`\x04aI\xE4V[a\x18!V[`\0Ta\x03\x1F\x90b\x01\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03\x1Fa\x18,V[`\x96Ta\x05\xFA\x90`\xFF\x16\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02\xD8V[a\x02\xACa\x06\x1A6`\x04aN\xE2V[a\x18EV[a\x03\x1F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xCE\x7F+\xD8!$\x05\x7F\t\x13\xBC;w,\xE7\xB8>\x80W\xC1\xAD\x1F5\x10\xFC\x83w\x8B\xE2\x0F\x10\xEC]\xE6\x81V[a\x02\xACa\x06{6`\x04aO\xDBV[a\x1B|V[a\x06\x93a\x06\x8E6`\x04aP\xA6V[a\x1D\0V[`@Qa\x02\xD8\x91\x90aQPV[a\x02\xCE\x7F\xF8C\xB3\x11mWOC\xE6\x9F\x8D\xDA]\x93\xEB\xF1\x1D\xCC\xC4\xA4e\x98?\x94S\x05\x80\x05\xCDk4\xA0\x81V[a\x02\xACa\x06\xD56`\x04aQ\x9AV[a\x1D\xB9V[`\x9CTa\x02\xCEV[a\x02\xACa\x06\xF06`\x04aR\x80V[a\x1E V[a\x02\xACa\x07\x036`\x04aT3V[a\x1E3V[a\x07wa\x07\x166`\x04aK\xF3V[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x92\x84\x01\x81\x90R`\xFF\x94\x90\x94\x16\x84R`\x97\x82R\x92\x82\x90 \x82Q\x93\x84\x01\x83RTc\xFF\xFF\xFF\xFF\x81\x16\x84Ra\xFF\xFF`\x01` \x1B\x82\x04\x81\x16\x92\x85\x01\x92\x90\x92R`\x01`0\x1B\x90\x04\x16\x90\x82\x01R\x90V[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x16\x81R` \x80\x84\x01Qa\xFF\xFF\x90\x81\x16\x91\x83\x01\x91\x90\x91R\x92\x82\x01Q\x90\x92\x16\x90\x82\x01R``\x01a\x02\xD8V[a\x02\xACa\x07\xB96`\x04aK\xA3V[a!7V[a\x02\xCEa\x07\xCC6`\x04aUGV[a!\xADV[a\x02\xACa\x07\xDF6`\x04aI\xE4V[a!\xF4V[a\x08\x13a\x07\xF26`\x04aK\xA3V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T`\xFF\x16\x90V[`@Qa\x02\xD8\x91\x90aU\xFCV[`\x01T`\x02\x90`\x04\x90\x81\x16\x14\x15a\x08RW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08I\x90aV\nV[`@Q\x80\x91\x03\x90\xFD[`\0[\x82\x81\x10\x15a\t0W`\0\x84\x84\x83\x81\x81\x10a\x08qWa\x08qaVAV[\x90P` \x02\x01` \x81\x01\x90a\x08\x86\x91\x90aK\xA3V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x99` \x90\x81R`@\x80\x83 \x81Q\x80\x83\x01\x90\x92R\x80T\x82R`\x01\x81\x01T\x94\x95P\x92\x93\x90\x92\x91\x83\x01\x90`\xFF\x16`\x02\x81\x11\x15a\x08\xD1Wa\x08\xD1aL\xD1V[`\x02\x81\x11\x15a\x08\xE2Wa\x08\xE2aL\xD1V[\x90RP\x80Q\x90\x91P`\0a\x08\xF5\x82a#PV[\x90P`\0a\t\x0B\x82`\x01`\x01`\xC0\x1B\x03\x16a#\xB9V[\x90Pa\t\x18\x85\x85\x83a$\x16V[PPPPP\x80\x80a\t(\x90aVmV[\x91PPa\x08UV[PPPPV[`\0\x83\x81R`\x98` R`@\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\tYWa\tYaVAV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x85R`\x01` \x1B\x83\x04\x82\x16\x95\x85\x01\x95\x90\x95R`\x01`@\x1B\x90\x91\x04`\x01`\x01`\xC0\x1B\x03\x16\x91\x83\x01\x91\x90\x91R\x90\x92P\x85\x16\x10\x15a\nSW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`e`$\x82\x01R\x7FRegistryCoordinator.getQuorumBit`D\x82\x01R\x7FmapAtBlockNumberByIndex: quorumB`d\x82\x01R\x7FitmapUpdate is from after blockN`\x84\x82\x01Rd:\xB6\xB12\xB9`\xD9\x1B`\xA4\x82\x01R`\xC4\x01a\x08IV[` \x81\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x80a\nyWP\x80` \x01Qc\xFF\xFF\xFF\xFF\x16\x84c\xFF\xFF\xFF\xFF\x16\x10[a\x0B W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`f`$\x82\x01R\x7FRegistryCoordinator.getQuorumBit`D\x82\x01R\x7FmapAtBlockNumberByIndex: quorumB`d\x82\x01R\x7FitmapUpdate is from before block`\x84\x82\x01Re':\xB6\xB12\xB9`\xD1\x1B`\xA4\x82\x01R`\xC4\x01a\x08IV[`@\x01Q\x94\x93PPPPV[`\x013`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T`\xFF\x16`\x02\x81\x11\x15a\x0BUWa\x0BUaL\xD1V[\x14a\x0B\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R\x7FRegistryCoordinator.updateSocket`D\x82\x01R\x7F: operator is not registered\0\0\0\0`d\x82\x01R`\x84\x01a\x08IV[3`\0\x90\x81R`\x99` R`@\x90\x81\x90 T\x90Q\x7F\xEC)c\xAB!\xC1\xE5\x0E\x1EX*\xA5B\xAF.K\xF7\xBF8\xE6\xE1@<'\xB4.\x1C]nb\x1E\xAA\x90a\x0C\t\x90\x84\x90aV\xE0V[`@Q\x80\x91\x03\x90\xA2PV[`\0`\x02\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CgW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x8B\x91\x90aV\xF3V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0C\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08I\x90aW\x10V[a\x0C\xC4\x81a%\x03V[PV[`\0T`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01Rb\x01\0\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\x14W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r8\x91\x90aWZV[a\rTW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08I\x90aW|V[`\x01T\x81\x81\x16\x14a\r\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08IV[`\x01\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01a\x0C\tV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R`\0\x83\x81R`\x98` R`@\x90 \x80T\x83\x90\x81\x10a\x0EAWa\x0EAaVAV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01`\xC0\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90P[\x92\x91PPV[`@Qc\x08\xF6b\x9D`\xE3\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cG\xB3\x14\xE8\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xFDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\x8F\x91\x90aV\xF3V[a\x0F)a&\x08V[a\x0C\xC4\x81a&gV[a\x0F:a&\x08V[a\x0C\xC4\x81a&\xD0V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x0E\x8Fa\x0F\xBD\x7F+\xD8!$\x05\x7F\t\x13\xBC;w,\xE7\xB8>\x80W\xC1\xAD\x1F5\x10\xFC\x83w\x8B\xE2\x0F\x10\xEC]\xE6\x84`@Q` \x01a\x0F\xA2\x92\x91\x90\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a'9V[a'\x87V[`\x01T`\x02\x90`\x04\x90\x81\x16\x14\x15a\x0F\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08I\x90aV\nV[`\0a\x103\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`\x96T`\xFF\x16\x91Pa(\x17\x90PV[\x90Pa\x10>\x81a(\xD0V[a\x10\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R`\0\x80Q` a^\x13\x839\x81Q\x91R`D\x82\x01R\x7ForsForQuorum: some quorums do no`d\x82\x01Rf\x1D\x08\x19^\x1A\\\xDD`\xCA\x1B`\x84\x82\x01R`\xA4\x01a\x08IV[\x84\x83\x14a\x11\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R`\0\x80Q` a^\x13\x839\x81Q\x91R`D\x82\x01R\x7ForsForQuorum: input length misma`d\x82\x01Rb\x0E\x8Cm`\xEB\x1B`\x84\x82\x01R`\xA4\x01a\x08IV[`\0[\x83\x81\x10\x15a\x15CW`\0\x85\x85\x83\x81\x81\x10a\x11<Wa\x11<aVAV[\x91\x90\x91\x015`\xF8\x1C\x91P6\x90P`\0\x89\x89\x85\x81\x81\x10a\x11]Wa\x11]aVAV[\x90P` \x02\x81\x01\x90a\x11o\x91\x90aW\xC4V[`@Qcy\xA0\x84\x91`\xE1\x1B\x81R`\xFF\x86\x16`\x04\x82\x01R\x91\x93P\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xF3A\t\"\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xDBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xFF\x91\x90aX\rV[c\xFF\xFF\xFF\xFF\x16\x81\x14a\x12\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`e`$\x82\x01R`\0\x80Q` a^\x13\x839\x81Q\x91R`D\x82\x01R\x7ForsForQuorum: number of updated `d\x82\x01R\x7Foperators does not match quorum `\x84\x82\x01Rd\x1D\x1B\xDD\x18[`\xDA\x1B`\xA4\x82\x01R`\xC4\x01a\x08IV[`\0\x80[\x82\x81\x10\x15a\x14\xE2W`\0\x84\x84\x83\x81\x81\x10a\x12\xBBWa\x12\xBBaVAV[\x90P` \x02\x01` \x81\x01\x90a\x12\xD0\x91\x90aK\xA3V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x99` \x90\x81R`@\x80\x83 \x81Q\x80\x83\x01\x90\x92R\x80T\x82R`\x01\x81\x01T\x94\x95P\x92\x93\x90\x92\x91\x83\x01\x90`\xFF\x16`\x02\x81\x11\x15a\x13\x1BWa\x13\x1BaL\xD1V[`\x02\x81\x11\x15a\x13,Wa\x13,aL\xD1V[\x90RP\x80Q\x90\x91P`\0a\x13?\x82a#PV[\x90P`\x01`\x01`\x01`\xC0\x1B\x03\x82\x16`\xFF\x8B\x16\x1C\x81\x16\x14a\x13\xC3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R`\0\x80Q` a^\x13\x839\x81Q\x91R\x90\x82\x01R\x7ForsForQuorum: operator not in qu`d\x82\x01Rcorum`\xE0\x1B`\x84\x82\x01R`\xA4\x01a\x08IV[\x85`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x11a\x14nW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`g`$\x82\x01R`\0\x80Q` a^\x13\x839\x81Q\x91R`D\x82\x01R\x7ForsForQuorum: operators array mu`d\x82\x01R\x7Fst be sorted in ascending addres`\x84\x82\x01Rf9\x907\xB922\xB9`\xC9\x1B`\xA4\x82\x01R`\xC4\x01a\x08IV[Pa\x14\xCC\x83\x83\x8F\x8F\x8D\x90\x8E`\x01a\x14\x85\x91\x90aX*V[\x92a\x14\x92\x93\x92\x91\x90aXBV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa$\x16\x92PPPV[P\x90\x92Pa\x14\xDB\x90P\x81aVmV[\x90Pa\x12\x9FV[P`\xFF\x84\x16`\0\x81\x81R`\x9B` \x90\x81R`@\x91\x82\x90 C\x90\x81\x90U\x91Q\x91\x82R\x7FF\x07}U3\x07c\xF1bi\xFDu\xE5v\x16c\xF4\x19-'\x91t|\x01\x89\xB1j\xD3\x1D\xB0}\xB4\x91\x01`@Q\x80\x91\x03\x90\xA2PPPP\x80a\x15<\x90aVmV[\x90Pa\x11 V[PPPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x99` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x81\x01T\x90\x91\x83\x01\x90`\xFF\x16`\x02\x81\x11\x15a\x15\xA6Wa\x15\xA6aL\xD1V[`\x02\x81\x11\x15a\x15\xB7Wa\x15\xB7aL\xD1V[\x90RP\x92\x91PPV[`\0T`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01Rb\x01\0\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x161\x91\x90aWZV[a\x16MW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08I\x90aW|V[`\0\x19`\x01\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[a\x16\x94a&\x08V[`\x96T\x82\x90`\xFF\x90\x81\x16\x90\x82\x16\x10a\x17\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FRegistryCoordinator.quorumExists`D\x82\x01R\x7F: quorum does not exist\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08IV[a\x17\x1E\x83\x83a)\x03V[PPPV[`\x9C\x81\x81T\x81\x10a\x173W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x81V[`\x9ET`\x01`\x01`\xA0\x1B\x03\x163\x14a\x17\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FRegistryCoordinator.onlyEjector:`D\x82\x01R\x7F caller is not the ejector\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08IV[a\x17\x1E\x83\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa)\xB0\x92PPPV[a\x18\x15a&\x08V[a\x18\x1F`\0a.`V[V[`\0a\x0E\x8F\x82a#PV[`\0a\x18@`dT`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x90P\x90V[`\x01\x80T`\0\x91\x90\x81\x16\x14\x15a\x18mW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08I\x90aV\nV[\x83\x89\x14a\x18\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FRegistryCoordinator.registerOper\x90\x82\x01R\x7FatorWithChurn: input length mism`d\x82\x01Rc\x0C.\x8Cm`\xE3\x1B`\x84\x82\x01R`\xA4\x01a\x08IV[`\0a\x18\xFC3\x88a.\xB2V[\x90Pa\x19[\x81\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x19PWa\x19A`@\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aXlV[\x81R` \x01\x90`\x01\x01\x90a\x19$V[PPPPP\x86a/\xE3V[`\0a\x19\xA23\x83\x8E\x8E\x8E\x8E\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8C\x92Pa1m\x91PPV[\x90P`\0[\x8B\x81\x10\x15a\x1BmW`\0`\x97`\0\x8F\x8F\x85\x81\x81\x10a\x19\xC7Wa\x19\xC7aVAV[\x91\x90\x91\x015`\xF8\x1C\x82RP` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0 \x81Q``\x81\x01\x83R\x90Tc\xFF\xFF\xFF\xFF\x81\x16\x80\x83Ra\xFF\xFF`\x01` \x1B\x83\x04\x81\x16\x95\x84\x01\x95\x90\x95R`\x01`0\x1B\x90\x91\x04\x90\x93\x16\x91\x81\x01\x91\x90\x91R\x84Q\x80Q\x91\x93P\x90\x84\x90\x81\x10a\x1A4Wa\x1A4aVAV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\x1BZWa\x1A\xD5\x8E\x8E\x84\x81\x81\x10a\x1A]Wa\x1A]aVAV[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x84`@\x01Q\x84\x81Q\x81\x10a\x1A\x80Wa\x1A\x80aVAV[` \x02` \x01\x01Q3\x86` \x01Q\x86\x81Q\x81\x10a\x1A\x9FWa\x1A\x9FaVAV[` \x02` \x01\x01Q\x8D\x8D\x88\x81\x81\x10a\x1A\xB9Wa\x1A\xB9aVAV[\x90P`@\x02\x01\x806\x03\x81\x01\x90a\x1A\xCF\x91\x90aXlV[\x86a7\x01V[a\x1BZ\x89\x89\x84\x81\x81\x10a\x1A\xEAWa\x1A\xEAaVAV[\x90P`@\x02\x01` \x01` \x81\x01\x90a\x1B\x02\x91\x90aK\xA3V[\x8F\x8F\x85\x90\x86`\x01a\x1B\x13\x91\x90aX*V[\x92a\x1B \x93\x92\x91\x90aXBV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa)\xB0\x92PPPV[P\x80a\x1Be\x81aVmV[\x91PPa\x19\xA7V[PPPPPPPPPPPPPV[`\x01\x80T`\0\x91\x90\x81\x16\x14\x15a\x1B\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08I\x90aV\nV[`\0a\x1B\xB03\x85a.\xB2V[\x90P`\0a\x1B\xF93\x83\x8B\x8B\x8B\x8B\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8C\x92Pa1m\x91PPV[Q\x90P`\0[\x88\x81\x10\x15a\x1C\xF4W`\0\x8A\x8A\x83\x81\x81\x10a\x1C\x1BWa\x1C\x1BaVAV[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x97` R`@\x90 T\x85Q\x91\x93Pc\xFF\xFF\xFF\xFF\x16\x91P\x84\x90\x84\x90\x81\x10a\x1CQWa\x1CQaVAV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\x1C\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FRegistryCoordinator.registerOper\x90\x82\x01R\x7Fator: operator count exceeds max`d\x82\x01Rcimum`\xE0\x1B`\x84\x82\x01R`\xA4\x01a\x08IV[P\x80a\x1C\xEC\x81aVmV[\x91PPa\x1B\xFFV[PPPPPPPPPPV[```\0\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\x1DWa\x1D\x1DaJGV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1DFW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\x1D\xB1Wa\x1Dx\x85\x85\x83\x81Q\x81\x10a\x1DkWa\x1DkaVAV[` \x02` \x01\x01Qa9\xD6V[\x82\x82\x81Q\x81\x10a\x1D\x8AWa\x1D\x8AaVAV[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x1D\xA9\x81aVmV[\x91PPa\x1DLV[P\x93\x92PPPV[`\x01\x80T`\x02\x90\x81\x16\x14\x15a\x1D\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08I\x90aV\nV[a\x17\x1E3\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa)\xB0\x92PPPV[a\x1E(a&\x08V[a\x17\x1E\x83\x83\x83a;\x12V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x1ESWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x1EmWP0;\x15\x80\x15a\x1EmWP`\0T`\xFF\x16`\x01\x14[a\x1E\xD0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x08IV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x1E\xF3W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[\x82Q\x84Q\x14\x80\x15a\x1F\x05WP\x81Q\x83Q\x14[a\x1FoW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FRegistryCoordinator.initialize: `D\x82\x01Rt\r-\xCE\x0E\xAE\x84\r\x8C\xAD\xCC\xEE\x8D\x04\r\xAD.m\xAC.\x8Cm`[\x1B`d\x82\x01R`\x84\x01a\x08IV[a\x1Fx\x89a.`V[a\x1F\x82\x86\x86a=)V[a\x1F\x8B\x88a&gV[a\x1F\x94\x87a&\xD0V[`\x9C\x80T`\x01\x81\x81\x01\x83U`\0\x83\x81R\x7F\xAF\x85\xB9\x07\x1D\xFA\xFE\xAC\x14\t\xD3\xF1\xD1\x9B\xAF\xC9\xBC|7\x97L\xDE\x8D\xF0\xEEah\xF0\x08nS\x9C\x92\x83\x01\x80T`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U\x85T\x80\x85\x01\x87U\x85\x01\x80T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x16\x90\x83\x16\x17\x90U\x85T\x93\x84\x01\x90\x95U\x91\x90\x92\x01\x80T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x16\x91\x90\x93\x16\x17\x90\x91U[\x84Q\x81\x10\x15a \xE5Wa \xD3\x85\x82\x81Q\x81\x10a \x92Wa \x92aVAV[` \x02` \x01\x01Q\x85\x83\x81Q\x81\x10a \xACWa \xACaVAV[` \x02` \x01\x01Q\x85\x84\x81Q\x81\x10a \xC6Wa \xC6aVAV[` \x02` \x01\x01Qa;\x12V[\x80a \xDD\x81aVmV[\x91PPa tV[P\x80\x15a!,W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPPPV[a!?a&\x08V[`\x01`\x01`\xA0\x1B\x03\x81\x16a!\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x08IV[a\x0C\xC4\x81a.`V[`\0a!\xEB\x7F\xF8C\xB3\x11mWOC\xE6\x9F\x8D\xDA]\x93\xEB\xF1\x1D\xCC\xC4\xA4e\x98?\x94S\x05\x80\x05\xCDk4\xA0\x86\x86\x86\x86`@Q` \x01a\x0F\xA2\x95\x94\x93\x92\x91\x90aX\x88V[\x95\x94PPPPPV[`\0`\x02\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"GW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"k\x91\x90aV\xF3V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\"\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08I\x90aW\x10V[`\x01T\x19\x81\x19`\x01T\x19\x16\x14a#\x19W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08IV[`\x01\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x0C\tV[`\0\x81\x81R`\x98` R`@\x81 T\x80a#mWP`\0\x92\x91PPV[`\0\x83\x81R`\x98` R`@\x90 a#\x86`\x01\x83aX\xFEV[\x81T\x81\x10a#\x96Wa#\x96aVAV[`\0\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01`\xC0\x1B\x03\x16\x93\x92PPPV[```\0\x80[a\x01\0\x81\x10\x15a$\x0FW`\x01\x81\x1B\x91P\x83\x82\x16\x15a#\xFFW\x82\x81`\xF8\x1B`@Q` \x01a#\xED\x92\x91\x90aY\x15V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92P[a$\x08\x81aVmV[\x90Pa#\xBFV[PP\x91\x90PV[`\x01\x82` \x01Q`\x02\x81\x11\x15a$.Wa$.aL\xD1V[\x14a$8WPPPV[\x81Q`@Qc3V\x7F\x7F`\xE1\x1B\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cf\xAC\xFE\xFE\x90a$\x8D\x90\x88\x90\x86\x90\x88\x90`\x04\x01aYDV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a$\xACW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$\xD0\x91\x90aYkV[\x90P`\x01`\x01`\xC0\x1B\x03\x81\x16\x15a$\xFCWa$\xFC\x85a$\xF7\x83`\x01`\x01`\xC0\x1B\x03\x16a#\xB9V[a)\xB0V[PPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16a%\x91W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x08IV[`\0T`@\x80Q`\x01`\x01`\xA0\x1B\x03b\x01\0\0\x90\x93\x04\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`\0\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16b\x01\0\0\x02b\x01\0\0`\x01`\xB0\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[3a&\x11a\x18,V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x18\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x08IV[`\x9DT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F1TW\xD8\xA8\xFE`\xF0J\xF1|\x16\xE2\xF5\xA5\xE1\xDBa+1d\x8EX\x03\x03`u\x9E\xF8\xF3R\x8C\x91\x01`@Q\x80\x91\x03\x90\xA1`\x9D\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x9ET`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F\x8F0\xAB\t\xF4:l\x15}\x7F\xCE~\n\x13\xC0\x03\x04,\x1C\x95\xE8\xA7.z\x14j!\xC0\xCA\xA2M\xC9\x91\x01`@Q\x80\x91\x03\x90\xA1`\x9E\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0a\x0E\x8Fa'Fa>\x19V[\x83`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x83\x90R`B\x81\x01\x82\x90R`\0\x90`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0\x80\x80a'\xB7`\0\x80Q` a^S\x839\x81Q\x91R\x86aY\xAAV[\x90P[a'\xC3\x81a?@V[\x90\x93P\x91P`\0\x80Q` a^S\x839\x81Q\x91R\x82\x83\t\x83\x14\x15a'\xFDW`@\x80Q\x80\x82\x01\x90\x91R\x90\x81R` \x81\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80Q` a^S\x839\x81Q\x91R`\x01\x82\x08\x90Pa'\xBAV[`\0\x80a(#\x84a?\xC2V[\x90P\x80\x15a(\xC9W\x82`\xFF\x16\x84`\x01\x86Qa(>\x91\x90aX\xFEV[\x81Q\x81\x10a(NWa(NaVAV[\x01` \x01Q`\xF8\x1C\x10a(\xC9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`?`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: bitmap exceeds max value\0`d\x82\x01R`\x84\x01a\x08IV[\x93\x92PPPV[`\x96T`\0\x90\x81\x90a(\xEA\x90`\x01\x90`\xFF\x16\x81\x90\x1BaX\xFEV[\x90Pa(\xC9`\x01`\x01`\xC0\x1B\x03\x84\x81\x16\x90\x83\x16\x81\x16\x14\x90V[`\xFF\x82\x16`\0\x81\x81R`\x97` \x90\x81R`@\x91\x82\x90 \x84Q\x81T\x86\x84\x01\x80Q\x88\x87\x01\x80Qc\xFF\xFF\xFF\xFF\x90\x95\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x94\x16\x84\x17`\x01` \x1Ba\xFF\xFF\x93\x84\x16\x02\x17g\xFF\xFF\0\0\0\0\0\0\x19\x16`\x01`0\x1B\x95\x83\x16\x95\x90\x95\x02\x94\x90\x94\x17\x90\x94U\x85Q\x91\x82RQ\x83\x16\x93\x81\x01\x93\x90\x93RQ\x16\x91\x81\x01\x91\x90\x91R\x7F>\xE6\xFE\x8DTa\x02D\xC3\xE9\xD3\xC0f\xAEJ\xEE\x99x\x84\xAA(\xF1\x06\x16\xAE\x82\x19%@\x13\x18\xAC\x90``\x01`@Q\x80\x91\x03\x90\xA2PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x99` R`@\x90 \x80T`\x01\x80\x83\x01T`\xFF\x16`\x02\x81\x11\x15a)\xE4Wa)\xE4aL\xD1V[\x14a*QW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R`\0\x80Q` a^s\x839\x81Q\x91R`D\x82\x01R\x7Fperator: operator is not registe`d\x82\x01Rb\x1C\x99Y`\xEA\x1B`\x84\x82\x01R`\xA4\x01a\x08IV[`\x96T`\0\x90a*e\x90\x85\x90`\xFF\x16a(\x17V[\x90P`\0a*r\x83a#PV[\x90P`\x01`\x01`\xC0\x1B\x03\x82\x16a*\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`;`$\x82\x01R`\0\x80Q` a^s\x839\x81Q\x91R`D\x82\x01R\x7Fperator: bitmap cannot be 0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08IV[a*\xE7\x82a(\xD0V[a+RW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`B`$\x82\x01R`\0\x80Q` a^s\x839\x81Q\x91R`D\x82\x01R\x7Fperator: some quorums do not exi`d\x82\x01Ra\x1C\xDD`\xF2\x1B`\x84\x82\x01R`\xA4\x01a\x08IV[a+i`\x01`\x01`\xC0\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x16\x14\x90V[a+\xEFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`Y`$\x82\x01R`\0\x80Q` a^s\x839\x81Q\x91R`D\x82\x01R\x7Fperator: operator is not registe`d\x82\x01R\x7Fred for specified quorums\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x08IV[`\x01`\x01`\xC0\x1B\x03\x82\x81\x16\x19\x82\x16\x16a,\x08\x84\x82aAOV[`\x01`\x01`\xC0\x1B\x03\x81\x16a,\xD7W`\x01\x85\x01\x80T`\xFF\x19\x16`\x02\x17\x90U`@QcQ\xB2zm`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA3d\xF4\xDA\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a,\x88W`\0\x80\xFD[PZ\xF1\x15\x80\x15a,\x9CW=`\0\x80>=`\0\xFD[PP`@Q\x86\x92P`\x01`\x01`\xA0\x1B\x03\x8A\x16\x91P\x7F9o\xDC\xB1\x80\xCB\x0F\xEA&\x92\x81\x13\xFB\x0F\xD1\xC3T\x98c\xF9\xCDV>j\x18O\x1DW\x81\x16\xC8\xE4\x90`\0\x90\xA3[`@Qc\xF4\xE2O\xE5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xF4\xE2O\xE5\x90a-%\x90\x8A\x90\x8A\x90`\x04\x01aY\xBEV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a-?W`\0\x80\xFD[PZ\xF1\x15\x80\x15a-SW=`\0\x80>=`\0\xFD[PP`@Qc\xBD)\xB8\xCD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92Pc\xBD)\xB8\xCD\x91Pa-\xA5\x90\x87\x90\x8A\x90`\x04\x01aY\xE2V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a-\xBFW`\0\x80\xFD[PZ\xF1\x15\x80\x15a-\xD3W=`\0\x80>=`\0\xFD[PP`@Qc\xBD)\xB8\xCD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92Pc\xBD)\xB8\xCD\x91Pa.%\x90\x87\x90\x8A\x90`\x04\x01aY\xE2V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a.?W`\0\x80\xFD[PZ\xF1\x15\x80\x15a.SW=`\0\x80>=`\0\xFD[PPPPPPPPPPPV[`d\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\x13T*N\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a/\x1DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a/A\x91\x90aY\xFBV[\x90P\x80a\x0E\x8FW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xBFy\xCEX\x84\x84a/\x82\x87a\x0FCV[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a/\xA0\x93\x92\x91\x90aZ\x14V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a/\xBFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(\xC9\x91\x90aY\xFBV[` \x80\x82\x01Q`\0\x90\x81R`\x9A\x90\x91R`@\x90 T`\xFF\x16\x15a0\x89W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`R`$\x82\x01R\x7FRegistryCoordinator._verifyChurn`D\x82\x01R\x7FApproverSignature: churnApprover`d\x82\x01Rq\x08\x1C\xD8[\x1D\x08\x18[\x1C\x99XY\x1EH\x1D\\\xD9Y`r\x1B`\x84\x82\x01R`\xA4\x01a\x08IV[B\x81`@\x01Q\x10\x15a1\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`R`$\x82\x01R\x7FRegistryCoordinator._verifyChurn`D\x82\x01R\x7FApproverSignature: churnApprover`d\x82\x01Rq\x08\x1C\xDAY\xDB\x98]\x1D\\\x99H\x19^\x1C\x1A\\\x99Y`r\x1B`\x84\x82\x01R`\xA4\x01a\x08IV[` \x80\x82\x01\x80Q`\0\x90\x81R`\x9A\x90\x92R`@\x91\x82\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U`\x9DT\x90Q\x91\x83\x01Qa\x17\x1E\x92`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91a1f\x91\x87\x91\x87\x91a!\xADV[\x83QaC\x0FV[a1\x91`@Q\x80``\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`\0a1\xD9\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`\x96T`\xFF\x16\x91Pa(\x17\x90PV[\x90P`\0a1\xE6\x88a#PV[\x90P`\x01`\x01`\xC0\x1B\x03\x82\x16a2dW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FRegistryCoordinator._registerOpe`D\x82\x01R\x7Frator: bitmap cannot be 0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08IV[a2m\x82a(\xD0V[a2\xE1W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FRegistryCoordinator._registerOpe`D\x82\x01R\x7Frator: some quorums do not exist`d\x82\x01R`\x84\x01a\x08IV[\x80\x82\x16`\x01`\x01`\xC0\x1B\x03\x16\x15a3\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`h`$\x82\x01R\x7FRegistryCoordinator._registerOpe`D\x82\x01R\x7Frator: operator already register`d\x82\x01R\x7Fed for some quorums being regist`\x84\x82\x01Rg2\xB92\xB2\x1037\xB9`\xC1\x1B`\xA4\x82\x01R`\xC4\x01a\x08IV[`\x01`\x01`\xC0\x1B\x03\x81\x81\x16\x90\x83\x16\x17a3\xB0\x89\x82aAOV[\x88\x7F\xEC)c\xAB!\xC1\xE5\x0E\x1EX*\xA5B\xAF.K\xF7\xBF8\xE6\xE1@<'\xB4.\x1C]nb\x1E\xAA\x87`@Qa3\xE0\x91\x90aV\xE0V[`@Q\x80\x91\x03\x90\xA2`\x01`\x01`\x01`\xA0\x1B\x03\x8B\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T`\xFF\x16`\x02\x81\x11\x15a4\x1AWa4\x1AaL\xD1V[\x14a53W`@\x80Q\x80\x82\x01\x82R\x8A\x81R`\x01` \x80\x83\x01\x82\x81R`\x01`\x01`\xA0\x1B\x03\x8F\x16`\0\x90\x81R`\x99\x90\x92R\x93\x90 \x82Q\x81U\x92Q\x83\x82\x01\x80T\x93\x94\x93\x91\x92\x90\x91`\xFF\x19\x16\x90\x83`\x02\x81\x11\x15a4uWa4uaL\xD1V[\x02\x17\x90UPP`@Qc\x99&\xEE}`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91Pc\x99&\xEE}\x90a4\xCA\x90\x8D\x90\x89\x90`\x04\x01aZ\x93V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a4\xE4W`\0\x80\xFD[PZ\xF1\x15\x80\x15a4\xF8W=`\0\x80>=`\0\xFD[PP`@Q\x8B\x92P`\x01`\x01`\xA0\x1B\x03\x8D\x16\x91P\x7F\xE8\xE6\x8C\xEF\x1C:v\x1E\xD7\xBE~\x84c\xA3u\xF2\x7F{\xC35\xE5\x18$\"<\xAC\xCEcn\xC5\xC3\xFE\x90`\0\x90\xA3[`@Qc\x1F\xD9<\xA9`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c?\xB2yR\x90a5\x83\x90\x8D\x90\x8C\x90\x8C\x90`\x04\x01a[\x07V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a5\x9DW`\0\x80\xFD[PZ\xF1\x15\x80\x15a5\xB1W=`\0\x80>=`\0\xFD[PP`@Qc%PGw`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92Pc%PGw\x91Pa6\x07\x90\x8D\x90\x8D\x90\x8D\x90\x8D\x90`\x04\x01a[,V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a6&W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra6N\x91\x90\x81\x01\x90a[\xC2V[`@\x80\x87\x01\x91\x90\x91R` \x86\x01\x91\x90\x91RQb\xBF\xF0M`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90b\xBF\xF0M\x90a6\xAB\x90\x8C\x90\x8C\x90\x8C\x90`\x04\x01a\\%V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a6\xCAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra6\xF2\x91\x90\x81\x01\x90a\\?V[\x84RPPP\x96\x95PPPPPPV[` \x80\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`\0\x81\x81R`\x99\x90\x94R`@\x90\x93 T\x91\x92\x90\x87\x16\x14\x15a7\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R`\0\x80Q` a^3\x839\x81Q\x91R`D\x82\x01Rt97\x1D\x101\xB0\xB777\xBA\x101\xB4:\xB97\x109\xB2\xB63`Y\x1B`d\x82\x01R`\x84\x01a\x08IV[\x87`\xFF\x16\x84`\0\x01Q`\xFF\x16\x14a7\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R`\0\x80Q` a^3\x839\x81Q\x91R`D\x82\x01R\x7Frn: quorumNumber not the same as`d\x82\x01Rf\x08\x1C\xDAY\xDB\x99Y`\xCA\x1B`\x84\x82\x01R`\xA4\x01a\x08IV[`@QcT\x01\xED'`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`\xFF\x89\x16`$\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cT\x01\xED'\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a8oW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8\x93\x91\x90a\\\xD8V[\x90Pa8\x9F\x81\x85aD\xC9V[`\x01`\x01``\x1B\x03\x16\x86`\x01`\x01``\x1B\x03\x16\x11a92W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`V`$\x82\x01R`\0\x80Q` a^3\x839\x81Q\x91R`D\x82\x01R\x7Frn: incoming operator has insuff`d\x82\x01Ru4\xB1\xB4\xB2\xB7:\x109\xBA0\xB5\xB2\x9037\xB9\x101\xB4:\xB97`Q\x1B`\x84\x82\x01R`\xA4\x01a\x08IV[a9<\x88\x85aD\xEDV[`\x01`\x01``\x1B\x03\x16\x81`\x01`\x01``\x1B\x03\x16\x10a!,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R`\0\x80Q` a^3\x839\x81Q\x91R`D\x82\x01R\x7Frn: cannot kick operator with mo`d\x82\x01R\x7Fre than kickBIPsOfTotalStake\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x08IV[`\0\x81\x81R`\x98` R`@\x81 T\x81[\x81\x81\x10\x15a:hW`\x01a9\xFB\x82\x84aX\xFEV[a:\x05\x91\x90aX\xFEV[\x92P\x84c\xFF\xFF\xFF\xFF\x16`\x98`\0\x86\x81R` \x01\x90\x81R` \x01`\0 \x84c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a:8Wa:8aVAV[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a:VWPPa\x0E\x8FV[\x80a:`\x81aVmV[\x91PPa9\xE7V[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`l`$\x82\x01R\x7FRegistryCoordinator.getQuorumBit`D\x82\x01R\x7FmapIndexAtBlockNumber: no bitmap`d\x82\x01R\x7F update found for operatorId at `\x84\x82\x01Rk167\xB1\xB5\x907:\xB6\xB12\xB9`\xA1\x1B`\xA4\x82\x01R`\xC4\x01a\x08IV[`\x96T`\xFF\x16`\xC0\x81\x10a;\x86W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FRegistryCoordinator.createQuorum`D\x82\x01Rt\x0E\x88\x1BX^\x08\x1C][\xDC\x9D[\\\xC8\x1C\x99XX\xDA\x19Y`Z\x1B`d\x82\x01R`\x84\x01a\x08IV[a;\x91\x81`\x01a\\\xF5V[`\x96\x80T`\xFF\x19\x16`\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a;\xB0\x81\x86a)\x03V[`@Q`\x01b\x96\xB5\x89`\xE0\x1B\x03\x19\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xFFiJw\x90a<\x03\x90\x84\x90\x88\x90\x88\x90`\x04\x01a]\x1AV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a<\x1DW`\0\x80\xFD[PZ\xF1\x15\x80\x15a<1W=`\0\x80>=`\0\xFD[PP`@Qc\x13l\xA0\xF9`\xE1\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x92Pc&\xD9A\xF2\x91P`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a<\x99W`\0\x80\xFD[PZ\xF1\x15\x80\x15a<\xADW=`\0\x80>=`\0\xFD[PP`@Qc\x13l\xA0\xF9`\xE1\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x92Pc&\xD9A\xF2\x91P`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a=\x15W`\0\x80\xFD[PZ\xF1\x15\x80\x15a!,W=`\0\x80>=`\0\xFD[`\0Tb\x01\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a=PWP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a=\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x08IV[`\x01\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a>\x15\x82a%\x03V[PPV[`\x000`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80\x15a>rWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14[\x15a>\x9CWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[P`@\x80Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x80\x83\x01\x91\x90\x91R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x84\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x83\x01RF`\x80\x83\x01R0`\xA0\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\xC0\x90\x92\x01\x90\x92R\x80Q\x91\x01 \x90V[`\0\x80\x80`\0\x80Q` a^S\x839\x81Q\x91R`\x03`\0\x80Q` a^S\x839\x81Q\x91R\x86`\0\x80Q` a^S\x839\x81Q\x91R\x88\x89\t\t\x08\x90P`\0a?\xB6\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\0\x80Q` a^S\x839\x81Q\x91RaE\x07V[\x91\x95\x91\x94P\x90\x92PPPV[`\0a\x01\0\x82Q\x11\x15a@KW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FBitmapUtils.orderedBytesArrayToB\x90\x82\x01R\x7Fitmap: orderedBytesArray is too `d\x82\x01Rclong`\xE0\x1B`\x84\x82\x01R`\xA4\x01a\x08IV[\x81Qa@YWP`\0\x91\x90PV[`\0\x80\x83`\0\x81Q\x81\x10a@oWa@oaVAV[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x81\x90\x1B\x92P[\x84Q\x81\x10\x15aAFW\x84\x81\x81Q\x81\x10a@\x9DWa@\x9DaVAV[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x1B\x91P\x82\x82\x11aA2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: orderedBytesArray is not `d\x82\x01Rf\x1B\xDC\x99\x19\\\x99Y`\xCA\x1B`\x84\x82\x01R`\xA4\x01a\x08IV[\x91\x81\x17\x91aA?\x81aVmV[\x90Pa@\x82V[P\x90\x93\x92PPPV[`\0\x82\x81R`\x98` R`@\x90 T\x80aA\xF4W`\0\x83\x81R`\x98` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x86\x81R`\x01`\x01`\xC0\x1B\x03\x80\x8A\x16\x95\x84\x01\x95\x86R\x84T`\x01\x81\x01\x86U\x94\x88R\x95\x90\x96 \x91Q\x91\x90\x92\x01\x80T\x95Q\x93Q\x90\x94\x16`\x01`@\x1B\x02`\x01`\x01`@\x1B\x03\x93\x83\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x96\x16\x91\x90\x92\x16\x17\x93\x90\x93\x17\x16\x91\x90\x91\x17\x90UPPPV[`\0\x83\x81R`\x98` R`@\x81 aB\r`\x01\x84aX\xFEV[\x81T\x81\x10aB\x1DWaB\x1DaVAV[`\0\x91\x82R` \x90\x91 \x01\x80T\x90\x91PCc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14\x15aBaW\x80T`\x01`\x01`@\x1B\x03\x16`\x01`@\x1B`\x01`\x01`\xC0\x1B\x03\x85\x16\x02\x17\x81Ua\t0V[\x80Tc\xFF\xFF\xFF\xFFC\x81\x16`\x01` \x1B\x81\x81\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x94\x16\x93\x90\x93\x17\x84U`\0\x87\x81R`\x98` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83R\x94\x85R\x84\x83\x01\x84\x81R`\x01`\x01`\xC0\x1B\x03\x80\x8C\x16\x93\x87\x01\x93\x84R\x82T`\x01\x81\x01\x84U\x92\x86R\x93\x90\x94 \x94Q\x94\x01\x80T\x93Q\x91Q\x90\x92\x16`\x01`@\x1B\x02`\x01`\x01`@\x1B\x03\x91\x86\x16\x90\x96\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x93\x90\x94\x16\x92\x90\x92\x17\x17\x91\x90\x91\x16\x91\x90\x91\x17\x90UPPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16;\x15aD)W`@Qc\x0B\x13]?`\xE1\x1B\x80\x82R\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\x16&\xBA~\x90aCO\x90\x86\x90\x86\x90`\x04\x01aY\xE2V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aClW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aC\x90\x91\x90a]\x93V[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14a\x17\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`S`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: ERC1271 signature `d\x82\x01Rr\x1D\x99\\\x9AY\x9AX\xD8]\x1A[\xDB\x88\x19\x98Z[\x19Y`j\x1B`\x84\x82\x01R`\xA4\x01a\x08IV[\x82`\x01`\x01`\xA0\x1B\x03\x16aD=\x83\x83aE\xB6V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x17\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: signature not from`d\x82\x01Rf\x109\xB4\xB3\xB72\xB9`\xC9\x1B`\x84\x82\x01R`\xA4\x01a\x08IV[` \x81\x01Q`\0\x90a'\x10\x90aD\xE3\x90a\xFF\xFF\x16\x85a]\xBDV[a(\xC9\x91\x90a]\xECV[`@\x81\x01Q`\0\x90a'\x10\x90aD\xE3\x90a\xFF\xFF\x16\x85a]\xBDV[`\0\x80aE\x12aI#V[aE\x1AaIAV[` \x80\x82R\x81\x81\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x88\x90R`\x80\x82\x01\x87\x90R`\xA0\x82\x01\x86\x90R\x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82\x80\x15aE[WaE]V[\xFE[P\x82aE\xABW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FBN254.expMod: call failure\0\0\0\0\0\0`D\x82\x01R`d\x01a\x08IV[PQ\x95\x94PPPPPV[`\0\x80`\0aE\xC5\x85\x85aE\xD2V[\x91P\x91Pa\x1D\xB1\x81aFBV[`\0\x80\x82Q`A\x14\x15aF\tW` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1AaE\xFD\x87\x82\x85\x85aG\xFDV[\x94P\x94PPPPaF;V[\x82Q`@\x14\x15aF3W` \x83\x01Q`@\x84\x01QaF(\x86\x83\x83aH\xEAV[\x93P\x93PPPaF;V[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0\x81`\x04\x81\x11\x15aFVWaFVaL\xD1V[\x14\x15aF_WPV[`\x01\x81`\x04\x81\x11\x15aFsWaFsaL\xD1V[\x14\x15aF\xC1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x08IV[`\x02\x81`\x04\x81\x11\x15aF\xD5WaF\xD5aL\xD1V[\x14\x15aG#W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x08IV[`\x03\x81`\x04\x81\x11\x15aG7WaG7aL\xD1V[\x14\x15aG\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x08IV[`\x04\x81`\x04\x81\x11\x15aG\xA4WaG\xA4aL\xD1V[\x14\x15a\x0C\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 'v' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x08IV[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15aH4WP`\0\x90P`\x03aH\xE1V[\x84`\xFF\x16`\x1B\x14\x15\x80\x15aHLWP\x84`\xFF\x16`\x1C\x14\x15[\x15aH]WP`\0\x90P`\x04aH\xE1V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15aH\xB1W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16aH\xDAW`\0`\x01\x92P\x92PPaH\xE1V[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x80`\x01`\x01`\xFF\x1B\x03\x83\x16\x81aI\x07`\xFF\x86\x90\x1C`\x1BaX*V[\x90PaI\x15\x87\x82\x88\x85aG\xFDV[\x93P\x93PPP\x93P\x93\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`\0\x80\x83`\x1F\x84\x01\x12aIqW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aI\x88W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15aF;W`\0\x80\xFD[`\0\x80` \x83\x85\x03\x12\x15aI\xB6W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aI\xCCW`\0\x80\xFD[aI\xD8\x85\x82\x86\x01aI_V[\x90\x96\x90\x95P\x93PPPPV[`\0` \x82\x84\x03\x12\x15aI\xF6W`\0\x80\xFD[P5\x91\x90PV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0C\xC4W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15aJ$W`\0\x80\xFD[\x835\x92P` \x84\x015aJ6\x81aI\xFDV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aJ\x7FWaJ\x7FaJGV[`@R\x90V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aJ\x7FWaJ\x7FaJGV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aJ\xCFWaJ\xCFaJGV[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x83\x11\x15aJ\xF0WaJ\xF0aJGV[aK\x03`\x1F\x84\x01`\x1F\x19\x16` \x01aJ\xA7V[\x90P\x82\x81R\x83\x83\x83\x01\x11\x15aK\x17W`\0\x80\xFD[\x82\x82` \x83\x017`\0` \x84\x83\x01\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15aK@W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aKVW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13aKgW`\0\x80\xFD[aKv\x84\x825` \x84\x01aJ\xD7V[\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0C\xC4W`\0\x80\xFD[\x805aK\x9E\x81aK~V[\x91\x90PV[`\0` \x82\x84\x03\x12\x15aK\xB5W`\0\x80\xFD[\x815a(\xC9\x81aK~V[`\0\x80`@\x83\x85\x03\x12\x15aK\xD3W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[\x805`\xFF\x81\x16\x81\x14aK\x9EW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aL\x05W`\0\x80\xFD[a(\xC9\x82aK\xE2V[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x81\x01a\x0E\x8FV[`\0\x80\x83`\x1F\x84\x01\x12aL7W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aLNW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aF;W`\0\x80\xFD[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15aL|W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aL\x93W`\0\x80\xFD[aL\x9F\x88\x83\x89\x01aI_V[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15aL\xB8W`\0\x80\xFD[PaL\xC5\x87\x82\x88\x01aL%V[\x95\x98\x94\x97P\x95PPPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x03\x81\x10aM\x05WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[\x81Q\x81R` \x80\x83\x01Q`@\x83\x01\x91aM$\x90\x84\x01\x82aL\xE7V[P\x92\x91PPV[\x805a\xFF\xFF\x81\x16\x81\x14aK\x9EW`\0\x80\xFD[`\0``\x82\x84\x03\x12\x15aMOW`\0\x80\xFD[aMWaJ]V[\x90P\x815aMd\x81aI\xFDV[\x81RaMr` \x83\x01aM+V[` \x82\x01RaM\x83`@\x83\x01aM+V[`@\x82\x01R\x92\x91PPV[`\0\x80`\x80\x83\x85\x03\x12\x15aM\xA1W`\0\x80\xFD[aM\xAA\x83aK\xE2V[\x91PaM\xB9\x84` \x85\x01aM=V[\x90P\x92P\x92\x90PV[`\0\x80`\0`@\x84\x86\x03\x12\x15aM\xD7W`\0\x80\xFD[\x835aM\xE2\x81aK~V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aM\xFDW`\0\x80\xFD[aN\t\x86\x82\x87\x01aL%V[\x94\x97\x90\x96P\x93\x94PPPPV[`\0a\x01\0\x82\x84\x03\x12\x15aN)W`\0\x80\xFD[P\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12aNAW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aNXW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x06\x1B\x85\x01\x01\x11\x15aF;W`\0\x80\xFD[`\0``\x82\x84\x03\x12\x15aN\x85W`\0\x80\xFD[aN\x8DaJ]V[\x90P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aN\xA5W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13aN\xB6W`\0\x80\xFD[aN\xC5\x84\x825` \x84\x01aJ\xD7V[\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\0\x80`\0a\x01\xA0\x8A\x8C\x03\x12\x15aO\x01W`\0\x80\xFD[\x895`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aO\x18W`\0\x80\xFD[aO$\x8D\x83\x8E\x01aL%V[\x90\x9BP\x99P` \x8C\x015\x91P\x80\x82\x11\x15aO=W`\0\x80\xFD[aOI\x8D\x83\x8E\x01aL%V[\x90\x99P\x97P\x87\x91PaO^\x8D`@\x8E\x01aN\x16V[\x96Pa\x01@\x8C\x015\x91P\x80\x82\x11\x15aOuW`\0\x80\xFD[aO\x81\x8D\x83\x8E\x01aN/V[\x90\x96P\x94Pa\x01`\x8C\x015\x91P\x80\x82\x11\x15aO\x9BW`\0\x80\xFD[aO\xA7\x8D\x83\x8E\x01aNsV[\x93Pa\x01\x80\x8C\x015\x91P\x80\x82\x11\x15aO\xBEW`\0\x80\xFD[PaO\xCB\x8C\x82\x8D\x01aNsV[\x91PP\x92\x95\x98P\x92\x95\x98P\x92\x95\x98V[`\0\x80`\0\x80`\0\x80a\x01`\x87\x89\x03\x12\x15aO\xF5W`\0\x80\xFD[\x865`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aP\x0CW`\0\x80\xFD[aP\x18\x8A\x83\x8B\x01aL%V[\x90\x98P\x96P` \x89\x015\x91P\x80\x82\x11\x15aP1W`\0\x80\xFD[aP=\x8A\x83\x8B\x01aL%V[\x90\x96P\x94P\x84\x91PaPR\x8A`@\x8B\x01aN\x16V[\x93Pa\x01@\x89\x015\x91P\x80\x82\x11\x15aPiW`\0\x80\xFD[PaPv\x89\x82\x8A\x01aNsV[\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15aP\x9CWaP\x9CaJGV[P`\x05\x1B` \x01\x90V[`\0\x80`@\x83\x85\x03\x12\x15aP\xB9W`\0\x80\xFD[\x825aP\xC4\x81aI\xFDV[\x91P` \x83\x81\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aP\xE0W`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13aP\xF1W`\0\x80\xFD[\x805aQ\x04aP\xFF\x82aP\x83V[aJ\xA7V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x88\x83\x11\x15aQ#W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aQAW\x835\x82R\x92\x84\x01\x92\x90\x84\x01\x90aQ(V[\x80\x95PPPPPP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aQ\x8EW\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aQlV[P\x90\x96\x95PPPPPPV[`\0\x80` \x83\x85\x03\x12\x15aQ\xADW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aQ\xC3W`\0\x80\xFD[aI\xD8\x85\x82\x86\x01aL%V[`\x01`\x01``\x1B\x03\x81\x16\x81\x14a\x0C\xC4W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12aQ\xF5W`\0\x80\xFD[\x815` aR\x05aP\xFF\x83aP\x83V[\x82\x81R`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aR$W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aRuW`@\x81\x89\x03\x12\x15aRAW`\0\x80\x81\xFD[aRIaJ\x85V[\x815aRT\x81aK~V[\x81R\x81\x85\x015aRc\x81aQ\xCFV[\x81\x86\x01R\x83R\x91\x83\x01\x91`@\x01aR(V[P\x96\x95PPPPPPV[`\0\x80`\0`\xA0\x84\x86\x03\x12\x15aR\x95W`\0\x80\xFD[aR\x9F\x85\x85aM=V[\x92P``\x84\x015aR\xAF\x81aQ\xCFV[\x91P`\x80\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aR\xCAW`\0\x80\xFD[aR\xD6\x86\x82\x87\x01aQ\xE4V[\x91PP\x92P\x92P\x92V[`\0\x82`\x1F\x83\x01\x12aR\xF1W`\0\x80\xFD[\x815` aS\x01aP\xFF\x83aP\x83V[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15aS W`\0\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15aSCWaS6\x89\x82aM=V[\x84R\x92\x84\x01\x92\x81\x01aS$V[P\x90\x97\x96PPPPPPPV[`\0\x82`\x1F\x83\x01\x12aSaW`\0\x80\xFD[\x815` aSqaP\xFF\x83aP\x83V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aS\x90W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aRuW\x805aS\xA7\x81aQ\xCFV[\x83R\x91\x83\x01\x91\x83\x01aS\x94V[`\0\x82`\x1F\x83\x01\x12aS\xC5W`\0\x80\xFD[\x815` aS\xD5aP\xFF\x83aP\x83V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aS\xF4W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aRuW\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aT\x17W`\0\x80\x81\xFD[aT%\x89\x86\x83\x8B\x01\x01aQ\xE4V[\x84RP\x91\x83\x01\x91\x83\x01aS\xF8V[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15aTPW`\0\x80\xFD[aTY\x89aK\x93V[\x97PaTg` \x8A\x01aK\x93V[\x96PaTu`@\x8A\x01aK\x93V[\x95PaT\x83``\x8A\x01aK\x93V[\x94P`\x80\x89\x015\x93P`\xA0\x89\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aT\xA6W`\0\x80\xFD[aT\xB2\x8C\x83\x8D\x01aR\xE0V[\x94P`\xC0\x8B\x015\x91P\x80\x82\x11\x15aT\xC8W`\0\x80\xFD[aT\xD4\x8C\x83\x8D\x01aSPV[\x93P`\xE0\x8B\x015\x91P\x80\x82\x11\x15aT\xEAW`\0\x80\xFD[PaT\xF7\x8B\x82\x8C\x01aS\xB4V[\x91PP\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[`\0`@\x82\x84\x03\x12\x15aU\x19W`\0\x80\xFD[aU!aJ\x85V[\x90PaU,\x82aK\xE2V[\x81R` \x82\x015aU<\x81aK~V[` \x82\x01R\x92\x91PPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aU]W`\0\x80\xFD[\x845\x93P` \x80\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aU{W`\0\x80\xFD[\x86\x01`\x1F\x81\x01\x88\x13aU\x8CW`\0\x80\xFD[\x805aU\x9AaP\xFF\x82aP\x83V[\x81\x81R`\x06\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x8A\x83\x11\x15aU\xB9W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aU\xE2WaU\xD0\x8B\x85aU\x07V[\x82R\x84\x82\x01\x91P`@\x84\x01\x93PaU\xBEV[\x97\x9A\x97\x99PPPP`@\x86\x015\x95``\x015\x94\x93PPPPV[` \x81\x01a\x0E\x8F\x82\x84aL\xE7V[` \x80\x82R`\x19\x90\x82\x01R\x7FPausable: index is paused\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15aV\x81WaV\x81aVWV[P`\x01\x01\x90V[`\0[\x83\x81\x10\x15aV\xA3W\x81\x81\x01Q\x83\x82\x01R` \x01aV\x8BV[\x83\x81\x11\x15a\t0WPP`\0\x91\x01RV[`\0\x81Q\x80\x84RaV\xCC\x81` \x86\x01` \x86\x01aV\x88V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a(\xC9` \x83\x01\x84aV\xB4V[`\0` \x82\x84\x03\x12\x15aW\x05W`\0\x80\xFD[\x81Qa(\xC9\x81aK~V[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15aWlW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a(\xC9W`\0\x80\xFD[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aW\xDBW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aW\xF5W`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15aF;W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aX\x1FW`\0\x80\xFD[\x81Qa(\xC9\x81aI\xFDV[`\0\x82\x19\x82\x11\x15aX=WaX=aVWV[P\x01\x90V[`\0\x80\x85\x85\x11\x15aXRW`\0\x80\xFD[\x83\x86\x11\x15aX_W`\0\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[`\0`@\x82\x84\x03\x12\x15aX~W`\0\x80\xFD[a(\xC9\x83\x83aU\x07V[`\0`\xA0\x82\x01\x87\x83R` \x87\x81\x85\x01R`@`\xA0\x81\x86\x01R\x82\x88Q\x80\x85R`\xC0\x87\x01\x91P\x83\x8A\x01\x94P`\0[\x81\x81\x10\x15aX\xE5W\x85Q\x80Q`\xFF\x16\x84R\x85\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x85\x84\x01R\x94\x84\x01\x94\x91\x83\x01\x91`\x01\x01aX\xB4V[PP``\x86\x01\x97\x90\x97RPPPP`\x80\x01R\x93\x92PPPV[`\0\x82\x82\x10\x15aY\x10WaY\x10aVWV[P\x03\x90V[`\0\x83QaY'\x81\x84` \x88\x01aV\x88V[`\x01`\x01`\xF8\x1B\x03\x19\x93\x90\x93\x16\x91\x90\x92\x01\x90\x81R`\x01\x01\x92\x91PPV[`\x01\x80`\xA0\x1B\x03\x84\x16\x81R\x82` \x82\x01R```@\x82\x01R`\0a!\xEB``\x83\x01\x84aV\xB4V[`\0` \x82\x84\x03\x12\x15aY}W`\0\x80\xFD[\x81Q`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x14a(\xC9W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82aY\xB9WaY\xB9aY\x94V[P\x06\x90V[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R`\0\x90aKv\x90\x83\x01\x84aV\xB4V[\x82\x81R`@` \x82\x01R`\0aKv`@\x83\x01\x84aV\xB4V[`\0` \x82\x84\x03\x12\x15aZ\rW`\0\x80\xFD[PQ\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81Ra\x01`\x81\x01aZ<` \x83\x01\x85\x805\x82R` \x90\x81\x015\x91\x01RV[aZV``\x83\x01`@\x86\x01\x805\x82R` \x90\x81\x015\x91\x01RV[`@`\x80\x85\x01`\xA0\x84\x017`\xE0\x82\x01`\0\x81R`@`\xC0\x86\x01\x827P`\0a\x01 \x83\x01\x90\x81R\x83Q\x90R` \x90\x92\x01Qa\x01@\x90\x91\x01R\x92\x91PPV[`\x01\x80`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R`\0\x82Q```@\x84\x01RaZ\xBD`\xA0\x84\x01\x82aV\xB4V[\x90P` \x84\x01Q``\x84\x01R`@\x84\x01Q`\x80\x84\x01R\x80\x91PP\x93\x92PPPV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R`@` \x82\x01\x81\x90R`\0\x90a!\xEB\x90\x83\x01\x84\x86aZ\xDEV[`\x01\x80`\xA0\x1B\x03\x85\x16\x81R\x83` \x82\x01R```@\x82\x01R`\0a[T``\x83\x01\x84\x86aZ\xDEV[\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a[oW`\0\x80\xFD[\x81Q` a[\x7FaP\xFF\x83aP\x83V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a[\x9EW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aRuW\x80Qa[\xB5\x81aQ\xCFV[\x83R\x91\x83\x01\x91\x83\x01a[\xA2V[`\0\x80`@\x83\x85\x03\x12\x15a[\xD5W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a[\xECW`\0\x80\xFD[a[\xF8\x86\x83\x87\x01a[^V[\x93P` \x85\x01Q\x91P\x80\x82\x11\x15a\\\x0EW`\0\x80\xFD[Pa\\\x1B\x85\x82\x86\x01a[^V[\x91PP\x92P\x92\x90PV[\x83\x81R`@` \x82\x01R`\0a!\xEB`@\x83\x01\x84\x86aZ\xDEV[`\0` \x80\x83\x85\x03\x12\x15a\\RW`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\\hW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\\yW`\0\x80\xFD[\x80Qa\\\x87aP\xFF\x82aP\x83V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a\\\xA6W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\\\xCDW\x83Qa\\\xBE\x81aI\xFDV[\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\\\xABV[\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\\\xEAW`\0\x80\xFD[\x81Qa(\xC9\x81aQ\xCFV[`\0`\xFF\x82\x16`\xFF\x84\x16\x80`\xFF\x03\x82\x11\x15a]\x12Wa]\x12aVWV[\x01\x93\x92PPPV[`\0``\x82\x01`\xFF\x86\x16\x83R` `\x01`\x01``\x1B\x03\x80\x87\x16\x82\x86\x01R`@``\x81\x87\x01R\x83\x87Q\x80\x86R`\x80\x88\x01\x91P\x84\x89\x01\x95P`\0[\x81\x81\x10\x15a]\x83W\x86Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x86\x01Q\x85\x16\x86\x84\x01R\x95\x85\x01\x95\x91\x83\x01\x91`\x01\x01a]SV[P\x90\x9A\x99PPPPPPPPPPV[`\0` \x82\x84\x03\x12\x15a]\xA5W`\0\x80\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a(\xC9W`\0\x80\xFD[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15a]\xE3Wa]\xE3aVWV[\x02\x94\x93PPPPV[`\0`\x01`\x01``\x1B\x03\x80\x84\x16\x80a^\x06Wa^\x06aY\x94V[\x92\x16\x91\x90\x91\x04\x92\x91PPV\xFERegistryCoordinator.updateOperatRegistryCoordinator._validateChu0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDGRegistryCoordinator._deregisterO\xA2dipfsX\"\x12 \x19\xA636\xDA\xB1G\xC6\xCCj\xDF\xAE6\x04\xB8.\x06\xBA\xB0M{q\xE3\x8FV\xA2\xE35\xA3;\xD0\xB3dsolcC\0\x08\x0C\x003";
    /// The bytecode of the contract.
    pub static REGISTRYCOORDINATOR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\x94W`\x005`\xE0\x1C\x80c]\xF4YF\x11a\x01gW\x80c\xA5\x08W\xBF\x11a\0\xCEW\x80c\xDD\x82\x83\xF3\x11a\0\x87W\x80c\xDD\x82\x83\xF3\x14a\x06\xF5W\x80c\xE6W\x97\xAD\x14a\x07\x08W\x80c\xF2\xFD\xE3\x8B\x14a\x07\xABW\x80c\xF8X\x11\x91\x14a\x07\xBEW\x80c\xFA\xBC\x1C\xBC\x14a\x07\xD1W\x80c\xFD9\x10Z\x14a\x07\xE4W`\0\x80\xFD[\x80c\xA5\x08W\xBF\x14a\x06mW\x80c\xC3\x91B^\x14a\x06\x80W\x80c\xCA\r\xE8\x82\x14a\x06\xA0W\x80c\xCAO-\x97\x14a\x06\xC7W\x80c\xD7-\x8D\xD6\x14a\x06\xDAW\x80c\xD7[L\x88\x14a\x06\xE2W`\0\x80\xFD[\x80c\x88o\x11\x95\x11a\x01 W\x80c\x88o\x11\x95\x14a\x05\xCCW\x80c\x8D\xA5\xCB[\x14a\x05\xE5W\x80c\x9A\xA1e=\x14a\x05\xEDW\x80c\x9B]\x17{\x14a\x06\x0CW\x80c\x9E\x99#\xC2\x14a\x06\x1FW\x80c\x9F\xEA\xB8Y\x14a\x06FW`\0\x80\xFD[\x80c]\xF4YF\x14a\x05=W\x80ccG\xC9\0\x14a\x05dW\x80ch0H5\x14a\x05wW\x80cn;\x17\xDB\x14a\x05\x9EW\x80cqP\x18\xA6\x14a\x05\xB1W\x80c\x87\x1E\xF0I\x14a\x05\xB9W`\0\x80\xFD[\x80c(\xF6\x1B1\x11a\x02\x0BW\x80cQ@\xA5H\x11a\x01\xC4W\x80cQ@\xA5H\x14a\x04\xC8W\x80cXe\xC6\x0C\x14a\x04\xDBW\x80cY\\jg\x14a\x04\xFBW\x80cZ\xC8j\xB7\x14a\x05\x03W\x80c[\x0B\x82\x9F\x14a\x05\"W\x80c\\\x97Z\xBB\x14a\x055W`\0\x80\xFD[\x80c(\xF6\x1B1\x14a\x045W\x80c)k\xB0d\x14a\x04HW\x80c)\xD1\xE0\xC3\x14a\x04[W\x80c,\xDD\x1E\x86\x14a\x04nW\x80c9\x98\xFD\xD3\x14a\x04\x81W\x80c<*\x7FL\x14a\x04\xA8W`\0\x80\xFD[\x80c\x10\xD6z/\x11a\x02]W\x80c\x10\xD6z/\x14a\x03JW\x80c\x13T*N\x14a\x03]W\x80c\x13d9\xDD\x14a\x03\x86W\x80c\x14x\x85\x1F\x14a\x03\x99W\x80c\x1E\xB8\x12\xDA\x14a\x03\xCCW\x80c$\x9A\x0CB\x14a\x04\x15W`\0\x80\xFD[\x80b\xCF*\xB5\x14a\x02\x99W\x80c\x03\xFD4\x92\x14a\x02\xAEW\x80c\x04\xECcQ\x14a\x02\xE1W\x80c\x05C\x10\xE6\x14a\x03\x0CW\x80c\x0C\xF4\xB7g\x14a\x037W[`\0\x80\xFD[a\x02\xACa\x02\xA76`\x04aI\xA3V[a\x08 V[\0[a\x02\xCEa\x02\xBC6`\x04aI\xE4V[`\0\x90\x81R`\x98` R`@\x90 T\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\xF4a\x02\xEF6`\x04aJ\x0FV[a\t6V[`@Q`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xD8V[`\x9DTa\x03\x1F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xD8V[a\x02\xACa\x03E6`\x04aK.V[a\x0B,V[a\x02\xACa\x03X6`\x04aK\xA3V[a\x0C\x14V[a\x02\xCEa\x03k6`\x04aK\xA3V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x99` R`@\x90 T\x90V[a\x02\xACa\x03\x946`\x04aI\xE4V[a\x0C\xC7V[a\x03\xBCa\x03\xA76`\x04aI\xE4V[`\x9A` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02\xD8V[a\x03\xDFa\x03\xDA6`\x04aK\xC0V[a\x0E\x04V[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x80\x85\x01Q\x90\x91\x16\x90\x82\x01R\x91\x81\x01Q`\x01`\x01`\xC0\x1B\x03\x16\x90\x82\x01R``\x01a\x02\xD8V[a\x02\xCEa\x04#6`\x04aK\xF3V[`\x9B` R`\0\x90\x81R`@\x90 T\x81V[`\x9ETa\x03\x1F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03\x1Fa\x04V6`\x04aI\xE4V[a\x0E\x95V[a\x02\xACa\x04i6`\x04aK\xA3V[a\x0F!V[a\x02\xACa\x04|6`\x04aK\xA3V[a\x0F2V[a\x03\x1F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x04\xBBa\x04\xB66`\x04aK\xA3V[a\x0FCV[`@Qa\x02\xD8\x91\x90aL\x0EV[a\x02\xACa\x04\xD66`\x04aLfV[a\x0F\xC2V[a\x04\xEEa\x04\xE96`\x04aK\xA3V[a\x15LV[`@Qa\x02\xD8\x91\x90aM\tV[a\x02\xACa\x15\xC0V[a\x03\xBCa\x05\x116`\x04aK\xF3V[`\x01\x80T`\xFF\x90\x92\x16\x1B\x90\x81\x16\x14\x90V[a\x02\xACa\x0506`\x04aM\x8EV[a\x16\x8CV[`\x01Ta\x02\xCEV[a\x03\x1F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\x1Fa\x05r6`\x04aI\xE4V[a\x17#V[a\x03\x1F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xACa\x05\xAC6`\x04aM\xC2V[a\x17MV[a\x02\xACa\x18\rV[a\x02\xF4a\x05\xC76`\x04aI\xE4V[a\x18!V[`\0Ta\x03\x1F\x90b\x01\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03\x1Fa\x18,V[`\x96Ta\x05\xFA\x90`\xFF\x16\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02\xD8V[a\x02\xACa\x06\x1A6`\x04aN\xE2V[a\x18EV[a\x03\x1F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xCE\x7F+\xD8!$\x05\x7F\t\x13\xBC;w,\xE7\xB8>\x80W\xC1\xAD\x1F5\x10\xFC\x83w\x8B\xE2\x0F\x10\xEC]\xE6\x81V[a\x02\xACa\x06{6`\x04aO\xDBV[a\x1B|V[a\x06\x93a\x06\x8E6`\x04aP\xA6V[a\x1D\0V[`@Qa\x02\xD8\x91\x90aQPV[a\x02\xCE\x7F\xF8C\xB3\x11mWOC\xE6\x9F\x8D\xDA]\x93\xEB\xF1\x1D\xCC\xC4\xA4e\x98?\x94S\x05\x80\x05\xCDk4\xA0\x81V[a\x02\xACa\x06\xD56`\x04aQ\x9AV[a\x1D\xB9V[`\x9CTa\x02\xCEV[a\x02\xACa\x06\xF06`\x04aR\x80V[a\x1E V[a\x02\xACa\x07\x036`\x04aT3V[a\x1E3V[a\x07wa\x07\x166`\x04aK\xF3V[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x92\x84\x01\x81\x90R`\xFF\x94\x90\x94\x16\x84R`\x97\x82R\x92\x82\x90 \x82Q\x93\x84\x01\x83RTc\xFF\xFF\xFF\xFF\x81\x16\x84Ra\xFF\xFF`\x01` \x1B\x82\x04\x81\x16\x92\x85\x01\x92\x90\x92R`\x01`0\x1B\x90\x04\x16\x90\x82\x01R\x90V[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x16\x81R` \x80\x84\x01Qa\xFF\xFF\x90\x81\x16\x91\x83\x01\x91\x90\x91R\x92\x82\x01Q\x90\x92\x16\x90\x82\x01R``\x01a\x02\xD8V[a\x02\xACa\x07\xB96`\x04aK\xA3V[a!7V[a\x02\xCEa\x07\xCC6`\x04aUGV[a!\xADV[a\x02\xACa\x07\xDF6`\x04aI\xE4V[a!\xF4V[a\x08\x13a\x07\xF26`\x04aK\xA3V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T`\xFF\x16\x90V[`@Qa\x02\xD8\x91\x90aU\xFCV[`\x01T`\x02\x90`\x04\x90\x81\x16\x14\x15a\x08RW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08I\x90aV\nV[`@Q\x80\x91\x03\x90\xFD[`\0[\x82\x81\x10\x15a\t0W`\0\x84\x84\x83\x81\x81\x10a\x08qWa\x08qaVAV[\x90P` \x02\x01` \x81\x01\x90a\x08\x86\x91\x90aK\xA3V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x99` \x90\x81R`@\x80\x83 \x81Q\x80\x83\x01\x90\x92R\x80T\x82R`\x01\x81\x01T\x94\x95P\x92\x93\x90\x92\x91\x83\x01\x90`\xFF\x16`\x02\x81\x11\x15a\x08\xD1Wa\x08\xD1aL\xD1V[`\x02\x81\x11\x15a\x08\xE2Wa\x08\xE2aL\xD1V[\x90RP\x80Q\x90\x91P`\0a\x08\xF5\x82a#PV[\x90P`\0a\t\x0B\x82`\x01`\x01`\xC0\x1B\x03\x16a#\xB9V[\x90Pa\t\x18\x85\x85\x83a$\x16V[PPPPP\x80\x80a\t(\x90aVmV[\x91PPa\x08UV[PPPPV[`\0\x83\x81R`\x98` R`@\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\tYWa\tYaVAV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x85R`\x01` \x1B\x83\x04\x82\x16\x95\x85\x01\x95\x90\x95R`\x01`@\x1B\x90\x91\x04`\x01`\x01`\xC0\x1B\x03\x16\x91\x83\x01\x91\x90\x91R\x90\x92P\x85\x16\x10\x15a\nSW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`e`$\x82\x01R\x7FRegistryCoordinator.getQuorumBit`D\x82\x01R\x7FmapAtBlockNumberByIndex: quorumB`d\x82\x01R\x7FitmapUpdate is from after blockN`\x84\x82\x01Rd:\xB6\xB12\xB9`\xD9\x1B`\xA4\x82\x01R`\xC4\x01a\x08IV[` \x81\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x80a\nyWP\x80` \x01Qc\xFF\xFF\xFF\xFF\x16\x84c\xFF\xFF\xFF\xFF\x16\x10[a\x0B W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`f`$\x82\x01R\x7FRegistryCoordinator.getQuorumBit`D\x82\x01R\x7FmapAtBlockNumberByIndex: quorumB`d\x82\x01R\x7FitmapUpdate is from before block`\x84\x82\x01Re':\xB6\xB12\xB9`\xD1\x1B`\xA4\x82\x01R`\xC4\x01a\x08IV[`@\x01Q\x94\x93PPPPV[`\x013`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T`\xFF\x16`\x02\x81\x11\x15a\x0BUWa\x0BUaL\xD1V[\x14a\x0B\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R\x7FRegistryCoordinator.updateSocket`D\x82\x01R\x7F: operator is not registered\0\0\0\0`d\x82\x01R`\x84\x01a\x08IV[3`\0\x90\x81R`\x99` R`@\x90\x81\x90 T\x90Q\x7F\xEC)c\xAB!\xC1\xE5\x0E\x1EX*\xA5B\xAF.K\xF7\xBF8\xE6\xE1@<'\xB4.\x1C]nb\x1E\xAA\x90a\x0C\t\x90\x84\x90aV\xE0V[`@Q\x80\x91\x03\x90\xA2PV[`\0`\x02\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CgW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x8B\x91\x90aV\xF3V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0C\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08I\x90aW\x10V[a\x0C\xC4\x81a%\x03V[PV[`\0T`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01Rb\x01\0\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\x14W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r8\x91\x90aWZV[a\rTW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08I\x90aW|V[`\x01T\x81\x81\x16\x14a\r\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08IV[`\x01\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01a\x0C\tV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R`\0\x83\x81R`\x98` R`@\x90 \x80T\x83\x90\x81\x10a\x0EAWa\x0EAaVAV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01`\xC0\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90P[\x92\x91PPV[`@Qc\x08\xF6b\x9D`\xE3\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cG\xB3\x14\xE8\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xFDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\x8F\x91\x90aV\xF3V[a\x0F)a&\x08V[a\x0C\xC4\x81a&gV[a\x0F:a&\x08V[a\x0C\xC4\x81a&\xD0V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x0E\x8Fa\x0F\xBD\x7F+\xD8!$\x05\x7F\t\x13\xBC;w,\xE7\xB8>\x80W\xC1\xAD\x1F5\x10\xFC\x83w\x8B\xE2\x0F\x10\xEC]\xE6\x84`@Q` \x01a\x0F\xA2\x92\x91\x90\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a'9V[a'\x87V[`\x01T`\x02\x90`\x04\x90\x81\x16\x14\x15a\x0F\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08I\x90aV\nV[`\0a\x103\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`\x96T`\xFF\x16\x91Pa(\x17\x90PV[\x90Pa\x10>\x81a(\xD0V[a\x10\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R`\0\x80Q` a^\x13\x839\x81Q\x91R`D\x82\x01R\x7ForsForQuorum: some quorums do no`d\x82\x01Rf\x1D\x08\x19^\x1A\\\xDD`\xCA\x1B`\x84\x82\x01R`\xA4\x01a\x08IV[\x84\x83\x14a\x11\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R`\0\x80Q` a^\x13\x839\x81Q\x91R`D\x82\x01R\x7ForsForQuorum: input length misma`d\x82\x01Rb\x0E\x8Cm`\xEB\x1B`\x84\x82\x01R`\xA4\x01a\x08IV[`\0[\x83\x81\x10\x15a\x15CW`\0\x85\x85\x83\x81\x81\x10a\x11<Wa\x11<aVAV[\x91\x90\x91\x015`\xF8\x1C\x91P6\x90P`\0\x89\x89\x85\x81\x81\x10a\x11]Wa\x11]aVAV[\x90P` \x02\x81\x01\x90a\x11o\x91\x90aW\xC4V[`@Qcy\xA0\x84\x91`\xE1\x1B\x81R`\xFF\x86\x16`\x04\x82\x01R\x91\x93P\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xF3A\t\"\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xDBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xFF\x91\x90aX\rV[c\xFF\xFF\xFF\xFF\x16\x81\x14a\x12\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`e`$\x82\x01R`\0\x80Q` a^\x13\x839\x81Q\x91R`D\x82\x01R\x7ForsForQuorum: number of updated `d\x82\x01R\x7Foperators does not match quorum `\x84\x82\x01Rd\x1D\x1B\xDD\x18[`\xDA\x1B`\xA4\x82\x01R`\xC4\x01a\x08IV[`\0\x80[\x82\x81\x10\x15a\x14\xE2W`\0\x84\x84\x83\x81\x81\x10a\x12\xBBWa\x12\xBBaVAV[\x90P` \x02\x01` \x81\x01\x90a\x12\xD0\x91\x90aK\xA3V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x99` \x90\x81R`@\x80\x83 \x81Q\x80\x83\x01\x90\x92R\x80T\x82R`\x01\x81\x01T\x94\x95P\x92\x93\x90\x92\x91\x83\x01\x90`\xFF\x16`\x02\x81\x11\x15a\x13\x1BWa\x13\x1BaL\xD1V[`\x02\x81\x11\x15a\x13,Wa\x13,aL\xD1V[\x90RP\x80Q\x90\x91P`\0a\x13?\x82a#PV[\x90P`\x01`\x01`\x01`\xC0\x1B\x03\x82\x16`\xFF\x8B\x16\x1C\x81\x16\x14a\x13\xC3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R`\0\x80Q` a^\x13\x839\x81Q\x91R\x90\x82\x01R\x7ForsForQuorum: operator not in qu`d\x82\x01Rcorum`\xE0\x1B`\x84\x82\x01R`\xA4\x01a\x08IV[\x85`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x11a\x14nW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`g`$\x82\x01R`\0\x80Q` a^\x13\x839\x81Q\x91R`D\x82\x01R\x7ForsForQuorum: operators array mu`d\x82\x01R\x7Fst be sorted in ascending addres`\x84\x82\x01Rf9\x907\xB922\xB9`\xC9\x1B`\xA4\x82\x01R`\xC4\x01a\x08IV[Pa\x14\xCC\x83\x83\x8F\x8F\x8D\x90\x8E`\x01a\x14\x85\x91\x90aX*V[\x92a\x14\x92\x93\x92\x91\x90aXBV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa$\x16\x92PPPV[P\x90\x92Pa\x14\xDB\x90P\x81aVmV[\x90Pa\x12\x9FV[P`\xFF\x84\x16`\0\x81\x81R`\x9B` \x90\x81R`@\x91\x82\x90 C\x90\x81\x90U\x91Q\x91\x82R\x7FF\x07}U3\x07c\xF1bi\xFDu\xE5v\x16c\xF4\x19-'\x91t|\x01\x89\xB1j\xD3\x1D\xB0}\xB4\x91\x01`@Q\x80\x91\x03\x90\xA2PPPP\x80a\x15<\x90aVmV[\x90Pa\x11 V[PPPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x99` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x81\x01T\x90\x91\x83\x01\x90`\xFF\x16`\x02\x81\x11\x15a\x15\xA6Wa\x15\xA6aL\xD1V[`\x02\x81\x11\x15a\x15\xB7Wa\x15\xB7aL\xD1V[\x90RP\x92\x91PPV[`\0T`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01Rb\x01\0\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x161\x91\x90aWZV[a\x16MW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08I\x90aW|V[`\0\x19`\x01\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[a\x16\x94a&\x08V[`\x96T\x82\x90`\xFF\x90\x81\x16\x90\x82\x16\x10a\x17\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FRegistryCoordinator.quorumExists`D\x82\x01R\x7F: quorum does not exist\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08IV[a\x17\x1E\x83\x83a)\x03V[PPPV[`\x9C\x81\x81T\x81\x10a\x173W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x81V[`\x9ET`\x01`\x01`\xA0\x1B\x03\x163\x14a\x17\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FRegistryCoordinator.onlyEjector:`D\x82\x01R\x7F caller is not the ejector\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08IV[a\x17\x1E\x83\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa)\xB0\x92PPPV[a\x18\x15a&\x08V[a\x18\x1F`\0a.`V[V[`\0a\x0E\x8F\x82a#PV[`\0a\x18@`dT`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x90P\x90V[`\x01\x80T`\0\x91\x90\x81\x16\x14\x15a\x18mW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08I\x90aV\nV[\x83\x89\x14a\x18\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FRegistryCoordinator.registerOper\x90\x82\x01R\x7FatorWithChurn: input length mism`d\x82\x01Rc\x0C.\x8Cm`\xE3\x1B`\x84\x82\x01R`\xA4\x01a\x08IV[`\0a\x18\xFC3\x88a.\xB2V[\x90Pa\x19[\x81\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x19PWa\x19A`@\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aXlV[\x81R` \x01\x90`\x01\x01\x90a\x19$V[PPPPP\x86a/\xE3V[`\0a\x19\xA23\x83\x8E\x8E\x8E\x8E\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8C\x92Pa1m\x91PPV[\x90P`\0[\x8B\x81\x10\x15a\x1BmW`\0`\x97`\0\x8F\x8F\x85\x81\x81\x10a\x19\xC7Wa\x19\xC7aVAV[\x91\x90\x91\x015`\xF8\x1C\x82RP` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0 \x81Q``\x81\x01\x83R\x90Tc\xFF\xFF\xFF\xFF\x81\x16\x80\x83Ra\xFF\xFF`\x01` \x1B\x83\x04\x81\x16\x95\x84\x01\x95\x90\x95R`\x01`0\x1B\x90\x91\x04\x90\x93\x16\x91\x81\x01\x91\x90\x91R\x84Q\x80Q\x91\x93P\x90\x84\x90\x81\x10a\x1A4Wa\x1A4aVAV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\x1BZWa\x1A\xD5\x8E\x8E\x84\x81\x81\x10a\x1A]Wa\x1A]aVAV[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x84`@\x01Q\x84\x81Q\x81\x10a\x1A\x80Wa\x1A\x80aVAV[` \x02` \x01\x01Q3\x86` \x01Q\x86\x81Q\x81\x10a\x1A\x9FWa\x1A\x9FaVAV[` \x02` \x01\x01Q\x8D\x8D\x88\x81\x81\x10a\x1A\xB9Wa\x1A\xB9aVAV[\x90P`@\x02\x01\x806\x03\x81\x01\x90a\x1A\xCF\x91\x90aXlV[\x86a7\x01V[a\x1BZ\x89\x89\x84\x81\x81\x10a\x1A\xEAWa\x1A\xEAaVAV[\x90P`@\x02\x01` \x01` \x81\x01\x90a\x1B\x02\x91\x90aK\xA3V[\x8F\x8F\x85\x90\x86`\x01a\x1B\x13\x91\x90aX*V[\x92a\x1B \x93\x92\x91\x90aXBV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa)\xB0\x92PPPV[P\x80a\x1Be\x81aVmV[\x91PPa\x19\xA7V[PPPPPPPPPPPPPV[`\x01\x80T`\0\x91\x90\x81\x16\x14\x15a\x1B\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08I\x90aV\nV[`\0a\x1B\xB03\x85a.\xB2V[\x90P`\0a\x1B\xF93\x83\x8B\x8B\x8B\x8B\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8C\x92Pa1m\x91PPV[Q\x90P`\0[\x88\x81\x10\x15a\x1C\xF4W`\0\x8A\x8A\x83\x81\x81\x10a\x1C\x1BWa\x1C\x1BaVAV[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x97` R`@\x90 T\x85Q\x91\x93Pc\xFF\xFF\xFF\xFF\x16\x91P\x84\x90\x84\x90\x81\x10a\x1CQWa\x1CQaVAV[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a\x1C\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FRegistryCoordinator.registerOper\x90\x82\x01R\x7Fator: operator count exceeds max`d\x82\x01Rcimum`\xE0\x1B`\x84\x82\x01R`\xA4\x01a\x08IV[P\x80a\x1C\xEC\x81aVmV[\x91PPa\x1B\xFFV[PPPPPPPPPPV[```\0\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\x1DWa\x1D\x1DaJGV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1DFW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\x1D\xB1Wa\x1Dx\x85\x85\x83\x81Q\x81\x10a\x1DkWa\x1DkaVAV[` \x02` \x01\x01Qa9\xD6V[\x82\x82\x81Q\x81\x10a\x1D\x8AWa\x1D\x8AaVAV[c\xFF\xFF\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80a\x1D\xA9\x81aVmV[\x91PPa\x1DLV[P\x93\x92PPPV[`\x01\x80T`\x02\x90\x81\x16\x14\x15a\x1D\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08I\x90aV\nV[a\x17\x1E3\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa)\xB0\x92PPPV[a\x1E(a&\x08V[a\x17\x1E\x83\x83\x83a;\x12V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x1ESWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x1EmWP0;\x15\x80\x15a\x1EmWP`\0T`\xFF\x16`\x01\x14[a\x1E\xD0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x08IV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x1E\xF3W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[\x82Q\x84Q\x14\x80\x15a\x1F\x05WP\x81Q\x83Q\x14[a\x1FoW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FRegistryCoordinator.initialize: `D\x82\x01Rt\r-\xCE\x0E\xAE\x84\r\x8C\xAD\xCC\xEE\x8D\x04\r\xAD.m\xAC.\x8Cm`[\x1B`d\x82\x01R`\x84\x01a\x08IV[a\x1Fx\x89a.`V[a\x1F\x82\x86\x86a=)V[a\x1F\x8B\x88a&gV[a\x1F\x94\x87a&\xD0V[`\x9C\x80T`\x01\x81\x81\x01\x83U`\0\x83\x81R\x7F\xAF\x85\xB9\x07\x1D\xFA\xFE\xAC\x14\t\xD3\xF1\xD1\x9B\xAF\xC9\xBC|7\x97L\xDE\x8D\xF0\xEEah\xF0\x08nS\x9C\x92\x83\x01\x80T`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U\x85T\x80\x85\x01\x87U\x85\x01\x80T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x16\x90\x83\x16\x17\x90U\x85T\x93\x84\x01\x90\x95U\x91\x90\x92\x01\x80T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x16\x91\x90\x93\x16\x17\x90\x91U[\x84Q\x81\x10\x15a \xE5Wa \xD3\x85\x82\x81Q\x81\x10a \x92Wa \x92aVAV[` \x02` \x01\x01Q\x85\x83\x81Q\x81\x10a \xACWa \xACaVAV[` \x02` \x01\x01Q\x85\x84\x81Q\x81\x10a \xC6Wa \xC6aVAV[` \x02` \x01\x01Qa;\x12V[\x80a \xDD\x81aVmV[\x91PPa tV[P\x80\x15a!,W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPPPV[a!?a&\x08V[`\x01`\x01`\xA0\x1B\x03\x81\x16a!\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x08IV[a\x0C\xC4\x81a.`V[`\0a!\xEB\x7F\xF8C\xB3\x11mWOC\xE6\x9F\x8D\xDA]\x93\xEB\xF1\x1D\xCC\xC4\xA4e\x98?\x94S\x05\x80\x05\xCDk4\xA0\x86\x86\x86\x86`@Q` \x01a\x0F\xA2\x95\x94\x93\x92\x91\x90aX\x88V[\x95\x94PPPPPV[`\0`\x02\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"GW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"k\x91\x90aV\xF3V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\"\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08I\x90aW\x10V[`\x01T\x19\x81\x19`\x01T\x19\x16\x14a#\x19W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08IV[`\x01\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x0C\tV[`\0\x81\x81R`\x98` R`@\x81 T\x80a#mWP`\0\x92\x91PPV[`\0\x83\x81R`\x98` R`@\x90 a#\x86`\x01\x83aX\xFEV[\x81T\x81\x10a#\x96Wa#\x96aVAV[`\0\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01`\xC0\x1B\x03\x16\x93\x92PPPV[```\0\x80[a\x01\0\x81\x10\x15a$\x0FW`\x01\x81\x1B\x91P\x83\x82\x16\x15a#\xFFW\x82\x81`\xF8\x1B`@Q` \x01a#\xED\x92\x91\x90aY\x15V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92P[a$\x08\x81aVmV[\x90Pa#\xBFV[PP\x91\x90PV[`\x01\x82` \x01Q`\x02\x81\x11\x15a$.Wa$.aL\xD1V[\x14a$8WPPPV[\x81Q`@Qc3V\x7F\x7F`\xE1\x1B\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cf\xAC\xFE\xFE\x90a$\x8D\x90\x88\x90\x86\x90\x88\x90`\x04\x01aYDV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a$\xACW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$\xD0\x91\x90aYkV[\x90P`\x01`\x01`\xC0\x1B\x03\x81\x16\x15a$\xFCWa$\xFC\x85a$\xF7\x83`\x01`\x01`\xC0\x1B\x03\x16a#\xB9V[a)\xB0V[PPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16a%\x91W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x08IV[`\0T`@\x80Q`\x01`\x01`\xA0\x1B\x03b\x01\0\0\x90\x93\x04\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`\0\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16b\x01\0\0\x02b\x01\0\0`\x01`\xB0\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[3a&\x11a\x18,V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x18\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x08IV[`\x9DT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F1TW\xD8\xA8\xFE`\xF0J\xF1|\x16\xE2\xF5\xA5\xE1\xDBa+1d\x8EX\x03\x03`u\x9E\xF8\xF3R\x8C\x91\x01`@Q\x80\x91\x03\x90\xA1`\x9D\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x9ET`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F\x8F0\xAB\t\xF4:l\x15}\x7F\xCE~\n\x13\xC0\x03\x04,\x1C\x95\xE8\xA7.z\x14j!\xC0\xCA\xA2M\xC9\x91\x01`@Q\x80\x91\x03\x90\xA1`\x9E\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0a\x0E\x8Fa'Fa>\x19V[\x83`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x83\x90R`B\x81\x01\x82\x90R`\0\x90`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0\x80\x80a'\xB7`\0\x80Q` a^S\x839\x81Q\x91R\x86aY\xAAV[\x90P[a'\xC3\x81a?@V[\x90\x93P\x91P`\0\x80Q` a^S\x839\x81Q\x91R\x82\x83\t\x83\x14\x15a'\xFDW`@\x80Q\x80\x82\x01\x90\x91R\x90\x81R` \x81\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80Q` a^S\x839\x81Q\x91R`\x01\x82\x08\x90Pa'\xBAV[`\0\x80a(#\x84a?\xC2V[\x90P\x80\x15a(\xC9W\x82`\xFF\x16\x84`\x01\x86Qa(>\x91\x90aX\xFEV[\x81Q\x81\x10a(NWa(NaVAV[\x01` \x01Q`\xF8\x1C\x10a(\xC9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`?`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: bitmap exceeds max value\0`d\x82\x01R`\x84\x01a\x08IV[\x93\x92PPPV[`\x96T`\0\x90\x81\x90a(\xEA\x90`\x01\x90`\xFF\x16\x81\x90\x1BaX\xFEV[\x90Pa(\xC9`\x01`\x01`\xC0\x1B\x03\x84\x81\x16\x90\x83\x16\x81\x16\x14\x90V[`\xFF\x82\x16`\0\x81\x81R`\x97` \x90\x81R`@\x91\x82\x90 \x84Q\x81T\x86\x84\x01\x80Q\x88\x87\x01\x80Qc\xFF\xFF\xFF\xFF\x90\x95\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x94\x16\x84\x17`\x01` \x1Ba\xFF\xFF\x93\x84\x16\x02\x17g\xFF\xFF\0\0\0\0\0\0\x19\x16`\x01`0\x1B\x95\x83\x16\x95\x90\x95\x02\x94\x90\x94\x17\x90\x94U\x85Q\x91\x82RQ\x83\x16\x93\x81\x01\x93\x90\x93RQ\x16\x91\x81\x01\x91\x90\x91R\x7F>\xE6\xFE\x8DTa\x02D\xC3\xE9\xD3\xC0f\xAEJ\xEE\x99x\x84\xAA(\xF1\x06\x16\xAE\x82\x19%@\x13\x18\xAC\x90``\x01`@Q\x80\x91\x03\x90\xA2PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x99` R`@\x90 \x80T`\x01\x80\x83\x01T`\xFF\x16`\x02\x81\x11\x15a)\xE4Wa)\xE4aL\xD1V[\x14a*QW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R`\0\x80Q` a^s\x839\x81Q\x91R`D\x82\x01R\x7Fperator: operator is not registe`d\x82\x01Rb\x1C\x99Y`\xEA\x1B`\x84\x82\x01R`\xA4\x01a\x08IV[`\x96T`\0\x90a*e\x90\x85\x90`\xFF\x16a(\x17V[\x90P`\0a*r\x83a#PV[\x90P`\x01`\x01`\xC0\x1B\x03\x82\x16a*\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`;`$\x82\x01R`\0\x80Q` a^s\x839\x81Q\x91R`D\x82\x01R\x7Fperator: bitmap cannot be 0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08IV[a*\xE7\x82a(\xD0V[a+RW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`B`$\x82\x01R`\0\x80Q` a^s\x839\x81Q\x91R`D\x82\x01R\x7Fperator: some quorums do not exi`d\x82\x01Ra\x1C\xDD`\xF2\x1B`\x84\x82\x01R`\xA4\x01a\x08IV[a+i`\x01`\x01`\xC0\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x16\x14\x90V[a+\xEFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`Y`$\x82\x01R`\0\x80Q` a^s\x839\x81Q\x91R`D\x82\x01R\x7Fperator: operator is not registe`d\x82\x01R\x7Fred for specified quorums\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x08IV[`\x01`\x01`\xC0\x1B\x03\x82\x81\x16\x19\x82\x16\x16a,\x08\x84\x82aAOV[`\x01`\x01`\xC0\x1B\x03\x81\x16a,\xD7W`\x01\x85\x01\x80T`\xFF\x19\x16`\x02\x17\x90U`@QcQ\xB2zm`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA3d\xF4\xDA\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a,\x88W`\0\x80\xFD[PZ\xF1\x15\x80\x15a,\x9CW=`\0\x80>=`\0\xFD[PP`@Q\x86\x92P`\x01`\x01`\xA0\x1B\x03\x8A\x16\x91P\x7F9o\xDC\xB1\x80\xCB\x0F\xEA&\x92\x81\x13\xFB\x0F\xD1\xC3T\x98c\xF9\xCDV>j\x18O\x1DW\x81\x16\xC8\xE4\x90`\0\x90\xA3[`@Qc\xF4\xE2O\xE5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xF4\xE2O\xE5\x90a-%\x90\x8A\x90\x8A\x90`\x04\x01aY\xBEV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a-?W`\0\x80\xFD[PZ\xF1\x15\x80\x15a-SW=`\0\x80>=`\0\xFD[PP`@Qc\xBD)\xB8\xCD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92Pc\xBD)\xB8\xCD\x91Pa-\xA5\x90\x87\x90\x8A\x90`\x04\x01aY\xE2V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a-\xBFW`\0\x80\xFD[PZ\xF1\x15\x80\x15a-\xD3W=`\0\x80>=`\0\xFD[PP`@Qc\xBD)\xB8\xCD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92Pc\xBD)\xB8\xCD\x91Pa.%\x90\x87\x90\x8A\x90`\x04\x01aY\xE2V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a.?W`\0\x80\xFD[PZ\xF1\x15\x80\x15a.SW=`\0\x80>=`\0\xFD[PPPPPPPPPPPV[`d\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\x13T*N\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a/\x1DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a/A\x91\x90aY\xFBV[\x90P\x80a\x0E\x8FW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xBFy\xCEX\x84\x84a/\x82\x87a\x0FCV[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a/\xA0\x93\x92\x91\x90aZ\x14V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a/\xBFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(\xC9\x91\x90aY\xFBV[` \x80\x82\x01Q`\0\x90\x81R`\x9A\x90\x91R`@\x90 T`\xFF\x16\x15a0\x89W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`R`$\x82\x01R\x7FRegistryCoordinator._verifyChurn`D\x82\x01R\x7FApproverSignature: churnApprover`d\x82\x01Rq\x08\x1C\xD8[\x1D\x08\x18[\x1C\x99XY\x1EH\x1D\\\xD9Y`r\x1B`\x84\x82\x01R`\xA4\x01a\x08IV[B\x81`@\x01Q\x10\x15a1\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`R`$\x82\x01R\x7FRegistryCoordinator._verifyChurn`D\x82\x01R\x7FApproverSignature: churnApprover`d\x82\x01Rq\x08\x1C\xDAY\xDB\x98]\x1D\\\x99H\x19^\x1C\x1A\\\x99Y`r\x1B`\x84\x82\x01R`\xA4\x01a\x08IV[` \x80\x82\x01\x80Q`\0\x90\x81R`\x9A\x90\x92R`@\x91\x82\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U`\x9DT\x90Q\x91\x83\x01Qa\x17\x1E\x92`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91a1f\x91\x87\x91\x87\x91a!\xADV[\x83QaC\x0FV[a1\x91`@Q\x80``\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81RP\x90V[`\0a1\xD9\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`\x96T`\xFF\x16\x91Pa(\x17\x90PV[\x90P`\0a1\xE6\x88a#PV[\x90P`\x01`\x01`\xC0\x1B\x03\x82\x16a2dW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FRegistryCoordinator._registerOpe`D\x82\x01R\x7Frator: bitmap cannot be 0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08IV[a2m\x82a(\xD0V[a2\xE1W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FRegistryCoordinator._registerOpe`D\x82\x01R\x7Frator: some quorums do not exist`d\x82\x01R`\x84\x01a\x08IV[\x80\x82\x16`\x01`\x01`\xC0\x1B\x03\x16\x15a3\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`h`$\x82\x01R\x7FRegistryCoordinator._registerOpe`D\x82\x01R\x7Frator: operator already register`d\x82\x01R\x7Fed for some quorums being regist`\x84\x82\x01Rg2\xB92\xB2\x1037\xB9`\xC1\x1B`\xA4\x82\x01R`\xC4\x01a\x08IV[`\x01`\x01`\xC0\x1B\x03\x81\x81\x16\x90\x83\x16\x17a3\xB0\x89\x82aAOV[\x88\x7F\xEC)c\xAB!\xC1\xE5\x0E\x1EX*\xA5B\xAF.K\xF7\xBF8\xE6\xE1@<'\xB4.\x1C]nb\x1E\xAA\x87`@Qa3\xE0\x91\x90aV\xE0V[`@Q\x80\x91\x03\x90\xA2`\x01`\x01`\x01`\xA0\x1B\x03\x8B\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T`\xFF\x16`\x02\x81\x11\x15a4\x1AWa4\x1AaL\xD1V[\x14a53W`@\x80Q\x80\x82\x01\x82R\x8A\x81R`\x01` \x80\x83\x01\x82\x81R`\x01`\x01`\xA0\x1B\x03\x8F\x16`\0\x90\x81R`\x99\x90\x92R\x93\x90 \x82Q\x81U\x92Q\x83\x82\x01\x80T\x93\x94\x93\x91\x92\x90\x91`\xFF\x19\x16\x90\x83`\x02\x81\x11\x15a4uWa4uaL\xD1V[\x02\x17\x90UPP`@Qc\x99&\xEE}`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91Pc\x99&\xEE}\x90a4\xCA\x90\x8D\x90\x89\x90`\x04\x01aZ\x93V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a4\xE4W`\0\x80\xFD[PZ\xF1\x15\x80\x15a4\xF8W=`\0\x80>=`\0\xFD[PP`@Q\x8B\x92P`\x01`\x01`\xA0\x1B\x03\x8D\x16\x91P\x7F\xE8\xE6\x8C\xEF\x1C:v\x1E\xD7\xBE~\x84c\xA3u\xF2\x7F{\xC35\xE5\x18$\"<\xAC\xCEcn\xC5\xC3\xFE\x90`\0\x90\xA3[`@Qc\x1F\xD9<\xA9`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c?\xB2yR\x90a5\x83\x90\x8D\x90\x8C\x90\x8C\x90`\x04\x01a[\x07V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a5\x9DW`\0\x80\xFD[PZ\xF1\x15\x80\x15a5\xB1W=`\0\x80>=`\0\xFD[PP`@Qc%PGw`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92Pc%PGw\x91Pa6\x07\x90\x8D\x90\x8D\x90\x8D\x90\x8D\x90`\x04\x01a[,V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a6&W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra6N\x91\x90\x81\x01\x90a[\xC2V[`@\x80\x87\x01\x91\x90\x91R` \x86\x01\x91\x90\x91RQb\xBF\xF0M`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90b\xBF\xF0M\x90a6\xAB\x90\x8C\x90\x8C\x90\x8C\x90`\x04\x01a\\%V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a6\xCAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra6\xF2\x91\x90\x81\x01\x90a\\?V[\x84RPPP\x96\x95PPPPPPV[` \x80\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`\0\x81\x81R`\x99\x90\x94R`@\x90\x93 T\x91\x92\x90\x87\x16\x14\x15a7\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R`\0\x80Q` a^3\x839\x81Q\x91R`D\x82\x01Rt97\x1D\x101\xB0\xB777\xBA\x101\xB4:\xB97\x109\xB2\xB63`Y\x1B`d\x82\x01R`\x84\x01a\x08IV[\x87`\xFF\x16\x84`\0\x01Q`\xFF\x16\x14a7\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R`\0\x80Q` a^3\x839\x81Q\x91R`D\x82\x01R\x7Frn: quorumNumber not the same as`d\x82\x01Rf\x08\x1C\xDAY\xDB\x99Y`\xCA\x1B`\x84\x82\x01R`\xA4\x01a\x08IV[`@QcT\x01\xED'`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`\xFF\x89\x16`$\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cT\x01\xED'\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a8oW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8\x93\x91\x90a\\\xD8V[\x90Pa8\x9F\x81\x85aD\xC9V[`\x01`\x01``\x1B\x03\x16\x86`\x01`\x01``\x1B\x03\x16\x11a92W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`V`$\x82\x01R`\0\x80Q` a^3\x839\x81Q\x91R`D\x82\x01R\x7Frn: incoming operator has insuff`d\x82\x01Ru4\xB1\xB4\xB2\xB7:\x109\xBA0\xB5\xB2\x9037\xB9\x101\xB4:\xB97`Q\x1B`\x84\x82\x01R`\xA4\x01a\x08IV[a9<\x88\x85aD\xEDV[`\x01`\x01``\x1B\x03\x16\x81`\x01`\x01``\x1B\x03\x16\x10a!,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R`\0\x80Q` a^3\x839\x81Q\x91R`D\x82\x01R\x7Frn: cannot kick operator with mo`d\x82\x01R\x7Fre than kickBIPsOfTotalStake\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x08IV[`\0\x81\x81R`\x98` R`@\x81 T\x81[\x81\x81\x10\x15a:hW`\x01a9\xFB\x82\x84aX\xFEV[a:\x05\x91\x90aX\xFEV[\x92P\x84c\xFF\xFF\xFF\xFF\x16`\x98`\0\x86\x81R` \x01\x90\x81R` \x01`\0 \x84c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a:8Wa:8aVAV[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a:VWPPa\x0E\x8FV[\x80a:`\x81aVmV[\x91PPa9\xE7V[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`l`$\x82\x01R\x7FRegistryCoordinator.getQuorumBit`D\x82\x01R\x7FmapIndexAtBlockNumber: no bitmap`d\x82\x01R\x7F update found for operatorId at `\x84\x82\x01Rk167\xB1\xB5\x907:\xB6\xB12\xB9`\xA1\x1B`\xA4\x82\x01R`\xC4\x01a\x08IV[`\x96T`\xFF\x16`\xC0\x81\x10a;\x86W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FRegistryCoordinator.createQuorum`D\x82\x01Rt\x0E\x88\x1BX^\x08\x1C][\xDC\x9D[\\\xC8\x1C\x99XX\xDA\x19Y`Z\x1B`d\x82\x01R`\x84\x01a\x08IV[a;\x91\x81`\x01a\\\xF5V[`\x96\x80T`\xFF\x19\x16`\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a;\xB0\x81\x86a)\x03V[`@Q`\x01b\x96\xB5\x89`\xE0\x1B\x03\x19\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xFFiJw\x90a<\x03\x90\x84\x90\x88\x90\x88\x90`\x04\x01a]\x1AV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a<\x1DW`\0\x80\xFD[PZ\xF1\x15\x80\x15a<1W=`\0\x80>=`\0\xFD[PP`@Qc\x13l\xA0\xF9`\xE1\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x92Pc&\xD9A\xF2\x91P`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a<\x99W`\0\x80\xFD[PZ\xF1\x15\x80\x15a<\xADW=`\0\x80>=`\0\xFD[PP`@Qc\x13l\xA0\xF9`\xE1\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x92Pc&\xD9A\xF2\x91P`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a=\x15W`\0\x80\xFD[PZ\xF1\x15\x80\x15a!,W=`\0\x80>=`\0\xFD[`\0Tb\x01\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a=PWP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a=\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x08IV[`\x01\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a>\x15\x82a%\x03V[PPV[`\x000`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80\x15a>rWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14[\x15a>\x9CWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[P`@\x80Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x80\x83\x01\x91\x90\x91R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x84\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x83\x01RF`\x80\x83\x01R0`\xA0\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\xC0\x90\x92\x01\x90\x92R\x80Q\x91\x01 \x90V[`\0\x80\x80`\0\x80Q` a^S\x839\x81Q\x91R`\x03`\0\x80Q` a^S\x839\x81Q\x91R\x86`\0\x80Q` a^S\x839\x81Q\x91R\x88\x89\t\t\x08\x90P`\0a?\xB6\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\0\x80Q` a^S\x839\x81Q\x91RaE\x07V[\x91\x95\x91\x94P\x90\x92PPPV[`\0a\x01\0\x82Q\x11\x15a@KW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FBitmapUtils.orderedBytesArrayToB\x90\x82\x01R\x7Fitmap: orderedBytesArray is too `d\x82\x01Rclong`\xE0\x1B`\x84\x82\x01R`\xA4\x01a\x08IV[\x81Qa@YWP`\0\x91\x90PV[`\0\x80\x83`\0\x81Q\x81\x10a@oWa@oaVAV[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x81\x90\x1B\x92P[\x84Q\x81\x10\x15aAFW\x84\x81\x81Q\x81\x10a@\x9DWa@\x9DaVAV[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x1B\x91P\x82\x82\x11aA2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: orderedBytesArray is not `d\x82\x01Rf\x1B\xDC\x99\x19\\\x99Y`\xCA\x1B`\x84\x82\x01R`\xA4\x01a\x08IV[\x91\x81\x17\x91aA?\x81aVmV[\x90Pa@\x82V[P\x90\x93\x92PPPV[`\0\x82\x81R`\x98` R`@\x90 T\x80aA\xF4W`\0\x83\x81R`\x98` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x86\x81R`\x01`\x01`\xC0\x1B\x03\x80\x8A\x16\x95\x84\x01\x95\x86R\x84T`\x01\x81\x01\x86U\x94\x88R\x95\x90\x96 \x91Q\x91\x90\x92\x01\x80T\x95Q\x93Q\x90\x94\x16`\x01`@\x1B\x02`\x01`\x01`@\x1B\x03\x93\x83\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x96\x16\x91\x90\x92\x16\x17\x93\x90\x93\x17\x16\x91\x90\x91\x17\x90UPPPV[`\0\x83\x81R`\x98` R`@\x81 aB\r`\x01\x84aX\xFEV[\x81T\x81\x10aB\x1DWaB\x1DaVAV[`\0\x91\x82R` \x90\x91 \x01\x80T\x90\x91PCc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14\x15aBaW\x80T`\x01`\x01`@\x1B\x03\x16`\x01`@\x1B`\x01`\x01`\xC0\x1B\x03\x85\x16\x02\x17\x81Ua\t0V[\x80Tc\xFF\xFF\xFF\xFFC\x81\x16`\x01` \x1B\x81\x81\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x94\x16\x93\x90\x93\x17\x84U`\0\x87\x81R`\x98` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83R\x94\x85R\x84\x83\x01\x84\x81R`\x01`\x01`\xC0\x1B\x03\x80\x8C\x16\x93\x87\x01\x93\x84R\x82T`\x01\x81\x01\x84U\x92\x86R\x93\x90\x94 \x94Q\x94\x01\x80T\x93Q\x91Q\x90\x92\x16`\x01`@\x1B\x02`\x01`\x01`@\x1B\x03\x91\x86\x16\x90\x96\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x93\x90\x94\x16\x92\x90\x92\x17\x17\x91\x90\x91\x16\x91\x90\x91\x17\x90UPPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16;\x15aD)W`@Qc\x0B\x13]?`\xE1\x1B\x80\x82R\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\x16&\xBA~\x90aCO\x90\x86\x90\x86\x90`\x04\x01aY\xE2V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aClW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aC\x90\x91\x90a]\x93V[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14a\x17\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`S`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: ERC1271 signature `d\x82\x01Rr\x1D\x99\\\x9AY\x9AX\xD8]\x1A[\xDB\x88\x19\x98Z[\x19Y`j\x1B`\x84\x82\x01R`\xA4\x01a\x08IV[\x82`\x01`\x01`\xA0\x1B\x03\x16aD=\x83\x83aE\xB6V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x17\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: signature not from`d\x82\x01Rf\x109\xB4\xB3\xB72\xB9`\xC9\x1B`\x84\x82\x01R`\xA4\x01a\x08IV[` \x81\x01Q`\0\x90a'\x10\x90aD\xE3\x90a\xFF\xFF\x16\x85a]\xBDV[a(\xC9\x91\x90a]\xECV[`@\x81\x01Q`\0\x90a'\x10\x90aD\xE3\x90a\xFF\xFF\x16\x85a]\xBDV[`\0\x80aE\x12aI#V[aE\x1AaIAV[` \x80\x82R\x81\x81\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x88\x90R`\x80\x82\x01\x87\x90R`\xA0\x82\x01\x86\x90R\x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82\x80\x15aE[WaE]V[\xFE[P\x82aE\xABW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FBN254.expMod: call failure\0\0\0\0\0\0`D\x82\x01R`d\x01a\x08IV[PQ\x95\x94PPPPPV[`\0\x80`\0aE\xC5\x85\x85aE\xD2V[\x91P\x91Pa\x1D\xB1\x81aFBV[`\0\x80\x82Q`A\x14\x15aF\tW` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1AaE\xFD\x87\x82\x85\x85aG\xFDV[\x94P\x94PPPPaF;V[\x82Q`@\x14\x15aF3W` \x83\x01Q`@\x84\x01QaF(\x86\x83\x83aH\xEAV[\x93P\x93PPPaF;V[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0\x81`\x04\x81\x11\x15aFVWaFVaL\xD1V[\x14\x15aF_WPV[`\x01\x81`\x04\x81\x11\x15aFsWaFsaL\xD1V[\x14\x15aF\xC1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x08IV[`\x02\x81`\x04\x81\x11\x15aF\xD5WaF\xD5aL\xD1V[\x14\x15aG#W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x08IV[`\x03\x81`\x04\x81\x11\x15aG7WaG7aL\xD1V[\x14\x15aG\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x08IV[`\x04\x81`\x04\x81\x11\x15aG\xA4WaG\xA4aL\xD1V[\x14\x15a\x0C\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 'v' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x08IV[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15aH4WP`\0\x90P`\x03aH\xE1V[\x84`\xFF\x16`\x1B\x14\x15\x80\x15aHLWP\x84`\xFF\x16`\x1C\x14\x15[\x15aH]WP`\0\x90P`\x04aH\xE1V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15aH\xB1W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16aH\xDAW`\0`\x01\x92P\x92PPaH\xE1V[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x80`\x01`\x01`\xFF\x1B\x03\x83\x16\x81aI\x07`\xFF\x86\x90\x1C`\x1BaX*V[\x90PaI\x15\x87\x82\x88\x85aG\xFDV[\x93P\x93PPP\x93P\x93\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`\0\x80\x83`\x1F\x84\x01\x12aIqW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aI\x88W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15aF;W`\0\x80\xFD[`\0\x80` \x83\x85\x03\x12\x15aI\xB6W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aI\xCCW`\0\x80\xFD[aI\xD8\x85\x82\x86\x01aI_V[\x90\x96\x90\x95P\x93PPPPV[`\0` \x82\x84\x03\x12\x15aI\xF6W`\0\x80\xFD[P5\x91\x90PV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0C\xC4W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15aJ$W`\0\x80\xFD[\x835\x92P` \x84\x015aJ6\x81aI\xFDV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aJ\x7FWaJ\x7FaJGV[`@R\x90V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aJ\x7FWaJ\x7FaJGV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aJ\xCFWaJ\xCFaJGV[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x83\x11\x15aJ\xF0WaJ\xF0aJGV[aK\x03`\x1F\x84\x01`\x1F\x19\x16` \x01aJ\xA7V[\x90P\x82\x81R\x83\x83\x83\x01\x11\x15aK\x17W`\0\x80\xFD[\x82\x82` \x83\x017`\0` \x84\x83\x01\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15aK@W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aKVW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13aKgW`\0\x80\xFD[aKv\x84\x825` \x84\x01aJ\xD7V[\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0C\xC4W`\0\x80\xFD[\x805aK\x9E\x81aK~V[\x91\x90PV[`\0` \x82\x84\x03\x12\x15aK\xB5W`\0\x80\xFD[\x815a(\xC9\x81aK~V[`\0\x80`@\x83\x85\x03\x12\x15aK\xD3W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[\x805`\xFF\x81\x16\x81\x14aK\x9EW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aL\x05W`\0\x80\xFD[a(\xC9\x82aK\xE2V[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x81\x01a\x0E\x8FV[`\0\x80\x83`\x1F\x84\x01\x12aL7W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aLNW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aF;W`\0\x80\xFD[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15aL|W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aL\x93W`\0\x80\xFD[aL\x9F\x88\x83\x89\x01aI_V[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15aL\xB8W`\0\x80\xFD[PaL\xC5\x87\x82\x88\x01aL%V[\x95\x98\x94\x97P\x95PPPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x03\x81\x10aM\x05WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[\x81Q\x81R` \x80\x83\x01Q`@\x83\x01\x91aM$\x90\x84\x01\x82aL\xE7V[P\x92\x91PPV[\x805a\xFF\xFF\x81\x16\x81\x14aK\x9EW`\0\x80\xFD[`\0``\x82\x84\x03\x12\x15aMOW`\0\x80\xFD[aMWaJ]V[\x90P\x815aMd\x81aI\xFDV[\x81RaMr` \x83\x01aM+V[` \x82\x01RaM\x83`@\x83\x01aM+V[`@\x82\x01R\x92\x91PPV[`\0\x80`\x80\x83\x85\x03\x12\x15aM\xA1W`\0\x80\xFD[aM\xAA\x83aK\xE2V[\x91PaM\xB9\x84` \x85\x01aM=V[\x90P\x92P\x92\x90PV[`\0\x80`\0`@\x84\x86\x03\x12\x15aM\xD7W`\0\x80\xFD[\x835aM\xE2\x81aK~V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aM\xFDW`\0\x80\xFD[aN\t\x86\x82\x87\x01aL%V[\x94\x97\x90\x96P\x93\x94PPPPV[`\0a\x01\0\x82\x84\x03\x12\x15aN)W`\0\x80\xFD[P\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12aNAW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aNXW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x06\x1B\x85\x01\x01\x11\x15aF;W`\0\x80\xFD[`\0``\x82\x84\x03\x12\x15aN\x85W`\0\x80\xFD[aN\x8DaJ]V[\x90P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aN\xA5W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13aN\xB6W`\0\x80\xFD[aN\xC5\x84\x825` \x84\x01aJ\xD7V[\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\0\x80`\0a\x01\xA0\x8A\x8C\x03\x12\x15aO\x01W`\0\x80\xFD[\x895`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aO\x18W`\0\x80\xFD[aO$\x8D\x83\x8E\x01aL%V[\x90\x9BP\x99P` \x8C\x015\x91P\x80\x82\x11\x15aO=W`\0\x80\xFD[aOI\x8D\x83\x8E\x01aL%V[\x90\x99P\x97P\x87\x91PaO^\x8D`@\x8E\x01aN\x16V[\x96Pa\x01@\x8C\x015\x91P\x80\x82\x11\x15aOuW`\0\x80\xFD[aO\x81\x8D\x83\x8E\x01aN/V[\x90\x96P\x94Pa\x01`\x8C\x015\x91P\x80\x82\x11\x15aO\x9BW`\0\x80\xFD[aO\xA7\x8D\x83\x8E\x01aNsV[\x93Pa\x01\x80\x8C\x015\x91P\x80\x82\x11\x15aO\xBEW`\0\x80\xFD[PaO\xCB\x8C\x82\x8D\x01aNsV[\x91PP\x92\x95\x98P\x92\x95\x98P\x92\x95\x98V[`\0\x80`\0\x80`\0\x80a\x01`\x87\x89\x03\x12\x15aO\xF5W`\0\x80\xFD[\x865`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aP\x0CW`\0\x80\xFD[aP\x18\x8A\x83\x8B\x01aL%V[\x90\x98P\x96P` \x89\x015\x91P\x80\x82\x11\x15aP1W`\0\x80\xFD[aP=\x8A\x83\x8B\x01aL%V[\x90\x96P\x94P\x84\x91PaPR\x8A`@\x8B\x01aN\x16V[\x93Pa\x01@\x89\x015\x91P\x80\x82\x11\x15aPiW`\0\x80\xFD[PaPv\x89\x82\x8A\x01aNsV[\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15aP\x9CWaP\x9CaJGV[P`\x05\x1B` \x01\x90V[`\0\x80`@\x83\x85\x03\x12\x15aP\xB9W`\0\x80\xFD[\x825aP\xC4\x81aI\xFDV[\x91P` \x83\x81\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aP\xE0W`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13aP\xF1W`\0\x80\xFD[\x805aQ\x04aP\xFF\x82aP\x83V[aJ\xA7V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x88\x83\x11\x15aQ#W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aQAW\x835\x82R\x92\x84\x01\x92\x90\x84\x01\x90aQ(V[\x80\x95PPPPPP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aQ\x8EW\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aQlV[P\x90\x96\x95PPPPPPV[`\0\x80` \x83\x85\x03\x12\x15aQ\xADW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aQ\xC3W`\0\x80\xFD[aI\xD8\x85\x82\x86\x01aL%V[`\x01`\x01``\x1B\x03\x81\x16\x81\x14a\x0C\xC4W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12aQ\xF5W`\0\x80\xFD[\x815` aR\x05aP\xFF\x83aP\x83V[\x82\x81R`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aR$W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aRuW`@\x81\x89\x03\x12\x15aRAW`\0\x80\x81\xFD[aRIaJ\x85V[\x815aRT\x81aK~V[\x81R\x81\x85\x015aRc\x81aQ\xCFV[\x81\x86\x01R\x83R\x91\x83\x01\x91`@\x01aR(V[P\x96\x95PPPPPPV[`\0\x80`\0`\xA0\x84\x86\x03\x12\x15aR\x95W`\0\x80\xFD[aR\x9F\x85\x85aM=V[\x92P``\x84\x015aR\xAF\x81aQ\xCFV[\x91P`\x80\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aR\xCAW`\0\x80\xFD[aR\xD6\x86\x82\x87\x01aQ\xE4V[\x91PP\x92P\x92P\x92V[`\0\x82`\x1F\x83\x01\x12aR\xF1W`\0\x80\xFD[\x815` aS\x01aP\xFF\x83aP\x83V[\x82\x81R``\x92\x83\x02\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15aS W`\0\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15aSCWaS6\x89\x82aM=V[\x84R\x92\x84\x01\x92\x81\x01aS$V[P\x90\x97\x96PPPPPPPV[`\0\x82`\x1F\x83\x01\x12aSaW`\0\x80\xFD[\x815` aSqaP\xFF\x83aP\x83V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aS\x90W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aRuW\x805aS\xA7\x81aQ\xCFV[\x83R\x91\x83\x01\x91\x83\x01aS\x94V[`\0\x82`\x1F\x83\x01\x12aS\xC5W`\0\x80\xFD[\x815` aS\xD5aP\xFF\x83aP\x83V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aS\xF4W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aRuW\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aT\x17W`\0\x80\x81\xFD[aT%\x89\x86\x83\x8B\x01\x01aQ\xE4V[\x84RP\x91\x83\x01\x91\x83\x01aS\xF8V[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15aTPW`\0\x80\xFD[aTY\x89aK\x93V[\x97PaTg` \x8A\x01aK\x93V[\x96PaTu`@\x8A\x01aK\x93V[\x95PaT\x83``\x8A\x01aK\x93V[\x94P`\x80\x89\x015\x93P`\xA0\x89\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aT\xA6W`\0\x80\xFD[aT\xB2\x8C\x83\x8D\x01aR\xE0V[\x94P`\xC0\x8B\x015\x91P\x80\x82\x11\x15aT\xC8W`\0\x80\xFD[aT\xD4\x8C\x83\x8D\x01aSPV[\x93P`\xE0\x8B\x015\x91P\x80\x82\x11\x15aT\xEAW`\0\x80\xFD[PaT\xF7\x8B\x82\x8C\x01aS\xB4V[\x91PP\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[`\0`@\x82\x84\x03\x12\x15aU\x19W`\0\x80\xFD[aU!aJ\x85V[\x90PaU,\x82aK\xE2V[\x81R` \x82\x015aU<\x81aK~V[` \x82\x01R\x92\x91PPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aU]W`\0\x80\xFD[\x845\x93P` \x80\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aU{W`\0\x80\xFD[\x86\x01`\x1F\x81\x01\x88\x13aU\x8CW`\0\x80\xFD[\x805aU\x9AaP\xFF\x82aP\x83V[\x81\x81R`\x06\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x8A\x83\x11\x15aU\xB9W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aU\xE2WaU\xD0\x8B\x85aU\x07V[\x82R\x84\x82\x01\x91P`@\x84\x01\x93PaU\xBEV[\x97\x9A\x97\x99PPPP`@\x86\x015\x95``\x015\x94\x93PPPPV[` \x81\x01a\x0E\x8F\x82\x84aL\xE7V[` \x80\x82R`\x19\x90\x82\x01R\x7FPausable: index is paused\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15aV\x81WaV\x81aVWV[P`\x01\x01\x90V[`\0[\x83\x81\x10\x15aV\xA3W\x81\x81\x01Q\x83\x82\x01R` \x01aV\x8BV[\x83\x81\x11\x15a\t0WPP`\0\x91\x01RV[`\0\x81Q\x80\x84RaV\xCC\x81` \x86\x01` \x86\x01aV\x88V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a(\xC9` \x83\x01\x84aV\xB4V[`\0` \x82\x84\x03\x12\x15aW\x05W`\0\x80\xFD[\x81Qa(\xC9\x81aK~V[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15aWlW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a(\xC9W`\0\x80\xFD[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aW\xDBW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aW\xF5W`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15aF;W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aX\x1FW`\0\x80\xFD[\x81Qa(\xC9\x81aI\xFDV[`\0\x82\x19\x82\x11\x15aX=WaX=aVWV[P\x01\x90V[`\0\x80\x85\x85\x11\x15aXRW`\0\x80\xFD[\x83\x86\x11\x15aX_W`\0\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[`\0`@\x82\x84\x03\x12\x15aX~W`\0\x80\xFD[a(\xC9\x83\x83aU\x07V[`\0`\xA0\x82\x01\x87\x83R` \x87\x81\x85\x01R`@`\xA0\x81\x86\x01R\x82\x88Q\x80\x85R`\xC0\x87\x01\x91P\x83\x8A\x01\x94P`\0[\x81\x81\x10\x15aX\xE5W\x85Q\x80Q`\xFF\x16\x84R\x85\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x85\x84\x01R\x94\x84\x01\x94\x91\x83\x01\x91`\x01\x01aX\xB4V[PP``\x86\x01\x97\x90\x97RPPPP`\x80\x01R\x93\x92PPPV[`\0\x82\x82\x10\x15aY\x10WaY\x10aVWV[P\x03\x90V[`\0\x83QaY'\x81\x84` \x88\x01aV\x88V[`\x01`\x01`\xF8\x1B\x03\x19\x93\x90\x93\x16\x91\x90\x92\x01\x90\x81R`\x01\x01\x92\x91PPV[`\x01\x80`\xA0\x1B\x03\x84\x16\x81R\x82` \x82\x01R```@\x82\x01R`\0a!\xEB``\x83\x01\x84aV\xB4V[`\0` \x82\x84\x03\x12\x15aY}W`\0\x80\xFD[\x81Q`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x14a(\xC9W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82aY\xB9WaY\xB9aY\x94V[P\x06\x90V[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R`\0\x90aKv\x90\x83\x01\x84aV\xB4V[\x82\x81R`@` \x82\x01R`\0aKv`@\x83\x01\x84aV\xB4V[`\0` \x82\x84\x03\x12\x15aZ\rW`\0\x80\xFD[PQ\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81Ra\x01`\x81\x01aZ<` \x83\x01\x85\x805\x82R` \x90\x81\x015\x91\x01RV[aZV``\x83\x01`@\x86\x01\x805\x82R` \x90\x81\x015\x91\x01RV[`@`\x80\x85\x01`\xA0\x84\x017`\xE0\x82\x01`\0\x81R`@`\xC0\x86\x01\x827P`\0a\x01 \x83\x01\x90\x81R\x83Q\x90R` \x90\x92\x01Qa\x01@\x90\x91\x01R\x92\x91PPV[`\x01\x80`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R`\0\x82Q```@\x84\x01RaZ\xBD`\xA0\x84\x01\x82aV\xB4V[\x90P` \x84\x01Q``\x84\x01R`@\x84\x01Q`\x80\x84\x01R\x80\x91PP\x93\x92PPPV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R`@` \x82\x01\x81\x90R`\0\x90a!\xEB\x90\x83\x01\x84\x86aZ\xDEV[`\x01\x80`\xA0\x1B\x03\x85\x16\x81R\x83` \x82\x01R```@\x82\x01R`\0a[T``\x83\x01\x84\x86aZ\xDEV[\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a[oW`\0\x80\xFD[\x81Q` a[\x7FaP\xFF\x83aP\x83V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a[\x9EW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aRuW\x80Qa[\xB5\x81aQ\xCFV[\x83R\x91\x83\x01\x91\x83\x01a[\xA2V[`\0\x80`@\x83\x85\x03\x12\x15a[\xD5W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a[\xECW`\0\x80\xFD[a[\xF8\x86\x83\x87\x01a[^V[\x93P` \x85\x01Q\x91P\x80\x82\x11\x15a\\\x0EW`\0\x80\xFD[Pa\\\x1B\x85\x82\x86\x01a[^V[\x91PP\x92P\x92\x90PV[\x83\x81R`@` \x82\x01R`\0a!\xEB`@\x83\x01\x84\x86aZ\xDEV[`\0` \x80\x83\x85\x03\x12\x15a\\RW`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\\hW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\\yW`\0\x80\xFD[\x80Qa\\\x87aP\xFF\x82aP\x83V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a\\\xA6W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\\\xCDW\x83Qa\\\xBE\x81aI\xFDV[\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\\\xABV[\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\\\xEAW`\0\x80\xFD[\x81Qa(\xC9\x81aQ\xCFV[`\0`\xFF\x82\x16`\xFF\x84\x16\x80`\xFF\x03\x82\x11\x15a]\x12Wa]\x12aVWV[\x01\x93\x92PPPV[`\0``\x82\x01`\xFF\x86\x16\x83R` `\x01`\x01``\x1B\x03\x80\x87\x16\x82\x86\x01R`@``\x81\x87\x01R\x83\x87Q\x80\x86R`\x80\x88\x01\x91P\x84\x89\x01\x95P`\0[\x81\x81\x10\x15a]\x83W\x86Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x86\x01Q\x85\x16\x86\x84\x01R\x95\x85\x01\x95\x91\x83\x01\x91`\x01\x01a]SV[P\x90\x9A\x99PPPPPPPPPPV[`\0` \x82\x84\x03\x12\x15a]\xA5W`\0\x80\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a(\xC9W`\0\x80\xFD[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15a]\xE3Wa]\xE3aVWV[\x02\x94\x93PPPPV[`\0`\x01`\x01``\x1B\x03\x80\x84\x16\x80a^\x06Wa^\x06aY\x94V[\x92\x16\x91\x90\x91\x04\x92\x91PPV\xFERegistryCoordinator.updateOperatRegistryCoordinator._validateChu0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDGRegistryCoordinator._deregisterO\xA2dipfsX\"\x12 \x19\xA636\xDA\xB1G\xC6\xCCj\xDF\xAE6\x04\xB8.\x06\xBA\xB0M{q\xE3\x8FV\xA2\xE35\xA3;\xD0\xB3dsolcC\0\x08\x0C\x003";
    /// The deployed bytecode of the contract.
    pub static REGISTRYCOORDINATOR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
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
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    REGISTRYCOORDINATOR_ABI.clone(),
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
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([93, 244, 89, 70], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateOperatorChurnApprovalDigestHash` (0xf8581191) function
        pub fn calculate_operator_churn_approval_digest_hash(
            &self,
            registering_operator_id: [u8; 32],
            operator_kick_params: ::std::vec::Vec<OperatorKickParam>,
            salt: [u8; 32],
            expiry: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [248, 88, 17, 145],
                    (registering_operator_id, operator_kick_params, salt, expiry),
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
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
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
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RegistryCoordinatorEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for RegistryCoordinator<M> {
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
                return Ok(
                    RegistryCoordinatorEvents::ChurnApproverUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = EjectorUpdatedFilter::decode_log(log) {
                return Ok(RegistryCoordinatorEvents::EjectorUpdatedFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(RegistryCoordinatorEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OperatorDeregisteredFilter::decode_log(log) {
                return Ok(
                    RegistryCoordinatorEvents::OperatorDeregisteredFilter(decoded),
                );
            }
            if let Ok(decoded) = OperatorRegisteredFilter::decode_log(log) {
                return Ok(RegistryCoordinatorEvents::OperatorRegisteredFilter(decoded));
            }
            if let Ok(decoded) = OperatorSetParamsUpdatedFilter::decode_log(log) {
                return Ok(
                    RegistryCoordinatorEvents::OperatorSetParamsUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = OperatorSocketUpdateFilter::decode_log(log) {
                return Ok(
                    RegistryCoordinatorEvents::OperatorSocketUpdateFilter(decoded),
                );
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(
                    RegistryCoordinatorEvents::OwnershipTransferredFilter(decoded),
                );
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(RegistryCoordinatorEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = PauserRegistrySetFilter::decode_log(log) {
                return Ok(RegistryCoordinatorEvents::PauserRegistrySetFilter(decoded));
            }
            if let Ok(decoded) = QuorumBlockNumberUpdatedFilter::decode_log(log) {
                return Ok(
                    RegistryCoordinatorEvents::QuorumBlockNumberUpdatedFilter(decoded),
                );
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
            }
        }
    }
    impl ::core::convert::From<ChurnApproverUpdatedFilter>
    for RegistryCoordinatorEvents {
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
    impl ::core::convert::From<OperatorDeregisteredFilter>
    for RegistryCoordinatorEvents {
        fn from(value: OperatorDeregisteredFilter) -> Self {
            Self::OperatorDeregisteredFilter(value)
        }
    }
    impl ::core::convert::From<OperatorRegisteredFilter> for RegistryCoordinatorEvents {
        fn from(value: OperatorRegisteredFilter) -> Self {
            Self::OperatorRegisteredFilter(value)
        }
    }
    impl ::core::convert::From<OperatorSetParamsUpdatedFilter>
    for RegistryCoordinatorEvents {
        fn from(value: OperatorSetParamsUpdatedFilter) -> Self {
            Self::OperatorSetParamsUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<OperatorSocketUpdateFilter>
    for RegistryCoordinatorEvents {
        fn from(value: OperatorSocketUpdateFilter) -> Self {
            Self::OperatorSocketUpdateFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter>
    for RegistryCoordinatorEvents {
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
    impl ::core::convert::From<QuorumBlockNumberUpdatedFilter>
    for RegistryCoordinatorEvents {
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
    ///Container type for all input parameters for the `calculateOperatorChurnApprovalDigestHash` function with signature `calculateOperatorChurnApprovalDigestHash(bytes32,(uint8,address)[],bytes32,uint256)` and selector `0xf8581191`
    #[derive(
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
        abi = "calculateOperatorChurnApprovalDigestHash(bytes32,(uint8,address)[],bytes32,uint256)"
    )]
    pub struct CalculateOperatorChurnApprovalDigestHashCall {
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
    pub enum RegistryCoordinatorCalls {
        OperatorChurnApprovalTypehash(OperatorChurnApprovalTypehashCall),
        PubkeyRegistrationTypehash(PubkeyRegistrationTypehashCall),
        BlsApkRegistry(BlsApkRegistryCall),
        CalculateOperatorChurnApprovalDigestHash(
            CalculateOperatorChurnApprovalDigestHashCall,
        ),
        ChurnApprover(ChurnApproverCall),
        CreateQuorum(CreateQuorumCall),
        DeregisterOperator(DeregisterOperatorCall),
        EjectOperator(EjectOperatorCall),
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
            if let Ok(decoded) = <StakeRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StakeRegistry(decoded));
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
    impl ::ethers::core::abi::AbiEncode for RegistryCoordinatorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::OperatorChurnApprovalTypehash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PubkeyRegistrationTypehash(element) => {
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
                Self::SetOperatorSetParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPauserRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StakeRegistry(element) => {
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
    impl ::core::fmt::Display for RegistryCoordinatorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::OperatorChurnApprovalTypehash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PubkeyRegistrationTypehash(element) => {
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
                Self::SetOperatorSetParams(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetPauserRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeRegistry(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<OperatorChurnApprovalTypehashCall>
    for RegistryCoordinatorCalls {
        fn from(value: OperatorChurnApprovalTypehashCall) -> Self {
            Self::OperatorChurnApprovalTypehash(value)
        }
    }
    impl ::core::convert::From<PubkeyRegistrationTypehashCall>
    for RegistryCoordinatorCalls {
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
    for RegistryCoordinatorCalls {
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
    impl ::core::convert::From<GetQuorumBitmapAtBlockNumberByIndexCall>
    for RegistryCoordinatorCalls {
        fn from(value: GetQuorumBitmapAtBlockNumberByIndexCall) -> Self {
            Self::GetQuorumBitmapAtBlockNumberByIndex(value)
        }
    }
    impl ::core::convert::From<GetQuorumBitmapHistoryLengthCall>
    for RegistryCoordinatorCalls {
        fn from(value: GetQuorumBitmapHistoryLengthCall) -> Self {
            Self::GetQuorumBitmapHistoryLength(value)
        }
    }
    impl ::core::convert::From<GetQuorumBitmapIndicesAtBlockNumberCall>
    for RegistryCoordinatorCalls {
        fn from(value: GetQuorumBitmapIndicesAtBlockNumberCall) -> Self {
            Self::GetQuorumBitmapIndicesAtBlockNumber(value)
        }
    }
    impl ::core::convert::From<GetQuorumBitmapUpdateByIndexCall>
    for RegistryCoordinatorCalls {
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
    impl ::core::convert::From<IsChurnApproverSaltUsedCall>
    for RegistryCoordinatorCalls {
        fn from(value: IsChurnApproverSaltUsedCall) -> Self {
            Self::IsChurnApproverSaltUsed(value)
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
    impl ::core::convert::From<PubkeyRegistrationMessageHashCall>
    for RegistryCoordinatorCalls {
        fn from(value: PubkeyRegistrationMessageHashCall) -> Self {
            Self::PubkeyRegistrationMessageHash(value)
        }
    }
    impl ::core::convert::From<QuorumCountCall> for RegistryCoordinatorCalls {
        fn from(value: QuorumCountCall) -> Self {
            Self::QuorumCount(value)
        }
    }
    impl ::core::convert::From<QuorumUpdateBlockNumberCall>
    for RegistryCoordinatorCalls {
        fn from(value: QuorumUpdateBlockNumberCall) -> Self {
            Self::QuorumUpdateBlockNumber(value)
        }
    }
    impl ::core::convert::From<RegisterOperatorCall> for RegistryCoordinatorCalls {
        fn from(value: RegisterOperatorCall) -> Self {
            Self::RegisterOperator(value)
        }
    }
    impl ::core::convert::From<RegisterOperatorWithChurnCall>
    for RegistryCoordinatorCalls {
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
    impl ::core::convert::From<UpdateOperatorsForQuorumCall>
    for RegistryCoordinatorCalls {
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
    ///Container type for all return fields from the `calculateOperatorChurnApprovalDigestHash` function with signature `calculateOperatorChurnApprovalDigestHash(bytes32,(uint8,address)[],bytes32,uint256)` and selector `0xf8581191`
    #[derive(
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
}
