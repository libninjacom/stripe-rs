#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let outbound_transfer = "your outbound transfer";
    let response = client
        .post_test_helpers_treasury_outbound_transfers_outbound_transfer_fail(
            outbound_transfer,
        )
        .await
        .unwrap();
    println!("{:#?}", response);
}