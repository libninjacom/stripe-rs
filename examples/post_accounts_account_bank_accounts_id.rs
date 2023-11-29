#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let account = "your account";
    let id = "your id";
    let response = client
        .post_accounts_account_bank_accounts_id(account, id)
        .await
        .unwrap();
    println!("{:#?}", response);
}