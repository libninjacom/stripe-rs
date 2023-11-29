#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let link = "your link";
    let response = client
        .get_file_links_link(link)
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}