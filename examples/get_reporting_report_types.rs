#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let response = client
        .get_reporting_report_types()
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}