#![allow(unused_imports)]
use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let customer = "your customer";
    let response = client.post_customers_customer(customer).send().await.unwrap();
    println!("{:#?}", response);
}
