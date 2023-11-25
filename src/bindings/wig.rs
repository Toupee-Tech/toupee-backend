pub use wig::*;
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
pub mod wig {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_BASE"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_supplyTOKEN"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_OTOKENFactory"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_VTOKENFactory"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_VTOKENRewarderFactory"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_TOKENFeesFactory"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("BASE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("BASE"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IERC20"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DECIMALS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DECIMALS"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DIVISOR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DIVISOR"),
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
                    ::std::borrow::ToOwned::to_owned("FEES"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("FEES"),
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
                    ::std::borrow::ToOwned::to_owned("FLOOR_PRICE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("FLOOR_PRICE"),
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
                    ::std::borrow::ToOwned::to_owned("PRECISION"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("PRECISION"),
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
                    ::std::borrow::ToOwned::to_owned("PROTOCOL_FEE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("PROTOCOL_FEE"),
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
                    ::std::borrow::ToOwned::to_owned("PROVIDER_FEE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("PROVIDER_FEE"),
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
                    ::std::borrow::ToOwned::to_owned("allowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allowance"),
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
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("borrow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("borrow"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountBase"),
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
                    ::std::borrow::ToOwned::to_owned("buy"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("buy"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountBase"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("minToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("expireTimestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("toAccount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("provider"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("debtTotal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("debtTotal"),
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
                    ::std::borrow::ToOwned::to_owned("debts"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("debts"),
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
                    ::std::borrow::ToOwned::to_owned("decimals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("decimals"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
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
                    ::std::borrow::ToOwned::to_owned("exercise"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("exercise"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountOToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("toAccount"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("frBASE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("frBASE"),
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
                    ::std::borrow::ToOwned::to_owned("getAccountCredit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAccountCredit"),
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
                    ::std::borrow::ToOwned::to_owned("getFloorPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getFloorPrice"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getMarketPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getMarketPrice"),
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
                    ::std::borrow::ToOwned::to_owned("getMaxSell"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getMaxSell"),
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
                    ::std::borrow::ToOwned::to_owned("getOTokenPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getOTokenPrice"),
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
                    ::std::borrow::ToOwned::to_owned("getTotalValueLocked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getTotalValueLocked",
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
                    ::std::borrow::ToOwned::to_owned("mrrBASE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mrrBASE"),
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
                    ::std::borrow::ToOwned::to_owned("mrrTOKEN"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mrrTOKEN"),
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
                    ::std::borrow::ToOwned::to_owned("mrvBASE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mrvBASE"),
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
                    ::std::borrow::ToOwned::to_owned("redeem"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("redeem"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("toAccount"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("repay"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("repay"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountBase"),
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
                    ::std::borrow::ToOwned::to_owned("sell"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sell"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("minBase"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("expireTimestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("toAccount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("provider"),
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
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("TOKEN__Borrow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("TOKEN__Borrow"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("TOKEN__Buy"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("TOKEN__Buy"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
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
                    ::std::borrow::ToOwned::to_owned("TOKEN__Exercise"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("TOKEN__Exercise"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
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
                    ::std::borrow::ToOwned::to_owned("TOKEN__Redeem"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("TOKEN__Redeem"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
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
                    ::std::borrow::ToOwned::to_owned("TOKEN__Repay"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("TOKEN__Repay"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                    ::std::borrow::ToOwned::to_owned("TOKEN__Sell"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("TOKEN__Sell"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
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
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("TOKEN__ExceedsBorrowCreditLimit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "TOKEN__ExceedsBorrowCreditLimit",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TOKEN__ExceedsSwapMarketReserves"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "TOKEN__ExceedsSwapMarketReserves",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "TOKEN__ExceedsSwapSlippageTolerance",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "TOKEN__ExceedsSwapSlippageTolerance",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TOKEN__InvalidDecimals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "TOKEN__InvalidDecimals",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TOKEN__InvalidZeroAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "TOKEN__InvalidZeroAddress",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TOKEN__InvalidZeroInput"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "TOKEN__InvalidZeroInput",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TOKEN__SwapExpired"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("TOKEN__SwapExpired"),
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
    pub static WIG_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct WIG<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for WIG<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for WIG<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for WIG<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for WIG<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(WIG)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> WIG<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    WIG_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `BASE` (0xec342ad0) function
        pub fn base(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([236, 52, 42, 208], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `DECIMALS` (0x2e0f2625) function
        pub fn DECIMALS(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([46, 15, 38, 37], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `DIVISOR` (0x3410fe6e) function
        pub fn divisor(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([52, 16, 254, 110], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `FEES` (0x8b7b23ee) function
        pub fn fees(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([139, 123, 35, 238], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `FLOOR_PRICE` (0xa71f48a4) function
        pub fn floor_price(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([167, 31, 72, 164], ())
                .expect("method not found (this should never happen)")
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
        ///Calls the contract's `PRECISION` (0xaaf5eb68) function
        pub fn precision(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([170, 245, 235, 104], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PROTOCOL_FEE` (0x0b4501fd) function
        pub fn protocol_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([11, 69, 1, 253], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PROVIDER_FEE` (0xf623bb8b) function
        pub fn provider_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([246, 35, 187, 139], ())
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
        ///Calls the contract's `allowance` (0xdd62ed3e) function
        pub fn allowance(
            &self,
            owner: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (owner, spender))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approve` (0x095ea7b3) function
        pub fn approve(
            &self,
            spender: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, amount))
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
        ///Calls the contract's `borrow` (0xc5ebeaec) function
        pub fn borrow(
            &self,
            amount_base: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([197, 235, 234, 236], amount_base)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `buy` (0xba002b70) function
        pub fn buy(
            &self,
            amount_base: ::ethers::core::types::U256,
            min_token: ::ethers::core::types::U256,
            expire_timestamp: ::ethers::core::types::U256,
            to_account: ::ethers::core::types::Address,
            provider: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [186, 0, 43, 112],
                    (amount_base, min_token, expire_timestamp, to_account, provider),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `debtTotal` (0xe7e7e387) function
        pub fn debt_total(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([231, 231, 227, 135], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `debts` (0x2ecd4e7d) function
        pub fn debts(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([46, 205, 78, 125], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decimals` (0x313ce567) function
        pub fn decimals(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
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
        ///Calls the contract's `exercise` (0x1b6b7298) function
        pub fn exercise(
            &self,
            amount_o_token: ::ethers::core::types::U256,
            to_account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([27, 107, 114, 152], (amount_o_token, to_account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `frBASE` (0xad6cb823) function
        pub fn fr_base(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([173, 108, 184, 35], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAccountCredit` (0x4c7ebd37) function
        pub fn get_account_credit(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([76, 126, 189, 55], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getFloorPrice` (0x2a33d6b2) function
        pub fn get_floor_price(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([42, 51, 214, 178], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMarketPrice` (0x660e16c3) function
        pub fn get_market_price(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([102, 14, 22, 195], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMaxSell` (0x466f6311) function
        pub fn get_max_sell(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([70, 111, 99, 17], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOTokenPrice` (0x006c72af) function
        pub fn get_o_token_price(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([0, 108, 114, 175], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTotalValueLocked` (0xb26025aa) function
        pub fn get_total_value_locked(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([178, 96, 37, 170], ())
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
        ///Calls the contract's `mrrBASE` (0xa0787856) function
        pub fn mrr_base(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([160, 120, 120, 86], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mrrTOKEN` (0x1b8f0bbc) function
        pub fn mrr_token(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([27, 143, 11, 188], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mrvBASE` (0xc2b96e9b) function
        pub fn mrv_base(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([194, 185, 110, 155], ())
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
        ///Calls the contract's `redeem` (0x7bde82f2) function
        pub fn redeem(
            &self,
            amount_token: ::ethers::core::types::U256,
            to_account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([123, 222, 130, 242], (amount_token, to_account))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `repay` (0x371fd8e6) function
        pub fn repay(
            &self,
            amount_base: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([55, 31, 216, 230], amount_base)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sell` (0x26124288) function
        pub fn sell(
            &self,
            amount_token: ::ethers::core::types::U256,
            min_base: ::ethers::core::types::U256,
            expire_timestamp: ::ethers::core::types::U256,
            to_account: ::ethers::core::types::Address,
            provider: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [38, 18, 66, 136],
                    (amount_token, min_base, expire_timestamp, to_account, provider),
                )
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
        ///Calls the contract's `totalSupply` (0x18160ddd) function
        pub fn total_supply(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transfer` (0xa9059cbb) function
        pub fn transfer(
            &self,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (to, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, amount))
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
        ///Gets the contract's `TOKEN__Borrow` event
        pub fn token_borrow_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TokenBorrowFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `TOKEN__Buy` event
        pub fn token_buy_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TokenBuyFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `TOKEN__Exercise` event
        pub fn token_exercise_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TokenExerciseFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `TOKEN__Redeem` event
        pub fn token_redeem_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TokenRedeemFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `TOKEN__Repay` event
        pub fn token_repay_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TokenRepayFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `TOKEN__Sell` event
        pub fn token_sell_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TokenSellFilter,
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
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, WIGEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for WIG<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `TOKEN__ExceedsBorrowCreditLimit` with signature `TOKEN__ExceedsBorrowCreditLimit()` and selector `0x9abf1d02`
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
        name = "TOKEN__ExceedsBorrowCreditLimit",
        abi = "TOKEN__ExceedsBorrowCreditLimit()"
    )]
    pub struct TOKEN__ExceedsBorrowCreditLimit;
    ///Custom Error type `TOKEN__ExceedsSwapMarketReserves` with signature `TOKEN__ExceedsSwapMarketReserves()` and selector `0x9e0cbc95`
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
        name = "TOKEN__ExceedsSwapMarketReserves",
        abi = "TOKEN__ExceedsSwapMarketReserves()"
    )]
    pub struct TOKEN__ExceedsSwapMarketReserves;
    ///Custom Error type `TOKEN__ExceedsSwapSlippageTolerance` with signature `TOKEN__ExceedsSwapSlippageTolerance()` and selector `0x3db0d7e5`
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
        name = "TOKEN__ExceedsSwapSlippageTolerance",
        abi = "TOKEN__ExceedsSwapSlippageTolerance()"
    )]
    pub struct TOKEN__ExceedsSwapSlippageTolerance;
    ///Custom Error type `TOKEN__InvalidDecimals` with signature `TOKEN__InvalidDecimals()` and selector `0x00778f2e`
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
    #[etherror(name = "TOKEN__InvalidDecimals", abi = "TOKEN__InvalidDecimals()")]
    pub struct TOKEN__InvalidDecimals;
    ///Custom Error type `TOKEN__InvalidZeroAddress` with signature `TOKEN__InvalidZeroAddress()` and selector `0xe859a7d3`
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
    #[etherror(name = "TOKEN__InvalidZeroAddress", abi = "TOKEN__InvalidZeroAddress()")]
    pub struct TOKEN__InvalidZeroAddress;
    ///Custom Error type `TOKEN__InvalidZeroInput` with signature `TOKEN__InvalidZeroInput()` and selector `0x88173801`
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
    #[etherror(name = "TOKEN__InvalidZeroInput", abi = "TOKEN__InvalidZeroInput()")]
    pub struct TOKEN__InvalidZeroInput;
    ///Custom Error type `TOKEN__SwapExpired` with signature `TOKEN__SwapExpired()` and selector `0x578b1288`
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
    #[etherror(name = "TOKEN__SwapExpired", abi = "TOKEN__SwapExpired()")]
    pub struct TOKEN__SwapExpired;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum WIGErrors {
        TOKEN__ExceedsBorrowCreditLimit(TOKEN__ExceedsBorrowCreditLimit),
        TOKEN__ExceedsSwapMarketReserves(TOKEN__ExceedsSwapMarketReserves),
        TOKEN__ExceedsSwapSlippageTolerance(TOKEN__ExceedsSwapSlippageTolerance),
        TOKEN__InvalidDecimals(TOKEN__InvalidDecimals),
        TOKEN__InvalidZeroAddress(TOKEN__InvalidZeroAddress),
        TOKEN__InvalidZeroInput(TOKEN__InvalidZeroInput),
        TOKEN__SwapExpired(TOKEN__SwapExpired),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for WIGErrors {
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
                = <TOKEN__ExceedsBorrowCreditLimit as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TOKEN__ExceedsBorrowCreditLimit(decoded));
            }
            if let Ok(decoded)
                = <TOKEN__ExceedsSwapMarketReserves as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TOKEN__ExceedsSwapMarketReserves(decoded));
            }
            if let Ok(decoded)
                = <TOKEN__ExceedsSwapSlippageTolerance as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TOKEN__ExceedsSwapSlippageTolerance(decoded));
            }
            if let Ok(decoded)
                = <TOKEN__InvalidDecimals as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TOKEN__InvalidDecimals(decoded));
            }
            if let Ok(decoded)
                = <TOKEN__InvalidZeroAddress as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TOKEN__InvalidZeroAddress(decoded));
            }
            if let Ok(decoded)
                = <TOKEN__InvalidZeroInput as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TOKEN__InvalidZeroInput(decoded));
            }
            if let Ok(decoded)
                = <TOKEN__SwapExpired as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TOKEN__SwapExpired(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for WIGErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::TOKEN__ExceedsBorrowCreditLimit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TOKEN__ExceedsSwapMarketReserves(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TOKEN__ExceedsSwapSlippageTolerance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TOKEN__InvalidDecimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TOKEN__InvalidZeroAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TOKEN__InvalidZeroInput(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TOKEN__SwapExpired(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for WIGErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <TOKEN__ExceedsBorrowCreditLimit as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TOKEN__ExceedsSwapMarketReserves as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TOKEN__ExceedsSwapSlippageTolerance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TOKEN__InvalidDecimals as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TOKEN__InvalidZeroAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TOKEN__InvalidZeroInput as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TOKEN__SwapExpired as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for WIGErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::TOKEN__ExceedsBorrowCreditLimit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TOKEN__ExceedsSwapMarketReserves(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TOKEN__ExceedsSwapSlippageTolerance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TOKEN__InvalidDecimals(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TOKEN__InvalidZeroAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TOKEN__InvalidZeroInput(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TOKEN__SwapExpired(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for WIGErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<TOKEN__ExceedsBorrowCreditLimit> for WIGErrors {
        fn from(value: TOKEN__ExceedsBorrowCreditLimit) -> Self {
            Self::TOKEN__ExceedsBorrowCreditLimit(value)
        }
    }
    impl ::core::convert::From<TOKEN__ExceedsSwapMarketReserves> for WIGErrors {
        fn from(value: TOKEN__ExceedsSwapMarketReserves) -> Self {
            Self::TOKEN__ExceedsSwapMarketReserves(value)
        }
    }
    impl ::core::convert::From<TOKEN__ExceedsSwapSlippageTolerance> for WIGErrors {
        fn from(value: TOKEN__ExceedsSwapSlippageTolerance) -> Self {
            Self::TOKEN__ExceedsSwapSlippageTolerance(value)
        }
    }
    impl ::core::convert::From<TOKEN__InvalidDecimals> for WIGErrors {
        fn from(value: TOKEN__InvalidDecimals) -> Self {
            Self::TOKEN__InvalidDecimals(value)
        }
    }
    impl ::core::convert::From<TOKEN__InvalidZeroAddress> for WIGErrors {
        fn from(value: TOKEN__InvalidZeroAddress) -> Self {
            Self::TOKEN__InvalidZeroAddress(value)
        }
    }
    impl ::core::convert::From<TOKEN__InvalidZeroInput> for WIGErrors {
        fn from(value: TOKEN__InvalidZeroInput) -> Self {
            Self::TOKEN__InvalidZeroInput(value)
        }
    }
    impl ::core::convert::From<TOKEN__SwapExpired> for WIGErrors {
        fn from(value: TOKEN__SwapExpired) -> Self {
            Self::TOKEN__SwapExpired(value)
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
    #[ethevent(name = "TOKEN__Borrow", abi = "TOKEN__Borrow(address,uint256)")]
    pub struct TokenBorrowFilter {
        #[ethevent(indexed)]
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
    #[ethevent(name = "TOKEN__Buy", abi = "TOKEN__Buy(address,address,uint256)")]
    pub struct TokenBuyFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
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
        name = "TOKEN__Exercise",
        abi = "TOKEN__Exercise(address,address,uint256)"
    )]
    pub struct TokenExerciseFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
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
    #[ethevent(name = "TOKEN__Redeem", abi = "TOKEN__Redeem(address,address,uint256)")]
    pub struct TokenRedeemFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
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
    #[ethevent(name = "TOKEN__Repay", abi = "TOKEN__Repay(address,uint256)")]
    pub struct TokenRepayFilter {
        #[ethevent(indexed)]
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
    #[ethevent(name = "TOKEN__Sell", abi = "TOKEN__Sell(address,address,uint256)")]
    pub struct TokenSellFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
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
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum WIGEvents {
        ApprovalFilter(ApprovalFilter),
        TokenBorrowFilter(TokenBorrowFilter),
        TokenBuyFilter(TokenBuyFilter),
        TokenExerciseFilter(TokenExerciseFilter),
        TokenRedeemFilter(TokenRedeemFilter),
        TokenRepayFilter(TokenRepayFilter),
        TokenSellFilter(TokenSellFilter),
        TransferFilter(TransferFilter),
    }
    impl ::ethers::contract::EthLogDecode for WIGEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(WIGEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = TokenBorrowFilter::decode_log(log) {
                return Ok(WIGEvents::TokenBorrowFilter(decoded));
            }
            if let Ok(decoded) = TokenBuyFilter::decode_log(log) {
                return Ok(WIGEvents::TokenBuyFilter(decoded));
            }
            if let Ok(decoded) = TokenExerciseFilter::decode_log(log) {
                return Ok(WIGEvents::TokenExerciseFilter(decoded));
            }
            if let Ok(decoded) = TokenRedeemFilter::decode_log(log) {
                return Ok(WIGEvents::TokenRedeemFilter(decoded));
            }
            if let Ok(decoded) = TokenRepayFilter::decode_log(log) {
                return Ok(WIGEvents::TokenRepayFilter(decoded));
            }
            if let Ok(decoded) = TokenSellFilter::decode_log(log) {
                return Ok(WIGEvents::TokenSellFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(WIGEvents::TransferFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for WIGEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenBorrowFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenBuyFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenExerciseFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TokenRedeemFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenRepayFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenSellFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for WIGEvents {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<TokenBorrowFilter> for WIGEvents {
        fn from(value: TokenBorrowFilter) -> Self {
            Self::TokenBorrowFilter(value)
        }
    }
    impl ::core::convert::From<TokenBuyFilter> for WIGEvents {
        fn from(value: TokenBuyFilter) -> Self {
            Self::TokenBuyFilter(value)
        }
    }
    impl ::core::convert::From<TokenExerciseFilter> for WIGEvents {
        fn from(value: TokenExerciseFilter) -> Self {
            Self::TokenExerciseFilter(value)
        }
    }
    impl ::core::convert::From<TokenRedeemFilter> for WIGEvents {
        fn from(value: TokenRedeemFilter) -> Self {
            Self::TokenRedeemFilter(value)
        }
    }
    impl ::core::convert::From<TokenRepayFilter> for WIGEvents {
        fn from(value: TokenRepayFilter) -> Self {
            Self::TokenRepayFilter(value)
        }
    }
    impl ::core::convert::From<TokenSellFilter> for WIGEvents {
        fn from(value: TokenSellFilter) -> Self {
            Self::TokenSellFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for WIGEvents {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    ///Container type for all input parameters for the `BASE` function with signature `BASE()` and selector `0xec342ad0`
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
    #[ethcall(name = "BASE", abi = "BASE()")]
    pub struct BaseCall;
    ///Container type for all input parameters for the `DECIMALS` function with signature `DECIMALS()` and selector `0x2e0f2625`
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
    #[ethcall(name = "DECIMALS", abi = "DECIMALS()")]
    pub struct DECIMALSCall;
    ///Container type for all input parameters for the `DIVISOR` function with signature `DIVISOR()` and selector `0x3410fe6e`
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
    #[ethcall(name = "DIVISOR", abi = "DIVISOR()")]
    pub struct DivisorCall;
    ///Container type for all input parameters for the `FEES` function with signature `FEES()` and selector `0x8b7b23ee`
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
    #[ethcall(name = "FEES", abi = "FEES()")]
    pub struct FeesCall;
    ///Container type for all input parameters for the `FLOOR_PRICE` function with signature `FLOOR_PRICE()` and selector `0xa71f48a4`
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
    #[ethcall(name = "FLOOR_PRICE", abi = "FLOOR_PRICE()")]
    pub struct FloorPriceCall;
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
    ///Container type for all input parameters for the `PRECISION` function with signature `PRECISION()` and selector `0xaaf5eb68`
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
    #[ethcall(name = "PRECISION", abi = "PRECISION()")]
    pub struct PrecisionCall;
    ///Container type for all input parameters for the `PROTOCOL_FEE` function with signature `PROTOCOL_FEE()` and selector `0x0b4501fd`
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
    #[ethcall(name = "PROTOCOL_FEE", abi = "PROTOCOL_FEE()")]
    pub struct ProtocolFeeCall;
    ///Container type for all input parameters for the `PROVIDER_FEE` function with signature `PROVIDER_FEE()` and selector `0xf623bb8b`
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
    #[ethcall(name = "PROVIDER_FEE", abi = "PROVIDER_FEE()")]
    pub struct ProviderFeeCall;
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
    pub struct AllowanceCall {
        pub owner: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
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
        pub amount: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `borrow` function with signature `borrow(uint256)` and selector `0xc5ebeaec`
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
    #[ethcall(name = "borrow", abi = "borrow(uint256)")]
    pub struct BorrowCall {
        pub amount_base: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `buy` function with signature `buy(uint256,uint256,uint256,address,address)` and selector `0xba002b70`
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
    #[ethcall(name = "buy", abi = "buy(uint256,uint256,uint256,address,address)")]
    pub struct BuyCall {
        pub amount_base: ::ethers::core::types::U256,
        pub min_token: ::ethers::core::types::U256,
        pub expire_timestamp: ::ethers::core::types::U256,
        pub to_account: ::ethers::core::types::Address,
        pub provider: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `debtTotal` function with signature `debtTotal()` and selector `0xe7e7e387`
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
    #[ethcall(name = "debtTotal", abi = "debtTotal()")]
    pub struct DebtTotalCall;
    ///Container type for all input parameters for the `debts` function with signature `debts(address)` and selector `0x2ecd4e7d`
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
    #[ethcall(name = "debts", abi = "debts(address)")]
    pub struct DebtsCall(pub ::ethers::core::types::Address);
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
    pub struct decimalsCall;
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
    ///Container type for all input parameters for the `exercise` function with signature `exercise(uint256,address)` and selector `0x1b6b7298`
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
    #[ethcall(name = "exercise", abi = "exercise(uint256,address)")]
    pub struct ExerciseCall {
        pub amount_o_token: ::ethers::core::types::U256,
        pub to_account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `frBASE` function with signature `frBASE()` and selector `0xad6cb823`
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
    #[ethcall(name = "frBASE", abi = "frBASE()")]
    pub struct FrBASECall;
    ///Container type for all input parameters for the `getAccountCredit` function with signature `getAccountCredit(address)` and selector `0x4c7ebd37`
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
    #[ethcall(name = "getAccountCredit", abi = "getAccountCredit(address)")]
    pub struct GetAccountCreditCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getFloorPrice` function with signature `getFloorPrice()` and selector `0x2a33d6b2`
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
    #[ethcall(name = "getFloorPrice", abi = "getFloorPrice()")]
    pub struct GetFloorPriceCall;
    ///Container type for all input parameters for the `getMarketPrice` function with signature `getMarketPrice()` and selector `0x660e16c3`
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
    #[ethcall(name = "getMarketPrice", abi = "getMarketPrice()")]
    pub struct GetMarketPriceCall;
    ///Container type for all input parameters for the `getMaxSell` function with signature `getMaxSell()` and selector `0x466f6311`
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
    #[ethcall(name = "getMaxSell", abi = "getMaxSell()")]
    pub struct GetMaxSellCall;
    ///Container type for all input parameters for the `getOTokenPrice` function with signature `getOTokenPrice()` and selector `0x006c72af`
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
    #[ethcall(name = "getOTokenPrice", abi = "getOTokenPrice()")]
    pub struct GetOTokenPriceCall;
    ///Container type for all input parameters for the `getTotalValueLocked` function with signature `getTotalValueLocked()` and selector `0xb26025aa`
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
    #[ethcall(name = "getTotalValueLocked", abi = "getTotalValueLocked()")]
    pub struct GetTotalValueLockedCall;
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
    ///Container type for all input parameters for the `mrrBASE` function with signature `mrrBASE()` and selector `0xa0787856`
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
    #[ethcall(name = "mrrBASE", abi = "mrrBASE()")]
    pub struct MrrBASECall;
    ///Container type for all input parameters for the `mrrTOKEN` function with signature `mrrTOKEN()` and selector `0x1b8f0bbc`
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
    #[ethcall(name = "mrrTOKEN", abi = "mrrTOKEN()")]
    pub struct MrrTOKENCall;
    ///Container type for all input parameters for the `mrvBASE` function with signature `mrvBASE()` and selector `0xc2b96e9b`
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
    #[ethcall(name = "mrvBASE", abi = "mrvBASE()")]
    pub struct MrvBASECall;
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
    ///Container type for all input parameters for the `redeem` function with signature `redeem(uint256,address)` and selector `0x7bde82f2`
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
    #[ethcall(name = "redeem", abi = "redeem(uint256,address)")]
    pub struct RedeemCall {
        pub amount_token: ::ethers::core::types::U256,
        pub to_account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `repay` function with signature `repay(uint256)` and selector `0x371fd8e6`
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
    #[ethcall(name = "repay", abi = "repay(uint256)")]
    pub struct RepayCall {
        pub amount_base: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `sell` function with signature `sell(uint256,uint256,uint256,address,address)` and selector `0x26124288`
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
    #[ethcall(name = "sell", abi = "sell(uint256,uint256,uint256,address,address)")]
    pub struct SellCall {
        pub amount_token: ::ethers::core::types::U256,
        pub min_base: ::ethers::core::types::U256,
        pub expire_timestamp: ::ethers::core::types::U256,
        pub to_account: ::ethers::core::types::Address,
        pub provider: ::ethers::core::types::Address,
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
        pub amount: ::ethers::core::types::U256,
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
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum WIGCalls {
        Base(BaseCall),
        DECIMALS(DECIMALSCall),
        Divisor(DivisorCall),
        Fees(FeesCall),
        FloorPrice(FloorPriceCall),
        Otoken(OtokenCall),
        Precision(PrecisionCall),
        ProtocolFee(ProtocolFeeCall),
        ProviderFee(ProviderFeeCall),
        Vtoken(VtokenCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Borrow(BorrowCall),
        Buy(BuyCall),
        DebtTotal(DebtTotalCall),
        Debts(DebtsCall),
        decimals(decimalsCall),
        DecreaseAllowance(DecreaseAllowanceCall),
        Exercise(ExerciseCall),
        FrBASE(FrBASECall),
        GetAccountCredit(GetAccountCreditCall),
        GetFloorPrice(GetFloorPriceCall),
        GetMarketPrice(GetMarketPriceCall),
        GetMaxSell(GetMaxSellCall),
        GetOTokenPrice(GetOTokenPriceCall),
        GetTotalValueLocked(GetTotalValueLockedCall),
        IncreaseAllowance(IncreaseAllowanceCall),
        MrrBASE(MrrBASECall),
        MrrTOKEN(MrrTOKENCall),
        MrvBASE(MrvBASECall),
        Name(NameCall),
        Redeem(RedeemCall),
        Repay(RepayCall),
        Sell(SellCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
    }
    impl ::ethers::core::abi::AbiDecode for WIGCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <BaseCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Base(decoded));
            }
            if let Ok(decoded)
                = <DECIMALSCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DECIMALS(decoded));
            }
            if let Ok(decoded)
                = <DivisorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Divisor(decoded));
            }
            if let Ok(decoded)
                = <FeesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Fees(decoded));
            }
            if let Ok(decoded)
                = <FloorPriceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FloorPrice(decoded));
            }
            if let Ok(decoded)
                = <OtokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Otoken(decoded));
            }
            if let Ok(decoded)
                = <PrecisionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Precision(decoded));
            }
            if let Ok(decoded)
                = <ProtocolFeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ProtocolFee(decoded));
            }
            if let Ok(decoded)
                = <ProviderFeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ProviderFee(decoded));
            }
            if let Ok(decoded)
                = <VtokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Vtoken(decoded));
            }
            if let Ok(decoded)
                = <AllowanceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Allowance(decoded));
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
                = <BorrowCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Borrow(decoded));
            }
            if let Ok(decoded)
                = <BuyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Buy(decoded));
            }
            if let Ok(decoded)
                = <DebtTotalCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DebtTotal(decoded));
            }
            if let Ok(decoded)
                = <DebtsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Debts(decoded));
            }
            if let Ok(decoded)
                = <decimalsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::decimals(decoded));
            }
            if let Ok(decoded)
                = <DecreaseAllowanceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DecreaseAllowance(decoded));
            }
            if let Ok(decoded)
                = <ExerciseCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Exercise(decoded));
            }
            if let Ok(decoded)
                = <FrBASECall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FrBASE(decoded));
            }
            if let Ok(decoded)
                = <GetAccountCreditCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetAccountCredit(decoded));
            }
            if let Ok(decoded)
                = <GetFloorPriceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetFloorPrice(decoded));
            }
            if let Ok(decoded)
                = <GetMarketPriceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetMarketPrice(decoded));
            }
            if let Ok(decoded)
                = <GetMaxSellCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetMaxSell(decoded));
            }
            if let Ok(decoded)
                = <GetOTokenPriceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetOTokenPrice(decoded));
            }
            if let Ok(decoded)
                = <GetTotalValueLockedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetTotalValueLocked(decoded));
            }
            if let Ok(decoded)
                = <IncreaseAllowanceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IncreaseAllowance(decoded));
            }
            if let Ok(decoded)
                = <MrrBASECall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MrrBASE(decoded));
            }
            if let Ok(decoded)
                = <MrrTOKENCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MrrTOKEN(decoded));
            }
            if let Ok(decoded)
                = <MrvBASECall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MrvBASE(decoded));
            }
            if let Ok(decoded)
                = <NameCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded)
                = <RedeemCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Redeem(decoded));
            }
            if let Ok(decoded)
                = <RepayCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Repay(decoded));
            }
            if let Ok(decoded)
                = <SellCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Sell(decoded));
            }
            if let Ok(decoded)
                = <SymbolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Symbol(decoded));
            }
            if let Ok(decoded)
                = <TotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TotalSupply(decoded));
            }
            if let Ok(decoded)
                = <TransferCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Transfer(decoded));
            }
            if let Ok(decoded)
                = <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TransferFrom(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for WIGCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Base(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DECIMALS(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Divisor(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Fees(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FloorPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Otoken(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Precision(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProtocolFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProviderFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Vtoken(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Allowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Borrow(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Buy(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DebtTotal(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Debts(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::decimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DecreaseAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Exercise(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FrBASE(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetAccountCredit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetFloorPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetMarketPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetMaxSell(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOTokenPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTotalValueLocked(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IncreaseAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MrrBASE(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MrrTOKEN(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MrvBASE(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Redeem(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Repay(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Sell(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Symbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Transfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for WIGCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Base(element) => ::core::fmt::Display::fmt(element, f),
                Self::DECIMALS(element) => ::core::fmt::Display::fmt(element, f),
                Self::Divisor(element) => ::core::fmt::Display::fmt(element, f),
                Self::Fees(element) => ::core::fmt::Display::fmt(element, f),
                Self::FloorPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::Otoken(element) => ::core::fmt::Display::fmt(element, f),
                Self::Precision(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProtocolFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProviderFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::Vtoken(element) => ::core::fmt::Display::fmt(element, f),
                Self::Allowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Borrow(element) => ::core::fmt::Display::fmt(element, f),
                Self::Buy(element) => ::core::fmt::Display::fmt(element, f),
                Self::DebtTotal(element) => ::core::fmt::Display::fmt(element, f),
                Self::Debts(element) => ::core::fmt::Display::fmt(element, f),
                Self::decimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::DecreaseAllowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Exercise(element) => ::core::fmt::Display::fmt(element, f),
                Self::FrBASE(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAccountCredit(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetFloorPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMarketPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMaxSell(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOTokenPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTotalValueLocked(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IncreaseAllowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::MrrBASE(element) => ::core::fmt::Display::fmt(element, f),
                Self::MrrTOKEN(element) => ::core::fmt::Display::fmt(element, f),
                Self::MrvBASE(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Redeem(element) => ::core::fmt::Display::fmt(element, f),
                Self::Repay(element) => ::core::fmt::Display::fmt(element, f),
                Self::Sell(element) => ::core::fmt::Display::fmt(element, f),
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::Transfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BaseCall> for WIGCalls {
        fn from(value: BaseCall) -> Self {
            Self::Base(value)
        }
    }
    impl ::core::convert::From<DECIMALSCall> for WIGCalls {
        fn from(value: DECIMALSCall) -> Self {
            Self::DECIMALS(value)
        }
    }
    impl ::core::convert::From<DivisorCall> for WIGCalls {
        fn from(value: DivisorCall) -> Self {
            Self::Divisor(value)
        }
    }
    impl ::core::convert::From<FeesCall> for WIGCalls {
        fn from(value: FeesCall) -> Self {
            Self::Fees(value)
        }
    }
    impl ::core::convert::From<FloorPriceCall> for WIGCalls {
        fn from(value: FloorPriceCall) -> Self {
            Self::FloorPrice(value)
        }
    }
    impl ::core::convert::From<OtokenCall> for WIGCalls {
        fn from(value: OtokenCall) -> Self {
            Self::Otoken(value)
        }
    }
    impl ::core::convert::From<PrecisionCall> for WIGCalls {
        fn from(value: PrecisionCall) -> Self {
            Self::Precision(value)
        }
    }
    impl ::core::convert::From<ProtocolFeeCall> for WIGCalls {
        fn from(value: ProtocolFeeCall) -> Self {
            Self::ProtocolFee(value)
        }
    }
    impl ::core::convert::From<ProviderFeeCall> for WIGCalls {
        fn from(value: ProviderFeeCall) -> Self {
            Self::ProviderFee(value)
        }
    }
    impl ::core::convert::From<VtokenCall> for WIGCalls {
        fn from(value: VtokenCall) -> Self {
            Self::Vtoken(value)
        }
    }
    impl ::core::convert::From<AllowanceCall> for WIGCalls {
        fn from(value: AllowanceCall) -> Self {
            Self::Allowance(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for WIGCalls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for WIGCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<BorrowCall> for WIGCalls {
        fn from(value: BorrowCall) -> Self {
            Self::Borrow(value)
        }
    }
    impl ::core::convert::From<BuyCall> for WIGCalls {
        fn from(value: BuyCall) -> Self {
            Self::Buy(value)
        }
    }
    impl ::core::convert::From<DebtTotalCall> for WIGCalls {
        fn from(value: DebtTotalCall) -> Self {
            Self::DebtTotal(value)
        }
    }
    impl ::core::convert::From<DebtsCall> for WIGCalls {
        fn from(value: DebtsCall) -> Self {
            Self::Debts(value)
        }
    }
    impl ::core::convert::From<decimalsCall> for WIGCalls {
        fn from(value: decimalsCall) -> Self {
            Self::decimals(value)
        }
    }
    impl ::core::convert::From<DecreaseAllowanceCall> for WIGCalls {
        fn from(value: DecreaseAllowanceCall) -> Self {
            Self::DecreaseAllowance(value)
        }
    }
    impl ::core::convert::From<ExerciseCall> for WIGCalls {
        fn from(value: ExerciseCall) -> Self {
            Self::Exercise(value)
        }
    }
    impl ::core::convert::From<FrBASECall> for WIGCalls {
        fn from(value: FrBASECall) -> Self {
            Self::FrBASE(value)
        }
    }
    impl ::core::convert::From<GetAccountCreditCall> for WIGCalls {
        fn from(value: GetAccountCreditCall) -> Self {
            Self::GetAccountCredit(value)
        }
    }
    impl ::core::convert::From<GetFloorPriceCall> for WIGCalls {
        fn from(value: GetFloorPriceCall) -> Self {
            Self::GetFloorPrice(value)
        }
    }
    impl ::core::convert::From<GetMarketPriceCall> for WIGCalls {
        fn from(value: GetMarketPriceCall) -> Self {
            Self::GetMarketPrice(value)
        }
    }
    impl ::core::convert::From<GetMaxSellCall> for WIGCalls {
        fn from(value: GetMaxSellCall) -> Self {
            Self::GetMaxSell(value)
        }
    }
    impl ::core::convert::From<GetOTokenPriceCall> for WIGCalls {
        fn from(value: GetOTokenPriceCall) -> Self {
            Self::GetOTokenPrice(value)
        }
    }
    impl ::core::convert::From<GetTotalValueLockedCall> for WIGCalls {
        fn from(value: GetTotalValueLockedCall) -> Self {
            Self::GetTotalValueLocked(value)
        }
    }
    impl ::core::convert::From<IncreaseAllowanceCall> for WIGCalls {
        fn from(value: IncreaseAllowanceCall) -> Self {
            Self::IncreaseAllowance(value)
        }
    }
    impl ::core::convert::From<MrrBASECall> for WIGCalls {
        fn from(value: MrrBASECall) -> Self {
            Self::MrrBASE(value)
        }
    }
    impl ::core::convert::From<MrrTOKENCall> for WIGCalls {
        fn from(value: MrrTOKENCall) -> Self {
            Self::MrrTOKEN(value)
        }
    }
    impl ::core::convert::From<MrvBASECall> for WIGCalls {
        fn from(value: MrvBASECall) -> Self {
            Self::MrvBASE(value)
        }
    }
    impl ::core::convert::From<NameCall> for WIGCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<RedeemCall> for WIGCalls {
        fn from(value: RedeemCall) -> Self {
            Self::Redeem(value)
        }
    }
    impl ::core::convert::From<RepayCall> for WIGCalls {
        fn from(value: RepayCall) -> Self {
            Self::Repay(value)
        }
    }
    impl ::core::convert::From<SellCall> for WIGCalls {
        fn from(value: SellCall) -> Self {
            Self::Sell(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for WIGCalls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for WIGCalls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<TransferCall> for WIGCalls {
        fn from(value: TransferCall) -> Self {
            Self::Transfer(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for WIGCalls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    ///Container type for all return fields from the `BASE` function with signature `BASE()` and selector `0xec342ad0`
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
    pub struct BaseReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `DECIMALS` function with signature `DECIMALS()` and selector `0x2e0f2625`
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
    pub struct DECIMALSReturn(pub u8);
    ///Container type for all return fields from the `DIVISOR` function with signature `DIVISOR()` and selector `0x3410fe6e`
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
    pub struct DivisorReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `FEES` function with signature `FEES()` and selector `0x8b7b23ee`
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
    pub struct FeesReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `FLOOR_PRICE` function with signature `FLOOR_PRICE()` and selector `0xa71f48a4`
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
    pub struct FloorPriceReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `PRECISION` function with signature `PRECISION()` and selector `0xaaf5eb68`
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
    pub struct PrecisionReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `PROTOCOL_FEE` function with signature `PROTOCOL_FEE()` and selector `0x0b4501fd`
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
    pub struct ProtocolFeeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `PROVIDER_FEE` function with signature `PROVIDER_FEE()` and selector `0xf623bb8b`
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
    pub struct ProviderFeeReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `borrow` function with signature `borrow(uint256)` and selector `0xc5ebeaec`
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
    pub struct BorrowReturn(pub bool);
    ///Container type for all return fields from the `buy` function with signature `buy(uint256,uint256,uint256,address,address)` and selector `0xba002b70`
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
    pub struct BuyReturn(pub bool);
    ///Container type for all return fields from the `debtTotal` function with signature `debtTotal()` and selector `0xe7e7e387`
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
    pub struct DebtTotalReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `debts` function with signature `debts(address)` and selector `0x2ecd4e7d`
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
    pub struct DebtsReturn(pub ::ethers::core::types::U256);
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
    pub struct decimalsReturn(pub u8);
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
    ///Container type for all return fields from the `exercise` function with signature `exercise(uint256,address)` and selector `0x1b6b7298`
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
    pub struct ExerciseReturn(pub bool);
    ///Container type for all return fields from the `frBASE` function with signature `frBASE()` and selector `0xad6cb823`
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
    pub struct FrBASEReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getAccountCredit` function with signature `getAccountCredit(address)` and selector `0x4c7ebd37`
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
    pub struct GetAccountCreditReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getFloorPrice` function with signature `getFloorPrice()` and selector `0x2a33d6b2`
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
    pub struct GetFloorPriceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getMarketPrice` function with signature `getMarketPrice()` and selector `0x660e16c3`
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
    pub struct GetMarketPriceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getMaxSell` function with signature `getMaxSell()` and selector `0x466f6311`
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
    pub struct GetMaxSellReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getOTokenPrice` function with signature `getOTokenPrice()` and selector `0x006c72af`
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
    pub struct GetOTokenPriceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getTotalValueLocked` function with signature `getTotalValueLocked()` and selector `0xb26025aa`
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
    pub struct GetTotalValueLockedReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `mrrBASE` function with signature `mrrBASE()` and selector `0xa0787856`
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
    pub struct MrrBASEReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `mrrTOKEN` function with signature `mrrTOKEN()` and selector `0x1b8f0bbc`
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
    pub struct MrrTOKENReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `mrvBASE` function with signature `mrvBASE()` and selector `0xc2b96e9b`
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
    pub struct MrvBASEReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `redeem` function with signature `redeem(uint256,address)` and selector `0x7bde82f2`
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
    pub struct RedeemReturn(pub bool);
    ///Container type for all return fields from the `repay` function with signature `repay(uint256)` and selector `0x371fd8e6`
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
    pub struct RepayReturn(pub bool);
    ///Container type for all return fields from the `sell` function with signature `sell(uint256,uint256,uint256,address,address)` and selector `0x26124288`
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
    pub struct SellReturn(pub bool);
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
}
