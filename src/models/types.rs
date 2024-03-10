use serde::{Deserialize, Serialize};
use std::{net::IpAddr, time::Duration};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub addr: IpAddr,
    pub port: u16,
    #[serde(with = "humantime_serde")]
    pub shutdown_timeout: Option<Duration>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProviderInfo {
    pub intro: String,
    pub routes: Vec<String>,
    pub documentation: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseError {
    pub message: String,
    pub error: String,
}
