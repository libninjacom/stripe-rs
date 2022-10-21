#![allow(unused_imports)]
use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let card = "your card";
    let response = client
        .post_test_helpers_issuing_cards_card_shipping_deliver(card)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
