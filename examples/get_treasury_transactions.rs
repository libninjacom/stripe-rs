use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let financial_account = "your financial account";
    let response = client
        .get_treasury_transactions(financial_account)
        .created(::serde_json::json!({}))
        .ending_before("your ending before")
        .expand(&["your expand"])
        .limit(1)
        .order_by("your order by")
        .starting_after("your starting after")
        .status("your status")
        .status_transitions(::serde_json::json!({}))
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
