mod url;

use alloy::primitives::{Address, TxHash};
use eyre::{Error, Result, WrapErr};
use reqwest::Client as HttpClient;
use serde_json::Value;
use url::OrderApiUrl;

use crate::{
    config::Network,
    models::{
        order::Order,
        response::{
            AppDataResponse, CompetitionOrderStatusResponse, SolverCompetitionResponse,
            TokenPriceResponse, TotalSurplusResponse,
        },
        trade::Trade,
    },
    parsing::parse_response,
    primitives::{app_data::AppDataHash, order_uid::OrderUid},
};

#[derive(Debug)]
pub struct OrderApiClient {
    client: HttpClient,
    api_url: OrderApiUrl,
}

#[derive(Debug)]
pub enum GetTradesQuery {
    ByOwner(Address),
    ByOrderId(OrderUid),
}

impl OrderApiClient {
    pub fn new(network: Network) -> Self {
        let api_url = OrderApiUrl::new(network.api_url());
        let client = HttpClient::new();
        Self { client, api_url }
    }

    /// Gets a resource from the orderbook API and returns the body as a string.
    async fn get_response_body(&self, url: &str) -> Result<String, Error> {
        let response = self
            .client
            .get(url)
            .send()
            .await
            .wrap_err_with(|| format!("Failed to send GET request to URL: {url}"))?;
        let status = response.status();
        let body = response.text().await.wrap_err("Failed to extract response body text")?;

        // If the status is not success, return an error.
        if !status.is_success() {
            return Err(eyre::eyre!("HTTP Error {}: {}", status, body));
        }

        Ok(body)
    }

    pub async fn get_order_by_id(&self, order_id: &OrderUid) -> Result<Order, Error> {
        let url = self.api_url.get_order_by_id(order_id.to_string().as_str())?;
        let body = self.get_response_body(&url).await?;
        let json: Order = parse_response(&body)?;
        Ok(json)
    }

    pub async fn get_orders_by_tx_hash(&self, tx_hash: &TxHash) -> Result<Vec<Order>, Error> {
        let url = self.api_url.get_order_by_tx_hash(tx_hash.to_string().as_str())?;
        let body = self.get_response_body(&url).await?;
        let json: Vec<Order> = parse_response(&body)?;
        Ok(json)
    }

    pub async fn get_order_status(
        &self,
        order_id: &OrderUid,
    ) -> Result<CompetitionOrderStatusResponse, Error> {
        let url = self.api_url.get_order_status(order_id.to_string().as_str())?;
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
        let url = self.api_url.get_user_orders(address.to_string().as_str())?;
        let body = self.get_response_body(&url).await?;
        let json: Vec<Order> = parse_response(&body)?;
        Ok(json)
    }

    pub async fn get_quote(&self) -> Result<(), Error> {
        unimplemented!()
    }

    pub async fn get_trades(&self, query: &GetTradesQuery) -> Result<Vec<Trade>, Error> {
        let url = self.api_url.get_trades(query)?;
        let body = self.get_response_body(&url).await?;
        let json: Vec<Trade> = parse_response(&body)?;
        Ok(json)
    }

    /// Permissioned endpoint.
    // TODO: get permission and implement struct
    pub async fn get_auction(&self) -> Result<Value, Error> {
        let url = self.api_url.get_auction()?;
        let body = self.get_response_body(&url).await?;
        let json: Value = parse_response(&body)?;
        Ok(json)
    }

    pub async fn get_competition_by_id(
        &self,
        auction_id: &i64,
    ) -> Result<SolverCompetitionResponse, Error> {
        let url = self.api_url.get_solver_competition_by_id(auction_id.to_string().as_str())?;
        let body = self.get_response_body(&url).await?;
        let json: SolverCompetitionResponse = parse_response(&body)?;
        Ok(json)
    }

    pub async fn get_competition_by_tx_hash(
        &self,
        tx_hash: &TxHash,
    ) -> Result<SolverCompetitionResponse, Error> {
        let url = self.api_url.get_solver_competition_by_tx_hash(tx_hash.to_string().as_str())?;
        let body = self.get_response_body(&url).await?;
        let json: SolverCompetitionResponse = parse_response(&body)?;
        Ok(json)
    }

    pub async fn get_latest_competition(&self) -> Result<SolverCompetitionResponse, Error> {
        let url = self.api_url.get_solver_competition_latest()?;
        let body = self.get_response_body(&url).await?;
        let json: SolverCompetitionResponse = parse_response(&body)?;
        Ok(json)
    }

    pub async fn get_token_price(
        &self,
        token_address: &Address,
    ) -> Result<TokenPriceResponse, Error> {
        let url = self.api_url.get_native_price(token_address.to_string().as_str())?;
        let body = self.get_response_body(&url).await?;
        let json: TokenPriceResponse = parse_response(&body)?;
        Ok(json)
    }

    pub async fn get_version(&self) -> Result<String, Error> {
        let url = self.api_url.get_api_version()?;
        let body = self.get_response_body(&url).await?;
        Ok(body)
    }

    pub async fn get_total_surplus(
        &self,
        address: &Address,
    ) -> Result<TotalSurplusResponse, Error> {
        let url = self.api_url.get_user_surplus(address.to_string().as_str())?;
        let body = self.get_response_body(&url).await?;
        let json: TotalSurplusResponse = parse_response(&body)?;
        Ok(json)
    }

    pub async fn get_app_data(
        &self,
        app_data_hash: &AppDataHash,
    ) -> Result<AppDataResponse, Error> {
        let app_data_hash_str = hex::encode(app_data_hash.0);
        let url = self.api_url.app_data_by_hash(app_data_hash_str.as_str())?;
        let body = self.get_response_body(&url).await?;
        let json: AppDataResponse = parse_response(&body)?;
        Ok(json)
    }

    pub async fn upload_app_data(&self) -> Result<(), Error> {
        unimplemented!()
    }
}
