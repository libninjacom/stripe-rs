#![allow(unused_imports)]
use stripe2::StripeClient;
use stripe2::model::*;
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
        .starting_after("your starting after")
        .status("your status")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
