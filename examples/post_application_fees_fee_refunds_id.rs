#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let fee = "your fee";
    let id = "your id";
    let response = client.post_application_fees_fee_refunds_id(fee, id).await.unwrap();
    println!("{:#?}", response);
}