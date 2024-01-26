pub use bls_registry_coordinator_with_indices_harness::*;
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
pub mod bls_registry_coordinator_with_indices_harness {
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
                    ::std::borrow::ToOwned::to_owned("recordOperatorQuorumBitmapUpdate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "recordOperatorQuorumBitmapUpdate",
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
    pub static BLSREGISTRYCOORDINATORWITHINDICESHARNESS_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01\xE0`@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@Qb\0S]8\x03\x80b\0S]\x839\x81\x01`@\x81\x90Rb\0\x005\x91b\0\x01mV[`@\x80Q\x80\x82\x01\x82R`\x16\x81R\x7FAVSRegistryCoordinator\0\0\0\0\0\0\0\0\0\0` \x80\x83\x01\x91\x82R\x83Q\x80\x85\x01\x85R`\x06\x81Rev0.0.1`\xD0\x1B\x90\x82\x01R\x91Q\x90 `\xE0\x81\x90R\x7Fk\xDA~?8^H\x84\x10H9\x04D\xCC\xED\\\xC7\x95\xAF\x87u\x8A\xF6v\"\xE5\xF4\xF0\x88,J\x99a\x01\0\x81\x90RF`\xA0\x81\x81R\x85Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81\x87\x01\x81\x90R\x81\x88\x01\x95\x90\x95R``\x81\x01\x93\x90\x93R`\x80\x80\x84\x01\x92\x90\x92R0\x83\x82\x01\x81\x90R\x86Q\x80\x85\x03\x90\x92\x01\x82R`\xC0\x93\x84\x01\x90\x96R\x80Q\x94\x01\x93\x90\x93 \x90\x92R\x91\x90Ra\x01 R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16a\x01@R\x92\x84\x16a\x01`R\x90\x83\x16a\x01\xA0R\x82\x16a\x01\x80R\x16a\x01\xC0Rb\0\x01\xEDV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01jW`\0\x80\xFD[PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15b\0\x01\x86W`\0\x80\xFD[\x85Qb\0\x01\x93\x81b\0\x01TV[` \x87\x01Q\x90\x95Pb\0\x01\xA6\x81b\0\x01TV[`@\x87\x01Q\x90\x94Pb\0\x01\xB9\x81b\0\x01TV[``\x87\x01Q\x90\x93Pb\0\x01\xCC\x81b\0\x01TV[`\x80\x87\x01Q\x90\x92Pb\0\x01\xDF\x81b\0\x01TV[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x01@Qa\x01`Qa\x01\x80Qa\x01\xA0Qa\x01\xC0QaPyb\0\x02\xE4`\09`\0\x81\x81a\x05\x9F\x01R\x81\x81a\x18\xCB\x01R\x81\x81a'\x92\x01Ra1r\x01R`\0\x81\x81a\x05?\x01R\x81\x81a\r\xCE\x01R\x81\x81a\x0E\xB5\x01R\x81\x81a\x0F&\x01R\x81\x81a\x18U\x01R\x81\x81a'\r\x01Ra0\xF2\x01R`\0\x81\x81a\x04C\x01R\x81\x81a\x12\\\x01R\x81\x81a\x18\x94\x01R\x81\x81a&u\x01Ra0r\x01R`\0\x81\x81a\x04j\x01R\x81\x81a\x12\xD1\x01R\x81\x81a\x13\x8C\x01Ra\x1B\xF4\x01R`\0a\x05\xD9\x01R`\0a;\x1C\x01R`\0a;k\x01R`\0a;F\x01R`\0a:\x9F\x01R`\0a:\xC9\x01R`\0a:\xF3\x01RaPy`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02^W`\x005`\xE0\x1C\x80cY\\jg\x11a\x01FW\x80c\xB14Bq\x11a\0\xC3W\x80c\xCA\r\xE8\x82\x11a\0\x87W\x80c\xCA\r\xE8\x82\x14a\x06GW\x80c\xD7-\x8D\xD6\x14a\x06nW\x80c\xD9,\xBB\x84\x14a\x06vW\x80c\xE6W\x97\xAD\x14a\x06\xA0W\x80c\xFA\xBC\x1C\xBC\x14a\x07CW\x80c\xFD9\x10Z\x14a\x07VW`\0\x80\xFD[\x80c\xB14Bq\x14a\x05\xD4W\x80c\xB7~\xC1\xD3\x14a\x05\xFBW\x80c\xB84z\xCC\x14a\x06\x0EW\x80c\xC6j\xB9\xCA\x14a\x06!W\x80c\xC8\x1B\x1F\xF4\x14a\x064W`\0\x80\xFD[\x80ch0H5\x11a\x01\nW\x80ch0H5\x14a\x05:W\x80c\x85\x02\rI\x14a\x05aW\x80c\x88o\x11\x95\x14a\x05\x81W\x80c\x9E\x99#\xC2\x14a\x05\x9AW\x80c\xAB&\x9B-\x14a\x05\xC1W`\0\x80\xFD[\x80cY\\jg\x14a\x04\xE5W\x80cZ\xC8j\xB7\x14a\x04\xEDW\x80c[\x0B\x82\x9F\x14a\x05\x0CW\x80c\\\x97Z\xBB\x14a\x05\x1FW\x80ccG\xC9\0\x14a\x05'W`\0\x80\xFD[\x80c)k\xB0d\x11a\x01\xDFW\x80c5a\xDE\xB1\x11a\x01\xA3W\x80c5a\xDE\xB1\x14a\x04>W\x80c9\x98\xFD\xD3\x14a\x04eW\x80cEZC\xFC\x14a\x04\x8CW\x80cKv\xB9\xD5\x14a\x04\x9FW\x80cRn\xA9N\x14a\x04\xB2W\x80cXe\xC6\x0C\x14a\x04\xC5W`\0\x80\xFD[\x80c)k\xB0d\x14a\x03\xC7W\x80c)\xD1\xE0\xC3\x14a\x03\xDAW\x80c,\xDD\x1E\x86\x14a\x03\xEDW\x80c0db\r\x14a\x04\0W\x80c41\xAF%\x14a\x04+W`\0\x80\xFD[\x80c\x13T*N\x11a\x02&W\x80c\x13T*N\x14a\x032W\x80c\x13d9\xDD\x14a\x03[W\x80c\x14x\x85\x1F\x14a\x03nW\x80c%\0uv\x14a\x03\xA1W\x80c(\xF6\x1B1\x14a\x03\xB4W`\0\x80\xFD[\x80c\x01Y\xF1\xCE\x14a\x02cW\x80c\x05C\x10\xE6\x14a\x02\xB1W\x80c\x05Zb\xB6\x14a\x02\xDCW\x80c\x0C\xF4\xB7g\x14a\x03\nW\x80c\x10\xD6z/\x14a\x03\x1FW[`\0\x80\xFD[a\x02va\x02q6`\x04a?&V[a\x07\x92V[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x80\x85\x01Q\x90\x91\x16\x90\x82\x01R\x91\x81\x01Q`\x01`\x01`\xC0\x1B\x03\x16\x90\x82\x01R``\x01[`@Q\x80\x91\x03\x90\xF3[`7Ta\x02\xC4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xA8V[a\x02\xFCa\x02\xEA6`\x04a?HV[`\0\x90\x81R`3` R`@\x90 T\x90V[`@Q\x90\x81R` \x01a\x02\xA8V[a\x03\x1Da\x03\x186`\x04a@MV[a\x08#V[\0[a\x03\x1Da\x03-6`\x04a@\xA6V[a\t$V[a\x02\xFCa\x03@6`\x04a@\xA6V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`4` R`@\x90 T\x90V[a\x03\x1Da\x03i6`\x04a?HV[a\t\xD7V[a\x03\x91a\x03|6`\x04a?HV[`5` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02\xA8V[a\x03\x1Da\x03\xAF6`\x04aA\xC1V[a\x0B\x14V[`8Ta\x02\xC4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02\xC4a\x03\xD56`\x04a?HV[a\x12CV[a\x03\x1Da\x03\xE86`\x04a@\xA6V[a\x12\xCFV[a\x03\x1Da\x03\xFB6`\x04a@\xA6V[a\x13\x8AV[a\x04\x13a\x04\x0E6`\x04aB\xD5V[a\x14EV[`@Q`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xA8V[a\x04\x13a\x0496`\x04a?HV[a\x16FV[a\x02\xC4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xC4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\x1Da\x04\x9A6`\x04aC\rV[a\x16\xFCV[a\x03\x1Da\x04\xAD6`\x04aC\xE9V[a\x175V[a\x03\x1Da\x04\xC06`\x04aD\xC6V[a\x1AaV[a\x04\xD8a\x04\xD36`\x04a@\xA6V[a\x1A\xB2V[`@Qa\x02\xA8\x91\x90aEiV[a\x03\x1Da\x1B&V[a\x03\x91a\x04\xFB6`\x04aE\x9CV[`\x01\x80T`\xFF\x90\x92\x16\x1B\x90\x81\x16\x14\x90V[a\x03\x1Da\x05\x1A6`\x04aE\xB7V[a\x1B\xF2V[`\x01Ta\x02\xFCV[a\x02\xC4a\x0556`\x04a?HV[a\x1C\xB2V[a\x02\xC4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x05ta\x05o6`\x04aE\xEBV[a\x1C\xDCV[`@Qa\x02\xA8\x91\x90aF\x90V[`\0Ta\x02\xC4\x90b\x01\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02\xC4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xFCa\x05\xCF6`\x04aG!V[a \x17V[a\x02\xC4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\x1Da\x06\t6`\x04aG\xD5V[a {V[a\x03\x1Da\x06\x1C6`\x04aH\x11V[a!tV[a\x03\x1Da\x06/6`\x04aHxV[a!\xFFV[a\x03\x1Da\x06B6`\x04aD\xC6V[a\"jV[a\x02\xFC\x7F\xAEc\xAA\x13\x18u\x1Fd\xEF\x85\x0E\x04T\x17\xF8h\x11\xEA\xEC\xEE\x1C\xEB\xC2&\x90M#{\x80pM\x92\x81V[`6Ta\x02\xFCV[a\x03\x1Da\x06\x846`\x04aH\xFCV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\0\x90\x81R`4` R`@\x90 UV[a\x07\x0Fa\x06\xAE6`\x04aE\x9CV[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x92\x84\x01\x81\x90R`\xFF\x94\x90\x94\x16\x84R`2\x82R\x92\x82\x90 \x82Q\x93\x84\x01\x83RTc\xFF\xFF\xFF\xFF\x81\x16\x84Ra\xFF\xFF`\x01` \x1B\x82\x04\x81\x16\x92\x85\x01\x92\x90\x92R`\x01`0\x1B\x90\x04\x16\x90\x82\x01R\x90V[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x16\x81R` \x80\x84\x01Qa\xFF\xFF\x90\x81\x16\x91\x83\x01\x91\x90\x91R\x92\x82\x01Q\x90\x92\x16\x90\x82\x01R``\x01a\x02\xA8V[a\x03\x1Da\x07Q6`\x04a?HV[a\"\xADV[a\x07\x85a\x07d6`\x04a@\xA6V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`4` R`@\x90 `\x01\x01T`\xFF\x16\x90V[`@Qa\x02\xA8\x91\x90aI(V[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R`\0\x83\x81R`3` R`@\x90 \x80T\x83\x90\x81\x10a\x07\xCFWa\x07\xCFaI6V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01`\xC0\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90P[\x92\x91PPV[`\x013`\0\x90\x81R`4` R`@\x90 `\x01\x01T`\xFF\x16`\x02\x81\x11\x15a\x08LWa\x08LaE1V[\x14a\x08\xD8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`K`$\x82\x01R\x7FBLSRegistryCoordinatorWithIndici`D\x82\x01R\x7Fes.updateSocket: operator is not`d\x82\x01Rj\x08\x1C\x99Y\xDA\\\xDD\x19\\\x99Y`\xAA\x1B`\x84\x82\x01R`\xA4\x01[`@Q\x80\x91\x03\x90\xFD[3`\0\x90\x81R`4` R`@\x90\x81\x90 T\x90Q\x7F\xEC)c\xAB!\xC1\xE5\x0E\x1EX*\xA5B\xAF.K\xF7\xBF8\xE6\xE1@<'\xB4.\x1C]nb\x1E\xAA\x90a\t\x19\x90\x84\x90aI\xA4V[`@Q\x80\x91\x03\x90\xA2PV[`\0`\x02\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\twW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x9B\x91\x90aI\xB7V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\t\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xCF\x90aI\xD4V[a\t\xD4\x81a$\tV[PV[`\0T`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01Rb\x01\0\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n$W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nH\x91\x90aJ\x1EV[a\ndW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xCF\x90aJ@V[`\x01T\x81\x81\x16\x14a\n\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08\xCFV[`\x01\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01a\t\x19V[`\x01\x80T`\0\x91\x90\x81\x16\x14\x15a\x0B<W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xCF\x90aJ\x88V[`\0a\x0B\x813\x8B\x8B\x8B\x8B\x8B\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa%\x0E\x92PPPV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x91\x92P`\0\x91\x90` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90Pa\x0B\xB0\x89a+aV[\x81`\0\x81Q\x81\x10a\x0B\xC3Wa\x0B\xC3aI6V[` \x02` \x01\x01\x81\x81RPPa\x0CF\x81`\0\x81Q\x81\x10a\x0B\xE5Wa\x0B\xE5aI6V[` \x02` \x01\x01Q\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x0C;Wa\x0C,`\x80\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aJ\xBFV[\x81R` \x01\x90`\x01\x01\x90a\x0C\x0FV[PPPPP\x86a+\xA4V[`\0\x80[\x8B\x81\x10\x15a\x124W`\0\x8D\x8D\x83\x81\x81\x10a\x0CfWa\x0CfaI6V[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`2` \x90\x81R`@\x91\x82\x90 \x82Q``\x81\x01\x84R\x90Tc\xFF\xFF\xFF\xFF\x81\x16\x80\x83Ra\xFF\xFF`\x01` \x1B\x83\x04\x81\x16\x94\x84\x01\x94\x90\x94R`\x01`0\x1B\x90\x91\x04\x90\x92\x16\x92\x81\x01\x92\x90\x92R\x88Q\x92\x94P\x90\x92P\x90\x87\x90\x85\x90\x81\x10a\x0C\xD4Wa\x0C\xD4aI6V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11a\x0C\xEEWPPa\x12\"V[\x81`\xFF\x16\x8A\x8A\x86\x81\x81\x10a\r\x04Wa\r\x04aI6V[a\r\x1A\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91PaE\x9CV[`\xFF\x16\x14a\r\xB3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`f`$\x82\x01R`\0\x80Q` aP$\x839\x81Q\x91R`D\x82\x01R\x7Fs.registerOperatorWithCoordinato`d\x82\x01R\x7Fr: quorumNumber not the same as `\x84\x82\x01Re\x1C\xDAY\xDB\x99Y`\xD2\x1B`\xA4\x82\x01R`\xC4\x01a\x08\xCFV[`@Qc\x0C\x8Fs\x9D`\xE4\x1B\x81R`\xFF\x83\x16`\x04\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xC8\xF79\xD0\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\x1DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0EA\x91\x90aJ\xDBV[\x90P`\0`4`\0\x8D\x8D\x88\x81\x81\x10a\x0E[Wa\x0E[aI6V[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\x0Es\x91\x90a@\xA6V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x82\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 T\x91Qc\x1A\xADN5`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`\xFF\x88\x16`$\x82\x01R\x91\x93P\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cj\xB58\xD4\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xFCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F \x91\x90aJ\xDBV[\x90P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cj\xB58\xD4\x8A`\0\x81Q\x81\x10a\x0FfWa\x0FfaI6V[` \x02` \x01\x01Q\x88`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0F\x98\x92\x91\x90\x91\x82R`\xFF\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xB5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xD9\x91\x90aJ\xDBV[` \x86\x01Q\x90\x91Pa'\x10\x90a\x0F\xF3\x90a\xFF\xFF\x16\x84aK\x1AV[a\x0F\xFD\x91\x90aKIV[`\x01`\x01``\x1B\x03\x16\x81`\x01`\x01``\x1B\x03\x16\x11a\x10\xBDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`}`$\x82\x01R`\0\x80Q` aP$\x839\x81Q\x91R`D\x82\x01R\x7Fs.registerOperatorWithCoordinato`d\x82\x01R\x7Fr: registering operator has less`\x84\x82\x01R\x7F than kickBIPsOfOperatorStake\0\0\0`\xA4\x82\x01R`\xC4\x01a\x08\xCFV[`@\x85\x01Qa'\x10\x90a\x10\xD4\x90a\xFF\xFF\x16\x86aK\x1AV[a\x10\xDE\x91\x90aKIV[`\x01`\x01``\x1B\x03\x16\x82`\x01`\x01``\x1B\x03\x16\x10a\x11\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`v`$\x82\x01R`\0\x80Q` aP$\x839\x81Q\x91R`D\x82\x01R\x7Fs.registerOperatorWithCoordinato`d\x82\x01R\x7Fr: operator to kick has more tha`\x84\x82\x01Run kickBIPSOfTotalStake`P\x1B`\xA4\x82\x01R`\xC4\x01a\x08\xCFV[\x87a\x11\xA1\x81aK}V[\x98PPPPPPPPa\x12\"\x88\x88\x83\x81\x81\x10a\x11\xBFWa\x11\xBFaI6V[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\x11\xD7\x91\x90a@\xA6V[\x8E\x83\x8Fa\x11\xE5\x82`\x01aK\x98V[\x92a\x11\xF2\x93\x92\x91\x90aK\xB0V[\x8B\x8B\x86\x81\x81\x10a\x12\x04Wa\x12\x04aI6V[\x90P`\x80\x02\x01`@\x01\x806\x03\x81\x01\x90a\x12\x1D\x91\x90aK\xDAV[a-qV[\x80a\x12,\x81aK}V[\x91PPa\x0CJV[PPPPPPPPPPPPPV[`@Qc\x08\xF6b\x9D`\xE3\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cG\xB3\x14\xE8\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xABW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x1D\x91\x90aI\xB7V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13-W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13Q\x91\x90aI\xB7V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x13\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xCF\x90aK\xF6V[a\t\xD4\x81a3\x1EV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xE8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\x0C\x91\x90aI\xB7V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x14<W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xCF\x90aK\xF6V[a\t\xD4\x81a3\x87V[`\0\x83\x81R`3` R`@\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x14hWa\x14haI6V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x85R`\x01` \x1B\x83\x04\x82\x16\x95\x85\x01\x95\x90\x95R`\x01`@\x1B\x90\x91\x04`\x01`\x01`\xC0\x1B\x03\x16\x91\x83\x01\x91\x90\x91R\x90\x92P\x85\x16\x10\x15a\x15hW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x7F`$\x82\x01R`\0\x80Q` aP$\x839\x81Q\x91R`D\x82\x01R\x7Fs.getQuorumBitmapByOperatorIdAtB`d\x82\x01R\x7FlockNumberByIndex: quorumBitmapU`\x84\x82\x01R\x7Fpdate is from after blockNumber\0`\xA4\x82\x01R`\xC4\x01a\x08\xCFV[\x83c\xFF\xFF\xFF\xFF\x16\x81` \x01Qc\xFF\xFF\xFF\xFF\x16\x11\x80a\x15\x8EWP` \x81\x01Qc\xFF\xFF\xFF\xFF\x16\x15[a\x16:W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x80`$\x82\x01R`\0\x80Q` aP$\x839\x81Q\x91R`D\x82\x01R\x7Fs.getQuorumBitmapByOperatorIdAtB`d\x82\x01R\x7FlockNumberByIndex: quorumBitmapU`\x84\x82\x01R\x7Fpdate is from before blockNumber`\xA4\x82\x01R`\xC4\x01a\x08\xCFV[`@\x01Q\x94\x93PPPPV[`\0\x81\x81R`3` R`@\x81 T\x80\x15\x80a\x16\xA2WP`\0\x83\x81R`3` R`@\x90 a\x16v`\x01\x83aLrV[\x81T\x81\x10a\x16\x86Wa\x16\x86aI6V[`\0\x91\x82R` \x90\x91 \x01T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15\x15[\x15a\x16\xB0WP`\0\x92\x91PPV[`\0\x83\x81R`3` R`@\x90 a\x16\xC9`\x01\x83aLrV[\x81T\x81\x10a\x16\xD9Wa\x16\xD9aI6V[`\0\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01`\xC0\x1B\x03\x16\x93\x92PPPV[`\x01\x80T`\x02\x90\x81\x16\x14\x15a\x17#W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xCF\x90aJ\x88V[a\x17/3\x85\x85\x85a-qV[PPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x17UWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x17oWP0;\x15\x80\x15a\x17oWP`\0T`\xFF\x16`\x01\x14[a\x17\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x08\xCFV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x17\xF5W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x17\xFF\x83\x83a3\xF0V[a\x18\x08\x86a3\x1EV[a\x18\x11\x85a3\x87V[`6\x80T`\x01\x80\x82\x01\x83U`\0\x83\x90R\x7FJ\x11\xF9N \xA9<y\xF6\xECt:\x19T\xECO\xC2\xC0\x84)\xAE!\"\x11\x8B\xF24\xB2\x18\\\x81\xB8\x91\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x82\x17\x90\x93U\x85T\x80\x85\x01\x87U\x85\x01\x80T\x83\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x16\x17\x90U\x85T\x93\x84\x01\x90\x95U\x91\x90\x92\x01\x80T\x90\x91\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x16\x91\x90\x91\x17\x90U\x84Q`@\x80Qc\x9A\xA1e=`\xE0\x1B\x81R\x90Q\x91\x92\x91c\x9A\xA1e=\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x193W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19W\x91\x90aL\x89V[a\xFF\xFF\x16\x14a\x19\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`F`$\x82\x01R`\0\x80Q` aP$\x839\x81Q\x91R`D\x82\x01R\x7Fs: operator set params length mi`d\x82\x01Re\x0Em\xAC.\x8Cm`\xD3\x1B`\x84\x82\x01R`\xA4\x01a\x08\xCFV[`\0[\x84Q\x81`\xFF\x16\x10\x15a\x1A\x12Wa\x1A\0\x81\x86\x83`\xFF\x16\x81Q\x81\x10a\x19\xF3Wa\x19\xF3aI6V[` \x02` \x01\x01Qa4\xDCV[\x80a\x1A\n\x81aL\xA6V[\x91PPa\x19\xCEV[P\x80\x15a\x1AYW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPV[`\x01\x80T`\0\x91\x90\x81\x16\x14\x15a\x1A\x89W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xCF\x90aJ\x88V[`\0\x80a\x1A\x98\x84\x86\x01\x86aL\xC6V[\x91P\x91Pa\x1A\xA93\x88\x88\x85\x85a5\x89V[PPPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`4` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x81\x01T\x90\x91\x83\x01\x90`\xFF\x16`\x02\x81\x11\x15a\x1B\x0CWa\x1B\x0CaE1V[`\x02\x81\x11\x15a\x1B\x1DWa\x1B\x1DaE1V[\x90RP\x92\x91PPV[`\0T`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01Rb\x01\0\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1BsW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\x97\x91\x90aJ\x1EV[a\x1B\xB3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xCF\x90aJ@V[`\0\x19`\x01\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1CPW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Ct\x91\x90aI\xB7V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1C\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xCF\x90aK\xF6V[a\x1C\xAE\x82\x82a4\xDCV[PPV[`6\x81\x81T\x81\x10a\x1C\xC2W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x81V[```\0\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1C\xF9Wa\x1C\xF9a?aV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1D\"W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a \x0FW`\0`3`\0\x86\x84\x81Q\x81\x10a\x1DIWa\x1DIaI6V[` \x02` \x01\x01Q\x81R` \x01\x90\x81R` \x01`\0 \x80T\x90P\x90P`\0[\x81c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x1F\xFAW\x86c\xFF\xFF\xFF\xFF\x16`3`\0\x88\x86\x81Q\x81\x10a\x1D\x99Wa\x1D\x99aI6V[` \x02` \x01\x01Q\x81R` \x01\x90\x81R` \x01`\0 `\x01\x83\x85a\x1D\xBD\x91\x90aM\x14V[a\x1D\xC7\x91\x90aM\x14V[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x1D\xDDWa\x1D\xDDaI6V[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a\x1F\xE8W`3`\0\x87\x85\x81Q\x81\x10a\x1E\nWa\x1E\naI6V[` \x02` \x01\x01Q\x81R` \x01\x90\x81R` \x01`\0 `\x01\x82\x84a\x1E.\x91\x90aM\x14V[a\x1E8\x91\x90aM\x14V[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x1ENWa\x1ENaI6V[`\0\x91\x82R` \x90\x91 \x01T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15\x80a\x1E\xEAWP\x86c\xFF\xFF\xFF\xFF\x16`3`\0\x88\x86\x81Q\x81\x10a\x1E\x8BWa\x1E\x8BaI6V[` \x02` \x01\x01Q\x81R` \x01\x90\x81R` \x01`\0 `\x01\x83\x85a\x1E\xAF\x91\x90aM\x14V[a\x1E\xB9\x91\x90aM\x14V[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x1E\xCFWa\x1E\xCFaI6V[`\0\x91\x82R` \x90\x91 \x01T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x11[a\x1F\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x82`$\x82\x01R`\0\x80Q` aP$\x839\x81Q\x91R`D\x82\x01R\x7Fs.getQuorumBitmapIndicesByOperat`d\x82\x01R\x7ForIdsAtBlockNumber: operatorId h`\x84\x82\x01R\x7Fas no quorumBitmaps at blockNumb`\xA4\x82\x01Ra2\xB9`\xF1\x1B`\xC4\x82\x01R`\xE4\x01a\x08\xCFV[`\x01a\x1F\xAD\x82\x84aM\x14V[a\x1F\xB7\x91\x90aM\x14V[\x84\x84\x81Q\x81\x10a\x1F\xC9Wa\x1F\xC9aI6V[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPPa\x1F\xFAV[\x80a\x1F\xF2\x81aM9V[\x91PPa\x1DhV[PP\x80\x80a \x07\x90aK}V[\x91PPa\x1D(V[P\x93\x92PPPV[`\0a p\x7F\xAEc\xAA\x13\x18u\x1Fd\xEF\x85\x0E\x04T\x17\xF8h\x11\xEA\xEC\xEE\x1C\xEB\xC2&\x90M#{\x80pM\x92\x86\x86\x86\x86`@Q` \x01a U\x95\x94\x93\x92\x91\x90aM]V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a6\xA0V[\x90P[\x94\x93PPPPV[`\0\x82\x81R`3` R`@\x90 T\x80\x15a \xE7W`\0\x83\x81R`3` R`@\x90 C\x90a \xAB`\x01\x84aLrV[\x81T\x81\x10a \xBBWa \xBBaI6V[\x90`\0R` `\0 \x01`\0\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP[P`\0\x91\x82R`3` \x90\x81R`@\x80\x84 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x87\x81R`\x01`\x01`\xC0\x1B\x03\x96\x87\x16\x94\x83\x01\x94\x85R\x83T`\x01\x81\x01\x85U\x93\x88R\x94\x90\x96 \x90Q\x91\x01\x80T\x93Q\x92Q\x90\x94\x16`\x01`@\x1B\x02`\x01`\x01`@\x1B\x03\x92\x86\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x94\x16\x91\x90\x95\x16\x17\x91\x90\x91\x17\x16\x91\x90\x91\x17\x90UV[`8T`\x01`\x01`\xA0\x1B\x03\x163\x14a!\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`H`$\x82\x01R`\0\x80Q` aP$\x839\x81Q\x91R`D\x82\x01R\x7Fs.onlyEjector: caller is not the`d\x82\x01Rg\x102\xB52\xB1\xBA7\xB9`\xC1\x1B`\x84\x82\x01R`\xA4\x01a\x08\xCFV[a\x17/\x84\x84\x84\x84a-qV[`\x01\x80T`\0\x91\x90\x81\x16\x14\x15a\"'W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xCF\x90aJ\x88V[a\x1AY3\x87\x87\x87\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa5\x89\x92PPPV[`\x01\x80T`\x02\x90\x81\x16\x14\x15a\"\x91W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xCF\x90aJ\x88V[`\0a\"\x9F\x83\x85\x01\x85aK\xDAV[\x90Pa\x1AY3\x87\x87\x84a-qV[`\0`\x02\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a#\0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#$\x91\x90aI\xB7V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a#TW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xCF\x90aI\xD4V[`\x01T\x19\x81\x19`\x01T\x19\x16\x14a#\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08\xCFV[`\x01\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\t\x19V[`\x01`\x01`\xA0\x1B\x03\x81\x16a$\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x08\xCFV[`\0T`@\x80Q`\x01`\x01`\xA0\x1B\x03b\x01\0\0\x90\x93\x04\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`\0\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16b\x01\0\0\x02b\x01\0\0`\x01`\xB0\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[```\0a%Q\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa6\xEE\x92PPPV[\x90P`\x01`\x01`\xC0\x1B\x03\x81\x11\x15a%\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`k`$\x82\x01R`\0\x80Q` aP$\x839\x81Q\x91R`D\x82\x01R`\0\x80Q` aP\x04\x839\x81Q\x91R`d\x82\x01R\x7For: quorumBitmap exceeds of max `\x84\x82\x01Rjbitmap size`\xA8\x1B`\xA4\x82\x01R`\xC4\x01a\x08\xCFV[\x80a&[W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R`\0\x80Q` aP$\x839\x81Q\x91R`D\x82\x01R`\0\x80Q` aP\x04\x839\x81Q\x91R`d\x82\x01R\x7For: quorumBitmap cannot be 0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x08\xCFV[`@Qc\x03\xCEK\xAD`\xE0\x1B\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x03\xCEK\xAD\x90a&\xB0\x90\x8B\x90\x8B\x90\x8B\x90\x8B\x90`\x04\x01aN\x1AV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a&\xCFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\xF3\x91\x90aNWV[`@Qc%PGw`\xE0\x1B\x81R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c%PGw\x90a'H\x90\x8B\x90\x85\x90\x8C\x90\x8C\x90`\x04\x01aNpV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a'bW`\0\x80\xFD[PZ\xF1\x15\x80\x15a'vW=`\0\x80>=`\0\xFD[PP`@Qb\xBF\xF0M`\xE0\x1B\x81R`\0\x92P`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91Pb\xBF\xF0M\x90a'\xCB\x90\x85\x90\x8C\x90\x8C\x90`\x04\x01aN\xA2V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a'\xEAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra(\x12\x91\x90\x81\x01\x90aN\xBCV[`\0\x83\x81R`3` R`@\x90 T\x90\x91P\x80\x15\x80\x15\x90a(rWP`\0\x83\x81R`3` R`@\x90 a(G`\x01\x83aLrV[\x81T\x81\x10a(WWa(WaI6V[`\0\x91\x82R` \x90\x91 \x01T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15[\x15a)\xC9W`\0\x83\x81R`3` R`@\x81 a(\x90`\x01\x84aLrV[\x81T\x81\x10a(\xA0Wa(\xA0aI6V[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xC0\x1B\x03`\x01`@\x1B\x90\x91\x04\x81\x81\x16\x92P\x86\x16\x16\x15a)mW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x85`$\x82\x01R`\0\x80Q` aP$\x839\x81Q\x91R`D\x82\x01R`\0\x80Q` aP\x04\x839\x81Q\x91R`d\x82\x01R\x7For: operator already registered `\x84\x82\x01R\x7Ffor some quorums being registere`\xA4\x82\x01Rd2\x1037\xB9`\xD9\x1B`\xC4\x82\x01R`\xE4\x01a\x08\xCFV[`\0\x84\x81R`3` R`@\x90 \x94\x81\x17\x94C\x90a)\x8C`\x01\x85aLrV[\x81T\x81\x10a)\x9CWa)\x9CaI6V[\x90`\0R` `\0 \x01`\0\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPP[`\0\x83\x81R`3` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x86\x81R`\x01`\x01`\xC0\x1B\x03\x8B\x81\x16\x84\x87\x01\x90\x81R\x85T`\x01\x80\x82\x01\x88U\x96\x8AR\x88\x8A \x95Q\x95\x01\x80T\x93Q\x91Q\x90\x92\x16`\x01`@\x1B\x02`\x01`\x01`@\x1B\x03\x91\x85\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x94\x16\x95\x90\x94\x16\x94\x90\x94\x17\x91\x90\x91\x17\x92\x90\x92\x16\x17\x90U`\x01`\x01`\xA0\x1B\x03\x8E\x16\x84R`4\x90\x92R\x90\x91 \x81\x01T`\xFF\x16`\x02\x81\x11\x15a*\x81Wa*\x81aE1V[\x14a+\x1AW`@\x80Q\x80\x82\x01\x82R\x84\x81R`\x01` \x80\x83\x01\x82\x81R`\x01`\x01`\xA0\x1B\x03\x8F\x16`\0\x90\x81R`4\x90\x92R\x93\x90 \x82Q\x81U\x92Q\x83\x82\x01\x80T\x93\x94\x93\x91\x92\x90\x91`\xFF\x19\x16\x90\x83`\x02\x81\x11\x15a*\xDCWa*\xDCaE1V[\x02\x17\x90UPP`@Q\x84\x91P`\x01`\x01`\xA0\x1B\x03\x8C\x16\x90\x7F\xE8\xE6\x8C\xEF\x1C:v\x1E\xD7\xBE~\x84c\xA3u\xF2\x7F{\xC35\xE5\x18$\"<\xAC\xCEcn\xC5\xC3\xFE\x90`\0\x90\xA3[\x82\x7F\xEC)c\xAB!\xC1\xE5\x0E\x1EX*\xA5B\xAF.K\xF7\xBF8\xE6\xE1@<'\xB4.\x1C]nb\x1E\xAA\x87`@Qa+J\x91\x90aI\xA4V[`@Q\x80\x91\x03\x90\xA2P\x92PPP[\x95\x94PPPPPV[`\0\x81`\0\x01Q\x82` \x01Q`@Q` \x01a+\x87\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[` \x80\x82\x01Q`\0\x90\x81R`5\x90\x91R`@\x90 T`\xFF\x16\x15a,iW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`w`$\x82\x01R`\0\x80Q` aP$\x839\x81Q\x91R`D\x82\x01R\x7Fs._verifyChurnApproverSignatureO`d\x82\x01R\x7FnOperatorChurnApproval: churnApp`\x84\x82\x01R\x7Frover salt already used\0\0\0\0\0\0\0\0\0`\xA4\x82\x01R`\xC4\x01a\x08\xCFV[B\x81`@\x01Q\x10\x15a-\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`w`$\x82\x01R`\0\x80Q` aP$\x839\x81Q\x91R`D\x82\x01R\x7Fs._verifyChurnApproverSignatureO`d\x82\x01R\x7FnOperatorChurnApproval: churnApp`\x84\x82\x01R\x7Frover signature expired\0\0\0\0\0\0\0\0\0`\xA4\x82\x01R`\xC4\x01a\x08\xCFV[` \x80\x82\x01\x80Q`\0\x90\x81R`5\x90\x92R`@\x91\x82\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U`7T\x90Q\x91\x83\x01Qa-l\x92`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91a-e\x91\x87\x91\x87\x91a \x17V[\x83Qa8{V[PPPV[`\x01`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`4` R`@\x90 `\x01\x01T`\xFF\x16`\x02\x81\x11\x15a-\xA3Wa-\xA3aE1V[\x14a.*W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R```$\x82\x01R`\0\x80Q` aP$\x839\x81Q\x91R`D\x82\x01R\x7Fs._deregisterOperatorWithCoordin`d\x82\x01R\x7Fator: operator is not registered`\x84\x82\x01R`\xA4\x01a\x08\xCFV[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`4` R`@\x90 Ta.L\x82a+aV[\x81\x14a.\xE8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`k`$\x82\x01R`\0\x80Q` aP$\x839\x81Q\x91R`D\x82\x01R\x7Fs._deregisterOperatorWithCoordin`d\x82\x01R\x7Fator: operatorId does not match `\x84\x82\x01Rj\x0E\x0E\xACMl\xAF$\r\x0C.m`\xAB\x1B`\xA4\x82\x01R`\xC4\x01a\x08\xCFV[`\0a/)\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa6\xEE\x92PPPV[`\0\x83\x81R`3` R`@\x81 T\x91\x92P\x90a/H\x90`\x01\x90aLrV[`\0\x84\x81R`3` R`@\x81 \x80T\x92\x93P\x90\x91\x83\x90\x81\x10a/mWa/maI6V[`\0\x91\x82R` \x82 \x01T`\x01`@\x1B\x90\x04\x93\x84\x16`\x01`\x01`\xC0\x1B\x03\x90\x81\x16\x94\x16\x91Pa/\x9A\x84a:5V[\x90P\x80Q`\0\x14\x15a0NW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x80`$\x82\x01R`\0\x80Q` aP$\x839\x81Q\x91R`D\x82\x01R\x7Fs._deregisterOperatorWithCoordin`d\x82\x01R\x7Fator: operator is not registered`\x84\x82\x01R\x7F for any of the provided quorums`\xA4\x82\x01R`\xC4\x01a\x08\xCFV[`@Qc\x12\x1BM\x95`\xE1\x1B\x81R`\x01`\x01`\xC0\x1B\x03\x83\x16\x85\x14\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c$6\x9B*\x90a0\xAB\x90\x8D\x90\x86\x90\x8C\x90`\x04\x01aOUV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a0\xC5W`\0\x80\xFD[PZ\xF1\x15\x80\x15a0\xD9W=`\0\x80>=`\0\xFD[PP`@Qc\xBD)\xB8\xCD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92Pc\xBD)\xB8\xCD\x91Pa1+\x90\x89\x90\x86\x90`\x04\x01aO\x91V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a1EW`\0\x80\xFD[PZ\xF1\x15\x80\x15a1YW=`\0\x80>=`\0\xFD[PP`@Qc\xBD)\xB8\xCD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92Pc\xBD)\xB8\xCD\x91Pa1\xAB\x90\x89\x90\x86\x90`\x04\x01aO\x91V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a1\xC5W`\0\x80\xFD[PZ\xF1\x15\x80\x15a1\xD9W=`\0\x80>=`\0\xFD[PPP`\0\x87\x81R`3` R`@\x90 \x80TC\x92P\x86\x90\x81\x10a1\xFFWa1\xFFaI6V[\x90`\0R` `\0 \x01`\0\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80a2\xC3W`\0\x86\x81R`3` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x86\x81R`\x01`\x01`\xC0\x1B\x03\x8C\x19\x8B\x16\x81\x16\x95\x84\x01\x95\x86R\x84T`\x01\x81\x01\x86U\x94\x88R\x95\x90\x96 \x91Q\x91\x90\x92\x01\x80T\x95Q\x93Q\x90\x94\x16`\x01`@\x1B\x02`\x01`\x01`@\x1B\x03\x93\x83\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x96\x16\x91\x90\x92\x16\x17\x93\x90\x93\x17\x16\x91\x90\x91\x17\x90Ua3\x12V[`\x01`\x01`\xA0\x1B\x03\x8A\x16`\0\x81\x81R`4` R`@\x80\x82 `\x01\x01\x80T`\xFF\x19\x16`\x02\x17\x90UQ\x88\x92\x91\x7F9o\xDC\xB1\x80\xCB\x0F\xEA&\x92\x81\x13\xFB\x0F\xD1\xC3T\x98c\xF9\xCDV>j\x18O\x1DW\x81\x16\xC8\xE4\x91\xA3[PPPPPPPPPPV[`7T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F1TW\xD8\xA8\xFE`\xF0J\xF1|\x16\xE2\xF5\xA5\xE1\xDBa+1d\x8EX\x03\x03`u\x9E\xF8\xF3R\x8C\x91\x01`@Q\x80\x91\x03\x90\xA1`7\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`8T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F\x8F0\xAB\t\xF4:l\x15}\x7F\xCE~\n\x13\xC0\x03\x04,\x1C\x95\xE8\xA7.z\x14j!\xC0\xCA\xA2M\xC9\x91\x01`@Q\x80\x91\x03\x90\xA1`8\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0Tb\x01\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a4\x17WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a4\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x08\xCFV[`\x01\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a\x1C\xAE\x82a$\tV[`\xFF\x82\x16`\0\x81\x81R`2` \x90\x81R`@\x91\x82\x90 \x84Q\x81T\x86\x84\x01\x80Q\x88\x87\x01\x80Qc\xFF\xFF\xFF\xFF\x90\x95\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x94\x16\x84\x17`\x01` \x1Ba\xFF\xFF\x93\x84\x16\x02\x17g\xFF\xFF\0\0\0\0\0\0\x19\x16`\x01`0\x1B\x95\x83\x16\x95\x90\x95\x02\x94\x90\x94\x17\x90\x94U\x85Q\x91\x82RQ\x83\x16\x93\x81\x01\x93\x90\x93RQ\x16\x91\x81\x01\x91\x90\x91R\x7F>\xE6\xFE\x8DTa\x02D\xC3\xE9\xD3\xC0f\xAEJ\xEE\x99x\x84\xAA(\xF1\x06\x16\xAE\x82\x19%@\x13\x18\xAC\x90``\x01`@Q\x80\x91\x03\x90\xA2PPV[`\0a5\x98\x86\x86\x86\x86\x86a%\x0EV[\x90P`\0[\x81Q\x81\x10\x15a\x1A\xA9W`2`\0\x87\x87\x84\x81\x81\x10a5\xBCWa5\xBCaI6V[\x91\x90\x91\x015`\xF8\x1C\x82RP` \x81\x01\x91\x90\x91R`@\x01`\0 T\x82Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x90\x83\x90\x83\x90\x81\x10a5\xF3Wa5\xF3aI6V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a6\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`n`$\x82\x01R`\0\x80Q` aP$\x839\x81Q\x91R`D\x82\x01R`\0\x80Q` aP\x04\x839\x81Q\x91R`d\x82\x01R\x7ForAndNoOverfilledQuorums: quorum`\x84\x82\x01Rm\x08\x1A\\\xC8\x1B\xDD\x99\\\x99\x9A[\x1B\x19Y`\x92\x1B`\xA4\x82\x01R`\xC4\x01a\x08\xCFV[\x80a6\x98\x81aK}V[\x91PPa5\x9DV[`\0a\x08\x1Da6\xADa:\x92V[\x83`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x83\x90R`B\x81\x01\x82\x90R`\0\x90`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[`\0a\x01\0\x82Q\x11\x15a7wW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FBitmapUtils.orderedBytesArrayToB\x90\x82\x01R\x7Fitmap: orderedBytesArray is too `d\x82\x01Rclong`\xE0\x1B`\x84\x82\x01R`\xA4\x01a\x08\xCFV[\x81Qa7\x85WP`\0\x91\x90PV[`\0\x80\x83`\0\x81Q\x81\x10a7\x9BWa7\x9BaI6V[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x81\x90\x1B\x92P[\x84Q\x81\x10\x15a8rW\x84\x81\x81Q\x81\x10a7\xC9Wa7\xC9aI6V[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x1B\x91P\x82\x82\x11a8^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: orderedBytesArray is not `d\x82\x01Rf\x1B\xDC\x99\x19\\\x99Y`\xCA\x1B`\x84\x82\x01R`\xA4\x01a\x08\xCFV[\x91\x81\x17\x91a8k\x81aK}V[\x90Pa7\xAEV[P\x90\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16;\x15a9\x95W`@Qc\x0B\x13]?`\xE1\x1B\x80\x82R\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\x16&\xBA~\x90a8\xBB\x90\x86\x90\x86\x90`\x04\x01aO\x91V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a8\xD8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8\xFC\x91\x90aO\xAAV[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14a-lW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`S`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: ERC1271 signature `d\x82\x01Rr\x1D\x99\\\x9AY\x9AX\xD8]\x1A[\xDB\x88\x19\x98Z[\x19Y`j\x1B`\x84\x82\x01R`\xA4\x01a\x08\xCFV[\x82`\x01`\x01`\xA0\x1B\x03\x16a9\xA9\x83\x83a;\xB9V[`\x01`\x01`\xA0\x1B\x03\x16\x14a-lW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: signature not from`d\x82\x01Rf\x109\xB4\xB3\xB72\xB9`\xC9\x1B`\x84\x82\x01R`\xA4\x01a\x08\xCFV[```\0\x80[a\x01\0\x81\x10\x15a:\x8BW`\x01\x81\x1B\x91P\x83\x82\x16\x15a:{W\x82\x81`\xF8\x1B`@Q` \x01a:i\x92\x91\x90aO\xD4V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92P[a:\x84\x81aK}V[\x90Pa:;V[PP\x91\x90PV[`\x000`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80\x15a:\xEBWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14[\x15a;\x15WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[P`@\x80Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x80\x83\x01\x91\x90\x91R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x84\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x83\x01RF`\x80\x83\x01R0`\xA0\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\xC0\x90\x92\x01\x90\x92R\x80Q\x91\x01 \x90V[`\0\x80`\0a;\xC8\x85\x85a;\xD5V[\x91P\x91Pa \x0F\x81a<EV[`\0\x80\x82Q`A\x14\x15a<\x0CW` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1Aa<\0\x87\x82\x85\x85a>\0V[\x94P\x94PPPPa<>V[\x82Q`@\x14\x15a<6W` \x83\x01Q`@\x84\x01Qa<+\x86\x83\x83a>\xEDV[\x93P\x93PPPa<>V[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0\x81`\x04\x81\x11\x15a<YWa<YaE1V[\x14\x15a<bWPV[`\x01\x81`\x04\x81\x11\x15a<vWa<vaE1V[\x14\x15a<\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x08\xCFV[`\x02\x81`\x04\x81\x11\x15a<\xD8Wa<\xD8aE1V[\x14\x15a=&W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x08\xCFV[`\x03\x81`\x04\x81\x11\x15a=:Wa=:aE1V[\x14\x15a=\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x08\xCFV[`\x04\x81`\x04\x81\x11\x15a=\xA7Wa=\xA7aE1V[\x14\x15a\t\xD4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 'v' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x08\xCFV[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a>7WP`\0\x90P`\x03a>\xE4V[\x84`\xFF\x16`\x1B\x14\x15\x80\x15a>OWP\x84`\xFF\x16`\x1C\x14\x15[\x15a>`WP`\0\x90P`\x04a>\xE4V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a>\xB4W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a>\xDDW`\0`\x01\x92P\x92PPa>\xE4V[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x80`\x01`\x01`\xFF\x1B\x03\x83\x16\x81a?\n`\xFF\x86\x90\x1C`\x1BaK\x98V[\x90Pa?\x18\x87\x82\x88\x85a>\0V[\x93P\x93PPP\x93P\x93\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a?9W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0` \x82\x84\x03\x12\x15a?ZW`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a?\x99Wa?\x99a?aV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a?\xC7Wa?\xC7a?aV[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x83\x11\x15a?\xE8Wa?\xE8a?aV[a?\xFB`\x1F\x84\x01`\x1F\x19\x16` \x01a?\x9FV[\x90P\x82\x81R\x83\x83\x83\x01\x11\x15a@\x0FW`\0\x80\xFD[\x82\x82` \x83\x017`\0` \x84\x83\x01\x01R\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a@7W`\0\x80\xFD[a@F\x83\x835` \x85\x01a?\xCFV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a@_W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a@uW`\0\x80\xFD[a s\x84\x82\x85\x01a@&V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\t\xD4W`\0\x80\xFD[\x805a@\xA1\x81a@\x81V[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a@\xB8W`\0\x80\xFD[\x815a@F\x81a@\x81V[`\0\x80\x83`\x1F\x84\x01\x12a@\xD5W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a@\xECW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a<>W`\0\x80\xFD[`\0`@\x82\x84\x03\x12\x15aA\x16W`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15aA8WaA8a?aV[`@R\x825\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0``\x82\x84\x03\x12\x15aAdW`\0\x80\xFD[aAla?wV[\x90P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aA\x84W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13aA\x95W`\0\x80\xFD[aA\xA4\x84\x825` \x84\x01a?\xCFV[\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\0\x80`\xC0\x89\x8B\x03\x12\x15aA\xDDW`\0\x80\xFD[\x885`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aA\xF4W`\0\x80\xFD[aB\0\x8C\x83\x8D\x01a@\xC3V[\x90\x9AP\x98P\x88\x91PaB\x15\x8C` \x8D\x01aA\x04V[\x97P``\x8B\x015\x91P\x80\x82\x11\x15aB+W`\0\x80\xFD[aB7\x8C\x83\x8D\x01a@\xC3V[\x90\x97P\x95P`\x80\x8B\x015\x91P\x80\x82\x11\x15aBPW`\0\x80\xFD[\x81\x8B\x01\x91P\x8B`\x1F\x83\x01\x12aBdW`\0\x80\xFD[\x815\x81\x81\x11\x15aBsW`\0\x80\xFD[\x8C` \x82`\x07\x1B\x85\x01\x01\x11\x15aB\x88W`\0\x80\xFD[` \x83\x01\x95P\x80\x94PP`\xA0\x8B\x015\x91P\x80\x82\x11\x15aB\xA6W`\0\x80\xFD[PaB\xB3\x8B\x82\x8C\x01aARV[\x91PP\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\t\xD4W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15aB\xEAW`\0\x80\xFD[\x835\x92P` \x84\x015aB\xFC\x81aB\xC3V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15aC\"W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x81\x11\x15aC8W`\0\x80\xFD[aCD\x86\x82\x87\x01a@\xC3V[\x90\x94P\x92PaCX\x90P\x85` \x86\x01aA\x04V[\x90P\x92P\x92P\x92V[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15aCzWaCza?aV[P`\x05\x1B` \x01\x90V[a\xFF\xFF\x81\x16\x81\x14a\t\xD4W`\0\x80\xFD[`\0``\x82\x84\x03\x12\x15aC\xA6W`\0\x80\xFD[aC\xAEa?wV[\x90P\x815aC\xBB\x81aB\xC3V[\x81R` \x82\x015aC\xCB\x81aC\x84V[` \x82\x01R`@\x82\x015aC\xDE\x81aC\x84V[`@\x82\x01R\x92\x91PPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15aD\x01W`\0\x80\xFD[\x855aD\x0C\x81a@\x81V[\x94P` \x86\x81\x015aD\x1D\x81a@\x81V[\x94P`@\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aD8W`\0\x80\xFD[\x87\x01`\x1F\x81\x01\x89\x13aDIW`\0\x80\xFD[\x805aD\\aDW\x82aCaV[a?\x9FV[\x81\x81R``\x91\x82\x02\x83\x01\x84\x01\x91\x84\x82\x01\x91\x90\x8C\x84\x11\x15aD{W`\0\x80\xFD[\x93\x85\x01\x93[\x83\x85\x10\x15aD\xA1WaD\x92\x8D\x86aC\x94V[\x83R\x93\x84\x01\x93\x91\x85\x01\x91aD\x80V[\x81\x98PaD\xAF\x81\x8D\x01a@\x96V[\x9A\x9D\x99\x9CP\x97\x9A`\x80\x015\x98\x97PPPPPPPPV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15aD\xDCW`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aD\xF3W`\0\x80\xFD[aD\xFF\x88\x83\x89\x01a@\xC3V[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15aE\x18W`\0\x80\xFD[PaE%\x87\x82\x88\x01a@\xC3V[\x95\x98\x94\x97P\x95PPPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x03\x81\x10aEeWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[\x81Q\x81R` \x80\x83\x01Q`@\x83\x01\x91aE\x84\x90\x84\x01\x82aEGV[P\x92\x91PPV[\x805`\xFF\x81\x16\x81\x14a@\xA1W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aE\xAEW`\0\x80\xFD[a@F\x82aE\x8BV[`\0\x80`\x80\x83\x85\x03\x12\x15aE\xCAW`\0\x80\xFD[aE\xD3\x83aE\x8BV[\x91PaE\xE2\x84` \x85\x01aC\x94V[\x90P\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15aE\xFEW`\0\x80\xFD[\x825aF\t\x81aB\xC3V[\x91P` \x83\x81\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aF%W`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13aF6W`\0\x80\xFD[\x805aFDaDW\x82aCaV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x88\x83\x11\x15aFcW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aF\x81W\x835\x82R\x92\x84\x01\x92\x90\x84\x01\x90aFhV[\x80\x95PPPPPP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aF\xCEW\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aF\xACV[P\x90\x96\x95PPPPPPV[`\0`\x80\x82\x84\x03\x12\x15aF\xECW`\0\x80\xFD[aF\xF4a?wV[\x90PaF\xFF\x82aE\x8BV[\x81R` \x82\x015aG\x0F\x81a@\x81V[` \x82\x01RaC\xDE\x83`@\x84\x01aA\x04V[`\0\x80`\0\x80`\x80\x80\x86\x88\x03\x12\x15aG8W`\0\x80\xFD[\x855\x94P` \x80\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aGVW`\0\x80\xFD[\x87\x01`\x1F\x81\x01\x89\x13aGgW`\0\x80\xFD[\x805aGuaDW\x82aCaV[\x81\x81R`\x07\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x8B\x83\x11\x15aG\x94W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aG\xBAWaG\xAB\x8C\x85aF\xDAV[\x82R\x92\x85\x01\x92\x90\x84\x01\x90aG\x99V[\x98\x9B\x98\x9APPPP`@\x87\x015\x96``\x015\x95\x94PPPPPV[`\0\x80`@\x83\x85\x03\x12\x15aG\xE8W`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x14aH\x06W`\0\x80\xFD[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aH'W`\0\x80\xFD[\x845aH2\x81a@\x81V[\x93P` \x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aHMW`\0\x80\xFD[aHY\x87\x82\x88\x01a@\xC3V[\x90\x94P\x92PaHm\x90P\x86`@\x87\x01aA\x04V[\x90P\x92\x95\x91\x94P\x92PV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15aH\x90W`\0\x80\xFD[\x855`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aH\xA7W`\0\x80\xFD[aH\xB3\x89\x83\x8A\x01a@\xC3V[\x90\x97P\x95P\x85\x91PaH\xC8\x89` \x8A\x01aA\x04V[\x94P``\x88\x015\x91P\x80\x82\x11\x15aH\xDEW`\0\x80\xFD[PaH\xEB\x88\x82\x89\x01a@\xC3V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15aI\x0FW`\0\x80\xFD[\x825aI\x1A\x81a@\x81V[\x94` \x93\x90\x93\x015\x93PPPV[` \x81\x01a\x08\x1D\x82\x84aEGV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0[\x83\x81\x10\x15aIgW\x81\x81\x01Q\x83\x82\x01R` \x01aIOV[\x83\x81\x11\x15a\x17/WPP`\0\x91\x01RV[`\0\x81Q\x80\x84RaI\x90\x81` \x86\x01` \x86\x01aILV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a@F` \x83\x01\x84aIxV[`\0` \x82\x84\x03\x12\x15aI\xC9W`\0\x80\xFD[\x81Qa@F\x81a@\x81V[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15aJ0W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a@FW`\0\x80\xFD[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`\x19\x90\x82\x01R\x7FPausable: index is paused\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`\0`\x80\x82\x84\x03\x12\x15aJ\xD1W`\0\x80\xFD[a@F\x83\x83aF\xDAV[`\0` \x82\x84\x03\x12\x15aJ\xEDW`\0\x80\xFD[\x81Q`\x01`\x01``\x1B\x03\x81\x16\x81\x14a@FW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aK@WaK@aK\x04V[\x02\x94\x93PPPPV[`\0`\x01`\x01``\x1B\x03\x80\x84\x16\x80aKqWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[\x92\x16\x91\x90\x91\x04\x92\x91PPV[`\0`\0\x19\x82\x14\x15aK\x91WaK\x91aK\x04V[P`\x01\x01\x90V[`\0\x82\x19\x82\x11\x15aK\xABWaK\xABaK\x04V[P\x01\x90V[`\0\x80\x85\x85\x11\x15aK\xC0W`\0\x80\xFD[\x83\x86\x11\x15aK\xCDW`\0\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[`\0`@\x82\x84\x03\x12\x15aK\xECW`\0\x80\xFD[a@F\x83\x83aA\x04V[` \x80\x82R`b\x90\x82\x01R`\0\x80Q` aP$\x839\x81Q\x91R`@\x82\x01R\x7Fs.onlyServiceManagerOwner: calle``\x82\x01R\x7Fr is not the service manager own`\x80\x82\x01Ra2\xB9`\xF1\x1B`\xA0\x82\x01R`\xC0\x01\x90V[`\0\x82\x82\x10\x15aL\x84WaL\x84aK\x04V[P\x03\x90V[`\0` \x82\x84\x03\x12\x15aL\x9BW`\0\x80\xFD[\x81Qa@F\x81aC\x84V[`\0`\xFF\x82\x16`\xFF\x81\x14\x15aL\xBDWaL\xBDaK\x04V[`\x01\x01\x92\x91PPV[`\0\x80``\x83\x85\x03\x12\x15aL\xD9W`\0\x80\xFD[aL\xE3\x84\x84aA\x04V[\x91P`@\x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aL\xFEW`\0\x80\xFD[aM\n\x85\x82\x86\x01a@&V[\x91PP\x92P\x92\x90PV[`\0c\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15aM1WaM1aK\x04V[\x03\x93\x92PPPV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x14\x15aMSWaMSaK\x04V[`\x01\x01\x93\x92PPPV[`\0`\xA0\x82\x01\x87\x83R` \x87\x81\x85\x01R`@`\xA0\x81\x86\x01R\x82\x88Q\x80\x85R`\xC0\x87\x01\x91P\x83\x8A\x01\x94P`\0[\x81\x81\x10\x15aM\xD8W\x85Q\x80Q`\xFF\x16\x84R\x85\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x86\x85\x01R\x84\x01QaM\xC4\x85\x85\x01\x82\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[P\x94\x84\x01\x94`\x80\x92\x90\x92\x01\x91`\x01\x01aM\x89V[PP``\x86\x01\x97\x90\x97RPPPP`\x80\x01R\x93\x92PPPV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x85\x16\x81R`\x80` \x82\x01\x81\x90R`\0\x90aN?\x90\x83\x01\x85\x87aM\xF1V[\x83Q`@\x84\x01R` \x84\x01Q``\x84\x01R\x90Pa+XV[`\0` \x82\x84\x03\x12\x15aNiW`\0\x80\xFD[PQ\x91\x90PV[`\x01\x80`\xA0\x1B\x03\x85\x16\x81R\x83` \x82\x01R```@\x82\x01R`\0aN\x98``\x83\x01\x84\x86aM\xF1V[\x96\x95PPPPPPV[\x83\x81R`@` \x82\x01R`\0a p`@\x83\x01\x84\x86aM\xF1V[`\0` \x80\x83\x85\x03\x12\x15aN\xCFW`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aN\xE5W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aN\xF6W`\0\x80\xFD[\x80QaO\x04aDW\x82aCaV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15aO#W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aOJW\x83QaO;\x81aB\xC3V[\x82R\x92\x84\x01\x92\x90\x84\x01\x90aO(V[\x97\x96PPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R`\x80` \x82\x01\x81\x90R`\0\x90aOy\x90\x83\x01\x85aIxV[\x83Q`@\x84\x01R` \x84\x01Q``\x84\x01R\x90Pa sV[\x82\x81R`@` \x82\x01R`\0a s`@\x83\x01\x84aIxV[`\0` \x82\x84\x03\x12\x15aO\xBCW`\0\x80\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a@FW`\0\x80\xFD[`\0\x83QaO\xE6\x81\x84` \x88\x01aILV[`\x01`\x01`\xF8\x1B\x03\x19\x93\x90\x93\x16\x91\x90\x92\x01\x90\x81R`\x01\x01\x92\x91PPV\xFEs._registerOperatorWithCoordinatBLSRegistryCoordinatorWithIndice\xA2dipfsX\"\x12 h\xF7].\xB5!x:0%A\xC5h\xD9-n\x11\x87w\xEB2\xD0\xA6\"t\xAF\xE6*U^x\x1EdsolcC\0\x08\x0C\x003";
    /// The bytecode of the contract.
    pub static BLSREGISTRYCOORDINATORWITHINDICESHARNESS_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02^W`\x005`\xE0\x1C\x80cY\\jg\x11a\x01FW\x80c\xB14Bq\x11a\0\xC3W\x80c\xCA\r\xE8\x82\x11a\0\x87W\x80c\xCA\r\xE8\x82\x14a\x06GW\x80c\xD7-\x8D\xD6\x14a\x06nW\x80c\xD9,\xBB\x84\x14a\x06vW\x80c\xE6W\x97\xAD\x14a\x06\xA0W\x80c\xFA\xBC\x1C\xBC\x14a\x07CW\x80c\xFD9\x10Z\x14a\x07VW`\0\x80\xFD[\x80c\xB14Bq\x14a\x05\xD4W\x80c\xB7~\xC1\xD3\x14a\x05\xFBW\x80c\xB84z\xCC\x14a\x06\x0EW\x80c\xC6j\xB9\xCA\x14a\x06!W\x80c\xC8\x1B\x1F\xF4\x14a\x064W`\0\x80\xFD[\x80ch0H5\x11a\x01\nW\x80ch0H5\x14a\x05:W\x80c\x85\x02\rI\x14a\x05aW\x80c\x88o\x11\x95\x14a\x05\x81W\x80c\x9E\x99#\xC2\x14a\x05\x9AW\x80c\xAB&\x9B-\x14a\x05\xC1W`\0\x80\xFD[\x80cY\\jg\x14a\x04\xE5W\x80cZ\xC8j\xB7\x14a\x04\xEDW\x80c[\x0B\x82\x9F\x14a\x05\x0CW\x80c\\\x97Z\xBB\x14a\x05\x1FW\x80ccG\xC9\0\x14a\x05'W`\0\x80\xFD[\x80c)k\xB0d\x11a\x01\xDFW\x80c5a\xDE\xB1\x11a\x01\xA3W\x80c5a\xDE\xB1\x14a\x04>W\x80c9\x98\xFD\xD3\x14a\x04eW\x80cEZC\xFC\x14a\x04\x8CW\x80cKv\xB9\xD5\x14a\x04\x9FW\x80cRn\xA9N\x14a\x04\xB2W\x80cXe\xC6\x0C\x14a\x04\xC5W`\0\x80\xFD[\x80c)k\xB0d\x14a\x03\xC7W\x80c)\xD1\xE0\xC3\x14a\x03\xDAW\x80c,\xDD\x1E\x86\x14a\x03\xEDW\x80c0db\r\x14a\x04\0W\x80c41\xAF%\x14a\x04+W`\0\x80\xFD[\x80c\x13T*N\x11a\x02&W\x80c\x13T*N\x14a\x032W\x80c\x13d9\xDD\x14a\x03[W\x80c\x14x\x85\x1F\x14a\x03nW\x80c%\0uv\x14a\x03\xA1W\x80c(\xF6\x1B1\x14a\x03\xB4W`\0\x80\xFD[\x80c\x01Y\xF1\xCE\x14a\x02cW\x80c\x05C\x10\xE6\x14a\x02\xB1W\x80c\x05Zb\xB6\x14a\x02\xDCW\x80c\x0C\xF4\xB7g\x14a\x03\nW\x80c\x10\xD6z/\x14a\x03\x1FW[`\0\x80\xFD[a\x02va\x02q6`\x04a?&V[a\x07\x92V[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x80\x85\x01Q\x90\x91\x16\x90\x82\x01R\x91\x81\x01Q`\x01`\x01`\xC0\x1B\x03\x16\x90\x82\x01R``\x01[`@Q\x80\x91\x03\x90\xF3[`7Ta\x02\xC4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xA8V[a\x02\xFCa\x02\xEA6`\x04a?HV[`\0\x90\x81R`3` R`@\x90 T\x90V[`@Q\x90\x81R` \x01a\x02\xA8V[a\x03\x1Da\x03\x186`\x04a@MV[a\x08#V[\0[a\x03\x1Da\x03-6`\x04a@\xA6V[a\t$V[a\x02\xFCa\x03@6`\x04a@\xA6V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`4` R`@\x90 T\x90V[a\x03\x1Da\x03i6`\x04a?HV[a\t\xD7V[a\x03\x91a\x03|6`\x04a?HV[`5` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02\xA8V[a\x03\x1Da\x03\xAF6`\x04aA\xC1V[a\x0B\x14V[`8Ta\x02\xC4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02\xC4a\x03\xD56`\x04a?HV[a\x12CV[a\x03\x1Da\x03\xE86`\x04a@\xA6V[a\x12\xCFV[a\x03\x1Da\x03\xFB6`\x04a@\xA6V[a\x13\x8AV[a\x04\x13a\x04\x0E6`\x04aB\xD5V[a\x14EV[`@Q`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xA8V[a\x04\x13a\x0496`\x04a?HV[a\x16FV[a\x02\xC4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xC4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\x1Da\x04\x9A6`\x04aC\rV[a\x16\xFCV[a\x03\x1Da\x04\xAD6`\x04aC\xE9V[a\x175V[a\x03\x1Da\x04\xC06`\x04aD\xC6V[a\x1AaV[a\x04\xD8a\x04\xD36`\x04a@\xA6V[a\x1A\xB2V[`@Qa\x02\xA8\x91\x90aEiV[a\x03\x1Da\x1B&V[a\x03\x91a\x04\xFB6`\x04aE\x9CV[`\x01\x80T`\xFF\x90\x92\x16\x1B\x90\x81\x16\x14\x90V[a\x03\x1Da\x05\x1A6`\x04aE\xB7V[a\x1B\xF2V[`\x01Ta\x02\xFCV[a\x02\xC4a\x0556`\x04a?HV[a\x1C\xB2V[a\x02\xC4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x05ta\x05o6`\x04aE\xEBV[a\x1C\xDCV[`@Qa\x02\xA8\x91\x90aF\x90V[`\0Ta\x02\xC4\x90b\x01\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02\xC4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xFCa\x05\xCF6`\x04aG!V[a \x17V[a\x02\xC4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\x1Da\x06\t6`\x04aG\xD5V[a {V[a\x03\x1Da\x06\x1C6`\x04aH\x11V[a!tV[a\x03\x1Da\x06/6`\x04aHxV[a!\xFFV[a\x03\x1Da\x06B6`\x04aD\xC6V[a\"jV[a\x02\xFC\x7F\xAEc\xAA\x13\x18u\x1Fd\xEF\x85\x0E\x04T\x17\xF8h\x11\xEA\xEC\xEE\x1C\xEB\xC2&\x90M#{\x80pM\x92\x81V[`6Ta\x02\xFCV[a\x03\x1Da\x06\x846`\x04aH\xFCV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\0\x90\x81R`4` R`@\x90 UV[a\x07\x0Fa\x06\xAE6`\x04aE\x9CV[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x92\x84\x01\x81\x90R`\xFF\x94\x90\x94\x16\x84R`2\x82R\x92\x82\x90 \x82Q\x93\x84\x01\x83RTc\xFF\xFF\xFF\xFF\x81\x16\x84Ra\xFF\xFF`\x01` \x1B\x82\x04\x81\x16\x92\x85\x01\x92\x90\x92R`\x01`0\x1B\x90\x04\x16\x90\x82\x01R\x90V[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x16\x81R` \x80\x84\x01Qa\xFF\xFF\x90\x81\x16\x91\x83\x01\x91\x90\x91R\x92\x82\x01Q\x90\x92\x16\x90\x82\x01R``\x01a\x02\xA8V[a\x03\x1Da\x07Q6`\x04a?HV[a\"\xADV[a\x07\x85a\x07d6`\x04a@\xA6V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`4` R`@\x90 `\x01\x01T`\xFF\x16\x90V[`@Qa\x02\xA8\x91\x90aI(V[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R`\0\x83\x81R`3` R`@\x90 \x80T\x83\x90\x81\x10a\x07\xCFWa\x07\xCFaI6V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01`\xC0\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90P[\x92\x91PPV[`\x013`\0\x90\x81R`4` R`@\x90 `\x01\x01T`\xFF\x16`\x02\x81\x11\x15a\x08LWa\x08LaE1V[\x14a\x08\xD8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`K`$\x82\x01R\x7FBLSRegistryCoordinatorWithIndici`D\x82\x01R\x7Fes.updateSocket: operator is not`d\x82\x01Rj\x08\x1C\x99Y\xDA\\\xDD\x19\\\x99Y`\xAA\x1B`\x84\x82\x01R`\xA4\x01[`@Q\x80\x91\x03\x90\xFD[3`\0\x90\x81R`4` R`@\x90\x81\x90 T\x90Q\x7F\xEC)c\xAB!\xC1\xE5\x0E\x1EX*\xA5B\xAF.K\xF7\xBF8\xE6\xE1@<'\xB4.\x1C]nb\x1E\xAA\x90a\t\x19\x90\x84\x90aI\xA4V[`@Q\x80\x91\x03\x90\xA2PV[`\0`\x02\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\twW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x9B\x91\x90aI\xB7V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\t\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xCF\x90aI\xD4V[a\t\xD4\x81a$\tV[PV[`\0T`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01Rb\x01\0\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n$W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nH\x91\x90aJ\x1EV[a\ndW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xCF\x90aJ@V[`\x01T\x81\x81\x16\x14a\n\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08\xCFV[`\x01\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01a\t\x19V[`\x01\x80T`\0\x91\x90\x81\x16\x14\x15a\x0B<W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xCF\x90aJ\x88V[`\0a\x0B\x813\x8B\x8B\x8B\x8B\x8B\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa%\x0E\x92PPPV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x91\x92P`\0\x91\x90` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90Pa\x0B\xB0\x89a+aV[\x81`\0\x81Q\x81\x10a\x0B\xC3Wa\x0B\xC3aI6V[` \x02` \x01\x01\x81\x81RPPa\x0CF\x81`\0\x81Q\x81\x10a\x0B\xE5Wa\x0B\xE5aI6V[` \x02` \x01\x01Q\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x0C;Wa\x0C,`\x80\x83\x02\x86\x016\x81\x90\x03\x81\x01\x90aJ\xBFV[\x81R` \x01\x90`\x01\x01\x90a\x0C\x0FV[PPPPP\x86a+\xA4V[`\0\x80[\x8B\x81\x10\x15a\x124W`\0\x8D\x8D\x83\x81\x81\x10a\x0CfWa\x0CfaI6V[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`2` \x90\x81R`@\x91\x82\x90 \x82Q``\x81\x01\x84R\x90Tc\xFF\xFF\xFF\xFF\x81\x16\x80\x83Ra\xFF\xFF`\x01` \x1B\x83\x04\x81\x16\x94\x84\x01\x94\x90\x94R`\x01`0\x1B\x90\x91\x04\x90\x92\x16\x92\x81\x01\x92\x90\x92R\x88Q\x92\x94P\x90\x92P\x90\x87\x90\x85\x90\x81\x10a\x0C\xD4Wa\x0C\xD4aI6V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11a\x0C\xEEWPPa\x12\"V[\x81`\xFF\x16\x8A\x8A\x86\x81\x81\x10a\r\x04Wa\r\x04aI6V[a\r\x1A\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91PaE\x9CV[`\xFF\x16\x14a\r\xB3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`f`$\x82\x01R`\0\x80Q` aP$\x839\x81Q\x91R`D\x82\x01R\x7Fs.registerOperatorWithCoordinato`d\x82\x01R\x7Fr: quorumNumber not the same as `\x84\x82\x01Re\x1C\xDAY\xDB\x99Y`\xD2\x1B`\xA4\x82\x01R`\xC4\x01a\x08\xCFV[`@Qc\x0C\x8Fs\x9D`\xE4\x1B\x81R`\xFF\x83\x16`\x04\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xC8\xF79\xD0\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\x1DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0EA\x91\x90aJ\xDBV[\x90P`\0`4`\0\x8D\x8D\x88\x81\x81\x10a\x0E[Wa\x0E[aI6V[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\x0Es\x91\x90a@\xA6V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x82\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 T\x91Qc\x1A\xADN5`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`\xFF\x88\x16`$\x82\x01R\x91\x93P\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cj\xB58\xD4\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xFCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F \x91\x90aJ\xDBV[\x90P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cj\xB58\xD4\x8A`\0\x81Q\x81\x10a\x0FfWa\x0FfaI6V[` \x02` \x01\x01Q\x88`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0F\x98\x92\x91\x90\x91\x82R`\xFF\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xB5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xD9\x91\x90aJ\xDBV[` \x86\x01Q\x90\x91Pa'\x10\x90a\x0F\xF3\x90a\xFF\xFF\x16\x84aK\x1AV[a\x0F\xFD\x91\x90aKIV[`\x01`\x01``\x1B\x03\x16\x81`\x01`\x01``\x1B\x03\x16\x11a\x10\xBDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`}`$\x82\x01R`\0\x80Q` aP$\x839\x81Q\x91R`D\x82\x01R\x7Fs.registerOperatorWithCoordinato`d\x82\x01R\x7Fr: registering operator has less`\x84\x82\x01R\x7F than kickBIPsOfOperatorStake\0\0\0`\xA4\x82\x01R`\xC4\x01a\x08\xCFV[`@\x85\x01Qa'\x10\x90a\x10\xD4\x90a\xFF\xFF\x16\x86aK\x1AV[a\x10\xDE\x91\x90aKIV[`\x01`\x01``\x1B\x03\x16\x82`\x01`\x01``\x1B\x03\x16\x10a\x11\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`v`$\x82\x01R`\0\x80Q` aP$\x839\x81Q\x91R`D\x82\x01R\x7Fs.registerOperatorWithCoordinato`d\x82\x01R\x7Fr: operator to kick has more tha`\x84\x82\x01Run kickBIPSOfTotalStake`P\x1B`\xA4\x82\x01R`\xC4\x01a\x08\xCFV[\x87a\x11\xA1\x81aK}V[\x98PPPPPPPPa\x12\"\x88\x88\x83\x81\x81\x10a\x11\xBFWa\x11\xBFaI6V[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\x11\xD7\x91\x90a@\xA6V[\x8E\x83\x8Fa\x11\xE5\x82`\x01aK\x98V[\x92a\x11\xF2\x93\x92\x91\x90aK\xB0V[\x8B\x8B\x86\x81\x81\x10a\x12\x04Wa\x12\x04aI6V[\x90P`\x80\x02\x01`@\x01\x806\x03\x81\x01\x90a\x12\x1D\x91\x90aK\xDAV[a-qV[\x80a\x12,\x81aK}V[\x91PPa\x0CJV[PPPPPPPPPPPPPV[`@Qc\x08\xF6b\x9D`\xE3\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cG\xB3\x14\xE8\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xABW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x1D\x91\x90aI\xB7V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13-W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13Q\x91\x90aI\xB7V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x13\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xCF\x90aK\xF6V[a\t\xD4\x81a3\x1EV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xE8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\x0C\x91\x90aI\xB7V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x14<W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xCF\x90aK\xF6V[a\t\xD4\x81a3\x87V[`\0\x83\x81R`3` R`@\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x14hWa\x14haI6V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x85R`\x01` \x1B\x83\x04\x82\x16\x95\x85\x01\x95\x90\x95R`\x01`@\x1B\x90\x91\x04`\x01`\x01`\xC0\x1B\x03\x16\x91\x83\x01\x91\x90\x91R\x90\x92P\x85\x16\x10\x15a\x15hW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x7F`$\x82\x01R`\0\x80Q` aP$\x839\x81Q\x91R`D\x82\x01R\x7Fs.getQuorumBitmapByOperatorIdAtB`d\x82\x01R\x7FlockNumberByIndex: quorumBitmapU`\x84\x82\x01R\x7Fpdate is from after blockNumber\0`\xA4\x82\x01R`\xC4\x01a\x08\xCFV[\x83c\xFF\xFF\xFF\xFF\x16\x81` \x01Qc\xFF\xFF\xFF\xFF\x16\x11\x80a\x15\x8EWP` \x81\x01Qc\xFF\xFF\xFF\xFF\x16\x15[a\x16:W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x80`$\x82\x01R`\0\x80Q` aP$\x839\x81Q\x91R`D\x82\x01R\x7Fs.getQuorumBitmapByOperatorIdAtB`d\x82\x01R\x7FlockNumberByIndex: quorumBitmapU`\x84\x82\x01R\x7Fpdate is from before blockNumber`\xA4\x82\x01R`\xC4\x01a\x08\xCFV[`@\x01Q\x94\x93PPPPV[`\0\x81\x81R`3` R`@\x81 T\x80\x15\x80a\x16\xA2WP`\0\x83\x81R`3` R`@\x90 a\x16v`\x01\x83aLrV[\x81T\x81\x10a\x16\x86Wa\x16\x86aI6V[`\0\x91\x82R` \x90\x91 \x01T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15\x15[\x15a\x16\xB0WP`\0\x92\x91PPV[`\0\x83\x81R`3` R`@\x90 a\x16\xC9`\x01\x83aLrV[\x81T\x81\x10a\x16\xD9Wa\x16\xD9aI6V[`\0\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01`\xC0\x1B\x03\x16\x93\x92PPPV[`\x01\x80T`\x02\x90\x81\x16\x14\x15a\x17#W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xCF\x90aJ\x88V[a\x17/3\x85\x85\x85a-qV[PPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x17UWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x17oWP0;\x15\x80\x15a\x17oWP`\0T`\xFF\x16`\x01\x14[a\x17\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x08\xCFV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x17\xF5W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x17\xFF\x83\x83a3\xF0V[a\x18\x08\x86a3\x1EV[a\x18\x11\x85a3\x87V[`6\x80T`\x01\x80\x82\x01\x83U`\0\x83\x90R\x7FJ\x11\xF9N \xA9<y\xF6\xECt:\x19T\xECO\xC2\xC0\x84)\xAE!\"\x11\x8B\xF24\xB2\x18\\\x81\xB8\x91\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x82\x17\x90\x93U\x85T\x80\x85\x01\x87U\x85\x01\x80T\x83\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x16\x17\x90U\x85T\x93\x84\x01\x90\x95U\x91\x90\x92\x01\x80T\x90\x91\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x16\x91\x90\x91\x17\x90U\x84Q`@\x80Qc\x9A\xA1e=`\xE0\x1B\x81R\x90Q\x91\x92\x91c\x9A\xA1e=\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x193W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19W\x91\x90aL\x89V[a\xFF\xFF\x16\x14a\x19\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`F`$\x82\x01R`\0\x80Q` aP$\x839\x81Q\x91R`D\x82\x01R\x7Fs: operator set params length mi`d\x82\x01Re\x0Em\xAC.\x8Cm`\xD3\x1B`\x84\x82\x01R`\xA4\x01a\x08\xCFV[`\0[\x84Q\x81`\xFF\x16\x10\x15a\x1A\x12Wa\x1A\0\x81\x86\x83`\xFF\x16\x81Q\x81\x10a\x19\xF3Wa\x19\xF3aI6V[` \x02` \x01\x01Qa4\xDCV[\x80a\x1A\n\x81aL\xA6V[\x91PPa\x19\xCEV[P\x80\x15a\x1AYW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPV[`\x01\x80T`\0\x91\x90\x81\x16\x14\x15a\x1A\x89W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xCF\x90aJ\x88V[`\0\x80a\x1A\x98\x84\x86\x01\x86aL\xC6V[\x91P\x91Pa\x1A\xA93\x88\x88\x85\x85a5\x89V[PPPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`4` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x81\x01T\x90\x91\x83\x01\x90`\xFF\x16`\x02\x81\x11\x15a\x1B\x0CWa\x1B\x0CaE1V[`\x02\x81\x11\x15a\x1B\x1DWa\x1B\x1DaE1V[\x90RP\x92\x91PPV[`\0T`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01Rb\x01\0\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1BsW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\x97\x91\x90aJ\x1EV[a\x1B\xB3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xCF\x90aJ@V[`\0\x19`\x01\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1CPW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Ct\x91\x90aI\xB7V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1C\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xCF\x90aK\xF6V[a\x1C\xAE\x82\x82a4\xDCV[PPV[`6\x81\x81T\x81\x10a\x1C\xC2W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x81V[```\0\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1C\xF9Wa\x1C\xF9a?aV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1D\"W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a \x0FW`\0`3`\0\x86\x84\x81Q\x81\x10a\x1DIWa\x1DIaI6V[` \x02` \x01\x01Q\x81R` \x01\x90\x81R` \x01`\0 \x80T\x90P\x90P`\0[\x81c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\x1F\xFAW\x86c\xFF\xFF\xFF\xFF\x16`3`\0\x88\x86\x81Q\x81\x10a\x1D\x99Wa\x1D\x99aI6V[` \x02` \x01\x01Q\x81R` \x01\x90\x81R` \x01`\0 `\x01\x83\x85a\x1D\xBD\x91\x90aM\x14V[a\x1D\xC7\x91\x90aM\x14V[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x1D\xDDWa\x1D\xDDaI6V[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a\x1F\xE8W`3`\0\x87\x85\x81Q\x81\x10a\x1E\nWa\x1E\naI6V[` \x02` \x01\x01Q\x81R` \x01\x90\x81R` \x01`\0 `\x01\x82\x84a\x1E.\x91\x90aM\x14V[a\x1E8\x91\x90aM\x14V[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x1ENWa\x1ENaI6V[`\0\x91\x82R` \x90\x91 \x01T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15\x80a\x1E\xEAWP\x86c\xFF\xFF\xFF\xFF\x16`3`\0\x88\x86\x81Q\x81\x10a\x1E\x8BWa\x1E\x8BaI6V[` \x02` \x01\x01Q\x81R` \x01\x90\x81R` \x01`\0 `\x01\x83\x85a\x1E\xAF\x91\x90aM\x14V[a\x1E\xB9\x91\x90aM\x14V[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x1E\xCFWa\x1E\xCFaI6V[`\0\x91\x82R` \x90\x91 \x01T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x11[a\x1F\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x82`$\x82\x01R`\0\x80Q` aP$\x839\x81Q\x91R`D\x82\x01R\x7Fs.getQuorumBitmapIndicesByOperat`d\x82\x01R\x7ForIdsAtBlockNumber: operatorId h`\x84\x82\x01R\x7Fas no quorumBitmaps at blockNumb`\xA4\x82\x01Ra2\xB9`\xF1\x1B`\xC4\x82\x01R`\xE4\x01a\x08\xCFV[`\x01a\x1F\xAD\x82\x84aM\x14V[a\x1F\xB7\x91\x90aM\x14V[\x84\x84\x81Q\x81\x10a\x1F\xC9Wa\x1F\xC9aI6V[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPPa\x1F\xFAV[\x80a\x1F\xF2\x81aM9V[\x91PPa\x1DhV[PP\x80\x80a \x07\x90aK}V[\x91PPa\x1D(V[P\x93\x92PPPV[`\0a p\x7F\xAEc\xAA\x13\x18u\x1Fd\xEF\x85\x0E\x04T\x17\xF8h\x11\xEA\xEC\xEE\x1C\xEB\xC2&\x90M#{\x80pM\x92\x86\x86\x86\x86`@Q` \x01a U\x95\x94\x93\x92\x91\x90aM]V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a6\xA0V[\x90P[\x94\x93PPPPV[`\0\x82\x81R`3` R`@\x90 T\x80\x15a \xE7W`\0\x83\x81R`3` R`@\x90 C\x90a \xAB`\x01\x84aLrV[\x81T\x81\x10a \xBBWa \xBBaI6V[\x90`\0R` `\0 \x01`\0\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP[P`\0\x91\x82R`3` \x90\x81R`@\x80\x84 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x87\x81R`\x01`\x01`\xC0\x1B\x03\x96\x87\x16\x94\x83\x01\x94\x85R\x83T`\x01\x81\x01\x85U\x93\x88R\x94\x90\x96 \x90Q\x91\x01\x80T\x93Q\x92Q\x90\x94\x16`\x01`@\x1B\x02`\x01`\x01`@\x1B\x03\x92\x86\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x94\x16\x91\x90\x95\x16\x17\x91\x90\x91\x17\x16\x91\x90\x91\x17\x90UV[`8T`\x01`\x01`\xA0\x1B\x03\x163\x14a!\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`H`$\x82\x01R`\0\x80Q` aP$\x839\x81Q\x91R`D\x82\x01R\x7Fs.onlyEjector: caller is not the`d\x82\x01Rg\x102\xB52\xB1\xBA7\xB9`\xC1\x1B`\x84\x82\x01R`\xA4\x01a\x08\xCFV[a\x17/\x84\x84\x84\x84a-qV[`\x01\x80T`\0\x91\x90\x81\x16\x14\x15a\"'W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xCF\x90aJ\x88V[a\x1AY3\x87\x87\x87\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa5\x89\x92PPPV[`\x01\x80T`\x02\x90\x81\x16\x14\x15a\"\x91W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xCF\x90aJ\x88V[`\0a\"\x9F\x83\x85\x01\x85aK\xDAV[\x90Pa\x1AY3\x87\x87\x84a-qV[`\0`\x02\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a#\0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#$\x91\x90aI\xB7V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a#TW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08\xCF\x90aI\xD4V[`\x01T\x19\x81\x19`\x01T\x19\x16\x14a#\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08\xCFV[`\x01\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\t\x19V[`\x01`\x01`\xA0\x1B\x03\x81\x16a$\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x08\xCFV[`\0T`@\x80Q`\x01`\x01`\xA0\x1B\x03b\x01\0\0\x90\x93\x04\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`\0\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16b\x01\0\0\x02b\x01\0\0`\x01`\xB0\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[```\0a%Q\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa6\xEE\x92PPPV[\x90P`\x01`\x01`\xC0\x1B\x03\x81\x11\x15a%\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`k`$\x82\x01R`\0\x80Q` aP$\x839\x81Q\x91R`D\x82\x01R`\0\x80Q` aP\x04\x839\x81Q\x91R`d\x82\x01R\x7For: quorumBitmap exceeds of max `\x84\x82\x01Rjbitmap size`\xA8\x1B`\xA4\x82\x01R`\xC4\x01a\x08\xCFV[\x80a&[W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R`\0\x80Q` aP$\x839\x81Q\x91R`D\x82\x01R`\0\x80Q` aP\x04\x839\x81Q\x91R`d\x82\x01R\x7For: quorumBitmap cannot be 0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x08\xCFV[`@Qc\x03\xCEK\xAD`\xE0\x1B\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x03\xCEK\xAD\x90a&\xB0\x90\x8B\x90\x8B\x90\x8B\x90\x8B\x90`\x04\x01aN\x1AV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a&\xCFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\xF3\x91\x90aNWV[`@Qc%PGw`\xE0\x1B\x81R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c%PGw\x90a'H\x90\x8B\x90\x85\x90\x8C\x90\x8C\x90`\x04\x01aNpV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a'bW`\0\x80\xFD[PZ\xF1\x15\x80\x15a'vW=`\0\x80>=`\0\xFD[PP`@Qb\xBF\xF0M`\xE0\x1B\x81R`\0\x92P`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91Pb\xBF\xF0M\x90a'\xCB\x90\x85\x90\x8C\x90\x8C\x90`\x04\x01aN\xA2V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a'\xEAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra(\x12\x91\x90\x81\x01\x90aN\xBCV[`\0\x83\x81R`3` R`@\x90 T\x90\x91P\x80\x15\x80\x15\x90a(rWP`\0\x83\x81R`3` R`@\x90 a(G`\x01\x83aLrV[\x81T\x81\x10a(WWa(WaI6V[`\0\x91\x82R` \x90\x91 \x01T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15[\x15a)\xC9W`\0\x83\x81R`3` R`@\x81 a(\x90`\x01\x84aLrV[\x81T\x81\x10a(\xA0Wa(\xA0aI6V[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xC0\x1B\x03`\x01`@\x1B\x90\x91\x04\x81\x81\x16\x92P\x86\x16\x16\x15a)mW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x85`$\x82\x01R`\0\x80Q` aP$\x839\x81Q\x91R`D\x82\x01R`\0\x80Q` aP\x04\x839\x81Q\x91R`d\x82\x01R\x7For: operator already registered `\x84\x82\x01R\x7Ffor some quorums being registere`\xA4\x82\x01Rd2\x1037\xB9`\xD9\x1B`\xC4\x82\x01R`\xE4\x01a\x08\xCFV[`\0\x84\x81R`3` R`@\x90 \x94\x81\x17\x94C\x90a)\x8C`\x01\x85aLrV[\x81T\x81\x10a)\x9CWa)\x9CaI6V[\x90`\0R` `\0 \x01`\0\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPP[`\0\x83\x81R`3` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x86\x81R`\x01`\x01`\xC0\x1B\x03\x8B\x81\x16\x84\x87\x01\x90\x81R\x85T`\x01\x80\x82\x01\x88U\x96\x8AR\x88\x8A \x95Q\x95\x01\x80T\x93Q\x91Q\x90\x92\x16`\x01`@\x1B\x02`\x01`\x01`@\x1B\x03\x91\x85\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x94\x16\x95\x90\x94\x16\x94\x90\x94\x17\x91\x90\x91\x17\x92\x90\x92\x16\x17\x90U`\x01`\x01`\xA0\x1B\x03\x8E\x16\x84R`4\x90\x92R\x90\x91 \x81\x01T`\xFF\x16`\x02\x81\x11\x15a*\x81Wa*\x81aE1V[\x14a+\x1AW`@\x80Q\x80\x82\x01\x82R\x84\x81R`\x01` \x80\x83\x01\x82\x81R`\x01`\x01`\xA0\x1B\x03\x8F\x16`\0\x90\x81R`4\x90\x92R\x93\x90 \x82Q\x81U\x92Q\x83\x82\x01\x80T\x93\x94\x93\x91\x92\x90\x91`\xFF\x19\x16\x90\x83`\x02\x81\x11\x15a*\xDCWa*\xDCaE1V[\x02\x17\x90UPP`@Q\x84\x91P`\x01`\x01`\xA0\x1B\x03\x8C\x16\x90\x7F\xE8\xE6\x8C\xEF\x1C:v\x1E\xD7\xBE~\x84c\xA3u\xF2\x7F{\xC35\xE5\x18$\"<\xAC\xCEcn\xC5\xC3\xFE\x90`\0\x90\xA3[\x82\x7F\xEC)c\xAB!\xC1\xE5\x0E\x1EX*\xA5B\xAF.K\xF7\xBF8\xE6\xE1@<'\xB4.\x1C]nb\x1E\xAA\x87`@Qa+J\x91\x90aI\xA4V[`@Q\x80\x91\x03\x90\xA2P\x92PPP[\x95\x94PPPPPV[`\0\x81`\0\x01Q\x82` \x01Q`@Q` \x01a+\x87\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[` \x80\x82\x01Q`\0\x90\x81R`5\x90\x91R`@\x90 T`\xFF\x16\x15a,iW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`w`$\x82\x01R`\0\x80Q` aP$\x839\x81Q\x91R`D\x82\x01R\x7Fs._verifyChurnApproverSignatureO`d\x82\x01R\x7FnOperatorChurnApproval: churnApp`\x84\x82\x01R\x7Frover salt already used\0\0\0\0\0\0\0\0\0`\xA4\x82\x01R`\xC4\x01a\x08\xCFV[B\x81`@\x01Q\x10\x15a-\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`w`$\x82\x01R`\0\x80Q` aP$\x839\x81Q\x91R`D\x82\x01R\x7Fs._verifyChurnApproverSignatureO`d\x82\x01R\x7FnOperatorChurnApproval: churnApp`\x84\x82\x01R\x7Frover signature expired\0\0\0\0\0\0\0\0\0`\xA4\x82\x01R`\xC4\x01a\x08\xCFV[` \x80\x82\x01\x80Q`\0\x90\x81R`5\x90\x92R`@\x91\x82\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U`7T\x90Q\x91\x83\x01Qa-l\x92`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91a-e\x91\x87\x91\x87\x91a \x17V[\x83Qa8{V[PPPV[`\x01`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`4` R`@\x90 `\x01\x01T`\xFF\x16`\x02\x81\x11\x15a-\xA3Wa-\xA3aE1V[\x14a.*W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R```$\x82\x01R`\0\x80Q` aP$\x839\x81Q\x91R`D\x82\x01R\x7Fs._deregisterOperatorWithCoordin`d\x82\x01R\x7Fator: operator is not registered`\x84\x82\x01R`\xA4\x01a\x08\xCFV[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`4` R`@\x90 Ta.L\x82a+aV[\x81\x14a.\xE8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`k`$\x82\x01R`\0\x80Q` aP$\x839\x81Q\x91R`D\x82\x01R\x7Fs._deregisterOperatorWithCoordin`d\x82\x01R\x7Fator: operatorId does not match `\x84\x82\x01Rj\x0E\x0E\xACMl\xAF$\r\x0C.m`\xAB\x1B`\xA4\x82\x01R`\xC4\x01a\x08\xCFV[`\0a/)\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa6\xEE\x92PPPV[`\0\x83\x81R`3` R`@\x81 T\x91\x92P\x90a/H\x90`\x01\x90aLrV[`\0\x84\x81R`3` R`@\x81 \x80T\x92\x93P\x90\x91\x83\x90\x81\x10a/mWa/maI6V[`\0\x91\x82R` \x82 \x01T`\x01`@\x1B\x90\x04\x93\x84\x16`\x01`\x01`\xC0\x1B\x03\x90\x81\x16\x94\x16\x91Pa/\x9A\x84a:5V[\x90P\x80Q`\0\x14\x15a0NW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x80`$\x82\x01R`\0\x80Q` aP$\x839\x81Q\x91R`D\x82\x01R\x7Fs._deregisterOperatorWithCoordin`d\x82\x01R\x7Fator: operator is not registered`\x84\x82\x01R\x7F for any of the provided quorums`\xA4\x82\x01R`\xC4\x01a\x08\xCFV[`@Qc\x12\x1BM\x95`\xE1\x1B\x81R`\x01`\x01`\xC0\x1B\x03\x83\x16\x85\x14\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c$6\x9B*\x90a0\xAB\x90\x8D\x90\x86\x90\x8C\x90`\x04\x01aOUV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a0\xC5W`\0\x80\xFD[PZ\xF1\x15\x80\x15a0\xD9W=`\0\x80>=`\0\xFD[PP`@Qc\xBD)\xB8\xCD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92Pc\xBD)\xB8\xCD\x91Pa1+\x90\x89\x90\x86\x90`\x04\x01aO\x91V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a1EW`\0\x80\xFD[PZ\xF1\x15\x80\x15a1YW=`\0\x80>=`\0\xFD[PP`@Qc\xBD)\xB8\xCD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92Pc\xBD)\xB8\xCD\x91Pa1\xAB\x90\x89\x90\x86\x90`\x04\x01aO\x91V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a1\xC5W`\0\x80\xFD[PZ\xF1\x15\x80\x15a1\xD9W=`\0\x80>=`\0\xFD[PPP`\0\x87\x81R`3` R`@\x90 \x80TC\x92P\x86\x90\x81\x10a1\xFFWa1\xFFaI6V[\x90`\0R` `\0 \x01`\0\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80a2\xC3W`\0\x86\x81R`3` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x86\x81R`\x01`\x01`\xC0\x1B\x03\x8C\x19\x8B\x16\x81\x16\x95\x84\x01\x95\x86R\x84T`\x01\x81\x01\x86U\x94\x88R\x95\x90\x96 \x91Q\x91\x90\x92\x01\x80T\x95Q\x93Q\x90\x94\x16`\x01`@\x1B\x02`\x01`\x01`@\x1B\x03\x93\x83\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x96\x16\x91\x90\x92\x16\x17\x93\x90\x93\x17\x16\x91\x90\x91\x17\x90Ua3\x12V[`\x01`\x01`\xA0\x1B\x03\x8A\x16`\0\x81\x81R`4` R`@\x80\x82 `\x01\x01\x80T`\xFF\x19\x16`\x02\x17\x90UQ\x88\x92\x91\x7F9o\xDC\xB1\x80\xCB\x0F\xEA&\x92\x81\x13\xFB\x0F\xD1\xC3T\x98c\xF9\xCDV>j\x18O\x1DW\x81\x16\xC8\xE4\x91\xA3[PPPPPPPPPPV[`7T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F1TW\xD8\xA8\xFE`\xF0J\xF1|\x16\xE2\xF5\xA5\xE1\xDBa+1d\x8EX\x03\x03`u\x9E\xF8\xF3R\x8C\x91\x01`@Q\x80\x91\x03\x90\xA1`7\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`8T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F\x8F0\xAB\t\xF4:l\x15}\x7F\xCE~\n\x13\xC0\x03\x04,\x1C\x95\xE8\xA7.z\x14j!\xC0\xCA\xA2M\xC9\x91\x01`@Q\x80\x91\x03\x90\xA1`8\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0Tb\x01\0\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a4\x17WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a4\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x08\xCFV[`\x01\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a\x1C\xAE\x82a$\tV[`\xFF\x82\x16`\0\x81\x81R`2` \x90\x81R`@\x91\x82\x90 \x84Q\x81T\x86\x84\x01\x80Q\x88\x87\x01\x80Qc\xFF\xFF\xFF\xFF\x90\x95\x16e\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x94\x16\x84\x17`\x01` \x1Ba\xFF\xFF\x93\x84\x16\x02\x17g\xFF\xFF\0\0\0\0\0\0\x19\x16`\x01`0\x1B\x95\x83\x16\x95\x90\x95\x02\x94\x90\x94\x17\x90\x94U\x85Q\x91\x82RQ\x83\x16\x93\x81\x01\x93\x90\x93RQ\x16\x91\x81\x01\x91\x90\x91R\x7F>\xE6\xFE\x8DTa\x02D\xC3\xE9\xD3\xC0f\xAEJ\xEE\x99x\x84\xAA(\xF1\x06\x16\xAE\x82\x19%@\x13\x18\xAC\x90``\x01`@Q\x80\x91\x03\x90\xA2PPV[`\0a5\x98\x86\x86\x86\x86\x86a%\x0EV[\x90P`\0[\x81Q\x81\x10\x15a\x1A\xA9W`2`\0\x87\x87\x84\x81\x81\x10a5\xBCWa5\xBCaI6V[\x91\x90\x91\x015`\xF8\x1C\x82RP` \x81\x01\x91\x90\x91R`@\x01`\0 T\x82Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x90\x83\x90\x83\x90\x81\x10a5\xF3Wa5\xF3aI6V[` \x02` \x01\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15a6\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`n`$\x82\x01R`\0\x80Q` aP$\x839\x81Q\x91R`D\x82\x01R`\0\x80Q` aP\x04\x839\x81Q\x91R`d\x82\x01R\x7ForAndNoOverfilledQuorums: quorum`\x84\x82\x01Rm\x08\x1A\\\xC8\x1B\xDD\x99\\\x99\x9A[\x1B\x19Y`\x92\x1B`\xA4\x82\x01R`\xC4\x01a\x08\xCFV[\x80a6\x98\x81aK}V[\x91PPa5\x9DV[`\0a\x08\x1Da6\xADa:\x92V[\x83`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x83\x90R`B\x81\x01\x82\x90R`\0\x90`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[`\0a\x01\0\x82Q\x11\x15a7wW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FBitmapUtils.orderedBytesArrayToB\x90\x82\x01R\x7Fitmap: orderedBytesArray is too `d\x82\x01Rclong`\xE0\x1B`\x84\x82\x01R`\xA4\x01a\x08\xCFV[\x81Qa7\x85WP`\0\x91\x90PV[`\0\x80\x83`\0\x81Q\x81\x10a7\x9BWa7\x9BaI6V[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x81\x90\x1B\x92P[\x84Q\x81\x10\x15a8rW\x84\x81\x81Q\x81\x10a7\xC9Wa7\xC9aI6V[\x01` \x01Q`\x01`\xF8\x91\x90\x91\x1C\x1B\x91P\x82\x82\x11a8^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FBitmapUtils.orderedBytesArrayToB`D\x82\x01R\x7Fitmap: orderedBytesArray is not `d\x82\x01Rf\x1B\xDC\x99\x19\\\x99Y`\xCA\x1B`\x84\x82\x01R`\xA4\x01a\x08\xCFV[\x91\x81\x17\x91a8k\x81aK}V[\x90Pa7\xAEV[P\x90\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16;\x15a9\x95W`@Qc\x0B\x13]?`\xE1\x1B\x80\x82R\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\x16&\xBA~\x90a8\xBB\x90\x86\x90\x86\x90`\x04\x01aO\x91V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a8\xD8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8\xFC\x91\x90aO\xAAV[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14a-lW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`S`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: ERC1271 signature `d\x82\x01Rr\x1D\x99\\\x9AY\x9AX\xD8]\x1A[\xDB\x88\x19\x98Z[\x19Y`j\x1B`\x84\x82\x01R`\xA4\x01a\x08\xCFV[\x82`\x01`\x01`\xA0\x1B\x03\x16a9\xA9\x83\x83a;\xB9V[`\x01`\x01`\xA0\x1B\x03\x16\x14a-lW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: signature not from`d\x82\x01Rf\x109\xB4\xB3\xB72\xB9`\xC9\x1B`\x84\x82\x01R`\xA4\x01a\x08\xCFV[```\0\x80[a\x01\0\x81\x10\x15a:\x8BW`\x01\x81\x1B\x91P\x83\x82\x16\x15a:{W\x82\x81`\xF8\x1B`@Q` \x01a:i\x92\x91\x90aO\xD4V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92P[a:\x84\x81aK}V[\x90Pa:;V[PP\x91\x90PV[`\x000`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80\x15a:\xEBWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14[\x15a;\x15WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[P`@\x80Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x80\x83\x01\x91\x90\x91R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x84\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x83\x01RF`\x80\x83\x01R0`\xA0\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\xC0\x90\x92\x01\x90\x92R\x80Q\x91\x01 \x90V[`\0\x80`\0a;\xC8\x85\x85a;\xD5V[\x91P\x91Pa \x0F\x81a<EV[`\0\x80\x82Q`A\x14\x15a<\x0CW` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1Aa<\0\x87\x82\x85\x85a>\0V[\x94P\x94PPPPa<>V[\x82Q`@\x14\x15a<6W` \x83\x01Q`@\x84\x01Qa<+\x86\x83\x83a>\xEDV[\x93P\x93PPPa<>V[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0\x81`\x04\x81\x11\x15a<YWa<YaE1V[\x14\x15a<bWPV[`\x01\x81`\x04\x81\x11\x15a<vWa<vaE1V[\x14\x15a<\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x08\xCFV[`\x02\x81`\x04\x81\x11\x15a<\xD8Wa<\xD8aE1V[\x14\x15a=&W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x08\xCFV[`\x03\x81`\x04\x81\x11\x15a=:Wa=:aE1V[\x14\x15a=\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x08\xCFV[`\x04\x81`\x04\x81\x11\x15a=\xA7Wa=\xA7aE1V[\x14\x15a\t\xD4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 'v' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x08\xCFV[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a>7WP`\0\x90P`\x03a>\xE4V[\x84`\xFF\x16`\x1B\x14\x15\x80\x15a>OWP\x84`\xFF\x16`\x1C\x14\x15[\x15a>`WP`\0\x90P`\x04a>\xE4V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a>\xB4W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a>\xDDW`\0`\x01\x92P\x92PPa>\xE4V[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x80`\x01`\x01`\xFF\x1B\x03\x83\x16\x81a?\n`\xFF\x86\x90\x1C`\x1BaK\x98V[\x90Pa?\x18\x87\x82\x88\x85a>\0V[\x93P\x93PPP\x93P\x93\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a?9W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0` \x82\x84\x03\x12\x15a?ZW`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a?\x99Wa?\x99a?aV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a?\xC7Wa?\xC7a?aV[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x83\x11\x15a?\xE8Wa?\xE8a?aV[a?\xFB`\x1F\x84\x01`\x1F\x19\x16` \x01a?\x9FV[\x90P\x82\x81R\x83\x83\x83\x01\x11\x15a@\x0FW`\0\x80\xFD[\x82\x82` \x83\x017`\0` \x84\x83\x01\x01R\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a@7W`\0\x80\xFD[a@F\x83\x835` \x85\x01a?\xCFV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a@_W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a@uW`\0\x80\xFD[a s\x84\x82\x85\x01a@&V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\t\xD4W`\0\x80\xFD[\x805a@\xA1\x81a@\x81V[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a@\xB8W`\0\x80\xFD[\x815a@F\x81a@\x81V[`\0\x80\x83`\x1F\x84\x01\x12a@\xD5W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a@\xECW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a<>W`\0\x80\xFD[`\0`@\x82\x84\x03\x12\x15aA\x16W`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15aA8WaA8a?aV[`@R\x825\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0``\x82\x84\x03\x12\x15aAdW`\0\x80\xFD[aAla?wV[\x90P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aA\x84W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13aA\x95W`\0\x80\xFD[aA\xA4\x84\x825` \x84\x01a?\xCFV[\x82RP` \x82\x015` \x82\x01R`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\0\x80`\xC0\x89\x8B\x03\x12\x15aA\xDDW`\0\x80\xFD[\x885`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aA\xF4W`\0\x80\xFD[aB\0\x8C\x83\x8D\x01a@\xC3V[\x90\x9AP\x98P\x88\x91PaB\x15\x8C` \x8D\x01aA\x04V[\x97P``\x8B\x015\x91P\x80\x82\x11\x15aB+W`\0\x80\xFD[aB7\x8C\x83\x8D\x01a@\xC3V[\x90\x97P\x95P`\x80\x8B\x015\x91P\x80\x82\x11\x15aBPW`\0\x80\xFD[\x81\x8B\x01\x91P\x8B`\x1F\x83\x01\x12aBdW`\0\x80\xFD[\x815\x81\x81\x11\x15aBsW`\0\x80\xFD[\x8C` \x82`\x07\x1B\x85\x01\x01\x11\x15aB\x88W`\0\x80\xFD[` \x83\x01\x95P\x80\x94PP`\xA0\x8B\x015\x91P\x80\x82\x11\x15aB\xA6W`\0\x80\xFD[PaB\xB3\x8B\x82\x8C\x01aARV[\x91PP\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\t\xD4W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15aB\xEAW`\0\x80\xFD[\x835\x92P` \x84\x015aB\xFC\x81aB\xC3V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15aC\"W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x81\x11\x15aC8W`\0\x80\xFD[aCD\x86\x82\x87\x01a@\xC3V[\x90\x94P\x92PaCX\x90P\x85` \x86\x01aA\x04V[\x90P\x92P\x92P\x92V[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15aCzWaCza?aV[P`\x05\x1B` \x01\x90V[a\xFF\xFF\x81\x16\x81\x14a\t\xD4W`\0\x80\xFD[`\0``\x82\x84\x03\x12\x15aC\xA6W`\0\x80\xFD[aC\xAEa?wV[\x90P\x815aC\xBB\x81aB\xC3V[\x81R` \x82\x015aC\xCB\x81aC\x84V[` \x82\x01R`@\x82\x015aC\xDE\x81aC\x84V[`@\x82\x01R\x92\x91PPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15aD\x01W`\0\x80\xFD[\x855aD\x0C\x81a@\x81V[\x94P` \x86\x81\x015aD\x1D\x81a@\x81V[\x94P`@\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aD8W`\0\x80\xFD[\x87\x01`\x1F\x81\x01\x89\x13aDIW`\0\x80\xFD[\x805aD\\aDW\x82aCaV[a?\x9FV[\x81\x81R``\x91\x82\x02\x83\x01\x84\x01\x91\x84\x82\x01\x91\x90\x8C\x84\x11\x15aD{W`\0\x80\xFD[\x93\x85\x01\x93[\x83\x85\x10\x15aD\xA1WaD\x92\x8D\x86aC\x94V[\x83R\x93\x84\x01\x93\x91\x85\x01\x91aD\x80V[\x81\x98PaD\xAF\x81\x8D\x01a@\x96V[\x9A\x9D\x99\x9CP\x97\x9A`\x80\x015\x98\x97PPPPPPPPV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15aD\xDCW`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aD\xF3W`\0\x80\xFD[aD\xFF\x88\x83\x89\x01a@\xC3V[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15aE\x18W`\0\x80\xFD[PaE%\x87\x82\x88\x01a@\xC3V[\x95\x98\x94\x97P\x95PPPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x03\x81\x10aEeWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[\x81Q\x81R` \x80\x83\x01Q`@\x83\x01\x91aE\x84\x90\x84\x01\x82aEGV[P\x92\x91PPV[\x805`\xFF\x81\x16\x81\x14a@\xA1W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aE\xAEW`\0\x80\xFD[a@F\x82aE\x8BV[`\0\x80`\x80\x83\x85\x03\x12\x15aE\xCAW`\0\x80\xFD[aE\xD3\x83aE\x8BV[\x91PaE\xE2\x84` \x85\x01aC\x94V[\x90P\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15aE\xFEW`\0\x80\xFD[\x825aF\t\x81aB\xC3V[\x91P` \x83\x81\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aF%W`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13aF6W`\0\x80\xFD[\x805aFDaDW\x82aCaV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x88\x83\x11\x15aFcW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aF\x81W\x835\x82R\x92\x84\x01\x92\x90\x84\x01\x90aFhV[\x80\x95PPPPPP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aF\xCEW\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aF\xACV[P\x90\x96\x95PPPPPPV[`\0`\x80\x82\x84\x03\x12\x15aF\xECW`\0\x80\xFD[aF\xF4a?wV[\x90PaF\xFF\x82aE\x8BV[\x81R` \x82\x015aG\x0F\x81a@\x81V[` \x82\x01RaC\xDE\x83`@\x84\x01aA\x04V[`\0\x80`\0\x80`\x80\x80\x86\x88\x03\x12\x15aG8W`\0\x80\xFD[\x855\x94P` \x80\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aGVW`\0\x80\xFD[\x87\x01`\x1F\x81\x01\x89\x13aGgW`\0\x80\xFD[\x805aGuaDW\x82aCaV[\x81\x81R`\x07\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x8B\x83\x11\x15aG\x94W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aG\xBAWaG\xAB\x8C\x85aF\xDAV[\x82R\x92\x85\x01\x92\x90\x84\x01\x90aG\x99V[\x98\x9B\x98\x9APPPP`@\x87\x015\x96``\x015\x95\x94PPPPPV[`\0\x80`@\x83\x85\x03\x12\x15aG\xE8W`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x14aH\x06W`\0\x80\xFD[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aH'W`\0\x80\xFD[\x845aH2\x81a@\x81V[\x93P` \x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aHMW`\0\x80\xFD[aHY\x87\x82\x88\x01a@\xC3V[\x90\x94P\x92PaHm\x90P\x86`@\x87\x01aA\x04V[\x90P\x92\x95\x91\x94P\x92PV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15aH\x90W`\0\x80\xFD[\x855`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aH\xA7W`\0\x80\xFD[aH\xB3\x89\x83\x8A\x01a@\xC3V[\x90\x97P\x95P\x85\x91PaH\xC8\x89` \x8A\x01aA\x04V[\x94P``\x88\x015\x91P\x80\x82\x11\x15aH\xDEW`\0\x80\xFD[PaH\xEB\x88\x82\x89\x01a@\xC3V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15aI\x0FW`\0\x80\xFD[\x825aI\x1A\x81a@\x81V[\x94` \x93\x90\x93\x015\x93PPPV[` \x81\x01a\x08\x1D\x82\x84aEGV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0[\x83\x81\x10\x15aIgW\x81\x81\x01Q\x83\x82\x01R` \x01aIOV[\x83\x81\x11\x15a\x17/WPP`\0\x91\x01RV[`\0\x81Q\x80\x84RaI\x90\x81` \x86\x01` \x86\x01aILV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a@F` \x83\x01\x84aIxV[`\0` \x82\x84\x03\x12\x15aI\xC9W`\0\x80\xFD[\x81Qa@F\x81a@\x81V[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15aJ0W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a@FW`\0\x80\xFD[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`\x19\x90\x82\x01R\x7FPausable: index is paused\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`\0`\x80\x82\x84\x03\x12\x15aJ\xD1W`\0\x80\xFD[a@F\x83\x83aF\xDAV[`\0` \x82\x84\x03\x12\x15aJ\xEDW`\0\x80\xFD[\x81Q`\x01`\x01``\x1B\x03\x81\x16\x81\x14a@FW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aK@WaK@aK\x04V[\x02\x94\x93PPPPV[`\0`\x01`\x01``\x1B\x03\x80\x84\x16\x80aKqWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[\x92\x16\x91\x90\x91\x04\x92\x91PPV[`\0`\0\x19\x82\x14\x15aK\x91WaK\x91aK\x04V[P`\x01\x01\x90V[`\0\x82\x19\x82\x11\x15aK\xABWaK\xABaK\x04V[P\x01\x90V[`\0\x80\x85\x85\x11\x15aK\xC0W`\0\x80\xFD[\x83\x86\x11\x15aK\xCDW`\0\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[`\0`@\x82\x84\x03\x12\x15aK\xECW`\0\x80\xFD[a@F\x83\x83aA\x04V[` \x80\x82R`b\x90\x82\x01R`\0\x80Q` aP$\x839\x81Q\x91R`@\x82\x01R\x7Fs.onlyServiceManagerOwner: calle``\x82\x01R\x7Fr is not the service manager own`\x80\x82\x01Ra2\xB9`\xF1\x1B`\xA0\x82\x01R`\xC0\x01\x90V[`\0\x82\x82\x10\x15aL\x84WaL\x84aK\x04V[P\x03\x90V[`\0` \x82\x84\x03\x12\x15aL\x9BW`\0\x80\xFD[\x81Qa@F\x81aC\x84V[`\0`\xFF\x82\x16`\xFF\x81\x14\x15aL\xBDWaL\xBDaK\x04V[`\x01\x01\x92\x91PPV[`\0\x80``\x83\x85\x03\x12\x15aL\xD9W`\0\x80\xFD[aL\xE3\x84\x84aA\x04V[\x91P`@\x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aL\xFEW`\0\x80\xFD[aM\n\x85\x82\x86\x01a@&V[\x91PP\x92P\x92\x90PV[`\0c\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15aM1WaM1aK\x04V[\x03\x93\x92PPPV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x14\x15aMSWaMSaK\x04V[`\x01\x01\x93\x92PPPV[`\0`\xA0\x82\x01\x87\x83R` \x87\x81\x85\x01R`@`\xA0\x81\x86\x01R\x82\x88Q\x80\x85R`\xC0\x87\x01\x91P\x83\x8A\x01\x94P`\0[\x81\x81\x10\x15aM\xD8W\x85Q\x80Q`\xFF\x16\x84R\x85\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x86\x85\x01R\x84\x01QaM\xC4\x85\x85\x01\x82\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[P\x94\x84\x01\x94`\x80\x92\x90\x92\x01\x91`\x01\x01aM\x89V[PP``\x86\x01\x97\x90\x97RPPPP`\x80\x01R\x93\x92PPPV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x85\x16\x81R`\x80` \x82\x01\x81\x90R`\0\x90aN?\x90\x83\x01\x85\x87aM\xF1V[\x83Q`@\x84\x01R` \x84\x01Q``\x84\x01R\x90Pa+XV[`\0` \x82\x84\x03\x12\x15aNiW`\0\x80\xFD[PQ\x91\x90PV[`\x01\x80`\xA0\x1B\x03\x85\x16\x81R\x83` \x82\x01R```@\x82\x01R`\0aN\x98``\x83\x01\x84\x86aM\xF1V[\x96\x95PPPPPPV[\x83\x81R`@` \x82\x01R`\0a p`@\x83\x01\x84\x86aM\xF1V[`\0` \x80\x83\x85\x03\x12\x15aN\xCFW`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15aN\xE5W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aN\xF6W`\0\x80\xFD[\x80QaO\x04aDW\x82aCaV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15aO#W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15aOJW\x83QaO;\x81aB\xC3V[\x82R\x92\x84\x01\x92\x90\x84\x01\x90aO(V[\x97\x96PPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R`\x80` \x82\x01\x81\x90R`\0\x90aOy\x90\x83\x01\x85aIxV[\x83Q`@\x84\x01R` \x84\x01Q``\x84\x01R\x90Pa sV[\x82\x81R`@` \x82\x01R`\0a s`@\x83\x01\x84aIxV[`\0` \x82\x84\x03\x12\x15aO\xBCW`\0\x80\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a@FW`\0\x80\xFD[`\0\x83QaO\xE6\x81\x84` \x88\x01aILV[`\x01`\x01`\xF8\x1B\x03\x19\x93\x90\x93\x16\x91\x90\x92\x01\x90\x81R`\x01\x01\x92\x91PPV\xFEs._registerOperatorWithCoordinatBLSRegistryCoordinatorWithIndice\xA2dipfsX\"\x12 h\xF7].\xB5!x:0%A\xC5h\xD9-n\x11\x87w\xEB2\xD0\xA6\"t\xAF\xE6*U^x\x1EdsolcC\0\x08\x0C\x003";
    /// The deployed bytecode of the contract.
    pub static BLSREGISTRYCOORDINATORWITHINDICESHARNESS_DEPLOYED_BYTECODE:
        ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct BLSRegistryCoordinatorWithIndicesHarness<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for BLSRegistryCoordinatorWithIndicesHarness<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for BLSRegistryCoordinatorWithIndicesHarness<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for BLSRegistryCoordinatorWithIndicesHarness<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for BLSRegistryCoordinatorWithIndicesHarness<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(BLSRegistryCoordinatorWithIndicesHarness))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> BLSRegistryCoordinatorWithIndicesHarness<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                BLSREGISTRYCOORDINATORWITHINDICESHARNESS_ABI.clone(),
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
                BLSREGISTRYCOORDINATORWITHINDICESHARNESS_ABI.clone(),
                BLSREGISTRYCOORDINATORWITHINDICESHARNESS_BYTECODE
                    .clone()
                    .into(),
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
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
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
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
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
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
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
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
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
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([136, 111, 17, 149], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `recordOperatorQuorumBitmapUpdate` (0xb77ec1d3) function
        pub fn record_operator_quorum_bitmap_update(
            &self,
            operator_id: [u8; 32],
            quorum_bitmap: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([183, 126, 193, 211], (operator_id, quorum_bitmap))
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
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([99, 71, 201, 0], p0)
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
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([177, 52, 66, 113], ())
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
        ///Gets the contract's `Unpaused` event
        pub fn unpaused_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, UnpausedFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BLSRegistryCoordinatorWithIndicesHarnessEvents,
        > {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for BLSRegistryCoordinatorWithIndicesHarness<M>
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
    pub enum BLSRegistryCoordinatorWithIndicesHarnessEvents {
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
    impl ::ethers::contract::EthLogDecode for BLSRegistryCoordinatorWithIndicesHarnessEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ChurnApproverUpdatedFilter::decode_log(log) {
                return Ok(
                    BLSRegistryCoordinatorWithIndicesHarnessEvents::ChurnApproverUpdatedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = EjectorUpdatedFilter::decode_log(log) {
                return Ok(
                    BLSRegistryCoordinatorWithIndicesHarnessEvents::EjectorUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(
                    BLSRegistryCoordinatorWithIndicesHarnessEvents::InitializedFilter(decoded),
                );
            }
            if let Ok(decoded) = OperatorDeregisteredFilter::decode_log(log) {
                return Ok(
                    BLSRegistryCoordinatorWithIndicesHarnessEvents::OperatorDeregisteredFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = OperatorRegisteredFilter::decode_log(log) {
                return Ok(
                    BLSRegistryCoordinatorWithIndicesHarnessEvents::OperatorRegisteredFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = OperatorSetParamsUpdatedFilter::decode_log(log) {
                return Ok(
                    BLSRegistryCoordinatorWithIndicesHarnessEvents::OperatorSetParamsUpdatedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = OperatorSocketUpdateFilter::decode_log(log) {
                return Ok(
                    BLSRegistryCoordinatorWithIndicesHarnessEvents::OperatorSocketUpdateFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(BLSRegistryCoordinatorWithIndicesHarnessEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = PauserRegistrySetFilter::decode_log(log) {
                return Ok(
                    BLSRegistryCoordinatorWithIndicesHarnessEvents::PauserRegistrySetFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(BLSRegistryCoordinatorWithIndicesHarnessEvents::UnpausedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for BLSRegistryCoordinatorWithIndicesHarnessEvents {
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
                Self::PausedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauserRegistrySetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnpausedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ChurnApproverUpdatedFilter>
        for BLSRegistryCoordinatorWithIndicesHarnessEvents
    {
        fn from(value: ChurnApproverUpdatedFilter) -> Self {
            Self::ChurnApproverUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<EjectorUpdatedFilter>
        for BLSRegistryCoordinatorWithIndicesHarnessEvents
    {
        fn from(value: EjectorUpdatedFilter) -> Self {
            Self::EjectorUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for BLSRegistryCoordinatorWithIndicesHarnessEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OperatorDeregisteredFilter>
        for BLSRegistryCoordinatorWithIndicesHarnessEvents
    {
        fn from(value: OperatorDeregisteredFilter) -> Self {
            Self::OperatorDeregisteredFilter(value)
        }
    }
    impl ::core::convert::From<OperatorRegisteredFilter>
        for BLSRegistryCoordinatorWithIndicesHarnessEvents
    {
        fn from(value: OperatorRegisteredFilter) -> Self {
            Self::OperatorRegisteredFilter(value)
        }
    }
    impl ::core::convert::From<OperatorSetParamsUpdatedFilter>
        for BLSRegistryCoordinatorWithIndicesHarnessEvents
    {
        fn from(value: OperatorSetParamsUpdatedFilter) -> Self {
            Self::OperatorSetParamsUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<OperatorSocketUpdateFilter>
        for BLSRegistryCoordinatorWithIndicesHarnessEvents
    {
        fn from(value: OperatorSocketUpdateFilter) -> Self {
            Self::OperatorSocketUpdateFilter(value)
        }
    }
    impl ::core::convert::From<PausedFilter> for BLSRegistryCoordinatorWithIndicesHarnessEvents {
        fn from(value: PausedFilter) -> Self {
            Self::PausedFilter(value)
        }
    }
    impl ::core::convert::From<PauserRegistrySetFilter>
        for BLSRegistryCoordinatorWithIndicesHarnessEvents
    {
        fn from(value: PauserRegistrySetFilter) -> Self {
            Self::PauserRegistrySetFilter(value)
        }
    }
    impl ::core::convert::From<UnpausedFilter> for BLSRegistryCoordinatorWithIndicesHarnessEvents {
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
    ///Container type for all input parameters for the `recordOperatorQuorumBitmapUpdate` function with signature `recordOperatorQuorumBitmapUpdate(bytes32,uint192)` and selector `0xb77ec1d3`
    #[derive(
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
        name = "recordOperatorQuorumBitmapUpdate",
        abi = "recordOperatorQuorumBitmapUpdate(bytes32,uint192)"
    )]
    pub struct RecordOperatorQuorumBitmapUpdateCall {
        pub operator_id: [u8; 32],
        pub quorum_bitmap: ::ethers::core::types::U256,
    }
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
    pub enum BLSRegistryCoordinatorWithIndicesHarnessCalls {
        OperatorChurnApprovalTypehash(OperatorChurnApprovalTypehashCall),
        BlsPubkeyRegistry(BlsPubkeyRegistryCall),
        CalculateOperatorChurnApprovalDigestHash(CalculateOperatorChurnApprovalDigestHashCall),
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
        GetQuorumBitmapUpdateByOperatorIdByIndex(GetQuorumBitmapUpdateByOperatorIdByIndexCall),
        GetQuorumBitmapUpdateByOperatorIdLength(GetQuorumBitmapUpdateByOperatorIdLengthCall),
        IndexRegistry(IndexRegistryCall),
        Initialize(InitializeCall),
        IsChurnApproverSaltUsed(IsChurnApproverSaltUsedCall),
        NumRegistries(NumRegistriesCall),
        Pause(PauseCall),
        PauseAll(PauseAllCall),
        PausedWithIndex(PausedWithIndexCall),
        Paused(PausedCall),
        PauserRegistry(PauserRegistryCall),
        RecordOperatorQuorumBitmapUpdate(RecordOperatorQuorumBitmapUpdateCall),
        RegisterOperatorWithCoordinator2(RegisterOperatorWithCoordinator2Call),
        RegisterOperatorWithCoordinator0(RegisterOperatorWithCoordinator0Call),
        RegisterOperatorWithCoordinator1(RegisterOperatorWithCoordinator1Call),
        Registries(RegistriesCall),
        ServiceManager(ServiceManagerCall),
        SetChurnApprover(SetChurnApproverCall),
        SetEjector(SetEjectorCall),
        SetOperatorId(SetOperatorIdCall),
        SetOperatorSetParams(SetOperatorSetParamsCall),
        SetPauserRegistry(SetPauserRegistryCall),
        Slasher(SlasherCall),
        StakeRegistry(StakeRegistryCall),
        Unpause(UnpauseCall),
        UpdateSocket(UpdateSocketCall),
    }
    impl ::ethers::core::abi::AbiDecode for BLSRegistryCoordinatorWithIndicesHarnessCalls {
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
                <BlsPubkeyRegistryCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BlsPubkeyRegistry(decoded));
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
            if let Ok(decoded) =
                <DeregisterOperatorWithCoordinatorCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
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
            if let Ok(decoded) =
                <EjectOperatorFromCoordinatorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EjectOperatorFromCoordinator(decoded));
            }
            if let Ok(decoded) = <EjectorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Ejector(decoded));
            }
            if let Ok(decoded) =
                <GetCurrentQuorumBitmapByOperatorIdCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetCurrentQuorumBitmapByOperatorId(decoded));
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
            if let Ok(decoded) = <NumRegistriesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NumRegistries(decoded));
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
                <RecordOperatorQuorumBitmapUpdateCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::RecordOperatorQuorumBitmapUpdate(decoded));
            }
            if let Ok(decoded) =
                <RegisterOperatorWithCoordinator2Call as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::RegisterOperatorWithCoordinator2(decoded));
            }
            if let Ok(decoded) =
                <RegisterOperatorWithCoordinator0Call as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::RegisterOperatorWithCoordinator0(decoded));
            }
            if let Ok(decoded) =
                <RegisterOperatorWithCoordinator1Call as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::RegisterOperatorWithCoordinator1(decoded));
            }
            if let Ok(decoded) = <RegistriesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Registries(decoded));
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
            if let Ok(decoded) = <SetEjectorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetEjector(decoded));
            }
            if let Ok(decoded) = <SetOperatorIdCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetOperatorId(decoded));
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
            if let Ok(decoded) = <SlasherCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Slasher(decoded));
            }
            if let Ok(decoded) = <StakeRegistryCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::StakeRegistry(decoded));
            }
            if let Ok(decoded) = <UnpauseCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Unpause(decoded));
            }
            if let Ok(decoded) = <UpdateSocketCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateSocket(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for BLSRegistryCoordinatorWithIndicesHarnessCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::OperatorChurnApprovalTypehash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BlsPubkeyRegistry(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CalculateOperatorChurnApprovalDigestHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChurnApprover(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::GetOperator(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetOperatorFromId(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetOperatorId(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetOperatorSetParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOperatorStatus(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::IndexRegistry(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsChurnApproverSaltUsed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NumRegistries(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PauseAll(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PausedWithIndex(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Paused(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PauserRegistry(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RecordOperatorQuorumBitmapUpdate(element) => {
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
                Self::Registries(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ServiceManager(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetChurnApprover(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetEjector(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetOperatorId(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetOperatorSetParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPauserRegistry(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Slasher(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StakeRegistry(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Unpause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateSocket(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for BLSRegistryCoordinatorWithIndicesHarnessCalls {
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
                Self::GetOperatorSetParams(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::IsChurnApproverSaltUsed(element) => ::core::fmt::Display::fmt(element, f),
                Self::NumRegistries(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pause(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauseAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::PausedWithIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::Paused(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauserRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::RecordOperatorQuorumBitmapUpdate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
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
                Self::SetOperatorId(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetOperatorSetParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPauserRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::Slasher(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unpause(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateSocket(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<OperatorChurnApprovalTypehashCall>
        for BLSRegistryCoordinatorWithIndicesHarnessCalls
    {
        fn from(value: OperatorChurnApprovalTypehashCall) -> Self {
            Self::OperatorChurnApprovalTypehash(value)
        }
    }
    impl ::core::convert::From<BlsPubkeyRegistryCall>
        for BLSRegistryCoordinatorWithIndicesHarnessCalls
    {
        fn from(value: BlsPubkeyRegistryCall) -> Self {
            Self::BlsPubkeyRegistry(value)
        }
    }
    impl ::core::convert::From<CalculateOperatorChurnApprovalDigestHashCall>
        for BLSRegistryCoordinatorWithIndicesHarnessCalls
    {
        fn from(value: CalculateOperatorChurnApprovalDigestHashCall) -> Self {
            Self::CalculateOperatorChurnApprovalDigestHash(value)
        }
    }
    impl ::core::convert::From<ChurnApproverCall> for BLSRegistryCoordinatorWithIndicesHarnessCalls {
        fn from(value: ChurnApproverCall) -> Self {
            Self::ChurnApprover(value)
        }
    }
    impl ::core::convert::From<DeregisterOperatorWithCoordinatorCall>
        for BLSRegistryCoordinatorWithIndicesHarnessCalls
    {
        fn from(value: DeregisterOperatorWithCoordinatorCall) -> Self {
            Self::DeregisterOperatorWithCoordinator(value)
        }
    }
    impl
        ::core::convert::From<
            DeregisterOperatorWithCoordinatorWithQuorumNumbersAndDeregistrationDataCall,
        > for BLSRegistryCoordinatorWithIndicesHarnessCalls
    {
        fn from(
            value: DeregisterOperatorWithCoordinatorWithQuorumNumbersAndDeregistrationDataCall,
        ) -> Self {
            Self::DeregisterOperatorWithCoordinatorWithQuorumNumbersAndDeregistrationData(value)
        }
    }
    impl ::core::convert::From<EjectOperatorFromCoordinatorCall>
        for BLSRegistryCoordinatorWithIndicesHarnessCalls
    {
        fn from(value: EjectOperatorFromCoordinatorCall) -> Self {
            Self::EjectOperatorFromCoordinator(value)
        }
    }
    impl ::core::convert::From<EjectorCall> for BLSRegistryCoordinatorWithIndicesHarnessCalls {
        fn from(value: EjectorCall) -> Self {
            Self::Ejector(value)
        }
    }
    impl ::core::convert::From<GetCurrentQuorumBitmapByOperatorIdCall>
        for BLSRegistryCoordinatorWithIndicesHarnessCalls
    {
        fn from(value: GetCurrentQuorumBitmapByOperatorIdCall) -> Self {
            Self::GetCurrentQuorumBitmapByOperatorId(value)
        }
    }
    impl ::core::convert::From<GetOperatorCall> for BLSRegistryCoordinatorWithIndicesHarnessCalls {
        fn from(value: GetOperatorCall) -> Self {
            Self::GetOperator(value)
        }
    }
    impl ::core::convert::From<GetOperatorFromIdCall>
        for BLSRegistryCoordinatorWithIndicesHarnessCalls
    {
        fn from(value: GetOperatorFromIdCall) -> Self {
            Self::GetOperatorFromId(value)
        }
    }
    impl ::core::convert::From<GetOperatorIdCall> for BLSRegistryCoordinatorWithIndicesHarnessCalls {
        fn from(value: GetOperatorIdCall) -> Self {
            Self::GetOperatorId(value)
        }
    }
    impl ::core::convert::From<GetOperatorSetParamsCall>
        for BLSRegistryCoordinatorWithIndicesHarnessCalls
    {
        fn from(value: GetOperatorSetParamsCall) -> Self {
            Self::GetOperatorSetParams(value)
        }
    }
    impl ::core::convert::From<GetOperatorStatusCall>
        for BLSRegistryCoordinatorWithIndicesHarnessCalls
    {
        fn from(value: GetOperatorStatusCall) -> Self {
            Self::GetOperatorStatus(value)
        }
    }
    impl ::core::convert::From<GetQuorumBitmapByOperatorIdAtBlockNumberByIndexCall>
        for BLSRegistryCoordinatorWithIndicesHarnessCalls
    {
        fn from(value: GetQuorumBitmapByOperatorIdAtBlockNumberByIndexCall) -> Self {
            Self::GetQuorumBitmapByOperatorIdAtBlockNumberByIndex(value)
        }
    }
    impl ::core::convert::From<GetQuorumBitmapIndicesByOperatorIdsAtBlockNumberCall>
        for BLSRegistryCoordinatorWithIndicesHarnessCalls
    {
        fn from(value: GetQuorumBitmapIndicesByOperatorIdsAtBlockNumberCall) -> Self {
            Self::GetQuorumBitmapIndicesByOperatorIdsAtBlockNumber(value)
        }
    }
    impl ::core::convert::From<GetQuorumBitmapUpdateByOperatorIdByIndexCall>
        for BLSRegistryCoordinatorWithIndicesHarnessCalls
    {
        fn from(value: GetQuorumBitmapUpdateByOperatorIdByIndexCall) -> Self {
            Self::GetQuorumBitmapUpdateByOperatorIdByIndex(value)
        }
    }
    impl ::core::convert::From<GetQuorumBitmapUpdateByOperatorIdLengthCall>
        for BLSRegistryCoordinatorWithIndicesHarnessCalls
    {
        fn from(value: GetQuorumBitmapUpdateByOperatorIdLengthCall) -> Self {
            Self::GetQuorumBitmapUpdateByOperatorIdLength(value)
        }
    }
    impl ::core::convert::From<IndexRegistryCall> for BLSRegistryCoordinatorWithIndicesHarnessCalls {
        fn from(value: IndexRegistryCall) -> Self {
            Self::IndexRegistry(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for BLSRegistryCoordinatorWithIndicesHarnessCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<IsChurnApproverSaltUsedCall>
        for BLSRegistryCoordinatorWithIndicesHarnessCalls
    {
        fn from(value: IsChurnApproverSaltUsedCall) -> Self {
            Self::IsChurnApproverSaltUsed(value)
        }
    }
    impl ::core::convert::From<NumRegistriesCall> for BLSRegistryCoordinatorWithIndicesHarnessCalls {
        fn from(value: NumRegistriesCall) -> Self {
            Self::NumRegistries(value)
        }
    }
    impl ::core::convert::From<PauseCall> for BLSRegistryCoordinatorWithIndicesHarnessCalls {
        fn from(value: PauseCall) -> Self {
            Self::Pause(value)
        }
    }
    impl ::core::convert::From<PauseAllCall> for BLSRegistryCoordinatorWithIndicesHarnessCalls {
        fn from(value: PauseAllCall) -> Self {
            Self::PauseAll(value)
        }
    }
    impl ::core::convert::From<PausedWithIndexCall> for BLSRegistryCoordinatorWithIndicesHarnessCalls {
        fn from(value: PausedWithIndexCall) -> Self {
            Self::PausedWithIndex(value)
        }
    }
    impl ::core::convert::From<PausedCall> for BLSRegistryCoordinatorWithIndicesHarnessCalls {
        fn from(value: PausedCall) -> Self {
            Self::Paused(value)
        }
    }
    impl ::core::convert::From<PauserRegistryCall> for BLSRegistryCoordinatorWithIndicesHarnessCalls {
        fn from(value: PauserRegistryCall) -> Self {
            Self::PauserRegistry(value)
        }
    }
    impl ::core::convert::From<RecordOperatorQuorumBitmapUpdateCall>
        for BLSRegistryCoordinatorWithIndicesHarnessCalls
    {
        fn from(value: RecordOperatorQuorumBitmapUpdateCall) -> Self {
            Self::RecordOperatorQuorumBitmapUpdate(value)
        }
    }
    impl ::core::convert::From<RegisterOperatorWithCoordinator2Call>
        for BLSRegistryCoordinatorWithIndicesHarnessCalls
    {
        fn from(value: RegisterOperatorWithCoordinator2Call) -> Self {
            Self::RegisterOperatorWithCoordinator2(value)
        }
    }
    impl ::core::convert::From<RegisterOperatorWithCoordinator0Call>
        for BLSRegistryCoordinatorWithIndicesHarnessCalls
    {
        fn from(value: RegisterOperatorWithCoordinator0Call) -> Self {
            Self::RegisterOperatorWithCoordinator0(value)
        }
    }
    impl ::core::convert::From<RegisterOperatorWithCoordinator1Call>
        for BLSRegistryCoordinatorWithIndicesHarnessCalls
    {
        fn from(value: RegisterOperatorWithCoordinator1Call) -> Self {
            Self::RegisterOperatorWithCoordinator1(value)
        }
    }
    impl ::core::convert::From<RegistriesCall> for BLSRegistryCoordinatorWithIndicesHarnessCalls {
        fn from(value: RegistriesCall) -> Self {
            Self::Registries(value)
        }
    }
    impl ::core::convert::From<ServiceManagerCall> for BLSRegistryCoordinatorWithIndicesHarnessCalls {
        fn from(value: ServiceManagerCall) -> Self {
            Self::ServiceManager(value)
        }
    }
    impl ::core::convert::From<SetChurnApproverCall> for BLSRegistryCoordinatorWithIndicesHarnessCalls {
        fn from(value: SetChurnApproverCall) -> Self {
            Self::SetChurnApprover(value)
        }
    }
    impl ::core::convert::From<SetEjectorCall> for BLSRegistryCoordinatorWithIndicesHarnessCalls {
        fn from(value: SetEjectorCall) -> Self {
            Self::SetEjector(value)
        }
    }
    impl ::core::convert::From<SetOperatorIdCall> for BLSRegistryCoordinatorWithIndicesHarnessCalls {
        fn from(value: SetOperatorIdCall) -> Self {
            Self::SetOperatorId(value)
        }
    }
    impl ::core::convert::From<SetOperatorSetParamsCall>
        for BLSRegistryCoordinatorWithIndicesHarnessCalls
    {
        fn from(value: SetOperatorSetParamsCall) -> Self {
            Self::SetOperatorSetParams(value)
        }
    }
    impl ::core::convert::From<SetPauserRegistryCall>
        for BLSRegistryCoordinatorWithIndicesHarnessCalls
    {
        fn from(value: SetPauserRegistryCall) -> Self {
            Self::SetPauserRegistry(value)
        }
    }
    impl ::core::convert::From<SlasherCall> for BLSRegistryCoordinatorWithIndicesHarnessCalls {
        fn from(value: SlasherCall) -> Self {
            Self::Slasher(value)
        }
    }
    impl ::core::convert::From<StakeRegistryCall> for BLSRegistryCoordinatorWithIndicesHarnessCalls {
        fn from(value: StakeRegistryCall) -> Self {
            Self::StakeRegistry(value)
        }
    }
    impl ::core::convert::From<UnpauseCall> for BLSRegistryCoordinatorWithIndicesHarnessCalls {
        fn from(value: UnpauseCall) -> Self {
            Self::Unpause(value)
        }
    }
    impl ::core::convert::From<UpdateSocketCall> for BLSRegistryCoordinatorWithIndicesHarnessCalls {
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    pub struct GetQuorumBitmapIndicesByOperatorIdsAtBlockNumberReturn(pub ::std::vec::Vec<u32>);
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
        Hash,
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
        Hash,
    )]
    pub struct GetQuorumBitmapUpdateByOperatorIdLengthReturn(pub ::ethers::core::types::U256);
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
        Hash,
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
        Hash,
    )]
    pub struct StakeRegistryReturn(pub ::ethers::core::types::Address);
}
