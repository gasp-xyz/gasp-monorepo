pub use delegation_manager::*;
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
pub mod delegation_manager {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_strategyManager"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IStrategyManager"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_slasher"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract ISlasher"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_eigenPodManager"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IEigenPodManager"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DELEGATION_APPROVAL_TYPEHASH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DELEGATION_APPROVAL_TYPEHASH",
                            ),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("DOMAIN_TYPEHASH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DOMAIN_TYPEHASH"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("MAX_STAKER_OPT_OUT_WINDOW_BLOCKS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MAX_STAKER_OPT_OUT_WINDOW_BLOCKS",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MAX_WITHDRAWAL_DELAY_BLOCKS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MAX_WITHDRAWAL_DELAY_BLOCKS",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "OPERATOR_AVS_REGISTRATION_TYPEHASH",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OPERATOR_AVS_REGISTRATION_TYPEHASH",
                            ),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("STAKER_DELEGATION_TYPEHASH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "STAKER_DELEGATION_TYPEHASH",
                            ),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("avsOperatorStatus"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum IDelegationManager.OperatorAVSRegistrationStatus",
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
                    ::std::borrow::ToOwned::to_owned("beaconChainETHStrategy"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "beaconChainETHStrategy",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IStrategy"),
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
                        "calculateCurrentStakerDelegationDigestHash",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateCurrentStakerDelegationDigestHash",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("staker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("expiry"),
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
                    ::std::borrow::ToOwned::to_owned(
                        "calculateDelegationApprovalDigestHash",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateDelegationApprovalDigestHash",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("staker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_delegationApprover",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("approverSalt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("expiry"),
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
                    ::std::borrow::ToOwned::to_owned(
                        "calculateOperatorAVSRegistrationDigestHash",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("expiry"),
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
                    ::std::borrow::ToOwned::to_owned(
                        "calculateStakerDelegationDigestHash",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateStakerDelegationDigestHash",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("staker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_stakerNonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("expiry"),
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
                    ::std::borrow::ToOwned::to_owned("calculateWithdrawalRoot"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "calculateWithdrawalRoot",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("withdrawal"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IDelegationManager.Withdrawal",
                                        ),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("completeQueuedWithdrawal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "completeQueuedWithdrawal",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("withdrawal"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IDelegationManager.Withdrawal",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IERC20[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "middlewareTimesIndex",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiveAsTokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                    ::std::borrow::ToOwned::to_owned("completeQueuedWithdrawals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "completeQueuedWithdrawals",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("withdrawals"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IDelegationManager.Withdrawal[]",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ),
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IERC20[][]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "middlewareTimesIndexes",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiveAsTokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool[]"),
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
                    ::std::borrow::ToOwned::to_owned("cumulativeWithdrawalsQueued"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "cumulativeWithdrawalsQueued",
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
                    ::std::borrow::ToOwned::to_owned("decreaseDelegatedShares"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "decreaseDelegatedShares",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("staker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("strategy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IStrategy"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("shares"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("delegateTo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("delegateTo"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "approverSignatureAndExpiry",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ISignatureUtils.SignatureWithExpiry",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("approverSalt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("delegateToBySignature"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "delegateToBySignature",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("staker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "stakerSignatureAndExpiry",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ISignatureUtils.SignatureWithExpiry",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "approverSignatureAndExpiry",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct ISignatureUtils.SignatureWithExpiry",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("approverSalt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("delegatedTo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("delegatedTo"),
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
                    ::std::borrow::ToOwned::to_owned("delegationApprover"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("delegationApprover"),
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
                    ::std::borrow::ToOwned::to_owned("delegationApproverSaltIsSpent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "delegationApproverSaltIsSpent",
                            ),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                    ::std::borrow::ToOwned::to_owned("domainSeparator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("domainSeparator"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("earningsReceiver"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("earningsReceiver"),
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
                    ::std::borrow::ToOwned::to_owned("eigenPodManager"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("eigenPodManager"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IEigenPodManager",
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
                    ::std::borrow::ToOwned::to_owned("getDelegatableShares"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getDelegatableShares",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("staker"),
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
                                        ::std::borrow::ToOwned::to_owned("contract IStrategy[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("increaseDelegatedShares"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "increaseDelegatedShares",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("staker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("strategy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IStrategy"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("shares"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                                    name: ::std::borrow::ToOwned::to_owned("_pauserRegistry"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IPauserRegistry"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "initialPausedStatus",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_withdrawalDelayBlocks",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("isDelegated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isDelegated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("staker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isOperator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isOperator"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("migrateQueuedWithdrawals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "migrateQueuedWithdrawals",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "withdrawalsToMigrate",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                                        ],
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IStrategyManager.DeprecatedStruct_QueuedWithdrawal[]",
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
                    ::std::borrow::ToOwned::to_owned("modifyOperatorDetails"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "modifyOperatorDetails",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newOperatorDetails",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IDelegationManager.OperatorDetails",
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
                    ::std::borrow::ToOwned::to_owned("operatorDetails"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("operatorDetails"),
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
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IDelegationManager.OperatorDetails",
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
                    ::std::borrow::ToOwned::to_owned("operatorSaltIsSpent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "operatorSaltIsSpent",
                            ),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("operatorShares"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("operatorShares"),
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
                                        ::std::borrow::ToOwned::to_owned("contract IStrategy"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                    ::std::borrow::ToOwned::to_owned("pause"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pause"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newPausedStatus"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("pauseAll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pauseAll"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("paused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("paused"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("paused"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pauserRegistry"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pauserRegistry"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IPauserRegistry"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pendingWithdrawals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pendingWithdrawals"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("queueWithdrawals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("queueWithdrawals"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "queuedWithdrawalParams",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IDelegationManager.QueuedWithdrawalParams[]",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("registerAsOperator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("registerAsOperator"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "registeringOperatorDetails",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IDelegationManager.OperatorDetails",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("metadataURI"),
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
                    ::std::borrow::ToOwned::to_owned("setPauserRegistry"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setPauserRegistry"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newPauserRegistry"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IPauserRegistry"),
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
                    ::std::borrow::ToOwned::to_owned("slasher"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("slasher"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ISlasher"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("stakerNonce"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("stakerNonce"),
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
                    ::std::borrow::ToOwned::to_owned("stakerOptOutWindowBlocks"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "stakerOptOutWindowBlocks",
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
                    ::std::borrow::ToOwned::to_owned("strategyManager"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("strategyManager"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IStrategyManager",
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
                    ::std::borrow::ToOwned::to_owned("undelegate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("undelegate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("staker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("withdrawalRoots"),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unpause"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("unpause"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newPausedStatus"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                                    name: ::std::borrow::ToOwned::to_owned("metadataURI"),
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
                (
                    ::std::borrow::ToOwned::to_owned("updateOperatorMetadataURI"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateOperatorMetadataURI",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("metadataURI"),
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
                (
                    ::std::borrow::ToOwned::to_owned("withdrawalDelayBlocks"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "withdrawalDelayBlocks",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AVSMetadataURIUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AVSMetadataURIUpdated",
                            ),
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
                    ::std::borrow::ToOwned::to_owned(
                        "OperatorAVSRegistrationStatusUpdated",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OperatorDetailsModified"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OperatorDetailsModified",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newOperatorDetails",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ],
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OperatorMetadataURIUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OperatorMetadataURIUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OperatorRegistered"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("OperatorRegistered"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("operatorDetails"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ],
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OperatorSharesDecreased"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OperatorSharesDecreased",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("staker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("strategy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("shares"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OperatorSharesIncreased"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OperatorSharesIncreased",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("staker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("strategy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("shares"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("Paused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Paused"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newPausedStatus"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PauserRegistrySet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StakerDelegated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("StakerDelegated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("staker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StakerForceUndelegated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "StakerForceUndelegated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("staker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StakerUndelegated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("StakerUndelegated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("staker"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Unpaused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Unpaused"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newPausedStatus"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WithdrawalCompleted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "WithdrawalCompleted",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("withdrawalRoot"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WithdrawalDelayBlocksSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "WithdrawalDelayBlocksSet",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousValue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newValue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WithdrawalMigrated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("WithdrawalMigrated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldWithdrawalRoot"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newWithdrawalRoot"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WithdrawalQueued"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("WithdrawalQueued"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("withdrawalRoot"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("withdrawal"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
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
    pub static DELEGATIONMANAGER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01\0`@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@Qb\0_\xB08\x03\x80b\0_\xB0\x839\x81\x01`@\x81\x90Rb\0\x005\x91b\0\x01@V[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\x80R\x80\x82\x16`\xC0R\x82\x16`\xA0Rb\0\0Xb\0\0eV[PPF`\xE0RPb\0\x01\x94V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15b\0\0\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\xFF\x90\x81\x16\x10\x15b\0\x01%W`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01=W`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15b\0\x01VW`\0\x80\xFD[\x83Qb\0\x01c\x81b\0\x01'V[` \x85\x01Q\x90\x93Pb\0\x01v\x81b\0\x01'V[`@\x85\x01Q\x90\x92Pb\0\x01\x89\x81b\0\x01'V[\x80\x91PP\x92P\x92P\x92V[`\x80Q`\xA0Q`\xC0Q`\xE0Qa]\x87b\0\x02)`\09`\0a+f\x01R`\0\x81\x81a\x05\xF2\x01R\x81\x81a\x10\xB2\x01R\x81\x81a\x12\xD6\x01R\x81\x81a \xC6\x01R\x81\x81a.\xBF\x01R\x81\x81a=\x10\x01RaD\xD7\x01R`\0a\x08?\x01R`\0\x81\x81a\x05J\x01R\x81\x81a\x10\x80\x01R\x81\x81a\x12\xA4\x01R\x81\x81a\x15\xCC\x01R\x81\x81a!Z\x01R\x81\x81a/q\x01R\x81\x81a>>\x01RaE}\x01Ra]\x87`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x03\x8EW`\x005`\xE0\x1C\x80ce\xDA\x12d\x11a\x01\xDEW\x80c\xB14Bq\x11a\x01\x0FW\x80c\xD7\x9A\xCE\xAB\x11a\0\xADW\x80c\xF1ar\xB0\x11a\0|W\x80c\xF1ar\xB0\x14a\t\xF5W\x80c\xF2\xFD\xE3\x8B\x14a\n\x08W\x80c\xF6\x98\xDA%\x14a\n\x1BW\x80c\xFA\xBC\x1C\xBC\x14a\n#W`\0\x80\xFD[\x80c\xD7\x9A\xCE\xAB\x14a\t\x95W\x80c\xDA\x8B\xE8d\x14a\t\xBCW\x80c\xEB\x99\x0CY\x14a\t\xCFW\x80c\xEE\xA9\x06K\x14a\t\xE2W`\0\x80\xFD[\x80c\xC5\xE4\x80\xDB\x11a\0\xE9W\x80c\xC5\xE4\x80\xDB\x14a\x08\xB2W\x80c\xC9KQ\x11\x14a\tXW\x80c\xCAf\x1C\x04\x14a\tkW\x80c\xCF\x80\x87>\x14a\ttW`\0\x80\xFD[\x80c\xB14Bq\x14a\x08:W\x80c\xB7\xF0n\xBE\x14a\x08aW\x80c\xBBE\xFE\xF2\x14a\x08\x84W`\0\x80\xFD[\x80c\x91\x04\xC3\x19\x11a\x01|W\x80c\xA1\x06\x0C\x88\x11a\x01VW\x80c\xA1\x06\x0C\x88\x14a\x07\xE1W\x80c\xA1x\x84\x84\x14a\x07\xF4W\x80c\xA3d\xF4\xDA\x14a\x08\x14W\x80c\xA9\x8F\xB3U\x14a\x08'W`\0\x80\xFD[\x80c\x91\x04\xC3\x19\x14a\x07\xA0W\x80c\x99&\xEE}\x14a\x07\xBBW\x80c\x99\xBE\x81\xC8\x14a\x07\xCEW`\0\x80\xFD[\x80cw\x8EU\xF3\x11a\x01\xB8W\x80cw\x8EU\xF3\x14a\x07>W\x80c\x7FT\x80q\x14a\x07iW\x80c\x88o\x11\x95\x14a\x07|W\x80c\x8D\xA5\xCB[\x14a\x07\x8FW`\0\x80\xFD[\x80ce\xDA\x12d\x14a\x06\xFAW\x80cmp\xF7\xAE\x14a\x07#W\x80cqP\x18\xA6\x14a\x076W`\0\x80\xFD[\x80c9\xB7\x0E8\x11a\x02\xC3W\x80cP\xF7>|\x11a\x02aW\x80c\\\x97Z\xBB\x11a\x020W\x80c\\\x97Z\xBB\x14a\x06\xA0W\x80c\\\xFE\x8D,\x14a\x06\xA8W\x80c_\x96o\x14\x14a\x06\xBBW\x80c`\xD7\xFA\xED\x14a\x06\xE7W`\0\x80\xFD[\x80cP\xF7>|\x14a\x06YW\x80cY\\jg\x14a\x06bW\x80cY{6\xDA\x14a\x06jW\x80cZ\xC8j\xB7\x14a\x06}W`\0\x80\xFD[\x80cC7s\x82\x11a\x02\x9DW\x80cC7s\x82\x14a\x05\xC6W\x80cFe\xBC\xDA\x14a\x05\xEDW\x80cI\x07]\xA3\x14a\x06\x14W\x80cO\xC4\x0Ba\x14a\x06OW`\0\x80\xFD[\x80c9\xB7\x0E8\x14a\x05EW\x80c<\xDE\xB5\xE0\x14a\x05\x84W\x80c>(9\x1D\x14a\x05\xB3W`\0\x80\xFD[\x80c\x16\x92\x83e\x11a\x030W\x80c(\xA5s\xAE\x11a\x03\nW\x80c(\xA5s\xAE\x14a\x04\xC1W\x80c)\xC7}O\x14a\x04\xD4W\x80c3@C\x96\x14a\x04\xF4W\x80c7H#\xB5\x14a\x05\x07W`\0\x80\xFD[\x80c\x16\x92\x83e\x14a\x04NW\x80c\x1B\xBC\xE0\x91\x14a\x04\x87W\x80c `kp\x14a\x04\x9AW`\0\x80\xFD[\x80c\x0FX\x9EY\x11a\x03lW\x80c\x0FX\x9EY\x14a\x04\0W\x80c\x10\xD6z/\x14a\x04\x15W\x80c\x13-Ig\x14a\x04(W\x80c\x13d9\xDD\x14a\x04;W`\0\x80\xFD[\x80c\x04\xA4\xF9y\x14a\x03\x93W\x80c\x0B\x9FHz\x14a\x03\xCDW\x80c\r\xD8\xDD\x02\x14a\x03\xE0W[`\0\x80\xFD[a\x03\xBA\x7F;\x89\xFC\xA1Q\xCB\xE5\x12-X\xAC\xEE\x86\xCF\x18D\x13\xD7Q\xD5\x85w\x9B\xDD\x19\xD3\xBB\xFA:0m\xCE\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x03\xBAa\x03\xDB6`\x04aIoV[a\n6V[a\x03\xF3a\x03\xEE6`\x04aJ\x0EV[a\n\xF8V[`@Qa\x03\xC4\x91\x90aJOV[a\x04\x13a\x04\x0E6`\x04aJ\xECV[a\x0ErV[\0[a\x04\x13a\x04#6`\x04aK?V[a\x0F\xC2V[a\x04\x13a\x0466`\x04aKcV[a\x10uV[a\x04\x13a\x04I6`\x04aK\xA4V[a\x11,V[a\x03\xBAa\x04\\6`\x04aK?V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x90V[a\x03\xBAa\x04\x956`\x04aKcV[a\x12kV[a\x03\xBA\x7F\x8C\xAD\x95h{\xA8,,\xE5\x0Et\xF7\xB7Td^Q\x17\xC3\xA5\xBE\xC8\x15\x1C\x07&\xD5\x85y\x80\xA8f\x81V[a\x04\x13a\x04\xCF6`\x04aKcV[a\x12\x99V[a\x03\xBAa\x04\xE26`\x04aK?V[`\x9B` R`\0\x90\x81R`@\x90 T\x81V[a\x04\x13a\x05\x026`\x04aK\xBDV[a\x13IV[a\x055a\x05\x156`\x04aL\x80V[`\xA2` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x03\xC4V[a\x05l\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03\xC4V[a\x05la\x05\x926`\x04aK?V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T\x16\x90V[a\x055a\x05\xC16`\x04aK?V[a\x14\x86V[a\x03\xBA\x7F9\x11\x1B\xC4\xA4\xD6\x88\xE1\xF6\x85\x12=t\x97\xD4aSp\x15*\x8E\xE4\xA0Y>d{\xD0j\xD8\xBB\x0B\x81V[a\x05l\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x06Ba\x06\"6`\x04aL\xACV[`\xA1` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[`@Qa\x03\xC4\x91\x90aL\xFBV[a\x03\xBAb\x13\xC6\x80\x81V[a\x03\xBA`\x9DT\x81V[a\x04\x13a\x14\xA6V[a\x03\xBAa\x06x6`\x04aO\xA0V[a\x15mV[a\x055a\x06\x8B6`\x04aO\xDCV[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`fTa\x03\xBAV[a\x04\x13a\x06\xB66`\x04aPRV[a\x15\x9DV[a\x05la\x06\xC96`\x04aK?V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x99` R`@\x90 T\x16\x90V[a\x04\x13a\x06\xF56`\x04aQ\xB2V[a\x18HV[a\x05la\x07\x086`\x04aK?V[`\x9A` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x055a\x0716`\x04aK?V[a\x18\xE3V[a\x04\x13a\x19\x03V[a\x03\xBAa\x07L6`\x04aL\xACV[`\x98` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[a\x04\x13a\x07w6`\x04aS\x01V[a\x19\x17V[`eTa\x05l\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x05lV[a\x05ls\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x81V[a\x04\x13a\x07\xC96`\x04aS\x91V[a\x1A\x1CV[a\x04\x13a\x07\xDC6`\x04aT<V[a\x1D(V[a\x03\xBAa\x07\xEF6`\x04aTqV[a\x1D\xFAV[a\x03\xBAa\x08\x026`\x04aK?V[`\x9F` R`\0\x90\x81R`@\x90 T\x81V[a\x04\x13a\x08\"6`\x04aK?V[a\x1E\xBAV[a\x04\x13a\x0856`\x04aT<V[a \x05V[a\x05l\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x055a\x08o6`\x04aK\xA4V[`\x9E` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x055a\x08\x926`\x04aL\x80V[`\x9C` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[a\t\"a\x08\xC06`\x04aK?V[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x92\x84\x01\x81\x90R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R`\x99\x83R\x83\x90 \x83Q\x91\x82\x01\x84R\x80T\x85\x16\x82R`\x01\x01T\x93\x84\x16\x91\x81\x01\x91\x90\x91R`\x01`\xA0\x1B\x90\x92\x04c\xFF\xFF\xFF\xFF\x16\x90\x82\x01R\x90V[`@\x80Q\x82Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x80\x85\x01Q\x90\x91\x16\x90\x82\x01R\x91\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R``\x01a\x03\xC4V[a\x03\xBAa\tf6`\x04aT\xB7V[a @V[a\x03\xBAa\xC4\xE0\x81V[a\t\x87a\t\x826`\x04aK?V[a \x9FV[`@Qa\x03\xC4\x92\x91\x90aUsV[a\x03\xBA\x7F\xF4\x8B\xD2T\xE3\x0C\xE2\x95=\x18\x7F\xDBN\xF0\xCF\xDD\xDBx*\xB1\xDAU\xBE\xEB\xAE\xF6|\x1F%\x8D%n\x81V[a\x03\xF3a\t\xCA6`\x04aK?V[a$WV[a\x04\x13a\t\xDD6`\x04aTqV[a)\x1BV[a\x04\x13a\t\xF06`\x04aU\x98V[a*OV[a\x04\x13a\n\x036`\x04aU\xF0V[a*[V[a\x04\x13a\n\x166`\x04aK?V[a*\xECV[a\x03\xBAa+bV[a\x04\x13a\n16`\x04aK\xA4V[a+\xA0V[`@\x80Q\x7F;\x89\xFC\xA1Q\xCB\xE5\x12-X\xAC\xEE\x86\xCF\x18D\x13\xD7Q\xD5\x85w\x9B\xDD\x19\xD3\xBB\xFA:0m\xCE` \x80\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x83\x85\x01R\x88\x81\x16``\x84\x01R\x87\x16`\x80\x83\x01R`\xA0\x82\x01\x85\x90R`\xC0\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\xE0\x90\x92\x01\x90\x92R\x80Q\x91\x01 `\0\x90\x81a\n\xB4a+bV[`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x91\x90\x91R`B\x81\x01\x83\x90R`b\x01`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x98\x97PPPPPPPPV[`fT``\x90`\x01\x90`\x02\x90\x81\x16\x14\x15a\x0B-W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B$\x90aV\x0CV[`@Q\x80\x91\x03\x90\xFD[`\0\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0BGWa\x0BGaM#V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0BpW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x84\x81\x10\x15a\x0EiW\x85\x85\x82\x81\x81\x10a\x0B\x90Wa\x0B\x90aVCV[\x90P` \x02\x81\x01\x90a\x0B\xA2\x91\x90aVYV[a\x0B\xB0\x90` \x81\x01\x90aVyV[\x90P\x86\x86\x83\x81\x81\x10a\x0B\xC4Wa\x0B\xC4aVCV[\x90P` \x02\x81\x01\x90a\x0B\xD6\x91\x90aVYV[a\x0B\xE0\x90\x80aVyV[\x90P\x14a\x0CUW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FDelegationManager.queueWithdrawa`D\x82\x01R\x7Fl: input length mismatch\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B$V[`\0\x86\x86\x83\x81\x81\x10a\x0CiWa\x0CiaVCV[\x90P` \x02\x81\x01\x90a\x0C{\x91\x90aVYV[a\x0C\x8C\x90``\x81\x01\x90`@\x01aK?V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\r\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`H`$\x82\x01R\x7FDelegationManager.queueWithdrawa`D\x82\x01R\x7Fl: must provide valid withdrawal`d\x82\x01Rg address`\xC0\x1B`\x84\x82\x01R`\xA4\x01a\x0B$V[3`\0\x81\x81R`\x9A` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90a\x0E9\x90\x82\x89\x89\x86\x81\x81\x10a\rKWa\rKaVCV[\x90P` \x02\x81\x01\x90a\r]\x91\x90aVYV[a\rn\x90``\x81\x01\x90`@\x01aK?V[\x8A\x8A\x87\x81\x81\x10a\r\x80Wa\r\x80aVCV[\x90P` \x02\x81\x01\x90a\r\x92\x91\x90aVYV[a\r\x9C\x90\x80aVyV[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8E\x92P\x8D\x91P\x89\x90P\x81\x81\x10a\r\xE2Wa\r\xE2aVCV[\x90P` \x02\x81\x01\x90a\r\xF4\x91\x90aVYV[a\x0E\x02\x90` \x81\x01\x90aVyV[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa,\xFC\x92PPPV[\x83\x83\x81Q\x81\x10a\x0EKWa\x0EKaVCV[` \x90\x81\x02\x91\x90\x91\x01\x01RP\x80a\x0Ea\x81aV\xD8V[\x91PPa\x0BvV[P\x94\x93PPPPV[3`\0\x90\x81R`\x99` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x0F\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R\x7FDelegationManager.registerAsOper`D\x82\x01R\x7Fator: operator has already regis`d\x82\x01Rd\x1D\x19\\\x99Y`\xDA\x1B`\x84\x82\x01R`\xA4\x01a\x0B$V[a\x0F\x163\x84a1\x1EV[`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01Ra\x0F83\x80\x83`\0a3\xBAV[3`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8E\x84\x85X:#\x10\xD4\x1F|\x82\xB9B}\x0B\xD4\x9B\xADt\xBB\x9C\xFF\x9D4\x02\xA2\x9D\x8F\x9B(\xA0\xE2\x85`@Qa\x0Fq\x91\x90aV\xF3V[`@Q\x80\x91\x03\x90\xA23`\x01`\x01`\xA0\x1B\x03\x16\x7F\x02\xA9\x19\xED\x0E*\xCA\xD1\xDD\x90\xF1~\xF2\xFAJ\xE5F.\xE13\x91p\x03J\x851\xCC\xA4\xB6p\x80\x90\x84\x84`@Qa\x0F\xB4\x92\x91\x90aWEV[`@Q\x80\x91\x03\x90\xA2PPPPV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\x15W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x109\x91\x90aWtV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x10iW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B$\x90aW\x91V[a\x10r\x81a7jV[PV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x10\xD4WP3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14[a\x10\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B$\x90aW\xDBV[a\x10\xF9\x83a\x14\x86V[\x15a\x11'W`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R`\x9A` R`@\x90 T\x16a\x11%\x81\x85\x85\x85a8aV[P[PPPV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11tW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x98\x91\x90aX8V[a\x11\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B$\x90aXUV[`fT\x81\x81\x16\x14a\x12-W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B$V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x9B` R`@\x81 Ta\x12\x90\x85\x82\x86\x86a @V[\x95\x94PPPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x12\xF8WP3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14[a\x13\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B$\x90aW\xDBV[a\x13\x1D\x83a\x14\x86V[\x15a\x11'W`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R`\x9A` R`@\x90 T\x16a\x11%\x81\x85\x85\x85a8\xDCV[`fT`\x02\x90`\x04\x90\x81\x16\x14\x15a\x13rW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B$\x90aV\x0CV[`\x02`\xC9T\x14\x15a\x13\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x0B$V[`\x02`\xC9U`\0[\x88\x81\x10\x15a\x14uWa\x14e\x8A\x8A\x83\x81\x81\x10a\x13\xEAWa\x13\xEAaVCV[\x90P` \x02\x81\x01\x90a\x13\xFC\x91\x90aX\x9DV[\x89\x89\x84\x81\x81\x10a\x14\x0EWa\x14\x0EaVCV[\x90P` \x02\x81\x01\x90a\x14 \x91\x90aVyV[\x89\x89\x86\x81\x81\x10a\x142Wa\x142aVCV[\x90P` \x02\x015\x88\x88\x87\x81\x81\x10a\x14KWa\x14KaVCV[\x90P` \x02\x01` \x81\x01\x90a\x14`\x91\x90aX\xB3V[a9WV[a\x14n\x81aV\xD8V[\x90Pa\x13\xCDV[PP`\x01`\xC9UPPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x9A` R`@\x90 T\x16\x15\x15\x90V[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\xEEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x12\x91\x90aX8V[a\x15.W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B$\x90aXUV[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[`\0\x81`@Q` \x01a\x15\x80\x91\x90aYDV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0[\x81Q\x81\x10\x15a\x18DW`\0\x82\x82\x81Q\x81\x10a\x15\xBDWa\x15\xBDaVCV[` \x02` \x01\x01Q\x90P`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xCD)?o\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x16\x16\x91\x90aYWV[`@\x80Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x164W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16X\x91\x90aZ\x06V[\x91P\x91P\x81\x15a\x186W`@\x80\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x9F` R\x91\x82 \x80T\x91\x92\x82\x91\x90a\x16\x8F\x83aV\xD8V[\x91\x90PUP`\0`@Q\x80`\xE0\x01`@R\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x87`\xA0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x87``\x01Q`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01\x87`\x80\x01Qc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x87`\0\x01Q\x81R` \x01\x87` \x01Q\x81RP\x90P`\0a\x17\x0F\x82a\x15mV[`\0\x81\x81R`\x9E` R`@\x90 T\x90\x91P`\xFF\x16\x15a\x17\xA5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R\x7FDelegationManager.migrateQueuedW`D\x82\x01R\x7Fithdrawals: withdrawal already e`d\x82\x01Rdxists`\xD8\x1B`\x84\x82\x01R`\xA4\x01a\x0B$V[`\0\x81\x81R`\x9E` R`@\x90\x81\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x7F\x90\t\xAB\x15>\x80\x14\xFB\xFB\x02\xF2!\x7F\\\xDEz\xA7\xF9\xADsJ\xE8\\\xA3\xEE?L\xA2\xFD\xD4\x99\xF9\x90a\x17\xF0\x90\x83\x90\x85\x90aZ4V[`@Q\x80\x91\x03\x90\xA1`@\x80Q\x86\x81R` \x81\x01\x83\x90R\x7F\xDC\0u\x8Be\xEE\xF7\x1D\xC3x\x0C\x04\xEB\xE3l\xABk\xDB&l:i\x81\x87\xE2\x9E\x0F\r\xCA\x01&0\x91\x01`@Q\x80\x91\x03\x90\xA1PPPP[\x83`\x01\x01\x93PPPPa\x15\xA0V[PPV[`fT`\x02\x90`\x04\x90\x81\x16\x14\x15a\x18qW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B$\x90aV\x0CV[`\x02`\xC9T\x14\x15a\x18\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x0B$V[`\x02`\xC9Ua\x18\xD6\x86\x86\x86\x86\x86a9WV[PP`\x01`\xC9UPPPPV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x99` R`@\x90 T\x16\x15\x15\x90V[a\x19\x0Ba?\xBFV[a\x19\x15`\0a@\x19V[V[B\x83` \x01Q\x10\x15a\x19\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FDelegationManager.delegateToBySi`D\x82\x01R\x7Fgnature: staker signature expire`d\x82\x01R`\x19`\xFA\x1B`\x84\x82\x01R`\xA4\x01a\x0B$V[`\0`\x9B`\0\x87`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 T\x90P`\0a\x19\xD7\x87\x83\x88\x88` \x01Qa @V[`\x01`\x01`\xA0\x1B\x03\x88\x16`\0\x90\x81R`\x9B` R`@\x90 `\x01\x84\x01\x90U\x85Q\x90\x91Pa\x1A\x07\x90\x88\x90\x83\x90a@kV[a\x1A\x13\x87\x87\x86\x86a3\xBAV[PPPPPPPV[`fT`\x03\x90`\x08\x90\x81\x16\x14\x15a\x1AEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B$\x90aV\x0CV[B\x82`@\x01Q\x10\x15a\x1A\xB9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R`\0\x80Q` a]\x12\x839\x81Q\x91R`D\x82\x01R\x7ForToAVS: operator signature expi`d\x82\x01Rb\x1C\x99Y`\xEA\x1B`\x84\x82\x01R`\xA4\x01a\x0B$V[`\x013`\0\x90\x81R`\xA1` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x88\x16\x84R\x90\x91R\x90 T`\xFF\x16`\x01\x81\x11\x15a\x1A\xF3Wa\x1A\xF3aL\xE5V[\x14\x15a\x1BcW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R`\0\x80Q` a]\x12\x839\x81Q\x91R\x90\x82\x01R\x7ForToAVS: operator already regist`d\x82\x01Rc\x19\\\x99Y`\xE2\x1B`\x84\x82\x01R`\xA4\x01a\x0B$V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\xA2` \x90\x81R`@\x80\x83 \x85\x83\x01Q\x84R\x90\x91R\x90 T`\xFF\x16\x15a\x1B\xEEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`;`$\x82\x01R`\0\x80Q` a]\x12\x839\x81Q\x91R`D\x82\x01R\x7ForToAVS: salt already spent\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B$V[a\x1B\xF7\x83a\x18\xE3V[a\x1CrW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`R`$\x82\x01R`\0\x80Q` a]\x12\x839\x81Q\x91R`D\x82\x01R\x7ForToAVS: operator not registered`d\x82\x01Rq\x08\x1D\x1B\xC8\x11ZY\xD9[\x93\x18^Y\\\x88\x1EY]`r\x1B`\x84\x82\x01R`\xA4\x01a\x0B$V[`\0a\x1C\x88\x843\x85` \x01Q\x86`@\x01Qa\x1D\xFAV[\x90Pa\x1C\x99\x84\x82\x85`\0\x01Qa@kV[3`\0\x81\x81R`\xA1` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x89\x16\x80\x85R\x90\x83R\x81\x84 \x80T`\x01`\xFF\x19\x91\x82\x16\x81\x17\x90\x92U`\xA2\x85R\x83\x86 \x8A\x86\x01Q\x87R\x90\x94R\x93\x82\x90 \x80T\x90\x93\x16\x84\x17\x90\x92UQ\x90\x91\x7F\xF0\x95+\x1Ce'\x1D\x81\x9D9\x98=*\xBB\x04K\x9C\xAC\xE5\x9B\xCCMM\xD3\x89\xF5\x86\xEB\xDC\xB1[A\x91a\x1D\x1A\x91\x90aL\xFBV[`@Q\x80\x91\x03\x90\xA3PPPPV[a\x1D13a\x18\xE3V[a\x1D\xB3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FDelegationManager.updateOperator`D\x82\x01R\x7FMetadataURI: caller must be an o`d\x82\x01Rf82\xB90\xBA7\xB9`\xC9\x1B`\x84\x82\x01R`\xA4\x01a\x0B$V[3`\x01`\x01`\xA0\x1B\x03\x16\x7F\x02\xA9\x19\xED\x0E*\xCA\xD1\xDD\x90\xF1~\xF2\xFAJ\xE5F.\xE13\x91p\x03J\x851\xCC\xA4\xB6p\x80\x90\x83\x83`@Qa\x1D\xEE\x92\x91\x90aWEV[`@Q\x80\x91\x03\x90\xA2PPV[`@\x80Q\x7F\xF4\x8B\xD2T\xE3\x0C\xE2\x95=\x18\x7F\xDBN\xF0\xCF\xDD\xDBx*\xB1\xDAU\xBE\xEB\xAE\xF6|\x1F%\x8D%n` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x80\x87\x16\x92\x82\x01\x92\x90\x92R\x90\x84\x16``\x82\x01R`\x80\x81\x01\x83\x90R`\xA0\x81\x01\x82\x90R`\0\x90\x81\x90`\xC0\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0a\x1Ewa+bV[`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x91\x90\x91R`B\x81\x01\x83\x90R`b\x01`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x97\x96PPPPPPPV[`fT`\x03\x90`\x08\x90\x81\x16\x14\x15a\x1E\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B$\x90aV\x0CV[`\x013`\0\x90\x81R`\xA1` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x84R\x90\x91R\x90 T`\xFF\x16`\x01\x81\x11\x15a\x1F\x1DWa\x1F\x1DaL\xE5V[\x14a\x1F\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FDelegationManager.deregisterOper\x90\x82\x01R\x7FatorFromAVS: operator not regist`d\x82\x01Rc\x19\\\x99Y`\xE2\x1B`\x84\x82\x01R`\xA4\x01a\x0B$V[3`\0\x81\x81R`\xA1` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ\x90\x91\x7F\xF0\x95+\x1Ce'\x1D\x81\x9D9\x98=*\xBB\x04K\x9C\xAC\xE5\x9B\xCCMM\xD3\x89\xF5\x86\xEB\xDC\xB1[A\x91a\x1F\xF9\x91\x90aL\xFBV[`@Q\x80\x91\x03\x90\xA3PPV[3`\x01`\x01`\xA0\x1B\x03\x16\x7F\xA8\x9C\x1D\xC2C\xD8\x90\x8A\x96\xDD\x84\x94K\xCC\x97\xD6\xBCj\xC0\r\xD7\x8E b\x15v\xBEj<\x947\x13\x83\x83`@Qa\x1D\xEE\x92\x91\x90aWEV[`@\x80Q\x7F9\x11\x1B\xC4\xA4\xD6\x88\xE1\xF6\x85\x12=t\x97\xD4aSp\x15*\x8E\xE4\xA0Y>d{\xD0j\xD8\xBB\x0B` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x80\x87\x16\x92\x82\x01\x92\x90\x92R\x90\x83\x16``\x82\x01R`\x80\x81\x01\x84\x90R`\xA0\x81\x01\x82\x90R`\0\x90\x81\x90`\xC0\x01a\x1EUV[`@Qc`\xF4\x06+`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R``\x91\x82\x91`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c`\xF4\x06+\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!\x0FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!3\x91\x90aZMV[`@Qc\x94\xF6I\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R\x91\x92P`\0\x91\x82\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\x94\xF6I\xDD\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!\xA3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra!\xCB\x91\x90\x81\x01\x90aZ\xC1V[\x91P\x91P`\0\x83\x13a!\xE2W\x90\x95\x90\x94P\x92PPPV[``\x80\x83Q`\0\x14\x15a\"\x9CW`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x90` \x80\x83\x01\x90\x806\x837PP`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x92\x94P\x90P` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90Ps\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x82`\0\x81Q\x81\x10a\"WWa\"WaVCV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x84\x81`\0\x81Q\x81\x10a\"\x8BWa\"\x8BaVCV[` \x02` \x01\x01\x81\x81RPPa$JV[\x83Qa\"\xA9\x90`\x01a[\x85V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\xC0Wa\"\xC0aM#V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\"\xE9W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a#\x05Wa#\x05aM#V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a#.W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x84Q\x81\x10\x15a#\xC8W\x84\x81\x81Q\x81\x10a#OWa#OaVCV[` \x02` \x01\x01Q\x83\x82\x81Q\x81\x10a#iWa#iaVCV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x83\x81\x81Q\x81\x10a#\x9BWa#\x9BaVCV[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a#\xB5Wa#\xB5aVCV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a#4V[Ps\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x82`\x01\x84Qa#\xED\x91\x90a[\x9DV[\x81Q\x81\x10a#\xFDWa#\xFDaVCV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x84\x81`\x01\x84Qa$-\x91\x90a[\x9DV[\x81Q\x81\x10a$=Wa$=aVCV[` \x02` \x01\x01\x81\x81RPP[\x90\x97\x90\x96P\x94PPPPPV[`fT``\x90`\x01\x90`\x02\x90\x81\x16\x14\x15a$\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B$\x90aV\x0CV[a$\x8C\x83a\x14\x86V[a%\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FDelegationManager.undelegate: st\x90\x82\x01R\x7Faker must be delegated to undele`d\x82\x01Rcgate`\xE0\x1B`\x84\x82\x01R`\xA4\x01a\x0B$V[a%\x15\x83a\x18\xE3V[\x15a%\x88W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FDelegationManager.undelegate: op`D\x82\x01R\x7Ferators cannot be undelegated\0\0\0`d\x82\x01R`\x84\x01a\x0B$V[`\x01`\x01`\xA0\x1B\x03\x83\x16a&\x04W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R\x7FDelegationManager.undelegate: ca`D\x82\x01R\x7Fnnot undelegate zero address\0\0\0\0`d\x82\x01R`\x84\x01a\x0B$V[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x81\x81R`\x9A` R`@\x90 T\x90\x91\x16\x903\x14\x80a&7WP3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14[\x80a&^WP`\x01`\x01`\xA0\x1B\x03\x81\x81\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T\x163\x14[a&\xD0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FDelegationManager.undelegate: ca`D\x82\x01R\x7Fller cannot undelegate staker\0\0\0`d\x82\x01R`\x84\x01a\x0B$V[`\0\x80a&\xDC\x86a \x9FV[\x90\x92P\x90P3`\x01`\x01`\xA0\x1B\x03\x87\x16\x14a'2W\x82`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x7F\xF0\xED\xDF\x07\xE6\xEA\x14\xF3\x88\xB4~\x1E\x94\xA0\xF4d\xEC\xBD\x9E\xEDAq\x13\x0E\x0F\xC0\xE9\x9F\xB4\x03\n\x8A`@Q`@Q\x80\x91\x03\x90\xA3[\x82`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFE\xE3\tf\xA2V\xB7\x1E\x14\xBC\x0E\xBF\xC9C\x15\xE2\x8E\xF4\xA9zq1\xA9\xE2\xB7\xA3\x10\xA7:\xF4Fv`@Q`@Q\x80\x91\x03\x90\xA3`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x9A` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x81Qa'\xB4W`@\x80Q`\0\x81R` \x81\x01\x90\x91R\x94Pa)\x12V[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a'\xCDWa'\xCDaM#V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a'\xF6W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x94P`\0[\x82Q\x81\x10\x15a)\x10W`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91` \x80\x83\x01\x90\x806\x837PP`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x92\x93P`\0\x92\x91P` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x84\x83\x81Q\x81\x10a(\\Wa(\\aVCV[` \x02` \x01\x01Q\x82`\0\x81Q\x81\x10a(wWa(waVCV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x83\x83\x81Q\x81\x10a(\xA9Wa(\xA9aVCV[` \x02` \x01\x01Q\x81`\0\x81Q\x81\x10a(\xC4Wa(\xC4aVCV[` \x02` \x01\x01\x81\x81RPPa(\xDD\x89\x87\x8B\x85\x85a,\xFCV[\x88\x84\x81Q\x81\x10a(\xEFWa(\xEFaVCV[` \x02` \x01\x01\x81\x81RPPPP\x80\x80a)\x08\x90aV\xD8V[\x91PPa'\xFCV[P[PPPP\x91\x90PV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a);WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a)UWP0;\x15\x80\x15a)UWP`\0T`\xFF\x16`\x01\x14[a)\xB8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x0B$V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a)\xDBW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a)\xE5\x84\x84aB%V[a)\xEDaC\x0BV[`\x97Ua)\xF9\x85a@\x19V[a*\x02\x82aC\xA2V[\x80\x15a*HW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPV[a\x11'3\x84\x84\x84a3\xBAV[a*d3a\x18\xE3V[a*\xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FDelegationManager.modifyOperator`D\x82\x01R\x7FDetails: caller must be an opera`d\x82\x01Rb:7\xB9`\xE9\x1B`\x84\x82\x01R`\xA4\x01a\x0B$V[a\x10r3\x82a1\x1EV[a*\xF4a?\xBFV[`\x01`\x01`\xA0\x1B\x03\x81\x16a+YW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x0B$V[a\x10r\x81a@\x19V[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14\x15a+\x93WP`\x97T\x90V[a+\x9BaC\x0BV[\x90P\x90V[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+\xF3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\x17\x91\x90aWtV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a,GW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B$\x90aW\x91V[`fT\x19\x81\x19`fT\x19\x16\x14a,\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B$V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x12`V[`\0`\x01`\x01`\xA0\x1B\x03\x86\x16a-\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`P`$\x82\x01R\x7FDelegationManager._removeSharesA`D\x82\x01R\x7FndQueueWithdrawal: staker cannot`d\x82\x01Ro be zero address`\x80\x1B`\x84\x82\x01R`\xA4\x01a\x0B$V[\x82Qa.\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`M`$\x82\x01R\x7FDelegationManager._removeSharesA`D\x82\x01R\x7FndQueueWithdrawal: strategies ca`d\x82\x01Rlnnot be empty`\x98\x1B`\x84\x82\x01R`\xA4\x01a\x0B$V[`\0[\x83Q\x81\x10\x15a0,W`\x01`\x01`\xA0\x1B\x03\x86\x16\x15a.vWa.v\x86\x88\x86\x84\x81Q\x81\x10a.OWa.OaVCV[` \x02` \x01\x01Q\x86\x85\x81Q\x81\x10a.iWa.iaVCV[` \x02` \x01\x01Qa8aV[s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0`\x01`\x01`\xA0\x1B\x03\x16\x84\x82\x81Q\x81\x10a.\xA6Wa.\xA6aVCV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a/oW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xBE\xFF\xBB\x89\x88\x85\x84\x81Q\x81\x10a.\xFFWa.\xFFaVCV[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a/8\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a/RW`\0\x80\xFD[PZ\xF1\x15\x80\x15a/fW=`\0\x80>=`\0\xFD[PPPPa0$V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8C\x80\xD4\xE5\x88\x86\x84\x81Q\x81\x10a/\xB1Wa/\xB1aVCV[` \x02` \x01\x01Q\x86\x85\x81Q\x81\x10a/\xCBWa/\xCBaVCV[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a/\xF1\x93\x92\x91\x90a[\xB4V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a0\x0BW`\0\x80\xFD[PZ\xF1\x15\x80\x15a0\x1FW=`\0\x80>=`\0\xFD[PPPP[`\x01\x01a. V[P`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x9F` R`@\x81 \x80T\x91\x82\x91\x90a0T\x83aV\xD8V[\x91\x90PUP`\0`@Q\x80`\xE0\x01`@R\x80\x89`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x88`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x87`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86\x81R` \x01\x85\x81RP\x90P`\0a0\xBC\x82a\x15mV[`\0\x81\x81R`\x9E` R`@\x90\x81\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x90\x91P\x7F\x90\t\xAB\x15>\x80\x14\xFB\xFB\x02\xF2!\x7F\\\xDEz\xA7\xF9\xADsJ\xE8\\\xA3\xEE?L\xA2\xFD\xD4\x99\xF9\x90a1\n\x90\x83\x90\x85\x90aZ4V[`@Q\x80\x91\x03\x90\xA1\x98\x97PPPPPPPPV[`\0a1-` \x83\x01\x83aK?V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a1\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`T`$\x82\x01R\x7FDelegationManager._setOperatorDe`D\x82\x01R\x7Ftails: cannot set `earningsRecei`d\x82\x01Rsver` to zero address``\x1B`\x84\x82\x01R`\xA4\x01a\x0B$V[b\x13\xC6\x80a1\xDB``\x83\x01`@\x84\x01a[\xD8V[c\xFF\xFF\xFF\xFF\x16\x11\x15a2\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`l`$\x82\x01R\x7FDelegationManager._setOperatorDe`D\x82\x01R\x7Ftails: stakerOptOutWindowBlocks `d\x82\x01R\x7Fcannot be > MAX_STAKER_OPT_OUT_W`\x84\x82\x01RkINDOW_BLOCKS`\xA0\x1B`\xA4\x82\x01R`\xC4\x01a\x0B$V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x99` R`@\x90\x81\x90 `\x01\x01T`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x90a2\xCC\x90``\x84\x01\x90\x84\x01a[\xD8V[c\xFF\xFF\xFF\xFF\x16\x10\x15a3bW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`S`$\x82\x01R\x7FDelegationManager._setOperatorDe`D\x82\x01R\x7Ftails: stakerOptOutWindowBlocks `d\x82\x01Rr\x18\xD8[\x9B\x9B\xDD\x08\x18\x99H\x19\x19X\xDC\x99X\\\xD9Y`j\x1B`\x84\x82\x01R`\xA4\x01a\x0B$V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x99` R`@\x90 \x81\x90a3\x86\x82\x82a\\\x15V[PP`@Q3\x90\x7F\xFE\xBE\\\xD2K,\xBC{\x06[\x9D\x0F\xDE\xB9\x04F\x1EJ\xFC\xFFW\xDDW\xAC\xDA\x1Ex2\x03\x1B\xA7\xAC\x90a\x1D\xEE\x90\x84\x90aV\xF3V[`fT`\0\x90`\x01\x90\x81\x16\x14\x15a3\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B$\x90aV\x0CV[a3\xEC\x85a\x14\x86V[\x15a4iW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FDelegationManager._delegate: sta`D\x82\x01R\x7Fker is already actively delegate`d\x82\x01R`\x19`\xFA\x1B`\x84\x82\x01R`\xA4\x01a\x0B$V[a4r\x84a\x18\xE3V[a4\xF2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R\x7FDelegationManager._delegate: ope`D\x82\x01R\x7Frator is not registered in Eigen`d\x82\x01Rd&0\xBC\xB2\xB9`\xD9\x1B`\x84\x82\x01R`\xA4\x01a\x0B$V[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T\x16\x80\x15\x80\x15\x90a5(WP3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14\x15[\x80\x15a5=WP3`\x01`\x01`\xA0\x1B\x03\x86\x16\x14\x15[\x15a6\xAAWB\x84` \x01Q\x10\x15a5\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FDelegationManager._delegate: app`D\x82\x01R\x7Frover signature expired\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B$V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x9C` \x90\x81R`@\x80\x83 \x86\x84R\x90\x91R\x90 T`\xFF\x16\x15a6VW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FDelegationManager._delegate: app`D\x82\x01R\x7FroverSalt already spent\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B$V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x9C` \x90\x81R`@\x80\x83 \x86\x84R\x82R\x82 \x80T`\xFF\x19\x16`\x01\x17\x90U\x85\x01Qa6\x97\x90\x88\x90\x88\x90\x85\x90\x88\x90a\n6V[\x90Pa6\xA8\x82\x82\x87`\0\x01Qa@kV[P[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\0\x81\x81R`\x9A` R`@\x80\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x94\x8A\x16\x94\x85\x17\x90UQ\x7F\xC3\xEE\x9F._\xDA\x98\xE8\x06j\x1Ft[-\xF9(_Ao\xE9\x8C\xF2U\x9C\xD2\x14\x84\xB3\xD8t3\x04\x91\x90\xA3`\0\x80a7\t\x88a \x9FV[\x91P\x91P`\0[\x82Q\x81\x10\x15a7_Wa7W\x88\x8A\x85\x84\x81Q\x81\x10a70Wa70aVCV[` \x02` \x01\x01Q\x85\x85\x81Q\x81\x10a7JWa7JaVCV[` \x02` \x01\x01Qa8\xDCV[`\x01\x01a7\x10V[PPPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16a7\xF8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x0B$V[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x98` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a8\x98\x90\x84\x90a[\x9DV[\x92PP\x81\x90UP\x83`\x01`\x01`\xA0\x1B\x03\x16\x7Fi\t`\x007\xB7]{G3\xAE\xDD\x81TB\xB5\xEC\x01\x8A\x82wQ\xC82\xAA\xFFd\xEB\xA5\xD6\xD2\xDD\x84\x84\x84`@Qa\x0F\xB4\x93\x92\x91\x90a[\xB4V[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x98` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a9\x13\x90\x84\x90a[\x85V[\x92PP\x81\x90UP\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\x1E\xC0B\xC9e\xE2\xED\xD7\x10{Q\x18\x8E\xE0\xF3\x83\xE2.v\x17\x90A\xAB:\x9D\x18\xFF\x15\x14\x05\x16l\x84\x84\x84`@Qa\x0F\xB4\x93\x92\x91\x90a[\xB4V[`\0a9ea\x06x\x87a\\xV[`\0\x81\x81R`\x9E` R`@\x90 T\x90\x91P`\xFF\x16a9\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R`\0\x80Q` a]2\x839\x81Q\x91R`D\x82\x01R\x7FAction: action is not in queue\0\0`d\x82\x01R`\x84\x01a\x0B$V[`\x9DTC\x90a9\xEF`\xA0\x89\x01`\x80\x8A\x01a[\xD8V[c\xFF\xFF\xFF\xFF\x16a9\xFF\x91\x90a[\x85V[\x11\x15a:\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`W`$\x82\x01R`\0\x80Q` a]2\x839\x81Q\x91R`D\x82\x01R\x7FAction: withdrawalDelayBlocks pe`d\x82\x01R\x7Friod has not yet passed\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x0B$V[a:\x97``\x87\x01`@\x88\x01aK?V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a;\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`K`$\x82\x01R`\0\x80Q` a]2\x839\x81Q\x91R`D\x82\x01R\x7FAction: only withdrawer can comp`d\x82\x01Rj62\xBA2\x900\xB1\xBA4\xB7\xB7`\xA9\x1B`\x84\x82\x01R`\xA4\x01a\x0B$V[\x81\x15a;\x96Wa;2`\xA0\x87\x01\x87aVyV[\x85\x14\x90Pa;\x96W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R`\0\x80Q` a]2\x839\x81Q\x91R`D\x82\x01R\x7FAction: input length mismatch\0\0\0`d\x82\x01R`\x84\x01a\x0B$V[`\0\x81\x81R`\x9E` R`@\x90 \x80T`\xFF\x19\x16\x90U\x81\x15a<lW`\0[a;\xC2`\xA0\x88\x01\x88aVyV[\x90P\x81\x10\x15a<fWa<^a;\xDB` \x89\x01\x89aK?V[3a;\xE9`\xA0\x8B\x01\x8BaVyV[\x85\x81\x81\x10a;\xF9Wa;\xF9aVCV[\x90P` \x02\x01` \x81\x01\x90a<\x0E\x91\x90aK?V[a<\x1B`\xC0\x8C\x01\x8CaVyV[\x86\x81\x81\x10a<+Wa<+aVCV[\x90P` \x02\x015\x8A\x8A\x87\x81\x81\x10a<DWa<DaVCV[\x90P` \x02\x01` \x81\x01\x90a<Y\x91\x90aK?V[aD\x9CV[`\x01\x01a;\xB5V[Pa?\x84V[3`\0\x90\x81R`\x9A` R`@\x81 T`\x01`\x01`\xA0\x1B\x03\x16\x90[a<\x94`\xA0\x89\x01\x89aVyV[\x90P\x81\x10\x15a?\x81Ws\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0a<\xBF`\xA0\x8A\x01\x8AaVyV[\x83\x81\x81\x10a<\xCFWa<\xCFaVCV[\x90P` \x02\x01` \x81\x01\x90a<\xE4\x91\x90aK?V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a>4W`\0a=\x02` \x8A\x01\x8AaK?V[\x90P`\0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\x0E\x81\x07<\x83a=C`\xC0\x8E\x01\x8EaVyV[\x87\x81\x81\x10a=SWa=SaVCV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x87\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x94\x16`\x04\x85\x01R` \x02\x91\x90\x91\x015`$\x83\x01RP`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a=\xA7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a=\xCB\x91\x90aZMV[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R`\x9A` R`@\x90 T\x91\x92P\x16\x80\x15a>,Wa>,\x81\x84a>\x01`\xA0\x8F\x01\x8FaVyV[\x88\x81\x81\x10a>\x11Wa>\x11aVCV[\x90P` \x02\x01` \x81\x01\x90a>&\x91\x90aK?V[\x85a8\xDCV[PPPa?yV[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16cP\xFFr%3a>q`\xA0\x8C\x01\x8CaVyV[\x85\x81\x81\x10a>\x81Wa>\x81aVCV[\x90P` \x02\x01` \x81\x01\x90a>\x96\x91\x90aK?V[a>\xA3`\xC0\x8D\x01\x8DaVyV[\x86\x81\x81\x10a>\xB3Wa>\xB3aVCV[\x90P` \x02\x015`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a>\xD8\x93\x92\x91\x90a[\xB4V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a>\xF2W`\0\x80\xFD[PZ\xF1\x15\x80\x15a?\x06W=`\0\x80>=`\0\xFD[PPPP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a?yWa?y\x823a?+`\xA0\x8C\x01\x8CaVyV[\x85\x81\x81\x10a?;Wa?;aVCV[\x90P` \x02\x01` \x81\x01\x90a?P\x91\x90aK?V[a?]`\xC0\x8D\x01\x8DaVyV[\x86\x81\x81\x10a?mWa?maVCV[\x90P` \x02\x015a8\xDCV[`\x01\x01a<\x87V[PP[`@Q\x81\x81R\x7F\xC9p\x98\xC2\xF6X\x80\x0BM\xF2\x90\x01R\x7Fs$\xBC\xDF\xFC\xF6\xE8u\x1Ai\x9A\xB9 \xA1\xEC\xED[\x1D\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPPPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x19\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x0B$V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16;\x15aA\x85W`@Qc\x0B\x13]?`\xE1\x1B\x80\x82R\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\x16&\xBA~\x90a@\xAB\x90\x86\x90\x86\x90`\x04\x01a\\\x8AV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a@\xC8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a@\xEC\x91\x90a\\\xE7V[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14a\x11'W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`S`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: ERC1271 signature `d\x82\x01Rr\x1D\x99\\\x9AY\x9AX\xD8]\x1A[\xDB\x88\x19\x98Z[\x19Y`j\x1B`\x84\x82\x01R`\xA4\x01a\x0B$V[\x82`\x01`\x01`\xA0\x1B\x03\x16aA\x99\x83\x83aE\xD5V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x11'W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: signature not from`d\x82\x01Rf\x109\xB4\xB3\xB72\xB9`\xC9\x1B`\x84\x82\x01R`\xA4\x01a\x0B$V[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15aBFWP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[aB\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x0B$V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a\x18D\x82a7jV[`@\x80Q\x80\x82\x01\x82R`\n\x81Ri\"\xB4\xB3\xB2\xB7&0\xBC\xB2\xB9`\xB1\x1B` \x91\x82\x01R\x81Q\x7F\x8C\xAD\x95h{\xA8,,\xE5\x0Et\xF7\xB7Td^Q\x17\xC3\xA5\xBE\xC8\x15\x1C\x07&\xD5\x85y\x80\xA8f\x81\x83\x01R\x7Fq\xB6%\xCF\xADD\xBA\xC6;\x13\xDB\xA0\x7F.\x1D`\x84\xEE\x04\xB6\xF8u!\x01\xEC\xE6\x12mXN\xE6\xEA\x81\x84\x01RF``\x82\x01R0`\x80\x80\x83\x01\x91\x90\x91R\x83Q\x80\x83\x03\x90\x91\x01\x81R`\xA0\x90\x91\x01\x90\x92R\x81Q\x91\x01 \x90V[a\xC4\xE0\x81\x11\x15aD[W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`r`$\x82\x01R\x7FDelegationManager._initializeWit`D\x82\x01R\x7FhdrawalDelayBlocks: _withdrawalD`d\x82\x01R\x7FelayBlocks cannot be > MAX_WITHD`\x84\x82\x01RqRAWAL_DELAY_BLOCKS`p\x1B`\xA4\x82\x01R`\xC4\x01a\x0B$V[`\x9DT`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7FO\xFB\0@\x05t\x14t)\xEE7zV38c!\xE6mE\xD8\xB1Fv\x01K_\xA3\x93\xE6\x1E\x9E\x91\x01`@Q\x80\x91\x03\x90\xA1`\x9DUV[`\x01`\x01`\xA0\x1B\x03\x83\x16s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x14\x15aEGW`@Qb8{\x13`\xE8\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c8{\x13\0\x90aE\x10\x90\x88\x90\x88\x90\x87\x90`\x04\x01a[\xB4V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aE*W`\0\x80\xFD[PZ\xF1\x15\x80\x15aE>W=`\0\x80>=`\0\xFD[PPPPa*HV[`@Qc\xC6\x08\xC7\xF3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x84\x81\x16`$\x83\x01R`D\x82\x01\x84\x90R\x82\x81\x16`d\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xC6\x08\xC7\xF3\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aE\xC1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a7_W=`\0\x80>=`\0\xFD[`\0\x80`\0aE\xE4\x85\x85aE\xF9V[\x91P\x91PaE\xF1\x81aFiV[P\x93\x92PPPV[`\0\x80\x82Q`A\x14\x15aF0W` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1AaF$\x87\x82\x85\x85aH$V[\x94P\x94PPPPaFbV[\x82Q`@\x14\x15aFZW` \x83\x01Q`@\x84\x01QaFO\x86\x83\x83aI\x11V[\x93P\x93PPPaFbV[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0\x81`\x04\x81\x11\x15aF}WaF}aL\xE5V[\x14\x15aF\x86WPV[`\x01\x81`\x04\x81\x11\x15aF\x9AWaF\x9AaL\xE5V[\x14\x15aF\xE8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x0B$V[`\x02\x81`\x04\x81\x11\x15aF\xFCWaF\xFCaL\xE5V[\x14\x15aGJW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x0B$V[`\x03\x81`\x04\x81\x11\x15aG^WaG^aL\xE5V[\x14\x15aG\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x0B$V[`\x04\x81`\x04\x81\x11\x15aG\xCBWaG\xCBaL\xE5V[\x14\x15a\x10rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 'v' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x0B$V[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15aH[WP`\0\x90P`\x03aI\x08V[\x84`\xFF\x16`\x1B\x14\x15\x80\x15aHsWP\x84`\xFF\x16`\x1C\x14\x15[\x15aH\x84WP`\0\x90P`\x04aI\x08V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15aH\xD8W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16aI\x01W`\0`\x01\x92P\x92PPaI\x08V[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x80`\x01`\x01`\xFF\x1B\x03\x83\x16\x81aI.`\xFF\x86\x90\x1C`\x1Ba[\x85V[\x90PaI<\x87\x82\x88\x85aH$V[\x93P\x93PPP\x93P\x93\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x10rW`\0\x80\xFD[\x805aIj\x81aIJV[\x91\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15aI\x87W`\0\x80\xFD[\x855aI\x92\x81aIJV[\x94P` \x86\x015aI\xA2\x81aIJV[\x93P`@\x86\x015aI\xB2\x81aIJV[\x94\x97\x93\x96P\x93\x94``\x81\x015\x94P`\x80\x015\x92\x91PPV[`\0\x80\x83`\x1F\x84\x01\x12aI\xDCW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aI\xF3W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15aFbW`\0\x80\xFD[`\0\x80` \x83\x85\x03\x12\x15aJ!W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aJ7W`\0\x80\xFD[aJC\x85\x82\x86\x01aI\xCAV[\x90\x96\x90\x95P\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aJ\x87W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aJkV[P\x90\x96\x95PPPPPPV[`\0``\x82\x84\x03\x12\x15aJ\xA5W`\0\x80\xFD[P\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12aJ\xBDW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aJ\xD4W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aFbW`\0\x80\xFD[`\0\x80`\0`\x80\x84\x86\x03\x12\x15aK\x01W`\0\x80\xFD[aK\x0B\x85\x85aJ\x93V[\x92P``\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aK&W`\0\x80\xFD[aK2\x86\x82\x87\x01aJ\xABV[\x94\x97\x90\x96P\x93\x94PPPPV[`\0` \x82\x84\x03\x12\x15aKQW`\0\x80\xFD[\x815aK\\\x81aIJV[\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15aKxW`\0\x80\xFD[\x835aK\x83\x81aIJV[\x92P` \x84\x015aK\x93\x81aIJV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0` \x82\x84\x03\x12\x15aK\xB6W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80`\x80\x89\x8B\x03\x12\x15aK\xD9W`\0\x80\xFD[\x885`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aK\xF0W`\0\x80\xFD[aK\xFC\x8C\x83\x8D\x01aI\xCAV[\x90\x9AP\x98P` \x8B\x015\x91P\x80\x82\x11\x15aL\x15W`\0\x80\xFD[aL!\x8C\x83\x8D\x01aI\xCAV[\x90\x98P\x96P`@\x8B\x015\x91P\x80\x82\x11\x15aL:W`\0\x80\xFD[aLF\x8C\x83\x8D\x01aI\xCAV[\x90\x96P\x94P``\x8B\x015\x91P\x80\x82\x11\x15aL_W`\0\x80\xFD[PaLl\x8B\x82\x8C\x01aI\xCAV[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[`\0\x80`@\x83\x85\x03\x12\x15aL\x93W`\0\x80\xFD[\x825aL\x9E\x81aIJV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`@\x83\x85\x03\x12\x15aL\xBFW`\0\x80\xFD[\x825aL\xCA\x81aIJV[\x91P` \x83\x015aL\xDA\x81aIJV[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x02\x83\x10aM\x1DWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aM[WaM[aM#V[`@R\x90V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aM[WaM[aM#V[`@Q`\xC0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aM[WaM[aM#V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aM\xCDWaM\xCDaM#V[`@R\x91\x90PV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x10rW`\0\x80\xFD[\x805aIj\x81aM\xD5V[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15aN\x0BWaN\x0BaM#V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12aN&W`\0\x80\xFD[\x815` aN;aN6\x83aM\xF2V[aM\xA5V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aNZW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aN~W\x805aNq\x81aIJV[\x83R\x91\x83\x01\x91\x83\x01aN^V[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12aN\x9AW`\0\x80\xFD[\x815` aN\xAAaN6\x83aM\xF2V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aN\xC9W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aN~W\x805\x83R\x91\x83\x01\x91\x83\x01aN\xCDV[`\0`\xE0\x82\x84\x03\x12\x15aN\xF6W`\0\x80\xFD[aN\xFEaM9V[\x90PaO\t\x82aI_V[\x81RaO\x17` \x83\x01aI_V[` \x82\x01RaO(`@\x83\x01aI_V[`@\x82\x01R``\x82\x015``\x82\x01RaOC`\x80\x83\x01aM\xE7V[`\x80\x82\x01R`\xA0\x82\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aObW`\0\x80\xFD[aOn\x85\x83\x86\x01aN\x15V[`\xA0\x84\x01R`\xC0\x84\x015\x91P\x80\x82\x11\x15aO\x87W`\0\x80\xFD[PaO\x94\x84\x82\x85\x01aN\x89V[`\xC0\x83\x01RP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15aO\xB2W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aO\xC8W`\0\x80\xFD[aO\xD4\x84\x82\x85\x01aN\xE4V[\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15aO\xEEW`\0\x80\xFD[\x815`\xFF\x81\x16\x81\x14aK\\W`\0\x80\xFD[`\0`@\x82\x84\x03\x12\x15aP\x11W`\0\x80\xFD[aP\x19aMaV[\x90P\x815aP&\x81aIJV[\x81R` \x82\x015k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14aPGW`\0\x80\xFD[` \x82\x01R\x92\x91PPV[`\0` \x80\x83\x85\x03\x12\x15aPeW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aP|W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12aP\x90W`\0\x80\xFD[\x815aP\x9EaN6\x82aM\xF2V[\x81\x81R`\x05\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x84\x81\x01\x90\x88\x83\x11\x15aP\xBDW`\0\x80\xFD[\x85\x85\x01[\x83\x81\x10\x15aQ\x97W\x805\x85\x81\x11\x15aP\xD9W`\0\x80\x81\xFD[\x86\x01`\xE0\x81\x8C\x03`\x1F\x19\x01\x81\x13\x15aP\xF1W`\0\x80\x81\xFD[aP\xF9aM\x83V[\x89\x83\x015\x88\x81\x11\x15aQ\x0BW`\0\x80\x81\xFD[aQ\x19\x8E\x8C\x83\x87\x01\x01aN\x15V[\x82RP`@\x80\x84\x015\x89\x81\x11\x15aQ0W`\0\x80\x81\xFD[aQ>\x8F\x8D\x83\x88\x01\x01aN\x89V[\x8C\x84\x01RP``aQP\x81\x86\x01aI_V[\x82\x84\x01R`\x80\x91PaQd\x8F\x83\x87\x01aO\xFFV[\x90\x83\x01RaQt`\xC0\x85\x01aM\xE7V[\x90\x82\x01RaQ\x83\x83\x83\x01aI_V[`\xA0\x82\x01R\x85RPP\x91\x86\x01\x91\x86\x01aP\xC1V[P\x98\x97PPPPPPPPV[\x80\x15\x15\x81\x14a\x10rW`\0\x80\xFD[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15aQ\xCAW`\0\x80\xFD[\x855`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aQ\xE1W`\0\x80\xFD[\x90\x87\x01\x90`\xE0\x82\x8A\x03\x12\x15aQ\xF5W`\0\x80\xFD[\x90\x95P` \x87\x015\x90\x80\x82\x11\x15aR\x0BW`\0\x80\xFD[PaR\x18\x88\x82\x89\x01aI\xCAV[\x90\x95P\x93PP`@\x86\x015\x91P``\x86\x015aR3\x81aQ\xA4V[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x82`\x1F\x83\x01\x12aRRW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aRkWaRkaM#V[aR~`\x1F\x82\x01`\x1F\x19\x16` \x01aM\xA5V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15aR\x93W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0`@\x82\x84\x03\x12\x15aR\xC2W`\0\x80\xFD[aR\xCAaMaV[\x90P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aR\xE2W`\0\x80\xFD[aR\xEE\x84\x82\x85\x01aRAV[\x82RP` \x82\x015` \x82\x01R\x92\x91PPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15aS\x19W`\0\x80\xFD[\x855aS$\x81aIJV[\x94P` \x86\x015aS4\x81aIJV[\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aSPW`\0\x80\xFD[aS\\\x89\x83\x8A\x01aR\xB0V[\x94P``\x88\x015\x91P\x80\x82\x11\x15aSrW`\0\x80\xFD[PaS\x7F\x88\x82\x89\x01aR\xB0V[\x95\x98\x94\x97P\x92\x95`\x80\x015\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15aS\xA4W`\0\x80\xFD[\x825aS\xAF\x81aIJV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aS\xCBW`\0\x80\xFD[\x90\x84\x01\x90``\x82\x87\x03\x12\x15aS\xDFW`\0\x80\xFD[`@Q``\x81\x01\x81\x81\x10\x83\x82\x11\x17\x15aS\xFAWaS\xFAaM#V[`@R\x825\x82\x81\x11\x15aT\x0CW`\0\x80\xFD[aT\x18\x88\x82\x86\x01aRAV[\x82RP` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R\x80\x93PPPP\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15aTOW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aTeW`\0\x80\xFD[aJC\x85\x82\x86\x01aJ\xABV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aT\x87W`\0\x80\xFD[\x845aT\x92\x81aIJV[\x93P` \x85\x015aT\xA2\x81aIJV[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aT\xCDW`\0\x80\xFD[\x845aT\xD8\x81aIJV[\x93P` \x85\x015\x92P`@\x85\x015aT\xEF\x81aIJV[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aU8W\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aU\x13V[P\x94\x95\x94PPPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aU8W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aUWV[`@\x81R`\0aU\x86`@\x83\x01\x85aT\xFFV[\x82\x81\x03` \x84\x01Ra\x12\x90\x81\x85aUCV[`\0\x80`\0``\x84\x86\x03\x12\x15aU\xADW`\0\x80\xFD[\x835aU\xB8\x81aIJV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aU\xD3W`\0\x80\xFD[aU\xDF\x86\x82\x87\x01aR\xB0V[\x92PP`@\x84\x015\x90P\x92P\x92P\x92V[`\0``\x82\x84\x03\x12\x15aV\x02W`\0\x80\xFD[aK\\\x83\x83aJ\x93V[` \x80\x82R`\x19\x90\x82\x01R\x7FPausable: index is paused\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x825`^\x19\x836\x03\x01\x81\x12aVoW`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aV\x90W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aV\xAAW`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15aFbW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15aV\xECWaV\xECaV\xC2V[P`\x01\x01\x90V[``\x81\x01\x825aW\x02\x81aIJV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x84\x015\x90aW\x1E\x82aIJV[\x16` \x83\x01R`@\x83\x015aW2\x81aM\xD5V[c\xFF\xFF\xFF\xFF\x81\x16`@\x84\x01RP\x92\x91PPV[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017`\0\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x91\x90PV[`\0` \x82\x84\x03\x12\x15aW\x86W`\0\x80\xFD[\x81QaK\\\x81aIJV[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`7\x90\x82\x01R\x7FDelegationManager: onlyStrategyM`@\x82\x01R\x7FanagerOrEigenPodManager\0\0\0\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15aXJW`\0\x80\xFD[\x81QaK\\\x81aQ\xA4V[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[`\0\x825`\xDE\x19\x836\x03\x01\x81\x12aVoW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aX\xC5W`\0\x80\xFD[\x815aK\\\x81aQ\xA4V[`\0`\x01\x80`\xA0\x1B\x03\x80\x83Q\x16\x84R\x80` \x84\x01Q\x16` \x85\x01R\x80`@\x84\x01Q\x16`@\x85\x01RP``\x82\x01Q``\x84\x01Rc\xFF\xFF\xFF\xFF`\x80\x83\x01Q\x16`\x80\x84\x01R`\xA0\x82\x01Q`\xE0`\xA0\x85\x01RaY+`\xE0\x85\x01\x82aT\xFFV[\x90P`\xC0\x83\x01Q\x84\x82\x03`\xC0\x86\x01Ra\x12\x90\x82\x82aUCV[` \x81R`\0aK\\` \x83\x01\x84aX\xD0V[` \x81R`\0\x82Q`\xE0` \x84\x01RaYta\x01\0\x84\x01\x82aT\xFFV[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01RaY\x91\x82\x82aUCV[\x91PP`@\x84\x01Q`\x01\x80`\xA0\x1B\x03\x80\x82\x16``\x86\x01R``\x86\x01Q\x91P\x80\x82Q\x16`\x80\x86\x01RPk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16`\xA0\x85\x01RP`\x80\x84\x01QaY\xE9`\xC0\x85\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P`\xA0\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16`\xE0\x85\x01RP\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15aZ\x19W`\0\x80\xFD[\x82QaZ$\x81aQ\xA4V[` \x93\x90\x93\x01Q\x92\x94\x92\x93PPPV[\x82\x81R`@` \x82\x01R`\0aO\xD4`@\x83\x01\x84aX\xD0V[`\0` \x82\x84\x03\x12\x15aZ_W`\0\x80\xFD[PQ\x91\x90PV[`\0\x82`\x1F\x83\x01\x12aZwW`\0\x80\xFD[\x81Q` aZ\x87aN6\x83aM\xF2V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aZ\xA6W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aN~W\x80Q\x83R\x91\x83\x01\x91\x83\x01aZ\xAAV[`\0\x80`@\x83\x85\x03\x12\x15aZ\xD4W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aZ\xEBW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12aZ\xFFW`\0\x80\xFD[\x81Q` a[\x0FaN6\x83aM\xF2V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a[.W`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a[UW\x85Qa[F\x81aIJV[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a[3V[\x91\x88\x01Q\x91\x96P\x90\x93PPP\x80\x82\x11\x15a[nW`\0\x80\xFD[Pa[{\x85\x82\x86\x01aZfV[\x91PP\x92P\x92\x90PV[`\0\x82\x19\x82\x11\x15a[\x98Wa[\x98aV\xC2V[P\x01\x90V[`\0\x82\x82\x10\x15a[\xAFWa[\xAFaV\xC2V[P\x03\x90V[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[`\0` \x82\x84\x03\x12\x15a[\xEAW`\0\x80\xFD[\x815aK\\\x81aM\xD5V[\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\x815a\\ \x81aIJV[a\\*\x81\x83a[\xF5V[P`\x01\x81\x01` \x83\x015a\\=\x81aIJV[a\\G\x81\x83a[\xF5V[P`@\x83\x015a\\V\x81aM\xD5V[\x81Tc\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x16`\xA0\x91\x90\x91\x1Bc\xFF\xFF\xFF\xFF`\xA0\x1B\x16\x17\x90UPPV[`\0a\\\x846\x83aN\xE4V[\x92\x91PPV[\x82\x81R`\0` `@\x81\x84\x01R\x83Q\x80`@\x85\x01R`\0[\x81\x81\x10\x15a\\\xBEW\x85\x81\x01\x83\x01Q\x85\x82\x01``\x01R\x82\x01a\\\xA2V[\x81\x81\x11\x15a\\\xD0W`\0``\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01``\x01\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\\\xF9W`\0\x80\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14aK\\W`\0\x80\xFD\xFEDelegationManager.registerOperatDelegationManager.completeQueued\xA2dipfsX\"\x12 \x8E>\x01\xA8m\xFE\x84h\xFE\xC8\xCC\xFF\xD3\xBBky\xB5\x9D\xDB\x87Q\xA7mp\x93)\xF3N2\xB5\xFF@dsolcC\0\x08\x0C\x003";
    /// The bytecode of the contract.
    pub static DELEGATIONMANAGER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x03\x8EW`\x005`\xE0\x1C\x80ce\xDA\x12d\x11a\x01\xDEW\x80c\xB14Bq\x11a\x01\x0FW\x80c\xD7\x9A\xCE\xAB\x11a\0\xADW\x80c\xF1ar\xB0\x11a\0|W\x80c\xF1ar\xB0\x14a\t\xF5W\x80c\xF2\xFD\xE3\x8B\x14a\n\x08W\x80c\xF6\x98\xDA%\x14a\n\x1BW\x80c\xFA\xBC\x1C\xBC\x14a\n#W`\0\x80\xFD[\x80c\xD7\x9A\xCE\xAB\x14a\t\x95W\x80c\xDA\x8B\xE8d\x14a\t\xBCW\x80c\xEB\x99\x0CY\x14a\t\xCFW\x80c\xEE\xA9\x06K\x14a\t\xE2W`\0\x80\xFD[\x80c\xC5\xE4\x80\xDB\x11a\0\xE9W\x80c\xC5\xE4\x80\xDB\x14a\x08\xB2W\x80c\xC9KQ\x11\x14a\tXW\x80c\xCAf\x1C\x04\x14a\tkW\x80c\xCF\x80\x87>\x14a\ttW`\0\x80\xFD[\x80c\xB14Bq\x14a\x08:W\x80c\xB7\xF0n\xBE\x14a\x08aW\x80c\xBBE\xFE\xF2\x14a\x08\x84W`\0\x80\xFD[\x80c\x91\x04\xC3\x19\x11a\x01|W\x80c\xA1\x06\x0C\x88\x11a\x01VW\x80c\xA1\x06\x0C\x88\x14a\x07\xE1W\x80c\xA1x\x84\x84\x14a\x07\xF4W\x80c\xA3d\xF4\xDA\x14a\x08\x14W\x80c\xA9\x8F\xB3U\x14a\x08'W`\0\x80\xFD[\x80c\x91\x04\xC3\x19\x14a\x07\xA0W\x80c\x99&\xEE}\x14a\x07\xBBW\x80c\x99\xBE\x81\xC8\x14a\x07\xCEW`\0\x80\xFD[\x80cw\x8EU\xF3\x11a\x01\xB8W\x80cw\x8EU\xF3\x14a\x07>W\x80c\x7FT\x80q\x14a\x07iW\x80c\x88o\x11\x95\x14a\x07|W\x80c\x8D\xA5\xCB[\x14a\x07\x8FW`\0\x80\xFD[\x80ce\xDA\x12d\x14a\x06\xFAW\x80cmp\xF7\xAE\x14a\x07#W\x80cqP\x18\xA6\x14a\x076W`\0\x80\xFD[\x80c9\xB7\x0E8\x11a\x02\xC3W\x80cP\xF7>|\x11a\x02aW\x80c\\\x97Z\xBB\x11a\x020W\x80c\\\x97Z\xBB\x14a\x06\xA0W\x80c\\\xFE\x8D,\x14a\x06\xA8W\x80c_\x96o\x14\x14a\x06\xBBW\x80c`\xD7\xFA\xED\x14a\x06\xE7W`\0\x80\xFD[\x80cP\xF7>|\x14a\x06YW\x80cY\\jg\x14a\x06bW\x80cY{6\xDA\x14a\x06jW\x80cZ\xC8j\xB7\x14a\x06}W`\0\x80\xFD[\x80cC7s\x82\x11a\x02\x9DW\x80cC7s\x82\x14a\x05\xC6W\x80cFe\xBC\xDA\x14a\x05\xEDW\x80cI\x07]\xA3\x14a\x06\x14W\x80cO\xC4\x0Ba\x14a\x06OW`\0\x80\xFD[\x80c9\xB7\x0E8\x14a\x05EW\x80c<\xDE\xB5\xE0\x14a\x05\x84W\x80c>(9\x1D\x14a\x05\xB3W`\0\x80\xFD[\x80c\x16\x92\x83e\x11a\x030W\x80c(\xA5s\xAE\x11a\x03\nW\x80c(\xA5s\xAE\x14a\x04\xC1W\x80c)\xC7}O\x14a\x04\xD4W\x80c3@C\x96\x14a\x04\xF4W\x80c7H#\xB5\x14a\x05\x07W`\0\x80\xFD[\x80c\x16\x92\x83e\x14a\x04NW\x80c\x1B\xBC\xE0\x91\x14a\x04\x87W\x80c `kp\x14a\x04\x9AW`\0\x80\xFD[\x80c\x0FX\x9EY\x11a\x03lW\x80c\x0FX\x9EY\x14a\x04\0W\x80c\x10\xD6z/\x14a\x04\x15W\x80c\x13-Ig\x14a\x04(W\x80c\x13d9\xDD\x14a\x04;W`\0\x80\xFD[\x80c\x04\xA4\xF9y\x14a\x03\x93W\x80c\x0B\x9FHz\x14a\x03\xCDW\x80c\r\xD8\xDD\x02\x14a\x03\xE0W[`\0\x80\xFD[a\x03\xBA\x7F;\x89\xFC\xA1Q\xCB\xE5\x12-X\xAC\xEE\x86\xCF\x18D\x13\xD7Q\xD5\x85w\x9B\xDD\x19\xD3\xBB\xFA:0m\xCE\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x03\xBAa\x03\xDB6`\x04aIoV[a\n6V[a\x03\xF3a\x03\xEE6`\x04aJ\x0EV[a\n\xF8V[`@Qa\x03\xC4\x91\x90aJOV[a\x04\x13a\x04\x0E6`\x04aJ\xECV[a\x0ErV[\0[a\x04\x13a\x04#6`\x04aK?V[a\x0F\xC2V[a\x04\x13a\x0466`\x04aKcV[a\x10uV[a\x04\x13a\x04I6`\x04aK\xA4V[a\x11,V[a\x03\xBAa\x04\\6`\x04aK?V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x90V[a\x03\xBAa\x04\x956`\x04aKcV[a\x12kV[a\x03\xBA\x7F\x8C\xAD\x95h{\xA8,,\xE5\x0Et\xF7\xB7Td^Q\x17\xC3\xA5\xBE\xC8\x15\x1C\x07&\xD5\x85y\x80\xA8f\x81V[a\x04\x13a\x04\xCF6`\x04aKcV[a\x12\x99V[a\x03\xBAa\x04\xE26`\x04aK?V[`\x9B` R`\0\x90\x81R`@\x90 T\x81V[a\x04\x13a\x05\x026`\x04aK\xBDV[a\x13IV[a\x055a\x05\x156`\x04aL\x80V[`\xA2` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x03\xC4V[a\x05l\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03\xC4V[a\x05la\x05\x926`\x04aK?V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T\x16\x90V[a\x055a\x05\xC16`\x04aK?V[a\x14\x86V[a\x03\xBA\x7F9\x11\x1B\xC4\xA4\xD6\x88\xE1\xF6\x85\x12=t\x97\xD4aSp\x15*\x8E\xE4\xA0Y>d{\xD0j\xD8\xBB\x0B\x81V[a\x05l\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x06Ba\x06\"6`\x04aL\xACV[`\xA1` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[`@Qa\x03\xC4\x91\x90aL\xFBV[a\x03\xBAb\x13\xC6\x80\x81V[a\x03\xBA`\x9DT\x81V[a\x04\x13a\x14\xA6V[a\x03\xBAa\x06x6`\x04aO\xA0V[a\x15mV[a\x055a\x06\x8B6`\x04aO\xDCV[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`fTa\x03\xBAV[a\x04\x13a\x06\xB66`\x04aPRV[a\x15\x9DV[a\x05la\x06\xC96`\x04aK?V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x99` R`@\x90 T\x16\x90V[a\x04\x13a\x06\xF56`\x04aQ\xB2V[a\x18HV[a\x05la\x07\x086`\x04aK?V[`\x9A` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x055a\x0716`\x04aK?V[a\x18\xE3V[a\x04\x13a\x19\x03V[a\x03\xBAa\x07L6`\x04aL\xACV[`\x98` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[a\x04\x13a\x07w6`\x04aS\x01V[a\x19\x17V[`eTa\x05l\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x05lV[a\x05ls\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x81V[a\x04\x13a\x07\xC96`\x04aS\x91V[a\x1A\x1CV[a\x04\x13a\x07\xDC6`\x04aT<V[a\x1D(V[a\x03\xBAa\x07\xEF6`\x04aTqV[a\x1D\xFAV[a\x03\xBAa\x08\x026`\x04aK?V[`\x9F` R`\0\x90\x81R`@\x90 T\x81V[a\x04\x13a\x08\"6`\x04aK?V[a\x1E\xBAV[a\x04\x13a\x0856`\x04aT<V[a \x05V[a\x05l\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x055a\x08o6`\x04aK\xA4V[`\x9E` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x055a\x08\x926`\x04aL\x80V[`\x9C` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[a\t\"a\x08\xC06`\x04aK?V[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x92\x84\x01\x81\x90R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R`\x99\x83R\x83\x90 \x83Q\x91\x82\x01\x84R\x80T\x85\x16\x82R`\x01\x01T\x93\x84\x16\x91\x81\x01\x91\x90\x91R`\x01`\xA0\x1B\x90\x92\x04c\xFF\xFF\xFF\xFF\x16\x90\x82\x01R\x90V[`@\x80Q\x82Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x80\x85\x01Q\x90\x91\x16\x90\x82\x01R\x91\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R``\x01a\x03\xC4V[a\x03\xBAa\tf6`\x04aT\xB7V[a @V[a\x03\xBAa\xC4\xE0\x81V[a\t\x87a\t\x826`\x04aK?V[a \x9FV[`@Qa\x03\xC4\x92\x91\x90aUsV[a\x03\xBA\x7F\xF4\x8B\xD2T\xE3\x0C\xE2\x95=\x18\x7F\xDBN\xF0\xCF\xDD\xDBx*\xB1\xDAU\xBE\xEB\xAE\xF6|\x1F%\x8D%n\x81V[a\x03\xF3a\t\xCA6`\x04aK?V[a$WV[a\x04\x13a\t\xDD6`\x04aTqV[a)\x1BV[a\x04\x13a\t\xF06`\x04aU\x98V[a*OV[a\x04\x13a\n\x036`\x04aU\xF0V[a*[V[a\x04\x13a\n\x166`\x04aK?V[a*\xECV[a\x03\xBAa+bV[a\x04\x13a\n16`\x04aK\xA4V[a+\xA0V[`@\x80Q\x7F;\x89\xFC\xA1Q\xCB\xE5\x12-X\xAC\xEE\x86\xCF\x18D\x13\xD7Q\xD5\x85w\x9B\xDD\x19\xD3\xBB\xFA:0m\xCE` \x80\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x83\x85\x01R\x88\x81\x16``\x84\x01R\x87\x16`\x80\x83\x01R`\xA0\x82\x01\x85\x90R`\xC0\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\xE0\x90\x92\x01\x90\x92R\x80Q\x91\x01 `\0\x90\x81a\n\xB4a+bV[`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x91\x90\x91R`B\x81\x01\x83\x90R`b\x01`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x98\x97PPPPPPPPV[`fT``\x90`\x01\x90`\x02\x90\x81\x16\x14\x15a\x0B-W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B$\x90aV\x0CV[`@Q\x80\x91\x03\x90\xFD[`\0\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0BGWa\x0BGaM#V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0BpW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x84\x81\x10\x15a\x0EiW\x85\x85\x82\x81\x81\x10a\x0B\x90Wa\x0B\x90aVCV[\x90P` \x02\x81\x01\x90a\x0B\xA2\x91\x90aVYV[a\x0B\xB0\x90` \x81\x01\x90aVyV[\x90P\x86\x86\x83\x81\x81\x10a\x0B\xC4Wa\x0B\xC4aVCV[\x90P` \x02\x81\x01\x90a\x0B\xD6\x91\x90aVYV[a\x0B\xE0\x90\x80aVyV[\x90P\x14a\x0CUW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FDelegationManager.queueWithdrawa`D\x82\x01R\x7Fl: input length mismatch\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B$V[`\0\x86\x86\x83\x81\x81\x10a\x0CiWa\x0CiaVCV[\x90P` \x02\x81\x01\x90a\x0C{\x91\x90aVYV[a\x0C\x8C\x90``\x81\x01\x90`@\x01aK?V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\r\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`H`$\x82\x01R\x7FDelegationManager.queueWithdrawa`D\x82\x01R\x7Fl: must provide valid withdrawal`d\x82\x01Rg address`\xC0\x1B`\x84\x82\x01R`\xA4\x01a\x0B$V[3`\0\x81\x81R`\x9A` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90a\x0E9\x90\x82\x89\x89\x86\x81\x81\x10a\rKWa\rKaVCV[\x90P` \x02\x81\x01\x90a\r]\x91\x90aVYV[a\rn\x90``\x81\x01\x90`@\x01aK?V[\x8A\x8A\x87\x81\x81\x10a\r\x80Wa\r\x80aVCV[\x90P` \x02\x81\x01\x90a\r\x92\x91\x90aVYV[a\r\x9C\x90\x80aVyV[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8E\x92P\x8D\x91P\x89\x90P\x81\x81\x10a\r\xE2Wa\r\xE2aVCV[\x90P` \x02\x81\x01\x90a\r\xF4\x91\x90aVYV[a\x0E\x02\x90` \x81\x01\x90aVyV[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa,\xFC\x92PPPV[\x83\x83\x81Q\x81\x10a\x0EKWa\x0EKaVCV[` \x90\x81\x02\x91\x90\x91\x01\x01RP\x80a\x0Ea\x81aV\xD8V[\x91PPa\x0BvV[P\x94\x93PPPPV[3`\0\x90\x81R`\x99` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x0F\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R\x7FDelegationManager.registerAsOper`D\x82\x01R\x7Fator: operator has already regis`d\x82\x01Rd\x1D\x19\\\x99Y`\xDA\x1B`\x84\x82\x01R`\xA4\x01a\x0B$V[a\x0F\x163\x84a1\x1EV[`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01Ra\x0F83\x80\x83`\0a3\xBAV[3`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8E\x84\x85X:#\x10\xD4\x1F|\x82\xB9B}\x0B\xD4\x9B\xADt\xBB\x9C\xFF\x9D4\x02\xA2\x9D\x8F\x9B(\xA0\xE2\x85`@Qa\x0Fq\x91\x90aV\xF3V[`@Q\x80\x91\x03\x90\xA23`\x01`\x01`\xA0\x1B\x03\x16\x7F\x02\xA9\x19\xED\x0E*\xCA\xD1\xDD\x90\xF1~\xF2\xFAJ\xE5F.\xE13\x91p\x03J\x851\xCC\xA4\xB6p\x80\x90\x84\x84`@Qa\x0F\xB4\x92\x91\x90aWEV[`@Q\x80\x91\x03\x90\xA2PPPPV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\x15W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x109\x91\x90aWtV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x10iW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B$\x90aW\x91V[a\x10r\x81a7jV[PV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x10\xD4WP3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14[a\x10\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B$\x90aW\xDBV[a\x10\xF9\x83a\x14\x86V[\x15a\x11'W`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R`\x9A` R`@\x90 T\x16a\x11%\x81\x85\x85\x85a8aV[P[PPPV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11tW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x98\x91\x90aX8V[a\x11\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B$\x90aXUV[`fT\x81\x81\x16\x14a\x12-W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B$V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x9B` R`@\x81 Ta\x12\x90\x85\x82\x86\x86a @V[\x95\x94PPPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x12\xF8WP3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14[a\x13\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B$\x90aW\xDBV[a\x13\x1D\x83a\x14\x86V[\x15a\x11'W`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R`\x9A` R`@\x90 T\x16a\x11%\x81\x85\x85\x85a8\xDCV[`fT`\x02\x90`\x04\x90\x81\x16\x14\x15a\x13rW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B$\x90aV\x0CV[`\x02`\xC9T\x14\x15a\x13\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x0B$V[`\x02`\xC9U`\0[\x88\x81\x10\x15a\x14uWa\x14e\x8A\x8A\x83\x81\x81\x10a\x13\xEAWa\x13\xEAaVCV[\x90P` \x02\x81\x01\x90a\x13\xFC\x91\x90aX\x9DV[\x89\x89\x84\x81\x81\x10a\x14\x0EWa\x14\x0EaVCV[\x90P` \x02\x81\x01\x90a\x14 \x91\x90aVyV[\x89\x89\x86\x81\x81\x10a\x142Wa\x142aVCV[\x90P` \x02\x015\x88\x88\x87\x81\x81\x10a\x14KWa\x14KaVCV[\x90P` \x02\x01` \x81\x01\x90a\x14`\x91\x90aX\xB3V[a9WV[a\x14n\x81aV\xD8V[\x90Pa\x13\xCDV[PP`\x01`\xC9UPPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x9A` R`@\x90 T\x16\x15\x15\x90V[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\xEEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x12\x91\x90aX8V[a\x15.W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B$\x90aXUV[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[`\0\x81`@Q` \x01a\x15\x80\x91\x90aYDV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0[\x81Q\x81\x10\x15a\x18DW`\0\x82\x82\x81Q\x81\x10a\x15\xBDWa\x15\xBDaVCV[` \x02` \x01\x01Q\x90P`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xCD)?o\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x16\x16\x91\x90aYWV[`@\x80Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x164W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16X\x91\x90aZ\x06V[\x91P\x91P\x81\x15a\x186W`@\x80\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x9F` R\x91\x82 \x80T\x91\x92\x82\x91\x90a\x16\x8F\x83aV\xD8V[\x91\x90PUP`\0`@Q\x80`\xE0\x01`@R\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x87`\xA0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x87``\x01Q`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01\x87`\x80\x01Qc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x87`\0\x01Q\x81R` \x01\x87` \x01Q\x81RP\x90P`\0a\x17\x0F\x82a\x15mV[`\0\x81\x81R`\x9E` R`@\x90 T\x90\x91P`\xFF\x16\x15a\x17\xA5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R\x7FDelegationManager.migrateQueuedW`D\x82\x01R\x7Fithdrawals: withdrawal already e`d\x82\x01Rdxists`\xD8\x1B`\x84\x82\x01R`\xA4\x01a\x0B$V[`\0\x81\x81R`\x9E` R`@\x90\x81\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x7F\x90\t\xAB\x15>\x80\x14\xFB\xFB\x02\xF2!\x7F\\\xDEz\xA7\xF9\xADsJ\xE8\\\xA3\xEE?L\xA2\xFD\xD4\x99\xF9\x90a\x17\xF0\x90\x83\x90\x85\x90aZ4V[`@Q\x80\x91\x03\x90\xA1`@\x80Q\x86\x81R` \x81\x01\x83\x90R\x7F\xDC\0u\x8Be\xEE\xF7\x1D\xC3x\x0C\x04\xEB\xE3l\xABk\xDB&l:i\x81\x87\xE2\x9E\x0F\r\xCA\x01&0\x91\x01`@Q\x80\x91\x03\x90\xA1PPPP[\x83`\x01\x01\x93PPPPa\x15\xA0V[PPV[`fT`\x02\x90`\x04\x90\x81\x16\x14\x15a\x18qW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B$\x90aV\x0CV[`\x02`\xC9T\x14\x15a\x18\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x0B$V[`\x02`\xC9Ua\x18\xD6\x86\x86\x86\x86\x86a9WV[PP`\x01`\xC9UPPPPV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x99` R`@\x90 T\x16\x15\x15\x90V[a\x19\x0Ba?\xBFV[a\x19\x15`\0a@\x19V[V[B\x83` \x01Q\x10\x15a\x19\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FDelegationManager.delegateToBySi`D\x82\x01R\x7Fgnature: staker signature expire`d\x82\x01R`\x19`\xFA\x1B`\x84\x82\x01R`\xA4\x01a\x0B$V[`\0`\x9B`\0\x87`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 T\x90P`\0a\x19\xD7\x87\x83\x88\x88` \x01Qa @V[`\x01`\x01`\xA0\x1B\x03\x88\x16`\0\x90\x81R`\x9B` R`@\x90 `\x01\x84\x01\x90U\x85Q\x90\x91Pa\x1A\x07\x90\x88\x90\x83\x90a@kV[a\x1A\x13\x87\x87\x86\x86a3\xBAV[PPPPPPPV[`fT`\x03\x90`\x08\x90\x81\x16\x14\x15a\x1AEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B$\x90aV\x0CV[B\x82`@\x01Q\x10\x15a\x1A\xB9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R`\0\x80Q` a]\x12\x839\x81Q\x91R`D\x82\x01R\x7ForToAVS: operator signature expi`d\x82\x01Rb\x1C\x99Y`\xEA\x1B`\x84\x82\x01R`\xA4\x01a\x0B$V[`\x013`\0\x90\x81R`\xA1` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x88\x16\x84R\x90\x91R\x90 T`\xFF\x16`\x01\x81\x11\x15a\x1A\xF3Wa\x1A\xF3aL\xE5V[\x14\x15a\x1BcW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R`\0\x80Q` a]\x12\x839\x81Q\x91R\x90\x82\x01R\x7ForToAVS: operator already regist`d\x82\x01Rc\x19\\\x99Y`\xE2\x1B`\x84\x82\x01R`\xA4\x01a\x0B$V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\xA2` \x90\x81R`@\x80\x83 \x85\x83\x01Q\x84R\x90\x91R\x90 T`\xFF\x16\x15a\x1B\xEEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`;`$\x82\x01R`\0\x80Q` a]\x12\x839\x81Q\x91R`D\x82\x01R\x7ForToAVS: salt already spent\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B$V[a\x1B\xF7\x83a\x18\xE3V[a\x1CrW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`R`$\x82\x01R`\0\x80Q` a]\x12\x839\x81Q\x91R`D\x82\x01R\x7ForToAVS: operator not registered`d\x82\x01Rq\x08\x1D\x1B\xC8\x11ZY\xD9[\x93\x18^Y\\\x88\x1EY]`r\x1B`\x84\x82\x01R`\xA4\x01a\x0B$V[`\0a\x1C\x88\x843\x85` \x01Q\x86`@\x01Qa\x1D\xFAV[\x90Pa\x1C\x99\x84\x82\x85`\0\x01Qa@kV[3`\0\x81\x81R`\xA1` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x89\x16\x80\x85R\x90\x83R\x81\x84 \x80T`\x01`\xFF\x19\x91\x82\x16\x81\x17\x90\x92U`\xA2\x85R\x83\x86 \x8A\x86\x01Q\x87R\x90\x94R\x93\x82\x90 \x80T\x90\x93\x16\x84\x17\x90\x92UQ\x90\x91\x7F\xF0\x95+\x1Ce'\x1D\x81\x9D9\x98=*\xBB\x04K\x9C\xAC\xE5\x9B\xCCMM\xD3\x89\xF5\x86\xEB\xDC\xB1[A\x91a\x1D\x1A\x91\x90aL\xFBV[`@Q\x80\x91\x03\x90\xA3PPPPV[a\x1D13a\x18\xE3V[a\x1D\xB3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FDelegationManager.updateOperator`D\x82\x01R\x7FMetadataURI: caller must be an o`d\x82\x01Rf82\xB90\xBA7\xB9`\xC9\x1B`\x84\x82\x01R`\xA4\x01a\x0B$V[3`\x01`\x01`\xA0\x1B\x03\x16\x7F\x02\xA9\x19\xED\x0E*\xCA\xD1\xDD\x90\xF1~\xF2\xFAJ\xE5F.\xE13\x91p\x03J\x851\xCC\xA4\xB6p\x80\x90\x83\x83`@Qa\x1D\xEE\x92\x91\x90aWEV[`@Q\x80\x91\x03\x90\xA2PPV[`@\x80Q\x7F\xF4\x8B\xD2T\xE3\x0C\xE2\x95=\x18\x7F\xDBN\xF0\xCF\xDD\xDBx*\xB1\xDAU\xBE\xEB\xAE\xF6|\x1F%\x8D%n` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x80\x87\x16\x92\x82\x01\x92\x90\x92R\x90\x84\x16``\x82\x01R`\x80\x81\x01\x83\x90R`\xA0\x81\x01\x82\x90R`\0\x90\x81\x90`\xC0\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0a\x1Ewa+bV[`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x91\x90\x91R`B\x81\x01\x83\x90R`b\x01`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x97\x96PPPPPPPV[`fT`\x03\x90`\x08\x90\x81\x16\x14\x15a\x1E\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B$\x90aV\x0CV[`\x013`\0\x90\x81R`\xA1` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x84R\x90\x91R\x90 T`\xFF\x16`\x01\x81\x11\x15a\x1F\x1DWa\x1F\x1DaL\xE5V[\x14a\x1F\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FDelegationManager.deregisterOper\x90\x82\x01R\x7FatorFromAVS: operator not regist`d\x82\x01Rc\x19\\\x99Y`\xE2\x1B`\x84\x82\x01R`\xA4\x01a\x0B$V[3`\0\x81\x81R`\xA1` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ\x90\x91\x7F\xF0\x95+\x1Ce'\x1D\x81\x9D9\x98=*\xBB\x04K\x9C\xAC\xE5\x9B\xCCMM\xD3\x89\xF5\x86\xEB\xDC\xB1[A\x91a\x1F\xF9\x91\x90aL\xFBV[`@Q\x80\x91\x03\x90\xA3PPV[3`\x01`\x01`\xA0\x1B\x03\x16\x7F\xA8\x9C\x1D\xC2C\xD8\x90\x8A\x96\xDD\x84\x94K\xCC\x97\xD6\xBCj\xC0\r\xD7\x8E b\x15v\xBEj<\x947\x13\x83\x83`@Qa\x1D\xEE\x92\x91\x90aWEV[`@\x80Q\x7F9\x11\x1B\xC4\xA4\xD6\x88\xE1\xF6\x85\x12=t\x97\xD4aSp\x15*\x8E\xE4\xA0Y>d{\xD0j\xD8\xBB\x0B` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x80\x87\x16\x92\x82\x01\x92\x90\x92R\x90\x83\x16``\x82\x01R`\x80\x81\x01\x84\x90R`\xA0\x81\x01\x82\x90R`\0\x90\x81\x90`\xC0\x01a\x1EUV[`@Qc`\xF4\x06+`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R``\x91\x82\x91`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c`\xF4\x06+\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!\x0FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!3\x91\x90aZMV[`@Qc\x94\xF6I\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R\x91\x92P`\0\x91\x82\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\x94\xF6I\xDD\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!\xA3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra!\xCB\x91\x90\x81\x01\x90aZ\xC1V[\x91P\x91P`\0\x83\x13a!\xE2W\x90\x95\x90\x94P\x92PPPV[``\x80\x83Q`\0\x14\x15a\"\x9CW`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x90` \x80\x83\x01\x90\x806\x837PP`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x92\x94P\x90P` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90Ps\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x82`\0\x81Q\x81\x10a\"WWa\"WaVCV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x84\x81`\0\x81Q\x81\x10a\"\x8BWa\"\x8BaVCV[` \x02` \x01\x01\x81\x81RPPa$JV[\x83Qa\"\xA9\x90`\x01a[\x85V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\xC0Wa\"\xC0aM#V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\"\xE9W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a#\x05Wa#\x05aM#V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a#.W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x84Q\x81\x10\x15a#\xC8W\x84\x81\x81Q\x81\x10a#OWa#OaVCV[` \x02` \x01\x01Q\x83\x82\x81Q\x81\x10a#iWa#iaVCV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x83\x81\x81Q\x81\x10a#\x9BWa#\x9BaVCV[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a#\xB5Wa#\xB5aVCV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a#4V[Ps\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x82`\x01\x84Qa#\xED\x91\x90a[\x9DV[\x81Q\x81\x10a#\xFDWa#\xFDaVCV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x84\x81`\x01\x84Qa$-\x91\x90a[\x9DV[\x81Q\x81\x10a$=Wa$=aVCV[` \x02` \x01\x01\x81\x81RPP[\x90\x97\x90\x96P\x94PPPPPV[`fT``\x90`\x01\x90`\x02\x90\x81\x16\x14\x15a$\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B$\x90aV\x0CV[a$\x8C\x83a\x14\x86V[a%\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FDelegationManager.undelegate: st\x90\x82\x01R\x7Faker must be delegated to undele`d\x82\x01Rcgate`\xE0\x1B`\x84\x82\x01R`\xA4\x01a\x0B$V[a%\x15\x83a\x18\xE3V[\x15a%\x88W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FDelegationManager.undelegate: op`D\x82\x01R\x7Ferators cannot be undelegated\0\0\0`d\x82\x01R`\x84\x01a\x0B$V[`\x01`\x01`\xA0\x1B\x03\x83\x16a&\x04W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R\x7FDelegationManager.undelegate: ca`D\x82\x01R\x7Fnnot undelegate zero address\0\0\0\0`d\x82\x01R`\x84\x01a\x0B$V[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x81\x81R`\x9A` R`@\x90 T\x90\x91\x16\x903\x14\x80a&7WP3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14[\x80a&^WP`\x01`\x01`\xA0\x1B\x03\x81\x81\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T\x163\x14[a&\xD0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FDelegationManager.undelegate: ca`D\x82\x01R\x7Fller cannot undelegate staker\0\0\0`d\x82\x01R`\x84\x01a\x0B$V[`\0\x80a&\xDC\x86a \x9FV[\x90\x92P\x90P3`\x01`\x01`\xA0\x1B\x03\x87\x16\x14a'2W\x82`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x7F\xF0\xED\xDF\x07\xE6\xEA\x14\xF3\x88\xB4~\x1E\x94\xA0\xF4d\xEC\xBD\x9E\xEDAq\x13\x0E\x0F\xC0\xE9\x9F\xB4\x03\n\x8A`@Q`@Q\x80\x91\x03\x90\xA3[\x82`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFE\xE3\tf\xA2V\xB7\x1E\x14\xBC\x0E\xBF\xC9C\x15\xE2\x8E\xF4\xA9zq1\xA9\xE2\xB7\xA3\x10\xA7:\xF4Fv`@Q`@Q\x80\x91\x03\x90\xA3`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x9A` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x81Qa'\xB4W`@\x80Q`\0\x81R` \x81\x01\x90\x91R\x94Pa)\x12V[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a'\xCDWa'\xCDaM#V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a'\xF6W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x94P`\0[\x82Q\x81\x10\x15a)\x10W`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91` \x80\x83\x01\x90\x806\x837PP`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x92\x93P`\0\x92\x91P` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x84\x83\x81Q\x81\x10a(\\Wa(\\aVCV[` \x02` \x01\x01Q\x82`\0\x81Q\x81\x10a(wWa(waVCV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x83\x83\x81Q\x81\x10a(\xA9Wa(\xA9aVCV[` \x02` \x01\x01Q\x81`\0\x81Q\x81\x10a(\xC4Wa(\xC4aVCV[` \x02` \x01\x01\x81\x81RPPa(\xDD\x89\x87\x8B\x85\x85a,\xFCV[\x88\x84\x81Q\x81\x10a(\xEFWa(\xEFaVCV[` \x02` \x01\x01\x81\x81RPPPP\x80\x80a)\x08\x90aV\xD8V[\x91PPa'\xFCV[P[PPPP\x91\x90PV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a);WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a)UWP0;\x15\x80\x15a)UWP`\0T`\xFF\x16`\x01\x14[a)\xB8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x0B$V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a)\xDBW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a)\xE5\x84\x84aB%V[a)\xEDaC\x0BV[`\x97Ua)\xF9\x85a@\x19V[a*\x02\x82aC\xA2V[\x80\x15a*HW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPV[a\x11'3\x84\x84\x84a3\xBAV[a*d3a\x18\xE3V[a*\xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FDelegationManager.modifyOperator`D\x82\x01R\x7FDetails: caller must be an opera`d\x82\x01Rb:7\xB9`\xE9\x1B`\x84\x82\x01R`\xA4\x01a\x0B$V[a\x10r3\x82a1\x1EV[a*\xF4a?\xBFV[`\x01`\x01`\xA0\x1B\x03\x81\x16a+YW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x0B$V[a\x10r\x81a@\x19V[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14\x15a+\x93WP`\x97T\x90V[a+\x9BaC\x0BV[\x90P\x90V[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+\xF3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\x17\x91\x90aWtV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a,GW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B$\x90aW\x91V[`fT\x19\x81\x19`fT\x19\x16\x14a,\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B$V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x12`V[`\0`\x01`\x01`\xA0\x1B\x03\x86\x16a-\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`P`$\x82\x01R\x7FDelegationManager._removeSharesA`D\x82\x01R\x7FndQueueWithdrawal: staker cannot`d\x82\x01Ro be zero address`\x80\x1B`\x84\x82\x01R`\xA4\x01a\x0B$V[\x82Qa.\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`M`$\x82\x01R\x7FDelegationManager._removeSharesA`D\x82\x01R\x7FndQueueWithdrawal: strategies ca`d\x82\x01Rlnnot be empty`\x98\x1B`\x84\x82\x01R`\xA4\x01a\x0B$V[`\0[\x83Q\x81\x10\x15a0,W`\x01`\x01`\xA0\x1B\x03\x86\x16\x15a.vWa.v\x86\x88\x86\x84\x81Q\x81\x10a.OWa.OaVCV[` \x02` \x01\x01Q\x86\x85\x81Q\x81\x10a.iWa.iaVCV[` \x02` \x01\x01Qa8aV[s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0`\x01`\x01`\xA0\x1B\x03\x16\x84\x82\x81Q\x81\x10a.\xA6Wa.\xA6aVCV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a/oW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xBE\xFF\xBB\x89\x88\x85\x84\x81Q\x81\x10a.\xFFWa.\xFFaVCV[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a/8\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a/RW`\0\x80\xFD[PZ\xF1\x15\x80\x15a/fW=`\0\x80>=`\0\xFD[PPPPa0$V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8C\x80\xD4\xE5\x88\x86\x84\x81Q\x81\x10a/\xB1Wa/\xB1aVCV[` \x02` \x01\x01Q\x86\x85\x81Q\x81\x10a/\xCBWa/\xCBaVCV[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a/\xF1\x93\x92\x91\x90a[\xB4V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a0\x0BW`\0\x80\xFD[PZ\xF1\x15\x80\x15a0\x1FW=`\0\x80>=`\0\xFD[PPPP[`\x01\x01a. V[P`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x9F` R`@\x81 \x80T\x91\x82\x91\x90a0T\x83aV\xD8V[\x91\x90PUP`\0`@Q\x80`\xE0\x01`@R\x80\x89`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x88`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x87`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86\x81R` \x01\x85\x81RP\x90P`\0a0\xBC\x82a\x15mV[`\0\x81\x81R`\x9E` R`@\x90\x81\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x90\x91P\x7F\x90\t\xAB\x15>\x80\x14\xFB\xFB\x02\xF2!\x7F\\\xDEz\xA7\xF9\xADsJ\xE8\\\xA3\xEE?L\xA2\xFD\xD4\x99\xF9\x90a1\n\x90\x83\x90\x85\x90aZ4V[`@Q\x80\x91\x03\x90\xA1\x98\x97PPPPPPPPV[`\0a1-` \x83\x01\x83aK?V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a1\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`T`$\x82\x01R\x7FDelegationManager._setOperatorDe`D\x82\x01R\x7Ftails: cannot set `earningsRecei`d\x82\x01Rsver` to zero address``\x1B`\x84\x82\x01R`\xA4\x01a\x0B$V[b\x13\xC6\x80a1\xDB``\x83\x01`@\x84\x01a[\xD8V[c\xFF\xFF\xFF\xFF\x16\x11\x15a2\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`l`$\x82\x01R\x7FDelegationManager._setOperatorDe`D\x82\x01R\x7Ftails: stakerOptOutWindowBlocks `d\x82\x01R\x7Fcannot be > MAX_STAKER_OPT_OUT_W`\x84\x82\x01RkINDOW_BLOCKS`\xA0\x1B`\xA4\x82\x01R`\xC4\x01a\x0B$V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x99` R`@\x90\x81\x90 `\x01\x01T`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x90a2\xCC\x90``\x84\x01\x90\x84\x01a[\xD8V[c\xFF\xFF\xFF\xFF\x16\x10\x15a3bW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`S`$\x82\x01R\x7FDelegationManager._setOperatorDe`D\x82\x01R\x7Ftails: stakerOptOutWindowBlocks `d\x82\x01Rr\x18\xD8[\x9B\x9B\xDD\x08\x18\x99H\x19\x19X\xDC\x99X\\\xD9Y`j\x1B`\x84\x82\x01R`\xA4\x01a\x0B$V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x99` R`@\x90 \x81\x90a3\x86\x82\x82a\\\x15V[PP`@Q3\x90\x7F\xFE\xBE\\\xD2K,\xBC{\x06[\x9D\x0F\xDE\xB9\x04F\x1EJ\xFC\xFFW\xDDW\xAC\xDA\x1Ex2\x03\x1B\xA7\xAC\x90a\x1D\xEE\x90\x84\x90aV\xF3V[`fT`\0\x90`\x01\x90\x81\x16\x14\x15a3\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B$\x90aV\x0CV[a3\xEC\x85a\x14\x86V[\x15a4iW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FDelegationManager._delegate: sta`D\x82\x01R\x7Fker is already actively delegate`d\x82\x01R`\x19`\xFA\x1B`\x84\x82\x01R`\xA4\x01a\x0B$V[a4r\x84a\x18\xE3V[a4\xF2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R\x7FDelegationManager._delegate: ope`D\x82\x01R\x7Frator is not registered in Eigen`d\x82\x01Rd&0\xBC\xB2\xB9`\xD9\x1B`\x84\x82\x01R`\xA4\x01a\x0B$V[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T\x16\x80\x15\x80\x15\x90a5(WP3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14\x15[\x80\x15a5=WP3`\x01`\x01`\xA0\x1B\x03\x86\x16\x14\x15[\x15a6\xAAWB\x84` \x01Q\x10\x15a5\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FDelegationManager._delegate: app`D\x82\x01R\x7Frover signature expired\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B$V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x9C` \x90\x81R`@\x80\x83 \x86\x84R\x90\x91R\x90 T`\xFF\x16\x15a6VW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FDelegationManager._delegate: app`D\x82\x01R\x7FroverSalt already spent\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B$V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x9C` \x90\x81R`@\x80\x83 \x86\x84R\x82R\x82 \x80T`\xFF\x19\x16`\x01\x17\x90U\x85\x01Qa6\x97\x90\x88\x90\x88\x90\x85\x90\x88\x90a\n6V[\x90Pa6\xA8\x82\x82\x87`\0\x01Qa@kV[P[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\0\x81\x81R`\x9A` R`@\x80\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x94\x8A\x16\x94\x85\x17\x90UQ\x7F\xC3\xEE\x9F._\xDA\x98\xE8\x06j\x1Ft[-\xF9(_Ao\xE9\x8C\xF2U\x9C\xD2\x14\x84\xB3\xD8t3\x04\x91\x90\xA3`\0\x80a7\t\x88a \x9FV[\x91P\x91P`\0[\x82Q\x81\x10\x15a7_Wa7W\x88\x8A\x85\x84\x81Q\x81\x10a70Wa70aVCV[` \x02` \x01\x01Q\x85\x85\x81Q\x81\x10a7JWa7JaVCV[` \x02` \x01\x01Qa8\xDCV[`\x01\x01a7\x10V[PPPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16a7\xF8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x0B$V[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x98` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a8\x98\x90\x84\x90a[\x9DV[\x92PP\x81\x90UP\x83`\x01`\x01`\xA0\x1B\x03\x16\x7Fi\t`\x007\xB7]{G3\xAE\xDD\x81TB\xB5\xEC\x01\x8A\x82wQ\xC82\xAA\xFFd\xEB\xA5\xD6\xD2\xDD\x84\x84\x84`@Qa\x0F\xB4\x93\x92\x91\x90a[\xB4V[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x98` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a9\x13\x90\x84\x90a[\x85V[\x92PP\x81\x90UP\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\x1E\xC0B\xC9e\xE2\xED\xD7\x10{Q\x18\x8E\xE0\xF3\x83\xE2.v\x17\x90A\xAB:\x9D\x18\xFF\x15\x14\x05\x16l\x84\x84\x84`@Qa\x0F\xB4\x93\x92\x91\x90a[\xB4V[`\0a9ea\x06x\x87a\\xV[`\0\x81\x81R`\x9E` R`@\x90 T\x90\x91P`\xFF\x16a9\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R`\0\x80Q` a]2\x839\x81Q\x91R`D\x82\x01R\x7FAction: action is not in queue\0\0`d\x82\x01R`\x84\x01a\x0B$V[`\x9DTC\x90a9\xEF`\xA0\x89\x01`\x80\x8A\x01a[\xD8V[c\xFF\xFF\xFF\xFF\x16a9\xFF\x91\x90a[\x85V[\x11\x15a:\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`W`$\x82\x01R`\0\x80Q` a]2\x839\x81Q\x91R`D\x82\x01R\x7FAction: withdrawalDelayBlocks pe`d\x82\x01R\x7Friod has not yet passed\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x0B$V[a:\x97``\x87\x01`@\x88\x01aK?V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a;\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`K`$\x82\x01R`\0\x80Q` a]2\x839\x81Q\x91R`D\x82\x01R\x7FAction: only withdrawer can comp`d\x82\x01Rj62\xBA2\x900\xB1\xBA4\xB7\xB7`\xA9\x1B`\x84\x82\x01R`\xA4\x01a\x0B$V[\x81\x15a;\x96Wa;2`\xA0\x87\x01\x87aVyV[\x85\x14\x90Pa;\x96W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R`\0\x80Q` a]2\x839\x81Q\x91R`D\x82\x01R\x7FAction: input length mismatch\0\0\0`d\x82\x01R`\x84\x01a\x0B$V[`\0\x81\x81R`\x9E` R`@\x90 \x80T`\xFF\x19\x16\x90U\x81\x15a<lW`\0[a;\xC2`\xA0\x88\x01\x88aVyV[\x90P\x81\x10\x15a<fWa<^a;\xDB` \x89\x01\x89aK?V[3a;\xE9`\xA0\x8B\x01\x8BaVyV[\x85\x81\x81\x10a;\xF9Wa;\xF9aVCV[\x90P` \x02\x01` \x81\x01\x90a<\x0E\x91\x90aK?V[a<\x1B`\xC0\x8C\x01\x8CaVyV[\x86\x81\x81\x10a<+Wa<+aVCV[\x90P` \x02\x015\x8A\x8A\x87\x81\x81\x10a<DWa<DaVCV[\x90P` \x02\x01` \x81\x01\x90a<Y\x91\x90aK?V[aD\x9CV[`\x01\x01a;\xB5V[Pa?\x84V[3`\0\x90\x81R`\x9A` R`@\x81 T`\x01`\x01`\xA0\x1B\x03\x16\x90[a<\x94`\xA0\x89\x01\x89aVyV[\x90P\x81\x10\x15a?\x81Ws\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0a<\xBF`\xA0\x8A\x01\x8AaVyV[\x83\x81\x81\x10a<\xCFWa<\xCFaVCV[\x90P` \x02\x01` \x81\x01\x90a<\xE4\x91\x90aK?V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a>4W`\0a=\x02` \x8A\x01\x8AaK?V[\x90P`\0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\x0E\x81\x07<\x83a=C`\xC0\x8E\x01\x8EaVyV[\x87\x81\x81\x10a=SWa=SaVCV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x87\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x94\x16`\x04\x85\x01R` \x02\x91\x90\x91\x015`$\x83\x01RP`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a=\xA7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a=\xCB\x91\x90aZMV[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R`\x9A` R`@\x90 T\x91\x92P\x16\x80\x15a>,Wa>,\x81\x84a>\x01`\xA0\x8F\x01\x8FaVyV[\x88\x81\x81\x10a>\x11Wa>\x11aVCV[\x90P` \x02\x01` \x81\x01\x90a>&\x91\x90aK?V[\x85a8\xDCV[PPPa?yV[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16cP\xFFr%3a>q`\xA0\x8C\x01\x8CaVyV[\x85\x81\x81\x10a>\x81Wa>\x81aVCV[\x90P` \x02\x01` \x81\x01\x90a>\x96\x91\x90aK?V[a>\xA3`\xC0\x8D\x01\x8DaVyV[\x86\x81\x81\x10a>\xB3Wa>\xB3aVCV[\x90P` \x02\x015`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a>\xD8\x93\x92\x91\x90a[\xB4V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a>\xF2W`\0\x80\xFD[PZ\xF1\x15\x80\x15a?\x06W=`\0\x80>=`\0\xFD[PPPP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a?yWa?y\x823a?+`\xA0\x8C\x01\x8CaVyV[\x85\x81\x81\x10a?;Wa?;aVCV[\x90P` \x02\x01` \x81\x01\x90a?P\x91\x90aK?V[a?]`\xC0\x8D\x01\x8DaVyV[\x86\x81\x81\x10a?mWa?maVCV[\x90P` \x02\x015a8\xDCV[`\x01\x01a<\x87V[PP[`@Q\x81\x81R\x7F\xC9p\x98\xC2\xF6X\x80\x0BM\xF2\x90\x01R\x7Fs$\xBC\xDF\xFC\xF6\xE8u\x1Ai\x9A\xB9 \xA1\xEC\xED[\x1D\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPPPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x19\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x0B$V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16;\x15aA\x85W`@Qc\x0B\x13]?`\xE1\x1B\x80\x82R\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\x16&\xBA~\x90a@\xAB\x90\x86\x90\x86\x90`\x04\x01a\\\x8AV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a@\xC8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a@\xEC\x91\x90a\\\xE7V[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14a\x11'W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`S`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: ERC1271 signature `d\x82\x01Rr\x1D\x99\\\x9AY\x9AX\xD8]\x1A[\xDB\x88\x19\x98Z[\x19Y`j\x1B`\x84\x82\x01R`\xA4\x01a\x0B$V[\x82`\x01`\x01`\xA0\x1B\x03\x16aA\x99\x83\x83aE\xD5V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x11'W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: signature not from`d\x82\x01Rf\x109\xB4\xB3\xB72\xB9`\xC9\x1B`\x84\x82\x01R`\xA4\x01a\x0B$V[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15aBFWP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[aB\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x0B$V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a\x18D\x82a7jV[`@\x80Q\x80\x82\x01\x82R`\n\x81Ri\"\xB4\xB3\xB2\xB7&0\xBC\xB2\xB9`\xB1\x1B` \x91\x82\x01R\x81Q\x7F\x8C\xAD\x95h{\xA8,,\xE5\x0Et\xF7\xB7Td^Q\x17\xC3\xA5\xBE\xC8\x15\x1C\x07&\xD5\x85y\x80\xA8f\x81\x83\x01R\x7Fq\xB6%\xCF\xADD\xBA\xC6;\x13\xDB\xA0\x7F.\x1D`\x84\xEE\x04\xB6\xF8u!\x01\xEC\xE6\x12mXN\xE6\xEA\x81\x84\x01RF``\x82\x01R0`\x80\x80\x83\x01\x91\x90\x91R\x83Q\x80\x83\x03\x90\x91\x01\x81R`\xA0\x90\x91\x01\x90\x92R\x81Q\x91\x01 \x90V[a\xC4\xE0\x81\x11\x15aD[W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`r`$\x82\x01R\x7FDelegationManager._initializeWit`D\x82\x01R\x7FhdrawalDelayBlocks: _withdrawalD`d\x82\x01R\x7FelayBlocks cannot be > MAX_WITHD`\x84\x82\x01RqRAWAL_DELAY_BLOCKS`p\x1B`\xA4\x82\x01R`\xC4\x01a\x0B$V[`\x9DT`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7FO\xFB\0@\x05t\x14t)\xEE7zV38c!\xE6mE\xD8\xB1Fv\x01K_\xA3\x93\xE6\x1E\x9E\x91\x01`@Q\x80\x91\x03\x90\xA1`\x9DUV[`\x01`\x01`\xA0\x1B\x03\x83\x16s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x14\x15aEGW`@Qb8{\x13`\xE8\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c8{\x13\0\x90aE\x10\x90\x88\x90\x88\x90\x87\x90`\x04\x01a[\xB4V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aE*W`\0\x80\xFD[PZ\xF1\x15\x80\x15aE>W=`\0\x80>=`\0\xFD[PPPPa*HV[`@Qc\xC6\x08\xC7\xF3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x84\x81\x16`$\x83\x01R`D\x82\x01\x84\x90R\x82\x81\x16`d\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xC6\x08\xC7\xF3\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aE\xC1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a7_W=`\0\x80>=`\0\xFD[`\0\x80`\0aE\xE4\x85\x85aE\xF9V[\x91P\x91PaE\xF1\x81aFiV[P\x93\x92PPPV[`\0\x80\x82Q`A\x14\x15aF0W` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1AaF$\x87\x82\x85\x85aH$V[\x94P\x94PPPPaFbV[\x82Q`@\x14\x15aFZW` \x83\x01Q`@\x84\x01QaFO\x86\x83\x83aI\x11V[\x93P\x93PPPaFbV[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0\x81`\x04\x81\x11\x15aF}WaF}aL\xE5V[\x14\x15aF\x86WPV[`\x01\x81`\x04\x81\x11\x15aF\x9AWaF\x9AaL\xE5V[\x14\x15aF\xE8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x0B$V[`\x02\x81`\x04\x81\x11\x15aF\xFCWaF\xFCaL\xE5V[\x14\x15aGJW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x0B$V[`\x03\x81`\x04\x81\x11\x15aG^WaG^aL\xE5V[\x14\x15aG\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x0B$V[`\x04\x81`\x04\x81\x11\x15aG\xCBWaG\xCBaL\xE5V[\x14\x15a\x10rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 'v' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x0B$V[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15aH[WP`\0\x90P`\x03aI\x08V[\x84`\xFF\x16`\x1B\x14\x15\x80\x15aHsWP\x84`\xFF\x16`\x1C\x14\x15[\x15aH\x84WP`\0\x90P`\x04aI\x08V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15aH\xD8W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16aI\x01W`\0`\x01\x92P\x92PPaI\x08V[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x80`\x01`\x01`\xFF\x1B\x03\x83\x16\x81aI.`\xFF\x86\x90\x1C`\x1Ba[\x85V[\x90PaI<\x87\x82\x88\x85aH$V[\x93P\x93PPP\x93P\x93\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x10rW`\0\x80\xFD[\x805aIj\x81aIJV[\x91\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15aI\x87W`\0\x80\xFD[\x855aI\x92\x81aIJV[\x94P` \x86\x015aI\xA2\x81aIJV[\x93P`@\x86\x015aI\xB2\x81aIJV[\x94\x97\x93\x96P\x93\x94``\x81\x015\x94P`\x80\x015\x92\x91PPV[`\0\x80\x83`\x1F\x84\x01\x12aI\xDCW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aI\xF3W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15aFbW`\0\x80\xFD[`\0\x80` \x83\x85\x03\x12\x15aJ!W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aJ7W`\0\x80\xFD[aJC\x85\x82\x86\x01aI\xCAV[\x90\x96\x90\x95P\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aJ\x87W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aJkV[P\x90\x96\x95PPPPPPV[`\0``\x82\x84\x03\x12\x15aJ\xA5W`\0\x80\xFD[P\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12aJ\xBDW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aJ\xD4W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aFbW`\0\x80\xFD[`\0\x80`\0`\x80\x84\x86\x03\x12\x15aK\x01W`\0\x80\xFD[aK\x0B\x85\x85aJ\x93V[\x92P``\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aK&W`\0\x80\xFD[aK2\x86\x82\x87\x01aJ\xABV[\x94\x97\x90\x96P\x93\x94PPPPV[`\0` \x82\x84\x03\x12\x15aKQW`\0\x80\xFD[\x815aK\\\x81aIJV[\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15aKxW`\0\x80\xFD[\x835aK\x83\x81aIJV[\x92P` \x84\x015aK\x93\x81aIJV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0` \x82\x84\x03\x12\x15aK\xB6W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80`\x80\x89\x8B\x03\x12\x15aK\xD9W`\0\x80\xFD[\x885`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aK\xF0W`\0\x80\xFD[aK\xFC\x8C\x83\x8D\x01aI\xCAV[\x90\x9AP\x98P` \x8B\x015\x91P\x80\x82\x11\x15aL\x15W`\0\x80\xFD[aL!\x8C\x83\x8D\x01aI\xCAV[\x90\x98P\x96P`@\x8B\x015\x91P\x80\x82\x11\x15aL:W`\0\x80\xFD[aLF\x8C\x83\x8D\x01aI\xCAV[\x90\x96P\x94P``\x8B\x015\x91P\x80\x82\x11\x15aL_W`\0\x80\xFD[PaLl\x8B\x82\x8C\x01aI\xCAV[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[`\0\x80`@\x83\x85\x03\x12\x15aL\x93W`\0\x80\xFD[\x825aL\x9E\x81aIJV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`@\x83\x85\x03\x12\x15aL\xBFW`\0\x80\xFD[\x825aL\xCA\x81aIJV[\x91P` \x83\x015aL\xDA\x81aIJV[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x02\x83\x10aM\x1DWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aM[WaM[aM#V[`@R\x90V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aM[WaM[aM#V[`@Q`\xC0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aM[WaM[aM#V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aM\xCDWaM\xCDaM#V[`@R\x91\x90PV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x10rW`\0\x80\xFD[\x805aIj\x81aM\xD5V[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15aN\x0BWaN\x0BaM#V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12aN&W`\0\x80\xFD[\x815` aN;aN6\x83aM\xF2V[aM\xA5V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aNZW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aN~W\x805aNq\x81aIJV[\x83R\x91\x83\x01\x91\x83\x01aN^V[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12aN\x9AW`\0\x80\xFD[\x815` aN\xAAaN6\x83aM\xF2V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aN\xC9W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aN~W\x805\x83R\x91\x83\x01\x91\x83\x01aN\xCDV[`\0`\xE0\x82\x84\x03\x12\x15aN\xF6W`\0\x80\xFD[aN\xFEaM9V[\x90PaO\t\x82aI_V[\x81RaO\x17` \x83\x01aI_V[` \x82\x01RaO(`@\x83\x01aI_V[`@\x82\x01R``\x82\x015``\x82\x01RaOC`\x80\x83\x01aM\xE7V[`\x80\x82\x01R`\xA0\x82\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aObW`\0\x80\xFD[aOn\x85\x83\x86\x01aN\x15V[`\xA0\x84\x01R`\xC0\x84\x015\x91P\x80\x82\x11\x15aO\x87W`\0\x80\xFD[PaO\x94\x84\x82\x85\x01aN\x89V[`\xC0\x83\x01RP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15aO\xB2W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aO\xC8W`\0\x80\xFD[aO\xD4\x84\x82\x85\x01aN\xE4V[\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15aO\xEEW`\0\x80\xFD[\x815`\xFF\x81\x16\x81\x14aK\\W`\0\x80\xFD[`\0`@\x82\x84\x03\x12\x15aP\x11W`\0\x80\xFD[aP\x19aMaV[\x90P\x815aP&\x81aIJV[\x81R` \x82\x015k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14aPGW`\0\x80\xFD[` \x82\x01R\x92\x91PPV[`\0` \x80\x83\x85\x03\x12\x15aPeW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aP|W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12aP\x90W`\0\x80\xFD[\x815aP\x9EaN6\x82aM\xF2V[\x81\x81R`\x05\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x84\x81\x01\x90\x88\x83\x11\x15aP\xBDW`\0\x80\xFD[\x85\x85\x01[\x83\x81\x10\x15aQ\x97W\x805\x85\x81\x11\x15aP\xD9W`\0\x80\x81\xFD[\x86\x01`\xE0\x81\x8C\x03`\x1F\x19\x01\x81\x13\x15aP\xF1W`\0\x80\x81\xFD[aP\xF9aM\x83V[\x89\x83\x015\x88\x81\x11\x15aQ\x0BW`\0\x80\x81\xFD[aQ\x19\x8E\x8C\x83\x87\x01\x01aN\x15V[\x82RP`@\x80\x84\x015\x89\x81\x11\x15aQ0W`\0\x80\x81\xFD[aQ>\x8F\x8D\x83\x88\x01\x01aN\x89V[\x8C\x84\x01RP``aQP\x81\x86\x01aI_V[\x82\x84\x01R`\x80\x91PaQd\x8F\x83\x87\x01aO\xFFV[\x90\x83\x01RaQt`\xC0\x85\x01aM\xE7V[\x90\x82\x01RaQ\x83\x83\x83\x01aI_V[`\xA0\x82\x01R\x85RPP\x91\x86\x01\x91\x86\x01aP\xC1V[P\x98\x97PPPPPPPPV[\x80\x15\x15\x81\x14a\x10rW`\0\x80\xFD[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15aQ\xCAW`\0\x80\xFD[\x855`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aQ\xE1W`\0\x80\xFD[\x90\x87\x01\x90`\xE0\x82\x8A\x03\x12\x15aQ\xF5W`\0\x80\xFD[\x90\x95P` \x87\x015\x90\x80\x82\x11\x15aR\x0BW`\0\x80\xFD[PaR\x18\x88\x82\x89\x01aI\xCAV[\x90\x95P\x93PP`@\x86\x015\x91P``\x86\x015aR3\x81aQ\xA4V[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x82`\x1F\x83\x01\x12aRRW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aRkWaRkaM#V[aR~`\x1F\x82\x01`\x1F\x19\x16` \x01aM\xA5V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15aR\x93W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0`@\x82\x84\x03\x12\x15aR\xC2W`\0\x80\xFD[aR\xCAaMaV[\x90P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aR\xE2W`\0\x80\xFD[aR\xEE\x84\x82\x85\x01aRAV[\x82RP` \x82\x015` \x82\x01R\x92\x91PPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15aS\x19W`\0\x80\xFD[\x855aS$\x81aIJV[\x94P` \x86\x015aS4\x81aIJV[\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aSPW`\0\x80\xFD[aS\\\x89\x83\x8A\x01aR\xB0V[\x94P``\x88\x015\x91P\x80\x82\x11\x15aSrW`\0\x80\xFD[PaS\x7F\x88\x82\x89\x01aR\xB0V[\x95\x98\x94\x97P\x92\x95`\x80\x015\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15aS\xA4W`\0\x80\xFD[\x825aS\xAF\x81aIJV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aS\xCBW`\0\x80\xFD[\x90\x84\x01\x90``\x82\x87\x03\x12\x15aS\xDFW`\0\x80\xFD[`@Q``\x81\x01\x81\x81\x10\x83\x82\x11\x17\x15aS\xFAWaS\xFAaM#V[`@R\x825\x82\x81\x11\x15aT\x0CW`\0\x80\xFD[aT\x18\x88\x82\x86\x01aRAV[\x82RP` \x83\x015` \x82\x01R`@\x83\x015`@\x82\x01R\x80\x93PPPP\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15aTOW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aTeW`\0\x80\xFD[aJC\x85\x82\x86\x01aJ\xABV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aT\x87W`\0\x80\xFD[\x845aT\x92\x81aIJV[\x93P` \x85\x015aT\xA2\x81aIJV[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aT\xCDW`\0\x80\xFD[\x845aT\xD8\x81aIJV[\x93P` \x85\x015\x92P`@\x85\x015aT\xEF\x81aIJV[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aU8W\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aU\x13V[P\x94\x95\x94PPPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aU8W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aUWV[`@\x81R`\0aU\x86`@\x83\x01\x85aT\xFFV[\x82\x81\x03` \x84\x01Ra\x12\x90\x81\x85aUCV[`\0\x80`\0``\x84\x86\x03\x12\x15aU\xADW`\0\x80\xFD[\x835aU\xB8\x81aIJV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aU\xD3W`\0\x80\xFD[aU\xDF\x86\x82\x87\x01aR\xB0V[\x92PP`@\x84\x015\x90P\x92P\x92P\x92V[`\0``\x82\x84\x03\x12\x15aV\x02W`\0\x80\xFD[aK\\\x83\x83aJ\x93V[` \x80\x82R`\x19\x90\x82\x01R\x7FPausable: index is paused\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x825`^\x19\x836\x03\x01\x81\x12aVoW`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aV\x90W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aV\xAAW`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15aFbW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15aV\xECWaV\xECaV\xC2V[P`\x01\x01\x90V[``\x81\x01\x825aW\x02\x81aIJV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x84\x015\x90aW\x1E\x82aIJV[\x16` \x83\x01R`@\x83\x015aW2\x81aM\xD5V[c\xFF\xFF\xFF\xFF\x81\x16`@\x84\x01RP\x92\x91PPV[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017`\0\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x91\x90PV[`\0` \x82\x84\x03\x12\x15aW\x86W`\0\x80\xFD[\x81QaK\\\x81aIJV[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`7\x90\x82\x01R\x7FDelegationManager: onlyStrategyM`@\x82\x01R\x7FanagerOrEigenPodManager\0\0\0\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15aXJW`\0\x80\xFD[\x81QaK\\\x81aQ\xA4V[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[`\0\x825`\xDE\x19\x836\x03\x01\x81\x12aVoW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aX\xC5W`\0\x80\xFD[\x815aK\\\x81aQ\xA4V[`\0`\x01\x80`\xA0\x1B\x03\x80\x83Q\x16\x84R\x80` \x84\x01Q\x16` \x85\x01R\x80`@\x84\x01Q\x16`@\x85\x01RP``\x82\x01Q``\x84\x01Rc\xFF\xFF\xFF\xFF`\x80\x83\x01Q\x16`\x80\x84\x01R`\xA0\x82\x01Q`\xE0`\xA0\x85\x01RaY+`\xE0\x85\x01\x82aT\xFFV[\x90P`\xC0\x83\x01Q\x84\x82\x03`\xC0\x86\x01Ra\x12\x90\x82\x82aUCV[` \x81R`\0aK\\` \x83\x01\x84aX\xD0V[` \x81R`\0\x82Q`\xE0` \x84\x01RaYta\x01\0\x84\x01\x82aT\xFFV[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01RaY\x91\x82\x82aUCV[\x91PP`@\x84\x01Q`\x01\x80`\xA0\x1B\x03\x80\x82\x16``\x86\x01R``\x86\x01Q\x91P\x80\x82Q\x16`\x80\x86\x01RPk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16`\xA0\x85\x01RP`\x80\x84\x01QaY\xE9`\xC0\x85\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P`\xA0\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16`\xE0\x85\x01RP\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15aZ\x19W`\0\x80\xFD[\x82QaZ$\x81aQ\xA4V[` \x93\x90\x93\x01Q\x92\x94\x92\x93PPPV[\x82\x81R`@` \x82\x01R`\0aO\xD4`@\x83\x01\x84aX\xD0V[`\0` \x82\x84\x03\x12\x15aZ_W`\0\x80\xFD[PQ\x91\x90PV[`\0\x82`\x1F\x83\x01\x12aZwW`\0\x80\xFD[\x81Q` aZ\x87aN6\x83aM\xF2V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aZ\xA6W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aN~W\x80Q\x83R\x91\x83\x01\x91\x83\x01aZ\xAAV[`\0\x80`@\x83\x85\x03\x12\x15aZ\xD4W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aZ\xEBW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12aZ\xFFW`\0\x80\xFD[\x81Q` a[\x0FaN6\x83aM\xF2V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a[.W`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a[UW\x85Qa[F\x81aIJV[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a[3V[\x91\x88\x01Q\x91\x96P\x90\x93PPP\x80\x82\x11\x15a[nW`\0\x80\xFD[Pa[{\x85\x82\x86\x01aZfV[\x91PP\x92P\x92\x90PV[`\0\x82\x19\x82\x11\x15a[\x98Wa[\x98aV\xC2V[P\x01\x90V[`\0\x82\x82\x10\x15a[\xAFWa[\xAFaV\xC2V[P\x03\x90V[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[`\0` \x82\x84\x03\x12\x15a[\xEAW`\0\x80\xFD[\x815aK\\\x81aM\xD5V[\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\x815a\\ \x81aIJV[a\\*\x81\x83a[\xF5V[P`\x01\x81\x01` \x83\x015a\\=\x81aIJV[a\\G\x81\x83a[\xF5V[P`@\x83\x015a\\V\x81aM\xD5V[\x81Tc\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x16`\xA0\x91\x90\x91\x1Bc\xFF\xFF\xFF\xFF`\xA0\x1B\x16\x17\x90UPPV[`\0a\\\x846\x83aN\xE4V[\x92\x91PPV[\x82\x81R`\0` `@\x81\x84\x01R\x83Q\x80`@\x85\x01R`\0[\x81\x81\x10\x15a\\\xBEW\x85\x81\x01\x83\x01Q\x85\x82\x01``\x01R\x82\x01a\\\xA2V[\x81\x81\x11\x15a\\\xD0W`\0``\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01``\x01\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\\\xF9W`\0\x80\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14aK\\W`\0\x80\xFD\xFEDelegationManager.registerOperatDelegationManager.completeQueued\xA2dipfsX\"\x12 \x8E>\x01\xA8m\xFE\x84h\xFE\xC8\xCC\xFF\xD3\xBBky\xB5\x9D\xDB\x87Q\xA7mp\x93)\xF3N2\xB5\xFF@dsolcC\0\x08\x0C\x003";
    /// The deployed bytecode of the contract.
    pub static DELEGATIONMANAGER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct DelegationManager<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for DelegationManager<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for DelegationManager<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for DelegationManager<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for DelegationManager<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(DelegationManager))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DelegationManager<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    DELEGATIONMANAGER_ABI.clone(),
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
                DELEGATIONMANAGER_ABI.clone(),
                DELEGATIONMANAGER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `DELEGATION_APPROVAL_TYPEHASH` (0x04a4f979) function
        pub fn delegation_approval_typehash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([4, 164, 249, 121], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `DOMAIN_TYPEHASH` (0x20606b70) function
        pub fn domain_typehash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([32, 96, 107, 112], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MAX_STAKER_OPT_OUT_WINDOW_BLOCKS` (0x4fc40b61) function
        pub fn max_staker_opt_out_window_blocks(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([79, 196, 11, 97], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MAX_WITHDRAWAL_DELAY_BLOCKS` (0xca661c04) function
        pub fn max_withdrawal_delay_blocks(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([202, 102, 28, 4], ())
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
        ///Calls the contract's `STAKER_DELEGATION_TYPEHASH` (0x43377382) function
        pub fn staker_delegation_typehash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([67, 55, 115, 130], ())
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
        ///Calls the contract's `beaconChainETHStrategy` (0x9104c319) function
        pub fn beacon_chain_eth_strategy(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([145, 4, 195, 25], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateCurrentStakerDelegationDigestHash` (0x1bbce091) function
        pub fn calculate_current_staker_delegation_digest_hash(
            &self,
            staker: ::ethers::core::types::Address,
            operator: ::ethers::core::types::Address,
            expiry: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([27, 188, 224, 145], (staker, operator, expiry))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateDelegationApprovalDigestHash` (0x0b9f487a) function
        pub fn calculate_delegation_approval_digest_hash(
            &self,
            staker: ::ethers::core::types::Address,
            operator: ::ethers::core::types::Address,
            delegation_approver: ::ethers::core::types::Address,
            approver_salt: [u8; 32],
            expiry: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [11, 159, 72, 122],
                    (staker, operator, delegation_approver, approver_salt, expiry),
                )
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
        ///Calls the contract's `calculateStakerDelegationDigestHash` (0xc94b5111) function
        pub fn calculate_staker_delegation_digest_hash(
            &self,
            staker: ::ethers::core::types::Address,
            staker_nonce: ::ethers::core::types::U256,
            operator: ::ethers::core::types::Address,
            expiry: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([201, 75, 81, 17], (staker, staker_nonce, operator, expiry))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `calculateWithdrawalRoot` (0x597b36da) function
        pub fn calculate_withdrawal_root(
            &self,
            withdrawal: Withdrawal,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([89, 123, 54, 218], (withdrawal,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `completeQueuedWithdrawal` (0x60d7faed) function
        pub fn complete_queued_withdrawal(
            &self,
            withdrawal: Withdrawal,
            tokens: ::std::vec::Vec<::ethers::core::types::Address>,
            middleware_times_index: ::ethers::core::types::U256,
            receive_as_tokens: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [96, 215, 250, 237],
                    (withdrawal, tokens, middleware_times_index, receive_as_tokens),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `completeQueuedWithdrawals` (0x33404396) function
        pub fn complete_queued_withdrawals(
            &self,
            withdrawals: ::std::vec::Vec<Withdrawal>,
            tokens: ::std::vec::Vec<::std::vec::Vec<::ethers::core::types::Address>>,
            middleware_times_indexes: ::std::vec::Vec<::ethers::core::types::U256>,
            receive_as_tokens: ::std::vec::Vec<bool>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [51, 64, 67, 150],
                    (withdrawals, tokens, middleware_times_indexes, receive_as_tokens),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `cumulativeWithdrawalsQueued` (0xa1788484) function
        pub fn cumulative_withdrawals_queued(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([161, 120, 132, 132], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decreaseDelegatedShares` (0x132d4967) function
        pub fn decrease_delegated_shares(
            &self,
            staker: ::ethers::core::types::Address,
            strategy: ::ethers::core::types::Address,
            shares: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([19, 45, 73, 103], (staker, strategy, shares))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `delegateTo` (0xeea9064b) function
        pub fn delegate_to(
            &self,
            operator: ::ethers::core::types::Address,
            approver_signature_and_expiry: SignatureWithExpiry,
            approver_salt: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [238, 169, 6, 75],
                    (operator, approver_signature_and_expiry, approver_salt),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `delegateToBySignature` (0x7f548071) function
        pub fn delegate_to_by_signature(
            &self,
            staker: ::ethers::core::types::Address,
            operator: ::ethers::core::types::Address,
            staker_signature_and_expiry: SignatureWithExpiry,
            approver_signature_and_expiry: SignatureWithExpiry,
            approver_salt: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [127, 84, 128, 113],
                    (
                        staker,
                        operator,
                        staker_signature_and_expiry,
                        approver_signature_and_expiry,
                        approver_salt,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `delegatedTo` (0x65da1264) function
        pub fn delegated_to(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([101, 218, 18, 100], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `delegationApprover` (0x3cdeb5e0) function
        pub fn delegation_approver(
            &self,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([60, 222, 181, 224], operator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `delegationApproverSaltIsSpent` (0xbb45fef2) function
        pub fn delegation_approver_salt_is_spent(
            &self,
            p0: ::ethers::core::types::Address,
            p1: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([187, 69, 254, 242], (p0, p1))
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
        ///Calls the contract's `domainSeparator` (0xf698da25) function
        pub fn domain_separator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([246, 152, 218, 37], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `earningsReceiver` (0x5f966f14) function
        pub fn earnings_receiver(
            &self,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([95, 150, 111, 20], operator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `eigenPodManager` (0x4665bcda) function
        pub fn eigen_pod_manager(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([70, 101, 188, 218], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDelegatableShares` (0xcf80873e) function
        pub fn get_delegatable_shares(
            &self,
            staker: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::std::vec::Vec<::ethers::core::types::Address>,
                ::std::vec::Vec<::ethers::core::types::U256>,
            ),
        > {
            self.0
                .method_hash([207, 128, 135, 62], staker)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `increaseDelegatedShares` (0x28a573ae) function
        pub fn increase_delegated_shares(
            &self,
            staker: ::ethers::core::types::Address,
            strategy: ::ethers::core::types::Address,
            shares: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([40, 165, 115, 174], (staker, strategy, shares))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0xeb990c59) function
        pub fn initialize(
            &self,
            initial_owner: ::ethers::core::types::Address,
            pauser_registry: ::ethers::core::types::Address,
            initial_paused_status: ::ethers::core::types::U256,
            withdrawal_delay_blocks: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [235, 153, 12, 89],
                    (
                        initial_owner,
                        pauser_registry,
                        initial_paused_status,
                        withdrawal_delay_blocks,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isDelegated` (0x3e28391d) function
        pub fn is_delegated(
            &self,
            staker: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([62, 40, 57, 29], staker)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isOperator` (0x6d70f7ae) function
        pub fn is_operator(
            &self,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([109, 112, 247, 174], operator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `migrateQueuedWithdrawals` (0x5cfe8d2c) function
        pub fn migrate_queued_withdrawals(
            &self,
            withdrawals_to_migrate: ::std::vec::Vec<DeprecatedStructQueuedWithdrawal>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([92, 254, 141, 44], withdrawals_to_migrate)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `modifyOperatorDetails` (0xf16172b0) function
        pub fn modify_operator_details(
            &self,
            new_operator_details: OperatorDetails,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([241, 97, 114, 176], (new_operator_details,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operatorDetails` (0xc5e480db) function
        pub fn operator_details(
            &self,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, OperatorDetails> {
            self.0
                .method_hash([197, 228, 128, 219], operator)
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
        ///Calls the contract's `operatorShares` (0x778e55f3) function
        pub fn operator_shares(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([119, 142, 85, 243], (p0, p1))
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
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([136, 111, 17, 149], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pendingWithdrawals` (0xb7f06ebe) function
        pub fn pending_withdrawals(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([183, 240, 110, 190], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `queueWithdrawals` (0x0dd8dd02) function
        pub fn queue_withdrawals(
            &self,
            queued_withdrawal_params: ::std::vec::Vec<QueuedWithdrawalParams>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 32]>> {
            self.0
                .method_hash([13, 216, 221, 2], queued_withdrawal_params)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerAsOperator` (0x0f589e59) function
        pub fn register_as_operator(
            &self,
            registering_operator_details: OperatorDetails,
            metadata_uri: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [15, 88, 158, 89],
                    (registering_operator_details, metadata_uri),
                )
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
        ///Calls the contract's `setPauserRegistry` (0x10d67a2f) function
        pub fn set_pauser_registry(
            &self,
            new_pauser_registry: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([16, 214, 122, 47], new_pauser_registry)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `slasher` (0xb1344271) function
        pub fn slasher(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([177, 52, 66, 113], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stakerNonce` (0x29c77d4f) function
        pub fn staker_nonce(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([41, 199, 125, 79], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stakerOptOutWindowBlocks` (0x16928365) function
        pub fn staker_opt_out_window_blocks(
            &self,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([22, 146, 131, 101], operator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `strategyManager` (0x39b70e38) function
        pub fn strategy_manager(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([57, 183, 14, 56], ())
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
        ///Calls the contract's `undelegate` (0xda8be864) function
        pub fn undelegate(
            &self,
            staker: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 32]>> {
            self.0
                .method_hash([218, 139, 232, 100], staker)
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
        ///Calls the contract's `updateOperatorMetadataURI` (0x99be81c8) function
        pub fn update_operator_metadata_uri(
            &self,
            metadata_uri: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([153, 190, 129, 200], metadata_uri)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawalDelayBlocks` (0x50f73e7c) function
        pub fn withdrawal_delay_blocks(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([80, 247, 62, 124], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AVSMetadataURIUpdated` event
        pub fn avs_metadata_uri_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AvsmetadataURIUpdatedFilter,
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
        ///Gets the contract's `OperatorDetailsModified` event
        pub fn operator_details_modified_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OperatorDetailsModifiedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OperatorMetadataURIUpdated` event
        pub fn operator_metadata_uri_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OperatorMetadataURIUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OperatorRegistered` event
        pub fn operator_registered_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OperatorRegisteredFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OperatorSharesDecreased` event
        pub fn operator_shares_decreased_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OperatorSharesDecreasedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OperatorSharesIncreased` event
        pub fn operator_shares_increased_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OperatorSharesIncreasedFilter,
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
        ///Gets the contract's `Paused` event
        pub fn paused_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PausedFilter> {
            self.0.event()
        }
        ///Gets the contract's `PauserRegistrySet` event
        pub fn pauser_registry_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PauserRegistrySetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `StakerDelegated` event
        pub fn staker_delegated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StakerDelegatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `StakerForceUndelegated` event
        pub fn staker_force_undelegated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StakerForceUndelegatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `StakerUndelegated` event
        pub fn staker_undelegated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StakerUndelegatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Unpaused` event
        pub fn unpaused_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UnpausedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `WithdrawalCompleted` event
        pub fn withdrawal_completed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            WithdrawalCompletedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `WithdrawalDelayBlocksSet` event
        pub fn withdrawal_delay_blocks_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            WithdrawalDelayBlocksSetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `WithdrawalMigrated` event
        pub fn withdrawal_migrated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            WithdrawalMigratedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `WithdrawalQueued` event
        pub fn withdrawal_queued_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            WithdrawalQueuedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DelegationManagerEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for DelegationManager<M> {
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
        Hash
    )]
    #[ethevent(
        name = "OperatorDetailsModified",
        abi = "OperatorDetailsModified(address,(address,address,uint32))"
    )]
    pub struct OperatorDetailsModifiedFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        pub new_operator_details: OperatorDetails,
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
        name = "OperatorMetadataURIUpdated",
        abi = "OperatorMetadataURIUpdated(address,string)"
    )]
    pub struct OperatorMetadataURIUpdatedFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
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
        Hash
    )]
    #[ethevent(
        name = "OperatorRegistered",
        abi = "OperatorRegistered(address,(address,address,uint32))"
    )]
    pub struct OperatorRegisteredFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        pub operator_details: OperatorDetails,
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
        name = "OperatorSharesDecreased",
        abi = "OperatorSharesDecreased(address,address,address,uint256)"
    )]
    pub struct OperatorSharesDecreasedFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        pub staker: ::ethers::core::types::Address,
        pub strategy: ::ethers::core::types::Address,
        pub shares: ::ethers::core::types::U256,
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
        name = "OperatorSharesIncreased",
        abi = "OperatorSharesIncreased(address,address,address,uint256)"
    )]
    pub struct OperatorSharesIncreasedFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        pub staker: ::ethers::core::types::Address,
        pub strategy: ::ethers::core::types::Address,
        pub shares: ::ethers::core::types::U256,
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
        Hash
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
        Hash
    )]
    #[ethevent(name = "StakerDelegated", abi = "StakerDelegated(address,address)")]
    pub struct StakerDelegatedFilter {
        #[ethevent(indexed)]
        pub staker: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
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
        name = "StakerForceUndelegated",
        abi = "StakerForceUndelegated(address,address)"
    )]
    pub struct StakerForceUndelegatedFilter {
        #[ethevent(indexed)]
        pub staker: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
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
    #[ethevent(name = "StakerUndelegated", abi = "StakerUndelegated(address,address)")]
    pub struct StakerUndelegatedFilter {
        #[ethevent(indexed)]
        pub staker: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
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
    #[ethevent(name = "Unpaused", abi = "Unpaused(address,uint256)")]
    pub struct UnpausedFilter {
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
        Hash
    )]
    #[ethevent(name = "WithdrawalCompleted", abi = "WithdrawalCompleted(bytes32)")]
    pub struct WithdrawalCompletedFilter {
        pub withdrawal_root: [u8; 32],
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
        name = "WithdrawalDelayBlocksSet",
        abi = "WithdrawalDelayBlocksSet(uint256,uint256)"
    )]
    pub struct WithdrawalDelayBlocksSetFilter {
        pub previous_value: ::ethers::core::types::U256,
        pub new_value: ::ethers::core::types::U256,
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
    #[ethevent(name = "WithdrawalMigrated", abi = "WithdrawalMigrated(bytes32,bytes32)")]
    pub struct WithdrawalMigratedFilter {
        pub old_withdrawal_root: [u8; 32],
        pub new_withdrawal_root: [u8; 32],
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
        name = "WithdrawalQueued",
        abi = "WithdrawalQueued(bytes32,(address,address,address,uint256,uint32,address[],uint256[]))"
    )]
    pub struct WithdrawalQueuedFilter {
        pub withdrawal_root: [u8; 32],
        pub withdrawal: Withdrawal,
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
    pub enum DelegationManagerEvents {
        AvsmetadataURIUpdatedFilter(AvsmetadataURIUpdatedFilter),
        InitializedFilter(InitializedFilter),
        OperatorAVSRegistrationStatusUpdatedFilter(
            OperatorAVSRegistrationStatusUpdatedFilter,
        ),
        OperatorDetailsModifiedFilter(OperatorDetailsModifiedFilter),
        OperatorMetadataURIUpdatedFilter(OperatorMetadataURIUpdatedFilter),
        OperatorRegisteredFilter(OperatorRegisteredFilter),
        OperatorSharesDecreasedFilter(OperatorSharesDecreasedFilter),
        OperatorSharesIncreasedFilter(OperatorSharesIncreasedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PausedFilter(PausedFilter),
        PauserRegistrySetFilter(PauserRegistrySetFilter),
        StakerDelegatedFilter(StakerDelegatedFilter),
        StakerForceUndelegatedFilter(StakerForceUndelegatedFilter),
        StakerUndelegatedFilter(StakerUndelegatedFilter),
        UnpausedFilter(UnpausedFilter),
        WithdrawalCompletedFilter(WithdrawalCompletedFilter),
        WithdrawalDelayBlocksSetFilter(WithdrawalDelayBlocksSetFilter),
        WithdrawalMigratedFilter(WithdrawalMigratedFilter),
        WithdrawalQueuedFilter(WithdrawalQueuedFilter),
    }
    impl ::ethers::contract::EthLogDecode for DelegationManagerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AvsmetadataURIUpdatedFilter::decode_log(log) {
                return Ok(DelegationManagerEvents::AvsmetadataURIUpdatedFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(DelegationManagerEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OperatorAVSRegistrationStatusUpdatedFilter::decode_log(
                log,
            ) {
                return Ok(
                    DelegationManagerEvents::OperatorAVSRegistrationStatusUpdatedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = OperatorDetailsModifiedFilter::decode_log(log) {
                return Ok(
                    DelegationManagerEvents::OperatorDetailsModifiedFilter(decoded),
                );
            }
            if let Ok(decoded) = OperatorMetadataURIUpdatedFilter::decode_log(log) {
                return Ok(
                    DelegationManagerEvents::OperatorMetadataURIUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = OperatorRegisteredFilter::decode_log(log) {
                return Ok(DelegationManagerEvents::OperatorRegisteredFilter(decoded));
            }
            if let Ok(decoded) = OperatorSharesDecreasedFilter::decode_log(log) {
                return Ok(
                    DelegationManagerEvents::OperatorSharesDecreasedFilter(decoded),
                );
            }
            if let Ok(decoded) = OperatorSharesIncreasedFilter::decode_log(log) {
                return Ok(
                    DelegationManagerEvents::OperatorSharesIncreasedFilter(decoded),
                );
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(DelegationManagerEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(DelegationManagerEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = PauserRegistrySetFilter::decode_log(log) {
                return Ok(DelegationManagerEvents::PauserRegistrySetFilter(decoded));
            }
            if let Ok(decoded) = StakerDelegatedFilter::decode_log(log) {
                return Ok(DelegationManagerEvents::StakerDelegatedFilter(decoded));
            }
            if let Ok(decoded) = StakerForceUndelegatedFilter::decode_log(log) {
                return Ok(
                    DelegationManagerEvents::StakerForceUndelegatedFilter(decoded),
                );
            }
            if let Ok(decoded) = StakerUndelegatedFilter::decode_log(log) {
                return Ok(DelegationManagerEvents::StakerUndelegatedFilter(decoded));
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(DelegationManagerEvents::UnpausedFilter(decoded));
            }
            if let Ok(decoded) = WithdrawalCompletedFilter::decode_log(log) {
                return Ok(DelegationManagerEvents::WithdrawalCompletedFilter(decoded));
            }
            if let Ok(decoded) = WithdrawalDelayBlocksSetFilter::decode_log(log) {
                return Ok(
                    DelegationManagerEvents::WithdrawalDelayBlocksSetFilter(decoded),
                );
            }
            if let Ok(decoded) = WithdrawalMigratedFilter::decode_log(log) {
                return Ok(DelegationManagerEvents::WithdrawalMigratedFilter(decoded));
            }
            if let Ok(decoded) = WithdrawalQueuedFilter::decode_log(log) {
                return Ok(DelegationManagerEvents::WithdrawalQueuedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for DelegationManagerEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AvsmetadataURIUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorAVSRegistrationStatusUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorDetailsModifiedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorMetadataURIUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorRegisteredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorSharesDecreasedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorSharesIncreasedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PausedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauserRegistrySetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StakerDelegatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StakerForceUndelegatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StakerUndelegatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnpausedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawalCompletedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawalDelayBlocksSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawalMigratedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawalQueuedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AvsmetadataURIUpdatedFilter> for DelegationManagerEvents {
        fn from(value: AvsmetadataURIUpdatedFilter) -> Self {
            Self::AvsmetadataURIUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for DelegationManagerEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OperatorAVSRegistrationStatusUpdatedFilter>
    for DelegationManagerEvents {
        fn from(value: OperatorAVSRegistrationStatusUpdatedFilter) -> Self {
            Self::OperatorAVSRegistrationStatusUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<OperatorDetailsModifiedFilter>
    for DelegationManagerEvents {
        fn from(value: OperatorDetailsModifiedFilter) -> Self {
            Self::OperatorDetailsModifiedFilter(value)
        }
    }
    impl ::core::convert::From<OperatorMetadataURIUpdatedFilter>
    for DelegationManagerEvents {
        fn from(value: OperatorMetadataURIUpdatedFilter) -> Self {
            Self::OperatorMetadataURIUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<OperatorRegisteredFilter> for DelegationManagerEvents {
        fn from(value: OperatorRegisteredFilter) -> Self {
            Self::OperatorRegisteredFilter(value)
        }
    }
    impl ::core::convert::From<OperatorSharesDecreasedFilter>
    for DelegationManagerEvents {
        fn from(value: OperatorSharesDecreasedFilter) -> Self {
            Self::OperatorSharesDecreasedFilter(value)
        }
    }
    impl ::core::convert::From<OperatorSharesIncreasedFilter>
    for DelegationManagerEvents {
        fn from(value: OperatorSharesIncreasedFilter) -> Self {
            Self::OperatorSharesIncreasedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for DelegationManagerEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<PausedFilter> for DelegationManagerEvents {
        fn from(value: PausedFilter) -> Self {
            Self::PausedFilter(value)
        }
    }
    impl ::core::convert::From<PauserRegistrySetFilter> for DelegationManagerEvents {
        fn from(value: PauserRegistrySetFilter) -> Self {
            Self::PauserRegistrySetFilter(value)
        }
    }
    impl ::core::convert::From<StakerDelegatedFilter> for DelegationManagerEvents {
        fn from(value: StakerDelegatedFilter) -> Self {
            Self::StakerDelegatedFilter(value)
        }
    }
    impl ::core::convert::From<StakerForceUndelegatedFilter>
    for DelegationManagerEvents {
        fn from(value: StakerForceUndelegatedFilter) -> Self {
            Self::StakerForceUndelegatedFilter(value)
        }
    }
    impl ::core::convert::From<StakerUndelegatedFilter> for DelegationManagerEvents {
        fn from(value: StakerUndelegatedFilter) -> Self {
            Self::StakerUndelegatedFilter(value)
        }
    }
    impl ::core::convert::From<UnpausedFilter> for DelegationManagerEvents {
        fn from(value: UnpausedFilter) -> Self {
            Self::UnpausedFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawalCompletedFilter> for DelegationManagerEvents {
        fn from(value: WithdrawalCompletedFilter) -> Self {
            Self::WithdrawalCompletedFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawalDelayBlocksSetFilter>
    for DelegationManagerEvents {
        fn from(value: WithdrawalDelayBlocksSetFilter) -> Self {
            Self::WithdrawalDelayBlocksSetFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawalMigratedFilter> for DelegationManagerEvents {
        fn from(value: WithdrawalMigratedFilter) -> Self {
            Self::WithdrawalMigratedFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawalQueuedFilter> for DelegationManagerEvents {
        fn from(value: WithdrawalQueuedFilter) -> Self {
            Self::WithdrawalQueuedFilter(value)
        }
    }
    ///Container type for all input parameters for the `DELEGATION_APPROVAL_TYPEHASH` function with signature `DELEGATION_APPROVAL_TYPEHASH()` and selector `0x04a4f979`
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
        name = "DELEGATION_APPROVAL_TYPEHASH",
        abi = "DELEGATION_APPROVAL_TYPEHASH()"
    )]
    pub struct DelegationApprovalTypehashCall;
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
        Hash
    )]
    #[ethcall(name = "DOMAIN_TYPEHASH", abi = "DOMAIN_TYPEHASH()")]
    pub struct DomainTypehashCall;
    ///Container type for all input parameters for the `MAX_STAKER_OPT_OUT_WINDOW_BLOCKS` function with signature `MAX_STAKER_OPT_OUT_WINDOW_BLOCKS()` and selector `0x4fc40b61`
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
        name = "MAX_STAKER_OPT_OUT_WINDOW_BLOCKS",
        abi = "MAX_STAKER_OPT_OUT_WINDOW_BLOCKS()"
    )]
    pub struct MaxStakerOptOutWindowBlocksCall;
    ///Container type for all input parameters for the `MAX_WITHDRAWAL_DELAY_BLOCKS` function with signature `MAX_WITHDRAWAL_DELAY_BLOCKS()` and selector `0xca661c04`
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
        name = "MAX_WITHDRAWAL_DELAY_BLOCKS",
        abi = "MAX_WITHDRAWAL_DELAY_BLOCKS()"
    )]
    pub struct MaxWithdrawalDelayBlocksCall;
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
        Hash
    )]
    #[ethcall(
        name = "OPERATOR_AVS_REGISTRATION_TYPEHASH",
        abi = "OPERATOR_AVS_REGISTRATION_TYPEHASH()"
    )]
    pub struct OperatorAvsRegistrationTypehashCall;
    ///Container type for all input parameters for the `STAKER_DELEGATION_TYPEHASH` function with signature `STAKER_DELEGATION_TYPEHASH()` and selector `0x43377382`
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
    #[ethcall(name = "STAKER_DELEGATION_TYPEHASH", abi = "STAKER_DELEGATION_TYPEHASH()")]
    pub struct StakerDelegationTypehashCall;
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
        Hash
    )]
    #[ethcall(name = "avsOperatorStatus", abi = "avsOperatorStatus(address,address)")]
    pub struct AvsOperatorStatusCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
    ///Container type for all input parameters for the `beaconChainETHStrategy` function with signature `beaconChainETHStrategy()` and selector `0x9104c319`
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
    #[ethcall(name = "beaconChainETHStrategy", abi = "beaconChainETHStrategy()")]
    pub struct BeaconChainETHStrategyCall;
    ///Container type for all input parameters for the `calculateCurrentStakerDelegationDigestHash` function with signature `calculateCurrentStakerDelegationDigestHash(address,address,uint256)` and selector `0x1bbce091`
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
        name = "calculateCurrentStakerDelegationDigestHash",
        abi = "calculateCurrentStakerDelegationDigestHash(address,address,uint256)"
    )]
    pub struct CalculateCurrentStakerDelegationDigestHashCall {
        pub staker: ::ethers::core::types::Address,
        pub operator: ::ethers::core::types::Address,
        pub expiry: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `calculateDelegationApprovalDigestHash` function with signature `calculateDelegationApprovalDigestHash(address,address,address,bytes32,uint256)` and selector `0x0b9f487a`
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
        name = "calculateDelegationApprovalDigestHash",
        abi = "calculateDelegationApprovalDigestHash(address,address,address,bytes32,uint256)"
    )]
    pub struct CalculateDelegationApprovalDigestHashCall {
        pub staker: ::ethers::core::types::Address,
        pub operator: ::ethers::core::types::Address,
        pub delegation_approver: ::ethers::core::types::Address,
        pub approver_salt: [u8; 32],
        pub expiry: ::ethers::core::types::U256,
    }
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
        Hash
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
    ///Container type for all input parameters for the `calculateStakerDelegationDigestHash` function with signature `calculateStakerDelegationDigestHash(address,uint256,address,uint256)` and selector `0xc94b5111`
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
        name = "calculateStakerDelegationDigestHash",
        abi = "calculateStakerDelegationDigestHash(address,uint256,address,uint256)"
    )]
    pub struct CalculateStakerDelegationDigestHashCall {
        pub staker: ::ethers::core::types::Address,
        pub staker_nonce: ::ethers::core::types::U256,
        pub operator: ::ethers::core::types::Address,
        pub expiry: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `calculateWithdrawalRoot` function with signature `calculateWithdrawalRoot((address,address,address,uint256,uint32,address[],uint256[]))` and selector `0x597b36da`
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
        name = "calculateWithdrawalRoot",
        abi = "calculateWithdrawalRoot((address,address,address,uint256,uint32,address[],uint256[]))"
    )]
    pub struct CalculateWithdrawalRootCall {
        pub withdrawal: Withdrawal,
    }
    ///Container type for all input parameters for the `completeQueuedWithdrawal` function with signature `completeQueuedWithdrawal((address,address,address,uint256,uint32,address[],uint256[]),address[],uint256,bool)` and selector `0x60d7faed`
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
        name = "completeQueuedWithdrawal",
        abi = "completeQueuedWithdrawal((address,address,address,uint256,uint32,address[],uint256[]),address[],uint256,bool)"
    )]
    pub struct CompleteQueuedWithdrawalCall {
        pub withdrawal: Withdrawal,
        pub tokens: ::std::vec::Vec<::ethers::core::types::Address>,
        pub middleware_times_index: ::ethers::core::types::U256,
        pub receive_as_tokens: bool,
    }
    ///Container type for all input parameters for the `completeQueuedWithdrawals` function with signature `completeQueuedWithdrawals((address,address,address,uint256,uint32,address[],uint256[])[],address[][],uint256[],bool[])` and selector `0x33404396`
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
        name = "completeQueuedWithdrawals",
        abi = "completeQueuedWithdrawals((address,address,address,uint256,uint32,address[],uint256[])[],address[][],uint256[],bool[])"
    )]
    pub struct CompleteQueuedWithdrawalsCall {
        pub withdrawals: ::std::vec::Vec<Withdrawal>,
        pub tokens: ::std::vec::Vec<::std::vec::Vec<::ethers::core::types::Address>>,
        pub middleware_times_indexes: ::std::vec::Vec<::ethers::core::types::U256>,
        pub receive_as_tokens: ::std::vec::Vec<bool>,
    }
    ///Container type for all input parameters for the `cumulativeWithdrawalsQueued` function with signature `cumulativeWithdrawalsQueued(address)` and selector `0xa1788484`
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
        name = "cumulativeWithdrawalsQueued",
        abi = "cumulativeWithdrawalsQueued(address)"
    )]
    pub struct CumulativeWithdrawalsQueuedCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `decreaseDelegatedShares` function with signature `decreaseDelegatedShares(address,address,uint256)` and selector `0x132d4967`
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
        name = "decreaseDelegatedShares",
        abi = "decreaseDelegatedShares(address,address,uint256)"
    )]
    pub struct DecreaseDelegatedSharesCall {
        pub staker: ::ethers::core::types::Address,
        pub strategy: ::ethers::core::types::Address,
        pub shares: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `delegateTo` function with signature `delegateTo(address,(bytes,uint256),bytes32)` and selector `0xeea9064b`
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
    #[ethcall(name = "delegateTo", abi = "delegateTo(address,(bytes,uint256),bytes32)")]
    pub struct DelegateToCall {
        pub operator: ::ethers::core::types::Address,
        pub approver_signature_and_expiry: SignatureWithExpiry,
        pub approver_salt: [u8; 32],
    }
    ///Container type for all input parameters for the `delegateToBySignature` function with signature `delegateToBySignature(address,address,(bytes,uint256),(bytes,uint256),bytes32)` and selector `0x7f548071`
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
        name = "delegateToBySignature",
        abi = "delegateToBySignature(address,address,(bytes,uint256),(bytes,uint256),bytes32)"
    )]
    pub struct DelegateToBySignatureCall {
        pub staker: ::ethers::core::types::Address,
        pub operator: ::ethers::core::types::Address,
        pub staker_signature_and_expiry: SignatureWithExpiry,
        pub approver_signature_and_expiry: SignatureWithExpiry,
        pub approver_salt: [u8; 32],
    }
    ///Container type for all input parameters for the `delegatedTo` function with signature `delegatedTo(address)` and selector `0x65da1264`
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
    #[ethcall(name = "delegatedTo", abi = "delegatedTo(address)")]
    pub struct DelegatedToCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `delegationApprover` function with signature `delegationApprover(address)` and selector `0x3cdeb5e0`
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
    #[ethcall(name = "delegationApprover", abi = "delegationApprover(address)")]
    pub struct DelegationApproverCall {
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `delegationApproverSaltIsSpent` function with signature `delegationApproverSaltIsSpent(address,bytes32)` and selector `0xbb45fef2`
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
        name = "delegationApproverSaltIsSpent",
        abi = "delegationApproverSaltIsSpent(address,bytes32)"
    )]
    pub struct DelegationApproverSaltIsSpentCall(
        pub ::ethers::core::types::Address,
        pub [u8; 32],
    );
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
        Hash
    )]
    #[ethcall(name = "domainSeparator", abi = "domainSeparator()")]
    pub struct DomainSeparatorCall;
    ///Container type for all input parameters for the `earningsReceiver` function with signature `earningsReceiver(address)` and selector `0x5f966f14`
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
    #[ethcall(name = "earningsReceiver", abi = "earningsReceiver(address)")]
    pub struct EarningsReceiverCall {
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `eigenPodManager` function with signature `eigenPodManager()` and selector `0x4665bcda`
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
    #[ethcall(name = "eigenPodManager", abi = "eigenPodManager()")]
    pub struct EigenPodManagerCall;
    ///Container type for all input parameters for the `getDelegatableShares` function with signature `getDelegatableShares(address)` and selector `0xcf80873e`
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
    #[ethcall(name = "getDelegatableShares", abi = "getDelegatableShares(address)")]
    pub struct GetDelegatableSharesCall {
        pub staker: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `increaseDelegatedShares` function with signature `increaseDelegatedShares(address,address,uint256)` and selector `0x28a573ae`
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
        name = "increaseDelegatedShares",
        abi = "increaseDelegatedShares(address,address,uint256)"
    )]
    pub struct IncreaseDelegatedSharesCall {
        pub staker: ::ethers::core::types::Address,
        pub strategy: ::ethers::core::types::Address,
        pub shares: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address,uint256,uint256)` and selector `0xeb990c59`
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
    #[ethcall(name = "initialize", abi = "initialize(address,address,uint256,uint256)")]
    pub struct InitializeCall {
        pub initial_owner: ::ethers::core::types::Address,
        pub pauser_registry: ::ethers::core::types::Address,
        pub initial_paused_status: ::ethers::core::types::U256,
        pub withdrawal_delay_blocks: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `isDelegated` function with signature `isDelegated(address)` and selector `0x3e28391d`
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
    #[ethcall(name = "isDelegated", abi = "isDelegated(address)")]
    pub struct IsDelegatedCall {
        pub staker: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isOperator` function with signature `isOperator(address)` and selector `0x6d70f7ae`
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
    #[ethcall(name = "isOperator", abi = "isOperator(address)")]
    pub struct IsOperatorCall {
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `migrateQueuedWithdrawals` function with signature `migrateQueuedWithdrawals((address[],uint256[],address,(address,uint96),uint32,address)[])` and selector `0x5cfe8d2c`
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
        name = "migrateQueuedWithdrawals",
        abi = "migrateQueuedWithdrawals((address[],uint256[],address,(address,uint96),uint32,address)[])"
    )]
    pub struct MigrateQueuedWithdrawalsCall {
        pub withdrawals_to_migrate: ::std::vec::Vec<DeprecatedStructQueuedWithdrawal>,
    }
    ///Container type for all input parameters for the `modifyOperatorDetails` function with signature `modifyOperatorDetails((address,address,uint32))` and selector `0xf16172b0`
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
        name = "modifyOperatorDetails",
        abi = "modifyOperatorDetails((address,address,uint32))"
    )]
    pub struct ModifyOperatorDetailsCall {
        pub new_operator_details: OperatorDetails,
    }
    ///Container type for all input parameters for the `operatorDetails` function with signature `operatorDetails(address)` and selector `0xc5e480db`
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
    #[ethcall(name = "operatorDetails", abi = "operatorDetails(address)")]
    pub struct OperatorDetailsCall {
        pub operator: ::ethers::core::types::Address,
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
        Hash
    )]
    #[ethcall(
        name = "operatorSaltIsSpent",
        abi = "operatorSaltIsSpent(address,bytes32)"
    )]
    pub struct OperatorSaltIsSpentCall(pub ::ethers::core::types::Address, pub [u8; 32]);
    ///Container type for all input parameters for the `operatorShares` function with signature `operatorShares(address,address)` and selector `0x778e55f3`
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
    #[ethcall(name = "operatorShares", abi = "operatorShares(address,address)")]
    pub struct OperatorSharesCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
    )]
    #[ethcall(name = "pauserRegistry", abi = "pauserRegistry()")]
    pub struct PauserRegistryCall;
    ///Container type for all input parameters for the `pendingWithdrawals` function with signature `pendingWithdrawals(bytes32)` and selector `0xb7f06ebe`
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
    #[ethcall(name = "pendingWithdrawals", abi = "pendingWithdrawals(bytes32)")]
    pub struct PendingWithdrawalsCall(pub [u8; 32]);
    ///Container type for all input parameters for the `queueWithdrawals` function with signature `queueWithdrawals((address[],uint256[],address)[])` and selector `0x0dd8dd02`
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
        name = "queueWithdrawals",
        abi = "queueWithdrawals((address[],uint256[],address)[])"
    )]
    pub struct QueueWithdrawalsCall {
        pub queued_withdrawal_params: ::std::vec::Vec<QueuedWithdrawalParams>,
    }
    ///Container type for all input parameters for the `registerAsOperator` function with signature `registerAsOperator((address,address,uint32),string)` and selector `0x0f589e59`
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
        name = "registerAsOperator",
        abi = "registerAsOperator((address,address,uint32),string)"
    )]
    pub struct RegisterAsOperatorCall {
        pub registering_operator_details: OperatorDetails,
        pub metadata_uri: ::std::string::String,
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
        Hash
    )]
    #[ethcall(name = "setPauserRegistry", abi = "setPauserRegistry(address)")]
    pub struct SetPauserRegistryCall {
        pub new_pauser_registry: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `slasher` function with signature `slasher()` and selector `0xb1344271`
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
    #[ethcall(name = "slasher", abi = "slasher()")]
    pub struct SlasherCall;
    ///Container type for all input parameters for the `stakerNonce` function with signature `stakerNonce(address)` and selector `0x29c77d4f`
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
    #[ethcall(name = "stakerNonce", abi = "stakerNonce(address)")]
    pub struct StakerNonceCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `stakerOptOutWindowBlocks` function with signature `stakerOptOutWindowBlocks(address)` and selector `0x16928365`
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
        name = "stakerOptOutWindowBlocks",
        abi = "stakerOptOutWindowBlocks(address)"
    )]
    pub struct StakerOptOutWindowBlocksCall {
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `strategyManager` function with signature `strategyManager()` and selector `0x39b70e38`
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
    #[ethcall(name = "strategyManager", abi = "strategyManager()")]
    pub struct StrategyManagerCall;
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
    ///Container type for all input parameters for the `undelegate` function with signature `undelegate(address)` and selector `0xda8be864`
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
    #[ethcall(name = "undelegate", abi = "undelegate(address)")]
    pub struct UndelegateCall {
        pub staker: ::ethers::core::types::Address,
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
        Hash
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
        Hash
    )]
    #[ethcall(name = "updateAVSMetadataURI", abi = "updateAVSMetadataURI(string)")]
    pub struct UpdateAVSMetadataURICall {
        pub metadata_uri: ::std::string::String,
    }
    ///Container type for all input parameters for the `updateOperatorMetadataURI` function with signature `updateOperatorMetadataURI(string)` and selector `0x99be81c8`
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
        name = "updateOperatorMetadataURI",
        abi = "updateOperatorMetadataURI(string)"
    )]
    pub struct UpdateOperatorMetadataURICall {
        pub metadata_uri: ::std::string::String,
    }
    ///Container type for all input parameters for the `withdrawalDelayBlocks` function with signature `withdrawalDelayBlocks()` and selector `0x50f73e7c`
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
    #[ethcall(name = "withdrawalDelayBlocks", abi = "withdrawalDelayBlocks()")]
    pub struct WithdrawalDelayBlocksCall;
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
    pub enum DelegationManagerCalls {
        DelegationApprovalTypehash(DelegationApprovalTypehashCall),
        DomainTypehash(DomainTypehashCall),
        MaxStakerOptOutWindowBlocks(MaxStakerOptOutWindowBlocksCall),
        MaxWithdrawalDelayBlocks(MaxWithdrawalDelayBlocksCall),
        OperatorAvsRegistrationTypehash(OperatorAvsRegistrationTypehashCall),
        StakerDelegationTypehash(StakerDelegationTypehashCall),
        AvsOperatorStatus(AvsOperatorStatusCall),
        BeaconChainETHStrategy(BeaconChainETHStrategyCall),
        CalculateCurrentStakerDelegationDigestHash(
            CalculateCurrentStakerDelegationDigestHashCall,
        ),
        CalculateDelegationApprovalDigestHash(CalculateDelegationApprovalDigestHashCall),
        CalculateOperatorAVSRegistrationDigestHash(
            CalculateOperatorAVSRegistrationDigestHashCall,
        ),
        CalculateStakerDelegationDigestHash(CalculateStakerDelegationDigestHashCall),
        CalculateWithdrawalRoot(CalculateWithdrawalRootCall),
        CompleteQueuedWithdrawal(CompleteQueuedWithdrawalCall),
        CompleteQueuedWithdrawals(CompleteQueuedWithdrawalsCall),
        CumulativeWithdrawalsQueued(CumulativeWithdrawalsQueuedCall),
        DecreaseDelegatedShares(DecreaseDelegatedSharesCall),
        DelegateTo(DelegateToCall),
        DelegateToBySignature(DelegateToBySignatureCall),
        DelegatedTo(DelegatedToCall),
        DelegationApprover(DelegationApproverCall),
        DelegationApproverSaltIsSpent(DelegationApproverSaltIsSpentCall),
        DeregisterOperatorFromAVS(DeregisterOperatorFromAVSCall),
        DomainSeparator(DomainSeparatorCall),
        EarningsReceiver(EarningsReceiverCall),
        EigenPodManager(EigenPodManagerCall),
        GetDelegatableShares(GetDelegatableSharesCall),
        IncreaseDelegatedShares(IncreaseDelegatedSharesCall),
        Initialize(InitializeCall),
        IsDelegated(IsDelegatedCall),
        IsOperator(IsOperatorCall),
        MigrateQueuedWithdrawals(MigrateQueuedWithdrawalsCall),
        ModifyOperatorDetails(ModifyOperatorDetailsCall),
        OperatorDetails(OperatorDetailsCall),
        OperatorSaltIsSpent(OperatorSaltIsSpentCall),
        OperatorShares(OperatorSharesCall),
        Owner(OwnerCall),
        Pause(PauseCall),
        PauseAll(PauseAllCall),
        PausedWithIndex(PausedWithIndexCall),
        Paused(PausedCall),
        PauserRegistry(PauserRegistryCall),
        PendingWithdrawals(PendingWithdrawalsCall),
        QueueWithdrawals(QueueWithdrawalsCall),
        RegisterAsOperator(RegisterAsOperatorCall),
        RegisterOperatorToAVS(RegisterOperatorToAVSCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetPauserRegistry(SetPauserRegistryCall),
        Slasher(SlasherCall),
        StakerNonce(StakerNonceCall),
        StakerOptOutWindowBlocks(StakerOptOutWindowBlocksCall),
        StrategyManager(StrategyManagerCall),
        TransferOwnership(TransferOwnershipCall),
        Undelegate(UndelegateCall),
        Unpause(UnpauseCall),
        UpdateAVSMetadataURI(UpdateAVSMetadataURICall),
        UpdateOperatorMetadataURI(UpdateOperatorMetadataURICall),
        WithdrawalDelayBlocks(WithdrawalDelayBlocksCall),
    }
    impl ::ethers::core::abi::AbiDecode for DelegationManagerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DelegationApprovalTypehashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DelegationApprovalTypehash(decoded));
            }
            if let Ok(decoded) = <DomainTypehashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DomainTypehash(decoded));
            }
            if let Ok(decoded) = <MaxStakerOptOutWindowBlocksCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxStakerOptOutWindowBlocks(decoded));
            }
            if let Ok(decoded) = <MaxWithdrawalDelayBlocksCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxWithdrawalDelayBlocks(decoded));
            }
            if let Ok(decoded) = <OperatorAvsRegistrationTypehashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OperatorAvsRegistrationTypehash(decoded));
            }
            if let Ok(decoded) = <StakerDelegationTypehashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StakerDelegationTypehash(decoded));
            }
            if let Ok(decoded) = <AvsOperatorStatusCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AvsOperatorStatus(decoded));
            }
            if let Ok(decoded) = <BeaconChainETHStrategyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BeaconChainETHStrategy(decoded));
            }
            if let Ok(decoded) = <CalculateCurrentStakerDelegationDigestHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateCurrentStakerDelegationDigestHash(decoded));
            }
            if let Ok(decoded) = <CalculateDelegationApprovalDigestHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateDelegationApprovalDigestHash(decoded));
            }
            if let Ok(decoded) = <CalculateOperatorAVSRegistrationDigestHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateOperatorAVSRegistrationDigestHash(decoded));
            }
            if let Ok(decoded) = <CalculateStakerDelegationDigestHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateStakerDelegationDigestHash(decoded));
            }
            if let Ok(decoded) = <CalculateWithdrawalRootCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateWithdrawalRoot(decoded));
            }
            if let Ok(decoded) = <CompleteQueuedWithdrawalCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CompleteQueuedWithdrawal(decoded));
            }
            if let Ok(decoded) = <CompleteQueuedWithdrawalsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CompleteQueuedWithdrawals(decoded));
            }
            if let Ok(decoded) = <CumulativeWithdrawalsQueuedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CumulativeWithdrawalsQueued(decoded));
            }
            if let Ok(decoded) = <DecreaseDelegatedSharesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DecreaseDelegatedShares(decoded));
            }
            if let Ok(decoded) = <DelegateToCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DelegateTo(decoded));
            }
            if let Ok(decoded) = <DelegateToBySignatureCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DelegateToBySignature(decoded));
            }
            if let Ok(decoded) = <DelegatedToCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DelegatedTo(decoded));
            }
            if let Ok(decoded) = <DelegationApproverCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DelegationApprover(decoded));
            }
            if let Ok(decoded) = <DelegationApproverSaltIsSpentCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DelegationApproverSaltIsSpent(decoded));
            }
            if let Ok(decoded) = <DeregisterOperatorFromAVSCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeregisterOperatorFromAVS(decoded));
            }
            if let Ok(decoded) = <DomainSeparatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DomainSeparator(decoded));
            }
            if let Ok(decoded) = <EarningsReceiverCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EarningsReceiver(decoded));
            }
            if let Ok(decoded) = <EigenPodManagerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EigenPodManager(decoded));
            }
            if let Ok(decoded) = <GetDelegatableSharesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetDelegatableShares(decoded));
            }
            if let Ok(decoded) = <IncreaseDelegatedSharesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IncreaseDelegatedShares(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <IsDelegatedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsDelegated(decoded));
            }
            if let Ok(decoded) = <IsOperatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsOperator(decoded));
            }
            if let Ok(decoded) = <MigrateQueuedWithdrawalsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MigrateQueuedWithdrawals(decoded));
            }
            if let Ok(decoded) = <ModifyOperatorDetailsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ModifyOperatorDetails(decoded));
            }
            if let Ok(decoded) = <OperatorDetailsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OperatorDetails(decoded));
            }
            if let Ok(decoded) = <OperatorSaltIsSpentCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OperatorSaltIsSpent(decoded));
            }
            if let Ok(decoded) = <OperatorSharesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OperatorShares(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <PauseCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Pause(decoded));
            }
            if let Ok(decoded) = <PauseAllCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PauseAll(decoded));
            }
            if let Ok(decoded) = <PausedWithIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PausedWithIndex(decoded));
            }
            if let Ok(decoded) = <PausedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Paused(decoded));
            }
            if let Ok(decoded) = <PauserRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PauserRegistry(decoded));
            }
            if let Ok(decoded) = <PendingWithdrawalsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PendingWithdrawals(decoded));
            }
            if let Ok(decoded) = <QueueWithdrawalsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::QueueWithdrawals(decoded));
            }
            if let Ok(decoded) = <RegisterAsOperatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RegisterAsOperator(decoded));
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
            if let Ok(decoded) = <SetPauserRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetPauserRegistry(decoded));
            }
            if let Ok(decoded) = <SlasherCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Slasher(decoded));
            }
            if let Ok(decoded) = <StakerNonceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StakerNonce(decoded));
            }
            if let Ok(decoded) = <StakerOptOutWindowBlocksCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StakerOptOutWindowBlocks(decoded));
            }
            if let Ok(decoded) = <StrategyManagerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StrategyManager(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <UndelegateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Undelegate(decoded));
            }
            if let Ok(decoded) = <UnpauseCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Unpause(decoded));
            }
            if let Ok(decoded) = <UpdateAVSMetadataURICall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateAVSMetadataURI(decoded));
            }
            if let Ok(decoded) = <UpdateOperatorMetadataURICall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateOperatorMetadataURI(decoded));
            }
            if let Ok(decoded) = <WithdrawalDelayBlocksCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawalDelayBlocks(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DelegationManagerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DelegationApprovalTypehash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DomainTypehash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxStakerOptOutWindowBlocks(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxWithdrawalDelayBlocks(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OperatorAvsRegistrationTypehash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StakerDelegationTypehash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AvsOperatorStatus(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BeaconChainETHStrategy(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateCurrentStakerDelegationDigestHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateDelegationApprovalDigestHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateOperatorAVSRegistrationDigestHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateStakerDelegationDigestHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateWithdrawalRoot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CompleteQueuedWithdrawal(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CompleteQueuedWithdrawals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CumulativeWithdrawalsQueued(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DecreaseDelegatedShares(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DelegateTo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DelegateToBySignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DelegatedTo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DelegationApprover(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DelegationApproverSaltIsSpent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeregisterOperatorFromAVS(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DomainSeparator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EarningsReceiver(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EigenPodManager(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetDelegatableShares(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IncreaseDelegatedShares(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsDelegated(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MigrateQueuedWithdrawals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ModifyOperatorDetails(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OperatorDetails(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OperatorSaltIsSpent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OperatorShares(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PauseAll(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PausedWithIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Paused(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PauserRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PendingWithdrawals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QueueWithdrawals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterAsOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterOperatorToAVS(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPauserRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Slasher(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StakerNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StakerOptOutWindowBlocks(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StrategyManager(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Undelegate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unpause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateAVSMetadataURI(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateOperatorMetadataURI(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawalDelayBlocks(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for DelegationManagerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DelegationApprovalTypehash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DomainTypehash(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxStakerOptOutWindowBlocks(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MaxWithdrawalDelayBlocks(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorAvsRegistrationTypehash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StakerDelegationTypehash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AvsOperatorStatus(element) => ::core::fmt::Display::fmt(element, f),
                Self::BeaconChainETHStrategy(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateCurrentStakerDelegationDigestHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateDelegationApprovalDigestHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateOperatorAVSRegistrationDigestHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateStakerDelegationDigestHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateWithdrawalRoot(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CompleteQueuedWithdrawal(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CompleteQueuedWithdrawals(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CumulativeWithdrawalsQueued(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DecreaseDelegatedShares(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DelegateTo(element) => ::core::fmt::Display::fmt(element, f),
                Self::DelegateToBySignature(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DelegatedTo(element) => ::core::fmt::Display::fmt(element, f),
                Self::DelegationApprover(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DelegationApproverSaltIsSpent(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DeregisterOperatorFromAVS(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DomainSeparator(element) => ::core::fmt::Display::fmt(element, f),
                Self::EarningsReceiver(element) => ::core::fmt::Display::fmt(element, f),
                Self::EigenPodManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDelegatableShares(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IncreaseDelegatedShares(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsDelegated(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::MigrateQueuedWithdrawals(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ModifyOperatorDetails(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorDetails(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorSaltIsSpent(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorShares(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pause(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauseAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::PausedWithIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::Paused(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauserRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::PendingWithdrawals(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::QueueWithdrawals(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterAsOperator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RegisterOperatorToAVS(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPauserRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::Slasher(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakerNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakerOptOutWindowBlocks(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StrategyManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::Undelegate(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unpause(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateAVSMetadataURI(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateOperatorMetadataURI(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawalDelayBlocks(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<DelegationApprovalTypehashCall>
    for DelegationManagerCalls {
        fn from(value: DelegationApprovalTypehashCall) -> Self {
            Self::DelegationApprovalTypehash(value)
        }
    }
    impl ::core::convert::From<DomainTypehashCall> for DelegationManagerCalls {
        fn from(value: DomainTypehashCall) -> Self {
            Self::DomainTypehash(value)
        }
    }
    impl ::core::convert::From<MaxStakerOptOutWindowBlocksCall>
    for DelegationManagerCalls {
        fn from(value: MaxStakerOptOutWindowBlocksCall) -> Self {
            Self::MaxStakerOptOutWindowBlocks(value)
        }
    }
    impl ::core::convert::From<MaxWithdrawalDelayBlocksCall> for DelegationManagerCalls {
        fn from(value: MaxWithdrawalDelayBlocksCall) -> Self {
            Self::MaxWithdrawalDelayBlocks(value)
        }
    }
    impl ::core::convert::From<OperatorAvsRegistrationTypehashCall>
    for DelegationManagerCalls {
        fn from(value: OperatorAvsRegistrationTypehashCall) -> Self {
            Self::OperatorAvsRegistrationTypehash(value)
        }
    }
    impl ::core::convert::From<StakerDelegationTypehashCall> for DelegationManagerCalls {
        fn from(value: StakerDelegationTypehashCall) -> Self {
            Self::StakerDelegationTypehash(value)
        }
    }
    impl ::core::convert::From<AvsOperatorStatusCall> for DelegationManagerCalls {
        fn from(value: AvsOperatorStatusCall) -> Self {
            Self::AvsOperatorStatus(value)
        }
    }
    impl ::core::convert::From<BeaconChainETHStrategyCall> for DelegationManagerCalls {
        fn from(value: BeaconChainETHStrategyCall) -> Self {
            Self::BeaconChainETHStrategy(value)
        }
    }
    impl ::core::convert::From<CalculateCurrentStakerDelegationDigestHashCall>
    for DelegationManagerCalls {
        fn from(value: CalculateCurrentStakerDelegationDigestHashCall) -> Self {
            Self::CalculateCurrentStakerDelegationDigestHash(value)
        }
    }
    impl ::core::convert::From<CalculateDelegationApprovalDigestHashCall>
    for DelegationManagerCalls {
        fn from(value: CalculateDelegationApprovalDigestHashCall) -> Self {
            Self::CalculateDelegationApprovalDigestHash(value)
        }
    }
    impl ::core::convert::From<CalculateOperatorAVSRegistrationDigestHashCall>
    for DelegationManagerCalls {
        fn from(value: CalculateOperatorAVSRegistrationDigestHashCall) -> Self {
            Self::CalculateOperatorAVSRegistrationDigestHash(value)
        }
    }
    impl ::core::convert::From<CalculateStakerDelegationDigestHashCall>
    for DelegationManagerCalls {
        fn from(value: CalculateStakerDelegationDigestHashCall) -> Self {
            Self::CalculateStakerDelegationDigestHash(value)
        }
    }
    impl ::core::convert::From<CalculateWithdrawalRootCall> for DelegationManagerCalls {
        fn from(value: CalculateWithdrawalRootCall) -> Self {
            Self::CalculateWithdrawalRoot(value)
        }
    }
    impl ::core::convert::From<CompleteQueuedWithdrawalCall> for DelegationManagerCalls {
        fn from(value: CompleteQueuedWithdrawalCall) -> Self {
            Self::CompleteQueuedWithdrawal(value)
        }
    }
    impl ::core::convert::From<CompleteQueuedWithdrawalsCall>
    for DelegationManagerCalls {
        fn from(value: CompleteQueuedWithdrawalsCall) -> Self {
            Self::CompleteQueuedWithdrawals(value)
        }
    }
    impl ::core::convert::From<CumulativeWithdrawalsQueuedCall>
    for DelegationManagerCalls {
        fn from(value: CumulativeWithdrawalsQueuedCall) -> Self {
            Self::CumulativeWithdrawalsQueued(value)
        }
    }
    impl ::core::convert::From<DecreaseDelegatedSharesCall> for DelegationManagerCalls {
        fn from(value: DecreaseDelegatedSharesCall) -> Self {
            Self::DecreaseDelegatedShares(value)
        }
    }
    impl ::core::convert::From<DelegateToCall> for DelegationManagerCalls {
        fn from(value: DelegateToCall) -> Self {
            Self::DelegateTo(value)
        }
    }
    impl ::core::convert::From<DelegateToBySignatureCall> for DelegationManagerCalls {
        fn from(value: DelegateToBySignatureCall) -> Self {
            Self::DelegateToBySignature(value)
        }
    }
    impl ::core::convert::From<DelegatedToCall> for DelegationManagerCalls {
        fn from(value: DelegatedToCall) -> Self {
            Self::DelegatedTo(value)
        }
    }
    impl ::core::convert::From<DelegationApproverCall> for DelegationManagerCalls {
        fn from(value: DelegationApproverCall) -> Self {
            Self::DelegationApprover(value)
        }
    }
    impl ::core::convert::From<DelegationApproverSaltIsSpentCall>
    for DelegationManagerCalls {
        fn from(value: DelegationApproverSaltIsSpentCall) -> Self {
            Self::DelegationApproverSaltIsSpent(value)
        }
    }
    impl ::core::convert::From<DeregisterOperatorFromAVSCall>
    for DelegationManagerCalls {
        fn from(value: DeregisterOperatorFromAVSCall) -> Self {
            Self::DeregisterOperatorFromAVS(value)
        }
    }
    impl ::core::convert::From<DomainSeparatorCall> for DelegationManagerCalls {
        fn from(value: DomainSeparatorCall) -> Self {
            Self::DomainSeparator(value)
        }
    }
    impl ::core::convert::From<EarningsReceiverCall> for DelegationManagerCalls {
        fn from(value: EarningsReceiverCall) -> Self {
            Self::EarningsReceiver(value)
        }
    }
    impl ::core::convert::From<EigenPodManagerCall> for DelegationManagerCalls {
        fn from(value: EigenPodManagerCall) -> Self {
            Self::EigenPodManager(value)
        }
    }
    impl ::core::convert::From<GetDelegatableSharesCall> for DelegationManagerCalls {
        fn from(value: GetDelegatableSharesCall) -> Self {
            Self::GetDelegatableShares(value)
        }
    }
    impl ::core::convert::From<IncreaseDelegatedSharesCall> for DelegationManagerCalls {
        fn from(value: IncreaseDelegatedSharesCall) -> Self {
            Self::IncreaseDelegatedShares(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for DelegationManagerCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<IsDelegatedCall> for DelegationManagerCalls {
        fn from(value: IsDelegatedCall) -> Self {
            Self::IsDelegated(value)
        }
    }
    impl ::core::convert::From<IsOperatorCall> for DelegationManagerCalls {
        fn from(value: IsOperatorCall) -> Self {
            Self::IsOperator(value)
        }
    }
    impl ::core::convert::From<MigrateQueuedWithdrawalsCall> for DelegationManagerCalls {
        fn from(value: MigrateQueuedWithdrawalsCall) -> Self {
            Self::MigrateQueuedWithdrawals(value)
        }
    }
    impl ::core::convert::From<ModifyOperatorDetailsCall> for DelegationManagerCalls {
        fn from(value: ModifyOperatorDetailsCall) -> Self {
            Self::ModifyOperatorDetails(value)
        }
    }
    impl ::core::convert::From<OperatorDetailsCall> for DelegationManagerCalls {
        fn from(value: OperatorDetailsCall) -> Self {
            Self::OperatorDetails(value)
        }
    }
    impl ::core::convert::From<OperatorSaltIsSpentCall> for DelegationManagerCalls {
        fn from(value: OperatorSaltIsSpentCall) -> Self {
            Self::OperatorSaltIsSpent(value)
        }
    }
    impl ::core::convert::From<OperatorSharesCall> for DelegationManagerCalls {
        fn from(value: OperatorSharesCall) -> Self {
            Self::OperatorShares(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for DelegationManagerCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PauseCall> for DelegationManagerCalls {
        fn from(value: PauseCall) -> Self {
            Self::Pause(value)
        }
    }
    impl ::core::convert::From<PauseAllCall> for DelegationManagerCalls {
        fn from(value: PauseAllCall) -> Self {
            Self::PauseAll(value)
        }
    }
    impl ::core::convert::From<PausedWithIndexCall> for DelegationManagerCalls {
        fn from(value: PausedWithIndexCall) -> Self {
            Self::PausedWithIndex(value)
        }
    }
    impl ::core::convert::From<PausedCall> for DelegationManagerCalls {
        fn from(value: PausedCall) -> Self {
            Self::Paused(value)
        }
    }
    impl ::core::convert::From<PauserRegistryCall> for DelegationManagerCalls {
        fn from(value: PauserRegistryCall) -> Self {
            Self::PauserRegistry(value)
        }
    }
    impl ::core::convert::From<PendingWithdrawalsCall> for DelegationManagerCalls {
        fn from(value: PendingWithdrawalsCall) -> Self {
            Self::PendingWithdrawals(value)
        }
    }
    impl ::core::convert::From<QueueWithdrawalsCall> for DelegationManagerCalls {
        fn from(value: QueueWithdrawalsCall) -> Self {
            Self::QueueWithdrawals(value)
        }
    }
    impl ::core::convert::From<RegisterAsOperatorCall> for DelegationManagerCalls {
        fn from(value: RegisterAsOperatorCall) -> Self {
            Self::RegisterAsOperator(value)
        }
    }
    impl ::core::convert::From<RegisterOperatorToAVSCall> for DelegationManagerCalls {
        fn from(value: RegisterOperatorToAVSCall) -> Self {
            Self::RegisterOperatorToAVS(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for DelegationManagerCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SetPauserRegistryCall> for DelegationManagerCalls {
        fn from(value: SetPauserRegistryCall) -> Self {
            Self::SetPauserRegistry(value)
        }
    }
    impl ::core::convert::From<SlasherCall> for DelegationManagerCalls {
        fn from(value: SlasherCall) -> Self {
            Self::Slasher(value)
        }
    }
    impl ::core::convert::From<StakerNonceCall> for DelegationManagerCalls {
        fn from(value: StakerNonceCall) -> Self {
            Self::StakerNonce(value)
        }
    }
    impl ::core::convert::From<StakerOptOutWindowBlocksCall> for DelegationManagerCalls {
        fn from(value: StakerOptOutWindowBlocksCall) -> Self {
            Self::StakerOptOutWindowBlocks(value)
        }
    }
    impl ::core::convert::From<StrategyManagerCall> for DelegationManagerCalls {
        fn from(value: StrategyManagerCall) -> Self {
            Self::StrategyManager(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for DelegationManagerCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UndelegateCall> for DelegationManagerCalls {
        fn from(value: UndelegateCall) -> Self {
            Self::Undelegate(value)
        }
    }
    impl ::core::convert::From<UnpauseCall> for DelegationManagerCalls {
        fn from(value: UnpauseCall) -> Self {
            Self::Unpause(value)
        }
    }
    impl ::core::convert::From<UpdateAVSMetadataURICall> for DelegationManagerCalls {
        fn from(value: UpdateAVSMetadataURICall) -> Self {
            Self::UpdateAVSMetadataURI(value)
        }
    }
    impl ::core::convert::From<UpdateOperatorMetadataURICall>
    for DelegationManagerCalls {
        fn from(value: UpdateOperatorMetadataURICall) -> Self {
            Self::UpdateOperatorMetadataURI(value)
        }
    }
    impl ::core::convert::From<WithdrawalDelayBlocksCall> for DelegationManagerCalls {
        fn from(value: WithdrawalDelayBlocksCall) -> Self {
            Self::WithdrawalDelayBlocks(value)
        }
    }
    ///Container type for all return fields from the `DELEGATION_APPROVAL_TYPEHASH` function with signature `DELEGATION_APPROVAL_TYPEHASH()` and selector `0x04a4f979`
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
    pub struct DelegationApprovalTypehashReturn(pub [u8; 32]);
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
        Hash
    )]
    pub struct DomainTypehashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `MAX_STAKER_OPT_OUT_WINDOW_BLOCKS` function with signature `MAX_STAKER_OPT_OUT_WINDOW_BLOCKS()` and selector `0x4fc40b61`
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
    pub struct MaxStakerOptOutWindowBlocksReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `MAX_WITHDRAWAL_DELAY_BLOCKS` function with signature `MAX_WITHDRAWAL_DELAY_BLOCKS()` and selector `0xca661c04`
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
    pub struct MaxWithdrawalDelayBlocksReturn(pub ::ethers::core::types::U256);
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
        Hash
    )]
    pub struct OperatorAvsRegistrationTypehashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `STAKER_DELEGATION_TYPEHASH` function with signature `STAKER_DELEGATION_TYPEHASH()` and selector `0x43377382`
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
    pub struct StakerDelegationTypehashReturn(pub [u8; 32]);
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
        Hash
    )]
    pub struct AvsOperatorStatusReturn(pub u8);
    ///Container type for all return fields from the `beaconChainETHStrategy` function with signature `beaconChainETHStrategy()` and selector `0x9104c319`
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
    pub struct BeaconChainETHStrategyReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `calculateCurrentStakerDelegationDigestHash` function with signature `calculateCurrentStakerDelegationDigestHash(address,address,uint256)` and selector `0x1bbce091`
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
    pub struct CalculateCurrentStakerDelegationDigestHashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `calculateDelegationApprovalDigestHash` function with signature `calculateDelegationApprovalDigestHash(address,address,address,bytes32,uint256)` and selector `0x0b9f487a`
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
    pub struct CalculateDelegationApprovalDigestHashReturn(pub [u8; 32]);
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
        Hash
    )]
    pub struct CalculateOperatorAVSRegistrationDigestHashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `calculateStakerDelegationDigestHash` function with signature `calculateStakerDelegationDigestHash(address,uint256,address,uint256)` and selector `0xc94b5111`
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
    pub struct CalculateStakerDelegationDigestHashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `calculateWithdrawalRoot` function with signature `calculateWithdrawalRoot((address,address,address,uint256,uint32,address[],uint256[]))` and selector `0x597b36da`
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
    pub struct CalculateWithdrawalRootReturn(pub [u8; 32]);
    ///Container type for all return fields from the `cumulativeWithdrawalsQueued` function with signature `cumulativeWithdrawalsQueued(address)` and selector `0xa1788484`
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
    pub struct CumulativeWithdrawalsQueuedReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `delegatedTo` function with signature `delegatedTo(address)` and selector `0x65da1264`
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
    pub struct DelegatedToReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `delegationApprover` function with signature `delegationApprover(address)` and selector `0x3cdeb5e0`
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
    pub struct DelegationApproverReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `delegationApproverSaltIsSpent` function with signature `delegationApproverSaltIsSpent(address,bytes32)` and selector `0xbb45fef2`
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
    pub struct DelegationApproverSaltIsSpentReturn(pub bool);
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
        Hash
    )]
    pub struct DomainSeparatorReturn(pub [u8; 32]);
    ///Container type for all return fields from the `earningsReceiver` function with signature `earningsReceiver(address)` and selector `0x5f966f14`
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
    pub struct EarningsReceiverReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `eigenPodManager` function with signature `eigenPodManager()` and selector `0x4665bcda`
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
    pub struct EigenPodManagerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getDelegatableShares` function with signature `getDelegatableShares(address)` and selector `0xcf80873e`
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
    pub struct GetDelegatableSharesReturn(
        pub ::std::vec::Vec<::ethers::core::types::Address>,
        pub ::std::vec::Vec<::ethers::core::types::U256>,
    );
    ///Container type for all return fields from the `isDelegated` function with signature `isDelegated(address)` and selector `0x3e28391d`
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
    pub struct IsDelegatedReturn(pub bool);
    ///Container type for all return fields from the `isOperator` function with signature `isOperator(address)` and selector `0x6d70f7ae`
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
    pub struct IsOperatorReturn(pub bool);
    ///Container type for all return fields from the `operatorDetails` function with signature `operatorDetails(address)` and selector `0xc5e480db`
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
    pub struct OperatorDetailsReturn(pub OperatorDetails);
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
        Hash
    )]
    pub struct OperatorSaltIsSpentReturn(pub bool);
    ///Container type for all return fields from the `operatorShares` function with signature `operatorShares(address,address)` and selector `0x778e55f3`
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
    pub struct OperatorSharesReturn(pub ::ethers::core::types::U256);
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
        Hash
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
        Hash
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
        Hash
    )]
    pub struct PauserRegistryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `pendingWithdrawals` function with signature `pendingWithdrawals(bytes32)` and selector `0xb7f06ebe`
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
    pub struct PendingWithdrawalsReturn(pub bool);
    ///Container type for all return fields from the `queueWithdrawals` function with signature `queueWithdrawals((address[],uint256[],address)[])` and selector `0x0dd8dd02`
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
    pub struct QueueWithdrawalsReturn(pub ::std::vec::Vec<[u8; 32]>);
    ///Container type for all return fields from the `slasher` function with signature `slasher()` and selector `0xb1344271`
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
    pub struct SlasherReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `stakerNonce` function with signature `stakerNonce(address)` and selector `0x29c77d4f`
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
    pub struct StakerNonceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `stakerOptOutWindowBlocks` function with signature `stakerOptOutWindowBlocks(address)` and selector `0x16928365`
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
    pub struct StakerOptOutWindowBlocksReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `strategyManager` function with signature `strategyManager()` and selector `0x39b70e38`
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
    pub struct StrategyManagerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `undelegate` function with signature `undelegate(address)` and selector `0xda8be864`
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
    pub struct UndelegateReturn {
        pub withdrawal_roots: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all return fields from the `withdrawalDelayBlocks` function with signature `withdrawalDelayBlocks()` and selector `0x50f73e7c`
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
    pub struct WithdrawalDelayBlocksReturn(pub ::ethers::core::types::U256);
}
