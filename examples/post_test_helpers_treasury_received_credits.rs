#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let response = client.post_test_helpers_treasury_received_credits().await.unwrap();
    println!("{:#?}", response);
}