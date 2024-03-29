#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let refund = "your refund";
    let response = client.post_test_helpers_refunds_refund_expire(refund).await.unwrap();
    println!("{:#?}", response);
}