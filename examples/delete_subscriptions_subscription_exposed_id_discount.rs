#![allow(unused_imports)]
use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let subscription_exposed_id = "your subscription exposed id";
    let response = client
        .delete_subscriptions_subscription_exposed_id_discount(subscription_exposed_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
