#![allow(unused_imports)]
use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let outbound_transfer = "your outbound transfer";
    let response = client
        .post_test_helpers_treasury_outbound_transfers_outbound_transfer_post(
            outbound_transfer,
        )
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
