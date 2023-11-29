#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let intent = "your intent";
    let response = client.post_setup_intents_intent_confirm(intent).await.unwrap();
    println!("{:#?}", response);
}