use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ProviderInfo {
    pub intro: String,
    pub routes: Vec<String>,
    pub documentation: String,
}

#[derive(Debug, Deserialize)]
pub struct FlixhqSearch {
    pub query: String,
    pub page: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseError {
    pub message: String,
    pub error: String,
}
