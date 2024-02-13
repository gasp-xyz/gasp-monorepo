pub use bls_public_key_compendium_mock::*;
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
pub mod bls_public_key_compendium_mock {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getMessageHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getMessageHash"),
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
                    ::std::borrow::ToOwned::to_owned("operatorToPubkeyHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "operatorToPubkeyHash",
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
                    ::std::borrow::ToOwned::to_owned("pubkeyHashToOperator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "pubkeyHashToOperator",
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
                    ::std::borrow::ToOwned::to_owned("registerBLSPublicKey"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "registerBLSPublicKey",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("signedMessageHash"),
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
                                    name: ::std::borrow::ToOwned::to_owned("pubkeyG1"),
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
                                    name: ::std::borrow::ToOwned::to_owned("pubkeyG2"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
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
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct BN254.G2Point"),
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
                    ::std::borrow::ToOwned::to_owned("registerPublicKey"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("registerPublicKey"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pk"),
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
                    ::std::borrow::ToOwned::to_owned("setBLSPublicKey"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setBLSPublicKey"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pk"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("NewPubkeyRegistration"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NewPubkeyRegistration",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pubkeyG1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pubkeyG2"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
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
    pub static BLSPUBLICKEYCOMPENDIUMMOCK_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x04U\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0bW`\x005`\xE0\x1C\x80c\x16\x1A3M\x14a\0gW\x80c\x1FZ\xC1\xB2\x14a\0|W\x80c\xA6h2\xFC\x14a\0\xC2W\x80c\xDE)\xFA\xC0\x14a\0\xD5W\x80c\xDFM\t\xE0\x14a\x01\x03W\x80c\xE8\xBB\x9A\xE6\x14a\x01\x16W[`\0\x80\xFD[a\0za\0u6`\x04a\x03\x04V[PPPV[\0[a\0\xA2a\0\x8A6`\x04a\x03\x94V[P`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x90V[`@\x80Q\x82Q\x81R` \x92\x83\x01Q\x92\x81\x01\x92\x90\x92R\x01[`@Q\x80\x91\x03\x90\xF3[a\0za\0\xD06`\x04a\x03\xB6V[a\x01\\V[a\0\xF5a\0\xE36`\x04a\x03\x94V[`\0` \x81\x90R\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01a\0\xB9V[a\0za\x01\x116`\x04a\x03\xD2V[a\x01\x9AV[a\x01?a\x01$6`\x04a\x04\x06V[`\x01` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xB9V[\x91\x90PV[`\0a\x01g\x82a\x01\xE3V[3`\0\x81\x81R` \x81\x81R`@\x80\x83 \x85\x90U\x93\x82R`\x01\x90R\x91\x90\x91 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90\x91\x17\x90UPPV[`\0a\x01\xA5\x82a\x01\xE3V[`\x01`\x01`\xA0\x1B\x03\x90\x93\x16`\0\x81\x81R` \x81\x81R`@\x80\x83 \x87\x90U\x95\x82R`\x01\x90R\x93\x90\x93 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90\x93\x17\x90\x92UPPV[`\0\x81`\0\x01Q\x82` \x01Q`@Q` \x01a\x02\t\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x02_Wa\x02_a\x02&V[`@R\x90V[`\0`@\x82\x84\x03\x12\x15a\x02wW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x02\x9AWa\x02\x9Aa\x02&V[`@R\x825\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x02\xC5W`\0\x80\xFD[a\x02\xCDa\x02<V[\x80`@\x84\x01\x85\x81\x11\x15a\x02\xDFW`\0\x80\xFD[\x84[\x81\x81\x10\x15a\x02\xF9W\x805\x84R` \x93\x84\x01\x93\x01a\x02\xE1V[P\x90\x95\x94PPPPPV[`\0\x80`\0\x83\x85\x03a\x01\0\x81\x12\x15a\x03\x1BW`\0\x80\xFD[a\x03%\x86\x86a\x02eV[\x93Pa\x034\x86`@\x87\x01a\x02eV[\x92P`\x80`\x7F\x19\x82\x01\x12\x15a\x03HW`\0\x80\xFD[Pa\x03Qa\x02<V[a\x03^\x86`\x80\x87\x01a\x02\xB4V[\x81Ra\x03m\x86`\xC0\x87\x01a\x02\xB4V[` \x82\x01R\x80\x91PP\x92P\x92P\x92V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01WW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x03\xA6W`\0\x80\xFD[a\x03\xAF\x82a\x03}V[\x93\x92PPPV[`\0`@\x82\x84\x03\x12\x15a\x03\xC8W`\0\x80\xFD[a\x03\xAF\x83\x83a\x02eV[`\0\x80``\x83\x85\x03\x12\x15a\x03\xE5W`\0\x80\xFD[a\x03\xEE\x83a\x03}V[\x91Pa\x03\xFD\x84` \x85\x01a\x02eV[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x04\x18W`\0\x80\xFD[P5\x91\x90PV\xFE\xA2dipfsX\"\x12 O\xE1\xEEv4 \xEA4n~\xDE)\xB6%\x8B\xB7\x9A\x0B\xF3T\x92jV\x8C\xC0g^XA#\xAA\x17dsolcC\0\x08\x0C\x003";
    /// The bytecode of the contract.
    pub static BLSPUBLICKEYCOMPENDIUMMOCK_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0bW`\x005`\xE0\x1C\x80c\x16\x1A3M\x14a\0gW\x80c\x1FZ\xC1\xB2\x14a\0|W\x80c\xA6h2\xFC\x14a\0\xC2W\x80c\xDE)\xFA\xC0\x14a\0\xD5W\x80c\xDFM\t\xE0\x14a\x01\x03W\x80c\xE8\xBB\x9A\xE6\x14a\x01\x16W[`\0\x80\xFD[a\0za\0u6`\x04a\x03\x04V[PPPV[\0[a\0\xA2a\0\x8A6`\x04a\x03\x94V[P`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x90V[`@\x80Q\x82Q\x81R` \x92\x83\x01Q\x92\x81\x01\x92\x90\x92R\x01[`@Q\x80\x91\x03\x90\xF3[a\0za\0\xD06`\x04a\x03\xB6V[a\x01\\V[a\0\xF5a\0\xE36`\x04a\x03\x94V[`\0` \x81\x90R\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01a\0\xB9V[a\0za\x01\x116`\x04a\x03\xD2V[a\x01\x9AV[a\x01?a\x01$6`\x04a\x04\x06V[`\x01` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xB9V[\x91\x90PV[`\0a\x01g\x82a\x01\xE3V[3`\0\x81\x81R` \x81\x81R`@\x80\x83 \x85\x90U\x93\x82R`\x01\x90R\x91\x90\x91 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90\x91\x17\x90UPPV[`\0a\x01\xA5\x82a\x01\xE3V[`\x01`\x01`\xA0\x1B\x03\x90\x93\x16`\0\x81\x81R` \x81\x81R`@\x80\x83 \x87\x90U\x95\x82R`\x01\x90R\x93\x90\x93 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90\x93\x17\x90\x92UPPV[`\0\x81`\0\x01Q\x82` \x01Q`@Q` \x01a\x02\t\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x02_Wa\x02_a\x02&V[`@R\x90V[`\0`@\x82\x84\x03\x12\x15a\x02wW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x02\x9AWa\x02\x9Aa\x02&V[`@R\x825\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x02\xC5W`\0\x80\xFD[a\x02\xCDa\x02<V[\x80`@\x84\x01\x85\x81\x11\x15a\x02\xDFW`\0\x80\xFD[\x84[\x81\x81\x10\x15a\x02\xF9W\x805\x84R` \x93\x84\x01\x93\x01a\x02\xE1V[P\x90\x95\x94PPPPPV[`\0\x80`\0\x83\x85\x03a\x01\0\x81\x12\x15a\x03\x1BW`\0\x80\xFD[a\x03%\x86\x86a\x02eV[\x93Pa\x034\x86`@\x87\x01a\x02eV[\x92P`\x80`\x7F\x19\x82\x01\x12\x15a\x03HW`\0\x80\xFD[Pa\x03Qa\x02<V[a\x03^\x86`\x80\x87\x01a\x02\xB4V[\x81Ra\x03m\x86`\xC0\x87\x01a\x02\xB4V[` \x82\x01R\x80\x91PP\x92P\x92P\x92V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01WW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x03\xA6W`\0\x80\xFD[a\x03\xAF\x82a\x03}V[\x93\x92PPPV[`\0`@\x82\x84\x03\x12\x15a\x03\xC8W`\0\x80\xFD[a\x03\xAF\x83\x83a\x02eV[`\0\x80``\x83\x85\x03\x12\x15a\x03\xE5W`\0\x80\xFD[a\x03\xEE\x83a\x03}V[\x91Pa\x03\xFD\x84` \x85\x01a\x02eV[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x04\x18W`\0\x80\xFD[P5\x91\x90PV\xFE\xA2dipfsX\"\x12 O\xE1\xEEv4 \xEA4n~\xDE)\xB6%\x8B\xB7\x9A\x0B\xF3T\x92jV\x8C\xC0g^XA#\xAA\x17dsolcC\0\x08\x0C\x003";
    /// The deployed bytecode of the contract.
    pub static BLSPUBLICKEYCOMPENDIUMMOCK_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct BLSPublicKeyCompendiumMock<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for BLSPublicKeyCompendiumMock<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for BLSPublicKeyCompendiumMock<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for BLSPublicKeyCompendiumMock<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for BLSPublicKeyCompendiumMock<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(BLSPublicKeyCompendiumMock))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> BLSPublicKeyCompendiumMock<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    BLSPUBLICKEYCOMPENDIUMMOCK_ABI.clone(),
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
                BLSPUBLICKEYCOMPENDIUMMOCK_ABI.clone(),
                BLSPUBLICKEYCOMPENDIUMMOCK_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `getMessageHash` (0x1f5ac1b2) function
        pub fn get_message_hash(
            &self,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, G1Point> {
            self.0
                .method_hash([31, 90, 193, 178], operator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operatorToPubkeyHash` (0xde29fac0) function
        pub fn operator_to_pubkey_hash(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([222, 41, 250, 192], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pubkeyHashToOperator` (0xe8bb9ae6) function
        pub fn pubkey_hash_to_operator(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([232, 187, 154, 230], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerBLSPublicKey` (0x161a334d) function
        pub fn register_bls_public_key(
            &self,
            signed_message_hash: G1Point,
            pubkey_g1: G1Point,
            pubkey_g2: G2Point,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [22, 26, 51, 77],
                    (signed_message_hash, pubkey_g1, pubkey_g2),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerPublicKey` (0xa66832fc) function
        pub fn register_public_key(
            &self,
            pk: G1Point,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([166, 104, 50, 252], (pk,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setBLSPublicKey` (0xdf4d09e0) function
        pub fn set_bls_public_key(
            &self,
            account: ::ethers::core::types::Address,
            pk: G1Point,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([223, 77, 9, 224], (account, pk))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `NewPubkeyRegistration` event
        pub fn new_pubkey_registration_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NewPubkeyRegistrationFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NewPubkeyRegistrationFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for BLSPublicKeyCompendiumMock<M> {
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
        name = "NewPubkeyRegistration",
        abi = "NewPubkeyRegistration(address,(uint256,uint256),(uint256[2],uint256[2]))"
    )]
    pub struct NewPubkeyRegistrationFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        pub pubkey_g1: G1Point,
        pub pubkey_g2: G2Point,
    }
    ///Container type for all input parameters for the `getMessageHash` function with signature `getMessageHash(address)` and selector `0x1f5ac1b2`
    #[derive(
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
    #[ethcall(name = "getMessageHash", abi = "getMessageHash(address)")]
    pub struct GetMessageHashCall {
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `operatorToPubkeyHash` function with signature `operatorToPubkeyHash(address)` and selector `0xde29fac0`
    #[derive(
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
    #[ethcall(name = "operatorToPubkeyHash", abi = "operatorToPubkeyHash(address)")]
    pub struct OperatorToPubkeyHashCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `pubkeyHashToOperator` function with signature `pubkeyHashToOperator(bytes32)` and selector `0xe8bb9ae6`
    #[derive(
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
    #[ethcall(name = "pubkeyHashToOperator", abi = "pubkeyHashToOperator(bytes32)")]
    pub struct PubkeyHashToOperatorCall(pub [u8; 32]);
    ///Container type for all input parameters for the `registerBLSPublicKey` function with signature `registerBLSPublicKey((uint256,uint256),(uint256,uint256),(uint256[2],uint256[2]))` and selector `0x161a334d`
    #[derive(
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
        name = "registerBLSPublicKey",
        abi = "registerBLSPublicKey((uint256,uint256),(uint256,uint256),(uint256[2],uint256[2]))"
    )]
    pub struct RegisterBLSPublicKeyCall {
        pub signed_message_hash: G1Point,
        pub pubkey_g1: G1Point,
        pub pubkey_g2: G2Point,
    }
    ///Container type for all input parameters for the `registerPublicKey` function with signature `registerPublicKey((uint256,uint256))` and selector `0xa66832fc`
    #[derive(
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
    #[ethcall(name = "registerPublicKey", abi = "registerPublicKey((uint256,uint256))")]
    pub struct RegisterPublicKeyCall {
        pub pk: G1Point,
    }
    ///Container type for all input parameters for the `setBLSPublicKey` function with signature `setBLSPublicKey(address,(uint256,uint256))` and selector `0xdf4d09e0`
    #[derive(
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
        name = "setBLSPublicKey",
        abi = "setBLSPublicKey(address,(uint256,uint256))"
    )]
    pub struct SetBLSPublicKeyCall {
        pub account: ::ethers::core::types::Address,
        pub pk: G1Point,
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
    pub enum BLSPublicKeyCompendiumMockCalls {
        GetMessageHash(GetMessageHashCall),
        OperatorToPubkeyHash(OperatorToPubkeyHashCall),
        PubkeyHashToOperator(PubkeyHashToOperatorCall),
        RegisterBLSPublicKey(RegisterBLSPublicKeyCall),
        RegisterPublicKey(RegisterPublicKeyCall),
        SetBLSPublicKey(SetBLSPublicKeyCall),
    }
    impl ::ethers::core::abi::AbiDecode for BLSPublicKeyCompendiumMockCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetMessageHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetMessageHash(decoded));
            }
            if let Ok(decoded) = <OperatorToPubkeyHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OperatorToPubkeyHash(decoded));
            }
            if let Ok(decoded) = <PubkeyHashToOperatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PubkeyHashToOperator(decoded));
            }
            if let Ok(decoded) = <RegisterBLSPublicKeyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RegisterBLSPublicKey(decoded));
            }
            if let Ok(decoded) = <RegisterPublicKeyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RegisterPublicKey(decoded));
            }
            if let Ok(decoded) = <SetBLSPublicKeyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetBLSPublicKey(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for BLSPublicKeyCompendiumMockCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetMessageHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OperatorToPubkeyHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PubkeyHashToOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterBLSPublicKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterPublicKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetBLSPublicKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for BLSPublicKeyCompendiumMockCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetMessageHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorToPubkeyHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PubkeyHashToOperator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RegisterBLSPublicKey(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RegisterPublicKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetBLSPublicKey(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetMessageHashCall> for BLSPublicKeyCompendiumMockCalls {
        fn from(value: GetMessageHashCall) -> Self {
            Self::GetMessageHash(value)
        }
    }
    impl ::core::convert::From<OperatorToPubkeyHashCall>
    for BLSPublicKeyCompendiumMockCalls {
        fn from(value: OperatorToPubkeyHashCall) -> Self {
            Self::OperatorToPubkeyHash(value)
        }
    }
    impl ::core::convert::From<PubkeyHashToOperatorCall>
    for BLSPublicKeyCompendiumMockCalls {
        fn from(value: PubkeyHashToOperatorCall) -> Self {
            Self::PubkeyHashToOperator(value)
        }
    }
    impl ::core::convert::From<RegisterBLSPublicKeyCall>
    for BLSPublicKeyCompendiumMockCalls {
        fn from(value: RegisterBLSPublicKeyCall) -> Self {
            Self::RegisterBLSPublicKey(value)
        }
    }
    impl ::core::convert::From<RegisterPublicKeyCall>
    for BLSPublicKeyCompendiumMockCalls {
        fn from(value: RegisterPublicKeyCall) -> Self {
            Self::RegisterPublicKey(value)
        }
    }
    impl ::core::convert::From<SetBLSPublicKeyCall> for BLSPublicKeyCompendiumMockCalls {
        fn from(value: SetBLSPublicKeyCall) -> Self {
            Self::SetBLSPublicKey(value)
        }
    }
    ///Container type for all return fields from the `getMessageHash` function with signature `getMessageHash(address)` and selector `0x1f5ac1b2`
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
    pub struct GetMessageHashReturn(pub G1Point);
    ///Container type for all return fields from the `operatorToPubkeyHash` function with signature `operatorToPubkeyHash(address)` and selector `0xde29fac0`
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
    pub struct OperatorToPubkeyHashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `pubkeyHashToOperator` function with signature `pubkeyHashToOperator(bytes32)` and selector `0xe8bb9ae6`
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
    pub struct PubkeyHashToOperatorReturn(pub ::ethers::core::types::Address);
}
