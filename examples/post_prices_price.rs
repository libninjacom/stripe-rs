#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let price = "your price";
    let response = client.post_prices_price(price).await.unwrap();
    println!("{:#?}", response);
}