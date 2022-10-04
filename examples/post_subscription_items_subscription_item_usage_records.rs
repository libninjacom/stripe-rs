use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let subscription_item = "your subscription item";
    let response = client
        .post_subscription_items_subscription_item_usage_records(subscription_item)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
