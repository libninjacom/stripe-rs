#![allow(unused_imports)]
use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let response = client
        .get_products()
        .active(true)
        .created(::serde_json::json!({}))
        .ending_before("your ending before")
        .expand(&["your expand"])
        .ids(&["your ids"])
        .limit(1)
        .shippable(true)
        .starting_after("your starting after")
        .url("your url")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
