#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let response = client
        .get_issuing_transactions()
        .card("your card")
        .cardholder("your cardholder")
        .created(serde_json::json!({}))
        .ending_before("your ending before")
        .expand(&["your expand"])
        .limit(1)
        .starting_after("your starting after")
        .type_("your type")
        .await
        .unwrap();
    println!("{:#?}", response);
}