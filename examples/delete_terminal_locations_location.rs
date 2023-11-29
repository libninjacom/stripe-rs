#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let location = "your location";
    let response = client.delete_terminal_locations_location(location).await.unwrap();
    println!("{:#?}", response);
}