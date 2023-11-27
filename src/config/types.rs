use crate::config::addresses::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct ChainData {
    pub id: i32,
    pub name: String,
    pub geckoterminal_name: String,
    pub rpc_url: String,
    pub velocimeter_router_address: String,
    pub aerodrome_router_address: String,
    pub wig_voter_address: String,
    pub bvm_address: String,
    pub o_bvm_address: String,
    pub wig_address: String,
    pub o_wig_address: String,
    pub weth_address: String,
    pub wblt_address: String,
    pub usdc_address: String,
    pub multicall_address: String,
}

#[derive(Debug, Clone)]
pub enum Chain {
    Base(ChainData),
}

impl Chain {
    pub fn new(rpc_url: String, chain_id: i32) -> Self {
        match chain_id {
            8453 => Self::Base(ChainData {
                id: 8453,
                rpc_url,
                name: String::from("Base"),
                geckoterminal_name: String::from("base"),
                velocimeter_router_address: VELOCIMETER_ROUTER.to_string(),
                aerodrome_router_address: AERODROME_ROUTER.to_string(),
                wig_voter_address: WIG_VOTER.to_string(),
                bvm_address: BVM_TOKEN.to_string(),
                o_bvm_address: OBVM_TOKEN.to_string(),
                wig_address: WIG_TOKEN.to_string(),
                o_wig_address: OWIG_TOKEN.to_string(),
                weth_address: WETH_TOKEN.to_string(),
                wblt_address: WBLT_TOKEN.to_string(),
                usdc_address: USDC_TOKEN.to_string(),
                multicall_address: MULTICALL_ADDRESS.to_string(),
            }),
            _ => panic!("Chain id not supported"),
        }
    }

    pub fn get_chain_data(&self) -> &ChainData {
        match self {
            Chain::Base(data) => data,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Asset {
    pub address: String,
    pub decimals: i32,
    pub logoURI: String,
    pub name: String,
    pub symbol: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct AssetWithPrice {
    pub address: String,
    pub decimals: i32,
    pub logoURI: String,
    pub name: String,
    pub symbol: String,
    pub price: f64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum GeckoTerminalResponse {
    Success(GeckoTerminalSuccessResponse),
    Error(GeckoTerminalFailedResponse),
    RateLimited(GeckoTermninalRateLimited),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GeckoTerminalSuccessResponse {
    pub data: GeckoTerminalData,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GeckoTerminalFailedResponse {
    pub errors: Vec<GeckoterminalError>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GeckoTermninalRateLimited {
    pub status: String,
    pub title: String,
    pub limit: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GeckoTerminalData {
    id: String,
    pub attributes: GeckoTerminalAttributes,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GeckoterminalError {
    pub status: String,
    pub title: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GeckoTerminalAttributes {
    name: String,
    address: String,
    symbol: String,
    decimals: i32,
    total_supply: String,
    coingecko_coin_id: Option<String>,
    pub price_usd: Option<String>,
    fdv_usd: Option<String>,
    total_reserve_in_usd: String,
    volume_usd: serde_json::Value,
    market_cap_usd: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DexscreenerResponse {
    pub pairs: Vec<DexscreenerPair>,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct DexscreenerPair {
    pub chainId: String,
    pub dexId: String,
    pub url: String,
    pub pairAddress: String,
    pub baseToken: DexscreenerToken,
    pub quoteToken: DexscreenerToken,
    pub priceNative: String,
    pub priceUsd: Option<String>,
    pub txns: DexscreenerTxns,
    pub volume: DexscreenerVolume,
    pub priceChange: DexscreenerPriceChange,
    pub liquidity: Option<DexscreenerLiquidity>,
    pub fdv: Option<f64>,
    pub pairCreatedAt: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DexscreenerToken {
    pub address: String,
    pub name: String,
    pub symbol: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DexscreenerTxns {
    pub m5: DexscreenerTxnsData,
    pub h1: DexscreenerTxnsData,
    pub h6: DexscreenerTxnsData,
    pub h24: DexscreenerTxnsData,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DexscreenerTxnsData {
    pub buys: i32,
    pub sells: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DexscreenerVolume {
    pub m5: f64,
    pub h1: f64,
    pub h6: f64,
    pub h24: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DexscreenerPriceChange {
    pub m5: f64,
    pub h1: f64,
    pub h6: f64,
    pub h24: f64,
}

#[derive(Debug, Deserialize, Serialize)]

pub struct DexscreenerLiquidity {
    pub usd: Option<f64>,
    pub base: f64,
    pub quote: f64,
}
