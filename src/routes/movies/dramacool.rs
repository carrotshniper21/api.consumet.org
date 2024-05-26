use crate::models::ProviderInfo;
use axum::{extract::Query, http::StatusCode, routing::get, Json, Router};
use consumet::{
    models::StreamingServers,
    providers::movies::dramacool::{
        DramaCool, DramaCoolInfo, DramaCoolSearchResults, DramaCoolServers, DramaCoolSources,
    },
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DramaCoolSearch {
    pub query: String,
    pub page: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct DramaCoolMediaInfo {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct DramaCoolServer {
    pub episode_id: String,
}

#[derive(Debug, Deserialize)]
pub struct DramaCoolSource {
    pub episode_id: String,
    pub server: Option<StreamingServers>,
}

pub async fn mount() -> Router {
    Router::new()
        .route("/", get(dramacool_home))
        .route("/search", get(dramacool_search))
        .route("/info", get(dramacool_info))
        .route("/servers", get(dramacool_servers))
        .route("/sources", get(dramacool_sources))
}

pub async fn dramacool_home() -> (StatusCode, Json<ProviderInfo>) {
    (
        StatusCode::OK,
        Json(ProviderInfo {
            intro: String::from(
                r#"Welcome to the dramacool provider: check out the provider's website @ https://dramacool.com.pa/"#,
            ),
            routes: vec![
                String::from("/search"),
                String::from("/info"),
                String::from("/servers"),
                String::from("/sources"),
            ],
            documentation: String::from("https://docs.consumet.org/#tag/dramacool"),
        }),
    )
}

pub async fn dramacool_search(
    route_params: Query<DramaCoolSearch>,
) -> (StatusCode, Json<DramaCoolSearchResults>) {
    let search = DramaCool
        .search(&route_params.query, route_params.page)
        .await
        .unwrap();

    (StatusCode::OK, Json(search))
}

pub async fn dramacool_info(
    route_params: Query<DramaCoolMediaInfo>,
) -> (StatusCode, Json<DramaCoolInfo>) {
    let info = DramaCool.info(&route_params.id).await.unwrap();

    (StatusCode::OK, Json(info))
}

pub async fn dramacool_servers(
    route_params: Query<DramaCoolServer>,
) -> (StatusCode, Json<DramaCoolServers>) {
    let server = DramaCool.servers(&route_params.episode_id).await.unwrap();

    (StatusCode::OK, Json(server))
}

pub async fn dramacool_sources(
    route_params: Query<DramaCoolSource>,
) -> (StatusCode, Json<DramaCoolSources>) {
    let sources = DramaCool
        .sources(&route_params.episode_id, route_params.server)
        .await
        .unwrap();

    (StatusCode::OK, Json(sources))
}
