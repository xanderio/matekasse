use warp::Filter;

pub fn server() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("v3").and(info_v1())
}

/// global server information and capabilities
/// API: https://space-market.github.io/API/swagger-ui/#!/server/get_info
fn info_v1() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("info")
        .and(warp::get())
        .and_then(handler::info_v1)
}

mod handler {
    use common::ServerInfo;
    use std::convert::Infallible;

    pub(super) async fn info_v1() -> Result<impl warp::Reply, Infallible> {
        let info = ServerInfo {
            version: "3.0.0".to_string(),
            currency: "€".to_string(),
            decimal_seperator: Some(",".to_string()),
            energy: "kJ".to_string(),
            ..Default::default()
        };

        Ok(warp::reply::json(&info))
    }
}
