use warp::Filter;

pub fn products() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("v1").and(
        list_products_v1()
            .or(create_product_v1())
            .or(delete_project_v1())
            .or(list_product_v1())
            .or(edit_product_v1()),
    )
}

/// returns all products
/// API: https://space-market.github.io/API/swagger-ui/#!/products/get_products
fn list_products_v1() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
{
    warp::path("products")
        .and(warp::get())
        .and_then(handler::list_products_v1)
}

fn create_product_v1() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
{
    warp::path("products")
        .and(warp::post())
        .and_then(handler::create_product_v1)
}

fn delete_project_v1() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
{
    warp::path!("products" / u64)
        .and(warp::delete())
        .and_then(handler::delete_product_v1)
}

fn list_product_v1() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
{
    warp::path!("products" / u64)
        .and(warp::get())
        .and_then(handler::list_product_v1)
}

fn edit_product_v1() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
{
    warp::path!("products" / u64)
        .and(warp::patch())
        .and_then(handler::edit_product_v1)
}

mod handler {
    use std::convert::Infallible;

    use common::Product;

    fn products() -> Vec<Product> {
        vec![
            Product {
                id: 1,
                name: "Flora Mate".to_string(),
                price: 200,
                active: true,
                ..Default::default()
            },
            Product {
                id: 2,
                name: "Mio Mio Mate".to_string(),
                price: 150,
                active: true,
                ..Default::default()
            },
            Product {
                id: 3,
                name: "Mate Mate".to_string(),
                price: 150,
                active: true,
                ..Default::default()
            },
            Product {
                id: 4,
                name: "Kaffe".to_string(),
                price: 50,
                active: true,
                ..Default::default()
            },
            Product {
                id: 5,
                name: "Spezi".to_string(),
                price: 150,
                active: true,
                ..Default::default()
            },
        ]
    }

    /// returns all products
    pub(super) async fn list_products_v1() -> Result<impl warp::Reply, Infallible> {
        Ok(warp::reply::json(&products()))
    }

    pub(super) async fn create_product_v1() -> Result<impl warp::Reply, Infallible> {
        Ok(warp::reply::reply())
    }

    pub(super) async fn delete_product_v1(_id: u64) -> Result<impl warp::Reply, Infallible> {
        Ok(warp::reply::reply())
    }

    pub(super) async fn list_product_v1(_id: u64) -> Result<impl warp::Reply, Infallible> {
        Ok(warp::reply::reply())
    }

    pub(super) async fn edit_product_v1(_id: u64) -> Result<impl warp::Reply, Infallible> {
        Ok(warp::reply::reply())
    }
}
