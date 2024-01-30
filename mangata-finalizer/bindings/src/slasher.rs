pub use slasher::*;
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
pub mod slasher {
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
                        name: ::std::borrow::ToOwned::to_owned("_delegation"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IDelegationManager",),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("canSlash"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("canSlash"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("toBeSlashed"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("slashingContract"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("canWithdraw"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("canWithdraw"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("withdrawalStartBlock",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("middlewareTimesIndex",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("contractCanSlashOperatorUntilBlock"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "contractCanSlashOperatorUntilBlock",
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
                                name: ::std::borrow::ToOwned::to_owned("serviceContract"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                    ::std::borrow::ToOwned::to_owned("freezeOperator"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("freezeOperator"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("toBeFrozen"),
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
                    ::std::borrow::ToOwned::to_owned("getCorrectValueForInsertAfter"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getCorrectValueForInsertAfter",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("updateBlock"),
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
                    ::std::borrow::ToOwned::to_owned("getMiddlewareTimesIndexServeUntilBlock"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "getMiddlewareTimesIndexServeUntilBlock",
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
                                name: ::std::borrow::ToOwned::to_owned("index"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getMiddlewareTimesIndexStalestUpdateBlock"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "getMiddlewareTimesIndexStalestUpdateBlock",
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
                                name: ::std::borrow::ToOwned::to_owned("index"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPreviousWhitelistedContractByUpdate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "getPreviousWhitelistedContractByUpdate",
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
                                name: ::std::borrow::ToOwned::to_owned("node"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
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
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
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
                    ::std::borrow::ToOwned::to_owned("isFrozen"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isFrozen"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("staker"),
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
                    ::std::borrow::ToOwned::to_owned("latestUpdateBlock"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("latestUpdateBlock"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("serviceContract"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("middlewareTimesLength"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("middlewareTimesLength",),
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
                    ::std::borrow::ToOwned::to_owned("operatorToMiddlewareTimes"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("operatorToMiddlewareTimes",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("arrayIndex"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct ISlasher.MiddlewareTimes",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("operatorWhitelistedContractsLinkedListEntry"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "operatorWhitelistedContractsLinkedListEntry",
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
                                name: ::std::borrow::ToOwned::to_owned("node"),
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
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("operatorWhitelistedContractsLinkedListSize"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "operatorWhitelistedContractsLinkedListSize",
                        ),
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
                    ::std::borrow::ToOwned::to_owned("optIntoSlashing"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("optIntoSlashing"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("contractAddress"),
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
                    ::std::borrow::ToOwned::to_owned("recordFirstStakeUpdate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("recordFirstStakeUpdate",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("serveUntilBlock"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "recordLastStakeUpdateAndRevokeSlashingAbility",
                    ),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "recordLastStakeUpdateAndRevokeSlashingAbility",
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
                                name: ::std::borrow::ToOwned::to_owned("serveUntilBlock"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("recordStakeUpdate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("recordStakeUpdate"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("updateBlock"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("serveUntilBlock"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("insertAfter"),
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
                    ::std::borrow::ToOwned::to_owned("resetFrozenStatus"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("resetFrozenStatus"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("frozenAddresses"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address[]"),
                            ),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("strategyManager"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("strategyManager"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract IStrategyManager",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                    ::std::borrow::ToOwned::to_owned("whitelistedContractDetails"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("whitelistedContractDetails",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("serviceContract"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct ISlasher.MiddlewareDetails",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("FrozenStatusReset"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("FrozenStatusReset"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("previouslySlashedAddress",),
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
                    ::std::borrow::ToOwned::to_owned("MiddlewareTimesAdded"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("MiddlewareTimesAdded",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("index"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("stalestUpdateBlock",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("latestServeUntilBlock",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OperatorFrozen"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("OperatorFrozen"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("slashedOperator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("slashingContract"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OptedIntoSlashing"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("OptedIntoSlashing"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("contractAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                        ],
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
                    ::std::borrow::ToOwned::to_owned("SlashingAbilityRevoked"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("SlashingAbilityRevoked",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("contractAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned(
                                    "contractCanSlashOperatorUntilBlock",
                                ),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
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
    pub static SLASHER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0,\xD78\x03\x80b\0,\xD7\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x014V[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\x80R\x81\x16`\xA0Rb\0\0Qb\0\0YV[PPb\0\x01sV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15b\0\0\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\xFF\x90\x81\x16\x10\x15b\0\x01\x19W`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x011W`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15b\0\x01HW`\0\x80\xFD[\x82Qb\0\x01U\x81b\0\x01\x1BV[` \x84\x01Q\x90\x92Pb\0\x01h\x81b\0\x01\x1BV[\x80\x91PP\x92P\x92\x90PV[`\x80Q`\xA0Qa+)b\0\x01\xAE`\09`\0\x81\x81a\x05\xC8\x01R\x81\x81a\x14\x99\x01R\x81\x81a\x15+\x01Ra\x16{\x01R`\0a\x03\x07\x01Ra+)`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xFBW`\x005`\xE0\x1C\x80crY\xA4\\\x11a\x01\x1AW\x80c\xD7\xB7\xFA\x13\x11a\0\xADW\x80c\xE5\x83\x986\x11a\0|W\x80c\xE5\x83\x986\x14a\x05\xEAW\x80c\xE9!\xD4\xFA\x14a\x05\xFDW\x80c\xF2\xFD\xE3\x8B\x14a\x06&W\x80c\xF7;u\x19\x14a\x069W\x80c\xFA\xBC\x1C\xBC\x14a\x06LW`\0\x80\xFD[\x80c\xD7\xB7\xFA\x13\x14a\x04\xB8W\x80c\xD9\x81(\xC0\x14a\x05jW\x80c\xDA\x16\xE2\x9B\x14a\x05}W\x80c\xDF\\\xF7#\x14a\x05\xC3W`\0\x80\xFD[\x80c\x88o\x11\x95\x11a\0\xE9W\x80c\x88o\x11\x95\x14a\x04XW\x80c\x8D\xA5\xCB[\x14a\x04kW\x80c\xA4\x9D\xB72\x14a\x04|W\x80c\xC7G\x07[\x14a\x04\xA5W`\0\x80\xFD[\x80crY\xA4\\\x14a\x03\xEFW\x80c|\xF7+\xBA\x14a\x04\x02W\x80c\x81\x05\xE0C\x14a\x04\x15W\x80c\x85_\xCCJ\x14a\x04(W`\0\x80\xFD[\x80c8\xC8\xEEd\x11a\x01\x92W\x80c\\\x97Z\xBB\x11a\x01aW\x80c\\\x97Z\xBB\x14a\x03|W\x80co\x0C/t\x14a\x03\x8EW\x80cqP\x18\xA6\x14a\x03\xD4W\x80cr>Y\xC7\x14a\x03\xDCW`\0\x80\xFD[\x80c8\xC8\xEEd\x14a\x02\xEFW\x80c9\xB7\x0E8\x14a\x03\x02W\x80cY\\jg\x14a\x03AW\x80cZ\xC8j\xB7\x14a\x03IW`\0\x80\xFD[\x80c\x17\x94\xBB<\x11a\x01\xCEW\x80c\x17\x94\xBB<\x14a\x02NW\x80c\x18t\xE5\xAE\x14a\x02aW\x80c(&p\xFC\x14a\x02\x8EW\x80c5W\x16v\x14a\x02\xC5W`\0\x80\xFD[\x80c\x0F\xFA\xBB\xCE\x14a\x02\0W\x80c\x10\xD6z/\x14a\x02\x15W\x80c\x13d9\xDD\x14a\x02(W\x80c\x17]2\x05\x14a\x02;W[`\0\x80\xFD[a\x02\x13a\x02\x0E6`\x04a&\xE6V[a\x06_V[\0[a\x02\x13a\x02#6`\x04a'\x1BV[a\x07\x8FV[a\x02\x13a\x0266`\x04a'8V[a\x08BV[a\x02\x13a\x02I6`\x04a&\xE6V[a\t\x81V[a\x02\x13a\x02\\6`\x04a'QV[a\n\xABV[a\x02ta\x02o6`\x04a&\xE6V[a\x0B\xC9V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\xA1a\x02\x9C6`\x04a'\x92V[a\x0C\x14V[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x93\x84\x01Q\x16\x92\x81\x01\x92\x90\x92R\x01a\x02\x85V[a\x02\xD8a\x02\xD36`\x04a'\x92V[a\x0C\x8BV[`@\x80Q\x92\x15\x15\x83R` \x83\x01\x91\x90\x91R\x01a\x02\x85V[a\x02\x13a\x02\xFD6`\x04a'\x1BV[a\x0C\xBAV[a\x03)\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x85V[a\x02\x13a\r\x88V[a\x03la\x03W6`\x04a'\xBEV[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`@Q\x90\x15\x15\x81R` \x01a\x02\x85V[`fT[`@Q\x90\x81R` \x01a\x02\x85V[a\x02ta\x03\x9C6`\x04a'\xE1V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x97` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x90V[a\x02\x13a\x0EOV[a\x03\x80a\x03\xEA6`\x04a&\xE6V[a\x0EcV[a\x02ta\x03\xFD6`\x04a&\xE6V[a\x0FsV[a\x02\x13a\x04\x106`\x04a(\x1AV[a\x0F\xC3V[a\x03la\x04#6`\x04a(\x8FV[a\x10\rV[a\x04;a\x0466`\x04a'\xE1V[a\x11oV[`@\x80Q\x93\x15\x15\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x02\x85V[`eTa\x03)\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x03)V[a\x03\x80a\x04\x8A6`\x04a'\x1BV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x9A` R`@\x90 T\x90V[a\x02\x13a\x04\xB36`\x04a(\xCDV[a\x11\xB2V[a\x05;a\x04\xC66`\x04a'\xE1V[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x92\x84\x01\x81\x90R`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x81R`\x97\x83R\x83\x81 \x94\x90\x95\x16\x85R\x92\x81R\x92\x81\x90 \x81Q\x92\x83\x01\x82RTc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x81\x16\x94\x84\x01\x94\x90\x94R`\x01`@\x1B\x90\x04\x90\x92\x16\x91\x81\x01\x91\x90\x91R\x90V[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x80\x85\x01Q\x82\x16\x90\x83\x01R\x92\x82\x01Q\x90\x92\x16\x90\x82\x01R``\x01a\x02\x85V[a\x03la\x05x6`\x04a'\xE1V[a\x14\x05V[a\x02ta\x05\x8B6`\x04a'\xE1V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x97` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x90V[a\x03)\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03la\x05\xF86`\x04a'\x1BV[a\x14QV[a\x03\x80a\x06\x0B6`\x04a'\x1BV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x99` R`@\x90 T\x90V[a\x02\x13a\x0646`\x04a'\x1BV[a\x15\xC7V[a\x02\x13a\x06G6`\x04a'\x1BV[a\x16=V[a\x02\x13a\x06Z6`\x04a'8V[a\x17lV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x97` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 T\x82\x90`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x14a\x06\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xB1\x90a)\x1AV[`@Q\x80\x91\x03\x90\xFD[a\x06\xC5\x83C\x84a\x18\xC8V[a\x06\xE73`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x99` R`@\x90 \x90a\x1C^V[a\x07\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`W`$\x82\x01R\x7FSlasher.recordLastStakeUpdateAnd`D\x82\x01R\x7FRevokeSlashingAbility: Removing `d\x82\x01R\x7Fmiddleware unsuccessful\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06\xB1V[a\x07\x8A\x833\x84a\x1C\xF8V[PPPV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xE2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x06\x91\x90a)\x90V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x086W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xB1\x90a)\xADV[a\x08?\x81a\x1E\x03V[PV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x8AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xAE\x91\x90a)\xF7V[a\x08\xCAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xB1\x90a*\x19V[`fT\x81\x81\x16\x14a\tCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\xB1V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`fT`\x01\x90`\x02\x90\x81\x16\x14\x15a\t\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xB1\x90a*aV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x97` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 T\x83\x90`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x14a\t\xFCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xB1\x90a)\x1AV[a\n\x07\x84C\x85a\x18\xC8V[a\n)3`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x99` R`@\x90 \x90a\x1E\xFAV[a\n\xA5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FSlasher.recordFirstStakeUpdate: `D\x82\x01R\x7FAppending middleware unsuccessfu`d\x82\x01R`\x1B`\xFA\x1B`\x84\x82\x01R`\xA4\x01a\x06\xB1V[PPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\n\xCBWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\n\xE5WP0;\x15\x80\x15a\n\xE5WP`\0T`\xFF\x16`\x01\x14[a\x0BHW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x06\xB1V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x0BkW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x0Bu\x83\x83a\x1F\x08V[a\x0B~\x84a\x1F\xEEV[\x80\x15a\n\xA5W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x9A` R`@\x81 \x80Tc\xFF\xFF\xFF\xFF\x84\x16\x90\x81\x10a\x0B\xF9Wa\x0B\xF9a*\x98V[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x90P[\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x9A` R`@\x90 \x80T\x83\x90\x81\x10a\x0CRWa\x0CRa*\x98V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x90\x91\x04\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x99` R`@\x81 \x81\x90a\x0C\xAF\x90\x84a @V[\x91P\x91P\x92P\x92\x90PV[`fT`\x02\x90`\x04\x90\x81\x16\x14\x15a\x0C\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xB1\x90a*aV[a\x0C\xED\x823a\x14\x05V[a\rzW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`R`$\x82\x01R\x7FSlasher.freezeOperator: msg.send`D\x82\x01R\x7Fer does not have permission to s`d\x82\x01Rq60\xB9\xB4\x10:44\xB9\x907\xB82\xB90\xBA7\xB9`q\x1B`\x84\x82\x01R`\xA4\x01a\x06\xB1V[a\r\x84\x823a OV[PPV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xD0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xF4\x91\x90a)\xF7V[a\x0E\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xB1\x90a*\x19V[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[a\x0EWa \xC1V[a\x0Ea`\0a\x1F\xEEV[V[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x81\x81R`\x99` \x90\x81R`@\x80\x83 \x83\x80R`\x01\x90\x81\x01\x83R\x81\x84 \x90\x84R\x82R\x80\x83 T\x93\x83R`\x97\x82R\x80\x83 \x94\x84\x16\x83R\x93\x90R\x91\x82 Tc\xFF\xFF\xFF\xFF\x84\x81\x16`\x01`@\x1B\x90\x92\x04\x16\x11\x15a\x0E\xCCW`\0\x91PPa\x0C\x0EV[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x99` R`@\x81 a\x0E\xEE\x90\x83a!\x1BV[\x91PP[\x80\x15\x80\x15\x90a\x0F6WP`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\0\x90\x81R`\x97` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R Tc\xFF\xFF\xFF\xFF\x85\x81\x16`\x01`@\x1B\x90\x92\x04\x16\x11\x15[\x15a\x0FkW`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x99` R`@\x90 \x90\x91P\x81\x90a\x0Fb\x90\x82a!\x1BV[\x91Pa\x0E\xF2\x90PV[P\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x9A` R`@\x81 \x80Tc\xFF\xFF\xFF\xFF\x84\x16\x90\x81\x10a\x0F\xA3Wa\x0F\xA3a*\x98V[`\0\x91\x82R` \x90\x91 \x01T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x93\x92PPPV[a\x0F\xCBa \xC1V[`\0[\x81\x81\x10\x15a\x07\x8AWa\x10\x05\x83\x83\x83\x81\x81\x10a\x0F\xEBWa\x0F\xEBa*\x98V[\x90P` \x02\x01` \x81\x01\x90a\x10\0\x91\x90a'\x1BV[a!*V[`\x01\x01a\x0F\xCEV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x9A` R`@\x81 Ta\x102WP`\x01a\x11hV[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x9A` R`@\x81 \x80T\x84\x90\x81\x10a\x10\\Wa\x10\\a*\x98V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84R`\x01` \x1B\x90\x92\x04\x81\x16\x93\x83\x01\x93\x90\x93R\x90\x92P\x90\x85\x16\x10\x80\x15\x90a\x10\xBAWP`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x99` R`@\x90 T\x15[\x15a\x116W`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x9A` R`@\x90 \x80Ta\x10\xE5\x90`\x01\x90a*\xC4V[\x81T\x81\x10a\x10\xF5Wa\x10\xF5a*\x98V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x90\x91\x04\x81\x16\x91\x90\x92\x01\x81\x90RC\x90\x91\x16\x11\x91Pa\x11h\x90PV[\x80`\0\x01Qc\xFF\xFF\xFF\xFF\x16\x84c\xFF\xFF\xFF\xFF\x16\x10\x80\x15a\x11dWP\x80` \x01Qc\xFF\xFF\xFF\xFF\x16Cc\xFF\xFF\xFF\xFF\x16\x11[\x91PP[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x99` R`@\x81 \x81\x90\x81\x90a\x11\xA4\x90a\x11\x9F\x86`\x01`\x01`\xA0\x1B\x03\x16\x90V[a!\x94V[\x92P\x92P\x92P[\x92P\x92P\x92V[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x97` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 T\x84\x90`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x14a\x12\x04W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xB1\x90a)\x1AV[C\x84c\xFF\xFF\xFF\xFF\x16\x11\x15a\x12\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FSlasher.recordStakeUpdate: canno`D\x82\x01R\x7Ft provide update for future bloc`d\x82\x01R`k`\xF8\x1B`\x84\x82\x01R`\xA4\x01a\x06\xB1V[a\x12\x95\x85\x85\x85a\x18\xC8V[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x99` R`@\x90 T`\x01\x14a\x13YWa\x12\xD73`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81R`\x99` R`@\x90 \x90a\x1C^V[a\x13IW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`;`$\x82\x01R\x7FSlasher.recordStakeUpdate: Remov`D\x82\x01R\x7Fing middleware unsuccessful\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\xB1V[a\x13T\x85\x85\x84a!\xE4V[a\x13\xFEV[3`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x99` \x90\x81R`@\x80\x83 \x83\x80R`\x01\x90\x81\x01\x83R\x81\x84 \x90\x84R\x90\x91R\x90 T\x14a\x13\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FSlasher.recordStakeUpdate: Calle`D\x82\x01R\x7Fr is not the list entrant\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\xB1V[PPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x90\x81R`\x97` \x90\x81R`@\x80\x83 \x93\x85\x16\x83R\x92\x90R\x90\x81 T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16C\x10\x15a\x14IWP`\x01a\x0C\x0EV[P`\0a\x0C\x0EV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x98` R`@\x81 T`\xFF\x16\x15a\x14zWP`\x01\x91\x90PV[`@Qc>(9\x1D`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c>(9\x1D\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\xE0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x04\x91\x90a)\xF7V[\x15a\x15\xBAW`@Qc\x19v\x84\x99`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90ce\xDA\x12d\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15tW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x98\x91\x90a)\x90V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x98` R`@\x90 T`\xFF\x16\x93\x92PPPV[P`\0\x91\x90PV[\x91\x90PV[a\x15\xCFa \xC1V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x164W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x06\xB1V[a\x08?\x81a\x1F\xEEV[`fT`\0\x90`\x01\x90\x81\x16\x14\x15a\x16fW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xB1\x90a*aV[`@Qc6\xB8{\xD7`\xE1\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cmp\xF7\xAE\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xCAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xEE\x91\x90a)\xF7V[a\x17bW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FSlasher.optIntoSlashing: msg.sen`D\x82\x01R\x7Fder is not a registered operator`d\x82\x01R`\x84\x01a\x06\xB1V[a\r\x843\x83a$\x9BV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\xBFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xE3\x91\x90a)\x90V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x18\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xB1\x90a)\xADV[`fT\x19\x81\x19`fT\x19\x16\x14a\x18\x91W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\xB1V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\tvV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x97` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 Tc\xFF\xFF\xFF\xFF\x80\x84\x16`\x01`@\x1B\x90\x92\x04\x16\x11\x15a\x19\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`J`$\x82\x01R\x7FSlasher._recordUpdateAndAddToMid`D\x82\x01R\x7FdlewareTimes: can't push a previ`d\x82\x01Rious update`\xB0\x1B`\x84\x82\x01R`\xA4\x01a\x06\xB1V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81R`\x97` \x90\x81R`@\x80\x83 3\x84R\x82R\x80\x83 \x80Tk\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x16`\x01`@\x1Bc\xFF\xFF\xFF\xFF\x89\x16\x02\x17\x90U\x80Q\x80\x82\x01\x82R\x83\x81R\x80\x83\x01\x84\x90R\x93\x83R`\x9A\x90\x91R\x90 T\x80\x15a\x1AQW`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x9A` R`@\x90 a\x1A\x0C`\x01\x83a*\xC4V[\x81T\x81\x10a\x1A\x1CWa\x1A\x1Ca*\x98V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x90\x91\x04\x16\x91\x81\x01\x91\x90\x91R\x91P[`\0\x82\x90P`\0\x83` \x01Qc\xFF\xFF\xFF\xFF\x16\x85c\xFF\xFF\xFF\xFF\x16\x11\x15a\x1A\x80WPc\xFF\xFF\xFF\xFF\x84\x16` \x82\x01R`\x01[`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81R`\x99` R`@\x90 Ta\x1A\xAEWPc\xFF\xFF\xFF\xFF\x85\x16\x81R`\x01a\x1B\x83V[3`\x01`\x01`\xA0\x1B\x03\x88\x16`\0\x90\x81R`\x99` \x90\x81R`@\x80\x83 \x83\x80R`\x01\x90\x81\x01\x83R\x81\x84 \x90\x84R\x90\x91R\x90 T\x14\x15a\x1B\x83W`\0\x80a\x1B\x0B3`\x01`\x01`\xA0\x1B\x03\x8B\x16`\0\x90\x81R`\x99` R`@\x90 \x90a!\x1BV[\x91P\x91P\x81\x15a\x1BrW`\x01`\x01`\xA0\x1B\x03\x89\x81\x16`\0\x90\x81R`\x97` \x90\x81R`@\x80\x83 \x93\x85\x16\x83R\x92\x90R T`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x89\x16\x81\x10\x15a\x1BbWc\xFF\xFF\xFF\xFF\x81\x16\x85Ra\x1BlV[c\xFF\xFF\xFF\xFF\x89\x16\x85R[Pa\x1B|V[c\xFF\xFF\xFF\xFF\x88\x16\x84R[`\x01\x92PPP[\x80\x15a\x1CUW`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x81\x81R`\x9A` \x90\x81R`@\x82 \x80T`\x01\x81\x81\x01\x83U\x82\x85R\x83\x85 \x88Q\x92\x01\x80T\x94\x89\x01Qc\xFF\xFF\xFF\xFF\x90\x81\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x96\x16\x93\x16\x92\x90\x92\x17\x93\x90\x93\x17\x90U\x92\x90\x91R\x90T\x7F\x1Bb\xBAd\xC7-\x01\xE4\x1A+\x8CF\xE6\xAE\xEF\xF7(\xEF:D8\xCF\x1C\xAC=\x92\xEE\x12\x18\x9DVI\x91\x89\x91a\x1C\x17\x91\x90a*\xC4V[\x84Q` \x80\x87\x01Q`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x96\x16\x86R\x91\x85\x01\x93\x90\x93Rc\xFF\xFF\xFF\xFF\x91\x82\x16\x90\x84\x01R\x16``\x82\x01R`\x80\x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPV[`\0\x81\x15\x80a\x1CtWPa\x1Cr\x83\x83a%\x06V[\x15[\x15a\x1C\x81WP`\0a\x0C\x0EV[`\0\x82\x81R`\x01\x84\x81\x01` \x90\x81R`@\x80\x84 \x84\x80R\x90\x91R\x80\x83 T\x82\x84R\x92 Ta\x1C\xB3\x92\x86\x92\x90\x91\x90a%\x82V[`\0\x82\x81R`\x01\x84\x81\x01` \x90\x81R`@\x80\x84 \x84\x80R\x90\x91R\x80\x83 \x83\x90U\x81\x83R\x82 \x82\x90U\x84T\x90\x91\x85\x91a\x1C\xEC\x90\x84\x90a*\xC4V[\x90\x91UP\x91\x93\x92PPPV[c\xFF\xFF\xFF\xFF\x81\x81\x16\x14\x15a\x1D\x82W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FSlasher._revokeSlashingAbility: \x90\x82\x01R\x7FserveUntilBlock time must be lim`d\x82\x01Rc\x1A]\x19Y`\xE2\x1B`\x84\x82\x01R`\xA4\x01a\x06\xB1V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x81\x81R`\x97` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x80Tg\xFF\xFF\xFF\xFF\0\0\0\0\x19\x16`\x01` \x1Bc\xFF\xFF\xFF\xFF\x88\x16\x90\x81\x02\x91\x90\x91\x17\x90\x91U\x91Q\x91\x82R\x7F\x9A\xA1\xB19\x1F5\xC6r\xED\x1F;~\xCEc/E\x13\xE6\x186k\xEFz/g\xB7\xC6\xBC\x1F-+\x14\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1E\x91W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x06\xB1V[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0a\x11h\x83\x83`\0a%\xB7V[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a\x1F)WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a\x1F\xABW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x06\xB1V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a\r\x84\x82a\x1E\x03V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0\x80a\x0C\xAF\x84\x84`\0a%\xCEV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x98` R`@\x90 T`\xFF\x16a\r\x84W`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\0\x81\x81R`\x98` R`@\x80\x82 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x92\x84\x16\x92\x7FDJ\x84\xF5\x12\x81j\xE7\xBE\x8E\xD8\xA6j\xA8\x8E6.\xB5M\t\x88\xE8:\xCC\x9D\x81tf\"\xB3\xBAQ\x91\x90\xA3PPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0EaW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x06\xB1V[`\0\x80a\x0C\xAF\x84\x84`\x01a%\xCEV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x98` R`@\x90 T`\xFF\x16\x15a\x08?W`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x81\x81R`\x98` R`@\x80\x82 \x80T`\xFF\x19\x16\x90UQ\x7F\xD4\xCE\xF0\xAF'\x80\rFo\xCA\xCD\x85w\x98W7\x8B\x85\xCBaV\x90\x05\xFF\x14d\xFAn\\\xEDi\xD8\x91\x90\xA2PV[`\0\x80`\0a!\xA3\x85\x85a%\x06V[a!\xB5WP`\0\x91P\x81\x90P\x80a\x11\xABV[PPP`\0\x81\x81R`\x01\x83\x81\x01` \x90\x81R`@\x80\x84 \x84\x80R\x90\x91R\x80\x83 T\x82\x84R\x92 T\x90\x91\x90a\x11\xABV[`\0\x81\x15a#\x8EW`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x99` R`@\x90 a\"\x0E\x90\x83a%\x06V[a\"\x16WP`\x01[\x80\x15\x80\x15a\"XWP`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\0\x90\x81R`\x97` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R Tc\xFF\xFF\xFF\xFF\x84\x81\x16`\x01`@\x1B\x90\x92\x04\x16\x11[\x15a\"aWP`\x01[\x80a\"\xD8W`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x99` R`@\x81 \x81\x90a\"\x8A\x90\x85a!\x1BV[\x91P\x91P\x81\x15a\"\xD5W`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\0\x90\x81R`\x97` \x90\x81R`@\x80\x83 \x93\x85\x16\x83R\x92\x90R Tc\xFF\xFF\xFF\xFF\x86\x81\x16`\x01`@\x1B\x90\x92\x04\x16\x11a\"\xD5W`\x01\x92P[PP[\x80a#wW`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x99` R`@\x90 a#\0\x90\x833a&\x17V[a#rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R\x7FSlasher.recordStakeUpdate: Inser`D\x82\x01R\x7Fting middleware unsuccessful\0\0\0\0`d\x82\x01R`\x84\x01a\x06\xB1V[a\n\xA5V[a#\x81\x84\x84a\x0EcV[\x91Pa#r\x84\x84\x84a!\xE4V[`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\0\x90\x81R`\x97` \x90\x81R`@\x80\x83 `\x99\x83R\x81\x84 \x84\x80R`\x01\x90\x81\x01\x84R\x82\x85 \x90\x85R\x83R\x81\x84 T\x90\x94\x16\x83R\x92\x90R Tc\xFF\xFF\xFF\xFF\x84\x81\x16`\x01`@\x1B\x90\x92\x04\x16\x11a#\xEBWP`\x01[\x80a$\x84Wa$\x123`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x99` R`@\x90 \x90a&&V[a#rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FSlasher.recordStakeUpdate: Prepp`D\x82\x01R\x7Fending middleware unsuccessful\0\0`d\x82\x01R`\x84\x01a\x06\xB1V[a$\x8E\x84\x84a\x0EcV[\x91Pa\n\xA5\x84\x84\x84a!\xE4V[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\0\x81\x81R`\x97` \x90\x81R`@\x80\x83 \x94\x86\x16\x80\x84R\x94\x90\x91R\x80\x82 \x80Tg\xFF\xFF\xFF\xFF\0\0\0\0\x19\x16g\xFF\xFF\xFF\xFF\0\0\0\0\x17\x90UQ\x7F\xEF\xA9\xFB8\xE8\x13\xD5<\x15\xED\xF5\x01\xE08R\x84:?\xEDi\x19`R3\x91\xD7\x1A\t+6'\xD8\x91\x90\xA3PPV[`\0\x81\x81R`\x01\x83\x01` \x90\x81R`@\x80\x83 \x83\x80R\x90\x91R\x81 T\x15\x80\x15a%IWP`\0\x82\x81R`\x01\x80\x85\x01` \x90\x81R`@\x80\x84 \x92\x84R\x91\x90R\x90 T\x15[\x15a%zW`\0\x80\x80R`\x01\x80\x85\x01` \x90\x81R`@\x80\x84 \x92\x84R\x91\x90R\x90 T\x82\x14\x15a\x14IWP`\x01a\x0C\x0EV[P`\x01a\x0C\x0EV[`\0\x82\x81R`\x01\x90\x94\x01` \x81\x81R`@\x80\x87 \x93\x15\x80\x88R\x93\x82R\x80\x87 \x86\x90U\x94\x86R\x90\x81R\x83\x85 \x91\x15\x85RR\x91 UV[`\0a%\xC6\x84`\0\x85\x85a&4V[\x94\x93PPPPV[`\0\x80a%\xDB\x85\x85a%\x06V[a%\xEAWP`\0\x90P\x80a&\x0FV[PP`\0\x82\x81R`\x01\x84\x01` \x90\x81R`@\x80\x83 \x84\x15\x15\x84R\x90\x91R\x90 T\x80\x15\x15\x90[\x93P\x93\x91PPV[`\0a%\xC6\x84\x84\x84`\x01a&4V[`\0a\x11h\x83\x83`\x01a%\xB7V[`\0a&@\x85\x84a%\x06V[\x15\x80\x15a&RWPa&R\x85\x85a%\x06V[\x15a&\xB2W`\0\x84\x81R`\x01\x86\x01` \x90\x81R`@\x80\x83 \x85\x15\x15\x84R\x90\x91R\x90 Ta&\x81\x86\x86\x86\x86a%\x82V[a&\x8D\x86\x85\x83\x86a%\x82V[`\x01\x86`\0\x01`\0\x82\x82Ta&\xA2\x91\x90a*\xDBV[\x90\x91UP`\x01\x92Pa%\xC6\x91PPV[P`\0\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x08?W`\0\x80\xFD[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x15\xC2W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a&\xF9W`\0\x80\xFD[\x825a'\x04\x81a&\xBDV[\x91Pa'\x12` \x84\x01a&\xD2V[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a'-W`\0\x80\xFD[\x815a\x11h\x81a&\xBDV[`\0` \x82\x84\x03\x12\x15a'JW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a'fW`\0\x80\xFD[\x835a'q\x81a&\xBDV[\x92P` \x84\x015a'\x81\x81a&\xBDV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80`@\x83\x85\x03\x12\x15a'\xA5W`\0\x80\xFD[\x825a'\xB0\x81a&\xBDV[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a'\xD0W`\0\x80\xFD[\x815`\xFF\x81\x16\x81\x14a\x11hW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a'\xF4W`\0\x80\xFD[\x825a'\xFF\x81a&\xBDV[\x91P` \x83\x015a(\x0F\x81a&\xBDV[\x80\x91PP\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a(-W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a(EW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a(YW`\0\x80\xFD[\x815\x81\x81\x11\x15a(hW`\0\x80\xFD[\x86` \x82`\x05\x1B\x85\x01\x01\x11\x15a(}W`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a(\xA4W`\0\x80\xFD[\x835a(\xAF\x81a&\xBDV[\x92Pa(\xBD` \x85\x01a&\xD2V[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a(\xE3W`\0\x80\xFD[\x845a(\xEE\x81a&\xBDV[\x93Pa(\xFC` \x86\x01a&\xD2V[\x92Pa)\n`@\x86\x01a&\xD2V[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[` \x80\x82R`P\x90\x82\x01R\x7FSlasher.onlyRegisteredForService`@\x82\x01R\x7F: Operator has not opted into sl``\x82\x01Ro0\xB9\xB44\xB73\x901<\x901\xB0\xB662\xB9`\x81\x1B`\x80\x82\x01R`\xA0\x01\x90V[`\0` \x82\x84\x03\x12\x15a)\xA2W`\0\x80\xFD[\x81Qa\x11h\x81a&\xBDV[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15a*\tW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x11hW`\0\x80\xFD[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`\x19\x90\x82\x01R\x7FPausable: index is paused\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a*\xD6Wa*\xD6a*\xAEV[P\x03\x90V[`\0\x82\x19\x82\x11\x15a*\xEEWa*\xEEa*\xAEV[P\x01\x90V\xFE\xA2dipfsX\"\x12 J\xC4`\x8B \xA2\xE5\xB5!B\x9D\xA0\x81\xB6\r\xD8\xAC<\xB0\x97\xF0\xA2j\xC1J\xF3\xA9\x1D\xAB\x9B\x03edsolcC\0\x08\x0C\x003";
    /// The bytecode of the contract.
    pub static SLASHER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xFBW`\x005`\xE0\x1C\x80crY\xA4\\\x11a\x01\x1AW\x80c\xD7\xB7\xFA\x13\x11a\0\xADW\x80c\xE5\x83\x986\x11a\0|W\x80c\xE5\x83\x986\x14a\x05\xEAW\x80c\xE9!\xD4\xFA\x14a\x05\xFDW\x80c\xF2\xFD\xE3\x8B\x14a\x06&W\x80c\xF7;u\x19\x14a\x069W\x80c\xFA\xBC\x1C\xBC\x14a\x06LW`\0\x80\xFD[\x80c\xD7\xB7\xFA\x13\x14a\x04\xB8W\x80c\xD9\x81(\xC0\x14a\x05jW\x80c\xDA\x16\xE2\x9B\x14a\x05}W\x80c\xDF\\\xF7#\x14a\x05\xC3W`\0\x80\xFD[\x80c\x88o\x11\x95\x11a\0\xE9W\x80c\x88o\x11\x95\x14a\x04XW\x80c\x8D\xA5\xCB[\x14a\x04kW\x80c\xA4\x9D\xB72\x14a\x04|W\x80c\xC7G\x07[\x14a\x04\xA5W`\0\x80\xFD[\x80crY\xA4\\\x14a\x03\xEFW\x80c|\xF7+\xBA\x14a\x04\x02W\x80c\x81\x05\xE0C\x14a\x04\x15W\x80c\x85_\xCCJ\x14a\x04(W`\0\x80\xFD[\x80c8\xC8\xEEd\x11a\x01\x92W\x80c\\\x97Z\xBB\x11a\x01aW\x80c\\\x97Z\xBB\x14a\x03|W\x80co\x0C/t\x14a\x03\x8EW\x80cqP\x18\xA6\x14a\x03\xD4W\x80cr>Y\xC7\x14a\x03\xDCW`\0\x80\xFD[\x80c8\xC8\xEEd\x14a\x02\xEFW\x80c9\xB7\x0E8\x14a\x03\x02W\x80cY\\jg\x14a\x03AW\x80cZ\xC8j\xB7\x14a\x03IW`\0\x80\xFD[\x80c\x17\x94\xBB<\x11a\x01\xCEW\x80c\x17\x94\xBB<\x14a\x02NW\x80c\x18t\xE5\xAE\x14a\x02aW\x80c(&p\xFC\x14a\x02\x8EW\x80c5W\x16v\x14a\x02\xC5W`\0\x80\xFD[\x80c\x0F\xFA\xBB\xCE\x14a\x02\0W\x80c\x10\xD6z/\x14a\x02\x15W\x80c\x13d9\xDD\x14a\x02(W\x80c\x17]2\x05\x14a\x02;W[`\0\x80\xFD[a\x02\x13a\x02\x0E6`\x04a&\xE6V[a\x06_V[\0[a\x02\x13a\x02#6`\x04a'\x1BV[a\x07\x8FV[a\x02\x13a\x0266`\x04a'8V[a\x08BV[a\x02\x13a\x02I6`\x04a&\xE6V[a\t\x81V[a\x02\x13a\x02\\6`\x04a'QV[a\n\xABV[a\x02ta\x02o6`\x04a&\xE6V[a\x0B\xC9V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\xA1a\x02\x9C6`\x04a'\x92V[a\x0C\x14V[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x93\x84\x01Q\x16\x92\x81\x01\x92\x90\x92R\x01a\x02\x85V[a\x02\xD8a\x02\xD36`\x04a'\x92V[a\x0C\x8BV[`@\x80Q\x92\x15\x15\x83R` \x83\x01\x91\x90\x91R\x01a\x02\x85V[a\x02\x13a\x02\xFD6`\x04a'\x1BV[a\x0C\xBAV[a\x03)\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x85V[a\x02\x13a\r\x88V[a\x03la\x03W6`\x04a'\xBEV[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`@Q\x90\x15\x15\x81R` \x01a\x02\x85V[`fT[`@Q\x90\x81R` \x01a\x02\x85V[a\x02ta\x03\x9C6`\x04a'\xE1V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x97` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x90V[a\x02\x13a\x0EOV[a\x03\x80a\x03\xEA6`\x04a&\xE6V[a\x0EcV[a\x02ta\x03\xFD6`\x04a&\xE6V[a\x0FsV[a\x02\x13a\x04\x106`\x04a(\x1AV[a\x0F\xC3V[a\x03la\x04#6`\x04a(\x8FV[a\x10\rV[a\x04;a\x0466`\x04a'\xE1V[a\x11oV[`@\x80Q\x93\x15\x15\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x02\x85V[`eTa\x03)\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x03)V[a\x03\x80a\x04\x8A6`\x04a'\x1BV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x9A` R`@\x90 T\x90V[a\x02\x13a\x04\xB36`\x04a(\xCDV[a\x11\xB2V[a\x05;a\x04\xC66`\x04a'\xE1V[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x92\x84\x01\x81\x90R`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x81R`\x97\x83R\x83\x81 \x94\x90\x95\x16\x85R\x92\x81R\x92\x81\x90 \x81Q\x92\x83\x01\x82RTc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x81\x16\x94\x84\x01\x94\x90\x94R`\x01`@\x1B\x90\x04\x90\x92\x16\x91\x81\x01\x91\x90\x91R\x90V[`@\x80Q\x82Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x80\x85\x01Q\x82\x16\x90\x83\x01R\x92\x82\x01Q\x90\x92\x16\x90\x82\x01R``\x01a\x02\x85V[a\x03la\x05x6`\x04a'\xE1V[a\x14\x05V[a\x02ta\x05\x8B6`\x04a'\xE1V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x97` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x90V[a\x03)\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03la\x05\xF86`\x04a'\x1BV[a\x14QV[a\x03\x80a\x06\x0B6`\x04a'\x1BV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x99` R`@\x90 T\x90V[a\x02\x13a\x0646`\x04a'\x1BV[a\x15\xC7V[a\x02\x13a\x06G6`\x04a'\x1BV[a\x16=V[a\x02\x13a\x06Z6`\x04a'8V[a\x17lV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x97` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 T\x82\x90`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x14a\x06\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xB1\x90a)\x1AV[`@Q\x80\x91\x03\x90\xFD[a\x06\xC5\x83C\x84a\x18\xC8V[a\x06\xE73`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x99` R`@\x90 \x90a\x1C^V[a\x07\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`W`$\x82\x01R\x7FSlasher.recordLastStakeUpdateAnd`D\x82\x01R\x7FRevokeSlashingAbility: Removing `d\x82\x01R\x7Fmiddleware unsuccessful\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06\xB1V[a\x07\x8A\x833\x84a\x1C\xF8V[PPPV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xE2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x06\x91\x90a)\x90V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x086W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xB1\x90a)\xADV[a\x08?\x81a\x1E\x03V[PV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x8AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xAE\x91\x90a)\xF7V[a\x08\xCAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xB1\x90a*\x19V[`fT\x81\x81\x16\x14a\tCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\xB1V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`fT`\x01\x90`\x02\x90\x81\x16\x14\x15a\t\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xB1\x90a*aV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x97` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 T\x83\x90`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x14a\t\xFCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xB1\x90a)\x1AV[a\n\x07\x84C\x85a\x18\xC8V[a\n)3`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x99` R`@\x90 \x90a\x1E\xFAV[a\n\xA5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FSlasher.recordFirstStakeUpdate: `D\x82\x01R\x7FAppending middleware unsuccessfu`d\x82\x01R`\x1B`\xFA\x1B`\x84\x82\x01R`\xA4\x01a\x06\xB1V[PPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\n\xCBWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\n\xE5WP0;\x15\x80\x15a\n\xE5WP`\0T`\xFF\x16`\x01\x14[a\x0BHW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x06\xB1V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x0BkW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x0Bu\x83\x83a\x1F\x08V[a\x0B~\x84a\x1F\xEEV[\x80\x15a\n\xA5W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x9A` R`@\x81 \x80Tc\xFF\xFF\xFF\xFF\x84\x16\x90\x81\x10a\x0B\xF9Wa\x0B\xF9a*\x98V[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x90P[\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x9A` R`@\x90 \x80T\x83\x90\x81\x10a\x0CRWa\x0CRa*\x98V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x90\x91\x04\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x99` R`@\x81 \x81\x90a\x0C\xAF\x90\x84a @V[\x91P\x91P\x92P\x92\x90PV[`fT`\x02\x90`\x04\x90\x81\x16\x14\x15a\x0C\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xB1\x90a*aV[a\x0C\xED\x823a\x14\x05V[a\rzW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`R`$\x82\x01R\x7FSlasher.freezeOperator: msg.send`D\x82\x01R\x7Fer does not have permission to s`d\x82\x01Rq60\xB9\xB4\x10:44\xB9\x907\xB82\xB90\xBA7\xB9`q\x1B`\x84\x82\x01R`\xA4\x01a\x06\xB1V[a\r\x84\x823a OV[PPV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xD0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xF4\x91\x90a)\xF7V[a\x0E\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xB1\x90a*\x19V[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[a\x0EWa \xC1V[a\x0Ea`\0a\x1F\xEEV[V[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x81\x81R`\x99` \x90\x81R`@\x80\x83 \x83\x80R`\x01\x90\x81\x01\x83R\x81\x84 \x90\x84R\x82R\x80\x83 T\x93\x83R`\x97\x82R\x80\x83 \x94\x84\x16\x83R\x93\x90R\x91\x82 Tc\xFF\xFF\xFF\xFF\x84\x81\x16`\x01`@\x1B\x90\x92\x04\x16\x11\x15a\x0E\xCCW`\0\x91PPa\x0C\x0EV[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x99` R`@\x81 a\x0E\xEE\x90\x83a!\x1BV[\x91PP[\x80\x15\x80\x15\x90a\x0F6WP`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\0\x90\x81R`\x97` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R Tc\xFF\xFF\xFF\xFF\x85\x81\x16`\x01`@\x1B\x90\x92\x04\x16\x11\x15[\x15a\x0FkW`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x99` R`@\x90 \x90\x91P\x81\x90a\x0Fb\x90\x82a!\x1BV[\x91Pa\x0E\xF2\x90PV[P\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x9A` R`@\x81 \x80Tc\xFF\xFF\xFF\xFF\x84\x16\x90\x81\x10a\x0F\xA3Wa\x0F\xA3a*\x98V[`\0\x91\x82R` \x90\x91 \x01T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x93\x92PPPV[a\x0F\xCBa \xC1V[`\0[\x81\x81\x10\x15a\x07\x8AWa\x10\x05\x83\x83\x83\x81\x81\x10a\x0F\xEBWa\x0F\xEBa*\x98V[\x90P` \x02\x01` \x81\x01\x90a\x10\0\x91\x90a'\x1BV[a!*V[`\x01\x01a\x0F\xCEV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x9A` R`@\x81 Ta\x102WP`\x01a\x11hV[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x9A` R`@\x81 \x80T\x84\x90\x81\x10a\x10\\Wa\x10\\a*\x98V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84R`\x01` \x1B\x90\x92\x04\x81\x16\x93\x83\x01\x93\x90\x93R\x90\x92P\x90\x85\x16\x10\x80\x15\x90a\x10\xBAWP`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x99` R`@\x90 T\x15[\x15a\x116W`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x9A` R`@\x90 \x80Ta\x10\xE5\x90`\x01\x90a*\xC4V[\x81T\x81\x10a\x10\xF5Wa\x10\xF5a*\x98V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x90\x91\x04\x81\x16\x91\x90\x92\x01\x81\x90RC\x90\x91\x16\x11\x91Pa\x11h\x90PV[\x80`\0\x01Qc\xFF\xFF\xFF\xFF\x16\x84c\xFF\xFF\xFF\xFF\x16\x10\x80\x15a\x11dWP\x80` \x01Qc\xFF\xFF\xFF\xFF\x16Cc\xFF\xFF\xFF\xFF\x16\x11[\x91PP[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x99` R`@\x81 \x81\x90\x81\x90a\x11\xA4\x90a\x11\x9F\x86`\x01`\x01`\xA0\x1B\x03\x16\x90V[a!\x94V[\x92P\x92P\x92P[\x92P\x92P\x92V[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x97` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 T\x84\x90`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x14a\x12\x04W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xB1\x90a)\x1AV[C\x84c\xFF\xFF\xFF\xFF\x16\x11\x15a\x12\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FSlasher.recordStakeUpdate: canno`D\x82\x01R\x7Ft provide update for future bloc`d\x82\x01R`k`\xF8\x1B`\x84\x82\x01R`\xA4\x01a\x06\xB1V[a\x12\x95\x85\x85\x85a\x18\xC8V[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x99` R`@\x90 T`\x01\x14a\x13YWa\x12\xD73`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81R`\x99` R`@\x90 \x90a\x1C^V[a\x13IW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`;`$\x82\x01R\x7FSlasher.recordStakeUpdate: Remov`D\x82\x01R\x7Fing middleware unsuccessful\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\xB1V[a\x13T\x85\x85\x84a!\xE4V[a\x13\xFEV[3`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x99` \x90\x81R`@\x80\x83 \x83\x80R`\x01\x90\x81\x01\x83R\x81\x84 \x90\x84R\x90\x91R\x90 T\x14a\x13\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FSlasher.recordStakeUpdate: Calle`D\x82\x01R\x7Fr is not the list entrant\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\xB1V[PPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x90\x81R`\x97` \x90\x81R`@\x80\x83 \x93\x85\x16\x83R\x92\x90R\x90\x81 T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16C\x10\x15a\x14IWP`\x01a\x0C\x0EV[P`\0a\x0C\x0EV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x98` R`@\x81 T`\xFF\x16\x15a\x14zWP`\x01\x91\x90PV[`@Qc>(9\x1D`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c>(9\x1D\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\xE0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x04\x91\x90a)\xF7V[\x15a\x15\xBAW`@Qc\x19v\x84\x99`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90ce\xDA\x12d\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15tW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x98\x91\x90a)\x90V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x98` R`@\x90 T`\xFF\x16\x93\x92PPPV[P`\0\x91\x90PV[\x91\x90PV[a\x15\xCFa \xC1V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x164W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x06\xB1V[a\x08?\x81a\x1F\xEEV[`fT`\0\x90`\x01\x90\x81\x16\x14\x15a\x16fW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xB1\x90a*aV[`@Qc6\xB8{\xD7`\xE1\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cmp\xF7\xAE\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xCAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xEE\x91\x90a)\xF7V[a\x17bW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FSlasher.optIntoSlashing: msg.sen`D\x82\x01R\x7Fder is not a registered operator`d\x82\x01R`\x84\x01a\x06\xB1V[a\r\x843\x83a$\x9BV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\xBFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xE3\x91\x90a)\x90V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x18\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xB1\x90a)\xADV[`fT\x19\x81\x19`fT\x19\x16\x14a\x18\x91W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\xB1V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\tvV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x97` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 Tc\xFF\xFF\xFF\xFF\x80\x84\x16`\x01`@\x1B\x90\x92\x04\x16\x11\x15a\x19\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`J`$\x82\x01R\x7FSlasher._recordUpdateAndAddToMid`D\x82\x01R\x7FdlewareTimes: can't push a previ`d\x82\x01Rious update`\xB0\x1B`\x84\x82\x01R`\xA4\x01a\x06\xB1V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81R`\x97` \x90\x81R`@\x80\x83 3\x84R\x82R\x80\x83 \x80Tk\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x16`\x01`@\x1Bc\xFF\xFF\xFF\xFF\x89\x16\x02\x17\x90U\x80Q\x80\x82\x01\x82R\x83\x81R\x80\x83\x01\x84\x90R\x93\x83R`\x9A\x90\x91R\x90 T\x80\x15a\x1AQW`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x9A` R`@\x90 a\x1A\x0C`\x01\x83a*\xC4V[\x81T\x81\x10a\x1A\x1CWa\x1A\x1Ca*\x98V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x90\x91\x04\x16\x91\x81\x01\x91\x90\x91R\x91P[`\0\x82\x90P`\0\x83` \x01Qc\xFF\xFF\xFF\xFF\x16\x85c\xFF\xFF\xFF\xFF\x16\x11\x15a\x1A\x80WPc\xFF\xFF\xFF\xFF\x84\x16` \x82\x01R`\x01[`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81R`\x99` R`@\x90 Ta\x1A\xAEWPc\xFF\xFF\xFF\xFF\x85\x16\x81R`\x01a\x1B\x83V[3`\x01`\x01`\xA0\x1B\x03\x88\x16`\0\x90\x81R`\x99` \x90\x81R`@\x80\x83 \x83\x80R`\x01\x90\x81\x01\x83R\x81\x84 \x90\x84R\x90\x91R\x90 T\x14\x15a\x1B\x83W`\0\x80a\x1B\x0B3`\x01`\x01`\xA0\x1B\x03\x8B\x16`\0\x90\x81R`\x99` R`@\x90 \x90a!\x1BV[\x91P\x91P\x81\x15a\x1BrW`\x01`\x01`\xA0\x1B\x03\x89\x81\x16`\0\x90\x81R`\x97` \x90\x81R`@\x80\x83 \x93\x85\x16\x83R\x92\x90R T`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x89\x16\x81\x10\x15a\x1BbWc\xFF\xFF\xFF\xFF\x81\x16\x85Ra\x1BlV[c\xFF\xFF\xFF\xFF\x89\x16\x85R[Pa\x1B|V[c\xFF\xFF\xFF\xFF\x88\x16\x84R[`\x01\x92PPP[\x80\x15a\x1CUW`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x81\x81R`\x9A` \x90\x81R`@\x82 \x80T`\x01\x81\x81\x01\x83U\x82\x85R\x83\x85 \x88Q\x92\x01\x80T\x94\x89\x01Qc\xFF\xFF\xFF\xFF\x90\x81\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x96\x16\x93\x16\x92\x90\x92\x17\x93\x90\x93\x17\x90U\x92\x90\x91R\x90T\x7F\x1Bb\xBAd\xC7-\x01\xE4\x1A+\x8CF\xE6\xAE\xEF\xF7(\xEF:D8\xCF\x1C\xAC=\x92\xEE\x12\x18\x9DVI\x91\x89\x91a\x1C\x17\x91\x90a*\xC4V[\x84Q` \x80\x87\x01Q`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x96\x16\x86R\x91\x85\x01\x93\x90\x93Rc\xFF\xFF\xFF\xFF\x91\x82\x16\x90\x84\x01R\x16``\x82\x01R`\x80\x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPV[`\0\x81\x15\x80a\x1CtWPa\x1Cr\x83\x83a%\x06V[\x15[\x15a\x1C\x81WP`\0a\x0C\x0EV[`\0\x82\x81R`\x01\x84\x81\x01` \x90\x81R`@\x80\x84 \x84\x80R\x90\x91R\x80\x83 T\x82\x84R\x92 Ta\x1C\xB3\x92\x86\x92\x90\x91\x90a%\x82V[`\0\x82\x81R`\x01\x84\x81\x01` \x90\x81R`@\x80\x84 \x84\x80R\x90\x91R\x80\x83 \x83\x90U\x81\x83R\x82 \x82\x90U\x84T\x90\x91\x85\x91a\x1C\xEC\x90\x84\x90a*\xC4V[\x90\x91UP\x91\x93\x92PPPV[c\xFF\xFF\xFF\xFF\x81\x81\x16\x14\x15a\x1D\x82W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FSlasher._revokeSlashingAbility: \x90\x82\x01R\x7FserveUntilBlock time must be lim`d\x82\x01Rc\x1A]\x19Y`\xE2\x1B`\x84\x82\x01R`\xA4\x01a\x06\xB1V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x81\x81R`\x97` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x80Tg\xFF\xFF\xFF\xFF\0\0\0\0\x19\x16`\x01` \x1Bc\xFF\xFF\xFF\xFF\x88\x16\x90\x81\x02\x91\x90\x91\x17\x90\x91U\x91Q\x91\x82R\x7F\x9A\xA1\xB19\x1F5\xC6r\xED\x1F;~\xCEc/E\x13\xE6\x186k\xEFz/g\xB7\xC6\xBC\x1F-+\x14\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1E\x91W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x06\xB1V[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0a\x11h\x83\x83`\0a%\xB7V[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a\x1F)WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a\x1F\xABW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x06\xB1V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a\r\x84\x82a\x1E\x03V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\0\x80a\x0C\xAF\x84\x84`\0a%\xCEV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x98` R`@\x90 T`\xFF\x16a\r\x84W`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\0\x81\x81R`\x98` R`@\x80\x82 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x92\x84\x16\x92\x7FDJ\x84\xF5\x12\x81j\xE7\xBE\x8E\xD8\xA6j\xA8\x8E6.\xB5M\t\x88\xE8:\xCC\x9D\x81tf\"\xB3\xBAQ\x91\x90\xA3PPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0EaW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x06\xB1V[`\0\x80a\x0C\xAF\x84\x84`\x01a%\xCEV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x98` R`@\x90 T`\xFF\x16\x15a\x08?W`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x81\x81R`\x98` R`@\x80\x82 \x80T`\xFF\x19\x16\x90UQ\x7F\xD4\xCE\xF0\xAF'\x80\rFo\xCA\xCD\x85w\x98W7\x8B\x85\xCBaV\x90\x05\xFF\x14d\xFAn\\\xEDi\xD8\x91\x90\xA2PV[`\0\x80`\0a!\xA3\x85\x85a%\x06V[a!\xB5WP`\0\x91P\x81\x90P\x80a\x11\xABV[PPP`\0\x81\x81R`\x01\x83\x81\x01` \x90\x81R`@\x80\x84 \x84\x80R\x90\x91R\x80\x83 T\x82\x84R\x92 T\x90\x91\x90a\x11\xABV[`\0\x81\x15a#\x8EW`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x99` R`@\x90 a\"\x0E\x90\x83a%\x06V[a\"\x16WP`\x01[\x80\x15\x80\x15a\"XWP`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\0\x90\x81R`\x97` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R Tc\xFF\xFF\xFF\xFF\x84\x81\x16`\x01`@\x1B\x90\x92\x04\x16\x11[\x15a\"aWP`\x01[\x80a\"\xD8W`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x99` R`@\x81 \x81\x90a\"\x8A\x90\x85a!\x1BV[\x91P\x91P\x81\x15a\"\xD5W`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\0\x90\x81R`\x97` \x90\x81R`@\x80\x83 \x93\x85\x16\x83R\x92\x90R Tc\xFF\xFF\xFF\xFF\x86\x81\x16`\x01`@\x1B\x90\x92\x04\x16\x11a\"\xD5W`\x01\x92P[PP[\x80a#wW`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x99` R`@\x90 a#\0\x90\x833a&\x17V[a#rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R\x7FSlasher.recordStakeUpdate: Inser`D\x82\x01R\x7Fting middleware unsuccessful\0\0\0\0`d\x82\x01R`\x84\x01a\x06\xB1V[a\n\xA5V[a#\x81\x84\x84a\x0EcV[\x91Pa#r\x84\x84\x84a!\xE4V[`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\0\x90\x81R`\x97` \x90\x81R`@\x80\x83 `\x99\x83R\x81\x84 \x84\x80R`\x01\x90\x81\x01\x84R\x82\x85 \x90\x85R\x83R\x81\x84 T\x90\x94\x16\x83R\x92\x90R Tc\xFF\xFF\xFF\xFF\x84\x81\x16`\x01`@\x1B\x90\x92\x04\x16\x11a#\xEBWP`\x01[\x80a$\x84Wa$\x123`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x99` R`@\x90 \x90a&&V[a#rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FSlasher.recordStakeUpdate: Prepp`D\x82\x01R\x7Fending middleware unsuccessful\0\0`d\x82\x01R`\x84\x01a\x06\xB1V[a$\x8E\x84\x84a\x0EcV[\x91Pa\n\xA5\x84\x84\x84a!\xE4V[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\0\x81\x81R`\x97` \x90\x81R`@\x80\x83 \x94\x86\x16\x80\x84R\x94\x90\x91R\x80\x82 \x80Tg\xFF\xFF\xFF\xFF\0\0\0\0\x19\x16g\xFF\xFF\xFF\xFF\0\0\0\0\x17\x90UQ\x7F\xEF\xA9\xFB8\xE8\x13\xD5<\x15\xED\xF5\x01\xE08R\x84:?\xEDi\x19`R3\x91\xD7\x1A\t+6'\xD8\x91\x90\xA3PPV[`\0\x81\x81R`\x01\x83\x01` \x90\x81R`@\x80\x83 \x83\x80R\x90\x91R\x81 T\x15\x80\x15a%IWP`\0\x82\x81R`\x01\x80\x85\x01` \x90\x81R`@\x80\x84 \x92\x84R\x91\x90R\x90 T\x15[\x15a%zW`\0\x80\x80R`\x01\x80\x85\x01` \x90\x81R`@\x80\x84 \x92\x84R\x91\x90R\x90 T\x82\x14\x15a\x14IWP`\x01a\x0C\x0EV[P`\x01a\x0C\x0EV[`\0\x82\x81R`\x01\x90\x94\x01` \x81\x81R`@\x80\x87 \x93\x15\x80\x88R\x93\x82R\x80\x87 \x86\x90U\x94\x86R\x90\x81R\x83\x85 \x91\x15\x85RR\x91 UV[`\0a%\xC6\x84`\0\x85\x85a&4V[\x94\x93PPPPV[`\0\x80a%\xDB\x85\x85a%\x06V[a%\xEAWP`\0\x90P\x80a&\x0FV[PP`\0\x82\x81R`\x01\x84\x01` \x90\x81R`@\x80\x83 \x84\x15\x15\x84R\x90\x91R\x90 T\x80\x15\x15\x90[\x93P\x93\x91PPV[`\0a%\xC6\x84\x84\x84`\x01a&4V[`\0a\x11h\x83\x83`\x01a%\xB7V[`\0a&@\x85\x84a%\x06V[\x15\x80\x15a&RWPa&R\x85\x85a%\x06V[\x15a&\xB2W`\0\x84\x81R`\x01\x86\x01` \x90\x81R`@\x80\x83 \x85\x15\x15\x84R\x90\x91R\x90 Ta&\x81\x86\x86\x86\x86a%\x82V[a&\x8D\x86\x85\x83\x86a%\x82V[`\x01\x86`\0\x01`\0\x82\x82Ta&\xA2\x91\x90a*\xDBV[\x90\x91UP`\x01\x92Pa%\xC6\x91PPV[P`\0\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x08?W`\0\x80\xFD[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x15\xC2W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a&\xF9W`\0\x80\xFD[\x825a'\x04\x81a&\xBDV[\x91Pa'\x12` \x84\x01a&\xD2V[\x90P\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a'-W`\0\x80\xFD[\x815a\x11h\x81a&\xBDV[`\0` \x82\x84\x03\x12\x15a'JW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a'fW`\0\x80\xFD[\x835a'q\x81a&\xBDV[\x92P` \x84\x015a'\x81\x81a&\xBDV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0\x80`@\x83\x85\x03\x12\x15a'\xA5W`\0\x80\xFD[\x825a'\xB0\x81a&\xBDV[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a'\xD0W`\0\x80\xFD[\x815`\xFF\x81\x16\x81\x14a\x11hW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a'\xF4W`\0\x80\xFD[\x825a'\xFF\x81a&\xBDV[\x91P` \x83\x015a(\x0F\x81a&\xBDV[\x80\x91PP\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a(-W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a(EW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a(YW`\0\x80\xFD[\x815\x81\x81\x11\x15a(hW`\0\x80\xFD[\x86` \x82`\x05\x1B\x85\x01\x01\x11\x15a(}W`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a(\xA4W`\0\x80\xFD[\x835a(\xAF\x81a&\xBDV[\x92Pa(\xBD` \x85\x01a&\xD2V[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a(\xE3W`\0\x80\xFD[\x845a(\xEE\x81a&\xBDV[\x93Pa(\xFC` \x86\x01a&\xD2V[\x92Pa)\n`@\x86\x01a&\xD2V[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[` \x80\x82R`P\x90\x82\x01R\x7FSlasher.onlyRegisteredForService`@\x82\x01R\x7F: Operator has not opted into sl``\x82\x01Ro0\xB9\xB44\xB73\x901<\x901\xB0\xB662\xB9`\x81\x1B`\x80\x82\x01R`\xA0\x01\x90V[`\0` \x82\x84\x03\x12\x15a)\xA2W`\0\x80\xFD[\x81Qa\x11h\x81a&\xBDV[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15a*\tW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x11hW`\0\x80\xFD[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`\x19\x90\x82\x01R\x7FPausable: index is paused\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a*\xD6Wa*\xD6a*\xAEV[P\x03\x90V[`\0\x82\x19\x82\x11\x15a*\xEEWa*\xEEa*\xAEV[P\x01\x90V\xFE\xA2dipfsX\"\x12 J\xC4`\x8B \xA2\xE5\xB5!B\x9D\xA0\x81\xB6\r\xD8\xAC<\xB0\x97\xF0\xA2j\xC1J\xF3\xA9\x1D\xAB\x9B\x03edsolcC\0\x08\x0C\x003";
    /// The deployed bytecode of the contract.
    pub static SLASHER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct Slasher<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Slasher<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Slasher<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Slasher<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Slasher<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Slasher))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Slasher<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                SLASHER_ABI.clone(),
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
                SLASHER_ABI.clone(),
                SLASHER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `canSlash` (0xd98128c0) function
        pub fn can_slash(
            &self,
            to_be_slashed: ::ethers::core::types::Address,
            slashing_contract: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([217, 129, 40, 192], (to_be_slashed, slashing_contract))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `canWithdraw` (0x8105e043) function
        pub fn can_withdraw(
            &self,
            operator: ::ethers::core::types::Address,
            withdrawal_start_block: u32,
            middleware_times_index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [129, 5, 224, 67],
                    (operator, withdrawal_start_block, middleware_times_index),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `contractCanSlashOperatorUntilBlock` (0x6f0c2f74) function
        pub fn contract_can_slash_operator_until_block(
            &self,
            operator: ::ethers::core::types::Address,
            service_contract: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([111, 12, 47, 116], (operator, service_contract))
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
        ///Calls the contract's `freezeOperator` (0x38c8ee64) function
        pub fn freeze_operator(
            &self,
            to_be_frozen: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([56, 200, 238, 100], to_be_frozen)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCorrectValueForInsertAfter` (0x723e59c7) function
        pub fn get_correct_value_for_insert_after(
            &self,
            operator: ::ethers::core::types::Address,
            update_block: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([114, 62, 89, 199], (operator, update_block))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMiddlewareTimesIndexServeUntilBlock` (0x7259a45c) function
        pub fn get_middleware_times_index_serve_until_block(
            &self,
            operator: ::ethers::core::types::Address,
            index: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([114, 89, 164, 92], (operator, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMiddlewareTimesIndexStalestUpdateBlock` (0x1874e5ae) function
        pub fn get_middleware_times_index_stalest_update_block(
            &self,
            operator: ::ethers::core::types::Address,
            index: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([24, 116, 229, 174], (operator, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPreviousWhitelistedContractByUpdate` (0x35571676) function
        pub fn get_previous_whitelisted_contract_by_update(
            &self,
            operator: ::ethers::core::types::Address,
            node: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, (bool, ::ethers::core::types::U256)>
        {
            self.0
                .method_hash([53, 87, 22, 118], (operator, node))
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
        ///Calls the contract's `isFrozen` (0xe5839836) function
        pub fn is_frozen(
            &self,
            staker: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([229, 131, 152, 54], staker)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `latestUpdateBlock` (0xda16e29b) function
        pub fn latest_update_block(
            &self,
            operator: ::ethers::core::types::Address,
            service_contract: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([218, 22, 226, 155], (operator, service_contract))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `middlewareTimesLength` (0xa49db732) function
        pub fn middleware_times_length(
            &self,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([164, 157, 183, 50], operator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operatorToMiddlewareTimes` (0x282670fc) function
        pub fn operator_to_middleware_times(
            &self,
            operator: ::ethers::core::types::Address,
            array_index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, MiddlewareTimes> {
            self.0
                .method_hash([40, 38, 112, 252], (operator, array_index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operatorWhitelistedContractsLinkedListEntry` (0x855fcc4a) function
        pub fn operator_whitelisted_contracts_linked_list_entry(
            &self,
            operator: ::ethers::core::types::Address,
            node: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                bool,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([133, 95, 204, 74], (operator, node))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operatorWhitelistedContractsLinkedListSize` (0xe921d4fa) function
        pub fn operator_whitelisted_contracts_linked_list_size(
            &self,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([233, 33, 212, 250], operator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `optIntoSlashing` (0xf73b7519) function
        pub fn opt_into_slashing(
            &self,
            contract_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([247, 59, 117, 25], contract_address)
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
        ///Calls the contract's `recordFirstStakeUpdate` (0x175d3205) function
        pub fn record_first_stake_update(
            &self,
            operator: ::ethers::core::types::Address,
            serve_until_block: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([23, 93, 50, 5], (operator, serve_until_block))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `recordLastStakeUpdateAndRevokeSlashingAbility` (0x0ffabbce) function
        pub fn record_last_stake_update_and_revoke_slashing_ability(
            &self,
            operator: ::ethers::core::types::Address,
            serve_until_block: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([15, 250, 187, 206], (operator, serve_until_block))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `recordStakeUpdate` (0xc747075b) function
        pub fn record_stake_update(
            &self,
            operator: ::ethers::core::types::Address,
            update_block: u32,
            serve_until_block: u32,
            insert_after: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [199, 71, 7, 91],
                    (operator, update_block, serve_until_block, insert_after),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `resetFrozenStatus` (0x7cf72bba) function
        pub fn reset_frozen_status(
            &self,
            frozen_addresses: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([124, 247, 43, 186], frozen_addresses)
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
        ///Calls the contract's `strategyManager` (0x39b70e38) function
        pub fn strategy_manager(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
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
        ///Calls the contract's `unpause` (0xfabc1cbc) function
        pub fn unpause(
            &self,
            new_paused_status: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 188, 28, 188], new_paused_status)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `whitelistedContractDetails` (0xd7b7fa13) function
        pub fn whitelisted_contract_details(
            &self,
            operator: ::ethers::core::types::Address,
            service_contract: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, MiddlewareDetails> {
            self.0
                .method_hash([215, 183, 250, 19], (operator, service_contract))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `FrozenStatusReset` event
        pub fn frozen_status_reset_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, FrozenStatusResetFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Initialized` event
        pub fn initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, InitializedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `MiddlewareTimesAdded` event
        pub fn middleware_times_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, MiddlewareTimesAddedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `OperatorFrozen` event
        pub fn operator_frozen_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OperatorFrozenFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `OptedIntoSlashing` event
        pub fn opted_into_slashing_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OptedIntoSlashingFilter>
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
        ///Gets the contract's `SlashingAbilityRevoked` event
        pub fn slashing_ability_revoked_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SlashingAbilityRevokedFilter>
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
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SlasherEvents> {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for Slasher<M> {
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
    #[ethevent(name = "FrozenStatusReset", abi = "FrozenStatusReset(address)")]
    pub struct FrozenStatusResetFilter {
        #[ethevent(indexed)]
        pub previously_slashed_address: ::ethers::core::types::Address,
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
        name = "MiddlewareTimesAdded",
        abi = "MiddlewareTimesAdded(address,uint256,uint32,uint32)"
    )]
    pub struct MiddlewareTimesAddedFilter {
        pub operator: ::ethers::core::types::Address,
        pub index: ::ethers::core::types::U256,
        pub stalest_update_block: u32,
        pub latest_serve_until_block: u32,
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
    #[ethevent(name = "OperatorFrozen", abi = "OperatorFrozen(address,address)")]
    pub struct OperatorFrozenFilter {
        #[ethevent(indexed)]
        pub slashed_operator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub slashing_contract: ::ethers::core::types::Address,
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
    #[ethevent(name = "OptedIntoSlashing", abi = "OptedIntoSlashing(address,address)")]
    pub struct OptedIntoSlashingFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub contract_address: ::ethers::core::types::Address,
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
    #[ethevent(
        name = "SlashingAbilityRevoked",
        abi = "SlashingAbilityRevoked(address,address,uint32)"
    )]
    pub struct SlashingAbilityRevokedFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub contract_address: ::ethers::core::types::Address,
        pub contract_can_slash_operator_until_block: u32,
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
    pub enum SlasherEvents {
        FrozenStatusResetFilter(FrozenStatusResetFilter),
        InitializedFilter(InitializedFilter),
        MiddlewareTimesAddedFilter(MiddlewareTimesAddedFilter),
        OperatorFrozenFilter(OperatorFrozenFilter),
        OptedIntoSlashingFilter(OptedIntoSlashingFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PausedFilter(PausedFilter),
        PauserRegistrySetFilter(PauserRegistrySetFilter),
        SlashingAbilityRevokedFilter(SlashingAbilityRevokedFilter),
        UnpausedFilter(UnpausedFilter),
    }
    impl ::ethers::contract::EthLogDecode for SlasherEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = FrozenStatusResetFilter::decode_log(log) {
                return Ok(SlasherEvents::FrozenStatusResetFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(SlasherEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = MiddlewareTimesAddedFilter::decode_log(log) {
                return Ok(SlasherEvents::MiddlewareTimesAddedFilter(decoded));
            }
            if let Ok(decoded) = OperatorFrozenFilter::decode_log(log) {
                return Ok(SlasherEvents::OperatorFrozenFilter(decoded));
            }
            if let Ok(decoded) = OptedIntoSlashingFilter::decode_log(log) {
                return Ok(SlasherEvents::OptedIntoSlashingFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(SlasherEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(SlasherEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = PauserRegistrySetFilter::decode_log(log) {
                return Ok(SlasherEvents::PauserRegistrySetFilter(decoded));
            }
            if let Ok(decoded) = SlashingAbilityRevokedFilter::decode_log(log) {
                return Ok(SlasherEvents::SlashingAbilityRevokedFilter(decoded));
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(SlasherEvents::UnpausedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for SlasherEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::FrozenStatusResetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MiddlewareTimesAddedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorFrozenFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OptedIntoSlashingFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PausedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauserRegistrySetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SlashingAbilityRevokedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnpausedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<FrozenStatusResetFilter> for SlasherEvents {
        fn from(value: FrozenStatusResetFilter) -> Self {
            Self::FrozenStatusResetFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for SlasherEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<MiddlewareTimesAddedFilter> for SlasherEvents {
        fn from(value: MiddlewareTimesAddedFilter) -> Self {
            Self::MiddlewareTimesAddedFilter(value)
        }
    }
    impl ::core::convert::From<OperatorFrozenFilter> for SlasherEvents {
        fn from(value: OperatorFrozenFilter) -> Self {
            Self::OperatorFrozenFilter(value)
        }
    }
    impl ::core::convert::From<OptedIntoSlashingFilter> for SlasherEvents {
        fn from(value: OptedIntoSlashingFilter) -> Self {
            Self::OptedIntoSlashingFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for SlasherEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<PausedFilter> for SlasherEvents {
        fn from(value: PausedFilter) -> Self {
            Self::PausedFilter(value)
        }
    }
    impl ::core::convert::From<PauserRegistrySetFilter> for SlasherEvents {
        fn from(value: PauserRegistrySetFilter) -> Self {
            Self::PauserRegistrySetFilter(value)
        }
    }
    impl ::core::convert::From<SlashingAbilityRevokedFilter> for SlasherEvents {
        fn from(value: SlashingAbilityRevokedFilter) -> Self {
            Self::SlashingAbilityRevokedFilter(value)
        }
    }
    impl ::core::convert::From<UnpausedFilter> for SlasherEvents {
        fn from(value: UnpausedFilter) -> Self {
            Self::UnpausedFilter(value)
        }
    }
    ///Container type for all input parameters for the `canSlash` function with signature `canSlash(address,address)` and selector `0xd98128c0`
    #[derive(
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
    #[ethcall(name = "canSlash", abi = "canSlash(address,address)")]
    pub struct CanSlashCall {
        pub to_be_slashed: ::ethers::core::types::Address,
        pub slashing_contract: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `canWithdraw` function with signature `canWithdraw(address,uint32,uint256)` and selector `0x8105e043`
    #[derive(
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
    #[ethcall(name = "canWithdraw", abi = "canWithdraw(address,uint32,uint256)")]
    pub struct CanWithdrawCall {
        pub operator: ::ethers::core::types::Address,
        pub withdrawal_start_block: u32,
        pub middleware_times_index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `contractCanSlashOperatorUntilBlock` function with signature `contractCanSlashOperatorUntilBlock(address,address)` and selector `0x6f0c2f74`
    #[derive(
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
        name = "contractCanSlashOperatorUntilBlock",
        abi = "contractCanSlashOperatorUntilBlock(address,address)"
    )]
    pub struct ContractCanSlashOperatorUntilBlockCall {
        pub operator: ::ethers::core::types::Address,
        pub service_contract: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `freezeOperator` function with signature `freezeOperator(address)` and selector `0x38c8ee64`
    #[derive(
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
    #[ethcall(name = "freezeOperator", abi = "freezeOperator(address)")]
    pub struct FreezeOperatorCall {
        pub to_be_frozen: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getCorrectValueForInsertAfter` function with signature `getCorrectValueForInsertAfter(address,uint32)` and selector `0x723e59c7`
    #[derive(
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
        name = "getCorrectValueForInsertAfter",
        abi = "getCorrectValueForInsertAfter(address,uint32)"
    )]
    pub struct GetCorrectValueForInsertAfterCall {
        pub operator: ::ethers::core::types::Address,
        pub update_block: u32,
    }
    ///Container type for all input parameters for the `getMiddlewareTimesIndexServeUntilBlock` function with signature `getMiddlewareTimesIndexServeUntilBlock(address,uint32)` and selector `0x7259a45c`
    #[derive(
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
        name = "getMiddlewareTimesIndexServeUntilBlock",
        abi = "getMiddlewareTimesIndexServeUntilBlock(address,uint32)"
    )]
    pub struct GetMiddlewareTimesIndexServeUntilBlockCall {
        pub operator: ::ethers::core::types::Address,
        pub index: u32,
    }
    ///Container type for all input parameters for the `getMiddlewareTimesIndexStalestUpdateBlock` function with signature `getMiddlewareTimesIndexStalestUpdateBlock(address,uint32)` and selector `0x1874e5ae`
    #[derive(
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
        name = "getMiddlewareTimesIndexStalestUpdateBlock",
        abi = "getMiddlewareTimesIndexStalestUpdateBlock(address,uint32)"
    )]
    pub struct GetMiddlewareTimesIndexStalestUpdateBlockCall {
        pub operator: ::ethers::core::types::Address,
        pub index: u32,
    }
    ///Container type for all input parameters for the `getPreviousWhitelistedContractByUpdate` function with signature `getPreviousWhitelistedContractByUpdate(address,uint256)` and selector `0x35571676`
    #[derive(
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
        name = "getPreviousWhitelistedContractByUpdate",
        abi = "getPreviousWhitelistedContractByUpdate(address,uint256)"
    )]
    pub struct GetPreviousWhitelistedContractByUpdateCall {
        pub operator: ::ethers::core::types::Address,
        pub node: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `isFrozen` function with signature `isFrozen(address)` and selector `0xe5839836`
    #[derive(
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
    #[ethcall(name = "isFrozen", abi = "isFrozen(address)")]
    pub struct IsFrozenCall {
        pub staker: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `latestUpdateBlock` function with signature `latestUpdateBlock(address,address)` and selector `0xda16e29b`
    #[derive(
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
    #[ethcall(name = "latestUpdateBlock", abi = "latestUpdateBlock(address,address)")]
    pub struct LatestUpdateBlockCall {
        pub operator: ::ethers::core::types::Address,
        pub service_contract: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `middlewareTimesLength` function with signature `middlewareTimesLength(address)` and selector `0xa49db732`
    #[derive(
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
    #[ethcall(name = "middlewareTimesLength", abi = "middlewareTimesLength(address)")]
    pub struct MiddlewareTimesLengthCall {
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `operatorToMiddlewareTimes` function with signature `operatorToMiddlewareTimes(address,uint256)` and selector `0x282670fc`
    #[derive(
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
        name = "operatorToMiddlewareTimes",
        abi = "operatorToMiddlewareTimes(address,uint256)"
    )]
    pub struct OperatorToMiddlewareTimesCall {
        pub operator: ::ethers::core::types::Address,
        pub array_index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `operatorWhitelistedContractsLinkedListEntry` function with signature `operatorWhitelistedContractsLinkedListEntry(address,address)` and selector `0x855fcc4a`
    #[derive(
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
        name = "operatorWhitelistedContractsLinkedListEntry",
        abi = "operatorWhitelistedContractsLinkedListEntry(address,address)"
    )]
    pub struct OperatorWhitelistedContractsLinkedListEntryCall {
        pub operator: ::ethers::core::types::Address,
        pub node: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `operatorWhitelistedContractsLinkedListSize` function with signature `operatorWhitelistedContractsLinkedListSize(address)` and selector `0xe921d4fa`
    #[derive(
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
        name = "operatorWhitelistedContractsLinkedListSize",
        abi = "operatorWhitelistedContractsLinkedListSize(address)"
    )]
    pub struct OperatorWhitelistedContractsLinkedListSizeCall {
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `optIntoSlashing` function with signature `optIntoSlashing(address)` and selector `0xf73b7519`
    #[derive(
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
    #[ethcall(name = "optIntoSlashing", abi = "optIntoSlashing(address)")]
    pub struct OptIntoSlashingCall {
        pub contract_address: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `recordFirstStakeUpdate` function with signature `recordFirstStakeUpdate(address,uint32)` and selector `0x175d3205`
    #[derive(
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
        name = "recordFirstStakeUpdate",
        abi = "recordFirstStakeUpdate(address,uint32)"
    )]
    pub struct RecordFirstStakeUpdateCall {
        pub operator: ::ethers::core::types::Address,
        pub serve_until_block: u32,
    }
    ///Container type for all input parameters for the `recordLastStakeUpdateAndRevokeSlashingAbility` function with signature `recordLastStakeUpdateAndRevokeSlashingAbility(address,uint32)` and selector `0x0ffabbce`
    #[derive(
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
        name = "recordLastStakeUpdateAndRevokeSlashingAbility",
        abi = "recordLastStakeUpdateAndRevokeSlashingAbility(address,uint32)"
    )]
    pub struct RecordLastStakeUpdateAndRevokeSlashingAbilityCall {
        pub operator: ::ethers::core::types::Address,
        pub serve_until_block: u32,
    }
    ///Container type for all input parameters for the `recordStakeUpdate` function with signature `recordStakeUpdate(address,uint32,uint32,uint256)` and selector `0xc747075b`
    #[derive(
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
        name = "recordStakeUpdate",
        abi = "recordStakeUpdate(address,uint32,uint32,uint256)"
    )]
    pub struct RecordStakeUpdateCall {
        pub operator: ::ethers::core::types::Address,
        pub update_block: u32,
        pub serve_until_block: u32,
        pub insert_after: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `resetFrozenStatus` function with signature `resetFrozenStatus(address[])` and selector `0x7cf72bba`
    #[derive(
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
    #[ethcall(name = "resetFrozenStatus", abi = "resetFrozenStatus(address[])")]
    pub struct ResetFrozenStatusCall {
        pub frozen_addresses: ::std::vec::Vec<::ethers::core::types::Address>,
    }
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
        Hash,
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
    ///Container type for all input parameters for the `whitelistedContractDetails` function with signature `whitelistedContractDetails(address,address)` and selector `0xd7b7fa13`
    #[derive(
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
        name = "whitelistedContractDetails",
        abi = "whitelistedContractDetails(address,address)"
    )]
    pub struct WhitelistedContractDetailsCall {
        pub operator: ::ethers::core::types::Address,
        pub service_contract: ::ethers::core::types::Address,
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
    pub enum SlasherCalls {
        CanSlash(CanSlashCall),
        CanWithdraw(CanWithdrawCall),
        ContractCanSlashOperatorUntilBlock(ContractCanSlashOperatorUntilBlockCall),
        Delegation(DelegationCall),
        FreezeOperator(FreezeOperatorCall),
        GetCorrectValueForInsertAfter(GetCorrectValueForInsertAfterCall),
        GetMiddlewareTimesIndexServeUntilBlock(GetMiddlewareTimesIndexServeUntilBlockCall),
        GetMiddlewareTimesIndexStalestUpdateBlock(GetMiddlewareTimesIndexStalestUpdateBlockCall),
        GetPreviousWhitelistedContractByUpdate(GetPreviousWhitelistedContractByUpdateCall),
        Initialize(InitializeCall),
        IsFrozen(IsFrozenCall),
        LatestUpdateBlock(LatestUpdateBlockCall),
        MiddlewareTimesLength(MiddlewareTimesLengthCall),
        OperatorToMiddlewareTimes(OperatorToMiddlewareTimesCall),
        OperatorWhitelistedContractsLinkedListEntry(
            OperatorWhitelistedContractsLinkedListEntryCall,
        ),
        OperatorWhitelistedContractsLinkedListSize(OperatorWhitelistedContractsLinkedListSizeCall),
        OptIntoSlashing(OptIntoSlashingCall),
        Owner(OwnerCall),
        Pause(PauseCall),
        PauseAll(PauseAllCall),
        PausedWithIndex(PausedWithIndexCall),
        Paused(PausedCall),
        PauserRegistry(PauserRegistryCall),
        RecordFirstStakeUpdate(RecordFirstStakeUpdateCall),
        RecordLastStakeUpdateAndRevokeSlashingAbility(
            RecordLastStakeUpdateAndRevokeSlashingAbilityCall,
        ),
        RecordStakeUpdate(RecordStakeUpdateCall),
        RenounceOwnership(RenounceOwnershipCall),
        ResetFrozenStatus(ResetFrozenStatusCall),
        SetPauserRegistry(SetPauserRegistryCall),
        StrategyManager(StrategyManagerCall),
        TransferOwnership(TransferOwnershipCall),
        Unpause(UnpauseCall),
        WhitelistedContractDetails(WhitelistedContractDetailsCall),
    }
    impl ::ethers::core::abi::AbiDecode for SlasherCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CanSlashCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CanSlash(decoded));
            }
            if let Ok(decoded) = <CanWithdrawCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CanWithdraw(decoded));
            }
            if let Ok(decoded) =
                <ContractCanSlashOperatorUntilBlockCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::ContractCanSlashOperatorUntilBlock(decoded));
            }
            if let Ok(decoded) = <DelegationCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Delegation(decoded));
            }
            if let Ok(decoded) =
                <FreezeOperatorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FreezeOperator(decoded));
            }
            if let Ok(decoded) =
                <GetCorrectValueForInsertAfterCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetCorrectValueForInsertAfter(decoded));
            }
            if let Ok(decoded) = <GetMiddlewareTimesIndexServeUntilBlockCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetMiddlewareTimesIndexServeUntilBlock(decoded));
            }
            if let Ok(decoded) = <GetMiddlewareTimesIndexStalestUpdateBlockCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetMiddlewareTimesIndexStalestUpdateBlock(decoded));
            }
            if let Ok(decoded) = <GetPreviousWhitelistedContractByUpdateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPreviousWhitelistedContractByUpdate(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <IsFrozenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsFrozen(decoded));
            }
            if let Ok(decoded) =
                <LatestUpdateBlockCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LatestUpdateBlock(decoded));
            }
            if let Ok(decoded) =
                <MiddlewareTimesLengthCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MiddlewareTimesLength(decoded));
            }
            if let Ok(decoded) =
                <OperatorToMiddlewareTimesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OperatorToMiddlewareTimes(decoded));
            }
            if let Ok(decoded) = <OperatorWhitelistedContractsLinkedListEntryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OperatorWhitelistedContractsLinkedListEntry(decoded));
            }
            if let Ok(decoded) = <OperatorWhitelistedContractsLinkedListSizeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OperatorWhitelistedContractsLinkedListSize(decoded));
            }
            if let Ok(decoded) =
                <OptIntoSlashingCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OptIntoSlashing(decoded));
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
                <RecordFirstStakeUpdateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RecordFirstStakeUpdate(decoded));
            }
            if let Ok(decoded) = <RecordLastStakeUpdateAndRevokeSlashingAbilityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RecordLastStakeUpdateAndRevokeSlashingAbility(decoded));
            }
            if let Ok(decoded) =
                <RecordStakeUpdateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RecordStakeUpdate(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <ResetFrozenStatusCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ResetFrozenStatus(decoded));
            }
            if let Ok(decoded) =
                <SetPauserRegistryCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetPauserRegistry(decoded));
            }
            if let Ok(decoded) =
                <StrategyManagerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::StrategyManager(decoded));
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
                <WhitelistedContractDetailsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WhitelistedContractDetails(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SlasherCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CanSlash(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CanWithdraw(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ContractCanSlashOperatorUntilBlock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Delegation(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FreezeOperator(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetCorrectValueForInsertAfter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetMiddlewareTimesIndexServeUntilBlock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetMiddlewareTimesIndexStalestUpdateBlock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPreviousWhitelistedContractByUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsFrozen(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LatestUpdateBlock(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MiddlewareTimesLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OperatorToMiddlewareTimes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OperatorWhitelistedContractsLinkedListEntry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OperatorWhitelistedContractsLinkedListSize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OptIntoSlashing(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PauseAll(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PausedWithIndex(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Paused(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PauserRegistry(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RecordFirstStakeUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RecordLastStakeUpdateAndRevokeSlashingAbility(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RecordStakeUpdate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ResetFrozenStatus(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetPauserRegistry(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StrategyManager(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Unpause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WhitelistedContractDetails(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for SlasherCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CanSlash(element) => ::core::fmt::Display::fmt(element, f),
                Self::CanWithdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::ContractCanSlashOperatorUntilBlock(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Delegation(element) => ::core::fmt::Display::fmt(element, f),
                Self::FreezeOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCorrectValueForInsertAfter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetMiddlewareTimesIndexServeUntilBlock(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetMiddlewareTimesIndexStalestUpdateBlock(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetPreviousWhitelistedContractByUpdate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsFrozen(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestUpdateBlock(element) => ::core::fmt::Display::fmt(element, f),
                Self::MiddlewareTimesLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorToMiddlewareTimes(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorWhitelistedContractsLinkedListEntry(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorWhitelistedContractsLinkedListSize(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OptIntoSlashing(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pause(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauseAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::PausedWithIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::Paused(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauserRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::RecordFirstStakeUpdate(element) => ::core::fmt::Display::fmt(element, f),
                Self::RecordLastStakeUpdateAndRevokeSlashingAbility(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RecordStakeUpdate(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::ResetFrozenStatus(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPauserRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::StrategyManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unpause(element) => ::core::fmt::Display::fmt(element, f),
                Self::WhitelistedContractDetails(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CanSlashCall> for SlasherCalls {
        fn from(value: CanSlashCall) -> Self {
            Self::CanSlash(value)
        }
    }
    impl ::core::convert::From<CanWithdrawCall> for SlasherCalls {
        fn from(value: CanWithdrawCall) -> Self {
            Self::CanWithdraw(value)
        }
    }
    impl ::core::convert::From<ContractCanSlashOperatorUntilBlockCall> for SlasherCalls {
        fn from(value: ContractCanSlashOperatorUntilBlockCall) -> Self {
            Self::ContractCanSlashOperatorUntilBlock(value)
        }
    }
    impl ::core::convert::From<DelegationCall> for SlasherCalls {
        fn from(value: DelegationCall) -> Self {
            Self::Delegation(value)
        }
    }
    impl ::core::convert::From<FreezeOperatorCall> for SlasherCalls {
        fn from(value: FreezeOperatorCall) -> Self {
            Self::FreezeOperator(value)
        }
    }
    impl ::core::convert::From<GetCorrectValueForInsertAfterCall> for SlasherCalls {
        fn from(value: GetCorrectValueForInsertAfterCall) -> Self {
            Self::GetCorrectValueForInsertAfter(value)
        }
    }
    impl ::core::convert::From<GetMiddlewareTimesIndexServeUntilBlockCall> for SlasherCalls {
        fn from(value: GetMiddlewareTimesIndexServeUntilBlockCall) -> Self {
            Self::GetMiddlewareTimesIndexServeUntilBlock(value)
        }
    }
    impl ::core::convert::From<GetMiddlewareTimesIndexStalestUpdateBlockCall> for SlasherCalls {
        fn from(value: GetMiddlewareTimesIndexStalestUpdateBlockCall) -> Self {
            Self::GetMiddlewareTimesIndexStalestUpdateBlock(value)
        }
    }
    impl ::core::convert::From<GetPreviousWhitelistedContractByUpdateCall> for SlasherCalls {
        fn from(value: GetPreviousWhitelistedContractByUpdateCall) -> Self {
            Self::GetPreviousWhitelistedContractByUpdate(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for SlasherCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<IsFrozenCall> for SlasherCalls {
        fn from(value: IsFrozenCall) -> Self {
            Self::IsFrozen(value)
        }
    }
    impl ::core::convert::From<LatestUpdateBlockCall> for SlasherCalls {
        fn from(value: LatestUpdateBlockCall) -> Self {
            Self::LatestUpdateBlock(value)
        }
    }
    impl ::core::convert::From<MiddlewareTimesLengthCall> for SlasherCalls {
        fn from(value: MiddlewareTimesLengthCall) -> Self {
            Self::MiddlewareTimesLength(value)
        }
    }
    impl ::core::convert::From<OperatorToMiddlewareTimesCall> for SlasherCalls {
        fn from(value: OperatorToMiddlewareTimesCall) -> Self {
            Self::OperatorToMiddlewareTimes(value)
        }
    }
    impl ::core::convert::From<OperatorWhitelistedContractsLinkedListEntryCall> for SlasherCalls {
        fn from(value: OperatorWhitelistedContractsLinkedListEntryCall) -> Self {
            Self::OperatorWhitelistedContractsLinkedListEntry(value)
        }
    }
    impl ::core::convert::From<OperatorWhitelistedContractsLinkedListSizeCall> for SlasherCalls {
        fn from(value: OperatorWhitelistedContractsLinkedListSizeCall) -> Self {
            Self::OperatorWhitelistedContractsLinkedListSize(value)
        }
    }
    impl ::core::convert::From<OptIntoSlashingCall> for SlasherCalls {
        fn from(value: OptIntoSlashingCall) -> Self {
            Self::OptIntoSlashing(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for SlasherCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PauseCall> for SlasherCalls {
        fn from(value: PauseCall) -> Self {
            Self::Pause(value)
        }
    }
    impl ::core::convert::From<PauseAllCall> for SlasherCalls {
        fn from(value: PauseAllCall) -> Self {
            Self::PauseAll(value)
        }
    }
    impl ::core::convert::From<PausedWithIndexCall> for SlasherCalls {
        fn from(value: PausedWithIndexCall) -> Self {
            Self::PausedWithIndex(value)
        }
    }
    impl ::core::convert::From<PausedCall> for SlasherCalls {
        fn from(value: PausedCall) -> Self {
            Self::Paused(value)
        }
    }
    impl ::core::convert::From<PauserRegistryCall> for SlasherCalls {
        fn from(value: PauserRegistryCall) -> Self {
            Self::PauserRegistry(value)
        }
    }
    impl ::core::convert::From<RecordFirstStakeUpdateCall> for SlasherCalls {
        fn from(value: RecordFirstStakeUpdateCall) -> Self {
            Self::RecordFirstStakeUpdate(value)
        }
    }
    impl ::core::convert::From<RecordLastStakeUpdateAndRevokeSlashingAbilityCall> for SlasherCalls {
        fn from(value: RecordLastStakeUpdateAndRevokeSlashingAbilityCall) -> Self {
            Self::RecordLastStakeUpdateAndRevokeSlashingAbility(value)
        }
    }
    impl ::core::convert::From<RecordStakeUpdateCall> for SlasherCalls {
        fn from(value: RecordStakeUpdateCall) -> Self {
            Self::RecordStakeUpdate(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for SlasherCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<ResetFrozenStatusCall> for SlasherCalls {
        fn from(value: ResetFrozenStatusCall) -> Self {
            Self::ResetFrozenStatus(value)
        }
    }
    impl ::core::convert::From<SetPauserRegistryCall> for SlasherCalls {
        fn from(value: SetPauserRegistryCall) -> Self {
            Self::SetPauserRegistry(value)
        }
    }
    impl ::core::convert::From<StrategyManagerCall> for SlasherCalls {
        fn from(value: StrategyManagerCall) -> Self {
            Self::StrategyManager(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for SlasherCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UnpauseCall> for SlasherCalls {
        fn from(value: UnpauseCall) -> Self {
            Self::Unpause(value)
        }
    }
    impl ::core::convert::From<WhitelistedContractDetailsCall> for SlasherCalls {
        fn from(value: WhitelistedContractDetailsCall) -> Self {
            Self::WhitelistedContractDetails(value)
        }
    }
    ///Container type for all return fields from the `canSlash` function with signature `canSlash(address,address)` and selector `0xd98128c0`
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
    pub struct CanSlashReturn(pub bool);
    ///Container type for all return fields from the `canWithdraw` function with signature `canWithdraw(address,uint32,uint256)` and selector `0x8105e043`
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
    pub struct CanWithdrawReturn(pub bool);
    ///Container type for all return fields from the `contractCanSlashOperatorUntilBlock` function with signature `contractCanSlashOperatorUntilBlock(address,address)` and selector `0x6f0c2f74`
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
    pub struct ContractCanSlashOperatorUntilBlockReturn(pub u32);
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
    ///Container type for all return fields from the `getCorrectValueForInsertAfter` function with signature `getCorrectValueForInsertAfter(address,uint32)` and selector `0x723e59c7`
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
    pub struct GetCorrectValueForInsertAfterReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getMiddlewareTimesIndexServeUntilBlock` function with signature `getMiddlewareTimesIndexServeUntilBlock(address,uint32)` and selector `0x7259a45c`
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
    pub struct GetMiddlewareTimesIndexServeUntilBlockReturn(pub u32);
    ///Container type for all return fields from the `getMiddlewareTimesIndexStalestUpdateBlock` function with signature `getMiddlewareTimesIndexStalestUpdateBlock(address,uint32)` and selector `0x1874e5ae`
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
    pub struct GetMiddlewareTimesIndexStalestUpdateBlockReturn(pub u32);
    ///Container type for all return fields from the `getPreviousWhitelistedContractByUpdate` function with signature `getPreviousWhitelistedContractByUpdate(address,uint256)` and selector `0x35571676`
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
    pub struct GetPreviousWhitelistedContractByUpdateReturn(
        pub bool,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `isFrozen` function with signature `isFrozen(address)` and selector `0xe5839836`
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
    pub struct IsFrozenReturn(pub bool);
    ///Container type for all return fields from the `latestUpdateBlock` function with signature `latestUpdateBlock(address,address)` and selector `0xda16e29b`
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
    pub struct LatestUpdateBlockReturn(pub u32);
    ///Container type for all return fields from the `middlewareTimesLength` function with signature `middlewareTimesLength(address)` and selector `0xa49db732`
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
    pub struct MiddlewareTimesLengthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `operatorToMiddlewareTimes` function with signature `operatorToMiddlewareTimes(address,uint256)` and selector `0x282670fc`
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
    pub struct OperatorToMiddlewareTimesReturn(pub MiddlewareTimes);
    ///Container type for all return fields from the `operatorWhitelistedContractsLinkedListEntry` function with signature `operatorWhitelistedContractsLinkedListEntry(address,address)` and selector `0x855fcc4a`
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
    pub struct OperatorWhitelistedContractsLinkedListEntryReturn(
        pub bool,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `operatorWhitelistedContractsLinkedListSize` function with signature `operatorWhitelistedContractsLinkedListSize(address)` and selector `0xe921d4fa`
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
    pub struct OperatorWhitelistedContractsLinkedListSizeReturn(pub ::ethers::core::types::U256);
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
        Hash,
    )]
    pub struct StrategyManagerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `whitelistedContractDetails` function with signature `whitelistedContractDetails(address,address)` and selector `0xd7b7fa13`
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
    pub struct WhitelistedContractDetailsReturn(pub MiddlewareDetails);
    ///`MiddlewareDetails(uint32,uint32,uint32)`
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
    pub struct MiddlewareDetails {
        pub registration_may_begin_at_block: u32,
        pub contract_can_slash_operator_until_block: u32,
        pub latest_update_block: u32,
    }
}
