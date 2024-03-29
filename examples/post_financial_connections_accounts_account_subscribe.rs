#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let account = "your account";
    let response = client
        .post_financial_connections_accounts_account_subscribe(account)
        .await
        .unwrap();
    println!("{:#?}", response);
}