use cow_sdk::{config::network::Network, orderbook_api::OrderApiClient};
use eyre::Result;

#[tokio::test]
#[ignore]
async fn test_get_order_by_id() -> Result<()> {
    let client = OrderApiClient::new(Network::Mainnet)?;
    let order_id = "0xeaef82ff8696bff255e130b266231acb53a8f02823ed89b33acda5fd3987a53ad8da6bf26964af9d7eed9e03e53415d37aa96045676d56da";
    let order = client.get_order_by_id(order_id).await?;

    assert_eq!(order.uid, order_id);

    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_get_order_by_tx_hash() -> Result<()> {
    let client = OrderApiClient::new(Network::Mainnet)?;
    let tx_hash = "0xffd92faa1419c59ff0ac7f090998e9159f4b7f28bf67ad6b061c728c0da265f2";

    let orders = client.get_orders_by_tx_hash(tx_hash).await?;

    assert_eq!(orders.len(), 1);
    assert_eq!(orders[0].uid, "0xeaef82ff8696bff255e130b266231acb53a8f02823ed89b33acda5fd3987a53ad8da6bf26964af9d7eed9e03e53415d37aa96045676d56da");

    Ok(())
}
