#![allow(unused_imports)]
use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let response = client
        .get_transfers()
        .created(::serde_json::json!({}))
        .destination("your destination")
        .ending_before("your ending before")
        .expand(&["your expand"])
        .limit(1)
        .starting_after("your starting after")
        .transfer_group("your transfer group")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
