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
                    ::std::borrow::ToOwned::to_owned("getOperatorShares"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getOperatorShares"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("strategies"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IStrategy[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("getWithdrawalDelay"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getWithdrawalDelay"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("strategies"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IStrategy[]"),
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
                                        "_minWithdrawalDelayBlocks",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_strategies"),
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
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_withdrawalDelayBlocks",
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
                    ::std::borrow::ToOwned::to_owned("minWithdrawalDelayBlocks"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "minWithdrawalDelayBlocks",
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
                    ::std::borrow::ToOwned::to_owned("setStrategyWithdrawalDelayBlocks"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setStrategyWithdrawalDelayBlocks",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("strategies"),
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
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "withdrawalDelayBlocks",
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
                    ::std::borrow::ToOwned::to_owned("strategyWithdrawalDelayBlocks"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "strategyWithdrawalDelayBlocks",
                            ),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("MinWithdrawalDelayBlocksSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MinWithdrawalDelayBlocksSet",
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
                    ::std::borrow::ToOwned::to_owned("StrategyWithdrawalDelayBlocksSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "StrategyWithdrawalDelayBlocksSet",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("strategy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
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
    const __BYTECODE: &[u8] = b"a\x01\0`@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@Qb\0aa8\x03\x80b\0aa\x839\x81\x01`@\x81\x90Rb\0\x005\x91b\0\x01@V[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\x80R\x80\x82\x16`\xC0R\x82\x16`\xA0Rb\0\0Xb\0\0eV[PPF`\xE0RPb\0\x01\x94V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15b\0\0\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\xFF\x90\x81\x16\x10\x15b\0\x01%W`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01=W`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15b\0\x01VW`\0\x80\xFD[\x83Qb\0\x01c\x81b\0\x01'V[` \x85\x01Q\x90\x93Pb\0\x01v\x81b\0\x01'V[`@\x85\x01Q\x90\x92Pb\0\x01\x89\x81b\0\x01'V[\x80\x91PP\x92P\x92P\x92V[`\x80Q`\xA0Q`\xC0Q`\xE0Qa_1b\0\x020`\09`\0a'M\x01R`\0\x81\x81a\x05\xCC\x01R\x81\x81a\x10\x93\x01R\x81\x81a\x14\x0F\x01R\x81\x81a\x1D\xE1\x01R\x81\x81a*\xA6\x01R\x81\x81aA\x19\x01RaF\x05\x01R`\0a\x07\xA9\x01R`\0\x81\x81a\x05\x14\x01R\x81\x81a\x10a\x01R\x81\x81a\x13\xDD\x01R\x81\x81a\x17\x05\x01R\x81\x81a\x1Eu\x01R\x81\x81a+s\x01R\x81\x81a,\xF6\x01R\x81\x81aB?\x01RaF\xAB\x01Ra_1`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x03]W`\x005`\xE0\x1C\x80c_\x96o\x14\x11a\x01\xD3W\x80c\xB7\xF0n\xBE\x11a\x01\x04W\x80c\xCF\x80\x87>\x11a\0\xA2W\x80c\xF1ar\xB0\x11a\0|W\x80c\xF1ar\xB0\x14a\tOW\x80c\xF2\xFD\xE3\x8B\x14a\tbW\x80c\xF6\x98\xDA%\x14a\tuW\x80c\xFA\xBC\x1C\xBC\x14a\t}W`\0\x80\xFD[\x80c\xCF\x80\x87>\x14a\t\x08W\x80c\xDA\x8B\xE8d\x14a\t)W\x80c\xEE\xA9\x06K\x14a\t<W`\0\x80\xFD[\x80c\xC4\x887Z\x11a\0\xDEW\x80c\xC4\x887Z\x14a\x08%W\x80c\xC5\xE4\x80\xDB\x14a\x08EW\x80c\xC9KQ\x11\x14a\x08\xEBW\x80c\xCAf\x1C\x04\x14a\x08\xFEW`\0\x80\xFD[\x80c\xB7\xF0n\xBE\x14a\x07\xCBW\x80c\xBBE\xFE\xF2\x14a\x07\xEEW\x80c\xC4H\xFE\xB8\x14a\x08\x1CW`\0\x80\xFD[\x80c\x88o\x11\x95\x11a\x01qW\x80c\x91\x04\xC3\x19\x11a\x01KW\x80c\x91\x04\xC3\x19\x14a\x07VW\x80c\x99\xBE\x81\xC8\x14a\x07qW\x80c\xA1x\x84\x84\x14a\x07\x84W\x80c\xB14Bq\x14a\x07\xA4W`\0\x80\xFD[\x80c\x88o\x11\x95\x14a\x07\x12W\x80c\x8D\xA5\xCB[\x14a\x07%W\x80c\x90\x04\x13G\x14a\x076W`\0\x80\xFD[\x80cmp\xF7\xAE\x11a\x01\xADW\x80cmp\xF7\xAE\x14a\x06\xB9W\x80cqP\x18\xA6\x14a\x06\xCCW\x80cw\x8EU\xF3\x14a\x06\xD4W\x80c\x7FT\x80q\x14a\x06\xFFW`\0\x80\xFD[\x80c_\x96o\x14\x14a\x06QW\x80c`\xD7\xFA\xED\x14a\x06}W\x80ce\xDA\x12d\x14a\x06\x90W`\0\x80\xFD[\x80c(\xA5s\xAE\x11a\x02\xADW\x80cFe\xBC\xDA\x11a\x02KW\x80cY{6\xDA\x11a\x02%W\x80cY{6\xDA\x14a\x06\0W\x80cZ\xC8j\xB7\x14a\x06\x13W\x80c\\\x97Z\xBB\x14a\x066W\x80c\\\xFE\x8D,\x14a\x06>W`\0\x80\xFD[\x80cFe\xBC\xDA\x14a\x05\xC7W\x80cO\xC4\x0Ba\x14a\x05\xEEW\x80cY\\jg\x14a\x05\xF8W`\0\x80\xFD[\x80c9\xB7\x0E8\x11a\x02\x87W\x80c9\xB7\x0E8\x14a\x05\x0FW\x80c<\xDE\xB5\xE0\x14a\x05NW\x80c>(9\x1D\x14a\x05}W\x80cC7s\x82\x14a\x05\xA0W`\0\x80\xFD[\x80c(\xA5s\xAE\x14a\x04\xC9W\x80c)\xC7}O\x14a\x04\xDCW\x80c3@C\x96\x14a\x04\xFCW`\0\x80\xFD[\x80c\x13-Ig\x11a\x03\x1AW\x80c\x16\x92\x83e\x11a\x02\xF4W\x80c\x16\x92\x83e\x14a\x04CW\x80c\x1B\xBC\xE0\x91\x14a\x04|W\x80c `kp\x14a\x04\x8FW\x80c\"\xBF@\xE4\x14a\x04\xB6W`\0\x80\xFD[\x80c\x13-Ig\x14a\x04\nW\x80c\x13d9\xDD\x14a\x04\x1DW\x80c\x15\"\xBF\x02\x14a\x040W`\0\x80\xFD[\x80c\x04I\xCA9\x14a\x03bW\x80c\x04\xA4\xF9y\x14a\x03\x88W\x80c\x0B\x9FHz\x14a\x03\xAFW\x80c\r\xD8\xDD\x02\x14a\x03\xC2W\x80c\x0FX\x9EY\x14a\x03\xE2W\x80c\x10\xD6z/\x14a\x03\xF7W[`\0\x80\xFD[a\x03ua\x03p6`\x04aJ\xBBV[a\t\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x03u\x7F;\x89\xFC\xA1Q\xCB\xE5\x12-X\xAC\xEE\x86\xCF\x18D\x13\xD7Q\xD5\x85w\x9B\xDD\x19\xD3\xBB\xFA:0m\xCE\x81V[a\x03ua\x03\xBD6`\x04aK!V[a\n\x15V[a\x03\xD5a\x03\xD06`\x04aJ\xBBV[a\n\xD7V[`@Qa\x03\x7F\x91\x90aK|V[a\x03\xF5a\x03\xF06`\x04aL\x19V[a\x0ESV[\0[a\x03\xF5a\x04\x056`\x04aLlV[a\x0F\xA3V[a\x03\xF5a\x04\x186`\x04aL\x90V[a\x10VV[a\x03\xF5a\x04+6`\x04aL\xD1V[a\x11\rV[a\x03\xF5a\x04>6`\x04aL\xEAV[a\x12LV[a\x03ua\x04Q6`\x04aLlV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x90V[a\x03ua\x04\x8A6`\x04aL\x90V[a\x12`V[a\x03u\x7F\x8C\xAD\x95h{\xA8,,\xE5\x0Et\xF7\xB7Td^Q\x17\xC3\xA5\xBE\xC8\x15\x1C\x07&\xD5\x85y\x80\xA8f\x81V[a\x03\xF5a\x04\xC46`\x04aMUV[a\x12\x8EV[a\x03\xF5a\x04\xD76`\x04aL\x90V[a\x13\xD2V[a\x03ua\x04\xEA6`\x04aLlV[`\x9B` R`\0\x90\x81R`@\x90 T\x81V[a\x03\xF5a\x05\n6`\x04aM\xFCV[a\x14\x82V[a\x056\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03\x7FV[a\x056a\x05\\6`\x04aLlV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T\x16\x90V[a\x05\x90a\x05\x8B6`\x04aLlV[a\x15\xBFV[`@Q\x90\x15\x15\x81R` \x01a\x03\x7FV[a\x03u\x7F9\x11\x1B\xC4\xA4\xD6\x88\xE1\xF6\x85\x12=t\x97\xD4aSp\x15*\x8E\xE4\xA0Y>d{\xD0j\xD8\xBB\x0B\x81V[a\x056\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03ub\x13\xC6\x80\x81V[a\x03\xF5a\x15\xDFV[a\x03ua\x06\x0E6`\x04aQ\x1BV[a\x16\xA6V[a\x05\x90a\x06!6`\x04aQWV[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`fTa\x03uV[a\x03\xF5a\x06L6`\x04aQ\xCDV[a\x16\xD6V[a\x056a\x06_6`\x04aLlV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x99` R`@\x90 T\x16\x90V[a\x03\xF5a\x06\x8B6`\x04aS-V[a\x19\x81V[a\x056a\x06\x9E6`\x04aLlV[`\x9A` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x05\x90a\x06\xC76`\x04aLlV[a\x1A\x1CV[a\x03\xF5a\x1A<V[a\x03ua\x06\xE26`\x04aS\xBCV[`\x98` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[a\x03\xF5a\x07\r6`\x04aT\x9DV[a\x1APV[`eTa\x056\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x056V[a\x07Ia\x07D6`\x04aU-V[a\x1BUV[`@Qa\x03\x7F\x91\x90aU\xB7V[a\x056s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x81V[a\x03\xF5a\x07\x7F6`\x04aU\xCAV[a\x1C/V[a\x03ua\x07\x926`\x04aLlV[`\x9F` R`\0\x90\x81R`@\x90 T\x81V[a\x056\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x05\x90a\x07\xD96`\x04aL\xD1V[`\x9E` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x05\x90a\x07\xFC6`\x04aU\xFFV[`\x9C` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[a\x03u`\x9DT\x81V[a\x03ua\x0836`\x04aLlV[`\xA1` R`\0\x90\x81R`@\x90 T\x81V[a\x08\xB5a\x08S6`\x04aLlV[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x92\x84\x01\x81\x90R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R`\x99\x83R\x83\x90 \x83Q\x91\x82\x01\x84R\x80T\x85\x16\x82R`\x01\x01T\x93\x84\x16\x91\x81\x01\x91\x90\x91R`\x01`\xA0\x1B\x90\x92\x04c\xFF\xFF\xFF\xFF\x16\x90\x82\x01R\x90V[`@\x80Q\x82Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x80\x85\x01Q\x90\x91\x16\x90\x82\x01R\x91\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R``\x01a\x03\x7FV[a\x03ua\x08\xF96`\x04aV+V[a\x1D\x01V[a\x03ub\x03K\xC0\x81V[a\t\x1Ba\t\x166`\x04aLlV[a\x1D\xBAV[`@Qa\x03\x7F\x92\x91\x90aV\xACV[a\x03\xD5a\t76`\x04aLlV[a!rV[a\x03\xF5a\tJ6`\x04aV\xD1V[a&6V[a\x03\xF5a\t]6`\x04aW)V[a&BV[a\x03\xF5a\tp6`\x04aLlV[a&\xD3V[a\x03ua'IV[a\x03\xF5a\t\x8B6`\x04aL\xD1V[a'\x87V[`\x9DT`\0\x90\x81[\x83\x81\x10\x15a\n\rW`\0`\xA1`\0\x87\x87\x85\x81\x81\x10a\t\xB8Wa\t\xB8aWEV[\x90P` \x02\x01` \x81\x01\x90a\t\xCD\x91\x90aLlV[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 T\x90P\x82\x81\x11\x15a\t\xFCW\x80\x92P[Pa\n\x06\x81aWqV[\x90Pa\t\x98V[P\x93\x92PPPV[`@\x80Q\x7F;\x89\xFC\xA1Q\xCB\xE5\x12-X\xAC\xEE\x86\xCF\x18D\x13\xD7Q\xD5\x85w\x9B\xDD\x19\xD3\xBB\xFA:0m\xCE` \x80\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x83\x85\x01R\x88\x81\x16``\x84\x01R\x87\x16`\x80\x83\x01R`\xA0\x82\x01\x85\x90R`\xC0\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\xE0\x90\x92\x01\x90\x92R\x80Q\x91\x01 `\0\x90\x81a\n\x93a'IV[`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x91\x90\x91R`B\x81\x01\x83\x90R`b\x01`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x98\x97PPPPPPPPV[`fT``\x90`\x01\x90`\x02\x90\x81\x16\x14\x15a\x0B\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\x03\x90aW\x8CV[`@Q\x80\x91\x03\x90\xFD[`\0\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0B&Wa\x0B&aN\x9EV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0BOW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P3`\0\x90\x81R`\x9A` R`@\x81 T\x91\x92P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90[\x85\x81\x10\x15a\x0EHW\x86\x86\x82\x81\x81\x10a\x0B\x8AWa\x0B\x8AaWEV[\x90P` \x02\x81\x01\x90a\x0B\x9C\x91\x90aW\xC3V[a\x0B\xAA\x90` \x81\x01\x90aW\xE3V[\x90P\x87\x87\x83\x81\x81\x10a\x0B\xBEWa\x0B\xBEaWEV[\x90P` \x02\x81\x01\x90a\x0B\xD0\x91\x90aW\xC3V[a\x0B\xDA\x90\x80aW\xE3V[\x90P\x14a\x0COW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FDelegationManager.queueWithdrawa`D\x82\x01R\x7Fl: input length mismatch\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\x03V[`\0\x87\x87\x83\x81\x81\x10a\x0CcWa\x0CcaWEV[\x90P` \x02\x81\x01\x90a\x0Cu\x91\x90aW\xC3V[a\x0C\x86\x90``\x81\x01\x90`@\x01aLlV[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\r\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`H`$\x82\x01R\x7FDelegationManager.queueWithdrawa`D\x82\x01R\x7Fl: must provide valid withdrawal`d\x82\x01Rg address`\xC0\x1B`\x84\x82\x01R`\xA4\x01a\x0B\x03V[a\x0E\x193\x83\x89\x89\x85\x81\x81\x10a\r+Wa\r+aWEV[\x90P` \x02\x81\x01\x90a\r=\x91\x90aW\xC3V[a\rN\x90``\x81\x01\x90`@\x01aLlV[\x8A\x8A\x86\x81\x81\x10a\r`Wa\r`aWEV[\x90P` \x02\x81\x01\x90a\rr\x91\x90aW\xC3V[a\r|\x90\x80aW\xE3V[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8E\x92P\x8D\x91P\x88\x90P\x81\x81\x10a\r\xC2Wa\r\xC2aWEV[\x90P` \x02\x81\x01\x90a\r\xD4\x91\x90aW\xC3V[a\r\xE2\x90` \x81\x01\x90aW\xE3V[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa(\xE3\x92PPPV[\x83\x82\x81Q\x81\x10a\x0E+Wa\x0E+aWEV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a\x0E@\x81aWqV[\x91PPa\x0BpV[P\x90\x95\x94PPPPPV[3`\0\x90\x81R`\x99` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x0E\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R\x7FDelegationManager.registerAsOper`D\x82\x01R\x7Fator: operator has already regis`d\x82\x01Rd\x1D\x19\\\x99Y`\xDA\x1B`\x84\x82\x01R`\xA4\x01a\x0B\x03V[a\x0E\xF73\x84a.\xA3V[`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01Ra\x0F\x193\x80\x83`\0a1?V[3`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8E\x84\x85X:#\x10\xD4\x1F|\x82\xB9B}\x0B\xD4\x9B\xADt\xBB\x9C\xFF\x9D4\x02\xA2\x9D\x8F\x9B(\xA0\xE2\x85`@Qa\x0FR\x91\x90aX,V[`@Q\x80\x91\x03\x90\xA23`\x01`\x01`\xA0\x1B\x03\x16\x7F\x02\xA9\x19\xED\x0E*\xCA\xD1\xDD\x90\xF1~\xF2\xFAJ\xE5F.\xE13\x91p\x03J\x851\xCC\xA4\xB6p\x80\x90\x84\x84`@Qa\x0F\x95\x92\x91\x90aX~V[`@Q\x80\x91\x03\x90\xA2PPPPV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xF6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x1A\x91\x90aX\xADV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x10JW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\x03\x90aX\xCAV[a\x10S\x81a4\xE4V[PV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x10\xB5WP3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14[a\x10\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\x03\x90aY\x14V[a\x10\xDA\x83a\x15\xBFV[\x15a\x11\x08W`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R`\x9A` R`@\x90 T\x16a\x11\x06\x81\x85\x85\x85a5\xDBV[P[PPPV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11UW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11y\x91\x90aYqV[a\x11\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\x03\x90aY\x8EV[`fT\x81\x81\x16\x14a\x12\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\x03V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[a\x12Ta6VV[a\x11\x06\x84\x84\x84\x84a6\xB0V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x9B` R`@\x81 Ta\x12\x85\x85\x82\x86\x86a\x1D\x01V[\x95\x94PPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x12\xAEWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x12\xC8WP0;\x15\x80\x15a\x12\xC8WP`\0T`\xFF\x16`\x01\x14[a\x13+W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x0B\x03V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x13NW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x13X\x88\x88a8\xD6V[a\x13`a9\xBCV[`\x97Ua\x13l\x89a:SV[a\x13u\x86a:\xA5V[a\x13\x81\x85\x85\x85\x85a6\xB0V[\x80\x15a\x13\xC7W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x141WP3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14[a\x14MW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\x03\x90aY\x14V[a\x14V\x83a\x15\xBFV[\x15a\x11\x08W`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R`\x9A` R`@\x90 T\x16a\x11\x06\x81\x85\x85\x85a;\xABV[`fT`\x02\x90`\x04\x90\x81\x16\x14\x15a\x14\xABW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\x03\x90aW\x8CV[`\x02`\xC9T\x14\x15a\x14\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x0B\x03V[`\x02`\xC9U`\0[\x88\x81\x10\x15a\x15\xAEWa\x15\x9E\x8A\x8A\x83\x81\x81\x10a\x15#Wa\x15#aWEV[\x90P` \x02\x81\x01\x90a\x155\x91\x90aY\xD6V[\x89\x89\x84\x81\x81\x10a\x15GWa\x15GaWEV[\x90P` \x02\x81\x01\x90a\x15Y\x91\x90aW\xE3V[\x89\x89\x86\x81\x81\x10a\x15kWa\x15kaWEV[\x90P` \x02\x015\x88\x88\x87\x81\x81\x10a\x15\x84Wa\x15\x84aWEV[\x90P` \x02\x01` \x81\x01\x90a\x15\x99\x91\x90aY\xECV[a<&V[a\x15\xA7\x81aWqV[\x90Pa\x15\x06V[PP`\x01`\xC9UPPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x9A` R`@\x90 T\x16\x15\x15\x90V[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16'W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16K\x91\x90aYqV[a\x16gW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\x03\x90aY\x8EV[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[`\0\x81`@Q` \x01a\x16\xB9\x91\x90aZ}V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0[\x81Q\x81\x10\x15a\x19}W`\0\x82\x82\x81Q\x81\x10a\x16\xF6Wa\x16\xF6aWEV[` \x02` \x01\x01Q\x90P`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xCD)?o\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17O\x91\x90aZ\x90V[`@\x80Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x17mW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\x91\x91\x90a[<V[\x91P\x91P\x81\x15a\x19oW`@\x80\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x9F` R\x91\x82 \x80T\x91\x92\x82\x91\x90a\x17\xC8\x83aWqV[\x91\x90PUP`\0`@Q\x80`\xE0\x01`@R\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x87`\xA0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x87``\x01Q`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01\x87`\x80\x01Qc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x87`\0\x01Q\x81R` \x01\x87` \x01Q\x81RP\x90P`\0a\x18H\x82a\x16\xA6V[`\0\x81\x81R`\x9E` R`@\x90 T\x90\x91P`\xFF\x16\x15a\x18\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R\x7FDelegationManager.migrateQueuedW`D\x82\x01R\x7Fithdrawals: withdrawal already e`d\x82\x01Rdxists`\xD8\x1B`\x84\x82\x01R`\xA4\x01a\x0B\x03V[`\0\x81\x81R`\x9E` R`@\x90\x81\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x7F\x90\t\xAB\x15>\x80\x14\xFB\xFB\x02\xF2!\x7F\\\xDEz\xA7\xF9\xADsJ\xE8\\\xA3\xEE?L\xA2\xFD\xD4\x99\xF9\x90a\x19)\x90\x83\x90\x85\x90a[jV[`@Q\x80\x91\x03\x90\xA1`@\x80Q\x86\x81R` \x81\x01\x83\x90R\x7F\xDC\0u\x8Be\xEE\xF7\x1D\xC3x\x0C\x04\xEB\xE3l\xABk\xDB&l:i\x81\x87\xE2\x9E\x0F\r\xCA\x01&0\x91\x01`@Q\x80\x91\x03\x90\xA1PPPP[\x83`\x01\x01\x93PPPPa\x16\xD9V[PPV[`fT`\x02\x90`\x04\x90\x81\x16\x14\x15a\x19\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\x03\x90aW\x8CV[`\x02`\xC9T\x14\x15a\x19\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x0B\x03V[`\x02`\xC9Ua\x1A\x0F\x86\x86\x86\x86\x86a<&V[PP`\x01`\xC9UPPPPV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x99` R`@\x90 T\x16\x15\x15\x90V[a\x1ADa6VV[a\x1AN`\0a:SV[V[B\x83` \x01Q\x10\x15a\x1A\xD4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FDelegationManager.delegateToBySi`D\x82\x01R\x7Fgnature: staker signature expire`d\x82\x01R`\x19`\xFA\x1B`\x84\x82\x01R`\xA4\x01a\x0B\x03V[`\0`\x9B`\0\x87`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 T\x90P`\0a\x1B\x10\x87\x83\x88\x88` \x01Qa\x1D\x01V[`\x01`\x01`\xA0\x1B\x03\x88\x16`\0\x90\x81R`\x9B` R`@\x90 `\x01\x84\x01\x90U\x85Q\x90\x91Pa\x1B@\x90\x88\x90\x83\x90aD\x10V[a\x1BL\x87\x87\x86\x86a1?V[PPPPPPPV[```\0\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1BrWa\x1BraN\x9EV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1B\x9BW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\n\rW`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x98` R`@\x81 \x85Q\x90\x91\x90\x86\x90\x84\x90\x81\x10a\x1B\xD9Wa\x1B\xD9aWEV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 T\x82\x82\x81Q\x81\x10a\x1C\x14Wa\x1C\x14aWEV[` \x90\x81\x02\x91\x90\x91\x01\x01Ra\x1C(\x81aWqV[\x90Pa\x1B\xA1V[a\x1C83a\x1A\x1CV[a\x1C\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FDelegationManager.updateOperator`D\x82\x01R\x7FMetadataURI: caller must be an o`d\x82\x01Rf82\xB90\xBA7\xB9`\xC9\x1B`\x84\x82\x01R`\xA4\x01a\x0B\x03V[3`\x01`\x01`\xA0\x1B\x03\x16\x7F\x02\xA9\x19\xED\x0E*\xCA\xD1\xDD\x90\xF1~\xF2\xFAJ\xE5F.\xE13\x91p\x03J\x851\xCC\xA4\xB6p\x80\x90\x83\x83`@Qa\x1C\xF5\x92\x91\x90aX~V[`@Q\x80\x91\x03\x90\xA2PPV[`@\x80Q\x7F9\x11\x1B\xC4\xA4\xD6\x88\xE1\xF6\x85\x12=t\x97\xD4aSp\x15*\x8E\xE4\xA0Y>d{\xD0j\xD8\xBB\x0B` \x80\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x83\x85\x01R\x85\x16``\x83\x01R`\x80\x82\x01\x86\x90R`\xA0\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\xC0\x90\x92\x01\x90\x92R\x80Q\x91\x01 `\0\x90\x81a\x1Dwa'IV[`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x91\x90\x91R`B\x81\x01\x83\x90R`b\x01`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x97\x96PPPPPPPV[`@Qc`\xF4\x06+`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R``\x91\x82\x91`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c`\xF4\x06+\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E*W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1EN\x91\x90a[\x83V[`@Qc\x94\xF6I\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R\x91\x92P`\0\x91\x82\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\x94\xF6I\xDD\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\xBEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1E\xE6\x91\x90\x81\x01\x90a[\xF7V[\x91P\x91P`\0\x83\x13a\x1E\xFDW\x90\x95\x90\x94P\x92PPPV[``\x80\x83Q`\0\x14\x15a\x1F\xB7W`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x90` \x80\x83\x01\x90\x806\x837PP`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x92\x94P\x90P` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90Ps\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x82`\0\x81Q\x81\x10a\x1FrWa\x1FraWEV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x84\x81`\0\x81Q\x81\x10a\x1F\xA6Wa\x1F\xA6aWEV[` \x02` \x01\x01\x81\x81RPPa!eV[\x83Qa\x1F\xC4\x90`\x01a\\\xB1V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\xDBWa\x1F\xDBaN\x9EV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a \x04W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a  Wa  aN\x9EV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a IW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x84Q\x81\x10\x15a \xE3W\x84\x81\x81Q\x81\x10a jWa jaWEV[` \x02` \x01\x01Q\x83\x82\x81Q\x81\x10a \x84Wa \x84aWEV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x83\x81\x81Q\x81\x10a \xB6Wa \xB6aWEV[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a \xD0Wa \xD0aWEV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a OV[Ps\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x82`\x01\x84Qa!\x08\x91\x90a\\\xC9V[\x81Q\x81\x10a!\x18Wa!\x18aWEV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x84\x81`\x01\x84Qa!H\x91\x90a\\\xC9V[\x81Q\x81\x10a!XWa!XaWEV[` \x02` \x01\x01\x81\x81RPP[\x90\x97\x90\x96P\x94PPPPPV[`fT``\x90`\x01\x90`\x02\x90\x81\x16\x14\x15a!\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\x03\x90aW\x8CV[a!\xA7\x83a\x15\xBFV[a\"'W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FDelegationManager.undelegate: st\x90\x82\x01R\x7Faker must be delegated to undele`d\x82\x01Rcgate`\xE0\x1B`\x84\x82\x01R`\xA4\x01a\x0B\x03V[a\"0\x83a\x1A\x1CV[\x15a\"\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FDelegationManager.undelegate: op`D\x82\x01R\x7Ferators cannot be undelegated\0\0\0`d\x82\x01R`\x84\x01a\x0B\x03V[`\x01`\x01`\xA0\x1B\x03\x83\x16a#\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R\x7FDelegationManager.undelegate: ca`D\x82\x01R\x7Fnnot undelegate zero address\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\x03V[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x81\x81R`\x9A` R`@\x90 T\x90\x91\x16\x903\x14\x80a#RWP3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14[\x80a#yWP`\x01`\x01`\xA0\x1B\x03\x81\x81\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T\x163\x14[a#\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FDelegationManager.undelegate: ca`D\x82\x01R\x7Fller cannot undelegate staker\0\0\0`d\x82\x01R`\x84\x01a\x0B\x03V[`\0\x80a#\xF7\x86a\x1D\xBAV[\x90\x92P\x90P3`\x01`\x01`\xA0\x1B\x03\x87\x16\x14a$MW\x82`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x7F\xF0\xED\xDF\x07\xE6\xEA\x14\xF3\x88\xB4~\x1E\x94\xA0\xF4d\xEC\xBD\x9E\xEDAq\x13\x0E\x0F\xC0\xE9\x9F\xB4\x03\n\x8A`@Q`@Q\x80\x91\x03\x90\xA3[\x82`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFE\xE3\tf\xA2V\xB7\x1E\x14\xBC\x0E\xBF\xC9C\x15\xE2\x8E\xF4\xA9zq1\xA9\xE2\xB7\xA3\x10\xA7:\xF4Fv`@Q`@Q\x80\x91\x03\x90\xA3`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x9A` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x81Qa$\xCFW`@\x80Q`\0\x81R` \x81\x01\x90\x91R\x94Pa&-V[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a$\xE8Wa$\xE8aN\x9EV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a%\x11W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x94P`\0[\x82Q\x81\x10\x15a&+W`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91` \x80\x83\x01\x90\x806\x837PP`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x92\x93P`\0\x92\x91P` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x84\x83\x81Q\x81\x10a%wWa%waWEV[` \x02` \x01\x01Q\x82`\0\x81Q\x81\x10a%\x92Wa%\x92aWEV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x83\x83\x81Q\x81\x10a%\xC4Wa%\xC4aWEV[` \x02` \x01\x01Q\x81`\0\x81Q\x81\x10a%\xDFWa%\xDFaWEV[` \x02` \x01\x01\x81\x81RPPa%\xF8\x89\x87\x8B\x85\x85a(\xE3V[\x88\x84\x81Q\x81\x10a&\nWa&\naWEV[` \x02` \x01\x01\x81\x81RPPPP\x80\x80a&#\x90aWqV[\x91PPa%\x17V[P[PPPP\x91\x90PV[a\x11\x083\x84\x84\x84a1?V[a&K3a\x1A\x1CV[a&\xC9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FDelegationManager.modifyOperator`D\x82\x01R\x7FDetails: caller must be an opera`d\x82\x01Rb:7\xB9`\xE9\x1B`\x84\x82\x01R`\xA4\x01a\x0B\x03V[a\x10S3\x82a.\xA3V[a&\xDBa6VV[`\x01`\x01`\xA0\x1B\x03\x81\x16a'@W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x0B\x03V[a\x10S\x81a:SV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14\x15a'zWP`\x97T\x90V[a'\x82a9\xBCV[\x90P\x90V[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a'\xDAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'\xFE\x91\x90aX\xADV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a(.W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\x03\x90aX\xCAV[`fT\x19\x81\x19`fT\x19\x16\x14a(\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\x03V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x12AV[`\0`\x01`\x01`\xA0\x1B\x03\x86\x16a)zW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`P`$\x82\x01R\x7FDelegationManager._removeSharesA`D\x82\x01R\x7FndQueueWithdrawal: staker cannot`d\x82\x01Ro be zero address`\x80\x1B`\x84\x82\x01R`\xA4\x01a\x0B\x03V[\x82Qa*\x04W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`M`$\x82\x01R\x7FDelegationManager._removeSharesA`D\x82\x01R\x7FndQueueWithdrawal: strategies ca`d\x82\x01Rlnnot be empty`\x98\x1B`\x84\x82\x01R`\xA4\x01a\x0B\x03V[`\0[\x83Q\x81\x10\x15a-\xB1W`\x01`\x01`\xA0\x1B\x03\x86\x16\x15a*]Wa*]\x86\x88\x86\x84\x81Q\x81\x10a*6Wa*6aWEV[` \x02` \x01\x01Q\x86\x85\x81Q\x81\x10a*PWa*PaWEV[` \x02` \x01\x01Qa5\xDBV[s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0`\x01`\x01`\xA0\x1B\x03\x16\x84\x82\x81Q\x81\x10a*\x8DWa*\x8DaWEV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a+VW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xBE\xFF\xBB\x89\x88\x85\x84\x81Q\x81\x10a*\xE6Wa*\xE6aWEV[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a+\x1F\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a+9W`\0\x80\xFD[PZ\xF1\x15\x80\x15a+MW=`\0\x80>=`\0\xFD[PPPPa-\xA9V[\x84`\x01`\x01`\xA0\x1B\x03\x16\x87`\x01`\x01`\xA0\x1B\x03\x16\x14\x80a,(WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x9BM\xA0=\x85\x83\x81Q\x81\x10a+\xB2Wa+\xB2aWEV[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a+\xE5\x91\x90`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,\x02W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,&\x91\x90aYqV[\x15[a,\xF4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x84`$\x82\x01\x81\x90R\x7FDelegationManager._removeSharesA`D\x83\x01R\x7FndQueueWithdrawal: withdrawer mu`d\x83\x01R\x7Fst be same address as staker if \x90\x82\x01R\x7FthirdPartyTransfersForbidden are`\xA4\x82\x01Rc\x08\x1C\xD9]`\xE2\x1B`\xC4\x82\x01R`\xE4\x01a\x0B\x03V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8C\x80\xD4\xE5\x88\x86\x84\x81Q\x81\x10a-6Wa-6aWEV[` \x02` \x01\x01Q\x86\x85\x81Q\x81\x10a-PWa-PaWEV[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a-v\x93\x92\x91\x90a\\\xE0V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a-\x90W`\0\x80\xFD[PZ\xF1\x15\x80\x15a-\xA4W=`\0\x80>=`\0\xFD[PPPP[`\x01\x01a*\x07V[P`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x9F` R`@\x81 \x80T\x91\x82\x91\x90a-\xD9\x83aWqV[\x91\x90PUP`\0`@Q\x80`\xE0\x01`@R\x80\x89`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x88`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x87`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86\x81R` \x01\x85\x81RP\x90P`\0a.A\x82a\x16\xA6V[`\0\x81\x81R`\x9E` R`@\x90\x81\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x90\x91P\x7F\x90\t\xAB\x15>\x80\x14\xFB\xFB\x02\xF2!\x7F\\\xDEz\xA7\xF9\xADsJ\xE8\\\xA3\xEE?L\xA2\xFD\xD4\x99\xF9\x90a.\x8F\x90\x83\x90\x85\x90a[jV[`@Q\x80\x91\x03\x90\xA1\x98\x97PPPPPPPPV[`\0a.\xB2` \x83\x01\x83aLlV[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a/LW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`T`$\x82\x01R\x7FDelegationManager._setOperatorDe`D\x82\x01R\x7Ftails: cannot set `earningsRecei`d\x82\x01Rsver` to zero address``\x1B`\x84\x82\x01R`\xA4\x01a\x0B\x03V[b\x13\xC6\x80a/```\x83\x01`@\x84\x01a]\x04V[c\xFF\xFF\xFF\xFF\x16\x11\x15a0\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`l`$\x82\x01R\x7FDelegationManager._setOperatorDe`D\x82\x01R\x7Ftails: stakerOptOutWindowBlocks `d\x82\x01R\x7Fcannot be > MAX_STAKER_OPT_OUT_W`\x84\x82\x01RkINDOW_BLOCKS`\xA0\x1B`\xA4\x82\x01R`\xC4\x01a\x0B\x03V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x99` R`@\x90\x81\x90 `\x01\x01T`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x90a0Q\x90``\x84\x01\x90\x84\x01a]\x04V[c\xFF\xFF\xFF\xFF\x16\x10\x15a0\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`S`$\x82\x01R\x7FDelegationManager._setOperatorDe`D\x82\x01R\x7Ftails: stakerOptOutWindowBlocks `d\x82\x01Rr\x18\xD8[\x9B\x9B\xDD\x08\x18\x99H\x19\x19X\xDC\x99X\\\xD9Y`j\x1B`\x84\x82\x01R`\xA4\x01a\x0B\x03V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x99` R`@\x90 \x81\x90a1\x0B\x82\x82a]AV[PP`@Q3\x90\x7F\xFE\xBE\\\xD2K,\xBC{\x06[\x9D\x0F\xDE\xB9\x04F\x1EJ\xFC\xFFW\xDDW\xAC\xDA\x1Ex2\x03\x1B\xA7\xAC\x90a\x1C\xF5\x90\x84\x90aX,V[`fT`\0\x90`\x01\x90\x81\x16\x14\x15a1hW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\x03\x90aW\x8CV[a1q\x85a\x15\xBFV[\x15a1\xEEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FDelegationManager._delegate: sta`D\x82\x01R\x7Fker is already actively delegate`d\x82\x01R`\x19`\xFA\x1B`\x84\x82\x01R`\xA4\x01a\x0B\x03V[a1\xF7\x84a\x1A\x1CV[a2wW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R\x7FDelegationManager._delegate: ope`D\x82\x01R\x7Frator is not registered in Eigen`d\x82\x01Rd&0\xBC\xB2\xB9`\xD9\x1B`\x84\x82\x01R`\xA4\x01a\x0B\x03V[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T\x16\x80\x15\x80\x15\x90a2\xADWP3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14\x15[\x80\x15a2\xC2WP3`\x01`\x01`\xA0\x1B\x03\x86\x16\x14\x15[\x15a4/WB\x84` \x01Q\x10\x15a3AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FDelegationManager._delegate: app`D\x82\x01R\x7Frover signature expired\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\x03V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x9C` \x90\x81R`@\x80\x83 \x86\x84R\x90\x91R\x90 T`\xFF\x16\x15a3\xDBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FDelegationManager._delegate: app`D\x82\x01R\x7FroverSalt already spent\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\x03V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x9C` \x90\x81R`@\x80\x83 \x86\x84R\x82R\x82 \x80T`\xFF\x19\x16`\x01\x17\x90U\x85\x01Qa4\x1C\x90\x88\x90\x88\x90\x85\x90\x88\x90a\n\x15V[\x90Pa4-\x82\x82\x87`\0\x01QaD\x10V[P[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\0\x81\x81R`\x9A` R`@\x80\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x94\x8A\x16\x94\x85\x17\x90UQ\x7F\xC3\xEE\x9F._\xDA\x98\xE8\x06j\x1Ft[-\xF9(_Ao\xE9\x8C\xF2U\x9C\xD2\x14\x84\xB3\xD8t3\x04\x91\x90\xA3`\0\x80a4\x8E\x88a\x1D\xBAV[\x91P\x91P`\0[\x82Q\x81\x10\x15a\x13\xC7Wa4\xDC\x88\x8A\x85\x84\x81Q\x81\x10a4\xB5Wa4\xB5aWEV[` \x02` \x01\x01Q\x85\x85\x81Q\x81\x10a4\xCFWa4\xCFaWEV[` \x02` \x01\x01Qa;\xABV[`\x01\x01a4\x95V[`\x01`\x01`\xA0\x1B\x03\x81\x16a5rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x0B\x03V[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x98` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a6\x12\x90\x84\x90a\\\xC9V[\x92PP\x81\x90UP\x83`\x01`\x01`\xA0\x1B\x03\x16\x7Fi\t`\x007\xB7]{G3\xAE\xDD\x81TB\xB5\xEC\x01\x8A\x82wQ\xC82\xAA\xFFd\xEB\xA5\xD6\xD2\xDD\x84\x84\x84`@Qa\x0F\x95\x93\x92\x91\x90a\\\xE0V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1ANW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x0B\x03V[\x82\x81\x14a78W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`J`$\x82\x01R\x7FDelegationManager._setStrategyWi`D\x82\x01R\x7FthdrawalDelayBlocks: input lengt`d\x82\x01Ri\r\x04\r\xAD.m\xAC.\x8Cm`\xB3\x1B`\x84\x82\x01R`\xA4\x01a\x0B\x03V[\x82`\0[\x81\x81\x10\x15a8\xCEW`\0\x86\x86\x83\x81\x81\x10a7XWa7XaWEV[\x90P` \x02\x01` \x81\x01\x90a7m\x91\x90aLlV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\xA1` R`@\x81 T\x91\x92P\x86\x86\x85\x81\x81\x10a7\x9BWa7\x9BaWEV[\x90P` \x02\x015\x90Pb\x03K\xC0\x81\x11\x15a8_W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`s`$\x82\x01R\x7FDelegationManager._setStrategyWi`D\x82\x01R\x7FthdrawalDelayBlocks: _withdrawal`d\x82\x01R\x7FDelayBlocks cannot be > MAX_WITH`\x84\x82\x01RrDRAWAL_DELAY_BLOCKS`h\x1B`\xA4\x82\x01R`\xC4\x01a\x0B\x03V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81R`\xA1` \x90\x81R`@\x91\x82\x90 \x84\x90U\x81Q\x92\x83R\x82\x01\x84\x90R\x81\x01\x82\x90R\x7F\x0E~\xFAs\x8E\x8B\x0C\xE67j\x0C\x1A\xF4qeU@\xD2\xE9\xA8\x16G\xD7\xB0\x9E\xD8#\x01\x84&Wm\x90``\x01`@Q\x80\x91\x03\x90\xA1PPP\x80a8\xC7\x90aWqV[\x90Pa7<V[PPPPPPV[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a8\xF7WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a9yW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x0B\x03V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a\x19}\x82a4\xE4V[`@\x80Q\x80\x82\x01\x82R`\n\x81Ri\"\xB4\xB3\xB2\xB7&0\xBC\xB2\xB9`\xB1\x1B` \x91\x82\x01R\x81Q\x7F\x8C\xAD\x95h{\xA8,,\xE5\x0Et\xF7\xB7Td^Q\x17\xC3\xA5\xBE\xC8\x15\x1C\x07&\xD5\x85y\x80\xA8f\x81\x83\x01R\x7Fq\xB6%\xCF\xADD\xBA\xC6;\x13\xDB\xA0\x7F.\x1D`\x84\xEE\x04\xB6\xF8u!\x01\xEC\xE6\x12mXN\xE6\xEA\x81\x84\x01RF``\x82\x01R0`\x80\x80\x83\x01\x91\x90\x91R\x83Q\x80\x83\x03\x90\x91\x01\x81R`\xA0\x90\x91\x01\x90\x92R\x81Q\x91\x01 \x90V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[b\x03K\xC0\x81\x11\x15a;jW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`x`$\x82\x01R\x7FDelegationManager._initializeMin`D\x82\x01R\x7FWithdrawalDelayBlocks: _minWithd`d\x82\x01R\x7FrawalDelayBlocks cannot be > MAX`\x84\x82\x01R\x7F_WITHDRAWAL_DELAY_BLOCKS\0\0\0\0\0\0\0\0`\xA4\x82\x01R`\xC4\x01a\x0B\x03V[`\x9DT`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7F\xAF\xA0\x03\xCDv\xF8\x7F\xF9\xD6+5\xBE\xEA\x88\x99 \xF3<\x0CB\xB8\xD4[t\x95Ma\xD5\x0FKki\x91\x01`@Q\x80\x91\x03\x90\xA1`\x9DUV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x98` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a;\xE2\x90\x84\x90a\\\xB1V[\x92PP\x81\x90UP\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\x1E\xC0B\xC9e\xE2\xED\xD7\x10{Q\x18\x8E\xE0\xF3\x83\xE2.v\x17\x90A\xAB:\x9D\x18\xFF\x15\x14\x05\x16l\x84\x84\x84`@Qa\x0F\x95\x93\x92\x91\x90a\\\xE0V[`\0a<4a\x06\x0E\x87a]\xA4V[`\0\x81\x81R`\x9E` R`@\x90 T\x90\x91P`\xFF\x16a<\xB5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R`\0\x80Q` a^\xDC\x839\x81Q\x91R`D\x82\x01R\x7FdWithdrawal: action is not in qu`d\x82\x01Rbeue`\xE8\x1B`\x84\x82\x01R`\xA4\x01a\x0B\x03V[`\x9DTC\x90a<\xCA`\xA0\x89\x01`\x80\x8A\x01a]\x04V[c\xFF\xFF\xFF\xFF\x16a<\xDA\x91\x90a\\\xB1V[\x11\x15a=bW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`_`$\x82\x01R`\0\x80Q` a^\xDC\x839\x81Q\x91R`D\x82\x01R\x7FdWithdrawal: minWithdrawalDelayB`d\x82\x01R\x7Flocks period has not yet passed\0`\x84\x82\x01R`\xA4\x01a\x0B\x03V[a=r``\x87\x01`@\x88\x01aLlV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a=\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`P`$\x82\x01R`\0\x80Q` a^\xDC\x839\x81Q\x91R`D\x82\x01R\x7FdWithdrawal: only withdrawer can`d\x82\x01Ro\x101\xB7\xB6\xB862\xBA2\x900\xB1\xBA4\xB7\xB7`\x81\x1B`\x84\x82\x01R`\xA4\x01a\x0B\x03V[\x81\x15a>\x81Wa>\x12`\xA0\x87\x01\x87aW\xE3V[\x85\x14\x90Pa>\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`B`$\x82\x01R`\0\x80Q` a^\xDC\x839\x81Q\x91R`D\x82\x01R\x7FdWithdrawal: input length mismat`d\x82\x01Ra\x0Cm`\xF3\x1B`\x84\x82\x01R`\xA4\x01a\x0B\x03V[`\0\x81\x81R`\x9E` R`@\x90 \x80T`\xFF\x19\x16\x90U\x81\x15a?\xE6W`\0[a>\xAD`\xA0\x88\x01\x88aW\xE3V[\x90P\x81\x10\x15a?\xE0WC`\xA1`\0a>\xC8`\xA0\x8B\x01\x8BaW\xE3V[\x85\x81\x81\x10a>\xD8Wa>\xD8aWEV[\x90P` \x02\x01` \x81\x01\x90a>\xED\x91\x90aLlV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 Ta?\x17`\xA0\x8A\x01`\x80\x8B\x01a]\x04V[c\xFF\xFF\xFF\xFF\x16a?'\x91\x90a\\\xB1V[\x11\x15a?EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\x03\x90a]\xB6V[a?\xD8a?U` \x89\x01\x89aLlV[3a?c`\xA0\x8B\x01\x8BaW\xE3V[\x85\x81\x81\x10a?sWa?saWEV[\x90P` \x02\x01` \x81\x01\x90a?\x88\x91\x90aLlV[a?\x95`\xC0\x8C\x01\x8CaW\xE3V[\x86\x81\x81\x10a?\xA5Wa?\xA5aWEV[\x90P` \x02\x015\x8A\x8A\x87\x81\x81\x10a?\xBEWa?\xBEaWEV[\x90P` \x02\x01` \x81\x01\x90a?\xD3\x91\x90aLlV[aE\xCAV[`\x01\x01a>\xA0V[PaC\xD5V[3`\0\x90\x81R`\x9A` R`@\x81 T`\x01`\x01`\xA0\x1B\x03\x16\x90[a@\x0E`\xA0\x89\x01\x89aW\xE3V[\x90P\x81\x10\x15aC\xD2WC`\xA1`\0a@)`\xA0\x8C\x01\x8CaW\xE3V[\x85\x81\x81\x10a@9Wa@9aWEV[\x90P` \x02\x01` \x81\x01\x90a@N\x91\x90aLlV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 Ta@x`\xA0\x8B\x01`\x80\x8C\x01a]\x04V[c\xFF\xFF\xFF\xFF\x16a@\x88\x91\x90a\\\xB1V[\x11\x15a@\xA6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\x03\x90a]\xB6V[s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0a@\xC8`\xA0\x8A\x01\x8AaW\xE3V[\x83\x81\x81\x10a@\xD8Wa@\xD8aWEV[\x90P` \x02\x01` \x81\x01\x90a@\xED\x91\x90aLlV[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15aB=W`\0aA\x0B` \x8A\x01\x8AaLlV[\x90P`\0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\x0E\x81\x07<\x83aAL`\xC0\x8E\x01\x8EaW\xE3V[\x87\x81\x81\x10aA\\WaA\\aWEV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x87\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x94\x16`\x04\x85\x01R` \x02\x91\x90\x91\x015`$\x83\x01RP`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15aA\xB0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aA\xD4\x91\x90a[\x83V[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R`\x9A` R`@\x90 T\x91\x92P\x16\x80\x15aB5WaB5\x81\x84aB\n`\xA0\x8F\x01\x8FaW\xE3V[\x88\x81\x81\x10aB\x1AWaB\x1AaWEV[\x90P` \x02\x01` \x81\x01\x90aB/\x91\x90aLlV[\x85a;\xABV[PPPaC\xCAV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xC4b>\xA13\x89\x89\x85\x81\x81\x10aB\x7FWaB\x7FaWEV[\x90P` \x02\x01` \x81\x01\x90aB\x94\x91\x90aLlV[aB\xA1`\xA0\x8D\x01\x8DaW\xE3V[\x86\x81\x81\x10aB\xB1WaB\xB1aWEV[\x90P` \x02\x01` \x81\x01\x90aB\xC6\x91\x90aLlV[aB\xD3`\xC0\x8E\x01\x8EaW\xE3V[\x87\x81\x81\x10aB\xE3WaB\xE3aWEV[`@Q`\xE0\x88\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R`\x01`\x01`\xA0\x1B\x03\x96\x87\x16`\x04\x82\x01R\x94\x86\x16`$\x86\x01R\x92\x90\x94\x16`D\x84\x01R` \x90\x91\x02\x015`d\x82\x01R`\x84\x01\x90P`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aCCW`\0\x80\xFD[PZ\xF1\x15\x80\x15aCWW=`\0\x80>=`\0\xFD[PPPP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15aC\xCAWaC\xCA\x823aC|`\xA0\x8C\x01\x8CaW\xE3V[\x85\x81\x81\x10aC\x8CWaC\x8CaWEV[\x90P` \x02\x01` \x81\x01\x90aC\xA1\x91\x90aLlV[aC\xAE`\xC0\x8D\x01\x8DaW\xE3V[\x86\x81\x81\x10aC\xBEWaC\xBEaWEV[\x90P` \x02\x015a;\xABV[`\x01\x01a@\x01V[PP[`@Q\x81\x81R\x7F\xC9p\x98\xC2\xF6X\x80\x0BM\xF2\x90\x01R\x7Fs$\xBC\xDF\xFC\xF6\xE8u\x1Ai\x9A\xB9 \xA1\xEC\xED[\x1D\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16;\x15aE*W`@Qc\x0B\x13]?`\xE1\x1B\x80\x82R\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\x16&\xBA~\x90aDP\x90\x86\x90\x86\x90`\x04\x01a^>V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aDmW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aD\x91\x91\x90a^\x9BV[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14a\x11\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`S`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: ERC1271 signature `d\x82\x01Rr\x1D\x99\\\x9AY\x9AX\xD8]\x1A[\xDB\x88\x19\x98Z[\x19Y`j\x1B`\x84\x82\x01R`\xA4\x01a\x0B\x03V[\x82`\x01`\x01`\xA0\x1B\x03\x16aE>\x83\x83aG\nV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x11\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: signature not from`d\x82\x01Rf\x109\xB4\xB3\xB72\xB9`\xC9\x1B`\x84\x82\x01R`\xA4\x01a\x0B\x03V[`\x01`\x01`\xA0\x1B\x03\x83\x16s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x14\x15aFuW`@Qb8{\x13`\xE8\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c8{\x13\0\x90aF>\x90\x88\x90\x88\x90\x87\x90`\x04\x01a\\\xE0V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aFXW`\0\x80\xFD[PZ\xF1\x15\x80\x15aFlW=`\0\x80>=`\0\xFD[PPPPaG\x03V[`@Qc\xC6\x08\xC7\xF3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x84\x81\x16`$\x83\x01R`D\x82\x01\x84\x90R\x82\x81\x16`d\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xC6\x08\xC7\xF3\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aF\xEFW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x13\xC7W=`\0\x80>=`\0\xFD[PPPPPV[`\0\x80`\0aG\x19\x85\x85aG&V[\x91P\x91Pa\n\r\x81aG\x96V[`\0\x80\x82Q`A\x14\x15aG]W` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1AaGQ\x87\x82\x85\x85aIQV[\x94P\x94PPPPaG\x8FV[\x82Q`@\x14\x15aG\x87W` \x83\x01Q`@\x84\x01QaG|\x86\x83\x83aJ>V[\x93P\x93PPPaG\x8FV[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0\x81`\x04\x81\x11\x15aG\xAAWaG\xAAa^\xC5V[\x14\x15aG\xB3WPV[`\x01\x81`\x04\x81\x11\x15aG\xC7WaG\xC7a^\xC5V[\x14\x15aH\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x0B\x03V[`\x02\x81`\x04\x81\x11\x15aH)WaH)a^\xC5V[\x14\x15aHwW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x0B\x03V[`\x03\x81`\x04\x81\x11\x15aH\x8BWaH\x8Ba^\xC5V[\x14\x15aH\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x0B\x03V[`\x04\x81`\x04\x81\x11\x15aH\xF8WaH\xF8a^\xC5V[\x14\x15a\x10SW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 'v' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x0B\x03V[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15aI\x88WP`\0\x90P`\x03aJ5V[\x84`\xFF\x16`\x1B\x14\x15\x80\x15aI\xA0WP\x84`\xFF\x16`\x1C\x14\x15[\x15aI\xB1WP`\0\x90P`\x04aJ5V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15aJ\x05W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16aJ.W`\0`\x01\x92P\x92PPaJ5V[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x80`\x01`\x01`\xFF\x1B\x03\x83\x16\x81aJ[`\xFF\x86\x90\x1C`\x1Ba\\\xB1V[\x90PaJi\x87\x82\x88\x85aIQV[\x93P\x93PPP\x93P\x93\x91PPV[`\0\x80\x83`\x1F\x84\x01\x12aJ\x89W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aJ\xA0W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15aG\x8FW`\0\x80\xFD[`\0\x80` \x83\x85\x03\x12\x15aJ\xCEW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aJ\xE4W`\0\x80\xFD[aJ\xF0\x85\x82\x86\x01aJwV[\x90\x96\x90\x95P\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x10SW`\0\x80\xFD[\x805aK\x1C\x81aJ\xFCV[\x91\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15aK9W`\0\x80\xFD[\x855aKD\x81aJ\xFCV[\x94P` \x86\x015aKT\x81aJ\xFCV[\x93P`@\x86\x015aKd\x81aJ\xFCV[\x94\x97\x93\x96P\x93\x94``\x81\x015\x94P`\x80\x015\x92\x91PPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aK\xB4W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aK\x98V[P\x90\x96\x95PPPPPPV[`\0``\x82\x84\x03\x12\x15aK\xD2W`\0\x80\xFD[P\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12aK\xEAW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aL\x01W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aG\x8FW`\0\x80\xFD[`\0\x80`\0`\x80\x84\x86\x03\x12\x15aL.W`\0\x80\xFD[aL8\x85\x85aK\xC0V[\x92P``\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aLSW`\0\x80\xFD[aL_\x86\x82\x87\x01aK\xD8V[\x94\x97\x90\x96P\x93\x94PPPPV[`\0` \x82\x84\x03\x12\x15aL~W`\0\x80\xFD[\x815aL\x89\x81aJ\xFCV[\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15aL\xA5W`\0\x80\xFD[\x835aL\xB0\x81aJ\xFCV[\x92P` \x84\x015aL\xC0\x81aJ\xFCV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0` \x82\x84\x03\x12\x15aL\xE3W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15aM\0W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aM\x17W`\0\x80\xFD[aM#\x88\x83\x89\x01aJwV[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15aM<W`\0\x80\xFD[PaMI\x87\x82\x88\x01aJwV[\x95\x98\x94\x97P\x95PPPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\xC0\x89\x8B\x03\x12\x15aMqW`\0\x80\xFD[\x885aM|\x81aJ\xFCV[\x97P` \x89\x015aM\x8C\x81aJ\xFCV[\x96P`@\x89\x015\x95P``\x89\x015\x94P`\x80\x89\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aM\xB6W`\0\x80\xFD[aM\xC2\x8C\x83\x8D\x01aJwV[\x90\x96P\x94P`\xA0\x8B\x015\x91P\x80\x82\x11\x15aM\xDBW`\0\x80\xFD[PaM\xE8\x8B\x82\x8C\x01aJwV[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\x80\x89\x8B\x03\x12\x15aN\x18W`\0\x80\xFD[\x885`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aN/W`\0\x80\xFD[aN;\x8C\x83\x8D\x01aJwV[\x90\x9AP\x98P` \x8B\x015\x91P\x80\x82\x11\x15aNTW`\0\x80\xFD[aN`\x8C\x83\x8D\x01aJwV[\x90\x98P\x96P`@\x8B\x015\x91P\x80\x82\x11\x15aNyW`\0\x80\xFD[aN\x85\x8C\x83\x8D\x01aJwV[\x90\x96P\x94P``\x8B\x015\x91P\x80\x82\x11\x15aM\xDBW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aN\xD6WaN\xD6aN\x9EV[`@R\x90V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aN\xD6WaN\xD6aN\x9EV[`@Q`\xC0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aN\xD6WaN\xD6aN\x9EV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aOHWaOHaN\x9EV[`@R\x91\x90PV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x10SW`\0\x80\xFD[\x805aK\x1C\x81aOPV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15aO\x86WaO\x86aN\x9EV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12aO\xA1W`\0\x80\xFD[\x815` aO\xB6aO\xB1\x83aOmV[aO V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aO\xD5W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aO\xF9W\x805aO\xEC\x81aJ\xFCV[\x83R\x91\x83\x01\x91\x83\x01aO\xD9V[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12aP\x15W`\0\x80\xFD[\x815` aP%aO\xB1\x83aOmV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aPDW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aO\xF9W\x805\x83R\x91\x83\x01\x91\x83\x01aPHV[`\0`\xE0\x82\x84\x03\x12\x15aPqW`\0\x80\xFD[aPyaN\xB4V[\x90PaP\x84\x82aK\x11V[\x81RaP\x92` \x83\x01aK\x11V[` \x82\x01RaP\xA3`@\x83\x01aK\x11V[`@\x82\x01R``\x82\x015``\x82\x01RaP\xBE`\x80\x83\x01aObV[`\x80\x82\x01R`\xA0\x82\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aP\xDDW`\0\x80\xFD[aP\xE9\x85\x83\x86\x01aO\x90V[`\xA0\x84\x01R`\xC0\x84\x015\x91P\x80\x82\x11\x15aQ\x02W`\0\x80\xFD[PaQ\x0F\x84\x82\x85\x01aP\x04V[`\xC0\x83\x01RP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15aQ-W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aQCW`\0\x80\xFD[aQO\x84\x82\x85\x01aP_V[\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15aQiW`\0\x80\xFD[\x815`\xFF\x81\x16\x81\x14aL\x89W`\0\x80\xFD[`\0`@\x82\x84\x03\x12\x15aQ\x8CW`\0\x80\xFD[aQ\x94aN\xDCV[\x90P\x815aQ\xA1\x81aJ\xFCV[\x81R` \x82\x015k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14aQ\xC2W`\0\x80\xFD[` \x82\x01R\x92\x91PPV[`\0` \x80\x83\x85\x03\x12\x15aQ\xE0W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aQ\xF7W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12aR\x0BW`\0\x80\xFD[\x815aR\x19aO\xB1\x82aOmV[\x81\x81R`\x05\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x84\x81\x01\x90\x88\x83\x11\x15aR8W`\0\x80\xFD[\x85\x85\x01[\x83\x81\x10\x15aS\x12W\x805\x85\x81\x11\x15aRTW`\0\x80\x81\xFD[\x86\x01`\xE0\x81\x8C\x03`\x1F\x19\x01\x81\x13\x15aRlW`\0\x80\x81\xFD[aRtaN\xFEV[\x89\x83\x015\x88\x81\x11\x15aR\x86W`\0\x80\x81\xFD[aR\x94\x8E\x8C\x83\x87\x01\x01aO\x90V[\x82RP`@\x80\x84\x015\x89\x81\x11\x15aR\xABW`\0\x80\x81\xFD[aR\xB9\x8F\x8D\x83\x88\x01\x01aP\x04V[\x8C\x84\x01RP``aR\xCB\x81\x86\x01aK\x11V[\x82\x84\x01R`\x80\x91PaR\xDF\x8F\x83\x87\x01aQzV[\x90\x83\x01RaR\xEF`\xC0\x85\x01aObV[\x90\x82\x01RaR\xFE\x83\x83\x01aK\x11V[`\xA0\x82\x01R\x85RPP\x91\x86\x01\x91\x86\x01aR<V[P\x98\x97PPPPPPPPV[\x80\x15\x15\x81\x14a\x10SW`\0\x80\xFD[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15aSEW`\0\x80\xFD[\x855`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aS\\W`\0\x80\xFD[\x90\x87\x01\x90`\xE0\x82\x8A\x03\x12\x15aSpW`\0\x80\xFD[\x90\x95P` \x87\x015\x90\x80\x82\x11\x15aS\x86W`\0\x80\xFD[PaS\x93\x88\x82\x89\x01aJwV[\x90\x95P\x93PP`@\x86\x015\x91P``\x86\x015aS\xAE\x81aS\x1FV[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x80`@\x83\x85\x03\x12\x15aS\xCFW`\0\x80\xFD[\x825aS\xDA\x81aJ\xFCV[\x91P` \x83\x015aS\xEA\x81aJ\xFCV[\x80\x91PP\x92P\x92\x90PV[`\0`@\x82\x84\x03\x12\x15aT\x07W`\0\x80\xFD[aT\x0FaN\xDCV[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aT(W`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12aT<W`\0\x80\xFD[\x815` \x82\x82\x11\x15aTPWaTPaN\x9EV[aTb`\x1F\x83\x01`\x1F\x19\x16\x82\x01aO V[\x92P\x81\x83R\x86\x81\x83\x86\x01\x01\x11\x15aTxW`\0\x80\xFD[\x81\x81\x85\x01\x82\x85\x017`\0\x81\x83\x85\x01\x01R\x82\x85R\x80\x86\x015\x81\x86\x01RPPPP\x92\x91PPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15aT\xB5W`\0\x80\xFD[\x855aT\xC0\x81aJ\xFCV[\x94P` \x86\x015aT\xD0\x81aJ\xFCV[\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aT\xECW`\0\x80\xFD[aT\xF8\x89\x83\x8A\x01aS\xF5V[\x94P``\x88\x015\x91P\x80\x82\x11\x15aU\x0EW`\0\x80\xFD[PaU\x1B\x88\x82\x89\x01aS\xF5V[\x95\x98\x94\x97P\x92\x95`\x80\x015\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15aU@W`\0\x80\xFD[\x825aUK\x81aJ\xFCV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aUfW`\0\x80\xFD[aUr\x85\x82\x86\x01aO\x90V[\x91PP\x92P\x92\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aU\xACW\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aU\x90V[P\x94\x95\x94PPPPPV[` \x81R`\0aL\x89` \x83\x01\x84aU|V[`\0\x80` \x83\x85\x03\x12\x15aU\xDDW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aU\xF3W`\0\x80\xFD[aJ\xF0\x85\x82\x86\x01aK\xD8V[`\0\x80`@\x83\x85\x03\x12\x15aV\x12W`\0\x80\xFD[\x825aV\x1D\x81aJ\xFCV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aVAW`\0\x80\xFD[\x845aVL\x81aJ\xFCV[\x93P` \x85\x015\x92P`@\x85\x015aVc\x81aJ\xFCV[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aU\xACW\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aV\x87V[`@\x81R`\0aV\xBF`@\x83\x01\x85aVsV[\x82\x81\x03` \x84\x01Ra\x12\x85\x81\x85aU|V[`\0\x80`\0``\x84\x86\x03\x12\x15aV\xE6W`\0\x80\xFD[\x835aV\xF1\x81aJ\xFCV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aW\x0CW`\0\x80\xFD[aW\x18\x86\x82\x87\x01aS\xF5V[\x92PP`@\x84\x015\x90P\x92P\x92P\x92V[`\0``\x82\x84\x03\x12\x15aW;W`\0\x80\xFD[aL\x89\x83\x83aK\xC0V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15aW\x85WaW\x85aW[V[P`\x01\x01\x90V[` \x80\x82R`\x19\x90\x82\x01R\x7FPausable: index is paused\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`\0\x825`^\x19\x836\x03\x01\x81\x12aW\xD9W`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aW\xFAW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aX\x14W`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15aG\x8FW`\0\x80\xFD[``\x81\x01\x825aX;\x81aJ\xFCV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x84\x015\x90aXW\x82aJ\xFCV[\x16` \x83\x01R`@\x83\x015aXk\x81aOPV[c\xFF\xFF\xFF\xFF\x81\x16`@\x84\x01RP\x92\x91PPV[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017`\0\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x91\x90PV[`\0` \x82\x84\x03\x12\x15aX\xBFW`\0\x80\xFD[\x81QaL\x89\x81aJ\xFCV[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`7\x90\x82\x01R\x7FDelegationManager: onlyStrategyM`@\x82\x01R\x7FanagerOrEigenPodManager\0\0\0\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15aY\x83W`\0\x80\xFD[\x81QaL\x89\x81aS\x1FV[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[`\0\x825`\xDE\x19\x836\x03\x01\x81\x12aW\xD9W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aY\xFEW`\0\x80\xFD[\x815aL\x89\x81aS\x1FV[`\0`\x01\x80`\xA0\x1B\x03\x80\x83Q\x16\x84R\x80` \x84\x01Q\x16` \x85\x01R\x80`@\x84\x01Q\x16`@\x85\x01RP``\x82\x01Q``\x84\x01Rc\xFF\xFF\xFF\xFF`\x80\x83\x01Q\x16`\x80\x84\x01R`\xA0\x82\x01Q`\xE0`\xA0\x85\x01RaZd`\xE0\x85\x01\x82aVsV[\x90P`\xC0\x83\x01Q\x84\x82\x03`\xC0\x86\x01Ra\x12\x85\x82\x82aU|V[` \x81R`\0aL\x89` \x83\x01\x84aZ\tV[` \x81R`\0\x82Q`\xE0` \x84\x01RaZ\xADa\x01\0\x84\x01\x82aVsV[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01RaZ\xCA\x82\x82aU|V[\x91PP`@\x84\x01Q`\x01\x80`\xA0\x1B\x03\x80\x82\x16``\x86\x01R``\x86\x01Q\x91P\x80\x82Q\x16`\x80\x86\x01RPk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16`\xA0\x85\x01RP`\x80\x84\x01Qa[\"`\xC0\x85\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P`\xA0\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16`\xE0\x85\x01Ra\n\rV[`\0\x80`@\x83\x85\x03\x12\x15a[OW`\0\x80\xFD[\x82Qa[Z\x81aS\x1FV[` \x93\x90\x93\x01Q\x92\x94\x92\x93PPPV[\x82\x81R`@` \x82\x01R`\0aQO`@\x83\x01\x84aZ\tV[`\0` \x82\x84\x03\x12\x15a[\x95W`\0\x80\xFD[PQ\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a[\xADW`\0\x80\xFD[\x81Q` a[\xBDaO\xB1\x83aOmV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a[\xDCW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aO\xF9W\x80Q\x83R\x91\x83\x01\x91\x83\x01a[\xE0V[`\0\x80`@\x83\x85\x03\x12\x15a\\\nW`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\\!W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\\5W`\0\x80\xFD[\x81Q` a\\EaO\xB1\x83aOmV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a\\dW`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a\\\x8BW\x85Qa\\|\x81aJ\xFCV[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a\\iV[\x91\x88\x01Q\x91\x96P\x90\x93PPP\x80\x82\x11\x15a\\\xA4W`\0\x80\xFD[PaUr\x85\x82\x86\x01a[\x9CV[`\0\x82\x19\x82\x11\x15a\\\xC4Wa\\\xC4aW[V[P\x01\x90V[`\0\x82\x82\x10\x15a\\\xDBWa\\\xDBaW[V[P\x03\x90V[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[`\0` \x82\x84\x03\x12\x15a]\x16W`\0\x80\xFD[\x815aL\x89\x81aOPV[\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\x815a]L\x81aJ\xFCV[a]V\x81\x83a]!V[P`\x01\x81\x01` \x83\x015a]i\x81aJ\xFCV[a]s\x81\x83a]!V[P`@\x83\x015a]\x82\x81aOPV[\x81Tc\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x16`\xA0\x91\x90\x91\x1Bc\xFF\xFF\xFF\xFF`\xA0\x1B\x16\x17\x90UPPV[`\0a]\xB06\x83aP_V[\x92\x91PPV[` \x80\x82R`n\x90\x82\x01R`\0\x80Q` a^\xDC\x839\x81Q\x91R`@\x82\x01R\x7FdWithdrawal: withdrawalDelayBloc``\x82\x01R\x7Fks period has not yet passed for`\x80\x82\x01Rm this strategy`\x90\x1B`\xA0\x82\x01R`\xC0\x01\x90V[\x82\x81R`\0` `@\x81\x84\x01R\x83Q\x80`@\x85\x01R`\0[\x81\x81\x10\x15a^rW\x85\x81\x01\x83\x01Q\x85\x82\x01``\x01R\x82\x01a^VV[\x81\x81\x11\x15a^\x84W`\0``\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01``\x01\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a^\xADW`\0\x80\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14aL\x89W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD\xFEDelegationManager._completeQueue\xA2dipfsX\"\x12 \x12Q\x8D\x84A\xBC\x8F\x89pd\xC3\xB0XM\xEDW\x8EN\x1F %\xDE\xFF\xE4@\xBB$\xB0i@*\xC7dsolcC\0\x08\x0C\x003";
    /// The bytecode of the contract.
    pub static DELEGATIONMANAGER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x03]W`\x005`\xE0\x1C\x80c_\x96o\x14\x11a\x01\xD3W\x80c\xB7\xF0n\xBE\x11a\x01\x04W\x80c\xCF\x80\x87>\x11a\0\xA2W\x80c\xF1ar\xB0\x11a\0|W\x80c\xF1ar\xB0\x14a\tOW\x80c\xF2\xFD\xE3\x8B\x14a\tbW\x80c\xF6\x98\xDA%\x14a\tuW\x80c\xFA\xBC\x1C\xBC\x14a\t}W`\0\x80\xFD[\x80c\xCF\x80\x87>\x14a\t\x08W\x80c\xDA\x8B\xE8d\x14a\t)W\x80c\xEE\xA9\x06K\x14a\t<W`\0\x80\xFD[\x80c\xC4\x887Z\x11a\0\xDEW\x80c\xC4\x887Z\x14a\x08%W\x80c\xC5\xE4\x80\xDB\x14a\x08EW\x80c\xC9KQ\x11\x14a\x08\xEBW\x80c\xCAf\x1C\x04\x14a\x08\xFEW`\0\x80\xFD[\x80c\xB7\xF0n\xBE\x14a\x07\xCBW\x80c\xBBE\xFE\xF2\x14a\x07\xEEW\x80c\xC4H\xFE\xB8\x14a\x08\x1CW`\0\x80\xFD[\x80c\x88o\x11\x95\x11a\x01qW\x80c\x91\x04\xC3\x19\x11a\x01KW\x80c\x91\x04\xC3\x19\x14a\x07VW\x80c\x99\xBE\x81\xC8\x14a\x07qW\x80c\xA1x\x84\x84\x14a\x07\x84W\x80c\xB14Bq\x14a\x07\xA4W`\0\x80\xFD[\x80c\x88o\x11\x95\x14a\x07\x12W\x80c\x8D\xA5\xCB[\x14a\x07%W\x80c\x90\x04\x13G\x14a\x076W`\0\x80\xFD[\x80cmp\xF7\xAE\x11a\x01\xADW\x80cmp\xF7\xAE\x14a\x06\xB9W\x80cqP\x18\xA6\x14a\x06\xCCW\x80cw\x8EU\xF3\x14a\x06\xD4W\x80c\x7FT\x80q\x14a\x06\xFFW`\0\x80\xFD[\x80c_\x96o\x14\x14a\x06QW\x80c`\xD7\xFA\xED\x14a\x06}W\x80ce\xDA\x12d\x14a\x06\x90W`\0\x80\xFD[\x80c(\xA5s\xAE\x11a\x02\xADW\x80cFe\xBC\xDA\x11a\x02KW\x80cY{6\xDA\x11a\x02%W\x80cY{6\xDA\x14a\x06\0W\x80cZ\xC8j\xB7\x14a\x06\x13W\x80c\\\x97Z\xBB\x14a\x066W\x80c\\\xFE\x8D,\x14a\x06>W`\0\x80\xFD[\x80cFe\xBC\xDA\x14a\x05\xC7W\x80cO\xC4\x0Ba\x14a\x05\xEEW\x80cY\\jg\x14a\x05\xF8W`\0\x80\xFD[\x80c9\xB7\x0E8\x11a\x02\x87W\x80c9\xB7\x0E8\x14a\x05\x0FW\x80c<\xDE\xB5\xE0\x14a\x05NW\x80c>(9\x1D\x14a\x05}W\x80cC7s\x82\x14a\x05\xA0W`\0\x80\xFD[\x80c(\xA5s\xAE\x14a\x04\xC9W\x80c)\xC7}O\x14a\x04\xDCW\x80c3@C\x96\x14a\x04\xFCW`\0\x80\xFD[\x80c\x13-Ig\x11a\x03\x1AW\x80c\x16\x92\x83e\x11a\x02\xF4W\x80c\x16\x92\x83e\x14a\x04CW\x80c\x1B\xBC\xE0\x91\x14a\x04|W\x80c `kp\x14a\x04\x8FW\x80c\"\xBF@\xE4\x14a\x04\xB6W`\0\x80\xFD[\x80c\x13-Ig\x14a\x04\nW\x80c\x13d9\xDD\x14a\x04\x1DW\x80c\x15\"\xBF\x02\x14a\x040W`\0\x80\xFD[\x80c\x04I\xCA9\x14a\x03bW\x80c\x04\xA4\xF9y\x14a\x03\x88W\x80c\x0B\x9FHz\x14a\x03\xAFW\x80c\r\xD8\xDD\x02\x14a\x03\xC2W\x80c\x0FX\x9EY\x14a\x03\xE2W\x80c\x10\xD6z/\x14a\x03\xF7W[`\0\x80\xFD[a\x03ua\x03p6`\x04aJ\xBBV[a\t\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x03u\x7F;\x89\xFC\xA1Q\xCB\xE5\x12-X\xAC\xEE\x86\xCF\x18D\x13\xD7Q\xD5\x85w\x9B\xDD\x19\xD3\xBB\xFA:0m\xCE\x81V[a\x03ua\x03\xBD6`\x04aK!V[a\n\x15V[a\x03\xD5a\x03\xD06`\x04aJ\xBBV[a\n\xD7V[`@Qa\x03\x7F\x91\x90aK|V[a\x03\xF5a\x03\xF06`\x04aL\x19V[a\x0ESV[\0[a\x03\xF5a\x04\x056`\x04aLlV[a\x0F\xA3V[a\x03\xF5a\x04\x186`\x04aL\x90V[a\x10VV[a\x03\xF5a\x04+6`\x04aL\xD1V[a\x11\rV[a\x03\xF5a\x04>6`\x04aL\xEAV[a\x12LV[a\x03ua\x04Q6`\x04aLlV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x90V[a\x03ua\x04\x8A6`\x04aL\x90V[a\x12`V[a\x03u\x7F\x8C\xAD\x95h{\xA8,,\xE5\x0Et\xF7\xB7Td^Q\x17\xC3\xA5\xBE\xC8\x15\x1C\x07&\xD5\x85y\x80\xA8f\x81V[a\x03\xF5a\x04\xC46`\x04aMUV[a\x12\x8EV[a\x03\xF5a\x04\xD76`\x04aL\x90V[a\x13\xD2V[a\x03ua\x04\xEA6`\x04aLlV[`\x9B` R`\0\x90\x81R`@\x90 T\x81V[a\x03\xF5a\x05\n6`\x04aM\xFCV[a\x14\x82V[a\x056\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03\x7FV[a\x056a\x05\\6`\x04aLlV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T\x16\x90V[a\x05\x90a\x05\x8B6`\x04aLlV[a\x15\xBFV[`@Q\x90\x15\x15\x81R` \x01a\x03\x7FV[a\x03u\x7F9\x11\x1B\xC4\xA4\xD6\x88\xE1\xF6\x85\x12=t\x97\xD4aSp\x15*\x8E\xE4\xA0Y>d{\xD0j\xD8\xBB\x0B\x81V[a\x056\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03ub\x13\xC6\x80\x81V[a\x03\xF5a\x15\xDFV[a\x03ua\x06\x0E6`\x04aQ\x1BV[a\x16\xA6V[a\x05\x90a\x06!6`\x04aQWV[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`fTa\x03uV[a\x03\xF5a\x06L6`\x04aQ\xCDV[a\x16\xD6V[a\x056a\x06_6`\x04aLlV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x99` R`@\x90 T\x16\x90V[a\x03\xF5a\x06\x8B6`\x04aS-V[a\x19\x81V[a\x056a\x06\x9E6`\x04aLlV[`\x9A` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x05\x90a\x06\xC76`\x04aLlV[a\x1A\x1CV[a\x03\xF5a\x1A<V[a\x03ua\x06\xE26`\x04aS\xBCV[`\x98` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[a\x03\xF5a\x07\r6`\x04aT\x9DV[a\x1APV[`eTa\x056\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x056V[a\x07Ia\x07D6`\x04aU-V[a\x1BUV[`@Qa\x03\x7F\x91\x90aU\xB7V[a\x056s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x81V[a\x03\xF5a\x07\x7F6`\x04aU\xCAV[a\x1C/V[a\x03ua\x07\x926`\x04aLlV[`\x9F` R`\0\x90\x81R`@\x90 T\x81V[a\x056\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x05\x90a\x07\xD96`\x04aL\xD1V[`\x9E` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x05\x90a\x07\xFC6`\x04aU\xFFV[`\x9C` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[a\x03u`\x9DT\x81V[a\x03ua\x0836`\x04aLlV[`\xA1` R`\0\x90\x81R`@\x90 T\x81V[a\x08\xB5a\x08S6`\x04aLlV[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x92\x84\x01\x81\x90R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R`\x99\x83R\x83\x90 \x83Q\x91\x82\x01\x84R\x80T\x85\x16\x82R`\x01\x01T\x93\x84\x16\x91\x81\x01\x91\x90\x91R`\x01`\xA0\x1B\x90\x92\x04c\xFF\xFF\xFF\xFF\x16\x90\x82\x01R\x90V[`@\x80Q\x82Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x80\x85\x01Q\x90\x91\x16\x90\x82\x01R\x91\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R``\x01a\x03\x7FV[a\x03ua\x08\xF96`\x04aV+V[a\x1D\x01V[a\x03ub\x03K\xC0\x81V[a\t\x1Ba\t\x166`\x04aLlV[a\x1D\xBAV[`@Qa\x03\x7F\x92\x91\x90aV\xACV[a\x03\xD5a\t76`\x04aLlV[a!rV[a\x03\xF5a\tJ6`\x04aV\xD1V[a&6V[a\x03\xF5a\t]6`\x04aW)V[a&BV[a\x03\xF5a\tp6`\x04aLlV[a&\xD3V[a\x03ua'IV[a\x03\xF5a\t\x8B6`\x04aL\xD1V[a'\x87V[`\x9DT`\0\x90\x81[\x83\x81\x10\x15a\n\rW`\0`\xA1`\0\x87\x87\x85\x81\x81\x10a\t\xB8Wa\t\xB8aWEV[\x90P` \x02\x01` \x81\x01\x90a\t\xCD\x91\x90aLlV[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 T\x90P\x82\x81\x11\x15a\t\xFCW\x80\x92P[Pa\n\x06\x81aWqV[\x90Pa\t\x98V[P\x93\x92PPPV[`@\x80Q\x7F;\x89\xFC\xA1Q\xCB\xE5\x12-X\xAC\xEE\x86\xCF\x18D\x13\xD7Q\xD5\x85w\x9B\xDD\x19\xD3\xBB\xFA:0m\xCE` \x80\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x83\x85\x01R\x88\x81\x16``\x84\x01R\x87\x16`\x80\x83\x01R`\xA0\x82\x01\x85\x90R`\xC0\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\xE0\x90\x92\x01\x90\x92R\x80Q\x91\x01 `\0\x90\x81a\n\x93a'IV[`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x91\x90\x91R`B\x81\x01\x83\x90R`b\x01`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x98\x97PPPPPPPPV[`fT``\x90`\x01\x90`\x02\x90\x81\x16\x14\x15a\x0B\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\x03\x90aW\x8CV[`@Q\x80\x91\x03\x90\xFD[`\0\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0B&Wa\x0B&aN\x9EV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0BOW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P3`\0\x90\x81R`\x9A` R`@\x81 T\x91\x92P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90[\x85\x81\x10\x15a\x0EHW\x86\x86\x82\x81\x81\x10a\x0B\x8AWa\x0B\x8AaWEV[\x90P` \x02\x81\x01\x90a\x0B\x9C\x91\x90aW\xC3V[a\x0B\xAA\x90` \x81\x01\x90aW\xE3V[\x90P\x87\x87\x83\x81\x81\x10a\x0B\xBEWa\x0B\xBEaWEV[\x90P` \x02\x81\x01\x90a\x0B\xD0\x91\x90aW\xC3V[a\x0B\xDA\x90\x80aW\xE3V[\x90P\x14a\x0COW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FDelegationManager.queueWithdrawa`D\x82\x01R\x7Fl: input length mismatch\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\x03V[`\0\x87\x87\x83\x81\x81\x10a\x0CcWa\x0CcaWEV[\x90P` \x02\x81\x01\x90a\x0Cu\x91\x90aW\xC3V[a\x0C\x86\x90``\x81\x01\x90`@\x01aLlV[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\r\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`H`$\x82\x01R\x7FDelegationManager.queueWithdrawa`D\x82\x01R\x7Fl: must provide valid withdrawal`d\x82\x01Rg address`\xC0\x1B`\x84\x82\x01R`\xA4\x01a\x0B\x03V[a\x0E\x193\x83\x89\x89\x85\x81\x81\x10a\r+Wa\r+aWEV[\x90P` \x02\x81\x01\x90a\r=\x91\x90aW\xC3V[a\rN\x90``\x81\x01\x90`@\x01aLlV[\x8A\x8A\x86\x81\x81\x10a\r`Wa\r`aWEV[\x90P` \x02\x81\x01\x90a\rr\x91\x90aW\xC3V[a\r|\x90\x80aW\xE3V[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8E\x92P\x8D\x91P\x88\x90P\x81\x81\x10a\r\xC2Wa\r\xC2aWEV[\x90P` \x02\x81\x01\x90a\r\xD4\x91\x90aW\xC3V[a\r\xE2\x90` \x81\x01\x90aW\xE3V[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa(\xE3\x92PPPV[\x83\x82\x81Q\x81\x10a\x0E+Wa\x0E+aWEV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a\x0E@\x81aWqV[\x91PPa\x0BpV[P\x90\x95\x94PPPPPV[3`\0\x90\x81R`\x99` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x0E\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R\x7FDelegationManager.registerAsOper`D\x82\x01R\x7Fator: operator has already regis`d\x82\x01Rd\x1D\x19\\\x99Y`\xDA\x1B`\x84\x82\x01R`\xA4\x01a\x0B\x03V[a\x0E\xF73\x84a.\xA3V[`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01Ra\x0F\x193\x80\x83`\0a1?V[3`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8E\x84\x85X:#\x10\xD4\x1F|\x82\xB9B}\x0B\xD4\x9B\xADt\xBB\x9C\xFF\x9D4\x02\xA2\x9D\x8F\x9B(\xA0\xE2\x85`@Qa\x0FR\x91\x90aX,V[`@Q\x80\x91\x03\x90\xA23`\x01`\x01`\xA0\x1B\x03\x16\x7F\x02\xA9\x19\xED\x0E*\xCA\xD1\xDD\x90\xF1~\xF2\xFAJ\xE5F.\xE13\x91p\x03J\x851\xCC\xA4\xB6p\x80\x90\x84\x84`@Qa\x0F\x95\x92\x91\x90aX~V[`@Q\x80\x91\x03\x90\xA2PPPPV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xF6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x1A\x91\x90aX\xADV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x10JW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\x03\x90aX\xCAV[a\x10S\x81a4\xE4V[PV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x10\xB5WP3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14[a\x10\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\x03\x90aY\x14V[a\x10\xDA\x83a\x15\xBFV[\x15a\x11\x08W`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R`\x9A` R`@\x90 T\x16a\x11\x06\x81\x85\x85\x85a5\xDBV[P[PPPV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11UW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11y\x91\x90aYqV[a\x11\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\x03\x90aY\x8EV[`fT\x81\x81\x16\x14a\x12\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\x03V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[a\x12Ta6VV[a\x11\x06\x84\x84\x84\x84a6\xB0V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x9B` R`@\x81 Ta\x12\x85\x85\x82\x86\x86a\x1D\x01V[\x95\x94PPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x12\xAEWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x12\xC8WP0;\x15\x80\x15a\x12\xC8WP`\0T`\xFF\x16`\x01\x14[a\x13+W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x0B\x03V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x13NW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x13X\x88\x88a8\xD6V[a\x13`a9\xBCV[`\x97Ua\x13l\x89a:SV[a\x13u\x86a:\xA5V[a\x13\x81\x85\x85\x85\x85a6\xB0V[\x80\x15a\x13\xC7W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x141WP3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14[a\x14MW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\x03\x90aY\x14V[a\x14V\x83a\x15\xBFV[\x15a\x11\x08W`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R`\x9A` R`@\x90 T\x16a\x11\x06\x81\x85\x85\x85a;\xABV[`fT`\x02\x90`\x04\x90\x81\x16\x14\x15a\x14\xABW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\x03\x90aW\x8CV[`\x02`\xC9T\x14\x15a\x14\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x0B\x03V[`\x02`\xC9U`\0[\x88\x81\x10\x15a\x15\xAEWa\x15\x9E\x8A\x8A\x83\x81\x81\x10a\x15#Wa\x15#aWEV[\x90P` \x02\x81\x01\x90a\x155\x91\x90aY\xD6V[\x89\x89\x84\x81\x81\x10a\x15GWa\x15GaWEV[\x90P` \x02\x81\x01\x90a\x15Y\x91\x90aW\xE3V[\x89\x89\x86\x81\x81\x10a\x15kWa\x15kaWEV[\x90P` \x02\x015\x88\x88\x87\x81\x81\x10a\x15\x84Wa\x15\x84aWEV[\x90P` \x02\x01` \x81\x01\x90a\x15\x99\x91\x90aY\xECV[a<&V[a\x15\xA7\x81aWqV[\x90Pa\x15\x06V[PP`\x01`\xC9UPPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x9A` R`@\x90 T\x16\x15\x15\x90V[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16'W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16K\x91\x90aYqV[a\x16gW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\x03\x90aY\x8EV[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[`\0\x81`@Q` \x01a\x16\xB9\x91\x90aZ}V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0[\x81Q\x81\x10\x15a\x19}W`\0\x82\x82\x81Q\x81\x10a\x16\xF6Wa\x16\xF6aWEV[` \x02` \x01\x01Q\x90P`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xCD)?o\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17O\x91\x90aZ\x90V[`@\x80Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x17mW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\x91\x91\x90a[<V[\x91P\x91P\x81\x15a\x19oW`@\x80\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x9F` R\x91\x82 \x80T\x91\x92\x82\x91\x90a\x17\xC8\x83aWqV[\x91\x90PUP`\0`@Q\x80`\xE0\x01`@R\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x87`\xA0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x87``\x01Q`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01\x87`\x80\x01Qc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x87`\0\x01Q\x81R` \x01\x87` \x01Q\x81RP\x90P`\0a\x18H\x82a\x16\xA6V[`\0\x81\x81R`\x9E` R`@\x90 T\x90\x91P`\xFF\x16\x15a\x18\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R\x7FDelegationManager.migrateQueuedW`D\x82\x01R\x7Fithdrawals: withdrawal already e`d\x82\x01Rdxists`\xD8\x1B`\x84\x82\x01R`\xA4\x01a\x0B\x03V[`\0\x81\x81R`\x9E` R`@\x90\x81\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x7F\x90\t\xAB\x15>\x80\x14\xFB\xFB\x02\xF2!\x7F\\\xDEz\xA7\xF9\xADsJ\xE8\\\xA3\xEE?L\xA2\xFD\xD4\x99\xF9\x90a\x19)\x90\x83\x90\x85\x90a[jV[`@Q\x80\x91\x03\x90\xA1`@\x80Q\x86\x81R` \x81\x01\x83\x90R\x7F\xDC\0u\x8Be\xEE\xF7\x1D\xC3x\x0C\x04\xEB\xE3l\xABk\xDB&l:i\x81\x87\xE2\x9E\x0F\r\xCA\x01&0\x91\x01`@Q\x80\x91\x03\x90\xA1PPPP[\x83`\x01\x01\x93PPPPa\x16\xD9V[PPV[`fT`\x02\x90`\x04\x90\x81\x16\x14\x15a\x19\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\x03\x90aW\x8CV[`\x02`\xC9T\x14\x15a\x19\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x0B\x03V[`\x02`\xC9Ua\x1A\x0F\x86\x86\x86\x86\x86a<&V[PP`\x01`\xC9UPPPPV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x99` R`@\x90 T\x16\x15\x15\x90V[a\x1ADa6VV[a\x1AN`\0a:SV[V[B\x83` \x01Q\x10\x15a\x1A\xD4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FDelegationManager.delegateToBySi`D\x82\x01R\x7Fgnature: staker signature expire`d\x82\x01R`\x19`\xFA\x1B`\x84\x82\x01R`\xA4\x01a\x0B\x03V[`\0`\x9B`\0\x87`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 T\x90P`\0a\x1B\x10\x87\x83\x88\x88` \x01Qa\x1D\x01V[`\x01`\x01`\xA0\x1B\x03\x88\x16`\0\x90\x81R`\x9B` R`@\x90 `\x01\x84\x01\x90U\x85Q\x90\x91Pa\x1B@\x90\x88\x90\x83\x90aD\x10V[a\x1BL\x87\x87\x86\x86a1?V[PPPPPPPV[```\0\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1BrWa\x1BraN\x9EV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1B\x9BW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\n\rW`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x98` R`@\x81 \x85Q\x90\x91\x90\x86\x90\x84\x90\x81\x10a\x1B\xD9Wa\x1B\xD9aWEV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 T\x82\x82\x81Q\x81\x10a\x1C\x14Wa\x1C\x14aWEV[` \x90\x81\x02\x91\x90\x91\x01\x01Ra\x1C(\x81aWqV[\x90Pa\x1B\xA1V[a\x1C83a\x1A\x1CV[a\x1C\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FDelegationManager.updateOperator`D\x82\x01R\x7FMetadataURI: caller must be an o`d\x82\x01Rf82\xB90\xBA7\xB9`\xC9\x1B`\x84\x82\x01R`\xA4\x01a\x0B\x03V[3`\x01`\x01`\xA0\x1B\x03\x16\x7F\x02\xA9\x19\xED\x0E*\xCA\xD1\xDD\x90\xF1~\xF2\xFAJ\xE5F.\xE13\x91p\x03J\x851\xCC\xA4\xB6p\x80\x90\x83\x83`@Qa\x1C\xF5\x92\x91\x90aX~V[`@Q\x80\x91\x03\x90\xA2PPV[`@\x80Q\x7F9\x11\x1B\xC4\xA4\xD6\x88\xE1\xF6\x85\x12=t\x97\xD4aSp\x15*\x8E\xE4\xA0Y>d{\xD0j\xD8\xBB\x0B` \x80\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x83\x85\x01R\x85\x16``\x83\x01R`\x80\x82\x01\x86\x90R`\xA0\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\xC0\x90\x92\x01\x90\x92R\x80Q\x91\x01 `\0\x90\x81a\x1Dwa'IV[`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x91\x90\x91R`B\x81\x01\x83\x90R`b\x01`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x97\x96PPPPPPPV[`@Qc`\xF4\x06+`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R``\x91\x82\x91`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c`\xF4\x06+\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E*W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1EN\x91\x90a[\x83V[`@Qc\x94\xF6I\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R\x91\x92P`\0\x91\x82\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\x94\xF6I\xDD\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\xBEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1E\xE6\x91\x90\x81\x01\x90a[\xF7V[\x91P\x91P`\0\x83\x13a\x1E\xFDW\x90\x95\x90\x94P\x92PPPV[``\x80\x83Q`\0\x14\x15a\x1F\xB7W`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x90` \x80\x83\x01\x90\x806\x837PP`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x92\x94P\x90P` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90Ps\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x82`\0\x81Q\x81\x10a\x1FrWa\x1FraWEV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x84\x81`\0\x81Q\x81\x10a\x1F\xA6Wa\x1F\xA6aWEV[` \x02` \x01\x01\x81\x81RPPa!eV[\x83Qa\x1F\xC4\x90`\x01a\\\xB1V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\xDBWa\x1F\xDBaN\x9EV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a \x04W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a  Wa  aN\x9EV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a IW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x84Q\x81\x10\x15a \xE3W\x84\x81\x81Q\x81\x10a jWa jaWEV[` \x02` \x01\x01Q\x83\x82\x81Q\x81\x10a \x84Wa \x84aWEV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x83\x81\x81Q\x81\x10a \xB6Wa \xB6aWEV[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a \xD0Wa \xD0aWEV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a OV[Ps\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x82`\x01\x84Qa!\x08\x91\x90a\\\xC9V[\x81Q\x81\x10a!\x18Wa!\x18aWEV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x84\x81`\x01\x84Qa!H\x91\x90a\\\xC9V[\x81Q\x81\x10a!XWa!XaWEV[` \x02` \x01\x01\x81\x81RPP[\x90\x97\x90\x96P\x94PPPPPV[`fT``\x90`\x01\x90`\x02\x90\x81\x16\x14\x15a!\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\x03\x90aW\x8CV[a!\xA7\x83a\x15\xBFV[a\"'W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FDelegationManager.undelegate: st\x90\x82\x01R\x7Faker must be delegated to undele`d\x82\x01Rcgate`\xE0\x1B`\x84\x82\x01R`\xA4\x01a\x0B\x03V[a\"0\x83a\x1A\x1CV[\x15a\"\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FDelegationManager.undelegate: op`D\x82\x01R\x7Ferators cannot be undelegated\0\0\0`d\x82\x01R`\x84\x01a\x0B\x03V[`\x01`\x01`\xA0\x1B\x03\x83\x16a#\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R\x7FDelegationManager.undelegate: ca`D\x82\x01R\x7Fnnot undelegate zero address\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\x03V[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x81\x81R`\x9A` R`@\x90 T\x90\x91\x16\x903\x14\x80a#RWP3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14[\x80a#yWP`\x01`\x01`\xA0\x1B\x03\x81\x81\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T\x163\x14[a#\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FDelegationManager.undelegate: ca`D\x82\x01R\x7Fller cannot undelegate staker\0\0\0`d\x82\x01R`\x84\x01a\x0B\x03V[`\0\x80a#\xF7\x86a\x1D\xBAV[\x90\x92P\x90P3`\x01`\x01`\xA0\x1B\x03\x87\x16\x14a$MW\x82`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x7F\xF0\xED\xDF\x07\xE6\xEA\x14\xF3\x88\xB4~\x1E\x94\xA0\xF4d\xEC\xBD\x9E\xEDAq\x13\x0E\x0F\xC0\xE9\x9F\xB4\x03\n\x8A`@Q`@Q\x80\x91\x03\x90\xA3[\x82`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFE\xE3\tf\xA2V\xB7\x1E\x14\xBC\x0E\xBF\xC9C\x15\xE2\x8E\xF4\xA9zq1\xA9\xE2\xB7\xA3\x10\xA7:\xF4Fv`@Q`@Q\x80\x91\x03\x90\xA3`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x9A` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x81Qa$\xCFW`@\x80Q`\0\x81R` \x81\x01\x90\x91R\x94Pa&-V[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a$\xE8Wa$\xE8aN\x9EV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a%\x11W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x94P`\0[\x82Q\x81\x10\x15a&+W`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91` \x80\x83\x01\x90\x806\x837PP`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x92\x93P`\0\x92\x91P` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x84\x83\x81Q\x81\x10a%wWa%waWEV[` \x02` \x01\x01Q\x82`\0\x81Q\x81\x10a%\x92Wa%\x92aWEV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x83\x83\x81Q\x81\x10a%\xC4Wa%\xC4aWEV[` \x02` \x01\x01Q\x81`\0\x81Q\x81\x10a%\xDFWa%\xDFaWEV[` \x02` \x01\x01\x81\x81RPPa%\xF8\x89\x87\x8B\x85\x85a(\xE3V[\x88\x84\x81Q\x81\x10a&\nWa&\naWEV[` \x02` \x01\x01\x81\x81RPPPP\x80\x80a&#\x90aWqV[\x91PPa%\x17V[P[PPPP\x91\x90PV[a\x11\x083\x84\x84\x84a1?V[a&K3a\x1A\x1CV[a&\xC9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FDelegationManager.modifyOperator`D\x82\x01R\x7FDetails: caller must be an opera`d\x82\x01Rb:7\xB9`\xE9\x1B`\x84\x82\x01R`\xA4\x01a\x0B\x03V[a\x10S3\x82a.\xA3V[a&\xDBa6VV[`\x01`\x01`\xA0\x1B\x03\x81\x16a'@W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x0B\x03V[a\x10S\x81a:SV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14\x15a'zWP`\x97T\x90V[a'\x82a9\xBCV[\x90P\x90V[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a'\xDAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'\xFE\x91\x90aX\xADV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a(.W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\x03\x90aX\xCAV[`fT\x19\x81\x19`fT\x19\x16\x14a(\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\x03V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x12AV[`\0`\x01`\x01`\xA0\x1B\x03\x86\x16a)zW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`P`$\x82\x01R\x7FDelegationManager._removeSharesA`D\x82\x01R\x7FndQueueWithdrawal: staker cannot`d\x82\x01Ro be zero address`\x80\x1B`\x84\x82\x01R`\xA4\x01a\x0B\x03V[\x82Qa*\x04W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`M`$\x82\x01R\x7FDelegationManager._removeSharesA`D\x82\x01R\x7FndQueueWithdrawal: strategies ca`d\x82\x01Rlnnot be empty`\x98\x1B`\x84\x82\x01R`\xA4\x01a\x0B\x03V[`\0[\x83Q\x81\x10\x15a-\xB1W`\x01`\x01`\xA0\x1B\x03\x86\x16\x15a*]Wa*]\x86\x88\x86\x84\x81Q\x81\x10a*6Wa*6aWEV[` \x02` \x01\x01Q\x86\x85\x81Q\x81\x10a*PWa*PaWEV[` \x02` \x01\x01Qa5\xDBV[s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0`\x01`\x01`\xA0\x1B\x03\x16\x84\x82\x81Q\x81\x10a*\x8DWa*\x8DaWEV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a+VW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xBE\xFF\xBB\x89\x88\x85\x84\x81Q\x81\x10a*\xE6Wa*\xE6aWEV[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a+\x1F\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a+9W`\0\x80\xFD[PZ\xF1\x15\x80\x15a+MW=`\0\x80>=`\0\xFD[PPPPa-\xA9V[\x84`\x01`\x01`\xA0\x1B\x03\x16\x87`\x01`\x01`\xA0\x1B\x03\x16\x14\x80a,(WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x9BM\xA0=\x85\x83\x81Q\x81\x10a+\xB2Wa+\xB2aWEV[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a+\xE5\x91\x90`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,\x02W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,&\x91\x90aYqV[\x15[a,\xF4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x84`$\x82\x01\x81\x90R\x7FDelegationManager._removeSharesA`D\x83\x01R\x7FndQueueWithdrawal: withdrawer mu`d\x83\x01R\x7Fst be same address as staker if \x90\x82\x01R\x7FthirdPartyTransfersForbidden are`\xA4\x82\x01Rc\x08\x1C\xD9]`\xE2\x1B`\xC4\x82\x01R`\xE4\x01a\x0B\x03V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8C\x80\xD4\xE5\x88\x86\x84\x81Q\x81\x10a-6Wa-6aWEV[` \x02` \x01\x01Q\x86\x85\x81Q\x81\x10a-PWa-PaWEV[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a-v\x93\x92\x91\x90a\\\xE0V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a-\x90W`\0\x80\xFD[PZ\xF1\x15\x80\x15a-\xA4W=`\0\x80>=`\0\xFD[PPPP[`\x01\x01a*\x07V[P`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x9F` R`@\x81 \x80T\x91\x82\x91\x90a-\xD9\x83aWqV[\x91\x90PUP`\0`@Q\x80`\xE0\x01`@R\x80\x89`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x88`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x87`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86\x81R` \x01\x85\x81RP\x90P`\0a.A\x82a\x16\xA6V[`\0\x81\x81R`\x9E` R`@\x90\x81\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x90\x91P\x7F\x90\t\xAB\x15>\x80\x14\xFB\xFB\x02\xF2!\x7F\\\xDEz\xA7\xF9\xADsJ\xE8\\\xA3\xEE?L\xA2\xFD\xD4\x99\xF9\x90a.\x8F\x90\x83\x90\x85\x90a[jV[`@Q\x80\x91\x03\x90\xA1\x98\x97PPPPPPPPV[`\0a.\xB2` \x83\x01\x83aLlV[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a/LW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`T`$\x82\x01R\x7FDelegationManager._setOperatorDe`D\x82\x01R\x7Ftails: cannot set `earningsRecei`d\x82\x01Rsver` to zero address``\x1B`\x84\x82\x01R`\xA4\x01a\x0B\x03V[b\x13\xC6\x80a/```\x83\x01`@\x84\x01a]\x04V[c\xFF\xFF\xFF\xFF\x16\x11\x15a0\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`l`$\x82\x01R\x7FDelegationManager._setOperatorDe`D\x82\x01R\x7Ftails: stakerOptOutWindowBlocks `d\x82\x01R\x7Fcannot be > MAX_STAKER_OPT_OUT_W`\x84\x82\x01RkINDOW_BLOCKS`\xA0\x1B`\xA4\x82\x01R`\xC4\x01a\x0B\x03V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x99` R`@\x90\x81\x90 `\x01\x01T`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x90a0Q\x90``\x84\x01\x90\x84\x01a]\x04V[c\xFF\xFF\xFF\xFF\x16\x10\x15a0\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`S`$\x82\x01R\x7FDelegationManager._setOperatorDe`D\x82\x01R\x7Ftails: stakerOptOutWindowBlocks `d\x82\x01Rr\x18\xD8[\x9B\x9B\xDD\x08\x18\x99H\x19\x19X\xDC\x99X\\\xD9Y`j\x1B`\x84\x82\x01R`\xA4\x01a\x0B\x03V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x99` R`@\x90 \x81\x90a1\x0B\x82\x82a]AV[PP`@Q3\x90\x7F\xFE\xBE\\\xD2K,\xBC{\x06[\x9D\x0F\xDE\xB9\x04F\x1EJ\xFC\xFFW\xDDW\xAC\xDA\x1Ex2\x03\x1B\xA7\xAC\x90a\x1C\xF5\x90\x84\x90aX,V[`fT`\0\x90`\x01\x90\x81\x16\x14\x15a1hW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\x03\x90aW\x8CV[a1q\x85a\x15\xBFV[\x15a1\xEEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FDelegationManager._delegate: sta`D\x82\x01R\x7Fker is already actively delegate`d\x82\x01R`\x19`\xFA\x1B`\x84\x82\x01R`\xA4\x01a\x0B\x03V[a1\xF7\x84a\x1A\x1CV[a2wW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R\x7FDelegationManager._delegate: ope`D\x82\x01R\x7Frator is not registered in Eigen`d\x82\x01Rd&0\xBC\xB2\xB9`\xD9\x1B`\x84\x82\x01R`\xA4\x01a\x0B\x03V[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T\x16\x80\x15\x80\x15\x90a2\xADWP3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14\x15[\x80\x15a2\xC2WP3`\x01`\x01`\xA0\x1B\x03\x86\x16\x14\x15[\x15a4/WB\x84` \x01Q\x10\x15a3AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FDelegationManager._delegate: app`D\x82\x01R\x7Frover signature expired\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\x03V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x9C` \x90\x81R`@\x80\x83 \x86\x84R\x90\x91R\x90 T`\xFF\x16\x15a3\xDBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FDelegationManager._delegate: app`D\x82\x01R\x7FroverSalt already spent\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\x03V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x9C` \x90\x81R`@\x80\x83 \x86\x84R\x82R\x82 \x80T`\xFF\x19\x16`\x01\x17\x90U\x85\x01Qa4\x1C\x90\x88\x90\x88\x90\x85\x90\x88\x90a\n\x15V[\x90Pa4-\x82\x82\x87`\0\x01QaD\x10V[P[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\0\x81\x81R`\x9A` R`@\x80\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x94\x8A\x16\x94\x85\x17\x90UQ\x7F\xC3\xEE\x9F._\xDA\x98\xE8\x06j\x1Ft[-\xF9(_Ao\xE9\x8C\xF2U\x9C\xD2\x14\x84\xB3\xD8t3\x04\x91\x90\xA3`\0\x80a4\x8E\x88a\x1D\xBAV[\x91P\x91P`\0[\x82Q\x81\x10\x15a\x13\xC7Wa4\xDC\x88\x8A\x85\x84\x81Q\x81\x10a4\xB5Wa4\xB5aWEV[` \x02` \x01\x01Q\x85\x85\x81Q\x81\x10a4\xCFWa4\xCFaWEV[` \x02` \x01\x01Qa;\xABV[`\x01\x01a4\x95V[`\x01`\x01`\xA0\x1B\x03\x81\x16a5rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x0B\x03V[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x98` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a6\x12\x90\x84\x90a\\\xC9V[\x92PP\x81\x90UP\x83`\x01`\x01`\xA0\x1B\x03\x16\x7Fi\t`\x007\xB7]{G3\xAE\xDD\x81TB\xB5\xEC\x01\x8A\x82wQ\xC82\xAA\xFFd\xEB\xA5\xD6\xD2\xDD\x84\x84\x84`@Qa\x0F\x95\x93\x92\x91\x90a\\\xE0V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1ANW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x0B\x03V[\x82\x81\x14a78W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`J`$\x82\x01R\x7FDelegationManager._setStrategyWi`D\x82\x01R\x7FthdrawalDelayBlocks: input lengt`d\x82\x01Ri\r\x04\r\xAD.m\xAC.\x8Cm`\xB3\x1B`\x84\x82\x01R`\xA4\x01a\x0B\x03V[\x82`\0[\x81\x81\x10\x15a8\xCEW`\0\x86\x86\x83\x81\x81\x10a7XWa7XaWEV[\x90P` \x02\x01` \x81\x01\x90a7m\x91\x90aLlV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\xA1` R`@\x81 T\x91\x92P\x86\x86\x85\x81\x81\x10a7\x9BWa7\x9BaWEV[\x90P` \x02\x015\x90Pb\x03K\xC0\x81\x11\x15a8_W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`s`$\x82\x01R\x7FDelegationManager._setStrategyWi`D\x82\x01R\x7FthdrawalDelayBlocks: _withdrawal`d\x82\x01R\x7FDelayBlocks cannot be > MAX_WITH`\x84\x82\x01RrDRAWAL_DELAY_BLOCKS`h\x1B`\xA4\x82\x01R`\xC4\x01a\x0B\x03V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81R`\xA1` \x90\x81R`@\x91\x82\x90 \x84\x90U\x81Q\x92\x83R\x82\x01\x84\x90R\x81\x01\x82\x90R\x7F\x0E~\xFAs\x8E\x8B\x0C\xE67j\x0C\x1A\xF4qeU@\xD2\xE9\xA8\x16G\xD7\xB0\x9E\xD8#\x01\x84&Wm\x90``\x01`@Q\x80\x91\x03\x90\xA1PPP\x80a8\xC7\x90aWqV[\x90Pa7<V[PPPPPPV[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a8\xF7WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a9yW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x0B\x03V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a\x19}\x82a4\xE4V[`@\x80Q\x80\x82\x01\x82R`\n\x81Ri\"\xB4\xB3\xB2\xB7&0\xBC\xB2\xB9`\xB1\x1B` \x91\x82\x01R\x81Q\x7F\x8C\xAD\x95h{\xA8,,\xE5\x0Et\xF7\xB7Td^Q\x17\xC3\xA5\xBE\xC8\x15\x1C\x07&\xD5\x85y\x80\xA8f\x81\x83\x01R\x7Fq\xB6%\xCF\xADD\xBA\xC6;\x13\xDB\xA0\x7F.\x1D`\x84\xEE\x04\xB6\xF8u!\x01\xEC\xE6\x12mXN\xE6\xEA\x81\x84\x01RF``\x82\x01R0`\x80\x80\x83\x01\x91\x90\x91R\x83Q\x80\x83\x03\x90\x91\x01\x81R`\xA0\x90\x91\x01\x90\x92R\x81Q\x91\x01 \x90V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[b\x03K\xC0\x81\x11\x15a;jW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`x`$\x82\x01R\x7FDelegationManager._initializeMin`D\x82\x01R\x7FWithdrawalDelayBlocks: _minWithd`d\x82\x01R\x7FrawalDelayBlocks cannot be > MAX`\x84\x82\x01R\x7F_WITHDRAWAL_DELAY_BLOCKS\0\0\0\0\0\0\0\0`\xA4\x82\x01R`\xC4\x01a\x0B\x03V[`\x9DT`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7F\xAF\xA0\x03\xCDv\xF8\x7F\xF9\xD6+5\xBE\xEA\x88\x99 \xF3<\x0CB\xB8\xD4[t\x95Ma\xD5\x0FKki\x91\x01`@Q\x80\x91\x03\x90\xA1`\x9DUV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x98` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a;\xE2\x90\x84\x90a\\\xB1V[\x92PP\x81\x90UP\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\x1E\xC0B\xC9e\xE2\xED\xD7\x10{Q\x18\x8E\xE0\xF3\x83\xE2.v\x17\x90A\xAB:\x9D\x18\xFF\x15\x14\x05\x16l\x84\x84\x84`@Qa\x0F\x95\x93\x92\x91\x90a\\\xE0V[`\0a<4a\x06\x0E\x87a]\xA4V[`\0\x81\x81R`\x9E` R`@\x90 T\x90\x91P`\xFF\x16a<\xB5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R`\0\x80Q` a^\xDC\x839\x81Q\x91R`D\x82\x01R\x7FdWithdrawal: action is not in qu`d\x82\x01Rbeue`\xE8\x1B`\x84\x82\x01R`\xA4\x01a\x0B\x03V[`\x9DTC\x90a<\xCA`\xA0\x89\x01`\x80\x8A\x01a]\x04V[c\xFF\xFF\xFF\xFF\x16a<\xDA\x91\x90a\\\xB1V[\x11\x15a=bW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`_`$\x82\x01R`\0\x80Q` a^\xDC\x839\x81Q\x91R`D\x82\x01R\x7FdWithdrawal: minWithdrawalDelayB`d\x82\x01R\x7Flocks period has not yet passed\0`\x84\x82\x01R`\xA4\x01a\x0B\x03V[a=r``\x87\x01`@\x88\x01aLlV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a=\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`P`$\x82\x01R`\0\x80Q` a^\xDC\x839\x81Q\x91R`D\x82\x01R\x7FdWithdrawal: only withdrawer can`d\x82\x01Ro\x101\xB7\xB6\xB862\xBA2\x900\xB1\xBA4\xB7\xB7`\x81\x1B`\x84\x82\x01R`\xA4\x01a\x0B\x03V[\x81\x15a>\x81Wa>\x12`\xA0\x87\x01\x87aW\xE3V[\x85\x14\x90Pa>\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`B`$\x82\x01R`\0\x80Q` a^\xDC\x839\x81Q\x91R`D\x82\x01R\x7FdWithdrawal: input length mismat`d\x82\x01Ra\x0Cm`\xF3\x1B`\x84\x82\x01R`\xA4\x01a\x0B\x03V[`\0\x81\x81R`\x9E` R`@\x90 \x80T`\xFF\x19\x16\x90U\x81\x15a?\xE6W`\0[a>\xAD`\xA0\x88\x01\x88aW\xE3V[\x90P\x81\x10\x15a?\xE0WC`\xA1`\0a>\xC8`\xA0\x8B\x01\x8BaW\xE3V[\x85\x81\x81\x10a>\xD8Wa>\xD8aWEV[\x90P` \x02\x01` \x81\x01\x90a>\xED\x91\x90aLlV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 Ta?\x17`\xA0\x8A\x01`\x80\x8B\x01a]\x04V[c\xFF\xFF\xFF\xFF\x16a?'\x91\x90a\\\xB1V[\x11\x15a?EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\x03\x90a]\xB6V[a?\xD8a?U` \x89\x01\x89aLlV[3a?c`\xA0\x8B\x01\x8BaW\xE3V[\x85\x81\x81\x10a?sWa?saWEV[\x90P` \x02\x01` \x81\x01\x90a?\x88\x91\x90aLlV[a?\x95`\xC0\x8C\x01\x8CaW\xE3V[\x86\x81\x81\x10a?\xA5Wa?\xA5aWEV[\x90P` \x02\x015\x8A\x8A\x87\x81\x81\x10a?\xBEWa?\xBEaWEV[\x90P` \x02\x01` \x81\x01\x90a?\xD3\x91\x90aLlV[aE\xCAV[`\x01\x01a>\xA0V[PaC\xD5V[3`\0\x90\x81R`\x9A` R`@\x81 T`\x01`\x01`\xA0\x1B\x03\x16\x90[a@\x0E`\xA0\x89\x01\x89aW\xE3V[\x90P\x81\x10\x15aC\xD2WC`\xA1`\0a@)`\xA0\x8C\x01\x8CaW\xE3V[\x85\x81\x81\x10a@9Wa@9aWEV[\x90P` \x02\x01` \x81\x01\x90a@N\x91\x90aLlV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 Ta@x`\xA0\x8B\x01`\x80\x8C\x01a]\x04V[c\xFF\xFF\xFF\xFF\x16a@\x88\x91\x90a\\\xB1V[\x11\x15a@\xA6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\x03\x90a]\xB6V[s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0a@\xC8`\xA0\x8A\x01\x8AaW\xE3V[\x83\x81\x81\x10a@\xD8Wa@\xD8aWEV[\x90P` \x02\x01` \x81\x01\x90a@\xED\x91\x90aLlV[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15aB=W`\0aA\x0B` \x8A\x01\x8AaLlV[\x90P`\0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\x0E\x81\x07<\x83aAL`\xC0\x8E\x01\x8EaW\xE3V[\x87\x81\x81\x10aA\\WaA\\aWEV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x87\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x94\x16`\x04\x85\x01R` \x02\x91\x90\x91\x015`$\x83\x01RP`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15aA\xB0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aA\xD4\x91\x90a[\x83V[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R`\x9A` R`@\x90 T\x91\x92P\x16\x80\x15aB5WaB5\x81\x84aB\n`\xA0\x8F\x01\x8FaW\xE3V[\x88\x81\x81\x10aB\x1AWaB\x1AaWEV[\x90P` \x02\x01` \x81\x01\x90aB/\x91\x90aLlV[\x85a;\xABV[PPPaC\xCAV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xC4b>\xA13\x89\x89\x85\x81\x81\x10aB\x7FWaB\x7FaWEV[\x90P` \x02\x01` \x81\x01\x90aB\x94\x91\x90aLlV[aB\xA1`\xA0\x8D\x01\x8DaW\xE3V[\x86\x81\x81\x10aB\xB1WaB\xB1aWEV[\x90P` \x02\x01` \x81\x01\x90aB\xC6\x91\x90aLlV[aB\xD3`\xC0\x8E\x01\x8EaW\xE3V[\x87\x81\x81\x10aB\xE3WaB\xE3aWEV[`@Q`\xE0\x88\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R`\x01`\x01`\xA0\x1B\x03\x96\x87\x16`\x04\x82\x01R\x94\x86\x16`$\x86\x01R\x92\x90\x94\x16`D\x84\x01R` \x90\x91\x02\x015`d\x82\x01R`\x84\x01\x90P`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aCCW`\0\x80\xFD[PZ\xF1\x15\x80\x15aCWW=`\0\x80>=`\0\xFD[PPPP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15aC\xCAWaC\xCA\x823aC|`\xA0\x8C\x01\x8CaW\xE3V[\x85\x81\x81\x10aC\x8CWaC\x8CaWEV[\x90P` \x02\x01` \x81\x01\x90aC\xA1\x91\x90aLlV[aC\xAE`\xC0\x8D\x01\x8DaW\xE3V[\x86\x81\x81\x10aC\xBEWaC\xBEaWEV[\x90P` \x02\x015a;\xABV[`\x01\x01a@\x01V[PP[`@Q\x81\x81R\x7F\xC9p\x98\xC2\xF6X\x80\x0BM\xF2\x90\x01R\x7Fs$\xBC\xDF\xFC\xF6\xE8u\x1Ai\x9A\xB9 \xA1\xEC\xED[\x1D\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16;\x15aE*W`@Qc\x0B\x13]?`\xE1\x1B\x80\x82R\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\x16&\xBA~\x90aDP\x90\x86\x90\x86\x90`\x04\x01a^>V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aDmW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aD\x91\x91\x90a^\x9BV[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14a\x11\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`S`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: ERC1271 signature `d\x82\x01Rr\x1D\x99\\\x9AY\x9AX\xD8]\x1A[\xDB\x88\x19\x98Z[\x19Y`j\x1B`\x84\x82\x01R`\xA4\x01a\x0B\x03V[\x82`\x01`\x01`\xA0\x1B\x03\x16aE>\x83\x83aG\nV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x11\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: signature not from`d\x82\x01Rf\x109\xB4\xB3\xB72\xB9`\xC9\x1B`\x84\x82\x01R`\xA4\x01a\x0B\x03V[`\x01`\x01`\xA0\x1B\x03\x83\x16s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x14\x15aFuW`@Qb8{\x13`\xE8\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c8{\x13\0\x90aF>\x90\x88\x90\x88\x90\x87\x90`\x04\x01a\\\xE0V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aFXW`\0\x80\xFD[PZ\xF1\x15\x80\x15aFlW=`\0\x80>=`\0\xFD[PPPPaG\x03V[`@Qc\xC6\x08\xC7\xF3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x84\x81\x16`$\x83\x01R`D\x82\x01\x84\x90R\x82\x81\x16`d\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xC6\x08\xC7\xF3\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aF\xEFW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x13\xC7W=`\0\x80>=`\0\xFD[PPPPPV[`\0\x80`\0aG\x19\x85\x85aG&V[\x91P\x91Pa\n\r\x81aG\x96V[`\0\x80\x82Q`A\x14\x15aG]W` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1AaGQ\x87\x82\x85\x85aIQV[\x94P\x94PPPPaG\x8FV[\x82Q`@\x14\x15aG\x87W` \x83\x01Q`@\x84\x01QaG|\x86\x83\x83aJ>V[\x93P\x93PPPaG\x8FV[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0\x81`\x04\x81\x11\x15aG\xAAWaG\xAAa^\xC5V[\x14\x15aG\xB3WPV[`\x01\x81`\x04\x81\x11\x15aG\xC7WaG\xC7a^\xC5V[\x14\x15aH\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x0B\x03V[`\x02\x81`\x04\x81\x11\x15aH)WaH)a^\xC5V[\x14\x15aHwW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x0B\x03V[`\x03\x81`\x04\x81\x11\x15aH\x8BWaH\x8Ba^\xC5V[\x14\x15aH\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x0B\x03V[`\x04\x81`\x04\x81\x11\x15aH\xF8WaH\xF8a^\xC5V[\x14\x15a\x10SW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 'v' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x0B\x03V[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15aI\x88WP`\0\x90P`\x03aJ5V[\x84`\xFF\x16`\x1B\x14\x15\x80\x15aI\xA0WP\x84`\xFF\x16`\x1C\x14\x15[\x15aI\xB1WP`\0\x90P`\x04aJ5V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15aJ\x05W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16aJ.W`\0`\x01\x92P\x92PPaJ5V[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x80`\x01`\x01`\xFF\x1B\x03\x83\x16\x81aJ[`\xFF\x86\x90\x1C`\x1Ba\\\xB1V[\x90PaJi\x87\x82\x88\x85aIQV[\x93P\x93PPP\x93P\x93\x91PPV[`\0\x80\x83`\x1F\x84\x01\x12aJ\x89W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aJ\xA0W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15aG\x8FW`\0\x80\xFD[`\0\x80` \x83\x85\x03\x12\x15aJ\xCEW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aJ\xE4W`\0\x80\xFD[aJ\xF0\x85\x82\x86\x01aJwV[\x90\x96\x90\x95P\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x10SW`\0\x80\xFD[\x805aK\x1C\x81aJ\xFCV[\x91\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15aK9W`\0\x80\xFD[\x855aKD\x81aJ\xFCV[\x94P` \x86\x015aKT\x81aJ\xFCV[\x93P`@\x86\x015aKd\x81aJ\xFCV[\x94\x97\x93\x96P\x93\x94``\x81\x015\x94P`\x80\x015\x92\x91PPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aK\xB4W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aK\x98V[P\x90\x96\x95PPPPPPV[`\0``\x82\x84\x03\x12\x15aK\xD2W`\0\x80\xFD[P\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12aK\xEAW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aL\x01W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aG\x8FW`\0\x80\xFD[`\0\x80`\0`\x80\x84\x86\x03\x12\x15aL.W`\0\x80\xFD[aL8\x85\x85aK\xC0V[\x92P``\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aLSW`\0\x80\xFD[aL_\x86\x82\x87\x01aK\xD8V[\x94\x97\x90\x96P\x93\x94PPPPV[`\0` \x82\x84\x03\x12\x15aL~W`\0\x80\xFD[\x815aL\x89\x81aJ\xFCV[\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15aL\xA5W`\0\x80\xFD[\x835aL\xB0\x81aJ\xFCV[\x92P` \x84\x015aL\xC0\x81aJ\xFCV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0` \x82\x84\x03\x12\x15aL\xE3W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15aM\0W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aM\x17W`\0\x80\xFD[aM#\x88\x83\x89\x01aJwV[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15aM<W`\0\x80\xFD[PaMI\x87\x82\x88\x01aJwV[\x95\x98\x94\x97P\x95PPPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\xC0\x89\x8B\x03\x12\x15aMqW`\0\x80\xFD[\x885aM|\x81aJ\xFCV[\x97P` \x89\x015aM\x8C\x81aJ\xFCV[\x96P`@\x89\x015\x95P``\x89\x015\x94P`\x80\x89\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aM\xB6W`\0\x80\xFD[aM\xC2\x8C\x83\x8D\x01aJwV[\x90\x96P\x94P`\xA0\x8B\x015\x91P\x80\x82\x11\x15aM\xDBW`\0\x80\xFD[PaM\xE8\x8B\x82\x8C\x01aJwV[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\x80\x89\x8B\x03\x12\x15aN\x18W`\0\x80\xFD[\x885`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aN/W`\0\x80\xFD[aN;\x8C\x83\x8D\x01aJwV[\x90\x9AP\x98P` \x8B\x015\x91P\x80\x82\x11\x15aNTW`\0\x80\xFD[aN`\x8C\x83\x8D\x01aJwV[\x90\x98P\x96P`@\x8B\x015\x91P\x80\x82\x11\x15aNyW`\0\x80\xFD[aN\x85\x8C\x83\x8D\x01aJwV[\x90\x96P\x94P``\x8B\x015\x91P\x80\x82\x11\x15aM\xDBW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aN\xD6WaN\xD6aN\x9EV[`@R\x90V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aN\xD6WaN\xD6aN\x9EV[`@Q`\xC0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aN\xD6WaN\xD6aN\x9EV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aOHWaOHaN\x9EV[`@R\x91\x90PV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x10SW`\0\x80\xFD[\x805aK\x1C\x81aOPV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15aO\x86WaO\x86aN\x9EV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12aO\xA1W`\0\x80\xFD[\x815` aO\xB6aO\xB1\x83aOmV[aO V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aO\xD5W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aO\xF9W\x805aO\xEC\x81aJ\xFCV[\x83R\x91\x83\x01\x91\x83\x01aO\xD9V[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12aP\x15W`\0\x80\xFD[\x815` aP%aO\xB1\x83aOmV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aPDW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aO\xF9W\x805\x83R\x91\x83\x01\x91\x83\x01aPHV[`\0`\xE0\x82\x84\x03\x12\x15aPqW`\0\x80\xFD[aPyaN\xB4V[\x90PaP\x84\x82aK\x11V[\x81RaP\x92` \x83\x01aK\x11V[` \x82\x01RaP\xA3`@\x83\x01aK\x11V[`@\x82\x01R``\x82\x015``\x82\x01RaP\xBE`\x80\x83\x01aObV[`\x80\x82\x01R`\xA0\x82\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aP\xDDW`\0\x80\xFD[aP\xE9\x85\x83\x86\x01aO\x90V[`\xA0\x84\x01R`\xC0\x84\x015\x91P\x80\x82\x11\x15aQ\x02W`\0\x80\xFD[PaQ\x0F\x84\x82\x85\x01aP\x04V[`\xC0\x83\x01RP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15aQ-W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aQCW`\0\x80\xFD[aQO\x84\x82\x85\x01aP_V[\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15aQiW`\0\x80\xFD[\x815`\xFF\x81\x16\x81\x14aL\x89W`\0\x80\xFD[`\0`@\x82\x84\x03\x12\x15aQ\x8CW`\0\x80\xFD[aQ\x94aN\xDCV[\x90P\x815aQ\xA1\x81aJ\xFCV[\x81R` \x82\x015k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14aQ\xC2W`\0\x80\xFD[` \x82\x01R\x92\x91PPV[`\0` \x80\x83\x85\x03\x12\x15aQ\xE0W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aQ\xF7W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12aR\x0BW`\0\x80\xFD[\x815aR\x19aO\xB1\x82aOmV[\x81\x81R`\x05\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x84\x81\x01\x90\x88\x83\x11\x15aR8W`\0\x80\xFD[\x85\x85\x01[\x83\x81\x10\x15aS\x12W\x805\x85\x81\x11\x15aRTW`\0\x80\x81\xFD[\x86\x01`\xE0\x81\x8C\x03`\x1F\x19\x01\x81\x13\x15aRlW`\0\x80\x81\xFD[aRtaN\xFEV[\x89\x83\x015\x88\x81\x11\x15aR\x86W`\0\x80\x81\xFD[aR\x94\x8E\x8C\x83\x87\x01\x01aO\x90V[\x82RP`@\x80\x84\x015\x89\x81\x11\x15aR\xABW`\0\x80\x81\xFD[aR\xB9\x8F\x8D\x83\x88\x01\x01aP\x04V[\x8C\x84\x01RP``aR\xCB\x81\x86\x01aK\x11V[\x82\x84\x01R`\x80\x91PaR\xDF\x8F\x83\x87\x01aQzV[\x90\x83\x01RaR\xEF`\xC0\x85\x01aObV[\x90\x82\x01RaR\xFE\x83\x83\x01aK\x11V[`\xA0\x82\x01R\x85RPP\x91\x86\x01\x91\x86\x01aR<V[P\x98\x97PPPPPPPPV[\x80\x15\x15\x81\x14a\x10SW`\0\x80\xFD[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15aSEW`\0\x80\xFD[\x855`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aS\\W`\0\x80\xFD[\x90\x87\x01\x90`\xE0\x82\x8A\x03\x12\x15aSpW`\0\x80\xFD[\x90\x95P` \x87\x015\x90\x80\x82\x11\x15aS\x86W`\0\x80\xFD[PaS\x93\x88\x82\x89\x01aJwV[\x90\x95P\x93PP`@\x86\x015\x91P``\x86\x015aS\xAE\x81aS\x1FV[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x80`@\x83\x85\x03\x12\x15aS\xCFW`\0\x80\xFD[\x825aS\xDA\x81aJ\xFCV[\x91P` \x83\x015aS\xEA\x81aJ\xFCV[\x80\x91PP\x92P\x92\x90PV[`\0`@\x82\x84\x03\x12\x15aT\x07W`\0\x80\xFD[aT\x0FaN\xDCV[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aT(W`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12aT<W`\0\x80\xFD[\x815` \x82\x82\x11\x15aTPWaTPaN\x9EV[aTb`\x1F\x83\x01`\x1F\x19\x16\x82\x01aO V[\x92P\x81\x83R\x86\x81\x83\x86\x01\x01\x11\x15aTxW`\0\x80\xFD[\x81\x81\x85\x01\x82\x85\x017`\0\x81\x83\x85\x01\x01R\x82\x85R\x80\x86\x015\x81\x86\x01RPPPP\x92\x91PPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15aT\xB5W`\0\x80\xFD[\x855aT\xC0\x81aJ\xFCV[\x94P` \x86\x015aT\xD0\x81aJ\xFCV[\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aT\xECW`\0\x80\xFD[aT\xF8\x89\x83\x8A\x01aS\xF5V[\x94P``\x88\x015\x91P\x80\x82\x11\x15aU\x0EW`\0\x80\xFD[PaU\x1B\x88\x82\x89\x01aS\xF5V[\x95\x98\x94\x97P\x92\x95`\x80\x015\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15aU@W`\0\x80\xFD[\x825aUK\x81aJ\xFCV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aUfW`\0\x80\xFD[aUr\x85\x82\x86\x01aO\x90V[\x91PP\x92P\x92\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aU\xACW\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aU\x90V[P\x94\x95\x94PPPPPV[` \x81R`\0aL\x89` \x83\x01\x84aU|V[`\0\x80` \x83\x85\x03\x12\x15aU\xDDW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aU\xF3W`\0\x80\xFD[aJ\xF0\x85\x82\x86\x01aK\xD8V[`\0\x80`@\x83\x85\x03\x12\x15aV\x12W`\0\x80\xFD[\x825aV\x1D\x81aJ\xFCV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aVAW`\0\x80\xFD[\x845aVL\x81aJ\xFCV[\x93P` \x85\x015\x92P`@\x85\x015aVc\x81aJ\xFCV[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aU\xACW\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aV\x87V[`@\x81R`\0aV\xBF`@\x83\x01\x85aVsV[\x82\x81\x03` \x84\x01Ra\x12\x85\x81\x85aU|V[`\0\x80`\0``\x84\x86\x03\x12\x15aV\xE6W`\0\x80\xFD[\x835aV\xF1\x81aJ\xFCV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aW\x0CW`\0\x80\xFD[aW\x18\x86\x82\x87\x01aS\xF5V[\x92PP`@\x84\x015\x90P\x92P\x92P\x92V[`\0``\x82\x84\x03\x12\x15aW;W`\0\x80\xFD[aL\x89\x83\x83aK\xC0V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15aW\x85WaW\x85aW[V[P`\x01\x01\x90V[` \x80\x82R`\x19\x90\x82\x01R\x7FPausable: index is paused\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`\0\x825`^\x19\x836\x03\x01\x81\x12aW\xD9W`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aW\xFAW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aX\x14W`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15aG\x8FW`\0\x80\xFD[``\x81\x01\x825aX;\x81aJ\xFCV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x84\x015\x90aXW\x82aJ\xFCV[\x16` \x83\x01R`@\x83\x015aXk\x81aOPV[c\xFF\xFF\xFF\xFF\x81\x16`@\x84\x01RP\x92\x91PPV[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017`\0\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x91\x90PV[`\0` \x82\x84\x03\x12\x15aX\xBFW`\0\x80\xFD[\x81QaL\x89\x81aJ\xFCV[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`7\x90\x82\x01R\x7FDelegationManager: onlyStrategyM`@\x82\x01R\x7FanagerOrEigenPodManager\0\0\0\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15aY\x83W`\0\x80\xFD[\x81QaL\x89\x81aS\x1FV[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[`\0\x825`\xDE\x19\x836\x03\x01\x81\x12aW\xD9W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aY\xFEW`\0\x80\xFD[\x815aL\x89\x81aS\x1FV[`\0`\x01\x80`\xA0\x1B\x03\x80\x83Q\x16\x84R\x80` \x84\x01Q\x16` \x85\x01R\x80`@\x84\x01Q\x16`@\x85\x01RP``\x82\x01Q``\x84\x01Rc\xFF\xFF\xFF\xFF`\x80\x83\x01Q\x16`\x80\x84\x01R`\xA0\x82\x01Q`\xE0`\xA0\x85\x01RaZd`\xE0\x85\x01\x82aVsV[\x90P`\xC0\x83\x01Q\x84\x82\x03`\xC0\x86\x01Ra\x12\x85\x82\x82aU|V[` \x81R`\0aL\x89` \x83\x01\x84aZ\tV[` \x81R`\0\x82Q`\xE0` \x84\x01RaZ\xADa\x01\0\x84\x01\x82aVsV[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01RaZ\xCA\x82\x82aU|V[\x91PP`@\x84\x01Q`\x01\x80`\xA0\x1B\x03\x80\x82\x16``\x86\x01R``\x86\x01Q\x91P\x80\x82Q\x16`\x80\x86\x01RPk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16`\xA0\x85\x01RP`\x80\x84\x01Qa[\"`\xC0\x85\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P`\xA0\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16`\xE0\x85\x01Ra\n\rV[`\0\x80`@\x83\x85\x03\x12\x15a[OW`\0\x80\xFD[\x82Qa[Z\x81aS\x1FV[` \x93\x90\x93\x01Q\x92\x94\x92\x93PPPV[\x82\x81R`@` \x82\x01R`\0aQO`@\x83\x01\x84aZ\tV[`\0` \x82\x84\x03\x12\x15a[\x95W`\0\x80\xFD[PQ\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a[\xADW`\0\x80\xFD[\x81Q` a[\xBDaO\xB1\x83aOmV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a[\xDCW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aO\xF9W\x80Q\x83R\x91\x83\x01\x91\x83\x01a[\xE0V[`\0\x80`@\x83\x85\x03\x12\x15a\\\nW`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\\!W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\\5W`\0\x80\xFD[\x81Q` a\\EaO\xB1\x83aOmV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a\\dW`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a\\\x8BW\x85Qa\\|\x81aJ\xFCV[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a\\iV[\x91\x88\x01Q\x91\x96P\x90\x93PPP\x80\x82\x11\x15a\\\xA4W`\0\x80\xFD[PaUr\x85\x82\x86\x01a[\x9CV[`\0\x82\x19\x82\x11\x15a\\\xC4Wa\\\xC4aW[V[P\x01\x90V[`\0\x82\x82\x10\x15a\\\xDBWa\\\xDBaW[V[P\x03\x90V[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[`\0` \x82\x84\x03\x12\x15a]\x16W`\0\x80\xFD[\x815aL\x89\x81aOPV[\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\x815a]L\x81aJ\xFCV[a]V\x81\x83a]!V[P`\x01\x81\x01` \x83\x015a]i\x81aJ\xFCV[a]s\x81\x83a]!V[P`@\x83\x015a]\x82\x81aOPV[\x81Tc\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x16`\xA0\x91\x90\x91\x1Bc\xFF\xFF\xFF\xFF`\xA0\x1B\x16\x17\x90UPPV[`\0a]\xB06\x83aP_V[\x92\x91PPV[` \x80\x82R`n\x90\x82\x01R`\0\x80Q` a^\xDC\x839\x81Q\x91R`@\x82\x01R\x7FdWithdrawal: withdrawalDelayBloc``\x82\x01R\x7Fks period has not yet passed for`\x80\x82\x01Rm this strategy`\x90\x1B`\xA0\x82\x01R`\xC0\x01\x90V[\x82\x81R`\0` `@\x81\x84\x01R\x83Q\x80`@\x85\x01R`\0[\x81\x81\x10\x15a^rW\x85\x81\x01\x83\x01Q\x85\x82\x01``\x01R\x82\x01a^VV[\x81\x81\x11\x15a^\x84W`\0``\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01``\x01\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a^\xADW`\0\x80\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14aL\x89W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD\xFEDelegationManager._completeQueue\xA2dipfsX\"\x12 \x12Q\x8D\x84A\xBC\x8F\x89pd\xC3\xB0XM\xEDW\x8EN\x1F %\xDE\xFF\xE4@\xBB$\xB0i@*\xC7dsolcC\0\x08\x0C\x003";
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
        ///Calls the contract's `STAKER_DELEGATION_TYPEHASH` (0x43377382) function
        pub fn staker_delegation_typehash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([67, 55, 115, 130], ())
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
        ///Calls the contract's `getOperatorShares` (0x90041347) function
        pub fn get_operator_shares(
            &self,
            operator: ::ethers::core::types::Address,
            strategies: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([144, 4, 19, 71], (operator, strategies))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getWithdrawalDelay` (0x0449ca39) function
        pub fn get_withdrawal_delay(
            &self,
            strategies: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([4, 73, 202, 57], strategies)
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
        ///Calls the contract's `initialize` (0x22bf40e4) function
        pub fn initialize(
            &self,
            initial_owner: ::ethers::core::types::Address,
            pauser_registry: ::ethers::core::types::Address,
            initial_paused_status: ::ethers::core::types::U256,
            min_withdrawal_delay_blocks: ::ethers::core::types::U256,
            strategies: ::std::vec::Vec<::ethers::core::types::Address>,
            withdrawal_delay_blocks: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [34, 191, 64, 228],
                    (
                        initial_owner,
                        pauser_registry,
                        initial_paused_status,
                        min_withdrawal_delay_blocks,
                        strategies,
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
        ///Calls the contract's `minWithdrawalDelayBlocks` (0xc448feb8) function
        pub fn min_withdrawal_delay_blocks(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([196, 72, 254, 184], ())
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
        ///Calls the contract's `setStrategyWithdrawalDelayBlocks` (0x1522bf02) function
        pub fn set_strategy_withdrawal_delay_blocks(
            &self,
            strategies: ::std::vec::Vec<::ethers::core::types::Address>,
            withdrawal_delay_blocks: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([21, 34, 191, 2], (strategies, withdrawal_delay_blocks))
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
        ///Calls the contract's `strategyWithdrawalDelayBlocks` (0xc488375a) function
        pub fn strategy_withdrawal_delay_blocks(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([196, 136, 55, 90], p0)
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
        ///Calls the contract's `updateOperatorMetadataURI` (0x99be81c8) function
        pub fn update_operator_metadata_uri(
            &self,
            metadata_uri: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([153, 190, 129, 200], metadata_uri)
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
        ///Gets the contract's `MinWithdrawalDelayBlocksSet` event
        pub fn min_withdrawal_delay_blocks_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MinWithdrawalDelayBlocksSetFilter,
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
        ///Gets the contract's `StrategyWithdrawalDelayBlocksSet` event
        pub fn strategy_withdrawal_delay_blocks_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StrategyWithdrawalDelayBlocksSetFilter,
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
        name = "MinWithdrawalDelayBlocksSet",
        abi = "MinWithdrawalDelayBlocksSet(uint256,uint256)"
    )]
    pub struct MinWithdrawalDelayBlocksSetFilter {
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
    #[ethevent(
        name = "StrategyWithdrawalDelayBlocksSet",
        abi = "StrategyWithdrawalDelayBlocksSet(address,uint256,uint256)"
    )]
    pub struct StrategyWithdrawalDelayBlocksSetFilter {
        pub strategy: ::ethers::core::types::Address,
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
        InitializedFilter(InitializedFilter),
        MinWithdrawalDelayBlocksSetFilter(MinWithdrawalDelayBlocksSetFilter),
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
        StrategyWithdrawalDelayBlocksSetFilter(StrategyWithdrawalDelayBlocksSetFilter),
        UnpausedFilter(UnpausedFilter),
        WithdrawalCompletedFilter(WithdrawalCompletedFilter),
        WithdrawalMigratedFilter(WithdrawalMigratedFilter),
        WithdrawalQueuedFilter(WithdrawalQueuedFilter),
    }
    impl ::ethers::contract::EthLogDecode for DelegationManagerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(DelegationManagerEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = MinWithdrawalDelayBlocksSetFilter::decode_log(log) {
                return Ok(
                    DelegationManagerEvents::MinWithdrawalDelayBlocksSetFilter(decoded),
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
            if let Ok(decoded) = StrategyWithdrawalDelayBlocksSetFilter::decode_log(
                log,
            ) {
                return Ok(
                    DelegationManagerEvents::StrategyWithdrawalDelayBlocksSetFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(DelegationManagerEvents::UnpausedFilter(decoded));
            }
            if let Ok(decoded) = WithdrawalCompletedFilter::decode_log(log) {
                return Ok(DelegationManagerEvents::WithdrawalCompletedFilter(decoded));
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
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinWithdrawalDelayBlocksSetFilter(element) => {
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
                Self::StrategyWithdrawalDelayBlocksSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnpausedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawalCompletedFilter(element) => {
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
    impl ::core::convert::From<InitializedFilter> for DelegationManagerEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<MinWithdrawalDelayBlocksSetFilter>
    for DelegationManagerEvents {
        fn from(value: MinWithdrawalDelayBlocksSetFilter) -> Self {
            Self::MinWithdrawalDelayBlocksSetFilter(value)
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
    impl ::core::convert::From<StrategyWithdrawalDelayBlocksSetFilter>
    for DelegationManagerEvents {
        fn from(value: StrategyWithdrawalDelayBlocksSetFilter) -> Self {
            Self::StrategyWithdrawalDelayBlocksSetFilter(value)
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
    ///Container type for all input parameters for the `getOperatorShares` function with signature `getOperatorShares(address,address[])` and selector `0x90041347`
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
    #[ethcall(name = "getOperatorShares", abi = "getOperatorShares(address,address[])")]
    pub struct GetOperatorSharesCall {
        pub operator: ::ethers::core::types::Address,
        pub strategies: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `getWithdrawalDelay` function with signature `getWithdrawalDelay(address[])` and selector `0x0449ca39`
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
    #[ethcall(name = "getWithdrawalDelay", abi = "getWithdrawalDelay(address[])")]
    pub struct GetWithdrawalDelayCall {
        pub strategies: ::std::vec::Vec<::ethers::core::types::Address>,
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
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address,uint256,uint256,address[],uint256[])` and selector `0x22bf40e4`
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
        name = "initialize",
        abi = "initialize(address,address,uint256,uint256,address[],uint256[])"
    )]
    pub struct InitializeCall {
        pub initial_owner: ::ethers::core::types::Address,
        pub pauser_registry: ::ethers::core::types::Address,
        pub initial_paused_status: ::ethers::core::types::U256,
        pub min_withdrawal_delay_blocks: ::ethers::core::types::U256,
        pub strategies: ::std::vec::Vec<::ethers::core::types::Address>,
        pub withdrawal_delay_blocks: ::std::vec::Vec<::ethers::core::types::U256>,
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
    ///Container type for all input parameters for the `minWithdrawalDelayBlocks` function with signature `minWithdrawalDelayBlocks()` and selector `0xc448feb8`
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
    #[ethcall(name = "minWithdrawalDelayBlocks", abi = "minWithdrawalDelayBlocks()")]
    pub struct MinWithdrawalDelayBlocksCall;
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
    ///Container type for all input parameters for the `setStrategyWithdrawalDelayBlocks` function with signature `setStrategyWithdrawalDelayBlocks(address[],uint256[])` and selector `0x1522bf02`
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
        name = "setStrategyWithdrawalDelayBlocks",
        abi = "setStrategyWithdrawalDelayBlocks(address[],uint256[])"
    )]
    pub struct SetStrategyWithdrawalDelayBlocksCall {
        pub strategies: ::std::vec::Vec<::ethers::core::types::Address>,
        pub withdrawal_delay_blocks: ::std::vec::Vec<::ethers::core::types::U256>,
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
    ///Container type for all input parameters for the `strategyWithdrawalDelayBlocks` function with signature `strategyWithdrawalDelayBlocks(address)` and selector `0xc488375a`
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
        name = "strategyWithdrawalDelayBlocks",
        abi = "strategyWithdrawalDelayBlocks(address)"
    )]
    pub struct StrategyWithdrawalDelayBlocksCall(pub ::ethers::core::types::Address);
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
        StakerDelegationTypehash(StakerDelegationTypehashCall),
        BeaconChainETHStrategy(BeaconChainETHStrategyCall),
        CalculateCurrentStakerDelegationDigestHash(
            CalculateCurrentStakerDelegationDigestHashCall,
        ),
        CalculateDelegationApprovalDigestHash(CalculateDelegationApprovalDigestHashCall),
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
        DomainSeparator(DomainSeparatorCall),
        EarningsReceiver(EarningsReceiverCall),
        EigenPodManager(EigenPodManagerCall),
        GetDelegatableShares(GetDelegatableSharesCall),
        GetOperatorShares(GetOperatorSharesCall),
        GetWithdrawalDelay(GetWithdrawalDelayCall),
        IncreaseDelegatedShares(IncreaseDelegatedSharesCall),
        Initialize(InitializeCall),
        IsDelegated(IsDelegatedCall),
        IsOperator(IsOperatorCall),
        MigrateQueuedWithdrawals(MigrateQueuedWithdrawalsCall),
        MinWithdrawalDelayBlocks(MinWithdrawalDelayBlocksCall),
        ModifyOperatorDetails(ModifyOperatorDetailsCall),
        OperatorDetails(OperatorDetailsCall),
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
        RenounceOwnership(RenounceOwnershipCall),
        SetPauserRegistry(SetPauserRegistryCall),
        SetStrategyWithdrawalDelayBlocks(SetStrategyWithdrawalDelayBlocksCall),
        Slasher(SlasherCall),
        StakerNonce(StakerNonceCall),
        StakerOptOutWindowBlocks(StakerOptOutWindowBlocksCall),
        StrategyManager(StrategyManagerCall),
        StrategyWithdrawalDelayBlocks(StrategyWithdrawalDelayBlocksCall),
        TransferOwnership(TransferOwnershipCall),
        Undelegate(UndelegateCall),
        Unpause(UnpauseCall),
        UpdateOperatorMetadataURI(UpdateOperatorMetadataURICall),
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
            if let Ok(decoded) = <StakerDelegationTypehashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StakerDelegationTypehash(decoded));
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
            if let Ok(decoded) = <GetOperatorSharesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetOperatorShares(decoded));
            }
            if let Ok(decoded) = <GetWithdrawalDelayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetWithdrawalDelay(decoded));
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
            if let Ok(decoded) = <MinWithdrawalDelayBlocksCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MinWithdrawalDelayBlocks(decoded));
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
            if let Ok(decoded) = <SetStrategyWithdrawalDelayBlocksCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetStrategyWithdrawalDelayBlocks(decoded));
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
            if let Ok(decoded) = <StrategyWithdrawalDelayBlocksCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StrategyWithdrawalDelayBlocks(decoded));
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
            if let Ok(decoded) = <UpdateOperatorMetadataURICall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateOperatorMetadataURI(decoded));
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
                Self::StakerDelegationTypehash(element) => {
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
                Self::GetOperatorShares(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetWithdrawalDelay(element) => {
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
                Self::MinWithdrawalDelayBlocks(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ModifyOperatorDetails(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OperatorDetails(element) => {
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
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPauserRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetStrategyWithdrawalDelayBlocks(element) => {
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
                Self::StrategyWithdrawalDelayBlocks(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Undelegate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unpause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateOperatorMetadataURI(element) => {
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
                Self::StakerDelegationTypehash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BeaconChainETHStrategy(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateCurrentStakerDelegationDigestHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateDelegationApprovalDigestHash(element) => {
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
                Self::DomainSeparator(element) => ::core::fmt::Display::fmt(element, f),
                Self::EarningsReceiver(element) => ::core::fmt::Display::fmt(element, f),
                Self::EigenPodManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDelegatableShares(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetOperatorShares(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetWithdrawalDelay(element) => {
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
                Self::MinWithdrawalDelayBlocks(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ModifyOperatorDetails(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorDetails(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPauserRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetStrategyWithdrawalDelayBlocks(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Slasher(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakerNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakerOptOutWindowBlocks(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StrategyManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::StrategyWithdrawalDelayBlocks(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::Undelegate(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unpause(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateOperatorMetadataURI(element) => {
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
    impl ::core::convert::From<StakerDelegationTypehashCall> for DelegationManagerCalls {
        fn from(value: StakerDelegationTypehashCall) -> Self {
            Self::StakerDelegationTypehash(value)
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
    impl ::core::convert::From<GetOperatorSharesCall> for DelegationManagerCalls {
        fn from(value: GetOperatorSharesCall) -> Self {
            Self::GetOperatorShares(value)
        }
    }
    impl ::core::convert::From<GetWithdrawalDelayCall> for DelegationManagerCalls {
        fn from(value: GetWithdrawalDelayCall) -> Self {
            Self::GetWithdrawalDelay(value)
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
    impl ::core::convert::From<MinWithdrawalDelayBlocksCall> for DelegationManagerCalls {
        fn from(value: MinWithdrawalDelayBlocksCall) -> Self {
            Self::MinWithdrawalDelayBlocks(value)
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
    impl ::core::convert::From<SetStrategyWithdrawalDelayBlocksCall>
    for DelegationManagerCalls {
        fn from(value: SetStrategyWithdrawalDelayBlocksCall) -> Self {
            Self::SetStrategyWithdrawalDelayBlocks(value)
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
    impl ::core::convert::From<StrategyWithdrawalDelayBlocksCall>
    for DelegationManagerCalls {
        fn from(value: StrategyWithdrawalDelayBlocksCall) -> Self {
            Self::StrategyWithdrawalDelayBlocks(value)
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
    impl ::core::convert::From<UpdateOperatorMetadataURICall>
    for DelegationManagerCalls {
        fn from(value: UpdateOperatorMetadataURICall) -> Self {
            Self::UpdateOperatorMetadataURI(value)
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
    ///Container type for all return fields from the `getOperatorShares` function with signature `getOperatorShares(address,address[])` and selector `0x90041347`
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
    pub struct GetOperatorSharesReturn(pub ::std::vec::Vec<::ethers::core::types::U256>);
    ///Container type for all return fields from the `getWithdrawalDelay` function with signature `getWithdrawalDelay(address[])` and selector `0x0449ca39`
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
    pub struct GetWithdrawalDelayReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `minWithdrawalDelayBlocks` function with signature `minWithdrawalDelayBlocks()` and selector `0xc448feb8`
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
    pub struct MinWithdrawalDelayBlocksReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `strategyWithdrawalDelayBlocks` function with signature `strategyWithdrawalDelayBlocks(address)` and selector `0xc488375a`
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
    pub struct StrategyWithdrawalDelayBlocksReturn(pub ::ethers::core::types::U256);
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
}
