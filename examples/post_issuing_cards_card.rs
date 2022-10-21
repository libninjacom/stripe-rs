#![allow(unused_imports)]
use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let card = "your card";
    let response = client.post_issuing_cards_card(card).send().await.unwrap();
    println!("{:#?}", response);
}
