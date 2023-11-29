#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let tax_rate = "your tax rate";
    let response = client
        .get_tax_rates_tax_rate(tax_rate)
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}