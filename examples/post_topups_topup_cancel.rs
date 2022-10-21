#![allow(unused_imports)]
use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let topup = "your topup";
    let response = client.post_topups_topup_cancel(topup).send().await.unwrap();
    println!("{:#?}", response);
}
