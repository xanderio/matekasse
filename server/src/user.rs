use warp::Filter;

use crate::storage::Db;

pub fn users(
    db: Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("v3").and(
        add_balance_v1(db.clone())
            .or(list_user_v1(db.clone()))
            .or(list_users_v1(db.clone()))
            .or(create_user_v1(db.clone()))
            .or(delete_user_v1(db.clone()))
            .or(edit_user_v1(db)),
    )
}

/// returns all products
/// API: https://space-market.github.io/API/swagger-ui/#!/products/get_products
fn list_users_v1(
    db: Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("users")
        .and(warp::get())
        .and(with_db(db))
        .and_then(handler::list_users_v1)
}

fn create_user_v1(
    db: Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("users")
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 32))
        .and(warp::body::json())
        .and(with_db(db))
        .and_then(handler::create_user_v1)
}

fn delete_user_v1(
    db: Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("users" / i32)
        .and(warp::delete())
        .and(with_db(db))
        .and_then(handler::delete_user_v1)
}

fn list_user_v1(
    db: Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("users" / i32)
        .and(warp::get())
        .and(with_db(db))
        .and_then(handler::list_user_v1)
}

fn edit_user_v1(
    db: Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("users" / i32)
        .and(warp::patch())
        .and(warp::body::content_length_limit(1024 * 32))
        .and(warp::body::json())
        .and(with_db(db))
        .and_then(handler::edit_user_v1)
}

fn add_balance_v1(
    db: Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("users" / i32 / "deposit")
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 32))
        .and(warp::body::json())
        .and(with_db(db))
        .and_then(handler::add_balance_v1)
}

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

mod handler {
    use std::convert::{Infallible, TryInto};

    use common::{User, UserCreateRequest, UserEditRequest};
    use sea_orm::{entity::*, DbErr};
    use warp::{hyper::StatusCode, reply};

    use crate::{
        entity::user::{self, Entity as UserModel},
        storage::Db,
    };

    /// returns all products
    pub(super) async fn list_users_v1(db: Db) -> Result<impl warp::Reply, Infallible> {
        let users = UserModel::find()
            .all(&db.orm)
            .await
            .unwrap()
            .into_iter()
            .map(Into::into)
            .collect::<Vec<User>>();

        Ok(reply::json(&users))
    }

    pub(super) async fn create_user_v1(
        user: UserCreateRequest,
        db: Db,
    ) -> Result<Box<dyn warp::Reply>, Infallible> {
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

        match user
            .save(&db.orm)
            .await
            .map_err(Into::into)
            .and_then::<User, _>(TryInto::try_into)
        {
            Ok(user) => Ok(Box::new(reply::with_status(
                reply::json(&user),
                StatusCode::CREATED,
            ))),
            Err(err) => {
                // this is a ugly hack, but i'm not sure how to clean this up :(
                if let Some(DbErr::Exec(msg)) = err.downcast_ref::<DbErr>() {
                    if msg.contains("UNIQUE constraint failed: user.name") {
                        return Ok(Box::new(reply::with_status(
                            "a user with the same name already exists",
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

    pub(super) async fn delete_user_v1(id: i32, db: Db) -> Result<impl warp::Reply, Infallible> {
        match UserModel::find_by_id(id).one(&db.orm).await {
            Ok(Some(user)) => {
                let user: user::ActiveModel = user.into();
                if user.delete(&db.orm).await.is_ok() {
                    Ok(reply::with_status("user deleted", StatusCode::OK))
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

    pub(super) async fn list_user_v1(id: i32, db: Db) -> Result<Box<dyn warp::Reply>, Infallible> {
        match UserModel::find_by_id(id).one(&db.orm).await {
            Ok(Some(user)) => {
                let user: User = user.into();
                Ok(Box::new(reply::json(&user)))
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

    pub(super) async fn edit_user_v1(
        id: i32,
        body: UserEditRequest,
        db: Db,
    ) -> Result<Box<dyn warp::Reply>, Infallible> {
        match UserModel::find_by_id(id).one(&db.orm).await {
            Ok(Some(user)) => {
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

                if let Ok(user) = user.save(&db.orm).await {
                    let user: User = user.try_into().unwrap();
                    Ok(Box::new(reply::with_status(
                        reply::json(&user),
                        StatusCode::OK,
                    )))
                } else {
                    Ok(Box::new(reply::with_status(
                        "server error",
                        StatusCode::INTERNAL_SERVER_ERROR,
                    )))
                }
            }
            Ok(None) => Ok(Box::new(reply::with_status(
                "id not existent",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))),
            Err(_) => Ok(Box::new(reply::with_status(
                "server error",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))),
        }
    }

    pub(super) async fn add_balance_v1(
        id: i32,
        amount: i32,
        db: Db,
    ) -> Result<Box<dyn warp::Reply>, Infallible> {
        match UserModel::find_by_id(id).one(&db.orm).await {
            Ok(Some(user)) => {
                let mut user = user.into_active_model();

                user.balance = Set(user.balance.unwrap() + amount);

                if let Ok(user) = user.save(&db.orm).await {
                    let user: User = user.try_into().unwrap();
                    Ok(Box::new(reply::with_status(
                        reply::json(&user),
                        StatusCode::OK,
                    )))
                } else {
                    Ok(Box::new(reply::with_status(
                        "server error",
                        StatusCode::INTERNAL_SERVER_ERROR,
                    )))
                }
            }
            Ok(None) => Ok(Box::new(reply::with_status(
                "id not existent",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))),
            Err(_) => Ok(Box::new(reply::with_status(
                "server error",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))),
        }
    }
}
