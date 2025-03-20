use eyre::Error;
use reqwest::Client as HttpClient;
use reqwest::Url;
use std::sync::Arc;

pub struct OrderApiClient {
    client: Arc<HttpClient>,
    url: Url,
}

impl OrderApiClient {
    pub fn new(url: &str) -> Result<Self, Error> {
        let url = Url::parse(url).expect("Invalid RPC URL");
        let client = Arc::new(HttpClient::new());
        Ok(Self { client, url })
    }

    pub async fn get_orders(&self) -> Result<(), Error> {
        unimplemented!()
    }

    pub async fn get_order(&self, order_id: &str) -> Result<(), Error> {
        unimplemented!()
    }

    pub async fn create_order(&self) -> Result<(), Error> {
        unimplemented!()
    }

    pub async fn cancel_order(&self, order_id: &str) -> Result<(), Error> {
        unimplemented!()
    }

    pub async fn get_quote(&self) -> Result<(), Error> {
        unimplemented!()
    }

    pub async fn get_allowance(&self, token_address: &str, spender: &str) -> Result<(), Error> {
        unimplemented!()
    }

    pub async fn set_allowance(&self, token_address: &str, spender: &str) -> Result<(), Error> {
        unimplemented!()
    }

    pub async fn get_trades(&self) -> Result<(), Error> {
        unimplemented!()
    }

    pub async fn get_trade(&self, trade_id: &str) -> Result<(), Error> {
        unimplemented!()
    }
}
