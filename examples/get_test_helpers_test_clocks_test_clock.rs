#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let test_clock = "your test clock";
    let response = client
        .get_test_helpers_test_clocks_test_clock(test_clock)
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}