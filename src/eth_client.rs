use std::sync::Arc;

use alloy::providers::{Provider, ProviderBuilder};
use eyre::{Context, Result};
use reqwest::Url;

use crate::config::Config;

pub struct EthClient {
    config: Config,
    provider: Arc<dyn Provider>,
    // TODO: Add a signer
}

impl EthClient {
    pub fn new(config: Config) -> Result<Self> {
        let rpc_url = Url::parse(&config.rpc_url()).wrap_err("Invalid RPC URL")?;
        let provider = Arc::new(ProviderBuilder::new().on_http(rpc_url));
        Ok(Self { config, provider })
    }
}
