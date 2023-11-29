#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let debit_reversal = "your debit reversal";
    let response = client
        .get_treasury_debit_reversals_debit_reversal(debit_reversal)
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}