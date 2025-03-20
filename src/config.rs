use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub network: String,
    pub rpc_url: String,
    pub eth_address: String,
    pub private_key: String,
    pub api_url: String,
}

impl Config {
    pub fn new() -> Self {
        Config {
            network: "mainnet".to_string(),
            rpc_url: "https://mainnet.infura.io/v3/".to_string(),
            eth_address: "0x0000000000000000000000000000000000000000".to_string(),
            private_key: "0x0000000000000000000000000000000000000000000000000000000000000000"
                .to_string(),
            api_url: "https://api.example.com".to_string(),
        }
    }
}
