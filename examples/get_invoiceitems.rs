#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let response = client
        .get_invoiceitems()
        .created(serde_json::json!({}))
        .customer("your customer")
        .ending_before("your ending before")
        .expand(&["your expand"])
        .invoice("your invoice")
        .limit(1)
        .pending(true)
        .starting_after("your starting after")
        .await
        .unwrap();
    println!("{:#?}", response);
}