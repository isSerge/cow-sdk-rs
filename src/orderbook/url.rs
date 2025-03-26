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

    pub fn path(mut self, path: &str) -> Self {
        self.path = path.to_string();
        self
    }

    pub fn query(mut self, query: &str) -> Self {
        if !query.is_empty() {
            self.query = Some(query.to_string());
        }
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
        let url = RequestBuilder::new().path("/api/v1/orders").build(&self.base_url)?;
        Ok(url.to_string())
    }

    pub fn get_order_by_id(&self, order_id: &str) -> Result<String> {
        let url = RequestBuilder::new()
            .path(&format!("/api/v1/orders/{}", order_id))
            .build(&self.base_url)?;
        Ok(url.to_string())
    }

    pub fn get_order_status(&self, order_id: &str) -> Result<String> {
        let url = RequestBuilder::new()
            .path(&format!("/api/v1/orders/{}/status", order_id))
            .build(&self.base_url)?;
        Ok(url.to_string())
    }

    pub fn get_order_by_tx_hash(&self, tx_hash: &str) -> Result<String> {
        let url = RequestBuilder::new()
            .path(&format!("/api/v1/transactions/{}/orders", tx_hash))
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

        let url =
            RequestBuilder::new().path("/api/v1/trades").query(&query).build(&self.base_url)?;
        Ok(url.to_string())
    }

    pub fn get_auction(&self) -> Result<String> {
        let url = RequestBuilder::new().path("/api/v1/auction").build(&self.base_url)?;
        Ok(url.to_string())
    }

    pub fn get_user_orders(
        &self,
        account: &str,
        offset: Option<u32>,
        limit: Option<u32>,
    ) -> Result<String> {
        let query = match (offset, limit) {
            (Some(offset), Some(limit)) => format!("offset={}&limit={}", offset, limit),
            (Some(offset), None) => format!("offset={}", offset),
            (None, Some(limit)) => format!("limit={}", limit),
            (None, None) => String::new(),
        };
        let url = RequestBuilder::new()
            .path(&format!("/api/v1/account/{account}/orders"))
            .query(&query)
            .build(&self.base_url)?;
        Ok(url.to_string())
    }

    pub fn get_native_price(&self, token_address: &str) -> Result<String> {
        let url = RequestBuilder::new()
            .path(&format!("/api/v1/token/{}/native_price", token_address))
            .build(&self.base_url)?;
        Ok(url.to_string())
    }

    pub fn quote(&self) -> Result<String> {
        let url = RequestBuilder::new().path("/api/v1/quote").build(&self.base_url)?;
        Ok(url.to_string())
    }

    pub fn get_solver_competition_by_id(&self, auction_id: &str) -> Result<String> {
        let url = RequestBuilder::new()
            .path(&format!("/api/v1/solver_competition/{}", auction_id))
            .build(&self.base_url)?;
        Ok(url.to_string())
    }

    pub fn get_solver_competition_by_tx_hash(&self, tx_hash: &str) -> Result<String> {
        let url = RequestBuilder::new()
            .path(&format!("/api/v1/solver_competition/by_tx_hash/{}", tx_hash))
            .build(&self.base_url)?;
        Ok(url.to_string())
    }

    pub fn get_solver_competition_latest(&self) -> Result<String> {
        let url = RequestBuilder::new()
            .path("/api/v1/solver_competition/latest")
            .build(&self.base_url)?;
        Ok(url.to_string())
    }

    pub fn get_api_version(&self) -> Result<String> {
        let url = RequestBuilder::new().path("/api/v1/version").build(&self.base_url)?;
        Ok(url.to_string())
    }

    pub fn app_data_by_hash(&self, app_data_hash: &str) -> Result<String> {
        let url = RequestBuilder::new()
            .path(&format!("/api/v1/app_data/{}", app_data_hash))
            .build(&self.base_url)?;
        Ok(url.to_string())
    }

    pub fn put_app_data(&self) -> Result<String> {
        let url = RequestBuilder::new().path("/api/v1/app_data").build(&self.base_url)?;
        Ok(url.to_string())
    }

    pub fn get_user_surplus(&self, account: &str) -> Result<String> {
        let url = RequestBuilder::new()
            .path(&format!("/api/v1/users/{}/total_surplus", account))
            .build(&self.base_url)?;
        Ok(url.to_string())
    }
}

#[cfg(test)]
mod tests {
    use alloy::primitives::Address;

    use super::*;
    use crate::orderbook::{AppDataHash, OrderUid};

    const ORDER_ID: &str = "0xeaef82ff8696bff255e130b266231acb53a8f02823ed89b33acda5fd3987a53ad8da6bf26964af9d7eed9e03e53415d37aa96045676d56da";
    const TX_HASH: &str = "0xffd92faa1419c59ff0ac7f090998e9159f4b7f28bf67ad6b061c728c0da265f2";
    const ACCOUNT: &str = "0xd8da6bf26964af9d7eed9e03e53415d37aa96045";
    const BASE_URL: &str = "https://api.cow.fi/mainnet";
    const TOKEN_ADDRESS: &str = "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48";

    #[test]
    fn test_request_builder_can_add_path() {
        let base_url = Url::parse(BASE_URL).unwrap();
        let url = RequestBuilder::new().path("/api/v1/orders").build(&base_url);

        assert_eq!(url.unwrap().to_string(), "https://api.cow.fi/mainnet/api/v1/orders");
    }

    #[test]
    fn test_request_builder_can_add_query_params() {
        let base_url = Url::parse(BASE_URL).unwrap();
        let url = RequestBuilder::new()
            .path("/api/v1/orders")
            .query(&format!("owner={}", ACCOUNT))
            .build(&base_url);

        assert_eq!(
            url.unwrap().to_string(),
            "https://api.cow.fi/mainnet/api/v1/orders?owner=0xd8da6bf26964af9d7eed9e03e53415d37aa96045"
        );
    }

    #[test]
    #[should_panic(expected = "Cannot modify URL segments")]
    fn test_request_builder_invalid_base_url() {
        // URLs with non-hierarchical schemes (like "data:") cannot have their path
        // segments modified.
        let base_url = Url::parse("data:,").unwrap();
        RequestBuilder::new().path("/api/v1/orders").build(&base_url).unwrap();
    }

    #[test]
    fn test_order_api_url_can_build_get_order_by_id() {
        let url = OrderApiUrl::new(BASE_URL).unwrap();
        let url = url.get_order_by_id(ORDER_ID);
        assert_eq!(url.unwrap().to_string(), "https://api.cow.fi/mainnet/api/v1/orders/0xeaef82ff8696bff255e130b266231acb53a8f02823ed89b33acda5fd3987a53ad8da6bf26964af9d7eed9e03e53415d37aa96045676d56da");
    }

    #[test]
    fn test_order_api_url_can_build_get_order_status() {
        let url = OrderApiUrl::new(BASE_URL).unwrap();
        let url = url.get_order_status(ORDER_ID);
        assert_eq!(url.unwrap().to_string(), "https://api.cow.fi/mainnet/api/v1/orders/0xeaef82ff8696bff255e130b266231acb53a8f02823ed89b33acda5fd3987a53ad8da6bf26964af9d7eed9e03e53415d37aa96045676d56da/status");
    }

    #[test]
    fn test_order_api_url_can_build_get_order_by_tx_hash() {
        let url = OrderApiUrl::new(BASE_URL).unwrap();
        let url = url.get_order_by_tx_hash(TX_HASH);
        assert_eq!(
            url.unwrap().to_string(),
            "https://api.cow.fi/mainnet/api/v1/transactions/0xffd92faa1419c59ff0ac7f090998e9159f4b7f28bf67ad6b061c728c0da265f2/orders"
        );
    }

    #[test]
    fn test_order_api_url_can_build_get_trades_by_owner() {
        let address: Address = ACCOUNT.parse().unwrap();
        let url = OrderApiUrl::new(BASE_URL).unwrap();
        let url = url.get_trades(&GetTradesQuery::ByOwner(address));
        assert_eq!(
            url.unwrap().to_string().to_lowercase(),
            "https://api.cow.fi/mainnet/api/v1/trades?owner=0xd8da6bf26964af9d7eed9e03e53415d37aa96045"
        );
    }

    #[test]
    fn test_order_api_url_can_build_get_trades_by_order_id() {
        let order_uid: OrderUid = ORDER_ID.parse().unwrap();
        let api_url = OrderApiUrl::new(BASE_URL).unwrap();
        let url = api_url.get_trades(&GetTradesQuery::ByOrderId(order_uid));

        assert_eq!(
            url.unwrap().to_string(),
            "https://api.cow.fi/mainnet/api/v1/trades?orderUid=0xeaef82ff8696bff255e130b266231acb53a8f02823ed89b33acda5fd3987a53ad8da6bf26964af9d7eed9e03e53415d37aa96045676d56da"
        );
    }

    #[test]
    fn test_order_api_url_can_build_get_user_orders() {
        let address: Address = ACCOUNT.parse().unwrap();
        let api_url = OrderApiUrl::new(BASE_URL).unwrap();
        let url = api_url.get_user_orders(&address.to_string(), None, None);
        assert_eq!(url.unwrap().to_string().to_lowercase(), "https://api.cow.fi/mainnet/api/v1/account/0xd8da6bf26964af9d7eed9e03e53415d37aa96045/orders");
    }

    #[test]
    fn test_order_api_url_can_build_get_user_orders_with_offset() {
        let address: Address = ACCOUNT.parse().unwrap();
        let api_url = OrderApiUrl::new(BASE_URL).unwrap();
        let url = api_url.get_user_orders(&address.to_string(), Some(10), None);
        assert_eq!(url.unwrap().to_string().to_lowercase(), "https://api.cow.fi/mainnet/api/v1/account/0xd8da6bf26964af9d7eed9e03e53415d37aa96045/orders?offset=10");
    }

    #[test]
    fn test_order_api_url_can_build_get_user_orders_with_limit() {
        let address: Address = ACCOUNT.parse().unwrap();
        let api_url = OrderApiUrl::new(BASE_URL).unwrap();
        let url = api_url.get_user_orders(&address.to_string(), None, Some(10));
        assert_eq!(url.unwrap().to_string().to_lowercase(), "https://api.cow.fi/mainnet/api/v1/account/0xd8da6bf26964af9d7eed9e03e53415d37aa96045/orders?limit=10");
    }

    #[test]
    fn test_order_api_url_can_build_get_native_price() {
        let token_address: Address = TOKEN_ADDRESS.parse().unwrap();
        let api_url = OrderApiUrl::new(BASE_URL).unwrap();
        let url = api_url.get_native_price(&token_address.to_string());
        assert_eq!(url.unwrap().to_string().to_lowercase(), "https://api.cow.fi/mainnet/api/v1/token/0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48/native_price");
    }

    #[test]
    fn test_order_api_url_can_build_get_solver_competition_by_id() {
        let api_url = OrderApiUrl::new(BASE_URL).unwrap();
        let url = api_url.get_solver_competition_by_id("1");
        assert_eq!(
            url.unwrap().to_string().to_lowercase(),
            "https://api.cow.fi/mainnet/api/v1/solver_competition/1"
        );
    }

    #[test]
    fn test_order_api_url_can_build_get_solver_competition_by_tx_hash() {
        let api_url = OrderApiUrl::new(BASE_URL).unwrap();
        let url = api_url.get_solver_competition_by_tx_hash(TX_HASH);
        assert_eq!(url.unwrap().to_string().to_lowercase(), "https://api.cow.fi/mainnet/api/v1/solver_competition/by_tx_hash/0xffd92faa1419c59ff0ac7f090998e9159f4b7f28bf67ad6b061c728c0da265f2");
    }

    #[test]
    fn test_order_api_url_can_build_get_solver_competition_latest() {
        let api_url = OrderApiUrl::new(BASE_URL).unwrap();
        let url = api_url.get_solver_competition_latest();
        assert_eq!(
            url.unwrap().to_string().to_lowercase(),
            "https://api.cow.fi/mainnet/api/v1/solver_competition/latest"
        );
    }

    #[test]
    fn test_order_api_url_can_build_get_api_version() {
        let api_url = OrderApiUrl::new(BASE_URL).unwrap();
        let url = api_url.get_api_version();
        assert_eq!(
            url.unwrap().to_string().to_lowercase(),
            "https://api.cow.fi/mainnet/api/v1/version"
        );
    }

    #[test]
    fn test_order_api_url_can_build_app_data_by_hash() {
        let api_url = OrderApiUrl::new(BASE_URL).unwrap();

        let app_data_hash: AppDataHash =
            "0x00e421be3c3b0e20c582c0d803018c418b56ea61add1811bec2509e003a17b42".parse().unwrap();
        let url = api_url.app_data_by_hash(&app_data_hash.to_string());
        assert_eq!(
            url.unwrap().to_string().to_lowercase(),
            "https://api.cow.fi/mainnet/api/v1/app_data/0x00e421be3c3b0e20c582c0d803018c418b56ea61add1811bec2509e003a17b42"
        );
    }
}
