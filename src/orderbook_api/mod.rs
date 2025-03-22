mod url;

use std::sync::Arc;

use alloy::primitives::{Address, TxHash};
use eyre::{Error, Result};
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Value;
use url::OrderApiUrl;

use crate::{
    config::Network,
    order::{CompetitionOrderStatus, Order},
    parsing::parse_response,
    primitives::order_uid::OrderUid,
    trade::Trade,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct CompetitionOrderStatusResponse {
    pub r#type: CompetitionOrderStatus,
    pub value: Value,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenPriceResponse {
    pub price: f64,
}

#[derive(Debug)]
pub struct OrderApiClient {
    client: Arc<HttpClient>,
    api_url: OrderApiUrl,
}

#[derive(Debug)]
pub enum GetTradesQuery {
    ByOwner(Address),
    ByOrderId(OrderUid),
}

impl OrderApiClient {
    pub fn new(network: Network) -> Result<Self, Error> {
        let api_url = OrderApiUrl::new(network.api_url());
        let client = Arc::new(HttpClient::new());
        Ok(Self { client, api_url })
    }

    /// Gets a resource from the orderbook API and parses it into the specified
    /// type.
    async fn get_and_parse<T: DeserializeOwned>(&self, url: &str) -> Result<T, Error> {
        let response = self.client.get(url).send().await?;
        let status = response.status();
        let body = response.text().await?;

        // If the status is not success, return an error.
        if !status.is_success() {
            return Err(eyre::eyre!("HTTP Error {}: {}", status, body));
        }

        println!("body: {}", body);
        let json: T = parse_response(&body)?;
        Ok(json)
    }

    pub async fn get_order_by_id(&self, order_id: &OrderUid) -> Result<Order, Error> {
        let url = self.api_url.get_order_by_id(order_id.to_string().as_str());
        self.get_and_parse(&url).await
    }

    pub async fn get_orders_by_tx_hash(&self, tx_hash: &TxHash) -> Result<Vec<Order>, Error> {
        let url = self.api_url.get_order_by_tx_hash(tx_hash.to_string().as_str());
        self.get_and_parse(&url).await
    }

    pub async fn get_order_status(
        &self,
        order_id: &OrderUid,
    ) -> Result<CompetitionOrderStatusResponse, Error> {
        let url = self.api_url.get_order_status(order_id.to_string().as_str());
        self.get_and_parse(&url).await
    }

    pub async fn create_order(&self) -> Result<(), Error> {
        unimplemented!()
    }

    pub async fn cancel_order(&self, order_id: &str) -> Result<(), Error> {
        unimplemented!()
    }

    pub async fn get_user_orders(&self, address: &Address) -> Result<Vec<Order>, Error> {
        let url = self.api_url.get_user_orders(address.to_string().as_str());
        self.get_and_parse(&url).await
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

    pub async fn get_trades(&self, query: GetTradesQuery) -> Result<Vec<Trade>, Error> {
        let trades_url = self.api_url.get_trades();
        let url = match query {
            GetTradesQuery::ByOwner(owner) => format!("{}?owner={}", trades_url, owner),
            GetTradesQuery::ByOrderId(order_id) => format!("{}?orderUid={}", trades_url, order_id),
        };
        self.get_and_parse(&url).await
    }

    /// Permissioned endpoint.
    // TODO: get permission and implement struct
    pub async fn get_auction(&self) -> Result<(), Error> {
        let url = self.api_url.get_auction();
        self.get_and_parse(&url).await
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

    pub async fn get_token_price(
        &self,
        token_address: &Address,
    ) -> Result<TokenPriceResponse, Error> {
        let url = self.api_url.get_native_price(token_address.to_string().as_str());
        self.get_and_parse(&url).await
    }

    pub async fn get_token_prices(&self) -> Result<(), Error> {
        unimplemented!()
    }

    pub async fn get_version(&self) -> Result<(), Error> {
        unimplemented!()
    }
}
