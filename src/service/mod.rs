pub mod material;
pub mod project;
pub mod submission;

use std::sync::OnceLock;

use gloo_console::info;
use serde::Deserialize;

use crate::utilities::requests::fetch::{get_request_struct, FetchError};

pub static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub backend_domain: String,
    pub backend_url: String,
    /// Website the "Back" button leads to
    pub auth_website: String,
    /// URL that authenticates the user and leads them back to this website immediately.
    pub auth_url: String,
}

pub async fn get_config() -> Result<Config, FetchError> {
    info!("Loading config");
    get_request_struct("/config/config.json").await
}

fn backend() -> String {
    CONFIG.get().expect("Config unset").backend_url.to_owned()
}
