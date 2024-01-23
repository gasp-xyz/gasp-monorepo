pub use bls_pubkey_registry::*;
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
pub mod bls_pubkey_registry {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
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
                        name: ::std::borrow::ToOwned::to_owned("_pubkeyCompendium"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "contract IBLSPublicKeyCompendium",
                            ),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("deregisterOperator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
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
                    ::std::borrow::ToOwned::to_owned("getApkForQuorum"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getApkForQuorum"),
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
                    ::std::borrow::ToOwned::to_owned(
                        "getApkHashForQuorumAtBlockNumberFromIndex",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getApkHashForQuorumAtBlockNumberFromIndex",
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        24usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes24"),
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
                        "getApkIndicesForQuorumsAtBlockNumber",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getApkIndicesForQuorumsAtBlockNumber",
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
                                    name: ::std::borrow::ToOwned::to_owned("blockNumber"),
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
                    ::std::borrow::ToOwned::to_owned("getApkUpdateForQuorumByIndex"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getApkUpdateForQuorumByIndex",
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
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(24usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IBLSPubkeyRegistry.ApkUpdate",
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
                    ::std::borrow::ToOwned::to_owned("getOperatorFromPubkeyHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getOperatorFromPubkeyHash",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pubkeyHash"),
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
                    ::std::borrow::ToOwned::to_owned("getQuorumApkHistoryLength"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getQuorumApkHistoryLength",
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pubkeyCompendium"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pubkeyCompendium"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IBLSPublicKeyCompendium",
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
                    ::std::borrow::ToOwned::to_owned("quorumApk"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("quorumApk"),
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
                                    name: ::std::borrow::ToOwned::to_owned("X"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("Y"),
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
                    ::std::borrow::ToOwned::to_owned("quorumApkUpdates"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("quorumApkUpdates"),
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
                                    name: ::std::borrow::ToOwned::to_owned("apkHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        24usize,
                                    ),
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
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "nextUpdateBlockNumber",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("registryCoordinator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "registryCoordinator",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IRegistryCoordinator",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                    ::std::borrow::ToOwned::to_owned("OperatorAddedToQuorums"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OperatorAddedToQuorums",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumbers"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OperatorRemovedFromQuorums"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OperatorRemovedFromQuorums",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("quorumNumbers"),
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
    pub static BLSPUBKEYREGISTRY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\x15\x8D8\x03\x80b\0\x15\x8D\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x018V[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\x80R\x81\x16`\xA0R\x81\x81b\0\0Sb\0\0]V[PPPPb\0\x01wV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15b\0\0\xCAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\xFF\x90\x81\x16\x10\x15b\0\x01\x1DW`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x015W`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15b\0\x01LW`\0\x80\xFD[\x82Qb\0\x01Y\x81b\0\x01\x1FV[` \x84\x01Q\x90\x92Pb\0\x01l\x81b\0\x01\x1FV[\x80\x91PP\x92P\x92\x90PV[`\x80Q`\xA0Qa\x13\xDCb\0\x01\xB1`\09`\0\x81\x81`\xE4\x01Ra\x05\x99\x01R`\0\x81\x81a\x01\xF1\x01R\x81\x81a\x034\x01Ra\x04\xAC\x01Ra\x13\xDC`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xB4W`\x005`\xE0\x1C\x80cm\x14\xA9\x87\x11a\0qW\x80cm\x14\xA9\x87\x14a\x01\xECW\x80cr%\x05~\x14a\x02\x13W\x80c\x7F^\xCC\xBB\x14a\x02aW\x80c\xC1\xAFk$\x14a\x02\xA2W\x80c\xED\xA1\x07c\x14a\x02\xCFW\x80c\xFB\x81\xA7\xBE\x14a\x02\xEFW`\0\x80\xFD[\x80c\x03\xCEK\xAD\x14a\0\xB9W\x80c\x18uH\xC8\x14a\0\xDFW\x80c$6\x9B*\x14a\x01\x1EW\x80c2\xDEc\x08\x14a\x013W\x80cG\xB3\x14\xE8\x14a\x01oW\x80cc\xA9E\x10\x14a\x01\x82W[`\0\x80\xFD[a\0\xCCa\0\xC76`\x04a\x0FMV[a\x03'V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x06\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xD6V[a\x011a\x01,6`\x04a\x0FMV[a\x04\xA1V[\0[a\x01Za\x01A6`\x04a\x10\x17V[`\x02` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T\x82V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\0\xD6V[a\x01\x06a\x01}6`\x04a\x109V[a\x05\x80V[a\x01\xD1a\x01\x906`\x04a\x10\x17V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01RP`\xFF\x16`\0\x90\x81R`\x02` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x01T\x90\x82\x01R\x90V[`@\x80Q\x82Q\x81R` \x92\x83\x01Q\x92\x81\x01\x92\x90\x92R\x01a\0\xD6V[a\x01\x06\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02&a\x02!6`\x04a\x10RV[a\x06\x12V[`@\x80Q\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x80\x84\x01Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x83\x01\x91\x90\x91R\x92\x82\x01Q\x90\x92\x16\x90\x82\x01R``\x01a\0\xD6V[a\x02ta\x02o6`\x04a\x10RV[a\x06\xA4V[`@\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x94\x16\x84Rc\xFF\xFF\xFF\xFF\x92\x83\x16` \x85\x01R\x91\x16\x90\x82\x01R``\x01a\0\xD6V[a\x02\xB5a\x02\xB06`\x04a\x10|V[a\x06\xEFV[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x91\x16\x81R` \x01a\0\xD6V[a\x02\xE2a\x02\xDD6`\x04a\x10\xC4V[a\x07xV[`@Qa\0\xD6\x91\x90a\x11<V[a\x03\x12a\x02\xFD6`\x04a\x10\x17V[`\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0\xD6V[`\x003`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x03zW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03q\x90a\x11\x86V[`@Q\x80\x91\x03\x90\xFD[`\0a\x03\x85\x83a\t\xD0V[\x90P\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5\x81\x14\x15a\x04\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`?`$\x82\x01R\x7FBLSPubkeyRegistry.registerOperat`D\x82\x01R\x7For: cannot register zero pubkey\0`d\x82\x01R`\x84\x01a\x03qV[\x84`\x01`\x01`\xA0\x1B\x03\x16a\x040\x82a\x05\x80V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x04VW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03q\x90a\x11\xFDV[a\x04`\x84\x84a\n\x13V[\x7FSX\xC5\xB4!y\x17\x8C\x8F\xC7WsJ\xC2\xA3\x19\x8F\x90q\xC7e\xEE\r\x83\x89!\x15%\xF5\0RF\x85\x85`@Qa\x04\x91\x92\x91\x90a\x12[V[`@Q\x80\x91\x03\x90\xA1\x94\x93PPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x04\xE9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03q\x90a\x11\x86V[`\0a\x04\xF4\x82a\t\xD0V[\x90P\x83`\x01`\x01`\xA0\x1B\x03\x16a\x05\t\x82a\x05\x80V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x05/W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03q\x90a\x11\xFDV[a\x05A\x83a\x05<\x84a\x0B\xD2V[a\n\x13V[\x7F\x14\xA5\x17+1.\x9D,\"\xB8F\x8F\x9Cp\xEC,\xAA\x9D\xE94\xFE8\x074\xFB\xC6\xF3\xBE\xFF+\x14\xBA\x84\x84`@Qa\x05r\x92\x91\x90a\x12[V[`@Q\x80\x91\x03\x90\xA1PPPPV[`@Qct]\xCDs`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE8\xBB\x9A\xE6\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xE8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x0C\x91\x90a\x12\xC0V[\x92\x91PPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R`\xFF\x86\x16\x82R`\x01\x90R\x91\x90\x91 \x80T\x83\x90\x81\x10a\x06OWa\x06Oa\x12\xDDV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x84\x1B\x16\x82Rc\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x82\x04\x81\x16\x94\x83\x01\x94\x90\x94R`\x01`\xE0\x1B\x90\x04\x90\x92\x16\x90\x82\x01R\x90P\x92\x91PPV[`\x01` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x06\xC0W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`@\x81\x90\x1B\x92Pc\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x82\x04\x81\x16\x92P`\x01`\xE0\x1B\x90\x91\x04\x16\x83V[`\xFF\x83\x16`\0\x90\x81R`\x01` R`@\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x07\x16Wa\x07\x16a\x12\xDDV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x84\x1B\x16\x82Rc\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x82\x04\x81\x16\x94\x83\x01\x94\x90\x94R`\x01`\xE0\x1B\x90\x04\x90\x92\x16\x90\x82\x01R\x90Pa\x07o\x81\x85a\x0C\x91V[Q\x94\x93PPPPV[```\0\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\x95Wa\x07\x95a\x0E\xB7V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07\xBEW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x84\x81\x10\x15a\t\xC7W`\0\x86\x86\x83\x81\x81\x10a\x07\xE0Wa\x07\xE0a\x12\xDDV[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x01` R`@\x90 T\x90\x92P\x90Pc\xFF\xFF\xFF\xFF\x81\x16\x15\x80a\x08IWP`\xFF\x82\x16`\0\x90\x81R`\x01` R`@\x81 \x80T\x90\x91\x90a\x08-Wa\x08-a\x12\xDDV[`\0\x91\x82R` \x90\x91 \x01T`\x01`\xC0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x86\x10[\x15a\x08\xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`^`$\x82\x01R\x7FBLSPubkeyRegistry.getApkIndicesF`D\x82\x01R\x7ForQuorumsAtBlockNumber: blockNum`d\x82\x01R\x7Fber is before the first update\0\0`\x84\x82\x01R`\xA4\x01a\x03qV[`\0[\x81c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\t\xB1W`\xFF\x83\x16`\0\x90\x81R`\x01` \x81\x90R`@\x90\x91 \x88\x91a\t\x1A\x84\x86a\x13\tV[a\t$\x91\x90a\x13\tV[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\t:Wa\t:a\x12\xDDV[`\0\x91\x82R` \x90\x91 \x01T`\x01`\xC0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x11a\t\x9FW`\x01a\td\x82\x84a\x13\tV[a\tn\x91\x90a\x13\tV[\x85\x85\x81Q\x81\x10a\t\x80Wa\t\x80a\x12\xDDV[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPPa\t\xB1V[\x80a\t\xA9\x81a\x13.V[\x91PPa\x08\xE5V[PPP\x80\x80a\t\xBF\x90a\x13RV[\x91PPa\x07\xC4V[P\x94\x93PPPPV[`\0\x81`\0\x01Q\x82` \x01Q`@Q` \x01a\t\xF6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0[\x83Q\x81\x10\x15a\x0B\xCCW`\0\x84\x82\x81Q\x81\x10a\nGWa\nGa\x12\xDDV[\x01` \x90\x81\x01Q`\xF8\x1C`\0\x81\x81R`\x01\x90\x92R`@\x90\x91 T\x90\x91P\x80\x15a\n\xC7W`\xFF\x82\x16`\0\x90\x81R`\x01` \x81\x90R`@\x90\x91 C\x91a\n\x8B\x90\x84a\x13mV[\x81T\x81\x10a\n\x9BWa\n\x9Ba\x12\xDDV[\x90`\0R` `\0 \x01`\0\x01`\x1Ca\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP[`\xFF\x82\x16`\0\x90\x81R`\x02` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x01T\x90\x82\x01Ra\n\xFB\x90\x86a\r\xDEV[`\xFF\x83\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x84Q\x81U\x82\x85\x01Q`\x01\x90\x91\x01U\x80Q``\x81\x01\x82R\x83\x81R\x91\x82\x01\x83\x90R\x81\x01\x91\x90\x91R\x90\x94Pa\x0BA\x85a\t\xD0V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81Rc\xFF\xFF\xFF\xFFC\x81\x16` \x80\x84\x01\x91\x82R`\xFF\x90\x95\x16`\0\x90\x81R`\x01\x80\x87R`@\x80\x83 \x80T\x80\x84\x01\x82U\x90\x84R\x97\x90\x92 \x85Q\x97\x01\x80T\x93Q\x95\x83\x01Q\x85\x16`\x01`\xE0\x1B\x02`\x01`\x01`\xE0\x1B\x03\x96\x90\x95\x16`\x01`\xC0\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x94\x16\x97\x90\x92\x1C\x96\x90\x96\x17\x91\x90\x91\x17\x92\x90\x92\x16\x17\x90UP\x01a\n*V[PPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q\x15\x80\x15a\x0B\xF7WP` \x82\x01Q\x15[\x15a\x0C\x15WPP`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x84` \x01Qa\x0CZ\x91\x90a\x13\x84V[a\x0C\x84\x90\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDGa\x13mV[\x90R\x92\x91PPV[\x91\x90PV[\x81` \x01Qc\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\r*W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`J`$\x82\x01R\x7FBLSPubkeyRegistry._validateApkHa`D\x82\x01R\x7FshForQuorumAtBlockNumber: index `d\x82\x01Ri\x1D\x1B\xDB\xC8\x1C\x99X\xD9[\x9D`\xB2\x1B`\x84\x82\x01R`\xA4\x01a\x03qV[`@\x82\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x80a\rPWP\x81`@\x01Qc\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10[a\r\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`O`$\x82\x01R\x7FBLSPubkeyRegistry._validateApkHa`D\x82\x01R\x7FshForQuorumAtBlockNumber: not la`d\x82\x01Rntest apk update`\x88\x1B`\x84\x82\x01R`\xA4\x01a\x03qV[PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\r\xFAa\x0E\x81V[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R\x83Q`@\x80\x84\x01\x91\x90\x91R\x90\x84\x01Q``\x83\x01R`\0\x90\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a\x0E9Wa\x0E;V[\xFE[P\x80a\x0EyW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCBXY\x19\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x03qV[PP\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0E\xB4W`\0\x80\xFD[PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0E\xF6Wa\x0E\xF6a\x0E\xB7V[`@R\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a\x0F\x10W`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x0F3Wa\x0F3a\x0E\xB7V[`@R\x825\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0\x80`\0`\x80\x84\x86\x03\x12\x15a\x0FbW`\0\x80\xFD[\x835a\x0Fm\x81a\x0E\x9FV[\x92P` \x84\x81\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0F\x8BW`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x0F\x9FW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x0F\xB1Wa\x0F\xB1a\x0E\xB7V[a\x0F\xC3`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\x0E\xCDV[\x91P\x80\x82R\x88\x84\x82\x85\x01\x01\x11\x15a\x0F\xD9W`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x94PPPPa\x0F\xFD\x85`@\x86\x01a\x0E\xFEV[\x90P\x92P\x92P\x92V[\x805`\xFF\x81\x16\x81\x14a\x0C\x8CW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x10)W`\0\x80\xFD[a\x102\x82a\x10\x06V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x10KW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x10eW`\0\x80\xFD[a\x10n\x83a\x10\x06V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x10\x91W`\0\x80\xFD[a\x10\x9A\x84a\x10\x06V[\x92P` \x84\x015c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x10\xB3W`\0\x80\xFD[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x10\xD9W`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x10\xF1W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x11\x05W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x11\x14W`\0\x80\xFD[\x87` \x82\x85\x01\x01\x11\x15a\x11&W`\0\x80\xFD[` \x92\x83\x01\x98\x90\x97P\x95\x90\x91\x015\x94\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x11zW\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x11XV[P\x90\x96\x95PPPPPPV[` \x80\x82R`Q\x90\x82\x01R\x7FBLSPubkeyRegistry.onlyRegistryCo`@\x82\x01R\x7Fordinator: caller is not the reg``\x82\x01Rp4\xB9\xBA9<\x901\xB7\xB7\xB924\xB70\xBA7\xB9`y\x1B`\x80\x82\x01R`\xA0\x01\x90V[` \x80\x82R`@\x90\x82\x01\x81\x90R\x7FBLSPubkeyRegistry.registerOperat\x90\x82\x01R\x7For: operator does not own pubkey``\x82\x01R`\x80\x01\x90V[`\x01\x80`\xA0\x1B\x03\x83\x16\x81R`\0` `@\x81\x84\x01R\x83Q\x80`@\x85\x01R`\0[\x81\x81\x10\x15a\x12\x97W\x85\x81\x01\x83\x01Q\x85\x82\x01``\x01R\x82\x01a\x12{V[\x81\x81\x11\x15a\x12\xA9W`\0``\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01``\x01\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x12\xD2W`\0\x80\xFD[\x81Qa\x102\x81a\x0E\x9FV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a\x13&Wa\x13&a\x12\xF3V[\x03\x93\x92PPPV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x14\x15a\x13HWa\x13Ha\x12\xF3V[`\x01\x01\x93\x92PPPV[`\0`\0\x19\x82\x14\x15a\x13fWa\x13fa\x12\xF3V[P`\x01\x01\x90V[`\0\x82\x82\x10\x15a\x13\x7FWa\x13\x7Fa\x12\xF3V[P\x03\x90V[`\0\x82a\x13\xA1WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V\xFE\xA2dipfsX\"\x12 \x08rR\xA3\x16\":\xED\x12\"\xF8\xC4q\x8B-Q+.$;\xE0\x05CM\xFC\x8DR\xA9\x91\xA9\x82\x01dsolcC\0\x08\x0C\x003";
    /// The bytecode of the contract.
    pub static BLSPUBKEYREGISTRY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xB4W`\x005`\xE0\x1C\x80cm\x14\xA9\x87\x11a\0qW\x80cm\x14\xA9\x87\x14a\x01\xECW\x80cr%\x05~\x14a\x02\x13W\x80c\x7F^\xCC\xBB\x14a\x02aW\x80c\xC1\xAFk$\x14a\x02\xA2W\x80c\xED\xA1\x07c\x14a\x02\xCFW\x80c\xFB\x81\xA7\xBE\x14a\x02\xEFW`\0\x80\xFD[\x80c\x03\xCEK\xAD\x14a\0\xB9W\x80c\x18uH\xC8\x14a\0\xDFW\x80c$6\x9B*\x14a\x01\x1EW\x80c2\xDEc\x08\x14a\x013W\x80cG\xB3\x14\xE8\x14a\x01oW\x80cc\xA9E\x10\x14a\x01\x82W[`\0\x80\xFD[a\0\xCCa\0\xC76`\x04a\x0FMV[a\x03'V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x06\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xD6V[a\x011a\x01,6`\x04a\x0FMV[a\x04\xA1V[\0[a\x01Za\x01A6`\x04a\x10\x17V[`\x02` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T\x82V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\0\xD6V[a\x01\x06a\x01}6`\x04a\x109V[a\x05\x80V[a\x01\xD1a\x01\x906`\x04a\x10\x17V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01RP`\xFF\x16`\0\x90\x81R`\x02` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x01T\x90\x82\x01R\x90V[`@\x80Q\x82Q\x81R` \x92\x83\x01Q\x92\x81\x01\x92\x90\x92R\x01a\0\xD6V[a\x01\x06\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02&a\x02!6`\x04a\x10RV[a\x06\x12V[`@\x80Q\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R` \x80\x84\x01Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x83\x01\x91\x90\x91R\x92\x82\x01Q\x90\x92\x16\x90\x82\x01R``\x01a\0\xD6V[a\x02ta\x02o6`\x04a\x10RV[a\x06\xA4V[`@\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x94\x16\x84Rc\xFF\xFF\xFF\xFF\x92\x83\x16` \x85\x01R\x91\x16\x90\x82\x01R``\x01a\0\xD6V[a\x02\xB5a\x02\xB06`\x04a\x10|V[a\x06\xEFV[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x91\x16\x81R` \x01a\0\xD6V[a\x02\xE2a\x02\xDD6`\x04a\x10\xC4V[a\x07xV[`@Qa\0\xD6\x91\x90a\x11<V[a\x03\x12a\x02\xFD6`\x04a\x10\x17V[`\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0\xD6V[`\x003`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x03zW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03q\x90a\x11\x86V[`@Q\x80\x91\x03\x90\xFD[`\0a\x03\x85\x83a\t\xD0V[\x90P\x7F\xAD2(\xB6v\xF7\xD3\xCDB\x84\xA5D?\x17\xF1\x96+6\xE4\x91\xB3\n@\xB2@XI\xE5\x97\xBA_\xB5\x81\x14\x15a\x04\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`?`$\x82\x01R\x7FBLSPubkeyRegistry.registerOperat`D\x82\x01R\x7For: cannot register zero pubkey\0`d\x82\x01R`\x84\x01a\x03qV[\x84`\x01`\x01`\xA0\x1B\x03\x16a\x040\x82a\x05\x80V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x04VW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03q\x90a\x11\xFDV[a\x04`\x84\x84a\n\x13V[\x7FSX\xC5\xB4!y\x17\x8C\x8F\xC7WsJ\xC2\xA3\x19\x8F\x90q\xC7e\xEE\r\x83\x89!\x15%\xF5\0RF\x85\x85`@Qa\x04\x91\x92\x91\x90a\x12[V[`@Q\x80\x91\x03\x90\xA1\x94\x93PPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x04\xE9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03q\x90a\x11\x86V[`\0a\x04\xF4\x82a\t\xD0V[\x90P\x83`\x01`\x01`\xA0\x1B\x03\x16a\x05\t\x82a\x05\x80V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x05/W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03q\x90a\x11\xFDV[a\x05A\x83a\x05<\x84a\x0B\xD2V[a\n\x13V[\x7F\x14\xA5\x17+1.\x9D,\"\xB8F\x8F\x9Cp\xEC,\xAA\x9D\xE94\xFE8\x074\xFB\xC6\xF3\xBE\xFF+\x14\xBA\x84\x84`@Qa\x05r\x92\x91\x90a\x12[V[`@Q\x80\x91\x03\x90\xA1PPPPV[`@Qct]\xCDs`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xE8\xBB\x9A\xE6\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xE8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x0C\x91\x90a\x12\xC0V[\x92\x91PPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R`\xFF\x86\x16\x82R`\x01\x90R\x91\x90\x91 \x80T\x83\x90\x81\x10a\x06OWa\x06Oa\x12\xDDV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x84\x1B\x16\x82Rc\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x82\x04\x81\x16\x94\x83\x01\x94\x90\x94R`\x01`\xE0\x1B\x90\x04\x90\x92\x16\x90\x82\x01R\x90P\x92\x91PPV[`\x01` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x06\xC0W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`@\x81\x90\x1B\x92Pc\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x82\x04\x81\x16\x92P`\x01`\xE0\x1B\x90\x91\x04\x16\x83V[`\xFF\x83\x16`\0\x90\x81R`\x01` R`@\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x07\x16Wa\x07\x16a\x12\xDDV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x84\x1B\x16\x82Rc\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x82\x04\x81\x16\x94\x83\x01\x94\x90\x94R`\x01`\xE0\x1B\x90\x04\x90\x92\x16\x90\x82\x01R\x90Pa\x07o\x81\x85a\x0C\x91V[Q\x94\x93PPPPV[```\0\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\x95Wa\x07\x95a\x0E\xB7V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07\xBEW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x84\x81\x10\x15a\t\xC7W`\0\x86\x86\x83\x81\x81\x10a\x07\xE0Wa\x07\xE0a\x12\xDDV[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x01` R`@\x90 T\x90\x92P\x90Pc\xFF\xFF\xFF\xFF\x81\x16\x15\x80a\x08IWP`\xFF\x82\x16`\0\x90\x81R`\x01` R`@\x81 \x80T\x90\x91\x90a\x08-Wa\x08-a\x12\xDDV[`\0\x91\x82R` \x90\x91 \x01T`\x01`\xC0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x86\x10[\x15a\x08\xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`^`$\x82\x01R\x7FBLSPubkeyRegistry.getApkIndicesF`D\x82\x01R\x7ForQuorumsAtBlockNumber: blockNum`d\x82\x01R\x7Fber is before the first update\0\0`\x84\x82\x01R`\xA4\x01a\x03qV[`\0[\x81c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\t\xB1W`\xFF\x83\x16`\0\x90\x81R`\x01` \x81\x90R`@\x90\x91 \x88\x91a\t\x1A\x84\x86a\x13\tV[a\t$\x91\x90a\x13\tV[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\t:Wa\t:a\x12\xDDV[`\0\x91\x82R` \x90\x91 \x01T`\x01`\xC0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x11a\t\x9FW`\x01a\td\x82\x84a\x13\tV[a\tn\x91\x90a\x13\tV[\x85\x85\x81Q\x81\x10a\t\x80Wa\t\x80a\x12\xDDV[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPPa\t\xB1V[\x80a\t\xA9\x81a\x13.V[\x91PPa\x08\xE5V[PPP\x80\x80a\t\xBF\x90a\x13RV[\x91PPa\x07\xC4V[P\x94\x93PPPPV[`\0\x81`\0\x01Q\x82` \x01Q`@Q` \x01a\t\xF6\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0[\x83Q\x81\x10\x15a\x0B\xCCW`\0\x84\x82\x81Q\x81\x10a\nGWa\nGa\x12\xDDV[\x01` \x90\x81\x01Q`\xF8\x1C`\0\x81\x81R`\x01\x90\x92R`@\x90\x91 T\x90\x91P\x80\x15a\n\xC7W`\xFF\x82\x16`\0\x90\x81R`\x01` \x81\x90R`@\x90\x91 C\x91a\n\x8B\x90\x84a\x13mV[\x81T\x81\x10a\n\x9BWa\n\x9Ba\x12\xDDV[\x90`\0R` `\0 \x01`\0\x01`\x1Ca\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP[`\xFF\x82\x16`\0\x90\x81R`\x02` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x01T\x90\x82\x01Ra\n\xFB\x90\x86a\r\xDEV[`\xFF\x83\x16`\0\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x84Q\x81U\x82\x85\x01Q`\x01\x90\x91\x01U\x80Q``\x81\x01\x82R\x83\x81R\x91\x82\x01\x83\x90R\x81\x01\x91\x90\x91R\x90\x94Pa\x0BA\x85a\t\xD0V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81Rc\xFF\xFF\xFF\xFFC\x81\x16` \x80\x84\x01\x91\x82R`\xFF\x90\x95\x16`\0\x90\x81R`\x01\x80\x87R`@\x80\x83 \x80T\x80\x84\x01\x82U\x90\x84R\x97\x90\x92 \x85Q\x97\x01\x80T\x93Q\x95\x83\x01Q\x85\x16`\x01`\xE0\x1B\x02`\x01`\x01`\xE0\x1B\x03\x96\x90\x95\x16`\x01`\xC0\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x94\x16\x97\x90\x92\x1C\x96\x90\x96\x17\x91\x90\x91\x17\x92\x90\x92\x16\x17\x90UP\x01a\n*V[PPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q\x15\x80\x15a\x0B\xF7WP` \x82\x01Q\x15[\x15a\x0C\x15WPP`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\x84` \x01Qa\x0CZ\x91\x90a\x13\x84V[a\x0C\x84\x90\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDGa\x13mV[\x90R\x92\x91PPV[\x91\x90PV[\x81` \x01Qc\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a\r*W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`J`$\x82\x01R\x7FBLSPubkeyRegistry._validateApkHa`D\x82\x01R\x7FshForQuorumAtBlockNumber: index `d\x82\x01Ri\x1D\x1B\xDB\xC8\x1C\x99X\xD9[\x9D`\xB2\x1B`\x84\x82\x01R`\xA4\x01a\x03qV[`@\x82\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x80a\rPWP\x81`@\x01Qc\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10[a\r\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`O`$\x82\x01R\x7FBLSPubkeyRegistry._validateApkHa`D\x82\x01R\x7FshForQuorumAtBlockNumber: not la`d\x82\x01Rntest apk update`\x88\x1B`\x84\x82\x01R`\xA4\x01a\x03qV[PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\r\xFAa\x0E\x81V[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R\x83Q`@\x80\x84\x01\x91\x90\x91R\x90\x84\x01Q``\x83\x01R`\0\x90\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a\x0E9Wa\x0E;V[\xFE[P\x80a\x0EyW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCBXY\x19\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x03qV[PP\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0E\xB4W`\0\x80\xFD[PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0E\xF6Wa\x0E\xF6a\x0E\xB7V[`@R\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a\x0F\x10W`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x0F3Wa\x0F3a\x0E\xB7V[`@R\x825\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0\x80`\0`\x80\x84\x86\x03\x12\x15a\x0FbW`\0\x80\xFD[\x835a\x0Fm\x81a\x0E\x9FV[\x92P` \x84\x81\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0F\x8BW`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x0F\x9FW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x0F\xB1Wa\x0F\xB1a\x0E\xB7V[a\x0F\xC3`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\x0E\xCDV[\x91P\x80\x82R\x88\x84\x82\x85\x01\x01\x11\x15a\x0F\xD9W`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x94PPPPa\x0F\xFD\x85`@\x86\x01a\x0E\xFEV[\x90P\x92P\x92P\x92V[\x805`\xFF\x81\x16\x81\x14a\x0C\x8CW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x10)W`\0\x80\xFD[a\x102\x82a\x10\x06V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x10KW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x10eW`\0\x80\xFD[a\x10n\x83a\x10\x06V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x10\x91W`\0\x80\xFD[a\x10\x9A\x84a\x10\x06V[\x92P` \x84\x015c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x10\xB3W`\0\x80\xFD[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x10\xD9W`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x10\xF1W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x11\x05W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x11\x14W`\0\x80\xFD[\x87` \x82\x85\x01\x01\x11\x15a\x11&W`\0\x80\xFD[` \x92\x83\x01\x98\x90\x97P\x95\x90\x91\x015\x94\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x11zW\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x11XV[P\x90\x96\x95PPPPPPV[` \x80\x82R`Q\x90\x82\x01R\x7FBLSPubkeyRegistry.onlyRegistryCo`@\x82\x01R\x7Fordinator: caller is not the reg``\x82\x01Rp4\xB9\xBA9<\x901\xB7\xB7\xB924\xB70\xBA7\xB9`y\x1B`\x80\x82\x01R`\xA0\x01\x90V[` \x80\x82R`@\x90\x82\x01\x81\x90R\x7FBLSPubkeyRegistry.registerOperat\x90\x82\x01R\x7For: operator does not own pubkey``\x82\x01R`\x80\x01\x90V[`\x01\x80`\xA0\x1B\x03\x83\x16\x81R`\0` `@\x81\x84\x01R\x83Q\x80`@\x85\x01R`\0[\x81\x81\x10\x15a\x12\x97W\x85\x81\x01\x83\x01Q\x85\x82\x01``\x01R\x82\x01a\x12{V[\x81\x81\x11\x15a\x12\xA9W`\0``\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01``\x01\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x12\xD2W`\0\x80\xFD[\x81Qa\x102\x81a\x0E\x9FV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a\x13&Wa\x13&a\x12\xF3V[\x03\x93\x92PPPV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x81\x14\x15a\x13HWa\x13Ha\x12\xF3V[`\x01\x01\x93\x92PPPV[`\0`\0\x19\x82\x14\x15a\x13fWa\x13fa\x12\xF3V[P`\x01\x01\x90V[`\0\x82\x82\x10\x15a\x13\x7FWa\x13\x7Fa\x12\xF3V[P\x03\x90V[`\0\x82a\x13\xA1WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V\xFE\xA2dipfsX\"\x12 \x08rR\xA3\x16\":\xED\x12\"\xF8\xC4q\x8B-Q+.$;\xE0\x05CM\xFC\x8DR\xA9\x91\xA9\x82\x01dsolcC\0\x08\x0C\x003";
    /// The deployed bytecode of the contract.
    pub static BLSPUBKEYREGISTRY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct BLSPubkeyRegistry<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for BLSPubkeyRegistry<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for BLSPubkeyRegistry<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for BLSPubkeyRegistry<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for BLSPubkeyRegistry<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(BLSPubkeyRegistry))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> BLSPubkeyRegistry<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    BLSPUBKEYREGISTRY_ABI.clone(),
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
                BLSPUBKEYREGISTRY_ABI.clone(),
                BLSPUBKEYREGISTRY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `deregisterOperator` (0x24369b2a) function
        pub fn deregister_operator(
            &self,
            operator: ::ethers::core::types::Address,
            quorum_numbers: ::ethers::core::types::Bytes,
            pubkey: G1Point,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([36, 54, 155, 42], (operator, quorum_numbers, pubkey))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getApkForQuorum` (0x63a94510) function
        pub fn get_apk_for_quorum(
            &self,
            quorum_number: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, G1Point> {
            self.0
                .method_hash([99, 169, 69, 16], quorum_number)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getApkHashForQuorumAtBlockNumberFromIndex` (0xc1af6b24) function
        pub fn get_apk_hash_for_quorum_at_block_number_from_index(
            &self,
            quorum_number: u8,
            block_number: u32,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 24]> {
            self.0
                .method_hash([193, 175, 107, 36], (quorum_number, block_number, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getApkIndicesForQuorumsAtBlockNumber` (0xeda10763) function
        pub fn get_apk_indices_for_quorums_at_block_number(
            &self,
            quorum_numbers: ::ethers::core::types::Bytes,
            block_number: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<u32>> {
            self.0
                .method_hash([237, 161, 7, 99], (quorum_numbers, block_number))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getApkUpdateForQuorumByIndex` (0x7225057e) function
        pub fn get_apk_update_for_quorum_by_index(
            &self,
            quorum_number: u8,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ApkUpdate> {
            self.0
                .method_hash([114, 37, 5, 126], (quorum_number, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOperatorFromPubkeyHash` (0x47b314e8) function
        pub fn get_operator_from_pubkey_hash(
            &self,
            pubkey_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([71, 179, 20, 232], pubkey_hash)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getQuorumApkHistoryLength` (0xfb81a7be) function
        pub fn get_quorum_apk_history_length(
            &self,
            quorum_number: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([251, 129, 167, 190], quorum_number)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pubkeyCompendium` (0x187548c8) function
        pub fn pubkey_compendium(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([24, 117, 72, 200], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quorumApk` (0x32de6308) function
        pub fn quorum_apk(
            &self,
            p0: u8,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([50, 222, 99, 8], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quorumApkUpdates` (0x7f5eccbb) function
        pub fn quorum_apk_updates(
            &self,
            p0: u8,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ([u8; 24], u32, u32)> {
            self.0
                .method_hash([127, 94, 204, 187], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerOperator` (0x03ce4bad) function
        pub fn register_operator(
            &self,
            operator: ::ethers::core::types::Address,
            quorum_numbers: ::ethers::core::types::Bytes,
            pubkey: G1Point,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([3, 206, 75, 173], (operator, quorum_numbers, pubkey))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registryCoordinator` (0x6d14a987) function
        pub fn registry_coordinator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([109, 20, 169, 135], ())
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
        ///Gets the contract's `OperatorAddedToQuorums` event
        pub fn operator_added_to_quorums_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OperatorAddedToQuorumsFilter,
        > {
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
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BLSPubkeyRegistryEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for BLSPubkeyRegistry<M> {
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
        name = "OperatorAddedToQuorums",
        abi = "OperatorAddedToQuorums(address,bytes)"
    )]
    pub struct OperatorAddedToQuorumsFilter {
        pub operator: ::ethers::core::types::Address,
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
        Hash
    )]
    #[ethevent(
        name = "OperatorRemovedFromQuorums",
        abi = "OperatorRemovedFromQuorums(address,bytes)"
    )]
    pub struct OperatorRemovedFromQuorumsFilter {
        pub operator: ::ethers::core::types::Address,
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
        Hash
    )]
    pub enum BLSPubkeyRegistryEvents {
        InitializedFilter(InitializedFilter),
        OperatorAddedToQuorumsFilter(OperatorAddedToQuorumsFilter),
        OperatorRemovedFromQuorumsFilter(OperatorRemovedFromQuorumsFilter),
    }
    impl ::ethers::contract::EthLogDecode for BLSPubkeyRegistryEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(BLSPubkeyRegistryEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OperatorAddedToQuorumsFilter::decode_log(log) {
                return Ok(
                    BLSPubkeyRegistryEvents::OperatorAddedToQuorumsFilter(decoded),
                );
            }
            if let Ok(decoded) = OperatorRemovedFromQuorumsFilter::decode_log(log) {
                return Ok(
                    BLSPubkeyRegistryEvents::OperatorRemovedFromQuorumsFilter(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for BLSPubkeyRegistryEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorAddedToQuorumsFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorRemovedFromQuorumsFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<InitializedFilter> for BLSPubkeyRegistryEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OperatorAddedToQuorumsFilter>
    for BLSPubkeyRegistryEvents {
        fn from(value: OperatorAddedToQuorumsFilter) -> Self {
            Self::OperatorAddedToQuorumsFilter(value)
        }
    }
    impl ::core::convert::From<OperatorRemovedFromQuorumsFilter>
    for BLSPubkeyRegistryEvents {
        fn from(value: OperatorRemovedFromQuorumsFilter) -> Self {
            Self::OperatorRemovedFromQuorumsFilter(value)
        }
    }
    ///Container type for all input parameters for the `deregisterOperator` function with signature `deregisterOperator(address,bytes,(uint256,uint256))` and selector `0x24369b2a`
    #[derive(
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
        name = "deregisterOperator",
        abi = "deregisterOperator(address,bytes,(uint256,uint256))"
    )]
    pub struct DeregisterOperatorCall {
        pub operator: ::ethers::core::types::Address,
        pub quorum_numbers: ::ethers::core::types::Bytes,
        pub pubkey: G1Point,
    }
    ///Container type for all input parameters for the `getApkForQuorum` function with signature `getApkForQuorum(uint8)` and selector `0x63a94510`
    #[derive(
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
    #[ethcall(name = "getApkForQuorum", abi = "getApkForQuorum(uint8)")]
    pub struct GetApkForQuorumCall {
        pub quorum_number: u8,
    }
    ///Container type for all input parameters for the `getApkHashForQuorumAtBlockNumberFromIndex` function with signature `getApkHashForQuorumAtBlockNumberFromIndex(uint8,uint32,uint256)` and selector `0xc1af6b24`
    #[derive(
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
        name = "getApkHashForQuorumAtBlockNumberFromIndex",
        abi = "getApkHashForQuorumAtBlockNumberFromIndex(uint8,uint32,uint256)"
    )]
    pub struct GetApkHashForQuorumAtBlockNumberFromIndexCall {
        pub quorum_number: u8,
        pub block_number: u32,
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getApkIndicesForQuorumsAtBlockNumber` function with signature `getApkIndicesForQuorumsAtBlockNumber(bytes,uint256)` and selector `0xeda10763`
    #[derive(
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
        name = "getApkIndicesForQuorumsAtBlockNumber",
        abi = "getApkIndicesForQuorumsAtBlockNumber(bytes,uint256)"
    )]
    pub struct GetApkIndicesForQuorumsAtBlockNumberCall {
        pub quorum_numbers: ::ethers::core::types::Bytes,
        pub block_number: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getApkUpdateForQuorumByIndex` function with signature `getApkUpdateForQuorumByIndex(uint8,uint256)` and selector `0x7225057e`
    #[derive(
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
        name = "getApkUpdateForQuorumByIndex",
        abi = "getApkUpdateForQuorumByIndex(uint8,uint256)"
    )]
    pub struct GetApkUpdateForQuorumByIndexCall {
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
        Hash
    )]
    #[ethcall(
        name = "getOperatorFromPubkeyHash",
        abi = "getOperatorFromPubkeyHash(bytes32)"
    )]
    pub struct GetOperatorFromPubkeyHashCall {
        pub pubkey_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `getQuorumApkHistoryLength` function with signature `getQuorumApkHistoryLength(uint8)` and selector `0xfb81a7be`
    #[derive(
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
        name = "getQuorumApkHistoryLength",
        abi = "getQuorumApkHistoryLength(uint8)"
    )]
    pub struct GetQuorumApkHistoryLengthCall {
        pub quorum_number: u8,
    }
    ///Container type for all input parameters for the `pubkeyCompendium` function with signature `pubkeyCompendium()` and selector `0x187548c8`
    #[derive(
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
    #[ethcall(name = "pubkeyCompendium", abi = "pubkeyCompendium()")]
    pub struct PubkeyCompendiumCall;
    ///Container type for all input parameters for the `quorumApk` function with signature `quorumApk(uint8)` and selector `0x32de6308`
    #[derive(
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
    #[ethcall(name = "quorumApk", abi = "quorumApk(uint8)")]
    pub struct QuorumApkCall(pub u8);
    ///Container type for all input parameters for the `quorumApkUpdates` function with signature `quorumApkUpdates(uint8,uint256)` and selector `0x7f5eccbb`
    #[derive(
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
    #[ethcall(name = "quorumApkUpdates", abi = "quorumApkUpdates(uint8,uint256)")]
    pub struct QuorumApkUpdatesCall(pub u8, pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `registerOperator` function with signature `registerOperator(address,bytes,(uint256,uint256))` and selector `0x03ce4bad`
    #[derive(
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
        abi = "registerOperator(address,bytes,(uint256,uint256))"
    )]
    pub struct RegisterOperatorCall {
        pub operator: ::ethers::core::types::Address,
        pub quorum_numbers: ::ethers::core::types::Bytes,
        pub pubkey: G1Point,
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
        Hash
    )]
    #[ethcall(name = "registryCoordinator", abi = "registryCoordinator()")]
    pub struct RegistryCoordinatorCall;
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
    pub enum BLSPubkeyRegistryCalls {
        DeregisterOperator(DeregisterOperatorCall),
        GetApkForQuorum(GetApkForQuorumCall),
        GetApkHashForQuorumAtBlockNumberFromIndex(
            GetApkHashForQuorumAtBlockNumberFromIndexCall,
        ),
        GetApkIndicesForQuorumsAtBlockNumber(GetApkIndicesForQuorumsAtBlockNumberCall),
        GetApkUpdateForQuorumByIndex(GetApkUpdateForQuorumByIndexCall),
        GetOperatorFromPubkeyHash(GetOperatorFromPubkeyHashCall),
        GetQuorumApkHistoryLength(GetQuorumApkHistoryLengthCall),
        PubkeyCompendium(PubkeyCompendiumCall),
        QuorumApk(QuorumApkCall),
        QuorumApkUpdates(QuorumApkUpdatesCall),
        RegisterOperator(RegisterOperatorCall),
        RegistryCoordinator(RegistryCoordinatorCall),
    }
    impl ::ethers::core::abi::AbiDecode for BLSPubkeyRegistryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DeregisterOperatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeregisterOperator(decoded));
            }
            if let Ok(decoded) = <GetApkForQuorumCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetApkForQuorum(decoded));
            }
            if let Ok(decoded) = <GetApkHashForQuorumAtBlockNumberFromIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetApkHashForQuorumAtBlockNumberFromIndex(decoded));
            }
            if let Ok(decoded) = <GetApkIndicesForQuorumsAtBlockNumberCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetApkIndicesForQuorumsAtBlockNumber(decoded));
            }
            if let Ok(decoded) = <GetApkUpdateForQuorumByIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetApkUpdateForQuorumByIndex(decoded));
            }
            if let Ok(decoded) = <GetOperatorFromPubkeyHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetOperatorFromPubkeyHash(decoded));
            }
            if let Ok(decoded) = <GetQuorumApkHistoryLengthCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetQuorumApkHistoryLength(decoded));
            }
            if let Ok(decoded) = <PubkeyCompendiumCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PubkeyCompendium(decoded));
            }
            if let Ok(decoded) = <QuorumApkCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::QuorumApk(decoded));
            }
            if let Ok(decoded) = <QuorumApkUpdatesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::QuorumApkUpdates(decoded));
            }
            if let Ok(decoded) = <RegisterOperatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RegisterOperator(decoded));
            }
            if let Ok(decoded) = <RegistryCoordinatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RegistryCoordinator(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for BLSPubkeyRegistryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DeregisterOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetApkForQuorum(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetApkHashForQuorumAtBlockNumberFromIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetApkIndicesForQuorumsAtBlockNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetApkUpdateForQuorumByIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOperatorFromPubkeyHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetQuorumApkHistoryLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PubkeyCompendium(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QuorumApk(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QuorumApkUpdates(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegistryCoordinator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for BLSPubkeyRegistryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DeregisterOperator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetApkForQuorum(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetApkHashForQuorumAtBlockNumberFromIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetApkIndicesForQuorumsAtBlockNumber(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetApkUpdateForQuorumByIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetOperatorFromPubkeyHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetQuorumApkHistoryLength(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PubkeyCompendium(element) => ::core::fmt::Display::fmt(element, f),
                Self::QuorumApk(element) => ::core::fmt::Display::fmt(element, f),
                Self::QuorumApkUpdates(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegistryCoordinator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<DeregisterOperatorCall> for BLSPubkeyRegistryCalls {
        fn from(value: DeregisterOperatorCall) -> Self {
            Self::DeregisterOperator(value)
        }
    }
    impl ::core::convert::From<GetApkForQuorumCall> for BLSPubkeyRegistryCalls {
        fn from(value: GetApkForQuorumCall) -> Self {
            Self::GetApkForQuorum(value)
        }
    }
    impl ::core::convert::From<GetApkHashForQuorumAtBlockNumberFromIndexCall>
    for BLSPubkeyRegistryCalls {
        fn from(value: GetApkHashForQuorumAtBlockNumberFromIndexCall) -> Self {
            Self::GetApkHashForQuorumAtBlockNumberFromIndex(value)
        }
    }
    impl ::core::convert::From<GetApkIndicesForQuorumsAtBlockNumberCall>
    for BLSPubkeyRegistryCalls {
        fn from(value: GetApkIndicesForQuorumsAtBlockNumberCall) -> Self {
            Self::GetApkIndicesForQuorumsAtBlockNumber(value)
        }
    }
    impl ::core::convert::From<GetApkUpdateForQuorumByIndexCall>
    for BLSPubkeyRegistryCalls {
        fn from(value: GetApkUpdateForQuorumByIndexCall) -> Self {
            Self::GetApkUpdateForQuorumByIndex(value)
        }
    }
    impl ::core::convert::From<GetOperatorFromPubkeyHashCall>
    for BLSPubkeyRegistryCalls {
        fn from(value: GetOperatorFromPubkeyHashCall) -> Self {
            Self::GetOperatorFromPubkeyHash(value)
        }
    }
    impl ::core::convert::From<GetQuorumApkHistoryLengthCall>
    for BLSPubkeyRegistryCalls {
        fn from(value: GetQuorumApkHistoryLengthCall) -> Self {
            Self::GetQuorumApkHistoryLength(value)
        }
    }
    impl ::core::convert::From<PubkeyCompendiumCall> for BLSPubkeyRegistryCalls {
        fn from(value: PubkeyCompendiumCall) -> Self {
            Self::PubkeyCompendium(value)
        }
    }
    impl ::core::convert::From<QuorumApkCall> for BLSPubkeyRegistryCalls {
        fn from(value: QuorumApkCall) -> Self {
            Self::QuorumApk(value)
        }
    }
    impl ::core::convert::From<QuorumApkUpdatesCall> for BLSPubkeyRegistryCalls {
        fn from(value: QuorumApkUpdatesCall) -> Self {
            Self::QuorumApkUpdates(value)
        }
    }
    impl ::core::convert::From<RegisterOperatorCall> for BLSPubkeyRegistryCalls {
        fn from(value: RegisterOperatorCall) -> Self {
            Self::RegisterOperator(value)
        }
    }
    impl ::core::convert::From<RegistryCoordinatorCall> for BLSPubkeyRegistryCalls {
        fn from(value: RegistryCoordinatorCall) -> Self {
            Self::RegistryCoordinator(value)
        }
    }
    ///Container type for all return fields from the `getApkForQuorum` function with signature `getApkForQuorum(uint8)` and selector `0x63a94510`
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
    pub struct GetApkForQuorumReturn(pub G1Point);
    ///Container type for all return fields from the `getApkHashForQuorumAtBlockNumberFromIndex` function with signature `getApkHashForQuorumAtBlockNumberFromIndex(uint8,uint32,uint256)` and selector `0xc1af6b24`
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
    pub struct GetApkHashForQuorumAtBlockNumberFromIndexReturn(pub [u8; 24]);
    ///Container type for all return fields from the `getApkIndicesForQuorumsAtBlockNumber` function with signature `getApkIndicesForQuorumsAtBlockNumber(bytes,uint256)` and selector `0xeda10763`
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
    pub struct GetApkIndicesForQuorumsAtBlockNumberReturn(pub ::std::vec::Vec<u32>);
    ///Container type for all return fields from the `getApkUpdateForQuorumByIndex` function with signature `getApkUpdateForQuorumByIndex(uint8,uint256)` and selector `0x7225057e`
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
    pub struct GetApkUpdateForQuorumByIndexReturn(pub ApkUpdate);
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
        Hash
    )]
    pub struct GetOperatorFromPubkeyHashReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getQuorumApkHistoryLength` function with signature `getQuorumApkHistoryLength(uint8)` and selector `0xfb81a7be`
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
    pub struct GetQuorumApkHistoryLengthReturn(pub u32);
    ///Container type for all return fields from the `pubkeyCompendium` function with signature `pubkeyCompendium()` and selector `0x187548c8`
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
    pub struct PubkeyCompendiumReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `quorumApk` function with signature `quorumApk(uint8)` and selector `0x32de6308`
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
    pub struct QuorumApkReturn {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `quorumApkUpdates` function with signature `quorumApkUpdates(uint8,uint256)` and selector `0x7f5eccbb`
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
    pub struct QuorumApkUpdatesReturn {
        pub apk_hash: [u8; 24],
        pub update_block_number: u32,
        pub next_update_block_number: u32,
    }
    ///Container type for all return fields from the `registerOperator` function with signature `registerOperator(address,bytes,(uint256,uint256))` and selector `0x03ce4bad`
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
    pub struct RegisterOperatorReturn(pub [u8; 32]);
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
        Hash
    )]
    pub struct RegistryCoordinatorReturn(pub ::ethers::core::types::Address);
}
