#![allow(unused_imports)]
use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let response = client
        .get_prices()
        .active(true)
        .created(::serde_json::json!({}))
        .currency("your currency")
        .ending_before("your ending before")
        .expand(&["your expand"])
        .limit(1)
        .lookup_keys(&["your lookup keys"])
        .product("your product")
        .recurring(::serde_json::json!({}))
        .starting_after("your starting after")
        .type_("your type")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
