use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Order {
    buy_token: String,
    sell_token: String,
    buy_amount: String,
    sell_amount: String,
    receiver: String,
    valid_to: String,
    app_data: String,
    fee_amount: String,
    kind: String,
    partial_fillable: bool,
    sell_token_balance: String,
    buy_token_balance: String,
    signing_scheme: String,
    signature: String,
    from: String,
}

impl Default for Order {
    fn default() -> Self {
        Self::new()
    }
}

impl Order {
    pub fn new() -> Self {
        Self {
            buy_token: "".to_string(),
            sell_token: "".to_string(),
            buy_amount: "".to_string(),
            sell_amount: "".to_string(),
            receiver: "".to_string(),
            valid_to: "".to_string(),
            app_data: "".to_string(),
            fee_amount: "".to_string(),
            kind: "".to_string(),
            partial_fillable: false,
            sell_token_balance: "".to_string(),
            buy_token_balance: "".to_string(),
            signing_scheme: "".to_string(),
            signature: "".to_string(),
            from: "".to_string(),
        }
    }
}
