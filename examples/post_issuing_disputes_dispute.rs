#![allow(unused_imports)]
use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let dispute = "your dispute";
    let response = client.post_issuing_disputes_dispute(dispute).send().await.unwrap();
    println!("{:#?}", response);
}
