#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let response = client
        .post_test_helpers_issuing_transactions_create_force_capture()
        .await
        .unwrap();
    println!("{:#?}", response);
}