pub use voter::*;
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
pub mod voter {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_VTOKEN"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_gaugefactory"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_bribefactory"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("OTOKEN"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("OTOKEN"),
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
                    ::std::borrow::ToOwned::to_owned("VTOKEN"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("VTOKEN"),
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
                    ::std::borrow::ToOwned::to_owned("addBribeReward"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addBribeReward"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_bribe"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_rewardToken"),
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
                    ::std::borrow::ToOwned::to_owned("addPlugin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addPlugin"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_plugin"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("bribefactory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("bribefactory"),
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
                    ::std::borrow::ToOwned::to_owned("bribes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("bribes"),
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
                    ::std::borrow::ToOwned::to_owned("claimBribes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("claimBribes"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_bribes"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("claimRewards"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("claimRewards"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_gauges"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("claimable"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("claimable"),
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
                    ::std::borrow::ToOwned::to_owned("distribute"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("distribute"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_gauge"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("distribute"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("start"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("finish"),
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
                    ::std::borrow::ToOwned::to_owned("distributeToBribes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("distributeToBribes"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_plugins"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("distro"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("distro"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("emitDeposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("emitDeposit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("emitWithdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("emitWithdraw"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("gaugefactory"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("gaugefactory"),
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
                    ::std::borrow::ToOwned::to_owned("gauges"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("gauges"),
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
                    ::std::borrow::ToOwned::to_owned("getPlugins"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPlugins"),
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
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_minter"),
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
                    ::std::borrow::ToOwned::to_owned("isAlive"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isAlive"),
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
                    ::std::borrow::ToOwned::to_owned("isGauge"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isGauge"),
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
                    ::std::borrow::ToOwned::to_owned("killGauge"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("killGauge"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_gauge"),
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
                    ::std::borrow::ToOwned::to_owned("lastVoted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("lastVoted"),
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
                    ::std::borrow::ToOwned::to_owned("length"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("length"),
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
                    ::std::borrow::ToOwned::to_owned("minter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("minter"),
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
                    ::std::borrow::ToOwned::to_owned("notifyRewardAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("notifyRewardAmount"),
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
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                    ::std::borrow::ToOwned::to_owned("pluginForGauge"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pluginForGauge"),
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
                    ::std::borrow::ToOwned::to_owned("pluginVote"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pluginVote"),
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
                    ::std::borrow::ToOwned::to_owned("plugins"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("plugins"),
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
                    ::std::borrow::ToOwned::to_owned("reset"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("reset"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("reviveGauge"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("reviveGauge"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_gauge"),
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
                    ::std::borrow::ToOwned::to_owned("totalWeight"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("totalWeight"),
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
                    ::std::borrow::ToOwned::to_owned("updateAll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateAll"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateFor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateFor"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_gauges"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateForRange"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateForRange"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("start"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("end"),
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
                    ::std::borrow::ToOwned::to_owned("updateGauge"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updateGauge"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_gauge"),
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
                    ::std::borrow::ToOwned::to_owned("usedWeights"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("usedWeights"),
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
                    ::std::borrow::ToOwned::to_owned("vote"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("vote"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_plugins"),
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
                                    name: ::std::borrow::ToOwned::to_owned("_weights"),
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
                    ::std::borrow::ToOwned::to_owned("votes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("votes"),
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
                    ::std::borrow::ToOwned::to_owned("weights"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("weights"),
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
            ]),
            events: ::core::convert::From::from([
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
                    ::std::borrow::ToOwned::to_owned("Voter__Abstained"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Voter__Abstained"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("weight"),
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
                    ::std::borrow::ToOwned::to_owned("Voter__BribeRewardAdded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Voter__BribeRewardAdded",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("bribe"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("reward"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Voter__Deposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Voter__Deposit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("plugin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("gauge"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
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
                    ::std::borrow::ToOwned::to_owned("Voter__DistributeReward"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Voter__DistributeReward",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("gauge"),
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
                    ::std::borrow::ToOwned::to_owned("Voter__GaugeCreated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Voter__GaugeCreated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("creator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("plugin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("gauge"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("bribe"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Voter__GaugeKilled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Voter__GaugeKilled"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("gauge"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Voter__GaugeRevived"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Voter__GaugeRevived",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("gauge"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Voter__NotifyReward"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Voter__NotifyReward",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("reward"),
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
                    ::std::borrow::ToOwned::to_owned("Voter__Voted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Voter__Voted"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("voter"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("weight"),
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
                    ::std::borrow::ToOwned::to_owned("Voter__Withdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Voter__Withdraw"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("plugin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("gauge"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
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
                    ::std::borrow::ToOwned::to_owned("Voter__AlreadyVotedThisEpoch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Voter__AlreadyVotedThisEpoch",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Voter__GaugeExists"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Voter__GaugeExists"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Voter__GaugeIsAlive"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Voter__GaugeIsAlive",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Voter__GaugeIsDead"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Voter__GaugeIsDead"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Voter__InvalidZeroAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Voter__InvalidZeroAddress",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Voter__NotAuthorizedGovernance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Voter__NotAuthorizedGovernance",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Voter__NotAuthorizedMinter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Voter__NotAuthorizedMinter",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Voter__NotGauge"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Voter__NotGauge"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Voter__NotMinter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Voter__NotMinter"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "Voter__PluginLengthNotEqualToWeightLength",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Voter__PluginLengthNotEqualToWeightLength",
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
    pub static VOTER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct Voter<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Voter<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Voter<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Voter<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Voter<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Voter)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Voter<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    VOTER_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `OTOKEN` (0xc544df0c) function
        pub fn otoken(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([197, 68, 223, 12], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `VTOKEN` (0xfb548427) function
        pub fn vtoken(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([251, 84, 132, 39], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addBribeReward` (0x7ceed4af) function
        pub fn add_bribe_reward(
            &self,
            bribe: ::ethers::core::types::Address,
            reward_token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([124, 238, 212, 175], (bribe, reward_token))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addPlugin` (0xd8867fc8) function
        pub fn add_plugin(
            &self,
            plugin: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([216, 134, 127, 200], plugin)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `bribefactory` (0x38752a9d) function
        pub fn bribefactory(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([56, 117, 42, 157], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `bribes` (0xa8c5d95a) function
        pub fn bribes(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([168, 197, 217, 90], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `claimBribes` (0x240f9774) function
        pub fn claim_bribes(
            &self,
            bribes: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([36, 15, 151, 116], bribes)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `claimRewards` (0xf9f031df) function
        pub fn claim_rewards(
            &self,
            gauges: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([249, 240, 49, 223], gauges)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `claimable` (0x402914f5) function
        pub fn claimable(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([64, 41, 20, 245], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `distribute` (0x63453ae1) function
        pub fn distribute(
            &self,
            gauge: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([99, 69, 58, 225], gauge)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `distribute` (0x7625391a) function
        pub fn distribute_with_start(
            &self,
            start: ::ethers::core::types::U256,
            finish: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([118, 37, 57, 26], (start, finish))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `distributeToBribes` (0x4978c512) function
        pub fn distribute_to_bribes(
            &self,
            plugins: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([73, 120, 197, 18], plugins)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `distro` (0x47b3c6ba) function
        pub fn distro(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([71, 179, 198, 186], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `emitDeposit` (0x28ba84ca) function
        pub fn emit_deposit(
            &self,
            account: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([40, 186, 132, 202], (account, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `emitWithdraw` (0xb014da21) function
        pub fn emit_withdraw(
            &self,
            account: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([176, 20, 218, 33], (account, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `gaugefactory` (0x68c3acb3) function
        pub fn gaugefactory(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([104, 195, 172, 179], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `gauges` (0xb9a09fd5) function
        pub fn gauges(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([185, 160, 159, 213], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPlugins` (0xa2d869b2) function
        pub fn get_plugins(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([162, 216, 105, 178], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0xc4d66de8) function
        pub fn initialize(
            &self,
            minter: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 214, 109, 232], minter)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isAlive` (0x1703e5f9) function
        pub fn is_alive(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([23, 3, 229, 249], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isGauge` (0xaa79979b) function
        pub fn is_gauge(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([170, 121, 151, 155], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `killGauge` (0x992a7933) function
        pub fn kill_gauge(
            &self,
            gauge: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([153, 42, 121, 51], gauge)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastVoted` (0x9a61df89) function
        pub fn last_voted(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([154, 97, 223, 137], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `length` (0x1f7b6d32) function
        pub fn length(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([31, 123, 109, 50], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `minter` (0x07546172) function
        pub fn minter(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([7, 84, 97, 114], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `notifyRewardAmount` (0x3c6b16ab) function
        pub fn notify_reward_amount(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([60, 107, 22, 171], amount)
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
        ///Calls the contract's `pluginForGauge` (0x6c60f246) function
        pub fn plugin_for_gauge(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([108, 96, 242, 70], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pluginVote` (0x773fac7d) function
        pub fn plugin_vote(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([119, 63, 172, 125], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `plugins` (0xf0a317eb) function
        pub fn plugins(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([240, 163, 23, 235], p0)
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
        ///Calls the contract's `reset` (0xd826f88f) function
        pub fn reset(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([216, 38, 248, 143], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `reviveGauge` (0x9f06247b) function
        pub fn revive_gauge(
            &self,
            gauge: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([159, 6, 36, 123], gauge)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalWeight` (0x96c82e57) function
        pub fn total_weight(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([150, 200, 46, 87], ())
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
        ///Calls the contract's `updateAll` (0x53d78693) function
        pub fn update_all(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([83, 215, 134, 147], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateFor` (0xd560b0d7) function
        pub fn update_for(
            &self,
            gauges: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 96, 176, 215], gauges)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateForRange` (0x9b6a9d72) function
        pub fn update_for_range(
            &self,
            start: ::ethers::core::types::U256,
            end: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([155, 106, 157, 114], (start, end))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateGauge` (0x6ecbe38a) function
        pub fn update_gauge(
            &self,
            gauge: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([110, 203, 227, 138], gauge)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `usedWeights` (0x002f8de4) function
        pub fn used_weights(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([0, 47, 141, 228], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `vote` (0x6f816a20) function
        pub fn vote(
            &self,
            plugins: ::std::vec::Vec<::ethers::core::types::Address>,
            weights: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([111, 129, 106, 32], (plugins, weights))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `votes` (0xcad1b906) function
        pub fn votes(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([202, 209, 185, 6], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `weights` (0xa7cac846) function
        pub fn weights(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([167, 202, 200, 70], p0)
                .expect("method not found (this should never happen)")
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
        ///Gets the contract's `Voter__Abstained` event
        pub fn voter_abstained_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            VoterAbstainedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Voter__BribeRewardAdded` event
        pub fn voter_bribe_reward_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            VoterBribeRewardAddedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Voter__Deposit` event
        pub fn voter_deposit_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            VoterDepositFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Voter__DistributeReward` event
        pub fn voter_distribute_reward_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            VoterDistributeRewardFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Voter__GaugeCreated` event
        pub fn voter_gauge_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            VoterGaugeCreatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Voter__GaugeKilled` event
        pub fn voter_gauge_killed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            VoterGaugeKilledFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Voter__GaugeRevived` event
        pub fn voter_gauge_revived_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            VoterGaugeRevivedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Voter__NotifyReward` event
        pub fn voter_notify_reward_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            VoterNotifyRewardFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Voter__Voted` event
        pub fn voter_voted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            VoterVotedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Voter__Withdraw` event
        pub fn voter_withdraw_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            VoterWithdrawFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, VoterEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Voter<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `Voter__AlreadyVotedThisEpoch` with signature `Voter__AlreadyVotedThisEpoch()` and selector `0xb3bd7114`
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
        name = "Voter__AlreadyVotedThisEpoch",
        abi = "Voter__AlreadyVotedThisEpoch()"
    )]
    pub struct Voter__AlreadyVotedThisEpoch;
    ///Custom Error type `Voter__GaugeExists` with signature `Voter__GaugeExists()` and selector `0x25d9db2f`
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
    #[etherror(name = "Voter__GaugeExists", abi = "Voter__GaugeExists()")]
    pub struct Voter__GaugeExists;
    ///Custom Error type `Voter__GaugeIsAlive` with signature `Voter__GaugeIsAlive()` and selector `0xac6553cd`
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
    #[etherror(name = "Voter__GaugeIsAlive", abi = "Voter__GaugeIsAlive()")]
    pub struct Voter__GaugeIsAlive;
    ///Custom Error type `Voter__GaugeIsDead` with signature `Voter__GaugeIsDead()` and selector `0x5cf06fbb`
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
    #[etherror(name = "Voter__GaugeIsDead", abi = "Voter__GaugeIsDead()")]
    pub struct Voter__GaugeIsDead;
    ///Custom Error type `Voter__InvalidZeroAddress` with signature `Voter__InvalidZeroAddress()` and selector `0xc7a2af73`
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
    #[etherror(name = "Voter__InvalidZeroAddress", abi = "Voter__InvalidZeroAddress()")]
    pub struct Voter__InvalidZeroAddress;
    ///Custom Error type `Voter__NotAuthorizedGovernance` with signature `Voter__NotAuthorizedGovernance()` and selector `0x931ddde6`
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
        name = "Voter__NotAuthorizedGovernance",
        abi = "Voter__NotAuthorizedGovernance()"
    )]
    pub struct Voter__NotAuthorizedGovernance;
    ///Custom Error type `Voter__NotAuthorizedMinter` with signature `Voter__NotAuthorizedMinter()` and selector `0x290507e7`
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
        name = "Voter__NotAuthorizedMinter",
        abi = "Voter__NotAuthorizedMinter()"
    )]
    pub struct Voter__NotAuthorizedMinter;
    ///Custom Error type `Voter__NotGauge` with signature `Voter__NotGauge()` and selector `0xaa7a99d1`
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
    #[etherror(name = "Voter__NotGauge", abi = "Voter__NotGauge()")]
    pub struct Voter__NotGauge;
    ///Custom Error type `Voter__NotMinter` with signature `Voter__NotMinter()` and selector `0x62f39aea`
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
    #[etherror(name = "Voter__NotMinter", abi = "Voter__NotMinter()")]
    pub struct Voter__NotMinter;
    ///Custom Error type `Voter__PluginLengthNotEqualToWeightLength` with signature `Voter__PluginLengthNotEqualToWeightLength()` and selector `0x070fd29b`
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
        name = "Voter__PluginLengthNotEqualToWeightLength",
        abi = "Voter__PluginLengthNotEqualToWeightLength()"
    )]
    pub struct Voter__PluginLengthNotEqualToWeightLength;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum VoterErrors {
        Voter__AlreadyVotedThisEpoch(Voter__AlreadyVotedThisEpoch),
        Voter__GaugeExists(Voter__GaugeExists),
        Voter__GaugeIsAlive(Voter__GaugeIsAlive),
        Voter__GaugeIsDead(Voter__GaugeIsDead),
        Voter__InvalidZeroAddress(Voter__InvalidZeroAddress),
        Voter__NotAuthorizedGovernance(Voter__NotAuthorizedGovernance),
        Voter__NotAuthorizedMinter(Voter__NotAuthorizedMinter),
        Voter__NotGauge(Voter__NotGauge),
        Voter__NotMinter(Voter__NotMinter),
        Voter__PluginLengthNotEqualToWeightLength(
            Voter__PluginLengthNotEqualToWeightLength,
        ),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for VoterErrors {
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
                = <Voter__AlreadyVotedThisEpoch as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::Voter__AlreadyVotedThisEpoch(decoded));
            }
            if let Ok(decoded)
                = <Voter__GaugeExists as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Voter__GaugeExists(decoded));
            }
            if let Ok(decoded)
                = <Voter__GaugeIsAlive as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Voter__GaugeIsAlive(decoded));
            }
            if let Ok(decoded)
                = <Voter__GaugeIsDead as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Voter__GaugeIsDead(decoded));
            }
            if let Ok(decoded)
                = <Voter__InvalidZeroAddress as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::Voter__InvalidZeroAddress(decoded));
            }
            if let Ok(decoded)
                = <Voter__NotAuthorizedGovernance as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::Voter__NotAuthorizedGovernance(decoded));
            }
            if let Ok(decoded)
                = <Voter__NotAuthorizedMinter as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::Voter__NotAuthorizedMinter(decoded));
            }
            if let Ok(decoded)
                = <Voter__NotGauge as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Voter__NotGauge(decoded));
            }
            if let Ok(decoded)
                = <Voter__NotMinter as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Voter__NotMinter(decoded));
            }
            if let Ok(decoded)
                = <Voter__PluginLengthNotEqualToWeightLength as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::Voter__PluginLengthNotEqualToWeightLength(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for VoterErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::Voter__AlreadyVotedThisEpoch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Voter__GaugeExists(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Voter__GaugeIsAlive(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Voter__GaugeIsDead(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Voter__InvalidZeroAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Voter__NotAuthorizedGovernance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Voter__NotAuthorizedMinter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Voter__NotGauge(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Voter__NotMinter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Voter__PluginLengthNotEqualToWeightLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for VoterErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <Voter__AlreadyVotedThisEpoch as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Voter__GaugeExists as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Voter__GaugeIsAlive as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Voter__GaugeIsDead as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Voter__InvalidZeroAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Voter__NotAuthorizedGovernance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Voter__NotAuthorizedMinter as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Voter__NotGauge as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Voter__NotMinter as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Voter__PluginLengthNotEqualToWeightLength as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for VoterErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Voter__AlreadyVotedThisEpoch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Voter__GaugeExists(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Voter__GaugeIsAlive(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Voter__GaugeIsDead(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Voter__InvalidZeroAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Voter__NotAuthorizedGovernance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Voter__NotAuthorizedMinter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Voter__NotGauge(element) => ::core::fmt::Display::fmt(element, f),
                Self::Voter__NotMinter(element) => ::core::fmt::Display::fmt(element, f),
                Self::Voter__PluginLengthNotEqualToWeightLength(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for VoterErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<Voter__AlreadyVotedThisEpoch> for VoterErrors {
        fn from(value: Voter__AlreadyVotedThisEpoch) -> Self {
            Self::Voter__AlreadyVotedThisEpoch(value)
        }
    }
    impl ::core::convert::From<Voter__GaugeExists> for VoterErrors {
        fn from(value: Voter__GaugeExists) -> Self {
            Self::Voter__GaugeExists(value)
        }
    }
    impl ::core::convert::From<Voter__GaugeIsAlive> for VoterErrors {
        fn from(value: Voter__GaugeIsAlive) -> Self {
            Self::Voter__GaugeIsAlive(value)
        }
    }
    impl ::core::convert::From<Voter__GaugeIsDead> for VoterErrors {
        fn from(value: Voter__GaugeIsDead) -> Self {
            Self::Voter__GaugeIsDead(value)
        }
    }
    impl ::core::convert::From<Voter__InvalidZeroAddress> for VoterErrors {
        fn from(value: Voter__InvalidZeroAddress) -> Self {
            Self::Voter__InvalidZeroAddress(value)
        }
    }
    impl ::core::convert::From<Voter__NotAuthorizedGovernance> for VoterErrors {
        fn from(value: Voter__NotAuthorizedGovernance) -> Self {
            Self::Voter__NotAuthorizedGovernance(value)
        }
    }
    impl ::core::convert::From<Voter__NotAuthorizedMinter> for VoterErrors {
        fn from(value: Voter__NotAuthorizedMinter) -> Self {
            Self::Voter__NotAuthorizedMinter(value)
        }
    }
    impl ::core::convert::From<Voter__NotGauge> for VoterErrors {
        fn from(value: Voter__NotGauge) -> Self {
            Self::Voter__NotGauge(value)
        }
    }
    impl ::core::convert::From<Voter__NotMinter> for VoterErrors {
        fn from(value: Voter__NotMinter) -> Self {
            Self::Voter__NotMinter(value)
        }
    }
    impl ::core::convert::From<Voter__PluginLengthNotEqualToWeightLength>
    for VoterErrors {
        fn from(value: Voter__PluginLengthNotEqualToWeightLength) -> Self {
            Self::Voter__PluginLengthNotEqualToWeightLength(value)
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
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Voter__Abstained", abi = "Voter__Abstained(address,uint256)")]
    pub struct VoterAbstainedFilter {
        pub account: ::ethers::core::types::Address,
        pub weight: ::ethers::core::types::U256,
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
        name = "Voter__BribeRewardAdded",
        abi = "Voter__BribeRewardAdded(address,address)"
    )]
    pub struct VoterBribeRewardAddedFilter {
        #[ethevent(indexed)]
        pub bribe: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub reward: ::ethers::core::types::Address,
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
        name = "Voter__Deposit",
        abi = "Voter__Deposit(address,address,address,uint256)"
    )]
    pub struct VoterDepositFilter {
        #[ethevent(indexed)]
        pub plugin: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub gauge: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
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
    #[ethevent(
        name = "Voter__DistributeReward",
        abi = "Voter__DistributeReward(address,address,uint256)"
    )]
    pub struct VoterDistributeRewardFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub gauge: ::ethers::core::types::Address,
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
    #[ethevent(
        name = "Voter__GaugeCreated",
        abi = "Voter__GaugeCreated(address,address,address,address)"
    )]
    pub struct VoterGaugeCreatedFilter {
        pub creator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub plugin: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub gauge: ::ethers::core::types::Address,
        pub bribe: ::ethers::core::types::Address,
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
    #[ethevent(name = "Voter__GaugeKilled", abi = "Voter__GaugeKilled(address)")]
    pub struct VoterGaugeKilledFilter {
        #[ethevent(indexed)]
        pub gauge: ::ethers::core::types::Address,
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
    #[ethevent(name = "Voter__GaugeRevived", abi = "Voter__GaugeRevived(address)")]
    pub struct VoterGaugeRevivedFilter {
        #[ethevent(indexed)]
        pub gauge: ::ethers::core::types::Address,
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
        name = "Voter__NotifyReward",
        abi = "Voter__NotifyReward(address,address,uint256)"
    )]
    pub struct VoterNotifyRewardFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub reward: ::ethers::core::types::Address,
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
    #[ethevent(name = "Voter__Voted", abi = "Voter__Voted(address,uint256)")]
    pub struct VoterVotedFilter {
        #[ethevent(indexed)]
        pub voter: ::ethers::core::types::Address,
        pub weight: ::ethers::core::types::U256,
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
        name = "Voter__Withdraw",
        abi = "Voter__Withdraw(address,address,address,uint256)"
    )]
    pub struct VoterWithdrawFilter {
        #[ethevent(indexed)]
        pub plugin: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub gauge: ::ethers::core::types::Address,
        pub account: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum VoterEvents {
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        VoterAbstainedFilter(VoterAbstainedFilter),
        VoterBribeRewardAddedFilter(VoterBribeRewardAddedFilter),
        VoterDepositFilter(VoterDepositFilter),
        VoterDistributeRewardFilter(VoterDistributeRewardFilter),
        VoterGaugeCreatedFilter(VoterGaugeCreatedFilter),
        VoterGaugeKilledFilter(VoterGaugeKilledFilter),
        VoterGaugeRevivedFilter(VoterGaugeRevivedFilter),
        VoterNotifyRewardFilter(VoterNotifyRewardFilter),
        VoterVotedFilter(VoterVotedFilter),
        VoterWithdrawFilter(VoterWithdrawFilter),
    }
    impl ::ethers::contract::EthLogDecode for VoterEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(VoterEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = VoterAbstainedFilter::decode_log(log) {
                return Ok(VoterEvents::VoterAbstainedFilter(decoded));
            }
            if let Ok(decoded) = VoterBribeRewardAddedFilter::decode_log(log) {
                return Ok(VoterEvents::VoterBribeRewardAddedFilter(decoded));
            }
            if let Ok(decoded) = VoterDepositFilter::decode_log(log) {
                return Ok(VoterEvents::VoterDepositFilter(decoded));
            }
            if let Ok(decoded) = VoterDistributeRewardFilter::decode_log(log) {
                return Ok(VoterEvents::VoterDistributeRewardFilter(decoded));
            }
            if let Ok(decoded) = VoterGaugeCreatedFilter::decode_log(log) {
                return Ok(VoterEvents::VoterGaugeCreatedFilter(decoded));
            }
            if let Ok(decoded) = VoterGaugeKilledFilter::decode_log(log) {
                return Ok(VoterEvents::VoterGaugeKilledFilter(decoded));
            }
            if let Ok(decoded) = VoterGaugeRevivedFilter::decode_log(log) {
                return Ok(VoterEvents::VoterGaugeRevivedFilter(decoded));
            }
            if let Ok(decoded) = VoterNotifyRewardFilter::decode_log(log) {
                return Ok(VoterEvents::VoterNotifyRewardFilter(decoded));
            }
            if let Ok(decoded) = VoterVotedFilter::decode_log(log) {
                return Ok(VoterEvents::VoterVotedFilter(decoded));
            }
            if let Ok(decoded) = VoterWithdrawFilter::decode_log(log) {
                return Ok(VoterEvents::VoterWithdrawFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for VoterEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VoterAbstainedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VoterBribeRewardAddedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VoterDepositFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VoterDistributeRewardFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VoterGaugeCreatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VoterGaugeKilledFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VoterGaugeRevivedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VoterNotifyRewardFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VoterVotedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::VoterWithdrawFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for VoterEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<VoterAbstainedFilter> for VoterEvents {
        fn from(value: VoterAbstainedFilter) -> Self {
            Self::VoterAbstainedFilter(value)
        }
    }
    impl ::core::convert::From<VoterBribeRewardAddedFilter> for VoterEvents {
        fn from(value: VoterBribeRewardAddedFilter) -> Self {
            Self::VoterBribeRewardAddedFilter(value)
        }
    }
    impl ::core::convert::From<VoterDepositFilter> for VoterEvents {
        fn from(value: VoterDepositFilter) -> Self {
            Self::VoterDepositFilter(value)
        }
    }
    impl ::core::convert::From<VoterDistributeRewardFilter> for VoterEvents {
        fn from(value: VoterDistributeRewardFilter) -> Self {
            Self::VoterDistributeRewardFilter(value)
        }
    }
    impl ::core::convert::From<VoterGaugeCreatedFilter> for VoterEvents {
        fn from(value: VoterGaugeCreatedFilter) -> Self {
            Self::VoterGaugeCreatedFilter(value)
        }
    }
    impl ::core::convert::From<VoterGaugeKilledFilter> for VoterEvents {
        fn from(value: VoterGaugeKilledFilter) -> Self {
            Self::VoterGaugeKilledFilter(value)
        }
    }
    impl ::core::convert::From<VoterGaugeRevivedFilter> for VoterEvents {
        fn from(value: VoterGaugeRevivedFilter) -> Self {
            Self::VoterGaugeRevivedFilter(value)
        }
    }
    impl ::core::convert::From<VoterNotifyRewardFilter> for VoterEvents {
        fn from(value: VoterNotifyRewardFilter) -> Self {
            Self::VoterNotifyRewardFilter(value)
        }
    }
    impl ::core::convert::From<VoterVotedFilter> for VoterEvents {
        fn from(value: VoterVotedFilter) -> Self {
            Self::VoterVotedFilter(value)
        }
    }
    impl ::core::convert::From<VoterWithdrawFilter> for VoterEvents {
        fn from(value: VoterWithdrawFilter) -> Self {
            Self::VoterWithdrawFilter(value)
        }
    }
    ///Container type for all input parameters for the `OTOKEN` function with signature `OTOKEN()` and selector `0xc544df0c`
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
    #[ethcall(name = "OTOKEN", abi = "OTOKEN()")]
    pub struct OtokenCall;
    ///Container type for all input parameters for the `VTOKEN` function with signature `VTOKEN()` and selector `0xfb548427`
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
    #[ethcall(name = "VTOKEN", abi = "VTOKEN()")]
    pub struct VtokenCall;
    ///Container type for all input parameters for the `addBribeReward` function with signature `addBribeReward(address,address)` and selector `0x7ceed4af`
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
    #[ethcall(name = "addBribeReward", abi = "addBribeReward(address,address)")]
    pub struct AddBribeRewardCall {
        pub bribe: ::ethers::core::types::Address,
        pub reward_token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `addPlugin` function with signature `addPlugin(address)` and selector `0xd8867fc8`
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
    #[ethcall(name = "addPlugin", abi = "addPlugin(address)")]
    pub struct AddPluginCall {
        pub plugin: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `bribefactory` function with signature `bribefactory()` and selector `0x38752a9d`
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
    #[ethcall(name = "bribefactory", abi = "bribefactory()")]
    pub struct BribefactoryCall;
    ///Container type for all input parameters for the `bribes` function with signature `bribes(address)` and selector `0xa8c5d95a`
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
    #[ethcall(name = "bribes", abi = "bribes(address)")]
    pub struct BribesCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `claimBribes` function with signature `claimBribes(address[])` and selector `0x240f9774`
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
    #[ethcall(name = "claimBribes", abi = "claimBribes(address[])")]
    pub struct ClaimBribesCall {
        pub bribes: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `claimRewards` function with signature `claimRewards(address[])` and selector `0xf9f031df`
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
    #[ethcall(name = "claimRewards", abi = "claimRewards(address[])")]
    pub struct ClaimRewardsCall {
        pub gauges: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `claimable` function with signature `claimable(address)` and selector `0x402914f5`
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
    #[ethcall(name = "claimable", abi = "claimable(address)")]
    pub struct ClaimableCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `distribute` function with signature `distribute(address)` and selector `0x63453ae1`
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
    #[ethcall(name = "distribute", abi = "distribute(address)")]
    pub struct DistributeCall {
        pub gauge: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `distribute` function with signature `distribute(uint256,uint256)` and selector `0x7625391a`
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
    #[ethcall(name = "distribute", abi = "distribute(uint256,uint256)")]
    pub struct DistributeWithStartCall {
        pub start: ::ethers::core::types::U256,
        pub finish: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `distributeToBribes` function with signature `distributeToBribes(address[])` and selector `0x4978c512`
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
    #[ethcall(name = "distributeToBribes", abi = "distributeToBribes(address[])")]
    pub struct DistributeToBribesCall {
        pub plugins: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `distro` function with signature `distro()` and selector `0x47b3c6ba`
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
    #[ethcall(name = "distro", abi = "distro()")]
    pub struct DistroCall;
    ///Container type for all input parameters for the `emitDeposit` function with signature `emitDeposit(address,uint256)` and selector `0x28ba84ca`
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
    #[ethcall(name = "emitDeposit", abi = "emitDeposit(address,uint256)")]
    pub struct EmitDepositCall {
        pub account: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `emitWithdraw` function with signature `emitWithdraw(address,uint256)` and selector `0xb014da21`
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
    #[ethcall(name = "emitWithdraw", abi = "emitWithdraw(address,uint256)")]
    pub struct EmitWithdrawCall {
        pub account: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `gaugefactory` function with signature `gaugefactory()` and selector `0x68c3acb3`
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
    #[ethcall(name = "gaugefactory", abi = "gaugefactory()")]
    pub struct GaugefactoryCall;
    ///Container type for all input parameters for the `gauges` function with signature `gauges(address)` and selector `0xb9a09fd5`
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
    #[ethcall(name = "gauges", abi = "gauges(address)")]
    pub struct GaugesCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `getPlugins` function with signature `getPlugins()` and selector `0xa2d869b2`
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
    #[ethcall(name = "getPlugins", abi = "getPlugins()")]
    pub struct GetPluginsCall;
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address)` and selector `0xc4d66de8`
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
    #[ethcall(name = "initialize", abi = "initialize(address)")]
    pub struct InitializeCall {
        pub minter: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isAlive` function with signature `isAlive(address)` and selector `0x1703e5f9`
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
    #[ethcall(name = "isAlive", abi = "isAlive(address)")]
    pub struct IsAliveCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `isGauge` function with signature `isGauge(address)` and selector `0xaa79979b`
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
    #[ethcall(name = "isGauge", abi = "isGauge(address)")]
    pub struct IsGaugeCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `killGauge` function with signature `killGauge(address)` and selector `0x992a7933`
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
    #[ethcall(name = "killGauge", abi = "killGauge(address)")]
    pub struct KillGaugeCall {
        pub gauge: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `lastVoted` function with signature `lastVoted(address)` and selector `0x9a61df89`
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
    #[ethcall(name = "lastVoted", abi = "lastVoted(address)")]
    pub struct LastVotedCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `length` function with signature `length()` and selector `0x1f7b6d32`
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
    #[ethcall(name = "length", abi = "length()")]
    pub struct LengthCall;
    ///Container type for all input parameters for the `minter` function with signature `minter()` and selector `0x07546172`
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
    #[ethcall(name = "minter", abi = "minter()")]
    pub struct MinterCall;
    ///Container type for all input parameters for the `notifyRewardAmount` function with signature `notifyRewardAmount(uint256)` and selector `0x3c6b16ab`
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
    #[ethcall(name = "notifyRewardAmount", abi = "notifyRewardAmount(uint256)")]
    pub struct NotifyRewardAmountCall {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `pluginForGauge` function with signature `pluginForGauge(address)` and selector `0x6c60f246`
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
    #[ethcall(name = "pluginForGauge", abi = "pluginForGauge(address)")]
    pub struct PluginForGaugeCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `pluginVote` function with signature `pluginVote(address,uint256)` and selector `0x773fac7d`
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
    #[ethcall(name = "pluginVote", abi = "pluginVote(address,uint256)")]
    pub struct PluginVoteCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `plugins` function with signature `plugins(uint256)` and selector `0xf0a317eb`
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
    #[ethcall(name = "plugins", abi = "plugins(uint256)")]
    pub struct PluginsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
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
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `reset` function with signature `reset()` and selector `0xd826f88f`
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
    #[ethcall(name = "reset", abi = "reset()")]
    pub struct ResetCall;
    ///Container type for all input parameters for the `reviveGauge` function with signature `reviveGauge(address)` and selector `0x9f06247b`
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
    #[ethcall(name = "reviveGauge", abi = "reviveGauge(address)")]
    pub struct ReviveGaugeCall {
        pub gauge: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `totalWeight` function with signature `totalWeight()` and selector `0x96c82e57`
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
    #[ethcall(name = "totalWeight", abi = "totalWeight()")]
    pub struct TotalWeightCall;
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
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
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `updateAll` function with signature `updateAll()` and selector `0x53d78693`
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
    #[ethcall(name = "updateAll", abi = "updateAll()")]
    pub struct UpdateAllCall;
    ///Container type for all input parameters for the `updateFor` function with signature `updateFor(address[])` and selector `0xd560b0d7`
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
    #[ethcall(name = "updateFor", abi = "updateFor(address[])")]
    pub struct UpdateForCall {
        pub gauges: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `updateForRange` function with signature `updateForRange(uint256,uint256)` and selector `0x9b6a9d72`
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
    #[ethcall(name = "updateForRange", abi = "updateForRange(uint256,uint256)")]
    pub struct UpdateForRangeCall {
        pub start: ::ethers::core::types::U256,
        pub end: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `updateGauge` function with signature `updateGauge(address)` and selector `0x6ecbe38a`
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
    #[ethcall(name = "updateGauge", abi = "updateGauge(address)")]
    pub struct UpdateGaugeCall {
        pub gauge: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `usedWeights` function with signature `usedWeights(address)` and selector `0x002f8de4`
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
    #[ethcall(name = "usedWeights", abi = "usedWeights(address)")]
    pub struct UsedWeightsCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `vote` function with signature `vote(address[],uint256[])` and selector `0x6f816a20`
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
    #[ethcall(name = "vote", abi = "vote(address[],uint256[])")]
    pub struct VoteCall {
        pub plugins: ::std::vec::Vec<::ethers::core::types::Address>,
        pub weights: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all input parameters for the `votes` function with signature `votes(address,address)` and selector `0xcad1b906`
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
    #[ethcall(name = "votes", abi = "votes(address,address)")]
    pub struct VotesCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
    ///Container type for all input parameters for the `weights` function with signature `weights(address)` and selector `0xa7cac846`
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
    #[ethcall(name = "weights", abi = "weights(address)")]
    pub struct WeightsCall(pub ::ethers::core::types::Address);
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum VoterCalls {
        Otoken(OtokenCall),
        Vtoken(VtokenCall),
        AddBribeReward(AddBribeRewardCall),
        AddPlugin(AddPluginCall),
        Bribefactory(BribefactoryCall),
        Bribes(BribesCall),
        ClaimBribes(ClaimBribesCall),
        ClaimRewards(ClaimRewardsCall),
        Claimable(ClaimableCall),
        Distribute(DistributeCall),
        DistributeWithStart(DistributeWithStartCall),
        DistributeToBribes(DistributeToBribesCall),
        Distro(DistroCall),
        EmitDeposit(EmitDepositCall),
        EmitWithdraw(EmitWithdrawCall),
        Gaugefactory(GaugefactoryCall),
        Gauges(GaugesCall),
        GetPlugins(GetPluginsCall),
        Initialize(InitializeCall),
        IsAlive(IsAliveCall),
        IsGauge(IsGaugeCall),
        KillGauge(KillGaugeCall),
        LastVoted(LastVotedCall),
        Length(LengthCall),
        Minter(MinterCall),
        NotifyRewardAmount(NotifyRewardAmountCall),
        Owner(OwnerCall),
        PluginForGauge(PluginForGaugeCall),
        PluginVote(PluginVoteCall),
        Plugins(PluginsCall),
        RenounceOwnership(RenounceOwnershipCall),
        Reset(ResetCall),
        ReviveGauge(ReviveGaugeCall),
        TotalWeight(TotalWeightCall),
        TransferOwnership(TransferOwnershipCall),
        UpdateAll(UpdateAllCall),
        UpdateFor(UpdateForCall),
        UpdateForRange(UpdateForRangeCall),
        UpdateGauge(UpdateGaugeCall),
        UsedWeights(UsedWeightsCall),
        Vote(VoteCall),
        Votes(VotesCall),
        Weights(WeightsCall),
    }
    impl ::ethers::core::abi::AbiDecode for VoterCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <OtokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Otoken(decoded));
            }
            if let Ok(decoded)
                = <VtokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Vtoken(decoded));
            }
            if let Ok(decoded)
                = <AddBribeRewardCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddBribeReward(decoded));
            }
            if let Ok(decoded)
                = <AddPluginCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddPlugin(decoded));
            }
            if let Ok(decoded)
                = <BribefactoryCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Bribefactory(decoded));
            }
            if let Ok(decoded)
                = <BribesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Bribes(decoded));
            }
            if let Ok(decoded)
                = <ClaimBribesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ClaimBribes(decoded));
            }
            if let Ok(decoded)
                = <ClaimRewardsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ClaimRewards(decoded));
            }
            if let Ok(decoded)
                = <ClaimableCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Claimable(decoded));
            }
            if let Ok(decoded)
                = <DistributeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Distribute(decoded));
            }
            if let Ok(decoded)
                = <DistributeWithStartCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DistributeWithStart(decoded));
            }
            if let Ok(decoded)
                = <DistributeToBribesCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DistributeToBribes(decoded));
            }
            if let Ok(decoded)
                = <DistroCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Distro(decoded));
            }
            if let Ok(decoded)
                = <EmitDepositCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::EmitDeposit(decoded));
            }
            if let Ok(decoded)
                = <EmitWithdrawCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::EmitWithdraw(decoded));
            }
            if let Ok(decoded)
                = <GaugefactoryCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Gaugefactory(decoded));
            }
            if let Ok(decoded)
                = <GaugesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Gauges(decoded));
            }
            if let Ok(decoded)
                = <GetPluginsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetPlugins(decoded));
            }
            if let Ok(decoded)
                = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded)
                = <IsAliveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsAlive(decoded));
            }
            if let Ok(decoded)
                = <IsGaugeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsGauge(decoded));
            }
            if let Ok(decoded)
                = <KillGaugeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::KillGauge(decoded));
            }
            if let Ok(decoded)
                = <LastVotedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LastVoted(decoded));
            }
            if let Ok(decoded)
                = <LengthCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Length(decoded));
            }
            if let Ok(decoded)
                = <MinterCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Minter(decoded));
            }
            if let Ok(decoded)
                = <NotifyRewardAmountCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::NotifyRewardAmount(decoded));
            }
            if let Ok(decoded)
                = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded)
                = <PluginForGaugeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PluginForGauge(decoded));
            }
            if let Ok(decoded)
                = <PluginVoteCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PluginVote(decoded));
            }
            if let Ok(decoded)
                = <PluginsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Plugins(decoded));
            }
            if let Ok(decoded)
                = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded)
                = <ResetCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Reset(decoded));
            }
            if let Ok(decoded)
                = <ReviveGaugeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ReviveGauge(decoded));
            }
            if let Ok(decoded)
                = <TotalWeightCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TotalWeight(decoded));
            }
            if let Ok(decoded)
                = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded)
                = <UpdateAllCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UpdateAll(decoded));
            }
            if let Ok(decoded)
                = <UpdateForCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UpdateFor(decoded));
            }
            if let Ok(decoded)
                = <UpdateForRangeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UpdateForRange(decoded));
            }
            if let Ok(decoded)
                = <UpdateGaugeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UpdateGauge(decoded));
            }
            if let Ok(decoded)
                = <UsedWeightsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UsedWeights(decoded));
            }
            if let Ok(decoded)
                = <VoteCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Vote(decoded));
            }
            if let Ok(decoded)
                = <VotesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Votes(decoded));
            }
            if let Ok(decoded)
                = <WeightsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Weights(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for VoterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Otoken(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Vtoken(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AddBribeReward(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddPlugin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Bribefactory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Bribes(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ClaimBribes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ClaimRewards(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Claimable(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Distribute(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DistributeWithStart(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DistributeToBribes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Distro(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EmitDeposit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EmitWithdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Gaugefactory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Gauges(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPlugins(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsAlive(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsGauge(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::KillGauge(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LastVoted(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Length(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Minter(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NotifyRewardAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PluginForGauge(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PluginVote(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Plugins(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Reset(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ReviveGauge(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalWeight(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateAll(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateFor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateForRange(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateGauge(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UsedWeights(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Vote(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Votes(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Weights(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for VoterCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Otoken(element) => ::core::fmt::Display::fmt(element, f),
                Self::Vtoken(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddBribeReward(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddPlugin(element) => ::core::fmt::Display::fmt(element, f),
                Self::Bribefactory(element) => ::core::fmt::Display::fmt(element, f),
                Self::Bribes(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClaimBribes(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClaimRewards(element) => ::core::fmt::Display::fmt(element, f),
                Self::Claimable(element) => ::core::fmt::Display::fmt(element, f),
                Self::Distribute(element) => ::core::fmt::Display::fmt(element, f),
                Self::DistributeWithStart(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DistributeToBribes(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Distro(element) => ::core::fmt::Display::fmt(element, f),
                Self::EmitDeposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::EmitWithdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::Gaugefactory(element) => ::core::fmt::Display::fmt(element, f),
                Self::Gauges(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPlugins(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsAlive(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsGauge(element) => ::core::fmt::Display::fmt(element, f),
                Self::KillGauge(element) => ::core::fmt::Display::fmt(element, f),
                Self::LastVoted(element) => ::core::fmt::Display::fmt(element, f),
                Self::Length(element) => ::core::fmt::Display::fmt(element, f),
                Self::Minter(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotifyRewardAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::PluginForGauge(element) => ::core::fmt::Display::fmt(element, f),
                Self::PluginVote(element) => ::core::fmt::Display::fmt(element, f),
                Self::Plugins(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::Reset(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReviveGauge(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalWeight(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateFor(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateForRange(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateGauge(element) => ::core::fmt::Display::fmt(element, f),
                Self::UsedWeights(element) => ::core::fmt::Display::fmt(element, f),
                Self::Vote(element) => ::core::fmt::Display::fmt(element, f),
                Self::Votes(element) => ::core::fmt::Display::fmt(element, f),
                Self::Weights(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<OtokenCall> for VoterCalls {
        fn from(value: OtokenCall) -> Self {
            Self::Otoken(value)
        }
    }
    impl ::core::convert::From<VtokenCall> for VoterCalls {
        fn from(value: VtokenCall) -> Self {
            Self::Vtoken(value)
        }
    }
    impl ::core::convert::From<AddBribeRewardCall> for VoterCalls {
        fn from(value: AddBribeRewardCall) -> Self {
            Self::AddBribeReward(value)
        }
    }
    impl ::core::convert::From<AddPluginCall> for VoterCalls {
        fn from(value: AddPluginCall) -> Self {
            Self::AddPlugin(value)
        }
    }
    impl ::core::convert::From<BribefactoryCall> for VoterCalls {
        fn from(value: BribefactoryCall) -> Self {
            Self::Bribefactory(value)
        }
    }
    impl ::core::convert::From<BribesCall> for VoterCalls {
        fn from(value: BribesCall) -> Self {
            Self::Bribes(value)
        }
    }
    impl ::core::convert::From<ClaimBribesCall> for VoterCalls {
        fn from(value: ClaimBribesCall) -> Self {
            Self::ClaimBribes(value)
        }
    }
    impl ::core::convert::From<ClaimRewardsCall> for VoterCalls {
        fn from(value: ClaimRewardsCall) -> Self {
            Self::ClaimRewards(value)
        }
    }
    impl ::core::convert::From<ClaimableCall> for VoterCalls {
        fn from(value: ClaimableCall) -> Self {
            Self::Claimable(value)
        }
    }
    impl ::core::convert::From<DistributeCall> for VoterCalls {
        fn from(value: DistributeCall) -> Self {
            Self::Distribute(value)
        }
    }
    impl ::core::convert::From<DistributeWithStartCall> for VoterCalls {
        fn from(value: DistributeWithStartCall) -> Self {
            Self::DistributeWithStart(value)
        }
    }
    impl ::core::convert::From<DistributeToBribesCall> for VoterCalls {
        fn from(value: DistributeToBribesCall) -> Self {
            Self::DistributeToBribes(value)
        }
    }
    impl ::core::convert::From<DistroCall> for VoterCalls {
        fn from(value: DistroCall) -> Self {
            Self::Distro(value)
        }
    }
    impl ::core::convert::From<EmitDepositCall> for VoterCalls {
        fn from(value: EmitDepositCall) -> Self {
            Self::EmitDeposit(value)
        }
    }
    impl ::core::convert::From<EmitWithdrawCall> for VoterCalls {
        fn from(value: EmitWithdrawCall) -> Self {
            Self::EmitWithdraw(value)
        }
    }
    impl ::core::convert::From<GaugefactoryCall> for VoterCalls {
        fn from(value: GaugefactoryCall) -> Self {
            Self::Gaugefactory(value)
        }
    }
    impl ::core::convert::From<GaugesCall> for VoterCalls {
        fn from(value: GaugesCall) -> Self {
            Self::Gauges(value)
        }
    }
    impl ::core::convert::From<GetPluginsCall> for VoterCalls {
        fn from(value: GetPluginsCall) -> Self {
            Self::GetPlugins(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for VoterCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<IsAliveCall> for VoterCalls {
        fn from(value: IsAliveCall) -> Self {
            Self::IsAlive(value)
        }
    }
    impl ::core::convert::From<IsGaugeCall> for VoterCalls {
        fn from(value: IsGaugeCall) -> Self {
            Self::IsGauge(value)
        }
    }
    impl ::core::convert::From<KillGaugeCall> for VoterCalls {
        fn from(value: KillGaugeCall) -> Self {
            Self::KillGauge(value)
        }
    }
    impl ::core::convert::From<LastVotedCall> for VoterCalls {
        fn from(value: LastVotedCall) -> Self {
            Self::LastVoted(value)
        }
    }
    impl ::core::convert::From<LengthCall> for VoterCalls {
        fn from(value: LengthCall) -> Self {
            Self::Length(value)
        }
    }
    impl ::core::convert::From<MinterCall> for VoterCalls {
        fn from(value: MinterCall) -> Self {
            Self::Minter(value)
        }
    }
    impl ::core::convert::From<NotifyRewardAmountCall> for VoterCalls {
        fn from(value: NotifyRewardAmountCall) -> Self {
            Self::NotifyRewardAmount(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for VoterCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PluginForGaugeCall> for VoterCalls {
        fn from(value: PluginForGaugeCall) -> Self {
            Self::PluginForGauge(value)
        }
    }
    impl ::core::convert::From<PluginVoteCall> for VoterCalls {
        fn from(value: PluginVoteCall) -> Self {
            Self::PluginVote(value)
        }
    }
    impl ::core::convert::From<PluginsCall> for VoterCalls {
        fn from(value: PluginsCall) -> Self {
            Self::Plugins(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for VoterCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<ResetCall> for VoterCalls {
        fn from(value: ResetCall) -> Self {
            Self::Reset(value)
        }
    }
    impl ::core::convert::From<ReviveGaugeCall> for VoterCalls {
        fn from(value: ReviveGaugeCall) -> Self {
            Self::ReviveGauge(value)
        }
    }
    impl ::core::convert::From<TotalWeightCall> for VoterCalls {
        fn from(value: TotalWeightCall) -> Self {
            Self::TotalWeight(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for VoterCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UpdateAllCall> for VoterCalls {
        fn from(value: UpdateAllCall) -> Self {
            Self::UpdateAll(value)
        }
    }
    impl ::core::convert::From<UpdateForCall> for VoterCalls {
        fn from(value: UpdateForCall) -> Self {
            Self::UpdateFor(value)
        }
    }
    impl ::core::convert::From<UpdateForRangeCall> for VoterCalls {
        fn from(value: UpdateForRangeCall) -> Self {
            Self::UpdateForRange(value)
        }
    }
    impl ::core::convert::From<UpdateGaugeCall> for VoterCalls {
        fn from(value: UpdateGaugeCall) -> Self {
            Self::UpdateGauge(value)
        }
    }
    impl ::core::convert::From<UsedWeightsCall> for VoterCalls {
        fn from(value: UsedWeightsCall) -> Self {
            Self::UsedWeights(value)
        }
    }
    impl ::core::convert::From<VoteCall> for VoterCalls {
        fn from(value: VoteCall) -> Self {
            Self::Vote(value)
        }
    }
    impl ::core::convert::From<VotesCall> for VoterCalls {
        fn from(value: VotesCall) -> Self {
            Self::Votes(value)
        }
    }
    impl ::core::convert::From<WeightsCall> for VoterCalls {
        fn from(value: WeightsCall) -> Self {
            Self::Weights(value)
        }
    }
    ///Container type for all return fields from the `OTOKEN` function with signature `OTOKEN()` and selector `0xc544df0c`
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
    pub struct OtokenReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `VTOKEN` function with signature `VTOKEN()` and selector `0xfb548427`
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
    pub struct VtokenReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `addPlugin` function with signature `addPlugin(address)` and selector `0xd8867fc8`
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
    pub struct AddPluginReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `bribefactory` function with signature `bribefactory()` and selector `0x38752a9d`
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
    pub struct BribefactoryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `bribes` function with signature `bribes(address)` and selector `0xa8c5d95a`
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
    pub struct BribesReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `claimable` function with signature `claimable(address)` and selector `0x402914f5`
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
    pub struct ClaimableReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `gaugefactory` function with signature `gaugefactory()` and selector `0x68c3acb3`
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
    pub struct GaugefactoryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `gauges` function with signature `gauges(address)` and selector `0xb9a09fd5`
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
    pub struct GaugesReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getPlugins` function with signature `getPlugins()` and selector `0xa2d869b2`
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
    pub struct GetPluginsReturn(pub ::std::vec::Vec<::ethers::core::types::Address>);
    ///Container type for all return fields from the `isAlive` function with signature `isAlive(address)` and selector `0x1703e5f9`
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
    pub struct IsAliveReturn(pub bool);
    ///Container type for all return fields from the `isGauge` function with signature `isGauge(address)` and selector `0xaa79979b`
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
    pub struct IsGaugeReturn(pub bool);
    ///Container type for all return fields from the `lastVoted` function with signature `lastVoted(address)` and selector `0x9a61df89`
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
    pub struct LastVotedReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `length` function with signature `length()` and selector `0x1f7b6d32`
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
    pub struct LengthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `minter` function with signature `minter()` and selector `0x07546172`
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
    pub struct MinterReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `pluginForGauge` function with signature `pluginForGauge(address)` and selector `0x6c60f246`
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
    pub struct PluginForGaugeReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `pluginVote` function with signature `pluginVote(address,uint256)` and selector `0x773fac7d`
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
    pub struct PluginVoteReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `plugins` function with signature `plugins(uint256)` and selector `0xf0a317eb`
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
    pub struct PluginsReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `totalWeight` function with signature `totalWeight()` and selector `0x96c82e57`
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
    pub struct TotalWeightReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `usedWeights` function with signature `usedWeights(address)` and selector `0x002f8de4`
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
    pub struct UsedWeightsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `votes` function with signature `votes(address,address)` and selector `0xcad1b906`
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
    pub struct VotesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `weights` function with signature `weights(address)` and selector `0xa7cac846`
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
    pub struct WeightsReturn(pub ::ethers::core::types::U256);
}
