use axum::http::StatusCode;
use ethers::{
    abi::Address,
    contract::{abigen, Multicall},
    prelude::{Http, Provider},
    types::{H160, U256},
    utils::{format_ether, format_units, to_checksum},
};
use eyre::Result;
use sea_orm::{sea_query, ActiveValue, DatabaseConnection, EntityTrait};
use serde_json::json;
use std::sync::Arc;
use tracing::{error, info, instrument};

use backend::bindings::{AerodromeRouter, Pair, Plugin, VelocimeterRouter, Voter};
use backend::config::types::Chain;
use backend::database::plugins::{
    ActiveModel as ActivePlugin, Column as PluginsColumn, Entity as Plugins,
};

use crate::server::internal_error;
use crate::syncer::{assets::find_asset, gauges::update_gauge};

struct ReserveConfig {
    protocol: String,
    token_0_address: H160,
    token_1_address: H160,
    stable: bool,
    plugin_total_supply: U256,
    pair_address: H160,
}

#[instrument(skip_all)]
pub async fn update_plugins(chain: &Chain, conn: &Arc<DatabaseConnection>) -> Result<()> {
    info!("Reading plugins",);
    let provider = Provider::<Http>::try_from(chain.get_chain_data().rpc_url.to_string())?;
    let client = Arc::new(provider);

    let voter_address = chain
        .get_chain_data()
        .wig_voter_address
        .parse::<Address>()
        .expect("Set by hand");
    let voter = Voter::new(voter_address, Arc::clone(&client));

    let plugin_addresses = voter.get_plugins().call().await?;

    for plugin_address in plugin_addresses {
        match update_plugin(plugin_address, chain.clone(), Arc::clone(&client), &conn).await {
            Ok(_) => {}
            Err(e) => {
                info!("Error updating plugin {}: {:?}", plugin_address, e);
            }
        }
    }

    Ok(())
}

async fn update_plugin(
    plugin_address: H160,
    chain: Chain,
    client: Arc<Provider<Http>>,
    conn: &Arc<DatabaseConnection>,
) -> Result<()> {
    let plugin = Plugin::new(plugin_address, Arc::clone(&client));
    let pair_address = plugin.get_underlying_address().await?;
    let pair = Pair::new(pair_address, Arc::clone(&client));

    let mut multicall = Multicall::<Provider<Http>>::new(
        client.clone(),
        Some(
            chain
                .get_chain_data()
                .multicall_address
                .parse::<Address>()
                .expect("Address is set by hand"),
        ),
    )
    .await?;

    multicall.add_calls(
        false,
        [
            pair.token_0(),
            pair.token_1(),
            plugin.get_gauge(),
            plugin.get_bribe(),
        ],
    );
    multicall.add_call(plugin.total_supply(), false);
    multicall.add_call(pair.stable(), false);
    multicall.add_calls(false, [plugin.symbol(), plugin.get_protocol()]);

    let (
        token_0_address,
        token_1_address,
        gauge,
        bribe,
        plugin_total_supply,
        stable,
        symbol,
        protocol,
    ) = multicall
        .call::<(H160, H160, H160, H160, U256, bool, String, String)>()
        .await?;

    let token_0 = find_asset(to_checksum(&token_0_address, None), &chain, conn).await?;
    let token_1 = find_asset(to_checksum(&token_1_address, None), &chain, conn).await?;

    let reserve_config = ReserveConfig {
        protocol,
        token_0_address,
        token_1_address,
        stable,
        plugin_total_supply,
        pair_address,
    };
    let (reserve0, reserve1) = get_reserves(reserve_config, &chain, &client).await?;

    let reserve0 = format_units(reserve0, token_0.decimals)
        .expect("Should not happen")
        .parse::<f64>()?;
    let reserve1 = format_units(reserve1, token_1.decimals)
        .expect("Should not happen")
        .parse::<f64>()?;
    let total_supply = format_ether(plugin_total_supply).parse::<f64>()?;
    let tvl = get_tvl(reserve0, reserve1, token_0.price, token_1.price);

    let plugin_address_checksumed = to_checksum(&plugin_address, None);

    let plugin = ActivePlugin {
        address: ActiveValue::Set(plugin_address_checksumed.to_owned()),
        gauge_address: ActiveValue::Set(to_checksum(&gauge, None)),
        symbol: ActiveValue::Set(symbol),
        token0_address: ActiveValue::Set(token_0.address.to_lowercase()),
        token0: ActiveValue::Set(json!(token_0)),
        token1_address: ActiveValue::Set(token_1.address.to_lowercase()),
        token1: ActiveValue::Set(json!(token_1)),
        reserve0: ActiveValue::Set(reserve0),
        reserve1: ActiveValue::Set(reserve1),
        total_supply: ActiveValue::Set(total_supply),
        tvl: ActiveValue::Set(tvl),
    };

    match write_plugin(conn, plugin_address_checksumed, plugin).await {
        Ok(_) => {}
        Err(e) => {
            error!("Error writing to DB: {:?}", e);
        }
    }

    if gauge != Address::zero() {
        info!("Pair {} is a gauge", plugin_address);
        update_gauge(
            plugin_address,
            gauge,
            bribe,
            tvl,
            &chain,
            Arc::clone(&client),
            &conn,
        )
        .await?;
    } else {
        info!("Pair {} is not a gauge", plugin_address);
    }

    Ok(())
}

fn get_tvl(reserve0: f64, reserve1: f64, token0_price: f64, token1_price: f64) -> f64 {
    let mut tvl = 0.0;

    if token0_price != 0.0 {
        tvl += reserve0 * token0_price;
    }

    if token1_price != 0.0 {
        tvl += reserve1 * token1_price;
    }

    if tvl != 0.0 && (token0_price == 0.0 || token1_price == 0.0) {
        tvl = tvl * 2.0;
    }

    tvl
}

async fn get_reserves(
    reserve_config: ReserveConfig,
    chain: &Chain,
    client: &Arc<Provider<Http>>,
) -> Result<(U256, U256)> {
    let ReserveConfig {
        protocol,
        token_0_address,
        token_1_address,
        stable,
        plugin_total_supply,
        pair_address,
    } = reserve_config;

    match protocol.as_str() {
        "Velocimeter" => {
            let router_address = &chain.get_chain_data().velocimeter_router_address;
            let router_parsed_address = router_address.parse::<Address>().expect("Set by hand");
            let router = VelocimeterRouter::new(router_parsed_address, Arc::clone(&client));
            let (reserve0, reserve1) = router
                .quote_remove_liquidity(
                    token_0_address,
                    token_1_address,
                    stable,
                    plugin_total_supply,
                )
                .call()
                .await?;
            return Ok((reserve0, reserve1));
        }
        "Aerodrome" => {
            abigen!(
                AerodromePair,
                r#"[
                    factory() public view returns (address)
                ]"#,
            );
            let aerodrome_pair = AerodromePair::new(pair_address, Arc::clone(&client));
            let factory_address = aerodrome_pair.factory().call().await?;

            let router_address = &chain.get_chain_data().aerodrome_router_address;
            let router_parsed_address = router_address.parse::<Address>().expect("Set by hand");
            let router = AerodromeRouter::new(router_parsed_address, Arc::clone(&client));
            let (reserve0, reserve1) = router
                .quote_remove_liquidity(
                    token_0_address,
                    token_1_address,
                    stable,
                    factory_address,
                    plugin_total_supply,
                )
                .call()
                .await?;
            return Ok((reserve0, reserve1));
        }
        _ => panic!("Protocol is not supported"),
    }
}

async fn write_plugin(
    conn: &Arc<DatabaseConnection>,
    plugin_address: String,
    plugin: ActivePlugin,
) -> Result<(), StatusCode> {
    match Plugins::insert(plugin)
        .on_conflict(
            sea_query::OnConflict::column(PluginsColumn::Address)
                .update_columns([
                    PluginsColumn::GaugeAddress,
                    PluginsColumn::Reserve0,
                    PluginsColumn::Reserve1,
                    PluginsColumn::TotalSupply,
                    PluginsColumn::Tvl,
                    PluginsColumn::Token0,
                    PluginsColumn::Token1,
                ])
                .to_owned(),
        )
        .exec(conn.as_ref())
        .await
        .map_err(internal_error)
    {
        Ok(_) => {}
        Err(e) => {
            error!("Error writing to DB: {:?}, pair {}", e, plugin_address);
            return Err(e);
        }
    }
    info!("Pair {} DB write successful", plugin_address);
    Ok(())
}
