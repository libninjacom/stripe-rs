#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let rate_id = "your rate id";
    let response = client
        .get_exchange_rates_rate_id(rate_id)
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}