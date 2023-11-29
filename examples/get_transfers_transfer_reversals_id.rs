#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let id = "your id";
    let transfer = "your transfer";
    let response = client
        .get_transfers_transfer_reversals_id(id, transfer)
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}