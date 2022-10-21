#![allow(unused_imports)]
use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let topup = "your topup";
    let response = client.post_topups_topup(topup).send().await.unwrap();
    println!("{:#?}", response);
}
