use warp::Filter;

use std::net::SocketAddr;

mod products;
mod server;

#[tokio::main]
async fn main() {
    let api = warp::path("api");

    let products = api.and(products::products());
    let server = api.and(server::server());

    let routes = products.or(server);

    warp::serve(routes)
        .run(
            "127.0.0.1:3000"
                .parse::<SocketAddr>()
                .expect("unable to parse socket addr"),
        )
        .await;
}
