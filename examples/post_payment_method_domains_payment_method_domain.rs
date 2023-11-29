#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let payment_method_domain = "your payment method domain";
    let response = client
        .post_payment_method_domains_payment_method_domain(payment_method_domain)
        .await
        .unwrap();
    println!("{:#?}", response);
}