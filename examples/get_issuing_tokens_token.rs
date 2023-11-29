#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let token = "your token";
    let response = client
        .get_issuing_tokens_token(token)
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}