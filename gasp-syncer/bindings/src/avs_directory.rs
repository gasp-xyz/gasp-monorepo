pub use avs_directory::*;
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
pub mod avs_directory {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                    name: ::std::borrow::ToOwned::to_owned("_delegation"),
                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                    internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                        "contract IDelegationManager",
                    ),),
                },],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DOMAIN_TYPEHASH"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("DOMAIN_TYPEHASH"),
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
                    ::std::borrow::ToOwned::to_owned("OPERATOR_SET_FORCE_DEREGISTRATION_TYPEHASH"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "OPERATOR_SET_FORCE_DEREGISTRATION_TYPEHASH",
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
                    ::std::borrow::ToOwned::to_owned("avsOperatorStatus"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("avsOperatorStatus"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "enum IAVSDirectory.OperatorAVSRegistrationStatus",
                                ),
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
                    ::std::borrow::ToOwned::to_owned("delegation"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("delegation"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract IDelegationManager",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                    ::std::borrow::ToOwned::to_owned("inTotalOperatorSets"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("inTotalOperatorSets",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("operator"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
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
                                name: ::std::borrow::ToOwned::to_owned("_pauserRegistry"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("contract IPauserRegistry"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("initialPausedStatus",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
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
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
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
                            name: ::std::string::String::new(),
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
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("operatorSetMemberCount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("operatorSetMemberCount",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                        ],
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
                    ::std::borrow::ToOwned::to_owned("operatorSetsMemberOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("operatorSetsMemberOf",),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("start"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("length"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operatorSets"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ],),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct IAVSDirectory.OperatorSet[]",
                                    ),
                                ),
                            },],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("operatorSetsMemberOf",),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct IAVSDirectory.OperatorSet",
                                    ),
                                ),
                            },],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("owner"),
                        inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("pause"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pause"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("newPausedStatus"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pauseAll"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pauseAll"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("paused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("paused"),
                            inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("index"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
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
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("paused"),
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pauserRegistry"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pauserRegistry"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract IPauserRegistry"),
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
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setPauserRegistry"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setPauserRegistry"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("newPauserRegistry"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract IPauserRegistry"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("newOwner"),
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
                    ::std::borrow::ToOwned::to_owned("unpause"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("unpause"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("newPausedStatus"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("Initialized"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Initialized"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("version"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            indexed: false,
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
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("OwnershipTransferred",),
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
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Paused"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Paused"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newPausedStatus"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PauserRegistrySet"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
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
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Unpaused"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Unpaused"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newPausedStatus"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
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
    pub static AVSDIRECTORY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0=\x1D8\x03\x80b\0=\x1D\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x01\x18V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\x80Rb\0\0Kb\0\0VV[PF`\xA0Rb\0\x01JV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15b\0\0\xC3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\xFF\x90\x81\x16\x10\x15b\0\x01\x16W`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\0` \x82\x84\x03\x12\x15b\0\x01+W`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01CW`\0\x80\xFD[\x93\x92PPPV[`\x80Q`\xA0Qa;\x9Fb\0\x01~`\09`\0a\"S\x01R`\0\x81\x81a\x05\xCD\x01R\x81\x81a\x0B\x8E\x01Ra\x13\xFF\x01Ra;\x9F`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02=W`\x005`\xE0\x1C\x80c\x99&\xEE}\x11a\x01;W\x80c\xD7\x9A\xCE\xAB\x11a\0\xB8W\x80c\xECv\xF4B\x11a\0|W\x80c\xECv\xF4B\x14a\x05\xEFW\x80c\xEF-\xFA\x8D\x14a\x06#W\x80c\xF2\xFD\xE3\x8B\x14a\x066W\x80c\xF6\x98\xDA%\x14a\x06IW\x80c\xFA\xBC\x1C\xBC\x14a\x06QW`\0\x80\xFD[\x80c\xD7\x9A\xCE\xAB\x14a\x05<W\x80c\xDA/\xF0]\x14a\x05cW\x80c\xDA\xE2&\xB6\x14a\x05vW\x80c\xDC\xE9t\xB9\x14a\x05\xA1W\x80c\xDF\\\xF7#\x14a\x05\xC8W`\0\x80\xFD[\x80c\xAF\xE0.\xD5\x11a\0\xFFW\x80c\xAF\xE0.\xD5\x14a\x04\xC9W\x80c\xB2\x84\x1DH\x14a\x04\xDCW\x80c\xC1\xA8\xE2\xC5\x14a\x04\xEFW\x80c\xC8%\xFEh\x14a\x05\x02W\x80c\xCB\xDF\x0EB\x14a\x05)W`\0\x80\xFD[\x80c\x99&\xEE}\x14a\x04uW\x80c\xA1\x06\x0C\x88\x14a\x04\x88W\x80c\xA3d\xF4\xDA\x14a\x04\x9BW\x80c\xA9\x8F\xB3U\x14a\x04\xAEW\x80c\xAE\xC2\x05\xC5\x14a\x04\xC1W`\0\x80\xFD[\x80cY\\jg\x11a\x01\xC9W\x80c\x84\xD7o{\x11a\x01\x8DW\x80c\x84\xD7o{\x14a\x03\xD8W\x80c\x88o\x11\x95\x14a\x04\x06W\x80c\x8D\xA5\xCB[\x14a\x041W\x80c\x8D\xE5ID\x14a\x04BW\x80c\x95^f\x96\x14a\x04bW`\0\x80\xFD[\x80cY\\jg\x14a\x03zW\x80cZ\xC8j\xB7\x14a\x03\x82W\x80c\\\x97Z\xBB\x14a\x03\xA5W\x80cqP\x18\xA6\x14a\x03\xADW\x80cvs\xE9:\x14a\x03\xB5W`\0\x80\xFD[\x80c\x1E!\x99\xE2\x11a\x02\x10W\x80c\x1E!\x99\xE2\x14a\x02\xA6W\x80c `kp\x14a\x02\xB9W\x80c7H#\xB5\x14a\x02\xEEW\x80c?\xEE3-\x14a\x03,W\x80cI\x07]\xA3\x14a\x03?W`\0\x80\xFD[\x80c\rS\x87\xC5\x14a\x02BW\x80c\x10\xD6z/\x14a\x02kW\x80c\x13d9\xDD\x14a\x02\x80W\x80c\x17\x94\xBB<\x14a\x02\x93W[`\0\x80\xFD[a\x02Ua\x02P6`\x04a/\xECV[a\x06dV[`@Qa\x02b\x91\x90a0!V[`@Q\x80\x91\x03\x90\xF3[a\x02~a\x02y6`\x04a0\x87V[a\x07\xA9V[\0[a\x02~a\x02\x8E6`\x04a0\xABV[a\x08eV[a\x02~a\x02\xA16`\x04a0\xC4V[a\t\xA4V[a\x02~a\x02\xB46`\x04a2iV[a\n\xCEV[a\x02\xE0\x7F\x8C\xAD\x95h{\xA8,,\xE5\x0Et\xF7\xB7Td^Q\x17\xC3\xA5\xBE\xC8\x15\x1C\x07&\xD5\x85y\x80\xA8f\x81V[`@Q\x90\x81R` \x01a\x02bV[a\x03\x1Ca\x02\xFC6`\x04a2\xE4V[`\x99` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02bV[a\x02~a\x03:6`\x04a3\x10V[a\r\xEEV[a\x03ma\x03M6`\x04a3\x9EV[`\x98` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[`@Qa\x02b\x91\x90a3\xEDV[a\x02~a\x108V[a\x03\x1Ca\x03\x906`\x04a4\x15V[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`fTa\x02\xE0V[a\x02~a\x10\xFFV[a\x03\x1Ca\x03\xC36`\x04a0\x87V[`\x9A` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x03\x1Ca\x03\xE66`\x04a4QV[`\x9B` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[`eTa\x04\x19\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02bV[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x04\x19V[a\x04Ua\x04P6`\x04a2\xE4V[a\x11\x13V[`@Qa\x02b\x91\x90a4\x86V[a\x02\xE0a\x04p6`\x04a4\xACV[a\x11VV[a\x02~a\x04\x836`\x04a5\x12V[a\x11\xBBV[a\x02\xE0a\x04\x966`\x04a5aV[a\x15\x89V[a\x02~a\x04\xA96`\x04a0\x87V[a\x15\xF3V[a\x02~a\x04\xBC6`\x04a5\xA7V[a\x170V[a\x02~a\x17wV[a\x02~a\x04\xD76`\x04a6\x18V[a\x18?V[a\x02\xE0a\x04\xEA6`\x04a4\xACV[a\x1A\x0CV[a\x02~a\x04\xFD6`\x04a6YV[a\x1ALV[a\x02\xE0\x7F\x80\x9CZ\xC0I\xC4[z\x7F\x05\n \xF0\x0C\x16\xCFcy~\xFB\xF8\xB1\xEB\x8Dt\x9F\xDF\xA3\x9F\xF8\xF9)\x81V[a\x02\xE0a\x0576`\x04a0\x87V[a\x1A\x81V[a\x02\xE0\x7F\xDA,\x89\xBA\xFD\xD3Gv\xA2\xB8\xBB\x9C\x83\xC8/A\x9E \xCC\x8Cg \x7Fp\xED\xD5\x82I\xB9&a\xBD\x81V[a\x03\x1Ca\x05q6`\x04a6\xADV[a\x1A\xA2V[a\x02\xE0a\x05\x846`\x04a4QV[`\x9C` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[a\x02\xE0\x7FN\xE6_d!\x8Cg\xB6\x8D\xA6o\xD0\xDB\x16V\0@\xA6\xB9s)\x0B\x9Eq\x91-f\x1E\xE5?\xE4\x95\x81V[a\x04\x19\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02~a\x05\xFD6`\x04a0\xABV[3`\0\x90\x81R`\x99` \x90\x81R`@\x80\x83 \x93\x83R\x92\x90R \x80T`\xFF\x19\x16`\x01\x17\x90UV[a\x02~a\x0616`\x04a72V[a\x1A\xCEV[a\x02~a\x06D6`\x04a0\x87V[a\x1EwV[a\x02\xE0a\x1E\xEDV[a\x02~a\x06_6`\x04a0\xABV[a\x1E\xFCV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x9D` R`@\x81 ``\x91\x90\x84\x90a\x06\x8B\x90a XV[a\x06\x95\x91\x90a7\xB3V[\x90P\x80\x83\x11\x15a\x06\xA3W\x80\x92P[\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x06\xBBWa\x06\xBBa1IV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07\0W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x06\xD9W\x90P[P\x91P`\0[\x83\x81\x10\x15a\x07\xA0Wa\x07ra\x07<a\x07\x1E\x83\x88a7\xCAV[`\x01`\x01`\xA0\x1B\x03\x89\x16`\0\x90\x81R`\x9D` R`@\x90 \x90a bV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01RP`@\x80Q\x80\x82\x01\x90\x91R``\x82\x90\x1C\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16` \x82\x01R\x90V[\x83\x82\x81Q\x81\x10a\x07\x84Wa\x07\x84a7\xE2V[` \x02` \x01\x01\x81\x90RP\x80a\x07\x99\x90a7\xF8V[\x90Pa\x07\x06V[PP\x93\x92PPPV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xFCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08 \x91\x90a8\x13V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x08YW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08P\x90a80V[`@Q\x80\x91\x03\x90\xFD[a\x08b\x81a nV[PV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xADW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xD1\x91\x90a8zV[a\x08\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08P\x90a8\x9CV[`fT\x81\x81\x16\x14a\tfW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08PV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\t\xC4WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\t\xDEWP0;\x15\x80\x15a\t\xDEWP`\0T`\xFF\x16`\x01\x14[a\nAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x08PV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\ndW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\nn\x83\x83a!eV[a\nva\"OV[`\x97Ua\n\x82\x84a#\x18V[\x80\x15a\n\xC8W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPV[`fT`\x01\x90`\x02\x90\x81\x16\x14\x15a\n\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08P\x90a8\xE4V[B\x82`@\x01Q\x10\x15a\x0BoW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R`\0\x80Q` a;*\x839\x81Q\x91R`D\x82\x01R\x7FperatorSets: operator signature `d\x82\x01Rf\x19^\x1C\x1A\\\x99Y`\xCA\x1B`\x84\x82\x01R`\xA4\x01a\x08PV[`@Qc6\xB8{\xD7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cmp\xF7\xAE\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xD5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xF9\x91\x90a8zV[a\x0CxW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`V`$\x82\x01R`\0\x80Q` a;*\x839\x81Q\x91R`D\x82\x01R\x7FperatorSets: operator not regist`d\x82\x01Ru\x19\\\x99Y\x08\x1D\x1B\xC8\x11ZY\xD9[\x93\x18^Y\\\x88\x1EY]`R\x1B`\x84\x82\x01R`\xA4\x01a\x08PV[3`\0\x90\x81R`\x9A` R`@\x90 T`\xFF\x16a\x0C\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`K`$\x82\x01R`\0\x80Q` a;*\x839\x81Q\x91R`D\x82\x01R\x7FperatorSets: AVS is not an opera`d\x82\x01Rjtor set AVS`\xA8\x1B`\x84\x82\x01R`\xA4\x01a\x08PV[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x99` \x90\x81R`@\x80\x83 \x85\x83\x01Q\x84R\x90\x91R\x90 T`\xFF\x16\x15a\r\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`?`$\x82\x01R`\0\x80Q` a;*\x839\x81Q\x91R`D\x82\x01R\x7FperatorSets: salt already spent\0`d\x82\x01R`\x84\x01a\x08PV[a\r\xAA\x85a\r\xA33\x87\x87\x87` \x01Q\x88`@\x01Qa\x11VV[\x84Qa#jV[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x99` \x90\x81R`@\x80\x83 \x85\x83\x01Q\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\r\xE7\x853\x86\x86a%$V[PPPPPV[`fT`\x01\x90`\x02\x90\x81\x16\x14\x15a\x0E\x17W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08P\x90a8\xE4V[\x81QQa\x0E\xAFW3`\x01`\x01`\xA0\x1B\x03\x87\x16\x14a\x0E\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R\x7FAVSDirectory.forceDeregisterFrom`D\x82\x01R\x7FOperatorSets: caller must be ope`d\x82\x01Rd90\xBA7\xB9`\xD9\x1B`\x84\x82\x01R`\xA4\x01a\x08PV[a\x10$V[B\x82`@\x01Q\x10\x15a\x0F:W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`H`$\x82\x01R\x7FAVSDirectory.forceDeregisterFrom`D\x82\x01R\x7FOperatorSets: operator signature`d\x82\x01Rg\x08\x19^\x1C\x1A\\\x99Y`\xC2\x1B`\x84\x82\x01R`\xA4\x01a\x08PV[`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x99` \x90\x81R`@\x80\x83 \x85\x83\x01Q\x84R\x90\x91R\x90 T`\xFF\x16\x15a\x0F\xD9W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FAVSDirectory.forceDeregisterFrom`D\x82\x01R\x7FOperatorSets: salt already spent`d\x82\x01R`\x84\x01a\x08PV[a\x0F\xF2\x86a\r\xA3\x87\x87\x87\x87` \x01Q\x88`@\x01Qa\x1A\x0CV[`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x99` \x90\x81R`@\x80\x83 \x85\x83\x01Q\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U[a\x100\x85\x87\x86\x86a'\xDEV[PPPPPPV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\x80W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xA4\x91\x90a8zV[a\x10\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08P\x90a8\x9CV[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[a\x11\x07a)\xC0V[a\x11\x11`\0a#\x18V[V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x9D` R`@\x90 a\x11M\x90a\x07<\x90\x84a bV[\x90P[\x92\x91PPV[`\0a\x11\xB1\x7F\x80\x9CZ\xC0I\xC4[z\x7F\x05\n \xF0\x0C\x16\xCFcy~\xFB\xF8\xB1\xEB\x8Dt\x9F\xDF\xA3\x9F\xF8\xF9)\x87\x87\x87\x87\x87`@Q` \x01a\x11\x96\x96\x95\x94\x93\x92\x91\x90a9_V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a*\x1AV[\x96\x95PPPPPPV[`fT`\0\x90`\x01\x90\x81\x16\x14\x15a\x11\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08P\x90a8\xE4V[B\x82`@\x01Q\x10\x15a\x12LW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R`\0\x80Q` a;J\x839\x81Q\x91R`D\x82\x01R\x7FVS: operator signature expired\0\0`d\x82\x01R`\x84\x01a\x08PV[3`\0\x90\x81R`\x9A` R`@\x90 T`\xFF\x16\x15a\x12\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R`\0\x80Q` a;J\x839\x81Q\x91R`D\x82\x01R\x7FVS: AVS is an operator set AVS\0\0`d\x82\x01R`\x84\x01a\x08PV[`\x013`\0\x90\x81R`\x98` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x88\x16\x84R\x90\x91R\x90 T`\xFF\x16`\x01\x81\x11\x15a\x12\xFAWa\x12\xFAa3\xD7V[\x14\x15a\x13\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`?`$\x82\x01R`\0\x80Q` a;J\x839\x81Q\x91R`D\x82\x01R\x7FVS: operator already registered\0`d\x82\x01R`\x84\x01a\x08PV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x99` \x90\x81R`@\x80\x83 \x85\x83\x01Q\x84R\x90\x91R\x90 T`\xFF\x16\x15a\x13\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R`\0\x80Q` a;J\x839\x81Q\x91R`D\x82\x01Ru\x15\x94\xCE\x88\x1C\xD8[\x1D\x08\x18[\x1C\x99XY\x1EH\x1C\xDC\x19[\x9D`R\x1B`d\x82\x01R`\x84\x01a\x08PV[`@Qc6\xB8{\xD7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cmp\xF7\xAE\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14j\x91\x90a8zV[a\x14\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`M`$\x82\x01R`\0\x80Q` a;J\x839\x81Q\x91R`D\x82\x01R\x7FVS: operator not registered to E`d\x82\x01Rl\x1AY\xD9[\x93\x18^Y\\\x88\x1EY]`\x9A\x1B`\x84\x82\x01R`\xA4\x01a\x08PV[a\x14\xF8\x83a\r\xA3\x853\x86` \x01Q\x87`@\x01Qa\x15\x89V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81R`\x99` \x90\x81R`@\x80\x83 \x86\x83\x01Q\x84R\x82R\x80\x83 \x80T`\x01`\xFF\x19\x91\x82\x16\x81\x17\x90\x92U3\x80\x86R`\x98\x85R\x83\x86 \x87\x87R\x90\x94R\x93\x82\x90 \x80T\x90\x94\x16\x81\x17\x90\x93UQ\x90\x92\x91\x7F\xF0\x95+\x1Ce'\x1D\x81\x9D9\x98=*\xBB\x04K\x9C\xAC\xE5\x9B\xCCMM\xD3\x89\xF5\x86\xEB\xDC\xB1[A\x91a\x15|\x91\x90a3\xEDV[`@Q\x80\x91\x03\x90\xA3PPPV[`@\x80Q\x7F\xDA,\x89\xBA\xFD\xD3Gv\xA2\xB8\xBB\x9C\x83\xC8/A\x9E \xCC\x8Cg \x7Fp\xED\xD5\x82I\xB9&a\xBD` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x80\x87\x16\x92\x82\x01\x92\x90\x92R\x90\x84\x16``\x82\x01R`\x80\x81\x01\x83\x90R`\xA0\x81\x01\x82\x90R`\0\x90a\x15\xEA\x90`\xC0\x01a\x11\x96V[\x95\x94PPPPPV[`fT`\0\x90`\x01\x90\x81\x16\x14\x15a\x16\x1CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08P\x90a8\xE4V[`\x013`\0\x90\x81R`\x98` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x84R\x90\x91R\x90 T`\xFF\x16`\x01\x81\x11\x15a\x16VWa\x16Va3\xD7V[\x14a\x16\xC9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`?`$\x82\x01R\x7FAVSDirectory.deregisterOperatorF`D\x82\x01R\x7FromAVS: operator not registered\0`d\x82\x01R`\x84\x01a\x08PV[3`\0\x81\x81R`\x98` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ\x90\x91\x7F\xF0\x95+\x1Ce'\x1D\x81\x9D9\x98=*\xBB\x04K\x9C\xAC\xE5\x9B\xCCMM\xD3\x89\xF5\x86\xEB\xDC\xB1[A\x91a\x17$\x91\x90a3\xEDV[`@Q\x80\x91\x03\x90\xA3PPV[3`\x01`\x01`\xA0\x1B\x03\x16\x7F\xA8\x9C\x1D\xC2C\xD8\x90\x8A\x96\xDD\x84\x94K\xCC\x97\xD6\xBCj\xC0\r\xD7\x8E b\x15v\xBEj<\x947\x13\x83\x83`@Qa\x17k\x92\x91\x90a9\x9FV[`@Q\x80\x91\x03\x90\xA2PPV[3`\0\x90\x81R`\x9A` R`@\x90 T`\xFF\x16\x15a\x17\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FAVSDirectory.becomeOperatorSetAV`D\x82\x01R\x7FS: already an operator set AVS\0\0`d\x82\x01R`\x84\x01a\x08PV[3`\0\x81\x81R`\x9A` R`@\x80\x82 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x7Fp+\x0C\x1Fl\xB1\xCFQ\x1A\xAA\x81\xF7+\xC0Z![\xB3Iv2\xD7,i\x0C\x82+\x04J\xB4\x94\xBF\x91\x90\xA2V[`\0[\x81\x81\x10\x15a\x1A\x07W3`\0\x90\x81R`\x9B` R`@\x81 \x90\x84\x84\x84\x81\x81\x10a\x18lWa\x18la7\xE2V[\x90P` \x02\x01` \x81\x01\x90a\x18\x81\x91\x90a9\xCEV[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16\x15a\x19\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`;`$\x82\x01R\x7FAVSDirectory.createOperatorSet: `D\x82\x01R\x7Foperator set already exists\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08PV[3`\0\x90\x81R`\x9B` R`@\x81 `\x01\x91\x85\x85\x85\x81\x81\x10a\x192Wa\x192a7\xE2V[\x90P` \x02\x01` \x81\x01\x90a\x19G\x91\x90a9\xCEV[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F1b\x92\x85\xEA\xD23Z\xE0\x93?\x86\xED*\xE63!\xF7\xAFw\xB4\xE6\xEA\xAB\xC4,\x05x\x80\x97~l`@Q\x80`@\x01`@R\x803`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x85\x85\x85\x81\x81\x10a\x19\xC5Wa\x19\xC5a7\xE2V[\x90P` \x02\x01` \x81\x01\x90a\x19\xDA\x91\x90a9\xCEV[c\xFF\xFF\xFF\xFF\x16\x90R`@Qa\x19\xEF\x91\x90a4\x86V[`@Q\x80\x91\x03\x90\xA1a\x1A\0\x81a7\xF8V[\x90Pa\x18BV[PPPV[`\0a\x11\xB1\x7FN\xE6_d!\x8Cg\xB6\x8D\xA6o\xD0\xDB\x16V\0@\xA6\xB9s)\x0B\x9Eq\x91-f\x1E\xE5?\xE4\x95\x87\x87\x87\x87\x87`@Q` \x01a\x11\x96\x96\x95\x94\x93\x92\x91\x90a9_V[`fT`\x01\x90`\x02\x90\x81\x16\x14\x15a\x1AuW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08P\x90a8\xE4V[a\n\xC83\x85\x85\x85a'\xDEV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x9D` R`@\x81 a\x11P\x90a XV[`\0a\x11Ma\x1A\xB0\x83a*aV[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x9D` R`@\x90 \x90a*\xC6V[`fT`\x01\x90`\x02\x90\x81\x16\x14\x15a\x1A\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08P\x90a8\xE4V[3`\0\x90\x81R`\x9A` R`@\x90 T`\xFF\x16a\x1B\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`K`$\x82\x01R\x7FAVSDirectory.migrateOperatorsToO`D\x82\x01R\x7FperatorSets: AVS is not an opera`d\x82\x01Rjtor set AVS`\xA8\x1B`\x84\x82\x01R`\xA4\x01a\x08PV[`\0[\x84\x81\x10\x15a\x100W`\x013`\0\x90\x81R`\x98` R`@\x81 \x90\x88\x88\x85\x81\x81\x10a\x1B\xBFWa\x1B\xBFa7\xE2V[\x90P` \x02\x01` \x81\x01\x90a\x1B\xD4\x91\x90a0\x87V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16`\x01\x81\x11\x15a\x1C\x02Wa\x1C\x02a3\xD7V[\x14a\x1C\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`j`$\x82\x01R\x7FAVSDirectory.migrateOperatorsToO`D\x82\x01R\x7FperatorSets: operator already mi`d\x82\x01R\x7Fgrated or not a legacy registere`\x84\x82\x01Ri2\x107\xB82\xB90\xBA7\xB9`\xB1\x1B`\xA4\x82\x01R`\xC4\x01a\x08PV[a\x1D\x02\x86\x86\x83\x81\x81\x10a\x1C\xC3Wa\x1C\xC3a7\xE2V[\x90P` \x02\x01` \x81\x01\x90a\x1C\xD8\x91\x90a0\x87V[3\x86\x86\x85\x81\x81\x10a\x1C\xEBWa\x1C\xEBa7\xE2V[\x90P` \x02\x81\x01\x90a\x1C\xFD\x91\x90a9\xE9V[a%$V[3`\0\x90\x81R`\x98` R`@\x81 \x81\x88\x88\x85\x81\x81\x10a\x1D$Wa\x1D$a7\xE2V[\x90P` \x02\x01` \x81\x01\x90a\x1D9\x91\x90a0\x87V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\xFF\x19\x16`\x01\x83\x81\x81\x11\x15a\x1DkWa\x1Dka3\xD7V[\x02\x17\x90UP3\x86\x86\x83\x81\x81\x10a\x1D\x83Wa\x1D\x83a7\xE2V[\x90P` \x02\x01` \x81\x01\x90a\x1D\x98\x91\x90a0\x87V[`\x01`\x01`\xA0\x1B\x03\x16\x7F\xF0\x95+\x1Ce'\x1D\x81\x9D9\x98=*\xBB\x04K\x9C\xAC\xE5\x9B\xCCMM\xD3\x89\xF5\x86\xEB\xDC\xB1[A`\0`@Qa\x1D\xD1\x91\x90a3\xEDV[`@Q\x80\x91\x03\x90\xA33\x86\x86\x83\x81\x81\x10a\x1D\xECWa\x1D\xECa7\xE2V[\x90P` \x02\x01` \x81\x01\x90a\x1E\x01\x91\x90a0\x87V[`\x01`\x01`\xA0\x1B\x03\x16\x7FT\xF3<\xFD\xD1\xCAp=yY\x86\xB9\x86\xFDG\xD7B\xEA\xB1\x90N\xCD*_\xDB\x8De\x95\xE5\x90J\x01\x86\x86\x85\x81\x81\x10a\x1E=Wa\x1E=a7\xE2V[\x90P` \x02\x81\x01\x90a\x1EO\x91\x90a9\xE9V[`@Qa\x1E]\x92\x91\x90a:2V[`@Q\x80\x91\x03\x90\xA3\x80a\x1Eo\x81a7\xF8V[\x91PPa\x1B\x93V[a\x1E\x7Fa)\xC0V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1E\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x08PV[a\x08b\x81a#\x18V[`\0a\x1E\xF7a\"OV[\x90P\x90V[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1FOW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Fs\x91\x90a8\x13V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1F\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08P\x90a80V[`fT\x19\x81\x19`fT\x19\x16\x14a !W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08PV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\t\x99V[`\0a\x11P\x82T\x90V[`\0a\x11M\x83\x83a*\xDEV[`\x01`\x01`\xA0\x1B\x03\x81\x16a \xFCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x08PV[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a!\x86WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a\"\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x08PV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a\"K\x82a nV[PPV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14\x15a\"\x80WP`\x97T\x90V[P`@\x80Q\x80\x82\x01\x82R`\n\x81Ri\"\xB4\xB3\xB2\xB7&0\xBC\xB2\xB9`\xB1\x1B` \x91\x82\x01R\x81Q\x7F\x8C\xAD\x95h{\xA8,,\xE5\x0Et\xF7\xB7Td^Q\x17\xC3\xA5\xBE\xC8\x15\x1C\x07&\xD5\x85y\x80\xA8f\x81\x83\x01R\x7Fq\xB6%\xCF\xADD\xBA\xC6;\x13\xDB\xA0\x7F.\x1D`\x84\xEE\x04\xB6\xF8u!\x01\xEC\xE6\x12mXN\xE6\xEA\x81\x84\x01RF``\x82\x01R0`\x80\x80\x83\x01\x91\x90\x91R\x83Q\x80\x83\x03\x90\x91\x01\x81R`\xA0\x90\x91\x01\x90\x92R\x81Q\x91\x01 \x90V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16;\x15a$\x84W`@Qc\x0B\x13]?`\xE1\x1B\x80\x82R\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\x16&\xBA~\x90a#\xAA\x90\x86\x90\x86\x90`\x04\x01a:NV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a#\xC7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#\xEB\x91\x90a:\xABV[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14a\x1A\x07W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`S`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: ERC1271 signature `d\x82\x01Rr\x1D\x99\\\x9AY\x9AX\xD8]\x1A[\xDB\x88\x19\x98Z[\x19Y`j\x1B`\x84\x82\x01R`\xA4\x01a\x08PV[\x82`\x01`\x01`\xA0\x1B\x03\x16a$\x98\x83\x83a+\x08V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1A\x07W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: signature not from`d\x82\x01Rf\x109\xB4\xB3\xB72\xB9`\xC9\x1B`\x84\x82\x01R`\xA4\x01a\x08PV[`\0[\x81\x81\x10\x15a\r\xE7W`\0`@Q\x80`@\x01`@R\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x85\x85\x85\x81\x81\x10a%]Wa%]a7\xE2V[\x90P` \x02\x01` \x81\x01\x90a%r\x91\x90a9\xCEV[c\xFF\xFF\xFF\xFF\x16\x90R`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x9B` R`@\x81 \x91\x92P\x85\x85\x85\x81\x81\x10a%\xA7Wa%\xA7a7\xE2V[\x90P` \x02\x01` \x81\x01\x90a%\xBC\x91\x90a9\xCEV[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16a&SW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`B`$\x82\x01R\x7FAVSDirectory._registerOperatorTo`D\x82\x01R\x7FOperatorSets: invalid operator s`d\x82\x01Ra\x19]`\xF2\x1B`\x84\x82\x01R`\xA4\x01a\x08PV[a&]\x86\x82a\x1A\xA2V[\x15a&\xF6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`Y`$\x82\x01R\x7FAVSDirectory._registerOperatorTo`D\x82\x01R\x7FOperatorSets: operator already r`d\x82\x01R\x7Fegistered to operator set\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x08PV[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x9C` R`@\x81 \x90\x85\x85\x85\x81\x81\x10a'!Wa'!a7\xE2V[\x90P` \x02\x01` \x81\x01\x90a'6\x91\x90a9\xCEV[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x81Ta']\x90a7\xF8V[\x90\x91UPa'\x8Ba'm\x82a*aV[`\x01`\x01`\xA0\x1B\x03\x88\x16`\0\x90\x81R`\x9D` R`@\x90 \x90a+,V[P\x85`\x01`\x01`\xA0\x1B\x03\x16\x7FC#.\xDF\x90qu=#!\xE5\xFA~\x01\x83c\xEE$\x8E_!B\xE6\xC0\x8E\xDD2e\xBF\xB4\x89^\x82`@Qa'\xC5\x91\x90a4\x86V[`@Q\x80\x91\x03\x90\xA2Pa'\xD7\x81a7\xF8V[\x90Pa%'V[`\0[\x81\x81\x10\x15a\r\xE7W`\0`@Q\x80`@\x01`@R\x80\x87`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x85\x85\x85\x81\x81\x10a(\x17Wa(\x17a7\xE2V[\x90P` \x02\x01` \x81\x01\x90a(,\x91\x90a9\xCEV[c\xFF\xFF\xFF\xFF\x16\x90R\x90Pa(@\x85\x82a\x1A\xA2V[a(\xD8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`Y`$\x82\x01R\x7FAVSDirectory._deregisterOperator`D\x82\x01R\x7FFromOperatorSet: operator not re`d\x82\x01R\x7Fgistered for operator set\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x08PV[`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x9C` R`@\x81 \x90\x85\x85\x85\x81\x81\x10a)\x03Wa)\x03a7\xE2V[\x90P` \x02\x01` \x81\x01\x90a)\x18\x91\x90a9\xCEV[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x81Ta)?\x90a:\xD5V[\x90\x91UPa)ma)O\x82a*aV[`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81R`\x9D` R`@\x90 \x90a+8V[P\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\xAD4\xC3\x07\x0B\xE1\xDF\xFB\xCA\xA4\x99\xD0\0\xBA+\x8D\x98H\xAE\xFC\xAC0Y\xDF$]\xD9\\N\xCE\x14\xFE\x82`@Qa)\xA7\x91\x90a4\x86V[`@Q\x80\x91\x03\x90\xA2Pa)\xB9\x81a7\xF8V[\x90Pa'\xE1V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x11\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x08PV[`\0a*$a\"OV[`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x91\x90\x91R`B\x81\x01\x83\x90R`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0\x81`\0\x01Q\x82` \x01Qc\xFF\xFF\xFF\xFF\x16`@Q` \x01a*\xAE\x92\x91\x90``\x92\x90\x92\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82R`\xA0\x1B`\x01`\x01`\xA0\x1B\x03\x19\x16`\x14\x82\x01R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x11P\x90a:\xECV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x15\x15a\x11MV[`\0\x82`\0\x01\x82\x81T\x81\x10a*\xF5Wa*\xF5a7\xE2V[\x90`\0R` `\0 \x01T\x90P\x92\x91PPV[`\0\x80`\0a+\x17\x85\x85a+DV[\x91P\x91Pa+$\x81a+\xB4V[P\x93\x92PPPV[`\0a\x11M\x83\x83a-oV[`\0a\x11M\x83\x83a-\xBEV[`\0\x80\x82Q`A\x14\x15a+{W` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1Aa+o\x87\x82\x85\x85a.\xB1V[\x94P\x94PPPPa+\xADV[\x82Q`@\x14\x15a+\xA5W` \x83\x01Q`@\x84\x01Qa+\x9A\x86\x83\x83a/\x9EV[\x93P\x93PPPa+\xADV[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0\x81`\x04\x81\x11\x15a+\xC8Wa+\xC8a3\xD7V[\x14\x15a+\xD1WPV[`\x01\x81`\x04\x81\x11\x15a+\xE5Wa+\xE5a3\xD7V[\x14\x15a,3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x08PV[`\x02\x81`\x04\x81\x11\x15a,GWa,Ga3\xD7V[\x14\x15a,\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x08PV[`\x03\x81`\x04\x81\x11\x15a,\xA9Wa,\xA9a3\xD7V[\x14\x15a-\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x08PV[`\x04\x81`\x04\x81\x11\x15a-\x16Wa-\x16a3\xD7V[\x14\x15a\x08bW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 'v' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x08PV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 Ta-\xB6WP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x11PV[P`\0a\x11PV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a.\xA7W`\0a-\xE2`\x01\x83a7\xB3V[\x85T\x90\x91P`\0\x90a-\xF6\x90`\x01\x90a7\xB3V[\x90P\x81\x81\x14a.[W`\0\x86`\0\x01\x82\x81T\x81\x10a.\x16Wa.\x16a7\xE2V[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10a.9Wa.9a7\xE2V[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80a.lWa.la;\x13V[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x85`\x01\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x93PPPPa\x11PV[`\0\x91PPa\x11PV[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a.\xE8WP`\0\x90P`\x03a/\x95V[\x84`\xFF\x16`\x1B\x14\x15\x80\x15a/\0WP\x84`\xFF\x16`\x1C\x14\x15[\x15a/\x11WP`\0\x90P`\x04a/\x95V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a/eW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a/\x8EW`\0`\x01\x92P\x92PPa/\x95V[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x80`\x01`\x01`\xFF\x1B\x03\x83\x16\x81a/\xBB`\xFF\x86\x90\x1C`\x1Ba7\xCAV[\x90Pa/\xC9\x87\x82\x88\x85a.\xB1V[\x93P\x93PPP\x93P\x93\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x08bW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a0\x01W`\0\x80\xFD[\x835a0\x0C\x81a/\xD7V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a0zWa0j\x84\x83Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x01RV[\x92\x84\x01\x92\x90\x85\x01\x90`\x01\x01a0>V[P\x91\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a0\x99W`\0\x80\xFD[\x815a0\xA4\x81a/\xD7V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a0\xBDW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a0\xD9W`\0\x80\xFD[\x835a0\xE4\x81a/\xD7V[\x92P` \x84\x015a0\xF4\x81a/\xD7V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80\x83`\x1F\x84\x01\x12a1\x17W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a1.W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a+\xADW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a1\x81Wa1\x81a1IV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a1\xAFWa1\xAFa1IV[`@R\x91\x90PV[`\0``\x82\x84\x03\x12\x15a1\xC9W`\0\x80\xFD[a1\xD1a1_V[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a1\xEAW`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a1\xFEW`\0\x80\xFD[\x815` \x82\x82\x11\x15a2\x12Wa2\x12a1IV[a2$`\x1F\x83\x01`\x1F\x19\x16\x82\x01a1\x87V[\x92P\x81\x83R\x86\x81\x83\x86\x01\x01\x11\x15a2:W`\0\x80\xFD[\x81\x81\x85\x01\x82\x85\x017`\0\x81\x83\x85\x01\x01R\x82\x85R\x80\x86\x015\x81\x86\x01RPPPP`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a2\x7FW`\0\x80\xFD[\x845a2\x8A\x81a/\xD7V[\x93P` \x85\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a2\xA6W`\0\x80\xFD[a2\xB2\x88\x83\x89\x01a1\x05V[\x90\x95P\x93P`@\x87\x015\x91P\x80\x82\x11\x15a2\xCBW`\0\x80\xFD[Pa2\xD8\x87\x82\x88\x01a1\xB7V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a2\xF7W`\0\x80\xFD[\x825a3\x02\x81a/\xD7V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a3(W`\0\x80\xFD[\x855a33\x81a/\xD7V[\x94P` \x86\x015a3C\x81a/\xD7V[\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a3_W`\0\x80\xFD[a3k\x89\x83\x8A\x01a1\x05V[\x90\x95P\x93P``\x88\x015\x91P\x80\x82\x11\x15a3\x84W`\0\x80\xFD[Pa3\x91\x88\x82\x89\x01a1\xB7V[\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x80`@\x83\x85\x03\x12\x15a3\xB1W`\0\x80\xFD[\x825a3\xBC\x81a/\xD7V[\x91P` \x83\x015a3\xCC\x81a/\xD7V[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x02\x83\x10a4\x0FWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`\0` \x82\x84\x03\x12\x15a4'W`\0\x80\xFD[\x815`\xFF\x81\x16\x81\x14a0\xA4W`\0\x80\xFD[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a4LW`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a4dW`\0\x80\xFD[\x825a4o\x81a/\xD7V[\x91Pa4}` \x84\x01a48V[\x90P\x92P\x92\x90PV[\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x83\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R`@\x81\x01a\x11PV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a4\xC4W`\0\x80\xFD[\x855a4\xCF\x81a/\xD7V[\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a4\xEAW`\0\x80\xFD[a4\xF6\x88\x82\x89\x01a1\x05V[\x96\x99\x90\x98P\x95\x96`@\x81\x015\x96``\x90\x91\x015\x95P\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a5%W`\0\x80\xFD[\x825a50\x81a/\xD7V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a5KW`\0\x80\xFD[a5W\x85\x82\x86\x01a1\xB7V[\x91PP\x92P\x92\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a5wW`\0\x80\xFD[\x845a5\x82\x81a/\xD7V[\x93P` \x85\x015a5\x92\x81a/\xD7V[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[`\0\x80` \x83\x85\x03\x12\x15a5\xBAW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a5\xD1W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a5\xE5W`\0\x80\xFD[\x815\x81\x81\x11\x15a5\xF4W`\0\x80\xFD[\x86` \x82\x85\x01\x01\x11\x15a6\x06W`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0\x80` \x83\x85\x03\x12\x15a6+W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a6AW`\0\x80\xFD[a6M\x85\x82\x86\x01a1\x05V[\x90\x96\x90\x95P\x93PPPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a6nW`\0\x80\xFD[\x835a6y\x81a/\xD7V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a6\x94W`\0\x80\xFD[a6\xA0\x86\x82\x87\x01a1\x05V[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x80\x82\x84\x03``\x81\x12\x15a6\xC1W`\0\x80\xFD[\x835a6\xCC\x81a/\xD7V[\x92P`@`\x1F\x19\x82\x01\x12\x15a6\xE0W`\0\x80\xFD[P`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a7\x03Wa7\x03a1IV[`@R` \x84\x015a7\x14\x81a/\xD7V[\x81Ra7\"`@\x85\x01a48V[` \x82\x01R\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a7HW`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a7_W`\0\x80\xFD[a7k\x88\x83\x89\x01a1\x05V[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a7\x84W`\0\x80\xFD[Pa7\x91\x87\x82\x88\x01a1\x05V[\x95\x98\x94\x97P\x95PPPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a7\xC5Wa7\xC5a7\x9DV[P\x03\x90V[`\0\x82\x19\x82\x11\x15a7\xDDWa7\xDDa7\x9DV[P\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15a8\x0CWa8\x0Ca7\x9DV[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15a8%W`\0\x80\xFD[\x81Qa0\xA4\x81a/\xD7V[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15a8\x8CW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a0\xA4W`\0\x80\xFD[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`\x19\x90\x82\x01R\x7FPausable: index is paused\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a9TWc\xFF\xFF\xFF\xFFa9A\x83a48V[\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a9+V[P\x94\x95\x94PPPPPV[\x86\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16` \x82\x01R`\xA0`@\x82\x01\x81\x90R`\0\x90a9\x8A\x90\x83\x01\x86\x88a9\x1BV[``\x83\x01\x94\x90\x94RP`\x80\x01R\x94\x93PPPPV[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017`\0\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x91\x90PV[`\0` \x82\x84\x03\x12\x15a9\xE0W`\0\x80\xFD[a\x11M\x82a48V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a:\0W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a:\x1AW`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a+\xADW`\0\x80\xFD[` \x81R`\0a:F` \x83\x01\x84\x86a9\x1BV[\x94\x93PPPPV[\x82\x81R`\0` `@\x81\x84\x01R\x83Q\x80`@\x85\x01R`\0[\x81\x81\x10\x15a:\x82W\x85\x81\x01\x83\x01Q\x85\x82\x01``\x01R\x82\x01a:fV[\x81\x81\x11\x15a:\x94W`\0``\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01``\x01\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a:\xBDW`\0\x80\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a0\xA4W`\0\x80\xFD[`\0\x81a:\xE4Wa:\xE4a7\x9DV[P`\0\x19\x01\x90V[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a;\rW`\0\x19\x81` \x03`\x03\x1B\x1B\x82\x16\x91P[P\x91\x90PV[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD\xFEAVSDirectory.registerOperatorToOAVSDirectory.registerOperatorToA\xA2dipfsX\"\x12 \xD1\xD2\x8BF\xCB\xAB\xBF\xA0\xAD?*@\x81'\xFE\xAAf\xC3\x13\xD3\"\x1Fb5\xF4)8}\xF5\xB1;\xAAdsolcC\0\x08\x0C\x003";
    /// The bytecode of the contract.
    pub static AVSDIRECTORY_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02=W`\x005`\xE0\x1C\x80c\x99&\xEE}\x11a\x01;W\x80c\xD7\x9A\xCE\xAB\x11a\0\xB8W\x80c\xECv\xF4B\x11a\0|W\x80c\xECv\xF4B\x14a\x05\xEFW\x80c\xEF-\xFA\x8D\x14a\x06#W\x80c\xF2\xFD\xE3\x8B\x14a\x066W\x80c\xF6\x98\xDA%\x14a\x06IW\x80c\xFA\xBC\x1C\xBC\x14a\x06QW`\0\x80\xFD[\x80c\xD7\x9A\xCE\xAB\x14a\x05<W\x80c\xDA/\xF0]\x14a\x05cW\x80c\xDA\xE2&\xB6\x14a\x05vW\x80c\xDC\xE9t\xB9\x14a\x05\xA1W\x80c\xDF\\\xF7#\x14a\x05\xC8W`\0\x80\xFD[\x80c\xAF\xE0.\xD5\x11a\0\xFFW\x80c\xAF\xE0.\xD5\x14a\x04\xC9W\x80c\xB2\x84\x1DH\x14a\x04\xDCW\x80c\xC1\xA8\xE2\xC5\x14a\x04\xEFW\x80c\xC8%\xFEh\x14a\x05\x02W\x80c\xCB\xDF\x0EB\x14a\x05)W`\0\x80\xFD[\x80c\x99&\xEE}\x14a\x04uW\x80c\xA1\x06\x0C\x88\x14a\x04\x88W\x80c\xA3d\xF4\xDA\x14a\x04\x9BW\x80c\xA9\x8F\xB3U\x14a\x04\xAEW\x80c\xAE\xC2\x05\xC5\x14a\x04\xC1W`\0\x80\xFD[\x80cY\\jg\x11a\x01\xC9W\x80c\x84\xD7o{\x11a\x01\x8DW\x80c\x84\xD7o{\x14a\x03\xD8W\x80c\x88o\x11\x95\x14a\x04\x06W\x80c\x8D\xA5\xCB[\x14a\x041W\x80c\x8D\xE5ID\x14a\x04BW\x80c\x95^f\x96\x14a\x04bW`\0\x80\xFD[\x80cY\\jg\x14a\x03zW\x80cZ\xC8j\xB7\x14a\x03\x82W\x80c\\\x97Z\xBB\x14a\x03\xA5W\x80cqP\x18\xA6\x14a\x03\xADW\x80cvs\xE9:\x14a\x03\xB5W`\0\x80\xFD[\x80c\x1E!\x99\xE2\x11a\x02\x10W\x80c\x1E!\x99\xE2\x14a\x02\xA6W\x80c `kp\x14a\x02\xB9W\x80c7H#\xB5\x14a\x02\xEEW\x80c?\xEE3-\x14a\x03,W\x80cI\x07]\xA3\x14a\x03?W`\0\x80\xFD[\x80c\rS\x87\xC5\x14a\x02BW\x80c\x10\xD6z/\x14a\x02kW\x80c\x13d9\xDD\x14a\x02\x80W\x80c\x17\x94\xBB<\x14a\x02\x93W[`\0\x80\xFD[a\x02Ua\x02P6`\x04a/\xECV[a\x06dV[`@Qa\x02b\x91\x90a0!V[`@Q\x80\x91\x03\x90\xF3[a\x02~a\x02y6`\x04a0\x87V[a\x07\xA9V[\0[a\x02~a\x02\x8E6`\x04a0\xABV[a\x08eV[a\x02~a\x02\xA16`\x04a0\xC4V[a\t\xA4V[a\x02~a\x02\xB46`\x04a2iV[a\n\xCEV[a\x02\xE0\x7F\x8C\xAD\x95h{\xA8,,\xE5\x0Et\xF7\xB7Td^Q\x17\xC3\xA5\xBE\xC8\x15\x1C\x07&\xD5\x85y\x80\xA8f\x81V[`@Q\x90\x81R` \x01a\x02bV[a\x03\x1Ca\x02\xFC6`\x04a2\xE4V[`\x99` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02bV[a\x02~a\x03:6`\x04a3\x10V[a\r\xEEV[a\x03ma\x03M6`\x04a3\x9EV[`\x98` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[`@Qa\x02b\x91\x90a3\xEDV[a\x02~a\x108V[a\x03\x1Ca\x03\x906`\x04a4\x15V[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`fTa\x02\xE0V[a\x02~a\x10\xFFV[a\x03\x1Ca\x03\xC36`\x04a0\x87V[`\x9A` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x03\x1Ca\x03\xE66`\x04a4QV[`\x9B` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[`eTa\x04\x19\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02bV[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x04\x19V[a\x04Ua\x04P6`\x04a2\xE4V[a\x11\x13V[`@Qa\x02b\x91\x90a4\x86V[a\x02\xE0a\x04p6`\x04a4\xACV[a\x11VV[a\x02~a\x04\x836`\x04a5\x12V[a\x11\xBBV[a\x02\xE0a\x04\x966`\x04a5aV[a\x15\x89V[a\x02~a\x04\xA96`\x04a0\x87V[a\x15\xF3V[a\x02~a\x04\xBC6`\x04a5\xA7V[a\x170V[a\x02~a\x17wV[a\x02~a\x04\xD76`\x04a6\x18V[a\x18?V[a\x02\xE0a\x04\xEA6`\x04a4\xACV[a\x1A\x0CV[a\x02~a\x04\xFD6`\x04a6YV[a\x1ALV[a\x02\xE0\x7F\x80\x9CZ\xC0I\xC4[z\x7F\x05\n \xF0\x0C\x16\xCFcy~\xFB\xF8\xB1\xEB\x8Dt\x9F\xDF\xA3\x9F\xF8\xF9)\x81V[a\x02\xE0a\x0576`\x04a0\x87V[a\x1A\x81V[a\x02\xE0\x7F\xDA,\x89\xBA\xFD\xD3Gv\xA2\xB8\xBB\x9C\x83\xC8/A\x9E \xCC\x8Cg \x7Fp\xED\xD5\x82I\xB9&a\xBD\x81V[a\x03\x1Ca\x05q6`\x04a6\xADV[a\x1A\xA2V[a\x02\xE0a\x05\x846`\x04a4QV[`\x9C` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[a\x02\xE0\x7FN\xE6_d!\x8Cg\xB6\x8D\xA6o\xD0\xDB\x16V\0@\xA6\xB9s)\x0B\x9Eq\x91-f\x1E\xE5?\xE4\x95\x81V[a\x04\x19\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02~a\x05\xFD6`\x04a0\xABV[3`\0\x90\x81R`\x99` \x90\x81R`@\x80\x83 \x93\x83R\x92\x90R \x80T`\xFF\x19\x16`\x01\x17\x90UV[a\x02~a\x0616`\x04a72V[a\x1A\xCEV[a\x02~a\x06D6`\x04a0\x87V[a\x1EwV[a\x02\xE0a\x1E\xEDV[a\x02~a\x06_6`\x04a0\xABV[a\x1E\xFCV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x9D` R`@\x81 ``\x91\x90\x84\x90a\x06\x8B\x90a XV[a\x06\x95\x91\x90a7\xB3V[\x90P\x80\x83\x11\x15a\x06\xA3W\x80\x92P[\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x06\xBBWa\x06\xBBa1IV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07\0W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x06\xD9W\x90P[P\x91P`\0[\x83\x81\x10\x15a\x07\xA0Wa\x07ra\x07<a\x07\x1E\x83\x88a7\xCAV[`\x01`\x01`\xA0\x1B\x03\x89\x16`\0\x90\x81R`\x9D` R`@\x90 \x90a bV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01RP`@\x80Q\x80\x82\x01\x90\x91R``\x82\x90\x1C\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16` \x82\x01R\x90V[\x83\x82\x81Q\x81\x10a\x07\x84Wa\x07\x84a7\xE2V[` \x02` \x01\x01\x81\x90RP\x80a\x07\x99\x90a7\xF8V[\x90Pa\x07\x06V[PP\x93\x92PPPV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xFCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08 \x91\x90a8\x13V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x08YW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08P\x90a80V[`@Q\x80\x91\x03\x90\xFD[a\x08b\x81a nV[PV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xADW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xD1\x91\x90a8zV[a\x08\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08P\x90a8\x9CV[`fT\x81\x81\x16\x14a\tfW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08PV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\t\xC4WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\t\xDEWP0;\x15\x80\x15a\t\xDEWP`\0T`\xFF\x16`\x01\x14[a\nAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x08PV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\ndW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\nn\x83\x83a!eV[a\nva\"OV[`\x97Ua\n\x82\x84a#\x18V[\x80\x15a\n\xC8W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPV[`fT`\x01\x90`\x02\x90\x81\x16\x14\x15a\n\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08P\x90a8\xE4V[B\x82`@\x01Q\x10\x15a\x0BoW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R`\0\x80Q` a;*\x839\x81Q\x91R`D\x82\x01R\x7FperatorSets: operator signature `d\x82\x01Rf\x19^\x1C\x1A\\\x99Y`\xCA\x1B`\x84\x82\x01R`\xA4\x01a\x08PV[`@Qc6\xB8{\xD7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cmp\xF7\xAE\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xD5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xF9\x91\x90a8zV[a\x0CxW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`V`$\x82\x01R`\0\x80Q` a;*\x839\x81Q\x91R`D\x82\x01R\x7FperatorSets: operator not regist`d\x82\x01Ru\x19\\\x99Y\x08\x1D\x1B\xC8\x11ZY\xD9[\x93\x18^Y\\\x88\x1EY]`R\x1B`\x84\x82\x01R`\xA4\x01a\x08PV[3`\0\x90\x81R`\x9A` R`@\x90 T`\xFF\x16a\x0C\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`K`$\x82\x01R`\0\x80Q` a;*\x839\x81Q\x91R`D\x82\x01R\x7FperatorSets: AVS is not an opera`d\x82\x01Rjtor set AVS`\xA8\x1B`\x84\x82\x01R`\xA4\x01a\x08PV[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x99` \x90\x81R`@\x80\x83 \x85\x83\x01Q\x84R\x90\x91R\x90 T`\xFF\x16\x15a\r\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`?`$\x82\x01R`\0\x80Q` a;*\x839\x81Q\x91R`D\x82\x01R\x7FperatorSets: salt already spent\0`d\x82\x01R`\x84\x01a\x08PV[a\r\xAA\x85a\r\xA33\x87\x87\x87` \x01Q\x88`@\x01Qa\x11VV[\x84Qa#jV[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x99` \x90\x81R`@\x80\x83 \x85\x83\x01Q\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\r\xE7\x853\x86\x86a%$V[PPPPPV[`fT`\x01\x90`\x02\x90\x81\x16\x14\x15a\x0E\x17W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08P\x90a8\xE4V[\x81QQa\x0E\xAFW3`\x01`\x01`\xA0\x1B\x03\x87\x16\x14a\x0E\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R\x7FAVSDirectory.forceDeregisterFrom`D\x82\x01R\x7FOperatorSets: caller must be ope`d\x82\x01Rd90\xBA7\xB9`\xD9\x1B`\x84\x82\x01R`\xA4\x01a\x08PV[a\x10$V[B\x82`@\x01Q\x10\x15a\x0F:W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`H`$\x82\x01R\x7FAVSDirectory.forceDeregisterFrom`D\x82\x01R\x7FOperatorSets: operator signature`d\x82\x01Rg\x08\x19^\x1C\x1A\\\x99Y`\xC2\x1B`\x84\x82\x01R`\xA4\x01a\x08PV[`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x99` \x90\x81R`@\x80\x83 \x85\x83\x01Q\x84R\x90\x91R\x90 T`\xFF\x16\x15a\x0F\xD9W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FAVSDirectory.forceDeregisterFrom`D\x82\x01R\x7FOperatorSets: salt already spent`d\x82\x01R`\x84\x01a\x08PV[a\x0F\xF2\x86a\r\xA3\x87\x87\x87\x87` \x01Q\x88`@\x01Qa\x1A\x0CV[`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x99` \x90\x81R`@\x80\x83 \x85\x83\x01Q\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U[a\x100\x85\x87\x86\x86a'\xDEV[PPPPPPV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\x80W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xA4\x91\x90a8zV[a\x10\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08P\x90a8\x9CV[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[a\x11\x07a)\xC0V[a\x11\x11`\0a#\x18V[V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x9D` R`@\x90 a\x11M\x90a\x07<\x90\x84a bV[\x90P[\x92\x91PPV[`\0a\x11\xB1\x7F\x80\x9CZ\xC0I\xC4[z\x7F\x05\n \xF0\x0C\x16\xCFcy~\xFB\xF8\xB1\xEB\x8Dt\x9F\xDF\xA3\x9F\xF8\xF9)\x87\x87\x87\x87\x87`@Q` \x01a\x11\x96\x96\x95\x94\x93\x92\x91\x90a9_V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a*\x1AV[\x96\x95PPPPPPV[`fT`\0\x90`\x01\x90\x81\x16\x14\x15a\x11\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08P\x90a8\xE4V[B\x82`@\x01Q\x10\x15a\x12LW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R`\0\x80Q` a;J\x839\x81Q\x91R`D\x82\x01R\x7FVS: operator signature expired\0\0`d\x82\x01R`\x84\x01a\x08PV[3`\0\x90\x81R`\x9A` R`@\x90 T`\xFF\x16\x15a\x12\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R`\0\x80Q` a;J\x839\x81Q\x91R`D\x82\x01R\x7FVS: AVS is an operator set AVS\0\0`d\x82\x01R`\x84\x01a\x08PV[`\x013`\0\x90\x81R`\x98` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x88\x16\x84R\x90\x91R\x90 T`\xFF\x16`\x01\x81\x11\x15a\x12\xFAWa\x12\xFAa3\xD7V[\x14\x15a\x13\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`?`$\x82\x01R`\0\x80Q` a;J\x839\x81Q\x91R`D\x82\x01R\x7FVS: operator already registered\0`d\x82\x01R`\x84\x01a\x08PV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x99` \x90\x81R`@\x80\x83 \x85\x83\x01Q\x84R\x90\x91R\x90 T`\xFF\x16\x15a\x13\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R`\0\x80Q` a;J\x839\x81Q\x91R`D\x82\x01Ru\x15\x94\xCE\x88\x1C\xD8[\x1D\x08\x18[\x1C\x99XY\x1EH\x1C\xDC\x19[\x9D`R\x1B`d\x82\x01R`\x84\x01a\x08PV[`@Qc6\xB8{\xD7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cmp\xF7\xAE\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14j\x91\x90a8zV[a\x14\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`M`$\x82\x01R`\0\x80Q` a;J\x839\x81Q\x91R`D\x82\x01R\x7FVS: operator not registered to E`d\x82\x01Rl\x1AY\xD9[\x93\x18^Y\\\x88\x1EY]`\x9A\x1B`\x84\x82\x01R`\xA4\x01a\x08PV[a\x14\xF8\x83a\r\xA3\x853\x86` \x01Q\x87`@\x01Qa\x15\x89V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81R`\x99` \x90\x81R`@\x80\x83 \x86\x83\x01Q\x84R\x82R\x80\x83 \x80T`\x01`\xFF\x19\x91\x82\x16\x81\x17\x90\x92U3\x80\x86R`\x98\x85R\x83\x86 \x87\x87R\x90\x94R\x93\x82\x90 \x80T\x90\x94\x16\x81\x17\x90\x93UQ\x90\x92\x91\x7F\xF0\x95+\x1Ce'\x1D\x81\x9D9\x98=*\xBB\x04K\x9C\xAC\xE5\x9B\xCCMM\xD3\x89\xF5\x86\xEB\xDC\xB1[A\x91a\x15|\x91\x90a3\xEDV[`@Q\x80\x91\x03\x90\xA3PPPV[`@\x80Q\x7F\xDA,\x89\xBA\xFD\xD3Gv\xA2\xB8\xBB\x9C\x83\xC8/A\x9E \xCC\x8Cg \x7Fp\xED\xD5\x82I\xB9&a\xBD` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x80\x87\x16\x92\x82\x01\x92\x90\x92R\x90\x84\x16``\x82\x01R`\x80\x81\x01\x83\x90R`\xA0\x81\x01\x82\x90R`\0\x90a\x15\xEA\x90`\xC0\x01a\x11\x96V[\x95\x94PPPPPV[`fT`\0\x90`\x01\x90\x81\x16\x14\x15a\x16\x1CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08P\x90a8\xE4V[`\x013`\0\x90\x81R`\x98` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x84R\x90\x91R\x90 T`\xFF\x16`\x01\x81\x11\x15a\x16VWa\x16Va3\xD7V[\x14a\x16\xC9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`?`$\x82\x01R\x7FAVSDirectory.deregisterOperatorF`D\x82\x01R\x7FromAVS: operator not registered\0`d\x82\x01R`\x84\x01a\x08PV[3`\0\x81\x81R`\x98` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ\x90\x91\x7F\xF0\x95+\x1Ce'\x1D\x81\x9D9\x98=*\xBB\x04K\x9C\xAC\xE5\x9B\xCCMM\xD3\x89\xF5\x86\xEB\xDC\xB1[A\x91a\x17$\x91\x90a3\xEDV[`@Q\x80\x91\x03\x90\xA3PPV[3`\x01`\x01`\xA0\x1B\x03\x16\x7F\xA8\x9C\x1D\xC2C\xD8\x90\x8A\x96\xDD\x84\x94K\xCC\x97\xD6\xBCj\xC0\r\xD7\x8E b\x15v\xBEj<\x947\x13\x83\x83`@Qa\x17k\x92\x91\x90a9\x9FV[`@Q\x80\x91\x03\x90\xA2PPV[3`\0\x90\x81R`\x9A` R`@\x90 T`\xFF\x16\x15a\x17\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FAVSDirectory.becomeOperatorSetAV`D\x82\x01R\x7FS: already an operator set AVS\0\0`d\x82\x01R`\x84\x01a\x08PV[3`\0\x81\x81R`\x9A` R`@\x80\x82 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x7Fp+\x0C\x1Fl\xB1\xCFQ\x1A\xAA\x81\xF7+\xC0Z![\xB3Iv2\xD7,i\x0C\x82+\x04J\xB4\x94\xBF\x91\x90\xA2V[`\0[\x81\x81\x10\x15a\x1A\x07W3`\0\x90\x81R`\x9B` R`@\x81 \x90\x84\x84\x84\x81\x81\x10a\x18lWa\x18la7\xE2V[\x90P` \x02\x01` \x81\x01\x90a\x18\x81\x91\x90a9\xCEV[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16\x15a\x19\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`;`$\x82\x01R\x7FAVSDirectory.createOperatorSet: `D\x82\x01R\x7Foperator set already exists\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08PV[3`\0\x90\x81R`\x9B` R`@\x81 `\x01\x91\x85\x85\x85\x81\x81\x10a\x192Wa\x192a7\xE2V[\x90P` \x02\x01` \x81\x01\x90a\x19G\x91\x90a9\xCEV[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F1b\x92\x85\xEA\xD23Z\xE0\x93?\x86\xED*\xE63!\xF7\xAFw\xB4\xE6\xEA\xAB\xC4,\x05x\x80\x97~l`@Q\x80`@\x01`@R\x803`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x85\x85\x85\x81\x81\x10a\x19\xC5Wa\x19\xC5a7\xE2V[\x90P` \x02\x01` \x81\x01\x90a\x19\xDA\x91\x90a9\xCEV[c\xFF\xFF\xFF\xFF\x16\x90R`@Qa\x19\xEF\x91\x90a4\x86V[`@Q\x80\x91\x03\x90\xA1a\x1A\0\x81a7\xF8V[\x90Pa\x18BV[PPPV[`\0a\x11\xB1\x7FN\xE6_d!\x8Cg\xB6\x8D\xA6o\xD0\xDB\x16V\0@\xA6\xB9s)\x0B\x9Eq\x91-f\x1E\xE5?\xE4\x95\x87\x87\x87\x87\x87`@Q` \x01a\x11\x96\x96\x95\x94\x93\x92\x91\x90a9_V[`fT`\x01\x90`\x02\x90\x81\x16\x14\x15a\x1AuW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08P\x90a8\xE4V[a\n\xC83\x85\x85\x85a'\xDEV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x9D` R`@\x81 a\x11P\x90a XV[`\0a\x11Ma\x1A\xB0\x83a*aV[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x9D` R`@\x90 \x90a*\xC6V[`fT`\x01\x90`\x02\x90\x81\x16\x14\x15a\x1A\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08P\x90a8\xE4V[3`\0\x90\x81R`\x9A` R`@\x90 T`\xFF\x16a\x1B\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`K`$\x82\x01R\x7FAVSDirectory.migrateOperatorsToO`D\x82\x01R\x7FperatorSets: AVS is not an opera`d\x82\x01Rjtor set AVS`\xA8\x1B`\x84\x82\x01R`\xA4\x01a\x08PV[`\0[\x84\x81\x10\x15a\x100W`\x013`\0\x90\x81R`\x98` R`@\x81 \x90\x88\x88\x85\x81\x81\x10a\x1B\xBFWa\x1B\xBFa7\xE2V[\x90P` \x02\x01` \x81\x01\x90a\x1B\xD4\x91\x90a0\x87V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16`\x01\x81\x11\x15a\x1C\x02Wa\x1C\x02a3\xD7V[\x14a\x1C\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`j`$\x82\x01R\x7FAVSDirectory.migrateOperatorsToO`D\x82\x01R\x7FperatorSets: operator already mi`d\x82\x01R\x7Fgrated or not a legacy registere`\x84\x82\x01Ri2\x107\xB82\xB90\xBA7\xB9`\xB1\x1B`\xA4\x82\x01R`\xC4\x01a\x08PV[a\x1D\x02\x86\x86\x83\x81\x81\x10a\x1C\xC3Wa\x1C\xC3a7\xE2V[\x90P` \x02\x01` \x81\x01\x90a\x1C\xD8\x91\x90a0\x87V[3\x86\x86\x85\x81\x81\x10a\x1C\xEBWa\x1C\xEBa7\xE2V[\x90P` \x02\x81\x01\x90a\x1C\xFD\x91\x90a9\xE9V[a%$V[3`\0\x90\x81R`\x98` R`@\x81 \x81\x88\x88\x85\x81\x81\x10a\x1D$Wa\x1D$a7\xE2V[\x90P` \x02\x01` \x81\x01\x90a\x1D9\x91\x90a0\x87V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\xFF\x19\x16`\x01\x83\x81\x81\x11\x15a\x1DkWa\x1Dka3\xD7V[\x02\x17\x90UP3\x86\x86\x83\x81\x81\x10a\x1D\x83Wa\x1D\x83a7\xE2V[\x90P` \x02\x01` \x81\x01\x90a\x1D\x98\x91\x90a0\x87V[`\x01`\x01`\xA0\x1B\x03\x16\x7F\xF0\x95+\x1Ce'\x1D\x81\x9D9\x98=*\xBB\x04K\x9C\xAC\xE5\x9B\xCCMM\xD3\x89\xF5\x86\xEB\xDC\xB1[A`\0`@Qa\x1D\xD1\x91\x90a3\xEDV[`@Q\x80\x91\x03\x90\xA33\x86\x86\x83\x81\x81\x10a\x1D\xECWa\x1D\xECa7\xE2V[\x90P` \x02\x01` \x81\x01\x90a\x1E\x01\x91\x90a0\x87V[`\x01`\x01`\xA0\x1B\x03\x16\x7FT\xF3<\xFD\xD1\xCAp=yY\x86\xB9\x86\xFDG\xD7B\xEA\xB1\x90N\xCD*_\xDB\x8De\x95\xE5\x90J\x01\x86\x86\x85\x81\x81\x10a\x1E=Wa\x1E=a7\xE2V[\x90P` \x02\x81\x01\x90a\x1EO\x91\x90a9\xE9V[`@Qa\x1E]\x92\x91\x90a:2V[`@Q\x80\x91\x03\x90\xA3\x80a\x1Eo\x81a7\xF8V[\x91PPa\x1B\x93V[a\x1E\x7Fa)\xC0V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1E\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x08PV[a\x08b\x81a#\x18V[`\0a\x1E\xF7a\"OV[\x90P\x90V[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1FOW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Fs\x91\x90a8\x13V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1F\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08P\x90a80V[`fT\x19\x81\x19`fT\x19\x16\x14a !W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x08PV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\t\x99V[`\0a\x11P\x82T\x90V[`\0a\x11M\x83\x83a*\xDEV[`\x01`\x01`\xA0\x1B\x03\x81\x16a \xFCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x08PV[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a!\x86WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a\"\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x08PV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a\"K\x82a nV[PPV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14\x15a\"\x80WP`\x97T\x90V[P`@\x80Q\x80\x82\x01\x82R`\n\x81Ri\"\xB4\xB3\xB2\xB7&0\xBC\xB2\xB9`\xB1\x1B` \x91\x82\x01R\x81Q\x7F\x8C\xAD\x95h{\xA8,,\xE5\x0Et\xF7\xB7Td^Q\x17\xC3\xA5\xBE\xC8\x15\x1C\x07&\xD5\x85y\x80\xA8f\x81\x83\x01R\x7Fq\xB6%\xCF\xADD\xBA\xC6;\x13\xDB\xA0\x7F.\x1D`\x84\xEE\x04\xB6\xF8u!\x01\xEC\xE6\x12mXN\xE6\xEA\x81\x84\x01RF``\x82\x01R0`\x80\x80\x83\x01\x91\x90\x91R\x83Q\x80\x83\x03\x90\x91\x01\x81R`\xA0\x90\x91\x01\x90\x92R\x81Q\x91\x01 \x90V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16;\x15a$\x84W`@Qc\x0B\x13]?`\xE1\x1B\x80\x82R\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\x16&\xBA~\x90a#\xAA\x90\x86\x90\x86\x90`\x04\x01a:NV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a#\xC7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#\xEB\x91\x90a:\xABV[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14a\x1A\x07W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`S`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: ERC1271 signature `d\x82\x01Rr\x1D\x99\\\x9AY\x9AX\xD8]\x1A[\xDB\x88\x19\x98Z[\x19Y`j\x1B`\x84\x82\x01R`\xA4\x01a\x08PV[\x82`\x01`\x01`\xA0\x1B\x03\x16a$\x98\x83\x83a+\x08V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1A\x07W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: signature not from`d\x82\x01Rf\x109\xB4\xB3\xB72\xB9`\xC9\x1B`\x84\x82\x01R`\xA4\x01a\x08PV[`\0[\x81\x81\x10\x15a\r\xE7W`\0`@Q\x80`@\x01`@R\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x85\x85\x85\x81\x81\x10a%]Wa%]a7\xE2V[\x90P` \x02\x01` \x81\x01\x90a%r\x91\x90a9\xCEV[c\xFF\xFF\xFF\xFF\x16\x90R`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x9B` R`@\x81 \x91\x92P\x85\x85\x85\x81\x81\x10a%\xA7Wa%\xA7a7\xE2V[\x90P` \x02\x01` \x81\x01\x90a%\xBC\x91\x90a9\xCEV[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T`\xFF\x16a&SW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`B`$\x82\x01R\x7FAVSDirectory._registerOperatorTo`D\x82\x01R\x7FOperatorSets: invalid operator s`d\x82\x01Ra\x19]`\xF2\x1B`\x84\x82\x01R`\xA4\x01a\x08PV[a&]\x86\x82a\x1A\xA2V[\x15a&\xF6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`Y`$\x82\x01R\x7FAVSDirectory._registerOperatorTo`D\x82\x01R\x7FOperatorSets: operator already r`d\x82\x01R\x7Fegistered to operator set\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x08PV[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x9C` R`@\x81 \x90\x85\x85\x85\x81\x81\x10a'!Wa'!a7\xE2V[\x90P` \x02\x01` \x81\x01\x90a'6\x91\x90a9\xCEV[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x81Ta']\x90a7\xF8V[\x90\x91UPa'\x8Ba'm\x82a*aV[`\x01`\x01`\xA0\x1B\x03\x88\x16`\0\x90\x81R`\x9D` R`@\x90 \x90a+,V[P\x85`\x01`\x01`\xA0\x1B\x03\x16\x7FC#.\xDF\x90qu=#!\xE5\xFA~\x01\x83c\xEE$\x8E_!B\xE6\xC0\x8E\xDD2e\xBF\xB4\x89^\x82`@Qa'\xC5\x91\x90a4\x86V[`@Q\x80\x91\x03\x90\xA2Pa'\xD7\x81a7\xF8V[\x90Pa%'V[`\0[\x81\x81\x10\x15a\r\xE7W`\0`@Q\x80`@\x01`@R\x80\x87`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x85\x85\x85\x81\x81\x10a(\x17Wa(\x17a7\xE2V[\x90P` \x02\x01` \x81\x01\x90a(,\x91\x90a9\xCEV[c\xFF\xFF\xFF\xFF\x16\x90R\x90Pa(@\x85\x82a\x1A\xA2V[a(\xD8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`Y`$\x82\x01R\x7FAVSDirectory._deregisterOperator`D\x82\x01R\x7FFromOperatorSet: operator not re`d\x82\x01R\x7Fgistered for operator set\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x08PV[`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x9C` R`@\x81 \x90\x85\x85\x85\x81\x81\x10a)\x03Wa)\x03a7\xE2V[\x90P` \x02\x01` \x81\x01\x90a)\x18\x91\x90a9\xCEV[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x81Ta)?\x90a:\xD5V[\x90\x91UPa)ma)O\x82a*aV[`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81R`\x9D` R`@\x90 \x90a+8V[P\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\xAD4\xC3\x07\x0B\xE1\xDF\xFB\xCA\xA4\x99\xD0\0\xBA+\x8D\x98H\xAE\xFC\xAC0Y\xDF$]\xD9\\N\xCE\x14\xFE\x82`@Qa)\xA7\x91\x90a4\x86V[`@Q\x80\x91\x03\x90\xA2Pa)\xB9\x81a7\xF8V[\x90Pa'\xE1V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x11\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x08PV[`\0a*$a\"OV[`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x91\x90\x91R`B\x81\x01\x83\x90R`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0\x81`\0\x01Q\x82` \x01Qc\xFF\xFF\xFF\xFF\x16`@Q` \x01a*\xAE\x92\x91\x90``\x92\x90\x92\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82R`\xA0\x1B`\x01`\x01`\xA0\x1B\x03\x19\x16`\x14\x82\x01R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x11P\x90a:\xECV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x15\x15a\x11MV[`\0\x82`\0\x01\x82\x81T\x81\x10a*\xF5Wa*\xF5a7\xE2V[\x90`\0R` `\0 \x01T\x90P\x92\x91PPV[`\0\x80`\0a+\x17\x85\x85a+DV[\x91P\x91Pa+$\x81a+\xB4V[P\x93\x92PPPV[`\0a\x11M\x83\x83a-oV[`\0a\x11M\x83\x83a-\xBEV[`\0\x80\x82Q`A\x14\x15a+{W` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1Aa+o\x87\x82\x85\x85a.\xB1V[\x94P\x94PPPPa+\xADV[\x82Q`@\x14\x15a+\xA5W` \x83\x01Q`@\x84\x01Qa+\x9A\x86\x83\x83a/\x9EV[\x93P\x93PPPa+\xADV[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0\x81`\x04\x81\x11\x15a+\xC8Wa+\xC8a3\xD7V[\x14\x15a+\xD1WPV[`\x01\x81`\x04\x81\x11\x15a+\xE5Wa+\xE5a3\xD7V[\x14\x15a,3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x08PV[`\x02\x81`\x04\x81\x11\x15a,GWa,Ga3\xD7V[\x14\x15a,\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x08PV[`\x03\x81`\x04\x81\x11\x15a,\xA9Wa,\xA9a3\xD7V[\x14\x15a-\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x08PV[`\x04\x81`\x04\x81\x11\x15a-\x16Wa-\x16a3\xD7V[\x14\x15a\x08bW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 'v' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x08PV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 Ta-\xB6WP\x81T`\x01\x81\x81\x01\x84U`\0\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x11PV[P`\0a\x11PV[`\0\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a.\xA7W`\0a-\xE2`\x01\x83a7\xB3V[\x85T\x90\x91P`\0\x90a-\xF6\x90`\x01\x90a7\xB3V[\x90P\x81\x81\x14a.[W`\0\x86`\0\x01\x82\x81T\x81\x10a.\x16Wa.\x16a7\xE2V[\x90`\0R` `\0 \x01T\x90P\x80\x87`\0\x01\x84\x81T\x81\x10a.9Wa.9a7\xE2V[`\0\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80a.lWa.la;\x13V[`\x01\x90\x03\x81\x81\x90`\0R` `\0 \x01`\0\x90U\x90U\x85`\x01\x01`\0\x86\x81R` \x01\x90\x81R` \x01`\0 `\0\x90U`\x01\x93PPPPa\x11PV[`\0\x91PPa\x11PV[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a.\xE8WP`\0\x90P`\x03a/\x95V[\x84`\xFF\x16`\x1B\x14\x15\x80\x15a/\0WP\x84`\xFF\x16`\x1C\x14\x15[\x15a/\x11WP`\0\x90P`\x04a/\x95V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a/eW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a/\x8EW`\0`\x01\x92P\x92PPa/\x95V[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x80`\x01`\x01`\xFF\x1B\x03\x83\x16\x81a/\xBB`\xFF\x86\x90\x1C`\x1Ba7\xCAV[\x90Pa/\xC9\x87\x82\x88\x85a.\xB1V[\x93P\x93PPP\x93P\x93\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x08bW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a0\x01W`\0\x80\xFD[\x835a0\x0C\x81a/\xD7V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a0zWa0j\x84\x83Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x01RV[\x92\x84\x01\x92\x90\x85\x01\x90`\x01\x01a0>V[P\x91\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a0\x99W`\0\x80\xFD[\x815a0\xA4\x81a/\xD7V[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a0\xBDW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a0\xD9W`\0\x80\xFD[\x835a0\xE4\x81a/\xD7V[\x92P` \x84\x015a0\xF4\x81a/\xD7V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80\x83`\x1F\x84\x01\x12a1\x17W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a1.W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a+\xADW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a1\x81Wa1\x81a1IV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a1\xAFWa1\xAFa1IV[`@R\x91\x90PV[`\0``\x82\x84\x03\x12\x15a1\xC9W`\0\x80\xFD[a1\xD1a1_V[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a1\xEAW`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a1\xFEW`\0\x80\xFD[\x815` \x82\x82\x11\x15a2\x12Wa2\x12a1IV[a2$`\x1F\x83\x01`\x1F\x19\x16\x82\x01a1\x87V[\x92P\x81\x83R\x86\x81\x83\x86\x01\x01\x11\x15a2:W`\0\x80\xFD[\x81\x81\x85\x01\x82\x85\x017`\0\x81\x83\x85\x01\x01R\x82\x85R\x80\x86\x015\x81\x86\x01RPPPP`@\x82\x015`@\x82\x01R\x92\x91PPV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a2\x7FW`\0\x80\xFD[\x845a2\x8A\x81a/\xD7V[\x93P` \x85\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a2\xA6W`\0\x80\xFD[a2\xB2\x88\x83\x89\x01a1\x05V[\x90\x95P\x93P`@\x87\x015\x91P\x80\x82\x11\x15a2\xCBW`\0\x80\xFD[Pa2\xD8\x87\x82\x88\x01a1\xB7V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a2\xF7W`\0\x80\xFD[\x825a3\x02\x81a/\xD7V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a3(W`\0\x80\xFD[\x855a33\x81a/\xD7V[\x94P` \x86\x015a3C\x81a/\xD7V[\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a3_W`\0\x80\xFD[a3k\x89\x83\x8A\x01a1\x05V[\x90\x95P\x93P``\x88\x015\x91P\x80\x82\x11\x15a3\x84W`\0\x80\xFD[Pa3\x91\x88\x82\x89\x01a1\xB7V[\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x80`@\x83\x85\x03\x12\x15a3\xB1W`\0\x80\xFD[\x825a3\xBC\x81a/\xD7V[\x91P` \x83\x015a3\xCC\x81a/\xD7V[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x02\x83\x10a4\x0FWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`\0` \x82\x84\x03\x12\x15a4'W`\0\x80\xFD[\x815`\xFF\x81\x16\x81\x14a0\xA4W`\0\x80\xFD[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a4LW`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a4dW`\0\x80\xFD[\x825a4o\x81a/\xD7V[\x91Pa4}` \x84\x01a48V[\x90P\x92P\x92\x90PV[\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x83\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R`@\x81\x01a\x11PV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a4\xC4W`\0\x80\xFD[\x855a4\xCF\x81a/\xD7V[\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a4\xEAW`\0\x80\xFD[a4\xF6\x88\x82\x89\x01a1\x05V[\x96\x99\x90\x98P\x95\x96`@\x81\x015\x96``\x90\x91\x015\x95P\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a5%W`\0\x80\xFD[\x825a50\x81a/\xD7V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a5KW`\0\x80\xFD[a5W\x85\x82\x86\x01a1\xB7V[\x91PP\x92P\x92\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a5wW`\0\x80\xFD[\x845a5\x82\x81a/\xD7V[\x93P` \x85\x015a5\x92\x81a/\xD7V[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[`\0\x80` \x83\x85\x03\x12\x15a5\xBAW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a5\xD1W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a5\xE5W`\0\x80\xFD[\x815\x81\x81\x11\x15a5\xF4W`\0\x80\xFD[\x86` \x82\x85\x01\x01\x11\x15a6\x06W`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0\x80` \x83\x85\x03\x12\x15a6+W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a6AW`\0\x80\xFD[a6M\x85\x82\x86\x01a1\x05V[\x90\x96\x90\x95P\x93PPPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a6nW`\0\x80\xFD[\x835a6y\x81a/\xD7V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a6\x94W`\0\x80\xFD[a6\xA0\x86\x82\x87\x01a1\x05V[\x94\x97\x90\x96P\x93\x94PPPPV[`\0\x80\x82\x84\x03``\x81\x12\x15a6\xC1W`\0\x80\xFD[\x835a6\xCC\x81a/\xD7V[\x92P`@`\x1F\x19\x82\x01\x12\x15a6\xE0W`\0\x80\xFD[P`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a7\x03Wa7\x03a1IV[`@R` \x84\x015a7\x14\x81a/\xD7V[\x81Ra7\"`@\x85\x01a48V[` \x82\x01R\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a7HW`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a7_W`\0\x80\xFD[a7k\x88\x83\x89\x01a1\x05V[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a7\x84W`\0\x80\xFD[Pa7\x91\x87\x82\x88\x01a1\x05V[\x95\x98\x94\x97P\x95PPPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a7\xC5Wa7\xC5a7\x9DV[P\x03\x90V[`\0\x82\x19\x82\x11\x15a7\xDDWa7\xDDa7\x9DV[P\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15a8\x0CWa8\x0Ca7\x9DV[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15a8%W`\0\x80\xFD[\x81Qa0\xA4\x81a/\xD7V[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15a8\x8CW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a0\xA4W`\0\x80\xFD[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`\x19\x90\x82\x01R\x7FPausable: index is paused\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a9TWc\xFF\xFF\xFF\xFFa9A\x83a48V[\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a9+V[P\x94\x95\x94PPPPPV[\x86\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16` \x82\x01R`\xA0`@\x82\x01\x81\x90R`\0\x90a9\x8A\x90\x83\x01\x86\x88a9\x1BV[``\x83\x01\x94\x90\x94RP`\x80\x01R\x94\x93PPPPV[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017`\0\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x91\x90PV[`\0` \x82\x84\x03\x12\x15a9\xE0W`\0\x80\xFD[a\x11M\x82a48V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a:\0W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a:\x1AW`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a+\xADW`\0\x80\xFD[` \x81R`\0a:F` \x83\x01\x84\x86a9\x1BV[\x94\x93PPPPV[\x82\x81R`\0` `@\x81\x84\x01R\x83Q\x80`@\x85\x01R`\0[\x81\x81\x10\x15a:\x82W\x85\x81\x01\x83\x01Q\x85\x82\x01``\x01R\x82\x01a:fV[\x81\x81\x11\x15a:\x94W`\0``\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01``\x01\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a:\xBDW`\0\x80\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a0\xA4W`\0\x80\xFD[`\0\x81a:\xE4Wa:\xE4a7\x9DV[P`\0\x19\x01\x90V[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a;\rW`\0\x19\x81` \x03`\x03\x1B\x1B\x82\x16\x91P[P\x91\x90PV[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD\xFEAVSDirectory.registerOperatorToOAVSDirectory.registerOperatorToA\xA2dipfsX\"\x12 \xD1\xD2\x8BF\xCB\xAB\xBF\xA0\xAD?*@\x81'\xFE\xAAf\xC3\x13\xD3\"\x1Fb5\xF4)8}\xF5\xB1;\xAAdsolcC\0\x08\x0C\x003";
    /// The deployed bytecode of the contract.
    pub static AVSDIRECTORY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct AVSDirectory<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for AVSDirectory<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for AVSDirectory<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for AVSDirectory<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for AVSDirectory<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(AVSDirectory))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> AVSDirectory<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                AVSDIRECTORY_ABI.clone(),
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
                AVSDIRECTORY_ABI.clone(),
                AVSDIRECTORY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `DOMAIN_TYPEHASH` (0x20606b70) function
        pub fn domain_typehash(&self) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([32, 96, 107, 112], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `OPERATOR_AVS_REGISTRATION_TYPEHASH` (0xd79aceab) function
        pub fn operator_avs_registration_typehash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([215, 154, 206, 171], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `OPERATOR_SET_FORCE_DEREGISTRATION_TYPEHASH` (0xdce974b9) function
        pub fn operator_set_force_deregistration_typehash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([220, 233, 116, 185], ())
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
        ///Calls the contract's `avsOperatorStatus` (0x49075da3) function
        pub fn avs_operator_status(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([73, 7, 93, 163], (p0, p1))
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
        ///Calls the contract's `delegation` (0xdf5cf723) function
        pub fn delegation(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([223, 92, 247, 35], ())
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
        ///Calls the contract's `inTotalOperatorSets` (0xcbdf0e42) function
        pub fn in_total_operator_sets(
            &self,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([203, 223, 14, 66], operator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x1794bb3c) function
        pub fn initialize(
            &self,
            initial_owner: ::ethers::core::types::Address,
            pauser_registry: ::ethers::core::types::Address,
            initial_paused_status: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [23, 148, 187, 60],
                    (initial_owner, pauser_registry, initial_paused_status),
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
            p0: ::ethers::core::types::Address,
            p1: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([132, 215, 111, 123], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isOperatorSetAVS` (0x7673e93a) function
        pub fn is_operator_set_avs(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([118, 115, 233, 58], p0)
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
            p0: ::ethers::core::types::Address,
            p1: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([55, 72, 35, 181], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operatorSetMemberCount` (0xdae226b6) function
        pub fn operator_set_member_count(
            &self,
            p0: ::ethers::core::types::Address,
            p1: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([218, 226, 38, 182], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operatorSetsMemberOf` (0x0d5387c5) function
        pub fn operator_sets_member_of_with_operator_and_start(
            &self,
            operator: ::ethers::core::types::Address,
            start: ::ethers::core::types::U256,
            length: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<OperatorSet>> {
            self.0
                .method_hash([13, 83, 135, 197], (operator, start, length))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operatorSetsMemberOf` (0x8de54944) function
        pub fn operator_sets_member_of(
            &self,
            operator: ::ethers::core::types::Address,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, OperatorSet> {
            self.0
                .method_hash([141, 229, 73, 68], (operator, index))
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
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
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
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
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
        ///Gets the contract's `Initialized` event
        pub fn initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, InitializedFilter>
        {
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
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OwnershipTransferredFilter>
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
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AVSDirectoryEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for AVSDirectory<M> {
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
    pub enum AVSDirectoryEvents {
        AvsmetadataURIUpdatedFilter(AvsmetadataURIUpdatedFilter),
        AvsmigratedToOperatorSetsFilter(AvsmigratedToOperatorSetsFilter),
        InitializedFilter(InitializedFilter),
        OperatorAVSRegistrationStatusUpdatedFilter(OperatorAVSRegistrationStatusUpdatedFilter),
        OperatorAddedToOperatorSetFilter(OperatorAddedToOperatorSetFilter),
        OperatorMigratedToOperatorSetsFilter(OperatorMigratedToOperatorSetsFilter),
        OperatorRemovedFromOperatorSetFilter(OperatorRemovedFromOperatorSetFilter),
        OperatorSetCreatedFilter(OperatorSetCreatedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PausedFilter(PausedFilter),
        PauserRegistrySetFilter(PauserRegistrySetFilter),
        UnpausedFilter(UnpausedFilter),
    }
    impl ::ethers::contract::EthLogDecode for AVSDirectoryEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AvsmetadataURIUpdatedFilter::decode_log(log) {
                return Ok(AVSDirectoryEvents::AvsmetadataURIUpdatedFilter(decoded));
            }
            if let Ok(decoded) = AvsmigratedToOperatorSetsFilter::decode_log(log) {
                return Ok(AVSDirectoryEvents::AvsmigratedToOperatorSetsFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(AVSDirectoryEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OperatorAVSRegistrationStatusUpdatedFilter::decode_log(log) {
                return Ok(AVSDirectoryEvents::OperatorAVSRegistrationStatusUpdatedFilter(decoded));
            }
            if let Ok(decoded) = OperatorAddedToOperatorSetFilter::decode_log(log) {
                return Ok(AVSDirectoryEvents::OperatorAddedToOperatorSetFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = OperatorMigratedToOperatorSetsFilter::decode_log(log) {
                return Ok(AVSDirectoryEvents::OperatorMigratedToOperatorSetsFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = OperatorRemovedFromOperatorSetFilter::decode_log(log) {
                return Ok(AVSDirectoryEvents::OperatorRemovedFromOperatorSetFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = OperatorSetCreatedFilter::decode_log(log) {
                return Ok(AVSDirectoryEvents::OperatorSetCreatedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(AVSDirectoryEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(AVSDirectoryEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = PauserRegistrySetFilter::decode_log(log) {
                return Ok(AVSDirectoryEvents::PauserRegistrySetFilter(decoded));
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(AVSDirectoryEvents::UnpausedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for AVSDirectoryEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AvsmetadataURIUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::AvsmigratedToOperatorSetsFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PausedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauserRegistrySetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnpausedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AvsmetadataURIUpdatedFilter> for AVSDirectoryEvents {
        fn from(value: AvsmetadataURIUpdatedFilter) -> Self {
            Self::AvsmetadataURIUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<AvsmigratedToOperatorSetsFilter> for AVSDirectoryEvents {
        fn from(value: AvsmigratedToOperatorSetsFilter) -> Self {
            Self::AvsmigratedToOperatorSetsFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for AVSDirectoryEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OperatorAVSRegistrationStatusUpdatedFilter> for AVSDirectoryEvents {
        fn from(value: OperatorAVSRegistrationStatusUpdatedFilter) -> Self {
            Self::OperatorAVSRegistrationStatusUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<OperatorAddedToOperatorSetFilter> for AVSDirectoryEvents {
        fn from(value: OperatorAddedToOperatorSetFilter) -> Self {
            Self::OperatorAddedToOperatorSetFilter(value)
        }
    }
    impl ::core::convert::From<OperatorMigratedToOperatorSetsFilter> for AVSDirectoryEvents {
        fn from(value: OperatorMigratedToOperatorSetsFilter) -> Self {
            Self::OperatorMigratedToOperatorSetsFilter(value)
        }
    }
    impl ::core::convert::From<OperatorRemovedFromOperatorSetFilter> for AVSDirectoryEvents {
        fn from(value: OperatorRemovedFromOperatorSetFilter) -> Self {
            Self::OperatorRemovedFromOperatorSetFilter(value)
        }
    }
    impl ::core::convert::From<OperatorSetCreatedFilter> for AVSDirectoryEvents {
        fn from(value: OperatorSetCreatedFilter) -> Self {
            Self::OperatorSetCreatedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for AVSDirectoryEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<PausedFilter> for AVSDirectoryEvents {
        fn from(value: PausedFilter) -> Self {
            Self::PausedFilter(value)
        }
    }
    impl ::core::convert::From<PauserRegistrySetFilter> for AVSDirectoryEvents {
        fn from(value: PauserRegistrySetFilter) -> Self {
            Self::PauserRegistrySetFilter(value)
        }
    }
    impl ::core::convert::From<UnpausedFilter> for AVSDirectoryEvents {
        fn from(value: UnpausedFilter) -> Self {
            Self::UnpausedFilter(value)
        }
    }
    ///Container type for all input parameters for the `DOMAIN_TYPEHASH` function with signature `DOMAIN_TYPEHASH()` and selector `0x20606b70`
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
    #[ethcall(name = "DOMAIN_TYPEHASH", abi = "DOMAIN_TYPEHASH()")]
    pub struct DomainTypehashCall;
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
    ///Container type for all input parameters for the `OPERATOR_SET_FORCE_DEREGISTRATION_TYPEHASH` function with signature `OPERATOR_SET_FORCE_DEREGISTRATION_TYPEHASH()` and selector `0xdce974b9`
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
        name = "OPERATOR_SET_FORCE_DEREGISTRATION_TYPEHASH",
        abi = "OPERATOR_SET_FORCE_DEREGISTRATION_TYPEHASH()"
    )]
    pub struct OperatorSetForceDeregistrationTypehashCall;
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
    ///Container type for all input parameters for the `avsOperatorStatus` function with signature `avsOperatorStatus(address,address)` and selector `0x49075da3`
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
    #[ethcall(name = "avsOperatorStatus", abi = "avsOperatorStatus(address,address)")]
    pub struct AvsOperatorStatusCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
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
    ///Container type for all input parameters for the `delegation` function with signature `delegation()` and selector `0xdf5cf723`
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
    #[ethcall(name = "delegation", abi = "delegation()")]
    pub struct DelegationCall;
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
    ///Container type for all input parameters for the `inTotalOperatorSets` function with signature `inTotalOperatorSets(address)` and selector `0xcbdf0e42`
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
    #[ethcall(name = "inTotalOperatorSets", abi = "inTotalOperatorSets(address)")]
    pub struct InTotalOperatorSetsCall {
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address,uint256)` and selector `0x1794bb3c`
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
    #[ethcall(name = "initialize", abi = "initialize(address,address,uint256)")]
    pub struct InitializeCall {
        pub initial_owner: ::ethers::core::types::Address,
        pub pauser_registry: ::ethers::core::types::Address,
        pub initial_paused_status: ::ethers::core::types::U256,
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
    pub struct IsOperatorSetCall(pub ::ethers::core::types::Address, pub u32);
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
    pub struct IsOperatorSetAVSCall(pub ::ethers::core::types::Address);
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
    pub struct OperatorSaltIsSpentCall(pub ::ethers::core::types::Address, pub [u8; 32]);
    ///Container type for all input parameters for the `operatorSetMemberCount` function with signature `operatorSetMemberCount(address,uint32)` and selector `0xdae226b6`
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
        name = "operatorSetMemberCount",
        abi = "operatorSetMemberCount(address,uint32)"
    )]
    pub struct OperatorSetMemberCountCall(pub ::ethers::core::types::Address, pub u32);
    ///Container type for all input parameters for the `operatorSetsMemberOf` function with signature `operatorSetsMemberOf(address,uint256,uint256)` and selector `0x0d5387c5`
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
        name = "operatorSetsMemberOf",
        abi = "operatorSetsMemberOf(address,uint256,uint256)"
    )]
    pub struct OperatorSetsMemberOfWithOperatorAndStartCall {
        pub operator: ::ethers::core::types::Address,
        pub start: ::ethers::core::types::U256,
        pub length: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `operatorSetsMemberOf` function with signature `operatorSetsMemberOf(address,uint256)` and selector `0x8de54944`
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
        name = "operatorSetsMemberOf",
        abi = "operatorSetsMemberOf(address,uint256)"
    )]
    pub struct OperatorSetsMemberOfCall {
        pub operator: ::ethers::core::types::Address,
        pub index: ::ethers::core::types::U256,
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
    pub enum AVSDirectoryCalls {
        DomainTypehash(DomainTypehashCall),
        OperatorAvsRegistrationTypehash(OperatorAvsRegistrationTypehashCall),
        OperatorSetForceDeregistrationTypehash(OperatorSetForceDeregistrationTypehashCall),
        OperatorSetRegistrationTypehash(OperatorSetRegistrationTypehashCall),
        AvsOperatorStatus(AvsOperatorStatusCall),
        BecomeOperatorSetAVS(BecomeOperatorSetAVSCall),
        CalculateOperatorAVSRegistrationDigestHash(CalculateOperatorAVSRegistrationDigestHashCall),
        CalculateOperatorSetForceDeregistrationTypehash(
            CalculateOperatorSetForceDeregistrationTypehashCall,
        ),
        CalculateOperatorSetRegistrationDigestHash(CalculateOperatorSetRegistrationDigestHashCall),
        CancelSalt(CancelSaltCall),
        CreateOperatorSets(CreateOperatorSetsCall),
        Delegation(DelegationCall),
        DeregisterOperatorFromAVS(DeregisterOperatorFromAVSCall),
        DeregisterOperatorFromOperatorSets(DeregisterOperatorFromOperatorSetsCall),
        DomainSeparator(DomainSeparatorCall),
        ForceDeregisterFromOperatorSets(ForceDeregisterFromOperatorSetsCall),
        InTotalOperatorSets(InTotalOperatorSetsCall),
        Initialize(InitializeCall),
        IsMember(IsMemberCall),
        IsOperatorSet(IsOperatorSetCall),
        IsOperatorSetAVS(IsOperatorSetAVSCall),
        MigrateOperatorsToOperatorSets(MigrateOperatorsToOperatorSetsCall),
        OperatorSaltIsSpent(OperatorSaltIsSpentCall),
        OperatorSetMemberCount(OperatorSetMemberCountCall),
        OperatorSetsMemberOfWithOperatorAndStart(OperatorSetsMemberOfWithOperatorAndStartCall),
        OperatorSetsMemberOf(OperatorSetsMemberOfCall),
        Owner(OwnerCall),
        Pause(PauseCall),
        PauseAll(PauseAllCall),
        PausedWithIndex(PausedWithIndexCall),
        Paused(PausedCall),
        PauserRegistry(PauserRegistryCall),
        RegisterOperatorToAVS(RegisterOperatorToAVSCall),
        RegisterOperatorToOperatorSets(RegisterOperatorToOperatorSetsCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetPauserRegistry(SetPauserRegistryCall),
        TransferOwnership(TransferOwnershipCall),
        Unpause(UnpauseCall),
        UpdateAVSMetadataURI(UpdateAVSMetadataURICall),
    }
    impl ::ethers::core::abi::AbiDecode for AVSDirectoryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <DomainTypehashCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DomainTypehash(decoded));
            }
            if let Ok(decoded) =
                <OperatorAvsRegistrationTypehashCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::OperatorAvsRegistrationTypehash(decoded));
            }
            if let Ok(decoded) = <OperatorSetForceDeregistrationTypehashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OperatorSetForceDeregistrationTypehash(decoded));
            }
            if let Ok(decoded) =
                <OperatorSetRegistrationTypehashCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::OperatorSetRegistrationTypehash(decoded));
            }
            if let Ok(decoded) =
                <AvsOperatorStatusCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AvsOperatorStatus(decoded));
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
            if let Ok(decoded) = <DelegationCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Delegation(decoded));
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
            if let Ok(decoded) =
                <InTotalOperatorSetsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InTotalOperatorSets(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
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
                <OperatorSetMemberCountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OperatorSetMemberCount(decoded));
            }
            if let Ok(decoded) = <OperatorSetsMemberOfWithOperatorAndStartCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OperatorSetsMemberOfWithOperatorAndStart(decoded));
            }
            if let Ok(decoded) =
                <OperatorSetsMemberOfCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OperatorSetsMemberOf(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
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
                <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <SetPauserRegistryCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetPauserRegistry(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <UnpauseCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Unpause(decoded));
            }
            if let Ok(decoded) =
                <UpdateAVSMetadataURICall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateAVSMetadataURI(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AVSDirectoryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DomainTypehash(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OperatorAvsRegistrationTypehash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OperatorSetForceDeregistrationTypehash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OperatorSetRegistrationTypehash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AvsOperatorStatus(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::Delegation(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::InTotalOperatorSets(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsMember(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsOperatorSet(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsOperatorSetAVS(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MigrateOperatorsToOperatorSets(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OperatorSaltIsSpent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OperatorSetMemberCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OperatorSetsMemberOfWithOperatorAndStart(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OperatorSetsMemberOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PauseAll(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PausedWithIndex(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Paused(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PauserRegistry(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RegisterOperatorToAVS(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterOperatorToOperatorSets(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetPauserRegistry(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Unpause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateAVSMetadataURI(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for AVSDirectoryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DomainTypehash(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorAvsRegistrationTypehash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorSetForceDeregistrationTypehash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorSetRegistrationTypehash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AvsOperatorStatus(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::Delegation(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeregisterOperatorFromAVS(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeregisterOperatorFromOperatorSets(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DomainSeparator(element) => ::core::fmt::Display::fmt(element, f),
                Self::ForceDeregisterFromOperatorSets(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InTotalOperatorSets(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsMember(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsOperatorSet(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsOperatorSetAVS(element) => ::core::fmt::Display::fmt(element, f),
                Self::MigrateOperatorsToOperatorSets(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorSaltIsSpent(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorSetMemberCount(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorSetsMemberOfWithOperatorAndStart(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorSetsMemberOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pause(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauseAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::PausedWithIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::Paused(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauserRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterOperatorToAVS(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterOperatorToOperatorSets(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPauserRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unpause(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateAVSMetadataURI(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DomainTypehashCall> for AVSDirectoryCalls {
        fn from(value: DomainTypehashCall) -> Self {
            Self::DomainTypehash(value)
        }
    }
    impl ::core::convert::From<OperatorAvsRegistrationTypehashCall> for AVSDirectoryCalls {
        fn from(value: OperatorAvsRegistrationTypehashCall) -> Self {
            Self::OperatorAvsRegistrationTypehash(value)
        }
    }
    impl ::core::convert::From<OperatorSetForceDeregistrationTypehashCall> for AVSDirectoryCalls {
        fn from(value: OperatorSetForceDeregistrationTypehashCall) -> Self {
            Self::OperatorSetForceDeregistrationTypehash(value)
        }
    }
    impl ::core::convert::From<OperatorSetRegistrationTypehashCall> for AVSDirectoryCalls {
        fn from(value: OperatorSetRegistrationTypehashCall) -> Self {
            Self::OperatorSetRegistrationTypehash(value)
        }
    }
    impl ::core::convert::From<AvsOperatorStatusCall> for AVSDirectoryCalls {
        fn from(value: AvsOperatorStatusCall) -> Self {
            Self::AvsOperatorStatus(value)
        }
    }
    impl ::core::convert::From<BecomeOperatorSetAVSCall> for AVSDirectoryCalls {
        fn from(value: BecomeOperatorSetAVSCall) -> Self {
            Self::BecomeOperatorSetAVS(value)
        }
    }
    impl ::core::convert::From<CalculateOperatorAVSRegistrationDigestHashCall> for AVSDirectoryCalls {
        fn from(value: CalculateOperatorAVSRegistrationDigestHashCall) -> Self {
            Self::CalculateOperatorAVSRegistrationDigestHash(value)
        }
    }
    impl ::core::convert::From<CalculateOperatorSetForceDeregistrationTypehashCall>
        for AVSDirectoryCalls
    {
        fn from(value: CalculateOperatorSetForceDeregistrationTypehashCall) -> Self {
            Self::CalculateOperatorSetForceDeregistrationTypehash(value)
        }
    }
    impl ::core::convert::From<CalculateOperatorSetRegistrationDigestHashCall> for AVSDirectoryCalls {
        fn from(value: CalculateOperatorSetRegistrationDigestHashCall) -> Self {
            Self::CalculateOperatorSetRegistrationDigestHash(value)
        }
    }
    impl ::core::convert::From<CancelSaltCall> for AVSDirectoryCalls {
        fn from(value: CancelSaltCall) -> Self {
            Self::CancelSalt(value)
        }
    }
    impl ::core::convert::From<CreateOperatorSetsCall> for AVSDirectoryCalls {
        fn from(value: CreateOperatorSetsCall) -> Self {
            Self::CreateOperatorSets(value)
        }
    }
    impl ::core::convert::From<DelegationCall> for AVSDirectoryCalls {
        fn from(value: DelegationCall) -> Self {
            Self::Delegation(value)
        }
    }
    impl ::core::convert::From<DeregisterOperatorFromAVSCall> for AVSDirectoryCalls {
        fn from(value: DeregisterOperatorFromAVSCall) -> Self {
            Self::DeregisterOperatorFromAVS(value)
        }
    }
    impl ::core::convert::From<DeregisterOperatorFromOperatorSetsCall> for AVSDirectoryCalls {
        fn from(value: DeregisterOperatorFromOperatorSetsCall) -> Self {
            Self::DeregisterOperatorFromOperatorSets(value)
        }
    }
    impl ::core::convert::From<DomainSeparatorCall> for AVSDirectoryCalls {
        fn from(value: DomainSeparatorCall) -> Self {
            Self::DomainSeparator(value)
        }
    }
    impl ::core::convert::From<ForceDeregisterFromOperatorSetsCall> for AVSDirectoryCalls {
        fn from(value: ForceDeregisterFromOperatorSetsCall) -> Self {
            Self::ForceDeregisterFromOperatorSets(value)
        }
    }
    impl ::core::convert::From<InTotalOperatorSetsCall> for AVSDirectoryCalls {
        fn from(value: InTotalOperatorSetsCall) -> Self {
            Self::InTotalOperatorSets(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for AVSDirectoryCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<IsMemberCall> for AVSDirectoryCalls {
        fn from(value: IsMemberCall) -> Self {
            Self::IsMember(value)
        }
    }
    impl ::core::convert::From<IsOperatorSetCall> for AVSDirectoryCalls {
        fn from(value: IsOperatorSetCall) -> Self {
            Self::IsOperatorSet(value)
        }
    }
    impl ::core::convert::From<IsOperatorSetAVSCall> for AVSDirectoryCalls {
        fn from(value: IsOperatorSetAVSCall) -> Self {
            Self::IsOperatorSetAVS(value)
        }
    }
    impl ::core::convert::From<MigrateOperatorsToOperatorSetsCall> for AVSDirectoryCalls {
        fn from(value: MigrateOperatorsToOperatorSetsCall) -> Self {
            Self::MigrateOperatorsToOperatorSets(value)
        }
    }
    impl ::core::convert::From<OperatorSaltIsSpentCall> for AVSDirectoryCalls {
        fn from(value: OperatorSaltIsSpentCall) -> Self {
            Self::OperatorSaltIsSpent(value)
        }
    }
    impl ::core::convert::From<OperatorSetMemberCountCall> for AVSDirectoryCalls {
        fn from(value: OperatorSetMemberCountCall) -> Self {
            Self::OperatorSetMemberCount(value)
        }
    }
    impl ::core::convert::From<OperatorSetsMemberOfWithOperatorAndStartCall> for AVSDirectoryCalls {
        fn from(value: OperatorSetsMemberOfWithOperatorAndStartCall) -> Self {
            Self::OperatorSetsMemberOfWithOperatorAndStart(value)
        }
    }
    impl ::core::convert::From<OperatorSetsMemberOfCall> for AVSDirectoryCalls {
        fn from(value: OperatorSetsMemberOfCall) -> Self {
            Self::OperatorSetsMemberOf(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for AVSDirectoryCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PauseCall> for AVSDirectoryCalls {
        fn from(value: PauseCall) -> Self {
            Self::Pause(value)
        }
    }
    impl ::core::convert::From<PauseAllCall> for AVSDirectoryCalls {
        fn from(value: PauseAllCall) -> Self {
            Self::PauseAll(value)
        }
    }
    impl ::core::convert::From<PausedWithIndexCall> for AVSDirectoryCalls {
        fn from(value: PausedWithIndexCall) -> Self {
            Self::PausedWithIndex(value)
        }
    }
    impl ::core::convert::From<PausedCall> for AVSDirectoryCalls {
        fn from(value: PausedCall) -> Self {
            Self::Paused(value)
        }
    }
    impl ::core::convert::From<PauserRegistryCall> for AVSDirectoryCalls {
        fn from(value: PauserRegistryCall) -> Self {
            Self::PauserRegistry(value)
        }
    }
    impl ::core::convert::From<RegisterOperatorToAVSCall> for AVSDirectoryCalls {
        fn from(value: RegisterOperatorToAVSCall) -> Self {
            Self::RegisterOperatorToAVS(value)
        }
    }
    impl ::core::convert::From<RegisterOperatorToOperatorSetsCall> for AVSDirectoryCalls {
        fn from(value: RegisterOperatorToOperatorSetsCall) -> Self {
            Self::RegisterOperatorToOperatorSets(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for AVSDirectoryCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SetPauserRegistryCall> for AVSDirectoryCalls {
        fn from(value: SetPauserRegistryCall) -> Self {
            Self::SetPauserRegistry(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for AVSDirectoryCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UnpauseCall> for AVSDirectoryCalls {
        fn from(value: UnpauseCall) -> Self {
            Self::Unpause(value)
        }
    }
    impl ::core::convert::From<UpdateAVSMetadataURICall> for AVSDirectoryCalls {
        fn from(value: UpdateAVSMetadataURICall) -> Self {
            Self::UpdateAVSMetadataURI(value)
        }
    }
    ///Container type for all return fields from the `DOMAIN_TYPEHASH` function with signature `DOMAIN_TYPEHASH()` and selector `0x20606b70`
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
    pub struct DomainTypehashReturn(pub [u8; 32]);
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
    ///Container type for all return fields from the `OPERATOR_SET_FORCE_DEREGISTRATION_TYPEHASH` function with signature `OPERATOR_SET_FORCE_DEREGISTRATION_TYPEHASH()` and selector `0xdce974b9`
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
    pub struct OperatorSetForceDeregistrationTypehashReturn(pub [u8; 32]);
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
    ///Container type for all return fields from the `avsOperatorStatus` function with signature `avsOperatorStatus(address,address)` and selector `0x49075da3`
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
    pub struct AvsOperatorStatusReturn(pub u8);
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
    ///Container type for all return fields from the `delegation` function with signature `delegation()` and selector `0xdf5cf723`
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
    pub struct DelegationReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `inTotalOperatorSets` function with signature `inTotalOperatorSets(address)` and selector `0xcbdf0e42`
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
    pub struct InTotalOperatorSetsReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `operatorSetMemberCount` function with signature `operatorSetMemberCount(address,uint32)` and selector `0xdae226b6`
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
    pub struct OperatorSetMemberCountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `operatorSetsMemberOf` function with signature `operatorSetsMemberOf(address,uint256,uint256)` and selector `0x0d5387c5`
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
    pub struct OperatorSetsMemberOfWithOperatorAndStartReturn {
        pub operator_sets: ::std::vec::Vec<OperatorSet>,
    }
    ///Container type for all return fields from the `operatorSetsMemberOf` function with signature `operatorSetsMemberOf(address,uint256)` and selector `0x8de54944`
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
    pub struct OperatorSetsMemberOfReturn(pub OperatorSet);
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
}
