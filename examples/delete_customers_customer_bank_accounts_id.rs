#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let customer = "your customer";
    let id = "your id";
    let response = client
        .delete_customers_customer_bank_accounts_id(customer, id)
        .await
        .unwrap();
    println!("{:#?}", response);
}