#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let product = "your product";
    let response = client
        .get_climate_products_product(product)
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}