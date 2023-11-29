#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let card = "your card";
    let response = client
        .get_issuing_cards_card(card)
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}