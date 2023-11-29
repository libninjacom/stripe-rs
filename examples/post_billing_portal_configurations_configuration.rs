#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let configuration = "your configuration";
    let response = client
        .post_billing_portal_configurations_configuration(configuration)
        .await
        .unwrap();
    println!("{:#?}", response);
}