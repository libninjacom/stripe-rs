#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let promotion_code = "your promotion code";
    let response = client
        .get_promotion_codes_promotion_code(promotion_code)
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}