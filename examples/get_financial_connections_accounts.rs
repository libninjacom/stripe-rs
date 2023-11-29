#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let response = client
        .get_financial_connections_accounts()
        .account_holder(AccountholderParams {
            account: Some("your account".to_owned()),
            customer: Some("your customer".to_owned()),
        })
        .ending_before("your ending before")
        .expand(&["your expand"])
        .limit(1)
        .session("your session")
        .starting_after("your starting after")
        .await
        .unwrap();
    println!("{:#?}", response);
}