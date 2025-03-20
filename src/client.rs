use crate::config::Config;
use alloy::providers::{Provider, ProviderBuilder};
use reqwest::Client as HttpClient;
use reqwest::Url;
use std::sync::Arc;

pub struct Client {
    config: Config,
    http_client: Arc<HttpClient>,
    provider: Arc<dyn Provider>,
    // TODO: Add a signer
}

impl Client {
    pub fn new(config: Config) -> Self {
        let rpc_url = Url::parse(&config.rpc_url).expect("Invalid RPC URL");
        Self {
            config,
            http_client: Arc::new(HttpClient::new()),
            provider: Arc::new(ProviderBuilder::new().on_http(rpc_url)),
        }
    }
}
