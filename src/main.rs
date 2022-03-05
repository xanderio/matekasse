use axum::{AddExtensionLayer, Router};
use eyre::Result;
use tower_http::trace::TraceLayer;
use tracing::info;

mod config;
mod entity;
mod models;
mod products;
mod server;
mod storage;
mod user;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let config = config::load_config().await.expect("unable to load config");

    let db = storage::open_db(config.storage.database.clone()).await?;

    let api_routes = Router::new()
        .nest("/info", server::router())
        .nest("/users", user::router())
        .nest("/products", products::router());

    let app = Router::new()
        .nest("/api/v3", api_routes)
        .layer(AddExtensionLayer::new(config.clone()))
        .layer(AddExtensionLayer::new(db.clone()))
        .layer(TraceLayer::new_for_http());

    info!("listening on {}", config.http.listen);
    axum::Server::bind(&config.http.listen)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
