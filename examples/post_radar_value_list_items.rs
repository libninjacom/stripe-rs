#![allow(unused_imports)]
use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let response = client.post_radar_value_list_items().send().await.unwrap();
    println!("{:#?}", response);
}
