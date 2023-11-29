#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let subscription_item = "your subscription item";
    let response = client
        .post_subscription_items_subscription_item_usage_records(subscription_item)
        .await
        .unwrap();
    println!("{:#?}", response);
}