#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let item = "your item";
    let response = client.delete_subscription_items_item(item).await.unwrap();
    println!("{:#?}", response);
}