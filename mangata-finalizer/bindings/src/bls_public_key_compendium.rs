pub use bls_public_key_compendium::*;
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
pub mod bls_public_key_compendium {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getMessageHash"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getMessageHash"),
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
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct BN254.G1Point"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("operatorToPubkeyHash"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("operatorToPubkeyHash",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("pubkeyHashToOperator"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pubkeyHashToOperator",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("registerBLSPublicKey"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("registerBLSPublicKey",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("signedMessageHash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct BN254.G1Point"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("pubkeyG1"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct BN254.G1Point"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("pubkeyG2"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
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
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct BN254.G2Point"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([(
                ::std::borrow::ToOwned::to_owned("NewPubkeyRegistration"),
                ::std::vec![::ethers::core::abi::ethabi::Event {
                    name: ::std::borrow::ToOwned::to_owned("NewPubkeyRegistration",),
                    inputs: ::std::vec![
                        ::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("operator"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: true,
                        },
                        ::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("pubkeyG1"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            indexed: false,
                        },
                        ::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("pubkeyG2"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
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
                            ],),
                            indexed: false,
                        },
                    ],
                    anonymous: false,
                },],
            )]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static BLSPUBLICKEYCOMPENDIUM_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x0F\x81\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80c\x16\x1A3M\x14a\0QW\x80c\x1FZ\xC1\xB2\x14a\0fW\x80c\xDE)\xFA\xC0\x14a\0\x8FW\x80c\xE8\xBB\x9A\xE6\x14a\0\xBDW[`\0\x80\xFD[a\0da\0_6`\x04a\r\x03V[a\0\xFEV[\0[a\0ya\0t6`\x04a\r|V[a\x04[V[`@Qa\0\x86\x91\x90a\r\xACV[`@Q\x80\x91\x03\x90\xF3[a\0\xAFa\0\x9D6`\x04a\r|V[`\0` \x81\x90R\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01a\0\x86V[a\0\xE6a\0\xCB6`\x04a\r\xC3V[`\x01` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\x86V[`\0a\x01\t\x83a\x04\xF6V[3`\0\x90\x81R` \x81\x90R`@\x90 T\x90\x91P\x15a\x01\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`O`$\x82\x01R\x7FBLSPublicKeyCompendium.registerB`D\x82\x01R\x7FLSPublicKey: operator already re`d\x82\x01Rngistered pubkey`\x88\x1B`\x84\x82\x01R`\xA4\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x81\x81R`\x01` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x02JW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`J`$\x82\x01R\x7FBLSPublicKeyCompendium.registerB`D\x82\x01R\x7FLSPublicKey: public key already `d\x82\x01Ri\x1C\x99Y\xDA\\\xDD\x19\\\x99Y`\xB2\x1B`\x84\x82\x01R`\xA4\x01a\x01\xA3V[`\0a\x02U3a\x04[V[\x85Q` \x80\x88\x01Q\x87Q\x88\x83\x01Q\x88Q\x89\x85\x01Q\x87Q\x86\x89\x01Q`@Q\x99\x9AP`\0\x99\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x99a\x02\xAD\x99\x90\x98\x97\x96\x95\x94\x93\x92\x91\x01a\x0E\x05V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\0\x1Ca\x02\xD0\x91\x90a\x0EQV[\x90Pa\x037a\x02\xE9a\x02\xE2\x87\x84a\x059V[\x88\x90a\x05\xD0V[a\x02\xF1a\x06dV[a\x031a\x03*\x85a\x03$`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x01\x82R`\x02\x90\x82\x01R\x90V[\x90a\x059V[\x86\x90a\x05\xD0V[\x87a\x07$V[a\x03\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`t`$\x82\x01R\x7FBLSPublicKeyCompendium.registerB`D\x82\x01R\x7FLSPublicKey: either the G1 signa`d\x82\x01R\x7Fture is wrong, or G1 and G2 priv`\x84\x82\x01Rs\x0C.\x8C\xA4\rl\xAF$\x0C\x8D\xE4\r\xCD\xEE\x84\r\xAC.\x8Cm`c\x1B`\xA4\x82\x01R`\xC4\x01a\x01\xA3V[3`\0\x81\x81R` \x81\x81R`@\x80\x83 \x87\x90U\x86\x83R`\x01\x90\x91R\x90\x81\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x83\x17\x90UQ\x7F\xE3\xFBf\x13\xAF.\x890\xCF\x85\xD4\x7F\xCFm\xB1\x01\x92\"Jd\xC6\xCB\xE8\x02>\x0E\xEE\x1B\xA3\x82\x80A\x90a\x04K\x90\x88\x90\x88\x90a\x0EsV[`@Q\x80\x91\x03\x90\xA2PPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`@Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x84\x81\x1B\x82\x16` \x84\x01R0\x90\x1B\x16`4\x82\x01RF`H\x82\x01R\x7FEigenLayer_BN254_Pubkey_Registra`h\x82\x01Rc:4\xB7\xB7`\xE1\x1B`\x88\x82\x01Ra\x04\xF0\x90`\x8C\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\t\x91V[\x92\x91PPV[`\0\x81`\0\x01Q\x82` \x01Q`@Q` \x01a\x05\x1C\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x05Ua\x0BKV[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x80\x82\x01\x84\x90R`\0\x90\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a\x05\x88Wa\x05\x8AV[\xFE[P\x80a\x05\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCB[][\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x01\xA3V[PP\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x05\xECa\x0BiV[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R\x83Q`@\x80\x84\x01\x91\x90\x91R\x90\x84\x01Q``\x83\x01R`\0\x90\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a\x05\x88WP\x80a\x05\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCBXY\x19\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x01\xA3V[a\x06la\x0B\x87V[P`@\x80Q`\x80\x81\x01\x82R\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81\x83\x01\x90\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED``\x83\x01R\x81R\x81Q\x80\x83\x01\x90\x92R\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x81\x01\x91\x90\x91R\x81\x01\x91\x90\x91R\x90V[`@\x80Q\x80\x82\x01\x82R\x85\x81R` \x80\x82\x01\x85\x90R\x82Q\x80\x84\x01\x90\x93R\x85\x83R\x82\x01\x83\x90R`\0\x91a\x07Sa\x0B\xACV[`\0[`\x02\x81\x10\x15a\t\x18W`\0a\x07l\x82`\x06a\x0E\xD9V[\x90P\x84\x82`\x02\x81\x10a\x07\x80Wa\x07\x80a\x0E\xADV[` \x02\x01QQ\x83a\x07\x92\x83`\0a\x0E\xF8V[`\x0C\x81\x10a\x07\xA2Wa\x07\xA2a\x0E\xADV[` \x02\x01R\x84\x82`\x02\x81\x10a\x07\xB9Wa\x07\xB9a\x0E\xADV[` \x02\x01Q` \x01Q\x83\x82`\x01a\x07\xD0\x91\x90a\x0E\xF8V[`\x0C\x81\x10a\x07\xE0Wa\x07\xE0a\x0E\xADV[` \x02\x01R\x83\x82`\x02\x81\x10a\x07\xF7Wa\x07\xF7a\x0E\xADV[` \x02\x01QQQ\x83a\x08\n\x83`\x02a\x0E\xF8V[`\x0C\x81\x10a\x08\x1AWa\x08\x1Aa\x0E\xADV[` \x02\x01R\x83\x82`\x02\x81\x10a\x081Wa\x081a\x0E\xADV[` \x02\x01QQ`\x01` \x02\x01Q\x83a\x08J\x83`\x03a\x0E\xF8V[`\x0C\x81\x10a\x08ZWa\x08Za\x0E\xADV[` \x02\x01R\x83\x82`\x02\x81\x10a\x08qWa\x08qa\x0E\xADV[` \x02\x01Q` \x01Q`\0`\x02\x81\x10a\x08\x8CWa\x08\x8Ca\x0E\xADV[` \x02\x01Q\x83a\x08\x9D\x83`\x04a\x0E\xF8V[`\x0C\x81\x10a\x08\xADWa\x08\xADa\x0E\xADV[` \x02\x01R\x83\x82`\x02\x81\x10a\x08\xC4Wa\x08\xC4a\x0E\xADV[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a\x08\xDFWa\x08\xDFa\x0E\xADV[` \x02\x01Q\x83a\x08\xF0\x83`\x05a\x0E\xF8V[`\x0C\x81\x10a\t\0Wa\t\0a\x0E\xADV[` \x02\x01RP\x80a\t\x10\x81a\x0F\x10V[\x91PPa\x07VV[Pa\t!a\x0B\xCBV[`\0` \x82a\x01\x80\x85`\x08a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a\x05\x88WP\x80a\t\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x1C\x18Z\\\x9A[\x99\xCB[\xDC\x18\xDB\xD9\x19KY\x98Z[\x19Y`Z\x1B`D\x82\x01R`d\x01a\x01\xA3V[PQ\x15\x15\x98\x97PPPPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0\x80\x80a\t\xC1`\0\x80Q` a\x0F,\x839\x81Q\x91R\x86a\x0EQV[\x90P[a\t\xCD\x81a\n!V[\x90\x93P\x91P`\0\x80Q` a\x0F,\x839\x81Q\x91R\x82\x83\t\x83\x14\x15a\n\x07W`@\x80Q\x80\x82\x01\x90\x91R\x90\x81R` \x81\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80Q` a\x0F,\x839\x81Q\x91R`\x01\x82\x08\x90Pa\t\xC4V[`\0\x80\x80`\0\x80Q` a\x0F,\x839\x81Q\x91R`\x03`\0\x80Q` a\x0F,\x839\x81Q\x91R\x86`\0\x80Q` a\x0F,\x839\x81Q\x91R\x88\x89\t\t\x08\x90P`\0a\n\x97\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\0\x80Q` a\x0F,\x839\x81Q\x91Ra\n\xA3V[\x91\x95\x91\x94P\x90\x92PPPV[`\0\x80a\n\xAEa\x0B\xCBV[a\n\xB6a\x0B\xE9V[` \x80\x82R\x81\x81\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x88\x90R`\x80\x82\x01\x87\x90R`\xA0\x82\x01\x86\x90R\x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82\x80\x15a\x05\x88WP\x82a\x0B@W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FBN254.expMod: call failure\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01\xA3V[PQ\x95\x94PPPPPV[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80a\x0B\x9Aa\x0C\x07V[\x81R` \x01a\x0B\xA7a\x0C\x07V[\x90R\x90V[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0C^Wa\x0C^a\x0C%V[`@R\x90V[`\0`@\x82\x84\x03\x12\x15a\x0CvW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x0C\x99Wa\x0C\x99a\x0C%V[`@R\x825\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x0C\xC4W`\0\x80\xFD[a\x0C\xCCa\x0C;V[\x80`@\x84\x01\x85\x81\x11\x15a\x0C\xDEW`\0\x80\xFD[\x84[\x81\x81\x10\x15a\x0C\xF8W\x805\x84R` \x93\x84\x01\x93\x01a\x0C\xE0V[P\x90\x95\x94PPPPPV[`\0\x80`\0\x83\x85\x03a\x01\0\x81\x12\x15a\r\x1AW`\0\x80\xFD[a\r$\x86\x86a\x0CdV[\x93Pa\r3\x86`@\x87\x01a\x0CdV[\x92P`\x80`\x7F\x19\x82\x01\x12\x15a\rGW`\0\x80\xFD[Pa\rPa\x0C;V[a\r]\x86`\x80\x87\x01a\x0C\xB3V[\x81Ra\rl\x86`\xC0\x87\x01a\x0C\xB3V[` \x82\x01R\x80\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\r\x8EW`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\r\xA5W`\0\x80\xFD[\x93\x92PPPV[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x81\x01a\x04\xF0V[`\0` \x82\x84\x03\x12\x15a\r\xD5W`\0\x80\xFD[P5\x91\x90PV[\x80`\0[`\x02\x81\x10\x15a\r\xFFW\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01a\r\xE0V[PPPPV[\x88\x81R\x87` \x82\x01R\x86`@\x82\x01R\x85``\x82\x01Ra\x0E'`\x80\x82\x01\x86a\r\xDCV[a\x0E4`\xC0\x82\x01\x85a\r\xDCV[a\x01\0\x81\x01\x92\x90\x92Ra\x01 \x82\x01Ra\x01@\x01\x96\x95PPPPPPV[`\0\x82a\x0EnWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[\x82Q\x81R` \x80\x84\x01Q\x90\x82\x01R`\xC0\x81\x01a\x0E\x93`@\x83\x01\x84Qa\r\xDCV[` \x83\x01Qa\x0E\xA5`\x80\x84\x01\x82a\r\xDCV[P\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a\x0E\xF3Wa\x0E\xF3a\x0E\xC3V[P\x02\x90V[`\0\x82\x19\x82\x11\x15a\x0F\x0BWa\x0F\x0Ba\x0E\xC3V[P\x01\x90V[`\0`\0\x19\x82\x14\x15a\x0F$Wa\x0F$a\x0E\xC3V[P`\x01\x01\x90V\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 M\xB4\x82\xA81H\xBB`\xBEH[\xCC\xBE\xC9\xF8H\xC7.V\xC5SX\x959.\xD1\x94}0\xF2a\xB2dsolcC\0\x08\x0C\x003";
    /// The bytecode of the contract.
    pub static BLSPUBLICKEYCOMPENDIUM_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80c\x16\x1A3M\x14a\0QW\x80c\x1FZ\xC1\xB2\x14a\0fW\x80c\xDE)\xFA\xC0\x14a\0\x8FW\x80c\xE8\xBB\x9A\xE6\x14a\0\xBDW[`\0\x80\xFD[a\0da\0_6`\x04a\r\x03V[a\0\xFEV[\0[a\0ya\0t6`\x04a\r|V[a\x04[V[`@Qa\0\x86\x91\x90a\r\xACV[`@Q\x80\x91\x03\x90\xF3[a\0\xAFa\0\x9D6`\x04a\r|V[`\0` \x81\x90R\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01a\0\x86V[a\0\xE6a\0\xCB6`\x04a\r\xC3V[`\x01` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\x86V[`\0a\x01\t\x83a\x04\xF6V[3`\0\x90\x81R` \x81\x90R`@\x90 T\x90\x91P\x15a\x01\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`O`$\x82\x01R\x7FBLSPublicKeyCompendium.registerB`D\x82\x01R\x7FLSPublicKey: operator already re`d\x82\x01Rngistered pubkey`\x88\x1B`\x84\x82\x01R`\xA4\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x81\x81R`\x01` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x02JW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`J`$\x82\x01R\x7FBLSPublicKeyCompendium.registerB`D\x82\x01R\x7FLSPublicKey: public key already `d\x82\x01Ri\x1C\x99Y\xDA\\\xDD\x19\\\x99Y`\xB2\x1B`\x84\x82\x01R`\xA4\x01a\x01\xA3V[`\0a\x02U3a\x04[V[\x85Q` \x80\x88\x01Q\x87Q\x88\x83\x01Q\x88Q\x89\x85\x01Q\x87Q\x86\x89\x01Q`@Q\x99\x9AP`\0\x99\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x99a\x02\xAD\x99\x90\x98\x97\x96\x95\x94\x93\x92\x91\x01a\x0E\x05V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\0\x1Ca\x02\xD0\x91\x90a\x0EQV[\x90Pa\x037a\x02\xE9a\x02\xE2\x87\x84a\x059V[\x88\x90a\x05\xD0V[a\x02\xF1a\x06dV[a\x031a\x03*\x85a\x03$`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x01\x82R`\x02\x90\x82\x01R\x90V[\x90a\x059V[\x86\x90a\x05\xD0V[\x87a\x07$V[a\x03\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`t`$\x82\x01R\x7FBLSPublicKeyCompendium.registerB`D\x82\x01R\x7FLSPublicKey: either the G1 signa`d\x82\x01R\x7Fture is wrong, or G1 and G2 priv`\x84\x82\x01Rs\x0C.\x8C\xA4\rl\xAF$\x0C\x8D\xE4\r\xCD\xEE\x84\r\xAC.\x8Cm`c\x1B`\xA4\x82\x01R`\xC4\x01a\x01\xA3V[3`\0\x81\x81R` \x81\x81R`@\x80\x83 \x87\x90U\x86\x83R`\x01\x90\x91R\x90\x81\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x83\x17\x90UQ\x7F\xE3\xFBf\x13\xAF.\x890\xCF\x85\xD4\x7F\xCFm\xB1\x01\x92\"Jd\xC6\xCB\xE8\x02>\x0E\xEE\x1B\xA3\x82\x80A\x90a\x04K\x90\x88\x90\x88\x90a\x0EsV[`@Q\x80\x91\x03\x90\xA2PPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`@Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x84\x81\x1B\x82\x16` \x84\x01R0\x90\x1B\x16`4\x82\x01RF`H\x82\x01R\x7FEigenLayer_BN254_Pubkey_Registra`h\x82\x01Rc:4\xB7\xB7`\xE1\x1B`\x88\x82\x01Ra\x04\xF0\x90`\x8C\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\t\x91V[\x92\x91PPV[`\0\x81`\0\x01Q\x82` \x01Q`@Q` \x01a\x05\x1C\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x05Ua\x0BKV[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x80\x82\x01\x84\x90R`\0\x90\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a\x05\x88Wa\x05\x8AV[\xFE[P\x80a\x05\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCB[][\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x01\xA3V[PP\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x05\xECa\x0BiV[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R\x83Q`@\x80\x84\x01\x91\x90\x91R\x90\x84\x01Q``\x83\x01R`\0\x90\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a\x05\x88WP\x80a\x05\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCBXY\x19\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x01\xA3V[a\x06la\x0B\x87V[P`@\x80Q`\x80\x81\x01\x82R\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81\x83\x01\x90\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED``\x83\x01R\x81R\x81Q\x80\x83\x01\x90\x92R\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x81\x01\x91\x90\x91R\x81\x01\x91\x90\x91R\x90V[`@\x80Q\x80\x82\x01\x82R\x85\x81R` \x80\x82\x01\x85\x90R\x82Q\x80\x84\x01\x90\x93R\x85\x83R\x82\x01\x83\x90R`\0\x91a\x07Sa\x0B\xACV[`\0[`\x02\x81\x10\x15a\t\x18W`\0a\x07l\x82`\x06a\x0E\xD9V[\x90P\x84\x82`\x02\x81\x10a\x07\x80Wa\x07\x80a\x0E\xADV[` \x02\x01QQ\x83a\x07\x92\x83`\0a\x0E\xF8V[`\x0C\x81\x10a\x07\xA2Wa\x07\xA2a\x0E\xADV[` \x02\x01R\x84\x82`\x02\x81\x10a\x07\xB9Wa\x07\xB9a\x0E\xADV[` \x02\x01Q` \x01Q\x83\x82`\x01a\x07\xD0\x91\x90a\x0E\xF8V[`\x0C\x81\x10a\x07\xE0Wa\x07\xE0a\x0E\xADV[` \x02\x01R\x83\x82`\x02\x81\x10a\x07\xF7Wa\x07\xF7a\x0E\xADV[` \x02\x01QQQ\x83a\x08\n\x83`\x02a\x0E\xF8V[`\x0C\x81\x10a\x08\x1AWa\x08\x1Aa\x0E\xADV[` \x02\x01R\x83\x82`\x02\x81\x10a\x081Wa\x081a\x0E\xADV[` \x02\x01QQ`\x01` \x02\x01Q\x83a\x08J\x83`\x03a\x0E\xF8V[`\x0C\x81\x10a\x08ZWa\x08Za\x0E\xADV[` \x02\x01R\x83\x82`\x02\x81\x10a\x08qWa\x08qa\x0E\xADV[` \x02\x01Q` \x01Q`\0`\x02\x81\x10a\x08\x8CWa\x08\x8Ca\x0E\xADV[` \x02\x01Q\x83a\x08\x9D\x83`\x04a\x0E\xF8V[`\x0C\x81\x10a\x08\xADWa\x08\xADa\x0E\xADV[` \x02\x01R\x83\x82`\x02\x81\x10a\x08\xC4Wa\x08\xC4a\x0E\xADV[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a\x08\xDFWa\x08\xDFa\x0E\xADV[` \x02\x01Q\x83a\x08\xF0\x83`\x05a\x0E\xF8V[`\x0C\x81\x10a\t\0Wa\t\0a\x0E\xADV[` \x02\x01RP\x80a\t\x10\x81a\x0F\x10V[\x91PPa\x07VV[Pa\t!a\x0B\xCBV[`\0` \x82a\x01\x80\x85`\x08a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a\x05\x88WP\x80a\t\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x1C\x18Z\\\x9A[\x99\xCB[\xDC\x18\xDB\xD9\x19KY\x98Z[\x19Y`Z\x1B`D\x82\x01R`d\x01a\x01\xA3V[PQ\x15\x15\x98\x97PPPPPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0\x80\x80a\t\xC1`\0\x80Q` a\x0F,\x839\x81Q\x91R\x86a\x0EQV[\x90P[a\t\xCD\x81a\n!V[\x90\x93P\x91P`\0\x80Q` a\x0F,\x839\x81Q\x91R\x82\x83\t\x83\x14\x15a\n\x07W`@\x80Q\x80\x82\x01\x90\x91R\x90\x81R` \x81\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80Q` a\x0F,\x839\x81Q\x91R`\x01\x82\x08\x90Pa\t\xC4V[`\0\x80\x80`\0\x80Q` a\x0F,\x839\x81Q\x91R`\x03`\0\x80Q` a\x0F,\x839\x81Q\x91R\x86`\0\x80Q` a\x0F,\x839\x81Q\x91R\x88\x89\t\t\x08\x90P`\0a\n\x97\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\0\x80Q` a\x0F,\x839\x81Q\x91Ra\n\xA3V[\x91\x95\x91\x94P\x90\x92PPPV[`\0\x80a\n\xAEa\x0B\xCBV[a\n\xB6a\x0B\xE9V[` \x80\x82R\x81\x81\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x88\x90R`\x80\x82\x01\x87\x90R`\xA0\x82\x01\x86\x90R\x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82\x80\x15a\x05\x88WP\x82a\x0B@W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FBN254.expMod: call failure\0\0\0\0\0\0`D\x82\x01R`d\x01a\x01\xA3V[PQ\x95\x94PPPPPV[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80a\x0B\x9Aa\x0C\x07V[\x81R` \x01a\x0B\xA7a\x0C\x07V[\x90R\x90V[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0C^Wa\x0C^a\x0C%V[`@R\x90V[`\0`@\x82\x84\x03\x12\x15a\x0CvW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x0C\x99Wa\x0C\x99a\x0C%V[`@R\x825\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x0C\xC4W`\0\x80\xFD[a\x0C\xCCa\x0C;V[\x80`@\x84\x01\x85\x81\x11\x15a\x0C\xDEW`\0\x80\xFD[\x84[\x81\x81\x10\x15a\x0C\xF8W\x805\x84R` \x93\x84\x01\x93\x01a\x0C\xE0V[P\x90\x95\x94PPPPPV[`\0\x80`\0\x83\x85\x03a\x01\0\x81\x12\x15a\r\x1AW`\0\x80\xFD[a\r$\x86\x86a\x0CdV[\x93Pa\r3\x86`@\x87\x01a\x0CdV[\x92P`\x80`\x7F\x19\x82\x01\x12\x15a\rGW`\0\x80\xFD[Pa\rPa\x0C;V[a\r]\x86`\x80\x87\x01a\x0C\xB3V[\x81Ra\rl\x86`\xC0\x87\x01a\x0C\xB3V[` \x82\x01R\x80\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\r\x8EW`\0\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\r\xA5W`\0\x80\xFD[\x93\x92PPPV[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x81\x01a\x04\xF0V[`\0` \x82\x84\x03\x12\x15a\r\xD5W`\0\x80\xFD[P5\x91\x90PV[\x80`\0[`\x02\x81\x10\x15a\r\xFFW\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01a\r\xE0V[PPPPV[\x88\x81R\x87` \x82\x01R\x86`@\x82\x01R\x85``\x82\x01Ra\x0E'`\x80\x82\x01\x86a\r\xDCV[a\x0E4`\xC0\x82\x01\x85a\r\xDCV[a\x01\0\x81\x01\x92\x90\x92Ra\x01 \x82\x01Ra\x01@\x01\x96\x95PPPPPPV[`\0\x82a\x0EnWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[\x82Q\x81R` \x80\x84\x01Q\x90\x82\x01R`\xC0\x81\x01a\x0E\x93`@\x83\x01\x84Qa\r\xDCV[` \x83\x01Qa\x0E\xA5`\x80\x84\x01\x82a\r\xDCV[P\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a\x0E\xF3Wa\x0E\xF3a\x0E\xC3V[P\x02\x90V[`\0\x82\x19\x82\x11\x15a\x0F\x0BWa\x0F\x0Ba\x0E\xC3V[P\x01\x90V[`\0`\0\x19\x82\x14\x15a\x0F$Wa\x0F$a\x0E\xC3V[P`\x01\x01\x90V\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 M\xB4\x82\xA81H\xBB`\xBEH[\xCC\xBE\xC9\xF8H\xC7.V\xC5SX\x959.\xD1\x94}0\xF2a\xB2dsolcC\0\x08\x0C\x003";
    /// The deployed bytecode of the contract.
    pub static BLSPUBLICKEYCOMPENDIUM_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct BLSPublicKeyCompendium<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for BLSPublicKeyCompendium<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for BLSPublicKeyCompendium<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for BLSPublicKeyCompendium<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for BLSPublicKeyCompendium<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(BLSPublicKeyCompendium))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> BLSPublicKeyCompendium<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                BLSPUBLICKEYCOMPENDIUM_ABI.clone(),
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
                BLSPUBLICKEYCOMPENDIUM_ABI.clone(),
                BLSPUBLICKEYCOMPENDIUM_BYTECODE.clone().into(),
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
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
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
        ///Gets the contract's `NewPubkeyRegistration` event
        pub fn new_pubkey_registration_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, NewPubkeyRegistrationFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, NewPubkeyRegistrationFilter>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for BLSPublicKeyCompendium<M>
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
    pub enum BLSPublicKeyCompendiumCalls {
        GetMessageHash(GetMessageHashCall),
        OperatorToPubkeyHash(OperatorToPubkeyHashCall),
        PubkeyHashToOperator(PubkeyHashToOperatorCall),
        RegisterBLSPublicKey(RegisterBLSPublicKeyCall),
    }
    impl ::ethers::core::abi::AbiDecode for BLSPublicKeyCompendiumCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <GetMessageHashCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetMessageHash(decoded));
            }
            if let Ok(decoded) =
                <OperatorToPubkeyHashCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OperatorToPubkeyHash(decoded));
            }
            if let Ok(decoded) =
                <PubkeyHashToOperatorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PubkeyHashToOperator(decoded));
            }
            if let Ok(decoded) =
                <RegisterBLSPublicKeyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RegisterBLSPublicKey(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for BLSPublicKeyCompendiumCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetMessageHash(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OperatorToPubkeyHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PubkeyHashToOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterBLSPublicKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for BLSPublicKeyCompendiumCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetMessageHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorToPubkeyHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::PubkeyHashToOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterBLSPublicKey(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetMessageHashCall> for BLSPublicKeyCompendiumCalls {
        fn from(value: GetMessageHashCall) -> Self {
            Self::GetMessageHash(value)
        }
    }
    impl ::core::convert::From<OperatorToPubkeyHashCall> for BLSPublicKeyCompendiumCalls {
        fn from(value: OperatorToPubkeyHashCall) -> Self {
            Self::OperatorToPubkeyHash(value)
        }
    }
    impl ::core::convert::From<PubkeyHashToOperatorCall> for BLSPublicKeyCompendiumCalls {
        fn from(value: PubkeyHashToOperatorCall) -> Self {
            Self::PubkeyHashToOperator(value)
        }
    }
    impl ::core::convert::From<RegisterBLSPublicKeyCall> for BLSPublicKeyCompendiumCalls {
        fn from(value: RegisterBLSPublicKeyCall) -> Self {
            Self::RegisterBLSPublicKey(value)
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    pub struct PubkeyHashToOperatorReturn(pub ::ethers::core::types::Address);
}
