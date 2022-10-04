use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let response = client.post_credit_notes().send().await.unwrap();
    println!("{:#?}", response);
}
