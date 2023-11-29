#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let id = "your id";
    let transfer = "your transfer";
    let response = client
        .post_transfers_transfer_reversals_id(id, transfer)
        .await
        .unwrap();
    println!("{:#?}", response);
}