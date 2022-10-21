#![allow(unused_imports)]
use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let link = "your link";
    let response = client.post_file_links_link(link).send().await.unwrap();
    println!("{:#?}", response);
}
