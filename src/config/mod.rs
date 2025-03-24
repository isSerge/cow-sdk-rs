use serde::{Deserialize, Serialize};
use serde_with::serde_as;

pub mod network;

pub use network::Network;

#[derive(Debug, Deserialize, Serialize)]
#[serde_as]
pub struct Config {
    #[serde_as(as = "DisplayFromStr")]
    pub network: Network,
    // pub eth_address: String,
    // pub private_key: String,
}

impl Config {
    pub fn new(network: Network) -> Self {
        Config {
            network,
            // eth_address: "0x0000000000000000000000000000000000000000".to_string(),
            // private_key: "0x0000000000000000000000000000000000000000000000000000000000000000"
            //     .to_string(),
        }
    }

    pub fn rpc_url(&self) -> String {
        self.network.rpc_url().to_string()
    }

    pub fn api_url(&self) -> String {
        self.network.api_url().to_string()
    }
}
