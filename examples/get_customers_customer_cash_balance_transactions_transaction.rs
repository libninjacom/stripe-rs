#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let customer = "your customer";
    let transaction = "your transaction";
    let response = client
        .get_customers_customer_cash_balance_transactions_transaction(
            customer,
            transaction,
        )
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}