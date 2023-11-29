#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let refund = "your refund";
    let response = client
        .get_refunds_refund(refund)
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}