#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let response = client
        .get_charges()
        .created(serde_json::json!({}))
        .customer("your customer")
        .ending_before("your ending before")
        .expand(&["your expand"])
        .limit(1)
        .payment_intent("your payment intent")
        .starting_after("your starting after")
        .transfer_group("your transfer group")
        .await
        .unwrap();
    println!("{:#?}", response);
}