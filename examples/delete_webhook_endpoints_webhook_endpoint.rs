use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let webhook_endpoint = "your webhook endpoint";
    let response = client
        .delete_webhook_endpoints_webhook_endpoint(webhook_endpoint)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
