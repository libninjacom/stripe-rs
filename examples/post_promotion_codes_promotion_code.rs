#![allow(unused_imports)]
use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let promotion_code = "your promotion code";
    let response = client
        .post_promotion_codes_promotion_code(promotion_code)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
