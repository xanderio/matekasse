use warp::Filter;

use crate::{config::Config, storage::Db};

pub fn products(
    db: Db,
    config: Config,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("v3").and(
        list_product_v1(db.clone())
            .or(list_products_v1(db.clone()))
            .or(create_product_v1(db.clone(), config))
            .or(delete_project_v1(db))
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
        .and(with_db(db))
        .and_then(handler::list_products_v1)
}

fn create_product_v1(
    db: Db,
    config: Config,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("products")
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 32))
        .and(warp::body::json())
        .and(with_db(db))
        .and(with_config(config))
        .and_then(handler::create_product_v1)
}

fn delete_project_v1(
    db: Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("products" / i32)
        .and(warp::delete())
        .and(with_db(db))
        .and_then(handler::delete_product_v1)
}

fn list_product_v1(
    db: Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("products" / i32)
        .and(warp::get())
        .and(with_db(db))
        .and_then(handler::list_product_v1)
}

fn edit_product_v1() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
{
    warp::path!("products" / i32)
        .and(warp::patch())
        .and_then(handler::edit_product_v1)
}

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

fn with_config(
    config: Config,
) -> impl Filter<Extract = (Config,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || config.clone())
}

mod handler {
    use std::convert::{Infallible, TryInto};

    use common::{Product, ProductCreateRequest};
    use sea_orm::{entity::*, DbErr};
    use warp::{hyper::StatusCode, reply};

    use crate::{
        config::Config,
        entity::product::{self, Entity as ProductModel},
        storage::Db,
    };

    /// returns all products
    pub(super) async fn list_products_v1(db: Db) -> Result<impl warp::Reply, Infallible> {
        let products = ProductModel::find()
            .all(&db.orm)
            .await
            .unwrap()
            .into_iter()
            .map(Into::into)
            .collect::<Vec<Product>>();

        Ok(reply::json(&products))
    }

    pub(super) async fn create_product_v1(
        product: ProductCreateRequest,
        db: Db,
        config: Config,
    ) -> Result<Box<dyn warp::Reply>, Infallible> {
        let product = product::ActiveModel {
            name: Set(product.name),
            caffeine: Set(product.caffeine.or(config.default_product.caffeine)),
            alcohol: Set(product.alcohol.or(config.default_product.alcohol)),
            energy: Set(product.energy.or(config.default_product.energy)),
            sugar: Set(product.sugar.or(config.default_product.sugar)),
            price: Set(product.price.unwrap_or(config.default_product.price)),
            active: Set(product.active.unwrap_or(config.default_product.active)),
            ..Default::default()
        };

        match product
            .save(&db.orm)
            .await
            .map_err(Into::into)
            .and_then::<Product, _>(TryInto::try_into)
        {
            Ok(product) => Ok(Box::new(reply::with_status(
                reply::json(&product),
                StatusCode::CREATED,
            ))),
            Err(err) => {
                // this is a ugly hack, but i'm not sure how to clean this up :(
                if let Some(DbErr::Exec(msg)) = err.downcast_ref::<DbErr>() {
                    if msg.contains("UNIQUE constraint failed: product.name") {
                        return Ok(Box::new(reply::with_status(
                            "a product with the same name already exists",
                            StatusCode::CONFLICT,
                        )));
                    }
                };
                Ok(Box::new(reply::with_status(
                    format!("server error: {}", err),
                    StatusCode::INTERNAL_SERVER_ERROR,
                )))
            }
        }
    }

    pub(super) async fn delete_product_v1(id: i32, db: Db) -> Result<impl warp::Reply, Infallible> {
        match ProductModel::find_by_id(id).one(&db.orm).await {
            Ok(Some(product)) => {
                let product: product::ActiveModel = product.into();
                if product.delete(&db.orm).await.is_ok() {
                    Ok(reply::with_status("product deleted", StatusCode::OK))
                } else {
                    Ok(reply::with_status(
                        "server error",
                        StatusCode::INTERNAL_SERVER_ERROR,
                    ))
                }
            }
            Ok(None) => Ok(reply::with_status("id not existent", StatusCode::NOT_FOUND)),
            Err(_) => Ok(reply::with_status(
                "server error",
                StatusCode::INTERNAL_SERVER_ERROR,
            )),
        }
    }

    pub(super) async fn list_product_v1(
        id: i32,
        db: Db,
    ) -> Result<Box<dyn warp::Reply>, Infallible> {
        match ProductModel::find_by_id(id).one(&db.orm).await {
            Ok(Some(product)) => {
                let product: Product = product.into();
                Ok(Box::new(reply::json(&product)))
            }
            Ok(None) => Ok(Box::new(reply::with_status(
                "id not existent",
                StatusCode::NOT_FOUND,
            ))),
            Err(_) => Ok(Box::new(reply::with_status(
                "server error",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))),
        }
    }

    pub(super) async fn edit_product_v1(_id: i32) -> Result<impl warp::Reply, Infallible> {
        Ok(reply::reply())
    }
}
