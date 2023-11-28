use axum::http::StatusCode;
use ethers::{
    contract::Multicall,
    providers::{Http, Provider},
    types::{Address, H160, U256},
    utils::{format_units, to_checksum},
};
use eyre::Result;
use sea_orm::{
    sea_query, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter,
};
use std::sync::Arc;
use tracing::{error, info};

use crate::server::internal_error;
use crate::syncer::assets::{check_if_token_is_option, check_option_ve_discount, find_asset};
use backend::bindings::{Gauge, Voter};
use backend::config::types::Chain;
use backend::database::aprs::{
    ActiveModel as ActiveAprsModel, Column as AprsColumn, Entity as Aprs,
};

pub async fn update_plugin_aprs(
    plugin_address: H160,
    gauge_address: H160,
    tvl: f64,
    chain: &Chain,
    client: Arc<Provider<Http>>,
    conn: &Arc<DatabaseConnection>,
) -> Result<()> {
    let gauge = Gauge::new(gauge_address, Arc::clone(&client));
    let voter_address = chain
        .get_chain_data()
        .wig_voter_address
        .parse::<Address>()?;
    let voter = Voter::new(voter_address, Arc::clone(&client));
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

    // let rewards_list_lenght = gauge.rewards_list_length().call().await?;
    // currently only one reward token is spitting out in toupee gauges
    let rewards_list_lenght = 1;

    // list of reward tokens in gauge contract
    let mut reward_token_addresses = vec![];

    for i in 0..rewards_list_lenght {
        let reward_token_addy = gauge.reward_tokens(U256::from(i)).call().await?;
        reward_token_addresses.push(reward_token_addy);
    }

    clean_up_stale_rewards(plugin_address, reward_token_addresses.clone(), conn).await?;

    for reward_token_addy in reward_token_addresses {
        let formatted_reward_token_addy = to_checksum(&reward_token_addy, None);

        let reward_token = find_asset(formatted_reward_token_addy.to_owned(), chain, conn).await?;

        multicall.add_call(gauge.get_reward_for_duration(reward_token_addy), false);
        multicall.add_call(gauge.left(reward_token_addy), false);
        multicall.add_call(voter.is_alive(gauge_address), false);

        let (seven_days_reward_rate, left, is_alive) =
            multicall.call::<(U256, U256, bool)>().await?;
        multicall.clear_calls();

        let reward: f64 = if is_alive && left > U256::zero() {
            // 86400 seconds in a day
            format_units(seven_days_reward_rate, reward_token.decimals)?.parse::<f64>()? / 7.0
        } else {
            0.0
        };

        let (is_option, underlying_token) =
            check_if_token_is_option(&reward_token.address, Arc::clone(&client)).await?;

        if is_option {
            let underlying_token =
                find_asset(to_checksum(&underlying_token, None), chain, conn).await?;

            let ve_discount =
                check_option_ve_discount(&reward_token.address, Arc::clone(&client)).await?;
            let max_token_price = underlying_token.price * (100.0 - ve_discount) / 100.0;

            let min_apr: f64;
            let max_apr: f64;

            if tvl == 0.0 {
                min_apr = 0.0;
                max_apr = 0.0;
            } else {
                min_apr = reward * reward_token.price / tvl * 100.0 * 365.0;
                max_apr = reward * max_token_price / tvl * 100.0 * 365.0;
            }

            if max_apr < 0.1 {
                let apr = ActiveAprsModel {
                    apr: ActiveValue::set(Some(0.0)),
                    plugin_address: ActiveValue::set(to_checksum(&plugin_address, None)),
                    logo_url: ActiveValue::set(reward_token.logoURI),
                    max_apr: ActiveValue::not_set(),
                    min_apr: ActiveValue::not_set(),
                    token_address: ActiveValue::set(formatted_reward_token_addy.to_lowercase()),
                    symbol: ActiveValue::set(reward_token.symbol),
                };

                match write_apr(conn, apr).await {
                    Ok(_) => {}
                    Err(e) => {
                        error!("Error writing to DB: {:?}", e);
                    }
                }
                continue;
            } else {
                let apr = ActiveAprsModel {
                    apr: ActiveValue::not_set(),
                    plugin_address: ActiveValue::set(to_checksum(&plugin_address, None)),
                    logo_url: ActiveValue::set(reward_token.logoURI),
                    max_apr: ActiveValue::set(Some(max_apr)),
                    min_apr: ActiveValue::set(Some(min_apr)),
                    token_address: ActiveValue::set(formatted_reward_token_addy.to_lowercase()),
                    symbol: ActiveValue::set(reward_token.symbol),
                };

                match write_apr(conn, apr).await {
                    Ok(_) => {}
                    Err(e) => {
                        error!("Error writing to DB: {:?}", e);
                    }
                }
                continue;
            }
        } else {
            let apr: f64 = if tvl == 0.0 {
                0.0
            } else {
                reward * reward_token.price / tvl * 100.0 * 365.0
            };

            let apr = ActiveAprsModel {
                apr: ActiveValue::set(Some(apr)),
                plugin_address: ActiveValue::set(to_checksum(&plugin_address, None)),
                logo_url: ActiveValue::set(reward_token.logoURI),
                max_apr: ActiveValue::not_set(),
                min_apr: ActiveValue::not_set(),
                token_address: ActiveValue::set(formatted_reward_token_addy.to_lowercase()),
                symbol: ActiveValue::set(reward_token.symbol),
            };

            match write_apr(conn, apr).await {
                Ok(_) => {}
                Err(e) => {
                    error!("Error writing to DB: {:?}", e);
                }
            }
        }
    }

    Ok(())
}

///
/// If DB has rewards that are not longer in list of reward tokens in gauge contract, remove them.
///
async fn clean_up_stale_rewards(
    plugin_address: H160,
    reward_tokens: Vec<Address>,
    conn: &Arc<DatabaseConnection>,
) -> Result<()> {
    let plugin_address = to_checksum(&plugin_address, None);
    let aprs = Aprs::find()
        .filter(AprsColumn::PluginAddress.eq(&plugin_address))
        .all(conn.as_ref())
        .await?;

    if aprs.is_empty() {
        return Ok(());
    }

    let reward_tokens = reward_tokens
        .into_iter()
        .map(|t_a| to_checksum(&t_a, None).to_lowercase())
        .collect::<Vec<String>>();

    for apr in aprs {
        if !reward_tokens.contains(&apr.token_address.to_lowercase()) {
            let p_a = apr.plugin_address.clone();
            let t_s = apr.symbol.clone();
            let delete_result = apr.delete(conn.as_ref()).await;
            match delete_result {
                Ok(_) => {
                    info!("Deleted stale apr entry: p_a {}, t_s {}", p_a, t_s);
                }
                Err(e) => {
                    error!("Error deleting stale bribe entry: {:?}", e);
                }
            }
        }
    }

    Ok(())
}

async fn write_apr(conn: &Arc<DatabaseConnection>, apr: ActiveAprsModel) -> Result<(), StatusCode> {
    match Aprs::insert(apr)
        .on_conflict(
            sea_query::OnConflict::columns([AprsColumn::PluginAddress, AprsColumn::TokenAddress])
                .update_columns([AprsColumn::Apr, AprsColumn::MaxApr, AprsColumn::MinApr])
                .to_owned(),
        )
        .exec(conn.as_ref())
        .await
        .map_err(internal_error)
    {
        Ok(_) => {}
        Err(e) => {
            error!("Error writing aprs to DB: {:?}", e);
            return Err(e);
        }
    }
    info!("Aprs DB write successful");
    Ok(())
}
