#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let response = client
        .get_balance_history()
        .created(serde_json::json!({}))
        .currency("your currency")
        .ending_before("your ending before")
        .expand(&["your expand"])
        .limit(1)
        .payout("your payout")
        .source("your source")
        .starting_after("your starting after")
        .type_("your type")
        .await
        .unwrap();
    println!("{:#?}", response);
}