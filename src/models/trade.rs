use alloy::primitives::{Address, TxHash, U256};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::primitives::order_uid::OrderUid;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub block_number: u64,
    pub order_uid: OrderUid,
    pub log_index: u64,
    pub sell_token: Address,
    pub buy_token: Address,
    pub sell_amount: U256,
    pub sell_amount_before_fees: U256,
    pub buy_amount: U256,
    pub tx_hash: TxHash,
    pub executed_protocol_fees: Vec<Value>, // TODO: create a type for this
}
