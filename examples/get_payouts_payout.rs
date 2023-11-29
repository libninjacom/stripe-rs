#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let payout = "your payout";
    let response = client
        .get_payouts_payout(payout)
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}