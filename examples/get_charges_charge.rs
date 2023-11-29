#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let charge = "your charge";
    let response = client
        .get_charges_charge(charge)
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}