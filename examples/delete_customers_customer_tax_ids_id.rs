#![allow(unused_imports)]
use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let customer = "your customer";
    let id = "your id";
    let response = client
        .delete_customers_customer_tax_ids_id(customer, id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
