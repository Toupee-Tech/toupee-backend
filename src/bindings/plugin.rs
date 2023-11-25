pub use plugin::*;
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
pub mod plugin {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_underlying"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_aeroGauge"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_voter"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_tokensInUnderlying"),
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
                        name: ::std::borrow::ToOwned::to_owned("_bribeTokens"),
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
                        name: ::std::borrow::ToOwned::to_owned("_protocol"),
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
                    ::std::borrow::ToOwned::to_owned("aeroGauge"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("aeroGauge"),
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
                    ::std::borrow::ToOwned::to_owned("claimAndDistribute"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("claimAndDistribute"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("depositFor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("depositFor"),
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
                    ::std::borrow::ToOwned::to_owned("getBribe"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getBribe"),
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
                    ::std::borrow::ToOwned::to_owned("getBribeTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getBribeTokens"),
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
                    ::std::borrow::ToOwned::to_owned("getGauge"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getGauge"),
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
                    ::std::borrow::ToOwned::to_owned("getProtocol"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getProtocol"),
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
                    ::std::borrow::ToOwned::to_owned("getTokensInUnderlying"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getTokensInUnderlying",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("getUnderlyingAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getUnderlyingAddress",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("getUnderlyingDecimals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getUnderlyingDecimals",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("getUnderlyingName"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getUnderlyingName"),
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
                    ::std::borrow::ToOwned::to_owned("getUnderlyingSymbol"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getUnderlyingSymbol",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("getVoter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getVoter"),
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
                    ::std::borrow::ToOwned::to_owned("setBribe"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setBribe"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_bribe"),
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
                    ::std::borrow::ToOwned::to_owned("setGauge"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setGauge"),
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
                    ::std::borrow::ToOwned::to_owned("withdrawTo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdrawTo"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Plugin__ClaimedAnDistributed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Plugin__ClaimedAnDistributed",
                            ),
                            inputs: ::std::vec![],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Plugin__Deposited"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Plugin__Deposited"),
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
                    ::std::borrow::ToOwned::to_owned("Plugin__Withdrawn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Plugin__Withdrawn"),
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
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Plugin__InvalidZeroInput"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Plugin__InvalidZeroInput",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Plugin__NotAuthorizedVoter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Plugin__NotAuthorizedVoter",
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
    pub static PLUGIN_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct Plugin<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Plugin<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Plugin<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Plugin<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Plugin<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Plugin)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Plugin<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    PLUGIN_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `aeroGauge` (0x8e8f7953) function
        pub fn aero_gauge(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([142, 143, 121, 83], ())
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
        ///Calls the contract's `claimAndDistribute` (0xdfb89c37) function
        pub fn claim_and_distribute(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([223, 184, 156, 55], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `depositFor` (0x2f4f21e2) function
        pub fn deposit_for(
            &self,
            account: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 79, 33, 226], (account, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBribe` (0x8df972be) function
        pub fn get_bribe(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 249, 114, 190], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBribeTokens` (0xe33ca839) function
        pub fn get_bribe_tokens(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([227, 60, 168, 57], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getGauge` (0xf2803f03) function
        pub fn get_gauge(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([242, 128, 63, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getProtocol` (0xd16352af) function
        pub fn get_protocol(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([209, 99, 82, 175], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTokensInUnderlying` (0xb1942b16) function
        pub fn get_tokens_in_underlying(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([177, 148, 43, 22], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getUnderlyingAddress` (0x88a68682) function
        pub fn get_underlying_address(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([136, 166, 134, 130], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getUnderlyingDecimals` (0x92081a47) function
        pub fn get_underlying_decimals(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([146, 8, 26, 71], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getUnderlyingName` (0xdf6b91b1) function
        pub fn get_underlying_name(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([223, 107, 145, 177], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getUnderlyingSymbol` (0x068acc39) function
        pub fn get_underlying_symbol(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 138, 204, 57], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getVoter` (0x2b37f53c) function
        pub fn get_voter(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([43, 55, 245, 60], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setBribe` (0xe81defce) function
        pub fn set_bribe(
            &self,
            bribe: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([232, 29, 239, 206], bribe)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setGauge` (0x55a68ed3) function
        pub fn set_gauge(
            &self,
            gauge: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([85, 166, 142, 211], gauge)
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
        ///Calls the contract's `withdrawTo` (0x205c2878) function
        pub fn withdraw_to(
            &self,
            account: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([32, 92, 40, 120], (account, amount))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Plugin__ClaimedAnDistributed` event
        pub fn plugin_claimed_an_distributed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PluginClaimedAnDistributedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Plugin__Deposited` event
        pub fn plugin_deposited_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PluginDepositedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Plugin__Withdrawn` event
        pub fn plugin_withdrawn_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PluginWithdrawnFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PluginEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Plugin<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `Plugin__InvalidZeroInput` with signature `Plugin__InvalidZeroInput()` and selector `0xf6ec1bed`
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
    #[etherror(name = "Plugin__InvalidZeroInput", abi = "Plugin__InvalidZeroInput()")]
    pub struct Plugin__InvalidZeroInput;
    ///Custom Error type `Plugin__NotAuthorizedVoter` with signature `Plugin__NotAuthorizedVoter()` and selector `0xcdb9c91a`
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
        name = "Plugin__NotAuthorizedVoter",
        abi = "Plugin__NotAuthorizedVoter()"
    )]
    pub struct Plugin__NotAuthorizedVoter;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PluginErrors {
        Plugin__InvalidZeroInput(Plugin__InvalidZeroInput),
        Plugin__NotAuthorizedVoter(Plugin__NotAuthorizedVoter),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for PluginErrors {
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
                = <Plugin__InvalidZeroInput as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::Plugin__InvalidZeroInput(decoded));
            }
            if let Ok(decoded)
                = <Plugin__NotAuthorizedVoter as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::Plugin__NotAuthorizedVoter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PluginErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::Plugin__InvalidZeroInput(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Plugin__NotAuthorizedVoter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for PluginErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <Plugin__InvalidZeroInput as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Plugin__NotAuthorizedVoter as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for PluginErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Plugin__InvalidZeroInput(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Plugin__NotAuthorizedVoter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for PluginErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<Plugin__InvalidZeroInput> for PluginErrors {
        fn from(value: Plugin__InvalidZeroInput) -> Self {
            Self::Plugin__InvalidZeroInput(value)
        }
    }
    impl ::core::convert::From<Plugin__NotAuthorizedVoter> for PluginErrors {
        fn from(value: Plugin__NotAuthorizedVoter) -> Self {
            Self::Plugin__NotAuthorizedVoter(value)
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
        name = "Plugin__ClaimedAnDistributed",
        abi = "Plugin__ClaimedAnDistributed()"
    )]
    pub struct PluginClaimedAnDistributedFilter;
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
    #[ethevent(name = "Plugin__Deposited", abi = "Plugin__Deposited(address,uint256)")]
    pub struct PluginDepositedFilter {
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
    #[ethevent(name = "Plugin__Withdrawn", abi = "Plugin__Withdrawn(address,uint256)")]
    pub struct PluginWithdrawnFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PluginEvents {
        PluginClaimedAnDistributedFilter(PluginClaimedAnDistributedFilter),
        PluginDepositedFilter(PluginDepositedFilter),
        PluginWithdrawnFilter(PluginWithdrawnFilter),
    }
    impl ::ethers::contract::EthLogDecode for PluginEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = PluginClaimedAnDistributedFilter::decode_log(log) {
                return Ok(PluginEvents::PluginClaimedAnDistributedFilter(decoded));
            }
            if let Ok(decoded) = PluginDepositedFilter::decode_log(log) {
                return Ok(PluginEvents::PluginDepositedFilter(decoded));
            }
            if let Ok(decoded) = PluginWithdrawnFilter::decode_log(log) {
                return Ok(PluginEvents::PluginWithdrawnFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for PluginEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::PluginClaimedAnDistributedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PluginDepositedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PluginWithdrawnFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<PluginClaimedAnDistributedFilter> for PluginEvents {
        fn from(value: PluginClaimedAnDistributedFilter) -> Self {
            Self::PluginClaimedAnDistributedFilter(value)
        }
    }
    impl ::core::convert::From<PluginDepositedFilter> for PluginEvents {
        fn from(value: PluginDepositedFilter) -> Self {
            Self::PluginDepositedFilter(value)
        }
    }
    impl ::core::convert::From<PluginWithdrawnFilter> for PluginEvents {
        fn from(value: PluginWithdrawnFilter) -> Self {
            Self::PluginWithdrawnFilter(value)
        }
    }
    ///Container type for all input parameters for the `aeroGauge` function with signature `aeroGauge()` and selector `0x8e8f7953`
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
    #[ethcall(name = "aeroGauge", abi = "aeroGauge()")]
    pub struct AeroGaugeCall;
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
    ///Container type for all input parameters for the `claimAndDistribute` function with signature `claimAndDistribute()` and selector `0xdfb89c37`
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
    #[ethcall(name = "claimAndDistribute", abi = "claimAndDistribute()")]
    pub struct ClaimAndDistributeCall;
    ///Container type for all input parameters for the `depositFor` function with signature `depositFor(address,uint256)` and selector `0x2f4f21e2`
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
    #[ethcall(name = "depositFor", abi = "depositFor(address,uint256)")]
    pub struct DepositForCall {
        pub account: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getBribe` function with signature `getBribe()` and selector `0x8df972be`
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
    #[ethcall(name = "getBribe", abi = "getBribe()")]
    pub struct GetBribeCall;
    ///Container type for all input parameters for the `getBribeTokens` function with signature `getBribeTokens()` and selector `0xe33ca839`
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
    #[ethcall(name = "getBribeTokens", abi = "getBribeTokens()")]
    pub struct GetBribeTokensCall;
    ///Container type for all input parameters for the `getGauge` function with signature `getGauge()` and selector `0xf2803f03`
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
    #[ethcall(name = "getGauge", abi = "getGauge()")]
    pub struct GetGaugeCall;
    ///Container type for all input parameters for the `getProtocol` function with signature `getProtocol()` and selector `0xd16352af`
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
    #[ethcall(name = "getProtocol", abi = "getProtocol()")]
    pub struct GetProtocolCall;
    ///Container type for all input parameters for the `getTokensInUnderlying` function with signature `getTokensInUnderlying()` and selector `0xb1942b16`
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
    #[ethcall(name = "getTokensInUnderlying", abi = "getTokensInUnderlying()")]
    pub struct GetTokensInUnderlyingCall;
    ///Container type for all input parameters for the `getUnderlyingAddress` function with signature `getUnderlyingAddress()` and selector `0x88a68682`
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
    #[ethcall(name = "getUnderlyingAddress", abi = "getUnderlyingAddress()")]
    pub struct GetUnderlyingAddressCall;
    ///Container type for all input parameters for the `getUnderlyingDecimals` function with signature `getUnderlyingDecimals()` and selector `0x92081a47`
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
    #[ethcall(name = "getUnderlyingDecimals", abi = "getUnderlyingDecimals()")]
    pub struct GetUnderlyingDecimalsCall;
    ///Container type for all input parameters for the `getUnderlyingName` function with signature `getUnderlyingName()` and selector `0xdf6b91b1`
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
    #[ethcall(name = "getUnderlyingName", abi = "getUnderlyingName()")]
    pub struct GetUnderlyingNameCall;
    ///Container type for all input parameters for the `getUnderlyingSymbol` function with signature `getUnderlyingSymbol()` and selector `0x068acc39`
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
    #[ethcall(name = "getUnderlyingSymbol", abi = "getUnderlyingSymbol()")]
    pub struct GetUnderlyingSymbolCall;
    ///Container type for all input parameters for the `getVoter` function with signature `getVoter()` and selector `0x2b37f53c`
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
    #[ethcall(name = "getVoter", abi = "getVoter()")]
    pub struct GetVoterCall;
    ///Container type for all input parameters for the `setBribe` function with signature `setBribe(address)` and selector `0xe81defce`
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
    #[ethcall(name = "setBribe", abi = "setBribe(address)")]
    pub struct SetBribeCall {
        pub bribe: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setGauge` function with signature `setGauge(address)` and selector `0x55a68ed3`
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
    #[ethcall(name = "setGauge", abi = "setGauge(address)")]
    pub struct SetGaugeCall {
        pub gauge: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `withdrawTo` function with signature `withdrawTo(address,uint256)` and selector `0x205c2878`
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
    #[ethcall(name = "withdrawTo", abi = "withdrawTo(address,uint256)")]
    pub struct WithdrawToCall {
        pub account: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PluginCalls {
        AeroGauge(AeroGaugeCall),
        BalanceOf(BalanceOfCall),
        ClaimAndDistribute(ClaimAndDistributeCall),
        DepositFor(DepositForCall),
        GetBribe(GetBribeCall),
        GetBribeTokens(GetBribeTokensCall),
        GetGauge(GetGaugeCall),
        GetProtocol(GetProtocolCall),
        GetTokensInUnderlying(GetTokensInUnderlyingCall),
        GetUnderlyingAddress(GetUnderlyingAddressCall),
        GetUnderlyingDecimals(GetUnderlyingDecimalsCall),
        GetUnderlyingName(GetUnderlyingNameCall),
        GetUnderlyingSymbol(GetUnderlyingSymbolCall),
        GetVoter(GetVoterCall),
        SetBribe(SetBribeCall),
        SetGauge(SetGaugeCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        WithdrawTo(WithdrawToCall),
    }
    impl ::ethers::core::abi::AbiDecode for PluginCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AeroGaugeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AeroGauge(decoded));
            }
            if let Ok(decoded)
                = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded)
                = <ClaimAndDistributeCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ClaimAndDistribute(decoded));
            }
            if let Ok(decoded)
                = <DepositForCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DepositFor(decoded));
            }
            if let Ok(decoded)
                = <GetBribeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetBribe(decoded));
            }
            if let Ok(decoded)
                = <GetBribeTokensCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetBribeTokens(decoded));
            }
            if let Ok(decoded)
                = <GetGaugeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetGauge(decoded));
            }
            if let Ok(decoded)
                = <GetProtocolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetProtocol(decoded));
            }
            if let Ok(decoded)
                = <GetTokensInUnderlyingCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetTokensInUnderlying(decoded));
            }
            if let Ok(decoded)
                = <GetUnderlyingAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetUnderlyingAddress(decoded));
            }
            if let Ok(decoded)
                = <GetUnderlyingDecimalsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetUnderlyingDecimals(decoded));
            }
            if let Ok(decoded)
                = <GetUnderlyingNameCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetUnderlyingName(decoded));
            }
            if let Ok(decoded)
                = <GetUnderlyingSymbolCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetUnderlyingSymbol(decoded));
            }
            if let Ok(decoded)
                = <GetVoterCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetVoter(decoded));
            }
            if let Ok(decoded)
                = <SetBribeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetBribe(decoded));
            }
            if let Ok(decoded)
                = <SetGaugeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetGauge(decoded));
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
                = <WithdrawToCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WithdrawTo(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PluginCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AeroGauge(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ClaimAndDistribute(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DepositFor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetBribe(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetBribeTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetGauge(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetProtocol(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTokensInUnderlying(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetUnderlyingAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetUnderlyingDecimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetUnderlyingName(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetUnderlyingSymbol(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetVoter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetBribe(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetGauge(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Symbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawTo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for PluginCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AeroGauge(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClaimAndDistribute(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DepositFor(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBribe(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBribeTokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetGauge(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetProtocol(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTokensInUnderlying(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetUnderlyingAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetUnderlyingDecimals(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetUnderlyingName(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetUnderlyingSymbol(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetVoter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetBribe(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetGauge(element) => ::core::fmt::Display::fmt(element, f),
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawTo(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AeroGaugeCall> for PluginCalls {
        fn from(value: AeroGaugeCall) -> Self {
            Self::AeroGauge(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for PluginCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<ClaimAndDistributeCall> for PluginCalls {
        fn from(value: ClaimAndDistributeCall) -> Self {
            Self::ClaimAndDistribute(value)
        }
    }
    impl ::core::convert::From<DepositForCall> for PluginCalls {
        fn from(value: DepositForCall) -> Self {
            Self::DepositFor(value)
        }
    }
    impl ::core::convert::From<GetBribeCall> for PluginCalls {
        fn from(value: GetBribeCall) -> Self {
            Self::GetBribe(value)
        }
    }
    impl ::core::convert::From<GetBribeTokensCall> for PluginCalls {
        fn from(value: GetBribeTokensCall) -> Self {
            Self::GetBribeTokens(value)
        }
    }
    impl ::core::convert::From<GetGaugeCall> for PluginCalls {
        fn from(value: GetGaugeCall) -> Self {
            Self::GetGauge(value)
        }
    }
    impl ::core::convert::From<GetProtocolCall> for PluginCalls {
        fn from(value: GetProtocolCall) -> Self {
            Self::GetProtocol(value)
        }
    }
    impl ::core::convert::From<GetTokensInUnderlyingCall> for PluginCalls {
        fn from(value: GetTokensInUnderlyingCall) -> Self {
            Self::GetTokensInUnderlying(value)
        }
    }
    impl ::core::convert::From<GetUnderlyingAddressCall> for PluginCalls {
        fn from(value: GetUnderlyingAddressCall) -> Self {
            Self::GetUnderlyingAddress(value)
        }
    }
    impl ::core::convert::From<GetUnderlyingDecimalsCall> for PluginCalls {
        fn from(value: GetUnderlyingDecimalsCall) -> Self {
            Self::GetUnderlyingDecimals(value)
        }
    }
    impl ::core::convert::From<GetUnderlyingNameCall> for PluginCalls {
        fn from(value: GetUnderlyingNameCall) -> Self {
            Self::GetUnderlyingName(value)
        }
    }
    impl ::core::convert::From<GetUnderlyingSymbolCall> for PluginCalls {
        fn from(value: GetUnderlyingSymbolCall) -> Self {
            Self::GetUnderlyingSymbol(value)
        }
    }
    impl ::core::convert::From<GetVoterCall> for PluginCalls {
        fn from(value: GetVoterCall) -> Self {
            Self::GetVoter(value)
        }
    }
    impl ::core::convert::From<SetBribeCall> for PluginCalls {
        fn from(value: SetBribeCall) -> Self {
            Self::SetBribe(value)
        }
    }
    impl ::core::convert::From<SetGaugeCall> for PluginCalls {
        fn from(value: SetGaugeCall) -> Self {
            Self::SetGauge(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for PluginCalls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for PluginCalls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<WithdrawToCall> for PluginCalls {
        fn from(value: WithdrawToCall) -> Self {
            Self::WithdrawTo(value)
        }
    }
    ///Container type for all return fields from the `aeroGauge` function with signature `aeroGauge()` and selector `0x8e8f7953`
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
    pub struct AeroGaugeReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `getBribe` function with signature `getBribe()` and selector `0x8df972be`
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
    pub struct GetBribeReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getBribeTokens` function with signature `getBribeTokens()` and selector `0xe33ca839`
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
    pub struct GetBribeTokensReturn(pub ::std::vec::Vec<::ethers::core::types::Address>);
    ///Container type for all return fields from the `getGauge` function with signature `getGauge()` and selector `0xf2803f03`
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
    pub struct GetGaugeReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getProtocol` function with signature `getProtocol()` and selector `0xd16352af`
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
    pub struct GetProtocolReturn(pub ::std::string::String);
    ///Container type for all return fields from the `getTokensInUnderlying` function with signature `getTokensInUnderlying()` and selector `0xb1942b16`
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
    pub struct GetTokensInUnderlyingReturn(
        pub ::std::vec::Vec<::ethers::core::types::Address>,
    );
    ///Container type for all return fields from the `getUnderlyingAddress` function with signature `getUnderlyingAddress()` and selector `0x88a68682`
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
    pub struct GetUnderlyingAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getUnderlyingDecimals` function with signature `getUnderlyingDecimals()` and selector `0x92081a47`
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
    pub struct GetUnderlyingDecimalsReturn(pub u8);
    ///Container type for all return fields from the `getUnderlyingName` function with signature `getUnderlyingName()` and selector `0xdf6b91b1`
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
    pub struct GetUnderlyingNameReturn(pub ::std::string::String);
    ///Container type for all return fields from the `getUnderlyingSymbol` function with signature `getUnderlyingSymbol()` and selector `0x068acc39`
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
    pub struct GetUnderlyingSymbolReturn(pub ::std::string::String);
    ///Container type for all return fields from the `getVoter` function with signature `getVoter()` and selector `0x2b37f53c`
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
    pub struct GetVoterReturn(pub ::ethers::core::types::Address);
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
}
