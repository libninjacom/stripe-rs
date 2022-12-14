#![allow(unused_imports)]
use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let financial_account = "your financial account";
    let response = client
        .get_treasury_received_credits(financial_account)
        .ending_before("your ending before")
        .expand(&["your expand"])
        .limit(1)
        .linked_flows(::serde_json::json!({}))
        .starting_after("your starting after")
        .status("your status")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
