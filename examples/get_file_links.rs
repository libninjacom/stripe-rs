#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let response = client
        .get_file_links()
        .created(serde_json::json!({}))
        .ending_before("your ending before")
        .expand(&["your expand"])
        .expired(true)
        .file("your file")
        .limit(1)
        .starting_after("your starting after")
        .await
        .unwrap();
    println!("{:#?}", response);
}