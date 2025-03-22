use serde_derive::{Deserialize, Serialize};

pub mod network;

pub use network::Network;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub network: String,
    pub rpc_url: String,
    // pub eth_address: String,
    // pub private_key: String,
    pub api_url: String,
}

impl Config {
    pub fn new(network: Network) -> Self {
        Config {
            network: network.to_string(),
            rpc_url: network.rpc_url().to_string(),
            // eth_address: "0x0000000000000000000000000000000000000000".to_string(),
            // private_key: "0x0000000000000000000000000000000000000000000000000000000000000000"
            //     .to_string(),
            api_url: network.api_url().to_string(),
        }
    }
}
