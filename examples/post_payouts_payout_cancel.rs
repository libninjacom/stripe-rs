#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let payout = "your payout";
    let response = client.post_payouts_payout_cancel(payout).await.unwrap();
    println!("{:#?}", response);
}