use eyre::{Result, WrapErr};
use serde::Serialize;
use serde_urlencoded;
use url::Url;

use crate::orderbook::GetTradesQuery;

#[derive(Debug, Default)]
struct RequestBuilder {
    path: String,
    query: Option<String>,
}

impl RequestBuilder {
    pub fn new() -> Self {
        Self { path: String::new(), query: None }
    }

    pub fn with_path(mut self, path: &str) -> Self {
        self.path = path.to_string();
        self
    }

    pub fn with_query(mut self, query: &str) -> Self {
        self.query = Some(query.to_string());
        self
    }

    pub fn build(self, base_url: &Url) -> Result<Url> {
        let mut url = base_url.clone();
        url.path_segments_mut()
            .map_err(|_| eyre::eyre!("Cannot modify URL segments"))
            .wrap_err("Failed to set URL segments")?
            .extend(self.path.split('/').filter(|s| !s.is_empty()));

        if let Some(query) = self.query {
            url.set_query(Some(&query));
        }
        Ok(url)
    }

    /// Serializes the query parameters.
    fn serialize_query<T: Serialize>(params: &T) -> Result<String, serde_urlencoded::ser::Error> {
        serde_urlencoded::to_string(params)
    }
}

#[derive(Debug)]
pub struct OrderApiUrl {
    base_url: Url,
}

impl OrderApiUrl {
    pub fn new(base_url: &str) -> Result<Self> {
        let base_url = Url::parse(base_url).wrap_err("Invalid base URL")?;
        Ok(Self { base_url })
    }

    pub fn orders(&self) -> Result<String> {
        let url = RequestBuilder::new().with_path("/api/v1/orders").build(&self.base_url)?;
        Ok(url.to_string())
    }

    pub fn get_order_by_id(&self, order_id: &str) -> Result<String> {
        let url = RequestBuilder::new()
            .with_path(&format!("/api/v1/orders/{}", order_id))
            .build(&self.base_url)?;
        Ok(url.to_string())
    }

    pub fn get_order_status(&self, order_id: &str) -> Result<String> {
        let url = RequestBuilder::new()
            .with_path(&format!("/api/v1/orders/{}/status", order_id))
            .build(&self.base_url)?;
        Ok(url.to_string())
    }

    pub fn get_order_by_tx_hash(&self, tx_hash: &str) -> Result<String> {
        let url = RequestBuilder::new()
            .with_path(&format!("/api/v1/transactions/{}/orders", tx_hash))
            .build(&self.base_url)?;
        Ok(url.to_string())
    }

    pub fn get_trades(&self, query: &GetTradesQuery) -> Result<String> {
        // Convert our enum into key-value pairs
        #[derive(Serialize)]
        struct TradesQueryParams {
            #[serde(skip_serializing_if = "Option::is_none")]
            owner: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none", rename = "orderUid")]
            order_uid: Option<String>,
        }
        let params = match query {
            GetTradesQuery::ByOwner(owner) =>
                TradesQueryParams { owner: Some(owner.to_string()), order_uid: None },
            GetTradesQuery::ByOrderId(order_id) =>
                TradesQueryParams { owner: None, order_uid: Some(order_id.to_string()) },
        };

        let query = RequestBuilder::serialize_query(&params)?;

        let url = RequestBuilder::new()
            .with_path("/api/v1/trades")
            .with_query(&query)
            .build(&self.base_url)?;
        Ok(url.to_string())
    }

    pub fn get_auction(&self) -> Result<String> {
        let url = RequestBuilder::new().with_path("/api/v1/auction").build(&self.base_url)?;
        Ok(url.to_string())
    }

    pub fn get_user_orders(&self, account: &str) -> Result<String> {
        let url = RequestBuilder::new()
            .with_path(&format!("/api/v1/account/{account}/orders"))
            .build(&self.base_url)?;
        Ok(url.to_string())
    }

    pub fn get_native_price(&self, token_address: &str) -> Result<String> {
        let url = RequestBuilder::new()
            .with_path(&format!("/api/v1/token/{}/native_price", token_address))
            .build(&self.base_url)?;
        Ok(url.to_string())
    }

    pub fn quote(&self) -> Result<String> {
        let url = RequestBuilder::new().with_path("/api/v1/quote").build(&self.base_url)?;
        Ok(url.to_string())
    }

    pub fn get_solver_competition_by_id(&self, auction_id: &str) -> Result<String> {
        let url = RequestBuilder::new()
            .with_path(&format!("/api/v1/solver_competition/{}", auction_id))
            .build(&self.base_url)?;
        Ok(url.to_string())
    }

    pub fn get_solver_competition_by_tx_hash(&self, tx_hash: &str) -> Result<String> {
        let url = RequestBuilder::new()
            .with_path(&format!("/api/v1/solver_competition/by_tx_hash/{}", tx_hash))
            .build(&self.base_url)?;
        Ok(url.to_string())
    }

    pub fn get_solver_competition_latest(&self) -> Result<String> {
        let url = RequestBuilder::new()
            .with_path("/api/v1/solver_competition/latest")
            .build(&self.base_url)?;
        Ok(url.to_string())
    }

    pub fn get_api_version(&self) -> Result<String> {
        let url = RequestBuilder::new().with_path("/api/v1/version").build(&self.base_url)?;
        Ok(url.to_string())
    }

    pub fn app_data_by_hash(&self, app_data_hash: &str) -> Result<String> {
        let url = RequestBuilder::new()
            .with_path(&format!("/api/v1/app_data/{}", app_data_hash))
            .build(&self.base_url)?;
        Ok(url.to_string())
    }

    pub fn put_app_data_by_hash(&self, app_data_hash: &str) -> Result<String> {
        let url = RequestBuilder::new()
            .with_path(&format!("/api/v1/app_data/{}", app_data_hash))
            .build(&self.base_url)?;
        Ok(url.to_string())
    }

    pub fn put_app_data(&self) -> Result<String> {
        let url = RequestBuilder::new().with_path("/api/v1/app_data").build(&self.base_url)?;
        Ok(url.to_string())
    }

    pub fn get_user_surplus(&self, account: &str) -> Result<String> {
        let url = RequestBuilder::new()
            .with_path(&format!("/api/v1/users/{}/total_surplus", account))
            .build(&self.base_url)?;
        Ok(url.to_string())
    }
}
