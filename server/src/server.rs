use warp::Filter;

use crate::config::DefaultProductConfig;

pub fn server(
    default_product: DefaultProductConfig,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("v3").and(info_v1(default_product))
}

/// global server information and capabilities
/// API: https://space-market.github.io/API/swagger-ui/#!/server/get_info
fn info_v1(
    default_product: DefaultProductConfig,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("info")
        .and(warp::get())
        .and(with_default_product(default_product))
        .and_then(handler::info_v1)
}

fn with_default_product(
    product: DefaultProductConfig,
) -> impl Filter<Extract = (DefaultProductConfig,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || product.clone())
}

mod handler {
    use common::ServerInfo;
    use std::convert::Infallible;

    use crate::config::DefaultProductConfig;

    pub(super) async fn info_v1(
        default_product: DefaultProductConfig,
    ) -> Result<impl warp::Reply, Infallible> {
        let info = ServerInfo {
            version: "3.0.0".to_string(),
            currency: "€".to_string(),
            decimal_seperator: Some(",".to_string()),
            energy: "kJ".to_string(),
            default_product: default_product.into(),
            ..Default::default()
        };

        Ok(warp::reply::json(&info))
    }
}
