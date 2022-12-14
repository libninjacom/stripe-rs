#![allow(unused_imports)]
use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let query = "your query";
    let response = client
        .get_customers_search(query)
        .expand(&["your expand"])
        .limit(1)
        .page("your page")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
