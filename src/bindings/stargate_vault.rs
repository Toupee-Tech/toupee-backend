pub use stargate_vault::*;
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
pub mod stargate_vault {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_poolId"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_router"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_token"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_sharedDecimals"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_localDecimals"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_feeLibrary"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_name"),
                        kind: ::ethers::core::abi::ethabi::ParamType::String,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("string"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_symbol"),
                        kind: ::ethers::core::abi::ethabi::ParamType::String,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("string"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("BP_DENOMINATOR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("BP_DENOMINATOR"),
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
                    ::std::borrow::ToOwned::to_owned("DOMAIN_SEPARATOR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DOMAIN_SEPARATOR"),
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
                    ::std::borrow::ToOwned::to_owned("PERMIT_TYPEHASH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("PERMIT_TYPEHASH"),
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
                    ::std::borrow::ToOwned::to_owned("activateChainPath"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("activateChainPath"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_dstChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_dstPoolId"),
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
                    ::std::borrow::ToOwned::to_owned("allowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allowance"),
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
                    ::std::borrow::ToOwned::to_owned("amountLPtoLD"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("amountLPtoLD"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amountLP"),
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
                    ::std::borrow::ToOwned::to_owned("approve"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("approve"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("batched"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("batched"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("callDelta"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("callDelta"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_fullMode"),
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
                    ::std::borrow::ToOwned::to_owned("chainPathIndexLookup"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "chainPathIndexLookup",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
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
                    ::std::borrow::ToOwned::to_owned("chainPaths"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("chainPaths"),
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
                                    name: ::std::borrow::ToOwned::to_owned("ready"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dstChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("dstPoolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("weight"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("balance"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("lkb"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("credits"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("idealBalance"),
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
                    ::std::borrow::ToOwned::to_owned("convertRate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("convertRate"),
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
                    ::std::borrow::ToOwned::to_owned("createChainPath"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("createChainPath"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_dstChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_dstPoolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_weight"),
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
                    ::std::borrow::ToOwned::to_owned("creditChainPath"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("creditChainPath"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_dstChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_dstPoolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_c"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Pool.CreditObj"),
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
                    ::std::borrow::ToOwned::to_owned("decimals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("decimals"),
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
                    ::std::borrow::ToOwned::to_owned("decreaseAllowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("decreaseAllowance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("subtractedValue"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("defaultLPMode"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("defaultLPMode"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("defaultSwapMode"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("defaultSwapMode"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("deltaCredit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deltaCredit"),
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
                    ::std::borrow::ToOwned::to_owned("eqFeePool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("eqFeePool"),
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
                    ::std::borrow::ToOwned::to_owned("feeLibrary"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("feeLibrary"),
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
                    ::std::borrow::ToOwned::to_owned("getChainPath"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getChainPath"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_dstChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_dstPoolId"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Pool.ChainPath"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getChainPathsLength"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getChainPathsLength",
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
                    ::std::borrow::ToOwned::to_owned("increaseAllowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("increaseAllowance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("addedValue"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("instantRedeemLocal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("instantRedeemLocal"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amountLP"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountSD"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("localDecimals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("localDecimals"),
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
                    ::std::borrow::ToOwned::to_owned("lpDeltaBP"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("lpDeltaBP"),
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
                    ::std::borrow::ToOwned::to_owned("mint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amountLD"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("mintFeeBP"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mintFeeBP"),
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
                    ::std::borrow::ToOwned::to_owned("mintFeeBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mintFeeBalance"),
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
                    ::std::borrow::ToOwned::to_owned("name"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("name"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("nonces"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("nonces"),
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
                    ::std::borrow::ToOwned::to_owned("permit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("permit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("v"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("r"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("s"),
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
                    ::std::borrow::ToOwned::to_owned("poolId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("poolId"),
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
                    ::std::borrow::ToOwned::to_owned("protocolFeeBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("protocolFeeBalance"),
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
                    ::std::borrow::ToOwned::to_owned("redeemLocal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("redeemLocal"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amountLP"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_dstChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_dstPoolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountSD"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("redeemLocalCallback"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "redeemLocalCallback",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_srcChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_srcPoolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amountSD"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amountToMintSD"),
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
                    ::std::borrow::ToOwned::to_owned("redeemLocalCheckOnRemote"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "redeemLocalCheckOnRemote",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_srcChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_srcPoolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amountSD"),
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
                                    name: ::std::borrow::ToOwned::to_owned("swapAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("mintAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("redeemRemote"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("redeemRemote"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_dstChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_dstPoolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amountLP"),
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
                    ::std::borrow::ToOwned::to_owned("router"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("router"),
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
                    ::std::borrow::ToOwned::to_owned("sendCredits"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sendCredits"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_dstChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_dstPoolId"),
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
                                    name: ::std::borrow::ToOwned::to_owned("c"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Pool.CreditObj"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setDeltaParam"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setDeltaParam"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_batched"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_swapDeltaBP"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_lpDeltaBP"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_defaultSwapMode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_defaultLPMode"),
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
                    ::std::borrow::ToOwned::to_owned("setFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setFee"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_mintFeeBP"),
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
                    ::std::borrow::ToOwned::to_owned("setFeeLibrary"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setFeeLibrary"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_feeLibraryAddr"),
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
                    ::std::borrow::ToOwned::to_owned("setSwapStop"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setSwapStop"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_swapStop"),
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
                    ::std::borrow::ToOwned::to_owned("setWeightForChainPath"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setWeightForChainPath",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_dstChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_dstPoolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_weight"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
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
                    ::std::borrow::ToOwned::to_owned("sharedDecimals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sharedDecimals"),
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
                    ::std::borrow::ToOwned::to_owned("stopSwap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("stopSwap"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("swap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("swap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_dstChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_dstPoolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amountLD"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_minAmountLD"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newLiquidity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Pool.SwapObj"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("swapDeltaBP"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("swapDeltaBP"),
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
                    ::std::borrow::ToOwned::to_owned("swapRemote"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("swapRemote"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_srcChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_srcPoolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_s"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Pool.SwapObj"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountLD"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("symbol"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("symbol"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("token"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("token"),
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
                    ::std::borrow::ToOwned::to_owned("totalLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("totalLiquidity"),
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
                    ::std::borrow::ToOwned::to_owned("transfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transfer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdrawMintFeeBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "withdrawMintFeeBalance",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_to"),
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
                    ::std::borrow::ToOwned::to_owned("withdrawProtocolFeeBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "withdrawProtocolFeeBalance",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_to"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Approval"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Approval"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
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
                    ::std::borrow::ToOwned::to_owned("Burn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Burn"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amountLP"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amountSD"),
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
                    ::std::borrow::ToOwned::to_owned("ChainPathUpdate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ChainPathUpdate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("dstChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("dstPoolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("CreditChainPath"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("CreditChainPath"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("chainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("srcPoolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amountSD"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("idealBalance"),
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
                    ::std::borrow::ToOwned::to_owned("DeltaParamUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("DeltaParamUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("batched"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("swapDeltaBP"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("lpDeltaBP"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("defaultSwapMode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("defaultLPMode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FeeLibraryUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("FeeLibraryUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("feeLibraryAddr"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FeesUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("FeesUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("mintFeeBP"),
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
                    ::std::borrow::ToOwned::to_owned("InstantRedeemLocal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("InstantRedeemLocal"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amountLP"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amountSD"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Mint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Mint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amountLP"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amountSD"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("mintFeeAmountSD"),
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
                    ::std::borrow::ToOwned::to_owned("RedeemLocal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RedeemLocal"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amountLP"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amountSD"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("chainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("dstPoolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RedeemLocalCallback"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RedeemLocalCallback",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_amountSD"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_amountToMintSD"),
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
                    ::std::borrow::ToOwned::to_owned("RedeemRemote"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RedeemRemote"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("chainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("dstPoolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amountLP"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amountSD"),
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
                    ::std::borrow::ToOwned::to_owned("SendCredits"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SendCredits"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("dstChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("dstPoolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("credits"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("idealBalance"),
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
                    ::std::borrow::ToOwned::to_owned("StopSwapUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("StopSwapUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("swapStop"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Swap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Swap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("chainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("dstPoolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amountSD"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("eqReward"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("eqFee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("protocolFee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("lpFee"),
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
                    ::std::borrow::ToOwned::to_owned("SwapRemote"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SwapRemote"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amountSD"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("protocolFee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("dstFee"),
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
                    ::std::borrow::ToOwned::to_owned("Transfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Transfer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
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
                    ::std::borrow::ToOwned::to_owned("WithdrawMintFeeBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "WithdrawMintFeeBalance",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amountSD"),
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
                    ::std::borrow::ToOwned::to_owned("WithdrawProtocolFeeBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "WithdrawProtocolFeeBalance",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amountSD"),
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
                    ::std::borrow::ToOwned::to_owned("WithdrawRemote"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("WithdrawRemote"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("srcChainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("srcPoolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("swapAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("mintAmount"),
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
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static STARGATEVAULT_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct StargateVault<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for StargateVault<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for StargateVault<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for StargateVault<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for StargateVault<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(StargateVault))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> StargateVault<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    STARGATEVAULT_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `BP_DENOMINATOR` (0xabe685cd) function
        pub fn bp_denominator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([171, 230, 133, 205], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `DOMAIN_SEPARATOR` (0x3644e515) function
        pub fn domain_separator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 68, 229, 21], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PERMIT_TYPEHASH` (0x30adf81f) function
        pub fn permit_typehash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([48, 173, 248, 31], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `activateChainPath` (0x8bd86d0a) function
        pub fn activate_chain_path(
            &self,
            dst_chain_id: u16,
            dst_pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([139, 216, 109, 10], (dst_chain_id, dst_pool_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allowance` (0xdd62ed3e) function
        pub fn allowance(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `amountLPtoLD` (0xf6cd35ee) function
        pub fn amount_l_pto_ld(
            &self,
            amount_lp: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([246, 205, 53, 238], amount_lp)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approve` (0x095ea7b3) function
        pub fn approve(
            &self,
            spender: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `batched` (0x27f92376) function
        pub fn batched(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([39, 249, 35, 118], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `callDelta` (0x7fb65265) function
        pub fn call_delta(
            &self,
            full_mode: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([127, 182, 82, 101], full_mode)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `chainPathIndexLookup` (0x64c5f02d) function
        pub fn chain_path_index_lookup(
            &self,
            p0: u16,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([100, 197, 240, 45], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `chainPaths` (0xa138ed6b) function
        pub fn chain_paths(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                bool,
                u16,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([161, 56, 237, 107], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `convertRate` (0xfeb56b15) function
        pub fn convert_rate(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([254, 181, 107, 21], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createChainPath` (0x20d6bc75) function
        pub fn create_chain_path(
            &self,
            dst_chain_id: u16,
            dst_pool_id: ::ethers::core::types::U256,
            weight: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([32, 214, 188, 117], (dst_chain_id, dst_pool_id, weight))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `creditChainPath` (0xb6addec7) function
        pub fn credit_chain_path(
            &self,
            dst_chain_id: u16,
            dst_pool_id: ::ethers::core::types::U256,
            c: CreditObj,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([182, 173, 222, 199], (dst_chain_id, dst_pool_id, c))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decimals` (0x313ce567) function
        pub fn decimals(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decreaseAllowance` (0xa457c2d7) function
        pub fn decrease_allowance(
            &self,
            spender: ::ethers::core::types::Address,
            subtracted_value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([164, 87, 194, 215], (spender, subtracted_value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `defaultLPMode` (0x28f079c2) function
        pub fn default_lp_mode(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([40, 240, 121, 194], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `defaultSwapMode` (0x99a22d68) function
        pub fn default_swap_mode(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([153, 162, 45, 104], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deltaCredit` (0x1e8e51da) function
        pub fn delta_credit(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([30, 142, 81, 218], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `eqFeePool` (0x9bb81119) function
        pub fn eq_fee_pool(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([155, 184, 17, 25], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `feeLibrary` (0x001edfab) function
        pub fn fee_library(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([0, 30, 223, 171], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getChainPath` (0x159f6add) function
        pub fn get_chain_path(
            &self,
            dst_chain_id: u16,
            dst_pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ChainPath> {
            self.0
                .method_hash([21, 159, 106, 221], (dst_chain_id, dst_pool_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getChainPathsLength` (0x163ef490) function
        pub fn get_chain_paths_length(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([22, 62, 244, 144], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `increaseAllowance` (0x39509351) function
        pub fn increase_allowance(
            &self,
            spender: ::ethers::core::types::Address,
            added_value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([57, 80, 147, 81], (spender, added_value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `instantRedeemLocal` (0x0986b61a) function
        pub fn instant_redeem_local(
            &self,
            from: ::ethers::core::types::Address,
            amount_lp: ::ethers::core::types::U256,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([9, 134, 182, 26], (from, amount_lp, to))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `localDecimals` (0xe46e7058) function
        pub fn local_decimals(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([228, 110, 112, 88], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lpDeltaBP` (0x36448777) function
        pub fn lp_delta_bp(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([54, 68, 135, 119], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mint` (0x40c10f19) function
        pub fn mint(
            &self,
            to: ::ethers::core::types::Address,
            amount_ld: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([64, 193, 15, 25], (to, amount_ld))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mintFeeBP` (0xfaa24f07) function
        pub fn mint_fee_bp(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([250, 162, 79, 7], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mintFeeBalance` (0x65152f2b) function
        pub fn mint_fee_balance(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([101, 21, 47, 43], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nonces` (0x7ecebe00) function
        pub fn nonces(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([126, 206, 190, 0], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `permit` (0xd505accf) function
        pub fn permit(
            &self,
            owner: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            deadline: ::ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [213, 5, 172, 207],
                    (owner, spender, value, deadline, v, r, s),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `poolId` (0x3e0dc34e) function
        pub fn pool_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([62, 13, 195, 78], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `protocolFeeBalance` (0x0a22d68c) function
        pub fn protocol_fee_balance(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([10, 34, 214, 140], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `redeemLocal` (0xb0fab0bc) function
        pub fn redeem_local(
            &self,
            from: ::ethers::core::types::Address,
            amount_lp: ::ethers::core::types::U256,
            dst_chain_id: u16,
            dst_pool_id: ::ethers::core::types::U256,
            to: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [176, 250, 176, 188],
                    (from, amount_lp, dst_chain_id, dst_pool_id, to),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `redeemLocalCallback` (0xb30daeac) function
        pub fn redeem_local_callback(
            &self,
            src_chain_id: u16,
            src_pool_id: ::ethers::core::types::U256,
            to: ::ethers::core::types::Address,
            amount_sd: ::ethers::core::types::U256,
            amount_to_mint_sd: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [179, 13, 174, 172],
                    (src_chain_id, src_pool_id, to, amount_sd, amount_to_mint_sd),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `redeemLocalCheckOnRemote` (0xea89e2aa) function
        pub fn redeem_local_check_on_remote(
            &self,
            src_chain_id: u16,
            src_pool_id: ::ethers::core::types::U256,
            amount_sd: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [234, 137, 226, 170],
                    (src_chain_id, src_pool_id, amount_sd),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `redeemRemote` (0x7298a5dc) function
        pub fn redeem_remote(
            &self,
            dst_chain_id: u16,
            dst_pool_id: ::ethers::core::types::U256,
            from: ::ethers::core::types::Address,
            amount_lp: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [114, 152, 165, 220],
                    (dst_chain_id, dst_pool_id, from, amount_lp),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `router` (0xf887ea40) function
        pub fn router(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([248, 135, 234, 64], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sendCredits` (0x08e9d8c2) function
        pub fn send_credits(
            &self,
            dst_chain_id: u16,
            dst_pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, CreditObj> {
            self.0
                .method_hash([8, 233, 216, 194], (dst_chain_id, dst_pool_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setDeltaParam` (0xe065608b) function
        pub fn set_delta_param(
            &self,
            batched: bool,
            swap_delta_bp: ::ethers::core::types::U256,
            lp_delta_bp: ::ethers::core::types::U256,
            default_swap_mode: bool,
            default_lp_mode: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [224, 101, 96, 139],
                    (
                        batched,
                        swap_delta_bp,
                        lp_delta_bp,
                        default_swap_mode,
                        default_lp_mode,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFee` (0x69fe0e2d) function
        pub fn set_fee(
            &self,
            mint_fee_bp: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([105, 254, 14, 45], mint_fee_bp)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFeeLibrary` (0x4b5cacbc) function
        pub fn set_fee_library(
            &self,
            fee_library_addr: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([75, 92, 172, 188], fee_library_addr)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setSwapStop` (0xac2cc36b) function
        pub fn set_swap_stop(
            &self,
            swap_stop: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([172, 44, 195, 107], swap_stop)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setWeightForChainPath` (0xa985565f) function
        pub fn set_weight_for_chain_path(
            &self,
            dst_chain_id: u16,
            dst_pool_id: ::ethers::core::types::U256,
            weight: u16,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([169, 133, 86, 95], (dst_chain_id, dst_pool_id, weight))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sharedDecimals` (0x857749b0) function
        pub fn shared_decimals(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([133, 119, 73, 176], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stopSwap` (0xb633b364) function
        pub fn stop_swap(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([182, 51, 179, 100], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swap` (0x1b7319b6) function
        pub fn swap(
            &self,
            dst_chain_id: u16,
            dst_pool_id: ::ethers::core::types::U256,
            from: ::ethers::core::types::Address,
            amount_ld: ::ethers::core::types::U256,
            min_amount_ld: ::ethers::core::types::U256,
            new_liquidity: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, SwapObj> {
            self.0
                .method_hash(
                    [27, 115, 25, 182],
                    (
                        dst_chain_id,
                        dst_pool_id,
                        from,
                        amount_ld,
                        min_amount_ld,
                        new_liquidity,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapDeltaBP` (0xcdfed0ab) function
        pub fn swap_delta_bp(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([205, 254, 208, 171], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapRemote` (0x902b8ab7) function
        pub fn swap_remote(
            &self,
            src_chain_id: u16,
            src_pool_id: ::ethers::core::types::U256,
            to: ::ethers::core::types::Address,
            s: SwapObj,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([144, 43, 138, 183], (src_chain_id, src_pool_id, to, s))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `symbol` (0x95d89b41) function
        pub fn symbol(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `token` (0xfc0c546a) function
        pub fn token(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([252, 12, 84, 106], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalLiquidity` (0x15770f92) function
        pub fn total_liquidity(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([21, 119, 15, 146], ())
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
        ///Calls the contract's `totalWeight` (0x96c82e57) function
        pub fn total_weight(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([150, 200, 46, 87], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transfer` (0xa9059cbb) function
        pub fn transfer(
            &self,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (to, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawMintFeeBalance` (0x476efe40) function
        pub fn withdraw_mint_fee_balance(
            &self,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([71, 110, 254, 64], to)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawProtocolFeeBalance` (0xbe310294) function
        pub fn withdraw_protocol_fee_balance(
            &self,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([190, 49, 2, 148], to)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Approval` event
        pub fn approval_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApprovalFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Burn` event
        pub fn burn_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, BurnFilter> {
            self.0.event()
        }
        ///Gets the contract's `ChainPathUpdate` event
        pub fn chain_path_update_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ChainPathUpdateFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `CreditChainPath` event
        pub fn credit_chain_path_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CreditChainPathFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DeltaParamUpdated` event
        pub fn delta_param_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DeltaParamUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `FeeLibraryUpdated` event
        pub fn fee_library_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            FeeLibraryUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `FeesUpdated` event
        pub fn fees_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            FeesUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `InstantRedeemLocal` event
        pub fn instant_redeem_local_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            InstantRedeemLocalFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Mint` event
        pub fn mint_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, MintFilter> {
            self.0.event()
        }
        ///Gets the contract's `RedeemLocal` event
        pub fn redeem_local_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RedeemLocalFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RedeemLocalCallback` event
        pub fn redeem_local_callback_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RedeemLocalCallbackFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RedeemRemote` event
        pub fn redeem_remote_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RedeemRemoteFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SendCredits` event
        pub fn send_credits_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SendCreditsFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `StopSwapUpdated` event
        pub fn stop_swap_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StopSwapUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Swap` event
        pub fn swap_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SwapFilter> {
            self.0.event()
        }
        ///Gets the contract's `SwapRemote` event
        pub fn swap_remote_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SwapRemoteFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Transfer` event
        pub fn transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TransferFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `WithdrawMintFeeBalance` event
        pub fn withdraw_mint_fee_balance_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            WithdrawMintFeeBalanceFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `WithdrawProtocolFeeBalance` event
        pub fn withdraw_protocol_fee_balance_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            WithdrawProtocolFeeBalanceFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `WithdrawRemote` event
        pub fn withdraw_remote_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            WithdrawRemoteFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StargateVaultEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for StargateVault<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
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
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
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
    #[ethevent(name = "Burn", abi = "Burn(address,uint256,uint256)")]
    pub struct BurnFilter {
        pub from: ::ethers::core::types::Address,
        pub amount_lp: ::ethers::core::types::U256,
        pub amount_sd: ::ethers::core::types::U256,
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
        name = "ChainPathUpdate",
        abi = "ChainPathUpdate(uint16,uint256,uint256)"
    )]
    pub struct ChainPathUpdateFilter {
        pub dst_chain_id: u16,
        pub dst_pool_id: ::ethers::core::types::U256,
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
        name = "CreditChainPath",
        abi = "CreditChainPath(uint16,uint256,uint256,uint256)"
    )]
    pub struct CreditChainPathFilter {
        pub chain_id: u16,
        pub src_pool_id: ::ethers::core::types::U256,
        pub amount_sd: ::ethers::core::types::U256,
        pub ideal_balance: ::ethers::core::types::U256,
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
        name = "DeltaParamUpdated",
        abi = "DeltaParamUpdated(bool,uint256,uint256,bool,bool)"
    )]
    pub struct DeltaParamUpdatedFilter {
        pub batched: bool,
        pub swap_delta_bp: ::ethers::core::types::U256,
        pub lp_delta_bp: ::ethers::core::types::U256,
        pub default_swap_mode: bool,
        pub default_lp_mode: bool,
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
    #[ethevent(name = "FeeLibraryUpdated", abi = "FeeLibraryUpdated(address)")]
    pub struct FeeLibraryUpdatedFilter {
        pub fee_library_addr: ::ethers::core::types::Address,
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
    #[ethevent(name = "FeesUpdated", abi = "FeesUpdated(uint256)")]
    pub struct FeesUpdatedFilter {
        pub mint_fee_bp: ::ethers::core::types::U256,
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
        name = "InstantRedeemLocal",
        abi = "InstantRedeemLocal(address,uint256,uint256,address)"
    )]
    pub struct InstantRedeemLocalFilter {
        pub from: ::ethers::core::types::Address,
        pub amount_lp: ::ethers::core::types::U256,
        pub amount_sd: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Address,
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
    #[ethevent(name = "Mint", abi = "Mint(address,uint256,uint256,uint256)")]
    pub struct MintFilter {
        pub to: ::ethers::core::types::Address,
        pub amount_lp: ::ethers::core::types::U256,
        pub amount_sd: ::ethers::core::types::U256,
        pub mint_fee_amount_sd: ::ethers::core::types::U256,
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
        name = "RedeemLocal",
        abi = "RedeemLocal(address,uint256,uint256,uint16,uint256,bytes)"
    )]
    pub struct RedeemLocalFilter {
        pub from: ::ethers::core::types::Address,
        pub amount_lp: ::ethers::core::types::U256,
        pub amount_sd: ::ethers::core::types::U256,
        pub chain_id: u16,
        pub dst_pool_id: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Bytes,
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
        name = "RedeemLocalCallback",
        abi = "RedeemLocalCallback(address,uint256,uint256)"
    )]
    pub struct RedeemLocalCallbackFilter {
        pub to: ::ethers::core::types::Address,
        pub amount_sd: ::ethers::core::types::U256,
        pub amount_to_mint_sd: ::ethers::core::types::U256,
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
        name = "RedeemRemote",
        abi = "RedeemRemote(uint16,uint256,address,uint256,uint256)"
    )]
    pub struct RedeemRemoteFilter {
        pub chain_id: u16,
        pub dst_pool_id: ::ethers::core::types::U256,
        pub from: ::ethers::core::types::Address,
        pub amount_lp: ::ethers::core::types::U256,
        pub amount_sd: ::ethers::core::types::U256,
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
        name = "SendCredits",
        abi = "SendCredits(uint16,uint256,uint256,uint256)"
    )]
    pub struct SendCreditsFilter {
        pub dst_chain_id: u16,
        pub dst_pool_id: ::ethers::core::types::U256,
        pub credits: ::ethers::core::types::U256,
        pub ideal_balance: ::ethers::core::types::U256,
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
    #[ethevent(name = "StopSwapUpdated", abi = "StopSwapUpdated(bool)")]
    pub struct StopSwapUpdatedFilter {
        pub swap_stop: bool,
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
        name = "Swap",
        abi = "Swap(uint16,uint256,address,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct SwapFilter {
        pub chain_id: u16,
        pub dst_pool_id: ::ethers::core::types::U256,
        pub from: ::ethers::core::types::Address,
        pub amount_sd: ::ethers::core::types::U256,
        pub eq_reward: ::ethers::core::types::U256,
        pub eq_fee: ::ethers::core::types::U256,
        pub protocol_fee: ::ethers::core::types::U256,
        pub lp_fee: ::ethers::core::types::U256,
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
    #[ethevent(name = "SwapRemote", abi = "SwapRemote(address,uint256,uint256,uint256)")]
    pub struct SwapRemoteFilter {
        pub to: ::ethers::core::types::Address,
        pub amount_sd: ::ethers::core::types::U256,
        pub protocol_fee: ::ethers::core::types::U256,
        pub dst_fee: ::ethers::core::types::U256,
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
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
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
        name = "WithdrawMintFeeBalance",
        abi = "WithdrawMintFeeBalance(address,uint256)"
    )]
    pub struct WithdrawMintFeeBalanceFilter {
        pub to: ::ethers::core::types::Address,
        pub amount_sd: ::ethers::core::types::U256,
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
        name = "WithdrawProtocolFeeBalance",
        abi = "WithdrawProtocolFeeBalance(address,uint256)"
    )]
    pub struct WithdrawProtocolFeeBalanceFilter {
        pub to: ::ethers::core::types::Address,
        pub amount_sd: ::ethers::core::types::U256,
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
        name = "WithdrawRemote",
        abi = "WithdrawRemote(uint16,uint256,uint256,uint256)"
    )]
    pub struct WithdrawRemoteFilter {
        pub src_chain_id: u16,
        pub src_pool_id: ::ethers::core::types::U256,
        pub swap_amount: ::ethers::core::types::U256,
        pub mint_amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum StargateVaultEvents {
        ApprovalFilter(ApprovalFilter),
        BurnFilter(BurnFilter),
        ChainPathUpdateFilter(ChainPathUpdateFilter),
        CreditChainPathFilter(CreditChainPathFilter),
        DeltaParamUpdatedFilter(DeltaParamUpdatedFilter),
        FeeLibraryUpdatedFilter(FeeLibraryUpdatedFilter),
        FeesUpdatedFilter(FeesUpdatedFilter),
        InstantRedeemLocalFilter(InstantRedeemLocalFilter),
        MintFilter(MintFilter),
        RedeemLocalFilter(RedeemLocalFilter),
        RedeemLocalCallbackFilter(RedeemLocalCallbackFilter),
        RedeemRemoteFilter(RedeemRemoteFilter),
        SendCreditsFilter(SendCreditsFilter),
        StopSwapUpdatedFilter(StopSwapUpdatedFilter),
        SwapFilter(SwapFilter),
        SwapRemoteFilter(SwapRemoteFilter),
        TransferFilter(TransferFilter),
        WithdrawMintFeeBalanceFilter(WithdrawMintFeeBalanceFilter),
        WithdrawProtocolFeeBalanceFilter(WithdrawProtocolFeeBalanceFilter),
        WithdrawRemoteFilter(WithdrawRemoteFilter),
    }
    impl ::ethers::contract::EthLogDecode for StargateVaultEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(StargateVaultEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = BurnFilter::decode_log(log) {
                return Ok(StargateVaultEvents::BurnFilter(decoded));
            }
            if let Ok(decoded) = ChainPathUpdateFilter::decode_log(log) {
                return Ok(StargateVaultEvents::ChainPathUpdateFilter(decoded));
            }
            if let Ok(decoded) = CreditChainPathFilter::decode_log(log) {
                return Ok(StargateVaultEvents::CreditChainPathFilter(decoded));
            }
            if let Ok(decoded) = DeltaParamUpdatedFilter::decode_log(log) {
                return Ok(StargateVaultEvents::DeltaParamUpdatedFilter(decoded));
            }
            if let Ok(decoded) = FeeLibraryUpdatedFilter::decode_log(log) {
                return Ok(StargateVaultEvents::FeeLibraryUpdatedFilter(decoded));
            }
            if let Ok(decoded) = FeesUpdatedFilter::decode_log(log) {
                return Ok(StargateVaultEvents::FeesUpdatedFilter(decoded));
            }
            if let Ok(decoded) = InstantRedeemLocalFilter::decode_log(log) {
                return Ok(StargateVaultEvents::InstantRedeemLocalFilter(decoded));
            }
            if let Ok(decoded) = MintFilter::decode_log(log) {
                return Ok(StargateVaultEvents::MintFilter(decoded));
            }
            if let Ok(decoded) = RedeemLocalFilter::decode_log(log) {
                return Ok(StargateVaultEvents::RedeemLocalFilter(decoded));
            }
            if let Ok(decoded) = RedeemLocalCallbackFilter::decode_log(log) {
                return Ok(StargateVaultEvents::RedeemLocalCallbackFilter(decoded));
            }
            if let Ok(decoded) = RedeemRemoteFilter::decode_log(log) {
                return Ok(StargateVaultEvents::RedeemRemoteFilter(decoded));
            }
            if let Ok(decoded) = SendCreditsFilter::decode_log(log) {
                return Ok(StargateVaultEvents::SendCreditsFilter(decoded));
            }
            if let Ok(decoded) = StopSwapUpdatedFilter::decode_log(log) {
                return Ok(StargateVaultEvents::StopSwapUpdatedFilter(decoded));
            }
            if let Ok(decoded) = SwapFilter::decode_log(log) {
                return Ok(StargateVaultEvents::SwapFilter(decoded));
            }
            if let Ok(decoded) = SwapRemoteFilter::decode_log(log) {
                return Ok(StargateVaultEvents::SwapRemoteFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(StargateVaultEvents::TransferFilter(decoded));
            }
            if let Ok(decoded) = WithdrawMintFeeBalanceFilter::decode_log(log) {
                return Ok(StargateVaultEvents::WithdrawMintFeeBalanceFilter(decoded));
            }
            if let Ok(decoded) = WithdrawProtocolFeeBalanceFilter::decode_log(log) {
                return Ok(
                    StargateVaultEvents::WithdrawProtocolFeeBalanceFilter(decoded),
                );
            }
            if let Ok(decoded) = WithdrawRemoteFilter::decode_log(log) {
                return Ok(StargateVaultEvents::WithdrawRemoteFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for StargateVaultEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::BurnFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChainPathUpdateFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CreditChainPathFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DeltaParamUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FeeLibraryUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FeesUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InstantRedeemLocalFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MintFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RedeemLocalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RedeemLocalCallbackFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RedeemRemoteFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SendCreditsFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::StopSwapUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapRemoteFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawMintFeeBalanceFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawProtocolFeeBalanceFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawRemoteFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for StargateVaultEvents {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<BurnFilter> for StargateVaultEvents {
        fn from(value: BurnFilter) -> Self {
            Self::BurnFilter(value)
        }
    }
    impl ::core::convert::From<ChainPathUpdateFilter> for StargateVaultEvents {
        fn from(value: ChainPathUpdateFilter) -> Self {
            Self::ChainPathUpdateFilter(value)
        }
    }
    impl ::core::convert::From<CreditChainPathFilter> for StargateVaultEvents {
        fn from(value: CreditChainPathFilter) -> Self {
            Self::CreditChainPathFilter(value)
        }
    }
    impl ::core::convert::From<DeltaParamUpdatedFilter> for StargateVaultEvents {
        fn from(value: DeltaParamUpdatedFilter) -> Self {
            Self::DeltaParamUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<FeeLibraryUpdatedFilter> for StargateVaultEvents {
        fn from(value: FeeLibraryUpdatedFilter) -> Self {
            Self::FeeLibraryUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<FeesUpdatedFilter> for StargateVaultEvents {
        fn from(value: FeesUpdatedFilter) -> Self {
            Self::FeesUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<InstantRedeemLocalFilter> for StargateVaultEvents {
        fn from(value: InstantRedeemLocalFilter) -> Self {
            Self::InstantRedeemLocalFilter(value)
        }
    }
    impl ::core::convert::From<MintFilter> for StargateVaultEvents {
        fn from(value: MintFilter) -> Self {
            Self::MintFilter(value)
        }
    }
    impl ::core::convert::From<RedeemLocalFilter> for StargateVaultEvents {
        fn from(value: RedeemLocalFilter) -> Self {
            Self::RedeemLocalFilter(value)
        }
    }
    impl ::core::convert::From<RedeemLocalCallbackFilter> for StargateVaultEvents {
        fn from(value: RedeemLocalCallbackFilter) -> Self {
            Self::RedeemLocalCallbackFilter(value)
        }
    }
    impl ::core::convert::From<RedeemRemoteFilter> for StargateVaultEvents {
        fn from(value: RedeemRemoteFilter) -> Self {
            Self::RedeemRemoteFilter(value)
        }
    }
    impl ::core::convert::From<SendCreditsFilter> for StargateVaultEvents {
        fn from(value: SendCreditsFilter) -> Self {
            Self::SendCreditsFilter(value)
        }
    }
    impl ::core::convert::From<StopSwapUpdatedFilter> for StargateVaultEvents {
        fn from(value: StopSwapUpdatedFilter) -> Self {
            Self::StopSwapUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<SwapFilter> for StargateVaultEvents {
        fn from(value: SwapFilter) -> Self {
            Self::SwapFilter(value)
        }
    }
    impl ::core::convert::From<SwapRemoteFilter> for StargateVaultEvents {
        fn from(value: SwapRemoteFilter) -> Self {
            Self::SwapRemoteFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for StargateVaultEvents {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawMintFeeBalanceFilter> for StargateVaultEvents {
        fn from(value: WithdrawMintFeeBalanceFilter) -> Self {
            Self::WithdrawMintFeeBalanceFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawProtocolFeeBalanceFilter>
    for StargateVaultEvents {
        fn from(value: WithdrawProtocolFeeBalanceFilter) -> Self {
            Self::WithdrawProtocolFeeBalanceFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawRemoteFilter> for StargateVaultEvents {
        fn from(value: WithdrawRemoteFilter) -> Self {
            Self::WithdrawRemoteFilter(value)
        }
    }
    ///Container type for all input parameters for the `BP_DENOMINATOR` function with signature `BP_DENOMINATOR()` and selector `0xabe685cd`
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
    #[ethcall(name = "BP_DENOMINATOR", abi = "BP_DENOMINATOR()")]
    pub struct BpDenominatorCall;
    ///Container type for all input parameters for the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
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
    #[ethcall(name = "DOMAIN_SEPARATOR", abi = "DOMAIN_SEPARATOR()")]
    pub struct DomainSeparatorCall;
    ///Container type for all input parameters for the `PERMIT_TYPEHASH` function with signature `PERMIT_TYPEHASH()` and selector `0x30adf81f`
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
    #[ethcall(name = "PERMIT_TYPEHASH", abi = "PERMIT_TYPEHASH()")]
    pub struct PermitTypehashCall;
    ///Container type for all input parameters for the `activateChainPath` function with signature `activateChainPath(uint16,uint256)` and selector `0x8bd86d0a`
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
    #[ethcall(name = "activateChainPath", abi = "activateChainPath(uint16,uint256)")]
    pub struct ActivateChainPathCall {
        pub dst_chain_id: u16,
        pub dst_pool_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
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
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
    ///Container type for all input parameters for the `amountLPtoLD` function with signature `amountLPtoLD(uint256)` and selector `0xf6cd35ee`
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
    #[ethcall(name = "amountLPtoLD", abi = "amountLPtoLD(uint256)")]
    pub struct AmountLPtoLDCall {
        pub amount_lp: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
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
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
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
    pub struct BalanceOfCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `batched` function with signature `batched()` and selector `0x27f92376`
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
    #[ethcall(name = "batched", abi = "batched()")]
    pub struct BatchedCall;
    ///Container type for all input parameters for the `callDelta` function with signature `callDelta(bool)` and selector `0x7fb65265`
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
    #[ethcall(name = "callDelta", abi = "callDelta(bool)")]
    pub struct CallDeltaCall {
        pub full_mode: bool,
    }
    ///Container type for all input parameters for the `chainPathIndexLookup` function with signature `chainPathIndexLookup(uint16,uint256)` and selector `0x64c5f02d`
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
        name = "chainPathIndexLookup",
        abi = "chainPathIndexLookup(uint16,uint256)"
    )]
    pub struct ChainPathIndexLookupCall(pub u16, pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `chainPaths` function with signature `chainPaths(uint256)` and selector `0xa138ed6b`
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
    #[ethcall(name = "chainPaths", abi = "chainPaths(uint256)")]
    pub struct ChainPathsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `convertRate` function with signature `convertRate()` and selector `0xfeb56b15`
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
    #[ethcall(name = "convertRate", abi = "convertRate()")]
    pub struct ConvertRateCall;
    ///Container type for all input parameters for the `createChainPath` function with signature `createChainPath(uint16,uint256,uint256)` and selector `0x20d6bc75`
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
    #[ethcall(name = "createChainPath", abi = "createChainPath(uint16,uint256,uint256)")]
    pub struct CreateChainPathCall {
        pub dst_chain_id: u16,
        pub dst_pool_id: ::ethers::core::types::U256,
        pub weight: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `creditChainPath` function with signature `creditChainPath(uint16,uint256,(uint256,uint256))` and selector `0xb6addec7`
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
        name = "creditChainPath",
        abi = "creditChainPath(uint16,uint256,(uint256,uint256))"
    )]
    pub struct CreditChainPathCall {
        pub dst_chain_id: u16,
        pub dst_pool_id: ::ethers::core::types::U256,
        pub c: CreditObj,
    }
    ///Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `0x313ce567`
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
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    ///Container type for all input parameters for the `decreaseAllowance` function with signature `decreaseAllowance(address,uint256)` and selector `0xa457c2d7`
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
    #[ethcall(name = "decreaseAllowance", abi = "decreaseAllowance(address,uint256)")]
    pub struct DecreaseAllowanceCall {
        pub spender: ::ethers::core::types::Address,
        pub subtracted_value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `defaultLPMode` function with signature `defaultLPMode()` and selector `0x28f079c2`
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
    #[ethcall(name = "defaultLPMode", abi = "defaultLPMode()")]
    pub struct DefaultLPModeCall;
    ///Container type for all input parameters for the `defaultSwapMode` function with signature `defaultSwapMode()` and selector `0x99a22d68`
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
    #[ethcall(name = "defaultSwapMode", abi = "defaultSwapMode()")]
    pub struct DefaultSwapModeCall;
    ///Container type for all input parameters for the `deltaCredit` function with signature `deltaCredit()` and selector `0x1e8e51da`
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
    #[ethcall(name = "deltaCredit", abi = "deltaCredit()")]
    pub struct DeltaCreditCall;
    ///Container type for all input parameters for the `eqFeePool` function with signature `eqFeePool()` and selector `0x9bb81119`
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
    #[ethcall(name = "eqFeePool", abi = "eqFeePool()")]
    pub struct EqFeePoolCall;
    ///Container type for all input parameters for the `feeLibrary` function with signature `feeLibrary()` and selector `0x001edfab`
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
    #[ethcall(name = "feeLibrary", abi = "feeLibrary()")]
    pub struct FeeLibraryCall;
    ///Container type for all input parameters for the `getChainPath` function with signature `getChainPath(uint16,uint256)` and selector `0x159f6add`
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
    #[ethcall(name = "getChainPath", abi = "getChainPath(uint16,uint256)")]
    pub struct GetChainPathCall {
        pub dst_chain_id: u16,
        pub dst_pool_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getChainPathsLength` function with signature `getChainPathsLength()` and selector `0x163ef490`
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
    #[ethcall(name = "getChainPathsLength", abi = "getChainPathsLength()")]
    pub struct GetChainPathsLengthCall;
    ///Container type for all input parameters for the `increaseAllowance` function with signature `increaseAllowance(address,uint256)` and selector `0x39509351`
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
    #[ethcall(name = "increaseAllowance", abi = "increaseAllowance(address,uint256)")]
    pub struct IncreaseAllowanceCall {
        pub spender: ::ethers::core::types::Address,
        pub added_value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `instantRedeemLocal` function with signature `instantRedeemLocal(address,uint256,address)` and selector `0x0986b61a`
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
        name = "instantRedeemLocal",
        abi = "instantRedeemLocal(address,uint256,address)"
    )]
    pub struct InstantRedeemLocalCall {
        pub from: ::ethers::core::types::Address,
        pub amount_lp: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `localDecimals` function with signature `localDecimals()` and selector `0xe46e7058`
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
    #[ethcall(name = "localDecimals", abi = "localDecimals()")]
    pub struct LocalDecimalsCall;
    ///Container type for all input parameters for the `lpDeltaBP` function with signature `lpDeltaBP()` and selector `0x36448777`
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
    #[ethcall(name = "lpDeltaBP", abi = "lpDeltaBP()")]
    pub struct LpDeltaBPCall;
    ///Container type for all input parameters for the `mint` function with signature `mint(address,uint256)` and selector `0x40c10f19`
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
    #[ethcall(name = "mint", abi = "mint(address,uint256)")]
    pub struct MintCall {
        pub to: ::ethers::core::types::Address,
        pub amount_ld: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `mintFeeBP` function with signature `mintFeeBP()` and selector `0xfaa24f07`
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
    #[ethcall(name = "mintFeeBP", abi = "mintFeeBP()")]
    pub struct MintFeeBPCall;
    ///Container type for all input parameters for the `mintFeeBalance` function with signature `mintFeeBalance()` and selector `0x65152f2b`
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
    #[ethcall(name = "mintFeeBalance", abi = "mintFeeBalance()")]
    pub struct MintFeeBalanceCall;
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
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
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    ///Container type for all input parameters for the `nonces` function with signature `nonces(address)` and selector `0x7ecebe00`
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
    #[ethcall(name = "nonces", abi = "nonces(address)")]
    pub struct NoncesCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `permit` function with signature `permit(address,address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xd505accf`
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
        name = "permit",
        abi = "permit(address,address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct PermitCall {
        pub owner: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub deadline: ::ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `poolId` function with signature `poolId()` and selector `0x3e0dc34e`
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
    #[ethcall(name = "poolId", abi = "poolId()")]
    pub struct PoolIdCall;
    ///Container type for all input parameters for the `protocolFeeBalance` function with signature `protocolFeeBalance()` and selector `0x0a22d68c`
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
    #[ethcall(name = "protocolFeeBalance", abi = "protocolFeeBalance()")]
    pub struct ProtocolFeeBalanceCall;
    ///Container type for all input parameters for the `redeemLocal` function with signature `redeemLocal(address,uint256,uint16,uint256,bytes)` and selector `0xb0fab0bc`
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
        name = "redeemLocal",
        abi = "redeemLocal(address,uint256,uint16,uint256,bytes)"
    )]
    pub struct RedeemLocalCall {
        pub from: ::ethers::core::types::Address,
        pub amount_lp: ::ethers::core::types::U256,
        pub dst_chain_id: u16,
        pub dst_pool_id: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `redeemLocalCallback` function with signature `redeemLocalCallback(uint16,uint256,address,uint256,uint256)` and selector `0xb30daeac`
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
        name = "redeemLocalCallback",
        abi = "redeemLocalCallback(uint16,uint256,address,uint256,uint256)"
    )]
    pub struct RedeemLocalCallbackCall {
        pub src_chain_id: u16,
        pub src_pool_id: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Address,
        pub amount_sd: ::ethers::core::types::U256,
        pub amount_to_mint_sd: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `redeemLocalCheckOnRemote` function with signature `redeemLocalCheckOnRemote(uint16,uint256,uint256)` and selector `0xea89e2aa`
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
        name = "redeemLocalCheckOnRemote",
        abi = "redeemLocalCheckOnRemote(uint16,uint256,uint256)"
    )]
    pub struct RedeemLocalCheckOnRemoteCall {
        pub src_chain_id: u16,
        pub src_pool_id: ::ethers::core::types::U256,
        pub amount_sd: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `redeemRemote` function with signature `redeemRemote(uint16,uint256,address,uint256)` and selector `0x7298a5dc`
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
        name = "redeemRemote",
        abi = "redeemRemote(uint16,uint256,address,uint256)"
    )]
    pub struct RedeemRemoteCall {
        pub dst_chain_id: u16,
        pub dst_pool_id: ::ethers::core::types::U256,
        pub from: ::ethers::core::types::Address,
        pub amount_lp: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `router` function with signature `router()` and selector `0xf887ea40`
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
    #[ethcall(name = "router", abi = "router()")]
    pub struct RouterCall;
    ///Container type for all input parameters for the `sendCredits` function with signature `sendCredits(uint16,uint256)` and selector `0x08e9d8c2`
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
    #[ethcall(name = "sendCredits", abi = "sendCredits(uint16,uint256)")]
    pub struct SendCreditsCall {
        pub dst_chain_id: u16,
        pub dst_pool_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setDeltaParam` function with signature `setDeltaParam(bool,uint256,uint256,bool,bool)` and selector `0xe065608b`
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
        name = "setDeltaParam",
        abi = "setDeltaParam(bool,uint256,uint256,bool,bool)"
    )]
    pub struct SetDeltaParamCall {
        pub batched: bool,
        pub swap_delta_bp: ::ethers::core::types::U256,
        pub lp_delta_bp: ::ethers::core::types::U256,
        pub default_swap_mode: bool,
        pub default_lp_mode: bool,
    }
    ///Container type for all input parameters for the `setFee` function with signature `setFee(uint256)` and selector `0x69fe0e2d`
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
    #[ethcall(name = "setFee", abi = "setFee(uint256)")]
    pub struct SetFeeCall {
        pub mint_fee_bp: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setFeeLibrary` function with signature `setFeeLibrary(address)` and selector `0x4b5cacbc`
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
    #[ethcall(name = "setFeeLibrary", abi = "setFeeLibrary(address)")]
    pub struct SetFeeLibraryCall {
        pub fee_library_addr: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setSwapStop` function with signature `setSwapStop(bool)` and selector `0xac2cc36b`
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
    #[ethcall(name = "setSwapStop", abi = "setSwapStop(bool)")]
    pub struct SetSwapStopCall {
        pub swap_stop: bool,
    }
    ///Container type for all input parameters for the `setWeightForChainPath` function with signature `setWeightForChainPath(uint16,uint256,uint16)` and selector `0xa985565f`
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
        name = "setWeightForChainPath",
        abi = "setWeightForChainPath(uint16,uint256,uint16)"
    )]
    pub struct SetWeightForChainPathCall {
        pub dst_chain_id: u16,
        pub dst_pool_id: ::ethers::core::types::U256,
        pub weight: u16,
    }
    ///Container type for all input parameters for the `sharedDecimals` function with signature `sharedDecimals()` and selector `0x857749b0`
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
    #[ethcall(name = "sharedDecimals", abi = "sharedDecimals()")]
    pub struct SharedDecimalsCall;
    ///Container type for all input parameters for the `stopSwap` function with signature `stopSwap()` and selector `0xb633b364`
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
    #[ethcall(name = "stopSwap", abi = "stopSwap()")]
    pub struct StopSwapCall;
    ///Container type for all input parameters for the `swap` function with signature `swap(uint16,uint256,address,uint256,uint256,bool)` and selector `0x1b7319b6`
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
    #[ethcall(name = "swap", abi = "swap(uint16,uint256,address,uint256,uint256,bool)")]
    pub struct SwapCall {
        pub dst_chain_id: u16,
        pub dst_pool_id: ::ethers::core::types::U256,
        pub from: ::ethers::core::types::Address,
        pub amount_ld: ::ethers::core::types::U256,
        pub min_amount_ld: ::ethers::core::types::U256,
        pub new_liquidity: bool,
    }
    ///Container type for all input parameters for the `swapDeltaBP` function with signature `swapDeltaBP()` and selector `0xcdfed0ab`
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
    #[ethcall(name = "swapDeltaBP", abi = "swapDeltaBP()")]
    pub struct SwapDeltaBPCall;
    ///Container type for all input parameters for the `swapRemote` function with signature `swapRemote(uint16,uint256,address,(uint256,uint256,uint256,uint256,uint256,uint256))` and selector `0x902b8ab7`
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
        name = "swapRemote",
        abi = "swapRemote(uint16,uint256,address,(uint256,uint256,uint256,uint256,uint256,uint256))"
    )]
    pub struct SwapRemoteCall {
        pub src_chain_id: u16,
        pub src_pool_id: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Address,
        pub s: SwapObj,
    }
    ///Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    ///Container type for all input parameters for the `token` function with signature `token()` and selector `0xfc0c546a`
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
    #[ethcall(name = "token", abi = "token()")]
    pub struct TokenCall;
    ///Container type for all input parameters for the `totalLiquidity` function with signature `totalLiquidity()` and selector `0x15770f92`
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
    #[ethcall(name = "totalLiquidity", abi = "totalLiquidity()")]
    pub struct TotalLiquidityCall;
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
    ///Container type for all input parameters for the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
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
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
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
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `withdrawMintFeeBalance` function with signature `withdrawMintFeeBalance(address)` and selector `0x476efe40`
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
    #[ethcall(name = "withdrawMintFeeBalance", abi = "withdrawMintFeeBalance(address)")]
    pub struct WithdrawMintFeeBalanceCall {
        pub to: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `withdrawProtocolFeeBalance` function with signature `withdrawProtocolFeeBalance(address)` and selector `0xbe310294`
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
        name = "withdrawProtocolFeeBalance",
        abi = "withdrawProtocolFeeBalance(address)"
    )]
    pub struct WithdrawProtocolFeeBalanceCall {
        pub to: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum StargateVaultCalls {
        BpDenominator(BpDenominatorCall),
        DomainSeparator(DomainSeparatorCall),
        PermitTypehash(PermitTypehashCall),
        ActivateChainPath(ActivateChainPathCall),
        Allowance(AllowanceCall),
        AmountLPtoLD(AmountLPtoLDCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Batched(BatchedCall),
        CallDelta(CallDeltaCall),
        ChainPathIndexLookup(ChainPathIndexLookupCall),
        ChainPaths(ChainPathsCall),
        ConvertRate(ConvertRateCall),
        CreateChainPath(CreateChainPathCall),
        CreditChainPath(CreditChainPathCall),
        Decimals(DecimalsCall),
        DecreaseAllowance(DecreaseAllowanceCall),
        DefaultLPMode(DefaultLPModeCall),
        DefaultSwapMode(DefaultSwapModeCall),
        DeltaCredit(DeltaCreditCall),
        EqFeePool(EqFeePoolCall),
        FeeLibrary(FeeLibraryCall),
        GetChainPath(GetChainPathCall),
        GetChainPathsLength(GetChainPathsLengthCall),
        IncreaseAllowance(IncreaseAllowanceCall),
        InstantRedeemLocal(InstantRedeemLocalCall),
        LocalDecimals(LocalDecimalsCall),
        LpDeltaBP(LpDeltaBPCall),
        Mint(MintCall),
        MintFeeBP(MintFeeBPCall),
        MintFeeBalance(MintFeeBalanceCall),
        Name(NameCall),
        Nonces(NoncesCall),
        Permit(PermitCall),
        PoolId(PoolIdCall),
        ProtocolFeeBalance(ProtocolFeeBalanceCall),
        RedeemLocal(RedeemLocalCall),
        RedeemLocalCallback(RedeemLocalCallbackCall),
        RedeemLocalCheckOnRemote(RedeemLocalCheckOnRemoteCall),
        RedeemRemote(RedeemRemoteCall),
        Router(RouterCall),
        SendCredits(SendCreditsCall),
        SetDeltaParam(SetDeltaParamCall),
        SetFee(SetFeeCall),
        SetFeeLibrary(SetFeeLibraryCall),
        SetSwapStop(SetSwapStopCall),
        SetWeightForChainPath(SetWeightForChainPathCall),
        SharedDecimals(SharedDecimalsCall),
        StopSwap(StopSwapCall),
        Swap(SwapCall),
        SwapDeltaBP(SwapDeltaBPCall),
        SwapRemote(SwapRemoteCall),
        Symbol(SymbolCall),
        Token(TokenCall),
        TotalLiquidity(TotalLiquidityCall),
        TotalSupply(TotalSupplyCall),
        TotalWeight(TotalWeightCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        WithdrawMintFeeBalance(WithdrawMintFeeBalanceCall),
        WithdrawProtocolFeeBalance(WithdrawProtocolFeeBalanceCall),
    }
    impl ::ethers::core::abi::AbiDecode for StargateVaultCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <BpDenominatorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BpDenominator(decoded));
            }
            if let Ok(decoded)
                = <DomainSeparatorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DomainSeparator(decoded));
            }
            if let Ok(decoded)
                = <PermitTypehashCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PermitTypehash(decoded));
            }
            if let Ok(decoded)
                = <ActivateChainPathCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ActivateChainPath(decoded));
            }
            if let Ok(decoded)
                = <AllowanceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Allowance(decoded));
            }
            if let Ok(decoded)
                = <AmountLPtoLDCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AmountLPtoLD(decoded));
            }
            if let Ok(decoded)
                = <ApproveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Approve(decoded));
            }
            if let Ok(decoded)
                = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded)
                = <BatchedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Batched(decoded));
            }
            if let Ok(decoded)
                = <CallDeltaCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CallDelta(decoded));
            }
            if let Ok(decoded)
                = <ChainPathIndexLookupCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ChainPathIndexLookup(decoded));
            }
            if let Ok(decoded)
                = <ChainPathsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ChainPaths(decoded));
            }
            if let Ok(decoded)
                = <ConvertRateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ConvertRate(decoded));
            }
            if let Ok(decoded)
                = <CreateChainPathCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CreateChainPath(decoded));
            }
            if let Ok(decoded)
                = <CreditChainPathCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CreditChainPath(decoded));
            }
            if let Ok(decoded)
                = <DecimalsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Decimals(decoded));
            }
            if let Ok(decoded)
                = <DecreaseAllowanceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DecreaseAllowance(decoded));
            }
            if let Ok(decoded)
                = <DefaultLPModeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DefaultLPMode(decoded));
            }
            if let Ok(decoded)
                = <DefaultSwapModeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DefaultSwapMode(decoded));
            }
            if let Ok(decoded)
                = <DeltaCreditCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DeltaCredit(decoded));
            }
            if let Ok(decoded)
                = <EqFeePoolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::EqFeePool(decoded));
            }
            if let Ok(decoded)
                = <FeeLibraryCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FeeLibrary(decoded));
            }
            if let Ok(decoded)
                = <GetChainPathCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetChainPath(decoded));
            }
            if let Ok(decoded)
                = <GetChainPathsLengthCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetChainPathsLength(decoded));
            }
            if let Ok(decoded)
                = <IncreaseAllowanceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IncreaseAllowance(decoded));
            }
            if let Ok(decoded)
                = <InstantRedeemLocalCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InstantRedeemLocal(decoded));
            }
            if let Ok(decoded)
                = <LocalDecimalsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LocalDecimals(decoded));
            }
            if let Ok(decoded)
                = <LpDeltaBPCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LpDeltaBP(decoded));
            }
            if let Ok(decoded)
                = <MintCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Mint(decoded));
            }
            if let Ok(decoded)
                = <MintFeeBPCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MintFeeBP(decoded));
            }
            if let Ok(decoded)
                = <MintFeeBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MintFeeBalance(decoded));
            }
            if let Ok(decoded)
                = <NameCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded)
                = <NoncesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Nonces(decoded));
            }
            if let Ok(decoded)
                = <PermitCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Permit(decoded));
            }
            if let Ok(decoded)
                = <PoolIdCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PoolId(decoded));
            }
            if let Ok(decoded)
                = <ProtocolFeeBalanceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ProtocolFeeBalance(decoded));
            }
            if let Ok(decoded)
                = <RedeemLocalCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RedeemLocal(decoded));
            }
            if let Ok(decoded)
                = <RedeemLocalCallbackCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RedeemLocalCallback(decoded));
            }
            if let Ok(decoded)
                = <RedeemLocalCheckOnRemoteCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RedeemLocalCheckOnRemote(decoded));
            }
            if let Ok(decoded)
                = <RedeemRemoteCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RedeemRemote(decoded));
            }
            if let Ok(decoded)
                = <RouterCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Router(decoded));
            }
            if let Ok(decoded)
                = <SendCreditsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SendCredits(decoded));
            }
            if let Ok(decoded)
                = <SetDeltaParamCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetDeltaParam(decoded));
            }
            if let Ok(decoded)
                = <SetFeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetFee(decoded));
            }
            if let Ok(decoded)
                = <SetFeeLibraryCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetFeeLibrary(decoded));
            }
            if let Ok(decoded)
                = <SetSwapStopCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetSwapStop(decoded));
            }
            if let Ok(decoded)
                = <SetWeightForChainPathCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetWeightForChainPath(decoded));
            }
            if let Ok(decoded)
                = <SharedDecimalsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SharedDecimals(decoded));
            }
            if let Ok(decoded)
                = <StopSwapCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::StopSwap(decoded));
            }
            if let Ok(decoded)
                = <SwapCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Swap(decoded));
            }
            if let Ok(decoded)
                = <SwapDeltaBPCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SwapDeltaBP(decoded));
            }
            if let Ok(decoded)
                = <SwapRemoteCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SwapRemote(decoded));
            }
            if let Ok(decoded)
                = <SymbolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Symbol(decoded));
            }
            if let Ok(decoded)
                = <TokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Token(decoded));
            }
            if let Ok(decoded)
                = <TotalLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TotalLiquidity(decoded));
            }
            if let Ok(decoded)
                = <TotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TotalSupply(decoded));
            }
            if let Ok(decoded)
                = <TotalWeightCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TotalWeight(decoded));
            }
            if let Ok(decoded)
                = <TransferCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Transfer(decoded));
            }
            if let Ok(decoded)
                = <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TransferFrom(decoded));
            }
            if let Ok(decoded)
                = <WithdrawMintFeeBalanceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::WithdrawMintFeeBalance(decoded));
            }
            if let Ok(decoded)
                = <WithdrawProtocolFeeBalanceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::WithdrawProtocolFeeBalance(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for StargateVaultCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BpDenominator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DomainSeparator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PermitTypehash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ActivateChainPath(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Allowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AmountLPtoLD(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Batched(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CallDelta(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChainPathIndexLookup(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChainPaths(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ConvertRate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreateChainPath(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreditChainPath(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Decimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DecreaseAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DefaultLPMode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DefaultSwapMode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeltaCredit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EqFeePool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FeeLibrary(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetChainPath(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetChainPathsLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IncreaseAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InstantRedeemLocal(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LocalDecimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LpDeltaBP(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Mint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MintFeeBP(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MintFeeBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Nonces(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Permit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PoolId(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProtocolFeeBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RedeemLocal(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RedeemLocalCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RedeemLocalCheckOnRemote(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RedeemRemote(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Router(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SendCredits(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetDeltaParam(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetFeeLibrary(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetSwapStop(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetWeightForChainPath(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SharedDecimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StopSwap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Swap(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SwapDeltaBP(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapRemote(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Symbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Token(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalWeight(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Transfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawMintFeeBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawProtocolFeeBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for StargateVaultCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BpDenominator(element) => ::core::fmt::Display::fmt(element, f),
                Self::DomainSeparator(element) => ::core::fmt::Display::fmt(element, f),
                Self::PermitTypehash(element) => ::core::fmt::Display::fmt(element, f),
                Self::ActivateChainPath(element) => ::core::fmt::Display::fmt(element, f),
                Self::Allowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::AmountLPtoLD(element) => ::core::fmt::Display::fmt(element, f),
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Batched(element) => ::core::fmt::Display::fmt(element, f),
                Self::CallDelta(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChainPathIndexLookup(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChainPaths(element) => ::core::fmt::Display::fmt(element, f),
                Self::ConvertRate(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateChainPath(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreditChainPath(element) => ::core::fmt::Display::fmt(element, f),
                Self::Decimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::DecreaseAllowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultLPMode(element) => ::core::fmt::Display::fmt(element, f),
                Self::DefaultSwapMode(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeltaCredit(element) => ::core::fmt::Display::fmt(element, f),
                Self::EqFeePool(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeeLibrary(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetChainPath(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetChainPathsLength(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IncreaseAllowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::InstantRedeemLocal(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LocalDecimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::LpDeltaBP(element) => ::core::fmt::Display::fmt(element, f),
                Self::Mint(element) => ::core::fmt::Display::fmt(element, f),
                Self::MintFeeBP(element) => ::core::fmt::Display::fmt(element, f),
                Self::MintFeeBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nonces(element) => ::core::fmt::Display::fmt(element, f),
                Self::Permit(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolId(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProtocolFeeBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RedeemLocal(element) => ::core::fmt::Display::fmt(element, f),
                Self::RedeemLocalCallback(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RedeemLocalCheckOnRemote(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RedeemRemote(element) => ::core::fmt::Display::fmt(element, f),
                Self::Router(element) => ::core::fmt::Display::fmt(element, f),
                Self::SendCredits(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetDeltaParam(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFeeLibrary(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetSwapStop(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetWeightForChainPath(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SharedDecimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::StopSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::Swap(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapDeltaBP(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapRemote(element) => ::core::fmt::Display::fmt(element, f),
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::Token(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalWeight(element) => ::core::fmt::Display::fmt(element, f),
                Self::Transfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawMintFeeBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawProtocolFeeBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<BpDenominatorCall> for StargateVaultCalls {
        fn from(value: BpDenominatorCall) -> Self {
            Self::BpDenominator(value)
        }
    }
    impl ::core::convert::From<DomainSeparatorCall> for StargateVaultCalls {
        fn from(value: DomainSeparatorCall) -> Self {
            Self::DomainSeparator(value)
        }
    }
    impl ::core::convert::From<PermitTypehashCall> for StargateVaultCalls {
        fn from(value: PermitTypehashCall) -> Self {
            Self::PermitTypehash(value)
        }
    }
    impl ::core::convert::From<ActivateChainPathCall> for StargateVaultCalls {
        fn from(value: ActivateChainPathCall) -> Self {
            Self::ActivateChainPath(value)
        }
    }
    impl ::core::convert::From<AllowanceCall> for StargateVaultCalls {
        fn from(value: AllowanceCall) -> Self {
            Self::Allowance(value)
        }
    }
    impl ::core::convert::From<AmountLPtoLDCall> for StargateVaultCalls {
        fn from(value: AmountLPtoLDCall) -> Self {
            Self::AmountLPtoLD(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for StargateVaultCalls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for StargateVaultCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<BatchedCall> for StargateVaultCalls {
        fn from(value: BatchedCall) -> Self {
            Self::Batched(value)
        }
    }
    impl ::core::convert::From<CallDeltaCall> for StargateVaultCalls {
        fn from(value: CallDeltaCall) -> Self {
            Self::CallDelta(value)
        }
    }
    impl ::core::convert::From<ChainPathIndexLookupCall> for StargateVaultCalls {
        fn from(value: ChainPathIndexLookupCall) -> Self {
            Self::ChainPathIndexLookup(value)
        }
    }
    impl ::core::convert::From<ChainPathsCall> for StargateVaultCalls {
        fn from(value: ChainPathsCall) -> Self {
            Self::ChainPaths(value)
        }
    }
    impl ::core::convert::From<ConvertRateCall> for StargateVaultCalls {
        fn from(value: ConvertRateCall) -> Self {
            Self::ConvertRate(value)
        }
    }
    impl ::core::convert::From<CreateChainPathCall> for StargateVaultCalls {
        fn from(value: CreateChainPathCall) -> Self {
            Self::CreateChainPath(value)
        }
    }
    impl ::core::convert::From<CreditChainPathCall> for StargateVaultCalls {
        fn from(value: CreditChainPathCall) -> Self {
            Self::CreditChainPath(value)
        }
    }
    impl ::core::convert::From<DecimalsCall> for StargateVaultCalls {
        fn from(value: DecimalsCall) -> Self {
            Self::Decimals(value)
        }
    }
    impl ::core::convert::From<DecreaseAllowanceCall> for StargateVaultCalls {
        fn from(value: DecreaseAllowanceCall) -> Self {
            Self::DecreaseAllowance(value)
        }
    }
    impl ::core::convert::From<DefaultLPModeCall> for StargateVaultCalls {
        fn from(value: DefaultLPModeCall) -> Self {
            Self::DefaultLPMode(value)
        }
    }
    impl ::core::convert::From<DefaultSwapModeCall> for StargateVaultCalls {
        fn from(value: DefaultSwapModeCall) -> Self {
            Self::DefaultSwapMode(value)
        }
    }
    impl ::core::convert::From<DeltaCreditCall> for StargateVaultCalls {
        fn from(value: DeltaCreditCall) -> Self {
            Self::DeltaCredit(value)
        }
    }
    impl ::core::convert::From<EqFeePoolCall> for StargateVaultCalls {
        fn from(value: EqFeePoolCall) -> Self {
            Self::EqFeePool(value)
        }
    }
    impl ::core::convert::From<FeeLibraryCall> for StargateVaultCalls {
        fn from(value: FeeLibraryCall) -> Self {
            Self::FeeLibrary(value)
        }
    }
    impl ::core::convert::From<GetChainPathCall> for StargateVaultCalls {
        fn from(value: GetChainPathCall) -> Self {
            Self::GetChainPath(value)
        }
    }
    impl ::core::convert::From<GetChainPathsLengthCall> for StargateVaultCalls {
        fn from(value: GetChainPathsLengthCall) -> Self {
            Self::GetChainPathsLength(value)
        }
    }
    impl ::core::convert::From<IncreaseAllowanceCall> for StargateVaultCalls {
        fn from(value: IncreaseAllowanceCall) -> Self {
            Self::IncreaseAllowance(value)
        }
    }
    impl ::core::convert::From<InstantRedeemLocalCall> for StargateVaultCalls {
        fn from(value: InstantRedeemLocalCall) -> Self {
            Self::InstantRedeemLocal(value)
        }
    }
    impl ::core::convert::From<LocalDecimalsCall> for StargateVaultCalls {
        fn from(value: LocalDecimalsCall) -> Self {
            Self::LocalDecimals(value)
        }
    }
    impl ::core::convert::From<LpDeltaBPCall> for StargateVaultCalls {
        fn from(value: LpDeltaBPCall) -> Self {
            Self::LpDeltaBP(value)
        }
    }
    impl ::core::convert::From<MintCall> for StargateVaultCalls {
        fn from(value: MintCall) -> Self {
            Self::Mint(value)
        }
    }
    impl ::core::convert::From<MintFeeBPCall> for StargateVaultCalls {
        fn from(value: MintFeeBPCall) -> Self {
            Self::MintFeeBP(value)
        }
    }
    impl ::core::convert::From<MintFeeBalanceCall> for StargateVaultCalls {
        fn from(value: MintFeeBalanceCall) -> Self {
            Self::MintFeeBalance(value)
        }
    }
    impl ::core::convert::From<NameCall> for StargateVaultCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<NoncesCall> for StargateVaultCalls {
        fn from(value: NoncesCall) -> Self {
            Self::Nonces(value)
        }
    }
    impl ::core::convert::From<PermitCall> for StargateVaultCalls {
        fn from(value: PermitCall) -> Self {
            Self::Permit(value)
        }
    }
    impl ::core::convert::From<PoolIdCall> for StargateVaultCalls {
        fn from(value: PoolIdCall) -> Self {
            Self::PoolId(value)
        }
    }
    impl ::core::convert::From<ProtocolFeeBalanceCall> for StargateVaultCalls {
        fn from(value: ProtocolFeeBalanceCall) -> Self {
            Self::ProtocolFeeBalance(value)
        }
    }
    impl ::core::convert::From<RedeemLocalCall> for StargateVaultCalls {
        fn from(value: RedeemLocalCall) -> Self {
            Self::RedeemLocal(value)
        }
    }
    impl ::core::convert::From<RedeemLocalCallbackCall> for StargateVaultCalls {
        fn from(value: RedeemLocalCallbackCall) -> Self {
            Self::RedeemLocalCallback(value)
        }
    }
    impl ::core::convert::From<RedeemLocalCheckOnRemoteCall> for StargateVaultCalls {
        fn from(value: RedeemLocalCheckOnRemoteCall) -> Self {
            Self::RedeemLocalCheckOnRemote(value)
        }
    }
    impl ::core::convert::From<RedeemRemoteCall> for StargateVaultCalls {
        fn from(value: RedeemRemoteCall) -> Self {
            Self::RedeemRemote(value)
        }
    }
    impl ::core::convert::From<RouterCall> for StargateVaultCalls {
        fn from(value: RouterCall) -> Self {
            Self::Router(value)
        }
    }
    impl ::core::convert::From<SendCreditsCall> for StargateVaultCalls {
        fn from(value: SendCreditsCall) -> Self {
            Self::SendCredits(value)
        }
    }
    impl ::core::convert::From<SetDeltaParamCall> for StargateVaultCalls {
        fn from(value: SetDeltaParamCall) -> Self {
            Self::SetDeltaParam(value)
        }
    }
    impl ::core::convert::From<SetFeeCall> for StargateVaultCalls {
        fn from(value: SetFeeCall) -> Self {
            Self::SetFee(value)
        }
    }
    impl ::core::convert::From<SetFeeLibraryCall> for StargateVaultCalls {
        fn from(value: SetFeeLibraryCall) -> Self {
            Self::SetFeeLibrary(value)
        }
    }
    impl ::core::convert::From<SetSwapStopCall> for StargateVaultCalls {
        fn from(value: SetSwapStopCall) -> Self {
            Self::SetSwapStop(value)
        }
    }
    impl ::core::convert::From<SetWeightForChainPathCall> for StargateVaultCalls {
        fn from(value: SetWeightForChainPathCall) -> Self {
            Self::SetWeightForChainPath(value)
        }
    }
    impl ::core::convert::From<SharedDecimalsCall> for StargateVaultCalls {
        fn from(value: SharedDecimalsCall) -> Self {
            Self::SharedDecimals(value)
        }
    }
    impl ::core::convert::From<StopSwapCall> for StargateVaultCalls {
        fn from(value: StopSwapCall) -> Self {
            Self::StopSwap(value)
        }
    }
    impl ::core::convert::From<SwapCall> for StargateVaultCalls {
        fn from(value: SwapCall) -> Self {
            Self::Swap(value)
        }
    }
    impl ::core::convert::From<SwapDeltaBPCall> for StargateVaultCalls {
        fn from(value: SwapDeltaBPCall) -> Self {
            Self::SwapDeltaBP(value)
        }
    }
    impl ::core::convert::From<SwapRemoteCall> for StargateVaultCalls {
        fn from(value: SwapRemoteCall) -> Self {
            Self::SwapRemote(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for StargateVaultCalls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TokenCall> for StargateVaultCalls {
        fn from(value: TokenCall) -> Self {
            Self::Token(value)
        }
    }
    impl ::core::convert::From<TotalLiquidityCall> for StargateVaultCalls {
        fn from(value: TotalLiquidityCall) -> Self {
            Self::TotalLiquidity(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for StargateVaultCalls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<TotalWeightCall> for StargateVaultCalls {
        fn from(value: TotalWeightCall) -> Self {
            Self::TotalWeight(value)
        }
    }
    impl ::core::convert::From<TransferCall> for StargateVaultCalls {
        fn from(value: TransferCall) -> Self {
            Self::Transfer(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for StargateVaultCalls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    impl ::core::convert::From<WithdrawMintFeeBalanceCall> for StargateVaultCalls {
        fn from(value: WithdrawMintFeeBalanceCall) -> Self {
            Self::WithdrawMintFeeBalance(value)
        }
    }
    impl ::core::convert::From<WithdrawProtocolFeeBalanceCall> for StargateVaultCalls {
        fn from(value: WithdrawProtocolFeeBalanceCall) -> Self {
            Self::WithdrawProtocolFeeBalance(value)
        }
    }
    ///Container type for all return fields from the `BP_DENOMINATOR` function with signature `BP_DENOMINATOR()` and selector `0xabe685cd`
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
    pub struct BpDenominatorReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
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
    pub struct DomainSeparatorReturn(pub [u8; 32]);
    ///Container type for all return fields from the `PERMIT_TYPEHASH` function with signature `PERMIT_TYPEHASH()` and selector `0x30adf81f`
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
    pub struct PermitTypehashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
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
    pub struct AllowanceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `amountLPtoLD` function with signature `amountLPtoLD(uint256)` and selector `0xf6cd35ee`
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
    pub struct AmountLPtoLDReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
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
    pub struct ApproveReturn(pub bool);
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
    ///Container type for all return fields from the `batched` function with signature `batched()` and selector `0x27f92376`
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
    pub struct BatchedReturn(pub bool);
    ///Container type for all return fields from the `chainPathIndexLookup` function with signature `chainPathIndexLookup(uint16,uint256)` and selector `0x64c5f02d`
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
    pub struct ChainPathIndexLookupReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `chainPaths` function with signature `chainPaths(uint256)` and selector `0xa138ed6b`
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
    pub struct ChainPathsReturn {
        pub ready: bool,
        pub dst_chain_id: u16,
        pub dst_pool_id: ::ethers::core::types::U256,
        pub weight: ::ethers::core::types::U256,
        pub balance: ::ethers::core::types::U256,
        pub lkb: ::ethers::core::types::U256,
        pub credits: ::ethers::core::types::U256,
        pub ideal_balance: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `convertRate` function with signature `convertRate()` and selector `0xfeb56b15`
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
    pub struct ConvertRateReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `decimals` function with signature `decimals()` and selector `0x313ce567`
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
    pub struct DecimalsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `decreaseAllowance` function with signature `decreaseAllowance(address,uint256)` and selector `0xa457c2d7`
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
    pub struct DecreaseAllowanceReturn(pub bool);
    ///Container type for all return fields from the `defaultLPMode` function with signature `defaultLPMode()` and selector `0x28f079c2`
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
    pub struct DefaultLPModeReturn(pub bool);
    ///Container type for all return fields from the `defaultSwapMode` function with signature `defaultSwapMode()` and selector `0x99a22d68`
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
    pub struct DefaultSwapModeReturn(pub bool);
    ///Container type for all return fields from the `deltaCredit` function with signature `deltaCredit()` and selector `0x1e8e51da`
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
    pub struct DeltaCreditReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `eqFeePool` function with signature `eqFeePool()` and selector `0x9bb81119`
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
    pub struct EqFeePoolReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `feeLibrary` function with signature `feeLibrary()` and selector `0x001edfab`
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
    pub struct FeeLibraryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getChainPath` function with signature `getChainPath(uint16,uint256)` and selector `0x159f6add`
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
    pub struct GetChainPathReturn(pub ChainPath);
    ///Container type for all return fields from the `getChainPathsLength` function with signature `getChainPathsLength()` and selector `0x163ef490`
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
    pub struct GetChainPathsLengthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `increaseAllowance` function with signature `increaseAllowance(address,uint256)` and selector `0x39509351`
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
    pub struct IncreaseAllowanceReturn(pub bool);
    ///Container type for all return fields from the `instantRedeemLocal` function with signature `instantRedeemLocal(address,uint256,address)` and selector `0x0986b61a`
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
    pub struct InstantRedeemLocalReturn {
        pub amount_sd: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `localDecimals` function with signature `localDecimals()` and selector `0xe46e7058`
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
    pub struct LocalDecimalsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `lpDeltaBP` function with signature `lpDeltaBP()` and selector `0x36448777`
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
    pub struct LpDeltaBPReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `mint` function with signature `mint(address,uint256)` and selector `0x40c10f19`
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
    pub struct MintReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `mintFeeBP` function with signature `mintFeeBP()` and selector `0xfaa24f07`
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
    pub struct MintFeeBPReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `mintFeeBalance` function with signature `mintFeeBalance()` and selector `0x65152f2b`
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
    pub struct MintFeeBalanceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
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
    pub struct NameReturn(pub ::std::string::String);
    ///Container type for all return fields from the `nonces` function with signature `nonces(address)` and selector `0x7ecebe00`
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
    pub struct NoncesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `poolId` function with signature `poolId()` and selector `0x3e0dc34e`
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
    pub struct PoolIdReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `protocolFeeBalance` function with signature `protocolFeeBalance()` and selector `0x0a22d68c`
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
    pub struct ProtocolFeeBalanceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `redeemLocal` function with signature `redeemLocal(address,uint256,uint16,uint256,bytes)` and selector `0xb0fab0bc`
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
    pub struct RedeemLocalReturn {
        pub amount_sd: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `redeemLocalCheckOnRemote` function with signature `redeemLocalCheckOnRemote(uint16,uint256,uint256)` and selector `0xea89e2aa`
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
    pub struct RedeemLocalCheckOnRemoteReturn {
        pub swap_amount: ::ethers::core::types::U256,
        pub mint_amount: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `router` function with signature `router()` and selector `0xf887ea40`
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
    pub struct RouterReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `sendCredits` function with signature `sendCredits(uint16,uint256)` and selector `0x08e9d8c2`
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
    pub struct SendCreditsReturn {
        pub c: CreditObj,
    }
    ///Container type for all return fields from the `sharedDecimals` function with signature `sharedDecimals()` and selector `0x857749b0`
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
    pub struct SharedDecimalsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `stopSwap` function with signature `stopSwap()` and selector `0xb633b364`
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
    pub struct StopSwapReturn(pub bool);
    ///Container type for all return fields from the `swap` function with signature `swap(uint16,uint256,address,uint256,uint256,bool)` and selector `0x1b7319b6`
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
    pub struct SwapReturn(pub SwapObj);
    ///Container type for all return fields from the `swapDeltaBP` function with signature `swapDeltaBP()` and selector `0xcdfed0ab`
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
    pub struct SwapDeltaBPReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `swapRemote` function with signature `swapRemote(uint16,uint256,address,(uint256,uint256,uint256,uint256,uint256,uint256))` and selector `0x902b8ab7`
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
    pub struct SwapRemoteReturn {
        pub amount_ld: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
    pub struct SymbolReturn(pub ::std::string::String);
    ///Container type for all return fields from the `token` function with signature `token()` and selector `0xfc0c546a`
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
    pub struct TokenReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `totalLiquidity` function with signature `totalLiquidity()` and selector `0x15770f92`
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
    pub struct TotalLiquidityReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
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
    pub struct TransferReturn(pub bool);
    ///Container type for all return fields from the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
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
    pub struct TransferFromReturn(pub bool);
    ///`ChainPath(bool,uint16,uint256,uint256,uint256,uint256,uint256,uint256)`
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
    pub struct ChainPath {
        pub ready: bool,
        pub dst_chain_id: u16,
        pub dst_pool_id: ::ethers::core::types::U256,
        pub weight: ::ethers::core::types::U256,
        pub balance: ::ethers::core::types::U256,
        pub lkb: ::ethers::core::types::U256,
        pub credits: ::ethers::core::types::U256,
        pub ideal_balance: ::ethers::core::types::U256,
    }
    ///`CreditObj(uint256,uint256)`
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
    pub struct CreditObj {
        pub credits: ::ethers::core::types::U256,
        pub ideal_balance: ::ethers::core::types::U256,
    }
    ///`SwapObj(uint256,uint256,uint256,uint256,uint256,uint256)`
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
    pub struct SwapObj {
        pub amount: ::ethers::core::types::U256,
        pub eq_fee: ::ethers::core::types::U256,
        pub eq_reward: ::ethers::core::types::U256,
        pub lp_fee: ::ethers::core::types::U256,
        pub protocol_fee: ::ethers::core::types::U256,
        pub lkb_remove: ::ethers::core::types::U256,
    }
}
