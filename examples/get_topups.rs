#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let response = client
        .get_topups()
        .amount(serde_json::json!({}))
        .created(serde_json::json!({}))
        .ending_before("your ending before")
        .expand(&["your expand"])
        .limit(1)
        .starting_after("your starting after")
        .status("your status")
        .await
        .unwrap();
    println!("{:#?}", response);
}