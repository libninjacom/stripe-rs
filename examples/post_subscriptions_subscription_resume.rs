#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let subscription = "your subscription";
    let response = client
        .post_subscriptions_subscription_resume(subscription)
        .await
        .unwrap();
    println!("{:#?}", response);
}