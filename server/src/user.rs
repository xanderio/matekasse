use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;

use std::convert::TryInto;

use common::{User, UserCreateRequest, UserEditRequest};
use sea_orm::entity::*;

use crate::{
    entity::{
        product,
        user::{self, Entity as UserModel},
    },
    storage::Db,
    utils::{AppError, Result},
};

pub fn router() -> Router {
    Router::new()
        .route("/", get(list_users_v3).post(create_user_v3))
        .route("/:id/:operation", post(modify_balance_v3))
        .route("/:id/buy", post(buy_product_v3))
        .route(
            "/:id",
            get(list_user_v3).patch(edit_user_v3).delete(delete_user_v3),
        )
}

/// returns all products
async fn list_users_v3(Extension(db): Extension<Db>) -> Result<Json<Vec<User>>> {
    let users = UserModel::find()
        .all(&db.orm)
        .await?
        .into_iter()
        .map(Into::into)
        .collect::<Vec<User>>();

    Ok(Json(users))
}

async fn create_user_v3(
    Json(user): Json<UserCreateRequest>,
    Extension(db): Extension<Db>,
) -> Result<(StatusCode, Json<User>)> {
    let user = user::ActiveModel {
        name: Set(user.name),
        email: Set(user.email),
        balance: Set(user.balance.unwrap_or(0)),
        active: Set(user.active.unwrap_or(true)),
        audit: Set(user.audit.unwrap_or(false)),
        redirect: Set(user.redirect.unwrap_or(true)),
        avatar: user
            .avatar
            .map(Option::Some)
            .map(ActiveValue::set)
            .unwrap_or_else(ActiveValue::unset),
        ..Default::default()
    };

    let user = user.save(&db.orm).await?.try_into()?;
    Ok((StatusCode::CREATED, Json(user)))
}

async fn delete_user_v3(Path(id): Path<i32>, Extension(db): Extension<Db>) -> Result<&'static str> {
    UserModel::find_by_id(id)
        .one(&db.orm)
        .await?
        .ok_or(AppError::NotFount)?
        .into_active_model()
        .delete(&db.orm)
        .await?;
    Ok("user deleted")
}

async fn list_user_v3(Path(id): Path<i32>, Extension(db): Extension<Db>) -> Result<Json<User>> {
    let user = UserModel::find_by_id(id)
        .one(&db.orm)
        .await?
        .ok_or(AppError::NotFount)?
        .into();
    Ok(Json(user))
}

async fn edit_user_v3(
    Path(id): Path<i32>,
    Json(body): Json<UserEditRequest>,
    Extension(db): Extension<Db>,
) -> Result<Json<User>> {
    let user = UserModel::find_by_id(id)
        .one(&db.orm)
        .await?
        .ok_or(AppError::NotFount)?;

    let mut user = user.into_active_model();

    user.name = body.name.map(ActiveValue::set).unwrap_or(user.name);
    user.email = body
        .email
        .map(Option::Some)
        .map(ActiveValue::set)
        .unwrap_or(user.email);
    user.balance = body.balance.map(ActiveValue::set).unwrap_or(user.balance);
    user.active = body.active.map(ActiveValue::set).unwrap_or(user.active);
    user.audit = body.audit.map(ActiveValue::set).unwrap_or(user.audit);
    user.redirect = body.redirect.map(ActiveValue::set).unwrap_or(user.redirect);
    user.avatar = body
        .avatar
        .map(Option::Some)
        .map(ActiveValue::set)
        .unwrap_or(user.avatar);

    let user = user.save(&db.orm).await?.try_into()?;
    Ok(Json(user))
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Operation {
    Spend,
    Deposit,
}

async fn modify_balance_v3(
    Path((id, operation)): Path<(i32, Operation)>,
    Json(amount): Json<i32>,
    Extension(db): Extension<Db>,
) -> Result<Json<User>> {
    let user = UserModel::find_by_id(id)
        .one(&db.orm)
        .await?
        .ok_or(AppError::NotFount)?;
    let balance = user.balance;
    let mut user = user.into_active_model();
    user.balance = Set(match operation {
        Operation::Deposit => balance + amount,
        Operation::Spend => balance - amount,
    });

    let user: User = user.save(&db.orm).await?.try_into()?;
    Ok(Json(user))
}

async fn buy_product_v3(
    Path(user_id): Path<i32>,
    Json(product_id): Json<i32>,
    Extension(db): Extension<Db>,
) -> Result<Json<User>> {
    let product = product::Entity::find_by_id(product_id)
        .one(&db.orm)
        .await?
        .ok_or(AppError::NotFount)?;

    let user = UserModel::find_by_id(user_id)
        .one(&db.orm)
        .await?
        .ok_or(AppError::NotFount)?;
    let balance = user.balance;
    let mut user = user.into_active_model();
    user.balance = Set(balance - product.price);

    let user: User = user.save(&db.orm).await?.try_into()?;
    Ok(Json(user))
}
