#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let response = client
        .get_terminal_readers()
        .device_type("your device type")
        .ending_before("your ending before")
        .expand(&["your expand"])
        .limit(1)
        .location("your location")
        .serial_number("your serial number")
        .starting_after("your starting after")
        .status("your status")
        .await
        .unwrap();
    println!("{:#?}", response);
}