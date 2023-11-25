use axum::{http::StatusCode, routing::get, Extension, Router};
use sea_orm::Database;
use std::net::SocketAddr;
use tracing::info;

use std::env;

mod handlers;
use handlers::{give_assets, give_plugins, root};

mod types;

pub async fn server() {
    let db_url = env::var("DATABASE_URL").expect("Should be defined in .env");

    // set up connection
    let conn = Database::connect(db_url)
        .await
        .expect("Could not connect to database");

    // build our application with some routes
    let app = Router::new()
        .route("/", get(root))
        .route("/plugins/", get(give_plugins))
        .route("/assets/", get(give_assets))
        .layer(Extension(conn));

    // run it with hyper
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Could not start server");
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
pub fn internal_error<E>(_err: E) -> StatusCode
where
    E: std::error::Error,
{
    StatusCode::INTERNAL_SERVER_ERROR
}
