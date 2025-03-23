mod url;

use std::sync::Arc;

use alloy::primitives::{Address, TxHash};
use eyre::{Error, Result};
use reqwest::Client as HttpClient;
use serde_json::Value;
use url::OrderApiUrl;

use crate::{
    config::Network,
    models::{
        order::Order,
        response::{CompetitionOrderStatusResponse, SolverCompetitionResponse, TokenPriceResponse},
        trade::Trade,
    },
    parsing::parse_response,
    primitives::order_uid::OrderUid,
};

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
    async fn get_response_body(&self, url: &str) -> Result<String, Error> {
        let response = self.client.get(url).send().await?;
        let status = response.status();
        let body = response.text().await?;

        // If the status is not success, return an error.
        if !status.is_success() {
            return Err(eyre::eyre!("HTTP Error {}: {}", status, body));
        }

        Ok(body)
    }

    pub async fn get_order_by_id(&self, order_id: &OrderUid) -> Result<Order, Error> {
        let url = self.api_url.get_order_by_id(order_id.to_string().as_str());
        let body = self.get_response_body(&url).await?;
        let json: Order = parse_response(&body)?;
        Ok(json)
    }

    pub async fn get_orders_by_tx_hash(&self, tx_hash: &TxHash) -> Result<Vec<Order>, Error> {
        let url = self.api_url.get_order_by_tx_hash(tx_hash.to_string().as_str());
        let body = self.get_response_body(&url).await?;
        let json: Vec<Order> = parse_response(&body)?;
        Ok(json)
    }

    pub async fn get_order_status(
        &self,
        order_id: &OrderUid,
    ) -> Result<CompetitionOrderStatusResponse, Error> {
        let url = self.api_url.get_order_status(order_id.to_string().as_str());
        let body = self.get_response_body(&url).await?;
        let json: CompetitionOrderStatusResponse = parse_response(&body)?;
        Ok(json)
    }

    pub async fn create_order(&self) -> Result<(), Error> {
        unimplemented!()
    }

    pub async fn cancel_order(&self, order_id: &str) -> Result<(), Error> {
        unimplemented!()
    }

    pub async fn get_user_orders(&self, address: &Address) -> Result<Vec<Order>, Error> {
        let url = self.api_url.get_user_orders(address.to_string().as_str());
        let body = self.get_response_body(&url).await?;
        let json: Vec<Order> = parse_response(&body)?;
        Ok(json)
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
        let body = self.get_response_body(&url).await?;
        let json: Vec<Trade> = parse_response(&body)?;
        Ok(json)
    }

    /// Permissioned endpoint.
    // TODO: get permission and implement struct
    pub async fn get_auction(&self) -> Result<Value, Error> {
        let url = self.api_url.get_auction();
        let body = self.get_response_body(&url).await?;
        let json: Value = parse_response(&body)?;
        Ok(json)
    }

    pub async fn get_competition_by_id(
        &self,
        auction_id: &i64,
    ) -> Result<SolverCompetitionResponse, Error> {
        let url = self.api_url.get_solver_competition_by_id(auction_id.to_string().as_str());
        let body = self.get_response_body(&url).await?;
        let json: SolverCompetitionResponse = parse_response(&body)?;
        Ok(json)
    }

    pub async fn get_competition_by_tx_hash(
        &self,
        tx_hash: &TxHash,
    ) -> Result<SolverCompetitionResponse, Error> {
        let url = self.api_url.get_solver_competition_by_tx_hash(tx_hash.to_string().as_str());
        let body = self.get_response_body(&url).await?;
        let json: SolverCompetitionResponse = parse_response(&body)?;
        Ok(json)
    }

    pub async fn get_latest_competition(&self) -> Result<SolverCompetitionResponse, Error> {
        let url = self.api_url.get_solver_competition_latest();
        let body = self.get_response_body(&url).await?;
        let json: SolverCompetitionResponse = parse_response(&body)?;
        Ok(json)
    }

    pub async fn get_token_price(
        &self,
        token_address: &Address,
    ) -> Result<TokenPriceResponse, Error> {
        let url = self.api_url.get_native_price(token_address.to_string().as_str());
        let body = self.get_response_body(&url).await?;
        let json: TokenPriceResponse = parse_response(&body)?;
        Ok(json)
    }

    pub async fn get_token_prices(&self) -> Result<(), Error> {
        unimplemented!()
    }

    pub async fn get_version(&self) -> Result<String, Error> {
        let url = self.api_url.get_api_version();
        let body = self.get_response_body(&url).await?;
        Ok(body)
    }
}
