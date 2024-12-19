pub use finalizer_service_manager::*;
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
pub mod finalizer_service_manager {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_avsDirectory"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IAVSDirectory"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_rewardsCoordinator"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "contract IRewardsCoordinator",
                            ),
                        ),
                    },
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
                        name: ::std::borrow::ToOwned::to_owned("_stakeRegistry"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IStakeRegistry"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_taskManager"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "contract IFinalizerTaskManager",
                            ),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned(
                            "_recurrentRegistrationBlocksLimit",
                        ),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint64"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("avsDirectory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("avsDirectory"),
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
                    ::std::borrow::ToOwned::to_owned("createAVSRewardsSubmission"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "createAVSRewardsSubmission",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "rewardsSubmissions",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
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
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IRewardsCoordinator.RewardsSubmission[]",
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
                    ::std::borrow::ToOwned::to_owned("deregisterOperatorFromAVS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "deregisterOperatorFromAVS",
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ejectOperators"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ejectOperators"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumbers"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
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
                    ::std::borrow::ToOwned::to_owned("getOperatorRestakedStrategies"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getOperatorRestakedStrategies",
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
                    ::std::borrow::ToOwned::to_owned("getRestakeableStrategies"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getRestakeableStrategies",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("initialOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_rewardsInitiator"),
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
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                    ::std::borrow::ToOwned::to_owned("recurrentRegistrationBlocksLimit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "recurrentRegistrationBlocksLimit",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("registerOperatorToAVS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "registerOperatorToAVS",
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
                    ::std::borrow::ToOwned::to_owned("rewardsInitiator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rewardsInitiator"),
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
                    ::std::borrow::ToOwned::to_owned("setRewardsInitiator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setRewardsInitiator",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newRewardsInitiator",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("taskManager"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("taskManager"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IFinalizerTaskManager",
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
                    ::std::borrow::ToOwned::to_owned("updateAVSMetadataURI"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateAVSMetadataURI",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_metadataURI"),
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
                    ::std::borrow::ToOwned::to_owned("RewardsInitiatorUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RewardsInitiatorUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "prevRewardsInitiator",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newRewardsInitiator",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
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
    pub static FINALIZERSERVICEMANAGER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01@`@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@Qb\0&\xFC8\x03\x80b\0&\xFC\x839\x81\x01`@\x81\x90Rb\0\x005\x91b\0\x01hV[`\x01`\x01`\xA0\x1B\x03\x80\x87\x16`\x80R\x80\x86\x16`\xA0R\x80\x85\x16`\xC0R\x83\x16`\xE0R\x85\x85\x85\x85b\0\0bb\0\0\x8DV[PPP`\x01`\x01`\xA0\x1B\x03\x90\x92\x16a\x01\0R`\x01`\x01`@\x1B\x03\x16a\x01 RPb\0\x02\x07\x93PPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15b\0\0\xFAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\xFF\x90\x81\x16\x10\x15b\0\x01MW`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01eW`\0\x80\xFD[PV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15b\0\x01\x82W`\0\x80\xFD[\x86Qb\0\x01\x8F\x81b\0\x01OV[` \x88\x01Q\x90\x96Pb\0\x01\xA2\x81b\0\x01OV[`@\x88\x01Q\x90\x95Pb\0\x01\xB5\x81b\0\x01OV[``\x88\x01Q\x90\x94Pb\0\x01\xC8\x81b\0\x01OV[`\x80\x88\x01Q\x90\x93Pb\0\x01\xDB\x81b\0\x01OV[`\xA0\x88\x01Q\x90\x92P`\x01`\x01`@\x1B\x03\x81\x16\x81\x14b\0\x01\xF9W`\0\x80\xFD[\x80\x91PP\x92\x95P\x92\x95P\x92\x95V[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa$\x12b\0\x02\xEA`\09`\0\x81\x81a\x01\xAB\x01Ra\x0B\xFF\x01R`\0a\x02O\x01R`\0\x81\x81a\x06\xCF\x01R\x81\x81a\x08*\x01R\x81\x81a\x08\xC1\x01R\x81\x81a\x0BV\x01R\x81\x81a\x10\x80\x01R\x81\x81a\x12\x03\x01Ra\x12\xA2\x01R`\0\x81\x81a\x03\xDF\x01R\x81\x81a\x04\xFA\x01R\x81\x81a\x05\x89\x01R\x81\x81a\x06\t\x01R\x81\x81a\t\xD4\x01R\x81\x81a\n3\x01R\x81\x81a\n\xA7\x01R\x81\x81a\r{\x01R\x81\x81a\x0F\xBB\x01Ra\x11^\x01R`\0\x81\x81a\x15C\x01R\x81\x81a\x15\xFF\x01Ra\x16\xEB\x01R`\0\x81\x81a\x01\xE7\x01R\x81\x81a\r\x02\x01R\x81\x81a\r\xD7\x01Ra\x0EO\x01Ra$\x12`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\x16W`\x005`\xE0\x1C\x80c\x99&\xEE}\x11a\0\xA2W\x80c\xC0\xC5;\x8B\x11a\0qW\x80c\xC0\xC5;\x8B\x14a\x02\x84W\x80c\xE4\x81\xAF\x9D\x14a\x02\x97W\x80c\xF2\xFD\xE3\x8B\x14a\x02\x9FW\x80c\xFC)\x9D\xEE\x14a\x02\xB2W\x80c\xFC\xE3l}\x14a\x02\xC5W`\0\x80\xFD[\x80c\x99&\xEE}\x14a\x02$W\x80c\xA3d\xF4\xDA\x14a\x027W\x80c\xA5\nd\x0E\x14a\x02JW\x80c\xA9\x8F\xB3U\x14a\x02qW`\0\x80\xFD[\x80c;\xC2\x8C\x8C\x11a\0\xE9W\x80c;\xC2\x8C\x8C\x14a\x01\x93W\x80caL\xC1D\x14a\x01\xA6W\x80ck:\xA7.\x14a\x01\xE5W\x80cqP\x18\xA6\x14a\x02\x0BW\x80c\x8D\xA5\xCB[\x14a\x02\x13W`\0\x80\xFD[\x80c\x1E%\xAB\xFD\x14a\x01\x1BW\x80c(\xF6\x1B1\x14a\x010W\x80c,\xDD\x1E\x86\x14a\x01`W\x80c3\xCF\xB7\xB7\x14a\x01sW[`\0\x80\xFD[a\x01.a\x01)6`\x04a\x1B*V[a\x02\xD8V[\0[`\x97Ta\x01C\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01.a\x01n6`\x04a\x1B\xAAV[a\x04\xC1V[a\x01\x86a\x01\x816`\x04a\x1B\xAAV[a\x04\xD5V[`@Qa\x01W\x91\x90a\x1B\xCEV[a\x01.a\x01\xA16`\x04a\x1B\xAAV[a\t\xA4V[a\x01\xCD\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x01WV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01CV[a\x01.a\t\xB5V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x01CV[a\x01.a\x0226`\x04a\x1C\xCEV[a\t\xC9V[a\x01.a\x02E6`\x04a\x1B\xAAV[a\rpV[a\x01C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01.a\x02\x7F6`\x04a\x1DxV[a\x0E0V[a\x01.a\x02\x926`\x04a\x1D\xC8V[a\x0E\x84V[a\x01\x86a\x0F\xB5V[a\x01.a\x02\xAD6`\x04a\x1B\xAAV[a\x13\x81V[`eTa\x01C\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01.a\x02\xD36`\x04a\x1E\x13V[a\x13\xF7V[`\x97T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x03]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FRegistryCoordinator.onlyEjector:`D\x82\x01R\x7F caller is not the ejector\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x82\x81\x14a\x03\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FRegistryCoordinator.ejectOperato`D\x82\x01R\x7Frs: args length does not match\0\0`d\x82\x01R`\x84\x01a\x03TV[`\0[\x83\x81\x10\x15a\x04\xBAW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cn;\x17\xDB\x86\x86\x84\x81\x81\x10a\x04\x1EWa\x04\x1Ea\x1ETV[\x90P` \x02\x01` \x81\x01\x90a\x043\x91\x90a\x1B\xAAV[\x85\x85\x85\x81\x81\x10a\x04EWa\x04Ea\x1ETV[\x90P` \x02\x81\x01\x90a\x04W\x91\x90a\x1EjV[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04u\x93\x92\x91\x90a\x1E\xB0V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\x8FW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\xA3W=`\0\x80>=`\0\xFD[PPPP\x80\x80a\x04\xB2\x90a\x1F\x06V[\x91PPa\x03\xD5V[PPPPPV[a\x04\xC9a\x17XV[a\x04\xD2\x81a\x17\xB2V[PV[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R``\x91`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x13T*N\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05e\x91\x90a\x1F\x1FV[`@Qc\x87\x1E\xF0I`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x87\x1E\xF0I\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xD0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xF4\x91\x90a\x1F8V[\x90P`\x01`\x01`\xC0\x1B\x03\x81\x16\x15\x80a\x06\x8EWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x9A\xA1e=`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06eW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x89\x91\x90a\x1FaV[`\xFF\x16\x15[\x15a\x06\xAAWPP`@\x80Q`\0\x81R` \x81\x01\x90\x91R\x92\x91PPV[`\0a\x06\xBE\x82`\x01`\x01`\xC0\x1B\x03\x16a\x18\x1BV[\x90P`\0\x80[\x82Q\x81\x10\x15a\x07\x94W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c<\xA5\xA5\xF5\x84\x83\x81Q\x81\x10a\x07\x0EWa\x07\x0Ea\x1ETV[\x01` \x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\xF8\x91\x90\x91\x1C`\x04\x82\x01R`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07RW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07v\x91\x90a\x1F\x1FV[a\x07\x80\x90\x83a\x1F\x84V[\x91P\x80a\x07\x8C\x81a\x1F\x06V[\x91PPa\x06\xC4V[P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07\xAFWa\x07\xAFa\x1C\x1BV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07\xD8W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80[\x84Q\x81\x10\x15a\t\x97W`\0\x85\x82\x81Q\x81\x10a\x07\xFCWa\x07\xFCa\x1ETV[\x01` \x01Q`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xF8\x91\x90\x91\x1C`\x04\x82\x01\x81\x90R\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c<\xA5\xA5\xF5\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08qW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x95\x91\x90a\x1F\x1FV[\x90P`\0[\x81\x81\x10\x15a\t\x81W`@QcV\xE4\x02m`\xE1\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R`$\x81\x01\x82\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xAD\xC8\x04\xDA\x90`D\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\x0FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t3\x91\x90a\x1F\xB1V[`\0\x01Q\x86\x86\x81Q\x81\x10a\tIWa\tIa\x1ETV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x84a\tk\x81a\x1F\x06V[\x95PP\x80\x80a\ty\x90a\x1F\x06V[\x91PPa\x08\x9AV[PPP\x80\x80a\t\x8F\x90a\x1F\x06V[\x91PPa\x07\xDFV[P\x90\x97\x96PPPPPPPV[a\t\xACa\x17XV[a\x04\xD2\x81a\x18\xDDV[a\t\xBDa\x17XV[a\t\xC7`\0a\x19FV[V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\n\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03T\x90a \x0FV[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\x13T*N\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n|W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xA0\x91\x90a\x1F\x1FV[\x90P`\0[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x9A\xA1e=`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\x03W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B'\x91\x90a\x1FaV[`\xFF\x16\x81`\xFF\x16\x10\x15a\x0C\xEAW`@Qc\x1F\n<3`\xE3\x1B\x81R`\x04\x81\x01\x83\x90R`\xFF\x82\x16`$\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xF8Q\xE1\x98\x90`D\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xA5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xC9\x91\x90a \x99V[\x90P\x80`@\x01Q`\x01`\x01``\x1B\x03\x16`\0\x14\x80\x15a\x0B\xEEWP\x80Qc\xFF\xFF\xFF\xFF\x16\x15\x15[\x15a\x0C\xD7W\x80Q`\x01`\x01`@\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90a\x0C0\x90c\xFF\xFF\xFF\xFF\x16Ca!\nV[\x11a\x0C\xD7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`e`$\x82\x01R\x7FServiceManager.registerOperatorT`D\x82\x01R\x7FoAVS: minimum blocks elapsed lim`d\x82\x01R\x7Fit for recurrent registration no`\x84\x82\x01Rd\x1D\x08\x1BY]`\xDA\x1B`\xA4\x82\x01R`\xC4\x01a\x03TV[P\x80a\x0C\xE2\x81a!!V[\x91PPa\n\xA5V[P`@Qc\x99&\xEE}`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x99&\xEE}\x90a\r9\x90\x86\x90\x86\x90`\x04\x01a!\x8DV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\rSW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\rgW=`\0\x80>=`\0\xFD[PPPPPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\r\xB8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03T\x90a \x0FV[`@QcQ\xB2zm`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA3d\xF4\xDA\x90`$\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\x1CW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\xBAW=`\0\x80>=`\0\xFD[a\x0E8a\x17XV[`@Qc\xA9\x8F\xB3U`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA9\x8F\xB3U\x90a\x0E\x02\x90\x84\x90`\x04\x01a!\xD8V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x0E\xA4WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x0E\xBEWP0;\x15\x80\x15a\x0E\xBEWP`\0T`\xFF\x16`\x01\x14[a\x0F!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x03TV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x0FDW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x0FN\x84\x84a\x19\x98V[`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90U\x80\x15a\x0F\xAFW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPV[```\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x9A\xA1e=`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\x17W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10;\x91\x90a\x1FaV[`\xFF\x16\x90P\x80`\0\x03a\x10\\WPP`@\x80Q`\0\x81R` \x81\x01\x90\x91R\x90V[`\0\x80[\x82\x81\x10\x15a\x11\x11W`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c<\xA5\xA5\xF5\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xCFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xF3\x91\x90a\x1F\x1FV[a\x10\xFD\x90\x83a\x1F\x84V[\x91P\x80a\x11\t\x81a\x1F\x06V[\x91PPa\x10`V[P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x11,Wa\x11,a\x1C\x1BV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x11UW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x9A\xA1e=`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xBAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xDE\x91\x90a\x1FaV[`\xFF\x16\x81\x10\x15a\x13wW`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c<\xA5\xA5\xF5\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12RW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12v\x91\x90a\x1F\x1FV[\x90P`\0[\x81\x81\x10\x15a\x13bW`@QcV\xE4\x02m`\xE1\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R`$\x81\x01\x82\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xAD\xC8\x04\xDA\x90`D\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xF0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x14\x91\x90a\x1F\xB1V[`\0\x01Q\x85\x85\x81Q\x81\x10a\x13*Wa\x13*a\x1ETV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x83a\x13L\x81a\x1F\x06V[\x94PP\x80\x80a\x13Z\x90a\x1F\x06V[\x91PPa\x12{V[PP\x80\x80a\x13o\x90a\x1F\x06V[\x91PPa\x11\\V[P\x90\x94\x93PPPPV[a\x13\x89a\x17XV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x13\xEEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x03TV[a\x04\xD2\x81a\x19FV[a\x13\xFFa\x1A\x19V[`\0[\x81\x81\x10\x15a\x16\xD3W\x82\x82\x82\x81\x81\x10a\x14\x1CWa\x14\x1Ca\x1ETV[\x90P` \x02\x81\x01\x90a\x14.\x91\x90a!\xEBV[a\x14?\x90`@\x81\x01\x90` \x01a\x1B\xAAV[`\x01`\x01`\xA0\x1B\x03\x16c#\xB8r\xDD30\x86\x86\x86\x81\x81\x10a\x14aWa\x14aa\x1ETV[\x90P` \x02\x81\x01\x90a\x14s\x91\x90a!\xEBV[`@\x80Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x87\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16`\x04\x82\x01R\x93\x90\x92\x16`$\x84\x01R\x015`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x14\xCAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\xEE\x91\x90a\"\x1BV[P`\0\x83\x83\x83\x81\x81\x10a\x15\x03Wa\x15\x03a\x1ETV[\x90P` \x02\x81\x01\x90a\x15\x15\x91\x90a!\xEBV[a\x15&\x90`@\x81\x01\x90` \x01a\x1B\xAAV[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16`$\x83\x01R\x91\x90\x91\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\x94W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xB8\x91\x90a\x1F\x1FV[\x90P\x83\x83\x83\x81\x81\x10a\x15\xCCWa\x15\xCCa\x1ETV[\x90P` \x02\x81\x01\x90a\x15\xDE\x91\x90a!\xEBV[a\x15\xEF\x90`@\x81\x01\x90` \x01a\x1B\xAAV[`\x01`\x01`\xA0\x1B\x03\x16c\t^\xA7\xB3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x87\x87\x87\x81\x81\x10a\x161Wa\x161a\x1ETV[\x90P` \x02\x81\x01\x90a\x16C\x91\x90a!\xEBV[`@\x015a\x16Q\x91\x90a\x1F\x84V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x16\x9CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xC0\x91\x90a\"\x1BV[PP\x80a\x16\xCC\x90a\x1F\x06V[\x90Pa\x14\x02V[P`@Qc\xFC\xE3l}`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xFC\xE3l}\x90a\x17\"\x90\x85\x90\x85\x90`\x04\x01a\"\xAEV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17<W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x17PW=`\0\x80>=`\0\xFD[PPPPPPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\t\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x03TV[`\x97T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F\x8F0\xAB\t\xF4:l\x15}\x7F\xCE~\n\x13\xC0\x03\x04,\x1C\x95\xE8\xA7.z\x14j!\xC0\xCA\xA2M\xC9\x91\x01`@Q\x80\x91\x03\x90\xA1`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[```\0\x80a\x18)\x84a\x1A\xAEV[a\xFF\xFF\x16`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18DWa\x18Da\x1C\x1BV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x18nW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x80[\x82Q\x82\x10\x80\x15a\x18\x86WPa\x01\0\x81\x10[\x15a\x13wW`\x01\x81\x1B\x93P\x85\x84\x16\x15a\x18\xCDW\x80`\xF8\x1B\x83\x83\x81Q\x81\x10a\x18\xAFWa\x18\xAFa\x1ETV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x81`\x01\x01\x91P[a\x18\xD6\x81a\x1F\x06V[\x90Pa\x18uV[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F\xE1\x1C\xDD\xF1\x81jC1\x8C\xA1u\xBB\xC5,\xD0\x18T6\xE9\xCB\xEA\xD7\xC8:\xCCT\xA7>F\x17\x17\xE3\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x1A\x03W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x03TV[a\x1A\x0C\x82a\x19FV[a\x1A\x15\x81a\x18\xDDV[PPV[`eT`\x01`\x01`\xA0\x1B\x03\x163\x14a\t\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`L`$\x82\x01R\x7FServiceManagerBase.onlyRewardsIn`D\x82\x01R\x7Fitiator: caller is not the rewar`d\x82\x01Rk29\x904\xB74\xBA4\xB0\xBA7\xB9`\xA1\x1B`\x84\x82\x01R`\xA4\x01a\x03TV[`\0\x80[\x82\x15a\x1A\xD9Wa\x1A\xC3`\x01\x84a!\nV[\x90\x92\x16\x91\x80a\x1A\xD1\x81a#\xBBV[\x91PPa\x1A\xB2V[\x92\x91PPV[`\0\x80\x83`\x1F\x84\x01\x12a\x1A\xF1W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1B\x08W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x1B#W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a\x1B@W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1BWW`\0\x80\xFD[a\x1Bc\x88\x83\x89\x01a\x1A\xDFV[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a\x1B|W`\0\x80\xFD[Pa\x1B\x89\x87\x82\x88\x01a\x1A\xDFV[\x95\x98\x94\x97P\x95PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04\xD2W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x1B\xBCW`\0\x80\xFD[\x815a\x1B\xC7\x81a\x1B\x95V[\x93\x92PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x1C\x0FW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x1B\xEAV[P\x90\x96\x95PPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1CSWa\x1CSa\x1C\x1BV[`@R\x90V[`\0`\x01`\x01`@\x1B\x03\x80\x84\x11\x15a\x1CsWa\x1Csa\x1C\x1BV[`@Q`\x1F\x85\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x1C\x9BWa\x1C\x9Ba\x1C\x1BV[\x81`@R\x80\x93P\x85\x81R\x86\x86\x86\x01\x11\x15a\x1C\xB4W`\0\x80\xFD[\x85\x85` \x83\x017`\0` \x87\x83\x01\x01RPPP\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1C\xE1W`\0\x80\xFD[\x825a\x1C\xEC\x81a\x1B\x95V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1D\x08W`\0\x80\xFD[\x90\x84\x01\x90``\x82\x87\x03\x12\x15a\x1D\x1CW`\0\x80\xFD[a\x1D$a\x1C1V[\x825\x82\x81\x11\x15a\x1D3W`\0\x80\xFD[\x83\x01\x91P`\x1F\x82\x01\x87\x13a\x1DFW`\0\x80\xFD[a\x1DU\x87\x835` \x85\x01a\x1CYV[\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R\x80\x93PPPP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x1D\x8AW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\xA0W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x1D\xB1W`\0\x80\xFD[a\x1D\xC0\x84\x825` \x84\x01a\x1CYV[\x94\x93PPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1D\xDDW`\0\x80\xFD[\x835a\x1D\xE8\x81a\x1B\x95V[\x92P` \x84\x015a\x1D\xF8\x81a\x1B\x95V[\x91P`@\x84\x015a\x1E\x08\x81a\x1B\x95V[\x80\x91PP\x92P\x92P\x92V[`\0\x80` \x83\x85\x03\x12\x15a\x1E&W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E<W`\0\x80\xFD[a\x1EH\x85\x82\x86\x01a\x1A\xDFV[\x90\x96\x90\x95P\x93PPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x1E\x81W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1E\x9BW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x1B#W`\0\x80\xFD[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R`@` \x82\x01\x81\x90R\x81\x01\x82\x90R\x81\x83``\x83\x017`\0\x81\x83\x01``\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\x1F\x18Wa\x1F\x18a\x1E\xF0V[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x1F1W`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x1FJW`\0\x80\xFD[\x81Q`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x14a\x1B\xC7W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x1FsW`\0\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14a\x1B\xC7W`\0\x80\xFD[`\0\x82\x19\x82\x11\x15a\x1F\x97Wa\x1F\x97a\x1E\xF0V[P\x01\x90V[`\x01`\x01``\x1B\x03\x81\x16\x81\x14a\x04\xD2W`\0\x80\xFD[`\0`@\x82\x84\x03\x12\x15a\x1F\xC3W`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\x1F\xE5Wa\x1F\xE5a\x1C\x1BV[`@R\x82Qa\x1F\xF3\x81a\x1B\x95V[\x81R` \x83\x01Qa \x03\x81a\x1F\x9CV[` \x82\x01R\x93\x92PPPV[` \x80\x82R`R\x90\x82\x01R\x7FServiceManagerBase.onlyRegistryC`@\x82\x01R\x7Foordinator: caller is not the re``\x82\x01Rq3\xB4\xB9\xBA9<\x901\xB7\xB7\xB924\xB70\xBA7\xB9`q\x1B`\x80\x82\x01R`\xA0\x01\x90V[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x04\xD2W`\0\x80\xFD[`\0``\x82\x84\x03\x12\x15a \xABW`\0\x80\xFD[`@Q``\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a \xCDWa \xCDa\x1C\x1BV[`@R\x82Qa \xDB\x81a \x87V[\x81R` \x83\x01Qa \xEB\x81a \x87V[` \x82\x01R`@\x83\x01Qa \xFE\x81a\x1F\x9CV[`@\x82\x01R\x93\x92PPPV[`\0\x82\x82\x10\x15a!\x1CWa!\x1Ca\x1E\xF0V[P\x03\x90V[`\0`\xFF\x82\x16`\xFF\x81\x03a!7Wa!7a\x1E\xF0V[`\x01\x01\x92\x91PPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a!fW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a!JV[\x81\x81\x11\x15a!xW`\0` \x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01\x80`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R`\0\x82Q```@\x84\x01Ra!\xB7`\xA0\x84\x01\x82a!@V[\x90P` \x84\x01Q``\x84\x01R`@\x84\x01Q`\x80\x84\x01R\x80\x91PP\x93\x92PPPV[` \x81R`\0a\x1B\xC7` \x83\x01\x84a!@V[`\0\x825`\x9E\x19\x836\x03\x01\x81\x12a\"\x01W`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[\x805a\"\x16\x81a\x1B\x95V[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\"-W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x1B\xC7W`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a\"\x98W\x815a\"`\x81a\x1B\x95V[`\x01`\x01`\xA0\x1B\x03\x16\x87R\x81\x83\x015a\"x\x81a\x1F\x9CV[`\x01`\x01``\x1B\x03\x16\x87\x84\x01R`@\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01a\"MV[P\x94\x95\x94PPPPPV[\x805a\"\x16\x81a \x87V[` \x80\x82R\x81\x81\x01\x83\x90R`\0\x90`@\x80\x84\x01`\x05\x86\x90\x1B\x85\x01\x82\x01\x87\x85[\x88\x81\x10\x15a#\xADW\x87\x83\x03`?\x19\x01\x84R\x8156\x8B\x90\x03`\x9E\x19\x01\x81\x12a\"\xF3W`\0\x80\xFD[\x8A\x01`\xA0\x8156\x83\x90\x03`\x1E\x19\x01\x81\x12a#\x0CW`\0\x80\xFD[\x82\x01\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a#$W`\0\x80\xFD[\x80`\x06\x1B6\x03\x84\x13\x15a#6W`\0\x80\xFD[\x82\x87Ra#H\x83\x88\x01\x82\x8C\x85\x01a\"=V[\x92PPPa#W\x88\x83\x01a\"\x0BV[`\x01`\x01`\xA0\x1B\x03\x16\x88\x86\x01R\x81\x87\x015\x87\x86\x01R``a#y\x81\x84\x01a\"\xA3V[c\xFF\xFF\xFF\xFF\x16\x90\x86\x01R`\x80a#\x90\x83\x82\x01a\"\xA3V[c\xFF\xFF\xFF\xFF\x16\x95\x01\x94\x90\x94RP\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a\"\xCDV[P\x90\x98\x97PPPPPPPPV[`\0a\xFF\xFF\x80\x83\x16\x81\x81\x03a#\xD2Wa#\xD2a\x1E\xF0V[`\x01\x01\x93\x92PPPV\xFE\xA2dipfsX\"\x12 |3\xC7\xFA\xE0\xF9I\x8C\xEAgk\xE6\xC2X4H\xDF\xF7\x89\xE6\xDEW&\xEE?\x03\xC7\\\xCA\xC1bZdsolcC\0\x08\r\x003";
    /// The bytecode of the contract.
    pub static FINALIZERSERVICEMANAGER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\x16W`\x005`\xE0\x1C\x80c\x99&\xEE}\x11a\0\xA2W\x80c\xC0\xC5;\x8B\x11a\0qW\x80c\xC0\xC5;\x8B\x14a\x02\x84W\x80c\xE4\x81\xAF\x9D\x14a\x02\x97W\x80c\xF2\xFD\xE3\x8B\x14a\x02\x9FW\x80c\xFC)\x9D\xEE\x14a\x02\xB2W\x80c\xFC\xE3l}\x14a\x02\xC5W`\0\x80\xFD[\x80c\x99&\xEE}\x14a\x02$W\x80c\xA3d\xF4\xDA\x14a\x027W\x80c\xA5\nd\x0E\x14a\x02JW\x80c\xA9\x8F\xB3U\x14a\x02qW`\0\x80\xFD[\x80c;\xC2\x8C\x8C\x11a\0\xE9W\x80c;\xC2\x8C\x8C\x14a\x01\x93W\x80caL\xC1D\x14a\x01\xA6W\x80ck:\xA7.\x14a\x01\xE5W\x80cqP\x18\xA6\x14a\x02\x0BW\x80c\x8D\xA5\xCB[\x14a\x02\x13W`\0\x80\xFD[\x80c\x1E%\xAB\xFD\x14a\x01\x1BW\x80c(\xF6\x1B1\x14a\x010W\x80c,\xDD\x1E\x86\x14a\x01`W\x80c3\xCF\xB7\xB7\x14a\x01sW[`\0\x80\xFD[a\x01.a\x01)6`\x04a\x1B*V[a\x02\xD8V[\0[`\x97Ta\x01C\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01.a\x01n6`\x04a\x1B\xAAV[a\x04\xC1V[a\x01\x86a\x01\x816`\x04a\x1B\xAAV[a\x04\xD5V[`@Qa\x01W\x91\x90a\x1B\xCEV[a\x01.a\x01\xA16`\x04a\x1B\xAAV[a\t\xA4V[a\x01\xCD\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x01WV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01CV[a\x01.a\t\xB5V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x01CV[a\x01.a\x0226`\x04a\x1C\xCEV[a\t\xC9V[a\x01.a\x02E6`\x04a\x1B\xAAV[a\rpV[a\x01C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01.a\x02\x7F6`\x04a\x1DxV[a\x0E0V[a\x01.a\x02\x926`\x04a\x1D\xC8V[a\x0E\x84V[a\x01\x86a\x0F\xB5V[a\x01.a\x02\xAD6`\x04a\x1B\xAAV[a\x13\x81V[`eTa\x01C\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01.a\x02\xD36`\x04a\x1E\x13V[a\x13\xF7V[`\x97T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x03]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FRegistryCoordinator.onlyEjector:`D\x82\x01R\x7F caller is not the ejector\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x82\x81\x14a\x03\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FRegistryCoordinator.ejectOperato`D\x82\x01R\x7Frs: args length does not match\0\0`d\x82\x01R`\x84\x01a\x03TV[`\0[\x83\x81\x10\x15a\x04\xBAW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cn;\x17\xDB\x86\x86\x84\x81\x81\x10a\x04\x1EWa\x04\x1Ea\x1ETV[\x90P` \x02\x01` \x81\x01\x90a\x043\x91\x90a\x1B\xAAV[\x85\x85\x85\x81\x81\x10a\x04EWa\x04Ea\x1ETV[\x90P` \x02\x81\x01\x90a\x04W\x91\x90a\x1EjV[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04u\x93\x92\x91\x90a\x1E\xB0V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\x8FW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\xA3W=`\0\x80>=`\0\xFD[PPPP\x80\x80a\x04\xB2\x90a\x1F\x06V[\x91PPa\x03\xD5V[PPPPPV[a\x04\xC9a\x17XV[a\x04\xD2\x81a\x17\xB2V[PV[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R``\x91`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x13T*N\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05e\x91\x90a\x1F\x1FV[`@Qc\x87\x1E\xF0I`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x87\x1E\xF0I\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xD0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xF4\x91\x90a\x1F8V[\x90P`\x01`\x01`\xC0\x1B\x03\x81\x16\x15\x80a\x06\x8EWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x9A\xA1e=`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06eW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x89\x91\x90a\x1FaV[`\xFF\x16\x15[\x15a\x06\xAAWPP`@\x80Q`\0\x81R` \x81\x01\x90\x91R\x92\x91PPV[`\0a\x06\xBE\x82`\x01`\x01`\xC0\x1B\x03\x16a\x18\x1BV[\x90P`\0\x80[\x82Q\x81\x10\x15a\x07\x94W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c<\xA5\xA5\xF5\x84\x83\x81Q\x81\x10a\x07\x0EWa\x07\x0Ea\x1ETV[\x01` \x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\xF8\x91\x90\x91\x1C`\x04\x82\x01R`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07RW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07v\x91\x90a\x1F\x1FV[a\x07\x80\x90\x83a\x1F\x84V[\x91P\x80a\x07\x8C\x81a\x1F\x06V[\x91PPa\x06\xC4V[P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07\xAFWa\x07\xAFa\x1C\x1BV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07\xD8W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80[\x84Q\x81\x10\x15a\t\x97W`\0\x85\x82\x81Q\x81\x10a\x07\xFCWa\x07\xFCa\x1ETV[\x01` \x01Q`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xF8\x91\x90\x91\x1C`\x04\x82\x01\x81\x90R\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c<\xA5\xA5\xF5\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08qW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x95\x91\x90a\x1F\x1FV[\x90P`\0[\x81\x81\x10\x15a\t\x81W`@QcV\xE4\x02m`\xE1\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R`$\x81\x01\x82\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xAD\xC8\x04\xDA\x90`D\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\x0FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t3\x91\x90a\x1F\xB1V[`\0\x01Q\x86\x86\x81Q\x81\x10a\tIWa\tIa\x1ETV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x84a\tk\x81a\x1F\x06V[\x95PP\x80\x80a\ty\x90a\x1F\x06V[\x91PPa\x08\x9AV[PPP\x80\x80a\t\x8F\x90a\x1F\x06V[\x91PPa\x07\xDFV[P\x90\x97\x96PPPPPPPV[a\t\xACa\x17XV[a\x04\xD2\x81a\x18\xDDV[a\t\xBDa\x17XV[a\t\xC7`\0a\x19FV[V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\n\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03T\x90a \x0FV[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\x13T*N\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n|W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xA0\x91\x90a\x1F\x1FV[\x90P`\0[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x9A\xA1e=`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\x03W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B'\x91\x90a\x1FaV[`\xFF\x16\x81`\xFF\x16\x10\x15a\x0C\xEAW`@Qc\x1F\n<3`\xE3\x1B\x81R`\x04\x81\x01\x83\x90R`\xFF\x82\x16`$\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xF8Q\xE1\x98\x90`D\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xA5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xC9\x91\x90a \x99V[\x90P\x80`@\x01Q`\x01`\x01``\x1B\x03\x16`\0\x14\x80\x15a\x0B\xEEWP\x80Qc\xFF\xFF\xFF\xFF\x16\x15\x15[\x15a\x0C\xD7W\x80Q`\x01`\x01`@\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90a\x0C0\x90c\xFF\xFF\xFF\xFF\x16Ca!\nV[\x11a\x0C\xD7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`e`$\x82\x01R\x7FServiceManager.registerOperatorT`D\x82\x01R\x7FoAVS: minimum blocks elapsed lim`d\x82\x01R\x7Fit for recurrent registration no`\x84\x82\x01Rd\x1D\x08\x1BY]`\xDA\x1B`\xA4\x82\x01R`\xC4\x01a\x03TV[P\x80a\x0C\xE2\x81a!!V[\x91PPa\n\xA5V[P`@Qc\x99&\xEE}`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x99&\xEE}\x90a\r9\x90\x86\x90\x86\x90`\x04\x01a!\x8DV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\rSW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\rgW=`\0\x80>=`\0\xFD[PPPPPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\r\xB8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03T\x90a \x0FV[`@QcQ\xB2zm`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA3d\xF4\xDA\x90`$\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\x1CW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\xBAW=`\0\x80>=`\0\xFD[a\x0E8a\x17XV[`@Qc\xA9\x8F\xB3U`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA9\x8F\xB3U\x90a\x0E\x02\x90\x84\x90`\x04\x01a!\xD8V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x0E\xA4WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x0E\xBEWP0;\x15\x80\x15a\x0E\xBEWP`\0T`\xFF\x16`\x01\x14[a\x0F!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x03TV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x0FDW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x0FN\x84\x84a\x19\x98V[`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90U\x80\x15a\x0F\xAFW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPV[```\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x9A\xA1e=`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\x17W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10;\x91\x90a\x1FaV[`\xFF\x16\x90P\x80`\0\x03a\x10\\WPP`@\x80Q`\0\x81R` \x81\x01\x90\x91R\x90V[`\0\x80[\x82\x81\x10\x15a\x11\x11W`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c<\xA5\xA5\xF5\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xCFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xF3\x91\x90a\x1F\x1FV[a\x10\xFD\x90\x83a\x1F\x84V[\x91P\x80a\x11\t\x81a\x1F\x06V[\x91PPa\x10`V[P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x11,Wa\x11,a\x1C\x1BV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x11UW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x9A\xA1e=`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xBAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xDE\x91\x90a\x1FaV[`\xFF\x16\x81\x10\x15a\x13wW`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c<\xA5\xA5\xF5\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12RW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12v\x91\x90a\x1F\x1FV[\x90P`\0[\x81\x81\x10\x15a\x13bW`@QcV\xE4\x02m`\xE1\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R`$\x81\x01\x82\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xAD\xC8\x04\xDA\x90`D\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xF0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x14\x91\x90a\x1F\xB1V[`\0\x01Q\x85\x85\x81Q\x81\x10a\x13*Wa\x13*a\x1ETV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x83a\x13L\x81a\x1F\x06V[\x94PP\x80\x80a\x13Z\x90a\x1F\x06V[\x91PPa\x12{V[PP\x80\x80a\x13o\x90a\x1F\x06V[\x91PPa\x11\\V[P\x90\x94\x93PPPPV[a\x13\x89a\x17XV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x13\xEEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x03TV[a\x04\xD2\x81a\x19FV[a\x13\xFFa\x1A\x19V[`\0[\x81\x81\x10\x15a\x16\xD3W\x82\x82\x82\x81\x81\x10a\x14\x1CWa\x14\x1Ca\x1ETV[\x90P` \x02\x81\x01\x90a\x14.\x91\x90a!\xEBV[a\x14?\x90`@\x81\x01\x90` \x01a\x1B\xAAV[`\x01`\x01`\xA0\x1B\x03\x16c#\xB8r\xDD30\x86\x86\x86\x81\x81\x10a\x14aWa\x14aa\x1ETV[\x90P` \x02\x81\x01\x90a\x14s\x91\x90a!\xEBV[`@\x80Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x87\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16`\x04\x82\x01R\x93\x90\x92\x16`$\x84\x01R\x015`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x14\xCAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\xEE\x91\x90a\"\x1BV[P`\0\x83\x83\x83\x81\x81\x10a\x15\x03Wa\x15\x03a\x1ETV[\x90P` \x02\x81\x01\x90a\x15\x15\x91\x90a!\xEBV[a\x15&\x90`@\x81\x01\x90` \x01a\x1B\xAAV[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16`$\x83\x01R\x91\x90\x91\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\x94W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xB8\x91\x90a\x1F\x1FV[\x90P\x83\x83\x83\x81\x81\x10a\x15\xCCWa\x15\xCCa\x1ETV[\x90P` \x02\x81\x01\x90a\x15\xDE\x91\x90a!\xEBV[a\x15\xEF\x90`@\x81\x01\x90` \x01a\x1B\xAAV[`\x01`\x01`\xA0\x1B\x03\x16c\t^\xA7\xB3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x87\x87\x87\x81\x81\x10a\x161Wa\x161a\x1ETV[\x90P` \x02\x81\x01\x90a\x16C\x91\x90a!\xEBV[`@\x015a\x16Q\x91\x90a\x1F\x84V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x16\x9CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xC0\x91\x90a\"\x1BV[PP\x80a\x16\xCC\x90a\x1F\x06V[\x90Pa\x14\x02V[P`@Qc\xFC\xE3l}`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xFC\xE3l}\x90a\x17\"\x90\x85\x90\x85\x90`\x04\x01a\"\xAEV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17<W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x17PW=`\0\x80>=`\0\xFD[PPPPPPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\t\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x03TV[`\x97T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F\x8F0\xAB\t\xF4:l\x15}\x7F\xCE~\n\x13\xC0\x03\x04,\x1C\x95\xE8\xA7.z\x14j!\xC0\xCA\xA2M\xC9\x91\x01`@Q\x80\x91\x03\x90\xA1`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[```\0\x80a\x18)\x84a\x1A\xAEV[a\xFF\xFF\x16`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18DWa\x18Da\x1C\x1BV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x18nW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x80[\x82Q\x82\x10\x80\x15a\x18\x86WPa\x01\0\x81\x10[\x15a\x13wW`\x01\x81\x1B\x93P\x85\x84\x16\x15a\x18\xCDW\x80`\xF8\x1B\x83\x83\x81Q\x81\x10a\x18\xAFWa\x18\xAFa\x1ETV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x81`\x01\x01\x91P[a\x18\xD6\x81a\x1F\x06V[\x90Pa\x18uV[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F\xE1\x1C\xDD\xF1\x81jC1\x8C\xA1u\xBB\xC5,\xD0\x18T6\xE9\xCB\xEA\xD7\xC8:\xCCT\xA7>F\x17\x17\xE3\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\x1A\x03W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x03TV[a\x1A\x0C\x82a\x19FV[a\x1A\x15\x81a\x18\xDDV[PPV[`eT`\x01`\x01`\xA0\x1B\x03\x163\x14a\t\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`L`$\x82\x01R\x7FServiceManagerBase.onlyRewardsIn`D\x82\x01R\x7Fitiator: caller is not the rewar`d\x82\x01Rk29\x904\xB74\xBA4\xB0\xBA7\xB9`\xA1\x1B`\x84\x82\x01R`\xA4\x01a\x03TV[`\0\x80[\x82\x15a\x1A\xD9Wa\x1A\xC3`\x01\x84a!\nV[\x90\x92\x16\x91\x80a\x1A\xD1\x81a#\xBBV[\x91PPa\x1A\xB2V[\x92\x91PPV[`\0\x80\x83`\x1F\x84\x01\x12a\x1A\xF1W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1B\x08W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x1B#W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a\x1B@W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1BWW`\0\x80\xFD[a\x1Bc\x88\x83\x89\x01a\x1A\xDFV[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a\x1B|W`\0\x80\xFD[Pa\x1B\x89\x87\x82\x88\x01a\x1A\xDFV[\x95\x98\x94\x97P\x95PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04\xD2W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x1B\xBCW`\0\x80\xFD[\x815a\x1B\xC7\x81a\x1B\x95V[\x93\x92PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x1C\x0FW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x1B\xEAV[P\x90\x96\x95PPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1CSWa\x1CSa\x1C\x1BV[`@R\x90V[`\0`\x01`\x01`@\x1B\x03\x80\x84\x11\x15a\x1CsWa\x1Csa\x1C\x1BV[`@Q`\x1F\x85\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x1C\x9BWa\x1C\x9Ba\x1C\x1BV[\x81`@R\x80\x93P\x85\x81R\x86\x86\x86\x01\x11\x15a\x1C\xB4W`\0\x80\xFD[\x85\x85` \x83\x017`\0` \x87\x83\x01\x01RPPP\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1C\xE1W`\0\x80\xFD[\x825a\x1C\xEC\x81a\x1B\x95V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1D\x08W`\0\x80\xFD[\x90\x84\x01\x90``\x82\x87\x03\x12\x15a\x1D\x1CW`\0\x80\xFD[a\x1D$a\x1C1V[\x825\x82\x81\x11\x15a\x1D3W`\0\x80\xFD[\x83\x01\x91P`\x1F\x82\x01\x87\x13a\x1DFW`\0\x80\xFD[a\x1DU\x87\x835` \x85\x01a\x1CYV[\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R\x80\x93PPPP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x1D\x8AW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\xA0W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x1D\xB1W`\0\x80\xFD[a\x1D\xC0\x84\x825` \x84\x01a\x1CYV[\x94\x93PPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1D\xDDW`\0\x80\xFD[\x835a\x1D\xE8\x81a\x1B\x95V[\x92P` \x84\x015a\x1D\xF8\x81a\x1B\x95V[\x91P`@\x84\x015a\x1E\x08\x81a\x1B\x95V[\x80\x91PP\x92P\x92P\x92V[`\0\x80` \x83\x85\x03\x12\x15a\x1E&W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E<W`\0\x80\xFD[a\x1EH\x85\x82\x86\x01a\x1A\xDFV[\x90\x96\x90\x95P\x93PPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x1E\x81W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1E\x9BW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x1B#W`\0\x80\xFD[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R`@` \x82\x01\x81\x90R\x81\x01\x82\x90R\x81\x83``\x83\x017`\0\x81\x83\x01``\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a\x1F\x18Wa\x1F\x18a\x1E\xF0V[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x1F1W`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x1FJW`\0\x80\xFD[\x81Q`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x14a\x1B\xC7W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x1FsW`\0\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14a\x1B\xC7W`\0\x80\xFD[`\0\x82\x19\x82\x11\x15a\x1F\x97Wa\x1F\x97a\x1E\xF0V[P\x01\x90V[`\x01`\x01``\x1B\x03\x81\x16\x81\x14a\x04\xD2W`\0\x80\xFD[`\0`@\x82\x84\x03\x12\x15a\x1F\xC3W`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\x1F\xE5Wa\x1F\xE5a\x1C\x1BV[`@R\x82Qa\x1F\xF3\x81a\x1B\x95V[\x81R` \x83\x01Qa \x03\x81a\x1F\x9CV[` \x82\x01R\x93\x92PPPV[` \x80\x82R`R\x90\x82\x01R\x7FServiceManagerBase.onlyRegistryC`@\x82\x01R\x7Foordinator: caller is not the re``\x82\x01Rq3\xB4\xB9\xBA9<\x901\xB7\xB7\xB924\xB70\xBA7\xB9`q\x1B`\x80\x82\x01R`\xA0\x01\x90V[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x04\xD2W`\0\x80\xFD[`\0``\x82\x84\x03\x12\x15a \xABW`\0\x80\xFD[`@Q``\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a \xCDWa \xCDa\x1C\x1BV[`@R\x82Qa \xDB\x81a \x87V[\x81R` \x83\x01Qa \xEB\x81a \x87V[` \x82\x01R`@\x83\x01Qa \xFE\x81a\x1F\x9CV[`@\x82\x01R\x93\x92PPPV[`\0\x82\x82\x10\x15a!\x1CWa!\x1Ca\x1E\xF0V[P\x03\x90V[`\0`\xFF\x82\x16`\xFF\x81\x03a!7Wa!7a\x1E\xF0V[`\x01\x01\x92\x91PPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a!fW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a!JV[\x81\x81\x11\x15a!xW`\0` \x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01\x80`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R`\0\x82Q```@\x84\x01Ra!\xB7`\xA0\x84\x01\x82a!@V[\x90P` \x84\x01Q``\x84\x01R`@\x84\x01Q`\x80\x84\x01R\x80\x91PP\x93\x92PPPV[` \x81R`\0a\x1B\xC7` \x83\x01\x84a!@V[`\0\x825`\x9E\x19\x836\x03\x01\x81\x12a\"\x01W`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[\x805a\"\x16\x81a\x1B\x95V[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\"-W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x1B\xC7W`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a\"\x98W\x815a\"`\x81a\x1B\x95V[`\x01`\x01`\xA0\x1B\x03\x16\x87R\x81\x83\x015a\"x\x81a\x1F\x9CV[`\x01`\x01``\x1B\x03\x16\x87\x84\x01R`@\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01a\"MV[P\x94\x95\x94PPPPPV[\x805a\"\x16\x81a \x87V[` \x80\x82R\x81\x81\x01\x83\x90R`\0\x90`@\x80\x84\x01`\x05\x86\x90\x1B\x85\x01\x82\x01\x87\x85[\x88\x81\x10\x15a#\xADW\x87\x83\x03`?\x19\x01\x84R\x8156\x8B\x90\x03`\x9E\x19\x01\x81\x12a\"\xF3W`\0\x80\xFD[\x8A\x01`\xA0\x8156\x83\x90\x03`\x1E\x19\x01\x81\x12a#\x0CW`\0\x80\xFD[\x82\x01\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a#$W`\0\x80\xFD[\x80`\x06\x1B6\x03\x84\x13\x15a#6W`\0\x80\xFD[\x82\x87Ra#H\x83\x88\x01\x82\x8C\x85\x01a\"=V[\x92PPPa#W\x88\x83\x01a\"\x0BV[`\x01`\x01`\xA0\x1B\x03\x16\x88\x86\x01R\x81\x87\x015\x87\x86\x01R``a#y\x81\x84\x01a\"\xA3V[c\xFF\xFF\xFF\xFF\x16\x90\x86\x01R`\x80a#\x90\x83\x82\x01a\"\xA3V[c\xFF\xFF\xFF\xFF\x16\x95\x01\x94\x90\x94RP\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a\"\xCDV[P\x90\x98\x97PPPPPPPPV[`\0a\xFF\xFF\x80\x83\x16\x81\x81\x03a#\xD2Wa#\xD2a\x1E\xF0V[`\x01\x01\x93\x92PPPV\xFE\xA2dipfsX\"\x12 |3\xC7\xFA\xE0\xF9I\x8C\xEAgk\xE6\xC2X4H\xDF\xF7\x89\xE6\xDEW&\xEE?\x03\xC7\\\xCA\xC1bZdsolcC\0\x08\r\x003";
    /// The deployed bytecode of the contract.
    pub static FINALIZERSERVICEMANAGER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct FinalizerServiceManager<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for FinalizerServiceManager<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for FinalizerServiceManager<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for FinalizerServiceManager<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for FinalizerServiceManager<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(FinalizerServiceManager))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> FinalizerServiceManager<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                FINALIZERSERVICEMANAGER_ABI.clone(),
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
                FINALIZERSERVICEMANAGER_ABI.clone(),
                FINALIZERSERVICEMANAGER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `avsDirectory` (0x6b3aa72e) function
        pub fn avs_directory(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([107, 58, 167, 46], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createAVSRewardsSubmission` (0xfce36c7d) function
        pub fn create_avs_rewards_submission(
            &self,
            rewards_submissions: ::std::vec::Vec<RewardsSubmission>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([252, 227, 108, 125], rewards_submissions)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deregisterOperatorFromAVS` (0xa364f4da) function
        pub fn deregister_operator_from_avs(
            &self,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([163, 100, 244, 218], operator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ejectOperators` (0x1e25abfd) function
        pub fn eject_operators(
            &self,
            operators: ::std::vec::Vec<::ethers::core::types::Address>,
            quorum_numbers: ::std::vec::Vec<::ethers::core::types::Bytes>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([30, 37, 171, 253], (operators, quorum_numbers))
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
        ///Calls the contract's `getOperatorRestakedStrategies` (0x33cfb7b7) function
        pub fn get_operator_restaked_strategies(
            &self,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([51, 207, 183, 183], operator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRestakeableStrategies` (0xe481af9d) function
        pub fn get_restakeable_strategies(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([228, 129, 175, 157], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0xc0c53b8b) function
        pub fn initialize(
            &self,
            initial_owner: ::ethers::core::types::Address,
            rewards_initiator: ::ethers::core::types::Address,
            ejector: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [192, 197, 59, 139],
                    (initial_owner, rewards_initiator, ejector),
                )
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
        ///Calls the contract's `recurrentRegistrationBlocksLimit` (0x614cc144) function
        pub fn recurrent_registration_blocks_limit(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([97, 76, 193, 68], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerOperatorToAVS` (0x9926ee7d) function
        pub fn register_operator_to_avs(
            &self,
            operator: ::ethers::core::types::Address,
            operator_signature: SignatureWithSaltAndExpiry,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([153, 38, 238, 125], (operator, operator_signature))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rewardsInitiator` (0xfc299dee) function
        pub fn rewards_initiator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([252, 41, 157, 238], ())
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
        ///Calls the contract's `setRewardsInitiator` (0x3bc28c8c) function
        pub fn set_rewards_initiator(
            &self,
            new_rewards_initiator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([59, 194, 140, 140], new_rewards_initiator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `taskManager` (0xa50a640e) function
        pub fn task_manager(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([165, 10, 100, 14], ())
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
        ///Calls the contract's `updateAVSMetadataURI` (0xa98fb355) function
        pub fn update_avs_metadata_uri(
            &self,
            metadata_uri: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([169, 143, 179, 85], metadata_uri)
                .expect("method not found (this should never happen)")
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
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OwnershipTransferredFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `RewardsInitiatorUpdated` event
        pub fn rewards_initiator_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RewardsInitiatorUpdatedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            FinalizerServiceManagerEvents,
        > {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for FinalizerServiceManager<M>
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
    #[ethevent(
        name = "RewardsInitiatorUpdated",
        abi = "RewardsInitiatorUpdated(address,address)"
    )]
    pub struct RewardsInitiatorUpdatedFilter {
        pub prev_rewards_initiator: ::ethers::core::types::Address,
        pub new_rewards_initiator: ::ethers::core::types::Address,
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
    pub enum FinalizerServiceManagerEvents {
        EjectorUpdatedFilter(EjectorUpdatedFilter),
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        RewardsInitiatorUpdatedFilter(RewardsInitiatorUpdatedFilter),
    }
    impl ::ethers::contract::EthLogDecode for FinalizerServiceManagerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = EjectorUpdatedFilter::decode_log(log) {
                return Ok(FinalizerServiceManagerEvents::EjectorUpdatedFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(FinalizerServiceManagerEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(FinalizerServiceManagerEvents::OwnershipTransferredFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = RewardsInitiatorUpdatedFilter::decode_log(log) {
                return Ok(FinalizerServiceManagerEvents::RewardsInitiatorUpdatedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for FinalizerServiceManagerEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::EjectorUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RewardsInitiatorUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<EjectorUpdatedFilter> for FinalizerServiceManagerEvents {
        fn from(value: EjectorUpdatedFilter) -> Self {
            Self::EjectorUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for FinalizerServiceManagerEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for FinalizerServiceManagerEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<RewardsInitiatorUpdatedFilter> for FinalizerServiceManagerEvents {
        fn from(value: RewardsInitiatorUpdatedFilter) -> Self {
            Self::RewardsInitiatorUpdatedFilter(value)
        }
    }
    ///Container type for all input parameters for the `avsDirectory` function with signature `avsDirectory()` and selector `0x6b3aa72e`
    #[derive(
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
    #[ethcall(name = "avsDirectory", abi = "avsDirectory()")]
    pub struct AvsDirectoryCall;
    ///Container type for all input parameters for the `createAVSRewardsSubmission` function with signature `createAVSRewardsSubmission(((address,uint96)[],address,uint256,uint32,uint32)[])` and selector `0xfce36c7d`
    #[derive(
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
        name = "createAVSRewardsSubmission",
        abi = "createAVSRewardsSubmission(((address,uint96)[],address,uint256,uint32,uint32)[])"
    )]
    pub struct CreateAVSRewardsSubmissionCall {
        pub rewards_submissions: ::std::vec::Vec<RewardsSubmission>,
    }
    ///Container type for all input parameters for the `deregisterOperatorFromAVS` function with signature `deregisterOperatorFromAVS(address)` and selector `0xa364f4da`
    #[derive(
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
        name = "deregisterOperatorFromAVS",
        abi = "deregisterOperatorFromAVS(address)"
    )]
    pub struct DeregisterOperatorFromAVSCall {
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `ejectOperators` function with signature `ejectOperators(address[],bytes[])` and selector `0x1e25abfd`
    #[derive(
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
    #[ethcall(name = "ejectOperators", abi = "ejectOperators(address[],bytes[])")]
    pub struct EjectOperatorsCall {
        pub operators: ::std::vec::Vec<::ethers::core::types::Address>,
        pub quorum_numbers: ::std::vec::Vec<::ethers::core::types::Bytes>,
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
    ///Container type for all input parameters for the `getOperatorRestakedStrategies` function with signature `getOperatorRestakedStrategies(address)` and selector `0x33cfb7b7`
    #[derive(
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
        name = "getOperatorRestakedStrategies",
        abi = "getOperatorRestakedStrategies(address)"
    )]
    pub struct GetOperatorRestakedStrategiesCall {
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getRestakeableStrategies` function with signature `getRestakeableStrategies()` and selector `0xe481af9d`
    #[derive(
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
    #[ethcall(name = "getRestakeableStrategies", abi = "getRestakeableStrategies()")]
    pub struct GetRestakeableStrategiesCall;
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address,address)` and selector `0xc0c53b8b`
    #[derive(
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
    #[ethcall(name = "initialize", abi = "initialize(address,address,address)")]
    pub struct InitializeCall {
        pub initial_owner: ::ethers::core::types::Address,
        pub rewards_initiator: ::ethers::core::types::Address,
        pub ejector: ::ethers::core::types::Address,
    }
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
    ///Container type for all input parameters for the `recurrentRegistrationBlocksLimit` function with signature `recurrentRegistrationBlocksLimit()` and selector `0x614cc144`
    #[derive(
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
        name = "recurrentRegistrationBlocksLimit",
        abi = "recurrentRegistrationBlocksLimit()"
    )]
    pub struct RecurrentRegistrationBlocksLimitCall;
    ///Container type for all input parameters for the `registerOperatorToAVS` function with signature `registerOperatorToAVS(address,(bytes,bytes32,uint256))` and selector `0x9926ee7d`
    #[derive(
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
        name = "registerOperatorToAVS",
        abi = "registerOperatorToAVS(address,(bytes,bytes32,uint256))"
    )]
    pub struct RegisterOperatorToAVSCall {
        pub operator: ::ethers::core::types::Address,
        pub operator_signature: SignatureWithSaltAndExpiry,
    }
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
    ///Container type for all input parameters for the `rewardsInitiator` function with signature `rewardsInitiator()` and selector `0xfc299dee`
    #[derive(
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
    #[ethcall(name = "rewardsInitiator", abi = "rewardsInitiator()")]
    pub struct RewardsInitiatorCall;
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
    ///Container type for all input parameters for the `setRewardsInitiator` function with signature `setRewardsInitiator(address)` and selector `0x3bc28c8c`
    #[derive(
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
    #[ethcall(name = "setRewardsInitiator", abi = "setRewardsInitiator(address)")]
    pub struct SetRewardsInitiatorCall {
        pub new_rewards_initiator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `taskManager` function with signature `taskManager()` and selector `0xa50a640e`
    #[derive(
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
    #[ethcall(name = "taskManager", abi = "taskManager()")]
    pub struct TaskManagerCall;
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
    ///Container type for all input parameters for the `updateAVSMetadataURI` function with signature `updateAVSMetadataURI(string)` and selector `0xa98fb355`
    #[derive(
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
    #[ethcall(name = "updateAVSMetadataURI", abi = "updateAVSMetadataURI(string)")]
    pub struct UpdateAVSMetadataURICall {
        pub metadata_uri: ::std::string::String,
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
    pub enum FinalizerServiceManagerCalls {
        AvsDirectory(AvsDirectoryCall),
        CreateAVSRewardsSubmission(CreateAVSRewardsSubmissionCall),
        DeregisterOperatorFromAVS(DeregisterOperatorFromAVSCall),
        EjectOperators(EjectOperatorsCall),
        Ejector(EjectorCall),
        GetOperatorRestakedStrategies(GetOperatorRestakedStrategiesCall),
        GetRestakeableStrategies(GetRestakeableStrategiesCall),
        Initialize(InitializeCall),
        Owner(OwnerCall),
        RecurrentRegistrationBlocksLimit(RecurrentRegistrationBlocksLimitCall),
        RegisterOperatorToAVS(RegisterOperatorToAVSCall),
        RenounceOwnership(RenounceOwnershipCall),
        RewardsInitiator(RewardsInitiatorCall),
        SetEjector(SetEjectorCall),
        SetRewardsInitiator(SetRewardsInitiatorCall),
        TaskManager(TaskManagerCall),
        TransferOwnership(TransferOwnershipCall),
        UpdateAVSMetadataURI(UpdateAVSMetadataURICall),
    }
    impl ::ethers::core::abi::AbiDecode for FinalizerServiceManagerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AvsDirectoryCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AvsDirectory(decoded));
            }
            if let Ok(decoded) =
                <CreateAVSRewardsSubmissionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CreateAVSRewardsSubmission(decoded));
            }
            if let Ok(decoded) =
                <DeregisterOperatorFromAVSCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DeregisterOperatorFromAVS(decoded));
            }
            if let Ok(decoded) =
                <EjectOperatorsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EjectOperators(decoded));
            }
            if let Ok(decoded) = <EjectorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Ejector(decoded));
            }
            if let Ok(decoded) =
                <GetOperatorRestakedStrategiesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetOperatorRestakedStrategies(decoded));
            }
            if let Ok(decoded) =
                <GetRestakeableStrategiesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetRestakeableStrategies(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) =
                <RecurrentRegistrationBlocksLimitCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::RecurrentRegistrationBlocksLimit(decoded));
            }
            if let Ok(decoded) =
                <RegisterOperatorToAVSCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RegisterOperatorToAVS(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <RewardsInitiatorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RewardsInitiator(decoded));
            }
            if let Ok(decoded) = <SetEjectorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetEjector(decoded));
            }
            if let Ok(decoded) =
                <SetRewardsInitiatorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetRewardsInitiator(decoded));
            }
            if let Ok(decoded) = <TaskManagerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TaskManager(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <UpdateAVSMetadataURICall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateAVSMetadataURI(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for FinalizerServiceManagerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AvsDirectory(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CreateAVSRewardsSubmission(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeregisterOperatorFromAVS(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EjectOperators(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Ejector(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetOperatorRestakedStrategies(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRestakeableStrategies(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RecurrentRegistrationBlocksLimit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterOperatorToAVS(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RewardsInitiator(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetEjector(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetRewardsInitiator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TaskManager(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateAVSMetadataURI(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for FinalizerServiceManagerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AvsDirectory(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateAVSRewardsSubmission(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeregisterOperatorFromAVS(element) => ::core::fmt::Display::fmt(element, f),
                Self::EjectOperators(element) => ::core::fmt::Display::fmt(element, f),
                Self::Ejector(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOperatorRestakedStrategies(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetRestakeableStrategies(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RecurrentRegistrationBlocksLimit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RegisterOperatorToAVS(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::RewardsInitiator(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetEjector(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetRewardsInitiator(element) => ::core::fmt::Display::fmt(element, f),
                Self::TaskManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateAVSMetadataURI(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AvsDirectoryCall> for FinalizerServiceManagerCalls {
        fn from(value: AvsDirectoryCall) -> Self {
            Self::AvsDirectory(value)
        }
    }
    impl ::core::convert::From<CreateAVSRewardsSubmissionCall> for FinalizerServiceManagerCalls {
        fn from(value: CreateAVSRewardsSubmissionCall) -> Self {
            Self::CreateAVSRewardsSubmission(value)
        }
    }
    impl ::core::convert::From<DeregisterOperatorFromAVSCall> for FinalizerServiceManagerCalls {
        fn from(value: DeregisterOperatorFromAVSCall) -> Self {
            Self::DeregisterOperatorFromAVS(value)
        }
    }
    impl ::core::convert::From<EjectOperatorsCall> for FinalizerServiceManagerCalls {
        fn from(value: EjectOperatorsCall) -> Self {
            Self::EjectOperators(value)
        }
    }
    impl ::core::convert::From<EjectorCall> for FinalizerServiceManagerCalls {
        fn from(value: EjectorCall) -> Self {
            Self::Ejector(value)
        }
    }
    impl ::core::convert::From<GetOperatorRestakedStrategiesCall> for FinalizerServiceManagerCalls {
        fn from(value: GetOperatorRestakedStrategiesCall) -> Self {
            Self::GetOperatorRestakedStrategies(value)
        }
    }
    impl ::core::convert::From<GetRestakeableStrategiesCall> for FinalizerServiceManagerCalls {
        fn from(value: GetRestakeableStrategiesCall) -> Self {
            Self::GetRestakeableStrategies(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for FinalizerServiceManagerCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for FinalizerServiceManagerCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RecurrentRegistrationBlocksLimitCall> for FinalizerServiceManagerCalls {
        fn from(value: RecurrentRegistrationBlocksLimitCall) -> Self {
            Self::RecurrentRegistrationBlocksLimit(value)
        }
    }
    impl ::core::convert::From<RegisterOperatorToAVSCall> for FinalizerServiceManagerCalls {
        fn from(value: RegisterOperatorToAVSCall) -> Self {
            Self::RegisterOperatorToAVS(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for FinalizerServiceManagerCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<RewardsInitiatorCall> for FinalizerServiceManagerCalls {
        fn from(value: RewardsInitiatorCall) -> Self {
            Self::RewardsInitiator(value)
        }
    }
    impl ::core::convert::From<SetEjectorCall> for FinalizerServiceManagerCalls {
        fn from(value: SetEjectorCall) -> Self {
            Self::SetEjector(value)
        }
    }
    impl ::core::convert::From<SetRewardsInitiatorCall> for FinalizerServiceManagerCalls {
        fn from(value: SetRewardsInitiatorCall) -> Self {
            Self::SetRewardsInitiator(value)
        }
    }
    impl ::core::convert::From<TaskManagerCall> for FinalizerServiceManagerCalls {
        fn from(value: TaskManagerCall) -> Self {
            Self::TaskManager(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for FinalizerServiceManagerCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UpdateAVSMetadataURICall> for FinalizerServiceManagerCalls {
        fn from(value: UpdateAVSMetadataURICall) -> Self {
            Self::UpdateAVSMetadataURI(value)
        }
    }
    ///Container type for all return fields from the `avsDirectory` function with signature `avsDirectory()` and selector `0x6b3aa72e`
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
    pub struct AvsDirectoryReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `getOperatorRestakedStrategies` function with signature `getOperatorRestakedStrategies(address)` and selector `0x33cfb7b7`
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
    pub struct GetOperatorRestakedStrategiesReturn(
        pub ::std::vec::Vec<::ethers::core::types::Address>,
    );
    ///Container type for all return fields from the `getRestakeableStrategies` function with signature `getRestakeableStrategies()` and selector `0xe481af9d`
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
    pub struct GetRestakeableStrategiesReturn(pub ::std::vec::Vec<::ethers::core::types::Address>);
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
    ///Container type for all return fields from the `recurrentRegistrationBlocksLimit` function with signature `recurrentRegistrationBlocksLimit()` and selector `0x614cc144`
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
    pub struct RecurrentRegistrationBlocksLimitReturn(pub u64);
    ///Container type for all return fields from the `rewardsInitiator` function with signature `rewardsInitiator()` and selector `0xfc299dee`
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
    pub struct RewardsInitiatorReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `taskManager` function with signature `taskManager()` and selector `0xa50a640e`
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
    pub struct TaskManagerReturn(pub ::ethers::core::types::Address);
    ///`RewardsSubmission((address,uint96)[],address,uint256,uint32,uint32)`
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
    pub struct RewardsSubmission {
        pub strategies_and_multipliers: ::std::vec::Vec<StrategyAndMultiplier>,
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub start_timestamp: u32,
        pub duration: u32,
    }
    ///`StrategyAndMultiplier(address,uint96)`
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
    pub struct StrategyAndMultiplier {
        pub strategy: ::ethers::core::types::Address,
        pub multiplier: u128,
    }
}
