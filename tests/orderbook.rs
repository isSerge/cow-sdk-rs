use alloy::primitives::{Address, TxHash};
use cow_sdk::{
    config::network::Network,
    models::order::CompetitionOrderStatus,
    orderbook_api::{GetTradesQuery, OrderApiClient},
    primitives::order_uid::OrderUid,
};
use eyre::Result;

const ORDER_ID: &str = "0xeaef82ff8696bff255e130b266231acb53a8f02823ed89b33acda5fd3987a53ad8da6bf26964af9d7eed9e03e53415d37aa96045676d56da";
const TX_HASH: &str = "0xffd92faa1419c59ff0ac7f090998e9159f4b7f28bf67ad6b061c728c0da265f2";

#[tokio::test]
#[ignore]
async fn test_get_order_by_id() -> Result<()> {
    let client = OrderApiClient::new(Network::Mainnet)?;
    let order_id: OrderUid = ORDER_ID.parse()?;
    let order = client.get_order_by_id(&order_id).await?;

    assert_eq!(order.uid, order_id);

    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_get_order_by_tx_hash() -> Result<()> {
    let client = OrderApiClient::new(Network::Mainnet)?;
    let tx_hash: TxHash = TX_HASH.parse()?;

    let orders = client.get_orders_by_tx_hash(&tx_hash).await?;

    assert_eq!(orders.len(), 1);
    assert_eq!(orders[0].uid, OrderUid::new(ORDER_ID.parse()?));

    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_get_order_status_by_id() -> Result<()> {
    let client = OrderApiClient::new(Network::Mainnet)?;
    let order_id: OrderUid = ORDER_ID.parse()?;

    let response = client.get_order_status(&order_id).await?;

    assert_eq!(response.r#type, CompetitionOrderStatus::Traded);

    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_get_user_orders() -> Result<()> {
    let client = OrderApiClient::new(Network::Mainnet)?;
    let address: Address = "0xd8da6bf26964af9d7eed9e03e53415d37aa96045".parse()?;

    let response = client.get_user_orders(&address).await?;

    assert_eq!(response.len(), 10);

    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_get_trades_by_owner() -> Result<()> {
    let client = OrderApiClient::new(Network::Mainnet)?;
    let address: Address = "0xd8da6bf26964af9d7eed9e03e53415d37aa96045".parse()?;

    let trades = client.get_trades(GetTradesQuery::ByOwner(address)).await?;

    assert_eq!(trades.len(), 36);

    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_get_trades_by_order_id() -> Result<()> {
    let client = OrderApiClient::new(Network::Mainnet)?;
    let order_id: OrderUid = ORDER_ID.parse()?;

    let trades = client.get_trades(GetTradesQuery::ByOrderId(order_id)).await?;

    assert_eq!(trades.len(), 1);
    assert_eq!(trades[0].order_uid, order_id);

    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_get_token_price() -> Result<()> {
    let client = OrderApiClient::new(Network::Mainnet)?;
    let token_address: Address = "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48".parse()?;

    let response = client.get_token_price(&token_address).await?;

    assert!(response.price > 0.0);

    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_get_competition_by_tx_hash() -> Result<()> {
    let client = OrderApiClient::new(Network::Mainnet)?;
    let tx_hash: TxHash = TX_HASH.parse()?;

    let response = client.get_competition_by_tx_hash(&tx_hash).await?;

    assert_eq!(response.transaction_hashes.len(), 1);
    assert_eq!(response.transaction_hashes[0], tx_hash);

    Ok(())
}
