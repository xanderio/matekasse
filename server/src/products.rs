use warp::Filter;

use crate::storage::Db;

pub fn products(
    db: Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("v3").and(
        list_products_v1(db)
            .or(create_product_v1())
            .or(delete_project_v1())
            .or(list_product_v1())
            .or(edit_product_v1()),
    )
}

/// returns all products
/// API: https://space-market.github.io/API/swagger-ui/#!/products/get_products
fn list_products_v1(
    db: Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("products")
        .and(warp::get())
        .and_then(move || handler::list_products_v1(db.clone()))
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
    use sea_orm::entity::*;

    use crate::{entity::product::Entity as ProductModel, storage::Db};

    /// returns all products
    pub(super) async fn list_products_v1(db: Db) -> Result<impl warp::Reply, Infallible> {
        let products = ProductModel::find()
            .all(&db.orm)
            .await
            .unwrap()
            .into_iter()
            .map(Into::into)
            .collect::<Vec<Product>>();

        Ok(warp::reply::json(&products))
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
