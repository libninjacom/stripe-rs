#![allow(unused_imports)]
use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let response = client
        .get_skus()
        .active(true)
        .attributes(::serde_json::json!({}))
        .ending_before("your ending before")
        .expand(&["your expand"])
        .ids(&["your ids"])
        .in_stock(true)
        .limit(1)
        .product("your product")
        .starting_after("your starting after")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
