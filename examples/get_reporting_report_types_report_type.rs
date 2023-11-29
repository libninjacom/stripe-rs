#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let report_type = "your report type";
    let response = client
        .get_reporting_report_types_report_type(report_type)
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}