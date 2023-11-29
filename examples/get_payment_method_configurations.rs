#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let response = client
        .get_payment_method_configurations()
        .application(serde_json::json!({}))
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}