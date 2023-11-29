#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let financial_account = "your financial account";
    let response = client
        .get_treasury_debit_reversals(financial_account)
        .ending_before("your ending before")
        .expand(&["your expand"])
        .limit(1)
        .received_debit("your received debit")
        .resolution("your resolution")
        .starting_after("your starting after")
        .status("your status")
        .await
        .unwrap();
    println!("{:#?}", response);
}