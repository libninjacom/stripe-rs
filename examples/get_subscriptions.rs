#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let response = client
        .get_subscriptions()
        .automatic_tax(AutomaticTaxFilterParams {
            enabled: true,
        })
        .collection_method("your collection method")
        .created(serde_json::json!({}))
        .current_period_end(serde_json::json!({}))
        .current_period_start(serde_json::json!({}))
        .customer("your customer")
        .ending_before("your ending before")
        .expand(&["your expand"])
        .limit(1)
        .price("your price")
        .starting_after("your starting after")
        .status("your status")
        .test_clock("your test clock")
        .await
        .unwrap();
    println!("{:#?}", response);
}