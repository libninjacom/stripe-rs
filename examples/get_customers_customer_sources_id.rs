#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let customer = "your customer";
    let id = "your id";
    let response = client
        .get_customers_customer_sources_id(customer, id)
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}