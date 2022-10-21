#![allow(unused_imports)]
use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let id = "your id";
    let transfer = "your transfer";
    let response = client
        .post_transfers_transfer_reversals_id(id, transfer)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
