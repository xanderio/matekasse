use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    routing::get,
    Json, Router,
};

use std::convert::TryInto;

use common::{Product, ProductCreateRequest, ProductEditRequest};
use sea_orm::entity::*;

use crate::{
    config::Config,
    entity::product::{self, Entity as ProductModel},
    storage::Db,
    utils::{AppError, Result},
};

pub fn router() -> Router {
    Router::new()
        .route("/", get(list_products_v3).post(create_product_v3))
        .route(
            "/:id",
            get(list_product_v3)
                .delete(delete_product_v3)
                .patch(edit_product_v3),
        )
}

async fn list_products_v3(Extension(db): Extension<Db>) -> Result<Json<Vec<Product>>> {
    let products = ProductModel::find()
        .all(&db.orm)
        .await?
        .into_iter()
        .map(Into::into)
        .collect::<Vec<Product>>();

    Ok(Json(products))
}

async fn create_product_v3(
    Json(product): Json<ProductCreateRequest>,
    Extension(db): Extension<Db>,
    Extension(config): Extension<Config>,
) -> Result<(StatusCode, Json<Product>)> {
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

    let product = product.save(&db.orm).await?.try_into()?;
    Ok((StatusCode::CREATED, Json(product)))
}

async fn delete_product_v3(
    Path(id): Path<i32>,
    Extension(db): Extension<Db>,
) -> Result<&'static str> {
    ProductModel::find_by_id(id)
        .one(&db.orm)
        .await?
        .ok_or(AppError::NotFount)?
        .into_active_model()
        .delete(&db.orm)
        .await?;
    Ok("product deleted")
}

async fn list_product_v3(
    Path(id): Path<i32>,
    Extension(db): Extension<Db>,
) -> Result<Json<Product>> {
    let product = ProductModel::find_by_id(id)
        .one(&db.orm)
        .await?
        .ok_or(AppError::NotFount)?
        .into();
    Ok(Json(product))
}

async fn edit_product_v3(
    Path(id): Path<i32>,
    Json(body): Json<ProductEditRequest>,
    Extension(db): Extension<Db>,
) -> Result<Json<Product>> {
    let mut product = ProductModel::find_by_id(id)
        .one(&db.orm)
        .await?
        .ok_or(AppError::NotFount)?
        .into_active_model();

    product.name = body.name.map(ActiveValue::set).unwrap_or(product.name);
    product.caffeine = body
        .caffeine
        .map(Option::Some)
        .map(ActiveValue::set)
        .unwrap_or(product.caffeine);
    product.alcohol = body
        .alcohol
        .map(Option::Some)
        .map(ActiveValue::set)
        .unwrap_or(product.alcohol);
    product.energy = body
        .energy
        .map(Option::Some)
        .map(ActiveValue::set)
        .unwrap_or(product.energy);
    product.sugar = body
        .sugar
        .map(Option::Some)
        .map(ActiveValue::set)
        .unwrap_or(product.sugar);
    product.price = body.price.map(ActiveValue::set).unwrap_or(product.price);
    product.active = body.active.map(ActiveValue::set).unwrap_or(product.active);
    product.image = body
        .image
        .map(Option::Some)
        .map(ActiveValue::set)
        .unwrap_or(product.image);

    let product = product.save(&db.orm).await?.try_into()?;
    Ok(Json(product))
}
