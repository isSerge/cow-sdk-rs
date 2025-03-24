#[derive(Debug)]
pub struct OrderApiUrl {
    base_url: String,
}

impl OrderApiUrl {
    pub fn new(base_url: &str) -> Self {
        Self { base_url: base_url.to_string() }
    }

    pub fn orders(&self) -> String {
        format!("{}//api/v1/orders", self.base_url)
    }

    pub fn get_order_by_id(&self, order_id: &str) -> String {
        format!("{}//api/v1/orders/{}", self.base_url, order_id)
    }

    pub fn get_order_status(&self, order_id: &str) -> String {
        format!("{}//api/v1/orders/{}/status", self.base_url, order_id)
    }

    pub fn get_order_by_tx_hash(&self, tx_hash: &str) -> String {
        format!("{}//api/v1/transactions/{}/orders", self.base_url, tx_hash)
    }

    pub fn get_trades(&self) -> String {
        format!("{}//api/v1/trades", self.base_url)
    }

    pub fn get_auction(&self) -> String {
        format!("{}//api/v1/auction", self.base_url)
    }

    pub fn get_user_orders(&self, account: &str) -> String {
        format!("{}//api/v1/account/{account}/orders", self.base_url)
    }

    pub fn get_native_price(&self, token_address: &str) -> String {
        format!("{}//api/v1/token/{}/native_price", self.base_url, token_address)
    }

    pub fn quote(&self) -> String {
        format!("{}//api/v1/quote", self.base_url)
    }

    pub fn get_solver_competition_by_id(&self, auction_id: &str) -> String {
        format!("{}//api/v1/solver_competition/{}", self.base_url, auction_id)
    }

    pub fn get_solver_competition_by_tx_hash(&self, tx_hash: &str) -> String {
        format!("{}//api/v1/solver_competition/by_tx_hash/{}", self.base_url, tx_hash)
    }

    pub fn get_solver_competition_latest(&self) -> String {
        format!("{}//api/v1/solver_competition/latest", self.base_url)
    }

    pub fn get_api_version(&self) -> String {
        format!("{}//api/v1/version", self.base_url)
    }

    pub fn app_data_by_hash(&self, app_data_hash: &str) -> String {
        format!("{}//api/v1/app_data/{}", self.base_url, app_data_hash)
    }

    pub fn get_user_surplus(&self, account: &str) -> String {
        format!("{}//api/v1/users/{}/total_surplus", self.base_url, account)
    }
}
