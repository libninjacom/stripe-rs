#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let account = "your account";
    let capability = "your capability";
    let response = client
        .get_accounts_account_capabilities_capability(account, capability)
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}