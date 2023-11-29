#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let country = "your country";
    let response = client
        .get_country_specs_country(country)
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}