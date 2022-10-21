#![allow(unused_imports)]
use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let quote = "your quote";
    let response = client.post_quotes_quote_finalize(quote).send().await.unwrap();
    println!("{:#?}", response);
}
