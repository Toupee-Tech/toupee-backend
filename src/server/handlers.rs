use axum::{http::StatusCode, response::Json, Extension};
use sea_orm::{DatabaseConnection, EntityTrait, LoaderTrait};

use backend::database::aprs::Entity as Aprs;
use backend::database::assets::{Entity as Assets, Model as Asset};
use backend::database::bribes::Entity as Bribes;
use backend::database::gauges::Entity as Gauges;
use backend::database::plugins::Entity as Plugins;

use crate::server::{
    internal_error,
    types::{PluginResponse, PluginsResponse},
};

// basic handler that responds with a static string
pub async fn root() -> &'static str {
    "Hello, World!"
}

#[axum_macros::debug_handler]
pub async fn give_plugins(
    Extension(conn): Extension<DatabaseConnection>,
) -> Result<Json<PluginsResponse>, StatusCode> {
    let plugins = Plugins::find().all(&conn).await.map_err(internal_error)?;

    let gauges = plugins
        .load_many(Gauges, &conn)
        .await
        .map_err(internal_error)?;

    let aprs = plugins
        .load_many(Aprs, &conn)
        .await
        .map_err(internal_error)?;

    let bribes = plugins
        .load_many(Bribes, &conn)
        .await
        .map_err(internal_error)?;

    let plugins_res = plugins
        .into_iter()
        .zip(gauges.into_iter())
        .zip(aprs.into_iter())
        .zip(bribes.into_iter())
        .map(|(((pair, gauges), aprs), bribes)| {
            PluginResponse::new(pair, gauges.first().cloned(), aprs, bribes)
        })
        .collect();

    let res = PluginsResponse {
        plugins: plugins_res,
    };

    Ok(Json(res))
}

#[axum_macros::debug_handler]
pub async fn give_assets(
    Extension(conn): Extension<DatabaseConnection>,
) -> Result<Json<Vec<Asset>>, StatusCode> {
    let res = Assets::find().all(&conn).await.map_err(internal_error)?;

    Ok(Json(res))
}
