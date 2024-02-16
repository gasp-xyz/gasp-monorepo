pub use ibls_registry_coordinator_with_indices::*;
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
pub mod ibls_registry_coordinator_with_indices {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("blsPubkeyRegistry"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("blsPubkeyRegistry"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract IBLSPubkeyRegistry",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deregisterOperatorWithCoordinator"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("deregisterOperatorWithCoordinator",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("quorumNumbers"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("deregistrationData",),
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
                    ::std::borrow::ToOwned::to_owned("ejectOperatorFromCoordinator"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("ejectOperatorFromCoordinator",),
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
                (
                    ::std::borrow::ToOwned::to_owned("getCurrentQuorumBitmapByOperatorId"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "getCurrentQuorumBitmapByOperatorId",
                        ),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("operatorId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(192usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint192"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getOperator"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getOperator"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("operator"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IRegistryCoordinator.Operator",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getOperatorFromId"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getOperatorFromId"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("operatorId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("operator"),
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
                    ::std::borrow::ToOwned::to_owned("getOperatorSetParams"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getOperatorSetParams",),
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
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IBLSRegistryCoordinatorWithIndices.OperatorSetParam",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getOperatorStatus"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getOperatorStatus"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("operator"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "enum IRegistryCoordinator.OperatorStatus",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "getQuorumBitmapByOperatorIdAtBlockNumberByIndex",
                    ),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "getQuorumBitmapByOperatorIdAtBlockNumberByIndex",
                        ),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(192usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint192"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "getQuorumBitmapIndicesByOperatorIdsAtBlockNumber",
                    ),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
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
                    ::std::borrow::ToOwned::to_owned("getQuorumBitmapUpdateByOperatorIdByIndex"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "getQuorumBitmapUpdateByOperatorIdByIndex",
                        ),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
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
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(192usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IRegistryCoordinator.QuorumBitmapUpdate",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getQuorumBitmapUpdateByOperatorIdLength"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "getQuorumBitmapUpdateByOperatorIdLength",
                        ),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("operatorId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("indexRegistry"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("indexRegistry"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract IIndexRegistry"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("numRegistries"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("numRegistries"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("registerOperatorWithCoordinator"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("registerOperatorWithCoordinator",),
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
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("registries"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("registries"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("stakeRegistry"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("stakeRegistry"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract IStakeRegistry"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ChurnApproverUpdated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ChurnApproverUpdated",),
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
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EjectorUpdated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
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
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OperatorDeregistered"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("OperatorDeregistered",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OperatorRegistered"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("OperatorRegistered"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operatorId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OperatorSetParamsUpdated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("OperatorSetParamsUpdated",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("quorumNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operatorSetParams"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                ],),
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
    pub static IBLSREGISTRYCOORDINATORWITHINDICES_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct IBLSRegistryCoordinatorWithIndices<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IBLSRegistryCoordinatorWithIndices<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IBLSRegistryCoordinatorWithIndices<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IBLSRegistryCoordinatorWithIndices<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IBLSRegistryCoordinatorWithIndices<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IBLSRegistryCoordinatorWithIndices))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IBLSRegistryCoordinatorWithIndices<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                IBLSREGISTRYCOORDINATORWITHINDICES_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `blsPubkeyRegistry` (0x3561deb1) function
        pub fn bls_pubkey_registry(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([53, 97, 222, 177], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deregisterOperatorWithCoordinator` (0xc81b1ff4) function
        pub fn deregister_operator_with_coordinator(
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
        ///Calls the contract's `numRegistries` (0xd72d8dd6) function
        pub fn num_registries(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([215, 45, 141, 214], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerOperatorWithCoordinator` (0x526ea94e) function
        pub fn register_operator_with_coordinator(
            &self,
            quorum_numbers: ::ethers::core::types::Bytes,
            registration_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([82, 110, 169, 78], (quorum_numbers, registration_data))
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
        ///Calls the contract's `stakeRegistry` (0x68304835) function
        pub fn stake_registry(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([104, 48, 72, 53], ())
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
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IBLSRegistryCoordinatorWithIndicesEvents,
        > {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for IBLSRegistryCoordinatorWithIndices<M>
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
    pub enum IBLSRegistryCoordinatorWithIndicesEvents {
        ChurnApproverUpdatedFilter(ChurnApproverUpdatedFilter),
        EjectorUpdatedFilter(EjectorUpdatedFilter),
        OperatorDeregisteredFilter(OperatorDeregisteredFilter),
        OperatorRegisteredFilter(OperatorRegisteredFilter),
        OperatorSetParamsUpdatedFilter(OperatorSetParamsUpdatedFilter),
    }
    impl ::ethers::contract::EthLogDecode for IBLSRegistryCoordinatorWithIndicesEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ChurnApproverUpdatedFilter::decode_log(log) {
                return Ok(
                    IBLSRegistryCoordinatorWithIndicesEvents::ChurnApproverUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = EjectorUpdatedFilter::decode_log(log) {
                return Ok(IBLSRegistryCoordinatorWithIndicesEvents::EjectorUpdatedFilter(decoded));
            }
            if let Ok(decoded) = OperatorDeregisteredFilter::decode_log(log) {
                return Ok(
                    IBLSRegistryCoordinatorWithIndicesEvents::OperatorDeregisteredFilter(decoded),
                );
            }
            if let Ok(decoded) = OperatorRegisteredFilter::decode_log(log) {
                return Ok(
                    IBLSRegistryCoordinatorWithIndicesEvents::OperatorRegisteredFilter(decoded),
                );
            }
            if let Ok(decoded) = OperatorSetParamsUpdatedFilter::decode_log(log) {
                return Ok(
                    IBLSRegistryCoordinatorWithIndicesEvents::OperatorSetParamsUpdatedFilter(
                        decoded,
                    ),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IBLSRegistryCoordinatorWithIndicesEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ChurnApproverUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::EjectorUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorDeregisteredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorRegisteredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorSetParamsUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ChurnApproverUpdatedFilter>
        for IBLSRegistryCoordinatorWithIndicesEvents
    {
        fn from(value: ChurnApproverUpdatedFilter) -> Self {
            Self::ChurnApproverUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<EjectorUpdatedFilter> for IBLSRegistryCoordinatorWithIndicesEvents {
        fn from(value: EjectorUpdatedFilter) -> Self {
            Self::EjectorUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<OperatorDeregisteredFilter>
        for IBLSRegistryCoordinatorWithIndicesEvents
    {
        fn from(value: OperatorDeregisteredFilter) -> Self {
            Self::OperatorDeregisteredFilter(value)
        }
    }
    impl ::core::convert::From<OperatorRegisteredFilter> for IBLSRegistryCoordinatorWithIndicesEvents {
        fn from(value: OperatorRegisteredFilter) -> Self {
            Self::OperatorRegisteredFilter(value)
        }
    }
    impl ::core::convert::From<OperatorSetParamsUpdatedFilter>
        for IBLSRegistryCoordinatorWithIndicesEvents
    {
        fn from(value: OperatorSetParamsUpdatedFilter) -> Self {
            Self::OperatorSetParamsUpdatedFilter(value)
        }
    }
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
    pub struct DeregisterOperatorWithCoordinatorCall {
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
    pub struct RegisterOperatorWithCoordinatorCall {
        pub quorum_numbers: ::ethers::core::types::Bytes,
        pub registration_data: ::ethers::core::types::Bytes,
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
    pub enum IBLSRegistryCoordinatorWithIndicesCalls {
        BlsPubkeyRegistry(BlsPubkeyRegistryCall),
        DeregisterOperatorWithCoordinator(DeregisterOperatorWithCoordinatorCall),
        EjectOperatorFromCoordinator(EjectOperatorFromCoordinatorCall),
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
        NumRegistries(NumRegistriesCall),
        RegisterOperatorWithCoordinator(RegisterOperatorWithCoordinatorCall),
        Registries(RegistriesCall),
        StakeRegistry(StakeRegistryCall),
    }
    impl ::ethers::core::abi::AbiDecode for IBLSRegistryCoordinatorWithIndicesCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <BlsPubkeyRegistryCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BlsPubkeyRegistry(decoded));
            }
            if let Ok(decoded) =
                <DeregisterOperatorWithCoordinatorCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::DeregisterOperatorWithCoordinator(decoded));
            }
            if let Ok(decoded) =
                <EjectOperatorFromCoordinatorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EjectOperatorFromCoordinator(decoded));
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
            if let Ok(decoded) = <NumRegistriesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NumRegistries(decoded));
            }
            if let Ok(decoded) =
                <RegisterOperatorWithCoordinatorCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::RegisterOperatorWithCoordinator(decoded));
            }
            if let Ok(decoded) = <RegistriesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Registries(decoded));
            }
            if let Ok(decoded) = <StakeRegistryCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::StakeRegistry(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IBLSRegistryCoordinatorWithIndicesCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BlsPubkeyRegistry(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DeregisterOperatorWithCoordinator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EjectOperatorFromCoordinator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
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
                Self::NumRegistries(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RegisterOperatorWithCoordinator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Registries(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StakeRegistry(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for IBLSRegistryCoordinatorWithIndicesCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BlsPubkeyRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeregisterOperatorWithCoordinator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EjectOperatorFromCoordinator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
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
                Self::NumRegistries(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterOperatorWithCoordinator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Registries(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeRegistry(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BlsPubkeyRegistryCall> for IBLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: BlsPubkeyRegistryCall) -> Self {
            Self::BlsPubkeyRegistry(value)
        }
    }
    impl ::core::convert::From<DeregisterOperatorWithCoordinatorCall>
        for IBLSRegistryCoordinatorWithIndicesCalls
    {
        fn from(value: DeregisterOperatorWithCoordinatorCall) -> Self {
            Self::DeregisterOperatorWithCoordinator(value)
        }
    }
    impl ::core::convert::From<EjectOperatorFromCoordinatorCall>
        for IBLSRegistryCoordinatorWithIndicesCalls
    {
        fn from(value: EjectOperatorFromCoordinatorCall) -> Self {
            Self::EjectOperatorFromCoordinator(value)
        }
    }
    impl ::core::convert::From<GetCurrentQuorumBitmapByOperatorIdCall>
        for IBLSRegistryCoordinatorWithIndicesCalls
    {
        fn from(value: GetCurrentQuorumBitmapByOperatorIdCall) -> Self {
            Self::GetCurrentQuorumBitmapByOperatorId(value)
        }
    }
    impl ::core::convert::From<GetOperatorCall> for IBLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: GetOperatorCall) -> Self {
            Self::GetOperator(value)
        }
    }
    impl ::core::convert::From<GetOperatorFromIdCall> for IBLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: GetOperatorFromIdCall) -> Self {
            Self::GetOperatorFromId(value)
        }
    }
    impl ::core::convert::From<GetOperatorIdCall> for IBLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: GetOperatorIdCall) -> Self {
            Self::GetOperatorId(value)
        }
    }
    impl ::core::convert::From<GetOperatorSetParamsCall> for IBLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: GetOperatorSetParamsCall) -> Self {
            Self::GetOperatorSetParams(value)
        }
    }
    impl ::core::convert::From<GetOperatorStatusCall> for IBLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: GetOperatorStatusCall) -> Self {
            Self::GetOperatorStatus(value)
        }
    }
    impl ::core::convert::From<GetQuorumBitmapByOperatorIdAtBlockNumberByIndexCall>
        for IBLSRegistryCoordinatorWithIndicesCalls
    {
        fn from(value: GetQuorumBitmapByOperatorIdAtBlockNumberByIndexCall) -> Self {
            Self::GetQuorumBitmapByOperatorIdAtBlockNumberByIndex(value)
        }
    }
    impl ::core::convert::From<GetQuorumBitmapIndicesByOperatorIdsAtBlockNumberCall>
        for IBLSRegistryCoordinatorWithIndicesCalls
    {
        fn from(value: GetQuorumBitmapIndicesByOperatorIdsAtBlockNumberCall) -> Self {
            Self::GetQuorumBitmapIndicesByOperatorIdsAtBlockNumber(value)
        }
    }
    impl ::core::convert::From<GetQuorumBitmapUpdateByOperatorIdByIndexCall>
        for IBLSRegistryCoordinatorWithIndicesCalls
    {
        fn from(value: GetQuorumBitmapUpdateByOperatorIdByIndexCall) -> Self {
            Self::GetQuorumBitmapUpdateByOperatorIdByIndex(value)
        }
    }
    impl ::core::convert::From<GetQuorumBitmapUpdateByOperatorIdLengthCall>
        for IBLSRegistryCoordinatorWithIndicesCalls
    {
        fn from(value: GetQuorumBitmapUpdateByOperatorIdLengthCall) -> Self {
            Self::GetQuorumBitmapUpdateByOperatorIdLength(value)
        }
    }
    impl ::core::convert::From<IndexRegistryCall> for IBLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: IndexRegistryCall) -> Self {
            Self::IndexRegistry(value)
        }
    }
    impl ::core::convert::From<NumRegistriesCall> for IBLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: NumRegistriesCall) -> Self {
            Self::NumRegistries(value)
        }
    }
    impl ::core::convert::From<RegisterOperatorWithCoordinatorCall>
        for IBLSRegistryCoordinatorWithIndicesCalls
    {
        fn from(value: RegisterOperatorWithCoordinatorCall) -> Self {
            Self::RegisterOperatorWithCoordinator(value)
        }
    }
    impl ::core::convert::From<RegistriesCall> for IBLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: RegistriesCall) -> Self {
            Self::Registries(value)
        }
    }
    impl ::core::convert::From<StakeRegistryCall> for IBLSRegistryCoordinatorWithIndicesCalls {
        fn from(value: StakeRegistryCall) -> Self {
            Self::StakeRegistry(value)
        }
    }
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
    pub struct GetOperatorFromIdReturn {
        pub operator: ::ethers::core::types::Address,
    }
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
