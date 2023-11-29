#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let financial_account = "your financial account";
    let response = client
        .get_treasury_transactions(financial_account)
        .created(serde_json::json!({}))
        .ending_before("your ending before")
        .expand(&["your expand"])
        .limit(1)
        .order_by("your order by")
        .starting_after("your starting after")
        .status("your status")
        .status_transitions(StatusTransitionTimestampSpecs {
            posted_at: Some(serde_json::json!({})),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}