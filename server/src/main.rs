use warp::Filter;

use std::net::SocketAddr;

mod products;

#[tokio::main]
async fn main() {
    let api = warp::path("api");

    let products = api.and(products::products());

    warp::serve(products)
        .run(
            "127.0.0.1:3000"
                .parse::<SocketAddr>()
                .expect("unable to parse socket addr"),
        )
        .await;
}
