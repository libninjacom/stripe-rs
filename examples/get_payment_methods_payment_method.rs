#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let payment_method = "your payment method";
    let response = client
        .get_payment_methods_payment_method(payment_method)
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}