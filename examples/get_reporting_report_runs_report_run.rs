#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let report_run = "your report run";
    let response = client
        .get_reporting_report_runs_report_run(report_run)
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}