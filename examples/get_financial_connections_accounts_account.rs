#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let account = "your account";
    let response = client
        .get_financial_connections_accounts_account(account)
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}