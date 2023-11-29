#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let value_list = "your value list";
    let response = client
        .get_radar_value_list_items(value_list)
        .created(serde_json::json!({}))
        .ending_before("your ending before")
        .expand(&["your expand"])
        .limit(1)
        .starting_after("your starting after")
        .value("your value")
        .await
        .unwrap();
    println!("{:#?}", response);
}