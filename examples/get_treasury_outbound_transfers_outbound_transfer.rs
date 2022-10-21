#![allow(unused_imports)]
use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let outbound_transfer = "your outbound transfer";
    let response = client
        .get_treasury_outbound_transfers_outbound_transfer(outbound_transfer)
        .expand(&["your expand"])
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
