#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let domain = "your domain";
    let response = client
        .get_apple_pay_domains_domain(domain)
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}