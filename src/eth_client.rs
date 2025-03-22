use std::sync::Arc;

use alloy::providers::{Provider, ProviderBuilder};
use reqwest::Url;

use crate::config::Config;

pub struct EthClient {
    config: Config,
    provider: Arc<dyn Provider>,
    // TODO: Add a signer
}

impl EthClient {
    pub fn new(config: Config) -> Self {
        let rpc_url = Url::parse(&config.rpc_url).expect("Invalid RPC URL");
        Self { config, provider: Arc::new(ProviderBuilder::new().on_http(rpc_url)) }
    }
}
