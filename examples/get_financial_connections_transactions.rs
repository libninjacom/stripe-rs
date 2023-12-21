#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let account = "your account";
    let response = client
        .get_financial_connections_transactions(account)
        .ending_before("your ending before")
        .expand(&["your expand"])
        .limit(1)
        .starting_after("your starting after")
        .transacted_at(serde_json::json!({}))
        .transaction_refresh(TransactionRefreshParams {
            after: "your after".to_owned(),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}