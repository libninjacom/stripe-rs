#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let intent = "your intent";
    let response = client
        .get_setup_intents_intent(intent)
        .client_secret("your client secret")
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}