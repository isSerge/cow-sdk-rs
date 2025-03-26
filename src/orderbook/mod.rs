mod url;

use alloy::primitives::{Address, TxHash};
use eyre::{Error, Result, WrapErr};
use log::{debug, error, info, trace};
use reqwest::{Client, Method, Response};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};
use serde::de::DeserializeOwned;
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
    parsing::parse_response_body,
    primitives::{
        app_data::{AppData, AppDataHash},
        order_uid::OrderUid,
    },
};

/// Client for the Order API.
#[derive(Debug)]
pub struct OrderApiClient {
    client: ClientWithMiddleware,
    api_url: OrderApiUrl,
}

/// Query to get trades by owner or order ID.
#[derive(Debug)]
pub enum GetTradesQuery {
    ByOwner(Address),
    ByOrderId(OrderUid),
}

impl OrderApiClient {
    pub fn new(network: Network) -> Result<Self> {
        info!("Creating new OrderApiClient for network: {:?}", network);
        let api_url = OrderApiUrl::new(network.api_url())?;
        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
        let client = ClientBuilder::new(Client::new())
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build();
        Ok(Self { client, api_url })
    }

    /// Helper function to send a request to the Order API.
    async fn send_request(
        &self,
        url: &str,
        method: Method,
        body: Option<String>,
    ) -> Result<Response, Error> {
        trace!("Sending request to {} with method {}", url, method);
        let mut request = self.client.request(method, url);

        if let Some(body) = body {
            debug!("Request body: {}", body);
            request = request.header("Content-Type", "application/json").body(body);
        }

        let response = request
            .send()
            .await
            .wrap_err_with(|| format!("Failed to send request to URL: {url}"))?;

        debug!("Received response: {:?}", response.status());
        Ok(response)
    }

    /// Helper function to handle a response from the Order API.
    async fn handle_response<T: DeserializeOwned>(&self, response: Response) -> Result<T, Error> {
        let status = response.status();
        let body_text = response.text().await.wrap_err("Failed to extract response body text")?;

        if !status.is_success() {
            error!("HTTP Error {}: {}", status, body_text);
            return Err(eyre::eyre!("HTTP Error {}: {}", status, body_text));
        }

        trace!("Response body: {}", body_text);
        let json: T = parse_response_body(&body_text)?;
        Ok(json)
    }

    /// Get an order by its ID.
    pub async fn get_order_by_id(&self, order_id: &OrderUid) -> Result<Order, Error> {
        let url = self.api_url.get_order_by_id(order_id.to_string().as_str())?;
        let response = self.send_request(&url, Method::GET, None).await?;
        self.handle_response(response).await
    }

    /// Get orders by transaction hash.
    pub async fn get_orders_by_tx_hash(&self, tx_hash: &TxHash) -> Result<Vec<Order>, Error> {
        let url = self.api_url.get_order_by_tx_hash(tx_hash.to_string().as_str())?;
        let response = self.send_request(&url, Method::GET, None).await?;
        self.handle_response(response).await
    }

    /// Get order status by order ID.
    pub async fn get_order_status(
        &self,
        order_id: &OrderUid,
    ) -> Result<CompetitionOrderStatusResponse, Error> {
        let url = self.api_url.get_order_status(order_id.to_string().as_str())?;
        let response = self.send_request(&url, Method::GET, None).await?;
        self.handle_response(response).await
    }

    /// Create an order.
    pub async fn create_order(&self, order: &Order) -> Result<(), Error> {
        let url = self.api_url.orders()?;
        let body = serde_json::to_string(order).wrap_err("Failed to serialize order")?;

        let response = self.send_request(&url, Method::POST, Some(body)).await?;
        self.handle_response(response).await
    }

    /// Cancel an order.
    pub async fn cancel_order(
        &self,
        order_cancellations: &OrderCancellations,
    ) -> Result<(), Error> {
        let url = self.api_url.orders()?;

        let body = serde_json::to_string(order_cancellations)
            .wrap_err("Failed to serialize order cancellations")?;

        let response = self.send_request(&url, Method::DELETE, Some(body)).await?;
        self.handle_response(response).await
    }

    /// Get orders by account.
    pub async fn get_user_orders(
        &self,
        address: &Address,
        offset: Option<u32>,
        limit: Option<u32>,
    ) -> Result<Vec<Order>, Error> {
        let url = self.api_url.get_user_orders(address.to_string().as_str(), offset, limit)?;
        let response = self.send_request(&url, Method::GET, None).await?;
        self.handle_response(response).await
    }

    /// Get a quote for an order.
    pub async fn get_quote(&self, partial_order: &PartialOrder) -> Result<QuoteResponse, Error> {
        let url = self.api_url.quote()?;
        let body =
            serde_json::to_string(partial_order).wrap_err("Failed to serialize partial order")?;

        let response = self.send_request(&url, Method::POST, Some(body)).await?;
        self.handle_response(response).await
    }

    /// Get trades by owner or order ID.
    pub async fn get_trades(&self, query: &GetTradesQuery) -> Result<Vec<Trade>, Error> {
        let url = self.api_url.get_trades(query)?;
        let response = self.send_request(&url, Method::GET, None).await?;
        self.handle_response(response).await
    }

    /// Get the current batch auction. Permissioned endpoint.
    // TODO: get permission and implement struct
    pub async fn get_auction(&self) -> Result<Value, Error> {
        let url = self.api_url.get_auction()?;
        let response = self.send_request(&url, Method::GET, None).await?;
        self.handle_response(response).await
    }

    /// Get a solver competition by ID
    pub async fn get_competition_by_id(
        &self,
        auction_id: &i64,
    ) -> Result<SolverCompetitionResponse, Error> {
        let url = self.api_url.get_solver_competition_by_id(auction_id.to_string().as_str())?;
        let response = self.send_request(&url, Method::GET, None).await?;
        self.handle_response(response).await
    }

    /// Get a solver competition by transaction hash
    pub async fn get_competition_by_tx_hash(
        &self,
        tx_hash: &TxHash,
    ) -> Result<SolverCompetitionResponse, Error> {
        let url = self.api_url.get_solver_competition_by_tx_hash(tx_hash.to_string().as_str())?;
        let response = self.send_request(&url, Method::GET, None).await?;
        self.handle_response(response).await
    }

    /// Get the latest solver competition.
    pub async fn get_latest_competition(&self) -> Result<SolverCompetitionResponse, Error> {
        let url = self.api_url.get_solver_competition_latest()?;
        let response = self.send_request(&url, Method::GET, None).await?;
        self.handle_response(response).await
    }

    /// Get the native price of a token.
    pub async fn get_token_price(
        &self,
        token_address: &Address,
    ) -> Result<TokenPriceResponse, Error> {
        let url = self.api_url.get_native_price(token_address.to_string().as_str())?;
        let response = self.send_request(&url, Method::GET, None).await?;
        self.handle_response(response).await
    }

    /// Get the API version.
    pub async fn get_version(&self) -> Result<String, Error> {
        let url = self.api_url.get_api_version()?;
        let response = self.send_request(&url, Method::GET, None).await?;

        Ok(response.text().await?)
    }

    /// Get the total surplus of a user. [UNSTABLE]
    pub async fn get_total_surplus(
        &self,
        address: &Address,
    ) -> Result<TotalSurplusResponse, Error> {
        let url = self.api_url.get_user_surplus(address.to_string().as_str())?;
        let response = self.send_request(&url, Method::GET, None).await?;
        self.handle_response(response).await
    }

    /// Get app data by hash.
    pub async fn get_app_data(
        &self,
        app_data_hash: &AppDataHash,
    ) -> Result<AppDataResponse, Error> {
        let app_data_hash_str = app_data_hash.to_string();
        let url = self.api_url.app_data_by_hash(app_data_hash_str.as_str())?;
        let response = self.send_request(&url, Method::GET, None).await?;
        self.handle_response(response).await
    }

    /// Upload app data.
    pub async fn upload_app_data(&self, app_data: &AppData) -> Result<AppDataHash, Error> {
        let url = self.api_url.put_app_data()?;
        let body = serde_json::to_string(&app_data).wrap_err("Failed to serialize app data")?;

        let response = self.send_request(&url, Method::PUT, Some(body)).await?;

        self.handle_response(response).await
    }

    /// Upload app data by hash.
    pub async fn upload_app_data_by_hash(
        &self,
        app_data_hash: &AppDataHash,
        app_data: &AppData,
    ) -> Result<AppDataHash, Error> {
        let app_data_hash_str = app_data_hash.to_string();
        let url = self.api_url.app_data_by_hash(app_data_hash_str.as_str())?;
        let body = serde_json::to_string(&app_data).wrap_err("Failed to serialize app data")?;

        let response = self.send_request(&url, Method::PUT, Some(body)).await?;

        self.handle_response(response).await
    }
}
