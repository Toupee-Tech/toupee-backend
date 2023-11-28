use serde::{Deserialize, Serialize};

use backend::database::{
    aprs::Model as AprsModel, bribes::Model as BribeModel, gauges::Model as GaugeModel,
    plugins::Model as PluginModel,
};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct PluginsResponse {
    pub plugins: Vec<PluginResponse>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginResponse {
    pub address: String,
    pub symbol: String,
    pub total_supply: f64,
    pub reserve0: f64,
    pub reserve1: f64,
    pub gauge_address: String,
    pub tvl: f64,
    pub token0_address: String,
    pub token1_address: String,
    pub token0: Value,
    pub token1: Value,
    pub gauge: Option<GaugeModel>,
    pub aprs: Option<Vec<AprsModel>>,
    pub bribes: Option<Vec<BribeModel>>,
}

impl PluginResponse {
    pub fn new(
        plugin: PluginModel,
        gauge: Option<GaugeModel>,
        aprs: Vec<AprsModel>,
        bribes: Vec<BribeModel>,
    ) -> Self {
        Self {
            address: plugin.address,
            symbol: plugin.symbol,
            total_supply: plugin.total_supply,
            reserve0: plugin.reserve0,
            reserve1: plugin.reserve1,
            gauge_address: plugin.gauge_address,
            tvl: plugin.tvl,
            token0_address: plugin.token0_address,
            token1_address: plugin.token1_address,
            token0: plugin.token0,
            token1: plugin.token1,
            gauge,
            aprs: aprs.len().gt(&0).then_some(aprs),
            bribes: bribes.len().gt(&0).then_some(bribes),
        }
    }
}
