#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let quote = "your quote";
    let response = client
        .get_quotes_quote_pdf(quote)
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}