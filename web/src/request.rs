use anyhow::{anyhow, Result};
use common::{Product, User};
use reqwasm::http::Request;

pub async fn fetch_all_users() -> Result<Vec<User>> {
    let url = "/api/v3/users";
    let resp = Request::get(url).send().await?;

    resp.json().await.map_err(|e| anyhow!(e))
}

pub async fn buy_product(user: &User, product: &Product) -> Result<User> {
    let url = format!("/api/v3/users/{}/buy", user.id);
    let resp = Request::post(&url).body(product.id).send().await?;

    resp.json().await.map_err(|e| anyhow!(e))
}

pub async fn fetch_all_products() -> Result<Vec<Product>> {
    let url = "/api/v3/products";
    let resp = Request::get(url).send().await?;
    resp.json().await.map_err(|e| anyhow!(e))
}
