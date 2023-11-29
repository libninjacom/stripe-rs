#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let payment_link = "your payment link";
    let response = client
        .get_payment_links_payment_link(payment_link)
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}