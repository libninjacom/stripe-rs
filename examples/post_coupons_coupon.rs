#![allow(unused_imports)]
use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let coupon = "your coupon";
    let response = client.post_coupons_coupon(coupon).send().await.unwrap();
    println!("{:#?}", response);
}
