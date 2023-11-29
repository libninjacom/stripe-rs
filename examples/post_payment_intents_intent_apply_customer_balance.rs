#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let intent = "your intent";
    let response = client
        .post_payment_intents_intent_apply_customer_balance(intent)
        .await
        .unwrap();
    println!("{:#?}", response);
}