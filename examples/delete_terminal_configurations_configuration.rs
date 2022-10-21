#![allow(unused_imports)]
use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let configuration = "your configuration";
    let response = client
        .delete_terminal_configurations_configuration(configuration)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
