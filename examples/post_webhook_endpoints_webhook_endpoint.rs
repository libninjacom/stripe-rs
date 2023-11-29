#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let webhook_endpoint = "your webhook endpoint";
    let response = client
        .post_webhook_endpoints_webhook_endpoint(webhook_endpoint)
        .await
        .unwrap();
    println!("{:#?}", response);
}