use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let mandate_notification = "your mandate notification";
    let source = "your source";
    let response = client
        .get_sources_source_mandate_notifications_mandate_notification(
            mandate_notification,
            source,
        )
        .expand(&["your expand"])
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
