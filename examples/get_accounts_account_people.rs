#![allow(unused_imports)]
use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let account = "your account";
    let response = client
        .get_accounts_account_people(account)
        .ending_before("your ending before")
        .expand(&["your expand"])
        .limit(1)
        .relationship(::serde_json::json!({}))
        .starting_after("your starting after")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
