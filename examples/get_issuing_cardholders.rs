#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let response = client
        .get_issuing_cardholders()
        .created(serde_json::json!({}))
        .email("your email")
        .ending_before("your ending before")
        .expand(&["your expand"])
        .limit(1)
        .phone_number("your phone number")
        .starting_after("your starting after")
        .status("your status")
        .type_("your type")
        .await
        .unwrap();
    println!("{:#?}", response);
}