use axum::{extract::Extension, routing::get, Json, Router};
use common::ServerInfo;

use crate::config::Config;

pub fn router() -> Router {
    Router::new().route("/", get(info_v3))
}

/// global server information and capabilities
/// API: https://space-market.github.io/API/swagger-ui/#!/server/get_info
pub(super) async fn info_v3(Extension(config): Extension<Config>) -> Json<ServerInfo> {
    Json(ServerInfo {
        version: "3.0.0".to_string(),
        currency: "€".to_string(),
        decimal_seperator: Some(",".to_string()),
        energy: "kJ".to_string(),
        default_product: config.default_product.into(),
        ..Default::default()
    })
}
