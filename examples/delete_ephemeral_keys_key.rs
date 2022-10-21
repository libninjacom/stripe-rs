#![allow(unused_imports)]
use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let key = "your key";
    let response = client.delete_ephemeral_keys_key(key).send().await.unwrap();
    println!("{:#?}", response);
}
