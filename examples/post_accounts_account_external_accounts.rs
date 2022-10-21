#![allow(unused_imports)]
use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let account = "your account";
    let response = client
        .post_accounts_account_external_accounts(account)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
