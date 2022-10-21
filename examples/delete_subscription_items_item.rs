#![allow(unused_imports)]
use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let item = "your item";
    let response = client.delete_subscription_items_item(item).send().await.unwrap();
    println!("{:#?}", response);
}
