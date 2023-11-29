#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let payment_method = "your payment method";
    let response = client
        .post_payment_methods_payment_method(payment_method)
        .await
        .unwrap();
    println!("{:#?}", response);
}