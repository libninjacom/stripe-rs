use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let response = client
        .get_refunds()
        .charge("your charge")
        .created(::serde_json::json!({}))
        .ending_before("your ending before")
        .expand(&["your expand"])
        .limit(1)
        .payment_intent("your payment intent")
        .starting_after("your starting after")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
