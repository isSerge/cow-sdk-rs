use serde::Serialize;
use serde_urlencoded;
use url::Url;

use crate::orderbook_api::GetTradesQuery;

#[derive(Serialize)]
struct TradesQueryParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "orderUid")]
    order_uid: Option<String>,
}

#[derive(Debug)]
pub struct OrderApiUrl {
    base_url: Url,
}

impl OrderApiUrl {
    pub fn new(base_url: &str) -> Self {
        let base_url = Url::parse(base_url).unwrap();
        Self { base_url }
    }

    /// Builds a URL from a path and optional parameters.
    fn build<T: Serialize>(
        &self,
        path: &str,
        params: Option<&T>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut url = self.base_url.clone();
        url.path_segments_mut()
            .map_err(|_| "Cannot modify URL segments")?
            .extend(path.split('/').filter(|s| !s.is_empty()));

        if let Some(params) = params {
            let query = serde_urlencoded::to_string(params)?;
            url.set_query(Some(&query));
        }
        Ok(url.to_string())
    }

    pub fn orders(&self) -> String {
        self.build::<()>("/api/v1/orders", None).unwrap()
    }

    pub fn get_order_by_id(&self, order_id: &str) -> String {
        self.build::<()>(&format!("/api/v1/orders/{}", order_id), None).unwrap()
    }

    pub fn get_order_status(&self, order_id: &str) -> String {
        self.build::<()>(&format!("/api/v1/orders/{}/status", order_id), None).unwrap()
    }

    pub fn get_order_by_tx_hash(&self, tx_hash: &str) -> String {
        self.build::<()>(&format!("/api/v1/transactions/{}/orders", tx_hash), None).unwrap()
    }

    pub fn get_trades(&self, query: &GetTradesQuery) -> String {
        let params = match query {
            GetTradesQuery::ByOwner(owner) =>
                TradesQueryParams { owner: Some(owner.to_string()), order_uid: None },
            GetTradesQuery::ByOrderId(order_id) =>
                TradesQueryParams { owner: None, order_uid: Some(order_id.to_string()) },
        };

        self.build("/api/v1/trades", Some(&params)).unwrap()
    }

    pub fn get_auction(&self) -> String {
        self.build::<()>("/api/v1/auction", None).unwrap()
    }

    pub fn get_user_orders(&self, account: &str) -> String {
        self.build::<()>(&format!("/api/v1/account/{account}/orders"), None).unwrap()
    }

    pub fn get_native_price(&self, token_address: &str) -> String {
        self.build::<()>(&format!("/api/v1/token/{}/native_price", token_address), None).unwrap()
    }

    pub fn quote(&self) -> String {
        self.build::<()>("/api/v1/quote", None).unwrap()
    }

    pub fn get_solver_competition_by_id(&self, auction_id: &str) -> String {
        self.build::<()>(&format!("/api/v1/solver_competition/{}", auction_id), None).unwrap()
    }

    pub fn get_solver_competition_by_tx_hash(&self, tx_hash: &str) -> String {
        self.build::<()>(&format!("/api/v1/solver_competition/by_tx_hash/{}", tx_hash), None)
            .unwrap()
    }

    pub fn get_solver_competition_latest(&self) -> String {
        self.build::<()>("/api/v1/solver_competition/latest", None).unwrap()
    }

    pub fn get_api_version(&self) -> String {
        self.build::<()>("/api/v1/version", None).unwrap()
    }

    pub fn app_data_by_hash(&self, app_data_hash: &str) -> String {
        self.build::<()>(&format!("/api/v1/app_data/{}", app_data_hash), None).unwrap()
    }

    pub fn get_user_surplus(&self, account: &str) -> String {
        self.build::<()>(&format!("/api/v1/users/{}/total_surplus", account), None).unwrap()
    }
}
