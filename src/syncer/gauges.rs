use axum::http::StatusCode;
use ethers::{
    contract::Multicall,
    providers::{Http, Provider},
    types::{Address, H160, U256},
    utils::{format_ether, format_units, to_checksum},
};
use eyre::Result;
use sea_orm::{sea_query, ActiveValue, DatabaseConnection, EntityTrait};
use std::sync::Arc;
use tracing::{error, info};

use crate::server::internal_error;
use crate::syncer::{assets::find_asset, bribes::update_bribe, plugin_aprs::update_plugin_aprs};
use backend::bindings::{Gauge, Voter};
use backend::config::types::Chain;
use backend::database::gauges::{
    ActiveModel as ActiveGauge, Column as GaugesColumn, Entity as Gauges,
};

pub async fn update_gauge(
    plugin_address: H160,
    gauge_address: H160,
    bribe_address: H160,
    tvl: f64,
    chain: &Chain,
    client: Arc<Provider<Http>>,
    conn: &Arc<DatabaseConnection>,
) -> Result<()> {
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

    let voter_address = chain
        .get_chain_data()
        .wig_voter_address
        .parse::<Address>()?;
    let default_token_address = chain.get_chain_data().o_wig_address.parse::<Address>()?;
    let gauge = Gauge::new(gauge_address, client.clone());
    let voter = Voter::new(voter_address, client.clone());

    multicall.add_call(gauge.total_supply(), false);
    multicall.add_call(gauge.get_reward_for_duration(default_token_address), false);
    multicall.add_call(voter.weights(plugin_address), false);

    let (total_supply, seven_days_reward_rate, votes) =
        multicall.call::<(U256, U256, U256)>().await?;

    let total_supply = format_ether(total_supply).parse::<f64>()?;
    let reward = format_units(seven_days_reward_rate, 18)?.parse::<f64>()? / 7.0;
    let votes = format_units(votes, 18)?.parse::<f64>()?;

    let mut max_tbv = 0.0;
    let mut median_tbv = 0.0;
    let mut min_tbv = 0.0;

    if bribe_address != H160::zero() {
        (min_tbv, median_tbv, max_tbv) = update_bribe(
            plugin_address,
            bribe_address,
            chain,
            Arc::clone(&client),
            conn,
        )
        .await?;
    }

    let (min_apr, max_apr) = match update_gauge_aprs(min_tbv, max_tbv, votes, chain, conn).await {
        Ok((min_apr, max_apr)) => (min_apr, max_apr),
        Err(e) => {
            info!(
                "Error updating gauge APRs: {:?}, plugin_address {:?}",
                e, plugin_address
            );
            (0.0, 0.0)
        }
    };

    let gauge_address_checksumed = to_checksum(&gauge_address, None);

    let gauge = ActiveGauge {
        address: ActiveValue::set(gauge_address_checksumed.to_owned()),
        plugin_address: ActiveValue::set(to_checksum(&plugin_address, None)),
        decimals: ActiveValue::set(18),
        total_supply: ActiveValue::set(total_supply),
        reward: ActiveValue::set(reward),
        bribe_address: ActiveValue::set(to_checksum(&bribe_address, None)),
        votes: ActiveValue::set(votes),
        max_tbv: ActiveValue::set(max_tbv),
        median_tbv: ActiveValue::set(median_tbv),
        min_tbv: ActiveValue::set(min_tbv),
        max_apr: ActiveValue::set(max_apr),
        min_apr: ActiveValue::set(min_apr),
    };

    update_plugin_aprs(plugin_address, gauge_address, tvl, chain, client, conn).await?;

    match write_gauge(conn, gauge_address_checksumed, gauge).await {
        Ok(_) => {}
        Err(e) => {
            error!("Error writing to DB: {:?}", e);
        }
    }

    Ok(())
}

async fn update_gauge_aprs(
    min_tbv: f64,
    max_tbv: f64,
    votes: f64,
    chain: &Chain,
    conn: &Arc<DatabaseConnection>,
) -> Result<(f64, f64)> {
    let mut max_apr = 0.0;
    let mut min_apr = 0.0;

    let token = find_asset(chain.get_chain_data().wig_address.to_string(), chain, conn).await?;

    if token.price * votes > 0.0 {
        max_apr = max_tbv * 52.0 / (token.price * votes) * 100.0;
        min_apr = min_tbv * 52.0 / (token.price * votes) * 100.0;
    }

    Ok((min_apr, max_apr))
}

async fn write_gauge(
    conn: &Arc<DatabaseConnection>,
    gauge_address: String,
    gauge: ActiveGauge,
) -> Result<(), StatusCode> {
    match Gauges::insert(gauge)
        .on_conflict(
            sea_query::OnConflict::column(GaugesColumn::Address)
                .update_columns([
                    GaugesColumn::Votes,
                    GaugesColumn::TotalSupply,
                    GaugesColumn::MaxApr,
                    GaugesColumn::MinApr,
                    GaugesColumn::MedianTbv,
                    GaugesColumn::MinTbv,
                    GaugesColumn::MaxTbv,
                    GaugesColumn::Reward,
                ])
                .to_owned(),
        )
        .exec(conn.as_ref())
        .await
        .map_err(internal_error)
    {
        Ok(_) => {}
        Err(e) => {
            error!("Error writing to DB: {:?}, gauge {}", e, gauge_address);
            return Err(e);
        }
    }
    info!("Gauge {} DB write successful", gauge_address);
    Ok(())
}
