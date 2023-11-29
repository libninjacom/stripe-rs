#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let account = "your account";
    let id = "your id";
    let response = client
        .get_accounts_account_bank_accounts_id(account, id)
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}