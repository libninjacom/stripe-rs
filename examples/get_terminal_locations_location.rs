#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let location = "your location";
    let response = client
        .get_terminal_locations_location(location)
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}