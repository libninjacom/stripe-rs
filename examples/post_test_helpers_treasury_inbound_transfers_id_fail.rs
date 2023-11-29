#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let id = "your id";
    let response = client
        .post_test_helpers_treasury_inbound_transfers_id_fail(id)
        .await
        .unwrap();
    println!("{:#?}", response);
}