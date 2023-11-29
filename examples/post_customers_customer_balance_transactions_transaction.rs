#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let customer = "your customer";
    let transaction = "your transaction";
    let response = client
        .post_customers_customer_balance_transactions_transaction(customer, transaction)
        .await
        .unwrap();
    println!("{:#?}", response);
}