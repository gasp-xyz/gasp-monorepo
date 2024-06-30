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
    const __BYTECODE: &[u8] = b"a\x01\0`@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@Qb\0\x17>8\x03\x80b\0\x17>\x839\x81\x01`@\x81\x90Rb\0\x005\x91b\0\x01OV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\xC0R\x80\x84\x16`\x80R\x82\x16`\xA0R\x83\x83\x83b\0\0[b\0\0tV[PPP`\x01`\x01`\xA0\x1B\x03\x16`\xE0RPb\0\x01\xB7\x91PPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15b\0\0\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\xFF\x90\x81\x16\x10\x15b\0\x014W`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01LW`\0\x80\xFD[PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15b\0\x01fW`\0\x80\xFD[\x84Qb\0\x01s\x81b\0\x016V[` \x86\x01Q\x90\x94Pb\0\x01\x86\x81b\0\x016V[`@\x86\x01Q\x90\x93Pb\0\x01\x99\x81b\0\x016V[``\x86\x01Q\x90\x92Pb\0\x01\xAC\x81b\0\x016V[\x93\x96\x92\x95P\x90\x93PPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x14\xECb\0\x02R`\09`\0a\x01W\x01R`\0\x81\x81`\xD9\x01R\x81\x81a\x07\x06\x01R\x81\x81a\x07\xDA\x01Ra\x08Y\x01R`\0\x81\x81a\x03\xB4\x01R\x81\x81a\x05\x10\x01R\x81\x81a\x05\xA7\x01R\x81\x81a\ni\x01R\x81\x81a\x0B\xED\x01Ra\x0C\x8C\x01R`\0\x81\x81a\x01\xDF\x01R\x81\x81a\x02n\x01R\x81\x81a\x02\xEE\x01R\x81\x81a\x06\xA9\x01R\x81\x81a\x07~\x01R\x81\x81a\t\xA7\x01Ra\x0BH\x01Ra\x14\xEC`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xA9W`\x005`\xE0\x1C\x80c\xA3d\xF4\xDA\x11a\0qW\x80c\xA3d\xF4\xDA\x14a\x01?W\x80c\xA5\nd\x0E\x14a\x01RW\x80c\xA9\x8F\xB3U\x14a\x01yW\x80c\xC4\xD6m\xE8\x14a\x01\x8CW\x80c\xE4\x81\xAF\x9D\x14a\x01\x9FW\x80c\xF2\xFD\xE3\x8B\x14a\x01\xA7W`\0\x80\xFD[\x80c3\xCF\xB7\xB7\x14a\0\xAEW\x80ck:\xA7.\x14a\0\xD7W\x80cqP\x18\xA6\x14a\x01\x11W\x80c\x8D\xA5\xCB[\x14a\x01\x1BW\x80c\x99&\xEE}\x14a\x01,W[`\0\x80\xFD[a\0\xC1a\0\xBC6`\x04a\x10\x04V[a\x01\xBAV[`@Qa\0\xCE\x91\x90a\x10(V[`@Q\x80\x91\x03\x90\xF3[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xCEV[a\x01\x19a\x06\x8AV[\0[`3T`\x01`\x01`\xA0\x1B\x03\x16a\0\xF9V[a\x01\x19a\x01:6`\x04a\x11*V[a\x06\x9EV[a\x01\x19a\x01M6`\x04a\x10\x04V[a\x07sV[a\0\xF9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\x19a\x01\x876`\x04a\x11\xD5V[a\x08:V[a\x01\x19a\x01\x9A6`\x04a\x10\x04V[a\x08\x8EV[a\0\xC1a\t\xA1V[a\x01\x19a\x01\xB56`\x04a\x10\x04V[a\rkV[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R``\x91`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x13T*N\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02&W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02J\x91\x90a\x12&V[`@Qc\x87\x1E\xF0I`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x87\x1E\xF0I\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xB5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xD9\x91\x90a\x12?V[\x90P`\x01`\x01`\xC0\x1B\x03\x81\x16\x15\x80a\x03sWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x9A\xA1e=`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03JW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03n\x91\x90a\x12hV[`\xFF\x16\x15[\x15a\x03\x8FWPP`@\x80Q`\0\x81R` \x81\x01\x90\x91R\x92\x91PPV[`\0a\x03\xA3\x82`\x01`\x01`\xC0\x1B\x03\x16a\r\xE4V[\x90P`\0\x80[\x82Q\x81\x10\x15a\x04yW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c<\xA5\xA5\xF5\x84\x83\x81Q\x81\x10a\x03\xF3Wa\x03\xF3a\x12\x8BV[\x01` \x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\xF8\x91\x90\x91\x1C`\x04\x82\x01R`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x047W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04[\x91\x90a\x12&V[a\x04e\x90\x83a\x12\xB7V[\x91P\x80a\x04q\x81a\x12\xCFV[\x91PPa\x03\xA9V[P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\x95Wa\x04\x95a\x10uV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x04\xBEW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80[\x84Q\x81\x10\x15a\x06}W`\0\x85\x82\x81Q\x81\x10a\x04\xE2Wa\x04\xE2a\x12\x8BV[\x01` \x01Q`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xF8\x91\x90\x91\x1C`\x04\x82\x01\x81\x90R\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c<\xA5\xA5\xF5\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05WW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05{\x91\x90a\x12&V[\x90P`\0[\x81\x81\x10\x15a\x06gW`@QcV\xE4\x02m`\xE1\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R`$\x81\x01\x82\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xAD\xC8\x04\xDA\x90`D\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xF5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x19\x91\x90a\x12\xEAV[`\0\x01Q\x86\x86\x81Q\x81\x10a\x06/Wa\x06/a\x12\x8BV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x84a\x06Q\x81a\x12\xCFV[\x95PP\x80\x80a\x06_\x90a\x12\xCFV[\x91PPa\x05\x80V[PPP\x80\x80a\x06u\x90a\x12\xCFV[\x91PPa\x04\xC5V[P\x90\x97\x96PPPPPPPV[a\x06\x92a\x0E\xA7V[a\x06\x9C`\0a\x0F\x01V[V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x06\xEFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xE6\x90a\x13ZV[`@Q\x80\x91\x03\x90\xFD[`@Qc\x99&\xEE}`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x99&\xEE}\x90a\x07=\x90\x85\x90\x85\x90`\x04\x01a\x14\x1FV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07WW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07kW=`\0\x80>=`\0\xFD[PPPPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x07\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xE6\x90a\x13ZV[`@QcQ\xB2zm`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA3d\xF4\xDA\x90`$\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08\x1FW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x083W=`\0\x80>=`\0\xFD[PPPPPV[a\x08Ba\x0E\xA7V[`@Qc\xA9\x8F\xB3U`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA9\x8F\xB3U\x90a\x08\x05\x90\x84\x90`\x04\x01a\x14jV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x08\xAEWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x08\xC8WP0;\x15\x80\x15a\x08\xC8WP`\0T`\xFF\x16`\x01\x14[a\t+W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x06\xE6V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\tNW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\tW\x82a\x0FSV[\x80\x15a\t\x9DW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPV[```\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x9A\xA1e=`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\x03W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n'\x91\x90a\x12hV[`\xFF\x16\x90P\x80a\nEWPP`@\x80Q`\0\x81R` \x81\x01\x90\x91R\x90V[`\0\x80[\x82\x81\x10\x15a\n\xFAW`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c<\xA5\xA5\xF5\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xB8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xDC\x91\x90a\x12&V[a\n\xE6\x90\x83a\x12\xB7V[\x91P\x80a\n\xF2\x81a\x12\xCFV[\x91PPa\nIV[P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\x16Wa\x0B\x16a\x10uV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0B?W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x9A\xA1e=`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xA4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xC8\x91\x90a\x12hV[`\xFF\x16\x81\x10\x15a\raW`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c<\xA5\xA5\xF5\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C<W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C`\x91\x90a\x12&V[\x90P`\0[\x81\x81\x10\x15a\rLW`@QcV\xE4\x02m`\xE1\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R`$\x81\x01\x82\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xAD\xC8\x04\xDA\x90`D\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xDAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xFE\x91\x90a\x12\xEAV[`\0\x01Q\x85\x85\x81Q\x81\x10a\r\x14Wa\r\x14a\x12\x8BV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x83a\r6\x81a\x12\xCFV[\x94PP\x80\x80a\rD\x90a\x12\xCFV[\x91PPa\x0CeV[PP\x80\x80a\rY\x90a\x12\xCFV[\x91PPa\x0BFV[P\x90\x94\x93PPPPV[a\rsa\x0E\xA7V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\r\xD8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x06\xE6V[a\r\xE1\x81a\x0F\x01V[PV[```\0\x80a\r\xF2\x84a\x0F\xBEV[a\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\x0EWa\x0E\x0Ea\x10uV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0E8W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x80[\x82Q\x82\x10\x80\x15a\x0EPWPa\x01\0\x81\x10[\x15a\raW`\x01\x81\x1B\x93P\x85\x84\x16\x15a\x0E\x97W\x80`\xF8\x1B\x83\x83\x81Q\x81\x10a\x0EyWa\x0Eya\x12\x8BV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x81`\x01\x01\x91P[a\x0E\xA0\x81a\x12\xCFV[\x90Pa\x0E?V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x06\x9CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x06\xE6V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\r\xD8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x06\xE6V[`\0\x80[\x82\x15a\x0F\xE9Wa\x0F\xD3`\x01\x84a\x14}V[\x90\x92\x16\x91\x80a\x0F\xE1\x81a\x14\x94V[\x91PPa\x0F\xC2V[\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\r\xE1W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x10\x16W`\0\x80\xFD[\x815a\x10!\x81a\x0F\xEFV[\x93\x92PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x10iW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x10DV[P\x90\x96\x95PPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x10\xAEWa\x10\xAEa\x10uV[`@R\x90V[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x11\x15a\x10\xCFWa\x10\xCFa\x10uV[`@Q`\x1F\x85\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x10\xF7Wa\x10\xF7a\x10uV[\x81`@R\x80\x93P\x85\x81R\x86\x86\x86\x01\x11\x15a\x11\x10W`\0\x80\xFD[\x85\x85` \x83\x017`\0` \x87\x83\x01\x01RPPP\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x11=W`\0\x80\xFD[\x825a\x11H\x81a\x0F\xEFV[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x11eW`\0\x80\xFD[\x90\x84\x01\x90``\x82\x87\x03\x12\x15a\x11yW`\0\x80\xFD[a\x11\x81a\x10\x8BV[\x825\x82\x81\x11\x15a\x11\x90W`\0\x80\xFD[\x83\x01\x91P`\x1F\x82\x01\x87\x13a\x11\xA3W`\0\x80\xFD[a\x11\xB2\x87\x835` \x85\x01a\x10\xB4V[\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R\x80\x93PPPP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x11\xE7W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\xFEW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x12\x0FW`\0\x80\xFD[a\x12\x1E\x84\x825` \x84\x01a\x10\xB4V[\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x128W`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x12QW`\0\x80\xFD[\x81Q`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x14a\x10!W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x12zW`\0\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14a\x10!W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a\x12\xCAWa\x12\xCAa\x12\xA1V[P\x01\x90V[`\0`\0\x19\x82\x14\x15a\x12\xE3Wa\x12\xE3a\x12\xA1V[P`\x01\x01\x90V[`\0`@\x82\x84\x03\x12\x15a\x12\xFCW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x13\x1FWa\x13\x1Fa\x10uV[`@R\x82Qa\x13-\x81a\x0F\xEFV[\x81R` \x83\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x13NW`\0\x80\xFD[` \x82\x01R\x93\x92PPPV[` \x80\x82R`R\x90\x82\x01R\x7FServiceManagerBase.onlyRegistryC`@\x82\x01R\x7Foordinator: caller is not the re``\x82\x01Rq3\xB4\xB9\xBA9<\x901\xB7\xB7\xB924\xB70\xBA7\xB9`q\x1B`\x80\x82\x01R`\xA0\x01\x90V[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x13\xF8W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x13\xDCV[\x81\x81\x11\x15a\x14\nW`\0` \x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01\x80`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R`\0\x82Q```@\x84\x01Ra\x14I`\xA0\x84\x01\x82a\x13\xD2V[\x90P` \x84\x01Q``\x84\x01R`@\x84\x01Q`\x80\x84\x01R\x80\x91PP\x93\x92PPPV[` \x81R`\0a\x10!` \x83\x01\x84a\x13\xD2V[`\0\x82\x82\x10\x15a\x14\x8FWa\x14\x8Fa\x12\xA1V[P\x03\x90V[`\0a\xFF\xFF\x80\x83\x16\x81\x81\x14\x15a\x14\xACWa\x14\xACa\x12\xA1V[`\x01\x01\x93\x92PPPV\xFE\xA2dipfsX\"\x12 5\xA1;*\xBD^\xDE.\xBFTC\xF2\x11\xD2\xE8\xA9G\x14QO\xED>5FX2m\xC1\x1B\x05\x97\xD0dsolcC\0\x08\x0C\x003";
    /// The bytecode of the contract.
    pub static FINALIZERSERVICEMANAGER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xA9W`\x005`\xE0\x1C\x80c\xA3d\xF4\xDA\x11a\0qW\x80c\xA3d\xF4\xDA\x14a\x01?W\x80c\xA5\nd\x0E\x14a\x01RW\x80c\xA9\x8F\xB3U\x14a\x01yW\x80c\xC4\xD6m\xE8\x14a\x01\x8CW\x80c\xE4\x81\xAF\x9D\x14a\x01\x9FW\x80c\xF2\xFD\xE3\x8B\x14a\x01\xA7W`\0\x80\xFD[\x80c3\xCF\xB7\xB7\x14a\0\xAEW\x80ck:\xA7.\x14a\0\xD7W\x80cqP\x18\xA6\x14a\x01\x11W\x80c\x8D\xA5\xCB[\x14a\x01\x1BW\x80c\x99&\xEE}\x14a\x01,W[`\0\x80\xFD[a\0\xC1a\0\xBC6`\x04a\x10\x04V[a\x01\xBAV[`@Qa\0\xCE\x91\x90a\x10(V[`@Q\x80\x91\x03\x90\xF3[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xCEV[a\x01\x19a\x06\x8AV[\0[`3T`\x01`\x01`\xA0\x1B\x03\x16a\0\xF9V[a\x01\x19a\x01:6`\x04a\x11*V[a\x06\x9EV[a\x01\x19a\x01M6`\x04a\x10\x04V[a\x07sV[a\0\xF9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\x19a\x01\x876`\x04a\x11\xD5V[a\x08:V[a\x01\x19a\x01\x9A6`\x04a\x10\x04V[a\x08\x8EV[a\0\xC1a\t\xA1V[a\x01\x19a\x01\xB56`\x04a\x10\x04V[a\rkV[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R``\x91`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x13T*N\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02&W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02J\x91\x90a\x12&V[`@Qc\x87\x1E\xF0I`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x87\x1E\xF0I\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xB5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xD9\x91\x90a\x12?V[\x90P`\x01`\x01`\xC0\x1B\x03\x81\x16\x15\x80a\x03sWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x9A\xA1e=`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03JW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03n\x91\x90a\x12hV[`\xFF\x16\x15[\x15a\x03\x8FWPP`@\x80Q`\0\x81R` \x81\x01\x90\x91R\x92\x91PPV[`\0a\x03\xA3\x82`\x01`\x01`\xC0\x1B\x03\x16a\r\xE4V[\x90P`\0\x80[\x82Q\x81\x10\x15a\x04yW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c<\xA5\xA5\xF5\x84\x83\x81Q\x81\x10a\x03\xF3Wa\x03\xF3a\x12\x8BV[\x01` \x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\xF8\x91\x90\x91\x1C`\x04\x82\x01R`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x047W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04[\x91\x90a\x12&V[a\x04e\x90\x83a\x12\xB7V[\x91P\x80a\x04q\x81a\x12\xCFV[\x91PPa\x03\xA9V[P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\x95Wa\x04\x95a\x10uV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x04\xBEW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80[\x84Q\x81\x10\x15a\x06}W`\0\x85\x82\x81Q\x81\x10a\x04\xE2Wa\x04\xE2a\x12\x8BV[\x01` \x01Q`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xF8\x91\x90\x91\x1C`\x04\x82\x01\x81\x90R\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c<\xA5\xA5\xF5\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05WW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05{\x91\x90a\x12&V[\x90P`\0[\x81\x81\x10\x15a\x06gW`@QcV\xE4\x02m`\xE1\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R`$\x81\x01\x82\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xAD\xC8\x04\xDA\x90`D\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xF5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x19\x91\x90a\x12\xEAV[`\0\x01Q\x86\x86\x81Q\x81\x10a\x06/Wa\x06/a\x12\x8BV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x84a\x06Q\x81a\x12\xCFV[\x95PP\x80\x80a\x06_\x90a\x12\xCFV[\x91PPa\x05\x80V[PPP\x80\x80a\x06u\x90a\x12\xCFV[\x91PPa\x04\xC5V[P\x90\x97\x96PPPPPPPV[a\x06\x92a\x0E\xA7V[a\x06\x9C`\0a\x0F\x01V[V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x06\xEFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xE6\x90a\x13ZV[`@Q\x80\x91\x03\x90\xFD[`@Qc\x99&\xEE}`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x99&\xEE}\x90a\x07=\x90\x85\x90\x85\x90`\x04\x01a\x14\x1FV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07WW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07kW=`\0\x80>=`\0\xFD[PPPPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x07\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xE6\x90a\x13ZV[`@QcQ\xB2zm`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA3d\xF4\xDA\x90`$\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08\x1FW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x083W=`\0\x80>=`\0\xFD[PPPPPV[a\x08Ba\x0E\xA7V[`@Qc\xA9\x8F\xB3U`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA9\x8F\xB3U\x90a\x08\x05\x90\x84\x90`\x04\x01a\x14jV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x08\xAEWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x08\xC8WP0;\x15\x80\x15a\x08\xC8WP`\0T`\xFF\x16`\x01\x14[a\t+W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x06\xE6V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\tNW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\tW\x82a\x0FSV[\x80\x15a\t\x9DW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPV[```\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x9A\xA1e=`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\x03W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n'\x91\x90a\x12hV[`\xFF\x16\x90P\x80a\nEWPP`@\x80Q`\0\x81R` \x81\x01\x90\x91R\x90V[`\0\x80[\x82\x81\x10\x15a\n\xFAW`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c<\xA5\xA5\xF5\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xB8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xDC\x91\x90a\x12&V[a\n\xE6\x90\x83a\x12\xB7V[\x91P\x80a\n\xF2\x81a\x12\xCFV[\x91PPa\nIV[P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\x16Wa\x0B\x16a\x10uV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0B?W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x9A\xA1e=`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xA4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xC8\x91\x90a\x12hV[`\xFF\x16\x81\x10\x15a\raW`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c<\xA5\xA5\xF5\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C<W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C`\x91\x90a\x12&V[\x90P`\0[\x81\x81\x10\x15a\rLW`@QcV\xE4\x02m`\xE1\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R`$\x81\x01\x82\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xAD\xC8\x04\xDA\x90`D\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xDAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xFE\x91\x90a\x12\xEAV[`\0\x01Q\x85\x85\x81Q\x81\x10a\r\x14Wa\r\x14a\x12\x8BV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x83a\r6\x81a\x12\xCFV[\x94PP\x80\x80a\rD\x90a\x12\xCFV[\x91PPa\x0CeV[PP\x80\x80a\rY\x90a\x12\xCFV[\x91PPa\x0BFV[P\x90\x94\x93PPPPV[a\rsa\x0E\xA7V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\r\xD8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x06\xE6V[a\r\xE1\x81a\x0F\x01V[PV[```\0\x80a\r\xF2\x84a\x0F\xBEV[a\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\x0EWa\x0E\x0Ea\x10uV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0E8W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0\x80[\x82Q\x82\x10\x80\x15a\x0EPWPa\x01\0\x81\x10[\x15a\raW`\x01\x81\x1B\x93P\x85\x84\x16\x15a\x0E\x97W\x80`\xF8\x1B\x83\x83\x81Q\x81\x10a\x0EyWa\x0Eya\x12\x8BV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81`\0\x1A\x90SP\x81`\x01\x01\x91P[a\x0E\xA0\x81a\x12\xCFV[\x90Pa\x0E?V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x06\x9CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x06\xE6V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0Ta\x01\0\x90\x04`\xFF\x16a\r\xD8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x01a\x06\xE6V[`\0\x80[\x82\x15a\x0F\xE9Wa\x0F\xD3`\x01\x84a\x14}V[\x90\x92\x16\x91\x80a\x0F\xE1\x81a\x14\x94V[\x91PPa\x0F\xC2V[\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\r\xE1W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x10\x16W`\0\x80\xFD[\x815a\x10!\x81a\x0F\xEFV[\x93\x92PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x10iW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x10DV[P\x90\x96\x95PPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x10\xAEWa\x10\xAEa\x10uV[`@R\x90V[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x11\x15a\x10\xCFWa\x10\xCFa\x10uV[`@Q`\x1F\x85\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x10\xF7Wa\x10\xF7a\x10uV[\x81`@R\x80\x93P\x85\x81R\x86\x86\x86\x01\x11\x15a\x11\x10W`\0\x80\xFD[\x85\x85` \x83\x017`\0` \x87\x83\x01\x01RPPP\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x11=W`\0\x80\xFD[\x825a\x11H\x81a\x0F\xEFV[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x11eW`\0\x80\xFD[\x90\x84\x01\x90``\x82\x87\x03\x12\x15a\x11yW`\0\x80\xFD[a\x11\x81a\x10\x8BV[\x825\x82\x81\x11\x15a\x11\x90W`\0\x80\xFD[\x83\x01\x91P`\x1F\x82\x01\x87\x13a\x11\xA3W`\0\x80\xFD[a\x11\xB2\x87\x835` \x85\x01a\x10\xB4V[\x81R` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R\x80\x93PPPP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x11\xE7W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\xFEW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x12\x0FW`\0\x80\xFD[a\x12\x1E\x84\x825` \x84\x01a\x10\xB4V[\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x128W`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x12QW`\0\x80\xFD[\x81Q`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x14a\x10!W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x12zW`\0\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14a\x10!W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a\x12\xCAWa\x12\xCAa\x12\xA1V[P\x01\x90V[`\0`\0\x19\x82\x14\x15a\x12\xE3Wa\x12\xE3a\x12\xA1V[P`\x01\x01\x90V[`\0`@\x82\x84\x03\x12\x15a\x12\xFCW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x13\x1FWa\x13\x1Fa\x10uV[`@R\x82Qa\x13-\x81a\x0F\xEFV[\x81R` \x83\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x13NW`\0\x80\xFD[` \x82\x01R\x93\x92PPPV[` \x80\x82R`R\x90\x82\x01R\x7FServiceManagerBase.onlyRegistryC`@\x82\x01R\x7Foordinator: caller is not the re``\x82\x01Rq3\xB4\xB9\xBA9<\x901\xB7\xB7\xB924\xB70\xBA7\xB9`q\x1B`\x80\x82\x01R`\xA0\x01\x90V[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x13\xF8W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x13\xDCV[\x81\x81\x11\x15a\x14\nW`\0` \x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01\x80`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R`\0\x82Q```@\x84\x01Ra\x14I`\xA0\x84\x01\x82a\x13\xD2V[\x90P` \x84\x01Q``\x84\x01R`@\x84\x01Q`\x80\x84\x01R\x80\x91PP\x93\x92PPPV[` \x81R`\0a\x10!` \x83\x01\x84a\x13\xD2V[`\0\x82\x82\x10\x15a\x14\x8FWa\x14\x8Fa\x12\xA1V[P\x03\x90V[`\0a\xFF\xFF\x80\x83\x16\x81\x81\x14\x15a\x14\xACWa\x14\xACa\x12\xA1V[`\x01\x01\x93\x92PPPV\xFE\xA2dipfsX\"\x12 5\xA1;*\xBD^\xDE.\xBFTC\xF2\x11\xD2\xE8\xA9G\x14QO\xED>5FX2m\xC1\x1B\x05\x97\xD0dsolcC\0\x08\x0C\x003";
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
        ///Calls the contract's `initialize` (0xc4d66de8) function
        pub fn initialize(
            &self,
            initial_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 214, 109, 232], initial_owner)
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
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ::ethers::contract::EthLogDecode for FinalizerServiceManagerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
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
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
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
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address)` and selector `0xc4d66de8`
    #[derive(
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
    #[ethcall(name = "initialize", abi = "initialize(address)")]
    pub struct InitializeCall {
        pub initial_owner: ::ethers::core::types::Address,
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
        GetOperatorRestakedStrategies(GetOperatorRestakedStrategiesCall),
        GetRestakeableStrategies(GetRestakeableStrategiesCall),
        Initialize(InitializeCall),
        Owner(OwnerCall),
        RegisterOperatorToAVS(RegisterOperatorToAVSCall),
        RenounceOwnership(RenounceOwnershipCall),
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
                Self::RegisterOperatorToAVS(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => {
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
                Self::GetOperatorRestakedStrategies(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetRestakeableStrategies(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterOperatorToAVS(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
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
