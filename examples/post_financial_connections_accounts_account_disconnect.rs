#![allow(unused_imports)]
use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let account = "your account";
    let response = client
        .post_financial_connections_accounts_account_disconnect(account)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
