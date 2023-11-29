#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let charge = "your charge";
    let response = client.post_charges_charge_dispute_close(charge).await.unwrap();
    println!("{:#?}", response);
}