#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let response = client
        .get_setup_intents()
        .attach_to_self(true)
        .created(serde_json::json!({}))
        .customer("your customer")
        .ending_before("your ending before")
        .expand(&["your expand"])
        .limit(1)
        .payment_method("your payment method")
        .starting_after("your starting after")
        .await
        .unwrap();
    println!("{:#?}", response);
}