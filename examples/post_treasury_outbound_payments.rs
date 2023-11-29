#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let response = client.post_treasury_outbound_payments().await.unwrap();
    println!("{:#?}", response);
}