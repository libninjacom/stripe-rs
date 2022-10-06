use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let response = client.post_setup_intents().send().await.unwrap();
    println!("{:#?}", response);
}