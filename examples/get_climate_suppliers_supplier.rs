#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let supplier = "your supplier";
    let response = client
        .get_climate_suppliers_supplier(supplier)
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}