#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let quote = "your quote";
    let response = client.post_quotes_quote_cancel(quote).await.unwrap();
    println!("{:#?}", response);
}