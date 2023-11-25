pub use bribe::*;
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
pub mod bribe {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_voter"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DURATION"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DURATION"),
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
                    ::std::borrow::ToOwned::to_owned("_deposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("_deposit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("_withdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("_withdraw"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("addReward"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addReward"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_rewardsToken"),
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
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balanceOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("earned"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("earned"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_rewardsToken"),
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
                    ::std::borrow::ToOwned::to_owned("getReward"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getReward"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("getRewardForDuration"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getRewardForDuration",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_rewardsToken"),
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
                    ::std::borrow::ToOwned::to_owned("getRewardTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getRewardTokens"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isRewardToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isRewardToken"),
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
                    ::std::borrow::ToOwned::to_owned("lastTimeRewardApplicable"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "lastTimeRewardApplicable",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_rewardsToken"),
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
                    ::std::borrow::ToOwned::to_owned("left"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("left"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_rewardsToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("leftover"),
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
                    ::std::borrow::ToOwned::to_owned("notifyRewardAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("notifyRewardAmount"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_rewardsToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reward"),
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
                    ::std::borrow::ToOwned::to_owned("rewardData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rewardData"),
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
                                    name: ::std::borrow::ToOwned::to_owned("periodFinish"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("rewardRate"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("lastUpdateTime"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "rewardPerTokenStored",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("rewardPerToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rewardPerToken"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_rewardsToken"),
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
                    ::std::borrow::ToOwned::to_owned("rewardTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rewardTokens"),
                            inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("rewards"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rewards"),
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
                    ::std::borrow::ToOwned::to_owned("totalSupply"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("totalSupply"),
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
                    ::std::borrow::ToOwned::to_owned("userRewardPerTokenPaid"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "userRewardPerTokenPaid",
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
                    ::std::borrow::ToOwned::to_owned("voter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("voter"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Bribe__Deposited"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Bribe__Deposited"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("Bribe__RewardAdded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Bribe__RewardAdded"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("rewardToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Bribe__RewardNotified"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Bribe__RewardNotified",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("rewardToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("reward"),
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
                    ::std::borrow::ToOwned::to_owned("Bribe__RewardPaid"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Bribe__RewardPaid"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("rewardsToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("reward"),
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
                    ::std::borrow::ToOwned::to_owned("Bribe__Withdrawn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Bribe__Withdrawn"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Bribe__InvalidZeroInput"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Bribe__InvalidZeroInput",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Bribe__NotAuthorizedVoter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Bribe__NotAuthorizedVoter",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Bribe__NotRewardToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Bribe__NotRewardToken",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Bribe__RewardSmallerThanDuration"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Bribe__RewardSmallerThanDuration",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Bribe__RewardTokenAlreadyAdded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Bribe__RewardTokenAlreadyAdded",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static BRIBE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct Bribe<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Bribe<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Bribe<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Bribe<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Bribe<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Bribe)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Bribe<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    BRIBE_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `DURATION` (0x1be05289) function
        pub fn duration(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([27, 224, 82, 137], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_deposit` (0x463cd970) function
        pub fn deposit(
            &self,
            amount: ::ethers::core::types::U256,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([70, 60, 217, 112], (amount, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `_withdraw` (0x293311ab) function
        pub fn withdraw(
            &self,
            amount: ::ethers::core::types::U256,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([41, 51, 17, 171], (amount, account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addReward` (0x9c9b2e21) function
        pub fn add_reward(
            &self,
            rewards_token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([156, 155, 46, 33], rewards_token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `earned` (0x211dc32d) function
        pub fn earned(
            &self,
            account: ::ethers::core::types::Address,
            rewards_token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([33, 29, 195, 45], (account, rewards_token))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getReward` (0xc00007b0) function
        pub fn get_reward(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([192, 0, 7, 176], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRewardForDuration` (0xbcd11014) function
        pub fn get_reward_for_duration(
            &self,
            rewards_token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([188, 209, 16, 20], rewards_token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRewardTokens` (0xc4f59f9b) function
        pub fn get_reward_tokens(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([196, 245, 159, 155], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isRewardToken` (0xb5fd73f8) function
        pub fn is_reward_token(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([181, 253, 115, 248], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastTimeRewardApplicable` (0x638634ee) function
        pub fn last_time_reward_applicable(
            &self,
            rewards_token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([99, 134, 52, 238], rewards_token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `left` (0x99bcc052) function
        pub fn left(
            &self,
            rewards_token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([153, 188, 192, 82], rewards_token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `notifyRewardAmount` (0xb66503cf) function
        pub fn notify_reward_amount(
            &self,
            rewards_token: ::ethers::core::types::Address,
            reward: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([182, 101, 3, 207], (rewards_token, reward))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rewardData` (0x48e5d9f8) function
        pub fn reward_data(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([72, 229, 217, 248], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rewardPerToken` (0xf1229777) function
        pub fn reward_per_token(
            &self,
            rewards_token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([241, 34, 151, 119], rewards_token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rewardTokens` (0x7bb7bed1) function
        pub fn reward_tokens(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([123, 183, 190, 209], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rewards` (0xe70b9e27) function
        pub fn rewards(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([231, 11, 158, 39], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalSupply` (0x18160ddd) function
        pub fn total_supply(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `userRewardPerTokenPaid` (0x7035ab98) function
        pub fn user_reward_per_token_paid(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 53, 171, 152], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `voter` (0x46c96aac) function
        pub fn voter(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([70, 201, 106, 172], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Bribe__Deposited` event
        pub fn bribe_deposited_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BribeDepositedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Bribe__RewardAdded` event
        pub fn bribe_reward_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BribeRewardAddedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Bribe__RewardNotified` event
        pub fn bribe_reward_notified_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BribeRewardNotifiedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Bribe__RewardPaid` event
        pub fn bribe_reward_paid_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BribeRewardPaidFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Bribe__Withdrawn` event
        pub fn bribe_withdrawn_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BribeWithdrawnFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, BribeEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Bribe<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `Bribe__InvalidZeroInput` with signature `Bribe__InvalidZeroInput()` and selector `0x9a57cd34`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "Bribe__InvalidZeroInput", abi = "Bribe__InvalidZeroInput()")]
    pub struct Bribe__InvalidZeroInput;
    ///Custom Error type `Bribe__NotAuthorizedVoter` with signature `Bribe__NotAuthorizedVoter()` and selector `0x101f8673`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "Bribe__NotAuthorizedVoter", abi = "Bribe__NotAuthorizedVoter()")]
    pub struct Bribe__NotAuthorizedVoter;
    ///Custom Error type `Bribe__NotRewardToken` with signature `Bribe__NotRewardToken()` and selector `0x68536d16`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "Bribe__NotRewardToken", abi = "Bribe__NotRewardToken()")]
    pub struct Bribe__NotRewardToken;
    ///Custom Error type `Bribe__RewardSmallerThanDuration` with signature `Bribe__RewardSmallerThanDuration()` and selector `0x6f65e037`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "Bribe__RewardSmallerThanDuration",
        abi = "Bribe__RewardSmallerThanDuration()"
    )]
    pub struct Bribe__RewardSmallerThanDuration;
    ///Custom Error type `Bribe__RewardTokenAlreadyAdded` with signature `Bribe__RewardTokenAlreadyAdded()` and selector `0xc907d3ac`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "Bribe__RewardTokenAlreadyAdded",
        abi = "Bribe__RewardTokenAlreadyAdded()"
    )]
    pub struct Bribe__RewardTokenAlreadyAdded;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum BribeErrors {
        Bribe__InvalidZeroInput(Bribe__InvalidZeroInput),
        Bribe__NotAuthorizedVoter(Bribe__NotAuthorizedVoter),
        Bribe__NotRewardToken(Bribe__NotRewardToken),
        Bribe__RewardSmallerThanDuration(Bribe__RewardSmallerThanDuration),
        Bribe__RewardTokenAlreadyAdded(Bribe__RewardTokenAlreadyAdded),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for BribeErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded)
                = <Bribe__InvalidZeroInput as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::Bribe__InvalidZeroInput(decoded));
            }
            if let Ok(decoded)
                = <Bribe__NotAuthorizedVoter as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::Bribe__NotAuthorizedVoter(decoded));
            }
            if let Ok(decoded)
                = <Bribe__NotRewardToken as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::Bribe__NotRewardToken(decoded));
            }
            if let Ok(decoded)
                = <Bribe__RewardSmallerThanDuration as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::Bribe__RewardSmallerThanDuration(decoded));
            }
            if let Ok(decoded)
                = <Bribe__RewardTokenAlreadyAdded as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::Bribe__RewardTokenAlreadyAdded(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for BribeErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::Bribe__InvalidZeroInput(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Bribe__NotAuthorizedVoter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Bribe__NotRewardToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Bribe__RewardSmallerThanDuration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Bribe__RewardTokenAlreadyAdded(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for BribeErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <Bribe__InvalidZeroInput as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Bribe__NotAuthorizedVoter as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Bribe__NotRewardToken as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Bribe__RewardSmallerThanDuration as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Bribe__RewardTokenAlreadyAdded as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for BribeErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Bribe__InvalidZeroInput(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Bribe__NotAuthorizedVoter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Bribe__NotRewardToken(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Bribe__RewardSmallerThanDuration(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Bribe__RewardTokenAlreadyAdded(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for BribeErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<Bribe__InvalidZeroInput> for BribeErrors {
        fn from(value: Bribe__InvalidZeroInput) -> Self {
            Self::Bribe__InvalidZeroInput(value)
        }
    }
    impl ::core::convert::From<Bribe__NotAuthorizedVoter> for BribeErrors {
        fn from(value: Bribe__NotAuthorizedVoter) -> Self {
            Self::Bribe__NotAuthorizedVoter(value)
        }
    }
    impl ::core::convert::From<Bribe__NotRewardToken> for BribeErrors {
        fn from(value: Bribe__NotRewardToken) -> Self {
            Self::Bribe__NotRewardToken(value)
        }
    }
    impl ::core::convert::From<Bribe__RewardSmallerThanDuration> for BribeErrors {
        fn from(value: Bribe__RewardSmallerThanDuration) -> Self {
            Self::Bribe__RewardSmallerThanDuration(value)
        }
    }
    impl ::core::convert::From<Bribe__RewardTokenAlreadyAdded> for BribeErrors {
        fn from(value: Bribe__RewardTokenAlreadyAdded) -> Self {
            Self::Bribe__RewardTokenAlreadyAdded(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Bribe__Deposited", abi = "Bribe__Deposited(address,uint256)")]
    pub struct BribeDepositedFilter {
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Bribe__RewardAdded", abi = "Bribe__RewardAdded(address)")]
    pub struct BribeRewardAddedFilter {
        #[ethevent(indexed)]
        pub reward_token: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "Bribe__RewardNotified",
        abi = "Bribe__RewardNotified(address,uint256)"
    )]
    pub struct BribeRewardNotifiedFilter {
        #[ethevent(indexed)]
        pub reward_token: ::ethers::core::types::Address,
        pub reward: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "Bribe__RewardPaid",
        abi = "Bribe__RewardPaid(address,address,uint256)"
    )]
    pub struct BribeRewardPaidFilter {
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub rewards_token: ::ethers::core::types::Address,
        pub reward: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Bribe__Withdrawn", abi = "Bribe__Withdrawn(address,uint256)")]
    pub struct BribeWithdrawnFilter {
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum BribeEvents {
        BribeDepositedFilter(BribeDepositedFilter),
        BribeRewardAddedFilter(BribeRewardAddedFilter),
        BribeRewardNotifiedFilter(BribeRewardNotifiedFilter),
        BribeRewardPaidFilter(BribeRewardPaidFilter),
        BribeWithdrawnFilter(BribeWithdrawnFilter),
    }
    impl ::ethers::contract::EthLogDecode for BribeEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = BribeDepositedFilter::decode_log(log) {
                return Ok(BribeEvents::BribeDepositedFilter(decoded));
            }
            if let Ok(decoded) = BribeRewardAddedFilter::decode_log(log) {
                return Ok(BribeEvents::BribeRewardAddedFilter(decoded));
            }
            if let Ok(decoded) = BribeRewardNotifiedFilter::decode_log(log) {
                return Ok(BribeEvents::BribeRewardNotifiedFilter(decoded));
            }
            if let Ok(decoded) = BribeRewardPaidFilter::decode_log(log) {
                return Ok(BribeEvents::BribeRewardPaidFilter(decoded));
            }
            if let Ok(decoded) = BribeWithdrawnFilter::decode_log(log) {
                return Ok(BribeEvents::BribeWithdrawnFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for BribeEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BribeDepositedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BribeRewardAddedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BribeRewardNotifiedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BribeRewardPaidFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BribeWithdrawnFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<BribeDepositedFilter> for BribeEvents {
        fn from(value: BribeDepositedFilter) -> Self {
            Self::BribeDepositedFilter(value)
        }
    }
    impl ::core::convert::From<BribeRewardAddedFilter> for BribeEvents {
        fn from(value: BribeRewardAddedFilter) -> Self {
            Self::BribeRewardAddedFilter(value)
        }
    }
    impl ::core::convert::From<BribeRewardNotifiedFilter> for BribeEvents {
        fn from(value: BribeRewardNotifiedFilter) -> Self {
            Self::BribeRewardNotifiedFilter(value)
        }
    }
    impl ::core::convert::From<BribeRewardPaidFilter> for BribeEvents {
        fn from(value: BribeRewardPaidFilter) -> Self {
            Self::BribeRewardPaidFilter(value)
        }
    }
    impl ::core::convert::From<BribeWithdrawnFilter> for BribeEvents {
        fn from(value: BribeWithdrawnFilter) -> Self {
            Self::BribeWithdrawnFilter(value)
        }
    }
    ///Container type for all input parameters for the `DURATION` function with signature `DURATION()` and selector `0x1be05289`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "DURATION", abi = "DURATION()")]
    pub struct DurationCall;
    ///Container type for all input parameters for the `_deposit` function with signature `_deposit(uint256,address)` and selector `0x463cd970`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "_deposit", abi = "_deposit(uint256,address)")]
    pub struct DepositCall {
        pub amount: ::ethers::core::types::U256,
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `_withdraw` function with signature `_withdraw(uint256,address)` and selector `0x293311ab`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "_withdraw", abi = "_withdraw(uint256,address)")]
    pub struct WithdrawCall {
        pub amount: ::ethers::core::types::U256,
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `addReward` function with signature `addReward(address)` and selector `0x9c9b2e21`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "addReward", abi = "addReward(address)")]
    pub struct AddRewardCall {
        pub rewards_token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `earned` function with signature `earned(address,address)` and selector `0x211dc32d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "earned", abi = "earned(address,address)")]
    pub struct EarnedCall {
        pub account: ::ethers::core::types::Address,
        pub rewards_token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getReward` function with signature `getReward(address)` and selector `0xc00007b0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getReward", abi = "getReward(address)")]
    pub struct GetRewardCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getRewardForDuration` function with signature `getRewardForDuration(address)` and selector `0xbcd11014`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getRewardForDuration", abi = "getRewardForDuration(address)")]
    pub struct GetRewardForDurationCall {
        pub rewards_token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getRewardTokens` function with signature `getRewardTokens()` and selector `0xc4f59f9b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getRewardTokens", abi = "getRewardTokens()")]
    pub struct GetRewardTokensCall;
    ///Container type for all input parameters for the `isRewardToken` function with signature `isRewardToken(address)` and selector `0xb5fd73f8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "isRewardToken", abi = "isRewardToken(address)")]
    pub struct IsRewardTokenCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `lastTimeRewardApplicable` function with signature `lastTimeRewardApplicable(address)` and selector `0x638634ee`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "lastTimeRewardApplicable",
        abi = "lastTimeRewardApplicable(address)"
    )]
    pub struct LastTimeRewardApplicableCall {
        pub rewards_token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `left` function with signature `left(address)` and selector `0x99bcc052`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "left", abi = "left(address)")]
    pub struct LeftCall {
        pub rewards_token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `notifyRewardAmount` function with signature `notifyRewardAmount(address,uint256)` and selector `0xb66503cf`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "notifyRewardAmount", abi = "notifyRewardAmount(address,uint256)")]
    pub struct NotifyRewardAmountCall {
        pub rewards_token: ::ethers::core::types::Address,
        pub reward: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `rewardData` function with signature `rewardData(address)` and selector `0x48e5d9f8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "rewardData", abi = "rewardData(address)")]
    pub struct RewardDataCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `rewardPerToken` function with signature `rewardPerToken(address)` and selector `0xf1229777`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "rewardPerToken", abi = "rewardPerToken(address)")]
    pub struct RewardPerTokenCall {
        pub rewards_token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `rewardTokens` function with signature `rewardTokens(uint256)` and selector `0x7bb7bed1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "rewardTokens", abi = "rewardTokens(uint256)")]
    pub struct RewardTokensCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `rewards` function with signature `rewards(address,address)` and selector `0xe70b9e27`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "rewards", abi = "rewards(address,address)")]
    pub struct RewardsCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
    ///Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    ///Container type for all input parameters for the `userRewardPerTokenPaid` function with signature `userRewardPerTokenPaid(address,address)` and selector `0x7035ab98`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "userRewardPerTokenPaid",
        abi = "userRewardPerTokenPaid(address,address)"
    )]
    pub struct UserRewardPerTokenPaidCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
    ///Container type for all input parameters for the `voter` function with signature `voter()` and selector `0x46c96aac`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "voter", abi = "voter()")]
    pub struct VoterCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum BribeCalls {
        Duration(DurationCall),
        Deposit(DepositCall),
        Withdraw(WithdrawCall),
        AddReward(AddRewardCall),
        BalanceOf(BalanceOfCall),
        Earned(EarnedCall),
        GetReward(GetRewardCall),
        GetRewardForDuration(GetRewardForDurationCall),
        GetRewardTokens(GetRewardTokensCall),
        IsRewardToken(IsRewardTokenCall),
        LastTimeRewardApplicable(LastTimeRewardApplicableCall),
        Left(LeftCall),
        NotifyRewardAmount(NotifyRewardAmountCall),
        RewardData(RewardDataCall),
        RewardPerToken(RewardPerTokenCall),
        RewardTokens(RewardTokensCall),
        Rewards(RewardsCall),
        TotalSupply(TotalSupplyCall),
        UserRewardPerTokenPaid(UserRewardPerTokenPaidCall),
        Voter(VoterCall),
    }
    impl ::ethers::core::abi::AbiDecode for BribeCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <DurationCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Duration(decoded));
            }
            if let Ok(decoded)
                = <DepositCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Deposit(decoded));
            }
            if let Ok(decoded)
                = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Withdraw(decoded));
            }
            if let Ok(decoded)
                = <AddRewardCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddReward(decoded));
            }
            if let Ok(decoded)
                = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded)
                = <EarnedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Earned(decoded));
            }
            if let Ok(decoded)
                = <GetRewardCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetReward(decoded));
            }
            if let Ok(decoded)
                = <GetRewardForDurationCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetRewardForDuration(decoded));
            }
            if let Ok(decoded)
                = <GetRewardTokensCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetRewardTokens(decoded));
            }
            if let Ok(decoded)
                = <IsRewardTokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsRewardToken(decoded));
            }
            if let Ok(decoded)
                = <LastTimeRewardApplicableCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::LastTimeRewardApplicable(decoded));
            }
            if let Ok(decoded)
                = <LeftCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Left(decoded));
            }
            if let Ok(decoded)
                = <NotifyRewardAmountCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::NotifyRewardAmount(decoded));
            }
            if let Ok(decoded)
                = <RewardDataCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RewardData(decoded));
            }
            if let Ok(decoded)
                = <RewardPerTokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RewardPerToken(decoded));
            }
            if let Ok(decoded)
                = <RewardTokensCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RewardTokens(decoded));
            }
            if let Ok(decoded)
                = <RewardsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Rewards(decoded));
            }
            if let Ok(decoded)
                = <TotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TotalSupply(decoded));
            }
            if let Ok(decoded)
                = <UserRewardPerTokenPaidCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::UserRewardPerTokenPaid(decoded));
            }
            if let Ok(decoded)
                = <VoterCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Voter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for BribeCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Duration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddReward(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Earned(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetReward(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRewardForDuration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRewardTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsRewardToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LastTimeRewardApplicable(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Left(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NotifyRewardAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RewardData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RewardPerToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RewardTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Rewards(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UserRewardPerTokenPaid(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Voter(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for BribeCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Duration(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddReward(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Earned(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReward(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRewardForDuration(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetRewardTokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsRewardToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::LastTimeRewardApplicable(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Left(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotifyRewardAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RewardData(element) => ::core::fmt::Display::fmt(element, f),
                Self::RewardPerToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::RewardTokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::Rewards(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::UserRewardPerTokenPaid(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Voter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DurationCall> for BribeCalls {
        fn from(value: DurationCall) -> Self {
            Self::Duration(value)
        }
    }
    impl ::core::convert::From<DepositCall> for BribeCalls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for BribeCalls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    impl ::core::convert::From<AddRewardCall> for BribeCalls {
        fn from(value: AddRewardCall) -> Self {
            Self::AddReward(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for BribeCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<EarnedCall> for BribeCalls {
        fn from(value: EarnedCall) -> Self {
            Self::Earned(value)
        }
    }
    impl ::core::convert::From<GetRewardCall> for BribeCalls {
        fn from(value: GetRewardCall) -> Self {
            Self::GetReward(value)
        }
    }
    impl ::core::convert::From<GetRewardForDurationCall> for BribeCalls {
        fn from(value: GetRewardForDurationCall) -> Self {
            Self::GetRewardForDuration(value)
        }
    }
    impl ::core::convert::From<GetRewardTokensCall> for BribeCalls {
        fn from(value: GetRewardTokensCall) -> Self {
            Self::GetRewardTokens(value)
        }
    }
    impl ::core::convert::From<IsRewardTokenCall> for BribeCalls {
        fn from(value: IsRewardTokenCall) -> Self {
            Self::IsRewardToken(value)
        }
    }
    impl ::core::convert::From<LastTimeRewardApplicableCall> for BribeCalls {
        fn from(value: LastTimeRewardApplicableCall) -> Self {
            Self::LastTimeRewardApplicable(value)
        }
    }
    impl ::core::convert::From<LeftCall> for BribeCalls {
        fn from(value: LeftCall) -> Self {
            Self::Left(value)
        }
    }
    impl ::core::convert::From<NotifyRewardAmountCall> for BribeCalls {
        fn from(value: NotifyRewardAmountCall) -> Self {
            Self::NotifyRewardAmount(value)
        }
    }
    impl ::core::convert::From<RewardDataCall> for BribeCalls {
        fn from(value: RewardDataCall) -> Self {
            Self::RewardData(value)
        }
    }
    impl ::core::convert::From<RewardPerTokenCall> for BribeCalls {
        fn from(value: RewardPerTokenCall) -> Self {
            Self::RewardPerToken(value)
        }
    }
    impl ::core::convert::From<RewardTokensCall> for BribeCalls {
        fn from(value: RewardTokensCall) -> Self {
            Self::RewardTokens(value)
        }
    }
    impl ::core::convert::From<RewardsCall> for BribeCalls {
        fn from(value: RewardsCall) -> Self {
            Self::Rewards(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for BribeCalls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<UserRewardPerTokenPaidCall> for BribeCalls {
        fn from(value: UserRewardPerTokenPaidCall) -> Self {
            Self::UserRewardPerTokenPaid(value)
        }
    }
    impl ::core::convert::From<VoterCall> for BribeCalls {
        fn from(value: VoterCall) -> Self {
            Self::Voter(value)
        }
    }
    ///Container type for all return fields from the `DURATION` function with signature `DURATION()` and selector `0x1be05289`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DurationReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `earned` function with signature `earned(address,address)` and selector `0x211dc32d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EarnedReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getRewardForDuration` function with signature `getRewardForDuration(address)` and selector `0xbcd11014`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetRewardForDurationReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getRewardTokens` function with signature `getRewardTokens()` and selector `0xc4f59f9b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetRewardTokensReturn(
        pub ::std::vec::Vec<::ethers::core::types::Address>,
    );
    ///Container type for all return fields from the `isRewardToken` function with signature `isRewardToken(address)` and selector `0xb5fd73f8`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IsRewardTokenReturn(pub bool);
    ///Container type for all return fields from the `lastTimeRewardApplicable` function with signature `lastTimeRewardApplicable(address)` and selector `0x638634ee`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct LastTimeRewardApplicableReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `left` function with signature `left(address)` and selector `0x99bcc052`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct LeftReturn {
        pub leftover: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `rewardData` function with signature `rewardData(address)` and selector `0x48e5d9f8`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct RewardDataReturn {
        pub period_finish: ::ethers::core::types::U256,
        pub reward_rate: ::ethers::core::types::U256,
        pub last_update_time: ::ethers::core::types::U256,
        pub reward_per_token_stored: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `rewardPerToken` function with signature `rewardPerToken(address)` and selector `0xf1229777`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct RewardPerTokenReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `rewardTokens` function with signature `rewardTokens(uint256)` and selector `0x7bb7bed1`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct RewardTokensReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `rewards` function with signature `rewards(address,address)` and selector `0xe70b9e27`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct RewardsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct TotalSupplyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `userRewardPerTokenPaid` function with signature `userRewardPerTokenPaid(address,address)` and selector `0x7035ab98`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct UserRewardPerTokenPaidReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `voter` function with signature `voter()` and selector `0x46c96aac`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct VoterReturn(pub ::ethers::core::types::Address);
}
