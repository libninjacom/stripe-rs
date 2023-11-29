#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let configuration = "your configuration";
    let response = client
        .get_terminal_configurations_configuration(configuration)
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}