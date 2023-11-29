#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let file = "your file";
    let response = client.get_files_file(file).expand(&["your expand"]).await.unwrap();
    println!("{:#?}", response);
}