use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let response = client.post_file_links().send().await.unwrap();
    println!("{:#?}", response);
}
