#![allow(unused_imports)]
use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let transaction = "your transaction";
    let response = client
        .post_issuing_transactions_transaction(transaction)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
