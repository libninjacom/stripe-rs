#![allow(unused_imports)]
use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let test_clock = "your test clock";
    let response = client
        .delete_test_helpers_test_clocks_test_clock(test_clock)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
