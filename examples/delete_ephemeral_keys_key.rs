#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let key = "your key";
    let response = client.delete_ephemeral_keys_key(key).await.unwrap();
    println!("{:#?}", response);
}