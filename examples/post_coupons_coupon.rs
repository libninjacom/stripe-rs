#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let coupon = "your coupon";
    let response = client.post_coupons_coupon(coupon).await.unwrap();
    println!("{:#?}", response);
}