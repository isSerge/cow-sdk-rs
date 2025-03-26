use alloy::primitives::{Address, TxHash, U256};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::order::{CompetitionOrderStatus, SolutionInclusion};

#[derive(Debug, Deserialize, Serialize)]
pub struct CompetitionOrderStatusResponse {
    pub r#type: CompetitionOrderStatus,
    pub value: Vec<SolutionInclusion>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenPriceResponse {
    pub price: f64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SolverCompetitionResponse {
    pub auction_id: i64,
    pub transaction_hashes: Vec<TxHash>,
    pub gap_price: Option<U256>,
    pub liquidity_collected_block: Option<u64>, // TODO: clarify if should be present
    pub competition_simulation_block: u64,
    pub auction_start_block: u64,
    pub auction: Option<Value>,        // TODO: add a type for this
    pub solutions: Option<Vec<Value>>, // TODO: add a type for this
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TotalSurplusResponse {
    pub total_surplus: U256,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppDataResponse {
    pub full_app_data: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteResponse {
    pub quote: Value,
    pub from: Address,
    pub expiration: String, // TODO: change to a DateTime type (e.g. chrono::DateTime)
    pub id: i64,
    pub verified: bool,
}
