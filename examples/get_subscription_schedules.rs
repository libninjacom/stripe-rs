use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let response = client
        .get_subscription_schedules()
        .canceled_at(::serde_json::json!({}))
        .completed_at(::serde_json::json!({}))
        .created(::serde_json::json!({}))
        .customer("your customer")
        .ending_before("your ending before")
        .expand(&["your expand"])
        .limit(1)
        .released_at(::serde_json::json!({}))
        .scheduled(true)
        .starting_after("your starting after")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
