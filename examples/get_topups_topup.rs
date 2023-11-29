#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let topup = "your topup";
    let response = client
        .get_topups_topup(topup)
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}