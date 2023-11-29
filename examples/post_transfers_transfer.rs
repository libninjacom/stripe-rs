#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let transfer = "your transfer";
    let response = client.post_transfers_transfer(transfer).await.unwrap();
    println!("{:#?}", response);
}