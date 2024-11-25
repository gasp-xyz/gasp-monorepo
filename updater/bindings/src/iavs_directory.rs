pub use iavs_directory::*;
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
pub mod iavs_directory {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("OPERATOR_AVS_REGISTRATION_TYPEHASH"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "OPERATOR_AVS_REGISTRATION_TYPEHASH",
                        ),
                        inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("OPERATOR_SET_REGISTRATION_TYPEHASH"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "OPERATOR_SET_REGISTRATION_TYPEHASH",
                        ),
                        inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("becomeOperatorSetAVS"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("becomeOperatorSetAVS",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("calculateOperatorAVSRegistrationDigestHash"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "calculateOperatorAVSRegistrationDigestHash",
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
                                name: ::std::borrow::ToOwned::to_owned("avs"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("salt"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("expiry"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
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
                    ::std::borrow::ToOwned::to_owned(
                        "calculateOperatorSetForceDeregistrationTypehash",
                    ),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "calculateOperatorSetForceDeregistrationTypehash",
                        ),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("avs"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operatorSetIds"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("salt"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("expiry"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
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
                    ::std::borrow::ToOwned::to_owned("calculateOperatorSetRegistrationDigestHash"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "calculateOperatorSetRegistrationDigestHash",
                        ),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("avs"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operatorSetIds"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("salt"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("expiry"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
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
                    ::std::borrow::ToOwned::to_owned("cancelSalt"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("cancelSalt"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("salt"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("createOperatorSets"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("createOperatorSets"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("operatorSetIds"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32[]"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deregisterOperatorFromAVS"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("deregisterOperatorFromAVS",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("operator"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deregisterOperatorFromOperatorSets"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "deregisterOperatorFromOperatorSets",
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
                                name: ::std::borrow::ToOwned::to_owned("operatorSetIds"),
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
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("domainSeparator"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("domainSeparator"),
                        inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("forceDeregisterFromOperatorSets"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("forceDeregisterFromOperatorSets",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("avs"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operatorSetIds"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operatorSignature"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
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
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isMember"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isMember"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operatorSet"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct IAVSDirectory.OperatorSet",
                                    ),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isOperatorSet"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isOperatorSet"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("avs"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operatorSetId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isOperatorSetAVS"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isOperatorSetAVS"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("avs"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("migrateOperatorsToOperatorSets"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("migrateOperatorsToOperatorSets",),
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
                                name: ::std::borrow::ToOwned::to_owned("operatorSetIds"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Array(
                                            ::std::boxed::Box::new(
                                                ::ethers::core::abi::ethabi::ParamType::Uint(
                                                    32usize
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32[][]"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("operatorSaltIsSpent"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("operatorSaltIsSpent",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("salt"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("registerOperatorToAVS"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("registerOperatorToAVS",),
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
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
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("registerOperatorToOperatorSets"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("registerOperatorToOperatorSets",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operatorSetIds"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operatorSignature"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
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
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateAVSMetadataURI"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updateAVSMetadataURI",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("metadataURI"),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AVSMetadataURIUpdated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("AVSMetadataURIUpdated",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("avs"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("metadataURI"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AVSMigratedToOperatorSets"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("AVSMigratedToOperatorSets",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("avs"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OperatorAVSRegistrationStatusUpdated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned(
                            "OperatorAVSRegistrationStatusUpdated",
                        ),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("avs"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("status"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OperatorAddedToOperatorSet"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("OperatorAddedToOperatorSet",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operatorSet"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ],),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OperatorMigratedToOperatorSets"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("OperatorMigratedToOperatorSets",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("avs"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operatorSetIds"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    ),
                                ),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OperatorRemovedFromOperatorSet"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("OperatorRemovedFromOperatorSet",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operatorSet"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ],),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OperatorSetCreated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("OperatorSetCreated"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("operatorSet"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            ],),
                            indexed: false,
                        },],
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
    pub static IAVSDIRECTORY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    pub struct IAVSDirectory<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IAVSDirectory<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IAVSDirectory<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IAVSDirectory<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IAVSDirectory<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IAVSDirectory))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IAVSDirectory<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                IAVSDIRECTORY_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `OPERATOR_AVS_REGISTRATION_TYPEHASH` (0xd79aceab) function
        pub fn operator_avs_registration_typehash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([215, 154, 206, 171], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `OPERATOR_SET_REGISTRATION_TYPEHASH` (0xc825fe68) function
        pub fn operator_set_registration_typehash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([200, 37, 254, 104], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `becomeOperatorSetAVS` (0xaec205c5) function
        pub fn become_operator_set_avs(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([174, 194, 5, 197], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateOperatorAVSRegistrationDigestHash` (0xa1060c88) function
        pub fn calculate_operator_avs_registration_digest_hash(
            &self,
            operator: ::ethers::core::types::Address,
            avs: ::ethers::core::types::Address,
            salt: [u8; 32],
            expiry: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([161, 6, 12, 136], (operator, avs, salt, expiry))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateOperatorSetForceDeregistrationTypehash` (0xb2841d48) function
        pub fn calculate_operator_set_force_deregistration_typehash(
            &self,
            avs: ::ethers::core::types::Address,
            operator_set_ids: ::std::vec::Vec<u32>,
            salt: [u8; 32],
            expiry: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([178, 132, 29, 72], (avs, operator_set_ids, salt, expiry))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateOperatorSetRegistrationDigestHash` (0x955e6696) function
        pub fn calculate_operator_set_registration_digest_hash(
            &self,
            avs: ::ethers::core::types::Address,
            operator_set_ids: ::std::vec::Vec<u32>,
            salt: [u8; 32],
            expiry: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([149, 94, 102, 150], (avs, operator_set_ids, salt, expiry))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `cancelSalt` (0xec76f442) function
        pub fn cancel_salt(
            &self,
            salt: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([236, 118, 244, 66], salt)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createOperatorSets` (0xafe02ed5) function
        pub fn create_operator_sets(
            &self,
            operator_set_ids: ::std::vec::Vec<u32>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([175, 224, 46, 213], operator_set_ids)
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
        ///Calls the contract's `deregisterOperatorFromOperatorSets` (0xc1a8e2c5) function
        pub fn deregister_operator_from_operator_sets(
            &self,
            operator: ::ethers::core::types::Address,
            operator_set_ids: ::std::vec::Vec<u32>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([193, 168, 226, 197], (operator, operator_set_ids))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `domainSeparator` (0xf698da25) function
        pub fn domain_separator(&self) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([246, 152, 218, 37], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `forceDeregisterFromOperatorSets` (0x3fee332d) function
        pub fn force_deregister_from_operator_sets(
            &self,
            operator: ::ethers::core::types::Address,
            avs: ::ethers::core::types::Address,
            operator_set_ids: ::std::vec::Vec<u32>,
            operator_signature: SignatureWithSaltAndExpiry,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [63, 238, 51, 45],
                    (operator, avs, operator_set_ids, operator_signature),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isMember` (0xda2ff05d) function
        pub fn is_member(
            &self,
            operator: ::ethers::core::types::Address,
            operator_set: OperatorSet,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([218, 47, 240, 93], (operator, operator_set))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isOperatorSet` (0x84d76f7b) function
        pub fn is_operator_set(
            &self,
            avs: ::ethers::core::types::Address,
            operator_set_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([132, 215, 111, 123], (avs, operator_set_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isOperatorSetAVS` (0x7673e93a) function
        pub fn is_operator_set_avs(
            &self,
            avs: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([118, 115, 233, 58], avs)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `migrateOperatorsToOperatorSets` (0xef2dfa8d) function
        pub fn migrate_operators_to_operator_sets(
            &self,
            operators: ::std::vec::Vec<::ethers::core::types::Address>,
            operator_set_ids: ::std::vec::Vec<::std::vec::Vec<u32>>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([239, 45, 250, 141], (operators, operator_set_ids))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operatorSaltIsSpent` (0x374823b5) function
        pub fn operator_salt_is_spent(
            &self,
            operator: ::ethers::core::types::Address,
            salt: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([55, 72, 35, 181], (operator, salt))
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
        ///Calls the contract's `registerOperatorToOperatorSets` (0x1e2199e2) function
        pub fn register_operator_to_operator_sets(
            &self,
            operator: ::ethers::core::types::Address,
            operator_set_ids: ::std::vec::Vec<u32>,
            operator_signature: SignatureWithSaltAndExpiry,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [30, 33, 153, 226],
                    (operator, operator_set_ids, operator_signature),
                )
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
        ///Gets the contract's `AVSMetadataURIUpdated` event
        pub fn avs_metadata_uri_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AvsmetadataURIUpdatedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `AVSMigratedToOperatorSets` event
        pub fn avs_migrated_to_operator_sets_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AvsmigratedToOperatorSetsFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OperatorAVSRegistrationStatusUpdated` event
        pub fn operator_avs_registration_status_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OperatorAVSRegistrationStatusUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OperatorAddedToOperatorSet` event
        pub fn operator_added_to_operator_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OperatorAddedToOperatorSetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OperatorMigratedToOperatorSets` event
        pub fn operator_migrated_to_operator_sets_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OperatorMigratedToOperatorSetsFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OperatorRemovedFromOperatorSet` event
        pub fn operator_removed_from_operator_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OperatorRemovedFromOperatorSetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OperatorSetCreated` event
        pub fn operator_set_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OperatorSetCreatedFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, IAVSDirectoryEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for IAVSDirectory<M>
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
        name = "AVSMetadataURIUpdated",
        abi = "AVSMetadataURIUpdated(address,string)"
    )]
    pub struct AvsmetadataURIUpdatedFilter {
        #[ethevent(indexed)]
        pub avs: ::ethers::core::types::Address,
        pub metadata_uri: ::std::string::String,
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
        name = "AVSMigratedToOperatorSets",
        abi = "AVSMigratedToOperatorSets(address)"
    )]
    pub struct AvsmigratedToOperatorSetsFilter {
        #[ethevent(indexed)]
        pub avs: ::ethers::core::types::Address,
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
        name = "OperatorAVSRegistrationStatusUpdated",
        abi = "OperatorAVSRegistrationStatusUpdated(address,address,uint8)"
    )]
    pub struct OperatorAVSRegistrationStatusUpdatedFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub avs: ::ethers::core::types::Address,
        pub status: u8,
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
        name = "OperatorAddedToOperatorSet",
        abi = "OperatorAddedToOperatorSet(address,(address,uint32))"
    )]
    pub struct OperatorAddedToOperatorSetFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        pub operator_set: OperatorSet,
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
        name = "OperatorMigratedToOperatorSets",
        abi = "OperatorMigratedToOperatorSets(address,address,uint32[])"
    )]
    pub struct OperatorMigratedToOperatorSetsFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub avs: ::ethers::core::types::Address,
        pub operator_set_ids: ::std::vec::Vec<u32>,
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
        name = "OperatorRemovedFromOperatorSet",
        abi = "OperatorRemovedFromOperatorSet(address,(address,uint32))"
    )]
    pub struct OperatorRemovedFromOperatorSetFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        pub operator_set: OperatorSet,
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
        name = "OperatorSetCreated",
        abi = "OperatorSetCreated((address,uint32))"
    )]
    pub struct OperatorSetCreatedFilter {
        pub operator_set: OperatorSet,
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
    pub enum IAVSDirectoryEvents {
        AvsmetadataURIUpdatedFilter(AvsmetadataURIUpdatedFilter),
        AvsmigratedToOperatorSetsFilter(AvsmigratedToOperatorSetsFilter),
        OperatorAVSRegistrationStatusUpdatedFilter(OperatorAVSRegistrationStatusUpdatedFilter),
        OperatorAddedToOperatorSetFilter(OperatorAddedToOperatorSetFilter),
        OperatorMigratedToOperatorSetsFilter(OperatorMigratedToOperatorSetsFilter),
        OperatorRemovedFromOperatorSetFilter(OperatorRemovedFromOperatorSetFilter),
        OperatorSetCreatedFilter(OperatorSetCreatedFilter),
    }
    impl ::ethers::contract::EthLogDecode for IAVSDirectoryEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AvsmetadataURIUpdatedFilter::decode_log(log) {
                return Ok(IAVSDirectoryEvents::AvsmetadataURIUpdatedFilter(decoded));
            }
            if let Ok(decoded) = AvsmigratedToOperatorSetsFilter::decode_log(log) {
                return Ok(IAVSDirectoryEvents::AvsmigratedToOperatorSetsFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = OperatorAVSRegistrationStatusUpdatedFilter::decode_log(log) {
                return Ok(
                    IAVSDirectoryEvents::OperatorAVSRegistrationStatusUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = OperatorAddedToOperatorSetFilter::decode_log(log) {
                return Ok(IAVSDirectoryEvents::OperatorAddedToOperatorSetFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = OperatorMigratedToOperatorSetsFilter::decode_log(log) {
                return Ok(IAVSDirectoryEvents::OperatorMigratedToOperatorSetsFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = OperatorRemovedFromOperatorSetFilter::decode_log(log) {
                return Ok(IAVSDirectoryEvents::OperatorRemovedFromOperatorSetFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = OperatorSetCreatedFilter::decode_log(log) {
                return Ok(IAVSDirectoryEvents::OperatorSetCreatedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IAVSDirectoryEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AvsmetadataURIUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::AvsmigratedToOperatorSetsFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorAVSRegistrationStatusUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorAddedToOperatorSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorMigratedToOperatorSetsFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorRemovedFromOperatorSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorSetCreatedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AvsmetadataURIUpdatedFilter> for IAVSDirectoryEvents {
        fn from(value: AvsmetadataURIUpdatedFilter) -> Self {
            Self::AvsmetadataURIUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<AvsmigratedToOperatorSetsFilter> for IAVSDirectoryEvents {
        fn from(value: AvsmigratedToOperatorSetsFilter) -> Self {
            Self::AvsmigratedToOperatorSetsFilter(value)
        }
    }
    impl ::core::convert::From<OperatorAVSRegistrationStatusUpdatedFilter> for IAVSDirectoryEvents {
        fn from(value: OperatorAVSRegistrationStatusUpdatedFilter) -> Self {
            Self::OperatorAVSRegistrationStatusUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<OperatorAddedToOperatorSetFilter> for IAVSDirectoryEvents {
        fn from(value: OperatorAddedToOperatorSetFilter) -> Self {
            Self::OperatorAddedToOperatorSetFilter(value)
        }
    }
    impl ::core::convert::From<OperatorMigratedToOperatorSetsFilter> for IAVSDirectoryEvents {
        fn from(value: OperatorMigratedToOperatorSetsFilter) -> Self {
            Self::OperatorMigratedToOperatorSetsFilter(value)
        }
    }
    impl ::core::convert::From<OperatorRemovedFromOperatorSetFilter> for IAVSDirectoryEvents {
        fn from(value: OperatorRemovedFromOperatorSetFilter) -> Self {
            Self::OperatorRemovedFromOperatorSetFilter(value)
        }
    }
    impl ::core::convert::From<OperatorSetCreatedFilter> for IAVSDirectoryEvents {
        fn from(value: OperatorSetCreatedFilter) -> Self {
            Self::OperatorSetCreatedFilter(value)
        }
    }
    ///Container type for all input parameters for the `OPERATOR_AVS_REGISTRATION_TYPEHASH` function with signature `OPERATOR_AVS_REGISTRATION_TYPEHASH()` and selector `0xd79aceab`
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
        name = "OPERATOR_AVS_REGISTRATION_TYPEHASH",
        abi = "OPERATOR_AVS_REGISTRATION_TYPEHASH()"
    )]
    pub struct OperatorAvsRegistrationTypehashCall;
    ///Container type for all input parameters for the `OPERATOR_SET_REGISTRATION_TYPEHASH` function with signature `OPERATOR_SET_REGISTRATION_TYPEHASH()` and selector `0xc825fe68`
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
        name = "OPERATOR_SET_REGISTRATION_TYPEHASH",
        abi = "OPERATOR_SET_REGISTRATION_TYPEHASH()"
    )]
    pub struct OperatorSetRegistrationTypehashCall;
    ///Container type for all input parameters for the `becomeOperatorSetAVS` function with signature `becomeOperatorSetAVS()` and selector `0xaec205c5`
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
    #[ethcall(name = "becomeOperatorSetAVS", abi = "becomeOperatorSetAVS()")]
    pub struct BecomeOperatorSetAVSCall;
    ///Container type for all input parameters for the `calculateOperatorAVSRegistrationDigestHash` function with signature `calculateOperatorAVSRegistrationDigestHash(address,address,bytes32,uint256)` and selector `0xa1060c88`
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
        name = "calculateOperatorAVSRegistrationDigestHash",
        abi = "calculateOperatorAVSRegistrationDigestHash(address,address,bytes32,uint256)"
    )]
    pub struct CalculateOperatorAVSRegistrationDigestHashCall {
        pub operator: ::ethers::core::types::Address,
        pub avs: ::ethers::core::types::Address,
        pub salt: [u8; 32],
        pub expiry: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `calculateOperatorSetForceDeregistrationTypehash` function with signature `calculateOperatorSetForceDeregistrationTypehash(address,uint32[],bytes32,uint256)` and selector `0xb2841d48`
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
        name = "calculateOperatorSetForceDeregistrationTypehash",
        abi = "calculateOperatorSetForceDeregistrationTypehash(address,uint32[],bytes32,uint256)"
    )]
    pub struct CalculateOperatorSetForceDeregistrationTypehashCall {
        pub avs: ::ethers::core::types::Address,
        pub operator_set_ids: ::std::vec::Vec<u32>,
        pub salt: [u8; 32],
        pub expiry: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `calculateOperatorSetRegistrationDigestHash` function with signature `calculateOperatorSetRegistrationDigestHash(address,uint32[],bytes32,uint256)` and selector `0x955e6696`
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
        name = "calculateOperatorSetRegistrationDigestHash",
        abi = "calculateOperatorSetRegistrationDigestHash(address,uint32[],bytes32,uint256)"
    )]
    pub struct CalculateOperatorSetRegistrationDigestHashCall {
        pub avs: ::ethers::core::types::Address,
        pub operator_set_ids: ::std::vec::Vec<u32>,
        pub salt: [u8; 32],
        pub expiry: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `cancelSalt` function with signature `cancelSalt(bytes32)` and selector `0xec76f442`
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
    #[ethcall(name = "cancelSalt", abi = "cancelSalt(bytes32)")]
    pub struct CancelSaltCall {
        pub salt: [u8; 32],
    }
    ///Container type for all input parameters for the `createOperatorSets` function with signature `createOperatorSets(uint32[])` and selector `0xafe02ed5`
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
    #[ethcall(name = "createOperatorSets", abi = "createOperatorSets(uint32[])")]
    pub struct CreateOperatorSetsCall {
        pub operator_set_ids: ::std::vec::Vec<u32>,
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
    ///Container type for all input parameters for the `deregisterOperatorFromOperatorSets` function with signature `deregisterOperatorFromOperatorSets(address,uint32[])` and selector `0xc1a8e2c5`
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
        name = "deregisterOperatorFromOperatorSets",
        abi = "deregisterOperatorFromOperatorSets(address,uint32[])"
    )]
    pub struct DeregisterOperatorFromOperatorSetsCall {
        pub operator: ::ethers::core::types::Address,
        pub operator_set_ids: ::std::vec::Vec<u32>,
    }
    ///Container type for all input parameters for the `domainSeparator` function with signature `domainSeparator()` and selector `0xf698da25`
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
    #[ethcall(name = "domainSeparator", abi = "domainSeparator()")]
    pub struct DomainSeparatorCall;
    ///Container type for all input parameters for the `forceDeregisterFromOperatorSets` function with signature `forceDeregisterFromOperatorSets(address,address,uint32[],(bytes,bytes32,uint256))` and selector `0x3fee332d`
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
        name = "forceDeregisterFromOperatorSets",
        abi = "forceDeregisterFromOperatorSets(address,address,uint32[],(bytes,bytes32,uint256))"
    )]
    pub struct ForceDeregisterFromOperatorSetsCall {
        pub operator: ::ethers::core::types::Address,
        pub avs: ::ethers::core::types::Address,
        pub operator_set_ids: ::std::vec::Vec<u32>,
        pub operator_signature: SignatureWithSaltAndExpiry,
    }
    ///Container type for all input parameters for the `isMember` function with signature `isMember(address,(address,uint32))` and selector `0xda2ff05d`
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
    #[ethcall(name = "isMember", abi = "isMember(address,(address,uint32))")]
    pub struct IsMemberCall {
        pub operator: ::ethers::core::types::Address,
        pub operator_set: OperatorSet,
    }
    ///Container type for all input parameters for the `isOperatorSet` function with signature `isOperatorSet(address,uint32)` and selector `0x84d76f7b`
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
    #[ethcall(name = "isOperatorSet", abi = "isOperatorSet(address,uint32)")]
    pub struct IsOperatorSetCall {
        pub avs: ::ethers::core::types::Address,
        pub operator_set_id: u32,
    }
    ///Container type for all input parameters for the `isOperatorSetAVS` function with signature `isOperatorSetAVS(address)` and selector `0x7673e93a`
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
    #[ethcall(name = "isOperatorSetAVS", abi = "isOperatorSetAVS(address)")]
    pub struct IsOperatorSetAVSCall {
        pub avs: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `migrateOperatorsToOperatorSets` function with signature `migrateOperatorsToOperatorSets(address[],uint32[][])` and selector `0xef2dfa8d`
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
        name = "migrateOperatorsToOperatorSets",
        abi = "migrateOperatorsToOperatorSets(address[],uint32[][])"
    )]
    pub struct MigrateOperatorsToOperatorSetsCall {
        pub operators: ::std::vec::Vec<::ethers::core::types::Address>,
        pub operator_set_ids: ::std::vec::Vec<::std::vec::Vec<u32>>,
    }
    ///Container type for all input parameters for the `operatorSaltIsSpent` function with signature `operatorSaltIsSpent(address,bytes32)` and selector `0x374823b5`
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
        name = "operatorSaltIsSpent",
        abi = "operatorSaltIsSpent(address,bytes32)"
    )]
    pub struct OperatorSaltIsSpentCall {
        pub operator: ::ethers::core::types::Address,
        pub salt: [u8; 32],
    }
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
    ///Container type for all input parameters for the `registerOperatorToOperatorSets` function with signature `registerOperatorToOperatorSets(address,uint32[],(bytes,bytes32,uint256))` and selector `0x1e2199e2`
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
        name = "registerOperatorToOperatorSets",
        abi = "registerOperatorToOperatorSets(address,uint32[],(bytes,bytes32,uint256))"
    )]
    pub struct RegisterOperatorToOperatorSetsCall {
        pub operator: ::ethers::core::types::Address,
        pub operator_set_ids: ::std::vec::Vec<u32>,
        pub operator_signature: SignatureWithSaltAndExpiry,
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
    pub enum IAVSDirectoryCalls {
        OperatorAvsRegistrationTypehash(OperatorAvsRegistrationTypehashCall),
        OperatorSetRegistrationTypehash(OperatorSetRegistrationTypehashCall),
        BecomeOperatorSetAVS(BecomeOperatorSetAVSCall),
        CalculateOperatorAVSRegistrationDigestHash(CalculateOperatorAVSRegistrationDigestHashCall),
        CalculateOperatorSetForceDeregistrationTypehash(
            CalculateOperatorSetForceDeregistrationTypehashCall,
        ),
        CalculateOperatorSetRegistrationDigestHash(CalculateOperatorSetRegistrationDigestHashCall),
        CancelSalt(CancelSaltCall),
        CreateOperatorSets(CreateOperatorSetsCall),
        DeregisterOperatorFromAVS(DeregisterOperatorFromAVSCall),
        DeregisterOperatorFromOperatorSets(DeregisterOperatorFromOperatorSetsCall),
        DomainSeparator(DomainSeparatorCall),
        ForceDeregisterFromOperatorSets(ForceDeregisterFromOperatorSetsCall),
        IsMember(IsMemberCall),
        IsOperatorSet(IsOperatorSetCall),
        IsOperatorSetAVS(IsOperatorSetAVSCall),
        MigrateOperatorsToOperatorSets(MigrateOperatorsToOperatorSetsCall),
        OperatorSaltIsSpent(OperatorSaltIsSpentCall),
        RegisterOperatorToAVS(RegisterOperatorToAVSCall),
        RegisterOperatorToOperatorSets(RegisterOperatorToOperatorSetsCall),
        UpdateAVSMetadataURI(UpdateAVSMetadataURICall),
    }
    impl ::ethers::core::abi::AbiDecode for IAVSDirectoryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <OperatorAvsRegistrationTypehashCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::OperatorAvsRegistrationTypehash(decoded));
            }
            if let Ok(decoded) =
                <OperatorSetRegistrationTypehashCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::OperatorSetRegistrationTypehash(decoded));
            }
            if let Ok(decoded) =
                <BecomeOperatorSetAVSCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BecomeOperatorSetAVS(decoded));
            }
            if let Ok(decoded) = <CalculateOperatorAVSRegistrationDigestHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateOperatorAVSRegistrationDigestHash(decoded));
            }
            if let Ok(decoded) = <CalculateOperatorSetForceDeregistrationTypehashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::CalculateOperatorSetForceDeregistrationTypehash(decoded),
                );
            }
            if let Ok(decoded) = <CalculateOperatorSetRegistrationDigestHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateOperatorSetRegistrationDigestHash(decoded));
            }
            if let Ok(decoded) = <CancelSaltCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CancelSalt(decoded));
            }
            if let Ok(decoded) =
                <CreateOperatorSetsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CreateOperatorSets(decoded));
            }
            if let Ok(decoded) =
                <DeregisterOperatorFromAVSCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DeregisterOperatorFromAVS(decoded));
            }
            if let Ok(decoded) =
                <DeregisterOperatorFromOperatorSetsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::DeregisterOperatorFromOperatorSets(decoded));
            }
            if let Ok(decoded) =
                <DomainSeparatorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DomainSeparator(decoded));
            }
            if let Ok(decoded) =
                <ForceDeregisterFromOperatorSetsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::ForceDeregisterFromOperatorSets(decoded));
            }
            if let Ok(decoded) = <IsMemberCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsMember(decoded));
            }
            if let Ok(decoded) = <IsOperatorSetCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IsOperatorSet(decoded));
            }
            if let Ok(decoded) =
                <IsOperatorSetAVSCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IsOperatorSetAVS(decoded));
            }
            if let Ok(decoded) =
                <MigrateOperatorsToOperatorSetsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MigrateOperatorsToOperatorSets(decoded));
            }
            if let Ok(decoded) =
                <OperatorSaltIsSpentCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OperatorSaltIsSpent(decoded));
            }
            if let Ok(decoded) =
                <RegisterOperatorToAVSCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RegisterOperatorToAVS(decoded));
            }
            if let Ok(decoded) =
                <RegisterOperatorToOperatorSetsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RegisterOperatorToOperatorSets(decoded));
            }
            if let Ok(decoded) =
                <UpdateAVSMetadataURICall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateAVSMetadataURI(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IAVSDirectoryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::OperatorAvsRegistrationTypehash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OperatorSetRegistrationTypehash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BecomeOperatorSetAVS(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateOperatorAVSRegistrationDigestHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateOperatorSetForceDeregistrationTypehash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateOperatorSetRegistrationDigestHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CancelSalt(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CreateOperatorSets(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeregisterOperatorFromAVS(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeregisterOperatorFromOperatorSets(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DomainSeparator(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ForceDeregisterFromOperatorSets(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsMember(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsOperatorSet(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsOperatorSetAVS(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MigrateOperatorsToOperatorSets(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OperatorSaltIsSpent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterOperatorToAVS(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterOperatorToOperatorSets(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateAVSMetadataURI(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IAVSDirectoryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::OperatorAvsRegistrationTypehash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorSetRegistrationTypehash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BecomeOperatorSetAVS(element) => ::core::fmt::Display::fmt(element, f),
                Self::CalculateOperatorAVSRegistrationDigestHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateOperatorSetForceDeregistrationTypehash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateOperatorSetRegistrationDigestHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CancelSalt(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateOperatorSets(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeregisterOperatorFromAVS(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeregisterOperatorFromOperatorSets(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DomainSeparator(element) => ::core::fmt::Display::fmt(element, f),
                Self::ForceDeregisterFromOperatorSets(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsMember(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsOperatorSet(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsOperatorSetAVS(element) => ::core::fmt::Display::fmt(element, f),
                Self::MigrateOperatorsToOperatorSets(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorSaltIsSpent(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterOperatorToAVS(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterOperatorToOperatorSets(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateAVSMetadataURI(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<OperatorAvsRegistrationTypehashCall> for IAVSDirectoryCalls {
        fn from(value: OperatorAvsRegistrationTypehashCall) -> Self {
            Self::OperatorAvsRegistrationTypehash(value)
        }
    }
    impl ::core::convert::From<OperatorSetRegistrationTypehashCall> for IAVSDirectoryCalls {
        fn from(value: OperatorSetRegistrationTypehashCall) -> Self {
            Self::OperatorSetRegistrationTypehash(value)
        }
    }
    impl ::core::convert::From<BecomeOperatorSetAVSCall> for IAVSDirectoryCalls {
        fn from(value: BecomeOperatorSetAVSCall) -> Self {
            Self::BecomeOperatorSetAVS(value)
        }
    }
    impl ::core::convert::From<CalculateOperatorAVSRegistrationDigestHashCall> for IAVSDirectoryCalls {
        fn from(value: CalculateOperatorAVSRegistrationDigestHashCall) -> Self {
            Self::CalculateOperatorAVSRegistrationDigestHash(value)
        }
    }
    impl ::core::convert::From<CalculateOperatorSetForceDeregistrationTypehashCall>
        for IAVSDirectoryCalls
    {
        fn from(value: CalculateOperatorSetForceDeregistrationTypehashCall) -> Self {
            Self::CalculateOperatorSetForceDeregistrationTypehash(value)
        }
    }
    impl ::core::convert::From<CalculateOperatorSetRegistrationDigestHashCall> for IAVSDirectoryCalls {
        fn from(value: CalculateOperatorSetRegistrationDigestHashCall) -> Self {
            Self::CalculateOperatorSetRegistrationDigestHash(value)
        }
    }
    impl ::core::convert::From<CancelSaltCall> for IAVSDirectoryCalls {
        fn from(value: CancelSaltCall) -> Self {
            Self::CancelSalt(value)
        }
    }
    impl ::core::convert::From<CreateOperatorSetsCall> for IAVSDirectoryCalls {
        fn from(value: CreateOperatorSetsCall) -> Self {
            Self::CreateOperatorSets(value)
        }
    }
    impl ::core::convert::From<DeregisterOperatorFromAVSCall> for IAVSDirectoryCalls {
        fn from(value: DeregisterOperatorFromAVSCall) -> Self {
            Self::DeregisterOperatorFromAVS(value)
        }
    }
    impl ::core::convert::From<DeregisterOperatorFromOperatorSetsCall> for IAVSDirectoryCalls {
        fn from(value: DeregisterOperatorFromOperatorSetsCall) -> Self {
            Self::DeregisterOperatorFromOperatorSets(value)
        }
    }
    impl ::core::convert::From<DomainSeparatorCall> for IAVSDirectoryCalls {
        fn from(value: DomainSeparatorCall) -> Self {
            Self::DomainSeparator(value)
        }
    }
    impl ::core::convert::From<ForceDeregisterFromOperatorSetsCall> for IAVSDirectoryCalls {
        fn from(value: ForceDeregisterFromOperatorSetsCall) -> Self {
            Self::ForceDeregisterFromOperatorSets(value)
        }
    }
    impl ::core::convert::From<IsMemberCall> for IAVSDirectoryCalls {
        fn from(value: IsMemberCall) -> Self {
            Self::IsMember(value)
        }
    }
    impl ::core::convert::From<IsOperatorSetCall> for IAVSDirectoryCalls {
        fn from(value: IsOperatorSetCall) -> Self {
            Self::IsOperatorSet(value)
        }
    }
    impl ::core::convert::From<IsOperatorSetAVSCall> for IAVSDirectoryCalls {
        fn from(value: IsOperatorSetAVSCall) -> Self {
            Self::IsOperatorSetAVS(value)
        }
    }
    impl ::core::convert::From<MigrateOperatorsToOperatorSetsCall> for IAVSDirectoryCalls {
        fn from(value: MigrateOperatorsToOperatorSetsCall) -> Self {
            Self::MigrateOperatorsToOperatorSets(value)
        }
    }
    impl ::core::convert::From<OperatorSaltIsSpentCall> for IAVSDirectoryCalls {
        fn from(value: OperatorSaltIsSpentCall) -> Self {
            Self::OperatorSaltIsSpent(value)
        }
    }
    impl ::core::convert::From<RegisterOperatorToAVSCall> for IAVSDirectoryCalls {
        fn from(value: RegisterOperatorToAVSCall) -> Self {
            Self::RegisterOperatorToAVS(value)
        }
    }
    impl ::core::convert::From<RegisterOperatorToOperatorSetsCall> for IAVSDirectoryCalls {
        fn from(value: RegisterOperatorToOperatorSetsCall) -> Self {
            Self::RegisterOperatorToOperatorSets(value)
        }
    }
    impl ::core::convert::From<UpdateAVSMetadataURICall> for IAVSDirectoryCalls {
        fn from(value: UpdateAVSMetadataURICall) -> Self {
            Self::UpdateAVSMetadataURI(value)
        }
    }
    ///Container type for all return fields from the `OPERATOR_AVS_REGISTRATION_TYPEHASH` function with signature `OPERATOR_AVS_REGISTRATION_TYPEHASH()` and selector `0xd79aceab`
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
    pub struct OperatorAvsRegistrationTypehashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `OPERATOR_SET_REGISTRATION_TYPEHASH` function with signature `OPERATOR_SET_REGISTRATION_TYPEHASH()` and selector `0xc825fe68`
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
    pub struct OperatorSetRegistrationTypehashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `calculateOperatorAVSRegistrationDigestHash` function with signature `calculateOperatorAVSRegistrationDigestHash(address,address,bytes32,uint256)` and selector `0xa1060c88`
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
    pub struct CalculateOperatorAVSRegistrationDigestHashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `calculateOperatorSetForceDeregistrationTypehash` function with signature `calculateOperatorSetForceDeregistrationTypehash(address,uint32[],bytes32,uint256)` and selector `0xb2841d48`
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
    pub struct CalculateOperatorSetForceDeregistrationTypehashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `calculateOperatorSetRegistrationDigestHash` function with signature `calculateOperatorSetRegistrationDigestHash(address,uint32[],bytes32,uint256)` and selector `0x955e6696`
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
    pub struct CalculateOperatorSetRegistrationDigestHashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `domainSeparator` function with signature `domainSeparator()` and selector `0xf698da25`
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
    pub struct DomainSeparatorReturn(pub [u8; 32]);
    ///Container type for all return fields from the `isMember` function with signature `isMember(address,(address,uint32))` and selector `0xda2ff05d`
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
    pub struct IsMemberReturn(pub bool);
    ///Container type for all return fields from the `isOperatorSet` function with signature `isOperatorSet(address,uint32)` and selector `0x84d76f7b`
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
    pub struct IsOperatorSetReturn(pub bool);
    ///Container type for all return fields from the `isOperatorSetAVS` function with signature `isOperatorSetAVS(address)` and selector `0x7673e93a`
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
    pub struct IsOperatorSetAVSReturn(pub bool);
    ///Container type for all return fields from the `operatorSaltIsSpent` function with signature `operatorSaltIsSpent(address,bytes32)` and selector `0x374823b5`
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
    pub struct OperatorSaltIsSpentReturn(pub bool);
}
