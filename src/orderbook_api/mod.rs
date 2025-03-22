mod url;

use std::sync::Arc;

use alloy::primitives::TxHash;
use eyre::Error;
use reqwest::Client as HttpClient;
use url::OrderApiUrl;

use crate::{
    config::Network, order::Order, parsing::parse_response, primitives::order_uid::OrderUid,
};

#[derive(Debug)]
pub struct OrderApiClient {
    client: Arc<HttpClient>,
    api_url: OrderApiUrl,
}

impl OrderApiClient {
    pub fn new(network: Network) -> Result<Self, Error> {
        let api_url = OrderApiUrl::new(network.api_url());
        let client = Arc::new(HttpClient::new());
        Ok(Self { client, api_url })
    }

    pub async fn get_orders(&self) -> Result<(), Error> {
        unimplemented!()
    }

    pub async fn get_order_by_id(&self, order_id: &OrderUid) -> Result<Order, Error> {
        let url = self.api_url.get_order_by_id(order_id.to_string().as_str());
        let response = self.client.get(url).send().await?;
        let body = response.text().await?;
        let order: Order = parse_response(&body)?;
        Ok(order)
    }

    pub async fn get_orders_by_tx_hash(&self, tx_hash: &TxHash) -> Result<Vec<Order>, Error> {
        let url = self.api_url.get_order_by_tx_hash(tx_hash.to_string().as_str());
        let response = self.client.get(url).send().await?;
        let body = response.text().await?;
        let orders: Vec<Order> = parse_response(&body)?;
        Ok(orders)
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
