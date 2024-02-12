pub use bls_registry_coordinator_with_indices::*;
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
pub mod bls_registry_coordinator_with_indices {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_slasher"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract ISlasher"),
                        ),
                    },
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
                        name: ::std::borrow::ToOwned::to_owned("_blsPubkeyRegistry"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "contract IBLSPubkeyRegistry",
                            ),
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
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IBLSRegistryCoordinatorWithIndices.OperatorKickParam[]",
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
                    ::std::borrow::ToOwned::to_owned(
                        "deregisterOperatorWithCoordinator",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "deregisterOperatorWithCoordinator",
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
                                    name: ::std::borrow::ToOwned::to_owned("pubkey"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "deregisterOperatorWithCoordinator",
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
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "deregistrationData",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("ejectOperatorFromCoordinator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ejectOperatorFromCoordinator",
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pubkey"),
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
                    ::std::borrow::ToOwned::to_owned(
                        "getCurrentQuorumBitmapByOperatorId",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getCurrentQuorumBitmapByOperatorId",
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
                                            "struct IRegistryCoordinator.Operator",
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
                                            "struct IBLSRegistryCoordinatorWithIndices.OperatorSetParam",
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
                        "getQuorumBitmapByOperatorIdAtBlockNumberByIndex",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getQuorumBitmapByOperatorIdAtBlockNumberByIndex",
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
                    ::std::borrow::ToOwned::to_owned(
                        "getQuorumBitmapIndicesByOperatorIdsAtBlockNumber",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getQuorumBitmapIndicesByOperatorIdsAtBlockNumber",
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
                    ::std::borrow::ToOwned::to_owned(
                        "getQuorumBitmapUpdateByOperatorIdByIndex",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getQuorumBitmapUpdateByOperatorIdByIndex",
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
                    ::std::borrow::ToOwned::to_owned(
                        "getQuorumBitmapUpdateByOperatorIdLength",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getQuorumBitmapUpdateByOperatorIdLength",
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
                                            "struct IBLSRegistryCoordinatorWithIndices.OperatorSetParam[]",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("registerOperatorWithCoordinator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "registerOperatorWithCoordinator",
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
                                    name: ::std::borrow::ToOwned::to_owned("pubkey"),
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
                                    name: ::std::borrow::ToOwned::to_owned("socket"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IBLSRegistryCoordinatorWithIndices.OperatorKickParam[]",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "signatureWithSaltAndExpiry",
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
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "registerOperatorWithCoordinator",
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
                                    name: ::std::borrow::ToOwned::to_owned("registrationData"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "registerOperatorWithCoordinator",
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
                                    name: ::std::borrow::ToOwned::to_owned("pubkey"),
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
                                    name: ::std::borrow::ToOwned::to_owned("operatorSetParam"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IBLSRegistryCoordinatorWithIndices.OperatorSetParam",
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
                    ::std::borrow::ToOwned::to_owned("slasher"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("slasher"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ISlasher"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
    pub static BLSREGISTRYCOORDINATORWITHINDICES_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01\xE0`@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@Qb\0Q\xBF8\x03\x80b\0Q\xBF\x839\x81\x01`@\x81\x90Rb\0\x005\x91b\0\x01\x83V[`@\x80Q\x80\x82\x01\x82R`\x16\x81R\x7FAVSRegistryCoordinator\0\0\0\0\0\0\0\0\0\0` \x80\x83\x01\x91\x82R\x83Q\x80\x85\x01\x90\x94R`\x06\x84Rev0.0.1`\xD0\x1B\x90\x84\x01R\x81Q\x90 `\xE0\x81\x90R\x7Fk\xDA~?8^H\x84\x10H9\x04D\xCC\xED\\\xC7\x95\xAF\x87u\x8A\xF6v\"\xE5\xF4\xF0\x88,J\x99a\x01\0\x81\x90RF`\xA0R\x91\x92\x91\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0Fb\0\x01-\x81\x84\x84`@\x80Q` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90RF`\x80\x82\x01R0`\xA0\x82\x01R`\0\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x93\x92PPPV[`\x80R0`\xC0Ra\x01 RPPPP`\x01`\x01`\xA0\x1B\x03\x94\x85\x16a\x01@R\x92\x84\x16a\x01`R\x90\x83\x16a\x01\xA0R\x82\x16a\x01\x80R\x16a\x01\xC0Rb\0\x02\x03V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01\x80W`\0\x80\xFD[PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15b\0\x01\x9CW`\0\x80\xFD[\x85Qb\0\x01\xA9\x81b\0\x01jV[` \x87\x01Q\x90\x95Pb\0\x01\xBC\x81b\0\x01jV[`@\x87\x01Q\x90\x94Pb\0\x01\xCF\x81b\0\x01jV[``\x87\x01Q\x90\x93Pb\0\x01\xE2\x81b\0\x01jV[`\x80\x87\x01Q\x90\x92Pb\0\x01\xF5\x81b\0\x01jV[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x01@Qa\x01`Qa\x01\x80Qa\x01\xA0Qa\x01\xC0QaN\xC5b\0\x02\xFA`\09`\0\x81\x81a\x05\x89\x01R\x81\x81a\x18x\x01R\x81\x81a&F\x01Ra0&\x01R`\0\x81\x81a\x05)\x01R\x81\x81a\r{\x01R\x81\x81a\x0Eb\x01R\x81\x81a\x0E\xD3\x01R\x81\x81a\x18\x02\x01R\x81\x81a%\xC1\x01Ra/\xA6\x01R`\0\x81\x81a\x04-\x01R\x81\x81a\x12\t\x01R\x81\x81a\x18A\x01R\x81\x81a%)\x01Ra/&\x01R`\0\x81\x81a\x04T\x01R\x81\x81a\x12~\x01R\x81\x81a\x139\x01Ra\x1B\xA1\x01R`\0a\x05\xC3\x01R`\0a9\xD0\x01R`\0a:\x1F\x01R`\0a9\xFA\x01R`\0a9S\x01R`\0a9}\x01R`\0a9\xA7\x01RaN\xC5`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02HW`\x005`\xE0\x1C\x80cXe\xC6\x0C\x11a\x01;W\x80c\xAB&\x9B-\x11a\0\xB8W\x80c\xCA\r\xE8\x82\x11a\0|W\x80c\xCA\r\xE8\x82\x14a\x06\x1EW\x80c\xD7-\x8D\xD6\x14a\x06EW\x80c\xE6W\x97\xAD\x14a\x06MW\x80c\xFA\xBC\x1C\xBC\x14a\x06\xF0W\x80c\xFD9\x10Z\x14a\x07\x03W`\0\x80\xFD[\x80c\xAB&\x9B-\x14a\x05\xABW\x80c\xB14Bq\x14a\x05\xBEW\x80c\xB84z\xCC\x14a\x05\xE5W\x80c\xC6j\xB9\xCA\x14a\x05\xF8W\x80c\xC8\x1B\x1F\xF4\x14a\x06\x0BW`\0\x80\xFD[\x80ccG\xC9\0\x11a\0\xFFW\x80ccG\xC9\0\x14a\x05\x11W\x80ch0H5\x14a\x05$W\x80c\x85\x02\rI\x14a\x05KW\x80c\x88o\x11\x95\x14a\x05kW\x80c\x9E\x99#\xC2\x14a\x05\x84W`\0\x80\xFD[\x80cXe\xC6\x0C\x14a\x04\xAFW\x80cY\\jg\x14a\x04\xCFW\x80cZ\xC8j\xB7\x14a\x04\xD7W\x80c[\x0B\x82\x9F\x14a\x04\xF6W\x80c\\\x97Z\xBB\x14a\x05\tW`\0\x80\xFD[\x80c)k\xB0d\x11a\x01\xC9W\x80c5a\xDE\xB1\x11a\x01\x8DW\x80c5a\xDE\xB1\x14a\x04(W\x80c9\x98\xFD\xD3\x14a\x04OW\x80cEZC\xFC\x14a\x04vW\x80cKv\xB9\xD5\x14a\x04\x89W\x80cRn\xA9N\x14a\x04\x9CW`\0\x80\xFD[\x80c)k\xB0d\x14a\x03\xB1W\x80c)\xD1\xE0\xC3\x14a\x03\xC4W\x80c,\xDD\x1E\x86\x14a\x03\xD7W\x80c0db\r\x14a\x03\xEAW\x80c41\xAF%\x14a\x04\x15W`\0\x80\xFD[\x80c\x13T*N\x11a\x02\x10W\x80c\x13T*N\x14a\x03\x1CW\x80c\x13d9\xDD\x14a\x03EW\x80c\x14x\x85\x1F\x14a\x03XW\x80c%\0uv\x14a\x03\x8BW\x80c(\xF6\x1B1\x14a\x03\x9EW`\0\x80\xFD[\x80c\x01Y\xF1\xCE\x14a\x02MW\x80c\x05C\x10\xE6\x14a\x02\x9BW\x80c\x05Zb\xB6\x14a\x02\xC6W\x80c\x0C\xF4\xB7g\x14a\x02\xF4W\x80c\x10\xD6z/\x14a\x03\tW[`\0\x80\xFD[a\x02`a\x02[6`\x04a=\xDAV[a\x07?V[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x80\x85\x01Q\x90\x91\x16\x90\x82\x01R\x91\x81\x01Q`\x01`\x01`\xC0\x1B\x03\x16\x90\x82\x01R``\x01[`@Q\x80\x91\x03\x90\xF3[`7Ta\x02\xAE\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x92V[a\x02\xE6a\x02\xD46`\x04a=\xFCV[`\0\x90\x81R`3` R`@\x90 T\x90V[`@Q\x90\x81R` \x01a\x02\x92V[a\x03\x07a\x03\x026`\x04a?\x01V[a\x07\xD0V[\0[a\x03\x07a\x03\x176`\x04a?ZV[a\x08\xD1V[a\x02\xE6a\x03*6`\x04a?ZV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`4` R`@\x90 T\x90V[a\x03\x07a\x03S6`\x04a=\xFCV[a\t\x84V[a\x03{a\x03f6`\x04a=\xFCV[`5` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02\x92V[a\x03\x07a\x03\x996`\x04a@uV[a\n\xC1V[`8Ta\x02\xAE\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02\xAEa\x03\xBF6`\x04a=\xFCV[a\x11\xF0V[a\x03\x07a\x03\xD26`\x04a?ZV[a\x12|V[a\x03\x07a\x03\xE56`\x04a?ZV[a\x137V[a\x03\xFDa\x03\xF86`\x04aA\x89V[a\x13\xF2V[`@Q`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x92V[a\x03\xFDa\x04#6`\x04a=\xFCV[a\x15\xF3V[a\x02\xAE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xAE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\x07a\x04\x846`\x04aA\xC1V[a\x16\xA9V[a\x03\x07a\x04\x976`\x04aB\x9DV[a\x16\xE2V[a\x03\x07a\x04\xAA6`\x04aCzV[a\x1A\x0EV[a\x04\xC2a\x04\xBD6`\x04a?ZV[a\x1A_V[`@Qa\x02\x92\x91\x90aD\x1DV[a\x03\x07a\x1A\xD3V[a\x03{a\x04\xE56`\x04aDPV[`\x01\x80T`\xFF\x90\x92\x16\x1B\x90\x81\x16\x14\x90V[a\x03\x07a\x05\x046`\x04aDkV[a\x1B\x9FV[`\x01Ta\x02\xE6V[a\x02\xAEa\x05\x1F6`\x04a=\xFCV[a\x1C_V[a\x02\xAE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x05^a\x05Y6`\x04aD\x9FV[a\x1C\x89V[`@Qa\x02\x92\x91\x90aEDV[`\0Ta\x02\xAE\x90b\x01\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02\xAE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xE6a\x05\xB96`\x04aE\xD5V[a\x1F\xC4V[a\x02\xAE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\x07a\x05\xF36`\x04aF\x89V[a (V[a\x03\x07a\x06\x066`\x04aF\xF0V[a \xB3V[a\x03\x07a\x06\x196`\x04aCzV[a!\x1EV[a\x02\xE6\x7F\xAEc\xAA\x13\x18u\x1Fd\xEF\x85\x0E\x04T\x17\xF8h\x11\xEA\xEC\xEE\x1C\xEB\xC2&\x90M#{\x80pM\x92\x81V[`6Ta\x02\xE6V[a\x06\xBCa\x06[6`\x04aDPV[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x92\x84\x01\x81\x90R`\xFF\x94\x90\x94\x16\x84R`2\x82R\x92\x82\x90 \x82Q\x93\x84\x01\x83RTc\xFF\xFF\xFF\xFF\x81\x16\x84Ra\xFF\xFF`\x01` \x1B\x82\x04\x81\x16\x92\x85\x01\x92\x90\x92R`\x01`0\x1B\x90\x04\x16\x90\x82\x01R\x90V[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x16\x81R` \x80\x84\x01Qa\xFF\xFF\x90\x81\x16\x91\x83\x01\x91\x90\x91R\x92\x82\x01Q\x90\x92\x16\x90\x82\x01R``\x01a\x02\x92V[a\x03\x07a\x06\xFE6`\x04a=\xFCV[a!aV[a\x072a\x07\x116`\x04a?ZV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`4` R`@\x90 `\x01\x01T`\xFF\x16\x90V[`@Qa\x02\x92\x91\x90aGtV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R`\0\x83\x81R`3` R`@\x90 \x80T\x83\x90\x81\x10a\x07|Wa\x07|aG\x82V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01`\xC0\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90P[\x92\x91PPV[`\x013`\0\x90\x81R`4` R`@\x90 `\x01\x01T`\xFF\x16`\x02\x81\x11\x15a\x07\xF9Wa\x07\xF9aC\xE5V[\x14a\x08\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`K`$\x82\x01R\x7FBLSRegistryCoordinatorWithIndici`D\x82\x01R\x7Fes.updateSocket: operator is not`d\x82\x01Rj\x08\x1C\x99Y\xDA\\\xDD\x19\\\x99Y`\xAA\x1B`\x84\x82\x01R`\xA4\x01[`@Q\x80\x91\x03\x90\xFD[3`\0\x90\x81R`4` R`@\x90\x81\x90 T\x90Q\x7F\xEC)c\xAB!\xC1\xE5\x0E\x1EX*\xA5B\xAF.K\xF7\xBF8\xE6\xE1@<'\xB4.\x1C]nb\x1E\xAA\x90a\x08\xC6\x90\x84\x90aG\xF0V[`@Q\x80\x91\x03\x90\xA2PV[`\0`\x02\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t$W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tH\x91\x90aH\x03V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\txW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08|\x90aH V[a\t\x81\x81a\"\xBDV[PV[`\0T`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01Rb\x01\0\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xD1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xF5\x91\x90aHjV[a\n\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08|\x90aH\x8CV[`\x01T\x81\x81\x16\x14a\n\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08|V[`\x01\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01a\x08\xC6V[`\x01\x80T`\0\x91\x90\x81\x16\x14\x15a\n\xE9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08|\x90aH\xD4V[`\0a\x0B.3\x8B\x8B\x8B\x8B\x8B\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa#\xC2\x92PPPV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x91\x92P`\0\x91\x90` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90Pa\x0B]\x89a*\x15V[\x81`\0\x81Q\x81\x10a\x0BpWa\x0BpaG\x82V[` \x02` \x01\x01\x81\x81RPPa\x0B\xF3\x81`\0\x81Q\x81\x10a\x0B\x92Wa\x0B\x92aG\x82V[` \x02` \x01\x01Q\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x0B\xE8Wa\x0B\xD9`\x80\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aI\x0BV[\x81R` \x01\x90`\x01\x01\x90a\x0B\xBCV[PPPPP\x86a*XV[`\0\x80[\x8B\x81\x10\x15a\x11\xE1W`\0\x8D\x8D\x83\x81\x81\x10a\x0C\x13Wa\x0C\x13aG\x82V[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`2` \x90\x81R`@\x91\x82\x90 \x82Q``\x81\x01\x84R\x90Tc\xFF\xFF\xFF\xFF\x81\x16\x80\x83Ra\xFF\xFF`\x01` \x1B\x83\x04\x81\x16\x94\x84\x01\x94\x90\x94R`\x01`0\x1B\x90\x91\x04\x90\x92\x16\x92\x81\x01\x92\x90\x92R\x88Q\x92\x94P\x90\x92P\x90\x87\x90\x85\x90\x81\x10a\x0C\x81Wa\x0C\x81aG\x82V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11a\x0C\x9BWPPa\x11\xCFV[\x81`\xFF\x16\x8A\x8A\x86\x81\x81\x10a\x0C\xB1Wa\x0C\xB1aG\x82V[a\x0C\xC7\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91PaDPV[`\xFF\x16\x14a\r`W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`f`$\x82\x01R`\0\x80Q` aNp\x839\x81Q\x91R`D\x82\x01R\x7Fs.registerOperatorWithCoordinato`d\x82\x01R\x7Fr: quorumNumber not the same as `\x84\x82\x01Re\x1C\xDAY\xDB\x99Y`\xD2\x1B`\xA4\x82\x01R`\xC4\x01a\x08|V[`@Qc\x0C\x8Fs\x9D`\xE4\x1B\x81R`\xFF\x83\x16`\x04\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xC8\xF79\xD0\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xCAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xEE\x91\x90aI'V[\x90P`\0`4`\0\x8D\x8D\x88\x81\x81\x10a\x0E\x08Wa\x0E\x08aG\x82V[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\x0E \x91\x90a?ZV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x82\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 T\x91Qc\x1A\xADN5`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`\xFF\x88\x16`$\x82\x01R\x91\x93P\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cj\xB58\xD4\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xA9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xCD\x91\x90aI'V[\x90P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cj\xB58\xD4\x8A`\0\x81Q\x81\x10a\x0F\x13Wa\x0F\x13aG\x82V[` \x02` \x01\x01Q\x88`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0FE\x92\x91\x90\x91\x82R`\xFF\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0FbW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x86\x91\x90aI'V[` \x86\x01Q\x90\x91Pa'\x10\x90a\x0F\xA0\x90a\xFF\xFF\x16\x84aIfV[a\x0F\xAA\x91\x90aI\x95V[`\x01`\x01``\x1B\x03\x16\x81`\x01`\x01``\x1B\x03\x16\x11a\x10jW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`}`$\x82\x01R`\0\x80Q` aNp\x839\x81Q\x91R`D\x82\x01R\x7Fs.registerOperatorWithCoordinato`d\x82\x01R\x7Fr: registering operator has less`\x84\x82\x01R\x7F than kickBIPsOfOperatorStake\0\0\0`\xA4\x82\x01R`\xC4\x01a\x08|V[`@\x85\x01Qa'\x10\x90a\x10\x81\x90a\xFF\xFF\x16\x86aIfV[a\x10\x8B\x91\x90aI\x95V[`\x01`\x01``\x1B\x03\x16\x82`\x01`\x01``\x1B\x03\x16\x10a\x11DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`v`$\x82\x01R`\0\x80Q` aNp\x839\x81Q\x91R`D\x82\x01R\x7Fs.registerOperatorWithCoordinato`d\x82\x01R\x7Fr: operator to kick has more tha`\x84\x82\x01Run kickBIPSOfTotalStake`P\x1B`\xA4\x82\x01R`\xC4\x01a\x08|V[\x87a\x11N\x81aI\xC9V[\x98PPPPPPPPa\x11\xCF\x88\x88\x83\x81\x81\x10a\x11lWa\x11laG\x82V[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\x11\x84\x91\x90a?ZV[\x8E\x83\x8Fa\x11\x92\x82`\x01aI\xE4V[\x92a\x11\x9F\x93\x92\x91\x90aI\xFCV[\x8B\x8B\x86\x81\x81\x10a\x11\xB1Wa\x11\xB1aG\x82V[\x90P`\x80\x02\x01`@\x01\x806\x03\x81\x01\x90a\x11\xCA\x91\x90aJ&V[a,%V[\x80a\x11\xD9\x81aI\xC9V[\x91PPa\x0B\xF7V[PPPPPPPPPPPPPV[`@Qc\x08\xF6b\x9D`\xE3\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cG\xB3\x14\xE8\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12XW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xCA\x91\x90aH\x03V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xDAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xFE\x91\x90aH\x03V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x13.W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08|\x90aJBV[a\t\x81\x81a1\xD2V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\x95W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xB9\x91\x90aH\x03V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x13\xE9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08|\x90aJBV[a\t\x81\x81a2;V[`\0\x83\x81R`3` R`@\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x14\x15Wa\x14\x15aG\x82V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x85R`\x01` \x1B\x83\x04\x82\x16\x95\x85\x01\x95\x90\x95R`\x01`@\x1B\x90\x91\x04`\x01`\x01`\xC0\x1B\x03\x16\x91\x83\x01\x91\x90\x91R\x90\x92P\x85\x16\x10\x15a\x15\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x7F`$\x82\x01R`\0\x80Q` aNp\x839\x81Q\x91R`D\x82\x01R\x7Fs.getQuorumBitmapByOperatorIdAtB`d\x82\x01R\x7FlockNumberByIndex: quorumBitmapU`\x84\x82\x01R\x7Fpdate is from after blockNumber\0`\xA4\x82\x01R`\xC4\x01a\x08|V[\x83c\xFF\xFF\xFF\xFF\x16\x81` \x01Qc\xFF\xFF\xFF\xFF\x16\x11\x80a\x15;WP` \x81\x01Qc\xFF\xFF\xFF\xFF\x16\x15[a\x15\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x80`$\x82\x01R`\0\x80Q` aNp\x839\x81Q\x91R`D\x82\x01R\x7Fs.getQuorumBitmapByOperatorIdAtB`d\x82\x01R\x7FlockNumberByIndex: quorumBitmapU`\x84\x82\x01R\x7Fpdate is from before blockNumber`\xA4\x82\x01R`\xC4\x01a\x08|V[`@\x01Q\x94\x93PPPPV[`\0\x81\x81R`3` R`@\x81 T\x80\x15\x80a\x16OWP`\0\x83\x81R`3` R`@\x90 a\x16#`\x01\x83aJ\xBEV[\x81T\x81\x10a\x163Wa\x163aG\x82V[`\0\x91\x82R` \x90\x91 \x01T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15\x15[\x15a\x16]WP`\0\x92\x91PPV[`\0\x83\x81R`3` R`@\x90 a\x16v`\x01\x83aJ\xBEV[\x81T\x81\x10a\x16\x86Wa\x16\x86aG\x82V[`\0\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01`\xC0\x1B\x03\x16\x93\x92PPPV[`\x01\x80T`\x02\x90\x81\x16\x14\x15a\x16\xD0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08|\x90aH\xD4V[a\x16\xDC3\x85\x85\x85a,%V[PPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x17\x02WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x17\x1CWP0;\x15\x80\x15a\x17\x1CWP`\0T`\xFF\x16`\x01\x14[a\x17\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x08|V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x17\xA2W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x17\xAC\x83\x83a2\xA4V[a\x17\xB5\x86a1\xD2V[a\x17\xBE\x85a2;V[`6\x80T`\x01\x80\x82\x01\x83U`\0\x83\x90R\x7FJ\x11\xF9N \xA9<y\xF6\xECt:\x19T\xECO\xC2\xC0\x84)\xAE!\"\x11\x8B\xF24\xB2\x18\\\x81\xB8\x91\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x82\x17\x90\x93U\x85T\x80\x85\x01\x87U\x85\x01\x80T\x83\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x16\x17\x90U\x85T\x93\x84\x01\x90\x95U\x91\x90\x92\x01\x80T\x90\x91\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x16\x91\x90\x91\x17\x90U\x84Q`@\x80Qc\x9A\xA1e=`\xE0\x1B\x81R\x90Q\x91\x92\x91c\x9A\xA1e=\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x18\xE0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\x04\x91\x90aJ\xD5V[a\xFF\xFF\x16\x14a\x19xW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`F`$\x82\x01R`\0\x80Q` aNp\x839\x81Q\x91R`D\x82\x01R\x7Fs: operator set params length mi`d\x82\x01Re\x0Em\xAC.\x8Cm`\xD3\x1B`\x84\x82\x01R`\xA4\x01a\x08|V[`\0[\x84Q\x81`\xFF\x16\x10\x15a\x19\xBFWa\x19\xAD\x81\x86\x83`\xFF\x16\x81Q\x81\x10a\x19\xA0Wa\x19\xA0aG\x82V[` \x02` \x01\x01Qa3\x90V[\x80a\x19\xB7\x81aJ\xF2V[\x91PPa\x19{V[P\x80\x15a\x1A\x06W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPV[`\x01\x80T`\0\x91\x90\x81\x16\x14\x15a\x1A6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08|\x90aH\xD4V[`\0\x80a\x1AE\x84\x86\x01\x86aK\x12V[\x91P\x91Pa\x1AV3\x88\x88\x85\x85a4=V[PPPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`4` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x81\x01T\x90\x91\x83\x01\x90`\xFF\x16`\x02\x81\x11\x15a\x1A\xB9Wa\x1A\xB9aC\xE5V[`\x02\x81\x11\x15a\x1A\xCAWa\x1A\xCAaC\xE5V[\x90RP\x92\x91PPV[`\0T`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01Rb\x01\0\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1BD\x91\x90aHjV[a\x1B`W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08|\x90aH\x8CV[`\0\x19`\x01\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xFDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C!\x91\x90aH\x03V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1CQW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08|\x90aJBV[a\x1C[\x82\x82a3\x90V[PPV[`6\x81\x81T\x81\x10a\x1CoW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x81V[```\0\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1C\xA6Wa\x1C\xA6a>\x15V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1C\xCFW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\x1F\xBCW`\0`3`\0\x86\x84\x81Q\x81\x10a\x1C\xF6Wa\x1C\xF6aG\x82V[` \x02` \x01\x01Q\x81R` \x01\x90\x81R` \x01`\0 \x80T\x90P\x90P`\0[\x81c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x1F\xA7W\x86c\xFF\xFF\xFF\xFF\x16`3`\0\x88\x86\x81Q\x81\x10a\x1DFWa\x1DFaG\x82V[` \x02` \x01\x01Q\x81R` \x01\x90\x81R` \x01`\0 `\x01\x83\x85a\x1Dj\x91\x90aK`V[a\x1Dt\x91\x90aK`V[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x1D\x8AWa\x1D\x8AaG\x82V[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a\x1F\x95W`3`\0\x87\x85\x81Q\x81\x10a\x1D\xB7Wa\x1D\xB7aG\x82V[` \x02` \x01\x01Q\x81R` \x01\x90\x81R` \x01`\0 `\x01\x82\x84a\x1D\xDB\x91\x90aK`V[a\x1D\xE5\x91\x90aK`V[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x1D\xFBWa\x1D\xFBaG\x82V[`\0\x91\x82R` \x90\x91 \x01T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15\x80a\x1E\x97WP\x86c\xFF\xFF\xFF\xFF\x16`3`\0\x88\x86\x81Q\x81\x10a\x1E8Wa\x1E8aG\x82V[` \x02` \x01\x01Q\x81R` \x01\x90\x81R` \x01`\0 `\x01\x83\x85a\x1E\\\x91\x90aK`V[a\x1Ef\x91\x90aK`V[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x1E|Wa\x1E|aG\x82V[`\0\x91\x82R` \x90\x91 \x01T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x11[a\x1FNW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x82`$\x82\x01R`\0\x80Q` aNp\x839\x81Q\x91R`D\x82\x01R\x7Fs.getQuorumBitmapIndicesByOperat`d\x82\x01R\x7ForIdsAtBlockNumber: operatorId h`\x84\x82\x01R\x7Fas no quorumBitmaps at blockNumb`\xA4\x82\x01Ra2\xB9`\xF1\x1B`\xC4\x82\x01R`\xE4\x01a\x08|V[`\x01a\x1FZ\x82\x84aK`V[a\x1Fd\x91\x90aK`V[\x84\x84\x81Q\x81\x10a\x1FvWa\x1FvaG\x82V[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPPa\x1F\xA7V[\x80a\x1F\x9F\x81aK\x85V[\x91PPa\x1D\x15V[PP\x80\x80a\x1F\xB4\x90aI\xC9V[\x91PPa\x1C\xD5V[P\x93\x92PPPV[`\0a \x1D\x7F\xAEc\xAA\x13\x18u\x1Fd\xEF\x85\x0E\x04T\x17\xF8h\x11\xEA\xEC\xEE\x1C\xEB\xC2&\x90M#{\x80pM\x92\x86\x86\x86\x86`@Q` \x01a \x02\x95\x94\x93\x92\x91\x90aK\xA9V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a5TV[\x90P[\x94\x93PPPPV[`8T`\x01`\x01`\xA0\x1B\x03\x163\x14a \xA7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`H`$\x82\x01R`\0\x80Q` aNp\x839\x81Q\x91R`D\x82\x01R\x7Fs.onlyEjector: caller is not the`d\x82\x01Rg\x102\xB52\xB1\xBA7\xB9`\xC1\x1B`\x84\x82\x01R`\xA4\x01a\x08|V[a\x16\xDC\x84\x84\x84\x84a,%V[`\x01\x80T`\0\x91\x90\x81\x16\x14\x15a \xDBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08|\x90aH\xD4V[a\x1A\x063\x87\x87\x87\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa4=\x92PPPV[`\x01\x80T`\x02\x90\x81\x16\x14\x15a!EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08|\x90aH\xD4V[`\0a!S\x83\x85\x01\x85aJ&V[\x90Pa\x1A\x063\x87\x87\x84a,%V[`\0`\x02\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!\xB4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\xD8\x91\x90aH\x03V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\"\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08|\x90aH V[`\x01T\x19\x81\x19`\x01T\x19\x16\x14a\"\x86W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08|V[`\x01\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x08\xC6V[`\x01`\x01`\xA0\x1B\x03\x81\x16a#KW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x08|V[`\0T`@\x80Q`\x01`\x01`\xA0\x1B\x03b\x01\0\0\x90\x93\x04\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`\0\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16b\x01\0\0\x02b\x01\0\0`\x01`\xB0\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[```\0a$\x05\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa5\xA2\x92PPPV[\x90P`\x01`\x01`\xC0\x1B\x03\x81\x11\x15a$\x9AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`k`$\x82\x01R`\0\x80Q` aNp\x839\x81Q\x91R`D\x82\x01R`\0\x80Q` aNP\x839\x81Q\x91R`d\x82\x01R\x7For: quorumBitmap exceeds of max `\x84\x82\x01Rjbitmap size`\xA8\x1B`\xA4\x82\x01R`\xC4\x01a\x08|V[\x80a%\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R`\0\x80Q` aNp\x839\x81Q\x91R`D\x82\x01R`\0\x80Q` aNP\x839\x81Q\x91R`d\x82\x01R\x7For: quorumBitmap cannot be 0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x08|V[`@Qc\x03\xCEK\xAD`\xE0\x1B\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x03\xCEK\xAD\x90a%d\x90\x8B\x90\x8B\x90\x8B\x90\x8B\x90`\x04\x01aLfV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a%\x83W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%\xA7\x91\x90aL\xA3V[`@Qc%PGw`\xE0\x1B\x81R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c%PGw\x90a%\xFC\x90\x8B\x90\x85\x90\x8C\x90\x8C\x90`\x04\x01aL\xBCV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a&\x16W`\0\x80\xFD[PZ\xF1\x15\x80\x15a&*W=`\0\x80>=`\0\xFD[PP`@Qb\xBF\xF0M`\xE0\x1B\x81R`\0\x92P`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91Pb\xBF\xF0M\x90a&\x7F\x90\x85\x90\x8C\x90\x8C\x90`\x04\x01aL\xEEV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a&\x9EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra&\xC6\x91\x90\x81\x01\x90aM\x08V[`\0\x83\x81R`3` R`@\x90 T\x90\x91P\x80\x15\x80\x15\x90a'&WP`\0\x83\x81R`3` R`@\x90 a&\xFB`\x01\x83aJ\xBEV[\x81T\x81\x10a'\x0BWa'\x0BaG\x82V[`\0\x91\x82R` \x90\x91 \x01T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15[\x15a(}W`\0\x83\x81R`3` R`@\x81 a'D`\x01\x84aJ\xBEV[\x81T\x81\x10a'TWa'TaG\x82V[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xC0\x1B\x03`\x01`@\x1B\x90\x91\x04\x81\x81\x16\x92P\x86\x16\x16\x15a(!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x85`$\x82\x01R`\0\x80Q` aNp\x839\x81Q\x91R`D\x82\x01R`\0\x80Q` aNP\x839\x81Q\x91R`d\x82\x01R\x7For: operator already registered `\x84\x82\x01R\x7Ffor some quorums being registere`\xA4\x82\x01Rd2\x1037\xB9`\xD9\x1B`\xC4\x82\x01R`\xE4\x01a\x08|V[`\0\x84\x81R`3` R`@\x90 \x94\x81\x17\x94C\x90a(@`\x01\x85aJ\xBEV[\x81T\x81\x10a(PWa(PaG\x82V[\x90`\0R` `\0 \x01`\0\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPP[`\0\x83\x81R`3` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x86\x81R`\x01`\x01`\xC0\x1B\x03\x8B\x81\x16\x84\x87\x01\x90\x81R\x85T`\x01\x80\x82\x01\x88U\x96\x8AR\x88\x8A \x95Q\x95\x01\x80T\x93Q\x91Q\x90\x92\x16`\x01`@\x1B\x02`\x01`\x01`@\x1B\x03\x91\x85\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x94\x16\x95\x90\x94\x16\x94\x90\x94\x17\x91\x90\x91\x17\x92\x90\x92\x16\x17\x90U`\x01`\x01`\xA0\x1B\x03\x8E\x16\x84R`4\x90\x92R\x90\x91 \x81\x01T`\xFF\x16`\x02\x81\x11\x15a)5Wa)5aC\xE5V[\x14a)\xCEW`@\x80Q\x80\x82\x01\x82R\x84\x81R`\x01` \x80\x83\x01\x82\x81R`\x01`\x01`\xA0\x1B\x03\x8F\x16`\0\x90\x81R`4\x90\x92R\x93\x90 \x82Q\x81U\x92Q\x83\x82\x01\x80T\x93\x94\x93\x91\x92\x90\x91`\xFF\x19\x16\x90\x83`\x02\x81\x11\x15a)\x90Wa)\x90aC\xE5V[\x02\x17\x90UPP`@Q\x84\x91P`\x01`\x01`\xA0\x1B\x03\x8C\x16\x90\x7F\xE8\xE6\x8C\xEF\x1C:v\x1E\xD7\xBE~\x84c\xA3u\xF2\x7F{\xC35\xE5\x18$\"<\xAC\xCEcn\xC5\xC3\xFE\x90`\0\x90\xA3[\x82\x7F\xEC)c\xAB!\xC1\xE5\x0E\x1EX*\xA5B\xAF.K\xF7\xBF8\xE6\xE1@<'\xB4.\x1C]nb\x1E\xAA\x87`@Qa)\xFE\x91\x90aG\xF0V[`@Q\x80\x91\x03\x90\xA2P\x92PPP[\x95\x94PPPPPV[`\0\x81`\0\x01Q\x82` \x01Q`@Q` \x01a*;\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[` \x80\x82\x01Q`\0\x90\x81R`5\x90\x91R`@\x90 T`\xFF\x16\x15a+\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`w`$\x82\x01R`\0\x80Q` aNp\x839\x81Q\x91R`D\x82\x01R\x7Fs._verifyChurnApproverSignatureO`d\x82\x01R\x7FnOperatorChurnApproval: churnApp`\x84\x82\x01R\x7Frover salt already used\0\0\0\0\0\0\0\0\0`\xA4\x82\x01R`\xC4\x01a\x08|V[B\x81`@\x01Q\x10\x15a+\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`w`$\x82\x01R`\0\x80Q` aNp\x839\x81Q\x91R`D\x82\x01R\x7Fs._verifyChurnApproverSignatureO`d\x82\x01R\x7FnOperatorChurnApproval: churnApp`\x84\x82\x01R\x7Frover signature expired\0\0\0\0\0\0\0\0\0`\xA4\x82\x01R`\xC4\x01a\x08|V[` \x80\x82\x01\x80Q`\0\x90\x81R`5\x90\x92R`@\x91\x82\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U`7T\x90Q\x91\x83\x01Qa, \x92`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91a,\x19\x91\x87\x91\x87\x91a\x1F\xC4V[\x83Qa7/V[PPPV[`\x01`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`4` R`@\x90 `\x01\x01T`\xFF\x16`\x02\x81\x11\x15a,WWa,WaC\xE5V[\x14a,\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R```$\x82\x01R`\0\x80Q` aNp\x839\x81Q\x91R`D\x82\x01R\x7Fs._deregisterOperatorWithCoordin`d\x82\x01R\x7Fator: operator is not registered`\x84\x82\x01R`\xA4\x01a\x08|V[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`4` R`@\x90 Ta-\0\x82a*\x15V[\x81\x14a-\x9CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`k`$\x82\x01R`\0\x80Q` aNp\x839\x81Q\x91R`D\x82\x01R\x7Fs._deregisterOperatorWithCoordin`d\x82\x01R\x7Fator: operatorId does not match `\x84\x82\x01Rj\x0E\x0E\xACMl\xAF$\r\x0C.m`\xAB\x1B`\xA4\x82\x01R`\xC4\x01a\x08|V[`\0a-\xDD\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa5\xA2\x92PPPV[`\0\x83\x81R`3` R`@\x81 T\x91\x92P\x90a-\xFC\x90`\x01\x90aJ\xBEV[`\0\x84\x81R`3` R`@\x81 \x80T\x92\x93P\x90\x91\x83\x90\x81\x10a.!Wa.!aG\x82V[`\0\x91\x82R` \x82 \x01T`\x01`@\x1B\x90\x04\x93\x84\x16`\x01`\x01`\xC0\x1B\x03\x90\x81\x16\x94\x16\x91Pa.N\x84a8\xE9V[\x90P\x80Q`\0\x14\x15a/\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x80`$\x82\x01R`\0\x80Q` aNp\x839\x81Q\x91R`D\x82\x01R\x7Fs._deregisterOperatorWithCoordin`d\x82\x01R\x7Fator: operator is not registered`\x84\x82\x01R\x7F for any of the provided quorums`\xA4\x82\x01R`\xC4\x01a\x08|V[`@Qc\x12\x1BM\x95`\xE1\x1B\x81R`\x01`\x01`\xC0\x1B\x03\x83\x16\x85\x14\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c$6\x9B*\x90a/_\x90\x8D\x90\x86\x90\x8C\x90`\x04\x01aM\xA1V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a/yW`\0\x80\xFD[PZ\xF1\x15\x80\x15a/\x8DW=`\0\x80>=`\0\xFD[PP`@Qc\xBD)\xB8\xCD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92Pc\xBD)\xB8\xCD\x91Pa/\xDF\x90\x89\x90\x86\x90`\x04\x01aM\xDDV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a/\xF9W`\0\x80\xFD[PZ\xF1\x15\x80\x15a0\rW=`\0\x80>=`\0\xFD[PP`@Qc\xBD)\xB8\xCD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92Pc\xBD)\xB8\xCD\x91Pa0_\x90\x89\x90\x86\x90`\x04\x01aM\xDDV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a0yW`\0\x80\xFD[PZ\xF1\x15\x80\x15a0\x8DW=`\0\x80>=`\0\xFD[PPP`\0\x87\x81R`3` R`@\x90 \x80TC\x92P\x86\x90\x81\x10a0\xB3Wa0\xB3aG\x82V[\x90`\0R` `\0 \x01`\0\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80a1wW`\0\x86\x81R`3` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x86\x81R`\x01`\x01`\xC0\x1B\x03\x8C\x19\x8B\x16\x81\x16\x95\x84\x01\x95\x86R\x84T`\x01\x81\x01\x86U\x94\x88R\x95\x90\x96 \x91Q\x91\x90\x92\x01\x80T\x95Q\x93Q\x90\x94\x16`\x01`@\x1B\x02`\x01`\x01`@\x1B\x03\x93\x83\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x96\x16\x91\x90\x92\x16\x17\x93\x90\x93\x17\x16\x91\x90\x91\x17\x90Ua1\xC6V[`\x01`\x01`\xA0\x1B\x03\x8A\x16`\0\x81\x81R`4` R`@\x80\x82 `\x01\x01\x80T`\xFF\x19\x16`\x02\x17\x90UQ\x88\x92\x91\x7F9o\xDC\xB1\x80\xCB\x0F\xEA&\x92\x81\x13\xFB\x0F\xD1\xC3T\x98c\xF9\xCDV>j\x18O\x1DW\x81\x16\xC8\xE4\x91\xA3[PPPPPPPPPPV[`7T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F1TW\xD8\xA8\xFE`\xF0J\xF1|\x16\xE2\xF5\xA5\xE1\xDBa+1d\x8EX\x03\x03`u\x9E\xF8\xF3R\x8C\x91\x01`@Q\x80\x91\x03\x90\xA1`7\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`8T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F\x8F0\xAB\t\xF4:l\x15}\x7F\xCE~\n\x13\xC0\x03\x04,\x1C\x95\xE8\xA7.z\x14j!\xC0\xCA\xA2M\xC9\x91\x01`@Q\x80\x91\x03\x90\xA1`8\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0Tb\x01\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a2\xCBWP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a3MW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x08|V[`\x01\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a\x1C[\x82a\"\xBDV[`\xFF\x82\x16`\0\x81\x81R`2` \x90\x81R`@\x91\x82\x90 \x84Q\x81T\x86\x84\x01\x80Q\x88\x87\x01\x80Qc\xFF\xFF\xFF\xFF\x90\x95\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x94\x16\x84\x17`\x01` \x1Ba\xFF\xFF\x93\x84\x16\x02\x17g\xFF\xFF\0\0\0\0\0\0\x19\x16`\x01`0\x1B\x95\x83\x16\x95\x90\x95\x02\x94\x90\x94\x17\x90\x94U\x85Q\x91\x82RQ\x83\x16\x93\x81\x01\x93\x90\x93RQ\x16\x91\x81\x01\x91\x90\x91R\x7F>\xE6\xFE\x8DTa\x02D\xC3\xE9\xD3\xC0f\xAEJ\xEE\x99x\x84\xAA(\xF1\x06\x16\xAE\x82\x19%@\x13\x18\xAC\x90``\x01`@Q\x80\x91\x03\x90\xA2PPV[`\0a4L\x86\x86\x86\x86\x86a#\xC2V[\x90P`\0[\x81Q\x81\x10\x15a\x1AVW`2`\0\x87\x87\x84\x81\x81\x10a4pWa4paG\x82V[\x91\x90\x91\x015`\xF8\x1C\x82RP` \x81\x01\x91\x90\x91R`@\x01`\0 T\x82Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x90\x83\x90\x83\x90\x81\x10a4\xA7Wa4\xA7aG\x82V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a5BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`n`$\x82\x01R`\0\x80Q` aNp\x839\x81Q\x91R`D\x82\x01R`\0\x80Q` aNP\x839\x81Q\x91R`d\x82\x01R\x7ForAndNoOverfilledQuorums: quorum`\x84\x82\x01Rm\x08\x1A\\\xC8\x1B\xDD\x99\\\x99\x9A[\x1B\x19Y`\x92\x1B`\xA4\x82\x01R`\xC4\x01a\x08|V[\x80a5L\x81aI\xC9V[\x91PPa4QV[`\0a\x07\xCAa5aa9FV[\x83`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x83\x90R`B\x81\x01\x82\x90R`\0\x90`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[`\0a\x01\0\x82Q\x11\x15a6+W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FBitmapUtils.orderedBytesArrayToB\x90\x82\x01R\x7Fitmap: orderedBytesArray is too `d\x82\x01Rclong`\xE0\x1B`\x84\x82\x01R`\xA4\x01a\x08|V[\x81Qa69WP`\0\x91\x90PV[`\0\x80\x83`\0\x81Q\x81\x10a6OWa6OaG\x82V[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x81\x90\x1B\x92P[\x84Q\x81\x10\x15a7&W\x84\x81\x81Q\x81\x10a6}Wa6}aG\x82V[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x1B\x91P\x82\x82\x11a7\x12W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: orderedBytesArray is not `d\x82\x01Rf\x1B\xDC\x99\x19\\\x99Y`\xCA\x1B`\x84\x82\x01R`\xA4\x01a\x08|V[\x91\x81\x17\x91a7\x1F\x81aI\xC9V[\x90Pa6bV[P\x90\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16;\x15a8IW`@Qc\x0B\x13]?`\xE1\x1B\x80\x82R\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\x16&\xBA~\x90a7o\x90\x86\x90\x86\x90`\x04\x01aM\xDDV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a7\x8CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a7\xB0\x91\x90aM\xF6V[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14a, W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`S`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: ERC1271 signature `d\x82\x01Rr\x1D\x99\\\x9AY\x9AX\xD8]\x1A[\xDB\x88\x19\x98Z[\x19Y`j\x1B`\x84\x82\x01R`\xA4\x01a\x08|V[\x82`\x01`\x01`\xA0\x1B\x03\x16a8]\x83\x83a:mV[`\x01`\x01`\xA0\x1B\x03\x16\x14a, W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: signature not from`d\x82\x01Rf\x109\xB4\xB3\xB72\xB9`\xC9\x1B`\x84\x82\x01R`\xA4\x01a\x08|V[```\0\x80[a\x01\0\x81\x10\x15a9?W`\x01\x81\x1B\x91P\x83\x82\x16\x15a9/W\x82\x81`\xF8\x1B`@Q` \x01a9\x1D\x92\x91\x90aN V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92P[a98\x81aI\xC9V[\x90Pa8\xEFV[PP\x91\x90PV[`\x000`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80\x15a9\x9FWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14[\x15a9\xC9WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[P`@\x80Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x80\x83\x01\x91\x90\x91R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x84\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x83\x01RF`\x80\x83\x01R0`\xA0\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\xC0\x90\x92\x01\x90\x92R\x80Q\x91\x01 \x90V[`\0\x80`\0a:|\x85\x85a:\x89V[\x91P\x91Pa\x1F\xBC\x81a:\xF9V[`\0\x80\x82Q`A\x14\x15a:\xC0W` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1Aa:\xB4\x87\x82\x85\x85a<\xB4V[\x94P\x94PPPPa:\xF2V[\x82Q`@\x14\x15a:\xEAW` \x83\x01Q`@\x84\x01Qa:\xDF\x86\x83\x83a=\xA1V[\x93P\x93PPPa:\xF2V[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0\x81`\x04\x81\x11\x15a;\rWa;\raC\xE5V[\x14\x15a;\x16WPV[`\x01\x81`\x04\x81\x11\x15a;*Wa;*aC\xE5V[\x14\x15a;xW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x08|V[`\x02\x81`\x04\x81\x11\x15a;\x8CWa;\x8CaC\xE5V[\x14\x15a;\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x08|V[`\x03\x81`\x04\x81\x11\x15a;\xEEWa;\xEEaC\xE5V[\x14\x15a<GW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x08|V[`\x04\x81`\x04\x81\x11\x15a<[Wa<[aC\xE5V[\x14\x15a\t\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 'v' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x08|V[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a<\xEBWP`\0\x90P`\x03a=\x98V[\x84`\xFF\x16`\x1B\x14\x15\x80\x15a=\x03WP\x84`\xFF\x16`\x1C\x14\x15[\x15a=\x14WP`\0\x90P`\x04a=\x98V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a=hW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a=\x91W`\0`\x01\x92P\x92PPa=\x98V[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x80`\x01`\x01`\xFF\x1B\x03\x83\x16\x81a=\xBE`\xFF\x86\x90\x1C`\x1BaI\xE4V[\x90Pa=\xCC\x87\x82\x88\x85a<\xB4V[\x93P\x93PPP\x93P\x93\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a=\xEDW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0` \x82\x84\x03\x12\x15a>\x0EW`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a>MWa>Ma>\x15V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a>{Wa>{a>\x15V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x83\x11\x15a>\x9CWa>\x9Ca>\x15V[a>\xAF`\x1F\x84\x01`\x1F\x19\x16` \x01a>SV[\x90P\x82\x81R\x83\x83\x83\x01\x11\x15a>\xC3W`\0\x80\xFD[\x82\x82` \x83\x017`\0` \x84\x83\x01\x01R\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a>\xEBW`\0\x80\xFD[a>\xFA\x83\x835` \x85\x01a>\x83V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a?\x13W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a?)W`\0\x80\xFD[a  \x84\x82\x85\x01a>\xDAV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\t\x81W`\0\x80\xFD[\x805a?U\x81a?5V[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a?lW`\0\x80\xFD[\x815a>\xFA\x81a?5V[`\0\x80\x83`\x1F\x84\x01\x12a?\x89W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a?\xA0W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a:\xF2W`\0\x80\xFD[`\0`@\x82\x84\x03\x12\x15a?\xCAW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a?\xECWa?\xECa>\x15V[`@R\x825\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0``\x82\x84\x03\x12\x15a@\x18W`\0\x80\xFD[a@ a>+V[\x90P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a@8W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a@IW`\0\x80\xFD[a@X\x84\x825` \x84\x01a>\x83V[\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\0\x80`\xC0\x89\x8B\x03\x12\x15a@\x91W`\0\x80\xFD[\x885`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a@\xA8W`\0\x80\xFD[a@\xB4\x8C\x83\x8D\x01a?wV[\x90\x9AP\x98P\x88\x91Pa@\xC9\x8C` \x8D\x01a?\xB8V[\x97P``\x8B\x015\x91P\x80\x82\x11\x15a@\xDFW`\0\x80\xFD[a@\xEB\x8C\x83\x8D\x01a?wV[\x90\x97P\x95P`\x80\x8B\x015\x91P\x80\x82\x11\x15aA\x04W`\0\x80\xFD[\x81\x8B\x01\x91P\x8B`\x1F\x83\x01\x12aA\x18W`\0\x80\xFD[\x815\x81\x81\x11\x15aA'W`\0\x80\xFD[\x8C` \x82`\x07\x1B\x85\x01\x01\x11\x15aA<W`\0\x80\xFD[` \x83\x01\x95P\x80\x94PP`\xA0\x8B\x015\x91P\x80\x82\x11\x15aAZW`\0\x80\xFD[PaAg\x8B\x82\x8C\x01a@\x06V[\x91PP\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\t\x81W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15aA\x9EW`\0\x80\xFD[\x835\x92P` \x84\x015aA\xB0\x81aAwV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15aA\xD6W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x81\x11\x15aA\xECW`\0\x80\xFD[aA\xF8\x86\x82\x87\x01a?wV[\x90\x94P\x92PaB\x0C\x90P\x85` \x86\x01a?\xB8V[\x90P\x92P\x92P\x92V[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15aB.WaB.a>\x15V[P`\x05\x1B` \x01\x90V[a\xFF\xFF\x81\x16\x81\x14a\t\x81W`\0\x80\xFD[`\0``\x82\x84\x03\x12\x15aBZW`\0\x80\xFD[aBba>+V[\x90P\x815aBo\x81aAwV[\x81R` \x82\x015aB\x7F\x81aB8V[` \x82\x01R`@\x82\x015aB\x92\x81aB8V[`@\x82\x01R\x92\x91PPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15aB\xB5W`\0\x80\xFD[\x855aB\xC0\x81a?5V[\x94P` \x86\x81\x015aB\xD1\x81a?5V[\x94P`@\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aB\xECW`\0\x80\xFD[\x87\x01`\x1F\x81\x01\x89\x13aB\xFDW`\0\x80\xFD[\x805aC\x10aC\x0B\x82aB\x15V[a>SV[\x81\x81R``\x91\x82\x02\x83\x01\x84\x01\x91\x84\x82\x01\x91\x90\x8C\x84\x11\x15aC/W`\0\x80\xFD[\x93\x85\x01\x93[\x83\x85\x10\x15aCUWaCF\x8D\x86aBHV[\x83R\x93\x84\x01\x93\x91\x85\x01\x91aC4V[\x81\x98PaCc\x81\x8D\x01a?JV[\x9A\x9D\x99\x9CP\x97\x9A`\x80\x015\x98\x97PPPPPPPPV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15aC\x90W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aC\xA7W`\0\x80\xFD[aC\xB3\x88\x83\x89\x01a?wV[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15aC\xCCW`\0\x80\xFD[PaC\xD9\x87\x82\x88\x01a?wV[\x95\x98\x94\x97P\x95PPPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x03\x81\x10aD\x19WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[\x81Q\x81R` \x80\x83\x01Q`@\x83\x01\x91aD8\x90\x84\x01\x82aC\xFBV[P\x92\x91PPV[\x805`\xFF\x81\x16\x81\x14a?UW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aDbW`\0\x80\xFD[a>\xFA\x82aD?V[`\0\x80`\x80\x83\x85\x03\x12\x15aD~W`\0\x80\xFD[aD\x87\x83aD?V[\x91PaD\x96\x84` \x85\x01aBHV[\x90P\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15aD\xB2W`\0\x80\xFD[\x825aD\xBD\x81aAwV[\x91P` \x83\x81\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aD\xD9W`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13aD\xEAW`\0\x80\xFD[\x805aD\xF8aC\x0B\x82aB\x15V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x88\x83\x11\x15aE\x17W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aE5W\x835\x82R\x92\x84\x01\x92\x90\x84\x01\x90aE\x1CV[\x80\x95PPPPPP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aE\x82W\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aE`V[P\x90\x96\x95PPPPPPV[`\0`\x80\x82\x84\x03\x12\x15aE\xA0W`\0\x80\xFD[aE\xA8a>+V[\x90PaE\xB3\x82aD?V[\x81R` \x82\x015aE\xC3\x81a?5V[` \x82\x01RaB\x92\x83`@\x84\x01a?\xB8V[`\0\x80`\0\x80`\x80\x80\x86\x88\x03\x12\x15aE\xECW`\0\x80\xFD[\x855\x94P` \x80\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aF\nW`\0\x80\xFD[\x87\x01`\x1F\x81\x01\x89\x13aF\x1BW`\0\x80\xFD[\x805aF)aC\x0B\x82aB\x15V[\x81\x81R`\x07\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x8B\x83\x11\x15aFHW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aFnWaF_\x8C\x85aE\x8EV[\x82R\x92\x85\x01\x92\x90\x84\x01\x90aFMV[\x98\x9B\x98\x9APPPP`@\x87\x015\x96``\x015\x95\x94PPPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aF\x9FW`\0\x80\xFD[\x845aF\xAA\x81a?5V[\x93P` \x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aF\xC5W`\0\x80\xFD[aF\xD1\x87\x82\x88\x01a?wV[\x90\x94P\x92PaF\xE5\x90P\x86`@\x87\x01a?\xB8V[\x90P\x92\x95\x91\x94P\x92PV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15aG\x08W`\0\x80\xFD[\x855`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aG\x1FW`\0\x80\xFD[aG+\x89\x83\x8A\x01a?wV[\x90\x97P\x95P\x85\x91PaG@\x89` \x8A\x01a?\xB8V[\x94P``\x88\x015\x91P\x80\x82\x11\x15aGVW`\0\x80\xFD[PaGc\x88\x82\x89\x01a?wV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[` \x81\x01a\x07\xCA\x82\x84aC\xFBV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0[\x83\x81\x10\x15aG\xB3W\x81\x81\x01Q\x83\x82\x01R` \x01aG\x9BV[\x83\x81\x11\x15a\x16\xDCWPP`\0\x91\x01RV[`\0\x81Q\x80\x84RaG\xDC\x81` \x86\x01` \x86\x01aG\x98V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a>\xFA` \x83\x01\x84aG\xC4V[`\0` \x82\x84\x03\x12\x15aH\x15W`\0\x80\xFD[\x81Qa>\xFA\x81a?5V[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15aH|W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a>\xFAW`\0\x80\xFD[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`\x19\x90\x82\x01R\x7FPausable: index is paused\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`\0`\x80\x82\x84\x03\x12\x15aI\x1DW`\0\x80\xFD[a>\xFA\x83\x83aE\x8EV[`\0` \x82\x84\x03\x12\x15aI9W`\0\x80\xFD[\x81Q`\x01`\x01``\x1B\x03\x81\x16\x81\x14a>\xFAW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aI\x8CWaI\x8CaIPV[\x02\x94\x93PPPPV[`\0`\x01`\x01``\x1B\x03\x80\x84\x16\x80aI\xBDWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[\x92\x16\x91\x90\x91\x04\x92\x91PPV[`\0`\0\x19\x82\x14\x15aI\xDDWaI\xDDaIPV[P`\x01\x01\x90V[`\0\x82\x19\x82\x11\x15aI\xF7WaI\xF7aIPV[P\x01\x90V[`\0\x80\x85\x85\x11\x15aJ\x0CW`\0\x80\xFD[\x83\x86\x11\x15aJ\x19W`\0\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[`\0`@\x82\x84\x03\x12\x15aJ8W`\0\x80\xFD[a>\xFA\x83\x83a?\xB8V[` \x80\x82R`b\x90\x82\x01R`\0\x80Q` aNp\x839\x81Q\x91R`@\x82\x01R\x7Fs.onlyServiceManagerOwner: calle``\x82\x01R\x7Fr is not the service manager own`\x80\x82\x01Ra2\xB9`\xF1\x1B`\xA0\x82\x01R`\xC0\x01\x90V[`\0\x82\x82\x10\x15aJ\xD0WaJ\xD0aIPV[P\x03\x90V[`\0` \x82\x84\x03\x12\x15aJ\xE7W`\0\x80\xFD[\x81Qa>\xFA\x81aB8V[`\0`\xFF\x82\x16`\xFF\x81\x14\x15aK\tWaK\taIPV[`\x01\x01\x92\x91PPV[`\0\x80``\x83\x85\x03\x12\x15aK%W`\0\x80\xFD[aK/\x84\x84a?\xB8V[\x91P`@\x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aKJW`\0\x80\xFD[aKV\x85\x82\x86\x01a>\xDAV[\x91PP\x92P\x92\x90PV[`\0c\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15aK}WaK}aIPV[\x03\x93\x92PPPV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x14\x15aK\x9FWaK\x9FaIPV[`\x01\x01\x93\x92PPPV[`\0`\xA0\x82\x01\x87\x83R` \x87\x81\x85\x01R`@`\xA0\x81\x86\x01R\x82\x88Q\x80\x85R`\xC0\x87\x01\x91P\x83\x8A\x01\x94P`\0[\x81\x81\x10\x15aL$W\x85Q\x80Q`\xFF\x16\x84R\x85\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x86\x85\x01R\x84\x01QaL\x10\x85\x85\x01\x82\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[P\x94\x84\x01\x94`\x80\x92\x90\x92\x01\x91`\x01\x01aK\xD5V[PP``\x86\x01\x97\x90\x97RPPPP`\x80\x01R\x93\x92PPPV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x85\x16\x81R`\x80` \x82\x01\x81\x90R`\0\x90aL\x8B\x90\x83\x01\x85\x87aL=V[\x83Q`@\x84\x01R` \x84\x01Q``\x84\x01R\x90Pa*\x0CV[`\0` \x82\x84\x03\x12\x15aL\xB5W`\0\x80\xFD[PQ\x91\x90PV[`\x01\x80`\xA0\x1B\x03\x85\x16\x81R\x83` \x82\x01R```@\x82\x01R`\0aL\xE4``\x83\x01\x84\x86aL=V[\x96\x95PPPPPPV[\x83\x81R`@` \x82\x01R`\0a \x1D`@\x83\x01\x84\x86aL=V[`\0` \x80\x83\x85\x03\x12\x15aM\x1BW`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aM1W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aMBW`\0\x80\xFD[\x80QaMPaC\x0B\x82aB\x15V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15aMoW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aM\x96W\x83QaM\x87\x81aAwV[\x82R\x92\x84\x01\x92\x90\x84\x01\x90aMtV[\x97\x96PPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R`\x80` \x82\x01\x81\x90R`\0\x90aM\xC5\x90\x83\x01\x85aG\xC4V[\x83Q`@\x84\x01R` \x84\x01Q``\x84\x01R\x90Pa  V[\x82\x81R`@` \x82\x01R`\0a  `@\x83\x01\x84aG\xC4V[`\0` \x82\x84\x03\x12\x15aN\x08W`\0\x80\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a>\xFAW`\0\x80\xFD[`\0\x83QaN2\x81\x84` \x88\x01aG\x98V[`\x01`\x01`\xF8\x1B\x03\x19\x93\x90\x93\x16\x91\x90\x92\x01\x90\x81R`\x01\x01\x92\x91PPV\xFEs._registerOperatorWithCoordinatBLSRegistryCoordinatorWithIndice\xA2dipfsX\"\x12 \xB1\xCC\0\x185f\x8B\xBF\xC8\xFAO\xBC\x88[\xF4\x1Ap\xE1\x1E\x83Tt\xFCqM\x7FW\xE3\xD2\x84\xF2\xBBdsolcC\0\x08\x0C\x003";
    /// The bytecode of the contract.
    pub static BLSREGISTRYCOORDINATORWITHINDICES_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02HW`\x005`\xE0\x1C\x80cXe\xC6\x0C\x11a\x01;W\x80c\xAB&\x9B-\x11a\0\xB8W\x80c\xCA\r\xE8\x82\x11a\0|W\x80c\xCA\r\xE8\x82\x14a\x06\x1EW\x80c\xD7-\x8D\xD6\x14a\x06EW\x80c\xE6W\x97\xAD\x14a\x06MW\x80c\xFA\xBC\x1C\xBC\x14a\x06\xF0W\x80c\xFD9\x10Z\x14a\x07\x03W`\0\x80\xFD[\x80c\xAB&\x9B-\x14a\x05\xABW\x80c\xB14Bq\x14a\x05\xBEW\x80c\xB84z\xCC\x14a\x05\xE5W\x80c\xC6j\xB9\xCA\x14a\x05\xF8W\x80c\xC8\x1B\x1F\xF4\x14a\x06\x0BW`\0\x80\xFD[\x80ccG\xC9\0\x11a\0\xFFW\x80ccG\xC9\0\x14a\x05\x11W\x80ch0H5\x14a\x05$W\x80c\x85\x02\rI\x14a\x05KW\x80c\x88o\x11\x95\x14a\x05kW\x80c\x9E\x99#\xC2\x14a\x05\x84W`\0\x80\xFD[\x80cXe\xC6\x0C\x14a\x04\xAFW\x80cY\\jg\x14a\x04\xCFW\x80cZ\xC8j\xB7\x14a\x04\xD7W\x80c[\x0B\x82\x9F\x14a\x04\xF6W\x80c\\\x97Z\xBB\x14a\x05\tW`\0\x80\xFD[\x80c)k\xB0d\x11a\x01\xC9W\x80c5a\xDE\xB1\x11a\x01\x8DW\x80c5a\xDE\xB1\x14a\x04(W\x80c9\x98\xFD\xD3\x14a\x04OW\x80cEZC\xFC\x14a\x04vW\x80cKv\xB9\xD5\x14a\x04\x89W\x80cRn\xA9N\x14a\x04\x9CW`\0\x80\xFD[\x80c)k\xB0d\x14a\x03\xB1W\x80c)\xD1\xE0\xC3\x14a\x03\xC4W\x80c,\xDD\x1E\x86\x14a\x03\xD7W\x80c0db\r\x14a\x03\xEAW\x80c41\xAF%\x14a\x04\x15W`\0\x80\xFD[\x80c\x13T*N\x11a\x02\x10W\x80c\x13T*N\x14a\x03\x1CW\x80c\x13d9\xDD\x14a\x03EW\x80c\x14x\x85\x1F\x14a\x03XW\x80c%\0uv\x14a\x03\x8BW\x80c(\xF6\x1B1\x14a\x03\x9EW`\0\x80\xFD[\x80c\x01Y\xF1\xCE\x14a\x02MW\x80c\x05C\x10\xE6\x14a\x02\x9BW\x80c\x05Zb\xB6\x14a\x02\xC6W\x80c\x0C\xF4\xB7g\x14a\x02\xF4W\x80c\x10\xD6z/\x14a\x03\tW[`\0\x80\xFD[a\x02`a\x02[6`\x04a=\xDAV[a\x07?V[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x80\x85\x01Q\x90\x91\x16\x90\x82\x01R\x91\x81\x01Q`\x01`\x01`\xC0\x1B\x03\x16\x90\x82\x01R``\x01[`@Q\x80\x91\x03\x90\xF3[`7Ta\x02\xAE\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x92V[a\x02\xE6a\x02\xD46`\x04a=\xFCV[`\0\x90\x81R`3` R`@\x90 T\x90V[`@Q\x90\x81R` \x01a\x02\x92V[a\x03\x07a\x03\x026`\x04a?\x01V[a\x07\xD0V[\0[a\x03\x07a\x03\x176`\x04a?ZV[a\x08\xD1V[a\x02\xE6a\x03*6`\x04a?ZV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`4` R`@\x90 T\x90V[a\x03\x07a\x03S6`\x04a=\xFCV[a\t\x84V[a\x03{a\x03f6`\x04a=\xFCV[`5` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02\x92V[a\x03\x07a\x03\x996`\x04a@uV[a\n\xC1V[`8Ta\x02\xAE\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02\xAEa\x03\xBF6`\x04a=\xFCV[a\x11\xF0V[a\x03\x07a\x03\xD26`\x04a?ZV[a\x12|V[a\x03\x07a\x03\xE56`\x04a?ZV[a\x137V[a\x03\xFDa\x03\xF86`\x04aA\x89V[a\x13\xF2V[`@Q`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x92V[a\x03\xFDa\x04#6`\x04a=\xFCV[a\x15\xF3V[a\x02\xAE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xAE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\x07a\x04\x846`\x04aA\xC1V[a\x16\xA9V[a\x03\x07a\x04\x976`\x04aB\x9DV[a\x16\xE2V[a\x03\x07a\x04\xAA6`\x04aCzV[a\x1A\x0EV[a\x04\xC2a\x04\xBD6`\x04a?ZV[a\x1A_V[`@Qa\x02\x92\x91\x90aD\x1DV[a\x03\x07a\x1A\xD3V[a\x03{a\x04\xE56`\x04aDPV[`\x01\x80T`\xFF\x90\x92\x16\x1B\x90\x81\x16\x14\x90V[a\x03\x07a\x05\x046`\x04aDkV[a\x1B\x9FV[`\x01Ta\x02\xE6V[a\x02\xAEa\x05\x1F6`\x04a=\xFCV[a\x1C_V[a\x02\xAE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x05^a\x05Y6`\x04aD\x9FV[a\x1C\x89V[`@Qa\x02\x92\x91\x90aEDV[`\0Ta\x02\xAE\x90b\x01\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02\xAE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xE6a\x05\xB96`\x04aE\xD5V[a\x1F\xC4V[a\x02\xAE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\x07a\x05\xF36`\x04aF\x89V[a (V[a\x03\x07a\x06\x066`\x04aF\xF0V[a \xB3V[a\x03\x07a\x06\x196`\x04aCzV[a!\x1EV[a\x02\xE6\x7F\xAEc\xAA\x13\x18u\x1Fd\xEF\x85\x0E\x04T\x17\xF8h\x11\xEA\xEC\xEE\x1C\xEB\xC2&\x90M#{\x80pM\x92\x81V[`6Ta\x02\xE6V[a\x06\xBCa\x06[6`\x04aDPV[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x92\x84\x01\x81\x90R`\xFF\x94\x90\x94\x16\x84R`2\x82R\x92\x82\x90 \x82Q\x93\x84\x01\x83RTc\xFF\xFF\xFF\xFF\x81\x16\x84Ra\xFF\xFF`\x01` \x1B\x82\x04\x81\x16\x92\x85\x01\x92\x90\x92R`\x01`0\x1B\x90\x04\x16\x90\x82\x01R\x90V[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x16\x81R` \x80\x84\x01Qa\xFF\xFF\x90\x81\x16\x91\x83\x01\x91\x90\x91R\x92\x82\x01Q\x90\x92\x16\x90\x82\x01R``\x01a\x02\x92V[a\x03\x07a\x06\xFE6`\x04a=\xFCV[a!aV[a\x072a\x07\x116`\x04a?ZV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`4` R`@\x90 `\x01\x01T`\xFF\x16\x90V[`@Qa\x02\x92\x91\x90aGtV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R`\0\x83\x81R`3` R`@\x90 \x80T\x83\x90\x81\x10a\x07|Wa\x07|aG\x82V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01`\xC0\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90P[\x92\x91PPV[`\x013`\0\x90\x81R`4` R`@\x90 `\x01\x01T`\xFF\x16`\x02\x81\x11\x15a\x07\xF9Wa\x07\xF9aC\xE5V[\x14a\x08\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`K`$\x82\x01R\x7FBLSRegistryCoordinatorWithIndici`D\x82\x01R\x7Fes.updateSocket: operator is not`d\x82\x01Rj\x08\x1C\x99Y\xDA\\\xDD\x19\\\x99Y`\xAA\x1B`\x84\x82\x01R`\xA4\x01[`@Q\x80\x91\x03\x90\xFD[3`\0\x90\x81R`4` R`@\x90\x81\x90 T\x90Q\x7F\xEC)c\xAB!\xC1\xE5\x0E\x1EX*\xA5B\xAF.K\xF7\xBF8\xE6\xE1@<'\xB4.\x1C]nb\x1E\xAA\x90a\x08\xC6\x90\x84\x90aG\xF0V[`@Q\x80\x91\x03\x90\xA2PV[`\0`\x02\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t$W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tH\x91\x90aH\x03V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\txW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08|\x90aH V[a\t\x81\x81a\"\xBDV[PV[`\0T`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01Rb\x01\0\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xD1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xF5\x91\x90aHjV[a\n\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08|\x90aH\x8CV[`\x01T\x81\x81\x16\x14a\n\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08|V[`\x01\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01a\x08\xC6V[`\x01\x80T`\0\x91\x90\x81\x16\x14\x15a\n\xE9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08|\x90aH\xD4V[`\0a\x0B.3\x8B\x8B\x8B\x8B\x8B\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa#\xC2\x92PPPV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x91\x92P`\0\x91\x90` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90Pa\x0B]\x89a*\x15V[\x81`\0\x81Q\x81\x10a\x0BpWa\x0BpaG\x82V[` \x02` \x01\x01\x81\x81RPPa\x0B\xF3\x81`\0\x81Q\x81\x10a\x0B\x92Wa\x0B\x92aG\x82V[` \x02` \x01\x01Q\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x0B\xE8Wa\x0B\xD9`\x80\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aI\x0BV[\x81R` \x01\x90`\x01\x01\x90a\x0B\xBCV[PPPPP\x86a*XV[`\0\x80[\x8B\x81\x10\x15a\x11\xE1W`\0\x8D\x8D\x83\x81\x81\x10a\x0C\x13Wa\x0C\x13aG\x82V[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`2` \x90\x81R`@\x91\x82\x90 \x82Q``\x81\x01\x84R\x90Tc\xFF\xFF\xFF\xFF\x81\x16\x80\x83Ra\xFF\xFF`\x01` \x1B\x83\x04\x81\x16\x94\x84\x01\x94\x90\x94R`\x01`0\x1B\x90\x91\x04\x90\x92\x16\x92\x81\x01\x92\x90\x92R\x88Q\x92\x94P\x90\x92P\x90\x87\x90\x85\x90\x81\x10a\x0C\x81Wa\x0C\x81aG\x82V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11a\x0C\x9BWPPa\x11\xCFV[\x81`\xFF\x16\x8A\x8A\x86\x81\x81\x10a\x0C\xB1Wa\x0C\xB1aG\x82V[a\x0C\xC7\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91PaDPV[`\xFF\x16\x14a\r`W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`f`$\x82\x01R`\0\x80Q` aNp\x839\x81Q\x91R`D\x82\x01R\x7Fs.registerOperatorWithCoordinato`d\x82\x01R\x7Fr: quorumNumber not the same as `\x84\x82\x01Re\x1C\xDAY\xDB\x99Y`\xD2\x1B`\xA4\x82\x01R`\xC4\x01a\x08|V[`@Qc\x0C\x8Fs\x9D`\xE4\x1B\x81R`\xFF\x83\x16`\x04\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xC8\xF79\xD0\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xCAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xEE\x91\x90aI'V[\x90P`\0`4`\0\x8D\x8D\x88\x81\x81\x10a\x0E\x08Wa\x0E\x08aG\x82V[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\x0E \x91\x90a?ZV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x82\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 T\x91Qc\x1A\xADN5`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`\xFF\x88\x16`$\x82\x01R\x91\x93P\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cj\xB58\xD4\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xA9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xCD\x91\x90aI'V[\x90P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cj\xB58\xD4\x8A`\0\x81Q\x81\x10a\x0F\x13Wa\x0F\x13aG\x82V[` \x02` \x01\x01Q\x88`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0FE\x92\x91\x90\x91\x82R`\xFF\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0FbW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x86\x91\x90aI'V[` \x86\x01Q\x90\x91Pa'\x10\x90a\x0F\xA0\x90a\xFF\xFF\x16\x84aIfV[a\x0F\xAA\x91\x90aI\x95V[`\x01`\x01``\x1B\x03\x16\x81`\x01`\x01``\x1B\x03\x16\x11a\x10jW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`}`$\x82\x01R`\0\x80Q` aNp\x839\x81Q\x91R`D\x82\x01R\x7Fs.registerOperatorWithCoordinato`d\x82\x01R\x7Fr: registering operator has less`\x84\x82\x01R\x7F than kickBIPsOfOperatorStake\0\0\0`\xA4\x82\x01R`\xC4\x01a\x08|V[`@\x85\x01Qa'\x10\x90a\x10\x81\x90a\xFF\xFF\x16\x86aIfV[a\x10\x8B\x91\x90aI\x95V[`\x01`\x01``\x1B\x03\x16\x82`\x01`\x01``\x1B\x03\x16\x10a\x11DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`v`$\x82\x01R`\0\x80Q` aNp\x839\x81Q\x91R`D\x82\x01R\x7Fs.registerOperatorWithCoordinato`d\x82\x01R\x7Fr: operator to kick has more tha`\x84\x82\x01Run kickBIPSOfTotalStake`P\x1B`\xA4\x82\x01R`\xC4\x01a\x08|V[\x87a\x11N\x81aI\xC9V[\x98PPPPPPPPa\x11\xCF\x88\x88\x83\x81\x81\x10a\x11lWa\x11laG\x82V[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\x11\x84\x91\x90a?ZV[\x8E\x83\x8Fa\x11\x92\x82`\x01aI\xE4V[\x92a\x11\x9F\x93\x92\x91\x90aI\xFCV[\x8B\x8B\x86\x81\x81\x10a\x11\xB1Wa\x11\xB1aG\x82V[\x90P`\x80\x02\x01`@\x01\x806\x03\x81\x01\x90a\x11\xCA\x91\x90aJ&V[a,%V[\x80a\x11\xD9\x81aI\xC9V[\x91PPa\x0B\xF7V[PPPPPPPPPPPPPV[`@Qc\x08\xF6b\x9D`\xE3\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cG\xB3\x14\xE8\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12XW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xCA\x91\x90aH\x03V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xDAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xFE\x91\x90aH\x03V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x13.W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08|\x90aJBV[a\t\x81\x81a1\xD2V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\x95W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xB9\x91\x90aH\x03V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x13\xE9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08|\x90aJBV[a\t\x81\x81a2;V[`\0\x83\x81R`3` R`@\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x14\x15Wa\x14\x15aG\x82V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x85R`\x01` \x1B\x83\x04\x82\x16\x95\x85\x01\x95\x90\x95R`\x01`@\x1B\x90\x91\x04`\x01`\x01`\xC0\x1B\x03\x16\x91\x83\x01\x91\x90\x91R\x90\x92P\x85\x16\x10\x15a\x15\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x7F`$\x82\x01R`\0\x80Q` aNp\x839\x81Q\x91R`D\x82\x01R\x7Fs.getQuorumBitmapByOperatorIdAtB`d\x82\x01R\x7FlockNumberByIndex: quorumBitmapU`\x84\x82\x01R\x7Fpdate is from after blockNumber\0`\xA4\x82\x01R`\xC4\x01a\x08|V[\x83c\xFF\xFF\xFF\xFF\x16\x81` \x01Qc\xFF\xFF\xFF\xFF\x16\x11\x80a\x15;WP` \x81\x01Qc\xFF\xFF\xFF\xFF\x16\x15[a\x15\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x80`$\x82\x01R`\0\x80Q` aNp\x839\x81Q\x91R`D\x82\x01R\x7Fs.getQuorumBitmapByOperatorIdAtB`d\x82\x01R\x7FlockNumberByIndex: quorumBitmapU`\x84\x82\x01R\x7Fpdate is from before blockNumber`\xA4\x82\x01R`\xC4\x01a\x08|V[`@\x01Q\x94\x93PPPPV[`\0\x81\x81R`3` R`@\x81 T\x80\x15\x80a\x16OWP`\0\x83\x81R`3` R`@\x90 a\x16#`\x01\x83aJ\xBEV[\x81T\x81\x10a\x163Wa\x163aG\x82V[`\0\x91\x82R` \x90\x91 \x01T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15\x15[\x15a\x16]WP`\0\x92\x91PPV[`\0\x83\x81R`3` R`@\x90 a\x16v`\x01\x83aJ\xBEV[\x81T\x81\x10a\x16\x86Wa\x16\x86aG\x82V[`\0\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01`\xC0\x1B\x03\x16\x93\x92PPPV[`\x01\x80T`\x02\x90\x81\x16\x14\x15a\x16\xD0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08|\x90aH\xD4V[a\x16\xDC3\x85\x85\x85a,%V[PPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x17\x02WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x17\x1CWP0;\x15\x80\x15a\x17\x1CWP`\0T`\xFF\x16`\x01\x14[a\x17\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x08|V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x17\xA2W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x17\xAC\x83\x83a2\xA4V[a\x17\xB5\x86a1\xD2V[a\x17\xBE\x85a2;V[`6\x80T`\x01\x80\x82\x01\x83U`\0\x83\x90R\x7FJ\x11\xF9N \xA9<y\xF6\xECt:\x19T\xECO\xC2\xC0\x84)\xAE!\"\x11\x8B\xF24\xB2\x18\\\x81\xB8\x91\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x82\x17\x90\x93U\x85T\x80\x85\x01\x87U\x85\x01\x80T\x83\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x16\x17\x90U\x85T\x93\x84\x01\x90\x95U\x91\x90\x92\x01\x80T\x90\x91\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x16\x91\x90\x91\x17\x90U\x84Q`@\x80Qc\x9A\xA1e=`\xE0\x1B\x81R\x90Q\x91\x92\x91c\x9A\xA1e=\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x18\xE0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\x04\x91\x90aJ\xD5V[a\xFF\xFF\x16\x14a\x19xW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`F`$\x82\x01R`\0\x80Q` aNp\x839\x81Q\x91R`D\x82\x01R\x7Fs: operator set params length mi`d\x82\x01Re\x0Em\xAC.\x8Cm`\xD3\x1B`\x84\x82\x01R`\xA4\x01a\x08|V[`\0[\x84Q\x81`\xFF\x16\x10\x15a\x19\xBFWa\x19\xAD\x81\x86\x83`\xFF\x16\x81Q\x81\x10a\x19\xA0Wa\x19\xA0aG\x82V[` \x02` \x01\x01Qa3\x90V[\x80a\x19\xB7\x81aJ\xF2V[\x91PPa\x19{V[P\x80\x15a\x1A\x06W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPV[`\x01\x80T`\0\x91\x90\x81\x16\x14\x15a\x1A6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08|\x90aH\xD4V[`\0\x80a\x1AE\x84\x86\x01\x86aK\x12V[\x91P\x91Pa\x1AV3\x88\x88\x85\x85a4=V[PPPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`4` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x81\x01T\x90\x91\x83\x01\x90`\xFF\x16`\x02\x81\x11\x15a\x1A\xB9Wa\x1A\xB9aC\xE5V[`\x02\x81\x11\x15a\x1A\xCAWa\x1A\xCAaC\xE5V[\x90RP\x92\x91PPV[`\0T`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01Rb\x01\0\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1BD\x91\x90aHjV[a\x1B`W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08|\x90aH\x8CV[`\0\x19`\x01\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xFDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C!\x91\x90aH\x03V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1CQW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08|\x90aJBV[a\x1C[\x82\x82a3\x90V[PPV[`6\x81\x81T\x81\x10a\x1CoW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x81V[```\0\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1C\xA6Wa\x1C\xA6a>\x15V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1C\xCFW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\x1F\xBCW`\0`3`\0\x86\x84\x81Q\x81\x10a\x1C\xF6Wa\x1C\xF6aG\x82V[` \x02` \x01\x01Q\x81R` \x01\x90\x81R` \x01`\0 \x80T\x90P\x90P`\0[\x81c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x1F\xA7W\x86c\xFF\xFF\xFF\xFF\x16`3`\0\x88\x86\x81Q\x81\x10a\x1DFWa\x1DFaG\x82V[` \x02` \x01\x01Q\x81R` \x01\x90\x81R` \x01`\0 `\x01\x83\x85a\x1Dj\x91\x90aK`V[a\x1Dt\x91\x90aK`V[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x1D\x8AWa\x1D\x8AaG\x82V[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a\x1F\x95W`3`\0\x87\x85\x81Q\x81\x10a\x1D\xB7Wa\x1D\xB7aG\x82V[` \x02` \x01\x01Q\x81R` \x01\x90\x81R` \x01`\0 `\x01\x82\x84a\x1D\xDB\x91\x90aK`V[a\x1D\xE5\x91\x90aK`V[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x1D\xFBWa\x1D\xFBaG\x82V[`\0\x91\x82R` \x90\x91 \x01T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15\x80a\x1E\x97WP\x86c\xFF\xFF\xFF\xFF\x16`3`\0\x88\x86\x81Q\x81\x10a\x1E8Wa\x1E8aG\x82V[` \x02` \x01\x01Q\x81R` \x01\x90\x81R` \x01`\0 `\x01\x83\x85a\x1E\\\x91\x90aK`V[a\x1Ef\x91\x90aK`V[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x1E|Wa\x1E|aG\x82V[`\0\x91\x82R` \x90\x91 \x01T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x11[a\x1FNW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x82`$\x82\x01R`\0\x80Q` aNp\x839\x81Q\x91R`D\x82\x01R\x7Fs.getQuorumBitmapIndicesByOperat`d\x82\x01R\x7ForIdsAtBlockNumber: operatorId h`\x84\x82\x01R\x7Fas no quorumBitmaps at blockNumb`\xA4\x82\x01Ra2\xB9`\xF1\x1B`\xC4\x82\x01R`\xE4\x01a\x08|V[`\x01a\x1FZ\x82\x84aK`V[a\x1Fd\x91\x90aK`V[\x84\x84\x81Q\x81\x10a\x1FvWa\x1FvaG\x82V[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPPa\x1F\xA7V[\x80a\x1F\x9F\x81aK\x85V[\x91PPa\x1D\x15V[PP\x80\x80a\x1F\xB4\x90aI\xC9V[\x91PPa\x1C\xD5V[P\x93\x92PPPV[`\0a \x1D\x7F\xAEc\xAA\x13\x18u\x1Fd\xEF\x85\x0E\x04T\x17\xF8h\x11\xEA\xEC\xEE\x1C\xEB\xC2&\x90M#{\x80pM\x92\x86\x86\x86\x86`@Q` \x01a \x02\x95\x94\x93\x92\x91\x90aK\xA9V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a5TV[\x90P[\x94\x93PPPPV[`8T`\x01`\x01`\xA0\x1B\x03\x163\x14a \xA7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`H`$\x82\x01R`\0\x80Q` aNp\x839\x81Q\x91R`D\x82\x01R\x7Fs.onlyEjector: caller is not the`d\x82\x01Rg\x102\xB52\xB1\xBA7\xB9`\xC1\x1B`\x84\x82\x01R`\xA4\x01a\x08|V[a\x16\xDC\x84\x84\x84\x84a,%V[`\x01\x80T`\0\x91\x90\x81\x16\x14\x15a \xDBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08|\x90aH\xD4V[a\x1A\x063\x87\x87\x87\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa4=\x92PPPV[`\x01\x80T`\x02\x90\x81\x16\x14\x15a!EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08|\x90aH\xD4V[`\0a!S\x83\x85\x01\x85aJ&V[\x90Pa\x1A\x063\x87\x87\x84a,%V[`\0`\x02\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!\xB4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\xD8\x91\x90aH\x03V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\"\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08|\x90aH V[`\x01T\x19\x81\x19`\x01T\x19\x16\x14a\"\x86W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08|V[`\x01\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x08\xC6V[`\x01`\x01`\xA0\x1B\x03\x81\x16a#KW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x08|V[`\0T`@\x80Q`\x01`\x01`\xA0\x1B\x03b\x01\0\0\x90\x93\x04\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`\0\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16b\x01\0\0\x02b\x01\0\0`\x01`\xB0\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[```\0a$\x05\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa5\xA2\x92PPPV[\x90P`\x01`\x01`\xC0\x1B\x03\x81\x11\x15a$\x9AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`k`$\x82\x01R`\0\x80Q` aNp\x839\x81Q\x91R`D\x82\x01R`\0\x80Q` aNP\x839\x81Q\x91R`d\x82\x01R\x7For: quorumBitmap exceeds of max `\x84\x82\x01Rjbitmap size`\xA8\x1B`\xA4\x82\x01R`\xC4\x01a\x08|V[\x80a%\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R`\0\x80Q` aNp\x839\x81Q\x91R`D\x82\x01R`\0\x80Q` aNP\x839\x81Q\x91R`d\x82\x01R\x7For: quorumBitmap cannot be 0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x08|V[`@Qc\x03\xCEK\xAD`\xE0\x1B\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x03\xCEK\xAD\x90a%d\x90\x8B\x90\x8B\x90\x8B\x90\x8B\x90`\x04\x01aLfV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a%\x83W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%\xA7\x91\x90aL\xA3V[`@Qc%PGw`\xE0\x1B\x81R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c%PGw\x90a%\xFC\x90\x8B\x90\x85\x90\x8C\x90\x8C\x90`\x04\x01aL\xBCV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a&\x16W`\0\x80\xFD[PZ\xF1\x15\x80\x15a&*W=`\0\x80>=`\0\xFD[PP`@Qb\xBF\xF0M`\xE0\x1B\x81R`\0\x92P`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91Pb\xBF\xF0M\x90a&\x7F\x90\x85\x90\x8C\x90\x8C\x90`\x04\x01aL\xEEV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a&\x9EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra&\xC6\x91\x90\x81\x01\x90aM\x08V[`\0\x83\x81R`3` R`@\x90 T\x90\x91P\x80\x15\x80\x15\x90a'&WP`\0\x83\x81R`3` R`@\x90 a&\xFB`\x01\x83aJ\xBEV[\x81T\x81\x10a'\x0BWa'\x0BaG\x82V[`\0\x91\x82R` \x90\x91 \x01T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15[\x15a(}W`\0\x83\x81R`3` R`@\x81 a'D`\x01\x84aJ\xBEV[\x81T\x81\x10a'TWa'TaG\x82V[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xC0\x1B\x03`\x01`@\x1B\x90\x91\x04\x81\x81\x16\x92P\x86\x16\x16\x15a(!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x85`$\x82\x01R`\0\x80Q` aNp\x839\x81Q\x91R`D\x82\x01R`\0\x80Q` aNP\x839\x81Q\x91R`d\x82\x01R\x7For: operator already registered `\x84\x82\x01R\x7Ffor some quorums being registere`\xA4\x82\x01Rd2\x1037\xB9`\xD9\x1B`\xC4\x82\x01R`\xE4\x01a\x08|V[`\0\x84\x81R`3` R`@\x90 \x94\x81\x17\x94C\x90a(@`\x01\x85aJ\xBEV[\x81T\x81\x10a(PWa(PaG\x82V[\x90`\0R` `\0 \x01`\0\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPP[`\0\x83\x81R`3` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x86\x81R`\x01`\x01`\xC0\x1B\x03\x8B\x81\x16\x84\x87\x01\x90\x81R\x85T`\x01\x80\x82\x01\x88U\x96\x8AR\x88\x8A \x95Q\x95\x01\x80T\x93Q\x91Q\x90\x92\x16`\x01`@\x1B\x02`\x01`\x01`@\x1B\x03\x91\x85\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x94\x16\x95\x90\x94\x16\x94\x90\x94\x17\x91\x90\x91\x17\x92\x90\x92\x16\x17\x90U`\x01`\x01`\xA0\x1B\x03\x8E\x16\x84R`4\x90\x92R\x90\x91 \x81\x01T`\xFF\x16`\x02\x81\x11\x15a)5Wa)5aC\xE5V[\x14a)\xCEW`@\x80Q\x80\x82\x01\x82R\x84\x81R`\x01` \x80\x83\x01\x82\x81R`\x01`\x01`\xA0\x1B\x03\x8F\x16`\0\x90\x81R`4\x90\x92R\x93\x90 \x82Q\x81U\x92Q\x83\x82\x01\x80T\x93\x94\x93\x91\x92\x90\x91`\xFF\x19\x16\x90\x83`\x02\x81\x11\x15a)\x90Wa)\x90aC\xE5V[\x02\x17\x90UPP`@Q\x84\x91P`\x01`\x01`\xA0\x1B\x03\x8C\x16\x90\x7F\xE8\xE6\x8C\xEF\x1C:v\x1E\xD7\xBE~\x84c\xA3u\xF2\x7F{\xC35\xE5\x18$\"<\xAC\xCEcn\xC5\xC3\xFE\x90`\0\x90\xA3[\x82\x7F\xEC)c\xAB!\xC1\xE5\x0E\x1EX*\xA5B\xAF.K\xF7\xBF8\xE6\xE1@<'\xB4.\x1C]nb\x1E\xAA\x87`@Qa)\xFE\x91\x90aG\xF0V[`@Q\x80\x91\x03\x90\xA2P\x92PPP[\x95\x94PPPPPV[`\0\x81`\0\x01Q\x82` \x01Q`@Q` \x01a*;\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[` \x80\x82\x01Q`\0\x90\x81R`5\x90\x91R`@\x90 T`\xFF\x16\x15a+\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`w`$\x82\x01R`\0\x80Q` aNp\x839\x81Q\x91R`D\x82\x01R\x7Fs._verifyChurnApproverSignatureO`d\x82\x01R\x7FnOperatorChurnApproval: churnApp`\x84\x82\x01R\x7Frover salt already used\0\0\0\0\0\0\0\0\0`\xA4\x82\x01R`\xC4\x01a\x08|V[B\x81`@\x01Q\x10\x15a+\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`w`$\x82\x01R`\0\x80Q` aNp\x839\x81Q\x91R`D\x82\x01R\x7Fs._verifyChurnApproverSignatureO`d\x82\x01R\x7FnOperatorChurnApproval: churnApp`\x84\x82\x01R\x7Frover signature expired\0\0\0\0\0\0\0\0\0`\xA4\x82\x01R`\xC4\x01a\x08|V[` \x80\x82\x01\x80Q`\0\x90\x81R`5\x90\x92R`@\x91\x82\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U`7T\x90Q\x91\x83\x01Qa, \x92`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91a,\x19\x91\x87\x91\x87\x91a\x1F\xC4V[\x83Qa7/V[PPPV[`\x01`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`4` R`@\x90 `\x01\x01T`\xFF\x16`\x02\x81\x11\x15a,WWa,WaC\xE5V[\x14a,\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R```$\x82\x01R`\0\x80Q` aNp\x839\x81Q\x91R`D\x82\x01R\x7Fs._deregisterOperatorWithCoordin`d\x82\x01R\x7Fator: operator is not registered`\x84\x82\x01R`\xA4\x01a\x08|V[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`4` R`@\x90 Ta-\0\x82a*\x15V[\x81\x14a-\x9CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`k`$\x82\x01R`\0\x80Q` aNp\x839\x81Q\x91R`D\x82\x01R\x7Fs._deregisterOperatorWithCoordin`d\x82\x01R\x7Fator: operatorId does not match `\x84\x82\x01Rj\x0E\x0E\xACMl\xAF$\r\x0C.m`\xAB\x1B`\xA4\x82\x01R`\xC4\x01a\x08|V[`\0a-\xDD\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa5\xA2\x92PPPV[`\0\x83\x81R`3` R`@\x81 T\x91\x92P\x90a-\xFC\x90`\x01\x90aJ\xBEV[`\0\x84\x81R`3` R`@\x81 \x80T\x92\x93P\x90\x91\x83\x90\x81\x10a.!Wa.!aG\x82V[`\0\x91\x82R` \x82 \x01T`\x01`@\x1B\x90\x04\x93\x84\x16`\x01`\x01`\xC0\x1B\x03\x90\x81\x16\x94\x16\x91Pa.N\x84a8\xE9V[\x90P\x80Q`\0\x14\x15a/\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x80`$\x82\x01R`\0\x80Q` aNp\x839\x81Q\x91R`D\x82\x01R\x7Fs._deregisterOperatorWithCoordin`d\x82\x01R\x7Fator: operator is not registered`\x84\x82\x01R\x7F for any of the provided quorums`\xA4\x82\x01R`\xC4\x01a\x08|V[`@Qc\x12\x1BM\x95`\xE1\x1B\x81R`\x01`\x01`\xC0\x1B\x03\x83\x16\x85\x14\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c$6\x9B*\x90a/_\x90\x8D\x90\x86\x90\x8C\x90`\x04\x01aM\xA1V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a/yW`\0\x80\xFD[PZ\xF1\x15\x80\x15a/\x8DW=`\0\x80>=`\0\xFD[PP`@Qc\xBD)\xB8\xCD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92Pc\xBD)\xB8\xCD\x91Pa/\xDF\x90\x89\x90\x86\x90`\x04\x01aM\xDDV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a/\xF9W`\0\x80\xFD[PZ\xF1\x15\x80\x15a0\rW=`\0\x80>=`\0\xFD[PP`@Qc\xBD)\xB8\xCD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92Pc\xBD)\xB8\xCD\x91Pa0_\x90\x89\x90\x86\x90`\x04\x01aM\xDDV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a0yW`\0\x80\xFD[PZ\xF1\x15\x80\x15a0\x8DW=`\0\x80>=`\0\xFD[PPP`\0\x87\x81R`3` R`@\x90 \x80TC\x92P\x86\x90\x81\x10a0\xB3Wa0\xB3aG\x82V[\x90`\0R` `\0 \x01`\0\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80a1wW`\0\x86\x81R`3` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x86\x81R`\x01`\x01`\xC0\x1B\x03\x8C\x19\x8B\x16\x81\x16\x95\x84\x01\x95\x86R\x84T`\x01\x81\x01\x86U\x94\x88R\x95\x90\x96 \x91Q\x91\x90\x92\x01\x80T\x95Q\x93Q\x90\x94\x16`\x01`@\x1B\x02`\x01`\x01`@\x1B\x03\x93\x83\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x96\x16\x91\x90\x92\x16\x17\x93\x90\x93\x17\x16\x91\x90\x91\x17\x90Ua1\xC6V[`\x01`\x01`\xA0\x1B\x03\x8A\x16`\0\x81\x81R`4` R`@\x80\x82 `\x01\x01\x80T`\xFF\x19\x16`\x02\x17\x90UQ\x88\x92\x91\x7F9o\xDC\xB1\x80\xCB\x0F\xEA&\x92\x81\x13\xFB\x0F\xD1\xC3T\x98c\xF9\xCDV>j\x18O\x1DW\x81\x16\xC8\xE4\x91\xA3[PPPPPPPPPPV[`7T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F1TW\xD8\xA8\xFE`\xF0J\xF1|\x16\xE2\xF5\xA5\xE1\xDBa+1d\x8EX\x03\x03`u\x9E\xF8\xF3R\x8C\x91\x01`@Q\x80\x91\x03\x90\xA1`7\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`8T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F\x8F0\xAB\t\xF4:l\x15}\x7F\xCE~\n\x13\xC0\x03\x04,\x1C\x95\xE8\xA7.z\x14j!\xC0\xCA\xA2M\xC9\x91\x01`@Q\x80\x91\x03\x90\xA1`8\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0Tb\x01\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a2\xCBWP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a3MW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x08|V[`\x01\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a\x1C[\x82a\"\xBDV[`\xFF\x82\x16`\0\x81\x81R`2` \x90\x81R`@\x91\x82\x90 \x84Q\x81T\x86\x84\x01\x80Q\x88\x87\x01\x80Qc\xFF\xFF\xFF\xFF\x90\x95\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x94\x16\x84\x17`\x01` \x1Ba\xFF\xFF\x93\x84\x16\x02\x17g\xFF\xFF\0\0\0\0\0\0\x19\x16`\x01`0\x1B\x95\x83\x16\x95\x90\x95\x02\x94\x90\x94\x17\x90\x94U\x85Q\x91\x82RQ\x83\x16\x93\x81\x01\x93\x90\x93RQ\x16\x91\x81\x01\x91\x90\x91R\x7F>\xE6\xFE\x8DTa\x02D\xC3\xE9\xD3\xC0f\xAEJ\xEE\x99x\x84\xAA(\xF1\x06\x16\xAE\x82\x19%@\x13\x18\xAC\x90``\x01`@Q\x80\x91\x03\x90\xA2PPV[`\0a4L\x86\x86\x86\x86\x86a#\xC2V[\x90P`\0[\x81Q\x81\x10\x15a\x1AVW`2`\0\x87\x87\x84\x81\x81\x10a4pWa4paG\x82V[\x91\x90\x91\x015`\xF8\x1C\x82RP` \x81\x01\x91\x90\x91R`@\x01`\0 T\x82Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x90\x83\x90\x83\x90\x81\x10a4\xA7Wa4\xA7aG\x82V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a5BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`n`$\x82\x01R`\0\x80Q` aNp\x839\x81Q\x91R`D\x82\x01R`\0\x80Q` aNP\x839\x81Q\x91R`d\x82\x01R\x7ForAndNoOverfilledQuorums: quorum`\x84\x82\x01Rm\x08\x1A\\\xC8\x1B\xDD\x99\\\x99\x9A[\x1B\x19Y`\x92\x1B`\xA4\x82\x01R`\xC4\x01a\x08|V[\x80a5L\x81aI\xC9V[\x91PPa4QV[`\0a\x07\xCAa5aa9FV[\x83`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x83\x90R`B\x81\x01\x82\x90R`\0\x90`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[`\0a\x01\0\x82Q\x11\x15a6+W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FBitmapUtils.orderedBytesArrayToB\x90\x82\x01R\x7Fitmap: orderedBytesArray is too `d\x82\x01Rclong`\xE0\x1B`\x84\x82\x01R`\xA4\x01a\x08|V[\x81Qa69WP`\0\x91\x90PV[`\0\x80\x83`\0\x81Q\x81\x10a6OWa6OaG\x82V[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x81\x90\x1B\x92P[\x84Q\x81\x10\x15a7&W\x84\x81\x81Q\x81\x10a6}Wa6}aG\x82V[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x1B\x91P\x82\x82\x11a7\x12W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: orderedBytesArray is not `d\x82\x01Rf\x1B\xDC\x99\x19\\\x99Y`\xCA\x1B`\x84\x82\x01R`\xA4\x01a\x08|V[\x91\x81\x17\x91a7\x1F\x81aI\xC9V[\x90Pa6bV[P\x90\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16;\x15a8IW`@Qc\x0B\x13]?`\xE1\x1B\x80\x82R\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\x16&\xBA~\x90a7o\x90\x86\x90\x86\x90`\x04\x01aM\xDDV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a7\x8CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a7\xB0\x91\x90aM\xF6V[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14a, W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`S`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: ERC1271 signature `d\x82\x01Rr\x1D\x99\\\x9AY\x9AX\xD8]\x1A[\xDB\x88\x19\x98Z[\x19Y`j\x1B`\x84\x82\x01R`\xA4\x01a\x08|V[\x82`\x01`\x01`\xA0\x1B\x03\x16a8]\x83\x83a:mV[`\x01`\x01`\xA0\x1B\x03\x16\x14a, W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: signature not from`d\x82\x01Rf\x109\xB4\xB3\xB72\xB9`\xC9\x1B`\x84\x82\x01R`\xA4\x01a\x08|V[```\0\x80[a\x01\0\x81\x10\x15a9?W`\x01\x81\x1B\x91P\x83\x82\x16\x15a9/W\x82\x81`\xF8\x1B`@Q` \x01a9\x1D\x92\x91\x90aN V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92P[a98\x81aI\xC9V[\x90Pa8\xEFV[PP\x91\x90PV[`\x000`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80\x15a9\x9FWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14[\x15a9\xC9WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[P`@\x80Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x80\x83\x01\x91\x90\x91R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x84\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x83\x01RF`\x80\x83\x01R0`\xA0\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\xC0\x90\x92\x01\x90\x92R\x80Q\x91\x01 \x90V[`\0\x80`\0a:|\x85\x85a:\x89V[\x91P\x91Pa\x1F\xBC\x81a:\xF9V[`\0\x80\x82Q`A\x14\x15a:\xC0W` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1Aa:\xB4\x87\x82\x85\x85a<\xB4V[\x94P\x94PPPPa:\xF2V[\x82Q`@\x14\x15a:\xEAW` \x83\x01Q`@\x84\x01Qa:\xDF\x86\x83\x83a=\xA1V[\x93P\x93PPPa:\xF2V[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0\x81`\x04\x81\x11\x15a;\rWa;\raC\xE5V[\x14\x15a;\x16WPV[`\x01\x81`\x04\x81\x11\x15a;*Wa;*aC\xE5V[\x14\x15a;xW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x08|V[`\x02\x81`\x04\x81\x11\x15a;\x8CWa;\x8CaC\xE5V[\x14\x15a;\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x08|V[`\x03\x81`\x04\x81\x11\x15a;\xEEWa;\xEEaC\xE5V[\x14\x15a<GW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x08|V[`\x04\x81`\x04\x81\x11\x15a<[Wa<[aC\xE5V[\x14\x15a\t\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 'v' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x08|V[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a<\xEBWP`\0\x90P`\x03a=\x98V[\x84`\xFF\x16`\x1B\x14\x15\x80\x15a=\x03WP\x84`\xFF\x16`\x1C\x14\x15[\x15a=\x14WP`\0\x90P`\x04a=\x98V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a=hW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a=\x91W`\0`\x01\x92P\x92PPa=\x98V[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x80`\x01`\x01`\xFF\x1B\x03\x83\x16\x81a=\xBE`\xFF\x86\x90\x1C`\x1BaI\xE4V[\x90Pa=\xCC\x87\x82\x88\x85a<\xB4V[\x93P\x93PPP\x93P\x93\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a=\xEDW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0` \x82\x84\x03\x12\x15a>\x0EW`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a>MWa>Ma>\x15V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a>{Wa>{a>\x15V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x83\x11\x15a>\x9CWa>\x9Ca>\x15V[a>\xAF`\x1F\x84\x01`\x1F\x19\x16` \x01a>SV[\x90P\x82\x81R\x83\x83\x83\x01\x11\x15a>\xC3W`\0\x80\xFD[\x82\x82` \x83\x017`\0` \x84\x83\x01\x01R\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a>\xEBW`\0\x80\xFD[a>\xFA\x83\x835` \x85\x01a>\x83V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a?\x13W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a?)W`\0\x80\xFD[a  \x84\x82\x85\x01a>\xDAV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\t\x81W`\0\x80\xFD[\x805a?U\x81a?5V[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a?lW`\0\x80\xFD[\x815a>\xFA\x81a?5V[`\0\x80\x83`\x1F\x84\x01\x12a?\x89W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a?\xA0W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a:\xF2W`\0\x80\xFD[`\0`@\x82\x84\x03\x12\x15a?\xCAW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a?\xECWa?\xECa>\x15V[`@R\x825\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0``\x82\x84\x03\x12\x15a@\x18W`\0\x80\xFD[a@ a>+V[\x90P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a@8W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a@IW`\0\x80\xFD[a@X\x84\x825` \x84\x01a>\x83V[\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\0\x80`\xC0\x89\x8B\x03\x12\x15a@\x91W`\0\x80\xFD[\x885`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a@\xA8W`\0\x80\xFD[a@\xB4\x8C\x83\x8D\x01a?wV[\x90\x9AP\x98P\x88\x91Pa@\xC9\x8C` \x8D\x01a?\xB8V[\x97P``\x8B\x015\x91P\x80\x82\x11\x15a@\xDFW`\0\x80\xFD[a@\xEB\x8C\x83\x8D\x01a?wV[\x90\x97P\x95P`\x80\x8B\x015\x91P\x80\x82\x11\x15aA\x04W`\0\x80\xFD[\x81\x8B\x01\x91P\x8B`\x1F\x83\x01\x12aA\x18W`\0\x80\xFD[\x815\x81\x81\x11\x15aA'W`\0\x80\xFD[\x8C` \x82`\x07\x1B\x85\x01\x01\x11\x15aA<W`\0\x80\xFD[` \x83\x01\x95P\x80\x94PP`\xA0\x8B\x015\x91P\x80\x82\x11\x15aAZW`\0\x80\xFD[PaAg\x8B\x82\x8C\x01a@\x06V[\x91PP\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\t\x81W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15aA\x9EW`\0\x80\xFD[\x835\x92P` \x84\x015aA\xB0\x81aAwV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15aA\xD6W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x81\x11\x15aA\xECW`\0\x80\xFD[aA\xF8\x86\x82\x87\x01a?wV[\x90\x94P\x92PaB\x0C\x90P\x85` \x86\x01a?\xB8V[\x90P\x92P\x92P\x92V[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15aB.WaB.a>\x15V[P`\x05\x1B` \x01\x90V[a\xFF\xFF\x81\x16\x81\x14a\t\x81W`\0\x80\xFD[`\0``\x82\x84\x03\x12\x15aBZW`\0\x80\xFD[aBba>+V[\x90P\x815aBo\x81aAwV[\x81R` \x82\x015aB\x7F\x81aB8V[` \x82\x01R`@\x82\x015aB\x92\x81aB8V[`@\x82\x01R\x92\x91PPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15aB\xB5W`\0\x80\xFD[\x855aB\xC0\x81a?5V[\x94P` \x86\x81\x015aB\xD1\x81a?5V[\x94P`@\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aB\xECW`\0\x80\xFD[\x87\x01`\x1F\x81\x01\x89\x13aB\xFDW`\0\x80\xFD[\x805aC\x10aC\x0B\x82aB\x15V[a>SV[\x81\x81R``\x91\x82\x02\x83\x01\x84\x01\x91\x84\x82\x01\x91\x90\x8C\x84\x11\x15aC/W`\0\x80\xFD[\x93\x85\x01\x93[\x83\x85\x10\x15aCUWaCF\x8D\x86aBHV[\x83R\x93\x84\x01\x93\x91\x85\x01\x91aC4V[\x81\x98PaCc\x81\x8D\x01a?JV[\x9A\x9D\x99\x9CP\x97\x9A`\x80\x015\x98\x97PPPPPPPPV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15aC\x90W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aC\xA7W`\0\x80\xFD[aC\xB3\x88\x83\x89\x01a?wV[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15aC\xCCW`\0\x80\xFD[PaC\xD9\x87\x82\x88\x01a?wV[\x95\x98\x94\x97P\x95PPPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x03\x81\x10aD\x19WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[\x81Q\x81R` \x80\x83\x01Q`@\x83\x01\x91aD8\x90\x84\x01\x82aC\xFBV[P\x92\x91PPV[\x805`\xFF\x81\x16\x81\x14a?UW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aDbW`\0\x80\xFD[a>\xFA\x82aD?V[`\0\x80`\x80\x83\x85\x03\x12\x15aD~W`\0\x80\xFD[aD\x87\x83aD?V[\x91PaD\x96\x84` \x85\x01aBHV[\x90P\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15aD\xB2W`\0\x80\xFD[\x825aD\xBD\x81aAwV[\x91P` \x83\x81\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aD\xD9W`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13aD\xEAW`\0\x80\xFD[\x805aD\xF8aC\x0B\x82aB\x15V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x88\x83\x11\x15aE\x17W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aE5W\x835\x82R\x92\x84\x01\x92\x90\x84\x01\x90aE\x1CV[\x80\x95PPPPPP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aE\x82W\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aE`V[P\x90\x96\x95PPPPPPV[`\0`\x80\x82\x84\x03\x12\x15aE\xA0W`\0\x80\xFD[aE\xA8a>+V[\x90PaE\xB3\x82aD?V[\x81R` \x82\x015aE\xC3\x81a?5V[` \x82\x01RaB\x92\x83`@\x84\x01a?\xB8V[`\0\x80`\0\x80`\x80\x80\x86\x88\x03\x12\x15aE\xECW`\0\x80\xFD[\x855\x94P` \x80\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aF\nW`\0\x80\xFD[\x87\x01`\x1F\x81\x01\x89\x13aF\x1BW`\0\x80\xFD[\x805aF)aC\x0B\x82aB\x15V[\x81\x81R`\x07\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x8B\x83\x11\x15aFHW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aFnWaF_\x8C\x85aE\x8EV[\x82R\x92\x85\x01\x92\x90\x84\x01\x90aFMV[\x98\x9B\x98\x9APPPP`@\x87\x015\x96``\x015\x95\x94PPPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aF\x9FW`\0\x80\xFD[\x845aF\xAA\x81a?5V[\x93P` \x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aF\xC5W`\0\x80\xFD[aF\xD1\x87\x82\x88\x01a?wV[\x90\x94P\x92PaF\xE5\x90P\x86`@\x87\x01a?\xB8V[\x90P\x92\x95\x91\x94P\x92PV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15aG\x08W`\0\x80\xFD[\x855`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aG\x1FW`\0\x80\xFD[aG+\x89\x83\x8A\x01a?wV[\x90\x97P\x95P\x85\x91PaG@\x89` \x8A\x01a?\xB8V[\x94P``\x88\x015\x91P\x80\x82\x11\x15aGVW`\0\x80\xFD[PaGc\x88\x82\x89\x01a?wV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[` \x81\x01a\x07\xCA\x82\x84aC\xFBV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0[\x83\x81\x10\x15aG\xB3W\x81\x81\x01Q\x83\x82\x01R` \x01aG\x9BV[\x83\x81\x11\x15a\x16\xDCWPP`\0\x91\x01RV[`\0\x81Q\x80\x84RaG\xDC\x81` \x86\x01` \x86\x01aG\x98V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a>\xFA` \x83\x01\x84aG\xC4V[`\0` \x82\x84\x03\x12\x15aH\x15W`\0\x80\xFD[\x81Qa>\xFA\x81a?5V[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15aH|W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a>\xFAW`\0\x80\xFD[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`\x19\x90\x82\x01R\x7FPausable: index is paused\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`\0`\x80\x82\x84\x03\x12\x15aI\x1DW`\0\x80\xFD[a>\xFA\x83\x83aE\x8EV[`\0` \x82\x84\x03\x12\x15aI9W`\0\x80\xFD[\x81Q`\x01`\x01``\x1B\x03\x81\x16\x81\x14a>\xFAW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aI\x8CWaI\x8CaIPV[\x02\x94\x93PPPPV[`\0`\x01`\x01``\x1B\x03\x80\x84\x16\x80aI\xBDWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[\x92\x16\x91\x90\x91\x04\x92\x91PPV[`\0`\0\x19\x82\x14\x15aI\xDDWaI\xDDaIPV[P`\x01\x01\x90V[`\0\x82\x19\x82\x11\x15aI\xF7WaI\xF7aIPV[P\x01\x90V[`\0\x80\x85\x85\x11\x15aJ\x0CW`\0\x80\xFD[\x83\x86\x11\x15aJ\x19W`\0\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[`\0`@\x82\x84\x03\x12\x15aJ8W`\0\x80\xFD[a>\xFA\x83\x83a?\xB8V[` \x80\x82R`b\x90\x82\x01R`\0\x80Q` aNp\x839\x81Q\x91R`@\x82\x01R\x7Fs.onlyServiceManagerOwner: calle``\x82\x01R\x7Fr is not the service manager own`\x80\x82\x01Ra2\xB9`\xF1\x1B`\xA0\x82\x01R`\xC0\x01\x90V[`\0\x82\x82\x10\x15aJ\xD0WaJ\xD0aIPV[P\x03\x90V[`\0` \x82\x84\x03\x12\x15aJ\xE7W`\0\x80\xFD[\x81Qa>\xFA\x81aB8V[`\0`\xFF\x82\x16`\xFF\x81\x14\x15aK\tWaK\taIPV[`\x01\x01\x92\x91PPV[`\0\x80``\x83\x85\x03\x12\x15aK%W`\0\x80\xFD[aK/\x84\x84a?\xB8V[\x91P`@\x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aKJW`\0\x80\xFD[aKV\x85\x82\x86\x01a>\xDAV[\x91PP\x92P\x92\x90PV[`\0c\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15aK}WaK}aIPV[\x03\x93\x92PPPV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x14\x15aK\x9FWaK\x9FaIPV[`\x01\x01\x93\x92PPPV[`\0`\xA0\x82\x01\x87\x83R` \x87\x81\x85\x01R`@`\xA0\x81\x86\x01R\x82\x88Q\x80\x85R`\xC0\x87\x01\x91P\x83\x8A\x01\x94P`\0[\x81\x81\x10\x15aL$W\x85Q\x80Q`\xFF\x16\x84R\x85\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x86\x85\x01R\x84\x01QaL\x10\x85\x85\x01\x82\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[P\x94\x84\x01\x94`\x80\x92\x90\x92\x01\x91`\x01\x01aK\xD5V[PP``\x86\x01\x97\x90\x97RPPPP`\x80\x01R\x93\x92PPPV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x85\x16\x81R`\x80` \x82\x01\x81\x90R`\0\x90aL\x8B\x90\x83\x01\x85\x87aL=V[\x83Q`@\x84\x01R` \x84\x01Q``\x84\x01R\x90Pa*\x0CV[`\0` \x82\x84\x03\x12\x15aL\xB5W`\0\x80\xFD[PQ\x91\x90PV[`\x01\x80`\xA0\x1B\x03\x85\x16\x81R\x83` \x82\x01R```@\x82\x01R`\0aL\xE4``\x83\x01\x84\x86aL=V[\x96\x95PPPPPPV[\x83\x81R`@` \x82\x01R`\0a \x1D`@\x83\x01\x84\x86aL=V[`\0` \x80\x83\x85\x03\x12\x15aM\x1BW`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aM1W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aMBW`\0\x80\xFD[\x80QaMPaC\x0B\x82aB\x15V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15aMoW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aM\x96W\x83QaM\x87\x81aAwV[\x82R\x92\x84\x01\x92\x90\x84\x01\x90aMtV[\x97\x96PPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R`\x80` \x82\x01\x81\x90R`\0\x90aM\xC5\x90\x83\x01\x85aG\xC4V[\x83Q`@\x84\x01R` \x84\x01Q``\x84\x01R\x90Pa  V[\x82\x81R`@` \x82\x01R`\0a  `@\x83\x01\x84aG\xC4V[`\0` \x82\x84\x03\x12\x15aN\x08W`\0\x80\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a>\xFAW`\0\x80\xFD[`\0\x83QaN2\x81\x84` \x88\x01aG\x98V[`\x01`\x01`\xF8\x1B\x03\x19\x93\x90\x93\x16\x91\x90\x92\x01\x90\x81R`\x01\x01\x92\x91PPV\xFEs._registerOperatorWithCoordinatBLSRegistryCoordinatorWithIndice\xA2dipfsX\"\x12 \xB1\xCC\0\x185f\x8B\xBF\xC8\xFAO\xBC\x88[\xF4\x1Ap\xE1\x1E\x83Tt\xFCqM\x7FW\xE3\xD2\x84\xF2\xBBdsolcC\0\x08\x0C\x003";
    /// The deployed bytecode of the contract.
    pub static BLSREGISTRYCOORDINATORWITHINDICES_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct BLSRegistryCoordinatorWithIndices<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for BLSRegistryCoordinatorWithIndices<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for BLSRegistryCoordinatorWithIndices<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for BLSRegistryCoordinatorWithIndices<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for BLSRegistryCoordinatorWithIndices<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(BLSRegistryCoordinatorWithIndices))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> BLSRegistryCoordinatorWithIndices<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    BLSREGISTRYCOORDINATORWITHINDICES_ABI.clone(),
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
                BLSREGISTRYCOORDINATORWITHINDICES_ABI.clone(),
                BLSREGISTRYCOORDINATORWITHINDICES_BYTECODE.clone().into(),
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
        ///Calls the contract's `calculateOperatorChurnApprovalDigestHash` (0xab269b2d) function
        pub fn calculate_operator_churn_approval_digest_hash(
            &self,
            registering_operator_id: [u8; 32],
            operator_kick_params: ::std::vec::Vec<OperatorKickParam>,
            salt: [u8; 32],
            expiry: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [171, 38, 155, 45],
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
        ///Calls the contract's `deregisterOperatorWithCoordinator` (0x455a43fc) function
        pub fn deregister_operator_with_coordinator(
            &self,
            quorum_numbers: ::ethers::core::types::Bytes,
            pubkey: G1Point,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([69, 90, 67, 252], (quorum_numbers, pubkey))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deregisterOperatorWithCoordinator` (0xc81b1ff4) function
        pub fn deregister_operator_with_coordinator_with_quorum_numbers_and_deregistration_data(
            &self,
            quorum_numbers: ::ethers::core::types::Bytes,
            deregistration_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([200, 27, 31, 244], (quorum_numbers, deregistration_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ejectOperatorFromCoordinator` (0xb8347acc) function
        pub fn eject_operator_from_coordinator(
            &self,
            operator: ::ethers::core::types::Address,
            quorum_numbers: ::ethers::core::types::Bytes,
            pubkey: G1Point,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([184, 52, 122, 204], (operator, quorum_numbers, pubkey))
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
        ///Calls the contract's `getCurrentQuorumBitmapByOperatorId` (0x3431af25) function
        pub fn get_current_quorum_bitmap_by_operator_id(
            &self,
            operator_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([52, 49, 175, 37], operator_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOperator` (0x5865c60c) function
        pub fn get_operator(
            &self,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, Operator> {
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
        ///Calls the contract's `getQuorumBitmapByOperatorIdAtBlockNumberByIndex` (0x3064620d) function
        pub fn get_quorum_bitmap_by_operator_id_at_block_number_by_index(
            &self,
            operator_id: [u8; 32],
            block_number: u32,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([48, 100, 98, 13], (operator_id, block_number, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getQuorumBitmapIndicesByOperatorIdsAtBlockNumber` (0x85020d49) function
        pub fn get_quorum_bitmap_indices_by_operator_ids_at_block_number(
            &self,
            block_number: u32,
            operator_ids: ::std::vec::Vec<[u8; 32]>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<u32>> {
            self.0
                .method_hash([133, 2, 13, 73], (block_number, operator_ids))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getQuorumBitmapUpdateByOperatorIdByIndex` (0x0159f1ce) function
        pub fn get_quorum_bitmap_update_by_operator_id_by_index(
            &self,
            operator_id: [u8; 32],
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, QuorumBitmapUpdate> {
            self.0
                .method_hash([1, 89, 241, 206], (operator_id, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getQuorumBitmapUpdateByOperatorIdLength` (0x055a62b6) function
        pub fn get_quorum_bitmap_update_by_operator_id_length(
            &self,
            operator_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([5, 90, 98, 182], operator_id)
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
        ///Calls the contract's `initialize` (0x4b76b9d5) function
        pub fn initialize(
            &self,
            churn_approver: ::ethers::core::types::Address,
            ejector: ::ethers::core::types::Address,
            operator_set_params: ::std::vec::Vec<OperatorSetParam>,
            pauser_registry: ::ethers::core::types::Address,
            initial_paused_status: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [75, 118, 185, 213],
                    (
                        churn_approver,
                        ejector,
                        operator_set_params,
                        pauser_registry,
                        initial_paused_status,
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
        ///Calls the contract's `registerOperatorWithCoordinator` (0x25007576) function
        pub fn register_operator_with_coordinator_2(
            &self,
            quorum_numbers: ::ethers::core::types::Bytes,
            pubkey: G1Point,
            socket: ::std::string::String,
            operator_kick_params: ::std::vec::Vec<OperatorKickParam>,
            signature_with_salt_and_expiry: SignatureWithSaltAndExpiry,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [37, 0, 117, 118],
                    (
                        quorum_numbers,
                        pubkey,
                        socket,
                        operator_kick_params,
                        signature_with_salt_and_expiry,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerOperatorWithCoordinator` (0x526ea94e) function
        pub fn register_operator_with_coordinator_0(
            &self,
            quorum_numbers: ::ethers::core::types::Bytes,
            registration_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([82, 110, 169, 78], (quorum_numbers, registration_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerOperatorWithCoordinator` (0xc66ab9ca) function
        pub fn register_operator_with_coordinator_1(
            &self,
            quorum_numbers: ::ethers::core::types::Bytes,
            pubkey: G1Point,
            socket: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([198, 106, 185, 202], (quorum_numbers, pubkey, socket))
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
            operator_set_param: OperatorSetParam,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([91, 11, 130, 159], (quorum_number, operator_set_param))
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
        ///Calls the contract's `slasher` (0xb1344271) function
        pub fn slasher(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([177, 52, 66, 113], ())
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
        ///Calls the contract's `unpause` (0xfabc1cbc) function
        pub fn unpause(
            &self,
            new_paused_status: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 188, 28, 188], new_paused_status)
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
            BLSRegistryCoordinatorWithIndicesEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for BLSRegistryCoordinatorWithIndices<M> {
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
    pub enum BLSRegistryCoordinatorWithIndicesEvents {
        ChurnApproverUpdatedFilter(ChurnApproverUpdatedFilter),
        EjectorUpdatedFilter(EjectorUpdatedFilter),
        InitializedFilter(InitializedFilter),
        OperatorDeregisteredFilter(OperatorDeregisteredFilter),
        OperatorRegisteredFilter(OperatorRegisteredFilter),
        OperatorSetParamsUpdatedFilter(OperatorSetParamsUpdatedFilter),
        OperatorSocketUpdateFilter(OperatorSocketUpdateFilter),
        PausedFilter(PausedFilter),
        PauserRegistrySetFilter(PauserRegistrySetFilter),
        UnpausedFilter(UnpausedFilter),
    }
    impl ::ethers::contract::EthLogDecode for BLSRegistryCoordinatorWithIndicesEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ChurnApproverUpdatedFilter::decode_log(log) {
                return Ok(
                    BLSRegistryCoordinatorWithIndicesEvents::ChurnApproverUpdatedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = EjectorUpdatedFilter::decode_log(log) {
                return Ok(
                    BLSRegistryCoordinatorWithIndicesEvents::EjectorUpdatedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(
                    BLSRegistryCoordinatorWithIndicesEvents::InitializedFilter(decoded),
                );
            }
            if let Ok(decoded) = OperatorDeregisteredFilter::decode_log(log) {
                return Ok(
                    BLSRegistryCoordinatorWithIndicesEvents::OperatorDeregisteredFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = OperatorRegisteredFilter::decode_log(log) {
                return Ok(
                    BLSRegistryCoordinatorWithIndicesEvents::OperatorRegisteredFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = OperatorSetParamsUpdatedFilter::decode_log(log) {
                return Ok(
                    BLSRegistryCoordinatorWithIndicesEvents::OperatorSetParamsUpdatedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = OperatorSocketUpdateFilter::decode_log(log) {
                return Ok(
                    BLSRegistryCoordinatorWithIndicesEvents::OperatorSocketUpdateFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(
                    BLSRegistryCoordinatorWithIndicesEvents::PausedFilter(decoded),
                );
            }
            if let Ok(decoded) = PauserRegistrySetFilter::decode_log(log) {
                return Ok(
                    BLSRegistryCoordinatorWithIndicesEvents::PauserRegistrySetFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(
                    BLSRegistryCoordinatorWithIndicesEvents::UnpausedFilter(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for BLSRegistryCoordinatorWithIndicesEvents {
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
                Self::PausedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauserRegistrySetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnpausedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ChurnApproverUpdatedFilter>
    for BLSRegistryCoordinatorWithIndicesEvents {
        fn from(value: ChurnApproverUpdatedFilter) -> Self {
            Self::ChurnApproverUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<EjectorUpdatedFilter>
    for BLSRegistryCoordinatorWithIndicesEvents {
        fn from(value: EjectorUpdatedFilter) -> Self {
            Self::EjectorUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter>
    for BLSRegistryCoordinatorWithIndicesEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OperatorDeregisteredFilter>
    for BLSRegistryCoordinatorWithIndicesEvents {
        fn from(value: OperatorDeregisteredFilter) -> Self {
            Self::OperatorDeregisteredFilter(value)
        }
    }
    impl ::core::convert::From<OperatorRegisteredFilter>
    for BLSRegistryCoordinatorWithIndicesEvents {
        fn from(value: OperatorRegisteredFilter) -> Self {
            Self::OperatorRegisteredFilter(value)
        }
    }
    impl ::core::convert::From<OperatorSetParamsUpdatedFilter>
    for BLSRegistryCoordinatorWithIndicesEvents {
        fn from(value: OperatorSetParamsUpdatedFilter) -> Self {
            Self::OperatorSetParamsUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<OperatorSocketUpdateFilter>
    for BLSRegistryCoordinatorWithIndicesEvents {
        fn from(value: OperatorSocketUpdateFilter) -> Self {
            Self::OperatorSocketUpdateFilter(value)
        }
    }
    impl ::core::convert::From<PausedFilter>
    for BLSRegistryCoordinatorWithIndicesEvents {
        fn from(value: PausedFilter) -> Self {
            Self::PausedFilter(value)
        }
    }
    impl ::core::convert::From<PauserRegistrySetFilter>
    for BLSRegistryCoordinatorWithIndicesEvents {
        fn from(value: PauserRegistrySetFilter) -> Self {
            Self::PauserRegistrySetFilter(value)
        }
    }
    impl ::core::convert::From<UnpausedFilter>
    for BLSRegistryCoordinatorWithIndicesEvents {
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
    ///Container type for all input parameters for the `calculateOperatorChurnApprovalDigestHash` function with signature `calculateOperatorChurnApprovalDigestHash(bytes32,(uint8,address,(uint256,uint256))[],bytes32,uint256)` and selector `0xab269b2d`
    #[derive(
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
        abi = "calculateOperatorChurnApprovalDigestHash(bytes32,(uint8,address,(uint256,uint256))[],bytes32,uint256)"
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
    ///Container type for all input parameters for the `deregisterOperatorWithCoordinator` function with signature `deregisterOperatorWithCoordinator(bytes,(uint256,uint256))` and selector `0x455a43fc`
    #[derive(
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
        name = "deregisterOperatorWithCoordinator",
        abi = "deregisterOperatorWithCoordinator(bytes,(uint256,uint256))"
    )]
    pub struct DeregisterOperatorWithCoordinatorCall {
        pub quorum_numbers: ::ethers::core::types::Bytes,
        pub pubkey: G1Point,
    }
    ///Container type for all input parameters for the `deregisterOperatorWithCoordinator` function with signature `deregisterOperatorWithCoordinator(bytes,bytes)` and selector `0xc81b1ff4`
    #[derive(
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
        name = "deregisterOperatorWithCoordinator",
        abi = "deregisterOperatorWithCoordinator(bytes,bytes)"
    )]
    pub struct DeregisterOperatorWithCoordinatorWithQuorumNumbersAndDeregistrationDataCall {
        pub quorum_numbers: ::ethers::core::types::Bytes,
        pub deregistration_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `ejectOperatorFromCoordinator` function with signature `ejectOperatorFromCoordinator(address,bytes,(uint256,uint256))` and selector `0xb8347acc`
    #[derive(
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
        name = "ejectOperatorFromCoordinator",
        abi = "ejectOperatorFromCoordinator(address,bytes,(uint256,uint256))"
    )]
    pub struct EjectOperatorFromCoordinatorCall {
        pub operator: ::ethers::core::types::Address,
        pub quorum_numbers: ::ethers::core::types::Bytes,
        pub pubkey: G1Point,
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
    ///Container type for all input parameters for the `getCurrentQuorumBitmapByOperatorId` function with signature `getCurrentQuorumBitmapByOperatorId(bytes32)` and selector `0x3431af25`
    #[derive(
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
        name = "getCurrentQuorumBitmapByOperatorId",
        abi = "getCurrentQuorumBitmapByOperatorId(bytes32)"
    )]
    pub struct GetCurrentQuorumBitmapByOperatorIdCall {
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
    ///Container type for all input parameters for the `getQuorumBitmapByOperatorIdAtBlockNumberByIndex` function with signature `getQuorumBitmapByOperatorIdAtBlockNumberByIndex(bytes32,uint32,uint256)` and selector `0x3064620d`
    #[derive(
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
        name = "getQuorumBitmapByOperatorIdAtBlockNumberByIndex",
        abi = "getQuorumBitmapByOperatorIdAtBlockNumberByIndex(bytes32,uint32,uint256)"
    )]
    pub struct GetQuorumBitmapByOperatorIdAtBlockNumberByIndexCall {
        pub operator_id: [u8; 32],
        pub block_number: u32,
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getQuorumBitmapIndicesByOperatorIdsAtBlockNumber` function with signature `getQuorumBitmapIndicesByOperatorIdsAtBlockNumber(uint32,bytes32[])` and selector `0x85020d49`
    #[derive(
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
        name = "getQuorumBitmapIndicesByOperatorIdsAtBlockNumber",
        abi = "getQuorumBitmapIndicesByOperatorIdsAtBlockNumber(uint32,bytes32[])"
    )]
    pub struct GetQuorumBitmapIndicesByOperatorIdsAtBlockNumberCall {
        pub block_number: u32,
        pub operator_ids: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all input parameters for the `getQuorumBitmapUpdateByOperatorIdByIndex` function with signature `getQuorumBitmapUpdateByOperatorIdByIndex(bytes32,uint256)` and selector `0x0159f1ce`
    #[derive(
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
        name = "getQuorumBitmapUpdateByOperatorIdByIndex",
        abi = "getQuorumBitmapUpdateByOperatorIdByIndex(bytes32,uint256)"
    )]
    pub struct GetQuorumBitmapUpdateByOperatorIdByIndexCall {
        pub operator_id: [u8; 32],
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getQuorumBitmapUpdateByOperatorIdLength` function with signature `getQuorumBitmapUpdateByOperatorIdLength(bytes32)` and selector `0x055a62b6`
    #[derive(
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
        name = "getQuorumBitmapUpdateByOperatorIdLength",
        abi = "getQuorumBitmapUpdateByOperatorIdLength(bytes32)"
    )]
    pub struct GetQuorumBitmapUpdateByOperatorIdLengthCall {
        pub operator_id: [u8; 32],
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
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address,(uint32,uint16,uint16)[],address,uint256)` and selector `0x4b76b9d5`
    #[derive(
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
        abi = "initialize(address,address,(uint32,uint16,uint16)[],address,uint256)"
    )]
    pub struct InitializeCall {
        pub churn_approver: ::ethers::core::types::Address,
        pub ejector: ::ethers::core::types::Address,
        pub operator_set_params: ::std::vec::Vec<OperatorSetParam>,
        pub pauser_registry: ::ethers::core::types::Address,
        pub initial_paused_status: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `registerOperatorWithCoordinator` function with signature `registerOperatorWithCoordinator(bytes,(uint256,uint256),string,(uint8,address,(uint256,uint256))[],(bytes,bytes32,uint256))` and selector `0x25007576`
    #[derive(
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
        name = "registerOperatorWithCoordinator",
        abi = "registerOperatorWithCoordinator(bytes,(uint256,uint256),string,(uint8,address,(uint256,uint256))[],(bytes,bytes32,uint256))"
    )]
    pub struct RegisterOperatorWithCoordinator2Call {
        pub quorum_numbers: ::ethers::core::types::Bytes,
        pub pubkey: G1Point,
        pub socket: ::std::string::String,
        pub operator_kick_params: ::std::vec::Vec<OperatorKickParam>,
        pub signature_with_salt_and_expiry: SignatureWithSaltAndExpiry,
    }
    ///Container type for all input parameters for the `registerOperatorWithCoordinator` function with signature `registerOperatorWithCoordinator(bytes,bytes)` and selector `0x526ea94e`
    #[derive(
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
        name = "registerOperatorWithCoordinator",
        abi = "registerOperatorWithCoordinator(bytes,bytes)"
    )]
    pub struct RegisterOperatorWithCoordinator0Call {
        pub quorum_numbers: ::ethers::core::types::Bytes,
        pub registration_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `registerOperatorWithCoordinator` function with signature `registerOperatorWithCoordinator(bytes,(uint256,uint256),string)` and selector `0xc66ab9ca`
    #[derive(
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
        name = "registerOperatorWithCoordinator",
        abi = "registerOperatorWithCoordinator(bytes,(uint256,uint256),string)"
    )]
    pub struct RegisterOperatorWithCoordinator1Call {
        pub quorum_numbers: ::ethers::core::types::Bytes,
        pub pubkey: G1Point,
        pub socket: ::std::string::String,
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
        pub operator_set_param: OperatorSetParam,
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
    ///Container type for all input parameters for the `slasher` function with signature `slasher()` and selector `0xb1344271`
    #[derive(
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
    #[ethcall(name = "slasher", abi = "slasher()")]
    pub struct SlasherCall;
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
    pub enum BLSRegistryCoordinatorWithIndicesCalls {
        OperatorChurnApprovalTypehash(OperatorChurnApprovalTypehashCall),
        BlsPubkeyRegistry(BlsPubkeyRegistryCall),
        CalculateOperatorChurnApprovalDigestHash(
            CalculateOperatorChurnApprovalDigestHashCall,
        ),
        ChurnApprover(ChurnApproverCall),
        DeregisterOperatorWithCoordinator(DeregisterOperatorWithCoordinatorCall),
        DeregisterOperatorWithCoordinatorWithQuorumNumbersAndDeregistrationData(
            DeregisterOperatorWithCoordinatorWithQuorumNumbersAndDeregistrationDataCall,
        ),
        EjectOperatorFromCoordinator(EjectOperatorFromCoordinatorCall),
        Ejector(EjectorCall),
        GetCurrentQuorumBitmapByOperatorId(GetCurrentQuorumBitmapByOperatorIdCall),
        GetOperator(GetOperatorCall),
        GetOperatorFromId(GetOperatorFromIdCall),
        GetOperatorId(GetOperatorIdCall),
        GetOperatorSetParams(GetOperatorSetParamsCall),
        GetOperatorStatus(GetOperatorStatusCall),
        GetQuorumBitmapByOperatorIdAtBlockNumberByIndex(
            GetQuorumBitmapByOperatorIdAtBlockNumberByIndexCall,
        ),
        GetQuorumBitmapIndicesByOperatorIdsAtBlockNumber(
            GetQuorumBitmapIndicesByOperatorIdsAtBlockNumberCall,
        ),
        GetQuorumBitmapUpdateByOperatorIdByIndex(
            GetQuorumBitmapUpdateByOperatorIdByIndexCall,
        ),
        GetQuorumBitmapUpdateByOperatorIdLength(
            GetQuorumBitmapUpdateByOperatorIdLengthCall,
        ),
        IndexRegistry(IndexRegistryCall),
        Initialize(InitializeCall),
        IsChurnApproverSaltUsed(IsChurnApproverSaltUsedCall),
        NumRegistries(NumRegistriesCall),
        Pause(PauseCall),
        PauseAll(PauseAllCall),
        PausedWithIndex(PausedWithIndexCall),
        Paused(PausedCall),
        PauserRegistry(PauserRegistryCall),
        RegisterOperatorWithCoordinator2(RegisterOperatorWithCoordinator2Call),
        RegisterOperatorWithCoordinator0(RegisterOperatorWithCoordinator0Call),
        RegisterOperatorWithCoordinator1(RegisterOperatorWithCoordinator1Call),
        Registries(RegistriesCall),
        ServiceManager(ServiceManagerCall),
        SetChurnApprover(SetChurnApproverCall),
        SetEjector(SetEjectorCall),
        SetOperatorSetParams(SetOperatorSetParamsCall),
        SetPauserRegistry(SetPauserRegistryCall),
        Slasher(SlasherCall),
        StakeRegistry(StakeRegistryCall),
        Unpause(UnpauseCall),
        UpdateSocket(UpdateSocketCall),
    }
    impl ::ethers::core::abi::AbiDecode for BLSRegistryCoordinatorWithIndicesCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <OperatorChurnApprovalTypehashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OperatorChurnApprovalTypehash(decoded));
            }
            if let Ok(decoded) = <BlsPubkeyRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BlsPubkeyRegistry(decoded));
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
            if let Ok(decoded) = <DeregisterOperatorWithCoordinatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeregisterOperatorWithCoordinator(decoded));
            }
            if let Ok(decoded) = <DeregisterOperatorWithCoordinatorWithQuorumNumbersAndDeregistrationDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::DeregisterOperatorWithCoordinatorWithQuorumNumbersAndDeregistrationData(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = <EjectOperatorFromCoordinatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EjectOperatorFromCoordinator(decoded));
            }
            if let Ok(decoded) = <EjectorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Ejector(decoded));
            }
            if let Ok(decoded) = <GetCurrentQuorumBitmapByOperatorIdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetCurrentQuorumBitmapByOperatorId(decoded));
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
            if let Ok(decoded) = <GetQuorumBitmapByOperatorIdAtBlockNumberByIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::GetQuorumBitmapByOperatorIdAtBlockNumberByIndex(decoded),
                );
            }
            if let Ok(decoded) = <GetQuorumBitmapIndicesByOperatorIdsAtBlockNumberCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::GetQuorumBitmapIndicesByOperatorIdsAtBlockNumber(decoded),
                );
            }
            if let Ok(decoded) = <GetQuorumBitmapUpdateByOperatorIdByIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetQuorumBitmapUpdateByOperatorIdByIndex(decoded));
            }
            if let Ok(decoded) = <GetQuorumBitmapUpdateByOperatorIdLengthCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetQuorumBitmapUpdateByOperatorIdLength(decoded));
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
            if let Ok(decoded) = <RegisterOperatorWithCoordinator2Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RegisterOperatorWithCoordinator2(decoded));
            }
            if let Ok(decoded) = <RegisterOperatorWithCoordinator0Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RegisterOperatorWithCoordinator0(decoded));
            }
            if let Ok(decoded) = <RegisterOperatorWithCoordinator1Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RegisterOperatorWithCoordinator1(decoded));
            }
            if let Ok(decoded) = <RegistriesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Registries(decoded));
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
            if let Ok(decoded) = <SlasherCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Slasher(decoded));
            }
            if let Ok(decoded) = <StakeRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StakeRegistry(decoded));
            }
            if let Ok(decoded) = <UnpauseCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Unpause(decoded));
            }
            if let Ok(decoded) = <UpdateSocketCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateSocket(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for BLSRegistryCoordinatorWithIndicesCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::OperatorChurnApprovalTypehash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BlsPubkeyRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateOperatorChurnApprovalDigestHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChurnApprover(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeregisterOperatorWithCoordinator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeregisterOperatorWithCoordinatorWithQuorumNumbersAndDeregistrationData(
                    element,
                ) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EjectOperatorFromCoordinator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Ejector(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetCurrentQuorumBitmapByOperatorId(element) => {
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
                Self::GetQuorumBitmapByOperatorIdAtBlockNumberByIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetQuorumBitmapIndicesByOperatorIdsAtBlockNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetQuorumBitmapUpdateByOperatorIdByIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetQuorumBitmapUpdateByOperatorIdLength(element) => {
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
                Self::RegisterOperatorWithCoordinator2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterOperatorWithCoordinator0(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterOperatorWithCoordinator1(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Registries(element) => {
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
                Self::Slasher(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StakeRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unpause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateSocket(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for BLSRegistryCoordinatorWithIndicesCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::OperatorChurnApprovalTypehash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BlsPubkeyRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::CalculateOperatorChurnApprovalDigestHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChurnApprover(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeregisterOperatorWithCoordinator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DeregisterOperatorWithCoordinatorWithQuorumNumbersAndDeregistrationData(
                    element,
                ) => ::core::fmt::Display::fmt(element, f),
                Self::EjectOperatorFromCoordinator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Ejector(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCurrentQuorumBitmapByOperatorId(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOperatorFromId(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOperatorId(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOperatorSetParams(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetOperatorStatus(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetQuorumBitmapByOperatorIdAtBlockNumberByIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetQuorumBitmapIndicesByOperatorIdsAtBlockNumber(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetQuorumBitmapUpdateByOperatorIdByIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetQuorumBitmapUpdateByOperatorIdLength(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IndexRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsChurnApproverSaltUsed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NumRegistries(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pause(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauseAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::PausedWithIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::Paused(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauserRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterOperatorWithCoordinator2(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RegisterOperatorWithCoordinator0(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RegisterOperatorWithCoordinator1(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Registries(element) => ::core::fmt::Display::fmt(element, f),
                Self::ServiceManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetChurnApprover(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetEjector(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetOperatorSetParams(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetPauserRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::Slasher(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unpause(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateSocket(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<OperatorChurnApprovalTypehashCall>
    for BLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: OperatorChurnApprovalTypehashCall) -> Self {
            Self::OperatorChurnApprovalTypehash(value)
        }
    }
    impl ::core::convert::From<BlsPubkeyRegistryCall>
    for BLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: BlsPubkeyRegistryCall) -> Self {
            Self::BlsPubkeyRegistry(value)
        }
    }
    impl ::core::convert::From<CalculateOperatorChurnApprovalDigestHashCall>
    for BLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: CalculateOperatorChurnApprovalDigestHashCall) -> Self {
            Self::CalculateOperatorChurnApprovalDigestHash(value)
        }
    }
    impl ::core::convert::From<ChurnApproverCall>
    for BLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: ChurnApproverCall) -> Self {
            Self::ChurnApprover(value)
        }
    }
    impl ::core::convert::From<DeregisterOperatorWithCoordinatorCall>
    for BLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: DeregisterOperatorWithCoordinatorCall) -> Self {
            Self::DeregisterOperatorWithCoordinator(value)
        }
    }
    impl ::core::convert::From<
        DeregisterOperatorWithCoordinatorWithQuorumNumbersAndDeregistrationDataCall,
    > for BLSRegistryCoordinatorWithIndicesCalls {
        fn from(
            value: DeregisterOperatorWithCoordinatorWithQuorumNumbersAndDeregistrationDataCall,
        ) -> Self {
            Self::DeregisterOperatorWithCoordinatorWithQuorumNumbersAndDeregistrationData(
                value,
            )
        }
    }
    impl ::core::convert::From<EjectOperatorFromCoordinatorCall>
    for BLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: EjectOperatorFromCoordinatorCall) -> Self {
            Self::EjectOperatorFromCoordinator(value)
        }
    }
    impl ::core::convert::From<EjectorCall> for BLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: EjectorCall) -> Self {
            Self::Ejector(value)
        }
    }
    impl ::core::convert::From<GetCurrentQuorumBitmapByOperatorIdCall>
    for BLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: GetCurrentQuorumBitmapByOperatorIdCall) -> Self {
            Self::GetCurrentQuorumBitmapByOperatorId(value)
        }
    }
    impl ::core::convert::From<GetOperatorCall>
    for BLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: GetOperatorCall) -> Self {
            Self::GetOperator(value)
        }
    }
    impl ::core::convert::From<GetOperatorFromIdCall>
    for BLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: GetOperatorFromIdCall) -> Self {
            Self::GetOperatorFromId(value)
        }
    }
    impl ::core::convert::From<GetOperatorIdCall>
    for BLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: GetOperatorIdCall) -> Self {
            Self::GetOperatorId(value)
        }
    }
    impl ::core::convert::From<GetOperatorSetParamsCall>
    for BLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: GetOperatorSetParamsCall) -> Self {
            Self::GetOperatorSetParams(value)
        }
    }
    impl ::core::convert::From<GetOperatorStatusCall>
    for BLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: GetOperatorStatusCall) -> Self {
            Self::GetOperatorStatus(value)
        }
    }
    impl ::core::convert::From<GetQuorumBitmapByOperatorIdAtBlockNumberByIndexCall>
    for BLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: GetQuorumBitmapByOperatorIdAtBlockNumberByIndexCall) -> Self {
            Self::GetQuorumBitmapByOperatorIdAtBlockNumberByIndex(value)
        }
    }
    impl ::core::convert::From<GetQuorumBitmapIndicesByOperatorIdsAtBlockNumberCall>
    for BLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: GetQuorumBitmapIndicesByOperatorIdsAtBlockNumberCall) -> Self {
            Self::GetQuorumBitmapIndicesByOperatorIdsAtBlockNumber(value)
        }
    }
    impl ::core::convert::From<GetQuorumBitmapUpdateByOperatorIdByIndexCall>
    for BLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: GetQuorumBitmapUpdateByOperatorIdByIndexCall) -> Self {
            Self::GetQuorumBitmapUpdateByOperatorIdByIndex(value)
        }
    }
    impl ::core::convert::From<GetQuorumBitmapUpdateByOperatorIdLengthCall>
    for BLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: GetQuorumBitmapUpdateByOperatorIdLengthCall) -> Self {
            Self::GetQuorumBitmapUpdateByOperatorIdLength(value)
        }
    }
    impl ::core::convert::From<IndexRegistryCall>
    for BLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: IndexRegistryCall) -> Self {
            Self::IndexRegistry(value)
        }
    }
    impl ::core::convert::From<InitializeCall>
    for BLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<IsChurnApproverSaltUsedCall>
    for BLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: IsChurnApproverSaltUsedCall) -> Self {
            Self::IsChurnApproverSaltUsed(value)
        }
    }
    impl ::core::convert::From<NumRegistriesCall>
    for BLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: NumRegistriesCall) -> Self {
            Self::NumRegistries(value)
        }
    }
    impl ::core::convert::From<PauseCall> for BLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: PauseCall) -> Self {
            Self::Pause(value)
        }
    }
    impl ::core::convert::From<PauseAllCall> for BLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: PauseAllCall) -> Self {
            Self::PauseAll(value)
        }
    }
    impl ::core::convert::From<PausedWithIndexCall>
    for BLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: PausedWithIndexCall) -> Self {
            Self::PausedWithIndex(value)
        }
    }
    impl ::core::convert::From<PausedCall> for BLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: PausedCall) -> Self {
            Self::Paused(value)
        }
    }
    impl ::core::convert::From<PauserRegistryCall>
    for BLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: PauserRegistryCall) -> Self {
            Self::PauserRegistry(value)
        }
    }
    impl ::core::convert::From<RegisterOperatorWithCoordinator2Call>
    for BLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: RegisterOperatorWithCoordinator2Call) -> Self {
            Self::RegisterOperatorWithCoordinator2(value)
        }
    }
    impl ::core::convert::From<RegisterOperatorWithCoordinator0Call>
    for BLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: RegisterOperatorWithCoordinator0Call) -> Self {
            Self::RegisterOperatorWithCoordinator0(value)
        }
    }
    impl ::core::convert::From<RegisterOperatorWithCoordinator1Call>
    for BLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: RegisterOperatorWithCoordinator1Call) -> Self {
            Self::RegisterOperatorWithCoordinator1(value)
        }
    }
    impl ::core::convert::From<RegistriesCall>
    for BLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: RegistriesCall) -> Self {
            Self::Registries(value)
        }
    }
    impl ::core::convert::From<ServiceManagerCall>
    for BLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: ServiceManagerCall) -> Self {
            Self::ServiceManager(value)
        }
    }
    impl ::core::convert::From<SetChurnApproverCall>
    for BLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: SetChurnApproverCall) -> Self {
            Self::SetChurnApprover(value)
        }
    }
    impl ::core::convert::From<SetEjectorCall>
    for BLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: SetEjectorCall) -> Self {
            Self::SetEjector(value)
        }
    }
    impl ::core::convert::From<SetOperatorSetParamsCall>
    for BLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: SetOperatorSetParamsCall) -> Self {
            Self::SetOperatorSetParams(value)
        }
    }
    impl ::core::convert::From<SetPauserRegistryCall>
    for BLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: SetPauserRegistryCall) -> Self {
            Self::SetPauserRegistry(value)
        }
    }
    impl ::core::convert::From<SlasherCall> for BLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: SlasherCall) -> Self {
            Self::Slasher(value)
        }
    }
    impl ::core::convert::From<StakeRegistryCall>
    for BLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: StakeRegistryCall) -> Self {
            Self::StakeRegistry(value)
        }
    }
    impl ::core::convert::From<UnpauseCall> for BLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: UnpauseCall) -> Self {
            Self::Unpause(value)
        }
    }
    impl ::core::convert::From<UpdateSocketCall>
    for BLSRegistryCoordinatorWithIndicesCalls {
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
    ///Container type for all return fields from the `calculateOperatorChurnApprovalDigestHash` function with signature `calculateOperatorChurnApprovalDigestHash(bytes32,(uint8,address,(uint256,uint256))[],bytes32,uint256)` and selector `0xab269b2d`
    #[derive(
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
    ///Container type for all return fields from the `getCurrentQuorumBitmapByOperatorId` function with signature `getCurrentQuorumBitmapByOperatorId(bytes32)` and selector `0x3431af25`
    #[derive(
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
    pub struct GetCurrentQuorumBitmapByOperatorIdReturn(pub ::ethers::core::types::U256);
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
    pub struct GetOperatorReturn(pub Operator);
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
    ///Container type for all return fields from the `getQuorumBitmapByOperatorIdAtBlockNumberByIndex` function with signature `getQuorumBitmapByOperatorIdAtBlockNumberByIndex(bytes32,uint32,uint256)` and selector `0x3064620d`
    #[derive(
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
    pub struct GetQuorumBitmapByOperatorIdAtBlockNumberByIndexReturn(
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `getQuorumBitmapIndicesByOperatorIdsAtBlockNumber` function with signature `getQuorumBitmapIndicesByOperatorIdsAtBlockNumber(uint32,bytes32[])` and selector `0x85020d49`
    #[derive(
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
    pub struct GetQuorumBitmapIndicesByOperatorIdsAtBlockNumberReturn(
        pub ::std::vec::Vec<u32>,
    );
    ///Container type for all return fields from the `getQuorumBitmapUpdateByOperatorIdByIndex` function with signature `getQuorumBitmapUpdateByOperatorIdByIndex(bytes32,uint256)` and selector `0x0159f1ce`
    #[derive(
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
    pub struct GetQuorumBitmapUpdateByOperatorIdByIndexReturn(pub QuorumBitmapUpdate);
    ///Container type for all return fields from the `getQuorumBitmapUpdateByOperatorIdLength` function with signature `getQuorumBitmapUpdateByOperatorIdLength(bytes32)` and selector `0x055a62b6`
    #[derive(
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
    pub struct GetQuorumBitmapUpdateByOperatorIdLengthReturn(
        pub ::ethers::core::types::U256,
    );
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
    ///Container type for all return fields from the `slasher` function with signature `slasher()` and selector `0xb1344271`
    #[derive(
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
    pub struct SlasherReturn(pub ::ethers::core::types::Address);
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
