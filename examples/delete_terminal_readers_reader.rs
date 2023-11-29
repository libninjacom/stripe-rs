#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let reader = "your reader";
    let response = client.delete_terminal_readers_reader(reader).await.unwrap();
    println!("{:#?}", response);
}