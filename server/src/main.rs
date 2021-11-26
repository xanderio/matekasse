use warp::Filter;

mod config;
mod products;
mod server;

#[tokio::main]
async fn main() {
    let config = config::load_config().await.expect("unable to load config");

    let api = warp::path("api");

    let products = api.and(products::products());
    let server = api.and(server::server());

    let routes = products.or(server);

    println!("listening on {}", config.http.listen);
    warp::serve(routes).run(config.http.listen).await;
}
