#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let source = "your source";
    let response = client.post_sources_source(source).await.unwrap();
    println!("{:#?}", response);
}