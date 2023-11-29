#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let response = client
        .get_checkout_sessions()
        .customer("your customer")
        .customer_details(CustomerDetailsParams {
            email: "your email".to_owned(),
        })
        .ending_before("your ending before")
        .expand(&["your expand"])
        .limit(1)
        .payment_intent("your payment intent")
        .payment_link("your payment link")
        .starting_after("your starting after")
        .status("your status")
        .subscription("your subscription")
        .await
        .unwrap();
    println!("{:#?}", response);
}