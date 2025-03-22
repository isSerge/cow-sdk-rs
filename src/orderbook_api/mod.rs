mod url;

use eyre::Error;
use reqwest::Client as HttpClient;
use std::sync::Arc;
use url::OrderApiUrl;

#[derive(Debug)]
pub struct OrderApiClient {
    client: Arc<HttpClient>,
    api_url: OrderApiUrl,
}

impl OrderApiClient {
    pub fn new(url: &str) -> Result<Self, Error> {
        let api_url = OrderApiUrl::new(url);
        let client = Arc::new(HttpClient::new());
        Ok(Self { client, api_url })
    }

    pub async fn get_orders(&self) -> Result<(), Error> {
        unimplemented!()
    }

    pub async fn get_order_by_id(&self, order_id: &str) -> Result<(), Error> {
        unimplemented!()
    }

    pub async fn get_order_by_tx_hash(&self, tx_hash: &str) -> Result<(), Error> {
        unimplemented!()
    }

    pub async fn get_order_status(&self, order_id: &str) -> Result<(), Error> {
        unimplemented!()
    }

    pub async fn create_order(&self) -> Result<(), Error> {
        unimplemented!()
    }

    pub async fn cancel_order(&self, order_id: &str) -> Result<(), Error> {
        unimplemented!()
    }

    pub async fn get_user_orders(&self, address: &str) -> Result<(), Error> {
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

    pub async fn get_auction(&self) -> Result<(), Error> {
        unimplemented!()
    }

    pub async fn get_competition_by_id(&self, auction_id: &str) -> Result<(), Error> {
        unimplemented!()
    }

    pub async fn get_competition_by_tx_hash(&self, tx_hash: &str) -> Result<(), Error> {
        unimplemented!()
    }

    pub async fn get_latest_competition(&self) -> Result<(), Error> {
        unimplemented!()
    }

    pub async fn get_token_price(&self, token_address: &str) -> Result<(), Error> {
        unimplemented!()
    }

    pub async fn get_token_prices(&self) -> Result<(), Error> {
        unimplemented!()
    }

    pub async fn get_version(&self) -> Result<(), Error> {
        unimplemented!()
    }
}
