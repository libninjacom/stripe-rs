#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let customer = "your customer";
    let response = client.delete_customers_customer_discount(customer).await.unwrap();
    println!("{:#?}", response);
}