#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let cardholder = "your cardholder";
    let response = client
        .get_issuing_cardholders_cardholder(cardholder)
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}