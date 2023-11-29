#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let dispute = "your dispute";
    let response = client.post_issuing_disputes_dispute_submit(dispute).await.unwrap();
    println!("{:#?}", response);
}