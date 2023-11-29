#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let topup = "your topup";
    let response = client.post_topups_topup(topup).await.unwrap();
    println!("{:#?}", response);
}