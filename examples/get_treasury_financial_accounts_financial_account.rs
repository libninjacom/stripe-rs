#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let financial_account = "your financial account";
    let response = client
        .get_treasury_financial_accounts_financial_account(financial_account)
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}