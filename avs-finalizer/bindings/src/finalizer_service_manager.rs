pub use finalizer_service_manager::*;
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
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static FINALIZERSERVICEMANAGER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01 `@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@Qb\0\x1F\xCC8\x03\x80b\0\x1F\xCC\x839\x81\x01`@\x81\x90Rb\0\x005\x91b\0\x01^V[`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\xC0R\x80\x85\x16`\x80R\x83\x16`\xA0R\x84\x84\x84b\0\0[b\0\0\x83V[PPP`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\xE0R`\x01`\x01`@\x1B\x03\x16a\x01\0RPb\0\x01\xE9\x91PPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15b\0\0\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\xFF\x90\x81\x16\x10\x15b\0\x01CW`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01[W`\0\x80\xFD[PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15b\0\x01wW`\0\x80\xFD[\x85Qb\0\x01\x84\x81b\0\x01EV[` \x87\x01Q\x90\x95Pb\0\x01\x97\x81b\0\x01EV[`@\x87\x01Q\x90\x94Pb\0\x01\xAA\x81b\0\x01EV[``\x87\x01Q\x90\x93Pb\0\x01\xBD\x81b\0\x01EV[`\x80\x87\x01Q\x90\x92P`\x01`\x01`@\x1B\x03\x81\x16\x81\x14b\0\x01\xDBW`\0\x80\xFD[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x1D\x19b\0\x02\xB3`\09`\0\x81\x81a\x01\x8A\x01Ra\x0C\xC8\x01R`\0a\x02.\x01R`\0\x81\x81a\x01\xC6\x01R\x81\x81a\r\xCB\x01R\x81\x81a\x0E\xA0\x01Ra\x0F\x18\x01R`\0\x81\x81a\x06u\x01R\x81\x81a\x07\xD0\x01R\x81\x81a\x08g\x01R\x81\x81a\x0C\x1A\x01R\x81\x81a\x10\x15\x01R\x81\x81a\x11\x98\x01Ra\x127\x01R`\0\x81\x81a\x03\x85\x01R\x81\x81a\x04\xA0\x01R\x81\x81a\x05/\x01R\x81\x81a\x05\xAF\x01R\x81\x81a\n\x98\x01R\x81\x81a\n\xF7\x01R\x81\x81a\x0Bk\x01R\x81\x81a\x0ED\x01R\x81\x81a\x0FS\x01Ra\x10\xF3\x01Ra\x1D\x19`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xF5W`\x005`\xE0\x1C\x80cqP\x18\xA6\x11a\0\x97W\x80c\xA5\nd\x0E\x11a\0fW\x80c\xA5\nd\x0E\x14a\x02)W\x80c\xA9\x8F\xB3U\x14a\x02PW\x80c\xE4\x81\xAF\x9D\x14a\x02cW\x80c\xF2\xFD\xE3\x8B\x14a\x02kW`\0\x80\xFD[\x80cqP\x18\xA6\x14a\x01\xEAW\x80c\x8D\xA5\xCB[\x14a\x01\xF2W\x80c\x99&\xEE}\x14a\x02\x03W\x80c\xA3d\xF4\xDA\x14a\x02\x16W`\0\x80\xFD[\x80c3\xCF\xB7\xB7\x11a\0\xD3W\x80c3\xCF\xB7\xB7\x14a\x01RW\x80cH\\\xC9U\x14a\x01rW\x80caL\xC1D\x14a\x01\x85W\x80ck:\xA7.\x14a\x01\xC4W`\0\x80\xFD[\x80c\x1E%\xAB\xFD\x14a\0\xFAW\x80c(\xF6\x1B1\x14a\x01\x0FW\x80c,\xDD\x1E\x86\x14a\x01?W[`\0\x80\xFD[a\x01\ra\x01\x086`\x04a\x16JV[a\x02~V[\0[`\x97Ta\x01\"\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\ra\x01M6`\x04a\x16\xCAV[a\x04gV[a\x01ea\x01`6`\x04a\x16\xCAV[a\x04{V[`@Qa\x016\x91\x90a\x16\xEEV[a\x01\ra\x01\x806`\x04a\x17;V[a\tJV[a\x01\xAC\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x016V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01\"V[a\x01\ra\nyV[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x01\"V[a\x01\ra\x02\x116`\x04a\x18'V[a\n\x8DV[a\x01\ra\x02$6`\x04a\x16\xCAV[a\x0E9V[a\x01\"\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\ra\x02^6`\x04a\x18\xD1V[a\x0E\xF9V[a\x01ea\x0FMV[a\x01\ra\x02y6`\x04a\x16\xCAV[a\x13\x16V[`\x97T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x03\x03W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FRegistryCoordinator.onlyEjector:`D\x82\x01R\x7F caller is not the ejector\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x82\x81\x14a\x03xW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FRegistryCoordinator.ejectOperato`D\x82\x01R\x7Frs: args length does not match\0\0`d\x82\x01R`\x84\x01a\x02\xFAV[`\0[\x83\x81\x10\x15a\x04`W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cn;\x17\xDB\x86\x86\x84\x81\x81\x10a\x03\xC4Wa\x03\xC4a\x19!V[\x90P` \x02\x01` \x81\x01\x90a\x03\xD9\x91\x90a\x16\xCAV[\x85\x85\x85\x81\x81\x10a\x03\xEBWa\x03\xEBa\x19!V[\x90P` \x02\x81\x01\x90a\x03\xFD\x91\x90a\x197V[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\x1B\x93\x92\x91\x90a\x19}V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x045W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04IW=`\0\x80>=`\0\xFD[PPPP\x80\x80a\x04X\x90a\x19\xD3V[\x91PPa\x03{V[PPPPPV[a\x04oa\x13\x8CV[a\x04x\x81a\x13\xE6V[PV[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R``\x91`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x13T*N\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xE7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x0B\x91\x90a\x19\xEEV[`@Qc\x87\x1E\xF0I`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x87\x1E\xF0I\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05vW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x9A\x91\x90a\x1A\x07V[\x90P`\x01`\x01`\xC0\x1B\x03\x81\x16\x15\x80a\x064WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x9A\xA1e=`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x0BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06/\x91\x90a\x1A0V[`\xFF\x16\x15[\x15a\x06PWPP`@\x80Q`\0\x81R` \x81\x01\x90\x91R\x92\x91PPV[`\0a\x06d\x82`\x01`\x01`\xC0\x1B\x03\x16a\x14OV[\x90P`\0\x80[\x82Q\x81\x10\x15a\x07:W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c<\xA5\xA5\xF5\x84\x83\x81Q\x81\x10a\x06\xB4Wa\x06\xB4a\x19!V[\x01` \x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\xF8\x91\x90\x91\x1C`\x04\x82\x01R`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xF8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x1C\x91\x90a\x19\xEEV[a\x07&\x90\x83a\x1ASV[\x91P\x80a\x072\x81a\x19\xD3V[\x91PPa\x06jV[P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07UWa\x07Ua\x17tV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07~W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80[\x84Q\x81\x10\x15a\t=W`\0\x85\x82\x81Q\x81\x10a\x07\xA2Wa\x07\xA2a\x19!V[\x01` \x01Q`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xF8\x91\x90\x91\x1C`\x04\x82\x01\x81\x90R\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c<\xA5\xA5\xF5\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x17W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08;\x91\x90a\x19\xEEV[\x90P`\0[\x81\x81\x10\x15a\t'W`@QcV\xE4\x02m`\xE1\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R`$\x81\x01\x82\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xAD\xC8\x04\xDA\x90`D\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xB5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xD9\x91\x90a\x1A\x8CV[`\0\x01Q\x86\x86\x81Q\x81\x10a\x08\xEFWa\x08\xEFa\x19!V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x84a\t\x11\x81a\x19\xD3V[\x95PP\x80\x80a\t\x1F\x90a\x19\xD3V[\x91PPa\x08@V[PPP\x80\x80a\t5\x90a\x19\xD3V[\x91PPa\x07\x85V[P\x90\x97\x96PPPPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\tjWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\t\x84WP0;\x15\x80\x15a\t\x84WP`\0T`\xFF\x16`\x01\x14[a\t\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x02\xFAV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\n\nW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\n\x13\x83a\x15\x11V[`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90U\x80\x15a\ntW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPV[a\n\x81a\x13\x8CV[a\n\x8B`\0a\x15|V[V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\n\xD5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\xFA\x90a\x1A\xE8V[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\x13T*N\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B@W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Bd\x91\x90a\x19\xEEV[\x90P`\0[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x9A\xA1e=`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xC7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xEB\x91\x90a\x1A0V[`\xFF\x16\x81`\xFF\x16\x10\x15a\r\xB3W`@Qc\x1F\n<3`\xE3\x1B\x81R`\x04\x81\x01\x83\x90R`\xFF\x82\x16`$\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xF8Q\xE1\x98\x90`D\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CiW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x8D\x91\x90a\x1BtV[\x90P\x80`@\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x14\x80\x15a\x0C\xB7WP\x80Qc\xFF\xFF\xFF\xFF\x16\x15\x15[\x15a\r\xA0W\x80Q`\x01`\x01`@\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90a\x0C\xF9\x90c\xFF\xFF\xFF\xFF\x16Ca\x1B\xDFV[\x11a\r\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`e`$\x82\x01R\x7FServiceManager.registerOperatorT`D\x82\x01R\x7FoAVS: minimum blocks elapsed lim`d\x82\x01R\x7Fit for recurrent registration no`\x84\x82\x01Rd\x1D\x08\x1BY]`\xDA\x1B`\xA4\x82\x01R`\xC4\x01a\x02\xFAV[P\x80a\r\xAB\x81a\x1B\xF6V[\x91PPa\x0BiV[P`@Qc\x99&\xEE}`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x99&\xEE}\x90a\x0E\x02\x90\x86\x90\x86\x90`\x04\x01a\x1CcV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\x1CW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E0W=`\0\x80>=`\0\xFD[PPPPPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0E\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\xFA\x90a\x1A\xE8V[`@QcQ\xB2zm`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA3d\xF4\xDA\x90`$\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\xE5W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04`W=`\0\x80>=`\0\xFD[a\x0F\x01a\x13\x8CV[`@Qc\xA9\x8F\xB3U`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA9\x8F\xB3U\x90a\x0E\xCB\x90\x84\x90`\x04\x01a\x1C\xAEV[```\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x9A\xA1e=`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xAFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xD3\x91\x90a\x1A0V[`\xFF\x16\x90P\x80a\x0F\xF1WPP`@\x80Q`\0\x81R` \x81\x01\x90\x91R\x90V[`\0\x80[\x82\x81\x10\x15a\x10\xA6W`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c<\xA5\xA5\xF5\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10dW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x88\x91\x90a\x19\xEEV[a\x10\x92\x90\x83a\x1ASV[\x91P\x80a\x10\x9E\x81a\x19\xD3V[\x91PPa\x0F\xF5V[P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x10\xC1Wa\x10\xC1a\x17tV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x10\xEAW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x9A\xA1e=`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11OW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11s\x91\x90a\x1A0V[`\xFF\x16\x81\x10\x15a\x13\x0CW`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c<\xA5\xA5\xF5\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xE7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\x0B\x91\x90a\x19\xEEV[\x90P`\0[\x81\x81\x10\x15a\x12\xF7W`@QcV\xE4\x02m`\xE1\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R`$\x81\x01\x82\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xAD\xC8\x04\xDA\x90`D\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\x85W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xA9\x91\x90a\x1A\x8CV[`\0\x01Q\x85\x85\x81Q\x81\x10a\x12\xBFWa\x12\xBFa\x19!V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x83a\x12\xE1\x81a\x19\xD3V[\x94PP\x80\x80a\x12\xEF\x90a\x19\xD3V[\x91PPa\x12\x10V[PP\x80\x80a\x13\x04\x90a\x19\xD3V[\x91PPa\x10\xF1V[P\x90\x94\x93PPPPV[a\x13\x1Ea\x13\x8CV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x13\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x02\xFAV[a\x04x\x81a\x15|V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\n\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x02\xFAV[`\x97T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F\x8F0\xAB\t\xF4:l\x15}\x7F\xCE~\n\x13\xC0\x03\x04,\x1C\x95\xE8\xA7.z\x14j!\xC0\xCA\xA2M\xC9\x91\x01`@Q\x80\x91\x03\x90\xA1`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[```\0\x80a\x14]\x84a\x15\xCEV[a\xFF\xFF\x16`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14xWa\x14xa\x17tV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x14\xA2W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x80[\x82Q\x82\x10\x80\x15a\x14\xBAWPa\x01\0\x81\x10[\x15a\x13\x0CW`\x01\x81\x1B\x93P\x85\x84\x16\x15a\x15\x01W\x80`\xF8\x1B\x83\x83\x81Q\x81\x10a\x14\xE3Wa\x14\xE3a\x19!V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x81`\x01\x01\x91P[a\x15\n\x81a\x19\xD3V[\x90Pa\x14\xA9V[`\0Ta\x01\0\x90\x04`\xFF\x16a\x13\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x02\xFAV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0\x80[\x82\x15a\x15\xF9Wa\x15\xE3`\x01\x84a\x1B\xDFV[\x90\x92\x16\x91\x80a\x15\xF1\x81a\x1C\xC1V[\x91PPa\x15\xD2V[\x92\x91PPV[`\0\x80\x83`\x1F\x84\x01\x12a\x16\x11W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16(W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x16CW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a\x16`W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x16wW`\0\x80\xFD[a\x16\x83\x88\x83\x89\x01a\x15\xFFV[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a\x16\x9CW`\0\x80\xFD[Pa\x16\xA9\x87\x82\x88\x01a\x15\xFFV[\x95\x98\x94\x97P\x95PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04xW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x16\xDCW`\0\x80\xFD[\x815a\x16\xE7\x81a\x16\xB5V[\x93\x92PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x17/W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x17\nV[P\x90\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x17NW`\0\x80\xFD[\x825a\x17Y\x81a\x16\xB5V[\x91P` \x83\x015a\x17i\x81a\x16\xB5V[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x17\xACWa\x17\xACa\x17tV[`@R\x90V[`\0`\x01`\x01`@\x1B\x03\x80\x84\x11\x15a\x17\xCCWa\x17\xCCa\x17tV[`@Q`\x1F\x85\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x17\xF4Wa\x17\xF4a\x17tV[\x81`@R\x80\x93P\x85\x81R\x86\x86\x86\x01\x11\x15a\x18\rW`\0\x80\xFD[\x85\x85` \x83\x017`\0` \x87\x83\x01\x01RPPP\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x18:W`\0\x80\xFD[\x825a\x18E\x81a\x16\xB5V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x18aW`\0\x80\xFD[\x90\x84\x01\x90``\x82\x87\x03\x12\x15a\x18uW`\0\x80\xFD[a\x18}a\x17\x8AV[\x825\x82\x81\x11\x15a\x18\x8CW`\0\x80\xFD[\x83\x01\x91P`\x1F\x82\x01\x87\x13a\x18\x9FW`\0\x80\xFD[a\x18\xAE\x87\x835` \x85\x01a\x17\xB2V[\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R\x80\x93PPPP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x18\xE3W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18\xF9W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x19\nW`\0\x80\xFD[a\x19\x19\x84\x825` \x84\x01a\x17\xB2V[\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x19NW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\x19hW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x16CW`\0\x80\xFD[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R`@` \x82\x01\x81\x90R\x81\x01\x82\x90R\x81\x83``\x83\x017`\0\x81\x83\x01``\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15a\x19\xE7Wa\x19\xE7a\x19\xBDV[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x1A\0W`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x1A\x19W`\0\x80\xFD[\x81Q`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x14a\x16\xE7W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x1ABW`\0\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14a\x16\xE7W`\0\x80\xFD[`\0\x82\x19\x82\x11\x15a\x1AfWa\x1Afa\x19\xBDV[P\x01\x90V[\x80Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1A\x87W`\0\x80\xFD[\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a\x1A\x9EW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\x1A\xC0Wa\x1A\xC0a\x17tV[`@R\x82Qa\x1A\xCE\x81a\x16\xB5V[\x81Ra\x1A\xDC` \x84\x01a\x1AkV[` \x82\x01R\x93\x92PPPV[` \x80\x82R`R\x90\x82\x01R\x7FServiceManagerBase.onlyRegistryC`@\x82\x01R\x7Foordinator: caller is not the re``\x82\x01Rq3\xB4\xB9\xBA9<\x901\xB7\xB7\xB924\xB70\xBA7\xB9`q\x1B`\x80\x82\x01R`\xA0\x01\x90V[\x80Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1A\x87W`\0\x80\xFD[`\0``\x82\x84\x03\x12\x15a\x1B\x86W`\0\x80\xFD[`@Q``\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\x1B\xA8Wa\x1B\xA8a\x17tV[`@Ra\x1B\xB4\x83a\x1B`V[\x81Ra\x1B\xC2` \x84\x01a\x1B`V[` \x82\x01Ra\x1B\xD3`@\x84\x01a\x1AkV[`@\x82\x01R\x93\x92PPPV[`\0\x82\x82\x10\x15a\x1B\xF1Wa\x1B\xF1a\x19\xBDV[P\x03\x90V[`\0`\xFF\x82\x16`\xFF\x81\x14\x15a\x1C\rWa\x1C\ra\x19\xBDV[`\x01\x01\x92\x91PPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x1C<W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x1C V[\x81\x81\x11\x15a\x1CNW`\0` \x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01\x80`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R`\0\x82Q```@\x84\x01Ra\x1C\x8D`\xA0\x84\x01\x82a\x1C\x16V[\x90P` \x84\x01Q``\x84\x01R`@\x84\x01Q`\x80\x84\x01R\x80\x91PP\x93\x92PPPV[` \x81R`\0a\x16\xE7` \x83\x01\x84a\x1C\x16V[`\0a\xFF\xFF\x80\x83\x16\x81\x81\x14\x15a\x1C\xD9Wa\x1C\xD9a\x19\xBDV[`\x01\x01\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \x13)\x17\xEDD\xB1\x11\xD0BA\xC3\x8F\xE1\xB2\xE8A\xDE\x8B\xEF\x03\xBDg\x1C\x15\x19\xEA\"\x87\xC3J\xC7\xA8dsolcC\0\x08\x0C\x003";
    /// The bytecode of the contract.
    pub static FINALIZERSERVICEMANAGER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xF5W`\x005`\xE0\x1C\x80cqP\x18\xA6\x11a\0\x97W\x80c\xA5\nd\x0E\x11a\0fW\x80c\xA5\nd\x0E\x14a\x02)W\x80c\xA9\x8F\xB3U\x14a\x02PW\x80c\xE4\x81\xAF\x9D\x14a\x02cW\x80c\xF2\xFD\xE3\x8B\x14a\x02kW`\0\x80\xFD[\x80cqP\x18\xA6\x14a\x01\xEAW\x80c\x8D\xA5\xCB[\x14a\x01\xF2W\x80c\x99&\xEE}\x14a\x02\x03W\x80c\xA3d\xF4\xDA\x14a\x02\x16W`\0\x80\xFD[\x80c3\xCF\xB7\xB7\x11a\0\xD3W\x80c3\xCF\xB7\xB7\x14a\x01RW\x80cH\\\xC9U\x14a\x01rW\x80caL\xC1D\x14a\x01\x85W\x80ck:\xA7.\x14a\x01\xC4W`\0\x80\xFD[\x80c\x1E%\xAB\xFD\x14a\0\xFAW\x80c(\xF6\x1B1\x14a\x01\x0FW\x80c,\xDD\x1E\x86\x14a\x01?W[`\0\x80\xFD[a\x01\ra\x01\x086`\x04a\x16JV[a\x02~V[\0[`\x97Ta\x01\"\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\ra\x01M6`\x04a\x16\xCAV[a\x04gV[a\x01ea\x01`6`\x04a\x16\xCAV[a\x04{V[`@Qa\x016\x91\x90a\x16\xEEV[a\x01\ra\x01\x806`\x04a\x17;V[a\tJV[a\x01\xAC\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x016V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x01\"V[a\x01\ra\nyV[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x01\"V[a\x01\ra\x02\x116`\x04a\x18'V[a\n\x8DV[a\x01\ra\x02$6`\x04a\x16\xCAV[a\x0E9V[a\x01\"\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\ra\x02^6`\x04a\x18\xD1V[a\x0E\xF9V[a\x01ea\x0FMV[a\x01\ra\x02y6`\x04a\x16\xCAV[a\x13\x16V[`\x97T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x03\x03W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FRegistryCoordinator.onlyEjector:`D\x82\x01R\x7F caller is not the ejector\0\0\0\0\0\0`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x82\x81\x14a\x03xW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FRegistryCoordinator.ejectOperato`D\x82\x01R\x7Frs: args length does not match\0\0`d\x82\x01R`\x84\x01a\x02\xFAV[`\0[\x83\x81\x10\x15a\x04`W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cn;\x17\xDB\x86\x86\x84\x81\x81\x10a\x03\xC4Wa\x03\xC4a\x19!V[\x90P` \x02\x01` \x81\x01\x90a\x03\xD9\x91\x90a\x16\xCAV[\x85\x85\x85\x81\x81\x10a\x03\xEBWa\x03\xEBa\x19!V[\x90P` \x02\x81\x01\x90a\x03\xFD\x91\x90a\x197V[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\x1B\x93\x92\x91\x90a\x19}V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x045W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04IW=`\0\x80>=`\0\xFD[PPPP\x80\x80a\x04X\x90a\x19\xD3V[\x91PPa\x03{V[PPPPPV[a\x04oa\x13\x8CV[a\x04x\x81a\x13\xE6V[PV[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R``\x91`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x13T*N\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xE7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x0B\x91\x90a\x19\xEEV[`@Qc\x87\x1E\xF0I`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x87\x1E\xF0I\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05vW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x9A\x91\x90a\x1A\x07V[\x90P`\x01`\x01`\xC0\x1B\x03\x81\x16\x15\x80a\x064WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x9A\xA1e=`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x0BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06/\x91\x90a\x1A0V[`\xFF\x16\x15[\x15a\x06PWPP`@\x80Q`\0\x81R` \x81\x01\x90\x91R\x92\x91PPV[`\0a\x06d\x82`\x01`\x01`\xC0\x1B\x03\x16a\x14OV[\x90P`\0\x80[\x82Q\x81\x10\x15a\x07:W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c<\xA5\xA5\xF5\x84\x83\x81Q\x81\x10a\x06\xB4Wa\x06\xB4a\x19!V[\x01` \x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\xF8\x91\x90\x91\x1C`\x04\x82\x01R`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xF8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x1C\x91\x90a\x19\xEEV[a\x07&\x90\x83a\x1ASV[\x91P\x80a\x072\x81a\x19\xD3V[\x91PPa\x06jV[P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x07UWa\x07Ua\x17tV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07~W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80[\x84Q\x81\x10\x15a\t=W`\0\x85\x82\x81Q\x81\x10a\x07\xA2Wa\x07\xA2a\x19!V[\x01` \x01Q`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xF8\x91\x90\x91\x1C`\x04\x82\x01\x81\x90R\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c<\xA5\xA5\xF5\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x17W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08;\x91\x90a\x19\xEEV[\x90P`\0[\x81\x81\x10\x15a\t'W`@QcV\xE4\x02m`\xE1\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R`$\x81\x01\x82\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xAD\xC8\x04\xDA\x90`D\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xB5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xD9\x91\x90a\x1A\x8CV[`\0\x01Q\x86\x86\x81Q\x81\x10a\x08\xEFWa\x08\xEFa\x19!V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x84a\t\x11\x81a\x19\xD3V[\x95PP\x80\x80a\t\x1F\x90a\x19\xD3V[\x91PPa\x08@V[PPP\x80\x80a\t5\x90a\x19\xD3V[\x91PPa\x07\x85V[P\x90\x97\x96PPPPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\tjWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\t\x84WP0;\x15\x80\x15a\t\x84WP`\0T`\xFF\x16`\x01\x14[a\t\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x02\xFAV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\n\nW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\n\x13\x83a\x15\x11V[`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90U\x80\x15a\ntW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPV[a\n\x81a\x13\x8CV[a\n\x8B`\0a\x15|V[V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\n\xD5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\xFA\x90a\x1A\xE8V[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\x13T*N\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B@W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Bd\x91\x90a\x19\xEEV[\x90P`\0[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x9A\xA1e=`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xC7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xEB\x91\x90a\x1A0V[`\xFF\x16\x81`\xFF\x16\x10\x15a\r\xB3W`@Qc\x1F\n<3`\xE3\x1B\x81R`\x04\x81\x01\x83\x90R`\xFF\x82\x16`$\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xF8Q\xE1\x98\x90`D\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CiW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x8D\x91\x90a\x1BtV[\x90P\x80`@\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x14\x80\x15a\x0C\xB7WP\x80Qc\xFF\xFF\xFF\xFF\x16\x15\x15[\x15a\r\xA0W\x80Q`\x01`\x01`@\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90a\x0C\xF9\x90c\xFF\xFF\xFF\xFF\x16Ca\x1B\xDFV[\x11a\r\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`e`$\x82\x01R\x7FServiceManager.registerOperatorT`D\x82\x01R\x7FoAVS: minimum blocks elapsed lim`d\x82\x01R\x7Fit for recurrent registration no`\x84\x82\x01Rd\x1D\x08\x1BY]`\xDA\x1B`\xA4\x82\x01R`\xC4\x01a\x02\xFAV[P\x80a\r\xAB\x81a\x1B\xF6V[\x91PPa\x0BiV[P`@Qc\x99&\xEE}`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x99&\xEE}\x90a\x0E\x02\x90\x86\x90\x86\x90`\x04\x01a\x1CcV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\x1CW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E0W=`\0\x80>=`\0\xFD[PPPPPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0E\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02\xFA\x90a\x1A\xE8V[`@QcQ\xB2zm`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA3d\xF4\xDA\x90`$\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\xE5W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04`W=`\0\x80>=`\0\xFD[a\x0F\x01a\x13\x8CV[`@Qc\xA9\x8F\xB3U`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA9\x8F\xB3U\x90a\x0E\xCB\x90\x84\x90`\x04\x01a\x1C\xAEV[```\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x9A\xA1e=`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xAFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xD3\x91\x90a\x1A0V[`\xFF\x16\x90P\x80a\x0F\xF1WPP`@\x80Q`\0\x81R` \x81\x01\x90\x91R\x90V[`\0\x80[\x82\x81\x10\x15a\x10\xA6W`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c<\xA5\xA5\xF5\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10dW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x88\x91\x90a\x19\xEEV[a\x10\x92\x90\x83a\x1ASV[\x91P\x80a\x10\x9E\x81a\x19\xD3V[\x91PPa\x0F\xF5V[P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x10\xC1Wa\x10\xC1a\x17tV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x10\xEAW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x9A\xA1e=`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11OW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11s\x91\x90a\x1A0V[`\xFF\x16\x81\x10\x15a\x13\x0CW`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c<\xA5\xA5\xF5\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xE7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\x0B\x91\x90a\x19\xEEV[\x90P`\0[\x81\x81\x10\x15a\x12\xF7W`@QcV\xE4\x02m`\xE1\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R`$\x81\x01\x82\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xAD\xC8\x04\xDA\x90`D\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\x85W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xA9\x91\x90a\x1A\x8CV[`\0\x01Q\x85\x85\x81Q\x81\x10a\x12\xBFWa\x12\xBFa\x19!V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x83a\x12\xE1\x81a\x19\xD3V[\x94PP\x80\x80a\x12\xEF\x90a\x19\xD3V[\x91PPa\x12\x10V[PP\x80\x80a\x13\x04\x90a\x19\xD3V[\x91PPa\x10\xF1V[P\x90\x94\x93PPPPV[a\x13\x1Ea\x13\x8CV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x13\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x02\xFAV[a\x04x\x81a\x15|V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\n\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x02\xFAV[`\x97T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F\x8F0\xAB\t\xF4:l\x15}\x7F\xCE~\n\x13\xC0\x03\x04,\x1C\x95\xE8\xA7.z\x14j!\xC0\xCA\xA2M\xC9\x91\x01`@Q\x80\x91\x03\x90\xA1`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[```\0\x80a\x14]\x84a\x15\xCEV[a\xFF\xFF\x16`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14xWa\x14xa\x17tV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x14\xA2W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x80[\x82Q\x82\x10\x80\x15a\x14\xBAWPa\x01\0\x81\x10[\x15a\x13\x0CW`\x01\x81\x1B\x93P\x85\x84\x16\x15a\x15\x01W\x80`\xF8\x1B\x83\x83\x81Q\x81\x10a\x14\xE3Wa\x14\xE3a\x19!V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x81`\x01\x01\x91P[a\x15\n\x81a\x19\xD3V[\x90Pa\x14\xA9V[`\0Ta\x01\0\x90\x04`\xFF\x16a\x13\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x02\xFAV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0\x80[\x82\x15a\x15\xF9Wa\x15\xE3`\x01\x84a\x1B\xDFV[\x90\x92\x16\x91\x80a\x15\xF1\x81a\x1C\xC1V[\x91PPa\x15\xD2V[\x92\x91PPV[`\0\x80\x83`\x1F\x84\x01\x12a\x16\x11W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16(W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x16CW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a\x16`W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x16wW`\0\x80\xFD[a\x16\x83\x88\x83\x89\x01a\x15\xFFV[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a\x16\x9CW`\0\x80\xFD[Pa\x16\xA9\x87\x82\x88\x01a\x15\xFFV[\x95\x98\x94\x97P\x95PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x04xW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x16\xDCW`\0\x80\xFD[\x815a\x16\xE7\x81a\x16\xB5V[\x93\x92PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x17/W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x17\nV[P\x90\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x17NW`\0\x80\xFD[\x825a\x17Y\x81a\x16\xB5V[\x91P` \x83\x015a\x17i\x81a\x16\xB5V[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x17\xACWa\x17\xACa\x17tV[`@R\x90V[`\0`\x01`\x01`@\x1B\x03\x80\x84\x11\x15a\x17\xCCWa\x17\xCCa\x17tV[`@Q`\x1F\x85\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x17\xF4Wa\x17\xF4a\x17tV[\x81`@R\x80\x93P\x85\x81R\x86\x86\x86\x01\x11\x15a\x18\rW`\0\x80\xFD[\x85\x85` \x83\x017`\0` \x87\x83\x01\x01RPPP\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x18:W`\0\x80\xFD[\x825a\x18E\x81a\x16\xB5V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x18aW`\0\x80\xFD[\x90\x84\x01\x90``\x82\x87\x03\x12\x15a\x18uW`\0\x80\xFD[a\x18}a\x17\x8AV[\x825\x82\x81\x11\x15a\x18\x8CW`\0\x80\xFD[\x83\x01\x91P`\x1F\x82\x01\x87\x13a\x18\x9FW`\0\x80\xFD[a\x18\xAE\x87\x835` \x85\x01a\x17\xB2V[\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R\x80\x93PPPP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x18\xE3W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18\xF9W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x19\nW`\0\x80\xFD[a\x19\x19\x84\x825` \x84\x01a\x17\xB2V[\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x19NW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\x19hW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x16CW`\0\x80\xFD[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R`@` \x82\x01\x81\x90R\x81\x01\x82\x90R\x81\x83``\x83\x017`\0\x81\x83\x01``\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15a\x19\xE7Wa\x19\xE7a\x19\xBDV[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x1A\0W`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x1A\x19W`\0\x80\xFD[\x81Q`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x14a\x16\xE7W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x1ABW`\0\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14a\x16\xE7W`\0\x80\xFD[`\0\x82\x19\x82\x11\x15a\x1AfWa\x1Afa\x19\xBDV[P\x01\x90V[\x80Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1A\x87W`\0\x80\xFD[\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a\x1A\x9EW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\x1A\xC0Wa\x1A\xC0a\x17tV[`@R\x82Qa\x1A\xCE\x81a\x16\xB5V[\x81Ra\x1A\xDC` \x84\x01a\x1AkV[` \x82\x01R\x93\x92PPPV[` \x80\x82R`R\x90\x82\x01R\x7FServiceManagerBase.onlyRegistryC`@\x82\x01R\x7Foordinator: caller is not the re``\x82\x01Rq3\xB4\xB9\xBA9<\x901\xB7\xB7\xB924\xB70\xBA7\xB9`q\x1B`\x80\x82\x01R`\xA0\x01\x90V[\x80Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1A\x87W`\0\x80\xFD[`\0``\x82\x84\x03\x12\x15a\x1B\x86W`\0\x80\xFD[`@Q``\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\x1B\xA8Wa\x1B\xA8a\x17tV[`@Ra\x1B\xB4\x83a\x1B`V[\x81Ra\x1B\xC2` \x84\x01a\x1B`V[` \x82\x01Ra\x1B\xD3`@\x84\x01a\x1AkV[`@\x82\x01R\x93\x92PPPV[`\0\x82\x82\x10\x15a\x1B\xF1Wa\x1B\xF1a\x19\xBDV[P\x03\x90V[`\0`\xFF\x82\x16`\xFF\x81\x14\x15a\x1C\rWa\x1C\ra\x19\xBDV[`\x01\x01\x92\x91PPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x1C<W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x1C V[\x81\x81\x11\x15a\x1CNW`\0` \x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01\x80`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R`\0\x82Q```@\x84\x01Ra\x1C\x8D`\xA0\x84\x01\x82a\x1C\x16V[\x90P` \x84\x01Q``\x84\x01R`@\x84\x01Q`\x80\x84\x01R\x80\x91PP\x93\x92PPPV[` \x81R`\0a\x16\xE7` \x83\x01\x84a\x1C\x16V[`\0a\xFF\xFF\x80\x83\x16\x81\x81\x14\x15a\x1C\xD9Wa\x1C\xD9a\x19\xBDV[`\x01\x01\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \x13)\x17\xEDD\xB1\x11\xD0BA\xC3\x8F\xE1\xB2\xE8A\xDE\x8B\xEF\x03\xBDg\x1C\x15\x19\xEA\"\x87\xC3J\xC7\xA8dsolcC\0\x08\x0C\x003";
    /// The deployed bytecode of the contract.
    pub static FINALIZERSERVICEMANAGER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
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
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    FINALIZERSERVICEMANAGER_ABI.clone(),
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
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([107, 58, 167, 46], ())
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
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
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
        ///Calls the contract's `initialize` (0x485cc955) function
        pub fn initialize(
            &self,
            initial_owner: ::ethers::core::types::Address,
            ejector: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([72, 92, 201, 85], (initial_owner, ejector))
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
        pub fn renounce_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
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
        ///Calls the contract's `taskManager` (0xa50a640e) function
        pub fn task_manager(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
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
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            FinalizerServiceManagerEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for FinalizerServiceManager<M> {
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
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
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
    pub enum FinalizerServiceManagerEvents {
        EjectorUpdatedFilter(EjectorUpdatedFilter),
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
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
                return Ok(
                    FinalizerServiceManagerEvents::OwnershipTransferredFilter(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for FinalizerServiceManagerEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::EjectorUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => {
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
    impl ::core::convert::From<OwnershipTransferredFilter>
    for FinalizerServiceManagerEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
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
        Hash
    )]
    #[ethcall(name = "avsDirectory", abi = "avsDirectory()")]
    pub struct AvsDirectoryCall;
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
    )]
    #[ethcall(name = "getRestakeableStrategies", abi = "getRestakeableStrategies()")]
    pub struct GetRestakeableStrategiesCall;
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address)` and selector `0x485cc955`
    #[derive(
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
    #[ethcall(name = "initialize", abi = "initialize(address,address)")]
    pub struct InitializeCall {
        pub initial_owner: ::ethers::core::types::Address,
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
        Hash
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
        Hash
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
        Hash
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
        Hash
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
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
        Hash
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
        Hash
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
        Hash
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
        Hash
    )]
    pub enum FinalizerServiceManagerCalls {
        AvsDirectory(AvsDirectoryCall),
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
        SetEjector(SetEjectorCall),
        TaskManager(TaskManagerCall),
        TransferOwnership(TransferOwnershipCall),
        UpdateAVSMetadataURI(UpdateAVSMetadataURICall),
    }
    impl ::ethers::core::abi::AbiDecode for FinalizerServiceManagerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AvsDirectoryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AvsDirectory(decoded));
            }
            if let Ok(decoded) = <DeregisterOperatorFromAVSCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeregisterOperatorFromAVS(decoded));
            }
            if let Ok(decoded) = <EjectOperatorsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EjectOperators(decoded));
            }
            if let Ok(decoded) = <EjectorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Ejector(decoded));
            }
            if let Ok(decoded) = <GetOperatorRestakedStrategiesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetOperatorRestakedStrategies(decoded));
            }
            if let Ok(decoded) = <GetRestakeableStrategiesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRestakeableStrategies(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <RecurrentRegistrationBlocksLimitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RecurrentRegistrationBlocksLimit(decoded));
            }
            if let Ok(decoded) = <RegisterOperatorToAVSCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RegisterOperatorToAVS(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <SetEjectorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetEjector(decoded));
            }
            if let Ok(decoded) = <TaskManagerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TaskManager(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <UpdateAVSMetadataURICall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateAVSMetadataURI(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for FinalizerServiceManagerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AvsDirectory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeregisterOperatorFromAVS(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EjectOperators(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Ejector(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetOperatorRestakedStrategies(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRestakeableStrategies(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RecurrentRegistrationBlocksLimit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterOperatorToAVS(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetEjector(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TaskManager(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
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
                Self::DeregisterOperatorFromAVS(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EjectOperators(element) => ::core::fmt::Display::fmt(element, f),
                Self::Ejector(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOperatorRestakedStrategies(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetRestakeableStrategies(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RecurrentRegistrationBlocksLimit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RegisterOperatorToAVS(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetEjector(element) => ::core::fmt::Display::fmt(element, f),
                Self::TaskManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateAVSMetadataURI(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AvsDirectoryCall> for FinalizerServiceManagerCalls {
        fn from(value: AvsDirectoryCall) -> Self {
            Self::AvsDirectory(value)
        }
    }
    impl ::core::convert::From<DeregisterOperatorFromAVSCall>
    for FinalizerServiceManagerCalls {
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
    impl ::core::convert::From<GetOperatorRestakedStrategiesCall>
    for FinalizerServiceManagerCalls {
        fn from(value: GetOperatorRestakedStrategiesCall) -> Self {
            Self::GetOperatorRestakedStrategies(value)
        }
    }
    impl ::core::convert::From<GetRestakeableStrategiesCall>
    for FinalizerServiceManagerCalls {
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
    impl ::core::convert::From<RecurrentRegistrationBlocksLimitCall>
    for FinalizerServiceManagerCalls {
        fn from(value: RecurrentRegistrationBlocksLimitCall) -> Self {
            Self::RecurrentRegistrationBlocksLimit(value)
        }
    }
    impl ::core::convert::From<RegisterOperatorToAVSCall>
    for FinalizerServiceManagerCalls {
        fn from(value: RegisterOperatorToAVSCall) -> Self {
            Self::RegisterOperatorToAVS(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for FinalizerServiceManagerCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SetEjectorCall> for FinalizerServiceManagerCalls {
        fn from(value: SetEjectorCall) -> Self {
            Self::SetEjector(value)
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
    impl ::core::convert::From<UpdateAVSMetadataURICall>
    for FinalizerServiceManagerCalls {
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
        Hash
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
        Hash
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
        Hash
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
        Hash
    )]
    pub struct GetRestakeableStrategiesReturn(
        pub ::std::vec::Vec<::ethers::core::types::Address>,
    );
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
        Hash
    )]
    pub struct RecurrentRegistrationBlocksLimitReturn(pub u64);
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
        Hash
    )]
    pub struct TaskManagerReturn(pub ::ethers::core::types::Address);
}
