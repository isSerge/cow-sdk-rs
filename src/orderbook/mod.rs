mod url;

use alloy::primitives::{Address, TxHash};
use eyre::{Error, Result, WrapErr};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};
use serde_json::Value;
use url::OrderApiUrl;

use crate::{
    config::Network,
    models::{
        order::{Order, OrderCancellations, PartialOrder},
        response::{
            AppDataResponse, CompetitionOrderStatusResponse, QuoteResponse,
            SolverCompetitionResponse, TokenPriceResponse, TotalSurplusResponse,
        },
        trade::Trade,
    },
    parsing::parse_response,
    primitives::{
        app_data::{AppData, AppDataHash},
        order_uid::OrderUid,
    },
};

#[derive(Debug)]
pub struct OrderApiClient {
    client: ClientWithMiddleware,
    api_url: OrderApiUrl,
}

#[derive(Debug)]
pub enum GetTradesQuery {
    ByOwner(Address),
    ByOrderId(OrderUid),
}

impl OrderApiClient {
    pub fn new(network: Network) -> Result<Self> {
        let api_url = OrderApiUrl::new(network.api_url())?;
        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
        let client = ClientBuilder::new(reqwest::Client::new())
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build();
        Ok(Self { client, api_url })
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

    pub async fn create_order(&self, order: &Order) -> Result<reqwest::StatusCode, Error> {
        let url = self.api_url.orders()?;
        let body = serde_json::to_string(order).wrap_err("Failed to serialize order")?;

        let response = self
            .client
            .post(url.clone())
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .wrap_err_with(|| format!("Failed to send POST request to URL: {url}"))?;

        let status = response.status();
        let body_text = response.text().await.wrap_err("Failed to extract response body text")?;

        if !status.is_success() {
            return Err(eyre::eyre!("HTTP Error {}: {}", status, body_text));
        }

        Ok(status)
    }

    pub async fn cancel_order(
        &self,
        order_cancellations: &OrderCancellations,
    ) -> Result<reqwest::StatusCode, Error> {
        let url = self.api_url.orders()?;

        let body = serde_json::to_string(order_cancellations)
            .wrap_err("Failed to serialize order cancellations")?;

        let response = self
            .client
            .delete(url.clone())
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .wrap_err_with(|| format!("Failed to send DELETE request to URL: {url}"))?;

        let status = response.status();
        let body_text = response.text().await.wrap_err("Failed to extract response body text")?;

        if !status.is_success() {
            return Err(eyre::eyre!("HTTP Error {}: {}", status, body_text));
        }

        Ok(status)
    }

    pub async fn get_user_orders(&self, address: &Address) -> Result<Vec<Order>, Error> {
        let url = self.api_url.get_user_orders(address.to_string().as_str())?;
        let body = self.get_response_body(&url).await?;
        let json: Vec<Order> = parse_response(&body)?;
        Ok(json)
    }

    pub async fn get_quote(&self, partial_order: &PartialOrder) -> Result<QuoteResponse, Error> {
        let url = self.api_url.quote()?;
        let body =
            serde_json::to_string(partial_order).wrap_err("Failed to serialize partial order")?;

        let response = self
            .client
            .post(url.clone())
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .wrap_err_with(|| format!("Failed to send POST request to URL: {url}"))?;

        let status = response.status();
        let body_text = response.text().await.wrap_err("Failed to extract response body text")?;

        if !status.is_success() {
            return Err(eyre::eyre!("HTTP Error {}: {}", status, body_text));
        }

        let json: QuoteResponse = parse_response(&body_text)?;
        Ok(json)
    }

    pub async fn get_trades(&self, query: &GetTradesQuery) -> Result<Vec<Trade>, Error> {
        let url = self.api_url.get_trades(query)?;
        println!("url: {}", url);
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

    pub async fn upload_app_data(&self, app_data: &AppData) -> Result<AppDataHash, Error> {
        let url = self.api_url.put_app_data()?;
        let body = serde_json::to_string(&app_data).wrap_err("Failed to serialize app data")?;

        let response = self
            .client
            .put(url.clone())
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .wrap_err_with(|| format!("Failed to send PUT request to URL: {url}"))?;

        let status = response.status();
        let body_text = response.text().await.wrap_err("Failed to extract response body text")?;

        if !status.is_success() {
            return Err(eyre::eyre!("HTTP Error {}: {}", status, body_text));
        }

        let returned_hash: AppDataHash = parse_response(&body_text)?;
        Ok(returned_hash)
    }

    pub async fn upload_app_data_by_hash(
        &self,
        app_data_hash: &AppDataHash,
        app_data: &AppData,
    ) -> Result<AppDataHash, Error> {
        let app_data_hash_str = hex::encode(app_data_hash.0);
        let url = self.api_url.put_app_data_by_hash(app_data_hash_str.as_str())?;
        let body = serde_json::to_string(&app_data).wrap_err("Failed to serialize app data")?;

        let response = self
            .client
            .put(url.clone())
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .wrap_err_with(|| format!("Failed to send PUT request to URL: {url}"))?;

        let status = response.status();
        let body_text = response.text().await.wrap_err("Failed to extract response body text")?;

        if !status.is_success() {
            return Err(eyre::eyre!("HTTP Error {}: {}", status, body_text));
        }

        let returned_hash: AppDataHash = parse_response(&body_text)?;
        Ok(returned_hash)
    }
}
