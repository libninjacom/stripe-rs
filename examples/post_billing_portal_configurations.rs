use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let response = client.post_billing_portal_configurations().send().await.unwrap();
    println!("{:#?}", response);
}
