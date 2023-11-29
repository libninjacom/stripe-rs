#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let transaction = "your transaction";
    let response = client
        .post_test_helpers_issuing_transactions_transaction_refund(transaction)
        .await
        .unwrap();
    println!("{:#?}", response);
}