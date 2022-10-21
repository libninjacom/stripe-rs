#![allow(unused_imports)]
use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let account = "your account";
    let response = client.delete_accounts_account(account).send().await.unwrap();
    println!("{:#?}", response);
}
