#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let fee = "your fee";
    let id = "your id";
    let response = client
        .get_application_fees_fee_refunds_id(fee, id)
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}