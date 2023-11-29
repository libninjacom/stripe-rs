#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let value_list = "your value list";
    let response = client
        .get_radar_value_lists_value_list(value_list)
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}