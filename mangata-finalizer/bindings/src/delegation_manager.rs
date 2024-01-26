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
                    ::std::borrow::ToOwned::to_owned("setStakeRegistry"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setStakeRegistry"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_stakeRegistry"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IStakeRegistryStub",
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
                    ::std::borrow::ToOwned::to_owned("setWithdrawalDelayBlocks"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setWithdrawalDelayBlocks",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newWithdrawalDelayBlocks",
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
                    ::std::borrow::ToOwned::to_owned("stakeRegistry"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("stakeRegistry"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IStakeRegistryStub",
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
                    ::std::borrow::ToOwned::to_owned("StakeRegistrySet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("StakeRegistrySet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("stakeRegistry"),
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
    const __BYTECODE: &[u8] = b"a\x01\0`@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@Qb\0Y\x9F8\x03\x80b\0Y\x9F\x839\x81\x01`@\x81\x90Rb\0\x005\x91b\0\x01@V[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\x80R\x80\x82\x16`\xC0R\x82\x16`\xA0Rb\0\0Xb\0\0eV[PPF`\xE0RPb\0\x01\x94V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15b\0\0\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\xFF\x90\x81\x16\x10\x15b\0\x01%W`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01=W`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15b\0\x01VW`\0\x80\xFD[\x83Qb\0\x01c\x81b\0\x01'V[` \x85\x01Q\x90\x93Pb\0\x01v\x81b\0\x01'V[`@\x85\x01Q\x90\x92Pb\0\x01\x89\x81b\0\x01'V[\x80\x91PP\x92P\x92P\x92V[`\x80Q`\xA0Q`\xC0Q`\xE0QaWvb\0\x02)`\09`\0a&l\x01R`\0\x81\x81a\x05\x8B\x01R\x81\x81a\x0F\xC3\x01R\x81\x81a\x13\x19\x01R\x81\x81a\x1C\xE8\x01R\x81\x81a)\xC5\x01R\x81\x81a:\xC0\x01Ra?\xD0\x01R`\0a\x07w\x01R`\0\x81\x81a\x04\xD3\x01R\x81\x81a\x0F\x91\x01R\x81\x81a\x12\xE7\x01R\x81\x81a\x16\xE6\x01R\x81\x81a\x1D|\x01R\x81\x81a*w\x01R\x81\x81a;\xF7\x01Ra@v\x01RaWv`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x03BW`\x005`\xE0\x1C\x80c_\x96o\x14\x11a\x01\xB8W\x80c\xB14Bq\x11a\x01\x04W\x80c\xDA\x8B\xE8d\x11a\0\xA2W\x80c\xF1ar\xB0\x11a\0|W\x80c\xF1ar\xB0\x14a\t\x06W\x80c\xF2\xFD\xE3\x8B\x14a\t\x19W\x80c\xF6\x98\xDA%\x14a\t,W\x80c\xFA\xBC\x1C\xBC\x14a\t4W`\0\x80\xFD[\x80c\xDA\x8B\xE8d\x14a\x08\xCDW\x80c\xE3\xB0_/\x14a\x08\xE0W\x80c\xEE\xA9\x06K\x14a\x08\xF3W`\0\x80\xFD[\x80c\xC5\xE4\x80\xDB\x11a\0\xDEW\x80c\xC5\xE4\x80\xDB\x14a\x07\xEAW\x80c\xC9KQ\x11\x14a\x08\x90W\x80c\xCAf\x1C\x04\x14a\x08\xA3W\x80c\xCF\x80\x87>\x14a\x08\xACW`\0\x80\xFD[\x80c\xB14Bq\x14a\x07rW\x80c\xB7\xF0n\xBE\x14a\x07\x99W\x80c\xBBE\xFE\xF2\x14a\x07\xBCW`\0\x80\xFD[\x80cw\x8EU\xF3\x11a\x01qW\x80c\x8D\xA5\xCB[\x11a\x01KW\x80c\x8D\xA5\xCB[\x14a\x07\x13W\x80c\x91\x04\xC3\x19\x14a\x07$W\x80c\x99\xBE\x81\xC8\x14a\x07?W\x80c\xA1x\x84\x84\x14a\x07RW`\0\x80\xFD[\x80cw\x8EU\xF3\x14a\x06\xC2W\x80c\x7FT\x80q\x14a\x06\xEDW\x80c\x88o\x11\x95\x14a\x07\0W`\0\x80\xFD[\x80c_\x96o\x14\x14a\x06,W\x80c`\xD7\xFA\xED\x14a\x06XW\x80ce\xDA\x12d\x14a\x06kW\x80ch0H5\x14a\x06\x94W\x80cmp\xF7\xAE\x14a\x06\xA7W\x80cqP\x18\xA6\x14a\x06\xBAW`\0\x80\xFD[\x80c3@C\x96\x11a\x02\x92W\x80cO\xC4\x0Ba\x11a\x020W\x80cY{6\xDA\x11a\x02\nW\x80cY{6\xDA\x14a\x05\xDBW\x80cZ\xC8j\xB7\x14a\x05\xEEW\x80c\\\x97Z\xBB\x14a\x06\x11W\x80c\\\xFE\x8D,\x14a\x06\x19W`\0\x80\xFD[\x80cO\xC4\x0Ba\x14a\x05\xC0W\x80cP\xF7>|\x14a\x05\xCAW\x80cY\\jg\x14a\x05\xD3W`\0\x80\xFD[\x80c>(9\x1D\x11a\x02lW\x80c>(9\x1D\x14a\x05<W\x80cC7s\x82\x14a\x05_W\x80cFe\xBC\xDA\x14a\x05\x86W\x80cMP\xF9\xA4\x14a\x05\xADW`\0\x80\xFD[\x80c3@C\x96\x14a\x04\xBBW\x80c9\xB7\x0E8\x14a\x04\xCEW\x80c<\xDE\xB5\xE0\x14a\x05\rW`\0\x80\xFD[\x80c\x13d9\xDD\x11a\x02\xFFW\x80c\x1B\xBC\xE0\x91\x11a\x02\xD9W\x80c\x1B\xBC\xE0\x91\x14a\x04NW\x80c `kp\x14a\x04aW\x80c(\xA5s\xAE\x14a\x04\x88W\x80c)\xC7}O\x14a\x04\x9BW`\0\x80\xFD[\x80c\x13d9\xDD\x14a\x03\xEFW\x80c\x16\x92\x83e\x14a\x04\x02W\x80c\x17\x94\xBB<\x14a\x04;W`\0\x80\xFD[\x80c\x04\xA4\xF9y\x14a\x03GW\x80c\x0B\x9FHz\x14a\x03\x81W\x80c\r\xD8\xDD\x02\x14a\x03\x94W\x80c\x0FX\x9EY\x14a\x03\xB4W\x80c\x10\xD6z/\x14a\x03\xC9W\x80c\x13-Ig\x14a\x03\xDCW[`\0\x80\xFD[a\x03n\x7F;\x89\xFC\xA1Q\xCB\xE5\x12-X\xAC\xEE\x86\xCF\x18D\x13\xD7Q\xD5\x85w\x9B\xDD\x19\xD3\xBB\xFA:0m\xCE\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x03na\x03\x8F6`\x04aDtV[a\tGV[a\x03\xA7a\x03\xA26`\x04aE\x13V[a\n\tV[`@Qa\x03x\x91\x90aETV[a\x03\xC7a\x03\xC26`\x04aE\xEBV[a\r\x83V[\0[a\x03\xC7a\x03\xD76`\x04aF>V[a\x0E\xD3V[a\x03\xC7a\x03\xEA6`\x04aFbV[a\x0F\x86V[a\x03\xC7a\x03\xFD6`\x04aF\xA3V[a\x10FV[a\x03na\x04\x106`\x04aF>V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x90V[a\x03\xC7a\x04I6`\x04aFbV[a\x11\x85V[a\x03na\x04\\6`\x04aFbV[a\x12\xAEV[a\x03n\x7F\x8C\xAD\x95h{\xA8,,\xE5\x0Et\xF7\xB7Td^Q\x17\xC3\xA5\xBE\xC8\x15\x1C\x07&\xD5\x85y\x80\xA8f\x81V[a\x03\xC7a\x04\x966`\x04aFbV[a\x12\xDCV[a\x03na\x04\xA96`\x04aF>V[`\x9B` R`\0\x90\x81R`@\x90 T\x81V[a\x03\xC7a\x04\xC96`\x04aF\xBCV[a\x13\x8CV[a\x04\xF5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03xV[a\x04\xF5a\x05\x1B6`\x04aF>V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T\x16\x90V[a\x05Oa\x05J6`\x04aF>V[a\x14\xC9V[`@Q\x90\x15\x15\x81R` \x01a\x03xV[a\x03n\x7F9\x11\x1B\xC4\xA4\xD6\x88\xE1\xF6\x85\x12=t\x97\xD4aSp\x15*\x8E\xE4\xA0Y>d{\xD0j\xD8\xBB\x0B\x81V[a\x04\xF5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\xC7a\x05\xBB6`\x04aF\xA3V[a\x14\xE9V[a\x03nb\x13\xC6\x80\x81V[a\x03n`\x9DT\x81V[a\x03\xC7a\x15\xC0V[a\x03na\x05\xE96`\x04aI\xFCV[a\x16\x87V[a\x05Oa\x05\xFC6`\x04aJ8V[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`fTa\x03nV[a\x03\xC7a\x06'6`\x04aJ\xAEV[a\x16\xB7V[a\x04\xF5a\x06:6`\x04aF>V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x99` R`@\x90 T\x16\x90V[a\x03\xC7a\x06f6`\x04aL\x0EV[a\x19bV[a\x04\xF5a\x06y6`\x04aF>V[`\x9A` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\xA0Ta\x04\xF5\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x05Oa\x06\xB56`\x04aF>V[a\x19\xFDV[a\x03\xC7a\x1A\x1DV[a\x03na\x06\xD06`\x04aL\x9DV[`\x98` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[a\x03\xC7a\x06\xFB6`\x04aM~V[a\x1A1V[`eTa\x04\xF5\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x04\xF5V[a\x04\xF5s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x81V[a\x03\xC7a\x07M6`\x04aN\x0EV[a\x1B6V[a\x03na\x07`6`\x04aF>V[`\x9F` R`\0\x90\x81R`@\x90 T\x81V[a\x04\xF5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x05Oa\x07\xA76`\x04aF\xA3V[`\x9E` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x05Oa\x07\xCA6`\x04aNCV[`\x9C` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[a\x08Za\x07\xF86`\x04aF>V[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x92\x84\x01\x81\x90R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R`\x99\x83R\x83\x90 \x83Q\x91\x82\x01\x84R\x80T\x85\x16\x82R`\x01\x01T\x93\x84\x16\x91\x81\x01\x91\x90\x91R`\x01`\xA0\x1B\x90\x92\x04c\xFF\xFF\xFF\xFF\x16\x90\x82\x01R\x90V[`@\x80Q\x82Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x80\x85\x01Q\x90\x91\x16\x90\x82\x01R\x91\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R``\x01a\x03xV[a\x03na\x08\x9E6`\x04aNoV[a\x1C\x08V[a\x03na\xC4\xE0\x81V[a\x08\xBFa\x08\xBA6`\x04aF>V[a\x1C\xC1V[`@Qa\x03x\x92\x91\x90aO+V[a\x03na\x08\xDB6`\x04aF>V[a yV[a\x03\xC7a\x08\xEE6`\x04aF>V[a#\xEDV[a\x03\xC7a\t\x016`\x04aOPV[a%UV[a\x03\xC7a\t\x146`\x04aO\xA8V[a%aV[a\x03\xC7a\t'6`\x04aF>V[a%\xF2V[a\x03na&hV[a\x03\xC7a\tB6`\x04aF\xA3V[a&\xA6V[`@\x80Q\x7F;\x89\xFC\xA1Q\xCB\xE5\x12-X\xAC\xEE\x86\xCF\x18D\x13\xD7Q\xD5\x85w\x9B\xDD\x19\xD3\xBB\xFA:0m\xCE` \x80\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x83\x85\x01R\x88\x81\x16``\x84\x01R\x87\x16`\x80\x83\x01R`\xA0\x82\x01\x85\x90R`\xC0\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\xE0\x90\x92\x01\x90\x92R\x80Q\x91\x01 `\0\x90\x81a\t\xC5a&hV[`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x91\x90\x91R`B\x81\x01\x83\x90R`b\x01`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x98\x97PPPPPPPPV[`fT``\x90`\x01\x90`\x02\x90\x81\x16\x14\x15a\n>W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n5\x90aO\xC4V[`@Q\x80\x91\x03\x90\xFD[`\0\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\nXWa\nXaG\x7FV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\n\x81W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x84\x81\x10\x15a\rzW\x85\x85\x82\x81\x81\x10a\n\xA1Wa\n\xA1aO\xFBV[\x90P` \x02\x81\x01\x90a\n\xB3\x91\x90aP\x11V[a\n\xC1\x90` \x81\x01\x90aP1V[\x90P\x86\x86\x83\x81\x81\x10a\n\xD5Wa\n\xD5aO\xFBV[\x90P` \x02\x81\x01\x90a\n\xE7\x91\x90aP\x11V[a\n\xF1\x90\x80aP1V[\x90P\x14a\x0BfW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FDelegationManager.queueWithdrawa`D\x82\x01R\x7Fl: input length mismatch\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\n5V[`\0\x86\x86\x83\x81\x81\x10a\x0BzWa\x0BzaO\xFBV[\x90P` \x02\x81\x01\x90a\x0B\x8C\x91\x90aP\x11V[a\x0B\x9D\x90``\x81\x01\x90`@\x01aF>V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\x0C+W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`H`$\x82\x01R\x7FDelegationManager.queueWithdrawa`D\x82\x01R\x7Fl: must provide valid withdrawal`d\x82\x01Rg address`\xC0\x1B`\x84\x82\x01R`\xA4\x01a\n5V[3`\0\x81\x81R`\x9A` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90a\rJ\x90\x82\x89\x89\x86\x81\x81\x10a\x0C\\Wa\x0C\\aO\xFBV[\x90P` \x02\x81\x01\x90a\x0Cn\x91\x90aP\x11V[a\x0C\x7F\x90``\x81\x01\x90`@\x01aF>V[\x8A\x8A\x87\x81\x81\x10a\x0C\x91Wa\x0C\x91aO\xFBV[\x90P` \x02\x81\x01\x90a\x0C\xA3\x91\x90aP\x11V[a\x0C\xAD\x90\x80aP1V[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8E\x92P\x8D\x91P\x89\x90P\x81\x81\x10a\x0C\xF3Wa\x0C\xF3aO\xFBV[\x90P` \x02\x81\x01\x90a\r\x05\x91\x90aP\x11V[a\r\x13\x90` \x81\x01\x90aP1V[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa(\x02\x92PPPV[\x83\x83\x81Q\x81\x10a\r\\Wa\r\\aO\xFBV[` \x90\x81\x02\x91\x90\x91\x01\x01RP\x80a\rr\x81aP\x90V[\x91PPa\n\x87V[P\x94\x93PPPPV[3`\0\x90\x81R`\x99` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x0E\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R\x7FDelegationManager.registerAsOper`D\x82\x01R\x7Fator: operator has already regis`d\x82\x01Rd\x1D\x19\\\x99Y`\xDA\x1B`\x84\x82\x01R`\xA4\x01a\n5V[a\x0E'3\x84a,<V[`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01Ra\x0EI3\x80\x83`\0a.\xD8V[3`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8E\x84\x85X:#\x10\xD4\x1F|\x82\xB9B}\x0B\xD4\x9B\xADt\xBB\x9C\xFF\x9D4\x02\xA2\x9D\x8F\x9B(\xA0\xE2\x85`@Qa\x0E\x82\x91\x90aP\xABV[`@Q\x80\x91\x03\x90\xA23`\x01`\x01`\xA0\x1B\x03\x16\x7F\x02\xA9\x19\xED\x0E*\xCA\xD1\xDD\x90\xF1~\xF2\xFAJ\xE5F.\xE13\x91p\x03J\x851\xCC\xA4\xB6p\x80\x90\x84\x84`@Qa\x0E\xC5\x92\x91\x90aP\xFDV[`@Q\x80\x91\x03\x90\xA2PPPPV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F&W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0FJ\x91\x90aQ,V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0FzW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n5\x90aQIV[a\x0F\x83\x81a2\x91V[PV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x0F\xE5WP3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14[a\x10\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n5\x90aQ\x93V[a\x10\n\x83a\x14\xC9V[\x15a\x10AW`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R`\x9A` R`@\x90 T\x16a\x106\x81\x85\x85\x85a3\x88V[a\x10?\x81a4\x03V[P[PPPV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\x8EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xB2\x91\x90aQ\xF0V[a\x10\xCEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n5\x90aR\rV[`fT\x81\x81\x16\x14a\x11GW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\n5V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x11\xA5WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x11\xBFWP0;\x15\x80\x15a\x11\xBFWP`\0T`\xFF\x16`\x01\x14[a\x12\"W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\n5V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x12EW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x12O\x83\x83a4\xBDV[a\x12Wa5\xA3V[`\x97Ua\x12c\x84a6:V[\x80\x15a\x10?W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x9B` R`@\x81 Ta\x12\xD3\x85\x82\x86\x86a\x1C\x08V[\x95\x94PPPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x13;WP3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14[a\x13WW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n5\x90aQ\x93V[a\x13`\x83a\x14\xC9V[\x15a\x10AW`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R`\x9A` R`@\x90 T\x16a\x106\x81\x85\x85\x85a6\x8CV[`fT`\x02\x90`\x04\x90\x81\x16\x14\x15a\x13\xB5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n5\x90aO\xC4V[`\x02`\xC9T\x14\x15a\x14\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\n5V[`\x02`\xC9U`\0[\x88\x81\x10\x15a\x14\xB8Wa\x14\xA8\x8A\x8A\x83\x81\x81\x10a\x14-Wa\x14-aO\xFBV[\x90P` \x02\x81\x01\x90a\x14?\x91\x90aRUV[\x89\x89\x84\x81\x81\x10a\x14QWa\x14QaO\xFBV[\x90P` \x02\x81\x01\x90a\x14c\x91\x90aP1V[\x89\x89\x86\x81\x81\x10a\x14uWa\x14uaO\xFBV[\x90P` \x02\x015\x88\x88\x87\x81\x81\x10a\x14\x8EWa\x14\x8EaO\xFBV[\x90P` \x02\x01` \x81\x01\x90a\x14\xA3\x91\x90aRkV[a7\x07V[a\x14\xB1\x81aP\x90V[\x90Pa\x14\x10V[PP`\x01`\xC9UPPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x9A` R`@\x90 T\x16\x15\x15\x90V[a\x14\xF1a=\x81V[a\xC4\xE0\x81\x11\x15a\x15\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`M`$\x82\x01R\x7FDelegationManager.setWithdrawalD`D\x82\x01R\x7FelayBlocks: newWithdrawalDelayBl`d\x82\x01Rl\r\xECmnd\x0E\x8D\xED\xE4\r\r,\xED`\x9B\x1B`\x84\x82\x01R`\xA4\x01a\n5V[`\x9DT`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7FO\xFB\0@\x05t\x14t)\xEE7zV38c!\xE6mE\xD8\xB1Fv\x01K_\xA3\x93\xE6\x1E\x9E\x91\x01`@Q\x80\x91\x03\x90\xA1`\x9DUV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\x08W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16,\x91\x90aQ\xF0V[a\x16HW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n5\x90aR\rV[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[`\0\x81`@Q` \x01a\x16\x9A\x91\x90aR\xFCV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0[\x81Q\x81\x10\x15a\x19^W`\0\x82\x82\x81Q\x81\x10a\x16\xD7Wa\x16\xD7aO\xFBV[` \x02` \x01\x01Q\x90P`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xCD)?o\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x170\x91\x90aS\x0FV[`@\x80Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x17NW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17r\x91\x90aS\xBEV[\x91P\x91P\x81\x15a\x19PW`@\x80\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x9F` R\x91\x82 \x80T\x91\x92\x82\x91\x90a\x17\xA9\x83aP\x90V[\x91\x90PUP`\0`@Q\x80`\xE0\x01`@R\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x87`\xA0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x87``\x01Q`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01\x87`\x80\x01Qc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x87`\0\x01Q\x81R` \x01\x87` \x01Q\x81RP\x90P`\0a\x18)\x82a\x16\x87V[`\0\x81\x81R`\x9E` R`@\x90 T\x90\x91P`\xFF\x16\x15a\x18\xBFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R\x7FDelegationManager.migrateQueuedW`D\x82\x01R\x7Fithdrawals: withdrawal already e`d\x82\x01Rdxists`\xD8\x1B`\x84\x82\x01R`\xA4\x01a\n5V[`\0\x81\x81R`\x9E` R`@\x90\x81\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x7F\x90\t\xAB\x15>\x80\x14\xFB\xFB\x02\xF2!\x7F\\\xDEz\xA7\xF9\xADsJ\xE8\\\xA3\xEE?L\xA2\xFD\xD4\x99\xF9\x90a\x19\n\x90\x83\x90\x85\x90aS\xECV[`@Q\x80\x91\x03\x90\xA1`@\x80Q\x86\x81R` \x81\x01\x83\x90R\x7F\xDC\0u\x8Be\xEE\xF7\x1D\xC3x\x0C\x04\xEB\xE3l\xABk\xDB&l:i\x81\x87\xE2\x9E\x0F\r\xCA\x01&0\x91\x01`@Q\x80\x91\x03\x90\xA1PPPP[\x83`\x01\x01\x93PPPPa\x16\xBAV[PPV[`fT`\x02\x90`\x04\x90\x81\x16\x14\x15a\x19\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n5\x90aO\xC4V[`\x02`\xC9T\x14\x15a\x19\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\n5V[`\x02`\xC9Ua\x19\xF0\x86\x86\x86\x86\x86a7\x07V[PP`\x01`\xC9UPPPPV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x99` R`@\x90 T\x16\x15\x15\x90V[a\x1A%a=\x81V[a\x1A/`\0a6:V[V[B\x83` \x01Q\x10\x15a\x1A\xB5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FDelegationManager.delegateToBySi`D\x82\x01R\x7Fgnature: staker signature expire`d\x82\x01R`\x19`\xFA\x1B`\x84\x82\x01R`\xA4\x01a\n5V[`\0`\x9B`\0\x87`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 T\x90P`\0a\x1A\xF1\x87\x83\x88\x88` \x01Qa\x1C\x08V[`\x01`\x01`\xA0\x1B\x03\x88\x16`\0\x90\x81R`\x9B` R`@\x90 `\x01\x84\x01\x90U\x85Q\x90\x91Pa\x1B!\x90\x88\x90\x83\x90a=\xDBV[a\x1B-\x87\x87\x86\x86a.\xD8V[PPPPPPPV[a\x1B?3a\x19\xFDV[a\x1B\xC1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FDelegationManager.updateOperator`D\x82\x01R\x7FMetadataURI: caller must be an o`d\x82\x01Rf82\xB90\xBA7\xB9`\xC9\x1B`\x84\x82\x01R`\xA4\x01a\n5V[3`\x01`\x01`\xA0\x1B\x03\x16\x7F\x02\xA9\x19\xED\x0E*\xCA\xD1\xDD\x90\xF1~\xF2\xFAJ\xE5F.\xE13\x91p\x03J\x851\xCC\xA4\xB6p\x80\x90\x83\x83`@Qa\x1B\xFC\x92\x91\x90aP\xFDV[`@Q\x80\x91\x03\x90\xA2PPV[`@\x80Q\x7F9\x11\x1B\xC4\xA4\xD6\x88\xE1\xF6\x85\x12=t\x97\xD4aSp\x15*\x8E\xE4\xA0Y>d{\xD0j\xD8\xBB\x0B` \x80\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x83\x85\x01R\x85\x16``\x83\x01R`\x80\x82\x01\x86\x90R`\xA0\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\xC0\x90\x92\x01\x90\x92R\x80Q\x91\x01 `\0\x90\x81a\x1C~a&hV[`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x91\x90\x91R`B\x81\x01\x83\x90R`b\x01`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x97\x96PPPPPPPV[`@Qc`\xF4\x06+`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R``\x91\x82\x91`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c`\xF4\x06+\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1DU\x91\x90aT\x05V[`@Qc\x94\xF6I\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R\x91\x92P`\0\x91\x82\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\x94\xF6I\xDD\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\xC5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1D\xED\x91\x90\x81\x01\x90aTyV[\x91P\x91P`\0\x83\x13a\x1E\x04W\x90\x95\x90\x94P\x92PPPV[``\x80\x83Q`\0\x14\x15a\x1E\xBEW`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x90` \x80\x83\x01\x90\x806\x837PP`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x92\x94P\x90P` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90Ps\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x82`\0\x81Q\x81\x10a\x1EyWa\x1EyaO\xFBV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x84\x81`\0\x81Q\x81\x10a\x1E\xADWa\x1E\xADaO\xFBV[` \x02` \x01\x01\x81\x81RPPa lV[\x83Qa\x1E\xCB\x90`\x01aU=V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E\xE2Wa\x1E\xE2aG\x7FV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1F\x0BW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F'Wa\x1F'aG\x7FV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1FPW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x84Q\x81\x10\x15a\x1F\xEAW\x84\x81\x81Q\x81\x10a\x1FqWa\x1FqaO\xFBV[` \x02` \x01\x01Q\x83\x82\x81Q\x81\x10a\x1F\x8BWa\x1F\x8BaO\xFBV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x83\x81\x81Q\x81\x10a\x1F\xBDWa\x1F\xBDaO\xFBV[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a\x1F\xD7Wa\x1F\xD7aO\xFBV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x1FVV[Ps\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x82`\x01\x84Qa \x0F\x91\x90aUUV[\x81Q\x81\x10a \x1FWa \x1FaO\xFBV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x84\x81`\x01\x84Qa O\x91\x90aUUV[\x81Q\x81\x10a _Wa _aO\xFBV[` \x02` \x01\x01\x81\x81RPP[\x90\x97\x90\x96P\x94PPPPPV[`fT`\0\x90`\x01\x90`\x02\x90\x81\x16\x14\x15a \xA5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n5\x90aO\xC4V[a \xAE\x83a\x14\xC9V[a!.W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FDelegationManager.undelegate: st\x90\x82\x01R\x7Faker must be delegated to undele`d\x82\x01Rcgate`\xE0\x1B`\x84\x82\x01R`\xA4\x01a\n5V[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R`\x9A` R`@\x90 T\x16a!R\x84a\x19\xFDV[\x15a!\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FDelegationManager.undelegate: op`D\x82\x01R\x7Ferators cannot be undelegated\0\0\0`d\x82\x01R`\x84\x01a\n5V[`\x01`\x01`\xA0\x1B\x03\x84\x16a\"AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R\x7FDelegationManager.undelegate: ca`D\x82\x01R\x7Fnnot undelegate zero address\0\0\0\0`d\x82\x01R`\x84\x01a\n5V[3`\x01`\x01`\xA0\x1B\x03\x85\x16\x14\x80a\"`WP3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14[\x80a\"\x87WP`\x01`\x01`\xA0\x1B\x03\x81\x81\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T\x163\x14[a\"\xF9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FDelegationManager.undelegate: ca`D\x82\x01R\x7Fller cannot undelegate staker\0\0\0`d\x82\x01R`\x84\x01a\n5V[`\0\x80a#\x05\x86a\x1C\xC1V[\x90\x92P\x90P3`\x01`\x01`\xA0\x1B\x03\x87\x16\x14a#[W\x82`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x7F\xF0\xED\xDF\x07\xE6\xEA\x14\xF3\x88\xB4~\x1E\x94\xA0\xF4d\xEC\xBD\x9E\xEDAq\x13\x0E\x0F\xC0\xE9\x9F\xB4\x03\n\x8A`@Q`@Q\x80\x91\x03\x90\xA3[\x82`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFE\xE3\tf\xA2V\xB7\x1E\x14\xBC\x0E\xBF\xC9C\x15\xE2\x8E\xF4\xA9zq1\xA9\xE2\xB7\xA3\x10\xA7:\xF4Fv`@Q`@Q\x80\x91\x03\x90\xA3`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x9A` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x81Qa#\xD4WP`\0\x93Pa#\xE7\x91PPV[a#\xE1\x86\x84\x88\x85\x85a(\x02V[\x94PPPP[P\x91\x90PV[a#\xF5a=\x81V[`\xA0T`\x01`\x01`\xA0\x1B\x03\x16\x15a$tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FDelegationManager.setStakeRegist`D\x82\x01R\x7Fry: stakeRegistry already set\0\0\0`d\x82\x01R`\x84\x01a\n5V[`\x01`\x01`\xA0\x1B\x03\x81\x16a%\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`H`$\x82\x01R\x7FDelegationManager.setStakeRegist`D\x82\x01R\x7Fry: stakeRegistry cannot be zero`d\x82\x01Rg address`\xC0\x1B`\x84\x82\x01R`\xA4\x01a\n5V[`\xA0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\xCEm\x87@i\xBC\xED\xA1\x86~\xCA\x9C`ck\xBF&.\x15!0Ae\x82s\xD8\x03\xA2\xB6\t\xA5\x1F\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[a\x10A3\x84\x84\x84a.\xD8V[a%j3a\x19\xFDV[a%\xE8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FDelegationManager.modifyOperator`D\x82\x01R\x7FDetails: caller must be an opera`d\x82\x01Rb:7\xB9`\xE9\x1B`\x84\x82\x01R`\xA4\x01a\n5V[a\x0F\x833\x82a,<V[a%\xFAa=\x81V[`\x01`\x01`\xA0\x1B\x03\x81\x16a&_W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\n5V[a\x0F\x83\x81a6:V[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14\x15a&\x99WP`\x97T\x90V[a&\xA1a5\xA3V[\x90P\x90V[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&\xF9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'\x1D\x91\x90aQ,V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a'MW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n5\x90aQIV[`fT\x19\x81\x19`fT\x19\x16\x14a'\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\n5V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x11zV[`\0`\x01`\x01`\xA0\x1B\x03\x86\x16a(\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`P`$\x82\x01R\x7FDelegationManager._removeSharesA`D\x82\x01R\x7FndQueueWithdrawal: staker cannot`d\x82\x01Ro be zero address`\x80\x1B`\x84\x82\x01R`\xA4\x01a\n5V[\x82Qa)#W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`M`$\x82\x01R\x7FDelegationManager._removeSharesA`D\x82\x01R\x7FndQueueWithdrawal: strategies ca`d\x82\x01Rlnnot be empty`\x98\x1B`\x84\x82\x01R`\xA4\x01a\n5V[`\0[\x83Q\x81\x10\x15a+2W`\x01`\x01`\xA0\x1B\x03\x86\x16\x15a)|Wa)|\x86\x88\x86\x84\x81Q\x81\x10a)UWa)UaO\xFBV[` \x02` \x01\x01Q\x86\x85\x81Q\x81\x10a)oWa)oaO\xFBV[` \x02` \x01\x01Qa3\x88V[s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0`\x01`\x01`\xA0\x1B\x03\x16\x84\x82\x81Q\x81\x10a)\xACWa)\xACaO\xFBV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a*uW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xBE\xFF\xBB\x89\x88\x85\x84\x81Q\x81\x10a*\x05Wa*\x05aO\xFBV[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a*>\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a*XW`\0\x80\xFD[PZ\xF1\x15\x80\x15a*lW=`\0\x80>=`\0\xFD[PPPPa+*V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8C\x80\xD4\xE5\x88\x86\x84\x81Q\x81\x10a*\xB7Wa*\xB7aO\xFBV[` \x02` \x01\x01Q\x86\x85\x81Q\x81\x10a*\xD1Wa*\xD1aO\xFBV[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a*\xF7\x93\x92\x91\x90aUlV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a+\x11W`\0\x80\xFD[PZ\xF1\x15\x80\x15a+%W=`\0\x80>=`\0\xFD[PPPP[`\x01\x01a)&V[P`\x01`\x01`\xA0\x1B\x03\x85\x16\x15a+KWa+K\x85a4\x03V[`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x9F` R`@\x81 \x80T\x91\x82\x91\x90a+r\x83aP\x90V[\x91\x90PUP`\0`@Q\x80`\xE0\x01`@R\x80\x89`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x88`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x87`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86\x81R` \x01\x85\x81RP\x90P`\0a+\xDA\x82a\x16\x87V[`\0\x81\x81R`\x9E` R`@\x90\x81\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x90\x91P\x7F\x90\t\xAB\x15>\x80\x14\xFB\xFB\x02\xF2!\x7F\\\xDEz\xA7\xF9\xADsJ\xE8\\\xA3\xEE?L\xA2\xFD\xD4\x99\xF9\x90a,(\x90\x83\x90\x85\x90aS\xECV[`@Q\x80\x91\x03\x90\xA1\x98\x97PPPPPPPPV[`\0a,K` \x83\x01\x83aF>V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a,\xE5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`T`$\x82\x01R\x7FDelegationManager._setOperatorDe`D\x82\x01R\x7Ftails: cannot set `earningsRecei`d\x82\x01Rsver` to zero address``\x1B`\x84\x82\x01R`\xA4\x01a\n5V[b\x13\xC6\x80a,\xF9``\x83\x01`@\x84\x01aU\x90V[c\xFF\xFF\xFF\xFF\x16\x11\x15a-\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`l`$\x82\x01R\x7FDelegationManager._setOperatorDe`D\x82\x01R\x7Ftails: stakerOptOutWindowBlocks `d\x82\x01R\x7Fcannot be > MAX_STAKER_OPT_OUT_W`\x84\x82\x01RkINDOW_BLOCKS`\xA0\x1B`\xA4\x82\x01R`\xC4\x01a\n5V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x99` R`@\x90\x81\x90 `\x01\x01T`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x90a-\xEA\x90``\x84\x01\x90\x84\x01aU\x90V[c\xFF\xFF\xFF\xFF\x16\x10\x15a.\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`S`$\x82\x01R\x7FDelegationManager._setOperatorDe`D\x82\x01R\x7Ftails: stakerOptOutWindowBlocks `d\x82\x01Rr\x18\xD8[\x9B\x9B\xDD\x08\x18\x99H\x19\x19X\xDC\x99X\\\xD9Y`j\x1B`\x84\x82\x01R`\xA4\x01a\n5V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x99` R`@\x90 \x81\x90a.\xA4\x82\x82aU\xCDV[PP`@Q3\x90\x7F\xFE\xBE\\\xD2K,\xBC{\x06[\x9D\x0F\xDE\xB9\x04F\x1EJ\xFC\xFFW\xDDW\xAC\xDA\x1Ex2\x03\x1B\xA7\xAC\x90a\x1B\xFC\x90\x84\x90aP\xABV[`fT`\0\x90`\x01\x90\x81\x16\x14\x15a/\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n5\x90aO\xC4V[a/\n\x85a\x14\xC9V[\x15a/\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FDelegationManager._delegate: sta`D\x82\x01R\x7Fker is already actively delegate`d\x82\x01R`\x19`\xFA\x1B`\x84\x82\x01R`\xA4\x01a\n5V[a/\x90\x84a\x19\xFDV[a0\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R\x7FDelegationManager._delegate: ope`D\x82\x01R\x7Frator is not registered in Eigen`d\x82\x01Rd&0\xBC\xB2\xB9`\xD9\x1B`\x84\x82\x01R`\xA4\x01a\n5V[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T\x16\x80\x15\x80\x15\x90a0FWP3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14\x15[\x80\x15a0[WP3`\x01`\x01`\xA0\x1B\x03\x86\x16\x14\x15[\x15a1\xC8WB\x84` \x01Q\x10\x15a0\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FDelegationManager._delegate: app`D\x82\x01R\x7Frover signature expired\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\n5V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x9C` \x90\x81R`@\x80\x83 \x86\x84R\x90\x91R\x90 T`\xFF\x16\x15a1tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FDelegationManager._delegate: app`D\x82\x01R\x7FroverSalt already spent\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\n5V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x9C` \x90\x81R`@\x80\x83 \x86\x84R\x82R\x82 \x80T`\xFF\x19\x16`\x01\x17\x90U\x85\x01Qa1\xB5\x90\x88\x90\x88\x90\x85\x90\x88\x90a\tGV[\x90Pa1\xC6\x82\x82\x87`\0\x01Qa=\xDBV[P[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\0\x81\x81R`\x9A` R`@\x80\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x94\x8A\x16\x94\x85\x17\x90UQ\x7F\xC3\xEE\x9F._\xDA\x98\xE8\x06j\x1Ft[-\xF9(_Ao\xE9\x8C\xF2U\x9C\xD2\x14\x84\xB3\xD8t3\x04\x91\x90\xA3`\0\x80a2'\x88a\x1C\xC1V[\x91P\x91P`\0[\x82Q\x81\x10\x15a2}Wa2u\x88\x8A\x85\x84\x81Q\x81\x10a2NWa2NaO\xFBV[` \x02` \x01\x01Q\x85\x85\x81Q\x81\x10a2hWa2haO\xFBV[` \x02` \x01\x01Qa6\x8CV[`\x01\x01a2.V[Pa2\x87\x87a4\x03V[PPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16a3\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\n5V[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x98` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a3\xBF\x90\x84\x90aUUV[\x92PP\x81\x90UP\x83`\x01`\x01`\xA0\x1B\x03\x16\x7Fi\t`\x007\xB7]{G3\xAE\xDD\x81TB\xB5\xEC\x01\x8A\x82wQ\xC82\xAA\xFFd\xEB\xA5\xD6\xD2\xDD\x84\x84\x84`@Qa\x0E\xC5\x93\x92\x91\x90aUlV[`\xA0T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x0F\x83W`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x81\x81`\0\x81Q\x81\x10a4JWa4JaO\xFBV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16` \x91\x82\x02\x92\x90\x92\x01\x01R`\xA0T`@Qc\xCE\x97~\xC3`\xE0\x1B\x81R\x91\x16\x90c\xCE\x97~\xC3\x90a4\x87\x90\x84\x90`\x04\x01aV0V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a4\xA1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a4\xB5W=`\0\x80>=`\0\xFD[PPPPPPV[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a4\xDEWP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a5`W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\n5V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a\x19^\x82a2\x91V[`@\x80Q\x80\x82\x01\x82R`\n\x81Ri\"\xB4\xB3\xB2\xB7&0\xBC\xB2\xB9`\xB1\x1B` \x91\x82\x01R\x81Q\x7F\x8C\xAD\x95h{\xA8,,\xE5\x0Et\xF7\xB7Td^Q\x17\xC3\xA5\xBE\xC8\x15\x1C\x07&\xD5\x85y\x80\xA8f\x81\x83\x01R\x7Fq\xB6%\xCF\xADD\xBA\xC6;\x13\xDB\xA0\x7F.\x1D`\x84\xEE\x04\xB6\xF8u!\x01\xEC\xE6\x12mXN\xE6\xEA\x81\x84\x01RF``\x82\x01R0`\x80\x80\x83\x01\x91\x90\x91R\x83Q\x80\x83\x03\x90\x91\x01\x81R`\xA0\x90\x91\x01\x90\x92R\x81Q\x91\x01 \x90V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x98` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a6\xC3\x90\x84\x90aU=V[\x92PP\x81\x90UP\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\x1E\xC0B\xC9e\xE2\xED\xD7\x10{Q\x18\x8E\xE0\xF3\x83\xE2.v\x17\x90A\xAB:\x9D\x18\xFF\x15\x14\x05\x16l\x84\x84\x84`@Qa\x0E\xC5\x93\x92\x91\x90aUlV[`\0a7\x15a\x05\xE9\x87aVqV[`\0\x81\x81R`\x9E` R`@\x90 T\x90\x91P`\xFF\x16a7\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R`\0\x80Q` aW!\x839\x81Q\x91R`D\x82\x01R\x7FAction: action is not in queue\0\0`d\x82\x01R`\x84\x01a\n5V[`\x9DTC\x90a7\x9F`\xA0\x89\x01`\x80\x8A\x01aU\x90V[c\xFF\xFF\xFF\xFF\x16a7\xAF\x91\x90aU=V[\x11\x15a87W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`W`$\x82\x01R`\0\x80Q` aW!\x839\x81Q\x91R`D\x82\x01R\x7FAction: withdrawalDelayBlocks pe`d\x82\x01R\x7Friod has not yet passed\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\n5V[a8G``\x87\x01`@\x88\x01aF>V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a8\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`K`$\x82\x01R`\0\x80Q` aW!\x839\x81Q\x91R`D\x82\x01R\x7FAction: only withdrawer can comp`d\x82\x01Rj62\xBA2\x900\xB1\xBA4\xB7\xB7`\xA9\x1B`\x84\x82\x01R`\xA4\x01a\n5V[\x81\x15a9FWa8\xE2`\xA0\x87\x01\x87aP1V[\x85\x14\x90Pa9FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R`\0\x80Q` aW!\x839\x81Q\x91R`D\x82\x01R\x7FAction: input length mismatch\0\0\0`d\x82\x01R`\x84\x01a\n5V[`\0\x81\x81R`\x9E` R`@\x90 \x80T`\xFF\x19\x16\x90U\x81\x15a:\x1CW`\0[a9r`\xA0\x88\x01\x88aP1V[\x90P\x81\x10\x15a:\x16Wa:\x0Ea9\x8B` \x89\x01\x89aF>V[3a9\x99`\xA0\x8B\x01\x8BaP1V[\x85\x81\x81\x10a9\xA9Wa9\xA9aO\xFBV[\x90P` \x02\x01` \x81\x01\x90a9\xBE\x91\x90aF>V[a9\xCB`\xC0\x8C\x01\x8CaP1V[\x86\x81\x81\x10a9\xDBWa9\xDBaO\xFBV[\x90P` \x02\x015\x8A\x8A\x87\x81\x81\x10a9\xF4Wa9\xF4aO\xFBV[\x90P` \x02\x01` \x81\x01\x90a:\t\x91\x90aF>V[a?\x95V[`\x01\x01a9eV[Pa=FV[3`\0\x90\x81R`\x9A` R`@\x81 T`\x01`\x01`\xA0\x1B\x03\x16\x90[a:D`\xA0\x89\x01\x89aP1V[\x90P\x81\x10\x15a=:Ws\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0a:o`\xA0\x8A\x01\x8AaP1V[\x83\x81\x81\x10a:\x7FWa:\x7FaO\xFBV[\x90P` \x02\x01` \x81\x01\x90a:\x94\x91\x90aF>V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a;\xEDW`\0a:\xB2` \x8A\x01\x8AaF>V[\x90P`\0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\x0E\x81\x07<\x83a:\xF3`\xC0\x8E\x01\x8EaP1V[\x87\x81\x81\x10a;\x03Wa;\x03aO\xFBV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x87\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x94\x16`\x04\x85\x01R` \x02\x91\x90\x91\x015`$\x83\x01RP`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a;WW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a;{\x91\x90aT\x05V[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R`\x9A` R`@\x90 T\x91\x92P\x16\x80\x15a;\xE5Wa;\xDC\x81\x84a;\xB1`\xA0\x8F\x01\x8FaP1V[\x88\x81\x81\x10a;\xC1Wa;\xC1aO\xFBV[\x90P` \x02\x01` \x81\x01\x90a;\xD6\x91\x90aF>V[\x85a6\x8CV[a;\xE5\x81a4\x03V[PPPa=2V[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16cP\xFFr%3a<*`\xA0\x8C\x01\x8CaP1V[\x85\x81\x81\x10a<:Wa<:aO\xFBV[\x90P` \x02\x01` \x81\x01\x90a<O\x91\x90aF>V[a<\\`\xC0\x8D\x01\x8DaP1V[\x86\x81\x81\x10a<lWa<laO\xFBV[\x90P` \x02\x015`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a<\x91\x93\x92\x91\x90aUlV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a<\xABW`\0\x80\xFD[PZ\xF1\x15\x80\x15a<\xBFW=`\0\x80>=`\0\xFD[PPPP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a=2Wa=2\x823a<\xE4`\xA0\x8C\x01\x8CaP1V[\x85\x81\x81\x10a<\xF4Wa<\xF4aO\xFBV[\x90P` \x02\x01` \x81\x01\x90a=\t\x91\x90aF>V[a=\x16`\xC0\x8D\x01\x8DaP1V[\x86\x81\x81\x10a=&Wa=&aO\xFBV[\x90P` \x02\x015a6\x8CV[`\x01\x01a:7V[Pa=D\x81a4\x03V[P[`@Q\x81\x81R\x7F\xC9p\x98\xC2\xF6X\x80\x0BM\xF2\x90\x01R\x7Fs$\xBC\xDF\xFC\xF6\xE8u\x1Ai\x9A\xB9 \xA1\xEC\xED[\x1D\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPPPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1A/W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\n5V[`\x01`\x01`\xA0\x1B\x03\x83\x16;\x15a>\xF5W`@Qc\x0B\x13]?`\xE1\x1B\x80\x82R\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\x16&\xBA~\x90a>\x1B\x90\x86\x90\x86\x90`\x04\x01aV\x83V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a>8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a>\\\x91\x90aV\xE0V[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14a\x10AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`S`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: ERC1271 signature `d\x82\x01Rr\x1D\x99\\\x9AY\x9AX\xD8]\x1A[\xDB\x88\x19\x98Z[\x19Y`j\x1B`\x84\x82\x01R`\xA4\x01a\n5V[\x82`\x01`\x01`\xA0\x1B\x03\x16a?\t\x83\x83a@\xDAV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x10AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: signature not from`d\x82\x01Rf\x109\xB4\xB3\xB72\xB9`\xC9\x1B`\x84\x82\x01R`\xA4\x01a\n5V[`\x01`\x01`\xA0\x1B\x03\x83\x16s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x14\x15a@@W`@Qb8{\x13`\xE8\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c8{\x13\0\x90a@\t\x90\x88\x90\x88\x90\x87\x90`\x04\x01aUlV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a@#W`\0\x80\xFD[PZ\xF1\x15\x80\x15a@7W=`\0\x80>=`\0\xFD[PPPPa@\xD3V[`@Qc\xC6\x08\xC7\xF3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x84\x81\x16`$\x83\x01R`D\x82\x01\x84\x90R\x82\x81\x16`d\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xC6\x08\xC7\xF3\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a@\xBAW`\0\x80\xFD[PZ\xF1\x15\x80\x15a@\xCEW=`\0\x80>=`\0\xFD[PPPP[PPPPPV[`\0\x80`\0a@\xE9\x85\x85a@\xFEV[\x91P\x91Pa@\xF6\x81aAnV[P\x93\x92PPPV[`\0\x80\x82Q`A\x14\x15aA5W` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1AaA)\x87\x82\x85\x85aC)V[\x94P\x94PPPPaAgV[\x82Q`@\x14\x15aA_W` \x83\x01Q`@\x84\x01QaAT\x86\x83\x83aD\x16V[\x93P\x93PPPaAgV[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0\x81`\x04\x81\x11\x15aA\x82WaA\x82aW\nV[\x14\x15aA\x8BWPV[`\x01\x81`\x04\x81\x11\x15aA\x9FWaA\x9FaW\nV[\x14\x15aA\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\n5V[`\x02\x81`\x04\x81\x11\x15aB\x01WaB\x01aW\nV[\x14\x15aBOW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\n5V[`\x03\x81`\x04\x81\x11\x15aBcWaBcaW\nV[\x14\x15aB\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\n5V[`\x04\x81`\x04\x81\x11\x15aB\xD0WaB\xD0aW\nV[\x14\x15a\x0F\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 'v' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\n5V[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15aC`WP`\0\x90P`\x03aD\rV[\x84`\xFF\x16`\x1B\x14\x15\x80\x15aCxWP\x84`\xFF\x16`\x1C\x14\x15[\x15aC\x89WP`\0\x90P`\x04aD\rV[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15aC\xDDW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16aD\x06W`\0`\x01\x92P\x92PPaD\rV[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x80`\x01`\x01`\xFF\x1B\x03\x83\x16\x81aD3`\xFF\x86\x90\x1C`\x1BaU=V[\x90PaDA\x87\x82\x88\x85aC)V[\x93P\x93PPP\x93P\x93\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0F\x83W`\0\x80\xFD[\x805aDo\x81aDOV[\x91\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15aD\x8CW`\0\x80\xFD[\x855aD\x97\x81aDOV[\x94P` \x86\x015aD\xA7\x81aDOV[\x93P`@\x86\x015aD\xB7\x81aDOV[\x94\x97\x93\x96P\x93\x94``\x81\x015\x94P`\x80\x015\x92\x91PPV[`\0\x80\x83`\x1F\x84\x01\x12aD\xE1W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aD\xF8W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15aAgW`\0\x80\xFD[`\0\x80` \x83\x85\x03\x12\x15aE&W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aE<W`\0\x80\xFD[aEH\x85\x82\x86\x01aD\xCFV[\x90\x96\x90\x95P\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aE\x8CW\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aEpV[P\x90\x96\x95PPPPPPV[`\0``\x82\x84\x03\x12\x15a#\xE7W`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12aE\xBCW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aE\xD3W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aAgW`\0\x80\xFD[`\0\x80`\0`\x80\x84\x86\x03\x12\x15aF\0W`\0\x80\xFD[aF\n\x85\x85aE\x98V[\x92P``\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aF%W`\0\x80\xFD[aF1\x86\x82\x87\x01aE\xAAV[\x94\x97\x90\x96P\x93\x94PPPPV[`\0` \x82\x84\x03\x12\x15aFPW`\0\x80\xFD[\x815aF[\x81aDOV[\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15aFwW`\0\x80\xFD[\x835aF\x82\x81aDOV[\x92P` \x84\x015aF\x92\x81aDOV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0` \x82\x84\x03\x12\x15aF\xB5W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80`\x80\x89\x8B\x03\x12\x15aF\xD8W`\0\x80\xFD[\x885`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aF\xEFW`\0\x80\xFD[aF\xFB\x8C\x83\x8D\x01aD\xCFV[\x90\x9AP\x98P` \x8B\x015\x91P\x80\x82\x11\x15aG\x14W`\0\x80\xFD[aG \x8C\x83\x8D\x01aD\xCFV[\x90\x98P\x96P`@\x8B\x015\x91P\x80\x82\x11\x15aG9W`\0\x80\xFD[aGE\x8C\x83\x8D\x01aD\xCFV[\x90\x96P\x94P``\x8B\x015\x91P\x80\x82\x11\x15aG^W`\0\x80\xFD[PaGk\x8B\x82\x8C\x01aD\xCFV[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aG\xB7WaG\xB7aG\x7FV[`@R\x90V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aG\xB7WaG\xB7aG\x7FV[`@Q`\xC0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aG\xB7WaG\xB7aG\x7FV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aH)WaH)aG\x7FV[`@R\x91\x90PV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0F\x83W`\0\x80\xFD[\x805aDo\x81aH1V[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15aHgWaHgaG\x7FV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12aH\x82W`\0\x80\xFD[\x815` aH\x97aH\x92\x83aHNV[aH\x01V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aH\xB6W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aH\xDAW\x805aH\xCD\x81aDOV[\x83R\x91\x83\x01\x91\x83\x01aH\xBAV[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12aH\xF6W`\0\x80\xFD[\x815` aI\x06aH\x92\x83aHNV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aI%W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aH\xDAW\x805\x83R\x91\x83\x01\x91\x83\x01aI)V[`\0`\xE0\x82\x84\x03\x12\x15aIRW`\0\x80\xFD[aIZaG\x95V[\x90PaIe\x82aDdV[\x81RaIs` \x83\x01aDdV[` \x82\x01RaI\x84`@\x83\x01aDdV[`@\x82\x01R``\x82\x015``\x82\x01RaI\x9F`\x80\x83\x01aHCV[`\x80\x82\x01R`\xA0\x82\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aI\xBEW`\0\x80\xFD[aI\xCA\x85\x83\x86\x01aHqV[`\xA0\x84\x01R`\xC0\x84\x015\x91P\x80\x82\x11\x15aI\xE3W`\0\x80\xFD[PaI\xF0\x84\x82\x85\x01aH\xE5V[`\xC0\x83\x01RP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15aJ\x0EW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aJ$W`\0\x80\xFD[aJ0\x84\x82\x85\x01aI@V[\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15aJJW`\0\x80\xFD[\x815`\xFF\x81\x16\x81\x14aF[W`\0\x80\xFD[`\0`@\x82\x84\x03\x12\x15aJmW`\0\x80\xFD[aJuaG\xBDV[\x90P\x815aJ\x82\x81aDOV[\x81R` \x82\x015k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14aJ\xA3W`\0\x80\xFD[` \x82\x01R\x92\x91PPV[`\0` \x80\x83\x85\x03\x12\x15aJ\xC1W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aJ\xD8W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12aJ\xECW`\0\x80\xFD[\x815aJ\xFAaH\x92\x82aHNV[\x81\x81R`\x05\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x84\x81\x01\x90\x88\x83\x11\x15aK\x19W`\0\x80\xFD[\x85\x85\x01[\x83\x81\x10\x15aK\xF3W\x805\x85\x81\x11\x15aK5W`\0\x80\x81\xFD[\x86\x01`\xE0\x81\x8C\x03`\x1F\x19\x01\x81\x13\x15aKMW`\0\x80\x81\xFD[aKUaG\xDFV[\x89\x83\x015\x88\x81\x11\x15aKgW`\0\x80\x81\xFD[aKu\x8E\x8C\x83\x87\x01\x01aHqV[\x82RP`@\x80\x84\x015\x89\x81\x11\x15aK\x8CW`\0\x80\x81\xFD[aK\x9A\x8F\x8D\x83\x88\x01\x01aH\xE5V[\x8C\x84\x01RP``aK\xAC\x81\x86\x01aDdV[\x82\x84\x01R`\x80\x91PaK\xC0\x8F\x83\x87\x01aJ[V[\x90\x83\x01RaK\xD0`\xC0\x85\x01aHCV[\x90\x82\x01RaK\xDF\x83\x83\x01aDdV[`\xA0\x82\x01R\x85RPP\x91\x86\x01\x91\x86\x01aK\x1DV[P\x98\x97PPPPPPPPV[\x80\x15\x15\x81\x14a\x0F\x83W`\0\x80\xFD[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15aL&W`\0\x80\xFD[\x855`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aL=W`\0\x80\xFD[\x90\x87\x01\x90`\xE0\x82\x8A\x03\x12\x15aLQW`\0\x80\xFD[\x90\x95P` \x87\x015\x90\x80\x82\x11\x15aLgW`\0\x80\xFD[PaLt\x88\x82\x89\x01aD\xCFV[\x90\x95P\x93PP`@\x86\x015\x91P``\x86\x015aL\x8F\x81aL\0V[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x80`@\x83\x85\x03\x12\x15aL\xB0W`\0\x80\xFD[\x825aL\xBB\x81aDOV[\x91P` \x83\x015aL\xCB\x81aDOV[\x80\x91PP\x92P\x92\x90PV[`\0`@\x82\x84\x03\x12\x15aL\xE8W`\0\x80\xFD[aL\xF0aG\xBDV[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aM\tW`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12aM\x1DW`\0\x80\xFD[\x815` \x82\x82\x11\x15aM1WaM1aG\x7FV[aMC`\x1F\x83\x01`\x1F\x19\x16\x82\x01aH\x01V[\x92P\x81\x83R\x86\x81\x83\x86\x01\x01\x11\x15aMYW`\0\x80\xFD[\x81\x81\x85\x01\x82\x85\x017`\0\x81\x83\x85\x01\x01R\x82\x85R\x80\x86\x015\x81\x86\x01RPPPP\x92\x91PPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15aM\x96W`\0\x80\xFD[\x855aM\xA1\x81aDOV[\x94P` \x86\x015aM\xB1\x81aDOV[\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aM\xCDW`\0\x80\xFD[aM\xD9\x89\x83\x8A\x01aL\xD6V[\x94P``\x88\x015\x91P\x80\x82\x11\x15aM\xEFW`\0\x80\xFD[PaM\xFC\x88\x82\x89\x01aL\xD6V[\x95\x98\x94\x97P\x92\x95`\x80\x015\x93\x92PPPV[`\0\x80` \x83\x85\x03\x12\x15aN!W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aN7W`\0\x80\xFD[aEH\x85\x82\x86\x01aE\xAAV[`\0\x80`@\x83\x85\x03\x12\x15aNVW`\0\x80\xFD[\x825aNa\x81aDOV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aN\x85W`\0\x80\xFD[\x845aN\x90\x81aDOV[\x93P` \x85\x015\x92P`@\x85\x015aN\xA7\x81aDOV[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aN\xF0W\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aN\xCBV[P\x94\x95\x94PPPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aN\xF0W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aO\x0FV[`@\x81R`\0aO>`@\x83\x01\x85aN\xB7V[\x82\x81\x03` \x84\x01Ra\x12\xD3\x81\x85aN\xFBV[`\0\x80`\0``\x84\x86\x03\x12\x15aOeW`\0\x80\xFD[\x835aOp\x81aDOV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aO\x8BW`\0\x80\xFD[aO\x97\x86\x82\x87\x01aL\xD6V[\x92PP`@\x84\x015\x90P\x92P\x92P\x92V[`\0``\x82\x84\x03\x12\x15aO\xBAW`\0\x80\xFD[aF[\x83\x83aE\x98V[` \x80\x82R`\x19\x90\x82\x01R\x7FPausable: index is paused\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x825`^\x19\x836\x03\x01\x81\x12aP'W`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aPHW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aPbW`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15aAgW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15aP\xA4WaP\xA4aPzV[P`\x01\x01\x90V[``\x81\x01\x825aP\xBA\x81aDOV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x84\x015\x90aP\xD6\x82aDOV[\x16` \x83\x01R`@\x83\x015aP\xEA\x81aH1V[c\xFF\xFF\xFF\xFF\x81\x16`@\x84\x01RP\x92\x91PPV[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017`\0\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x91\x90PV[`\0` \x82\x84\x03\x12\x15aQ>W`\0\x80\xFD[\x81QaF[\x81aDOV[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`7\x90\x82\x01R\x7FDelegationManager: onlyStrategyM`@\x82\x01R\x7FanagerOrEigenPodManager\0\0\0\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15aR\x02W`\0\x80\xFD[\x81QaF[\x81aL\0V[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[`\0\x825`\xDE\x19\x836\x03\x01\x81\x12aP'W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aR}W`\0\x80\xFD[\x815aF[\x81aL\0V[`\0`\x01\x80`\xA0\x1B\x03\x80\x83Q\x16\x84R\x80` \x84\x01Q\x16` \x85\x01R\x80`@\x84\x01Q\x16`@\x85\x01RP``\x82\x01Q``\x84\x01Rc\xFF\xFF\xFF\xFF`\x80\x83\x01Q\x16`\x80\x84\x01R`\xA0\x82\x01Q`\xE0`\xA0\x85\x01RaR\xE3`\xE0\x85\x01\x82aN\xB7V[\x90P`\xC0\x83\x01Q\x84\x82\x03`\xC0\x86\x01Ra\x12\xD3\x82\x82aN\xFBV[` \x81R`\0aF[` \x83\x01\x84aR\x88V[` \x81R`\0\x82Q`\xE0` \x84\x01RaS,a\x01\0\x84\x01\x82aN\xB7V[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01RaSI\x82\x82aN\xFBV[\x91PP`@\x84\x01Q`\x01\x80`\xA0\x1B\x03\x80\x82\x16``\x86\x01R``\x86\x01Q\x91P\x80\x82Q\x16`\x80\x86\x01RPk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16`\xA0\x85\x01RP`\x80\x84\x01QaS\xA1`\xC0\x85\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P`\xA0\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16`\xE0\x85\x01RP\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15aS\xD1W`\0\x80\xFD[\x82QaS\xDC\x81aL\0V[` \x93\x90\x93\x01Q\x92\x94\x92\x93PPPV[\x82\x81R`@` \x82\x01R`\0aJ0`@\x83\x01\x84aR\x88V[`\0` \x82\x84\x03\x12\x15aT\x17W`\0\x80\xFD[PQ\x91\x90PV[`\0\x82`\x1F\x83\x01\x12aT/W`\0\x80\xFD[\x81Q` aT?aH\x92\x83aHNV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aT^W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aH\xDAW\x80Q\x83R\x91\x83\x01\x91\x83\x01aTbV[`\0\x80`@\x83\x85\x03\x12\x15aT\x8CW`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aT\xA3W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12aT\xB7W`\0\x80\xFD[\x81Q` aT\xC7aH\x92\x83aHNV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15aT\xE6W`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15aU\rW\x85QaT\xFE\x81aDOV[\x82R\x94\x82\x01\x94\x90\x82\x01\x90aT\xEBV[\x91\x88\x01Q\x91\x96P\x90\x93PPP\x80\x82\x11\x15aU&W`\0\x80\xFD[PaU3\x85\x82\x86\x01aT\x1EV[\x91PP\x92P\x92\x90PV[`\0\x82\x19\x82\x11\x15aUPWaUPaPzV[P\x01\x90V[`\0\x82\x82\x10\x15aUgWaUgaPzV[P\x03\x90V[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[`\0` \x82\x84\x03\x12\x15aU\xA2W`\0\x80\xFD[\x815aF[\x81aH1V[\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\x815aU\xD8\x81aDOV[aU\xE2\x81\x83aU\xADV[P`\x01\x81\x01` \x83\x015aU\xF5\x81aDOV[aU\xFF\x81\x83aU\xADV[P`@\x83\x015aV\x0E\x81aH1V[\x81Tc\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x16`\xA0\x91\x90\x91\x1Bc\xFF\xFF\xFF\xFF`\xA0\x1B\x16\x17\x90UPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aE\x8CW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aVLV[`\0aV}6\x83aI@V[\x92\x91PPV[\x82\x81R`\0` `@\x81\x84\x01R\x83Q\x80`@\x85\x01R`\0[\x81\x81\x10\x15aV\xB7W\x85\x81\x01\x83\x01Q\x85\x82\x01``\x01R\x82\x01aV\x9BV[\x81\x81\x11\x15aV\xC9W`\0``\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01``\x01\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15aV\xF2W`\0\x80\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14aF[W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD\xFEDelegationManager.completeQueued\xA2dipfsX\"\x12 2\xCC\xFB\"0E\x9C7\x1A\xBA\xCB\xD0\xDA\x01\x9C\xD5o%V\x8D\xE7~5!VT\x8F@R\xCA\xBE\xD7dsolcC\0\x08\x0C\x003";
    /// The bytecode of the contract.
    pub static DELEGATIONMANAGER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x03BW`\x005`\xE0\x1C\x80c_\x96o\x14\x11a\x01\xB8W\x80c\xB14Bq\x11a\x01\x04W\x80c\xDA\x8B\xE8d\x11a\0\xA2W\x80c\xF1ar\xB0\x11a\0|W\x80c\xF1ar\xB0\x14a\t\x06W\x80c\xF2\xFD\xE3\x8B\x14a\t\x19W\x80c\xF6\x98\xDA%\x14a\t,W\x80c\xFA\xBC\x1C\xBC\x14a\t4W`\0\x80\xFD[\x80c\xDA\x8B\xE8d\x14a\x08\xCDW\x80c\xE3\xB0_/\x14a\x08\xE0W\x80c\xEE\xA9\x06K\x14a\x08\xF3W`\0\x80\xFD[\x80c\xC5\xE4\x80\xDB\x11a\0\xDEW\x80c\xC5\xE4\x80\xDB\x14a\x07\xEAW\x80c\xC9KQ\x11\x14a\x08\x90W\x80c\xCAf\x1C\x04\x14a\x08\xA3W\x80c\xCF\x80\x87>\x14a\x08\xACW`\0\x80\xFD[\x80c\xB14Bq\x14a\x07rW\x80c\xB7\xF0n\xBE\x14a\x07\x99W\x80c\xBBE\xFE\xF2\x14a\x07\xBCW`\0\x80\xFD[\x80cw\x8EU\xF3\x11a\x01qW\x80c\x8D\xA5\xCB[\x11a\x01KW\x80c\x8D\xA5\xCB[\x14a\x07\x13W\x80c\x91\x04\xC3\x19\x14a\x07$W\x80c\x99\xBE\x81\xC8\x14a\x07?W\x80c\xA1x\x84\x84\x14a\x07RW`\0\x80\xFD[\x80cw\x8EU\xF3\x14a\x06\xC2W\x80c\x7FT\x80q\x14a\x06\xEDW\x80c\x88o\x11\x95\x14a\x07\0W`\0\x80\xFD[\x80c_\x96o\x14\x14a\x06,W\x80c`\xD7\xFA\xED\x14a\x06XW\x80ce\xDA\x12d\x14a\x06kW\x80ch0H5\x14a\x06\x94W\x80cmp\xF7\xAE\x14a\x06\xA7W\x80cqP\x18\xA6\x14a\x06\xBAW`\0\x80\xFD[\x80c3@C\x96\x11a\x02\x92W\x80cO\xC4\x0Ba\x11a\x020W\x80cY{6\xDA\x11a\x02\nW\x80cY{6\xDA\x14a\x05\xDBW\x80cZ\xC8j\xB7\x14a\x05\xEEW\x80c\\\x97Z\xBB\x14a\x06\x11W\x80c\\\xFE\x8D,\x14a\x06\x19W`\0\x80\xFD[\x80cO\xC4\x0Ba\x14a\x05\xC0W\x80cP\xF7>|\x14a\x05\xCAW\x80cY\\jg\x14a\x05\xD3W`\0\x80\xFD[\x80c>(9\x1D\x11a\x02lW\x80c>(9\x1D\x14a\x05<W\x80cC7s\x82\x14a\x05_W\x80cFe\xBC\xDA\x14a\x05\x86W\x80cMP\xF9\xA4\x14a\x05\xADW`\0\x80\xFD[\x80c3@C\x96\x14a\x04\xBBW\x80c9\xB7\x0E8\x14a\x04\xCEW\x80c<\xDE\xB5\xE0\x14a\x05\rW`\0\x80\xFD[\x80c\x13d9\xDD\x11a\x02\xFFW\x80c\x1B\xBC\xE0\x91\x11a\x02\xD9W\x80c\x1B\xBC\xE0\x91\x14a\x04NW\x80c `kp\x14a\x04aW\x80c(\xA5s\xAE\x14a\x04\x88W\x80c)\xC7}O\x14a\x04\x9BW`\0\x80\xFD[\x80c\x13d9\xDD\x14a\x03\xEFW\x80c\x16\x92\x83e\x14a\x04\x02W\x80c\x17\x94\xBB<\x14a\x04;W`\0\x80\xFD[\x80c\x04\xA4\xF9y\x14a\x03GW\x80c\x0B\x9FHz\x14a\x03\x81W\x80c\r\xD8\xDD\x02\x14a\x03\x94W\x80c\x0FX\x9EY\x14a\x03\xB4W\x80c\x10\xD6z/\x14a\x03\xC9W\x80c\x13-Ig\x14a\x03\xDCW[`\0\x80\xFD[a\x03n\x7F;\x89\xFC\xA1Q\xCB\xE5\x12-X\xAC\xEE\x86\xCF\x18D\x13\xD7Q\xD5\x85w\x9B\xDD\x19\xD3\xBB\xFA:0m\xCE\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x03na\x03\x8F6`\x04aDtV[a\tGV[a\x03\xA7a\x03\xA26`\x04aE\x13V[a\n\tV[`@Qa\x03x\x91\x90aETV[a\x03\xC7a\x03\xC26`\x04aE\xEBV[a\r\x83V[\0[a\x03\xC7a\x03\xD76`\x04aF>V[a\x0E\xD3V[a\x03\xC7a\x03\xEA6`\x04aFbV[a\x0F\x86V[a\x03\xC7a\x03\xFD6`\x04aF\xA3V[a\x10FV[a\x03na\x04\x106`\x04aF>V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x90V[a\x03\xC7a\x04I6`\x04aFbV[a\x11\x85V[a\x03na\x04\\6`\x04aFbV[a\x12\xAEV[a\x03n\x7F\x8C\xAD\x95h{\xA8,,\xE5\x0Et\xF7\xB7Td^Q\x17\xC3\xA5\xBE\xC8\x15\x1C\x07&\xD5\x85y\x80\xA8f\x81V[a\x03\xC7a\x04\x966`\x04aFbV[a\x12\xDCV[a\x03na\x04\xA96`\x04aF>V[`\x9B` R`\0\x90\x81R`@\x90 T\x81V[a\x03\xC7a\x04\xC96`\x04aF\xBCV[a\x13\x8CV[a\x04\xF5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03xV[a\x04\xF5a\x05\x1B6`\x04aF>V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T\x16\x90V[a\x05Oa\x05J6`\x04aF>V[a\x14\xC9V[`@Q\x90\x15\x15\x81R` \x01a\x03xV[a\x03n\x7F9\x11\x1B\xC4\xA4\xD6\x88\xE1\xF6\x85\x12=t\x97\xD4aSp\x15*\x8E\xE4\xA0Y>d{\xD0j\xD8\xBB\x0B\x81V[a\x04\xF5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\xC7a\x05\xBB6`\x04aF\xA3V[a\x14\xE9V[a\x03nb\x13\xC6\x80\x81V[a\x03n`\x9DT\x81V[a\x03\xC7a\x15\xC0V[a\x03na\x05\xE96`\x04aI\xFCV[a\x16\x87V[a\x05Oa\x05\xFC6`\x04aJ8V[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`fTa\x03nV[a\x03\xC7a\x06'6`\x04aJ\xAEV[a\x16\xB7V[a\x04\xF5a\x06:6`\x04aF>V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x99` R`@\x90 T\x16\x90V[a\x03\xC7a\x06f6`\x04aL\x0EV[a\x19bV[a\x04\xF5a\x06y6`\x04aF>V[`\x9A` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\xA0Ta\x04\xF5\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x05Oa\x06\xB56`\x04aF>V[a\x19\xFDV[a\x03\xC7a\x1A\x1DV[a\x03na\x06\xD06`\x04aL\x9DV[`\x98` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[a\x03\xC7a\x06\xFB6`\x04aM~V[a\x1A1V[`eTa\x04\xF5\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x04\xF5V[a\x04\xF5s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x81V[a\x03\xC7a\x07M6`\x04aN\x0EV[a\x1B6V[a\x03na\x07`6`\x04aF>V[`\x9F` R`\0\x90\x81R`@\x90 T\x81V[a\x04\xF5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x05Oa\x07\xA76`\x04aF\xA3V[`\x9E` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x05Oa\x07\xCA6`\x04aNCV[`\x9C` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[a\x08Za\x07\xF86`\x04aF>V[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x92\x84\x01\x81\x90R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R`\x99\x83R\x83\x90 \x83Q\x91\x82\x01\x84R\x80T\x85\x16\x82R`\x01\x01T\x93\x84\x16\x91\x81\x01\x91\x90\x91R`\x01`\xA0\x1B\x90\x92\x04c\xFF\xFF\xFF\xFF\x16\x90\x82\x01R\x90V[`@\x80Q\x82Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x80\x85\x01Q\x90\x91\x16\x90\x82\x01R\x91\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R``\x01a\x03xV[a\x03na\x08\x9E6`\x04aNoV[a\x1C\x08V[a\x03na\xC4\xE0\x81V[a\x08\xBFa\x08\xBA6`\x04aF>V[a\x1C\xC1V[`@Qa\x03x\x92\x91\x90aO+V[a\x03na\x08\xDB6`\x04aF>V[a yV[a\x03\xC7a\x08\xEE6`\x04aF>V[a#\xEDV[a\x03\xC7a\t\x016`\x04aOPV[a%UV[a\x03\xC7a\t\x146`\x04aO\xA8V[a%aV[a\x03\xC7a\t'6`\x04aF>V[a%\xF2V[a\x03na&hV[a\x03\xC7a\tB6`\x04aF\xA3V[a&\xA6V[`@\x80Q\x7F;\x89\xFC\xA1Q\xCB\xE5\x12-X\xAC\xEE\x86\xCF\x18D\x13\xD7Q\xD5\x85w\x9B\xDD\x19\xD3\xBB\xFA:0m\xCE` \x80\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x83\x85\x01R\x88\x81\x16``\x84\x01R\x87\x16`\x80\x83\x01R`\xA0\x82\x01\x85\x90R`\xC0\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\xE0\x90\x92\x01\x90\x92R\x80Q\x91\x01 `\0\x90\x81a\t\xC5a&hV[`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x91\x90\x91R`B\x81\x01\x83\x90R`b\x01`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x98\x97PPPPPPPPV[`fT``\x90`\x01\x90`\x02\x90\x81\x16\x14\x15a\n>W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n5\x90aO\xC4V[`@Q\x80\x91\x03\x90\xFD[`\0\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\nXWa\nXaG\x7FV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\n\x81W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x84\x81\x10\x15a\rzW\x85\x85\x82\x81\x81\x10a\n\xA1Wa\n\xA1aO\xFBV[\x90P` \x02\x81\x01\x90a\n\xB3\x91\x90aP\x11V[a\n\xC1\x90` \x81\x01\x90aP1V[\x90P\x86\x86\x83\x81\x81\x10a\n\xD5Wa\n\xD5aO\xFBV[\x90P` \x02\x81\x01\x90a\n\xE7\x91\x90aP\x11V[a\n\xF1\x90\x80aP1V[\x90P\x14a\x0BfW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FDelegationManager.queueWithdrawa`D\x82\x01R\x7Fl: input length mismatch\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\n5V[`\0\x86\x86\x83\x81\x81\x10a\x0BzWa\x0BzaO\xFBV[\x90P` \x02\x81\x01\x90a\x0B\x8C\x91\x90aP\x11V[a\x0B\x9D\x90``\x81\x01\x90`@\x01aF>V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\x0C+W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`H`$\x82\x01R\x7FDelegationManager.queueWithdrawa`D\x82\x01R\x7Fl: must provide valid withdrawal`d\x82\x01Rg address`\xC0\x1B`\x84\x82\x01R`\xA4\x01a\n5V[3`\0\x81\x81R`\x9A` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90a\rJ\x90\x82\x89\x89\x86\x81\x81\x10a\x0C\\Wa\x0C\\aO\xFBV[\x90P` \x02\x81\x01\x90a\x0Cn\x91\x90aP\x11V[a\x0C\x7F\x90``\x81\x01\x90`@\x01aF>V[\x8A\x8A\x87\x81\x81\x10a\x0C\x91Wa\x0C\x91aO\xFBV[\x90P` \x02\x81\x01\x90a\x0C\xA3\x91\x90aP\x11V[a\x0C\xAD\x90\x80aP1V[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8E\x92P\x8D\x91P\x89\x90P\x81\x81\x10a\x0C\xF3Wa\x0C\xF3aO\xFBV[\x90P` \x02\x81\x01\x90a\r\x05\x91\x90aP\x11V[a\r\x13\x90` \x81\x01\x90aP1V[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa(\x02\x92PPPV[\x83\x83\x81Q\x81\x10a\r\\Wa\r\\aO\xFBV[` \x90\x81\x02\x91\x90\x91\x01\x01RP\x80a\rr\x81aP\x90V[\x91PPa\n\x87V[P\x94\x93PPPPV[3`\0\x90\x81R`\x99` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x0E\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R\x7FDelegationManager.registerAsOper`D\x82\x01R\x7Fator: operator has already regis`d\x82\x01Rd\x1D\x19\\\x99Y`\xDA\x1B`\x84\x82\x01R`\xA4\x01a\n5V[a\x0E'3\x84a,<V[`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01Ra\x0EI3\x80\x83`\0a.\xD8V[3`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8E\x84\x85X:#\x10\xD4\x1F|\x82\xB9B}\x0B\xD4\x9B\xADt\xBB\x9C\xFF\x9D4\x02\xA2\x9D\x8F\x9B(\xA0\xE2\x85`@Qa\x0E\x82\x91\x90aP\xABV[`@Q\x80\x91\x03\x90\xA23`\x01`\x01`\xA0\x1B\x03\x16\x7F\x02\xA9\x19\xED\x0E*\xCA\xD1\xDD\x90\xF1~\xF2\xFAJ\xE5F.\xE13\x91p\x03J\x851\xCC\xA4\xB6p\x80\x90\x84\x84`@Qa\x0E\xC5\x92\x91\x90aP\xFDV[`@Q\x80\x91\x03\x90\xA2PPPPV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F&W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0FJ\x91\x90aQ,V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0FzW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n5\x90aQIV[a\x0F\x83\x81a2\x91V[PV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x0F\xE5WP3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14[a\x10\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n5\x90aQ\x93V[a\x10\n\x83a\x14\xC9V[\x15a\x10AW`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R`\x9A` R`@\x90 T\x16a\x106\x81\x85\x85\x85a3\x88V[a\x10?\x81a4\x03V[P[PPPV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\x8EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xB2\x91\x90aQ\xF0V[a\x10\xCEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n5\x90aR\rV[`fT\x81\x81\x16\x14a\x11GW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\n5V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x11\xA5WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x11\xBFWP0;\x15\x80\x15a\x11\xBFWP`\0T`\xFF\x16`\x01\x14[a\x12\"W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\n5V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x12EW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x12O\x83\x83a4\xBDV[a\x12Wa5\xA3V[`\x97Ua\x12c\x84a6:V[\x80\x15a\x10?W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x9B` R`@\x81 Ta\x12\xD3\x85\x82\x86\x86a\x1C\x08V[\x95\x94PPPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x13;WP3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14[a\x13WW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n5\x90aQ\x93V[a\x13`\x83a\x14\xC9V[\x15a\x10AW`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R`\x9A` R`@\x90 T\x16a\x106\x81\x85\x85\x85a6\x8CV[`fT`\x02\x90`\x04\x90\x81\x16\x14\x15a\x13\xB5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n5\x90aO\xC4V[`\x02`\xC9T\x14\x15a\x14\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\n5V[`\x02`\xC9U`\0[\x88\x81\x10\x15a\x14\xB8Wa\x14\xA8\x8A\x8A\x83\x81\x81\x10a\x14-Wa\x14-aO\xFBV[\x90P` \x02\x81\x01\x90a\x14?\x91\x90aRUV[\x89\x89\x84\x81\x81\x10a\x14QWa\x14QaO\xFBV[\x90P` \x02\x81\x01\x90a\x14c\x91\x90aP1V[\x89\x89\x86\x81\x81\x10a\x14uWa\x14uaO\xFBV[\x90P` \x02\x015\x88\x88\x87\x81\x81\x10a\x14\x8EWa\x14\x8EaO\xFBV[\x90P` \x02\x01` \x81\x01\x90a\x14\xA3\x91\x90aRkV[a7\x07V[a\x14\xB1\x81aP\x90V[\x90Pa\x14\x10V[PP`\x01`\xC9UPPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x9A` R`@\x90 T\x16\x15\x15\x90V[a\x14\xF1a=\x81V[a\xC4\xE0\x81\x11\x15a\x15\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`M`$\x82\x01R\x7FDelegationManager.setWithdrawalD`D\x82\x01R\x7FelayBlocks: newWithdrawalDelayBl`d\x82\x01Rl\r\xECmnd\x0E\x8D\xED\xE4\r\r,\xED`\x9B\x1B`\x84\x82\x01R`\xA4\x01a\n5V[`\x9DT`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7FO\xFB\0@\x05t\x14t)\xEE7zV38c!\xE6mE\xD8\xB1Fv\x01K_\xA3\x93\xE6\x1E\x9E\x91\x01`@Q\x80\x91\x03\x90\xA1`\x9DUV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\x08W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16,\x91\x90aQ\xF0V[a\x16HW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n5\x90aR\rV[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[`\0\x81`@Q` \x01a\x16\x9A\x91\x90aR\xFCV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`\0[\x81Q\x81\x10\x15a\x19^W`\0\x82\x82\x81Q\x81\x10a\x16\xD7Wa\x16\xD7aO\xFBV[` \x02` \x01\x01Q\x90P`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xCD)?o\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x170\x91\x90aS\x0FV[`@\x80Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x17NW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17r\x91\x90aS\xBEV[\x91P\x91P\x81\x15a\x19PW`@\x80\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x9F` R\x91\x82 \x80T\x91\x92\x82\x91\x90a\x17\xA9\x83aP\x90V[\x91\x90PUP`\0`@Q\x80`\xE0\x01`@R\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x87`\xA0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x87``\x01Q`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01\x87`\x80\x01Qc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x87`\0\x01Q\x81R` \x01\x87` \x01Q\x81RP\x90P`\0a\x18)\x82a\x16\x87V[`\0\x81\x81R`\x9E` R`@\x90 T\x90\x91P`\xFF\x16\x15a\x18\xBFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R\x7FDelegationManager.migrateQueuedW`D\x82\x01R\x7Fithdrawals: withdrawal already e`d\x82\x01Rdxists`\xD8\x1B`\x84\x82\x01R`\xA4\x01a\n5V[`\0\x81\x81R`\x9E` R`@\x90\x81\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x7F\x90\t\xAB\x15>\x80\x14\xFB\xFB\x02\xF2!\x7F\\\xDEz\xA7\xF9\xADsJ\xE8\\\xA3\xEE?L\xA2\xFD\xD4\x99\xF9\x90a\x19\n\x90\x83\x90\x85\x90aS\xECV[`@Q\x80\x91\x03\x90\xA1`@\x80Q\x86\x81R` \x81\x01\x83\x90R\x7F\xDC\0u\x8Be\xEE\xF7\x1D\xC3x\x0C\x04\xEB\xE3l\xABk\xDB&l:i\x81\x87\xE2\x9E\x0F\r\xCA\x01&0\x91\x01`@Q\x80\x91\x03\x90\xA1PPPP[\x83`\x01\x01\x93PPPPa\x16\xBAV[PPV[`fT`\x02\x90`\x04\x90\x81\x16\x14\x15a\x19\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n5\x90aO\xC4V[`\x02`\xC9T\x14\x15a\x19\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\n5V[`\x02`\xC9Ua\x19\xF0\x86\x86\x86\x86\x86a7\x07V[PP`\x01`\xC9UPPPPV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x99` R`@\x90 T\x16\x15\x15\x90V[a\x1A%a=\x81V[a\x1A/`\0a6:V[V[B\x83` \x01Q\x10\x15a\x1A\xB5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FDelegationManager.delegateToBySi`D\x82\x01R\x7Fgnature: staker signature expire`d\x82\x01R`\x19`\xFA\x1B`\x84\x82\x01R`\xA4\x01a\n5V[`\0`\x9B`\0\x87`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 T\x90P`\0a\x1A\xF1\x87\x83\x88\x88` \x01Qa\x1C\x08V[`\x01`\x01`\xA0\x1B\x03\x88\x16`\0\x90\x81R`\x9B` R`@\x90 `\x01\x84\x01\x90U\x85Q\x90\x91Pa\x1B!\x90\x88\x90\x83\x90a=\xDBV[a\x1B-\x87\x87\x86\x86a.\xD8V[PPPPPPPV[a\x1B?3a\x19\xFDV[a\x1B\xC1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FDelegationManager.updateOperator`D\x82\x01R\x7FMetadataURI: caller must be an o`d\x82\x01Rf82\xB90\xBA7\xB9`\xC9\x1B`\x84\x82\x01R`\xA4\x01a\n5V[3`\x01`\x01`\xA0\x1B\x03\x16\x7F\x02\xA9\x19\xED\x0E*\xCA\xD1\xDD\x90\xF1~\xF2\xFAJ\xE5F.\xE13\x91p\x03J\x851\xCC\xA4\xB6p\x80\x90\x83\x83`@Qa\x1B\xFC\x92\x91\x90aP\xFDV[`@Q\x80\x91\x03\x90\xA2PPV[`@\x80Q\x7F9\x11\x1B\xC4\xA4\xD6\x88\xE1\xF6\x85\x12=t\x97\xD4aSp\x15*\x8E\xE4\xA0Y>d{\xD0j\xD8\xBB\x0B` \x80\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x83\x85\x01R\x85\x16``\x83\x01R`\x80\x82\x01\x86\x90R`\xA0\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\xC0\x90\x92\x01\x90\x92R\x80Q\x91\x01 `\0\x90\x81a\x1C~a&hV[`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x91\x90\x91R`B\x81\x01\x83\x90R`b\x01`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x97\x96PPPPPPPV[`@Qc`\xF4\x06+`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R``\x91\x82\x91`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c`\xF4\x06+\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1DU\x91\x90aT\x05V[`@Qc\x94\xF6I\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R\x91\x92P`\0\x91\x82\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\x94\xF6I\xDD\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\xC5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1D\xED\x91\x90\x81\x01\x90aTyV[\x91P\x91P`\0\x83\x13a\x1E\x04W\x90\x95\x90\x94P\x92PPPV[``\x80\x83Q`\0\x14\x15a\x1E\xBEW`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x90` \x80\x83\x01\x90\x806\x837PP`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x92\x94P\x90P` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90Ps\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x82`\0\x81Q\x81\x10a\x1EyWa\x1EyaO\xFBV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x84\x81`\0\x81Q\x81\x10a\x1E\xADWa\x1E\xADaO\xFBV[` \x02` \x01\x01\x81\x81RPPa lV[\x83Qa\x1E\xCB\x90`\x01aU=V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E\xE2Wa\x1E\xE2aG\x7FV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1F\x0BW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F'Wa\x1F'aG\x7FV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1FPW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x84Q\x81\x10\x15a\x1F\xEAW\x84\x81\x81Q\x81\x10a\x1FqWa\x1FqaO\xFBV[` \x02` \x01\x01Q\x83\x82\x81Q\x81\x10a\x1F\x8BWa\x1F\x8BaO\xFBV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x83\x81\x81Q\x81\x10a\x1F\xBDWa\x1F\xBDaO\xFBV[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a\x1F\xD7Wa\x1F\xD7aO\xFBV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x1FVV[Ps\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x82`\x01\x84Qa \x0F\x91\x90aUUV[\x81Q\x81\x10a \x1FWa \x1FaO\xFBV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x84\x81`\x01\x84Qa O\x91\x90aUUV[\x81Q\x81\x10a _Wa _aO\xFBV[` \x02` \x01\x01\x81\x81RPP[\x90\x97\x90\x96P\x94PPPPPV[`fT`\0\x90`\x01\x90`\x02\x90\x81\x16\x14\x15a \xA5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n5\x90aO\xC4V[a \xAE\x83a\x14\xC9V[a!.W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FDelegationManager.undelegate: st\x90\x82\x01R\x7Faker must be delegated to undele`d\x82\x01Rcgate`\xE0\x1B`\x84\x82\x01R`\xA4\x01a\n5V[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R`\x9A` R`@\x90 T\x16a!R\x84a\x19\xFDV[\x15a!\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FDelegationManager.undelegate: op`D\x82\x01R\x7Ferators cannot be undelegated\0\0\0`d\x82\x01R`\x84\x01a\n5V[`\x01`\x01`\xA0\x1B\x03\x84\x16a\"AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R\x7FDelegationManager.undelegate: ca`D\x82\x01R\x7Fnnot undelegate zero address\0\0\0\0`d\x82\x01R`\x84\x01a\n5V[3`\x01`\x01`\xA0\x1B\x03\x85\x16\x14\x80a\"`WP3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14[\x80a\"\x87WP`\x01`\x01`\xA0\x1B\x03\x81\x81\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T\x163\x14[a\"\xF9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FDelegationManager.undelegate: ca`D\x82\x01R\x7Fller cannot undelegate staker\0\0\0`d\x82\x01R`\x84\x01a\n5V[`\0\x80a#\x05\x86a\x1C\xC1V[\x90\x92P\x90P3`\x01`\x01`\xA0\x1B\x03\x87\x16\x14a#[W\x82`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x7F\xF0\xED\xDF\x07\xE6\xEA\x14\xF3\x88\xB4~\x1E\x94\xA0\xF4d\xEC\xBD\x9E\xEDAq\x13\x0E\x0F\xC0\xE9\x9F\xB4\x03\n\x8A`@Q`@Q\x80\x91\x03\x90\xA3[\x82`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFE\xE3\tf\xA2V\xB7\x1E\x14\xBC\x0E\xBF\xC9C\x15\xE2\x8E\xF4\xA9zq1\xA9\xE2\xB7\xA3\x10\xA7:\xF4Fv`@Q`@Q\x80\x91\x03\x90\xA3`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x9A` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x81Qa#\xD4WP`\0\x93Pa#\xE7\x91PPV[a#\xE1\x86\x84\x88\x85\x85a(\x02V[\x94PPPP[P\x91\x90PV[a#\xF5a=\x81V[`\xA0T`\x01`\x01`\xA0\x1B\x03\x16\x15a$tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FDelegationManager.setStakeRegist`D\x82\x01R\x7Fry: stakeRegistry already set\0\0\0`d\x82\x01R`\x84\x01a\n5V[`\x01`\x01`\xA0\x1B\x03\x81\x16a%\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`H`$\x82\x01R\x7FDelegationManager.setStakeRegist`D\x82\x01R\x7Fry: stakeRegistry cannot be zero`d\x82\x01Rg address`\xC0\x1B`\x84\x82\x01R`\xA4\x01a\n5V[`\xA0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\xCEm\x87@i\xBC\xED\xA1\x86~\xCA\x9C`ck\xBF&.\x15!0Ae\x82s\xD8\x03\xA2\xB6\t\xA5\x1F\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[a\x10A3\x84\x84\x84a.\xD8V[a%j3a\x19\xFDV[a%\xE8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FDelegationManager.modifyOperator`D\x82\x01R\x7FDetails: caller must be an opera`d\x82\x01Rb:7\xB9`\xE9\x1B`\x84\x82\x01R`\xA4\x01a\n5V[a\x0F\x833\x82a,<V[a%\xFAa=\x81V[`\x01`\x01`\xA0\x1B\x03\x81\x16a&_W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\n5V[a\x0F\x83\x81a6:V[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14\x15a&\x99WP`\x97T\x90V[a&\xA1a5\xA3V[\x90P\x90V[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&\xF9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'\x1D\x91\x90aQ,V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a'MW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n5\x90aQIV[`fT\x19\x81\x19`fT\x19\x16\x14a'\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\n5V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x11zV[`\0`\x01`\x01`\xA0\x1B\x03\x86\x16a(\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`P`$\x82\x01R\x7FDelegationManager._removeSharesA`D\x82\x01R\x7FndQueueWithdrawal: staker cannot`d\x82\x01Ro be zero address`\x80\x1B`\x84\x82\x01R`\xA4\x01a\n5V[\x82Qa)#W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`M`$\x82\x01R\x7FDelegationManager._removeSharesA`D\x82\x01R\x7FndQueueWithdrawal: strategies ca`d\x82\x01Rlnnot be empty`\x98\x1B`\x84\x82\x01R`\xA4\x01a\n5V[`\0[\x83Q\x81\x10\x15a+2W`\x01`\x01`\xA0\x1B\x03\x86\x16\x15a)|Wa)|\x86\x88\x86\x84\x81Q\x81\x10a)UWa)UaO\xFBV[` \x02` \x01\x01Q\x86\x85\x81Q\x81\x10a)oWa)oaO\xFBV[` \x02` \x01\x01Qa3\x88V[s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0`\x01`\x01`\xA0\x1B\x03\x16\x84\x82\x81Q\x81\x10a)\xACWa)\xACaO\xFBV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a*uW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xBE\xFF\xBB\x89\x88\x85\x84\x81Q\x81\x10a*\x05Wa*\x05aO\xFBV[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a*>\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a*XW`\0\x80\xFD[PZ\xF1\x15\x80\x15a*lW=`\0\x80>=`\0\xFD[PPPPa+*V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8C\x80\xD4\xE5\x88\x86\x84\x81Q\x81\x10a*\xB7Wa*\xB7aO\xFBV[` \x02` \x01\x01Q\x86\x85\x81Q\x81\x10a*\xD1Wa*\xD1aO\xFBV[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a*\xF7\x93\x92\x91\x90aUlV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a+\x11W`\0\x80\xFD[PZ\xF1\x15\x80\x15a+%W=`\0\x80>=`\0\xFD[PPPP[`\x01\x01a)&V[P`\x01`\x01`\xA0\x1B\x03\x85\x16\x15a+KWa+K\x85a4\x03V[`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x9F` R`@\x81 \x80T\x91\x82\x91\x90a+r\x83aP\x90V[\x91\x90PUP`\0`@Q\x80`\xE0\x01`@R\x80\x89`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x88`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x87`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86\x81R` \x01\x85\x81RP\x90P`\0a+\xDA\x82a\x16\x87V[`\0\x81\x81R`\x9E` R`@\x90\x81\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x90\x91P\x7F\x90\t\xAB\x15>\x80\x14\xFB\xFB\x02\xF2!\x7F\\\xDEz\xA7\xF9\xADsJ\xE8\\\xA3\xEE?L\xA2\xFD\xD4\x99\xF9\x90a,(\x90\x83\x90\x85\x90aS\xECV[`@Q\x80\x91\x03\x90\xA1\x98\x97PPPPPPPPV[`\0a,K` \x83\x01\x83aF>V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a,\xE5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`T`$\x82\x01R\x7FDelegationManager._setOperatorDe`D\x82\x01R\x7Ftails: cannot set `earningsRecei`d\x82\x01Rsver` to zero address``\x1B`\x84\x82\x01R`\xA4\x01a\n5V[b\x13\xC6\x80a,\xF9``\x83\x01`@\x84\x01aU\x90V[c\xFF\xFF\xFF\xFF\x16\x11\x15a-\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`l`$\x82\x01R\x7FDelegationManager._setOperatorDe`D\x82\x01R\x7Ftails: stakerOptOutWindowBlocks `d\x82\x01R\x7Fcannot be > MAX_STAKER_OPT_OUT_W`\x84\x82\x01RkINDOW_BLOCKS`\xA0\x1B`\xA4\x82\x01R`\xC4\x01a\n5V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x99` R`@\x90\x81\x90 `\x01\x01T`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x90a-\xEA\x90``\x84\x01\x90\x84\x01aU\x90V[c\xFF\xFF\xFF\xFF\x16\x10\x15a.\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`S`$\x82\x01R\x7FDelegationManager._setOperatorDe`D\x82\x01R\x7Ftails: stakerOptOutWindowBlocks `d\x82\x01Rr\x18\xD8[\x9B\x9B\xDD\x08\x18\x99H\x19\x19X\xDC\x99X\\\xD9Y`j\x1B`\x84\x82\x01R`\xA4\x01a\n5V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x99` R`@\x90 \x81\x90a.\xA4\x82\x82aU\xCDV[PP`@Q3\x90\x7F\xFE\xBE\\\xD2K,\xBC{\x06[\x9D\x0F\xDE\xB9\x04F\x1EJ\xFC\xFFW\xDDW\xAC\xDA\x1Ex2\x03\x1B\xA7\xAC\x90a\x1B\xFC\x90\x84\x90aP\xABV[`fT`\0\x90`\x01\x90\x81\x16\x14\x15a/\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n5\x90aO\xC4V[a/\n\x85a\x14\xC9V[\x15a/\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FDelegationManager._delegate: sta`D\x82\x01R\x7Fker is already actively delegate`d\x82\x01R`\x19`\xFA\x1B`\x84\x82\x01R`\xA4\x01a\n5V[a/\x90\x84a\x19\xFDV[a0\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R\x7FDelegationManager._delegate: ope`D\x82\x01R\x7Frator is not registered in Eigen`d\x82\x01Rd&0\xBC\xB2\xB9`\xD9\x1B`\x84\x82\x01R`\xA4\x01a\n5V[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T\x16\x80\x15\x80\x15\x90a0FWP3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14\x15[\x80\x15a0[WP3`\x01`\x01`\xA0\x1B\x03\x86\x16\x14\x15[\x15a1\xC8WB\x84` \x01Q\x10\x15a0\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FDelegationManager._delegate: app`D\x82\x01R\x7Frover signature expired\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\n5V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x9C` \x90\x81R`@\x80\x83 \x86\x84R\x90\x91R\x90 T`\xFF\x16\x15a1tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FDelegationManager._delegate: app`D\x82\x01R\x7FroverSalt already spent\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\n5V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x9C` \x90\x81R`@\x80\x83 \x86\x84R\x82R\x82 \x80T`\xFF\x19\x16`\x01\x17\x90U\x85\x01Qa1\xB5\x90\x88\x90\x88\x90\x85\x90\x88\x90a\tGV[\x90Pa1\xC6\x82\x82\x87`\0\x01Qa=\xDBV[P[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\0\x81\x81R`\x9A` R`@\x80\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x94\x8A\x16\x94\x85\x17\x90UQ\x7F\xC3\xEE\x9F._\xDA\x98\xE8\x06j\x1Ft[-\xF9(_Ao\xE9\x8C\xF2U\x9C\xD2\x14\x84\xB3\xD8t3\x04\x91\x90\xA3`\0\x80a2'\x88a\x1C\xC1V[\x91P\x91P`\0[\x82Q\x81\x10\x15a2}Wa2u\x88\x8A\x85\x84\x81Q\x81\x10a2NWa2NaO\xFBV[` \x02` \x01\x01Q\x85\x85\x81Q\x81\x10a2hWa2haO\xFBV[` \x02` \x01\x01Qa6\x8CV[`\x01\x01a2.V[Pa2\x87\x87a4\x03V[PPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16a3\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\n5V[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x98` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a3\xBF\x90\x84\x90aUUV[\x92PP\x81\x90UP\x83`\x01`\x01`\xA0\x1B\x03\x16\x7Fi\t`\x007\xB7]{G3\xAE\xDD\x81TB\xB5\xEC\x01\x8A\x82wQ\xC82\xAA\xFFd\xEB\xA5\xD6\xD2\xDD\x84\x84\x84`@Qa\x0E\xC5\x93\x92\x91\x90aUlV[`\xA0T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x0F\x83W`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x81\x81`\0\x81Q\x81\x10a4JWa4JaO\xFBV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16` \x91\x82\x02\x92\x90\x92\x01\x01R`\xA0T`@Qc\xCE\x97~\xC3`\xE0\x1B\x81R\x91\x16\x90c\xCE\x97~\xC3\x90a4\x87\x90\x84\x90`\x04\x01aV0V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a4\xA1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a4\xB5W=`\0\x80>=`\0\xFD[PPPPPPV[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a4\xDEWP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a5`W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\n5V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a\x19^\x82a2\x91V[`@\x80Q\x80\x82\x01\x82R`\n\x81Ri\"\xB4\xB3\xB2\xB7&0\xBC\xB2\xB9`\xB1\x1B` \x91\x82\x01R\x81Q\x7F\x8C\xAD\x95h{\xA8,,\xE5\x0Et\xF7\xB7Td^Q\x17\xC3\xA5\xBE\xC8\x15\x1C\x07&\xD5\x85y\x80\xA8f\x81\x83\x01R\x7Fq\xB6%\xCF\xADD\xBA\xC6;\x13\xDB\xA0\x7F.\x1D`\x84\xEE\x04\xB6\xF8u!\x01\xEC\xE6\x12mXN\xE6\xEA\x81\x84\x01RF``\x82\x01R0`\x80\x80\x83\x01\x91\x90\x91R\x83Q\x80\x83\x03\x90\x91\x01\x81R`\xA0\x90\x91\x01\x90\x92R\x81Q\x91\x01 \x90V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x98` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a6\xC3\x90\x84\x90aU=V[\x92PP\x81\x90UP\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\x1E\xC0B\xC9e\xE2\xED\xD7\x10{Q\x18\x8E\xE0\xF3\x83\xE2.v\x17\x90A\xAB:\x9D\x18\xFF\x15\x14\x05\x16l\x84\x84\x84`@Qa\x0E\xC5\x93\x92\x91\x90aUlV[`\0a7\x15a\x05\xE9\x87aVqV[`\0\x81\x81R`\x9E` R`@\x90 T\x90\x91P`\xFF\x16a7\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R`\0\x80Q` aW!\x839\x81Q\x91R`D\x82\x01R\x7FAction: action is not in queue\0\0`d\x82\x01R`\x84\x01a\n5V[`\x9DTC\x90a7\x9F`\xA0\x89\x01`\x80\x8A\x01aU\x90V[c\xFF\xFF\xFF\xFF\x16a7\xAF\x91\x90aU=V[\x11\x15a87W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`W`$\x82\x01R`\0\x80Q` aW!\x839\x81Q\x91R`D\x82\x01R\x7FAction: withdrawalDelayBlocks pe`d\x82\x01R\x7Friod has not yet passed\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\n5V[a8G``\x87\x01`@\x88\x01aF>V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a8\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`K`$\x82\x01R`\0\x80Q` aW!\x839\x81Q\x91R`D\x82\x01R\x7FAction: only withdrawer can comp`d\x82\x01Rj62\xBA2\x900\xB1\xBA4\xB7\xB7`\xA9\x1B`\x84\x82\x01R`\xA4\x01a\n5V[\x81\x15a9FWa8\xE2`\xA0\x87\x01\x87aP1V[\x85\x14\x90Pa9FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R`\0\x80Q` aW!\x839\x81Q\x91R`D\x82\x01R\x7FAction: input length mismatch\0\0\0`d\x82\x01R`\x84\x01a\n5V[`\0\x81\x81R`\x9E` R`@\x90 \x80T`\xFF\x19\x16\x90U\x81\x15a:\x1CW`\0[a9r`\xA0\x88\x01\x88aP1V[\x90P\x81\x10\x15a:\x16Wa:\x0Ea9\x8B` \x89\x01\x89aF>V[3a9\x99`\xA0\x8B\x01\x8BaP1V[\x85\x81\x81\x10a9\xA9Wa9\xA9aO\xFBV[\x90P` \x02\x01` \x81\x01\x90a9\xBE\x91\x90aF>V[a9\xCB`\xC0\x8C\x01\x8CaP1V[\x86\x81\x81\x10a9\xDBWa9\xDBaO\xFBV[\x90P` \x02\x015\x8A\x8A\x87\x81\x81\x10a9\xF4Wa9\xF4aO\xFBV[\x90P` \x02\x01` \x81\x01\x90a:\t\x91\x90aF>V[a?\x95V[`\x01\x01a9eV[Pa=FV[3`\0\x90\x81R`\x9A` R`@\x81 T`\x01`\x01`\xA0\x1B\x03\x16\x90[a:D`\xA0\x89\x01\x89aP1V[\x90P\x81\x10\x15a=:Ws\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0a:o`\xA0\x8A\x01\x8AaP1V[\x83\x81\x81\x10a:\x7FWa:\x7FaO\xFBV[\x90P` \x02\x01` \x81\x01\x90a:\x94\x91\x90aF>V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a;\xEDW`\0a:\xB2` \x8A\x01\x8AaF>V[\x90P`\0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\x0E\x81\x07<\x83a:\xF3`\xC0\x8E\x01\x8EaP1V[\x87\x81\x81\x10a;\x03Wa;\x03aO\xFBV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x87\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x94\x16`\x04\x85\x01R` \x02\x91\x90\x91\x015`$\x83\x01RP`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a;WW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a;{\x91\x90aT\x05V[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R`\x9A` R`@\x90 T\x91\x92P\x16\x80\x15a;\xE5Wa;\xDC\x81\x84a;\xB1`\xA0\x8F\x01\x8FaP1V[\x88\x81\x81\x10a;\xC1Wa;\xC1aO\xFBV[\x90P` \x02\x01` \x81\x01\x90a;\xD6\x91\x90aF>V[\x85a6\x8CV[a;\xE5\x81a4\x03V[PPPa=2V[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16cP\xFFr%3a<*`\xA0\x8C\x01\x8CaP1V[\x85\x81\x81\x10a<:Wa<:aO\xFBV[\x90P` \x02\x01` \x81\x01\x90a<O\x91\x90aF>V[a<\\`\xC0\x8D\x01\x8DaP1V[\x86\x81\x81\x10a<lWa<laO\xFBV[\x90P` \x02\x015`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a<\x91\x93\x92\x91\x90aUlV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a<\xABW`\0\x80\xFD[PZ\xF1\x15\x80\x15a<\xBFW=`\0\x80>=`\0\xFD[PPPP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a=2Wa=2\x823a<\xE4`\xA0\x8C\x01\x8CaP1V[\x85\x81\x81\x10a<\xF4Wa<\xF4aO\xFBV[\x90P` \x02\x01` \x81\x01\x90a=\t\x91\x90aF>V[a=\x16`\xC0\x8D\x01\x8DaP1V[\x86\x81\x81\x10a=&Wa=&aO\xFBV[\x90P` \x02\x015a6\x8CV[`\x01\x01a:7V[Pa=D\x81a4\x03V[P[`@Q\x81\x81R\x7F\xC9p\x98\xC2\xF6X\x80\x0BM\xF2\x90\x01R\x7Fs$\xBC\xDF\xFC\xF6\xE8u\x1Ai\x9A\xB9 \xA1\xEC\xED[\x1D\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPPPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1A/W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\n5V[`\x01`\x01`\xA0\x1B\x03\x83\x16;\x15a>\xF5W`@Qc\x0B\x13]?`\xE1\x1B\x80\x82R\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\x16&\xBA~\x90a>\x1B\x90\x86\x90\x86\x90`\x04\x01aV\x83V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a>8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a>\\\x91\x90aV\xE0V[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14a\x10AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`S`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: ERC1271 signature `d\x82\x01Rr\x1D\x99\\\x9AY\x9AX\xD8]\x1A[\xDB\x88\x19\x98Z[\x19Y`j\x1B`\x84\x82\x01R`\xA4\x01a\n5V[\x82`\x01`\x01`\xA0\x1B\x03\x16a?\t\x83\x83a@\xDAV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x10AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: signature not from`d\x82\x01Rf\x109\xB4\xB3\xB72\xB9`\xC9\x1B`\x84\x82\x01R`\xA4\x01a\n5V[`\x01`\x01`\xA0\x1B\x03\x83\x16s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x14\x15a@@W`@Qb8{\x13`\xE8\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c8{\x13\0\x90a@\t\x90\x88\x90\x88\x90\x87\x90`\x04\x01aUlV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a@#W`\0\x80\xFD[PZ\xF1\x15\x80\x15a@7W=`\0\x80>=`\0\xFD[PPPPa@\xD3V[`@Qc\xC6\x08\xC7\xF3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x84\x81\x16`$\x83\x01R`D\x82\x01\x84\x90R\x82\x81\x16`d\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xC6\x08\xC7\xF3\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a@\xBAW`\0\x80\xFD[PZ\xF1\x15\x80\x15a@\xCEW=`\0\x80>=`\0\xFD[PPPP[PPPPPV[`\0\x80`\0a@\xE9\x85\x85a@\xFEV[\x91P\x91Pa@\xF6\x81aAnV[P\x93\x92PPPV[`\0\x80\x82Q`A\x14\x15aA5W` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1AaA)\x87\x82\x85\x85aC)V[\x94P\x94PPPPaAgV[\x82Q`@\x14\x15aA_W` \x83\x01Q`@\x84\x01QaAT\x86\x83\x83aD\x16V[\x93P\x93PPPaAgV[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0\x81`\x04\x81\x11\x15aA\x82WaA\x82aW\nV[\x14\x15aA\x8BWPV[`\x01\x81`\x04\x81\x11\x15aA\x9FWaA\x9FaW\nV[\x14\x15aA\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\n5V[`\x02\x81`\x04\x81\x11\x15aB\x01WaB\x01aW\nV[\x14\x15aBOW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\n5V[`\x03\x81`\x04\x81\x11\x15aBcWaBcaW\nV[\x14\x15aB\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\n5V[`\x04\x81`\x04\x81\x11\x15aB\xD0WaB\xD0aW\nV[\x14\x15a\x0F\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 'v' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\n5V[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15aC`WP`\0\x90P`\x03aD\rV[\x84`\xFF\x16`\x1B\x14\x15\x80\x15aCxWP\x84`\xFF\x16`\x1C\x14\x15[\x15aC\x89WP`\0\x90P`\x04aD\rV[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15aC\xDDW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16aD\x06W`\0`\x01\x92P\x92PPaD\rV[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x80`\x01`\x01`\xFF\x1B\x03\x83\x16\x81aD3`\xFF\x86\x90\x1C`\x1BaU=V[\x90PaDA\x87\x82\x88\x85aC)V[\x93P\x93PPP\x93P\x93\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0F\x83W`\0\x80\xFD[\x805aDo\x81aDOV[\x91\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15aD\x8CW`\0\x80\xFD[\x855aD\x97\x81aDOV[\x94P` \x86\x015aD\xA7\x81aDOV[\x93P`@\x86\x015aD\xB7\x81aDOV[\x94\x97\x93\x96P\x93\x94``\x81\x015\x94P`\x80\x015\x92\x91PPV[`\0\x80\x83`\x1F\x84\x01\x12aD\xE1W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aD\xF8W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15aAgW`\0\x80\xFD[`\0\x80` \x83\x85\x03\x12\x15aE&W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aE<W`\0\x80\xFD[aEH\x85\x82\x86\x01aD\xCFV[\x90\x96\x90\x95P\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aE\x8CW\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aEpV[P\x90\x96\x95PPPPPPV[`\0``\x82\x84\x03\x12\x15a#\xE7W`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12aE\xBCW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aE\xD3W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aAgW`\0\x80\xFD[`\0\x80`\0`\x80\x84\x86\x03\x12\x15aF\0W`\0\x80\xFD[aF\n\x85\x85aE\x98V[\x92P``\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aF%W`\0\x80\xFD[aF1\x86\x82\x87\x01aE\xAAV[\x94\x97\x90\x96P\x93\x94PPPPV[`\0` \x82\x84\x03\x12\x15aFPW`\0\x80\xFD[\x815aF[\x81aDOV[\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15aFwW`\0\x80\xFD[\x835aF\x82\x81aDOV[\x92P` \x84\x015aF\x92\x81aDOV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0` \x82\x84\x03\x12\x15aF\xB5W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80`\x80\x89\x8B\x03\x12\x15aF\xD8W`\0\x80\xFD[\x885`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aF\xEFW`\0\x80\xFD[aF\xFB\x8C\x83\x8D\x01aD\xCFV[\x90\x9AP\x98P` \x8B\x015\x91P\x80\x82\x11\x15aG\x14W`\0\x80\xFD[aG \x8C\x83\x8D\x01aD\xCFV[\x90\x98P\x96P`@\x8B\x015\x91P\x80\x82\x11\x15aG9W`\0\x80\xFD[aGE\x8C\x83\x8D\x01aD\xCFV[\x90\x96P\x94P``\x8B\x015\x91P\x80\x82\x11\x15aG^W`\0\x80\xFD[PaGk\x8B\x82\x8C\x01aD\xCFV[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aG\xB7WaG\xB7aG\x7FV[`@R\x90V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aG\xB7WaG\xB7aG\x7FV[`@Q`\xC0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aG\xB7WaG\xB7aG\x7FV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aH)WaH)aG\x7FV[`@R\x91\x90PV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0F\x83W`\0\x80\xFD[\x805aDo\x81aH1V[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15aHgWaHgaG\x7FV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12aH\x82W`\0\x80\xFD[\x815` aH\x97aH\x92\x83aHNV[aH\x01V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aH\xB6W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aH\xDAW\x805aH\xCD\x81aDOV[\x83R\x91\x83\x01\x91\x83\x01aH\xBAV[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12aH\xF6W`\0\x80\xFD[\x815` aI\x06aH\x92\x83aHNV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aI%W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aH\xDAW\x805\x83R\x91\x83\x01\x91\x83\x01aI)V[`\0`\xE0\x82\x84\x03\x12\x15aIRW`\0\x80\xFD[aIZaG\x95V[\x90PaIe\x82aDdV[\x81RaIs` \x83\x01aDdV[` \x82\x01RaI\x84`@\x83\x01aDdV[`@\x82\x01R``\x82\x015``\x82\x01RaI\x9F`\x80\x83\x01aHCV[`\x80\x82\x01R`\xA0\x82\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aI\xBEW`\0\x80\xFD[aI\xCA\x85\x83\x86\x01aHqV[`\xA0\x84\x01R`\xC0\x84\x015\x91P\x80\x82\x11\x15aI\xE3W`\0\x80\xFD[PaI\xF0\x84\x82\x85\x01aH\xE5V[`\xC0\x83\x01RP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15aJ\x0EW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aJ$W`\0\x80\xFD[aJ0\x84\x82\x85\x01aI@V[\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15aJJW`\0\x80\xFD[\x815`\xFF\x81\x16\x81\x14aF[W`\0\x80\xFD[`\0`@\x82\x84\x03\x12\x15aJmW`\0\x80\xFD[aJuaG\xBDV[\x90P\x815aJ\x82\x81aDOV[\x81R` \x82\x015k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14aJ\xA3W`\0\x80\xFD[` \x82\x01R\x92\x91PPV[`\0` \x80\x83\x85\x03\x12\x15aJ\xC1W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aJ\xD8W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12aJ\xECW`\0\x80\xFD[\x815aJ\xFAaH\x92\x82aHNV[\x81\x81R`\x05\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x84\x81\x01\x90\x88\x83\x11\x15aK\x19W`\0\x80\xFD[\x85\x85\x01[\x83\x81\x10\x15aK\xF3W\x805\x85\x81\x11\x15aK5W`\0\x80\x81\xFD[\x86\x01`\xE0\x81\x8C\x03`\x1F\x19\x01\x81\x13\x15aKMW`\0\x80\x81\xFD[aKUaG\xDFV[\x89\x83\x015\x88\x81\x11\x15aKgW`\0\x80\x81\xFD[aKu\x8E\x8C\x83\x87\x01\x01aHqV[\x82RP`@\x80\x84\x015\x89\x81\x11\x15aK\x8CW`\0\x80\x81\xFD[aK\x9A\x8F\x8D\x83\x88\x01\x01aH\xE5V[\x8C\x84\x01RP``aK\xAC\x81\x86\x01aDdV[\x82\x84\x01R`\x80\x91PaK\xC0\x8F\x83\x87\x01aJ[V[\x90\x83\x01RaK\xD0`\xC0\x85\x01aHCV[\x90\x82\x01RaK\xDF\x83\x83\x01aDdV[`\xA0\x82\x01R\x85RPP\x91\x86\x01\x91\x86\x01aK\x1DV[P\x98\x97PPPPPPPPV[\x80\x15\x15\x81\x14a\x0F\x83W`\0\x80\xFD[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15aL&W`\0\x80\xFD[\x855`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aL=W`\0\x80\xFD[\x90\x87\x01\x90`\xE0\x82\x8A\x03\x12\x15aLQW`\0\x80\xFD[\x90\x95P` \x87\x015\x90\x80\x82\x11\x15aLgW`\0\x80\xFD[PaLt\x88\x82\x89\x01aD\xCFV[\x90\x95P\x93PP`@\x86\x015\x91P``\x86\x015aL\x8F\x81aL\0V[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x80`@\x83\x85\x03\x12\x15aL\xB0W`\0\x80\xFD[\x825aL\xBB\x81aDOV[\x91P` \x83\x015aL\xCB\x81aDOV[\x80\x91PP\x92P\x92\x90PV[`\0`@\x82\x84\x03\x12\x15aL\xE8W`\0\x80\xFD[aL\xF0aG\xBDV[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aM\tW`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12aM\x1DW`\0\x80\xFD[\x815` \x82\x82\x11\x15aM1WaM1aG\x7FV[aMC`\x1F\x83\x01`\x1F\x19\x16\x82\x01aH\x01V[\x92P\x81\x83R\x86\x81\x83\x86\x01\x01\x11\x15aMYW`\0\x80\xFD[\x81\x81\x85\x01\x82\x85\x017`\0\x81\x83\x85\x01\x01R\x82\x85R\x80\x86\x015\x81\x86\x01RPPPP\x92\x91PPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15aM\x96W`\0\x80\xFD[\x855aM\xA1\x81aDOV[\x94P` \x86\x015aM\xB1\x81aDOV[\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aM\xCDW`\0\x80\xFD[aM\xD9\x89\x83\x8A\x01aL\xD6V[\x94P``\x88\x015\x91P\x80\x82\x11\x15aM\xEFW`\0\x80\xFD[PaM\xFC\x88\x82\x89\x01aL\xD6V[\x95\x98\x94\x97P\x92\x95`\x80\x015\x93\x92PPPV[`\0\x80` \x83\x85\x03\x12\x15aN!W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aN7W`\0\x80\xFD[aEH\x85\x82\x86\x01aE\xAAV[`\0\x80`@\x83\x85\x03\x12\x15aNVW`\0\x80\xFD[\x825aNa\x81aDOV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aN\x85W`\0\x80\xFD[\x845aN\x90\x81aDOV[\x93P` \x85\x015\x92P`@\x85\x015aN\xA7\x81aDOV[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aN\xF0W\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aN\xCBV[P\x94\x95\x94PPPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aN\xF0W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aO\x0FV[`@\x81R`\0aO>`@\x83\x01\x85aN\xB7V[\x82\x81\x03` \x84\x01Ra\x12\xD3\x81\x85aN\xFBV[`\0\x80`\0``\x84\x86\x03\x12\x15aOeW`\0\x80\xFD[\x835aOp\x81aDOV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aO\x8BW`\0\x80\xFD[aO\x97\x86\x82\x87\x01aL\xD6V[\x92PP`@\x84\x015\x90P\x92P\x92P\x92V[`\0``\x82\x84\x03\x12\x15aO\xBAW`\0\x80\xFD[aF[\x83\x83aE\x98V[` \x80\x82R`\x19\x90\x82\x01R\x7FPausable: index is paused\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x825`^\x19\x836\x03\x01\x81\x12aP'W`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aPHW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aPbW`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15aAgW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15aP\xA4WaP\xA4aPzV[P`\x01\x01\x90V[``\x81\x01\x825aP\xBA\x81aDOV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x84\x015\x90aP\xD6\x82aDOV[\x16` \x83\x01R`@\x83\x015aP\xEA\x81aH1V[c\xFF\xFF\xFF\xFF\x81\x16`@\x84\x01RP\x92\x91PPV[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017`\0\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x91\x90PV[`\0` \x82\x84\x03\x12\x15aQ>W`\0\x80\xFD[\x81QaF[\x81aDOV[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`7\x90\x82\x01R\x7FDelegationManager: onlyStrategyM`@\x82\x01R\x7FanagerOrEigenPodManager\0\0\0\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15aR\x02W`\0\x80\xFD[\x81QaF[\x81aL\0V[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[`\0\x825`\xDE\x19\x836\x03\x01\x81\x12aP'W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aR}W`\0\x80\xFD[\x815aF[\x81aL\0V[`\0`\x01\x80`\xA0\x1B\x03\x80\x83Q\x16\x84R\x80` \x84\x01Q\x16` \x85\x01R\x80`@\x84\x01Q\x16`@\x85\x01RP``\x82\x01Q``\x84\x01Rc\xFF\xFF\xFF\xFF`\x80\x83\x01Q\x16`\x80\x84\x01R`\xA0\x82\x01Q`\xE0`\xA0\x85\x01RaR\xE3`\xE0\x85\x01\x82aN\xB7V[\x90P`\xC0\x83\x01Q\x84\x82\x03`\xC0\x86\x01Ra\x12\xD3\x82\x82aN\xFBV[` \x81R`\0aF[` \x83\x01\x84aR\x88V[` \x81R`\0\x82Q`\xE0` \x84\x01RaS,a\x01\0\x84\x01\x82aN\xB7V[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01RaSI\x82\x82aN\xFBV[\x91PP`@\x84\x01Q`\x01\x80`\xA0\x1B\x03\x80\x82\x16``\x86\x01R``\x86\x01Q\x91P\x80\x82Q\x16`\x80\x86\x01RPk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16`\xA0\x85\x01RP`\x80\x84\x01QaS\xA1`\xC0\x85\x01\x82c\xFF\xFF\xFF\xFF\x16\x90RV[P`\xA0\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16`\xE0\x85\x01RP\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15aS\xD1W`\0\x80\xFD[\x82QaS\xDC\x81aL\0V[` \x93\x90\x93\x01Q\x92\x94\x92\x93PPPV[\x82\x81R`@` \x82\x01R`\0aJ0`@\x83\x01\x84aR\x88V[`\0` \x82\x84\x03\x12\x15aT\x17W`\0\x80\xFD[PQ\x91\x90PV[`\0\x82`\x1F\x83\x01\x12aT/W`\0\x80\xFD[\x81Q` aT?aH\x92\x83aHNV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aT^W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aH\xDAW\x80Q\x83R\x91\x83\x01\x91\x83\x01aTbV[`\0\x80`@\x83\x85\x03\x12\x15aT\x8CW`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aT\xA3W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12aT\xB7W`\0\x80\xFD[\x81Q` aT\xC7aH\x92\x83aHNV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15aT\xE6W`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15aU\rW\x85QaT\xFE\x81aDOV[\x82R\x94\x82\x01\x94\x90\x82\x01\x90aT\xEBV[\x91\x88\x01Q\x91\x96P\x90\x93PPP\x80\x82\x11\x15aU&W`\0\x80\xFD[PaU3\x85\x82\x86\x01aT\x1EV[\x91PP\x92P\x92\x90PV[`\0\x82\x19\x82\x11\x15aUPWaUPaPzV[P\x01\x90V[`\0\x82\x82\x10\x15aUgWaUgaPzV[P\x03\x90V[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[`\0` \x82\x84\x03\x12\x15aU\xA2W`\0\x80\xFD[\x815aF[\x81aH1V[\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\x815aU\xD8\x81aDOV[aU\xE2\x81\x83aU\xADV[P`\x01\x81\x01` \x83\x015aU\xF5\x81aDOV[aU\xFF\x81\x83aU\xADV[P`@\x83\x015aV\x0E\x81aH1V[\x81Tc\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x16`\xA0\x91\x90\x91\x1Bc\xFF\xFF\xFF\xFF`\xA0\x1B\x16\x17\x90UPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aE\x8CW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aVLV[`\0aV}6\x83aI@V[\x92\x91PPV[\x82\x81R`\0` `@\x81\x84\x01R\x83Q\x80`@\x85\x01R`\0[\x81\x81\x10\x15aV\xB7W\x85\x81\x01\x83\x01Q\x85\x82\x01``\x01R\x82\x01aV\x9BV[\x81\x81\x11\x15aV\xC9W`\0``\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01``\x01\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15aV\xF2W`\0\x80\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14aF[W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD\xFEDelegationManager.completeQueued\xA2dipfsX\"\x12 2\xCC\xFB\"0E\x9C7\x1A\xBA\xCB\xD0\xDA\x01\x9C\xD5o%V\x8D\xE7~5!VT\x8F@R\xCA\xBE\xD7dsolcC\0\x08\x0C\x003";
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
        ///Calls the contract's `setStakeRegistry` (0xe3b05f2f) function
        pub fn set_stake_registry(
            &self,
            stake_registry: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([227, 176, 95, 47], stake_registry)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setWithdrawalDelayBlocks` (0x4d50f9a4) function
        pub fn set_withdrawal_delay_blocks(
            &self,
            new_withdrawal_delay_blocks: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([77, 80, 249, 164], new_withdrawal_delay_blocks)
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
        ///Calls the contract's `stakeRegistry` (0x68304835) function
        pub fn stake_registry(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([104, 48, 72, 53], ())
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
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
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
        ///Calls the contract's `withdrawalDelayBlocks` (0x50f73e7c) function
        pub fn withdrawal_delay_blocks(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([80, 247, 62, 124], ())
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
        ///Gets the contract's `StakeRegistrySet` event
        pub fn stake_registry_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StakeRegistrySetFilter,
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
    #[ethevent(name = "StakeRegistrySet", abi = "StakeRegistrySet(address)")]
    pub struct StakeRegistrySetFilter {
        pub stake_registry: ::ethers::core::types::Address,
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
        InitializedFilter(InitializedFilter),
        OperatorDetailsModifiedFilter(OperatorDetailsModifiedFilter),
        OperatorMetadataURIUpdatedFilter(OperatorMetadataURIUpdatedFilter),
        OperatorRegisteredFilter(OperatorRegisteredFilter),
        OperatorSharesDecreasedFilter(OperatorSharesDecreasedFilter),
        OperatorSharesIncreasedFilter(OperatorSharesIncreasedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PausedFilter(PausedFilter),
        PauserRegistrySetFilter(PauserRegistrySetFilter),
        StakeRegistrySetFilter(StakeRegistrySetFilter),
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
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(DelegationManagerEvents::InitializedFilter(decoded));
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
            if let Ok(decoded) = StakeRegistrySetFilter::decode_log(log) {
                return Ok(DelegationManagerEvents::StakeRegistrySetFilter(decoded));
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
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::StakeRegistrySetFilter(element) => {
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
    impl ::core::convert::From<InitializedFilter> for DelegationManagerEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
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
    impl ::core::convert::From<StakeRegistrySetFilter> for DelegationManagerEvents {
        fn from(value: StakeRegistrySetFilter) -> Self {
            Self::StakeRegistrySetFilter(value)
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
        Hash
    )]
    #[ethcall(name = "initialize", abi = "initialize(address,address,uint256)")]
    pub struct InitializeCall {
        pub initial_owner: ::ethers::core::types::Address,
        pub pauser_registry: ::ethers::core::types::Address,
        pub initial_paused_status: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `setStakeRegistry` function with signature `setStakeRegistry(address)` and selector `0xe3b05f2f`
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
    #[ethcall(name = "setStakeRegistry", abi = "setStakeRegistry(address)")]
    pub struct SetStakeRegistryCall {
        pub stake_registry: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setWithdrawalDelayBlocks` function with signature `setWithdrawalDelayBlocks(uint256)` and selector `0x4d50f9a4`
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
        name = "setWithdrawalDelayBlocks",
        abi = "setWithdrawalDelayBlocks(uint256)"
    )]
    pub struct SetWithdrawalDelayBlocksCall {
        pub new_withdrawal_delay_blocks: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `stakeRegistry` function with signature `stakeRegistry()` and selector `0x68304835`
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
    #[ethcall(name = "stakeRegistry", abi = "stakeRegistry()")]
    pub struct StakeRegistryCall;
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
        IncreaseDelegatedShares(IncreaseDelegatedSharesCall),
        Initialize(InitializeCall),
        IsDelegated(IsDelegatedCall),
        IsOperator(IsOperatorCall),
        MigrateQueuedWithdrawals(MigrateQueuedWithdrawalsCall),
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
        SetStakeRegistry(SetStakeRegistryCall),
        SetWithdrawalDelayBlocks(SetWithdrawalDelayBlocksCall),
        Slasher(SlasherCall),
        StakeRegistry(StakeRegistryCall),
        StakerNonce(StakerNonceCall),
        StakerOptOutWindowBlocks(StakerOptOutWindowBlocksCall),
        StrategyManager(StrategyManagerCall),
        TransferOwnership(TransferOwnershipCall),
        Undelegate(UndelegateCall),
        Unpause(UnpauseCall),
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
            if let Ok(decoded) = <SetStakeRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetStakeRegistry(decoded));
            }
            if let Ok(decoded) = <SetWithdrawalDelayBlocksCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetWithdrawalDelayBlocks(decoded));
            }
            if let Ok(decoded) = <SlasherCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Slasher(decoded));
            }
            if let Ok(decoded) = <StakeRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StakeRegistry(decoded));
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
                Self::SetStakeRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetWithdrawalDelayBlocks(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Slasher(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StakeRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
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
                Self::SetStakeRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetWithdrawalDelayBlocks(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Slasher(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakerNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakerOptOutWindowBlocks(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StrategyManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::Undelegate(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unpause(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<SetStakeRegistryCall> for DelegationManagerCalls {
        fn from(value: SetStakeRegistryCall) -> Self {
            Self::SetStakeRegistry(value)
        }
    }
    impl ::core::convert::From<SetWithdrawalDelayBlocksCall> for DelegationManagerCalls {
        fn from(value: SetWithdrawalDelayBlocksCall) -> Self {
            Self::SetWithdrawalDelayBlocks(value)
        }
    }
    impl ::core::convert::From<SlasherCall> for DelegationManagerCalls {
        fn from(value: SlasherCall) -> Self {
            Self::Slasher(value)
        }
    }
    impl ::core::convert::From<StakeRegistryCall> for DelegationManagerCalls {
        fn from(value: StakeRegistryCall) -> Self {
            Self::StakeRegistry(value)
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
    ///Container type for all return fields from the `stakeRegistry` function with signature `stakeRegistry()` and selector `0x68304835`
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
    pub struct StakeRegistryReturn(pub ::ethers::core::types::Address);
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
    pub struct UndelegateReturn(pub [u8; 32]);
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
