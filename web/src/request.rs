use anyhow::Result;
use common::{Product, User};
use yew::{
    format::{Json, Nothing},
    services::{
        fetch::{FetchTask, Request, Response},
        FetchService,
    },
    Callback,
};

pub fn fetch_all_users(cb: Callback<Result<Vec<User>>>) -> Result<FetchTask> {
    let request = Request::get("/api/v3/users").body(Nothing)?;

    FetchService::fetch(
        request,
        Callback::once(move |resp: Response<Json<Result<Vec<User>>>>| {
            let Json(data) = resp.into_body();
            cb.emit(data)
        }),
    )
}

pub fn buy_product(
    user: &User,
    product: &Product,
    cb: Callback<Result<User>>,
) -> Result<FetchTask> {
    let request =
        Request::post(format!("/api/v3/users/{}/buy", user.id)).body(Ok(product.id.to_string()))?;

    FetchService::fetch(
        request,
        Callback::once(move |resp: Response<Json<Result<User>>>| {
            let Json(data) = resp.into_body();
            cb.emit(data)
        }),
    )
}

pub fn fetch_all_products(cb: Callback<Result<Vec<Product>>>) -> Result<FetchTask> {
    let request = Request::get("/api/v3/products").body(Nothing)?;

    FetchService::fetch(
        request,
        Callback::once(move |resp: Response<Json<Result<Vec<Product>>>>| {
            let Json(data) = resp.into_body();
            cb.emit(data)
        }),
    )
}
