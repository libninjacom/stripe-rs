#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let settlement = "your settlement";
    let response = client
        .get_issuing_settlements_settlement(settlement)
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}