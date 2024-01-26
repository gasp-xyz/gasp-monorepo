pub use bls_pubkey_registry_storage::*;
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
pub mod bls_pubkey_registry_storage {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
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
    pub static BLSPUBKEYREGISTRYSTORAGE_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct BLSPubkeyRegistryStorage<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for BLSPubkeyRegistryStorage<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for BLSPubkeyRegistryStorage<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for BLSPubkeyRegistryStorage<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for BLSPubkeyRegistryStorage<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(BLSPubkeyRegistryStorage))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> BLSPubkeyRegistryStorage<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    BLSPUBKEYREGISTRYSTORAGE_ABI.clone(),
                    client,
                ),
            )
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
            BLSPubkeyRegistryStorageEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for BLSPubkeyRegistryStorage<M> {
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
    pub enum BLSPubkeyRegistryStorageEvents {
        InitializedFilter(InitializedFilter),
        OperatorAddedToQuorumsFilter(OperatorAddedToQuorumsFilter),
        OperatorRemovedFromQuorumsFilter(OperatorRemovedFromQuorumsFilter),
    }
    impl ::ethers::contract::EthLogDecode for BLSPubkeyRegistryStorageEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(BLSPubkeyRegistryStorageEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OperatorAddedToQuorumsFilter::decode_log(log) {
                return Ok(
                    BLSPubkeyRegistryStorageEvents::OperatorAddedToQuorumsFilter(decoded),
                );
            }
            if let Ok(decoded) = OperatorRemovedFromQuorumsFilter::decode_log(log) {
                return Ok(
                    BLSPubkeyRegistryStorageEvents::OperatorRemovedFromQuorumsFilter(
                        decoded,
                    ),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for BLSPubkeyRegistryStorageEvents {
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
    impl ::core::convert::From<InitializedFilter> for BLSPubkeyRegistryStorageEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OperatorAddedToQuorumsFilter>
    for BLSPubkeyRegistryStorageEvents {
        fn from(value: OperatorAddedToQuorumsFilter) -> Self {
            Self::OperatorAddedToQuorumsFilter(value)
        }
    }
    impl ::core::convert::From<OperatorRemovedFromQuorumsFilter>
    for BLSPubkeyRegistryStorageEvents {
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
    pub enum BLSPubkeyRegistryStorageCalls {
        DeregisterOperator(DeregisterOperatorCall),
        GetApkForQuorum(GetApkForQuorumCall),
        GetApkHashForQuorumAtBlockNumberFromIndex(
            GetApkHashForQuorumAtBlockNumberFromIndexCall,
        ),
        GetApkIndicesForQuorumsAtBlockNumber(GetApkIndicesForQuorumsAtBlockNumberCall),
        GetApkUpdateForQuorumByIndex(GetApkUpdateForQuorumByIndexCall),
        GetOperatorFromPubkeyHash(GetOperatorFromPubkeyHashCall),
        PubkeyCompendium(PubkeyCompendiumCall),
        QuorumApk(QuorumApkCall),
        QuorumApkUpdates(QuorumApkUpdatesCall),
        RegisterOperator(RegisterOperatorCall),
        RegistryCoordinator(RegistryCoordinatorCall),
    }
    impl ::ethers::core::abi::AbiDecode for BLSPubkeyRegistryStorageCalls {
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
    impl ::ethers::core::abi::AbiEncode for BLSPubkeyRegistryStorageCalls {
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
    impl ::core::fmt::Display for BLSPubkeyRegistryStorageCalls {
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
    impl ::core::convert::From<DeregisterOperatorCall>
    for BLSPubkeyRegistryStorageCalls {
        fn from(value: DeregisterOperatorCall) -> Self {
            Self::DeregisterOperator(value)
        }
    }
    impl ::core::convert::From<GetApkForQuorumCall> for BLSPubkeyRegistryStorageCalls {
        fn from(value: GetApkForQuorumCall) -> Self {
            Self::GetApkForQuorum(value)
        }
    }
    impl ::core::convert::From<GetApkHashForQuorumAtBlockNumberFromIndexCall>
    for BLSPubkeyRegistryStorageCalls {
        fn from(value: GetApkHashForQuorumAtBlockNumberFromIndexCall) -> Self {
            Self::GetApkHashForQuorumAtBlockNumberFromIndex(value)
        }
    }
    impl ::core::convert::From<GetApkIndicesForQuorumsAtBlockNumberCall>
    for BLSPubkeyRegistryStorageCalls {
        fn from(value: GetApkIndicesForQuorumsAtBlockNumberCall) -> Self {
            Self::GetApkIndicesForQuorumsAtBlockNumber(value)
        }
    }
    impl ::core::convert::From<GetApkUpdateForQuorumByIndexCall>
    for BLSPubkeyRegistryStorageCalls {
        fn from(value: GetApkUpdateForQuorumByIndexCall) -> Self {
            Self::GetApkUpdateForQuorumByIndex(value)
        }
    }
    impl ::core::convert::From<GetOperatorFromPubkeyHashCall>
    for BLSPubkeyRegistryStorageCalls {
        fn from(value: GetOperatorFromPubkeyHashCall) -> Self {
            Self::GetOperatorFromPubkeyHash(value)
        }
    }
    impl ::core::convert::From<PubkeyCompendiumCall> for BLSPubkeyRegistryStorageCalls {
        fn from(value: PubkeyCompendiumCall) -> Self {
            Self::PubkeyCompendium(value)
        }
    }
    impl ::core::convert::From<QuorumApkCall> for BLSPubkeyRegistryStorageCalls {
        fn from(value: QuorumApkCall) -> Self {
            Self::QuorumApk(value)
        }
    }
    impl ::core::convert::From<QuorumApkUpdatesCall> for BLSPubkeyRegistryStorageCalls {
        fn from(value: QuorumApkUpdatesCall) -> Self {
            Self::QuorumApkUpdates(value)
        }
    }
    impl ::core::convert::From<RegisterOperatorCall> for BLSPubkeyRegistryStorageCalls {
        fn from(value: RegisterOperatorCall) -> Self {
            Self::RegisterOperator(value)
        }
    }
    impl ::core::convert::From<RegistryCoordinatorCall>
    for BLSPubkeyRegistryStorageCalls {
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
