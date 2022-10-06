use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let response = client
        .get_issuing_disputes()
        .created(::serde_json::json!({}))
        .ending_before("your ending before")
        .expand(&["your expand"])
        .limit(1)
        .starting_after("your starting after")
        .status("your status")
        .transaction("your transaction")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}