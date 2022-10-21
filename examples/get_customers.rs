#![allow(unused_imports)]
use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let response = client
        .get_customers()
        .created(::serde_json::json!({}))
        .email("your email")
        .ending_before("your ending before")
        .expand(&["your expand"])
        .limit(1)
        .starting_after("your starting after")
        .test_clock("your test clock")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
