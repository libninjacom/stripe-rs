#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let customer = "your customer";
    let subscription_exposed_id = "your subscription exposed id";
    let response = client
        .post_customers_customer_subscriptions_subscription_exposed_id(
            customer,
            subscription_exposed_id,
        )
        .await
        .unwrap();
    println!("{:#?}", response);
}