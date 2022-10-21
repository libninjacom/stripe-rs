#![allow(unused_imports)]
use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let value_list = "your value list";
    let response = client
        .delete_radar_value_lists_value_list(value_list)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
