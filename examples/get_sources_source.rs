#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let source = "your source";
    let response = client
        .get_sources_source(source)
        .client_secret("your client secret")
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}