#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let dispute = "your dispute";
    let response = client
        .get_disputes_dispute(dispute)
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}