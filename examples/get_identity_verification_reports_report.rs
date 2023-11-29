#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let report = "your report";
    let response = client
        .get_identity_verification_reports_report(report)
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}