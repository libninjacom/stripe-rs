#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let account = "your account";
    let response = client
        .get_accounts_account_external_accounts(account)
        .ending_before("your ending before")
        .expand(&["your expand"])
        .limit(1)
        .object("your object")
        .starting_after("your starting after")
        .await
        .unwrap();
    println!("{:#?}", response);
}