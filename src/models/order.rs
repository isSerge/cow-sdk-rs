use alloy::primitives::{Address, U256};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::primitives::{app_data::AppDataHash, order_uid::OrderUid};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub app_data: String,
    pub available_balance: Option<U256>,
    pub buy_amount: U256,
    pub buy_token: Address,
    pub buy_token_balance: String,
    pub class: String,
    pub creation_date: DateTime<Utc>,
    pub executed_buy_amount: U256,
    pub executed_fee: U256,
    pub executed_fee_amount: U256,
    pub executed_fee_token: U256,
    pub executed_sell_amount: U256,
    pub executed_sell_amount_before_fees: U256,
    pub fee_amount: U256,
    pub full_app_data: String,
    pub interactions: Interactions,
    pub invalidated: bool,
    pub is_liquidity_order: bool,
    pub kind: String,
    pub owner: Address,
    pub partially_fillable: bool,
    pub quote: Option<Quote>,
    pub receiver: Address,
    pub sell_amount: U256,
    pub sell_token: Address,
    pub sell_token_balance: String,
    pub settlement_contract: String,
    pub signature: String,
    pub signing_scheme: String,
    pub status: OrderStatus,
    pub uid: OrderUid,
    pub valid_to: u64,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum CompetitionOrderStatus {
    Cancelled,
    Open,
    Schedules,
    Active,
    Solved,
    Executing,
    Traded,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SolutionInclusion {
    /// The name or identifier of the solver.
    pub solver: String,
    /// The executed amounts for the order as proposed by the solver, included
    /// if the solution was for the desired order, or omitted otherwise.
    pub executed_amounts: Option<ExecutedAmounts>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ExecutedAmounts {
    pub sell: U256,
    pub buy: U256,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum OrderStatus {
    Fulfilled,
    Expired,
    Cancelled,
    Open,
    PresignaturePending,
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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderCancellations {
    pub order_ids: Vec<OrderUid>,
    pub signature: String,
    pub signing_scheme: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PartialOrder {
    pub app_data: String,
    pub buy_amount: U256,
    pub buy_token: Address,
    pub sell_amount: U256,
    pub sell_token: Address,
    pub receiver: Address,
    pub app_data_hash: AppDataHash,
    pub sell_token_balance: U256,
    pub buy_token_balance: U256,
    pub from: Address,
    pub price_quality: String,
    pub signing_scheme: String,
    pub onchain_order: bool,
    pub kind: String,
    pub sell_amount_before_fees: U256,
}
