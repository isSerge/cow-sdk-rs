use std::{fmt, str};

// Orderbook API URLs
const MAINNET_PROD_API_URL: &str = "https://api.cow.fi/mainnet";
const MAINNET_STAGING_API_URL: &str = "https://barn.api.cow.fi/mainnet";
const SEPOLIA_PROD_API_URL: &str = "https://api.cow.fi/sepolia";
const SEPOLIA_STAGING_API_URL: &str = "https://barn.api.cow.fi/sepolia";
const BASE_PROD_API_URL: &str = "https://api.cow.fi/base";
const BASE_STAGING_API_URL: &str = "https://barn.api.cow.fi/base";
const ARBITRUM_PROD_API_URL: &str = "https://api.cow.fi/arbitrum_one";
const ARBITRUM_STAGING_API_URL: &str = "https://barn.api.cow.fi/arbitrum_one";
const GNOSIS_PROD_API_URL: &str = "https://api.cow.fi/xdai";
const GNOSIS_STAGING_API_URL: &str = "https://barn.api.cow.fi/xdai";
const LOCAL_API_URL: &str = "http://localhost:8080";

// RPC URLs
const MAINNET_RPC_URL: &str = "https://mainnet.infura.io/v3/";
const SEPOLIA_RPC_URL: &str = "https://sepolia.infura.io/v3/";
const BASE_RPC_URL: &str = "https://base.infura.io/v3/";
const ARBITRUM_RPC_URL: &str = "https://arbitrum.infura.io/v3/";
const GNOSIS_RPC_URL: &str = "https://xdai.infura.io/v3/";
const LOCAL_RPC_URL: &str = "http://localhost:8545";

#[derive(Debug, Default, Clone)]
pub enum Network {
    #[default]
    Mainnet,
    MainnetStaging,
    Sepolia,
    SepoliaStaging,
    Base,
    BaseStaging,
    Arbitrum,
    ArbitrumStaging,
    Gnosis,
    GnosisStaging,
    Local,
}

impl Network {
    pub fn api_url(&self) -> &str {
        match self {
            Network::Mainnet => MAINNET_PROD_API_URL,
            Network::MainnetStaging => MAINNET_STAGING_API_URL,
            Network::Sepolia => SEPOLIA_PROD_API_URL,
            Network::SepoliaStaging => SEPOLIA_STAGING_API_URL,
            Network::Base => BASE_PROD_API_URL,
            Network::BaseStaging => BASE_STAGING_API_URL,
            Network::Arbitrum => ARBITRUM_PROD_API_URL,
            Network::ArbitrumStaging => ARBITRUM_STAGING_API_URL,
            Network::Gnosis => GNOSIS_PROD_API_URL,
            Network::GnosisStaging => GNOSIS_STAGING_API_URL,
            Network::Local => LOCAL_API_URL,
        }
    }

    pub fn rpc_url(&self) -> &str {
        match self {
            Network::Mainnet | Network::MainnetStaging => MAINNET_RPC_URL,
            Network::Sepolia | Network::SepoliaStaging => SEPOLIA_RPC_URL,
            Network::Base | Network::BaseStaging => BASE_RPC_URL,
            Network::Arbitrum | Network::ArbitrumStaging => ARBITRUM_RPC_URL,
            Network::Gnosis | Network::GnosisStaging => GNOSIS_RPC_URL,
            Network::Local => LOCAL_RPC_URL,
        }
    }
}

impl str::FromStr for Network {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mainnet" => Ok(Network::Mainnet),
            "mainnet-staging" => Ok(Network::MainnetStaging),
            "sepolia" => Ok(Network::Sepolia),
            "sepolia-staging" => Ok(Network::SepoliaStaging),
            "base" => Ok(Network::Base),
            "base-staging" => Ok(Network::BaseStaging),
            "arbitrum" => Ok(Network::Arbitrum),
            "arbitrum-staging" => Ok(Network::ArbitrumStaging),
            "gnosis" => Ok(Network::Gnosis),
            "gnosis-staging" => Ok(Network::GnosisStaging),
            "local" => Ok(Network::Local),
            _ => Err(format!("Network not found: {}", s)),
        }
    }
}

impl fmt::Display for Network {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Network::Mainnet => write!(f, "mainnet"),
            Network::MainnetStaging => write!(f, "mainnet-staging"),
            Network::Sepolia => write!(f, "sepolia"),
            Network::SepoliaStaging => write!(f, "sepolia-staging"),
            Network::Base => write!(f, "base"),
            Network::BaseStaging => write!(f, "base-staging"),
            Network::Arbitrum => write!(f, "arbitrum"),
            Network::ArbitrumStaging => write!(f, "arbitrum-staging"),
            Network::Gnosis => write!(f, "gnosis"),
            Network::GnosisStaging => write!(f, "gnosis-staging"),
            Network::Local => write!(f, "local"),
        }
    }
}
