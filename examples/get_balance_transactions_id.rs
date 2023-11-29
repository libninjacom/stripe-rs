#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let id = "your id";
    let response = client
        .get_balance_transactions_id(id)
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}