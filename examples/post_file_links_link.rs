#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let link = "your link";
    let response = client.post_file_links_link(link).await.unwrap();
    println!("{:#?}", response);
}