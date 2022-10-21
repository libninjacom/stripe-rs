#![allow(unused_imports)]
use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let refund = "your refund";
    let response = client
        .post_test_helpers_refunds_refund_expire(refund)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
