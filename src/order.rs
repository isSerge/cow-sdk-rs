use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    // basic order fields
    pub app_data: String,
    pub available_balance: Option<String>,
    pub buy_amount: String,
    pub buy_token: String,
    pub buy_token_balance: String,
    pub class: String,
    pub creation_date: String, // TODO: change this to a DateTime type (e.g. chrono::DateTime)
    pub executed_buy_amount: String,
    pub executed_fee: String,
    pub executed_fee_amount: String,
    pub executed_fee_token: String,
    pub executed_sell_amount: String,
    pub executed_sell_amount_before_fees: String,
    pub fee_amount: String,
    pub full_app_data: String,
    pub interactions: Interactions,
    pub invalidated: bool,
    pub is_liquidity_order: bool,
    pub kind: String,
    pub owner: String,
    pub partially_fillable: bool,
    pub quote: Option<Quote>,
    pub receiver: String,
    pub sell_amount: String,
    pub sell_token: String,
    pub sell_token_balance: String,
    pub settlement_contract: String,
    pub signature: String,
    pub signing_scheme: String,
    pub status: String,
    pub uid: String,
    pub valid_to: u64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Interactions {
    pub post: Vec<Interaction>,
    pub pre: Vec<Interaction>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Interaction {
    pub call_data: String,
    pub target: String,
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    pub buy_amount: String,
    pub gas_amount: String,
    pub gas_price: String,
    pub metadata: QuoteMetadata,
    pub sell_amount: String,
    pub sell_token_price: String,
    pub solver: String,
    pub verified: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteMetadata {
    pub interactions: Vec<Interaction>,
    pub jit_orders: Vec<Value>, // if these are dynamic, you can use serde_json::Value
    pub pre_interactions: Vec<Value>, // likewise here
    pub version: String,
}
