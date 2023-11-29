#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let financial_account = "your financial account";
    let response = client
        .get_treasury_transaction_entries(financial_account)
        .created(serde_json::json!({}))
        .effective_at(serde_json::json!({}))
        .ending_before("your ending before")
        .expand(&["your expand"])
        .limit(1)
        .order_by("your order by")
        .starting_after("your starting after")
        .transaction("your transaction")
        .await
        .unwrap();
    println!("{:#?}", response);
}