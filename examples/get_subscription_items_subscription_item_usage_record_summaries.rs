use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let subscription_item = "your subscription item";
    let response = client
        .get_subscription_items_subscription_item_usage_record_summaries(
            subscription_item,
        )
        .ending_before("your ending before")
        .expand(&["your expand"])
        .limit(1)
        .starting_after("your starting after")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
