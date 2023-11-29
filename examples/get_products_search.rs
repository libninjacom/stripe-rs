#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let query = "your query";
    let response = client
        .get_products_search(query)
        .expand(&["your expand"])
        .limit(1)
        .page("your page")
        .await
        .unwrap();
    println!("{:#?}", response);
}