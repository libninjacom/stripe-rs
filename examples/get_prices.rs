#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let response = client
        .get_prices()
        .active(true)
        .created(serde_json::json!({}))
        .currency("your currency")
        .ending_before("your ending before")
        .expand(&["your expand"])
        .limit(1)
        .lookup_keys(&["your lookup keys"])
        .product("your product")
        .recurring(AllPricesRecurringParams {
            interval: Some("your interval".to_owned()),
            usage_type: Some("your usage type".to_owned()),
        })
        .starting_after("your starting after")
        .type_("your type")
        .await
        .unwrap();
    println!("{:#?}", response);
}