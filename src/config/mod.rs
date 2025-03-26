use serde::{Deserialize, Serialize};
use serde_with::serde_as;

pub mod network;

pub use network::Network;

#[derive(Debug, Deserialize, Serialize)]
#[serde_as]
pub struct Config {
    #[serde_as(as = "DisplayFromStr")]
    pub network: Network,
}

impl Config {
    pub fn new(network: Network) -> Self {
        Config { network }
    }

    pub fn rpc_url(&self) -> String {
        self.network.rpc_url().to_string()
    }

    pub fn api_url(&self) -> String {
        self.network.api_url().to_string()
    }
}
