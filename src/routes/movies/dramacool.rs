use crate::models::ProviderInfo;
use axum::{extract::Query, http::StatusCode, routing::get, Json, Router};
use consumet::providers::movies::{DramaCool, DramaCoolSearchResults};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DramaCoolSearch {
    pub query: String,
    pub page: Option<usize>,
}

pub async fn mount() -> Router {
    Router::new()
        .route("/", get(dramacool_home))
        .route("/search", get(dramacool_search))
}

pub async fn dramacool_home() -> (StatusCode, Json<ProviderInfo>) {
    (
        StatusCode::OK,
        Json(ProviderInfo {
            intro: String::from(
                r#"Welcome to the dramacool provider: check out the provider's website @ https://dramacool.com.pa/"#,
            ),
            routes: vec![String::from("/search")],
            documentation: String::from("https://docs.consumet.org/#tag/dramacool"),
        }),
    )
}

pub async fn dramacool_search(
    route_param: Query<DramaCoolSearch>,
) -> (StatusCode, Json<DramaCoolSearchResults>) {
    let search = DramaCool
        .search(&route_param.query, route_param.page)
        .await
        .unwrap();

    (StatusCode::OK, Json(search))
}
