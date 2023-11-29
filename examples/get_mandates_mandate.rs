#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let mandate = "your mandate";
    let response = client
        .get_mandates_mandate(mandate)
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}