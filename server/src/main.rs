use eyre::Result;
use warp::Filter;

mod config;
mod entity;
mod products;
mod server;
mod storage;
mod user;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    let config = config::load_config().await.expect("unable to load config");

    let db = storage::open_db(config.storage.database.clone()).await?;

    let api = warp::path("api");

    let products = api.and(products::products(db.clone(), config.clone()));
    let users = api.and(user::users(db.clone()));
    let server = api.and(server::server(config.default_product.clone()));

    let routes = crate::balanced_or_tree!(products, users, server);

    println!("listening on {}", config.http.listen);
    warp::serve(routes).run(config.http.listen).await;

    Ok(())
}
