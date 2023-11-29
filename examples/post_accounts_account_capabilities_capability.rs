#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let account = "your account";
    let capability = "your capability";
    let response = client
        .post_accounts_account_capabilities_capability(account, capability)
        .await
        .unwrap();
    println!("{:#?}", response);
}