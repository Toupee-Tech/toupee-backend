use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::sync::Arc;
use std::{env, time::Instant};
use tokio::time::{sleep, Duration};
use tracing::{info, instrument, log::LevelFilter};

mod assets;
mod bribes;
mod gauges;
mod plugin_aprs;
mod plugins;

use assets::{update_assets_from_tokenlist, update_other_db_assets_prices};
use backend::config::types::Chain;
use plugins::update_plugins;

#[instrument]
pub async fn syncer() {
    let db_url = env::var("DATABASE_URL").expect("Should be defined in .env");
    let mut opt = ConnectOptions::new(db_url);
    opt.sqlx_logging_level(LevelFilter::Trace);

    let iteration_time_secs = env::var("ITERATION_TIME_SECS")
        .expect("Should be defined in .env")
        .parse::<u64>()
        .expect("Should be a number");

    // set up connection
    let conn = Database::connect(opt)
        .await
        .expect("Could not connect to database");

    let conn: Arc<_> = Arc::new(conn);

    let base_rpc_url = env::var("BASE_RPC_URL")
        .expect("Should be defined in .env")
        .to_string();
    let base_chain = Chain::new(base_rpc_url, 8453);

    iteration_run(base_chain.clone(), Arc::clone(&conn)).await;

    let six_hours = Duration::from_secs(iteration_time_secs);
    loop {
        info!("Sleeping for {} seconds", six_hours.as_secs());
        sleep(six_hours).await;
        iteration_run(base_chain.clone(), Arc::clone(&conn)).await;
    }
}

async fn iteration_run(chain: Chain, conn: Arc<DatabaseConnection>) {
    let now = Instant::now();

    update_assets_from_tokenlist(&chain, &conn).await.unwrap();
    update_other_db_assets_prices(&chain, &conn).await.unwrap();
    update_plugins(&chain, &conn).await.unwrap();

    info!("Iteration took {} seconds", now.elapsed().as_secs());
}
