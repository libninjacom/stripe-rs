#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let authorization = "your authorization";
    let response = client
        .post_test_helpers_issuing_authorizations_authorization_reverse(authorization)
        .await
        .unwrap();
    println!("{:#?}", response);
}